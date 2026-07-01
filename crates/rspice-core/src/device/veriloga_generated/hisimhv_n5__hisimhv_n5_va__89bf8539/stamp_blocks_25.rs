#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_8(
        var_arg_slot: &mut f64,
        var_arg_dn0_slot: &mut f64,
        var_arg_dn10_slot: &mut f64,
        var_arg_dn11_slot: &mut f64,
        var_arg_dn14_slot: &mut f64,
        var_arg_dn2_slot: &mut f64,
        var_arg_dn4_slot: &mut f64,
        var_arg_dn5_slot: &mut f64,
        var_arg_dn6_slot: &mut f64,
        var_arg_dn7_slot: &mut f64,
        var_arg_dn8_slot: &mut f64,
        var_arg_dn9_slot: &mut f64,
        var_arg_rv_slot: &mut f64,
        var_betatnom_slot: &mut f64,
        var_betatnom_rv_slot: &mut f64,
        var_car_slot: &mut f64,
        var_car_rv_slot: &mut f64,
        var_carr_slot: &mut f64,
        var_carr_dn0_slot: &mut f64,
        var_carr_dn10_slot: &mut f64,
        var_carr_dn11_slot: &mut f64,
        var_carr_dn14_slot: &mut f64,
        var_carr_dn2_slot: &mut f64,
        var_carr_dn4_slot: &mut f64,
        var_carr_dn5_slot: &mut f64,
        var_carr_dn6_slot: &mut f64,
        var_carr_dn7_slot: &mut f64,
        var_carr_dn8_slot: &mut f64,
        var_carr_dn9_slot: &mut f64,
        var_carr_rv_slot: &mut f64,
        var_cgsb_slot: &mut f64,
        var_cgsb_dn0_slot: &mut f64,
        var_cgsb_dn10_slot: &mut f64,
        var_cgsb_dn11_slot: &mut f64,
        var_cgsb_dn14_slot: &mut f64,
        var_cgsb_dn2_slot: &mut f64,
        var_cgsb_dn4_slot: &mut f64,
        var_cgsb_dn5_slot: &mut f64,
        var_cgsb_dn6_slot: &mut f64,
        var_cgsb_dn7_slot: &mut f64,
        var_cgsb_dn8_slot: &mut f64,
        var_cgsb_dn9_slot: &mut f64,
        var_cgsb_rv_slot: &mut f64,
        var_cnst0over_slot: &mut f64,
        var_cnst0over_dn0_slot: &mut f64,
        var_cnst0over_dn10_slot: &mut f64,
        var_cnst0over_dn11_slot: &mut f64,
        var_cnst0over_dn14_slot: &mut f64,
        var_cnst0over_dn2_slot: &mut f64,
        var_cnst0over_dn4_slot: &mut f64,
        var_cnst0over_dn5_slot: &mut f64,
        var_cnst0over_dn6_slot: &mut f64,
        var_cnst0over_dn7_slot: &mut f64,
        var_cnst0over_dn8_slot: &mut f64,
        var_cnst0over_dn9_slot: &mut f64,
        var_cnst0over_rv_slot: &mut f64,
        var_cnst0overs_slot: &mut f64,
        var_cnst0overs_dn0_slot: &mut f64,
        var_cnst0overs_dn10_slot: &mut f64,
        var_cnst0overs_dn11_slot: &mut f64,
        var_cnst0overs_dn14_slot: &mut f64,
        var_cnst0overs_dn2_slot: &mut f64,
        var_cnst0overs_dn4_slot: &mut f64,
        var_cnst0overs_dn5_slot: &mut f64,
        var_cnst0overs_dn6_slot: &mut f64,
        var_cnst0overs_dn7_slot: &mut f64,
        var_cnst0overs_dn8_slot: &mut f64,
        var_cnst0overs_dn9_slot: &mut f64,
        var_cnst0overs_rv_slot: &mut f64,
        var_cx_slot: &mut f64,
        var_cx_rv_slot: &mut f64,
        var_deltemp_slot: &mut f64,
        var_deltemp_dn0_slot: &mut f64,
        var_deltemp_dn10_slot: &mut f64,
        var_deltemp_dn11_slot: &mut f64,
        var_deltemp_dn14_slot: &mut f64,
        var_deltemp_dn2_slot: &mut f64,
        var_deltemp_dn4_slot: &mut f64,
        var_deltemp_dn5_slot: &mut f64,
        var_deltemp_dn6_slot: &mut f64,
        var_deltemp_dn7_slot: &mut f64,
        var_deltemp_dn8_slot: &mut f64,
        var_deltemp_dn9_slot: &mut f64,
        var_deltemp_rv_slot: &mut f64,
        var_gd_slot: &mut f64,
        var_gd_dn0_slot: &mut f64,
        var_gd_dn10_slot: &mut f64,
        var_gd_dn11_slot: &mut f64,
        var_gd_dn14_slot: &mut f64,
        var_gd_dn2_slot: &mut f64,
        var_gd_dn4_slot: &mut f64,
        var_gd_dn5_slot: &mut f64,
        var_gd_dn6_slot: &mut f64,
        var_gd_dn7_slot: &mut f64,
        var_gd_dn8_slot: &mut f64,
        var_gd_dn9_slot: &mut f64,
        var_gd_rv_slot: &mut f64,
        var_gth_slot: &mut f64,
        var_gth_dn0_slot: &mut f64,
        var_gth_dn10_slot: &mut f64,
        var_gth_dn11_slot: &mut f64,
        var_gth_dn14_slot: &mut f64,
        var_gth_dn2_slot: &mut f64,
        var_gth_dn4_slot: &mut f64,
        var_gth_dn5_slot: &mut f64,
        var_gth_dn6_slot: &mut f64,
        var_gth_dn7_slot: &mut f64,
        var_gth_dn8_slot: &mut f64,
        var_gth_dn9_slot: &mut f64,
        var_gth_rv_slot: &mut f64,
        var_mu0_slot: &mut f64,
        var_mu0_dn0_slot: &mut f64,
        var_mu0_dn10_slot: &mut f64,
        var_mu0_dn11_slot: &mut f64,
        var_mu0_dn14_slot: &mut f64,
        var_mu0_dn2_slot: &mut f64,
        var_mu0_dn4_slot: &mut f64,
        var_mu0_dn5_slot: &mut f64,
        var_mu0_dn6_slot: &mut f64,
        var_mu0_dn7_slot: &mut f64,
        var_mu0_dn8_slot: &mut f64,
        var_mu0_dn9_slot: &mut f64,
        var_mu0_rv_slot: &mut f64,
        var_ninvde_slot: &mut f64,
        var_ninvde_dn0_slot: &mut f64,
        var_ninvde_dn10_slot: &mut f64,
        var_ninvde_dn11_slot: &mut f64,
        var_ninvde_dn14_slot: &mut f64,
        var_ninvde_dn2_slot: &mut f64,
        var_ninvde_dn4_slot: &mut f64,
        var_ninvde_dn5_slot: &mut f64,
        var_ninvde_dn6_slot: &mut f64,
        var_ninvde_dn7_slot: &mut f64,
        var_ninvde_dn8_slot: &mut f64,
        var_ninvde_dn9_slot: &mut f64,
        var_ninvde_rv_slot: &mut f64,
        var_ninvdecres_slot: &mut f64,
        var_ninvdecres_dn0_slot: &mut f64,
        var_ninvdecres_dn10_slot: &mut f64,
        var_ninvdecres_dn11_slot: &mut f64,
        var_ninvdecres_dn14_slot: &mut f64,
        var_ninvdecres_dn2_slot: &mut f64,
        var_ninvdecres_dn4_slot: &mut f64,
        var_ninvdecres_dn5_slot: &mut f64,
        var_ninvdecres_dn6_slot: &mut f64,
        var_ninvdecres_dn7_slot: &mut f64,
        var_ninvdecres_dn8_slot: &mut f64,
        var_ninvdecres_dn9_slot: &mut f64,
        var_ninvdecres_rv_slot: &mut f64,
        var_ninvdehres_slot: &mut f64,
        var_ninvdehres_dn0_slot: &mut f64,
        var_ninvdehres_dn10_slot: &mut f64,
        var_ninvdehres_dn11_slot: &mut f64,
        var_ninvdehres_dn14_slot: &mut f64,
        var_ninvdehres_dn2_slot: &mut f64,
        var_ninvdehres_dn4_slot: &mut f64,
        var_ninvdehres_dn5_slot: &mut f64,
        var_ninvdehres_dn6_slot: &mut f64,
        var_ninvdehres_dn7_slot: &mut f64,
        var_ninvdehres_dn8_slot: &mut f64,
        var_ninvdehres_dn9_slot: &mut f64,
        var_ninvdehres_rv_slot: &mut f64,
        var_p_slot: &mut f64,
        var_p_dn0_slot: &mut f64,
        var_p_dn10_slot: &mut f64,
        var_p_dn11_slot: &mut f64,
        var_p_dn14_slot: &mut f64,
        var_p_dn2_slot: &mut f64,
        var_p_dn4_slot: &mut f64,
        var_p_dn5_slot: &mut f64,
        var_p_dn6_slot: &mut f64,
        var_p_dn7_slot: &mut f64,
        var_p_dn8_slot: &mut f64,
        var_p_dn9_slot: &mut f64,
        var_p_rv_slot: &mut f64,
        var_qb_nqs_slot: &mut f64,
        var_qb_nqs_dn13_slot: &mut f64,
        var_qb_nqs_rv_slot: &mut f64,
        var_qd_nqs_slot: &mut f64,
        var_qd_nqs_dn0_slot: &mut f64,
        var_qd_nqs_dn10_slot: &mut f64,
        var_qd_nqs_dn11_slot: &mut f64,
        var_qd_nqs_dn12_slot: &mut f64,
        var_qd_nqs_dn14_slot: &mut f64,
        var_qd_nqs_dn2_slot: &mut f64,
        var_qd_nqs_dn4_slot: &mut f64,
        var_qd_nqs_dn5_slot: &mut f64,
        var_qd_nqs_dn6_slot: &mut f64,
        var_qd_nqs_dn7_slot: &mut f64,
        var_qd_nqs_dn8_slot: &mut f64,
        var_qd_nqs_dn9_slot: &mut f64,
        var_qd_nqs_rv_slot: &mut f64,
        var_qg_slot: &mut f64,
        var_qg_dn0_slot: &mut f64,
        var_qg_dn10_slot: &mut f64,
        var_qg_dn11_slot: &mut f64,
        var_qg_dn14_slot: &mut f64,
        var_qg_dn2_slot: &mut f64,
        var_qg_dn4_slot: &mut f64,
        var_qg_dn5_slot: &mut f64,
        var_qg_dn6_slot: &mut f64,
        var_qg_dn7_slot: &mut f64,
        var_qg_dn8_slot: &mut f64,
        var_qg_dn9_slot: &mut f64,
        var_qg_nqs_slot: &mut f64,
        var_qg_nqs_dn12_slot: &mut f64,
        var_qg_nqs_dn13_slot: &mut f64,
        var_qg_nqs_rv_slot: &mut f64,
        var_qg_rv_slot: &mut f64,
        var_qi_nqs_slot: &mut f64,
        var_qi_nqs_dn12_slot: &mut f64,
        var_qi_nqs_rv_slot: &mut f64,
        var_qs_slot: &mut f64,
        var_qs_dn0_slot: &mut f64,
        var_qs_dn10_slot: &mut f64,
        var_qs_dn11_slot: &mut f64,
        var_qs_dn14_slot: &mut f64,
        var_qs_dn2_slot: &mut f64,
        var_qs_dn4_slot: &mut f64,
        var_qs_dn5_slot: &mut f64,
        var_qs_dn6_slot: &mut f64,
        var_qs_dn7_slot: &mut f64,
        var_qs_dn8_slot: &mut f64,
        var_qs_dn9_slot: &mut f64,
        var_qs_nqs_slot: &mut f64,
        var_qs_nqs_dn0_slot: &mut f64,
        var_qs_nqs_dn10_slot: &mut f64,
        var_qs_nqs_dn11_slot: &mut f64,
        var_qs_nqs_dn12_slot: &mut f64,
        var_qs_nqs_dn14_slot: &mut f64,
        var_qs_nqs_dn2_slot: &mut f64,
        var_qs_nqs_dn4_slot: &mut f64,
        var_qs_nqs_dn5_slot: &mut f64,
        var_qs_nqs_dn6_slot: &mut f64,
        var_qs_nqs_dn7_slot: &mut f64,
        var_qs_nqs_dn8_slot: &mut f64,
        var_qs_nqs_dn9_slot: &mut f64,
        var_qs_nqs_rv_slot: &mut f64,
        var_qs_rv_slot: &mut f64,
        var_rde_slot: &mut f64,
        var_rde_dn0_slot: &mut f64,
        var_rde_dn10_slot: &mut f64,
        var_rde_dn11_slot: &mut f64,
        var_rde_dn14_slot: &mut f64,
        var_rde_dn2_slot: &mut f64,
        var_rde_dn4_slot: &mut f64,
        var_rde_dn5_slot: &mut f64,
        var_rde_dn6_slot: &mut f64,
        var_rde_dn7_slot: &mut f64,
        var_rde_dn8_slot: &mut f64,
        var_rde_dn9_slot: &mut f64,
        var_rde_rv_slot: &mut f64,
        var_rdvde_slot: &mut f64,
        var_rdvde_dn0_slot: &mut f64,
        var_rdvde_dn10_slot: &mut f64,
        var_rdvde_dn11_slot: &mut f64,
        var_rdvde_dn14_slot: &mut f64,
        var_rdvde_dn2_slot: &mut f64,
        var_rdvde_dn4_slot: &mut f64,
        var_rdvde_dn5_slot: &mut f64,
        var_rdvde_dn6_slot: &mut f64,
        var_rdvde_dn7_slot: &mut f64,
        var_rdvde_dn8_slot: &mut f64,
        var_rdvde_dn9_slot: &mut f64,
        var_rdvde_rv_slot: &mut f64,
        var_rrdrmue_slot: &mut f64,
        var_rrdrmue_dn0_slot: &mut f64,
        var_rrdrmue_dn10_slot: &mut f64,
        var_rrdrmue_dn11_slot: &mut f64,
        var_rrdrmue_dn14_slot: &mut f64,
        var_rrdrmue_dn2_slot: &mut f64,
        var_rrdrmue_dn4_slot: &mut f64,
        var_rrdrmue_dn5_slot: &mut f64,
        var_rrdrmue_dn6_slot: &mut f64,
        var_rrdrmue_dn7_slot: &mut f64,
        var_rrdrmue_dn8_slot: &mut f64,
        var_rrdrmue_dn9_slot: &mut f64,
        var_rrdrmue_rv_slot: &mut f64,
        var_rrdrmues_slot: &mut f64,
        var_rrdrmues_dn0_slot: &mut f64,
        var_rrdrmues_dn10_slot: &mut f64,
        var_rrdrmues_dn11_slot: &mut f64,
        var_rrdrmues_dn14_slot: &mut f64,
        var_rrdrmues_dn2_slot: &mut f64,
        var_rrdrmues_dn4_slot: &mut f64,
        var_rrdrmues_dn5_slot: &mut f64,
        var_rrdrmues_dn6_slot: &mut f64,
        var_rrdrmues_dn7_slot: &mut f64,
        var_rrdrmues_dn8_slot: &mut f64,
        var_rrdrmues_dn9_slot: &mut f64,
        var_rrdrmues_rv_slot: &mut f64,
        var_rrdrvmax_slot: &mut f64,
        var_rrdrvmax_dn0_slot: &mut f64,
        var_rrdrvmax_dn10_slot: &mut f64,
        var_rrdrvmax_dn11_slot: &mut f64,
        var_rrdrvmax_dn14_slot: &mut f64,
        var_rrdrvmax_dn2_slot: &mut f64,
        var_rrdrvmax_dn4_slot: &mut f64,
        var_rrdrvmax_dn5_slot: &mut f64,
        var_rrdrvmax_dn6_slot: &mut f64,
        var_rrdrvmax_dn7_slot: &mut f64,
        var_rrdrvmax_dn8_slot: &mut f64,
        var_rrdrvmax_dn9_slot: &mut f64,
        var_rrdrvmax_rv_slot: &mut f64,
        var_rrdrvmaxs_slot: &mut f64,
        var_rrdrvmaxs_dn0_slot: &mut f64,
        var_rrdrvmaxs_dn10_slot: &mut f64,
        var_rrdrvmaxs_dn11_slot: &mut f64,
        var_rrdrvmaxs_dn14_slot: &mut f64,
        var_rrdrvmaxs_dn2_slot: &mut f64,
        var_rrdrvmaxs_dn4_slot: &mut f64,
        var_rrdrvmaxs_dn5_slot: &mut f64,
        var_rrdrvmaxs_dn6_slot: &mut f64,
        var_rrdrvmaxs_dn7_slot: &mut f64,
        var_rrdrvmaxs_dn8_slot: &mut f64,
        var_rrdrvmaxs_dn9_slot: &mut f64,
        var_rrdrvmaxs_rv_slot: &mut f64,
        var_rse_slot: &mut f64,
        var_rse_dn0_slot: &mut f64,
        var_rse_dn10_slot: &mut f64,
        var_rse_dn11_slot: &mut f64,
        var_rse_dn14_slot: &mut f64,
        var_rse_dn2_slot: &mut f64,
        var_rse_dn4_slot: &mut f64,
        var_rse_dn5_slot: &mut f64,
        var_rse_dn6_slot: &mut f64,
        var_rse_dn7_slot: &mut f64,
        var_rse_dn8_slot: &mut f64,
        var_rse_dn9_slot: &mut f64,
        var_rse_rv_slot: &mut f64,
        var_rsvde_slot: &mut f64,
        var_rsvde_dn0_slot: &mut f64,
        var_rsvde_dn10_slot: &mut f64,
        var_rsvde_dn11_slot: &mut f64,
        var_rsvde_dn14_slot: &mut f64,
        var_rsvde_dn2_slot: &mut f64,
        var_rsvde_dn4_slot: &mut f64,
        var_rsvde_dn5_slot: &mut f64,
        var_rsvde_dn6_slot: &mut f64,
        var_rsvde_dn7_slot: &mut f64,
        var_rsvde_dn8_slot: &mut f64,
        var_rsvde_dn9_slot: &mut f64,
        var_rsvde_rv_slot: &mut f64,
        var_tratio_slot: &mut f64,
        var_tratio_dn0_slot: &mut f64,
        var_tratio_dn10_slot: &mut f64,
        var_tratio_dn11_slot: &mut f64,
        var_tratio_dn14_slot: &mut f64,
        var_tratio_dn2_slot: &mut f64,
        var_tratio_dn4_slot: &mut f64,
        var_tratio_dn5_slot: &mut f64,
        var_tratio_dn6_slot: &mut f64,
        var_tratio_dn7_slot: &mut f64,
        var_tratio_dn8_slot: &mut f64,
        var_tratio_dn9_slot: &mut f64,
        var_tratio_rv_slot: &mut f64,
        var_vbd_slot: &mut f64,
        var_vbd_dn6_slot: &mut f64,
        var_vbd_dn8_slot: &mut f64,
        var_vbd_dn9_slot: &mut f64,
        var_vbd_rv_slot: &mut f64,
        var_vbsei_slot: &mut f64,
        var_vbsei_dn2_slot: &mut f64,
        var_vbsei_dn9_slot: &mut f64,
        var_vbsei_rv_slot: &mut f64,
        var_vbsi_slot: &mut f64,
        var_vbsi_dn8_slot: &mut f64,
        var_vbsi_dn9_slot: &mut f64,
        var_vbsi_rv_slot: &mut f64,
        var_vddpz_slot: &mut f64,
        var_vddpz_dn0_slot: &mut f64,
        var_vddpz_dn10_slot: &mut f64,
        var_vddpz_dn11_slot: &mut f64,
        var_vddpz_dn14_slot: &mut f64,
        var_vddpz_dn2_slot: &mut f64,
        var_vddpz_dn4_slot: &mut f64,
        var_vddpz_dn5_slot: &mut f64,
        var_vddpz_dn6_slot: &mut f64,
        var_vddpz_dn7_slot: &mut f64,
        var_vddpz_dn8_slot: &mut f64,
        var_vddpz_dn9_slot: &mut f64,
        var_vddpz_rv_slot: &mut f64,
        var_vdri_slot: &mut f64,
        var_vdri_dn0_slot: &mut f64,
        var_vdri_dn10_slot: &mut f64,
        var_vdri_dn11_slot: &mut f64,
        var_vdri_dn14_slot: &mut f64,
        var_vdri_dn2_slot: &mut f64,
        var_vdri_dn4_slot: &mut f64,
        var_vdri_dn5_slot: &mut f64,
        var_vdri_dn6_slot: &mut f64,
        var_vdri_dn7_slot: &mut f64,
        var_vdri_dn8_slot: &mut f64,
        var_vdri_dn9_slot: &mut f64,
        var_vdri_rv_slot: &mut f64,
        var_vdsei_slot: &mut f64,
        var_vdsei_dn0_slot: &mut f64,
        var_vdsei_dn2_slot: &mut f64,
        var_vdsei_rv_slot: &mut f64,
        var_vdsi_slot: &mut f64,
        var_vdsi_dn6_slot: &mut f64,
        var_vdsi_dn8_slot: &mut f64,
        var_vdsi_rv_slot: &mut f64,
        var_veffpower_slot: &mut f64,
        var_veffpower_dn0_slot: &mut f64,
        var_veffpower_dn10_slot: &mut f64,
        var_veffpower_dn11_slot: &mut f64,
        var_veffpower_dn14_slot: &mut f64,
        var_veffpower_dn2_slot: &mut f64,
        var_veffpower_dn4_slot: &mut f64,
        var_veffpower_dn5_slot: &mut f64,
        var_veffpower_dn6_slot: &mut f64,
        var_veffpower_dn7_slot: &mut f64,
        var_veffpower_dn8_slot: &mut f64,
        var_veffpower_dn9_slot: &mut f64,
        var_veffpower_rv_slot: &mut f64,
        var_vgd_slot: &mut f64,
        var_vgd_dn6_slot: &mut f64,
        var_vgd_dn7_slot: &mut f64,
        var_vgd_dn8_slot: &mut f64,
        var_vgd_rv_slot: &mut f64,
        var_vgsei_slot: &mut f64,
        var_vgsei_dn2_slot: &mut f64,
        var_vgsei_dn7_slot: &mut f64,
        var_vgsei_rv_slot: &mut f64,
        var_vgsi_slot: &mut f64,
        var_vgsi_dn7_slot: &mut f64,
        var_vgsi_dn8_slot: &mut f64,
        var_vgsi_rv_slot: &mut f64,
        var_vmaxeff_slot: &mut f64,
        var_vmaxeff_dn0_slot: &mut f64,
        var_vmaxeff_dn10_slot: &mut f64,
        var_vmaxeff_dn11_slot: &mut f64,
        var_vmaxeff_dn14_slot: &mut f64,
        var_vmaxeff_dn2_slot: &mut f64,
        var_vmaxeff_dn4_slot: &mut f64,
        var_vmaxeff_dn5_slot: &mut f64,
        var_vmaxeff_dn6_slot: &mut f64,
        var_vmaxeff_dn7_slot: &mut f64,
        var_vmaxeff_dn8_slot: &mut f64,
        var_vmaxeff_dn9_slot: &mut f64,
        var_vmaxeff_rv_slot: &mut f64,
        var_xov_slot: &mut f64,
        var_xov_dn0_slot: &mut f64,
        var_xov_dn10_slot: &mut f64,
        var_xov_dn11_slot: &mut f64,
        var_xov_dn14_slot: &mut f64,
        var_xov_dn2_slot: &mut f64,
        var_xov_dn4_slot: &mut f64,
        var_xov_dn5_slot: &mut f64,
        var_xov_dn6_slot: &mut f64,
        var_xov_dn7_slot: &mut f64,
        var_xov_dn8_slot: &mut f64,
        var_xov_dn9_slot: &mut f64,
        var_xov_rv_slot: &mut f64,
    ) {
        let mut var_arg: f64 = *var_arg_slot;
        let mut var_arg_dn0: f64 = *var_arg_dn0_slot;
        let mut var_arg_dn10: f64 = *var_arg_dn10_slot;
        let mut var_arg_dn11: f64 = *var_arg_dn11_slot;
        let mut var_arg_dn14: f64 = *var_arg_dn14_slot;
        let mut var_arg_dn2: f64 = *var_arg_dn2_slot;
        let mut var_arg_dn4: f64 = *var_arg_dn4_slot;
        let mut var_arg_dn5: f64 = *var_arg_dn5_slot;
        let mut var_arg_dn6: f64 = *var_arg_dn6_slot;
        let mut var_arg_dn7: f64 = *var_arg_dn7_slot;
        let mut var_arg_dn8: f64 = *var_arg_dn8_slot;
        let mut var_arg_dn9: f64 = *var_arg_dn9_slot;
        let mut var_arg_rv: f64 = *var_arg_rv_slot;
        let mut var_betatnom: f64 = *var_betatnom_slot;
        let mut var_betatnom_rv: f64 = *var_betatnom_rv_slot;
        let mut var_car: f64 = *var_car_slot;
        let mut var_car_rv: f64 = *var_car_rv_slot;
        let mut var_carr: f64 = *var_carr_slot;
        let mut var_carr_dn0: f64 = *var_carr_dn0_slot;
        let mut var_carr_dn10: f64 = *var_carr_dn10_slot;
        let mut var_carr_dn11: f64 = *var_carr_dn11_slot;
        let mut var_carr_dn14: f64 = *var_carr_dn14_slot;
        let mut var_carr_dn2: f64 = *var_carr_dn2_slot;
        let mut var_carr_dn4: f64 = *var_carr_dn4_slot;
        let mut var_carr_dn5: f64 = *var_carr_dn5_slot;
        let mut var_carr_dn6: f64 = *var_carr_dn6_slot;
        let mut var_carr_dn7: f64 = *var_carr_dn7_slot;
        let mut var_carr_dn8: f64 = *var_carr_dn8_slot;
        let mut var_carr_dn9: f64 = *var_carr_dn9_slot;
        let mut var_carr_rv: f64 = *var_carr_rv_slot;
        let mut var_cgsb: f64 = *var_cgsb_slot;
        let mut var_cgsb_dn0: f64 = *var_cgsb_dn0_slot;
        let mut var_cgsb_dn10: f64 = *var_cgsb_dn10_slot;
        let mut var_cgsb_dn11: f64 = *var_cgsb_dn11_slot;
        let mut var_cgsb_dn14: f64 = *var_cgsb_dn14_slot;
        let mut var_cgsb_dn2: f64 = *var_cgsb_dn2_slot;
        let mut var_cgsb_dn4: f64 = *var_cgsb_dn4_slot;
        let mut var_cgsb_dn5: f64 = *var_cgsb_dn5_slot;
        let mut var_cgsb_dn6: f64 = *var_cgsb_dn6_slot;
        let mut var_cgsb_dn7: f64 = *var_cgsb_dn7_slot;
        let mut var_cgsb_dn8: f64 = *var_cgsb_dn8_slot;
        let mut var_cgsb_dn9: f64 = *var_cgsb_dn9_slot;
        let mut var_cgsb_rv: f64 = *var_cgsb_rv_slot;
        let mut var_cnst0over: f64 = *var_cnst0over_slot;
        let mut var_cnst0over_dn0: f64 = *var_cnst0over_dn0_slot;
        let mut var_cnst0over_dn10: f64 = *var_cnst0over_dn10_slot;
        let mut var_cnst0over_dn11: f64 = *var_cnst0over_dn11_slot;
        let mut var_cnst0over_dn14: f64 = *var_cnst0over_dn14_slot;
        let mut var_cnst0over_dn2: f64 = *var_cnst0over_dn2_slot;
        let mut var_cnst0over_dn4: f64 = *var_cnst0over_dn4_slot;
        let mut var_cnst0over_dn5: f64 = *var_cnst0over_dn5_slot;
        let mut var_cnst0over_dn6: f64 = *var_cnst0over_dn6_slot;
        let mut var_cnst0over_dn7: f64 = *var_cnst0over_dn7_slot;
        let mut var_cnst0over_dn8: f64 = *var_cnst0over_dn8_slot;
        let mut var_cnst0over_dn9: f64 = *var_cnst0over_dn9_slot;
        let mut var_cnst0over_rv: f64 = *var_cnst0over_rv_slot;
        let mut var_cnst0overs: f64 = *var_cnst0overs_slot;
        let mut var_cnst0overs_dn0: f64 = *var_cnst0overs_dn0_slot;
        let mut var_cnst0overs_dn10: f64 = *var_cnst0overs_dn10_slot;
        let mut var_cnst0overs_dn11: f64 = *var_cnst0overs_dn11_slot;
        let mut var_cnst0overs_dn14: f64 = *var_cnst0overs_dn14_slot;
        let mut var_cnst0overs_dn2: f64 = *var_cnst0overs_dn2_slot;
        let mut var_cnst0overs_dn4: f64 = *var_cnst0overs_dn4_slot;
        let mut var_cnst0overs_dn5: f64 = *var_cnst0overs_dn5_slot;
        let mut var_cnst0overs_dn6: f64 = *var_cnst0overs_dn6_slot;
        let mut var_cnst0overs_dn7: f64 = *var_cnst0overs_dn7_slot;
        let mut var_cnst0overs_dn8: f64 = *var_cnst0overs_dn8_slot;
        let mut var_cnst0overs_dn9: f64 = *var_cnst0overs_dn9_slot;
        let mut var_cnst0overs_rv: f64 = *var_cnst0overs_rv_slot;
        let mut var_cx: f64 = *var_cx_slot;
        let mut var_cx_rv: f64 = *var_cx_rv_slot;
        let mut var_deltemp: f64 = *var_deltemp_slot;
        let mut var_deltemp_dn0: f64 = *var_deltemp_dn0_slot;
        let mut var_deltemp_dn10: f64 = *var_deltemp_dn10_slot;
        let mut var_deltemp_dn11: f64 = *var_deltemp_dn11_slot;
        let mut var_deltemp_dn14: f64 = *var_deltemp_dn14_slot;
        let mut var_deltemp_dn2: f64 = *var_deltemp_dn2_slot;
        let mut var_deltemp_dn4: f64 = *var_deltemp_dn4_slot;
        let mut var_deltemp_dn5: f64 = *var_deltemp_dn5_slot;
        let mut var_deltemp_dn6: f64 = *var_deltemp_dn6_slot;
        let mut var_deltemp_dn7: f64 = *var_deltemp_dn7_slot;
        let mut var_deltemp_dn8: f64 = *var_deltemp_dn8_slot;
        let mut var_deltemp_dn9: f64 = *var_deltemp_dn9_slot;
        let mut var_deltemp_rv: f64 = *var_deltemp_rv_slot;
        let mut var_gd: f64 = *var_gd_slot;
        let mut var_gd_dn0: f64 = *var_gd_dn0_slot;
        let mut var_gd_dn10: f64 = *var_gd_dn10_slot;
        let mut var_gd_dn11: f64 = *var_gd_dn11_slot;
        let mut var_gd_dn14: f64 = *var_gd_dn14_slot;
        let mut var_gd_dn2: f64 = *var_gd_dn2_slot;
        let mut var_gd_dn4: f64 = *var_gd_dn4_slot;
        let mut var_gd_dn5: f64 = *var_gd_dn5_slot;
        let mut var_gd_dn6: f64 = *var_gd_dn6_slot;
        let mut var_gd_dn7: f64 = *var_gd_dn7_slot;
        let mut var_gd_dn8: f64 = *var_gd_dn8_slot;
        let mut var_gd_dn9: f64 = *var_gd_dn9_slot;
        let mut var_gd_rv: f64 = *var_gd_rv_slot;
        let mut var_gth: f64 = *var_gth_slot;
        let mut var_gth_dn0: f64 = *var_gth_dn0_slot;
        let mut var_gth_dn10: f64 = *var_gth_dn10_slot;
        let mut var_gth_dn11: f64 = *var_gth_dn11_slot;
        let mut var_gth_dn14: f64 = *var_gth_dn14_slot;
        let mut var_gth_dn2: f64 = *var_gth_dn2_slot;
        let mut var_gth_dn4: f64 = *var_gth_dn4_slot;
        let mut var_gth_dn5: f64 = *var_gth_dn5_slot;
        let mut var_gth_dn6: f64 = *var_gth_dn6_slot;
        let mut var_gth_dn7: f64 = *var_gth_dn7_slot;
        let mut var_gth_dn8: f64 = *var_gth_dn8_slot;
        let mut var_gth_dn9: f64 = *var_gth_dn9_slot;
        let mut var_gth_rv: f64 = *var_gth_rv_slot;
        let mut var_mu0: f64 = *var_mu0_slot;
        let mut var_mu0_dn0: f64 = *var_mu0_dn0_slot;
        let mut var_mu0_dn10: f64 = *var_mu0_dn10_slot;
        let mut var_mu0_dn11: f64 = *var_mu0_dn11_slot;
        let mut var_mu0_dn14: f64 = *var_mu0_dn14_slot;
        let mut var_mu0_dn2: f64 = *var_mu0_dn2_slot;
        let mut var_mu0_dn4: f64 = *var_mu0_dn4_slot;
        let mut var_mu0_dn5: f64 = *var_mu0_dn5_slot;
        let mut var_mu0_dn6: f64 = *var_mu0_dn6_slot;
        let mut var_mu0_dn7: f64 = *var_mu0_dn7_slot;
        let mut var_mu0_dn8: f64 = *var_mu0_dn8_slot;
        let mut var_mu0_dn9: f64 = *var_mu0_dn9_slot;
        let mut var_mu0_rv: f64 = *var_mu0_rv_slot;
        let mut var_ninvde: f64 = *var_ninvde_slot;
        let mut var_ninvde_dn0: f64 = *var_ninvde_dn0_slot;
        let mut var_ninvde_dn10: f64 = *var_ninvde_dn10_slot;
        let mut var_ninvde_dn11: f64 = *var_ninvde_dn11_slot;
        let mut var_ninvde_dn14: f64 = *var_ninvde_dn14_slot;
        let mut var_ninvde_dn2: f64 = *var_ninvde_dn2_slot;
        let mut var_ninvde_dn4: f64 = *var_ninvde_dn4_slot;
        let mut var_ninvde_dn5: f64 = *var_ninvde_dn5_slot;
        let mut var_ninvde_dn6: f64 = *var_ninvde_dn6_slot;
        let mut var_ninvde_dn7: f64 = *var_ninvde_dn7_slot;
        let mut var_ninvde_dn8: f64 = *var_ninvde_dn8_slot;
        let mut var_ninvde_dn9: f64 = *var_ninvde_dn9_slot;
        let mut var_ninvde_rv: f64 = *var_ninvde_rv_slot;
        let mut var_ninvdecres: f64 = *var_ninvdecres_slot;
        let mut var_ninvdecres_dn0: f64 = *var_ninvdecres_dn0_slot;
        let mut var_ninvdecres_dn10: f64 = *var_ninvdecres_dn10_slot;
        let mut var_ninvdecres_dn11: f64 = *var_ninvdecres_dn11_slot;
        let mut var_ninvdecres_dn14: f64 = *var_ninvdecres_dn14_slot;
        let mut var_ninvdecres_dn2: f64 = *var_ninvdecres_dn2_slot;
        let mut var_ninvdecres_dn4: f64 = *var_ninvdecres_dn4_slot;
        let mut var_ninvdecres_dn5: f64 = *var_ninvdecres_dn5_slot;
        let mut var_ninvdecres_dn6: f64 = *var_ninvdecres_dn6_slot;
        let mut var_ninvdecres_dn7: f64 = *var_ninvdecres_dn7_slot;
        let mut var_ninvdecres_dn8: f64 = *var_ninvdecres_dn8_slot;
        let mut var_ninvdecres_dn9: f64 = *var_ninvdecres_dn9_slot;
        let mut var_ninvdecres_rv: f64 = *var_ninvdecres_rv_slot;
        let mut var_ninvdehres: f64 = *var_ninvdehres_slot;
        let mut var_ninvdehres_dn0: f64 = *var_ninvdehres_dn0_slot;
        let mut var_ninvdehres_dn10: f64 = *var_ninvdehres_dn10_slot;
        let mut var_ninvdehres_dn11: f64 = *var_ninvdehres_dn11_slot;
        let mut var_ninvdehres_dn14: f64 = *var_ninvdehres_dn14_slot;
        let mut var_ninvdehres_dn2: f64 = *var_ninvdehres_dn2_slot;
        let mut var_ninvdehres_dn4: f64 = *var_ninvdehres_dn4_slot;
        let mut var_ninvdehres_dn5: f64 = *var_ninvdehres_dn5_slot;
        let mut var_ninvdehres_dn6: f64 = *var_ninvdehres_dn6_slot;
        let mut var_ninvdehres_dn7: f64 = *var_ninvdehres_dn7_slot;
        let mut var_ninvdehres_dn8: f64 = *var_ninvdehres_dn8_slot;
        let mut var_ninvdehres_dn9: f64 = *var_ninvdehres_dn9_slot;
        let mut var_ninvdehres_rv: f64 = *var_ninvdehres_rv_slot;
        let mut var_p: f64 = *var_p_slot;
        let mut var_p_dn0: f64 = *var_p_dn0_slot;
        let mut var_p_dn10: f64 = *var_p_dn10_slot;
        let mut var_p_dn11: f64 = *var_p_dn11_slot;
        let mut var_p_dn14: f64 = *var_p_dn14_slot;
        let mut var_p_dn2: f64 = *var_p_dn2_slot;
        let mut var_p_dn4: f64 = *var_p_dn4_slot;
        let mut var_p_dn5: f64 = *var_p_dn5_slot;
        let mut var_p_dn6: f64 = *var_p_dn6_slot;
        let mut var_p_dn7: f64 = *var_p_dn7_slot;
        let mut var_p_dn8: f64 = *var_p_dn8_slot;
        let mut var_p_dn9: f64 = *var_p_dn9_slot;
        let mut var_p_rv: f64 = *var_p_rv_slot;
        let mut var_qb_nqs: f64 = *var_qb_nqs_slot;
        let mut var_qb_nqs_dn13: f64 = *var_qb_nqs_dn13_slot;
        let mut var_qb_nqs_rv: f64 = *var_qb_nqs_rv_slot;
        let mut var_qd_nqs: f64 = *var_qd_nqs_slot;
        let mut var_qd_nqs_dn0: f64 = *var_qd_nqs_dn0_slot;
        let mut var_qd_nqs_dn10: f64 = *var_qd_nqs_dn10_slot;
        let mut var_qd_nqs_dn11: f64 = *var_qd_nqs_dn11_slot;
        let mut var_qd_nqs_dn12: f64 = *var_qd_nqs_dn12_slot;
        let mut var_qd_nqs_dn14: f64 = *var_qd_nqs_dn14_slot;
        let mut var_qd_nqs_dn2: f64 = *var_qd_nqs_dn2_slot;
        let mut var_qd_nqs_dn4: f64 = *var_qd_nqs_dn4_slot;
        let mut var_qd_nqs_dn5: f64 = *var_qd_nqs_dn5_slot;
        let mut var_qd_nqs_dn6: f64 = *var_qd_nqs_dn6_slot;
        let mut var_qd_nqs_dn7: f64 = *var_qd_nqs_dn7_slot;
        let mut var_qd_nqs_dn8: f64 = *var_qd_nqs_dn8_slot;
        let mut var_qd_nqs_dn9: f64 = *var_qd_nqs_dn9_slot;
        let mut var_qd_nqs_rv: f64 = *var_qd_nqs_rv_slot;
        let mut var_qg: f64 = *var_qg_slot;
        let mut var_qg_dn0: f64 = *var_qg_dn0_slot;
        let mut var_qg_dn10: f64 = *var_qg_dn10_slot;
        let mut var_qg_dn11: f64 = *var_qg_dn11_slot;
        let mut var_qg_dn14: f64 = *var_qg_dn14_slot;
        let mut var_qg_dn2: f64 = *var_qg_dn2_slot;
        let mut var_qg_dn4: f64 = *var_qg_dn4_slot;
        let mut var_qg_dn5: f64 = *var_qg_dn5_slot;
        let mut var_qg_dn6: f64 = *var_qg_dn6_slot;
        let mut var_qg_dn7: f64 = *var_qg_dn7_slot;
        let mut var_qg_dn8: f64 = *var_qg_dn8_slot;
        let mut var_qg_dn9: f64 = *var_qg_dn9_slot;
        let mut var_qg_nqs: f64 = *var_qg_nqs_slot;
        let mut var_qg_nqs_dn12: f64 = *var_qg_nqs_dn12_slot;
        let mut var_qg_nqs_dn13: f64 = *var_qg_nqs_dn13_slot;
        let mut var_qg_nqs_rv: f64 = *var_qg_nqs_rv_slot;
        let mut var_qg_rv: f64 = *var_qg_rv_slot;
        let mut var_qi_nqs: f64 = *var_qi_nqs_slot;
        let mut var_qi_nqs_dn12: f64 = *var_qi_nqs_dn12_slot;
        let mut var_qi_nqs_rv: f64 = *var_qi_nqs_rv_slot;
        let mut var_qs: f64 = *var_qs_slot;
        let mut var_qs_dn0: f64 = *var_qs_dn0_slot;
        let mut var_qs_dn10: f64 = *var_qs_dn10_slot;
        let mut var_qs_dn11: f64 = *var_qs_dn11_slot;
        let mut var_qs_dn14: f64 = *var_qs_dn14_slot;
        let mut var_qs_dn2: f64 = *var_qs_dn2_slot;
        let mut var_qs_dn4: f64 = *var_qs_dn4_slot;
        let mut var_qs_dn5: f64 = *var_qs_dn5_slot;
        let mut var_qs_dn6: f64 = *var_qs_dn6_slot;
        let mut var_qs_dn7: f64 = *var_qs_dn7_slot;
        let mut var_qs_dn8: f64 = *var_qs_dn8_slot;
        let mut var_qs_dn9: f64 = *var_qs_dn9_slot;
        let mut var_qs_nqs: f64 = *var_qs_nqs_slot;
        let mut var_qs_nqs_dn0: f64 = *var_qs_nqs_dn0_slot;
        let mut var_qs_nqs_dn10: f64 = *var_qs_nqs_dn10_slot;
        let mut var_qs_nqs_dn11: f64 = *var_qs_nqs_dn11_slot;
        let mut var_qs_nqs_dn12: f64 = *var_qs_nqs_dn12_slot;
        let mut var_qs_nqs_dn14: f64 = *var_qs_nqs_dn14_slot;
        let mut var_qs_nqs_dn2: f64 = *var_qs_nqs_dn2_slot;
        let mut var_qs_nqs_dn4: f64 = *var_qs_nqs_dn4_slot;
        let mut var_qs_nqs_dn5: f64 = *var_qs_nqs_dn5_slot;
        let mut var_qs_nqs_dn6: f64 = *var_qs_nqs_dn6_slot;
        let mut var_qs_nqs_dn7: f64 = *var_qs_nqs_dn7_slot;
        let mut var_qs_nqs_dn8: f64 = *var_qs_nqs_dn8_slot;
        let mut var_qs_nqs_dn9: f64 = *var_qs_nqs_dn9_slot;
        let mut var_qs_nqs_rv: f64 = *var_qs_nqs_rv_slot;
        let mut var_qs_rv: f64 = *var_qs_rv_slot;
        let mut var_rde: f64 = *var_rde_slot;
        let mut var_rde_dn0: f64 = *var_rde_dn0_slot;
        let mut var_rde_dn10: f64 = *var_rde_dn10_slot;
        let mut var_rde_dn11: f64 = *var_rde_dn11_slot;
        let mut var_rde_dn14: f64 = *var_rde_dn14_slot;
        let mut var_rde_dn2: f64 = *var_rde_dn2_slot;
        let mut var_rde_dn4: f64 = *var_rde_dn4_slot;
        let mut var_rde_dn5: f64 = *var_rde_dn5_slot;
        let mut var_rde_dn6: f64 = *var_rde_dn6_slot;
        let mut var_rde_dn7: f64 = *var_rde_dn7_slot;
        let mut var_rde_dn8: f64 = *var_rde_dn8_slot;
        let mut var_rde_dn9: f64 = *var_rde_dn9_slot;
        let mut var_rde_rv: f64 = *var_rde_rv_slot;
        let mut var_rdvde: f64 = *var_rdvde_slot;
        let mut var_rdvde_dn0: f64 = *var_rdvde_dn0_slot;
        let mut var_rdvde_dn10: f64 = *var_rdvde_dn10_slot;
        let mut var_rdvde_dn11: f64 = *var_rdvde_dn11_slot;
        let mut var_rdvde_dn14: f64 = *var_rdvde_dn14_slot;
        let mut var_rdvde_dn2: f64 = *var_rdvde_dn2_slot;
        let mut var_rdvde_dn4: f64 = *var_rdvde_dn4_slot;
        let mut var_rdvde_dn5: f64 = *var_rdvde_dn5_slot;
        let mut var_rdvde_dn6: f64 = *var_rdvde_dn6_slot;
        let mut var_rdvde_dn7: f64 = *var_rdvde_dn7_slot;
        let mut var_rdvde_dn8: f64 = *var_rdvde_dn8_slot;
        let mut var_rdvde_dn9: f64 = *var_rdvde_dn9_slot;
        let mut var_rdvde_rv: f64 = *var_rdvde_rv_slot;
        let mut var_rrdrmue: f64 = *var_rrdrmue_slot;
        let mut var_rrdrmue_dn0: f64 = *var_rrdrmue_dn0_slot;
        let mut var_rrdrmue_dn10: f64 = *var_rrdrmue_dn10_slot;
        let mut var_rrdrmue_dn11: f64 = *var_rrdrmue_dn11_slot;
        let mut var_rrdrmue_dn14: f64 = *var_rrdrmue_dn14_slot;
        let mut var_rrdrmue_dn2: f64 = *var_rrdrmue_dn2_slot;
        let mut var_rrdrmue_dn4: f64 = *var_rrdrmue_dn4_slot;
        let mut var_rrdrmue_dn5: f64 = *var_rrdrmue_dn5_slot;
        let mut var_rrdrmue_dn6: f64 = *var_rrdrmue_dn6_slot;
        let mut var_rrdrmue_dn7: f64 = *var_rrdrmue_dn7_slot;
        let mut var_rrdrmue_dn8: f64 = *var_rrdrmue_dn8_slot;
        let mut var_rrdrmue_dn9: f64 = *var_rrdrmue_dn9_slot;
        let mut var_rrdrmue_rv: f64 = *var_rrdrmue_rv_slot;
        let mut var_rrdrmues: f64 = *var_rrdrmues_slot;
        let mut var_rrdrmues_dn0: f64 = *var_rrdrmues_dn0_slot;
        let mut var_rrdrmues_dn10: f64 = *var_rrdrmues_dn10_slot;
        let mut var_rrdrmues_dn11: f64 = *var_rrdrmues_dn11_slot;
        let mut var_rrdrmues_dn14: f64 = *var_rrdrmues_dn14_slot;
        let mut var_rrdrmues_dn2: f64 = *var_rrdrmues_dn2_slot;
        let mut var_rrdrmues_dn4: f64 = *var_rrdrmues_dn4_slot;
        let mut var_rrdrmues_dn5: f64 = *var_rrdrmues_dn5_slot;
        let mut var_rrdrmues_dn6: f64 = *var_rrdrmues_dn6_slot;
        let mut var_rrdrmues_dn7: f64 = *var_rrdrmues_dn7_slot;
        let mut var_rrdrmues_dn8: f64 = *var_rrdrmues_dn8_slot;
        let mut var_rrdrmues_dn9: f64 = *var_rrdrmues_dn9_slot;
        let mut var_rrdrmues_rv: f64 = *var_rrdrmues_rv_slot;
        let mut var_rrdrvmax: f64 = *var_rrdrvmax_slot;
        let mut var_rrdrvmax_dn0: f64 = *var_rrdrvmax_dn0_slot;
        let mut var_rrdrvmax_dn10: f64 = *var_rrdrvmax_dn10_slot;
        let mut var_rrdrvmax_dn11: f64 = *var_rrdrvmax_dn11_slot;
        let mut var_rrdrvmax_dn14: f64 = *var_rrdrvmax_dn14_slot;
        let mut var_rrdrvmax_dn2: f64 = *var_rrdrvmax_dn2_slot;
        let mut var_rrdrvmax_dn4: f64 = *var_rrdrvmax_dn4_slot;
        let mut var_rrdrvmax_dn5: f64 = *var_rrdrvmax_dn5_slot;
        let mut var_rrdrvmax_dn6: f64 = *var_rrdrvmax_dn6_slot;
        let mut var_rrdrvmax_dn7: f64 = *var_rrdrvmax_dn7_slot;
        let mut var_rrdrvmax_dn8: f64 = *var_rrdrvmax_dn8_slot;
        let mut var_rrdrvmax_dn9: f64 = *var_rrdrvmax_dn9_slot;
        let mut var_rrdrvmax_rv: f64 = *var_rrdrvmax_rv_slot;
        let mut var_rrdrvmaxs: f64 = *var_rrdrvmaxs_slot;
        let mut var_rrdrvmaxs_dn0: f64 = *var_rrdrvmaxs_dn0_slot;
        let mut var_rrdrvmaxs_dn10: f64 = *var_rrdrvmaxs_dn10_slot;
        let mut var_rrdrvmaxs_dn11: f64 = *var_rrdrvmaxs_dn11_slot;
        let mut var_rrdrvmaxs_dn14: f64 = *var_rrdrvmaxs_dn14_slot;
        let mut var_rrdrvmaxs_dn2: f64 = *var_rrdrvmaxs_dn2_slot;
        let mut var_rrdrvmaxs_dn4: f64 = *var_rrdrvmaxs_dn4_slot;
        let mut var_rrdrvmaxs_dn5: f64 = *var_rrdrvmaxs_dn5_slot;
        let mut var_rrdrvmaxs_dn6: f64 = *var_rrdrvmaxs_dn6_slot;
        let mut var_rrdrvmaxs_dn7: f64 = *var_rrdrvmaxs_dn7_slot;
        let mut var_rrdrvmaxs_dn8: f64 = *var_rrdrvmaxs_dn8_slot;
        let mut var_rrdrvmaxs_dn9: f64 = *var_rrdrvmaxs_dn9_slot;
        let mut var_rrdrvmaxs_rv: f64 = *var_rrdrvmaxs_rv_slot;
        let mut var_rse: f64 = *var_rse_slot;
        let mut var_rse_dn0: f64 = *var_rse_dn0_slot;
        let mut var_rse_dn10: f64 = *var_rse_dn10_slot;
        let mut var_rse_dn11: f64 = *var_rse_dn11_slot;
        let mut var_rse_dn14: f64 = *var_rse_dn14_slot;
        let mut var_rse_dn2: f64 = *var_rse_dn2_slot;
        let mut var_rse_dn4: f64 = *var_rse_dn4_slot;
        let mut var_rse_dn5: f64 = *var_rse_dn5_slot;
        let mut var_rse_dn6: f64 = *var_rse_dn6_slot;
        let mut var_rse_dn7: f64 = *var_rse_dn7_slot;
        let mut var_rse_dn8: f64 = *var_rse_dn8_slot;
        let mut var_rse_dn9: f64 = *var_rse_dn9_slot;
        let mut var_rse_rv: f64 = *var_rse_rv_slot;
        let mut var_rsvde: f64 = *var_rsvde_slot;
        let mut var_rsvde_dn0: f64 = *var_rsvde_dn0_slot;
        let mut var_rsvde_dn10: f64 = *var_rsvde_dn10_slot;
        let mut var_rsvde_dn11: f64 = *var_rsvde_dn11_slot;
        let mut var_rsvde_dn14: f64 = *var_rsvde_dn14_slot;
        let mut var_rsvde_dn2: f64 = *var_rsvde_dn2_slot;
        let mut var_rsvde_dn4: f64 = *var_rsvde_dn4_slot;
        let mut var_rsvde_dn5: f64 = *var_rsvde_dn5_slot;
        let mut var_rsvde_dn6: f64 = *var_rsvde_dn6_slot;
        let mut var_rsvde_dn7: f64 = *var_rsvde_dn7_slot;
        let mut var_rsvde_dn8: f64 = *var_rsvde_dn8_slot;
        let mut var_rsvde_dn9: f64 = *var_rsvde_dn9_slot;
        let mut var_rsvde_rv: f64 = *var_rsvde_rv_slot;
        let mut var_tratio: f64 = *var_tratio_slot;
        let mut var_tratio_dn0: f64 = *var_tratio_dn0_slot;
        let mut var_tratio_dn10: f64 = *var_tratio_dn10_slot;
        let mut var_tratio_dn11: f64 = *var_tratio_dn11_slot;
        let mut var_tratio_dn14: f64 = *var_tratio_dn14_slot;
        let mut var_tratio_dn2: f64 = *var_tratio_dn2_slot;
        let mut var_tratio_dn4: f64 = *var_tratio_dn4_slot;
        let mut var_tratio_dn5: f64 = *var_tratio_dn5_slot;
        let mut var_tratio_dn6: f64 = *var_tratio_dn6_slot;
        let mut var_tratio_dn7: f64 = *var_tratio_dn7_slot;
        let mut var_tratio_dn8: f64 = *var_tratio_dn8_slot;
        let mut var_tratio_dn9: f64 = *var_tratio_dn9_slot;
        let mut var_tratio_rv: f64 = *var_tratio_rv_slot;
        let mut var_vbd: f64 = *var_vbd_slot;
        let mut var_vbd_dn6: f64 = *var_vbd_dn6_slot;
        let mut var_vbd_dn8: f64 = *var_vbd_dn8_slot;
        let mut var_vbd_dn9: f64 = *var_vbd_dn9_slot;
        let mut var_vbd_rv: f64 = *var_vbd_rv_slot;
        let mut var_vbsei: f64 = *var_vbsei_slot;
        let mut var_vbsei_dn2: f64 = *var_vbsei_dn2_slot;
        let mut var_vbsei_dn9: f64 = *var_vbsei_dn9_slot;
        let mut var_vbsei_rv: f64 = *var_vbsei_rv_slot;
        let mut var_vbsi: f64 = *var_vbsi_slot;
        let mut var_vbsi_dn8: f64 = *var_vbsi_dn8_slot;
        let mut var_vbsi_dn9: f64 = *var_vbsi_dn9_slot;
        let mut var_vbsi_rv: f64 = *var_vbsi_rv_slot;
        let mut var_vddpz: f64 = *var_vddpz_slot;
        let mut var_vddpz_dn0: f64 = *var_vddpz_dn0_slot;
        let mut var_vddpz_dn10: f64 = *var_vddpz_dn10_slot;
        let mut var_vddpz_dn11: f64 = *var_vddpz_dn11_slot;
        let mut var_vddpz_dn14: f64 = *var_vddpz_dn14_slot;
        let mut var_vddpz_dn2: f64 = *var_vddpz_dn2_slot;
        let mut var_vddpz_dn4: f64 = *var_vddpz_dn4_slot;
        let mut var_vddpz_dn5: f64 = *var_vddpz_dn5_slot;
        let mut var_vddpz_dn6: f64 = *var_vddpz_dn6_slot;
        let mut var_vddpz_dn7: f64 = *var_vddpz_dn7_slot;
        let mut var_vddpz_dn8: f64 = *var_vddpz_dn8_slot;
        let mut var_vddpz_dn9: f64 = *var_vddpz_dn9_slot;
        let mut var_vddpz_rv: f64 = *var_vddpz_rv_slot;
        let mut var_vdri: f64 = *var_vdri_slot;
        let mut var_vdri_dn0: f64 = *var_vdri_dn0_slot;
        let mut var_vdri_dn10: f64 = *var_vdri_dn10_slot;
        let mut var_vdri_dn11: f64 = *var_vdri_dn11_slot;
        let mut var_vdri_dn14: f64 = *var_vdri_dn14_slot;
        let mut var_vdri_dn2: f64 = *var_vdri_dn2_slot;
        let mut var_vdri_dn4: f64 = *var_vdri_dn4_slot;
        let mut var_vdri_dn5: f64 = *var_vdri_dn5_slot;
        let mut var_vdri_dn6: f64 = *var_vdri_dn6_slot;
        let mut var_vdri_dn7: f64 = *var_vdri_dn7_slot;
        let mut var_vdri_dn8: f64 = *var_vdri_dn8_slot;
        let mut var_vdri_dn9: f64 = *var_vdri_dn9_slot;
        let mut var_vdri_rv: f64 = *var_vdri_rv_slot;
        let mut var_vdsei: f64 = *var_vdsei_slot;
        let mut var_vdsei_dn0: f64 = *var_vdsei_dn0_slot;
        let mut var_vdsei_dn2: f64 = *var_vdsei_dn2_slot;
        let mut var_vdsei_rv: f64 = *var_vdsei_rv_slot;
        let mut var_vdsi: f64 = *var_vdsi_slot;
        let mut var_vdsi_dn6: f64 = *var_vdsi_dn6_slot;
        let mut var_vdsi_dn8: f64 = *var_vdsi_dn8_slot;
        let mut var_vdsi_rv: f64 = *var_vdsi_rv_slot;
        let mut var_veffpower: f64 = *var_veffpower_slot;
        let mut var_veffpower_dn0: f64 = *var_veffpower_dn0_slot;
        let mut var_veffpower_dn10: f64 = *var_veffpower_dn10_slot;
        let mut var_veffpower_dn11: f64 = *var_veffpower_dn11_slot;
        let mut var_veffpower_dn14: f64 = *var_veffpower_dn14_slot;
        let mut var_veffpower_dn2: f64 = *var_veffpower_dn2_slot;
        let mut var_veffpower_dn4: f64 = *var_veffpower_dn4_slot;
        let mut var_veffpower_dn5: f64 = *var_veffpower_dn5_slot;
        let mut var_veffpower_dn6: f64 = *var_veffpower_dn6_slot;
        let mut var_veffpower_dn7: f64 = *var_veffpower_dn7_slot;
        let mut var_veffpower_dn8: f64 = *var_veffpower_dn8_slot;
        let mut var_veffpower_dn9: f64 = *var_veffpower_dn9_slot;
        let mut var_veffpower_rv: f64 = *var_veffpower_rv_slot;
        let mut var_vgd: f64 = *var_vgd_slot;
        let mut var_vgd_dn6: f64 = *var_vgd_dn6_slot;
        let mut var_vgd_dn7: f64 = *var_vgd_dn7_slot;
        let mut var_vgd_dn8: f64 = *var_vgd_dn8_slot;
        let mut var_vgd_rv: f64 = *var_vgd_rv_slot;
        let mut var_vgsei: f64 = *var_vgsei_slot;
        let mut var_vgsei_dn2: f64 = *var_vgsei_dn2_slot;
        let mut var_vgsei_dn7: f64 = *var_vgsei_dn7_slot;
        let mut var_vgsei_rv: f64 = *var_vgsei_rv_slot;
        let mut var_vgsi: f64 = *var_vgsi_slot;
        let mut var_vgsi_dn7: f64 = *var_vgsi_dn7_slot;
        let mut var_vgsi_dn8: f64 = *var_vgsi_dn8_slot;
        let mut var_vgsi_rv: f64 = *var_vgsi_rv_slot;
        let mut var_vmaxeff: f64 = *var_vmaxeff_slot;
        let mut var_vmaxeff_dn0: f64 = *var_vmaxeff_dn0_slot;
        let mut var_vmaxeff_dn10: f64 = *var_vmaxeff_dn10_slot;
        let mut var_vmaxeff_dn11: f64 = *var_vmaxeff_dn11_slot;
        let mut var_vmaxeff_dn14: f64 = *var_vmaxeff_dn14_slot;
        let mut var_vmaxeff_dn2: f64 = *var_vmaxeff_dn2_slot;
        let mut var_vmaxeff_dn4: f64 = *var_vmaxeff_dn4_slot;
        let mut var_vmaxeff_dn5: f64 = *var_vmaxeff_dn5_slot;
        let mut var_vmaxeff_dn6: f64 = *var_vmaxeff_dn6_slot;
        let mut var_vmaxeff_dn7: f64 = *var_vmaxeff_dn7_slot;
        let mut var_vmaxeff_dn8: f64 = *var_vmaxeff_dn8_slot;
        let mut var_vmaxeff_dn9: f64 = *var_vmaxeff_dn9_slot;
        let mut var_vmaxeff_rv: f64 = *var_vmaxeff_rv_slot;
        let mut var_xov: f64 = *var_xov_slot;
        let mut var_xov_dn0: f64 = *var_xov_dn0_slot;
        let mut var_xov_dn10: f64 = *var_xov_dn10_slot;
        let mut var_xov_dn11: f64 = *var_xov_dn11_slot;
        let mut var_xov_dn14: f64 = *var_xov_dn14_slot;
        let mut var_xov_dn2: f64 = *var_xov_dn2_slot;
        let mut var_xov_dn4: f64 = *var_xov_dn4_slot;
        let mut var_xov_dn5: f64 = *var_xov_dn5_slot;
        let mut var_xov_dn6: f64 = *var_xov_dn6_slot;
        let mut var_xov_dn7: f64 = *var_xov_dn7_slot;
        let mut var_xov_dn8: f64 = *var_xov_dn8_slot;
        let mut var_xov_dn9: f64 = *var_xov_dn9_slot;
        let mut var_xov_rv: f64 = *var_xov_rv_slot;

        var_vdri = 0.0;
        var_vdri_dn0 = 0.0;
        var_vdri_dn2 = 0.0;
        var_vdri_dn4 = 0.0;
        var_vdri_dn5 = 0.0;
        var_vdri_dn6 = 0.0;
        var_vdri_dn7 = 0.0;
        var_vdri_dn8 = 0.0;
        var_vdri_dn9 = 0.0;
        var_vdri_dn10 = 0.0;
        var_vdri_dn11 = 0.0;
        var_vdri_dn14 = 0.0;
        var_vdri_rv = 0.0;

        var_mu0 = 0.0;
        var_mu0_dn0 = 0.0;
        var_mu0_dn2 = 0.0;
        var_mu0_dn4 = 0.0;
        var_mu0_dn5 = 0.0;
        var_mu0_dn6 = 0.0;
        var_mu0_dn7 = 0.0;
        var_mu0_dn8 = 0.0;
        var_mu0_dn9 = 0.0;
        var_mu0_dn10 = 0.0;
        var_mu0_dn11 = 0.0;
        var_mu0_dn14 = 0.0;
        var_mu0_rv = 0.0;

        var_cx = 0.0;
        var_cx_rv = 0.0;

        var_car = 0.0;
        var_car_rv = 0.0;

        var_xov = 0.0;
        var_xov_dn0 = 0.0;
        var_xov_dn2 = 0.0;
        var_xov_dn4 = 0.0;
        var_xov_dn5 = 0.0;
        var_xov_dn6 = 0.0;
        var_xov_dn7 = 0.0;
        var_xov_dn8 = 0.0;
        var_xov_dn9 = 0.0;
        var_xov_dn10 = 0.0;
        var_xov_dn11 = 0.0;
        var_xov_dn14 = 0.0;
        var_xov_rv = 0.0;

        var_carr = 0.0;
        var_carr_dn0 = 0.0;
        var_carr_dn2 = 0.0;
        var_carr_dn4 = 0.0;
        var_carr_dn5 = 0.0;
        var_carr_dn6 = 0.0;
        var_carr_dn7 = 0.0;
        var_carr_dn8 = 0.0;
        var_carr_dn9 = 0.0;
        var_carr_dn10 = 0.0;
        var_carr_dn11 = 0.0;
        var_carr_dn14 = 0.0;
        var_carr_rv = 0.0;

        var_gd = 0.0;
        var_gd_dn0 = 0.0;
        var_gd_dn2 = 0.0;
        var_gd_dn4 = 0.0;
        var_gd_dn5 = 0.0;
        var_gd_dn6 = 0.0;
        var_gd_dn7 = 0.0;
        var_gd_dn8 = 0.0;
        var_gd_dn9 = 0.0;
        var_gd_dn10 = 0.0;
        var_gd_dn11 = 0.0;
        var_gd_dn14 = 0.0;
        var_gd_rv = 0.0;

        var_vddpz = 0.0;
        var_vddpz_dn0 = 0.0;
        var_vddpz_dn2 = 0.0;
        var_vddpz_dn4 = 0.0;
        var_vddpz_dn5 = 0.0;
        var_vddpz_dn6 = 0.0;
        var_vddpz_dn7 = 0.0;
        var_vddpz_dn8 = 0.0;
        var_vddpz_dn9 = 0.0;
        var_vddpz_dn10 = 0.0;
        var_vddpz_dn11 = 0.0;
        var_vddpz_dn14 = 0.0;
        var_vddpz_rv = 0.0;

        var_arg = 0.0;
        var_arg_dn0 = 0.0;
        var_arg_dn2 = 0.0;
        var_arg_dn4 = 0.0;
        var_arg_dn5 = 0.0;
        var_arg_dn6 = 0.0;
        var_arg_dn7 = 0.0;
        var_arg_dn8 = 0.0;
        var_arg_dn9 = 0.0;
        var_arg_dn10 = 0.0;
        var_arg_dn11 = 0.0;
        var_arg_dn14 = 0.0;
        var_arg_rv = 0.0;

        var_vbd = 0.0;
        var_vbd_dn6 = 0.0;
        var_vbd_dn8 = 0.0;
        var_vbd_dn9 = 0.0;
        var_vbd_rv = 0.0;

        var_vbsi = 0.0;
        var_vbsi_dn8 = 0.0;
        var_vbsi_dn9 = 0.0;
        var_vbsi_rv = 0.0;

        var_vdsi = 0.0;
        var_vdsi_dn6 = 0.0;
        var_vdsi_dn8 = 0.0;
        var_vdsi_rv = 0.0;

        var_vgd = 0.0;
        var_vgd_dn6 = 0.0;
        var_vgd_dn7 = 0.0;
        var_vgd_dn8 = 0.0;
        var_vgd_rv = 0.0;

        var_vgsi = 0.0;
        var_vgsi_dn7 = 0.0;
        var_vgsi_dn8 = 0.0;
        var_vgsi_rv = 0.0;

        var_deltemp = 0.0;
        var_deltemp_dn0 = 0.0;
        var_deltemp_dn2 = 0.0;
        var_deltemp_dn4 = 0.0;
        var_deltemp_dn5 = 0.0;
        var_deltemp_dn6 = 0.0;
        var_deltemp_dn7 = 0.0;
        var_deltemp_dn8 = 0.0;
        var_deltemp_dn9 = 0.0;
        var_deltemp_dn10 = 0.0;
        var_deltemp_dn11 = 0.0;
        var_deltemp_dn14 = 0.0;
        var_deltemp_rv = 0.0;

        var_vdsei = 0.0;
        var_vdsei_dn0 = 0.0;
        var_vdsei_dn2 = 0.0;
        var_vdsei_rv = 0.0;

        var_vgsei = 0.0;
        var_vgsei_dn2 = 0.0;
        var_vgsei_dn7 = 0.0;
        var_vgsei_rv = 0.0;

        var_vbsei = 0.0;
        var_vbsei_dn2 = 0.0;
        var_vbsei_dn9 = 0.0;
        var_vbsei_rv = 0.0;

        var_gth = 0.0;
        var_gth_dn0 = 0.0;
        var_gth_dn2 = 0.0;
        var_gth_dn4 = 0.0;
        var_gth_dn5 = 0.0;
        var_gth_dn6 = 0.0;
        var_gth_dn7 = 0.0;
        var_gth_dn8 = 0.0;
        var_gth_dn9 = 0.0;
        var_gth_dn10 = 0.0;
        var_gth_dn11 = 0.0;
        var_gth_dn14 = 0.0;
        var_gth_rv = 0.0;

        var_qg = 0.0;
        var_qg_dn0 = 0.0;
        var_qg_dn2 = 0.0;
        var_qg_dn4 = 0.0;
        var_qg_dn5 = 0.0;
        var_qg_dn6 = 0.0;
        var_qg_dn7 = 0.0;
        var_qg_dn8 = 0.0;
        var_qg_dn9 = 0.0;
        var_qg_dn10 = 0.0;
        var_qg_dn11 = 0.0;
        var_qg_dn14 = 0.0;
        var_qg_rv = 0.0;

        var_qs = 0.0;
        var_qs_dn0 = 0.0;
        var_qs_dn2 = 0.0;
        var_qs_dn4 = 0.0;
        var_qs_dn5 = 0.0;
        var_qs_dn6 = 0.0;
        var_qs_dn7 = 0.0;
        var_qs_dn8 = 0.0;
        var_qs_dn9 = 0.0;
        var_qs_dn10 = 0.0;
        var_qs_dn11 = 0.0;
        var_qs_dn14 = 0.0;
        var_qs_rv = 0.0;

        var_veffpower = 0.0;
        var_veffpower_dn0 = 0.0;
        var_veffpower_dn2 = 0.0;
        var_veffpower_dn4 = 0.0;
        var_veffpower_dn5 = 0.0;
        var_veffpower_dn6 = 0.0;
        var_veffpower_dn7 = 0.0;
        var_veffpower_dn8 = 0.0;
        var_veffpower_dn9 = 0.0;
        var_veffpower_dn10 = 0.0;
        var_veffpower_dn11 = 0.0;
        var_veffpower_dn14 = 0.0;
        var_veffpower_rv = 0.0;

        var_p = 0.0;
        var_p_dn0 = 0.0;
        var_p_dn2 = 0.0;
        var_p_dn4 = 0.0;
        var_p_dn5 = 0.0;
        var_p_dn6 = 0.0;
        var_p_dn7 = 0.0;
        var_p_dn8 = 0.0;
        var_p_dn9 = 0.0;
        var_p_dn10 = 0.0;
        var_p_dn11 = 0.0;
        var_p_dn14 = 0.0;
        var_p_rv = 0.0;

        var_qi_nqs = 0.0;
        var_qi_nqs_dn12 = 0.0;
        var_qi_nqs_rv = 0.0;

        var_qb_nqs = 0.0;
        var_qb_nqs_dn13 = 0.0;
        var_qb_nqs_rv = 0.0;

        var_qd_nqs = 0.0;
        var_qd_nqs_dn0 = 0.0;
        var_qd_nqs_dn2 = 0.0;
        var_qd_nqs_dn4 = 0.0;
        var_qd_nqs_dn5 = 0.0;
        var_qd_nqs_dn6 = 0.0;
        var_qd_nqs_dn7 = 0.0;
        var_qd_nqs_dn8 = 0.0;
        var_qd_nqs_dn9 = 0.0;
        var_qd_nqs_dn10 = 0.0;
        var_qd_nqs_dn11 = 0.0;
        var_qd_nqs_dn12 = 0.0;
        var_qd_nqs_dn14 = 0.0;
        var_qd_nqs_rv = 0.0;

        var_qs_nqs = 0.0;
        var_qs_nqs_dn0 = 0.0;
        var_qs_nqs_dn2 = 0.0;
        var_qs_nqs_dn4 = 0.0;
        var_qs_nqs_dn5 = 0.0;
        var_qs_nqs_dn6 = 0.0;
        var_qs_nqs_dn7 = 0.0;
        var_qs_nqs_dn8 = 0.0;
        var_qs_nqs_dn9 = 0.0;
        var_qs_nqs_dn10 = 0.0;
        var_qs_nqs_dn11 = 0.0;
        var_qs_nqs_dn12 = 0.0;
        var_qs_nqs_dn14 = 0.0;
        var_qs_nqs_rv = 0.0;

        var_qg_nqs = 0.0;
        var_qg_nqs_dn12 = 0.0;
        var_qg_nqs_dn13 = 0.0;
        var_qg_nqs_rv = 0.0;

        var_cgsb = 0.0;
        var_cgsb_dn0 = 0.0;
        var_cgsb_dn2 = 0.0;
        var_cgsb_dn4 = 0.0;
        var_cgsb_dn5 = 0.0;
        var_cgsb_dn6 = 0.0;
        var_cgsb_dn7 = 0.0;
        var_cgsb_dn8 = 0.0;
        var_cgsb_dn9 = 0.0;
        var_cgsb_dn10 = 0.0;
        var_cgsb_dn11 = 0.0;
        var_cgsb_dn14 = 0.0;
        var_cgsb_rv = 0.0;

        var_ninvde = 0.0;
        var_ninvde_dn0 = 0.0;
        var_ninvde_dn2 = 0.0;
        var_ninvde_dn4 = 0.0;
        var_ninvde_dn5 = 0.0;
        var_ninvde_dn6 = 0.0;
        var_ninvde_dn7 = 0.0;
        var_ninvde_dn8 = 0.0;
        var_ninvde_dn9 = 0.0;
        var_ninvde_dn10 = 0.0;
        var_ninvde_dn11 = 0.0;
        var_ninvde_dn14 = 0.0;
        var_ninvde_rv = 0.0;

        var_ninvdecres = 0.0;
        var_ninvdecres_dn0 = 0.0;
        var_ninvdecres_dn2 = 0.0;
        var_ninvdecres_dn4 = 0.0;
        var_ninvdecres_dn5 = 0.0;
        var_ninvdecres_dn6 = 0.0;
        var_ninvdecres_dn7 = 0.0;
        var_ninvdecres_dn8 = 0.0;
        var_ninvdecres_dn9 = 0.0;
        var_ninvdecres_dn10 = 0.0;
        var_ninvdecres_dn11 = 0.0;
        var_ninvdecres_dn14 = 0.0;
        var_ninvdecres_rv = 0.0;

        var_ninvdehres = 0.0;
        var_ninvdehres_dn0 = 0.0;
        var_ninvdehres_dn2 = 0.0;
        var_ninvdehres_dn4 = 0.0;
        var_ninvdehres_dn5 = 0.0;
        var_ninvdehres_dn6 = 0.0;
        var_ninvdehres_dn7 = 0.0;
        var_ninvdehres_dn8 = 0.0;
        var_ninvdehres_dn9 = 0.0;
        var_ninvdehres_dn10 = 0.0;
        var_ninvdehres_dn11 = 0.0;
        var_ninvdehres_dn14 = 0.0;
        var_ninvdehres_rv = 0.0;

        var_rrdrmue = 0.0;
        var_rrdrmue_dn0 = 0.0;
        var_rrdrmue_dn2 = 0.0;
        var_rrdrmue_dn4 = 0.0;
        var_rrdrmue_dn5 = 0.0;
        var_rrdrmue_dn6 = 0.0;
        var_rrdrmue_dn7 = 0.0;
        var_rrdrmue_dn8 = 0.0;
        var_rrdrmue_dn9 = 0.0;
        var_rrdrmue_dn10 = 0.0;
        var_rrdrmue_dn11 = 0.0;
        var_rrdrmue_dn14 = 0.0;
        var_rrdrmue_rv = 0.0;

        var_rrdrmues = 0.0;
        var_rrdrmues_dn0 = 0.0;
        var_rrdrmues_dn2 = 0.0;
        var_rrdrmues_dn4 = 0.0;
        var_rrdrmues_dn5 = 0.0;
        var_rrdrmues_dn6 = 0.0;
        var_rrdrmues_dn7 = 0.0;
        var_rrdrmues_dn8 = 0.0;
        var_rrdrmues_dn9 = 0.0;
        var_rrdrmues_dn10 = 0.0;
        var_rrdrmues_dn11 = 0.0;
        var_rrdrmues_dn14 = 0.0;
        var_rrdrmues_rv = 0.0;

        var_rrdrvmax = 0.0;
        var_rrdrvmax_dn0 = 0.0;
        var_rrdrvmax_dn2 = 0.0;
        var_rrdrvmax_dn4 = 0.0;
        var_rrdrvmax_dn5 = 0.0;
        var_rrdrvmax_dn6 = 0.0;
        var_rrdrvmax_dn7 = 0.0;
        var_rrdrvmax_dn8 = 0.0;
        var_rrdrvmax_dn9 = 0.0;
        var_rrdrvmax_dn10 = 0.0;
        var_rrdrvmax_dn11 = 0.0;
        var_rrdrvmax_dn14 = 0.0;
        var_rrdrvmax_rv = 0.0;

        var_rde = 0.0;
        var_rde_dn0 = 0.0;
        var_rde_dn2 = 0.0;
        var_rde_dn4 = 0.0;
        var_rde_dn5 = 0.0;
        var_rde_dn6 = 0.0;
        var_rde_dn7 = 0.0;
        var_rde_dn8 = 0.0;
        var_rde_dn9 = 0.0;
        var_rde_dn10 = 0.0;
        var_rde_dn11 = 0.0;
        var_rde_dn14 = 0.0;
        var_rde_rv = 0.0;

        var_rdvde = 0.0;
        var_rdvde_dn0 = 0.0;
        var_rdvde_dn2 = 0.0;
        var_rdvde_dn4 = 0.0;
        var_rdvde_dn5 = 0.0;
        var_rdvde_dn6 = 0.0;
        var_rdvde_dn7 = 0.0;
        var_rdvde_dn8 = 0.0;
        var_rdvde_dn9 = 0.0;
        var_rdvde_dn10 = 0.0;
        var_rdvde_dn11 = 0.0;
        var_rdvde_dn14 = 0.0;
        var_rdvde_rv = 0.0;

        var_rse = 0.0;
        var_rse_dn0 = 0.0;
        var_rse_dn2 = 0.0;
        var_rse_dn4 = 0.0;
        var_rse_dn5 = 0.0;
        var_rse_dn6 = 0.0;
        var_rse_dn7 = 0.0;
        var_rse_dn8 = 0.0;
        var_rse_dn9 = 0.0;
        var_rse_dn10 = 0.0;
        var_rse_dn11 = 0.0;
        var_rse_dn14 = 0.0;
        var_rse_rv = 0.0;

        var_rsvde = 0.0;
        var_rsvde_dn0 = 0.0;
        var_rsvde_dn2 = 0.0;
        var_rsvde_dn4 = 0.0;
        var_rsvde_dn5 = 0.0;
        var_rsvde_dn6 = 0.0;
        var_rsvde_dn7 = 0.0;
        var_rsvde_dn8 = 0.0;
        var_rsvde_dn9 = 0.0;
        var_rsvde_dn10 = 0.0;
        var_rsvde_dn11 = 0.0;
        var_rsvde_dn14 = 0.0;
        var_rsvde_rv = 0.0;

        var_rrdrvmaxs = 0.0;
        var_rrdrvmaxs_dn0 = 0.0;
        var_rrdrvmaxs_dn2 = 0.0;
        var_rrdrvmaxs_dn4 = 0.0;
        var_rrdrvmaxs_dn5 = 0.0;
        var_rrdrvmaxs_dn6 = 0.0;
        var_rrdrvmaxs_dn7 = 0.0;
        var_rrdrvmaxs_dn8 = 0.0;
        var_rrdrvmaxs_dn9 = 0.0;
        var_rrdrvmaxs_dn10 = 0.0;
        var_rrdrvmaxs_dn11 = 0.0;
        var_rrdrvmaxs_dn14 = 0.0;
        var_rrdrvmaxs_rv = 0.0;

        var_tratio = 0.0;
        var_tratio_dn0 = 0.0;
        var_tratio_dn2 = 0.0;
        var_tratio_dn4 = 0.0;
        var_tratio_dn5 = 0.0;
        var_tratio_dn6 = 0.0;
        var_tratio_dn7 = 0.0;
        var_tratio_dn8 = 0.0;
        var_tratio_dn9 = 0.0;
        var_tratio_dn10 = 0.0;
        var_tratio_dn11 = 0.0;
        var_tratio_dn14 = 0.0;
        var_tratio_rv = 0.0;

        var_vmaxeff = 0.0;
        var_vmaxeff_dn0 = 0.0;
        var_vmaxeff_dn2 = 0.0;
        var_vmaxeff_dn4 = 0.0;
        var_vmaxeff_dn5 = 0.0;
        var_vmaxeff_dn6 = 0.0;
        var_vmaxeff_dn7 = 0.0;
        var_vmaxeff_dn8 = 0.0;
        var_vmaxeff_dn9 = 0.0;
        var_vmaxeff_dn10 = 0.0;
        var_vmaxeff_dn11 = 0.0;
        var_vmaxeff_dn14 = 0.0;
        var_vmaxeff_rv = 0.0;

        var_betatnom = 0.0;
        var_betatnom_rv = 0.0;

        var_cnst0over = 0.0;
        var_cnst0over_dn0 = 0.0;
        var_cnst0over_dn2 = 0.0;
        var_cnst0over_dn4 = 0.0;
        var_cnst0over_dn5 = 0.0;
        var_cnst0over_dn6 = 0.0;
        var_cnst0over_dn7 = 0.0;
        var_cnst0over_dn8 = 0.0;
        var_cnst0over_dn9 = 0.0;
        var_cnst0over_dn10 = 0.0;
        var_cnst0over_dn11 = 0.0;
        var_cnst0over_dn14 = 0.0;
        var_cnst0over_rv = 0.0;

        var_cnst0overs = 0.0;
        var_cnst0overs_dn0 = 0.0;
        var_cnst0overs_dn2 = 0.0;
        var_cnst0overs_dn4 = 0.0;
        var_cnst0overs_dn5 = 0.0;
        var_cnst0overs_dn6 = 0.0;
        var_cnst0overs_dn7 = 0.0;
        var_cnst0overs_dn8 = 0.0;
        var_cnst0overs_dn9 = 0.0;
        var_cnst0overs_dn10 = 0.0;
        var_cnst0overs_dn11 = 0.0;
        var_cnst0overs_dn14 = 0.0;
        var_cnst0overs_rv = 0.0;

        *var_arg_slot = var_arg;
        *var_arg_dn0_slot = var_arg_dn0;
        *var_arg_dn10_slot = var_arg_dn10;
        *var_arg_dn11_slot = var_arg_dn11;
        *var_arg_dn14_slot = var_arg_dn14;
        *var_arg_dn2_slot = var_arg_dn2;
        *var_arg_dn4_slot = var_arg_dn4;
        *var_arg_dn5_slot = var_arg_dn5;
        *var_arg_dn6_slot = var_arg_dn6;
        *var_arg_dn7_slot = var_arg_dn7;
        *var_arg_dn8_slot = var_arg_dn8;
        *var_arg_dn9_slot = var_arg_dn9;
        *var_arg_rv_slot = var_arg_rv;
        *var_betatnom_slot = var_betatnom;
        *var_betatnom_rv_slot = var_betatnom_rv;
        *var_car_slot = var_car;
        *var_car_rv_slot = var_car_rv;
        *var_carr_slot = var_carr;
        *var_carr_dn0_slot = var_carr_dn0;
        *var_carr_dn10_slot = var_carr_dn10;
        *var_carr_dn11_slot = var_carr_dn11;
        *var_carr_dn14_slot = var_carr_dn14;
        *var_carr_dn2_slot = var_carr_dn2;
        *var_carr_dn4_slot = var_carr_dn4;
        *var_carr_dn5_slot = var_carr_dn5;
        *var_carr_dn6_slot = var_carr_dn6;
        *var_carr_dn7_slot = var_carr_dn7;
        *var_carr_dn8_slot = var_carr_dn8;
        *var_carr_dn9_slot = var_carr_dn9;
        *var_carr_rv_slot = var_carr_rv;
        *var_cgsb_slot = var_cgsb;
        *var_cgsb_dn0_slot = var_cgsb_dn0;
        *var_cgsb_dn10_slot = var_cgsb_dn10;
        *var_cgsb_dn11_slot = var_cgsb_dn11;
        *var_cgsb_dn14_slot = var_cgsb_dn14;
        *var_cgsb_dn2_slot = var_cgsb_dn2;
        *var_cgsb_dn4_slot = var_cgsb_dn4;
        *var_cgsb_dn5_slot = var_cgsb_dn5;
        *var_cgsb_dn6_slot = var_cgsb_dn6;
        *var_cgsb_dn7_slot = var_cgsb_dn7;
        *var_cgsb_dn8_slot = var_cgsb_dn8;
        *var_cgsb_dn9_slot = var_cgsb_dn9;
        *var_cgsb_rv_slot = var_cgsb_rv;
        *var_cnst0over_slot = var_cnst0over;
        *var_cnst0over_dn0_slot = var_cnst0over_dn0;
        *var_cnst0over_dn10_slot = var_cnst0over_dn10;
        *var_cnst0over_dn11_slot = var_cnst0over_dn11;
        *var_cnst0over_dn14_slot = var_cnst0over_dn14;
        *var_cnst0over_dn2_slot = var_cnst0over_dn2;
        *var_cnst0over_dn4_slot = var_cnst0over_dn4;
        *var_cnst0over_dn5_slot = var_cnst0over_dn5;
        *var_cnst0over_dn6_slot = var_cnst0over_dn6;
        *var_cnst0over_dn7_slot = var_cnst0over_dn7;
        *var_cnst0over_dn8_slot = var_cnst0over_dn8;
        *var_cnst0over_dn9_slot = var_cnst0over_dn9;
        *var_cnst0over_rv_slot = var_cnst0over_rv;
        *var_cnst0overs_slot = var_cnst0overs;
        *var_cnst0overs_dn0_slot = var_cnst0overs_dn0;
        *var_cnst0overs_dn10_slot = var_cnst0overs_dn10;
        *var_cnst0overs_dn11_slot = var_cnst0overs_dn11;
        *var_cnst0overs_dn14_slot = var_cnst0overs_dn14;
        *var_cnst0overs_dn2_slot = var_cnst0overs_dn2;
        *var_cnst0overs_dn4_slot = var_cnst0overs_dn4;
        *var_cnst0overs_dn5_slot = var_cnst0overs_dn5;
        *var_cnst0overs_dn6_slot = var_cnst0overs_dn6;
        *var_cnst0overs_dn7_slot = var_cnst0overs_dn7;
        *var_cnst0overs_dn8_slot = var_cnst0overs_dn8;
        *var_cnst0overs_dn9_slot = var_cnst0overs_dn9;
        *var_cnst0overs_rv_slot = var_cnst0overs_rv;
        *var_cx_slot = var_cx;
        *var_cx_rv_slot = var_cx_rv;
        *var_deltemp_slot = var_deltemp;
        *var_deltemp_dn0_slot = var_deltemp_dn0;
        *var_deltemp_dn10_slot = var_deltemp_dn10;
        *var_deltemp_dn11_slot = var_deltemp_dn11;
        *var_deltemp_dn14_slot = var_deltemp_dn14;
        *var_deltemp_dn2_slot = var_deltemp_dn2;
        *var_deltemp_dn4_slot = var_deltemp_dn4;
        *var_deltemp_dn5_slot = var_deltemp_dn5;
        *var_deltemp_dn6_slot = var_deltemp_dn6;
        *var_deltemp_dn7_slot = var_deltemp_dn7;
        *var_deltemp_dn8_slot = var_deltemp_dn8;
        *var_deltemp_dn9_slot = var_deltemp_dn9;
        *var_deltemp_rv_slot = var_deltemp_rv;
        *var_gd_slot = var_gd;
        *var_gd_dn0_slot = var_gd_dn0;
        *var_gd_dn10_slot = var_gd_dn10;
        *var_gd_dn11_slot = var_gd_dn11;
        *var_gd_dn14_slot = var_gd_dn14;
        *var_gd_dn2_slot = var_gd_dn2;
        *var_gd_dn4_slot = var_gd_dn4;
        *var_gd_dn5_slot = var_gd_dn5;
        *var_gd_dn6_slot = var_gd_dn6;
        *var_gd_dn7_slot = var_gd_dn7;
        *var_gd_dn8_slot = var_gd_dn8;
        *var_gd_dn9_slot = var_gd_dn9;
        *var_gd_rv_slot = var_gd_rv;
        *var_gth_slot = var_gth;
        *var_gth_dn0_slot = var_gth_dn0;
        *var_gth_dn10_slot = var_gth_dn10;
        *var_gth_dn11_slot = var_gth_dn11;
        *var_gth_dn14_slot = var_gth_dn14;
        *var_gth_dn2_slot = var_gth_dn2;
        *var_gth_dn4_slot = var_gth_dn4;
        *var_gth_dn5_slot = var_gth_dn5;
        *var_gth_dn6_slot = var_gth_dn6;
        *var_gth_dn7_slot = var_gth_dn7;
        *var_gth_dn8_slot = var_gth_dn8;
        *var_gth_dn9_slot = var_gth_dn9;
        *var_gth_rv_slot = var_gth_rv;
        *var_mu0_slot = var_mu0;
        *var_mu0_dn0_slot = var_mu0_dn0;
        *var_mu0_dn10_slot = var_mu0_dn10;
        *var_mu0_dn11_slot = var_mu0_dn11;
        *var_mu0_dn14_slot = var_mu0_dn14;
        *var_mu0_dn2_slot = var_mu0_dn2;
        *var_mu0_dn4_slot = var_mu0_dn4;
        *var_mu0_dn5_slot = var_mu0_dn5;
        *var_mu0_dn6_slot = var_mu0_dn6;
        *var_mu0_dn7_slot = var_mu0_dn7;
        *var_mu0_dn8_slot = var_mu0_dn8;
        *var_mu0_dn9_slot = var_mu0_dn9;
        *var_mu0_rv_slot = var_mu0_rv;
        *var_ninvde_slot = var_ninvde;
        *var_ninvde_dn0_slot = var_ninvde_dn0;
        *var_ninvde_dn10_slot = var_ninvde_dn10;
        *var_ninvde_dn11_slot = var_ninvde_dn11;
        *var_ninvde_dn14_slot = var_ninvde_dn14;
        *var_ninvde_dn2_slot = var_ninvde_dn2;
        *var_ninvde_dn4_slot = var_ninvde_dn4;
        *var_ninvde_dn5_slot = var_ninvde_dn5;
        *var_ninvde_dn6_slot = var_ninvde_dn6;
        *var_ninvde_dn7_slot = var_ninvde_dn7;
        *var_ninvde_dn8_slot = var_ninvde_dn8;
        *var_ninvde_dn9_slot = var_ninvde_dn9;
        *var_ninvde_rv_slot = var_ninvde_rv;
        *var_ninvdecres_slot = var_ninvdecres;
        *var_ninvdecres_dn0_slot = var_ninvdecres_dn0;
        *var_ninvdecres_dn10_slot = var_ninvdecres_dn10;
        *var_ninvdecres_dn11_slot = var_ninvdecres_dn11;
        *var_ninvdecres_dn14_slot = var_ninvdecres_dn14;
        *var_ninvdecres_dn2_slot = var_ninvdecres_dn2;
        *var_ninvdecres_dn4_slot = var_ninvdecres_dn4;
        *var_ninvdecres_dn5_slot = var_ninvdecres_dn5;
        *var_ninvdecres_dn6_slot = var_ninvdecres_dn6;
        *var_ninvdecres_dn7_slot = var_ninvdecres_dn7;
        *var_ninvdecres_dn8_slot = var_ninvdecres_dn8;
        *var_ninvdecres_dn9_slot = var_ninvdecres_dn9;
        *var_ninvdecres_rv_slot = var_ninvdecres_rv;
        *var_ninvdehres_slot = var_ninvdehres;
        *var_ninvdehres_dn0_slot = var_ninvdehres_dn0;
        *var_ninvdehres_dn10_slot = var_ninvdehres_dn10;
        *var_ninvdehres_dn11_slot = var_ninvdehres_dn11;
        *var_ninvdehres_dn14_slot = var_ninvdehres_dn14;
        *var_ninvdehres_dn2_slot = var_ninvdehres_dn2;
        *var_ninvdehres_dn4_slot = var_ninvdehres_dn4;
        *var_ninvdehres_dn5_slot = var_ninvdehres_dn5;
        *var_ninvdehres_dn6_slot = var_ninvdehres_dn6;
        *var_ninvdehres_dn7_slot = var_ninvdehres_dn7;
        *var_ninvdehres_dn8_slot = var_ninvdehres_dn8;
        *var_ninvdehres_dn9_slot = var_ninvdehres_dn9;
        *var_ninvdehres_rv_slot = var_ninvdehres_rv;
        *var_p_slot = var_p;
        *var_p_dn0_slot = var_p_dn0;
        *var_p_dn10_slot = var_p_dn10;
        *var_p_dn11_slot = var_p_dn11;
        *var_p_dn14_slot = var_p_dn14;
        *var_p_dn2_slot = var_p_dn2;
        *var_p_dn4_slot = var_p_dn4;
        *var_p_dn5_slot = var_p_dn5;
        *var_p_dn6_slot = var_p_dn6;
        *var_p_dn7_slot = var_p_dn7;
        *var_p_dn8_slot = var_p_dn8;
        *var_p_dn9_slot = var_p_dn9;
        *var_p_rv_slot = var_p_rv;
        *var_qb_nqs_slot = var_qb_nqs;
        *var_qb_nqs_dn13_slot = var_qb_nqs_dn13;
        *var_qb_nqs_rv_slot = var_qb_nqs_rv;
        *var_qd_nqs_slot = var_qd_nqs;
        *var_qd_nqs_dn0_slot = var_qd_nqs_dn0;
        *var_qd_nqs_dn10_slot = var_qd_nqs_dn10;
        *var_qd_nqs_dn11_slot = var_qd_nqs_dn11;
        *var_qd_nqs_dn12_slot = var_qd_nqs_dn12;
        *var_qd_nqs_dn14_slot = var_qd_nqs_dn14;
        *var_qd_nqs_dn2_slot = var_qd_nqs_dn2;
        *var_qd_nqs_dn4_slot = var_qd_nqs_dn4;
        *var_qd_nqs_dn5_slot = var_qd_nqs_dn5;
        *var_qd_nqs_dn6_slot = var_qd_nqs_dn6;
        *var_qd_nqs_dn7_slot = var_qd_nqs_dn7;
        *var_qd_nqs_dn8_slot = var_qd_nqs_dn8;
        *var_qd_nqs_dn9_slot = var_qd_nqs_dn9;
        *var_qd_nqs_rv_slot = var_qd_nqs_rv;
        *var_qg_slot = var_qg;
        *var_qg_dn0_slot = var_qg_dn0;
        *var_qg_dn10_slot = var_qg_dn10;
        *var_qg_dn11_slot = var_qg_dn11;
        *var_qg_dn14_slot = var_qg_dn14;
        *var_qg_dn2_slot = var_qg_dn2;
        *var_qg_dn4_slot = var_qg_dn4;
        *var_qg_dn5_slot = var_qg_dn5;
        *var_qg_dn6_slot = var_qg_dn6;
        *var_qg_dn7_slot = var_qg_dn7;
        *var_qg_dn8_slot = var_qg_dn8;
        *var_qg_dn9_slot = var_qg_dn9;
        *var_qg_nqs_slot = var_qg_nqs;
        *var_qg_nqs_dn12_slot = var_qg_nqs_dn12;
        *var_qg_nqs_dn13_slot = var_qg_nqs_dn13;
        *var_qg_nqs_rv_slot = var_qg_nqs_rv;
        *var_qg_rv_slot = var_qg_rv;
        *var_qi_nqs_slot = var_qi_nqs;
        *var_qi_nqs_dn12_slot = var_qi_nqs_dn12;
        *var_qi_nqs_rv_slot = var_qi_nqs_rv;
        *var_qs_slot = var_qs;
        *var_qs_dn0_slot = var_qs_dn0;
        *var_qs_dn10_slot = var_qs_dn10;
        *var_qs_dn11_slot = var_qs_dn11;
        *var_qs_dn14_slot = var_qs_dn14;
        *var_qs_dn2_slot = var_qs_dn2;
        *var_qs_dn4_slot = var_qs_dn4;
        *var_qs_dn5_slot = var_qs_dn5;
        *var_qs_dn6_slot = var_qs_dn6;
        *var_qs_dn7_slot = var_qs_dn7;
        *var_qs_dn8_slot = var_qs_dn8;
        *var_qs_dn9_slot = var_qs_dn9;
        *var_qs_nqs_slot = var_qs_nqs;
        *var_qs_nqs_dn0_slot = var_qs_nqs_dn0;
        *var_qs_nqs_dn10_slot = var_qs_nqs_dn10;
        *var_qs_nqs_dn11_slot = var_qs_nqs_dn11;
        *var_qs_nqs_dn12_slot = var_qs_nqs_dn12;
        *var_qs_nqs_dn14_slot = var_qs_nqs_dn14;
        *var_qs_nqs_dn2_slot = var_qs_nqs_dn2;
        *var_qs_nqs_dn4_slot = var_qs_nqs_dn4;
        *var_qs_nqs_dn5_slot = var_qs_nqs_dn5;
        *var_qs_nqs_dn6_slot = var_qs_nqs_dn6;
        *var_qs_nqs_dn7_slot = var_qs_nqs_dn7;
        *var_qs_nqs_dn8_slot = var_qs_nqs_dn8;
        *var_qs_nqs_dn9_slot = var_qs_nqs_dn9;
        *var_qs_nqs_rv_slot = var_qs_nqs_rv;
        *var_qs_rv_slot = var_qs_rv;
        *var_rde_slot = var_rde;
        *var_rde_dn0_slot = var_rde_dn0;
        *var_rde_dn10_slot = var_rde_dn10;
        *var_rde_dn11_slot = var_rde_dn11;
        *var_rde_dn14_slot = var_rde_dn14;
        *var_rde_dn2_slot = var_rde_dn2;
        *var_rde_dn4_slot = var_rde_dn4;
        *var_rde_dn5_slot = var_rde_dn5;
        *var_rde_dn6_slot = var_rde_dn6;
        *var_rde_dn7_slot = var_rde_dn7;
        *var_rde_dn8_slot = var_rde_dn8;
        *var_rde_dn9_slot = var_rde_dn9;
        *var_rde_rv_slot = var_rde_rv;
        *var_rdvde_slot = var_rdvde;
        *var_rdvde_dn0_slot = var_rdvde_dn0;
        *var_rdvde_dn10_slot = var_rdvde_dn10;
        *var_rdvde_dn11_slot = var_rdvde_dn11;
        *var_rdvde_dn14_slot = var_rdvde_dn14;
        *var_rdvde_dn2_slot = var_rdvde_dn2;
        *var_rdvde_dn4_slot = var_rdvde_dn4;
        *var_rdvde_dn5_slot = var_rdvde_dn5;
        *var_rdvde_dn6_slot = var_rdvde_dn6;
        *var_rdvde_dn7_slot = var_rdvde_dn7;
        *var_rdvde_dn8_slot = var_rdvde_dn8;
        *var_rdvde_dn9_slot = var_rdvde_dn9;
        *var_rdvde_rv_slot = var_rdvde_rv;
        *var_rrdrmue_slot = var_rrdrmue;
        *var_rrdrmue_dn0_slot = var_rrdrmue_dn0;
        *var_rrdrmue_dn10_slot = var_rrdrmue_dn10;
        *var_rrdrmue_dn11_slot = var_rrdrmue_dn11;
        *var_rrdrmue_dn14_slot = var_rrdrmue_dn14;
        *var_rrdrmue_dn2_slot = var_rrdrmue_dn2;
        *var_rrdrmue_dn4_slot = var_rrdrmue_dn4;
        *var_rrdrmue_dn5_slot = var_rrdrmue_dn5;
        *var_rrdrmue_dn6_slot = var_rrdrmue_dn6;
        *var_rrdrmue_dn7_slot = var_rrdrmue_dn7;
        *var_rrdrmue_dn8_slot = var_rrdrmue_dn8;
        *var_rrdrmue_dn9_slot = var_rrdrmue_dn9;
        *var_rrdrmue_rv_slot = var_rrdrmue_rv;
        *var_rrdrmues_slot = var_rrdrmues;
        *var_rrdrmues_dn0_slot = var_rrdrmues_dn0;
        *var_rrdrmues_dn10_slot = var_rrdrmues_dn10;
        *var_rrdrmues_dn11_slot = var_rrdrmues_dn11;
        *var_rrdrmues_dn14_slot = var_rrdrmues_dn14;
        *var_rrdrmues_dn2_slot = var_rrdrmues_dn2;
        *var_rrdrmues_dn4_slot = var_rrdrmues_dn4;
        *var_rrdrmues_dn5_slot = var_rrdrmues_dn5;
        *var_rrdrmues_dn6_slot = var_rrdrmues_dn6;
        *var_rrdrmues_dn7_slot = var_rrdrmues_dn7;
        *var_rrdrmues_dn8_slot = var_rrdrmues_dn8;
        *var_rrdrmues_dn9_slot = var_rrdrmues_dn9;
        *var_rrdrmues_rv_slot = var_rrdrmues_rv;
        *var_rrdrvmax_slot = var_rrdrvmax;
        *var_rrdrvmax_dn0_slot = var_rrdrvmax_dn0;
        *var_rrdrvmax_dn10_slot = var_rrdrvmax_dn10;
        *var_rrdrvmax_dn11_slot = var_rrdrvmax_dn11;
        *var_rrdrvmax_dn14_slot = var_rrdrvmax_dn14;
        *var_rrdrvmax_dn2_slot = var_rrdrvmax_dn2;
        *var_rrdrvmax_dn4_slot = var_rrdrvmax_dn4;
        *var_rrdrvmax_dn5_slot = var_rrdrvmax_dn5;
        *var_rrdrvmax_dn6_slot = var_rrdrvmax_dn6;
        *var_rrdrvmax_dn7_slot = var_rrdrvmax_dn7;
        *var_rrdrvmax_dn8_slot = var_rrdrvmax_dn8;
        *var_rrdrvmax_dn9_slot = var_rrdrvmax_dn9;
        *var_rrdrvmax_rv_slot = var_rrdrvmax_rv;
        *var_rrdrvmaxs_slot = var_rrdrvmaxs;
        *var_rrdrvmaxs_dn0_slot = var_rrdrvmaxs_dn0;
        *var_rrdrvmaxs_dn10_slot = var_rrdrvmaxs_dn10;
        *var_rrdrvmaxs_dn11_slot = var_rrdrvmaxs_dn11;
        *var_rrdrvmaxs_dn14_slot = var_rrdrvmaxs_dn14;
        *var_rrdrvmaxs_dn2_slot = var_rrdrvmaxs_dn2;
        *var_rrdrvmaxs_dn4_slot = var_rrdrvmaxs_dn4;
        *var_rrdrvmaxs_dn5_slot = var_rrdrvmaxs_dn5;
        *var_rrdrvmaxs_dn6_slot = var_rrdrvmaxs_dn6;
        *var_rrdrvmaxs_dn7_slot = var_rrdrvmaxs_dn7;
        *var_rrdrvmaxs_dn8_slot = var_rrdrvmaxs_dn8;
        *var_rrdrvmaxs_dn9_slot = var_rrdrvmaxs_dn9;
        *var_rrdrvmaxs_rv_slot = var_rrdrvmaxs_rv;
        *var_rse_slot = var_rse;
        *var_rse_dn0_slot = var_rse_dn0;
        *var_rse_dn10_slot = var_rse_dn10;
        *var_rse_dn11_slot = var_rse_dn11;
        *var_rse_dn14_slot = var_rse_dn14;
        *var_rse_dn2_slot = var_rse_dn2;
        *var_rse_dn4_slot = var_rse_dn4;
        *var_rse_dn5_slot = var_rse_dn5;
        *var_rse_dn6_slot = var_rse_dn6;
        *var_rse_dn7_slot = var_rse_dn7;
        *var_rse_dn8_slot = var_rse_dn8;
        *var_rse_dn9_slot = var_rse_dn9;
        *var_rse_rv_slot = var_rse_rv;
        *var_rsvde_slot = var_rsvde;
        *var_rsvde_dn0_slot = var_rsvde_dn0;
        *var_rsvde_dn10_slot = var_rsvde_dn10;
        *var_rsvde_dn11_slot = var_rsvde_dn11;
        *var_rsvde_dn14_slot = var_rsvde_dn14;
        *var_rsvde_dn2_slot = var_rsvde_dn2;
        *var_rsvde_dn4_slot = var_rsvde_dn4;
        *var_rsvde_dn5_slot = var_rsvde_dn5;
        *var_rsvde_dn6_slot = var_rsvde_dn6;
        *var_rsvde_dn7_slot = var_rsvde_dn7;
        *var_rsvde_dn8_slot = var_rsvde_dn8;
        *var_rsvde_dn9_slot = var_rsvde_dn9;
        *var_rsvde_rv_slot = var_rsvde_rv;
        *var_tratio_slot = var_tratio;
        *var_tratio_dn0_slot = var_tratio_dn0;
        *var_tratio_dn10_slot = var_tratio_dn10;
        *var_tratio_dn11_slot = var_tratio_dn11;
        *var_tratio_dn14_slot = var_tratio_dn14;
        *var_tratio_dn2_slot = var_tratio_dn2;
        *var_tratio_dn4_slot = var_tratio_dn4;
        *var_tratio_dn5_slot = var_tratio_dn5;
        *var_tratio_dn6_slot = var_tratio_dn6;
        *var_tratio_dn7_slot = var_tratio_dn7;
        *var_tratio_dn8_slot = var_tratio_dn8;
        *var_tratio_dn9_slot = var_tratio_dn9;
        *var_tratio_rv_slot = var_tratio_rv;
        *var_vbd_slot = var_vbd;
        *var_vbd_dn6_slot = var_vbd_dn6;
        *var_vbd_dn8_slot = var_vbd_dn8;
        *var_vbd_dn9_slot = var_vbd_dn9;
        *var_vbd_rv_slot = var_vbd_rv;
        *var_vbsei_slot = var_vbsei;
        *var_vbsei_dn2_slot = var_vbsei_dn2;
        *var_vbsei_dn9_slot = var_vbsei_dn9;
        *var_vbsei_rv_slot = var_vbsei_rv;
        *var_vbsi_slot = var_vbsi;
        *var_vbsi_dn8_slot = var_vbsi_dn8;
        *var_vbsi_dn9_slot = var_vbsi_dn9;
        *var_vbsi_rv_slot = var_vbsi_rv;
        *var_vddpz_slot = var_vddpz;
        *var_vddpz_dn0_slot = var_vddpz_dn0;
        *var_vddpz_dn10_slot = var_vddpz_dn10;
        *var_vddpz_dn11_slot = var_vddpz_dn11;
        *var_vddpz_dn14_slot = var_vddpz_dn14;
        *var_vddpz_dn2_slot = var_vddpz_dn2;
        *var_vddpz_dn4_slot = var_vddpz_dn4;
        *var_vddpz_dn5_slot = var_vddpz_dn5;
        *var_vddpz_dn6_slot = var_vddpz_dn6;
        *var_vddpz_dn7_slot = var_vddpz_dn7;
        *var_vddpz_dn8_slot = var_vddpz_dn8;
        *var_vddpz_dn9_slot = var_vddpz_dn9;
        *var_vddpz_rv_slot = var_vddpz_rv;
        *var_vdri_slot = var_vdri;
        *var_vdri_dn0_slot = var_vdri_dn0;
        *var_vdri_dn10_slot = var_vdri_dn10;
        *var_vdri_dn11_slot = var_vdri_dn11;
        *var_vdri_dn14_slot = var_vdri_dn14;
        *var_vdri_dn2_slot = var_vdri_dn2;
        *var_vdri_dn4_slot = var_vdri_dn4;
        *var_vdri_dn5_slot = var_vdri_dn5;
        *var_vdri_dn6_slot = var_vdri_dn6;
        *var_vdri_dn7_slot = var_vdri_dn7;
        *var_vdri_dn8_slot = var_vdri_dn8;
        *var_vdri_dn9_slot = var_vdri_dn9;
        *var_vdri_rv_slot = var_vdri_rv;
        *var_vdsei_slot = var_vdsei;
        *var_vdsei_dn0_slot = var_vdsei_dn0;
        *var_vdsei_dn2_slot = var_vdsei_dn2;
        *var_vdsei_rv_slot = var_vdsei_rv;
        *var_vdsi_slot = var_vdsi;
        *var_vdsi_dn6_slot = var_vdsi_dn6;
        *var_vdsi_dn8_slot = var_vdsi_dn8;
        *var_vdsi_rv_slot = var_vdsi_rv;
        *var_veffpower_slot = var_veffpower;
        *var_veffpower_dn0_slot = var_veffpower_dn0;
        *var_veffpower_dn10_slot = var_veffpower_dn10;
        *var_veffpower_dn11_slot = var_veffpower_dn11;
        *var_veffpower_dn14_slot = var_veffpower_dn14;
        *var_veffpower_dn2_slot = var_veffpower_dn2;
        *var_veffpower_dn4_slot = var_veffpower_dn4;
        *var_veffpower_dn5_slot = var_veffpower_dn5;
        *var_veffpower_dn6_slot = var_veffpower_dn6;
        *var_veffpower_dn7_slot = var_veffpower_dn7;
        *var_veffpower_dn8_slot = var_veffpower_dn8;
        *var_veffpower_dn9_slot = var_veffpower_dn9;
        *var_veffpower_rv_slot = var_veffpower_rv;
        *var_vgd_slot = var_vgd;
        *var_vgd_dn6_slot = var_vgd_dn6;
        *var_vgd_dn7_slot = var_vgd_dn7;
        *var_vgd_dn8_slot = var_vgd_dn8;
        *var_vgd_rv_slot = var_vgd_rv;
        *var_vgsei_slot = var_vgsei;
        *var_vgsei_dn2_slot = var_vgsei_dn2;
        *var_vgsei_dn7_slot = var_vgsei_dn7;
        *var_vgsei_rv_slot = var_vgsei_rv;
        *var_vgsi_slot = var_vgsi;
        *var_vgsi_dn7_slot = var_vgsi_dn7;
        *var_vgsi_dn8_slot = var_vgsi_dn8;
        *var_vgsi_rv_slot = var_vgsi_rv;
        *var_vmaxeff_slot = var_vmaxeff;
        *var_vmaxeff_dn0_slot = var_vmaxeff_dn0;
        *var_vmaxeff_dn10_slot = var_vmaxeff_dn10;
        *var_vmaxeff_dn11_slot = var_vmaxeff_dn11;
        *var_vmaxeff_dn14_slot = var_vmaxeff_dn14;
        *var_vmaxeff_dn2_slot = var_vmaxeff_dn2;
        *var_vmaxeff_dn4_slot = var_vmaxeff_dn4;
        *var_vmaxeff_dn5_slot = var_vmaxeff_dn5;
        *var_vmaxeff_dn6_slot = var_vmaxeff_dn6;
        *var_vmaxeff_dn7_slot = var_vmaxeff_dn7;
        *var_vmaxeff_dn8_slot = var_vmaxeff_dn8;
        *var_vmaxeff_dn9_slot = var_vmaxeff_dn9;
        *var_vmaxeff_rv_slot = var_vmaxeff_rv;
        *var_xov_slot = var_xov;
        *var_xov_dn0_slot = var_xov_dn0;
        *var_xov_dn10_slot = var_xov_dn10;
        *var_xov_dn11_slot = var_xov_dn11;
        *var_xov_dn14_slot = var_xov_dn14;
        *var_xov_dn2_slot = var_xov_dn2;
        *var_xov_dn4_slot = var_xov_dn4;
        *var_xov_dn5_slot = var_xov_dn5;
        *var_xov_dn6_slot = var_xov_dn6;
        *var_xov_dn7_slot = var_xov_dn7;
        *var_xov_dn8_slot = var_xov_dn8;
        *var_xov_dn9_slot = var_xov_dn9;
        *var_xov_rv_slot = var_xov_rv;
    }

    pub(super) fn stamp_reactive_block_9(
        p: &Parameters,
        var_costi0_p2_slot: &mut f64,
        var_costi0_p2_dn0_slot: &mut f64,
        var_costi0_p2_dn10_slot: &mut f64,
        var_costi0_p2_dn11_slot: &mut f64,
        var_costi0_p2_dn14_slot: &mut f64,
        var_costi0_p2_dn2_slot: &mut f64,
        var_costi0_p2_dn4_slot: &mut f64,
        var_costi0_p2_dn5_slot: &mut f64,
        var_costi0_p2_dn6_slot: &mut f64,
        var_costi0_p2_dn7_slot: &mut f64,
        var_costi0_p2_dn8_slot: &mut f64,
        var_costi0_p2_dn9_slot: &mut f64,
        var_costi0_p2_rv_slot: &mut f64,
        var_czbd_slot: &mut f64,
        var_czbd_dn0_slot: &mut f64,
        var_czbd_dn10_slot: &mut f64,
        var_czbd_dn11_slot: &mut f64,
        var_czbd_dn14_slot: &mut f64,
        var_czbd_dn2_slot: &mut f64,
        var_czbd_dn4_slot: &mut f64,
        var_czbd_dn5_slot: &mut f64,
        var_czbd_dn6_slot: &mut f64,
        var_czbd_dn7_slot: &mut f64,
        var_czbd_dn8_slot: &mut f64,
        var_czbd_dn9_slot: &mut f64,
        var_czbd_rv_slot: &mut f64,
        var_czbdsw_slot: &mut f64,
        var_czbdsw_dn0_slot: &mut f64,
        var_czbdsw_dn10_slot: &mut f64,
        var_czbdsw_dn11_slot: &mut f64,
        var_czbdsw_dn14_slot: &mut f64,
        var_czbdsw_dn2_slot: &mut f64,
        var_czbdsw_dn4_slot: &mut f64,
        var_czbdsw_dn5_slot: &mut f64,
        var_czbdsw_dn6_slot: &mut f64,
        var_czbdsw_dn7_slot: &mut f64,
        var_czbdsw_dn8_slot: &mut f64,
        var_czbdsw_dn9_slot: &mut f64,
        var_czbdsw_rv_slot: &mut f64,
        var_czbdswg_slot: &mut f64,
        var_czbdswg_dn0_slot: &mut f64,
        var_czbdswg_dn10_slot: &mut f64,
        var_czbdswg_dn11_slot: &mut f64,
        var_czbdswg_dn14_slot: &mut f64,
        var_czbdswg_dn2_slot: &mut f64,
        var_czbdswg_dn4_slot: &mut f64,
        var_czbdswg_dn5_slot: &mut f64,
        var_czbdswg_dn6_slot: &mut f64,
        var_czbdswg_dn7_slot: &mut f64,
        var_czbdswg_dn8_slot: &mut f64,
        var_czbdswg_dn9_slot: &mut f64,
        var_czbdswg_rv_slot: &mut f64,
        var_czbs_slot: &mut f64,
        var_czbs_dn0_slot: &mut f64,
        var_czbs_dn10_slot: &mut f64,
        var_czbs_dn11_slot: &mut f64,
        var_czbs_dn14_slot: &mut f64,
        var_czbs_dn2_slot: &mut f64,
        var_czbs_dn4_slot: &mut f64,
        var_czbs_dn5_slot: &mut f64,
        var_czbs_dn6_slot: &mut f64,
        var_czbs_dn7_slot: &mut f64,
        var_czbs_dn8_slot: &mut f64,
        var_czbs_dn9_slot: &mut f64,
        var_czbs_rv_slot: &mut f64,
        var_czbssw_slot: &mut f64,
        var_czbssw_dn0_slot: &mut f64,
        var_czbssw_dn10_slot: &mut f64,
        var_czbssw_dn11_slot: &mut f64,
        var_czbssw_dn14_slot: &mut f64,
        var_czbssw_dn2_slot: &mut f64,
        var_czbssw_dn4_slot: &mut f64,
        var_czbssw_dn5_slot: &mut f64,
        var_czbssw_dn6_slot: &mut f64,
        var_czbssw_dn7_slot: &mut f64,
        var_czbssw_dn8_slot: &mut f64,
        var_czbssw_dn9_slot: &mut f64,
        var_czbssw_rv_slot: &mut f64,
        var_czbsswg_slot: &mut f64,
        var_czbsswg_dn0_slot: &mut f64,
        var_czbsswg_dn10_slot: &mut f64,
        var_czbsswg_dn11_slot: &mut f64,
        var_czbsswg_dn14_slot: &mut f64,
        var_czbsswg_dn2_slot: &mut f64,
        var_czbsswg_dn4_slot: &mut f64,
        var_czbsswg_dn5_slot: &mut f64,
        var_czbsswg_dn6_slot: &mut f64,
        var_czbsswg_dn7_slot: &mut f64,
        var_czbsswg_dn8_slot: &mut f64,
        var_czbsswg_dn9_slot: &mut f64,
        var_czbsswg_rv_slot: &mut f64,
        var_depmphn0_slot: &mut f64,
        var_depmphn0_dn0_slot: &mut f64,
        var_depmphn0_dn10_slot: &mut f64,
        var_depmphn0_dn11_slot: &mut f64,
        var_depmphn0_dn14_slot: &mut f64,
        var_depmphn0_dn2_slot: &mut f64,
        var_depmphn0_dn4_slot: &mut f64,
        var_depmphn0_dn5_slot: &mut f64,
        var_depmphn0_dn6_slot: &mut f64,
        var_depmphn0_dn7_slot: &mut f64,
        var_depmphn0_dn8_slot: &mut f64,
        var_depmphn0_dn9_slot: &mut f64,
        var_depmphn0_rv_slot: &mut f64,
        var_hbdceff_slot: &mut f64,
        var_hbdceff_dn0_slot: &mut f64,
        var_hbdceff_dn10_slot: &mut f64,
        var_hbdceff_dn11_slot: &mut f64,
        var_hbdceff_dn14_slot: &mut f64,
        var_hbdceff_dn2_slot: &mut f64,
        var_hbdceff_dn4_slot: &mut f64,
        var_hbdceff_dn5_slot: &mut f64,
        var_hbdceff_dn6_slot: &mut f64,
        var_hbdceff_dn7_slot: &mut f64,
        var_hbdceff_dn8_slot: &mut f64,
        var_hbdceff_dn9_slot: &mut f64,
        var_hbdceff_rv_slot: &mut f64,
        var_ids_acc_slot: &mut f64,
        var_ids_acc_dn0_slot: &mut f64,
        var_ids_acc_dn10_slot: &mut f64,
        var_ids_acc_dn11_slot: &mut f64,
        var_ids_acc_dn14_slot: &mut f64,
        var_ids_acc_dn2_slot: &mut f64,
        var_ids_acc_dn4_slot: &mut f64,
        var_ids_acc_dn5_slot: &mut f64,
        var_ids_acc_dn6_slot: &mut f64,
        var_ids_acc_dn7_slot: &mut f64,
        var_ids_acc_dn8_slot: &mut f64,
        var_ids_acc_dn9_slot: &mut f64,
        var_ids_acc_rv_slot: &mut f64,
        var_ids_res_slot: &mut f64,
        var_ids_res_dn0_slot: &mut f64,
        var_ids_res_dn10_slot: &mut f64,
        var_ids_res_dn11_slot: &mut f64,
        var_ids_res_dn14_slot: &mut f64,
        var_ids_res_dn2_slot: &mut f64,
        var_ids_res_dn4_slot: &mut f64,
        var_ids_res_dn5_slot: &mut f64,
        var_ids_res_dn6_slot: &mut f64,
        var_ids_res_dn7_slot: &mut f64,
        var_ids_res_dn8_slot: &mut f64,
        var_ids_res_dn9_slot: &mut f64,
        var_ids_res_rv_slot: &mut f64,
        var_ires_leak_slot: &mut f64,
        var_ires_leak_dn0_slot: &mut f64,
        var_ires_leak_dn10_slot: &mut f64,
        var_ires_leak_dn11_slot: &mut f64,
        var_ires_leak_dn14_slot: &mut f64,
        var_ires_leak_dn2_slot: &mut f64,
        var_ires_leak_dn4_slot: &mut f64,
        var_ires_leak_dn5_slot: &mut f64,
        var_ires_leak_dn6_slot: &mut f64,
        var_ires_leak_dn7_slot: &mut f64,
        var_ires_leak_dn8_slot: &mut f64,
        var_ires_leak_dn9_slot: &mut f64,
        var_ires_leak_rv_slot: &mut f64,
        var_js_slot: &mut f64,
        var_js2_slot: &mut f64,
        var_js2_dn0_slot: &mut f64,
        var_js2_dn10_slot: &mut f64,
        var_js2_dn11_slot: &mut f64,
        var_js2_dn14_slot: &mut f64,
        var_js2_dn2_slot: &mut f64,
        var_js2_dn4_slot: &mut f64,
        var_js2_dn5_slot: &mut f64,
        var_js2_dn6_slot: &mut f64,
        var_js2_dn7_slot: &mut f64,
        var_js2_dn8_slot: &mut f64,
        var_js2_dn9_slot: &mut f64,
        var_js2_rv_slot: &mut f64,
        var_js_dn0_slot: &mut f64,
        var_js_dn10_slot: &mut f64,
        var_js_dn11_slot: &mut f64,
        var_js_dn14_slot: &mut f64,
        var_js_dn2_slot: &mut f64,
        var_js_dn4_slot: &mut f64,
        var_js_dn5_slot: &mut f64,
        var_js_dn6_slot: &mut f64,
        var_js_dn7_slot: &mut f64,
        var_js_dn8_slot: &mut f64,
        var_js_dn9_slot: &mut f64,
        var_js_rv_slot: &mut f64,
        var_jssw_slot: &mut f64,
        var_jssw2_slot: &mut f64,
        var_jssw2_dn0_slot: &mut f64,
        var_jssw2_dn10_slot: &mut f64,
        var_jssw2_dn11_slot: &mut f64,
        var_jssw2_dn14_slot: &mut f64,
        var_jssw2_dn2_slot: &mut f64,
        var_jssw2_dn4_slot: &mut f64,
        var_jssw2_dn5_slot: &mut f64,
        var_jssw2_dn6_slot: &mut f64,
        var_jssw2_dn7_slot: &mut f64,
        var_jssw2_dn8_slot: &mut f64,
        var_jssw2_dn9_slot: &mut f64,
        var_jssw2_rv_slot: &mut f64,
        var_jssw_dn0_slot: &mut f64,
        var_jssw_dn10_slot: &mut f64,
        var_jssw_dn11_slot: &mut f64,
        var_jssw_dn14_slot: &mut f64,
        var_jssw_dn2_slot: &mut f64,
        var_jssw_dn4_slot: &mut f64,
        var_jssw_dn5_slot: &mut f64,
        var_jssw_dn6_slot: &mut f64,
        var_jssw_dn7_slot: &mut f64,
        var_jssw_dn8_slot: &mut f64,
        var_jssw_dn9_slot: &mut f64,
        var_jssw_rv_slot: &mut f64,
        var_lp_s0_max_slot: &mut f64,
        var_lp_s0_max_rv_slot: &mut f64,
        var_mphn0_slot: &mut f64,
        var_mphn0_dn0_slot: &mut f64,
        var_mphn0_dn10_slot: &mut f64,
        var_mphn0_dn11_slot: &mut f64,
        var_mphn0_dn14_slot: &mut f64,
        var_mphn0_dn2_slot: &mut f64,
        var_mphn0_dn4_slot: &mut f64,
        var_mphn0_dn5_slot: &mut f64,
        var_mphn0_dn6_slot: &mut f64,
        var_mphn0_dn7_slot: &mut f64,
        var_mphn0_dn8_slot: &mut f64,
        var_mphn0_dn9_slot: &mut f64,
        var_mphn0_rv_slot: &mut f64,
        var_pb2n_slot: &mut f64,
        var_pb2n_dn0_slot: &mut f64,
        var_pb2n_dn10_slot: &mut f64,
        var_pb2n_dn11_slot: &mut f64,
        var_pb2n_dn14_slot: &mut f64,
        var_pb2n_dn2_slot: &mut f64,
        var_pb2n_dn4_slot: &mut f64,
        var_pb2n_dn5_slot: &mut f64,
        var_pb2n_dn6_slot: &mut f64,
        var_pb2n_dn7_slot: &mut f64,
        var_pb2n_dn8_slot: &mut f64,
        var_pb2n_dn9_slot: &mut f64,
        var_pb2n_rv_slot: &mut f64,
        var_powratio_slot: &mut f64,
        var_powratio_dn0_slot: &mut f64,
        var_powratio_dn10_slot: &mut f64,
        var_powratio_dn11_slot: &mut f64,
        var_powratio_dn14_slot: &mut f64,
        var_powratio_dn2_slot: &mut f64,
        var_powratio_dn4_slot: &mut f64,
        var_powratio_dn5_slot: &mut f64,
        var_powratio_dn6_slot: &mut f64,
        var_powratio_dn7_slot: &mut f64,
        var_powratio_dn8_slot: &mut f64,
        var_powratio_dn9_slot: &mut f64,
        var_powratio_rv_slot: &mut f64,
        var_ptovr_slot: &mut f64,
        var_ptovr_dn0_slot: &mut f64,
        var_ptovr_dn10_slot: &mut f64,
        var_ptovr_dn11_slot: &mut f64,
        var_ptovr_dn14_slot: &mut f64,
        var_ptovr_dn2_slot: &mut f64,
        var_ptovr_dn4_slot: &mut f64,
        var_ptovr_dn5_slot: &mut f64,
        var_ptovr_dn6_slot: &mut f64,
        var_ptovr_dn7_slot: &mut f64,
        var_ptovr_dn8_slot: &mut f64,
        var_ptovr_dn9_slot: &mut f64,
        var_ptovr_rv_slot: &mut f64,
        var_pzbd_slot: &mut f64,
        var_pzbd_dn0_slot: &mut f64,
        var_pzbd_dn10_slot: &mut f64,
        var_pzbd_dn11_slot: &mut f64,
        var_pzbd_dn14_slot: &mut f64,
        var_pzbd_dn2_slot: &mut f64,
        var_pzbd_dn4_slot: &mut f64,
        var_pzbd_dn5_slot: &mut f64,
        var_pzbd_dn6_slot: &mut f64,
        var_pzbd_dn7_slot: &mut f64,
        var_pzbd_dn8_slot: &mut f64,
        var_pzbd_dn9_slot: &mut f64,
        var_pzbd_rv_slot: &mut f64,
        var_pzbdsw_slot: &mut f64,
        var_pzbdsw_dn0_slot: &mut f64,
        var_pzbdsw_dn10_slot: &mut f64,
        var_pzbdsw_dn11_slot: &mut f64,
        var_pzbdsw_dn14_slot: &mut f64,
        var_pzbdsw_dn2_slot: &mut f64,
        var_pzbdsw_dn4_slot: &mut f64,
        var_pzbdsw_dn5_slot: &mut f64,
        var_pzbdsw_dn6_slot: &mut f64,
        var_pzbdsw_dn7_slot: &mut f64,
        var_pzbdsw_dn8_slot: &mut f64,
        var_pzbdsw_dn9_slot: &mut f64,
        var_pzbdsw_rv_slot: &mut f64,
        var_pzbdswg_slot: &mut f64,
        var_pzbdswg_dn0_slot: &mut f64,
        var_pzbdswg_dn10_slot: &mut f64,
        var_pzbdswg_dn11_slot: &mut f64,
        var_pzbdswg_dn14_slot: &mut f64,
        var_pzbdswg_dn2_slot: &mut f64,
        var_pzbdswg_dn4_slot: &mut f64,
        var_pzbdswg_dn5_slot: &mut f64,
        var_pzbdswg_dn6_slot: &mut f64,
        var_pzbdswg_dn7_slot: &mut f64,
        var_pzbdswg_dn8_slot: &mut f64,
        var_pzbdswg_dn9_slot: &mut f64,
        var_pzbdswg_rv_slot: &mut f64,
        var_pzbs_slot: &mut f64,
        var_pzbs_dn0_slot: &mut f64,
        var_pzbs_dn10_slot: &mut f64,
        var_pzbs_dn11_slot: &mut f64,
        var_pzbs_dn14_slot: &mut f64,
        var_pzbs_dn2_slot: &mut f64,
        var_pzbs_dn4_slot: &mut f64,
        var_pzbs_dn5_slot: &mut f64,
        var_pzbs_dn6_slot: &mut f64,
        var_pzbs_dn7_slot: &mut f64,
        var_pzbs_dn8_slot: &mut f64,
        var_pzbs_dn9_slot: &mut f64,
        var_pzbs_rv_slot: &mut f64,
        var_qbd_slot: &mut f64,
        var_qbd_dn0_slot: &mut f64,
        var_qbd_dn10_slot: &mut f64,
        var_qbd_dn11_slot: &mut f64,
        var_qbd_dn14_slot: &mut f64,
        var_qbd_dn16_slot: &mut f64,
        var_qbd_dn17_slot: &mut f64,
        var_qbd_dn18_slot: &mut f64,
        var_qbd_dn2_slot: &mut f64,
        var_qbd_dn4_slot: &mut f64,
        var_qbd_dn5_slot: &mut f64,
        var_qbd_dn6_slot: &mut f64,
        var_qbd_dn7_slot: &mut f64,
        var_qbd_dn8_slot: &mut f64,
        var_qbd_dn9_slot: &mut f64,
        var_qbd_rv_slot: &mut f64,
        var_qbdi_slot: &mut f64,
        var_qbdi_dn0_slot: &mut f64,
        var_qbdi_dn10_slot: &mut f64,
        var_qbdi_dn11_slot: &mut f64,
        var_qbdi_dn14_slot: &mut f64,
        var_qbdi_dn2_slot: &mut f64,
        var_qbdi_dn4_slot: &mut f64,
        var_qbdi_dn5_slot: &mut f64,
        var_qbdi_dn6_slot: &mut f64,
        var_qbdi_dn7_slot: &mut f64,
        var_qbdi_dn8_slot: &mut f64,
        var_qbdi_dn9_slot: &mut f64,
        var_qbdi_rv_slot: &mut f64,
        var_qbs_slot: &mut f64,
        var_qbs_dn0_slot: &mut f64,
        var_qbs_dn10_slot: &mut f64,
        var_qbs_dn11_slot: &mut f64,
        var_qbs_dn14_slot: &mut f64,
        var_qbs_dn2_slot: &mut f64,
        var_qbs_dn4_slot: &mut f64,
        var_qbs_dn5_slot: &mut f64,
        var_qbs_dn6_slot: &mut f64,
        var_qbs_dn7_slot: &mut f64,
        var_qbs_dn8_slot: &mut f64,
        var_qbs_dn9_slot: &mut f64,
        var_qbs_rv_slot: &mut f64,
        var_qbsi_slot: &mut f64,
        var_qbsi_dn0_slot: &mut f64,
        var_qbsi_dn10_slot: &mut f64,
        var_qbsi_dn11_slot: &mut f64,
        var_qbsi_dn14_slot: &mut f64,
        var_qbsi_dn2_slot: &mut f64,
        var_qbsi_dn4_slot: &mut f64,
        var_qbsi_dn5_slot: &mut f64,
        var_qbsi_dn6_slot: &mut f64,
        var_qbsi_dn7_slot: &mut f64,
        var_qbsi_dn8_slot: &mut f64,
        var_qbsi_dn9_slot: &mut f64,
        var_qbsi_rv_slot: &mut f64,
        var_qiu_noi_slot: &mut f64,
        var_qiu_noi_dn0_slot: &mut f64,
        var_qiu_noi_dn10_slot: &mut f64,
        var_qiu_noi_dn11_slot: &mut f64,
        var_qiu_noi_dn14_slot: &mut f64,
        var_qiu_noi_dn2_slot: &mut f64,
        var_qiu_noi_dn4_slot: &mut f64,
        var_qiu_noi_dn5_slot: &mut f64,
        var_qiu_noi_dn6_slot: &mut f64,
        var_qiu_noi_dn7_slot: &mut f64,
        var_qiu_noi_dn8_slot: &mut f64,
        var_qiu_noi_dn9_slot: &mut f64,
        var_qiu_noi_rv_slot: &mut f64,
        var_sqrt_eg_slot: &mut f64,
        var_sqrt_eg_dn0_slot: &mut f64,
        var_sqrt_eg_dn10_slot: &mut f64,
        var_sqrt_eg_dn11_slot: &mut f64,
        var_sqrt_eg_dn14_slot: &mut f64,
        var_sqrt_eg_dn2_slot: &mut f64,
        var_sqrt_eg_dn4_slot: &mut f64,
        var_sqrt_eg_dn5_slot: &mut f64,
        var_sqrt_eg_dn6_slot: &mut f64,
        var_sqrt_eg_dn7_slot: &mut f64,
        var_sqrt_eg_dn8_slot: &mut f64,
        var_sqrt_eg_dn9_slot: &mut f64,
        var_sqrt_eg_rv_slot: &mut f64,
        var_uc_rdrbb_slot: &mut f64,
        var_uc_rdrbb_dn0_slot: &mut f64,
        var_uc_rdrbb_dn10_slot: &mut f64,
        var_uc_rdrbb_dn11_slot: &mut f64,
        var_uc_rdrbb_dn14_slot: &mut f64,
        var_uc_rdrbb_dn2_slot: &mut f64,
        var_uc_rdrbb_dn4_slot: &mut f64,
        var_uc_rdrbb_dn5_slot: &mut f64,
        var_uc_rdrbb_dn6_slot: &mut f64,
        var_uc_rdrbb_dn7_slot: &mut f64,
        var_uc_rdrbb_dn8_slot: &mut f64,
        var_uc_rdrbb_dn9_slot: &mut f64,
        var_uc_rdrbb_rv_slot: &mut f64,
        var_uc_rdrbb_s_slot: &mut f64,
        var_uc_rdrbb_s_dn0_slot: &mut f64,
        var_uc_rdrbb_s_dn10_slot: &mut f64,
        var_uc_rdrbb_s_dn11_slot: &mut f64,
        var_uc_rdrbb_s_dn14_slot: &mut f64,
        var_uc_rdrbb_s_dn2_slot: &mut f64,
        var_uc_rdrbb_s_dn4_slot: &mut f64,
        var_uc_rdrbb_s_dn5_slot: &mut f64,
        var_uc_rdrbb_s_dn6_slot: &mut f64,
        var_uc_rdrbb_s_dn7_slot: &mut f64,
        var_uc_rdrbb_s_dn8_slot: &mut f64,
        var_uc_rdrbb_s_dn9_slot: &mut f64,
        var_uc_rdrbb_s_rv_slot: &mut f64,
        var_uc_subtmp_slot: &mut f64,
        var_uc_subtmp_rv_slot: &mut f64,
        var_vbipn_slot: &mut f64,
        var_vbipn_dn0_slot: &mut f64,
        var_vbipn_dn10_slot: &mut f64,
        var_vbipn_dn11_slot: &mut f64,
        var_vbipn_dn14_slot: &mut f64,
        var_vbipn_dn2_slot: &mut f64,
        var_vbipn_dn4_slot: &mut f64,
        var_vbipn_dn5_slot: &mut f64,
        var_vbipn_dn6_slot: &mut f64,
        var_vbipn_dn7_slot: &mut f64,
        var_vbipn_dn8_slot: &mut f64,
        var_vbipn_dn9_slot: &mut f64,
        var_vbipn_rv_slot: &mut f64,
        var_wdpl_slot: &mut f64,
        var_wdpl_dn0_slot: &mut f64,
        var_wdpl_dn10_slot: &mut f64,
        var_wdpl_dn11_slot: &mut f64,
        var_wdpl_dn14_slot: &mut f64,
        var_wdpl_dn2_slot: &mut f64,
        var_wdpl_dn4_slot: &mut f64,
        var_wdpl_dn5_slot: &mut f64,
        var_wdpl_dn6_slot: &mut f64,
        var_wdpl_dn7_slot: &mut f64,
        var_wdpl_dn8_slot: &mut f64,
        var_wdpl_dn9_slot: &mut f64,
        var_wdpl_rv_slot: &mut f64,
        var_wdplp_slot: &mut f64,
        var_wdplp_dn0_slot: &mut f64,
        var_wdplp_dn10_slot: &mut f64,
        var_wdplp_dn11_slot: &mut f64,
        var_wdplp_dn14_slot: &mut f64,
        var_wdplp_dn2_slot: &mut f64,
        var_wdplp_dn4_slot: &mut f64,
        var_wdplp_dn5_slot: &mut f64,
        var_wdplp_dn6_slot: &mut f64,
        var_wdplp_dn7_slot: &mut f64,
        var_wdplp_dn8_slot: &mut f64,
        var_wdplp_dn9_slot: &mut f64,
        var_wdplp_rv_slot: &mut f64,
    ) {
        let mut var_costi0_p2: f64 = *var_costi0_p2_slot;
        let mut var_costi0_p2_dn0: f64 = *var_costi0_p2_dn0_slot;
        let mut var_costi0_p2_dn10: f64 = *var_costi0_p2_dn10_slot;
        let mut var_costi0_p2_dn11: f64 = *var_costi0_p2_dn11_slot;
        let mut var_costi0_p2_dn14: f64 = *var_costi0_p2_dn14_slot;
        let mut var_costi0_p2_dn2: f64 = *var_costi0_p2_dn2_slot;
        let mut var_costi0_p2_dn4: f64 = *var_costi0_p2_dn4_slot;
        let mut var_costi0_p2_dn5: f64 = *var_costi0_p2_dn5_slot;
        let mut var_costi0_p2_dn6: f64 = *var_costi0_p2_dn6_slot;
        let mut var_costi0_p2_dn7: f64 = *var_costi0_p2_dn7_slot;
        let mut var_costi0_p2_dn8: f64 = *var_costi0_p2_dn8_slot;
        let mut var_costi0_p2_dn9: f64 = *var_costi0_p2_dn9_slot;
        let mut var_costi0_p2_rv: f64 = *var_costi0_p2_rv_slot;
        let mut var_czbd: f64 = *var_czbd_slot;
        let mut var_czbd_dn0: f64 = *var_czbd_dn0_slot;
        let mut var_czbd_dn10: f64 = *var_czbd_dn10_slot;
        let mut var_czbd_dn11: f64 = *var_czbd_dn11_slot;
        let mut var_czbd_dn14: f64 = *var_czbd_dn14_slot;
        let mut var_czbd_dn2: f64 = *var_czbd_dn2_slot;
        let mut var_czbd_dn4: f64 = *var_czbd_dn4_slot;
        let mut var_czbd_dn5: f64 = *var_czbd_dn5_slot;
        let mut var_czbd_dn6: f64 = *var_czbd_dn6_slot;
        let mut var_czbd_dn7: f64 = *var_czbd_dn7_slot;
        let mut var_czbd_dn8: f64 = *var_czbd_dn8_slot;
        let mut var_czbd_dn9: f64 = *var_czbd_dn9_slot;
        let mut var_czbd_rv: f64 = *var_czbd_rv_slot;
        let mut var_czbdsw: f64 = *var_czbdsw_slot;
        let mut var_czbdsw_dn0: f64 = *var_czbdsw_dn0_slot;
        let mut var_czbdsw_dn10: f64 = *var_czbdsw_dn10_slot;
        let mut var_czbdsw_dn11: f64 = *var_czbdsw_dn11_slot;
        let mut var_czbdsw_dn14: f64 = *var_czbdsw_dn14_slot;
        let mut var_czbdsw_dn2: f64 = *var_czbdsw_dn2_slot;
        let mut var_czbdsw_dn4: f64 = *var_czbdsw_dn4_slot;
        let mut var_czbdsw_dn5: f64 = *var_czbdsw_dn5_slot;
        let mut var_czbdsw_dn6: f64 = *var_czbdsw_dn6_slot;
        let mut var_czbdsw_dn7: f64 = *var_czbdsw_dn7_slot;
        let mut var_czbdsw_dn8: f64 = *var_czbdsw_dn8_slot;
        let mut var_czbdsw_dn9: f64 = *var_czbdsw_dn9_slot;
        let mut var_czbdsw_rv: f64 = *var_czbdsw_rv_slot;
        let mut var_czbdswg: f64 = *var_czbdswg_slot;
        let mut var_czbdswg_dn0: f64 = *var_czbdswg_dn0_slot;
        let mut var_czbdswg_dn10: f64 = *var_czbdswg_dn10_slot;
        let mut var_czbdswg_dn11: f64 = *var_czbdswg_dn11_slot;
        let mut var_czbdswg_dn14: f64 = *var_czbdswg_dn14_slot;
        let mut var_czbdswg_dn2: f64 = *var_czbdswg_dn2_slot;
        let mut var_czbdswg_dn4: f64 = *var_czbdswg_dn4_slot;
        let mut var_czbdswg_dn5: f64 = *var_czbdswg_dn5_slot;
        let mut var_czbdswg_dn6: f64 = *var_czbdswg_dn6_slot;
        let mut var_czbdswg_dn7: f64 = *var_czbdswg_dn7_slot;
        let mut var_czbdswg_dn8: f64 = *var_czbdswg_dn8_slot;
        let mut var_czbdswg_dn9: f64 = *var_czbdswg_dn9_slot;
        let mut var_czbdswg_rv: f64 = *var_czbdswg_rv_slot;
        let mut var_czbs: f64 = *var_czbs_slot;
        let mut var_czbs_dn0: f64 = *var_czbs_dn0_slot;
        let mut var_czbs_dn10: f64 = *var_czbs_dn10_slot;
        let mut var_czbs_dn11: f64 = *var_czbs_dn11_slot;
        let mut var_czbs_dn14: f64 = *var_czbs_dn14_slot;
        let mut var_czbs_dn2: f64 = *var_czbs_dn2_slot;
        let mut var_czbs_dn4: f64 = *var_czbs_dn4_slot;
        let mut var_czbs_dn5: f64 = *var_czbs_dn5_slot;
        let mut var_czbs_dn6: f64 = *var_czbs_dn6_slot;
        let mut var_czbs_dn7: f64 = *var_czbs_dn7_slot;
        let mut var_czbs_dn8: f64 = *var_czbs_dn8_slot;
        let mut var_czbs_dn9: f64 = *var_czbs_dn9_slot;
        let mut var_czbs_rv: f64 = *var_czbs_rv_slot;
        let mut var_czbssw: f64 = *var_czbssw_slot;
        let mut var_czbssw_dn0: f64 = *var_czbssw_dn0_slot;
        let mut var_czbssw_dn10: f64 = *var_czbssw_dn10_slot;
        let mut var_czbssw_dn11: f64 = *var_czbssw_dn11_slot;
        let mut var_czbssw_dn14: f64 = *var_czbssw_dn14_slot;
        let mut var_czbssw_dn2: f64 = *var_czbssw_dn2_slot;
        let mut var_czbssw_dn4: f64 = *var_czbssw_dn4_slot;
        let mut var_czbssw_dn5: f64 = *var_czbssw_dn5_slot;
        let mut var_czbssw_dn6: f64 = *var_czbssw_dn6_slot;
        let mut var_czbssw_dn7: f64 = *var_czbssw_dn7_slot;
        let mut var_czbssw_dn8: f64 = *var_czbssw_dn8_slot;
        let mut var_czbssw_dn9: f64 = *var_czbssw_dn9_slot;
        let mut var_czbssw_rv: f64 = *var_czbssw_rv_slot;
        let mut var_czbsswg: f64 = *var_czbsswg_slot;
        let mut var_czbsswg_dn0: f64 = *var_czbsswg_dn0_slot;
        let mut var_czbsswg_dn10: f64 = *var_czbsswg_dn10_slot;
        let mut var_czbsswg_dn11: f64 = *var_czbsswg_dn11_slot;
        let mut var_czbsswg_dn14: f64 = *var_czbsswg_dn14_slot;
        let mut var_czbsswg_dn2: f64 = *var_czbsswg_dn2_slot;
        let mut var_czbsswg_dn4: f64 = *var_czbsswg_dn4_slot;
        let mut var_czbsswg_dn5: f64 = *var_czbsswg_dn5_slot;
        let mut var_czbsswg_dn6: f64 = *var_czbsswg_dn6_slot;
        let mut var_czbsswg_dn7: f64 = *var_czbsswg_dn7_slot;
        let mut var_czbsswg_dn8: f64 = *var_czbsswg_dn8_slot;
        let mut var_czbsswg_dn9: f64 = *var_czbsswg_dn9_slot;
        let mut var_czbsswg_rv: f64 = *var_czbsswg_rv_slot;
        let mut var_depmphn0: f64 = *var_depmphn0_slot;
        let mut var_depmphn0_dn0: f64 = *var_depmphn0_dn0_slot;
        let mut var_depmphn0_dn10: f64 = *var_depmphn0_dn10_slot;
        let mut var_depmphn0_dn11: f64 = *var_depmphn0_dn11_slot;
        let mut var_depmphn0_dn14: f64 = *var_depmphn0_dn14_slot;
        let mut var_depmphn0_dn2: f64 = *var_depmphn0_dn2_slot;
        let mut var_depmphn0_dn4: f64 = *var_depmphn0_dn4_slot;
        let mut var_depmphn0_dn5: f64 = *var_depmphn0_dn5_slot;
        let mut var_depmphn0_dn6: f64 = *var_depmphn0_dn6_slot;
        let mut var_depmphn0_dn7: f64 = *var_depmphn0_dn7_slot;
        let mut var_depmphn0_dn8: f64 = *var_depmphn0_dn8_slot;
        let mut var_depmphn0_dn9: f64 = *var_depmphn0_dn9_slot;
        let mut var_depmphn0_rv: f64 = *var_depmphn0_rv_slot;
        let mut var_hbdceff: f64 = *var_hbdceff_slot;
        let mut var_hbdceff_dn0: f64 = *var_hbdceff_dn0_slot;
        let mut var_hbdceff_dn10: f64 = *var_hbdceff_dn10_slot;
        let mut var_hbdceff_dn11: f64 = *var_hbdceff_dn11_slot;
        let mut var_hbdceff_dn14: f64 = *var_hbdceff_dn14_slot;
        let mut var_hbdceff_dn2: f64 = *var_hbdceff_dn2_slot;
        let mut var_hbdceff_dn4: f64 = *var_hbdceff_dn4_slot;
        let mut var_hbdceff_dn5: f64 = *var_hbdceff_dn5_slot;
        let mut var_hbdceff_dn6: f64 = *var_hbdceff_dn6_slot;
        let mut var_hbdceff_dn7: f64 = *var_hbdceff_dn7_slot;
        let mut var_hbdceff_dn8: f64 = *var_hbdceff_dn8_slot;
        let mut var_hbdceff_dn9: f64 = *var_hbdceff_dn9_slot;
        let mut var_hbdceff_rv: f64 = *var_hbdceff_rv_slot;
        let mut var_ids_acc: f64 = *var_ids_acc_slot;
        let mut var_ids_acc_dn0: f64 = *var_ids_acc_dn0_slot;
        let mut var_ids_acc_dn10: f64 = *var_ids_acc_dn10_slot;
        let mut var_ids_acc_dn11: f64 = *var_ids_acc_dn11_slot;
        let mut var_ids_acc_dn14: f64 = *var_ids_acc_dn14_slot;
        let mut var_ids_acc_dn2: f64 = *var_ids_acc_dn2_slot;
        let mut var_ids_acc_dn4: f64 = *var_ids_acc_dn4_slot;
        let mut var_ids_acc_dn5: f64 = *var_ids_acc_dn5_slot;
        let mut var_ids_acc_dn6: f64 = *var_ids_acc_dn6_slot;
        let mut var_ids_acc_dn7: f64 = *var_ids_acc_dn7_slot;
        let mut var_ids_acc_dn8: f64 = *var_ids_acc_dn8_slot;
        let mut var_ids_acc_dn9: f64 = *var_ids_acc_dn9_slot;
        let mut var_ids_acc_rv: f64 = *var_ids_acc_rv_slot;
        let mut var_ids_res: f64 = *var_ids_res_slot;
        let mut var_ids_res_dn0: f64 = *var_ids_res_dn0_slot;
        let mut var_ids_res_dn10: f64 = *var_ids_res_dn10_slot;
        let mut var_ids_res_dn11: f64 = *var_ids_res_dn11_slot;
        let mut var_ids_res_dn14: f64 = *var_ids_res_dn14_slot;
        let mut var_ids_res_dn2: f64 = *var_ids_res_dn2_slot;
        let mut var_ids_res_dn4: f64 = *var_ids_res_dn4_slot;
        let mut var_ids_res_dn5: f64 = *var_ids_res_dn5_slot;
        let mut var_ids_res_dn6: f64 = *var_ids_res_dn6_slot;
        let mut var_ids_res_dn7: f64 = *var_ids_res_dn7_slot;
        let mut var_ids_res_dn8: f64 = *var_ids_res_dn8_slot;
        let mut var_ids_res_dn9: f64 = *var_ids_res_dn9_slot;
        let mut var_ids_res_rv: f64 = *var_ids_res_rv_slot;
        let mut var_ires_leak: f64 = *var_ires_leak_slot;
        let mut var_ires_leak_dn0: f64 = *var_ires_leak_dn0_slot;
        let mut var_ires_leak_dn10: f64 = *var_ires_leak_dn10_slot;
        let mut var_ires_leak_dn11: f64 = *var_ires_leak_dn11_slot;
        let mut var_ires_leak_dn14: f64 = *var_ires_leak_dn14_slot;
        let mut var_ires_leak_dn2: f64 = *var_ires_leak_dn2_slot;
        let mut var_ires_leak_dn4: f64 = *var_ires_leak_dn4_slot;
        let mut var_ires_leak_dn5: f64 = *var_ires_leak_dn5_slot;
        let mut var_ires_leak_dn6: f64 = *var_ires_leak_dn6_slot;
        let mut var_ires_leak_dn7: f64 = *var_ires_leak_dn7_slot;
        let mut var_ires_leak_dn8: f64 = *var_ires_leak_dn8_slot;
        let mut var_ires_leak_dn9: f64 = *var_ires_leak_dn9_slot;
        let mut var_ires_leak_rv: f64 = *var_ires_leak_rv_slot;
        let mut var_js: f64 = *var_js_slot;
        let mut var_js2: f64 = *var_js2_slot;
        let mut var_js2_dn0: f64 = *var_js2_dn0_slot;
        let mut var_js2_dn10: f64 = *var_js2_dn10_slot;
        let mut var_js2_dn11: f64 = *var_js2_dn11_slot;
        let mut var_js2_dn14: f64 = *var_js2_dn14_slot;
        let mut var_js2_dn2: f64 = *var_js2_dn2_slot;
        let mut var_js2_dn4: f64 = *var_js2_dn4_slot;
        let mut var_js2_dn5: f64 = *var_js2_dn5_slot;
        let mut var_js2_dn6: f64 = *var_js2_dn6_slot;
        let mut var_js2_dn7: f64 = *var_js2_dn7_slot;
        let mut var_js2_dn8: f64 = *var_js2_dn8_slot;
        let mut var_js2_dn9: f64 = *var_js2_dn9_slot;
        let mut var_js2_rv: f64 = *var_js2_rv_slot;
        let mut var_js_dn0: f64 = *var_js_dn0_slot;
        let mut var_js_dn10: f64 = *var_js_dn10_slot;
        let mut var_js_dn11: f64 = *var_js_dn11_slot;
        let mut var_js_dn14: f64 = *var_js_dn14_slot;
        let mut var_js_dn2: f64 = *var_js_dn2_slot;
        let mut var_js_dn4: f64 = *var_js_dn4_slot;
        let mut var_js_dn5: f64 = *var_js_dn5_slot;
        let mut var_js_dn6: f64 = *var_js_dn6_slot;
        let mut var_js_dn7: f64 = *var_js_dn7_slot;
        let mut var_js_dn8: f64 = *var_js_dn8_slot;
        let mut var_js_dn9: f64 = *var_js_dn9_slot;
        let mut var_js_rv: f64 = *var_js_rv_slot;
        let mut var_jssw: f64 = *var_jssw_slot;
        let mut var_jssw2: f64 = *var_jssw2_slot;
        let mut var_jssw2_dn0: f64 = *var_jssw2_dn0_slot;
        let mut var_jssw2_dn10: f64 = *var_jssw2_dn10_slot;
        let mut var_jssw2_dn11: f64 = *var_jssw2_dn11_slot;
        let mut var_jssw2_dn14: f64 = *var_jssw2_dn14_slot;
        let mut var_jssw2_dn2: f64 = *var_jssw2_dn2_slot;
        let mut var_jssw2_dn4: f64 = *var_jssw2_dn4_slot;
        let mut var_jssw2_dn5: f64 = *var_jssw2_dn5_slot;
        let mut var_jssw2_dn6: f64 = *var_jssw2_dn6_slot;
        let mut var_jssw2_dn7: f64 = *var_jssw2_dn7_slot;
        let mut var_jssw2_dn8: f64 = *var_jssw2_dn8_slot;
        let mut var_jssw2_dn9: f64 = *var_jssw2_dn9_slot;
        let mut var_jssw2_rv: f64 = *var_jssw2_rv_slot;
        let mut var_jssw_dn0: f64 = *var_jssw_dn0_slot;
        let mut var_jssw_dn10: f64 = *var_jssw_dn10_slot;
        let mut var_jssw_dn11: f64 = *var_jssw_dn11_slot;
        let mut var_jssw_dn14: f64 = *var_jssw_dn14_slot;
        let mut var_jssw_dn2: f64 = *var_jssw_dn2_slot;
        let mut var_jssw_dn4: f64 = *var_jssw_dn4_slot;
        let mut var_jssw_dn5: f64 = *var_jssw_dn5_slot;
        let mut var_jssw_dn6: f64 = *var_jssw_dn6_slot;
        let mut var_jssw_dn7: f64 = *var_jssw_dn7_slot;
        let mut var_jssw_dn8: f64 = *var_jssw_dn8_slot;
        let mut var_jssw_dn9: f64 = *var_jssw_dn9_slot;
        let mut var_jssw_rv: f64 = *var_jssw_rv_slot;
        let mut var_lp_s0_max: f64 = *var_lp_s0_max_slot;
        let mut var_lp_s0_max_rv: f64 = *var_lp_s0_max_rv_slot;
        let mut var_mphn0: f64 = *var_mphn0_slot;
        let mut var_mphn0_dn0: f64 = *var_mphn0_dn0_slot;
        let mut var_mphn0_dn10: f64 = *var_mphn0_dn10_slot;
        let mut var_mphn0_dn11: f64 = *var_mphn0_dn11_slot;
        let mut var_mphn0_dn14: f64 = *var_mphn0_dn14_slot;
        let mut var_mphn0_dn2: f64 = *var_mphn0_dn2_slot;
        let mut var_mphn0_dn4: f64 = *var_mphn0_dn4_slot;
        let mut var_mphn0_dn5: f64 = *var_mphn0_dn5_slot;
        let mut var_mphn0_dn6: f64 = *var_mphn0_dn6_slot;
        let mut var_mphn0_dn7: f64 = *var_mphn0_dn7_slot;
        let mut var_mphn0_dn8: f64 = *var_mphn0_dn8_slot;
        let mut var_mphn0_dn9: f64 = *var_mphn0_dn9_slot;
        let mut var_mphn0_rv: f64 = *var_mphn0_rv_slot;
        let mut var_pb2n: f64 = *var_pb2n_slot;
        let mut var_pb2n_dn0: f64 = *var_pb2n_dn0_slot;
        let mut var_pb2n_dn10: f64 = *var_pb2n_dn10_slot;
        let mut var_pb2n_dn11: f64 = *var_pb2n_dn11_slot;
        let mut var_pb2n_dn14: f64 = *var_pb2n_dn14_slot;
        let mut var_pb2n_dn2: f64 = *var_pb2n_dn2_slot;
        let mut var_pb2n_dn4: f64 = *var_pb2n_dn4_slot;
        let mut var_pb2n_dn5: f64 = *var_pb2n_dn5_slot;
        let mut var_pb2n_dn6: f64 = *var_pb2n_dn6_slot;
        let mut var_pb2n_dn7: f64 = *var_pb2n_dn7_slot;
        let mut var_pb2n_dn8: f64 = *var_pb2n_dn8_slot;
        let mut var_pb2n_dn9: f64 = *var_pb2n_dn9_slot;
        let mut var_pb2n_rv: f64 = *var_pb2n_rv_slot;
        let mut var_powratio: f64 = *var_powratio_slot;
        let mut var_powratio_dn0: f64 = *var_powratio_dn0_slot;
        let mut var_powratio_dn10: f64 = *var_powratio_dn10_slot;
        let mut var_powratio_dn11: f64 = *var_powratio_dn11_slot;
        let mut var_powratio_dn14: f64 = *var_powratio_dn14_slot;
        let mut var_powratio_dn2: f64 = *var_powratio_dn2_slot;
        let mut var_powratio_dn4: f64 = *var_powratio_dn4_slot;
        let mut var_powratio_dn5: f64 = *var_powratio_dn5_slot;
        let mut var_powratio_dn6: f64 = *var_powratio_dn6_slot;
        let mut var_powratio_dn7: f64 = *var_powratio_dn7_slot;
        let mut var_powratio_dn8: f64 = *var_powratio_dn8_slot;
        let mut var_powratio_dn9: f64 = *var_powratio_dn9_slot;
        let mut var_powratio_rv: f64 = *var_powratio_rv_slot;
        let mut var_ptovr: f64 = *var_ptovr_slot;
        let mut var_ptovr_dn0: f64 = *var_ptovr_dn0_slot;
        let mut var_ptovr_dn10: f64 = *var_ptovr_dn10_slot;
        let mut var_ptovr_dn11: f64 = *var_ptovr_dn11_slot;
        let mut var_ptovr_dn14: f64 = *var_ptovr_dn14_slot;
        let mut var_ptovr_dn2: f64 = *var_ptovr_dn2_slot;
        let mut var_ptovr_dn4: f64 = *var_ptovr_dn4_slot;
        let mut var_ptovr_dn5: f64 = *var_ptovr_dn5_slot;
        let mut var_ptovr_dn6: f64 = *var_ptovr_dn6_slot;
        let mut var_ptovr_dn7: f64 = *var_ptovr_dn7_slot;
        let mut var_ptovr_dn8: f64 = *var_ptovr_dn8_slot;
        let mut var_ptovr_dn9: f64 = *var_ptovr_dn9_slot;
        let mut var_ptovr_rv: f64 = *var_ptovr_rv_slot;
        let mut var_pzbd: f64 = *var_pzbd_slot;
        let mut var_pzbd_dn0: f64 = *var_pzbd_dn0_slot;
        let mut var_pzbd_dn10: f64 = *var_pzbd_dn10_slot;
        let mut var_pzbd_dn11: f64 = *var_pzbd_dn11_slot;
        let mut var_pzbd_dn14: f64 = *var_pzbd_dn14_slot;
        let mut var_pzbd_dn2: f64 = *var_pzbd_dn2_slot;
        let mut var_pzbd_dn4: f64 = *var_pzbd_dn4_slot;
        let mut var_pzbd_dn5: f64 = *var_pzbd_dn5_slot;
        let mut var_pzbd_dn6: f64 = *var_pzbd_dn6_slot;
        let mut var_pzbd_dn7: f64 = *var_pzbd_dn7_slot;
        let mut var_pzbd_dn8: f64 = *var_pzbd_dn8_slot;
        let mut var_pzbd_dn9: f64 = *var_pzbd_dn9_slot;
        let mut var_pzbd_rv: f64 = *var_pzbd_rv_slot;
        let mut var_pzbdsw: f64 = *var_pzbdsw_slot;
        let mut var_pzbdsw_dn0: f64 = *var_pzbdsw_dn0_slot;
        let mut var_pzbdsw_dn10: f64 = *var_pzbdsw_dn10_slot;
        let mut var_pzbdsw_dn11: f64 = *var_pzbdsw_dn11_slot;
        let mut var_pzbdsw_dn14: f64 = *var_pzbdsw_dn14_slot;
        let mut var_pzbdsw_dn2: f64 = *var_pzbdsw_dn2_slot;
        let mut var_pzbdsw_dn4: f64 = *var_pzbdsw_dn4_slot;
        let mut var_pzbdsw_dn5: f64 = *var_pzbdsw_dn5_slot;
        let mut var_pzbdsw_dn6: f64 = *var_pzbdsw_dn6_slot;
        let mut var_pzbdsw_dn7: f64 = *var_pzbdsw_dn7_slot;
        let mut var_pzbdsw_dn8: f64 = *var_pzbdsw_dn8_slot;
        let mut var_pzbdsw_dn9: f64 = *var_pzbdsw_dn9_slot;
        let mut var_pzbdsw_rv: f64 = *var_pzbdsw_rv_slot;
        let mut var_pzbdswg: f64 = *var_pzbdswg_slot;
        let mut var_pzbdswg_dn0: f64 = *var_pzbdswg_dn0_slot;
        let mut var_pzbdswg_dn10: f64 = *var_pzbdswg_dn10_slot;
        let mut var_pzbdswg_dn11: f64 = *var_pzbdswg_dn11_slot;
        let mut var_pzbdswg_dn14: f64 = *var_pzbdswg_dn14_slot;
        let mut var_pzbdswg_dn2: f64 = *var_pzbdswg_dn2_slot;
        let mut var_pzbdswg_dn4: f64 = *var_pzbdswg_dn4_slot;
        let mut var_pzbdswg_dn5: f64 = *var_pzbdswg_dn5_slot;
        let mut var_pzbdswg_dn6: f64 = *var_pzbdswg_dn6_slot;
        let mut var_pzbdswg_dn7: f64 = *var_pzbdswg_dn7_slot;
        let mut var_pzbdswg_dn8: f64 = *var_pzbdswg_dn8_slot;
        let mut var_pzbdswg_dn9: f64 = *var_pzbdswg_dn9_slot;
        let mut var_pzbdswg_rv: f64 = *var_pzbdswg_rv_slot;
        let mut var_pzbs: f64 = *var_pzbs_slot;
        let mut var_pzbs_dn0: f64 = *var_pzbs_dn0_slot;
        let mut var_pzbs_dn10: f64 = *var_pzbs_dn10_slot;
        let mut var_pzbs_dn11: f64 = *var_pzbs_dn11_slot;
        let mut var_pzbs_dn14: f64 = *var_pzbs_dn14_slot;
        let mut var_pzbs_dn2: f64 = *var_pzbs_dn2_slot;
        let mut var_pzbs_dn4: f64 = *var_pzbs_dn4_slot;
        let mut var_pzbs_dn5: f64 = *var_pzbs_dn5_slot;
        let mut var_pzbs_dn6: f64 = *var_pzbs_dn6_slot;
        let mut var_pzbs_dn7: f64 = *var_pzbs_dn7_slot;
        let mut var_pzbs_dn8: f64 = *var_pzbs_dn8_slot;
        let mut var_pzbs_dn9: f64 = *var_pzbs_dn9_slot;
        let mut var_pzbs_rv: f64 = *var_pzbs_rv_slot;
        let mut var_qbd: f64 = *var_qbd_slot;
        let mut var_qbd_dn0: f64 = *var_qbd_dn0_slot;
        let mut var_qbd_dn10: f64 = *var_qbd_dn10_slot;
        let mut var_qbd_dn11: f64 = *var_qbd_dn11_slot;
        let mut var_qbd_dn14: f64 = *var_qbd_dn14_slot;
        let mut var_qbd_dn16: f64 = *var_qbd_dn16_slot;
        let mut var_qbd_dn17: f64 = *var_qbd_dn17_slot;
        let mut var_qbd_dn18: f64 = *var_qbd_dn18_slot;
        let mut var_qbd_dn2: f64 = *var_qbd_dn2_slot;
        let mut var_qbd_dn4: f64 = *var_qbd_dn4_slot;
        let mut var_qbd_dn5: f64 = *var_qbd_dn5_slot;
        let mut var_qbd_dn6: f64 = *var_qbd_dn6_slot;
        let mut var_qbd_dn7: f64 = *var_qbd_dn7_slot;
        let mut var_qbd_dn8: f64 = *var_qbd_dn8_slot;
        let mut var_qbd_dn9: f64 = *var_qbd_dn9_slot;
        let mut var_qbd_rv: f64 = *var_qbd_rv_slot;
        let mut var_qbdi: f64 = *var_qbdi_slot;
        let mut var_qbdi_dn0: f64 = *var_qbdi_dn0_slot;
        let mut var_qbdi_dn10: f64 = *var_qbdi_dn10_slot;
        let mut var_qbdi_dn11: f64 = *var_qbdi_dn11_slot;
        let mut var_qbdi_dn14: f64 = *var_qbdi_dn14_slot;
        let mut var_qbdi_dn2: f64 = *var_qbdi_dn2_slot;
        let mut var_qbdi_dn4: f64 = *var_qbdi_dn4_slot;
        let mut var_qbdi_dn5: f64 = *var_qbdi_dn5_slot;
        let mut var_qbdi_dn6: f64 = *var_qbdi_dn6_slot;
        let mut var_qbdi_dn7: f64 = *var_qbdi_dn7_slot;
        let mut var_qbdi_dn8: f64 = *var_qbdi_dn8_slot;
        let mut var_qbdi_dn9: f64 = *var_qbdi_dn9_slot;
        let mut var_qbdi_rv: f64 = *var_qbdi_rv_slot;
        let mut var_qbs: f64 = *var_qbs_slot;
        let mut var_qbs_dn0: f64 = *var_qbs_dn0_slot;
        let mut var_qbs_dn10: f64 = *var_qbs_dn10_slot;
        let mut var_qbs_dn11: f64 = *var_qbs_dn11_slot;
        let mut var_qbs_dn14: f64 = *var_qbs_dn14_slot;
        let mut var_qbs_dn2: f64 = *var_qbs_dn2_slot;
        let mut var_qbs_dn4: f64 = *var_qbs_dn4_slot;
        let mut var_qbs_dn5: f64 = *var_qbs_dn5_slot;
        let mut var_qbs_dn6: f64 = *var_qbs_dn6_slot;
        let mut var_qbs_dn7: f64 = *var_qbs_dn7_slot;
        let mut var_qbs_dn8: f64 = *var_qbs_dn8_slot;
        let mut var_qbs_dn9: f64 = *var_qbs_dn9_slot;
        let mut var_qbs_rv: f64 = *var_qbs_rv_slot;
        let mut var_qbsi: f64 = *var_qbsi_slot;
        let mut var_qbsi_dn0: f64 = *var_qbsi_dn0_slot;
        let mut var_qbsi_dn10: f64 = *var_qbsi_dn10_slot;
        let mut var_qbsi_dn11: f64 = *var_qbsi_dn11_slot;
        let mut var_qbsi_dn14: f64 = *var_qbsi_dn14_slot;
        let mut var_qbsi_dn2: f64 = *var_qbsi_dn2_slot;
        let mut var_qbsi_dn4: f64 = *var_qbsi_dn4_slot;
        let mut var_qbsi_dn5: f64 = *var_qbsi_dn5_slot;
        let mut var_qbsi_dn6: f64 = *var_qbsi_dn6_slot;
        let mut var_qbsi_dn7: f64 = *var_qbsi_dn7_slot;
        let mut var_qbsi_dn8: f64 = *var_qbsi_dn8_slot;
        let mut var_qbsi_dn9: f64 = *var_qbsi_dn9_slot;
        let mut var_qbsi_rv: f64 = *var_qbsi_rv_slot;
        let mut var_qiu_noi: f64 = *var_qiu_noi_slot;
        let mut var_qiu_noi_dn0: f64 = *var_qiu_noi_dn0_slot;
        let mut var_qiu_noi_dn10: f64 = *var_qiu_noi_dn10_slot;
        let mut var_qiu_noi_dn11: f64 = *var_qiu_noi_dn11_slot;
        let mut var_qiu_noi_dn14: f64 = *var_qiu_noi_dn14_slot;
        let mut var_qiu_noi_dn2: f64 = *var_qiu_noi_dn2_slot;
        let mut var_qiu_noi_dn4: f64 = *var_qiu_noi_dn4_slot;
        let mut var_qiu_noi_dn5: f64 = *var_qiu_noi_dn5_slot;
        let mut var_qiu_noi_dn6: f64 = *var_qiu_noi_dn6_slot;
        let mut var_qiu_noi_dn7: f64 = *var_qiu_noi_dn7_slot;
        let mut var_qiu_noi_dn8: f64 = *var_qiu_noi_dn8_slot;
        let mut var_qiu_noi_dn9: f64 = *var_qiu_noi_dn9_slot;
        let mut var_qiu_noi_rv: f64 = *var_qiu_noi_rv_slot;
        let mut var_sqrt_eg: f64 = *var_sqrt_eg_slot;
        let mut var_sqrt_eg_dn0: f64 = *var_sqrt_eg_dn0_slot;
        let mut var_sqrt_eg_dn10: f64 = *var_sqrt_eg_dn10_slot;
        let mut var_sqrt_eg_dn11: f64 = *var_sqrt_eg_dn11_slot;
        let mut var_sqrt_eg_dn14: f64 = *var_sqrt_eg_dn14_slot;
        let mut var_sqrt_eg_dn2: f64 = *var_sqrt_eg_dn2_slot;
        let mut var_sqrt_eg_dn4: f64 = *var_sqrt_eg_dn4_slot;
        let mut var_sqrt_eg_dn5: f64 = *var_sqrt_eg_dn5_slot;
        let mut var_sqrt_eg_dn6: f64 = *var_sqrt_eg_dn6_slot;
        let mut var_sqrt_eg_dn7: f64 = *var_sqrt_eg_dn7_slot;
        let mut var_sqrt_eg_dn8: f64 = *var_sqrt_eg_dn8_slot;
        let mut var_sqrt_eg_dn9: f64 = *var_sqrt_eg_dn9_slot;
        let mut var_sqrt_eg_rv: f64 = *var_sqrt_eg_rv_slot;
        let mut var_uc_rdrbb: f64 = *var_uc_rdrbb_slot;
        let mut var_uc_rdrbb_dn0: f64 = *var_uc_rdrbb_dn0_slot;
        let mut var_uc_rdrbb_dn10: f64 = *var_uc_rdrbb_dn10_slot;
        let mut var_uc_rdrbb_dn11: f64 = *var_uc_rdrbb_dn11_slot;
        let mut var_uc_rdrbb_dn14: f64 = *var_uc_rdrbb_dn14_slot;
        let mut var_uc_rdrbb_dn2: f64 = *var_uc_rdrbb_dn2_slot;
        let mut var_uc_rdrbb_dn4: f64 = *var_uc_rdrbb_dn4_slot;
        let mut var_uc_rdrbb_dn5: f64 = *var_uc_rdrbb_dn5_slot;
        let mut var_uc_rdrbb_dn6: f64 = *var_uc_rdrbb_dn6_slot;
        let mut var_uc_rdrbb_dn7: f64 = *var_uc_rdrbb_dn7_slot;
        let mut var_uc_rdrbb_dn8: f64 = *var_uc_rdrbb_dn8_slot;
        let mut var_uc_rdrbb_dn9: f64 = *var_uc_rdrbb_dn9_slot;
        let mut var_uc_rdrbb_rv: f64 = *var_uc_rdrbb_rv_slot;
        let mut var_uc_rdrbb_s: f64 = *var_uc_rdrbb_s_slot;
        let mut var_uc_rdrbb_s_dn0: f64 = *var_uc_rdrbb_s_dn0_slot;
        let mut var_uc_rdrbb_s_dn10: f64 = *var_uc_rdrbb_s_dn10_slot;
        let mut var_uc_rdrbb_s_dn11: f64 = *var_uc_rdrbb_s_dn11_slot;
        let mut var_uc_rdrbb_s_dn14: f64 = *var_uc_rdrbb_s_dn14_slot;
        let mut var_uc_rdrbb_s_dn2: f64 = *var_uc_rdrbb_s_dn2_slot;
        let mut var_uc_rdrbb_s_dn4: f64 = *var_uc_rdrbb_s_dn4_slot;
        let mut var_uc_rdrbb_s_dn5: f64 = *var_uc_rdrbb_s_dn5_slot;
        let mut var_uc_rdrbb_s_dn6: f64 = *var_uc_rdrbb_s_dn6_slot;
        let mut var_uc_rdrbb_s_dn7: f64 = *var_uc_rdrbb_s_dn7_slot;
        let mut var_uc_rdrbb_s_dn8: f64 = *var_uc_rdrbb_s_dn8_slot;
        let mut var_uc_rdrbb_s_dn9: f64 = *var_uc_rdrbb_s_dn9_slot;
        let mut var_uc_rdrbb_s_rv: f64 = *var_uc_rdrbb_s_rv_slot;
        let mut var_uc_subtmp: f64 = *var_uc_subtmp_slot;
        let mut var_uc_subtmp_rv: f64 = *var_uc_subtmp_rv_slot;
        let mut var_vbipn: f64 = *var_vbipn_slot;
        let mut var_vbipn_dn0: f64 = *var_vbipn_dn0_slot;
        let mut var_vbipn_dn10: f64 = *var_vbipn_dn10_slot;
        let mut var_vbipn_dn11: f64 = *var_vbipn_dn11_slot;
        let mut var_vbipn_dn14: f64 = *var_vbipn_dn14_slot;
        let mut var_vbipn_dn2: f64 = *var_vbipn_dn2_slot;
        let mut var_vbipn_dn4: f64 = *var_vbipn_dn4_slot;
        let mut var_vbipn_dn5: f64 = *var_vbipn_dn5_slot;
        let mut var_vbipn_dn6: f64 = *var_vbipn_dn6_slot;
        let mut var_vbipn_dn7: f64 = *var_vbipn_dn7_slot;
        let mut var_vbipn_dn8: f64 = *var_vbipn_dn8_slot;
        let mut var_vbipn_dn9: f64 = *var_vbipn_dn9_slot;
        let mut var_vbipn_rv: f64 = *var_vbipn_rv_slot;
        let mut var_wdpl: f64 = *var_wdpl_slot;
        let mut var_wdpl_dn0: f64 = *var_wdpl_dn0_slot;
        let mut var_wdpl_dn10: f64 = *var_wdpl_dn10_slot;
        let mut var_wdpl_dn11: f64 = *var_wdpl_dn11_slot;
        let mut var_wdpl_dn14: f64 = *var_wdpl_dn14_slot;
        let mut var_wdpl_dn2: f64 = *var_wdpl_dn2_slot;
        let mut var_wdpl_dn4: f64 = *var_wdpl_dn4_slot;
        let mut var_wdpl_dn5: f64 = *var_wdpl_dn5_slot;
        let mut var_wdpl_dn6: f64 = *var_wdpl_dn6_slot;
        let mut var_wdpl_dn7: f64 = *var_wdpl_dn7_slot;
        let mut var_wdpl_dn8: f64 = *var_wdpl_dn8_slot;
        let mut var_wdpl_dn9: f64 = *var_wdpl_dn9_slot;
        let mut var_wdpl_rv: f64 = *var_wdpl_rv_slot;
        let mut var_wdplp: f64 = *var_wdplp_slot;
        let mut var_wdplp_dn0: f64 = *var_wdplp_dn0_slot;
        let mut var_wdplp_dn10: f64 = *var_wdplp_dn10_slot;
        let mut var_wdplp_dn11: f64 = *var_wdplp_dn11_slot;
        let mut var_wdplp_dn14: f64 = *var_wdplp_dn14_slot;
        let mut var_wdplp_dn2: f64 = *var_wdplp_dn2_slot;
        let mut var_wdplp_dn4: f64 = *var_wdplp_dn4_slot;
        let mut var_wdplp_dn5: f64 = *var_wdplp_dn5_slot;
        let mut var_wdplp_dn6: f64 = *var_wdplp_dn6_slot;
        let mut var_wdplp_dn7: f64 = *var_wdplp_dn7_slot;
        let mut var_wdplp_dn8: f64 = *var_wdplp_dn8_slot;
        let mut var_wdplp_dn9: f64 = *var_wdplp_dn9_slot;
        let mut var_wdplp_rv: f64 = *var_wdplp_rv_slot;

        var_costi0_p2 = 0.0;
        var_costi0_p2_dn0 = 0.0;
        var_costi0_p2_dn2 = 0.0;
        var_costi0_p2_dn4 = 0.0;
        var_costi0_p2_dn5 = 0.0;
        var_costi0_p2_dn6 = 0.0;
        var_costi0_p2_dn7 = 0.0;
        var_costi0_p2_dn8 = 0.0;
        var_costi0_p2_dn9 = 0.0;
        var_costi0_p2_dn10 = 0.0;
        var_costi0_p2_dn11 = 0.0;
        var_costi0_p2_dn14 = 0.0;
        var_costi0_p2_rv = 0.0;

        var_mphn0 = 0.0;
        var_mphn0_dn0 = 0.0;
        var_mphn0_dn2 = 0.0;
        var_mphn0_dn4 = 0.0;
        var_mphn0_dn5 = 0.0;
        var_mphn0_dn6 = 0.0;
        var_mphn0_dn7 = 0.0;
        var_mphn0_dn8 = 0.0;
        var_mphn0_dn9 = 0.0;
        var_mphn0_dn10 = 0.0;
        var_mphn0_dn11 = 0.0;
        var_mphn0_dn14 = 0.0;
        var_mphn0_rv = 0.0;

        var_powratio = 0.0;
        var_powratio_dn0 = 0.0;
        var_powratio_dn2 = 0.0;
        var_powratio_dn4 = 0.0;
        var_powratio_dn5 = 0.0;
        var_powratio_dn6 = 0.0;
        var_powratio_dn7 = 0.0;
        var_powratio_dn8 = 0.0;
        var_powratio_dn9 = 0.0;
        var_powratio_dn10 = 0.0;
        var_powratio_dn11 = 0.0;
        var_powratio_dn14 = 0.0;
        var_powratio_rv = 0.0;

        var_ptovr = 0.0;
        var_ptovr_dn0 = 0.0;
        var_ptovr_dn2 = 0.0;
        var_ptovr_dn4 = 0.0;
        var_ptovr_dn5 = 0.0;
        var_ptovr_dn6 = 0.0;
        var_ptovr_dn7 = 0.0;
        var_ptovr_dn8 = 0.0;
        var_ptovr_dn9 = 0.0;
        var_ptovr_dn10 = 0.0;
        var_ptovr_dn11 = 0.0;
        var_ptovr_dn14 = 0.0;
        var_ptovr_rv = 0.0;

        var_sqrt_eg = 0.0;
        var_sqrt_eg_dn0 = 0.0;
        var_sqrt_eg_dn2 = 0.0;
        var_sqrt_eg_dn4 = 0.0;
        var_sqrt_eg_dn5 = 0.0;
        var_sqrt_eg_dn6 = 0.0;
        var_sqrt_eg_dn7 = 0.0;
        var_sqrt_eg_dn8 = 0.0;
        var_sqrt_eg_dn9 = 0.0;
        var_sqrt_eg_dn10 = 0.0;
        var_sqrt_eg_dn11 = 0.0;
        var_sqrt_eg_dn14 = 0.0;
        var_sqrt_eg_rv = 0.0;

        var_wdpl = 0.0;
        var_wdpl_dn0 = 0.0;
        var_wdpl_dn2 = 0.0;
        var_wdpl_dn4 = 0.0;
        var_wdpl_dn5 = 0.0;
        var_wdpl_dn6 = 0.0;
        var_wdpl_dn7 = 0.0;
        var_wdpl_dn8 = 0.0;
        var_wdpl_dn9 = 0.0;
        var_wdpl_dn10 = 0.0;
        var_wdpl_dn11 = 0.0;
        var_wdpl_dn14 = 0.0;
        var_wdpl_rv = 0.0;

        var_wdplp = 0.0;
        var_wdplp_dn0 = 0.0;
        var_wdplp_dn2 = 0.0;
        var_wdplp_dn4 = 0.0;
        var_wdplp_dn5 = 0.0;
        var_wdplp_dn6 = 0.0;
        var_wdplp_dn7 = 0.0;
        var_wdplp_dn8 = 0.0;
        var_wdplp_dn9 = 0.0;
        var_wdplp_dn10 = 0.0;
        var_wdplp_dn11 = 0.0;
        var_wdplp_dn14 = 0.0;
        var_wdplp_rv = 0.0;

        var_uc_rdrbb = p.p436;
        var_uc_rdrbb_dn0 = 0.0;
        var_uc_rdrbb_dn2 = 0.0;
        var_uc_rdrbb_dn4 = 0.0;
        var_uc_rdrbb_dn5 = 0.0;
        var_uc_rdrbb_dn6 = 0.0;
        var_uc_rdrbb_dn7 = 0.0;
        var_uc_rdrbb_dn8 = 0.0;
        var_uc_rdrbb_dn9 = 0.0;
        var_uc_rdrbb_dn10 = 0.0;
        var_uc_rdrbb_dn11 = 0.0;
        var_uc_rdrbb_dn14 = 0.0;
        var_uc_rdrbb_rv = 0.0;

        var_uc_rdrbb_s = p.p437;
        var_uc_rdrbb_s_dn0 = 0.0;
        var_uc_rdrbb_s_dn2 = 0.0;
        var_uc_rdrbb_s_dn4 = 0.0;
        var_uc_rdrbb_s_dn5 = 0.0;
        var_uc_rdrbb_s_dn6 = 0.0;
        var_uc_rdrbb_s_dn7 = 0.0;
        var_uc_rdrbb_s_dn8 = 0.0;
        var_uc_rdrbb_s_dn9 = 0.0;
        var_uc_rdrbb_s_dn10 = 0.0;
        var_uc_rdrbb_s_dn11 = 0.0;
        var_uc_rdrbb_s_dn14 = 0.0;
        var_uc_rdrbb_s_rv = 0.0;

        var_ids_acc = 0.0;
        var_ids_acc_dn0 = 0.0;
        var_ids_acc_dn2 = 0.0;
        var_ids_acc_dn4 = 0.0;
        var_ids_acc_dn5 = 0.0;
        var_ids_acc_dn6 = 0.0;
        var_ids_acc_dn7 = 0.0;
        var_ids_acc_dn8 = 0.0;
        var_ids_acc_dn9 = 0.0;
        var_ids_acc_dn10 = 0.0;
        var_ids_acc_dn11 = 0.0;
        var_ids_acc_dn14 = 0.0;
        var_ids_acc_rv = 0.0;

        var_ids_res = 0.0;
        var_ids_res_dn0 = 0.0;
        var_ids_res_dn2 = 0.0;
        var_ids_res_dn4 = 0.0;
        var_ids_res_dn5 = 0.0;
        var_ids_res_dn6 = 0.0;
        var_ids_res_dn7 = 0.0;
        var_ids_res_dn8 = 0.0;
        var_ids_res_dn9 = 0.0;
        var_ids_res_dn10 = 0.0;
        var_ids_res_dn11 = 0.0;
        var_ids_res_dn14 = 0.0;
        var_ids_res_rv = 0.0;

        var_ires_leak = 0.0;
        var_ires_leak_dn0 = 0.0;
        var_ires_leak_dn2 = 0.0;
        var_ires_leak_dn4 = 0.0;
        var_ires_leak_dn5 = 0.0;
        var_ires_leak_dn6 = 0.0;
        var_ires_leak_dn7 = 0.0;
        var_ires_leak_dn8 = 0.0;
        var_ires_leak_dn9 = 0.0;
        var_ires_leak_dn10 = 0.0;
        var_ires_leak_dn11 = 0.0;
        var_ires_leak_dn14 = 0.0;
        var_ires_leak_rv = 0.0;

        var_pb2n = 0.0;
        var_pb2n_dn0 = 0.0;
        var_pb2n_dn2 = 0.0;
        var_pb2n_dn4 = 0.0;
        var_pb2n_dn5 = 0.0;
        var_pb2n_dn6 = 0.0;
        var_pb2n_dn7 = 0.0;
        var_pb2n_dn8 = 0.0;
        var_pb2n_dn9 = 0.0;
        var_pb2n_dn10 = 0.0;
        var_pb2n_dn11 = 0.0;
        var_pb2n_dn14 = 0.0;
        var_pb2n_rv = 0.0;

        var_vbipn = 0.0;
        var_vbipn_dn0 = 0.0;
        var_vbipn_dn2 = 0.0;
        var_vbipn_dn4 = 0.0;
        var_vbipn_dn5 = 0.0;
        var_vbipn_dn6 = 0.0;
        var_vbipn_dn7 = 0.0;
        var_vbipn_dn8 = 0.0;
        var_vbipn_dn9 = 0.0;
        var_vbipn_dn10 = 0.0;
        var_vbipn_dn11 = 0.0;
        var_vbipn_dn14 = 0.0;
        var_vbipn_rv = 0.0;

        var_hbdceff = p.p447;
        var_hbdceff_dn0 = 0.0;
        var_hbdceff_dn2 = 0.0;
        var_hbdceff_dn4 = 0.0;
        var_hbdceff_dn5 = 0.0;
        var_hbdceff_dn6 = 0.0;
        var_hbdceff_dn7 = 0.0;
        var_hbdceff_dn8 = 0.0;
        var_hbdceff_dn9 = 0.0;
        var_hbdceff_dn10 = 0.0;
        var_hbdceff_dn11 = 0.0;
        var_hbdceff_dn14 = 0.0;
        var_hbdceff_rv = 0.0;

        var_uc_subtmp = p.p193;
        var_uc_subtmp_rv = 0.0;

        var_depmphn0 = 0.0;
        var_depmphn0_dn0 = 0.0;
        var_depmphn0_dn2 = 0.0;
        var_depmphn0_dn4 = 0.0;
        var_depmphn0_dn5 = 0.0;
        var_depmphn0_dn6 = 0.0;
        var_depmphn0_dn7 = 0.0;
        var_depmphn0_dn8 = 0.0;
        var_depmphn0_dn9 = 0.0;
        var_depmphn0_dn10 = 0.0;
        var_depmphn0_dn11 = 0.0;
        var_depmphn0_dn14 = 0.0;
        var_depmphn0_rv = 0.0;

        var_qiu_noi = 0.0;
        var_qiu_noi_dn0 = 0.0;
        var_qiu_noi_dn2 = 0.0;
        var_qiu_noi_dn4 = 0.0;
        var_qiu_noi_dn5 = 0.0;
        var_qiu_noi_dn6 = 0.0;
        var_qiu_noi_dn7 = 0.0;
        var_qiu_noi_dn8 = 0.0;
        var_qiu_noi_dn9 = 0.0;
        var_qiu_noi_dn10 = 0.0;
        var_qiu_noi_dn11 = 0.0;
        var_qiu_noi_dn14 = 0.0;
        var_qiu_noi_rv = 0.0;

        var_lp_s0_max = 40.0;
        var_lp_s0_max_rv = 0.0;

        var_js = 0.0;
        var_js_dn0 = 0.0;
        var_js_dn2 = 0.0;
        var_js_dn4 = 0.0;
        var_js_dn5 = 0.0;
        var_js_dn6 = 0.0;
        var_js_dn7 = 0.0;
        var_js_dn8 = 0.0;
        var_js_dn9 = 0.0;
        var_js_dn10 = 0.0;
        var_js_dn11 = 0.0;
        var_js_dn14 = 0.0;
        var_js_rv = 0.0;

        var_jssw = 0.0;
        var_jssw_dn0 = 0.0;
        var_jssw_dn2 = 0.0;
        var_jssw_dn4 = 0.0;
        var_jssw_dn5 = 0.0;
        var_jssw_dn6 = 0.0;
        var_jssw_dn7 = 0.0;
        var_jssw_dn8 = 0.0;
        var_jssw_dn9 = 0.0;
        var_jssw_dn10 = 0.0;
        var_jssw_dn11 = 0.0;
        var_jssw_dn14 = 0.0;
        var_jssw_rv = 0.0;

        var_js2 = 0.0;
        var_js2_dn0 = 0.0;
        var_js2_dn2 = 0.0;
        var_js2_dn4 = 0.0;
        var_js2_dn5 = 0.0;
        var_js2_dn6 = 0.0;
        var_js2_dn7 = 0.0;
        var_js2_dn8 = 0.0;
        var_js2_dn9 = 0.0;
        var_js2_dn10 = 0.0;
        var_js2_dn11 = 0.0;
        var_js2_dn14 = 0.0;
        var_js2_rv = 0.0;

        var_jssw2 = 0.0;
        var_jssw2_dn0 = 0.0;
        var_jssw2_dn2 = 0.0;
        var_jssw2_dn4 = 0.0;
        var_jssw2_dn5 = 0.0;
        var_jssw2_dn6 = 0.0;
        var_jssw2_dn7 = 0.0;
        var_jssw2_dn8 = 0.0;
        var_jssw2_dn9 = 0.0;
        var_jssw2_dn10 = 0.0;
        var_jssw2_dn11 = 0.0;
        var_jssw2_dn14 = 0.0;
        var_jssw2_rv = 0.0;

        var_qbs = 0.0;
        var_qbs_dn0 = 0.0;
        var_qbs_dn2 = 0.0;
        var_qbs_dn4 = 0.0;
        var_qbs_dn5 = 0.0;
        var_qbs_dn6 = 0.0;
        var_qbs_dn7 = 0.0;
        var_qbs_dn8 = 0.0;
        var_qbs_dn9 = 0.0;
        var_qbs_dn10 = 0.0;
        var_qbs_dn11 = 0.0;
        var_qbs_dn14 = 0.0;
        var_qbs_rv = 0.0;

        var_qbd = 0.0;
        var_qbd_dn0 = 0.0;
        var_qbd_dn2 = 0.0;
        var_qbd_dn4 = 0.0;
        var_qbd_dn5 = 0.0;
        var_qbd_dn6 = 0.0;
        var_qbd_dn7 = 0.0;
        var_qbd_dn8 = 0.0;
        var_qbd_dn9 = 0.0;
        var_qbd_dn10 = 0.0;
        var_qbd_dn11 = 0.0;
        var_qbd_dn14 = 0.0;
        var_qbd_dn16 = 0.0;
        var_qbd_dn17 = 0.0;
        var_qbd_dn18 = 0.0;
        var_qbd_rv = 0.0;

        var_qbsi = 0.0;
        var_qbsi_dn0 = 0.0;
        var_qbsi_dn2 = 0.0;
        var_qbsi_dn4 = 0.0;
        var_qbsi_dn5 = 0.0;
        var_qbsi_dn6 = 0.0;
        var_qbsi_dn7 = 0.0;
        var_qbsi_dn8 = 0.0;
        var_qbsi_dn9 = 0.0;
        var_qbsi_dn10 = 0.0;
        var_qbsi_dn11 = 0.0;
        var_qbsi_dn14 = 0.0;
        var_qbsi_rv = 0.0;

        var_qbdi = 0.0;
        var_qbdi_dn0 = 0.0;
        var_qbdi_dn2 = 0.0;
        var_qbdi_dn4 = 0.0;
        var_qbdi_dn5 = 0.0;
        var_qbdi_dn6 = 0.0;
        var_qbdi_dn7 = 0.0;
        var_qbdi_dn8 = 0.0;
        var_qbdi_dn9 = 0.0;
        var_qbdi_dn10 = 0.0;
        var_qbdi_dn11 = 0.0;
        var_qbdi_dn14 = 0.0;
        var_qbdi_rv = 0.0;

        var_czbd = 0.0;
        var_czbd_dn0 = 0.0;
        var_czbd_dn2 = 0.0;
        var_czbd_dn4 = 0.0;
        var_czbd_dn5 = 0.0;
        var_czbd_dn6 = 0.0;
        var_czbd_dn7 = 0.0;
        var_czbd_dn8 = 0.0;
        var_czbd_dn9 = 0.0;
        var_czbd_dn10 = 0.0;
        var_czbd_dn11 = 0.0;
        var_czbd_dn14 = 0.0;
        var_czbd_rv = 0.0;

        var_czbdsw = 0.0;
        var_czbdsw_dn0 = 0.0;
        var_czbdsw_dn2 = 0.0;
        var_czbdsw_dn4 = 0.0;
        var_czbdsw_dn5 = 0.0;
        var_czbdsw_dn6 = 0.0;
        var_czbdsw_dn7 = 0.0;
        var_czbdsw_dn8 = 0.0;
        var_czbdsw_dn9 = 0.0;
        var_czbdsw_dn10 = 0.0;
        var_czbdsw_dn11 = 0.0;
        var_czbdsw_dn14 = 0.0;
        var_czbdsw_rv = 0.0;

        var_czbdswg = 0.0;
        var_czbdswg_dn0 = 0.0;
        var_czbdswg_dn2 = 0.0;
        var_czbdswg_dn4 = 0.0;
        var_czbdswg_dn5 = 0.0;
        var_czbdswg_dn6 = 0.0;
        var_czbdswg_dn7 = 0.0;
        var_czbdswg_dn8 = 0.0;
        var_czbdswg_dn9 = 0.0;
        var_czbdswg_dn10 = 0.0;
        var_czbdswg_dn11 = 0.0;
        var_czbdswg_dn14 = 0.0;
        var_czbdswg_rv = 0.0;

        var_czbs = 0.0;
        var_czbs_dn0 = 0.0;
        var_czbs_dn2 = 0.0;
        var_czbs_dn4 = 0.0;
        var_czbs_dn5 = 0.0;
        var_czbs_dn6 = 0.0;
        var_czbs_dn7 = 0.0;
        var_czbs_dn8 = 0.0;
        var_czbs_dn9 = 0.0;
        var_czbs_dn10 = 0.0;
        var_czbs_dn11 = 0.0;
        var_czbs_dn14 = 0.0;
        var_czbs_rv = 0.0;

        var_czbssw = 0.0;
        var_czbssw_dn0 = 0.0;
        var_czbssw_dn2 = 0.0;
        var_czbssw_dn4 = 0.0;
        var_czbssw_dn5 = 0.0;
        var_czbssw_dn6 = 0.0;
        var_czbssw_dn7 = 0.0;
        var_czbssw_dn8 = 0.0;
        var_czbssw_dn9 = 0.0;
        var_czbssw_dn10 = 0.0;
        var_czbssw_dn11 = 0.0;
        var_czbssw_dn14 = 0.0;
        var_czbssw_rv = 0.0;

        var_czbsswg = 0.0;
        var_czbsswg_dn0 = 0.0;
        var_czbsswg_dn2 = 0.0;
        var_czbsswg_dn4 = 0.0;
        var_czbsswg_dn5 = 0.0;
        var_czbsswg_dn6 = 0.0;
        var_czbsswg_dn7 = 0.0;
        var_czbsswg_dn8 = 0.0;
        var_czbsswg_dn9 = 0.0;
        var_czbsswg_dn10 = 0.0;
        var_czbsswg_dn11 = 0.0;
        var_czbsswg_dn14 = 0.0;
        var_czbsswg_rv = 0.0;

        var_pzbd = 0.0;
        var_pzbd_dn0 = 0.0;
        var_pzbd_dn2 = 0.0;
        var_pzbd_dn4 = 0.0;
        var_pzbd_dn5 = 0.0;
        var_pzbd_dn6 = 0.0;
        var_pzbd_dn7 = 0.0;
        var_pzbd_dn8 = 0.0;
        var_pzbd_dn9 = 0.0;
        var_pzbd_dn10 = 0.0;
        var_pzbd_dn11 = 0.0;
        var_pzbd_dn14 = 0.0;
        var_pzbd_rv = 0.0;

        var_pzbdsw = 0.0;
        var_pzbdsw_dn0 = 0.0;
        var_pzbdsw_dn2 = 0.0;
        var_pzbdsw_dn4 = 0.0;
        var_pzbdsw_dn5 = 0.0;
        var_pzbdsw_dn6 = 0.0;
        var_pzbdsw_dn7 = 0.0;
        var_pzbdsw_dn8 = 0.0;
        var_pzbdsw_dn9 = 0.0;
        var_pzbdsw_dn10 = 0.0;
        var_pzbdsw_dn11 = 0.0;
        var_pzbdsw_dn14 = 0.0;
        var_pzbdsw_rv = 0.0;

        var_pzbdswg = 0.0;
        var_pzbdswg_dn0 = 0.0;
        var_pzbdswg_dn2 = 0.0;
        var_pzbdswg_dn4 = 0.0;
        var_pzbdswg_dn5 = 0.0;
        var_pzbdswg_dn6 = 0.0;
        var_pzbdswg_dn7 = 0.0;
        var_pzbdswg_dn8 = 0.0;
        var_pzbdswg_dn9 = 0.0;
        var_pzbdswg_dn10 = 0.0;
        var_pzbdswg_dn11 = 0.0;
        var_pzbdswg_dn14 = 0.0;
        var_pzbdswg_rv = 0.0;

        var_pzbs = 0.0;
        var_pzbs_dn0 = 0.0;
        var_pzbs_dn2 = 0.0;
        var_pzbs_dn4 = 0.0;
        var_pzbs_dn5 = 0.0;
        var_pzbs_dn6 = 0.0;
        var_pzbs_dn7 = 0.0;
        var_pzbs_dn8 = 0.0;
        var_pzbs_dn9 = 0.0;
        var_pzbs_dn10 = 0.0;
        var_pzbs_dn11 = 0.0;
        var_pzbs_dn14 = 0.0;
        var_pzbs_rv = 0.0;

        *var_costi0_p2_slot = var_costi0_p2;
        *var_costi0_p2_dn0_slot = var_costi0_p2_dn0;
        *var_costi0_p2_dn10_slot = var_costi0_p2_dn10;
        *var_costi0_p2_dn11_slot = var_costi0_p2_dn11;
        *var_costi0_p2_dn14_slot = var_costi0_p2_dn14;
        *var_costi0_p2_dn2_slot = var_costi0_p2_dn2;
        *var_costi0_p2_dn4_slot = var_costi0_p2_dn4;
        *var_costi0_p2_dn5_slot = var_costi0_p2_dn5;
        *var_costi0_p2_dn6_slot = var_costi0_p2_dn6;
        *var_costi0_p2_dn7_slot = var_costi0_p2_dn7;
        *var_costi0_p2_dn8_slot = var_costi0_p2_dn8;
        *var_costi0_p2_dn9_slot = var_costi0_p2_dn9;
        *var_costi0_p2_rv_slot = var_costi0_p2_rv;
        *var_czbd_slot = var_czbd;
        *var_czbd_dn0_slot = var_czbd_dn0;
        *var_czbd_dn10_slot = var_czbd_dn10;
        *var_czbd_dn11_slot = var_czbd_dn11;
        *var_czbd_dn14_slot = var_czbd_dn14;
        *var_czbd_dn2_slot = var_czbd_dn2;
        *var_czbd_dn4_slot = var_czbd_dn4;
        *var_czbd_dn5_slot = var_czbd_dn5;
        *var_czbd_dn6_slot = var_czbd_dn6;
        *var_czbd_dn7_slot = var_czbd_dn7;
        *var_czbd_dn8_slot = var_czbd_dn8;
        *var_czbd_dn9_slot = var_czbd_dn9;
        *var_czbd_rv_slot = var_czbd_rv;
        *var_czbdsw_slot = var_czbdsw;
        *var_czbdsw_dn0_slot = var_czbdsw_dn0;
        *var_czbdsw_dn10_slot = var_czbdsw_dn10;
        *var_czbdsw_dn11_slot = var_czbdsw_dn11;
        *var_czbdsw_dn14_slot = var_czbdsw_dn14;
        *var_czbdsw_dn2_slot = var_czbdsw_dn2;
        *var_czbdsw_dn4_slot = var_czbdsw_dn4;
        *var_czbdsw_dn5_slot = var_czbdsw_dn5;
        *var_czbdsw_dn6_slot = var_czbdsw_dn6;
        *var_czbdsw_dn7_slot = var_czbdsw_dn7;
        *var_czbdsw_dn8_slot = var_czbdsw_dn8;
        *var_czbdsw_dn9_slot = var_czbdsw_dn9;
        *var_czbdsw_rv_slot = var_czbdsw_rv;
        *var_czbdswg_slot = var_czbdswg;
        *var_czbdswg_dn0_slot = var_czbdswg_dn0;
        *var_czbdswg_dn10_slot = var_czbdswg_dn10;
        *var_czbdswg_dn11_slot = var_czbdswg_dn11;
        *var_czbdswg_dn14_slot = var_czbdswg_dn14;
        *var_czbdswg_dn2_slot = var_czbdswg_dn2;
        *var_czbdswg_dn4_slot = var_czbdswg_dn4;
        *var_czbdswg_dn5_slot = var_czbdswg_dn5;
        *var_czbdswg_dn6_slot = var_czbdswg_dn6;
        *var_czbdswg_dn7_slot = var_czbdswg_dn7;
        *var_czbdswg_dn8_slot = var_czbdswg_dn8;
        *var_czbdswg_dn9_slot = var_czbdswg_dn9;
        *var_czbdswg_rv_slot = var_czbdswg_rv;
        *var_czbs_slot = var_czbs;
        *var_czbs_dn0_slot = var_czbs_dn0;
        *var_czbs_dn10_slot = var_czbs_dn10;
        *var_czbs_dn11_slot = var_czbs_dn11;
        *var_czbs_dn14_slot = var_czbs_dn14;
        *var_czbs_dn2_slot = var_czbs_dn2;
        *var_czbs_dn4_slot = var_czbs_dn4;
        *var_czbs_dn5_slot = var_czbs_dn5;
        *var_czbs_dn6_slot = var_czbs_dn6;
        *var_czbs_dn7_slot = var_czbs_dn7;
        *var_czbs_dn8_slot = var_czbs_dn8;
        *var_czbs_dn9_slot = var_czbs_dn9;
        *var_czbs_rv_slot = var_czbs_rv;
        *var_czbssw_slot = var_czbssw;
        *var_czbssw_dn0_slot = var_czbssw_dn0;
        *var_czbssw_dn10_slot = var_czbssw_dn10;
        *var_czbssw_dn11_slot = var_czbssw_dn11;
        *var_czbssw_dn14_slot = var_czbssw_dn14;
        *var_czbssw_dn2_slot = var_czbssw_dn2;
        *var_czbssw_dn4_slot = var_czbssw_dn4;
        *var_czbssw_dn5_slot = var_czbssw_dn5;
        *var_czbssw_dn6_slot = var_czbssw_dn6;
        *var_czbssw_dn7_slot = var_czbssw_dn7;
        *var_czbssw_dn8_slot = var_czbssw_dn8;
        *var_czbssw_dn9_slot = var_czbssw_dn9;
        *var_czbssw_rv_slot = var_czbssw_rv;
        *var_czbsswg_slot = var_czbsswg;
        *var_czbsswg_dn0_slot = var_czbsswg_dn0;
        *var_czbsswg_dn10_slot = var_czbsswg_dn10;
        *var_czbsswg_dn11_slot = var_czbsswg_dn11;
        *var_czbsswg_dn14_slot = var_czbsswg_dn14;
        *var_czbsswg_dn2_slot = var_czbsswg_dn2;
        *var_czbsswg_dn4_slot = var_czbsswg_dn4;
        *var_czbsswg_dn5_slot = var_czbsswg_dn5;
        *var_czbsswg_dn6_slot = var_czbsswg_dn6;
        *var_czbsswg_dn7_slot = var_czbsswg_dn7;
        *var_czbsswg_dn8_slot = var_czbsswg_dn8;
        *var_czbsswg_dn9_slot = var_czbsswg_dn9;
        *var_czbsswg_rv_slot = var_czbsswg_rv;
        *var_depmphn0_slot = var_depmphn0;
        *var_depmphn0_dn0_slot = var_depmphn0_dn0;
        *var_depmphn0_dn10_slot = var_depmphn0_dn10;
        *var_depmphn0_dn11_slot = var_depmphn0_dn11;
        *var_depmphn0_dn14_slot = var_depmphn0_dn14;
        *var_depmphn0_dn2_slot = var_depmphn0_dn2;
        *var_depmphn0_dn4_slot = var_depmphn0_dn4;
        *var_depmphn0_dn5_slot = var_depmphn0_dn5;
        *var_depmphn0_dn6_slot = var_depmphn0_dn6;
        *var_depmphn0_dn7_slot = var_depmphn0_dn7;
        *var_depmphn0_dn8_slot = var_depmphn0_dn8;
        *var_depmphn0_dn9_slot = var_depmphn0_dn9;
        *var_depmphn0_rv_slot = var_depmphn0_rv;
        *var_hbdceff_slot = var_hbdceff;
        *var_hbdceff_dn0_slot = var_hbdceff_dn0;
        *var_hbdceff_dn10_slot = var_hbdceff_dn10;
        *var_hbdceff_dn11_slot = var_hbdceff_dn11;
        *var_hbdceff_dn14_slot = var_hbdceff_dn14;
        *var_hbdceff_dn2_slot = var_hbdceff_dn2;
        *var_hbdceff_dn4_slot = var_hbdceff_dn4;
        *var_hbdceff_dn5_slot = var_hbdceff_dn5;
        *var_hbdceff_dn6_slot = var_hbdceff_dn6;
        *var_hbdceff_dn7_slot = var_hbdceff_dn7;
        *var_hbdceff_dn8_slot = var_hbdceff_dn8;
        *var_hbdceff_dn9_slot = var_hbdceff_dn9;
        *var_hbdceff_rv_slot = var_hbdceff_rv;
        *var_ids_acc_slot = var_ids_acc;
        *var_ids_acc_dn0_slot = var_ids_acc_dn0;
        *var_ids_acc_dn10_slot = var_ids_acc_dn10;
        *var_ids_acc_dn11_slot = var_ids_acc_dn11;
        *var_ids_acc_dn14_slot = var_ids_acc_dn14;
        *var_ids_acc_dn2_slot = var_ids_acc_dn2;
        *var_ids_acc_dn4_slot = var_ids_acc_dn4;
        *var_ids_acc_dn5_slot = var_ids_acc_dn5;
        *var_ids_acc_dn6_slot = var_ids_acc_dn6;
        *var_ids_acc_dn7_slot = var_ids_acc_dn7;
        *var_ids_acc_dn8_slot = var_ids_acc_dn8;
        *var_ids_acc_dn9_slot = var_ids_acc_dn9;
        *var_ids_acc_rv_slot = var_ids_acc_rv;
        *var_ids_res_slot = var_ids_res;
        *var_ids_res_dn0_slot = var_ids_res_dn0;
        *var_ids_res_dn10_slot = var_ids_res_dn10;
        *var_ids_res_dn11_slot = var_ids_res_dn11;
        *var_ids_res_dn14_slot = var_ids_res_dn14;
        *var_ids_res_dn2_slot = var_ids_res_dn2;
        *var_ids_res_dn4_slot = var_ids_res_dn4;
        *var_ids_res_dn5_slot = var_ids_res_dn5;
        *var_ids_res_dn6_slot = var_ids_res_dn6;
        *var_ids_res_dn7_slot = var_ids_res_dn7;
        *var_ids_res_dn8_slot = var_ids_res_dn8;
        *var_ids_res_dn9_slot = var_ids_res_dn9;
        *var_ids_res_rv_slot = var_ids_res_rv;
        *var_ires_leak_slot = var_ires_leak;
        *var_ires_leak_dn0_slot = var_ires_leak_dn0;
        *var_ires_leak_dn10_slot = var_ires_leak_dn10;
        *var_ires_leak_dn11_slot = var_ires_leak_dn11;
        *var_ires_leak_dn14_slot = var_ires_leak_dn14;
        *var_ires_leak_dn2_slot = var_ires_leak_dn2;
        *var_ires_leak_dn4_slot = var_ires_leak_dn4;
        *var_ires_leak_dn5_slot = var_ires_leak_dn5;
        *var_ires_leak_dn6_slot = var_ires_leak_dn6;
        *var_ires_leak_dn7_slot = var_ires_leak_dn7;
        *var_ires_leak_dn8_slot = var_ires_leak_dn8;
        *var_ires_leak_dn9_slot = var_ires_leak_dn9;
        *var_ires_leak_rv_slot = var_ires_leak_rv;
        *var_js_slot = var_js;
        *var_js2_slot = var_js2;
        *var_js2_dn0_slot = var_js2_dn0;
        *var_js2_dn10_slot = var_js2_dn10;
        *var_js2_dn11_slot = var_js2_dn11;
        *var_js2_dn14_slot = var_js2_dn14;
        *var_js2_dn2_slot = var_js2_dn2;
        *var_js2_dn4_slot = var_js2_dn4;
        *var_js2_dn5_slot = var_js2_dn5;
        *var_js2_dn6_slot = var_js2_dn6;
        *var_js2_dn7_slot = var_js2_dn7;
        *var_js2_dn8_slot = var_js2_dn8;
        *var_js2_dn9_slot = var_js2_dn9;
        *var_js2_rv_slot = var_js2_rv;
        *var_js_dn0_slot = var_js_dn0;
        *var_js_dn10_slot = var_js_dn10;
        *var_js_dn11_slot = var_js_dn11;
        *var_js_dn14_slot = var_js_dn14;
        *var_js_dn2_slot = var_js_dn2;
        *var_js_dn4_slot = var_js_dn4;
        *var_js_dn5_slot = var_js_dn5;
        *var_js_dn6_slot = var_js_dn6;
        *var_js_dn7_slot = var_js_dn7;
        *var_js_dn8_slot = var_js_dn8;
        *var_js_dn9_slot = var_js_dn9;
        *var_js_rv_slot = var_js_rv;
        *var_jssw_slot = var_jssw;
        *var_jssw2_slot = var_jssw2;
        *var_jssw2_dn0_slot = var_jssw2_dn0;
        *var_jssw2_dn10_slot = var_jssw2_dn10;
        *var_jssw2_dn11_slot = var_jssw2_dn11;
        *var_jssw2_dn14_slot = var_jssw2_dn14;
        *var_jssw2_dn2_slot = var_jssw2_dn2;
        *var_jssw2_dn4_slot = var_jssw2_dn4;
        *var_jssw2_dn5_slot = var_jssw2_dn5;
        *var_jssw2_dn6_slot = var_jssw2_dn6;
        *var_jssw2_dn7_slot = var_jssw2_dn7;
        *var_jssw2_dn8_slot = var_jssw2_dn8;
        *var_jssw2_dn9_slot = var_jssw2_dn9;
        *var_jssw2_rv_slot = var_jssw2_rv;
        *var_jssw_dn0_slot = var_jssw_dn0;
        *var_jssw_dn10_slot = var_jssw_dn10;
        *var_jssw_dn11_slot = var_jssw_dn11;
        *var_jssw_dn14_slot = var_jssw_dn14;
        *var_jssw_dn2_slot = var_jssw_dn2;
        *var_jssw_dn4_slot = var_jssw_dn4;
        *var_jssw_dn5_slot = var_jssw_dn5;
        *var_jssw_dn6_slot = var_jssw_dn6;
        *var_jssw_dn7_slot = var_jssw_dn7;
        *var_jssw_dn8_slot = var_jssw_dn8;
        *var_jssw_dn9_slot = var_jssw_dn9;
        *var_jssw_rv_slot = var_jssw_rv;
        *var_lp_s0_max_slot = var_lp_s0_max;
        *var_lp_s0_max_rv_slot = var_lp_s0_max_rv;
        *var_mphn0_slot = var_mphn0;
        *var_mphn0_dn0_slot = var_mphn0_dn0;
        *var_mphn0_dn10_slot = var_mphn0_dn10;
        *var_mphn0_dn11_slot = var_mphn0_dn11;
        *var_mphn0_dn14_slot = var_mphn0_dn14;
        *var_mphn0_dn2_slot = var_mphn0_dn2;
        *var_mphn0_dn4_slot = var_mphn0_dn4;
        *var_mphn0_dn5_slot = var_mphn0_dn5;
        *var_mphn0_dn6_slot = var_mphn0_dn6;
        *var_mphn0_dn7_slot = var_mphn0_dn7;
        *var_mphn0_dn8_slot = var_mphn0_dn8;
        *var_mphn0_dn9_slot = var_mphn0_dn9;
        *var_mphn0_rv_slot = var_mphn0_rv;
        *var_pb2n_slot = var_pb2n;
        *var_pb2n_dn0_slot = var_pb2n_dn0;
        *var_pb2n_dn10_slot = var_pb2n_dn10;
        *var_pb2n_dn11_slot = var_pb2n_dn11;
        *var_pb2n_dn14_slot = var_pb2n_dn14;
        *var_pb2n_dn2_slot = var_pb2n_dn2;
        *var_pb2n_dn4_slot = var_pb2n_dn4;
        *var_pb2n_dn5_slot = var_pb2n_dn5;
        *var_pb2n_dn6_slot = var_pb2n_dn6;
        *var_pb2n_dn7_slot = var_pb2n_dn7;
        *var_pb2n_dn8_slot = var_pb2n_dn8;
        *var_pb2n_dn9_slot = var_pb2n_dn9;
        *var_pb2n_rv_slot = var_pb2n_rv;
        *var_powratio_slot = var_powratio;
        *var_powratio_dn0_slot = var_powratio_dn0;
        *var_powratio_dn10_slot = var_powratio_dn10;
        *var_powratio_dn11_slot = var_powratio_dn11;
        *var_powratio_dn14_slot = var_powratio_dn14;
        *var_powratio_dn2_slot = var_powratio_dn2;
        *var_powratio_dn4_slot = var_powratio_dn4;
        *var_powratio_dn5_slot = var_powratio_dn5;
        *var_powratio_dn6_slot = var_powratio_dn6;
        *var_powratio_dn7_slot = var_powratio_dn7;
        *var_powratio_dn8_slot = var_powratio_dn8;
        *var_powratio_dn9_slot = var_powratio_dn9;
        *var_powratio_rv_slot = var_powratio_rv;
        *var_ptovr_slot = var_ptovr;
        *var_ptovr_dn0_slot = var_ptovr_dn0;
        *var_ptovr_dn10_slot = var_ptovr_dn10;
        *var_ptovr_dn11_slot = var_ptovr_dn11;
        *var_ptovr_dn14_slot = var_ptovr_dn14;
        *var_ptovr_dn2_slot = var_ptovr_dn2;
        *var_ptovr_dn4_slot = var_ptovr_dn4;
        *var_ptovr_dn5_slot = var_ptovr_dn5;
        *var_ptovr_dn6_slot = var_ptovr_dn6;
        *var_ptovr_dn7_slot = var_ptovr_dn7;
        *var_ptovr_dn8_slot = var_ptovr_dn8;
        *var_ptovr_dn9_slot = var_ptovr_dn9;
        *var_ptovr_rv_slot = var_ptovr_rv;
        *var_pzbd_slot = var_pzbd;
        *var_pzbd_dn0_slot = var_pzbd_dn0;
        *var_pzbd_dn10_slot = var_pzbd_dn10;
        *var_pzbd_dn11_slot = var_pzbd_dn11;
        *var_pzbd_dn14_slot = var_pzbd_dn14;
        *var_pzbd_dn2_slot = var_pzbd_dn2;
        *var_pzbd_dn4_slot = var_pzbd_dn4;
        *var_pzbd_dn5_slot = var_pzbd_dn5;
        *var_pzbd_dn6_slot = var_pzbd_dn6;
        *var_pzbd_dn7_slot = var_pzbd_dn7;
        *var_pzbd_dn8_slot = var_pzbd_dn8;
        *var_pzbd_dn9_slot = var_pzbd_dn9;
        *var_pzbd_rv_slot = var_pzbd_rv;
        *var_pzbdsw_slot = var_pzbdsw;
        *var_pzbdsw_dn0_slot = var_pzbdsw_dn0;
        *var_pzbdsw_dn10_slot = var_pzbdsw_dn10;
        *var_pzbdsw_dn11_slot = var_pzbdsw_dn11;
        *var_pzbdsw_dn14_slot = var_pzbdsw_dn14;
        *var_pzbdsw_dn2_slot = var_pzbdsw_dn2;
        *var_pzbdsw_dn4_slot = var_pzbdsw_dn4;
        *var_pzbdsw_dn5_slot = var_pzbdsw_dn5;
        *var_pzbdsw_dn6_slot = var_pzbdsw_dn6;
        *var_pzbdsw_dn7_slot = var_pzbdsw_dn7;
        *var_pzbdsw_dn8_slot = var_pzbdsw_dn8;
        *var_pzbdsw_dn9_slot = var_pzbdsw_dn9;
        *var_pzbdsw_rv_slot = var_pzbdsw_rv;
        *var_pzbdswg_slot = var_pzbdswg;
        *var_pzbdswg_dn0_slot = var_pzbdswg_dn0;
        *var_pzbdswg_dn10_slot = var_pzbdswg_dn10;
        *var_pzbdswg_dn11_slot = var_pzbdswg_dn11;
        *var_pzbdswg_dn14_slot = var_pzbdswg_dn14;
        *var_pzbdswg_dn2_slot = var_pzbdswg_dn2;
        *var_pzbdswg_dn4_slot = var_pzbdswg_dn4;
        *var_pzbdswg_dn5_slot = var_pzbdswg_dn5;
        *var_pzbdswg_dn6_slot = var_pzbdswg_dn6;
        *var_pzbdswg_dn7_slot = var_pzbdswg_dn7;
        *var_pzbdswg_dn8_slot = var_pzbdswg_dn8;
        *var_pzbdswg_dn9_slot = var_pzbdswg_dn9;
        *var_pzbdswg_rv_slot = var_pzbdswg_rv;
        *var_pzbs_slot = var_pzbs;
        *var_pzbs_dn0_slot = var_pzbs_dn0;
        *var_pzbs_dn10_slot = var_pzbs_dn10;
        *var_pzbs_dn11_slot = var_pzbs_dn11;
        *var_pzbs_dn14_slot = var_pzbs_dn14;
        *var_pzbs_dn2_slot = var_pzbs_dn2;
        *var_pzbs_dn4_slot = var_pzbs_dn4;
        *var_pzbs_dn5_slot = var_pzbs_dn5;
        *var_pzbs_dn6_slot = var_pzbs_dn6;
        *var_pzbs_dn7_slot = var_pzbs_dn7;
        *var_pzbs_dn8_slot = var_pzbs_dn8;
        *var_pzbs_dn9_slot = var_pzbs_dn9;
        *var_pzbs_rv_slot = var_pzbs_rv;
        *var_qbd_slot = var_qbd;
        *var_qbd_dn0_slot = var_qbd_dn0;
        *var_qbd_dn10_slot = var_qbd_dn10;
        *var_qbd_dn11_slot = var_qbd_dn11;
        *var_qbd_dn14_slot = var_qbd_dn14;
        *var_qbd_dn16_slot = var_qbd_dn16;
        *var_qbd_dn17_slot = var_qbd_dn17;
        *var_qbd_dn18_slot = var_qbd_dn18;
        *var_qbd_dn2_slot = var_qbd_dn2;
        *var_qbd_dn4_slot = var_qbd_dn4;
        *var_qbd_dn5_slot = var_qbd_dn5;
        *var_qbd_dn6_slot = var_qbd_dn6;
        *var_qbd_dn7_slot = var_qbd_dn7;
        *var_qbd_dn8_slot = var_qbd_dn8;
        *var_qbd_dn9_slot = var_qbd_dn9;
        *var_qbd_rv_slot = var_qbd_rv;
        *var_qbdi_slot = var_qbdi;
        *var_qbdi_dn0_slot = var_qbdi_dn0;
        *var_qbdi_dn10_slot = var_qbdi_dn10;
        *var_qbdi_dn11_slot = var_qbdi_dn11;
        *var_qbdi_dn14_slot = var_qbdi_dn14;
        *var_qbdi_dn2_slot = var_qbdi_dn2;
        *var_qbdi_dn4_slot = var_qbdi_dn4;
        *var_qbdi_dn5_slot = var_qbdi_dn5;
        *var_qbdi_dn6_slot = var_qbdi_dn6;
        *var_qbdi_dn7_slot = var_qbdi_dn7;
        *var_qbdi_dn8_slot = var_qbdi_dn8;
        *var_qbdi_dn9_slot = var_qbdi_dn9;
        *var_qbdi_rv_slot = var_qbdi_rv;
        *var_qbs_slot = var_qbs;
        *var_qbs_dn0_slot = var_qbs_dn0;
        *var_qbs_dn10_slot = var_qbs_dn10;
        *var_qbs_dn11_slot = var_qbs_dn11;
        *var_qbs_dn14_slot = var_qbs_dn14;
        *var_qbs_dn2_slot = var_qbs_dn2;
        *var_qbs_dn4_slot = var_qbs_dn4;
        *var_qbs_dn5_slot = var_qbs_dn5;
        *var_qbs_dn6_slot = var_qbs_dn6;
        *var_qbs_dn7_slot = var_qbs_dn7;
        *var_qbs_dn8_slot = var_qbs_dn8;
        *var_qbs_dn9_slot = var_qbs_dn9;
        *var_qbs_rv_slot = var_qbs_rv;
        *var_qbsi_slot = var_qbsi;
        *var_qbsi_dn0_slot = var_qbsi_dn0;
        *var_qbsi_dn10_slot = var_qbsi_dn10;
        *var_qbsi_dn11_slot = var_qbsi_dn11;
        *var_qbsi_dn14_slot = var_qbsi_dn14;
        *var_qbsi_dn2_slot = var_qbsi_dn2;
        *var_qbsi_dn4_slot = var_qbsi_dn4;
        *var_qbsi_dn5_slot = var_qbsi_dn5;
        *var_qbsi_dn6_slot = var_qbsi_dn6;
        *var_qbsi_dn7_slot = var_qbsi_dn7;
        *var_qbsi_dn8_slot = var_qbsi_dn8;
        *var_qbsi_dn9_slot = var_qbsi_dn9;
        *var_qbsi_rv_slot = var_qbsi_rv;
        *var_qiu_noi_slot = var_qiu_noi;
        *var_qiu_noi_dn0_slot = var_qiu_noi_dn0;
        *var_qiu_noi_dn10_slot = var_qiu_noi_dn10;
        *var_qiu_noi_dn11_slot = var_qiu_noi_dn11;
        *var_qiu_noi_dn14_slot = var_qiu_noi_dn14;
        *var_qiu_noi_dn2_slot = var_qiu_noi_dn2;
        *var_qiu_noi_dn4_slot = var_qiu_noi_dn4;
        *var_qiu_noi_dn5_slot = var_qiu_noi_dn5;
        *var_qiu_noi_dn6_slot = var_qiu_noi_dn6;
        *var_qiu_noi_dn7_slot = var_qiu_noi_dn7;
        *var_qiu_noi_dn8_slot = var_qiu_noi_dn8;
        *var_qiu_noi_dn9_slot = var_qiu_noi_dn9;
        *var_qiu_noi_rv_slot = var_qiu_noi_rv;
        *var_sqrt_eg_slot = var_sqrt_eg;
        *var_sqrt_eg_dn0_slot = var_sqrt_eg_dn0;
        *var_sqrt_eg_dn10_slot = var_sqrt_eg_dn10;
        *var_sqrt_eg_dn11_slot = var_sqrt_eg_dn11;
        *var_sqrt_eg_dn14_slot = var_sqrt_eg_dn14;
        *var_sqrt_eg_dn2_slot = var_sqrt_eg_dn2;
        *var_sqrt_eg_dn4_slot = var_sqrt_eg_dn4;
        *var_sqrt_eg_dn5_slot = var_sqrt_eg_dn5;
        *var_sqrt_eg_dn6_slot = var_sqrt_eg_dn6;
        *var_sqrt_eg_dn7_slot = var_sqrt_eg_dn7;
        *var_sqrt_eg_dn8_slot = var_sqrt_eg_dn8;
        *var_sqrt_eg_dn9_slot = var_sqrt_eg_dn9;
        *var_sqrt_eg_rv_slot = var_sqrt_eg_rv;
        *var_uc_rdrbb_slot = var_uc_rdrbb;
        *var_uc_rdrbb_dn0_slot = var_uc_rdrbb_dn0;
        *var_uc_rdrbb_dn10_slot = var_uc_rdrbb_dn10;
        *var_uc_rdrbb_dn11_slot = var_uc_rdrbb_dn11;
        *var_uc_rdrbb_dn14_slot = var_uc_rdrbb_dn14;
        *var_uc_rdrbb_dn2_slot = var_uc_rdrbb_dn2;
        *var_uc_rdrbb_dn4_slot = var_uc_rdrbb_dn4;
        *var_uc_rdrbb_dn5_slot = var_uc_rdrbb_dn5;
        *var_uc_rdrbb_dn6_slot = var_uc_rdrbb_dn6;
        *var_uc_rdrbb_dn7_slot = var_uc_rdrbb_dn7;
        *var_uc_rdrbb_dn8_slot = var_uc_rdrbb_dn8;
        *var_uc_rdrbb_dn9_slot = var_uc_rdrbb_dn9;
        *var_uc_rdrbb_rv_slot = var_uc_rdrbb_rv;
        *var_uc_rdrbb_s_slot = var_uc_rdrbb_s;
        *var_uc_rdrbb_s_dn0_slot = var_uc_rdrbb_s_dn0;
        *var_uc_rdrbb_s_dn10_slot = var_uc_rdrbb_s_dn10;
        *var_uc_rdrbb_s_dn11_slot = var_uc_rdrbb_s_dn11;
        *var_uc_rdrbb_s_dn14_slot = var_uc_rdrbb_s_dn14;
        *var_uc_rdrbb_s_dn2_slot = var_uc_rdrbb_s_dn2;
        *var_uc_rdrbb_s_dn4_slot = var_uc_rdrbb_s_dn4;
        *var_uc_rdrbb_s_dn5_slot = var_uc_rdrbb_s_dn5;
        *var_uc_rdrbb_s_dn6_slot = var_uc_rdrbb_s_dn6;
        *var_uc_rdrbb_s_dn7_slot = var_uc_rdrbb_s_dn7;
        *var_uc_rdrbb_s_dn8_slot = var_uc_rdrbb_s_dn8;
        *var_uc_rdrbb_s_dn9_slot = var_uc_rdrbb_s_dn9;
        *var_uc_rdrbb_s_rv_slot = var_uc_rdrbb_s_rv;
        *var_uc_subtmp_slot = var_uc_subtmp;
        *var_uc_subtmp_rv_slot = var_uc_subtmp_rv;
        *var_vbipn_slot = var_vbipn;
        *var_vbipn_dn0_slot = var_vbipn_dn0;
        *var_vbipn_dn10_slot = var_vbipn_dn10;
        *var_vbipn_dn11_slot = var_vbipn_dn11;
        *var_vbipn_dn14_slot = var_vbipn_dn14;
        *var_vbipn_dn2_slot = var_vbipn_dn2;
        *var_vbipn_dn4_slot = var_vbipn_dn4;
        *var_vbipn_dn5_slot = var_vbipn_dn5;
        *var_vbipn_dn6_slot = var_vbipn_dn6;
        *var_vbipn_dn7_slot = var_vbipn_dn7;
        *var_vbipn_dn8_slot = var_vbipn_dn8;
        *var_vbipn_dn9_slot = var_vbipn_dn9;
        *var_vbipn_rv_slot = var_vbipn_rv;
        *var_wdpl_slot = var_wdpl;
        *var_wdpl_dn0_slot = var_wdpl_dn0;
        *var_wdpl_dn10_slot = var_wdpl_dn10;
        *var_wdpl_dn11_slot = var_wdpl_dn11;
        *var_wdpl_dn14_slot = var_wdpl_dn14;
        *var_wdpl_dn2_slot = var_wdpl_dn2;
        *var_wdpl_dn4_slot = var_wdpl_dn4;
        *var_wdpl_dn5_slot = var_wdpl_dn5;
        *var_wdpl_dn6_slot = var_wdpl_dn6;
        *var_wdpl_dn7_slot = var_wdpl_dn7;
        *var_wdpl_dn8_slot = var_wdpl_dn8;
        *var_wdpl_dn9_slot = var_wdpl_dn9;
        *var_wdpl_rv_slot = var_wdpl_rv;
        *var_wdplp_slot = var_wdplp;
        *var_wdplp_dn0_slot = var_wdplp_dn0;
        *var_wdplp_dn10_slot = var_wdplp_dn10;
        *var_wdplp_dn11_slot = var_wdplp_dn11;
        *var_wdplp_dn14_slot = var_wdplp_dn14;
        *var_wdplp_dn2_slot = var_wdplp_dn2;
        *var_wdplp_dn4_slot = var_wdplp_dn4;
        *var_wdplp_dn5_slot = var_wdplp_dn5;
        *var_wdplp_dn6_slot = var_wdplp_dn6;
        *var_wdplp_dn7_slot = var_wdplp_dn7;
        *var_wdplp_dn8_slot = var_wdplp_dn8;
        *var_wdplp_dn9_slot = var_wdplp_dn9;
        *var_wdplp_rv_slot = var_wdplp_rv;
    }

    pub(super) fn stamp_reactive_block_10(
        var_end_of_part_1_slot: &mut f64,
        var_end_of_part_1_rv_slot: &mut f64,
        var_exptempd_slot: &mut f64,
        var_exptempd_dn0_slot: &mut f64,
        var_exptempd_dn10_slot: &mut f64,
        var_exptempd_dn11_slot: &mut f64,
        var_exptempd_dn14_slot: &mut f64,
        var_exptempd_dn2_slot: &mut f64,
        var_exptempd_dn4_slot: &mut f64,
        var_exptempd_dn5_slot: &mut f64,
        var_exptempd_dn6_slot: &mut f64,
        var_exptempd_dn7_slot: &mut f64,
        var_exptempd_dn8_slot: &mut f64,
        var_exptempd_dn9_slot: &mut f64,
        var_exptempd_rv_slot: &mut f64,
        var_exptemps_slot: &mut f64,
        var_exptemps_dn0_slot: &mut f64,
        var_exptemps_dn10_slot: &mut f64,
        var_exptemps_dn11_slot: &mut f64,
        var_exptemps_dn14_slot: &mut f64,
        var_exptemps_dn2_slot: &mut f64,
        var_exptemps_dn4_slot: &mut f64,
        var_exptemps_dn5_slot: &mut f64,
        var_exptemps_dn6_slot: &mut f64,
        var_exptemps_dn7_slot: &mut f64,
        var_exptemps_dn8_slot: &mut f64,
        var_exptemps_dn9_slot: &mut f64,
        var_exptemps_rv_slot: &mut f64,
        var_flg_brk1_slot: &mut f64,
        var_flg_brk1_rv_slot: &mut f64,
        var_flg_brk2_slot: &mut f64,
        var_flg_brk2_rv_slot: &mut f64,
        var_idspt0_slot: &mut f64,
        var_idspt0_dn0_slot: &mut f64,
        var_idspt0_dn10_slot: &mut f64,
        var_idspt0_dn11_slot: &mut f64,
        var_idspt0_dn14_slot: &mut f64,
        var_idspt0_dn2_slot: &mut f64,
        var_idspt0_dn4_slot: &mut f64,
        var_idspt0_dn5_slot: &mut f64,
        var_idspt0_dn6_slot: &mut f64,
        var_idspt0_dn7_slot: &mut f64,
        var_idspt0_dn8_slot: &mut f64,
        var_idspt0_dn9_slot: &mut f64,
        var_idspt0_rv_slot: &mut f64,
        var_isbd_slot: &mut f64,
        var_isbd2_btm_slot: &mut f64,
        var_isbd2_btm_dn0_slot: &mut f64,
        var_isbd2_btm_dn10_slot: &mut f64,
        var_isbd2_btm_dn11_slot: &mut f64,
        var_isbd2_btm_dn14_slot: &mut f64,
        var_isbd2_btm_dn2_slot: &mut f64,
        var_isbd2_btm_dn4_slot: &mut f64,
        var_isbd2_btm_dn5_slot: &mut f64,
        var_isbd2_btm_dn6_slot: &mut f64,
        var_isbd2_btm_dn7_slot: &mut f64,
        var_isbd2_btm_dn8_slot: &mut f64,
        var_isbd2_btm_dn9_slot: &mut f64,
        var_isbd2_btm_rv_slot: &mut f64,
        var_isbd2_swg_slot: &mut f64,
        var_isbd2_swg_dn0_slot: &mut f64,
        var_isbd2_swg_dn10_slot: &mut f64,
        var_isbd2_swg_dn11_slot: &mut f64,
        var_isbd2_swg_dn14_slot: &mut f64,
        var_isbd2_swg_dn2_slot: &mut f64,
        var_isbd2_swg_dn4_slot: &mut f64,
        var_isbd2_swg_dn5_slot: &mut f64,
        var_isbd2_swg_dn6_slot: &mut f64,
        var_isbd2_swg_dn7_slot: &mut f64,
        var_isbd2_swg_dn8_slot: &mut f64,
        var_isbd2_swg_dn9_slot: &mut f64,
        var_isbd2_swg_rv_slot: &mut f64,
        var_isbd2_sws_slot: &mut f64,
        var_isbd2_sws_dn0_slot: &mut f64,
        var_isbd2_sws_dn10_slot: &mut f64,
        var_isbd2_sws_dn11_slot: &mut f64,
        var_isbd2_sws_dn14_slot: &mut f64,
        var_isbd2_sws_dn2_slot: &mut f64,
        var_isbd2_sws_dn4_slot: &mut f64,
        var_isbd2_sws_dn5_slot: &mut f64,
        var_isbd2_sws_dn6_slot: &mut f64,
        var_isbd2_sws_dn7_slot: &mut f64,
        var_isbd2_sws_dn8_slot: &mut f64,
        var_isbd2_sws_dn9_slot: &mut f64,
        var_isbd2_sws_rv_slot: &mut f64,
        var_isbd_btm_slot: &mut f64,
        var_isbd_btm_dn0_slot: &mut f64,
        var_isbd_btm_dn10_slot: &mut f64,
        var_isbd_btm_dn11_slot: &mut f64,
        var_isbd_btm_dn14_slot: &mut f64,
        var_isbd_btm_dn2_slot: &mut f64,
        var_isbd_btm_dn4_slot: &mut f64,
        var_isbd_btm_dn5_slot: &mut f64,
        var_isbd_btm_dn6_slot: &mut f64,
        var_isbd_btm_dn7_slot: &mut f64,
        var_isbd_btm_dn8_slot: &mut f64,
        var_isbd_btm_dn9_slot: &mut f64,
        var_isbd_btm_rv_slot: &mut f64,
        var_isbd_dn0_slot: &mut f64,
        var_isbd_dn10_slot: &mut f64,
        var_isbd_dn11_slot: &mut f64,
        var_isbd_dn14_slot: &mut f64,
        var_isbd_dn2_slot: &mut f64,
        var_isbd_dn4_slot: &mut f64,
        var_isbd_dn5_slot: &mut f64,
        var_isbd_dn6_slot: &mut f64,
        var_isbd_dn7_slot: &mut f64,
        var_isbd_dn8_slot: &mut f64,
        var_isbd_dn9_slot: &mut f64,
        var_isbd_rv_slot: &mut f64,
        var_isbd_swg_slot: &mut f64,
        var_isbd_swg_dn0_slot: &mut f64,
        var_isbd_swg_dn10_slot: &mut f64,
        var_isbd_swg_dn11_slot: &mut f64,
        var_isbd_swg_dn14_slot: &mut f64,
        var_isbd_swg_dn2_slot: &mut f64,
        var_isbd_swg_dn4_slot: &mut f64,
        var_isbd_swg_dn5_slot: &mut f64,
        var_isbd_swg_dn6_slot: &mut f64,
        var_isbd_swg_dn7_slot: &mut f64,
        var_isbd_swg_dn8_slot: &mut f64,
        var_isbd_swg_dn9_slot: &mut f64,
        var_isbd_swg_rv_slot: &mut f64,
        var_isbd_sws_slot: &mut f64,
        var_isbd_sws_dn0_slot: &mut f64,
        var_isbd_sws_dn10_slot: &mut f64,
        var_isbd_sws_dn11_slot: &mut f64,
        var_isbd_sws_dn14_slot: &mut f64,
        var_isbd_sws_dn2_slot: &mut f64,
        var_isbd_sws_dn4_slot: &mut f64,
        var_isbd_sws_dn5_slot: &mut f64,
        var_isbd_sws_dn6_slot: &mut f64,
        var_isbd_sws_dn7_slot: &mut f64,
        var_isbd_sws_dn8_slot: &mut f64,
        var_isbd_sws_dn9_slot: &mut f64,
        var_isbd_sws_rv_slot: &mut f64,
        var_isbs_slot: &mut f64,
        var_isbs2_btm_slot: &mut f64,
        var_isbs2_btm_dn0_slot: &mut f64,
        var_isbs2_btm_dn10_slot: &mut f64,
        var_isbs2_btm_dn11_slot: &mut f64,
        var_isbs2_btm_dn14_slot: &mut f64,
        var_isbs2_btm_dn2_slot: &mut f64,
        var_isbs2_btm_dn4_slot: &mut f64,
        var_isbs2_btm_dn5_slot: &mut f64,
        var_isbs2_btm_dn6_slot: &mut f64,
        var_isbs2_btm_dn7_slot: &mut f64,
        var_isbs2_btm_dn8_slot: &mut f64,
        var_isbs2_btm_dn9_slot: &mut f64,
        var_isbs2_btm_rv_slot: &mut f64,
        var_isbs2_swg_slot: &mut f64,
        var_isbs2_swg_dn0_slot: &mut f64,
        var_isbs2_swg_dn10_slot: &mut f64,
        var_isbs2_swg_dn11_slot: &mut f64,
        var_isbs2_swg_dn14_slot: &mut f64,
        var_isbs2_swg_dn2_slot: &mut f64,
        var_isbs2_swg_dn4_slot: &mut f64,
        var_isbs2_swg_dn5_slot: &mut f64,
        var_isbs2_swg_dn6_slot: &mut f64,
        var_isbs2_swg_dn7_slot: &mut f64,
        var_isbs2_swg_dn8_slot: &mut f64,
        var_isbs2_swg_dn9_slot: &mut f64,
        var_isbs2_swg_rv_slot: &mut f64,
        var_isbs2_sws_slot: &mut f64,
        var_isbs2_sws_dn0_slot: &mut f64,
        var_isbs2_sws_dn10_slot: &mut f64,
        var_isbs2_sws_dn11_slot: &mut f64,
        var_isbs2_sws_dn14_slot: &mut f64,
        var_isbs2_sws_dn2_slot: &mut f64,
        var_isbs2_sws_dn4_slot: &mut f64,
        var_isbs2_sws_dn5_slot: &mut f64,
        var_isbs2_sws_dn6_slot: &mut f64,
        var_isbs2_sws_dn7_slot: &mut f64,
        var_isbs2_sws_dn8_slot: &mut f64,
        var_isbs2_sws_dn9_slot: &mut f64,
        var_isbs2_sws_rv_slot: &mut f64,
        var_isbs_btm_slot: &mut f64,
        var_isbs_btm_dn0_slot: &mut f64,
        var_isbs_btm_dn10_slot: &mut f64,
        var_isbs_btm_dn11_slot: &mut f64,
        var_isbs_btm_dn14_slot: &mut f64,
        var_isbs_btm_dn2_slot: &mut f64,
        var_isbs_btm_dn4_slot: &mut f64,
        var_isbs_btm_dn5_slot: &mut f64,
        var_isbs_btm_dn6_slot: &mut f64,
        var_isbs_btm_dn7_slot: &mut f64,
        var_isbs_btm_dn8_slot: &mut f64,
        var_isbs_btm_dn9_slot: &mut f64,
        var_isbs_btm_rv_slot: &mut f64,
        var_isbs_dn0_slot: &mut f64,
        var_isbs_dn10_slot: &mut f64,
        var_isbs_dn11_slot: &mut f64,
        var_isbs_dn14_slot: &mut f64,
        var_isbs_dn2_slot: &mut f64,
        var_isbs_dn4_slot: &mut f64,
        var_isbs_dn5_slot: &mut f64,
        var_isbs_dn6_slot: &mut f64,
        var_isbs_dn7_slot: &mut f64,
        var_isbs_dn8_slot: &mut f64,
        var_isbs_dn9_slot: &mut f64,
        var_isbs_rv_slot: &mut f64,
        var_isbs_swg_slot: &mut f64,
        var_isbs_swg_dn0_slot: &mut f64,
        var_isbs_swg_dn10_slot: &mut f64,
        var_isbs_swg_dn11_slot: &mut f64,
        var_isbs_swg_dn14_slot: &mut f64,
        var_isbs_swg_dn2_slot: &mut f64,
        var_isbs_swg_dn4_slot: &mut f64,
        var_isbs_swg_dn5_slot: &mut f64,
        var_isbs_swg_dn6_slot: &mut f64,
        var_isbs_swg_dn7_slot: &mut f64,
        var_isbs_swg_dn8_slot: &mut f64,
        var_isbs_swg_dn9_slot: &mut f64,
        var_isbs_swg_rv_slot: &mut f64,
        var_isbs_sws_slot: &mut f64,
        var_isbs_sws_dn0_slot: &mut f64,
        var_isbs_sws_dn10_slot: &mut f64,
        var_isbs_sws_dn11_slot: &mut f64,
        var_isbs_sws_dn14_slot: &mut f64,
        var_isbs_sws_dn2_slot: &mut f64,
        var_isbs_sws_dn4_slot: &mut f64,
        var_isbs_sws_dn5_slot: &mut f64,
        var_isbs_sws_dn6_slot: &mut f64,
        var_isbs_sws_dn7_slot: &mut f64,
        var_isbs_sws_dn8_slot: &mut f64,
        var_isbs_sws_dn9_slot: &mut f64,
        var_isbs_sws_rv_slot: &mut f64,
        var_jd_expcd_slot: &mut f64,
        var_jd_expcd_dn0_slot: &mut f64,
        var_jd_expcd_dn10_slot: &mut f64,
        var_jd_expcd_dn11_slot: &mut f64,
        var_jd_expcd_dn14_slot: &mut f64,
        var_jd_expcd_dn2_slot: &mut f64,
        var_jd_expcd_dn4_slot: &mut f64,
        var_jd_expcd_dn5_slot: &mut f64,
        var_jd_expcd_dn6_slot: &mut f64,
        var_jd_expcd_dn7_slot: &mut f64,
        var_jd_expcd_dn8_slot: &mut f64,
        var_jd_expcd_dn9_slot: &mut f64,
        var_jd_expcd_rv_slot: &mut f64,
        var_jd_expcs_slot: &mut f64,
        var_jd_expcs_dn0_slot: &mut f64,
        var_jd_expcs_dn10_slot: &mut f64,
        var_jd_expcs_dn11_slot: &mut f64,
        var_jd_expcs_dn14_slot: &mut f64,
        var_jd_expcs_dn2_slot: &mut f64,
        var_jd_expcs_dn4_slot: &mut f64,
        var_jd_expcs_dn5_slot: &mut f64,
        var_jd_expcs_dn6_slot: &mut f64,
        var_jd_expcs_dn7_slot: &mut f64,
        var_jd_expcs_dn8_slot: &mut f64,
        var_jd_expcs_dn9_slot: &mut f64,
        var_jd_expcs_rv_slot: &mut f64,
        var_jd_nvtm_invd_slot: &mut f64,
        var_jd_nvtm_invd_dn0_slot: &mut f64,
        var_jd_nvtm_invd_dn10_slot: &mut f64,
        var_jd_nvtm_invd_dn11_slot: &mut f64,
        var_jd_nvtm_invd_dn14_slot: &mut f64,
        var_jd_nvtm_invd_dn2_slot: &mut f64,
        var_jd_nvtm_invd_dn4_slot: &mut f64,
        var_jd_nvtm_invd_dn5_slot: &mut f64,
        var_jd_nvtm_invd_dn6_slot: &mut f64,
        var_jd_nvtm_invd_dn7_slot: &mut f64,
        var_jd_nvtm_invd_dn8_slot: &mut f64,
        var_jd_nvtm_invd_dn9_slot: &mut f64,
        var_jd_nvtm_invd_rv_slot: &mut f64,
        var_jd_nvtm_invs_slot: &mut f64,
        var_jd_nvtm_invs_dn0_slot: &mut f64,
        var_jd_nvtm_invs_dn10_slot: &mut f64,
        var_jd_nvtm_invs_dn11_slot: &mut f64,
        var_jd_nvtm_invs_dn14_slot: &mut f64,
        var_jd_nvtm_invs_dn2_slot: &mut f64,
        var_jd_nvtm_invs_dn4_slot: &mut f64,
        var_jd_nvtm_invs_dn5_slot: &mut f64,
        var_jd_nvtm_invs_dn6_slot: &mut f64,
        var_jd_nvtm_invs_dn7_slot: &mut f64,
        var_jd_nvtm_invs_dn8_slot: &mut f64,
        var_jd_nvtm_invs_dn9_slot: &mut f64,
        var_jd_nvtm_invs_rv_slot: &mut f64,
        var_pzbssw_slot: &mut f64,
        var_pzbssw_dn0_slot: &mut f64,
        var_pzbssw_dn10_slot: &mut f64,
        var_pzbssw_dn11_slot: &mut f64,
        var_pzbssw_dn14_slot: &mut f64,
        var_pzbssw_dn2_slot: &mut f64,
        var_pzbssw_dn4_slot: &mut f64,
        var_pzbssw_dn5_slot: &mut f64,
        var_pzbssw_dn6_slot: &mut f64,
        var_pzbssw_dn7_slot: &mut f64,
        var_pzbssw_dn8_slot: &mut f64,
        var_pzbssw_dn9_slot: &mut f64,
        var_pzbssw_rv_slot: &mut f64,
        var_pzbsswg_slot: &mut f64,
        var_pzbsswg_dn0_slot: &mut f64,
        var_pzbsswg_dn10_slot: &mut f64,
        var_pzbsswg_dn11_slot: &mut f64,
        var_pzbsswg_dn14_slot: &mut f64,
        var_pzbsswg_dn2_slot: &mut f64,
        var_pzbsswg_dn4_slot: &mut f64,
        var_pzbsswg_dn5_slot: &mut f64,
        var_pzbsswg_dn6_slot: &mut f64,
        var_pzbsswg_dn7_slot: &mut f64,
        var_pzbsswg_dn8_slot: &mut f64,
        var_pzbsswg_dn9_slot: &mut f64,
        var_pzbsswg_rv_slot: &mut f64,
        var_qbd_qs_slot: &mut f64,
        var_qbd_qs_dn0_slot: &mut f64,
        var_qbd_qs_dn10_slot: &mut f64,
        var_qbd_qs_dn11_slot: &mut f64,
        var_qbd_qs_dn14_slot: &mut f64,
        var_qbd_qs_dn2_slot: &mut f64,
        var_qbd_qs_dn4_slot: &mut f64,
        var_qbd_qs_dn5_slot: &mut f64,
        var_qbd_qs_dn6_slot: &mut f64,
        var_qbd_qs_dn7_slot: &mut f64,
        var_qbd_qs_dn8_slot: &mut f64,
        var_qbd_qs_dn9_slot: &mut f64,
        var_qbd_qs_rv_slot: &mut f64,
        var_qbdld_add_slot: &mut f64,
        var_qbdld_add_dn0_slot: &mut f64,
        var_qbdld_add_dn10_slot: &mut f64,
        var_qbdld_add_dn11_slot: &mut f64,
        var_qbdld_add_dn14_slot: &mut f64,
        var_qbdld_add_dn2_slot: &mut f64,
        var_qbdld_add_dn4_slot: &mut f64,
        var_qbdld_add_dn5_slot: &mut f64,
        var_qbdld_add_dn6_slot: &mut f64,
        var_qbdld_add_dn7_slot: &mut f64,
        var_qbdld_add_dn8_slot: &mut f64,
        var_qbdld_add_dn9_slot: &mut f64,
        var_qbdld_add_rv_slot: &mut f64,
        var_qbsld_add_slot: &mut f64,
        var_qbsld_add_dn0_slot: &mut f64,
        var_qbsld_add_dn10_slot: &mut f64,
        var_qbsld_add_dn11_slot: &mut f64,
        var_qbsld_add_dn14_slot: &mut f64,
        var_qbsld_add_dn2_slot: &mut f64,
        var_qbsld_add_dn4_slot: &mut f64,
        var_qbsld_add_dn5_slot: &mut f64,
        var_qbsld_add_dn6_slot: &mut f64,
        var_qbsld_add_dn7_slot: &mut f64,
        var_qbsld_add_dn8_slot: &mut f64,
        var_qbsld_add_dn9_slot: &mut f64,
        var_qbsld_add_rv_slot: &mut f64,
        var_qovd_add_slot: &mut f64,
        var_qovd_add_dn0_slot: &mut f64,
        var_qovd_add_dn10_slot: &mut f64,
        var_qovd_add_dn11_slot: &mut f64,
        var_qovd_add_dn14_slot: &mut f64,
        var_qovd_add_dn2_slot: &mut f64,
        var_qovd_add_dn4_slot: &mut f64,
        var_qovd_add_dn5_slot: &mut f64,
        var_qovd_add_dn6_slot: &mut f64,
        var_qovd_add_dn7_slot: &mut f64,
        var_qovd_add_dn8_slot: &mut f64,
        var_qovd_add_dn9_slot: &mut f64,
        var_qovd_add_rv_slot: &mut f64,
        var_qovs_add_slot: &mut f64,
        var_qovs_add_dn0_slot: &mut f64,
        var_qovs_add_dn10_slot: &mut f64,
        var_qovs_add_dn11_slot: &mut f64,
        var_qovs_add_dn14_slot: &mut f64,
        var_qovs_add_dn2_slot: &mut f64,
        var_qovs_add_dn4_slot: &mut f64,
        var_qovs_add_dn5_slot: &mut f64,
        var_qovs_add_dn6_slot: &mut f64,
        var_qovs_add_dn7_slot: &mut f64,
        var_qovs_add_dn8_slot: &mut f64,
        var_qovs_add_dn9_slot: &mut f64,
        var_qovs_add_rv_slot: &mut f64,
        var_sarg_slot: &mut f64,
        var_sarg_dn0_slot: &mut f64,
        var_sarg_dn10_slot: &mut f64,
        var_sarg_dn11_slot: &mut f64,
        var_sarg_dn14_slot: &mut f64,
        var_sarg_dn2_slot: &mut f64,
        var_sarg_dn4_slot: &mut f64,
        var_sarg_dn5_slot: &mut f64,
        var_sarg_dn6_slot: &mut f64,
        var_sarg_dn7_slot: &mut f64,
        var_sarg_dn8_slot: &mut f64,
        var_sarg_dn9_slot: &mut f64,
        var_sarg_rv_slot: &mut f64,
        var_start_of_loopl_slot: &mut f64,
        var_start_of_loopl_rv_slot: &mut f64,
        var_start_of_mobility_slot: &mut f64,
        var_start_of_mobility_rv_slot: &mut f64,
        var_vbd_jct_slot: &mut f64,
        var_vbd_jct_dn0_slot: &mut f64,
        var_vbd_jct_dn10_slot: &mut f64,
        var_vbd_jct_rv_slot: &mut f64,
        var_vbdi_jct_slot: &mut f64,
        var_vbdi_jct_dn6_slot: &mut f64,
        var_vbdi_jct_dn9_slot: &mut f64,
        var_vbdi_jct_rv_slot: &mut f64,
        var_vbdt_slot: &mut f64,
        var_vbdt_dn0_slot: &mut f64,
        var_vbdt_dn10_slot: &mut f64,
        var_vbdt_dn11_slot: &mut f64,
        var_vbdt_dn14_slot: &mut f64,
        var_vbdt_dn2_slot: &mut f64,
        var_vbdt_dn4_slot: &mut f64,
        var_vbdt_dn5_slot: &mut f64,
        var_vbdt_dn6_slot: &mut f64,
        var_vbdt_dn7_slot: &mut f64,
        var_vbdt_dn8_slot: &mut f64,
        var_vbdt_dn9_slot: &mut f64,
        var_vbdt_rv_slot: &mut f64,
        var_vbpdp_slot: &mut f64,
        var_vbpdp_dn6_slot: &mut f64,
        var_vbpdp_dn9_slot: &mut f64,
        var_vbpdp_rv_slot: &mut f64,
        var_vbpsp_slot: &mut f64,
        var_vbpsp_dn8_slot: &mut f64,
        var_vbpsp_dn9_slot: &mut f64,
        var_vbpsp_rv_slot: &mut f64,
        var_vbs_jct_slot: &mut f64,
        var_vbs_jct_dn11_slot: &mut f64,
        var_vbs_jct_dn2_slot: &mut f64,
        var_vbs_jct_rv_slot: &mut f64,
        var_vbsi_jct_slot: &mut f64,
        var_vbsi_jct_dn8_slot: &mut f64,
        var_vbsi_jct_dn9_slot: &mut f64,
        var_vbsi_jct_rv_slot: &mut f64,
        var_vbst_slot: &mut f64,
        var_vbst_dn0_slot: &mut f64,
        var_vbst_dn10_slot: &mut f64,
        var_vbst_dn11_slot: &mut f64,
        var_vbst_dn14_slot: &mut f64,
        var_vbst_dn2_slot: &mut f64,
        var_vbst_dn4_slot: &mut f64,
        var_vbst_dn5_slot: &mut f64,
        var_vbst_dn6_slot: &mut f64,
        var_vbst_dn7_slot: &mut f64,
        var_vbst_dn8_slot: &mut f64,
        var_vbst_dn9_slot: &mut f64,
        var_vbst_rv_slot: &mut f64,
        var_vdbd_slot: &mut f64,
        var_vdbd_dn0_slot: &mut f64,
        var_vdbd_dn10_slot: &mut f64,
        var_vdbd_rv_slot: &mut f64,
        var_vsbs_slot: &mut f64,
        var_vsbs_dn11_slot: &mut f64,
        var_vsbs_dn2_slot: &mut f64,
        var_vsbs_rv_slot: &mut f64,
        var_wjuncld_slot: &mut f64,
        var_wjuncld_dn0_slot: &mut f64,
        var_wjuncld_dn10_slot: &mut f64,
        var_wjuncld_dn11_slot: &mut f64,
        var_wjuncld_dn14_slot: &mut f64,
        var_wjuncld_dn2_slot: &mut f64,
        var_wjuncld_dn4_slot: &mut f64,
        var_wjuncld_dn5_slot: &mut f64,
        var_wjuncld_dn6_slot: &mut f64,
        var_wjuncld_dn7_slot: &mut f64,
        var_wjuncld_dn8_slot: &mut f64,
        var_wjuncld_dn9_slot: &mut f64,
        var_wjuncld_rv_slot: &mut f64,
    ) {
        let mut var_end_of_part_1: f64 = *var_end_of_part_1_slot;
        let mut var_end_of_part_1_rv: f64 = *var_end_of_part_1_rv_slot;
        let mut var_exptempd: f64 = *var_exptempd_slot;
        let mut var_exptempd_dn0: f64 = *var_exptempd_dn0_slot;
        let mut var_exptempd_dn10: f64 = *var_exptempd_dn10_slot;
        let mut var_exptempd_dn11: f64 = *var_exptempd_dn11_slot;
        let mut var_exptempd_dn14: f64 = *var_exptempd_dn14_slot;
        let mut var_exptempd_dn2: f64 = *var_exptempd_dn2_slot;
        let mut var_exptempd_dn4: f64 = *var_exptempd_dn4_slot;
        let mut var_exptempd_dn5: f64 = *var_exptempd_dn5_slot;
        let mut var_exptempd_dn6: f64 = *var_exptempd_dn6_slot;
        let mut var_exptempd_dn7: f64 = *var_exptempd_dn7_slot;
        let mut var_exptempd_dn8: f64 = *var_exptempd_dn8_slot;
        let mut var_exptempd_dn9: f64 = *var_exptempd_dn9_slot;
        let mut var_exptempd_rv: f64 = *var_exptempd_rv_slot;
        let mut var_exptemps: f64 = *var_exptemps_slot;
        let mut var_exptemps_dn0: f64 = *var_exptemps_dn0_slot;
        let mut var_exptemps_dn10: f64 = *var_exptemps_dn10_slot;
        let mut var_exptemps_dn11: f64 = *var_exptemps_dn11_slot;
        let mut var_exptemps_dn14: f64 = *var_exptemps_dn14_slot;
        let mut var_exptemps_dn2: f64 = *var_exptemps_dn2_slot;
        let mut var_exptemps_dn4: f64 = *var_exptemps_dn4_slot;
        let mut var_exptemps_dn5: f64 = *var_exptemps_dn5_slot;
        let mut var_exptemps_dn6: f64 = *var_exptemps_dn6_slot;
        let mut var_exptemps_dn7: f64 = *var_exptemps_dn7_slot;
        let mut var_exptemps_dn8: f64 = *var_exptemps_dn8_slot;
        let mut var_exptemps_dn9: f64 = *var_exptemps_dn9_slot;
        let mut var_exptemps_rv: f64 = *var_exptemps_rv_slot;
        let mut var_flg_brk1: f64 = *var_flg_brk1_slot;
        let mut var_flg_brk1_rv: f64 = *var_flg_brk1_rv_slot;
        let mut var_flg_brk2: f64 = *var_flg_brk2_slot;
        let mut var_flg_brk2_rv: f64 = *var_flg_brk2_rv_slot;
        let mut var_idspt0: f64 = *var_idspt0_slot;
        let mut var_idspt0_dn0: f64 = *var_idspt0_dn0_slot;
        let mut var_idspt0_dn10: f64 = *var_idspt0_dn10_slot;
        let mut var_idspt0_dn11: f64 = *var_idspt0_dn11_slot;
        let mut var_idspt0_dn14: f64 = *var_idspt0_dn14_slot;
        let mut var_idspt0_dn2: f64 = *var_idspt0_dn2_slot;
        let mut var_idspt0_dn4: f64 = *var_idspt0_dn4_slot;
        let mut var_idspt0_dn5: f64 = *var_idspt0_dn5_slot;
        let mut var_idspt0_dn6: f64 = *var_idspt0_dn6_slot;
        let mut var_idspt0_dn7: f64 = *var_idspt0_dn7_slot;
        let mut var_idspt0_dn8: f64 = *var_idspt0_dn8_slot;
        let mut var_idspt0_dn9: f64 = *var_idspt0_dn9_slot;
        let mut var_idspt0_rv: f64 = *var_idspt0_rv_slot;
        let mut var_isbd: f64 = *var_isbd_slot;
        let mut var_isbd2_btm: f64 = *var_isbd2_btm_slot;
        let mut var_isbd2_btm_dn0: f64 = *var_isbd2_btm_dn0_slot;
        let mut var_isbd2_btm_dn10: f64 = *var_isbd2_btm_dn10_slot;
        let mut var_isbd2_btm_dn11: f64 = *var_isbd2_btm_dn11_slot;
        let mut var_isbd2_btm_dn14: f64 = *var_isbd2_btm_dn14_slot;
        let mut var_isbd2_btm_dn2: f64 = *var_isbd2_btm_dn2_slot;
        let mut var_isbd2_btm_dn4: f64 = *var_isbd2_btm_dn4_slot;
        let mut var_isbd2_btm_dn5: f64 = *var_isbd2_btm_dn5_slot;
        let mut var_isbd2_btm_dn6: f64 = *var_isbd2_btm_dn6_slot;
        let mut var_isbd2_btm_dn7: f64 = *var_isbd2_btm_dn7_slot;
        let mut var_isbd2_btm_dn8: f64 = *var_isbd2_btm_dn8_slot;
        let mut var_isbd2_btm_dn9: f64 = *var_isbd2_btm_dn9_slot;
        let mut var_isbd2_btm_rv: f64 = *var_isbd2_btm_rv_slot;
        let mut var_isbd2_swg: f64 = *var_isbd2_swg_slot;
        let mut var_isbd2_swg_dn0: f64 = *var_isbd2_swg_dn0_slot;
        let mut var_isbd2_swg_dn10: f64 = *var_isbd2_swg_dn10_slot;
        let mut var_isbd2_swg_dn11: f64 = *var_isbd2_swg_dn11_slot;
        let mut var_isbd2_swg_dn14: f64 = *var_isbd2_swg_dn14_slot;
        let mut var_isbd2_swg_dn2: f64 = *var_isbd2_swg_dn2_slot;
        let mut var_isbd2_swg_dn4: f64 = *var_isbd2_swg_dn4_slot;
        let mut var_isbd2_swg_dn5: f64 = *var_isbd2_swg_dn5_slot;
        let mut var_isbd2_swg_dn6: f64 = *var_isbd2_swg_dn6_slot;
        let mut var_isbd2_swg_dn7: f64 = *var_isbd2_swg_dn7_slot;
        let mut var_isbd2_swg_dn8: f64 = *var_isbd2_swg_dn8_slot;
        let mut var_isbd2_swg_dn9: f64 = *var_isbd2_swg_dn9_slot;
        let mut var_isbd2_swg_rv: f64 = *var_isbd2_swg_rv_slot;
        let mut var_isbd2_sws: f64 = *var_isbd2_sws_slot;
        let mut var_isbd2_sws_dn0: f64 = *var_isbd2_sws_dn0_slot;
        let mut var_isbd2_sws_dn10: f64 = *var_isbd2_sws_dn10_slot;
        let mut var_isbd2_sws_dn11: f64 = *var_isbd2_sws_dn11_slot;
        let mut var_isbd2_sws_dn14: f64 = *var_isbd2_sws_dn14_slot;
        let mut var_isbd2_sws_dn2: f64 = *var_isbd2_sws_dn2_slot;
        let mut var_isbd2_sws_dn4: f64 = *var_isbd2_sws_dn4_slot;
        let mut var_isbd2_sws_dn5: f64 = *var_isbd2_sws_dn5_slot;
        let mut var_isbd2_sws_dn6: f64 = *var_isbd2_sws_dn6_slot;
        let mut var_isbd2_sws_dn7: f64 = *var_isbd2_sws_dn7_slot;
        let mut var_isbd2_sws_dn8: f64 = *var_isbd2_sws_dn8_slot;
        let mut var_isbd2_sws_dn9: f64 = *var_isbd2_sws_dn9_slot;
        let mut var_isbd2_sws_rv: f64 = *var_isbd2_sws_rv_slot;
        let mut var_isbd_btm: f64 = *var_isbd_btm_slot;
        let mut var_isbd_btm_dn0: f64 = *var_isbd_btm_dn0_slot;
        let mut var_isbd_btm_dn10: f64 = *var_isbd_btm_dn10_slot;
        let mut var_isbd_btm_dn11: f64 = *var_isbd_btm_dn11_slot;
        let mut var_isbd_btm_dn14: f64 = *var_isbd_btm_dn14_slot;
        let mut var_isbd_btm_dn2: f64 = *var_isbd_btm_dn2_slot;
        let mut var_isbd_btm_dn4: f64 = *var_isbd_btm_dn4_slot;
        let mut var_isbd_btm_dn5: f64 = *var_isbd_btm_dn5_slot;
        let mut var_isbd_btm_dn6: f64 = *var_isbd_btm_dn6_slot;
        let mut var_isbd_btm_dn7: f64 = *var_isbd_btm_dn7_slot;
        let mut var_isbd_btm_dn8: f64 = *var_isbd_btm_dn8_slot;
        let mut var_isbd_btm_dn9: f64 = *var_isbd_btm_dn9_slot;
        let mut var_isbd_btm_rv: f64 = *var_isbd_btm_rv_slot;
        let mut var_isbd_dn0: f64 = *var_isbd_dn0_slot;
        let mut var_isbd_dn10: f64 = *var_isbd_dn10_slot;
        let mut var_isbd_dn11: f64 = *var_isbd_dn11_slot;
        let mut var_isbd_dn14: f64 = *var_isbd_dn14_slot;
        let mut var_isbd_dn2: f64 = *var_isbd_dn2_slot;
        let mut var_isbd_dn4: f64 = *var_isbd_dn4_slot;
        let mut var_isbd_dn5: f64 = *var_isbd_dn5_slot;
        let mut var_isbd_dn6: f64 = *var_isbd_dn6_slot;
        let mut var_isbd_dn7: f64 = *var_isbd_dn7_slot;
        let mut var_isbd_dn8: f64 = *var_isbd_dn8_slot;
        let mut var_isbd_dn9: f64 = *var_isbd_dn9_slot;
        let mut var_isbd_rv: f64 = *var_isbd_rv_slot;
        let mut var_isbd_swg: f64 = *var_isbd_swg_slot;
        let mut var_isbd_swg_dn0: f64 = *var_isbd_swg_dn0_slot;
        let mut var_isbd_swg_dn10: f64 = *var_isbd_swg_dn10_slot;
        let mut var_isbd_swg_dn11: f64 = *var_isbd_swg_dn11_slot;
        let mut var_isbd_swg_dn14: f64 = *var_isbd_swg_dn14_slot;
        let mut var_isbd_swg_dn2: f64 = *var_isbd_swg_dn2_slot;
        let mut var_isbd_swg_dn4: f64 = *var_isbd_swg_dn4_slot;
        let mut var_isbd_swg_dn5: f64 = *var_isbd_swg_dn5_slot;
        let mut var_isbd_swg_dn6: f64 = *var_isbd_swg_dn6_slot;
        let mut var_isbd_swg_dn7: f64 = *var_isbd_swg_dn7_slot;
        let mut var_isbd_swg_dn8: f64 = *var_isbd_swg_dn8_slot;
        let mut var_isbd_swg_dn9: f64 = *var_isbd_swg_dn9_slot;
        let mut var_isbd_swg_rv: f64 = *var_isbd_swg_rv_slot;
        let mut var_isbd_sws: f64 = *var_isbd_sws_slot;
        let mut var_isbd_sws_dn0: f64 = *var_isbd_sws_dn0_slot;
        let mut var_isbd_sws_dn10: f64 = *var_isbd_sws_dn10_slot;
        let mut var_isbd_sws_dn11: f64 = *var_isbd_sws_dn11_slot;
        let mut var_isbd_sws_dn14: f64 = *var_isbd_sws_dn14_slot;
        let mut var_isbd_sws_dn2: f64 = *var_isbd_sws_dn2_slot;
        let mut var_isbd_sws_dn4: f64 = *var_isbd_sws_dn4_slot;
        let mut var_isbd_sws_dn5: f64 = *var_isbd_sws_dn5_slot;
        let mut var_isbd_sws_dn6: f64 = *var_isbd_sws_dn6_slot;
        let mut var_isbd_sws_dn7: f64 = *var_isbd_sws_dn7_slot;
        let mut var_isbd_sws_dn8: f64 = *var_isbd_sws_dn8_slot;
        let mut var_isbd_sws_dn9: f64 = *var_isbd_sws_dn9_slot;
        let mut var_isbd_sws_rv: f64 = *var_isbd_sws_rv_slot;
        let mut var_isbs: f64 = *var_isbs_slot;
        let mut var_isbs2_btm: f64 = *var_isbs2_btm_slot;
        let mut var_isbs2_btm_dn0: f64 = *var_isbs2_btm_dn0_slot;
        let mut var_isbs2_btm_dn10: f64 = *var_isbs2_btm_dn10_slot;
        let mut var_isbs2_btm_dn11: f64 = *var_isbs2_btm_dn11_slot;
        let mut var_isbs2_btm_dn14: f64 = *var_isbs2_btm_dn14_slot;
        let mut var_isbs2_btm_dn2: f64 = *var_isbs2_btm_dn2_slot;
        let mut var_isbs2_btm_dn4: f64 = *var_isbs2_btm_dn4_slot;
        let mut var_isbs2_btm_dn5: f64 = *var_isbs2_btm_dn5_slot;
        let mut var_isbs2_btm_dn6: f64 = *var_isbs2_btm_dn6_slot;
        let mut var_isbs2_btm_dn7: f64 = *var_isbs2_btm_dn7_slot;
        let mut var_isbs2_btm_dn8: f64 = *var_isbs2_btm_dn8_slot;
        let mut var_isbs2_btm_dn9: f64 = *var_isbs2_btm_dn9_slot;
        let mut var_isbs2_btm_rv: f64 = *var_isbs2_btm_rv_slot;
        let mut var_isbs2_swg: f64 = *var_isbs2_swg_slot;
        let mut var_isbs2_swg_dn0: f64 = *var_isbs2_swg_dn0_slot;
        let mut var_isbs2_swg_dn10: f64 = *var_isbs2_swg_dn10_slot;
        let mut var_isbs2_swg_dn11: f64 = *var_isbs2_swg_dn11_slot;
        let mut var_isbs2_swg_dn14: f64 = *var_isbs2_swg_dn14_slot;
        let mut var_isbs2_swg_dn2: f64 = *var_isbs2_swg_dn2_slot;
        let mut var_isbs2_swg_dn4: f64 = *var_isbs2_swg_dn4_slot;
        let mut var_isbs2_swg_dn5: f64 = *var_isbs2_swg_dn5_slot;
        let mut var_isbs2_swg_dn6: f64 = *var_isbs2_swg_dn6_slot;
        let mut var_isbs2_swg_dn7: f64 = *var_isbs2_swg_dn7_slot;
        let mut var_isbs2_swg_dn8: f64 = *var_isbs2_swg_dn8_slot;
        let mut var_isbs2_swg_dn9: f64 = *var_isbs2_swg_dn9_slot;
        let mut var_isbs2_swg_rv: f64 = *var_isbs2_swg_rv_slot;
        let mut var_isbs2_sws: f64 = *var_isbs2_sws_slot;
        let mut var_isbs2_sws_dn0: f64 = *var_isbs2_sws_dn0_slot;
        let mut var_isbs2_sws_dn10: f64 = *var_isbs2_sws_dn10_slot;
        let mut var_isbs2_sws_dn11: f64 = *var_isbs2_sws_dn11_slot;
        let mut var_isbs2_sws_dn14: f64 = *var_isbs2_sws_dn14_slot;
        let mut var_isbs2_sws_dn2: f64 = *var_isbs2_sws_dn2_slot;
        let mut var_isbs2_sws_dn4: f64 = *var_isbs2_sws_dn4_slot;
        let mut var_isbs2_sws_dn5: f64 = *var_isbs2_sws_dn5_slot;
        let mut var_isbs2_sws_dn6: f64 = *var_isbs2_sws_dn6_slot;
        let mut var_isbs2_sws_dn7: f64 = *var_isbs2_sws_dn7_slot;
        let mut var_isbs2_sws_dn8: f64 = *var_isbs2_sws_dn8_slot;
        let mut var_isbs2_sws_dn9: f64 = *var_isbs2_sws_dn9_slot;
        let mut var_isbs2_sws_rv: f64 = *var_isbs2_sws_rv_slot;
        let mut var_isbs_btm: f64 = *var_isbs_btm_slot;
        let mut var_isbs_btm_dn0: f64 = *var_isbs_btm_dn0_slot;
        let mut var_isbs_btm_dn10: f64 = *var_isbs_btm_dn10_slot;
        let mut var_isbs_btm_dn11: f64 = *var_isbs_btm_dn11_slot;
        let mut var_isbs_btm_dn14: f64 = *var_isbs_btm_dn14_slot;
        let mut var_isbs_btm_dn2: f64 = *var_isbs_btm_dn2_slot;
        let mut var_isbs_btm_dn4: f64 = *var_isbs_btm_dn4_slot;
        let mut var_isbs_btm_dn5: f64 = *var_isbs_btm_dn5_slot;
        let mut var_isbs_btm_dn6: f64 = *var_isbs_btm_dn6_slot;
        let mut var_isbs_btm_dn7: f64 = *var_isbs_btm_dn7_slot;
        let mut var_isbs_btm_dn8: f64 = *var_isbs_btm_dn8_slot;
        let mut var_isbs_btm_dn9: f64 = *var_isbs_btm_dn9_slot;
        let mut var_isbs_btm_rv: f64 = *var_isbs_btm_rv_slot;
        let mut var_isbs_dn0: f64 = *var_isbs_dn0_slot;
        let mut var_isbs_dn10: f64 = *var_isbs_dn10_slot;
        let mut var_isbs_dn11: f64 = *var_isbs_dn11_slot;
        let mut var_isbs_dn14: f64 = *var_isbs_dn14_slot;
        let mut var_isbs_dn2: f64 = *var_isbs_dn2_slot;
        let mut var_isbs_dn4: f64 = *var_isbs_dn4_slot;
        let mut var_isbs_dn5: f64 = *var_isbs_dn5_slot;
        let mut var_isbs_dn6: f64 = *var_isbs_dn6_slot;
        let mut var_isbs_dn7: f64 = *var_isbs_dn7_slot;
        let mut var_isbs_dn8: f64 = *var_isbs_dn8_slot;
        let mut var_isbs_dn9: f64 = *var_isbs_dn9_slot;
        let mut var_isbs_rv: f64 = *var_isbs_rv_slot;
        let mut var_isbs_swg: f64 = *var_isbs_swg_slot;
        let mut var_isbs_swg_dn0: f64 = *var_isbs_swg_dn0_slot;
        let mut var_isbs_swg_dn10: f64 = *var_isbs_swg_dn10_slot;
        let mut var_isbs_swg_dn11: f64 = *var_isbs_swg_dn11_slot;
        let mut var_isbs_swg_dn14: f64 = *var_isbs_swg_dn14_slot;
        let mut var_isbs_swg_dn2: f64 = *var_isbs_swg_dn2_slot;
        let mut var_isbs_swg_dn4: f64 = *var_isbs_swg_dn4_slot;
        let mut var_isbs_swg_dn5: f64 = *var_isbs_swg_dn5_slot;
        let mut var_isbs_swg_dn6: f64 = *var_isbs_swg_dn6_slot;
        let mut var_isbs_swg_dn7: f64 = *var_isbs_swg_dn7_slot;
        let mut var_isbs_swg_dn8: f64 = *var_isbs_swg_dn8_slot;
        let mut var_isbs_swg_dn9: f64 = *var_isbs_swg_dn9_slot;
        let mut var_isbs_swg_rv: f64 = *var_isbs_swg_rv_slot;
        let mut var_isbs_sws: f64 = *var_isbs_sws_slot;
        let mut var_isbs_sws_dn0: f64 = *var_isbs_sws_dn0_slot;
        let mut var_isbs_sws_dn10: f64 = *var_isbs_sws_dn10_slot;
        let mut var_isbs_sws_dn11: f64 = *var_isbs_sws_dn11_slot;
        let mut var_isbs_sws_dn14: f64 = *var_isbs_sws_dn14_slot;
        let mut var_isbs_sws_dn2: f64 = *var_isbs_sws_dn2_slot;
        let mut var_isbs_sws_dn4: f64 = *var_isbs_sws_dn4_slot;
        let mut var_isbs_sws_dn5: f64 = *var_isbs_sws_dn5_slot;
        let mut var_isbs_sws_dn6: f64 = *var_isbs_sws_dn6_slot;
        let mut var_isbs_sws_dn7: f64 = *var_isbs_sws_dn7_slot;
        let mut var_isbs_sws_dn8: f64 = *var_isbs_sws_dn8_slot;
        let mut var_isbs_sws_dn9: f64 = *var_isbs_sws_dn9_slot;
        let mut var_isbs_sws_rv: f64 = *var_isbs_sws_rv_slot;
        let mut var_jd_expcd: f64 = *var_jd_expcd_slot;
        let mut var_jd_expcd_dn0: f64 = *var_jd_expcd_dn0_slot;
        let mut var_jd_expcd_dn10: f64 = *var_jd_expcd_dn10_slot;
        let mut var_jd_expcd_dn11: f64 = *var_jd_expcd_dn11_slot;
        let mut var_jd_expcd_dn14: f64 = *var_jd_expcd_dn14_slot;
        let mut var_jd_expcd_dn2: f64 = *var_jd_expcd_dn2_slot;
        let mut var_jd_expcd_dn4: f64 = *var_jd_expcd_dn4_slot;
        let mut var_jd_expcd_dn5: f64 = *var_jd_expcd_dn5_slot;
        let mut var_jd_expcd_dn6: f64 = *var_jd_expcd_dn6_slot;
        let mut var_jd_expcd_dn7: f64 = *var_jd_expcd_dn7_slot;
        let mut var_jd_expcd_dn8: f64 = *var_jd_expcd_dn8_slot;
        let mut var_jd_expcd_dn9: f64 = *var_jd_expcd_dn9_slot;
        let mut var_jd_expcd_rv: f64 = *var_jd_expcd_rv_slot;
        let mut var_jd_expcs: f64 = *var_jd_expcs_slot;
        let mut var_jd_expcs_dn0: f64 = *var_jd_expcs_dn0_slot;
        let mut var_jd_expcs_dn10: f64 = *var_jd_expcs_dn10_slot;
        let mut var_jd_expcs_dn11: f64 = *var_jd_expcs_dn11_slot;
        let mut var_jd_expcs_dn14: f64 = *var_jd_expcs_dn14_slot;
        let mut var_jd_expcs_dn2: f64 = *var_jd_expcs_dn2_slot;
        let mut var_jd_expcs_dn4: f64 = *var_jd_expcs_dn4_slot;
        let mut var_jd_expcs_dn5: f64 = *var_jd_expcs_dn5_slot;
        let mut var_jd_expcs_dn6: f64 = *var_jd_expcs_dn6_slot;
        let mut var_jd_expcs_dn7: f64 = *var_jd_expcs_dn7_slot;
        let mut var_jd_expcs_dn8: f64 = *var_jd_expcs_dn8_slot;
        let mut var_jd_expcs_dn9: f64 = *var_jd_expcs_dn9_slot;
        let mut var_jd_expcs_rv: f64 = *var_jd_expcs_rv_slot;
        let mut var_jd_nvtm_invd: f64 = *var_jd_nvtm_invd_slot;
        let mut var_jd_nvtm_invd_dn0: f64 = *var_jd_nvtm_invd_dn0_slot;
        let mut var_jd_nvtm_invd_dn10: f64 = *var_jd_nvtm_invd_dn10_slot;
        let mut var_jd_nvtm_invd_dn11: f64 = *var_jd_nvtm_invd_dn11_slot;
        let mut var_jd_nvtm_invd_dn14: f64 = *var_jd_nvtm_invd_dn14_slot;
        let mut var_jd_nvtm_invd_dn2: f64 = *var_jd_nvtm_invd_dn2_slot;
        let mut var_jd_nvtm_invd_dn4: f64 = *var_jd_nvtm_invd_dn4_slot;
        let mut var_jd_nvtm_invd_dn5: f64 = *var_jd_nvtm_invd_dn5_slot;
        let mut var_jd_nvtm_invd_dn6: f64 = *var_jd_nvtm_invd_dn6_slot;
        let mut var_jd_nvtm_invd_dn7: f64 = *var_jd_nvtm_invd_dn7_slot;
        let mut var_jd_nvtm_invd_dn8: f64 = *var_jd_nvtm_invd_dn8_slot;
        let mut var_jd_nvtm_invd_dn9: f64 = *var_jd_nvtm_invd_dn9_slot;
        let mut var_jd_nvtm_invd_rv: f64 = *var_jd_nvtm_invd_rv_slot;
        let mut var_jd_nvtm_invs: f64 = *var_jd_nvtm_invs_slot;
        let mut var_jd_nvtm_invs_dn0: f64 = *var_jd_nvtm_invs_dn0_slot;
        let mut var_jd_nvtm_invs_dn10: f64 = *var_jd_nvtm_invs_dn10_slot;
        let mut var_jd_nvtm_invs_dn11: f64 = *var_jd_nvtm_invs_dn11_slot;
        let mut var_jd_nvtm_invs_dn14: f64 = *var_jd_nvtm_invs_dn14_slot;
        let mut var_jd_nvtm_invs_dn2: f64 = *var_jd_nvtm_invs_dn2_slot;
        let mut var_jd_nvtm_invs_dn4: f64 = *var_jd_nvtm_invs_dn4_slot;
        let mut var_jd_nvtm_invs_dn5: f64 = *var_jd_nvtm_invs_dn5_slot;
        let mut var_jd_nvtm_invs_dn6: f64 = *var_jd_nvtm_invs_dn6_slot;
        let mut var_jd_nvtm_invs_dn7: f64 = *var_jd_nvtm_invs_dn7_slot;
        let mut var_jd_nvtm_invs_dn8: f64 = *var_jd_nvtm_invs_dn8_slot;
        let mut var_jd_nvtm_invs_dn9: f64 = *var_jd_nvtm_invs_dn9_slot;
        let mut var_jd_nvtm_invs_rv: f64 = *var_jd_nvtm_invs_rv_slot;
        let mut var_pzbssw: f64 = *var_pzbssw_slot;
        let mut var_pzbssw_dn0: f64 = *var_pzbssw_dn0_slot;
        let mut var_pzbssw_dn10: f64 = *var_pzbssw_dn10_slot;
        let mut var_pzbssw_dn11: f64 = *var_pzbssw_dn11_slot;
        let mut var_pzbssw_dn14: f64 = *var_pzbssw_dn14_slot;
        let mut var_pzbssw_dn2: f64 = *var_pzbssw_dn2_slot;
        let mut var_pzbssw_dn4: f64 = *var_pzbssw_dn4_slot;
        let mut var_pzbssw_dn5: f64 = *var_pzbssw_dn5_slot;
        let mut var_pzbssw_dn6: f64 = *var_pzbssw_dn6_slot;
        let mut var_pzbssw_dn7: f64 = *var_pzbssw_dn7_slot;
        let mut var_pzbssw_dn8: f64 = *var_pzbssw_dn8_slot;
        let mut var_pzbssw_dn9: f64 = *var_pzbssw_dn9_slot;
        let mut var_pzbssw_rv: f64 = *var_pzbssw_rv_slot;
        let mut var_pzbsswg: f64 = *var_pzbsswg_slot;
        let mut var_pzbsswg_dn0: f64 = *var_pzbsswg_dn0_slot;
        let mut var_pzbsswg_dn10: f64 = *var_pzbsswg_dn10_slot;
        let mut var_pzbsswg_dn11: f64 = *var_pzbsswg_dn11_slot;
        let mut var_pzbsswg_dn14: f64 = *var_pzbsswg_dn14_slot;
        let mut var_pzbsswg_dn2: f64 = *var_pzbsswg_dn2_slot;
        let mut var_pzbsswg_dn4: f64 = *var_pzbsswg_dn4_slot;
        let mut var_pzbsswg_dn5: f64 = *var_pzbsswg_dn5_slot;
        let mut var_pzbsswg_dn6: f64 = *var_pzbsswg_dn6_slot;
        let mut var_pzbsswg_dn7: f64 = *var_pzbsswg_dn7_slot;
        let mut var_pzbsswg_dn8: f64 = *var_pzbsswg_dn8_slot;
        let mut var_pzbsswg_dn9: f64 = *var_pzbsswg_dn9_slot;
        let mut var_pzbsswg_rv: f64 = *var_pzbsswg_rv_slot;
        let mut var_qbd_qs: f64 = *var_qbd_qs_slot;
        let mut var_qbd_qs_dn0: f64 = *var_qbd_qs_dn0_slot;
        let mut var_qbd_qs_dn10: f64 = *var_qbd_qs_dn10_slot;
        let mut var_qbd_qs_dn11: f64 = *var_qbd_qs_dn11_slot;
        let mut var_qbd_qs_dn14: f64 = *var_qbd_qs_dn14_slot;
        let mut var_qbd_qs_dn2: f64 = *var_qbd_qs_dn2_slot;
        let mut var_qbd_qs_dn4: f64 = *var_qbd_qs_dn4_slot;
        let mut var_qbd_qs_dn5: f64 = *var_qbd_qs_dn5_slot;
        let mut var_qbd_qs_dn6: f64 = *var_qbd_qs_dn6_slot;
        let mut var_qbd_qs_dn7: f64 = *var_qbd_qs_dn7_slot;
        let mut var_qbd_qs_dn8: f64 = *var_qbd_qs_dn8_slot;
        let mut var_qbd_qs_dn9: f64 = *var_qbd_qs_dn9_slot;
        let mut var_qbd_qs_rv: f64 = *var_qbd_qs_rv_slot;
        let mut var_qbdld_add: f64 = *var_qbdld_add_slot;
        let mut var_qbdld_add_dn0: f64 = *var_qbdld_add_dn0_slot;
        let mut var_qbdld_add_dn10: f64 = *var_qbdld_add_dn10_slot;
        let mut var_qbdld_add_dn11: f64 = *var_qbdld_add_dn11_slot;
        let mut var_qbdld_add_dn14: f64 = *var_qbdld_add_dn14_slot;
        let mut var_qbdld_add_dn2: f64 = *var_qbdld_add_dn2_slot;
        let mut var_qbdld_add_dn4: f64 = *var_qbdld_add_dn4_slot;
        let mut var_qbdld_add_dn5: f64 = *var_qbdld_add_dn5_slot;
        let mut var_qbdld_add_dn6: f64 = *var_qbdld_add_dn6_slot;
        let mut var_qbdld_add_dn7: f64 = *var_qbdld_add_dn7_slot;
        let mut var_qbdld_add_dn8: f64 = *var_qbdld_add_dn8_slot;
        let mut var_qbdld_add_dn9: f64 = *var_qbdld_add_dn9_slot;
        let mut var_qbdld_add_rv: f64 = *var_qbdld_add_rv_slot;
        let mut var_qbsld_add: f64 = *var_qbsld_add_slot;
        let mut var_qbsld_add_dn0: f64 = *var_qbsld_add_dn0_slot;
        let mut var_qbsld_add_dn10: f64 = *var_qbsld_add_dn10_slot;
        let mut var_qbsld_add_dn11: f64 = *var_qbsld_add_dn11_slot;
        let mut var_qbsld_add_dn14: f64 = *var_qbsld_add_dn14_slot;
        let mut var_qbsld_add_dn2: f64 = *var_qbsld_add_dn2_slot;
        let mut var_qbsld_add_dn4: f64 = *var_qbsld_add_dn4_slot;
        let mut var_qbsld_add_dn5: f64 = *var_qbsld_add_dn5_slot;
        let mut var_qbsld_add_dn6: f64 = *var_qbsld_add_dn6_slot;
        let mut var_qbsld_add_dn7: f64 = *var_qbsld_add_dn7_slot;
        let mut var_qbsld_add_dn8: f64 = *var_qbsld_add_dn8_slot;
        let mut var_qbsld_add_dn9: f64 = *var_qbsld_add_dn9_slot;
        let mut var_qbsld_add_rv: f64 = *var_qbsld_add_rv_slot;
        let mut var_qovd_add: f64 = *var_qovd_add_slot;
        let mut var_qovd_add_dn0: f64 = *var_qovd_add_dn0_slot;
        let mut var_qovd_add_dn10: f64 = *var_qovd_add_dn10_slot;
        let mut var_qovd_add_dn11: f64 = *var_qovd_add_dn11_slot;
        let mut var_qovd_add_dn14: f64 = *var_qovd_add_dn14_slot;
        let mut var_qovd_add_dn2: f64 = *var_qovd_add_dn2_slot;
        let mut var_qovd_add_dn4: f64 = *var_qovd_add_dn4_slot;
        let mut var_qovd_add_dn5: f64 = *var_qovd_add_dn5_slot;
        let mut var_qovd_add_dn6: f64 = *var_qovd_add_dn6_slot;
        let mut var_qovd_add_dn7: f64 = *var_qovd_add_dn7_slot;
        let mut var_qovd_add_dn8: f64 = *var_qovd_add_dn8_slot;
        let mut var_qovd_add_dn9: f64 = *var_qovd_add_dn9_slot;
        let mut var_qovd_add_rv: f64 = *var_qovd_add_rv_slot;
        let mut var_qovs_add: f64 = *var_qovs_add_slot;
        let mut var_qovs_add_dn0: f64 = *var_qovs_add_dn0_slot;
        let mut var_qovs_add_dn10: f64 = *var_qovs_add_dn10_slot;
        let mut var_qovs_add_dn11: f64 = *var_qovs_add_dn11_slot;
        let mut var_qovs_add_dn14: f64 = *var_qovs_add_dn14_slot;
        let mut var_qovs_add_dn2: f64 = *var_qovs_add_dn2_slot;
        let mut var_qovs_add_dn4: f64 = *var_qovs_add_dn4_slot;
        let mut var_qovs_add_dn5: f64 = *var_qovs_add_dn5_slot;
        let mut var_qovs_add_dn6: f64 = *var_qovs_add_dn6_slot;
        let mut var_qovs_add_dn7: f64 = *var_qovs_add_dn7_slot;
        let mut var_qovs_add_dn8: f64 = *var_qovs_add_dn8_slot;
        let mut var_qovs_add_dn9: f64 = *var_qovs_add_dn9_slot;
        let mut var_qovs_add_rv: f64 = *var_qovs_add_rv_slot;
        let mut var_sarg: f64 = *var_sarg_slot;
        let mut var_sarg_dn0: f64 = *var_sarg_dn0_slot;
        let mut var_sarg_dn10: f64 = *var_sarg_dn10_slot;
        let mut var_sarg_dn11: f64 = *var_sarg_dn11_slot;
        let mut var_sarg_dn14: f64 = *var_sarg_dn14_slot;
        let mut var_sarg_dn2: f64 = *var_sarg_dn2_slot;
        let mut var_sarg_dn4: f64 = *var_sarg_dn4_slot;
        let mut var_sarg_dn5: f64 = *var_sarg_dn5_slot;
        let mut var_sarg_dn6: f64 = *var_sarg_dn6_slot;
        let mut var_sarg_dn7: f64 = *var_sarg_dn7_slot;
        let mut var_sarg_dn8: f64 = *var_sarg_dn8_slot;
        let mut var_sarg_dn9: f64 = *var_sarg_dn9_slot;
        let mut var_sarg_rv: f64 = *var_sarg_rv_slot;
        let mut var_start_of_loopl: f64 = *var_start_of_loopl_slot;
        let mut var_start_of_loopl_rv: f64 = *var_start_of_loopl_rv_slot;
        let mut var_start_of_mobility: f64 = *var_start_of_mobility_slot;
        let mut var_start_of_mobility_rv: f64 = *var_start_of_mobility_rv_slot;
        let mut var_vbd_jct: f64 = *var_vbd_jct_slot;
        let mut var_vbd_jct_dn0: f64 = *var_vbd_jct_dn0_slot;
        let mut var_vbd_jct_dn10: f64 = *var_vbd_jct_dn10_slot;
        let mut var_vbd_jct_rv: f64 = *var_vbd_jct_rv_slot;
        let mut var_vbdi_jct: f64 = *var_vbdi_jct_slot;
        let mut var_vbdi_jct_dn6: f64 = *var_vbdi_jct_dn6_slot;
        let mut var_vbdi_jct_dn9: f64 = *var_vbdi_jct_dn9_slot;
        let mut var_vbdi_jct_rv: f64 = *var_vbdi_jct_rv_slot;
        let mut var_vbdt: f64 = *var_vbdt_slot;
        let mut var_vbdt_dn0: f64 = *var_vbdt_dn0_slot;
        let mut var_vbdt_dn10: f64 = *var_vbdt_dn10_slot;
        let mut var_vbdt_dn11: f64 = *var_vbdt_dn11_slot;
        let mut var_vbdt_dn14: f64 = *var_vbdt_dn14_slot;
        let mut var_vbdt_dn2: f64 = *var_vbdt_dn2_slot;
        let mut var_vbdt_dn4: f64 = *var_vbdt_dn4_slot;
        let mut var_vbdt_dn5: f64 = *var_vbdt_dn5_slot;
        let mut var_vbdt_dn6: f64 = *var_vbdt_dn6_slot;
        let mut var_vbdt_dn7: f64 = *var_vbdt_dn7_slot;
        let mut var_vbdt_dn8: f64 = *var_vbdt_dn8_slot;
        let mut var_vbdt_dn9: f64 = *var_vbdt_dn9_slot;
        let mut var_vbdt_rv: f64 = *var_vbdt_rv_slot;
        let mut var_vbpdp: f64 = *var_vbpdp_slot;
        let mut var_vbpdp_dn6: f64 = *var_vbpdp_dn6_slot;
        let mut var_vbpdp_dn9: f64 = *var_vbpdp_dn9_slot;
        let mut var_vbpdp_rv: f64 = *var_vbpdp_rv_slot;
        let mut var_vbpsp: f64 = *var_vbpsp_slot;
        let mut var_vbpsp_dn8: f64 = *var_vbpsp_dn8_slot;
        let mut var_vbpsp_dn9: f64 = *var_vbpsp_dn9_slot;
        let mut var_vbpsp_rv: f64 = *var_vbpsp_rv_slot;
        let mut var_vbs_jct: f64 = *var_vbs_jct_slot;
        let mut var_vbs_jct_dn11: f64 = *var_vbs_jct_dn11_slot;
        let mut var_vbs_jct_dn2: f64 = *var_vbs_jct_dn2_slot;
        let mut var_vbs_jct_rv: f64 = *var_vbs_jct_rv_slot;
        let mut var_vbsi_jct: f64 = *var_vbsi_jct_slot;
        let mut var_vbsi_jct_dn8: f64 = *var_vbsi_jct_dn8_slot;
        let mut var_vbsi_jct_dn9: f64 = *var_vbsi_jct_dn9_slot;
        let mut var_vbsi_jct_rv: f64 = *var_vbsi_jct_rv_slot;
        let mut var_vbst: f64 = *var_vbst_slot;
        let mut var_vbst_dn0: f64 = *var_vbst_dn0_slot;
        let mut var_vbst_dn10: f64 = *var_vbst_dn10_slot;
        let mut var_vbst_dn11: f64 = *var_vbst_dn11_slot;
        let mut var_vbst_dn14: f64 = *var_vbst_dn14_slot;
        let mut var_vbst_dn2: f64 = *var_vbst_dn2_slot;
        let mut var_vbst_dn4: f64 = *var_vbst_dn4_slot;
        let mut var_vbst_dn5: f64 = *var_vbst_dn5_slot;
        let mut var_vbst_dn6: f64 = *var_vbst_dn6_slot;
        let mut var_vbst_dn7: f64 = *var_vbst_dn7_slot;
        let mut var_vbst_dn8: f64 = *var_vbst_dn8_slot;
        let mut var_vbst_dn9: f64 = *var_vbst_dn9_slot;
        let mut var_vbst_rv: f64 = *var_vbst_rv_slot;
        let mut var_vdbd: f64 = *var_vdbd_slot;
        let mut var_vdbd_dn0: f64 = *var_vdbd_dn0_slot;
        let mut var_vdbd_dn10: f64 = *var_vdbd_dn10_slot;
        let mut var_vdbd_rv: f64 = *var_vdbd_rv_slot;
        let mut var_vsbs: f64 = *var_vsbs_slot;
        let mut var_vsbs_dn11: f64 = *var_vsbs_dn11_slot;
        let mut var_vsbs_dn2: f64 = *var_vsbs_dn2_slot;
        let mut var_vsbs_rv: f64 = *var_vsbs_rv_slot;
        let mut var_wjuncld: f64 = *var_wjuncld_slot;
        let mut var_wjuncld_dn0: f64 = *var_wjuncld_dn0_slot;
        let mut var_wjuncld_dn10: f64 = *var_wjuncld_dn10_slot;
        let mut var_wjuncld_dn11: f64 = *var_wjuncld_dn11_slot;
        let mut var_wjuncld_dn14: f64 = *var_wjuncld_dn14_slot;
        let mut var_wjuncld_dn2: f64 = *var_wjuncld_dn2_slot;
        let mut var_wjuncld_dn4: f64 = *var_wjuncld_dn4_slot;
        let mut var_wjuncld_dn5: f64 = *var_wjuncld_dn5_slot;
        let mut var_wjuncld_dn6: f64 = *var_wjuncld_dn6_slot;
        let mut var_wjuncld_dn7: f64 = *var_wjuncld_dn7_slot;
        let mut var_wjuncld_dn8: f64 = *var_wjuncld_dn8_slot;
        let mut var_wjuncld_dn9: f64 = *var_wjuncld_dn9_slot;
        let mut var_wjuncld_rv: f64 = *var_wjuncld_rv_slot;

        var_pzbssw = 0.0;
        var_pzbssw_dn0 = 0.0;
        var_pzbssw_dn2 = 0.0;
        var_pzbssw_dn4 = 0.0;
        var_pzbssw_dn5 = 0.0;
        var_pzbssw_dn6 = 0.0;
        var_pzbssw_dn7 = 0.0;
        var_pzbssw_dn8 = 0.0;
        var_pzbssw_dn9 = 0.0;
        var_pzbssw_dn10 = 0.0;
        var_pzbssw_dn11 = 0.0;
        var_pzbssw_dn14 = 0.0;
        var_pzbssw_rv = 0.0;

        var_pzbsswg = 0.0;
        var_pzbsswg_dn0 = 0.0;
        var_pzbsswg_dn2 = 0.0;
        var_pzbsswg_dn4 = 0.0;
        var_pzbsswg_dn5 = 0.0;
        var_pzbsswg_dn6 = 0.0;
        var_pzbsswg_dn7 = 0.0;
        var_pzbsswg_dn8 = 0.0;
        var_pzbsswg_dn9 = 0.0;
        var_pzbsswg_dn10 = 0.0;
        var_pzbsswg_dn11 = 0.0;
        var_pzbsswg_dn14 = 0.0;
        var_pzbsswg_rv = 0.0;

        var_sarg = 0.0;
        var_sarg_dn0 = 0.0;
        var_sarg_dn2 = 0.0;
        var_sarg_dn4 = 0.0;
        var_sarg_dn5 = 0.0;
        var_sarg_dn6 = 0.0;
        var_sarg_dn7 = 0.0;
        var_sarg_dn8 = 0.0;
        var_sarg_dn9 = 0.0;
        var_sarg_dn10 = 0.0;
        var_sarg_dn11 = 0.0;
        var_sarg_dn14 = 0.0;
        var_sarg_rv = 0.0;

        var_vsbs = 0.0;
        var_vsbs_dn2 = 0.0;
        var_vsbs_dn11 = 0.0;
        var_vsbs_rv = 0.0;

        var_vdbd = 0.0;
        var_vdbd_dn0 = 0.0;
        var_vdbd_dn10 = 0.0;
        var_vdbd_rv = 0.0;

        var_vbs_jct = 0.0;
        var_vbs_jct_dn2 = 0.0;
        var_vbs_jct_dn11 = 0.0;
        var_vbs_jct_rv = 0.0;

        var_vbd_jct = 0.0;
        var_vbd_jct_dn0 = 0.0;
        var_vbd_jct_dn10 = 0.0;
        var_vbd_jct_rv = 0.0;

        var_vbpsp = 0.0;
        var_vbpsp_dn8 = 0.0;
        var_vbpsp_dn9 = 0.0;
        var_vbpsp_rv = 0.0;

        var_vbpdp = 0.0;
        var_vbpdp_dn6 = 0.0;
        var_vbpdp_dn9 = 0.0;
        var_vbpdp_rv = 0.0;

        var_vbsi_jct = 0.0;
        var_vbsi_jct_dn8 = 0.0;
        var_vbsi_jct_dn9 = 0.0;
        var_vbsi_jct_rv = 0.0;

        var_vbdi_jct = 0.0;
        var_vbdi_jct_dn6 = 0.0;
        var_vbdi_jct_dn9 = 0.0;
        var_vbdi_jct_rv = 0.0;

        var_exptempd = 0.0;
        var_exptempd_dn0 = 0.0;
        var_exptempd_dn2 = 0.0;
        var_exptempd_dn4 = 0.0;
        var_exptempd_dn5 = 0.0;
        var_exptempd_dn6 = 0.0;
        var_exptempd_dn7 = 0.0;
        var_exptempd_dn8 = 0.0;
        var_exptempd_dn9 = 0.0;
        var_exptempd_dn10 = 0.0;
        var_exptempd_dn11 = 0.0;
        var_exptempd_dn14 = 0.0;
        var_exptempd_rv = 0.0;

        var_exptemps = 0.0;
        var_exptemps_dn0 = 0.0;
        var_exptemps_dn2 = 0.0;
        var_exptemps_dn4 = 0.0;
        var_exptemps_dn5 = 0.0;
        var_exptemps_dn6 = 0.0;
        var_exptemps_dn7 = 0.0;
        var_exptemps_dn8 = 0.0;
        var_exptemps_dn9 = 0.0;
        var_exptemps_dn10 = 0.0;
        var_exptemps_dn11 = 0.0;
        var_exptemps_dn14 = 0.0;
        var_exptemps_rv = 0.0;

        var_isbd = 0.0;
        var_isbd_dn0 = 0.0;
        var_isbd_dn2 = 0.0;
        var_isbd_dn4 = 0.0;
        var_isbd_dn5 = 0.0;
        var_isbd_dn6 = 0.0;
        var_isbd_dn7 = 0.0;
        var_isbd_dn8 = 0.0;
        var_isbd_dn9 = 0.0;
        var_isbd_dn10 = 0.0;
        var_isbd_dn11 = 0.0;
        var_isbd_dn14 = 0.0;
        var_isbd_rv = 0.0;

        var_isbs = 0.0;
        var_isbs_dn0 = 0.0;
        var_isbs_dn2 = 0.0;
        var_isbs_dn4 = 0.0;
        var_isbs_dn5 = 0.0;
        var_isbs_dn6 = 0.0;
        var_isbs_dn7 = 0.0;
        var_isbs_dn8 = 0.0;
        var_isbs_dn9 = 0.0;
        var_isbs_dn10 = 0.0;
        var_isbs_dn11 = 0.0;
        var_isbs_dn14 = 0.0;
        var_isbs_rv = 0.0;

        var_jd_expcd = 0.0;
        var_jd_expcd_dn0 = 0.0;
        var_jd_expcd_dn2 = 0.0;
        var_jd_expcd_dn4 = 0.0;
        var_jd_expcd_dn5 = 0.0;
        var_jd_expcd_dn6 = 0.0;
        var_jd_expcd_dn7 = 0.0;
        var_jd_expcd_dn8 = 0.0;
        var_jd_expcd_dn9 = 0.0;
        var_jd_expcd_dn10 = 0.0;
        var_jd_expcd_dn11 = 0.0;
        var_jd_expcd_dn14 = 0.0;
        var_jd_expcd_rv = 0.0;

        var_jd_expcs = 0.0;
        var_jd_expcs_dn0 = 0.0;
        var_jd_expcs_dn2 = 0.0;
        var_jd_expcs_dn4 = 0.0;
        var_jd_expcs_dn5 = 0.0;
        var_jd_expcs_dn6 = 0.0;
        var_jd_expcs_dn7 = 0.0;
        var_jd_expcs_dn8 = 0.0;
        var_jd_expcs_dn9 = 0.0;
        var_jd_expcs_dn10 = 0.0;
        var_jd_expcs_dn11 = 0.0;
        var_jd_expcs_dn14 = 0.0;
        var_jd_expcs_rv = 0.0;

        var_vbdt = 0.0;
        var_vbdt_dn0 = 0.0;
        var_vbdt_dn2 = 0.0;
        var_vbdt_dn4 = 0.0;
        var_vbdt_dn5 = 0.0;
        var_vbdt_dn6 = 0.0;
        var_vbdt_dn7 = 0.0;
        var_vbdt_dn8 = 0.0;
        var_vbdt_dn9 = 0.0;
        var_vbdt_dn10 = 0.0;
        var_vbdt_dn11 = 0.0;
        var_vbdt_dn14 = 0.0;
        var_vbdt_rv = 0.0;

        var_vbst = 0.0;
        var_vbst_dn0 = 0.0;
        var_vbst_dn2 = 0.0;
        var_vbst_dn4 = 0.0;
        var_vbst_dn5 = 0.0;
        var_vbst_dn6 = 0.0;
        var_vbst_dn7 = 0.0;
        var_vbst_dn8 = 0.0;
        var_vbst_dn9 = 0.0;
        var_vbst_dn10 = 0.0;
        var_vbst_dn11 = 0.0;
        var_vbst_dn14 = 0.0;
        var_vbst_rv = 0.0;

        var_jd_nvtm_invd = 0.0;
        var_jd_nvtm_invd_dn0 = 0.0;
        var_jd_nvtm_invd_dn2 = 0.0;
        var_jd_nvtm_invd_dn4 = 0.0;
        var_jd_nvtm_invd_dn5 = 0.0;
        var_jd_nvtm_invd_dn6 = 0.0;
        var_jd_nvtm_invd_dn7 = 0.0;
        var_jd_nvtm_invd_dn8 = 0.0;
        var_jd_nvtm_invd_dn9 = 0.0;
        var_jd_nvtm_invd_dn10 = 0.0;
        var_jd_nvtm_invd_dn11 = 0.0;
        var_jd_nvtm_invd_dn14 = 0.0;
        var_jd_nvtm_invd_rv = 0.0;

        var_jd_nvtm_invs = 0.0;
        var_jd_nvtm_invs_dn0 = 0.0;
        var_jd_nvtm_invs_dn2 = 0.0;
        var_jd_nvtm_invs_dn4 = 0.0;
        var_jd_nvtm_invs_dn5 = 0.0;
        var_jd_nvtm_invs_dn6 = 0.0;
        var_jd_nvtm_invs_dn7 = 0.0;
        var_jd_nvtm_invs_dn8 = 0.0;
        var_jd_nvtm_invs_dn9 = 0.0;
        var_jd_nvtm_invs_dn10 = 0.0;
        var_jd_nvtm_invs_dn11 = 0.0;
        var_jd_nvtm_invs_dn14 = 0.0;
        var_jd_nvtm_invs_rv = 0.0;

        var_end_of_part_1 = 0.0;
        var_end_of_part_1_rv = 0.0;

        var_flg_brk1 = 0.0;
        var_flg_brk1_rv = 0.0;

        var_start_of_loopl = 0.0;
        var_start_of_loopl_rv = 0.0;

        var_flg_brk2 = 0.0;
        var_flg_brk2_rv = 0.0;

        var_start_of_mobility = 0.0;
        var_start_of_mobility_rv = 0.0;

        var_qbd_qs = 0.0;
        var_qbd_qs_dn0 = 0.0;
        var_qbd_qs_dn2 = 0.0;
        var_qbd_qs_dn4 = 0.0;
        var_qbd_qs_dn5 = 0.0;
        var_qbd_qs_dn6 = 0.0;
        var_qbd_qs_dn7 = 0.0;
        var_qbd_qs_dn8 = 0.0;
        var_qbd_qs_dn9 = 0.0;
        var_qbd_qs_dn10 = 0.0;
        var_qbd_qs_dn11 = 0.0;
        var_qbd_qs_dn14 = 0.0;
        var_qbd_qs_rv = 0.0;

        var_isbd_btm = 0.0;
        var_isbd_btm_dn0 = 0.0;
        var_isbd_btm_dn2 = 0.0;
        var_isbd_btm_dn4 = 0.0;
        var_isbd_btm_dn5 = 0.0;
        var_isbd_btm_dn6 = 0.0;
        var_isbd_btm_dn7 = 0.0;
        var_isbd_btm_dn8 = 0.0;
        var_isbd_btm_dn9 = 0.0;
        var_isbd_btm_dn10 = 0.0;
        var_isbd_btm_dn11 = 0.0;
        var_isbd_btm_dn14 = 0.0;
        var_isbd_btm_rv = 0.0;

        var_isbd2_btm = 0.0;
        var_isbd2_btm_dn0 = 0.0;
        var_isbd2_btm_dn2 = 0.0;
        var_isbd2_btm_dn4 = 0.0;
        var_isbd2_btm_dn5 = 0.0;
        var_isbd2_btm_dn6 = 0.0;
        var_isbd2_btm_dn7 = 0.0;
        var_isbd2_btm_dn8 = 0.0;
        var_isbd2_btm_dn9 = 0.0;
        var_isbd2_btm_dn10 = 0.0;
        var_isbd2_btm_dn11 = 0.0;
        var_isbd2_btm_dn14 = 0.0;
        var_isbd2_btm_rv = 0.0;

        var_isbd_sws = 0.0;
        var_isbd_sws_dn0 = 0.0;
        var_isbd_sws_dn2 = 0.0;
        var_isbd_sws_dn4 = 0.0;
        var_isbd_sws_dn5 = 0.0;
        var_isbd_sws_dn6 = 0.0;
        var_isbd_sws_dn7 = 0.0;
        var_isbd_sws_dn8 = 0.0;
        var_isbd_sws_dn9 = 0.0;
        var_isbd_sws_dn10 = 0.0;
        var_isbd_sws_dn11 = 0.0;
        var_isbd_sws_dn14 = 0.0;
        var_isbd_sws_rv = 0.0;

        var_isbd2_sws = 0.0;
        var_isbd2_sws_dn0 = 0.0;
        var_isbd2_sws_dn2 = 0.0;
        var_isbd2_sws_dn4 = 0.0;
        var_isbd2_sws_dn5 = 0.0;
        var_isbd2_sws_dn6 = 0.0;
        var_isbd2_sws_dn7 = 0.0;
        var_isbd2_sws_dn8 = 0.0;
        var_isbd2_sws_dn9 = 0.0;
        var_isbd2_sws_dn10 = 0.0;
        var_isbd2_sws_dn11 = 0.0;
        var_isbd2_sws_dn14 = 0.0;
        var_isbd2_sws_rv = 0.0;

        var_isbd_swg = 0.0;
        var_isbd_swg_dn0 = 0.0;
        var_isbd_swg_dn2 = 0.0;
        var_isbd_swg_dn4 = 0.0;
        var_isbd_swg_dn5 = 0.0;
        var_isbd_swg_dn6 = 0.0;
        var_isbd_swg_dn7 = 0.0;
        var_isbd_swg_dn8 = 0.0;
        var_isbd_swg_dn9 = 0.0;
        var_isbd_swg_dn10 = 0.0;
        var_isbd_swg_dn11 = 0.0;
        var_isbd_swg_dn14 = 0.0;
        var_isbd_swg_rv = 0.0;

        var_isbd2_swg = 0.0;
        var_isbd2_swg_dn0 = 0.0;
        var_isbd2_swg_dn2 = 0.0;
        var_isbd2_swg_dn4 = 0.0;
        var_isbd2_swg_dn5 = 0.0;
        var_isbd2_swg_dn6 = 0.0;
        var_isbd2_swg_dn7 = 0.0;
        var_isbd2_swg_dn8 = 0.0;
        var_isbd2_swg_dn9 = 0.0;
        var_isbd2_swg_dn10 = 0.0;
        var_isbd2_swg_dn11 = 0.0;
        var_isbd2_swg_dn14 = 0.0;
        var_isbd2_swg_rv = 0.0;

        var_isbs_btm = 0.0;
        var_isbs_btm_dn0 = 0.0;
        var_isbs_btm_dn2 = 0.0;
        var_isbs_btm_dn4 = 0.0;
        var_isbs_btm_dn5 = 0.0;
        var_isbs_btm_dn6 = 0.0;
        var_isbs_btm_dn7 = 0.0;
        var_isbs_btm_dn8 = 0.0;
        var_isbs_btm_dn9 = 0.0;
        var_isbs_btm_dn10 = 0.0;
        var_isbs_btm_dn11 = 0.0;
        var_isbs_btm_dn14 = 0.0;
        var_isbs_btm_rv = 0.0;

        var_isbs2_btm = 0.0;
        var_isbs2_btm_dn0 = 0.0;
        var_isbs2_btm_dn2 = 0.0;
        var_isbs2_btm_dn4 = 0.0;
        var_isbs2_btm_dn5 = 0.0;
        var_isbs2_btm_dn6 = 0.0;
        var_isbs2_btm_dn7 = 0.0;
        var_isbs2_btm_dn8 = 0.0;
        var_isbs2_btm_dn9 = 0.0;
        var_isbs2_btm_dn10 = 0.0;
        var_isbs2_btm_dn11 = 0.0;
        var_isbs2_btm_dn14 = 0.0;
        var_isbs2_btm_rv = 0.0;

        var_isbs_sws = 0.0;
        var_isbs_sws_dn0 = 0.0;
        var_isbs_sws_dn2 = 0.0;
        var_isbs_sws_dn4 = 0.0;
        var_isbs_sws_dn5 = 0.0;
        var_isbs_sws_dn6 = 0.0;
        var_isbs_sws_dn7 = 0.0;
        var_isbs_sws_dn8 = 0.0;
        var_isbs_sws_dn9 = 0.0;
        var_isbs_sws_dn10 = 0.0;
        var_isbs_sws_dn11 = 0.0;
        var_isbs_sws_dn14 = 0.0;
        var_isbs_sws_rv = 0.0;

        var_isbs2_sws = 0.0;
        var_isbs2_sws_dn0 = 0.0;
        var_isbs2_sws_dn2 = 0.0;
        var_isbs2_sws_dn4 = 0.0;
        var_isbs2_sws_dn5 = 0.0;
        var_isbs2_sws_dn6 = 0.0;
        var_isbs2_sws_dn7 = 0.0;
        var_isbs2_sws_dn8 = 0.0;
        var_isbs2_sws_dn9 = 0.0;
        var_isbs2_sws_dn10 = 0.0;
        var_isbs2_sws_dn11 = 0.0;
        var_isbs2_sws_dn14 = 0.0;
        var_isbs2_sws_rv = 0.0;

        var_isbs_swg = 0.0;
        var_isbs_swg_dn0 = 0.0;
        var_isbs_swg_dn2 = 0.0;
        var_isbs_swg_dn4 = 0.0;
        var_isbs_swg_dn5 = 0.0;
        var_isbs_swg_dn6 = 0.0;
        var_isbs_swg_dn7 = 0.0;
        var_isbs_swg_dn8 = 0.0;
        var_isbs_swg_dn9 = 0.0;
        var_isbs_swg_dn10 = 0.0;
        var_isbs_swg_dn11 = 0.0;
        var_isbs_swg_dn14 = 0.0;
        var_isbs_swg_rv = 0.0;

        var_isbs2_swg = 0.0;
        var_isbs2_swg_dn0 = 0.0;
        var_isbs2_swg_dn2 = 0.0;
        var_isbs2_swg_dn4 = 0.0;
        var_isbs2_swg_dn5 = 0.0;
        var_isbs2_swg_dn6 = 0.0;
        var_isbs2_swg_dn7 = 0.0;
        var_isbs2_swg_dn8 = 0.0;
        var_isbs2_swg_dn9 = 0.0;
        var_isbs2_swg_dn10 = 0.0;
        var_isbs2_swg_dn11 = 0.0;
        var_isbs2_swg_dn14 = 0.0;
        var_isbs2_swg_rv = 0.0;

        var_qovd_add = 0.0;
        var_qovd_add_dn0 = 0.0;
        var_qovd_add_dn2 = 0.0;
        var_qovd_add_dn4 = 0.0;
        var_qovd_add_dn5 = 0.0;
        var_qovd_add_dn6 = 0.0;
        var_qovd_add_dn7 = 0.0;
        var_qovd_add_dn8 = 0.0;
        var_qovd_add_dn9 = 0.0;
        var_qovd_add_dn10 = 0.0;
        var_qovd_add_dn11 = 0.0;
        var_qovd_add_dn14 = 0.0;
        var_qovd_add_rv = 0.0;

        var_qovs_add = 0.0;
        var_qovs_add_dn0 = 0.0;
        var_qovs_add_dn2 = 0.0;
        var_qovs_add_dn4 = 0.0;
        var_qovs_add_dn5 = 0.0;
        var_qovs_add_dn6 = 0.0;
        var_qovs_add_dn7 = 0.0;
        var_qovs_add_dn8 = 0.0;
        var_qovs_add_dn9 = 0.0;
        var_qovs_add_dn10 = 0.0;
        var_qovs_add_dn11 = 0.0;
        var_qovs_add_dn14 = 0.0;
        var_qovs_add_rv = 0.0;

        var_qbdld_add = 0.0;
        var_qbdld_add_dn0 = 0.0;
        var_qbdld_add_dn2 = 0.0;
        var_qbdld_add_dn4 = 0.0;
        var_qbdld_add_dn5 = 0.0;
        var_qbdld_add_dn6 = 0.0;
        var_qbdld_add_dn7 = 0.0;
        var_qbdld_add_dn8 = 0.0;
        var_qbdld_add_dn9 = 0.0;
        var_qbdld_add_dn10 = 0.0;
        var_qbdld_add_dn11 = 0.0;
        var_qbdld_add_dn14 = 0.0;
        var_qbdld_add_rv = 0.0;

        var_qbsld_add = 0.0;
        var_qbsld_add_dn0 = 0.0;
        var_qbsld_add_dn2 = 0.0;
        var_qbsld_add_dn4 = 0.0;
        var_qbsld_add_dn5 = 0.0;
        var_qbsld_add_dn6 = 0.0;
        var_qbsld_add_dn7 = 0.0;
        var_qbsld_add_dn8 = 0.0;
        var_qbsld_add_dn9 = 0.0;
        var_qbsld_add_dn10 = 0.0;
        var_qbsld_add_dn11 = 0.0;
        var_qbsld_add_dn14 = 0.0;
        var_qbsld_add_rv = 0.0;

        var_wjuncld = 0.0;
        var_wjuncld_dn0 = 0.0;
        var_wjuncld_dn2 = 0.0;
        var_wjuncld_dn4 = 0.0;
        var_wjuncld_dn5 = 0.0;
        var_wjuncld_dn6 = 0.0;
        var_wjuncld_dn7 = 0.0;
        var_wjuncld_dn8 = 0.0;
        var_wjuncld_dn9 = 0.0;
        var_wjuncld_dn10 = 0.0;
        var_wjuncld_dn11 = 0.0;
        var_wjuncld_dn14 = 0.0;
        var_wjuncld_rv = 0.0;

        var_idspt0 = 0.0;
        var_idspt0_dn0 = 0.0;
        var_idspt0_dn2 = 0.0;
        var_idspt0_dn4 = 0.0;
        var_idspt0_dn5 = 0.0;
        var_idspt0_dn6 = 0.0;
        var_idspt0_dn7 = 0.0;
        var_idspt0_dn8 = 0.0;
        var_idspt0_dn9 = 0.0;
        var_idspt0_dn10 = 0.0;
        var_idspt0_dn11 = 0.0;
        var_idspt0_dn14 = 0.0;
        var_idspt0_rv = 0.0;

        *var_end_of_part_1_slot = var_end_of_part_1;
        *var_end_of_part_1_rv_slot = var_end_of_part_1_rv;
        *var_exptempd_slot = var_exptempd;
        *var_exptempd_dn0_slot = var_exptempd_dn0;
        *var_exptempd_dn10_slot = var_exptempd_dn10;
        *var_exptempd_dn11_slot = var_exptempd_dn11;
        *var_exptempd_dn14_slot = var_exptempd_dn14;
        *var_exptempd_dn2_slot = var_exptempd_dn2;
        *var_exptempd_dn4_slot = var_exptempd_dn4;
        *var_exptempd_dn5_slot = var_exptempd_dn5;
        *var_exptempd_dn6_slot = var_exptempd_dn6;
        *var_exptempd_dn7_slot = var_exptempd_dn7;
        *var_exptempd_dn8_slot = var_exptempd_dn8;
        *var_exptempd_dn9_slot = var_exptempd_dn9;
        *var_exptempd_rv_slot = var_exptempd_rv;
        *var_exptemps_slot = var_exptemps;
        *var_exptemps_dn0_slot = var_exptemps_dn0;
        *var_exptemps_dn10_slot = var_exptemps_dn10;
        *var_exptemps_dn11_slot = var_exptemps_dn11;
        *var_exptemps_dn14_slot = var_exptemps_dn14;
        *var_exptemps_dn2_slot = var_exptemps_dn2;
        *var_exptemps_dn4_slot = var_exptemps_dn4;
        *var_exptemps_dn5_slot = var_exptemps_dn5;
        *var_exptemps_dn6_slot = var_exptemps_dn6;
        *var_exptemps_dn7_slot = var_exptemps_dn7;
        *var_exptemps_dn8_slot = var_exptemps_dn8;
        *var_exptemps_dn9_slot = var_exptemps_dn9;
        *var_exptemps_rv_slot = var_exptemps_rv;
        *var_flg_brk1_slot = var_flg_brk1;
        *var_flg_brk1_rv_slot = var_flg_brk1_rv;
        *var_flg_brk2_slot = var_flg_brk2;
        *var_flg_brk2_rv_slot = var_flg_brk2_rv;
        *var_idspt0_slot = var_idspt0;
        *var_idspt0_dn0_slot = var_idspt0_dn0;
        *var_idspt0_dn10_slot = var_idspt0_dn10;
        *var_idspt0_dn11_slot = var_idspt0_dn11;
        *var_idspt0_dn14_slot = var_idspt0_dn14;
        *var_idspt0_dn2_slot = var_idspt0_dn2;
        *var_idspt0_dn4_slot = var_idspt0_dn4;
        *var_idspt0_dn5_slot = var_idspt0_dn5;
        *var_idspt0_dn6_slot = var_idspt0_dn6;
        *var_idspt0_dn7_slot = var_idspt0_dn7;
        *var_idspt0_dn8_slot = var_idspt0_dn8;
        *var_idspt0_dn9_slot = var_idspt0_dn9;
        *var_idspt0_rv_slot = var_idspt0_rv;
        *var_isbd_slot = var_isbd;
        *var_isbd2_btm_slot = var_isbd2_btm;
        *var_isbd2_btm_dn0_slot = var_isbd2_btm_dn0;
        *var_isbd2_btm_dn10_slot = var_isbd2_btm_dn10;
        *var_isbd2_btm_dn11_slot = var_isbd2_btm_dn11;
        *var_isbd2_btm_dn14_slot = var_isbd2_btm_dn14;
        *var_isbd2_btm_dn2_slot = var_isbd2_btm_dn2;
        *var_isbd2_btm_dn4_slot = var_isbd2_btm_dn4;
        *var_isbd2_btm_dn5_slot = var_isbd2_btm_dn5;
        *var_isbd2_btm_dn6_slot = var_isbd2_btm_dn6;
        *var_isbd2_btm_dn7_slot = var_isbd2_btm_dn7;
        *var_isbd2_btm_dn8_slot = var_isbd2_btm_dn8;
        *var_isbd2_btm_dn9_slot = var_isbd2_btm_dn9;
        *var_isbd2_btm_rv_slot = var_isbd2_btm_rv;
        *var_isbd2_swg_slot = var_isbd2_swg;
        *var_isbd2_swg_dn0_slot = var_isbd2_swg_dn0;
        *var_isbd2_swg_dn10_slot = var_isbd2_swg_dn10;
        *var_isbd2_swg_dn11_slot = var_isbd2_swg_dn11;
        *var_isbd2_swg_dn14_slot = var_isbd2_swg_dn14;
        *var_isbd2_swg_dn2_slot = var_isbd2_swg_dn2;
        *var_isbd2_swg_dn4_slot = var_isbd2_swg_dn4;
        *var_isbd2_swg_dn5_slot = var_isbd2_swg_dn5;
        *var_isbd2_swg_dn6_slot = var_isbd2_swg_dn6;
        *var_isbd2_swg_dn7_slot = var_isbd2_swg_dn7;
        *var_isbd2_swg_dn8_slot = var_isbd2_swg_dn8;
        *var_isbd2_swg_dn9_slot = var_isbd2_swg_dn9;
        *var_isbd2_swg_rv_slot = var_isbd2_swg_rv;
        *var_isbd2_sws_slot = var_isbd2_sws;
        *var_isbd2_sws_dn0_slot = var_isbd2_sws_dn0;
        *var_isbd2_sws_dn10_slot = var_isbd2_sws_dn10;
        *var_isbd2_sws_dn11_slot = var_isbd2_sws_dn11;
        *var_isbd2_sws_dn14_slot = var_isbd2_sws_dn14;
        *var_isbd2_sws_dn2_slot = var_isbd2_sws_dn2;
        *var_isbd2_sws_dn4_slot = var_isbd2_sws_dn4;
        *var_isbd2_sws_dn5_slot = var_isbd2_sws_dn5;
        *var_isbd2_sws_dn6_slot = var_isbd2_sws_dn6;
        *var_isbd2_sws_dn7_slot = var_isbd2_sws_dn7;
        *var_isbd2_sws_dn8_slot = var_isbd2_sws_dn8;
        *var_isbd2_sws_dn9_slot = var_isbd2_sws_dn9;
        *var_isbd2_sws_rv_slot = var_isbd2_sws_rv;
        *var_isbd_btm_slot = var_isbd_btm;
        *var_isbd_btm_dn0_slot = var_isbd_btm_dn0;
        *var_isbd_btm_dn10_slot = var_isbd_btm_dn10;
        *var_isbd_btm_dn11_slot = var_isbd_btm_dn11;
        *var_isbd_btm_dn14_slot = var_isbd_btm_dn14;
        *var_isbd_btm_dn2_slot = var_isbd_btm_dn2;
        *var_isbd_btm_dn4_slot = var_isbd_btm_dn4;
        *var_isbd_btm_dn5_slot = var_isbd_btm_dn5;
        *var_isbd_btm_dn6_slot = var_isbd_btm_dn6;
        *var_isbd_btm_dn7_slot = var_isbd_btm_dn7;
        *var_isbd_btm_dn8_slot = var_isbd_btm_dn8;
        *var_isbd_btm_dn9_slot = var_isbd_btm_dn9;
        *var_isbd_btm_rv_slot = var_isbd_btm_rv;
        *var_isbd_dn0_slot = var_isbd_dn0;
        *var_isbd_dn10_slot = var_isbd_dn10;
        *var_isbd_dn11_slot = var_isbd_dn11;
        *var_isbd_dn14_slot = var_isbd_dn14;
        *var_isbd_dn2_slot = var_isbd_dn2;
        *var_isbd_dn4_slot = var_isbd_dn4;
        *var_isbd_dn5_slot = var_isbd_dn5;
        *var_isbd_dn6_slot = var_isbd_dn6;
        *var_isbd_dn7_slot = var_isbd_dn7;
        *var_isbd_dn8_slot = var_isbd_dn8;
        *var_isbd_dn9_slot = var_isbd_dn9;
        *var_isbd_rv_slot = var_isbd_rv;
        *var_isbd_swg_slot = var_isbd_swg;
        *var_isbd_swg_dn0_slot = var_isbd_swg_dn0;
        *var_isbd_swg_dn10_slot = var_isbd_swg_dn10;
        *var_isbd_swg_dn11_slot = var_isbd_swg_dn11;
        *var_isbd_swg_dn14_slot = var_isbd_swg_dn14;
        *var_isbd_swg_dn2_slot = var_isbd_swg_dn2;
        *var_isbd_swg_dn4_slot = var_isbd_swg_dn4;
        *var_isbd_swg_dn5_slot = var_isbd_swg_dn5;
        *var_isbd_swg_dn6_slot = var_isbd_swg_dn6;
        *var_isbd_swg_dn7_slot = var_isbd_swg_dn7;
        *var_isbd_swg_dn8_slot = var_isbd_swg_dn8;
        *var_isbd_swg_dn9_slot = var_isbd_swg_dn9;
        *var_isbd_swg_rv_slot = var_isbd_swg_rv;
        *var_isbd_sws_slot = var_isbd_sws;
        *var_isbd_sws_dn0_slot = var_isbd_sws_dn0;
        *var_isbd_sws_dn10_slot = var_isbd_sws_dn10;
        *var_isbd_sws_dn11_slot = var_isbd_sws_dn11;
        *var_isbd_sws_dn14_slot = var_isbd_sws_dn14;
        *var_isbd_sws_dn2_slot = var_isbd_sws_dn2;
        *var_isbd_sws_dn4_slot = var_isbd_sws_dn4;
        *var_isbd_sws_dn5_slot = var_isbd_sws_dn5;
        *var_isbd_sws_dn6_slot = var_isbd_sws_dn6;
        *var_isbd_sws_dn7_slot = var_isbd_sws_dn7;
        *var_isbd_sws_dn8_slot = var_isbd_sws_dn8;
        *var_isbd_sws_dn9_slot = var_isbd_sws_dn9;
        *var_isbd_sws_rv_slot = var_isbd_sws_rv;
        *var_isbs_slot = var_isbs;
        *var_isbs2_btm_slot = var_isbs2_btm;
        *var_isbs2_btm_dn0_slot = var_isbs2_btm_dn0;
        *var_isbs2_btm_dn10_slot = var_isbs2_btm_dn10;
        *var_isbs2_btm_dn11_slot = var_isbs2_btm_dn11;
        *var_isbs2_btm_dn14_slot = var_isbs2_btm_dn14;
        *var_isbs2_btm_dn2_slot = var_isbs2_btm_dn2;
        *var_isbs2_btm_dn4_slot = var_isbs2_btm_dn4;
        *var_isbs2_btm_dn5_slot = var_isbs2_btm_dn5;
        *var_isbs2_btm_dn6_slot = var_isbs2_btm_dn6;
        *var_isbs2_btm_dn7_slot = var_isbs2_btm_dn7;
        *var_isbs2_btm_dn8_slot = var_isbs2_btm_dn8;
        *var_isbs2_btm_dn9_slot = var_isbs2_btm_dn9;
        *var_isbs2_btm_rv_slot = var_isbs2_btm_rv;
        *var_isbs2_swg_slot = var_isbs2_swg;
        *var_isbs2_swg_dn0_slot = var_isbs2_swg_dn0;
        *var_isbs2_swg_dn10_slot = var_isbs2_swg_dn10;
        *var_isbs2_swg_dn11_slot = var_isbs2_swg_dn11;
        *var_isbs2_swg_dn14_slot = var_isbs2_swg_dn14;
        *var_isbs2_swg_dn2_slot = var_isbs2_swg_dn2;
        *var_isbs2_swg_dn4_slot = var_isbs2_swg_dn4;
        *var_isbs2_swg_dn5_slot = var_isbs2_swg_dn5;
        *var_isbs2_swg_dn6_slot = var_isbs2_swg_dn6;
        *var_isbs2_swg_dn7_slot = var_isbs2_swg_dn7;
        *var_isbs2_swg_dn8_slot = var_isbs2_swg_dn8;
        *var_isbs2_swg_dn9_slot = var_isbs2_swg_dn9;
        *var_isbs2_swg_rv_slot = var_isbs2_swg_rv;
        *var_isbs2_sws_slot = var_isbs2_sws;
        *var_isbs2_sws_dn0_slot = var_isbs2_sws_dn0;
        *var_isbs2_sws_dn10_slot = var_isbs2_sws_dn10;
        *var_isbs2_sws_dn11_slot = var_isbs2_sws_dn11;
        *var_isbs2_sws_dn14_slot = var_isbs2_sws_dn14;
        *var_isbs2_sws_dn2_slot = var_isbs2_sws_dn2;
        *var_isbs2_sws_dn4_slot = var_isbs2_sws_dn4;
        *var_isbs2_sws_dn5_slot = var_isbs2_sws_dn5;
        *var_isbs2_sws_dn6_slot = var_isbs2_sws_dn6;
        *var_isbs2_sws_dn7_slot = var_isbs2_sws_dn7;
        *var_isbs2_sws_dn8_slot = var_isbs2_sws_dn8;
        *var_isbs2_sws_dn9_slot = var_isbs2_sws_dn9;
        *var_isbs2_sws_rv_slot = var_isbs2_sws_rv;
        *var_isbs_btm_slot = var_isbs_btm;
        *var_isbs_btm_dn0_slot = var_isbs_btm_dn0;
        *var_isbs_btm_dn10_slot = var_isbs_btm_dn10;
        *var_isbs_btm_dn11_slot = var_isbs_btm_dn11;
        *var_isbs_btm_dn14_slot = var_isbs_btm_dn14;
        *var_isbs_btm_dn2_slot = var_isbs_btm_dn2;
        *var_isbs_btm_dn4_slot = var_isbs_btm_dn4;
        *var_isbs_btm_dn5_slot = var_isbs_btm_dn5;
        *var_isbs_btm_dn6_slot = var_isbs_btm_dn6;
        *var_isbs_btm_dn7_slot = var_isbs_btm_dn7;
        *var_isbs_btm_dn8_slot = var_isbs_btm_dn8;
        *var_isbs_btm_dn9_slot = var_isbs_btm_dn9;
        *var_isbs_btm_rv_slot = var_isbs_btm_rv;
        *var_isbs_dn0_slot = var_isbs_dn0;
        *var_isbs_dn10_slot = var_isbs_dn10;
        *var_isbs_dn11_slot = var_isbs_dn11;
        *var_isbs_dn14_slot = var_isbs_dn14;
        *var_isbs_dn2_slot = var_isbs_dn2;
        *var_isbs_dn4_slot = var_isbs_dn4;
        *var_isbs_dn5_slot = var_isbs_dn5;
        *var_isbs_dn6_slot = var_isbs_dn6;
        *var_isbs_dn7_slot = var_isbs_dn7;
        *var_isbs_dn8_slot = var_isbs_dn8;
        *var_isbs_dn9_slot = var_isbs_dn9;
        *var_isbs_rv_slot = var_isbs_rv;
        *var_isbs_swg_slot = var_isbs_swg;
        *var_isbs_swg_dn0_slot = var_isbs_swg_dn0;
        *var_isbs_swg_dn10_slot = var_isbs_swg_dn10;
        *var_isbs_swg_dn11_slot = var_isbs_swg_dn11;
        *var_isbs_swg_dn14_slot = var_isbs_swg_dn14;
        *var_isbs_swg_dn2_slot = var_isbs_swg_dn2;
        *var_isbs_swg_dn4_slot = var_isbs_swg_dn4;
        *var_isbs_swg_dn5_slot = var_isbs_swg_dn5;
        *var_isbs_swg_dn6_slot = var_isbs_swg_dn6;
        *var_isbs_swg_dn7_slot = var_isbs_swg_dn7;
        *var_isbs_swg_dn8_slot = var_isbs_swg_dn8;
        *var_isbs_swg_dn9_slot = var_isbs_swg_dn9;
        *var_isbs_swg_rv_slot = var_isbs_swg_rv;
        *var_isbs_sws_slot = var_isbs_sws;
        *var_isbs_sws_dn0_slot = var_isbs_sws_dn0;
        *var_isbs_sws_dn10_slot = var_isbs_sws_dn10;
        *var_isbs_sws_dn11_slot = var_isbs_sws_dn11;
        *var_isbs_sws_dn14_slot = var_isbs_sws_dn14;
        *var_isbs_sws_dn2_slot = var_isbs_sws_dn2;
        *var_isbs_sws_dn4_slot = var_isbs_sws_dn4;
        *var_isbs_sws_dn5_slot = var_isbs_sws_dn5;
        *var_isbs_sws_dn6_slot = var_isbs_sws_dn6;
        *var_isbs_sws_dn7_slot = var_isbs_sws_dn7;
        *var_isbs_sws_dn8_slot = var_isbs_sws_dn8;
        *var_isbs_sws_dn9_slot = var_isbs_sws_dn9;
        *var_isbs_sws_rv_slot = var_isbs_sws_rv;
        *var_jd_expcd_slot = var_jd_expcd;
        *var_jd_expcd_dn0_slot = var_jd_expcd_dn0;
        *var_jd_expcd_dn10_slot = var_jd_expcd_dn10;
        *var_jd_expcd_dn11_slot = var_jd_expcd_dn11;
        *var_jd_expcd_dn14_slot = var_jd_expcd_dn14;
        *var_jd_expcd_dn2_slot = var_jd_expcd_dn2;
        *var_jd_expcd_dn4_slot = var_jd_expcd_dn4;
        *var_jd_expcd_dn5_slot = var_jd_expcd_dn5;
        *var_jd_expcd_dn6_slot = var_jd_expcd_dn6;
        *var_jd_expcd_dn7_slot = var_jd_expcd_dn7;
        *var_jd_expcd_dn8_slot = var_jd_expcd_dn8;
        *var_jd_expcd_dn9_slot = var_jd_expcd_dn9;
        *var_jd_expcd_rv_slot = var_jd_expcd_rv;
        *var_jd_expcs_slot = var_jd_expcs;
        *var_jd_expcs_dn0_slot = var_jd_expcs_dn0;
        *var_jd_expcs_dn10_slot = var_jd_expcs_dn10;
        *var_jd_expcs_dn11_slot = var_jd_expcs_dn11;
        *var_jd_expcs_dn14_slot = var_jd_expcs_dn14;
        *var_jd_expcs_dn2_slot = var_jd_expcs_dn2;
        *var_jd_expcs_dn4_slot = var_jd_expcs_dn4;
        *var_jd_expcs_dn5_slot = var_jd_expcs_dn5;
        *var_jd_expcs_dn6_slot = var_jd_expcs_dn6;
        *var_jd_expcs_dn7_slot = var_jd_expcs_dn7;
        *var_jd_expcs_dn8_slot = var_jd_expcs_dn8;
        *var_jd_expcs_dn9_slot = var_jd_expcs_dn9;
        *var_jd_expcs_rv_slot = var_jd_expcs_rv;
        *var_jd_nvtm_invd_slot = var_jd_nvtm_invd;
        *var_jd_nvtm_invd_dn0_slot = var_jd_nvtm_invd_dn0;
        *var_jd_nvtm_invd_dn10_slot = var_jd_nvtm_invd_dn10;
        *var_jd_nvtm_invd_dn11_slot = var_jd_nvtm_invd_dn11;
        *var_jd_nvtm_invd_dn14_slot = var_jd_nvtm_invd_dn14;
        *var_jd_nvtm_invd_dn2_slot = var_jd_nvtm_invd_dn2;
        *var_jd_nvtm_invd_dn4_slot = var_jd_nvtm_invd_dn4;
        *var_jd_nvtm_invd_dn5_slot = var_jd_nvtm_invd_dn5;
        *var_jd_nvtm_invd_dn6_slot = var_jd_nvtm_invd_dn6;
        *var_jd_nvtm_invd_dn7_slot = var_jd_nvtm_invd_dn7;
        *var_jd_nvtm_invd_dn8_slot = var_jd_nvtm_invd_dn8;
        *var_jd_nvtm_invd_dn9_slot = var_jd_nvtm_invd_dn9;
        *var_jd_nvtm_invd_rv_slot = var_jd_nvtm_invd_rv;
        *var_jd_nvtm_invs_slot = var_jd_nvtm_invs;
        *var_jd_nvtm_invs_dn0_slot = var_jd_nvtm_invs_dn0;
        *var_jd_nvtm_invs_dn10_slot = var_jd_nvtm_invs_dn10;
        *var_jd_nvtm_invs_dn11_slot = var_jd_nvtm_invs_dn11;
        *var_jd_nvtm_invs_dn14_slot = var_jd_nvtm_invs_dn14;
        *var_jd_nvtm_invs_dn2_slot = var_jd_nvtm_invs_dn2;
        *var_jd_nvtm_invs_dn4_slot = var_jd_nvtm_invs_dn4;
        *var_jd_nvtm_invs_dn5_slot = var_jd_nvtm_invs_dn5;
        *var_jd_nvtm_invs_dn6_slot = var_jd_nvtm_invs_dn6;
        *var_jd_nvtm_invs_dn7_slot = var_jd_nvtm_invs_dn7;
        *var_jd_nvtm_invs_dn8_slot = var_jd_nvtm_invs_dn8;
        *var_jd_nvtm_invs_dn9_slot = var_jd_nvtm_invs_dn9;
        *var_jd_nvtm_invs_rv_slot = var_jd_nvtm_invs_rv;
        *var_pzbssw_slot = var_pzbssw;
        *var_pzbssw_dn0_slot = var_pzbssw_dn0;
        *var_pzbssw_dn10_slot = var_pzbssw_dn10;
        *var_pzbssw_dn11_slot = var_pzbssw_dn11;
        *var_pzbssw_dn14_slot = var_pzbssw_dn14;
        *var_pzbssw_dn2_slot = var_pzbssw_dn2;
        *var_pzbssw_dn4_slot = var_pzbssw_dn4;
        *var_pzbssw_dn5_slot = var_pzbssw_dn5;
        *var_pzbssw_dn6_slot = var_pzbssw_dn6;
        *var_pzbssw_dn7_slot = var_pzbssw_dn7;
        *var_pzbssw_dn8_slot = var_pzbssw_dn8;
        *var_pzbssw_dn9_slot = var_pzbssw_dn9;
        *var_pzbssw_rv_slot = var_pzbssw_rv;
        *var_pzbsswg_slot = var_pzbsswg;
        *var_pzbsswg_dn0_slot = var_pzbsswg_dn0;
        *var_pzbsswg_dn10_slot = var_pzbsswg_dn10;
        *var_pzbsswg_dn11_slot = var_pzbsswg_dn11;
        *var_pzbsswg_dn14_slot = var_pzbsswg_dn14;
        *var_pzbsswg_dn2_slot = var_pzbsswg_dn2;
        *var_pzbsswg_dn4_slot = var_pzbsswg_dn4;
        *var_pzbsswg_dn5_slot = var_pzbsswg_dn5;
        *var_pzbsswg_dn6_slot = var_pzbsswg_dn6;
        *var_pzbsswg_dn7_slot = var_pzbsswg_dn7;
        *var_pzbsswg_dn8_slot = var_pzbsswg_dn8;
        *var_pzbsswg_dn9_slot = var_pzbsswg_dn9;
        *var_pzbsswg_rv_slot = var_pzbsswg_rv;
        *var_qbd_qs_slot = var_qbd_qs;
        *var_qbd_qs_dn0_slot = var_qbd_qs_dn0;
        *var_qbd_qs_dn10_slot = var_qbd_qs_dn10;
        *var_qbd_qs_dn11_slot = var_qbd_qs_dn11;
        *var_qbd_qs_dn14_slot = var_qbd_qs_dn14;
        *var_qbd_qs_dn2_slot = var_qbd_qs_dn2;
        *var_qbd_qs_dn4_slot = var_qbd_qs_dn4;
        *var_qbd_qs_dn5_slot = var_qbd_qs_dn5;
        *var_qbd_qs_dn6_slot = var_qbd_qs_dn6;
        *var_qbd_qs_dn7_slot = var_qbd_qs_dn7;
        *var_qbd_qs_dn8_slot = var_qbd_qs_dn8;
        *var_qbd_qs_dn9_slot = var_qbd_qs_dn9;
        *var_qbd_qs_rv_slot = var_qbd_qs_rv;
        *var_qbdld_add_slot = var_qbdld_add;
        *var_qbdld_add_dn0_slot = var_qbdld_add_dn0;
        *var_qbdld_add_dn10_slot = var_qbdld_add_dn10;
        *var_qbdld_add_dn11_slot = var_qbdld_add_dn11;
        *var_qbdld_add_dn14_slot = var_qbdld_add_dn14;
        *var_qbdld_add_dn2_slot = var_qbdld_add_dn2;
        *var_qbdld_add_dn4_slot = var_qbdld_add_dn4;
        *var_qbdld_add_dn5_slot = var_qbdld_add_dn5;
        *var_qbdld_add_dn6_slot = var_qbdld_add_dn6;
        *var_qbdld_add_dn7_slot = var_qbdld_add_dn7;
        *var_qbdld_add_dn8_slot = var_qbdld_add_dn8;
        *var_qbdld_add_dn9_slot = var_qbdld_add_dn9;
        *var_qbdld_add_rv_slot = var_qbdld_add_rv;
        *var_qbsld_add_slot = var_qbsld_add;
        *var_qbsld_add_dn0_slot = var_qbsld_add_dn0;
        *var_qbsld_add_dn10_slot = var_qbsld_add_dn10;
        *var_qbsld_add_dn11_slot = var_qbsld_add_dn11;
        *var_qbsld_add_dn14_slot = var_qbsld_add_dn14;
        *var_qbsld_add_dn2_slot = var_qbsld_add_dn2;
        *var_qbsld_add_dn4_slot = var_qbsld_add_dn4;
        *var_qbsld_add_dn5_slot = var_qbsld_add_dn5;
        *var_qbsld_add_dn6_slot = var_qbsld_add_dn6;
        *var_qbsld_add_dn7_slot = var_qbsld_add_dn7;
        *var_qbsld_add_dn8_slot = var_qbsld_add_dn8;
        *var_qbsld_add_dn9_slot = var_qbsld_add_dn9;
        *var_qbsld_add_rv_slot = var_qbsld_add_rv;
        *var_qovd_add_slot = var_qovd_add;
        *var_qovd_add_dn0_slot = var_qovd_add_dn0;
        *var_qovd_add_dn10_slot = var_qovd_add_dn10;
        *var_qovd_add_dn11_slot = var_qovd_add_dn11;
        *var_qovd_add_dn14_slot = var_qovd_add_dn14;
        *var_qovd_add_dn2_slot = var_qovd_add_dn2;
        *var_qovd_add_dn4_slot = var_qovd_add_dn4;
        *var_qovd_add_dn5_slot = var_qovd_add_dn5;
        *var_qovd_add_dn6_slot = var_qovd_add_dn6;
        *var_qovd_add_dn7_slot = var_qovd_add_dn7;
        *var_qovd_add_dn8_slot = var_qovd_add_dn8;
        *var_qovd_add_dn9_slot = var_qovd_add_dn9;
        *var_qovd_add_rv_slot = var_qovd_add_rv;
        *var_qovs_add_slot = var_qovs_add;
        *var_qovs_add_dn0_slot = var_qovs_add_dn0;
        *var_qovs_add_dn10_slot = var_qovs_add_dn10;
        *var_qovs_add_dn11_slot = var_qovs_add_dn11;
        *var_qovs_add_dn14_slot = var_qovs_add_dn14;
        *var_qovs_add_dn2_slot = var_qovs_add_dn2;
        *var_qovs_add_dn4_slot = var_qovs_add_dn4;
        *var_qovs_add_dn5_slot = var_qovs_add_dn5;
        *var_qovs_add_dn6_slot = var_qovs_add_dn6;
        *var_qovs_add_dn7_slot = var_qovs_add_dn7;
        *var_qovs_add_dn8_slot = var_qovs_add_dn8;
        *var_qovs_add_dn9_slot = var_qovs_add_dn9;
        *var_qovs_add_rv_slot = var_qovs_add_rv;
        *var_sarg_slot = var_sarg;
        *var_sarg_dn0_slot = var_sarg_dn0;
        *var_sarg_dn10_slot = var_sarg_dn10;
        *var_sarg_dn11_slot = var_sarg_dn11;
        *var_sarg_dn14_slot = var_sarg_dn14;
        *var_sarg_dn2_slot = var_sarg_dn2;
        *var_sarg_dn4_slot = var_sarg_dn4;
        *var_sarg_dn5_slot = var_sarg_dn5;
        *var_sarg_dn6_slot = var_sarg_dn6;
        *var_sarg_dn7_slot = var_sarg_dn7;
        *var_sarg_dn8_slot = var_sarg_dn8;
        *var_sarg_dn9_slot = var_sarg_dn9;
        *var_sarg_rv_slot = var_sarg_rv;
        *var_start_of_loopl_slot = var_start_of_loopl;
        *var_start_of_loopl_rv_slot = var_start_of_loopl_rv;
        *var_start_of_mobility_slot = var_start_of_mobility;
        *var_start_of_mobility_rv_slot = var_start_of_mobility_rv;
        *var_vbd_jct_slot = var_vbd_jct;
        *var_vbd_jct_dn0_slot = var_vbd_jct_dn0;
        *var_vbd_jct_dn10_slot = var_vbd_jct_dn10;
        *var_vbd_jct_rv_slot = var_vbd_jct_rv;
        *var_vbdi_jct_slot = var_vbdi_jct;
        *var_vbdi_jct_dn6_slot = var_vbdi_jct_dn6;
        *var_vbdi_jct_dn9_slot = var_vbdi_jct_dn9;
        *var_vbdi_jct_rv_slot = var_vbdi_jct_rv;
        *var_vbdt_slot = var_vbdt;
        *var_vbdt_dn0_slot = var_vbdt_dn0;
        *var_vbdt_dn10_slot = var_vbdt_dn10;
        *var_vbdt_dn11_slot = var_vbdt_dn11;
        *var_vbdt_dn14_slot = var_vbdt_dn14;
        *var_vbdt_dn2_slot = var_vbdt_dn2;
        *var_vbdt_dn4_slot = var_vbdt_dn4;
        *var_vbdt_dn5_slot = var_vbdt_dn5;
        *var_vbdt_dn6_slot = var_vbdt_dn6;
        *var_vbdt_dn7_slot = var_vbdt_dn7;
        *var_vbdt_dn8_slot = var_vbdt_dn8;
        *var_vbdt_dn9_slot = var_vbdt_dn9;
        *var_vbdt_rv_slot = var_vbdt_rv;
        *var_vbpdp_slot = var_vbpdp;
        *var_vbpdp_dn6_slot = var_vbpdp_dn6;
        *var_vbpdp_dn9_slot = var_vbpdp_dn9;
        *var_vbpdp_rv_slot = var_vbpdp_rv;
        *var_vbpsp_slot = var_vbpsp;
        *var_vbpsp_dn8_slot = var_vbpsp_dn8;
        *var_vbpsp_dn9_slot = var_vbpsp_dn9;
        *var_vbpsp_rv_slot = var_vbpsp_rv;
        *var_vbs_jct_slot = var_vbs_jct;
        *var_vbs_jct_dn11_slot = var_vbs_jct_dn11;
        *var_vbs_jct_dn2_slot = var_vbs_jct_dn2;
        *var_vbs_jct_rv_slot = var_vbs_jct_rv;
        *var_vbsi_jct_slot = var_vbsi_jct;
        *var_vbsi_jct_dn8_slot = var_vbsi_jct_dn8;
        *var_vbsi_jct_dn9_slot = var_vbsi_jct_dn9;
        *var_vbsi_jct_rv_slot = var_vbsi_jct_rv;
        *var_vbst_slot = var_vbst;
        *var_vbst_dn0_slot = var_vbst_dn0;
        *var_vbst_dn10_slot = var_vbst_dn10;
        *var_vbst_dn11_slot = var_vbst_dn11;
        *var_vbst_dn14_slot = var_vbst_dn14;
        *var_vbst_dn2_slot = var_vbst_dn2;
        *var_vbst_dn4_slot = var_vbst_dn4;
        *var_vbst_dn5_slot = var_vbst_dn5;
        *var_vbst_dn6_slot = var_vbst_dn6;
        *var_vbst_dn7_slot = var_vbst_dn7;
        *var_vbst_dn8_slot = var_vbst_dn8;
        *var_vbst_dn9_slot = var_vbst_dn9;
        *var_vbst_rv_slot = var_vbst_rv;
        *var_vdbd_slot = var_vdbd;
        *var_vdbd_dn0_slot = var_vdbd_dn0;
        *var_vdbd_dn10_slot = var_vdbd_dn10;
        *var_vdbd_rv_slot = var_vdbd_rv;
        *var_vsbs_slot = var_vsbs;
        *var_vsbs_dn11_slot = var_vsbs_dn11;
        *var_vsbs_dn2_slot = var_vsbs_dn2;
        *var_vsbs_rv_slot = var_vsbs_rv;
        *var_wjuncld_slot = var_wjuncld;
        *var_wjuncld_dn0_slot = var_wjuncld_dn0;
        *var_wjuncld_dn10_slot = var_wjuncld_dn10;
        *var_wjuncld_dn11_slot = var_wjuncld_dn11;
        *var_wjuncld_dn14_slot = var_wjuncld_dn14;
        *var_wjuncld_dn2_slot = var_wjuncld_dn2;
        *var_wjuncld_dn4_slot = var_wjuncld_dn4;
        *var_wjuncld_dn5_slot = var_wjuncld_dn5;
        *var_wjuncld_dn6_slot = var_wjuncld_dn6;
        *var_wjuncld_dn7_slot = var_wjuncld_dn7;
        *var_wjuncld_dn8_slot = var_wjuncld_dn8;
        *var_wjuncld_dn9_slot = var_wjuncld_dn9;
        *var_wjuncld_rv_slot = var_wjuncld_rv;
    }

    pub(super) fn stamp_reactive_block_11(
        p: &Parameters,
        var_cox0_func_slot: &mut f64,
        var_cox0_func_rv_slot: &mut f64,
        var_guard10_slot: &mut f64,
        var_guard10_rv_slot: &mut f64,
        var_guard11_slot: &mut f64,
        var_guard112_slot: &mut f64,
        var_guard112_rv_slot: &mut f64,
        var_guard115_slot: &mut f64,
        var_guard115_rv_slot: &mut f64,
        var_guard116_slot: &mut f64,
        var_guard116_rv_slot: &mut f64,
        var_guard119_slot: &mut f64,
        var_guard119_rv_slot: &mut f64,
        var_guard11_rv_slot: &mut f64,
        var_guard120_slot: &mut f64,
        var_guard120_rv_slot: &mut f64,
        var_guard13_slot: &mut f64,
        var_guard13_rv_slot: &mut f64,
        var_guard16_slot: &mut f64,
        var_guard16_rv_slot: &mut f64,
        var_guard17_slot: &mut f64,
        var_guard17_rv_slot: &mut f64,
        var_idspt1_slot: &mut f64,
        var_idspt1_dn0_slot: &mut f64,
        var_idspt1_dn10_slot: &mut f64,
        var_idspt1_dn11_slot: &mut f64,
        var_idspt1_dn14_slot: &mut f64,
        var_idspt1_dn2_slot: &mut f64,
        var_idspt1_dn4_slot: &mut f64,
        var_idspt1_dn5_slot: &mut f64,
        var_idspt1_dn6_slot: &mut f64,
        var_idspt1_dn7_slot: &mut f64,
        var_idspt1_dn8_slot: &mut f64,
        var_idspt1_dn9_slot: &mut f64,
        var_idspt1_rv_slot: &mut f64,
        var_inqs0_a_slot: &mut f64,
        var_inqs0_a_dn0_slot: &mut f64,
        var_inqs0_a_dn10_slot: &mut f64,
        var_inqs0_a_dn11_slot: &mut f64,
        var_inqs0_a_dn14_slot: &mut f64,
        var_inqs0_a_dn16_slot: &mut f64,
        var_inqs0_a_dn2_slot: &mut f64,
        var_inqs0_a_dn4_slot: &mut f64,
        var_inqs0_a_dn5_slot: &mut f64,
        var_inqs0_a_dn6_slot: &mut f64,
        var_inqs0_a_dn7_slot: &mut f64,
        var_inqs0_a_dn8_slot: &mut f64,
        var_inqs0_a_dn9_slot: &mut f64,
        var_inqs0_a_rv_slot: &mut f64,
        var_inqs0_k_slot: &mut f64,
        var_inqs0_k_dn0_slot: &mut f64,
        var_inqs0_k_dn10_slot: &mut f64,
        var_inqs0_k_dn11_slot: &mut f64,
        var_inqs0_k_dn14_slot: &mut f64,
        var_inqs0_k_dn17_slot: &mut f64,
        var_inqs0_k_dn2_slot: &mut f64,
        var_inqs0_k_dn4_slot: &mut f64,
        var_inqs0_k_dn5_slot: &mut f64,
        var_inqs0_k_dn6_slot: &mut f64,
        var_inqs0_k_dn7_slot: &mut f64,
        var_inqs0_k_dn8_slot: &mut f64,
        var_inqs0_k_dn9_slot: &mut f64,
        var_inqs0_k_rv_slot: &mut f64,
        var_isubibpc_slot: &mut f64,
        var_isubibpc_dn0_slot: &mut f64,
        var_isubibpc_dn10_slot: &mut f64,
        var_isubibpc_dn11_slot: &mut f64,
        var_isubibpc_dn14_slot: &mut f64,
        var_isubibpc_dn2_slot: &mut f64,
        var_isubibpc_dn4_slot: &mut f64,
        var_isubibpc_dn5_slot: &mut f64,
        var_isubibpc_dn6_slot: &mut f64,
        var_isubibpc_dn7_slot: &mut f64,
        var_isubibpc_dn8_slot: &mut f64,
        var_isubibpc_dn9_slot: &mut f64,
        var_isubibpc_rv_slot: &mut f64,
        var_iwnqs0_a_slot: &mut f64,
        var_iwnqs0_a_dn0_slot: &mut f64,
        var_iwnqs0_a_dn10_slot: &mut f64,
        var_iwnqs0_a_dn11_slot: &mut f64,
        var_iwnqs0_a_dn14_slot: &mut f64,
        var_iwnqs0_a_dn18_slot: &mut f64,
        var_iwnqs0_a_dn2_slot: &mut f64,
        var_iwnqs0_a_dn4_slot: &mut f64,
        var_iwnqs0_a_dn5_slot: &mut f64,
        var_iwnqs0_a_dn6_slot: &mut f64,
        var_iwnqs0_a_dn7_slot: &mut f64,
        var_iwnqs0_a_dn8_slot: &mut f64,
        var_iwnqs0_a_dn9_slot: &mut f64,
        var_iwnqs0_a_rv_slot: &mut f64,
        var_lover_func_slot: &mut f64,
        var_lover_func_dn0_slot: &mut f64,
        var_lover_func_dn10_slot: &mut f64,
        var_lover_func_dn11_slot: &mut f64,
        var_lover_func_dn14_slot: &mut f64,
        var_lover_func_dn2_slot: &mut f64,
        var_lover_func_dn4_slot: &mut f64,
        var_lover_func_dn5_slot: &mut f64,
        var_lover_func_dn6_slot: &mut f64,
        var_lover_func_dn7_slot: &mut f64,
        var_lover_func_dn8_slot: &mut f64,
        var_lover_func_dn9_slot: &mut f64,
        var_lover_func_rv_slot: &mut f64,
        var_mfactor_slot: &mut f64,
        var_mfactor_rv_slot: &mut f64,
        var_q_nqs_a_slot: &mut f64,
        var_q_nqs_a_dn16_slot: &mut f64,
        var_q_nqs_a_rv_slot: &mut f64,
        var_q_nqs_k_slot: &mut f64,
        var_q_nqs_k_dn17_slot: &mut f64,
        var_q_nqs_k_rv_slot: &mut f64,
        var_uc_codep_slot: &mut f64,
        var_uc_codep_rv_slot: &mut f64,
        var_uc_corsrd_slot: &mut f64,
        var_uc_corsrd_rv_slot: &mut f64,
        var_uc_depleak_slot: &mut f64,
        var_uc_depleak_dn0_slot: &mut f64,
        var_uc_depleak_dn10_slot: &mut f64,
        var_uc_depleak_dn11_slot: &mut f64,
        var_uc_depleak_dn14_slot: &mut f64,
        var_uc_depleak_dn2_slot: &mut f64,
        var_uc_depleak_dn4_slot: &mut f64,
        var_uc_depleak_dn5_slot: &mut f64,
        var_uc_depleak_dn6_slot: &mut f64,
        var_uc_depleak_dn7_slot: &mut f64,
        var_uc_depleak_dn8_slot: &mut f64,
        var_uc_depleak_dn9_slot: &mut f64,
        var_uc_depleak_rv_slot: &mut f64,
        var_uc_depmue0_slot: &mut f64,
        var_uc_depmue0_dn0_slot: &mut f64,
        var_uc_depmue0_dn10_slot: &mut f64,
        var_uc_depmue0_dn11_slot: &mut f64,
        var_uc_depmue0_dn14_slot: &mut f64,
        var_uc_depmue0_dn2_slot: &mut f64,
        var_uc_depmue0_dn4_slot: &mut f64,
        var_uc_depmue0_dn5_slot: &mut f64,
        var_uc_depmue0_dn6_slot: &mut f64,
        var_uc_depmue0_dn7_slot: &mut f64,
        var_uc_depmue0_dn8_slot: &mut f64,
        var_uc_depmue0_dn9_slot: &mut f64,
        var_uc_depmue0_rv_slot: &mut f64,
        var_uc_depmue1_slot: &mut f64,
        var_uc_depmue1_dn0_slot: &mut f64,
        var_uc_depmue1_dn10_slot: &mut f64,
        var_uc_depmue1_dn11_slot: &mut f64,
        var_uc_depmue1_dn14_slot: &mut f64,
        var_uc_depmue1_dn2_slot: &mut f64,
        var_uc_depmue1_dn4_slot: &mut f64,
        var_uc_depmue1_dn5_slot: &mut f64,
        var_uc_depmue1_dn6_slot: &mut f64,
        var_uc_depmue1_dn7_slot: &mut f64,
        var_uc_depmue1_dn8_slot: &mut f64,
        var_uc_depmue1_dn9_slot: &mut f64,
        var_uc_depmue1_rv_slot: &mut f64,
        var_uc_depmue2_slot: &mut f64,
        var_uc_depmue2_dn0_slot: &mut f64,
        var_uc_depmue2_dn10_slot: &mut f64,
        var_uc_depmue2_dn11_slot: &mut f64,
        var_uc_depmue2_dn14_slot: &mut f64,
        var_uc_depmue2_dn2_slot: &mut f64,
        var_uc_depmue2_dn4_slot: &mut f64,
        var_uc_depmue2_dn5_slot: &mut f64,
        var_uc_depmue2_dn6_slot: &mut f64,
        var_uc_depmue2_dn7_slot: &mut f64,
        var_uc_depmue2_dn8_slot: &mut f64,
        var_uc_depmue2_dn9_slot: &mut f64,
        var_uc_depmue2_rv_slot: &mut f64,
        var_uc_depmueback0_slot: &mut f64,
        var_uc_depmueback0_dn0_slot: &mut f64,
        var_uc_depmueback0_dn10_slot: &mut f64,
        var_uc_depmueback0_dn11_slot: &mut f64,
        var_uc_depmueback0_dn14_slot: &mut f64,
        var_uc_depmueback0_dn2_slot: &mut f64,
        var_uc_depmueback0_dn4_slot: &mut f64,
        var_uc_depmueback0_dn5_slot: &mut f64,
        var_uc_depmueback0_dn6_slot: &mut f64,
        var_uc_depmueback0_dn7_slot: &mut f64,
        var_uc_depmueback0_dn8_slot: &mut f64,
        var_uc_depmueback0_dn9_slot: &mut f64,
        var_uc_depmueback0_rv_slot: &mut f64,
        var_uc_depmueback1_slot: &mut f64,
        var_uc_depmueback1_dn0_slot: &mut f64,
        var_uc_depmueback1_dn10_slot: &mut f64,
        var_uc_depmueback1_dn11_slot: &mut f64,
        var_uc_depmueback1_dn14_slot: &mut f64,
        var_uc_depmueback1_dn2_slot: &mut f64,
        var_uc_depmueback1_dn4_slot: &mut f64,
        var_uc_depmueback1_dn5_slot: &mut f64,
        var_uc_depmueback1_dn6_slot: &mut f64,
        var_uc_depmueback1_dn7_slot: &mut f64,
        var_uc_depmueback1_dn8_slot: &mut f64,
        var_uc_depmueback1_dn9_slot: &mut f64,
        var_uc_depmueback1_rv_slot: &mut f64,
        var_uc_depmueph1_slot: &mut f64,
        var_uc_depmueph1_rv_slot: &mut f64,
        var_uc_depthn_slot: &mut f64,
        var_uc_depthn_dn0_slot: &mut f64,
        var_uc_depthn_dn10_slot: &mut f64,
        var_uc_depthn_dn11_slot: &mut f64,
        var_uc_depthn_dn14_slot: &mut f64,
        var_uc_depthn_dn2_slot: &mut f64,
        var_uc_depthn_dn4_slot: &mut f64,
        var_uc_depthn_dn5_slot: &mut f64,
        var_uc_depthn_dn6_slot: &mut f64,
        var_uc_depthn_dn7_slot: &mut f64,
        var_uc_depthn_dn8_slot: &mut f64,
        var_uc_depthn_dn9_slot: &mut f64,
        var_uc_depthn_rv_slot: &mut f64,
        var_uc_depvdsef1_slot: &mut f64,
        var_uc_depvdsef1_dn0_slot: &mut f64,
        var_uc_depvdsef1_dn10_slot: &mut f64,
        var_uc_depvdsef1_dn11_slot: &mut f64,
        var_uc_depvdsef1_dn14_slot: &mut f64,
        var_uc_depvdsef1_dn2_slot: &mut f64,
        var_uc_depvdsef1_dn4_slot: &mut f64,
        var_uc_depvdsef1_dn5_slot: &mut f64,
        var_uc_depvdsef1_dn6_slot: &mut f64,
        var_uc_depvdsef1_dn7_slot: &mut f64,
        var_uc_depvdsef1_dn8_slot: &mut f64,
        var_uc_depvdsef1_dn9_slot: &mut f64,
        var_uc_depvdsef1_rv_slot: &mut f64,
        var_uc_depvdsef2_slot: &mut f64,
        var_uc_depvdsef2_dn0_slot: &mut f64,
        var_uc_depvdsef2_dn10_slot: &mut f64,
        var_uc_depvdsef2_dn11_slot: &mut f64,
        var_uc_depvdsef2_dn14_slot: &mut f64,
        var_uc_depvdsef2_dn2_slot: &mut f64,
        var_uc_depvdsef2_dn4_slot: &mut f64,
        var_uc_depvdsef2_dn5_slot: &mut f64,
        var_uc_depvdsef2_dn6_slot: &mut f64,
        var_uc_depvdsef2_dn7_slot: &mut f64,
        var_uc_depvdsef2_dn8_slot: &mut f64,
        var_uc_depvdsef2_dn9_slot: &mut f64,
        var_uc_depvdsef2_rv_slot: &mut f64,
        var_uc_depvmax_slot: &mut f64,
        var_uc_depvmax_dn0_slot: &mut f64,
        var_uc_depvmax_dn10_slot: &mut f64,
        var_uc_depvmax_dn11_slot: &mut f64,
        var_uc_depvmax_dn14_slot: &mut f64,
        var_uc_depvmax_dn2_slot: &mut f64,
        var_uc_depvmax_dn4_slot: &mut f64,
        var_uc_depvmax_dn5_slot: &mut f64,
        var_uc_depvmax_dn6_slot: &mut f64,
        var_uc_depvmax_dn7_slot: &mut f64,
        var_uc_depvmax_dn8_slot: &mut f64,
        var_uc_depvmax_dn9_slot: &mut f64,
        var_uc_depvmax_rv_slot: &mut f64,
        var_uc_depwlp_slot: &mut f64,
        var_uc_depwlp_dn0_slot: &mut f64,
        var_uc_depwlp_dn10_slot: &mut f64,
        var_uc_depwlp_dn11_slot: &mut f64,
        var_uc_depwlp_dn14_slot: &mut f64,
        var_uc_depwlp_dn2_slot: &mut f64,
        var_uc_depwlp_dn4_slot: &mut f64,
        var_uc_depwlp_dn5_slot: &mut f64,
        var_uc_depwlp_dn6_slot: &mut f64,
        var_uc_depwlp_dn7_slot: &mut f64,
        var_uc_depwlp_dn8_slot: &mut f64,
        var_uc_depwlp_dn9_slot: &mut f64,
        var_uc_depwlp_rv_slot: &mut f64,
        var_uc_ndepm_slot: &mut f64,
        var_uc_ndepm_dn0_slot: &mut f64,
        var_uc_ndepm_dn10_slot: &mut f64,
        var_uc_ndepm_dn11_slot: &mut f64,
        var_uc_ndepm_dn14_slot: &mut f64,
        var_uc_ndepm_dn2_slot: &mut f64,
        var_uc_ndepm_dn4_slot: &mut f64,
        var_uc_ndepm_dn5_slot: &mut f64,
        var_uc_ndepm_dn6_slot: &mut f64,
        var_uc_ndepm_dn7_slot: &mut f64,
        var_uc_ndepm_dn8_slot: &mut f64,
        var_uc_ndepm_dn9_slot: &mut f64,
        var_uc_ndepm_rv_slot: &mut f64,
        var_uc_rdrcx_slot: &mut f64,
        var_uc_rdrcx_rv_slot: &mut f64,
        var_uc_scp22_slot: &mut f64,
        var_uc_scp22_rv_slot: &mut f64,
        var_uc_xldld_slot: &mut f64,
        var_uc_xldld_rv_slot: &mut f64,
        var_uc_xpdv_slot: &mut f64,
        var_uc_xpdv_rv_slot: &mut f64,
        var_w_nqs_a_slot: &mut f64,
        var_w_nqs_a_dn18_slot: &mut f64,
        var_w_nqs_a_rv_slot: &mut f64,
        var_w_res_slot: &mut f64,
        var_w_res_dn0_slot: &mut f64,
        var_w_res_dn10_slot: &mut f64,
        var_w_res_dn11_slot: &mut f64,
        var_w_res_dn14_slot: &mut f64,
        var_w_res_dn2_slot: &mut f64,
        var_w_res_dn4_slot: &mut f64,
        var_w_res_dn5_slot: &mut f64,
        var_w_res_dn6_slot: &mut f64,
        var_w_res_dn7_slot: &mut f64,
        var_w_res_dn8_slot: &mut f64,
        var_w_res_dn9_slot: &mut f64,
        var_w_res_rv_slot: &mut f64,
        var_wdep_func_slot: &mut f64,
        var_wdep_func_dn0_slot: &mut f64,
        var_wdep_func_dn10_slot: &mut f64,
        var_wdep_func_dn11_slot: &mut f64,
        var_wdep_func_dn14_slot: &mut f64,
        var_wdep_func_dn2_slot: &mut f64,
        var_wdep_func_dn4_slot: &mut f64,
        var_wdep_func_dn5_slot: &mut f64,
        var_wdep_func_dn6_slot: &mut f64,
        var_wdep_func_dn7_slot: &mut f64,
        var_wdep_func_dn8_slot: &mut f64,
        var_wdep_func_dn9_slot: &mut f64,
        var_wdep_func_rv_slot: &mut f64,
        var_wk_ii_slot: &mut f64,
        var_wk_ii_dn0_slot: &mut f64,
        var_wk_ii_dn10_slot: &mut f64,
        var_wk_ii_dn11_slot: &mut f64,
        var_wk_ii_dn14_slot: &mut f64,
        var_wk_ii_dn2_slot: &mut f64,
        var_wk_ii_dn4_slot: &mut f64,
        var_wk_ii_dn5_slot: &mut f64,
        var_wk_ii_dn6_slot: &mut f64,
        var_wk_ii_dn7_slot: &mut f64,
        var_wk_ii_dn8_slot: &mut f64,
        var_wk_ii_dn9_slot: &mut f64,
        var_wk_ii_rv_slot: &mut f64,
    ) {
        let mut var_cox0_func: f64 = *var_cox0_func_slot;
        let mut var_cox0_func_rv: f64 = *var_cox0_func_rv_slot;
        let mut var_guard10: f64 = *var_guard10_slot;
        let mut var_guard10_rv: f64 = *var_guard10_rv_slot;
        let mut var_guard11: f64 = *var_guard11_slot;
        let mut var_guard112: f64 = *var_guard112_slot;
        let mut var_guard112_rv: f64 = *var_guard112_rv_slot;
        let mut var_guard115: f64 = *var_guard115_slot;
        let mut var_guard115_rv: f64 = *var_guard115_rv_slot;
        let mut var_guard116: f64 = *var_guard116_slot;
        let mut var_guard116_rv: f64 = *var_guard116_rv_slot;
        let mut var_guard119: f64 = *var_guard119_slot;
        let mut var_guard119_rv: f64 = *var_guard119_rv_slot;
        let mut var_guard11_rv: f64 = *var_guard11_rv_slot;
        let mut var_guard120: f64 = *var_guard120_slot;
        let mut var_guard120_rv: f64 = *var_guard120_rv_slot;
        let mut var_guard13: f64 = *var_guard13_slot;
        let mut var_guard13_rv: f64 = *var_guard13_rv_slot;
        let mut var_guard16: f64 = *var_guard16_slot;
        let mut var_guard16_rv: f64 = *var_guard16_rv_slot;
        let mut var_guard17: f64 = *var_guard17_slot;
        let mut var_guard17_rv: f64 = *var_guard17_rv_slot;
        let mut var_idspt1: f64 = *var_idspt1_slot;
        let mut var_idspt1_dn0: f64 = *var_idspt1_dn0_slot;
        let mut var_idspt1_dn10: f64 = *var_idspt1_dn10_slot;
        let mut var_idspt1_dn11: f64 = *var_idspt1_dn11_slot;
        let mut var_idspt1_dn14: f64 = *var_idspt1_dn14_slot;
        let mut var_idspt1_dn2: f64 = *var_idspt1_dn2_slot;
        let mut var_idspt1_dn4: f64 = *var_idspt1_dn4_slot;
        let mut var_idspt1_dn5: f64 = *var_idspt1_dn5_slot;
        let mut var_idspt1_dn6: f64 = *var_idspt1_dn6_slot;
        let mut var_idspt1_dn7: f64 = *var_idspt1_dn7_slot;
        let mut var_idspt1_dn8: f64 = *var_idspt1_dn8_slot;
        let mut var_idspt1_dn9: f64 = *var_idspt1_dn9_slot;
        let mut var_idspt1_rv: f64 = *var_idspt1_rv_slot;
        let mut var_inqs0_a: f64 = *var_inqs0_a_slot;
        let mut var_inqs0_a_dn0: f64 = *var_inqs0_a_dn0_slot;
        let mut var_inqs0_a_dn10: f64 = *var_inqs0_a_dn10_slot;
        let mut var_inqs0_a_dn11: f64 = *var_inqs0_a_dn11_slot;
        let mut var_inqs0_a_dn14: f64 = *var_inqs0_a_dn14_slot;
        let mut var_inqs0_a_dn16: f64 = *var_inqs0_a_dn16_slot;
        let mut var_inqs0_a_dn2: f64 = *var_inqs0_a_dn2_slot;
        let mut var_inqs0_a_dn4: f64 = *var_inqs0_a_dn4_slot;
        let mut var_inqs0_a_dn5: f64 = *var_inqs0_a_dn5_slot;
        let mut var_inqs0_a_dn6: f64 = *var_inqs0_a_dn6_slot;
        let mut var_inqs0_a_dn7: f64 = *var_inqs0_a_dn7_slot;
        let mut var_inqs0_a_dn8: f64 = *var_inqs0_a_dn8_slot;
        let mut var_inqs0_a_dn9: f64 = *var_inqs0_a_dn9_slot;
        let mut var_inqs0_a_rv: f64 = *var_inqs0_a_rv_slot;
        let mut var_inqs0_k: f64 = *var_inqs0_k_slot;
        let mut var_inqs0_k_dn0: f64 = *var_inqs0_k_dn0_slot;
        let mut var_inqs0_k_dn10: f64 = *var_inqs0_k_dn10_slot;
        let mut var_inqs0_k_dn11: f64 = *var_inqs0_k_dn11_slot;
        let mut var_inqs0_k_dn14: f64 = *var_inqs0_k_dn14_slot;
        let mut var_inqs0_k_dn17: f64 = *var_inqs0_k_dn17_slot;
        let mut var_inqs0_k_dn2: f64 = *var_inqs0_k_dn2_slot;
        let mut var_inqs0_k_dn4: f64 = *var_inqs0_k_dn4_slot;
        let mut var_inqs0_k_dn5: f64 = *var_inqs0_k_dn5_slot;
        let mut var_inqs0_k_dn6: f64 = *var_inqs0_k_dn6_slot;
        let mut var_inqs0_k_dn7: f64 = *var_inqs0_k_dn7_slot;
        let mut var_inqs0_k_dn8: f64 = *var_inqs0_k_dn8_slot;
        let mut var_inqs0_k_dn9: f64 = *var_inqs0_k_dn9_slot;
        let mut var_inqs0_k_rv: f64 = *var_inqs0_k_rv_slot;
        let mut var_isubibpc: f64 = *var_isubibpc_slot;
        let mut var_isubibpc_dn0: f64 = *var_isubibpc_dn0_slot;
        let mut var_isubibpc_dn10: f64 = *var_isubibpc_dn10_slot;
        let mut var_isubibpc_dn11: f64 = *var_isubibpc_dn11_slot;
        let mut var_isubibpc_dn14: f64 = *var_isubibpc_dn14_slot;
        let mut var_isubibpc_dn2: f64 = *var_isubibpc_dn2_slot;
        let mut var_isubibpc_dn4: f64 = *var_isubibpc_dn4_slot;
        let mut var_isubibpc_dn5: f64 = *var_isubibpc_dn5_slot;
        let mut var_isubibpc_dn6: f64 = *var_isubibpc_dn6_slot;
        let mut var_isubibpc_dn7: f64 = *var_isubibpc_dn7_slot;
        let mut var_isubibpc_dn8: f64 = *var_isubibpc_dn8_slot;
        let mut var_isubibpc_dn9: f64 = *var_isubibpc_dn9_slot;
        let mut var_isubibpc_rv: f64 = *var_isubibpc_rv_slot;
        let mut var_iwnqs0_a: f64 = *var_iwnqs0_a_slot;
        let mut var_iwnqs0_a_dn0: f64 = *var_iwnqs0_a_dn0_slot;
        let mut var_iwnqs0_a_dn10: f64 = *var_iwnqs0_a_dn10_slot;
        let mut var_iwnqs0_a_dn11: f64 = *var_iwnqs0_a_dn11_slot;
        let mut var_iwnqs0_a_dn14: f64 = *var_iwnqs0_a_dn14_slot;
        let mut var_iwnqs0_a_dn18: f64 = *var_iwnqs0_a_dn18_slot;
        let mut var_iwnqs0_a_dn2: f64 = *var_iwnqs0_a_dn2_slot;
        let mut var_iwnqs0_a_dn4: f64 = *var_iwnqs0_a_dn4_slot;
        let mut var_iwnqs0_a_dn5: f64 = *var_iwnqs0_a_dn5_slot;
        let mut var_iwnqs0_a_dn6: f64 = *var_iwnqs0_a_dn6_slot;
        let mut var_iwnqs0_a_dn7: f64 = *var_iwnqs0_a_dn7_slot;
        let mut var_iwnqs0_a_dn8: f64 = *var_iwnqs0_a_dn8_slot;
        let mut var_iwnqs0_a_dn9: f64 = *var_iwnqs0_a_dn9_slot;
        let mut var_iwnqs0_a_rv: f64 = *var_iwnqs0_a_rv_slot;
        let mut var_lover_func: f64 = *var_lover_func_slot;
        let mut var_lover_func_dn0: f64 = *var_lover_func_dn0_slot;
        let mut var_lover_func_dn10: f64 = *var_lover_func_dn10_slot;
        let mut var_lover_func_dn11: f64 = *var_lover_func_dn11_slot;
        let mut var_lover_func_dn14: f64 = *var_lover_func_dn14_slot;
        let mut var_lover_func_dn2: f64 = *var_lover_func_dn2_slot;
        let mut var_lover_func_dn4: f64 = *var_lover_func_dn4_slot;
        let mut var_lover_func_dn5: f64 = *var_lover_func_dn5_slot;
        let mut var_lover_func_dn6: f64 = *var_lover_func_dn6_slot;
        let mut var_lover_func_dn7: f64 = *var_lover_func_dn7_slot;
        let mut var_lover_func_dn8: f64 = *var_lover_func_dn8_slot;
        let mut var_lover_func_dn9: f64 = *var_lover_func_dn9_slot;
        let mut var_lover_func_rv: f64 = *var_lover_func_rv_slot;
        let mut var_mfactor: f64 = *var_mfactor_slot;
        let mut var_mfactor_rv: f64 = *var_mfactor_rv_slot;
        let mut var_q_nqs_a: f64 = *var_q_nqs_a_slot;
        let mut var_q_nqs_a_dn16: f64 = *var_q_nqs_a_dn16_slot;
        let mut var_q_nqs_a_rv: f64 = *var_q_nqs_a_rv_slot;
        let mut var_q_nqs_k: f64 = *var_q_nqs_k_slot;
        let mut var_q_nqs_k_dn17: f64 = *var_q_nqs_k_dn17_slot;
        let mut var_q_nqs_k_rv: f64 = *var_q_nqs_k_rv_slot;
        let mut var_uc_codep: f64 = *var_uc_codep_slot;
        let mut var_uc_codep_rv: f64 = *var_uc_codep_rv_slot;
        let mut var_uc_corsrd: f64 = *var_uc_corsrd_slot;
        let mut var_uc_corsrd_rv: f64 = *var_uc_corsrd_rv_slot;
        let mut var_uc_depleak: f64 = *var_uc_depleak_slot;
        let mut var_uc_depleak_dn0: f64 = *var_uc_depleak_dn0_slot;
        let mut var_uc_depleak_dn10: f64 = *var_uc_depleak_dn10_slot;
        let mut var_uc_depleak_dn11: f64 = *var_uc_depleak_dn11_slot;
        let mut var_uc_depleak_dn14: f64 = *var_uc_depleak_dn14_slot;
        let mut var_uc_depleak_dn2: f64 = *var_uc_depleak_dn2_slot;
        let mut var_uc_depleak_dn4: f64 = *var_uc_depleak_dn4_slot;
        let mut var_uc_depleak_dn5: f64 = *var_uc_depleak_dn5_slot;
        let mut var_uc_depleak_dn6: f64 = *var_uc_depleak_dn6_slot;
        let mut var_uc_depleak_dn7: f64 = *var_uc_depleak_dn7_slot;
        let mut var_uc_depleak_dn8: f64 = *var_uc_depleak_dn8_slot;
        let mut var_uc_depleak_dn9: f64 = *var_uc_depleak_dn9_slot;
        let mut var_uc_depleak_rv: f64 = *var_uc_depleak_rv_slot;
        let mut var_uc_depmue0: f64 = *var_uc_depmue0_slot;
        let mut var_uc_depmue0_dn0: f64 = *var_uc_depmue0_dn0_slot;
        let mut var_uc_depmue0_dn10: f64 = *var_uc_depmue0_dn10_slot;
        let mut var_uc_depmue0_dn11: f64 = *var_uc_depmue0_dn11_slot;
        let mut var_uc_depmue0_dn14: f64 = *var_uc_depmue0_dn14_slot;
        let mut var_uc_depmue0_dn2: f64 = *var_uc_depmue0_dn2_slot;
        let mut var_uc_depmue0_dn4: f64 = *var_uc_depmue0_dn4_slot;
        let mut var_uc_depmue0_dn5: f64 = *var_uc_depmue0_dn5_slot;
        let mut var_uc_depmue0_dn6: f64 = *var_uc_depmue0_dn6_slot;
        let mut var_uc_depmue0_dn7: f64 = *var_uc_depmue0_dn7_slot;
        let mut var_uc_depmue0_dn8: f64 = *var_uc_depmue0_dn8_slot;
        let mut var_uc_depmue0_dn9: f64 = *var_uc_depmue0_dn9_slot;
        let mut var_uc_depmue0_rv: f64 = *var_uc_depmue0_rv_slot;
        let mut var_uc_depmue1: f64 = *var_uc_depmue1_slot;
        let mut var_uc_depmue1_dn0: f64 = *var_uc_depmue1_dn0_slot;
        let mut var_uc_depmue1_dn10: f64 = *var_uc_depmue1_dn10_slot;
        let mut var_uc_depmue1_dn11: f64 = *var_uc_depmue1_dn11_slot;
        let mut var_uc_depmue1_dn14: f64 = *var_uc_depmue1_dn14_slot;
        let mut var_uc_depmue1_dn2: f64 = *var_uc_depmue1_dn2_slot;
        let mut var_uc_depmue1_dn4: f64 = *var_uc_depmue1_dn4_slot;
        let mut var_uc_depmue1_dn5: f64 = *var_uc_depmue1_dn5_slot;
        let mut var_uc_depmue1_dn6: f64 = *var_uc_depmue1_dn6_slot;
        let mut var_uc_depmue1_dn7: f64 = *var_uc_depmue1_dn7_slot;
        let mut var_uc_depmue1_dn8: f64 = *var_uc_depmue1_dn8_slot;
        let mut var_uc_depmue1_dn9: f64 = *var_uc_depmue1_dn9_slot;
        let mut var_uc_depmue1_rv: f64 = *var_uc_depmue1_rv_slot;
        let mut var_uc_depmue2: f64 = *var_uc_depmue2_slot;
        let mut var_uc_depmue2_dn0: f64 = *var_uc_depmue2_dn0_slot;
        let mut var_uc_depmue2_dn10: f64 = *var_uc_depmue2_dn10_slot;
        let mut var_uc_depmue2_dn11: f64 = *var_uc_depmue2_dn11_slot;
        let mut var_uc_depmue2_dn14: f64 = *var_uc_depmue2_dn14_slot;
        let mut var_uc_depmue2_dn2: f64 = *var_uc_depmue2_dn2_slot;
        let mut var_uc_depmue2_dn4: f64 = *var_uc_depmue2_dn4_slot;
        let mut var_uc_depmue2_dn5: f64 = *var_uc_depmue2_dn5_slot;
        let mut var_uc_depmue2_dn6: f64 = *var_uc_depmue2_dn6_slot;
        let mut var_uc_depmue2_dn7: f64 = *var_uc_depmue2_dn7_slot;
        let mut var_uc_depmue2_dn8: f64 = *var_uc_depmue2_dn8_slot;
        let mut var_uc_depmue2_dn9: f64 = *var_uc_depmue2_dn9_slot;
        let mut var_uc_depmue2_rv: f64 = *var_uc_depmue2_rv_slot;
        let mut var_uc_depmueback0: f64 = *var_uc_depmueback0_slot;
        let mut var_uc_depmueback0_dn0: f64 = *var_uc_depmueback0_dn0_slot;
        let mut var_uc_depmueback0_dn10: f64 = *var_uc_depmueback0_dn10_slot;
        let mut var_uc_depmueback0_dn11: f64 = *var_uc_depmueback0_dn11_slot;
        let mut var_uc_depmueback0_dn14: f64 = *var_uc_depmueback0_dn14_slot;
        let mut var_uc_depmueback0_dn2: f64 = *var_uc_depmueback0_dn2_slot;
        let mut var_uc_depmueback0_dn4: f64 = *var_uc_depmueback0_dn4_slot;
        let mut var_uc_depmueback0_dn5: f64 = *var_uc_depmueback0_dn5_slot;
        let mut var_uc_depmueback0_dn6: f64 = *var_uc_depmueback0_dn6_slot;
        let mut var_uc_depmueback0_dn7: f64 = *var_uc_depmueback0_dn7_slot;
        let mut var_uc_depmueback0_dn8: f64 = *var_uc_depmueback0_dn8_slot;
        let mut var_uc_depmueback0_dn9: f64 = *var_uc_depmueback0_dn9_slot;
        let mut var_uc_depmueback0_rv: f64 = *var_uc_depmueback0_rv_slot;
        let mut var_uc_depmueback1: f64 = *var_uc_depmueback1_slot;
        let mut var_uc_depmueback1_dn0: f64 = *var_uc_depmueback1_dn0_slot;
        let mut var_uc_depmueback1_dn10: f64 = *var_uc_depmueback1_dn10_slot;
        let mut var_uc_depmueback1_dn11: f64 = *var_uc_depmueback1_dn11_slot;
        let mut var_uc_depmueback1_dn14: f64 = *var_uc_depmueback1_dn14_slot;
        let mut var_uc_depmueback1_dn2: f64 = *var_uc_depmueback1_dn2_slot;
        let mut var_uc_depmueback1_dn4: f64 = *var_uc_depmueback1_dn4_slot;
        let mut var_uc_depmueback1_dn5: f64 = *var_uc_depmueback1_dn5_slot;
        let mut var_uc_depmueback1_dn6: f64 = *var_uc_depmueback1_dn6_slot;
        let mut var_uc_depmueback1_dn7: f64 = *var_uc_depmueback1_dn7_slot;
        let mut var_uc_depmueback1_dn8: f64 = *var_uc_depmueback1_dn8_slot;
        let mut var_uc_depmueback1_dn9: f64 = *var_uc_depmueback1_dn9_slot;
        let mut var_uc_depmueback1_rv: f64 = *var_uc_depmueback1_rv_slot;
        let mut var_uc_depmueph1: f64 = *var_uc_depmueph1_slot;
        let mut var_uc_depmueph1_rv: f64 = *var_uc_depmueph1_rv_slot;
        let mut var_uc_depthn: f64 = *var_uc_depthn_slot;
        let mut var_uc_depthn_dn0: f64 = *var_uc_depthn_dn0_slot;
        let mut var_uc_depthn_dn10: f64 = *var_uc_depthn_dn10_slot;
        let mut var_uc_depthn_dn11: f64 = *var_uc_depthn_dn11_slot;
        let mut var_uc_depthn_dn14: f64 = *var_uc_depthn_dn14_slot;
        let mut var_uc_depthn_dn2: f64 = *var_uc_depthn_dn2_slot;
        let mut var_uc_depthn_dn4: f64 = *var_uc_depthn_dn4_slot;
        let mut var_uc_depthn_dn5: f64 = *var_uc_depthn_dn5_slot;
        let mut var_uc_depthn_dn6: f64 = *var_uc_depthn_dn6_slot;
        let mut var_uc_depthn_dn7: f64 = *var_uc_depthn_dn7_slot;
        let mut var_uc_depthn_dn8: f64 = *var_uc_depthn_dn8_slot;
        let mut var_uc_depthn_dn9: f64 = *var_uc_depthn_dn9_slot;
        let mut var_uc_depthn_rv: f64 = *var_uc_depthn_rv_slot;
        let mut var_uc_depvdsef1: f64 = *var_uc_depvdsef1_slot;
        let mut var_uc_depvdsef1_dn0: f64 = *var_uc_depvdsef1_dn0_slot;
        let mut var_uc_depvdsef1_dn10: f64 = *var_uc_depvdsef1_dn10_slot;
        let mut var_uc_depvdsef1_dn11: f64 = *var_uc_depvdsef1_dn11_slot;
        let mut var_uc_depvdsef1_dn14: f64 = *var_uc_depvdsef1_dn14_slot;
        let mut var_uc_depvdsef1_dn2: f64 = *var_uc_depvdsef1_dn2_slot;
        let mut var_uc_depvdsef1_dn4: f64 = *var_uc_depvdsef1_dn4_slot;
        let mut var_uc_depvdsef1_dn5: f64 = *var_uc_depvdsef1_dn5_slot;
        let mut var_uc_depvdsef1_dn6: f64 = *var_uc_depvdsef1_dn6_slot;
        let mut var_uc_depvdsef1_dn7: f64 = *var_uc_depvdsef1_dn7_slot;
        let mut var_uc_depvdsef1_dn8: f64 = *var_uc_depvdsef1_dn8_slot;
        let mut var_uc_depvdsef1_dn9: f64 = *var_uc_depvdsef1_dn9_slot;
        let mut var_uc_depvdsef1_rv: f64 = *var_uc_depvdsef1_rv_slot;
        let mut var_uc_depvdsef2: f64 = *var_uc_depvdsef2_slot;
        let mut var_uc_depvdsef2_dn0: f64 = *var_uc_depvdsef2_dn0_slot;
        let mut var_uc_depvdsef2_dn10: f64 = *var_uc_depvdsef2_dn10_slot;
        let mut var_uc_depvdsef2_dn11: f64 = *var_uc_depvdsef2_dn11_slot;
        let mut var_uc_depvdsef2_dn14: f64 = *var_uc_depvdsef2_dn14_slot;
        let mut var_uc_depvdsef2_dn2: f64 = *var_uc_depvdsef2_dn2_slot;
        let mut var_uc_depvdsef2_dn4: f64 = *var_uc_depvdsef2_dn4_slot;
        let mut var_uc_depvdsef2_dn5: f64 = *var_uc_depvdsef2_dn5_slot;
        let mut var_uc_depvdsef2_dn6: f64 = *var_uc_depvdsef2_dn6_slot;
        let mut var_uc_depvdsef2_dn7: f64 = *var_uc_depvdsef2_dn7_slot;
        let mut var_uc_depvdsef2_dn8: f64 = *var_uc_depvdsef2_dn8_slot;
        let mut var_uc_depvdsef2_dn9: f64 = *var_uc_depvdsef2_dn9_slot;
        let mut var_uc_depvdsef2_rv: f64 = *var_uc_depvdsef2_rv_slot;
        let mut var_uc_depvmax: f64 = *var_uc_depvmax_slot;
        let mut var_uc_depvmax_dn0: f64 = *var_uc_depvmax_dn0_slot;
        let mut var_uc_depvmax_dn10: f64 = *var_uc_depvmax_dn10_slot;
        let mut var_uc_depvmax_dn11: f64 = *var_uc_depvmax_dn11_slot;
        let mut var_uc_depvmax_dn14: f64 = *var_uc_depvmax_dn14_slot;
        let mut var_uc_depvmax_dn2: f64 = *var_uc_depvmax_dn2_slot;
        let mut var_uc_depvmax_dn4: f64 = *var_uc_depvmax_dn4_slot;
        let mut var_uc_depvmax_dn5: f64 = *var_uc_depvmax_dn5_slot;
        let mut var_uc_depvmax_dn6: f64 = *var_uc_depvmax_dn6_slot;
        let mut var_uc_depvmax_dn7: f64 = *var_uc_depvmax_dn7_slot;
        let mut var_uc_depvmax_dn8: f64 = *var_uc_depvmax_dn8_slot;
        let mut var_uc_depvmax_dn9: f64 = *var_uc_depvmax_dn9_slot;
        let mut var_uc_depvmax_rv: f64 = *var_uc_depvmax_rv_slot;
        let mut var_uc_depwlp: f64 = *var_uc_depwlp_slot;
        let mut var_uc_depwlp_dn0: f64 = *var_uc_depwlp_dn0_slot;
        let mut var_uc_depwlp_dn10: f64 = *var_uc_depwlp_dn10_slot;
        let mut var_uc_depwlp_dn11: f64 = *var_uc_depwlp_dn11_slot;
        let mut var_uc_depwlp_dn14: f64 = *var_uc_depwlp_dn14_slot;
        let mut var_uc_depwlp_dn2: f64 = *var_uc_depwlp_dn2_slot;
        let mut var_uc_depwlp_dn4: f64 = *var_uc_depwlp_dn4_slot;
        let mut var_uc_depwlp_dn5: f64 = *var_uc_depwlp_dn5_slot;
        let mut var_uc_depwlp_dn6: f64 = *var_uc_depwlp_dn6_slot;
        let mut var_uc_depwlp_dn7: f64 = *var_uc_depwlp_dn7_slot;
        let mut var_uc_depwlp_dn8: f64 = *var_uc_depwlp_dn8_slot;
        let mut var_uc_depwlp_dn9: f64 = *var_uc_depwlp_dn9_slot;
        let mut var_uc_depwlp_rv: f64 = *var_uc_depwlp_rv_slot;
        let mut var_uc_ndepm: f64 = *var_uc_ndepm_slot;
        let mut var_uc_ndepm_dn0: f64 = *var_uc_ndepm_dn0_slot;
        let mut var_uc_ndepm_dn10: f64 = *var_uc_ndepm_dn10_slot;
        let mut var_uc_ndepm_dn11: f64 = *var_uc_ndepm_dn11_slot;
        let mut var_uc_ndepm_dn14: f64 = *var_uc_ndepm_dn14_slot;
        let mut var_uc_ndepm_dn2: f64 = *var_uc_ndepm_dn2_slot;
        let mut var_uc_ndepm_dn4: f64 = *var_uc_ndepm_dn4_slot;
        let mut var_uc_ndepm_dn5: f64 = *var_uc_ndepm_dn5_slot;
        let mut var_uc_ndepm_dn6: f64 = *var_uc_ndepm_dn6_slot;
        let mut var_uc_ndepm_dn7: f64 = *var_uc_ndepm_dn7_slot;
        let mut var_uc_ndepm_dn8: f64 = *var_uc_ndepm_dn8_slot;
        let mut var_uc_ndepm_dn9: f64 = *var_uc_ndepm_dn9_slot;
        let mut var_uc_ndepm_rv: f64 = *var_uc_ndepm_rv_slot;
        let mut var_uc_rdrcx: f64 = *var_uc_rdrcx_slot;
        let mut var_uc_rdrcx_rv: f64 = *var_uc_rdrcx_rv_slot;
        let mut var_uc_scp22: f64 = *var_uc_scp22_slot;
        let mut var_uc_scp22_rv: f64 = *var_uc_scp22_rv_slot;
        let mut var_uc_xldld: f64 = *var_uc_xldld_slot;
        let mut var_uc_xldld_rv: f64 = *var_uc_xldld_rv_slot;
        let mut var_uc_xpdv: f64 = *var_uc_xpdv_slot;
        let mut var_uc_xpdv_rv: f64 = *var_uc_xpdv_rv_slot;
        let mut var_w_nqs_a: f64 = *var_w_nqs_a_slot;
        let mut var_w_nqs_a_dn18: f64 = *var_w_nqs_a_dn18_slot;
        let mut var_w_nqs_a_rv: f64 = *var_w_nqs_a_rv_slot;
        let mut var_w_res: f64 = *var_w_res_slot;
        let mut var_w_res_dn0: f64 = *var_w_res_dn0_slot;
        let mut var_w_res_dn10: f64 = *var_w_res_dn10_slot;
        let mut var_w_res_dn11: f64 = *var_w_res_dn11_slot;
        let mut var_w_res_dn14: f64 = *var_w_res_dn14_slot;
        let mut var_w_res_dn2: f64 = *var_w_res_dn2_slot;
        let mut var_w_res_dn4: f64 = *var_w_res_dn4_slot;
        let mut var_w_res_dn5: f64 = *var_w_res_dn5_slot;
        let mut var_w_res_dn6: f64 = *var_w_res_dn6_slot;
        let mut var_w_res_dn7: f64 = *var_w_res_dn7_slot;
        let mut var_w_res_dn8: f64 = *var_w_res_dn8_slot;
        let mut var_w_res_dn9: f64 = *var_w_res_dn9_slot;
        let mut var_w_res_rv: f64 = *var_w_res_rv_slot;
        let mut var_wdep_func: f64 = *var_wdep_func_slot;
        let mut var_wdep_func_dn0: f64 = *var_wdep_func_dn0_slot;
        let mut var_wdep_func_dn10: f64 = *var_wdep_func_dn10_slot;
        let mut var_wdep_func_dn11: f64 = *var_wdep_func_dn11_slot;
        let mut var_wdep_func_dn14: f64 = *var_wdep_func_dn14_slot;
        let mut var_wdep_func_dn2: f64 = *var_wdep_func_dn2_slot;
        let mut var_wdep_func_dn4: f64 = *var_wdep_func_dn4_slot;
        let mut var_wdep_func_dn5: f64 = *var_wdep_func_dn5_slot;
        let mut var_wdep_func_dn6: f64 = *var_wdep_func_dn6_slot;
        let mut var_wdep_func_dn7: f64 = *var_wdep_func_dn7_slot;
        let mut var_wdep_func_dn8: f64 = *var_wdep_func_dn8_slot;
        let mut var_wdep_func_dn9: f64 = *var_wdep_func_dn9_slot;
        let mut var_wdep_func_rv: f64 = *var_wdep_func_rv_slot;
        let mut var_wk_ii: f64 = *var_wk_ii_slot;
        let mut var_wk_ii_dn0: f64 = *var_wk_ii_dn0_slot;
        let mut var_wk_ii_dn10: f64 = *var_wk_ii_dn10_slot;
        let mut var_wk_ii_dn11: f64 = *var_wk_ii_dn11_slot;
        let mut var_wk_ii_dn14: f64 = *var_wk_ii_dn14_slot;
        let mut var_wk_ii_dn2: f64 = *var_wk_ii_dn2_slot;
        let mut var_wk_ii_dn4: f64 = *var_wk_ii_dn4_slot;
        let mut var_wk_ii_dn5: f64 = *var_wk_ii_dn5_slot;
        let mut var_wk_ii_dn6: f64 = *var_wk_ii_dn6_slot;
        let mut var_wk_ii_dn7: f64 = *var_wk_ii_dn7_slot;
        let mut var_wk_ii_dn8: f64 = *var_wk_ii_dn8_slot;
        let mut var_wk_ii_dn9: f64 = *var_wk_ii_dn9_slot;
        let mut var_wk_ii_rv: f64 = *var_wk_ii_rv_slot;

        var_idspt1 = 0.0;
        var_idspt1_dn0 = 0.0;
        var_idspt1_dn2 = 0.0;
        var_idspt1_dn4 = 0.0;
        var_idspt1_dn5 = 0.0;
        var_idspt1_dn6 = 0.0;
        var_idspt1_dn7 = 0.0;
        var_idspt1_dn8 = 0.0;
        var_idspt1_dn9 = 0.0;
        var_idspt1_dn10 = 0.0;
        var_idspt1_dn11 = 0.0;
        var_idspt1_dn14 = 0.0;
        var_idspt1_rv = 0.0;

        var_cox0_func = 0.0;
        var_cox0_func_rv = 0.0;

        var_iwnqs0_a = 0.0;
        var_iwnqs0_a_dn0 = 0.0;
        var_iwnqs0_a_dn2 = 0.0;
        var_iwnqs0_a_dn4 = 0.0;
        var_iwnqs0_a_dn5 = 0.0;
        var_iwnqs0_a_dn6 = 0.0;
        var_iwnqs0_a_dn7 = 0.0;
        var_iwnqs0_a_dn8 = 0.0;
        var_iwnqs0_a_dn9 = 0.0;
        var_iwnqs0_a_dn10 = 0.0;
        var_iwnqs0_a_dn11 = 0.0;
        var_iwnqs0_a_dn14 = 0.0;
        var_iwnqs0_a_dn18 = 0.0;
        var_iwnqs0_a_rv = 0.0;

        var_inqs0_a = 0.0;
        var_inqs0_a_dn0 = 0.0;
        var_inqs0_a_dn2 = 0.0;
        var_inqs0_a_dn4 = 0.0;
        var_inqs0_a_dn5 = 0.0;
        var_inqs0_a_dn6 = 0.0;
        var_inqs0_a_dn7 = 0.0;
        var_inqs0_a_dn8 = 0.0;
        var_inqs0_a_dn9 = 0.0;
        var_inqs0_a_dn10 = 0.0;
        var_inqs0_a_dn11 = 0.0;
        var_inqs0_a_dn14 = 0.0;
        var_inqs0_a_dn16 = 0.0;
        var_inqs0_a_rv = 0.0;

        var_inqs0_k = 0.0;
        var_inqs0_k_dn0 = 0.0;
        var_inqs0_k_dn2 = 0.0;
        var_inqs0_k_dn4 = 0.0;
        var_inqs0_k_dn5 = 0.0;
        var_inqs0_k_dn6 = 0.0;
        var_inqs0_k_dn7 = 0.0;
        var_inqs0_k_dn8 = 0.0;
        var_inqs0_k_dn9 = 0.0;
        var_inqs0_k_dn10 = 0.0;
        var_inqs0_k_dn11 = 0.0;
        var_inqs0_k_dn14 = 0.0;
        var_inqs0_k_dn17 = 0.0;
        var_inqs0_k_rv = 0.0;

        var_isubibpc = 0.0;
        var_isubibpc_dn0 = 0.0;
        var_isubibpc_dn2 = 0.0;
        var_isubibpc_dn4 = 0.0;
        var_isubibpc_dn5 = 0.0;
        var_isubibpc_dn6 = 0.0;
        var_isubibpc_dn7 = 0.0;
        var_isubibpc_dn8 = 0.0;
        var_isubibpc_dn9 = 0.0;
        var_isubibpc_dn10 = 0.0;
        var_isubibpc_dn11 = 0.0;
        var_isubibpc_dn14 = 0.0;
        var_isubibpc_rv = 0.0;

        var_lover_func = 0.0;
        var_lover_func_dn0 = 0.0;
        var_lover_func_dn2 = 0.0;
        var_lover_func_dn4 = 0.0;
        var_lover_func_dn5 = 0.0;
        var_lover_func_dn6 = 0.0;
        var_lover_func_dn7 = 0.0;
        var_lover_func_dn8 = 0.0;
        var_lover_func_dn9 = 0.0;
        var_lover_func_dn10 = 0.0;
        var_lover_func_dn11 = 0.0;
        var_lover_func_dn14 = 0.0;
        var_lover_func_rv = 0.0;

        var_q_nqs_a = 0.0;
        var_q_nqs_a_dn16 = 0.0;
        var_q_nqs_a_rv = 0.0;

        var_q_nqs_k = 0.0;
        var_q_nqs_k_dn17 = 0.0;
        var_q_nqs_k_rv = 0.0;

        var_w_nqs_a = 0.0;
        var_w_nqs_a_dn18 = 0.0;
        var_w_nqs_a_rv = 0.0;

        var_w_res = 0.0;
        var_w_res_dn0 = 0.0;
        var_w_res_dn2 = 0.0;
        var_w_res_dn4 = 0.0;
        var_w_res_dn5 = 0.0;
        var_w_res_dn6 = 0.0;
        var_w_res_dn7 = 0.0;
        var_w_res_dn8 = 0.0;
        var_w_res_dn9 = 0.0;
        var_w_res_dn10 = 0.0;
        var_w_res_dn11 = 0.0;
        var_w_res_dn14 = 0.0;
        var_w_res_rv = 0.0;

        var_wdep_func = 0.0;
        var_wdep_func_dn0 = 0.0;
        var_wdep_func_dn2 = 0.0;
        var_wdep_func_dn4 = 0.0;
        var_wdep_func_dn5 = 0.0;
        var_wdep_func_dn6 = 0.0;
        var_wdep_func_dn7 = 0.0;
        var_wdep_func_dn8 = 0.0;
        var_wdep_func_dn9 = 0.0;
        var_wdep_func_dn10 = 0.0;
        var_wdep_func_dn11 = 0.0;
        var_wdep_func_dn14 = 0.0;
        var_wdep_func_rv = 0.0;

        var_wk_ii = 0.0;
        var_wk_ii_dn0 = 0.0;
        var_wk_ii_dn2 = 0.0;
        var_wk_ii_dn4 = 0.0;
        var_wk_ii_dn5 = 0.0;
        var_wk_ii_dn6 = 0.0;
        var_wk_ii_dn7 = 0.0;
        var_wk_ii_dn8 = 0.0;
        var_wk_ii_dn9 = 0.0;
        var_wk_ii_dn10 = 0.0;
        var_wk_ii_dn11 = 0.0;
        var_wk_ii_dn14 = 0.0;
        var_wk_ii_rv = 0.0;

        let (assign5340_e1947,) = {
    if (p.p40 != 0.0) {
        (0.0,)
    } else {
        (p.p17,)
    }
};
        var_uc_corsrd = assign5340_e1947;
        var_uc_corsrd_rv = 0.0;

        var_uc_xpdv = p.p104;
        var_uc_xpdv_rv = 0.0;

        var_uc_xldld = p.p294;
        var_uc_xldld_rv = 0.0;

        var_uc_scp22 = p.p222;
        var_uc_scp22_rv = 0.0;

        var_uc_rdrcx = p.p420;
        var_uc_rdrcx_rv = 0.0;

        var_mfactor = 1.0;
        var_mfactor_rv = 0.0;

        let assign5500_e1990: f64 = if var_uc_scp22 < 0.0 { 1.0 } else { 0.0 };
        var_guard10 = assign5500_e1990;
        var_guard10_rv = 0.0;

        let (assign5510_e1994,) = {
    if (var_guard10 != 0.0) {
        (0.0,)
    } else {
        (var_uc_scp22,)
    }
};
        var_uc_scp22 = assign5510_e1994;
        var_uc_scp22_rv = 0.0;

        let assign5520_e1997: f64 = if var_uc_scp22 > 0.0 { 1.0 } else { 0.0 };
        var_guard11 = assign5520_e1997;
        var_guard11_rv = 0.0;

        let (assign5530_e2001,) = {
    if (var_guard11 != 0.0) {
        (0.0,)
    } else {
        (var_uc_scp22,)
    }
};
        var_uc_scp22 = assign5530_e2001;
        var_uc_scp22_rv = 0.0;

        let assign5550_e2009: f64 = if var_uc_xldld < 0.0 { 1.0 } else { 0.0 };
        var_guard13 = assign5550_e2009;
        var_guard13_rv = 0.0;

        let (assign5560_e2013,) = {
    if (var_guard13 != 0.0) {
        (0.0,)
    } else {
        (var_uc_xldld,)
    }
};
        var_uc_xldld = assign5560_e2013;
        var_uc_xldld_rv = 0.0;

        let assign5590_e2026: f64 = if var_uc_rdrcx < 0.0 { 1.0 } else { 0.0 };
        var_guard16 = assign5590_e2026;
        var_guard16_rv = 0.0;

        let (assign5600_e2030,) = {
    if (var_guard16 != 0.0) {
        (0.0,)
    } else {
        (var_uc_rdrcx,)
    }
};
        var_uc_rdrcx = assign5600_e2030;
        var_uc_rdrcx_rv = 0.0;

        let assign5610_e2033: f64 = if var_uc_rdrcx > 1.0 { 1.0 } else { 0.0 };
        var_guard17 = assign5610_e2033;
        var_guard17_rv = 0.0;

        let (assign5620_e2037,) = {
    if (var_guard17 != 0.0) {
        (1.0,)
    } else {
        (var_uc_rdrcx,)
    }
};
        var_uc_rdrcx = assign5620_e2037;
        var_uc_rdrcx_rv = 0.0;

        var_uc_ndepm = p.p340;
        var_uc_ndepm_dn0 = 0.0;
        var_uc_ndepm_dn2 = 0.0;
        var_uc_ndepm_dn4 = 0.0;
        var_uc_ndepm_dn5 = 0.0;
        var_uc_ndepm_dn6 = 0.0;
        var_uc_ndepm_dn7 = 0.0;
        var_uc_ndepm_dn8 = 0.0;
        var_uc_ndepm_dn9 = 0.0;
        var_uc_ndepm_dn10 = 0.0;
        var_uc_ndepm_dn11 = 0.0;
        var_uc_ndepm_dn14 = 0.0;
        var_uc_ndepm_rv = 0.0;

        var_uc_depthn = p.p343;
        var_uc_depthn_dn0 = 0.0;
        var_uc_depthn_dn2 = 0.0;
        var_uc_depthn_dn4 = 0.0;
        var_uc_depthn_dn5 = 0.0;
        var_uc_depthn_dn6 = 0.0;
        var_uc_depthn_dn7 = 0.0;
        var_uc_depthn_dn8 = 0.0;
        var_uc_depthn_dn9 = 0.0;
        var_uc_depthn_dn10 = 0.0;
        var_uc_depthn_dn11 = 0.0;
        var_uc_depthn_dn14 = 0.0;
        var_uc_depthn_rv = 0.0;

        var_uc_codep = p.p42;
        var_uc_codep_rv = 0.0;

        var_uc_depmueback0 = p.p354;
        var_uc_depmueback0_dn0 = 0.0;
        var_uc_depmueback0_dn2 = 0.0;
        var_uc_depmueback0_dn4 = 0.0;
        var_uc_depmueback0_dn5 = 0.0;
        var_uc_depmueback0_dn6 = 0.0;
        var_uc_depmueback0_dn7 = 0.0;
        var_uc_depmueback0_dn8 = 0.0;
        var_uc_depmueback0_dn9 = 0.0;
        var_uc_depmueback0_dn10 = 0.0;
        var_uc_depmueback0_dn11 = 0.0;
        var_uc_depmueback0_dn14 = 0.0;
        var_uc_depmueback0_rv = 0.0;

        var_uc_depmueback1 = p.p355;
        var_uc_depmueback1_dn0 = 0.0;
        var_uc_depmueback1_dn2 = 0.0;
        var_uc_depmueback1_dn4 = 0.0;
        var_uc_depmueback1_dn5 = 0.0;
        var_uc_depmueback1_dn6 = 0.0;
        var_uc_depmueback1_dn7 = 0.0;
        var_uc_depmueback1_dn8 = 0.0;
        var_uc_depmueback1_dn9 = 0.0;
        var_uc_depmueback1_dn10 = 0.0;
        var_uc_depmueback1_dn11 = 0.0;
        var_uc_depmueback1_dn14 = 0.0;
        var_uc_depmueback1_rv = 0.0;

        var_uc_depmue0 = p.p346;
        var_uc_depmue0_dn0 = 0.0;
        var_uc_depmue0_dn2 = 0.0;
        var_uc_depmue0_dn4 = 0.0;
        var_uc_depmue0_dn5 = 0.0;
        var_uc_depmue0_dn6 = 0.0;
        var_uc_depmue0_dn7 = 0.0;
        var_uc_depmue0_dn8 = 0.0;
        var_uc_depmue0_dn9 = 0.0;
        var_uc_depmue0_dn10 = 0.0;
        var_uc_depmue0_dn11 = 0.0;
        var_uc_depmue0_dn14 = 0.0;
        var_uc_depmue0_rv = 0.0;

        var_uc_depmue1 = p.p349;
        var_uc_depmue1_dn0 = 0.0;
        var_uc_depmue1_dn2 = 0.0;
        var_uc_depmue1_dn4 = 0.0;
        var_uc_depmue1_dn5 = 0.0;
        var_uc_depmue1_dn6 = 0.0;
        var_uc_depmue1_dn7 = 0.0;
        var_uc_depmue1_dn8 = 0.0;
        var_uc_depmue1_dn9 = 0.0;
        var_uc_depmue1_dn10 = 0.0;
        var_uc_depmue1_dn11 = 0.0;
        var_uc_depmue1_dn14 = 0.0;
        var_uc_depmue1_rv = 0.0;

        var_uc_depmue2 = p.p352;
        var_uc_depmue2_dn0 = 0.0;
        var_uc_depmue2_dn2 = 0.0;
        var_uc_depmue2_dn4 = 0.0;
        var_uc_depmue2_dn5 = 0.0;
        var_uc_depmue2_dn6 = 0.0;
        var_uc_depmue2_dn7 = 0.0;
        var_uc_depmue2_dn8 = 0.0;
        var_uc_depmue2_dn9 = 0.0;
        var_uc_depmue2_dn10 = 0.0;
        var_uc_depmue2_dn11 = 0.0;
        var_uc_depmue2_dn14 = 0.0;
        var_uc_depmue2_rv = 0.0;

        var_uc_depleak = p.p360;
        var_uc_depleak_dn0 = 0.0;
        var_uc_depleak_dn2 = 0.0;
        var_uc_depleak_dn4 = 0.0;
        var_uc_depleak_dn5 = 0.0;
        var_uc_depleak_dn6 = 0.0;
        var_uc_depleak_dn7 = 0.0;
        var_uc_depleak_dn8 = 0.0;
        var_uc_depleak_dn9 = 0.0;
        var_uc_depleak_dn10 = 0.0;
        var_uc_depleak_dn11 = 0.0;
        var_uc_depleak_dn14 = 0.0;
        var_uc_depleak_rv = 0.0;

        var_uc_depvmax = p.p367;
        var_uc_depvmax_dn0 = 0.0;
        var_uc_depvmax_dn2 = 0.0;
        var_uc_depvmax_dn4 = 0.0;
        var_uc_depvmax_dn5 = 0.0;
        var_uc_depvmax_dn6 = 0.0;
        var_uc_depvmax_dn7 = 0.0;
        var_uc_depvmax_dn8 = 0.0;
        var_uc_depvmax_dn9 = 0.0;
        var_uc_depvmax_dn10 = 0.0;
        var_uc_depvmax_dn11 = 0.0;
        var_uc_depvmax_dn14 = 0.0;
        var_uc_depvmax_rv = 0.0;

        var_uc_depwlp = p.p364;
        var_uc_depwlp_dn0 = 0.0;
        var_uc_depwlp_dn2 = 0.0;
        var_uc_depwlp_dn4 = 0.0;
        var_uc_depwlp_dn5 = 0.0;
        var_uc_depwlp_dn6 = 0.0;
        var_uc_depwlp_dn7 = 0.0;
        var_uc_depwlp_dn8 = 0.0;
        var_uc_depwlp_dn9 = 0.0;
        var_uc_depwlp_dn10 = 0.0;
        var_uc_depwlp_dn11 = 0.0;
        var_uc_depwlp_dn14 = 0.0;
        var_uc_depwlp_rv = 0.0;

        var_uc_depmueph1 = p.p377;
        var_uc_depmueph1_rv = 0.0;

        var_uc_depvdsef1 = p.p370;
        var_uc_depvdsef1_dn0 = 0.0;
        var_uc_depvdsef1_dn2 = 0.0;
        var_uc_depvdsef1_dn4 = 0.0;
        var_uc_depvdsef1_dn5 = 0.0;
        var_uc_depvdsef1_dn6 = 0.0;
        var_uc_depvdsef1_dn7 = 0.0;
        var_uc_depvdsef1_dn8 = 0.0;
        var_uc_depvdsef1_dn9 = 0.0;
        var_uc_depvdsef1_dn10 = 0.0;
        var_uc_depvdsef1_dn11 = 0.0;
        var_uc_depvdsef1_dn14 = 0.0;
        var_uc_depvdsef1_rv = 0.0;

        var_uc_depvdsef2 = p.p371;
        var_uc_depvdsef2_dn0 = 0.0;
        var_uc_depvdsef2_dn2 = 0.0;
        var_uc_depvdsef2_dn4 = 0.0;
        var_uc_depvdsef2_dn5 = 0.0;
        var_uc_depvdsef2_dn6 = 0.0;
        var_uc_depvdsef2_dn7 = 0.0;
        var_uc_depvdsef2_dn8 = 0.0;
        var_uc_depvdsef2_dn9 = 0.0;
        var_uc_depvdsef2_dn10 = 0.0;
        var_uc_depvdsef2_dn11 = 0.0;
        var_uc_depvdsef2_dn14 = 0.0;
        var_uc_depvdsef2_rv = 0.0;

        let assign6710_e2710: f64 = if ((var_uc_codep < 3.0) && (var_uc_codep > 0.0)) { 1.0 } else { 0.0 };
        var_guard112 = assign6710_e2710;
        var_guard112_rv = 0.0;

        let assign6740_e2723: f64 = if var_uc_ndepm < 5000000000000000.0 { 1.0 } else { 0.0 };
        var_guard115 = assign6740_e2723;
        var_guard115_rv = 0.0;

        let (assign6750_e2729, assign6750_e2729_d_n0, assign6750_e2729_d_n2, assign6750_e2729_d_n4, assign6750_e2729_d_n5, assign6750_e2729_d_n6, assign6750_e2729_d_n7, assign6750_e2729_d_n8, assign6750_e2729_d_n9, assign6750_e2729_d_n10, assign6750_e2729_d_n11, assign6750_e2729_d_n14,) = {
    if ((var_guard112 != 0.0) && (var_guard115 != 0.0)) {
        (5000000000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_ndepm, var_uc_ndepm_dn0, var_uc_ndepm_dn2, var_uc_ndepm_dn4, var_uc_ndepm_dn5, var_uc_ndepm_dn6, var_uc_ndepm_dn7, var_uc_ndepm_dn8, var_uc_ndepm_dn9, var_uc_ndepm_dn10, var_uc_ndepm_dn11, var_uc_ndepm_dn14,)
    }
};
        var_uc_ndepm = assign6750_e2729;
        var_uc_ndepm_dn0 = assign6750_e2729_d_n0;
        var_uc_ndepm_dn2 = assign6750_e2729_d_n2;
        var_uc_ndepm_dn4 = assign6750_e2729_d_n4;
        var_uc_ndepm_dn5 = assign6750_e2729_d_n5;
        var_uc_ndepm_dn6 = assign6750_e2729_d_n6;
        var_uc_ndepm_dn7 = assign6750_e2729_d_n7;
        var_uc_ndepm_dn8 = assign6750_e2729_d_n8;
        var_uc_ndepm_dn9 = assign6750_e2729_d_n9;
        var_uc_ndepm_dn10 = assign6750_e2729_d_n10;
        var_uc_ndepm_dn11 = assign6750_e2729_d_n11;
        var_uc_ndepm_dn14 = assign6750_e2729_d_n14;
        var_uc_ndepm_rv = 0.0;

        let assign6760_e2732: f64 = if var_uc_ndepm > 1e18 { 1.0 } else { 0.0 };
        var_guard116 = assign6760_e2732;
        var_guard116_rv = 0.0;

        let (assign6770_e2738, assign6770_e2738_d_n0, assign6770_e2738_d_n2, assign6770_e2738_d_n4, assign6770_e2738_d_n5, assign6770_e2738_d_n6, assign6770_e2738_d_n7, assign6770_e2738_d_n8, assign6770_e2738_d_n9, assign6770_e2738_d_n10, assign6770_e2738_d_n11, assign6770_e2738_d_n14,) = {
    if ((var_guard112 != 0.0) && (var_guard116 != 0.0)) {
        (1e18, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_ndepm, var_uc_ndepm_dn0, var_uc_ndepm_dn2, var_uc_ndepm_dn4, var_uc_ndepm_dn5, var_uc_ndepm_dn6, var_uc_ndepm_dn7, var_uc_ndepm_dn8, var_uc_ndepm_dn9, var_uc_ndepm_dn10, var_uc_ndepm_dn11, var_uc_ndepm_dn14,)
    }
};
        var_uc_ndepm = assign6770_e2738;
        var_uc_ndepm_dn0 = assign6770_e2738_d_n0;
        var_uc_ndepm_dn2 = assign6770_e2738_d_n2;
        var_uc_ndepm_dn4 = assign6770_e2738_d_n4;
        var_uc_ndepm_dn5 = assign6770_e2738_d_n5;
        var_uc_ndepm_dn6 = assign6770_e2738_d_n6;
        var_uc_ndepm_dn7 = assign6770_e2738_d_n7;
        var_uc_ndepm_dn8 = assign6770_e2738_d_n8;
        var_uc_ndepm_dn9 = assign6770_e2738_d_n9;
        var_uc_ndepm_dn10 = assign6770_e2738_d_n10;
        var_uc_ndepm_dn11 = assign6770_e2738_d_n11;
        var_uc_ndepm_dn14 = assign6770_e2738_d_n14;
        var_uc_ndepm_rv = 0.0;

        let assign6800_e2751: f64 = if var_uc_depthn < 1e-8 { 1.0 } else { 0.0 };
        var_guard119 = assign6800_e2751;
        var_guard119_rv = 0.0;

        let (assign6810_e2757, assign6810_e2757_d_n0, assign6810_e2757_d_n2, assign6810_e2757_d_n4, assign6810_e2757_d_n5, assign6810_e2757_d_n6, assign6810_e2757_d_n7, assign6810_e2757_d_n8, assign6810_e2757_d_n9, assign6810_e2757_d_n10, assign6810_e2757_d_n11, assign6810_e2757_d_n14,) = {
    if ((var_guard112 != 0.0) && (var_guard119 != 0.0)) {
        (1e-8, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depthn, var_uc_depthn_dn0, var_uc_depthn_dn2, var_uc_depthn_dn4, var_uc_depthn_dn5, var_uc_depthn_dn6, var_uc_depthn_dn7, var_uc_depthn_dn8, var_uc_depthn_dn9, var_uc_depthn_dn10, var_uc_depthn_dn11, var_uc_depthn_dn14,)
    }
};
        var_uc_depthn = assign6810_e2757;
        var_uc_depthn_dn0 = assign6810_e2757_d_n0;
        var_uc_depthn_dn2 = assign6810_e2757_d_n2;
        var_uc_depthn_dn4 = assign6810_e2757_d_n4;
        var_uc_depthn_dn5 = assign6810_e2757_d_n5;
        var_uc_depthn_dn6 = assign6810_e2757_d_n6;
        var_uc_depthn_dn7 = assign6810_e2757_d_n7;
        var_uc_depthn_dn8 = assign6810_e2757_d_n8;
        var_uc_depthn_dn9 = assign6810_e2757_d_n9;
        var_uc_depthn_dn10 = assign6810_e2757_d_n10;
        var_uc_depthn_dn11 = assign6810_e2757_d_n11;
        var_uc_depthn_dn14 = assign6810_e2757_d_n14;
        var_uc_depthn_rv = 0.0;

        let assign6820_e2760: f64 = if var_uc_depthn > 1e-6 { 1.0 } else { 0.0 };
        var_guard120 = assign6820_e2760;
        var_guard120_rv = 0.0;

        *var_cox0_func_slot = var_cox0_func;
        *var_cox0_func_rv_slot = var_cox0_func_rv;
        *var_guard10_slot = var_guard10;
        *var_guard10_rv_slot = var_guard10_rv;
        *var_guard11_slot = var_guard11;
        *var_guard112_slot = var_guard112;
        *var_guard112_rv_slot = var_guard112_rv;
        *var_guard115_slot = var_guard115;
        *var_guard115_rv_slot = var_guard115_rv;
        *var_guard116_slot = var_guard116;
        *var_guard116_rv_slot = var_guard116_rv;
        *var_guard119_slot = var_guard119;
        *var_guard119_rv_slot = var_guard119_rv;
        *var_guard11_rv_slot = var_guard11_rv;
        *var_guard120_slot = var_guard120;
        *var_guard120_rv_slot = var_guard120_rv;
        *var_guard13_slot = var_guard13;
        *var_guard13_rv_slot = var_guard13_rv;
        *var_guard16_slot = var_guard16;
        *var_guard16_rv_slot = var_guard16_rv;
        *var_guard17_slot = var_guard17;
        *var_guard17_rv_slot = var_guard17_rv;
        *var_idspt1_slot = var_idspt1;
        *var_idspt1_dn0_slot = var_idspt1_dn0;
        *var_idspt1_dn10_slot = var_idspt1_dn10;
        *var_idspt1_dn11_slot = var_idspt1_dn11;
        *var_idspt1_dn14_slot = var_idspt1_dn14;
        *var_idspt1_dn2_slot = var_idspt1_dn2;
        *var_idspt1_dn4_slot = var_idspt1_dn4;
        *var_idspt1_dn5_slot = var_idspt1_dn5;
        *var_idspt1_dn6_slot = var_idspt1_dn6;
        *var_idspt1_dn7_slot = var_idspt1_dn7;
        *var_idspt1_dn8_slot = var_idspt1_dn8;
        *var_idspt1_dn9_slot = var_idspt1_dn9;
        *var_idspt1_rv_slot = var_idspt1_rv;
        *var_inqs0_a_slot = var_inqs0_a;
        *var_inqs0_a_dn0_slot = var_inqs0_a_dn0;
        *var_inqs0_a_dn10_slot = var_inqs0_a_dn10;
        *var_inqs0_a_dn11_slot = var_inqs0_a_dn11;
        *var_inqs0_a_dn14_slot = var_inqs0_a_dn14;
        *var_inqs0_a_dn16_slot = var_inqs0_a_dn16;
        *var_inqs0_a_dn2_slot = var_inqs0_a_dn2;
        *var_inqs0_a_dn4_slot = var_inqs0_a_dn4;
        *var_inqs0_a_dn5_slot = var_inqs0_a_dn5;
        *var_inqs0_a_dn6_slot = var_inqs0_a_dn6;
        *var_inqs0_a_dn7_slot = var_inqs0_a_dn7;
        *var_inqs0_a_dn8_slot = var_inqs0_a_dn8;
        *var_inqs0_a_dn9_slot = var_inqs0_a_dn9;
        *var_inqs0_a_rv_slot = var_inqs0_a_rv;
        *var_inqs0_k_slot = var_inqs0_k;
        *var_inqs0_k_dn0_slot = var_inqs0_k_dn0;
        *var_inqs0_k_dn10_slot = var_inqs0_k_dn10;
        *var_inqs0_k_dn11_slot = var_inqs0_k_dn11;
        *var_inqs0_k_dn14_slot = var_inqs0_k_dn14;
        *var_inqs0_k_dn17_slot = var_inqs0_k_dn17;
        *var_inqs0_k_dn2_slot = var_inqs0_k_dn2;
        *var_inqs0_k_dn4_slot = var_inqs0_k_dn4;
        *var_inqs0_k_dn5_slot = var_inqs0_k_dn5;
        *var_inqs0_k_dn6_slot = var_inqs0_k_dn6;
        *var_inqs0_k_dn7_slot = var_inqs0_k_dn7;
        *var_inqs0_k_dn8_slot = var_inqs0_k_dn8;
        *var_inqs0_k_dn9_slot = var_inqs0_k_dn9;
        *var_inqs0_k_rv_slot = var_inqs0_k_rv;
        *var_isubibpc_slot = var_isubibpc;
        *var_isubibpc_dn0_slot = var_isubibpc_dn0;
        *var_isubibpc_dn10_slot = var_isubibpc_dn10;
        *var_isubibpc_dn11_slot = var_isubibpc_dn11;
        *var_isubibpc_dn14_slot = var_isubibpc_dn14;
        *var_isubibpc_dn2_slot = var_isubibpc_dn2;
        *var_isubibpc_dn4_slot = var_isubibpc_dn4;
        *var_isubibpc_dn5_slot = var_isubibpc_dn5;
        *var_isubibpc_dn6_slot = var_isubibpc_dn6;
        *var_isubibpc_dn7_slot = var_isubibpc_dn7;
        *var_isubibpc_dn8_slot = var_isubibpc_dn8;
        *var_isubibpc_dn9_slot = var_isubibpc_dn9;
        *var_isubibpc_rv_slot = var_isubibpc_rv;
        *var_iwnqs0_a_slot = var_iwnqs0_a;
        *var_iwnqs0_a_dn0_slot = var_iwnqs0_a_dn0;
        *var_iwnqs0_a_dn10_slot = var_iwnqs0_a_dn10;
        *var_iwnqs0_a_dn11_slot = var_iwnqs0_a_dn11;
        *var_iwnqs0_a_dn14_slot = var_iwnqs0_a_dn14;
        *var_iwnqs0_a_dn18_slot = var_iwnqs0_a_dn18;
        *var_iwnqs0_a_dn2_slot = var_iwnqs0_a_dn2;
        *var_iwnqs0_a_dn4_slot = var_iwnqs0_a_dn4;
        *var_iwnqs0_a_dn5_slot = var_iwnqs0_a_dn5;
        *var_iwnqs0_a_dn6_slot = var_iwnqs0_a_dn6;
        *var_iwnqs0_a_dn7_slot = var_iwnqs0_a_dn7;
        *var_iwnqs0_a_dn8_slot = var_iwnqs0_a_dn8;
        *var_iwnqs0_a_dn9_slot = var_iwnqs0_a_dn9;
        *var_iwnqs0_a_rv_slot = var_iwnqs0_a_rv;
        *var_lover_func_slot = var_lover_func;
        *var_lover_func_dn0_slot = var_lover_func_dn0;
        *var_lover_func_dn10_slot = var_lover_func_dn10;
        *var_lover_func_dn11_slot = var_lover_func_dn11;
        *var_lover_func_dn14_slot = var_lover_func_dn14;
        *var_lover_func_dn2_slot = var_lover_func_dn2;
        *var_lover_func_dn4_slot = var_lover_func_dn4;
        *var_lover_func_dn5_slot = var_lover_func_dn5;
        *var_lover_func_dn6_slot = var_lover_func_dn6;
        *var_lover_func_dn7_slot = var_lover_func_dn7;
        *var_lover_func_dn8_slot = var_lover_func_dn8;
        *var_lover_func_dn9_slot = var_lover_func_dn9;
        *var_lover_func_rv_slot = var_lover_func_rv;
        *var_mfactor_slot = var_mfactor;
        *var_mfactor_rv_slot = var_mfactor_rv;
        *var_q_nqs_a_slot = var_q_nqs_a;
        *var_q_nqs_a_dn16_slot = var_q_nqs_a_dn16;
        *var_q_nqs_a_rv_slot = var_q_nqs_a_rv;
        *var_q_nqs_k_slot = var_q_nqs_k;
        *var_q_nqs_k_dn17_slot = var_q_nqs_k_dn17;
        *var_q_nqs_k_rv_slot = var_q_nqs_k_rv;
        *var_uc_codep_slot = var_uc_codep;
        *var_uc_codep_rv_slot = var_uc_codep_rv;
        *var_uc_corsrd_slot = var_uc_corsrd;
        *var_uc_corsrd_rv_slot = var_uc_corsrd_rv;
        *var_uc_depleak_slot = var_uc_depleak;
        *var_uc_depleak_dn0_slot = var_uc_depleak_dn0;
        *var_uc_depleak_dn10_slot = var_uc_depleak_dn10;
        *var_uc_depleak_dn11_slot = var_uc_depleak_dn11;
        *var_uc_depleak_dn14_slot = var_uc_depleak_dn14;
        *var_uc_depleak_dn2_slot = var_uc_depleak_dn2;
        *var_uc_depleak_dn4_slot = var_uc_depleak_dn4;
        *var_uc_depleak_dn5_slot = var_uc_depleak_dn5;
        *var_uc_depleak_dn6_slot = var_uc_depleak_dn6;
        *var_uc_depleak_dn7_slot = var_uc_depleak_dn7;
        *var_uc_depleak_dn8_slot = var_uc_depleak_dn8;
        *var_uc_depleak_dn9_slot = var_uc_depleak_dn9;
        *var_uc_depleak_rv_slot = var_uc_depleak_rv;
        *var_uc_depmue0_slot = var_uc_depmue0;
        *var_uc_depmue0_dn0_slot = var_uc_depmue0_dn0;
        *var_uc_depmue0_dn10_slot = var_uc_depmue0_dn10;
        *var_uc_depmue0_dn11_slot = var_uc_depmue0_dn11;
        *var_uc_depmue0_dn14_slot = var_uc_depmue0_dn14;
        *var_uc_depmue0_dn2_slot = var_uc_depmue0_dn2;
        *var_uc_depmue0_dn4_slot = var_uc_depmue0_dn4;
        *var_uc_depmue0_dn5_slot = var_uc_depmue0_dn5;
        *var_uc_depmue0_dn6_slot = var_uc_depmue0_dn6;
        *var_uc_depmue0_dn7_slot = var_uc_depmue0_dn7;
        *var_uc_depmue0_dn8_slot = var_uc_depmue0_dn8;
        *var_uc_depmue0_dn9_slot = var_uc_depmue0_dn9;
        *var_uc_depmue0_rv_slot = var_uc_depmue0_rv;
        *var_uc_depmue1_slot = var_uc_depmue1;
        *var_uc_depmue1_dn0_slot = var_uc_depmue1_dn0;
        *var_uc_depmue1_dn10_slot = var_uc_depmue1_dn10;
        *var_uc_depmue1_dn11_slot = var_uc_depmue1_dn11;
        *var_uc_depmue1_dn14_slot = var_uc_depmue1_dn14;
        *var_uc_depmue1_dn2_slot = var_uc_depmue1_dn2;
        *var_uc_depmue1_dn4_slot = var_uc_depmue1_dn4;
        *var_uc_depmue1_dn5_slot = var_uc_depmue1_dn5;
        *var_uc_depmue1_dn6_slot = var_uc_depmue1_dn6;
        *var_uc_depmue1_dn7_slot = var_uc_depmue1_dn7;
        *var_uc_depmue1_dn8_slot = var_uc_depmue1_dn8;
        *var_uc_depmue1_dn9_slot = var_uc_depmue1_dn9;
        *var_uc_depmue1_rv_slot = var_uc_depmue1_rv;
        *var_uc_depmue2_slot = var_uc_depmue2;
        *var_uc_depmue2_dn0_slot = var_uc_depmue2_dn0;
        *var_uc_depmue2_dn10_slot = var_uc_depmue2_dn10;
        *var_uc_depmue2_dn11_slot = var_uc_depmue2_dn11;
        *var_uc_depmue2_dn14_slot = var_uc_depmue2_dn14;
        *var_uc_depmue2_dn2_slot = var_uc_depmue2_dn2;
        *var_uc_depmue2_dn4_slot = var_uc_depmue2_dn4;
        *var_uc_depmue2_dn5_slot = var_uc_depmue2_dn5;
        *var_uc_depmue2_dn6_slot = var_uc_depmue2_dn6;
        *var_uc_depmue2_dn7_slot = var_uc_depmue2_dn7;
        *var_uc_depmue2_dn8_slot = var_uc_depmue2_dn8;
        *var_uc_depmue2_dn9_slot = var_uc_depmue2_dn9;
        *var_uc_depmue2_rv_slot = var_uc_depmue2_rv;
        *var_uc_depmueback0_slot = var_uc_depmueback0;
        *var_uc_depmueback0_dn0_slot = var_uc_depmueback0_dn0;
        *var_uc_depmueback0_dn10_slot = var_uc_depmueback0_dn10;
        *var_uc_depmueback0_dn11_slot = var_uc_depmueback0_dn11;
        *var_uc_depmueback0_dn14_slot = var_uc_depmueback0_dn14;
        *var_uc_depmueback0_dn2_slot = var_uc_depmueback0_dn2;
        *var_uc_depmueback0_dn4_slot = var_uc_depmueback0_dn4;
        *var_uc_depmueback0_dn5_slot = var_uc_depmueback0_dn5;
        *var_uc_depmueback0_dn6_slot = var_uc_depmueback0_dn6;
        *var_uc_depmueback0_dn7_slot = var_uc_depmueback0_dn7;
        *var_uc_depmueback0_dn8_slot = var_uc_depmueback0_dn8;
        *var_uc_depmueback0_dn9_slot = var_uc_depmueback0_dn9;
        *var_uc_depmueback0_rv_slot = var_uc_depmueback0_rv;
        *var_uc_depmueback1_slot = var_uc_depmueback1;
        *var_uc_depmueback1_dn0_slot = var_uc_depmueback1_dn0;
        *var_uc_depmueback1_dn10_slot = var_uc_depmueback1_dn10;
        *var_uc_depmueback1_dn11_slot = var_uc_depmueback1_dn11;
        *var_uc_depmueback1_dn14_slot = var_uc_depmueback1_dn14;
        *var_uc_depmueback1_dn2_slot = var_uc_depmueback1_dn2;
        *var_uc_depmueback1_dn4_slot = var_uc_depmueback1_dn4;
        *var_uc_depmueback1_dn5_slot = var_uc_depmueback1_dn5;
        *var_uc_depmueback1_dn6_slot = var_uc_depmueback1_dn6;
        *var_uc_depmueback1_dn7_slot = var_uc_depmueback1_dn7;
        *var_uc_depmueback1_dn8_slot = var_uc_depmueback1_dn8;
        *var_uc_depmueback1_dn9_slot = var_uc_depmueback1_dn9;
        *var_uc_depmueback1_rv_slot = var_uc_depmueback1_rv;
        *var_uc_depmueph1_slot = var_uc_depmueph1;
        *var_uc_depmueph1_rv_slot = var_uc_depmueph1_rv;
        *var_uc_depthn_slot = var_uc_depthn;
        *var_uc_depthn_dn0_slot = var_uc_depthn_dn0;
        *var_uc_depthn_dn10_slot = var_uc_depthn_dn10;
        *var_uc_depthn_dn11_slot = var_uc_depthn_dn11;
        *var_uc_depthn_dn14_slot = var_uc_depthn_dn14;
        *var_uc_depthn_dn2_slot = var_uc_depthn_dn2;
        *var_uc_depthn_dn4_slot = var_uc_depthn_dn4;
        *var_uc_depthn_dn5_slot = var_uc_depthn_dn5;
        *var_uc_depthn_dn6_slot = var_uc_depthn_dn6;
        *var_uc_depthn_dn7_slot = var_uc_depthn_dn7;
        *var_uc_depthn_dn8_slot = var_uc_depthn_dn8;
        *var_uc_depthn_dn9_slot = var_uc_depthn_dn9;
        *var_uc_depthn_rv_slot = var_uc_depthn_rv;
        *var_uc_depvdsef1_slot = var_uc_depvdsef1;
        *var_uc_depvdsef1_dn0_slot = var_uc_depvdsef1_dn0;
        *var_uc_depvdsef1_dn10_slot = var_uc_depvdsef1_dn10;
        *var_uc_depvdsef1_dn11_slot = var_uc_depvdsef1_dn11;
        *var_uc_depvdsef1_dn14_slot = var_uc_depvdsef1_dn14;
        *var_uc_depvdsef1_dn2_slot = var_uc_depvdsef1_dn2;
        *var_uc_depvdsef1_dn4_slot = var_uc_depvdsef1_dn4;
        *var_uc_depvdsef1_dn5_slot = var_uc_depvdsef1_dn5;
        *var_uc_depvdsef1_dn6_slot = var_uc_depvdsef1_dn6;
        *var_uc_depvdsef1_dn7_slot = var_uc_depvdsef1_dn7;
        *var_uc_depvdsef1_dn8_slot = var_uc_depvdsef1_dn8;
        *var_uc_depvdsef1_dn9_slot = var_uc_depvdsef1_dn9;
        *var_uc_depvdsef1_rv_slot = var_uc_depvdsef1_rv;
        *var_uc_depvdsef2_slot = var_uc_depvdsef2;
        *var_uc_depvdsef2_dn0_slot = var_uc_depvdsef2_dn0;
        *var_uc_depvdsef2_dn10_slot = var_uc_depvdsef2_dn10;
        *var_uc_depvdsef2_dn11_slot = var_uc_depvdsef2_dn11;
        *var_uc_depvdsef2_dn14_slot = var_uc_depvdsef2_dn14;
        *var_uc_depvdsef2_dn2_slot = var_uc_depvdsef2_dn2;
        *var_uc_depvdsef2_dn4_slot = var_uc_depvdsef2_dn4;
        *var_uc_depvdsef2_dn5_slot = var_uc_depvdsef2_dn5;
        *var_uc_depvdsef2_dn6_slot = var_uc_depvdsef2_dn6;
        *var_uc_depvdsef2_dn7_slot = var_uc_depvdsef2_dn7;
        *var_uc_depvdsef2_dn8_slot = var_uc_depvdsef2_dn8;
        *var_uc_depvdsef2_dn9_slot = var_uc_depvdsef2_dn9;
        *var_uc_depvdsef2_rv_slot = var_uc_depvdsef2_rv;
        *var_uc_depvmax_slot = var_uc_depvmax;
        *var_uc_depvmax_dn0_slot = var_uc_depvmax_dn0;
        *var_uc_depvmax_dn10_slot = var_uc_depvmax_dn10;
        *var_uc_depvmax_dn11_slot = var_uc_depvmax_dn11;
        *var_uc_depvmax_dn14_slot = var_uc_depvmax_dn14;
        *var_uc_depvmax_dn2_slot = var_uc_depvmax_dn2;
        *var_uc_depvmax_dn4_slot = var_uc_depvmax_dn4;
        *var_uc_depvmax_dn5_slot = var_uc_depvmax_dn5;
        *var_uc_depvmax_dn6_slot = var_uc_depvmax_dn6;
        *var_uc_depvmax_dn7_slot = var_uc_depvmax_dn7;
        *var_uc_depvmax_dn8_slot = var_uc_depvmax_dn8;
        *var_uc_depvmax_dn9_slot = var_uc_depvmax_dn9;
        *var_uc_depvmax_rv_slot = var_uc_depvmax_rv;
        *var_uc_depwlp_slot = var_uc_depwlp;
        *var_uc_depwlp_dn0_slot = var_uc_depwlp_dn0;
        *var_uc_depwlp_dn10_slot = var_uc_depwlp_dn10;
        *var_uc_depwlp_dn11_slot = var_uc_depwlp_dn11;
        *var_uc_depwlp_dn14_slot = var_uc_depwlp_dn14;
        *var_uc_depwlp_dn2_slot = var_uc_depwlp_dn2;
        *var_uc_depwlp_dn4_slot = var_uc_depwlp_dn4;
        *var_uc_depwlp_dn5_slot = var_uc_depwlp_dn5;
        *var_uc_depwlp_dn6_slot = var_uc_depwlp_dn6;
        *var_uc_depwlp_dn7_slot = var_uc_depwlp_dn7;
        *var_uc_depwlp_dn8_slot = var_uc_depwlp_dn8;
        *var_uc_depwlp_dn9_slot = var_uc_depwlp_dn9;
        *var_uc_depwlp_rv_slot = var_uc_depwlp_rv;
        *var_uc_ndepm_slot = var_uc_ndepm;
        *var_uc_ndepm_dn0_slot = var_uc_ndepm_dn0;
        *var_uc_ndepm_dn10_slot = var_uc_ndepm_dn10;
        *var_uc_ndepm_dn11_slot = var_uc_ndepm_dn11;
        *var_uc_ndepm_dn14_slot = var_uc_ndepm_dn14;
        *var_uc_ndepm_dn2_slot = var_uc_ndepm_dn2;
        *var_uc_ndepm_dn4_slot = var_uc_ndepm_dn4;
        *var_uc_ndepm_dn5_slot = var_uc_ndepm_dn5;
        *var_uc_ndepm_dn6_slot = var_uc_ndepm_dn6;
        *var_uc_ndepm_dn7_slot = var_uc_ndepm_dn7;
        *var_uc_ndepm_dn8_slot = var_uc_ndepm_dn8;
        *var_uc_ndepm_dn9_slot = var_uc_ndepm_dn9;
        *var_uc_ndepm_rv_slot = var_uc_ndepm_rv;
        *var_uc_rdrcx_slot = var_uc_rdrcx;
        *var_uc_rdrcx_rv_slot = var_uc_rdrcx_rv;
        *var_uc_scp22_slot = var_uc_scp22;
        *var_uc_scp22_rv_slot = var_uc_scp22_rv;
        *var_uc_xldld_slot = var_uc_xldld;
        *var_uc_xldld_rv_slot = var_uc_xldld_rv;
        *var_uc_xpdv_slot = var_uc_xpdv;
        *var_uc_xpdv_rv_slot = var_uc_xpdv_rv;
        *var_w_nqs_a_slot = var_w_nqs_a;
        *var_w_nqs_a_dn18_slot = var_w_nqs_a_dn18;
        *var_w_nqs_a_rv_slot = var_w_nqs_a_rv;
        *var_w_res_slot = var_w_res;
        *var_w_res_dn0_slot = var_w_res_dn0;
        *var_w_res_dn10_slot = var_w_res_dn10;
        *var_w_res_dn11_slot = var_w_res_dn11;
        *var_w_res_dn14_slot = var_w_res_dn14;
        *var_w_res_dn2_slot = var_w_res_dn2;
        *var_w_res_dn4_slot = var_w_res_dn4;
        *var_w_res_dn5_slot = var_w_res_dn5;
        *var_w_res_dn6_slot = var_w_res_dn6;
        *var_w_res_dn7_slot = var_w_res_dn7;
        *var_w_res_dn8_slot = var_w_res_dn8;
        *var_w_res_dn9_slot = var_w_res_dn9;
        *var_w_res_rv_slot = var_w_res_rv;
        *var_wdep_func_slot = var_wdep_func;
        *var_wdep_func_dn0_slot = var_wdep_func_dn0;
        *var_wdep_func_dn10_slot = var_wdep_func_dn10;
        *var_wdep_func_dn11_slot = var_wdep_func_dn11;
        *var_wdep_func_dn14_slot = var_wdep_func_dn14;
        *var_wdep_func_dn2_slot = var_wdep_func_dn2;
        *var_wdep_func_dn4_slot = var_wdep_func_dn4;
        *var_wdep_func_dn5_slot = var_wdep_func_dn5;
        *var_wdep_func_dn6_slot = var_wdep_func_dn6;
        *var_wdep_func_dn7_slot = var_wdep_func_dn7;
        *var_wdep_func_dn8_slot = var_wdep_func_dn8;
        *var_wdep_func_dn9_slot = var_wdep_func_dn9;
        *var_wdep_func_rv_slot = var_wdep_func_rv;
        *var_wk_ii_slot = var_wk_ii;
        *var_wk_ii_dn0_slot = var_wk_ii_dn0;
        *var_wk_ii_dn10_slot = var_wk_ii_dn10;
        *var_wk_ii_dn11_slot = var_wk_ii_dn11;
        *var_wk_ii_dn14_slot = var_wk_ii_dn14;
        *var_wk_ii_dn2_slot = var_wk_ii_dn2;
        *var_wk_ii_dn4_slot = var_wk_ii_dn4;
        *var_wk_ii_dn5_slot = var_wk_ii_dn5;
        *var_wk_ii_dn6_slot = var_wk_ii_dn6;
        *var_wk_ii_dn7_slot = var_wk_ii_dn7;
        *var_wk_ii_dn8_slot = var_wk_ii_dn8;
        *var_wk_ii_dn9_slot = var_wk_ii_dn9;
        *var_wk_ii_rv_slot = var_wk_ii_rv;
    }

    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        var_guard112: f64,
        var_guard120: f64,
        var_uc_codep: f64,
        var_guard123_slot: &mut f64,
        var_guard123_rv_slot: &mut f64,
        var_guard124_slot: &mut f64,
        var_guard124_rv_slot: &mut f64,
        var_guard127_slot: &mut f64,
        var_guard127_rv_slot: &mut f64,
        var_guard128_slot: &mut f64,
        var_guard128_rv_slot: &mut f64,
        var_guard131_slot: &mut f64,
        var_guard131_rv_slot: &mut f64,
        var_guard132_slot: &mut f64,
        var_guard132_rv_slot: &mut f64,
        var_guard135_slot: &mut f64,
        var_guard135_rv_slot: &mut f64,
        var_guard136_slot: &mut f64,
        var_guard136_rv_slot: &mut f64,
        var_guard139_slot: &mut f64,
        var_guard139_rv_slot: &mut f64,
        var_guard140_slot: &mut f64,
        var_guard140_rv_slot: &mut f64,
        var_guard141_slot: &mut f64,
        var_guard141_rv_slot: &mut f64,
        var_guard144_slot: &mut f64,
        var_guard144_rv_slot: &mut f64,
        var_guard145_slot: &mut f64,
        var_guard145_rv_slot: &mut f64,
        var_guard148_slot: &mut f64,
        var_guard148_rv_slot: &mut f64,
        var_guard149_slot: &mut f64,
        var_guard149_rv_slot: &mut f64,
        var_guard152_slot: &mut f64,
        var_guard152_rv_slot: &mut f64,
        var_guard153_slot: &mut f64,
        var_guard153_rv_slot: &mut f64,
        var_guard156_slot: &mut f64,
        var_guard156_rv_slot: &mut f64,
        var_guard157_slot: &mut f64,
        var_guard157_rv_slot: &mut f64,
        var_guard160_slot: &mut f64,
        var_guard160_rv_slot: &mut f64,
        var_guard161_slot: &mut f64,
        var_guard161_rv_slot: &mut f64,
        var_guard170_slot: &mut f64,
        var_guard170_rv_slot: &mut f64,
        var_guard171_slot: &mut f64,
        var_guard171_rv_slot: &mut f64,
        var_uc_depleak_slot: &mut f64,
        var_uc_depleak_dn0_slot: &mut f64,
        var_uc_depleak_dn10_slot: &mut f64,
        var_uc_depleak_dn11_slot: &mut f64,
        var_uc_depleak_dn14_slot: &mut f64,
        var_uc_depleak_dn2_slot: &mut f64,
        var_uc_depleak_dn4_slot: &mut f64,
        var_uc_depleak_dn5_slot: &mut f64,
        var_uc_depleak_dn6_slot: &mut f64,
        var_uc_depleak_dn7_slot: &mut f64,
        var_uc_depleak_dn8_slot: &mut f64,
        var_uc_depleak_dn9_slot: &mut f64,
        var_uc_depleak_rv_slot: &mut f64,
        var_uc_depmue0_slot: &mut f64,
        var_uc_depmue0_dn0_slot: &mut f64,
        var_uc_depmue0_dn10_slot: &mut f64,
        var_uc_depmue0_dn11_slot: &mut f64,
        var_uc_depmue0_dn14_slot: &mut f64,
        var_uc_depmue0_dn2_slot: &mut f64,
        var_uc_depmue0_dn4_slot: &mut f64,
        var_uc_depmue0_dn5_slot: &mut f64,
        var_uc_depmue0_dn6_slot: &mut f64,
        var_uc_depmue0_dn7_slot: &mut f64,
        var_uc_depmue0_dn8_slot: &mut f64,
        var_uc_depmue0_dn9_slot: &mut f64,
        var_uc_depmue0_rv_slot: &mut f64,
        var_uc_depmueback0_slot: &mut f64,
        var_uc_depmueback0_dn0_slot: &mut f64,
        var_uc_depmueback0_dn10_slot: &mut f64,
        var_uc_depmueback0_dn11_slot: &mut f64,
        var_uc_depmueback0_dn14_slot: &mut f64,
        var_uc_depmueback0_dn2_slot: &mut f64,
        var_uc_depmueback0_dn4_slot: &mut f64,
        var_uc_depmueback0_dn5_slot: &mut f64,
        var_uc_depmueback0_dn6_slot: &mut f64,
        var_uc_depmueback0_dn7_slot: &mut f64,
        var_uc_depmueback0_dn8_slot: &mut f64,
        var_uc_depmueback0_dn9_slot: &mut f64,
        var_uc_depmueback0_rv_slot: &mut f64,
        var_uc_depmueph1_slot: &mut f64,
        var_uc_depmueph1_rv_slot: &mut f64,
        var_uc_depthn_slot: &mut f64,
        var_uc_depthn_dn0_slot: &mut f64,
        var_uc_depthn_dn10_slot: &mut f64,
        var_uc_depthn_dn11_slot: &mut f64,
        var_uc_depthn_dn14_slot: &mut f64,
        var_uc_depthn_dn2_slot: &mut f64,
        var_uc_depthn_dn4_slot: &mut f64,
        var_uc_depthn_dn5_slot: &mut f64,
        var_uc_depthn_dn6_slot: &mut f64,
        var_uc_depthn_dn7_slot: &mut f64,
        var_uc_depthn_dn8_slot: &mut f64,
        var_uc_depthn_dn9_slot: &mut f64,
        var_uc_depthn_rv_slot: &mut f64,
        var_uc_depvdsef2_slot: &mut f64,
        var_uc_depvdsef2_dn0_slot: &mut f64,
        var_uc_depvdsef2_dn10_slot: &mut f64,
        var_uc_depvdsef2_dn11_slot: &mut f64,
        var_uc_depvdsef2_dn14_slot: &mut f64,
        var_uc_depvdsef2_dn2_slot: &mut f64,
        var_uc_depvdsef2_dn4_slot: &mut f64,
        var_uc_depvdsef2_dn5_slot: &mut f64,
        var_uc_depvdsef2_dn6_slot: &mut f64,
        var_uc_depvdsef2_dn7_slot: &mut f64,
        var_uc_depvdsef2_dn8_slot: &mut f64,
        var_uc_depvdsef2_dn9_slot: &mut f64,
        var_uc_depvdsef2_rv_slot: &mut f64,
        var_uc_ndepm_slot: &mut f64,
        var_uc_ndepm_dn0_slot: &mut f64,
        var_uc_ndepm_dn10_slot: &mut f64,
        var_uc_ndepm_dn11_slot: &mut f64,
        var_uc_ndepm_dn14_slot: &mut f64,
        var_uc_ndepm_dn2_slot: &mut f64,
        var_uc_ndepm_dn4_slot: &mut f64,
        var_uc_ndepm_dn5_slot: &mut f64,
        var_uc_ndepm_dn6_slot: &mut f64,
        var_uc_ndepm_dn7_slot: &mut f64,
        var_uc_ndepm_dn8_slot: &mut f64,
        var_uc_ndepm_dn9_slot: &mut f64,
        var_uc_ndepm_rv_slot: &mut f64,
        var_uc_toxb_slot: &mut f64,
        var_uc_toxb_rv_slot: &mut f64,
    ) {
        let mut var_guard123: f64 = *var_guard123_slot;
        let mut var_guard123_rv: f64 = *var_guard123_rv_slot;
        let mut var_guard124: f64 = *var_guard124_slot;
        let mut var_guard124_rv: f64 = *var_guard124_rv_slot;
        let mut var_guard127: f64 = *var_guard127_slot;
        let mut var_guard127_rv: f64 = *var_guard127_rv_slot;
        let mut var_guard128: f64 = *var_guard128_slot;
        let mut var_guard128_rv: f64 = *var_guard128_rv_slot;
        let mut var_guard131: f64 = *var_guard131_slot;
        let mut var_guard131_rv: f64 = *var_guard131_rv_slot;
        let mut var_guard132: f64 = *var_guard132_slot;
        let mut var_guard132_rv: f64 = *var_guard132_rv_slot;
        let mut var_guard135: f64 = *var_guard135_slot;
        let mut var_guard135_rv: f64 = *var_guard135_rv_slot;
        let mut var_guard136: f64 = *var_guard136_slot;
        let mut var_guard136_rv: f64 = *var_guard136_rv_slot;
        let mut var_guard139: f64 = *var_guard139_slot;
        let mut var_guard139_rv: f64 = *var_guard139_rv_slot;
        let mut var_guard140: f64 = *var_guard140_slot;
        let mut var_guard140_rv: f64 = *var_guard140_rv_slot;
        let mut var_guard141: f64 = *var_guard141_slot;
        let mut var_guard141_rv: f64 = *var_guard141_rv_slot;
        let mut var_guard144: f64 = *var_guard144_slot;
        let mut var_guard144_rv: f64 = *var_guard144_rv_slot;
        let mut var_guard145: f64 = *var_guard145_slot;
        let mut var_guard145_rv: f64 = *var_guard145_rv_slot;
        let mut var_guard148: f64 = *var_guard148_slot;
        let mut var_guard148_rv: f64 = *var_guard148_rv_slot;
        let mut var_guard149: f64 = *var_guard149_slot;
        let mut var_guard149_rv: f64 = *var_guard149_rv_slot;
        let mut var_guard152: f64 = *var_guard152_slot;
        let mut var_guard152_rv: f64 = *var_guard152_rv_slot;
        let mut var_guard153: f64 = *var_guard153_slot;
        let mut var_guard153_rv: f64 = *var_guard153_rv_slot;
        let mut var_guard156: f64 = *var_guard156_slot;
        let mut var_guard156_rv: f64 = *var_guard156_rv_slot;
        let mut var_guard157: f64 = *var_guard157_slot;
        let mut var_guard157_rv: f64 = *var_guard157_rv_slot;
        let mut var_guard160: f64 = *var_guard160_slot;
        let mut var_guard160_rv: f64 = *var_guard160_rv_slot;
        let mut var_guard161: f64 = *var_guard161_slot;
        let mut var_guard161_rv: f64 = *var_guard161_rv_slot;
        let mut var_guard170: f64 = *var_guard170_slot;
        let mut var_guard170_rv: f64 = *var_guard170_rv_slot;
        let mut var_guard171: f64 = *var_guard171_slot;
        let mut var_guard171_rv: f64 = *var_guard171_rv_slot;
        let mut var_uc_depleak: f64 = *var_uc_depleak_slot;
        let mut var_uc_depleak_dn0: f64 = *var_uc_depleak_dn0_slot;
        let mut var_uc_depleak_dn10: f64 = *var_uc_depleak_dn10_slot;
        let mut var_uc_depleak_dn11: f64 = *var_uc_depleak_dn11_slot;
        let mut var_uc_depleak_dn14: f64 = *var_uc_depleak_dn14_slot;
        let mut var_uc_depleak_dn2: f64 = *var_uc_depleak_dn2_slot;
        let mut var_uc_depleak_dn4: f64 = *var_uc_depleak_dn4_slot;
        let mut var_uc_depleak_dn5: f64 = *var_uc_depleak_dn5_slot;
        let mut var_uc_depleak_dn6: f64 = *var_uc_depleak_dn6_slot;
        let mut var_uc_depleak_dn7: f64 = *var_uc_depleak_dn7_slot;
        let mut var_uc_depleak_dn8: f64 = *var_uc_depleak_dn8_slot;
        let mut var_uc_depleak_dn9: f64 = *var_uc_depleak_dn9_slot;
        let mut var_uc_depleak_rv: f64 = *var_uc_depleak_rv_slot;
        let mut var_uc_depmue0: f64 = *var_uc_depmue0_slot;
        let mut var_uc_depmue0_dn0: f64 = *var_uc_depmue0_dn0_slot;
        let mut var_uc_depmue0_dn10: f64 = *var_uc_depmue0_dn10_slot;
        let mut var_uc_depmue0_dn11: f64 = *var_uc_depmue0_dn11_slot;
        let mut var_uc_depmue0_dn14: f64 = *var_uc_depmue0_dn14_slot;
        let mut var_uc_depmue0_dn2: f64 = *var_uc_depmue0_dn2_slot;
        let mut var_uc_depmue0_dn4: f64 = *var_uc_depmue0_dn4_slot;
        let mut var_uc_depmue0_dn5: f64 = *var_uc_depmue0_dn5_slot;
        let mut var_uc_depmue0_dn6: f64 = *var_uc_depmue0_dn6_slot;
        let mut var_uc_depmue0_dn7: f64 = *var_uc_depmue0_dn7_slot;
        let mut var_uc_depmue0_dn8: f64 = *var_uc_depmue0_dn8_slot;
        let mut var_uc_depmue0_dn9: f64 = *var_uc_depmue0_dn9_slot;
        let mut var_uc_depmue0_rv: f64 = *var_uc_depmue0_rv_slot;
        let mut var_uc_depmueback0: f64 = *var_uc_depmueback0_slot;
        let mut var_uc_depmueback0_dn0: f64 = *var_uc_depmueback0_dn0_slot;
        let mut var_uc_depmueback0_dn10: f64 = *var_uc_depmueback0_dn10_slot;
        let mut var_uc_depmueback0_dn11: f64 = *var_uc_depmueback0_dn11_slot;
        let mut var_uc_depmueback0_dn14: f64 = *var_uc_depmueback0_dn14_slot;
        let mut var_uc_depmueback0_dn2: f64 = *var_uc_depmueback0_dn2_slot;
        let mut var_uc_depmueback0_dn4: f64 = *var_uc_depmueback0_dn4_slot;
        let mut var_uc_depmueback0_dn5: f64 = *var_uc_depmueback0_dn5_slot;
        let mut var_uc_depmueback0_dn6: f64 = *var_uc_depmueback0_dn6_slot;
        let mut var_uc_depmueback0_dn7: f64 = *var_uc_depmueback0_dn7_slot;
        let mut var_uc_depmueback0_dn8: f64 = *var_uc_depmueback0_dn8_slot;
        let mut var_uc_depmueback0_dn9: f64 = *var_uc_depmueback0_dn9_slot;
        let mut var_uc_depmueback0_rv: f64 = *var_uc_depmueback0_rv_slot;
        let mut var_uc_depmueph1: f64 = *var_uc_depmueph1_slot;
        let mut var_uc_depmueph1_rv: f64 = *var_uc_depmueph1_rv_slot;
        let mut var_uc_depthn: f64 = *var_uc_depthn_slot;
        let mut var_uc_depthn_dn0: f64 = *var_uc_depthn_dn0_slot;
        let mut var_uc_depthn_dn10: f64 = *var_uc_depthn_dn10_slot;
        let mut var_uc_depthn_dn11: f64 = *var_uc_depthn_dn11_slot;
        let mut var_uc_depthn_dn14: f64 = *var_uc_depthn_dn14_slot;
        let mut var_uc_depthn_dn2: f64 = *var_uc_depthn_dn2_slot;
        let mut var_uc_depthn_dn4: f64 = *var_uc_depthn_dn4_slot;
        let mut var_uc_depthn_dn5: f64 = *var_uc_depthn_dn5_slot;
        let mut var_uc_depthn_dn6: f64 = *var_uc_depthn_dn6_slot;
        let mut var_uc_depthn_dn7: f64 = *var_uc_depthn_dn7_slot;
        let mut var_uc_depthn_dn8: f64 = *var_uc_depthn_dn8_slot;
        let mut var_uc_depthn_dn9: f64 = *var_uc_depthn_dn9_slot;
        let mut var_uc_depthn_rv: f64 = *var_uc_depthn_rv_slot;
        let mut var_uc_depvdsef2: f64 = *var_uc_depvdsef2_slot;
        let mut var_uc_depvdsef2_dn0: f64 = *var_uc_depvdsef2_dn0_slot;
        let mut var_uc_depvdsef2_dn10: f64 = *var_uc_depvdsef2_dn10_slot;
        let mut var_uc_depvdsef2_dn11: f64 = *var_uc_depvdsef2_dn11_slot;
        let mut var_uc_depvdsef2_dn14: f64 = *var_uc_depvdsef2_dn14_slot;
        let mut var_uc_depvdsef2_dn2: f64 = *var_uc_depvdsef2_dn2_slot;
        let mut var_uc_depvdsef2_dn4: f64 = *var_uc_depvdsef2_dn4_slot;
        let mut var_uc_depvdsef2_dn5: f64 = *var_uc_depvdsef2_dn5_slot;
        let mut var_uc_depvdsef2_dn6: f64 = *var_uc_depvdsef2_dn6_slot;
        let mut var_uc_depvdsef2_dn7: f64 = *var_uc_depvdsef2_dn7_slot;
        let mut var_uc_depvdsef2_dn8: f64 = *var_uc_depvdsef2_dn8_slot;
        let mut var_uc_depvdsef2_dn9: f64 = *var_uc_depvdsef2_dn9_slot;
        let mut var_uc_depvdsef2_rv: f64 = *var_uc_depvdsef2_rv_slot;
        let mut var_uc_ndepm: f64 = *var_uc_ndepm_slot;
        let mut var_uc_ndepm_dn0: f64 = *var_uc_ndepm_dn0_slot;
        let mut var_uc_ndepm_dn10: f64 = *var_uc_ndepm_dn10_slot;
        let mut var_uc_ndepm_dn11: f64 = *var_uc_ndepm_dn11_slot;
        let mut var_uc_ndepm_dn14: f64 = *var_uc_ndepm_dn14_slot;
        let mut var_uc_ndepm_dn2: f64 = *var_uc_ndepm_dn2_slot;
        let mut var_uc_ndepm_dn4: f64 = *var_uc_ndepm_dn4_slot;
        let mut var_uc_ndepm_dn5: f64 = *var_uc_ndepm_dn5_slot;
        let mut var_uc_ndepm_dn6: f64 = *var_uc_ndepm_dn6_slot;
        let mut var_uc_ndepm_dn7: f64 = *var_uc_ndepm_dn7_slot;
        let mut var_uc_ndepm_dn8: f64 = *var_uc_ndepm_dn8_slot;
        let mut var_uc_ndepm_dn9: f64 = *var_uc_ndepm_dn9_slot;
        let mut var_uc_ndepm_rv: f64 = *var_uc_ndepm_rv_slot;
        let mut var_uc_toxb: f64 = *var_uc_toxb_slot;
        let mut var_uc_toxb_rv: f64 = *var_uc_toxb_rv_slot;

        let (assign6830_e2766, assign6830_e2766_d_n0, assign6830_e2766_d_n2, assign6830_e2766_d_n4, assign6830_e2766_d_n5, assign6830_e2766_d_n6, assign6830_e2766_d_n7, assign6830_e2766_d_n8, assign6830_e2766_d_n9, assign6830_e2766_d_n10, assign6830_e2766_d_n11, assign6830_e2766_d_n14,) = {
    if ((var_guard112 != 0.0) && (var_guard120 != 0.0)) {
        (1e-6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depthn, var_uc_depthn_dn0, var_uc_depthn_dn2, var_uc_depthn_dn4, var_uc_depthn_dn5, var_uc_depthn_dn6, var_uc_depthn_dn7, var_uc_depthn_dn8, var_uc_depthn_dn9, var_uc_depthn_dn10, var_uc_depthn_dn11, var_uc_depthn_dn14,)
    }
};
        var_uc_depthn = assign6830_e2766;
        var_uc_depthn_dn0 = assign6830_e2766_d_n0;
        var_uc_depthn_dn2 = assign6830_e2766_d_n2;
        var_uc_depthn_dn4 = assign6830_e2766_d_n4;
        var_uc_depthn_dn5 = assign6830_e2766_d_n5;
        var_uc_depthn_dn6 = assign6830_e2766_d_n6;
        var_uc_depthn_dn7 = assign6830_e2766_d_n7;
        var_uc_depthn_dn8 = assign6830_e2766_d_n8;
        var_uc_depthn_dn9 = assign6830_e2766_d_n9;
        var_uc_depthn_dn10 = assign6830_e2766_d_n10;
        var_uc_depthn_dn11 = assign6830_e2766_d_n11;
        var_uc_depthn_dn14 = assign6830_e2766_d_n14;
        var_uc_depthn_rv = 0.0;

        let assign6860_e2779: f64 = if var_uc_depmue0 < 1.0 { 1.0 } else { 0.0 };
        var_guard123 = assign6860_e2779;
        var_guard123_rv = 0.0;

        let (assign6870_e2785, assign6870_e2785_d_n0, assign6870_e2785_d_n2, assign6870_e2785_d_n4, assign6870_e2785_d_n5, assign6870_e2785_d_n6, assign6870_e2785_d_n7, assign6870_e2785_d_n8, assign6870_e2785_d_n9, assign6870_e2785_d_n10, assign6870_e2785_d_n11, assign6870_e2785_d_n14,) = {
    if ((var_guard112 != 0.0) && (var_guard123 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depmue0, var_uc_depmue0_dn0, var_uc_depmue0_dn2, var_uc_depmue0_dn4, var_uc_depmue0_dn5, var_uc_depmue0_dn6, var_uc_depmue0_dn7, var_uc_depmue0_dn8, var_uc_depmue0_dn9, var_uc_depmue0_dn10, var_uc_depmue0_dn11, var_uc_depmue0_dn14,)
    }
};
        var_uc_depmue0 = assign6870_e2785;
        var_uc_depmue0_dn0 = assign6870_e2785_d_n0;
        var_uc_depmue0_dn2 = assign6870_e2785_d_n2;
        var_uc_depmue0_dn4 = assign6870_e2785_d_n4;
        var_uc_depmue0_dn5 = assign6870_e2785_d_n5;
        var_uc_depmue0_dn6 = assign6870_e2785_d_n6;
        var_uc_depmue0_dn7 = assign6870_e2785_d_n7;
        var_uc_depmue0_dn8 = assign6870_e2785_d_n8;
        var_uc_depmue0_dn9 = assign6870_e2785_d_n9;
        var_uc_depmue0_dn10 = assign6870_e2785_d_n10;
        var_uc_depmue0_dn11 = assign6870_e2785_d_n11;
        var_uc_depmue0_dn14 = assign6870_e2785_d_n14;
        var_uc_depmue0_rv = 0.0;

        let assign6880_e2788: f64 = if var_uc_depmue0 > 100000.0 { 1.0 } else { 0.0 };
        var_guard124 = assign6880_e2788;
        var_guard124_rv = 0.0;

        let (assign6890_e2794, assign6890_e2794_d_n0, assign6890_e2794_d_n2, assign6890_e2794_d_n4, assign6890_e2794_d_n5, assign6890_e2794_d_n6, assign6890_e2794_d_n7, assign6890_e2794_d_n8, assign6890_e2794_d_n9, assign6890_e2794_d_n10, assign6890_e2794_d_n11, assign6890_e2794_d_n14,) = {
    if ((var_guard112 != 0.0) && (var_guard124 != 0.0)) {
        (100000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depmue0, var_uc_depmue0_dn0, var_uc_depmue0_dn2, var_uc_depmue0_dn4, var_uc_depmue0_dn5, var_uc_depmue0_dn6, var_uc_depmue0_dn7, var_uc_depmue0_dn8, var_uc_depmue0_dn9, var_uc_depmue0_dn10, var_uc_depmue0_dn11, var_uc_depmue0_dn14,)
    }
};
        var_uc_depmue0 = assign6890_e2794;
        var_uc_depmue0_dn0 = assign6890_e2794_d_n0;
        var_uc_depmue0_dn2 = assign6890_e2794_d_n2;
        var_uc_depmue0_dn4 = assign6890_e2794_d_n4;
        var_uc_depmue0_dn5 = assign6890_e2794_d_n5;
        var_uc_depmue0_dn6 = assign6890_e2794_d_n6;
        var_uc_depmue0_dn7 = assign6890_e2794_d_n7;
        var_uc_depmue0_dn8 = assign6890_e2794_d_n8;
        var_uc_depmue0_dn9 = assign6890_e2794_d_n9;
        var_uc_depmue0_dn10 = assign6890_e2794_d_n10;
        var_uc_depmue0_dn11 = assign6890_e2794_d_n11;
        var_uc_depmue0_dn14 = assign6890_e2794_d_n14;
        var_uc_depmue0_rv = 0.0;

        let assign6920_e2807: f64 = if var_uc_depmueback0 < 1.0 { 1.0 } else { 0.0 };
        var_guard127 = assign6920_e2807;
        var_guard127_rv = 0.0;

        let (assign6930_e2813, assign6930_e2813_d_n0, assign6930_e2813_d_n2, assign6930_e2813_d_n4, assign6930_e2813_d_n5, assign6930_e2813_d_n6, assign6930_e2813_d_n7, assign6930_e2813_d_n8, assign6930_e2813_d_n9, assign6930_e2813_d_n10, assign6930_e2813_d_n11, assign6930_e2813_d_n14,) = {
    if ((var_guard112 != 0.0) && (var_guard127 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depmueback0, var_uc_depmueback0_dn0, var_uc_depmueback0_dn2, var_uc_depmueback0_dn4, var_uc_depmueback0_dn5, var_uc_depmueback0_dn6, var_uc_depmueback0_dn7, var_uc_depmueback0_dn8, var_uc_depmueback0_dn9, var_uc_depmueback0_dn10, var_uc_depmueback0_dn11, var_uc_depmueback0_dn14,)
    }
};
        var_uc_depmueback0 = assign6930_e2813;
        var_uc_depmueback0_dn0 = assign6930_e2813_d_n0;
        var_uc_depmueback0_dn2 = assign6930_e2813_d_n2;
        var_uc_depmueback0_dn4 = assign6930_e2813_d_n4;
        var_uc_depmueback0_dn5 = assign6930_e2813_d_n5;
        var_uc_depmueback0_dn6 = assign6930_e2813_d_n6;
        var_uc_depmueback0_dn7 = assign6930_e2813_d_n7;
        var_uc_depmueback0_dn8 = assign6930_e2813_d_n8;
        var_uc_depmueback0_dn9 = assign6930_e2813_d_n9;
        var_uc_depmueback0_dn10 = assign6930_e2813_d_n10;
        var_uc_depmueback0_dn11 = assign6930_e2813_d_n11;
        var_uc_depmueback0_dn14 = assign6930_e2813_d_n14;
        var_uc_depmueback0_rv = 0.0;

        let assign6940_e2816: f64 = if var_uc_depmueback0 > 100000.0 { 1.0 } else { 0.0 };
        var_guard128 = assign6940_e2816;
        var_guard128_rv = 0.0;

        let (assign6950_e2822, assign6950_e2822_d_n0, assign6950_e2822_d_n2, assign6950_e2822_d_n4, assign6950_e2822_d_n5, assign6950_e2822_d_n6, assign6950_e2822_d_n7, assign6950_e2822_d_n8, assign6950_e2822_d_n9, assign6950_e2822_d_n10, assign6950_e2822_d_n11, assign6950_e2822_d_n14,) = {
    if ((var_guard112 != 0.0) && (var_guard128 != 0.0)) {
        (100000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depmueback0, var_uc_depmueback0_dn0, var_uc_depmueback0_dn2, var_uc_depmueback0_dn4, var_uc_depmueback0_dn5, var_uc_depmueback0_dn6, var_uc_depmueback0_dn7, var_uc_depmueback0_dn8, var_uc_depmueback0_dn9, var_uc_depmueback0_dn10, var_uc_depmueback0_dn11, var_uc_depmueback0_dn14,)
    }
};
        var_uc_depmueback0 = assign6950_e2822;
        var_uc_depmueback0_dn0 = assign6950_e2822_d_n0;
        var_uc_depmueback0_dn2 = assign6950_e2822_d_n2;
        var_uc_depmueback0_dn4 = assign6950_e2822_d_n4;
        var_uc_depmueback0_dn5 = assign6950_e2822_d_n5;
        var_uc_depmueback0_dn6 = assign6950_e2822_d_n6;
        var_uc_depmueback0_dn7 = assign6950_e2822_d_n7;
        var_uc_depmueback0_dn8 = assign6950_e2822_d_n8;
        var_uc_depmueback0_dn9 = assign6950_e2822_d_n9;
        var_uc_depmueback0_dn10 = assign6950_e2822_d_n10;
        var_uc_depmueback0_dn11 = assign6950_e2822_d_n11;
        var_uc_depmueback0_dn14 = assign6950_e2822_d_n14;
        var_uc_depmueback0_rv = 0.0;

        let assign6980_e2835: f64 = if var_uc_depmueph1 < 1.0 { 1.0 } else { 0.0 };
        var_guard131 = assign6980_e2835;
        var_guard131_rv = 0.0;

        let (assign6990_e2841,) = {
    if ((var_guard112 != 0.0) && (var_guard131 != 0.0)) {
        (1.0,)
    } else {
        (var_uc_depmueph1,)
    }
};
        var_uc_depmueph1 = assign6990_e2841;
        var_uc_depmueph1_rv = 0.0;

        let assign7000_e2844: f64 = if var_uc_depmueph1 > 100000.0 { 1.0 } else { 0.0 };
        var_guard132 = assign7000_e2844;
        var_guard132_rv = 0.0;

        let (assign7010_e2850,) = {
    if ((var_guard112 != 0.0) && (var_guard132 != 0.0)) {
        (100000.0,)
    } else {
        (var_uc_depmueph1,)
    }
};
        var_uc_depmueph1 = assign7010_e2850;
        var_uc_depmueph1_rv = 0.0;

        let assign7040_e2863: f64 = if var_uc_depvdsef2 < 0.1 { 1.0 } else { 0.0 };
        var_guard135 = assign7040_e2863;
        var_guard135_rv = 0.0;

        let (assign7050_e2869, assign7050_e2869_d_n0, assign7050_e2869_d_n2, assign7050_e2869_d_n4, assign7050_e2869_d_n5, assign7050_e2869_d_n6, assign7050_e2869_d_n7, assign7050_e2869_d_n8, assign7050_e2869_d_n9, assign7050_e2869_d_n10, assign7050_e2869_d_n11, assign7050_e2869_d_n14,) = {
    if ((var_guard112 != 0.0) && (var_guard135 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depvdsef2, var_uc_depvdsef2_dn0, var_uc_depvdsef2_dn2, var_uc_depvdsef2_dn4, var_uc_depvdsef2_dn5, var_uc_depvdsef2_dn6, var_uc_depvdsef2_dn7, var_uc_depvdsef2_dn8, var_uc_depvdsef2_dn9, var_uc_depvdsef2_dn10, var_uc_depvdsef2_dn11, var_uc_depvdsef2_dn14,)
    }
};
        var_uc_depvdsef2 = assign7050_e2869;
        var_uc_depvdsef2_dn0 = assign7050_e2869_d_n0;
        var_uc_depvdsef2_dn2 = assign7050_e2869_d_n2;
        var_uc_depvdsef2_dn4 = assign7050_e2869_d_n4;
        var_uc_depvdsef2_dn5 = assign7050_e2869_d_n5;
        var_uc_depvdsef2_dn6 = assign7050_e2869_d_n6;
        var_uc_depvdsef2_dn7 = assign7050_e2869_d_n7;
        var_uc_depvdsef2_dn8 = assign7050_e2869_d_n8;
        var_uc_depvdsef2_dn9 = assign7050_e2869_d_n9;
        var_uc_depvdsef2_dn10 = assign7050_e2869_d_n10;
        var_uc_depvdsef2_dn11 = assign7050_e2869_d_n11;
        var_uc_depvdsef2_dn14 = assign7050_e2869_d_n14;
        var_uc_depvdsef2_rv = 0.0;

        let assign7060_e2872: f64 = if var_uc_depvdsef2 > 4.0 { 1.0 } else { 0.0 };
        var_guard136 = assign7060_e2872;
        var_guard136_rv = 0.0;

        let (assign7070_e2878, assign7070_e2878_d_n0, assign7070_e2878_d_n2, assign7070_e2878_d_n4, assign7070_e2878_d_n5, assign7070_e2878_d_n6, assign7070_e2878_d_n7, assign7070_e2878_d_n8, assign7070_e2878_d_n9, assign7070_e2878_d_n10, assign7070_e2878_d_n11, assign7070_e2878_d_n14,) = {
    if ((var_guard112 != 0.0) && (var_guard136 != 0.0)) {
        (4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depvdsef2, var_uc_depvdsef2_dn0, var_uc_depvdsef2_dn2, var_uc_depvdsef2_dn4, var_uc_depvdsef2_dn5, var_uc_depvdsef2_dn6, var_uc_depvdsef2_dn7, var_uc_depvdsef2_dn8, var_uc_depvdsef2_dn9, var_uc_depvdsef2_dn10, var_uc_depvdsef2_dn11, var_uc_depvdsef2_dn14,)
    }
};
        var_uc_depvdsef2 = assign7070_e2878;
        var_uc_depvdsef2_dn0 = assign7070_e2878_d_n0;
        var_uc_depvdsef2_dn2 = assign7070_e2878_d_n2;
        var_uc_depvdsef2_dn4 = assign7070_e2878_d_n4;
        var_uc_depvdsef2_dn5 = assign7070_e2878_d_n5;
        var_uc_depvdsef2_dn6 = assign7070_e2878_d_n6;
        var_uc_depvdsef2_dn7 = assign7070_e2878_d_n7;
        var_uc_depvdsef2_dn8 = assign7070_e2878_d_n8;
        var_uc_depvdsef2_dn9 = assign7070_e2878_d_n9;
        var_uc_depvdsef2_dn10 = assign7070_e2878_d_n10;
        var_uc_depvdsef2_dn11 = assign7070_e2878_d_n11;
        var_uc_depvdsef2_dn14 = assign7070_e2878_d_n14;
        var_uc_depvdsef2_rv = 0.0;

        let assign7100_e2891: f64 = if var_uc_depleak < 0.0 { 1.0 } else { 0.0 };
        var_guard139 = assign7100_e2891;
        var_guard139_rv = 0.0;

        let (assign7110_e2897, assign7110_e2897_d_n0, assign7110_e2897_d_n2, assign7110_e2897_d_n4, assign7110_e2897_d_n5, assign7110_e2897_d_n6, assign7110_e2897_d_n7, assign7110_e2897_d_n8, assign7110_e2897_d_n9, assign7110_e2897_d_n10, assign7110_e2897_d_n11, assign7110_e2897_d_n14,) = {
    if ((var_guard112 != 0.0) && (var_guard139 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depleak, var_uc_depleak_dn0, var_uc_depleak_dn2, var_uc_depleak_dn4, var_uc_depleak_dn5, var_uc_depleak_dn6, var_uc_depleak_dn7, var_uc_depleak_dn8, var_uc_depleak_dn9, var_uc_depleak_dn10, var_uc_depleak_dn11, var_uc_depleak_dn14,)
    }
};
        var_uc_depleak = assign7110_e2897;
        var_uc_depleak_dn0 = assign7110_e2897_d_n0;
        var_uc_depleak_dn2 = assign7110_e2897_d_n2;
        var_uc_depleak_dn4 = assign7110_e2897_d_n4;
        var_uc_depleak_dn5 = assign7110_e2897_d_n5;
        var_uc_depleak_dn6 = assign7110_e2897_d_n6;
        var_uc_depleak_dn7 = assign7110_e2897_d_n7;
        var_uc_depleak_dn8 = assign7110_e2897_d_n8;
        var_uc_depleak_dn9 = assign7110_e2897_d_n9;
        var_uc_depleak_dn10 = assign7110_e2897_d_n10;
        var_uc_depleak_dn11 = assign7110_e2897_d_n11;
        var_uc_depleak_dn14 = assign7110_e2897_d_n14;
        var_uc_depleak_rv = 0.0;

        let assign7120_e2900: f64 = if var_uc_depleak > 5.0 { 1.0 } else { 0.0 };
        var_guard140 = assign7120_e2900;
        var_guard140_rv = 0.0;

        let (assign7130_e2906, assign7130_e2906_d_n0, assign7130_e2906_d_n2, assign7130_e2906_d_n4, assign7130_e2906_d_n5, assign7130_e2906_d_n6, assign7130_e2906_d_n7, assign7130_e2906_d_n8, assign7130_e2906_d_n9, assign7130_e2906_d_n10, assign7130_e2906_d_n11, assign7130_e2906_d_n14,) = {
    if ((var_guard112 != 0.0) && (var_guard140 != 0.0)) {
        (5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depleak, var_uc_depleak_dn0, var_uc_depleak_dn2, var_uc_depleak_dn4, var_uc_depleak_dn5, var_uc_depleak_dn6, var_uc_depleak_dn7, var_uc_depleak_dn8, var_uc_depleak_dn9, var_uc_depleak_dn10, var_uc_depleak_dn11, var_uc_depleak_dn14,)
    }
};
        var_uc_depleak = assign7130_e2906;
        var_uc_depleak_dn0 = assign7130_e2906_d_n0;
        var_uc_depleak_dn2 = assign7130_e2906_d_n2;
        var_uc_depleak_dn4 = assign7130_e2906_d_n4;
        var_uc_depleak_dn5 = assign7130_e2906_d_n5;
        var_uc_depleak_dn6 = assign7130_e2906_d_n6;
        var_uc_depleak_dn7 = assign7130_e2906_d_n7;
        var_uc_depleak_dn8 = assign7130_e2906_d_n8;
        var_uc_depleak_dn9 = assign7130_e2906_d_n9;
        var_uc_depleak_dn10 = assign7130_e2906_d_n10;
        var_uc_depleak_dn11 = assign7130_e2906_d_n11;
        var_uc_depleak_dn14 = assign7130_e2906_d_n14;
        var_uc_depleak_rv = 0.0;

        let assign7140_e2909: f64 = if var_uc_codep == 3.0 { 1.0 } else { 0.0 };
        var_guard141 = assign7140_e2909;
        var_guard141_rv = 0.0;

        let assign7170_e2922: f64 = if var_uc_ndepm < 5000000000000000.0 { 1.0 } else { 0.0 };
        var_guard144 = assign7170_e2922;
        var_guard144_rv = 0.0;

        let (assign7180_e2931, assign7180_e2931_d_n0, assign7180_e2931_d_n2, assign7180_e2931_d_n4, assign7180_e2931_d_n5, assign7180_e2931_d_n6, assign7180_e2931_d_n7, assign7180_e2931_d_n8, assign7180_e2931_d_n9, assign7180_e2931_d_n10, assign7180_e2931_d_n11, assign7180_e2931_d_n14,) = {
    if (((var_guard112 == 0.0) && (var_guard141 != 0.0)) && (var_guard144 != 0.0)) {
        (5000000000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_ndepm, var_uc_ndepm_dn0, var_uc_ndepm_dn2, var_uc_ndepm_dn4, var_uc_ndepm_dn5, var_uc_ndepm_dn6, var_uc_ndepm_dn7, var_uc_ndepm_dn8, var_uc_ndepm_dn9, var_uc_ndepm_dn10, var_uc_ndepm_dn11, var_uc_ndepm_dn14,)
    }
};
        var_uc_ndepm = assign7180_e2931;
        var_uc_ndepm_dn0 = assign7180_e2931_d_n0;
        var_uc_ndepm_dn2 = assign7180_e2931_d_n2;
        var_uc_ndepm_dn4 = assign7180_e2931_d_n4;
        var_uc_ndepm_dn5 = assign7180_e2931_d_n5;
        var_uc_ndepm_dn6 = assign7180_e2931_d_n6;
        var_uc_ndepm_dn7 = assign7180_e2931_d_n7;
        var_uc_ndepm_dn8 = assign7180_e2931_d_n8;
        var_uc_ndepm_dn9 = assign7180_e2931_d_n9;
        var_uc_ndepm_dn10 = assign7180_e2931_d_n10;
        var_uc_ndepm_dn11 = assign7180_e2931_d_n11;
        var_uc_ndepm_dn14 = assign7180_e2931_d_n14;
        var_uc_ndepm_rv = 0.0;

        let assign7190_e2934: f64 = if var_uc_ndepm > 1e18 { 1.0 } else { 0.0 };
        var_guard145 = assign7190_e2934;
        var_guard145_rv = 0.0;

        let (assign7200_e2943, assign7200_e2943_d_n0, assign7200_e2943_d_n2, assign7200_e2943_d_n4, assign7200_e2943_d_n5, assign7200_e2943_d_n6, assign7200_e2943_d_n7, assign7200_e2943_d_n8, assign7200_e2943_d_n9, assign7200_e2943_d_n10, assign7200_e2943_d_n11, assign7200_e2943_d_n14,) = {
    if (((var_guard112 == 0.0) && (var_guard141 != 0.0)) && (var_guard145 != 0.0)) {
        (1e18, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_ndepm, var_uc_ndepm_dn0, var_uc_ndepm_dn2, var_uc_ndepm_dn4, var_uc_ndepm_dn5, var_uc_ndepm_dn6, var_uc_ndepm_dn7, var_uc_ndepm_dn8, var_uc_ndepm_dn9, var_uc_ndepm_dn10, var_uc_ndepm_dn11, var_uc_ndepm_dn14,)
    }
};
        var_uc_ndepm = assign7200_e2943;
        var_uc_ndepm_dn0 = assign7200_e2943_d_n0;
        var_uc_ndepm_dn2 = assign7200_e2943_d_n2;
        var_uc_ndepm_dn4 = assign7200_e2943_d_n4;
        var_uc_ndepm_dn5 = assign7200_e2943_d_n5;
        var_uc_ndepm_dn6 = assign7200_e2943_d_n6;
        var_uc_ndepm_dn7 = assign7200_e2943_d_n7;
        var_uc_ndepm_dn8 = assign7200_e2943_d_n8;
        var_uc_ndepm_dn9 = assign7200_e2943_d_n9;
        var_uc_ndepm_dn10 = assign7200_e2943_d_n10;
        var_uc_ndepm_dn11 = assign7200_e2943_d_n11;
        var_uc_ndepm_dn14 = assign7200_e2943_d_n14;
        var_uc_ndepm_rv = 0.0;

        let assign7230_e2956: f64 = if var_uc_depthn < 1e-8 { 1.0 } else { 0.0 };
        var_guard148 = assign7230_e2956;
        var_guard148_rv = 0.0;

        let (assign7240_e2965, assign7240_e2965_d_n0, assign7240_e2965_d_n2, assign7240_e2965_d_n4, assign7240_e2965_d_n5, assign7240_e2965_d_n6, assign7240_e2965_d_n7, assign7240_e2965_d_n8, assign7240_e2965_d_n9, assign7240_e2965_d_n10, assign7240_e2965_d_n11, assign7240_e2965_d_n14,) = {
    if (((var_guard112 == 0.0) && (var_guard141 != 0.0)) && (var_guard148 != 0.0)) {
        (1e-8, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depthn, var_uc_depthn_dn0, var_uc_depthn_dn2, var_uc_depthn_dn4, var_uc_depthn_dn5, var_uc_depthn_dn6, var_uc_depthn_dn7, var_uc_depthn_dn8, var_uc_depthn_dn9, var_uc_depthn_dn10, var_uc_depthn_dn11, var_uc_depthn_dn14,)
    }
};
        var_uc_depthn = assign7240_e2965;
        var_uc_depthn_dn0 = assign7240_e2965_d_n0;
        var_uc_depthn_dn2 = assign7240_e2965_d_n2;
        var_uc_depthn_dn4 = assign7240_e2965_d_n4;
        var_uc_depthn_dn5 = assign7240_e2965_d_n5;
        var_uc_depthn_dn6 = assign7240_e2965_d_n6;
        var_uc_depthn_dn7 = assign7240_e2965_d_n7;
        var_uc_depthn_dn8 = assign7240_e2965_d_n8;
        var_uc_depthn_dn9 = assign7240_e2965_d_n9;
        var_uc_depthn_dn10 = assign7240_e2965_d_n10;
        var_uc_depthn_dn11 = assign7240_e2965_d_n11;
        var_uc_depthn_dn14 = assign7240_e2965_d_n14;
        var_uc_depthn_rv = 0.0;

        let assign7250_e2968: f64 = if var_uc_depthn > 1e-6 { 1.0 } else { 0.0 };
        var_guard149 = assign7250_e2968;
        var_guard149_rv = 0.0;

        let (assign7260_e2977, assign7260_e2977_d_n0, assign7260_e2977_d_n2, assign7260_e2977_d_n4, assign7260_e2977_d_n5, assign7260_e2977_d_n6, assign7260_e2977_d_n7, assign7260_e2977_d_n8, assign7260_e2977_d_n9, assign7260_e2977_d_n10, assign7260_e2977_d_n11, assign7260_e2977_d_n14,) = {
    if (((var_guard112 == 0.0) && (var_guard141 != 0.0)) && (var_guard149 != 0.0)) {
        (1e-6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depthn, var_uc_depthn_dn0, var_uc_depthn_dn2, var_uc_depthn_dn4, var_uc_depthn_dn5, var_uc_depthn_dn6, var_uc_depthn_dn7, var_uc_depthn_dn8, var_uc_depthn_dn9, var_uc_depthn_dn10, var_uc_depthn_dn11, var_uc_depthn_dn14,)
    }
};
        var_uc_depthn = assign7260_e2977;
        var_uc_depthn_dn0 = assign7260_e2977_d_n0;
        var_uc_depthn_dn2 = assign7260_e2977_d_n2;
        var_uc_depthn_dn4 = assign7260_e2977_d_n4;
        var_uc_depthn_dn5 = assign7260_e2977_d_n5;
        var_uc_depthn_dn6 = assign7260_e2977_d_n6;
        var_uc_depthn_dn7 = assign7260_e2977_d_n7;
        var_uc_depthn_dn8 = assign7260_e2977_d_n8;
        var_uc_depthn_dn9 = assign7260_e2977_d_n9;
        var_uc_depthn_dn10 = assign7260_e2977_d_n10;
        var_uc_depthn_dn11 = assign7260_e2977_d_n11;
        var_uc_depthn_dn14 = assign7260_e2977_d_n14;
        var_uc_depthn_rv = 0.0;

        let assign7290_e2990: f64 = if var_uc_depmue0 < 1.0 { 1.0 } else { 0.0 };
        var_guard152 = assign7290_e2990;
        var_guard152_rv = 0.0;

        let (assign7300_e2999, assign7300_e2999_d_n0, assign7300_e2999_d_n2, assign7300_e2999_d_n4, assign7300_e2999_d_n5, assign7300_e2999_d_n6, assign7300_e2999_d_n7, assign7300_e2999_d_n8, assign7300_e2999_d_n9, assign7300_e2999_d_n10, assign7300_e2999_d_n11, assign7300_e2999_d_n14,) = {
    if (((var_guard112 == 0.0) && (var_guard141 != 0.0)) && (var_guard152 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depmue0, var_uc_depmue0_dn0, var_uc_depmue0_dn2, var_uc_depmue0_dn4, var_uc_depmue0_dn5, var_uc_depmue0_dn6, var_uc_depmue0_dn7, var_uc_depmue0_dn8, var_uc_depmue0_dn9, var_uc_depmue0_dn10, var_uc_depmue0_dn11, var_uc_depmue0_dn14,)
    }
};
        var_uc_depmue0 = assign7300_e2999;
        var_uc_depmue0_dn0 = assign7300_e2999_d_n0;
        var_uc_depmue0_dn2 = assign7300_e2999_d_n2;
        var_uc_depmue0_dn4 = assign7300_e2999_d_n4;
        var_uc_depmue0_dn5 = assign7300_e2999_d_n5;
        var_uc_depmue0_dn6 = assign7300_e2999_d_n6;
        var_uc_depmue0_dn7 = assign7300_e2999_d_n7;
        var_uc_depmue0_dn8 = assign7300_e2999_d_n8;
        var_uc_depmue0_dn9 = assign7300_e2999_d_n9;
        var_uc_depmue0_dn10 = assign7300_e2999_d_n10;
        var_uc_depmue0_dn11 = assign7300_e2999_d_n11;
        var_uc_depmue0_dn14 = assign7300_e2999_d_n14;
        var_uc_depmue0_rv = 0.0;

        let assign7310_e3002: f64 = if var_uc_depmue0 > 10000000000.0 { 1.0 } else { 0.0 };
        var_guard153 = assign7310_e3002;
        var_guard153_rv = 0.0;

        let (assign7320_e3011, assign7320_e3011_d_n0, assign7320_e3011_d_n2, assign7320_e3011_d_n4, assign7320_e3011_d_n5, assign7320_e3011_d_n6, assign7320_e3011_d_n7, assign7320_e3011_d_n8, assign7320_e3011_d_n9, assign7320_e3011_d_n10, assign7320_e3011_d_n11, assign7320_e3011_d_n14,) = {
    if (((var_guard112 == 0.0) && (var_guard141 != 0.0)) && (var_guard153 != 0.0)) {
        (10000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depmue0, var_uc_depmue0_dn0, var_uc_depmue0_dn2, var_uc_depmue0_dn4, var_uc_depmue0_dn5, var_uc_depmue0_dn6, var_uc_depmue0_dn7, var_uc_depmue0_dn8, var_uc_depmue0_dn9, var_uc_depmue0_dn10, var_uc_depmue0_dn11, var_uc_depmue0_dn14,)
    }
};
        var_uc_depmue0 = assign7320_e3011;
        var_uc_depmue0_dn0 = assign7320_e3011_d_n0;
        var_uc_depmue0_dn2 = assign7320_e3011_d_n2;
        var_uc_depmue0_dn4 = assign7320_e3011_d_n4;
        var_uc_depmue0_dn5 = assign7320_e3011_d_n5;
        var_uc_depmue0_dn6 = assign7320_e3011_d_n6;
        var_uc_depmue0_dn7 = assign7320_e3011_d_n7;
        var_uc_depmue0_dn8 = assign7320_e3011_d_n8;
        var_uc_depmue0_dn9 = assign7320_e3011_d_n9;
        var_uc_depmue0_dn10 = assign7320_e3011_d_n10;
        var_uc_depmue0_dn11 = assign7320_e3011_d_n11;
        var_uc_depmue0_dn14 = assign7320_e3011_d_n14;
        var_uc_depmue0_rv = 0.0;

        let assign7350_e3024: f64 = if var_uc_depmueph1 < 100.0 { 1.0 } else { 0.0 };
        var_guard156 = assign7350_e3024;
        var_guard156_rv = 0.0;

        let (assign7360_e3033,) = {
    if (((var_guard112 == 0.0) && (var_guard141 != 0.0)) && (var_guard156 != 0.0)) {
        (100.0,)
    } else {
        (var_uc_depmueph1,)
    }
};
        var_uc_depmueph1 = assign7360_e3033;
        var_uc_depmueph1_rv = 0.0;

        let assign7370_e3036: f64 = if var_uc_depmueph1 > 2000000000.0 { 1.0 } else { 0.0 };
        var_guard157 = assign7370_e3036;
        var_guard157_rv = 0.0;

        let (assign7380_e3045,) = {
    if (((var_guard112 == 0.0) && (var_guard141 != 0.0)) && (var_guard157 != 0.0)) {
        (2000000000.0,)
    } else {
        (var_uc_depmueph1,)
    }
};
        var_uc_depmueph1 = assign7380_e3045;
        var_uc_depmueph1_rv = 0.0;

        let assign7410_e3058: f64 = if var_uc_depleak < 0.0 { 1.0 } else { 0.0 };
        var_guard160 = assign7410_e3058;
        var_guard160_rv = 0.0;

        let (assign7420_e3067, assign7420_e3067_d_n0, assign7420_e3067_d_n2, assign7420_e3067_d_n4, assign7420_e3067_d_n5, assign7420_e3067_d_n6, assign7420_e3067_d_n7, assign7420_e3067_d_n8, assign7420_e3067_d_n9, assign7420_e3067_d_n10, assign7420_e3067_d_n11, assign7420_e3067_d_n14,) = {
    if (((var_guard112 == 0.0) && (var_guard141 != 0.0)) && (var_guard160 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depleak, var_uc_depleak_dn0, var_uc_depleak_dn2, var_uc_depleak_dn4, var_uc_depleak_dn5, var_uc_depleak_dn6, var_uc_depleak_dn7, var_uc_depleak_dn8, var_uc_depleak_dn9, var_uc_depleak_dn10, var_uc_depleak_dn11, var_uc_depleak_dn14,)
    }
};
        var_uc_depleak = assign7420_e3067;
        var_uc_depleak_dn0 = assign7420_e3067_d_n0;
        var_uc_depleak_dn2 = assign7420_e3067_d_n2;
        var_uc_depleak_dn4 = assign7420_e3067_d_n4;
        var_uc_depleak_dn5 = assign7420_e3067_d_n5;
        var_uc_depleak_dn6 = assign7420_e3067_d_n6;
        var_uc_depleak_dn7 = assign7420_e3067_d_n7;
        var_uc_depleak_dn8 = assign7420_e3067_d_n8;
        var_uc_depleak_dn9 = assign7420_e3067_d_n9;
        var_uc_depleak_dn10 = assign7420_e3067_d_n10;
        var_uc_depleak_dn11 = assign7420_e3067_d_n11;
        var_uc_depleak_dn14 = assign7420_e3067_d_n14;
        var_uc_depleak_rv = 0.0;

        let assign7430_e3070: f64 = if var_uc_depleak > 5.0 { 1.0 } else { 0.0 };
        var_guard161 = assign7430_e3070;
        var_guard161_rv = 0.0;

        let (assign7440_e3079, assign7440_e3079_d_n0, assign7440_e3079_d_n2, assign7440_e3079_d_n4, assign7440_e3079_d_n5, assign7440_e3079_d_n6, assign7440_e3079_d_n7, assign7440_e3079_d_n8, assign7440_e3079_d_n9, assign7440_e3079_d_n10, assign7440_e3079_d_n11, assign7440_e3079_d_n14,) = {
    if (((var_guard112 == 0.0) && (var_guard141 != 0.0)) && (var_guard161 != 0.0)) {
        (5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depleak, var_uc_depleak_dn0, var_uc_depleak_dn2, var_uc_depleak_dn4, var_uc_depleak_dn5, var_uc_depleak_dn6, var_uc_depleak_dn7, var_uc_depleak_dn8, var_uc_depleak_dn9, var_uc_depleak_dn10, var_uc_depleak_dn11, var_uc_depleak_dn14,)
    }
};
        var_uc_depleak = assign7440_e3079;
        var_uc_depleak_dn0 = assign7440_e3079_d_n0;
        var_uc_depleak_dn2 = assign7440_e3079_d_n2;
        var_uc_depleak_dn4 = assign7440_e3079_d_n4;
        var_uc_depleak_dn5 = assign7440_e3079_d_n5;
        var_uc_depleak_dn6 = assign7440_e3079_d_n6;
        var_uc_depleak_dn7 = assign7440_e3079_d_n7;
        var_uc_depleak_dn8 = assign7440_e3079_d_n8;
        var_uc_depleak_dn9 = assign7440_e3079_d_n9;
        var_uc_depleak_dn10 = assign7440_e3079_d_n10;
        var_uc_depleak_dn11 = assign7440_e3079_d_n11;
        var_uc_depleak_dn14 = assign7440_e3079_d_n14;
        var_uc_depleak_rv = 0.0;

        var_uc_toxb = p.p96;
        var_uc_toxb_rv = 0.0;

        let assign7540_e3117: f64 = if var_uc_toxb < p.p95 { 1.0 } else { 0.0 };
        var_guard170 = assign7540_e3117;
        var_guard170_rv = 0.0;

        let (assign7550_e3121,) = {
    if (var_guard170 != 0.0) {
        (p.p95,)
    } else {
        (var_uc_toxb,)
    }
};
        var_uc_toxb = assign7550_e3121;
        var_uc_toxb_rv = 0.0;

        let assign7560_e3124: f64 = if var_uc_toxb > 5e-7 { 1.0 } else { 0.0 };
        var_guard171 = assign7560_e3124;
        var_guard171_rv = 0.0;

        let (assign7570_e3128,) = {
    if (var_guard171 != 0.0) {
        (5e-7,)
    } else {
        (var_uc_toxb,)
    }
};
        var_uc_toxb = assign7570_e3128;
        var_uc_toxb_rv = 0.0;

        *var_guard123_slot = var_guard123;
        *var_guard123_rv_slot = var_guard123_rv;
        *var_guard124_slot = var_guard124;
        *var_guard124_rv_slot = var_guard124_rv;
        *var_guard127_slot = var_guard127;
        *var_guard127_rv_slot = var_guard127_rv;
        *var_guard128_slot = var_guard128;
        *var_guard128_rv_slot = var_guard128_rv;
        *var_guard131_slot = var_guard131;
        *var_guard131_rv_slot = var_guard131_rv;
        *var_guard132_slot = var_guard132;
        *var_guard132_rv_slot = var_guard132_rv;
        *var_guard135_slot = var_guard135;
        *var_guard135_rv_slot = var_guard135_rv;
        *var_guard136_slot = var_guard136;
        *var_guard136_rv_slot = var_guard136_rv;
        *var_guard139_slot = var_guard139;
        *var_guard139_rv_slot = var_guard139_rv;
        *var_guard140_slot = var_guard140;
        *var_guard140_rv_slot = var_guard140_rv;
        *var_guard141_slot = var_guard141;
        *var_guard141_rv_slot = var_guard141_rv;
        *var_guard144_slot = var_guard144;
        *var_guard144_rv_slot = var_guard144_rv;
        *var_guard145_slot = var_guard145;
        *var_guard145_rv_slot = var_guard145_rv;
        *var_guard148_slot = var_guard148;
        *var_guard148_rv_slot = var_guard148_rv;
        *var_guard149_slot = var_guard149;
        *var_guard149_rv_slot = var_guard149_rv;
        *var_guard152_slot = var_guard152;
        *var_guard152_rv_slot = var_guard152_rv;
        *var_guard153_slot = var_guard153;
        *var_guard153_rv_slot = var_guard153_rv;
        *var_guard156_slot = var_guard156;
        *var_guard156_rv_slot = var_guard156_rv;
        *var_guard157_slot = var_guard157;
        *var_guard157_rv_slot = var_guard157_rv;
        *var_guard160_slot = var_guard160;
        *var_guard160_rv_slot = var_guard160_rv;
        *var_guard161_slot = var_guard161;
        *var_guard161_rv_slot = var_guard161_rv;
        *var_guard170_slot = var_guard170;
        *var_guard170_rv_slot = var_guard170_rv;
        *var_guard171_slot = var_guard171;
        *var_guard171_rv_slot = var_guard171_rv;
        *var_uc_depleak_slot = var_uc_depleak;
        *var_uc_depleak_dn0_slot = var_uc_depleak_dn0;
        *var_uc_depleak_dn10_slot = var_uc_depleak_dn10;
        *var_uc_depleak_dn11_slot = var_uc_depleak_dn11;
        *var_uc_depleak_dn14_slot = var_uc_depleak_dn14;
        *var_uc_depleak_dn2_slot = var_uc_depleak_dn2;
        *var_uc_depleak_dn4_slot = var_uc_depleak_dn4;
        *var_uc_depleak_dn5_slot = var_uc_depleak_dn5;
        *var_uc_depleak_dn6_slot = var_uc_depleak_dn6;
        *var_uc_depleak_dn7_slot = var_uc_depleak_dn7;
        *var_uc_depleak_dn8_slot = var_uc_depleak_dn8;
        *var_uc_depleak_dn9_slot = var_uc_depleak_dn9;
        *var_uc_depleak_rv_slot = var_uc_depleak_rv;
        *var_uc_depmue0_slot = var_uc_depmue0;
        *var_uc_depmue0_dn0_slot = var_uc_depmue0_dn0;
        *var_uc_depmue0_dn10_slot = var_uc_depmue0_dn10;
        *var_uc_depmue0_dn11_slot = var_uc_depmue0_dn11;
        *var_uc_depmue0_dn14_slot = var_uc_depmue0_dn14;
        *var_uc_depmue0_dn2_slot = var_uc_depmue0_dn2;
        *var_uc_depmue0_dn4_slot = var_uc_depmue0_dn4;
        *var_uc_depmue0_dn5_slot = var_uc_depmue0_dn5;
        *var_uc_depmue0_dn6_slot = var_uc_depmue0_dn6;
        *var_uc_depmue0_dn7_slot = var_uc_depmue0_dn7;
        *var_uc_depmue0_dn8_slot = var_uc_depmue0_dn8;
        *var_uc_depmue0_dn9_slot = var_uc_depmue0_dn9;
        *var_uc_depmue0_rv_slot = var_uc_depmue0_rv;
        *var_uc_depmueback0_slot = var_uc_depmueback0;
        *var_uc_depmueback0_dn0_slot = var_uc_depmueback0_dn0;
        *var_uc_depmueback0_dn10_slot = var_uc_depmueback0_dn10;
        *var_uc_depmueback0_dn11_slot = var_uc_depmueback0_dn11;
        *var_uc_depmueback0_dn14_slot = var_uc_depmueback0_dn14;
        *var_uc_depmueback0_dn2_slot = var_uc_depmueback0_dn2;
        *var_uc_depmueback0_dn4_slot = var_uc_depmueback0_dn4;
        *var_uc_depmueback0_dn5_slot = var_uc_depmueback0_dn5;
        *var_uc_depmueback0_dn6_slot = var_uc_depmueback0_dn6;
        *var_uc_depmueback0_dn7_slot = var_uc_depmueback0_dn7;
        *var_uc_depmueback0_dn8_slot = var_uc_depmueback0_dn8;
        *var_uc_depmueback0_dn9_slot = var_uc_depmueback0_dn9;
        *var_uc_depmueback0_rv_slot = var_uc_depmueback0_rv;
        *var_uc_depmueph1_slot = var_uc_depmueph1;
        *var_uc_depmueph1_rv_slot = var_uc_depmueph1_rv;
        *var_uc_depthn_slot = var_uc_depthn;
        *var_uc_depthn_dn0_slot = var_uc_depthn_dn0;
        *var_uc_depthn_dn10_slot = var_uc_depthn_dn10;
        *var_uc_depthn_dn11_slot = var_uc_depthn_dn11;
        *var_uc_depthn_dn14_slot = var_uc_depthn_dn14;
        *var_uc_depthn_dn2_slot = var_uc_depthn_dn2;
        *var_uc_depthn_dn4_slot = var_uc_depthn_dn4;
        *var_uc_depthn_dn5_slot = var_uc_depthn_dn5;
        *var_uc_depthn_dn6_slot = var_uc_depthn_dn6;
        *var_uc_depthn_dn7_slot = var_uc_depthn_dn7;
        *var_uc_depthn_dn8_slot = var_uc_depthn_dn8;
        *var_uc_depthn_dn9_slot = var_uc_depthn_dn9;
        *var_uc_depthn_rv_slot = var_uc_depthn_rv;
        *var_uc_depvdsef2_slot = var_uc_depvdsef2;
        *var_uc_depvdsef2_dn0_slot = var_uc_depvdsef2_dn0;
        *var_uc_depvdsef2_dn10_slot = var_uc_depvdsef2_dn10;
        *var_uc_depvdsef2_dn11_slot = var_uc_depvdsef2_dn11;
        *var_uc_depvdsef2_dn14_slot = var_uc_depvdsef2_dn14;
        *var_uc_depvdsef2_dn2_slot = var_uc_depvdsef2_dn2;
        *var_uc_depvdsef2_dn4_slot = var_uc_depvdsef2_dn4;
        *var_uc_depvdsef2_dn5_slot = var_uc_depvdsef2_dn5;
        *var_uc_depvdsef2_dn6_slot = var_uc_depvdsef2_dn6;
        *var_uc_depvdsef2_dn7_slot = var_uc_depvdsef2_dn7;
        *var_uc_depvdsef2_dn8_slot = var_uc_depvdsef2_dn8;
        *var_uc_depvdsef2_dn9_slot = var_uc_depvdsef2_dn9;
        *var_uc_depvdsef2_rv_slot = var_uc_depvdsef2_rv;
        *var_uc_ndepm_slot = var_uc_ndepm;
        *var_uc_ndepm_dn0_slot = var_uc_ndepm_dn0;
        *var_uc_ndepm_dn10_slot = var_uc_ndepm_dn10;
        *var_uc_ndepm_dn11_slot = var_uc_ndepm_dn11;
        *var_uc_ndepm_dn14_slot = var_uc_ndepm_dn14;
        *var_uc_ndepm_dn2_slot = var_uc_ndepm_dn2;
        *var_uc_ndepm_dn4_slot = var_uc_ndepm_dn4;
        *var_uc_ndepm_dn5_slot = var_uc_ndepm_dn5;
        *var_uc_ndepm_dn6_slot = var_uc_ndepm_dn6;
        *var_uc_ndepm_dn7_slot = var_uc_ndepm_dn7;
        *var_uc_ndepm_dn8_slot = var_uc_ndepm_dn8;
        *var_uc_ndepm_dn9_slot = var_uc_ndepm_dn9;
        *var_uc_ndepm_rv_slot = var_uc_ndepm_rv;
        *var_uc_toxb_slot = var_uc_toxb;
        *var_uc_toxb_rv_slot = var_uc_toxb_rv;
    }

    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        var_ktnom_slot: &mut f64,
        var_ktnom_rv_slot: &mut f64,
        var_lbin_slot: &mut f64,
        var_lbin_rv_slot: &mut f64,
        var_lg_slot: &mut f64,
        var_lg_rv_slot: &mut f64,
        var_lgate_slot: &mut f64,
        var_lgate_rv_slot: &mut f64,
        var_lwbin_slot: &mut f64,
        var_lwbin_rv_slot: &mut f64,
        var_mks_cit_slot: &mut f64,
        var_mks_cit_rv_slot: &mut f64,
        var_mks_dly3_slot: &mut f64,
        var_mks_dly3_rv_slot: &mut f64,
        var_mks_dlyov_slot: &mut f64,
        var_mks_dlyov_dn0_slot: &mut f64,
        var_mks_dlyov_dn10_slot: &mut f64,
        var_mks_dlyov_dn11_slot: &mut f64,
        var_mks_dlyov_dn14_slot: &mut f64,
        var_mks_dlyov_dn2_slot: &mut f64,
        var_mks_dlyov_dn4_slot: &mut f64,
        var_mks_dlyov_dn5_slot: &mut f64,
        var_mks_dlyov_dn6_slot: &mut f64,
        var_mks_dlyov_dn7_slot: &mut f64,
        var_mks_dlyov_dn8_slot: &mut f64,
        var_mks_dlyov_dn9_slot: &mut f64,
        var_mks_dlyov_rv_slot: &mut f64,
        var_mks_gleak4_slot: &mut f64,
        var_mks_gleak4_rv_slot: &mut f64,
        var_mks_gleak5_slot: &mut f64,
        var_mks_gleak5_rv_slot: &mut f64,
        var_mks_gleak7_slot: &mut f64,
        var_mks_gleak7_rv_slot: &mut f64,
        var_mks_glksd3_slot: &mut f64,
        var_mks_glksd3_rv_slot: &mut f64,
        var_mks_ll_slot: &mut f64,
        var_mks_ll_rv_slot: &mut f64,
        var_mks_nsubsub_slot: &mut f64,
        var_mks_nsubsub_rv_slot: &mut f64,
        var_mks_ovslp_slot: &mut f64,
        var_mks_ovslp_rv_slot: &mut f64,
        var_mks_rdrmue_slot: &mut f64,
        var_mks_rdrmue_rv_slot: &mut f64,
        var_mks_rdrmues_slot: &mut f64,
        var_mks_rdrmues_rv_slot: &mut f64,
        var_mks_rdrvmax_slot: &mut f64,
        var_mks_rdrvmax_rv_slot: &mut f64,
        var_mks_rdrvmaxs_slot: &mut f64,
        var_mks_rdrvmaxs_rv_slot: &mut f64,
        var_mks_rdtemp1_slot: &mut f64,
        var_mks_rdtemp1_rv_slot: &mut f64,
        var_mks_rdtemp2_slot: &mut f64,
        var_mks_rdtemp2_rv_slot: &mut f64,
        var_mks_rdvdtemp1_slot: &mut f64,
        var_mks_rdvdtemp1_rv_slot: &mut f64,
        var_mks_rdvdtemp2_slot: &mut f64,
        var_mks_rdvdtemp2_rv_slot: &mut f64,
        var_mks_slg_slot: &mut f64,
        var_mks_slg_rv_slot: &mut f64,
        var_mks_slgl_slot: &mut f64,
        var_mks_slgl_rv_slot: &mut f64,
        var_mks_sub1l_slot: &mut f64,
        var_mks_sub1l_rv_slot: &mut f64,
        var_mks_sub2l_slot: &mut f64,
        var_mks_sub2l_rv_slot: &mut f64,
        var_mks_subld2_slot: &mut f64,
        var_mks_subld2_rv_slot: &mut f64,
        var_mks_svbsl_slot: &mut f64,
        var_mks_svbsl_rv_slot: &mut f64,
        var_mks_svgsl_slot: &mut f64,
        var_mks_svgsl_rv_slot: &mut f64,
        var_mks_svgsw_slot: &mut f64,
        var_mks_svgsw_rv_slot: &mut f64,
        var_mks_wl_slot: &mut f64,
        var_mks_wl_rv_slot: &mut f64,
        var_uc_bgtmp1_slot: &mut f64,
        var_uc_bgtmp1_rv_slot: &mut f64,
        var_uc_bgtmp2_slot: &mut f64,
        var_uc_bgtmp2_rv_slot: &mut f64,
        var_uc_eg0_slot: &mut f64,
        var_uc_eg0_rv_slot: &mut f64,
        var_uc_fn1_slot: &mut f64,
        var_uc_fn1_rv_slot: &mut f64,
        var_uc_fn2_slot: &mut f64,
        var_uc_fn2_rv_slot: &mut f64,
        var_uc_muecb0_slot: &mut f64,
        var_uc_muecb0_rv_slot: &mut f64,
        var_uc_muecb1_slot: &mut f64,
        var_uc_muecb1_rv_slot: &mut f64,
        var_uc_mueph1_slot: &mut f64,
        var_uc_mueph1_rv_slot: &mut f64,
        var_uc_muesr1_slot: &mut f64,
        var_uc_muesr1_rv_slot: &mut f64,
        var_uc_muetmp_slot: &mut f64,
        var_uc_muetmp_rv_slot: &mut f64,
        var_uc_ndep_slot: &mut f64,
        var_uc_ndep_rv_slot: &mut f64,
        var_uc_ndepm_slot: &mut f64,
        var_uc_ndepm_dn0_slot: &mut f64,
        var_uc_ndepm_dn10_slot: &mut f64,
        var_uc_ndepm_dn11_slot: &mut f64,
        var_uc_ndepm_dn14_slot: &mut f64,
        var_uc_ndepm_dn2_slot: &mut f64,
        var_uc_ndepm_dn4_slot: &mut f64,
        var_uc_ndepm_dn5_slot: &mut f64,
        var_uc_ndepm_dn6_slot: &mut f64,
        var_uc_ndepm_dn7_slot: &mut f64,
        var_uc_ndepm_dn8_slot: &mut f64,
        var_uc_ndepm_dn9_slot: &mut f64,
        var_uc_ndepm_rv_slot: &mut f64,
        var_uc_ninv_slot: &mut f64,
        var_uc_ninv_rv_slot: &mut f64,
        var_uc_njunc_slot: &mut f64,
        var_uc_njunc_rv_slot: &mut f64,
        var_uc_nover_slot: &mut f64,
        var_uc_nover_rv_slot: &mut f64,
        var_uc_novers_slot: &mut f64,
        var_uc_novers_rv_slot: &mut f64,
        var_uc_nsubc_slot: &mut f64,
        var_uc_nsubc_rv_slot: &mut f64,
        var_uc_nsubp_slot: &mut f64,
        var_uc_nsubp_rv_slot: &mut f64,
        var_uc_pgd1_slot: &mut f64,
        var_uc_pgd1_rv_slot: &mut f64,
        var_uc_sc1_slot: &mut f64,
        var_uc_sc1_rv_slot: &mut f64,
        var_uc_sc2_slot: &mut f64,
        var_uc_sc2_rv_slot: &mut f64,
        var_uc_sc3_slot: &mut f64,
        var_uc_sc3_rv_slot: &mut f64,
        var_uc_scp1_slot: &mut f64,
        var_uc_scp1_rv_slot: &mut f64,
        var_uc_scp2_slot: &mut f64,
        var_uc_scp2_rv_slot: &mut f64,
        var_uc_scp3_slot: &mut f64,
        var_uc_scp3_rv_slot: &mut f64,
        var_uc_sub1_slot: &mut f64,
        var_uc_sub1_rv_slot: &mut f64,
        var_uc_sub1snp_slot: &mut f64,
        var_uc_sub1snp_rv_slot: &mut f64,
        var_uc_sub2_slot: &mut f64,
        var_uc_sub2_rv_slot: &mut f64,
        var_uc_sub2snp_slot: &mut f64,
        var_uc_sub2snp_rv_slot: &mut f64,
        var_uc_svbs_slot: &mut f64,
        var_uc_svbs_rv_slot: &mut f64,
        var_uc_svds_slot: &mut f64,
        var_uc_svds_rv_slot: &mut f64,
        var_uc_svdssnp_slot: &mut f64,
        var_uc_svdssnp_rv_slot: &mut f64,
        var_uc_svgs_slot: &mut f64,
        var_uc_svgs_rv_slot: &mut f64,
        var_uc_vfbc_slot: &mut f64,
        var_uc_vfbc_rv_slot: &mut f64,
        var_uc_vfbover_slot: &mut f64,
        var_uc_vfbover_rv_slot: &mut f64,
        var_uc_vmax_slot: &mut f64,
        var_uc_vmax_rv_slot: &mut f64,
        var_uc_vtmp_slot: &mut f64,
        var_uc_vtmp_rv_slot: &mut f64,
        var_uc_wl2_slot: &mut f64,
        var_uc_wl2_rv_slot: &mut f64,
        var_uc_wvth0_slot: &mut f64,
        var_uc_wvth0_rv_slot: &mut f64,
        var_wbin_slot: &mut f64,
        var_wbin_rv_slot: &mut f64,
        var_wg_slot: &mut f64,
        var_wg_rv_slot: &mut f64,
        var_wgate_slot: &mut f64,
        var_wgate_rv_slot: &mut f64,
    ) {
        let mut var_ktnom: f64 = *var_ktnom_slot;
        let mut var_ktnom_rv: f64 = *var_ktnom_rv_slot;
        let mut var_lbin: f64 = *var_lbin_slot;
        let mut var_lbin_rv: f64 = *var_lbin_rv_slot;
        let mut var_lg: f64 = *var_lg_slot;
        let mut var_lg_rv: f64 = *var_lg_rv_slot;
        let mut var_lgate: f64 = *var_lgate_slot;
        let mut var_lgate_rv: f64 = *var_lgate_rv_slot;
        let mut var_lwbin: f64 = *var_lwbin_slot;
        let mut var_lwbin_rv: f64 = *var_lwbin_rv_slot;
        let mut var_mks_cit: f64 = *var_mks_cit_slot;
        let mut var_mks_cit_rv: f64 = *var_mks_cit_rv_slot;
        let mut var_mks_dly3: f64 = *var_mks_dly3_slot;
        let mut var_mks_dly3_rv: f64 = *var_mks_dly3_rv_slot;
        let mut var_mks_dlyov: f64 = *var_mks_dlyov_slot;
        let mut var_mks_dlyov_dn0: f64 = *var_mks_dlyov_dn0_slot;
        let mut var_mks_dlyov_dn10: f64 = *var_mks_dlyov_dn10_slot;
        let mut var_mks_dlyov_dn11: f64 = *var_mks_dlyov_dn11_slot;
        let mut var_mks_dlyov_dn14: f64 = *var_mks_dlyov_dn14_slot;
        let mut var_mks_dlyov_dn2: f64 = *var_mks_dlyov_dn2_slot;
        let mut var_mks_dlyov_dn4: f64 = *var_mks_dlyov_dn4_slot;
        let mut var_mks_dlyov_dn5: f64 = *var_mks_dlyov_dn5_slot;
        let mut var_mks_dlyov_dn6: f64 = *var_mks_dlyov_dn6_slot;
        let mut var_mks_dlyov_dn7: f64 = *var_mks_dlyov_dn7_slot;
        let mut var_mks_dlyov_dn8: f64 = *var_mks_dlyov_dn8_slot;
        let mut var_mks_dlyov_dn9: f64 = *var_mks_dlyov_dn9_slot;
        let mut var_mks_dlyov_rv: f64 = *var_mks_dlyov_rv_slot;
        let mut var_mks_gleak4: f64 = *var_mks_gleak4_slot;
        let mut var_mks_gleak4_rv: f64 = *var_mks_gleak4_rv_slot;
        let mut var_mks_gleak5: f64 = *var_mks_gleak5_slot;
        let mut var_mks_gleak5_rv: f64 = *var_mks_gleak5_rv_slot;
        let mut var_mks_gleak7: f64 = *var_mks_gleak7_slot;
        let mut var_mks_gleak7_rv: f64 = *var_mks_gleak7_rv_slot;
        let mut var_mks_glksd3: f64 = *var_mks_glksd3_slot;
        let mut var_mks_glksd3_rv: f64 = *var_mks_glksd3_rv_slot;
        let mut var_mks_ll: f64 = *var_mks_ll_slot;
        let mut var_mks_ll_rv: f64 = *var_mks_ll_rv_slot;
        let mut var_mks_nsubsub: f64 = *var_mks_nsubsub_slot;
        let mut var_mks_nsubsub_rv: f64 = *var_mks_nsubsub_rv_slot;
        let mut var_mks_ovslp: f64 = *var_mks_ovslp_slot;
        let mut var_mks_ovslp_rv: f64 = *var_mks_ovslp_rv_slot;
        let mut var_mks_rdrmue: f64 = *var_mks_rdrmue_slot;
        let mut var_mks_rdrmue_rv: f64 = *var_mks_rdrmue_rv_slot;
        let mut var_mks_rdrmues: f64 = *var_mks_rdrmues_slot;
        let mut var_mks_rdrmues_rv: f64 = *var_mks_rdrmues_rv_slot;
        let mut var_mks_rdrvmax: f64 = *var_mks_rdrvmax_slot;
        let mut var_mks_rdrvmax_rv: f64 = *var_mks_rdrvmax_rv_slot;
        let mut var_mks_rdrvmaxs: f64 = *var_mks_rdrvmaxs_slot;
        let mut var_mks_rdrvmaxs_rv: f64 = *var_mks_rdrvmaxs_rv_slot;
        let mut var_mks_rdtemp1: f64 = *var_mks_rdtemp1_slot;
        let mut var_mks_rdtemp1_rv: f64 = *var_mks_rdtemp1_rv_slot;
        let mut var_mks_rdtemp2: f64 = *var_mks_rdtemp2_slot;
        let mut var_mks_rdtemp2_rv: f64 = *var_mks_rdtemp2_rv_slot;
        let mut var_mks_rdvdtemp1: f64 = *var_mks_rdvdtemp1_slot;
        let mut var_mks_rdvdtemp1_rv: f64 = *var_mks_rdvdtemp1_rv_slot;
        let mut var_mks_rdvdtemp2: f64 = *var_mks_rdvdtemp2_slot;
        let mut var_mks_rdvdtemp2_rv: f64 = *var_mks_rdvdtemp2_rv_slot;
        let mut var_mks_slg: f64 = *var_mks_slg_slot;
        let mut var_mks_slg_rv: f64 = *var_mks_slg_rv_slot;
        let mut var_mks_slgl: f64 = *var_mks_slgl_slot;
        let mut var_mks_slgl_rv: f64 = *var_mks_slgl_rv_slot;
        let mut var_mks_sub1l: f64 = *var_mks_sub1l_slot;
        let mut var_mks_sub1l_rv: f64 = *var_mks_sub1l_rv_slot;
        let mut var_mks_sub2l: f64 = *var_mks_sub2l_slot;
        let mut var_mks_sub2l_rv: f64 = *var_mks_sub2l_rv_slot;
        let mut var_mks_subld2: f64 = *var_mks_subld2_slot;
        let mut var_mks_subld2_rv: f64 = *var_mks_subld2_rv_slot;
        let mut var_mks_svbsl: f64 = *var_mks_svbsl_slot;
        let mut var_mks_svbsl_rv: f64 = *var_mks_svbsl_rv_slot;
        let mut var_mks_svgsl: f64 = *var_mks_svgsl_slot;
        let mut var_mks_svgsl_rv: f64 = *var_mks_svgsl_rv_slot;
        let mut var_mks_svgsw: f64 = *var_mks_svgsw_slot;
        let mut var_mks_svgsw_rv: f64 = *var_mks_svgsw_rv_slot;
        let mut var_mks_wl: f64 = *var_mks_wl_slot;
        let mut var_mks_wl_rv: f64 = *var_mks_wl_rv_slot;
        let mut var_uc_bgtmp1: f64 = *var_uc_bgtmp1_slot;
        let mut var_uc_bgtmp1_rv: f64 = *var_uc_bgtmp1_rv_slot;
        let mut var_uc_bgtmp2: f64 = *var_uc_bgtmp2_slot;
        let mut var_uc_bgtmp2_rv: f64 = *var_uc_bgtmp2_rv_slot;
        let mut var_uc_eg0: f64 = *var_uc_eg0_slot;
        let mut var_uc_eg0_rv: f64 = *var_uc_eg0_rv_slot;
        let mut var_uc_fn1: f64 = *var_uc_fn1_slot;
        let mut var_uc_fn1_rv: f64 = *var_uc_fn1_rv_slot;
        let mut var_uc_fn2: f64 = *var_uc_fn2_slot;
        let mut var_uc_fn2_rv: f64 = *var_uc_fn2_rv_slot;
        let mut var_uc_muecb0: f64 = *var_uc_muecb0_slot;
        let mut var_uc_muecb0_rv: f64 = *var_uc_muecb0_rv_slot;
        let mut var_uc_muecb1: f64 = *var_uc_muecb1_slot;
        let mut var_uc_muecb1_rv: f64 = *var_uc_muecb1_rv_slot;
        let mut var_uc_mueph1: f64 = *var_uc_mueph1_slot;
        let mut var_uc_mueph1_rv: f64 = *var_uc_mueph1_rv_slot;
        let mut var_uc_muesr1: f64 = *var_uc_muesr1_slot;
        let mut var_uc_muesr1_rv: f64 = *var_uc_muesr1_rv_slot;
        let mut var_uc_muetmp: f64 = *var_uc_muetmp_slot;
        let mut var_uc_muetmp_rv: f64 = *var_uc_muetmp_rv_slot;
        let mut var_uc_ndep: f64 = *var_uc_ndep_slot;
        let mut var_uc_ndep_rv: f64 = *var_uc_ndep_rv_slot;
        let mut var_uc_ndepm: f64 = *var_uc_ndepm_slot;
        let mut var_uc_ndepm_dn0: f64 = *var_uc_ndepm_dn0_slot;
        let mut var_uc_ndepm_dn10: f64 = *var_uc_ndepm_dn10_slot;
        let mut var_uc_ndepm_dn11: f64 = *var_uc_ndepm_dn11_slot;
        let mut var_uc_ndepm_dn14: f64 = *var_uc_ndepm_dn14_slot;
        let mut var_uc_ndepm_dn2: f64 = *var_uc_ndepm_dn2_slot;
        let mut var_uc_ndepm_dn4: f64 = *var_uc_ndepm_dn4_slot;
        let mut var_uc_ndepm_dn5: f64 = *var_uc_ndepm_dn5_slot;
        let mut var_uc_ndepm_dn6: f64 = *var_uc_ndepm_dn6_slot;
        let mut var_uc_ndepm_dn7: f64 = *var_uc_ndepm_dn7_slot;
        let mut var_uc_ndepm_dn8: f64 = *var_uc_ndepm_dn8_slot;
        let mut var_uc_ndepm_dn9: f64 = *var_uc_ndepm_dn9_slot;
        let mut var_uc_ndepm_rv: f64 = *var_uc_ndepm_rv_slot;
        let mut var_uc_ninv: f64 = *var_uc_ninv_slot;
        let mut var_uc_ninv_rv: f64 = *var_uc_ninv_rv_slot;
        let mut var_uc_njunc: f64 = *var_uc_njunc_slot;
        let mut var_uc_njunc_rv: f64 = *var_uc_njunc_rv_slot;
        let mut var_uc_nover: f64 = *var_uc_nover_slot;
        let mut var_uc_nover_rv: f64 = *var_uc_nover_rv_slot;
        let mut var_uc_novers: f64 = *var_uc_novers_slot;
        let mut var_uc_novers_rv: f64 = *var_uc_novers_rv_slot;
        let mut var_uc_nsubc: f64 = *var_uc_nsubc_slot;
        let mut var_uc_nsubc_rv: f64 = *var_uc_nsubc_rv_slot;
        let mut var_uc_nsubp: f64 = *var_uc_nsubp_slot;
        let mut var_uc_nsubp_rv: f64 = *var_uc_nsubp_rv_slot;
        let mut var_uc_pgd1: f64 = *var_uc_pgd1_slot;
        let mut var_uc_pgd1_rv: f64 = *var_uc_pgd1_rv_slot;
        let mut var_uc_sc1: f64 = *var_uc_sc1_slot;
        let mut var_uc_sc1_rv: f64 = *var_uc_sc1_rv_slot;
        let mut var_uc_sc2: f64 = *var_uc_sc2_slot;
        let mut var_uc_sc2_rv: f64 = *var_uc_sc2_rv_slot;
        let mut var_uc_sc3: f64 = *var_uc_sc3_slot;
        let mut var_uc_sc3_rv: f64 = *var_uc_sc3_rv_slot;
        let mut var_uc_scp1: f64 = *var_uc_scp1_slot;
        let mut var_uc_scp1_rv: f64 = *var_uc_scp1_rv_slot;
        let mut var_uc_scp2: f64 = *var_uc_scp2_slot;
        let mut var_uc_scp2_rv: f64 = *var_uc_scp2_rv_slot;
        let mut var_uc_scp3: f64 = *var_uc_scp3_slot;
        let mut var_uc_scp3_rv: f64 = *var_uc_scp3_rv_slot;
        let mut var_uc_sub1: f64 = *var_uc_sub1_slot;
        let mut var_uc_sub1_rv: f64 = *var_uc_sub1_rv_slot;
        let mut var_uc_sub1snp: f64 = *var_uc_sub1snp_slot;
        let mut var_uc_sub1snp_rv: f64 = *var_uc_sub1snp_rv_slot;
        let mut var_uc_sub2: f64 = *var_uc_sub2_slot;
        let mut var_uc_sub2_rv: f64 = *var_uc_sub2_rv_slot;
        let mut var_uc_sub2snp: f64 = *var_uc_sub2snp_slot;
        let mut var_uc_sub2snp_rv: f64 = *var_uc_sub2snp_rv_slot;
        let mut var_uc_svbs: f64 = *var_uc_svbs_slot;
        let mut var_uc_svbs_rv: f64 = *var_uc_svbs_rv_slot;
        let mut var_uc_svds: f64 = *var_uc_svds_slot;
        let mut var_uc_svds_rv: f64 = *var_uc_svds_rv_slot;
        let mut var_uc_svdssnp: f64 = *var_uc_svdssnp_slot;
        let mut var_uc_svdssnp_rv: f64 = *var_uc_svdssnp_rv_slot;
        let mut var_uc_svgs: f64 = *var_uc_svgs_slot;
        let mut var_uc_svgs_rv: f64 = *var_uc_svgs_rv_slot;
        let mut var_uc_vfbc: f64 = *var_uc_vfbc_slot;
        let mut var_uc_vfbc_rv: f64 = *var_uc_vfbc_rv_slot;
        let mut var_uc_vfbover: f64 = *var_uc_vfbover_slot;
        let mut var_uc_vfbover_rv: f64 = *var_uc_vfbover_rv_slot;
        let mut var_uc_vmax: f64 = *var_uc_vmax_slot;
        let mut var_uc_vmax_rv: f64 = *var_uc_vmax_rv_slot;
        let mut var_uc_vtmp: f64 = *var_uc_vtmp_slot;
        let mut var_uc_vtmp_rv: f64 = *var_uc_vtmp_rv_slot;
        let mut var_uc_wl2: f64 = *var_uc_wl2_slot;
        let mut var_uc_wl2_rv: f64 = *var_uc_wl2_rv_slot;
        let mut var_uc_wvth0: f64 = *var_uc_wvth0_slot;
        let mut var_uc_wvth0_rv: f64 = *var_uc_wvth0_rv_slot;
        let mut var_wbin: f64 = *var_wbin_slot;
        let mut var_wbin_rv: f64 = *var_wbin_rv_slot;
        let mut var_wg: f64 = *var_wg_slot;
        let mut var_wg_rv: f64 = *var_wg_rv_slot;
        let mut var_wgate: f64 = *var_wgate_slot;
        let mut var_wgate_rv: f64 = *var_wgate_rv_slot;

        let assign7580_e3132: f64 = (100.0_f64).powf(p.p122);
        let assign7580_e3133: f64 = (p.p120 / assign7580_e3132);
        var_mks_ll = assign7580_e3133;
        var_mks_ll_rv = 0.0;

        let assign7590_e3137: f64 = (100.0_f64).powf(p.p129);
        let assign7590_e3138: f64 = (p.p123 / assign7590_e3137);
        var_mks_wl = assign7590_e3138;
        var_mks_wl_rv = 0.0;

        let assign7600_e3142: f64 = (100.0_f64).powf(p.p199);
        let assign7600_e3143: f64 = (p.p198 / assign7600_e3142);
        var_mks_svgsl = assign7600_e3143;
        var_mks_svgsl_rv = 0.0;

        let assign7610_e3147: f64 = (100.0_f64).powf(p.p201);
        let assign7610_e3148: f64 = (p.p200 / assign7610_e3147);
        var_mks_svgsw = assign7610_e3148;
        var_mks_svgsw_rv = 0.0;

        let assign7620_e3152: f64 = (100.0_f64).powf(p.p184);
        let assign7620_e3153: f64 = (p.p183 / assign7620_e3152);
        var_mks_svbsl = assign7620_e3153;
        var_mks_svbsl_rv = 0.0;

        let assign7630_e3157: f64 = (100.0_f64).powf(p.p203);
        let assign7630_e3158: f64 = (p.p202 / assign7630_e3157);
        var_mks_slgl = assign7630_e3158;
        var_mks_slgl_rv = 0.0;

        let assign7640_e3162: f64 = (100.0_f64).powf(p.p191);
        let assign7640_e3163: f64 = (p.p190 / assign7640_e3162);
        var_mks_sub1l = assign7640_e3163;
        var_mks_sub1l_rv = 0.0;

        let assign7650_e3166: f64 = (p.p186 / 100.0);
        var_mks_slg = assign7650_e3166;
        var_mks_slg_rv = 0.0;

        let assign7660_e3169: f64 = (p.p192 / 100.0);
        var_mks_sub2l = assign7660_e3169;
        var_mks_sub2l_rv = 0.0;

        let assign7670_e3172: f64 = (p.p73 * 100.0);
        var_mks_subld2 = assign7670_e3172;
        var_mks_subld2_rv = 0.0;

        let assign7680_e3175: f64 = (p.p311 / 100.0);
        var_mks_rdtemp1 = assign7680_e3175;
        var_mks_rdtemp1_rv = 0.0;

        let assign7690_e3178: f64 = (p.p312 / 100.0);
        var_mks_rdtemp2 = assign7690_e3178;
        var_mks_rdtemp2_rv = 0.0;

        let assign7700_e3181: f64 = (p.p313 / 100.0);
        var_mks_rdvdtemp1 = assign7700_e3181;
        var_mks_rdvdtemp1_rv = 0.0;

        let assign7710_e3184: f64 = (p.p314 / 100.0);
        var_mks_rdvdtemp2 = assign7710_e3184;
        var_mks_rdvdtemp2_rv = 0.0;

        let assign7720_e3187: f64 = (p.p336 / 1e-6);
        var_mks_nsubsub = assign7720_e3187;
        var_mks_nsubsub_rv = 0.0;

        let assign7730_e3190: f64 = (p.p255 * 100.0);
        var_mks_glksd3 = assign7730_e3190;
        var_mks_glksd3_rv = 0.0;

        let assign7740_e3193: f64 = (p.p248 * 100.0);
        var_mks_gleak4 = assign7740_e3193;
        var_mks_gleak4_rv = 0.0;

        let assign7750_e3196: f64 = (p.p249 * 100.0);
        var_mks_gleak5 = assign7750_e3196;
        var_mks_gleak5_rv = 0.0;

        let assign7760_e3199: f64 = (p.p251 / 10000.0);
        var_mks_gleak7 = assign7760_e3199;
        var_mks_gleak7_rv = 0.0;

        let assign7770_e3202: f64 = (p.p266 * 10000.0);
        var_mks_cit = assign7770_e3202;
        var_mks_cit_rv = 0.0;

        let assign7780_e3205: f64 = (p.p275 / 100.0);
        var_mks_ovslp = assign7780_e3205;
        var_mks_ovslp_rv = 0.0;

        let assign7790_e3208: f64 = (p.p272 / 10000.0);
        var_mks_dly3 = assign7790_e3208;
        var_mks_dly3_rv = 0.0;

        let assign7800_e3211: f64 = (p.p273 / 10000.0);
        var_mks_dlyov = assign7800_e3211;
        var_mks_dlyov_dn0 = 0.0;
        var_mks_dlyov_dn2 = 0.0;
        var_mks_dlyov_dn4 = 0.0;
        var_mks_dlyov_dn5 = 0.0;
        var_mks_dlyov_dn6 = 0.0;
        var_mks_dlyov_dn7 = 0.0;
        var_mks_dlyov_dn8 = 0.0;
        var_mks_dlyov_dn9 = 0.0;
        var_mks_dlyov_dn10 = 0.0;
        var_mks_dlyov_dn11 = 0.0;
        var_mks_dlyov_dn14 = 0.0;
        var_mks_dlyov_rv = 0.0;

        let assign7820_e3217: f64 = (p.p409 / 10000.0);
        var_mks_rdrmue = assign7820_e3217;
        var_mks_rdrmue_rv = 0.0;

        let assign7830_e3220: f64 = (p.p412 / 100.0);
        var_mks_rdrvmax = assign7830_e3220;
        var_mks_rdrvmax_rv = 0.0;

        let assign7840_e3223: f64 = (p.p413 / 10000.0);
        var_mks_rdrmues = assign7840_e3223;
        var_mks_rdrmues_rv = 0.0;

        let assign7850_e3226: f64 = (p.p414 / 100.0);
        var_mks_rdrvmaxs = assign7850_e3226;
        var_mks_rdrvmaxs_rv = 0.0;

        let assign7860_e3229: f64 = (var_uc_ndepm / 1e-6);
        var_uc_ndepm = assign7860_e3229;
        var_uc_ndepm_dn0 = (var_uc_ndepm_dn0 / 1e-6);
        var_uc_ndepm_dn2 = (var_uc_ndepm_dn2 / 1e-6);
        var_uc_ndepm_dn4 = (var_uc_ndepm_dn4 / 1e-6);
        var_uc_ndepm_dn5 = (var_uc_ndepm_dn5 / 1e-6);
        var_uc_ndepm_dn6 = (var_uc_ndepm_dn6 / 1e-6);
        var_uc_ndepm_dn7 = (var_uc_ndepm_dn7 / 1e-6);
        var_uc_ndepm_dn8 = (var_uc_ndepm_dn8 / 1e-6);
        var_uc_ndepm_dn9 = (var_uc_ndepm_dn9 / 1e-6);
        var_uc_ndepm_dn10 = (var_uc_ndepm_dn10 / 1e-6);
        var_uc_ndepm_dn11 = (var_uc_ndepm_dn11 / 1e-6);
        var_uc_ndepm_dn14 = (var_uc_ndepm_dn14 / 1e-6);
        var_uc_ndepm_rv = 0.0;

        let assign7870_e3232: f64 = (p.p453 / 1e-6);
        var_uc_njunc = assign7870_e3232;
        var_uc_njunc_rv = 0.0;

        let assign7880_e3235: f64 = (p.p274 + 273.15);
        var_ktnom = assign7880_e3235;
        var_ktnom_rv = 0.0;

        let assign7930_e3258: f64 = (p.p0 + p.p116);
        var_lgate = assign7930_e3258;
        var_lgate_rv = 0.0;

        let assign7940_e3261: f64 = (p.p1 / p.p7);
        let assign7940_e3263: f64 = (assign7940_e3261 + p.p117);
        var_wgate = assign7940_e3263;
        var_wgate_rv = 0.0;

        let assign8090_e3363: f64 = (var_lgate * 1000000.0);
        var_lg = assign8090_e3363;
        var_lg_rv = 0.0;

        let assign8100_e3366: f64 = (var_wgate * 1000000.0);
        var_wg = assign8100_e3366;
        var_wg_rv = 0.0;

        let assign8110_e3369: f64 = (var_lg).powf(p.p553);
        var_lbin = assign8110_e3369;
        var_lbin_rv = 0.0;

        let assign8120_e3372: f64 = (var_wg).powf(p.p554);
        var_wbin = assign8120_e3372;
        var_wbin_rv = 0.0;

        let assign8130_e3375: f64 = (var_lbin * var_wbin);
        var_lwbin = assign8130_e3375;
        var_lwbin_rv = 0.0;

        let assign8140_e3379: f64 = (p.p555 / var_lbin);
        let assign8140_e3380: f64 = (p.p89 + assign8140_e3379);
        let assign8140_e3383: f64 = (p.p643 / var_wbin);
        let assign8140_e3384: f64 = (assign8140_e3380 + assign8140_e3383);
        let assign8140_e3387: f64 = (p.p731 / var_lwbin);
        let assign8140_e3388: f64 = (assign8140_e3384 + assign8140_e3387);
        var_uc_vmax = assign8140_e3388;
        var_uc_vmax_rv = 0.0;

        let assign8150_e3392: f64 = (p.p556 / var_lbin);
        let assign8150_e3393: f64 = (p.p92 + assign8150_e3392);
        let assign8150_e3396: f64 = (p.p644 / var_wbin);
        let assign8150_e3397: f64 = (assign8150_e3393 + assign8150_e3396);
        let assign8150_e3400: f64 = (p.p732 / var_lwbin);
        let assign8150_e3401: f64 = (assign8150_e3397 + assign8150_e3400);
        var_uc_bgtmp1 = assign8150_e3401;
        var_uc_bgtmp1_rv = 0.0;

        let assign8160_e3405: f64 = (p.p557 / var_lbin);
        let assign8160_e3406: f64 = (p.p93 + assign8160_e3405);
        let assign8160_e3409: f64 = (p.p645 / var_wbin);
        let assign8160_e3410: f64 = (assign8160_e3406 + assign8160_e3409);
        let assign8160_e3413: f64 = (p.p733 / var_lwbin);
        let assign8160_e3414: f64 = (assign8160_e3410 + assign8160_e3413);
        var_uc_bgtmp2 = assign8160_e3414;
        var_uc_bgtmp2_rv = 0.0;

        let assign8170_e3418: f64 = (p.p558 / var_lbin);
        let assign8170_e3419: f64 = (p.p94 + assign8170_e3418);
        let assign8170_e3422: f64 = (p.p646 / var_wbin);
        let assign8170_e3423: f64 = (assign8170_e3419 + assign8170_e3422);
        let assign8170_e3426: f64 = (p.p734 / var_lwbin);
        let assign8170_e3427: f64 = (assign8170_e3423 + assign8170_e3426);
        var_uc_eg0 = assign8170_e3427;
        var_uc_eg0_rv = 0.0;

        let assign8180_e3431: f64 = (p.p559 / var_lbin);
        let assign8180_e3432: f64 = (p.p110 + assign8180_e3431);
        let assign8180_e3435: f64 = (p.p647 / var_wbin);
        let assign8180_e3436: f64 = (assign8180_e3432 + assign8180_e3435);
        let assign8180_e3439: f64 = (p.p735 / var_lwbin);
        let assign8180_e3440: f64 = (assign8180_e3436 + assign8180_e3439);
        var_uc_vfbover = assign8180_e3440;
        var_uc_vfbover_rv = 0.0;

        let assign8190_e3444: f64 = (p.p560 / var_lbin);
        let assign8190_e3445: f64 = (p.p111 + assign8190_e3444);
        let assign8190_e3448: f64 = (p.p648 / var_wbin);
        let assign8190_e3449: f64 = (assign8190_e3445 + assign8190_e3448);
        let assign8190_e3452: f64 = (p.p736 / var_lwbin);
        let assign8190_e3453: f64 = (assign8190_e3449 + assign8190_e3452);
        var_uc_nover = assign8190_e3453;
        var_uc_nover_rv = 0.0;

        let assign8200_e3457: f64 = (p.p561 / var_lbin);
        let assign8200_e3458: f64 = (p.p112 + assign8200_e3457);
        let assign8200_e3461: f64 = (p.p649 / var_wbin);
        let assign8200_e3462: f64 = (assign8200_e3458 + assign8200_e3461);
        let assign8200_e3465: f64 = (p.p737 / var_lwbin);
        let assign8200_e3466: f64 = (assign8200_e3462 + assign8200_e3465);
        var_uc_novers = assign8200_e3466;
        var_uc_novers_rv = 0.0;

        let assign8210_e3470: f64 = (p.p562 / var_lbin);
        let assign8210_e3471: f64 = (p.p126 + assign8210_e3470);
        let assign8210_e3474: f64 = (p.p650 / var_wbin);
        let assign8210_e3475: f64 = (assign8210_e3471 + assign8210_e3474);
        let assign8210_e3478: f64 = (p.p738 / var_lwbin);
        let assign8210_e3479: f64 = (assign8210_e3475 + assign8210_e3478);
        var_uc_wl2 = assign8210_e3479;
        var_uc_wl2_rv = 0.0;

        let assign8220_e3483: f64 = (p.p563 / var_lbin);
        let assign8220_e3484: f64 = (p.p136 + assign8220_e3483);
        let assign8220_e3487: f64 = (p.p651 / var_wbin);
        let assign8220_e3488: f64 = (assign8220_e3484 + assign8220_e3487);
        let assign8220_e3491: f64 = (p.p739 / var_lwbin);
        let assign8220_e3492: f64 = (assign8220_e3488 + assign8220_e3491);
        var_uc_vfbc = assign8220_e3492;
        var_uc_vfbc_rv = 0.0;

        let assign8230_e3496: f64 = (p.p564 / var_lbin);
        let assign8230_e3497: f64 = (p.p138 + assign8230_e3496);
        let assign8230_e3500: f64 = (p.p652 / var_wbin);
        let assign8230_e3501: f64 = (assign8230_e3497 + assign8230_e3500);
        let assign8230_e3504: f64 = (p.p740 / var_lwbin);
        let assign8230_e3505: f64 = (assign8230_e3501 + assign8230_e3504);
        var_uc_nsubc = assign8230_e3505;
        var_uc_nsubc_rv = 0.0;

        let assign8240_e3509: f64 = (p.p565 / var_lbin);
        let assign8240_e3510: f64 = (p.p141 + assign8240_e3509);
        let assign8240_e3513: f64 = (p.p653 / var_wbin);
        let assign8240_e3514: f64 = (assign8240_e3510 + assign8240_e3513);
        let assign8240_e3517: f64 = (p.p741 / var_lwbin);
        let assign8240_e3518: f64 = (assign8240_e3514 + assign8240_e3517);
        var_uc_nsubp = assign8240_e3518;
        var_uc_nsubp_rv = 0.0;

        let assign8250_e3522: f64 = (p.p566 / var_lbin);
        let assign8250_e3523: f64 = (p.p144 + assign8250_e3522);
        let assign8250_e3526: f64 = (p.p654 / var_wbin);
        let assign8250_e3527: f64 = (assign8250_e3523 + assign8250_e3526);
        let assign8250_e3530: f64 = (p.p742 / var_lwbin);
        let assign8250_e3531: f64 = (assign8250_e3527 + assign8250_e3530);
        var_uc_scp1 = assign8250_e3531;
        var_uc_scp1_rv = 0.0;

        let assign8260_e3535: f64 = (p.p567 / var_lbin);
        let assign8260_e3536: f64 = (p.p145 + assign8260_e3535);
        let assign8260_e3539: f64 = (p.p655 / var_wbin);
        let assign8260_e3540: f64 = (assign8260_e3536 + assign8260_e3539);
        let assign8260_e3543: f64 = (p.p743 / var_lwbin);
        let assign8260_e3544: f64 = (assign8260_e3540 + assign8260_e3543);
        var_uc_scp2 = assign8260_e3544;
        var_uc_scp2_rv = 0.0;

        let assign8270_e3548: f64 = (p.p568 / var_lbin);
        let assign8270_e3549: f64 = (p.p146 + assign8270_e3548);
        let assign8270_e3552: f64 = (p.p656 / var_wbin);
        let assign8270_e3553: f64 = (assign8270_e3549 + assign8270_e3552);
        let assign8270_e3556: f64 = (p.p744 / var_lwbin);
        let assign8270_e3557: f64 = (assign8270_e3553 + assign8270_e3556);
        var_uc_scp3 = assign8270_e3557;
        var_uc_scp3_rv = 0.0;

        let assign8280_e3561: f64 = (p.p569 / var_lbin);
        let assign8280_e3562: f64 = (p.p147 + assign8280_e3561);
        let assign8280_e3565: f64 = (p.p657 / var_wbin);
        let assign8280_e3566: f64 = (assign8280_e3562 + assign8280_e3565);
        let assign8280_e3569: f64 = (p.p745 / var_lwbin);
        let assign8280_e3570: f64 = (assign8280_e3566 + assign8280_e3569);
        var_uc_sc1 = assign8280_e3570;
        var_uc_sc1_rv = 0.0;

        let assign8290_e3574: f64 = (p.p570 / var_lbin);
        let assign8290_e3575: f64 = (p.p148 + assign8290_e3574);
        let assign8290_e3578: f64 = (p.p658 / var_wbin);
        let assign8290_e3579: f64 = (assign8290_e3575 + assign8290_e3578);
        let assign8290_e3582: f64 = (p.p746 / var_lwbin);
        let assign8290_e3583: f64 = (assign8290_e3579 + assign8290_e3582);
        var_uc_sc2 = assign8290_e3583;
        var_uc_sc2_rv = 0.0;

        let assign8300_e3587: f64 = (p.p571 / var_lbin);
        let assign8300_e3588: f64 = (p.p149 + assign8300_e3587);
        let assign8300_e3591: f64 = (p.p659 / var_wbin);
        let assign8300_e3592: f64 = (assign8300_e3588 + assign8300_e3591);
        let assign8300_e3595: f64 = (p.p747 / var_lwbin);
        let assign8300_e3596: f64 = (assign8300_e3592 + assign8300_e3595);
        var_uc_sc3 = assign8300_e3596;
        var_uc_sc3_rv = 0.0;

        let assign8310_e3600: f64 = (p.p572 / var_lbin);
        let assign8310_e3601: f64 = (p.p151 + assign8310_e3600);
        let assign8310_e3604: f64 = (p.p660 / var_wbin);
        let assign8310_e3605: f64 = (assign8310_e3601 + assign8310_e3604);
        let assign8310_e3608: f64 = (p.p748 / var_lwbin);
        let assign8310_e3609: f64 = (assign8310_e3605 + assign8310_e3608);
        var_uc_pgd1 = assign8310_e3609;
        var_uc_pgd1_rv = 0.0;

        let assign8320_e3613: f64 = (p.p573 / var_lbin);
        let assign8320_e3614: f64 = (p.p154 + assign8320_e3613);
        let assign8320_e3617: f64 = (p.p661 / var_wbin);
        let assign8320_e3618: f64 = (assign8320_e3614 + assign8320_e3617);
        let assign8320_e3621: f64 = (p.p749 / var_lwbin);
        let assign8320_e3622: f64 = (assign8320_e3618 + assign8320_e3621);
        var_uc_ndep = assign8320_e3622;
        var_uc_ndep_rv = 0.0;

        let assign8330_e3626: f64 = (p.p574 / var_lbin);
        let assign8330_e3627: f64 = (p.p157 + assign8330_e3626);
        let assign8330_e3630: f64 = (p.p662 / var_wbin);
        let assign8330_e3631: f64 = (assign8330_e3627 + assign8330_e3630);
        let assign8330_e3634: f64 = (p.p750 / var_lwbin);
        let assign8330_e3635: f64 = (assign8330_e3631 + assign8330_e3634);
        var_uc_ninv = assign8330_e3635;
        var_uc_ninv_rv = 0.0;

        let assign8340_e3639: f64 = (p.p575 / var_lbin);
        let assign8340_e3640: f64 = (p.p158 + assign8340_e3639);
        let assign8340_e3643: f64 = (p.p663 / var_wbin);
        let assign8340_e3644: f64 = (assign8340_e3640 + assign8340_e3643);
        let assign8340_e3647: f64 = (p.p751 / var_lwbin);
        let assign8340_e3648: f64 = (assign8340_e3644 + assign8340_e3647);
        var_uc_muecb0 = assign8340_e3648;
        var_uc_muecb0_rv = 0.0;

        let assign8350_e3652: f64 = (p.p576 / var_lbin);
        let assign8350_e3653: f64 = (p.p159 + assign8350_e3652);
        let assign8350_e3656: f64 = (p.p664 / var_wbin);
        let assign8350_e3657: f64 = (assign8350_e3653 + assign8350_e3656);
        let assign8350_e3660: f64 = (p.p752 / var_lwbin);
        let assign8350_e3661: f64 = (assign8350_e3657 + assign8350_e3660);
        var_uc_muecb1 = assign8350_e3661;
        var_uc_muecb1_rv = 0.0;

        let assign8360_e3665: f64 = (p.p577 / var_lbin);
        let assign8360_e3666: f64 = (p.p161 + assign8360_e3665);
        let assign8360_e3669: f64 = (p.p665 / var_wbin);
        let assign8360_e3670: f64 = (assign8360_e3666 + assign8360_e3669);
        let assign8360_e3673: f64 = (p.p753 / var_lwbin);
        let assign8360_e3674: f64 = (assign8360_e3670 + assign8360_e3673);
        var_uc_mueph1 = assign8360_e3674;
        var_uc_mueph1_rv = 0.0;

        let assign8370_e3678: f64 = (p.p578 / var_lbin);
        let assign8370_e3679: f64 = (p.p169 + assign8370_e3678);
        let assign8370_e3682: f64 = (p.p666 / var_wbin);
        let assign8370_e3683: f64 = (assign8370_e3679 + assign8370_e3682);
        let assign8370_e3686: f64 = (p.p754 / var_lwbin);
        let assign8370_e3687: f64 = (assign8370_e3683 + assign8370_e3686);
        var_uc_vtmp = assign8370_e3687;
        var_uc_vtmp_rv = 0.0;

        let assign8380_e3691: f64 = (p.p579 / var_lbin);
        let assign8380_e3692: f64 = (p.p170 + assign8380_e3691);
        let assign8380_e3695: f64 = (p.p667 / var_wbin);
        let assign8380_e3696: f64 = (assign8380_e3692 + assign8380_e3695);
        let assign8380_e3699: f64 = (p.p755 / var_lwbin);
        let assign8380_e3700: f64 = (assign8380_e3696 + assign8380_e3699);
        var_uc_wvth0 = assign8380_e3700;
        var_uc_wvth0_rv = 0.0;

        let assign8390_e3704: f64 = (p.p580 / var_lbin);
        let assign8390_e3705: f64 = (p.p172 + assign8390_e3704);
        let assign8390_e3708: f64 = (p.p668 / var_wbin);
        let assign8390_e3709: f64 = (assign8390_e3705 + assign8390_e3708);
        let assign8390_e3712: f64 = (p.p756 / var_lwbin);
        let assign8390_e3713: f64 = (assign8390_e3709 + assign8390_e3712);
        var_uc_muesr1 = assign8390_e3713;
        var_uc_muesr1_rv = 0.0;

        let assign8400_e3717: f64 = (p.p581 / var_lbin);
        let assign8400_e3718: f64 = (p.p177 + assign8400_e3717);
        let assign8400_e3721: f64 = (p.p669 / var_wbin);
        let assign8400_e3722: f64 = (assign8400_e3718 + assign8400_e3721);
        let assign8400_e3725: f64 = (p.p757 / var_lwbin);
        let assign8400_e3726: f64 = (assign8400_e3722 + assign8400_e3725);
        var_uc_muetmp = assign8400_e3726;
        var_uc_muetmp_rv = 0.0;

        let assign8410_e3730: f64 = (p.p582 / var_lbin);
        let assign8410_e3731: f64 = (p.p179 + assign8410_e3730);
        let assign8410_e3734: f64 = (p.p670 / var_wbin);
        let assign8410_e3735: f64 = (assign8410_e3731 + assign8410_e3734);
        let assign8410_e3738: f64 = (p.p758 / var_lwbin);
        let assign8410_e3739: f64 = (assign8410_e3735 + assign8410_e3738);
        var_uc_sub1 = assign8410_e3739;
        var_uc_sub1_rv = 0.0;

        let assign8420_e3743: f64 = (p.p583 / var_lbin);
        let assign8420_e3744: f64 = (p.p180 + assign8420_e3743);
        let assign8420_e3747: f64 = (p.p671 / var_wbin);
        let assign8420_e3748: f64 = (assign8420_e3744 + assign8420_e3747);
        let assign8420_e3751: f64 = (p.p759 / var_lwbin);
        let assign8420_e3752: f64 = (assign8420_e3748 + assign8420_e3751);
        var_uc_sub2 = assign8420_e3752;
        var_uc_sub2_rv = 0.0;

        let assign8430_e3756: f64 = (p.p584 / var_lbin);
        let assign8430_e3757: f64 = (p.p185 + assign8430_e3756);
        let assign8430_e3760: f64 = (p.p672 / var_wbin);
        let assign8430_e3761: f64 = (assign8430_e3757 + assign8430_e3760);
        let assign8430_e3764: f64 = (p.p760 / var_lwbin);
        let assign8430_e3765: f64 = (assign8430_e3761 + assign8430_e3764);
        var_uc_svds = assign8430_e3765;
        var_uc_svds_rv = 0.0;

        let assign8440_e3769: f64 = (p.p585 / var_lbin);
        let assign8440_e3770: f64 = (p.p182 + assign8440_e3769);
        let assign8440_e3773: f64 = (p.p673 / var_wbin);
        let assign8440_e3774: f64 = (assign8440_e3770 + assign8440_e3773);
        let assign8440_e3777: f64 = (p.p761 / var_lwbin);
        let assign8440_e3778: f64 = (assign8440_e3774 + assign8440_e3777);
        var_uc_svbs = assign8440_e3778;
        var_uc_svbs_rv = 0.0;

        let assign8450_e3782: f64 = (p.p586 / var_lbin);
        let assign8450_e3783: f64 = (p.p181 + assign8450_e3782);
        let assign8450_e3786: f64 = (p.p674 / var_wbin);
        let assign8450_e3787: f64 = (assign8450_e3783 + assign8450_e3786);
        let assign8450_e3790: f64 = (p.p762 / var_lwbin);
        let assign8450_e3791: f64 = (assign8450_e3787 + assign8450_e3790);
        var_uc_svgs = assign8450_e3791;
        var_uc_svgs_rv = 0.0;

        let assign8460_e3795: f64 = (p.p587 / var_lbin);
        let assign8460_e3796: f64 = (p.p187 + assign8460_e3795);
        let assign8460_e3799: f64 = (p.p675 / var_wbin);
        let assign8460_e3800: f64 = (assign8460_e3796 + assign8460_e3799);
        let assign8460_e3803: f64 = (p.p763 / var_lwbin);
        let assign8460_e3804: f64 = (assign8460_e3800 + assign8460_e3803);
        var_uc_sub1snp = assign8460_e3804;
        var_uc_sub1snp_rv = 0.0;

        let assign8470_e3808: f64 = (p.p588 / var_lbin);
        let assign8470_e3809: f64 = (p.p188 + assign8470_e3808);
        let assign8470_e3812: f64 = (p.p676 / var_wbin);
        let assign8470_e3813: f64 = (assign8470_e3809 + assign8470_e3812);
        let assign8470_e3816: f64 = (p.p764 / var_lwbin);
        let assign8470_e3817: f64 = (assign8470_e3813 + assign8470_e3816);
        var_uc_sub2snp = assign8470_e3817;
        var_uc_sub2snp_rv = 0.0;

        let assign8480_e3821: f64 = (p.p589 / var_lbin);
        let assign8480_e3822: f64 = (p.p189 + assign8480_e3821);
        let assign8480_e3825: f64 = (p.p677 / var_wbin);
        let assign8480_e3826: f64 = (assign8480_e3822 + assign8480_e3825);
        let assign8480_e3829: f64 = (p.p765 / var_lwbin);
        let assign8480_e3830: f64 = (assign8480_e3826 + assign8480_e3829);
        var_uc_svdssnp = assign8480_e3830;
        var_uc_svdssnp_rv = 0.0;

        let assign8490_e3834: f64 = (p.p590 / var_lbin);
        let assign8490_e3835: f64 = (p.p194 + assign8490_e3834);
        let assign8490_e3838: f64 = (p.p678 / var_wbin);
        let assign8490_e3839: f64 = (assign8490_e3835 + assign8490_e3838);
        let assign8490_e3842: f64 = (p.p766 / var_lwbin);
        let assign8490_e3843: f64 = (assign8490_e3839 + assign8490_e3842);
        var_uc_fn1 = assign8490_e3843;
        var_uc_fn1_rv = 0.0;

        let assign8500_e3847: f64 = (p.p591 / var_lbin);
        let assign8500_e3848: f64 = (p.p195 + assign8500_e3847);
        let assign8500_e3851: f64 = (p.p679 / var_wbin);
        let assign8500_e3852: f64 = (assign8500_e3848 + assign8500_e3851);
        let assign8500_e3855: f64 = (p.p767 / var_lwbin);
        let assign8500_e3856: f64 = (assign8500_e3852 + assign8500_e3855);
        var_uc_fn2 = assign8500_e3856;
        var_uc_fn2_rv = 0.0;

        *var_ktnom_slot = var_ktnom;
        *var_ktnom_rv_slot = var_ktnom_rv;
        *var_lbin_slot = var_lbin;
        *var_lbin_rv_slot = var_lbin_rv;
        *var_lg_slot = var_lg;
        *var_lg_rv_slot = var_lg_rv;
        *var_lgate_slot = var_lgate;
        *var_lgate_rv_slot = var_lgate_rv;
        *var_lwbin_slot = var_lwbin;
        *var_lwbin_rv_slot = var_lwbin_rv;
        *var_mks_cit_slot = var_mks_cit;
        *var_mks_cit_rv_slot = var_mks_cit_rv;
        *var_mks_dly3_slot = var_mks_dly3;
        *var_mks_dly3_rv_slot = var_mks_dly3_rv;
        *var_mks_dlyov_slot = var_mks_dlyov;
        *var_mks_dlyov_dn0_slot = var_mks_dlyov_dn0;
        *var_mks_dlyov_dn10_slot = var_mks_dlyov_dn10;
        *var_mks_dlyov_dn11_slot = var_mks_dlyov_dn11;
        *var_mks_dlyov_dn14_slot = var_mks_dlyov_dn14;
        *var_mks_dlyov_dn2_slot = var_mks_dlyov_dn2;
        *var_mks_dlyov_dn4_slot = var_mks_dlyov_dn4;
        *var_mks_dlyov_dn5_slot = var_mks_dlyov_dn5;
        *var_mks_dlyov_dn6_slot = var_mks_dlyov_dn6;
        *var_mks_dlyov_dn7_slot = var_mks_dlyov_dn7;
        *var_mks_dlyov_dn8_slot = var_mks_dlyov_dn8;
        *var_mks_dlyov_dn9_slot = var_mks_dlyov_dn9;
        *var_mks_dlyov_rv_slot = var_mks_dlyov_rv;
        *var_mks_gleak4_slot = var_mks_gleak4;
        *var_mks_gleak4_rv_slot = var_mks_gleak4_rv;
        *var_mks_gleak5_slot = var_mks_gleak5;
        *var_mks_gleak5_rv_slot = var_mks_gleak5_rv;
        *var_mks_gleak7_slot = var_mks_gleak7;
        *var_mks_gleak7_rv_slot = var_mks_gleak7_rv;
        *var_mks_glksd3_slot = var_mks_glksd3;
        *var_mks_glksd3_rv_slot = var_mks_glksd3_rv;
        *var_mks_ll_slot = var_mks_ll;
        *var_mks_ll_rv_slot = var_mks_ll_rv;
        *var_mks_nsubsub_slot = var_mks_nsubsub;
        *var_mks_nsubsub_rv_slot = var_mks_nsubsub_rv;
        *var_mks_ovslp_slot = var_mks_ovslp;
        *var_mks_ovslp_rv_slot = var_mks_ovslp_rv;
        *var_mks_rdrmue_slot = var_mks_rdrmue;
        *var_mks_rdrmue_rv_slot = var_mks_rdrmue_rv;
        *var_mks_rdrmues_slot = var_mks_rdrmues;
        *var_mks_rdrmues_rv_slot = var_mks_rdrmues_rv;
        *var_mks_rdrvmax_slot = var_mks_rdrvmax;
        *var_mks_rdrvmax_rv_slot = var_mks_rdrvmax_rv;
        *var_mks_rdrvmaxs_slot = var_mks_rdrvmaxs;
        *var_mks_rdrvmaxs_rv_slot = var_mks_rdrvmaxs_rv;
        *var_mks_rdtemp1_slot = var_mks_rdtemp1;
        *var_mks_rdtemp1_rv_slot = var_mks_rdtemp1_rv;
        *var_mks_rdtemp2_slot = var_mks_rdtemp2;
        *var_mks_rdtemp2_rv_slot = var_mks_rdtemp2_rv;
        *var_mks_rdvdtemp1_slot = var_mks_rdvdtemp1;
        *var_mks_rdvdtemp1_rv_slot = var_mks_rdvdtemp1_rv;
        *var_mks_rdvdtemp2_slot = var_mks_rdvdtemp2;
        *var_mks_rdvdtemp2_rv_slot = var_mks_rdvdtemp2_rv;
        *var_mks_slg_slot = var_mks_slg;
        *var_mks_slg_rv_slot = var_mks_slg_rv;
        *var_mks_slgl_slot = var_mks_slgl;
        *var_mks_slgl_rv_slot = var_mks_slgl_rv;
        *var_mks_sub1l_slot = var_mks_sub1l;
        *var_mks_sub1l_rv_slot = var_mks_sub1l_rv;
        *var_mks_sub2l_slot = var_mks_sub2l;
        *var_mks_sub2l_rv_slot = var_mks_sub2l_rv;
        *var_mks_subld2_slot = var_mks_subld2;
        *var_mks_subld2_rv_slot = var_mks_subld2_rv;
        *var_mks_svbsl_slot = var_mks_svbsl;
        *var_mks_svbsl_rv_slot = var_mks_svbsl_rv;
        *var_mks_svgsl_slot = var_mks_svgsl;
        *var_mks_svgsl_rv_slot = var_mks_svgsl_rv;
        *var_mks_svgsw_slot = var_mks_svgsw;
        *var_mks_svgsw_rv_slot = var_mks_svgsw_rv;
        *var_mks_wl_slot = var_mks_wl;
        *var_mks_wl_rv_slot = var_mks_wl_rv;
        *var_uc_bgtmp1_slot = var_uc_bgtmp1;
        *var_uc_bgtmp1_rv_slot = var_uc_bgtmp1_rv;
        *var_uc_bgtmp2_slot = var_uc_bgtmp2;
        *var_uc_bgtmp2_rv_slot = var_uc_bgtmp2_rv;
        *var_uc_eg0_slot = var_uc_eg0;
        *var_uc_eg0_rv_slot = var_uc_eg0_rv;
        *var_uc_fn1_slot = var_uc_fn1;
        *var_uc_fn1_rv_slot = var_uc_fn1_rv;
        *var_uc_fn2_slot = var_uc_fn2;
        *var_uc_fn2_rv_slot = var_uc_fn2_rv;
        *var_uc_muecb0_slot = var_uc_muecb0;
        *var_uc_muecb0_rv_slot = var_uc_muecb0_rv;
        *var_uc_muecb1_slot = var_uc_muecb1;
        *var_uc_muecb1_rv_slot = var_uc_muecb1_rv;
        *var_uc_mueph1_slot = var_uc_mueph1;
        *var_uc_mueph1_rv_slot = var_uc_mueph1_rv;
        *var_uc_muesr1_slot = var_uc_muesr1;
        *var_uc_muesr1_rv_slot = var_uc_muesr1_rv;
        *var_uc_muetmp_slot = var_uc_muetmp;
        *var_uc_muetmp_rv_slot = var_uc_muetmp_rv;
        *var_uc_ndep_slot = var_uc_ndep;
        *var_uc_ndep_rv_slot = var_uc_ndep_rv;
        *var_uc_ndepm_slot = var_uc_ndepm;
        *var_uc_ndepm_dn0_slot = var_uc_ndepm_dn0;
        *var_uc_ndepm_dn10_slot = var_uc_ndepm_dn10;
        *var_uc_ndepm_dn11_slot = var_uc_ndepm_dn11;
        *var_uc_ndepm_dn14_slot = var_uc_ndepm_dn14;
        *var_uc_ndepm_dn2_slot = var_uc_ndepm_dn2;
        *var_uc_ndepm_dn4_slot = var_uc_ndepm_dn4;
        *var_uc_ndepm_dn5_slot = var_uc_ndepm_dn5;
        *var_uc_ndepm_dn6_slot = var_uc_ndepm_dn6;
        *var_uc_ndepm_dn7_slot = var_uc_ndepm_dn7;
        *var_uc_ndepm_dn8_slot = var_uc_ndepm_dn8;
        *var_uc_ndepm_dn9_slot = var_uc_ndepm_dn9;
        *var_uc_ndepm_rv_slot = var_uc_ndepm_rv;
        *var_uc_ninv_slot = var_uc_ninv;
        *var_uc_ninv_rv_slot = var_uc_ninv_rv;
        *var_uc_njunc_slot = var_uc_njunc;
        *var_uc_njunc_rv_slot = var_uc_njunc_rv;
        *var_uc_nover_slot = var_uc_nover;
        *var_uc_nover_rv_slot = var_uc_nover_rv;
        *var_uc_novers_slot = var_uc_novers;
        *var_uc_novers_rv_slot = var_uc_novers_rv;
        *var_uc_nsubc_slot = var_uc_nsubc;
        *var_uc_nsubc_rv_slot = var_uc_nsubc_rv;
        *var_uc_nsubp_slot = var_uc_nsubp;
        *var_uc_nsubp_rv_slot = var_uc_nsubp_rv;
        *var_uc_pgd1_slot = var_uc_pgd1;
        *var_uc_pgd1_rv_slot = var_uc_pgd1_rv;
        *var_uc_sc1_slot = var_uc_sc1;
        *var_uc_sc1_rv_slot = var_uc_sc1_rv;
        *var_uc_sc2_slot = var_uc_sc2;
        *var_uc_sc2_rv_slot = var_uc_sc2_rv;
        *var_uc_sc3_slot = var_uc_sc3;
        *var_uc_sc3_rv_slot = var_uc_sc3_rv;
        *var_uc_scp1_slot = var_uc_scp1;
        *var_uc_scp1_rv_slot = var_uc_scp1_rv;
        *var_uc_scp2_slot = var_uc_scp2;
        *var_uc_scp2_rv_slot = var_uc_scp2_rv;
        *var_uc_scp3_slot = var_uc_scp3;
        *var_uc_scp3_rv_slot = var_uc_scp3_rv;
        *var_uc_sub1_slot = var_uc_sub1;
        *var_uc_sub1_rv_slot = var_uc_sub1_rv;
        *var_uc_sub1snp_slot = var_uc_sub1snp;
        *var_uc_sub1snp_rv_slot = var_uc_sub1snp_rv;
        *var_uc_sub2_slot = var_uc_sub2;
        *var_uc_sub2_rv_slot = var_uc_sub2_rv;
        *var_uc_sub2snp_slot = var_uc_sub2snp;
        *var_uc_sub2snp_rv_slot = var_uc_sub2snp_rv;
        *var_uc_svbs_slot = var_uc_svbs;
        *var_uc_svbs_rv_slot = var_uc_svbs_rv;
        *var_uc_svds_slot = var_uc_svds;
        *var_uc_svds_rv_slot = var_uc_svds_rv;
        *var_uc_svdssnp_slot = var_uc_svdssnp;
        *var_uc_svdssnp_rv_slot = var_uc_svdssnp_rv;
        *var_uc_svgs_slot = var_uc_svgs;
        *var_uc_svgs_rv_slot = var_uc_svgs_rv;
        *var_uc_vfbc_slot = var_uc_vfbc;
        *var_uc_vfbc_rv_slot = var_uc_vfbc_rv;
        *var_uc_vfbover_slot = var_uc_vfbover;
        *var_uc_vfbover_rv_slot = var_uc_vfbover_rv;
        *var_uc_vmax_slot = var_uc_vmax;
        *var_uc_vmax_rv_slot = var_uc_vmax_rv;
        *var_uc_vtmp_slot = var_uc_vtmp;
        *var_uc_vtmp_rv_slot = var_uc_vtmp_rv;
        *var_uc_wl2_slot = var_uc_wl2;
        *var_uc_wl2_rv_slot = var_uc_wl2_rv;
        *var_uc_wvth0_slot = var_uc_wvth0;
        *var_uc_wvth0_rv_slot = var_uc_wvth0_rv;
        *var_wbin_slot = var_wbin;
        *var_wbin_rv_slot = var_wbin_rv;
        *var_wg_slot = var_wg;
        *var_wg_rv_slot = var_wg_rv;
        *var_wgate_slot = var_wgate;
        *var_wgate_rv_slot = var_wgate_rv;
    }

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        var_lbin: f64,
        var_lwbin: f64,
        var_wbin: f64,
        var_uc_cgbo_slot: &mut f64,
        var_uc_cgbo_rv_slot: &mut f64,
        var_uc_cgdo_slot: &mut f64,
        var_uc_cgdo_rv_slot: &mut f64,
        var_uc_cgso_slot: &mut f64,
        var_uc_cgso_rv_slot: &mut f64,
        var_uc_clm1_slot: &mut f64,
        var_uc_clm1_rv_slot: &mut f64,
        var_uc_clm2_slot: &mut f64,
        var_uc_clm2_dn0_slot: &mut f64,
        var_uc_clm2_dn10_slot: &mut f64,
        var_uc_clm2_dn11_slot: &mut f64,
        var_uc_clm2_dn14_slot: &mut f64,
        var_uc_clm2_dn2_slot: &mut f64,
        var_uc_clm2_dn4_slot: &mut f64,
        var_uc_clm2_dn5_slot: &mut f64,
        var_uc_clm2_dn6_slot: &mut f64,
        var_uc_clm2_dn7_slot: &mut f64,
        var_uc_clm2_dn8_slot: &mut f64,
        var_uc_clm2_dn9_slot: &mut f64,
        var_uc_clm2_rv_slot: &mut f64,
        var_uc_clm3_slot: &mut f64,
        var_uc_clm3_rv_slot: &mut f64,
        var_uc_cvdsover_slot: &mut f64,
        var_uc_cvdsover_rv_slot: &mut f64,
        var_uc_fn3_slot: &mut f64,
        var_uc_fn3_rv_slot: &mut f64,
        var_uc_fvbs_slot: &mut f64,
        var_uc_fvbs_rv_slot: &mut f64,
        var_uc_gidl1_slot: &mut f64,
        var_uc_gidl1_rv_slot: &mut f64,
        var_uc_gidl2_slot: &mut f64,
        var_uc_gidl2_rv_slot: &mut f64,
        var_uc_gleak1_slot: &mut f64,
        var_uc_gleak1_rv_slot: &mut f64,
        var_uc_gleak2_slot: &mut f64,
        var_uc_gleak2_rv_slot: &mut f64,
        var_uc_gleak3_slot: &mut f64,
        var_uc_gleak3_rv_slot: &mut f64,
        var_uc_gleak6_slot: &mut f64,
        var_uc_gleak6_rv_slot: &mut f64,
        var_uc_glkb1_slot: &mut f64,
        var_uc_glkb1_rv_slot: &mut f64,
        var_uc_glkb2_slot: &mut f64,
        var_uc_glkb2_rv_slot: &mut f64,
        var_uc_glksd1_slot: &mut f64,
        var_uc_glksd1_rv_slot: &mut f64,
        var_uc_glksd2_slot: &mut f64,
        var_uc_glksd2_rv_slot: &mut f64,
        var_uc_ibpc1_slot: &mut f64,
        var_uc_ibpc1_rv_slot: &mut f64,
        var_uc_ibpc2_slot: &mut f64,
        var_uc_ibpc2_rv_slot: &mut f64,
        var_uc_js0d_slot: &mut f64,
        var_uc_js0d_rv_slot: &mut f64,
        var_uc_js0s_slot: &mut f64,
        var_uc_js0s_rv_slot: &mut f64,
        var_uc_js0swd_slot: &mut f64,
        var_uc_js0swd_rv_slot: &mut f64,
        var_uc_muesti1_slot: &mut f64,
        var_uc_muesti1_rv_slot: &mut f64,
        var_uc_muesti2_slot: &mut f64,
        var_uc_muesti2_rv_slot: &mut f64,
        var_uc_muesti3_slot: &mut f64,
        var_uc_muesti3_rv_slot: &mut f64,
        var_uc_nfalp_slot: &mut f64,
        var_uc_nfalp_rv_slot: &mut f64,
        var_uc_njd_slot: &mut f64,
        var_uc_njd_rv_slot: &mut f64,
        var_uc_npext_slot: &mut f64,
        var_uc_npext_rv_slot: &mut f64,
        var_uc_nsti_slot: &mut f64,
        var_uc_nsti_rv_slot: &mut f64,
        var_uc_nsubpsti1_slot: &mut f64,
        var_uc_nsubpsti1_rv_slot: &mut f64,
        var_uc_nsubpsti2_slot: &mut f64,
        var_uc_nsubpsti2_rv_slot: &mut f64,
        var_uc_nsubpsti3_slot: &mut f64,
        var_uc_nsubpsti3_rv_slot: &mut f64,
        var_uc_powrat_slot: &mut f64,
        var_uc_powrat_rv_slot: &mut f64,
        var_uc_rd_slot: &mut f64,
        var_uc_rd22_slot: &mut f64,
        var_uc_rd22_rv_slot: &mut f64,
        var_uc_rd23_slot: &mut f64,
        var_uc_rd23_rv_slot: &mut f64,
        var_uc_rd24_slot: &mut f64,
        var_uc_rd24_rv_slot: &mut f64,
        var_uc_rd_rv_slot: &mut f64,
        var_uc_rdict1_slot: &mut f64,
        var_uc_rdict1_rv_slot: &mut f64,
        var_uc_rdov13_slot: &mut f64,
        var_uc_rdov13_rv_slot: &mut f64,
        var_uc_rdslp1_slot: &mut f64,
        var_uc_rdslp1_rv_slot: &mut f64,
        var_uc_rdvb_slot: &mut f64,
        var_uc_rdvb_rv_slot: &mut f64,
        var_uc_rdvd_slot: &mut f64,
        var_uc_rdvd_rv_slot: &mut f64,
        var_uc_rdvg11_slot: &mut f64,
        var_uc_rdvg11_rv_slot: &mut f64,
        var_uc_rs_slot: &mut f64,
        var_uc_rs_rv_slot: &mut f64,
        var_uc_rth0_slot: &mut f64,
        var_uc_rth0_rv_slot: &mut f64,
        var_uc_scsti1_slot: &mut f64,
        var_uc_scsti1_rv_slot: &mut f64,
        var_uc_scsti2_slot: &mut f64,
        var_uc_scsti2_rv_slot: &mut f64,
        var_uc_vdiffjd_slot: &mut f64,
        var_uc_vdiffjd_rv_slot: &mut f64,
        var_uc_vover_slot: &mut f64,
        var_uc_vover_rv_slot: &mut f64,
        var_uc_vthsti_slot: &mut f64,
        var_uc_vthsti_rv_slot: &mut f64,
        var_uc_wfc_slot: &mut f64,
        var_uc_wfc_rv_slot: &mut f64,
        var_uc_wsti_slot: &mut f64,
        var_uc_wsti_dn0_slot: &mut f64,
        var_uc_wsti_dn10_slot: &mut f64,
        var_uc_wsti_dn11_slot: &mut f64,
        var_uc_wsti_dn14_slot: &mut f64,
        var_uc_wsti_dn2_slot: &mut f64,
        var_uc_wsti_dn4_slot: &mut f64,
        var_uc_wsti_dn5_slot: &mut f64,
        var_uc_wsti_dn6_slot: &mut f64,
        var_uc_wsti_dn7_slot: &mut f64,
        var_uc_wsti_dn8_slot: &mut f64,
        var_uc_wsti_dn9_slot: &mut f64,
        var_uc_wsti_rv_slot: &mut f64,
    ) {
        let mut var_uc_cgbo: f64 = *var_uc_cgbo_slot;
        let mut var_uc_cgbo_rv: f64 = *var_uc_cgbo_rv_slot;
        let mut var_uc_cgdo: f64 = *var_uc_cgdo_slot;
        let mut var_uc_cgdo_rv: f64 = *var_uc_cgdo_rv_slot;
        let mut var_uc_cgso: f64 = *var_uc_cgso_slot;
        let mut var_uc_cgso_rv: f64 = *var_uc_cgso_rv_slot;
        let mut var_uc_clm1: f64 = *var_uc_clm1_slot;
        let mut var_uc_clm1_rv: f64 = *var_uc_clm1_rv_slot;
        let mut var_uc_clm2: f64 = *var_uc_clm2_slot;
        let mut var_uc_clm2_dn0: f64 = *var_uc_clm2_dn0_slot;
        let mut var_uc_clm2_dn10: f64 = *var_uc_clm2_dn10_slot;
        let mut var_uc_clm2_dn11: f64 = *var_uc_clm2_dn11_slot;
        let mut var_uc_clm2_dn14: f64 = *var_uc_clm2_dn14_slot;
        let mut var_uc_clm2_dn2: f64 = *var_uc_clm2_dn2_slot;
        let mut var_uc_clm2_dn4: f64 = *var_uc_clm2_dn4_slot;
        let mut var_uc_clm2_dn5: f64 = *var_uc_clm2_dn5_slot;
        let mut var_uc_clm2_dn6: f64 = *var_uc_clm2_dn6_slot;
        let mut var_uc_clm2_dn7: f64 = *var_uc_clm2_dn7_slot;
        let mut var_uc_clm2_dn8: f64 = *var_uc_clm2_dn8_slot;
        let mut var_uc_clm2_dn9: f64 = *var_uc_clm2_dn9_slot;
        let mut var_uc_clm2_rv: f64 = *var_uc_clm2_rv_slot;
        let mut var_uc_clm3: f64 = *var_uc_clm3_slot;
        let mut var_uc_clm3_rv: f64 = *var_uc_clm3_rv_slot;
        let mut var_uc_cvdsover: f64 = *var_uc_cvdsover_slot;
        let mut var_uc_cvdsover_rv: f64 = *var_uc_cvdsover_rv_slot;
        let mut var_uc_fn3: f64 = *var_uc_fn3_slot;
        let mut var_uc_fn3_rv: f64 = *var_uc_fn3_rv_slot;
        let mut var_uc_fvbs: f64 = *var_uc_fvbs_slot;
        let mut var_uc_fvbs_rv: f64 = *var_uc_fvbs_rv_slot;
        let mut var_uc_gidl1: f64 = *var_uc_gidl1_slot;
        let mut var_uc_gidl1_rv: f64 = *var_uc_gidl1_rv_slot;
        let mut var_uc_gidl2: f64 = *var_uc_gidl2_slot;
        let mut var_uc_gidl2_rv: f64 = *var_uc_gidl2_rv_slot;
        let mut var_uc_gleak1: f64 = *var_uc_gleak1_slot;
        let mut var_uc_gleak1_rv: f64 = *var_uc_gleak1_rv_slot;
        let mut var_uc_gleak2: f64 = *var_uc_gleak2_slot;
        let mut var_uc_gleak2_rv: f64 = *var_uc_gleak2_rv_slot;
        let mut var_uc_gleak3: f64 = *var_uc_gleak3_slot;
        let mut var_uc_gleak3_rv: f64 = *var_uc_gleak3_rv_slot;
        let mut var_uc_gleak6: f64 = *var_uc_gleak6_slot;
        let mut var_uc_gleak6_rv: f64 = *var_uc_gleak6_rv_slot;
        let mut var_uc_glkb1: f64 = *var_uc_glkb1_slot;
        let mut var_uc_glkb1_rv: f64 = *var_uc_glkb1_rv_slot;
        let mut var_uc_glkb2: f64 = *var_uc_glkb2_slot;
        let mut var_uc_glkb2_rv: f64 = *var_uc_glkb2_rv_slot;
        let mut var_uc_glksd1: f64 = *var_uc_glksd1_slot;
        let mut var_uc_glksd1_rv: f64 = *var_uc_glksd1_rv_slot;
        let mut var_uc_glksd2: f64 = *var_uc_glksd2_slot;
        let mut var_uc_glksd2_rv: f64 = *var_uc_glksd2_rv_slot;
        let mut var_uc_ibpc1: f64 = *var_uc_ibpc1_slot;
        let mut var_uc_ibpc1_rv: f64 = *var_uc_ibpc1_rv_slot;
        let mut var_uc_ibpc2: f64 = *var_uc_ibpc2_slot;
        let mut var_uc_ibpc2_rv: f64 = *var_uc_ibpc2_rv_slot;
        let mut var_uc_js0d: f64 = *var_uc_js0d_slot;
        let mut var_uc_js0d_rv: f64 = *var_uc_js0d_rv_slot;
        let mut var_uc_js0s: f64 = *var_uc_js0s_slot;
        let mut var_uc_js0s_rv: f64 = *var_uc_js0s_rv_slot;
        let mut var_uc_js0swd: f64 = *var_uc_js0swd_slot;
        let mut var_uc_js0swd_rv: f64 = *var_uc_js0swd_rv_slot;
        let mut var_uc_muesti1: f64 = *var_uc_muesti1_slot;
        let mut var_uc_muesti1_rv: f64 = *var_uc_muesti1_rv_slot;
        let mut var_uc_muesti2: f64 = *var_uc_muesti2_slot;
        let mut var_uc_muesti2_rv: f64 = *var_uc_muesti2_rv_slot;
        let mut var_uc_muesti3: f64 = *var_uc_muesti3_slot;
        let mut var_uc_muesti3_rv: f64 = *var_uc_muesti3_rv_slot;
        let mut var_uc_nfalp: f64 = *var_uc_nfalp_slot;
        let mut var_uc_nfalp_rv: f64 = *var_uc_nfalp_rv_slot;
        let mut var_uc_njd: f64 = *var_uc_njd_slot;
        let mut var_uc_njd_rv: f64 = *var_uc_njd_rv_slot;
        let mut var_uc_npext: f64 = *var_uc_npext_slot;
        let mut var_uc_npext_rv: f64 = *var_uc_npext_rv_slot;
        let mut var_uc_nsti: f64 = *var_uc_nsti_slot;
        let mut var_uc_nsti_rv: f64 = *var_uc_nsti_rv_slot;
        let mut var_uc_nsubpsti1: f64 = *var_uc_nsubpsti1_slot;
        let mut var_uc_nsubpsti1_rv: f64 = *var_uc_nsubpsti1_rv_slot;
        let mut var_uc_nsubpsti2: f64 = *var_uc_nsubpsti2_slot;
        let mut var_uc_nsubpsti2_rv: f64 = *var_uc_nsubpsti2_rv_slot;
        let mut var_uc_nsubpsti3: f64 = *var_uc_nsubpsti3_slot;
        let mut var_uc_nsubpsti3_rv: f64 = *var_uc_nsubpsti3_rv_slot;
        let mut var_uc_powrat: f64 = *var_uc_powrat_slot;
        let mut var_uc_powrat_rv: f64 = *var_uc_powrat_rv_slot;
        let mut var_uc_rd: f64 = *var_uc_rd_slot;
        let mut var_uc_rd22: f64 = *var_uc_rd22_slot;
        let mut var_uc_rd22_rv: f64 = *var_uc_rd22_rv_slot;
        let mut var_uc_rd23: f64 = *var_uc_rd23_slot;
        let mut var_uc_rd23_rv: f64 = *var_uc_rd23_rv_slot;
        let mut var_uc_rd24: f64 = *var_uc_rd24_slot;
        let mut var_uc_rd24_rv: f64 = *var_uc_rd24_rv_slot;
        let mut var_uc_rd_rv: f64 = *var_uc_rd_rv_slot;
        let mut var_uc_rdict1: f64 = *var_uc_rdict1_slot;
        let mut var_uc_rdict1_rv: f64 = *var_uc_rdict1_rv_slot;
        let mut var_uc_rdov13: f64 = *var_uc_rdov13_slot;
        let mut var_uc_rdov13_rv: f64 = *var_uc_rdov13_rv_slot;
        let mut var_uc_rdslp1: f64 = *var_uc_rdslp1_slot;
        let mut var_uc_rdslp1_rv: f64 = *var_uc_rdslp1_rv_slot;
        let mut var_uc_rdvb: f64 = *var_uc_rdvb_slot;
        let mut var_uc_rdvb_rv: f64 = *var_uc_rdvb_rv_slot;
        let mut var_uc_rdvd: f64 = *var_uc_rdvd_slot;
        let mut var_uc_rdvd_rv: f64 = *var_uc_rdvd_rv_slot;
        let mut var_uc_rdvg11: f64 = *var_uc_rdvg11_slot;
        let mut var_uc_rdvg11_rv: f64 = *var_uc_rdvg11_rv_slot;
        let mut var_uc_rs: f64 = *var_uc_rs_slot;
        let mut var_uc_rs_rv: f64 = *var_uc_rs_rv_slot;
        let mut var_uc_rth0: f64 = *var_uc_rth0_slot;
        let mut var_uc_rth0_rv: f64 = *var_uc_rth0_rv_slot;
        let mut var_uc_scsti1: f64 = *var_uc_scsti1_slot;
        let mut var_uc_scsti1_rv: f64 = *var_uc_scsti1_rv_slot;
        let mut var_uc_scsti2: f64 = *var_uc_scsti2_slot;
        let mut var_uc_scsti2_rv: f64 = *var_uc_scsti2_rv_slot;
        let mut var_uc_vdiffjd: f64 = *var_uc_vdiffjd_slot;
        let mut var_uc_vdiffjd_rv: f64 = *var_uc_vdiffjd_rv_slot;
        let mut var_uc_vover: f64 = *var_uc_vover_slot;
        let mut var_uc_vover_rv: f64 = *var_uc_vover_rv_slot;
        let mut var_uc_vthsti: f64 = *var_uc_vthsti_slot;
        let mut var_uc_vthsti_rv: f64 = *var_uc_vthsti_rv_slot;
        let mut var_uc_wfc: f64 = *var_uc_wfc_slot;
        let mut var_uc_wfc_rv: f64 = *var_uc_wfc_rv_slot;
        let mut var_uc_wsti: f64 = *var_uc_wsti_slot;
        let mut var_uc_wsti_dn0: f64 = *var_uc_wsti_dn0_slot;
        let mut var_uc_wsti_dn10: f64 = *var_uc_wsti_dn10_slot;
        let mut var_uc_wsti_dn11: f64 = *var_uc_wsti_dn11_slot;
        let mut var_uc_wsti_dn14: f64 = *var_uc_wsti_dn14_slot;
        let mut var_uc_wsti_dn2: f64 = *var_uc_wsti_dn2_slot;
        let mut var_uc_wsti_dn4: f64 = *var_uc_wsti_dn4_slot;
        let mut var_uc_wsti_dn5: f64 = *var_uc_wsti_dn5_slot;
        let mut var_uc_wsti_dn6: f64 = *var_uc_wsti_dn6_slot;
        let mut var_uc_wsti_dn7: f64 = *var_uc_wsti_dn7_slot;
        let mut var_uc_wsti_dn8: f64 = *var_uc_wsti_dn8_slot;
        let mut var_uc_wsti_dn9: f64 = *var_uc_wsti_dn9_slot;
        let mut var_uc_wsti_rv: f64 = *var_uc_wsti_rv_slot;

        let assign8510_e3860: f64 = (p.p592 / var_lbin);
        let assign8510_e3861: f64 = (p.p196 + assign8510_e3860);
        let assign8510_e3864: f64 = (p.p680 / var_wbin);
        let assign8510_e3865: f64 = (assign8510_e3861 + assign8510_e3864);
        let assign8510_e3868: f64 = (p.p768 / var_lwbin);
        let assign8510_e3869: f64 = (assign8510_e3865 + assign8510_e3868);
        var_uc_fn3 = assign8510_e3869;
        var_uc_fn3_rv = 0.0;

        let assign8520_e3873: f64 = (p.p593 / var_lbin);
        let assign8520_e3874: f64 = (p.p197 + assign8520_e3873);
        let assign8520_e3877: f64 = (p.p681 / var_wbin);
        let assign8520_e3878: f64 = (assign8520_e3874 + assign8520_e3877);
        let assign8520_e3881: f64 = (p.p769 / var_lwbin);
        let assign8520_e3882: f64 = (assign8520_e3878 + assign8520_e3881);
        var_uc_fvbs = assign8520_e3882;
        var_uc_fvbs_rv = 0.0;

        let assign8530_e3886: f64 = (p.p594 / var_lbin);
        let assign8530_e3887: f64 = (p.p204 + assign8530_e3886);
        let assign8530_e3890: f64 = (p.p682 / var_wbin);
        let assign8530_e3891: f64 = (assign8530_e3887 + assign8530_e3890);
        let assign8530_e3894: f64 = (p.p770 / var_lwbin);
        let assign8530_e3895: f64 = (assign8530_e3891 + assign8530_e3894);
        var_uc_nsti = assign8530_e3895;
        var_uc_nsti_rv = 0.0;

        let assign8540_e3899: f64 = (p.p595 / var_lbin);
        let assign8540_e3900: f64 = (p.p205 + assign8540_e3899);
        let assign8540_e3903: f64 = (p.p683 / var_wbin);
        let assign8540_e3904: f64 = (assign8540_e3900 + assign8540_e3903);
        let assign8540_e3907: f64 = (p.p771 / var_lwbin);
        let assign8540_e3908: f64 = (assign8540_e3904 + assign8540_e3907);
        var_uc_wsti = assign8540_e3908;
        var_uc_wsti_dn0 = 0.0;
        var_uc_wsti_dn2 = 0.0;
        var_uc_wsti_dn4 = 0.0;
        var_uc_wsti_dn5 = 0.0;
        var_uc_wsti_dn6 = 0.0;
        var_uc_wsti_dn7 = 0.0;
        var_uc_wsti_dn8 = 0.0;
        var_uc_wsti_dn9 = 0.0;
        var_uc_wsti_dn10 = 0.0;
        var_uc_wsti_dn11 = 0.0;
        var_uc_wsti_dn14 = 0.0;
        var_uc_wsti_rv = 0.0;

        let assign8550_e3912: f64 = (p.p596 / var_lbin);
        let assign8550_e3913: f64 = (p.p210 + assign8550_e3912);
        let assign8550_e3916: f64 = (p.p684 / var_wbin);
        let assign8550_e3917: f64 = (assign8550_e3913 + assign8550_e3916);
        let assign8550_e3920: f64 = (p.p772 / var_lwbin);
        let assign8550_e3921: f64 = (assign8550_e3917 + assign8550_e3920);
        var_uc_scsti1 = assign8550_e3921;
        var_uc_scsti1_rv = 0.0;

        let assign8560_e3925: f64 = (p.p597 / var_lbin);
        let assign8560_e3926: f64 = (p.p211 + assign8560_e3925);
        let assign8560_e3929: f64 = (p.p685 / var_wbin);
        let assign8560_e3930: f64 = (assign8560_e3926 + assign8560_e3929);
        let assign8560_e3933: f64 = (p.p773 / var_lwbin);
        let assign8560_e3934: f64 = (assign8560_e3930 + assign8560_e3933);
        var_uc_scsti2 = assign8560_e3934;
        var_uc_scsti2_rv = 0.0;

        let assign8570_e3938: f64 = (p.p598 / var_lbin);
        let assign8570_e3939: f64 = (p.p212 + assign8570_e3938);
        let assign8570_e3942: f64 = (p.p686 / var_wbin);
        let assign8570_e3943: f64 = (assign8570_e3939 + assign8570_e3942);
        let assign8570_e3946: f64 = (p.p774 / var_lwbin);
        let assign8570_e3947: f64 = (assign8570_e3943 + assign8570_e3946);
        var_uc_vthsti = assign8570_e3947;
        var_uc_vthsti_rv = 0.0;

        let assign8580_e3951: f64 = (p.p599 / var_lbin);
        let assign8580_e3952: f64 = (p.p214 + assign8580_e3951);
        let assign8580_e3955: f64 = (p.p687 / var_wbin);
        let assign8580_e3956: f64 = (assign8580_e3952 + assign8580_e3955);
        let assign8580_e3959: f64 = (p.p775 / var_lwbin);
        let assign8580_e3960: f64 = (assign8580_e3956 + assign8580_e3959);
        var_uc_muesti1 = assign8580_e3960;
        var_uc_muesti1_rv = 0.0;

        let assign8590_e3964: f64 = (p.p600 / var_lbin);
        let assign8590_e3965: f64 = (p.p215 + assign8590_e3964);
        let assign8590_e3968: f64 = (p.p688 / var_wbin);
        let assign8590_e3969: f64 = (assign8590_e3965 + assign8590_e3968);
        let assign8590_e3972: f64 = (p.p776 / var_lwbin);
        let assign8590_e3973: f64 = (assign8590_e3969 + assign8590_e3972);
        var_uc_muesti2 = assign8590_e3973;
        var_uc_muesti2_rv = 0.0;

        let assign8600_e3977: f64 = (p.p601 / var_lbin);
        let assign8600_e3978: f64 = (p.p216 + assign8600_e3977);
        let assign8600_e3981: f64 = (p.p689 / var_wbin);
        let assign8600_e3982: f64 = (assign8600_e3978 + assign8600_e3981);
        let assign8600_e3985: f64 = (p.p777 / var_lwbin);
        let assign8600_e3986: f64 = (assign8600_e3982 + assign8600_e3985);
        var_uc_muesti3 = assign8600_e3986;
        var_uc_muesti3_rv = 0.0;

        let assign8610_e3990: f64 = (p.p602 / var_lbin);
        let assign8610_e3991: f64 = (p.p217 + assign8610_e3990);
        let assign8610_e3994: f64 = (p.p690 / var_wbin);
        let assign8610_e3995: f64 = (assign8610_e3991 + assign8610_e3994);
        let assign8610_e3998: f64 = (p.p778 / var_lwbin);
        let assign8610_e3999: f64 = (assign8610_e3995 + assign8610_e3998);
        var_uc_nsubpsti1 = assign8610_e3999;
        var_uc_nsubpsti1_rv = 0.0;

        let assign8620_e4003: f64 = (p.p603 / var_lbin);
        let assign8620_e4004: f64 = (p.p218 + assign8620_e4003);
        let assign8620_e4007: f64 = (p.p691 / var_wbin);
        let assign8620_e4008: f64 = (assign8620_e4004 + assign8620_e4007);
        let assign8620_e4011: f64 = (p.p779 / var_lwbin);
        let assign8620_e4012: f64 = (assign8620_e4008 + assign8620_e4011);
        var_uc_nsubpsti2 = assign8620_e4012;
        var_uc_nsubpsti2_rv = 0.0;

        let assign8630_e4016: f64 = (p.p604 / var_lbin);
        let assign8630_e4017: f64 = (p.p219 + assign8630_e4016);
        let assign8630_e4020: f64 = (p.p692 / var_wbin);
        let assign8630_e4021: f64 = (assign8630_e4017 + assign8630_e4020);
        let assign8630_e4024: f64 = (p.p780 / var_lwbin);
        let assign8630_e4025: f64 = (assign8630_e4021 + assign8630_e4024);
        var_uc_nsubpsti3 = assign8630_e4025;
        var_uc_nsubpsti3_rv = 0.0;

        let assign8640_e4029: f64 = (p.p605 / var_lbin);
        let assign8640_e4030: f64 = (p.p269 + assign8640_e4029);
        let assign8640_e4033: f64 = (p.p693 / var_wbin);
        let assign8640_e4034: f64 = (assign8640_e4030 + assign8640_e4033);
        let assign8640_e4037: f64 = (p.p781 / var_lwbin);
        let assign8640_e4038: f64 = (assign8640_e4034 + assign8640_e4037);
        var_uc_cgso = assign8640_e4038;
        var_uc_cgso_rv = 0.0;

        let assign8650_e4042: f64 = (p.p606 / var_lbin);
        let assign8650_e4043: f64 = (p.p268 + assign8650_e4042);
        let assign8650_e4046: f64 = (p.p694 / var_wbin);
        let assign8650_e4047: f64 = (assign8650_e4043 + assign8650_e4046);
        let assign8650_e4050: f64 = (p.p782 / var_lwbin);
        let assign8650_e4051: f64 = (assign8650_e4047 + assign8650_e4050);
        var_uc_cgdo = assign8650_e4051;
        var_uc_cgdo_rv = 0.0;

        let assign8660_e4055: f64 = (p.p607 / var_lbin);
        let assign8660_e4056: f64 = (p.p226 + assign8660_e4055);
        let assign8660_e4059: f64 = (p.p695 / var_wbin);
        let assign8660_e4060: f64 = (assign8660_e4056 + assign8660_e4059);
        let assign8660_e4063: f64 = (p.p783 / var_lwbin);
        let assign8660_e4064: f64 = (assign8660_e4060 + assign8660_e4063);
        var_uc_clm1 = assign8660_e4064;
        var_uc_clm1_rv = 0.0;

        let assign8670_e4068: f64 = (p.p608 / var_lbin);
        let assign8670_e4069: f64 = (p.p227 + assign8670_e4068);
        let assign8670_e4072: f64 = (p.p696 / var_wbin);
        let assign8670_e4073: f64 = (assign8670_e4069 + assign8670_e4072);
        let assign8670_e4076: f64 = (p.p784 / var_lwbin);
        let assign8670_e4077: f64 = (assign8670_e4073 + assign8670_e4076);
        var_uc_clm2 = assign8670_e4077;
        var_uc_clm2_dn0 = 0.0;
        var_uc_clm2_dn2 = 0.0;
        var_uc_clm2_dn4 = 0.0;
        var_uc_clm2_dn5 = 0.0;
        var_uc_clm2_dn6 = 0.0;
        var_uc_clm2_dn7 = 0.0;
        var_uc_clm2_dn8 = 0.0;
        var_uc_clm2_dn9 = 0.0;
        var_uc_clm2_dn10 = 0.0;
        var_uc_clm2_dn11 = 0.0;
        var_uc_clm2_dn14 = 0.0;
        var_uc_clm2_rv = 0.0;

        let assign8680_e4081: f64 = (p.p609 / var_lbin);
        let assign8680_e4082: f64 = (p.p228 + assign8680_e4081);
        let assign8680_e4085: f64 = (p.p697 / var_wbin);
        let assign8680_e4086: f64 = (assign8680_e4082 + assign8680_e4085);
        let assign8680_e4089: f64 = (p.p785 / var_lwbin);
        let assign8680_e4090: f64 = (assign8680_e4086 + assign8680_e4089);
        var_uc_clm3 = assign8680_e4090;
        var_uc_clm3_rv = 0.0;

        let assign8690_e4094: f64 = (p.p610 / var_lbin);
        let assign8690_e4095: f64 = (p.p232 + assign8690_e4094);
        let assign8690_e4098: f64 = (p.p698 / var_wbin);
        let assign8690_e4099: f64 = (assign8690_e4095 + assign8690_e4098);
        let assign8690_e4102: f64 = (p.p786 / var_lwbin);
        let assign8690_e4103: f64 = (assign8690_e4099 + assign8690_e4102);
        var_uc_wfc = assign8690_e4103;
        var_uc_wfc_rv = 0.0;

        let assign8700_e4107: f64 = (p.p611 / var_lbin);
        let assign8700_e4108: f64 = (p.p240 + assign8700_e4107);
        let assign8700_e4111: f64 = (p.p699 / var_wbin);
        let assign8700_e4112: f64 = (assign8700_e4108 + assign8700_e4111);
        let assign8700_e4115: f64 = (p.p787 / var_lwbin);
        let assign8700_e4116: f64 = (assign8700_e4112 + assign8700_e4115);
        var_uc_gidl1 = assign8700_e4116;
        var_uc_gidl1_rv = 0.0;

        let assign8710_e4120: f64 = (p.p612 / var_lbin);
        let assign8710_e4121: f64 = (p.p241 + assign8710_e4120);
        let assign8710_e4124: f64 = (p.p700 / var_wbin);
        let assign8710_e4125: f64 = (assign8710_e4121 + assign8710_e4124);
        let assign8710_e4128: f64 = (p.p788 / var_lwbin);
        let assign8710_e4129: f64 = (assign8710_e4125 + assign8710_e4128);
        var_uc_gidl2 = assign8710_e4129;
        var_uc_gidl2_rv = 0.0;

        let assign8720_e4133: f64 = (p.p613 / var_lbin);
        let assign8720_e4134: f64 = (p.p245 + assign8720_e4133);
        let assign8720_e4137: f64 = (p.p701 / var_wbin);
        let assign8720_e4138: f64 = (assign8720_e4134 + assign8720_e4137);
        let assign8720_e4141: f64 = (p.p789 / var_lwbin);
        let assign8720_e4142: f64 = (assign8720_e4138 + assign8720_e4141);
        var_uc_gleak1 = assign8720_e4142;
        var_uc_gleak1_rv = 0.0;

        let assign8730_e4146: f64 = (p.p614 / var_lbin);
        let assign8730_e4147: f64 = (p.p246 + assign8730_e4146);
        let assign8730_e4150: f64 = (p.p702 / var_wbin);
        let assign8730_e4151: f64 = (assign8730_e4147 + assign8730_e4150);
        let assign8730_e4154: f64 = (p.p790 / var_lwbin);
        let assign8730_e4155: f64 = (assign8730_e4151 + assign8730_e4154);
        var_uc_gleak2 = assign8730_e4155;
        var_uc_gleak2_rv = 0.0;

        let assign8740_e4159: f64 = (p.p615 / var_lbin);
        let assign8740_e4160: f64 = (p.p247 + assign8740_e4159);
        let assign8740_e4163: f64 = (p.p703 / var_wbin);
        let assign8740_e4164: f64 = (assign8740_e4160 + assign8740_e4163);
        let assign8740_e4167: f64 = (p.p791 / var_lwbin);
        let assign8740_e4168: f64 = (assign8740_e4164 + assign8740_e4167);
        var_uc_gleak3 = assign8740_e4168;
        var_uc_gleak3_rv = 0.0;

        let assign8750_e4172: f64 = (p.p616 / var_lbin);
        let assign8750_e4173: f64 = (p.p250 + assign8750_e4172);
        let assign8750_e4176: f64 = (p.p704 / var_wbin);
        let assign8750_e4177: f64 = (assign8750_e4173 + assign8750_e4176);
        let assign8750_e4180: f64 = (p.p792 / var_lwbin);
        let assign8750_e4181: f64 = (assign8750_e4177 + assign8750_e4180);
        var_uc_gleak6 = assign8750_e4181;
        var_uc_gleak6_rv = 0.0;

        let assign8760_e4185: f64 = (p.p617 / var_lbin);
        let assign8760_e4186: f64 = (p.p253 + assign8760_e4185);
        let assign8760_e4189: f64 = (p.p705 / var_wbin);
        let assign8760_e4190: f64 = (assign8760_e4186 + assign8760_e4189);
        let assign8760_e4193: f64 = (p.p793 / var_lwbin);
        let assign8760_e4194: f64 = (assign8760_e4190 + assign8760_e4193);
        var_uc_glksd1 = assign8760_e4194;
        var_uc_glksd1_rv = 0.0;

        let assign8770_e4198: f64 = (p.p618 / var_lbin);
        let assign8770_e4199: f64 = (p.p254 + assign8770_e4198);
        let assign8770_e4202: f64 = (p.p706 / var_wbin);
        let assign8770_e4203: f64 = (assign8770_e4199 + assign8770_e4202);
        let assign8770_e4206: f64 = (p.p794 / var_lwbin);
        let assign8770_e4207: f64 = (assign8770_e4203 + assign8770_e4206);
        var_uc_glksd2 = assign8770_e4207;
        var_uc_glksd2_rv = 0.0;

        let assign8780_e4211: f64 = (p.p619 / var_lbin);
        let assign8780_e4212: f64 = (p.p256 + assign8780_e4211);
        let assign8780_e4215: f64 = (p.p707 / var_wbin);
        let assign8780_e4216: f64 = (assign8780_e4212 + assign8780_e4215);
        let assign8780_e4219: f64 = (p.p795 / var_lwbin);
        let assign8780_e4220: f64 = (assign8780_e4216 + assign8780_e4219);
        var_uc_glkb1 = assign8780_e4220;
        var_uc_glkb1_rv = 0.0;

        let assign8790_e4224: f64 = (p.p620 / var_lbin);
        let assign8790_e4225: f64 = (p.p257 + assign8790_e4224);
        let assign8790_e4228: f64 = (p.p708 / var_wbin);
        let assign8790_e4229: f64 = (assign8790_e4225 + assign8790_e4228);
        let assign8790_e4232: f64 = (p.p796 / var_lwbin);
        let assign8790_e4233: f64 = (assign8790_e4229 + assign8790_e4232);
        var_uc_glkb2 = assign8790_e4233;
        var_uc_glkb2_rv = 0.0;

        let assign8810_e4250: f64 = (p.p622 / var_lbin);
        let assign8810_e4251: f64 = (p.p265 + assign8810_e4250);
        let assign8810_e4254: f64 = (p.p710 / var_wbin);
        let assign8810_e4255: f64 = (assign8810_e4251 + assign8810_e4254);
        let assign8810_e4258: f64 = (p.p798 / var_lwbin);
        let assign8810_e4259: f64 = (assign8810_e4255 + assign8810_e4258);
        var_uc_nfalp = assign8810_e4259;
        var_uc_nfalp_rv = 0.0;

        let assign8820_e4263: f64 = (p.p623 / var_lbin);
        let assign8820_e4264: f64 = (p.p278 + assign8820_e4263);
        let assign8820_e4267: f64 = (p.p711 / var_wbin);
        let assign8820_e4268: f64 = (assign8820_e4264 + assign8820_e4267);
        let assign8820_e4271: f64 = (p.p799 / var_lwbin);
        let assign8820_e4272: f64 = (assign8820_e4268 + assign8820_e4271);
        var_uc_ibpc1 = assign8820_e4272;
        var_uc_ibpc1_rv = 0.0;

        let assign8830_e4276: f64 = (p.p624 / var_lbin);
        let assign8830_e4277: f64 = (p.p281 + assign8830_e4276);
        let assign8830_e4280: f64 = (p.p712 / var_wbin);
        let assign8830_e4281: f64 = (assign8830_e4277 + assign8830_e4280);
        let assign8830_e4284: f64 = (p.p800 / var_lwbin);
        let assign8830_e4285: f64 = (assign8830_e4281 + assign8830_e4284);
        var_uc_ibpc2 = assign8830_e4285;
        var_uc_ibpc2_rv = 0.0;

        let assign8840_e4289: f64 = (p.p625 / var_lbin);
        let assign8840_e4290: f64 = (p.p79 + assign8840_e4289);
        let assign8840_e4293: f64 = (p.p713 / var_wbin);
        let assign8840_e4294: f64 = (assign8840_e4290 + assign8840_e4293);
        let assign8840_e4297: f64 = (p.p801 / var_lwbin);
        let assign8840_e4298: f64 = (assign8840_e4294 + assign8840_e4297);
        var_uc_cgbo = assign8840_e4298;
        var_uc_cgbo_rv = 0.0;

        let assign8850_e4302: f64 = (p.p626 / var_lbin);
        let assign8850_e4303: f64 = (p.p86 + assign8850_e4302);
        let assign8850_e4306: f64 = (p.p714 / var_wbin);
        let assign8850_e4307: f64 = (assign8850_e4303 + assign8850_e4306);
        let assign8850_e4310: f64 = (p.p802 / var_lwbin);
        let assign8850_e4311: f64 = (assign8850_e4307 + assign8850_e4310);
        var_uc_cvdsover = assign8850_e4311;
        var_uc_cvdsover_rv = 0.0;

        let assign8870_e4328: f64 = (p.p628 / var_lbin);
        let assign8870_e4329: f64 = (p.p76 + assign8870_e4328);
        let assign8870_e4332: f64 = (p.p716 / var_wbin);
        let assign8870_e4333: f64 = (assign8870_e4329 + assign8870_e4332);
        let assign8870_e4336: f64 = (p.p804 / var_lwbin);
        let assign8870_e4337: f64 = (assign8870_e4333 + assign8870_e4336);
        var_uc_npext = assign8870_e4337;
        var_uc_npext_rv = 0.0;

        let assign8880_e4341: f64 = (p.p629 / var_lbin);
        let assign8880_e4342: f64 = (p.p81 + assign8880_e4341);
        let assign8880_e4345: f64 = (p.p717 / var_wbin);
        let assign8880_e4346: f64 = (assign8880_e4342 + assign8880_e4345);
        let assign8880_e4349: f64 = (p.p805 / var_lwbin);
        let assign8880_e4350: f64 = (assign8880_e4346 + assign8880_e4349);
        var_uc_powrat = assign8880_e4350;
        var_uc_powrat_rv = 0.0;

        let assign8890_e4354: f64 = (p.p630 / var_lbin);
        let assign8890_e4355: f64 = (p.p74 + assign8890_e4354);
        let assign8890_e4358: f64 = (p.p718 / var_wbin);
        let assign8890_e4359: f64 = (assign8890_e4355 + assign8890_e4358);
        let assign8890_e4362: f64 = (p.p806 / var_lwbin);
        let assign8890_e4363: f64 = (assign8890_e4359 + assign8890_e4362);
        var_uc_rd = assign8890_e4363;
        var_uc_rd_rv = 0.0;

        let assign8900_e4367: f64 = (p.p631 / var_lbin);
        let assign8900_e4368: f64 = (p.p298 + assign8900_e4367);
        let assign8900_e4371: f64 = (p.p719 / var_wbin);
        let assign8900_e4372: f64 = (assign8900_e4368 + assign8900_e4371);
        let assign8900_e4375: f64 = (p.p807 / var_lwbin);
        let assign8900_e4376: f64 = (assign8900_e4372 + assign8900_e4375);
        var_uc_rd22 = assign8900_e4376;
        var_uc_rd22_rv = 0.0;

        let assign8910_e4380: f64 = (p.p632 / var_lbin);
        let assign8910_e4381: f64 = (p.p83 + assign8910_e4380);
        let assign8910_e4384: f64 = (p.p720 / var_wbin);
        let assign8910_e4385: f64 = (assign8910_e4381 + assign8910_e4384);
        let assign8910_e4388: f64 = (p.p808 / var_lwbin);
        let assign8910_e4389: f64 = (assign8910_e4385 + assign8910_e4388);
        var_uc_rd23 = assign8910_e4389;
        var_uc_rd23_rv = 0.0;

        let assign8920_e4393: f64 = (p.p633 / var_lbin);
        let assign8920_e4394: f64 = (p.p84 + assign8920_e4393);
        let assign8920_e4397: f64 = (p.p721 / var_wbin);
        let assign8920_e4398: f64 = (assign8920_e4394 + assign8920_e4397);
        let assign8920_e4401: f64 = (p.p809 / var_lwbin);
        let assign8920_e4402: f64 = (assign8920_e4398 + assign8920_e4401);
        var_uc_rd24 = assign8920_e4402;
        var_uc_rd24_rv = 0.0;

        let assign8930_e4406: f64 = (p.p634 / var_lbin);
        let assign8930_e4407: f64 = (p.p62 + assign8930_e4406);
        let assign8930_e4410: f64 = (p.p722 / var_wbin);
        let assign8930_e4411: f64 = (assign8930_e4407 + assign8930_e4410);
        let assign8930_e4414: f64 = (p.p810 / var_lwbin);
        let assign8930_e4415: f64 = (assign8930_e4411 + assign8930_e4414);
        var_uc_rdict1 = assign8930_e4415;
        var_uc_rdict1_rv = 0.0;

        let assign8940_e4419: f64 = (p.p635 / var_lbin);
        let assign8940_e4420: f64 = (p.p59 + assign8940_e4419);
        let assign8940_e4423: f64 = (p.p723 / var_wbin);
        let assign8940_e4424: f64 = (assign8940_e4420 + assign8940_e4423);
        let assign8940_e4427: f64 = (p.p811 / var_lwbin);
        let assign8940_e4428: f64 = (assign8940_e4424 + assign8940_e4427);
        var_uc_rdov13 = assign8940_e4428;
        var_uc_rdov13_rv = 0.0;

        let assign8950_e4432: f64 = (p.p636 / var_lbin);
        let assign8950_e4433: f64 = (p.p60 + assign8950_e4432);
        let assign8950_e4436: f64 = (p.p724 / var_wbin);
        let assign8950_e4437: f64 = (assign8950_e4433 + assign8950_e4436);
        let assign8950_e4440: f64 = (p.p812 / var_lwbin);
        let assign8950_e4441: f64 = (assign8950_e4437 + assign8950_e4440);
        var_uc_rdslp1 = assign8950_e4441;
        var_uc_rdslp1_rv = 0.0;

        let assign8960_e4445: f64 = (p.p637 / var_lbin);
        let assign8960_e4446: f64 = (p.p85 + assign8960_e4445);
        let assign8960_e4449: f64 = (p.p725 / var_wbin);
        let assign8960_e4450: f64 = (assign8960_e4446 + assign8960_e4449);
        let assign8960_e4453: f64 = (p.p813 / var_lwbin);
        let assign8960_e4454: f64 = (assign8960_e4450 + assign8960_e4453);
        var_uc_rdvb = assign8960_e4454;
        var_uc_rdvb_rv = 0.0;

        let assign8970_e4458: f64 = (p.p638 / var_lbin);
        let assign8970_e4459: f64 = (p.p82 + assign8970_e4458);
        let assign8970_e4462: f64 = (p.p726 / var_wbin);
        let assign8970_e4463: f64 = (assign8970_e4459 + assign8970_e4462);
        let assign8970_e4466: f64 = (p.p814 / var_lwbin);
        let assign8970_e4467: f64 = (assign8970_e4463 + assign8970_e4466);
        var_uc_rdvd = assign8970_e4467;
        var_uc_rdvd_rv = 0.0;

        let assign8980_e4471: f64 = (p.p639 / var_lbin);
        let assign8980_e4472: f64 = (p.p61 + assign8980_e4471);
        let assign8980_e4475: f64 = (p.p727 / var_wbin);
        let assign8980_e4476: f64 = (assign8980_e4472 + assign8980_e4475);
        let assign8980_e4479: f64 = (p.p815 / var_lwbin);
        let assign8980_e4480: f64 = (assign8980_e4476 + assign8980_e4479);
        var_uc_rdvg11 = assign8980_e4480;
        var_uc_rdvg11_rv = 0.0;

        let assign8990_e4484: f64 = (p.p640 / var_lbin);
        let assign8990_e4485: f64 = (p.p75 + assign8990_e4484);
        let assign8990_e4488: f64 = (p.p728 / var_wbin);
        let assign8990_e4489: f64 = (assign8990_e4485 + assign8990_e4488);
        let assign8990_e4492: f64 = (p.p816 / var_lwbin);
        let assign8990_e4493: f64 = (assign8990_e4489 + assign8990_e4492);
        var_uc_rs = assign8990_e4493;
        var_uc_rs_rv = 0.0;

        let assign9000_e4497: f64 = (p.p641 / var_lbin);
        let assign9000_e4498: f64 = (p.p80 + assign9000_e4497);
        let assign9000_e4501: f64 = (p.p729 / var_wbin);
        let assign9000_e4502: f64 = (assign9000_e4498 + assign9000_e4501);
        let assign9000_e4505: f64 = (p.p817 / var_lwbin);
        let assign9000_e4506: f64 = (assign9000_e4502 + assign9000_e4505);
        var_uc_rth0 = assign9000_e4506;
        var_uc_rth0_rv = 0.0;

        let assign9010_e4510: f64 = (p.p642 / var_lbin);
        let assign9010_e4511: f64 = (p.p77 + assign9010_e4510);
        let assign9010_e4514: f64 = (p.p730 / var_wbin);
        let assign9010_e4515: f64 = (assign9010_e4511 + assign9010_e4514);
        let assign9010_e4518: f64 = (p.p818 / var_lwbin);
        let assign9010_e4519: f64 = (assign9010_e4515 + assign9010_e4518);
        var_uc_vover = assign9010_e4519;
        var_uc_vover_rv = 0.0;

        let assign9020_e4523: f64 = (p.p824 / var_lbin);
        let assign9020_e4524: f64 = (p.p493 + assign9020_e4523);
        let assign9020_e4527: f64 = (p.p839 / var_wbin);
        let assign9020_e4528: f64 = (assign9020_e4524 + assign9020_e4527);
        let assign9020_e4531: f64 = (p.p854 / var_lwbin);
        let assign9020_e4532: f64 = (assign9020_e4528 + assign9020_e4531);
        var_uc_js0d = assign9020_e4532;
        var_uc_js0d_rv = 0.0;

        let assign9030_e4536: f64 = (p.p825 / var_lbin);
        let assign9030_e4537: f64 = (p.p494 + assign9030_e4536);
        let assign9030_e4540: f64 = (p.p840 / var_wbin);
        let assign9030_e4541: f64 = (assign9030_e4537 + assign9030_e4540);
        let assign9030_e4544: f64 = (p.p855 / var_lwbin);
        let assign9030_e4545: f64 = (assign9030_e4541 + assign9030_e4544);
        var_uc_js0swd = assign9030_e4545;
        var_uc_js0swd_rv = 0.0;

        let assign9040_e4549: f64 = (p.p826 / var_lbin);
        let assign9040_e4550: f64 = (p.p496 + assign9040_e4549);
        let assign9040_e4553: f64 = (p.p841 / var_wbin);
        let assign9040_e4554: f64 = (assign9040_e4550 + assign9040_e4553);
        let assign9040_e4557: f64 = (p.p856 / var_lwbin);
        let assign9040_e4558: f64 = (assign9040_e4554 + assign9040_e4557);
        var_uc_njd = assign9040_e4558;
        var_uc_njd_rv = 0.0;

        let assign9060_e4575: f64 = (p.p828 / var_lbin);
        let assign9060_e4576: f64 = (p.p515 + assign9060_e4575);
        let assign9060_e4579: f64 = (p.p843 / var_wbin);
        let assign9060_e4580: f64 = (assign9060_e4576 + assign9060_e4579);
        let assign9060_e4583: f64 = (p.p858 / var_lwbin);
        let assign9060_e4584: f64 = (assign9060_e4580 + assign9060_e4583);
        var_uc_vdiffjd = assign9060_e4584;
        var_uc_vdiffjd_rv = 0.0;

        let assign9070_e4588: f64 = (p.p829 / var_lbin);
        let assign9070_e4589: f64 = (p.p516 + assign9070_e4588);
        let assign9070_e4592: f64 = (p.p844 / var_wbin);
        let assign9070_e4593: f64 = (assign9070_e4589 + assign9070_e4592);
        let assign9070_e4596: f64 = (p.p859 / var_lwbin);
        let assign9070_e4597: f64 = (assign9070_e4593 + assign9070_e4596);
        var_uc_js0s = assign9070_e4597;
        var_uc_js0s_rv = 0.0;

        *var_uc_cgbo_slot = var_uc_cgbo;
        *var_uc_cgbo_rv_slot = var_uc_cgbo_rv;
        *var_uc_cgdo_slot = var_uc_cgdo;
        *var_uc_cgdo_rv_slot = var_uc_cgdo_rv;
        *var_uc_cgso_slot = var_uc_cgso;
        *var_uc_cgso_rv_slot = var_uc_cgso_rv;
        *var_uc_clm1_slot = var_uc_clm1;
        *var_uc_clm1_rv_slot = var_uc_clm1_rv;
        *var_uc_clm2_slot = var_uc_clm2;
        *var_uc_clm2_dn0_slot = var_uc_clm2_dn0;
        *var_uc_clm2_dn10_slot = var_uc_clm2_dn10;
        *var_uc_clm2_dn11_slot = var_uc_clm2_dn11;
        *var_uc_clm2_dn14_slot = var_uc_clm2_dn14;
        *var_uc_clm2_dn2_slot = var_uc_clm2_dn2;
        *var_uc_clm2_dn4_slot = var_uc_clm2_dn4;
        *var_uc_clm2_dn5_slot = var_uc_clm2_dn5;
        *var_uc_clm2_dn6_slot = var_uc_clm2_dn6;
        *var_uc_clm2_dn7_slot = var_uc_clm2_dn7;
        *var_uc_clm2_dn8_slot = var_uc_clm2_dn8;
        *var_uc_clm2_dn9_slot = var_uc_clm2_dn9;
        *var_uc_clm2_rv_slot = var_uc_clm2_rv;
        *var_uc_clm3_slot = var_uc_clm3;
        *var_uc_clm3_rv_slot = var_uc_clm3_rv;
        *var_uc_cvdsover_slot = var_uc_cvdsover;
        *var_uc_cvdsover_rv_slot = var_uc_cvdsover_rv;
        *var_uc_fn3_slot = var_uc_fn3;
        *var_uc_fn3_rv_slot = var_uc_fn3_rv;
        *var_uc_fvbs_slot = var_uc_fvbs;
        *var_uc_fvbs_rv_slot = var_uc_fvbs_rv;
        *var_uc_gidl1_slot = var_uc_gidl1;
        *var_uc_gidl1_rv_slot = var_uc_gidl1_rv;
        *var_uc_gidl2_slot = var_uc_gidl2;
        *var_uc_gidl2_rv_slot = var_uc_gidl2_rv;
        *var_uc_gleak1_slot = var_uc_gleak1;
        *var_uc_gleak1_rv_slot = var_uc_gleak1_rv;
        *var_uc_gleak2_slot = var_uc_gleak2;
        *var_uc_gleak2_rv_slot = var_uc_gleak2_rv;
        *var_uc_gleak3_slot = var_uc_gleak3;
        *var_uc_gleak3_rv_slot = var_uc_gleak3_rv;
        *var_uc_gleak6_slot = var_uc_gleak6;
        *var_uc_gleak6_rv_slot = var_uc_gleak6_rv;
        *var_uc_glkb1_slot = var_uc_glkb1;
        *var_uc_glkb1_rv_slot = var_uc_glkb1_rv;
        *var_uc_glkb2_slot = var_uc_glkb2;
        *var_uc_glkb2_rv_slot = var_uc_glkb2_rv;
        *var_uc_glksd1_slot = var_uc_glksd1;
        *var_uc_glksd1_rv_slot = var_uc_glksd1_rv;
        *var_uc_glksd2_slot = var_uc_glksd2;
        *var_uc_glksd2_rv_slot = var_uc_glksd2_rv;
        *var_uc_ibpc1_slot = var_uc_ibpc1;
        *var_uc_ibpc1_rv_slot = var_uc_ibpc1_rv;
        *var_uc_ibpc2_slot = var_uc_ibpc2;
        *var_uc_ibpc2_rv_slot = var_uc_ibpc2_rv;
        *var_uc_js0d_slot = var_uc_js0d;
        *var_uc_js0d_rv_slot = var_uc_js0d_rv;
        *var_uc_js0s_slot = var_uc_js0s;
        *var_uc_js0s_rv_slot = var_uc_js0s_rv;
        *var_uc_js0swd_slot = var_uc_js0swd;
        *var_uc_js0swd_rv_slot = var_uc_js0swd_rv;
        *var_uc_muesti1_slot = var_uc_muesti1;
        *var_uc_muesti1_rv_slot = var_uc_muesti1_rv;
        *var_uc_muesti2_slot = var_uc_muesti2;
        *var_uc_muesti2_rv_slot = var_uc_muesti2_rv;
        *var_uc_muesti3_slot = var_uc_muesti3;
        *var_uc_muesti3_rv_slot = var_uc_muesti3_rv;
        *var_uc_nfalp_slot = var_uc_nfalp;
        *var_uc_nfalp_rv_slot = var_uc_nfalp_rv;
        *var_uc_njd_slot = var_uc_njd;
        *var_uc_njd_rv_slot = var_uc_njd_rv;
        *var_uc_npext_slot = var_uc_npext;
        *var_uc_npext_rv_slot = var_uc_npext_rv;
        *var_uc_nsti_slot = var_uc_nsti;
        *var_uc_nsti_rv_slot = var_uc_nsti_rv;
        *var_uc_nsubpsti1_slot = var_uc_nsubpsti1;
        *var_uc_nsubpsti1_rv_slot = var_uc_nsubpsti1_rv;
        *var_uc_nsubpsti2_slot = var_uc_nsubpsti2;
        *var_uc_nsubpsti2_rv_slot = var_uc_nsubpsti2_rv;
        *var_uc_nsubpsti3_slot = var_uc_nsubpsti3;
        *var_uc_nsubpsti3_rv_slot = var_uc_nsubpsti3_rv;
        *var_uc_powrat_slot = var_uc_powrat;
        *var_uc_powrat_rv_slot = var_uc_powrat_rv;
        *var_uc_rd_slot = var_uc_rd;
        *var_uc_rd22_slot = var_uc_rd22;
        *var_uc_rd22_rv_slot = var_uc_rd22_rv;
        *var_uc_rd23_slot = var_uc_rd23;
        *var_uc_rd23_rv_slot = var_uc_rd23_rv;
        *var_uc_rd24_slot = var_uc_rd24;
        *var_uc_rd24_rv_slot = var_uc_rd24_rv;
        *var_uc_rd_rv_slot = var_uc_rd_rv;
        *var_uc_rdict1_slot = var_uc_rdict1;
        *var_uc_rdict1_rv_slot = var_uc_rdict1_rv;
        *var_uc_rdov13_slot = var_uc_rdov13;
        *var_uc_rdov13_rv_slot = var_uc_rdov13_rv;
        *var_uc_rdslp1_slot = var_uc_rdslp1;
        *var_uc_rdslp1_rv_slot = var_uc_rdslp1_rv;
        *var_uc_rdvb_slot = var_uc_rdvb;
        *var_uc_rdvb_rv_slot = var_uc_rdvb_rv;
        *var_uc_rdvd_slot = var_uc_rdvd;
        *var_uc_rdvd_rv_slot = var_uc_rdvd_rv;
        *var_uc_rdvg11_slot = var_uc_rdvg11;
        *var_uc_rdvg11_rv_slot = var_uc_rdvg11_rv;
        *var_uc_rs_slot = var_uc_rs;
        *var_uc_rs_rv_slot = var_uc_rs_rv;
        *var_uc_rth0_slot = var_uc_rth0;
        *var_uc_rth0_rv_slot = var_uc_rth0_rv;
        *var_uc_scsti1_slot = var_uc_scsti1;
        *var_uc_scsti1_rv_slot = var_uc_scsti1_rv;
        *var_uc_scsti2_slot = var_uc_scsti2;
        *var_uc_scsti2_rv_slot = var_uc_scsti2_rv;
        *var_uc_vdiffjd_slot = var_uc_vdiffjd;
        *var_uc_vdiffjd_rv_slot = var_uc_vdiffjd_rv;
        *var_uc_vover_slot = var_uc_vover;
        *var_uc_vover_rv_slot = var_uc_vover_rv;
        *var_uc_vthsti_slot = var_uc_vthsti;
        *var_uc_vthsti_rv_slot = var_uc_vthsti_rv;
        *var_uc_wfc_slot = var_uc_wfc;
        *var_uc_wfc_rv_slot = var_uc_wfc_rv;
        *var_uc_wsti_slot = var_uc_wsti;
        *var_uc_wsti_dn0_slot = var_uc_wsti_dn0;
        *var_uc_wsti_dn10_slot = var_uc_wsti_dn10;
        *var_uc_wsti_dn11_slot = var_uc_wsti_dn11;
        *var_uc_wsti_dn14_slot = var_uc_wsti_dn14;
        *var_uc_wsti_dn2_slot = var_uc_wsti_dn2;
        *var_uc_wsti_dn4_slot = var_uc_wsti_dn4;
        *var_uc_wsti_dn5_slot = var_uc_wsti_dn5;
        *var_uc_wsti_dn6_slot = var_uc_wsti_dn6;
        *var_uc_wsti_dn7_slot = var_uc_wsti_dn7;
        *var_uc_wsti_dn8_slot = var_uc_wsti_dn8;
        *var_uc_wsti_dn9_slot = var_uc_wsti_dn9;
        *var_uc_wsti_rv_slot = var_uc_wsti_rv;
    }

    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        var_lbin: f64,
        var_lg: f64,
        var_lwbin: f64,
        var_uc_codep: f64,
        var_wbin: f64,
        var_guard187_slot: &mut f64,
        var_guard187_rv_slot: &mut f64,
        var_guard188_slot: &mut f64,
        var_guard188_rv_slot: &mut f64,
        var_guard189_slot: &mut f64,
        var_guard189_rv_slot: &mut f64,
        var_guard190_slot: &mut f64,
        var_guard190_rv_slot: &mut f64,
        var_guard191_slot: &mut f64,
        var_guard191_rv_slot: &mut f64,
        var_guard192_slot: &mut f64,
        var_guard192_rv_slot: &mut f64,
        var_guard193_slot: &mut f64,
        var_guard193_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn14_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_uc_depleak_slot: &mut f64,
        var_uc_depleak_dn0_slot: &mut f64,
        var_uc_depleak_dn10_slot: &mut f64,
        var_uc_depleak_dn11_slot: &mut f64,
        var_uc_depleak_dn14_slot: &mut f64,
        var_uc_depleak_dn2_slot: &mut f64,
        var_uc_depleak_dn4_slot: &mut f64,
        var_uc_depleak_dn5_slot: &mut f64,
        var_uc_depleak_dn6_slot: &mut f64,
        var_uc_depleak_dn7_slot: &mut f64,
        var_uc_depleak_dn8_slot: &mut f64,
        var_uc_depleak_dn9_slot: &mut f64,
        var_uc_depleak_rv_slot: &mut f64,
        var_uc_depmue0_slot: &mut f64,
        var_uc_depmue0_dn0_slot: &mut f64,
        var_uc_depmue0_dn10_slot: &mut f64,
        var_uc_depmue0_dn11_slot: &mut f64,
        var_uc_depmue0_dn14_slot: &mut f64,
        var_uc_depmue0_dn2_slot: &mut f64,
        var_uc_depmue0_dn4_slot: &mut f64,
        var_uc_depmue0_dn5_slot: &mut f64,
        var_uc_depmue0_dn6_slot: &mut f64,
        var_uc_depmue0_dn7_slot: &mut f64,
        var_uc_depmue0_dn8_slot: &mut f64,
        var_uc_depmue0_dn9_slot: &mut f64,
        var_uc_depmue0_rv_slot: &mut f64,
        var_uc_depmue1_slot: &mut f64,
        var_uc_depmue1_dn0_slot: &mut f64,
        var_uc_depmue1_dn10_slot: &mut f64,
        var_uc_depmue1_dn11_slot: &mut f64,
        var_uc_depmue1_dn14_slot: &mut f64,
        var_uc_depmue1_dn2_slot: &mut f64,
        var_uc_depmue1_dn4_slot: &mut f64,
        var_uc_depmue1_dn5_slot: &mut f64,
        var_uc_depmue1_dn6_slot: &mut f64,
        var_uc_depmue1_dn7_slot: &mut f64,
        var_uc_depmue1_dn8_slot: &mut f64,
        var_uc_depmue1_dn9_slot: &mut f64,
        var_uc_depmue1_rv_slot: &mut f64,
        var_uc_depmueback0_slot: &mut f64,
        var_uc_depmueback0_dn0_slot: &mut f64,
        var_uc_depmueback0_dn10_slot: &mut f64,
        var_uc_depmueback0_dn11_slot: &mut f64,
        var_uc_depmueback0_dn14_slot: &mut f64,
        var_uc_depmueback0_dn2_slot: &mut f64,
        var_uc_depmueback0_dn4_slot: &mut f64,
        var_uc_depmueback0_dn5_slot: &mut f64,
        var_uc_depmueback0_dn6_slot: &mut f64,
        var_uc_depmueback0_dn7_slot: &mut f64,
        var_uc_depmueback0_dn8_slot: &mut f64,
        var_uc_depmueback0_dn9_slot: &mut f64,
        var_uc_depmueback0_rv_slot: &mut f64,
        var_uc_depmueback1_slot: &mut f64,
        var_uc_depmueback1_dn0_slot: &mut f64,
        var_uc_depmueback1_dn10_slot: &mut f64,
        var_uc_depmueback1_dn11_slot: &mut f64,
        var_uc_depmueback1_dn14_slot: &mut f64,
        var_uc_depmueback1_dn2_slot: &mut f64,
        var_uc_depmueback1_dn4_slot: &mut f64,
        var_uc_depmueback1_dn5_slot: &mut f64,
        var_uc_depmueback1_dn6_slot: &mut f64,
        var_uc_depmueback1_dn7_slot: &mut f64,
        var_uc_depmueback1_dn8_slot: &mut f64,
        var_uc_depmueback1_dn9_slot: &mut f64,
        var_uc_depmueback1_rv_slot: &mut f64,
        var_uc_depvmax_slot: &mut f64,
        var_uc_depvmax_dn0_slot: &mut f64,
        var_uc_depvmax_dn10_slot: &mut f64,
        var_uc_depvmax_dn11_slot: &mut f64,
        var_uc_depvmax_dn14_slot: &mut f64,
        var_uc_depvmax_dn2_slot: &mut f64,
        var_uc_depvmax_dn4_slot: &mut f64,
        var_uc_depvmax_dn5_slot: &mut f64,
        var_uc_depvmax_dn6_slot: &mut f64,
        var_uc_depvmax_dn7_slot: &mut f64,
        var_uc_depvmax_dn8_slot: &mut f64,
        var_uc_depvmax_dn9_slot: &mut f64,
        var_uc_depvmax_rv_slot: &mut f64,
        var_uc_js0sws_slot: &mut f64,
        var_uc_js0sws_rv_slot: &mut f64,
        var_uc_ndepm_slot: &mut f64,
        var_uc_ndepm_dn0_slot: &mut f64,
        var_uc_ndepm_dn10_slot: &mut f64,
        var_uc_ndepm_dn11_slot: &mut f64,
        var_uc_ndepm_dn14_slot: &mut f64,
        var_uc_ndepm_dn2_slot: &mut f64,
        var_uc_ndepm_dn4_slot: &mut f64,
        var_uc_ndepm_dn5_slot: &mut f64,
        var_uc_ndepm_dn6_slot: &mut f64,
        var_uc_ndepm_dn7_slot: &mut f64,
        var_uc_ndepm_dn8_slot: &mut f64,
        var_uc_ndepm_dn9_slot: &mut f64,
        var_uc_ndepm_rv_slot: &mut f64,
        var_uc_njs_slot: &mut f64,
        var_uc_njs_rv_slot: &mut f64,
        var_uc_vdiffjs_slot: &mut f64,
        var_uc_vdiffjs_rv_slot: &mut f64,
    ) {
        let mut var_guard187: f64 = *var_guard187_slot;
        let mut var_guard187_rv: f64 = *var_guard187_rv_slot;
        let mut var_guard188: f64 = *var_guard188_slot;
        let mut var_guard188_rv: f64 = *var_guard188_rv_slot;
        let mut var_guard189: f64 = *var_guard189_slot;
        let mut var_guard189_rv: f64 = *var_guard189_rv_slot;
        let mut var_guard190: f64 = *var_guard190_slot;
        let mut var_guard190_rv: f64 = *var_guard190_rv_slot;
        let mut var_guard191: f64 = *var_guard191_slot;
        let mut var_guard191_rv: f64 = *var_guard191_rv_slot;
        let mut var_guard192: f64 = *var_guard192_slot;
        let mut var_guard192_rv: f64 = *var_guard192_rv_slot;
        let mut var_guard193: f64 = *var_guard193_slot;
        let mut var_guard193_rv: f64 = *var_guard193_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn14: f64 = *var_t3_dn14_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_uc_depleak: f64 = *var_uc_depleak_slot;
        let mut var_uc_depleak_dn0: f64 = *var_uc_depleak_dn0_slot;
        let mut var_uc_depleak_dn10: f64 = *var_uc_depleak_dn10_slot;
        let mut var_uc_depleak_dn11: f64 = *var_uc_depleak_dn11_slot;
        let mut var_uc_depleak_dn14: f64 = *var_uc_depleak_dn14_slot;
        let mut var_uc_depleak_dn2: f64 = *var_uc_depleak_dn2_slot;
        let mut var_uc_depleak_dn4: f64 = *var_uc_depleak_dn4_slot;
        let mut var_uc_depleak_dn5: f64 = *var_uc_depleak_dn5_slot;
        let mut var_uc_depleak_dn6: f64 = *var_uc_depleak_dn6_slot;
        let mut var_uc_depleak_dn7: f64 = *var_uc_depleak_dn7_slot;
        let mut var_uc_depleak_dn8: f64 = *var_uc_depleak_dn8_slot;
        let mut var_uc_depleak_dn9: f64 = *var_uc_depleak_dn9_slot;
        let mut var_uc_depleak_rv: f64 = *var_uc_depleak_rv_slot;
        let mut var_uc_depmue0: f64 = *var_uc_depmue0_slot;
        let mut var_uc_depmue0_dn0: f64 = *var_uc_depmue0_dn0_slot;
        let mut var_uc_depmue0_dn10: f64 = *var_uc_depmue0_dn10_slot;
        let mut var_uc_depmue0_dn11: f64 = *var_uc_depmue0_dn11_slot;
        let mut var_uc_depmue0_dn14: f64 = *var_uc_depmue0_dn14_slot;
        let mut var_uc_depmue0_dn2: f64 = *var_uc_depmue0_dn2_slot;
        let mut var_uc_depmue0_dn4: f64 = *var_uc_depmue0_dn4_slot;
        let mut var_uc_depmue0_dn5: f64 = *var_uc_depmue0_dn5_slot;
        let mut var_uc_depmue0_dn6: f64 = *var_uc_depmue0_dn6_slot;
        let mut var_uc_depmue0_dn7: f64 = *var_uc_depmue0_dn7_slot;
        let mut var_uc_depmue0_dn8: f64 = *var_uc_depmue0_dn8_slot;
        let mut var_uc_depmue0_dn9: f64 = *var_uc_depmue0_dn9_slot;
        let mut var_uc_depmue0_rv: f64 = *var_uc_depmue0_rv_slot;
        let mut var_uc_depmue1: f64 = *var_uc_depmue1_slot;
        let mut var_uc_depmue1_dn0: f64 = *var_uc_depmue1_dn0_slot;
        let mut var_uc_depmue1_dn10: f64 = *var_uc_depmue1_dn10_slot;
        let mut var_uc_depmue1_dn11: f64 = *var_uc_depmue1_dn11_slot;
        let mut var_uc_depmue1_dn14: f64 = *var_uc_depmue1_dn14_slot;
        let mut var_uc_depmue1_dn2: f64 = *var_uc_depmue1_dn2_slot;
        let mut var_uc_depmue1_dn4: f64 = *var_uc_depmue1_dn4_slot;
        let mut var_uc_depmue1_dn5: f64 = *var_uc_depmue1_dn5_slot;
        let mut var_uc_depmue1_dn6: f64 = *var_uc_depmue1_dn6_slot;
        let mut var_uc_depmue1_dn7: f64 = *var_uc_depmue1_dn7_slot;
        let mut var_uc_depmue1_dn8: f64 = *var_uc_depmue1_dn8_slot;
        let mut var_uc_depmue1_dn9: f64 = *var_uc_depmue1_dn9_slot;
        let mut var_uc_depmue1_rv: f64 = *var_uc_depmue1_rv_slot;
        let mut var_uc_depmueback0: f64 = *var_uc_depmueback0_slot;
        let mut var_uc_depmueback0_dn0: f64 = *var_uc_depmueback0_dn0_slot;
        let mut var_uc_depmueback0_dn10: f64 = *var_uc_depmueback0_dn10_slot;
        let mut var_uc_depmueback0_dn11: f64 = *var_uc_depmueback0_dn11_slot;
        let mut var_uc_depmueback0_dn14: f64 = *var_uc_depmueback0_dn14_slot;
        let mut var_uc_depmueback0_dn2: f64 = *var_uc_depmueback0_dn2_slot;
        let mut var_uc_depmueback0_dn4: f64 = *var_uc_depmueback0_dn4_slot;
        let mut var_uc_depmueback0_dn5: f64 = *var_uc_depmueback0_dn5_slot;
        let mut var_uc_depmueback0_dn6: f64 = *var_uc_depmueback0_dn6_slot;
        let mut var_uc_depmueback0_dn7: f64 = *var_uc_depmueback0_dn7_slot;
        let mut var_uc_depmueback0_dn8: f64 = *var_uc_depmueback0_dn8_slot;
        let mut var_uc_depmueback0_dn9: f64 = *var_uc_depmueback0_dn9_slot;
        let mut var_uc_depmueback0_rv: f64 = *var_uc_depmueback0_rv_slot;
        let mut var_uc_depmueback1: f64 = *var_uc_depmueback1_slot;
        let mut var_uc_depmueback1_dn0: f64 = *var_uc_depmueback1_dn0_slot;
        let mut var_uc_depmueback1_dn10: f64 = *var_uc_depmueback1_dn10_slot;
        let mut var_uc_depmueback1_dn11: f64 = *var_uc_depmueback1_dn11_slot;
        let mut var_uc_depmueback1_dn14: f64 = *var_uc_depmueback1_dn14_slot;
        let mut var_uc_depmueback1_dn2: f64 = *var_uc_depmueback1_dn2_slot;
        let mut var_uc_depmueback1_dn4: f64 = *var_uc_depmueback1_dn4_slot;
        let mut var_uc_depmueback1_dn5: f64 = *var_uc_depmueback1_dn5_slot;
        let mut var_uc_depmueback1_dn6: f64 = *var_uc_depmueback1_dn6_slot;
        let mut var_uc_depmueback1_dn7: f64 = *var_uc_depmueback1_dn7_slot;
        let mut var_uc_depmueback1_dn8: f64 = *var_uc_depmueback1_dn8_slot;
        let mut var_uc_depmueback1_dn9: f64 = *var_uc_depmueback1_dn9_slot;
        let mut var_uc_depmueback1_rv: f64 = *var_uc_depmueback1_rv_slot;
        let mut var_uc_depvmax: f64 = *var_uc_depvmax_slot;
        let mut var_uc_depvmax_dn0: f64 = *var_uc_depvmax_dn0_slot;
        let mut var_uc_depvmax_dn10: f64 = *var_uc_depvmax_dn10_slot;
        let mut var_uc_depvmax_dn11: f64 = *var_uc_depvmax_dn11_slot;
        let mut var_uc_depvmax_dn14: f64 = *var_uc_depvmax_dn14_slot;
        let mut var_uc_depvmax_dn2: f64 = *var_uc_depvmax_dn2_slot;
        let mut var_uc_depvmax_dn4: f64 = *var_uc_depvmax_dn4_slot;
        let mut var_uc_depvmax_dn5: f64 = *var_uc_depvmax_dn5_slot;
        let mut var_uc_depvmax_dn6: f64 = *var_uc_depvmax_dn6_slot;
        let mut var_uc_depvmax_dn7: f64 = *var_uc_depvmax_dn7_slot;
        let mut var_uc_depvmax_dn8: f64 = *var_uc_depvmax_dn8_slot;
        let mut var_uc_depvmax_dn9: f64 = *var_uc_depvmax_dn9_slot;
        let mut var_uc_depvmax_rv: f64 = *var_uc_depvmax_rv_slot;
        let mut var_uc_js0sws: f64 = *var_uc_js0sws_slot;
        let mut var_uc_js0sws_rv: f64 = *var_uc_js0sws_rv_slot;
        let mut var_uc_ndepm: f64 = *var_uc_ndepm_slot;
        let mut var_uc_ndepm_dn0: f64 = *var_uc_ndepm_dn0_slot;
        let mut var_uc_ndepm_dn10: f64 = *var_uc_ndepm_dn10_slot;
        let mut var_uc_ndepm_dn11: f64 = *var_uc_ndepm_dn11_slot;
        let mut var_uc_ndepm_dn14: f64 = *var_uc_ndepm_dn14_slot;
        let mut var_uc_ndepm_dn2: f64 = *var_uc_ndepm_dn2_slot;
        let mut var_uc_ndepm_dn4: f64 = *var_uc_ndepm_dn4_slot;
        let mut var_uc_ndepm_dn5: f64 = *var_uc_ndepm_dn5_slot;
        let mut var_uc_ndepm_dn6: f64 = *var_uc_ndepm_dn6_slot;
        let mut var_uc_ndepm_dn7: f64 = *var_uc_ndepm_dn7_slot;
        let mut var_uc_ndepm_dn8: f64 = *var_uc_ndepm_dn8_slot;
        let mut var_uc_ndepm_dn9: f64 = *var_uc_ndepm_dn9_slot;
        let mut var_uc_ndepm_rv: f64 = *var_uc_ndepm_rv_slot;
        let mut var_uc_njs: f64 = *var_uc_njs_slot;
        let mut var_uc_njs_rv: f64 = *var_uc_njs_rv_slot;
        let mut var_uc_vdiffjs: f64 = *var_uc_vdiffjs_slot;
        let mut var_uc_vdiffjs_rv: f64 = *var_uc_vdiffjs_rv_slot;

        let assign9080_e4601: f64 = (p.p830 / var_lbin);
        let assign9080_e4602: f64 = (p.p517 + assign9080_e4601);
        let assign9080_e4605: f64 = (p.p845 / var_wbin);
        let assign9080_e4606: f64 = (assign9080_e4602 + assign9080_e4605);
        let assign9080_e4609: f64 = (p.p860 / var_lwbin);
        let assign9080_e4610: f64 = (assign9080_e4606 + assign9080_e4609);
        var_uc_js0sws = assign9080_e4610;
        var_uc_js0sws_rv = 0.0;

        let assign9090_e4614: f64 = (p.p831 / var_lbin);
        let assign9090_e4615: f64 = (p.p519 + assign9090_e4614);
        let assign9090_e4618: f64 = (p.p846 / var_wbin);
        let assign9090_e4619: f64 = (assign9090_e4615 + assign9090_e4618);
        let assign9090_e4622: f64 = (p.p861 / var_lwbin);
        let assign9090_e4623: f64 = (assign9090_e4619 + assign9090_e4622);
        var_uc_njs = assign9090_e4623;
        var_uc_njs_rv = 0.0;

        let assign9110_e4640: f64 = (p.p833 / var_lbin);
        let assign9110_e4641: f64 = (p.p538 + assign9110_e4640);
        let assign9110_e4644: f64 = (p.p848 / var_wbin);
        let assign9110_e4645: f64 = (assign9110_e4641 + assign9110_e4644);
        let assign9110_e4648: f64 = (p.p863 / var_lwbin);
        let assign9110_e4649: f64 = (assign9110_e4645 + assign9110_e4648);
        var_uc_vdiffjs = assign9110_e4649;
        var_uc_vdiffjs_rv = 0.0;

        let assign9210_e4700: f64 = if var_uc_codep != 0.0 { 1.0 } else { 0.0 };
        var_guard187 = assign9210_e4700;
        var_guard187_rv = 0.0;

        let (assign9220_e4706, assign9220_e4706_d_n0, assign9220_e4706_d_n2, assign9220_e4706_d_n4, assign9220_e4706_d_n5, assign9220_e4706_d_n6, assign9220_e4706_d_n7, assign9220_e4706_d_n8, assign9220_e4706_d_n9, assign9220_e4706_d_n10, assign9220_e4706_d_n11, assign9220_e4706_d_n14,) = {
    if (var_guard187 != 0.0) {
        let assign9220_e4704: f64 = (var_lg).powf(p.p342);
        (assign9220_e4704, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn14,)
    }
};
        var_t3 = assign9220_e4706;
        var_t3_dn0 = assign9220_e4706_d_n0;
        var_t3_dn2 = assign9220_e4706_d_n2;
        var_t3_dn4 = assign9220_e4706_d_n4;
        var_t3_dn5 = assign9220_e4706_d_n5;
        var_t3_dn6 = assign9220_e4706_d_n6;
        var_t3_dn7 = assign9220_e4706_d_n7;
        var_t3_dn8 = assign9220_e4706_d_n8;
        var_t3_dn9 = assign9220_e4706_d_n9;
        var_t3_dn10 = assign9220_e4706_d_n10;
        var_t3_dn11 = assign9220_e4706_d_n11;
        var_t3_dn14 = assign9220_e4706_d_n14;
        var_t3_rv = 0.0;

        let (assign9230_e4716, assign9230_e4716_d_n0, assign9230_e4716_d_n2, assign9230_e4716_d_n4, assign9230_e4716_d_n5, assign9230_e4716_d_n6, assign9230_e4716_d_n7, assign9230_e4716_d_n8, assign9230_e4716_d_n9, assign9230_e4716_d_n10, assign9230_e4716_d_n11, assign9230_e4716_d_n14,) = {
    if (var_guard187 != 0.0) {
        let assign9230_e4712: f64 = (p.p341 / var_t3);
        let assign9230_e4713: f64 = (1.0 + assign9230_e4712);
        let assign9230_e4714: f64 = (var_uc_ndepm * assign9230_e4713);
        (assign9230_e4714, ((var_uc_ndepm_dn0 * assign9230_e4713) + (var_uc_ndepm * (-((p.p341 * var_t3_dn0) / (var_t3 * var_t3))))), ((var_uc_ndepm_dn2 * assign9230_e4713) + (var_uc_ndepm * (-((p.p341 * var_t3_dn2) / (var_t3 * var_t3))))), ((var_uc_ndepm_dn4 * assign9230_e4713) + (var_uc_ndepm * (-((p.p341 * var_t3_dn4) / (var_t3 * var_t3))))), ((var_uc_ndepm_dn5 * assign9230_e4713) + (var_uc_ndepm * (-((p.p341 * var_t3_dn5) / (var_t3 * var_t3))))), ((var_uc_ndepm_dn6 * assign9230_e4713) + (var_uc_ndepm * (-((p.p341 * var_t3_dn6) / (var_t3 * var_t3))))), ((var_uc_ndepm_dn7 * assign9230_e4713) + (var_uc_ndepm * (-((p.p341 * var_t3_dn7) / (var_t3 * var_t3))))), ((var_uc_ndepm_dn8 * assign9230_e4713) + (var_uc_ndepm * (-((p.p341 * var_t3_dn8) / (var_t3 * var_t3))))), ((var_uc_ndepm_dn9 * assign9230_e4713) + (var_uc_ndepm * (-((p.p341 * var_t3_dn9) / (var_t3 * var_t3))))), ((var_uc_ndepm_dn10 * assign9230_e4713) + (var_uc_ndepm * (-((p.p341 * var_t3_dn10) / (var_t3 * var_t3))))), ((var_uc_ndepm_dn11 * assign9230_e4713) + (var_uc_ndepm * (-((p.p341 * var_t3_dn11) / (var_t3 * var_t3))))), ((var_uc_ndepm_dn14 * assign9230_e4713) + (var_uc_ndepm * (-((p.p341 * var_t3_dn14) / (var_t3 * var_t3))))),)
    } else {
        (var_uc_ndepm, var_uc_ndepm_dn0, var_uc_ndepm_dn2, var_uc_ndepm_dn4, var_uc_ndepm_dn5, var_uc_ndepm_dn6, var_uc_ndepm_dn7, var_uc_ndepm_dn8, var_uc_ndepm_dn9, var_uc_ndepm_dn10, var_uc_ndepm_dn11, var_uc_ndepm_dn14,)
    }
};
        var_uc_ndepm = assign9230_e4716;
        var_uc_ndepm_dn0 = assign9230_e4716_d_n0;
        var_uc_ndepm_dn2 = assign9230_e4716_d_n2;
        var_uc_ndepm_dn4 = assign9230_e4716_d_n4;
        var_uc_ndepm_dn5 = assign9230_e4716_d_n5;
        var_uc_ndepm_dn6 = assign9230_e4716_d_n6;
        var_uc_ndepm_dn7 = assign9230_e4716_d_n7;
        var_uc_ndepm_dn8 = assign9230_e4716_d_n8;
        var_uc_ndepm_dn9 = assign9230_e4716_d_n9;
        var_uc_ndepm_dn10 = assign9230_e4716_d_n10;
        var_uc_ndepm_dn11 = assign9230_e4716_d_n11;
        var_uc_ndepm_dn14 = assign9230_e4716_d_n14;
        var_uc_ndepm_rv = 0.0;

        let assign9240_e4719: f64 = if var_uc_ndepm < 1e21 { 1.0 } else { 0.0 };
        var_guard188 = assign9240_e4719;
        var_guard188_rv = 0.0;

        let (assign9250_e4725, assign9250_e4725_d_n0, assign9250_e4725_d_n2, assign9250_e4725_d_n4, assign9250_e4725_d_n5, assign9250_e4725_d_n6, assign9250_e4725_d_n7, assign9250_e4725_d_n8, assign9250_e4725_d_n9, assign9250_e4725_d_n10, assign9250_e4725_d_n11, assign9250_e4725_d_n14,) = {
    if ((var_guard187 != 0.0) && (var_guard188 != 0.0)) {
        (1e21, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_ndepm, var_uc_ndepm_dn0, var_uc_ndepm_dn2, var_uc_ndepm_dn4, var_uc_ndepm_dn5, var_uc_ndepm_dn6, var_uc_ndepm_dn7, var_uc_ndepm_dn8, var_uc_ndepm_dn9, var_uc_ndepm_dn10, var_uc_ndepm_dn11, var_uc_ndepm_dn14,)
    }
};
        var_uc_ndepm = assign9250_e4725;
        var_uc_ndepm_dn0 = assign9250_e4725_d_n0;
        var_uc_ndepm_dn2 = assign9250_e4725_d_n2;
        var_uc_ndepm_dn4 = assign9250_e4725_d_n4;
        var_uc_ndepm_dn5 = assign9250_e4725_d_n5;
        var_uc_ndepm_dn6 = assign9250_e4725_d_n6;
        var_uc_ndepm_dn7 = assign9250_e4725_d_n7;
        var_uc_ndepm_dn8 = assign9250_e4725_d_n8;
        var_uc_ndepm_dn9 = assign9250_e4725_d_n9;
        var_uc_ndepm_dn10 = assign9250_e4725_d_n10;
        var_uc_ndepm_dn11 = assign9250_e4725_d_n11;
        var_uc_ndepm_dn14 = assign9250_e4725_d_n14;
        var_uc_ndepm_rv = 0.0;

        let (assign9260_e4731, assign9260_e4731_d_n0, assign9260_e4731_d_n2, assign9260_e4731_d_n4, assign9260_e4731_d_n5, assign9260_e4731_d_n6, assign9260_e4731_d_n7, assign9260_e4731_d_n8, assign9260_e4731_d_n9, assign9260_e4731_d_n10, assign9260_e4731_d_n11, assign9260_e4731_d_n14,) = {
    if (var_guard187 != 0.0) {
        let assign9260_e4729: f64 = (var_lg).powf(p.p369);
        (assign9260_e4729, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn14,)
    }
};
        var_t3 = assign9260_e4731;
        var_t3_dn0 = assign9260_e4731_d_n0;
        var_t3_dn2 = assign9260_e4731_d_n2;
        var_t3_dn4 = assign9260_e4731_d_n4;
        var_t3_dn5 = assign9260_e4731_d_n5;
        var_t3_dn6 = assign9260_e4731_d_n6;
        var_t3_dn7 = assign9260_e4731_d_n7;
        var_t3_dn8 = assign9260_e4731_d_n8;
        var_t3_dn9 = assign9260_e4731_d_n9;
        var_t3_dn10 = assign9260_e4731_d_n10;
        var_t3_dn11 = assign9260_e4731_d_n11;
        var_t3_dn14 = assign9260_e4731_d_n14;
        var_t3_rv = 0.0;

        let (assign9270_e4741, assign9270_e4741_d_n0, assign9270_e4741_d_n2, assign9270_e4741_d_n4, assign9270_e4741_d_n5, assign9270_e4741_d_n6, assign9270_e4741_d_n7, assign9270_e4741_d_n8, assign9270_e4741_d_n9, assign9270_e4741_d_n10, assign9270_e4741_d_n11, assign9270_e4741_d_n14,) = {
    if (var_guard187 != 0.0) {
        let assign9270_e4737: f64 = (p.p368 / var_t3);
        let assign9270_e4738: f64 = (1.0 + assign9270_e4737);
        let assign9270_e4739: f64 = (var_uc_depvmax * assign9270_e4738);
        (assign9270_e4739, ((var_uc_depvmax_dn0 * assign9270_e4738) + (var_uc_depvmax * (-((p.p368 * var_t3_dn0) / (var_t3 * var_t3))))), ((var_uc_depvmax_dn2 * assign9270_e4738) + (var_uc_depvmax * (-((p.p368 * var_t3_dn2) / (var_t3 * var_t3))))), ((var_uc_depvmax_dn4 * assign9270_e4738) + (var_uc_depvmax * (-((p.p368 * var_t3_dn4) / (var_t3 * var_t3))))), ((var_uc_depvmax_dn5 * assign9270_e4738) + (var_uc_depvmax * (-((p.p368 * var_t3_dn5) / (var_t3 * var_t3))))), ((var_uc_depvmax_dn6 * assign9270_e4738) + (var_uc_depvmax * (-((p.p368 * var_t3_dn6) / (var_t3 * var_t3))))), ((var_uc_depvmax_dn7 * assign9270_e4738) + (var_uc_depvmax * (-((p.p368 * var_t3_dn7) / (var_t3 * var_t3))))), ((var_uc_depvmax_dn8 * assign9270_e4738) + (var_uc_depvmax * (-((p.p368 * var_t3_dn8) / (var_t3 * var_t3))))), ((var_uc_depvmax_dn9 * assign9270_e4738) + (var_uc_depvmax * (-((p.p368 * var_t3_dn9) / (var_t3 * var_t3))))), ((var_uc_depvmax_dn10 * assign9270_e4738) + (var_uc_depvmax * (-((p.p368 * var_t3_dn10) / (var_t3 * var_t3))))), ((var_uc_depvmax_dn11 * assign9270_e4738) + (var_uc_depvmax * (-((p.p368 * var_t3_dn11) / (var_t3 * var_t3))))), ((var_uc_depvmax_dn14 * assign9270_e4738) + (var_uc_depvmax * (-((p.p368 * var_t3_dn14) / (var_t3 * var_t3))))),)
    } else {
        (var_uc_depvmax, var_uc_depvmax_dn0, var_uc_depvmax_dn2, var_uc_depvmax_dn4, var_uc_depvmax_dn5, var_uc_depvmax_dn6, var_uc_depvmax_dn7, var_uc_depvmax_dn8, var_uc_depvmax_dn9, var_uc_depvmax_dn10, var_uc_depvmax_dn11, var_uc_depvmax_dn14,)
    }
};
        var_uc_depvmax = assign9270_e4741;
        var_uc_depvmax_dn0 = assign9270_e4741_d_n0;
        var_uc_depvmax_dn2 = assign9270_e4741_d_n2;
        var_uc_depvmax_dn4 = assign9270_e4741_d_n4;
        var_uc_depvmax_dn5 = assign9270_e4741_d_n5;
        var_uc_depvmax_dn6 = assign9270_e4741_d_n6;
        var_uc_depvmax_dn7 = assign9270_e4741_d_n7;
        var_uc_depvmax_dn8 = assign9270_e4741_d_n8;
        var_uc_depvmax_dn9 = assign9270_e4741_d_n9;
        var_uc_depvmax_dn10 = assign9270_e4741_d_n10;
        var_uc_depvmax_dn11 = assign9270_e4741_d_n11;
        var_uc_depvmax_dn14 = assign9270_e4741_d_n14;
        var_uc_depvmax_rv = 0.0;

        let (assign9280_e4747, assign9280_e4747_d_n0, assign9280_e4747_d_n2, assign9280_e4747_d_n4, assign9280_e4747_d_n5, assign9280_e4747_d_n6, assign9280_e4747_d_n7, assign9280_e4747_d_n8, assign9280_e4747_d_n9, assign9280_e4747_d_n10, assign9280_e4747_d_n11, assign9280_e4747_d_n14,) = {
    if (var_guard187 != 0.0) {
        let assign9280_e4745: f64 = (var_lg).powf(p.p362);
        (assign9280_e4745, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn14,)
    }
};
        var_t3 = assign9280_e4747;
        var_t3_dn0 = assign9280_e4747_d_n0;
        var_t3_dn2 = assign9280_e4747_d_n2;
        var_t3_dn4 = assign9280_e4747_d_n4;
        var_t3_dn5 = assign9280_e4747_d_n5;
        var_t3_dn6 = assign9280_e4747_d_n6;
        var_t3_dn7 = assign9280_e4747_d_n7;
        var_t3_dn8 = assign9280_e4747_d_n8;
        var_t3_dn9 = assign9280_e4747_d_n9;
        var_t3_dn10 = assign9280_e4747_d_n10;
        var_t3_dn11 = assign9280_e4747_d_n11;
        var_t3_dn14 = assign9280_e4747_d_n14;
        var_t3_rv = 0.0;

        let (assign9290_e4757, assign9290_e4757_d_n0, assign9290_e4757_d_n2, assign9290_e4757_d_n4, assign9290_e4757_d_n5, assign9290_e4757_d_n6, assign9290_e4757_d_n7, assign9290_e4757_d_n8, assign9290_e4757_d_n9, assign9290_e4757_d_n10, assign9290_e4757_d_n11, assign9290_e4757_d_n14,) = {
    if (var_guard187 != 0.0) {
        let assign9290_e4753: f64 = (p.p361 / var_t3);
        let assign9290_e4754: f64 = (1.0 + assign9290_e4753);
        let assign9290_e4755: f64 = (p.p360 * assign9290_e4754);
        (assign9290_e4755, (p.p360 * (-((p.p361 * var_t3_dn0) / (var_t3 * var_t3)))), (p.p360 * (-((p.p361 * var_t3_dn2) / (var_t3 * var_t3)))), (p.p360 * (-((p.p361 * var_t3_dn4) / (var_t3 * var_t3)))), (p.p360 * (-((p.p361 * var_t3_dn5) / (var_t3 * var_t3)))), (p.p360 * (-((p.p361 * var_t3_dn6) / (var_t3 * var_t3)))), (p.p360 * (-((p.p361 * var_t3_dn7) / (var_t3 * var_t3)))), (p.p360 * (-((p.p361 * var_t3_dn8) / (var_t3 * var_t3)))), (p.p360 * (-((p.p361 * var_t3_dn9) / (var_t3 * var_t3)))), (p.p360 * (-((p.p361 * var_t3_dn10) / (var_t3 * var_t3)))), (p.p360 * (-((p.p361 * var_t3_dn11) / (var_t3 * var_t3)))), (p.p360 * (-((p.p361 * var_t3_dn14) / (var_t3 * var_t3)))),)
    } else {
        (var_uc_depleak, var_uc_depleak_dn0, var_uc_depleak_dn2, var_uc_depleak_dn4, var_uc_depleak_dn5, var_uc_depleak_dn6, var_uc_depleak_dn7, var_uc_depleak_dn8, var_uc_depleak_dn9, var_uc_depleak_dn10, var_uc_depleak_dn11, var_uc_depleak_dn14,)
    }
};
        var_uc_depleak = assign9290_e4757;
        var_uc_depleak_dn0 = assign9290_e4757_d_n0;
        var_uc_depleak_dn2 = assign9290_e4757_d_n2;
        var_uc_depleak_dn4 = assign9290_e4757_d_n4;
        var_uc_depleak_dn5 = assign9290_e4757_d_n5;
        var_uc_depleak_dn6 = assign9290_e4757_d_n6;
        var_uc_depleak_dn7 = assign9290_e4757_d_n7;
        var_uc_depleak_dn8 = assign9290_e4757_d_n8;
        var_uc_depleak_dn9 = assign9290_e4757_d_n9;
        var_uc_depleak_dn10 = assign9290_e4757_d_n10;
        var_uc_depleak_dn11 = assign9290_e4757_d_n11;
        var_uc_depleak_dn14 = assign9290_e4757_d_n14;
        var_uc_depleak_rv = 0.0;

        let assign9300_e4760: f64 = if var_uc_depleak < 0.0 { 1.0 } else { 0.0 };
        var_guard189 = assign9300_e4760;
        var_guard189_rv = 0.0;

        let (assign9310_e4766, assign9310_e4766_d_n0, assign9310_e4766_d_n2, assign9310_e4766_d_n4, assign9310_e4766_d_n5, assign9310_e4766_d_n6, assign9310_e4766_d_n7, assign9310_e4766_d_n8, assign9310_e4766_d_n9, assign9310_e4766_d_n10, assign9310_e4766_d_n11, assign9310_e4766_d_n14,) = {
    if ((var_guard187 != 0.0) && (var_guard189 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depleak, var_uc_depleak_dn0, var_uc_depleak_dn2, var_uc_depleak_dn4, var_uc_depleak_dn5, var_uc_depleak_dn6, var_uc_depleak_dn7, var_uc_depleak_dn8, var_uc_depleak_dn9, var_uc_depleak_dn10, var_uc_depleak_dn11, var_uc_depleak_dn14,)
    }
};
        var_uc_depleak = assign9310_e4766;
        var_uc_depleak_dn0 = assign9310_e4766_d_n0;
        var_uc_depleak_dn2 = assign9310_e4766_d_n2;
        var_uc_depleak_dn4 = assign9310_e4766_d_n4;
        var_uc_depleak_dn5 = assign9310_e4766_d_n5;
        var_uc_depleak_dn6 = assign9310_e4766_d_n6;
        var_uc_depleak_dn7 = assign9310_e4766_d_n7;
        var_uc_depleak_dn8 = assign9310_e4766_d_n8;
        var_uc_depleak_dn9 = assign9310_e4766_d_n9;
        var_uc_depleak_dn10 = assign9310_e4766_d_n10;
        var_uc_depleak_dn11 = assign9310_e4766_d_n11;
        var_uc_depleak_dn14 = assign9310_e4766_d_n14;
        var_uc_depleak_rv = 0.0;

        let (assign9320_e4772, assign9320_e4772_d_n0, assign9320_e4772_d_n2, assign9320_e4772_d_n4, assign9320_e4772_d_n5, assign9320_e4772_d_n6, assign9320_e4772_d_n7, assign9320_e4772_d_n8, assign9320_e4772_d_n9, assign9320_e4772_d_n10, assign9320_e4772_d_n11, assign9320_e4772_d_n14,) = {
    if (var_guard187 != 0.0) {
        let assign9320_e4770: f64 = (var_lg).powf(p.p348);
        (assign9320_e4770, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn14,)
    }
};
        var_t3 = assign9320_e4772;
        var_t3_dn0 = assign9320_e4772_d_n0;
        var_t3_dn2 = assign9320_e4772_d_n2;
        var_t3_dn4 = assign9320_e4772_d_n4;
        var_t3_dn5 = assign9320_e4772_d_n5;
        var_t3_dn6 = assign9320_e4772_d_n6;
        var_t3_dn7 = assign9320_e4772_d_n7;
        var_t3_dn8 = assign9320_e4772_d_n8;
        var_t3_dn9 = assign9320_e4772_d_n9;
        var_t3_dn10 = assign9320_e4772_d_n10;
        var_t3_dn11 = assign9320_e4772_d_n11;
        var_t3_dn14 = assign9320_e4772_d_n14;
        var_t3_rv = 0.0;

        let (assign9330_e4782, assign9330_e4782_d_n0, assign9330_e4782_d_n2, assign9330_e4782_d_n4, assign9330_e4782_d_n5, assign9330_e4782_d_n6, assign9330_e4782_d_n7, assign9330_e4782_d_n8, assign9330_e4782_d_n9, assign9330_e4782_d_n10, assign9330_e4782_d_n11, assign9330_e4782_d_n14,) = {
    if (var_guard187 != 0.0) {
        let assign9330_e4778: f64 = (p.p347 / var_t3);
        let assign9330_e4779: f64 = (1.0 + assign9330_e4778);
        let assign9330_e4780: f64 = (p.p346 * assign9330_e4779);
        (assign9330_e4780, (p.p346 * (-((p.p347 * var_t3_dn0) / (var_t3 * var_t3)))), (p.p346 * (-((p.p347 * var_t3_dn2) / (var_t3 * var_t3)))), (p.p346 * (-((p.p347 * var_t3_dn4) / (var_t3 * var_t3)))), (p.p346 * (-((p.p347 * var_t3_dn5) / (var_t3 * var_t3)))), (p.p346 * (-((p.p347 * var_t3_dn6) / (var_t3 * var_t3)))), (p.p346 * (-((p.p347 * var_t3_dn7) / (var_t3 * var_t3)))), (p.p346 * (-((p.p347 * var_t3_dn8) / (var_t3 * var_t3)))), (p.p346 * (-((p.p347 * var_t3_dn9) / (var_t3 * var_t3)))), (p.p346 * (-((p.p347 * var_t3_dn10) / (var_t3 * var_t3)))), (p.p346 * (-((p.p347 * var_t3_dn11) / (var_t3 * var_t3)))), (p.p346 * (-((p.p347 * var_t3_dn14) / (var_t3 * var_t3)))),)
    } else {
        (var_uc_depmue0, var_uc_depmue0_dn0, var_uc_depmue0_dn2, var_uc_depmue0_dn4, var_uc_depmue0_dn5, var_uc_depmue0_dn6, var_uc_depmue0_dn7, var_uc_depmue0_dn8, var_uc_depmue0_dn9, var_uc_depmue0_dn10, var_uc_depmue0_dn11, var_uc_depmue0_dn14,)
    }
};
        var_uc_depmue0 = assign9330_e4782;
        var_uc_depmue0_dn0 = assign9330_e4782_d_n0;
        var_uc_depmue0_dn2 = assign9330_e4782_d_n2;
        var_uc_depmue0_dn4 = assign9330_e4782_d_n4;
        var_uc_depmue0_dn5 = assign9330_e4782_d_n5;
        var_uc_depmue0_dn6 = assign9330_e4782_d_n6;
        var_uc_depmue0_dn7 = assign9330_e4782_d_n7;
        var_uc_depmue0_dn8 = assign9330_e4782_d_n8;
        var_uc_depmue0_dn9 = assign9330_e4782_d_n9;
        var_uc_depmue0_dn10 = assign9330_e4782_d_n10;
        var_uc_depmue0_dn11 = assign9330_e4782_d_n11;
        var_uc_depmue0_dn14 = assign9330_e4782_d_n14;
        var_uc_depmue0_rv = 0.0;

        let assign9340_e4785: f64 = if var_uc_depmue0 < 1.0 { 1.0 } else { 0.0 };
        var_guard190 = assign9340_e4785;
        var_guard190_rv = 0.0;

        let (assign9350_e4791, assign9350_e4791_d_n0, assign9350_e4791_d_n2, assign9350_e4791_d_n4, assign9350_e4791_d_n5, assign9350_e4791_d_n6, assign9350_e4791_d_n7, assign9350_e4791_d_n8, assign9350_e4791_d_n9, assign9350_e4791_d_n10, assign9350_e4791_d_n11, assign9350_e4791_d_n14,) = {
    if ((var_guard187 != 0.0) && (var_guard190 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depmue0, var_uc_depmue0_dn0, var_uc_depmue0_dn2, var_uc_depmue0_dn4, var_uc_depmue0_dn5, var_uc_depmue0_dn6, var_uc_depmue0_dn7, var_uc_depmue0_dn8, var_uc_depmue0_dn9, var_uc_depmue0_dn10, var_uc_depmue0_dn11, var_uc_depmue0_dn14,)
    }
};
        var_uc_depmue0 = assign9350_e4791;
        var_uc_depmue0_dn0 = assign9350_e4791_d_n0;
        var_uc_depmue0_dn2 = assign9350_e4791_d_n2;
        var_uc_depmue0_dn4 = assign9350_e4791_d_n4;
        var_uc_depmue0_dn5 = assign9350_e4791_d_n5;
        var_uc_depmue0_dn6 = assign9350_e4791_d_n6;
        var_uc_depmue0_dn7 = assign9350_e4791_d_n7;
        var_uc_depmue0_dn8 = assign9350_e4791_d_n8;
        var_uc_depmue0_dn9 = assign9350_e4791_d_n9;
        var_uc_depmue0_dn10 = assign9350_e4791_d_n10;
        var_uc_depmue0_dn11 = assign9350_e4791_d_n11;
        var_uc_depmue0_dn14 = assign9350_e4791_d_n14;
        var_uc_depmue0_rv = 0.0;

        let (assign9360_e4797, assign9360_e4797_d_n0, assign9360_e4797_d_n2, assign9360_e4797_d_n4, assign9360_e4797_d_n5, assign9360_e4797_d_n6, assign9360_e4797_d_n7, assign9360_e4797_d_n8, assign9360_e4797_d_n9, assign9360_e4797_d_n10, assign9360_e4797_d_n11, assign9360_e4797_d_n14,) = {
    if (var_guard187 != 0.0) {
        let assign9360_e4795: f64 = (var_lg).powf(p.p351);
        (assign9360_e4795, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn14,)
    }
};
        var_t3 = assign9360_e4797;
        var_t3_dn0 = assign9360_e4797_d_n0;
        var_t3_dn2 = assign9360_e4797_d_n2;
        var_t3_dn4 = assign9360_e4797_d_n4;
        var_t3_dn5 = assign9360_e4797_d_n5;
        var_t3_dn6 = assign9360_e4797_d_n6;
        var_t3_dn7 = assign9360_e4797_d_n7;
        var_t3_dn8 = assign9360_e4797_d_n8;
        var_t3_dn9 = assign9360_e4797_d_n9;
        var_t3_dn10 = assign9360_e4797_d_n10;
        var_t3_dn11 = assign9360_e4797_d_n11;
        var_t3_dn14 = assign9360_e4797_d_n14;
        var_t3_rv = 0.0;

        let (assign9370_e4807, assign9370_e4807_d_n0, assign9370_e4807_d_n2, assign9370_e4807_d_n4, assign9370_e4807_d_n5, assign9370_e4807_d_n6, assign9370_e4807_d_n7, assign9370_e4807_d_n8, assign9370_e4807_d_n9, assign9370_e4807_d_n10, assign9370_e4807_d_n11, assign9370_e4807_d_n14,) = {
    if (var_guard187 != 0.0) {
        let assign9370_e4803: f64 = (p.p350 / var_t3);
        let assign9370_e4804: f64 = (1.0 + assign9370_e4803);
        let assign9370_e4805: f64 = (p.p349 * assign9370_e4804);
        (assign9370_e4805, (p.p349 * (-((p.p350 * var_t3_dn0) / (var_t3 * var_t3)))), (p.p349 * (-((p.p350 * var_t3_dn2) / (var_t3 * var_t3)))), (p.p349 * (-((p.p350 * var_t3_dn4) / (var_t3 * var_t3)))), (p.p349 * (-((p.p350 * var_t3_dn5) / (var_t3 * var_t3)))), (p.p349 * (-((p.p350 * var_t3_dn6) / (var_t3 * var_t3)))), (p.p349 * (-((p.p350 * var_t3_dn7) / (var_t3 * var_t3)))), (p.p349 * (-((p.p350 * var_t3_dn8) / (var_t3 * var_t3)))), (p.p349 * (-((p.p350 * var_t3_dn9) / (var_t3 * var_t3)))), (p.p349 * (-((p.p350 * var_t3_dn10) / (var_t3 * var_t3)))), (p.p349 * (-((p.p350 * var_t3_dn11) / (var_t3 * var_t3)))), (p.p349 * (-((p.p350 * var_t3_dn14) / (var_t3 * var_t3)))),)
    } else {
        (var_uc_depmue1, var_uc_depmue1_dn0, var_uc_depmue1_dn2, var_uc_depmue1_dn4, var_uc_depmue1_dn5, var_uc_depmue1_dn6, var_uc_depmue1_dn7, var_uc_depmue1_dn8, var_uc_depmue1_dn9, var_uc_depmue1_dn10, var_uc_depmue1_dn11, var_uc_depmue1_dn14,)
    }
};
        var_uc_depmue1 = assign9370_e4807;
        var_uc_depmue1_dn0 = assign9370_e4807_d_n0;
        var_uc_depmue1_dn2 = assign9370_e4807_d_n2;
        var_uc_depmue1_dn4 = assign9370_e4807_d_n4;
        var_uc_depmue1_dn5 = assign9370_e4807_d_n5;
        var_uc_depmue1_dn6 = assign9370_e4807_d_n6;
        var_uc_depmue1_dn7 = assign9370_e4807_d_n7;
        var_uc_depmue1_dn8 = assign9370_e4807_d_n8;
        var_uc_depmue1_dn9 = assign9370_e4807_d_n9;
        var_uc_depmue1_dn10 = assign9370_e4807_d_n10;
        var_uc_depmue1_dn11 = assign9370_e4807_d_n11;
        var_uc_depmue1_dn14 = assign9370_e4807_d_n14;
        var_uc_depmue1_rv = 0.0;

        let assign9380_e4810: f64 = if var_uc_depmue1 < 0.0 { 1.0 } else { 0.0 };
        var_guard191 = assign9380_e4810;
        var_guard191_rv = 0.0;

        let (assign9390_e4816, assign9390_e4816_d_n0, assign9390_e4816_d_n2, assign9390_e4816_d_n4, assign9390_e4816_d_n5, assign9390_e4816_d_n6, assign9390_e4816_d_n7, assign9390_e4816_d_n8, assign9390_e4816_d_n9, assign9390_e4816_d_n10, assign9390_e4816_d_n11, assign9390_e4816_d_n14,) = {
    if ((var_guard187 != 0.0) && (var_guard191 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depmue1, var_uc_depmue1_dn0, var_uc_depmue1_dn2, var_uc_depmue1_dn4, var_uc_depmue1_dn5, var_uc_depmue1_dn6, var_uc_depmue1_dn7, var_uc_depmue1_dn8, var_uc_depmue1_dn9, var_uc_depmue1_dn10, var_uc_depmue1_dn11, var_uc_depmue1_dn14,)
    }
};
        var_uc_depmue1 = assign9390_e4816;
        var_uc_depmue1_dn0 = assign9390_e4816_d_n0;
        var_uc_depmue1_dn2 = assign9390_e4816_d_n2;
        var_uc_depmue1_dn4 = assign9390_e4816_d_n4;
        var_uc_depmue1_dn5 = assign9390_e4816_d_n5;
        var_uc_depmue1_dn6 = assign9390_e4816_d_n6;
        var_uc_depmue1_dn7 = assign9390_e4816_d_n7;
        var_uc_depmue1_dn8 = assign9390_e4816_d_n8;
        var_uc_depmue1_dn9 = assign9390_e4816_d_n9;
        var_uc_depmue1_dn10 = assign9390_e4816_d_n10;
        var_uc_depmue1_dn11 = assign9390_e4816_d_n11;
        var_uc_depmue1_dn14 = assign9390_e4816_d_n14;
        var_uc_depmue1_rv = 0.0;

        let (assign9400_e4822, assign9400_e4822_d_n0, assign9400_e4822_d_n2, assign9400_e4822_d_n4, assign9400_e4822_d_n5, assign9400_e4822_d_n6, assign9400_e4822_d_n7, assign9400_e4822_d_n8, assign9400_e4822_d_n9, assign9400_e4822_d_n10, assign9400_e4822_d_n11, assign9400_e4822_d_n14,) = {
    if (var_guard187 != 0.0) {
        let assign9400_e4820: f64 = (var_lg).powf(p.p357);
        (assign9400_e4820, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn14,)
    }
};
        var_t3 = assign9400_e4822;
        var_t3_dn0 = assign9400_e4822_d_n0;
        var_t3_dn2 = assign9400_e4822_d_n2;
        var_t3_dn4 = assign9400_e4822_d_n4;
        var_t3_dn5 = assign9400_e4822_d_n5;
        var_t3_dn6 = assign9400_e4822_d_n6;
        var_t3_dn7 = assign9400_e4822_d_n7;
        var_t3_dn8 = assign9400_e4822_d_n8;
        var_t3_dn9 = assign9400_e4822_d_n9;
        var_t3_dn10 = assign9400_e4822_d_n10;
        var_t3_dn11 = assign9400_e4822_d_n11;
        var_t3_dn14 = assign9400_e4822_d_n14;
        var_t3_rv = 0.0;

        let (assign9410_e4832, assign9410_e4832_d_n0, assign9410_e4832_d_n2, assign9410_e4832_d_n4, assign9410_e4832_d_n5, assign9410_e4832_d_n6, assign9410_e4832_d_n7, assign9410_e4832_d_n8, assign9410_e4832_d_n9, assign9410_e4832_d_n10, assign9410_e4832_d_n11, assign9410_e4832_d_n14,) = {
    if (var_guard187 != 0.0) {
        let assign9410_e4828: f64 = (p.p356 / var_t3);
        let assign9410_e4829: f64 = (1.0 + assign9410_e4828);
        let assign9410_e4830: f64 = (p.p354 * assign9410_e4829);
        (assign9410_e4830, (p.p354 * (-((p.p356 * var_t3_dn0) / (var_t3 * var_t3)))), (p.p354 * (-((p.p356 * var_t3_dn2) / (var_t3 * var_t3)))), (p.p354 * (-((p.p356 * var_t3_dn4) / (var_t3 * var_t3)))), (p.p354 * (-((p.p356 * var_t3_dn5) / (var_t3 * var_t3)))), (p.p354 * (-((p.p356 * var_t3_dn6) / (var_t3 * var_t3)))), (p.p354 * (-((p.p356 * var_t3_dn7) / (var_t3 * var_t3)))), (p.p354 * (-((p.p356 * var_t3_dn8) / (var_t3 * var_t3)))), (p.p354 * (-((p.p356 * var_t3_dn9) / (var_t3 * var_t3)))), (p.p354 * (-((p.p356 * var_t3_dn10) / (var_t3 * var_t3)))), (p.p354 * (-((p.p356 * var_t3_dn11) / (var_t3 * var_t3)))), (p.p354 * (-((p.p356 * var_t3_dn14) / (var_t3 * var_t3)))),)
    } else {
        (var_uc_depmueback0, var_uc_depmueback0_dn0, var_uc_depmueback0_dn2, var_uc_depmueback0_dn4, var_uc_depmueback0_dn5, var_uc_depmueback0_dn6, var_uc_depmueback0_dn7, var_uc_depmueback0_dn8, var_uc_depmueback0_dn9, var_uc_depmueback0_dn10, var_uc_depmueback0_dn11, var_uc_depmueback0_dn14,)
    }
};
        var_uc_depmueback0 = assign9410_e4832;
        var_uc_depmueback0_dn0 = assign9410_e4832_d_n0;
        var_uc_depmueback0_dn2 = assign9410_e4832_d_n2;
        var_uc_depmueback0_dn4 = assign9410_e4832_d_n4;
        var_uc_depmueback0_dn5 = assign9410_e4832_d_n5;
        var_uc_depmueback0_dn6 = assign9410_e4832_d_n6;
        var_uc_depmueback0_dn7 = assign9410_e4832_d_n7;
        var_uc_depmueback0_dn8 = assign9410_e4832_d_n8;
        var_uc_depmueback0_dn9 = assign9410_e4832_d_n9;
        var_uc_depmueback0_dn10 = assign9410_e4832_d_n10;
        var_uc_depmueback0_dn11 = assign9410_e4832_d_n11;
        var_uc_depmueback0_dn14 = assign9410_e4832_d_n14;
        var_uc_depmueback0_rv = 0.0;

        let assign9420_e4835: f64 = if var_uc_depmueback0 < 0.0 { 1.0 } else { 0.0 };
        var_guard192 = assign9420_e4835;
        var_guard192_rv = 0.0;

        let (assign9430_e4841, assign9430_e4841_d_n0, assign9430_e4841_d_n2, assign9430_e4841_d_n4, assign9430_e4841_d_n5, assign9430_e4841_d_n6, assign9430_e4841_d_n7, assign9430_e4841_d_n8, assign9430_e4841_d_n9, assign9430_e4841_d_n10, assign9430_e4841_d_n11, assign9430_e4841_d_n14,) = {
    if ((var_guard187 != 0.0) && (var_guard192 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depmueback0, var_uc_depmueback0_dn0, var_uc_depmueback0_dn2, var_uc_depmueback0_dn4, var_uc_depmueback0_dn5, var_uc_depmueback0_dn6, var_uc_depmueback0_dn7, var_uc_depmueback0_dn8, var_uc_depmueback0_dn9, var_uc_depmueback0_dn10, var_uc_depmueback0_dn11, var_uc_depmueback0_dn14,)
    }
};
        var_uc_depmueback0 = assign9430_e4841;
        var_uc_depmueback0_dn0 = assign9430_e4841_d_n0;
        var_uc_depmueback0_dn2 = assign9430_e4841_d_n2;
        var_uc_depmueback0_dn4 = assign9430_e4841_d_n4;
        var_uc_depmueback0_dn5 = assign9430_e4841_d_n5;
        var_uc_depmueback0_dn6 = assign9430_e4841_d_n6;
        var_uc_depmueback0_dn7 = assign9430_e4841_d_n7;
        var_uc_depmueback0_dn8 = assign9430_e4841_d_n8;
        var_uc_depmueback0_dn9 = assign9430_e4841_d_n9;
        var_uc_depmueback0_dn10 = assign9430_e4841_d_n10;
        var_uc_depmueback0_dn11 = assign9430_e4841_d_n11;
        var_uc_depmueback0_dn14 = assign9430_e4841_d_n14;
        var_uc_depmueback0_rv = 0.0;

        let (assign9440_e4847, assign9440_e4847_d_n0, assign9440_e4847_d_n2, assign9440_e4847_d_n4, assign9440_e4847_d_n5, assign9440_e4847_d_n6, assign9440_e4847_d_n7, assign9440_e4847_d_n8, assign9440_e4847_d_n9, assign9440_e4847_d_n10, assign9440_e4847_d_n11, assign9440_e4847_d_n14,) = {
    if (var_guard187 != 0.0) {
        let assign9440_e4845: f64 = (var_lg).powf(p.p359);
        (assign9440_e4845, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn14,)
    }
};
        var_t3 = assign9440_e4847;
        var_t3_dn0 = assign9440_e4847_d_n0;
        var_t3_dn2 = assign9440_e4847_d_n2;
        var_t3_dn4 = assign9440_e4847_d_n4;
        var_t3_dn5 = assign9440_e4847_d_n5;
        var_t3_dn6 = assign9440_e4847_d_n6;
        var_t3_dn7 = assign9440_e4847_d_n7;
        var_t3_dn8 = assign9440_e4847_d_n8;
        var_t3_dn9 = assign9440_e4847_d_n9;
        var_t3_dn10 = assign9440_e4847_d_n10;
        var_t3_dn11 = assign9440_e4847_d_n11;
        var_t3_dn14 = assign9440_e4847_d_n14;
        var_t3_rv = 0.0;

        let (assign9450_e4857, assign9450_e4857_d_n0, assign9450_e4857_d_n2, assign9450_e4857_d_n4, assign9450_e4857_d_n5, assign9450_e4857_d_n6, assign9450_e4857_d_n7, assign9450_e4857_d_n8, assign9450_e4857_d_n9, assign9450_e4857_d_n10, assign9450_e4857_d_n11, assign9450_e4857_d_n14,) = {
    if (var_guard187 != 0.0) {
        let assign9450_e4853: f64 = (p.p358 / var_t3);
        let assign9450_e4854: f64 = (1.0 + assign9450_e4853);
        let assign9450_e4855: f64 = (p.p355 * assign9450_e4854);
        (assign9450_e4855, (p.p355 * (-((p.p358 * var_t3_dn0) / (var_t3 * var_t3)))), (p.p355 * (-((p.p358 * var_t3_dn2) / (var_t3 * var_t3)))), (p.p355 * (-((p.p358 * var_t3_dn4) / (var_t3 * var_t3)))), (p.p355 * (-((p.p358 * var_t3_dn5) / (var_t3 * var_t3)))), (p.p355 * (-((p.p358 * var_t3_dn6) / (var_t3 * var_t3)))), (p.p355 * (-((p.p358 * var_t3_dn7) / (var_t3 * var_t3)))), (p.p355 * (-((p.p358 * var_t3_dn8) / (var_t3 * var_t3)))), (p.p355 * (-((p.p358 * var_t3_dn9) / (var_t3 * var_t3)))), (p.p355 * (-((p.p358 * var_t3_dn10) / (var_t3 * var_t3)))), (p.p355 * (-((p.p358 * var_t3_dn11) / (var_t3 * var_t3)))), (p.p355 * (-((p.p358 * var_t3_dn14) / (var_t3 * var_t3)))),)
    } else {
        (var_uc_depmueback1, var_uc_depmueback1_dn0, var_uc_depmueback1_dn2, var_uc_depmueback1_dn4, var_uc_depmueback1_dn5, var_uc_depmueback1_dn6, var_uc_depmueback1_dn7, var_uc_depmueback1_dn8, var_uc_depmueback1_dn9, var_uc_depmueback1_dn10, var_uc_depmueback1_dn11, var_uc_depmueback1_dn14,)
    }
};
        var_uc_depmueback1 = assign9450_e4857;
        var_uc_depmueback1_dn0 = assign9450_e4857_d_n0;
        var_uc_depmueback1_dn2 = assign9450_e4857_d_n2;
        var_uc_depmueback1_dn4 = assign9450_e4857_d_n4;
        var_uc_depmueback1_dn5 = assign9450_e4857_d_n5;
        var_uc_depmueback1_dn6 = assign9450_e4857_d_n6;
        var_uc_depmueback1_dn7 = assign9450_e4857_d_n7;
        var_uc_depmueback1_dn8 = assign9450_e4857_d_n8;
        var_uc_depmueback1_dn9 = assign9450_e4857_d_n9;
        var_uc_depmueback1_dn10 = assign9450_e4857_d_n10;
        var_uc_depmueback1_dn11 = assign9450_e4857_d_n11;
        var_uc_depmueback1_dn14 = assign9450_e4857_d_n14;
        var_uc_depmueback1_rv = 0.0;

        let assign9460_e4860: f64 = if var_uc_depmueback1 < 0.0 { 1.0 } else { 0.0 };
        var_guard193 = assign9460_e4860;
        var_guard193_rv = 0.0;

        let (assign9470_e4866, assign9470_e4866_d_n0, assign9470_e4866_d_n2, assign9470_e4866_d_n4, assign9470_e4866_d_n5, assign9470_e4866_d_n6, assign9470_e4866_d_n7, assign9470_e4866_d_n8, assign9470_e4866_d_n9, assign9470_e4866_d_n10, assign9470_e4866_d_n11, assign9470_e4866_d_n14,) = {
    if ((var_guard187 != 0.0) && (var_guard193 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depmueback1, var_uc_depmueback1_dn0, var_uc_depmueback1_dn2, var_uc_depmueback1_dn4, var_uc_depmueback1_dn5, var_uc_depmueback1_dn6, var_uc_depmueback1_dn7, var_uc_depmueback1_dn8, var_uc_depmueback1_dn9, var_uc_depmueback1_dn10, var_uc_depmueback1_dn11, var_uc_depmueback1_dn14,)
    }
};
        var_uc_depmueback1 = assign9470_e4866;
        var_uc_depmueback1_dn0 = assign9470_e4866_d_n0;
        var_uc_depmueback1_dn2 = assign9470_e4866_d_n2;
        var_uc_depmueback1_dn4 = assign9470_e4866_d_n4;
        var_uc_depmueback1_dn5 = assign9470_e4866_d_n5;
        var_uc_depmueback1_dn6 = assign9470_e4866_d_n6;
        var_uc_depmueback1_dn7 = assign9470_e4866_d_n7;
        var_uc_depmueback1_dn8 = assign9470_e4866_d_n8;
        var_uc_depmueback1_dn9 = assign9470_e4866_d_n9;
        var_uc_depmueback1_dn10 = assign9470_e4866_d_n10;
        var_uc_depmueback1_dn11 = assign9470_e4866_d_n11;
        var_uc_depmueback1_dn14 = assign9470_e4866_d_n14;
        var_uc_depmueback1_rv = 0.0;

        *var_guard187_slot = var_guard187;
        *var_guard187_rv_slot = var_guard187_rv;
        *var_guard188_slot = var_guard188;
        *var_guard188_rv_slot = var_guard188_rv;
        *var_guard189_slot = var_guard189;
        *var_guard189_rv_slot = var_guard189_rv;
        *var_guard190_slot = var_guard190;
        *var_guard190_rv_slot = var_guard190_rv;
        *var_guard191_slot = var_guard191;
        *var_guard191_rv_slot = var_guard191_rv;
        *var_guard192_slot = var_guard192;
        *var_guard192_rv_slot = var_guard192_rv;
        *var_guard193_slot = var_guard193;
        *var_guard193_rv_slot = var_guard193_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn14_slot = var_t3_dn14;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t3_rv_slot = var_t3_rv;
        *var_uc_depleak_slot = var_uc_depleak;
        *var_uc_depleak_dn0_slot = var_uc_depleak_dn0;
        *var_uc_depleak_dn10_slot = var_uc_depleak_dn10;
        *var_uc_depleak_dn11_slot = var_uc_depleak_dn11;
        *var_uc_depleak_dn14_slot = var_uc_depleak_dn14;
        *var_uc_depleak_dn2_slot = var_uc_depleak_dn2;
        *var_uc_depleak_dn4_slot = var_uc_depleak_dn4;
        *var_uc_depleak_dn5_slot = var_uc_depleak_dn5;
        *var_uc_depleak_dn6_slot = var_uc_depleak_dn6;
        *var_uc_depleak_dn7_slot = var_uc_depleak_dn7;
        *var_uc_depleak_dn8_slot = var_uc_depleak_dn8;
        *var_uc_depleak_dn9_slot = var_uc_depleak_dn9;
        *var_uc_depleak_rv_slot = var_uc_depleak_rv;
        *var_uc_depmue0_slot = var_uc_depmue0;
        *var_uc_depmue0_dn0_slot = var_uc_depmue0_dn0;
        *var_uc_depmue0_dn10_slot = var_uc_depmue0_dn10;
        *var_uc_depmue0_dn11_slot = var_uc_depmue0_dn11;
        *var_uc_depmue0_dn14_slot = var_uc_depmue0_dn14;
        *var_uc_depmue0_dn2_slot = var_uc_depmue0_dn2;
        *var_uc_depmue0_dn4_slot = var_uc_depmue0_dn4;
        *var_uc_depmue0_dn5_slot = var_uc_depmue0_dn5;
        *var_uc_depmue0_dn6_slot = var_uc_depmue0_dn6;
        *var_uc_depmue0_dn7_slot = var_uc_depmue0_dn7;
        *var_uc_depmue0_dn8_slot = var_uc_depmue0_dn8;
        *var_uc_depmue0_dn9_slot = var_uc_depmue0_dn9;
        *var_uc_depmue0_rv_slot = var_uc_depmue0_rv;
        *var_uc_depmue1_slot = var_uc_depmue1;
        *var_uc_depmue1_dn0_slot = var_uc_depmue1_dn0;
        *var_uc_depmue1_dn10_slot = var_uc_depmue1_dn10;
        *var_uc_depmue1_dn11_slot = var_uc_depmue1_dn11;
        *var_uc_depmue1_dn14_slot = var_uc_depmue1_dn14;
        *var_uc_depmue1_dn2_slot = var_uc_depmue1_dn2;
        *var_uc_depmue1_dn4_slot = var_uc_depmue1_dn4;
        *var_uc_depmue1_dn5_slot = var_uc_depmue1_dn5;
        *var_uc_depmue1_dn6_slot = var_uc_depmue1_dn6;
        *var_uc_depmue1_dn7_slot = var_uc_depmue1_dn7;
        *var_uc_depmue1_dn8_slot = var_uc_depmue1_dn8;
        *var_uc_depmue1_dn9_slot = var_uc_depmue1_dn9;
        *var_uc_depmue1_rv_slot = var_uc_depmue1_rv;
        *var_uc_depmueback0_slot = var_uc_depmueback0;
        *var_uc_depmueback0_dn0_slot = var_uc_depmueback0_dn0;
        *var_uc_depmueback0_dn10_slot = var_uc_depmueback0_dn10;
        *var_uc_depmueback0_dn11_slot = var_uc_depmueback0_dn11;
        *var_uc_depmueback0_dn14_slot = var_uc_depmueback0_dn14;
        *var_uc_depmueback0_dn2_slot = var_uc_depmueback0_dn2;
        *var_uc_depmueback0_dn4_slot = var_uc_depmueback0_dn4;
        *var_uc_depmueback0_dn5_slot = var_uc_depmueback0_dn5;
        *var_uc_depmueback0_dn6_slot = var_uc_depmueback0_dn6;
        *var_uc_depmueback0_dn7_slot = var_uc_depmueback0_dn7;
        *var_uc_depmueback0_dn8_slot = var_uc_depmueback0_dn8;
        *var_uc_depmueback0_dn9_slot = var_uc_depmueback0_dn9;
        *var_uc_depmueback0_rv_slot = var_uc_depmueback0_rv;
        *var_uc_depmueback1_slot = var_uc_depmueback1;
        *var_uc_depmueback1_dn0_slot = var_uc_depmueback1_dn0;
        *var_uc_depmueback1_dn10_slot = var_uc_depmueback1_dn10;
        *var_uc_depmueback1_dn11_slot = var_uc_depmueback1_dn11;
        *var_uc_depmueback1_dn14_slot = var_uc_depmueback1_dn14;
        *var_uc_depmueback1_dn2_slot = var_uc_depmueback1_dn2;
        *var_uc_depmueback1_dn4_slot = var_uc_depmueback1_dn4;
        *var_uc_depmueback1_dn5_slot = var_uc_depmueback1_dn5;
        *var_uc_depmueback1_dn6_slot = var_uc_depmueback1_dn6;
        *var_uc_depmueback1_dn7_slot = var_uc_depmueback1_dn7;
        *var_uc_depmueback1_dn8_slot = var_uc_depmueback1_dn8;
        *var_uc_depmueback1_dn9_slot = var_uc_depmueback1_dn9;
        *var_uc_depmueback1_rv_slot = var_uc_depmueback1_rv;
        *var_uc_depvmax_slot = var_uc_depvmax;
        *var_uc_depvmax_dn0_slot = var_uc_depvmax_dn0;
        *var_uc_depvmax_dn10_slot = var_uc_depvmax_dn10;
        *var_uc_depvmax_dn11_slot = var_uc_depvmax_dn11;
        *var_uc_depvmax_dn14_slot = var_uc_depvmax_dn14;
        *var_uc_depvmax_dn2_slot = var_uc_depvmax_dn2;
        *var_uc_depvmax_dn4_slot = var_uc_depvmax_dn4;
        *var_uc_depvmax_dn5_slot = var_uc_depvmax_dn5;
        *var_uc_depvmax_dn6_slot = var_uc_depvmax_dn6;
        *var_uc_depvmax_dn7_slot = var_uc_depvmax_dn7;
        *var_uc_depvmax_dn8_slot = var_uc_depvmax_dn8;
        *var_uc_depvmax_dn9_slot = var_uc_depvmax_dn9;
        *var_uc_depvmax_rv_slot = var_uc_depvmax_rv;
        *var_uc_js0sws_slot = var_uc_js0sws;
        *var_uc_js0sws_rv_slot = var_uc_js0sws_rv;
        *var_uc_ndepm_slot = var_uc_ndepm;
        *var_uc_ndepm_dn0_slot = var_uc_ndepm_dn0;
        *var_uc_ndepm_dn10_slot = var_uc_ndepm_dn10;
        *var_uc_ndepm_dn11_slot = var_uc_ndepm_dn11;
        *var_uc_ndepm_dn14_slot = var_uc_ndepm_dn14;
        *var_uc_ndepm_dn2_slot = var_uc_ndepm_dn2;
        *var_uc_ndepm_dn4_slot = var_uc_ndepm_dn4;
        *var_uc_ndepm_dn5_slot = var_uc_ndepm_dn5;
        *var_uc_ndepm_dn6_slot = var_uc_ndepm_dn6;
        *var_uc_ndepm_dn7_slot = var_uc_ndepm_dn7;
        *var_uc_ndepm_dn8_slot = var_uc_ndepm_dn8;
        *var_uc_ndepm_dn9_slot = var_uc_ndepm_dn9;
        *var_uc_ndepm_rv_slot = var_uc_ndepm_rv;
        *var_uc_njs_slot = var_uc_njs;
        *var_uc_njs_rv_slot = var_uc_njs_rv;
        *var_uc_vdiffjs_slot = var_uc_vdiffjs;
        *var_uc_vdiffjs_rv_slot = var_uc_vdiffjs_rv;
    }

    pub(super) fn stamp_reactive_block_16(
        p: &Parameters,
        var_guard187: f64,
        var_lg: f64,
        var_uc_nover: f64,
        var_uc_novers: f64,
        var_uc_rd: f64,
        var_uc_rdict1: f64,
        var_uc_rdslp1: f64,
        var_uc_rdvd: f64,
        var_uc_rs: f64,
        var_uc_xldld: f64,
        var_flg_rd_slot: &mut f64,
        var_flg_rd_rv_slot: &mut f64,
        var_flg_rs_slot: &mut f64,
        var_flg_rs_rv_slot: &mut f64,
        var_guard194_slot: &mut f64,
        var_guard194_rv_slot: &mut f64,
        var_guard246_slot: &mut f64,
        var_guard246_rv_slot: &mut f64,
        var_guard248_slot: &mut f64,
        var_guard248_rv_slot: &mut f64,
        var_guard249_slot: &mut f64,
        var_guard249_rv_slot: &mut f64,
        var_guard250_slot: &mut f64,
        var_guard250_rv_slot: &mut f64,
        var_mks_nsubcdfm_slot: &mut f64,
        var_mks_nsubcdfm_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn14_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_uc_cordrift_slot: &mut f64,
        var_uc_cordrift_rv_slot: &mut f64,
        var_uc_depleak_slot: &mut f64,
        var_uc_depleak_dn0_slot: &mut f64,
        var_uc_depleak_dn10_slot: &mut f64,
        var_uc_depleak_dn11_slot: &mut f64,
        var_uc_depleak_dn14_slot: &mut f64,
        var_uc_depleak_dn2_slot: &mut f64,
        var_uc_depleak_dn4_slot: &mut f64,
        var_uc_depleak_dn5_slot: &mut f64,
        var_uc_depleak_dn6_slot: &mut f64,
        var_uc_depleak_dn7_slot: &mut f64,
        var_uc_depleak_dn8_slot: &mut f64,
        var_uc_depleak_dn9_slot: &mut f64,
        var_uc_depleak_rv_slot: &mut f64,
        var_uc_depmue0_slot: &mut f64,
        var_uc_depmue0_dn0_slot: &mut f64,
        var_uc_depmue0_dn10_slot: &mut f64,
        var_uc_depmue0_dn11_slot: &mut f64,
        var_uc_depmue0_dn14_slot: &mut f64,
        var_uc_depmue0_dn2_slot: &mut f64,
        var_uc_depmue0_dn4_slot: &mut f64,
        var_uc_depmue0_dn5_slot: &mut f64,
        var_uc_depmue0_dn6_slot: &mut f64,
        var_uc_depmue0_dn7_slot: &mut f64,
        var_uc_depmue0_dn8_slot: &mut f64,
        var_uc_depmue0_dn9_slot: &mut f64,
        var_uc_depmue0_rv_slot: &mut f64,
        var_uc_depmue1_slot: &mut f64,
        var_uc_depmue1_dn0_slot: &mut f64,
        var_uc_depmue1_dn10_slot: &mut f64,
        var_uc_depmue1_dn11_slot: &mut f64,
        var_uc_depmue1_dn14_slot: &mut f64,
        var_uc_depmue1_dn2_slot: &mut f64,
        var_uc_depmue1_dn4_slot: &mut f64,
        var_uc_depmue1_dn5_slot: &mut f64,
        var_uc_depmue1_dn6_slot: &mut f64,
        var_uc_depmue1_dn7_slot: &mut f64,
        var_uc_depmue1_dn8_slot: &mut f64,
        var_uc_depmue1_dn9_slot: &mut f64,
        var_uc_depmue1_rv_slot: &mut f64,
        var_uc_depmueback0_slot: &mut f64,
        var_uc_depmueback0_dn0_slot: &mut f64,
        var_uc_depmueback0_dn10_slot: &mut f64,
        var_uc_depmueback0_dn11_slot: &mut f64,
        var_uc_depmueback0_dn14_slot: &mut f64,
        var_uc_depmueback0_dn2_slot: &mut f64,
        var_uc_depmueback0_dn4_slot: &mut f64,
        var_uc_depmueback0_dn5_slot: &mut f64,
        var_uc_depmueback0_dn6_slot: &mut f64,
        var_uc_depmueback0_dn7_slot: &mut f64,
        var_uc_depmueback0_dn8_slot: &mut f64,
        var_uc_depmueback0_dn9_slot: &mut f64,
        var_uc_depmueback0_rv_slot: &mut f64,
        var_uc_depmueback1_slot: &mut f64,
        var_uc_depmueback1_dn0_slot: &mut f64,
        var_uc_depmueback1_dn10_slot: &mut f64,
        var_uc_depmueback1_dn11_slot: &mut f64,
        var_uc_depmueback1_dn14_slot: &mut f64,
        var_uc_depmueback1_dn2_slot: &mut f64,
        var_uc_depmueback1_dn4_slot: &mut f64,
        var_uc_depmueback1_dn5_slot: &mut f64,
        var_uc_depmueback1_dn6_slot: &mut f64,
        var_uc_depmueback1_dn7_slot: &mut f64,
        var_uc_depmueback1_dn8_slot: &mut f64,
        var_uc_depmueback1_dn9_slot: &mut f64,
        var_uc_depmueback1_rv_slot: &mut f64,
        var_uc_depvdsef1_slot: &mut f64,
        var_uc_depvdsef1_dn0_slot: &mut f64,
        var_uc_depvdsef1_dn10_slot: &mut f64,
        var_uc_depvdsef1_dn11_slot: &mut f64,
        var_uc_depvdsef1_dn14_slot: &mut f64,
        var_uc_depvdsef1_dn2_slot: &mut f64,
        var_uc_depvdsef1_dn4_slot: &mut f64,
        var_uc_depvdsef1_dn5_slot: &mut f64,
        var_uc_depvdsef1_dn6_slot: &mut f64,
        var_uc_depvdsef1_dn7_slot: &mut f64,
        var_uc_depvdsef1_dn8_slot: &mut f64,
        var_uc_depvdsef1_dn9_slot: &mut f64,
        var_uc_depvdsef1_rv_slot: &mut f64,
        var_uc_depvdsef2_slot: &mut f64,
        var_uc_depvdsef2_dn0_slot: &mut f64,
        var_uc_depvdsef2_dn10_slot: &mut f64,
        var_uc_depvdsef2_dn11_slot: &mut f64,
        var_uc_depvdsef2_dn14_slot: &mut f64,
        var_uc_depvdsef2_dn2_slot: &mut f64,
        var_uc_depvdsef2_dn4_slot: &mut f64,
        var_uc_depvdsef2_dn5_slot: &mut f64,
        var_uc_depvdsef2_dn6_slot: &mut f64,
        var_uc_depvdsef2_dn7_slot: &mut f64,
        var_uc_depvdsef2_dn8_slot: &mut f64,
        var_uc_depvdsef2_dn9_slot: &mut f64,
        var_uc_depvdsef2_rv_slot: &mut f64,
        var_uc_depvmax_slot: &mut f64,
        var_uc_depvmax_dn0_slot: &mut f64,
        var_uc_depvmax_dn10_slot: &mut f64,
        var_uc_depvmax_dn11_slot: &mut f64,
        var_uc_depvmax_dn14_slot: &mut f64,
        var_uc_depvmax_dn2_slot: &mut f64,
        var_uc_depvmax_dn4_slot: &mut f64,
        var_uc_depvmax_dn5_slot: &mut f64,
        var_uc_depvmax_dn6_slot: &mut f64,
        var_uc_depvmax_dn7_slot: &mut f64,
        var_uc_depvmax_dn8_slot: &mut f64,
        var_uc_depvmax_dn9_slot: &mut f64,
        var_uc_depvmax_rv_slot: &mut f64,
        var_uc_ndepm_slot: &mut f64,
        var_uc_ndepm_dn0_slot: &mut f64,
        var_uc_ndepm_dn10_slot: &mut f64,
        var_uc_ndepm_dn11_slot: &mut f64,
        var_uc_ndepm_dn14_slot: &mut f64,
        var_uc_ndepm_dn2_slot: &mut f64,
        var_uc_ndepm_dn4_slot: &mut f64,
        var_uc_ndepm_dn5_slot: &mut f64,
        var_uc_ndepm_dn6_slot: &mut f64,
        var_uc_ndepm_dn7_slot: &mut f64,
        var_uc_ndepm_dn8_slot: &mut f64,
        var_uc_ndepm_dn9_slot: &mut f64,
        var_uc_ndepm_rv_slot: &mut f64,
        var_uc_xpdv_slot: &mut f64,
        var_uc_xpdv_rv_slot: &mut f64,
    ) {
        let mut var_flg_rd: f64 = *var_flg_rd_slot;
        let mut var_flg_rd_rv: f64 = *var_flg_rd_rv_slot;
        let mut var_flg_rs: f64 = *var_flg_rs_slot;
        let mut var_flg_rs_rv: f64 = *var_flg_rs_rv_slot;
        let mut var_guard194: f64 = *var_guard194_slot;
        let mut var_guard194_rv: f64 = *var_guard194_rv_slot;
        let mut var_guard246: f64 = *var_guard246_slot;
        let mut var_guard246_rv: f64 = *var_guard246_rv_slot;
        let mut var_guard248: f64 = *var_guard248_slot;
        let mut var_guard248_rv: f64 = *var_guard248_rv_slot;
        let mut var_guard249: f64 = *var_guard249_slot;
        let mut var_guard249_rv: f64 = *var_guard249_rv_slot;
        let mut var_guard250: f64 = *var_guard250_slot;
        let mut var_guard250_rv: f64 = *var_guard250_rv_slot;
        let mut var_mks_nsubcdfm: f64 = *var_mks_nsubcdfm_slot;
        let mut var_mks_nsubcdfm_rv: f64 = *var_mks_nsubcdfm_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn14: f64 = *var_t3_dn14_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_uc_cordrift: f64 = *var_uc_cordrift_slot;
        let mut var_uc_cordrift_rv: f64 = *var_uc_cordrift_rv_slot;
        let mut var_uc_depleak: f64 = *var_uc_depleak_slot;
        let mut var_uc_depleak_dn0: f64 = *var_uc_depleak_dn0_slot;
        let mut var_uc_depleak_dn10: f64 = *var_uc_depleak_dn10_slot;
        let mut var_uc_depleak_dn11: f64 = *var_uc_depleak_dn11_slot;
        let mut var_uc_depleak_dn14: f64 = *var_uc_depleak_dn14_slot;
        let mut var_uc_depleak_dn2: f64 = *var_uc_depleak_dn2_slot;
        let mut var_uc_depleak_dn4: f64 = *var_uc_depleak_dn4_slot;
        let mut var_uc_depleak_dn5: f64 = *var_uc_depleak_dn5_slot;
        let mut var_uc_depleak_dn6: f64 = *var_uc_depleak_dn6_slot;
        let mut var_uc_depleak_dn7: f64 = *var_uc_depleak_dn7_slot;
        let mut var_uc_depleak_dn8: f64 = *var_uc_depleak_dn8_slot;
        let mut var_uc_depleak_dn9: f64 = *var_uc_depleak_dn9_slot;
        let mut var_uc_depleak_rv: f64 = *var_uc_depleak_rv_slot;
        let mut var_uc_depmue0: f64 = *var_uc_depmue0_slot;
        let mut var_uc_depmue0_dn0: f64 = *var_uc_depmue0_dn0_slot;
        let mut var_uc_depmue0_dn10: f64 = *var_uc_depmue0_dn10_slot;
        let mut var_uc_depmue0_dn11: f64 = *var_uc_depmue0_dn11_slot;
        let mut var_uc_depmue0_dn14: f64 = *var_uc_depmue0_dn14_slot;
        let mut var_uc_depmue0_dn2: f64 = *var_uc_depmue0_dn2_slot;
        let mut var_uc_depmue0_dn4: f64 = *var_uc_depmue0_dn4_slot;
        let mut var_uc_depmue0_dn5: f64 = *var_uc_depmue0_dn5_slot;
        let mut var_uc_depmue0_dn6: f64 = *var_uc_depmue0_dn6_slot;
        let mut var_uc_depmue0_dn7: f64 = *var_uc_depmue0_dn7_slot;
        let mut var_uc_depmue0_dn8: f64 = *var_uc_depmue0_dn8_slot;
        let mut var_uc_depmue0_dn9: f64 = *var_uc_depmue0_dn9_slot;
        let mut var_uc_depmue0_rv: f64 = *var_uc_depmue0_rv_slot;
        let mut var_uc_depmue1: f64 = *var_uc_depmue1_slot;
        let mut var_uc_depmue1_dn0: f64 = *var_uc_depmue1_dn0_slot;
        let mut var_uc_depmue1_dn10: f64 = *var_uc_depmue1_dn10_slot;
        let mut var_uc_depmue1_dn11: f64 = *var_uc_depmue1_dn11_slot;
        let mut var_uc_depmue1_dn14: f64 = *var_uc_depmue1_dn14_slot;
        let mut var_uc_depmue1_dn2: f64 = *var_uc_depmue1_dn2_slot;
        let mut var_uc_depmue1_dn4: f64 = *var_uc_depmue1_dn4_slot;
        let mut var_uc_depmue1_dn5: f64 = *var_uc_depmue1_dn5_slot;
        let mut var_uc_depmue1_dn6: f64 = *var_uc_depmue1_dn6_slot;
        let mut var_uc_depmue1_dn7: f64 = *var_uc_depmue1_dn7_slot;
        let mut var_uc_depmue1_dn8: f64 = *var_uc_depmue1_dn8_slot;
        let mut var_uc_depmue1_dn9: f64 = *var_uc_depmue1_dn9_slot;
        let mut var_uc_depmue1_rv: f64 = *var_uc_depmue1_rv_slot;
        let mut var_uc_depmueback0: f64 = *var_uc_depmueback0_slot;
        let mut var_uc_depmueback0_dn0: f64 = *var_uc_depmueback0_dn0_slot;
        let mut var_uc_depmueback0_dn10: f64 = *var_uc_depmueback0_dn10_slot;
        let mut var_uc_depmueback0_dn11: f64 = *var_uc_depmueback0_dn11_slot;
        let mut var_uc_depmueback0_dn14: f64 = *var_uc_depmueback0_dn14_slot;
        let mut var_uc_depmueback0_dn2: f64 = *var_uc_depmueback0_dn2_slot;
        let mut var_uc_depmueback0_dn4: f64 = *var_uc_depmueback0_dn4_slot;
        let mut var_uc_depmueback0_dn5: f64 = *var_uc_depmueback0_dn5_slot;
        let mut var_uc_depmueback0_dn6: f64 = *var_uc_depmueback0_dn6_slot;
        let mut var_uc_depmueback0_dn7: f64 = *var_uc_depmueback0_dn7_slot;
        let mut var_uc_depmueback0_dn8: f64 = *var_uc_depmueback0_dn8_slot;
        let mut var_uc_depmueback0_dn9: f64 = *var_uc_depmueback0_dn9_slot;
        let mut var_uc_depmueback0_rv: f64 = *var_uc_depmueback0_rv_slot;
        let mut var_uc_depmueback1: f64 = *var_uc_depmueback1_slot;
        let mut var_uc_depmueback1_dn0: f64 = *var_uc_depmueback1_dn0_slot;
        let mut var_uc_depmueback1_dn10: f64 = *var_uc_depmueback1_dn10_slot;
        let mut var_uc_depmueback1_dn11: f64 = *var_uc_depmueback1_dn11_slot;
        let mut var_uc_depmueback1_dn14: f64 = *var_uc_depmueback1_dn14_slot;
        let mut var_uc_depmueback1_dn2: f64 = *var_uc_depmueback1_dn2_slot;
        let mut var_uc_depmueback1_dn4: f64 = *var_uc_depmueback1_dn4_slot;
        let mut var_uc_depmueback1_dn5: f64 = *var_uc_depmueback1_dn5_slot;
        let mut var_uc_depmueback1_dn6: f64 = *var_uc_depmueback1_dn6_slot;
        let mut var_uc_depmueback1_dn7: f64 = *var_uc_depmueback1_dn7_slot;
        let mut var_uc_depmueback1_dn8: f64 = *var_uc_depmueback1_dn8_slot;
        let mut var_uc_depmueback1_dn9: f64 = *var_uc_depmueback1_dn9_slot;
        let mut var_uc_depmueback1_rv: f64 = *var_uc_depmueback1_rv_slot;
        let mut var_uc_depvdsef1: f64 = *var_uc_depvdsef1_slot;
        let mut var_uc_depvdsef1_dn0: f64 = *var_uc_depvdsef1_dn0_slot;
        let mut var_uc_depvdsef1_dn10: f64 = *var_uc_depvdsef1_dn10_slot;
        let mut var_uc_depvdsef1_dn11: f64 = *var_uc_depvdsef1_dn11_slot;
        let mut var_uc_depvdsef1_dn14: f64 = *var_uc_depvdsef1_dn14_slot;
        let mut var_uc_depvdsef1_dn2: f64 = *var_uc_depvdsef1_dn2_slot;
        let mut var_uc_depvdsef1_dn4: f64 = *var_uc_depvdsef1_dn4_slot;
        let mut var_uc_depvdsef1_dn5: f64 = *var_uc_depvdsef1_dn5_slot;
        let mut var_uc_depvdsef1_dn6: f64 = *var_uc_depvdsef1_dn6_slot;
        let mut var_uc_depvdsef1_dn7: f64 = *var_uc_depvdsef1_dn7_slot;
        let mut var_uc_depvdsef1_dn8: f64 = *var_uc_depvdsef1_dn8_slot;
        let mut var_uc_depvdsef1_dn9: f64 = *var_uc_depvdsef1_dn9_slot;
        let mut var_uc_depvdsef1_rv: f64 = *var_uc_depvdsef1_rv_slot;
        let mut var_uc_depvdsef2: f64 = *var_uc_depvdsef2_slot;
        let mut var_uc_depvdsef2_dn0: f64 = *var_uc_depvdsef2_dn0_slot;
        let mut var_uc_depvdsef2_dn10: f64 = *var_uc_depvdsef2_dn10_slot;
        let mut var_uc_depvdsef2_dn11: f64 = *var_uc_depvdsef2_dn11_slot;
        let mut var_uc_depvdsef2_dn14: f64 = *var_uc_depvdsef2_dn14_slot;
        let mut var_uc_depvdsef2_dn2: f64 = *var_uc_depvdsef2_dn2_slot;
        let mut var_uc_depvdsef2_dn4: f64 = *var_uc_depvdsef2_dn4_slot;
        let mut var_uc_depvdsef2_dn5: f64 = *var_uc_depvdsef2_dn5_slot;
        let mut var_uc_depvdsef2_dn6: f64 = *var_uc_depvdsef2_dn6_slot;
        let mut var_uc_depvdsef2_dn7: f64 = *var_uc_depvdsef2_dn7_slot;
        let mut var_uc_depvdsef2_dn8: f64 = *var_uc_depvdsef2_dn8_slot;
        let mut var_uc_depvdsef2_dn9: f64 = *var_uc_depvdsef2_dn9_slot;
        let mut var_uc_depvdsef2_rv: f64 = *var_uc_depvdsef2_rv_slot;
        let mut var_uc_depvmax: f64 = *var_uc_depvmax_slot;
        let mut var_uc_depvmax_dn0: f64 = *var_uc_depvmax_dn0_slot;
        let mut var_uc_depvmax_dn10: f64 = *var_uc_depvmax_dn10_slot;
        let mut var_uc_depvmax_dn11: f64 = *var_uc_depvmax_dn11_slot;
        let mut var_uc_depvmax_dn14: f64 = *var_uc_depvmax_dn14_slot;
        let mut var_uc_depvmax_dn2: f64 = *var_uc_depvmax_dn2_slot;
        let mut var_uc_depvmax_dn4: f64 = *var_uc_depvmax_dn4_slot;
        let mut var_uc_depvmax_dn5: f64 = *var_uc_depvmax_dn5_slot;
        let mut var_uc_depvmax_dn6: f64 = *var_uc_depvmax_dn6_slot;
        let mut var_uc_depvmax_dn7: f64 = *var_uc_depvmax_dn7_slot;
        let mut var_uc_depvmax_dn8: f64 = *var_uc_depvmax_dn8_slot;
        let mut var_uc_depvmax_dn9: f64 = *var_uc_depvmax_dn9_slot;
        let mut var_uc_depvmax_rv: f64 = *var_uc_depvmax_rv_slot;
        let mut var_uc_ndepm: f64 = *var_uc_ndepm_slot;
        let mut var_uc_ndepm_dn0: f64 = *var_uc_ndepm_dn0_slot;
        let mut var_uc_ndepm_dn10: f64 = *var_uc_ndepm_dn10_slot;
        let mut var_uc_ndepm_dn11: f64 = *var_uc_ndepm_dn11_slot;
        let mut var_uc_ndepm_dn14: f64 = *var_uc_ndepm_dn14_slot;
        let mut var_uc_ndepm_dn2: f64 = *var_uc_ndepm_dn2_slot;
        let mut var_uc_ndepm_dn4: f64 = *var_uc_ndepm_dn4_slot;
        let mut var_uc_ndepm_dn5: f64 = *var_uc_ndepm_dn5_slot;
        let mut var_uc_ndepm_dn6: f64 = *var_uc_ndepm_dn6_slot;
        let mut var_uc_ndepm_dn7: f64 = *var_uc_ndepm_dn7_slot;
        let mut var_uc_ndepm_dn8: f64 = *var_uc_ndepm_dn8_slot;
        let mut var_uc_ndepm_dn9: f64 = *var_uc_ndepm_dn9_slot;
        let mut var_uc_ndepm_rv: f64 = *var_uc_ndepm_rv_slot;
        let mut var_uc_xpdv: f64 = *var_uc_xpdv_slot;
        let mut var_uc_xpdv_rv: f64 = *var_uc_xpdv_rv_slot;

        let (assign9480_e4872, assign9480_e4872_d_n0, assign9480_e4872_d_n2, assign9480_e4872_d_n4, assign9480_e4872_d_n5, assign9480_e4872_d_n6, assign9480_e4872_d_n7, assign9480_e4872_d_n8, assign9480_e4872_d_n9, assign9480_e4872_d_n10, assign9480_e4872_d_n11, assign9480_e4872_d_n14,) = {
    if (var_guard187 != 0.0) {
        let assign9480_e4870: f64 = (var_lg).powf(p.p373);
        (assign9480_e4870, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn14,)
    }
};
        var_t3 = assign9480_e4872;
        var_t3_dn0 = assign9480_e4872_d_n0;
        var_t3_dn2 = assign9480_e4872_d_n2;
        var_t3_dn4 = assign9480_e4872_d_n4;
        var_t3_dn5 = assign9480_e4872_d_n5;
        var_t3_dn6 = assign9480_e4872_d_n6;
        var_t3_dn7 = assign9480_e4872_d_n7;
        var_t3_dn8 = assign9480_e4872_d_n8;
        var_t3_dn9 = assign9480_e4872_d_n9;
        var_t3_dn10 = assign9480_e4872_d_n10;
        var_t3_dn11 = assign9480_e4872_d_n11;
        var_t3_dn14 = assign9480_e4872_d_n14;
        var_t3_rv = 0.0;

        let (assign9490_e4882, assign9490_e4882_d_n0, assign9490_e4882_d_n2, assign9490_e4882_d_n4, assign9490_e4882_d_n5, assign9490_e4882_d_n6, assign9490_e4882_d_n7, assign9490_e4882_d_n8, assign9490_e4882_d_n9, assign9490_e4882_d_n10, assign9490_e4882_d_n11, assign9490_e4882_d_n14,) = {
    if (var_guard187 != 0.0) {
        let assign9490_e4878: f64 = (p.p372 / var_t3);
        let assign9490_e4879: f64 = (1.0 + assign9490_e4878);
        let assign9490_e4880: f64 = (var_uc_depvdsef1 * assign9490_e4879);
        (assign9490_e4880, ((var_uc_depvdsef1_dn0 * assign9490_e4879) + (var_uc_depvdsef1 * (-((p.p372 * var_t3_dn0) / (var_t3 * var_t3))))), ((var_uc_depvdsef1_dn2 * assign9490_e4879) + (var_uc_depvdsef1 * (-((p.p372 * var_t3_dn2) / (var_t3 * var_t3))))), ((var_uc_depvdsef1_dn4 * assign9490_e4879) + (var_uc_depvdsef1 * (-((p.p372 * var_t3_dn4) / (var_t3 * var_t3))))), ((var_uc_depvdsef1_dn5 * assign9490_e4879) + (var_uc_depvdsef1 * (-((p.p372 * var_t3_dn5) / (var_t3 * var_t3))))), ((var_uc_depvdsef1_dn6 * assign9490_e4879) + (var_uc_depvdsef1 * (-((p.p372 * var_t3_dn6) / (var_t3 * var_t3))))), ((var_uc_depvdsef1_dn7 * assign9490_e4879) + (var_uc_depvdsef1 * (-((p.p372 * var_t3_dn7) / (var_t3 * var_t3))))), ((var_uc_depvdsef1_dn8 * assign9490_e4879) + (var_uc_depvdsef1 * (-((p.p372 * var_t3_dn8) / (var_t3 * var_t3))))), ((var_uc_depvdsef1_dn9 * assign9490_e4879) + (var_uc_depvdsef1 * (-((p.p372 * var_t3_dn9) / (var_t3 * var_t3))))), ((var_uc_depvdsef1_dn10 * assign9490_e4879) + (var_uc_depvdsef1 * (-((p.p372 * var_t3_dn10) / (var_t3 * var_t3))))), ((var_uc_depvdsef1_dn11 * assign9490_e4879) + (var_uc_depvdsef1 * (-((p.p372 * var_t3_dn11) / (var_t3 * var_t3))))), ((var_uc_depvdsef1_dn14 * assign9490_e4879) + (var_uc_depvdsef1 * (-((p.p372 * var_t3_dn14) / (var_t3 * var_t3))))),)
    } else {
        (var_uc_depvdsef1, var_uc_depvdsef1_dn0, var_uc_depvdsef1_dn2, var_uc_depvdsef1_dn4, var_uc_depvdsef1_dn5, var_uc_depvdsef1_dn6, var_uc_depvdsef1_dn7, var_uc_depvdsef1_dn8, var_uc_depvdsef1_dn9, var_uc_depvdsef1_dn10, var_uc_depvdsef1_dn11, var_uc_depvdsef1_dn14,)
    }
};
        var_uc_depvdsef1 = assign9490_e4882;
        var_uc_depvdsef1_dn0 = assign9490_e4882_d_n0;
        var_uc_depvdsef1_dn2 = assign9490_e4882_d_n2;
        var_uc_depvdsef1_dn4 = assign9490_e4882_d_n4;
        var_uc_depvdsef1_dn5 = assign9490_e4882_d_n5;
        var_uc_depvdsef1_dn6 = assign9490_e4882_d_n6;
        var_uc_depvdsef1_dn7 = assign9490_e4882_d_n7;
        var_uc_depvdsef1_dn8 = assign9490_e4882_d_n8;
        var_uc_depvdsef1_dn9 = assign9490_e4882_d_n9;
        var_uc_depvdsef1_dn10 = assign9490_e4882_d_n10;
        var_uc_depvdsef1_dn11 = assign9490_e4882_d_n11;
        var_uc_depvdsef1_dn14 = assign9490_e4882_d_n14;
        var_uc_depvdsef1_rv = 0.0;

        let (assign9500_e4888, assign9500_e4888_d_n0, assign9500_e4888_d_n2, assign9500_e4888_d_n4, assign9500_e4888_d_n5, assign9500_e4888_d_n6, assign9500_e4888_d_n7, assign9500_e4888_d_n8, assign9500_e4888_d_n9, assign9500_e4888_d_n10, assign9500_e4888_d_n11, assign9500_e4888_d_n14,) = {
    if (var_guard187 != 0.0) {
        let assign9500_e4886: f64 = (var_lg).powf(p.p375);
        (assign9500_e4886, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn14,)
    }
};
        var_t3 = assign9500_e4888;
        var_t3_dn0 = assign9500_e4888_d_n0;
        var_t3_dn2 = assign9500_e4888_d_n2;
        var_t3_dn4 = assign9500_e4888_d_n4;
        var_t3_dn5 = assign9500_e4888_d_n5;
        var_t3_dn6 = assign9500_e4888_d_n6;
        var_t3_dn7 = assign9500_e4888_d_n7;
        var_t3_dn8 = assign9500_e4888_d_n8;
        var_t3_dn9 = assign9500_e4888_d_n9;
        var_t3_dn10 = assign9500_e4888_d_n10;
        var_t3_dn11 = assign9500_e4888_d_n11;
        var_t3_dn14 = assign9500_e4888_d_n14;
        var_t3_rv = 0.0;

        let (assign9510_e4898, assign9510_e4898_d_n0, assign9510_e4898_d_n2, assign9510_e4898_d_n4, assign9510_e4898_d_n5, assign9510_e4898_d_n6, assign9510_e4898_d_n7, assign9510_e4898_d_n8, assign9510_e4898_d_n9, assign9510_e4898_d_n10, assign9510_e4898_d_n11, assign9510_e4898_d_n14,) = {
    if (var_guard187 != 0.0) {
        let assign9510_e4894: f64 = (p.p374 / var_t3);
        let assign9510_e4895: f64 = (1.0 + assign9510_e4894);
        let assign9510_e4896: f64 = (var_uc_depvdsef2 * assign9510_e4895);
        (assign9510_e4896, ((var_uc_depvdsef2_dn0 * assign9510_e4895) + (var_uc_depvdsef2 * (-((p.p374 * var_t3_dn0) / (var_t3 * var_t3))))), ((var_uc_depvdsef2_dn2 * assign9510_e4895) + (var_uc_depvdsef2 * (-((p.p374 * var_t3_dn2) / (var_t3 * var_t3))))), ((var_uc_depvdsef2_dn4 * assign9510_e4895) + (var_uc_depvdsef2 * (-((p.p374 * var_t3_dn4) / (var_t3 * var_t3))))), ((var_uc_depvdsef2_dn5 * assign9510_e4895) + (var_uc_depvdsef2 * (-((p.p374 * var_t3_dn5) / (var_t3 * var_t3))))), ((var_uc_depvdsef2_dn6 * assign9510_e4895) + (var_uc_depvdsef2 * (-((p.p374 * var_t3_dn6) / (var_t3 * var_t3))))), ((var_uc_depvdsef2_dn7 * assign9510_e4895) + (var_uc_depvdsef2 * (-((p.p374 * var_t3_dn7) / (var_t3 * var_t3))))), ((var_uc_depvdsef2_dn8 * assign9510_e4895) + (var_uc_depvdsef2 * (-((p.p374 * var_t3_dn8) / (var_t3 * var_t3))))), ((var_uc_depvdsef2_dn9 * assign9510_e4895) + (var_uc_depvdsef2 * (-((p.p374 * var_t3_dn9) / (var_t3 * var_t3))))), ((var_uc_depvdsef2_dn10 * assign9510_e4895) + (var_uc_depvdsef2 * (-((p.p374 * var_t3_dn10) / (var_t3 * var_t3))))), ((var_uc_depvdsef2_dn11 * assign9510_e4895) + (var_uc_depvdsef2 * (-((p.p374 * var_t3_dn11) / (var_t3 * var_t3))))), ((var_uc_depvdsef2_dn14 * assign9510_e4895) + (var_uc_depvdsef2 * (-((p.p374 * var_t3_dn14) / (var_t3 * var_t3))))),)
    } else {
        (var_uc_depvdsef2, var_uc_depvdsef2_dn0, var_uc_depvdsef2_dn2, var_uc_depvdsef2_dn4, var_uc_depvdsef2_dn5, var_uc_depvdsef2_dn6, var_uc_depvdsef2_dn7, var_uc_depvdsef2_dn8, var_uc_depvdsef2_dn9, var_uc_depvdsef2_dn10, var_uc_depvdsef2_dn11, var_uc_depvdsef2_dn14,)
    }
};
        var_uc_depvdsef2 = assign9510_e4898;
        var_uc_depvdsef2_dn0 = assign9510_e4898_d_n0;
        var_uc_depvdsef2_dn2 = assign9510_e4898_d_n2;
        var_uc_depvdsef2_dn4 = assign9510_e4898_d_n4;
        var_uc_depvdsef2_dn5 = assign9510_e4898_d_n5;
        var_uc_depvdsef2_dn6 = assign9510_e4898_d_n6;
        var_uc_depvdsef2_dn7 = assign9510_e4898_d_n7;
        var_uc_depvdsef2_dn8 = assign9510_e4898_d_n8;
        var_uc_depvdsef2_dn9 = assign9510_e4898_d_n9;
        var_uc_depvdsef2_dn10 = assign9510_e4898_d_n10;
        var_uc_depvdsef2_dn11 = assign9510_e4898_d_n11;
        var_uc_depvdsef2_dn14 = assign9510_e4898_d_n14;
        var_uc_depvdsef2_rv = 0.0;

        let assign9520_e4901: f64 = if var_uc_depvdsef2 < 0.1 { 1.0 } else { 0.0 };
        var_guard194 = assign9520_e4901;
        var_guard194_rv = 0.0;

        let (assign9530_e4907, assign9530_e4907_d_n0, assign9530_e4907_d_n2, assign9530_e4907_d_n4, assign9530_e4907_d_n5, assign9530_e4907_d_n6, assign9530_e4907_d_n7, assign9530_e4907_d_n8, assign9530_e4907_d_n9, assign9530_e4907_d_n10, assign9530_e4907_d_n11, assign9530_e4907_d_n14,) = {
    if ((var_guard187 != 0.0) && (var_guard194 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depvdsef2, var_uc_depvdsef2_dn0, var_uc_depvdsef2_dn2, var_uc_depvdsef2_dn4, var_uc_depvdsef2_dn5, var_uc_depvdsef2_dn6, var_uc_depvdsef2_dn7, var_uc_depvdsef2_dn8, var_uc_depvdsef2_dn9, var_uc_depvdsef2_dn10, var_uc_depvdsef2_dn11, var_uc_depvdsef2_dn14,)
    }
};
        var_uc_depvdsef2 = assign9530_e4907;
        var_uc_depvdsef2_dn0 = assign9530_e4907_d_n0;
        var_uc_depvdsef2_dn2 = assign9530_e4907_d_n2;
        var_uc_depvdsef2_dn4 = assign9530_e4907_d_n4;
        var_uc_depvdsef2_dn5 = assign9530_e4907_d_n5;
        var_uc_depvdsef2_dn6 = assign9530_e4907_d_n6;
        var_uc_depvdsef2_dn7 = assign9530_e4907_d_n7;
        var_uc_depvdsef2_dn8 = assign9530_e4907_d_n8;
        var_uc_depvdsef2_dn9 = assign9530_e4907_d_n9;
        var_uc_depvdsef2_dn10 = assign9530_e4907_d_n10;
        var_uc_depvdsef2_dn11 = assign9530_e4907_d_n11;
        var_uc_depvdsef2_dn14 = assign9530_e4907_d_n14;
        var_uc_depvdsef2_rv = 0.0;

        let (assign9540_e4912, assign9540_e4912_d_n0, assign9540_e4912_d_n2, assign9540_e4912_d_n4, assign9540_e4912_d_n5, assign9540_e4912_d_n6, assign9540_e4912_d_n7, assign9540_e4912_d_n8, assign9540_e4912_d_n9, assign9540_e4912_d_n10, assign9540_e4912_d_n11, assign9540_e4912_d_n14,) = {
    if (var_guard187 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_ndepm, var_uc_ndepm_dn0, var_uc_ndepm_dn2, var_uc_ndepm_dn4, var_uc_ndepm_dn5, var_uc_ndepm_dn6, var_uc_ndepm_dn7, var_uc_ndepm_dn8, var_uc_ndepm_dn9, var_uc_ndepm_dn10, var_uc_ndepm_dn11, var_uc_ndepm_dn14,)
    }
};
        var_uc_ndepm = assign9540_e4912;
        var_uc_ndepm_dn0 = assign9540_e4912_d_n0;
        var_uc_ndepm_dn2 = assign9540_e4912_d_n2;
        var_uc_ndepm_dn4 = assign9540_e4912_d_n4;
        var_uc_ndepm_dn5 = assign9540_e4912_d_n5;
        var_uc_ndepm_dn6 = assign9540_e4912_d_n6;
        var_uc_ndepm_dn7 = assign9540_e4912_d_n7;
        var_uc_ndepm_dn8 = assign9540_e4912_d_n8;
        var_uc_ndepm_dn9 = assign9540_e4912_d_n9;
        var_uc_ndepm_dn10 = assign9540_e4912_d_n10;
        var_uc_ndepm_dn11 = assign9540_e4912_d_n11;
        var_uc_ndepm_dn14 = assign9540_e4912_d_n14;
        var_uc_ndepm_rv = 0.0;

        let (assign9550_e4917, assign9550_e4917_d_n0, assign9550_e4917_d_n2, assign9550_e4917_d_n4, assign9550_e4917_d_n5, assign9550_e4917_d_n6, assign9550_e4917_d_n7, assign9550_e4917_d_n8, assign9550_e4917_d_n9, assign9550_e4917_d_n10, assign9550_e4917_d_n11, assign9550_e4917_d_n14,) = {
    if (var_guard187 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depvmax, var_uc_depvmax_dn0, var_uc_depvmax_dn2, var_uc_depvmax_dn4, var_uc_depvmax_dn5, var_uc_depvmax_dn6, var_uc_depvmax_dn7, var_uc_depvmax_dn8, var_uc_depvmax_dn9, var_uc_depvmax_dn10, var_uc_depvmax_dn11, var_uc_depvmax_dn14,)
    }
};
        var_uc_depvmax = assign9550_e4917;
        var_uc_depvmax_dn0 = assign9550_e4917_d_n0;
        var_uc_depvmax_dn2 = assign9550_e4917_d_n2;
        var_uc_depvmax_dn4 = assign9550_e4917_d_n4;
        var_uc_depvmax_dn5 = assign9550_e4917_d_n5;
        var_uc_depvmax_dn6 = assign9550_e4917_d_n6;
        var_uc_depvmax_dn7 = assign9550_e4917_d_n7;
        var_uc_depvmax_dn8 = assign9550_e4917_d_n8;
        var_uc_depvmax_dn9 = assign9550_e4917_d_n9;
        var_uc_depvmax_dn10 = assign9550_e4917_d_n10;
        var_uc_depvmax_dn11 = assign9550_e4917_d_n11;
        var_uc_depvmax_dn14 = assign9550_e4917_d_n14;
        var_uc_depvmax_rv = 0.0;

        let (assign9560_e4922, assign9560_e4922_d_n0, assign9560_e4922_d_n2, assign9560_e4922_d_n4, assign9560_e4922_d_n5, assign9560_e4922_d_n6, assign9560_e4922_d_n7, assign9560_e4922_d_n8, assign9560_e4922_d_n9, assign9560_e4922_d_n10, assign9560_e4922_d_n11, assign9560_e4922_d_n14,) = {
    if (var_guard187 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depleak, var_uc_depleak_dn0, var_uc_depleak_dn2, var_uc_depleak_dn4, var_uc_depleak_dn5, var_uc_depleak_dn6, var_uc_depleak_dn7, var_uc_depleak_dn8, var_uc_depleak_dn9, var_uc_depleak_dn10, var_uc_depleak_dn11, var_uc_depleak_dn14,)
    }
};
        var_uc_depleak = assign9560_e4922;
        var_uc_depleak_dn0 = assign9560_e4922_d_n0;
        var_uc_depleak_dn2 = assign9560_e4922_d_n2;
        var_uc_depleak_dn4 = assign9560_e4922_d_n4;
        var_uc_depleak_dn5 = assign9560_e4922_d_n5;
        var_uc_depleak_dn6 = assign9560_e4922_d_n6;
        var_uc_depleak_dn7 = assign9560_e4922_d_n7;
        var_uc_depleak_dn8 = assign9560_e4922_d_n8;
        var_uc_depleak_dn9 = assign9560_e4922_d_n9;
        var_uc_depleak_dn10 = assign9560_e4922_d_n10;
        var_uc_depleak_dn11 = assign9560_e4922_d_n11;
        var_uc_depleak_dn14 = assign9560_e4922_d_n14;
        var_uc_depleak_rv = 0.0;

        let (assign9570_e4927, assign9570_e4927_d_n0, assign9570_e4927_d_n2, assign9570_e4927_d_n4, assign9570_e4927_d_n5, assign9570_e4927_d_n6, assign9570_e4927_d_n7, assign9570_e4927_d_n8, assign9570_e4927_d_n9, assign9570_e4927_d_n10, assign9570_e4927_d_n11, assign9570_e4927_d_n14,) = {
    if (var_guard187 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depmue0, var_uc_depmue0_dn0, var_uc_depmue0_dn2, var_uc_depmue0_dn4, var_uc_depmue0_dn5, var_uc_depmue0_dn6, var_uc_depmue0_dn7, var_uc_depmue0_dn8, var_uc_depmue0_dn9, var_uc_depmue0_dn10, var_uc_depmue0_dn11, var_uc_depmue0_dn14,)
    }
};
        var_uc_depmue0 = assign9570_e4927;
        var_uc_depmue0_dn0 = assign9570_e4927_d_n0;
        var_uc_depmue0_dn2 = assign9570_e4927_d_n2;
        var_uc_depmue0_dn4 = assign9570_e4927_d_n4;
        var_uc_depmue0_dn5 = assign9570_e4927_d_n5;
        var_uc_depmue0_dn6 = assign9570_e4927_d_n6;
        var_uc_depmue0_dn7 = assign9570_e4927_d_n7;
        var_uc_depmue0_dn8 = assign9570_e4927_d_n8;
        var_uc_depmue0_dn9 = assign9570_e4927_d_n9;
        var_uc_depmue0_dn10 = assign9570_e4927_d_n10;
        var_uc_depmue0_dn11 = assign9570_e4927_d_n11;
        var_uc_depmue0_dn14 = assign9570_e4927_d_n14;
        var_uc_depmue0_rv = 0.0;

        let (assign9580_e4932, assign9580_e4932_d_n0, assign9580_e4932_d_n2, assign9580_e4932_d_n4, assign9580_e4932_d_n5, assign9580_e4932_d_n6, assign9580_e4932_d_n7, assign9580_e4932_d_n8, assign9580_e4932_d_n9, assign9580_e4932_d_n10, assign9580_e4932_d_n11, assign9580_e4932_d_n14,) = {
    if (var_guard187 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depmue1, var_uc_depmue1_dn0, var_uc_depmue1_dn2, var_uc_depmue1_dn4, var_uc_depmue1_dn5, var_uc_depmue1_dn6, var_uc_depmue1_dn7, var_uc_depmue1_dn8, var_uc_depmue1_dn9, var_uc_depmue1_dn10, var_uc_depmue1_dn11, var_uc_depmue1_dn14,)
    }
};
        var_uc_depmue1 = assign9580_e4932;
        var_uc_depmue1_dn0 = assign9580_e4932_d_n0;
        var_uc_depmue1_dn2 = assign9580_e4932_d_n2;
        var_uc_depmue1_dn4 = assign9580_e4932_d_n4;
        var_uc_depmue1_dn5 = assign9580_e4932_d_n5;
        var_uc_depmue1_dn6 = assign9580_e4932_d_n6;
        var_uc_depmue1_dn7 = assign9580_e4932_d_n7;
        var_uc_depmue1_dn8 = assign9580_e4932_d_n8;
        var_uc_depmue1_dn9 = assign9580_e4932_d_n9;
        var_uc_depmue1_dn10 = assign9580_e4932_d_n10;
        var_uc_depmue1_dn11 = assign9580_e4932_d_n11;
        var_uc_depmue1_dn14 = assign9580_e4932_d_n14;
        var_uc_depmue1_rv = 0.0;

        let (assign9590_e4937, assign9590_e4937_d_n0, assign9590_e4937_d_n2, assign9590_e4937_d_n4, assign9590_e4937_d_n5, assign9590_e4937_d_n6, assign9590_e4937_d_n7, assign9590_e4937_d_n8, assign9590_e4937_d_n9, assign9590_e4937_d_n10, assign9590_e4937_d_n11, assign9590_e4937_d_n14,) = {
    if (var_guard187 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depmueback0, var_uc_depmueback0_dn0, var_uc_depmueback0_dn2, var_uc_depmueback0_dn4, var_uc_depmueback0_dn5, var_uc_depmueback0_dn6, var_uc_depmueback0_dn7, var_uc_depmueback0_dn8, var_uc_depmueback0_dn9, var_uc_depmueback0_dn10, var_uc_depmueback0_dn11, var_uc_depmueback0_dn14,)
    }
};
        var_uc_depmueback0 = assign9590_e4937;
        var_uc_depmueback0_dn0 = assign9590_e4937_d_n0;
        var_uc_depmueback0_dn2 = assign9590_e4937_d_n2;
        var_uc_depmueback0_dn4 = assign9590_e4937_d_n4;
        var_uc_depmueback0_dn5 = assign9590_e4937_d_n5;
        var_uc_depmueback0_dn6 = assign9590_e4937_d_n6;
        var_uc_depmueback0_dn7 = assign9590_e4937_d_n7;
        var_uc_depmueback0_dn8 = assign9590_e4937_d_n8;
        var_uc_depmueback0_dn9 = assign9590_e4937_d_n9;
        var_uc_depmueback0_dn10 = assign9590_e4937_d_n10;
        var_uc_depmueback0_dn11 = assign9590_e4937_d_n11;
        var_uc_depmueback0_dn14 = assign9590_e4937_d_n14;
        var_uc_depmueback0_rv = 0.0;

        let (assign9600_e4942, assign9600_e4942_d_n0, assign9600_e4942_d_n2, assign9600_e4942_d_n4, assign9600_e4942_d_n5, assign9600_e4942_d_n6, assign9600_e4942_d_n7, assign9600_e4942_d_n8, assign9600_e4942_d_n9, assign9600_e4942_d_n10, assign9600_e4942_d_n11, assign9600_e4942_d_n14,) = {
    if (var_guard187 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depmueback1, var_uc_depmueback1_dn0, var_uc_depmueback1_dn2, var_uc_depmueback1_dn4, var_uc_depmueback1_dn5, var_uc_depmueback1_dn6, var_uc_depmueback1_dn7, var_uc_depmueback1_dn8, var_uc_depmueback1_dn9, var_uc_depmueback1_dn10, var_uc_depmueback1_dn11, var_uc_depmueback1_dn14,)
    }
};
        var_uc_depmueback1 = assign9600_e4942;
        var_uc_depmueback1_dn0 = assign9600_e4942_d_n0;
        var_uc_depmueback1_dn2 = assign9600_e4942_d_n2;
        var_uc_depmueback1_dn4 = assign9600_e4942_d_n4;
        var_uc_depmueback1_dn5 = assign9600_e4942_d_n5;
        var_uc_depmueback1_dn6 = assign9600_e4942_d_n6;
        var_uc_depmueback1_dn7 = assign9600_e4942_d_n7;
        var_uc_depmueback1_dn8 = assign9600_e4942_d_n8;
        var_uc_depmueback1_dn9 = assign9600_e4942_d_n9;
        var_uc_depmueback1_dn10 = assign9600_e4942_d_n10;
        var_uc_depmueback1_dn11 = assign9600_e4942_d_n11;
        var_uc_depmueback1_dn14 = assign9600_e4942_d_n14;
        var_uc_depmueback1_rv = 0.0;

        let (assign9610_e4947, assign9610_e4947_d_n0, assign9610_e4947_d_n2, assign9610_e4947_d_n4, assign9610_e4947_d_n5, assign9610_e4947_d_n6, assign9610_e4947_d_n7, assign9610_e4947_d_n8, assign9610_e4947_d_n9, assign9610_e4947_d_n10, assign9610_e4947_d_n11, assign9610_e4947_d_n14,) = {
    if (var_guard187 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depvdsef1, var_uc_depvdsef1_dn0, var_uc_depvdsef1_dn2, var_uc_depvdsef1_dn4, var_uc_depvdsef1_dn5, var_uc_depvdsef1_dn6, var_uc_depvdsef1_dn7, var_uc_depvdsef1_dn8, var_uc_depvdsef1_dn9, var_uc_depvdsef1_dn10, var_uc_depvdsef1_dn11, var_uc_depvdsef1_dn14,)
    }
};
        var_uc_depvdsef1 = assign9610_e4947;
        var_uc_depvdsef1_dn0 = assign9610_e4947_d_n0;
        var_uc_depvdsef1_dn2 = assign9610_e4947_d_n2;
        var_uc_depvdsef1_dn4 = assign9610_e4947_d_n4;
        var_uc_depvdsef1_dn5 = assign9610_e4947_d_n5;
        var_uc_depvdsef1_dn6 = assign9610_e4947_d_n6;
        var_uc_depvdsef1_dn7 = assign9610_e4947_d_n7;
        var_uc_depvdsef1_dn8 = assign9610_e4947_d_n8;
        var_uc_depvdsef1_dn9 = assign9610_e4947_d_n9;
        var_uc_depvdsef1_dn10 = assign9610_e4947_d_n10;
        var_uc_depvdsef1_dn11 = assign9610_e4947_d_n11;
        var_uc_depvdsef1_dn14 = assign9610_e4947_d_n14;
        var_uc_depvdsef1_rv = 0.0;

        let (assign9620_e4952, assign9620_e4952_d_n0, assign9620_e4952_d_n2, assign9620_e4952_d_n4, assign9620_e4952_d_n5, assign9620_e4952_d_n6, assign9620_e4952_d_n7, assign9620_e4952_d_n8, assign9620_e4952_d_n9, assign9620_e4952_d_n10, assign9620_e4952_d_n11, assign9620_e4952_d_n14,) = {
    if (var_guard187 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_uc_depvdsef2, var_uc_depvdsef2_dn0, var_uc_depvdsef2_dn2, var_uc_depvdsef2_dn4, var_uc_depvdsef2_dn5, var_uc_depvdsef2_dn6, var_uc_depvdsef2_dn7, var_uc_depvdsef2_dn8, var_uc_depvdsef2_dn9, var_uc_depvdsef2_dn10, var_uc_depvdsef2_dn11, var_uc_depvdsef2_dn14,)
    }
};
        var_uc_depvdsef2 = assign9620_e4952;
        var_uc_depvdsef2_dn0 = assign9620_e4952_d_n0;
        var_uc_depvdsef2_dn2 = assign9620_e4952_d_n2;
        var_uc_depvdsef2_dn4 = assign9620_e4952_d_n4;
        var_uc_depvdsef2_dn5 = assign9620_e4952_d_n5;
        var_uc_depvdsef2_dn6 = assign9620_e4952_d_n6;
        var_uc_depvdsef2_dn7 = assign9620_e4952_d_n7;
        var_uc_depvdsef2_dn8 = assign9620_e4952_d_n8;
        var_uc_depvdsef2_dn9 = assign9620_e4952_d_n9;
        var_uc_depvdsef2_dn10 = assign9620_e4952_d_n10;
        var_uc_depvdsef2_dn11 = assign9620_e4952_d_n11;
        var_uc_depvdsef2_dn14 = assign9620_e4952_d_n14;
        var_uc_depvdsef2_rv = 0.0;

        let assign10140_e5325: f64 = (var_uc_xpdv * var_uc_xldld);
        let assign10140_e5327: f64 = if assign10140_e5325 > 1.0 { 1.0 } else { 0.0 };
        var_guard246 = assign10140_e5327;
        var_guard246_rv = 0.0;

        let (assign10150_e5333,) = {
    if (var_guard246 != 0.0) {
        let assign10150_e5331: f64 = (1.0 / var_uc_xldld);
        (assign10150_e5331,)
    } else {
        (var_uc_xpdv,)
    }
};
        var_uc_xpdv = assign10150_e5333;
        var_uc_xpdv_rv = 0.0;

        let assign10170_e5361: f64 = if ((p.p40 == 1.0) && (((p.p19 > 0.0) && (var_uc_nover == 0.0)) || ((p.p18 > 0.0) && (var_uc_novers == 0.0)))) { 1.0 } else { 0.0 };
        var_guard248 = assign10170_e5361;
        var_guard248_rv = 0.0;

        let (assign10180_e5365,) = {
    if (var_guard248 != 0.0) {
        (0.0,)
    } else {
        (var_uc_cordrift,)
    }
};
        var_uc_cordrift = assign10180_e5365;
        var_uc_cordrift_rv = 0.0;

        let (assign10190_e5370,) = {
    if (var_guard248 == 0.0) {
        (p.p40,)
    } else {
        (var_uc_cordrift,)
    }
};
        var_uc_cordrift = assign10190_e5370;
        var_uc_cordrift_rv = 0.0;

        let assign10200_e5373: f64 = if var_uc_cordrift == 1.0 { 1.0 } else { 0.0 };
        var_guard249 = assign10200_e5373;
        var_guard249_rv = 0.0;

        let (assign10210_e5382,) = {
    if (var_guard249 != 0.0) {
        let (assign10210_e5380,) = {
            if (p.p19 > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10210_e5380,)
    } else {
        (var_flg_rd,)
    }
};
        var_flg_rd = assign10210_e5382;
        var_flg_rd_rv = 0.0;

        let (assign10220_e5391,) = {
    if (var_guard249 != 0.0) {
        let (assign10220_e5389,) = {
            if (p.p18 > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10220_e5389,)
    } else {
        (var_flg_rs,)
    }
};
        var_flg_rs = assign10220_e5391;
        var_flg_rs_rv = 0.0;

        let assign10230_e5398: f64 = if ((p.p17 == 0.0) || (p.p17 == 2.0)) { 1.0 } else { 0.0 };
        var_guard250 = assign10230_e5398;
        var_guard250_rv = 0.0;

        let (assign10240_e5405,) = {
    if ((var_guard249 == 0.0) && (var_guard250 != 0.0)) {
        (0.0,)
    } else {
        (var_flg_rd,)
    }
};
        var_flg_rd = assign10240_e5405;
        var_flg_rd_rv = 0.0;

        let (assign10250_e5412,) = {
    if ((var_guard249 == 0.0) && (var_guard250 != 0.0)) {
        (0.0,)
    } else {
        (var_flg_rs,)
    }
};
        var_flg_rs = assign10250_e5412;
        var_flg_rs_rv = 0.0;

        let (assign10260_e5444, assign10260_e5444_d_n0, assign10260_e5444_d_n2, assign10260_e5444_d_n4, assign10260_e5444_d_n5, assign10260_e5444_d_n6, assign10260_e5444_d_n7, assign10260_e5444_d_n8, assign10260_e5444_d_n9, assign10260_e5444_d_n10, assign10260_e5444_d_n11, assign10260_e5444_d_n14,) = {
    if ((var_guard249 == 0.0) && (var_guard250 == 0.0)) {
        let assign10260_e5420: f64 = (p.p130 * p.p2);
        let assign10260_e5422: f64 = (assign10260_e5420 * p.p7);
        let assign10260_e5425: f64 = (var_uc_rd + var_uc_rdvd);
        let assign10260_e5428: f64 = (p.p67 * var_uc_rdslp1);
        let assign10260_e5430: f64 = (assign10260_e5428 * 1000000.0);
        let assign10260_e5432: f64 = (assign10260_e5430 + var_uc_rdict1);
        let assign10260_e5433: f64 = (assign10260_e5425 * assign10260_e5432);
        let assign10260_e5436: f64 = (p.p68 * p.p100);
        let assign10260_e5438: f64 = (assign10260_e5436 * 1000000.0);
        let assign10260_e5440: f64 = (assign10260_e5438 + p.p101);
        let assign10260_e5441: f64 = (assign10260_e5433 * assign10260_e5440);
        let assign10260_e5442: f64 = (assign10260_e5422 + assign10260_e5441);
        (assign10260_e5442, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign10260_e5444;
        var_t1_dn0 = assign10260_e5444_d_n0;
        var_t1_dn2 = assign10260_e5444_d_n2;
        var_t1_dn4 = assign10260_e5444_d_n4;
        var_t1_dn5 = assign10260_e5444_d_n5;
        var_t1_dn6 = assign10260_e5444_d_n6;
        var_t1_dn7 = assign10260_e5444_d_n7;
        var_t1_dn8 = assign10260_e5444_d_n8;
        var_t1_dn9 = assign10260_e5444_d_n9;
        var_t1_dn10 = assign10260_e5444_d_n10;
        var_t1_dn11 = assign10260_e5444_d_n11;
        var_t1_dn14 = assign10260_e5444_d_n14;
        var_t1_rv = 0.0;

        let (assign10270_e5457,) = {
    if ((var_guard249 == 0.0) && (var_guard250 == 0.0)) {
        let (assign10270_e5455,) = {
            if (var_t1 > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10270_e5455,)
    } else {
        (var_flg_rd,)
    }
};
        var_flg_rd = assign10270_e5457;
        var_flg_rd_rv = 0.0;

        let (assign10280_e5487, assign10280_e5487_d_n0, assign10280_e5487_d_n2, assign10280_e5487_d_n4, assign10280_e5487_d_n5, assign10280_e5487_d_n6, assign10280_e5487_d_n7, assign10280_e5487_d_n8, assign10280_e5487_d_n9, assign10280_e5487_d_n10, assign10280_e5487_d_n11, assign10280_e5487_d_n14,) = {
    if ((var_guard249 == 0.0) && (var_guard250 == 0.0)) {
        let assign10280_e5465: f64 = (p.p131 * p.p3);
        let assign10280_e5467: f64 = (assign10280_e5465 * p.p7);
        let assign10280_e5471: f64 = (p.p69 * var_uc_rdslp1);
        let assign10280_e5473: f64 = (assign10280_e5471 * 1000000.0);
        let assign10280_e5475: f64 = (assign10280_e5473 + var_uc_rdict1);
        let assign10280_e5476: f64 = (var_uc_rs * assign10280_e5475);
        let assign10280_e5479: f64 = (p.p70 * p.p100);
        let assign10280_e5481: f64 = (assign10280_e5479 * 1000000.0);
        let assign10280_e5483: f64 = (assign10280_e5481 + p.p101);
        let assign10280_e5484: f64 = (assign10280_e5476 * assign10280_e5483);
        let assign10280_e5485: f64 = (assign10280_e5467 + assign10280_e5484);
        (assign10280_e5485, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign10280_e5487;
        var_t1_dn0 = assign10280_e5487_d_n0;
        var_t1_dn2 = assign10280_e5487_d_n2;
        var_t1_dn4 = assign10280_e5487_d_n4;
        var_t1_dn5 = assign10280_e5487_d_n5;
        var_t1_dn6 = assign10280_e5487_d_n6;
        var_t1_dn7 = assign10280_e5487_d_n7;
        var_t1_dn8 = assign10280_e5487_d_n8;
        var_t1_dn9 = assign10280_e5487_d_n9;
        var_t1_dn10 = assign10280_e5487_d_n10;
        var_t1_dn11 = assign10280_e5487_d_n11;
        var_t1_dn14 = assign10280_e5487_d_n14;
        var_t1_rv = 0.0;

        let (assign10290_e5500,) = {
    if ((var_guard249 == 0.0) && (var_guard250 == 0.0)) {
        let (assign10290_e5498,) = {
            if (var_t1 > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10290_e5498,)
    } else {
        (var_flg_rs,)
    }
};
        var_flg_rs = assign10290_e5500;
        var_flg_rs_rv = 0.0;

        let assign10300_e5503: f64 = (p.p12 / 1e-6);
        var_mks_nsubcdfm = assign10300_e5503;
        var_mks_nsubcdfm_rv = 0.0;

        *var_flg_rd_slot = var_flg_rd;
        *var_flg_rd_rv_slot = var_flg_rd_rv;
        *var_flg_rs_slot = var_flg_rs;
        *var_flg_rs_rv_slot = var_flg_rs_rv;
        *var_guard194_slot = var_guard194;
        *var_guard194_rv_slot = var_guard194_rv;
        *var_guard246_slot = var_guard246;
        *var_guard246_rv_slot = var_guard246_rv;
        *var_guard248_slot = var_guard248;
        *var_guard248_rv_slot = var_guard248_rv;
        *var_guard249_slot = var_guard249;
        *var_guard249_rv_slot = var_guard249_rv;
        *var_guard250_slot = var_guard250;
        *var_guard250_rv_slot = var_guard250_rv;
        *var_mks_nsubcdfm_slot = var_mks_nsubcdfm;
        *var_mks_nsubcdfm_rv_slot = var_mks_nsubcdfm_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn14_slot = var_t3_dn14;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t3_rv_slot = var_t3_rv;
        *var_uc_cordrift_slot = var_uc_cordrift;
        *var_uc_cordrift_rv_slot = var_uc_cordrift_rv;
        *var_uc_depleak_slot = var_uc_depleak;
        *var_uc_depleak_dn0_slot = var_uc_depleak_dn0;
        *var_uc_depleak_dn10_slot = var_uc_depleak_dn10;
        *var_uc_depleak_dn11_slot = var_uc_depleak_dn11;
        *var_uc_depleak_dn14_slot = var_uc_depleak_dn14;
        *var_uc_depleak_dn2_slot = var_uc_depleak_dn2;
        *var_uc_depleak_dn4_slot = var_uc_depleak_dn4;
        *var_uc_depleak_dn5_slot = var_uc_depleak_dn5;
        *var_uc_depleak_dn6_slot = var_uc_depleak_dn6;
        *var_uc_depleak_dn7_slot = var_uc_depleak_dn7;
        *var_uc_depleak_dn8_slot = var_uc_depleak_dn8;
        *var_uc_depleak_dn9_slot = var_uc_depleak_dn9;
        *var_uc_depleak_rv_slot = var_uc_depleak_rv;
        *var_uc_depmue0_slot = var_uc_depmue0;
        *var_uc_depmue0_dn0_slot = var_uc_depmue0_dn0;
        *var_uc_depmue0_dn10_slot = var_uc_depmue0_dn10;
        *var_uc_depmue0_dn11_slot = var_uc_depmue0_dn11;
        *var_uc_depmue0_dn14_slot = var_uc_depmue0_dn14;
        *var_uc_depmue0_dn2_slot = var_uc_depmue0_dn2;
        *var_uc_depmue0_dn4_slot = var_uc_depmue0_dn4;
        *var_uc_depmue0_dn5_slot = var_uc_depmue0_dn5;
        *var_uc_depmue0_dn6_slot = var_uc_depmue0_dn6;
        *var_uc_depmue0_dn7_slot = var_uc_depmue0_dn7;
        *var_uc_depmue0_dn8_slot = var_uc_depmue0_dn8;
        *var_uc_depmue0_dn9_slot = var_uc_depmue0_dn9;
        *var_uc_depmue0_rv_slot = var_uc_depmue0_rv;
        *var_uc_depmue1_slot = var_uc_depmue1;
        *var_uc_depmue1_dn0_slot = var_uc_depmue1_dn0;
        *var_uc_depmue1_dn10_slot = var_uc_depmue1_dn10;
        *var_uc_depmue1_dn11_slot = var_uc_depmue1_dn11;
        *var_uc_depmue1_dn14_slot = var_uc_depmue1_dn14;
        *var_uc_depmue1_dn2_slot = var_uc_depmue1_dn2;
        *var_uc_depmue1_dn4_slot = var_uc_depmue1_dn4;
        *var_uc_depmue1_dn5_slot = var_uc_depmue1_dn5;
        *var_uc_depmue1_dn6_slot = var_uc_depmue1_dn6;
        *var_uc_depmue1_dn7_slot = var_uc_depmue1_dn7;
        *var_uc_depmue1_dn8_slot = var_uc_depmue1_dn8;
        *var_uc_depmue1_dn9_slot = var_uc_depmue1_dn9;
        *var_uc_depmue1_rv_slot = var_uc_depmue1_rv;
        *var_uc_depmueback0_slot = var_uc_depmueback0;
        *var_uc_depmueback0_dn0_slot = var_uc_depmueback0_dn0;
        *var_uc_depmueback0_dn10_slot = var_uc_depmueback0_dn10;
        *var_uc_depmueback0_dn11_slot = var_uc_depmueback0_dn11;
        *var_uc_depmueback0_dn14_slot = var_uc_depmueback0_dn14;
        *var_uc_depmueback0_dn2_slot = var_uc_depmueback0_dn2;
        *var_uc_depmueback0_dn4_slot = var_uc_depmueback0_dn4;
        *var_uc_depmueback0_dn5_slot = var_uc_depmueback0_dn5;
        *var_uc_depmueback0_dn6_slot = var_uc_depmueback0_dn6;
        *var_uc_depmueback0_dn7_slot = var_uc_depmueback0_dn7;
        *var_uc_depmueback0_dn8_slot = var_uc_depmueback0_dn8;
        *var_uc_depmueback0_dn9_slot = var_uc_depmueback0_dn9;
        *var_uc_depmueback0_rv_slot = var_uc_depmueback0_rv;
        *var_uc_depmueback1_slot = var_uc_depmueback1;
        *var_uc_depmueback1_dn0_slot = var_uc_depmueback1_dn0;
        *var_uc_depmueback1_dn10_slot = var_uc_depmueback1_dn10;
        *var_uc_depmueback1_dn11_slot = var_uc_depmueback1_dn11;
        *var_uc_depmueback1_dn14_slot = var_uc_depmueback1_dn14;
        *var_uc_depmueback1_dn2_slot = var_uc_depmueback1_dn2;
        *var_uc_depmueback1_dn4_slot = var_uc_depmueback1_dn4;
        *var_uc_depmueback1_dn5_slot = var_uc_depmueback1_dn5;
        *var_uc_depmueback1_dn6_slot = var_uc_depmueback1_dn6;
        *var_uc_depmueback1_dn7_slot = var_uc_depmueback1_dn7;
        *var_uc_depmueback1_dn8_slot = var_uc_depmueback1_dn8;
        *var_uc_depmueback1_dn9_slot = var_uc_depmueback1_dn9;
        *var_uc_depmueback1_rv_slot = var_uc_depmueback1_rv;
        *var_uc_depvdsef1_slot = var_uc_depvdsef1;
        *var_uc_depvdsef1_dn0_slot = var_uc_depvdsef1_dn0;
        *var_uc_depvdsef1_dn10_slot = var_uc_depvdsef1_dn10;
        *var_uc_depvdsef1_dn11_slot = var_uc_depvdsef1_dn11;
        *var_uc_depvdsef1_dn14_slot = var_uc_depvdsef1_dn14;
        *var_uc_depvdsef1_dn2_slot = var_uc_depvdsef1_dn2;
        *var_uc_depvdsef1_dn4_slot = var_uc_depvdsef1_dn4;
        *var_uc_depvdsef1_dn5_slot = var_uc_depvdsef1_dn5;
        *var_uc_depvdsef1_dn6_slot = var_uc_depvdsef1_dn6;
        *var_uc_depvdsef1_dn7_slot = var_uc_depvdsef1_dn7;
        *var_uc_depvdsef1_dn8_slot = var_uc_depvdsef1_dn8;
        *var_uc_depvdsef1_dn9_slot = var_uc_depvdsef1_dn9;
        *var_uc_depvdsef1_rv_slot = var_uc_depvdsef1_rv;
        *var_uc_depvdsef2_slot = var_uc_depvdsef2;
        *var_uc_depvdsef2_dn0_slot = var_uc_depvdsef2_dn0;
        *var_uc_depvdsef2_dn10_slot = var_uc_depvdsef2_dn10;
        *var_uc_depvdsef2_dn11_slot = var_uc_depvdsef2_dn11;
        *var_uc_depvdsef2_dn14_slot = var_uc_depvdsef2_dn14;
        *var_uc_depvdsef2_dn2_slot = var_uc_depvdsef2_dn2;
        *var_uc_depvdsef2_dn4_slot = var_uc_depvdsef2_dn4;
        *var_uc_depvdsef2_dn5_slot = var_uc_depvdsef2_dn5;
        *var_uc_depvdsef2_dn6_slot = var_uc_depvdsef2_dn6;
        *var_uc_depvdsef2_dn7_slot = var_uc_depvdsef2_dn7;
        *var_uc_depvdsef2_dn8_slot = var_uc_depvdsef2_dn8;
        *var_uc_depvdsef2_dn9_slot = var_uc_depvdsef2_dn9;
        *var_uc_depvdsef2_rv_slot = var_uc_depvdsef2_rv;
        *var_uc_depvmax_slot = var_uc_depvmax;
        *var_uc_depvmax_dn0_slot = var_uc_depvmax_dn0;
        *var_uc_depvmax_dn10_slot = var_uc_depvmax_dn10;
        *var_uc_depvmax_dn11_slot = var_uc_depvmax_dn11;
        *var_uc_depvmax_dn14_slot = var_uc_depvmax_dn14;
        *var_uc_depvmax_dn2_slot = var_uc_depvmax_dn2;
        *var_uc_depvmax_dn4_slot = var_uc_depvmax_dn4;
        *var_uc_depvmax_dn5_slot = var_uc_depvmax_dn5;
        *var_uc_depvmax_dn6_slot = var_uc_depvmax_dn6;
        *var_uc_depvmax_dn7_slot = var_uc_depvmax_dn7;
        *var_uc_depvmax_dn8_slot = var_uc_depvmax_dn8;
        *var_uc_depvmax_dn9_slot = var_uc_depvmax_dn9;
        *var_uc_depvmax_rv_slot = var_uc_depvmax_rv;
        *var_uc_ndepm_slot = var_uc_ndepm;
        *var_uc_ndepm_dn0_slot = var_uc_ndepm_dn0;
        *var_uc_ndepm_dn10_slot = var_uc_ndepm_dn10;
        *var_uc_ndepm_dn11_slot = var_uc_ndepm_dn11;
        *var_uc_ndepm_dn14_slot = var_uc_ndepm_dn14;
        *var_uc_ndepm_dn2_slot = var_uc_ndepm_dn2;
        *var_uc_ndepm_dn4_slot = var_uc_ndepm_dn4;
        *var_uc_ndepm_dn5_slot = var_uc_ndepm_dn5;
        *var_uc_ndepm_dn6_slot = var_uc_ndepm_dn6;
        *var_uc_ndepm_dn7_slot = var_uc_ndepm_dn7;
        *var_uc_ndepm_dn8_slot = var_uc_ndepm_dn8;
        *var_uc_ndepm_dn9_slot = var_uc_ndepm_dn9;
        *var_uc_ndepm_rv_slot = var_uc_ndepm_rv;
        *var_uc_xpdv_slot = var_uc_xpdv;
        *var_uc_xpdv_rv_slot = var_uc_xpdv_rv;
    }

    pub(super) fn stamp_reactive_block_17(
        p: &Parameters,
        var_ktnom: f64,
        var_lg: f64,
        var_mks_nsubcdfm: f64,
        var_nsubcdfm_given: f64,
        var_uc_eg0: f64,
        var_uc_mueph1: f64,
        var_uc_pgd1: f64,
        var_uc_scp22: f64,
        var_wg: f64,
        var_cecox_slot: &mut f64,
        var_cecox_rv_slot: &mut f64,
        var_clmmod_slot: &mut f64,
        var_clmmod_rv_slot: &mut f64,
        var_cnstpgd_slot: &mut f64,
        var_cnstpgd_rv_slot: &mut f64,
        var_ef_mueph1_slot: &mut f64,
        var_ef_mueph1_dn0_slot: &mut f64,
        var_ef_mueph1_dn10_slot: &mut f64,
        var_ef_mueph1_dn11_slot: &mut f64,
        var_ef_mueph1_dn14_slot: &mut f64,
        var_ef_mueph1_dn2_slot: &mut f64,
        var_ef_mueph1_dn4_slot: &mut f64,
        var_ef_mueph1_dn5_slot: &mut f64,
        var_ef_mueph1_dn6_slot: &mut f64,
        var_ef_mueph1_dn7_slot: &mut f64,
        var_ef_mueph1_dn8_slot: &mut f64,
        var_ef_mueph1_dn9_slot: &mut f64,
        var_ef_mueph1_rv_slot: &mut f64,
        var_ef_nsubc_slot: &mut f64,
        var_ef_nsubc_dn0_slot: &mut f64,
        var_ef_nsubc_dn10_slot: &mut f64,
        var_ef_nsubc_dn11_slot: &mut f64,
        var_ef_nsubc_dn14_slot: &mut f64,
        var_ef_nsubc_dn2_slot: &mut f64,
        var_ef_nsubc_dn4_slot: &mut f64,
        var_ef_nsubc_dn5_slot: &mut f64,
        var_ef_nsubc_dn6_slot: &mut f64,
        var_ef_nsubc_dn7_slot: &mut f64,
        var_ef_nsubc_dn8_slot: &mut f64,
        var_ef_nsubc_dn9_slot: &mut f64,
        var_ef_nsubc_rv_slot: &mut f64,
        var_ef_nsubp_slot: &mut f64,
        var_ef_nsubp_dn0_slot: &mut f64,
        var_ef_nsubp_dn10_slot: &mut f64,
        var_ef_nsubp_dn11_slot: &mut f64,
        var_ef_nsubp_dn14_slot: &mut f64,
        var_ef_nsubp_dn2_slot: &mut f64,
        var_ef_nsubp_dn4_slot: &mut f64,
        var_ef_nsubp_dn5_slot: &mut f64,
        var_ef_nsubp_dn6_slot: &mut f64,
        var_ef_nsubp_dn7_slot: &mut f64,
        var_ef_nsubp_dn8_slot: &mut f64,
        var_ef_nsubp_dn9_slot: &mut f64,
        var_ef_nsubp_rv_slot: &mut f64,
        var_egtnom_slot: &mut f64,
        var_egtnom_rv_slot: &mut f64,
        var_flg_nqs_slot: &mut f64,
        var_flg_nqs_rv_slot: &mut f64,
        var_flg_pgd_slot: &mut f64,
        var_flg_pgd_rv_slot: &mut f64,
        var_flg_qmetemp_slot: &mut f64,
        var_flg_qmetemp_rv_slot: &mut f64,
        var_flg_qy_slot: &mut f64,
        var_flg_qy_rv_slot: &mut f64,
        var_guard252_slot: &mut f64,
        var_guard252_rv_slot: &mut f64,
        var_guard253_slot: &mut f64,
        var_guard253_rv_slot: &mut f64,
        var_guard254_slot: &mut f64,
        var_guard254_rv_slot: &mut f64,
        var_guard255_slot: &mut f64,
        var_guard255_rv_slot: &mut f64,
        var_i_slot: &mut f64,
        var_i_rv_slot: &mut f64,
        var_lod_half_slot: &mut f64,
        var_lod_half_dn0_slot: &mut f64,
        var_lod_half_dn10_slot: &mut f64,
        var_lod_half_dn11_slot: &mut f64,
        var_lod_half_dn14_slot: &mut f64,
        var_lod_half_dn2_slot: &mut f64,
        var_lod_half_dn4_slot: &mut f64,
        var_lod_half_dn5_slot: &mut f64,
        var_lod_half_dn6_slot: &mut f64,
        var_lod_half_dn7_slot: &mut f64,
        var_lod_half_dn8_slot: &mut f64,
        var_lod_half_dn9_slot: &mut f64,
        var_lod_half_ref_slot: &mut f64,
        var_lod_half_ref_dn0_slot: &mut f64,
        var_lod_half_ref_dn10_slot: &mut f64,
        var_lod_half_ref_dn11_slot: &mut f64,
        var_lod_half_ref_dn14_slot: &mut f64,
        var_lod_half_ref_dn2_slot: &mut f64,
        var_lod_half_ref_dn4_slot: &mut f64,
        var_lod_half_ref_dn5_slot: &mut f64,
        var_lod_half_ref_dn6_slot: &mut f64,
        var_lod_half_ref_dn7_slot: &mut f64,
        var_lod_half_ref_dn8_slot: &mut f64,
        var_lod_half_ref_dn9_slot: &mut f64,
        var_lod_half_ref_rv_slot: &mut f64,
        var_lod_half_rv_slot: &mut f64,
        var_mks_subld2_slot: &mut f64,
        var_mks_subld2_rv_slot: &mut f64,
        var_msc_slot: &mut f64,
        var_msc_rv_slot: &mut f64,
        var_npexte_slot: &mut f64,
        var_npexte_dn0_slot: &mut f64,
        var_npexte_dn10_slot: &mut f64,
        var_npexte_dn11_slot: &mut f64,
        var_npexte_dn14_slot: &mut f64,
        var_npexte_dn2_slot: &mut f64,
        var_npexte_dn4_slot: &mut f64,
        var_npexte_dn5_slot: &mut f64,
        var_npexte_dn6_slot: &mut f64,
        var_npexte_dn7_slot: &mut f64,
        var_npexte_dn8_slot: &mut f64,
        var_npexte_dn9_slot: &mut f64,
        var_npexte_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_uc_depvmax_slot: &mut f64,
        var_uc_depvmax_dn0_slot: &mut f64,
        var_uc_depvmax_dn10_slot: &mut f64,
        var_uc_depvmax_dn11_slot: &mut f64,
        var_uc_depvmax_dn14_slot: &mut f64,
        var_uc_depvmax_dn2_slot: &mut f64,
        var_uc_depvmax_dn4_slot: &mut f64,
        var_uc_depvmax_dn5_slot: &mut f64,
        var_uc_depvmax_dn6_slot: &mut f64,
        var_uc_depvmax_dn7_slot: &mut f64,
        var_uc_depvmax_dn8_slot: &mut f64,
        var_uc_depvmax_dn9_slot: &mut f64,
        var_uc_depvmax_rv_slot: &mut f64,
        var_uc_fn2_slot: &mut f64,
        var_uc_fn2_rv_slot: &mut f64,
        var_uc_gdld_slot: &mut f64,
        var_uc_gdld_rv_slot: &mut f64,
        var_uc_gidl1_slot: &mut f64,
        var_uc_gidl1_rv_slot: &mut f64,
        var_uc_gidl2_slot: &mut f64,
        var_uc_gidl2_rv_slot: &mut f64,
        var_uc_gleak2_slot: &mut f64,
        var_uc_gleak2_rv_slot: &mut f64,
        var_uc_glkb2_slot: &mut f64,
        var_uc_glkb2_rv_slot: &mut f64,
        var_uc_glksd1_slot: &mut f64,
        var_uc_glksd1_rv_slot: &mut f64,
        var_uc_glksd2_slot: &mut f64,
        var_uc_glksd2_rv_slot: &mut f64,
        var_uc_muesti1_slot: &mut f64,
        var_uc_muesti1_rv_slot: &mut f64,
        var_uc_nfalp_slot: &mut f64,
        var_uc_nfalp_rv_slot: &mut f64,
        var_uc_nover_slot: &mut f64,
        var_uc_nover_rv_slot: &mut f64,
        var_uc_novers_slot: &mut f64,
        var_uc_novers_rv_slot: &mut f64,
        var_uc_npext_slot: &mut f64,
        var_uc_npext_rv_slot: &mut f64,
        var_uc_nsti_slot: &mut f64,
        var_uc_nsti_rv_slot: &mut f64,
        var_uc_nsubc_slot: &mut f64,
        var_uc_nsubc_rv_slot: &mut f64,
        var_uc_nsubp_slot: &mut f64,
        var_uc_nsubp_rv_slot: &mut f64,
        var_uc_nsubpsti1_slot: &mut f64,
        var_uc_nsubpsti1_rv_slot: &mut f64,
        var_uc_rd22_slot: &mut f64,
        var_uc_rd22_rv_slot: &mut f64,
        var_uc_rd23_slot: &mut f64,
        var_uc_rd23_rv_slot: &mut f64,
        var_uc_rd24_slot: &mut f64,
        var_uc_rd24_rv_slot: &mut f64,
        var_uc_rdvd_slot: &mut f64,
        var_uc_rdvd_rv_slot: &mut f64,
        var_uc_rth0_slot: &mut f64,
        var_uc_rth0_rv_slot: &mut f64,
        var_uc_vfbover_slot: &mut f64,
        var_uc_vfbover_rv_slot: &mut f64,
        var_uc_vmax_slot: &mut f64,
        var_uc_vmax_rv_slot: &mut f64,
        var_uc_wfc_slot: &mut f64,
        var_uc_wfc_rv_slot: &mut f64,
        var_wlg_slot: &mut f64,
        var_wlg_rv_slot: &mut f64,
    ) {
        let mut var_cecox: f64 = *var_cecox_slot;
        let mut var_cecox_rv: f64 = *var_cecox_rv_slot;
        let mut var_clmmod: f64 = *var_clmmod_slot;
        let mut var_clmmod_rv: f64 = *var_clmmod_rv_slot;
        let mut var_cnstpgd: f64 = *var_cnstpgd_slot;
        let mut var_cnstpgd_rv: f64 = *var_cnstpgd_rv_slot;
        let mut var_ef_mueph1: f64 = *var_ef_mueph1_slot;
        let mut var_ef_mueph1_dn0: f64 = *var_ef_mueph1_dn0_slot;
        let mut var_ef_mueph1_dn10: f64 = *var_ef_mueph1_dn10_slot;
        let mut var_ef_mueph1_dn11: f64 = *var_ef_mueph1_dn11_slot;
        let mut var_ef_mueph1_dn14: f64 = *var_ef_mueph1_dn14_slot;
        let mut var_ef_mueph1_dn2: f64 = *var_ef_mueph1_dn2_slot;
        let mut var_ef_mueph1_dn4: f64 = *var_ef_mueph1_dn4_slot;
        let mut var_ef_mueph1_dn5: f64 = *var_ef_mueph1_dn5_slot;
        let mut var_ef_mueph1_dn6: f64 = *var_ef_mueph1_dn6_slot;
        let mut var_ef_mueph1_dn7: f64 = *var_ef_mueph1_dn7_slot;
        let mut var_ef_mueph1_dn8: f64 = *var_ef_mueph1_dn8_slot;
        let mut var_ef_mueph1_dn9: f64 = *var_ef_mueph1_dn9_slot;
        let mut var_ef_mueph1_rv: f64 = *var_ef_mueph1_rv_slot;
        let mut var_ef_nsubc: f64 = *var_ef_nsubc_slot;
        let mut var_ef_nsubc_dn0: f64 = *var_ef_nsubc_dn0_slot;
        let mut var_ef_nsubc_dn10: f64 = *var_ef_nsubc_dn10_slot;
        let mut var_ef_nsubc_dn11: f64 = *var_ef_nsubc_dn11_slot;
        let mut var_ef_nsubc_dn14: f64 = *var_ef_nsubc_dn14_slot;
        let mut var_ef_nsubc_dn2: f64 = *var_ef_nsubc_dn2_slot;
        let mut var_ef_nsubc_dn4: f64 = *var_ef_nsubc_dn4_slot;
        let mut var_ef_nsubc_dn5: f64 = *var_ef_nsubc_dn5_slot;
        let mut var_ef_nsubc_dn6: f64 = *var_ef_nsubc_dn6_slot;
        let mut var_ef_nsubc_dn7: f64 = *var_ef_nsubc_dn7_slot;
        let mut var_ef_nsubc_dn8: f64 = *var_ef_nsubc_dn8_slot;
        let mut var_ef_nsubc_dn9: f64 = *var_ef_nsubc_dn9_slot;
        let mut var_ef_nsubc_rv: f64 = *var_ef_nsubc_rv_slot;
        let mut var_ef_nsubp: f64 = *var_ef_nsubp_slot;
        let mut var_ef_nsubp_dn0: f64 = *var_ef_nsubp_dn0_slot;
        let mut var_ef_nsubp_dn10: f64 = *var_ef_nsubp_dn10_slot;
        let mut var_ef_nsubp_dn11: f64 = *var_ef_nsubp_dn11_slot;
        let mut var_ef_nsubp_dn14: f64 = *var_ef_nsubp_dn14_slot;
        let mut var_ef_nsubp_dn2: f64 = *var_ef_nsubp_dn2_slot;
        let mut var_ef_nsubp_dn4: f64 = *var_ef_nsubp_dn4_slot;
        let mut var_ef_nsubp_dn5: f64 = *var_ef_nsubp_dn5_slot;
        let mut var_ef_nsubp_dn6: f64 = *var_ef_nsubp_dn6_slot;
        let mut var_ef_nsubp_dn7: f64 = *var_ef_nsubp_dn7_slot;
        let mut var_ef_nsubp_dn8: f64 = *var_ef_nsubp_dn8_slot;
        let mut var_ef_nsubp_dn9: f64 = *var_ef_nsubp_dn9_slot;
        let mut var_ef_nsubp_rv: f64 = *var_ef_nsubp_rv_slot;
        let mut var_egtnom: f64 = *var_egtnom_slot;
        let mut var_egtnom_rv: f64 = *var_egtnom_rv_slot;
        let mut var_flg_nqs: f64 = *var_flg_nqs_slot;
        let mut var_flg_nqs_rv: f64 = *var_flg_nqs_rv_slot;
        let mut var_flg_pgd: f64 = *var_flg_pgd_slot;
        let mut var_flg_pgd_rv: f64 = *var_flg_pgd_rv_slot;
        let mut var_flg_qmetemp: f64 = *var_flg_qmetemp_slot;
        let mut var_flg_qmetemp_rv: f64 = *var_flg_qmetemp_rv_slot;
        let mut var_flg_qy: f64 = *var_flg_qy_slot;
        let mut var_flg_qy_rv: f64 = *var_flg_qy_rv_slot;
        let mut var_guard252: f64 = *var_guard252_slot;
        let mut var_guard252_rv: f64 = *var_guard252_rv_slot;
        let mut var_guard253: f64 = *var_guard253_slot;
        let mut var_guard253_rv: f64 = *var_guard253_rv_slot;
        let mut var_guard254: f64 = *var_guard254_slot;
        let mut var_guard254_rv: f64 = *var_guard254_rv_slot;
        let mut var_guard255: f64 = *var_guard255_slot;
        let mut var_guard255_rv: f64 = *var_guard255_rv_slot;
        let mut var_i: f64 = *var_i_slot;
        let mut var_i_rv: f64 = *var_i_rv_slot;
        let mut var_lod_half: f64 = *var_lod_half_slot;
        let mut var_lod_half_dn0: f64 = *var_lod_half_dn0_slot;
        let mut var_lod_half_dn10: f64 = *var_lod_half_dn10_slot;
        let mut var_lod_half_dn11: f64 = *var_lod_half_dn11_slot;
        let mut var_lod_half_dn14: f64 = *var_lod_half_dn14_slot;
        let mut var_lod_half_dn2: f64 = *var_lod_half_dn2_slot;
        let mut var_lod_half_dn4: f64 = *var_lod_half_dn4_slot;
        let mut var_lod_half_dn5: f64 = *var_lod_half_dn5_slot;
        let mut var_lod_half_dn6: f64 = *var_lod_half_dn6_slot;
        let mut var_lod_half_dn7: f64 = *var_lod_half_dn7_slot;
        let mut var_lod_half_dn8: f64 = *var_lod_half_dn8_slot;
        let mut var_lod_half_dn9: f64 = *var_lod_half_dn9_slot;
        let mut var_lod_half_ref: f64 = *var_lod_half_ref_slot;
        let mut var_lod_half_ref_dn0: f64 = *var_lod_half_ref_dn0_slot;
        let mut var_lod_half_ref_dn10: f64 = *var_lod_half_ref_dn10_slot;
        let mut var_lod_half_ref_dn11: f64 = *var_lod_half_ref_dn11_slot;
        let mut var_lod_half_ref_dn14: f64 = *var_lod_half_ref_dn14_slot;
        let mut var_lod_half_ref_dn2: f64 = *var_lod_half_ref_dn2_slot;
        let mut var_lod_half_ref_dn4: f64 = *var_lod_half_ref_dn4_slot;
        let mut var_lod_half_ref_dn5: f64 = *var_lod_half_ref_dn5_slot;
        let mut var_lod_half_ref_dn6: f64 = *var_lod_half_ref_dn6_slot;
        let mut var_lod_half_ref_dn7: f64 = *var_lod_half_ref_dn7_slot;
        let mut var_lod_half_ref_dn8: f64 = *var_lod_half_ref_dn8_slot;
        let mut var_lod_half_ref_dn9: f64 = *var_lod_half_ref_dn9_slot;
        let mut var_lod_half_ref_rv: f64 = *var_lod_half_ref_rv_slot;
        let mut var_lod_half_rv: f64 = *var_lod_half_rv_slot;
        let mut var_mks_subld2: f64 = *var_mks_subld2_slot;
        let mut var_mks_subld2_rv: f64 = *var_mks_subld2_rv_slot;
        let mut var_msc: f64 = *var_msc_slot;
        let mut var_msc_rv: f64 = *var_msc_rv_slot;
        let mut var_npexte: f64 = *var_npexte_slot;
        let mut var_npexte_dn0: f64 = *var_npexte_dn0_slot;
        let mut var_npexte_dn10: f64 = *var_npexte_dn10_slot;
        let mut var_npexte_dn11: f64 = *var_npexte_dn11_slot;
        let mut var_npexte_dn14: f64 = *var_npexte_dn14_slot;
        let mut var_npexte_dn2: f64 = *var_npexte_dn2_slot;
        let mut var_npexte_dn4: f64 = *var_npexte_dn4_slot;
        let mut var_npexte_dn5: f64 = *var_npexte_dn5_slot;
        let mut var_npexte_dn6: f64 = *var_npexte_dn6_slot;
        let mut var_npexte_dn7: f64 = *var_npexte_dn7_slot;
        let mut var_npexte_dn8: f64 = *var_npexte_dn8_slot;
        let mut var_npexte_dn9: f64 = *var_npexte_dn9_slot;
        let mut var_npexte_rv: f64 = *var_npexte_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_uc_depvmax: f64 = *var_uc_depvmax_slot;
        let mut var_uc_depvmax_dn0: f64 = *var_uc_depvmax_dn0_slot;
        let mut var_uc_depvmax_dn10: f64 = *var_uc_depvmax_dn10_slot;
        let mut var_uc_depvmax_dn11: f64 = *var_uc_depvmax_dn11_slot;
        let mut var_uc_depvmax_dn14: f64 = *var_uc_depvmax_dn14_slot;
        let mut var_uc_depvmax_dn2: f64 = *var_uc_depvmax_dn2_slot;
        let mut var_uc_depvmax_dn4: f64 = *var_uc_depvmax_dn4_slot;
        let mut var_uc_depvmax_dn5: f64 = *var_uc_depvmax_dn5_slot;
        let mut var_uc_depvmax_dn6: f64 = *var_uc_depvmax_dn6_slot;
        let mut var_uc_depvmax_dn7: f64 = *var_uc_depvmax_dn7_slot;
        let mut var_uc_depvmax_dn8: f64 = *var_uc_depvmax_dn8_slot;
        let mut var_uc_depvmax_dn9: f64 = *var_uc_depvmax_dn9_slot;
        let mut var_uc_depvmax_rv: f64 = *var_uc_depvmax_rv_slot;
        let mut var_uc_fn2: f64 = *var_uc_fn2_slot;
        let mut var_uc_fn2_rv: f64 = *var_uc_fn2_rv_slot;
        let mut var_uc_gdld: f64 = *var_uc_gdld_slot;
        let mut var_uc_gdld_rv: f64 = *var_uc_gdld_rv_slot;
        let mut var_uc_gidl1: f64 = *var_uc_gidl1_slot;
        let mut var_uc_gidl1_rv: f64 = *var_uc_gidl1_rv_slot;
        let mut var_uc_gidl2: f64 = *var_uc_gidl2_slot;
        let mut var_uc_gidl2_rv: f64 = *var_uc_gidl2_rv_slot;
        let mut var_uc_gleak2: f64 = *var_uc_gleak2_slot;
        let mut var_uc_gleak2_rv: f64 = *var_uc_gleak2_rv_slot;
        let mut var_uc_glkb2: f64 = *var_uc_glkb2_slot;
        let mut var_uc_glkb2_rv: f64 = *var_uc_glkb2_rv_slot;
        let mut var_uc_glksd1: f64 = *var_uc_glksd1_slot;
        let mut var_uc_glksd1_rv: f64 = *var_uc_glksd1_rv_slot;
        let mut var_uc_glksd2: f64 = *var_uc_glksd2_slot;
        let mut var_uc_glksd2_rv: f64 = *var_uc_glksd2_rv_slot;
        let mut var_uc_muesti1: f64 = *var_uc_muesti1_slot;
        let mut var_uc_muesti1_rv: f64 = *var_uc_muesti1_rv_slot;
        let mut var_uc_nfalp: f64 = *var_uc_nfalp_slot;
        let mut var_uc_nfalp_rv: f64 = *var_uc_nfalp_rv_slot;
        let mut var_uc_nover: f64 = *var_uc_nover_slot;
        let mut var_uc_nover_rv: f64 = *var_uc_nover_rv_slot;
        let mut var_uc_novers: f64 = *var_uc_novers_slot;
        let mut var_uc_novers_rv: f64 = *var_uc_novers_rv_slot;
        let mut var_uc_npext: f64 = *var_uc_npext_slot;
        let mut var_uc_npext_rv: f64 = *var_uc_npext_rv_slot;
        let mut var_uc_nsti: f64 = *var_uc_nsti_slot;
        let mut var_uc_nsti_rv: f64 = *var_uc_nsti_rv_slot;
        let mut var_uc_nsubc: f64 = *var_uc_nsubc_slot;
        let mut var_uc_nsubc_rv: f64 = *var_uc_nsubc_rv_slot;
        let mut var_uc_nsubp: f64 = *var_uc_nsubp_slot;
        let mut var_uc_nsubp_rv: f64 = *var_uc_nsubp_rv_slot;
        let mut var_uc_nsubpsti1: f64 = *var_uc_nsubpsti1_slot;
        let mut var_uc_nsubpsti1_rv: f64 = *var_uc_nsubpsti1_rv_slot;
        let mut var_uc_rd22: f64 = *var_uc_rd22_slot;
        let mut var_uc_rd22_rv: f64 = *var_uc_rd22_rv_slot;
        let mut var_uc_rd23: f64 = *var_uc_rd23_slot;
        let mut var_uc_rd23_rv: f64 = *var_uc_rd23_rv_slot;
        let mut var_uc_rd24: f64 = *var_uc_rd24_slot;
        let mut var_uc_rd24_rv: f64 = *var_uc_rd24_rv_slot;
        let mut var_uc_rdvd: f64 = *var_uc_rdvd_slot;
        let mut var_uc_rdvd_rv: f64 = *var_uc_rdvd_rv_slot;
        let mut var_uc_rth0: f64 = *var_uc_rth0_slot;
        let mut var_uc_rth0_rv: f64 = *var_uc_rth0_rv_slot;
        let mut var_uc_vfbover: f64 = *var_uc_vfbover_slot;
        let mut var_uc_vfbover_rv: f64 = *var_uc_vfbover_rv_slot;
        let mut var_uc_vmax: f64 = *var_uc_vmax_slot;
        let mut var_uc_vmax_rv: f64 = *var_uc_vmax_rv_slot;
        let mut var_uc_wfc: f64 = *var_uc_wfc_slot;
        let mut var_uc_wfc_rv: f64 = *var_uc_wfc_rv_slot;
        let mut var_wlg: f64 = *var_wlg_slot;
        let mut var_wlg_rv: f64 = *var_wlg_rv_slot;

        let assign10310_e5506: f64 = (p.p73 * 100.0);
        var_mks_subld2 = assign10310_e5506;
        var_mks_subld2_rv = 0.0;

        let assign10320_e5509: f64 = (var_uc_nsubc / 1e-6);
        var_uc_nsubc = assign10320_e5509;
        var_uc_nsubc_rv = 0.0;

        let assign10330_e5512: f64 = (var_uc_nsubp / 1e-6);
        var_uc_nsubp = assign10330_e5512;
        var_uc_nsubp_rv = 0.0;

        let assign10340_e5515: f64 = (var_uc_nsti / 1e-6);
        var_uc_nsti = assign10340_e5515;
        var_uc_nsti_rv = 0.0;

        let assign10350_e5518: f64 = (var_uc_nover / 1e-6);
        var_uc_nover = assign10350_e5518;
        var_uc_nover_rv = 0.0;

        let assign10360_e5521: f64 = (var_uc_novers / 1e-6);
        var_uc_novers = assign10360_e5521;
        var_uc_novers_rv = 0.0;

        let assign10370_e5524: f64 = (var_uc_nsubpsti1 / 100.0);
        var_uc_nsubpsti1 = assign10370_e5524;
        var_uc_nsubpsti1_rv = 0.0;

        let assign10380_e5527: f64 = (var_uc_muesti1 / 100.0);
        var_uc_muesti1 = assign10380_e5527;
        var_uc_muesti1_rv = 0.0;

        let assign10390_e5530: f64 = (var_uc_vmax / 100.0);
        var_uc_vmax = assign10390_e5530;
        var_uc_vmax_rv = 0.0;

        let assign10400_e5533: f64 = (var_uc_wfc * 10000.0);
        var_uc_wfc = assign10400_e5533;
        var_uc_wfc_rv = 0.0;

        let assign10410_e5536: f64 = (var_uc_glksd1 / 100.0);
        var_uc_glksd1 = assign10410_e5536;
        var_uc_glksd1_rv = 0.0;

        let assign10420_e5539: f64 = (var_uc_glksd2 * 100.0);
        var_uc_glksd2 = assign10420_e5539;
        var_uc_glksd2_rv = 0.0;

        let assign10430_e5542: f64 = (var_uc_gleak2 * 100.0);
        var_uc_gleak2 = assign10430_e5542;
        var_uc_gleak2_rv = 0.0;

        let assign10440_e5545: f64 = (var_uc_glkb2 * 100.0);
        var_uc_glkb2 = assign10440_e5545;
        var_uc_glkb2_rv = 0.0;

        let assign10450_e5548: f64 = (var_uc_fn2 * 100.0);
        var_uc_fn2 = assign10450_e5548;
        var_uc_fn2_rv = 0.0;

        let assign10460_e5551: f64 = (var_uc_gidl1 / 10.0);
        var_uc_gidl1 = assign10460_e5551;
        var_uc_gidl1_rv = 0.0;

        let assign10470_e5554: f64 = (var_uc_gidl2 * 100.0);
        var_uc_gidl2 = assign10470_e5554;
        var_uc_gidl2_rv = 0.0;

        let assign10480_e5557: f64 = (var_uc_nfalp / 100.0);
        var_uc_nfalp = assign10480_e5557;
        var_uc_nfalp_rv = 0.0;

        let assign10500_e5563: f64 = (var_uc_npext / 1e-6);
        var_uc_npext = assign10500_e5563;
        var_uc_npext_rv = 0.0;

        let assign10510_e5566: f64 = (var_uc_rd22 / 100.0);
        var_uc_rd22 = assign10510_e5566;
        var_uc_rd22_rv = 0.0;

        let assign10520_e5569: f64 = (var_uc_rd23 / 100.0);
        var_uc_rd23 = assign10520_e5569;
        var_uc_rd23_rv = 0.0;

        let assign10530_e5572: f64 = (var_uc_rd24 / 100.0);
        var_uc_rd24 = assign10530_e5572;
        var_uc_rd24_rv = 0.0;

        let assign10540_e5575: f64 = (var_uc_rdvd / 100.0);
        var_uc_rdvd = assign10540_e5575;
        var_uc_rdvd_rv = 0.0;

        let assign10550_e5578: f64 = (var_uc_rth0 / 100.0);
        var_uc_rth0 = assign10550_e5578;
        var_uc_rth0_rv = 0.0;

        let assign10560_e5580: f64 = (-var_uc_vfbover);
        var_uc_vfbover = assign10560_e5580;
        var_uc_vfbover_rv = 0.0;

        let assign10570_e5583: f64 = (var_uc_depvmax / 100.0);
        var_uc_depvmax = assign10570_e5583;
        var_uc_depvmax_dn0 = (var_uc_depvmax_dn0 / 100.0);
        var_uc_depvmax_dn2 = (var_uc_depvmax_dn2 / 100.0);
        var_uc_depvmax_dn4 = (var_uc_depvmax_dn4 / 100.0);
        var_uc_depvmax_dn5 = (var_uc_depvmax_dn5 / 100.0);
        var_uc_depvmax_dn6 = (var_uc_depvmax_dn6 / 100.0);
        var_uc_depvmax_dn7 = (var_uc_depvmax_dn7 / 100.0);
        var_uc_depvmax_dn8 = (var_uc_depvmax_dn8 / 100.0);
        var_uc_depvmax_dn9 = (var_uc_depvmax_dn9 / 100.0);
        var_uc_depvmax_dn10 = (var_uc_depvmax_dn10 / 100.0);
        var_uc_depvmax_dn11 = (var_uc_depvmax_dn11 / 100.0);
        var_uc_depvmax_dn14 = (var_uc_depvmax_dn14 / 100.0);
        var_uc_depvmax_rv = 0.0;

        var_flg_nqs = p.p28;
        var_flg_nqs_rv = 0.0;

        let (assign10590_e5594,) = {
    if ((p.p133 != 0.0) || (p.p134 != 0.0)) {
        (1.0,)
    } else {
        (0.0,)
    }
};
        var_flg_qy = assign10590_e5594;
        var_flg_qy_rv = 0.0;

        let assign10610_e5608: f64 = if (((p.p235 == 0.0) && (p.p237 == 0.0)) || (p.p236 == 0.0)) { 1.0 } else { 0.0 };
        var_guard252 = assign10610_e5608;
        var_guard252_rv = 0.0;

        let (assign10620_e5612,) = {
    if (var_guard252 != 0.0) {
        (0.0,)
    } else {
        (var_flg_qmetemp,)
    }
};
        var_flg_qmetemp = assign10620_e5612;
        var_flg_qmetemp_rv = 0.0;

        let (assign10630_e5617,) = {
    if (var_guard252 == 0.0) {
        (1.0,)
    } else {
        (var_flg_qmetemp,)
    }
};
        var_flg_qmetemp = assign10630_e5617;
        var_flg_qmetemp_rv = 0.0;

        let assign10640_e5620: f64 = (var_wg * var_lg);
        var_wlg = assign10640_e5620;
        var_wlg_rv = 0.0;

        let assign10650_e5623: f64 = (p.p289 * 1000000.0);
        var_uc_gdld = assign10650_e5623;
        var_uc_gdld_rv = 0.0;

        let assign10660_e5629: f64 = (var_ktnom * 1e-7);
        let assign10660_e5630: f64 = (9.025e-5 + assign10660_e5629);
        let assign10660_e5631: f64 = (var_ktnom * assign10660_e5630);
        let assign10660_e5632: f64 = (var_uc_eg0 - assign10660_e5631);
        var_egtnom = assign10660_e5632;
        var_egtnom_rv = 0.0;

        let assign10670_e5635: f64 = (8.8541878e-12 * p.p267);
        var_cecox = assign10670_e5635;
        var_cecox_rv = 0.0;

        var_msc = var_uc_scp22;
        var_msc_rv = 0.0;

        let assign10690_e5639: f64 = if var_uc_pgd1 == 0.0 { 1.0 } else { 0.0 };
        var_guard253 = assign10690_e5639;
        var_guard253_rv = 0.0;

        let (assign10700_e5643,) = {
    if (var_guard253 != 0.0) {
        (0.0,)
    } else {
        (var_flg_pgd,)
    }
};
        var_flg_pgd = assign10700_e5643;
        var_flg_pgd_rv = 0.0;

        let (assign10710_e5647,) = {
    if (var_guard253 != 0.0) {
        (0.0,)
    } else {
        (var_cnstpgd,)
    }
};
        var_cnstpgd = assign10710_e5647;
        var_cnstpgd_rv = 0.0;

        let (assign10720_e5652,) = {
    if (var_guard253 == 0.0) {
        (1.0,)
    } else {
        (var_flg_pgd,)
    }
};
        var_flg_pgd = assign10720_e5652;
        var_flg_pgd_rv = 0.0;

        let (assign10730_e5665,) = {
    if (var_guard253 == 0.0) {
        let assign10730_e5658: f64 = (1.0 / var_lg);
        let assign10730_e5659: f64 = (1.0 + assign10730_e5658);
        let assign10730_e5661: f64 = (assign10730_e5659).powf(p.p153);
        let assign10730_e5663: f64 = (assign10730_e5661 * var_uc_pgd1);
        (assign10730_e5663,)
    } else {
        (var_cnstpgd,)
    }
};
        var_cnstpgd = assign10730_e5665;
        var_cnstpgd_rv = 0.0;

        let assign10740_e5669: f64 = (var_lg).powf(p.p229);
        let assign10740_e5671: f64 = (assign10740_e5669 * p.p230);
        let assign10740_e5672: f64 = (1.0 + assign10740_e5671);
        var_clmmod = assign10740_e5672;
        var_clmmod_rv = 0.0;

        let assign10750_e5677: f64 = (0.5 * p.p0);
        let assign10750_e5678: f64 = (p.p118 + assign10750_e5677);
        let assign10750_e5679: f64 = (1.0 / assign10750_e5678);
        let assign10750_e5684: f64 = (0.5 * p.p0);
        let assign10750_e5685: f64 = (p.p119 + assign10750_e5684);
        let assign10750_e5686: f64 = (1.0 / assign10750_e5685);
        let assign10750_e5687: f64 = (assign10750_e5679 + assign10750_e5686);
        var_t1 = assign10750_e5687;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn11 = 0.0;
        var_t1_dn14 = 0.0;
        var_t1_rv = 0.0;

        let assign10760_e5690: f64 = (2.0 / var_t1);
        var_lod_half_ref = assign10760_e5690;
        var_lod_half_ref_dn0 = (-((2.0 * var_t1_dn0) / (var_t1 * var_t1)));
        var_lod_half_ref_dn2 = (-((2.0 * var_t1_dn2) / (var_t1 * var_t1)));
        var_lod_half_ref_dn4 = (-((2.0 * var_t1_dn4) / (var_t1 * var_t1)));
        var_lod_half_ref_dn5 = (-((2.0 * var_t1_dn5) / (var_t1 * var_t1)));
        var_lod_half_ref_dn6 = (-((2.0 * var_t1_dn6) / (var_t1 * var_t1)));
        var_lod_half_ref_dn7 = (-((2.0 * var_t1_dn7) / (var_t1 * var_t1)));
        var_lod_half_ref_dn8 = (-((2.0 * var_t1_dn8) / (var_t1 * var_t1)));
        var_lod_half_ref_dn9 = (-((2.0 * var_t1_dn9) / (var_t1 * var_t1)));
        var_lod_half_ref_dn10 = (-((2.0 * var_t1_dn10) / (var_t1 * var_t1)));
        var_lod_half_ref_dn11 = (-((2.0 * var_t1_dn11) / (var_t1 * var_t1)));
        var_lod_half_ref_dn14 = (-((2.0 * var_t1_dn14) / (var_t1 * var_t1)));
        var_lod_half_ref_rv = 0.0;

        let assign10770_e5709: f64 = if (((p.p8 > 0.0) && (p.p9 > 0.0)) && ((p.p7 == 1.0) || ((p.p7 > 1.0) && (p.p10 > 0.0)))) { 1.0 } else { 0.0 };
        var_guard254 = assign10770_e5709;
        var_guard254_rv = 0.0;

        let (assign10780_e5713, assign10780_e5713_d_n0, assign10780_e5713_d_n2, assign10780_e5713_d_n4, assign10780_e5713_d_n5, assign10780_e5713_d_n6, assign10780_e5713_d_n7, assign10780_e5713_d_n8, assign10780_e5713_d_n9, assign10780_e5713_d_n10, assign10780_e5713_d_n11, assign10780_e5713_d_n14,) = {
    if (var_guard254 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign10780_e5713;
        var_t1_dn0 = assign10780_e5713_d_n0;
        var_t1_dn2 = assign10780_e5713_d_n2;
        var_t1_dn4 = assign10780_e5713_d_n4;
        var_t1_dn5 = assign10780_e5713_d_n5;
        var_t1_dn6 = assign10780_e5713_d_n6;
        var_t1_dn7 = assign10780_e5713_d_n7;
        var_t1_dn8 = assign10780_e5713_d_n8;
        var_t1_dn9 = assign10780_e5713_d_n9;
        var_t1_dn10 = assign10780_e5713_d_n10;
        var_t1_dn11 = assign10780_e5713_d_n11;
        var_t1_dn14 = assign10780_e5713_d_n14;
        var_t1_rv = 0.0;

        let (assign10790_e5717,) = {
    if (var_guard254 != 0.0) {
        (0.0,)
    } else {
        (var_i,)
    }
};
        var_i = assign10790_e5717;
        var_i_rv = 0.0;

        let mut assign10800_loop_guard: usize = 0;
        while {
            let assign10800_cond_e5722: f64 = if ((var_guard254 != 0.0) && (var_i < p.p7)) { 1.0 } else { 0.0 };
            assign10800_cond_e5722 != 0.0
        } {
            assign10800_loop_guard += 1;
            assert!(assign10800_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign10800_body0_e5754, assign10800_body0_e5754_d_n0, assign10800_body0_e5754_d_n2, assign10800_body0_e5754_d_n4, assign10800_body0_e5754_d_n5, assign10800_body0_e5754_d_n6, assign10800_body0_e5754_d_n7, assign10800_body0_e5754_d_n8, assign10800_body0_e5754_d_n9, assign10800_body0_e5754_d_n10, assign10800_body0_e5754_d_n11, assign10800_body0_e5754_d_n14,) = {
    if (var_guard254 != 0.0) {
        let assign10800_body0_e5729: f64 = (0.5 * p.p0);
        let assign10800_body0_e5730: f64 = (p.p8 + assign10800_body0_e5729);
        let assign10800_body0_e5734: f64 = (p.p10 + p.p0);
        let assign10800_body0_e5735: f64 = (var_i * assign10800_body0_e5734);
        let assign10800_body0_e5736: f64 = (assign10800_body0_e5730 + assign10800_body0_e5735);
        let assign10800_body0_e5737: f64 = (1.0 / assign10800_body0_e5736);
        let assign10800_body0_e5738: f64 = (var_t1 + assign10800_body0_e5737);
        let assign10800_body0_e5743: f64 = (0.5 * p.p0);
        let assign10800_body0_e5744: f64 = (p.p9 + assign10800_body0_e5743);
        let assign10800_body0_e5748: f64 = (p.p10 + p.p0);
        let assign10800_body0_e5749: f64 = (var_i * assign10800_body0_e5748);
        let assign10800_body0_e5750: f64 = (assign10800_body0_e5744 + assign10800_body0_e5749);
        let assign10800_body0_e5751: f64 = (1.0 / assign10800_body0_e5750);
        let assign10800_body0_e5752: f64 = (assign10800_body0_e5738 + assign10800_body0_e5751);
        (assign10800_body0_e5752, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
            var_t1 = assign10800_body0_e5754;
            var_t1_dn0 = assign10800_body0_e5754_d_n0;
            var_t1_dn2 = assign10800_body0_e5754_d_n2;
            var_t1_dn4 = assign10800_body0_e5754_d_n4;
            var_t1_dn5 = assign10800_body0_e5754_d_n5;
            var_t1_dn6 = assign10800_body0_e5754_d_n6;
            var_t1_dn7 = assign10800_body0_e5754_d_n7;
            var_t1_dn8 = assign10800_body0_e5754_d_n8;
            var_t1_dn9 = assign10800_body0_e5754_d_n9;
            var_t1_dn10 = assign10800_body0_e5754_d_n10;
            var_t1_dn11 = assign10800_body0_e5754_d_n11;
            var_t1_dn14 = assign10800_body0_e5754_d_n14;
            var_t1_rv = 0.0;
            let (assign10800_body1_e5760,) = {
    if (var_guard254 != 0.0) {
        let assign10800_body1_e5758: f64 = (var_i + 1.0);
        (assign10800_body1_e5758,)
    } else {
        (var_i,)
    }
};
            var_i = assign10800_body1_e5760;
            var_i_rv = 0.0;
        }

        let (assign10810_e5768, assign10810_e5768_d_n0, assign10810_e5768_d_n2, assign10810_e5768_d_n4, assign10810_e5768_d_n5, assign10810_e5768_d_n6, assign10810_e5768_d_n7, assign10810_e5768_d_n8, assign10810_e5768_d_n9, assign10810_e5768_d_n10, assign10810_e5768_d_n11, assign10810_e5768_d_n14,) = {
    if (var_guard254 != 0.0) {
        let assign10810_e5764: f64 = (2.0 * p.p7);
        let assign10810_e5766: f64 = (assign10810_e5764 / var_t1);
        (assign10810_e5766, (-((assign10810_e5764 * var_t1_dn0) / (var_t1 * var_t1))), (-((assign10810_e5764 * var_t1_dn2) / (var_t1 * var_t1))), (-((assign10810_e5764 * var_t1_dn4) / (var_t1 * var_t1))), (-((assign10810_e5764 * var_t1_dn5) / (var_t1 * var_t1))), (-((assign10810_e5764 * var_t1_dn6) / (var_t1 * var_t1))), (-((assign10810_e5764 * var_t1_dn7) / (var_t1 * var_t1))), (-((assign10810_e5764 * var_t1_dn8) / (var_t1 * var_t1))), (-((assign10810_e5764 * var_t1_dn9) / (var_t1 * var_t1))), (-((assign10810_e5764 * var_t1_dn10) / (var_t1 * var_t1))), (-((assign10810_e5764 * var_t1_dn11) / (var_t1 * var_t1))), (-((assign10810_e5764 * var_t1_dn14) / (var_t1 * var_t1))),)
    } else {
        (var_lod_half, var_lod_half_dn0, var_lod_half_dn2, var_lod_half_dn4, var_lod_half_dn5, var_lod_half_dn6, var_lod_half_dn7, var_lod_half_dn8, var_lod_half_dn9, var_lod_half_dn10, var_lod_half_dn11, var_lod_half_dn14,)
    }
};
        var_lod_half = assign10810_e5768;
        var_lod_half_dn0 = assign10810_e5768_d_n0;
        var_lod_half_dn2 = assign10810_e5768_d_n2;
        var_lod_half_dn4 = assign10810_e5768_d_n4;
        var_lod_half_dn5 = assign10810_e5768_d_n5;
        var_lod_half_dn6 = assign10810_e5768_d_n6;
        var_lod_half_dn7 = assign10810_e5768_d_n7;
        var_lod_half_dn8 = assign10810_e5768_d_n8;
        var_lod_half_dn9 = assign10810_e5768_d_n9;
        var_lod_half_dn10 = assign10810_e5768_d_n10;
        var_lod_half_dn11 = assign10810_e5768_d_n11;
        var_lod_half_dn14 = assign10810_e5768_d_n14;
        var_lod_half_rv = 0.0;

        let (assign10820_e5773, assign10820_e5773_d_n0, assign10820_e5773_d_n2, assign10820_e5773_d_n4, assign10820_e5773_d_n5, assign10820_e5773_d_n6, assign10820_e5773_d_n7, assign10820_e5773_d_n8, assign10820_e5773_d_n9, assign10820_e5773_d_n10, assign10820_e5773_d_n11, assign10820_e5773_d_n14,) = {
    if (var_guard254 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_lod_half, var_lod_half_dn0, var_lod_half_dn2, var_lod_half_dn4, var_lod_half_dn5, var_lod_half_dn6, var_lod_half_dn7, var_lod_half_dn8, var_lod_half_dn9, var_lod_half_dn10, var_lod_half_dn11, var_lod_half_dn14,)
    }
};
        var_lod_half = assign10820_e5773;
        var_lod_half_dn0 = assign10820_e5773_d_n0;
        var_lod_half_dn2 = assign10820_e5773_d_n2;
        var_lod_half_dn4 = assign10820_e5773_d_n4;
        var_lod_half_dn5 = assign10820_e5773_d_n5;
        var_lod_half_dn6 = assign10820_e5773_d_n6;
        var_lod_half_dn7 = assign10820_e5773_d_n7;
        var_lod_half_dn8 = assign10820_e5773_d_n8;
        var_lod_half_dn9 = assign10820_e5773_d_n9;
        var_lod_half_dn10 = assign10820_e5773_d_n10;
        var_lod_half_dn11 = assign10820_e5773_d_n11;
        var_lod_half_dn14 = assign10820_e5773_d_n14;
        var_lod_half_rv = 0.0;

        var_npexte = var_uc_npext;
        var_npexte_dn0 = 0.0;
        var_npexte_dn2 = 0.0;
        var_npexte_dn4 = 0.0;
        var_npexte_dn5 = 0.0;
        var_npexte_dn6 = 0.0;
        var_npexte_dn7 = 0.0;
        var_npexte_dn8 = 0.0;
        var_npexte_dn9 = 0.0;
        var_npexte_dn10 = 0.0;
        var_npexte_dn11 = 0.0;
        var_npexte_dn14 = 0.0;
        var_npexte_rv = 0.0;

        var_ef_mueph1 = var_uc_mueph1;
        var_ef_mueph1_dn0 = 0.0;
        var_ef_mueph1_dn2 = 0.0;
        var_ef_mueph1_dn4 = 0.0;
        var_ef_mueph1_dn5 = 0.0;
        var_ef_mueph1_dn6 = 0.0;
        var_ef_mueph1_dn7 = 0.0;
        var_ef_mueph1_dn8 = 0.0;
        var_ef_mueph1_dn9 = 0.0;
        var_ef_mueph1_dn10 = 0.0;
        var_ef_mueph1_dn11 = 0.0;
        var_ef_mueph1_dn14 = 0.0;
        var_ef_mueph1_rv = 0.0;

        var_ef_nsubp = var_uc_nsubp;
        var_ef_nsubp_dn0 = 0.0;
        var_ef_nsubp_dn2 = 0.0;
        var_ef_nsubp_dn4 = 0.0;
        var_ef_nsubp_dn5 = 0.0;
        var_ef_nsubp_dn6 = 0.0;
        var_ef_nsubp_dn7 = 0.0;
        var_ef_nsubp_dn8 = 0.0;
        var_ef_nsubp_dn9 = 0.0;
        var_ef_nsubp_dn10 = 0.0;
        var_ef_nsubp_dn11 = 0.0;
        var_ef_nsubp_dn14 = 0.0;
        var_ef_nsubp_rv = 0.0;

        var_ef_nsubc = var_uc_nsubc;
        var_ef_nsubc_dn0 = 0.0;
        var_ef_nsubc_dn2 = 0.0;
        var_ef_nsubc_dn4 = 0.0;
        var_ef_nsubc_dn5 = 0.0;
        var_ef_nsubc_dn6 = 0.0;
        var_ef_nsubc_dn7 = 0.0;
        var_ef_nsubc_dn8 = 0.0;
        var_ef_nsubc_dn9 = 0.0;
        var_ef_nsubc_dn10 = 0.0;
        var_ef_nsubc_dn11 = 0.0;
        var_ef_nsubc_dn14 = 0.0;
        var_ef_nsubc_rv = 0.0;

        let assign10870_e5782: f64 = if ((p.p32 == 1.0) && (var_nsubcdfm_given != 0.0)) { 1.0 } else { 0.0 };
        var_guard255 = assign10870_e5782;
        var_guard255_rv = 0.0;

        let (assign10890_e5803, assign10890_e5803_d_n0, assign10890_e5803_d_n2, assign10890_e5803_d_n4, assign10890_e5803_d_n5, assign10890_e5803_d_n6, assign10890_e5803_d_n7, assign10890_e5803_d_n8, assign10890_e5803_d_n9, assign10890_e5803_d_n10, assign10890_e5803_d_n11, assign10890_e5803_d_n14,) = {
    if (var_guard255 != 0.0) {
        let assign10890_e5794: f64 = (var_mks_nsubcdfm).ln();
        let assign10890_e5796: f64 = (var_ef_nsubc).ln();
        let assign10890_e5797: f64 = (assign10890_e5794 - assign10890_e5796);
        let assign10890_e5798: f64 = (p.p282 * assign10890_e5797);
        let assign10890_e5800: f64 = (assign10890_e5798 + 1.0);
        let assign10890_e5801: f64 = (var_ef_mueph1 * assign10890_e5800);
        (assign10890_e5801, ((var_ef_mueph1_dn0 * assign10890_e5800) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn0 / var_ef_nsubc))))), ((var_ef_mueph1_dn2 * assign10890_e5800) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn2 / var_ef_nsubc))))), ((var_ef_mueph1_dn4 * assign10890_e5800) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn4 / var_ef_nsubc))))), ((var_ef_mueph1_dn5 * assign10890_e5800) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn5 / var_ef_nsubc))))), ((var_ef_mueph1_dn6 * assign10890_e5800) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn6 / var_ef_nsubc))))), ((var_ef_mueph1_dn7 * assign10890_e5800) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn7 / var_ef_nsubc))))), ((var_ef_mueph1_dn8 * assign10890_e5800) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn8 / var_ef_nsubc))))), ((var_ef_mueph1_dn9 * assign10890_e5800) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn9 / var_ef_nsubc))))), ((var_ef_mueph1_dn10 * assign10890_e5800) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn10 / var_ef_nsubc))))), ((var_ef_mueph1_dn11 * assign10890_e5800) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn11 / var_ef_nsubc))))), ((var_ef_mueph1_dn14 * assign10890_e5800) + (var_ef_mueph1 * (p.p282 * (-(var_ef_nsubc_dn14 / var_ef_nsubc))))),)
    } else {
        (var_ef_mueph1, var_ef_mueph1_dn0, var_ef_mueph1_dn2, var_ef_mueph1_dn4, var_ef_mueph1_dn5, var_ef_mueph1_dn6, var_ef_mueph1_dn7, var_ef_mueph1_dn8, var_ef_mueph1_dn9, var_ef_mueph1_dn10, var_ef_mueph1_dn11, var_ef_mueph1_dn14,)
    }
};
        var_ef_mueph1 = assign10890_e5803;
        var_ef_mueph1_dn0 = assign10890_e5803_d_n0;
        var_ef_mueph1_dn2 = assign10890_e5803_d_n2;
        var_ef_mueph1_dn4 = assign10890_e5803_d_n4;
        var_ef_mueph1_dn5 = assign10890_e5803_d_n5;
        var_ef_mueph1_dn6 = assign10890_e5803_d_n6;
        var_ef_mueph1_dn7 = assign10890_e5803_d_n7;
        var_ef_mueph1_dn8 = assign10890_e5803_d_n8;
        var_ef_mueph1_dn9 = assign10890_e5803_d_n9;
        var_ef_mueph1_dn10 = assign10890_e5803_d_n10;
        var_ef_mueph1_dn11 = assign10890_e5803_d_n11;
        var_ef_mueph1_dn14 = assign10890_e5803_d_n14;
        var_ef_mueph1_rv = 0.0;

        let (assign10900_e5811, assign10900_e5811_d_n0, assign10900_e5811_d_n2, assign10900_e5811_d_n4, assign10900_e5811_d_n5, assign10900_e5811_d_n6, assign10900_e5811_d_n7, assign10900_e5811_d_n8, assign10900_e5811_d_n9, assign10900_e5811_d_n10, assign10900_e5811_d_n11, assign10900_e5811_d_n14,) = {
    if (var_guard255 != 0.0) {
        let assign10900_e5807: f64 = (var_ef_nsubp + var_mks_nsubcdfm);
        let assign10900_e5809: f64 = (assign10900_e5807 - var_ef_nsubc);
        (assign10900_e5809, (var_ef_nsubp_dn0 - var_ef_nsubc_dn0), (var_ef_nsubp_dn2 - var_ef_nsubc_dn2), (var_ef_nsubp_dn4 - var_ef_nsubc_dn4), (var_ef_nsubp_dn5 - var_ef_nsubc_dn5), (var_ef_nsubp_dn6 - var_ef_nsubc_dn6), (var_ef_nsubp_dn7 - var_ef_nsubc_dn7), (var_ef_nsubp_dn8 - var_ef_nsubc_dn8), (var_ef_nsubp_dn9 - var_ef_nsubc_dn9), (var_ef_nsubp_dn10 - var_ef_nsubc_dn10), (var_ef_nsubp_dn11 - var_ef_nsubc_dn11), (var_ef_nsubp_dn14 - var_ef_nsubc_dn14),)
    } else {
        (var_ef_nsubp, var_ef_nsubp_dn0, var_ef_nsubp_dn2, var_ef_nsubp_dn4, var_ef_nsubp_dn5, var_ef_nsubp_dn6, var_ef_nsubp_dn7, var_ef_nsubp_dn8, var_ef_nsubp_dn9, var_ef_nsubp_dn10, var_ef_nsubp_dn11, var_ef_nsubp_dn14,)
    }
};
        var_ef_nsubp = assign10900_e5811;
        var_ef_nsubp_dn0 = assign10900_e5811_d_n0;
        var_ef_nsubp_dn2 = assign10900_e5811_d_n2;
        var_ef_nsubp_dn4 = assign10900_e5811_d_n4;
        var_ef_nsubp_dn5 = assign10900_e5811_d_n5;
        var_ef_nsubp_dn6 = assign10900_e5811_d_n6;
        var_ef_nsubp_dn7 = assign10900_e5811_d_n7;
        var_ef_nsubp_dn8 = assign10900_e5811_d_n8;
        var_ef_nsubp_dn9 = assign10900_e5811_d_n9;
        var_ef_nsubp_dn10 = assign10900_e5811_d_n10;
        var_ef_nsubp_dn11 = assign10900_e5811_d_n11;
        var_ef_nsubp_dn14 = assign10900_e5811_d_n14;
        var_ef_nsubp_rv = 0.0;

        *var_cecox_slot = var_cecox;
        *var_cecox_rv_slot = var_cecox_rv;
        *var_clmmod_slot = var_clmmod;
        *var_clmmod_rv_slot = var_clmmod_rv;
        *var_cnstpgd_slot = var_cnstpgd;
        *var_cnstpgd_rv_slot = var_cnstpgd_rv;
        *var_ef_mueph1_slot = var_ef_mueph1;
        *var_ef_mueph1_dn0_slot = var_ef_mueph1_dn0;
        *var_ef_mueph1_dn10_slot = var_ef_mueph1_dn10;
        *var_ef_mueph1_dn11_slot = var_ef_mueph1_dn11;
        *var_ef_mueph1_dn14_slot = var_ef_mueph1_dn14;
        *var_ef_mueph1_dn2_slot = var_ef_mueph1_dn2;
        *var_ef_mueph1_dn4_slot = var_ef_mueph1_dn4;
        *var_ef_mueph1_dn5_slot = var_ef_mueph1_dn5;
        *var_ef_mueph1_dn6_slot = var_ef_mueph1_dn6;
        *var_ef_mueph1_dn7_slot = var_ef_mueph1_dn7;
        *var_ef_mueph1_dn8_slot = var_ef_mueph1_dn8;
        *var_ef_mueph1_dn9_slot = var_ef_mueph1_dn9;
        *var_ef_mueph1_rv_slot = var_ef_mueph1_rv;
        *var_ef_nsubc_slot = var_ef_nsubc;
        *var_ef_nsubc_dn0_slot = var_ef_nsubc_dn0;
        *var_ef_nsubc_dn10_slot = var_ef_nsubc_dn10;
        *var_ef_nsubc_dn11_slot = var_ef_nsubc_dn11;
        *var_ef_nsubc_dn14_slot = var_ef_nsubc_dn14;
        *var_ef_nsubc_dn2_slot = var_ef_nsubc_dn2;
        *var_ef_nsubc_dn4_slot = var_ef_nsubc_dn4;
        *var_ef_nsubc_dn5_slot = var_ef_nsubc_dn5;
        *var_ef_nsubc_dn6_slot = var_ef_nsubc_dn6;
        *var_ef_nsubc_dn7_slot = var_ef_nsubc_dn7;
        *var_ef_nsubc_dn8_slot = var_ef_nsubc_dn8;
        *var_ef_nsubc_dn9_slot = var_ef_nsubc_dn9;
        *var_ef_nsubc_rv_slot = var_ef_nsubc_rv;
        *var_ef_nsubp_slot = var_ef_nsubp;
        *var_ef_nsubp_dn0_slot = var_ef_nsubp_dn0;
        *var_ef_nsubp_dn10_slot = var_ef_nsubp_dn10;
        *var_ef_nsubp_dn11_slot = var_ef_nsubp_dn11;
        *var_ef_nsubp_dn14_slot = var_ef_nsubp_dn14;
        *var_ef_nsubp_dn2_slot = var_ef_nsubp_dn2;
        *var_ef_nsubp_dn4_slot = var_ef_nsubp_dn4;
        *var_ef_nsubp_dn5_slot = var_ef_nsubp_dn5;
        *var_ef_nsubp_dn6_slot = var_ef_nsubp_dn6;
        *var_ef_nsubp_dn7_slot = var_ef_nsubp_dn7;
        *var_ef_nsubp_dn8_slot = var_ef_nsubp_dn8;
        *var_ef_nsubp_dn9_slot = var_ef_nsubp_dn9;
        *var_ef_nsubp_rv_slot = var_ef_nsubp_rv;
        *var_egtnom_slot = var_egtnom;
        *var_egtnom_rv_slot = var_egtnom_rv;
        *var_flg_nqs_slot = var_flg_nqs;
        *var_flg_nqs_rv_slot = var_flg_nqs_rv;
        *var_flg_pgd_slot = var_flg_pgd;
        *var_flg_pgd_rv_slot = var_flg_pgd_rv;
        *var_flg_qmetemp_slot = var_flg_qmetemp;
        *var_flg_qmetemp_rv_slot = var_flg_qmetemp_rv;
        *var_flg_qy_slot = var_flg_qy;
        *var_flg_qy_rv_slot = var_flg_qy_rv;
        *var_guard252_slot = var_guard252;
        *var_guard252_rv_slot = var_guard252_rv;
        *var_guard253_slot = var_guard253;
        *var_guard253_rv_slot = var_guard253_rv;
        *var_guard254_slot = var_guard254;
        *var_guard254_rv_slot = var_guard254_rv;
        *var_guard255_slot = var_guard255;
        *var_guard255_rv_slot = var_guard255_rv;
        *var_i_slot = var_i;
        *var_i_rv_slot = var_i_rv;
        *var_lod_half_slot = var_lod_half;
        *var_lod_half_dn0_slot = var_lod_half_dn0;
        *var_lod_half_dn10_slot = var_lod_half_dn10;
        *var_lod_half_dn11_slot = var_lod_half_dn11;
        *var_lod_half_dn14_slot = var_lod_half_dn14;
        *var_lod_half_dn2_slot = var_lod_half_dn2;
        *var_lod_half_dn4_slot = var_lod_half_dn4;
        *var_lod_half_dn5_slot = var_lod_half_dn5;
        *var_lod_half_dn6_slot = var_lod_half_dn6;
        *var_lod_half_dn7_slot = var_lod_half_dn7;
        *var_lod_half_dn8_slot = var_lod_half_dn8;
        *var_lod_half_dn9_slot = var_lod_half_dn9;
        *var_lod_half_ref_slot = var_lod_half_ref;
        *var_lod_half_ref_dn0_slot = var_lod_half_ref_dn0;
        *var_lod_half_ref_dn10_slot = var_lod_half_ref_dn10;
        *var_lod_half_ref_dn11_slot = var_lod_half_ref_dn11;
        *var_lod_half_ref_dn14_slot = var_lod_half_ref_dn14;
        *var_lod_half_ref_dn2_slot = var_lod_half_ref_dn2;
        *var_lod_half_ref_dn4_slot = var_lod_half_ref_dn4;
        *var_lod_half_ref_dn5_slot = var_lod_half_ref_dn5;
        *var_lod_half_ref_dn6_slot = var_lod_half_ref_dn6;
        *var_lod_half_ref_dn7_slot = var_lod_half_ref_dn7;
        *var_lod_half_ref_dn8_slot = var_lod_half_ref_dn8;
        *var_lod_half_ref_dn9_slot = var_lod_half_ref_dn9;
        *var_lod_half_ref_rv_slot = var_lod_half_ref_rv;
        *var_lod_half_rv_slot = var_lod_half_rv;
        *var_mks_subld2_slot = var_mks_subld2;
        *var_mks_subld2_rv_slot = var_mks_subld2_rv;
        *var_msc_slot = var_msc;
        *var_msc_rv_slot = var_msc_rv;
        *var_npexte_slot = var_npexte;
        *var_npexte_dn0_slot = var_npexte_dn0;
        *var_npexte_dn10_slot = var_npexte_dn10;
        *var_npexte_dn11_slot = var_npexte_dn11;
        *var_npexte_dn14_slot = var_npexte_dn14;
        *var_npexte_dn2_slot = var_npexte_dn2;
        *var_npexte_dn4_slot = var_npexte_dn4;
        *var_npexte_dn5_slot = var_npexte_dn5;
        *var_npexte_dn6_slot = var_npexte_dn6;
        *var_npexte_dn7_slot = var_npexte_dn7;
        *var_npexte_dn8_slot = var_npexte_dn8;
        *var_npexte_dn9_slot = var_npexte_dn9;
        *var_npexte_rv_slot = var_npexte_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_uc_depvmax_slot = var_uc_depvmax;
        *var_uc_depvmax_dn0_slot = var_uc_depvmax_dn0;
        *var_uc_depvmax_dn10_slot = var_uc_depvmax_dn10;
        *var_uc_depvmax_dn11_slot = var_uc_depvmax_dn11;
        *var_uc_depvmax_dn14_slot = var_uc_depvmax_dn14;
        *var_uc_depvmax_dn2_slot = var_uc_depvmax_dn2;
        *var_uc_depvmax_dn4_slot = var_uc_depvmax_dn4;
        *var_uc_depvmax_dn5_slot = var_uc_depvmax_dn5;
        *var_uc_depvmax_dn6_slot = var_uc_depvmax_dn6;
        *var_uc_depvmax_dn7_slot = var_uc_depvmax_dn7;
        *var_uc_depvmax_dn8_slot = var_uc_depvmax_dn8;
        *var_uc_depvmax_dn9_slot = var_uc_depvmax_dn9;
        *var_uc_depvmax_rv_slot = var_uc_depvmax_rv;
        *var_uc_fn2_slot = var_uc_fn2;
        *var_uc_fn2_rv_slot = var_uc_fn2_rv;
        *var_uc_gdld_slot = var_uc_gdld;
        *var_uc_gdld_rv_slot = var_uc_gdld_rv;
        *var_uc_gidl1_slot = var_uc_gidl1;
        *var_uc_gidl1_rv_slot = var_uc_gidl1_rv;
        *var_uc_gidl2_slot = var_uc_gidl2;
        *var_uc_gidl2_rv_slot = var_uc_gidl2_rv;
        *var_uc_gleak2_slot = var_uc_gleak2;
        *var_uc_gleak2_rv_slot = var_uc_gleak2_rv;
        *var_uc_glkb2_slot = var_uc_glkb2;
        *var_uc_glkb2_rv_slot = var_uc_glkb2_rv;
        *var_uc_glksd1_slot = var_uc_glksd1;
        *var_uc_glksd1_rv_slot = var_uc_glksd1_rv;
        *var_uc_glksd2_slot = var_uc_glksd2;
        *var_uc_glksd2_rv_slot = var_uc_glksd2_rv;
        *var_uc_muesti1_slot = var_uc_muesti1;
        *var_uc_muesti1_rv_slot = var_uc_muesti1_rv;
        *var_uc_nfalp_slot = var_uc_nfalp;
        *var_uc_nfalp_rv_slot = var_uc_nfalp_rv;
        *var_uc_nover_slot = var_uc_nover;
        *var_uc_nover_rv_slot = var_uc_nover_rv;
        *var_uc_novers_slot = var_uc_novers;
        *var_uc_novers_rv_slot = var_uc_novers_rv;
        *var_uc_npext_slot = var_uc_npext;
        *var_uc_npext_rv_slot = var_uc_npext_rv;
        *var_uc_nsti_slot = var_uc_nsti;
        *var_uc_nsti_rv_slot = var_uc_nsti_rv;
        *var_uc_nsubc_slot = var_uc_nsubc;
        *var_uc_nsubc_rv_slot = var_uc_nsubc_rv;
        *var_uc_nsubp_slot = var_uc_nsubp;
        *var_uc_nsubp_rv_slot = var_uc_nsubp_rv;
        *var_uc_nsubpsti1_slot = var_uc_nsubpsti1;
        *var_uc_nsubpsti1_rv_slot = var_uc_nsubpsti1_rv;
        *var_uc_rd22_slot = var_uc_rd22;
        *var_uc_rd22_rv_slot = var_uc_rd22_rv;
        *var_uc_rd23_slot = var_uc_rd23;
        *var_uc_rd23_rv_slot = var_uc_rd23_rv;
        *var_uc_rd24_slot = var_uc_rd24;
        *var_uc_rd24_rv_slot = var_uc_rd24_rv;
        *var_uc_rdvd_slot = var_uc_rdvd;
        *var_uc_rdvd_rv_slot = var_uc_rdvd_rv;
        *var_uc_rth0_slot = var_uc_rth0;
        *var_uc_rth0_rv_slot = var_uc_rth0_rv;
        *var_uc_vfbover_slot = var_uc_vfbover;
        *var_uc_vfbover_rv_slot = var_uc_vfbover_rv;
        *var_uc_vmax_slot = var_uc_vmax;
        *var_uc_vmax_rv_slot = var_uc_vmax_rv;
        *var_uc_wfc_slot = var_uc_wfc;
        *var_uc_wfc_rv_slot = var_uc_wfc_rv;
        *var_wlg_slot = var_wlg;
        *var_wlg_rv_slot = var_wlg_rv;
    }

    pub(super) fn stamp_reactive_block_18(
        p: &Parameters,
        var_ef_mueph1: f64,
        var_ef_mueph1_dn0: f64,
        var_ef_mueph1_dn10: f64,
        var_ef_mueph1_dn11: f64,
        var_ef_mueph1_dn14: f64,
        var_ef_mueph1_dn2: f64,
        var_ef_mueph1_dn4: f64,
        var_ef_mueph1_dn5: f64,
        var_ef_mueph1_dn6: f64,
        var_ef_mueph1_dn7: f64,
        var_ef_mueph1_dn8: f64,
        var_ef_mueph1_dn9: f64,
        var_ef_nsubp: f64,
        var_ef_nsubp_dn0: f64,
        var_ef_nsubp_dn10: f64,
        var_ef_nsubp_dn11: f64,
        var_ef_nsubp_dn14: f64,
        var_ef_nsubp_dn2: f64,
        var_ef_nsubp_dn4: f64,
        var_ef_nsubp_dn5: f64,
        var_ef_nsubp_dn6: f64,
        var_ef_nsubp_dn7: f64,
        var_ef_nsubp_dn8: f64,
        var_ef_nsubp_dn9: f64,
        var_guard255: f64,
        var_lg: f64,
        var_lgate: f64,
        var_lod_half: f64,
        var_lod_half_dn0: f64,
        var_lod_half_dn10: f64,
        var_lod_half_dn11: f64,
        var_lod_half_dn14: f64,
        var_lod_half_dn2: f64,
        var_lod_half_dn4: f64,
        var_lod_half_dn5: f64,
        var_lod_half_dn6: f64,
        var_lod_half_dn7: f64,
        var_lod_half_dn8: f64,
        var_lod_half_dn9: f64,
        var_lod_half_ref: f64,
        var_lod_half_ref_dn0: f64,
        var_lod_half_ref_dn10: f64,
        var_lod_half_ref_dn11: f64,
        var_lod_half_ref_dn14: f64,
        var_lod_half_ref_dn2: f64,
        var_lod_half_ref_dn4: f64,
        var_lod_half_ref_dn5: f64,
        var_lod_half_ref_dn6: f64,
        var_lod_half_ref_dn7: f64,
        var_lod_half_ref_dn8: f64,
        var_lod_half_ref_dn9: f64,
        var_mks_ll: f64,
        var_mks_nsubcdfm: f64,
        var_mks_wl: f64,
        var_uc_muesti1: f64,
        var_uc_muesti2: f64,
        var_uc_muesti3: f64,
        var_uc_ndep: f64,
        var_uc_ninv: f64,
        var_uc_wl2: f64,
        var_uc_xldld: f64,
        var_wg: f64,
        var_wgate: f64,
        var_wlg: f64,
        var_dl_slot: &mut f64,
        var_dl_rv_slot: &mut f64,
        var_dlld_slot: &mut f64,
        var_dlld_rv_slot: &mut f64,
        var_dvthsm_slot: &mut f64,
        var_dvthsm_rv_slot: &mut f64,
        var_dw_slot: &mut f64,
        var_dw_rv_slot: &mut f64,
        var_dwcv_slot: &mut f64,
        var_dwcv_rv_slot: &mut f64,
        var_dwld_slot: &mut f64,
        var_dwld_rv_slot: &mut f64,
        var_ef_nsubc_slot: &mut f64,
        var_ef_nsubc_dn0_slot: &mut f64,
        var_ef_nsubc_dn10_slot: &mut f64,
        var_ef_nsubc_dn11_slot: &mut f64,
        var_ef_nsubc_dn14_slot: &mut f64,
        var_ef_nsubc_dn2_slot: &mut f64,
        var_ef_nsubc_dn4_slot: &mut f64,
        var_ef_nsubc_dn5_slot: &mut f64,
        var_ef_nsubc_dn6_slot: &mut f64,
        var_ef_nsubc_dn7_slot: &mut f64,
        var_ef_nsubc_dn8_slot: &mut f64,
        var_ef_nsubc_dn9_slot: &mut f64,
        var_ef_nsubc_rv_slot: &mut f64,
        var_guard257_slot: &mut f64,
        var_guard257_rv_slot: &mut f64,
        var_guard265_slot: &mut f64,
        var_guard265_rv_slot: &mut f64,
        var_leff_slot: &mut f64,
        var_leff_rv_slot: &mut f64,
        var_lgatesm_slot: &mut f64,
        var_lgatesm_rv_slot: &mut f64,
        var_mueph_slot: &mut f64,
        var_mueph_dn0_slot: &mut f64,
        var_mueph_dn10_slot: &mut f64,
        var_mueph_dn11_slot: &mut f64,
        var_mueph_dn14_slot: &mut f64,
        var_mueph_dn2_slot: &mut f64,
        var_mueph_dn4_slot: &mut f64,
        var_mueph_dn5_slot: &mut f64,
        var_mueph_dn6_slot: &mut f64,
        var_mueph_dn7_slot: &mut f64,
        var_mueph_dn8_slot: &mut f64,
        var_mueph_dn9_slot: &mut f64,
        var_mueph_rv_slot: &mut f64,
        var_muesr_slot: &mut f64,
        var_muesr_rv_slot: &mut f64,
        var_ndep_o_esi_slot: &mut f64,
        var_ndep_o_esi_dn0_slot: &mut f64,
        var_ndep_o_esi_dn10_slot: &mut f64,
        var_ndep_o_esi_dn11_slot: &mut f64,
        var_ndep_o_esi_dn14_slot: &mut f64,
        var_ndep_o_esi_dn2_slot: &mut f64,
        var_ndep_o_esi_dn4_slot: &mut f64,
        var_ndep_o_esi_dn5_slot: &mut f64,
        var_ndep_o_esi_dn6_slot: &mut f64,
        var_ndep_o_esi_dn7_slot: &mut f64,
        var_ndep_o_esi_dn8_slot: &mut f64,
        var_ndep_o_esi_dn9_slot: &mut f64,
        var_ndep_o_esi_rv_slot: &mut f64,
        var_ninv_o_esi_slot: &mut f64,
        var_ninv_o_esi_rv_slot: &mut f64,
        var_ninvd0_slot: &mut f64,
        var_ninvd0_rv_slot: &mut f64,
        var_ninvd0cres_slot: &mut f64,
        var_ninvd0cres_dn0_slot: &mut f64,
        var_ninvd0cres_dn10_slot: &mut f64,
        var_ninvd0cres_dn11_slot: &mut f64,
        var_ninvd0cres_dn14_slot: &mut f64,
        var_ninvd0cres_dn2_slot: &mut f64,
        var_ninvd0cres_dn4_slot: &mut f64,
        var_ninvd0cres_dn5_slot: &mut f64,
        var_ninvd0cres_dn6_slot: &mut f64,
        var_ninvd0cres_dn7_slot: &mut f64,
        var_ninvd0cres_dn8_slot: &mut f64,
        var_ninvd0cres_dn9_slot: &mut f64,
        var_ninvd0cres_rv_slot: &mut f64,
        var_ninvd0hres_slot: &mut f64,
        var_ninvd0hres_dn0_slot: &mut f64,
        var_ninvd0hres_dn10_slot: &mut f64,
        var_ninvd0hres_dn11_slot: &mut f64,
        var_ninvd0hres_dn14_slot: &mut f64,
        var_ninvd0hres_dn2_slot: &mut f64,
        var_ninvd0hres_dn4_slot: &mut f64,
        var_ninvd0hres_dn5_slot: &mut f64,
        var_ninvd0hres_dn6_slot: &mut f64,
        var_ninvd0hres_dn7_slot: &mut f64,
        var_ninvd0hres_dn8_slot: &mut f64,
        var_ninvd0hres_dn9_slot: &mut f64,
        var_ninvd0hres_rv_slot: &mut f64,
        var_npexte_slot: &mut f64,
        var_npexte_dn0_slot: &mut f64,
        var_npexte_dn10_slot: &mut f64,
        var_npexte_dn11_slot: &mut f64,
        var_npexte_dn14_slot: &mut f64,
        var_npexte_dn2_slot: &mut f64,
        var_npexte_dn4_slot: &mut f64,
        var_npexte_dn5_slot: &mut f64,
        var_npexte_dn6_slot: &mut f64,
        var_npexte_dn7_slot: &mut f64,
        var_npexte_dn8_slot: &mut f64,
        var_npexte_dn9_slot: &mut f64,
        var_npexte_rv_slot: &mut f64,
        var_nsubpp_slot: &mut f64,
        var_nsubpp_dn0_slot: &mut f64,
        var_nsubpp_dn10_slot: &mut f64,
        var_nsubpp_dn11_slot: &mut f64,
        var_nsubpp_dn14_slot: &mut f64,
        var_nsubpp_dn2_slot: &mut f64,
        var_nsubpp_dn4_slot: &mut f64,
        var_nsubpp_dn5_slot: &mut f64,
        var_nsubpp_dn6_slot: &mut f64,
        var_nsubpp_dn7_slot: &mut f64,
        var_nsubpp_dn8_slot: &mut f64,
        var_nsubpp_dn9_slot: &mut f64,
        var_nsubpp_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn14_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_uc_wsti_slot: &mut f64,
        var_uc_wsti_dn0_slot: &mut f64,
        var_uc_wsti_dn10_slot: &mut f64,
        var_uc_wsti_dn11_slot: &mut f64,
        var_uc_wsti_dn14_slot: &mut f64,
        var_uc_wsti_dn2_slot: &mut f64,
        var_uc_wsti_dn4_slot: &mut f64,
        var_uc_wsti_dn5_slot: &mut f64,
        var_uc_wsti_dn6_slot: &mut f64,
        var_uc_wsti_dn7_slot: &mut f64,
        var_uc_wsti_dn8_slot: &mut f64,
        var_uc_wsti_dn9_slot: &mut f64,
        var_uc_wsti_rv_slot: &mut f64,
        var_weff_slot: &mut f64,
        var_weff_cv_slot: &mut f64,
        var_weff_cv_rv_slot: &mut f64,
        var_weff_ld_slot: &mut f64,
        var_weff_ld_rv_slot: &mut f64,
        var_weff_nf_slot: &mut f64,
        var_weff_nf_rv_slot: &mut f64,
        var_weff_rv_slot: &mut f64,
        var_weffcv_nf_slot: &mut f64,
        var_weffcv_nf_rv_slot: &mut f64,
    ) {
        let mut var_dl: f64 = *var_dl_slot;
        let mut var_dl_rv: f64 = *var_dl_rv_slot;
        let mut var_dlld: f64 = *var_dlld_slot;
        let mut var_dlld_rv: f64 = *var_dlld_rv_slot;
        let mut var_dvthsm: f64 = *var_dvthsm_slot;
        let mut var_dvthsm_rv: f64 = *var_dvthsm_rv_slot;
        let mut var_dw: f64 = *var_dw_slot;
        let mut var_dw_rv: f64 = *var_dw_rv_slot;
        let mut var_dwcv: f64 = *var_dwcv_slot;
        let mut var_dwcv_rv: f64 = *var_dwcv_rv_slot;
        let mut var_dwld: f64 = *var_dwld_slot;
        let mut var_dwld_rv: f64 = *var_dwld_rv_slot;
        let mut var_ef_nsubc: f64 = *var_ef_nsubc_slot;
        let mut var_ef_nsubc_dn0: f64 = *var_ef_nsubc_dn0_slot;
        let mut var_ef_nsubc_dn10: f64 = *var_ef_nsubc_dn10_slot;
        let mut var_ef_nsubc_dn11: f64 = *var_ef_nsubc_dn11_slot;
        let mut var_ef_nsubc_dn14: f64 = *var_ef_nsubc_dn14_slot;
        let mut var_ef_nsubc_dn2: f64 = *var_ef_nsubc_dn2_slot;
        let mut var_ef_nsubc_dn4: f64 = *var_ef_nsubc_dn4_slot;
        let mut var_ef_nsubc_dn5: f64 = *var_ef_nsubc_dn5_slot;
        let mut var_ef_nsubc_dn6: f64 = *var_ef_nsubc_dn6_slot;
        let mut var_ef_nsubc_dn7: f64 = *var_ef_nsubc_dn7_slot;
        let mut var_ef_nsubc_dn8: f64 = *var_ef_nsubc_dn8_slot;
        let mut var_ef_nsubc_dn9: f64 = *var_ef_nsubc_dn9_slot;
        let mut var_ef_nsubc_rv: f64 = *var_ef_nsubc_rv_slot;
        let mut var_guard257: f64 = *var_guard257_slot;
        let mut var_guard257_rv: f64 = *var_guard257_rv_slot;
        let mut var_guard265: f64 = *var_guard265_slot;
        let mut var_guard265_rv: f64 = *var_guard265_rv_slot;
        let mut var_leff: f64 = *var_leff_slot;
        let mut var_leff_rv: f64 = *var_leff_rv_slot;
        let mut var_lgatesm: f64 = *var_lgatesm_slot;
        let mut var_lgatesm_rv: f64 = *var_lgatesm_rv_slot;
        let mut var_mueph: f64 = *var_mueph_slot;
        let mut var_mueph_dn0: f64 = *var_mueph_dn0_slot;
        let mut var_mueph_dn10: f64 = *var_mueph_dn10_slot;
        let mut var_mueph_dn11: f64 = *var_mueph_dn11_slot;
        let mut var_mueph_dn14: f64 = *var_mueph_dn14_slot;
        let mut var_mueph_dn2: f64 = *var_mueph_dn2_slot;
        let mut var_mueph_dn4: f64 = *var_mueph_dn4_slot;
        let mut var_mueph_dn5: f64 = *var_mueph_dn5_slot;
        let mut var_mueph_dn6: f64 = *var_mueph_dn6_slot;
        let mut var_mueph_dn7: f64 = *var_mueph_dn7_slot;
        let mut var_mueph_dn8: f64 = *var_mueph_dn8_slot;
        let mut var_mueph_dn9: f64 = *var_mueph_dn9_slot;
        let mut var_mueph_rv: f64 = *var_mueph_rv_slot;
        let mut var_muesr: f64 = *var_muesr_slot;
        let mut var_muesr_rv: f64 = *var_muesr_rv_slot;
        let mut var_ndep_o_esi: f64 = *var_ndep_o_esi_slot;
        let mut var_ndep_o_esi_dn0: f64 = *var_ndep_o_esi_dn0_slot;
        let mut var_ndep_o_esi_dn10: f64 = *var_ndep_o_esi_dn10_slot;
        let mut var_ndep_o_esi_dn11: f64 = *var_ndep_o_esi_dn11_slot;
        let mut var_ndep_o_esi_dn14: f64 = *var_ndep_o_esi_dn14_slot;
        let mut var_ndep_o_esi_dn2: f64 = *var_ndep_o_esi_dn2_slot;
        let mut var_ndep_o_esi_dn4: f64 = *var_ndep_o_esi_dn4_slot;
        let mut var_ndep_o_esi_dn5: f64 = *var_ndep_o_esi_dn5_slot;
        let mut var_ndep_o_esi_dn6: f64 = *var_ndep_o_esi_dn6_slot;
        let mut var_ndep_o_esi_dn7: f64 = *var_ndep_o_esi_dn7_slot;
        let mut var_ndep_o_esi_dn8: f64 = *var_ndep_o_esi_dn8_slot;
        let mut var_ndep_o_esi_dn9: f64 = *var_ndep_o_esi_dn9_slot;
        let mut var_ndep_o_esi_rv: f64 = *var_ndep_o_esi_rv_slot;
        let mut var_ninv_o_esi: f64 = *var_ninv_o_esi_slot;
        let mut var_ninv_o_esi_rv: f64 = *var_ninv_o_esi_rv_slot;
        let mut var_ninvd0: f64 = *var_ninvd0_slot;
        let mut var_ninvd0_rv: f64 = *var_ninvd0_rv_slot;
        let mut var_ninvd0cres: f64 = *var_ninvd0cres_slot;
        let mut var_ninvd0cres_dn0: f64 = *var_ninvd0cres_dn0_slot;
        let mut var_ninvd0cres_dn10: f64 = *var_ninvd0cres_dn10_slot;
        let mut var_ninvd0cres_dn11: f64 = *var_ninvd0cres_dn11_slot;
        let mut var_ninvd0cres_dn14: f64 = *var_ninvd0cres_dn14_slot;
        let mut var_ninvd0cres_dn2: f64 = *var_ninvd0cres_dn2_slot;
        let mut var_ninvd0cres_dn4: f64 = *var_ninvd0cres_dn4_slot;
        let mut var_ninvd0cres_dn5: f64 = *var_ninvd0cres_dn5_slot;
        let mut var_ninvd0cres_dn6: f64 = *var_ninvd0cres_dn6_slot;
        let mut var_ninvd0cres_dn7: f64 = *var_ninvd0cres_dn7_slot;
        let mut var_ninvd0cres_dn8: f64 = *var_ninvd0cres_dn8_slot;
        let mut var_ninvd0cres_dn9: f64 = *var_ninvd0cres_dn9_slot;
        let mut var_ninvd0cres_rv: f64 = *var_ninvd0cres_rv_slot;
        let mut var_ninvd0hres: f64 = *var_ninvd0hres_slot;
        let mut var_ninvd0hres_dn0: f64 = *var_ninvd0hres_dn0_slot;
        let mut var_ninvd0hres_dn10: f64 = *var_ninvd0hres_dn10_slot;
        let mut var_ninvd0hres_dn11: f64 = *var_ninvd0hres_dn11_slot;
        let mut var_ninvd0hres_dn14: f64 = *var_ninvd0hres_dn14_slot;
        let mut var_ninvd0hres_dn2: f64 = *var_ninvd0hres_dn2_slot;
        let mut var_ninvd0hres_dn4: f64 = *var_ninvd0hres_dn4_slot;
        let mut var_ninvd0hres_dn5: f64 = *var_ninvd0hres_dn5_slot;
        let mut var_ninvd0hres_dn6: f64 = *var_ninvd0hres_dn6_slot;
        let mut var_ninvd0hres_dn7: f64 = *var_ninvd0hres_dn7_slot;
        let mut var_ninvd0hres_dn8: f64 = *var_ninvd0hres_dn8_slot;
        let mut var_ninvd0hres_dn9: f64 = *var_ninvd0hres_dn9_slot;
        let mut var_ninvd0hres_rv: f64 = *var_ninvd0hres_rv_slot;
        let mut var_npexte: f64 = *var_npexte_slot;
        let mut var_npexte_dn0: f64 = *var_npexte_dn0_slot;
        let mut var_npexte_dn10: f64 = *var_npexte_dn10_slot;
        let mut var_npexte_dn11: f64 = *var_npexte_dn11_slot;
        let mut var_npexte_dn14: f64 = *var_npexte_dn14_slot;
        let mut var_npexte_dn2: f64 = *var_npexte_dn2_slot;
        let mut var_npexte_dn4: f64 = *var_npexte_dn4_slot;
        let mut var_npexte_dn5: f64 = *var_npexte_dn5_slot;
        let mut var_npexte_dn6: f64 = *var_npexte_dn6_slot;
        let mut var_npexte_dn7: f64 = *var_npexte_dn7_slot;
        let mut var_npexte_dn8: f64 = *var_npexte_dn8_slot;
        let mut var_npexte_dn9: f64 = *var_npexte_dn9_slot;
        let mut var_npexte_rv: f64 = *var_npexte_rv_slot;
        let mut var_nsubpp: f64 = *var_nsubpp_slot;
        let mut var_nsubpp_dn0: f64 = *var_nsubpp_dn0_slot;
        let mut var_nsubpp_dn10: f64 = *var_nsubpp_dn10_slot;
        let mut var_nsubpp_dn11: f64 = *var_nsubpp_dn11_slot;
        let mut var_nsubpp_dn14: f64 = *var_nsubpp_dn14_slot;
        let mut var_nsubpp_dn2: f64 = *var_nsubpp_dn2_slot;
        let mut var_nsubpp_dn4: f64 = *var_nsubpp_dn4_slot;
        let mut var_nsubpp_dn5: f64 = *var_nsubpp_dn5_slot;
        let mut var_nsubpp_dn6: f64 = *var_nsubpp_dn6_slot;
        let mut var_nsubpp_dn7: f64 = *var_nsubpp_dn7_slot;
        let mut var_nsubpp_dn8: f64 = *var_nsubpp_dn8_slot;
        let mut var_nsubpp_dn9: f64 = *var_nsubpp_dn9_slot;
        let mut var_nsubpp_rv: f64 = *var_nsubpp_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn14: f64 = *var_t3_dn14_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_uc_wsti: f64 = *var_uc_wsti_slot;
        let mut var_uc_wsti_dn0: f64 = *var_uc_wsti_dn0_slot;
        let mut var_uc_wsti_dn10: f64 = *var_uc_wsti_dn10_slot;
        let mut var_uc_wsti_dn11: f64 = *var_uc_wsti_dn11_slot;
        let mut var_uc_wsti_dn14: f64 = *var_uc_wsti_dn14_slot;
        let mut var_uc_wsti_dn2: f64 = *var_uc_wsti_dn2_slot;
        let mut var_uc_wsti_dn4: f64 = *var_uc_wsti_dn4_slot;
        let mut var_uc_wsti_dn5: f64 = *var_uc_wsti_dn5_slot;
        let mut var_uc_wsti_dn6: f64 = *var_uc_wsti_dn6_slot;
        let mut var_uc_wsti_dn7: f64 = *var_uc_wsti_dn7_slot;
        let mut var_uc_wsti_dn8: f64 = *var_uc_wsti_dn8_slot;
        let mut var_uc_wsti_dn9: f64 = *var_uc_wsti_dn9_slot;
        let mut var_uc_wsti_rv: f64 = *var_uc_wsti_rv_slot;
        let mut var_weff: f64 = *var_weff_slot;
        let mut var_weff_cv: f64 = *var_weff_cv_slot;
        let mut var_weff_cv_rv: f64 = *var_weff_cv_rv_slot;
        let mut var_weff_ld: f64 = *var_weff_ld_slot;
        let mut var_weff_ld_rv: f64 = *var_weff_ld_rv_slot;
        let mut var_weff_nf: f64 = *var_weff_nf_slot;
        let mut var_weff_nf_rv: f64 = *var_weff_nf_rv_slot;
        let mut var_weff_rv: f64 = *var_weff_rv_slot;
        let mut var_weffcv_nf: f64 = *var_weffcv_nf_slot;
        let mut var_weffcv_nf_rv: f64 = *var_weffcv_nf_rv_slot;

        let (assign10910_e5819, assign10910_e5819_d_n0, assign10910_e5819_d_n2, assign10910_e5819_d_n4, assign10910_e5819_d_n5, assign10910_e5819_d_n6, assign10910_e5819_d_n7, assign10910_e5819_d_n8, assign10910_e5819_d_n9, assign10910_e5819_d_n10, assign10910_e5819_d_n11, assign10910_e5819_d_n14,) = {
    if (var_guard255 != 0.0) {
        let assign10910_e5815: f64 = (var_npexte + var_mks_nsubcdfm);
        let assign10910_e5817: f64 = (assign10910_e5815 - var_ef_nsubc);
        (assign10910_e5817, (var_npexte_dn0 - var_ef_nsubc_dn0), (var_npexte_dn2 - var_ef_nsubc_dn2), (var_npexte_dn4 - var_ef_nsubc_dn4), (var_npexte_dn5 - var_ef_nsubc_dn5), (var_npexte_dn6 - var_ef_nsubc_dn6), (var_npexte_dn7 - var_ef_nsubc_dn7), (var_npexte_dn8 - var_ef_nsubc_dn8), (var_npexte_dn9 - var_ef_nsubc_dn9), (var_npexte_dn10 - var_ef_nsubc_dn10), (var_npexte_dn11 - var_ef_nsubc_dn11), (var_npexte_dn14 - var_ef_nsubc_dn14),)
    } else {
        (var_npexte, var_npexte_dn0, var_npexte_dn2, var_npexte_dn4, var_npexte_dn5, var_npexte_dn6, var_npexte_dn7, var_npexte_dn8, var_npexte_dn9, var_npexte_dn10, var_npexte_dn11, var_npexte_dn14,)
    }
};
        var_npexte = assign10910_e5819;
        var_npexte_dn0 = assign10910_e5819_d_n0;
        var_npexte_dn2 = assign10910_e5819_d_n2;
        var_npexte_dn4 = assign10910_e5819_d_n4;
        var_npexte_dn5 = assign10910_e5819_d_n5;
        var_npexte_dn6 = assign10910_e5819_d_n6;
        var_npexte_dn7 = assign10910_e5819_d_n7;
        var_npexte_dn8 = assign10910_e5819_d_n8;
        var_npexte_dn9 = assign10910_e5819_d_n9;
        var_npexte_dn10 = assign10910_e5819_d_n10;
        var_npexte_dn11 = assign10910_e5819_d_n11;
        var_npexte_dn14 = assign10910_e5819_d_n14;
        var_npexte_rv = 0.0;

        let (assign10920_e5823, assign10920_e5823_d_n0, assign10920_e5823_d_n2, assign10920_e5823_d_n4, assign10920_e5823_d_n5, assign10920_e5823_d_n6, assign10920_e5823_d_n7, assign10920_e5823_d_n8, assign10920_e5823_d_n9, assign10920_e5823_d_n10, assign10920_e5823_d_n11, assign10920_e5823_d_n14,) = {
    if (var_guard255 != 0.0) {
        (var_mks_nsubcdfm, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ef_nsubc, var_ef_nsubc_dn0, var_ef_nsubc_dn2, var_ef_nsubc_dn4, var_ef_nsubc_dn5, var_ef_nsubc_dn6, var_ef_nsubc_dn7, var_ef_nsubc_dn8, var_ef_nsubc_dn9, var_ef_nsubc_dn10, var_ef_nsubc_dn11, var_ef_nsubc_dn14,)
    }
};
        var_ef_nsubc = assign10920_e5823;
        var_ef_nsubc_dn0 = assign10920_e5823_d_n0;
        var_ef_nsubc_dn2 = assign10920_e5823_d_n2;
        var_ef_nsubc_dn4 = assign10920_e5823_d_n4;
        var_ef_nsubc_dn5 = assign10920_e5823_d_n5;
        var_ef_nsubc_dn6 = assign10920_e5823_d_n6;
        var_ef_nsubc_dn7 = assign10920_e5823_d_n7;
        var_ef_nsubc_dn8 = assign10920_e5823_d_n8;
        var_ef_nsubc_dn9 = assign10920_e5823_d_n9;
        var_ef_nsubc_dn10 = assign10920_e5823_d_n10;
        var_ef_nsubc_dn11 = assign10920_e5823_d_n11;
        var_ef_nsubc_dn14 = assign10920_e5823_d_n14;
        var_ef_nsubc_rv = 0.0;

        let assign10930_e5829: f64 = (var_wg).powf(p.p163);
        let assign10930_e5830: f64 = (p.p162 / assign10930_e5829);
        let assign10930_e5831: f64 = (1.0 + assign10930_e5830);
        let assign10930_e5832: f64 = (var_ef_mueph1 * assign10930_e5831);
        let assign10930_e5837: f64 = (var_lg).powf(p.p165);
        let assign10930_e5838: f64 = (p.p164 / assign10930_e5837);
        let assign10930_e5839: f64 = (1.0 + assign10930_e5838);
        let assign10930_e5840: f64 = (assign10930_e5832 * assign10930_e5839);
        let assign10930_e5845: f64 = (var_wlg).powf(p.p168);
        let assign10930_e5846: f64 = (p.p167 / assign10930_e5845);
        let assign10930_e5847: f64 = (1.0 + assign10930_e5846);
        let assign10930_e5848: f64 = (assign10930_e5840 * assign10930_e5847);
        var_mueph = assign10930_e5848;
        var_mueph_dn0 = (((var_ef_mueph1_dn0 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        var_mueph_dn2 = (((var_ef_mueph1_dn2 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        var_mueph_dn4 = (((var_ef_mueph1_dn4 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        var_mueph_dn5 = (((var_ef_mueph1_dn5 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        var_mueph_dn6 = (((var_ef_mueph1_dn6 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        var_mueph_dn7 = (((var_ef_mueph1_dn7 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        var_mueph_dn8 = (((var_ef_mueph1_dn8 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        var_mueph_dn9 = (((var_ef_mueph1_dn9 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        var_mueph_dn10 = (((var_ef_mueph1_dn10 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        var_mueph_dn11 = (((var_ef_mueph1_dn11 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        var_mueph_dn14 = (((var_ef_mueph1_dn14 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        var_mueph_rv = 0.0;

        let assign10940_e5851: f64 = if var_lod_half > 0.0 { 1.0 } else { 0.0 };
        var_guard257 = assign10940_e5851;
        var_guard257_rv = 0.0;

        let (assign10950_e5859, assign10950_e5859_d_n0, assign10950_e5859_d_n2, assign10950_e5859_d_n4, assign10950_e5859_d_n5, assign10950_e5859_d_n6, assign10950_e5859_d_n7, assign10950_e5859_d_n8, assign10950_e5859_d_n9, assign10950_e5859_d_n10, assign10950_e5859_d_n11, assign10950_e5859_d_n14,) = {
    if (var_guard257 != 0.0) {
        let assign10950_e5856: f64 = (1.0 + var_uc_muesti2);
        let assign10950_e5857: f64 = (1.0 / assign10950_e5856);
        (assign10950_e5857, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign10950_e5859;
        var_t1_dn0 = assign10950_e5859_d_n0;
        var_t1_dn2 = assign10950_e5859_d_n2;
        var_t1_dn4 = assign10950_e5859_d_n4;
        var_t1_dn5 = assign10950_e5859_d_n5;
        var_t1_dn6 = assign10950_e5859_d_n6;
        var_t1_dn7 = assign10950_e5859_d_n7;
        var_t1_dn8 = assign10950_e5859_d_n8;
        var_t1_dn9 = assign10950_e5859_d_n9;
        var_t1_dn10 = assign10950_e5859_d_n10;
        var_t1_dn11 = assign10950_e5859_d_n11;
        var_t1_dn14 = assign10950_e5859_d_n14;
        var_t1_rv = 0.0;

        let (assign10960_e5867, assign10960_e5867_d_n0, assign10960_e5867_d_n2, assign10960_e5867_d_n4, assign10960_e5867_d_n5, assign10960_e5867_d_n6, assign10960_e5867_d_n7, assign10960_e5867_d_n8, assign10960_e5867_d_n9, assign10960_e5867_d_n10, assign10960_e5867_d_n11, assign10960_e5867_d_n14,) = {
    if (var_guard257 != 0.0) {
        let assign10960_e5863: f64 = (var_uc_muesti1 / var_lod_half);
        let assign10960_e5865: f64 = (assign10960_e5863).powf(var_uc_muesti3);
        (assign10960_e5865, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10960_e5863).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn0) / (var_lod_half * var_lod_half))))) } } else { (assign10960_e5865 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn0) / (var_lod_half * var_lod_half))) / assign10960_e5863))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10960_e5863).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn2) / (var_lod_half * var_lod_half))))) } } else { (assign10960_e5865 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn2) / (var_lod_half * var_lod_half))) / assign10960_e5863))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10960_e5863).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn4) / (var_lod_half * var_lod_half))))) } } else { (assign10960_e5865 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn4) / (var_lod_half * var_lod_half))) / assign10960_e5863))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10960_e5863).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn5) / (var_lod_half * var_lod_half))))) } } else { (assign10960_e5865 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn5) / (var_lod_half * var_lod_half))) / assign10960_e5863))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10960_e5863).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn6) / (var_lod_half * var_lod_half))))) } } else { (assign10960_e5865 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn6) / (var_lod_half * var_lod_half))) / assign10960_e5863))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10960_e5863).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn7) / (var_lod_half * var_lod_half))))) } } else { (assign10960_e5865 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn7) / (var_lod_half * var_lod_half))) / assign10960_e5863))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10960_e5863).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn8) / (var_lod_half * var_lod_half))))) } } else { (assign10960_e5865 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn8) / (var_lod_half * var_lod_half))) / assign10960_e5863))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10960_e5863).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn9) / (var_lod_half * var_lod_half))))) } } else { (assign10960_e5865 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn9) / (var_lod_half * var_lod_half))) / assign10960_e5863))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10960_e5863).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn10) / (var_lod_half * var_lod_half))))) } } else { (assign10960_e5865 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn10) / (var_lod_half * var_lod_half))) / assign10960_e5863))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10960_e5863).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn11) / (var_lod_half * var_lod_half))))) } } else { (assign10960_e5865 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn11) / (var_lod_half * var_lod_half))) / assign10960_e5863))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10960_e5863).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_dn14) / (var_lod_half * var_lod_half))))) } } else { (assign10960_e5865 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_dn14) / (var_lod_half * var_lod_half))) / assign10960_e5863))) },)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    }
};
        var_t2 = assign10960_e5867;
        var_t2_dn0 = assign10960_e5867_d_n0;
        var_t2_dn2 = assign10960_e5867_d_n2;
        var_t2_dn4 = assign10960_e5867_d_n4;
        var_t2_dn5 = assign10960_e5867_d_n5;
        var_t2_dn6 = assign10960_e5867_d_n6;
        var_t2_dn7 = assign10960_e5867_d_n7;
        var_t2_dn8 = assign10960_e5867_d_n8;
        var_t2_dn9 = assign10960_e5867_d_n9;
        var_t2_dn10 = assign10960_e5867_d_n10;
        var_t2_dn11 = assign10960_e5867_d_n11;
        var_t2_dn14 = assign10960_e5867_d_n14;
        var_t2_rv = 0.0;

        let (assign10970_e5875, assign10970_e5875_d_n0, assign10970_e5875_d_n2, assign10970_e5875_d_n4, assign10970_e5875_d_n5, assign10970_e5875_d_n6, assign10970_e5875_d_n7, assign10970_e5875_d_n8, assign10970_e5875_d_n9, assign10970_e5875_d_n10, assign10970_e5875_d_n11, assign10970_e5875_d_n14,) = {
    if (var_guard257 != 0.0) {
        let assign10970_e5871: f64 = (var_uc_muesti1 / var_lod_half_ref);
        let assign10970_e5873: f64 = (assign10970_e5871).powf(var_uc_muesti3);
        (assign10970_e5873, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10970_e5871).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn0) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10970_e5873 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn0) / (var_lod_half_ref * var_lod_half_ref))) / assign10970_e5871))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10970_e5871).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn2) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10970_e5873 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn2) / (var_lod_half_ref * var_lod_half_ref))) / assign10970_e5871))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10970_e5871).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn4) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10970_e5873 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn4) / (var_lod_half_ref * var_lod_half_ref))) / assign10970_e5871))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10970_e5871).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn5) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10970_e5873 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn5) / (var_lod_half_ref * var_lod_half_ref))) / assign10970_e5871))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10970_e5871).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn6) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10970_e5873 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn6) / (var_lod_half_ref * var_lod_half_ref))) / assign10970_e5871))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10970_e5871).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn7) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10970_e5873 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn7) / (var_lod_half_ref * var_lod_half_ref))) / assign10970_e5871))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10970_e5871).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn8) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10970_e5873 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn8) / (var_lod_half_ref * var_lod_half_ref))) / assign10970_e5871))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10970_e5871).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn9) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10970_e5873 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn9) / (var_lod_half_ref * var_lod_half_ref))) / assign10970_e5871))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10970_e5871).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn10) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10970_e5873 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn10) / (var_lod_half_ref * var_lod_half_ref))) / assign10970_e5871))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10970_e5871).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn11) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10970_e5873 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn11) / (var_lod_half_ref * var_lod_half_ref))) / assign10970_e5871))) }, if 0.0 == 0.0 && ((var_uc_muesti3) as f64).is_finite() && ((var_uc_muesti3) as f64).fract() == 0.0 { if var_uc_muesti3 == 0.0 { 0.0 } else { (var_uc_muesti3 * ((assign10970_e5871).powf(var_uc_muesti3 - 1.0) * (-((var_uc_muesti1 * var_lod_half_ref_dn14) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign10970_e5873 * (var_uc_muesti3 * ((-((var_uc_muesti1 * var_lod_half_ref_dn14) / (var_lod_half_ref * var_lod_half_ref))) / assign10970_e5871))) },)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn14,)
    }
};
        var_t3 = assign10970_e5875;
        var_t3_dn0 = assign10970_e5875_d_n0;
        var_t3_dn2 = assign10970_e5875_d_n2;
        var_t3_dn4 = assign10970_e5875_d_n4;
        var_t3_dn5 = assign10970_e5875_d_n5;
        var_t3_dn6 = assign10970_e5875_d_n6;
        var_t3_dn7 = assign10970_e5875_d_n7;
        var_t3_dn8 = assign10970_e5875_d_n8;
        var_t3_dn9 = assign10970_e5875_d_n9;
        var_t3_dn10 = assign10970_e5875_d_n10;
        var_t3_dn11 = assign10970_e5875_d_n11;
        var_t3_dn14 = assign10970_e5875_d_n14;
        var_t3_rv = 0.0;

        let (assign10980_e5891, assign10980_e5891_d_n0, assign10980_e5891_d_n2, assign10980_e5891_d_n4, assign10980_e5891_d_n5, assign10980_e5891_d_n6, assign10980_e5891_d_n7, assign10980_e5891_d_n8, assign10980_e5891_d_n9, assign10980_e5891_d_n10, assign10980_e5891_d_n11, assign10980_e5891_d_n14,) = {
    if (var_guard257 != 0.0) {
        let assign10980_e5881: f64 = (var_t1 * var_t2);
        let assign10980_e5882: f64 = (1.0 + assign10980_e5881);
        let assign10980_e5883: f64 = (var_mueph * assign10980_e5882);
        let assign10980_e5887: f64 = (var_t1 * var_t3);
        let assign10980_e5888: f64 = (1.0 + assign10980_e5887);
        let assign10980_e5889: f64 = (assign10980_e5883 / assign10980_e5888);
        (assign10980_e5889, (((((var_mueph_dn0 * assign10980_e5882) + (var_mueph * ((var_t1_dn0 * var_t2) + (var_t1 * var_t2_dn0)))) * assign10980_e5888) - (assign10980_e5883 * ((var_t1_dn0 * var_t3) + (var_t1 * var_t3_dn0)))) / (assign10980_e5888 * assign10980_e5888)), (((((var_mueph_dn2 * assign10980_e5882) + (var_mueph * ((var_t1_dn2 * var_t2) + (var_t1 * var_t2_dn2)))) * assign10980_e5888) - (assign10980_e5883 * ((var_t1_dn2 * var_t3) + (var_t1 * var_t3_dn2)))) / (assign10980_e5888 * assign10980_e5888)), (((((var_mueph_dn4 * assign10980_e5882) + (var_mueph * ((var_t1_dn4 * var_t2) + (var_t1 * var_t2_dn4)))) * assign10980_e5888) - (assign10980_e5883 * ((var_t1_dn4 * var_t3) + (var_t1 * var_t3_dn4)))) / (assign10980_e5888 * assign10980_e5888)), (((((var_mueph_dn5 * assign10980_e5882) + (var_mueph * ((var_t1_dn5 * var_t2) + (var_t1 * var_t2_dn5)))) * assign10980_e5888) - (assign10980_e5883 * ((var_t1_dn5 * var_t3) + (var_t1 * var_t3_dn5)))) / (assign10980_e5888 * assign10980_e5888)), (((((var_mueph_dn6 * assign10980_e5882) + (var_mueph * ((var_t1_dn6 * var_t2) + (var_t1 * var_t2_dn6)))) * assign10980_e5888) - (assign10980_e5883 * ((var_t1_dn6 * var_t3) + (var_t1 * var_t3_dn6)))) / (assign10980_e5888 * assign10980_e5888)), (((((var_mueph_dn7 * assign10980_e5882) + (var_mueph * ((var_t1_dn7 * var_t2) + (var_t1 * var_t2_dn7)))) * assign10980_e5888) - (assign10980_e5883 * ((var_t1_dn7 * var_t3) + (var_t1 * var_t3_dn7)))) / (assign10980_e5888 * assign10980_e5888)), (((((var_mueph_dn8 * assign10980_e5882) + (var_mueph * ((var_t1_dn8 * var_t2) + (var_t1 * var_t2_dn8)))) * assign10980_e5888) - (assign10980_e5883 * ((var_t1_dn8 * var_t3) + (var_t1 * var_t3_dn8)))) / (assign10980_e5888 * assign10980_e5888)), (((((var_mueph_dn9 * assign10980_e5882) + (var_mueph * ((var_t1_dn9 * var_t2) + (var_t1 * var_t2_dn9)))) * assign10980_e5888) - (assign10980_e5883 * ((var_t1_dn9 * var_t3) + (var_t1 * var_t3_dn9)))) / (assign10980_e5888 * assign10980_e5888)), (((((var_mueph_dn10 * assign10980_e5882) + (var_mueph * ((var_t1_dn10 * var_t2) + (var_t1 * var_t2_dn10)))) * assign10980_e5888) - (assign10980_e5883 * ((var_t1_dn10 * var_t3) + (var_t1 * var_t3_dn10)))) / (assign10980_e5888 * assign10980_e5888)), (((((var_mueph_dn11 * assign10980_e5882) + (var_mueph * ((var_t1_dn11 * var_t2) + (var_t1 * var_t2_dn11)))) * assign10980_e5888) - (assign10980_e5883 * ((var_t1_dn11 * var_t3) + (var_t1 * var_t3_dn11)))) / (assign10980_e5888 * assign10980_e5888)), (((((var_mueph_dn14 * assign10980_e5882) + (var_mueph * ((var_t1_dn14 * var_t2) + (var_t1 * var_t2_dn14)))) * assign10980_e5888) - (assign10980_e5883 * ((var_t1_dn14 * var_t3) + (var_t1 * var_t3_dn14)))) / (assign10980_e5888 * assign10980_e5888)),)
    } else {
        (var_mueph, var_mueph_dn0, var_mueph_dn2, var_mueph_dn4, var_mueph_dn5, var_mueph_dn6, var_mueph_dn7, var_mueph_dn8, var_mueph_dn9, var_mueph_dn10, var_mueph_dn11, var_mueph_dn14,)
    }
};
        var_mueph = assign10980_e5891;
        var_mueph_dn0 = assign10980_e5891_d_n0;
        var_mueph_dn2 = assign10980_e5891_d_n2;
        var_mueph_dn4 = assign10980_e5891_d_n4;
        var_mueph_dn5 = assign10980_e5891_d_n5;
        var_mueph_dn6 = assign10980_e5891_d_n6;
        var_mueph_dn7 = assign10980_e5891_d_n7;
        var_mueph_dn8 = assign10980_e5891_d_n8;
        var_mueph_dn9 = assign10980_e5891_d_n9;
        var_mueph_dn10 = assign10980_e5891_d_n10;
        var_mueph_dn11 = assign10980_e5891_d_n11;
        var_mueph_dn14 = assign10980_e5891_d_n14;
        var_mueph_rv = 0.0;

        let assign10990_e5897: f64 = (var_lg).powf(p.p176);
        let assign10990_e5898: f64 = (p.p173 / assign10990_e5897);
        let assign10990_e5899: f64 = (1.0 + assign10990_e5898);
        let assign10990_e5900: f64 = (p.p171 * assign10990_e5899);
        let assign10990_e5905: f64 = (var_wg).powf(p.p175);
        let assign10990_e5906: f64 = (p.p174 / assign10990_e5905);
        let assign10990_e5907: f64 = (1.0 + assign10990_e5906);
        let assign10990_e5908: f64 = (assign10990_e5900 * assign10990_e5907);
        var_muesr = assign10990_e5908;
        var_muesr_rv = 0.0;

        let (assign11020_e5932, assign11020_e5932_d_n0, assign11020_e5932_d_n2, assign11020_e5932_d_n4, assign11020_e5932_d_n5, assign11020_e5932_d_n6, assign11020_e5932_d_n7, assign11020_e5932_d_n8, assign11020_e5932_d_n9, assign11020_e5932_d_n10, assign11020_e5932_d_n11, assign11020_e5932_d_n14,) = {
    if (var_mueph < 1e-25) {
        (1e-25, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_mueph, var_mueph_dn0, var_mueph_dn2, var_mueph_dn4, var_mueph_dn5, var_mueph_dn6, var_mueph_dn7, var_mueph_dn8, var_mueph_dn9, var_mueph_dn10, var_mueph_dn11, var_mueph_dn14,)
    }
};
        var_mueph = assign11020_e5932;
        var_mueph_dn0 = assign11020_e5932_d_n0;
        var_mueph_dn2 = assign11020_e5932_d_n2;
        var_mueph_dn4 = assign11020_e5932_d_n4;
        var_mueph_dn5 = assign11020_e5932_d_n5;
        var_mueph_dn6 = assign11020_e5932_d_n6;
        var_mueph_dn7 = assign11020_e5932_d_n7;
        var_mueph_dn8 = assign11020_e5932_d_n8;
        var_mueph_dn9 = assign11020_e5932_d_n9;
        var_mueph_dn10 = assign11020_e5932_d_n10;
        var_mueph_dn11 = assign11020_e5932_d_n11;
        var_mueph_dn14 = assign11020_e5932_d_n14;
        var_mueph_rv = 0.0;

        let (assign11030_e5938,) = {
    if (var_muesr < 1e-25) {
        (1e-25,)
    } else {
        (var_muesr,)
    }
};
        var_muesr = assign11030_e5938;
        var_muesr_rv = 0.0;

        let assign11040_e5941: f64 = (var_lg).powf(p.p156);
        var_t1 = assign11040_e5941;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn11 = 0.0;
        var_t1_dn14 = 0.0;
        var_t1_rv = 0.0;

        let assign11050_e5944: f64 = (var_uc_ndep * var_t1);
        let assign11050_e5947: f64 = (var_t1 + p.p155);
        let assign11050_e5948: f64 = (assign11050_e5944 / assign11050_e5947);
        let assign11050_e5950: f64 = (assign11050_e5948 / 1.034943e-10);
        var_ndep_o_esi = assign11050_e5950;
        var_ndep_o_esi_dn0 = (((((var_uc_ndep * var_t1_dn0) * assign11050_e5947) - (assign11050_e5944 * var_t1_dn0)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        var_ndep_o_esi_dn2 = (((((var_uc_ndep * var_t1_dn2) * assign11050_e5947) - (assign11050_e5944 * var_t1_dn2)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        var_ndep_o_esi_dn4 = (((((var_uc_ndep * var_t1_dn4) * assign11050_e5947) - (assign11050_e5944 * var_t1_dn4)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        var_ndep_o_esi_dn5 = (((((var_uc_ndep * var_t1_dn5) * assign11050_e5947) - (assign11050_e5944 * var_t1_dn5)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        var_ndep_o_esi_dn6 = (((((var_uc_ndep * var_t1_dn6) * assign11050_e5947) - (assign11050_e5944 * var_t1_dn6)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        var_ndep_o_esi_dn7 = (((((var_uc_ndep * var_t1_dn7) * assign11050_e5947) - (assign11050_e5944 * var_t1_dn7)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        var_ndep_o_esi_dn8 = (((((var_uc_ndep * var_t1_dn8) * assign11050_e5947) - (assign11050_e5944 * var_t1_dn8)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        var_ndep_o_esi_dn9 = (((((var_uc_ndep * var_t1_dn9) * assign11050_e5947) - (assign11050_e5944 * var_t1_dn9)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        var_ndep_o_esi_dn10 = (((((var_uc_ndep * var_t1_dn10) * assign11050_e5947) - (assign11050_e5944 * var_t1_dn10)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        var_ndep_o_esi_dn11 = (((((var_uc_ndep * var_t1_dn11) * assign11050_e5947) - (assign11050_e5944 * var_t1_dn11)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        var_ndep_o_esi_dn14 = (((((var_uc_ndep * var_t1_dn14) * assign11050_e5947) - (assign11050_e5944 * var_t1_dn14)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        var_ndep_o_esi_rv = 0.0;

        let assign11060_e5953: f64 = (var_uc_ninv / 1.034943e-10);
        var_ninv_o_esi = assign11060_e5953;
        var_ninv_o_esi_rv = 0.0;

        let assign11070_e5959: f64 = (var_lg).powf(p.p321);
        let assign11070_e5960: f64 = (p.p320 / assign11070_e5959);
        let assign11070_e5961: f64 = (1.0 + assign11070_e5960);
        let assign11070_e5962: f64 = (p.p319 * assign11070_e5961);
        let assign11070_e5967: f64 = (var_wg).powf(p.p323);
        let assign11070_e5968: f64 = (p.p322 / assign11070_e5967);
        let assign11070_e5969: f64 = (1.0 + assign11070_e5968);
        let assign11070_e5970: f64 = (assign11070_e5962 * assign11070_e5969);
        var_ninvd0 = assign11070_e5970;
        var_ninvd0_rv = 0.0;

        let assign11080_e5975: f64 = (var_lg).powf(p.p387);
        let assign11080_e5976: f64 = (p.p386 / assign11080_e5975);
        let assign11080_e5977: f64 = (1.0 + assign11080_e5976);
        let assign11080_e5982: f64 = (var_wg).powf(p.p389);
        let assign11080_e5983: f64 = (p.p388 / assign11080_e5982);
        let assign11080_e5984: f64 = (1.0 + assign11080_e5983);
        let assign11080_e5985: f64 = (assign11080_e5977 * assign11080_e5984);
        var_t1 = assign11080_e5985;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn11 = 0.0;
        var_t1_dn14 = 0.0;
        var_t1_rv = 0.0;

        let assign11090_e5988: f64 = (p.p384 * var_t1);
        var_ninvd0cres = assign11090_e5988;
        var_ninvd0cres_dn0 = (p.p384 * var_t1_dn0);
        var_ninvd0cres_dn2 = (p.p384 * var_t1_dn2);
        var_ninvd0cres_dn4 = (p.p384 * var_t1_dn4);
        var_ninvd0cres_dn5 = (p.p384 * var_t1_dn5);
        var_ninvd0cres_dn6 = (p.p384 * var_t1_dn6);
        var_ninvd0cres_dn7 = (p.p384 * var_t1_dn7);
        var_ninvd0cres_dn8 = (p.p384 * var_t1_dn8);
        var_ninvd0cres_dn9 = (p.p384 * var_t1_dn9);
        var_ninvd0cres_dn10 = (p.p384 * var_t1_dn10);
        var_ninvd0cres_dn11 = (p.p384 * var_t1_dn11);
        var_ninvd0cres_dn14 = (p.p384 * var_t1_dn14);
        var_ninvd0cres_rv = 0.0;

        let assign11100_e5991: f64 = (p.p385 * var_t1);
        var_ninvd0hres = assign11100_e5991;
        var_ninvd0hres_dn0 = (p.p385 * var_t1_dn0);
        var_ninvd0hres_dn2 = (p.p385 * var_t1_dn2);
        var_ninvd0hres_dn4 = (p.p385 * var_t1_dn4);
        var_ninvd0hres_dn5 = (p.p385 * var_t1_dn5);
        var_ninvd0hres_dn6 = (p.p385 * var_t1_dn6);
        var_ninvd0hres_dn7 = (p.p385 * var_t1_dn7);
        var_ninvd0hres_dn8 = (p.p385 * var_t1_dn8);
        var_ninvd0hres_dn9 = (p.p385 * var_t1_dn9);
        var_ninvd0hres_dn10 = (p.p385 * var_t1_dn10);
        var_ninvd0hres_dn11 = (p.p385 * var_t1_dn11);
        var_ninvd0hres_dn14 = (p.p385 * var_t1_dn14);
        var_ninvd0hres_rv = 0.0;

        let assign11110_e5996: f64 = (var_lgate + p.p121);
        let assign11110_e5998: f64 = (assign11110_e5996).powf(p.p122);
        let assign11110_e5999: f64 = (var_mks_ll / assign11110_e5998);
        let assign11110_e6000: f64 = (p.p97 + assign11110_e5999);
        var_dl = assign11110_e6000;
        var_dl_rv = 0.0;

        let assign11120_e6005: f64 = (var_lgate + p.p121);
        let assign11120_e6007: f64 = (assign11120_e6005).powf(p.p122);
        let assign11120_e6008: f64 = (var_mks_ll / assign11120_e6007);
        let assign11120_e6009: f64 = (var_uc_xldld + assign11120_e6008);
        var_dlld = assign11120_e6009;
        var_dlld_rv = 0.0;

        let assign11130_e6014: f64 = (var_wgate + p.p128);
        let assign11130_e6016: f64 = (assign11130_e6014).powf(p.p129);
        let assign11130_e6017: f64 = (var_mks_wl / assign11130_e6016);
        let assign11130_e6018: f64 = (p.p114 + assign11130_e6017);
        var_dw = assign11130_e6018;
        var_dw_rv = 0.0;

        let assign11140_e6023: f64 = (var_wgate + p.p128);
        let assign11140_e6025: f64 = (assign11140_e6023).powf(p.p129);
        let assign11140_e6026: f64 = (var_mks_wl / assign11140_e6025);
        let assign11140_e6027: f64 = (p.p295 + assign11140_e6026);
        var_dwld = assign11140_e6027;
        var_dwld_rv = 0.0;

        let assign11150_e6032: f64 = (var_wgate + p.p128);
        let assign11150_e6034: f64 = (assign11150_e6032).powf(p.p129);
        let assign11150_e6035: f64 = (var_mks_wl / assign11150_e6034);
        let assign11150_e6036: f64 = (p.p115 + assign11150_e6035);
        var_dwcv = assign11150_e6036;
        var_dwcv_rv = 0.0;

        let assign11160_e6040: f64 = (var_dl + var_dlld);
        let assign11160_e6041: f64 = (var_lgate - assign11160_e6040);
        var_leff = assign11160_e6041;
        var_leff_rv = 0.0;

        let assign11190_e6053: f64 = (var_wlg).powf(p.p125);
        let assign11190_e6054: f64 = (p.p124 / assign11190_e6053);
        let assign11190_e6055: f64 = (var_lgate + assign11190_e6054);
        var_lgatesm = assign11190_e6055;
        var_lgatesm_rv = 0.0;

        let assign11200_e6059: f64 = (var_wlg).powf(p.p127);
        let assign11200_e6060: f64 = (var_uc_wl2 / assign11200_e6059);
        var_dvthsm = assign11200_e6060;
        var_dvthsm_rv = 0.0;

        let assign11210_e6065: f64 = (var_lgatesm * 1000000.0);
        let assign11210_e6067: f64 = (assign11210_e6065).powf(p.p207);
        let assign11210_e6068: f64 = (p.p206 / assign11210_e6067);
        let assign11210_e6069: f64 = (1.0 + assign11210_e6068);
        var_t1 = assign11210_e6069;
        var_t1_dn0 = 0.0;
        var_t1_dn2 = 0.0;
        var_t1_dn4 = 0.0;
        var_t1_dn5 = 0.0;
        var_t1_dn6 = 0.0;
        var_t1_dn7 = 0.0;
        var_t1_dn8 = 0.0;
        var_t1_dn9 = 0.0;
        var_t1_dn10 = 0.0;
        var_t1_dn11 = 0.0;
        var_t1_dn14 = 0.0;
        var_t1_rv = 0.0;

        let assign11220_e6074: f64 = (var_wg).powf(p.p209);
        let assign11220_e6075: f64 = (p.p208 / assign11220_e6074);
        let assign11220_e6076: f64 = (1.0 + assign11220_e6075);
        var_t2 = assign11220_e6076;
        var_t2_dn0 = 0.0;
        var_t2_dn2 = 0.0;
        var_t2_dn4 = 0.0;
        var_t2_dn5 = 0.0;
        var_t2_dn6 = 0.0;
        var_t2_dn7 = 0.0;
        var_t2_dn8 = 0.0;
        var_t2_dn9 = 0.0;
        var_t2_dn10 = 0.0;
        var_t2_dn11 = 0.0;
        var_t2_dn14 = 0.0;
        var_t2_rv = 0.0;

        let assign11230_e6079: f64 = (var_uc_wsti * var_t1);
        let assign11230_e6081: f64 = (assign11230_e6079 * var_t2);
        var_uc_wsti = assign11230_e6081;
        var_uc_wsti_dn0 = ((((var_uc_wsti_dn0 * var_t1) + (var_uc_wsti * var_t1_dn0)) * var_t2) + (assign11230_e6079 * var_t2_dn0));
        var_uc_wsti_dn2 = ((((var_uc_wsti_dn2 * var_t1) + (var_uc_wsti * var_t1_dn2)) * var_t2) + (assign11230_e6079 * var_t2_dn2));
        var_uc_wsti_dn4 = ((((var_uc_wsti_dn4 * var_t1) + (var_uc_wsti * var_t1_dn4)) * var_t2) + (assign11230_e6079 * var_t2_dn4));
        var_uc_wsti_dn5 = ((((var_uc_wsti_dn5 * var_t1) + (var_uc_wsti * var_t1_dn5)) * var_t2) + (assign11230_e6079 * var_t2_dn5));
        var_uc_wsti_dn6 = ((((var_uc_wsti_dn6 * var_t1) + (var_uc_wsti * var_t1_dn6)) * var_t2) + (assign11230_e6079 * var_t2_dn6));
        var_uc_wsti_dn7 = ((((var_uc_wsti_dn7 * var_t1) + (var_uc_wsti * var_t1_dn7)) * var_t2) + (assign11230_e6079 * var_t2_dn7));
        var_uc_wsti_dn8 = ((((var_uc_wsti_dn8 * var_t1) + (var_uc_wsti * var_t1_dn8)) * var_t2) + (assign11230_e6079 * var_t2_dn8));
        var_uc_wsti_dn9 = ((((var_uc_wsti_dn9 * var_t1) + (var_uc_wsti * var_t1_dn9)) * var_t2) + (assign11230_e6079 * var_t2_dn9));
        var_uc_wsti_dn10 = ((((var_uc_wsti_dn10 * var_t1) + (var_uc_wsti * var_t1_dn10)) * var_t2) + (assign11230_e6079 * var_t2_dn10));
        var_uc_wsti_dn11 = ((((var_uc_wsti_dn11 * var_t1) + (var_uc_wsti * var_t1_dn11)) * var_t2) + (assign11230_e6079 * var_t2_dn11));
        var_uc_wsti_dn14 = ((((var_uc_wsti_dn14 * var_t1) + (var_uc_wsti * var_t1_dn14)) * var_t2) + (assign11230_e6079 * var_t2_dn14));
        var_uc_wsti_rv = 0.0;

        let assign11240_e6085: f64 = (2.0 * var_dw);
        let assign11240_e6086: f64 = (var_wgate - assign11240_e6085);
        var_weff = assign11240_e6086;
        var_weff_rv = 0.0;

        let assign11250_e6090: f64 = (2.0 * var_dwld);
        let assign11250_e6091: f64 = (var_wgate - assign11250_e6090);
        var_weff_ld = assign11250_e6091;
        var_weff_ld_rv = 0.0;

        let assign11260_e6095: f64 = (2.0 * var_dwcv);
        let assign11260_e6096: f64 = (var_wgate - assign11260_e6095);
        var_weff_cv = assign11260_e6096;
        var_weff_cv_rv = 0.0;

        let assign11330_e6120: f64 = (var_weff * p.p7);
        var_weff_nf = assign11330_e6120;
        var_weff_nf_rv = 0.0;

        let assign11340_e6123: f64 = (var_weff_cv * p.p7);
        var_weffcv_nf = assign11340_e6123;
        var_weffcv_nf_rv = 0.0;

        let assign11350_e6129: f64 = (var_wg).powf(p.p143);
        let assign11350_e6130: f64 = (p.p142 / assign11350_e6129);
        let assign11350_e6131: f64 = (1.0 + assign11350_e6130);
        let assign11350_e6132: f64 = (var_ef_nsubp * assign11350_e6131);
        var_nsubpp = assign11350_e6132;
        var_nsubpp_dn0 = (var_ef_nsubp_dn0 * assign11350_e6131);
        var_nsubpp_dn2 = (var_ef_nsubp_dn2 * assign11350_e6131);
        var_nsubpp_dn4 = (var_ef_nsubp_dn4 * assign11350_e6131);
        var_nsubpp_dn5 = (var_ef_nsubp_dn5 * assign11350_e6131);
        var_nsubpp_dn6 = (var_ef_nsubp_dn6 * assign11350_e6131);
        var_nsubpp_dn7 = (var_ef_nsubp_dn7 * assign11350_e6131);
        var_nsubpp_dn8 = (var_ef_nsubp_dn8 * assign11350_e6131);
        var_nsubpp_dn9 = (var_ef_nsubp_dn9 * assign11350_e6131);
        var_nsubpp_dn10 = (var_ef_nsubp_dn10 * assign11350_e6131);
        var_nsubpp_dn11 = (var_ef_nsubp_dn11 * assign11350_e6131);
        var_nsubpp_dn14 = (var_ef_nsubp_dn14 * assign11350_e6131);
        var_nsubpp_rv = 0.0;

        let assign11360_e6138: f64 = (var_wg).powf(p.p234);
        let assign11360_e6139: f64 = (p.p233 / assign11360_e6138);
        let assign11360_e6140: f64 = (1.0 + assign11360_e6139);
        let assign11360_e6141: f64 = (var_ef_nsubc * assign11360_e6140);
        var_ef_nsubc = assign11360_e6141;
        var_ef_nsubc_dn0 = (var_ef_nsubc_dn0 * assign11360_e6140);
        var_ef_nsubc_dn2 = (var_ef_nsubc_dn2 * assign11360_e6140);
        var_ef_nsubc_dn4 = (var_ef_nsubc_dn4 * assign11360_e6140);
        var_ef_nsubc_dn5 = (var_ef_nsubc_dn5 * assign11360_e6140);
        var_ef_nsubc_dn6 = (var_ef_nsubc_dn6 * assign11360_e6140);
        var_ef_nsubc_dn7 = (var_ef_nsubc_dn7 * assign11360_e6140);
        var_ef_nsubc_dn8 = (var_ef_nsubc_dn8 * assign11360_e6140);
        var_ef_nsubc_dn9 = (var_ef_nsubc_dn9 * assign11360_e6140);
        var_ef_nsubc_dn10 = (var_ef_nsubc_dn10 * assign11360_e6140);
        var_ef_nsubc_dn11 = (var_ef_nsubc_dn11 * assign11360_e6140);
        var_ef_nsubc_dn14 = (var_ef_nsubc_dn14 * assign11360_e6140);
        var_ef_nsubc_rv = 0.0;

        let assign11370_e6144: f64 = (var_ef_nsubc * 1e-6);
        var_t1 = assign11370_e6144;
        var_t1_dn0 = (var_ef_nsubc_dn0 * 1e-6);
        var_t1_dn2 = (var_ef_nsubc_dn2 * 1e-6);
        var_t1_dn4 = (var_ef_nsubc_dn4 * 1e-6);
        var_t1_dn5 = (var_ef_nsubc_dn5 * 1e-6);
        var_t1_dn6 = (var_ef_nsubc_dn6 * 1e-6);
        var_t1_dn7 = (var_ef_nsubc_dn7 * 1e-6);
        var_t1_dn8 = (var_ef_nsubc_dn8 * 1e-6);
        var_t1_dn9 = (var_ef_nsubc_dn9 * 1e-6);
        var_t1_dn10 = (var_ef_nsubc_dn10 * 1e-6);
        var_t1_dn11 = (var_ef_nsubc_dn11 * 1e-6);
        var_t1_dn14 = (var_ef_nsubc_dn14 * 1e-6);
        var_t1_rv = 0.0;

        let assign11380_e6147: f64 = (var_nsubpp * 1e-6);
        var_t2 = assign11380_e6147;
        var_t2_dn0 = (var_nsubpp_dn0 * 1e-6);
        var_t2_dn2 = (var_nsubpp_dn2 * 1e-6);
        var_t2_dn4 = (var_nsubpp_dn4 * 1e-6);
        var_t2_dn5 = (var_nsubpp_dn5 * 1e-6);
        var_t2_dn6 = (var_nsubpp_dn6 * 1e-6);
        var_t2_dn7 = (var_nsubpp_dn7 * 1e-6);
        var_t2_dn8 = (var_nsubpp_dn8 * 1e-6);
        var_t2_dn9 = (var_nsubpp_dn9 * 1e-6);
        var_t2_dn10 = (var_nsubpp_dn10 * 1e-6);
        var_t2_dn11 = (var_nsubpp_dn11 * 1e-6);
        var_t2_dn14 = (var_nsubpp_dn14 * 1e-6);
        var_t2_rv = 0.0;

        let assign11400_e6155: f64 = if var_t1 < 1000000000000000.0 { 1.0 } else { 0.0 };
        var_guard265 = assign11400_e6155;
        var_guard265_rv = 0.0;

        *var_dl_slot = var_dl;
        *var_dl_rv_slot = var_dl_rv;
        *var_dlld_slot = var_dlld;
        *var_dlld_rv_slot = var_dlld_rv;
        *var_dvthsm_slot = var_dvthsm;
        *var_dvthsm_rv_slot = var_dvthsm_rv;
        *var_dw_slot = var_dw;
        *var_dw_rv_slot = var_dw_rv;
        *var_dwcv_slot = var_dwcv;
        *var_dwcv_rv_slot = var_dwcv_rv;
        *var_dwld_slot = var_dwld;
        *var_dwld_rv_slot = var_dwld_rv;
        *var_ef_nsubc_slot = var_ef_nsubc;
        *var_ef_nsubc_dn0_slot = var_ef_nsubc_dn0;
        *var_ef_nsubc_dn10_slot = var_ef_nsubc_dn10;
        *var_ef_nsubc_dn11_slot = var_ef_nsubc_dn11;
        *var_ef_nsubc_dn14_slot = var_ef_nsubc_dn14;
        *var_ef_nsubc_dn2_slot = var_ef_nsubc_dn2;
        *var_ef_nsubc_dn4_slot = var_ef_nsubc_dn4;
        *var_ef_nsubc_dn5_slot = var_ef_nsubc_dn5;
        *var_ef_nsubc_dn6_slot = var_ef_nsubc_dn6;
        *var_ef_nsubc_dn7_slot = var_ef_nsubc_dn7;
        *var_ef_nsubc_dn8_slot = var_ef_nsubc_dn8;
        *var_ef_nsubc_dn9_slot = var_ef_nsubc_dn9;
        *var_ef_nsubc_rv_slot = var_ef_nsubc_rv;
        *var_guard257_slot = var_guard257;
        *var_guard257_rv_slot = var_guard257_rv;
        *var_guard265_slot = var_guard265;
        *var_guard265_rv_slot = var_guard265_rv;
        *var_leff_slot = var_leff;
        *var_leff_rv_slot = var_leff_rv;
        *var_lgatesm_slot = var_lgatesm;
        *var_lgatesm_rv_slot = var_lgatesm_rv;
        *var_mueph_slot = var_mueph;
        *var_mueph_dn0_slot = var_mueph_dn0;
        *var_mueph_dn10_slot = var_mueph_dn10;
        *var_mueph_dn11_slot = var_mueph_dn11;
        *var_mueph_dn14_slot = var_mueph_dn14;
        *var_mueph_dn2_slot = var_mueph_dn2;
        *var_mueph_dn4_slot = var_mueph_dn4;
        *var_mueph_dn5_slot = var_mueph_dn5;
        *var_mueph_dn6_slot = var_mueph_dn6;
        *var_mueph_dn7_slot = var_mueph_dn7;
        *var_mueph_dn8_slot = var_mueph_dn8;
        *var_mueph_dn9_slot = var_mueph_dn9;
        *var_mueph_rv_slot = var_mueph_rv;
        *var_muesr_slot = var_muesr;
        *var_muesr_rv_slot = var_muesr_rv;
        *var_ndep_o_esi_slot = var_ndep_o_esi;
        *var_ndep_o_esi_dn0_slot = var_ndep_o_esi_dn0;
        *var_ndep_o_esi_dn10_slot = var_ndep_o_esi_dn10;
        *var_ndep_o_esi_dn11_slot = var_ndep_o_esi_dn11;
        *var_ndep_o_esi_dn14_slot = var_ndep_o_esi_dn14;
        *var_ndep_o_esi_dn2_slot = var_ndep_o_esi_dn2;
        *var_ndep_o_esi_dn4_slot = var_ndep_o_esi_dn4;
        *var_ndep_o_esi_dn5_slot = var_ndep_o_esi_dn5;
        *var_ndep_o_esi_dn6_slot = var_ndep_o_esi_dn6;
        *var_ndep_o_esi_dn7_slot = var_ndep_o_esi_dn7;
        *var_ndep_o_esi_dn8_slot = var_ndep_o_esi_dn8;
        *var_ndep_o_esi_dn9_slot = var_ndep_o_esi_dn9;
        *var_ndep_o_esi_rv_slot = var_ndep_o_esi_rv;
        *var_ninv_o_esi_slot = var_ninv_o_esi;
        *var_ninv_o_esi_rv_slot = var_ninv_o_esi_rv;
        *var_ninvd0_slot = var_ninvd0;
        *var_ninvd0_rv_slot = var_ninvd0_rv;
        *var_ninvd0cres_slot = var_ninvd0cres;
        *var_ninvd0cres_dn0_slot = var_ninvd0cres_dn0;
        *var_ninvd0cres_dn10_slot = var_ninvd0cres_dn10;
        *var_ninvd0cres_dn11_slot = var_ninvd0cres_dn11;
        *var_ninvd0cres_dn14_slot = var_ninvd0cres_dn14;
        *var_ninvd0cres_dn2_slot = var_ninvd0cres_dn2;
        *var_ninvd0cres_dn4_slot = var_ninvd0cres_dn4;
        *var_ninvd0cres_dn5_slot = var_ninvd0cres_dn5;
        *var_ninvd0cres_dn6_slot = var_ninvd0cres_dn6;
        *var_ninvd0cres_dn7_slot = var_ninvd0cres_dn7;
        *var_ninvd0cres_dn8_slot = var_ninvd0cres_dn8;
        *var_ninvd0cres_dn9_slot = var_ninvd0cres_dn9;
        *var_ninvd0cres_rv_slot = var_ninvd0cres_rv;
        *var_ninvd0hres_slot = var_ninvd0hres;
        *var_ninvd0hres_dn0_slot = var_ninvd0hres_dn0;
        *var_ninvd0hres_dn10_slot = var_ninvd0hres_dn10;
        *var_ninvd0hres_dn11_slot = var_ninvd0hres_dn11;
        *var_ninvd0hres_dn14_slot = var_ninvd0hres_dn14;
        *var_ninvd0hres_dn2_slot = var_ninvd0hres_dn2;
        *var_ninvd0hres_dn4_slot = var_ninvd0hres_dn4;
        *var_ninvd0hres_dn5_slot = var_ninvd0hres_dn5;
        *var_ninvd0hres_dn6_slot = var_ninvd0hres_dn6;
        *var_ninvd0hres_dn7_slot = var_ninvd0hres_dn7;
        *var_ninvd0hres_dn8_slot = var_ninvd0hres_dn8;
        *var_ninvd0hres_dn9_slot = var_ninvd0hres_dn9;
        *var_ninvd0hres_rv_slot = var_ninvd0hres_rv;
        *var_npexte_slot = var_npexte;
        *var_npexte_dn0_slot = var_npexte_dn0;
        *var_npexte_dn10_slot = var_npexte_dn10;
        *var_npexte_dn11_slot = var_npexte_dn11;
        *var_npexte_dn14_slot = var_npexte_dn14;
        *var_npexte_dn2_slot = var_npexte_dn2;
        *var_npexte_dn4_slot = var_npexte_dn4;
        *var_npexte_dn5_slot = var_npexte_dn5;
        *var_npexte_dn6_slot = var_npexte_dn6;
        *var_npexte_dn7_slot = var_npexte_dn7;
        *var_npexte_dn8_slot = var_npexte_dn8;
        *var_npexte_dn9_slot = var_npexte_dn9;
        *var_npexte_rv_slot = var_npexte_rv;
        *var_nsubpp_slot = var_nsubpp;
        *var_nsubpp_dn0_slot = var_nsubpp_dn0;
        *var_nsubpp_dn10_slot = var_nsubpp_dn10;
        *var_nsubpp_dn11_slot = var_nsubpp_dn11;
        *var_nsubpp_dn14_slot = var_nsubpp_dn14;
        *var_nsubpp_dn2_slot = var_nsubpp_dn2;
        *var_nsubpp_dn4_slot = var_nsubpp_dn4;
        *var_nsubpp_dn5_slot = var_nsubpp_dn5;
        *var_nsubpp_dn6_slot = var_nsubpp_dn6;
        *var_nsubpp_dn7_slot = var_nsubpp_dn7;
        *var_nsubpp_dn8_slot = var_nsubpp_dn8;
        *var_nsubpp_dn9_slot = var_nsubpp_dn9;
        *var_nsubpp_rv_slot = var_nsubpp_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn14_slot = var_t3_dn14;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t3_rv_slot = var_t3_rv;
        *var_uc_wsti_slot = var_uc_wsti;
        *var_uc_wsti_dn0_slot = var_uc_wsti_dn0;
        *var_uc_wsti_dn10_slot = var_uc_wsti_dn10;
        *var_uc_wsti_dn11_slot = var_uc_wsti_dn11;
        *var_uc_wsti_dn14_slot = var_uc_wsti_dn14;
        *var_uc_wsti_dn2_slot = var_uc_wsti_dn2;
        *var_uc_wsti_dn4_slot = var_uc_wsti_dn4;
        *var_uc_wsti_dn5_slot = var_uc_wsti_dn5;
        *var_uc_wsti_dn6_slot = var_uc_wsti_dn6;
        *var_uc_wsti_dn7_slot = var_uc_wsti_dn7;
        *var_uc_wsti_dn8_slot = var_uc_wsti_dn8;
        *var_uc_wsti_dn9_slot = var_uc_wsti_dn9;
        *var_uc_wsti_rv_slot = var_uc_wsti_rv;
        *var_weff_slot = var_weff;
        *var_weff_cv_slot = var_weff_cv;
        *var_weff_cv_rv_slot = var_weff_cv_rv;
        *var_weff_ld_slot = var_weff_ld;
        *var_weff_ld_rv_slot = var_weff_ld_rv;
        *var_weff_nf_slot = var_weff_nf;
        *var_weff_nf_rv_slot = var_weff_nf_rv;
        *var_weff_rv_slot = var_weff_rv;
        *var_weffcv_nf_slot = var_weffcv_nf;
        *var_weffcv_nf_rv_slot = var_weffcv_nf_rv;
    }

    pub(super) fn stamp_reactive_block_19(
        p: &Parameters,
        var_guard265: f64,
        var_lgate: f64,
        var_lod_half: f64,
        var_lod_half_dn0: f64,
        var_lod_half_dn10: f64,
        var_lod_half_dn11: f64,
        var_lod_half_dn14: f64,
        var_lod_half_dn2: f64,
        var_lod_half_dn4: f64,
        var_lod_half_dn5: f64,
        var_lod_half_dn6: f64,
        var_lod_half_dn7: f64,
        var_lod_half_dn8: f64,
        var_lod_half_dn9: f64,
        var_lod_half_ref: f64,
        var_lod_half_ref_dn0: f64,
        var_lod_half_ref_dn10: f64,
        var_lod_half_ref_dn11: f64,
        var_lod_half_ref_dn14: f64,
        var_lod_half_ref_dn2: f64,
        var_lod_half_ref_dn4: f64,
        var_lod_half_ref_dn5: f64,
        var_lod_half_ref_dn6: f64,
        var_lod_half_ref_dn7: f64,
        var_lod_half_ref_dn8: f64,
        var_lod_half_ref_dn9: f64,
        var_npexte: f64,
        var_npexte_dn0: f64,
        var_npexte_dn10: f64,
        var_npexte_dn11: f64,
        var_npexte_dn14: f64,
        var_npexte_dn2: f64,
        var_npexte_dn4: f64,
        var_npexte_dn5: f64,
        var_npexte_dn6: f64,
        var_npexte_dn7: f64,
        var_npexte_dn8: f64,
        var_npexte_dn9: f64,
        var_uc_nsubpsti1: f64,
        var_uc_nsubpsti2: f64,
        var_uc_nsubpsti3: f64,
        var_ef_nsubc_slot: &mut f64,
        var_ef_nsubc_dn0_slot: &mut f64,
        var_ef_nsubc_dn10_slot: &mut f64,
        var_ef_nsubc_dn11_slot: &mut f64,
        var_ef_nsubc_dn14_slot: &mut f64,
        var_ef_nsubc_dn2_slot: &mut f64,
        var_ef_nsubc_dn4_slot: &mut f64,
        var_ef_nsubc_dn5_slot: &mut f64,
        var_ef_nsubc_dn6_slot: &mut f64,
        var_ef_nsubc_dn7_slot: &mut f64,
        var_ef_nsubc_dn8_slot: &mut f64,
        var_ef_nsubc_dn9_slot: &mut f64,
        var_ef_nsubc_rv_slot: &mut f64,
        var_guard267_slot: &mut f64,
        var_guard267_rv_slot: &mut f64,
        var_guard268_slot: &mut f64,
        var_guard268_rv_slot: &mut f64,
        var_guard269_slot: &mut f64,
        var_guard269_rv_slot: &mut f64,
        var_guard270_slot: &mut f64,
        var_guard270_rv_slot: &mut f64,
        var_nsub_slot: &mut f64,
        var_nsub_dn0_slot: &mut f64,
        var_nsub_dn10_slot: &mut f64,
        var_nsub_dn11_slot: &mut f64,
        var_nsub_dn14_slot: &mut f64,
        var_nsub_dn2_slot: &mut f64,
        var_nsub_dn4_slot: &mut f64,
        var_nsub_dn5_slot: &mut f64,
        var_nsub_dn6_slot: &mut f64,
        var_nsub_dn7_slot: &mut f64,
        var_nsub_dn8_slot: &mut f64,
        var_nsub_dn9_slot: &mut f64,
        var_nsub_rv_slot: &mut f64,
        var_nsubb_slot: &mut f64,
        var_nsubb_dn0_slot: &mut f64,
        var_nsubb_dn10_slot: &mut f64,
        var_nsubb_dn11_slot: &mut f64,
        var_nsubb_dn14_slot: &mut f64,
        var_nsubb_dn2_slot: &mut f64,
        var_nsubb_dn4_slot: &mut f64,
        var_nsubb_dn5_slot: &mut f64,
        var_nsubb_dn6_slot: &mut f64,
        var_nsubb_dn7_slot: &mut f64,
        var_nsubb_dn8_slot: &mut f64,
        var_nsubb_dn9_slot: &mut f64,
        var_nsubb_rv_slot: &mut f64,
        var_nsubpp_slot: &mut f64,
        var_nsubpp_dn0_slot: &mut f64,
        var_nsubpp_dn10_slot: &mut f64,
        var_nsubpp_dn11_slot: &mut f64,
        var_nsubpp_dn14_slot: &mut f64,
        var_nsubpp_dn2_slot: &mut f64,
        var_nsubpp_dn4_slot: &mut f64,
        var_nsubpp_dn5_slot: &mut f64,
        var_nsubpp_dn6_slot: &mut f64,
        var_nsubpp_dn7_slot: &mut f64,
        var_nsubpp_dn8_slot: &mut f64,
        var_nsubpp_dn9_slot: &mut f64,
        var_nsubpp_rv_slot: &mut f64,
        var_nsubps_slot: &mut f64,
        var_nsubps_dn0_slot: &mut f64,
        var_nsubps_dn10_slot: &mut f64,
        var_nsubps_dn11_slot: &mut f64,
        var_nsubps_dn14_slot: &mut f64,
        var_nsubps_dn2_slot: &mut f64,
        var_nsubps_dn4_slot: &mut f64,
        var_nsubps_dn5_slot: &mut f64,
        var_nsubps_dn6_slot: &mut f64,
        var_nsubps_dn7_slot: &mut f64,
        var_nsubps_dn8_slot: &mut f64,
        var_nsubps_dn9_slot: &mut f64,
        var_nsubps_rv_slot: &mut f64,
        var_q_nsub_slot: &mut f64,
        var_q_nsub_dn0_slot: &mut f64,
        var_q_nsub_dn10_slot: &mut f64,
        var_q_nsub_dn11_slot: &mut f64,
        var_q_nsub_dn14_slot: &mut f64,
        var_q_nsub_dn2_slot: &mut f64,
        var_q_nsub_dn4_slot: &mut f64,
        var_q_nsub_dn5_slot: &mut f64,
        var_q_nsub_dn6_slot: &mut f64,
        var_q_nsub_dn7_slot: &mut f64,
        var_q_nsub_dn8_slot: &mut f64,
        var_q_nsub_dn9_slot: &mut f64,
        var_q_nsub_rv_slot: &mut f64,
        var_qnsub_esi_slot: &mut f64,
        var_qnsub_esi2_slot: &mut f64,
        var_qnsub_esi2_dn0_slot: &mut f64,
        var_qnsub_esi2_dn10_slot: &mut f64,
        var_qnsub_esi2_dn11_slot: &mut f64,
        var_qnsub_esi2_dn14_slot: &mut f64,
        var_qnsub_esi2_dn2_slot: &mut f64,
        var_qnsub_esi2_dn4_slot: &mut f64,
        var_qnsub_esi2_dn5_slot: &mut f64,
        var_qnsub_esi2_dn6_slot: &mut f64,
        var_qnsub_esi2_dn7_slot: &mut f64,
        var_qnsub_esi2_dn8_slot: &mut f64,
        var_qnsub_esi2_dn9_slot: &mut f64,
        var_qnsub_esi2_rv_slot: &mut f64,
        var_qnsub_esi_dn0_slot: &mut f64,
        var_qnsub_esi_dn10_slot: &mut f64,
        var_qnsub_esi_dn11_slot: &mut f64,
        var_qnsub_esi_dn14_slot: &mut f64,
        var_qnsub_esi_dn2_slot: &mut f64,
        var_qnsub_esi_dn4_slot: &mut f64,
        var_qnsub_esi_dn5_slot: &mut f64,
        var_qnsub_esi_dn6_slot: &mut f64,
        var_qnsub_esi_dn7_slot: &mut f64,
        var_qnsub_esi_dn8_slot: &mut f64,
        var_qnsub_esi_dn9_slot: &mut f64,
        var_qnsub_esi_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn11_slot: &mut f64,
        var_t0_dn14_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn14_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn11_slot: &mut f64,
        var_tmf1_dn14_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn7_slot: &mut f64,
        var_tmf1_dn8_slot: &mut f64,
        var_tmf1_dn9_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn11_slot: &mut f64,
        var_tmf2_dn14_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn7_slot: &mut f64,
        var_tmf2_dn8_slot: &mut f64,
        var_tmf2_dn9_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
    ) {
        let mut var_ef_nsubc: f64 = *var_ef_nsubc_slot;
        let mut var_ef_nsubc_dn0: f64 = *var_ef_nsubc_dn0_slot;
        let mut var_ef_nsubc_dn10: f64 = *var_ef_nsubc_dn10_slot;
        let mut var_ef_nsubc_dn11: f64 = *var_ef_nsubc_dn11_slot;
        let mut var_ef_nsubc_dn14: f64 = *var_ef_nsubc_dn14_slot;
        let mut var_ef_nsubc_dn2: f64 = *var_ef_nsubc_dn2_slot;
        let mut var_ef_nsubc_dn4: f64 = *var_ef_nsubc_dn4_slot;
        let mut var_ef_nsubc_dn5: f64 = *var_ef_nsubc_dn5_slot;
        let mut var_ef_nsubc_dn6: f64 = *var_ef_nsubc_dn6_slot;
        let mut var_ef_nsubc_dn7: f64 = *var_ef_nsubc_dn7_slot;
        let mut var_ef_nsubc_dn8: f64 = *var_ef_nsubc_dn8_slot;
        let mut var_ef_nsubc_dn9: f64 = *var_ef_nsubc_dn9_slot;
        let mut var_ef_nsubc_rv: f64 = *var_ef_nsubc_rv_slot;
        let mut var_guard267: f64 = *var_guard267_slot;
        let mut var_guard267_rv: f64 = *var_guard267_rv_slot;
        let mut var_guard268: f64 = *var_guard268_slot;
        let mut var_guard268_rv: f64 = *var_guard268_rv_slot;
        let mut var_guard269: f64 = *var_guard269_slot;
        let mut var_guard269_rv: f64 = *var_guard269_rv_slot;
        let mut var_guard270: f64 = *var_guard270_slot;
        let mut var_guard270_rv: f64 = *var_guard270_rv_slot;
        let mut var_nsub: f64 = *var_nsub_slot;
        let mut var_nsub_dn0: f64 = *var_nsub_dn0_slot;
        let mut var_nsub_dn10: f64 = *var_nsub_dn10_slot;
        let mut var_nsub_dn11: f64 = *var_nsub_dn11_slot;
        let mut var_nsub_dn14: f64 = *var_nsub_dn14_slot;
        let mut var_nsub_dn2: f64 = *var_nsub_dn2_slot;
        let mut var_nsub_dn4: f64 = *var_nsub_dn4_slot;
        let mut var_nsub_dn5: f64 = *var_nsub_dn5_slot;
        let mut var_nsub_dn6: f64 = *var_nsub_dn6_slot;
        let mut var_nsub_dn7: f64 = *var_nsub_dn7_slot;
        let mut var_nsub_dn8: f64 = *var_nsub_dn8_slot;
        let mut var_nsub_dn9: f64 = *var_nsub_dn9_slot;
        let mut var_nsub_rv: f64 = *var_nsub_rv_slot;
        let mut var_nsubb: f64 = *var_nsubb_slot;
        let mut var_nsubb_dn0: f64 = *var_nsubb_dn0_slot;
        let mut var_nsubb_dn10: f64 = *var_nsubb_dn10_slot;
        let mut var_nsubb_dn11: f64 = *var_nsubb_dn11_slot;
        let mut var_nsubb_dn14: f64 = *var_nsubb_dn14_slot;
        let mut var_nsubb_dn2: f64 = *var_nsubb_dn2_slot;
        let mut var_nsubb_dn4: f64 = *var_nsubb_dn4_slot;
        let mut var_nsubb_dn5: f64 = *var_nsubb_dn5_slot;
        let mut var_nsubb_dn6: f64 = *var_nsubb_dn6_slot;
        let mut var_nsubb_dn7: f64 = *var_nsubb_dn7_slot;
        let mut var_nsubb_dn8: f64 = *var_nsubb_dn8_slot;
        let mut var_nsubb_dn9: f64 = *var_nsubb_dn9_slot;
        let mut var_nsubb_rv: f64 = *var_nsubb_rv_slot;
        let mut var_nsubpp: f64 = *var_nsubpp_slot;
        let mut var_nsubpp_dn0: f64 = *var_nsubpp_dn0_slot;
        let mut var_nsubpp_dn10: f64 = *var_nsubpp_dn10_slot;
        let mut var_nsubpp_dn11: f64 = *var_nsubpp_dn11_slot;
        let mut var_nsubpp_dn14: f64 = *var_nsubpp_dn14_slot;
        let mut var_nsubpp_dn2: f64 = *var_nsubpp_dn2_slot;
        let mut var_nsubpp_dn4: f64 = *var_nsubpp_dn4_slot;
        let mut var_nsubpp_dn5: f64 = *var_nsubpp_dn5_slot;
        let mut var_nsubpp_dn6: f64 = *var_nsubpp_dn6_slot;
        let mut var_nsubpp_dn7: f64 = *var_nsubpp_dn7_slot;
        let mut var_nsubpp_dn8: f64 = *var_nsubpp_dn8_slot;
        let mut var_nsubpp_dn9: f64 = *var_nsubpp_dn9_slot;
        let mut var_nsubpp_rv: f64 = *var_nsubpp_rv_slot;
        let mut var_nsubps: f64 = *var_nsubps_slot;
        let mut var_nsubps_dn0: f64 = *var_nsubps_dn0_slot;
        let mut var_nsubps_dn10: f64 = *var_nsubps_dn10_slot;
        let mut var_nsubps_dn11: f64 = *var_nsubps_dn11_slot;
        let mut var_nsubps_dn14: f64 = *var_nsubps_dn14_slot;
        let mut var_nsubps_dn2: f64 = *var_nsubps_dn2_slot;
        let mut var_nsubps_dn4: f64 = *var_nsubps_dn4_slot;
        let mut var_nsubps_dn5: f64 = *var_nsubps_dn5_slot;
        let mut var_nsubps_dn6: f64 = *var_nsubps_dn6_slot;
        let mut var_nsubps_dn7: f64 = *var_nsubps_dn7_slot;
        let mut var_nsubps_dn8: f64 = *var_nsubps_dn8_slot;
        let mut var_nsubps_dn9: f64 = *var_nsubps_dn9_slot;
        let mut var_nsubps_rv: f64 = *var_nsubps_rv_slot;
        let mut var_q_nsub: f64 = *var_q_nsub_slot;
        let mut var_q_nsub_dn0: f64 = *var_q_nsub_dn0_slot;
        let mut var_q_nsub_dn10: f64 = *var_q_nsub_dn10_slot;
        let mut var_q_nsub_dn11: f64 = *var_q_nsub_dn11_slot;
        let mut var_q_nsub_dn14: f64 = *var_q_nsub_dn14_slot;
        let mut var_q_nsub_dn2: f64 = *var_q_nsub_dn2_slot;
        let mut var_q_nsub_dn4: f64 = *var_q_nsub_dn4_slot;
        let mut var_q_nsub_dn5: f64 = *var_q_nsub_dn5_slot;
        let mut var_q_nsub_dn6: f64 = *var_q_nsub_dn6_slot;
        let mut var_q_nsub_dn7: f64 = *var_q_nsub_dn7_slot;
        let mut var_q_nsub_dn8: f64 = *var_q_nsub_dn8_slot;
        let mut var_q_nsub_dn9: f64 = *var_q_nsub_dn9_slot;
        let mut var_q_nsub_rv: f64 = *var_q_nsub_rv_slot;
        let mut var_qnsub_esi: f64 = *var_qnsub_esi_slot;
        let mut var_qnsub_esi2: f64 = *var_qnsub_esi2_slot;
        let mut var_qnsub_esi2_dn0: f64 = *var_qnsub_esi2_dn0_slot;
        let mut var_qnsub_esi2_dn10: f64 = *var_qnsub_esi2_dn10_slot;
        let mut var_qnsub_esi2_dn11: f64 = *var_qnsub_esi2_dn11_slot;
        let mut var_qnsub_esi2_dn14: f64 = *var_qnsub_esi2_dn14_slot;
        let mut var_qnsub_esi2_dn2: f64 = *var_qnsub_esi2_dn2_slot;
        let mut var_qnsub_esi2_dn4: f64 = *var_qnsub_esi2_dn4_slot;
        let mut var_qnsub_esi2_dn5: f64 = *var_qnsub_esi2_dn5_slot;
        let mut var_qnsub_esi2_dn6: f64 = *var_qnsub_esi2_dn6_slot;
        let mut var_qnsub_esi2_dn7: f64 = *var_qnsub_esi2_dn7_slot;
        let mut var_qnsub_esi2_dn8: f64 = *var_qnsub_esi2_dn8_slot;
        let mut var_qnsub_esi2_dn9: f64 = *var_qnsub_esi2_dn9_slot;
        let mut var_qnsub_esi2_rv: f64 = *var_qnsub_esi2_rv_slot;
        let mut var_qnsub_esi_dn0: f64 = *var_qnsub_esi_dn0_slot;
        let mut var_qnsub_esi_dn10: f64 = *var_qnsub_esi_dn10_slot;
        let mut var_qnsub_esi_dn11: f64 = *var_qnsub_esi_dn11_slot;
        let mut var_qnsub_esi_dn14: f64 = *var_qnsub_esi_dn14_slot;
        let mut var_qnsub_esi_dn2: f64 = *var_qnsub_esi_dn2_slot;
        let mut var_qnsub_esi_dn4: f64 = *var_qnsub_esi_dn4_slot;
        let mut var_qnsub_esi_dn5: f64 = *var_qnsub_esi_dn5_slot;
        let mut var_qnsub_esi_dn6: f64 = *var_qnsub_esi_dn6_slot;
        let mut var_qnsub_esi_dn7: f64 = *var_qnsub_esi_dn7_slot;
        let mut var_qnsub_esi_dn8: f64 = *var_qnsub_esi_dn8_slot;
        let mut var_qnsub_esi_dn9: f64 = *var_qnsub_esi_dn9_slot;
        let mut var_qnsub_esi_rv: f64 = *var_qnsub_esi_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn11: f64 = *var_t0_dn11_slot;
        let mut var_t0_dn14: f64 = *var_t0_dn14_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn14: f64 = *var_t3_dn14_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn11: f64 = *var_tmf1_dn11_slot;
        let mut var_tmf1_dn14: f64 = *var_tmf1_dn14_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn7: f64 = *var_tmf1_dn7_slot;
        let mut var_tmf1_dn8: f64 = *var_tmf1_dn8_slot;
        let mut var_tmf1_dn9: f64 = *var_tmf1_dn9_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn11: f64 = *var_tmf2_dn11_slot;
        let mut var_tmf2_dn14: f64 = *var_tmf2_dn14_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn7: f64 = *var_tmf2_dn7_slot;
        let mut var_tmf2_dn8: f64 = *var_tmf2_dn8_slot;
        let mut var_tmf2_dn9: f64 = *var_tmf2_dn9_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;

        let (assign11410_e6159, assign11410_e6159_d_n0, assign11410_e6159_d_n2, assign11410_e6159_d_n4, assign11410_e6159_d_n5, assign11410_e6159_d_n6, assign11410_e6159_d_n7, assign11410_e6159_d_n8, assign11410_e6159_d_n9, assign11410_e6159_d_n10, assign11410_e6159_d_n11, assign11410_e6159_d_n14,) = {
    if (var_guard265 != 0.0) {
        (1000000000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign11410_e6159;
        var_t1_dn0 = assign11410_e6159_d_n0;
        var_t1_dn2 = assign11410_e6159_d_n2;
        var_t1_dn4 = assign11410_e6159_d_n4;
        var_t1_dn5 = assign11410_e6159_d_n5;
        var_t1_dn6 = assign11410_e6159_d_n6;
        var_t1_dn7 = assign11410_e6159_d_n7;
        var_t1_dn8 = assign11410_e6159_d_n8;
        var_t1_dn9 = assign11410_e6159_d_n9;
        var_t1_dn10 = assign11410_e6159_d_n10;
        var_t1_dn11 = assign11410_e6159_d_n11;
        var_t1_dn14 = assign11410_e6159_d_n14;
        var_t1_rv = 0.0;

        let assign11420_e6162: f64 = (var_t1 / 1e-6);
        var_ef_nsubc = assign11420_e6162;
        var_ef_nsubc_dn0 = (var_t1_dn0 / 1e-6);
        var_ef_nsubc_dn2 = (var_t1_dn2 / 1e-6);
        var_ef_nsubc_dn4 = (var_t1_dn4 / 1e-6);
        var_ef_nsubc_dn5 = (var_t1_dn5 / 1e-6);
        var_ef_nsubc_dn6 = (var_t1_dn6 / 1e-6);
        var_ef_nsubc_dn7 = (var_t1_dn7 / 1e-6);
        var_ef_nsubc_dn8 = (var_t1_dn8 / 1e-6);
        var_ef_nsubc_dn9 = (var_t1_dn9 / 1e-6);
        var_ef_nsubc_dn10 = (var_t1_dn10 / 1e-6);
        var_ef_nsubc_dn11 = (var_t1_dn11 / 1e-6);
        var_ef_nsubc_dn14 = (var_t1_dn14 / 1e-6);
        var_ef_nsubc_rv = 0.0;

        let assign11440_e6170: f64 = if var_t2 < 1000000000000000.0 { 1.0 } else { 0.0 };
        var_guard267 = assign11440_e6170;
        var_guard267_rv = 0.0;

        let (assign11450_e6174, assign11450_e6174_d_n0, assign11450_e6174_d_n2, assign11450_e6174_d_n4, assign11450_e6174_d_n5, assign11450_e6174_d_n6, assign11450_e6174_d_n7, assign11450_e6174_d_n8, assign11450_e6174_d_n9, assign11450_e6174_d_n10, assign11450_e6174_d_n11, assign11450_e6174_d_n14,) = {
    if (var_guard267 != 0.0) {
        (1000000000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    }
};
        var_t2 = assign11450_e6174;
        var_t2_dn0 = assign11450_e6174_d_n0;
        var_t2_dn2 = assign11450_e6174_d_n2;
        var_t2_dn4 = assign11450_e6174_d_n4;
        var_t2_dn5 = assign11450_e6174_d_n5;
        var_t2_dn6 = assign11450_e6174_d_n6;
        var_t2_dn7 = assign11450_e6174_d_n7;
        var_t2_dn8 = assign11450_e6174_d_n8;
        var_t2_dn9 = assign11450_e6174_d_n9;
        var_t2_dn10 = assign11450_e6174_d_n10;
        var_t2_dn11 = assign11450_e6174_d_n11;
        var_t2_dn14 = assign11450_e6174_d_n14;
        var_t2_rv = 0.0;

        let assign11460_e6177: f64 = (var_t2 / 1e-6);
        var_nsubpp = assign11460_e6177;
        var_nsubpp_dn0 = (var_t2_dn0 / 1e-6);
        var_nsubpp_dn2 = (var_t2_dn2 / 1e-6);
        var_nsubpp_dn4 = (var_t2_dn4 / 1e-6);
        var_nsubpp_dn5 = (var_t2_dn5 / 1e-6);
        var_nsubpp_dn6 = (var_t2_dn6 / 1e-6);
        var_nsubpp_dn7 = (var_t2_dn7 / 1e-6);
        var_nsubpp_dn8 = (var_t2_dn8 / 1e-6);
        var_nsubpp_dn9 = (var_t2_dn9 / 1e-6);
        var_nsubpp_dn10 = (var_t2_dn10 / 1e-6);
        var_nsubpp_dn11 = (var_t2_dn11 / 1e-6);
        var_nsubpp_dn14 = (var_t2_dn14 / 1e-6);
        var_nsubpp_rv = 0.0;

        let assign11470_e6180: f64 = if var_lod_half > 0.0 { 1.0 } else { 0.0 };
        var_guard268 = assign11470_e6180;
        var_guard268_rv = 0.0;

        let (assign11480_e6188, assign11480_e6188_d_n0, assign11480_e6188_d_n2, assign11480_e6188_d_n4, assign11480_e6188_d_n5, assign11480_e6188_d_n6, assign11480_e6188_d_n7, assign11480_e6188_d_n8, assign11480_e6188_d_n9, assign11480_e6188_d_n10, assign11480_e6188_d_n11, assign11480_e6188_d_n14,) = {
    if (var_guard268 != 0.0) {
        let assign11480_e6185: f64 = (1.0 + var_uc_nsubpsti2);
        let assign11480_e6186: f64 = (1.0 / assign11480_e6185);
        (assign11480_e6186, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign11480_e6188;
        var_t1_dn0 = assign11480_e6188_d_n0;
        var_t1_dn2 = assign11480_e6188_d_n2;
        var_t1_dn4 = assign11480_e6188_d_n4;
        var_t1_dn5 = assign11480_e6188_d_n5;
        var_t1_dn6 = assign11480_e6188_d_n6;
        var_t1_dn7 = assign11480_e6188_d_n7;
        var_t1_dn8 = assign11480_e6188_d_n8;
        var_t1_dn9 = assign11480_e6188_d_n9;
        var_t1_dn10 = assign11480_e6188_d_n10;
        var_t1_dn11 = assign11480_e6188_d_n11;
        var_t1_dn14 = assign11480_e6188_d_n14;
        var_t1_rv = 0.0;

        let (assign11490_e6196, assign11490_e6196_d_n0, assign11490_e6196_d_n2, assign11490_e6196_d_n4, assign11490_e6196_d_n5, assign11490_e6196_d_n6, assign11490_e6196_d_n7, assign11490_e6196_d_n8, assign11490_e6196_d_n9, assign11490_e6196_d_n10, assign11490_e6196_d_n11, assign11490_e6196_d_n14,) = {
    if (var_guard268 != 0.0) {
        let assign11490_e6192: f64 = (var_uc_nsubpsti1 / var_lod_half);
        let assign11490_e6194: f64 = (assign11490_e6192).powf(var_uc_nsubpsti3);
        (assign11490_e6194, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11490_e6192).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn0) / (var_lod_half * var_lod_half))))) } } else { (assign11490_e6194 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn0) / (var_lod_half * var_lod_half))) / assign11490_e6192))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11490_e6192).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn2) / (var_lod_half * var_lod_half))))) } } else { (assign11490_e6194 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn2) / (var_lod_half * var_lod_half))) / assign11490_e6192))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11490_e6192).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn4) / (var_lod_half * var_lod_half))))) } } else { (assign11490_e6194 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn4) / (var_lod_half * var_lod_half))) / assign11490_e6192))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11490_e6192).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn5) / (var_lod_half * var_lod_half))))) } } else { (assign11490_e6194 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn5) / (var_lod_half * var_lod_half))) / assign11490_e6192))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11490_e6192).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn6) / (var_lod_half * var_lod_half))))) } } else { (assign11490_e6194 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn6) / (var_lod_half * var_lod_half))) / assign11490_e6192))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11490_e6192).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn7) / (var_lod_half * var_lod_half))))) } } else { (assign11490_e6194 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn7) / (var_lod_half * var_lod_half))) / assign11490_e6192))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11490_e6192).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn8) / (var_lod_half * var_lod_half))))) } } else { (assign11490_e6194 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn8) / (var_lod_half * var_lod_half))) / assign11490_e6192))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11490_e6192).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn9) / (var_lod_half * var_lod_half))))) } } else { (assign11490_e6194 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn9) / (var_lod_half * var_lod_half))) / assign11490_e6192))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11490_e6192).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn10) / (var_lod_half * var_lod_half))))) } } else { (assign11490_e6194 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn10) / (var_lod_half * var_lod_half))) / assign11490_e6192))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11490_e6192).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn11) / (var_lod_half * var_lod_half))))) } } else { (assign11490_e6194 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn11) / (var_lod_half * var_lod_half))) / assign11490_e6192))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11490_e6192).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_dn14) / (var_lod_half * var_lod_half))))) } } else { (assign11490_e6194 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_dn14) / (var_lod_half * var_lod_half))) / assign11490_e6192))) },)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    }
};
        var_t2 = assign11490_e6196;
        var_t2_dn0 = assign11490_e6196_d_n0;
        var_t2_dn2 = assign11490_e6196_d_n2;
        var_t2_dn4 = assign11490_e6196_d_n4;
        var_t2_dn5 = assign11490_e6196_d_n5;
        var_t2_dn6 = assign11490_e6196_d_n6;
        var_t2_dn7 = assign11490_e6196_d_n7;
        var_t2_dn8 = assign11490_e6196_d_n8;
        var_t2_dn9 = assign11490_e6196_d_n9;
        var_t2_dn10 = assign11490_e6196_d_n10;
        var_t2_dn11 = assign11490_e6196_d_n11;
        var_t2_dn14 = assign11490_e6196_d_n14;
        var_t2_rv = 0.0;

        let (assign11500_e6204, assign11500_e6204_d_n0, assign11500_e6204_d_n2, assign11500_e6204_d_n4, assign11500_e6204_d_n5, assign11500_e6204_d_n6, assign11500_e6204_d_n7, assign11500_e6204_d_n8, assign11500_e6204_d_n9, assign11500_e6204_d_n10, assign11500_e6204_d_n11, assign11500_e6204_d_n14,) = {
    if (var_guard268 != 0.0) {
        let assign11500_e6200: f64 = (var_uc_nsubpsti1 / var_lod_half_ref);
        let assign11500_e6202: f64 = (assign11500_e6200).powf(var_uc_nsubpsti3);
        (assign11500_e6202, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11500_e6200).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn0) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11500_e6202 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn0) / (var_lod_half_ref * var_lod_half_ref))) / assign11500_e6200))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11500_e6200).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn2) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11500_e6202 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn2) / (var_lod_half_ref * var_lod_half_ref))) / assign11500_e6200))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11500_e6200).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn4) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11500_e6202 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn4) / (var_lod_half_ref * var_lod_half_ref))) / assign11500_e6200))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11500_e6200).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn5) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11500_e6202 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn5) / (var_lod_half_ref * var_lod_half_ref))) / assign11500_e6200))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11500_e6200).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn6) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11500_e6202 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn6) / (var_lod_half_ref * var_lod_half_ref))) / assign11500_e6200))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11500_e6200).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn7) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11500_e6202 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn7) / (var_lod_half_ref * var_lod_half_ref))) / assign11500_e6200))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11500_e6200).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn8) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11500_e6202 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn8) / (var_lod_half_ref * var_lod_half_ref))) / assign11500_e6200))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11500_e6200).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn9) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11500_e6202 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn9) / (var_lod_half_ref * var_lod_half_ref))) / assign11500_e6200))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11500_e6200).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn10) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11500_e6202 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn10) / (var_lod_half_ref * var_lod_half_ref))) / assign11500_e6200))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11500_e6200).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn11) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11500_e6202 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn11) / (var_lod_half_ref * var_lod_half_ref))) / assign11500_e6200))) }, if 0.0 == 0.0 && ((var_uc_nsubpsti3) as f64).is_finite() && ((var_uc_nsubpsti3) as f64).fract() == 0.0 { if var_uc_nsubpsti3 == 0.0 { 0.0 } else { (var_uc_nsubpsti3 * ((assign11500_e6200).powf(var_uc_nsubpsti3 - 1.0) * (-((var_uc_nsubpsti1 * var_lod_half_ref_dn14) / (var_lod_half_ref * var_lod_half_ref))))) } } else { (assign11500_e6202 * (var_uc_nsubpsti3 * ((-((var_uc_nsubpsti1 * var_lod_half_ref_dn14) / (var_lod_half_ref * var_lod_half_ref))) / assign11500_e6200))) },)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn14,)
    }
};
        var_t3 = assign11500_e6204;
        var_t3_dn0 = assign11500_e6204_d_n0;
        var_t3_dn2 = assign11500_e6204_d_n2;
        var_t3_dn4 = assign11500_e6204_d_n4;
        var_t3_dn5 = assign11500_e6204_d_n5;
        var_t3_dn6 = assign11500_e6204_d_n6;
        var_t3_dn7 = assign11500_e6204_d_n7;
        var_t3_dn8 = assign11500_e6204_d_n8;
        var_t3_dn9 = assign11500_e6204_d_n9;
        var_t3_dn10 = assign11500_e6204_d_n10;
        var_t3_dn11 = assign11500_e6204_d_n11;
        var_t3_dn14 = assign11500_e6204_d_n14;
        var_t3_rv = 0.0;

        let (assign11510_e6220, assign11510_e6220_d_n0, assign11510_e6220_d_n2, assign11510_e6220_d_n4, assign11510_e6220_d_n5, assign11510_e6220_d_n6, assign11510_e6220_d_n7, assign11510_e6220_d_n8, assign11510_e6220_d_n9, assign11510_e6220_d_n10, assign11510_e6220_d_n11, assign11510_e6220_d_n14,) = {
    if (var_guard268 != 0.0) {
        let assign11510_e6210: f64 = (var_t1 * var_t2);
        let assign11510_e6211: f64 = (1.0 + assign11510_e6210);
        let assign11510_e6212: f64 = (var_nsubpp * assign11510_e6211);
        let assign11510_e6216: f64 = (var_t1 * var_t3);
        let assign11510_e6217: f64 = (1.0 + assign11510_e6216);
        let assign11510_e6218: f64 = (assign11510_e6212 / assign11510_e6217);
        (assign11510_e6218, (((((var_nsubpp_dn0 * assign11510_e6211) + (var_nsubpp * ((var_t1_dn0 * var_t2) + (var_t1 * var_t2_dn0)))) * assign11510_e6217) - (assign11510_e6212 * ((var_t1_dn0 * var_t3) + (var_t1 * var_t3_dn0)))) / (assign11510_e6217 * assign11510_e6217)), (((((var_nsubpp_dn2 * assign11510_e6211) + (var_nsubpp * ((var_t1_dn2 * var_t2) + (var_t1 * var_t2_dn2)))) * assign11510_e6217) - (assign11510_e6212 * ((var_t1_dn2 * var_t3) + (var_t1 * var_t3_dn2)))) / (assign11510_e6217 * assign11510_e6217)), (((((var_nsubpp_dn4 * assign11510_e6211) + (var_nsubpp * ((var_t1_dn4 * var_t2) + (var_t1 * var_t2_dn4)))) * assign11510_e6217) - (assign11510_e6212 * ((var_t1_dn4 * var_t3) + (var_t1 * var_t3_dn4)))) / (assign11510_e6217 * assign11510_e6217)), (((((var_nsubpp_dn5 * assign11510_e6211) + (var_nsubpp * ((var_t1_dn5 * var_t2) + (var_t1 * var_t2_dn5)))) * assign11510_e6217) - (assign11510_e6212 * ((var_t1_dn5 * var_t3) + (var_t1 * var_t3_dn5)))) / (assign11510_e6217 * assign11510_e6217)), (((((var_nsubpp_dn6 * assign11510_e6211) + (var_nsubpp * ((var_t1_dn6 * var_t2) + (var_t1 * var_t2_dn6)))) * assign11510_e6217) - (assign11510_e6212 * ((var_t1_dn6 * var_t3) + (var_t1 * var_t3_dn6)))) / (assign11510_e6217 * assign11510_e6217)), (((((var_nsubpp_dn7 * assign11510_e6211) + (var_nsubpp * ((var_t1_dn7 * var_t2) + (var_t1 * var_t2_dn7)))) * assign11510_e6217) - (assign11510_e6212 * ((var_t1_dn7 * var_t3) + (var_t1 * var_t3_dn7)))) / (assign11510_e6217 * assign11510_e6217)), (((((var_nsubpp_dn8 * assign11510_e6211) + (var_nsubpp * ((var_t1_dn8 * var_t2) + (var_t1 * var_t2_dn8)))) * assign11510_e6217) - (assign11510_e6212 * ((var_t1_dn8 * var_t3) + (var_t1 * var_t3_dn8)))) / (assign11510_e6217 * assign11510_e6217)), (((((var_nsubpp_dn9 * assign11510_e6211) + (var_nsubpp * ((var_t1_dn9 * var_t2) + (var_t1 * var_t2_dn9)))) * assign11510_e6217) - (assign11510_e6212 * ((var_t1_dn9 * var_t3) + (var_t1 * var_t3_dn9)))) / (assign11510_e6217 * assign11510_e6217)), (((((var_nsubpp_dn10 * assign11510_e6211) + (var_nsubpp * ((var_t1_dn10 * var_t2) + (var_t1 * var_t2_dn10)))) * assign11510_e6217) - (assign11510_e6212 * ((var_t1_dn10 * var_t3) + (var_t1 * var_t3_dn10)))) / (assign11510_e6217 * assign11510_e6217)), (((((var_nsubpp_dn11 * assign11510_e6211) + (var_nsubpp * ((var_t1_dn11 * var_t2) + (var_t1 * var_t2_dn11)))) * assign11510_e6217) - (assign11510_e6212 * ((var_t1_dn11 * var_t3) + (var_t1 * var_t3_dn11)))) / (assign11510_e6217 * assign11510_e6217)), (((((var_nsubpp_dn14 * assign11510_e6211) + (var_nsubpp * ((var_t1_dn14 * var_t2) + (var_t1 * var_t2_dn14)))) * assign11510_e6217) - (assign11510_e6212 * ((var_t1_dn14 * var_t3) + (var_t1 * var_t3_dn14)))) / (assign11510_e6217 * assign11510_e6217)),)
    } else {
        (var_nsubps, var_nsubps_dn0, var_nsubps_dn2, var_nsubps_dn4, var_nsubps_dn5, var_nsubps_dn6, var_nsubps_dn7, var_nsubps_dn8, var_nsubps_dn9, var_nsubps_dn10, var_nsubps_dn11, var_nsubps_dn14,)
    }
};
        var_nsubps = assign11510_e6220;
        var_nsubps_dn0 = assign11510_e6220_d_n0;
        var_nsubps_dn2 = assign11510_e6220_d_n2;
        var_nsubps_dn4 = assign11510_e6220_d_n4;
        var_nsubps_dn5 = assign11510_e6220_d_n5;
        var_nsubps_dn6 = assign11510_e6220_d_n6;
        var_nsubps_dn7 = assign11510_e6220_d_n7;
        var_nsubps_dn8 = assign11510_e6220_d_n8;
        var_nsubps_dn9 = assign11510_e6220_d_n9;
        var_nsubps_dn10 = assign11510_e6220_d_n10;
        var_nsubps_dn11 = assign11510_e6220_d_n11;
        var_nsubps_dn14 = assign11510_e6220_d_n14;
        var_nsubps_rv = 0.0;

        let (assign11520_e6225, assign11520_e6225_d_n0, assign11520_e6225_d_n2, assign11520_e6225_d_n4, assign11520_e6225_d_n5, assign11520_e6225_d_n6, assign11520_e6225_d_n7, assign11520_e6225_d_n8, assign11520_e6225_d_n9, assign11520_e6225_d_n10, assign11520_e6225_d_n11, assign11520_e6225_d_n14,) = {
    if (var_guard268 == 0.0) {
        (var_nsubpp, var_nsubpp_dn0, var_nsubpp_dn2, var_nsubpp_dn4, var_nsubpp_dn5, var_nsubpp_dn6, var_nsubpp_dn7, var_nsubpp_dn8, var_nsubpp_dn9, var_nsubpp_dn10, var_nsubpp_dn11, var_nsubpp_dn14,)
    } else {
        (var_nsubps, var_nsubps_dn0, var_nsubps_dn2, var_nsubps_dn4, var_nsubps_dn5, var_nsubps_dn6, var_nsubps_dn7, var_nsubps_dn8, var_nsubps_dn9, var_nsubps_dn10, var_nsubps_dn11, var_nsubps_dn14,)
    }
};
        var_nsubps = assign11520_e6225;
        var_nsubps_dn0 = assign11520_e6225_d_n0;
        var_nsubps_dn2 = assign11520_e6225_d_n2;
        var_nsubps_dn4 = assign11520_e6225_d_n4;
        var_nsubps_dn5 = assign11520_e6225_d_n5;
        var_nsubps_dn6 = assign11520_e6225_d_n6;
        var_nsubps_dn7 = assign11520_e6225_d_n7;
        var_nsubps_dn8 = assign11520_e6225_d_n8;
        var_nsubps_dn9 = assign11520_e6225_d_n9;
        var_nsubps_dn10 = assign11520_e6225_d_n10;
        var_nsubps_dn11 = assign11520_e6225_d_n11;
        var_nsubps_dn14 = assign11520_e6225_d_n14;
        var_nsubps_rv = 0.0;

        let assign11530_e6232: f64 = if ((var_lgate > p.p140) || (p.p140 <= 0.0)) { 1.0 } else { 0.0 };
        var_guard269 = assign11530_e6232;
        var_guard269_rv = 0.0;

        let (assign11540_e6246, assign11540_e6246_d_n0, assign11540_e6246_d_n2, assign11540_e6246_d_n4, assign11540_e6246_d_n5, assign11540_e6246_d_n6, assign11540_e6246_d_n7, assign11540_e6246_d_n8, assign11540_e6246_d_n9, assign11540_e6246_d_n10, assign11540_e6246_d_n11, assign11540_e6246_d_n14,) = {
    if (var_guard269 != 0.0) {
        let assign11540_e6237: f64 = (var_lgate - p.p140);
        let assign11540_e6238: f64 = (var_ef_nsubc * assign11540_e6237);
        let assign11540_e6241: f64 = (var_nsubps * p.p140);
        let assign11540_e6242: f64 = (assign11540_e6238 + assign11540_e6241);
        let assign11540_e6244: f64 = (assign11540_e6242 / var_lgate);
        (assign11540_e6244, (((var_ef_nsubc_dn0 * assign11540_e6237) + (var_nsubps_dn0 * p.p140)) / var_lgate), (((var_ef_nsubc_dn2 * assign11540_e6237) + (var_nsubps_dn2 * p.p140)) / var_lgate), (((var_ef_nsubc_dn4 * assign11540_e6237) + (var_nsubps_dn4 * p.p140)) / var_lgate), (((var_ef_nsubc_dn5 * assign11540_e6237) + (var_nsubps_dn5 * p.p140)) / var_lgate), (((var_ef_nsubc_dn6 * assign11540_e6237) + (var_nsubps_dn6 * p.p140)) / var_lgate), (((var_ef_nsubc_dn7 * assign11540_e6237) + (var_nsubps_dn7 * p.p140)) / var_lgate), (((var_ef_nsubc_dn8 * assign11540_e6237) + (var_nsubps_dn8 * p.p140)) / var_lgate), (((var_ef_nsubc_dn9 * assign11540_e6237) + (var_nsubps_dn9 * p.p140)) / var_lgate), (((var_ef_nsubc_dn10 * assign11540_e6237) + (var_nsubps_dn10 * p.p140)) / var_lgate), (((var_ef_nsubc_dn11 * assign11540_e6237) + (var_nsubps_dn11 * p.p140)) / var_lgate), (((var_ef_nsubc_dn14 * assign11540_e6237) + (var_nsubps_dn14 * p.p140)) / var_lgate),)
    } else {
        (var_nsub, var_nsub_dn0, var_nsub_dn2, var_nsub_dn4, var_nsub_dn5, var_nsub_dn6, var_nsub_dn7, var_nsub_dn8, var_nsub_dn9, var_nsub_dn10, var_nsub_dn11, var_nsub_dn14,)
    }
};
        var_nsub = assign11540_e6246;
        var_nsub_dn0 = assign11540_e6246_d_n0;
        var_nsub_dn2 = assign11540_e6246_d_n2;
        var_nsub_dn4 = assign11540_e6246_d_n4;
        var_nsub_dn5 = assign11540_e6246_d_n5;
        var_nsub_dn6 = assign11540_e6246_d_n6;
        var_nsub_dn7 = assign11540_e6246_d_n7;
        var_nsub_dn8 = assign11540_e6246_d_n8;
        var_nsub_dn9 = assign11540_e6246_d_n9;
        var_nsub_dn10 = assign11540_e6246_d_n10;
        var_nsub_dn11 = assign11540_e6246_d_n11;
        var_nsub_dn14 = assign11540_e6246_d_n14;
        var_nsub_rv = 0.0;

        let (assign11550_e6261, assign11550_e6261_d_n0, assign11550_e6261_d_n2, assign11550_e6261_d_n4, assign11550_e6261_d_n5, assign11550_e6261_d_n6, assign11550_e6261_d_n7, assign11550_e6261_d_n8, assign11550_e6261_d_n9, assign11550_e6261_d_n10, assign11550_e6261_d_n11, assign11550_e6261_d_n14,) = {
    if (var_guard269 == 0.0) {
        let assign11550_e6252: f64 = (var_nsubps - var_ef_nsubc);
        let assign11550_e6255: f64 = (p.p140 - var_lgate);
        let assign11550_e6256: f64 = (assign11550_e6252 * assign11550_e6255);
        let assign11550_e6258: f64 = (assign11550_e6256 / p.p140);
        let assign11550_e6259: f64 = (var_nsubps + assign11550_e6258);
        (assign11550_e6259, (var_nsubps_dn0 + (((var_nsubps_dn0 - var_ef_nsubc_dn0) * assign11550_e6255) / p.p140)), (var_nsubps_dn2 + (((var_nsubps_dn2 - var_ef_nsubc_dn2) * assign11550_e6255) / p.p140)), (var_nsubps_dn4 + (((var_nsubps_dn4 - var_ef_nsubc_dn4) * assign11550_e6255) / p.p140)), (var_nsubps_dn5 + (((var_nsubps_dn5 - var_ef_nsubc_dn5) * assign11550_e6255) / p.p140)), (var_nsubps_dn6 + (((var_nsubps_dn6 - var_ef_nsubc_dn6) * assign11550_e6255) / p.p140)), (var_nsubps_dn7 + (((var_nsubps_dn7 - var_ef_nsubc_dn7) * assign11550_e6255) / p.p140)), (var_nsubps_dn8 + (((var_nsubps_dn8 - var_ef_nsubc_dn8) * assign11550_e6255) / p.p140)), (var_nsubps_dn9 + (((var_nsubps_dn9 - var_ef_nsubc_dn9) * assign11550_e6255) / p.p140)), (var_nsubps_dn10 + (((var_nsubps_dn10 - var_ef_nsubc_dn10) * assign11550_e6255) / p.p140)), (var_nsubps_dn11 + (((var_nsubps_dn11 - var_ef_nsubc_dn11) * assign11550_e6255) / p.p140)), (var_nsubps_dn14 + (((var_nsubps_dn14 - var_ef_nsubc_dn14) * assign11550_e6255) / p.p140)),)
    } else {
        (var_nsub, var_nsub_dn0, var_nsub_dn2, var_nsub_dn4, var_nsub_dn5, var_nsub_dn6, var_nsub_dn7, var_nsub_dn8, var_nsub_dn9, var_nsub_dn10, var_nsub_dn11, var_nsub_dn14,)
    }
};
        var_nsub = assign11550_e6261;
        var_nsub_dn0 = assign11550_e6261_d_n0;
        var_nsub_dn2 = assign11550_e6261_d_n2;
        var_nsub_dn4 = assign11550_e6261_d_n4;
        var_nsub_dn5 = assign11550_e6261_d_n5;
        var_nsub_dn6 = assign11550_e6261_d_n6;
        var_nsub_dn7 = assign11550_e6261_d_n7;
        var_nsub_dn8 = assign11550_e6261_d_n8;
        var_nsub_dn9 = assign11550_e6261_d_n9;
        var_nsub_dn10 = assign11550_e6261_d_n10;
        var_nsub_dn11 = assign11550_e6261_d_n11;
        var_nsub_dn14 = assign11550_e6261_d_n14;
        var_nsub_rv = 0.0;

        let assign11560_e6264: f64 = (0.5 * var_lgate);
        let assign11560_e6266: f64 = (assign11560_e6264 - p.p140);
        var_t3 = assign11560_e6266;
        var_t3_dn0 = 0.0;
        var_t3_dn2 = 0.0;
        var_t3_dn4 = 0.0;
        var_t3_dn5 = 0.0;
        var_t3_dn6 = 0.0;
        var_t3_dn7 = 0.0;
        var_t3_dn8 = 0.0;
        var_t3_dn9 = 0.0;
        var_t3_dn10 = 0.0;
        var_t3_dn11 = 0.0;
        var_t3_dn14 = 0.0;
        var_t3_rv = 0.0;

        let assign11570_e6269: f64 = (var_t3 - 1e-9);
        let assign11570_e6271: f64 = (assign11570_e6269 - 1e-10);
        var_tmf1 = assign11570_e6271;
        var_tmf1_dn0 = var_t3_dn0;
        var_tmf1_dn2 = var_t3_dn2;
        var_tmf1_dn4 = var_t3_dn4;
        var_tmf1_dn5 = var_t3_dn5;
        var_tmf1_dn6 = var_t3_dn6;
        var_tmf1_dn7 = var_t3_dn7;
        var_tmf1_dn8 = var_t3_dn8;
        var_tmf1_dn9 = var_t3_dn9;
        var_tmf1_dn10 = var_t3_dn10;
        var_tmf1_dn11 = var_t3_dn11;
        var_tmf1_dn14 = var_t3_dn14;
        var_tmf1_rv = 0.0;

        let assign11580_e6274: f64 = (4.0 * 1e-9);
        let assign11580_e6276: f64 = (assign11580_e6274 * 1e-10);
        var_tmf2 = assign11580_e6276;
        var_tmf2_dn0 = 0.0;
        var_tmf2_dn2 = 0.0;
        var_tmf2_dn4 = 0.0;
        var_tmf2_dn5 = 0.0;
        var_tmf2_dn6 = 0.0;
        var_tmf2_dn7 = 0.0;
        var_tmf2_dn8 = 0.0;
        var_tmf2_dn9 = 0.0;
        var_tmf2_dn10 = 0.0;
        var_tmf2_dn11 = 0.0;
        var_tmf2_dn14 = 0.0;
        var_tmf2_rv = 0.0;

        let (assign11590_e6283, assign11590_e6283_d_n0, assign11590_e6283_d_n2, assign11590_e6283_d_n4, assign11590_e6283_d_n5, assign11590_e6283_d_n6, assign11590_e6283_d_n7, assign11590_e6283_d_n8, assign11590_e6283_d_n9, assign11590_e6283_d_n10, assign11590_e6283_d_n11, assign11590_e6283_d_n14,) = {
    if (var_tmf2 > 0.0) {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn11, var_tmf2_dn14,)
    } else {
        let assign11590_e6282: f64 = (-var_tmf2);
        (assign11590_e6282, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn8), (-var_tmf2_dn9), (-var_tmf2_dn10), (-var_tmf2_dn11), (-var_tmf2_dn14),)
    }
};
        var_tmf2 = assign11590_e6283;
        var_tmf2_dn0 = assign11590_e6283_d_n0;
        var_tmf2_dn2 = assign11590_e6283_d_n2;
        var_tmf2_dn4 = assign11590_e6283_d_n4;
        var_tmf2_dn5 = assign11590_e6283_d_n5;
        var_tmf2_dn6 = assign11590_e6283_d_n6;
        var_tmf2_dn7 = assign11590_e6283_d_n7;
        var_tmf2_dn8 = assign11590_e6283_d_n8;
        var_tmf2_dn9 = assign11590_e6283_d_n9;
        var_tmf2_dn10 = assign11590_e6283_d_n10;
        var_tmf2_dn11 = assign11590_e6283_d_n11;
        var_tmf2_dn14 = assign11590_e6283_d_n14;
        var_tmf2_rv = 0.0;

        let assign11600_e6286: f64 = (var_tmf1 * var_tmf1);
        let assign11600_e6288: f64 = (assign11600_e6286 + var_tmf2);
        let assign11600_e6289: f64 = (assign11600_e6288).sqrt();
        var_tmf2 = assign11600_e6289;
        var_tmf2_dn0 = ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign11600_e6289));
        var_tmf2_dn2 = ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign11600_e6289));
        var_tmf2_dn4 = ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign11600_e6289));
        var_tmf2_dn5 = ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign11600_e6289));
        var_tmf2_dn6 = ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign11600_e6289));
        var_tmf2_dn7 = ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign11600_e6289));
        var_tmf2_dn8 = ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign11600_e6289));
        var_tmf2_dn9 = ((((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)) + var_tmf2_dn9) / (2.0 * assign11600_e6289));
        var_tmf2_dn10 = ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign11600_e6289));
        var_tmf2_dn11 = ((((var_tmf1_dn11 * var_tmf1) + (var_tmf1 * var_tmf1_dn11)) + var_tmf2_dn11) / (2.0 * assign11600_e6289));
        var_tmf2_dn14 = ((((var_tmf1_dn14 * var_tmf1) + (var_tmf1 * var_tmf1_dn14)) + var_tmf2_dn14) / (2.0 * assign11600_e6289));
        var_tmf2_rv = 0.0;

        let assign11610_e6294: f64 = (var_tmf1 / var_tmf2);
        let assign11610_e6295: f64 = (1.0 + assign11610_e6294);
        let assign11610_e6296: f64 = (0.5 * assign11610_e6295);
        var_t0 = assign11610_e6296;
        var_t0_dn0 = (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2)));
        var_t0_dn2 = (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2)));
        var_t0_dn4 = (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2)));
        var_t0_dn5 = (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2)));
        var_t0_dn6 = (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2)));
        var_t0_dn7 = (0.5 * (((var_tmf1_dn7 * var_tmf2) - (var_tmf1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2)));
        var_t0_dn8 = (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2)));
        var_t0_dn9 = (0.5 * (((var_tmf1_dn9 * var_tmf2) - (var_tmf1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2)));
        var_t0_dn10 = (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2)));
        var_t0_dn11 = (0.5 * (((var_tmf1_dn11 * var_tmf2) - (var_tmf1 * var_tmf2_dn11)) / (var_tmf2 * var_tmf2)));
        var_t0_dn14 = (0.5 * (((var_tmf1_dn14 * var_tmf2) - (var_tmf1 * var_tmf2_dn14)) / (var_tmf2 * var_tmf2)));
        var_t0_rv = 0.0;

        let assign11620_e6301: f64 = (var_tmf1 + var_tmf2);
        let assign11620_e6302: f64 = (0.5 * assign11620_e6301);
        let assign11620_e6303: f64 = (1e-9 + assign11620_e6302);
        var_t3 = assign11620_e6303;
        var_t3_dn0 = (0.5 * (var_tmf1_dn0 + var_tmf2_dn0));
        var_t3_dn2 = (0.5 * (var_tmf1_dn2 + var_tmf2_dn2));
        var_t3_dn4 = (0.5 * (var_tmf1_dn4 + var_tmf2_dn4));
        var_t3_dn5 = (0.5 * (var_tmf1_dn5 + var_tmf2_dn5));
        var_t3_dn6 = (0.5 * (var_tmf1_dn6 + var_tmf2_dn6));
        var_t3_dn7 = (0.5 * (var_tmf1_dn7 + var_tmf2_dn7));
        var_t3_dn8 = (0.5 * (var_tmf1_dn8 + var_tmf2_dn8));
        var_t3_dn9 = (0.5 * (var_tmf1_dn9 + var_tmf2_dn9));
        var_t3_dn10 = (0.5 * (var_tmf1_dn10 + var_tmf2_dn10));
        var_t3_dn11 = (0.5 * (var_tmf1_dn11 + var_tmf2_dn11));
        var_t3_dn14 = (0.5 * (var_tmf1_dn14 + var_tmf2_dn14));
        var_t3_rv = 0.0;

        let assign11630_e6307: f64 = (1.0 / var_t3);
        let assign11630_e6310: f64 = (1.0 / p.p220);
        let assign11630_e6311: f64 = (assign11630_e6307 + assign11630_e6310);
        let assign11630_e6312: f64 = (1.0 / assign11630_e6311);
        var_t1 = assign11630_e6312;
        var_t1_dn0 = (-((-(var_t3_dn0 / (var_t3 * var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        var_t1_dn2 = (-((-(var_t3_dn2 / (var_t3 * var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        var_t1_dn4 = (-((-(var_t3_dn4 / (var_t3 * var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        var_t1_dn5 = (-((-(var_t3_dn5 / (var_t3 * var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        var_t1_dn6 = (-((-(var_t3_dn6 / (var_t3 * var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        var_t1_dn7 = (-((-(var_t3_dn7 / (var_t3 * var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        var_t1_dn8 = (-((-(var_t3_dn8 / (var_t3 * var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        var_t1_dn9 = (-((-(var_t3_dn9 / (var_t3 * var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        var_t1_dn10 = (-((-(var_t3_dn10 / (var_t3 * var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        var_t1_dn11 = (-((-(var_t3_dn11 / (var_t3 * var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        var_t1_dn14 = (-((-(var_t3_dn14 / (var_t3 * var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        var_t1_rv = 0.0;

        let (assign11640_e6318, assign11640_e6318_d_n0, assign11640_e6318_d_n2, assign11640_e6318_d_n4, assign11640_e6318_d_n5, assign11640_e6318_d_n6, assign11640_e6318_d_n7, assign11640_e6318_d_n8, assign11640_e6318_d_n9, assign11640_e6318_d_n10, assign11640_e6318_d_n11, assign11640_e6318_d_n14,) = {
    if (0.0 >= var_t1) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t2 = assign11640_e6318;
        var_t2_dn0 = assign11640_e6318_d_n0;
        var_t2_dn2 = assign11640_e6318_d_n2;
        var_t2_dn4 = assign11640_e6318_d_n4;
        var_t2_dn5 = assign11640_e6318_d_n5;
        var_t2_dn6 = assign11640_e6318_d_n6;
        var_t2_dn7 = assign11640_e6318_d_n7;
        var_t2_dn8 = assign11640_e6318_d_n8;
        var_t2_dn9 = assign11640_e6318_d_n9;
        var_t2_dn10 = assign11640_e6318_d_n10;
        var_t2_dn11 = assign11640_e6318_d_n11;
        var_t2_dn14 = assign11640_e6318_d_n14;
        var_t2_rv = 0.0;

        let assign11650_e6323: f64 = (var_npexte - var_ef_nsubc);
        let assign11650_e6324: f64 = (var_t2 * assign11650_e6323);
        let assign11650_e6326: f64 = (assign11650_e6324 / var_lgate);
        let assign11650_e6327: f64 = (var_nsub + assign11650_e6326);
        var_nsub = assign11650_e6327;
        var_nsub_dn0 = (var_nsub_dn0 + (((var_t2_dn0 * assign11650_e6323) + (var_t2 * (var_npexte_dn0 - var_ef_nsubc_dn0))) / var_lgate));
        var_nsub_dn2 = (var_nsub_dn2 + (((var_t2_dn2 * assign11650_e6323) + (var_t2 * (var_npexte_dn2 - var_ef_nsubc_dn2))) / var_lgate));
        var_nsub_dn4 = (var_nsub_dn4 + (((var_t2_dn4 * assign11650_e6323) + (var_t2 * (var_npexte_dn4 - var_ef_nsubc_dn4))) / var_lgate));
        var_nsub_dn5 = (var_nsub_dn5 + (((var_t2_dn5 * assign11650_e6323) + (var_t2 * (var_npexte_dn5 - var_ef_nsubc_dn5))) / var_lgate));
        var_nsub_dn6 = (var_nsub_dn6 + (((var_t2_dn6 * assign11650_e6323) + (var_t2 * (var_npexte_dn6 - var_ef_nsubc_dn6))) / var_lgate));
        var_nsub_dn7 = (var_nsub_dn7 + (((var_t2_dn7 * assign11650_e6323) + (var_t2 * (var_npexte_dn7 - var_ef_nsubc_dn7))) / var_lgate));
        var_nsub_dn8 = (var_nsub_dn8 + (((var_t2_dn8 * assign11650_e6323) + (var_t2 * (var_npexte_dn8 - var_ef_nsubc_dn8))) / var_lgate));
        var_nsub_dn9 = (var_nsub_dn9 + (((var_t2_dn9 * assign11650_e6323) + (var_t2 * (var_npexte_dn9 - var_ef_nsubc_dn9))) / var_lgate));
        var_nsub_dn10 = (var_nsub_dn10 + (((var_t2_dn10 * assign11650_e6323) + (var_t2 * (var_npexte_dn10 - var_ef_nsubc_dn10))) / var_lgate));
        var_nsub_dn11 = (var_nsub_dn11 + (((var_t2_dn11 * assign11650_e6323) + (var_t2 * (var_npexte_dn11 - var_ef_nsubc_dn11))) / var_lgate));
        var_nsub_dn14 = (var_nsub_dn14 + (((var_t2_dn14 * assign11650_e6323) + (var_t2 * (var_npexte_dn14 - var_ef_nsubc_dn14))) / var_lgate));
        var_nsub_rv = 0.0;

        let assign11660_e6330: f64 = (1.6021918e-19 * var_nsub);
        var_q_nsub = assign11660_e6330;
        var_q_nsub_dn0 = (1.6021918e-19 * var_nsub_dn0);
        var_q_nsub_dn2 = (1.6021918e-19 * var_nsub_dn2);
        var_q_nsub_dn4 = (1.6021918e-19 * var_nsub_dn4);
        var_q_nsub_dn5 = (1.6021918e-19 * var_nsub_dn5);
        var_q_nsub_dn6 = (1.6021918e-19 * var_nsub_dn6);
        var_q_nsub_dn7 = (1.6021918e-19 * var_nsub_dn7);
        var_q_nsub_dn8 = (1.6021918e-19 * var_nsub_dn8);
        var_q_nsub_dn9 = (1.6021918e-19 * var_nsub_dn9);
        var_q_nsub_dn10 = (1.6021918e-19 * var_nsub_dn10);
        var_q_nsub_dn11 = (1.6021918e-19 * var_nsub_dn11);
        var_q_nsub_dn14 = (1.6021918e-19 * var_nsub_dn14);
        var_q_nsub_rv = 0.0;

        let assign11670_e6333: f64 = (var_q_nsub * 1.034943e-10);
        var_qnsub_esi = assign11670_e6333;
        var_qnsub_esi_dn0 = (var_q_nsub_dn0 * 1.034943e-10);
        var_qnsub_esi_dn2 = (var_q_nsub_dn2 * 1.034943e-10);
        var_qnsub_esi_dn4 = (var_q_nsub_dn4 * 1.034943e-10);
        var_qnsub_esi_dn5 = (var_q_nsub_dn5 * 1.034943e-10);
        var_qnsub_esi_dn6 = (var_q_nsub_dn6 * 1.034943e-10);
        var_qnsub_esi_dn7 = (var_q_nsub_dn7 * 1.034943e-10);
        var_qnsub_esi_dn8 = (var_q_nsub_dn8 * 1.034943e-10);
        var_qnsub_esi_dn9 = (var_q_nsub_dn9 * 1.034943e-10);
        var_qnsub_esi_dn10 = (var_q_nsub_dn10 * 1.034943e-10);
        var_qnsub_esi_dn11 = (var_q_nsub_dn11 * 1.034943e-10);
        var_qnsub_esi_dn14 = (var_q_nsub_dn14 * 1.034943e-10);
        var_qnsub_esi_rv = 0.0;

        let assign11680_e6336: f64 = (2.0 * var_qnsub_esi);
        var_qnsub_esi2 = assign11680_e6336;
        var_qnsub_esi2_dn0 = (2.0 * var_qnsub_esi_dn0);
        var_qnsub_esi2_dn2 = (2.0 * var_qnsub_esi_dn2);
        var_qnsub_esi2_dn4 = (2.0 * var_qnsub_esi_dn4);
        var_qnsub_esi2_dn5 = (2.0 * var_qnsub_esi_dn5);
        var_qnsub_esi2_dn6 = (2.0 * var_qnsub_esi_dn6);
        var_qnsub_esi2_dn7 = (2.0 * var_qnsub_esi_dn7);
        var_qnsub_esi2_dn8 = (2.0 * var_qnsub_esi_dn8);
        var_qnsub_esi2_dn9 = (2.0 * var_qnsub_esi_dn9);
        var_qnsub_esi2_dn10 = (2.0 * var_qnsub_esi_dn10);
        var_qnsub_esi2_dn11 = (2.0 * var_qnsub_esi_dn11);
        var_qnsub_esi2_dn14 = (2.0 * var_qnsub_esi_dn14);
        var_qnsub_esi2_rv = 0.0;

        let assign11690_e6340: f64 = (2.0 * p.p140);
        let assign11690_e6345: f64 = if ((var_lgate <= assign11690_e6340) && (p.p140 > 0.0)) { 1.0 } else { 0.0 };
        var_guard270 = assign11690_e6345;
        var_guard270_rv = 0.0;

        let (assign11700_e6361, assign11700_e6361_d_n0, assign11700_e6361_d_n2, assign11700_e6361_d_n4, assign11700_e6361_d_n5, assign11700_e6361_d_n6, assign11700_e6361_d_n7, assign11700_e6361_d_n8, assign11700_e6361_d_n9, assign11700_e6361_d_n10, assign11700_e6361_d_n11, assign11700_e6361_d_n14,) = {
    if (var_guard270 != 0.0) {
        let assign11700_e6349: f64 = (2.0 * var_nsubps);
        let assign11700_e6352: f64 = (var_nsubps - var_ef_nsubc);
        let assign11700_e6354: f64 = (assign11700_e6352 * var_lgate);
        let assign11700_e6356: f64 = (assign11700_e6354 / p.p140);
        let assign11700_e6357: f64 = (assign11700_e6349 - assign11700_e6356);
        let assign11700_e6359: f64 = (assign11700_e6357 - var_ef_nsubc);
        (assign11700_e6359, (((2.0 * var_nsubps_dn0) - (((var_nsubps_dn0 - var_ef_nsubc_dn0) * var_lgate) / p.p140)) - var_ef_nsubc_dn0), (((2.0 * var_nsubps_dn2) - (((var_nsubps_dn2 - var_ef_nsubc_dn2) * var_lgate) / p.p140)) - var_ef_nsubc_dn2), (((2.0 * var_nsubps_dn4) - (((var_nsubps_dn4 - var_ef_nsubc_dn4) * var_lgate) / p.p140)) - var_ef_nsubc_dn4), (((2.0 * var_nsubps_dn5) - (((var_nsubps_dn5 - var_ef_nsubc_dn5) * var_lgate) / p.p140)) - var_ef_nsubc_dn5), (((2.0 * var_nsubps_dn6) - (((var_nsubps_dn6 - var_ef_nsubc_dn6) * var_lgate) / p.p140)) - var_ef_nsubc_dn6), (((2.0 * var_nsubps_dn7) - (((var_nsubps_dn7 - var_ef_nsubc_dn7) * var_lgate) / p.p140)) - var_ef_nsubc_dn7), (((2.0 * var_nsubps_dn8) - (((var_nsubps_dn8 - var_ef_nsubc_dn8) * var_lgate) / p.p140)) - var_ef_nsubc_dn8), (((2.0 * var_nsubps_dn9) - (((var_nsubps_dn9 - var_ef_nsubc_dn9) * var_lgate) / p.p140)) - var_ef_nsubc_dn9), (((2.0 * var_nsubps_dn10) - (((var_nsubps_dn10 - var_ef_nsubc_dn10) * var_lgate) / p.p140)) - var_ef_nsubc_dn10), (((2.0 * var_nsubps_dn11) - (((var_nsubps_dn11 - var_ef_nsubc_dn11) * var_lgate) / p.p140)) - var_ef_nsubc_dn11), (((2.0 * var_nsubps_dn14) - (((var_nsubps_dn14 - var_ef_nsubc_dn14) * var_lgate) / p.p140)) - var_ef_nsubc_dn14),)
    } else {
        (var_nsubb, var_nsubb_dn0, var_nsubb_dn2, var_nsubb_dn4, var_nsubb_dn5, var_nsubb_dn6, var_nsubb_dn7, var_nsubb_dn8, var_nsubb_dn9, var_nsubb_dn10, var_nsubb_dn11, var_nsubb_dn14,)
    }
};
        var_nsubb = assign11700_e6361;
        var_nsubb_dn0 = assign11700_e6361_d_n0;
        var_nsubb_dn2 = assign11700_e6361_d_n2;
        var_nsubb_dn4 = assign11700_e6361_d_n4;
        var_nsubb_dn5 = assign11700_e6361_d_n5;
        var_nsubb_dn6 = assign11700_e6361_d_n6;
        var_nsubb_dn7 = assign11700_e6361_d_n7;
        var_nsubb_dn8 = assign11700_e6361_d_n8;
        var_nsubb_dn9 = assign11700_e6361_d_n9;
        var_nsubb_dn10 = assign11700_e6361_d_n10;
        var_nsubb_dn11 = assign11700_e6361_d_n11;
        var_nsubb_dn14 = assign11700_e6361_d_n14;
        var_nsubb_rv = 0.0;

        *var_ef_nsubc_slot = var_ef_nsubc;
        *var_ef_nsubc_dn0_slot = var_ef_nsubc_dn0;
        *var_ef_nsubc_dn10_slot = var_ef_nsubc_dn10;
        *var_ef_nsubc_dn11_slot = var_ef_nsubc_dn11;
        *var_ef_nsubc_dn14_slot = var_ef_nsubc_dn14;
        *var_ef_nsubc_dn2_slot = var_ef_nsubc_dn2;
        *var_ef_nsubc_dn4_slot = var_ef_nsubc_dn4;
        *var_ef_nsubc_dn5_slot = var_ef_nsubc_dn5;
        *var_ef_nsubc_dn6_slot = var_ef_nsubc_dn6;
        *var_ef_nsubc_dn7_slot = var_ef_nsubc_dn7;
        *var_ef_nsubc_dn8_slot = var_ef_nsubc_dn8;
        *var_ef_nsubc_dn9_slot = var_ef_nsubc_dn9;
        *var_ef_nsubc_rv_slot = var_ef_nsubc_rv;
        *var_guard267_slot = var_guard267;
        *var_guard267_rv_slot = var_guard267_rv;
        *var_guard268_slot = var_guard268;
        *var_guard268_rv_slot = var_guard268_rv;
        *var_guard269_slot = var_guard269;
        *var_guard269_rv_slot = var_guard269_rv;
        *var_guard270_slot = var_guard270;
        *var_guard270_rv_slot = var_guard270_rv;
        *var_nsub_slot = var_nsub;
        *var_nsub_dn0_slot = var_nsub_dn0;
        *var_nsub_dn10_slot = var_nsub_dn10;
        *var_nsub_dn11_slot = var_nsub_dn11;
        *var_nsub_dn14_slot = var_nsub_dn14;
        *var_nsub_dn2_slot = var_nsub_dn2;
        *var_nsub_dn4_slot = var_nsub_dn4;
        *var_nsub_dn5_slot = var_nsub_dn5;
        *var_nsub_dn6_slot = var_nsub_dn6;
        *var_nsub_dn7_slot = var_nsub_dn7;
        *var_nsub_dn8_slot = var_nsub_dn8;
        *var_nsub_dn9_slot = var_nsub_dn9;
        *var_nsub_rv_slot = var_nsub_rv;
        *var_nsubb_slot = var_nsubb;
        *var_nsubb_dn0_slot = var_nsubb_dn0;
        *var_nsubb_dn10_slot = var_nsubb_dn10;
        *var_nsubb_dn11_slot = var_nsubb_dn11;
        *var_nsubb_dn14_slot = var_nsubb_dn14;
        *var_nsubb_dn2_slot = var_nsubb_dn2;
        *var_nsubb_dn4_slot = var_nsubb_dn4;
        *var_nsubb_dn5_slot = var_nsubb_dn5;
        *var_nsubb_dn6_slot = var_nsubb_dn6;
        *var_nsubb_dn7_slot = var_nsubb_dn7;
        *var_nsubb_dn8_slot = var_nsubb_dn8;
        *var_nsubb_dn9_slot = var_nsubb_dn9;
        *var_nsubb_rv_slot = var_nsubb_rv;
        *var_nsubpp_slot = var_nsubpp;
        *var_nsubpp_dn0_slot = var_nsubpp_dn0;
        *var_nsubpp_dn10_slot = var_nsubpp_dn10;
        *var_nsubpp_dn11_slot = var_nsubpp_dn11;
        *var_nsubpp_dn14_slot = var_nsubpp_dn14;
        *var_nsubpp_dn2_slot = var_nsubpp_dn2;
        *var_nsubpp_dn4_slot = var_nsubpp_dn4;
        *var_nsubpp_dn5_slot = var_nsubpp_dn5;
        *var_nsubpp_dn6_slot = var_nsubpp_dn6;
        *var_nsubpp_dn7_slot = var_nsubpp_dn7;
        *var_nsubpp_dn8_slot = var_nsubpp_dn8;
        *var_nsubpp_dn9_slot = var_nsubpp_dn9;
        *var_nsubpp_rv_slot = var_nsubpp_rv;
        *var_nsubps_slot = var_nsubps;
        *var_nsubps_dn0_slot = var_nsubps_dn0;
        *var_nsubps_dn10_slot = var_nsubps_dn10;
        *var_nsubps_dn11_slot = var_nsubps_dn11;
        *var_nsubps_dn14_slot = var_nsubps_dn14;
        *var_nsubps_dn2_slot = var_nsubps_dn2;
        *var_nsubps_dn4_slot = var_nsubps_dn4;
        *var_nsubps_dn5_slot = var_nsubps_dn5;
        *var_nsubps_dn6_slot = var_nsubps_dn6;
        *var_nsubps_dn7_slot = var_nsubps_dn7;
        *var_nsubps_dn8_slot = var_nsubps_dn8;
        *var_nsubps_dn9_slot = var_nsubps_dn9;
        *var_nsubps_rv_slot = var_nsubps_rv;
        *var_q_nsub_slot = var_q_nsub;
        *var_q_nsub_dn0_slot = var_q_nsub_dn0;
        *var_q_nsub_dn10_slot = var_q_nsub_dn10;
        *var_q_nsub_dn11_slot = var_q_nsub_dn11;
        *var_q_nsub_dn14_slot = var_q_nsub_dn14;
        *var_q_nsub_dn2_slot = var_q_nsub_dn2;
        *var_q_nsub_dn4_slot = var_q_nsub_dn4;
        *var_q_nsub_dn5_slot = var_q_nsub_dn5;
        *var_q_nsub_dn6_slot = var_q_nsub_dn6;
        *var_q_nsub_dn7_slot = var_q_nsub_dn7;
        *var_q_nsub_dn8_slot = var_q_nsub_dn8;
        *var_q_nsub_dn9_slot = var_q_nsub_dn9;
        *var_q_nsub_rv_slot = var_q_nsub_rv;
        *var_qnsub_esi_slot = var_qnsub_esi;
        *var_qnsub_esi2_slot = var_qnsub_esi2;
        *var_qnsub_esi2_dn0_slot = var_qnsub_esi2_dn0;
        *var_qnsub_esi2_dn10_slot = var_qnsub_esi2_dn10;
        *var_qnsub_esi2_dn11_slot = var_qnsub_esi2_dn11;
        *var_qnsub_esi2_dn14_slot = var_qnsub_esi2_dn14;
        *var_qnsub_esi2_dn2_slot = var_qnsub_esi2_dn2;
        *var_qnsub_esi2_dn4_slot = var_qnsub_esi2_dn4;
        *var_qnsub_esi2_dn5_slot = var_qnsub_esi2_dn5;
        *var_qnsub_esi2_dn6_slot = var_qnsub_esi2_dn6;
        *var_qnsub_esi2_dn7_slot = var_qnsub_esi2_dn7;
        *var_qnsub_esi2_dn8_slot = var_qnsub_esi2_dn8;
        *var_qnsub_esi2_dn9_slot = var_qnsub_esi2_dn9;
        *var_qnsub_esi2_rv_slot = var_qnsub_esi2_rv;
        *var_qnsub_esi_dn0_slot = var_qnsub_esi_dn0;
        *var_qnsub_esi_dn10_slot = var_qnsub_esi_dn10;
        *var_qnsub_esi_dn11_slot = var_qnsub_esi_dn11;
        *var_qnsub_esi_dn14_slot = var_qnsub_esi_dn14;
        *var_qnsub_esi_dn2_slot = var_qnsub_esi_dn2;
        *var_qnsub_esi_dn4_slot = var_qnsub_esi_dn4;
        *var_qnsub_esi_dn5_slot = var_qnsub_esi_dn5;
        *var_qnsub_esi_dn6_slot = var_qnsub_esi_dn6;
        *var_qnsub_esi_dn7_slot = var_qnsub_esi_dn7;
        *var_qnsub_esi_dn8_slot = var_qnsub_esi_dn8;
        *var_qnsub_esi_dn9_slot = var_qnsub_esi_dn9;
        *var_qnsub_esi_rv_slot = var_qnsub_esi_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn11_slot = var_t0_dn11;
        *var_t0_dn14_slot = var_t0_dn14;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn14_slot = var_t3_dn14;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t3_rv_slot = var_t3_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn11_slot = var_tmf1_dn11;
        *var_tmf1_dn14_slot = var_tmf1_dn14;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn7_slot = var_tmf1_dn7;
        *var_tmf1_dn8_slot = var_tmf1_dn8;
        *var_tmf1_dn9_slot = var_tmf1_dn9;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn11_slot = var_tmf2_dn11;
        *var_tmf2_dn14_slot = var_tmf2_dn14;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn7_slot = var_tmf2_dn7;
        *var_tmf2_dn8_slot = var_tmf2_dn8;
        *var_tmf2_dn9_slot = var_tmf2_dn9;
        *var_tmf2_rv_slot = var_tmf2_rv;
    }

    pub(super) fn stamp_reactive_block_20(
        p: &Parameters,
        var_ef_nsubc: f64,
        var_ef_nsubc_dn0: f64,
        var_ef_nsubc_dn10: f64,
        var_ef_nsubc_dn11: f64,
        var_ef_nsubc_dn14: f64,
        var_ef_nsubc_dn2: f64,
        var_ef_nsubc_dn4: f64,
        var_ef_nsubc_dn5: f64,
        var_ef_nsubc_dn6: f64,
        var_ef_nsubc_dn7: f64,
        var_ef_nsubc_dn8: f64,
        var_ef_nsubc_dn9: f64,
        var_guard270: f64,
        var_lg: f64,
        var_lgate: f64,
        var_nsub: f64,
        var_nsub_dn0: f64,
        var_nsub_dn10: f64,
        var_nsub_dn11: f64,
        var_nsub_dn14: f64,
        var_nsub_dn2: f64,
        var_nsub_dn4: f64,
        var_nsub_dn5: f64,
        var_nsub_dn6: f64,
        var_nsub_dn7: f64,
        var_nsub_dn8: f64,
        var_nsub_dn9: f64,
        var_nsubb: f64,
        var_nsubb_dn0: f64,
        var_nsubb_dn10: f64,
        var_nsubb_dn11: f64,
        var_nsubb_dn14: f64,
        var_nsubb_dn2: f64,
        var_nsubb_dn4: f64,
        var_nsubb_dn5: f64,
        var_nsubb_dn6: f64,
        var_nsubb_dn7: f64,
        var_nsubb_dn8: f64,
        var_nsubb_dn9: f64,
        var_uc_cordrift: f64,
        var_uc_nsti: f64,
        var_uc_rd: f64,
        var_uc_rd23: f64,
        var_uc_rdvd: f64,
        var_uc_rs: f64,
        var_uc_vover: f64,
        var_weff: f64,
        var_wlg: f64,
        var_costi00_slot: &mut f64,
        var_costi00_rv_slot: &mut f64,
        var_guard271_slot: &mut f64,
        var_guard271_rv_slot: &mut f64,
        var_guard273_slot: &mut f64,
        var_guard273_rv_slot: &mut f64,
        var_guard274_slot: &mut f64,
        var_guard274_rv_slot: &mut f64,
        var_guard275_slot: &mut f64,
        var_guard275_rv_slot: &mut f64,
        var_guard276_slot: &mut f64,
        var_guard276_rv_slot: &mut f64,
        var_guard277_slot: &mut f64,
        var_guard277_rv_slot: &mut f64,
        var_guard278_slot: &mut f64,
        var_guard278_rv_slot: &mut f64,
        var_guard279_slot: &mut f64,
        var_guard279_rv_slot: &mut f64,
        var_guard280_slot: &mut f64,
        var_guard280_rv_slot: &mut f64,
        var_nsti_p2_slot: &mut f64,
        var_nsti_p2_rv_slot: &mut f64,
        var_pb20_slot: &mut f64,
        var_pb20_dn0_slot: &mut f64,
        var_pb20_dn10_slot: &mut f64,
        var_pb20_dn11_slot: &mut f64,
        var_pb20_dn14_slot: &mut f64,
        var_pb20_dn2_slot: &mut f64,
        var_pb20_dn4_slot: &mut f64,
        var_pb20_dn5_slot: &mut f64,
        var_pb20_dn6_slot: &mut f64,
        var_pb20_dn7_slot: &mut f64,
        var_pb20_dn8_slot: &mut f64,
        var_pb20_dn9_slot: &mut f64,
        var_pb20_rv_slot: &mut f64,
        var_pb2c_slot: &mut f64,
        var_pb2c_dn0_slot: &mut f64,
        var_pb2c_dn10_slot: &mut f64,
        var_pb2c_dn11_slot: &mut f64,
        var_pb2c_dn14_slot: &mut f64,
        var_pb2c_dn2_slot: &mut f64,
        var_pb2c_dn4_slot: &mut f64,
        var_pb2c_dn5_slot: &mut f64,
        var_pb2c_dn6_slot: &mut f64,
        var_pb2c_dn7_slot: &mut f64,
        var_pb2c_dn8_slot: &mut f64,
        var_pb2c_dn9_slot: &mut f64,
        var_pb2c_rv_slot: &mut f64,
        var_ptovr0_slot: &mut f64,
        var_ptovr0_dn0_slot: &mut f64,
        var_ptovr0_dn10_slot: &mut f64,
        var_ptovr0_dn11_slot: &mut f64,
        var_ptovr0_dn14_slot: &mut f64,
        var_ptovr0_dn2_slot: &mut f64,
        var_ptovr0_dn4_slot: &mut f64,
        var_ptovr0_dn5_slot: &mut f64,
        var_ptovr0_dn6_slot: &mut f64,
        var_ptovr0_dn7_slot: &mut f64,
        var_ptovr0_dn8_slot: &mut f64,
        var_ptovr0_dn9_slot: &mut f64,
        var_ptovr0_rv_slot: &mut f64,
        var_rd0_slot: &mut f64,
        var_rd0_rv_slot: &mut f64,
        var_rdtemp0_slot: &mut f64,
        var_rdtemp0_rv_slot: &mut f64,
        var_rdvdtemp0_slot: &mut f64,
        var_rdvdtemp0_dn0_slot: &mut f64,
        var_rdvdtemp0_dn10_slot: &mut f64,
        var_rdvdtemp0_dn11_slot: &mut f64,
        var_rdvdtemp0_dn14_slot: &mut f64,
        var_rdvdtemp0_dn2_slot: &mut f64,
        var_rdvdtemp0_dn4_slot: &mut f64,
        var_rdvdtemp0_dn5_slot: &mut f64,
        var_rdvdtemp0_dn6_slot: &mut f64,
        var_rdvdtemp0_dn7_slot: &mut f64,
        var_rdvdtemp0_dn8_slot: &mut f64,
        var_rdvdtemp0_dn9_slot: &mut f64,
        var_rdvdtemp0_rv_slot: &mut f64,
        var_rs0_slot: &mut f64,
        var_rs0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn11_slot: &mut f64,
        var_t6_dn14_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t6_dn9_slot: &mut f64,
        var_t6_rv_slot: &mut f64,
        var_t7_slot: &mut f64,
        var_t7_dn0_slot: &mut f64,
        var_t7_dn10_slot: &mut f64,
        var_t7_dn11_slot: &mut f64,
        var_t7_dn14_slot: &mut f64,
        var_t7_dn2_slot: &mut f64,
        var_t7_dn4_slot: &mut f64,
        var_t7_dn5_slot: &mut f64,
        var_t7_dn6_slot: &mut f64,
        var_t7_dn7_slot: &mut f64,
        var_t7_dn8_slot: &mut f64,
        var_t7_dn9_slot: &mut f64,
        var_t7_rv_slot: &mut f64,
        var_vmax0_slot: &mut f64,
        var_vmax0_rv_slot: &mut f64,
    ) {
        let mut var_costi00: f64 = *var_costi00_slot;
        let mut var_costi00_rv: f64 = *var_costi00_rv_slot;
        let mut var_guard271: f64 = *var_guard271_slot;
        let mut var_guard271_rv: f64 = *var_guard271_rv_slot;
        let mut var_guard273: f64 = *var_guard273_slot;
        let mut var_guard273_rv: f64 = *var_guard273_rv_slot;
        let mut var_guard274: f64 = *var_guard274_slot;
        let mut var_guard274_rv: f64 = *var_guard274_rv_slot;
        let mut var_guard275: f64 = *var_guard275_slot;
        let mut var_guard275_rv: f64 = *var_guard275_rv_slot;
        let mut var_guard276: f64 = *var_guard276_slot;
        let mut var_guard276_rv: f64 = *var_guard276_rv_slot;
        let mut var_guard277: f64 = *var_guard277_slot;
        let mut var_guard277_rv: f64 = *var_guard277_rv_slot;
        let mut var_guard278: f64 = *var_guard278_slot;
        let mut var_guard278_rv: f64 = *var_guard278_rv_slot;
        let mut var_guard279: f64 = *var_guard279_slot;
        let mut var_guard279_rv: f64 = *var_guard279_rv_slot;
        let mut var_guard280: f64 = *var_guard280_slot;
        let mut var_guard280_rv: f64 = *var_guard280_rv_slot;
        let mut var_nsti_p2: f64 = *var_nsti_p2_slot;
        let mut var_nsti_p2_rv: f64 = *var_nsti_p2_rv_slot;
        let mut var_pb20: f64 = *var_pb20_slot;
        let mut var_pb20_dn0: f64 = *var_pb20_dn0_slot;
        let mut var_pb20_dn10: f64 = *var_pb20_dn10_slot;
        let mut var_pb20_dn11: f64 = *var_pb20_dn11_slot;
        let mut var_pb20_dn14: f64 = *var_pb20_dn14_slot;
        let mut var_pb20_dn2: f64 = *var_pb20_dn2_slot;
        let mut var_pb20_dn4: f64 = *var_pb20_dn4_slot;
        let mut var_pb20_dn5: f64 = *var_pb20_dn5_slot;
        let mut var_pb20_dn6: f64 = *var_pb20_dn6_slot;
        let mut var_pb20_dn7: f64 = *var_pb20_dn7_slot;
        let mut var_pb20_dn8: f64 = *var_pb20_dn8_slot;
        let mut var_pb20_dn9: f64 = *var_pb20_dn9_slot;
        let mut var_pb20_rv: f64 = *var_pb20_rv_slot;
        let mut var_pb2c: f64 = *var_pb2c_slot;
        let mut var_pb2c_dn0: f64 = *var_pb2c_dn0_slot;
        let mut var_pb2c_dn10: f64 = *var_pb2c_dn10_slot;
        let mut var_pb2c_dn11: f64 = *var_pb2c_dn11_slot;
        let mut var_pb2c_dn14: f64 = *var_pb2c_dn14_slot;
        let mut var_pb2c_dn2: f64 = *var_pb2c_dn2_slot;
        let mut var_pb2c_dn4: f64 = *var_pb2c_dn4_slot;
        let mut var_pb2c_dn5: f64 = *var_pb2c_dn5_slot;
        let mut var_pb2c_dn6: f64 = *var_pb2c_dn6_slot;
        let mut var_pb2c_dn7: f64 = *var_pb2c_dn7_slot;
        let mut var_pb2c_dn8: f64 = *var_pb2c_dn8_slot;
        let mut var_pb2c_dn9: f64 = *var_pb2c_dn9_slot;
        let mut var_pb2c_rv: f64 = *var_pb2c_rv_slot;
        let mut var_ptovr0: f64 = *var_ptovr0_slot;
        let mut var_ptovr0_dn0: f64 = *var_ptovr0_dn0_slot;
        let mut var_ptovr0_dn10: f64 = *var_ptovr0_dn10_slot;
        let mut var_ptovr0_dn11: f64 = *var_ptovr0_dn11_slot;
        let mut var_ptovr0_dn14: f64 = *var_ptovr0_dn14_slot;
        let mut var_ptovr0_dn2: f64 = *var_ptovr0_dn2_slot;
        let mut var_ptovr0_dn4: f64 = *var_ptovr0_dn4_slot;
        let mut var_ptovr0_dn5: f64 = *var_ptovr0_dn5_slot;
        let mut var_ptovr0_dn6: f64 = *var_ptovr0_dn6_slot;
        let mut var_ptovr0_dn7: f64 = *var_ptovr0_dn7_slot;
        let mut var_ptovr0_dn8: f64 = *var_ptovr0_dn8_slot;
        let mut var_ptovr0_dn9: f64 = *var_ptovr0_dn9_slot;
        let mut var_ptovr0_rv: f64 = *var_ptovr0_rv_slot;
        let mut var_rd0: f64 = *var_rd0_slot;
        let mut var_rd0_rv: f64 = *var_rd0_rv_slot;
        let mut var_rdtemp0: f64 = *var_rdtemp0_slot;
        let mut var_rdtemp0_rv: f64 = *var_rdtemp0_rv_slot;
        let mut var_rdvdtemp0: f64 = *var_rdvdtemp0_slot;
        let mut var_rdvdtemp0_dn0: f64 = *var_rdvdtemp0_dn0_slot;
        let mut var_rdvdtemp0_dn10: f64 = *var_rdvdtemp0_dn10_slot;
        let mut var_rdvdtemp0_dn11: f64 = *var_rdvdtemp0_dn11_slot;
        let mut var_rdvdtemp0_dn14: f64 = *var_rdvdtemp0_dn14_slot;
        let mut var_rdvdtemp0_dn2: f64 = *var_rdvdtemp0_dn2_slot;
        let mut var_rdvdtemp0_dn4: f64 = *var_rdvdtemp0_dn4_slot;
        let mut var_rdvdtemp0_dn5: f64 = *var_rdvdtemp0_dn5_slot;
        let mut var_rdvdtemp0_dn6: f64 = *var_rdvdtemp0_dn6_slot;
        let mut var_rdvdtemp0_dn7: f64 = *var_rdvdtemp0_dn7_slot;
        let mut var_rdvdtemp0_dn8: f64 = *var_rdvdtemp0_dn8_slot;
        let mut var_rdvdtemp0_dn9: f64 = *var_rdvdtemp0_dn9_slot;
        let mut var_rdvdtemp0_rv: f64 = *var_rdvdtemp0_rv_slot;
        let mut var_rs0: f64 = *var_rs0_slot;
        let mut var_rs0_rv: f64 = *var_rs0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn11: f64 = *var_t6_dn11_slot;
        let mut var_t6_dn14: f64 = *var_t6_dn14_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t6_dn9: f64 = *var_t6_dn9_slot;
        let mut var_t6_rv: f64 = *var_t6_rv_slot;
        let mut var_t7: f64 = *var_t7_slot;
        let mut var_t7_dn0: f64 = *var_t7_dn0_slot;
        let mut var_t7_dn10: f64 = *var_t7_dn10_slot;
        let mut var_t7_dn11: f64 = *var_t7_dn11_slot;
        let mut var_t7_dn14: f64 = *var_t7_dn14_slot;
        let mut var_t7_dn2: f64 = *var_t7_dn2_slot;
        let mut var_t7_dn4: f64 = *var_t7_dn4_slot;
        let mut var_t7_dn5: f64 = *var_t7_dn5_slot;
        let mut var_t7_dn6: f64 = *var_t7_dn6_slot;
        let mut var_t7_dn7: f64 = *var_t7_dn7_slot;
        let mut var_t7_dn8: f64 = *var_t7_dn8_slot;
        let mut var_t7_dn9: f64 = *var_t7_dn9_slot;
        let mut var_t7_rv: f64 = *var_t7_rv_slot;
        let mut var_vmax0: f64 = *var_vmax0_slot;
        let mut var_vmax0_rv: f64 = *var_vmax0_rv_slot;

        let (assign11710_e6368, assign11710_e6368_d_n0, assign11710_e6368_d_n2, assign11710_e6368_d_n4, assign11710_e6368_d_n5, assign11710_e6368_d_n6, assign11710_e6368_d_n7, assign11710_e6368_d_n8, assign11710_e6368_d_n9, assign11710_e6368_d_n10, assign11710_e6368_d_n11, assign11710_e6368_d_n14,) = {
    if (var_guard270 != 0.0) {
        let assign11710_e6365: f64 = (var_nsubb / var_ef_nsubc);
        let assign11710_e6366: f64 = (assign11710_e6365).ln();
        (assign11710_e6366, ((((var_nsubb_dn0 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn0)) / (var_ef_nsubc * var_ef_nsubc)) / assign11710_e6365), ((((var_nsubb_dn2 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn2)) / (var_ef_nsubc * var_ef_nsubc)) / assign11710_e6365), ((((var_nsubb_dn4 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn4)) / (var_ef_nsubc * var_ef_nsubc)) / assign11710_e6365), ((((var_nsubb_dn5 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn5)) / (var_ef_nsubc * var_ef_nsubc)) / assign11710_e6365), ((((var_nsubb_dn6 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn6)) / (var_ef_nsubc * var_ef_nsubc)) / assign11710_e6365), ((((var_nsubb_dn7 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn7)) / (var_ef_nsubc * var_ef_nsubc)) / assign11710_e6365), ((((var_nsubb_dn8 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn8)) / (var_ef_nsubc * var_ef_nsubc)) / assign11710_e6365), ((((var_nsubb_dn9 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn9)) / (var_ef_nsubc * var_ef_nsubc)) / assign11710_e6365), ((((var_nsubb_dn10 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn10)) / (var_ef_nsubc * var_ef_nsubc)) / assign11710_e6365), ((((var_nsubb_dn11 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn11)) / (var_ef_nsubc * var_ef_nsubc)) / assign11710_e6365), ((((var_nsubb_dn14 * var_ef_nsubc) - (var_nsubb * var_ef_nsubc_dn14)) / (var_ef_nsubc * var_ef_nsubc)) / assign11710_e6365),)
    } else {
        (var_ptovr0, var_ptovr0_dn0, var_ptovr0_dn2, var_ptovr0_dn4, var_ptovr0_dn5, var_ptovr0_dn6, var_ptovr0_dn7, var_ptovr0_dn8, var_ptovr0_dn9, var_ptovr0_dn10, var_ptovr0_dn11, var_ptovr0_dn14,)
    }
};
        var_ptovr0 = assign11710_e6368;
        var_ptovr0_dn0 = assign11710_e6368_d_n0;
        var_ptovr0_dn2 = assign11710_e6368_d_n2;
        var_ptovr0_dn4 = assign11710_e6368_d_n4;
        var_ptovr0_dn5 = assign11710_e6368_d_n5;
        var_ptovr0_dn6 = assign11710_e6368_d_n6;
        var_ptovr0_dn7 = assign11710_e6368_d_n7;
        var_ptovr0_dn8 = assign11710_e6368_d_n8;
        var_ptovr0_dn9 = assign11710_e6368_d_n9;
        var_ptovr0_dn10 = assign11710_e6368_d_n10;
        var_ptovr0_dn11 = assign11710_e6368_d_n11;
        var_ptovr0_dn14 = assign11710_e6368_d_n14;
        var_ptovr0_rv = 0.0;

        let (assign11720_e6373, assign11720_e6373_d_n0, assign11720_e6373_d_n2, assign11720_e6373_d_n4, assign11720_e6373_d_n5, assign11720_e6373_d_n6, assign11720_e6373_d_n7, assign11720_e6373_d_n8, assign11720_e6373_d_n9, assign11720_e6373_d_n10, assign11720_e6373_d_n11, assign11720_e6373_d_n14,) = {
    if (var_guard270 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ptovr0, var_ptovr0_dn0, var_ptovr0_dn2, var_ptovr0_dn4, var_ptovr0_dn5, var_ptovr0_dn6, var_ptovr0_dn7, var_ptovr0_dn8, var_ptovr0_dn9, var_ptovr0_dn10, var_ptovr0_dn11, var_ptovr0_dn14,)
    }
};
        var_ptovr0 = assign11720_e6373;
        var_ptovr0_dn0 = assign11720_e6373_d_n0;
        var_ptovr0_dn2 = assign11720_e6373_d_n2;
        var_ptovr0_dn4 = assign11720_e6373_d_n4;
        var_ptovr0_dn5 = assign11720_e6373_d_n5;
        var_ptovr0_dn6 = assign11720_e6373_d_n6;
        var_ptovr0_dn7 = assign11720_e6373_d_n7;
        var_ptovr0_dn8 = assign11720_e6373_d_n8;
        var_ptovr0_dn9 = assign11720_e6373_d_n9;
        var_ptovr0_dn10 = assign11720_e6373_d_n10;
        var_ptovr0_dn11 = assign11720_e6373_d_n11;
        var_ptovr0_dn14 = assign11720_e6373_d_n14;
        var_ptovr0_rv = 0.0;

        let assign11730_e6376: f64 = (2.0 * 1.6021918e-19);
        let assign11730_e6378: f64 = (assign11730_e6376 * var_uc_nsti);
        let assign11730_e6380: f64 = (assign11730_e6378 * 1.034943e-10);
        let assign11730_e6381: f64 = (assign11730_e6380).sqrt();
        var_costi00 = assign11730_e6381;
        var_costi00_rv = 0.0;

        let assign11740_e6385: f64 = (var_uc_nsti * var_uc_nsti);
        let assign11740_e6386: f64 = (1.0 / assign11740_e6385);
        var_nsti_p2 = assign11740_e6386;
        var_nsti_p2_rv = 0.0;

        let assign11750_e6391: f64 = (var_lg).powf(p.p231);
        let assign11750_e6392: f64 = (var_uc_vover / assign11750_e6391);
        let assign11750_e6393: f64 = (1.0 + assign11750_e6392);
        let assign11750_e6398: f64 = (var_wlg).powf(p.p239);
        let assign11750_e6399: f64 = (p.p238 / assign11750_e6398);
        let assign11750_e6400: f64 = (1.0 + assign11750_e6399);
        let assign11750_e6401: f64 = (assign11750_e6393 * assign11750_e6400);
        var_vmax0 = assign11750_e6401;
        var_vmax0_rv = 0.0;

        let assign11760_e6404: f64 = (2.0 / 38.68283);
        let assign11760_e6407: f64 = (var_nsub / 1.04e16);
        let assign11760_e6408: f64 = (assign11760_e6407).ln();
        let assign11760_e6409: f64 = (assign11760_e6404 * assign11760_e6408);
        var_pb20 = assign11760_e6409;
        var_pb20_dn0 = (assign11760_e6404 * ((var_nsub_dn0 / 1.04e16) / assign11760_e6407));
        var_pb20_dn2 = (assign11760_e6404 * ((var_nsub_dn2 / 1.04e16) / assign11760_e6407));
        var_pb20_dn4 = (assign11760_e6404 * ((var_nsub_dn4 / 1.04e16) / assign11760_e6407));
        var_pb20_dn5 = (assign11760_e6404 * ((var_nsub_dn5 / 1.04e16) / assign11760_e6407));
        var_pb20_dn6 = (assign11760_e6404 * ((var_nsub_dn6 / 1.04e16) / assign11760_e6407));
        var_pb20_dn7 = (assign11760_e6404 * ((var_nsub_dn7 / 1.04e16) / assign11760_e6407));
        var_pb20_dn8 = (assign11760_e6404 * ((var_nsub_dn8 / 1.04e16) / assign11760_e6407));
        var_pb20_dn9 = (assign11760_e6404 * ((var_nsub_dn9 / 1.04e16) / assign11760_e6407));
        var_pb20_dn10 = (assign11760_e6404 * ((var_nsub_dn10 / 1.04e16) / assign11760_e6407));
        var_pb20_dn11 = (assign11760_e6404 * ((var_nsub_dn11 / 1.04e16) / assign11760_e6407));
        var_pb20_dn14 = (assign11760_e6404 * ((var_nsub_dn14 / 1.04e16) / assign11760_e6407));
        var_pb20_rv = 0.0;

        let assign11770_e6412: f64 = (2.0 / 38.68283);
        let assign11770_e6415: f64 = (var_ef_nsubc / 1.04e16);
        let assign11770_e6416: f64 = (assign11770_e6415).ln();
        let assign11770_e6417: f64 = (assign11770_e6412 * assign11770_e6416);
        var_pb2c = assign11770_e6417;
        var_pb2c_dn0 = (assign11770_e6412 * ((var_ef_nsubc_dn0 / 1.04e16) / assign11770_e6415));
        var_pb2c_dn2 = (assign11770_e6412 * ((var_ef_nsubc_dn2 / 1.04e16) / assign11770_e6415));
        var_pb2c_dn4 = (assign11770_e6412 * ((var_ef_nsubc_dn4 / 1.04e16) / assign11770_e6415));
        var_pb2c_dn5 = (assign11770_e6412 * ((var_ef_nsubc_dn5 / 1.04e16) / assign11770_e6415));
        var_pb2c_dn6 = (assign11770_e6412 * ((var_ef_nsubc_dn6 / 1.04e16) / assign11770_e6415));
        var_pb2c_dn7 = (assign11770_e6412 * ((var_ef_nsubc_dn7 / 1.04e16) / assign11770_e6415));
        var_pb2c_dn8 = (assign11770_e6412 * ((var_ef_nsubc_dn8 / 1.04e16) / assign11770_e6415));
        var_pb2c_dn9 = (assign11770_e6412 * ((var_ef_nsubc_dn9 / 1.04e16) / assign11770_e6415));
        var_pb2c_dn10 = (assign11770_e6412 * ((var_ef_nsubc_dn10 / 1.04e16) / assign11770_e6415));
        var_pb2c_dn11 = (assign11770_e6412 * ((var_ef_nsubc_dn11 / 1.04e16) / assign11770_e6415));
        var_pb2c_dn14 = (assign11770_e6412 * ((var_ef_nsubc_dn14 / 1.04e16) / assign11770_e6415));
        var_pb2c_rv = 0.0;

        let assign11780_e6420: f64 = if p.p51 == 1.0 { 1.0 } else { 0.0 };
        var_guard271 = assign11780_e6420;
        var_guard271_rv = 0.0;

        let (assign11790_e6430, assign11790_e6430_d_n0, assign11790_e6430_d_n2, assign11790_e6430_d_n4, assign11790_e6430_d_n5, assign11790_e6430_d_n6, assign11790_e6430_d_n7, assign11790_e6430_d_n8, assign11790_e6430_d_n9, assign11790_e6430_d_n10, assign11790_e6430_d_n11, assign11790_e6430_d_n14,) = {
    if (var_guard271 != 0.0) {
        let assign11790_e6426: f64 = (3.0 * p.p4);
        let assign11790_e6427: f64 = (var_weff / assign11790_e6426);
        let assign11790_e6428: f64 = (p.p5 + assign11790_e6427);
        (assign11790_e6428, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign11790_e6430;
        var_t1_dn0 = assign11790_e6430_d_n0;
        var_t1_dn2 = assign11790_e6430_d_n2;
        var_t1_dn4 = assign11790_e6430_d_n4;
        var_t1_dn5 = assign11790_e6430_d_n5;
        var_t1_dn6 = assign11790_e6430_d_n6;
        var_t1_dn7 = assign11790_e6430_d_n7;
        var_t1_dn8 = assign11790_e6430_d_n8;
        var_t1_dn9 = assign11790_e6430_d_n9;
        var_t1_dn10 = assign11790_e6430_d_n10;
        var_t1_dn11 = assign11790_e6430_d_n11;
        var_t1_dn14 = assign11790_e6430_d_n14;
        var_t1_rv = 0.0;

        let (assign11800_e6436, assign11800_e6436_d_n0, assign11800_e6436_d_n2, assign11800_e6436_d_n4, assign11800_e6436_d_n5, assign11800_e6436_d_n6, assign11800_e6436_d_n7, assign11800_e6436_d_n8, assign11800_e6436_d_n9, assign11800_e6436_d_n10, assign11800_e6436_d_n11, assign11800_e6436_d_n14,) = {
    if (var_guard271 != 0.0) {
        let assign11800_e6434: f64 = (var_lgate - p.p6);
        (assign11800_e6434, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    }
};
        var_t2 = assign11800_e6436;
        var_t2_dn0 = assign11800_e6436_d_n0;
        var_t2_dn2 = assign11800_e6436_d_n2;
        var_t2_dn4 = assign11800_e6436_d_n4;
        var_t2_dn5 = assign11800_e6436_d_n5;
        var_t2_dn6 = assign11800_e6436_d_n6;
        var_t2_dn7 = assign11800_e6436_d_n7;
        var_t2_dn8 = assign11800_e6436_d_n8;
        var_t2_dn9 = assign11800_e6436_d_n9;
        var_t2_dn10 = assign11800_e6436_d_n10;
        var_t2_dn11 = assign11800_e6436_d_n11;
        var_t2_dn14 = assign11800_e6436_d_n14;
        var_t2_rv = 0.0;

        let assign11860_e6478: f64 = if p.p130 > 0.0 { 1.0 } else { 0.0 };
        var_guard273 = assign11860_e6478;
        var_guard273_rv = 0.0;

        let (assign11870_e6484,) = {
    if (var_guard273 != 0.0) {
        let assign11870_e6482: f64 = (p.p130 * p.p2);
        (assign11870_e6482,)
    } else {
        (var_rd0,)
    }
};
        var_rd0 = assign11870_e6484;
        var_rd0_rv = 0.0;

        let (assign11880_e6490,) = {
    if (var_guard273 != 0.0) {
        let assign11880_e6488: f64 = (p.p130 * p.p3);
        (assign11880_e6488,)
    } else {
        (var_rs0,)
    }
};
        var_rs0 = assign11880_e6490;
        var_rs0_rv = 0.0;

        let (assign11890_e6495,) = {
    if (var_guard273 == 0.0) {
        (0.0,)
    } else {
        (var_rd0,)
    }
};
        var_rd0 = assign11890_e6495;
        var_rd0_rv = 0.0;

        let (assign11900_e6500,) = {
    if (var_guard273 == 0.0) {
        (0.0,)
    } else {
        (var_rs0,)
    }
};
        var_rs0 = assign11900_e6500;
        var_rs0_rv = 0.0;

        let assign11910_e6503: f64 = if p.p131 > 0.0 { 1.0 } else { 0.0 };
        var_guard274 = assign11910_e6503;
        var_guard274_rv = 0.0;

        let (assign11920_e6509,) = {
    if (var_guard274 != 0.0) {
        let assign11920_e6507: f64 = (p.p131 * p.p3);
        (assign11920_e6507,)
    } else {
        (var_rs0,)
    }
};
        var_rs0 = assign11920_e6509;
        var_rs0_rv = 0.0;

        let (assign11930_e6514,) = {
    if (var_guard274 == 0.0) {
        (0.0,)
    } else {
        (var_rs0,)
    }
};
        var_rs0 = assign11930_e6514;
        var_rs0_rv = 0.0;

        let assign11940_e6517: f64 = if var_uc_cordrift == 0.0 { 1.0 } else { 0.0 };
        var_guard275 = assign11940_e6517;
        var_guard275_rv = 0.0;

        let assign11950_e6524: f64 = if ((var_uc_rd > 0.0) || (var_uc_rs > 0.0)) { 1.0 } else { 0.0 };
        var_guard276 = assign11950_e6524;
        var_guard276_rv = 0.0;

        let (assign11960_e6536,) = {
    if ((var_guard275 != 0.0) && (var_guard276 != 0.0)) {
        let assign11960_e6532: f64 = (var_wlg).powf(p.p310);
        let assign11960_e6533: f64 = (p.p309 / assign11960_e6532);
        let assign11960_e6534: f64 = (1.0 + assign11960_e6533);
        (assign11960_e6534,)
    } else {
        (var_rdtemp0,)
    }
};
        var_rdtemp0 = assign11960_e6536;
        var_rdtemp0_rv = 0.0;

        let assign11970_e6539: f64 = if var_uc_rdvd != 0.0 { 1.0 } else { 0.0 };
        var_guard277 = assign11970_e6539;
        var_guard277_rv = 0.0;

        let (assign11980_e6553, assign11980_e6553_d_n0, assign11980_e6553_d_n2, assign11980_e6553_d_n4, assign11980_e6553_d_n5, assign11980_e6553_d_n6, assign11980_e6553_d_n7, assign11980_e6553_d_n8, assign11980_e6553_d_n9, assign11980_e6553_d_n10, assign11980_e6553_d_n11, assign11980_e6553_d_n14,) = {
    if (((var_guard275 != 0.0) && (var_guard276 != 0.0)) && (var_guard277 != 0.0)) {
        let assign11980_e6549: f64 = (var_wlg).powf(p.p304);
        let assign11980_e6550: f64 = (p.p303 / assign11980_e6549);
        let assign11980_e6551: f64 = (1.0 + assign11980_e6550);
        (assign11980_e6551, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn7, var_t7_dn8, var_t7_dn9, var_t7_dn10, var_t7_dn11, var_t7_dn14,)
    }
};
        var_t7 = assign11980_e6553;
        var_t7_dn0 = assign11980_e6553_d_n0;
        var_t7_dn2 = assign11980_e6553_d_n2;
        var_t7_dn4 = assign11980_e6553_d_n4;
        var_t7_dn5 = assign11980_e6553_d_n5;
        var_t7_dn6 = assign11980_e6553_d_n6;
        var_t7_dn7 = assign11980_e6553_d_n7;
        var_t7_dn8 = assign11980_e6553_d_n8;
        var_t7_dn9 = assign11980_e6553_d_n9;
        var_t7_dn10 = assign11980_e6553_d_n10;
        var_t7_dn11 = assign11980_e6553_d_n11;
        var_t7_dn14 = assign11980_e6553_d_n14;
        var_t7_rv = 0.0;

        let (assign11990_e6566, assign11990_e6566_d_n0, assign11990_e6566_d_n2, assign11990_e6566_d_n4, assign11990_e6566_d_n5, assign11990_e6566_d_n6, assign11990_e6566_d_n7, assign11990_e6566_d_n8, assign11990_e6566_d_n9, assign11990_e6566_d_n10, assign11990_e6566_d_n11, assign11990_e6566_d_n14,) = {
    if (((var_guard275 != 0.0) && (var_guard276 != 0.0)) && (var_guard277 != 0.0)) {
        let assign11990_e6560: f64 = (-p.p301);
        let assign11990_e6563: f64 = (var_lg).powf(p.p302);
        let assign11990_e6564: f64 = (assign11990_e6560 * assign11990_e6563);
        (assign11990_e6564, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn11, var_t6_dn14,)
    }
};
        var_t6 = assign11990_e6566;
        var_t6_dn0 = assign11990_e6566_d_n0;
        var_t6_dn2 = assign11990_e6566_d_n2;
        var_t6_dn4 = assign11990_e6566_d_n4;
        var_t6_dn5 = assign11990_e6566_d_n5;
        var_t6_dn6 = assign11990_e6566_d_n6;
        var_t6_dn7 = assign11990_e6566_d_n7;
        var_t6_dn8 = assign11990_e6566_d_n8;
        var_t6_dn9 = assign11990_e6566_d_n9;
        var_t6_dn10 = assign11990_e6566_d_n10;
        var_t6_dn11 = assign11990_e6566_d_n11;
        var_t6_dn14 = assign11990_e6566_d_n14;
        var_t6_rv = 0.0;

        let assign12000_e6569: f64 = if var_t6 > 60.0 { 1.0 } else { 0.0 };
        var_guard278 = assign12000_e6569;
        var_guard278_rv = 0.0;

        let (assign12010_e6579, assign12010_e6579_d_n0, assign12010_e6579_d_n2, assign12010_e6579_d_n4, assign12010_e6579_d_n5, assign12010_e6579_d_n6, assign12010_e6579_d_n7, assign12010_e6579_d_n8, assign12010_e6579_d_n9, assign12010_e6579_d_n10, assign12010_e6579_d_n11, assign12010_e6579_d_n14,) = {
    if ((((var_guard275 != 0.0) && (var_guard276 != 0.0)) && (var_guard277 != 0.0)) && (var_guard278 != 0.0)) {
        (60.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn11, var_t6_dn14,)
    }
};
        var_t6 = assign12010_e6579;
        var_t6_dn0 = assign12010_e6579_d_n0;
        var_t6_dn2 = assign12010_e6579_d_n2;
        var_t6_dn4 = assign12010_e6579_d_n4;
        var_t6_dn5 = assign12010_e6579_d_n5;
        var_t6_dn6 = assign12010_e6579_d_n6;
        var_t6_dn7 = assign12010_e6579_d_n7;
        var_t6_dn8 = assign12010_e6579_d_n8;
        var_t6_dn9 = assign12010_e6579_d_n9;
        var_t6_dn10 = assign12010_e6579_d_n10;
        var_t6_dn11 = assign12010_e6579_d_n11;
        var_t6_dn14 = assign12010_e6579_d_n14;
        var_t6_rv = 0.0;

        let (assign12020_e6588, assign12020_e6588_d_n0, assign12020_e6588_d_n2, assign12020_e6588_d_n4, assign12020_e6588_d_n5, assign12020_e6588_d_n6, assign12020_e6588_d_n7, assign12020_e6588_d_n8, assign12020_e6588_d_n9, assign12020_e6588_d_n10, assign12020_e6588_d_n11, assign12020_e6588_d_n14,) = {
    if (((var_guard275 != 0.0) && (var_guard276 != 0.0)) && (var_guard277 != 0.0)) {
        let assign12020_e6586: f64 = (var_t6).exp();
        (assign12020_e6586, (assign12020_e6586 * var_t6_dn0), (assign12020_e6586 * var_t6_dn2), (assign12020_e6586 * var_t6_dn4), (assign12020_e6586 * var_t6_dn5), (assign12020_e6586 * var_t6_dn6), (assign12020_e6586 * var_t6_dn7), (assign12020_e6586 * var_t6_dn8), (assign12020_e6586 * var_t6_dn9), (assign12020_e6586 * var_t6_dn10), (assign12020_e6586 * var_t6_dn11), (assign12020_e6586 * var_t6_dn14),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn11, var_t6_dn14,)
    }
};
        var_t6 = assign12020_e6588;
        var_t6_dn0 = assign12020_e6588_d_n0;
        var_t6_dn2 = assign12020_e6588_d_n2;
        var_t6_dn4 = assign12020_e6588_d_n4;
        var_t6_dn5 = assign12020_e6588_d_n5;
        var_t6_dn6 = assign12020_e6588_d_n6;
        var_t6_dn7 = assign12020_e6588_d_n7;
        var_t6_dn8 = assign12020_e6588_d_n8;
        var_t6_dn9 = assign12020_e6588_d_n9;
        var_t6_dn10 = assign12020_e6588_d_n10;
        var_t6_dn11 = assign12020_e6588_d_n11;
        var_t6_dn14 = assign12020_e6588_d_n14;
        var_t6_rv = 0.0;

        let (assign12030_e6598, assign12030_e6598_d_n0, assign12030_e6598_d_n2, assign12030_e6598_d_n4, assign12030_e6598_d_n5, assign12030_e6598_d_n6, assign12030_e6598_d_n7, assign12030_e6598_d_n8, assign12030_e6598_d_n9, assign12030_e6598_d_n10, assign12030_e6598_d_n11, assign12030_e6598_d_n14,) = {
    if (((var_guard275 != 0.0) && (var_guard276 != 0.0)) && (var_guard277 != 0.0)) {
        let assign12030_e6596: f64 = (var_t6 * var_t7);
        (assign12030_e6596, ((var_t6_dn0 * var_t7) + (var_t6 * var_t7_dn0)), ((var_t6_dn2 * var_t7) + (var_t6 * var_t7_dn2)), ((var_t6_dn4 * var_t7) + (var_t6 * var_t7_dn4)), ((var_t6_dn5 * var_t7) + (var_t6 * var_t7_dn5)), ((var_t6_dn6 * var_t7) + (var_t6 * var_t7_dn6)), ((var_t6_dn7 * var_t7) + (var_t6 * var_t7_dn7)), ((var_t6_dn8 * var_t7) + (var_t6 * var_t7_dn8)), ((var_t6_dn9 * var_t7) + (var_t6 * var_t7_dn9)), ((var_t6_dn10 * var_t7) + (var_t6 * var_t7_dn10)), ((var_t6_dn11 * var_t7) + (var_t6 * var_t7_dn11)), ((var_t6_dn14 * var_t7) + (var_t6 * var_t7_dn14)),)
    } else {
        (var_rdvdtemp0, var_rdvdtemp0_dn0, var_rdvdtemp0_dn2, var_rdvdtemp0_dn4, var_rdvdtemp0_dn5, var_rdvdtemp0_dn6, var_rdvdtemp0_dn7, var_rdvdtemp0_dn8, var_rdvdtemp0_dn9, var_rdvdtemp0_dn10, var_rdvdtemp0_dn11, var_rdvdtemp0_dn14,)
    }
};
        var_rdvdtemp0 = assign12030_e6598;
        var_rdvdtemp0_dn0 = assign12030_e6598_d_n0;
        var_rdvdtemp0_dn2 = assign12030_e6598_d_n2;
        var_rdvdtemp0_dn4 = assign12030_e6598_d_n4;
        var_rdvdtemp0_dn5 = assign12030_e6598_d_n5;
        var_rdvdtemp0_dn6 = assign12030_e6598_d_n6;
        var_rdvdtemp0_dn7 = assign12030_e6598_d_n7;
        var_rdvdtemp0_dn8 = assign12030_e6598_d_n8;
        var_rdvdtemp0_dn9 = assign12030_e6598_d_n9;
        var_rdvdtemp0_dn10 = assign12030_e6598_d_n10;
        var_rdvdtemp0_dn11 = assign12030_e6598_d_n11;
        var_rdvdtemp0_dn14 = assign12030_e6598_d_n14;
        var_rdvdtemp0_rv = 0.0;

        let (assign12040_e6607, assign12040_e6607_d_n0, assign12040_e6607_d_n2, assign12040_e6607_d_n4, assign12040_e6607_d_n5, assign12040_e6607_d_n6, assign12040_e6607_d_n7, assign12040_e6607_d_n8, assign12040_e6607_d_n9, assign12040_e6607_d_n10, assign12040_e6607_d_n11, assign12040_e6607_d_n14,) = {
    if (((var_guard275 != 0.0) && (var_guard276 != 0.0)) && (var_guard277 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rdvdtemp0, var_rdvdtemp0_dn0, var_rdvdtemp0_dn2, var_rdvdtemp0_dn4, var_rdvdtemp0_dn5, var_rdvdtemp0_dn6, var_rdvdtemp0_dn7, var_rdvdtemp0_dn8, var_rdvdtemp0_dn9, var_rdvdtemp0_dn10, var_rdvdtemp0_dn11, var_rdvdtemp0_dn14,)
    }
};
        var_rdvdtemp0 = assign12040_e6607;
        var_rdvdtemp0_dn0 = assign12040_e6607_d_n0;
        var_rdvdtemp0_dn2 = assign12040_e6607_d_n2;
        var_rdvdtemp0_dn4 = assign12040_e6607_d_n4;
        var_rdvdtemp0_dn5 = assign12040_e6607_d_n5;
        var_rdvdtemp0_dn6 = assign12040_e6607_d_n6;
        var_rdvdtemp0_dn7 = assign12040_e6607_d_n7;
        var_rdvdtemp0_dn8 = assign12040_e6607_d_n8;
        var_rdvdtemp0_dn9 = assign12040_e6607_d_n9;
        var_rdvdtemp0_dn10 = assign12040_e6607_d_n10;
        var_rdvdtemp0_dn11 = assign12040_e6607_d_n11;
        var_rdvdtemp0_dn14 = assign12040_e6607_d_n14;
        var_rdvdtemp0_rv = 0.0;

        let (assign12050_e6614,) = {
    if ((var_guard275 != 0.0) && (var_guard276 == 0.0)) {
        (0.0,)
    } else {
        (var_rdtemp0,)
    }
};
        var_rdtemp0 = assign12050_e6614;
        var_rdtemp0_rv = 0.0;

        let (assign12060_e6621, assign12060_e6621_d_n0, assign12060_e6621_d_n2, assign12060_e6621_d_n4, assign12060_e6621_d_n5, assign12060_e6621_d_n6, assign12060_e6621_d_n7, assign12060_e6621_d_n8, assign12060_e6621_d_n9, assign12060_e6621_d_n10, assign12060_e6621_d_n11, assign12060_e6621_d_n14,) = {
    if ((var_guard275 != 0.0) && (var_guard276 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rdvdtemp0, var_rdvdtemp0_dn0, var_rdvdtemp0_dn2, var_rdvdtemp0_dn4, var_rdvdtemp0_dn5, var_rdvdtemp0_dn6, var_rdvdtemp0_dn7, var_rdvdtemp0_dn8, var_rdvdtemp0_dn9, var_rdvdtemp0_dn10, var_rdvdtemp0_dn11, var_rdvdtemp0_dn14,)
    }
};
        var_rdvdtemp0 = assign12060_e6621;
        var_rdvdtemp0_dn0 = assign12060_e6621_d_n0;
        var_rdvdtemp0_dn2 = assign12060_e6621_d_n2;
        var_rdvdtemp0_dn4 = assign12060_e6621_d_n4;
        var_rdvdtemp0_dn5 = assign12060_e6621_d_n5;
        var_rdvdtemp0_dn6 = assign12060_e6621_d_n6;
        var_rdvdtemp0_dn7 = assign12060_e6621_d_n7;
        var_rdvdtemp0_dn8 = assign12060_e6621_d_n8;
        var_rdvdtemp0_dn9 = assign12060_e6621_d_n9;
        var_rdvdtemp0_dn10 = assign12060_e6621_d_n10;
        var_rdvdtemp0_dn11 = assign12060_e6621_d_n11;
        var_rdvdtemp0_dn14 = assign12060_e6621_d_n14;
        var_rdvdtemp0_rv = 0.0;

        let assign12070_e6624: f64 = if var_uc_rd23 != 0.0 { 1.0 } else { 0.0 };
        var_guard279 = assign12070_e6624;
        var_guard279_rv = 0.0;

        let (assign12080_e6636, assign12080_e6636_d_n0, assign12080_e6636_d_n2, assign12080_e6636_d_n4, assign12080_e6636_d_n5, assign12080_e6636_d_n6, assign12080_e6636_d_n7, assign12080_e6636_d_n8, assign12080_e6636_d_n9, assign12080_e6636_d_n10, assign12080_e6636_d_n11, assign12080_e6636_d_n14,) = {
    if ((var_guard275 != 0.0) && (var_guard279 != 0.0)) {
        let assign12080_e6632: f64 = (var_wlg).powf(p.p308);
        let assign12080_e6633: f64 = (p.p307 / assign12080_e6632);
        let assign12080_e6634: f64 = (1.0 + assign12080_e6633);
        (assign12080_e6634, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    }
};
        var_t2 = assign12080_e6636;
        var_t2_dn0 = assign12080_e6636_d_n0;
        var_t2_dn2 = assign12080_e6636_d_n2;
        var_t2_dn4 = assign12080_e6636_d_n4;
        var_t2_dn5 = assign12080_e6636_d_n5;
        var_t2_dn6 = assign12080_e6636_d_n6;
        var_t2_dn7 = assign12080_e6636_d_n7;
        var_t2_dn8 = assign12080_e6636_d_n8;
        var_t2_dn9 = assign12080_e6636_d_n9;
        var_t2_dn10 = assign12080_e6636_d_n10;
        var_t2_dn11 = assign12080_e6636_d_n11;
        var_t2_dn14 = assign12080_e6636_d_n14;
        var_t2_rv = 0.0;

        let (assign12090_e6647, assign12090_e6647_d_n0, assign12090_e6647_d_n2, assign12090_e6647_d_n4, assign12090_e6647_d_n5, assign12090_e6647_d_n6, assign12090_e6647_d_n7, assign12090_e6647_d_n8, assign12090_e6647_d_n9, assign12090_e6647_d_n10, assign12090_e6647_d_n11, assign12090_e6647_d_n14,) = {
    if ((var_guard275 != 0.0) && (var_guard279 != 0.0)) {
        let assign12090_e6641: f64 = (-p.p305);
        let assign12090_e6644: f64 = (var_lg).powf(p.p306);
        let assign12090_e6645: f64 = (assign12090_e6641 * assign12090_e6644);
        (assign12090_e6645, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign12090_e6647;
        var_t1_dn0 = assign12090_e6647_d_n0;
        var_t1_dn2 = assign12090_e6647_d_n2;
        var_t1_dn4 = assign12090_e6647_d_n4;
        var_t1_dn5 = assign12090_e6647_d_n5;
        var_t1_dn6 = assign12090_e6647_d_n6;
        var_t1_dn7 = assign12090_e6647_d_n7;
        var_t1_dn8 = assign12090_e6647_d_n8;
        var_t1_dn9 = assign12090_e6647_d_n9;
        var_t1_dn10 = assign12090_e6647_d_n10;
        var_t1_dn11 = assign12090_e6647_d_n11;
        var_t1_dn14 = assign12090_e6647_d_n14;
        var_t1_rv = 0.0;

        let assign12100_e6650: f64 = if var_t1 > 60.0 { 1.0 } else { 0.0 };
        var_guard280 = assign12100_e6650;
        var_guard280_rv = 0.0;

        let (assign12110_e6658, assign12110_e6658_d_n0, assign12110_e6658_d_n2, assign12110_e6658_d_n4, assign12110_e6658_d_n5, assign12110_e6658_d_n6, assign12110_e6658_d_n7, assign12110_e6658_d_n8, assign12110_e6658_d_n9, assign12110_e6658_d_n10, assign12110_e6658_d_n11, assign12110_e6658_d_n14,) = {
    if (((var_guard275 != 0.0) && (var_guard279 != 0.0)) && (var_guard280 != 0.0)) {
        (60.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign12110_e6658;
        var_t1_dn0 = assign12110_e6658_d_n0;
        var_t1_dn2 = assign12110_e6658_d_n2;
        var_t1_dn4 = assign12110_e6658_d_n4;
        var_t1_dn5 = assign12110_e6658_d_n5;
        var_t1_dn6 = assign12110_e6658_d_n6;
        var_t1_dn7 = assign12110_e6658_d_n7;
        var_t1_dn8 = assign12110_e6658_d_n8;
        var_t1_dn9 = assign12110_e6658_d_n9;
        var_t1_dn10 = assign12110_e6658_d_n10;
        var_t1_dn11 = assign12110_e6658_d_n11;
        var_t1_dn14 = assign12110_e6658_d_n14;
        var_t1_rv = 0.0;

        *var_costi00_slot = var_costi00;
        *var_costi00_rv_slot = var_costi00_rv;
        *var_guard271_slot = var_guard271;
        *var_guard271_rv_slot = var_guard271_rv;
        *var_guard273_slot = var_guard273;
        *var_guard273_rv_slot = var_guard273_rv;
        *var_guard274_slot = var_guard274;
        *var_guard274_rv_slot = var_guard274_rv;
        *var_guard275_slot = var_guard275;
        *var_guard275_rv_slot = var_guard275_rv;
        *var_guard276_slot = var_guard276;
        *var_guard276_rv_slot = var_guard276_rv;
        *var_guard277_slot = var_guard277;
        *var_guard277_rv_slot = var_guard277_rv;
        *var_guard278_slot = var_guard278;
        *var_guard278_rv_slot = var_guard278_rv;
        *var_guard279_slot = var_guard279;
        *var_guard279_rv_slot = var_guard279_rv;
        *var_guard280_slot = var_guard280;
        *var_guard280_rv_slot = var_guard280_rv;
        *var_nsti_p2_slot = var_nsti_p2;
        *var_nsti_p2_rv_slot = var_nsti_p2_rv;
        *var_pb20_slot = var_pb20;
        *var_pb20_dn0_slot = var_pb20_dn0;
        *var_pb20_dn10_slot = var_pb20_dn10;
        *var_pb20_dn11_slot = var_pb20_dn11;
        *var_pb20_dn14_slot = var_pb20_dn14;
        *var_pb20_dn2_slot = var_pb20_dn2;
        *var_pb20_dn4_slot = var_pb20_dn4;
        *var_pb20_dn5_slot = var_pb20_dn5;
        *var_pb20_dn6_slot = var_pb20_dn6;
        *var_pb20_dn7_slot = var_pb20_dn7;
        *var_pb20_dn8_slot = var_pb20_dn8;
        *var_pb20_dn9_slot = var_pb20_dn9;
        *var_pb20_rv_slot = var_pb20_rv;
        *var_pb2c_slot = var_pb2c;
        *var_pb2c_dn0_slot = var_pb2c_dn0;
        *var_pb2c_dn10_slot = var_pb2c_dn10;
        *var_pb2c_dn11_slot = var_pb2c_dn11;
        *var_pb2c_dn14_slot = var_pb2c_dn14;
        *var_pb2c_dn2_slot = var_pb2c_dn2;
        *var_pb2c_dn4_slot = var_pb2c_dn4;
        *var_pb2c_dn5_slot = var_pb2c_dn5;
        *var_pb2c_dn6_slot = var_pb2c_dn6;
        *var_pb2c_dn7_slot = var_pb2c_dn7;
        *var_pb2c_dn8_slot = var_pb2c_dn8;
        *var_pb2c_dn9_slot = var_pb2c_dn9;
        *var_pb2c_rv_slot = var_pb2c_rv;
        *var_ptovr0_slot = var_ptovr0;
        *var_ptovr0_dn0_slot = var_ptovr0_dn0;
        *var_ptovr0_dn10_slot = var_ptovr0_dn10;
        *var_ptovr0_dn11_slot = var_ptovr0_dn11;
        *var_ptovr0_dn14_slot = var_ptovr0_dn14;
        *var_ptovr0_dn2_slot = var_ptovr0_dn2;
        *var_ptovr0_dn4_slot = var_ptovr0_dn4;
        *var_ptovr0_dn5_slot = var_ptovr0_dn5;
        *var_ptovr0_dn6_slot = var_ptovr0_dn6;
        *var_ptovr0_dn7_slot = var_ptovr0_dn7;
        *var_ptovr0_dn8_slot = var_ptovr0_dn8;
        *var_ptovr0_dn9_slot = var_ptovr0_dn9;
        *var_ptovr0_rv_slot = var_ptovr0_rv;
        *var_rd0_slot = var_rd0;
        *var_rd0_rv_slot = var_rd0_rv;
        *var_rdtemp0_slot = var_rdtemp0;
        *var_rdtemp0_rv_slot = var_rdtemp0_rv;
        *var_rdvdtemp0_slot = var_rdvdtemp0;
        *var_rdvdtemp0_dn0_slot = var_rdvdtemp0_dn0;
        *var_rdvdtemp0_dn10_slot = var_rdvdtemp0_dn10;
        *var_rdvdtemp0_dn11_slot = var_rdvdtemp0_dn11;
        *var_rdvdtemp0_dn14_slot = var_rdvdtemp0_dn14;
        *var_rdvdtemp0_dn2_slot = var_rdvdtemp0_dn2;
        *var_rdvdtemp0_dn4_slot = var_rdvdtemp0_dn4;
        *var_rdvdtemp0_dn5_slot = var_rdvdtemp0_dn5;
        *var_rdvdtemp0_dn6_slot = var_rdvdtemp0_dn6;
        *var_rdvdtemp0_dn7_slot = var_rdvdtemp0_dn7;
        *var_rdvdtemp0_dn8_slot = var_rdvdtemp0_dn8;
        *var_rdvdtemp0_dn9_slot = var_rdvdtemp0_dn9;
        *var_rdvdtemp0_rv_slot = var_rdvdtemp0_rv;
        *var_rs0_slot = var_rs0;
        *var_rs0_rv_slot = var_rs0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t2_rv_slot = var_t2_rv;
        *var_t6_slot = var_t6;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn11_slot = var_t6_dn11;
        *var_t6_dn14_slot = var_t6_dn14;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t6_dn9_slot = var_t6_dn9;
        *var_t6_rv_slot = var_t6_rv;
        *var_t7_slot = var_t7;
        *var_t7_dn0_slot = var_t7_dn0;
        *var_t7_dn10_slot = var_t7_dn10;
        *var_t7_dn11_slot = var_t7_dn11;
        *var_t7_dn14_slot = var_t7_dn14;
        *var_t7_dn2_slot = var_t7_dn2;
        *var_t7_dn4_slot = var_t7_dn4;
        *var_t7_dn5_slot = var_t7_dn5;
        *var_t7_dn6_slot = var_t7_dn6;
        *var_t7_dn7_slot = var_t7_dn7;
        *var_t7_dn8_slot = var_t7_dn8;
        *var_t7_dn9_slot = var_t7_dn9;
        *var_t7_rv_slot = var_t7_rv;
        *var_vmax0_slot = var_vmax0;
        *var_vmax0_rv_slot = var_vmax0_rv;
    }

    pub(super) fn stamp_reactive_block_21(
        p: &Parameters,
        var_ef_nsubc: f64,
        var_ef_nsubc_dn0: f64,
        var_ef_nsubc_dn10: f64,
        var_ef_nsubc_dn11: f64,
        var_ef_nsubc_dn14: f64,
        var_ef_nsubc_dn2: f64,
        var_ef_nsubc_dn4: f64,
        var_ef_nsubc_dn5: f64,
        var_ef_nsubc_dn6: f64,
        var_ef_nsubc_dn7: f64,
        var_ef_nsubc_dn8: f64,
        var_ef_nsubc_dn9: f64,
        var_guard275: f64,
        var_guard279: f64,
        var_lg: f64,
        var_t2: f64,
        var_t2_dn0: f64,
        var_t2_dn10: f64,
        var_t2_dn11: f64,
        var_t2_dn14: f64,
        var_t2_dn2: f64,
        var_t2_dn4: f64,
        var_t2_dn5: f64,
        var_t2_dn6: f64,
        var_t2_dn7: f64,
        var_t2_dn8: f64,
        var_t2_dn9: f64,
        var_uc_nover: f64,
        var_uc_rd23: f64,
        var_uc_xldld: f64,
        var_wg: f64,
        var_ddlte_slot: &mut f64,
        var_ddlte_dn0_slot: &mut f64,
        var_ddlte_dn10_slot: &mut f64,
        var_ddlte_dn11_slot: &mut f64,
        var_ddlte_dn14_slot: &mut f64,
        var_ddlte_dn2_slot: &mut f64,
        var_ddlte_dn4_slot: &mut f64,
        var_ddlte_dn5_slot: &mut f64,
        var_ddlte_dn6_slot: &mut f64,
        var_ddlte_dn7_slot: &mut f64,
        var_ddlte_dn8_slot: &mut f64,
        var_ddlte_dn9_slot: &mut f64,
        var_ddlte_rv_slot: &mut f64,
        var_guard281_slot: &mut f64,
        var_guard281_rv_slot: &mut f64,
        var_guard286_slot: &mut f64,
        var_guard286_rv_slot: &mut f64,
        var_guard287_slot: &mut f64,
        var_guard287_rv_slot: &mut f64,
        var_guard288_slot: &mut f64,
        var_guard288_rv_slot: &mut f64,
        var_kdep_slot: &mut f64,
        var_kdep_rv_slot: &mut f64,
        var_kjunc_slot: &mut f64,
        var_kjunc_dn0_slot: &mut f64,
        var_kjunc_dn10_slot: &mut f64,
        var_kjunc_dn11_slot: &mut f64,
        var_kjunc_dn14_slot: &mut f64,
        var_kjunc_dn2_slot: &mut f64,
        var_kjunc_dn4_slot: &mut f64,
        var_kjunc_dn5_slot: &mut f64,
        var_kjunc_dn6_slot: &mut f64,
        var_kjunc_dn7_slot: &mut f64,
        var_kjunc_dn8_slot: &mut f64,
        var_kjunc_dn9_slot: &mut f64,
        var_kjunc_rv_slot: &mut f64,
        var_rd23e_slot: &mut f64,
        var_rd23e_dn0_slot: &mut f64,
        var_rd23e_dn10_slot: &mut f64,
        var_rd23e_dn11_slot: &mut f64,
        var_rd23e_dn14_slot: &mut f64,
        var_rd23e_dn2_slot: &mut f64,
        var_rd23e_dn4_slot: &mut f64,
        var_rd23e_dn5_slot: &mut f64,
        var_rd23e_dn6_slot: &mut f64,
        var_rd23e_dn7_slot: &mut f64,
        var_rd23e_dn8_slot: &mut f64,
        var_rd23e_dn9_slot: &mut f64,
        var_rd23e_rv_slot: &mut f64,
        var_rdrmuele_slot: &mut f64,
        var_rdrmuele_rv_slot: &mut f64,
        var_rdrmuevbs_slot: &mut f64,
        var_rdrmuevbs_dn0_slot: &mut f64,
        var_rdrmuevbs_dn10_slot: &mut f64,
        var_rdrmuevbs_dn11_slot: &mut f64,
        var_rdrmuevbs_dn14_slot: &mut f64,
        var_rdrmuevbs_dn2_slot: &mut f64,
        var_rdrmuevbs_dn4_slot: &mut f64,
        var_rdrmuevbs_dn5_slot: &mut f64,
        var_rdrmuevbs_dn6_slot: &mut f64,
        var_rdrmuevbs_dn7_slot: &mut f64,
        var_rdrmuevbs_dn8_slot: &mut f64,
        var_rdrmuevbs_dn9_slot: &mut f64,
        var_rdrmuevbs_rv_slot: &mut f64,
        var_rdrvmaxle_slot: &mut f64,
        var_rdrvmaxle_rv_slot: &mut f64,
        var_rdrvmaxwe_slot: &mut f64,
        var_rdrvmaxwe_rv_slot: &mut f64,
        var_rdtemp0_slot: &mut f64,
        var_rdtemp0_rv_slot: &mut f64,
        var_rdvdtemp0_slot: &mut f64,
        var_rdvdtemp0_dn0_slot: &mut f64,
        var_rdvdtemp0_dn10_slot: &mut f64,
        var_rdvdtemp0_dn11_slot: &mut f64,
        var_rdvdtemp0_dn14_slot: &mut f64,
        var_rdvdtemp0_dn2_slot: &mut f64,
        var_rdvdtemp0_dn4_slot: &mut f64,
        var_rdvdtemp0_dn5_slot: &mut f64,
        var_rdvdtemp0_dn6_slot: &mut f64,
        var_rdvdtemp0_dn7_slot: &mut f64,
        var_rdvdtemp0_dn8_slot: &mut f64,
        var_rdvdtemp0_dn9_slot: &mut f64,
        var_rdvdtemp0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn14_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_xmax_slot: &mut f64,
        var_xmax_rv_slot: &mut f64,
        var_xmax_s_slot: &mut f64,
        var_xmax_s_rv_slot: &mut f64,
    ) {
        let mut var_ddlte: f64 = *var_ddlte_slot;
        let mut var_ddlte_dn0: f64 = *var_ddlte_dn0_slot;
        let mut var_ddlte_dn10: f64 = *var_ddlte_dn10_slot;
        let mut var_ddlte_dn11: f64 = *var_ddlte_dn11_slot;
        let mut var_ddlte_dn14: f64 = *var_ddlte_dn14_slot;
        let mut var_ddlte_dn2: f64 = *var_ddlte_dn2_slot;
        let mut var_ddlte_dn4: f64 = *var_ddlte_dn4_slot;
        let mut var_ddlte_dn5: f64 = *var_ddlte_dn5_slot;
        let mut var_ddlte_dn6: f64 = *var_ddlte_dn6_slot;
        let mut var_ddlte_dn7: f64 = *var_ddlte_dn7_slot;
        let mut var_ddlte_dn8: f64 = *var_ddlte_dn8_slot;
        let mut var_ddlte_dn9: f64 = *var_ddlte_dn9_slot;
        let mut var_ddlte_rv: f64 = *var_ddlte_rv_slot;
        let mut var_guard281: f64 = *var_guard281_slot;
        let mut var_guard281_rv: f64 = *var_guard281_rv_slot;
        let mut var_guard286: f64 = *var_guard286_slot;
        let mut var_guard286_rv: f64 = *var_guard286_rv_slot;
        let mut var_guard287: f64 = *var_guard287_slot;
        let mut var_guard287_rv: f64 = *var_guard287_rv_slot;
        let mut var_guard288: f64 = *var_guard288_slot;
        let mut var_guard288_rv: f64 = *var_guard288_rv_slot;
        let mut var_kdep: f64 = *var_kdep_slot;
        let mut var_kdep_rv: f64 = *var_kdep_rv_slot;
        let mut var_kjunc: f64 = *var_kjunc_slot;
        let mut var_kjunc_dn0: f64 = *var_kjunc_dn0_slot;
        let mut var_kjunc_dn10: f64 = *var_kjunc_dn10_slot;
        let mut var_kjunc_dn11: f64 = *var_kjunc_dn11_slot;
        let mut var_kjunc_dn14: f64 = *var_kjunc_dn14_slot;
        let mut var_kjunc_dn2: f64 = *var_kjunc_dn2_slot;
        let mut var_kjunc_dn4: f64 = *var_kjunc_dn4_slot;
        let mut var_kjunc_dn5: f64 = *var_kjunc_dn5_slot;
        let mut var_kjunc_dn6: f64 = *var_kjunc_dn6_slot;
        let mut var_kjunc_dn7: f64 = *var_kjunc_dn7_slot;
        let mut var_kjunc_dn8: f64 = *var_kjunc_dn8_slot;
        let mut var_kjunc_dn9: f64 = *var_kjunc_dn9_slot;
        let mut var_kjunc_rv: f64 = *var_kjunc_rv_slot;
        let mut var_rd23e: f64 = *var_rd23e_slot;
        let mut var_rd23e_dn0: f64 = *var_rd23e_dn0_slot;
        let mut var_rd23e_dn10: f64 = *var_rd23e_dn10_slot;
        let mut var_rd23e_dn11: f64 = *var_rd23e_dn11_slot;
        let mut var_rd23e_dn14: f64 = *var_rd23e_dn14_slot;
        let mut var_rd23e_dn2: f64 = *var_rd23e_dn2_slot;
        let mut var_rd23e_dn4: f64 = *var_rd23e_dn4_slot;
        let mut var_rd23e_dn5: f64 = *var_rd23e_dn5_slot;
        let mut var_rd23e_dn6: f64 = *var_rd23e_dn6_slot;
        let mut var_rd23e_dn7: f64 = *var_rd23e_dn7_slot;
        let mut var_rd23e_dn8: f64 = *var_rd23e_dn8_slot;
        let mut var_rd23e_dn9: f64 = *var_rd23e_dn9_slot;
        let mut var_rd23e_rv: f64 = *var_rd23e_rv_slot;
        let mut var_rdrmuele: f64 = *var_rdrmuele_slot;
        let mut var_rdrmuele_rv: f64 = *var_rdrmuele_rv_slot;
        let mut var_rdrmuevbs: f64 = *var_rdrmuevbs_slot;
        let mut var_rdrmuevbs_dn0: f64 = *var_rdrmuevbs_dn0_slot;
        let mut var_rdrmuevbs_dn10: f64 = *var_rdrmuevbs_dn10_slot;
        let mut var_rdrmuevbs_dn11: f64 = *var_rdrmuevbs_dn11_slot;
        let mut var_rdrmuevbs_dn14: f64 = *var_rdrmuevbs_dn14_slot;
        let mut var_rdrmuevbs_dn2: f64 = *var_rdrmuevbs_dn2_slot;
        let mut var_rdrmuevbs_dn4: f64 = *var_rdrmuevbs_dn4_slot;
        let mut var_rdrmuevbs_dn5: f64 = *var_rdrmuevbs_dn5_slot;
        let mut var_rdrmuevbs_dn6: f64 = *var_rdrmuevbs_dn6_slot;
        let mut var_rdrmuevbs_dn7: f64 = *var_rdrmuevbs_dn7_slot;
        let mut var_rdrmuevbs_dn8: f64 = *var_rdrmuevbs_dn8_slot;
        let mut var_rdrmuevbs_dn9: f64 = *var_rdrmuevbs_dn9_slot;
        let mut var_rdrmuevbs_rv: f64 = *var_rdrmuevbs_rv_slot;
        let mut var_rdrvmaxle: f64 = *var_rdrvmaxle_slot;
        let mut var_rdrvmaxle_rv: f64 = *var_rdrvmaxle_rv_slot;
        let mut var_rdrvmaxwe: f64 = *var_rdrvmaxwe_slot;
        let mut var_rdrvmaxwe_rv: f64 = *var_rdrvmaxwe_rv_slot;
        let mut var_rdtemp0: f64 = *var_rdtemp0_slot;
        let mut var_rdtemp0_rv: f64 = *var_rdtemp0_rv_slot;
        let mut var_rdvdtemp0: f64 = *var_rdvdtemp0_slot;
        let mut var_rdvdtemp0_dn0: f64 = *var_rdvdtemp0_dn0_slot;
        let mut var_rdvdtemp0_dn10: f64 = *var_rdvdtemp0_dn10_slot;
        let mut var_rdvdtemp0_dn11: f64 = *var_rdvdtemp0_dn11_slot;
        let mut var_rdvdtemp0_dn14: f64 = *var_rdvdtemp0_dn14_slot;
        let mut var_rdvdtemp0_dn2: f64 = *var_rdvdtemp0_dn2_slot;
        let mut var_rdvdtemp0_dn4: f64 = *var_rdvdtemp0_dn4_slot;
        let mut var_rdvdtemp0_dn5: f64 = *var_rdvdtemp0_dn5_slot;
        let mut var_rdvdtemp0_dn6: f64 = *var_rdvdtemp0_dn6_slot;
        let mut var_rdvdtemp0_dn7: f64 = *var_rdvdtemp0_dn7_slot;
        let mut var_rdvdtemp0_dn8: f64 = *var_rdvdtemp0_dn8_slot;
        let mut var_rdvdtemp0_dn9: f64 = *var_rdvdtemp0_dn9_slot;
        let mut var_rdvdtemp0_rv: f64 = *var_rdvdtemp0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn14: f64 = *var_t3_dn14_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_xmax: f64 = *var_xmax_slot;
        let mut var_xmax_rv: f64 = *var_xmax_rv_slot;
        let mut var_xmax_s: f64 = *var_xmax_s_slot;
        let mut var_xmax_s_rv: f64 = *var_xmax_s_rv_slot;

        let (assign12120_e6665, assign12120_e6665_d_n0, assign12120_e6665_d_n2, assign12120_e6665_d_n4, assign12120_e6665_d_n5, assign12120_e6665_d_n6, assign12120_e6665_d_n7, assign12120_e6665_d_n8, assign12120_e6665_d_n9, assign12120_e6665_d_n10, assign12120_e6665_d_n11, assign12120_e6665_d_n14,) = {
    if ((var_guard275 != 0.0) && (var_guard279 != 0.0)) {
        let assign12120_e6663: f64 = (var_t1).exp();
        (assign12120_e6663, (assign12120_e6663 * var_t1_dn0), (assign12120_e6663 * var_t1_dn2), (assign12120_e6663 * var_t1_dn4), (assign12120_e6663 * var_t1_dn5), (assign12120_e6663 * var_t1_dn6), (assign12120_e6663 * var_t1_dn7), (assign12120_e6663 * var_t1_dn8), (assign12120_e6663 * var_t1_dn9), (assign12120_e6663 * var_t1_dn10), (assign12120_e6663 * var_t1_dn11), (assign12120_e6663 * var_t1_dn14),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign12120_e6665;
        var_t1_dn0 = assign12120_e6665_d_n0;
        var_t1_dn2 = assign12120_e6665_d_n2;
        var_t1_dn4 = assign12120_e6665_d_n4;
        var_t1_dn5 = assign12120_e6665_d_n5;
        var_t1_dn6 = assign12120_e6665_d_n6;
        var_t1_dn7 = assign12120_e6665_d_n7;
        var_t1_dn8 = assign12120_e6665_d_n8;
        var_t1_dn9 = assign12120_e6665_d_n9;
        var_t1_dn10 = assign12120_e6665_d_n10;
        var_t1_dn11 = assign12120_e6665_d_n11;
        var_t1_dn14 = assign12120_e6665_d_n14;
        var_t1_rv = 0.0;

        let (assign12130_e6675, assign12130_e6675_d_n0, assign12130_e6675_d_n2, assign12130_e6675_d_n4, assign12130_e6675_d_n5, assign12130_e6675_d_n6, assign12130_e6675_d_n7, assign12130_e6675_d_n8, assign12130_e6675_d_n9, assign12130_e6675_d_n10, assign12130_e6675_d_n11, assign12130_e6675_d_n14,) = {
    if ((var_guard275 != 0.0) && (var_guard279 != 0.0)) {
        let assign12130_e6671: f64 = (var_uc_rd23 * var_t2);
        let assign12130_e6673: f64 = (assign12130_e6671 * var_t1);
        (assign12130_e6673, (((var_uc_rd23 * var_t2_dn0) * var_t1) + (assign12130_e6671 * var_t1_dn0)), (((var_uc_rd23 * var_t2_dn2) * var_t1) + (assign12130_e6671 * var_t1_dn2)), (((var_uc_rd23 * var_t2_dn4) * var_t1) + (assign12130_e6671 * var_t1_dn4)), (((var_uc_rd23 * var_t2_dn5) * var_t1) + (assign12130_e6671 * var_t1_dn5)), (((var_uc_rd23 * var_t2_dn6) * var_t1) + (assign12130_e6671 * var_t1_dn6)), (((var_uc_rd23 * var_t2_dn7) * var_t1) + (assign12130_e6671 * var_t1_dn7)), (((var_uc_rd23 * var_t2_dn8) * var_t1) + (assign12130_e6671 * var_t1_dn8)), (((var_uc_rd23 * var_t2_dn9) * var_t1) + (assign12130_e6671 * var_t1_dn9)), (((var_uc_rd23 * var_t2_dn10) * var_t1) + (assign12130_e6671 * var_t1_dn10)), (((var_uc_rd23 * var_t2_dn11) * var_t1) + (assign12130_e6671 * var_t1_dn11)), (((var_uc_rd23 * var_t2_dn14) * var_t1) + (assign12130_e6671 * var_t1_dn14)),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn14,)
    }
};
        var_t3 = assign12130_e6675;
        var_t3_dn0 = assign12130_e6675_d_n0;
        var_t3_dn2 = assign12130_e6675_d_n2;
        var_t3_dn4 = assign12130_e6675_d_n4;
        var_t3_dn5 = assign12130_e6675_d_n5;
        var_t3_dn6 = assign12130_e6675_d_n6;
        var_t3_dn7 = assign12130_e6675_d_n7;
        var_t3_dn8 = assign12130_e6675_d_n8;
        var_t3_dn9 = assign12130_e6675_d_n9;
        var_t3_dn10 = assign12130_e6675_d_n10;
        var_t3_dn11 = assign12130_e6675_d_n11;
        var_t3_dn14 = assign12130_e6675_d_n14;
        var_t3_rv = 0.0;

        let (assign12140_e6698, assign12140_e6698_d_n0, assign12140_e6698_d_n2, assign12140_e6698_d_n4, assign12140_e6698_d_n5, assign12140_e6698_d_n6, assign12140_e6698_d_n7, assign12140_e6698_d_n8, assign12140_e6698_d_n9, assign12140_e6698_d_n10, assign12140_e6698_d_n11, assign12140_e6698_d_n14,) = {
    if ((var_guard275 != 0.0) && (var_guard279 != 0.0)) {
        let assign12140_e6683: f64 = (var_t3 * var_t3);
        let assign12140_e6686: f64 = (4.0 * 1e-6);
        let assign12140_e6688: f64 = (assign12140_e6686 / 100.0);
        let assign12140_e6690: f64 = (assign12140_e6688 * 1e-6);
        let assign12140_e6692: f64 = (assign12140_e6690 / 100.0);
        let assign12140_e6693: f64 = (assign12140_e6683 + assign12140_e6692);
        let assign12140_e6694: f64 = (assign12140_e6693).sqrt();
        let assign12140_e6695: f64 = (var_t3 + assign12140_e6694);
        let assign12140_e6696: f64 = (0.5 * assign12140_e6695);
        (assign12140_e6696, (0.5 * (var_t3_dn0 + (((var_t3_dn0 * var_t3) + (var_t3 * var_t3_dn0)) / (2.0 * assign12140_e6694)))), (0.5 * (var_t3_dn2 + (((var_t3_dn2 * var_t3) + (var_t3 * var_t3_dn2)) / (2.0 * assign12140_e6694)))), (0.5 * (var_t3_dn4 + (((var_t3_dn4 * var_t3) + (var_t3 * var_t3_dn4)) / (2.0 * assign12140_e6694)))), (0.5 * (var_t3_dn5 + (((var_t3_dn5 * var_t3) + (var_t3 * var_t3_dn5)) / (2.0 * assign12140_e6694)))), (0.5 * (var_t3_dn6 + (((var_t3_dn6 * var_t3) + (var_t3 * var_t3_dn6)) / (2.0 * assign12140_e6694)))), (0.5 * (var_t3_dn7 + (((var_t3_dn7 * var_t3) + (var_t3 * var_t3_dn7)) / (2.0 * assign12140_e6694)))), (0.5 * (var_t3_dn8 + (((var_t3_dn8 * var_t3) + (var_t3 * var_t3_dn8)) / (2.0 * assign12140_e6694)))), (0.5 * (var_t3_dn9 + (((var_t3_dn9 * var_t3) + (var_t3 * var_t3_dn9)) / (2.0 * assign12140_e6694)))), (0.5 * (var_t3_dn10 + (((var_t3_dn10 * var_t3) + (var_t3 * var_t3_dn10)) / (2.0 * assign12140_e6694)))), (0.5 * (var_t3_dn11 + (((var_t3_dn11 * var_t3) + (var_t3 * var_t3_dn11)) / (2.0 * assign12140_e6694)))), (0.5 * (var_t3_dn14 + (((var_t3_dn14 * var_t3) + (var_t3 * var_t3_dn14)) / (2.0 * assign12140_e6694)))),)
    } else {
        (var_rd23e, var_rd23e_dn0, var_rd23e_dn2, var_rd23e_dn4, var_rd23e_dn5, var_rd23e_dn6, var_rd23e_dn7, var_rd23e_dn8, var_rd23e_dn9, var_rd23e_dn10, var_rd23e_dn11, var_rd23e_dn14,)
    }
};
        var_rd23e = assign12140_e6698;
        var_rd23e_dn0 = assign12140_e6698_d_n0;
        var_rd23e_dn2 = assign12140_e6698_d_n2;
        var_rd23e_dn4 = assign12140_e6698_d_n4;
        var_rd23e_dn5 = assign12140_e6698_d_n5;
        var_rd23e_dn6 = assign12140_e6698_d_n6;
        var_rd23e_dn7 = assign12140_e6698_d_n7;
        var_rd23e_dn8 = assign12140_e6698_d_n8;
        var_rd23e_dn9 = assign12140_e6698_d_n9;
        var_rd23e_dn10 = assign12140_e6698_d_n10;
        var_rd23e_dn11 = assign12140_e6698_d_n11;
        var_rd23e_dn14 = assign12140_e6698_d_n14;
        var_rd23e_rv = 0.0;

        let (assign12150_e6705, assign12150_e6705_d_n0, assign12150_e6705_d_n2, assign12150_e6705_d_n4, assign12150_e6705_d_n5, assign12150_e6705_d_n6, assign12150_e6705_d_n7, assign12150_e6705_d_n8, assign12150_e6705_d_n9, assign12150_e6705_d_n10, assign12150_e6705_d_n11, assign12150_e6705_d_n14,) = {
    if ((var_guard275 != 0.0) && (var_guard279 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rd23e, var_rd23e_dn0, var_rd23e_dn2, var_rd23e_dn4, var_rd23e_dn5, var_rd23e_dn6, var_rd23e_dn7, var_rd23e_dn8, var_rd23e_dn9, var_rd23e_dn10, var_rd23e_dn11, var_rd23e_dn14,)
    }
};
        var_rd23e = assign12150_e6705;
        var_rd23e_dn0 = assign12150_e6705_d_n0;
        var_rd23e_dn2 = assign12150_e6705_d_n2;
        var_rd23e_dn4 = assign12150_e6705_d_n4;
        var_rd23e_dn5 = assign12150_e6705_d_n5;
        var_rd23e_dn6 = assign12150_e6705_d_n6;
        var_rd23e_dn7 = assign12150_e6705_d_n7;
        var_rd23e_dn8 = assign12150_e6705_d_n8;
        var_rd23e_dn9 = assign12150_e6705_d_n9;
        var_rd23e_dn10 = assign12150_e6705_d_n10;
        var_rd23e_dn11 = assign12150_e6705_d_n11;
        var_rd23e_dn14 = assign12150_e6705_d_n14;
        var_rd23e_rv = 0.0;

        let (assign12160_e6709,) = {
    if (var_guard275 != 0.0) {
        (0.0,)
    } else {
        (var_xmax,)
    }
};
        var_xmax = assign12160_e6709;
        var_xmax_rv = 0.0;

        let (assign12170_e6713,) = {
    if (var_guard275 != 0.0) {
        (0.0,)
    } else {
        (var_xmax_s,)
    }
};
        var_xmax_s = assign12170_e6713;
        var_xmax_s_rv = 0.0;

        let (assign12180_e6717,) = {
    if (var_guard275 != 0.0) {
        (0.0,)
    } else {
        (var_rdrvmaxwe,)
    }
};
        var_rdrvmaxwe = assign12180_e6717;
        var_rdrvmaxwe_rv = 0.0;

        let (assign12190_e6721,) = {
    if (var_guard275 != 0.0) {
        (0.0,)
    } else {
        (var_rdrvmaxle,)
    }
};
        var_rdrvmaxle = assign12190_e6721;
        var_rdrvmaxle_rv = 0.0;

        let (assign12200_e6725,) = {
    if (var_guard275 != 0.0) {
        (0.0,)
    } else {
        (var_rdrmuele,)
    }
};
        var_rdrmuele = assign12200_e6725;
        var_rdrmuele_rv = 0.0;

        let (assign12210_e6729, assign12210_e6729_d_n0, assign12210_e6729_d_n2, assign12210_e6729_d_n4, assign12210_e6729_d_n5, assign12210_e6729_d_n6, assign12210_e6729_d_n7, assign12210_e6729_d_n8, assign12210_e6729_d_n9, assign12210_e6729_d_n10, assign12210_e6729_d_n11, assign12210_e6729_d_n14,) = {
    if (var_guard275 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rdrmuevbs, var_rdrmuevbs_dn0, var_rdrmuevbs_dn2, var_rdrmuevbs_dn4, var_rdrmuevbs_dn5, var_rdrmuevbs_dn6, var_rdrmuevbs_dn7, var_rdrmuevbs_dn8, var_rdrmuevbs_dn9, var_rdrmuevbs_dn10, var_rdrmuevbs_dn11, var_rdrmuevbs_dn14,)
    }
};
        var_rdrmuevbs = assign12210_e6729;
        var_rdrmuevbs_dn0 = assign12210_e6729_d_n0;
        var_rdrmuevbs_dn2 = assign12210_e6729_d_n2;
        var_rdrmuevbs_dn4 = assign12210_e6729_d_n4;
        var_rdrmuevbs_dn5 = assign12210_e6729_d_n5;
        var_rdrmuevbs_dn6 = assign12210_e6729_d_n6;
        var_rdrmuevbs_dn7 = assign12210_e6729_d_n7;
        var_rdrmuevbs_dn8 = assign12210_e6729_d_n8;
        var_rdrmuevbs_dn9 = assign12210_e6729_d_n9;
        var_rdrmuevbs_dn10 = assign12210_e6729_d_n10;
        var_rdrmuevbs_dn11 = assign12210_e6729_d_n11;
        var_rdrmuevbs_dn14 = assign12210_e6729_d_n14;
        var_rdrmuevbs_rv = 0.0;

        let (assign12220_e6741,) = {
    if (var_guard275 == 0.0) {
        let assign12220_e6734: f64 = (p.p419 * p.p419);
        let assign12220_e6737: f64 = (var_uc_xldld * var_uc_xldld);
        let assign12220_e6738: f64 = (assign12220_e6734 + assign12220_e6737);
        let assign12220_e6739: f64 = (assign12220_e6738).sqrt();
        (assign12220_e6739,)
    } else {
        (var_xmax,)
    }
};
        var_xmax = assign12220_e6741;
        var_xmax_rv = 0.0;

        let (assign12230_e6753,) = {
    if (var_guard275 == 0.0) {
        let assign12230_e6746: f64 = (p.p419 * p.p419);
        let assign12230_e6749: f64 = (p.p97 * p.p97);
        let assign12230_e6750: f64 = (assign12230_e6746 + assign12230_e6749);
        let assign12230_e6751: f64 = (assign12230_e6750).sqrt();
        (assign12230_e6751,)
    } else {
        (var_xmax_s,)
    }
};
        var_xmax_s = assign12230_e6753;
        var_xmax_s_rv = 0.0;

        let (assign12240_e6764,) = {
    if (var_guard275 == 0.0) {
        let assign12240_e6760: f64 = (var_wg).powf(p.p425);
        let assign12240_e6761: f64 = (p.p424 / assign12240_e6760);
        let assign12240_e6762: f64 = (1.0 + assign12240_e6761);
        (assign12240_e6762,)
    } else {
        (var_rdrvmaxwe,)
    }
};
        var_rdrvmaxwe = assign12240_e6764;
        var_rdrvmaxwe_rv = 0.0;

        let (assign12250_e6775,) = {
    if (var_guard275 == 0.0) {
        let assign12250_e6771: f64 = (var_lg).powf(p.p427);
        let assign12250_e6772: f64 = (p.p426 / assign12250_e6771);
        let assign12250_e6773: f64 = (1.0 + assign12250_e6772);
        (assign12250_e6773,)
    } else {
        (var_rdrvmaxle,)
    }
};
        var_rdrvmaxle = assign12250_e6775;
        var_rdrvmaxle_rv = 0.0;

        let (assign12260_e6786,) = {
    if (var_guard275 == 0.0) {
        let assign12260_e6782: f64 = (var_lg).powf(p.p429);
        let assign12260_e6783: f64 = (p.p428 / assign12260_e6782);
        let assign12260_e6784: f64 = (1.0 + assign12260_e6783);
        (assign12260_e6784,)
    } else {
        (var_rdrmuele,)
    }
};
        var_rdrmuele = assign12260_e6786;
        var_rdrmuele_rv = 0.0;

        let (assign12270_e6791, assign12270_e6791_d_n0, assign12270_e6791_d_n2, assign12270_e6791_d_n4, assign12270_e6791_d_n5, assign12270_e6791_d_n6, assign12270_e6791_d_n7, assign12270_e6791_d_n8, assign12270_e6791_d_n9, assign12270_e6791_d_n10, assign12270_e6791_d_n11, assign12270_e6791_d_n14,) = {
    if (var_guard275 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rdrmuevbs, var_rdrmuevbs_dn0, var_rdrmuevbs_dn2, var_rdrmuevbs_dn4, var_rdrmuevbs_dn5, var_rdrmuevbs_dn6, var_rdrmuevbs_dn7, var_rdrmuevbs_dn8, var_rdrmuevbs_dn9, var_rdrmuevbs_dn10, var_rdrmuevbs_dn11, var_rdrmuevbs_dn14,)
    }
};
        var_rdrmuevbs = assign12270_e6791;
        var_rdrmuevbs_dn0 = assign12270_e6791_d_n0;
        var_rdrmuevbs_dn2 = assign12270_e6791_d_n2;
        var_rdrmuevbs_dn4 = assign12270_e6791_d_n4;
        var_rdrmuevbs_dn5 = assign12270_e6791_d_n5;
        var_rdrmuevbs_dn6 = assign12270_e6791_d_n6;
        var_rdrmuevbs_dn7 = assign12270_e6791_d_n7;
        var_rdrmuevbs_dn8 = assign12270_e6791_d_n8;
        var_rdrmuevbs_dn9 = assign12270_e6791_d_n9;
        var_rdrmuevbs_dn10 = assign12270_e6791_d_n10;
        var_rdrmuevbs_dn11 = assign12270_e6791_d_n11;
        var_rdrmuevbs_dn14 = assign12270_e6791_d_n14;
        var_rdrmuevbs_rv = 0.0;

        let (assign12280_e6796,) = {
    if (var_guard275 == 0.0) {
        (0.0,)
    } else {
        (var_rdtemp0,)
    }
};
        var_rdtemp0 = assign12280_e6796;
        var_rdtemp0_rv = 0.0;

        let (assign12290_e6801, assign12290_e6801_d_n0, assign12290_e6801_d_n2, assign12290_e6801_d_n4, assign12290_e6801_d_n5, assign12290_e6801_d_n6, assign12290_e6801_d_n7, assign12290_e6801_d_n8, assign12290_e6801_d_n9, assign12290_e6801_d_n10, assign12290_e6801_d_n11, assign12290_e6801_d_n14,) = {
    if (var_guard275 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rdvdtemp0, var_rdvdtemp0_dn0, var_rdvdtemp0_dn2, var_rdvdtemp0_dn4, var_rdvdtemp0_dn5, var_rdvdtemp0_dn6, var_rdvdtemp0_dn7, var_rdvdtemp0_dn8, var_rdvdtemp0_dn9, var_rdvdtemp0_dn10, var_rdvdtemp0_dn11, var_rdvdtemp0_dn14,)
    }
};
        var_rdvdtemp0 = assign12290_e6801;
        var_rdvdtemp0_dn0 = assign12290_e6801_d_n0;
        var_rdvdtemp0_dn2 = assign12290_e6801_d_n2;
        var_rdvdtemp0_dn4 = assign12290_e6801_d_n4;
        var_rdvdtemp0_dn5 = assign12290_e6801_d_n5;
        var_rdvdtemp0_dn6 = assign12290_e6801_d_n6;
        var_rdvdtemp0_dn7 = assign12290_e6801_d_n7;
        var_rdvdtemp0_dn8 = assign12290_e6801_d_n8;
        var_rdvdtemp0_dn9 = assign12290_e6801_d_n9;
        var_rdvdtemp0_dn10 = assign12290_e6801_d_n10;
        var_rdvdtemp0_dn11 = assign12290_e6801_d_n11;
        var_rdvdtemp0_dn14 = assign12290_e6801_d_n14;
        var_rdvdtemp0_rv = 0.0;

        let (assign12300_e6806, assign12300_e6806_d_n0, assign12300_e6806_d_n2, assign12300_e6806_d_n4, assign12300_e6806_d_n5, assign12300_e6806_d_n6, assign12300_e6806_d_n7, assign12300_e6806_d_n8, assign12300_e6806_d_n9, assign12300_e6806_d_n10, assign12300_e6806_d_n11, assign12300_e6806_d_n14,) = {
    if (var_guard275 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_rd23e, var_rd23e_dn0, var_rd23e_dn2, var_rd23e_dn4, var_rd23e_dn5, var_rd23e_dn6, var_rd23e_dn7, var_rd23e_dn8, var_rd23e_dn9, var_rd23e_dn10, var_rd23e_dn11, var_rd23e_dn14,)
    }
};
        var_rd23e = assign12300_e6806;
        var_rd23e_dn0 = assign12300_e6806_d_n0;
        var_rd23e_dn2 = assign12300_e6806_d_n2;
        var_rd23e_dn4 = assign12300_e6806_d_n4;
        var_rd23e_dn5 = assign12300_e6806_d_n5;
        var_rd23e_dn6 = assign12300_e6806_d_n6;
        var_rd23e_dn7 = assign12300_e6806_d_n7;
        var_rd23e_dn8 = assign12300_e6806_d_n8;
        var_rd23e_dn9 = assign12300_e6806_d_n9;
        var_rd23e_dn10 = assign12300_e6806_d_n10;
        var_rd23e_dn11 = assign12300_e6806_d_n11;
        var_rd23e_dn14 = assign12300_e6806_d_n14;
        var_rd23e_rv = 0.0;

        let assign12310_e6809: f64 = if var_uc_nover > 0.0 { 1.0 } else { 0.0 };
        var_guard281 = assign12310_e6809;
        var_guard281_rv = 0.0;

        let (assign12320_e6819,) = {
    if (var_guard281 != 0.0) {
        let assign12320_e6813: f64 = (2.0 * 1.034943e-10);
        let assign12320_e6816: f64 = (1.6021918e-19 * var_uc_nover);
        let assign12320_e6817: f64 = (assign12320_e6813 / assign12320_e6816);
        (assign12320_e6817,)
    } else {
        (var_kdep,)
    }
};
        var_kdep = assign12320_e6819;
        var_kdep_rv = 0.0;

        let (assign12330_e6835, assign12330_e6835_d_n0, assign12330_e6835_d_n2, assign12330_e6835_d_n4, assign12330_e6835_d_n5, assign12330_e6835_d_n6, assign12330_e6835_d_n7, assign12330_e6835_d_n8, assign12330_e6835_d_n9, assign12330_e6835_d_n10, assign12330_e6835_d_n11, assign12330_e6835_d_n14,) = {
    if (var_guard281 != 0.0) {
        let assign12330_e6823: f64 = (2.0 * 1.034943e-10);
        let assign12330_e6825: f64 = (assign12330_e6823 / 1.6021918e-19);
        let assign12330_e6827: f64 = (assign12330_e6825 * var_ef_nsubc);
        let assign12330_e6830: f64 = (var_uc_nover + var_ef_nsubc);
        let assign12330_e6831: f64 = (assign12330_e6827 / assign12330_e6830);
        let assign12330_e6833: f64 = (assign12330_e6831 / var_uc_nover);
        (assign12330_e6833, (((((assign12330_e6825 * var_ef_nsubc_dn0) * assign12330_e6830) - (assign12330_e6827 * var_ef_nsubc_dn0)) / (assign12330_e6830 * assign12330_e6830)) / var_uc_nover), (((((assign12330_e6825 * var_ef_nsubc_dn2) * assign12330_e6830) - (assign12330_e6827 * var_ef_nsubc_dn2)) / (assign12330_e6830 * assign12330_e6830)) / var_uc_nover), (((((assign12330_e6825 * var_ef_nsubc_dn4) * assign12330_e6830) - (assign12330_e6827 * var_ef_nsubc_dn4)) / (assign12330_e6830 * assign12330_e6830)) / var_uc_nover), (((((assign12330_e6825 * var_ef_nsubc_dn5) * assign12330_e6830) - (assign12330_e6827 * var_ef_nsubc_dn5)) / (assign12330_e6830 * assign12330_e6830)) / var_uc_nover), (((((assign12330_e6825 * var_ef_nsubc_dn6) * assign12330_e6830) - (assign12330_e6827 * var_ef_nsubc_dn6)) / (assign12330_e6830 * assign12330_e6830)) / var_uc_nover), (((((assign12330_e6825 * var_ef_nsubc_dn7) * assign12330_e6830) - (assign12330_e6827 * var_ef_nsubc_dn7)) / (assign12330_e6830 * assign12330_e6830)) / var_uc_nover), (((((assign12330_e6825 * var_ef_nsubc_dn8) * assign12330_e6830) - (assign12330_e6827 * var_ef_nsubc_dn8)) / (assign12330_e6830 * assign12330_e6830)) / var_uc_nover), (((((assign12330_e6825 * var_ef_nsubc_dn9) * assign12330_e6830) - (assign12330_e6827 * var_ef_nsubc_dn9)) / (assign12330_e6830 * assign12330_e6830)) / var_uc_nover), (((((assign12330_e6825 * var_ef_nsubc_dn10) * assign12330_e6830) - (assign12330_e6827 * var_ef_nsubc_dn10)) / (assign12330_e6830 * assign12330_e6830)) / var_uc_nover), (((((assign12330_e6825 * var_ef_nsubc_dn11) * assign12330_e6830) - (assign12330_e6827 * var_ef_nsubc_dn11)) / (assign12330_e6830 * assign12330_e6830)) / var_uc_nover), (((((assign12330_e6825 * var_ef_nsubc_dn14) * assign12330_e6830) - (assign12330_e6827 * var_ef_nsubc_dn14)) / (assign12330_e6830 * assign12330_e6830)) / var_uc_nover),)
    } else {
        (var_kjunc, var_kjunc_dn0, var_kjunc_dn2, var_kjunc_dn4, var_kjunc_dn5, var_kjunc_dn6, var_kjunc_dn7, var_kjunc_dn8, var_kjunc_dn9, var_kjunc_dn10, var_kjunc_dn11, var_kjunc_dn14,)
    }
};
        var_kjunc = assign12330_e6835;
        var_kjunc_dn0 = assign12330_e6835_d_n0;
        var_kjunc_dn2 = assign12330_e6835_d_n2;
        var_kjunc_dn4 = assign12330_e6835_d_n4;
        var_kjunc_dn5 = assign12330_e6835_d_n5;
        var_kjunc_dn6 = assign12330_e6835_d_n6;
        var_kjunc_dn7 = assign12330_e6835_d_n7;
        var_kjunc_dn8 = assign12330_e6835_d_n8;
        var_kjunc_dn9 = assign12330_e6835_d_n9;
        var_kjunc_dn10 = assign12330_e6835_d_n10;
        var_kjunc_dn11 = assign12330_e6835_d_n11;
        var_kjunc_dn14 = assign12330_e6835_d_n14;
        var_kjunc_rv = 0.0;

        let (assign12340_e6840,) = {
    if (var_guard281 == 0.0) {
        (0.0,)
    } else {
        (var_kdep,)
    }
};
        var_kdep = assign12340_e6840;
        var_kdep_rv = 0.0;

        let (assign12350_e6845, assign12350_e6845_d_n0, assign12350_e6845_d_n2, assign12350_e6845_d_n4, assign12350_e6845_d_n5, assign12350_e6845_d_n6, assign12350_e6845_d_n7, assign12350_e6845_d_n8, assign12350_e6845_d_n9, assign12350_e6845_d_n10, assign12350_e6845_d_n11, assign12350_e6845_d_n14,) = {
    if (var_guard281 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_kjunc, var_kjunc_dn0, var_kjunc_dn2, var_kjunc_dn4, var_kjunc_dn5, var_kjunc_dn6, var_kjunc_dn7, var_kjunc_dn8, var_kjunc_dn9, var_kjunc_dn10, var_kjunc_dn11, var_kjunc_dn14,)
    }
};
        var_kjunc = assign12350_e6845;
        var_kjunc_dn0 = assign12350_e6845_d_n0;
        var_kjunc_dn2 = assign12350_e6845_d_n2;
        var_kjunc_dn4 = assign12350_e6845_d_n4;
        var_kjunc_dn5 = assign12350_e6845_d_n5;
        var_kjunc_dn6 = assign12350_e6845_d_n6;
        var_kjunc_dn7 = assign12350_e6845_d_n7;
        var_kjunc_dn8 = assign12350_e6845_d_n8;
        var_kjunc_dn9 = assign12350_e6845_d_n9;
        var_kjunc_dn10 = assign12350_e6845_d_n10;
        var_kjunc_dn11 = assign12350_e6845_d_n11;
        var_kjunc_dn14 = assign12350_e6845_d_n14;
        var_kjunc_rv = 0.0;

        let assign12490_e6940: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        var_guard286 = assign12490_e6940;
        var_guard286_rv = 0.0;

        let (assign12500_e6948, assign12500_e6948_d_n0, assign12500_e6948_d_n2, assign12500_e6948_d_n4, assign12500_e6948_d_n5, assign12500_e6948_d_n6, assign12500_e6948_d_n7, assign12500_e6948_d_n8, assign12500_e6948_d_n9, assign12500_e6948_d_n10, assign12500_e6948_d_n11, assign12500_e6948_d_n14,) = {
    if (var_guard286 != 0.0) {
        let assign12500_e6944: f64 = (p.p108 * var_lg);
        let assign12500_e6946: f64 = (assign12500_e6944 + p.p109);
        (assign12500_e6946, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign12500_e6948;
        var_t1_dn0 = assign12500_e6948_d_n0;
        var_t1_dn2 = assign12500_e6948_d_n2;
        var_t1_dn4 = assign12500_e6948_d_n4;
        var_t1_dn5 = assign12500_e6948_d_n5;
        var_t1_dn6 = assign12500_e6948_d_n6;
        var_t1_dn7 = assign12500_e6948_d_n7;
        var_t1_dn8 = assign12500_e6948_d_n8;
        var_t1_dn9 = assign12500_e6948_d_n9;
        var_t1_dn10 = assign12500_e6948_d_n10;
        var_t1_dn11 = assign12500_e6948_d_n11;
        var_t1_dn14 = assign12500_e6948_d_n14;
        var_t1_rv = 0.0;

        let assign12510_e6951: f64 = if var_t1 < 0.0 { 1.0 } else { 0.0 };
        var_guard287 = assign12510_e6951;
        var_guard287_rv = 0.0;

        let (assign12520_e6957, assign12520_e6957_d_n0, assign12520_e6957_d_n2, assign12520_e6957_d_n4, assign12520_e6957_d_n5, assign12520_e6957_d_n6, assign12520_e6957_d_n7, assign12520_e6957_d_n8, assign12520_e6957_d_n9, assign12520_e6957_d_n10, assign12520_e6957_d_n11, assign12520_e6957_d_n14,) = {
    if ((var_guard286 != 0.0) && (var_guard287 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign12520_e6957;
        var_t1_dn0 = assign12520_e6957_d_n0;
        var_t1_dn2 = assign12520_e6957_d_n2;
        var_t1_dn4 = assign12520_e6957_d_n4;
        var_t1_dn5 = assign12520_e6957_d_n5;
        var_t1_dn6 = assign12520_e6957_d_n6;
        var_t1_dn7 = assign12520_e6957_d_n7;
        var_t1_dn8 = assign12520_e6957_d_n8;
        var_t1_dn9 = assign12520_e6957_d_n9;
        var_t1_dn10 = assign12520_e6957_d_n10;
        var_t1_dn11 = assign12520_e6957_d_n11;
        var_t1_dn14 = assign12520_e6957_d_n14;
        var_t1_rv = 0.0;

        let (assign12530_e6969, assign12530_e6969_d_n0, assign12530_e6969_d_n2, assign12530_e6969_d_n4, assign12530_e6969_d_n5, assign12530_e6969_d_n6, assign12530_e6969_d_n7, assign12530_e6969_d_n8, assign12530_e6969_d_n9, assign12530_e6969_d_n10, assign12530_e6969_d_n11, assign12530_e6969_d_n14,) = {
    if (var_guard286 != 0.0) {
        let assign12530_e6961: f64 = (var_t1 * p.p107);
        let assign12530_e6964: f64 = (var_t1 + p.p107);
        let assign12530_e6965: f64 = (assign12530_e6961 / assign12530_e6964);
        let assign12530_e6967: f64 = (assign12530_e6965 + 1.0);
        (assign12530_e6967, ((((var_t1_dn0 * p.p107) * assign12530_e6964) - (assign12530_e6961 * var_t1_dn0)) / (assign12530_e6964 * assign12530_e6964)), ((((var_t1_dn2 * p.p107) * assign12530_e6964) - (assign12530_e6961 * var_t1_dn2)) / (assign12530_e6964 * assign12530_e6964)), ((((var_t1_dn4 * p.p107) * assign12530_e6964) - (assign12530_e6961 * var_t1_dn4)) / (assign12530_e6964 * assign12530_e6964)), ((((var_t1_dn5 * p.p107) * assign12530_e6964) - (assign12530_e6961 * var_t1_dn5)) / (assign12530_e6964 * assign12530_e6964)), ((((var_t1_dn6 * p.p107) * assign12530_e6964) - (assign12530_e6961 * var_t1_dn6)) / (assign12530_e6964 * assign12530_e6964)), ((((var_t1_dn7 * p.p107) * assign12530_e6964) - (assign12530_e6961 * var_t1_dn7)) / (assign12530_e6964 * assign12530_e6964)), ((((var_t1_dn8 * p.p107) * assign12530_e6964) - (assign12530_e6961 * var_t1_dn8)) / (assign12530_e6964 * assign12530_e6964)), ((((var_t1_dn9 * p.p107) * assign12530_e6964) - (assign12530_e6961 * var_t1_dn9)) / (assign12530_e6964 * assign12530_e6964)), ((((var_t1_dn10 * p.p107) * assign12530_e6964) - (assign12530_e6961 * var_t1_dn10)) / (assign12530_e6964 * assign12530_e6964)), ((((var_t1_dn11 * p.p107) * assign12530_e6964) - (assign12530_e6961 * var_t1_dn11)) / (assign12530_e6964 * assign12530_e6964)), ((((var_t1_dn14 * p.p107) * assign12530_e6964) - (assign12530_e6961 * var_t1_dn14)) / (assign12530_e6964 * assign12530_e6964)),)
    } else {
        (var_ddlte, var_ddlte_dn0, var_ddlte_dn2, var_ddlte_dn4, var_ddlte_dn5, var_ddlte_dn6, var_ddlte_dn7, var_ddlte_dn8, var_ddlte_dn9, var_ddlte_dn10, var_ddlte_dn11, var_ddlte_dn14,)
    }
};
        var_ddlte = assign12530_e6969;
        var_ddlte_dn0 = assign12530_e6969_d_n0;
        var_ddlte_dn2 = assign12530_e6969_d_n2;
        var_ddlte_dn4 = assign12530_e6969_d_n4;
        var_ddlte_dn5 = assign12530_e6969_d_n5;
        var_ddlte_dn6 = assign12530_e6969_d_n6;
        var_ddlte_dn7 = assign12530_e6969_d_n7;
        var_ddlte_dn8 = assign12530_e6969_d_n8;
        var_ddlte_dn9 = assign12530_e6969_d_n9;
        var_ddlte_dn10 = assign12530_e6969_d_n10;
        var_ddlte_dn11 = assign12530_e6969_d_n11;
        var_ddlte_dn14 = assign12530_e6969_d_n14;
        var_ddlte_rv = 0.0;

        let (assign12540_e6976, assign12540_e6976_d_n0, assign12540_e6976_d_n2, assign12540_e6976_d_n4, assign12540_e6976_d_n5, assign12540_e6976_d_n6, assign12540_e6976_d_n7, assign12540_e6976_d_n8, assign12540_e6976_d_n9, assign12540_e6976_d_n10, assign12540_e6976_d_n11, assign12540_e6976_d_n14,) = {
    if (var_guard286 == 0.0) {
        let assign12540_e6974: f64 = (p.p108 * var_lg);
        (assign12540_e6974, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign12540_e6976;
        var_t1_dn0 = assign12540_e6976_d_n0;
        var_t1_dn2 = assign12540_e6976_d_n2;
        var_t1_dn4 = assign12540_e6976_d_n4;
        var_t1_dn5 = assign12540_e6976_d_n5;
        var_t1_dn6 = assign12540_e6976_d_n6;
        var_t1_dn7 = assign12540_e6976_d_n7;
        var_t1_dn8 = assign12540_e6976_d_n8;
        var_t1_dn9 = assign12540_e6976_d_n9;
        var_t1_dn10 = assign12540_e6976_d_n10;
        var_t1_dn11 = assign12540_e6976_d_n11;
        var_t1_dn14 = assign12540_e6976_d_n14;
        var_t1_rv = 0.0;

        let assign12550_e6979: f64 = if var_t1 < 0.0 { 1.0 } else { 0.0 };
        var_guard288 = assign12550_e6979;
        var_guard288_rv = 0.0;

        let (assign12560_e6986, assign12560_e6986_d_n0, assign12560_e6986_d_n2, assign12560_e6986_d_n4, assign12560_e6986_d_n5, assign12560_e6986_d_n6, assign12560_e6986_d_n7, assign12560_e6986_d_n8, assign12560_e6986_d_n9, assign12560_e6986_d_n10, assign12560_e6986_d_n11, assign12560_e6986_d_n14,) = {
    if ((var_guard286 == 0.0) && (var_guard288 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign12560_e6986;
        var_t1_dn0 = assign12560_e6986_d_n0;
        var_t1_dn2 = assign12560_e6986_d_n2;
        var_t1_dn4 = assign12560_e6986_d_n4;
        var_t1_dn5 = assign12560_e6986_d_n5;
        var_t1_dn6 = assign12560_e6986_d_n6;
        var_t1_dn7 = assign12560_e6986_d_n7;
        var_t1_dn8 = assign12560_e6986_d_n8;
        var_t1_dn9 = assign12560_e6986_d_n9;
        var_t1_dn10 = assign12560_e6986_d_n10;
        var_t1_dn11 = assign12560_e6986_d_n11;
        var_t1_dn14 = assign12560_e6986_d_n14;
        var_t1_rv = 0.0;

        *var_ddlte_slot = var_ddlte;
        *var_ddlte_dn0_slot = var_ddlte_dn0;
        *var_ddlte_dn10_slot = var_ddlte_dn10;
        *var_ddlte_dn11_slot = var_ddlte_dn11;
        *var_ddlte_dn14_slot = var_ddlte_dn14;
        *var_ddlte_dn2_slot = var_ddlte_dn2;
        *var_ddlte_dn4_slot = var_ddlte_dn4;
        *var_ddlte_dn5_slot = var_ddlte_dn5;
        *var_ddlte_dn6_slot = var_ddlte_dn6;
        *var_ddlte_dn7_slot = var_ddlte_dn7;
        *var_ddlte_dn8_slot = var_ddlte_dn8;
        *var_ddlte_dn9_slot = var_ddlte_dn9;
        *var_ddlte_rv_slot = var_ddlte_rv;
        *var_guard281_slot = var_guard281;
        *var_guard281_rv_slot = var_guard281_rv;
        *var_guard286_slot = var_guard286;
        *var_guard286_rv_slot = var_guard286_rv;
        *var_guard287_slot = var_guard287;
        *var_guard287_rv_slot = var_guard287_rv;
        *var_guard288_slot = var_guard288;
        *var_guard288_rv_slot = var_guard288_rv;
        *var_kdep_slot = var_kdep;
        *var_kdep_rv_slot = var_kdep_rv;
        *var_kjunc_slot = var_kjunc;
        *var_kjunc_dn0_slot = var_kjunc_dn0;
        *var_kjunc_dn10_slot = var_kjunc_dn10;
        *var_kjunc_dn11_slot = var_kjunc_dn11;
        *var_kjunc_dn14_slot = var_kjunc_dn14;
        *var_kjunc_dn2_slot = var_kjunc_dn2;
        *var_kjunc_dn4_slot = var_kjunc_dn4;
        *var_kjunc_dn5_slot = var_kjunc_dn5;
        *var_kjunc_dn6_slot = var_kjunc_dn6;
        *var_kjunc_dn7_slot = var_kjunc_dn7;
        *var_kjunc_dn8_slot = var_kjunc_dn8;
        *var_kjunc_dn9_slot = var_kjunc_dn9;
        *var_kjunc_rv_slot = var_kjunc_rv;
        *var_rd23e_slot = var_rd23e;
        *var_rd23e_dn0_slot = var_rd23e_dn0;
        *var_rd23e_dn10_slot = var_rd23e_dn10;
        *var_rd23e_dn11_slot = var_rd23e_dn11;
        *var_rd23e_dn14_slot = var_rd23e_dn14;
        *var_rd23e_dn2_slot = var_rd23e_dn2;
        *var_rd23e_dn4_slot = var_rd23e_dn4;
        *var_rd23e_dn5_slot = var_rd23e_dn5;
        *var_rd23e_dn6_slot = var_rd23e_dn6;
        *var_rd23e_dn7_slot = var_rd23e_dn7;
        *var_rd23e_dn8_slot = var_rd23e_dn8;
        *var_rd23e_dn9_slot = var_rd23e_dn9;
        *var_rd23e_rv_slot = var_rd23e_rv;
        *var_rdrmuele_slot = var_rdrmuele;
        *var_rdrmuele_rv_slot = var_rdrmuele_rv;
        *var_rdrmuevbs_slot = var_rdrmuevbs;
        *var_rdrmuevbs_dn0_slot = var_rdrmuevbs_dn0;
        *var_rdrmuevbs_dn10_slot = var_rdrmuevbs_dn10;
        *var_rdrmuevbs_dn11_slot = var_rdrmuevbs_dn11;
        *var_rdrmuevbs_dn14_slot = var_rdrmuevbs_dn14;
        *var_rdrmuevbs_dn2_slot = var_rdrmuevbs_dn2;
        *var_rdrmuevbs_dn4_slot = var_rdrmuevbs_dn4;
        *var_rdrmuevbs_dn5_slot = var_rdrmuevbs_dn5;
        *var_rdrmuevbs_dn6_slot = var_rdrmuevbs_dn6;
        *var_rdrmuevbs_dn7_slot = var_rdrmuevbs_dn7;
        *var_rdrmuevbs_dn8_slot = var_rdrmuevbs_dn8;
        *var_rdrmuevbs_dn9_slot = var_rdrmuevbs_dn9;
        *var_rdrmuevbs_rv_slot = var_rdrmuevbs_rv;
        *var_rdrvmaxle_slot = var_rdrvmaxle;
        *var_rdrvmaxle_rv_slot = var_rdrvmaxle_rv;
        *var_rdrvmaxwe_slot = var_rdrvmaxwe;
        *var_rdrvmaxwe_rv_slot = var_rdrvmaxwe_rv;
        *var_rdtemp0_slot = var_rdtemp0;
        *var_rdtemp0_rv_slot = var_rdtemp0_rv;
        *var_rdvdtemp0_slot = var_rdvdtemp0;
        *var_rdvdtemp0_dn0_slot = var_rdvdtemp0_dn0;
        *var_rdvdtemp0_dn10_slot = var_rdvdtemp0_dn10;
        *var_rdvdtemp0_dn11_slot = var_rdvdtemp0_dn11;
        *var_rdvdtemp0_dn14_slot = var_rdvdtemp0_dn14;
        *var_rdvdtemp0_dn2_slot = var_rdvdtemp0_dn2;
        *var_rdvdtemp0_dn4_slot = var_rdvdtemp0_dn4;
        *var_rdvdtemp0_dn5_slot = var_rdvdtemp0_dn5;
        *var_rdvdtemp0_dn6_slot = var_rdvdtemp0_dn6;
        *var_rdvdtemp0_dn7_slot = var_rdvdtemp0_dn7;
        *var_rdvdtemp0_dn8_slot = var_rdvdtemp0_dn8;
        *var_rdvdtemp0_dn9_slot = var_rdvdtemp0_dn9;
        *var_rdvdtemp0_rv_slot = var_rdvdtemp0_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn14_slot = var_t3_dn14;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t3_rv_slot = var_t3_rv;
        *var_xmax_slot = var_xmax;
        *var_xmax_rv_slot = var_xmax_rv;
        *var_xmax_s_slot = var_xmax_s;
        *var_xmax_s_rv_slot = var_xmax_s_rv;
    }

    pub(super) fn stamp_reactive_block_22(
        p: &Parameters,
        var_guard286: f64,
        var_lg: f64,
        var_lgate: f64,
        var_mfactor: f64,
        var_mks_slg: f64,
        var_mks_slgl: f64,
        var_mks_sub1l: f64,
        var_mks_sub2l: f64,
        var_mks_svbsl: f64,
        var_mks_svgsl: f64,
        var_mks_svgsw: f64,
        var_t1: f64,
        var_t1_dn0: f64,
        var_t1_dn10: f64,
        var_t1_dn11: f64,
        var_t1_dn14: f64,
        var_t1_dn2: f64,
        var_t1_dn4: f64,
        var_t1_dn5: f64,
        var_t1_dn6: f64,
        var_t1_dn7: f64,
        var_t1_dn8: f64,
        var_t1_dn9: f64,
        var_uc_gdld: f64,
        var_uc_rth0: f64,
        var_uc_sub1: f64,
        var_uc_sub1snp: f64,
        var_uc_sub2: f64,
        var_uc_sub2snp: f64,
        var_uc_svbs: f64,
        var_uc_svgs: f64,
        var_weff: f64,
        var_weff_nf: f64,
        var_weffcv_nf: f64,
        var_wg: f64,
        var_cfrng_slot: &mut f64,
        var_cfrng_rv_slot: &mut f64,
        var_cqyb0_slot: &mut f64,
        var_cqyb0_rv_slot: &mut f64,
        var_ddlte_slot: &mut f64,
        var_ddlte_dn0_slot: &mut f64,
        var_ddlte_dn10_slot: &mut f64,
        var_ddlte_dn11_slot: &mut f64,
        var_ddlte_dn14_slot: &mut f64,
        var_ddlte_dn2_slot: &mut f64,
        var_ddlte_dn4_slot: &mut f64,
        var_ddlte_dn5_slot: &mut f64,
        var_ddlte_dn6_slot: &mut f64,
        var_ddlte_dn7_slot: &mut f64,
        var_ddlte_dn8_slot: &mut f64,
        var_ddlte_dn9_slot: &mut f64,
        var_ddlte_rv_slot: &mut f64,
        var_gdl0_slot: &mut f64,
        var_gdl0_rv_slot: &mut f64,
        var_guard290_slot: &mut f64,
        var_guard290_rv_slot: &mut f64,
        var_guard291_slot: &mut f64,
        var_guard291_rv_slot: &mut f64,
        var_pt40_slot: &mut f64,
        var_pt40_rv_slot: &mut f64,
        var_ptl0_slot: &mut f64,
        var_ptl0_rv_slot: &mut f64,
        var_rth_slot: &mut f64,
        var_rth_dn0_slot: &mut f64,
        var_rth_dn10_slot: &mut f64,
        var_rth_dn11_slot: &mut f64,
        var_rth_dn14_slot: &mut f64,
        var_rth_dn2_slot: &mut f64,
        var_rth_dn4_slot: &mut f64,
        var_rth_dn5_slot: &mut f64,
        var_rth_dn6_slot: &mut f64,
        var_rth_dn7_slot: &mut f64,
        var_rth_dn8_slot: &mut f64,
        var_rth_dn9_slot: &mut f64,
        var_rth_rv_slot: &mut f64,
        var_rthtemp0_slot: &mut f64,
        var_rthtemp0_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_uc_ibpc1_slot: &mut f64,
        var_uc_ibpc1_rv_slot: &mut f64,
        var_uc_subld1_slot: &mut f64,
        var_uc_subld1_rv_slot: &mut f64,
        var_vg2const_slot: &mut f64,
        var_vg2const_1_slot: &mut f64,
        var_vg2const_1_dn0_slot: &mut f64,
        var_vg2const_1_dn10_slot: &mut f64,
        var_vg2const_1_dn11_slot: &mut f64,
        var_vg2const_1_dn14_slot: &mut f64,
        var_vg2const_1_dn2_slot: &mut f64,
        var_vg2const_1_dn4_slot: &mut f64,
        var_vg2const_1_dn5_slot: &mut f64,
        var_vg2const_1_dn6_slot: &mut f64,
        var_vg2const_1_dn7_slot: &mut f64,
        var_vg2const_1_dn8_slot: &mut f64,
        var_vg2const_1_dn9_slot: &mut f64,
        var_vg2const_1_rv_slot: &mut f64,
        var_vg2const_dn0_slot: &mut f64,
        var_vg2const_dn10_slot: &mut f64,
        var_vg2const_dn11_slot: &mut f64,
        var_vg2const_dn14_slot: &mut f64,
        var_vg2const_dn2_slot: &mut f64,
        var_vg2const_dn4_slot: &mut f64,
        var_vg2const_dn5_slot: &mut f64,
        var_vg2const_dn6_slot: &mut f64,
        var_vg2const_dn7_slot: &mut f64,
        var_vg2const_dn8_slot: &mut f64,
        var_vg2const_dn9_slot: &mut f64,
        var_vg2const_rv_slot: &mut f64,
        var_xgate_slot: &mut f64,
        var_xgate_1_slot: &mut f64,
        var_xgate_1_rv_slot: &mut f64,
        var_xgate_rv_slot: &mut f64,
        var_xsub1_slot: &mut f64,
        var_xsub1_1_slot: &mut f64,
        var_xsub1_1_rv_slot: &mut f64,
        var_xsub1_rv_slot: &mut f64,
        var_xsub2_slot: &mut f64,
        var_xsub2_1_slot: &mut f64,
        var_xsub2_1_rv_slot: &mut f64,
        var_xsub2_rv_slot: &mut f64,
        var_xvbs_slot: &mut f64,
        var_xvbs_1_slot: &mut f64,
        var_xvbs_1_rv_slot: &mut f64,
        var_xvbs_rv_slot: &mut f64,
    ) {
        let mut var_cfrng: f64 = *var_cfrng_slot;
        let mut var_cfrng_rv: f64 = *var_cfrng_rv_slot;
        let mut var_cqyb0: f64 = *var_cqyb0_slot;
        let mut var_cqyb0_rv: f64 = *var_cqyb0_rv_slot;
        let mut var_ddlte: f64 = *var_ddlte_slot;
        let mut var_ddlte_dn0: f64 = *var_ddlte_dn0_slot;
        let mut var_ddlte_dn10: f64 = *var_ddlte_dn10_slot;
        let mut var_ddlte_dn11: f64 = *var_ddlte_dn11_slot;
        let mut var_ddlte_dn14: f64 = *var_ddlte_dn14_slot;
        let mut var_ddlte_dn2: f64 = *var_ddlte_dn2_slot;
        let mut var_ddlte_dn4: f64 = *var_ddlte_dn4_slot;
        let mut var_ddlte_dn5: f64 = *var_ddlte_dn5_slot;
        let mut var_ddlte_dn6: f64 = *var_ddlte_dn6_slot;
        let mut var_ddlte_dn7: f64 = *var_ddlte_dn7_slot;
        let mut var_ddlte_dn8: f64 = *var_ddlte_dn8_slot;
        let mut var_ddlte_dn9: f64 = *var_ddlte_dn9_slot;
        let mut var_ddlte_rv: f64 = *var_ddlte_rv_slot;
        let mut var_gdl0: f64 = *var_gdl0_slot;
        let mut var_gdl0_rv: f64 = *var_gdl0_rv_slot;
        let mut var_guard290: f64 = *var_guard290_slot;
        let mut var_guard290_rv: f64 = *var_guard290_rv_slot;
        let mut var_guard291: f64 = *var_guard291_slot;
        let mut var_guard291_rv: f64 = *var_guard291_rv_slot;
        let mut var_pt40: f64 = *var_pt40_slot;
        let mut var_pt40_rv: f64 = *var_pt40_rv_slot;
        let mut var_ptl0: f64 = *var_ptl0_slot;
        let mut var_ptl0_rv: f64 = *var_ptl0_rv_slot;
        let mut var_rth: f64 = *var_rth_slot;
        let mut var_rth_dn0: f64 = *var_rth_dn0_slot;
        let mut var_rth_dn10: f64 = *var_rth_dn10_slot;
        let mut var_rth_dn11: f64 = *var_rth_dn11_slot;
        let mut var_rth_dn14: f64 = *var_rth_dn14_slot;
        let mut var_rth_dn2: f64 = *var_rth_dn2_slot;
        let mut var_rth_dn4: f64 = *var_rth_dn4_slot;
        let mut var_rth_dn5: f64 = *var_rth_dn5_slot;
        let mut var_rth_dn6: f64 = *var_rth_dn6_slot;
        let mut var_rth_dn7: f64 = *var_rth_dn7_slot;
        let mut var_rth_dn8: f64 = *var_rth_dn8_slot;
        let mut var_rth_dn9: f64 = *var_rth_dn9_slot;
        let mut var_rth_rv: f64 = *var_rth_rv_slot;
        let mut var_rthtemp0: f64 = *var_rthtemp0_slot;
        let mut var_rthtemp0_rv: f64 = *var_rthtemp0_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_uc_ibpc1: f64 = *var_uc_ibpc1_slot;
        let mut var_uc_ibpc1_rv: f64 = *var_uc_ibpc1_rv_slot;
        let mut var_uc_subld1: f64 = *var_uc_subld1_slot;
        let mut var_uc_subld1_rv: f64 = *var_uc_subld1_rv_slot;
        let mut var_vg2const: f64 = *var_vg2const_slot;
        let mut var_vg2const_1: f64 = *var_vg2const_1_slot;
        let mut var_vg2const_1_dn0: f64 = *var_vg2const_1_dn0_slot;
        let mut var_vg2const_1_dn10: f64 = *var_vg2const_1_dn10_slot;
        let mut var_vg2const_1_dn11: f64 = *var_vg2const_1_dn11_slot;
        let mut var_vg2const_1_dn14: f64 = *var_vg2const_1_dn14_slot;
        let mut var_vg2const_1_dn2: f64 = *var_vg2const_1_dn2_slot;
        let mut var_vg2const_1_dn4: f64 = *var_vg2const_1_dn4_slot;
        let mut var_vg2const_1_dn5: f64 = *var_vg2const_1_dn5_slot;
        let mut var_vg2const_1_dn6: f64 = *var_vg2const_1_dn6_slot;
        let mut var_vg2const_1_dn7: f64 = *var_vg2const_1_dn7_slot;
        let mut var_vg2const_1_dn8: f64 = *var_vg2const_1_dn8_slot;
        let mut var_vg2const_1_dn9: f64 = *var_vg2const_1_dn9_slot;
        let mut var_vg2const_1_rv: f64 = *var_vg2const_1_rv_slot;
        let mut var_vg2const_dn0: f64 = *var_vg2const_dn0_slot;
        let mut var_vg2const_dn10: f64 = *var_vg2const_dn10_slot;
        let mut var_vg2const_dn11: f64 = *var_vg2const_dn11_slot;
        let mut var_vg2const_dn14: f64 = *var_vg2const_dn14_slot;
        let mut var_vg2const_dn2: f64 = *var_vg2const_dn2_slot;
        let mut var_vg2const_dn4: f64 = *var_vg2const_dn4_slot;
        let mut var_vg2const_dn5: f64 = *var_vg2const_dn5_slot;
        let mut var_vg2const_dn6: f64 = *var_vg2const_dn6_slot;
        let mut var_vg2const_dn7: f64 = *var_vg2const_dn7_slot;
        let mut var_vg2const_dn8: f64 = *var_vg2const_dn8_slot;
        let mut var_vg2const_dn9: f64 = *var_vg2const_dn9_slot;
        let mut var_vg2const_rv: f64 = *var_vg2const_rv_slot;
        let mut var_xgate: f64 = *var_xgate_slot;
        let mut var_xgate_1: f64 = *var_xgate_1_slot;
        let mut var_xgate_1_rv: f64 = *var_xgate_1_rv_slot;
        let mut var_xgate_rv: f64 = *var_xgate_rv_slot;
        let mut var_xsub1: f64 = *var_xsub1_slot;
        let mut var_xsub1_1: f64 = *var_xsub1_1_slot;
        let mut var_xsub1_1_rv: f64 = *var_xsub1_1_rv_slot;
        let mut var_xsub1_rv: f64 = *var_xsub1_rv_slot;
        let mut var_xsub2: f64 = *var_xsub2_slot;
        let mut var_xsub2_1: f64 = *var_xsub2_1_slot;
        let mut var_xsub2_1_rv: f64 = *var_xsub2_1_rv_slot;
        let mut var_xsub2_rv: f64 = *var_xsub2_rv_slot;
        let mut var_xvbs: f64 = *var_xvbs_slot;
        let mut var_xvbs_1: f64 = *var_xvbs_1_slot;
        let mut var_xvbs_1_rv: f64 = *var_xvbs_1_rv_slot;
        let mut var_xvbs_rv: f64 = *var_xvbs_rv_slot;

        let (assign12570_e7001, assign12570_e7001_d_n0, assign12570_e7001_d_n2, assign12570_e7001_d_n4, assign12570_e7001_d_n5, assign12570_e7001_d_n6, assign12570_e7001_d_n7, assign12570_e7001_d_n8, assign12570_e7001_d_n9, assign12570_e7001_d_n10, assign12570_e7001_d_n11, assign12570_e7001_d_n14,) = {
    if (var_guard286 == 0.0) {
        let assign12570_e6991: f64 = (var_t1 * p.p107);
        let assign12570_e6994: f64 = (var_t1 + p.p107);
        let assign12570_e6995: f64 = (assign12570_e6991 / assign12570_e6994);
        let assign12570_e6997: f64 = (assign12570_e6995 + p.p109);
        let assign12570_e6999: f64 = (assign12570_e6997 + 1e-25);
        (assign12570_e6999, ((((var_t1_dn0 * p.p107) * assign12570_e6994) - (assign12570_e6991 * var_t1_dn0)) / (assign12570_e6994 * assign12570_e6994)), ((((var_t1_dn2 * p.p107) * assign12570_e6994) - (assign12570_e6991 * var_t1_dn2)) / (assign12570_e6994 * assign12570_e6994)), ((((var_t1_dn4 * p.p107) * assign12570_e6994) - (assign12570_e6991 * var_t1_dn4)) / (assign12570_e6994 * assign12570_e6994)), ((((var_t1_dn5 * p.p107) * assign12570_e6994) - (assign12570_e6991 * var_t1_dn5)) / (assign12570_e6994 * assign12570_e6994)), ((((var_t1_dn6 * p.p107) * assign12570_e6994) - (assign12570_e6991 * var_t1_dn6)) / (assign12570_e6994 * assign12570_e6994)), ((((var_t1_dn7 * p.p107) * assign12570_e6994) - (assign12570_e6991 * var_t1_dn7)) / (assign12570_e6994 * assign12570_e6994)), ((((var_t1_dn8 * p.p107) * assign12570_e6994) - (assign12570_e6991 * var_t1_dn8)) / (assign12570_e6994 * assign12570_e6994)), ((((var_t1_dn9 * p.p107) * assign12570_e6994) - (assign12570_e6991 * var_t1_dn9)) / (assign12570_e6994 * assign12570_e6994)), ((((var_t1_dn10 * p.p107) * assign12570_e6994) - (assign12570_e6991 * var_t1_dn10)) / (assign12570_e6994 * assign12570_e6994)), ((((var_t1_dn11 * p.p107) * assign12570_e6994) - (assign12570_e6991 * var_t1_dn11)) / (assign12570_e6994 * assign12570_e6994)), ((((var_t1_dn14 * p.p107) * assign12570_e6994) - (assign12570_e6991 * var_t1_dn14)) / (assign12570_e6994 * assign12570_e6994)),)
    } else {
        (var_ddlte, var_ddlte_dn0, var_ddlte_dn2, var_ddlte_dn4, var_ddlte_dn5, var_ddlte_dn6, var_ddlte_dn7, var_ddlte_dn8, var_ddlte_dn9, var_ddlte_dn10, var_ddlte_dn11, var_ddlte_dn14,)
    }
};
        var_ddlte = assign12570_e7001;
        var_ddlte_dn0 = assign12570_e7001_d_n0;
        var_ddlte_dn2 = assign12570_e7001_d_n2;
        var_ddlte_dn4 = assign12570_e7001_d_n4;
        var_ddlte_dn5 = assign12570_e7001_d_n5;
        var_ddlte_dn6 = assign12570_e7001_d_n6;
        var_ddlte_dn7 = assign12570_e7001_d_n7;
        var_ddlte_dn8 = assign12570_e7001_d_n8;
        var_ddlte_dn9 = assign12570_e7001_d_n9;
        var_ddlte_dn10 = assign12570_e7001_d_n10;
        var_ddlte_dn11 = assign12570_e7001_d_n11;
        var_ddlte_dn14 = assign12570_e7001_d_n14;
        var_ddlte_rv = 0.0;

        let assign12590_e7009: f64 = if var_ddlte < 0.1 { 1.0 } else { 0.0 };
        var_guard290 = assign12590_e7009;
        var_guard290_rv = 0.0;

        let (assign12600_e7013, assign12600_e7013_d_n0, assign12600_e7013_d_n2, assign12600_e7013_d_n4, assign12600_e7013_d_n5, assign12600_e7013_d_n6, assign12600_e7013_d_n7, assign12600_e7013_d_n8, assign12600_e7013_d_n9, assign12600_e7013_d_n10, assign12600_e7013_d_n11, assign12600_e7013_d_n14,) = {
    if (var_guard290 != 0.0) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ddlte, var_ddlte_dn0, var_ddlte_dn2, var_ddlte_dn4, var_ddlte_dn5, var_ddlte_dn6, var_ddlte_dn7, var_ddlte_dn8, var_ddlte_dn9, var_ddlte_dn10, var_ddlte_dn11, var_ddlte_dn14,)
    }
};
        var_ddlte = assign12600_e7013;
        var_ddlte_dn0 = assign12600_e7013_d_n0;
        var_ddlte_dn2 = assign12600_e7013_d_n2;
        var_ddlte_dn4 = assign12600_e7013_d_n4;
        var_ddlte_dn5 = assign12600_e7013_d_n5;
        var_ddlte_dn6 = assign12600_e7013_d_n6;
        var_ddlte_dn7 = assign12600_e7013_d_n7;
        var_ddlte_dn8 = assign12600_e7013_d_n8;
        var_ddlte_dn9 = assign12600_e7013_d_n9;
        var_ddlte_dn10 = assign12600_e7013_d_n10;
        var_ddlte_dn11 = assign12600_e7013_d_n11;
        var_ddlte_dn14 = assign12600_e7013_d_n14;
        var_ddlte_rv = 0.0;

        let (assign12610_e7019, assign12610_e7019_d_n0, assign12610_e7019_d_n2, assign12610_e7019_d_n4, assign12610_e7019_d_n5, assign12610_e7019_d_n6, assign12610_e7019_d_n7, assign12610_e7019_d_n8, assign12610_e7019_d_n9, assign12610_e7019_d_n10, assign12610_e7019_d_n11, assign12610_e7019_d_n14,) = {
    if (p.p23 != 0.0) {
        let assign12610_e7017: f64 = (var_weff).powf(p.p201);
        (assign12610_e7017, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    }
};
        var_t2 = assign12610_e7019;
        var_t2_dn0 = assign12610_e7019_d_n0;
        var_t2_dn2 = assign12610_e7019_d_n2;
        var_t2_dn4 = assign12610_e7019_d_n4;
        var_t2_dn5 = assign12610_e7019_d_n5;
        var_t2_dn6 = assign12610_e7019_d_n6;
        var_t2_dn7 = assign12610_e7019_d_n7;
        var_t2_dn8 = assign12610_e7019_d_n8;
        var_t2_dn9 = assign12610_e7019_d_n9;
        var_t2_dn10 = assign12610_e7019_d_n10;
        var_t2_dn11 = assign12610_e7019_d_n11;
        var_t2_dn14 = assign12610_e7019_d_n14;
        var_t2_rv = 0.0;

        let (assign12620_e7037, assign12620_e7037_d_n0, assign12620_e7037_d_n2, assign12620_e7037_d_n4, assign12620_e7037_d_n5, assign12620_e7037_d_n6, assign12620_e7037_d_n7, assign12620_e7037_d_n8, assign12620_e7037_d_n9, assign12620_e7037_d_n10, assign12620_e7037_d_n11, assign12620_e7037_d_n14,) = {
    if (p.p23 != 0.0) {
        let assign12620_e7026: f64 = (var_lgate).powf(p.p199);
        let assign12620_e7027: f64 = (var_mks_svgsl / assign12620_e7026);
        let assign12620_e7028: f64 = (1.0 + assign12620_e7027);
        let assign12620_e7029: f64 = (var_uc_svgs * assign12620_e7028);
        let assign12620_e7033: f64 = (var_t2 + var_mks_svgsw);
        let assign12620_e7034: f64 = (var_t2 / assign12620_e7033);
        let assign12620_e7035: f64 = (assign12620_e7029 * assign12620_e7034);
        (assign12620_e7035, (assign12620_e7029 * (((var_t2_dn0 * assign12620_e7033) - (var_t2 * var_t2_dn0)) / (assign12620_e7033 * assign12620_e7033))), (assign12620_e7029 * (((var_t2_dn2 * assign12620_e7033) - (var_t2 * var_t2_dn2)) / (assign12620_e7033 * assign12620_e7033))), (assign12620_e7029 * (((var_t2_dn4 * assign12620_e7033) - (var_t2 * var_t2_dn4)) / (assign12620_e7033 * assign12620_e7033))), (assign12620_e7029 * (((var_t2_dn5 * assign12620_e7033) - (var_t2 * var_t2_dn5)) / (assign12620_e7033 * assign12620_e7033))), (assign12620_e7029 * (((var_t2_dn6 * assign12620_e7033) - (var_t2 * var_t2_dn6)) / (assign12620_e7033 * assign12620_e7033))), (assign12620_e7029 * (((var_t2_dn7 * assign12620_e7033) - (var_t2 * var_t2_dn7)) / (assign12620_e7033 * assign12620_e7033))), (assign12620_e7029 * (((var_t2_dn8 * assign12620_e7033) - (var_t2 * var_t2_dn8)) / (assign12620_e7033 * assign12620_e7033))), (assign12620_e7029 * (((var_t2_dn9 * assign12620_e7033) - (var_t2 * var_t2_dn9)) / (assign12620_e7033 * assign12620_e7033))), (assign12620_e7029 * (((var_t2_dn10 * assign12620_e7033) - (var_t2 * var_t2_dn10)) / (assign12620_e7033 * assign12620_e7033))), (assign12620_e7029 * (((var_t2_dn11 * assign12620_e7033) - (var_t2 * var_t2_dn11)) / (assign12620_e7033 * assign12620_e7033))), (assign12620_e7029 * (((var_t2_dn14 * assign12620_e7033) - (var_t2 * var_t2_dn14)) / (assign12620_e7033 * assign12620_e7033))),)
    } else {
        (var_vg2const, var_vg2const_dn0, var_vg2const_dn2, var_vg2const_dn4, var_vg2const_dn5, var_vg2const_dn6, var_vg2const_dn7, var_vg2const_dn8, var_vg2const_dn9, var_vg2const_dn10, var_vg2const_dn11, var_vg2const_dn14,)
    }
};
        var_vg2const = assign12620_e7037;
        var_vg2const_dn0 = assign12620_e7037_d_n0;
        var_vg2const_dn2 = assign12620_e7037_d_n2;
        var_vg2const_dn4 = assign12620_e7037_d_n4;
        var_vg2const_dn5 = assign12620_e7037_d_n5;
        var_vg2const_dn6 = assign12620_e7037_d_n6;
        var_vg2const_dn7 = assign12620_e7037_d_n7;
        var_vg2const_dn8 = assign12620_e7037_d_n8;
        var_vg2const_dn9 = assign12620_e7037_d_n9;
        var_vg2const_dn10 = assign12620_e7037_d_n10;
        var_vg2const_dn11 = assign12620_e7037_d_n11;
        var_vg2const_dn14 = assign12620_e7037_d_n14;
        var_vg2const_rv = 0.0;

        let (assign12630_e7049,) = {
    if (p.p23 != 0.0) {
        let assign12630_e7044: f64 = (var_lgate).powf(p.p184);
        let assign12630_e7045: f64 = (var_mks_svbsl / assign12630_e7044);
        let assign12630_e7046: f64 = (1.0 + assign12630_e7045);
        let assign12630_e7047: f64 = (var_uc_svbs * assign12630_e7046);
        (assign12630_e7047,)
    } else {
        (var_xvbs,)
    }
};
        var_xvbs = assign12630_e7049;
        var_xvbs_rv = 0.0;

        let (assign12640_e7061,) = {
    if (p.p23 != 0.0) {
        let assign12640_e7056: f64 = (var_lgate).powf(p.p203);
        let assign12640_e7057: f64 = (var_mks_slgl / assign12640_e7056);
        let assign12640_e7058: f64 = (1.0 + assign12640_e7057);
        let assign12640_e7059: f64 = (var_mks_slg * assign12640_e7058);
        (assign12640_e7059,)
    } else {
        (var_xgate,)
    }
};
        var_xgate = assign12640_e7061;
        var_xgate_rv = 0.0;

        let (assign12650_e7073,) = {
    if (p.p23 != 0.0) {
        let assign12650_e7068: f64 = (var_lgate).powf(p.p191);
        let assign12650_e7069: f64 = (var_mks_sub1l / assign12650_e7068);
        let assign12650_e7070: f64 = (1.0 + assign12650_e7069);
        let assign12650_e7071: f64 = (var_uc_sub1 * assign12650_e7070);
        (assign12650_e7071,)
    } else {
        (var_xsub1,)
    }
};
        var_xsub1 = assign12650_e7073;
        var_xsub1_rv = 0.0;

        let (assign12660_e7083,) = {
    if (p.p23 != 0.0) {
        let assign12660_e7079: f64 = (var_mks_sub2l / var_lgate);
        let assign12660_e7080: f64 = (1.0 + assign12660_e7079);
        let assign12660_e7081: f64 = (var_uc_sub2 * assign12660_e7080);
        (assign12660_e7081,)
    } else {
        (var_xsub2,)
    }
};
        var_xsub2 = assign12660_e7083;
        var_xsub2_rv = 0.0;

        let (assign12670_e7087,) = {
    if (p.p23 != 0.0) {
        (var_xsub1,)
    } else {
        (var_xsub1_1,)
    }
};
        var_xsub1_1 = assign12670_e7087;
        var_xsub1_1_rv = 0.0;

        let (assign12680_e7091,) = {
    if (p.p23 != 0.0) {
        (var_xsub2,)
    } else {
        (var_xsub2_1,)
    }
};
        var_xsub2_1 = assign12680_e7091;
        var_xsub2_1_rv = 0.0;

        let (assign12690_e7095, assign12690_e7095_d_n0, assign12690_e7095_d_n2, assign12690_e7095_d_n4, assign12690_e7095_d_n5, assign12690_e7095_d_n6, assign12690_e7095_d_n7, assign12690_e7095_d_n8, assign12690_e7095_d_n9, assign12690_e7095_d_n10, assign12690_e7095_d_n11, assign12690_e7095_d_n14,) = {
    if (p.p23 != 0.0) {
        (var_vg2const, var_vg2const_dn0, var_vg2const_dn2, var_vg2const_dn4, var_vg2const_dn5, var_vg2const_dn6, var_vg2const_dn7, var_vg2const_dn8, var_vg2const_dn9, var_vg2const_dn10, var_vg2const_dn11, var_vg2const_dn14,)
    } else {
        (var_vg2const_1, var_vg2const_1_dn0, var_vg2const_1_dn2, var_vg2const_1_dn4, var_vg2const_1_dn5, var_vg2const_1_dn6, var_vg2const_1_dn7, var_vg2const_1_dn8, var_vg2const_1_dn9, var_vg2const_1_dn10, var_vg2const_1_dn11, var_vg2const_1_dn14,)
    }
};
        var_vg2const_1 = assign12690_e7095;
        var_vg2const_1_dn0 = assign12690_e7095_d_n0;
        var_vg2const_1_dn2 = assign12690_e7095_d_n2;
        var_vg2const_1_dn4 = assign12690_e7095_d_n4;
        var_vg2const_1_dn5 = assign12690_e7095_d_n5;
        var_vg2const_1_dn6 = assign12690_e7095_d_n6;
        var_vg2const_1_dn7 = assign12690_e7095_d_n7;
        var_vg2const_1_dn8 = assign12690_e7095_d_n8;
        var_vg2const_1_dn9 = assign12690_e7095_d_n9;
        var_vg2const_1_dn10 = assign12690_e7095_d_n10;
        var_vg2const_1_dn11 = assign12690_e7095_d_n11;
        var_vg2const_1_dn14 = assign12690_e7095_d_n14;
        var_vg2const_1_rv = 0.0;

        let (assign12700_e7099,) = {
    if (p.p23 != 0.0) {
        (var_xvbs,)
    } else {
        (var_xvbs_1,)
    }
};
        var_xvbs_1 = assign12700_e7099;
        var_xvbs_1_rv = 0.0;

        let (assign12710_e7103,) = {
    if (p.p23 != 0.0) {
        (var_xgate,)
    } else {
        (var_xgate_1,)
    }
};
        var_xgate_1 = assign12710_e7103;
        var_xgate_1_rv = 0.0;

        let (assign12720_e7117,) = {
    if ((p.p23 != 0.0) && (p.p46 != 0.0)) {
        let assign12720_e7112: f64 = (var_lgate).powf(p.p191);
        let assign12720_e7113: f64 = (var_mks_sub1l / assign12720_e7112);
        let assign12720_e7114: f64 = (1.0 + assign12720_e7113);
        let assign12720_e7115: f64 = (var_uc_sub1snp * assign12720_e7114);
        (assign12720_e7115,)
    } else {
        (var_xsub1_1,)
    }
};
        var_xsub1_1 = assign12720_e7117;
        var_xsub1_1_rv = 0.0;

        let (assign12730_e7129,) = {
    if ((p.p23 != 0.0) && (p.p46 != 0.0)) {
        let assign12730_e7125: f64 = (var_mks_sub2l / var_lgate);
        let assign12730_e7126: f64 = (1.0 + assign12730_e7125);
        let assign12730_e7127: f64 = (var_uc_sub2snp * assign12730_e7126);
        (assign12730_e7127,)
    } else {
        (var_xsub2_1,)
    }
};
        var_xsub2_1 = assign12730_e7129;
        var_xsub2_1_rv = 0.0;

        let (assign12740_e7141,) = {
    if (p.p23 != 0.0) {
        let assign12740_e7136: f64 = (var_lg).powf(p.p103);
        let assign12740_e7137: f64 = (p.p102 / assign12740_e7136);
        let assign12740_e7138: f64 = (1.0 + assign12740_e7137);
        let assign12740_e7139: f64 = (p.p72 * assign12740_e7138);
        (assign12740_e7139,)
    } else {
        (var_uc_subld1,)
    }
};
        var_uc_subld1 = assign12740_e7141;
        var_uc_subld1_rv = 0.0;

        let (assign12750_e7146, assign12750_e7146_d_n0, assign12750_e7146_d_n2, assign12750_e7146_d_n4, assign12750_e7146_d_n5, assign12750_e7146_d_n6, assign12750_e7146_d_n7, assign12750_e7146_d_n8, assign12750_e7146_d_n9, assign12750_e7146_d_n10, assign12750_e7146_d_n11, assign12750_e7146_d_n14,) = {
    if (p.p23 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vg2const, var_vg2const_dn0, var_vg2const_dn2, var_vg2const_dn4, var_vg2const_dn5, var_vg2const_dn6, var_vg2const_dn7, var_vg2const_dn8, var_vg2const_dn9, var_vg2const_dn10, var_vg2const_dn11, var_vg2const_dn14,)
    }
};
        var_vg2const = assign12750_e7146;
        var_vg2const_dn0 = assign12750_e7146_d_n0;
        var_vg2const_dn2 = assign12750_e7146_d_n2;
        var_vg2const_dn4 = assign12750_e7146_d_n4;
        var_vg2const_dn5 = assign12750_e7146_d_n5;
        var_vg2const_dn6 = assign12750_e7146_d_n6;
        var_vg2const_dn7 = assign12750_e7146_d_n7;
        var_vg2const_dn8 = assign12750_e7146_d_n8;
        var_vg2const_dn9 = assign12750_e7146_d_n9;
        var_vg2const_dn10 = assign12750_e7146_d_n10;
        var_vg2const_dn11 = assign12750_e7146_d_n11;
        var_vg2const_dn14 = assign12750_e7146_d_n14;
        var_vg2const_rv = 0.0;

        let (assign12760_e7151,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (var_xvbs,)
    }
};
        var_xvbs = assign12760_e7151;
        var_xvbs_rv = 0.0;

        let (assign12770_e7156,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (var_xgate,)
    }
};
        var_xgate = assign12770_e7156;
        var_xgate_rv = 0.0;

        let (assign12780_e7161,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (var_xsub1,)
    }
};
        var_xsub1 = assign12780_e7161;
        var_xsub1_rv = 0.0;

        let (assign12790_e7166,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (var_xsub2,)
    }
};
        var_xsub2 = assign12790_e7166;
        var_xsub2_rv = 0.0;

        let (assign12800_e7171,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (var_uc_subld1,)
    }
};
        var_uc_subld1 = assign12800_e7171;
        var_uc_subld1_rv = 0.0;

        let (assign12810_e7176, assign12810_e7176_d_n0, assign12810_e7176_d_n2, assign12810_e7176_d_n4, assign12810_e7176_d_n5, assign12810_e7176_d_n6, assign12810_e7176_d_n7, assign12810_e7176_d_n8, assign12810_e7176_d_n9, assign12810_e7176_d_n10, assign12810_e7176_d_n11, assign12810_e7176_d_n14,) = {
    if (p.p23 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vg2const_1, var_vg2const_1_dn0, var_vg2const_1_dn2, var_vg2const_1_dn4, var_vg2const_1_dn5, var_vg2const_1_dn6, var_vg2const_1_dn7, var_vg2const_1_dn8, var_vg2const_1_dn9, var_vg2const_1_dn10, var_vg2const_1_dn11, var_vg2const_1_dn14,)
    }
};
        var_vg2const_1 = assign12810_e7176;
        var_vg2const_1_dn0 = assign12810_e7176_d_n0;
        var_vg2const_1_dn2 = assign12810_e7176_d_n2;
        var_vg2const_1_dn4 = assign12810_e7176_d_n4;
        var_vg2const_1_dn5 = assign12810_e7176_d_n5;
        var_vg2const_1_dn6 = assign12810_e7176_d_n6;
        var_vg2const_1_dn7 = assign12810_e7176_d_n7;
        var_vg2const_1_dn8 = assign12810_e7176_d_n8;
        var_vg2const_1_dn9 = assign12810_e7176_d_n9;
        var_vg2const_1_dn10 = assign12810_e7176_d_n10;
        var_vg2const_1_dn11 = assign12810_e7176_d_n11;
        var_vg2const_1_dn14 = assign12810_e7176_d_n14;
        var_vg2const_1_rv = 0.0;

        let (assign12820_e7181,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (var_xvbs_1,)
    }
};
        var_xvbs_1 = assign12820_e7181;
        var_xvbs_1_rv = 0.0;

        let (assign12830_e7186,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (var_xgate_1,)
    }
};
        var_xgate_1 = assign12830_e7186;
        var_xgate_1_rv = 0.0;

        let (assign12840_e7191,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (var_xsub1_1,)
    }
};
        var_xsub1_1 = assign12840_e7191;
        var_xsub1_1_rv = 0.0;

        let (assign12850_e7196,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (var_xsub2_1,)
    }
};
        var_xsub2_1 = assign12850_e7196;
        var_xsub2_1_rv = 0.0;

        let (assign12860_e7210,) = {
    if (var_uc_ibpc1 != 0.0) {
        let assign12860_e7205: f64 = (var_lg).powf(p.p280);
        let assign12860_e7206: f64 = (p.p279 / assign12860_e7205);
        let assign12860_e7207: f64 = (1.0 + assign12860_e7206);
        let assign12860_e7208: f64 = (var_uc_ibpc1 * assign12860_e7207);
        (assign12860_e7208,)
    } else {
        (0.0,)
    }
};
        var_uc_ibpc1 = assign12860_e7210;
        var_uc_ibpc1_rv = 0.0;

        let assign12870_e7214: f64 = (3.141592653589793 / 2.0);
        let assign12870_e7215: f64 = (3.453133e-11 / assign12870_e7214);
        let assign12870_e7217: f64 = (assign12870_e7215 * var_weffcv_nf);
        let assign12870_e7221: f64 = (p.p225 / p.p95);
        let assign12870_e7222: f64 = (1.0 + assign12870_e7221);
        let assign12870_e7223: f64 = (assign12870_e7222).ln();
        let assign12870_e7224: f64 = (assign12870_e7217 * assign12870_e7223);
        var_cfrng = assign12870_e7224;
        var_cfrng_rv = 0.0;

        let (assign12880_e7238,) = {
    if (p.p134 != 0.0) {
        let assign12880_e7230: f64 = (1000000.0 * var_weffcv_nf);
        let assign12880_e7232: f64 = (assign12880_e7230 * p.p134);
        let assign12880_e7235: f64 = (var_lg).powf(p.p135);
        let assign12880_e7236: f64 = (assign12880_e7232 / assign12880_e7235);
        (assign12880_e7236,)
    } else {
        (0.0,)
    }
};
        var_cqyb0 = assign12880_e7238;
        var_cqyb0_rv = 0.0;

        let assign12890_e7242: f64 = (-p.p286);
        let assign12890_e7243: f64 = (var_lg).powf(assign12890_e7242);
        let assign12890_e7244: f64 = (p.p283 * assign12890_e7243);
        var_ptl0 = assign12890_e7244;
        var_ptl0_rv = 0.0;

        let assign12900_e7248: f64 = (-p.p291);
        let assign12900_e7249: f64 = (var_lg).powf(assign12900_e7248);
        let assign12900_e7250: f64 = (p.p290 * assign12900_e7249);
        var_pt40 = assign12900_e7250;
        var_pt40_rv = 0.0;

        let assign12910_e7254: f64 = (var_lg + var_uc_gdld);
        let assign12910_e7256: f64 = (-p.p288);
        let assign12910_e7257: f64 = (assign12910_e7254).powf(assign12910_e7256);
        let assign12910_e7258: f64 = (p.p287 * assign12910_e7257);
        var_gdl0 = assign12910_e7258;
        var_gdl0_rv = 0.0;

        let assign12920_e7262: f64 = (var_mfactor * var_weff_nf);
        let assign12920_e7263: f64 = (var_uc_rth0 / assign12920_e7262);
        let assign12920_e7268: f64 = (var_lg).powf(p.p318);
        let assign12920_e7269: f64 = (p.p317 / assign12920_e7268);
        let assign12920_e7270: f64 = (1.0 + assign12920_e7269);
        let assign12920_e7271: f64 = (assign12920_e7263 * assign12920_e7270);
        let assign12920_e7276: f64 = (var_wg).powf(p.p316);
        let assign12920_e7277: f64 = (p.p315 / assign12920_e7276);
        let assign12920_e7278: f64 = (1.0 + assign12920_e7277);
        let assign12920_e7279: f64 = (assign12920_e7271 * assign12920_e7278);
        var_rth = assign12920_e7279;
        var_rth_dn0 = 0.0;
        var_rth_dn2 = 0.0;
        var_rth_dn4 = 0.0;
        var_rth_dn5 = 0.0;
        var_rth_dn6 = 0.0;
        var_rth_dn7 = 0.0;
        var_rth_dn8 = 0.0;
        var_rth_dn9 = 0.0;
        var_rth_dn10 = 0.0;
        var_rth_dn11 = 0.0;
        var_rth_dn14 = 0.0;
        var_rth_rv = 0.0;

        let assign12940_e7289: f64 = (p.p7).powf(p.p327);
        let assign12940_e7290: f64 = (1.0 / assign12940_e7289);
        let assign12940_e7291: f64 = (var_rth * assign12940_e7290);
        var_rth = assign12940_e7291;
        var_rth_dn0 = (var_rth_dn0 * assign12940_e7290);
        var_rth_dn2 = (var_rth_dn2 * assign12940_e7290);
        var_rth_dn4 = (var_rth_dn4 * assign12940_e7290);
        var_rth_dn5 = (var_rth_dn5 * assign12940_e7290);
        var_rth_dn6 = (var_rth_dn6 * assign12940_e7290);
        var_rth_dn7 = (var_rth_dn7 * assign12940_e7290);
        var_rth_dn8 = (var_rth_dn8 * assign12940_e7290);
        var_rth_dn9 = (var_rth_dn9 * assign12940_e7290);
        var_rth_dn10 = (var_rth_dn10 * assign12940_e7290);
        var_rth_dn11 = (var_rth_dn11 * assign12940_e7290);
        var_rth_dn14 = (var_rth_dn14 * assign12940_e7290);
        var_rth_rv = 0.0;

        let assign12950_e7295: f64 = (p.p7).powf(p.p327);
        let assign12950_e7296: f64 = (1.0 / assign12950_e7295);
        let assign12950_e7299: f64 = (var_mfactor * var_weff_nf);
        let assign12950_e7300: f64 = (assign12950_e7296 / assign12950_e7299);
        let assign12950_e7305: f64 = (var_lg).powf(p.p318);
        let assign12950_e7306: f64 = (p.p317 / assign12950_e7305);
        let assign12950_e7307: f64 = (1.0 + assign12950_e7306);
        let assign12950_e7308: f64 = (assign12950_e7300 * assign12950_e7307);
        let assign12950_e7313: f64 = (var_wg).powf(p.p316);
        let assign12950_e7314: f64 = (p.p315 / assign12950_e7313);
        let assign12950_e7315: f64 = (1.0 + assign12950_e7314);
        let assign12950_e7316: f64 = (assign12950_e7308 * assign12950_e7315);
        var_rthtemp0 = assign12950_e7316;
        var_rthtemp0_rv = 0.0;

        let assign12960_e7323: f64 = if ((p.p53 == 0.0) || (var_uc_rth0 == 0.0)) { 1.0 } else { 0.0 };
        var_guard291 = assign12960_e7323;
        var_guard291_rv = 0.0;

        *var_cfrng_slot = var_cfrng;
        *var_cfrng_rv_slot = var_cfrng_rv;
        *var_cqyb0_slot = var_cqyb0;
        *var_cqyb0_rv_slot = var_cqyb0_rv;
        *var_ddlte_slot = var_ddlte;
        *var_ddlte_dn0_slot = var_ddlte_dn0;
        *var_ddlte_dn10_slot = var_ddlte_dn10;
        *var_ddlte_dn11_slot = var_ddlte_dn11;
        *var_ddlte_dn14_slot = var_ddlte_dn14;
        *var_ddlte_dn2_slot = var_ddlte_dn2;
        *var_ddlte_dn4_slot = var_ddlte_dn4;
        *var_ddlte_dn5_slot = var_ddlte_dn5;
        *var_ddlte_dn6_slot = var_ddlte_dn6;
        *var_ddlte_dn7_slot = var_ddlte_dn7;
        *var_ddlte_dn8_slot = var_ddlte_dn8;
        *var_ddlte_dn9_slot = var_ddlte_dn9;
        *var_ddlte_rv_slot = var_ddlte_rv;
        *var_gdl0_slot = var_gdl0;
        *var_gdl0_rv_slot = var_gdl0_rv;
        *var_guard290_slot = var_guard290;
        *var_guard290_rv_slot = var_guard290_rv;
        *var_guard291_slot = var_guard291;
        *var_guard291_rv_slot = var_guard291_rv;
        *var_pt40_slot = var_pt40;
        *var_pt40_rv_slot = var_pt40_rv;
        *var_ptl0_slot = var_ptl0;
        *var_ptl0_rv_slot = var_ptl0_rv;
        *var_rth_slot = var_rth;
        *var_rth_dn0_slot = var_rth_dn0;
        *var_rth_dn10_slot = var_rth_dn10;
        *var_rth_dn11_slot = var_rth_dn11;
        *var_rth_dn14_slot = var_rth_dn14;
        *var_rth_dn2_slot = var_rth_dn2;
        *var_rth_dn4_slot = var_rth_dn4;
        *var_rth_dn5_slot = var_rth_dn5;
        *var_rth_dn6_slot = var_rth_dn6;
        *var_rth_dn7_slot = var_rth_dn7;
        *var_rth_dn8_slot = var_rth_dn8;
        *var_rth_dn9_slot = var_rth_dn9;
        *var_rth_rv_slot = var_rth_rv;
        *var_rthtemp0_slot = var_rthtemp0;
        *var_rthtemp0_rv_slot = var_rthtemp0_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t2_rv_slot = var_t2_rv;
        *var_uc_ibpc1_slot = var_uc_ibpc1;
        *var_uc_ibpc1_rv_slot = var_uc_ibpc1_rv;
        *var_uc_subld1_slot = var_uc_subld1;
        *var_uc_subld1_rv_slot = var_uc_subld1_rv;
        *var_vg2const_slot = var_vg2const;
        *var_vg2const_1_slot = var_vg2const_1;
        *var_vg2const_1_dn0_slot = var_vg2const_1_dn0;
        *var_vg2const_1_dn10_slot = var_vg2const_1_dn10;
        *var_vg2const_1_dn11_slot = var_vg2const_1_dn11;
        *var_vg2const_1_dn14_slot = var_vg2const_1_dn14;
        *var_vg2const_1_dn2_slot = var_vg2const_1_dn2;
        *var_vg2const_1_dn4_slot = var_vg2const_1_dn4;
        *var_vg2const_1_dn5_slot = var_vg2const_1_dn5;
        *var_vg2const_1_dn6_slot = var_vg2const_1_dn6;
        *var_vg2const_1_dn7_slot = var_vg2const_1_dn7;
        *var_vg2const_1_dn8_slot = var_vg2const_1_dn8;
        *var_vg2const_1_dn9_slot = var_vg2const_1_dn9;
        *var_vg2const_1_rv_slot = var_vg2const_1_rv;
        *var_vg2const_dn0_slot = var_vg2const_dn0;
        *var_vg2const_dn10_slot = var_vg2const_dn10;
        *var_vg2const_dn11_slot = var_vg2const_dn11;
        *var_vg2const_dn14_slot = var_vg2const_dn14;
        *var_vg2const_dn2_slot = var_vg2const_dn2;
        *var_vg2const_dn4_slot = var_vg2const_dn4;
        *var_vg2const_dn5_slot = var_vg2const_dn5;
        *var_vg2const_dn6_slot = var_vg2const_dn6;
        *var_vg2const_dn7_slot = var_vg2const_dn7;
        *var_vg2const_dn8_slot = var_vg2const_dn8;
        *var_vg2const_dn9_slot = var_vg2const_dn9;
        *var_vg2const_rv_slot = var_vg2const_rv;
        *var_xgate_slot = var_xgate;
        *var_xgate_1_slot = var_xgate_1;
        *var_xgate_1_rv_slot = var_xgate_1_rv;
        *var_xgate_rv_slot = var_xgate_rv;
        *var_xsub1_slot = var_xsub1;
        *var_xsub1_1_slot = var_xsub1_1;
        *var_xsub1_1_rv_slot = var_xsub1_1_rv;
        *var_xsub1_rv_slot = var_xsub1_rv;
        *var_xsub2_slot = var_xsub2;
        *var_xsub2_1_slot = var_xsub2_1;
        *var_xsub2_1_rv_slot = var_xsub2_1_rv;
        *var_xsub2_rv_slot = var_xsub2_rv;
        *var_xvbs_slot = var_xvbs;
        *var_xvbs_1_slot = var_xvbs_1;
        *var_xvbs_1_rv_slot = var_xvbs_1_rv;
        *var_xvbs_rv_slot = var_xvbs_rv;
    }

    pub(super) fn stamp_reactive_block_23(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        var_deltemp: f64,
        var_deltemp_dn0: f64,
        var_deltemp_dn10: f64,
        var_deltemp_dn11: f64,
        var_deltemp_dn14: f64,
        var_deltemp_dn2: f64,
        var_deltemp_dn4: f64,
        var_deltemp_dn5: f64,
        var_deltemp_dn6: f64,
        var_deltemp_dn7: f64,
        var_deltemp_dn8: f64,
        var_deltemp_dn9: f64,
        var_egtnom: f64,
        var_guard291: f64,
        var_ktnom: f64,
        var_uc_bgtmp1: f64,
        var_uc_bgtmp2: f64,
        var_beta_slot: &mut f64,
        var_beta2_slot: &mut f64,
        var_beta2_dn0_slot: &mut f64,
        var_beta2_dn10_slot: &mut f64,
        var_beta2_dn11_slot: &mut f64,
        var_beta2_dn14_slot: &mut f64,
        var_beta2_dn2_slot: &mut f64,
        var_beta2_dn4_slot: &mut f64,
        var_beta2_dn5_slot: &mut f64,
        var_beta2_dn6_slot: &mut f64,
        var_beta2_dn7_slot: &mut f64,
        var_beta2_dn8_slot: &mut f64,
        var_beta2_dn9_slot: &mut f64,
        var_beta2_rv_slot: &mut f64,
        var_beta_dn0_slot: &mut f64,
        var_beta_dn10_slot: &mut f64,
        var_beta_dn11_slot: &mut f64,
        var_beta_dn14_slot: &mut f64,
        var_beta_dn2_slot: &mut f64,
        var_beta_dn4_slot: &mut f64,
        var_beta_dn5_slot: &mut f64,
        var_beta_dn6_slot: &mut f64,
        var_beta_dn7_slot: &mut f64,
        var_beta_dn8_slot: &mut f64,
        var_beta_dn9_slot: &mut f64,
        var_beta_inv_slot: &mut f64,
        var_beta_inv_dn0_slot: &mut f64,
        var_beta_inv_dn10_slot: &mut f64,
        var_beta_inv_dn11_slot: &mut f64,
        var_beta_inv_dn14_slot: &mut f64,
        var_beta_inv_dn2_slot: &mut f64,
        var_beta_inv_dn4_slot: &mut f64,
        var_beta_inv_dn5_slot: &mut f64,
        var_beta_inv_dn6_slot: &mut f64,
        var_beta_inv_dn7_slot: &mut f64,
        var_beta_inv_dn8_slot: &mut f64,
        var_beta_inv_dn9_slot: &mut f64,
        var_beta_inv_rv_slot: &mut f64,
        var_beta_rv_slot: &mut f64,
        var_betatnom_slot: &mut f64,
        var_betatnom_rv_slot: &mut f64,
        var_cnst0over_slot: &mut f64,
        var_cnst0over_dn0_slot: &mut f64,
        var_cnst0over_dn10_slot: &mut f64,
        var_cnst0over_dn11_slot: &mut f64,
        var_cnst0over_dn14_slot: &mut f64,
        var_cnst0over_dn2_slot: &mut f64,
        var_cnst0over_dn4_slot: &mut f64,
        var_cnst0over_dn5_slot: &mut f64,
        var_cnst0over_dn6_slot: &mut f64,
        var_cnst0over_dn7_slot: &mut f64,
        var_cnst0over_dn8_slot: &mut f64,
        var_cnst0over_dn9_slot: &mut f64,
        var_cnst0over_rv_slot: &mut f64,
        var_cnst0overs_slot: &mut f64,
        var_cnst0overs_dn0_slot: &mut f64,
        var_cnst0overs_dn10_slot: &mut f64,
        var_cnst0overs_dn11_slot: &mut f64,
        var_cnst0overs_dn14_slot: &mut f64,
        var_cnst0overs_dn2_slot: &mut f64,
        var_cnst0overs_dn4_slot: &mut f64,
        var_cnst0overs_dn5_slot: &mut f64,
        var_cnst0overs_dn6_slot: &mut f64,
        var_cnst0overs_dn7_slot: &mut f64,
        var_cnst0overs_dn8_slot: &mut f64,
        var_cnst0overs_dn9_slot: &mut f64,
        var_cnst0overs_rv_slot: &mut f64,
        var_eg_slot: &mut f64,
        var_eg_dn0_slot: &mut f64,
        var_eg_dn10_slot: &mut f64,
        var_eg_dn11_slot: &mut f64,
        var_eg_dn14_slot: &mut f64,
        var_eg_dn2_slot: &mut f64,
        var_eg_dn4_slot: &mut f64,
        var_eg_dn5_slot: &mut f64,
        var_eg_dn6_slot: &mut f64,
        var_eg_dn7_slot: &mut f64,
        var_eg_dn8_slot: &mut f64,
        var_eg_dn9_slot: &mut f64,
        var_eg_rv_slot: &mut f64,
        var_egp12_slot: &mut f64,
        var_egp12_dn0_slot: &mut f64,
        var_egp12_dn10_slot: &mut f64,
        var_egp12_dn11_slot: &mut f64,
        var_egp12_dn14_slot: &mut f64,
        var_egp12_dn2_slot: &mut f64,
        var_egp12_dn4_slot: &mut f64,
        var_egp12_dn5_slot: &mut f64,
        var_egp12_dn6_slot: &mut f64,
        var_egp12_dn7_slot: &mut f64,
        var_egp12_dn8_slot: &mut f64,
        var_egp12_dn9_slot: &mut f64,
        var_egp12_rv_slot: &mut f64,
        var_egp32_slot: &mut f64,
        var_egp32_dn0_slot: &mut f64,
        var_egp32_dn10_slot: &mut f64,
        var_egp32_dn11_slot: &mut f64,
        var_egp32_dn14_slot: &mut f64,
        var_egp32_dn2_slot: &mut f64,
        var_egp32_dn4_slot: &mut f64,
        var_egp32_dn5_slot: &mut f64,
        var_egp32_dn6_slot: &mut f64,
        var_egp32_dn7_slot: &mut f64,
        var_egp32_dn8_slot: &mut f64,
        var_egp32_dn9_slot: &mut f64,
        var_egp32_rv_slot: &mut f64,
        var_log_tratio_slot: &mut f64,
        var_log_tratio_dn0_slot: &mut f64,
        var_log_tratio_dn10_slot: &mut f64,
        var_log_tratio_dn11_slot: &mut f64,
        var_log_tratio_dn14_slot: &mut f64,
        var_log_tratio_dn2_slot: &mut f64,
        var_log_tratio_dn4_slot: &mut f64,
        var_log_tratio_dn5_slot: &mut f64,
        var_log_tratio_dn6_slot: &mut f64,
        var_log_tratio_dn7_slot: &mut f64,
        var_log_tratio_dn8_slot: &mut f64,
        var_log_tratio_dn9_slot: &mut f64,
        var_log_tratio_rv_slot: &mut f64,
        var_sqrt_eg_slot: &mut f64,
        var_sqrt_eg_dn0_slot: &mut f64,
        var_sqrt_eg_dn10_slot: &mut f64,
        var_sqrt_eg_dn11_slot: &mut f64,
        var_sqrt_eg_dn14_slot: &mut f64,
        var_sqrt_eg_dn2_slot: &mut f64,
        var_sqrt_eg_dn4_slot: &mut f64,
        var_sqrt_eg_dn5_slot: &mut f64,
        var_sqrt_eg_dn6_slot: &mut f64,
        var_sqrt_eg_dn7_slot: &mut f64,
        var_sqrt_eg_dn8_slot: &mut f64,
        var_sqrt_eg_dn9_slot: &mut f64,
        var_sqrt_eg_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn11_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn11_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn10_slot: &mut f64,
        var_t3_dn11_slot: &mut f64,
        var_t3_dn14_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_tdiff_slot: &mut f64,
        var_tdiff0_slot: &mut f64,
        var_tdiff0_2_slot: &mut f64,
        var_tdiff0_2_dn0_slot: &mut f64,
        var_tdiff0_2_dn10_slot: &mut f64,
        var_tdiff0_2_dn11_slot: &mut f64,
        var_tdiff0_2_dn14_slot: &mut f64,
        var_tdiff0_2_dn2_slot: &mut f64,
        var_tdiff0_2_dn4_slot: &mut f64,
        var_tdiff0_2_dn5_slot: &mut f64,
        var_tdiff0_2_dn6_slot: &mut f64,
        var_tdiff0_2_dn7_slot: &mut f64,
        var_tdiff0_2_dn8_slot: &mut f64,
        var_tdiff0_2_dn9_slot: &mut f64,
        var_tdiff0_2_rv_slot: &mut f64,
        var_tdiff0_dn0_slot: &mut f64,
        var_tdiff0_dn10_slot: &mut f64,
        var_tdiff0_dn11_slot: &mut f64,
        var_tdiff0_dn14_slot: &mut f64,
        var_tdiff0_dn2_slot: &mut f64,
        var_tdiff0_dn4_slot: &mut f64,
        var_tdiff0_dn5_slot: &mut f64,
        var_tdiff0_dn6_slot: &mut f64,
        var_tdiff0_dn7_slot: &mut f64,
        var_tdiff0_dn8_slot: &mut f64,
        var_tdiff0_dn9_slot: &mut f64,
        var_tdiff0_rv_slot: &mut f64,
        var_tdiff_2_slot: &mut f64,
        var_tdiff_2_dn0_slot: &mut f64,
        var_tdiff_2_dn10_slot: &mut f64,
        var_tdiff_2_dn11_slot: &mut f64,
        var_tdiff_2_dn14_slot: &mut f64,
        var_tdiff_2_dn2_slot: &mut f64,
        var_tdiff_2_dn4_slot: &mut f64,
        var_tdiff_2_dn5_slot: &mut f64,
        var_tdiff_2_dn6_slot: &mut f64,
        var_tdiff_2_dn7_slot: &mut f64,
        var_tdiff_2_dn8_slot: &mut f64,
        var_tdiff_2_dn9_slot: &mut f64,
        var_tdiff_2_rv_slot: &mut f64,
        var_tdiff_dn0_slot: &mut f64,
        var_tdiff_dn10_slot: &mut f64,
        var_tdiff_dn11_slot: &mut f64,
        var_tdiff_dn14_slot: &mut f64,
        var_tdiff_dn2_slot: &mut f64,
        var_tdiff_dn4_slot: &mut f64,
        var_tdiff_dn5_slot: &mut f64,
        var_tdiff_dn6_slot: &mut f64,
        var_tdiff_dn7_slot: &mut f64,
        var_tdiff_dn8_slot: &mut f64,
        var_tdiff_dn9_slot: &mut f64,
        var_tdiff_rv_slot: &mut f64,
        var_tratio_slot: &mut f64,
        var_tratio_dn0_slot: &mut f64,
        var_tratio_dn10_slot: &mut f64,
        var_tratio_dn11_slot: &mut f64,
        var_tratio_dn14_slot: &mut f64,
        var_tratio_dn2_slot: &mut f64,
        var_tratio_dn4_slot: &mut f64,
        var_tratio_dn5_slot: &mut f64,
        var_tratio_dn6_slot: &mut f64,
        var_tratio_dn7_slot: &mut f64,
        var_tratio_dn8_slot: &mut f64,
        var_tratio_dn9_slot: &mut f64,
        var_tratio_rv_slot: &mut f64,
        var_ttemp_slot: &mut f64,
        var_ttemp0_slot: &mut f64,
        var_ttemp0_dn0_slot: &mut f64,
        var_ttemp0_dn10_slot: &mut f64,
        var_ttemp0_dn11_slot: &mut f64,
        var_ttemp0_dn14_slot: &mut f64,
        var_ttemp0_dn2_slot: &mut f64,
        var_ttemp0_dn4_slot: &mut f64,
        var_ttemp0_dn5_slot: &mut f64,
        var_ttemp0_dn6_slot: &mut f64,
        var_ttemp0_dn7_slot: &mut f64,
        var_ttemp0_dn8_slot: &mut f64,
        var_ttemp0_dn9_slot: &mut f64,
        var_ttemp0_rv_slot: &mut f64,
        var_ttemp_dn0_slot: &mut f64,
        var_ttemp_dn10_slot: &mut f64,
        var_ttemp_dn11_slot: &mut f64,
        var_ttemp_dn14_slot: &mut f64,
        var_ttemp_dn2_slot: &mut f64,
        var_ttemp_dn4_slot: &mut f64,
        var_ttemp_dn5_slot: &mut f64,
        var_ttemp_dn6_slot: &mut f64,
        var_ttemp_dn7_slot: &mut f64,
        var_ttemp_dn8_slot: &mut f64,
        var_ttemp_dn9_slot: &mut f64,
        var_ttemp_rv_slot: &mut f64,
    ) {
        let ctx_temp = ctx.temperature();
        let mut var_beta: f64 = *var_beta_slot;
        let mut var_beta2: f64 = *var_beta2_slot;
        let mut var_beta2_dn0: f64 = *var_beta2_dn0_slot;
        let mut var_beta2_dn10: f64 = *var_beta2_dn10_slot;
        let mut var_beta2_dn11: f64 = *var_beta2_dn11_slot;
        let mut var_beta2_dn14: f64 = *var_beta2_dn14_slot;
        let mut var_beta2_dn2: f64 = *var_beta2_dn2_slot;
        let mut var_beta2_dn4: f64 = *var_beta2_dn4_slot;
        let mut var_beta2_dn5: f64 = *var_beta2_dn5_slot;
        let mut var_beta2_dn6: f64 = *var_beta2_dn6_slot;
        let mut var_beta2_dn7: f64 = *var_beta2_dn7_slot;
        let mut var_beta2_dn8: f64 = *var_beta2_dn8_slot;
        let mut var_beta2_dn9: f64 = *var_beta2_dn9_slot;
        let mut var_beta2_rv: f64 = *var_beta2_rv_slot;
        let mut var_beta_dn0: f64 = *var_beta_dn0_slot;
        let mut var_beta_dn10: f64 = *var_beta_dn10_slot;
        let mut var_beta_dn11: f64 = *var_beta_dn11_slot;
        let mut var_beta_dn14: f64 = *var_beta_dn14_slot;
        let mut var_beta_dn2: f64 = *var_beta_dn2_slot;
        let mut var_beta_dn4: f64 = *var_beta_dn4_slot;
        let mut var_beta_dn5: f64 = *var_beta_dn5_slot;
        let mut var_beta_dn6: f64 = *var_beta_dn6_slot;
        let mut var_beta_dn7: f64 = *var_beta_dn7_slot;
        let mut var_beta_dn8: f64 = *var_beta_dn8_slot;
        let mut var_beta_dn9: f64 = *var_beta_dn9_slot;
        let mut var_beta_inv: f64 = *var_beta_inv_slot;
        let mut var_beta_inv_dn0: f64 = *var_beta_inv_dn0_slot;
        let mut var_beta_inv_dn10: f64 = *var_beta_inv_dn10_slot;
        let mut var_beta_inv_dn11: f64 = *var_beta_inv_dn11_slot;
        let mut var_beta_inv_dn14: f64 = *var_beta_inv_dn14_slot;
        let mut var_beta_inv_dn2: f64 = *var_beta_inv_dn2_slot;
        let mut var_beta_inv_dn4: f64 = *var_beta_inv_dn4_slot;
        let mut var_beta_inv_dn5: f64 = *var_beta_inv_dn5_slot;
        let mut var_beta_inv_dn6: f64 = *var_beta_inv_dn6_slot;
        let mut var_beta_inv_dn7: f64 = *var_beta_inv_dn7_slot;
        let mut var_beta_inv_dn8: f64 = *var_beta_inv_dn8_slot;
        let mut var_beta_inv_dn9: f64 = *var_beta_inv_dn9_slot;
        let mut var_beta_inv_rv: f64 = *var_beta_inv_rv_slot;
        let mut var_beta_rv: f64 = *var_beta_rv_slot;
        let mut var_betatnom: f64 = *var_betatnom_slot;
        let mut var_betatnom_rv: f64 = *var_betatnom_rv_slot;
        let mut var_cnst0over: f64 = *var_cnst0over_slot;
        let mut var_cnst0over_dn0: f64 = *var_cnst0over_dn0_slot;
        let mut var_cnst0over_dn10: f64 = *var_cnst0over_dn10_slot;
        let mut var_cnst0over_dn11: f64 = *var_cnst0over_dn11_slot;
        let mut var_cnst0over_dn14: f64 = *var_cnst0over_dn14_slot;
        let mut var_cnst0over_dn2: f64 = *var_cnst0over_dn2_slot;
        let mut var_cnst0over_dn4: f64 = *var_cnst0over_dn4_slot;
        let mut var_cnst0over_dn5: f64 = *var_cnst0over_dn5_slot;
        let mut var_cnst0over_dn6: f64 = *var_cnst0over_dn6_slot;
        let mut var_cnst0over_dn7: f64 = *var_cnst0over_dn7_slot;
        let mut var_cnst0over_dn8: f64 = *var_cnst0over_dn8_slot;
        let mut var_cnst0over_dn9: f64 = *var_cnst0over_dn9_slot;
        let mut var_cnst0over_rv: f64 = *var_cnst0over_rv_slot;
        let mut var_cnst0overs: f64 = *var_cnst0overs_slot;
        let mut var_cnst0overs_dn0: f64 = *var_cnst0overs_dn0_slot;
        let mut var_cnst0overs_dn10: f64 = *var_cnst0overs_dn10_slot;
        let mut var_cnst0overs_dn11: f64 = *var_cnst0overs_dn11_slot;
        let mut var_cnst0overs_dn14: f64 = *var_cnst0overs_dn14_slot;
        let mut var_cnst0overs_dn2: f64 = *var_cnst0overs_dn2_slot;
        let mut var_cnst0overs_dn4: f64 = *var_cnst0overs_dn4_slot;
        let mut var_cnst0overs_dn5: f64 = *var_cnst0overs_dn5_slot;
        let mut var_cnst0overs_dn6: f64 = *var_cnst0overs_dn6_slot;
        let mut var_cnst0overs_dn7: f64 = *var_cnst0overs_dn7_slot;
        let mut var_cnst0overs_dn8: f64 = *var_cnst0overs_dn8_slot;
        let mut var_cnst0overs_dn9: f64 = *var_cnst0overs_dn9_slot;
        let mut var_cnst0overs_rv: f64 = *var_cnst0overs_rv_slot;
        let mut var_eg: f64 = *var_eg_slot;
        let mut var_eg_dn0: f64 = *var_eg_dn0_slot;
        let mut var_eg_dn10: f64 = *var_eg_dn10_slot;
        let mut var_eg_dn11: f64 = *var_eg_dn11_slot;
        let mut var_eg_dn14: f64 = *var_eg_dn14_slot;
        let mut var_eg_dn2: f64 = *var_eg_dn2_slot;
        let mut var_eg_dn4: f64 = *var_eg_dn4_slot;
        let mut var_eg_dn5: f64 = *var_eg_dn5_slot;
        let mut var_eg_dn6: f64 = *var_eg_dn6_slot;
        let mut var_eg_dn7: f64 = *var_eg_dn7_slot;
        let mut var_eg_dn8: f64 = *var_eg_dn8_slot;
        let mut var_eg_dn9: f64 = *var_eg_dn9_slot;
        let mut var_eg_rv: f64 = *var_eg_rv_slot;
        let mut var_egp12: f64 = *var_egp12_slot;
        let mut var_egp12_dn0: f64 = *var_egp12_dn0_slot;
        let mut var_egp12_dn10: f64 = *var_egp12_dn10_slot;
        let mut var_egp12_dn11: f64 = *var_egp12_dn11_slot;
        let mut var_egp12_dn14: f64 = *var_egp12_dn14_slot;
        let mut var_egp12_dn2: f64 = *var_egp12_dn2_slot;
        let mut var_egp12_dn4: f64 = *var_egp12_dn4_slot;
        let mut var_egp12_dn5: f64 = *var_egp12_dn5_slot;
        let mut var_egp12_dn6: f64 = *var_egp12_dn6_slot;
        let mut var_egp12_dn7: f64 = *var_egp12_dn7_slot;
        let mut var_egp12_dn8: f64 = *var_egp12_dn8_slot;
        let mut var_egp12_dn9: f64 = *var_egp12_dn9_slot;
        let mut var_egp12_rv: f64 = *var_egp12_rv_slot;
        let mut var_egp32: f64 = *var_egp32_slot;
        let mut var_egp32_dn0: f64 = *var_egp32_dn0_slot;
        let mut var_egp32_dn10: f64 = *var_egp32_dn10_slot;
        let mut var_egp32_dn11: f64 = *var_egp32_dn11_slot;
        let mut var_egp32_dn14: f64 = *var_egp32_dn14_slot;
        let mut var_egp32_dn2: f64 = *var_egp32_dn2_slot;
        let mut var_egp32_dn4: f64 = *var_egp32_dn4_slot;
        let mut var_egp32_dn5: f64 = *var_egp32_dn5_slot;
        let mut var_egp32_dn6: f64 = *var_egp32_dn6_slot;
        let mut var_egp32_dn7: f64 = *var_egp32_dn7_slot;
        let mut var_egp32_dn8: f64 = *var_egp32_dn8_slot;
        let mut var_egp32_dn9: f64 = *var_egp32_dn9_slot;
        let mut var_egp32_rv: f64 = *var_egp32_rv_slot;
        let mut var_log_tratio: f64 = *var_log_tratio_slot;
        let mut var_log_tratio_dn0: f64 = *var_log_tratio_dn0_slot;
        let mut var_log_tratio_dn10: f64 = *var_log_tratio_dn10_slot;
        let mut var_log_tratio_dn11: f64 = *var_log_tratio_dn11_slot;
        let mut var_log_tratio_dn14: f64 = *var_log_tratio_dn14_slot;
        let mut var_log_tratio_dn2: f64 = *var_log_tratio_dn2_slot;
        let mut var_log_tratio_dn4: f64 = *var_log_tratio_dn4_slot;
        let mut var_log_tratio_dn5: f64 = *var_log_tratio_dn5_slot;
        let mut var_log_tratio_dn6: f64 = *var_log_tratio_dn6_slot;
        let mut var_log_tratio_dn7: f64 = *var_log_tratio_dn7_slot;
        let mut var_log_tratio_dn8: f64 = *var_log_tratio_dn8_slot;
        let mut var_log_tratio_dn9: f64 = *var_log_tratio_dn9_slot;
        let mut var_log_tratio_rv: f64 = *var_log_tratio_rv_slot;
        let mut var_sqrt_eg: f64 = *var_sqrt_eg_slot;
        let mut var_sqrt_eg_dn0: f64 = *var_sqrt_eg_dn0_slot;
        let mut var_sqrt_eg_dn10: f64 = *var_sqrt_eg_dn10_slot;
        let mut var_sqrt_eg_dn11: f64 = *var_sqrt_eg_dn11_slot;
        let mut var_sqrt_eg_dn14: f64 = *var_sqrt_eg_dn14_slot;
        let mut var_sqrt_eg_dn2: f64 = *var_sqrt_eg_dn2_slot;
        let mut var_sqrt_eg_dn4: f64 = *var_sqrt_eg_dn4_slot;
        let mut var_sqrt_eg_dn5: f64 = *var_sqrt_eg_dn5_slot;
        let mut var_sqrt_eg_dn6: f64 = *var_sqrt_eg_dn6_slot;
        let mut var_sqrt_eg_dn7: f64 = *var_sqrt_eg_dn7_slot;
        let mut var_sqrt_eg_dn8: f64 = *var_sqrt_eg_dn8_slot;
        let mut var_sqrt_eg_dn9: f64 = *var_sqrt_eg_dn9_slot;
        let mut var_sqrt_eg_rv: f64 = *var_sqrt_eg_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn11: f64 = *var_t1_dn11_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn11: f64 = *var_t2_dn11_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn10: f64 = *var_t3_dn10_slot;
        let mut var_t3_dn11: f64 = *var_t3_dn11_slot;
        let mut var_t3_dn14: f64 = *var_t3_dn14_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_tdiff: f64 = *var_tdiff_slot;
        let mut var_tdiff0: f64 = *var_tdiff0_slot;
        let mut var_tdiff0_2: f64 = *var_tdiff0_2_slot;
        let mut var_tdiff0_2_dn0: f64 = *var_tdiff0_2_dn0_slot;
        let mut var_tdiff0_2_dn10: f64 = *var_tdiff0_2_dn10_slot;
        let mut var_tdiff0_2_dn11: f64 = *var_tdiff0_2_dn11_slot;
        let mut var_tdiff0_2_dn14: f64 = *var_tdiff0_2_dn14_slot;
        let mut var_tdiff0_2_dn2: f64 = *var_tdiff0_2_dn2_slot;
        let mut var_tdiff0_2_dn4: f64 = *var_tdiff0_2_dn4_slot;
        let mut var_tdiff0_2_dn5: f64 = *var_tdiff0_2_dn5_slot;
        let mut var_tdiff0_2_dn6: f64 = *var_tdiff0_2_dn6_slot;
        let mut var_tdiff0_2_dn7: f64 = *var_tdiff0_2_dn7_slot;
        let mut var_tdiff0_2_dn8: f64 = *var_tdiff0_2_dn8_slot;
        let mut var_tdiff0_2_dn9: f64 = *var_tdiff0_2_dn9_slot;
        let mut var_tdiff0_2_rv: f64 = *var_tdiff0_2_rv_slot;
        let mut var_tdiff0_dn0: f64 = *var_tdiff0_dn0_slot;
        let mut var_tdiff0_dn10: f64 = *var_tdiff0_dn10_slot;
        let mut var_tdiff0_dn11: f64 = *var_tdiff0_dn11_slot;
        let mut var_tdiff0_dn14: f64 = *var_tdiff0_dn14_slot;
        let mut var_tdiff0_dn2: f64 = *var_tdiff0_dn2_slot;
        let mut var_tdiff0_dn4: f64 = *var_tdiff0_dn4_slot;
        let mut var_tdiff0_dn5: f64 = *var_tdiff0_dn5_slot;
        let mut var_tdiff0_dn6: f64 = *var_tdiff0_dn6_slot;
        let mut var_tdiff0_dn7: f64 = *var_tdiff0_dn7_slot;
        let mut var_tdiff0_dn8: f64 = *var_tdiff0_dn8_slot;
        let mut var_tdiff0_dn9: f64 = *var_tdiff0_dn9_slot;
        let mut var_tdiff0_rv: f64 = *var_tdiff0_rv_slot;
        let mut var_tdiff_2: f64 = *var_tdiff_2_slot;
        let mut var_tdiff_2_dn0: f64 = *var_tdiff_2_dn0_slot;
        let mut var_tdiff_2_dn10: f64 = *var_tdiff_2_dn10_slot;
        let mut var_tdiff_2_dn11: f64 = *var_tdiff_2_dn11_slot;
        let mut var_tdiff_2_dn14: f64 = *var_tdiff_2_dn14_slot;
        let mut var_tdiff_2_dn2: f64 = *var_tdiff_2_dn2_slot;
        let mut var_tdiff_2_dn4: f64 = *var_tdiff_2_dn4_slot;
        let mut var_tdiff_2_dn5: f64 = *var_tdiff_2_dn5_slot;
        let mut var_tdiff_2_dn6: f64 = *var_tdiff_2_dn6_slot;
        let mut var_tdiff_2_dn7: f64 = *var_tdiff_2_dn7_slot;
        let mut var_tdiff_2_dn8: f64 = *var_tdiff_2_dn8_slot;
        let mut var_tdiff_2_dn9: f64 = *var_tdiff_2_dn9_slot;
        let mut var_tdiff_2_rv: f64 = *var_tdiff_2_rv_slot;
        let mut var_tdiff_dn0: f64 = *var_tdiff_dn0_slot;
        let mut var_tdiff_dn10: f64 = *var_tdiff_dn10_slot;
        let mut var_tdiff_dn11: f64 = *var_tdiff_dn11_slot;
        let mut var_tdiff_dn14: f64 = *var_tdiff_dn14_slot;
        let mut var_tdiff_dn2: f64 = *var_tdiff_dn2_slot;
        let mut var_tdiff_dn4: f64 = *var_tdiff_dn4_slot;
        let mut var_tdiff_dn5: f64 = *var_tdiff_dn5_slot;
        let mut var_tdiff_dn6: f64 = *var_tdiff_dn6_slot;
        let mut var_tdiff_dn7: f64 = *var_tdiff_dn7_slot;
        let mut var_tdiff_dn8: f64 = *var_tdiff_dn8_slot;
        let mut var_tdiff_dn9: f64 = *var_tdiff_dn9_slot;
        let mut var_tdiff_rv: f64 = *var_tdiff_rv_slot;
        let mut var_tratio: f64 = *var_tratio_slot;
        let mut var_tratio_dn0: f64 = *var_tratio_dn0_slot;
        let mut var_tratio_dn10: f64 = *var_tratio_dn10_slot;
        let mut var_tratio_dn11: f64 = *var_tratio_dn11_slot;
        let mut var_tratio_dn14: f64 = *var_tratio_dn14_slot;
        let mut var_tratio_dn2: f64 = *var_tratio_dn2_slot;
        let mut var_tratio_dn4: f64 = *var_tratio_dn4_slot;
        let mut var_tratio_dn5: f64 = *var_tratio_dn5_slot;
        let mut var_tratio_dn6: f64 = *var_tratio_dn6_slot;
        let mut var_tratio_dn7: f64 = *var_tratio_dn7_slot;
        let mut var_tratio_dn8: f64 = *var_tratio_dn8_slot;
        let mut var_tratio_dn9: f64 = *var_tratio_dn9_slot;
        let mut var_tratio_rv: f64 = *var_tratio_rv_slot;
        let mut var_ttemp: f64 = *var_ttemp_slot;
        let mut var_ttemp0: f64 = *var_ttemp0_slot;
        let mut var_ttemp0_dn0: f64 = *var_ttemp0_dn0_slot;
        let mut var_ttemp0_dn10: f64 = *var_ttemp0_dn10_slot;
        let mut var_ttemp0_dn11: f64 = *var_ttemp0_dn11_slot;
        let mut var_ttemp0_dn14: f64 = *var_ttemp0_dn14_slot;
        let mut var_ttemp0_dn2: f64 = *var_ttemp0_dn2_slot;
        let mut var_ttemp0_dn4: f64 = *var_ttemp0_dn4_slot;
        let mut var_ttemp0_dn5: f64 = *var_ttemp0_dn5_slot;
        let mut var_ttemp0_dn6: f64 = *var_ttemp0_dn6_slot;
        let mut var_ttemp0_dn7: f64 = *var_ttemp0_dn7_slot;
        let mut var_ttemp0_dn8: f64 = *var_ttemp0_dn8_slot;
        let mut var_ttemp0_dn9: f64 = *var_ttemp0_dn9_slot;
        let mut var_ttemp0_rv: f64 = *var_ttemp0_rv_slot;
        let mut var_ttemp_dn0: f64 = *var_ttemp_dn0_slot;
        let mut var_ttemp_dn10: f64 = *var_ttemp_dn10_slot;
        let mut var_ttemp_dn11: f64 = *var_ttemp_dn11_slot;
        let mut var_ttemp_dn14: f64 = *var_ttemp_dn14_slot;
        let mut var_ttemp_dn2: f64 = *var_ttemp_dn2_slot;
        let mut var_ttemp_dn4: f64 = *var_ttemp_dn4_slot;
        let mut var_ttemp_dn5: f64 = *var_ttemp_dn5_slot;
        let mut var_ttemp_dn6: f64 = *var_ttemp_dn6_slot;
        let mut var_ttemp_dn7: f64 = *var_ttemp_dn7_slot;
        let mut var_ttemp_dn8: f64 = *var_ttemp_dn8_slot;
        let mut var_ttemp_dn9: f64 = *var_ttemp_dn9_slot;
        let mut var_ttemp_rv: f64 = *var_ttemp_rv_slot;

        let (assign12970_e7327, assign12970_e7327_d_n0, assign12970_e7327_d_n2, assign12970_e7327_d_n4, assign12970_e7327_d_n5, assign12970_e7327_d_n6, assign12970_e7327_d_n7, assign12970_e7327_d_n8, assign12970_e7327_d_n9, assign12970_e7327_d_n10, assign12970_e7327_d_n11, assign12970_e7327_d_n14,) = {
    if (var_guard291 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cnst0over, var_cnst0over_dn0, var_cnst0over_dn2, var_cnst0over_dn4, var_cnst0over_dn5, var_cnst0over_dn6, var_cnst0over_dn7, var_cnst0over_dn8, var_cnst0over_dn9, var_cnst0over_dn10, var_cnst0over_dn11, var_cnst0over_dn14,)
    }
};
        var_cnst0over = assign12970_e7327;
        var_cnst0over_dn0 = assign12970_e7327_d_n0;
        var_cnst0over_dn2 = assign12970_e7327_d_n2;
        var_cnst0over_dn4 = assign12970_e7327_d_n4;
        var_cnst0over_dn5 = assign12970_e7327_d_n5;
        var_cnst0over_dn6 = assign12970_e7327_d_n6;
        var_cnst0over_dn7 = assign12970_e7327_d_n7;
        var_cnst0over_dn8 = assign12970_e7327_d_n8;
        var_cnst0over_dn9 = assign12970_e7327_d_n9;
        var_cnst0over_dn10 = assign12970_e7327_d_n10;
        var_cnst0over_dn11 = assign12970_e7327_d_n11;
        var_cnst0over_dn14 = assign12970_e7327_d_n14;
        var_cnst0over_rv = 0.0;

        let (assign12980_e7331, assign12980_e7331_d_n0, assign12980_e7331_d_n2, assign12980_e7331_d_n4, assign12980_e7331_d_n5, assign12980_e7331_d_n6, assign12980_e7331_d_n7, assign12980_e7331_d_n8, assign12980_e7331_d_n9, assign12980_e7331_d_n10, assign12980_e7331_d_n11, assign12980_e7331_d_n14,) = {
    if (var_guard291 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_cnst0overs, var_cnst0overs_dn0, var_cnst0overs_dn2, var_cnst0overs_dn4, var_cnst0overs_dn5, var_cnst0overs_dn6, var_cnst0overs_dn7, var_cnst0overs_dn8, var_cnst0overs_dn9, var_cnst0overs_dn10, var_cnst0overs_dn11, var_cnst0overs_dn14,)
    }
};
        var_cnst0overs = assign12980_e7331;
        var_cnst0overs_dn0 = assign12980_e7331_d_n0;
        var_cnst0overs_dn2 = assign12980_e7331_d_n2;
        var_cnst0overs_dn4 = assign12980_e7331_d_n4;
        var_cnst0overs_dn5 = assign12980_e7331_d_n5;
        var_cnst0overs_dn6 = assign12980_e7331_d_n6;
        var_cnst0overs_dn7 = assign12980_e7331_d_n7;
        var_cnst0overs_dn8 = assign12980_e7331_d_n8;
        var_cnst0overs_dn9 = assign12980_e7331_d_n9;
        var_cnst0overs_dn10 = assign12980_e7331_d_n10;
        var_cnst0overs_dn11 = assign12980_e7331_d_n11;
        var_cnst0overs_dn14 = assign12980_e7331_d_n14;
        var_cnst0overs_rv = 0.0;

        let (assign12990_e7337, assign12990_e7337_d_n0, assign12990_e7337_d_n2, assign12990_e7337_d_n4, assign12990_e7337_d_n5, assign12990_e7337_d_n6, assign12990_e7337_d_n7, assign12990_e7337_d_n8, assign12990_e7337_d_n9, assign12990_e7337_d_n10, assign12990_e7337_d_n11, assign12990_e7337_d_n14,) = {
    if (var_guard291 != 0.0) {
        let assign12990_e7333: f64 = ctx_temp;
        let assign12990_e7335: f64 = (assign12990_e7333 + p.p11);
        (assign12990_e7335, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ttemp, var_ttemp_dn0, var_ttemp_dn2, var_ttemp_dn4, var_ttemp_dn5, var_ttemp_dn6, var_ttemp_dn7, var_ttemp_dn8, var_ttemp_dn9, var_ttemp_dn10, var_ttemp_dn11, var_ttemp_dn14,)
    }
};
        var_ttemp = assign12990_e7337;
        var_ttemp_dn0 = assign12990_e7337_d_n0;
        var_ttemp_dn2 = assign12990_e7337_d_n2;
        var_ttemp_dn4 = assign12990_e7337_d_n4;
        var_ttemp_dn5 = assign12990_e7337_d_n5;
        var_ttemp_dn6 = assign12990_e7337_d_n6;
        var_ttemp_dn7 = assign12990_e7337_d_n7;
        var_ttemp_dn8 = assign12990_e7337_d_n8;
        var_ttemp_dn9 = assign12990_e7337_d_n9;
        var_ttemp_dn10 = assign12990_e7337_d_n10;
        var_ttemp_dn11 = assign12990_e7337_d_n11;
        var_ttemp_dn14 = assign12990_e7337_d_n14;
        var_ttemp_rv = 0.0;

        let (assign13000_e7341, assign13000_e7341_d_n0, assign13000_e7341_d_n2, assign13000_e7341_d_n4, assign13000_e7341_d_n5, assign13000_e7341_d_n6, assign13000_e7341_d_n7, assign13000_e7341_d_n8, assign13000_e7341_d_n9, assign13000_e7341_d_n10, assign13000_e7341_d_n11, assign13000_e7341_d_n14,) = {
    if (var_guard291 != 0.0) {
        (var_ttemp, var_ttemp_dn0, var_ttemp_dn2, var_ttemp_dn4, var_ttemp_dn5, var_ttemp_dn6, var_ttemp_dn7, var_ttemp_dn8, var_ttemp_dn9, var_ttemp_dn10, var_ttemp_dn11, var_ttemp_dn14,)
    } else {
        (var_ttemp0, var_ttemp0_dn0, var_ttemp0_dn2, var_ttemp0_dn4, var_ttemp0_dn5, var_ttemp0_dn6, var_ttemp0_dn7, var_ttemp0_dn8, var_ttemp0_dn9, var_ttemp0_dn10, var_ttemp0_dn11, var_ttemp0_dn14,)
    }
};
        var_ttemp0 = assign13000_e7341;
        var_ttemp0_dn0 = assign13000_e7341_d_n0;
        var_ttemp0_dn2 = assign13000_e7341_d_n2;
        var_ttemp0_dn4 = assign13000_e7341_d_n4;
        var_ttemp0_dn5 = assign13000_e7341_d_n5;
        var_ttemp0_dn6 = assign13000_e7341_d_n6;
        var_ttemp0_dn7 = assign13000_e7341_d_n7;
        var_ttemp0_dn8 = assign13000_e7341_d_n8;
        var_ttemp0_dn9 = assign13000_e7341_d_n9;
        var_ttemp0_dn10 = assign13000_e7341_d_n10;
        var_ttemp0_dn11 = assign13000_e7341_d_n11;
        var_ttemp0_dn14 = assign13000_e7341_d_n14;
        var_ttemp0_rv = 0.0;

        let (assign13010_e7347, assign13010_e7347_d_n0, assign13010_e7347_d_n2, assign13010_e7347_d_n4, assign13010_e7347_d_n5, assign13010_e7347_d_n6, assign13010_e7347_d_n7, assign13010_e7347_d_n8, assign13010_e7347_d_n9, assign13010_e7347_d_n10, assign13010_e7347_d_n11, assign13010_e7347_d_n14,) = {
    if (var_guard291 != 0.0) {
        let assign13010_e7345: f64 = (var_ttemp + var_deltemp);
        (assign13010_e7345, (var_ttemp_dn0 + var_deltemp_dn0), (var_ttemp_dn2 + var_deltemp_dn2), (var_ttemp_dn4 + var_deltemp_dn4), (var_ttemp_dn5 + var_deltemp_dn5), (var_ttemp_dn6 + var_deltemp_dn6), (var_ttemp_dn7 + var_deltemp_dn7), (var_ttemp_dn8 + var_deltemp_dn8), (var_ttemp_dn9 + var_deltemp_dn9), (var_ttemp_dn10 + var_deltemp_dn10), (var_ttemp_dn11 + var_deltemp_dn11), (var_ttemp_dn14 + var_deltemp_dn14),)
    } else {
        (var_ttemp, var_ttemp_dn0, var_ttemp_dn2, var_ttemp_dn4, var_ttemp_dn5, var_ttemp_dn6, var_ttemp_dn7, var_ttemp_dn8, var_ttemp_dn9, var_ttemp_dn10, var_ttemp_dn11, var_ttemp_dn14,)
    }
};
        var_ttemp = assign13010_e7347;
        var_ttemp_dn0 = assign13010_e7347_d_n0;
        var_ttemp_dn2 = assign13010_e7347_d_n2;
        var_ttemp_dn4 = assign13010_e7347_d_n4;
        var_ttemp_dn5 = assign13010_e7347_d_n5;
        var_ttemp_dn6 = assign13010_e7347_d_n6;
        var_ttemp_dn7 = assign13010_e7347_d_n7;
        var_ttemp_dn8 = assign13010_e7347_d_n8;
        var_ttemp_dn9 = assign13010_e7347_d_n9;
        var_ttemp_dn10 = assign13010_e7347_d_n10;
        var_ttemp_dn11 = assign13010_e7347_d_n11;
        var_ttemp_dn14 = assign13010_e7347_d_n14;
        var_ttemp_rv = 0.0;

        let (assign13020_e7353, assign13020_e7353_d_n0, assign13020_e7353_d_n2, assign13020_e7353_d_n4, assign13020_e7353_d_n5, assign13020_e7353_d_n6, assign13020_e7353_d_n7, assign13020_e7353_d_n8, assign13020_e7353_d_n9, assign13020_e7353_d_n10, assign13020_e7353_d_n11, assign13020_e7353_d_n14,) = {
    if (var_guard291 != 0.0) {
        let assign13020_e7351: f64 = (var_ttemp0 - var_ktnom);
        (assign13020_e7351, var_ttemp0_dn0, var_ttemp0_dn2, var_ttemp0_dn4, var_ttemp0_dn5, var_ttemp0_dn6, var_ttemp0_dn7, var_ttemp0_dn8, var_ttemp0_dn9, var_ttemp0_dn10, var_ttemp0_dn11, var_ttemp0_dn14,)
    } else {
        (var_tdiff0, var_tdiff0_dn0, var_tdiff0_dn2, var_tdiff0_dn4, var_tdiff0_dn5, var_tdiff0_dn6, var_tdiff0_dn7, var_tdiff0_dn8, var_tdiff0_dn9, var_tdiff0_dn10, var_tdiff0_dn11, var_tdiff0_dn14,)
    }
};
        var_tdiff0 = assign13020_e7353;
        var_tdiff0_dn0 = assign13020_e7353_d_n0;
        var_tdiff0_dn2 = assign13020_e7353_d_n2;
        var_tdiff0_dn4 = assign13020_e7353_d_n4;
        var_tdiff0_dn5 = assign13020_e7353_d_n5;
        var_tdiff0_dn6 = assign13020_e7353_d_n6;
        var_tdiff0_dn7 = assign13020_e7353_d_n7;
        var_tdiff0_dn8 = assign13020_e7353_d_n8;
        var_tdiff0_dn9 = assign13020_e7353_d_n9;
        var_tdiff0_dn10 = assign13020_e7353_d_n10;
        var_tdiff0_dn11 = assign13020_e7353_d_n11;
        var_tdiff0_dn14 = assign13020_e7353_d_n14;
        var_tdiff0_rv = 0.0;

        let (assign13030_e7363, assign13030_e7363_d_n0, assign13030_e7363_d_n2, assign13030_e7363_d_n4, assign13030_e7363_d_n5, assign13030_e7363_d_n6, assign13030_e7363_d_n7, assign13030_e7363_d_n8, assign13030_e7363_d_n9, assign13030_e7363_d_n10, assign13030_e7363_d_n11, assign13030_e7363_d_n14,) = {
    if (var_guard291 != 0.0) {
        let assign13030_e7357: f64 = (var_ttemp0 * var_ttemp0);
        let assign13030_e7360: f64 = (var_ktnom * var_ktnom);
        let assign13030_e7361: f64 = (assign13030_e7357 - assign13030_e7360);
        (assign13030_e7361, ((var_ttemp0_dn0 * var_ttemp0) + (var_ttemp0 * var_ttemp0_dn0)), ((var_ttemp0_dn2 * var_ttemp0) + (var_ttemp0 * var_ttemp0_dn2)), ((var_ttemp0_dn4 * var_ttemp0) + (var_ttemp0 * var_ttemp0_dn4)), ((var_ttemp0_dn5 * var_ttemp0) + (var_ttemp0 * var_ttemp0_dn5)), ((var_ttemp0_dn6 * var_ttemp0) + (var_ttemp0 * var_ttemp0_dn6)), ((var_ttemp0_dn7 * var_ttemp0) + (var_ttemp0 * var_ttemp0_dn7)), ((var_ttemp0_dn8 * var_ttemp0) + (var_ttemp0 * var_ttemp0_dn8)), ((var_ttemp0_dn9 * var_ttemp0) + (var_ttemp0 * var_ttemp0_dn9)), ((var_ttemp0_dn10 * var_ttemp0) + (var_ttemp0 * var_ttemp0_dn10)), ((var_ttemp0_dn11 * var_ttemp0) + (var_ttemp0 * var_ttemp0_dn11)), ((var_ttemp0_dn14 * var_ttemp0) + (var_ttemp0 * var_ttemp0_dn14)),)
    } else {
        (var_tdiff0_2, var_tdiff0_2_dn0, var_tdiff0_2_dn2, var_tdiff0_2_dn4, var_tdiff0_2_dn5, var_tdiff0_2_dn6, var_tdiff0_2_dn7, var_tdiff0_2_dn8, var_tdiff0_2_dn9, var_tdiff0_2_dn10, var_tdiff0_2_dn11, var_tdiff0_2_dn14,)
    }
};
        var_tdiff0_2 = assign13030_e7363;
        var_tdiff0_2_dn0 = assign13030_e7363_d_n0;
        var_tdiff0_2_dn2 = assign13030_e7363_d_n2;
        var_tdiff0_2_dn4 = assign13030_e7363_d_n4;
        var_tdiff0_2_dn5 = assign13030_e7363_d_n5;
        var_tdiff0_2_dn6 = assign13030_e7363_d_n6;
        var_tdiff0_2_dn7 = assign13030_e7363_d_n7;
        var_tdiff0_2_dn8 = assign13030_e7363_d_n8;
        var_tdiff0_2_dn9 = assign13030_e7363_d_n9;
        var_tdiff0_2_dn10 = assign13030_e7363_d_n10;
        var_tdiff0_2_dn11 = assign13030_e7363_d_n11;
        var_tdiff0_2_dn14 = assign13030_e7363_d_n14;
        var_tdiff0_2_rv = 0.0;

        let (assign13040_e7369, assign13040_e7369_d_n0, assign13040_e7369_d_n2, assign13040_e7369_d_n4, assign13040_e7369_d_n5, assign13040_e7369_d_n6, assign13040_e7369_d_n7, assign13040_e7369_d_n8, assign13040_e7369_d_n9, assign13040_e7369_d_n10, assign13040_e7369_d_n11, assign13040_e7369_d_n14,) = {
    if (var_guard291 != 0.0) {
        let assign13040_e7367: f64 = (var_ttemp - var_ktnom);
        (assign13040_e7367, var_ttemp_dn0, var_ttemp_dn2, var_ttemp_dn4, var_ttemp_dn5, var_ttemp_dn6, var_ttemp_dn7, var_ttemp_dn8, var_ttemp_dn9, var_ttemp_dn10, var_ttemp_dn11, var_ttemp_dn14,)
    } else {
        (var_tdiff, var_tdiff_dn0, var_tdiff_dn2, var_tdiff_dn4, var_tdiff_dn5, var_tdiff_dn6, var_tdiff_dn7, var_tdiff_dn8, var_tdiff_dn9, var_tdiff_dn10, var_tdiff_dn11, var_tdiff_dn14,)
    }
};
        var_tdiff = assign13040_e7369;
        var_tdiff_dn0 = assign13040_e7369_d_n0;
        var_tdiff_dn2 = assign13040_e7369_d_n2;
        var_tdiff_dn4 = assign13040_e7369_d_n4;
        var_tdiff_dn5 = assign13040_e7369_d_n5;
        var_tdiff_dn6 = assign13040_e7369_d_n6;
        var_tdiff_dn7 = assign13040_e7369_d_n7;
        var_tdiff_dn8 = assign13040_e7369_d_n8;
        var_tdiff_dn9 = assign13040_e7369_d_n9;
        var_tdiff_dn10 = assign13040_e7369_d_n10;
        var_tdiff_dn11 = assign13040_e7369_d_n11;
        var_tdiff_dn14 = assign13040_e7369_d_n14;
        var_tdiff_rv = 0.0;

        let (assign13050_e7379, assign13050_e7379_d_n0, assign13050_e7379_d_n2, assign13050_e7379_d_n4, assign13050_e7379_d_n5, assign13050_e7379_d_n6, assign13050_e7379_d_n7, assign13050_e7379_d_n8, assign13050_e7379_d_n9, assign13050_e7379_d_n10, assign13050_e7379_d_n11, assign13050_e7379_d_n14,) = {
    if (var_guard291 != 0.0) {
        let assign13050_e7373: f64 = (var_ttemp * var_ttemp);
        let assign13050_e7376: f64 = (var_ktnom * var_ktnom);
        let assign13050_e7377: f64 = (assign13050_e7373 - assign13050_e7376);
        (assign13050_e7377, ((var_ttemp_dn0 * var_ttemp) + (var_ttemp * var_ttemp_dn0)), ((var_ttemp_dn2 * var_ttemp) + (var_ttemp * var_ttemp_dn2)), ((var_ttemp_dn4 * var_ttemp) + (var_ttemp * var_ttemp_dn4)), ((var_ttemp_dn5 * var_ttemp) + (var_ttemp * var_ttemp_dn5)), ((var_ttemp_dn6 * var_ttemp) + (var_ttemp * var_ttemp_dn6)), ((var_ttemp_dn7 * var_ttemp) + (var_ttemp * var_ttemp_dn7)), ((var_ttemp_dn8 * var_ttemp) + (var_ttemp * var_ttemp_dn8)), ((var_ttemp_dn9 * var_ttemp) + (var_ttemp * var_ttemp_dn9)), ((var_ttemp_dn10 * var_ttemp) + (var_ttemp * var_ttemp_dn10)), ((var_ttemp_dn11 * var_ttemp) + (var_ttemp * var_ttemp_dn11)), ((var_ttemp_dn14 * var_ttemp) + (var_ttemp * var_ttemp_dn14)),)
    } else {
        (var_tdiff_2, var_tdiff_2_dn0, var_tdiff_2_dn2, var_tdiff_2_dn4, var_tdiff_2_dn5, var_tdiff_2_dn6, var_tdiff_2_dn7, var_tdiff_2_dn8, var_tdiff_2_dn9, var_tdiff_2_dn10, var_tdiff_2_dn11, var_tdiff_2_dn14,)
    }
};
        var_tdiff_2 = assign13050_e7379;
        var_tdiff_2_dn0 = assign13050_e7379_d_n0;
        var_tdiff_2_dn2 = assign13050_e7379_d_n2;
        var_tdiff_2_dn4 = assign13050_e7379_d_n4;
        var_tdiff_2_dn5 = assign13050_e7379_d_n5;
        var_tdiff_2_dn6 = assign13050_e7379_d_n6;
        var_tdiff_2_dn7 = assign13050_e7379_d_n7;
        var_tdiff_2_dn8 = assign13050_e7379_d_n8;
        var_tdiff_2_dn9 = assign13050_e7379_d_n9;
        var_tdiff_2_dn10 = assign13050_e7379_d_n10;
        var_tdiff_2_dn11 = assign13050_e7379_d_n11;
        var_tdiff_2_dn14 = assign13050_e7379_d_n14;
        var_tdiff_2_rv = 0.0;

        let (assign13060_e7385, assign13060_e7385_d_n0, assign13060_e7385_d_n2, assign13060_e7385_d_n4, assign13060_e7385_d_n5, assign13060_e7385_d_n6, assign13060_e7385_d_n7, assign13060_e7385_d_n8, assign13060_e7385_d_n9, assign13060_e7385_d_n10, assign13060_e7385_d_n11, assign13060_e7385_d_n14,) = {
    if (var_guard291 != 0.0) {
        let assign13060_e7383: f64 = (var_ttemp / var_ktnom);
        (assign13060_e7383, (var_ttemp_dn0 / var_ktnom), (var_ttemp_dn2 / var_ktnom), (var_ttemp_dn4 / var_ktnom), (var_ttemp_dn5 / var_ktnom), (var_ttemp_dn6 / var_ktnom), (var_ttemp_dn7 / var_ktnom), (var_ttemp_dn8 / var_ktnom), (var_ttemp_dn9 / var_ktnom), (var_ttemp_dn10 / var_ktnom), (var_ttemp_dn11 / var_ktnom), (var_ttemp_dn14 / var_ktnom),)
    } else {
        (var_tratio, var_tratio_dn0, var_tratio_dn2, var_tratio_dn4, var_tratio_dn5, var_tratio_dn6, var_tratio_dn7, var_tratio_dn8, var_tratio_dn9, var_tratio_dn10, var_tratio_dn11, var_tratio_dn14,)
    }
};
        var_tratio = assign13060_e7385;
        var_tratio_dn0 = assign13060_e7385_d_n0;
        var_tratio_dn2 = assign13060_e7385_d_n2;
        var_tratio_dn4 = assign13060_e7385_d_n4;
        var_tratio_dn5 = assign13060_e7385_d_n5;
        var_tratio_dn6 = assign13060_e7385_d_n6;
        var_tratio_dn7 = assign13060_e7385_d_n7;
        var_tratio_dn8 = assign13060_e7385_d_n8;
        var_tratio_dn9 = assign13060_e7385_d_n9;
        var_tratio_dn10 = assign13060_e7385_d_n10;
        var_tratio_dn11 = assign13060_e7385_d_n11;
        var_tratio_dn14 = assign13060_e7385_d_n14;
        var_tratio_rv = 0.0;

        let (assign13070_e7390, assign13070_e7390_d_n0, assign13070_e7390_d_n2, assign13070_e7390_d_n4, assign13070_e7390_d_n5, assign13070_e7390_d_n6, assign13070_e7390_d_n7, assign13070_e7390_d_n8, assign13070_e7390_d_n9, assign13070_e7390_d_n10, assign13070_e7390_d_n11, assign13070_e7390_d_n14,) = {
    if (var_guard291 != 0.0) {
        let assign13070_e7388: f64 = (var_tratio).ln();
        (assign13070_e7388, (var_tratio_dn0 / var_tratio), (var_tratio_dn2 / var_tratio), (var_tratio_dn4 / var_tratio), (var_tratio_dn5 / var_tratio), (var_tratio_dn6 / var_tratio), (var_tratio_dn7 / var_tratio), (var_tratio_dn8 / var_tratio), (var_tratio_dn9 / var_tratio), (var_tratio_dn10 / var_tratio), (var_tratio_dn11 / var_tratio), (var_tratio_dn14 / var_tratio),)
    } else {
        (var_log_tratio, var_log_tratio_dn0, var_log_tratio_dn2, var_log_tratio_dn4, var_log_tratio_dn5, var_log_tratio_dn6, var_log_tratio_dn7, var_log_tratio_dn8, var_log_tratio_dn9, var_log_tratio_dn10, var_log_tratio_dn11, var_log_tratio_dn14,)
    }
};
        var_log_tratio = assign13070_e7390;
        var_log_tratio_dn0 = assign13070_e7390_d_n0;
        var_log_tratio_dn2 = assign13070_e7390_d_n2;
        var_log_tratio_dn4 = assign13070_e7390_d_n4;
        var_log_tratio_dn5 = assign13070_e7390_d_n5;
        var_log_tratio_dn6 = assign13070_e7390_d_n6;
        var_log_tratio_dn7 = assign13070_e7390_d_n7;
        var_log_tratio_dn8 = assign13070_e7390_d_n8;
        var_log_tratio_dn9 = assign13070_e7390_d_n9;
        var_log_tratio_dn10 = assign13070_e7390_d_n10;
        var_log_tratio_dn11 = assign13070_e7390_d_n11;
        var_log_tratio_dn14 = assign13070_e7390_d_n14;
        var_log_tratio_rv = 0.0;

        let (assign13080_e7402, assign13080_e7402_d_n0, assign13080_e7402_d_n2, assign13080_e7402_d_n4, assign13080_e7402_d_n5, assign13080_e7402_d_n6, assign13080_e7402_d_n7, assign13080_e7402_d_n8, assign13080_e7402_d_n9, assign13080_e7402_d_n10, assign13080_e7402_d_n11, assign13080_e7402_d_n14,) = {
    if (var_guard291 != 0.0) {
        let assign13080_e7395: f64 = (var_uc_bgtmp1 * var_tdiff);
        let assign13080_e7396: f64 = (var_egtnom - assign13080_e7395);
        let assign13080_e7399: f64 = (var_uc_bgtmp2 * var_tdiff_2);
        let assign13080_e7400: f64 = (assign13080_e7396 - assign13080_e7399);
        (assign13080_e7400, ((-(var_uc_bgtmp1 * var_tdiff_dn0)) - (var_uc_bgtmp2 * var_tdiff_2_dn0)), ((-(var_uc_bgtmp1 * var_tdiff_dn2)) - (var_uc_bgtmp2 * var_tdiff_2_dn2)), ((-(var_uc_bgtmp1 * var_tdiff_dn4)) - (var_uc_bgtmp2 * var_tdiff_2_dn4)), ((-(var_uc_bgtmp1 * var_tdiff_dn5)) - (var_uc_bgtmp2 * var_tdiff_2_dn5)), ((-(var_uc_bgtmp1 * var_tdiff_dn6)) - (var_uc_bgtmp2 * var_tdiff_2_dn6)), ((-(var_uc_bgtmp1 * var_tdiff_dn7)) - (var_uc_bgtmp2 * var_tdiff_2_dn7)), ((-(var_uc_bgtmp1 * var_tdiff_dn8)) - (var_uc_bgtmp2 * var_tdiff_2_dn8)), ((-(var_uc_bgtmp1 * var_tdiff_dn9)) - (var_uc_bgtmp2 * var_tdiff_2_dn9)), ((-(var_uc_bgtmp1 * var_tdiff_dn10)) - (var_uc_bgtmp2 * var_tdiff_2_dn10)), ((-(var_uc_bgtmp1 * var_tdiff_dn11)) - (var_uc_bgtmp2 * var_tdiff_2_dn11)), ((-(var_uc_bgtmp1 * var_tdiff_dn14)) - (var_uc_bgtmp2 * var_tdiff_2_dn14)),)
    } else {
        (var_eg, var_eg_dn0, var_eg_dn2, var_eg_dn4, var_eg_dn5, var_eg_dn6, var_eg_dn7, var_eg_dn8, var_eg_dn9, var_eg_dn10, var_eg_dn11, var_eg_dn14,)
    }
};
        var_eg = assign13080_e7402;
        var_eg_dn0 = assign13080_e7402_d_n0;
        var_eg_dn2 = assign13080_e7402_d_n2;
        var_eg_dn4 = assign13080_e7402_d_n4;
        var_eg_dn5 = assign13080_e7402_d_n5;
        var_eg_dn6 = assign13080_e7402_d_n6;
        var_eg_dn7 = assign13080_e7402_d_n7;
        var_eg_dn8 = assign13080_e7402_d_n8;
        var_eg_dn9 = assign13080_e7402_d_n9;
        var_eg_dn10 = assign13080_e7402_d_n10;
        var_eg_dn11 = assign13080_e7402_d_n11;
        var_eg_dn14 = assign13080_e7402_d_n14;
        var_eg_rv = 0.0;

        let (assign13090_e7407, assign13090_e7407_d_n0, assign13090_e7407_d_n2, assign13090_e7407_d_n4, assign13090_e7407_d_n5, assign13090_e7407_d_n6, assign13090_e7407_d_n7, assign13090_e7407_d_n8, assign13090_e7407_d_n9, assign13090_e7407_d_n10, assign13090_e7407_d_n11, assign13090_e7407_d_n14,) = {
    if (var_guard291 != 0.0) {
        let assign13090_e7405: f64 = (var_eg).sqrt();
        (assign13090_e7405, (var_eg_dn0 / (2.0 * assign13090_e7405)), (var_eg_dn2 / (2.0 * assign13090_e7405)), (var_eg_dn4 / (2.0 * assign13090_e7405)), (var_eg_dn5 / (2.0 * assign13090_e7405)), (var_eg_dn6 / (2.0 * assign13090_e7405)), (var_eg_dn7 / (2.0 * assign13090_e7405)), (var_eg_dn8 / (2.0 * assign13090_e7405)), (var_eg_dn9 / (2.0 * assign13090_e7405)), (var_eg_dn10 / (2.0 * assign13090_e7405)), (var_eg_dn11 / (2.0 * assign13090_e7405)), (var_eg_dn14 / (2.0 * assign13090_e7405)),)
    } else {
        (var_sqrt_eg, var_sqrt_eg_dn0, var_sqrt_eg_dn2, var_sqrt_eg_dn4, var_sqrt_eg_dn5, var_sqrt_eg_dn6, var_sqrt_eg_dn7, var_sqrt_eg_dn8, var_sqrt_eg_dn9, var_sqrt_eg_dn10, var_sqrt_eg_dn11, var_sqrt_eg_dn14,)
    }
};
        var_sqrt_eg = assign13090_e7407;
        var_sqrt_eg_dn0 = assign13090_e7407_d_n0;
        var_sqrt_eg_dn2 = assign13090_e7407_d_n2;
        var_sqrt_eg_dn4 = assign13090_e7407_d_n4;
        var_sqrt_eg_dn5 = assign13090_e7407_d_n5;
        var_sqrt_eg_dn6 = assign13090_e7407_d_n6;
        var_sqrt_eg_dn7 = assign13090_e7407_d_n7;
        var_sqrt_eg_dn8 = assign13090_e7407_d_n8;
        var_sqrt_eg_dn9 = assign13090_e7407_d_n9;
        var_sqrt_eg_dn10 = assign13090_e7407_d_n10;
        var_sqrt_eg_dn11 = assign13090_e7407_d_n11;
        var_sqrt_eg_dn14 = assign13090_e7407_d_n14;
        var_sqrt_eg_rv = 0.0;

        let (assign13100_e7413, assign13100_e7413_d_n0, assign13100_e7413_d_n2, assign13100_e7413_d_n4, assign13100_e7413_d_n5, assign13100_e7413_d_n6, assign13100_e7413_d_n7, assign13100_e7413_d_n8, assign13100_e7413_d_n9, assign13100_e7413_d_n10, assign13100_e7413_d_n11, assign13100_e7413_d_n14,) = {
    if (var_guard291 != 0.0) {
        let assign13100_e7411: f64 = (1.0 / var_ttemp);
        (assign13100_e7411, (-(var_ttemp_dn0 / (var_ttemp * var_ttemp))), (-(var_ttemp_dn2 / (var_ttemp * var_ttemp))), (-(var_ttemp_dn4 / (var_ttemp * var_ttemp))), (-(var_ttemp_dn5 / (var_ttemp * var_ttemp))), (-(var_ttemp_dn6 / (var_ttemp * var_ttemp))), (-(var_ttemp_dn7 / (var_ttemp * var_ttemp))), (-(var_ttemp_dn8 / (var_ttemp * var_ttemp))), (-(var_ttemp_dn9 / (var_ttemp * var_ttemp))), (-(var_ttemp_dn10 / (var_ttemp * var_ttemp))), (-(var_ttemp_dn11 / (var_ttemp * var_ttemp))), (-(var_ttemp_dn14 / (var_ttemp * var_ttemp))),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn11, var_t1_dn14,)
    }
};
        var_t1 = assign13100_e7413;
        var_t1_dn0 = assign13100_e7413_d_n0;
        var_t1_dn2 = assign13100_e7413_d_n2;
        var_t1_dn4 = assign13100_e7413_d_n4;
        var_t1_dn5 = assign13100_e7413_d_n5;
        var_t1_dn6 = assign13100_e7413_d_n6;
        var_t1_dn7 = assign13100_e7413_d_n7;
        var_t1_dn8 = assign13100_e7413_d_n8;
        var_t1_dn9 = assign13100_e7413_d_n9;
        var_t1_dn10 = assign13100_e7413_d_n10;
        var_t1_dn11 = assign13100_e7413_d_n11;
        var_t1_dn14 = assign13100_e7413_d_n14;
        var_t1_rv = 0.0;

        let (assign13110_e7419, assign13110_e7419_d_n0, assign13110_e7419_d_n2, assign13110_e7419_d_n4, assign13110_e7419_d_n5, assign13110_e7419_d_n6, assign13110_e7419_d_n7, assign13110_e7419_d_n8, assign13110_e7419_d_n9, assign13110_e7419_d_n10, assign13110_e7419_d_n11, assign13110_e7419_d_n14,) = {
    if (var_guard291 != 0.0) {
        let assign13110_e7417: f64 = (1.0 / var_ktnom);
        (assign13110_e7417, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn11, var_t2_dn14,)
    }
};
        var_t2 = assign13110_e7419;
        var_t2_dn0 = assign13110_e7419_d_n0;
        var_t2_dn2 = assign13110_e7419_d_n2;
        var_t2_dn4 = assign13110_e7419_d_n4;
        var_t2_dn5 = assign13110_e7419_d_n5;
        var_t2_dn6 = assign13110_e7419_d_n6;
        var_t2_dn7 = assign13110_e7419_d_n7;
        var_t2_dn8 = assign13110_e7419_d_n8;
        var_t2_dn9 = assign13110_e7419_d_n9;
        var_t2_dn10 = assign13110_e7419_d_n10;
        var_t2_dn11 = assign13110_e7419_d_n11;
        var_t2_dn14 = assign13110_e7419_d_n14;
        var_t2_rv = 0.0;

        let (assign13120_e7441, assign13120_e7441_d_n0, assign13120_e7441_d_n2, assign13120_e7441_d_n4, assign13120_e7441_d_n5, assign13120_e7441_d_n6, assign13120_e7441_d_n7, assign13120_e7441_d_n8, assign13120_e7441_d_n9, assign13120_e7441_d_n10, assign13120_e7441_d_n11, assign13120_e7441_d_n14,) = {
    if (var_guard291 != 0.0) {
        let assign13120_e7423: f64 = (var_egtnom + p.p259);
        let assign13120_e7427: f64 = (var_t1 - var_t2);
        let assign13120_e7428: f64 = (p.p260 * assign13120_e7427);
        let assign13120_e7429: f64 = (assign13120_e7423 + assign13120_e7428);
        let assign13120_e7433: f64 = (var_t1 * var_t1);
        let assign13120_e7436: f64 = (var_t2 * var_t2);
        let assign13120_e7437: f64 = (assign13120_e7433 - assign13120_e7436);
        let assign13120_e7438: f64 = (p.p261 * assign13120_e7437);
        let assign13120_e7439: f64 = (assign13120_e7429 + assign13120_e7438);
        (assign13120_e7439, ((p.p260 * (var_t1_dn0 - var_t2_dn0)) + (p.p261 * (((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)) - ((var_t2_dn0 * var_t2) + (var_t2 * var_t2_dn0))))), ((p.p260 * (var_t1_dn2 - var_t2_dn2)) + (p.p261 * (((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)) - ((var_t2_dn2 * var_t2) + (var_t2 * var_t2_dn2))))), ((p.p260 * (var_t1_dn4 - var_t2_dn4)) + (p.p261 * (((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)) - ((var_t2_dn4 * var_t2) + (var_t2 * var_t2_dn4))))), ((p.p260 * (var_t1_dn5 - var_t2_dn5)) + (p.p261 * (((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)) - ((var_t2_dn5 * var_t2) + (var_t2 * var_t2_dn5))))), ((p.p260 * (var_t1_dn6 - var_t2_dn6)) + (p.p261 * (((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)) - ((var_t2_dn6 * var_t2) + (var_t2 * var_t2_dn6))))), ((p.p260 * (var_t1_dn7 - var_t2_dn7)) + (p.p261 * (((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7)) - ((var_t2_dn7 * var_t2) + (var_t2 * var_t2_dn7))))), ((p.p260 * (var_t1_dn8 - var_t2_dn8)) + (p.p261 * (((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)) - ((var_t2_dn8 * var_t2) + (var_t2 * var_t2_dn8))))), ((p.p260 * (var_t1_dn9 - var_t2_dn9)) + (p.p261 * (((var_t1_dn9 * var_t1) + (var_t1 * var_t1_dn9)) - ((var_t2_dn9 * var_t2) + (var_t2 * var_t2_dn9))))), ((p.p260 * (var_t1_dn10 - var_t2_dn10)) + (p.p261 * (((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10)) - ((var_t2_dn10 * var_t2) + (var_t2 * var_t2_dn10))))), ((p.p260 * (var_t1_dn11 - var_t2_dn11)) + (p.p261 * (((var_t1_dn11 * var_t1) + (var_t1 * var_t1_dn11)) - ((var_t2_dn11 * var_t2) + (var_t2 * var_t2_dn11))))), ((p.p260 * (var_t1_dn14 - var_t2_dn14)) + (p.p261 * (((var_t1_dn14 * var_t1) + (var_t1 * var_t1_dn14)) - ((var_t2_dn14 * var_t2) + (var_t2 * var_t2_dn14))))),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn11, var_t3_dn14,)
    }
};
        var_t3 = assign13120_e7441;
        var_t3_dn0 = assign13120_e7441_d_n0;
        var_t3_dn2 = assign13120_e7441_d_n2;
        var_t3_dn4 = assign13120_e7441_d_n4;
        var_t3_dn5 = assign13120_e7441_d_n5;
        var_t3_dn6 = assign13120_e7441_d_n6;
        var_t3_dn7 = assign13120_e7441_d_n7;
        var_t3_dn8 = assign13120_e7441_d_n8;
        var_t3_dn9 = assign13120_e7441_d_n9;
        var_t3_dn10 = assign13120_e7441_d_n10;
        var_t3_dn11 = assign13120_e7441_d_n11;
        var_t3_dn14 = assign13120_e7441_d_n14;
        var_t3_rv = 0.0;

        let (assign13130_e7446, assign13130_e7446_d_n0, assign13130_e7446_d_n2, assign13130_e7446_d_n4, assign13130_e7446_d_n5, assign13130_e7446_d_n6, assign13130_e7446_d_n7, assign13130_e7446_d_n8, assign13130_e7446_d_n9, assign13130_e7446_d_n10, assign13130_e7446_d_n11, assign13130_e7446_d_n14,) = {
    if (var_guard291 != 0.0) {
        let assign13130_e7444: f64 = (var_t3).sqrt();
        (assign13130_e7444, (var_t3_dn0 / (2.0 * assign13130_e7444)), (var_t3_dn2 / (2.0 * assign13130_e7444)), (var_t3_dn4 / (2.0 * assign13130_e7444)), (var_t3_dn5 / (2.0 * assign13130_e7444)), (var_t3_dn6 / (2.0 * assign13130_e7444)), (var_t3_dn7 / (2.0 * assign13130_e7444)), (var_t3_dn8 / (2.0 * assign13130_e7444)), (var_t3_dn9 / (2.0 * assign13130_e7444)), (var_t3_dn10 / (2.0 * assign13130_e7444)), (var_t3_dn11 / (2.0 * assign13130_e7444)), (var_t3_dn14 / (2.0 * assign13130_e7444)),)
    } else {
        (var_egp12, var_egp12_dn0, var_egp12_dn2, var_egp12_dn4, var_egp12_dn5, var_egp12_dn6, var_egp12_dn7, var_egp12_dn8, var_egp12_dn9, var_egp12_dn10, var_egp12_dn11, var_egp12_dn14,)
    }
};
        var_egp12 = assign13130_e7446;
        var_egp12_dn0 = assign13130_e7446_d_n0;
        var_egp12_dn2 = assign13130_e7446_d_n2;
        var_egp12_dn4 = assign13130_e7446_d_n4;
        var_egp12_dn5 = assign13130_e7446_d_n5;
        var_egp12_dn6 = assign13130_e7446_d_n6;
        var_egp12_dn7 = assign13130_e7446_d_n7;
        var_egp12_dn8 = assign13130_e7446_d_n8;
        var_egp12_dn9 = assign13130_e7446_d_n9;
        var_egp12_dn10 = assign13130_e7446_d_n10;
        var_egp12_dn11 = assign13130_e7446_d_n11;
        var_egp12_dn14 = assign13130_e7446_d_n14;
        var_egp12_rv = 0.0;

        let (assign13140_e7452, assign13140_e7452_d_n0, assign13140_e7452_d_n2, assign13140_e7452_d_n4, assign13140_e7452_d_n5, assign13140_e7452_d_n6, assign13140_e7452_d_n7, assign13140_e7452_d_n8, assign13140_e7452_d_n9, assign13140_e7452_d_n10, assign13140_e7452_d_n11, assign13140_e7452_d_n14,) = {
    if (var_guard291 != 0.0) {
        let assign13140_e7450: f64 = (var_t3 * var_egp12);
        (assign13140_e7450, ((var_t3_dn0 * var_egp12) + (var_t3 * var_egp12_dn0)), ((var_t3_dn2 * var_egp12) + (var_t3 * var_egp12_dn2)), ((var_t3_dn4 * var_egp12) + (var_t3 * var_egp12_dn4)), ((var_t3_dn5 * var_egp12) + (var_t3 * var_egp12_dn5)), ((var_t3_dn6 * var_egp12) + (var_t3 * var_egp12_dn6)), ((var_t3_dn7 * var_egp12) + (var_t3 * var_egp12_dn7)), ((var_t3_dn8 * var_egp12) + (var_t3 * var_egp12_dn8)), ((var_t3_dn9 * var_egp12) + (var_t3 * var_egp12_dn9)), ((var_t3_dn10 * var_egp12) + (var_t3 * var_egp12_dn10)), ((var_t3_dn11 * var_egp12) + (var_t3 * var_egp12_dn11)), ((var_t3_dn14 * var_egp12) + (var_t3 * var_egp12_dn14)),)
    } else {
        (var_egp32, var_egp32_dn0, var_egp32_dn2, var_egp32_dn4, var_egp32_dn5, var_egp32_dn6, var_egp32_dn7, var_egp32_dn8, var_egp32_dn9, var_egp32_dn10, var_egp32_dn11, var_egp32_dn14,)
    }
};
        var_egp32 = assign13140_e7452;
        var_egp32_dn0 = assign13140_e7452_d_n0;
        var_egp32_dn2 = assign13140_e7452_d_n2;
        var_egp32_dn4 = assign13140_e7452_d_n4;
        var_egp32_dn5 = assign13140_e7452_d_n5;
        var_egp32_dn6 = assign13140_e7452_d_n6;
        var_egp32_dn7 = assign13140_e7452_d_n7;
        var_egp32_dn8 = assign13140_e7452_d_n8;
        var_egp32_dn9 = assign13140_e7452_d_n9;
        var_egp32_dn10 = assign13140_e7452_d_n10;
        var_egp32_dn11 = assign13140_e7452_d_n11;
        var_egp32_dn14 = assign13140_e7452_d_n14;
        var_egp32_rv = 0.0;

        let (assign13150_e7460, assign13150_e7460_d_n0, assign13150_e7460_d_n2, assign13150_e7460_d_n4, assign13150_e7460_d_n5, assign13150_e7460_d_n6, assign13150_e7460_d_n7, assign13150_e7460_d_n8, assign13150_e7460_d_n9, assign13150_e7460_d_n10, assign13150_e7460_d_n11, assign13150_e7460_d_n14,) = {
    if (var_guard291 != 0.0) {
        let assign13150_e7457: f64 = (1.3806226e-23 * var_ttemp);
        let assign13150_e7458: f64 = (1.6021918e-19 / assign13150_e7457);
        (assign13150_e7458, (-((1.6021918e-19 * (1.3806226e-23 * var_ttemp_dn0)) / (assign13150_e7457 * assign13150_e7457))), (-((1.6021918e-19 * (1.3806226e-23 * var_ttemp_dn2)) / (assign13150_e7457 * assign13150_e7457))), (-((1.6021918e-19 * (1.3806226e-23 * var_ttemp_dn4)) / (assign13150_e7457 * assign13150_e7457))), (-((1.6021918e-19 * (1.3806226e-23 * var_ttemp_dn5)) / (assign13150_e7457 * assign13150_e7457))), (-((1.6021918e-19 * (1.3806226e-23 * var_ttemp_dn6)) / (assign13150_e7457 * assign13150_e7457))), (-((1.6021918e-19 * (1.3806226e-23 * var_ttemp_dn7)) / (assign13150_e7457 * assign13150_e7457))), (-((1.6021918e-19 * (1.3806226e-23 * var_ttemp_dn8)) / (assign13150_e7457 * assign13150_e7457))), (-((1.6021918e-19 * (1.3806226e-23 * var_ttemp_dn9)) / (assign13150_e7457 * assign13150_e7457))), (-((1.6021918e-19 * (1.3806226e-23 * var_ttemp_dn10)) / (assign13150_e7457 * assign13150_e7457))), (-((1.6021918e-19 * (1.3806226e-23 * var_ttemp_dn11)) / (assign13150_e7457 * assign13150_e7457))), (-((1.6021918e-19 * (1.3806226e-23 * var_ttemp_dn14)) / (assign13150_e7457 * assign13150_e7457))),)
    } else {
        (var_beta, var_beta_dn0, var_beta_dn2, var_beta_dn4, var_beta_dn5, var_beta_dn6, var_beta_dn7, var_beta_dn8, var_beta_dn9, var_beta_dn10, var_beta_dn11, var_beta_dn14,)
    }
};
        var_beta = assign13150_e7460;
        var_beta_dn0 = assign13150_e7460_d_n0;
        var_beta_dn2 = assign13150_e7460_d_n2;
        var_beta_dn4 = assign13150_e7460_d_n4;
        var_beta_dn5 = assign13150_e7460_d_n5;
        var_beta_dn6 = assign13150_e7460_d_n6;
        var_beta_dn7 = assign13150_e7460_d_n7;
        var_beta_dn8 = assign13150_e7460_d_n8;
        var_beta_dn9 = assign13150_e7460_d_n9;
        var_beta_dn10 = assign13150_e7460_d_n10;
        var_beta_dn11 = assign13150_e7460_d_n11;
        var_beta_dn14 = assign13150_e7460_d_n14;
        var_beta_rv = 0.0;

        let (assign13160_e7466, assign13160_e7466_d_n0, assign13160_e7466_d_n2, assign13160_e7466_d_n4, assign13160_e7466_d_n5, assign13160_e7466_d_n6, assign13160_e7466_d_n7, assign13160_e7466_d_n8, assign13160_e7466_d_n9, assign13160_e7466_d_n10, assign13160_e7466_d_n11, assign13160_e7466_d_n14,) = {
    if (var_guard291 != 0.0) {
        let assign13160_e7464: f64 = (1.0 / var_beta);
        (assign13160_e7464, (-(var_beta_dn0 / (var_beta * var_beta))), (-(var_beta_dn2 / (var_beta * var_beta))), (-(var_beta_dn4 / (var_beta * var_beta))), (-(var_beta_dn5 / (var_beta * var_beta))), (-(var_beta_dn6 / (var_beta * var_beta))), (-(var_beta_dn7 / (var_beta * var_beta))), (-(var_beta_dn8 / (var_beta * var_beta))), (-(var_beta_dn9 / (var_beta * var_beta))), (-(var_beta_dn10 / (var_beta * var_beta))), (-(var_beta_dn11 / (var_beta * var_beta))), (-(var_beta_dn14 / (var_beta * var_beta))),)
    } else {
        (var_beta_inv, var_beta_inv_dn0, var_beta_inv_dn2, var_beta_inv_dn4, var_beta_inv_dn5, var_beta_inv_dn6, var_beta_inv_dn7, var_beta_inv_dn8, var_beta_inv_dn9, var_beta_inv_dn10, var_beta_inv_dn11, var_beta_inv_dn14,)
    }
};
        var_beta_inv = assign13160_e7466;
        var_beta_inv_dn0 = assign13160_e7466_d_n0;
        var_beta_inv_dn2 = assign13160_e7466_d_n2;
        var_beta_inv_dn4 = assign13160_e7466_d_n4;
        var_beta_inv_dn5 = assign13160_e7466_d_n5;
        var_beta_inv_dn6 = assign13160_e7466_d_n6;
        var_beta_inv_dn7 = assign13160_e7466_d_n7;
        var_beta_inv_dn8 = assign13160_e7466_d_n8;
        var_beta_inv_dn9 = assign13160_e7466_d_n9;
        var_beta_inv_dn10 = assign13160_e7466_d_n10;
        var_beta_inv_dn11 = assign13160_e7466_d_n11;
        var_beta_inv_dn14 = assign13160_e7466_d_n14;
        var_beta_inv_rv = 0.0;

        let (assign13170_e7472, assign13170_e7472_d_n0, assign13170_e7472_d_n2, assign13170_e7472_d_n4, assign13170_e7472_d_n5, assign13170_e7472_d_n6, assign13170_e7472_d_n7, assign13170_e7472_d_n8, assign13170_e7472_d_n9, assign13170_e7472_d_n10, assign13170_e7472_d_n11, assign13170_e7472_d_n14,) = {
    if (var_guard291 != 0.0) {
        let assign13170_e7470: f64 = (var_beta * var_beta);
        (assign13170_e7470, ((var_beta_dn0 * var_beta) + (var_beta * var_beta_dn0)), ((var_beta_dn2 * var_beta) + (var_beta * var_beta_dn2)), ((var_beta_dn4 * var_beta) + (var_beta * var_beta_dn4)), ((var_beta_dn5 * var_beta) + (var_beta * var_beta_dn5)), ((var_beta_dn6 * var_beta) + (var_beta * var_beta_dn6)), ((var_beta_dn7 * var_beta) + (var_beta * var_beta_dn7)), ((var_beta_dn8 * var_beta) + (var_beta * var_beta_dn8)), ((var_beta_dn9 * var_beta) + (var_beta * var_beta_dn9)), ((var_beta_dn10 * var_beta) + (var_beta * var_beta_dn10)), ((var_beta_dn11 * var_beta) + (var_beta * var_beta_dn11)), ((var_beta_dn14 * var_beta) + (var_beta * var_beta_dn14)),)
    } else {
        (var_beta2, var_beta2_dn0, var_beta2_dn2, var_beta2_dn4, var_beta2_dn5, var_beta2_dn6, var_beta2_dn7, var_beta2_dn8, var_beta2_dn9, var_beta2_dn10, var_beta2_dn11, var_beta2_dn14,)
    }
};
        var_beta2 = assign13170_e7472;
        var_beta2_dn0 = assign13170_e7472_d_n0;
        var_beta2_dn2 = assign13170_e7472_d_n2;
        var_beta2_dn4 = assign13170_e7472_d_n4;
        var_beta2_dn5 = assign13170_e7472_d_n5;
        var_beta2_dn6 = assign13170_e7472_d_n6;
        var_beta2_dn7 = assign13170_e7472_d_n7;
        var_beta2_dn8 = assign13170_e7472_d_n8;
        var_beta2_dn9 = assign13170_e7472_d_n9;
        var_beta2_dn10 = assign13170_e7472_d_n10;
        var_beta2_dn11 = assign13170_e7472_d_n11;
        var_beta2_dn14 = assign13170_e7472_d_n14;
        var_beta2_rv = 0.0;

        let (assign13180_e7480,) = {
    if (var_guard291 != 0.0) {
        let assign13180_e7477: f64 = (1.3806226e-23 * var_ktnom);
        let assign13180_e7478: f64 = (1.6021918e-19 / assign13180_e7477);
        (assign13180_e7478,)
    } else {
        (var_betatnom,)
    }
};
        var_betatnom = assign13180_e7480;
        var_betatnom_rv = 0.0;

        *var_beta_slot = var_beta;
        *var_beta2_slot = var_beta2;
        *var_beta2_dn0_slot = var_beta2_dn0;
        *var_beta2_dn10_slot = var_beta2_dn10;
        *var_beta2_dn11_slot = var_beta2_dn11;
        *var_beta2_dn14_slot = var_beta2_dn14;
        *var_beta2_dn2_slot = var_beta2_dn2;
        *var_beta2_dn4_slot = var_beta2_dn4;
        *var_beta2_dn5_slot = var_beta2_dn5;
        *var_beta2_dn6_slot = var_beta2_dn6;
        *var_beta2_dn7_slot = var_beta2_dn7;
        *var_beta2_dn8_slot = var_beta2_dn8;
        *var_beta2_dn9_slot = var_beta2_dn9;
        *var_beta2_rv_slot = var_beta2_rv;
        *var_beta_dn0_slot = var_beta_dn0;
        *var_beta_dn10_slot = var_beta_dn10;
        *var_beta_dn11_slot = var_beta_dn11;
        *var_beta_dn14_slot = var_beta_dn14;
        *var_beta_dn2_slot = var_beta_dn2;
        *var_beta_dn4_slot = var_beta_dn4;
        *var_beta_dn5_slot = var_beta_dn5;
        *var_beta_dn6_slot = var_beta_dn6;
        *var_beta_dn7_slot = var_beta_dn7;
        *var_beta_dn8_slot = var_beta_dn8;
        *var_beta_dn9_slot = var_beta_dn9;
        *var_beta_inv_slot = var_beta_inv;
        *var_beta_inv_dn0_slot = var_beta_inv_dn0;
        *var_beta_inv_dn10_slot = var_beta_inv_dn10;
        *var_beta_inv_dn11_slot = var_beta_inv_dn11;
        *var_beta_inv_dn14_slot = var_beta_inv_dn14;
        *var_beta_inv_dn2_slot = var_beta_inv_dn2;
        *var_beta_inv_dn4_slot = var_beta_inv_dn4;
        *var_beta_inv_dn5_slot = var_beta_inv_dn5;
        *var_beta_inv_dn6_slot = var_beta_inv_dn6;
        *var_beta_inv_dn7_slot = var_beta_inv_dn7;
        *var_beta_inv_dn8_slot = var_beta_inv_dn8;
        *var_beta_inv_dn9_slot = var_beta_inv_dn9;
        *var_beta_inv_rv_slot = var_beta_inv_rv;
        *var_beta_rv_slot = var_beta_rv;
        *var_betatnom_slot = var_betatnom;
        *var_betatnom_rv_slot = var_betatnom_rv;
        *var_cnst0over_slot = var_cnst0over;
        *var_cnst0over_dn0_slot = var_cnst0over_dn0;
        *var_cnst0over_dn10_slot = var_cnst0over_dn10;
        *var_cnst0over_dn11_slot = var_cnst0over_dn11;
        *var_cnst0over_dn14_slot = var_cnst0over_dn14;
        *var_cnst0over_dn2_slot = var_cnst0over_dn2;
        *var_cnst0over_dn4_slot = var_cnst0over_dn4;
        *var_cnst0over_dn5_slot = var_cnst0over_dn5;
        *var_cnst0over_dn6_slot = var_cnst0over_dn6;
        *var_cnst0over_dn7_slot = var_cnst0over_dn7;
        *var_cnst0over_dn8_slot = var_cnst0over_dn8;
        *var_cnst0over_dn9_slot = var_cnst0over_dn9;
        *var_cnst0over_rv_slot = var_cnst0over_rv;
        *var_cnst0overs_slot = var_cnst0overs;
        *var_cnst0overs_dn0_slot = var_cnst0overs_dn0;
        *var_cnst0overs_dn10_slot = var_cnst0overs_dn10;
        *var_cnst0overs_dn11_slot = var_cnst0overs_dn11;
        *var_cnst0overs_dn14_slot = var_cnst0overs_dn14;
        *var_cnst0overs_dn2_slot = var_cnst0overs_dn2;
        *var_cnst0overs_dn4_slot = var_cnst0overs_dn4;
        *var_cnst0overs_dn5_slot = var_cnst0overs_dn5;
        *var_cnst0overs_dn6_slot = var_cnst0overs_dn6;
        *var_cnst0overs_dn7_slot = var_cnst0overs_dn7;
        *var_cnst0overs_dn8_slot = var_cnst0overs_dn8;
        *var_cnst0overs_dn9_slot = var_cnst0overs_dn9;
        *var_cnst0overs_rv_slot = var_cnst0overs_rv;
        *var_eg_slot = var_eg;
        *var_eg_dn0_slot = var_eg_dn0;
        *var_eg_dn10_slot = var_eg_dn10;
        *var_eg_dn11_slot = var_eg_dn11;
        *var_eg_dn14_slot = var_eg_dn14;
        *var_eg_dn2_slot = var_eg_dn2;
        *var_eg_dn4_slot = var_eg_dn4;
        *var_eg_dn5_slot = var_eg_dn5;
        *var_eg_dn6_slot = var_eg_dn6;
        *var_eg_dn7_slot = var_eg_dn7;
        *var_eg_dn8_slot = var_eg_dn8;
        *var_eg_dn9_slot = var_eg_dn9;
        *var_eg_rv_slot = var_eg_rv;
        *var_egp12_slot = var_egp12;
        *var_egp12_dn0_slot = var_egp12_dn0;
        *var_egp12_dn10_slot = var_egp12_dn10;
        *var_egp12_dn11_slot = var_egp12_dn11;
        *var_egp12_dn14_slot = var_egp12_dn14;
        *var_egp12_dn2_slot = var_egp12_dn2;
        *var_egp12_dn4_slot = var_egp12_dn4;
        *var_egp12_dn5_slot = var_egp12_dn5;
        *var_egp12_dn6_slot = var_egp12_dn6;
        *var_egp12_dn7_slot = var_egp12_dn7;
        *var_egp12_dn8_slot = var_egp12_dn8;
        *var_egp12_dn9_slot = var_egp12_dn9;
        *var_egp12_rv_slot = var_egp12_rv;
        *var_egp32_slot = var_egp32;
        *var_egp32_dn0_slot = var_egp32_dn0;
        *var_egp32_dn10_slot = var_egp32_dn10;
        *var_egp32_dn11_slot = var_egp32_dn11;
        *var_egp32_dn14_slot = var_egp32_dn14;
        *var_egp32_dn2_slot = var_egp32_dn2;
        *var_egp32_dn4_slot = var_egp32_dn4;
        *var_egp32_dn5_slot = var_egp32_dn5;
        *var_egp32_dn6_slot = var_egp32_dn6;
        *var_egp32_dn7_slot = var_egp32_dn7;
        *var_egp32_dn8_slot = var_egp32_dn8;
        *var_egp32_dn9_slot = var_egp32_dn9;
        *var_egp32_rv_slot = var_egp32_rv;
        *var_log_tratio_slot = var_log_tratio;
        *var_log_tratio_dn0_slot = var_log_tratio_dn0;
        *var_log_tratio_dn10_slot = var_log_tratio_dn10;
        *var_log_tratio_dn11_slot = var_log_tratio_dn11;
        *var_log_tratio_dn14_slot = var_log_tratio_dn14;
        *var_log_tratio_dn2_slot = var_log_tratio_dn2;
        *var_log_tratio_dn4_slot = var_log_tratio_dn4;
        *var_log_tratio_dn5_slot = var_log_tratio_dn5;
        *var_log_tratio_dn6_slot = var_log_tratio_dn6;
        *var_log_tratio_dn7_slot = var_log_tratio_dn7;
        *var_log_tratio_dn8_slot = var_log_tratio_dn8;
        *var_log_tratio_dn9_slot = var_log_tratio_dn9;
        *var_log_tratio_rv_slot = var_log_tratio_rv;
        *var_sqrt_eg_slot = var_sqrt_eg;
        *var_sqrt_eg_dn0_slot = var_sqrt_eg_dn0;
        *var_sqrt_eg_dn10_slot = var_sqrt_eg_dn10;
        *var_sqrt_eg_dn11_slot = var_sqrt_eg_dn11;
        *var_sqrt_eg_dn14_slot = var_sqrt_eg_dn14;
        *var_sqrt_eg_dn2_slot = var_sqrt_eg_dn2;
        *var_sqrt_eg_dn4_slot = var_sqrt_eg_dn4;
        *var_sqrt_eg_dn5_slot = var_sqrt_eg_dn5;
        *var_sqrt_eg_dn6_slot = var_sqrt_eg_dn6;
        *var_sqrt_eg_dn7_slot = var_sqrt_eg_dn7;
        *var_sqrt_eg_dn8_slot = var_sqrt_eg_dn8;
        *var_sqrt_eg_dn9_slot = var_sqrt_eg_dn9;
        *var_sqrt_eg_rv_slot = var_sqrt_eg_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn11_slot = var_t1_dn11;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn11_slot = var_t2_dn11;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t2_rv_slot = var_t2_rv;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn10_slot = var_t3_dn10;
        *var_t3_dn11_slot = var_t3_dn11;
        *var_t3_dn14_slot = var_t3_dn14;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t3_rv_slot = var_t3_rv;
        *var_tdiff_slot = var_tdiff;
        *var_tdiff0_slot = var_tdiff0;
        *var_tdiff0_2_slot = var_tdiff0_2;
        *var_tdiff0_2_dn0_slot = var_tdiff0_2_dn0;
        *var_tdiff0_2_dn10_slot = var_tdiff0_2_dn10;
        *var_tdiff0_2_dn11_slot = var_tdiff0_2_dn11;
        *var_tdiff0_2_dn14_slot = var_tdiff0_2_dn14;
        *var_tdiff0_2_dn2_slot = var_tdiff0_2_dn2;
        *var_tdiff0_2_dn4_slot = var_tdiff0_2_dn4;
        *var_tdiff0_2_dn5_slot = var_tdiff0_2_dn5;
        *var_tdiff0_2_dn6_slot = var_tdiff0_2_dn6;
        *var_tdiff0_2_dn7_slot = var_tdiff0_2_dn7;
        *var_tdiff0_2_dn8_slot = var_tdiff0_2_dn8;
        *var_tdiff0_2_dn9_slot = var_tdiff0_2_dn9;
        *var_tdiff0_2_rv_slot = var_tdiff0_2_rv;
        *var_tdiff0_dn0_slot = var_tdiff0_dn0;
        *var_tdiff0_dn10_slot = var_tdiff0_dn10;
        *var_tdiff0_dn11_slot = var_tdiff0_dn11;
        *var_tdiff0_dn14_slot = var_tdiff0_dn14;
        *var_tdiff0_dn2_slot = var_tdiff0_dn2;
        *var_tdiff0_dn4_slot = var_tdiff0_dn4;
        *var_tdiff0_dn5_slot = var_tdiff0_dn5;
        *var_tdiff0_dn6_slot = var_tdiff0_dn6;
        *var_tdiff0_dn7_slot = var_tdiff0_dn7;
        *var_tdiff0_dn8_slot = var_tdiff0_dn8;
        *var_tdiff0_dn9_slot = var_tdiff0_dn9;
        *var_tdiff0_rv_slot = var_tdiff0_rv;
        *var_tdiff_2_slot = var_tdiff_2;
        *var_tdiff_2_dn0_slot = var_tdiff_2_dn0;
        *var_tdiff_2_dn10_slot = var_tdiff_2_dn10;
        *var_tdiff_2_dn11_slot = var_tdiff_2_dn11;
        *var_tdiff_2_dn14_slot = var_tdiff_2_dn14;
        *var_tdiff_2_dn2_slot = var_tdiff_2_dn2;
        *var_tdiff_2_dn4_slot = var_tdiff_2_dn4;
        *var_tdiff_2_dn5_slot = var_tdiff_2_dn5;
        *var_tdiff_2_dn6_slot = var_tdiff_2_dn6;
        *var_tdiff_2_dn7_slot = var_tdiff_2_dn7;
        *var_tdiff_2_dn8_slot = var_tdiff_2_dn8;
        *var_tdiff_2_dn9_slot = var_tdiff_2_dn9;
        *var_tdiff_2_rv_slot = var_tdiff_2_rv;
        *var_tdiff_dn0_slot = var_tdiff_dn0;
        *var_tdiff_dn10_slot = var_tdiff_dn10;
        *var_tdiff_dn11_slot = var_tdiff_dn11;
        *var_tdiff_dn14_slot = var_tdiff_dn14;
        *var_tdiff_dn2_slot = var_tdiff_dn2;
        *var_tdiff_dn4_slot = var_tdiff_dn4;
        *var_tdiff_dn5_slot = var_tdiff_dn5;
        *var_tdiff_dn6_slot = var_tdiff_dn6;
        *var_tdiff_dn7_slot = var_tdiff_dn7;
        *var_tdiff_dn8_slot = var_tdiff_dn8;
        *var_tdiff_dn9_slot = var_tdiff_dn9;
        *var_tdiff_rv_slot = var_tdiff_rv;
        *var_tratio_slot = var_tratio;
        *var_tratio_dn0_slot = var_tratio_dn0;
        *var_tratio_dn10_slot = var_tratio_dn10;
        *var_tratio_dn11_slot = var_tratio_dn11;
        *var_tratio_dn14_slot = var_tratio_dn14;
        *var_tratio_dn2_slot = var_tratio_dn2;
        *var_tratio_dn4_slot = var_tratio_dn4;
        *var_tratio_dn5_slot = var_tratio_dn5;
        *var_tratio_dn6_slot = var_tratio_dn6;
        *var_tratio_dn7_slot = var_tratio_dn7;
        *var_tratio_dn8_slot = var_tratio_dn8;
        *var_tratio_dn9_slot = var_tratio_dn9;
        *var_tratio_rv_slot = var_tratio_rv;
        *var_ttemp_slot = var_ttemp;
        *var_ttemp0_slot = var_ttemp0;
        *var_ttemp0_dn0_slot = var_ttemp0_dn0;
        *var_ttemp0_dn10_slot = var_ttemp0_dn10;
        *var_ttemp0_dn11_slot = var_ttemp0_dn11;
        *var_ttemp0_dn14_slot = var_ttemp0_dn14;
        *var_ttemp0_dn2_slot = var_ttemp0_dn2;
        *var_ttemp0_dn4_slot = var_ttemp0_dn4;
        *var_ttemp0_dn5_slot = var_ttemp0_dn5;
        *var_ttemp0_dn6_slot = var_ttemp0_dn6;
        *var_ttemp0_dn7_slot = var_ttemp0_dn7;
        *var_ttemp0_dn8_slot = var_ttemp0_dn8;
        *var_ttemp0_dn9_slot = var_ttemp0_dn9;
        *var_ttemp0_rv_slot = var_ttemp0_rv;
        *var_ttemp_dn0_slot = var_ttemp_dn0;
        *var_ttemp_dn10_slot = var_ttemp_dn10;
        *var_ttemp_dn11_slot = var_ttemp_dn11;
        *var_ttemp_dn14_slot = var_ttemp_dn14;
        *var_ttemp_dn2_slot = var_ttemp_dn2;
        *var_ttemp_dn4_slot = var_ttemp_dn4;
        *var_ttemp_dn5_slot = var_ttemp_dn5;
        *var_ttemp_dn6_slot = var_ttemp_dn6;
        *var_ttemp_dn7_slot = var_ttemp_dn7;
        *var_ttemp_dn8_slot = var_ttemp_dn8;
        *var_ttemp_dn9_slot = var_ttemp_dn9;
        *var_ttemp_rv_slot = var_ttemp_rv;
    }
}
