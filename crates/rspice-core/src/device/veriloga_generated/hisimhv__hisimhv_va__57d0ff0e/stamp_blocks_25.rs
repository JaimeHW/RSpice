#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_8(
        locals: &mut StampLocals,
    ) {
        locals.var_vdri = 0.0;
        locals.var_vdri_dn0 = 0.0;
        locals.var_vdri_dn2 = 0.0;
        locals.var_vdri_dn4 = 0.0;
        locals.var_vdri_dn5 = 0.0;
        locals.var_vdri_dn6 = 0.0;
        locals.var_vdri_dn7 = 0.0;
        locals.var_vdri_dn8 = 0.0;
        locals.var_vdri_dn9 = 0.0;
        locals.var_vdri_dn10 = 0.0;
        locals.var_vdri_dn11 = 0.0;
        locals.var_vdri_dn14 = 0.0;
        locals.var_vdri_rv = 0.0;

        locals.var_mu0 = 0.0;
        locals.var_mu0_dn0 = 0.0;
        locals.var_mu0_dn2 = 0.0;
        locals.var_mu0_dn4 = 0.0;
        locals.var_mu0_dn5 = 0.0;
        locals.var_mu0_dn6 = 0.0;
        locals.var_mu0_dn7 = 0.0;
        locals.var_mu0_dn8 = 0.0;
        locals.var_mu0_dn9 = 0.0;
        locals.var_mu0_dn10 = 0.0;
        locals.var_mu0_dn11 = 0.0;
        locals.var_mu0_dn14 = 0.0;
        locals.var_mu0_rv = 0.0;

        locals.var_cx = 0.0;
        locals.var_cx_rv = 0.0;

        locals.var_car = 0.0;
        locals.var_car_rv = 0.0;

        locals.var_xov = 0.0;
        locals.var_xov_dn0 = 0.0;
        locals.var_xov_dn2 = 0.0;
        locals.var_xov_dn4 = 0.0;
        locals.var_xov_dn5 = 0.0;
        locals.var_xov_dn6 = 0.0;
        locals.var_xov_dn7 = 0.0;
        locals.var_xov_dn8 = 0.0;
        locals.var_xov_dn9 = 0.0;
        locals.var_xov_dn10 = 0.0;
        locals.var_xov_dn11 = 0.0;
        locals.var_xov_dn14 = 0.0;
        locals.var_xov_rv = 0.0;

        locals.var_carr = 0.0;
        locals.var_carr_dn0 = 0.0;
        locals.var_carr_dn2 = 0.0;
        locals.var_carr_dn4 = 0.0;
        locals.var_carr_dn5 = 0.0;
        locals.var_carr_dn6 = 0.0;
        locals.var_carr_dn7 = 0.0;
        locals.var_carr_dn8 = 0.0;
        locals.var_carr_dn9 = 0.0;
        locals.var_carr_dn10 = 0.0;
        locals.var_carr_dn11 = 0.0;
        locals.var_carr_dn14 = 0.0;
        locals.var_carr_rv = 0.0;

        locals.var_gd = 0.0;
        locals.var_gd_dn0 = 0.0;
        locals.var_gd_dn2 = 0.0;
        locals.var_gd_dn4 = 0.0;
        locals.var_gd_dn5 = 0.0;
        locals.var_gd_dn6 = 0.0;
        locals.var_gd_dn7 = 0.0;
        locals.var_gd_dn8 = 0.0;
        locals.var_gd_dn9 = 0.0;
        locals.var_gd_dn10 = 0.0;
        locals.var_gd_dn11 = 0.0;
        locals.var_gd_dn14 = 0.0;
        locals.var_gd_rv = 0.0;

        locals.var_vddpz = 0.0;
        locals.var_vddpz_dn0 = 0.0;
        locals.var_vddpz_dn2 = 0.0;
        locals.var_vddpz_dn4 = 0.0;
        locals.var_vddpz_dn5 = 0.0;
        locals.var_vddpz_dn6 = 0.0;
        locals.var_vddpz_dn7 = 0.0;
        locals.var_vddpz_dn8 = 0.0;
        locals.var_vddpz_dn9 = 0.0;
        locals.var_vddpz_dn10 = 0.0;
        locals.var_vddpz_dn11 = 0.0;
        locals.var_vddpz_dn14 = 0.0;
        locals.var_vddpz_rv = 0.0;

        locals.var_arg = 0.0;
        locals.var_arg_dn0 = 0.0;
        locals.var_arg_dn2 = 0.0;
        locals.var_arg_dn4 = 0.0;
        locals.var_arg_dn5 = 0.0;
        locals.var_arg_dn6 = 0.0;
        locals.var_arg_dn7 = 0.0;
        locals.var_arg_dn8 = 0.0;
        locals.var_arg_dn9 = 0.0;
        locals.var_arg_dn10 = 0.0;
        locals.var_arg_dn11 = 0.0;
        locals.var_arg_dn14 = 0.0;
        locals.var_arg_rv = 0.0;

        locals.var_vbd = 0.0;
        locals.var_vbd_dn6 = 0.0;
        locals.var_vbd_dn8 = 0.0;
        locals.var_vbd_dn9 = 0.0;
        locals.var_vbd_rv = 0.0;

        locals.var_vbsi = 0.0;
        locals.var_vbsi_dn8 = 0.0;
        locals.var_vbsi_dn9 = 0.0;
        locals.var_vbsi_rv = 0.0;

        locals.var_vdsi = 0.0;
        locals.var_vdsi_dn6 = 0.0;
        locals.var_vdsi_dn8 = 0.0;
        locals.var_vdsi_rv = 0.0;

        locals.var_vgd = 0.0;
        locals.var_vgd_dn6 = 0.0;
        locals.var_vgd_dn7 = 0.0;
        locals.var_vgd_dn8 = 0.0;
        locals.var_vgd_rv = 0.0;

        locals.var_vgsi = 0.0;
        locals.var_vgsi_dn7 = 0.0;
        locals.var_vgsi_dn8 = 0.0;
        locals.var_vgsi_rv = 0.0;

        locals.var_deltemp = 0.0;
        locals.var_deltemp_dn0 = 0.0;
        locals.var_deltemp_dn2 = 0.0;
        locals.var_deltemp_dn4 = 0.0;
        locals.var_deltemp_dn5 = 0.0;
        locals.var_deltemp_dn6 = 0.0;
        locals.var_deltemp_dn7 = 0.0;
        locals.var_deltemp_dn8 = 0.0;
        locals.var_deltemp_dn9 = 0.0;
        locals.var_deltemp_dn10 = 0.0;
        locals.var_deltemp_dn11 = 0.0;
        locals.var_deltemp_dn14 = 0.0;
        locals.var_deltemp_rv = 0.0;

        locals.var_vdsei = 0.0;
        locals.var_vdsei_dn0 = 0.0;
        locals.var_vdsei_dn2 = 0.0;
        locals.var_vdsei_rv = 0.0;

        locals.var_vgsei = 0.0;
        locals.var_vgsei_dn2 = 0.0;
        locals.var_vgsei_dn7 = 0.0;
        locals.var_vgsei_rv = 0.0;

        locals.var_vbsei = 0.0;
        locals.var_vbsei_dn2 = 0.0;
        locals.var_vbsei_dn9 = 0.0;
        locals.var_vbsei_rv = 0.0;

        locals.var_gth = 0.0;
        locals.var_gth_dn0 = 0.0;
        locals.var_gth_dn2 = 0.0;
        locals.var_gth_dn4 = 0.0;
        locals.var_gth_dn5 = 0.0;
        locals.var_gth_dn6 = 0.0;
        locals.var_gth_dn7 = 0.0;
        locals.var_gth_dn8 = 0.0;
        locals.var_gth_dn9 = 0.0;
        locals.var_gth_dn10 = 0.0;
        locals.var_gth_dn11 = 0.0;
        locals.var_gth_dn14 = 0.0;
        locals.var_gth_rv = 0.0;

        locals.var_qg = 0.0;
        locals.var_qg_dn0 = 0.0;
        locals.var_qg_dn2 = 0.0;
        locals.var_qg_dn4 = 0.0;
        locals.var_qg_dn5 = 0.0;
        locals.var_qg_dn6 = 0.0;
        locals.var_qg_dn7 = 0.0;
        locals.var_qg_dn8 = 0.0;
        locals.var_qg_dn9 = 0.0;
        locals.var_qg_dn10 = 0.0;
        locals.var_qg_dn11 = 0.0;
        locals.var_qg_dn14 = 0.0;
        locals.var_qg_rv = 0.0;

        locals.var_qs = 0.0;
        locals.var_qs_dn0 = 0.0;
        locals.var_qs_dn2 = 0.0;
        locals.var_qs_dn4 = 0.0;
        locals.var_qs_dn5 = 0.0;
        locals.var_qs_dn6 = 0.0;
        locals.var_qs_dn7 = 0.0;
        locals.var_qs_dn8 = 0.0;
        locals.var_qs_dn9 = 0.0;
        locals.var_qs_dn10 = 0.0;
        locals.var_qs_dn11 = 0.0;
        locals.var_qs_dn14 = 0.0;
        locals.var_qs_rv = 0.0;

        locals.var_veffpower = 0.0;
        locals.var_veffpower_dn0 = 0.0;
        locals.var_veffpower_dn2 = 0.0;
        locals.var_veffpower_dn4 = 0.0;
        locals.var_veffpower_dn5 = 0.0;
        locals.var_veffpower_dn6 = 0.0;
        locals.var_veffpower_dn7 = 0.0;
        locals.var_veffpower_dn8 = 0.0;
        locals.var_veffpower_dn9 = 0.0;
        locals.var_veffpower_dn10 = 0.0;
        locals.var_veffpower_dn11 = 0.0;
        locals.var_veffpower_dn14 = 0.0;
        locals.var_veffpower_rv = 0.0;

        locals.var_p = 0.0;
        locals.var_p_dn0 = 0.0;
        locals.var_p_dn2 = 0.0;
        locals.var_p_dn4 = 0.0;
        locals.var_p_dn5 = 0.0;
        locals.var_p_dn6 = 0.0;
        locals.var_p_dn7 = 0.0;
        locals.var_p_dn8 = 0.0;
        locals.var_p_dn9 = 0.0;
        locals.var_p_dn10 = 0.0;
        locals.var_p_dn11 = 0.0;
        locals.var_p_dn14 = 0.0;
        locals.var_p_rv = 0.0;

        locals.var_qi_nqs = 0.0;
        locals.var_qi_nqs_dn12 = 0.0;
        locals.var_qi_nqs_rv = 0.0;

        locals.var_qb_nqs = 0.0;
        locals.var_qb_nqs_dn13 = 0.0;
        locals.var_qb_nqs_rv = 0.0;

        locals.var_qd_nqs = 0.0;
        locals.var_qd_nqs_dn0 = 0.0;
        locals.var_qd_nqs_dn2 = 0.0;
        locals.var_qd_nqs_dn4 = 0.0;
        locals.var_qd_nqs_dn5 = 0.0;
        locals.var_qd_nqs_dn6 = 0.0;
        locals.var_qd_nqs_dn7 = 0.0;
        locals.var_qd_nqs_dn8 = 0.0;
        locals.var_qd_nqs_dn9 = 0.0;
        locals.var_qd_nqs_dn10 = 0.0;
        locals.var_qd_nqs_dn11 = 0.0;
        locals.var_qd_nqs_dn12 = 0.0;
        locals.var_qd_nqs_dn14 = 0.0;
        locals.var_qd_nqs_rv = 0.0;

        locals.var_qs_nqs = 0.0;
        locals.var_qs_nqs_dn0 = 0.0;
        locals.var_qs_nqs_dn2 = 0.0;
        locals.var_qs_nqs_dn4 = 0.0;
        locals.var_qs_nqs_dn5 = 0.0;
        locals.var_qs_nqs_dn6 = 0.0;
        locals.var_qs_nqs_dn7 = 0.0;
        locals.var_qs_nqs_dn8 = 0.0;
        locals.var_qs_nqs_dn9 = 0.0;
        locals.var_qs_nqs_dn10 = 0.0;
        locals.var_qs_nqs_dn11 = 0.0;
        locals.var_qs_nqs_dn12 = 0.0;
        locals.var_qs_nqs_dn14 = 0.0;
        locals.var_qs_nqs_rv = 0.0;

        locals.var_qg_nqs = 0.0;
        locals.var_qg_nqs_dn12 = 0.0;
        locals.var_qg_nqs_dn13 = 0.0;
        locals.var_qg_nqs_rv = 0.0;

        locals.var_cgsb = 0.0;
        locals.var_cgsb_dn0 = 0.0;
        locals.var_cgsb_dn2 = 0.0;
        locals.var_cgsb_dn4 = 0.0;
        locals.var_cgsb_dn5 = 0.0;
        locals.var_cgsb_dn6 = 0.0;
        locals.var_cgsb_dn7 = 0.0;
        locals.var_cgsb_dn8 = 0.0;
        locals.var_cgsb_dn9 = 0.0;
        locals.var_cgsb_dn10 = 0.0;
        locals.var_cgsb_dn11 = 0.0;
        locals.var_cgsb_dn14 = 0.0;
        locals.var_cgsb_rv = 0.0;

        locals.var_ninvde = 0.0;
        locals.var_ninvde_dn0 = 0.0;
        locals.var_ninvde_dn2 = 0.0;
        locals.var_ninvde_dn4 = 0.0;
        locals.var_ninvde_dn5 = 0.0;
        locals.var_ninvde_dn6 = 0.0;
        locals.var_ninvde_dn7 = 0.0;
        locals.var_ninvde_dn8 = 0.0;
        locals.var_ninvde_dn9 = 0.0;
        locals.var_ninvde_dn10 = 0.0;
        locals.var_ninvde_dn11 = 0.0;
        locals.var_ninvde_dn14 = 0.0;
        locals.var_ninvde_rv = 0.0;

        locals.var_ninvdecres = 0.0;
        locals.var_ninvdecres_dn0 = 0.0;
        locals.var_ninvdecres_dn2 = 0.0;
        locals.var_ninvdecres_dn4 = 0.0;
        locals.var_ninvdecres_dn5 = 0.0;
        locals.var_ninvdecres_dn6 = 0.0;
        locals.var_ninvdecres_dn7 = 0.0;
        locals.var_ninvdecres_dn8 = 0.0;
        locals.var_ninvdecres_dn9 = 0.0;
        locals.var_ninvdecres_dn10 = 0.0;
        locals.var_ninvdecres_dn11 = 0.0;
        locals.var_ninvdecres_dn14 = 0.0;
        locals.var_ninvdecres_rv = 0.0;

        locals.var_ninvdehres = 0.0;
        locals.var_ninvdehres_dn0 = 0.0;
        locals.var_ninvdehres_dn2 = 0.0;
        locals.var_ninvdehres_dn4 = 0.0;
        locals.var_ninvdehres_dn5 = 0.0;
        locals.var_ninvdehres_dn6 = 0.0;
        locals.var_ninvdehres_dn7 = 0.0;
        locals.var_ninvdehres_dn8 = 0.0;
        locals.var_ninvdehres_dn9 = 0.0;
        locals.var_ninvdehres_dn10 = 0.0;
        locals.var_ninvdehres_dn11 = 0.0;
        locals.var_ninvdehres_dn14 = 0.0;
        locals.var_ninvdehres_rv = 0.0;

        locals.var_rrdrmue = 0.0;
        locals.var_rrdrmue_dn0 = 0.0;
        locals.var_rrdrmue_dn2 = 0.0;
        locals.var_rrdrmue_dn4 = 0.0;
        locals.var_rrdrmue_dn5 = 0.0;
        locals.var_rrdrmue_dn6 = 0.0;
        locals.var_rrdrmue_dn7 = 0.0;
        locals.var_rrdrmue_dn8 = 0.0;
        locals.var_rrdrmue_dn9 = 0.0;
        locals.var_rrdrmue_dn10 = 0.0;
        locals.var_rrdrmue_dn11 = 0.0;
        locals.var_rrdrmue_dn14 = 0.0;
        locals.var_rrdrmue_rv = 0.0;

        locals.var_rrdrmues = 0.0;
        locals.var_rrdrmues_dn0 = 0.0;
        locals.var_rrdrmues_dn2 = 0.0;
        locals.var_rrdrmues_dn4 = 0.0;
        locals.var_rrdrmues_dn5 = 0.0;
        locals.var_rrdrmues_dn6 = 0.0;
        locals.var_rrdrmues_dn7 = 0.0;
        locals.var_rrdrmues_dn8 = 0.0;
        locals.var_rrdrmues_dn9 = 0.0;
        locals.var_rrdrmues_dn10 = 0.0;
        locals.var_rrdrmues_dn11 = 0.0;
        locals.var_rrdrmues_dn14 = 0.0;
        locals.var_rrdrmues_rv = 0.0;

        locals.var_rrdrvmax = 0.0;
        locals.var_rrdrvmax_dn0 = 0.0;
        locals.var_rrdrvmax_dn2 = 0.0;
        locals.var_rrdrvmax_dn4 = 0.0;
        locals.var_rrdrvmax_dn5 = 0.0;
        locals.var_rrdrvmax_dn6 = 0.0;
        locals.var_rrdrvmax_dn7 = 0.0;
        locals.var_rrdrvmax_dn8 = 0.0;
        locals.var_rrdrvmax_dn9 = 0.0;
        locals.var_rrdrvmax_dn10 = 0.0;
        locals.var_rrdrvmax_dn11 = 0.0;
        locals.var_rrdrvmax_dn14 = 0.0;
        locals.var_rrdrvmax_rv = 0.0;

        locals.var_rde = 0.0;
        locals.var_rde_dn0 = 0.0;
        locals.var_rde_dn2 = 0.0;
        locals.var_rde_dn4 = 0.0;
        locals.var_rde_dn5 = 0.0;
        locals.var_rde_dn6 = 0.0;
        locals.var_rde_dn7 = 0.0;
        locals.var_rde_dn8 = 0.0;
        locals.var_rde_dn9 = 0.0;
        locals.var_rde_dn10 = 0.0;
        locals.var_rde_dn11 = 0.0;
        locals.var_rde_dn14 = 0.0;
        locals.var_rde_rv = 0.0;

        locals.var_rdvde = 0.0;
        locals.var_rdvde_dn0 = 0.0;
        locals.var_rdvde_dn2 = 0.0;
        locals.var_rdvde_dn4 = 0.0;
        locals.var_rdvde_dn5 = 0.0;
        locals.var_rdvde_dn6 = 0.0;
        locals.var_rdvde_dn7 = 0.0;
        locals.var_rdvde_dn8 = 0.0;
        locals.var_rdvde_dn9 = 0.0;
        locals.var_rdvde_dn10 = 0.0;
        locals.var_rdvde_dn11 = 0.0;
        locals.var_rdvde_dn14 = 0.0;
        locals.var_rdvde_rv = 0.0;

        locals.var_rse = 0.0;
        locals.var_rse_dn0 = 0.0;
        locals.var_rse_dn2 = 0.0;
        locals.var_rse_dn4 = 0.0;
        locals.var_rse_dn5 = 0.0;
        locals.var_rse_dn6 = 0.0;
        locals.var_rse_dn7 = 0.0;
        locals.var_rse_dn8 = 0.0;
        locals.var_rse_dn9 = 0.0;
        locals.var_rse_dn10 = 0.0;
        locals.var_rse_dn11 = 0.0;
        locals.var_rse_dn14 = 0.0;
        locals.var_rse_rv = 0.0;

        locals.var_rsvde = 0.0;
        locals.var_rsvde_dn0 = 0.0;
        locals.var_rsvde_dn2 = 0.0;
        locals.var_rsvde_dn4 = 0.0;
        locals.var_rsvde_dn5 = 0.0;
        locals.var_rsvde_dn6 = 0.0;
        locals.var_rsvde_dn7 = 0.0;
        locals.var_rsvde_dn8 = 0.0;
        locals.var_rsvde_dn9 = 0.0;
        locals.var_rsvde_dn10 = 0.0;
        locals.var_rsvde_dn11 = 0.0;
        locals.var_rsvde_dn14 = 0.0;
        locals.var_rsvde_rv = 0.0;

        locals.var_rrdrvmaxs = 0.0;
        locals.var_rrdrvmaxs_dn0 = 0.0;
        locals.var_rrdrvmaxs_dn2 = 0.0;
        locals.var_rrdrvmaxs_dn4 = 0.0;
        locals.var_rrdrvmaxs_dn5 = 0.0;
        locals.var_rrdrvmaxs_dn6 = 0.0;
        locals.var_rrdrvmaxs_dn7 = 0.0;
        locals.var_rrdrvmaxs_dn8 = 0.0;
        locals.var_rrdrvmaxs_dn9 = 0.0;
        locals.var_rrdrvmaxs_dn10 = 0.0;
        locals.var_rrdrvmaxs_dn11 = 0.0;
        locals.var_rrdrvmaxs_dn14 = 0.0;
        locals.var_rrdrvmaxs_rv = 0.0;

        locals.var_tratio = 0.0;
        locals.var_tratio_dn0 = 0.0;
        locals.var_tratio_dn2 = 0.0;
        locals.var_tratio_dn4 = 0.0;
        locals.var_tratio_dn5 = 0.0;
        locals.var_tratio_dn6 = 0.0;
        locals.var_tratio_dn7 = 0.0;
        locals.var_tratio_dn8 = 0.0;
        locals.var_tratio_dn9 = 0.0;
        locals.var_tratio_dn10 = 0.0;
        locals.var_tratio_dn11 = 0.0;
        locals.var_tratio_dn14 = 0.0;
        locals.var_tratio_rv = 0.0;

        locals.var_vmaxeff = 0.0;
        locals.var_vmaxeff_dn0 = 0.0;
        locals.var_vmaxeff_dn2 = 0.0;
        locals.var_vmaxeff_dn4 = 0.0;
        locals.var_vmaxeff_dn5 = 0.0;
        locals.var_vmaxeff_dn6 = 0.0;
        locals.var_vmaxeff_dn7 = 0.0;
        locals.var_vmaxeff_dn8 = 0.0;
        locals.var_vmaxeff_dn9 = 0.0;
        locals.var_vmaxeff_dn10 = 0.0;
        locals.var_vmaxeff_dn11 = 0.0;
        locals.var_vmaxeff_dn14 = 0.0;
        locals.var_vmaxeff_rv = 0.0;

        locals.var_betatnom = 0.0;
        locals.var_betatnom_rv = 0.0;

        locals.var_cnst0over = 0.0;
        locals.var_cnst0over_dn0 = 0.0;
        locals.var_cnst0over_dn2 = 0.0;
        locals.var_cnst0over_dn4 = 0.0;
        locals.var_cnst0over_dn5 = 0.0;
        locals.var_cnst0over_dn6 = 0.0;
        locals.var_cnst0over_dn7 = 0.0;
        locals.var_cnst0over_dn8 = 0.0;
        locals.var_cnst0over_dn9 = 0.0;
        locals.var_cnst0over_dn10 = 0.0;
        locals.var_cnst0over_dn11 = 0.0;
        locals.var_cnst0over_dn14 = 0.0;
        locals.var_cnst0over_rv = 0.0;

        locals.var_cnst0overs = 0.0;
        locals.var_cnst0overs_dn0 = 0.0;
        locals.var_cnst0overs_dn2 = 0.0;
        locals.var_cnst0overs_dn4 = 0.0;
        locals.var_cnst0overs_dn5 = 0.0;
        locals.var_cnst0overs_dn6 = 0.0;
        locals.var_cnst0overs_dn7 = 0.0;
        locals.var_cnst0overs_dn8 = 0.0;
        locals.var_cnst0overs_dn9 = 0.0;
        locals.var_cnst0overs_dn10 = 0.0;
        locals.var_cnst0overs_dn11 = 0.0;
        locals.var_cnst0overs_dn14 = 0.0;
        locals.var_cnst0overs_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_9(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_costi0_p2 = 0.0;
        locals.var_costi0_p2_dn0 = 0.0;
        locals.var_costi0_p2_dn2 = 0.0;
        locals.var_costi0_p2_dn4 = 0.0;
        locals.var_costi0_p2_dn5 = 0.0;
        locals.var_costi0_p2_dn6 = 0.0;
        locals.var_costi0_p2_dn7 = 0.0;
        locals.var_costi0_p2_dn8 = 0.0;
        locals.var_costi0_p2_dn9 = 0.0;
        locals.var_costi0_p2_dn10 = 0.0;
        locals.var_costi0_p2_dn11 = 0.0;
        locals.var_costi0_p2_dn14 = 0.0;
        locals.var_costi0_p2_rv = 0.0;

        locals.var_mphn0 = 0.0;
        locals.var_mphn0_dn0 = 0.0;
        locals.var_mphn0_dn2 = 0.0;
        locals.var_mphn0_dn4 = 0.0;
        locals.var_mphn0_dn5 = 0.0;
        locals.var_mphn0_dn6 = 0.0;
        locals.var_mphn0_dn7 = 0.0;
        locals.var_mphn0_dn8 = 0.0;
        locals.var_mphn0_dn9 = 0.0;
        locals.var_mphn0_dn10 = 0.0;
        locals.var_mphn0_dn11 = 0.0;
        locals.var_mphn0_dn14 = 0.0;
        locals.var_mphn0_rv = 0.0;

        locals.var_powratio = 0.0;
        locals.var_powratio_dn0 = 0.0;
        locals.var_powratio_dn2 = 0.0;
        locals.var_powratio_dn4 = 0.0;
        locals.var_powratio_dn5 = 0.0;
        locals.var_powratio_dn6 = 0.0;
        locals.var_powratio_dn7 = 0.0;
        locals.var_powratio_dn8 = 0.0;
        locals.var_powratio_dn9 = 0.0;
        locals.var_powratio_dn10 = 0.0;
        locals.var_powratio_dn11 = 0.0;
        locals.var_powratio_dn14 = 0.0;
        locals.var_powratio_rv = 0.0;

        locals.var_ptovr = 0.0;
        locals.var_ptovr_dn0 = 0.0;
        locals.var_ptovr_dn2 = 0.0;
        locals.var_ptovr_dn4 = 0.0;
        locals.var_ptovr_dn5 = 0.0;
        locals.var_ptovr_dn6 = 0.0;
        locals.var_ptovr_dn7 = 0.0;
        locals.var_ptovr_dn8 = 0.0;
        locals.var_ptovr_dn9 = 0.0;
        locals.var_ptovr_dn10 = 0.0;
        locals.var_ptovr_dn11 = 0.0;
        locals.var_ptovr_dn14 = 0.0;
        locals.var_ptovr_rv = 0.0;

        locals.var_sqrt_eg = 0.0;
        locals.var_sqrt_eg_dn0 = 0.0;
        locals.var_sqrt_eg_dn2 = 0.0;
        locals.var_sqrt_eg_dn4 = 0.0;
        locals.var_sqrt_eg_dn5 = 0.0;
        locals.var_sqrt_eg_dn6 = 0.0;
        locals.var_sqrt_eg_dn7 = 0.0;
        locals.var_sqrt_eg_dn8 = 0.0;
        locals.var_sqrt_eg_dn9 = 0.0;
        locals.var_sqrt_eg_dn10 = 0.0;
        locals.var_sqrt_eg_dn11 = 0.0;
        locals.var_sqrt_eg_dn14 = 0.0;
        locals.var_sqrt_eg_rv = 0.0;

        locals.var_wdpl = 0.0;
        locals.var_wdpl_dn0 = 0.0;
        locals.var_wdpl_dn2 = 0.0;
        locals.var_wdpl_dn4 = 0.0;
        locals.var_wdpl_dn5 = 0.0;
        locals.var_wdpl_dn6 = 0.0;
        locals.var_wdpl_dn7 = 0.0;
        locals.var_wdpl_dn8 = 0.0;
        locals.var_wdpl_dn9 = 0.0;
        locals.var_wdpl_dn10 = 0.0;
        locals.var_wdpl_dn11 = 0.0;
        locals.var_wdpl_dn14 = 0.0;
        locals.var_wdpl_rv = 0.0;

        locals.var_wdplp = 0.0;
        locals.var_wdplp_dn0 = 0.0;
        locals.var_wdplp_dn2 = 0.0;
        locals.var_wdplp_dn4 = 0.0;
        locals.var_wdplp_dn5 = 0.0;
        locals.var_wdplp_dn6 = 0.0;
        locals.var_wdplp_dn7 = 0.0;
        locals.var_wdplp_dn8 = 0.0;
        locals.var_wdplp_dn9 = 0.0;
        locals.var_wdplp_dn10 = 0.0;
        locals.var_wdplp_dn11 = 0.0;
        locals.var_wdplp_dn14 = 0.0;
        locals.var_wdplp_rv = 0.0;

        locals.var_uc_rdrbb = p.p436;
        locals.var_uc_rdrbb_dn0 = 0.0;
        locals.var_uc_rdrbb_dn2 = 0.0;
        locals.var_uc_rdrbb_dn4 = 0.0;
        locals.var_uc_rdrbb_dn5 = 0.0;
        locals.var_uc_rdrbb_dn6 = 0.0;
        locals.var_uc_rdrbb_dn7 = 0.0;
        locals.var_uc_rdrbb_dn8 = 0.0;
        locals.var_uc_rdrbb_dn9 = 0.0;
        locals.var_uc_rdrbb_dn10 = 0.0;
        locals.var_uc_rdrbb_dn11 = 0.0;
        locals.var_uc_rdrbb_dn14 = 0.0;
        locals.var_uc_rdrbb_rv = 0.0;

        locals.var_uc_rdrbb_s = p.p437;
        locals.var_uc_rdrbb_s_dn0 = 0.0;
        locals.var_uc_rdrbb_s_dn2 = 0.0;
        locals.var_uc_rdrbb_s_dn4 = 0.0;
        locals.var_uc_rdrbb_s_dn5 = 0.0;
        locals.var_uc_rdrbb_s_dn6 = 0.0;
        locals.var_uc_rdrbb_s_dn7 = 0.0;
        locals.var_uc_rdrbb_s_dn8 = 0.0;
        locals.var_uc_rdrbb_s_dn9 = 0.0;
        locals.var_uc_rdrbb_s_dn10 = 0.0;
        locals.var_uc_rdrbb_s_dn11 = 0.0;
        locals.var_uc_rdrbb_s_dn14 = 0.0;
        locals.var_uc_rdrbb_s_rv = 0.0;

        locals.var_ids_acc = 0.0;
        locals.var_ids_acc_dn0 = 0.0;
        locals.var_ids_acc_dn2 = 0.0;
        locals.var_ids_acc_dn4 = 0.0;
        locals.var_ids_acc_dn5 = 0.0;
        locals.var_ids_acc_dn6 = 0.0;
        locals.var_ids_acc_dn7 = 0.0;
        locals.var_ids_acc_dn8 = 0.0;
        locals.var_ids_acc_dn9 = 0.0;
        locals.var_ids_acc_dn10 = 0.0;
        locals.var_ids_acc_dn11 = 0.0;
        locals.var_ids_acc_dn14 = 0.0;
        locals.var_ids_acc_rv = 0.0;

        locals.var_ids_res = 0.0;
        locals.var_ids_res_dn0 = 0.0;
        locals.var_ids_res_dn2 = 0.0;
        locals.var_ids_res_dn4 = 0.0;
        locals.var_ids_res_dn5 = 0.0;
        locals.var_ids_res_dn6 = 0.0;
        locals.var_ids_res_dn7 = 0.0;
        locals.var_ids_res_dn8 = 0.0;
        locals.var_ids_res_dn9 = 0.0;
        locals.var_ids_res_dn10 = 0.0;
        locals.var_ids_res_dn11 = 0.0;
        locals.var_ids_res_dn14 = 0.0;
        locals.var_ids_res_rv = 0.0;

        locals.var_ires_leak = 0.0;
        locals.var_ires_leak_dn0 = 0.0;
        locals.var_ires_leak_dn2 = 0.0;
        locals.var_ires_leak_dn4 = 0.0;
        locals.var_ires_leak_dn5 = 0.0;
        locals.var_ires_leak_dn6 = 0.0;
        locals.var_ires_leak_dn7 = 0.0;
        locals.var_ires_leak_dn8 = 0.0;
        locals.var_ires_leak_dn9 = 0.0;
        locals.var_ires_leak_dn10 = 0.0;
        locals.var_ires_leak_dn11 = 0.0;
        locals.var_ires_leak_dn14 = 0.0;
        locals.var_ires_leak_rv = 0.0;

        locals.var_pb2n = 0.0;
        locals.var_pb2n_dn0 = 0.0;
        locals.var_pb2n_dn2 = 0.0;
        locals.var_pb2n_dn4 = 0.0;
        locals.var_pb2n_dn5 = 0.0;
        locals.var_pb2n_dn6 = 0.0;
        locals.var_pb2n_dn7 = 0.0;
        locals.var_pb2n_dn8 = 0.0;
        locals.var_pb2n_dn9 = 0.0;
        locals.var_pb2n_dn10 = 0.0;
        locals.var_pb2n_dn11 = 0.0;
        locals.var_pb2n_dn14 = 0.0;
        locals.var_pb2n_rv = 0.0;

        locals.var_vbipn = 0.0;
        locals.var_vbipn_dn0 = 0.0;
        locals.var_vbipn_dn2 = 0.0;
        locals.var_vbipn_dn4 = 0.0;
        locals.var_vbipn_dn5 = 0.0;
        locals.var_vbipn_dn6 = 0.0;
        locals.var_vbipn_dn7 = 0.0;
        locals.var_vbipn_dn8 = 0.0;
        locals.var_vbipn_dn9 = 0.0;
        locals.var_vbipn_dn10 = 0.0;
        locals.var_vbipn_dn11 = 0.0;
        locals.var_vbipn_dn14 = 0.0;
        locals.var_vbipn_rv = 0.0;

        locals.var_hbdceff = p.p447;
        locals.var_hbdceff_dn0 = 0.0;
        locals.var_hbdceff_dn2 = 0.0;
        locals.var_hbdceff_dn4 = 0.0;
        locals.var_hbdceff_dn5 = 0.0;
        locals.var_hbdceff_dn6 = 0.0;
        locals.var_hbdceff_dn7 = 0.0;
        locals.var_hbdceff_dn8 = 0.0;
        locals.var_hbdceff_dn9 = 0.0;
        locals.var_hbdceff_dn10 = 0.0;
        locals.var_hbdceff_dn11 = 0.0;
        locals.var_hbdceff_dn14 = 0.0;
        locals.var_hbdceff_rv = 0.0;

        locals.var_uc_subtmp = p.p193;
        locals.var_uc_subtmp_rv = 0.0;

        locals.var_depmphn0 = 0.0;
        locals.var_depmphn0_dn0 = 0.0;
        locals.var_depmphn0_dn2 = 0.0;
        locals.var_depmphn0_dn4 = 0.0;
        locals.var_depmphn0_dn5 = 0.0;
        locals.var_depmphn0_dn6 = 0.0;
        locals.var_depmphn0_dn7 = 0.0;
        locals.var_depmphn0_dn8 = 0.0;
        locals.var_depmphn0_dn9 = 0.0;
        locals.var_depmphn0_dn10 = 0.0;
        locals.var_depmphn0_dn11 = 0.0;
        locals.var_depmphn0_dn14 = 0.0;
        locals.var_depmphn0_rv = 0.0;

        locals.var_qiu_noi = 0.0;
        locals.var_qiu_noi_dn0 = 0.0;
        locals.var_qiu_noi_dn2 = 0.0;
        locals.var_qiu_noi_dn4 = 0.0;
        locals.var_qiu_noi_dn5 = 0.0;
        locals.var_qiu_noi_dn6 = 0.0;
        locals.var_qiu_noi_dn7 = 0.0;
        locals.var_qiu_noi_dn8 = 0.0;
        locals.var_qiu_noi_dn9 = 0.0;
        locals.var_qiu_noi_dn10 = 0.0;
        locals.var_qiu_noi_dn11 = 0.0;
        locals.var_qiu_noi_dn14 = 0.0;
        locals.var_qiu_noi_rv = 0.0;

        locals.var_lp_s0_max = 40.0;
        locals.var_lp_s0_max_rv = 0.0;

        locals.var_js = 0.0;
        locals.var_js_dn0 = 0.0;
        locals.var_js_dn2 = 0.0;
        locals.var_js_dn4 = 0.0;
        locals.var_js_dn5 = 0.0;
        locals.var_js_dn6 = 0.0;
        locals.var_js_dn7 = 0.0;
        locals.var_js_dn8 = 0.0;
        locals.var_js_dn9 = 0.0;
        locals.var_js_dn10 = 0.0;
        locals.var_js_dn11 = 0.0;
        locals.var_js_dn14 = 0.0;
        locals.var_js_rv = 0.0;

        locals.var_jssw = 0.0;
        locals.var_jssw_dn0 = 0.0;
        locals.var_jssw_dn2 = 0.0;
        locals.var_jssw_dn4 = 0.0;
        locals.var_jssw_dn5 = 0.0;
        locals.var_jssw_dn6 = 0.0;
        locals.var_jssw_dn7 = 0.0;
        locals.var_jssw_dn8 = 0.0;
        locals.var_jssw_dn9 = 0.0;
        locals.var_jssw_dn10 = 0.0;
        locals.var_jssw_dn11 = 0.0;
        locals.var_jssw_dn14 = 0.0;
        locals.var_jssw_rv = 0.0;

        locals.var_js2 = 0.0;
        locals.var_js2_dn0 = 0.0;
        locals.var_js2_dn2 = 0.0;
        locals.var_js2_dn4 = 0.0;
        locals.var_js2_dn5 = 0.0;
        locals.var_js2_dn6 = 0.0;
        locals.var_js2_dn7 = 0.0;
        locals.var_js2_dn8 = 0.0;
        locals.var_js2_dn9 = 0.0;
        locals.var_js2_dn10 = 0.0;
        locals.var_js2_dn11 = 0.0;
        locals.var_js2_dn14 = 0.0;
        locals.var_js2_rv = 0.0;

        locals.var_jssw2 = 0.0;
        locals.var_jssw2_dn0 = 0.0;
        locals.var_jssw2_dn2 = 0.0;
        locals.var_jssw2_dn4 = 0.0;
        locals.var_jssw2_dn5 = 0.0;
        locals.var_jssw2_dn6 = 0.0;
        locals.var_jssw2_dn7 = 0.0;
        locals.var_jssw2_dn8 = 0.0;
        locals.var_jssw2_dn9 = 0.0;
        locals.var_jssw2_dn10 = 0.0;
        locals.var_jssw2_dn11 = 0.0;
        locals.var_jssw2_dn14 = 0.0;
        locals.var_jssw2_rv = 0.0;

        locals.var_qbs = 0.0;
        locals.var_qbs_dn0 = 0.0;
        locals.var_qbs_dn2 = 0.0;
        locals.var_qbs_dn4 = 0.0;
        locals.var_qbs_dn5 = 0.0;
        locals.var_qbs_dn6 = 0.0;
        locals.var_qbs_dn7 = 0.0;
        locals.var_qbs_dn8 = 0.0;
        locals.var_qbs_dn9 = 0.0;
        locals.var_qbs_dn10 = 0.0;
        locals.var_qbs_dn11 = 0.0;
        locals.var_qbs_dn14 = 0.0;
        locals.var_qbs_rv = 0.0;

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
        locals.var_qbd_dn11 = 0.0;
        locals.var_qbd_dn14 = 0.0;
        locals.var_qbd_dn16 = 0.0;
        locals.var_qbd_dn17 = 0.0;
        locals.var_qbd_dn18 = 0.0;
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
        locals.var_qbsi_dn11 = 0.0;
        locals.var_qbsi_dn14 = 0.0;
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
        locals.var_qbdi_dn11 = 0.0;
        locals.var_qbdi_dn14 = 0.0;
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
        locals.var_czbd_dn11 = 0.0;
        locals.var_czbd_dn14 = 0.0;
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
        locals.var_czbdsw_dn11 = 0.0;
        locals.var_czbdsw_dn14 = 0.0;
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
        locals.var_czbdswg_dn11 = 0.0;
        locals.var_czbdswg_dn14 = 0.0;
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
        locals.var_czbs_dn11 = 0.0;
        locals.var_czbs_dn14 = 0.0;
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
        locals.var_czbssw_dn11 = 0.0;
        locals.var_czbssw_dn14 = 0.0;
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
        locals.var_czbsswg_dn11 = 0.0;
        locals.var_czbsswg_dn14 = 0.0;
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
        locals.var_pzbd_dn11 = 0.0;
        locals.var_pzbd_dn14 = 0.0;
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
        locals.var_pzbdsw_dn11 = 0.0;
        locals.var_pzbdsw_dn14 = 0.0;
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
        locals.var_pzbdswg_dn11 = 0.0;
        locals.var_pzbdswg_dn14 = 0.0;
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
        locals.var_pzbs_dn11 = 0.0;
        locals.var_pzbs_dn14 = 0.0;
        locals.var_pzbs_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_10(
        locals: &mut StampLocals,
    ) {
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
        locals.var_pzbssw_dn11 = 0.0;
        locals.var_pzbssw_dn14 = 0.0;
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
        locals.var_pzbsswg_dn11 = 0.0;
        locals.var_pzbsswg_dn14 = 0.0;
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
        locals.var_sarg_dn11 = 0.0;
        locals.var_sarg_dn14 = 0.0;
        locals.var_sarg_rv = 0.0;

        locals.var_vsbs = 0.0;
        locals.var_vsbs_dn2 = 0.0;
        locals.var_vsbs_dn11 = 0.0;
        locals.var_vsbs_rv = 0.0;

        locals.var_vdbd = 0.0;
        locals.var_vdbd_dn0 = 0.0;
        locals.var_vdbd_dn10 = 0.0;
        locals.var_vdbd_rv = 0.0;

        locals.var_vbs_jct = 0.0;
        locals.var_vbs_jct_dn2 = 0.0;
        locals.var_vbs_jct_dn11 = 0.0;
        locals.var_vbs_jct_rv = 0.0;

        locals.var_vbd_jct = 0.0;
        locals.var_vbd_jct_dn0 = 0.0;
        locals.var_vbd_jct_dn10 = 0.0;
        locals.var_vbd_jct_rv = 0.0;

        locals.var_vbpsp = 0.0;
        locals.var_vbpsp_dn8 = 0.0;
        locals.var_vbpsp_dn9 = 0.0;
        locals.var_vbpsp_rv = 0.0;

        locals.var_vbpdp = 0.0;
        locals.var_vbpdp_dn6 = 0.0;
        locals.var_vbpdp_dn9 = 0.0;
        locals.var_vbpdp_rv = 0.0;

        locals.var_vbsi_jct = 0.0;
        locals.var_vbsi_jct_dn8 = 0.0;
        locals.var_vbsi_jct_dn9 = 0.0;
        locals.var_vbsi_jct_rv = 0.0;

        locals.var_vbdi_jct = 0.0;
        locals.var_vbdi_jct_dn6 = 0.0;
        locals.var_vbdi_jct_dn9 = 0.0;
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
        locals.var_exptempd_dn11 = 0.0;
        locals.var_exptempd_dn14 = 0.0;
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
        locals.var_exptemps_dn11 = 0.0;
        locals.var_exptemps_dn14 = 0.0;
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
        locals.var_isbd_dn11 = 0.0;
        locals.var_isbd_dn14 = 0.0;
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
        locals.var_isbs_dn11 = 0.0;
        locals.var_isbs_dn14 = 0.0;
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
        locals.var_jd_expcd_dn11 = 0.0;
        locals.var_jd_expcd_dn14 = 0.0;
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
        locals.var_jd_expcs_dn11 = 0.0;
        locals.var_jd_expcs_dn14 = 0.0;
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
        locals.var_vbdt_dn11 = 0.0;
        locals.var_vbdt_dn14 = 0.0;
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
        locals.var_vbst_dn11 = 0.0;
        locals.var_vbst_dn14 = 0.0;
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
        locals.var_jd_nvtm_invd_dn11 = 0.0;
        locals.var_jd_nvtm_invd_dn14 = 0.0;
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
        locals.var_jd_nvtm_invs_dn11 = 0.0;
        locals.var_jd_nvtm_invs_dn14 = 0.0;
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
        locals.var_qbd_qs_dn11 = 0.0;
        locals.var_qbd_qs_dn14 = 0.0;
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
        locals.var_isbd_btm_dn11 = 0.0;
        locals.var_isbd_btm_dn14 = 0.0;
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
        locals.var_isbd2_btm_dn11 = 0.0;
        locals.var_isbd2_btm_dn14 = 0.0;
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
        locals.var_isbd_sws_dn11 = 0.0;
        locals.var_isbd_sws_dn14 = 0.0;
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
        locals.var_isbd2_sws_dn11 = 0.0;
        locals.var_isbd2_sws_dn14 = 0.0;
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
        locals.var_isbd_swg_dn11 = 0.0;
        locals.var_isbd_swg_dn14 = 0.0;
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
        locals.var_isbd2_swg_dn11 = 0.0;
        locals.var_isbd2_swg_dn14 = 0.0;
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
        locals.var_isbs_btm_dn11 = 0.0;
        locals.var_isbs_btm_dn14 = 0.0;
        locals.var_isbs_btm_rv = 0.0;

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
        locals.var_isbs2_btm_dn11 = 0.0;
        locals.var_isbs2_btm_dn14 = 0.0;
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
        locals.var_isbs_sws_dn11 = 0.0;
        locals.var_isbs_sws_dn14 = 0.0;
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
        locals.var_isbs2_sws_dn11 = 0.0;
        locals.var_isbs2_sws_dn14 = 0.0;
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
        locals.var_isbs_swg_dn11 = 0.0;
        locals.var_isbs_swg_dn14 = 0.0;
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
        locals.var_isbs2_swg_dn11 = 0.0;
        locals.var_isbs2_swg_dn14 = 0.0;
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
        locals.var_qovd_add_dn11 = 0.0;
        locals.var_qovd_add_dn14 = 0.0;
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
        locals.var_qovs_add_dn11 = 0.0;
        locals.var_qovs_add_dn14 = 0.0;
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
        locals.var_qbdld_add_dn11 = 0.0;
        locals.var_qbdld_add_dn14 = 0.0;
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
        locals.var_qbsld_add_dn11 = 0.0;
        locals.var_qbsld_add_dn14 = 0.0;
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
        locals.var_wjuncld_dn11 = 0.0;
        locals.var_wjuncld_dn14 = 0.0;
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
        locals.var_idspt0_dn11 = 0.0;
        locals.var_idspt0_dn14 = 0.0;
        locals.var_idspt0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_11(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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
        locals.var_idspt1_dn11 = 0.0;
        locals.var_idspt1_dn14 = 0.0;
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
        locals.var_iwnqs0_a_dn11 = 0.0;
        locals.var_iwnqs0_a_dn14 = 0.0;
        locals.var_iwnqs0_a_dn18 = 0.0;
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
        locals.var_inqs0_a_dn11 = 0.0;
        locals.var_inqs0_a_dn14 = 0.0;
        locals.var_inqs0_a_dn16 = 0.0;
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
        locals.var_inqs0_k_dn11 = 0.0;
        locals.var_inqs0_k_dn14 = 0.0;
        locals.var_inqs0_k_dn17 = 0.0;
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
        locals.var_isubibpc_dn11 = 0.0;
        locals.var_isubibpc_dn14 = 0.0;
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
        locals.var_lover_func_dn11 = 0.0;
        locals.var_lover_func_dn14 = 0.0;
        locals.var_lover_func_rv = 0.0;

        locals.var_q_nqs_a = 0.0;
        locals.var_q_nqs_a_dn16 = 0.0;
        locals.var_q_nqs_a_rv = 0.0;

        locals.var_q_nqs_k = 0.0;
        locals.var_q_nqs_k_dn17 = 0.0;
        locals.var_q_nqs_k_rv = 0.0;

        locals.var_w_nqs_a = 0.0;
        locals.var_w_nqs_a_dn18 = 0.0;
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
        locals.var_w_res_dn11 = 0.0;
        locals.var_w_res_dn14 = 0.0;
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
        locals.var_wdep_func_dn11 = 0.0;
        locals.var_wdep_func_dn14 = 0.0;
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
        locals.var_wk_ii_dn11 = 0.0;
        locals.var_wk_ii_dn14 = 0.0;
        locals.var_wk_ii_rv = 0.0;

        let (assign5360_e1954,) = {
    if (p.p40 != 0.0) {
        (0.0,)
    } else {
        (p.p17,)
    }
};
        locals.var_uc_corsrd = assign5360_e1954;
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

        let assign5520_e1997: f64 = if locals.var_uc_scp22 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard12 = assign5520_e1997;
        locals.var_guard12_rv = 0.0;

        let (assign5530_e2001,) = {
    if (locals.var_guard12 != 0.0) {
        (0.0,)
    } else {
        (locals.var_uc_scp22,)
    }
};
        locals.var_uc_scp22 = assign5530_e2001;
        locals.var_uc_scp22_rv = 0.0;

        let assign5540_e2004: f64 = if locals.var_uc_scp22 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard13 = assign5540_e2004;
        locals.var_guard13_rv = 0.0;

        let (assign5550_e2008,) = {
    if (locals.var_guard13 != 0.0) {
        (0.0,)
    } else {
        (locals.var_uc_scp22,)
    }
};
        locals.var_uc_scp22 = assign5550_e2008;
        locals.var_uc_scp22_rv = 0.0;

        let assign5570_e2016: f64 = if locals.var_uc_xldld < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard15 = assign5570_e2016;
        locals.var_guard15_rv = 0.0;

        let (assign5580_e2020,) = {
    if (locals.var_guard15 != 0.0) {
        (0.0,)
    } else {
        (locals.var_uc_xldld,)
    }
};
        locals.var_uc_xldld = assign5580_e2020;
        locals.var_uc_xldld_rv = 0.0;

        let assign5610_e2033: f64 = if locals.var_uc_rdrcx < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard18 = assign5610_e2033;
        locals.var_guard18_rv = 0.0;

        let (assign5620_e2037,) = {
    if (locals.var_guard18 != 0.0) {
        (0.0,)
    } else {
        (locals.var_uc_rdrcx,)
    }
};
        locals.var_uc_rdrcx = assign5620_e2037;
        locals.var_uc_rdrcx_rv = 0.0;

        let assign5630_e2040: f64 = if locals.var_uc_rdrcx > 1.0 { 1.0 } else { 0.0 };
        locals.var_guard19 = assign5630_e2040;
        locals.var_guard19_rv = 0.0;

        let (assign5640_e2044,) = {
    if (locals.var_guard19 != 0.0) {
        (1.0,)
    } else {
        (locals.var_uc_rdrcx,)
    }
};
        locals.var_uc_rdrcx = assign5640_e2044;
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
        locals.var_uc_ndepm_dn11 = 0.0;
        locals.var_uc_ndepm_dn14 = 0.0;
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
        locals.var_uc_depthn_dn11 = 0.0;
        locals.var_uc_depthn_dn14 = 0.0;
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
        locals.var_uc_depmueback0_dn11 = 0.0;
        locals.var_uc_depmueback0_dn14 = 0.0;
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
        locals.var_uc_depmueback1_dn11 = 0.0;
        locals.var_uc_depmueback1_dn14 = 0.0;
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
        locals.var_uc_depmue0_dn11 = 0.0;
        locals.var_uc_depmue0_dn14 = 0.0;
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
        locals.var_uc_depmue1_dn11 = 0.0;
        locals.var_uc_depmue1_dn14 = 0.0;
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
        locals.var_uc_depmue2_dn11 = 0.0;
        locals.var_uc_depmue2_dn14 = 0.0;
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
        locals.var_uc_depleak_dn11 = 0.0;
        locals.var_uc_depleak_dn14 = 0.0;
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
        locals.var_uc_depvmax_dn11 = 0.0;
        locals.var_uc_depvmax_dn14 = 0.0;
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
        locals.var_uc_depwlp_dn11 = 0.0;
        locals.var_uc_depwlp_dn14 = 0.0;
        locals.var_uc_depwlp_rv = 0.0;

        locals.var_uc_depmueph1 = p.p377;
        locals.var_uc_depmueph1_rv = 0.0;

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
        locals.var_uc_depvdsef1_dn11 = 0.0;
        locals.var_uc_depvdsef1_dn14 = 0.0;
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
        locals.var_uc_depvdsef2_dn11 = 0.0;
        locals.var_uc_depvdsef2_dn14 = 0.0;
        locals.var_uc_depvdsef2_rv = 0.0;

        let assign6730_e2717: f64 = if ((locals.var_uc_codep < 3.0) && (locals.var_uc_codep > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard114 = assign6730_e2717;
        locals.var_guard114_rv = 0.0;

        let assign6760_e2730: f64 = if locals.var_uc_ndepm < 5000000000000000.0 { 1.0 } else { 0.0 };
        locals.var_guard117 = assign6760_e2730;
        locals.var_guard117_rv = 0.0;

        let (assign6770_e2736, assign6770_e2736_d_n0, assign6770_e2736_d_n2, assign6770_e2736_d_n4, assign6770_e2736_d_n5, assign6770_e2736_d_n6, assign6770_e2736_d_n7, assign6770_e2736_d_n8, assign6770_e2736_d_n9, assign6770_e2736_d_n10, assign6770_e2736_d_n11, assign6770_e2736_d_n14,) = {
    if ((locals.var_guard114 != 0.0) && (locals.var_guard117 != 0.0)) {
        (5000000000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn11, locals.var_uc_ndepm_dn14,)
    }
};
        locals.var_uc_ndepm = assign6770_e2736;
        locals.var_uc_ndepm_dn0 = assign6770_e2736_d_n0;
        locals.var_uc_ndepm_dn2 = assign6770_e2736_d_n2;
        locals.var_uc_ndepm_dn4 = assign6770_e2736_d_n4;
        locals.var_uc_ndepm_dn5 = assign6770_e2736_d_n5;
        locals.var_uc_ndepm_dn6 = assign6770_e2736_d_n6;
        locals.var_uc_ndepm_dn7 = assign6770_e2736_d_n7;
        locals.var_uc_ndepm_dn8 = assign6770_e2736_d_n8;
        locals.var_uc_ndepm_dn9 = assign6770_e2736_d_n9;
        locals.var_uc_ndepm_dn10 = assign6770_e2736_d_n10;
        locals.var_uc_ndepm_dn11 = assign6770_e2736_d_n11;
        locals.var_uc_ndepm_dn14 = assign6770_e2736_d_n14;
        locals.var_uc_ndepm_rv = 0.0;

        let assign6780_e2739: f64 = if locals.var_uc_ndepm > 1e18 { 1.0 } else { 0.0 };
        locals.var_guard118 = assign6780_e2739;
        locals.var_guard118_rv = 0.0;

        let (assign6790_e2745, assign6790_e2745_d_n0, assign6790_e2745_d_n2, assign6790_e2745_d_n4, assign6790_e2745_d_n5, assign6790_e2745_d_n6, assign6790_e2745_d_n7, assign6790_e2745_d_n8, assign6790_e2745_d_n9, assign6790_e2745_d_n10, assign6790_e2745_d_n11, assign6790_e2745_d_n14,) = {
    if ((locals.var_guard114 != 0.0) && (locals.var_guard118 != 0.0)) {
        (1e18, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn11, locals.var_uc_ndepm_dn14,)
    }
};
        locals.var_uc_ndepm = assign6790_e2745;
        locals.var_uc_ndepm_dn0 = assign6790_e2745_d_n0;
        locals.var_uc_ndepm_dn2 = assign6790_e2745_d_n2;
        locals.var_uc_ndepm_dn4 = assign6790_e2745_d_n4;
        locals.var_uc_ndepm_dn5 = assign6790_e2745_d_n5;
        locals.var_uc_ndepm_dn6 = assign6790_e2745_d_n6;
        locals.var_uc_ndepm_dn7 = assign6790_e2745_d_n7;
        locals.var_uc_ndepm_dn8 = assign6790_e2745_d_n8;
        locals.var_uc_ndepm_dn9 = assign6790_e2745_d_n9;
        locals.var_uc_ndepm_dn10 = assign6790_e2745_d_n10;
        locals.var_uc_ndepm_dn11 = assign6790_e2745_d_n11;
        locals.var_uc_ndepm_dn14 = assign6790_e2745_d_n14;
        locals.var_uc_ndepm_rv = 0.0;

        let assign6820_e2758: f64 = if locals.var_uc_depthn < 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard121 = assign6820_e2758;
        locals.var_guard121_rv = 0.0;

        let (assign6830_e2764, assign6830_e2764_d_n0, assign6830_e2764_d_n2, assign6830_e2764_d_n4, assign6830_e2764_d_n5, assign6830_e2764_d_n6, assign6830_e2764_d_n7, assign6830_e2764_d_n8, assign6830_e2764_d_n9, assign6830_e2764_d_n10, assign6830_e2764_d_n11, assign6830_e2764_d_n14,) = {
    if ((locals.var_guard114 != 0.0) && (locals.var_guard121 != 0.0)) {
        (1e-8, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depthn, locals.var_uc_depthn_dn0, locals.var_uc_depthn_dn2, locals.var_uc_depthn_dn4, locals.var_uc_depthn_dn5, locals.var_uc_depthn_dn6, locals.var_uc_depthn_dn7, locals.var_uc_depthn_dn8, locals.var_uc_depthn_dn9, locals.var_uc_depthn_dn10, locals.var_uc_depthn_dn11, locals.var_uc_depthn_dn14,)
    }
};
        locals.var_uc_depthn = assign6830_e2764;
        locals.var_uc_depthn_dn0 = assign6830_e2764_d_n0;
        locals.var_uc_depthn_dn2 = assign6830_e2764_d_n2;
        locals.var_uc_depthn_dn4 = assign6830_e2764_d_n4;
        locals.var_uc_depthn_dn5 = assign6830_e2764_d_n5;
        locals.var_uc_depthn_dn6 = assign6830_e2764_d_n6;
        locals.var_uc_depthn_dn7 = assign6830_e2764_d_n7;
        locals.var_uc_depthn_dn8 = assign6830_e2764_d_n8;
        locals.var_uc_depthn_dn9 = assign6830_e2764_d_n9;
        locals.var_uc_depthn_dn10 = assign6830_e2764_d_n10;
        locals.var_uc_depthn_dn11 = assign6830_e2764_d_n11;
        locals.var_uc_depthn_dn14 = assign6830_e2764_d_n14;
        locals.var_uc_depthn_rv = 0.0;

        let assign6840_e2767: f64 = if locals.var_uc_depthn > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard122 = assign6840_e2767;
        locals.var_guard122_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6850_e2773, assign6850_e2773_d_n0, assign6850_e2773_d_n2, assign6850_e2773_d_n4, assign6850_e2773_d_n5, assign6850_e2773_d_n6, assign6850_e2773_d_n7, assign6850_e2773_d_n8, assign6850_e2773_d_n9, assign6850_e2773_d_n10, assign6850_e2773_d_n11, assign6850_e2773_d_n14,) = {
    if ((locals.var_guard114 != 0.0) && (locals.var_guard122 != 0.0)) {
        (1e-6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depthn, locals.var_uc_depthn_dn0, locals.var_uc_depthn_dn2, locals.var_uc_depthn_dn4, locals.var_uc_depthn_dn5, locals.var_uc_depthn_dn6, locals.var_uc_depthn_dn7, locals.var_uc_depthn_dn8, locals.var_uc_depthn_dn9, locals.var_uc_depthn_dn10, locals.var_uc_depthn_dn11, locals.var_uc_depthn_dn14,)
    }
};
        locals.var_uc_depthn = assign6850_e2773;
        locals.var_uc_depthn_dn0 = assign6850_e2773_d_n0;
        locals.var_uc_depthn_dn2 = assign6850_e2773_d_n2;
        locals.var_uc_depthn_dn4 = assign6850_e2773_d_n4;
        locals.var_uc_depthn_dn5 = assign6850_e2773_d_n5;
        locals.var_uc_depthn_dn6 = assign6850_e2773_d_n6;
        locals.var_uc_depthn_dn7 = assign6850_e2773_d_n7;
        locals.var_uc_depthn_dn8 = assign6850_e2773_d_n8;
        locals.var_uc_depthn_dn9 = assign6850_e2773_d_n9;
        locals.var_uc_depthn_dn10 = assign6850_e2773_d_n10;
        locals.var_uc_depthn_dn11 = assign6850_e2773_d_n11;
        locals.var_uc_depthn_dn14 = assign6850_e2773_d_n14;
        locals.var_uc_depthn_rv = 0.0;

        let assign6880_e2786: f64 = if locals.var_uc_depmue0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard125 = assign6880_e2786;
        locals.var_guard125_rv = 0.0;

        let (assign6890_e2792, assign6890_e2792_d_n0, assign6890_e2792_d_n2, assign6890_e2792_d_n4, assign6890_e2792_d_n5, assign6890_e2792_d_n6, assign6890_e2792_d_n7, assign6890_e2792_d_n8, assign6890_e2792_d_n9, assign6890_e2792_d_n10, assign6890_e2792_d_n11, assign6890_e2792_d_n14,) = {
    if ((locals.var_guard114 != 0.0) && (locals.var_guard125 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    }
};
        locals.var_uc_depmue0 = assign6890_e2792;
        locals.var_uc_depmue0_dn0 = assign6890_e2792_d_n0;
        locals.var_uc_depmue0_dn2 = assign6890_e2792_d_n2;
        locals.var_uc_depmue0_dn4 = assign6890_e2792_d_n4;
        locals.var_uc_depmue0_dn5 = assign6890_e2792_d_n5;
        locals.var_uc_depmue0_dn6 = assign6890_e2792_d_n6;
        locals.var_uc_depmue0_dn7 = assign6890_e2792_d_n7;
        locals.var_uc_depmue0_dn8 = assign6890_e2792_d_n8;
        locals.var_uc_depmue0_dn9 = assign6890_e2792_d_n9;
        locals.var_uc_depmue0_dn10 = assign6890_e2792_d_n10;
        locals.var_uc_depmue0_dn11 = assign6890_e2792_d_n11;
        locals.var_uc_depmue0_dn14 = assign6890_e2792_d_n14;
        locals.var_uc_depmue0_rv = 0.0;

        let assign6900_e2795: f64 = if locals.var_uc_depmue0 > 100000.0 { 1.0 } else { 0.0 };
        locals.var_guard126 = assign6900_e2795;
        locals.var_guard126_rv = 0.0;

        let (assign6910_e2801, assign6910_e2801_d_n0, assign6910_e2801_d_n2, assign6910_e2801_d_n4, assign6910_e2801_d_n5, assign6910_e2801_d_n6, assign6910_e2801_d_n7, assign6910_e2801_d_n8, assign6910_e2801_d_n9, assign6910_e2801_d_n10, assign6910_e2801_d_n11, assign6910_e2801_d_n14,) = {
    if ((locals.var_guard114 != 0.0) && (locals.var_guard126 != 0.0)) {
        (100000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    }
};
        locals.var_uc_depmue0 = assign6910_e2801;
        locals.var_uc_depmue0_dn0 = assign6910_e2801_d_n0;
        locals.var_uc_depmue0_dn2 = assign6910_e2801_d_n2;
        locals.var_uc_depmue0_dn4 = assign6910_e2801_d_n4;
        locals.var_uc_depmue0_dn5 = assign6910_e2801_d_n5;
        locals.var_uc_depmue0_dn6 = assign6910_e2801_d_n6;
        locals.var_uc_depmue0_dn7 = assign6910_e2801_d_n7;
        locals.var_uc_depmue0_dn8 = assign6910_e2801_d_n8;
        locals.var_uc_depmue0_dn9 = assign6910_e2801_d_n9;
        locals.var_uc_depmue0_dn10 = assign6910_e2801_d_n10;
        locals.var_uc_depmue0_dn11 = assign6910_e2801_d_n11;
        locals.var_uc_depmue0_dn14 = assign6910_e2801_d_n14;
        locals.var_uc_depmue0_rv = 0.0;

        let assign6940_e2814: f64 = if locals.var_uc_depmueback0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard129 = assign6940_e2814;
        locals.var_guard129_rv = 0.0;

        let (assign6950_e2820, assign6950_e2820_d_n0, assign6950_e2820_d_n2, assign6950_e2820_d_n4, assign6950_e2820_d_n5, assign6950_e2820_d_n6, assign6950_e2820_d_n7, assign6950_e2820_d_n8, assign6950_e2820_d_n9, assign6950_e2820_d_n10, assign6950_e2820_d_n11, assign6950_e2820_d_n14,) = {
    if ((locals.var_guard114 != 0.0) && (locals.var_guard129 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmueback0, locals.var_uc_depmueback0_dn0, locals.var_uc_depmueback0_dn2, locals.var_uc_depmueback0_dn4, locals.var_uc_depmueback0_dn5, locals.var_uc_depmueback0_dn6, locals.var_uc_depmueback0_dn7, locals.var_uc_depmueback0_dn8, locals.var_uc_depmueback0_dn9, locals.var_uc_depmueback0_dn10, locals.var_uc_depmueback0_dn11, locals.var_uc_depmueback0_dn14,)
    }
};
        locals.var_uc_depmueback0 = assign6950_e2820;
        locals.var_uc_depmueback0_dn0 = assign6950_e2820_d_n0;
        locals.var_uc_depmueback0_dn2 = assign6950_e2820_d_n2;
        locals.var_uc_depmueback0_dn4 = assign6950_e2820_d_n4;
        locals.var_uc_depmueback0_dn5 = assign6950_e2820_d_n5;
        locals.var_uc_depmueback0_dn6 = assign6950_e2820_d_n6;
        locals.var_uc_depmueback0_dn7 = assign6950_e2820_d_n7;
        locals.var_uc_depmueback0_dn8 = assign6950_e2820_d_n8;
        locals.var_uc_depmueback0_dn9 = assign6950_e2820_d_n9;
        locals.var_uc_depmueback0_dn10 = assign6950_e2820_d_n10;
        locals.var_uc_depmueback0_dn11 = assign6950_e2820_d_n11;
        locals.var_uc_depmueback0_dn14 = assign6950_e2820_d_n14;
        locals.var_uc_depmueback0_rv = 0.0;

        let assign6960_e2823: f64 = if locals.var_uc_depmueback0 > 100000.0 { 1.0 } else { 0.0 };
        locals.var_guard130 = assign6960_e2823;
        locals.var_guard130_rv = 0.0;

        let (assign6970_e2829, assign6970_e2829_d_n0, assign6970_e2829_d_n2, assign6970_e2829_d_n4, assign6970_e2829_d_n5, assign6970_e2829_d_n6, assign6970_e2829_d_n7, assign6970_e2829_d_n8, assign6970_e2829_d_n9, assign6970_e2829_d_n10, assign6970_e2829_d_n11, assign6970_e2829_d_n14,) = {
    if ((locals.var_guard114 != 0.0) && (locals.var_guard130 != 0.0)) {
        (100000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmueback0, locals.var_uc_depmueback0_dn0, locals.var_uc_depmueback0_dn2, locals.var_uc_depmueback0_dn4, locals.var_uc_depmueback0_dn5, locals.var_uc_depmueback0_dn6, locals.var_uc_depmueback0_dn7, locals.var_uc_depmueback0_dn8, locals.var_uc_depmueback0_dn9, locals.var_uc_depmueback0_dn10, locals.var_uc_depmueback0_dn11, locals.var_uc_depmueback0_dn14,)
    }
};
        locals.var_uc_depmueback0 = assign6970_e2829;
        locals.var_uc_depmueback0_dn0 = assign6970_e2829_d_n0;
        locals.var_uc_depmueback0_dn2 = assign6970_e2829_d_n2;
        locals.var_uc_depmueback0_dn4 = assign6970_e2829_d_n4;
        locals.var_uc_depmueback0_dn5 = assign6970_e2829_d_n5;
        locals.var_uc_depmueback0_dn6 = assign6970_e2829_d_n6;
        locals.var_uc_depmueback0_dn7 = assign6970_e2829_d_n7;
        locals.var_uc_depmueback0_dn8 = assign6970_e2829_d_n8;
        locals.var_uc_depmueback0_dn9 = assign6970_e2829_d_n9;
        locals.var_uc_depmueback0_dn10 = assign6970_e2829_d_n10;
        locals.var_uc_depmueback0_dn11 = assign6970_e2829_d_n11;
        locals.var_uc_depmueback0_dn14 = assign6970_e2829_d_n14;
        locals.var_uc_depmueback0_rv = 0.0;

        let assign7000_e2842: f64 = if locals.var_uc_depmueph1 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard133 = assign7000_e2842;
        locals.var_guard133_rv = 0.0;

        let (assign7010_e2848,) = {
    if ((locals.var_guard114 != 0.0) && (locals.var_guard133 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_uc_depmueph1,)
    }
};
        locals.var_uc_depmueph1 = assign7010_e2848;
        locals.var_uc_depmueph1_rv = 0.0;

        let assign7020_e2851: f64 = if locals.var_uc_depmueph1 > 100000.0 { 1.0 } else { 0.0 };
        locals.var_guard134 = assign7020_e2851;
        locals.var_guard134_rv = 0.0;

        let (assign7030_e2857,) = {
    if ((locals.var_guard114 != 0.0) && (locals.var_guard134 != 0.0)) {
        (100000.0,)
    } else {
        (locals.var_uc_depmueph1,)
    }
};
        locals.var_uc_depmueph1 = assign7030_e2857;
        locals.var_uc_depmueph1_rv = 0.0;

        let assign7060_e2870: f64 = if locals.var_uc_depvdsef2 < 0.1 { 1.0 } else { 0.0 };
        locals.var_guard137 = assign7060_e2870;
        locals.var_guard137_rv = 0.0;

        let (assign7070_e2876, assign7070_e2876_d_n0, assign7070_e2876_d_n2, assign7070_e2876_d_n4, assign7070_e2876_d_n5, assign7070_e2876_d_n6, assign7070_e2876_d_n7, assign7070_e2876_d_n8, assign7070_e2876_d_n9, assign7070_e2876_d_n10, assign7070_e2876_d_n11, assign7070_e2876_d_n14,) = {
    if ((locals.var_guard114 != 0.0) && (locals.var_guard137 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvdsef2, locals.var_uc_depvdsef2_dn0, locals.var_uc_depvdsef2_dn2, locals.var_uc_depvdsef2_dn4, locals.var_uc_depvdsef2_dn5, locals.var_uc_depvdsef2_dn6, locals.var_uc_depvdsef2_dn7, locals.var_uc_depvdsef2_dn8, locals.var_uc_depvdsef2_dn9, locals.var_uc_depvdsef2_dn10, locals.var_uc_depvdsef2_dn11, locals.var_uc_depvdsef2_dn14,)
    }
};
        locals.var_uc_depvdsef2 = assign7070_e2876;
        locals.var_uc_depvdsef2_dn0 = assign7070_e2876_d_n0;
        locals.var_uc_depvdsef2_dn2 = assign7070_e2876_d_n2;
        locals.var_uc_depvdsef2_dn4 = assign7070_e2876_d_n4;
        locals.var_uc_depvdsef2_dn5 = assign7070_e2876_d_n5;
        locals.var_uc_depvdsef2_dn6 = assign7070_e2876_d_n6;
        locals.var_uc_depvdsef2_dn7 = assign7070_e2876_d_n7;
        locals.var_uc_depvdsef2_dn8 = assign7070_e2876_d_n8;
        locals.var_uc_depvdsef2_dn9 = assign7070_e2876_d_n9;
        locals.var_uc_depvdsef2_dn10 = assign7070_e2876_d_n10;
        locals.var_uc_depvdsef2_dn11 = assign7070_e2876_d_n11;
        locals.var_uc_depvdsef2_dn14 = assign7070_e2876_d_n14;
        locals.var_uc_depvdsef2_rv = 0.0;

        let assign7080_e2879: f64 = if locals.var_uc_depvdsef2 > 4.0 { 1.0 } else { 0.0 };
        locals.var_guard138 = assign7080_e2879;
        locals.var_guard138_rv = 0.0;

        let (assign7090_e2885, assign7090_e2885_d_n0, assign7090_e2885_d_n2, assign7090_e2885_d_n4, assign7090_e2885_d_n5, assign7090_e2885_d_n6, assign7090_e2885_d_n7, assign7090_e2885_d_n8, assign7090_e2885_d_n9, assign7090_e2885_d_n10, assign7090_e2885_d_n11, assign7090_e2885_d_n14,) = {
    if ((locals.var_guard114 != 0.0) && (locals.var_guard138 != 0.0)) {
        (4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvdsef2, locals.var_uc_depvdsef2_dn0, locals.var_uc_depvdsef2_dn2, locals.var_uc_depvdsef2_dn4, locals.var_uc_depvdsef2_dn5, locals.var_uc_depvdsef2_dn6, locals.var_uc_depvdsef2_dn7, locals.var_uc_depvdsef2_dn8, locals.var_uc_depvdsef2_dn9, locals.var_uc_depvdsef2_dn10, locals.var_uc_depvdsef2_dn11, locals.var_uc_depvdsef2_dn14,)
    }
};
        locals.var_uc_depvdsef2 = assign7090_e2885;
        locals.var_uc_depvdsef2_dn0 = assign7090_e2885_d_n0;
        locals.var_uc_depvdsef2_dn2 = assign7090_e2885_d_n2;
        locals.var_uc_depvdsef2_dn4 = assign7090_e2885_d_n4;
        locals.var_uc_depvdsef2_dn5 = assign7090_e2885_d_n5;
        locals.var_uc_depvdsef2_dn6 = assign7090_e2885_d_n6;
        locals.var_uc_depvdsef2_dn7 = assign7090_e2885_d_n7;
        locals.var_uc_depvdsef2_dn8 = assign7090_e2885_d_n8;
        locals.var_uc_depvdsef2_dn9 = assign7090_e2885_d_n9;
        locals.var_uc_depvdsef2_dn10 = assign7090_e2885_d_n10;
        locals.var_uc_depvdsef2_dn11 = assign7090_e2885_d_n11;
        locals.var_uc_depvdsef2_dn14 = assign7090_e2885_d_n14;
        locals.var_uc_depvdsef2_rv = 0.0;

        let assign7120_e2898: f64 = if locals.var_uc_depleak < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard141 = assign7120_e2898;
        locals.var_guard141_rv = 0.0;

        let (assign7130_e2904, assign7130_e2904_d_n0, assign7130_e2904_d_n2, assign7130_e2904_d_n4, assign7130_e2904_d_n5, assign7130_e2904_d_n6, assign7130_e2904_d_n7, assign7130_e2904_d_n8, assign7130_e2904_d_n9, assign7130_e2904_d_n10, assign7130_e2904_d_n11, assign7130_e2904_d_n14,) = {
    if ((locals.var_guard114 != 0.0) && (locals.var_guard141 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn11, locals.var_uc_depleak_dn14,)
    }
};
        locals.var_uc_depleak = assign7130_e2904;
        locals.var_uc_depleak_dn0 = assign7130_e2904_d_n0;
        locals.var_uc_depleak_dn2 = assign7130_e2904_d_n2;
        locals.var_uc_depleak_dn4 = assign7130_e2904_d_n4;
        locals.var_uc_depleak_dn5 = assign7130_e2904_d_n5;
        locals.var_uc_depleak_dn6 = assign7130_e2904_d_n6;
        locals.var_uc_depleak_dn7 = assign7130_e2904_d_n7;
        locals.var_uc_depleak_dn8 = assign7130_e2904_d_n8;
        locals.var_uc_depleak_dn9 = assign7130_e2904_d_n9;
        locals.var_uc_depleak_dn10 = assign7130_e2904_d_n10;
        locals.var_uc_depleak_dn11 = assign7130_e2904_d_n11;
        locals.var_uc_depleak_dn14 = assign7130_e2904_d_n14;
        locals.var_uc_depleak_rv = 0.0;

        let assign7140_e2907: f64 = if locals.var_uc_depleak > 5.0 { 1.0 } else { 0.0 };
        locals.var_guard142 = assign7140_e2907;
        locals.var_guard142_rv = 0.0;

        let (assign7150_e2913, assign7150_e2913_d_n0, assign7150_e2913_d_n2, assign7150_e2913_d_n4, assign7150_e2913_d_n5, assign7150_e2913_d_n6, assign7150_e2913_d_n7, assign7150_e2913_d_n8, assign7150_e2913_d_n9, assign7150_e2913_d_n10, assign7150_e2913_d_n11, assign7150_e2913_d_n14,) = {
    if ((locals.var_guard114 != 0.0) && (locals.var_guard142 != 0.0)) {
        (5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn11, locals.var_uc_depleak_dn14,)
    }
};
        locals.var_uc_depleak = assign7150_e2913;
        locals.var_uc_depleak_dn0 = assign7150_e2913_d_n0;
        locals.var_uc_depleak_dn2 = assign7150_e2913_d_n2;
        locals.var_uc_depleak_dn4 = assign7150_e2913_d_n4;
        locals.var_uc_depleak_dn5 = assign7150_e2913_d_n5;
        locals.var_uc_depleak_dn6 = assign7150_e2913_d_n6;
        locals.var_uc_depleak_dn7 = assign7150_e2913_d_n7;
        locals.var_uc_depleak_dn8 = assign7150_e2913_d_n8;
        locals.var_uc_depleak_dn9 = assign7150_e2913_d_n9;
        locals.var_uc_depleak_dn10 = assign7150_e2913_d_n10;
        locals.var_uc_depleak_dn11 = assign7150_e2913_d_n11;
        locals.var_uc_depleak_dn14 = assign7150_e2913_d_n14;
        locals.var_uc_depleak_rv = 0.0;

        let assign7160_e2916: f64 = if locals.var_uc_codep == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard143 = assign7160_e2916;
        locals.var_guard143_rv = 0.0;

        let assign7190_e2929: f64 = if locals.var_uc_ndepm < 5000000000000000.0 { 1.0 } else { 0.0 };
        locals.var_guard146 = assign7190_e2929;
        locals.var_guard146_rv = 0.0;

        let (assign7200_e2938, assign7200_e2938_d_n0, assign7200_e2938_d_n2, assign7200_e2938_d_n4, assign7200_e2938_d_n5, assign7200_e2938_d_n6, assign7200_e2938_d_n7, assign7200_e2938_d_n8, assign7200_e2938_d_n9, assign7200_e2938_d_n10, assign7200_e2938_d_n11, assign7200_e2938_d_n14,) = {
    if (((locals.var_guard114 == 0.0) && (locals.var_guard143 != 0.0)) && (locals.var_guard146 != 0.0)) {
        (5000000000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn11, locals.var_uc_ndepm_dn14,)
    }
};
        locals.var_uc_ndepm = assign7200_e2938;
        locals.var_uc_ndepm_dn0 = assign7200_e2938_d_n0;
        locals.var_uc_ndepm_dn2 = assign7200_e2938_d_n2;
        locals.var_uc_ndepm_dn4 = assign7200_e2938_d_n4;
        locals.var_uc_ndepm_dn5 = assign7200_e2938_d_n5;
        locals.var_uc_ndepm_dn6 = assign7200_e2938_d_n6;
        locals.var_uc_ndepm_dn7 = assign7200_e2938_d_n7;
        locals.var_uc_ndepm_dn8 = assign7200_e2938_d_n8;
        locals.var_uc_ndepm_dn9 = assign7200_e2938_d_n9;
        locals.var_uc_ndepm_dn10 = assign7200_e2938_d_n10;
        locals.var_uc_ndepm_dn11 = assign7200_e2938_d_n11;
        locals.var_uc_ndepm_dn14 = assign7200_e2938_d_n14;
        locals.var_uc_ndepm_rv = 0.0;

        let assign7210_e2941: f64 = if locals.var_uc_ndepm > 1e18 { 1.0 } else { 0.0 };
        locals.var_guard147 = assign7210_e2941;
        locals.var_guard147_rv = 0.0;

        let (assign7220_e2950, assign7220_e2950_d_n0, assign7220_e2950_d_n2, assign7220_e2950_d_n4, assign7220_e2950_d_n5, assign7220_e2950_d_n6, assign7220_e2950_d_n7, assign7220_e2950_d_n8, assign7220_e2950_d_n9, assign7220_e2950_d_n10, assign7220_e2950_d_n11, assign7220_e2950_d_n14,) = {
    if (((locals.var_guard114 == 0.0) && (locals.var_guard143 != 0.0)) && (locals.var_guard147 != 0.0)) {
        (1e18, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn11, locals.var_uc_ndepm_dn14,)
    }
};
        locals.var_uc_ndepm = assign7220_e2950;
        locals.var_uc_ndepm_dn0 = assign7220_e2950_d_n0;
        locals.var_uc_ndepm_dn2 = assign7220_e2950_d_n2;
        locals.var_uc_ndepm_dn4 = assign7220_e2950_d_n4;
        locals.var_uc_ndepm_dn5 = assign7220_e2950_d_n5;
        locals.var_uc_ndepm_dn6 = assign7220_e2950_d_n6;
        locals.var_uc_ndepm_dn7 = assign7220_e2950_d_n7;
        locals.var_uc_ndepm_dn8 = assign7220_e2950_d_n8;
        locals.var_uc_ndepm_dn9 = assign7220_e2950_d_n9;
        locals.var_uc_ndepm_dn10 = assign7220_e2950_d_n10;
        locals.var_uc_ndepm_dn11 = assign7220_e2950_d_n11;
        locals.var_uc_ndepm_dn14 = assign7220_e2950_d_n14;
        locals.var_uc_ndepm_rv = 0.0;

        let assign7250_e2963: f64 = if locals.var_uc_depthn < 1e-8 { 1.0 } else { 0.0 };
        locals.var_guard150 = assign7250_e2963;
        locals.var_guard150_rv = 0.0;

        let (assign7260_e2972, assign7260_e2972_d_n0, assign7260_e2972_d_n2, assign7260_e2972_d_n4, assign7260_e2972_d_n5, assign7260_e2972_d_n6, assign7260_e2972_d_n7, assign7260_e2972_d_n8, assign7260_e2972_d_n9, assign7260_e2972_d_n10, assign7260_e2972_d_n11, assign7260_e2972_d_n14,) = {
    if (((locals.var_guard114 == 0.0) && (locals.var_guard143 != 0.0)) && (locals.var_guard150 != 0.0)) {
        (1e-8, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depthn, locals.var_uc_depthn_dn0, locals.var_uc_depthn_dn2, locals.var_uc_depthn_dn4, locals.var_uc_depthn_dn5, locals.var_uc_depthn_dn6, locals.var_uc_depthn_dn7, locals.var_uc_depthn_dn8, locals.var_uc_depthn_dn9, locals.var_uc_depthn_dn10, locals.var_uc_depthn_dn11, locals.var_uc_depthn_dn14,)
    }
};
        locals.var_uc_depthn = assign7260_e2972;
        locals.var_uc_depthn_dn0 = assign7260_e2972_d_n0;
        locals.var_uc_depthn_dn2 = assign7260_e2972_d_n2;
        locals.var_uc_depthn_dn4 = assign7260_e2972_d_n4;
        locals.var_uc_depthn_dn5 = assign7260_e2972_d_n5;
        locals.var_uc_depthn_dn6 = assign7260_e2972_d_n6;
        locals.var_uc_depthn_dn7 = assign7260_e2972_d_n7;
        locals.var_uc_depthn_dn8 = assign7260_e2972_d_n8;
        locals.var_uc_depthn_dn9 = assign7260_e2972_d_n9;
        locals.var_uc_depthn_dn10 = assign7260_e2972_d_n10;
        locals.var_uc_depthn_dn11 = assign7260_e2972_d_n11;
        locals.var_uc_depthn_dn14 = assign7260_e2972_d_n14;
        locals.var_uc_depthn_rv = 0.0;

        let assign7270_e2975: f64 = if locals.var_uc_depthn > 1e-6 { 1.0 } else { 0.0 };
        locals.var_guard151 = assign7270_e2975;
        locals.var_guard151_rv = 0.0;

        let (assign7280_e2984, assign7280_e2984_d_n0, assign7280_e2984_d_n2, assign7280_e2984_d_n4, assign7280_e2984_d_n5, assign7280_e2984_d_n6, assign7280_e2984_d_n7, assign7280_e2984_d_n8, assign7280_e2984_d_n9, assign7280_e2984_d_n10, assign7280_e2984_d_n11, assign7280_e2984_d_n14,) = {
    if (((locals.var_guard114 == 0.0) && (locals.var_guard143 != 0.0)) && (locals.var_guard151 != 0.0)) {
        (1e-6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depthn, locals.var_uc_depthn_dn0, locals.var_uc_depthn_dn2, locals.var_uc_depthn_dn4, locals.var_uc_depthn_dn5, locals.var_uc_depthn_dn6, locals.var_uc_depthn_dn7, locals.var_uc_depthn_dn8, locals.var_uc_depthn_dn9, locals.var_uc_depthn_dn10, locals.var_uc_depthn_dn11, locals.var_uc_depthn_dn14,)
    }
};
        locals.var_uc_depthn = assign7280_e2984;
        locals.var_uc_depthn_dn0 = assign7280_e2984_d_n0;
        locals.var_uc_depthn_dn2 = assign7280_e2984_d_n2;
        locals.var_uc_depthn_dn4 = assign7280_e2984_d_n4;
        locals.var_uc_depthn_dn5 = assign7280_e2984_d_n5;
        locals.var_uc_depthn_dn6 = assign7280_e2984_d_n6;
        locals.var_uc_depthn_dn7 = assign7280_e2984_d_n7;
        locals.var_uc_depthn_dn8 = assign7280_e2984_d_n8;
        locals.var_uc_depthn_dn9 = assign7280_e2984_d_n9;
        locals.var_uc_depthn_dn10 = assign7280_e2984_d_n10;
        locals.var_uc_depthn_dn11 = assign7280_e2984_d_n11;
        locals.var_uc_depthn_dn14 = assign7280_e2984_d_n14;
        locals.var_uc_depthn_rv = 0.0;

        let assign7310_e2997: f64 = if locals.var_uc_depmue0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard154 = assign7310_e2997;
        locals.var_guard154_rv = 0.0;

        let (assign7320_e3006, assign7320_e3006_d_n0, assign7320_e3006_d_n2, assign7320_e3006_d_n4, assign7320_e3006_d_n5, assign7320_e3006_d_n6, assign7320_e3006_d_n7, assign7320_e3006_d_n8, assign7320_e3006_d_n9, assign7320_e3006_d_n10, assign7320_e3006_d_n11, assign7320_e3006_d_n14,) = {
    if (((locals.var_guard114 == 0.0) && (locals.var_guard143 != 0.0)) && (locals.var_guard154 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    }
};
        locals.var_uc_depmue0 = assign7320_e3006;
        locals.var_uc_depmue0_dn0 = assign7320_e3006_d_n0;
        locals.var_uc_depmue0_dn2 = assign7320_e3006_d_n2;
        locals.var_uc_depmue0_dn4 = assign7320_e3006_d_n4;
        locals.var_uc_depmue0_dn5 = assign7320_e3006_d_n5;
        locals.var_uc_depmue0_dn6 = assign7320_e3006_d_n6;
        locals.var_uc_depmue0_dn7 = assign7320_e3006_d_n7;
        locals.var_uc_depmue0_dn8 = assign7320_e3006_d_n8;
        locals.var_uc_depmue0_dn9 = assign7320_e3006_d_n9;
        locals.var_uc_depmue0_dn10 = assign7320_e3006_d_n10;
        locals.var_uc_depmue0_dn11 = assign7320_e3006_d_n11;
        locals.var_uc_depmue0_dn14 = assign7320_e3006_d_n14;
        locals.var_uc_depmue0_rv = 0.0;

        let assign7330_e3009: f64 = if locals.var_uc_depmue0 > 10000000000.0 { 1.0 } else { 0.0 };
        locals.var_guard155 = assign7330_e3009;
        locals.var_guard155_rv = 0.0;

        let (assign7340_e3018, assign7340_e3018_d_n0, assign7340_e3018_d_n2, assign7340_e3018_d_n4, assign7340_e3018_d_n5, assign7340_e3018_d_n6, assign7340_e3018_d_n7, assign7340_e3018_d_n8, assign7340_e3018_d_n9, assign7340_e3018_d_n10, assign7340_e3018_d_n11, assign7340_e3018_d_n14,) = {
    if (((locals.var_guard114 == 0.0) && (locals.var_guard143 != 0.0)) && (locals.var_guard155 != 0.0)) {
        (10000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    }
};
        locals.var_uc_depmue0 = assign7340_e3018;
        locals.var_uc_depmue0_dn0 = assign7340_e3018_d_n0;
        locals.var_uc_depmue0_dn2 = assign7340_e3018_d_n2;
        locals.var_uc_depmue0_dn4 = assign7340_e3018_d_n4;
        locals.var_uc_depmue0_dn5 = assign7340_e3018_d_n5;
        locals.var_uc_depmue0_dn6 = assign7340_e3018_d_n6;
        locals.var_uc_depmue0_dn7 = assign7340_e3018_d_n7;
        locals.var_uc_depmue0_dn8 = assign7340_e3018_d_n8;
        locals.var_uc_depmue0_dn9 = assign7340_e3018_d_n9;
        locals.var_uc_depmue0_dn10 = assign7340_e3018_d_n10;
        locals.var_uc_depmue0_dn11 = assign7340_e3018_d_n11;
        locals.var_uc_depmue0_dn14 = assign7340_e3018_d_n14;
        locals.var_uc_depmue0_rv = 0.0;

        let assign7370_e3031: f64 = if locals.var_uc_depmueph1 < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard158 = assign7370_e3031;
        locals.var_guard158_rv = 0.0;

        let (assign7380_e3040,) = {
    if (((locals.var_guard114 == 0.0) && (locals.var_guard143 != 0.0)) && (locals.var_guard158 != 0.0)) {
        (100.0,)
    } else {
        (locals.var_uc_depmueph1,)
    }
};
        locals.var_uc_depmueph1 = assign7380_e3040;
        locals.var_uc_depmueph1_rv = 0.0;

        let assign7390_e3043: f64 = if locals.var_uc_depmueph1 > 2000000000.0 { 1.0 } else { 0.0 };
        locals.var_guard159 = assign7390_e3043;
        locals.var_guard159_rv = 0.0;

        let (assign7400_e3052,) = {
    if (((locals.var_guard114 == 0.0) && (locals.var_guard143 != 0.0)) && (locals.var_guard159 != 0.0)) {
        (2000000000.0,)
    } else {
        (locals.var_uc_depmueph1,)
    }
};
        locals.var_uc_depmueph1 = assign7400_e3052;
        locals.var_uc_depmueph1_rv = 0.0;

        let assign7430_e3065: f64 = if locals.var_uc_depleak < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard162 = assign7430_e3065;
        locals.var_guard162_rv = 0.0;

        let (assign7440_e3074, assign7440_e3074_d_n0, assign7440_e3074_d_n2, assign7440_e3074_d_n4, assign7440_e3074_d_n5, assign7440_e3074_d_n6, assign7440_e3074_d_n7, assign7440_e3074_d_n8, assign7440_e3074_d_n9, assign7440_e3074_d_n10, assign7440_e3074_d_n11, assign7440_e3074_d_n14,) = {
    if (((locals.var_guard114 == 0.0) && (locals.var_guard143 != 0.0)) && (locals.var_guard162 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn11, locals.var_uc_depleak_dn14,)
    }
};
        locals.var_uc_depleak = assign7440_e3074;
        locals.var_uc_depleak_dn0 = assign7440_e3074_d_n0;
        locals.var_uc_depleak_dn2 = assign7440_e3074_d_n2;
        locals.var_uc_depleak_dn4 = assign7440_e3074_d_n4;
        locals.var_uc_depleak_dn5 = assign7440_e3074_d_n5;
        locals.var_uc_depleak_dn6 = assign7440_e3074_d_n6;
        locals.var_uc_depleak_dn7 = assign7440_e3074_d_n7;
        locals.var_uc_depleak_dn8 = assign7440_e3074_d_n8;
        locals.var_uc_depleak_dn9 = assign7440_e3074_d_n9;
        locals.var_uc_depleak_dn10 = assign7440_e3074_d_n10;
        locals.var_uc_depleak_dn11 = assign7440_e3074_d_n11;
        locals.var_uc_depleak_dn14 = assign7440_e3074_d_n14;
        locals.var_uc_depleak_rv = 0.0;

        let assign7450_e3077: f64 = if locals.var_uc_depleak > 5.0 { 1.0 } else { 0.0 };
        locals.var_guard163 = assign7450_e3077;
        locals.var_guard163_rv = 0.0;

        let (assign7460_e3086, assign7460_e3086_d_n0, assign7460_e3086_d_n2, assign7460_e3086_d_n4, assign7460_e3086_d_n5, assign7460_e3086_d_n6, assign7460_e3086_d_n7, assign7460_e3086_d_n8, assign7460_e3086_d_n9, assign7460_e3086_d_n10, assign7460_e3086_d_n11, assign7460_e3086_d_n14,) = {
    if (((locals.var_guard114 == 0.0) && (locals.var_guard143 != 0.0)) && (locals.var_guard163 != 0.0)) {
        (5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn11, locals.var_uc_depleak_dn14,)
    }
};
        locals.var_uc_depleak = assign7460_e3086;
        locals.var_uc_depleak_dn0 = assign7460_e3086_d_n0;
        locals.var_uc_depleak_dn2 = assign7460_e3086_d_n2;
        locals.var_uc_depleak_dn4 = assign7460_e3086_d_n4;
        locals.var_uc_depleak_dn5 = assign7460_e3086_d_n5;
        locals.var_uc_depleak_dn6 = assign7460_e3086_d_n6;
        locals.var_uc_depleak_dn7 = assign7460_e3086_d_n7;
        locals.var_uc_depleak_dn8 = assign7460_e3086_d_n8;
        locals.var_uc_depleak_dn9 = assign7460_e3086_d_n9;
        locals.var_uc_depleak_dn10 = assign7460_e3086_d_n10;
        locals.var_uc_depleak_dn11 = assign7460_e3086_d_n11;
        locals.var_uc_depleak_dn14 = assign7460_e3086_d_n14;
        locals.var_uc_depleak_rv = 0.0;

        locals.var_uc_toxb = p.p96;
        locals.var_uc_toxb_rv = 0.0;

        let assign7560_e3124: f64 = if locals.var_uc_toxb < p.p95 { 1.0 } else { 0.0 };
        locals.var_guard172 = assign7560_e3124;
        locals.var_guard172_rv = 0.0;

        let (assign7570_e3128,) = {
    if (locals.var_guard172 != 0.0) {
        (p.p95,)
    } else {
        (locals.var_uc_toxb,)
    }
};
        locals.var_uc_toxb = assign7570_e3128;
        locals.var_uc_toxb_rv = 0.0;

        let assign7580_e3131: f64 = if locals.var_uc_toxb > 5e-7 { 1.0 } else { 0.0 };
        locals.var_guard173 = assign7580_e3131;
        locals.var_guard173_rv = 0.0;

        let (assign7590_e3135,) = {
    if (locals.var_guard173 != 0.0) {
        (5e-7,)
    } else {
        (locals.var_uc_toxb,)
    }
};
        locals.var_uc_toxb = assign7590_e3135;
        locals.var_uc_toxb_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign7600_e3139: f64 = (100.0_f64).powf(p.p122);
        let assign7600_e3140: f64 = (p.p120 / assign7600_e3139);
        locals.var_mks_ll = assign7600_e3140;
        locals.var_mks_ll_rv = 0.0;

        let assign7610_e3144: f64 = (100.0_f64).powf(p.p129);
        let assign7610_e3145: f64 = (p.p123 / assign7610_e3144);
        locals.var_mks_wl = assign7610_e3145;
        locals.var_mks_wl_rv = 0.0;

        let assign7620_e3149: f64 = (100.0_f64).powf(p.p199);
        let assign7620_e3150: f64 = (p.p198 / assign7620_e3149);
        locals.var_mks_svgsl = assign7620_e3150;
        locals.var_mks_svgsl_rv = 0.0;

        let assign7630_e3154: f64 = (100.0_f64).powf(p.p201);
        let assign7630_e3155: f64 = (p.p200 / assign7630_e3154);
        locals.var_mks_svgsw = assign7630_e3155;
        locals.var_mks_svgsw_rv = 0.0;

        let assign7640_e3159: f64 = (100.0_f64).powf(p.p184);
        let assign7640_e3160: f64 = (p.p183 / assign7640_e3159);
        locals.var_mks_svbsl = assign7640_e3160;
        locals.var_mks_svbsl_rv = 0.0;

        let assign7650_e3164: f64 = (100.0_f64).powf(p.p203);
        let assign7650_e3165: f64 = (p.p202 / assign7650_e3164);
        locals.var_mks_slgl = assign7650_e3165;
        locals.var_mks_slgl_rv = 0.0;

        let assign7660_e3169: f64 = (100.0_f64).powf(p.p191);
        let assign7660_e3170: f64 = (p.p190 / assign7660_e3169);
        locals.var_mks_sub1l = assign7660_e3170;
        locals.var_mks_sub1l_rv = 0.0;

        let assign7670_e3173: f64 = (p.p186 / 100.0);
        locals.var_mks_slg = assign7670_e3173;
        locals.var_mks_slg_rv = 0.0;

        let assign7680_e3176: f64 = (p.p192 / 100.0);
        locals.var_mks_sub2l = assign7680_e3176;
        locals.var_mks_sub2l_rv = 0.0;

        let assign7690_e3179: f64 = (p.p73 * 100.0);
        locals.var_mks_subld2 = assign7690_e3179;
        locals.var_mks_subld2_rv = 0.0;

        let assign7700_e3182: f64 = (p.p311 / 100.0);
        locals.var_mks_rdtemp1 = assign7700_e3182;
        locals.var_mks_rdtemp1_rv = 0.0;

        let assign7710_e3185: f64 = (p.p312 / 100.0);
        locals.var_mks_rdtemp2 = assign7710_e3185;
        locals.var_mks_rdtemp2_rv = 0.0;

        let assign7720_e3188: f64 = (p.p313 / 100.0);
        locals.var_mks_rdvdtemp1 = assign7720_e3188;
        locals.var_mks_rdvdtemp1_rv = 0.0;

        let assign7730_e3191: f64 = (p.p314 / 100.0);
        locals.var_mks_rdvdtemp2 = assign7730_e3191;
        locals.var_mks_rdvdtemp2_rv = 0.0;

        let assign7740_e3194: f64 = (p.p336 / 1e-6);
        locals.var_mks_nsubsub = assign7740_e3194;
        locals.var_mks_nsubsub_rv = 0.0;

        let assign7750_e3197: f64 = (p.p255 * 100.0);
        locals.var_mks_glksd3 = assign7750_e3197;
        locals.var_mks_glksd3_rv = 0.0;

        let assign7760_e3200: f64 = (p.p248 * 100.0);
        locals.var_mks_gleak4 = assign7760_e3200;
        locals.var_mks_gleak4_rv = 0.0;

        let assign7770_e3203: f64 = (p.p249 * 100.0);
        locals.var_mks_gleak5 = assign7770_e3203;
        locals.var_mks_gleak5_rv = 0.0;

        let assign7780_e3206: f64 = (p.p251 / 10000.0);
        locals.var_mks_gleak7 = assign7780_e3206;
        locals.var_mks_gleak7_rv = 0.0;

        let assign7790_e3209: f64 = (p.p266 * 10000.0);
        locals.var_mks_cit = assign7790_e3209;
        locals.var_mks_cit_rv = 0.0;

        let assign7800_e3212: f64 = (p.p275 / 100.0);
        locals.var_mks_ovslp = assign7800_e3212;
        locals.var_mks_ovslp_rv = 0.0;

        let assign7810_e3215: f64 = (p.p272 / 10000.0);
        locals.var_mks_dly3 = assign7810_e3215;
        locals.var_mks_dly3_rv = 0.0;

        let assign7820_e3218: f64 = (p.p273 / 10000.0);
        locals.var_mks_dlyov = assign7820_e3218;
        locals.var_mks_dlyov_dn0 = 0.0;
        locals.var_mks_dlyov_dn2 = 0.0;
        locals.var_mks_dlyov_dn4 = 0.0;
        locals.var_mks_dlyov_dn5 = 0.0;
        locals.var_mks_dlyov_dn6 = 0.0;
        locals.var_mks_dlyov_dn7 = 0.0;
        locals.var_mks_dlyov_dn8 = 0.0;
        locals.var_mks_dlyov_dn9 = 0.0;
        locals.var_mks_dlyov_dn10 = 0.0;
        locals.var_mks_dlyov_dn11 = 0.0;
        locals.var_mks_dlyov_dn14 = 0.0;
        locals.var_mks_dlyov_rv = 0.0;

        let assign7840_e3224: f64 = (p.p409 / 10000.0);
        locals.var_mks_rdrmue = assign7840_e3224;
        locals.var_mks_rdrmue_rv = 0.0;

        let assign7850_e3227: f64 = (p.p412 / 100.0);
        locals.var_mks_rdrvmax = assign7850_e3227;
        locals.var_mks_rdrvmax_rv = 0.0;

        let assign7860_e3230: f64 = (p.p413 / 10000.0);
        locals.var_mks_rdrmues = assign7860_e3230;
        locals.var_mks_rdrmues_rv = 0.0;

        let assign7870_e3233: f64 = (p.p414 / 100.0);
        locals.var_mks_rdrvmaxs = assign7870_e3233;
        locals.var_mks_rdrvmaxs_rv = 0.0;

        let assign7880_e3236: f64 = (locals.var_uc_ndepm / 1e-6);
        locals.var_uc_ndepm = assign7880_e3236;
        locals.var_uc_ndepm_dn0 = (locals.var_uc_ndepm_dn0 / 1e-6);
        locals.var_uc_ndepm_dn2 = (locals.var_uc_ndepm_dn2 / 1e-6);
        locals.var_uc_ndepm_dn4 = (locals.var_uc_ndepm_dn4 / 1e-6);
        locals.var_uc_ndepm_dn5 = (locals.var_uc_ndepm_dn5 / 1e-6);
        locals.var_uc_ndepm_dn6 = (locals.var_uc_ndepm_dn6 / 1e-6);
        locals.var_uc_ndepm_dn7 = (locals.var_uc_ndepm_dn7 / 1e-6);
        locals.var_uc_ndepm_dn8 = (locals.var_uc_ndepm_dn8 / 1e-6);
        locals.var_uc_ndepm_dn9 = (locals.var_uc_ndepm_dn9 / 1e-6);
        locals.var_uc_ndepm_dn10 = (locals.var_uc_ndepm_dn10 / 1e-6);
        locals.var_uc_ndepm_dn11 = (locals.var_uc_ndepm_dn11 / 1e-6);
        locals.var_uc_ndepm_dn14 = (locals.var_uc_ndepm_dn14 / 1e-6);
        locals.var_uc_ndepm_rv = 0.0;

        let assign7890_e3239: f64 = (p.p453 / 1e-6);
        locals.var_uc_njunc = assign7890_e3239;
        locals.var_uc_njunc_rv = 0.0;

        let assign7900_e3242: f64 = (p.p274 + 273.15);
        locals.var_ktnom = assign7900_e3242;
        locals.var_ktnom_rv = 0.0;

        let assign7950_e3265: f64 = (p.p0 + p.p116);
        locals.var_lgate = assign7950_e3265;
        locals.var_lgate_rv = 0.0;

        let assign7960_e3268: f64 = (p.p1 / p.p7);
        let assign7960_e3270: f64 = (assign7960_e3268 + p.p117);
        locals.var_wgate = assign7960_e3270;
        locals.var_wgate_rv = 0.0;

        let assign8110_e3370: f64 = (locals.var_lgate * 1000000.0);
        locals.var_lg = assign8110_e3370;
        locals.var_lg_rv = 0.0;

        let assign8120_e3373: f64 = (locals.var_wgate * 1000000.0);
        locals.var_wg = assign8120_e3373;
        locals.var_wg_rv = 0.0;

        let assign8130_e3376: f64 = (locals.var_lg).powf(p.p553);
        locals.var_lbin = assign8130_e3376;
        locals.var_lbin_rv = 0.0;

        let assign8140_e3379: f64 = (locals.var_wg).powf(p.p554);
        locals.var_wbin = assign8140_e3379;
        locals.var_wbin_rv = 0.0;

        let assign8150_e3382: f64 = (locals.var_lbin * locals.var_wbin);
        locals.var_lwbin = assign8150_e3382;
        locals.var_lwbin_rv = 0.0;

        let assign8160_e3386: f64 = (p.p555 / locals.var_lbin);
        let assign8160_e3387: f64 = (p.p89 + assign8160_e3386);
        let assign8160_e3390: f64 = (p.p643 / locals.var_wbin);
        let assign8160_e3391: f64 = (assign8160_e3387 + assign8160_e3390);
        let assign8160_e3394: f64 = (p.p731 / locals.var_lwbin);
        let assign8160_e3395: f64 = (assign8160_e3391 + assign8160_e3394);
        locals.var_uc_vmax = assign8160_e3395;
        locals.var_uc_vmax_rv = 0.0;

        let assign8170_e3399: f64 = (p.p556 / locals.var_lbin);
        let assign8170_e3400: f64 = (p.p92 + assign8170_e3399);
        let assign8170_e3403: f64 = (p.p644 / locals.var_wbin);
        let assign8170_e3404: f64 = (assign8170_e3400 + assign8170_e3403);
        let assign8170_e3407: f64 = (p.p732 / locals.var_lwbin);
        let assign8170_e3408: f64 = (assign8170_e3404 + assign8170_e3407);
        locals.var_uc_bgtmp1 = assign8170_e3408;
        locals.var_uc_bgtmp1_rv = 0.0;

        let assign8180_e3412: f64 = (p.p557 / locals.var_lbin);
        let assign8180_e3413: f64 = (p.p93 + assign8180_e3412);
        let assign8180_e3416: f64 = (p.p645 / locals.var_wbin);
        let assign8180_e3417: f64 = (assign8180_e3413 + assign8180_e3416);
        let assign8180_e3420: f64 = (p.p733 / locals.var_lwbin);
        let assign8180_e3421: f64 = (assign8180_e3417 + assign8180_e3420);
        locals.var_uc_bgtmp2 = assign8180_e3421;
        locals.var_uc_bgtmp2_rv = 0.0;

        let assign8190_e3425: f64 = (p.p558 / locals.var_lbin);
        let assign8190_e3426: f64 = (p.p94 + assign8190_e3425);
        let assign8190_e3429: f64 = (p.p646 / locals.var_wbin);
        let assign8190_e3430: f64 = (assign8190_e3426 + assign8190_e3429);
        let assign8190_e3433: f64 = (p.p734 / locals.var_lwbin);
        let assign8190_e3434: f64 = (assign8190_e3430 + assign8190_e3433);
        locals.var_uc_eg0 = assign8190_e3434;
        locals.var_uc_eg0_rv = 0.0;

        let assign8200_e3438: f64 = (p.p559 / locals.var_lbin);
        let assign8200_e3439: f64 = (p.p110 + assign8200_e3438);
        let assign8200_e3442: f64 = (p.p647 / locals.var_wbin);
        let assign8200_e3443: f64 = (assign8200_e3439 + assign8200_e3442);
        let assign8200_e3446: f64 = (p.p735 / locals.var_lwbin);
        let assign8200_e3447: f64 = (assign8200_e3443 + assign8200_e3446);
        locals.var_uc_vfbover = assign8200_e3447;
        locals.var_uc_vfbover_rv = 0.0;

        let assign8210_e3451: f64 = (p.p560 / locals.var_lbin);
        let assign8210_e3452: f64 = (p.p111 + assign8210_e3451);
        let assign8210_e3455: f64 = (p.p648 / locals.var_wbin);
        let assign8210_e3456: f64 = (assign8210_e3452 + assign8210_e3455);
        let assign8210_e3459: f64 = (p.p736 / locals.var_lwbin);
        let assign8210_e3460: f64 = (assign8210_e3456 + assign8210_e3459);
        locals.var_uc_nover = assign8210_e3460;
        locals.var_uc_nover_rv = 0.0;

        let assign8220_e3464: f64 = (p.p561 / locals.var_lbin);
        let assign8220_e3465: f64 = (p.p112 + assign8220_e3464);
        let assign8220_e3468: f64 = (p.p649 / locals.var_wbin);
        let assign8220_e3469: f64 = (assign8220_e3465 + assign8220_e3468);
        let assign8220_e3472: f64 = (p.p737 / locals.var_lwbin);
        let assign8220_e3473: f64 = (assign8220_e3469 + assign8220_e3472);
        locals.var_uc_novers = assign8220_e3473;
        locals.var_uc_novers_rv = 0.0;

        let assign8230_e3477: f64 = (p.p562 / locals.var_lbin);
        let assign8230_e3478: f64 = (p.p126 + assign8230_e3477);
        let assign8230_e3481: f64 = (p.p650 / locals.var_wbin);
        let assign8230_e3482: f64 = (assign8230_e3478 + assign8230_e3481);
        let assign8230_e3485: f64 = (p.p738 / locals.var_lwbin);
        let assign8230_e3486: f64 = (assign8230_e3482 + assign8230_e3485);
        locals.var_uc_wl2 = assign8230_e3486;
        locals.var_uc_wl2_rv = 0.0;

        let assign8240_e3490: f64 = (p.p563 / locals.var_lbin);
        let assign8240_e3491: f64 = (p.p136 + assign8240_e3490);
        let assign8240_e3494: f64 = (p.p651 / locals.var_wbin);
        let assign8240_e3495: f64 = (assign8240_e3491 + assign8240_e3494);
        let assign8240_e3498: f64 = (p.p739 / locals.var_lwbin);
        let assign8240_e3499: f64 = (assign8240_e3495 + assign8240_e3498);
        locals.var_uc_vfbc = assign8240_e3499;
        locals.var_uc_vfbc_rv = 0.0;

        let assign8250_e3503: f64 = (p.p564 / locals.var_lbin);
        let assign8250_e3504: f64 = (p.p138 + assign8250_e3503);
        let assign8250_e3507: f64 = (p.p652 / locals.var_wbin);
        let assign8250_e3508: f64 = (assign8250_e3504 + assign8250_e3507);
        let assign8250_e3511: f64 = (p.p740 / locals.var_lwbin);
        let assign8250_e3512: f64 = (assign8250_e3508 + assign8250_e3511);
        locals.var_uc_nsubc = assign8250_e3512;
        locals.var_uc_nsubc_rv = 0.0;

        let assign8260_e3516: f64 = (p.p565 / locals.var_lbin);
        let assign8260_e3517: f64 = (p.p141 + assign8260_e3516);
        let assign8260_e3520: f64 = (p.p653 / locals.var_wbin);
        let assign8260_e3521: f64 = (assign8260_e3517 + assign8260_e3520);
        let assign8260_e3524: f64 = (p.p741 / locals.var_lwbin);
        let assign8260_e3525: f64 = (assign8260_e3521 + assign8260_e3524);
        locals.var_uc_nsubp = assign8260_e3525;
        locals.var_uc_nsubp_rv = 0.0;

        let assign8270_e3529: f64 = (p.p566 / locals.var_lbin);
        let assign8270_e3530: f64 = (p.p144 + assign8270_e3529);
        let assign8270_e3533: f64 = (p.p654 / locals.var_wbin);
        let assign8270_e3534: f64 = (assign8270_e3530 + assign8270_e3533);
        let assign8270_e3537: f64 = (p.p742 / locals.var_lwbin);
        let assign8270_e3538: f64 = (assign8270_e3534 + assign8270_e3537);
        locals.var_uc_scp1 = assign8270_e3538;
        locals.var_uc_scp1_rv = 0.0;

        let assign8280_e3542: f64 = (p.p567 / locals.var_lbin);
        let assign8280_e3543: f64 = (p.p145 + assign8280_e3542);
        let assign8280_e3546: f64 = (p.p655 / locals.var_wbin);
        let assign8280_e3547: f64 = (assign8280_e3543 + assign8280_e3546);
        let assign8280_e3550: f64 = (p.p743 / locals.var_lwbin);
        let assign8280_e3551: f64 = (assign8280_e3547 + assign8280_e3550);
        locals.var_uc_scp2 = assign8280_e3551;
        locals.var_uc_scp2_rv = 0.0;

        let assign8290_e3555: f64 = (p.p568 / locals.var_lbin);
        let assign8290_e3556: f64 = (p.p146 + assign8290_e3555);
        let assign8290_e3559: f64 = (p.p656 / locals.var_wbin);
        let assign8290_e3560: f64 = (assign8290_e3556 + assign8290_e3559);
        let assign8290_e3563: f64 = (p.p744 / locals.var_lwbin);
        let assign8290_e3564: f64 = (assign8290_e3560 + assign8290_e3563);
        locals.var_uc_scp3 = assign8290_e3564;
        locals.var_uc_scp3_rv = 0.0;

        let assign8300_e3568: f64 = (p.p569 / locals.var_lbin);
        let assign8300_e3569: f64 = (p.p147 + assign8300_e3568);
        let assign8300_e3572: f64 = (p.p657 / locals.var_wbin);
        let assign8300_e3573: f64 = (assign8300_e3569 + assign8300_e3572);
        let assign8300_e3576: f64 = (p.p745 / locals.var_lwbin);
        let assign8300_e3577: f64 = (assign8300_e3573 + assign8300_e3576);
        locals.var_uc_sc1 = assign8300_e3577;
        locals.var_uc_sc1_rv = 0.0;

        let assign8310_e3581: f64 = (p.p570 / locals.var_lbin);
        let assign8310_e3582: f64 = (p.p148 + assign8310_e3581);
        let assign8310_e3585: f64 = (p.p658 / locals.var_wbin);
        let assign8310_e3586: f64 = (assign8310_e3582 + assign8310_e3585);
        let assign8310_e3589: f64 = (p.p746 / locals.var_lwbin);
        let assign8310_e3590: f64 = (assign8310_e3586 + assign8310_e3589);
        locals.var_uc_sc2 = assign8310_e3590;
        locals.var_uc_sc2_rv = 0.0;

        let assign8320_e3594: f64 = (p.p571 / locals.var_lbin);
        let assign8320_e3595: f64 = (p.p149 + assign8320_e3594);
        let assign8320_e3598: f64 = (p.p659 / locals.var_wbin);
        let assign8320_e3599: f64 = (assign8320_e3595 + assign8320_e3598);
        let assign8320_e3602: f64 = (p.p747 / locals.var_lwbin);
        let assign8320_e3603: f64 = (assign8320_e3599 + assign8320_e3602);
        locals.var_uc_sc3 = assign8320_e3603;
        locals.var_uc_sc3_rv = 0.0;

        let assign8330_e3607: f64 = (p.p572 / locals.var_lbin);
        let assign8330_e3608: f64 = (p.p151 + assign8330_e3607);
        let assign8330_e3611: f64 = (p.p660 / locals.var_wbin);
        let assign8330_e3612: f64 = (assign8330_e3608 + assign8330_e3611);
        let assign8330_e3615: f64 = (p.p748 / locals.var_lwbin);
        let assign8330_e3616: f64 = (assign8330_e3612 + assign8330_e3615);
        locals.var_uc_pgd1 = assign8330_e3616;
        locals.var_uc_pgd1_rv = 0.0;

        let assign8340_e3620: f64 = (p.p573 / locals.var_lbin);
        let assign8340_e3621: f64 = (p.p154 + assign8340_e3620);
        let assign8340_e3624: f64 = (p.p661 / locals.var_wbin);
        let assign8340_e3625: f64 = (assign8340_e3621 + assign8340_e3624);
        let assign8340_e3628: f64 = (p.p749 / locals.var_lwbin);
        let assign8340_e3629: f64 = (assign8340_e3625 + assign8340_e3628);
        locals.var_uc_ndep = assign8340_e3629;
        locals.var_uc_ndep_rv = 0.0;

        let assign8350_e3633: f64 = (p.p574 / locals.var_lbin);
        let assign8350_e3634: f64 = (p.p157 + assign8350_e3633);
        let assign8350_e3637: f64 = (p.p662 / locals.var_wbin);
        let assign8350_e3638: f64 = (assign8350_e3634 + assign8350_e3637);
        let assign8350_e3641: f64 = (p.p750 / locals.var_lwbin);
        let assign8350_e3642: f64 = (assign8350_e3638 + assign8350_e3641);
        locals.var_uc_ninv = assign8350_e3642;
        locals.var_uc_ninv_rv = 0.0;

        let assign8360_e3646: f64 = (p.p575 / locals.var_lbin);
        let assign8360_e3647: f64 = (p.p158 + assign8360_e3646);
        let assign8360_e3650: f64 = (p.p663 / locals.var_wbin);
        let assign8360_e3651: f64 = (assign8360_e3647 + assign8360_e3650);
        let assign8360_e3654: f64 = (p.p751 / locals.var_lwbin);
        let assign8360_e3655: f64 = (assign8360_e3651 + assign8360_e3654);
        locals.var_uc_muecb0 = assign8360_e3655;
        locals.var_uc_muecb0_rv = 0.0;

        let assign8370_e3659: f64 = (p.p576 / locals.var_lbin);
        let assign8370_e3660: f64 = (p.p159 + assign8370_e3659);
        let assign8370_e3663: f64 = (p.p664 / locals.var_wbin);
        let assign8370_e3664: f64 = (assign8370_e3660 + assign8370_e3663);
        let assign8370_e3667: f64 = (p.p752 / locals.var_lwbin);
        let assign8370_e3668: f64 = (assign8370_e3664 + assign8370_e3667);
        locals.var_uc_muecb1 = assign8370_e3668;
        locals.var_uc_muecb1_rv = 0.0;

        let assign8380_e3672: f64 = (p.p577 / locals.var_lbin);
        let assign8380_e3673: f64 = (p.p161 + assign8380_e3672);
        let assign8380_e3676: f64 = (p.p665 / locals.var_wbin);
        let assign8380_e3677: f64 = (assign8380_e3673 + assign8380_e3676);
        let assign8380_e3680: f64 = (p.p753 / locals.var_lwbin);
        let assign8380_e3681: f64 = (assign8380_e3677 + assign8380_e3680);
        locals.var_uc_mueph1 = assign8380_e3681;
        locals.var_uc_mueph1_rv = 0.0;

        let assign8390_e3685: f64 = (p.p578 / locals.var_lbin);
        let assign8390_e3686: f64 = (p.p169 + assign8390_e3685);
        let assign8390_e3689: f64 = (p.p666 / locals.var_wbin);
        let assign8390_e3690: f64 = (assign8390_e3686 + assign8390_e3689);
        let assign8390_e3693: f64 = (p.p754 / locals.var_lwbin);
        let assign8390_e3694: f64 = (assign8390_e3690 + assign8390_e3693);
        locals.var_uc_vtmp = assign8390_e3694;
        locals.var_uc_vtmp_rv = 0.0;

        let assign8400_e3698: f64 = (p.p579 / locals.var_lbin);
        let assign8400_e3699: f64 = (p.p170 + assign8400_e3698);
        let assign8400_e3702: f64 = (p.p667 / locals.var_wbin);
        let assign8400_e3703: f64 = (assign8400_e3699 + assign8400_e3702);
        let assign8400_e3706: f64 = (p.p755 / locals.var_lwbin);
        let assign8400_e3707: f64 = (assign8400_e3703 + assign8400_e3706);
        locals.var_uc_wvth0 = assign8400_e3707;
        locals.var_uc_wvth0_rv = 0.0;

        let assign8410_e3711: f64 = (p.p580 / locals.var_lbin);
        let assign8410_e3712: f64 = (p.p172 + assign8410_e3711);
        let assign8410_e3715: f64 = (p.p668 / locals.var_wbin);
        let assign8410_e3716: f64 = (assign8410_e3712 + assign8410_e3715);
        let assign8410_e3719: f64 = (p.p756 / locals.var_lwbin);
        let assign8410_e3720: f64 = (assign8410_e3716 + assign8410_e3719);
        locals.var_uc_muesr1 = assign8410_e3720;
        locals.var_uc_muesr1_rv = 0.0;

        let assign8420_e3724: f64 = (p.p581 / locals.var_lbin);
        let assign8420_e3725: f64 = (p.p177 + assign8420_e3724);
        let assign8420_e3728: f64 = (p.p669 / locals.var_wbin);
        let assign8420_e3729: f64 = (assign8420_e3725 + assign8420_e3728);
        let assign8420_e3732: f64 = (p.p757 / locals.var_lwbin);
        let assign8420_e3733: f64 = (assign8420_e3729 + assign8420_e3732);
        locals.var_uc_muetmp = assign8420_e3733;
        locals.var_uc_muetmp_rv = 0.0;

        let assign8430_e3737: f64 = (p.p582 / locals.var_lbin);
        let assign8430_e3738: f64 = (p.p179 + assign8430_e3737);
        let assign8430_e3741: f64 = (p.p670 / locals.var_wbin);
        let assign8430_e3742: f64 = (assign8430_e3738 + assign8430_e3741);
        let assign8430_e3745: f64 = (p.p758 / locals.var_lwbin);
        let assign8430_e3746: f64 = (assign8430_e3742 + assign8430_e3745);
        locals.var_uc_sub1 = assign8430_e3746;
        locals.var_uc_sub1_rv = 0.0;

        let assign8440_e3750: f64 = (p.p583 / locals.var_lbin);
        let assign8440_e3751: f64 = (p.p180 + assign8440_e3750);
        let assign8440_e3754: f64 = (p.p671 / locals.var_wbin);
        let assign8440_e3755: f64 = (assign8440_e3751 + assign8440_e3754);
        let assign8440_e3758: f64 = (p.p759 / locals.var_lwbin);
        let assign8440_e3759: f64 = (assign8440_e3755 + assign8440_e3758);
        locals.var_uc_sub2 = assign8440_e3759;
        locals.var_uc_sub2_rv = 0.0;

        let assign8450_e3763: f64 = (p.p584 / locals.var_lbin);
        let assign8450_e3764: f64 = (p.p185 + assign8450_e3763);
        let assign8450_e3767: f64 = (p.p672 / locals.var_wbin);
        let assign8450_e3768: f64 = (assign8450_e3764 + assign8450_e3767);
        let assign8450_e3771: f64 = (p.p760 / locals.var_lwbin);
        let assign8450_e3772: f64 = (assign8450_e3768 + assign8450_e3771);
        locals.var_uc_svds = assign8450_e3772;
        locals.var_uc_svds_rv = 0.0;

        let assign8460_e3776: f64 = (p.p585 / locals.var_lbin);
        let assign8460_e3777: f64 = (p.p182 + assign8460_e3776);
        let assign8460_e3780: f64 = (p.p673 / locals.var_wbin);
        let assign8460_e3781: f64 = (assign8460_e3777 + assign8460_e3780);
        let assign8460_e3784: f64 = (p.p761 / locals.var_lwbin);
        let assign8460_e3785: f64 = (assign8460_e3781 + assign8460_e3784);
        locals.var_uc_svbs = assign8460_e3785;
        locals.var_uc_svbs_rv = 0.0;

        let assign8470_e3789: f64 = (p.p586 / locals.var_lbin);
        let assign8470_e3790: f64 = (p.p181 + assign8470_e3789);
        let assign8470_e3793: f64 = (p.p674 / locals.var_wbin);
        let assign8470_e3794: f64 = (assign8470_e3790 + assign8470_e3793);
        let assign8470_e3797: f64 = (p.p762 / locals.var_lwbin);
        let assign8470_e3798: f64 = (assign8470_e3794 + assign8470_e3797);
        locals.var_uc_svgs = assign8470_e3798;
        locals.var_uc_svgs_rv = 0.0;

        let assign8480_e3802: f64 = (p.p587 / locals.var_lbin);
        let assign8480_e3803: f64 = (p.p187 + assign8480_e3802);
        let assign8480_e3806: f64 = (p.p675 / locals.var_wbin);
        let assign8480_e3807: f64 = (assign8480_e3803 + assign8480_e3806);
        let assign8480_e3810: f64 = (p.p763 / locals.var_lwbin);
        let assign8480_e3811: f64 = (assign8480_e3807 + assign8480_e3810);
        locals.var_uc_sub1snp = assign8480_e3811;
        locals.var_uc_sub1snp_rv = 0.0;

        let assign8490_e3815: f64 = (p.p588 / locals.var_lbin);
        let assign8490_e3816: f64 = (p.p188 + assign8490_e3815);
        let assign8490_e3819: f64 = (p.p676 / locals.var_wbin);
        let assign8490_e3820: f64 = (assign8490_e3816 + assign8490_e3819);
        let assign8490_e3823: f64 = (p.p764 / locals.var_lwbin);
        let assign8490_e3824: f64 = (assign8490_e3820 + assign8490_e3823);
        locals.var_uc_sub2snp = assign8490_e3824;
        locals.var_uc_sub2snp_rv = 0.0;

        let assign8500_e3828: f64 = (p.p589 / locals.var_lbin);
        let assign8500_e3829: f64 = (p.p189 + assign8500_e3828);
        let assign8500_e3832: f64 = (p.p677 / locals.var_wbin);
        let assign8500_e3833: f64 = (assign8500_e3829 + assign8500_e3832);
        let assign8500_e3836: f64 = (p.p765 / locals.var_lwbin);
        let assign8500_e3837: f64 = (assign8500_e3833 + assign8500_e3836);
        locals.var_uc_svdssnp = assign8500_e3837;
        locals.var_uc_svdssnp_rv = 0.0;

        let assign8510_e3841: f64 = (p.p590 / locals.var_lbin);
        let assign8510_e3842: f64 = (p.p194 + assign8510_e3841);
        let assign8510_e3845: f64 = (p.p678 / locals.var_wbin);
        let assign8510_e3846: f64 = (assign8510_e3842 + assign8510_e3845);
        let assign8510_e3849: f64 = (p.p766 / locals.var_lwbin);
        let assign8510_e3850: f64 = (assign8510_e3846 + assign8510_e3849);
        locals.var_uc_fn1 = assign8510_e3850;
        locals.var_uc_fn1_rv = 0.0;

        let assign8520_e3854: f64 = (p.p591 / locals.var_lbin);
        let assign8520_e3855: f64 = (p.p195 + assign8520_e3854);
        let assign8520_e3858: f64 = (p.p679 / locals.var_wbin);
        let assign8520_e3859: f64 = (assign8520_e3855 + assign8520_e3858);
        let assign8520_e3862: f64 = (p.p767 / locals.var_lwbin);
        let assign8520_e3863: f64 = (assign8520_e3859 + assign8520_e3862);
        locals.var_uc_fn2 = assign8520_e3863;
        locals.var_uc_fn2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign8530_e3867: f64 = (p.p592 / locals.var_lbin);
        let assign8530_e3868: f64 = (p.p196 + assign8530_e3867);
        let assign8530_e3871: f64 = (p.p680 / locals.var_wbin);
        let assign8530_e3872: f64 = (assign8530_e3868 + assign8530_e3871);
        let assign8530_e3875: f64 = (p.p768 / locals.var_lwbin);
        let assign8530_e3876: f64 = (assign8530_e3872 + assign8530_e3875);
        locals.var_uc_fn3 = assign8530_e3876;
        locals.var_uc_fn3_rv = 0.0;

        let assign8540_e3880: f64 = (p.p593 / locals.var_lbin);
        let assign8540_e3881: f64 = (p.p197 + assign8540_e3880);
        let assign8540_e3884: f64 = (p.p681 / locals.var_wbin);
        let assign8540_e3885: f64 = (assign8540_e3881 + assign8540_e3884);
        let assign8540_e3888: f64 = (p.p769 / locals.var_lwbin);
        let assign8540_e3889: f64 = (assign8540_e3885 + assign8540_e3888);
        locals.var_uc_fvbs = assign8540_e3889;
        locals.var_uc_fvbs_rv = 0.0;

        let assign8550_e3893: f64 = (p.p594 / locals.var_lbin);
        let assign8550_e3894: f64 = (p.p204 + assign8550_e3893);
        let assign8550_e3897: f64 = (p.p682 / locals.var_wbin);
        let assign8550_e3898: f64 = (assign8550_e3894 + assign8550_e3897);
        let assign8550_e3901: f64 = (p.p770 / locals.var_lwbin);
        let assign8550_e3902: f64 = (assign8550_e3898 + assign8550_e3901);
        locals.var_uc_nsti = assign8550_e3902;
        locals.var_uc_nsti_rv = 0.0;

        let assign8560_e3906: f64 = (p.p595 / locals.var_lbin);
        let assign8560_e3907: f64 = (p.p205 + assign8560_e3906);
        let assign8560_e3910: f64 = (p.p683 / locals.var_wbin);
        let assign8560_e3911: f64 = (assign8560_e3907 + assign8560_e3910);
        let assign8560_e3914: f64 = (p.p771 / locals.var_lwbin);
        let assign8560_e3915: f64 = (assign8560_e3911 + assign8560_e3914);
        locals.var_uc_wsti = assign8560_e3915;
        locals.var_uc_wsti_dn0 = 0.0;
        locals.var_uc_wsti_dn2 = 0.0;
        locals.var_uc_wsti_dn4 = 0.0;
        locals.var_uc_wsti_dn5 = 0.0;
        locals.var_uc_wsti_dn6 = 0.0;
        locals.var_uc_wsti_dn7 = 0.0;
        locals.var_uc_wsti_dn8 = 0.0;
        locals.var_uc_wsti_dn9 = 0.0;
        locals.var_uc_wsti_dn10 = 0.0;
        locals.var_uc_wsti_dn11 = 0.0;
        locals.var_uc_wsti_dn14 = 0.0;
        locals.var_uc_wsti_rv = 0.0;

        let assign8570_e3919: f64 = (p.p596 / locals.var_lbin);
        let assign8570_e3920: f64 = (p.p210 + assign8570_e3919);
        let assign8570_e3923: f64 = (p.p684 / locals.var_wbin);
        let assign8570_e3924: f64 = (assign8570_e3920 + assign8570_e3923);
        let assign8570_e3927: f64 = (p.p772 / locals.var_lwbin);
        let assign8570_e3928: f64 = (assign8570_e3924 + assign8570_e3927);
        locals.var_uc_scsti1 = assign8570_e3928;
        locals.var_uc_scsti1_rv = 0.0;

        let assign8580_e3932: f64 = (p.p597 / locals.var_lbin);
        let assign8580_e3933: f64 = (p.p211 + assign8580_e3932);
        let assign8580_e3936: f64 = (p.p685 / locals.var_wbin);
        let assign8580_e3937: f64 = (assign8580_e3933 + assign8580_e3936);
        let assign8580_e3940: f64 = (p.p773 / locals.var_lwbin);
        let assign8580_e3941: f64 = (assign8580_e3937 + assign8580_e3940);
        locals.var_uc_scsti2 = assign8580_e3941;
        locals.var_uc_scsti2_rv = 0.0;

        let assign8590_e3945: f64 = (p.p598 / locals.var_lbin);
        let assign8590_e3946: f64 = (p.p212 + assign8590_e3945);
        let assign8590_e3949: f64 = (p.p686 / locals.var_wbin);
        let assign8590_e3950: f64 = (assign8590_e3946 + assign8590_e3949);
        let assign8590_e3953: f64 = (p.p774 / locals.var_lwbin);
        let assign8590_e3954: f64 = (assign8590_e3950 + assign8590_e3953);
        locals.var_uc_vthsti = assign8590_e3954;
        locals.var_uc_vthsti_rv = 0.0;

        let assign8600_e3958: f64 = (p.p599 / locals.var_lbin);
        let assign8600_e3959: f64 = (p.p214 + assign8600_e3958);
        let assign8600_e3962: f64 = (p.p687 / locals.var_wbin);
        let assign8600_e3963: f64 = (assign8600_e3959 + assign8600_e3962);
        let assign8600_e3966: f64 = (p.p775 / locals.var_lwbin);
        let assign8600_e3967: f64 = (assign8600_e3963 + assign8600_e3966);
        locals.var_uc_muesti1 = assign8600_e3967;
        locals.var_uc_muesti1_rv = 0.0;

        let assign8610_e3971: f64 = (p.p600 / locals.var_lbin);
        let assign8610_e3972: f64 = (p.p215 + assign8610_e3971);
        let assign8610_e3975: f64 = (p.p688 / locals.var_wbin);
        let assign8610_e3976: f64 = (assign8610_e3972 + assign8610_e3975);
        let assign8610_e3979: f64 = (p.p776 / locals.var_lwbin);
        let assign8610_e3980: f64 = (assign8610_e3976 + assign8610_e3979);
        locals.var_uc_muesti2 = assign8610_e3980;
        locals.var_uc_muesti2_rv = 0.0;

        let assign8620_e3984: f64 = (p.p601 / locals.var_lbin);
        let assign8620_e3985: f64 = (p.p216 + assign8620_e3984);
        let assign8620_e3988: f64 = (p.p689 / locals.var_wbin);
        let assign8620_e3989: f64 = (assign8620_e3985 + assign8620_e3988);
        let assign8620_e3992: f64 = (p.p777 / locals.var_lwbin);
        let assign8620_e3993: f64 = (assign8620_e3989 + assign8620_e3992);
        locals.var_uc_muesti3 = assign8620_e3993;
        locals.var_uc_muesti3_rv = 0.0;

        let assign8630_e3997: f64 = (p.p602 / locals.var_lbin);
        let assign8630_e3998: f64 = (p.p217 + assign8630_e3997);
        let assign8630_e4001: f64 = (p.p690 / locals.var_wbin);
        let assign8630_e4002: f64 = (assign8630_e3998 + assign8630_e4001);
        let assign8630_e4005: f64 = (p.p778 / locals.var_lwbin);
        let assign8630_e4006: f64 = (assign8630_e4002 + assign8630_e4005);
        locals.var_uc_nsubpsti1 = assign8630_e4006;
        locals.var_uc_nsubpsti1_rv = 0.0;

        let assign8640_e4010: f64 = (p.p603 / locals.var_lbin);
        let assign8640_e4011: f64 = (p.p218 + assign8640_e4010);
        let assign8640_e4014: f64 = (p.p691 / locals.var_wbin);
        let assign8640_e4015: f64 = (assign8640_e4011 + assign8640_e4014);
        let assign8640_e4018: f64 = (p.p779 / locals.var_lwbin);
        let assign8640_e4019: f64 = (assign8640_e4015 + assign8640_e4018);
        locals.var_uc_nsubpsti2 = assign8640_e4019;
        locals.var_uc_nsubpsti2_rv = 0.0;

        let assign8650_e4023: f64 = (p.p604 / locals.var_lbin);
        let assign8650_e4024: f64 = (p.p219 + assign8650_e4023);
        let assign8650_e4027: f64 = (p.p692 / locals.var_wbin);
        let assign8650_e4028: f64 = (assign8650_e4024 + assign8650_e4027);
        let assign8650_e4031: f64 = (p.p780 / locals.var_lwbin);
        let assign8650_e4032: f64 = (assign8650_e4028 + assign8650_e4031);
        locals.var_uc_nsubpsti3 = assign8650_e4032;
        locals.var_uc_nsubpsti3_rv = 0.0;

        let assign8660_e4036: f64 = (p.p605 / locals.var_lbin);
        let assign8660_e4037: f64 = (p.p269 + assign8660_e4036);
        let assign8660_e4040: f64 = (p.p693 / locals.var_wbin);
        let assign8660_e4041: f64 = (assign8660_e4037 + assign8660_e4040);
        let assign8660_e4044: f64 = (p.p781 / locals.var_lwbin);
        let assign8660_e4045: f64 = (assign8660_e4041 + assign8660_e4044);
        locals.var_uc_cgso = assign8660_e4045;
        locals.var_uc_cgso_rv = 0.0;

        let assign8670_e4049: f64 = (p.p606 / locals.var_lbin);
        let assign8670_e4050: f64 = (p.p268 + assign8670_e4049);
        let assign8670_e4053: f64 = (p.p694 / locals.var_wbin);
        let assign8670_e4054: f64 = (assign8670_e4050 + assign8670_e4053);
        let assign8670_e4057: f64 = (p.p782 / locals.var_lwbin);
        let assign8670_e4058: f64 = (assign8670_e4054 + assign8670_e4057);
        locals.var_uc_cgdo = assign8670_e4058;
        locals.var_uc_cgdo_rv = 0.0;

        let assign8680_e4062: f64 = (p.p607 / locals.var_lbin);
        let assign8680_e4063: f64 = (p.p226 + assign8680_e4062);
        let assign8680_e4066: f64 = (p.p695 / locals.var_wbin);
        let assign8680_e4067: f64 = (assign8680_e4063 + assign8680_e4066);
        let assign8680_e4070: f64 = (p.p783 / locals.var_lwbin);
        let assign8680_e4071: f64 = (assign8680_e4067 + assign8680_e4070);
        locals.var_uc_clm1 = assign8680_e4071;
        locals.var_uc_clm1_rv = 0.0;

        let assign8690_e4075: f64 = (p.p608 / locals.var_lbin);
        let assign8690_e4076: f64 = (p.p227 + assign8690_e4075);
        let assign8690_e4079: f64 = (p.p696 / locals.var_wbin);
        let assign8690_e4080: f64 = (assign8690_e4076 + assign8690_e4079);
        let assign8690_e4083: f64 = (p.p784 / locals.var_lwbin);
        let assign8690_e4084: f64 = (assign8690_e4080 + assign8690_e4083);
        locals.var_uc_clm2 = assign8690_e4084;
        locals.var_uc_clm2_dn0 = 0.0;
        locals.var_uc_clm2_dn2 = 0.0;
        locals.var_uc_clm2_dn4 = 0.0;
        locals.var_uc_clm2_dn5 = 0.0;
        locals.var_uc_clm2_dn6 = 0.0;
        locals.var_uc_clm2_dn7 = 0.0;
        locals.var_uc_clm2_dn8 = 0.0;
        locals.var_uc_clm2_dn9 = 0.0;
        locals.var_uc_clm2_dn10 = 0.0;
        locals.var_uc_clm2_dn11 = 0.0;
        locals.var_uc_clm2_dn14 = 0.0;
        locals.var_uc_clm2_rv = 0.0;

        let assign8700_e4088: f64 = (p.p609 / locals.var_lbin);
        let assign8700_e4089: f64 = (p.p228 + assign8700_e4088);
        let assign8700_e4092: f64 = (p.p697 / locals.var_wbin);
        let assign8700_e4093: f64 = (assign8700_e4089 + assign8700_e4092);
        let assign8700_e4096: f64 = (p.p785 / locals.var_lwbin);
        let assign8700_e4097: f64 = (assign8700_e4093 + assign8700_e4096);
        locals.var_uc_clm3 = assign8700_e4097;
        locals.var_uc_clm3_rv = 0.0;

        let assign8710_e4101: f64 = (p.p610 / locals.var_lbin);
        let assign8710_e4102: f64 = (p.p232 + assign8710_e4101);
        let assign8710_e4105: f64 = (p.p698 / locals.var_wbin);
        let assign8710_e4106: f64 = (assign8710_e4102 + assign8710_e4105);
        let assign8710_e4109: f64 = (p.p786 / locals.var_lwbin);
        let assign8710_e4110: f64 = (assign8710_e4106 + assign8710_e4109);
        locals.var_uc_wfc = assign8710_e4110;
        locals.var_uc_wfc_rv = 0.0;

        let assign8720_e4114: f64 = (p.p611 / locals.var_lbin);
        let assign8720_e4115: f64 = (p.p240 + assign8720_e4114);
        let assign8720_e4118: f64 = (p.p699 / locals.var_wbin);
        let assign8720_e4119: f64 = (assign8720_e4115 + assign8720_e4118);
        let assign8720_e4122: f64 = (p.p787 / locals.var_lwbin);
        let assign8720_e4123: f64 = (assign8720_e4119 + assign8720_e4122);
        locals.var_uc_gidl1 = assign8720_e4123;
        locals.var_uc_gidl1_rv = 0.0;

        let assign8730_e4127: f64 = (p.p612 / locals.var_lbin);
        let assign8730_e4128: f64 = (p.p241 + assign8730_e4127);
        let assign8730_e4131: f64 = (p.p700 / locals.var_wbin);
        let assign8730_e4132: f64 = (assign8730_e4128 + assign8730_e4131);
        let assign8730_e4135: f64 = (p.p788 / locals.var_lwbin);
        let assign8730_e4136: f64 = (assign8730_e4132 + assign8730_e4135);
        locals.var_uc_gidl2 = assign8730_e4136;
        locals.var_uc_gidl2_rv = 0.0;

        let assign8740_e4140: f64 = (p.p613 / locals.var_lbin);
        let assign8740_e4141: f64 = (p.p245 + assign8740_e4140);
        let assign8740_e4144: f64 = (p.p701 / locals.var_wbin);
        let assign8740_e4145: f64 = (assign8740_e4141 + assign8740_e4144);
        let assign8740_e4148: f64 = (p.p789 / locals.var_lwbin);
        let assign8740_e4149: f64 = (assign8740_e4145 + assign8740_e4148);
        locals.var_uc_gleak1 = assign8740_e4149;
        locals.var_uc_gleak1_rv = 0.0;

        let assign8750_e4153: f64 = (p.p614 / locals.var_lbin);
        let assign8750_e4154: f64 = (p.p246 + assign8750_e4153);
        let assign8750_e4157: f64 = (p.p702 / locals.var_wbin);
        let assign8750_e4158: f64 = (assign8750_e4154 + assign8750_e4157);
        let assign8750_e4161: f64 = (p.p790 / locals.var_lwbin);
        let assign8750_e4162: f64 = (assign8750_e4158 + assign8750_e4161);
        locals.var_uc_gleak2 = assign8750_e4162;
        locals.var_uc_gleak2_rv = 0.0;

        let assign8760_e4166: f64 = (p.p615 / locals.var_lbin);
        let assign8760_e4167: f64 = (p.p247 + assign8760_e4166);
        let assign8760_e4170: f64 = (p.p703 / locals.var_wbin);
        let assign8760_e4171: f64 = (assign8760_e4167 + assign8760_e4170);
        let assign8760_e4174: f64 = (p.p791 / locals.var_lwbin);
        let assign8760_e4175: f64 = (assign8760_e4171 + assign8760_e4174);
        locals.var_uc_gleak3 = assign8760_e4175;
        locals.var_uc_gleak3_rv = 0.0;

        let assign8770_e4179: f64 = (p.p616 / locals.var_lbin);
        let assign8770_e4180: f64 = (p.p250 + assign8770_e4179);
        let assign8770_e4183: f64 = (p.p704 / locals.var_wbin);
        let assign8770_e4184: f64 = (assign8770_e4180 + assign8770_e4183);
        let assign8770_e4187: f64 = (p.p792 / locals.var_lwbin);
        let assign8770_e4188: f64 = (assign8770_e4184 + assign8770_e4187);
        locals.var_uc_gleak6 = assign8770_e4188;
        locals.var_uc_gleak6_rv = 0.0;

        let assign8780_e4192: f64 = (p.p617 / locals.var_lbin);
        let assign8780_e4193: f64 = (p.p253 + assign8780_e4192);
        let assign8780_e4196: f64 = (p.p705 / locals.var_wbin);
        let assign8780_e4197: f64 = (assign8780_e4193 + assign8780_e4196);
        let assign8780_e4200: f64 = (p.p793 / locals.var_lwbin);
        let assign8780_e4201: f64 = (assign8780_e4197 + assign8780_e4200);
        locals.var_uc_glksd1 = assign8780_e4201;
        locals.var_uc_glksd1_rv = 0.0;

        let assign8790_e4205: f64 = (p.p618 / locals.var_lbin);
        let assign8790_e4206: f64 = (p.p254 + assign8790_e4205);
        let assign8790_e4209: f64 = (p.p706 / locals.var_wbin);
        let assign8790_e4210: f64 = (assign8790_e4206 + assign8790_e4209);
        let assign8790_e4213: f64 = (p.p794 / locals.var_lwbin);
        let assign8790_e4214: f64 = (assign8790_e4210 + assign8790_e4213);
        locals.var_uc_glksd2 = assign8790_e4214;
        locals.var_uc_glksd2_rv = 0.0;

        let assign8800_e4218: f64 = (p.p619 / locals.var_lbin);
        let assign8800_e4219: f64 = (p.p256 + assign8800_e4218);
        let assign8800_e4222: f64 = (p.p707 / locals.var_wbin);
        let assign8800_e4223: f64 = (assign8800_e4219 + assign8800_e4222);
        let assign8800_e4226: f64 = (p.p795 / locals.var_lwbin);
        let assign8800_e4227: f64 = (assign8800_e4223 + assign8800_e4226);
        locals.var_uc_glkb1 = assign8800_e4227;
        locals.var_uc_glkb1_rv = 0.0;

        let assign8810_e4231: f64 = (p.p620 / locals.var_lbin);
        let assign8810_e4232: f64 = (p.p257 + assign8810_e4231);
        let assign8810_e4235: f64 = (p.p708 / locals.var_wbin);
        let assign8810_e4236: f64 = (assign8810_e4232 + assign8810_e4235);
        let assign8810_e4239: f64 = (p.p796 / locals.var_lwbin);
        let assign8810_e4240: f64 = (assign8810_e4236 + assign8810_e4239);
        locals.var_uc_glkb2 = assign8810_e4240;
        locals.var_uc_glkb2_rv = 0.0;

        let assign8830_e4257: f64 = (p.p622 / locals.var_lbin);
        let assign8830_e4258: f64 = (p.p265 + assign8830_e4257);
        let assign8830_e4261: f64 = (p.p710 / locals.var_wbin);
        let assign8830_e4262: f64 = (assign8830_e4258 + assign8830_e4261);
        let assign8830_e4265: f64 = (p.p798 / locals.var_lwbin);
        let assign8830_e4266: f64 = (assign8830_e4262 + assign8830_e4265);
        locals.var_uc_nfalp = assign8830_e4266;
        locals.var_uc_nfalp_rv = 0.0;

        let assign8840_e4270: f64 = (p.p623 / locals.var_lbin);
        let assign8840_e4271: f64 = (p.p278 + assign8840_e4270);
        let assign8840_e4274: f64 = (p.p711 / locals.var_wbin);
        let assign8840_e4275: f64 = (assign8840_e4271 + assign8840_e4274);
        let assign8840_e4278: f64 = (p.p799 / locals.var_lwbin);
        let assign8840_e4279: f64 = (assign8840_e4275 + assign8840_e4278);
        locals.var_uc_ibpc1 = assign8840_e4279;
        locals.var_uc_ibpc1_rv = 0.0;

        let assign8850_e4283: f64 = (p.p624 / locals.var_lbin);
        let assign8850_e4284: f64 = (p.p281 + assign8850_e4283);
        let assign8850_e4287: f64 = (p.p712 / locals.var_wbin);
        let assign8850_e4288: f64 = (assign8850_e4284 + assign8850_e4287);
        let assign8850_e4291: f64 = (p.p800 / locals.var_lwbin);
        let assign8850_e4292: f64 = (assign8850_e4288 + assign8850_e4291);
        locals.var_uc_ibpc2 = assign8850_e4292;
        locals.var_uc_ibpc2_rv = 0.0;

        let assign8860_e4296: f64 = (p.p625 / locals.var_lbin);
        let assign8860_e4297: f64 = (p.p79 + assign8860_e4296);
        let assign8860_e4300: f64 = (p.p713 / locals.var_wbin);
        let assign8860_e4301: f64 = (assign8860_e4297 + assign8860_e4300);
        let assign8860_e4304: f64 = (p.p801 / locals.var_lwbin);
        let assign8860_e4305: f64 = (assign8860_e4301 + assign8860_e4304);
        locals.var_uc_cgbo = assign8860_e4305;
        locals.var_uc_cgbo_rv = 0.0;

        let assign8870_e4309: f64 = (p.p626 / locals.var_lbin);
        let assign8870_e4310: f64 = (p.p86 + assign8870_e4309);
        let assign8870_e4313: f64 = (p.p714 / locals.var_wbin);
        let assign8870_e4314: f64 = (assign8870_e4310 + assign8870_e4313);
        let assign8870_e4317: f64 = (p.p802 / locals.var_lwbin);
        let assign8870_e4318: f64 = (assign8870_e4314 + assign8870_e4317);
        locals.var_uc_cvdsover = assign8870_e4318;
        locals.var_uc_cvdsover_rv = 0.0;

        let assign8890_e4335: f64 = (p.p628 / locals.var_lbin);
        let assign8890_e4336: f64 = (p.p76 + assign8890_e4335);
        let assign8890_e4339: f64 = (p.p716 / locals.var_wbin);
        let assign8890_e4340: f64 = (assign8890_e4336 + assign8890_e4339);
        let assign8890_e4343: f64 = (p.p804 / locals.var_lwbin);
        let assign8890_e4344: f64 = (assign8890_e4340 + assign8890_e4343);
        locals.var_uc_npext = assign8890_e4344;
        locals.var_uc_npext_rv = 0.0;

        let assign8900_e4348: f64 = (p.p629 / locals.var_lbin);
        let assign8900_e4349: f64 = (p.p81 + assign8900_e4348);
        let assign8900_e4352: f64 = (p.p717 / locals.var_wbin);
        let assign8900_e4353: f64 = (assign8900_e4349 + assign8900_e4352);
        let assign8900_e4356: f64 = (p.p805 / locals.var_lwbin);
        let assign8900_e4357: f64 = (assign8900_e4353 + assign8900_e4356);
        locals.var_uc_powrat = assign8900_e4357;
        locals.var_uc_powrat_rv = 0.0;

        let assign8910_e4361: f64 = (p.p630 / locals.var_lbin);
        let assign8910_e4362: f64 = (p.p74 + assign8910_e4361);
        let assign8910_e4365: f64 = (p.p718 / locals.var_wbin);
        let assign8910_e4366: f64 = (assign8910_e4362 + assign8910_e4365);
        let assign8910_e4369: f64 = (p.p806 / locals.var_lwbin);
        let assign8910_e4370: f64 = (assign8910_e4366 + assign8910_e4369);
        locals.var_uc_rd = assign8910_e4370;
        locals.var_uc_rd_rv = 0.0;

        let assign8920_e4374: f64 = (p.p631 / locals.var_lbin);
        let assign8920_e4375: f64 = (p.p298 + assign8920_e4374);
        let assign8920_e4378: f64 = (p.p719 / locals.var_wbin);
        let assign8920_e4379: f64 = (assign8920_e4375 + assign8920_e4378);
        let assign8920_e4382: f64 = (p.p807 / locals.var_lwbin);
        let assign8920_e4383: f64 = (assign8920_e4379 + assign8920_e4382);
        locals.var_uc_rd22 = assign8920_e4383;
        locals.var_uc_rd22_rv = 0.0;

        let assign8930_e4387: f64 = (p.p632 / locals.var_lbin);
        let assign8930_e4388: f64 = (p.p83 + assign8930_e4387);
        let assign8930_e4391: f64 = (p.p720 / locals.var_wbin);
        let assign8930_e4392: f64 = (assign8930_e4388 + assign8930_e4391);
        let assign8930_e4395: f64 = (p.p808 / locals.var_lwbin);
        let assign8930_e4396: f64 = (assign8930_e4392 + assign8930_e4395);
        locals.var_uc_rd23 = assign8930_e4396;
        locals.var_uc_rd23_rv = 0.0;

        let assign8940_e4400: f64 = (p.p633 / locals.var_lbin);
        let assign8940_e4401: f64 = (p.p84 + assign8940_e4400);
        let assign8940_e4404: f64 = (p.p721 / locals.var_wbin);
        let assign8940_e4405: f64 = (assign8940_e4401 + assign8940_e4404);
        let assign8940_e4408: f64 = (p.p809 / locals.var_lwbin);
        let assign8940_e4409: f64 = (assign8940_e4405 + assign8940_e4408);
        locals.var_uc_rd24 = assign8940_e4409;
        locals.var_uc_rd24_rv = 0.0;

        let assign8950_e4413: f64 = (p.p634 / locals.var_lbin);
        let assign8950_e4414: f64 = (p.p62 + assign8950_e4413);
        let assign8950_e4417: f64 = (p.p722 / locals.var_wbin);
        let assign8950_e4418: f64 = (assign8950_e4414 + assign8950_e4417);
        let assign8950_e4421: f64 = (p.p810 / locals.var_lwbin);
        let assign8950_e4422: f64 = (assign8950_e4418 + assign8950_e4421);
        locals.var_uc_rdict1 = assign8950_e4422;
        locals.var_uc_rdict1_rv = 0.0;

        let assign8960_e4426: f64 = (p.p635 / locals.var_lbin);
        let assign8960_e4427: f64 = (p.p59 + assign8960_e4426);
        let assign8960_e4430: f64 = (p.p723 / locals.var_wbin);
        let assign8960_e4431: f64 = (assign8960_e4427 + assign8960_e4430);
        let assign8960_e4434: f64 = (p.p811 / locals.var_lwbin);
        let assign8960_e4435: f64 = (assign8960_e4431 + assign8960_e4434);
        locals.var_uc_rdov13 = assign8960_e4435;
        locals.var_uc_rdov13_rv = 0.0;

        let assign8970_e4439: f64 = (p.p636 / locals.var_lbin);
        let assign8970_e4440: f64 = (p.p60 + assign8970_e4439);
        let assign8970_e4443: f64 = (p.p724 / locals.var_wbin);
        let assign8970_e4444: f64 = (assign8970_e4440 + assign8970_e4443);
        let assign8970_e4447: f64 = (p.p812 / locals.var_lwbin);
        let assign8970_e4448: f64 = (assign8970_e4444 + assign8970_e4447);
        locals.var_uc_rdslp1 = assign8970_e4448;
        locals.var_uc_rdslp1_rv = 0.0;

        let assign8980_e4452: f64 = (p.p637 / locals.var_lbin);
        let assign8980_e4453: f64 = (p.p85 + assign8980_e4452);
        let assign8980_e4456: f64 = (p.p725 / locals.var_wbin);
        let assign8980_e4457: f64 = (assign8980_e4453 + assign8980_e4456);
        let assign8980_e4460: f64 = (p.p813 / locals.var_lwbin);
        let assign8980_e4461: f64 = (assign8980_e4457 + assign8980_e4460);
        locals.var_uc_rdvb = assign8980_e4461;
        locals.var_uc_rdvb_rv = 0.0;

        let assign8990_e4465: f64 = (p.p638 / locals.var_lbin);
        let assign8990_e4466: f64 = (p.p82 + assign8990_e4465);
        let assign8990_e4469: f64 = (p.p726 / locals.var_wbin);
        let assign8990_e4470: f64 = (assign8990_e4466 + assign8990_e4469);
        let assign8990_e4473: f64 = (p.p814 / locals.var_lwbin);
        let assign8990_e4474: f64 = (assign8990_e4470 + assign8990_e4473);
        locals.var_uc_rdvd = assign8990_e4474;
        locals.var_uc_rdvd_rv = 0.0;

        let assign9000_e4478: f64 = (p.p639 / locals.var_lbin);
        let assign9000_e4479: f64 = (p.p61 + assign9000_e4478);
        let assign9000_e4482: f64 = (p.p727 / locals.var_wbin);
        let assign9000_e4483: f64 = (assign9000_e4479 + assign9000_e4482);
        let assign9000_e4486: f64 = (p.p815 / locals.var_lwbin);
        let assign9000_e4487: f64 = (assign9000_e4483 + assign9000_e4486);
        locals.var_uc_rdvg11 = assign9000_e4487;
        locals.var_uc_rdvg11_rv = 0.0;

        let assign9010_e4491: f64 = (p.p640 / locals.var_lbin);
        let assign9010_e4492: f64 = (p.p75 + assign9010_e4491);
        let assign9010_e4495: f64 = (p.p728 / locals.var_wbin);
        let assign9010_e4496: f64 = (assign9010_e4492 + assign9010_e4495);
        let assign9010_e4499: f64 = (p.p816 / locals.var_lwbin);
        let assign9010_e4500: f64 = (assign9010_e4496 + assign9010_e4499);
        locals.var_uc_rs = assign9010_e4500;
        locals.var_uc_rs_rv = 0.0;

        let assign9020_e4504: f64 = (p.p641 / locals.var_lbin);
        let assign9020_e4505: f64 = (p.p80 + assign9020_e4504);
        let assign9020_e4508: f64 = (p.p729 / locals.var_wbin);
        let assign9020_e4509: f64 = (assign9020_e4505 + assign9020_e4508);
        let assign9020_e4512: f64 = (p.p817 / locals.var_lwbin);
        let assign9020_e4513: f64 = (assign9020_e4509 + assign9020_e4512);
        locals.var_uc_rth0 = assign9020_e4513;
        locals.var_uc_rth0_rv = 0.0;

        let assign9030_e4517: f64 = (p.p642 / locals.var_lbin);
        let assign9030_e4518: f64 = (p.p77 + assign9030_e4517);
        let assign9030_e4521: f64 = (p.p730 / locals.var_wbin);
        let assign9030_e4522: f64 = (assign9030_e4518 + assign9030_e4521);
        let assign9030_e4525: f64 = (p.p818 / locals.var_lwbin);
        let assign9030_e4526: f64 = (assign9030_e4522 + assign9030_e4525);
        locals.var_uc_vover = assign9030_e4526;
        locals.var_uc_vover_rv = 0.0;

        let assign9040_e4530: f64 = (p.p824 / locals.var_lbin);
        let assign9040_e4531: f64 = (p.p493 + assign9040_e4530);
        let assign9040_e4534: f64 = (p.p839 / locals.var_wbin);
        let assign9040_e4535: f64 = (assign9040_e4531 + assign9040_e4534);
        let assign9040_e4538: f64 = (p.p854 / locals.var_lwbin);
        let assign9040_e4539: f64 = (assign9040_e4535 + assign9040_e4538);
        locals.var_uc_js0d = assign9040_e4539;
        locals.var_uc_js0d_rv = 0.0;

        let assign9050_e4543: f64 = (p.p825 / locals.var_lbin);
        let assign9050_e4544: f64 = (p.p494 + assign9050_e4543);
        let assign9050_e4547: f64 = (p.p840 / locals.var_wbin);
        let assign9050_e4548: f64 = (assign9050_e4544 + assign9050_e4547);
        let assign9050_e4551: f64 = (p.p855 / locals.var_lwbin);
        let assign9050_e4552: f64 = (assign9050_e4548 + assign9050_e4551);
        locals.var_uc_js0swd = assign9050_e4552;
        locals.var_uc_js0swd_rv = 0.0;

        let assign9060_e4556: f64 = (p.p826 / locals.var_lbin);
        let assign9060_e4557: f64 = (p.p496 + assign9060_e4556);
        let assign9060_e4560: f64 = (p.p841 / locals.var_wbin);
        let assign9060_e4561: f64 = (assign9060_e4557 + assign9060_e4560);
        let assign9060_e4564: f64 = (p.p856 / locals.var_lwbin);
        let assign9060_e4565: f64 = (assign9060_e4561 + assign9060_e4564);
        locals.var_uc_njd = assign9060_e4565;
        locals.var_uc_njd_rv = 0.0;

        let assign9080_e4582: f64 = (p.p828 / locals.var_lbin);
        let assign9080_e4583: f64 = (p.p515 + assign9080_e4582);
        let assign9080_e4586: f64 = (p.p843 / locals.var_wbin);
        let assign9080_e4587: f64 = (assign9080_e4583 + assign9080_e4586);
        let assign9080_e4590: f64 = (p.p858 / locals.var_lwbin);
        let assign9080_e4591: f64 = (assign9080_e4587 + assign9080_e4590);
        locals.var_uc_vdiffjd = assign9080_e4591;
        locals.var_uc_vdiffjd_rv = 0.0;

        let assign9090_e4595: f64 = (p.p829 / locals.var_lbin);
        let assign9090_e4596: f64 = (p.p516 + assign9090_e4595);
        let assign9090_e4599: f64 = (p.p844 / locals.var_wbin);
        let assign9090_e4600: f64 = (assign9090_e4596 + assign9090_e4599);
        let assign9090_e4603: f64 = (p.p859 / locals.var_lwbin);
        let assign9090_e4604: f64 = (assign9090_e4600 + assign9090_e4603);
        locals.var_uc_js0s = assign9090_e4604;
        locals.var_uc_js0s_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign9100_e4608: f64 = (p.p830 / locals.var_lbin);
        let assign9100_e4609: f64 = (p.p517 + assign9100_e4608);
        let assign9100_e4612: f64 = (p.p845 / locals.var_wbin);
        let assign9100_e4613: f64 = (assign9100_e4609 + assign9100_e4612);
        let assign9100_e4616: f64 = (p.p860 / locals.var_lwbin);
        let assign9100_e4617: f64 = (assign9100_e4613 + assign9100_e4616);
        locals.var_uc_js0sws = assign9100_e4617;
        locals.var_uc_js0sws_rv = 0.0;

        let assign9110_e4621: f64 = (p.p831 / locals.var_lbin);
        let assign9110_e4622: f64 = (p.p519 + assign9110_e4621);
        let assign9110_e4625: f64 = (p.p846 / locals.var_wbin);
        let assign9110_e4626: f64 = (assign9110_e4622 + assign9110_e4625);
        let assign9110_e4629: f64 = (p.p861 / locals.var_lwbin);
        let assign9110_e4630: f64 = (assign9110_e4626 + assign9110_e4629);
        locals.var_uc_njs = assign9110_e4630;
        locals.var_uc_njs_rv = 0.0;

        let assign9130_e4647: f64 = (p.p833 / locals.var_lbin);
        let assign9130_e4648: f64 = (p.p538 + assign9130_e4647);
        let assign9130_e4651: f64 = (p.p848 / locals.var_wbin);
        let assign9130_e4652: f64 = (assign9130_e4648 + assign9130_e4651);
        let assign9130_e4655: f64 = (p.p863 / locals.var_lwbin);
        let assign9130_e4656: f64 = (assign9130_e4652 + assign9130_e4655);
        locals.var_uc_vdiffjs = assign9130_e4656;
        locals.var_uc_vdiffjs_rv = 0.0;

        let assign9230_e4707: f64 = if locals.var_uc_codep != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard189 = assign9230_e4707;
        locals.var_guard189_rv = 0.0;

        let (assign9240_e4713, assign9240_e4713_d_n0, assign9240_e4713_d_n2, assign9240_e4713_d_n4, assign9240_e4713_d_n5, assign9240_e4713_d_n6, assign9240_e4713_d_n7, assign9240_e4713_d_n8, assign9240_e4713_d_n9, assign9240_e4713_d_n10, assign9240_e4713_d_n11, assign9240_e4713_d_n14,) = {
    if (locals.var_guard189 != 0.0) {
        let assign9240_e4711: f64 = (locals.var_lg).powf(p.p342);
        (assign9240_e4711, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9240_e4713;
        locals.var_t3_dn0 = assign9240_e4713_d_n0;
        locals.var_t3_dn2 = assign9240_e4713_d_n2;
        locals.var_t3_dn4 = assign9240_e4713_d_n4;
        locals.var_t3_dn5 = assign9240_e4713_d_n5;
        locals.var_t3_dn6 = assign9240_e4713_d_n6;
        locals.var_t3_dn7 = assign9240_e4713_d_n7;
        locals.var_t3_dn8 = assign9240_e4713_d_n8;
        locals.var_t3_dn9 = assign9240_e4713_d_n9;
        locals.var_t3_dn10 = assign9240_e4713_d_n10;
        locals.var_t3_dn11 = assign9240_e4713_d_n11;
        locals.var_t3_dn14 = assign9240_e4713_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign9250_e4723, assign9250_e4723_d_n0, assign9250_e4723_d_n2, assign9250_e4723_d_n4, assign9250_e4723_d_n5, assign9250_e4723_d_n6, assign9250_e4723_d_n7, assign9250_e4723_d_n8, assign9250_e4723_d_n9, assign9250_e4723_d_n10, assign9250_e4723_d_n11, assign9250_e4723_d_n14,) = {
    if (locals.var_guard189 != 0.0) {
        let assign9250_e4719: f64 = (p.p341 / locals.var_t3);
        let assign9250_e4720: f64 = (1.0 + assign9250_e4719);
        let assign9250_e4721: f64 = (locals.var_uc_ndepm * assign9250_e4720);
        (assign9250_e4721, ((locals.var_uc_ndepm_dn0 * assign9250_e4720) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn2 * assign9250_e4720) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn4 * assign9250_e4720) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn5 * assign9250_e4720) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn6 * assign9250_e4720) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn7 * assign9250_e4720) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn8 * assign9250_e4720) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn9 * assign9250_e4720) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn10 * assign9250_e4720) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn11 * assign9250_e4720) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_ndepm_dn14 * assign9250_e4720) + (locals.var_uc_ndepm * (-((p.p341 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3))))),)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn11, locals.var_uc_ndepm_dn14,)
    }
};
        locals.var_uc_ndepm = assign9250_e4723;
        locals.var_uc_ndepm_dn0 = assign9250_e4723_d_n0;
        locals.var_uc_ndepm_dn2 = assign9250_e4723_d_n2;
        locals.var_uc_ndepm_dn4 = assign9250_e4723_d_n4;
        locals.var_uc_ndepm_dn5 = assign9250_e4723_d_n5;
        locals.var_uc_ndepm_dn6 = assign9250_e4723_d_n6;
        locals.var_uc_ndepm_dn7 = assign9250_e4723_d_n7;
        locals.var_uc_ndepm_dn8 = assign9250_e4723_d_n8;
        locals.var_uc_ndepm_dn9 = assign9250_e4723_d_n9;
        locals.var_uc_ndepm_dn10 = assign9250_e4723_d_n10;
        locals.var_uc_ndepm_dn11 = assign9250_e4723_d_n11;
        locals.var_uc_ndepm_dn14 = assign9250_e4723_d_n14;
        locals.var_uc_ndepm_rv = 0.0;

        let assign9260_e4726: f64 = if locals.var_uc_ndepm < 1e21 { 1.0 } else { 0.0 };
        locals.var_guard190 = assign9260_e4726;
        locals.var_guard190_rv = 0.0;

        let (assign9270_e4732, assign9270_e4732_d_n0, assign9270_e4732_d_n2, assign9270_e4732_d_n4, assign9270_e4732_d_n5, assign9270_e4732_d_n6, assign9270_e4732_d_n7, assign9270_e4732_d_n8, assign9270_e4732_d_n9, assign9270_e4732_d_n10, assign9270_e4732_d_n11, assign9270_e4732_d_n14,) = {
    if ((locals.var_guard189 != 0.0) && (locals.var_guard190 != 0.0)) {
        (1e21, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn11, locals.var_uc_ndepm_dn14,)
    }
};
        locals.var_uc_ndepm = assign9270_e4732;
        locals.var_uc_ndepm_dn0 = assign9270_e4732_d_n0;
        locals.var_uc_ndepm_dn2 = assign9270_e4732_d_n2;
        locals.var_uc_ndepm_dn4 = assign9270_e4732_d_n4;
        locals.var_uc_ndepm_dn5 = assign9270_e4732_d_n5;
        locals.var_uc_ndepm_dn6 = assign9270_e4732_d_n6;
        locals.var_uc_ndepm_dn7 = assign9270_e4732_d_n7;
        locals.var_uc_ndepm_dn8 = assign9270_e4732_d_n8;
        locals.var_uc_ndepm_dn9 = assign9270_e4732_d_n9;
        locals.var_uc_ndepm_dn10 = assign9270_e4732_d_n10;
        locals.var_uc_ndepm_dn11 = assign9270_e4732_d_n11;
        locals.var_uc_ndepm_dn14 = assign9270_e4732_d_n14;
        locals.var_uc_ndepm_rv = 0.0;

        let (assign9280_e4738, assign9280_e4738_d_n0, assign9280_e4738_d_n2, assign9280_e4738_d_n4, assign9280_e4738_d_n5, assign9280_e4738_d_n6, assign9280_e4738_d_n7, assign9280_e4738_d_n8, assign9280_e4738_d_n9, assign9280_e4738_d_n10, assign9280_e4738_d_n11, assign9280_e4738_d_n14,) = {
    if (locals.var_guard189 != 0.0) {
        let assign9280_e4736: f64 = (locals.var_lg).powf(p.p369);
        (assign9280_e4736, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9280_e4738;
        locals.var_t3_dn0 = assign9280_e4738_d_n0;
        locals.var_t3_dn2 = assign9280_e4738_d_n2;
        locals.var_t3_dn4 = assign9280_e4738_d_n4;
        locals.var_t3_dn5 = assign9280_e4738_d_n5;
        locals.var_t3_dn6 = assign9280_e4738_d_n6;
        locals.var_t3_dn7 = assign9280_e4738_d_n7;
        locals.var_t3_dn8 = assign9280_e4738_d_n8;
        locals.var_t3_dn9 = assign9280_e4738_d_n9;
        locals.var_t3_dn10 = assign9280_e4738_d_n10;
        locals.var_t3_dn11 = assign9280_e4738_d_n11;
        locals.var_t3_dn14 = assign9280_e4738_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign9290_e4748, assign9290_e4748_d_n0, assign9290_e4748_d_n2, assign9290_e4748_d_n4, assign9290_e4748_d_n5, assign9290_e4748_d_n6, assign9290_e4748_d_n7, assign9290_e4748_d_n8, assign9290_e4748_d_n9, assign9290_e4748_d_n10, assign9290_e4748_d_n11, assign9290_e4748_d_n14,) = {
    if (locals.var_guard189 != 0.0) {
        let assign9290_e4744: f64 = (p.p368 / locals.var_t3);
        let assign9290_e4745: f64 = (1.0 + assign9290_e4744);
        let assign9290_e4746: f64 = (locals.var_uc_depvmax * assign9290_e4745);
        (assign9290_e4746, ((locals.var_uc_depvmax_dn0 * assign9290_e4745) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn2 * assign9290_e4745) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn4 * assign9290_e4745) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn5 * assign9290_e4745) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn6 * assign9290_e4745) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn7 * assign9290_e4745) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn8 * assign9290_e4745) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn9 * assign9290_e4745) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn10 * assign9290_e4745) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn11 * assign9290_e4745) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvmax_dn14 * assign9290_e4745) + (locals.var_uc_depvmax * (-((p.p368 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3))))),)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn11, locals.var_uc_depvmax_dn14,)
    }
};
        locals.var_uc_depvmax = assign9290_e4748;
        locals.var_uc_depvmax_dn0 = assign9290_e4748_d_n0;
        locals.var_uc_depvmax_dn2 = assign9290_e4748_d_n2;
        locals.var_uc_depvmax_dn4 = assign9290_e4748_d_n4;
        locals.var_uc_depvmax_dn5 = assign9290_e4748_d_n5;
        locals.var_uc_depvmax_dn6 = assign9290_e4748_d_n6;
        locals.var_uc_depvmax_dn7 = assign9290_e4748_d_n7;
        locals.var_uc_depvmax_dn8 = assign9290_e4748_d_n8;
        locals.var_uc_depvmax_dn9 = assign9290_e4748_d_n9;
        locals.var_uc_depvmax_dn10 = assign9290_e4748_d_n10;
        locals.var_uc_depvmax_dn11 = assign9290_e4748_d_n11;
        locals.var_uc_depvmax_dn14 = assign9290_e4748_d_n14;
        locals.var_uc_depvmax_rv = 0.0;

        let (assign9300_e4754, assign9300_e4754_d_n0, assign9300_e4754_d_n2, assign9300_e4754_d_n4, assign9300_e4754_d_n5, assign9300_e4754_d_n6, assign9300_e4754_d_n7, assign9300_e4754_d_n8, assign9300_e4754_d_n9, assign9300_e4754_d_n10, assign9300_e4754_d_n11, assign9300_e4754_d_n14,) = {
    if (locals.var_guard189 != 0.0) {
        let assign9300_e4752: f64 = (locals.var_lg).powf(p.p362);
        (assign9300_e4752, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9300_e4754;
        locals.var_t3_dn0 = assign9300_e4754_d_n0;
        locals.var_t3_dn2 = assign9300_e4754_d_n2;
        locals.var_t3_dn4 = assign9300_e4754_d_n4;
        locals.var_t3_dn5 = assign9300_e4754_d_n5;
        locals.var_t3_dn6 = assign9300_e4754_d_n6;
        locals.var_t3_dn7 = assign9300_e4754_d_n7;
        locals.var_t3_dn8 = assign9300_e4754_d_n8;
        locals.var_t3_dn9 = assign9300_e4754_d_n9;
        locals.var_t3_dn10 = assign9300_e4754_d_n10;
        locals.var_t3_dn11 = assign9300_e4754_d_n11;
        locals.var_t3_dn14 = assign9300_e4754_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign9310_e4764, assign9310_e4764_d_n0, assign9310_e4764_d_n2, assign9310_e4764_d_n4, assign9310_e4764_d_n5, assign9310_e4764_d_n6, assign9310_e4764_d_n7, assign9310_e4764_d_n8, assign9310_e4764_d_n9, assign9310_e4764_d_n10, assign9310_e4764_d_n11, assign9310_e4764_d_n14,) = {
    if (locals.var_guard189 != 0.0) {
        let assign9310_e4760: f64 = (p.p361 / locals.var_t3);
        let assign9310_e4761: f64 = (1.0 + assign9310_e4760);
        let assign9310_e4762: f64 = (p.p360 * assign9310_e4761);
        (assign9310_e4762, (p.p360 * (-((p.p361 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3)))), (p.p360 * (-((p.p361 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3)))),)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn11, locals.var_uc_depleak_dn14,)
    }
};
        locals.var_uc_depleak = assign9310_e4764;
        locals.var_uc_depleak_dn0 = assign9310_e4764_d_n0;
        locals.var_uc_depleak_dn2 = assign9310_e4764_d_n2;
        locals.var_uc_depleak_dn4 = assign9310_e4764_d_n4;
        locals.var_uc_depleak_dn5 = assign9310_e4764_d_n5;
        locals.var_uc_depleak_dn6 = assign9310_e4764_d_n6;
        locals.var_uc_depleak_dn7 = assign9310_e4764_d_n7;
        locals.var_uc_depleak_dn8 = assign9310_e4764_d_n8;
        locals.var_uc_depleak_dn9 = assign9310_e4764_d_n9;
        locals.var_uc_depleak_dn10 = assign9310_e4764_d_n10;
        locals.var_uc_depleak_dn11 = assign9310_e4764_d_n11;
        locals.var_uc_depleak_dn14 = assign9310_e4764_d_n14;
        locals.var_uc_depleak_rv = 0.0;

        let assign9320_e4767: f64 = if locals.var_uc_depleak < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard191 = assign9320_e4767;
        locals.var_guard191_rv = 0.0;

        let (assign9330_e4773, assign9330_e4773_d_n0, assign9330_e4773_d_n2, assign9330_e4773_d_n4, assign9330_e4773_d_n5, assign9330_e4773_d_n6, assign9330_e4773_d_n7, assign9330_e4773_d_n8, assign9330_e4773_d_n9, assign9330_e4773_d_n10, assign9330_e4773_d_n11, assign9330_e4773_d_n14,) = {
    if ((locals.var_guard189 != 0.0) && (locals.var_guard191 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn11, locals.var_uc_depleak_dn14,)
    }
};
        locals.var_uc_depleak = assign9330_e4773;
        locals.var_uc_depleak_dn0 = assign9330_e4773_d_n0;
        locals.var_uc_depleak_dn2 = assign9330_e4773_d_n2;
        locals.var_uc_depleak_dn4 = assign9330_e4773_d_n4;
        locals.var_uc_depleak_dn5 = assign9330_e4773_d_n5;
        locals.var_uc_depleak_dn6 = assign9330_e4773_d_n6;
        locals.var_uc_depleak_dn7 = assign9330_e4773_d_n7;
        locals.var_uc_depleak_dn8 = assign9330_e4773_d_n8;
        locals.var_uc_depleak_dn9 = assign9330_e4773_d_n9;
        locals.var_uc_depleak_dn10 = assign9330_e4773_d_n10;
        locals.var_uc_depleak_dn11 = assign9330_e4773_d_n11;
        locals.var_uc_depleak_dn14 = assign9330_e4773_d_n14;
        locals.var_uc_depleak_rv = 0.0;

        let (assign9340_e4779, assign9340_e4779_d_n0, assign9340_e4779_d_n2, assign9340_e4779_d_n4, assign9340_e4779_d_n5, assign9340_e4779_d_n6, assign9340_e4779_d_n7, assign9340_e4779_d_n8, assign9340_e4779_d_n9, assign9340_e4779_d_n10, assign9340_e4779_d_n11, assign9340_e4779_d_n14,) = {
    if (locals.var_guard189 != 0.0) {
        let assign9340_e4777: f64 = (locals.var_lg).powf(p.p348);
        (assign9340_e4777, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9340_e4779;
        locals.var_t3_dn0 = assign9340_e4779_d_n0;
        locals.var_t3_dn2 = assign9340_e4779_d_n2;
        locals.var_t3_dn4 = assign9340_e4779_d_n4;
        locals.var_t3_dn5 = assign9340_e4779_d_n5;
        locals.var_t3_dn6 = assign9340_e4779_d_n6;
        locals.var_t3_dn7 = assign9340_e4779_d_n7;
        locals.var_t3_dn8 = assign9340_e4779_d_n8;
        locals.var_t3_dn9 = assign9340_e4779_d_n9;
        locals.var_t3_dn10 = assign9340_e4779_d_n10;
        locals.var_t3_dn11 = assign9340_e4779_d_n11;
        locals.var_t3_dn14 = assign9340_e4779_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign9350_e4789, assign9350_e4789_d_n0, assign9350_e4789_d_n2, assign9350_e4789_d_n4, assign9350_e4789_d_n5, assign9350_e4789_d_n6, assign9350_e4789_d_n7, assign9350_e4789_d_n8, assign9350_e4789_d_n9, assign9350_e4789_d_n10, assign9350_e4789_d_n11, assign9350_e4789_d_n14,) = {
    if (locals.var_guard189 != 0.0) {
        let assign9350_e4785: f64 = (p.p347 / locals.var_t3);
        let assign9350_e4786: f64 = (1.0 + assign9350_e4785);
        let assign9350_e4787: f64 = (p.p346 * assign9350_e4786);
        (assign9350_e4787, (p.p346 * (-((p.p347 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3)))), (p.p346 * (-((p.p347 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3)))),)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    }
};
        locals.var_uc_depmue0 = assign9350_e4789;
        locals.var_uc_depmue0_dn0 = assign9350_e4789_d_n0;
        locals.var_uc_depmue0_dn2 = assign9350_e4789_d_n2;
        locals.var_uc_depmue0_dn4 = assign9350_e4789_d_n4;
        locals.var_uc_depmue0_dn5 = assign9350_e4789_d_n5;
        locals.var_uc_depmue0_dn6 = assign9350_e4789_d_n6;
        locals.var_uc_depmue0_dn7 = assign9350_e4789_d_n7;
        locals.var_uc_depmue0_dn8 = assign9350_e4789_d_n8;
        locals.var_uc_depmue0_dn9 = assign9350_e4789_d_n9;
        locals.var_uc_depmue0_dn10 = assign9350_e4789_d_n10;
        locals.var_uc_depmue0_dn11 = assign9350_e4789_d_n11;
        locals.var_uc_depmue0_dn14 = assign9350_e4789_d_n14;
        locals.var_uc_depmue0_rv = 0.0;

        let assign9360_e4792: f64 = if locals.var_uc_depmue0 < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard192 = assign9360_e4792;
        locals.var_guard192_rv = 0.0;

        let (assign9370_e4798, assign9370_e4798_d_n0, assign9370_e4798_d_n2, assign9370_e4798_d_n4, assign9370_e4798_d_n5, assign9370_e4798_d_n6, assign9370_e4798_d_n7, assign9370_e4798_d_n8, assign9370_e4798_d_n9, assign9370_e4798_d_n10, assign9370_e4798_d_n11, assign9370_e4798_d_n14,) = {
    if ((locals.var_guard189 != 0.0) && (locals.var_guard192 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    }
};
        locals.var_uc_depmue0 = assign9370_e4798;
        locals.var_uc_depmue0_dn0 = assign9370_e4798_d_n0;
        locals.var_uc_depmue0_dn2 = assign9370_e4798_d_n2;
        locals.var_uc_depmue0_dn4 = assign9370_e4798_d_n4;
        locals.var_uc_depmue0_dn5 = assign9370_e4798_d_n5;
        locals.var_uc_depmue0_dn6 = assign9370_e4798_d_n6;
        locals.var_uc_depmue0_dn7 = assign9370_e4798_d_n7;
        locals.var_uc_depmue0_dn8 = assign9370_e4798_d_n8;
        locals.var_uc_depmue0_dn9 = assign9370_e4798_d_n9;
        locals.var_uc_depmue0_dn10 = assign9370_e4798_d_n10;
        locals.var_uc_depmue0_dn11 = assign9370_e4798_d_n11;
        locals.var_uc_depmue0_dn14 = assign9370_e4798_d_n14;
        locals.var_uc_depmue0_rv = 0.0;

        let (assign9380_e4804, assign9380_e4804_d_n0, assign9380_e4804_d_n2, assign9380_e4804_d_n4, assign9380_e4804_d_n5, assign9380_e4804_d_n6, assign9380_e4804_d_n7, assign9380_e4804_d_n8, assign9380_e4804_d_n9, assign9380_e4804_d_n10, assign9380_e4804_d_n11, assign9380_e4804_d_n14,) = {
    if (locals.var_guard189 != 0.0) {
        let assign9380_e4802: f64 = (locals.var_lg).powf(p.p351);
        (assign9380_e4802, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9380_e4804;
        locals.var_t3_dn0 = assign9380_e4804_d_n0;
        locals.var_t3_dn2 = assign9380_e4804_d_n2;
        locals.var_t3_dn4 = assign9380_e4804_d_n4;
        locals.var_t3_dn5 = assign9380_e4804_d_n5;
        locals.var_t3_dn6 = assign9380_e4804_d_n6;
        locals.var_t3_dn7 = assign9380_e4804_d_n7;
        locals.var_t3_dn8 = assign9380_e4804_d_n8;
        locals.var_t3_dn9 = assign9380_e4804_d_n9;
        locals.var_t3_dn10 = assign9380_e4804_d_n10;
        locals.var_t3_dn11 = assign9380_e4804_d_n11;
        locals.var_t3_dn14 = assign9380_e4804_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign9390_e4814, assign9390_e4814_d_n0, assign9390_e4814_d_n2, assign9390_e4814_d_n4, assign9390_e4814_d_n5, assign9390_e4814_d_n6, assign9390_e4814_d_n7, assign9390_e4814_d_n8, assign9390_e4814_d_n9, assign9390_e4814_d_n10, assign9390_e4814_d_n11, assign9390_e4814_d_n14,) = {
    if (locals.var_guard189 != 0.0) {
        let assign9390_e4810: f64 = (p.p350 / locals.var_t3);
        let assign9390_e4811: f64 = (1.0 + assign9390_e4810);
        let assign9390_e4812: f64 = (p.p349 * assign9390_e4811);
        (assign9390_e4812, (p.p349 * (-((p.p350 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3)))), (p.p349 * (-((p.p350 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3)))),)
    } else {
        (locals.var_uc_depmue1, locals.var_uc_depmue1_dn0, locals.var_uc_depmue1_dn2, locals.var_uc_depmue1_dn4, locals.var_uc_depmue1_dn5, locals.var_uc_depmue1_dn6, locals.var_uc_depmue1_dn7, locals.var_uc_depmue1_dn8, locals.var_uc_depmue1_dn9, locals.var_uc_depmue1_dn10, locals.var_uc_depmue1_dn11, locals.var_uc_depmue1_dn14,)
    }
};
        locals.var_uc_depmue1 = assign9390_e4814;
        locals.var_uc_depmue1_dn0 = assign9390_e4814_d_n0;
        locals.var_uc_depmue1_dn2 = assign9390_e4814_d_n2;
        locals.var_uc_depmue1_dn4 = assign9390_e4814_d_n4;
        locals.var_uc_depmue1_dn5 = assign9390_e4814_d_n5;
        locals.var_uc_depmue1_dn6 = assign9390_e4814_d_n6;
        locals.var_uc_depmue1_dn7 = assign9390_e4814_d_n7;
        locals.var_uc_depmue1_dn8 = assign9390_e4814_d_n8;
        locals.var_uc_depmue1_dn9 = assign9390_e4814_d_n9;
        locals.var_uc_depmue1_dn10 = assign9390_e4814_d_n10;
        locals.var_uc_depmue1_dn11 = assign9390_e4814_d_n11;
        locals.var_uc_depmue1_dn14 = assign9390_e4814_d_n14;
        locals.var_uc_depmue1_rv = 0.0;

        let assign9400_e4817: f64 = if locals.var_uc_depmue1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard193 = assign9400_e4817;
        locals.var_guard193_rv = 0.0;

        let (assign9410_e4823, assign9410_e4823_d_n0, assign9410_e4823_d_n2, assign9410_e4823_d_n4, assign9410_e4823_d_n5, assign9410_e4823_d_n6, assign9410_e4823_d_n7, assign9410_e4823_d_n8, assign9410_e4823_d_n9, assign9410_e4823_d_n10, assign9410_e4823_d_n11, assign9410_e4823_d_n14,) = {
    if ((locals.var_guard189 != 0.0) && (locals.var_guard193 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue1, locals.var_uc_depmue1_dn0, locals.var_uc_depmue1_dn2, locals.var_uc_depmue1_dn4, locals.var_uc_depmue1_dn5, locals.var_uc_depmue1_dn6, locals.var_uc_depmue1_dn7, locals.var_uc_depmue1_dn8, locals.var_uc_depmue1_dn9, locals.var_uc_depmue1_dn10, locals.var_uc_depmue1_dn11, locals.var_uc_depmue1_dn14,)
    }
};
        locals.var_uc_depmue1 = assign9410_e4823;
        locals.var_uc_depmue1_dn0 = assign9410_e4823_d_n0;
        locals.var_uc_depmue1_dn2 = assign9410_e4823_d_n2;
        locals.var_uc_depmue1_dn4 = assign9410_e4823_d_n4;
        locals.var_uc_depmue1_dn5 = assign9410_e4823_d_n5;
        locals.var_uc_depmue1_dn6 = assign9410_e4823_d_n6;
        locals.var_uc_depmue1_dn7 = assign9410_e4823_d_n7;
        locals.var_uc_depmue1_dn8 = assign9410_e4823_d_n8;
        locals.var_uc_depmue1_dn9 = assign9410_e4823_d_n9;
        locals.var_uc_depmue1_dn10 = assign9410_e4823_d_n10;
        locals.var_uc_depmue1_dn11 = assign9410_e4823_d_n11;
        locals.var_uc_depmue1_dn14 = assign9410_e4823_d_n14;
        locals.var_uc_depmue1_rv = 0.0;

        let (assign9420_e4829, assign9420_e4829_d_n0, assign9420_e4829_d_n2, assign9420_e4829_d_n4, assign9420_e4829_d_n5, assign9420_e4829_d_n6, assign9420_e4829_d_n7, assign9420_e4829_d_n8, assign9420_e4829_d_n9, assign9420_e4829_d_n10, assign9420_e4829_d_n11, assign9420_e4829_d_n14,) = {
    if (locals.var_guard189 != 0.0) {
        let assign9420_e4827: f64 = (locals.var_lg).powf(p.p357);
        (assign9420_e4827, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9420_e4829;
        locals.var_t3_dn0 = assign9420_e4829_d_n0;
        locals.var_t3_dn2 = assign9420_e4829_d_n2;
        locals.var_t3_dn4 = assign9420_e4829_d_n4;
        locals.var_t3_dn5 = assign9420_e4829_d_n5;
        locals.var_t3_dn6 = assign9420_e4829_d_n6;
        locals.var_t3_dn7 = assign9420_e4829_d_n7;
        locals.var_t3_dn8 = assign9420_e4829_d_n8;
        locals.var_t3_dn9 = assign9420_e4829_d_n9;
        locals.var_t3_dn10 = assign9420_e4829_d_n10;
        locals.var_t3_dn11 = assign9420_e4829_d_n11;
        locals.var_t3_dn14 = assign9420_e4829_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign9430_e4839, assign9430_e4839_d_n0, assign9430_e4839_d_n2, assign9430_e4839_d_n4, assign9430_e4839_d_n5, assign9430_e4839_d_n6, assign9430_e4839_d_n7, assign9430_e4839_d_n8, assign9430_e4839_d_n9, assign9430_e4839_d_n10, assign9430_e4839_d_n11, assign9430_e4839_d_n14,) = {
    if (locals.var_guard189 != 0.0) {
        let assign9430_e4835: f64 = (p.p356 / locals.var_t3);
        let assign9430_e4836: f64 = (1.0 + assign9430_e4835);
        let assign9430_e4837: f64 = (p.p354 * assign9430_e4836);
        (assign9430_e4837, (p.p354 * (-((p.p356 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3)))), (p.p354 * (-((p.p356 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3)))),)
    } else {
        (locals.var_uc_depmueback0, locals.var_uc_depmueback0_dn0, locals.var_uc_depmueback0_dn2, locals.var_uc_depmueback0_dn4, locals.var_uc_depmueback0_dn5, locals.var_uc_depmueback0_dn6, locals.var_uc_depmueback0_dn7, locals.var_uc_depmueback0_dn8, locals.var_uc_depmueback0_dn9, locals.var_uc_depmueback0_dn10, locals.var_uc_depmueback0_dn11, locals.var_uc_depmueback0_dn14,)
    }
};
        locals.var_uc_depmueback0 = assign9430_e4839;
        locals.var_uc_depmueback0_dn0 = assign9430_e4839_d_n0;
        locals.var_uc_depmueback0_dn2 = assign9430_e4839_d_n2;
        locals.var_uc_depmueback0_dn4 = assign9430_e4839_d_n4;
        locals.var_uc_depmueback0_dn5 = assign9430_e4839_d_n5;
        locals.var_uc_depmueback0_dn6 = assign9430_e4839_d_n6;
        locals.var_uc_depmueback0_dn7 = assign9430_e4839_d_n7;
        locals.var_uc_depmueback0_dn8 = assign9430_e4839_d_n8;
        locals.var_uc_depmueback0_dn9 = assign9430_e4839_d_n9;
        locals.var_uc_depmueback0_dn10 = assign9430_e4839_d_n10;
        locals.var_uc_depmueback0_dn11 = assign9430_e4839_d_n11;
        locals.var_uc_depmueback0_dn14 = assign9430_e4839_d_n14;
        locals.var_uc_depmueback0_rv = 0.0;

        let assign9440_e4842: f64 = if locals.var_uc_depmueback0 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard194 = assign9440_e4842;
        locals.var_guard194_rv = 0.0;

        let (assign9450_e4848, assign9450_e4848_d_n0, assign9450_e4848_d_n2, assign9450_e4848_d_n4, assign9450_e4848_d_n5, assign9450_e4848_d_n6, assign9450_e4848_d_n7, assign9450_e4848_d_n8, assign9450_e4848_d_n9, assign9450_e4848_d_n10, assign9450_e4848_d_n11, assign9450_e4848_d_n14,) = {
    if ((locals.var_guard189 != 0.0) && (locals.var_guard194 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmueback0, locals.var_uc_depmueback0_dn0, locals.var_uc_depmueback0_dn2, locals.var_uc_depmueback0_dn4, locals.var_uc_depmueback0_dn5, locals.var_uc_depmueback0_dn6, locals.var_uc_depmueback0_dn7, locals.var_uc_depmueback0_dn8, locals.var_uc_depmueback0_dn9, locals.var_uc_depmueback0_dn10, locals.var_uc_depmueback0_dn11, locals.var_uc_depmueback0_dn14,)
    }
};
        locals.var_uc_depmueback0 = assign9450_e4848;
        locals.var_uc_depmueback0_dn0 = assign9450_e4848_d_n0;
        locals.var_uc_depmueback0_dn2 = assign9450_e4848_d_n2;
        locals.var_uc_depmueback0_dn4 = assign9450_e4848_d_n4;
        locals.var_uc_depmueback0_dn5 = assign9450_e4848_d_n5;
        locals.var_uc_depmueback0_dn6 = assign9450_e4848_d_n6;
        locals.var_uc_depmueback0_dn7 = assign9450_e4848_d_n7;
        locals.var_uc_depmueback0_dn8 = assign9450_e4848_d_n8;
        locals.var_uc_depmueback0_dn9 = assign9450_e4848_d_n9;
        locals.var_uc_depmueback0_dn10 = assign9450_e4848_d_n10;
        locals.var_uc_depmueback0_dn11 = assign9450_e4848_d_n11;
        locals.var_uc_depmueback0_dn14 = assign9450_e4848_d_n14;
        locals.var_uc_depmueback0_rv = 0.0;

        let (assign9460_e4854, assign9460_e4854_d_n0, assign9460_e4854_d_n2, assign9460_e4854_d_n4, assign9460_e4854_d_n5, assign9460_e4854_d_n6, assign9460_e4854_d_n7, assign9460_e4854_d_n8, assign9460_e4854_d_n9, assign9460_e4854_d_n10, assign9460_e4854_d_n11, assign9460_e4854_d_n14,) = {
    if (locals.var_guard189 != 0.0) {
        let assign9460_e4852: f64 = (locals.var_lg).powf(p.p359);
        (assign9460_e4852, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9460_e4854;
        locals.var_t3_dn0 = assign9460_e4854_d_n0;
        locals.var_t3_dn2 = assign9460_e4854_d_n2;
        locals.var_t3_dn4 = assign9460_e4854_d_n4;
        locals.var_t3_dn5 = assign9460_e4854_d_n5;
        locals.var_t3_dn6 = assign9460_e4854_d_n6;
        locals.var_t3_dn7 = assign9460_e4854_d_n7;
        locals.var_t3_dn8 = assign9460_e4854_d_n8;
        locals.var_t3_dn9 = assign9460_e4854_d_n9;
        locals.var_t3_dn10 = assign9460_e4854_d_n10;
        locals.var_t3_dn11 = assign9460_e4854_d_n11;
        locals.var_t3_dn14 = assign9460_e4854_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign9470_e4864, assign9470_e4864_d_n0, assign9470_e4864_d_n2, assign9470_e4864_d_n4, assign9470_e4864_d_n5, assign9470_e4864_d_n6, assign9470_e4864_d_n7, assign9470_e4864_d_n8, assign9470_e4864_d_n9, assign9470_e4864_d_n10, assign9470_e4864_d_n11, assign9470_e4864_d_n14,) = {
    if (locals.var_guard189 != 0.0) {
        let assign9470_e4860: f64 = (p.p358 / locals.var_t3);
        let assign9470_e4861: f64 = (1.0 + assign9470_e4860);
        let assign9470_e4862: f64 = (p.p355 * assign9470_e4861);
        (assign9470_e4862, (p.p355 * (-((p.p358 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3)))), (p.p355 * (-((p.p358 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3)))),)
    } else {
        (locals.var_uc_depmueback1, locals.var_uc_depmueback1_dn0, locals.var_uc_depmueback1_dn2, locals.var_uc_depmueback1_dn4, locals.var_uc_depmueback1_dn5, locals.var_uc_depmueback1_dn6, locals.var_uc_depmueback1_dn7, locals.var_uc_depmueback1_dn8, locals.var_uc_depmueback1_dn9, locals.var_uc_depmueback1_dn10, locals.var_uc_depmueback1_dn11, locals.var_uc_depmueback1_dn14,)
    }
};
        locals.var_uc_depmueback1 = assign9470_e4864;
        locals.var_uc_depmueback1_dn0 = assign9470_e4864_d_n0;
        locals.var_uc_depmueback1_dn2 = assign9470_e4864_d_n2;
        locals.var_uc_depmueback1_dn4 = assign9470_e4864_d_n4;
        locals.var_uc_depmueback1_dn5 = assign9470_e4864_d_n5;
        locals.var_uc_depmueback1_dn6 = assign9470_e4864_d_n6;
        locals.var_uc_depmueback1_dn7 = assign9470_e4864_d_n7;
        locals.var_uc_depmueback1_dn8 = assign9470_e4864_d_n8;
        locals.var_uc_depmueback1_dn9 = assign9470_e4864_d_n9;
        locals.var_uc_depmueback1_dn10 = assign9470_e4864_d_n10;
        locals.var_uc_depmueback1_dn11 = assign9470_e4864_d_n11;
        locals.var_uc_depmueback1_dn14 = assign9470_e4864_d_n14;
        locals.var_uc_depmueback1_rv = 0.0;

        let assign9480_e4867: f64 = if locals.var_uc_depmueback1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard195 = assign9480_e4867;
        locals.var_guard195_rv = 0.0;

        let (assign9490_e4873, assign9490_e4873_d_n0, assign9490_e4873_d_n2, assign9490_e4873_d_n4, assign9490_e4873_d_n5, assign9490_e4873_d_n6, assign9490_e4873_d_n7, assign9490_e4873_d_n8, assign9490_e4873_d_n9, assign9490_e4873_d_n10, assign9490_e4873_d_n11, assign9490_e4873_d_n14,) = {
    if ((locals.var_guard189 != 0.0) && (locals.var_guard195 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmueback1, locals.var_uc_depmueback1_dn0, locals.var_uc_depmueback1_dn2, locals.var_uc_depmueback1_dn4, locals.var_uc_depmueback1_dn5, locals.var_uc_depmueback1_dn6, locals.var_uc_depmueback1_dn7, locals.var_uc_depmueback1_dn8, locals.var_uc_depmueback1_dn9, locals.var_uc_depmueback1_dn10, locals.var_uc_depmueback1_dn11, locals.var_uc_depmueback1_dn14,)
    }
};
        locals.var_uc_depmueback1 = assign9490_e4873;
        locals.var_uc_depmueback1_dn0 = assign9490_e4873_d_n0;
        locals.var_uc_depmueback1_dn2 = assign9490_e4873_d_n2;
        locals.var_uc_depmueback1_dn4 = assign9490_e4873_d_n4;
        locals.var_uc_depmueback1_dn5 = assign9490_e4873_d_n5;
        locals.var_uc_depmueback1_dn6 = assign9490_e4873_d_n6;
        locals.var_uc_depmueback1_dn7 = assign9490_e4873_d_n7;
        locals.var_uc_depmueback1_dn8 = assign9490_e4873_d_n8;
        locals.var_uc_depmueback1_dn9 = assign9490_e4873_d_n9;
        locals.var_uc_depmueback1_dn10 = assign9490_e4873_d_n10;
        locals.var_uc_depmueback1_dn11 = assign9490_e4873_d_n11;
        locals.var_uc_depmueback1_dn14 = assign9490_e4873_d_n14;
        locals.var_uc_depmueback1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_16(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign9500_e4879, assign9500_e4879_d_n0, assign9500_e4879_d_n2, assign9500_e4879_d_n4, assign9500_e4879_d_n5, assign9500_e4879_d_n6, assign9500_e4879_d_n7, assign9500_e4879_d_n8, assign9500_e4879_d_n9, assign9500_e4879_d_n10, assign9500_e4879_d_n11, assign9500_e4879_d_n14,) = {
    if (locals.var_guard189 != 0.0) {
        let assign9500_e4877: f64 = (locals.var_lg).powf(p.p373);
        (assign9500_e4877, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9500_e4879;
        locals.var_t3_dn0 = assign9500_e4879_d_n0;
        locals.var_t3_dn2 = assign9500_e4879_d_n2;
        locals.var_t3_dn4 = assign9500_e4879_d_n4;
        locals.var_t3_dn5 = assign9500_e4879_d_n5;
        locals.var_t3_dn6 = assign9500_e4879_d_n6;
        locals.var_t3_dn7 = assign9500_e4879_d_n7;
        locals.var_t3_dn8 = assign9500_e4879_d_n8;
        locals.var_t3_dn9 = assign9500_e4879_d_n9;
        locals.var_t3_dn10 = assign9500_e4879_d_n10;
        locals.var_t3_dn11 = assign9500_e4879_d_n11;
        locals.var_t3_dn14 = assign9500_e4879_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign9510_e4889, assign9510_e4889_d_n0, assign9510_e4889_d_n2, assign9510_e4889_d_n4, assign9510_e4889_d_n5, assign9510_e4889_d_n6, assign9510_e4889_d_n7, assign9510_e4889_d_n8, assign9510_e4889_d_n9, assign9510_e4889_d_n10, assign9510_e4889_d_n11, assign9510_e4889_d_n14,) = {
    if (locals.var_guard189 != 0.0) {
        let assign9510_e4885: f64 = (p.p372 / locals.var_t3);
        let assign9510_e4886: f64 = (1.0 + assign9510_e4885);
        let assign9510_e4887: f64 = (locals.var_uc_depvdsef1 * assign9510_e4886);
        (assign9510_e4887, ((locals.var_uc_depvdsef1_dn0 * assign9510_e4886) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn2 * assign9510_e4886) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn4 * assign9510_e4886) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn5 * assign9510_e4886) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn6 * assign9510_e4886) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn7 * assign9510_e4886) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn8 * assign9510_e4886) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn9 * assign9510_e4886) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn10 * assign9510_e4886) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn11 * assign9510_e4886) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef1_dn14 * assign9510_e4886) + (locals.var_uc_depvdsef1 * (-((p.p372 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3))))),)
    } else {
        (locals.var_uc_depvdsef1, locals.var_uc_depvdsef1_dn0, locals.var_uc_depvdsef1_dn2, locals.var_uc_depvdsef1_dn4, locals.var_uc_depvdsef1_dn5, locals.var_uc_depvdsef1_dn6, locals.var_uc_depvdsef1_dn7, locals.var_uc_depvdsef1_dn8, locals.var_uc_depvdsef1_dn9, locals.var_uc_depvdsef1_dn10, locals.var_uc_depvdsef1_dn11, locals.var_uc_depvdsef1_dn14,)
    }
};
        locals.var_uc_depvdsef1 = assign9510_e4889;
        locals.var_uc_depvdsef1_dn0 = assign9510_e4889_d_n0;
        locals.var_uc_depvdsef1_dn2 = assign9510_e4889_d_n2;
        locals.var_uc_depvdsef1_dn4 = assign9510_e4889_d_n4;
        locals.var_uc_depvdsef1_dn5 = assign9510_e4889_d_n5;
        locals.var_uc_depvdsef1_dn6 = assign9510_e4889_d_n6;
        locals.var_uc_depvdsef1_dn7 = assign9510_e4889_d_n7;
        locals.var_uc_depvdsef1_dn8 = assign9510_e4889_d_n8;
        locals.var_uc_depvdsef1_dn9 = assign9510_e4889_d_n9;
        locals.var_uc_depvdsef1_dn10 = assign9510_e4889_d_n10;
        locals.var_uc_depvdsef1_dn11 = assign9510_e4889_d_n11;
        locals.var_uc_depvdsef1_dn14 = assign9510_e4889_d_n14;
        locals.var_uc_depvdsef1_rv = 0.0;

        let (assign9520_e4895, assign9520_e4895_d_n0, assign9520_e4895_d_n2, assign9520_e4895_d_n4, assign9520_e4895_d_n5, assign9520_e4895_d_n6, assign9520_e4895_d_n7, assign9520_e4895_d_n8, assign9520_e4895_d_n9, assign9520_e4895_d_n10, assign9520_e4895_d_n11, assign9520_e4895_d_n14,) = {
    if (locals.var_guard189 != 0.0) {
        let assign9520_e4893: f64 = (locals.var_lg).powf(p.p375);
        (assign9520_e4893, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign9520_e4895;
        locals.var_t3_dn0 = assign9520_e4895_d_n0;
        locals.var_t3_dn2 = assign9520_e4895_d_n2;
        locals.var_t3_dn4 = assign9520_e4895_d_n4;
        locals.var_t3_dn5 = assign9520_e4895_d_n5;
        locals.var_t3_dn6 = assign9520_e4895_d_n6;
        locals.var_t3_dn7 = assign9520_e4895_d_n7;
        locals.var_t3_dn8 = assign9520_e4895_d_n8;
        locals.var_t3_dn9 = assign9520_e4895_d_n9;
        locals.var_t3_dn10 = assign9520_e4895_d_n10;
        locals.var_t3_dn11 = assign9520_e4895_d_n11;
        locals.var_t3_dn14 = assign9520_e4895_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign9530_e4905, assign9530_e4905_d_n0, assign9530_e4905_d_n2, assign9530_e4905_d_n4, assign9530_e4905_d_n5, assign9530_e4905_d_n6, assign9530_e4905_d_n7, assign9530_e4905_d_n8, assign9530_e4905_d_n9, assign9530_e4905_d_n10, assign9530_e4905_d_n11, assign9530_e4905_d_n14,) = {
    if (locals.var_guard189 != 0.0) {
        let assign9530_e4901: f64 = (p.p374 / locals.var_t3);
        let assign9530_e4902: f64 = (1.0 + assign9530_e4901);
        let assign9530_e4903: f64 = (locals.var_uc_depvdsef2 * assign9530_e4902);
        (assign9530_e4903, ((locals.var_uc_depvdsef2_dn0 * assign9530_e4902) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn2 * assign9530_e4902) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn4 * assign9530_e4902) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn5 * assign9530_e4902) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn6 * assign9530_e4902) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn7 * assign9530_e4902) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn8 * assign9530_e4902) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn9 * assign9530_e4902) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn10 * assign9530_e4902) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn11 * assign9530_e4902) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))))), ((locals.var_uc_depvdsef2_dn14 * assign9530_e4902) + (locals.var_uc_depvdsef2 * (-((p.p374 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3))))),)
    } else {
        (locals.var_uc_depvdsef2, locals.var_uc_depvdsef2_dn0, locals.var_uc_depvdsef2_dn2, locals.var_uc_depvdsef2_dn4, locals.var_uc_depvdsef2_dn5, locals.var_uc_depvdsef2_dn6, locals.var_uc_depvdsef2_dn7, locals.var_uc_depvdsef2_dn8, locals.var_uc_depvdsef2_dn9, locals.var_uc_depvdsef2_dn10, locals.var_uc_depvdsef2_dn11, locals.var_uc_depvdsef2_dn14,)
    }
};
        locals.var_uc_depvdsef2 = assign9530_e4905;
        locals.var_uc_depvdsef2_dn0 = assign9530_e4905_d_n0;
        locals.var_uc_depvdsef2_dn2 = assign9530_e4905_d_n2;
        locals.var_uc_depvdsef2_dn4 = assign9530_e4905_d_n4;
        locals.var_uc_depvdsef2_dn5 = assign9530_e4905_d_n5;
        locals.var_uc_depvdsef2_dn6 = assign9530_e4905_d_n6;
        locals.var_uc_depvdsef2_dn7 = assign9530_e4905_d_n7;
        locals.var_uc_depvdsef2_dn8 = assign9530_e4905_d_n8;
        locals.var_uc_depvdsef2_dn9 = assign9530_e4905_d_n9;
        locals.var_uc_depvdsef2_dn10 = assign9530_e4905_d_n10;
        locals.var_uc_depvdsef2_dn11 = assign9530_e4905_d_n11;
        locals.var_uc_depvdsef2_dn14 = assign9530_e4905_d_n14;
        locals.var_uc_depvdsef2_rv = 0.0;

        let assign9540_e4908: f64 = if locals.var_uc_depvdsef2 < 0.1 { 1.0 } else { 0.0 };
        locals.var_guard196 = assign9540_e4908;
        locals.var_guard196_rv = 0.0;

        let (assign9550_e4914, assign9550_e4914_d_n0, assign9550_e4914_d_n2, assign9550_e4914_d_n4, assign9550_e4914_d_n5, assign9550_e4914_d_n6, assign9550_e4914_d_n7, assign9550_e4914_d_n8, assign9550_e4914_d_n9, assign9550_e4914_d_n10, assign9550_e4914_d_n11, assign9550_e4914_d_n14,) = {
    if ((locals.var_guard189 != 0.0) && (locals.var_guard196 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvdsef2, locals.var_uc_depvdsef2_dn0, locals.var_uc_depvdsef2_dn2, locals.var_uc_depvdsef2_dn4, locals.var_uc_depvdsef2_dn5, locals.var_uc_depvdsef2_dn6, locals.var_uc_depvdsef2_dn7, locals.var_uc_depvdsef2_dn8, locals.var_uc_depvdsef2_dn9, locals.var_uc_depvdsef2_dn10, locals.var_uc_depvdsef2_dn11, locals.var_uc_depvdsef2_dn14,)
    }
};
        locals.var_uc_depvdsef2 = assign9550_e4914;
        locals.var_uc_depvdsef2_dn0 = assign9550_e4914_d_n0;
        locals.var_uc_depvdsef2_dn2 = assign9550_e4914_d_n2;
        locals.var_uc_depvdsef2_dn4 = assign9550_e4914_d_n4;
        locals.var_uc_depvdsef2_dn5 = assign9550_e4914_d_n5;
        locals.var_uc_depvdsef2_dn6 = assign9550_e4914_d_n6;
        locals.var_uc_depvdsef2_dn7 = assign9550_e4914_d_n7;
        locals.var_uc_depvdsef2_dn8 = assign9550_e4914_d_n8;
        locals.var_uc_depvdsef2_dn9 = assign9550_e4914_d_n9;
        locals.var_uc_depvdsef2_dn10 = assign9550_e4914_d_n10;
        locals.var_uc_depvdsef2_dn11 = assign9550_e4914_d_n11;
        locals.var_uc_depvdsef2_dn14 = assign9550_e4914_d_n14;
        locals.var_uc_depvdsef2_rv = 0.0;

        let (assign9560_e4919, assign9560_e4919_d_n0, assign9560_e4919_d_n2, assign9560_e4919_d_n4, assign9560_e4919_d_n5, assign9560_e4919_d_n6, assign9560_e4919_d_n7, assign9560_e4919_d_n8, assign9560_e4919_d_n9, assign9560_e4919_d_n10, assign9560_e4919_d_n11, assign9560_e4919_d_n14,) = {
    if (locals.var_guard189 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_ndepm, locals.var_uc_ndepm_dn0, locals.var_uc_ndepm_dn2, locals.var_uc_ndepm_dn4, locals.var_uc_ndepm_dn5, locals.var_uc_ndepm_dn6, locals.var_uc_ndepm_dn7, locals.var_uc_ndepm_dn8, locals.var_uc_ndepm_dn9, locals.var_uc_ndepm_dn10, locals.var_uc_ndepm_dn11, locals.var_uc_ndepm_dn14,)
    }
};
        locals.var_uc_ndepm = assign9560_e4919;
        locals.var_uc_ndepm_dn0 = assign9560_e4919_d_n0;
        locals.var_uc_ndepm_dn2 = assign9560_e4919_d_n2;
        locals.var_uc_ndepm_dn4 = assign9560_e4919_d_n4;
        locals.var_uc_ndepm_dn5 = assign9560_e4919_d_n5;
        locals.var_uc_ndepm_dn6 = assign9560_e4919_d_n6;
        locals.var_uc_ndepm_dn7 = assign9560_e4919_d_n7;
        locals.var_uc_ndepm_dn8 = assign9560_e4919_d_n8;
        locals.var_uc_ndepm_dn9 = assign9560_e4919_d_n9;
        locals.var_uc_ndepm_dn10 = assign9560_e4919_d_n10;
        locals.var_uc_ndepm_dn11 = assign9560_e4919_d_n11;
        locals.var_uc_ndepm_dn14 = assign9560_e4919_d_n14;
        locals.var_uc_ndepm_rv = 0.0;

        let (assign9570_e4924, assign9570_e4924_d_n0, assign9570_e4924_d_n2, assign9570_e4924_d_n4, assign9570_e4924_d_n5, assign9570_e4924_d_n6, assign9570_e4924_d_n7, assign9570_e4924_d_n8, assign9570_e4924_d_n9, assign9570_e4924_d_n10, assign9570_e4924_d_n11, assign9570_e4924_d_n14,) = {
    if (locals.var_guard189 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn11, locals.var_uc_depvmax_dn14,)
    }
};
        locals.var_uc_depvmax = assign9570_e4924;
        locals.var_uc_depvmax_dn0 = assign9570_e4924_d_n0;
        locals.var_uc_depvmax_dn2 = assign9570_e4924_d_n2;
        locals.var_uc_depvmax_dn4 = assign9570_e4924_d_n4;
        locals.var_uc_depvmax_dn5 = assign9570_e4924_d_n5;
        locals.var_uc_depvmax_dn6 = assign9570_e4924_d_n6;
        locals.var_uc_depvmax_dn7 = assign9570_e4924_d_n7;
        locals.var_uc_depvmax_dn8 = assign9570_e4924_d_n8;
        locals.var_uc_depvmax_dn9 = assign9570_e4924_d_n9;
        locals.var_uc_depvmax_dn10 = assign9570_e4924_d_n10;
        locals.var_uc_depvmax_dn11 = assign9570_e4924_d_n11;
        locals.var_uc_depvmax_dn14 = assign9570_e4924_d_n14;
        locals.var_uc_depvmax_rv = 0.0;

        let (assign9580_e4929, assign9580_e4929_d_n0, assign9580_e4929_d_n2, assign9580_e4929_d_n4, assign9580_e4929_d_n5, assign9580_e4929_d_n6, assign9580_e4929_d_n7, assign9580_e4929_d_n8, assign9580_e4929_d_n9, assign9580_e4929_d_n10, assign9580_e4929_d_n11, assign9580_e4929_d_n14,) = {
    if (locals.var_guard189 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depleak, locals.var_uc_depleak_dn0, locals.var_uc_depleak_dn2, locals.var_uc_depleak_dn4, locals.var_uc_depleak_dn5, locals.var_uc_depleak_dn6, locals.var_uc_depleak_dn7, locals.var_uc_depleak_dn8, locals.var_uc_depleak_dn9, locals.var_uc_depleak_dn10, locals.var_uc_depleak_dn11, locals.var_uc_depleak_dn14,)
    }
};
        locals.var_uc_depleak = assign9580_e4929;
        locals.var_uc_depleak_dn0 = assign9580_e4929_d_n0;
        locals.var_uc_depleak_dn2 = assign9580_e4929_d_n2;
        locals.var_uc_depleak_dn4 = assign9580_e4929_d_n4;
        locals.var_uc_depleak_dn5 = assign9580_e4929_d_n5;
        locals.var_uc_depleak_dn6 = assign9580_e4929_d_n6;
        locals.var_uc_depleak_dn7 = assign9580_e4929_d_n7;
        locals.var_uc_depleak_dn8 = assign9580_e4929_d_n8;
        locals.var_uc_depleak_dn9 = assign9580_e4929_d_n9;
        locals.var_uc_depleak_dn10 = assign9580_e4929_d_n10;
        locals.var_uc_depleak_dn11 = assign9580_e4929_d_n11;
        locals.var_uc_depleak_dn14 = assign9580_e4929_d_n14;
        locals.var_uc_depleak_rv = 0.0;

        let (assign9590_e4934, assign9590_e4934_d_n0, assign9590_e4934_d_n2, assign9590_e4934_d_n4, assign9590_e4934_d_n5, assign9590_e4934_d_n6, assign9590_e4934_d_n7, assign9590_e4934_d_n8, assign9590_e4934_d_n9, assign9590_e4934_d_n10, assign9590_e4934_d_n11, assign9590_e4934_d_n14,) = {
    if (locals.var_guard189 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    }
};
        locals.var_uc_depmue0 = assign9590_e4934;
        locals.var_uc_depmue0_dn0 = assign9590_e4934_d_n0;
        locals.var_uc_depmue0_dn2 = assign9590_e4934_d_n2;
        locals.var_uc_depmue0_dn4 = assign9590_e4934_d_n4;
        locals.var_uc_depmue0_dn5 = assign9590_e4934_d_n5;
        locals.var_uc_depmue0_dn6 = assign9590_e4934_d_n6;
        locals.var_uc_depmue0_dn7 = assign9590_e4934_d_n7;
        locals.var_uc_depmue0_dn8 = assign9590_e4934_d_n8;
        locals.var_uc_depmue0_dn9 = assign9590_e4934_d_n9;
        locals.var_uc_depmue0_dn10 = assign9590_e4934_d_n10;
        locals.var_uc_depmue0_dn11 = assign9590_e4934_d_n11;
        locals.var_uc_depmue0_dn14 = assign9590_e4934_d_n14;
        locals.var_uc_depmue0_rv = 0.0;

        let (assign9600_e4939, assign9600_e4939_d_n0, assign9600_e4939_d_n2, assign9600_e4939_d_n4, assign9600_e4939_d_n5, assign9600_e4939_d_n6, assign9600_e4939_d_n7, assign9600_e4939_d_n8, assign9600_e4939_d_n9, assign9600_e4939_d_n10, assign9600_e4939_d_n11, assign9600_e4939_d_n14,) = {
    if (locals.var_guard189 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmue1, locals.var_uc_depmue1_dn0, locals.var_uc_depmue1_dn2, locals.var_uc_depmue1_dn4, locals.var_uc_depmue1_dn5, locals.var_uc_depmue1_dn6, locals.var_uc_depmue1_dn7, locals.var_uc_depmue1_dn8, locals.var_uc_depmue1_dn9, locals.var_uc_depmue1_dn10, locals.var_uc_depmue1_dn11, locals.var_uc_depmue1_dn14,)
    }
};
        locals.var_uc_depmue1 = assign9600_e4939;
        locals.var_uc_depmue1_dn0 = assign9600_e4939_d_n0;
        locals.var_uc_depmue1_dn2 = assign9600_e4939_d_n2;
        locals.var_uc_depmue1_dn4 = assign9600_e4939_d_n4;
        locals.var_uc_depmue1_dn5 = assign9600_e4939_d_n5;
        locals.var_uc_depmue1_dn6 = assign9600_e4939_d_n6;
        locals.var_uc_depmue1_dn7 = assign9600_e4939_d_n7;
        locals.var_uc_depmue1_dn8 = assign9600_e4939_d_n8;
        locals.var_uc_depmue1_dn9 = assign9600_e4939_d_n9;
        locals.var_uc_depmue1_dn10 = assign9600_e4939_d_n10;
        locals.var_uc_depmue1_dn11 = assign9600_e4939_d_n11;
        locals.var_uc_depmue1_dn14 = assign9600_e4939_d_n14;
        locals.var_uc_depmue1_rv = 0.0;

        let (assign9610_e4944, assign9610_e4944_d_n0, assign9610_e4944_d_n2, assign9610_e4944_d_n4, assign9610_e4944_d_n5, assign9610_e4944_d_n6, assign9610_e4944_d_n7, assign9610_e4944_d_n8, assign9610_e4944_d_n9, assign9610_e4944_d_n10, assign9610_e4944_d_n11, assign9610_e4944_d_n14,) = {
    if (locals.var_guard189 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmueback0, locals.var_uc_depmueback0_dn0, locals.var_uc_depmueback0_dn2, locals.var_uc_depmueback0_dn4, locals.var_uc_depmueback0_dn5, locals.var_uc_depmueback0_dn6, locals.var_uc_depmueback0_dn7, locals.var_uc_depmueback0_dn8, locals.var_uc_depmueback0_dn9, locals.var_uc_depmueback0_dn10, locals.var_uc_depmueback0_dn11, locals.var_uc_depmueback0_dn14,)
    }
};
        locals.var_uc_depmueback0 = assign9610_e4944;
        locals.var_uc_depmueback0_dn0 = assign9610_e4944_d_n0;
        locals.var_uc_depmueback0_dn2 = assign9610_e4944_d_n2;
        locals.var_uc_depmueback0_dn4 = assign9610_e4944_d_n4;
        locals.var_uc_depmueback0_dn5 = assign9610_e4944_d_n5;
        locals.var_uc_depmueback0_dn6 = assign9610_e4944_d_n6;
        locals.var_uc_depmueback0_dn7 = assign9610_e4944_d_n7;
        locals.var_uc_depmueback0_dn8 = assign9610_e4944_d_n8;
        locals.var_uc_depmueback0_dn9 = assign9610_e4944_d_n9;
        locals.var_uc_depmueback0_dn10 = assign9610_e4944_d_n10;
        locals.var_uc_depmueback0_dn11 = assign9610_e4944_d_n11;
        locals.var_uc_depmueback0_dn14 = assign9610_e4944_d_n14;
        locals.var_uc_depmueback0_rv = 0.0;

        let (assign9620_e4949, assign9620_e4949_d_n0, assign9620_e4949_d_n2, assign9620_e4949_d_n4, assign9620_e4949_d_n5, assign9620_e4949_d_n6, assign9620_e4949_d_n7, assign9620_e4949_d_n8, assign9620_e4949_d_n9, assign9620_e4949_d_n10, assign9620_e4949_d_n11, assign9620_e4949_d_n14,) = {
    if (locals.var_guard189 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depmueback1, locals.var_uc_depmueback1_dn0, locals.var_uc_depmueback1_dn2, locals.var_uc_depmueback1_dn4, locals.var_uc_depmueback1_dn5, locals.var_uc_depmueback1_dn6, locals.var_uc_depmueback1_dn7, locals.var_uc_depmueback1_dn8, locals.var_uc_depmueback1_dn9, locals.var_uc_depmueback1_dn10, locals.var_uc_depmueback1_dn11, locals.var_uc_depmueback1_dn14,)
    }
};
        locals.var_uc_depmueback1 = assign9620_e4949;
        locals.var_uc_depmueback1_dn0 = assign9620_e4949_d_n0;
        locals.var_uc_depmueback1_dn2 = assign9620_e4949_d_n2;
        locals.var_uc_depmueback1_dn4 = assign9620_e4949_d_n4;
        locals.var_uc_depmueback1_dn5 = assign9620_e4949_d_n5;
        locals.var_uc_depmueback1_dn6 = assign9620_e4949_d_n6;
        locals.var_uc_depmueback1_dn7 = assign9620_e4949_d_n7;
        locals.var_uc_depmueback1_dn8 = assign9620_e4949_d_n8;
        locals.var_uc_depmueback1_dn9 = assign9620_e4949_d_n9;
        locals.var_uc_depmueback1_dn10 = assign9620_e4949_d_n10;
        locals.var_uc_depmueback1_dn11 = assign9620_e4949_d_n11;
        locals.var_uc_depmueback1_dn14 = assign9620_e4949_d_n14;
        locals.var_uc_depmueback1_rv = 0.0;

        let (assign9630_e4954, assign9630_e4954_d_n0, assign9630_e4954_d_n2, assign9630_e4954_d_n4, assign9630_e4954_d_n5, assign9630_e4954_d_n6, assign9630_e4954_d_n7, assign9630_e4954_d_n8, assign9630_e4954_d_n9, assign9630_e4954_d_n10, assign9630_e4954_d_n11, assign9630_e4954_d_n14,) = {
    if (locals.var_guard189 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvdsef1, locals.var_uc_depvdsef1_dn0, locals.var_uc_depvdsef1_dn2, locals.var_uc_depvdsef1_dn4, locals.var_uc_depvdsef1_dn5, locals.var_uc_depvdsef1_dn6, locals.var_uc_depvdsef1_dn7, locals.var_uc_depvdsef1_dn8, locals.var_uc_depvdsef1_dn9, locals.var_uc_depvdsef1_dn10, locals.var_uc_depvdsef1_dn11, locals.var_uc_depvdsef1_dn14,)
    }
};
        locals.var_uc_depvdsef1 = assign9630_e4954;
        locals.var_uc_depvdsef1_dn0 = assign9630_e4954_d_n0;
        locals.var_uc_depvdsef1_dn2 = assign9630_e4954_d_n2;
        locals.var_uc_depvdsef1_dn4 = assign9630_e4954_d_n4;
        locals.var_uc_depvdsef1_dn5 = assign9630_e4954_d_n5;
        locals.var_uc_depvdsef1_dn6 = assign9630_e4954_d_n6;
        locals.var_uc_depvdsef1_dn7 = assign9630_e4954_d_n7;
        locals.var_uc_depvdsef1_dn8 = assign9630_e4954_d_n8;
        locals.var_uc_depvdsef1_dn9 = assign9630_e4954_d_n9;
        locals.var_uc_depvdsef1_dn10 = assign9630_e4954_d_n10;
        locals.var_uc_depvdsef1_dn11 = assign9630_e4954_d_n11;
        locals.var_uc_depvdsef1_dn14 = assign9630_e4954_d_n14;
        locals.var_uc_depvdsef1_rv = 0.0;

        let (assign9640_e4959, assign9640_e4959_d_n0, assign9640_e4959_d_n2, assign9640_e4959_d_n4, assign9640_e4959_d_n5, assign9640_e4959_d_n6, assign9640_e4959_d_n7, assign9640_e4959_d_n8, assign9640_e4959_d_n9, assign9640_e4959_d_n10, assign9640_e4959_d_n11, assign9640_e4959_d_n14,) = {
    if (locals.var_guard189 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvdsef2, locals.var_uc_depvdsef2_dn0, locals.var_uc_depvdsef2_dn2, locals.var_uc_depvdsef2_dn4, locals.var_uc_depvdsef2_dn5, locals.var_uc_depvdsef2_dn6, locals.var_uc_depvdsef2_dn7, locals.var_uc_depvdsef2_dn8, locals.var_uc_depvdsef2_dn9, locals.var_uc_depvdsef2_dn10, locals.var_uc_depvdsef2_dn11, locals.var_uc_depvdsef2_dn14,)
    }
};
        locals.var_uc_depvdsef2 = assign9640_e4959;
        locals.var_uc_depvdsef2_dn0 = assign9640_e4959_d_n0;
        locals.var_uc_depvdsef2_dn2 = assign9640_e4959_d_n2;
        locals.var_uc_depvdsef2_dn4 = assign9640_e4959_d_n4;
        locals.var_uc_depvdsef2_dn5 = assign9640_e4959_d_n5;
        locals.var_uc_depvdsef2_dn6 = assign9640_e4959_d_n6;
        locals.var_uc_depvdsef2_dn7 = assign9640_e4959_d_n7;
        locals.var_uc_depvdsef2_dn8 = assign9640_e4959_d_n8;
        locals.var_uc_depvdsef2_dn9 = assign9640_e4959_d_n9;
        locals.var_uc_depvdsef2_dn10 = assign9640_e4959_d_n10;
        locals.var_uc_depvdsef2_dn11 = assign9640_e4959_d_n11;
        locals.var_uc_depvdsef2_dn14 = assign9640_e4959_d_n14;
        locals.var_uc_depvdsef2_rv = 0.0;

        let assign10160_e5332: f64 = (locals.var_uc_xpdv * locals.var_uc_xldld);
        let assign10160_e5334: f64 = if assign10160_e5332 > 1.0 { 1.0 } else { 0.0 };
        locals.var_guard248 = assign10160_e5334;
        locals.var_guard248_rv = 0.0;

        let (assign10170_e5340,) = {
    if (locals.var_guard248 != 0.0) {
        let assign10170_e5338: f64 = (1.0 / locals.var_uc_xldld);
        (assign10170_e5338,)
    } else {
        (locals.var_uc_xpdv,)
    }
};
        locals.var_uc_xpdv = assign10170_e5340;
        locals.var_uc_xpdv_rv = 0.0;

        let assign10190_e5368: f64 = if ((p.p40 == 1.0) && (((p.p19 > 0.0) && (locals.var_uc_nover == 0.0)) || ((p.p18 > 0.0) && (locals.var_uc_novers == 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard250 = assign10190_e5368;
        locals.var_guard250_rv = 0.0;

        let (assign10200_e5372,) = {
    if (locals.var_guard250 != 0.0) {
        (0.0,)
    } else {
        (locals.var_uc_cordrift,)
    }
};
        locals.var_uc_cordrift = assign10200_e5372;
        locals.var_uc_cordrift_rv = 0.0;

        let (assign10210_e5377,) = {
    if (locals.var_guard250 == 0.0) {
        (p.p40,)
    } else {
        (locals.var_uc_cordrift,)
    }
};
        locals.var_uc_cordrift = assign10210_e5377;
        locals.var_uc_cordrift_rv = 0.0;

        let assign10220_e5380: f64 = if locals.var_uc_cordrift == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard251 = assign10220_e5380;
        locals.var_guard251_rv = 0.0;

        let (assign10230_e5389,) = {
    if (locals.var_guard251 != 0.0) {
        let (assign10230_e5387,) = {
            if (p.p19 > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10230_e5387,)
    } else {
        (locals.var_flg_rd,)
    }
};
        locals.var_flg_rd = assign10230_e5389;
        locals.var_flg_rd_rv = 0.0;

        let (assign10240_e5398,) = {
    if (locals.var_guard251 != 0.0) {
        let (assign10240_e5396,) = {
            if (p.p18 > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10240_e5396,)
    } else {
        (locals.var_flg_rs,)
    }
};
        locals.var_flg_rs = assign10240_e5398;
        locals.var_flg_rs_rv = 0.0;

        let assign10250_e5405: f64 = if ((p.p17 == 0.0) || (p.p17 == 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard252 = assign10250_e5405;
        locals.var_guard252_rv = 0.0;

        let (assign10260_e5412,) = {
    if ((locals.var_guard251 == 0.0) && (locals.var_guard252 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_rd,)
    }
};
        locals.var_flg_rd = assign10260_e5412;
        locals.var_flg_rd_rv = 0.0;

        let (assign10270_e5419,) = {
    if ((locals.var_guard251 == 0.0) && (locals.var_guard252 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_rs,)
    }
};
        locals.var_flg_rs = assign10270_e5419;
        locals.var_flg_rs_rv = 0.0;

        let (assign10280_e5451, assign10280_e5451_d_n0, assign10280_e5451_d_n2, assign10280_e5451_d_n4, assign10280_e5451_d_n5, assign10280_e5451_d_n6, assign10280_e5451_d_n7, assign10280_e5451_d_n8, assign10280_e5451_d_n9, assign10280_e5451_d_n10, assign10280_e5451_d_n11, assign10280_e5451_d_n14,) = {
    if ((locals.var_guard251 == 0.0) && (locals.var_guard252 == 0.0)) {
        let assign10280_e5427: f64 = (p.p130 * p.p2);
        let assign10280_e5429: f64 = (assign10280_e5427 * p.p7);
        let assign10280_e5432: f64 = (locals.var_uc_rd + locals.var_uc_rdvd);
        let assign10280_e5435: f64 = (p.p67 * locals.var_uc_rdslp1);
        let assign10280_e5437: f64 = (assign10280_e5435 * 1000000.0);
        let assign10280_e5439: f64 = (assign10280_e5437 + locals.var_uc_rdict1);
        let assign10280_e5440: f64 = (assign10280_e5432 * assign10280_e5439);
        let assign10280_e5443: f64 = (p.p68 * p.p100);
        let assign10280_e5445: f64 = (assign10280_e5443 * 1000000.0);
        let assign10280_e5447: f64 = (assign10280_e5445 + p.p101);
        let assign10280_e5448: f64 = (assign10280_e5440 * assign10280_e5447);
        let assign10280_e5449: f64 = (assign10280_e5429 + assign10280_e5448);
        (assign10280_e5449, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign10280_e5451;
        locals.var_t1_dn0 = assign10280_e5451_d_n0;
        locals.var_t1_dn2 = assign10280_e5451_d_n2;
        locals.var_t1_dn4 = assign10280_e5451_d_n4;
        locals.var_t1_dn5 = assign10280_e5451_d_n5;
        locals.var_t1_dn6 = assign10280_e5451_d_n6;
        locals.var_t1_dn7 = assign10280_e5451_d_n7;
        locals.var_t1_dn8 = assign10280_e5451_d_n8;
        locals.var_t1_dn9 = assign10280_e5451_d_n9;
        locals.var_t1_dn10 = assign10280_e5451_d_n10;
        locals.var_t1_dn11 = assign10280_e5451_d_n11;
        locals.var_t1_dn14 = assign10280_e5451_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign10290_e5464,) = {
    if ((locals.var_guard251 == 0.0) && (locals.var_guard252 == 0.0)) {
        let (assign10290_e5462,) = {
            if (locals.var_t1 > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10290_e5462,)
    } else {
        (locals.var_flg_rd,)
    }
};
        locals.var_flg_rd = assign10290_e5464;
        locals.var_flg_rd_rv = 0.0;

        let (assign10300_e5494, assign10300_e5494_d_n0, assign10300_e5494_d_n2, assign10300_e5494_d_n4, assign10300_e5494_d_n5, assign10300_e5494_d_n6, assign10300_e5494_d_n7, assign10300_e5494_d_n8, assign10300_e5494_d_n9, assign10300_e5494_d_n10, assign10300_e5494_d_n11, assign10300_e5494_d_n14,) = {
    if ((locals.var_guard251 == 0.0) && (locals.var_guard252 == 0.0)) {
        let assign10300_e5472: f64 = (p.p131 * p.p3);
        let assign10300_e5474: f64 = (assign10300_e5472 * p.p7);
        let assign10300_e5478: f64 = (p.p69 * locals.var_uc_rdslp1);
        let assign10300_e5480: f64 = (assign10300_e5478 * 1000000.0);
        let assign10300_e5482: f64 = (assign10300_e5480 + locals.var_uc_rdict1);
        let assign10300_e5483: f64 = (locals.var_uc_rs * assign10300_e5482);
        let assign10300_e5486: f64 = (p.p70 * p.p100);
        let assign10300_e5488: f64 = (assign10300_e5486 * 1000000.0);
        let assign10300_e5490: f64 = (assign10300_e5488 + p.p101);
        let assign10300_e5491: f64 = (assign10300_e5483 * assign10300_e5490);
        let assign10300_e5492: f64 = (assign10300_e5474 + assign10300_e5491);
        (assign10300_e5492, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign10300_e5494;
        locals.var_t1_dn0 = assign10300_e5494_d_n0;
        locals.var_t1_dn2 = assign10300_e5494_d_n2;
        locals.var_t1_dn4 = assign10300_e5494_d_n4;
        locals.var_t1_dn5 = assign10300_e5494_d_n5;
        locals.var_t1_dn6 = assign10300_e5494_d_n6;
        locals.var_t1_dn7 = assign10300_e5494_d_n7;
        locals.var_t1_dn8 = assign10300_e5494_d_n8;
        locals.var_t1_dn9 = assign10300_e5494_d_n9;
        locals.var_t1_dn10 = assign10300_e5494_d_n10;
        locals.var_t1_dn11 = assign10300_e5494_d_n11;
        locals.var_t1_dn14 = assign10300_e5494_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign10310_e5507,) = {
    if ((locals.var_guard251 == 0.0) && (locals.var_guard252 == 0.0)) {
        let (assign10310_e5505,) = {
            if (locals.var_t1 > 0.0) {
                (1.0,)
            } else {
                (0.0,)
            }
        };
        (assign10310_e5505,)
    } else {
        (locals.var_flg_rs,)
    }
};
        locals.var_flg_rs = assign10310_e5507;
        locals.var_flg_rs_rv = 0.0;

        let assign10320_e5510: f64 = (p.p12 / 1e-6);
        locals.var_mks_nsubcdfm = assign10320_e5510;
        locals.var_mks_nsubcdfm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_17(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign10330_e5513: f64 = (p.p73 * 100.0);
        locals.var_mks_subld2 = assign10330_e5513;
        locals.var_mks_subld2_rv = 0.0;

        let assign10340_e5516: f64 = (locals.var_uc_nsubc / 1e-6);
        locals.var_uc_nsubc = assign10340_e5516;
        locals.var_uc_nsubc_rv = 0.0;

        let assign10350_e5519: f64 = (locals.var_uc_nsubp / 1e-6);
        locals.var_uc_nsubp = assign10350_e5519;
        locals.var_uc_nsubp_rv = 0.0;

        let assign10360_e5522: f64 = (locals.var_uc_nsti / 1e-6);
        locals.var_uc_nsti = assign10360_e5522;
        locals.var_uc_nsti_rv = 0.0;

        let assign10370_e5525: f64 = (locals.var_uc_nover / 1e-6);
        locals.var_uc_nover = assign10370_e5525;
        locals.var_uc_nover_rv = 0.0;

        let assign10380_e5528: f64 = (locals.var_uc_novers / 1e-6);
        locals.var_uc_novers = assign10380_e5528;
        locals.var_uc_novers_rv = 0.0;

        let assign10390_e5531: f64 = (locals.var_uc_nsubpsti1 / 100.0);
        locals.var_uc_nsubpsti1 = assign10390_e5531;
        locals.var_uc_nsubpsti1_rv = 0.0;

        let assign10400_e5534: f64 = (locals.var_uc_muesti1 / 100.0);
        locals.var_uc_muesti1 = assign10400_e5534;
        locals.var_uc_muesti1_rv = 0.0;

        let assign10410_e5537: f64 = (locals.var_uc_vmax / 100.0);
        locals.var_uc_vmax = assign10410_e5537;
        locals.var_uc_vmax_rv = 0.0;

        let assign10420_e5540: f64 = (locals.var_uc_wfc * 10000.0);
        locals.var_uc_wfc = assign10420_e5540;
        locals.var_uc_wfc_rv = 0.0;

        let assign10430_e5543: f64 = (locals.var_uc_glksd1 / 100.0);
        locals.var_uc_glksd1 = assign10430_e5543;
        locals.var_uc_glksd1_rv = 0.0;

        let assign10440_e5546: f64 = (locals.var_uc_glksd2 * 100.0);
        locals.var_uc_glksd2 = assign10440_e5546;
        locals.var_uc_glksd2_rv = 0.0;

        let assign10450_e5549: f64 = (locals.var_uc_gleak2 * 100.0);
        locals.var_uc_gleak2 = assign10450_e5549;
        locals.var_uc_gleak2_rv = 0.0;

        let assign10460_e5552: f64 = (locals.var_uc_glkb2 * 100.0);
        locals.var_uc_glkb2 = assign10460_e5552;
        locals.var_uc_glkb2_rv = 0.0;

        let assign10470_e5555: f64 = (locals.var_uc_fn2 * 100.0);
        locals.var_uc_fn2 = assign10470_e5555;
        locals.var_uc_fn2_rv = 0.0;

        let assign10480_e5558: f64 = (locals.var_uc_gidl1 / 10.0);
        locals.var_uc_gidl1 = assign10480_e5558;
        locals.var_uc_gidl1_rv = 0.0;

        let assign10490_e5561: f64 = (locals.var_uc_gidl2 * 100.0);
        locals.var_uc_gidl2 = assign10490_e5561;
        locals.var_uc_gidl2_rv = 0.0;

        let assign10500_e5564: f64 = (locals.var_uc_nfalp / 100.0);
        locals.var_uc_nfalp = assign10500_e5564;
        locals.var_uc_nfalp_rv = 0.0;

        let assign10520_e5570: f64 = (locals.var_uc_npext / 1e-6);
        locals.var_uc_npext = assign10520_e5570;
        locals.var_uc_npext_rv = 0.0;

        let assign10530_e5573: f64 = (locals.var_uc_rd22 / 100.0);
        locals.var_uc_rd22 = assign10530_e5573;
        locals.var_uc_rd22_rv = 0.0;

        let assign10540_e5576: f64 = (locals.var_uc_rd23 / 100.0);
        locals.var_uc_rd23 = assign10540_e5576;
        locals.var_uc_rd23_rv = 0.0;

        let assign10550_e5579: f64 = (locals.var_uc_rd24 / 100.0);
        locals.var_uc_rd24 = assign10550_e5579;
        locals.var_uc_rd24_rv = 0.0;

        let assign10560_e5582: f64 = (locals.var_uc_rdvd / 100.0);
        locals.var_uc_rdvd = assign10560_e5582;
        locals.var_uc_rdvd_rv = 0.0;

        let assign10570_e5585: f64 = (locals.var_uc_rth0 / 100.0);
        locals.var_uc_rth0 = assign10570_e5585;
        locals.var_uc_rth0_rv = 0.0;

        let assign10580_e5587: f64 = (-locals.var_uc_vfbover);
        locals.var_uc_vfbover = assign10580_e5587;
        locals.var_uc_vfbover_rv = 0.0;

        let assign10590_e5590: f64 = (locals.var_uc_depvmax / 100.0);
        locals.var_uc_depvmax = assign10590_e5590;
        locals.var_uc_depvmax_dn0 = (locals.var_uc_depvmax_dn0 / 100.0);
        locals.var_uc_depvmax_dn2 = (locals.var_uc_depvmax_dn2 / 100.0);
        locals.var_uc_depvmax_dn4 = (locals.var_uc_depvmax_dn4 / 100.0);
        locals.var_uc_depvmax_dn5 = (locals.var_uc_depvmax_dn5 / 100.0);
        locals.var_uc_depvmax_dn6 = (locals.var_uc_depvmax_dn6 / 100.0);
        locals.var_uc_depvmax_dn7 = (locals.var_uc_depvmax_dn7 / 100.0);
        locals.var_uc_depvmax_dn8 = (locals.var_uc_depvmax_dn8 / 100.0);
        locals.var_uc_depvmax_dn9 = (locals.var_uc_depvmax_dn9 / 100.0);
        locals.var_uc_depvmax_dn10 = (locals.var_uc_depvmax_dn10 / 100.0);
        locals.var_uc_depvmax_dn11 = (locals.var_uc_depvmax_dn11 / 100.0);
        locals.var_uc_depvmax_dn14 = (locals.var_uc_depvmax_dn14 / 100.0);
        locals.var_uc_depvmax_rv = 0.0;

        locals.var_flg_nqs = p.p28;
        locals.var_flg_nqs_rv = 0.0;

        let (assign10610_e5601,) = {
    if ((p.p133 != 0.0) || (p.p134 != 0.0)) {
        (1.0,)
    } else {
        (0.0,)
    }
};
        locals.var_flg_qy = assign10610_e5601;
        locals.var_flg_qy_rv = 0.0;

        let assign10630_e5615: f64 = if (((p.p235 == 0.0) && (p.p237 == 0.0)) || (p.p236 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard254 = assign10630_e5615;
        locals.var_guard254_rv = 0.0;

        let (assign10640_e5619,) = {
    if (locals.var_guard254 != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_qmetemp,)
    }
};
        locals.var_flg_qmetemp = assign10640_e5619;
        locals.var_flg_qmetemp_rv = 0.0;

        let (assign10650_e5624,) = {
    if (locals.var_guard254 == 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_qmetemp,)
    }
};
        locals.var_flg_qmetemp = assign10650_e5624;
        locals.var_flg_qmetemp_rv = 0.0;

        let assign10660_e5627: f64 = (locals.var_wg * locals.var_lg);
        locals.var_wlg = assign10660_e5627;
        locals.var_wlg_rv = 0.0;

        let assign10670_e5630: f64 = (p.p289 * 1000000.0);
        locals.var_uc_gdld = assign10670_e5630;
        locals.var_uc_gdld_rv = 0.0;

        let assign10680_e5636: f64 = (locals.var_ktnom * 1e-7);
        let assign10680_e5637: f64 = (9.025e-5 + assign10680_e5636);
        let assign10680_e5638: f64 = (locals.var_ktnom * assign10680_e5637);
        let assign10680_e5639: f64 = (locals.var_uc_eg0 - assign10680_e5638);
        locals.var_egtnom = assign10680_e5639;
        locals.var_egtnom_rv = 0.0;

        let assign10690_e5642: f64 = (8.8541878e-12 * p.p267);
        locals.var_cecox = assign10690_e5642;
        locals.var_cecox_rv = 0.0;

        locals.var_msc = locals.var_uc_scp22;
        locals.var_msc_rv = 0.0;

        let assign10710_e5646: f64 = if locals.var_uc_pgd1 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard255 = assign10710_e5646;
        locals.var_guard255_rv = 0.0;

        let (assign10720_e5650,) = {
    if (locals.var_guard255 != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_pgd,)
    }
};
        locals.var_flg_pgd = assign10720_e5650;
        locals.var_flg_pgd_rv = 0.0;

        let (assign10730_e5654,) = {
    if (locals.var_guard255 != 0.0) {
        (0.0,)
    } else {
        (locals.var_cnstpgd,)
    }
};
        locals.var_cnstpgd = assign10730_e5654;
        locals.var_cnstpgd_rv = 0.0;

        let (assign10740_e5659,) = {
    if (locals.var_guard255 == 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_pgd,)
    }
};
        locals.var_flg_pgd = assign10740_e5659;
        locals.var_flg_pgd_rv = 0.0;

        let (assign10750_e5672,) = {
    if (locals.var_guard255 == 0.0) {
        let assign10750_e5665: f64 = (1.0 / locals.var_lg);
        let assign10750_e5666: f64 = (1.0 + assign10750_e5665);
        let assign10750_e5668: f64 = (assign10750_e5666).powf(p.p153);
        let assign10750_e5670: f64 = (assign10750_e5668 * locals.var_uc_pgd1);
        (assign10750_e5670,)
    } else {
        (locals.var_cnstpgd,)
    }
};
        locals.var_cnstpgd = assign10750_e5672;
        locals.var_cnstpgd_rv = 0.0;

        let assign10760_e5676: f64 = (locals.var_lg).powf(p.p229);
        let assign10760_e5678: f64 = (assign10760_e5676 * p.p230);
        let assign10760_e5679: f64 = (1.0 + assign10760_e5678);
        locals.var_clmmod = assign10760_e5679;
        locals.var_clmmod_rv = 0.0;

        let assign10770_e5684: f64 = (0.5 * p.p0);
        let assign10770_e5685: f64 = (p.p118 + assign10770_e5684);
        let assign10770_e5686: f64 = (1.0 / assign10770_e5685);
        let assign10770_e5691: f64 = (0.5 * p.p0);
        let assign10770_e5692: f64 = (p.p119 + assign10770_e5691);
        let assign10770_e5693: f64 = (1.0 / assign10770_e5692);
        let assign10770_e5694: f64 = (assign10770_e5686 + assign10770_e5693);
        locals.var_t1 = assign10770_e5694;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign10780_e5697: f64 = (2.0 / locals.var_t1);
        locals.var_lod_half_ref = assign10780_e5697;
        locals.var_lod_half_ref_dn0 = (-((2.0 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn2 = (-((2.0 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn4 = (-((2.0 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn5 = (-((2.0 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn6 = (-((2.0 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn7 = (-((2.0 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn8 = (-((2.0 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn9 = (-((2.0 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn10 = (-((2.0 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn11 = (-((2.0 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn14 = (-((2.0 * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_rv = 0.0;

        let assign10790_e5716: f64 = if (((p.p8 > 0.0) && (p.p9 > 0.0)) && ((p.p7 == 1.0) || ((p.p7 > 1.0) && (p.p10 > 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard256 = assign10790_e5716;
        locals.var_guard256_rv = 0.0;

        let (assign10800_e5720, assign10800_e5720_d_n0, assign10800_e5720_d_n2, assign10800_e5720_d_n4, assign10800_e5720_d_n5, assign10800_e5720_d_n6, assign10800_e5720_d_n7, assign10800_e5720_d_n8, assign10800_e5720_d_n9, assign10800_e5720_d_n10, assign10800_e5720_d_n11, assign10800_e5720_d_n14,) = {
    if (locals.var_guard256 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign10800_e5720;
        locals.var_t1_dn0 = assign10800_e5720_d_n0;
        locals.var_t1_dn2 = assign10800_e5720_d_n2;
        locals.var_t1_dn4 = assign10800_e5720_d_n4;
        locals.var_t1_dn5 = assign10800_e5720_d_n5;
        locals.var_t1_dn6 = assign10800_e5720_d_n6;
        locals.var_t1_dn7 = assign10800_e5720_d_n7;
        locals.var_t1_dn8 = assign10800_e5720_d_n8;
        locals.var_t1_dn9 = assign10800_e5720_d_n9;
        locals.var_t1_dn10 = assign10800_e5720_d_n10;
        locals.var_t1_dn11 = assign10800_e5720_d_n11;
        locals.var_t1_dn14 = assign10800_e5720_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign10810_e5724,) = {
    if (locals.var_guard256 != 0.0) {
        (0.0,)
    } else {
        (locals.var_i,)
    }
};
        locals.var_i = assign10810_e5724;
        locals.var_i_rv = 0.0;

        let mut assign10820_loop_guard: usize = 0;
        while {
            let assign10820_cond_e5729: f64 = if ((locals.var_guard256 != 0.0) && (locals.var_i < p.p7)) { 1.0 } else { 0.0 };
            assign10820_cond_e5729 != 0.0
        } {
            assign10820_loop_guard += 1;
            assert!(assign10820_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign10820_body0_e5761, assign10820_body0_e5761_d_n0, assign10820_body0_e5761_d_n2, assign10820_body0_e5761_d_n4, assign10820_body0_e5761_d_n5, assign10820_body0_e5761_d_n6, assign10820_body0_e5761_d_n7, assign10820_body0_e5761_d_n8, assign10820_body0_e5761_d_n9, assign10820_body0_e5761_d_n10, assign10820_body0_e5761_d_n11, assign10820_body0_e5761_d_n14,) = {
    if (locals.var_guard256 != 0.0) {
        let assign10820_body0_e5736: f64 = (0.5 * p.p0);
        let assign10820_body0_e5737: f64 = (p.p8 + assign10820_body0_e5736);
        let assign10820_body0_e5741: f64 = (p.p10 + p.p0);
        let assign10820_body0_e5742: f64 = (locals.var_i * assign10820_body0_e5741);
        let assign10820_body0_e5743: f64 = (assign10820_body0_e5737 + assign10820_body0_e5742);
        let assign10820_body0_e5744: f64 = (1.0 / assign10820_body0_e5743);
        let assign10820_body0_e5745: f64 = (locals.var_t1 + assign10820_body0_e5744);
        let assign10820_body0_e5750: f64 = (0.5 * p.p0);
        let assign10820_body0_e5751: f64 = (p.p9 + assign10820_body0_e5750);
        let assign10820_body0_e5755: f64 = (p.p10 + p.p0);
        let assign10820_body0_e5756: f64 = (locals.var_i * assign10820_body0_e5755);
        let assign10820_body0_e5757: f64 = (assign10820_body0_e5751 + assign10820_body0_e5756);
        let assign10820_body0_e5758: f64 = (1.0 / assign10820_body0_e5757);
        let assign10820_body0_e5759: f64 = (assign10820_body0_e5745 + assign10820_body0_e5758);
        (assign10820_body0_e5759, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign10820_body0_e5761;
            locals.var_t1_dn0 = assign10820_body0_e5761_d_n0;
            locals.var_t1_dn2 = assign10820_body0_e5761_d_n2;
            locals.var_t1_dn4 = assign10820_body0_e5761_d_n4;
            locals.var_t1_dn5 = assign10820_body0_e5761_d_n5;
            locals.var_t1_dn6 = assign10820_body0_e5761_d_n6;
            locals.var_t1_dn7 = assign10820_body0_e5761_d_n7;
            locals.var_t1_dn8 = assign10820_body0_e5761_d_n8;
            locals.var_t1_dn9 = assign10820_body0_e5761_d_n9;
            locals.var_t1_dn10 = assign10820_body0_e5761_d_n10;
            locals.var_t1_dn11 = assign10820_body0_e5761_d_n11;
            locals.var_t1_dn14 = assign10820_body0_e5761_d_n14;
            locals.var_t1_rv = 0.0;
            let (assign10820_body1_e5767,) = {
    if (locals.var_guard256 != 0.0) {
        let assign10820_body1_e5765: f64 = (locals.var_i + 1.0);
        (assign10820_body1_e5765,)
    } else {
        (locals.var_i,)
    }
};
            locals.var_i = assign10820_body1_e5767;
            locals.var_i_rv = 0.0;
        }

        let (assign10830_e5775, assign10830_e5775_d_n0, assign10830_e5775_d_n2, assign10830_e5775_d_n4, assign10830_e5775_d_n5, assign10830_e5775_d_n6, assign10830_e5775_d_n7, assign10830_e5775_d_n8, assign10830_e5775_d_n9, assign10830_e5775_d_n10, assign10830_e5775_d_n11, assign10830_e5775_d_n14,) = {
    if (locals.var_guard256 != 0.0) {
        let assign10830_e5771: f64 = (2.0 * p.p7);
        let assign10830_e5773: f64 = (assign10830_e5771 / locals.var_t1);
        (assign10830_e5773, (-((assign10830_e5771 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((assign10830_e5771 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((assign10830_e5771 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((assign10830_e5771 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((assign10830_e5771 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((assign10830_e5771 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((assign10830_e5771 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((assign10830_e5771 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((assign10830_e5771 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((assign10830_e5771 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((assign10830_e5771 * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_lod_half, locals.var_lod_half_dn0, locals.var_lod_half_dn2, locals.var_lod_half_dn4, locals.var_lod_half_dn5, locals.var_lod_half_dn6, locals.var_lod_half_dn7, locals.var_lod_half_dn8, locals.var_lod_half_dn9, locals.var_lod_half_dn10, locals.var_lod_half_dn11, locals.var_lod_half_dn14,)
    }
};
        locals.var_lod_half = assign10830_e5775;
        locals.var_lod_half_dn0 = assign10830_e5775_d_n0;
        locals.var_lod_half_dn2 = assign10830_e5775_d_n2;
        locals.var_lod_half_dn4 = assign10830_e5775_d_n4;
        locals.var_lod_half_dn5 = assign10830_e5775_d_n5;
        locals.var_lod_half_dn6 = assign10830_e5775_d_n6;
        locals.var_lod_half_dn7 = assign10830_e5775_d_n7;
        locals.var_lod_half_dn8 = assign10830_e5775_d_n8;
        locals.var_lod_half_dn9 = assign10830_e5775_d_n9;
        locals.var_lod_half_dn10 = assign10830_e5775_d_n10;
        locals.var_lod_half_dn11 = assign10830_e5775_d_n11;
        locals.var_lod_half_dn14 = assign10830_e5775_d_n14;
        locals.var_lod_half_rv = 0.0;

        let (assign10840_e5780, assign10840_e5780_d_n0, assign10840_e5780_d_n2, assign10840_e5780_d_n4, assign10840_e5780_d_n5, assign10840_e5780_d_n6, assign10840_e5780_d_n7, assign10840_e5780_d_n8, assign10840_e5780_d_n9, assign10840_e5780_d_n10, assign10840_e5780_d_n11, assign10840_e5780_d_n14,) = {
    if (locals.var_guard256 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lod_half, locals.var_lod_half_dn0, locals.var_lod_half_dn2, locals.var_lod_half_dn4, locals.var_lod_half_dn5, locals.var_lod_half_dn6, locals.var_lod_half_dn7, locals.var_lod_half_dn8, locals.var_lod_half_dn9, locals.var_lod_half_dn10, locals.var_lod_half_dn11, locals.var_lod_half_dn14,)
    }
};
        locals.var_lod_half = assign10840_e5780;
        locals.var_lod_half_dn0 = assign10840_e5780_d_n0;
        locals.var_lod_half_dn2 = assign10840_e5780_d_n2;
        locals.var_lod_half_dn4 = assign10840_e5780_d_n4;
        locals.var_lod_half_dn5 = assign10840_e5780_d_n5;
        locals.var_lod_half_dn6 = assign10840_e5780_d_n6;
        locals.var_lod_half_dn7 = assign10840_e5780_d_n7;
        locals.var_lod_half_dn8 = assign10840_e5780_d_n8;
        locals.var_lod_half_dn9 = assign10840_e5780_d_n9;
        locals.var_lod_half_dn10 = assign10840_e5780_d_n10;
        locals.var_lod_half_dn11 = assign10840_e5780_d_n11;
        locals.var_lod_half_dn14 = assign10840_e5780_d_n14;
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
        locals.var_npexte_dn11 = 0.0;
        locals.var_npexte_dn14 = 0.0;
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
        locals.var_ef_mueph1_dn11 = 0.0;
        locals.var_ef_mueph1_dn14 = 0.0;
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
        locals.var_ef_nsubp_dn11 = 0.0;
        locals.var_ef_nsubp_dn14 = 0.0;
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
        locals.var_ef_nsubc_dn11 = 0.0;
        locals.var_ef_nsubc_dn14 = 0.0;
        locals.var_ef_nsubc_rv = 0.0;

        let assign10890_e5789: f64 = if ((p.p32 == 1.0) && (locals.var_nsubcdfm_given != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard257 = assign10890_e5789;
        locals.var_guard257_rv = 0.0;

        let (assign10910_e5810, assign10910_e5810_d_n0, assign10910_e5810_d_n2, assign10910_e5810_d_n4, assign10910_e5810_d_n5, assign10910_e5810_d_n6, assign10910_e5810_d_n7, assign10910_e5810_d_n8, assign10910_e5810_d_n9, assign10910_e5810_d_n10, assign10910_e5810_d_n11, assign10910_e5810_d_n14,) = {
    if (locals.var_guard257 != 0.0) {
        let assign10910_e5801: f64 = (locals.var_mks_nsubcdfm).ln();
        let assign10910_e5803: f64 = (locals.var_ef_nsubc).ln();
        let assign10910_e5804: f64 = (assign10910_e5801 - assign10910_e5803);
        let assign10910_e5805: f64 = (p.p282 * assign10910_e5804);
        let assign10910_e5807: f64 = (assign10910_e5805 + 1.0);
        let assign10910_e5808: f64 = (locals.var_ef_mueph1 * assign10910_e5807);
        (assign10910_e5808, ((locals.var_ef_mueph1_dn0 * assign10910_e5807) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn0 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn2 * assign10910_e5807) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn2 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn4 * assign10910_e5807) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn4 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn5 * assign10910_e5807) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn5 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn6 * assign10910_e5807) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn6 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn7 * assign10910_e5807) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn7 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn8 * assign10910_e5807) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn8 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn9 * assign10910_e5807) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn9 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn10 * assign10910_e5807) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn10 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn11 * assign10910_e5807) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn11 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn14 * assign10910_e5807) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn14 / locals.var_ef_nsubc))))),)
    } else {
        (locals.var_ef_mueph1, locals.var_ef_mueph1_dn0, locals.var_ef_mueph1_dn2, locals.var_ef_mueph1_dn4, locals.var_ef_mueph1_dn5, locals.var_ef_mueph1_dn6, locals.var_ef_mueph1_dn7, locals.var_ef_mueph1_dn8, locals.var_ef_mueph1_dn9, locals.var_ef_mueph1_dn10, locals.var_ef_mueph1_dn11, locals.var_ef_mueph1_dn14,)
    }
};
        locals.var_ef_mueph1 = assign10910_e5810;
        locals.var_ef_mueph1_dn0 = assign10910_e5810_d_n0;
        locals.var_ef_mueph1_dn2 = assign10910_e5810_d_n2;
        locals.var_ef_mueph1_dn4 = assign10910_e5810_d_n4;
        locals.var_ef_mueph1_dn5 = assign10910_e5810_d_n5;
        locals.var_ef_mueph1_dn6 = assign10910_e5810_d_n6;
        locals.var_ef_mueph1_dn7 = assign10910_e5810_d_n7;
        locals.var_ef_mueph1_dn8 = assign10910_e5810_d_n8;
        locals.var_ef_mueph1_dn9 = assign10910_e5810_d_n9;
        locals.var_ef_mueph1_dn10 = assign10910_e5810_d_n10;
        locals.var_ef_mueph1_dn11 = assign10910_e5810_d_n11;
        locals.var_ef_mueph1_dn14 = assign10910_e5810_d_n14;
        locals.var_ef_mueph1_rv = 0.0;

        let (assign10920_e5818, assign10920_e5818_d_n0, assign10920_e5818_d_n2, assign10920_e5818_d_n4, assign10920_e5818_d_n5, assign10920_e5818_d_n6, assign10920_e5818_d_n7, assign10920_e5818_d_n8, assign10920_e5818_d_n9, assign10920_e5818_d_n10, assign10920_e5818_d_n11, assign10920_e5818_d_n14,) = {
    if (locals.var_guard257 != 0.0) {
        let assign10920_e5814: f64 = (locals.var_ef_nsubp + locals.var_mks_nsubcdfm);
        let assign10920_e5816: f64 = (assign10920_e5814 - locals.var_ef_nsubc);
        (assign10920_e5816, (locals.var_ef_nsubp_dn0 - locals.var_ef_nsubc_dn0), (locals.var_ef_nsubp_dn2 - locals.var_ef_nsubc_dn2), (locals.var_ef_nsubp_dn4 - locals.var_ef_nsubc_dn4), (locals.var_ef_nsubp_dn5 - locals.var_ef_nsubc_dn5), (locals.var_ef_nsubp_dn6 - locals.var_ef_nsubc_dn6), (locals.var_ef_nsubp_dn7 - locals.var_ef_nsubc_dn7), (locals.var_ef_nsubp_dn8 - locals.var_ef_nsubc_dn8), (locals.var_ef_nsubp_dn9 - locals.var_ef_nsubc_dn9), (locals.var_ef_nsubp_dn10 - locals.var_ef_nsubc_dn10), (locals.var_ef_nsubp_dn11 - locals.var_ef_nsubc_dn11), (locals.var_ef_nsubp_dn14 - locals.var_ef_nsubc_dn14),)
    } else {
        (locals.var_ef_nsubp, locals.var_ef_nsubp_dn0, locals.var_ef_nsubp_dn2, locals.var_ef_nsubp_dn4, locals.var_ef_nsubp_dn5, locals.var_ef_nsubp_dn6, locals.var_ef_nsubp_dn7, locals.var_ef_nsubp_dn8, locals.var_ef_nsubp_dn9, locals.var_ef_nsubp_dn10, locals.var_ef_nsubp_dn11, locals.var_ef_nsubp_dn14,)
    }
};
        locals.var_ef_nsubp = assign10920_e5818;
        locals.var_ef_nsubp_dn0 = assign10920_e5818_d_n0;
        locals.var_ef_nsubp_dn2 = assign10920_e5818_d_n2;
        locals.var_ef_nsubp_dn4 = assign10920_e5818_d_n4;
        locals.var_ef_nsubp_dn5 = assign10920_e5818_d_n5;
        locals.var_ef_nsubp_dn6 = assign10920_e5818_d_n6;
        locals.var_ef_nsubp_dn7 = assign10920_e5818_d_n7;
        locals.var_ef_nsubp_dn8 = assign10920_e5818_d_n8;
        locals.var_ef_nsubp_dn9 = assign10920_e5818_d_n9;
        locals.var_ef_nsubp_dn10 = assign10920_e5818_d_n10;
        locals.var_ef_nsubp_dn11 = assign10920_e5818_d_n11;
        locals.var_ef_nsubp_dn14 = assign10920_e5818_d_n14;
        locals.var_ef_nsubp_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_18(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10930_e5826, assign10930_e5826_d_n0, assign10930_e5826_d_n2, assign10930_e5826_d_n4, assign10930_e5826_d_n5, assign10930_e5826_d_n6, assign10930_e5826_d_n7, assign10930_e5826_d_n8, assign10930_e5826_d_n9, assign10930_e5826_d_n10, assign10930_e5826_d_n11, assign10930_e5826_d_n14,) = {
    if (locals.var_guard257 != 0.0) {
        let assign10930_e5822: f64 = (locals.var_npexte + locals.var_mks_nsubcdfm);
        let assign10930_e5824: f64 = (assign10930_e5822 - locals.var_ef_nsubc);
        (assign10930_e5824, (locals.var_npexte_dn0 - locals.var_ef_nsubc_dn0), (locals.var_npexte_dn2 - locals.var_ef_nsubc_dn2), (locals.var_npexte_dn4 - locals.var_ef_nsubc_dn4), (locals.var_npexte_dn5 - locals.var_ef_nsubc_dn5), (locals.var_npexte_dn6 - locals.var_ef_nsubc_dn6), (locals.var_npexte_dn7 - locals.var_ef_nsubc_dn7), (locals.var_npexte_dn8 - locals.var_ef_nsubc_dn8), (locals.var_npexte_dn9 - locals.var_ef_nsubc_dn9), (locals.var_npexte_dn10 - locals.var_ef_nsubc_dn10), (locals.var_npexte_dn11 - locals.var_ef_nsubc_dn11), (locals.var_npexte_dn14 - locals.var_ef_nsubc_dn14),)
    } else {
        (locals.var_npexte, locals.var_npexte_dn0, locals.var_npexte_dn2, locals.var_npexte_dn4, locals.var_npexte_dn5, locals.var_npexte_dn6, locals.var_npexte_dn7, locals.var_npexte_dn8, locals.var_npexte_dn9, locals.var_npexte_dn10, locals.var_npexte_dn11, locals.var_npexte_dn14,)
    }
};
        locals.var_npexte = assign10930_e5826;
        locals.var_npexte_dn0 = assign10930_e5826_d_n0;
        locals.var_npexte_dn2 = assign10930_e5826_d_n2;
        locals.var_npexte_dn4 = assign10930_e5826_d_n4;
        locals.var_npexte_dn5 = assign10930_e5826_d_n5;
        locals.var_npexte_dn6 = assign10930_e5826_d_n6;
        locals.var_npexte_dn7 = assign10930_e5826_d_n7;
        locals.var_npexte_dn8 = assign10930_e5826_d_n8;
        locals.var_npexte_dn9 = assign10930_e5826_d_n9;
        locals.var_npexte_dn10 = assign10930_e5826_d_n10;
        locals.var_npexte_dn11 = assign10930_e5826_d_n11;
        locals.var_npexte_dn14 = assign10930_e5826_d_n14;
        locals.var_npexte_rv = 0.0;

        let (assign10940_e5830, assign10940_e5830_d_n0, assign10940_e5830_d_n2, assign10940_e5830_d_n4, assign10940_e5830_d_n5, assign10940_e5830_d_n6, assign10940_e5830_d_n7, assign10940_e5830_d_n8, assign10940_e5830_d_n9, assign10940_e5830_d_n10, assign10940_e5830_d_n11, assign10940_e5830_d_n14,) = {
    if (locals.var_guard257 != 0.0) {
        (locals.var_mks_nsubcdfm, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ef_nsubc, locals.var_ef_nsubc_dn0, locals.var_ef_nsubc_dn2, locals.var_ef_nsubc_dn4, locals.var_ef_nsubc_dn5, locals.var_ef_nsubc_dn6, locals.var_ef_nsubc_dn7, locals.var_ef_nsubc_dn8, locals.var_ef_nsubc_dn9, locals.var_ef_nsubc_dn10, locals.var_ef_nsubc_dn11, locals.var_ef_nsubc_dn14,)
    }
};
        locals.var_ef_nsubc = assign10940_e5830;
        locals.var_ef_nsubc_dn0 = assign10940_e5830_d_n0;
        locals.var_ef_nsubc_dn2 = assign10940_e5830_d_n2;
        locals.var_ef_nsubc_dn4 = assign10940_e5830_d_n4;
        locals.var_ef_nsubc_dn5 = assign10940_e5830_d_n5;
        locals.var_ef_nsubc_dn6 = assign10940_e5830_d_n6;
        locals.var_ef_nsubc_dn7 = assign10940_e5830_d_n7;
        locals.var_ef_nsubc_dn8 = assign10940_e5830_d_n8;
        locals.var_ef_nsubc_dn9 = assign10940_e5830_d_n9;
        locals.var_ef_nsubc_dn10 = assign10940_e5830_d_n10;
        locals.var_ef_nsubc_dn11 = assign10940_e5830_d_n11;
        locals.var_ef_nsubc_dn14 = assign10940_e5830_d_n14;
        locals.var_ef_nsubc_rv = 0.0;

        let assign10950_e5836: f64 = (locals.var_wg).powf(p.p163);
        let assign10950_e5837: f64 = (p.p162 / assign10950_e5836);
        let assign10950_e5838: f64 = (1.0 + assign10950_e5837);
        let assign10950_e5839: f64 = (locals.var_ef_mueph1 * assign10950_e5838);
        let assign10950_e5844: f64 = (locals.var_lg).powf(p.p165);
        let assign10950_e5845: f64 = (p.p164 / assign10950_e5844);
        let assign10950_e5846: f64 = (1.0 + assign10950_e5845);
        let assign10950_e5847: f64 = (assign10950_e5839 * assign10950_e5846);
        let assign10950_e5852: f64 = (locals.var_wlg).powf(p.p168);
        let assign10950_e5853: f64 = (p.p167 / assign10950_e5852);
        let assign10950_e5854: f64 = (1.0 + assign10950_e5853);
        let assign10950_e5855: f64 = (assign10950_e5847 * assign10950_e5854);
        locals.var_mueph = assign10950_e5855;
        locals.var_mueph_dn0 = (((locals.var_ef_mueph1_dn0 * assign10950_e5838) * assign10950_e5846) * assign10950_e5854);
        locals.var_mueph_dn2 = (((locals.var_ef_mueph1_dn2 * assign10950_e5838) * assign10950_e5846) * assign10950_e5854);
        locals.var_mueph_dn4 = (((locals.var_ef_mueph1_dn4 * assign10950_e5838) * assign10950_e5846) * assign10950_e5854);
        locals.var_mueph_dn5 = (((locals.var_ef_mueph1_dn5 * assign10950_e5838) * assign10950_e5846) * assign10950_e5854);
        locals.var_mueph_dn6 = (((locals.var_ef_mueph1_dn6 * assign10950_e5838) * assign10950_e5846) * assign10950_e5854);
        locals.var_mueph_dn7 = (((locals.var_ef_mueph1_dn7 * assign10950_e5838) * assign10950_e5846) * assign10950_e5854);
        locals.var_mueph_dn8 = (((locals.var_ef_mueph1_dn8 * assign10950_e5838) * assign10950_e5846) * assign10950_e5854);
        locals.var_mueph_dn9 = (((locals.var_ef_mueph1_dn9 * assign10950_e5838) * assign10950_e5846) * assign10950_e5854);
        locals.var_mueph_dn10 = (((locals.var_ef_mueph1_dn10 * assign10950_e5838) * assign10950_e5846) * assign10950_e5854);
        locals.var_mueph_dn11 = (((locals.var_ef_mueph1_dn11 * assign10950_e5838) * assign10950_e5846) * assign10950_e5854);
        locals.var_mueph_dn14 = (((locals.var_ef_mueph1_dn14 * assign10950_e5838) * assign10950_e5846) * assign10950_e5854);
        locals.var_mueph_rv = 0.0;

        let assign10960_e5858: f64 = if locals.var_lod_half > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard259 = assign10960_e5858;
        locals.var_guard259_rv = 0.0;

        let (assign10970_e5866, assign10970_e5866_d_n0, assign10970_e5866_d_n2, assign10970_e5866_d_n4, assign10970_e5866_d_n5, assign10970_e5866_d_n6, assign10970_e5866_d_n7, assign10970_e5866_d_n8, assign10970_e5866_d_n9, assign10970_e5866_d_n10, assign10970_e5866_d_n11, assign10970_e5866_d_n14,) = {
    if (locals.var_guard259 != 0.0) {
        let assign10970_e5863: f64 = (1.0 + locals.var_uc_muesti2);
        let assign10970_e5864: f64 = (1.0 / assign10970_e5863);
        (assign10970_e5864, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign10970_e5866;
        locals.var_t1_dn0 = assign10970_e5866_d_n0;
        locals.var_t1_dn2 = assign10970_e5866_d_n2;
        locals.var_t1_dn4 = assign10970_e5866_d_n4;
        locals.var_t1_dn5 = assign10970_e5866_d_n5;
        locals.var_t1_dn6 = assign10970_e5866_d_n6;
        locals.var_t1_dn7 = assign10970_e5866_d_n7;
        locals.var_t1_dn8 = assign10970_e5866_d_n8;
        locals.var_t1_dn9 = assign10970_e5866_d_n9;
        locals.var_t1_dn10 = assign10970_e5866_d_n10;
        locals.var_t1_dn11 = assign10970_e5866_d_n11;
        locals.var_t1_dn14 = assign10970_e5866_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign10980_e5874, assign10980_e5874_d_n0, assign10980_e5874_d_n2, assign10980_e5874_d_n4, assign10980_e5874_d_n5, assign10980_e5874_d_n6, assign10980_e5874_d_n7, assign10980_e5874_d_n8, assign10980_e5874_d_n9, assign10980_e5874_d_n10, assign10980_e5874_d_n11, assign10980_e5874_d_n14,) = {
    if (locals.var_guard259 != 0.0) {
        let assign10980_e5870: f64 = (locals.var_uc_muesti1 / locals.var_lod_half);
        let assign10980_e5872: f64 = (assign10980_e5870).powf(locals.var_uc_muesti3);
        (assign10980_e5872, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10980_e5870).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10980_e5872 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))) / assign10980_e5870))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10980_e5870).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10980_e5872 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))) / assign10980_e5870))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10980_e5870).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn4) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10980_e5872 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn4) / (locals.var_lod_half * locals.var_lod_half))) / assign10980_e5870))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10980_e5870).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn5) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10980_e5872 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn5) / (locals.var_lod_half * locals.var_lod_half))) / assign10980_e5870))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10980_e5870).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10980_e5872 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))) / assign10980_e5870))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10980_e5870).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn7) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10980_e5872 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn7) / (locals.var_lod_half * locals.var_lod_half))) / assign10980_e5870))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10980_e5870).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn8) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10980_e5872 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn8) / (locals.var_lod_half * locals.var_lod_half))) / assign10980_e5870))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10980_e5870).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn9) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10980_e5872 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn9) / (locals.var_lod_half * locals.var_lod_half))) / assign10980_e5870))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10980_e5870).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10980_e5872 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))) / assign10980_e5870))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10980_e5870).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn11) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10980_e5872 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn11) / (locals.var_lod_half * locals.var_lod_half))) / assign10980_e5870))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10980_e5870).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn14) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10980_e5872 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn14) / (locals.var_lod_half * locals.var_lod_half))) / assign10980_e5870))) },)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign10980_e5874;
        locals.var_t2_dn0 = assign10980_e5874_d_n0;
        locals.var_t2_dn2 = assign10980_e5874_d_n2;
        locals.var_t2_dn4 = assign10980_e5874_d_n4;
        locals.var_t2_dn5 = assign10980_e5874_d_n5;
        locals.var_t2_dn6 = assign10980_e5874_d_n6;
        locals.var_t2_dn7 = assign10980_e5874_d_n7;
        locals.var_t2_dn8 = assign10980_e5874_d_n8;
        locals.var_t2_dn9 = assign10980_e5874_d_n9;
        locals.var_t2_dn10 = assign10980_e5874_d_n10;
        locals.var_t2_dn11 = assign10980_e5874_d_n11;
        locals.var_t2_dn14 = assign10980_e5874_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign10990_e5882, assign10990_e5882_d_n0, assign10990_e5882_d_n2, assign10990_e5882_d_n4, assign10990_e5882_d_n5, assign10990_e5882_d_n6, assign10990_e5882_d_n7, assign10990_e5882_d_n8, assign10990_e5882_d_n9, assign10990_e5882_d_n10, assign10990_e5882_d_n11, assign10990_e5882_d_n14,) = {
    if (locals.var_guard259 != 0.0) {
        let assign10990_e5878: f64 = (locals.var_uc_muesti1 / locals.var_lod_half_ref);
        let assign10990_e5880: f64 = (assign10990_e5878).powf(locals.var_uc_muesti3);
        (assign10990_e5880, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10990_e5878).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn0) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10990_e5880 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn0) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10990_e5878))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10990_e5878).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn2) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10990_e5880 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn2) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10990_e5878))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10990_e5878).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn4) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10990_e5880 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn4) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10990_e5878))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10990_e5878).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn5) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10990_e5880 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn5) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10990_e5878))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10990_e5878).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn6) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10990_e5880 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn6) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10990_e5878))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10990_e5878).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn7) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10990_e5880 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn7) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10990_e5878))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10990_e5878).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn8) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10990_e5880 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn8) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10990_e5878))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10990_e5878).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn9) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10990_e5880 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn9) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10990_e5878))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10990_e5878).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn10) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10990_e5880 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn10) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10990_e5878))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10990_e5878).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn11) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10990_e5880 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn11) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10990_e5878))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10990_e5878).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn14) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10990_e5880 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn14) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10990_e5878))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign10990_e5882;
        locals.var_t3_dn0 = assign10990_e5882_d_n0;
        locals.var_t3_dn2 = assign10990_e5882_d_n2;
        locals.var_t3_dn4 = assign10990_e5882_d_n4;
        locals.var_t3_dn5 = assign10990_e5882_d_n5;
        locals.var_t3_dn6 = assign10990_e5882_d_n6;
        locals.var_t3_dn7 = assign10990_e5882_d_n7;
        locals.var_t3_dn8 = assign10990_e5882_d_n8;
        locals.var_t3_dn9 = assign10990_e5882_d_n9;
        locals.var_t3_dn10 = assign10990_e5882_d_n10;
        locals.var_t3_dn11 = assign10990_e5882_d_n11;
        locals.var_t3_dn14 = assign10990_e5882_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign11000_e5898, assign11000_e5898_d_n0, assign11000_e5898_d_n2, assign11000_e5898_d_n4, assign11000_e5898_d_n5, assign11000_e5898_d_n6, assign11000_e5898_d_n7, assign11000_e5898_d_n8, assign11000_e5898_d_n9, assign11000_e5898_d_n10, assign11000_e5898_d_n11, assign11000_e5898_d_n14,) = {
    if (locals.var_guard259 != 0.0) {
        let assign11000_e5888: f64 = (locals.var_t1 * locals.var_t2);
        let assign11000_e5889: f64 = (1.0 + assign11000_e5888);
        let assign11000_e5890: f64 = (locals.var_mueph * assign11000_e5889);
        let assign11000_e5894: f64 = (locals.var_t1 * locals.var_t3);
        let assign11000_e5895: f64 = (1.0 + assign11000_e5894);
        let assign11000_e5896: f64 = (assign11000_e5890 / assign11000_e5895);
        (assign11000_e5896, (((((locals.var_mueph_dn0 * assign11000_e5889) + (locals.var_mueph * ((locals.var_t1_dn0 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn0)))) * assign11000_e5895) - (assign11000_e5890 * ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)))) / (assign11000_e5895 * assign11000_e5895)), (((((locals.var_mueph_dn2 * assign11000_e5889) + (locals.var_mueph * ((locals.var_t1_dn2 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn2)))) * assign11000_e5895) - (assign11000_e5890 * ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)))) / (assign11000_e5895 * assign11000_e5895)), (((((locals.var_mueph_dn4 * assign11000_e5889) + (locals.var_mueph * ((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)))) * assign11000_e5895) - (assign11000_e5890 * ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)))) / (assign11000_e5895 * assign11000_e5895)), (((((locals.var_mueph_dn5 * assign11000_e5889) + (locals.var_mueph * ((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)))) * assign11000_e5895) - (assign11000_e5890 * ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)))) / (assign11000_e5895 * assign11000_e5895)), (((((locals.var_mueph_dn6 * assign11000_e5889) + (locals.var_mueph * ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)))) * assign11000_e5895) - (assign11000_e5890 * ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)))) / (assign11000_e5895 * assign11000_e5895)), (((((locals.var_mueph_dn7 * assign11000_e5889) + (locals.var_mueph * ((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)))) * assign11000_e5895) - (assign11000_e5890 * ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)))) / (assign11000_e5895 * assign11000_e5895)), (((((locals.var_mueph_dn8 * assign11000_e5889) + (locals.var_mueph * ((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)))) * assign11000_e5895) - (assign11000_e5890 * ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)))) / (assign11000_e5895 * assign11000_e5895)), (((((locals.var_mueph_dn9 * assign11000_e5889) + (locals.var_mueph * ((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)))) * assign11000_e5895) - (assign11000_e5890 * ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)))) / (assign11000_e5895 * assign11000_e5895)), (((((locals.var_mueph_dn10 * assign11000_e5889) + (locals.var_mueph * ((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)))) * assign11000_e5895) - (assign11000_e5890 * ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)))) / (assign11000_e5895 * assign11000_e5895)), (((((locals.var_mueph_dn11 * assign11000_e5889) + (locals.var_mueph * ((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)))) * assign11000_e5895) - (assign11000_e5890 * ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)))) / (assign11000_e5895 * assign11000_e5895)), (((((locals.var_mueph_dn14 * assign11000_e5889) + (locals.var_mueph * ((locals.var_t1_dn14 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn14)))) * assign11000_e5895) - (assign11000_e5890 * ((locals.var_t1_dn14 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn14)))) / (assign11000_e5895 * assign11000_e5895)),)
    } else {
        (locals.var_mueph, locals.var_mueph_dn0, locals.var_mueph_dn2, locals.var_mueph_dn4, locals.var_mueph_dn5, locals.var_mueph_dn6, locals.var_mueph_dn7, locals.var_mueph_dn8, locals.var_mueph_dn9, locals.var_mueph_dn10, locals.var_mueph_dn11, locals.var_mueph_dn14,)
    }
};
        locals.var_mueph = assign11000_e5898;
        locals.var_mueph_dn0 = assign11000_e5898_d_n0;
        locals.var_mueph_dn2 = assign11000_e5898_d_n2;
        locals.var_mueph_dn4 = assign11000_e5898_d_n4;
        locals.var_mueph_dn5 = assign11000_e5898_d_n5;
        locals.var_mueph_dn6 = assign11000_e5898_d_n6;
        locals.var_mueph_dn7 = assign11000_e5898_d_n7;
        locals.var_mueph_dn8 = assign11000_e5898_d_n8;
        locals.var_mueph_dn9 = assign11000_e5898_d_n9;
        locals.var_mueph_dn10 = assign11000_e5898_d_n10;
        locals.var_mueph_dn11 = assign11000_e5898_d_n11;
        locals.var_mueph_dn14 = assign11000_e5898_d_n14;
        locals.var_mueph_rv = 0.0;

        let assign11010_e5904: f64 = (locals.var_lg).powf(p.p176);
        let assign11010_e5905: f64 = (p.p173 / assign11010_e5904);
        let assign11010_e5906: f64 = (1.0 + assign11010_e5905);
        let assign11010_e5907: f64 = (p.p171 * assign11010_e5906);
        let assign11010_e5912: f64 = (locals.var_wg).powf(p.p175);
        let assign11010_e5913: f64 = (p.p174 / assign11010_e5912);
        let assign11010_e5914: f64 = (1.0 + assign11010_e5913);
        let assign11010_e5915: f64 = (assign11010_e5907 * assign11010_e5914);
        locals.var_muesr = assign11010_e5915;
        locals.var_muesr_rv = 0.0;

        let (assign11040_e5939, assign11040_e5939_d_n0, assign11040_e5939_d_n2, assign11040_e5939_d_n4, assign11040_e5939_d_n5, assign11040_e5939_d_n6, assign11040_e5939_d_n7, assign11040_e5939_d_n8, assign11040_e5939_d_n9, assign11040_e5939_d_n10, assign11040_e5939_d_n11, assign11040_e5939_d_n14,) = {
    if (locals.var_mueph < 1e-25) {
        (1e-25, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_mueph, locals.var_mueph_dn0, locals.var_mueph_dn2, locals.var_mueph_dn4, locals.var_mueph_dn5, locals.var_mueph_dn6, locals.var_mueph_dn7, locals.var_mueph_dn8, locals.var_mueph_dn9, locals.var_mueph_dn10, locals.var_mueph_dn11, locals.var_mueph_dn14,)
    }
};
        locals.var_mueph = assign11040_e5939;
        locals.var_mueph_dn0 = assign11040_e5939_d_n0;
        locals.var_mueph_dn2 = assign11040_e5939_d_n2;
        locals.var_mueph_dn4 = assign11040_e5939_d_n4;
        locals.var_mueph_dn5 = assign11040_e5939_d_n5;
        locals.var_mueph_dn6 = assign11040_e5939_d_n6;
        locals.var_mueph_dn7 = assign11040_e5939_d_n7;
        locals.var_mueph_dn8 = assign11040_e5939_d_n8;
        locals.var_mueph_dn9 = assign11040_e5939_d_n9;
        locals.var_mueph_dn10 = assign11040_e5939_d_n10;
        locals.var_mueph_dn11 = assign11040_e5939_d_n11;
        locals.var_mueph_dn14 = assign11040_e5939_d_n14;
        locals.var_mueph_rv = 0.0;

        let (assign11050_e5945,) = {
    if (locals.var_muesr < 1e-25) {
        (1e-25,)
    } else {
        (locals.var_muesr,)
    }
};
        locals.var_muesr = assign11050_e5945;
        locals.var_muesr_rv = 0.0;

        let assign11060_e5948: f64 = (locals.var_lg).powf(p.p156);
        locals.var_t1 = assign11060_e5948;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign11070_e5951: f64 = (locals.var_uc_ndep * locals.var_t1);
        let assign11070_e5954: f64 = (locals.var_t1 + p.p155);
        let assign11070_e5955: f64 = (assign11070_e5951 / assign11070_e5954);
        let assign11070_e5957: f64 = (assign11070_e5955 / 1.034943e-10);
        locals.var_ndep_o_esi = assign11070_e5957;
        locals.var_ndep_o_esi_dn0 = (((((locals.var_uc_ndep * locals.var_t1_dn0) * assign11070_e5954) - (assign11070_e5951 * locals.var_t1_dn0)) / (assign11070_e5954 * assign11070_e5954)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn2 = (((((locals.var_uc_ndep * locals.var_t1_dn2) * assign11070_e5954) - (assign11070_e5951 * locals.var_t1_dn2)) / (assign11070_e5954 * assign11070_e5954)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn4 = (((((locals.var_uc_ndep * locals.var_t1_dn4) * assign11070_e5954) - (assign11070_e5951 * locals.var_t1_dn4)) / (assign11070_e5954 * assign11070_e5954)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn5 = (((((locals.var_uc_ndep * locals.var_t1_dn5) * assign11070_e5954) - (assign11070_e5951 * locals.var_t1_dn5)) / (assign11070_e5954 * assign11070_e5954)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn6 = (((((locals.var_uc_ndep * locals.var_t1_dn6) * assign11070_e5954) - (assign11070_e5951 * locals.var_t1_dn6)) / (assign11070_e5954 * assign11070_e5954)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn7 = (((((locals.var_uc_ndep * locals.var_t1_dn7) * assign11070_e5954) - (assign11070_e5951 * locals.var_t1_dn7)) / (assign11070_e5954 * assign11070_e5954)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn8 = (((((locals.var_uc_ndep * locals.var_t1_dn8) * assign11070_e5954) - (assign11070_e5951 * locals.var_t1_dn8)) / (assign11070_e5954 * assign11070_e5954)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn9 = (((((locals.var_uc_ndep * locals.var_t1_dn9) * assign11070_e5954) - (assign11070_e5951 * locals.var_t1_dn9)) / (assign11070_e5954 * assign11070_e5954)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn10 = (((((locals.var_uc_ndep * locals.var_t1_dn10) * assign11070_e5954) - (assign11070_e5951 * locals.var_t1_dn10)) / (assign11070_e5954 * assign11070_e5954)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn11 = (((((locals.var_uc_ndep * locals.var_t1_dn11) * assign11070_e5954) - (assign11070_e5951 * locals.var_t1_dn11)) / (assign11070_e5954 * assign11070_e5954)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn14 = (((((locals.var_uc_ndep * locals.var_t1_dn14) * assign11070_e5954) - (assign11070_e5951 * locals.var_t1_dn14)) / (assign11070_e5954 * assign11070_e5954)) / 1.034943e-10);
        locals.var_ndep_o_esi_rv = 0.0;

        let assign11080_e5960: f64 = (locals.var_uc_ninv / 1.034943e-10);
        locals.var_ninv_o_esi = assign11080_e5960;
        locals.var_ninv_o_esi_rv = 0.0;

        let assign11090_e5966: f64 = (locals.var_lg).powf(p.p321);
        let assign11090_e5967: f64 = (p.p320 / assign11090_e5966);
        let assign11090_e5968: f64 = (1.0 + assign11090_e5967);
        let assign11090_e5969: f64 = (p.p319 * assign11090_e5968);
        let assign11090_e5974: f64 = (locals.var_wg).powf(p.p323);
        let assign11090_e5975: f64 = (p.p322 / assign11090_e5974);
        let assign11090_e5976: f64 = (1.0 + assign11090_e5975);
        let assign11090_e5977: f64 = (assign11090_e5969 * assign11090_e5976);
        locals.var_ninvd0 = assign11090_e5977;
        locals.var_ninvd0_rv = 0.0;

        let assign11100_e5982: f64 = (locals.var_lg).powf(p.p387);
        let assign11100_e5983: f64 = (p.p386 / assign11100_e5982);
        let assign11100_e5984: f64 = (1.0 + assign11100_e5983);
        let assign11100_e5989: f64 = (locals.var_wg).powf(p.p389);
        let assign11100_e5990: f64 = (p.p388 / assign11100_e5989);
        let assign11100_e5991: f64 = (1.0 + assign11100_e5990);
        let assign11100_e5992: f64 = (assign11100_e5984 * assign11100_e5991);
        locals.var_t1 = assign11100_e5992;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign11110_e5995: f64 = (p.p384 * locals.var_t1);
        locals.var_ninvd0cres = assign11110_e5995;
        locals.var_ninvd0cres_dn0 = (p.p384 * locals.var_t1_dn0);
        locals.var_ninvd0cres_dn2 = (p.p384 * locals.var_t1_dn2);
        locals.var_ninvd0cres_dn4 = (p.p384 * locals.var_t1_dn4);
        locals.var_ninvd0cres_dn5 = (p.p384 * locals.var_t1_dn5);
        locals.var_ninvd0cres_dn6 = (p.p384 * locals.var_t1_dn6);
        locals.var_ninvd0cres_dn7 = (p.p384 * locals.var_t1_dn7);
        locals.var_ninvd0cres_dn8 = (p.p384 * locals.var_t1_dn8);
        locals.var_ninvd0cres_dn9 = (p.p384 * locals.var_t1_dn9);
        locals.var_ninvd0cres_dn10 = (p.p384 * locals.var_t1_dn10);
        locals.var_ninvd0cres_dn11 = (p.p384 * locals.var_t1_dn11);
        locals.var_ninvd0cres_dn14 = (p.p384 * locals.var_t1_dn14);
        locals.var_ninvd0cres_rv = 0.0;

        let assign11120_e5998: f64 = (p.p385 * locals.var_t1);
        locals.var_ninvd0hres = assign11120_e5998;
        locals.var_ninvd0hres_dn0 = (p.p385 * locals.var_t1_dn0);
        locals.var_ninvd0hres_dn2 = (p.p385 * locals.var_t1_dn2);
        locals.var_ninvd0hres_dn4 = (p.p385 * locals.var_t1_dn4);
        locals.var_ninvd0hres_dn5 = (p.p385 * locals.var_t1_dn5);
        locals.var_ninvd0hres_dn6 = (p.p385 * locals.var_t1_dn6);
        locals.var_ninvd0hres_dn7 = (p.p385 * locals.var_t1_dn7);
        locals.var_ninvd0hres_dn8 = (p.p385 * locals.var_t1_dn8);
        locals.var_ninvd0hres_dn9 = (p.p385 * locals.var_t1_dn9);
        locals.var_ninvd0hres_dn10 = (p.p385 * locals.var_t1_dn10);
        locals.var_ninvd0hres_dn11 = (p.p385 * locals.var_t1_dn11);
        locals.var_ninvd0hres_dn14 = (p.p385 * locals.var_t1_dn14);
        locals.var_ninvd0hres_rv = 0.0;

        let assign11130_e6003: f64 = (locals.var_lgate + p.p121);
        let assign11130_e6005: f64 = (assign11130_e6003).powf(p.p122);
        let assign11130_e6006: f64 = (locals.var_mks_ll / assign11130_e6005);
        let assign11130_e6007: f64 = (p.p97 + assign11130_e6006);
        locals.var_dl = assign11130_e6007;
        locals.var_dl_rv = 0.0;

        let assign11140_e6012: f64 = (locals.var_lgate + p.p121);
        let assign11140_e6014: f64 = (assign11140_e6012).powf(p.p122);
        let assign11140_e6015: f64 = (locals.var_mks_ll / assign11140_e6014);
        let assign11140_e6016: f64 = (locals.var_uc_xldld + assign11140_e6015);
        locals.var_dlld = assign11140_e6016;
        locals.var_dlld_rv = 0.0;

        let assign11150_e6021: f64 = (locals.var_wgate + p.p128);
        let assign11150_e6023: f64 = (assign11150_e6021).powf(p.p129);
        let assign11150_e6024: f64 = (locals.var_mks_wl / assign11150_e6023);
        let assign11150_e6025: f64 = (p.p114 + assign11150_e6024);
        locals.var_dw = assign11150_e6025;
        locals.var_dw_rv = 0.0;

        let assign11160_e6030: f64 = (locals.var_wgate + p.p128);
        let assign11160_e6032: f64 = (assign11160_e6030).powf(p.p129);
        let assign11160_e6033: f64 = (locals.var_mks_wl / assign11160_e6032);
        let assign11160_e6034: f64 = (p.p295 + assign11160_e6033);
        locals.var_dwld = assign11160_e6034;
        locals.var_dwld_rv = 0.0;

        let assign11170_e6039: f64 = (locals.var_wgate + p.p128);
        let assign11170_e6041: f64 = (assign11170_e6039).powf(p.p129);
        let assign11170_e6042: f64 = (locals.var_mks_wl / assign11170_e6041);
        let assign11170_e6043: f64 = (p.p115 + assign11170_e6042);
        locals.var_dwcv = assign11170_e6043;
        locals.var_dwcv_rv = 0.0;

        let assign11180_e6047: f64 = (locals.var_dl + locals.var_dlld);
        let assign11180_e6048: f64 = (locals.var_lgate - assign11180_e6047);
        locals.var_leff = assign11180_e6048;
        locals.var_leff_rv = 0.0;

        let assign11210_e6060: f64 = (locals.var_wlg).powf(p.p125);
        let assign11210_e6061: f64 = (p.p124 / assign11210_e6060);
        let assign11210_e6062: f64 = (locals.var_lgate + assign11210_e6061);
        locals.var_lgatesm = assign11210_e6062;
        locals.var_lgatesm_rv = 0.0;

        let assign11220_e6066: f64 = (locals.var_wlg).powf(p.p127);
        let assign11220_e6067: f64 = (locals.var_uc_wl2 / assign11220_e6066);
        locals.var_dvthsm = assign11220_e6067;
        locals.var_dvthsm_rv = 0.0;

        let assign11230_e6072: f64 = (locals.var_lgatesm * 1000000.0);
        let assign11230_e6074: f64 = (assign11230_e6072).powf(p.p207);
        let assign11230_e6075: f64 = (p.p206 / assign11230_e6074);
        let assign11230_e6076: f64 = (1.0 + assign11230_e6075);
        locals.var_t1 = assign11230_e6076;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign11240_e6081: f64 = (locals.var_wg).powf(p.p209);
        let assign11240_e6082: f64 = (p.p208 / assign11240_e6081);
        let assign11240_e6083: f64 = (1.0 + assign11240_e6082);
        locals.var_t2 = assign11240_e6083;
        locals.var_t2_dn0 = 0.0;
        locals.var_t2_dn2 = 0.0;
        locals.var_t2_dn4 = 0.0;
        locals.var_t2_dn5 = 0.0;
        locals.var_t2_dn6 = 0.0;
        locals.var_t2_dn7 = 0.0;
        locals.var_t2_dn8 = 0.0;
        locals.var_t2_dn9 = 0.0;
        locals.var_t2_dn10 = 0.0;
        locals.var_t2_dn11 = 0.0;
        locals.var_t2_dn14 = 0.0;
        locals.var_t2_rv = 0.0;

        let assign11250_e6086: f64 = (locals.var_uc_wsti * locals.var_t1);
        let assign11250_e6088: f64 = (assign11250_e6086 * locals.var_t2);
        locals.var_uc_wsti = assign11250_e6088;
        locals.var_uc_wsti_dn0 = ((((locals.var_uc_wsti_dn0 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn0)) * locals.var_t2) + (assign11250_e6086 * locals.var_t2_dn0));
        locals.var_uc_wsti_dn2 = ((((locals.var_uc_wsti_dn2 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn2)) * locals.var_t2) + (assign11250_e6086 * locals.var_t2_dn2));
        locals.var_uc_wsti_dn4 = ((((locals.var_uc_wsti_dn4 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn4)) * locals.var_t2) + (assign11250_e6086 * locals.var_t2_dn4));
        locals.var_uc_wsti_dn5 = ((((locals.var_uc_wsti_dn5 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn5)) * locals.var_t2) + (assign11250_e6086 * locals.var_t2_dn5));
        locals.var_uc_wsti_dn6 = ((((locals.var_uc_wsti_dn6 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn6)) * locals.var_t2) + (assign11250_e6086 * locals.var_t2_dn6));
        locals.var_uc_wsti_dn7 = ((((locals.var_uc_wsti_dn7 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn7)) * locals.var_t2) + (assign11250_e6086 * locals.var_t2_dn7));
        locals.var_uc_wsti_dn8 = ((((locals.var_uc_wsti_dn8 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn8)) * locals.var_t2) + (assign11250_e6086 * locals.var_t2_dn8));
        locals.var_uc_wsti_dn9 = ((((locals.var_uc_wsti_dn9 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn9)) * locals.var_t2) + (assign11250_e6086 * locals.var_t2_dn9));
        locals.var_uc_wsti_dn10 = ((((locals.var_uc_wsti_dn10 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn10)) * locals.var_t2) + (assign11250_e6086 * locals.var_t2_dn10));
        locals.var_uc_wsti_dn11 = ((((locals.var_uc_wsti_dn11 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn11)) * locals.var_t2) + (assign11250_e6086 * locals.var_t2_dn11));
        locals.var_uc_wsti_dn14 = ((((locals.var_uc_wsti_dn14 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn14)) * locals.var_t2) + (assign11250_e6086 * locals.var_t2_dn14));
        locals.var_uc_wsti_rv = 0.0;

        let assign11260_e6092: f64 = (2.0 * locals.var_dw);
        let assign11260_e6093: f64 = (locals.var_wgate - assign11260_e6092);
        locals.var_weff = assign11260_e6093;
        locals.var_weff_rv = 0.0;

        let assign11270_e6097: f64 = (2.0 * locals.var_dwld);
        let assign11270_e6098: f64 = (locals.var_wgate - assign11270_e6097);
        locals.var_weff_ld = assign11270_e6098;
        locals.var_weff_ld_rv = 0.0;

        let assign11280_e6102: f64 = (2.0 * locals.var_dwcv);
        let assign11280_e6103: f64 = (locals.var_wgate - assign11280_e6102);
        locals.var_weff_cv = assign11280_e6103;
        locals.var_weff_cv_rv = 0.0;

        let assign11350_e6127: f64 = (locals.var_weff * p.p7);
        locals.var_weff_nf = assign11350_e6127;
        locals.var_weff_nf_rv = 0.0;

        let assign11360_e6130: f64 = (locals.var_weff_cv * p.p7);
        locals.var_weffcv_nf = assign11360_e6130;
        locals.var_weffcv_nf_rv = 0.0;

        let assign11370_e6136: f64 = (locals.var_wg).powf(p.p143);
        let assign11370_e6137: f64 = (p.p142 / assign11370_e6136);
        let assign11370_e6138: f64 = (1.0 + assign11370_e6137);
        let assign11370_e6139: f64 = (locals.var_ef_nsubp * assign11370_e6138);
        locals.var_nsubpp = assign11370_e6139;
        locals.var_nsubpp_dn0 = (locals.var_ef_nsubp_dn0 * assign11370_e6138);
        locals.var_nsubpp_dn2 = (locals.var_ef_nsubp_dn2 * assign11370_e6138);
        locals.var_nsubpp_dn4 = (locals.var_ef_nsubp_dn4 * assign11370_e6138);
        locals.var_nsubpp_dn5 = (locals.var_ef_nsubp_dn5 * assign11370_e6138);
        locals.var_nsubpp_dn6 = (locals.var_ef_nsubp_dn6 * assign11370_e6138);
        locals.var_nsubpp_dn7 = (locals.var_ef_nsubp_dn7 * assign11370_e6138);
        locals.var_nsubpp_dn8 = (locals.var_ef_nsubp_dn8 * assign11370_e6138);
        locals.var_nsubpp_dn9 = (locals.var_ef_nsubp_dn9 * assign11370_e6138);
        locals.var_nsubpp_dn10 = (locals.var_ef_nsubp_dn10 * assign11370_e6138);
        locals.var_nsubpp_dn11 = (locals.var_ef_nsubp_dn11 * assign11370_e6138);
        locals.var_nsubpp_dn14 = (locals.var_ef_nsubp_dn14 * assign11370_e6138);
        locals.var_nsubpp_rv = 0.0;

        let assign11380_e6145: f64 = (locals.var_wg).powf(p.p234);
        let assign11380_e6146: f64 = (p.p233 / assign11380_e6145);
        let assign11380_e6147: f64 = (1.0 + assign11380_e6146);
        let assign11380_e6148: f64 = (locals.var_ef_nsubc * assign11380_e6147);
        locals.var_ef_nsubc = assign11380_e6148;
        locals.var_ef_nsubc_dn0 = (locals.var_ef_nsubc_dn0 * assign11380_e6147);
        locals.var_ef_nsubc_dn2 = (locals.var_ef_nsubc_dn2 * assign11380_e6147);
        locals.var_ef_nsubc_dn4 = (locals.var_ef_nsubc_dn4 * assign11380_e6147);
        locals.var_ef_nsubc_dn5 = (locals.var_ef_nsubc_dn5 * assign11380_e6147);
        locals.var_ef_nsubc_dn6 = (locals.var_ef_nsubc_dn6 * assign11380_e6147);
        locals.var_ef_nsubc_dn7 = (locals.var_ef_nsubc_dn7 * assign11380_e6147);
        locals.var_ef_nsubc_dn8 = (locals.var_ef_nsubc_dn8 * assign11380_e6147);
        locals.var_ef_nsubc_dn9 = (locals.var_ef_nsubc_dn9 * assign11380_e6147);
        locals.var_ef_nsubc_dn10 = (locals.var_ef_nsubc_dn10 * assign11380_e6147);
        locals.var_ef_nsubc_dn11 = (locals.var_ef_nsubc_dn11 * assign11380_e6147);
        locals.var_ef_nsubc_dn14 = (locals.var_ef_nsubc_dn14 * assign11380_e6147);
        locals.var_ef_nsubc_rv = 0.0;

        let assign11390_e6151: f64 = (locals.var_ef_nsubc * 1e-6);
        locals.var_t1 = assign11390_e6151;
        locals.var_t1_dn0 = (locals.var_ef_nsubc_dn0 * 1e-6);
        locals.var_t1_dn2 = (locals.var_ef_nsubc_dn2 * 1e-6);
        locals.var_t1_dn4 = (locals.var_ef_nsubc_dn4 * 1e-6);
        locals.var_t1_dn5 = (locals.var_ef_nsubc_dn5 * 1e-6);
        locals.var_t1_dn6 = (locals.var_ef_nsubc_dn6 * 1e-6);
        locals.var_t1_dn7 = (locals.var_ef_nsubc_dn7 * 1e-6);
        locals.var_t1_dn8 = (locals.var_ef_nsubc_dn8 * 1e-6);
        locals.var_t1_dn9 = (locals.var_ef_nsubc_dn9 * 1e-6);
        locals.var_t1_dn10 = (locals.var_ef_nsubc_dn10 * 1e-6);
        locals.var_t1_dn11 = (locals.var_ef_nsubc_dn11 * 1e-6);
        locals.var_t1_dn14 = (locals.var_ef_nsubc_dn14 * 1e-6);
        locals.var_t1_rv = 0.0;

        let assign11400_e6154: f64 = (locals.var_nsubpp * 1e-6);
        locals.var_t2 = assign11400_e6154;
        locals.var_t2_dn0 = (locals.var_nsubpp_dn0 * 1e-6);
        locals.var_t2_dn2 = (locals.var_nsubpp_dn2 * 1e-6);
        locals.var_t2_dn4 = (locals.var_nsubpp_dn4 * 1e-6);
        locals.var_t2_dn5 = (locals.var_nsubpp_dn5 * 1e-6);
        locals.var_t2_dn6 = (locals.var_nsubpp_dn6 * 1e-6);
        locals.var_t2_dn7 = (locals.var_nsubpp_dn7 * 1e-6);
        locals.var_t2_dn8 = (locals.var_nsubpp_dn8 * 1e-6);
        locals.var_t2_dn9 = (locals.var_nsubpp_dn9 * 1e-6);
        locals.var_t2_dn10 = (locals.var_nsubpp_dn10 * 1e-6);
        locals.var_t2_dn11 = (locals.var_nsubpp_dn11 * 1e-6);
        locals.var_t2_dn14 = (locals.var_nsubpp_dn14 * 1e-6);
        locals.var_t2_rv = 0.0;

        let assign11420_e6162: f64 = if locals.var_t1 < 1000000000000000.0 { 1.0 } else { 0.0 };
        locals.var_guard267 = assign11420_e6162;
        locals.var_guard267_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_19(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign11430_e6166, assign11430_e6166_d_n0, assign11430_e6166_d_n2, assign11430_e6166_d_n4, assign11430_e6166_d_n5, assign11430_e6166_d_n6, assign11430_e6166_d_n7, assign11430_e6166_d_n8, assign11430_e6166_d_n9, assign11430_e6166_d_n10, assign11430_e6166_d_n11, assign11430_e6166_d_n14,) = {
    if (locals.var_guard267 != 0.0) {
        (1000000000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign11430_e6166;
        locals.var_t1_dn0 = assign11430_e6166_d_n0;
        locals.var_t1_dn2 = assign11430_e6166_d_n2;
        locals.var_t1_dn4 = assign11430_e6166_d_n4;
        locals.var_t1_dn5 = assign11430_e6166_d_n5;
        locals.var_t1_dn6 = assign11430_e6166_d_n6;
        locals.var_t1_dn7 = assign11430_e6166_d_n7;
        locals.var_t1_dn8 = assign11430_e6166_d_n8;
        locals.var_t1_dn9 = assign11430_e6166_d_n9;
        locals.var_t1_dn10 = assign11430_e6166_d_n10;
        locals.var_t1_dn11 = assign11430_e6166_d_n11;
        locals.var_t1_dn14 = assign11430_e6166_d_n14;
        locals.var_t1_rv = 0.0;

        let assign11440_e6169: f64 = (locals.var_t1 / 1e-6);
        locals.var_ef_nsubc = assign11440_e6169;
        locals.var_ef_nsubc_dn0 = (locals.var_t1_dn0 / 1e-6);
        locals.var_ef_nsubc_dn2 = (locals.var_t1_dn2 / 1e-6);
        locals.var_ef_nsubc_dn4 = (locals.var_t1_dn4 / 1e-6);
        locals.var_ef_nsubc_dn5 = (locals.var_t1_dn5 / 1e-6);
        locals.var_ef_nsubc_dn6 = (locals.var_t1_dn6 / 1e-6);
        locals.var_ef_nsubc_dn7 = (locals.var_t1_dn7 / 1e-6);
        locals.var_ef_nsubc_dn8 = (locals.var_t1_dn8 / 1e-6);
        locals.var_ef_nsubc_dn9 = (locals.var_t1_dn9 / 1e-6);
        locals.var_ef_nsubc_dn10 = (locals.var_t1_dn10 / 1e-6);
        locals.var_ef_nsubc_dn11 = (locals.var_t1_dn11 / 1e-6);
        locals.var_ef_nsubc_dn14 = (locals.var_t1_dn14 / 1e-6);
        locals.var_ef_nsubc_rv = 0.0;

        let assign11460_e6177: f64 = if locals.var_t2 < 1000000000000000.0 { 1.0 } else { 0.0 };
        locals.var_guard269 = assign11460_e6177;
        locals.var_guard269_rv = 0.0;

        let (assign11470_e6181, assign11470_e6181_d_n0, assign11470_e6181_d_n2, assign11470_e6181_d_n4, assign11470_e6181_d_n5, assign11470_e6181_d_n6, assign11470_e6181_d_n7, assign11470_e6181_d_n8, assign11470_e6181_d_n9, assign11470_e6181_d_n10, assign11470_e6181_d_n11, assign11470_e6181_d_n14,) = {
    if (locals.var_guard269 != 0.0) {
        (1000000000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign11470_e6181;
        locals.var_t2_dn0 = assign11470_e6181_d_n0;
        locals.var_t2_dn2 = assign11470_e6181_d_n2;
        locals.var_t2_dn4 = assign11470_e6181_d_n4;
        locals.var_t2_dn5 = assign11470_e6181_d_n5;
        locals.var_t2_dn6 = assign11470_e6181_d_n6;
        locals.var_t2_dn7 = assign11470_e6181_d_n7;
        locals.var_t2_dn8 = assign11470_e6181_d_n8;
        locals.var_t2_dn9 = assign11470_e6181_d_n9;
        locals.var_t2_dn10 = assign11470_e6181_d_n10;
        locals.var_t2_dn11 = assign11470_e6181_d_n11;
        locals.var_t2_dn14 = assign11470_e6181_d_n14;
        locals.var_t2_rv = 0.0;

        let assign11480_e6184: f64 = (locals.var_t2 / 1e-6);
        locals.var_nsubpp = assign11480_e6184;
        locals.var_nsubpp_dn0 = (locals.var_t2_dn0 / 1e-6);
        locals.var_nsubpp_dn2 = (locals.var_t2_dn2 / 1e-6);
        locals.var_nsubpp_dn4 = (locals.var_t2_dn4 / 1e-6);
        locals.var_nsubpp_dn5 = (locals.var_t2_dn5 / 1e-6);
        locals.var_nsubpp_dn6 = (locals.var_t2_dn6 / 1e-6);
        locals.var_nsubpp_dn7 = (locals.var_t2_dn7 / 1e-6);
        locals.var_nsubpp_dn8 = (locals.var_t2_dn8 / 1e-6);
        locals.var_nsubpp_dn9 = (locals.var_t2_dn9 / 1e-6);
        locals.var_nsubpp_dn10 = (locals.var_t2_dn10 / 1e-6);
        locals.var_nsubpp_dn11 = (locals.var_t2_dn11 / 1e-6);
        locals.var_nsubpp_dn14 = (locals.var_t2_dn14 / 1e-6);
        locals.var_nsubpp_rv = 0.0;

        let assign11490_e6187: f64 = if locals.var_lod_half > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard270 = assign11490_e6187;
        locals.var_guard270_rv = 0.0;

        let (assign11500_e6195, assign11500_e6195_d_n0, assign11500_e6195_d_n2, assign11500_e6195_d_n4, assign11500_e6195_d_n5, assign11500_e6195_d_n6, assign11500_e6195_d_n7, assign11500_e6195_d_n8, assign11500_e6195_d_n9, assign11500_e6195_d_n10, assign11500_e6195_d_n11, assign11500_e6195_d_n14,) = {
    if (locals.var_guard270 != 0.0) {
        let assign11500_e6192: f64 = (1.0 + locals.var_uc_nsubpsti2);
        let assign11500_e6193: f64 = (1.0 / assign11500_e6192);
        (assign11500_e6193, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign11500_e6195;
        locals.var_t1_dn0 = assign11500_e6195_d_n0;
        locals.var_t1_dn2 = assign11500_e6195_d_n2;
        locals.var_t1_dn4 = assign11500_e6195_d_n4;
        locals.var_t1_dn5 = assign11500_e6195_d_n5;
        locals.var_t1_dn6 = assign11500_e6195_d_n6;
        locals.var_t1_dn7 = assign11500_e6195_d_n7;
        locals.var_t1_dn8 = assign11500_e6195_d_n8;
        locals.var_t1_dn9 = assign11500_e6195_d_n9;
        locals.var_t1_dn10 = assign11500_e6195_d_n10;
        locals.var_t1_dn11 = assign11500_e6195_d_n11;
        locals.var_t1_dn14 = assign11500_e6195_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign11510_e6203, assign11510_e6203_d_n0, assign11510_e6203_d_n2, assign11510_e6203_d_n4, assign11510_e6203_d_n5, assign11510_e6203_d_n6, assign11510_e6203_d_n7, assign11510_e6203_d_n8, assign11510_e6203_d_n9, assign11510_e6203_d_n10, assign11510_e6203_d_n11, assign11510_e6203_d_n14,) = {
    if (locals.var_guard270 != 0.0) {
        let assign11510_e6199: f64 = (locals.var_uc_nsubpsti1 / locals.var_lod_half);
        let assign11510_e6201: f64 = (assign11510_e6199).powf(locals.var_uc_nsubpsti3);
        (assign11510_e6201, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11510_e6199).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11510_e6201 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))) / assign11510_e6199))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11510_e6199).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11510_e6201 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))) / assign11510_e6199))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11510_e6199).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn4) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11510_e6201 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn4) / (locals.var_lod_half * locals.var_lod_half))) / assign11510_e6199))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11510_e6199).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn5) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11510_e6201 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn5) / (locals.var_lod_half * locals.var_lod_half))) / assign11510_e6199))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11510_e6199).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11510_e6201 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))) / assign11510_e6199))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11510_e6199).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn7) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11510_e6201 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn7) / (locals.var_lod_half * locals.var_lod_half))) / assign11510_e6199))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11510_e6199).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn8) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11510_e6201 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn8) / (locals.var_lod_half * locals.var_lod_half))) / assign11510_e6199))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11510_e6199).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn9) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11510_e6201 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn9) / (locals.var_lod_half * locals.var_lod_half))) / assign11510_e6199))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11510_e6199).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11510_e6201 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))) / assign11510_e6199))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11510_e6199).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn11) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11510_e6201 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn11) / (locals.var_lod_half * locals.var_lod_half))) / assign11510_e6199))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11510_e6199).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn14) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11510_e6201 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn14) / (locals.var_lod_half * locals.var_lod_half))) / assign11510_e6199))) },)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign11510_e6203;
        locals.var_t2_dn0 = assign11510_e6203_d_n0;
        locals.var_t2_dn2 = assign11510_e6203_d_n2;
        locals.var_t2_dn4 = assign11510_e6203_d_n4;
        locals.var_t2_dn5 = assign11510_e6203_d_n5;
        locals.var_t2_dn6 = assign11510_e6203_d_n6;
        locals.var_t2_dn7 = assign11510_e6203_d_n7;
        locals.var_t2_dn8 = assign11510_e6203_d_n8;
        locals.var_t2_dn9 = assign11510_e6203_d_n9;
        locals.var_t2_dn10 = assign11510_e6203_d_n10;
        locals.var_t2_dn11 = assign11510_e6203_d_n11;
        locals.var_t2_dn14 = assign11510_e6203_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign11520_e6211, assign11520_e6211_d_n0, assign11520_e6211_d_n2, assign11520_e6211_d_n4, assign11520_e6211_d_n5, assign11520_e6211_d_n6, assign11520_e6211_d_n7, assign11520_e6211_d_n8, assign11520_e6211_d_n9, assign11520_e6211_d_n10, assign11520_e6211_d_n11, assign11520_e6211_d_n14,) = {
    if (locals.var_guard270 != 0.0) {
        let assign11520_e6207: f64 = (locals.var_uc_nsubpsti1 / locals.var_lod_half_ref);
        let assign11520_e6209: f64 = (assign11520_e6207).powf(locals.var_uc_nsubpsti3);
        (assign11520_e6209, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11520_e6207).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn0) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11520_e6209 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn0) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11520_e6207))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11520_e6207).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn2) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11520_e6209 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn2) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11520_e6207))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11520_e6207).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn4) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11520_e6209 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn4) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11520_e6207))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11520_e6207).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn5) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11520_e6209 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn5) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11520_e6207))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11520_e6207).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn6) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11520_e6209 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn6) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11520_e6207))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11520_e6207).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn7) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11520_e6209 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn7) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11520_e6207))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11520_e6207).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn8) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11520_e6209 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn8) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11520_e6207))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11520_e6207).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn9) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11520_e6209 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn9) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11520_e6207))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11520_e6207).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn10) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11520_e6209 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn10) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11520_e6207))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11520_e6207).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn11) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11520_e6209 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn11) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11520_e6207))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11520_e6207).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn14) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11520_e6209 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn14) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11520_e6207))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign11520_e6211;
        locals.var_t3_dn0 = assign11520_e6211_d_n0;
        locals.var_t3_dn2 = assign11520_e6211_d_n2;
        locals.var_t3_dn4 = assign11520_e6211_d_n4;
        locals.var_t3_dn5 = assign11520_e6211_d_n5;
        locals.var_t3_dn6 = assign11520_e6211_d_n6;
        locals.var_t3_dn7 = assign11520_e6211_d_n7;
        locals.var_t3_dn8 = assign11520_e6211_d_n8;
        locals.var_t3_dn9 = assign11520_e6211_d_n9;
        locals.var_t3_dn10 = assign11520_e6211_d_n10;
        locals.var_t3_dn11 = assign11520_e6211_d_n11;
        locals.var_t3_dn14 = assign11520_e6211_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign11530_e6227, assign11530_e6227_d_n0, assign11530_e6227_d_n2, assign11530_e6227_d_n4, assign11530_e6227_d_n5, assign11530_e6227_d_n6, assign11530_e6227_d_n7, assign11530_e6227_d_n8, assign11530_e6227_d_n9, assign11530_e6227_d_n10, assign11530_e6227_d_n11, assign11530_e6227_d_n14,) = {
    if (locals.var_guard270 != 0.0) {
        let assign11530_e6217: f64 = (locals.var_t1 * locals.var_t2);
        let assign11530_e6218: f64 = (1.0 + assign11530_e6217);
        let assign11530_e6219: f64 = (locals.var_nsubpp * assign11530_e6218);
        let assign11530_e6223: f64 = (locals.var_t1 * locals.var_t3);
        let assign11530_e6224: f64 = (1.0 + assign11530_e6223);
        let assign11530_e6225: f64 = (assign11530_e6219 / assign11530_e6224);
        (assign11530_e6225, (((((locals.var_nsubpp_dn0 * assign11530_e6218) + (locals.var_nsubpp * ((locals.var_t1_dn0 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn0)))) * assign11530_e6224) - (assign11530_e6219 * ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)))) / (assign11530_e6224 * assign11530_e6224)), (((((locals.var_nsubpp_dn2 * assign11530_e6218) + (locals.var_nsubpp * ((locals.var_t1_dn2 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn2)))) * assign11530_e6224) - (assign11530_e6219 * ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)))) / (assign11530_e6224 * assign11530_e6224)), (((((locals.var_nsubpp_dn4 * assign11530_e6218) + (locals.var_nsubpp * ((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)))) * assign11530_e6224) - (assign11530_e6219 * ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)))) / (assign11530_e6224 * assign11530_e6224)), (((((locals.var_nsubpp_dn5 * assign11530_e6218) + (locals.var_nsubpp * ((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)))) * assign11530_e6224) - (assign11530_e6219 * ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)))) / (assign11530_e6224 * assign11530_e6224)), (((((locals.var_nsubpp_dn6 * assign11530_e6218) + (locals.var_nsubpp * ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)))) * assign11530_e6224) - (assign11530_e6219 * ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)))) / (assign11530_e6224 * assign11530_e6224)), (((((locals.var_nsubpp_dn7 * assign11530_e6218) + (locals.var_nsubpp * ((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)))) * assign11530_e6224) - (assign11530_e6219 * ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)))) / (assign11530_e6224 * assign11530_e6224)), (((((locals.var_nsubpp_dn8 * assign11530_e6218) + (locals.var_nsubpp * ((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)))) * assign11530_e6224) - (assign11530_e6219 * ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)))) / (assign11530_e6224 * assign11530_e6224)), (((((locals.var_nsubpp_dn9 * assign11530_e6218) + (locals.var_nsubpp * ((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)))) * assign11530_e6224) - (assign11530_e6219 * ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)))) / (assign11530_e6224 * assign11530_e6224)), (((((locals.var_nsubpp_dn10 * assign11530_e6218) + (locals.var_nsubpp * ((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)))) * assign11530_e6224) - (assign11530_e6219 * ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)))) / (assign11530_e6224 * assign11530_e6224)), (((((locals.var_nsubpp_dn11 * assign11530_e6218) + (locals.var_nsubpp * ((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)))) * assign11530_e6224) - (assign11530_e6219 * ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)))) / (assign11530_e6224 * assign11530_e6224)), (((((locals.var_nsubpp_dn14 * assign11530_e6218) + (locals.var_nsubpp * ((locals.var_t1_dn14 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn14)))) * assign11530_e6224) - (assign11530_e6219 * ((locals.var_t1_dn14 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn14)))) / (assign11530_e6224 * assign11530_e6224)),)
    } else {
        (locals.var_nsubps, locals.var_nsubps_dn0, locals.var_nsubps_dn2, locals.var_nsubps_dn4, locals.var_nsubps_dn5, locals.var_nsubps_dn6, locals.var_nsubps_dn7, locals.var_nsubps_dn8, locals.var_nsubps_dn9, locals.var_nsubps_dn10, locals.var_nsubps_dn11, locals.var_nsubps_dn14,)
    }
};
        locals.var_nsubps = assign11530_e6227;
        locals.var_nsubps_dn0 = assign11530_e6227_d_n0;
        locals.var_nsubps_dn2 = assign11530_e6227_d_n2;
        locals.var_nsubps_dn4 = assign11530_e6227_d_n4;
        locals.var_nsubps_dn5 = assign11530_e6227_d_n5;
        locals.var_nsubps_dn6 = assign11530_e6227_d_n6;
        locals.var_nsubps_dn7 = assign11530_e6227_d_n7;
        locals.var_nsubps_dn8 = assign11530_e6227_d_n8;
        locals.var_nsubps_dn9 = assign11530_e6227_d_n9;
        locals.var_nsubps_dn10 = assign11530_e6227_d_n10;
        locals.var_nsubps_dn11 = assign11530_e6227_d_n11;
        locals.var_nsubps_dn14 = assign11530_e6227_d_n14;
        locals.var_nsubps_rv = 0.0;

        let (assign11540_e6232, assign11540_e6232_d_n0, assign11540_e6232_d_n2, assign11540_e6232_d_n4, assign11540_e6232_d_n5, assign11540_e6232_d_n6, assign11540_e6232_d_n7, assign11540_e6232_d_n8, assign11540_e6232_d_n9, assign11540_e6232_d_n10, assign11540_e6232_d_n11, assign11540_e6232_d_n14,) = {
    if (locals.var_guard270 == 0.0) {
        (locals.var_nsubpp, locals.var_nsubpp_dn0, locals.var_nsubpp_dn2, locals.var_nsubpp_dn4, locals.var_nsubpp_dn5, locals.var_nsubpp_dn6, locals.var_nsubpp_dn7, locals.var_nsubpp_dn8, locals.var_nsubpp_dn9, locals.var_nsubpp_dn10, locals.var_nsubpp_dn11, locals.var_nsubpp_dn14,)
    } else {
        (locals.var_nsubps, locals.var_nsubps_dn0, locals.var_nsubps_dn2, locals.var_nsubps_dn4, locals.var_nsubps_dn5, locals.var_nsubps_dn6, locals.var_nsubps_dn7, locals.var_nsubps_dn8, locals.var_nsubps_dn9, locals.var_nsubps_dn10, locals.var_nsubps_dn11, locals.var_nsubps_dn14,)
    }
};
        locals.var_nsubps = assign11540_e6232;
        locals.var_nsubps_dn0 = assign11540_e6232_d_n0;
        locals.var_nsubps_dn2 = assign11540_e6232_d_n2;
        locals.var_nsubps_dn4 = assign11540_e6232_d_n4;
        locals.var_nsubps_dn5 = assign11540_e6232_d_n5;
        locals.var_nsubps_dn6 = assign11540_e6232_d_n6;
        locals.var_nsubps_dn7 = assign11540_e6232_d_n7;
        locals.var_nsubps_dn8 = assign11540_e6232_d_n8;
        locals.var_nsubps_dn9 = assign11540_e6232_d_n9;
        locals.var_nsubps_dn10 = assign11540_e6232_d_n10;
        locals.var_nsubps_dn11 = assign11540_e6232_d_n11;
        locals.var_nsubps_dn14 = assign11540_e6232_d_n14;
        locals.var_nsubps_rv = 0.0;

        let assign11550_e6239: f64 = if ((locals.var_lgate > p.p140) || (p.p140 <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard271 = assign11550_e6239;
        locals.var_guard271_rv = 0.0;

        let (assign11560_e6253, assign11560_e6253_d_n0, assign11560_e6253_d_n2, assign11560_e6253_d_n4, assign11560_e6253_d_n5, assign11560_e6253_d_n6, assign11560_e6253_d_n7, assign11560_e6253_d_n8, assign11560_e6253_d_n9, assign11560_e6253_d_n10, assign11560_e6253_d_n11, assign11560_e6253_d_n14,) = {
    if (locals.var_guard271 != 0.0) {
        let assign11560_e6244: f64 = (locals.var_lgate - p.p140);
        let assign11560_e6245: f64 = (locals.var_ef_nsubc * assign11560_e6244);
        let assign11560_e6248: f64 = (locals.var_nsubps * p.p140);
        let assign11560_e6249: f64 = (assign11560_e6245 + assign11560_e6248);
        let assign11560_e6251: f64 = (assign11560_e6249 / locals.var_lgate);
        (assign11560_e6251, (((locals.var_ef_nsubc_dn0 * assign11560_e6244) + (locals.var_nsubps_dn0 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn2 * assign11560_e6244) + (locals.var_nsubps_dn2 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn4 * assign11560_e6244) + (locals.var_nsubps_dn4 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn5 * assign11560_e6244) + (locals.var_nsubps_dn5 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn6 * assign11560_e6244) + (locals.var_nsubps_dn6 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn7 * assign11560_e6244) + (locals.var_nsubps_dn7 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn8 * assign11560_e6244) + (locals.var_nsubps_dn8 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn9 * assign11560_e6244) + (locals.var_nsubps_dn9 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn10 * assign11560_e6244) + (locals.var_nsubps_dn10 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn11 * assign11560_e6244) + (locals.var_nsubps_dn11 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn14 * assign11560_e6244) + (locals.var_nsubps_dn14 * p.p140)) / locals.var_lgate),)
    } else {
        (locals.var_nsub, locals.var_nsub_dn0, locals.var_nsub_dn2, locals.var_nsub_dn4, locals.var_nsub_dn5, locals.var_nsub_dn6, locals.var_nsub_dn7, locals.var_nsub_dn8, locals.var_nsub_dn9, locals.var_nsub_dn10, locals.var_nsub_dn11, locals.var_nsub_dn14,)
    }
};
        locals.var_nsub = assign11560_e6253;
        locals.var_nsub_dn0 = assign11560_e6253_d_n0;
        locals.var_nsub_dn2 = assign11560_e6253_d_n2;
        locals.var_nsub_dn4 = assign11560_e6253_d_n4;
        locals.var_nsub_dn5 = assign11560_e6253_d_n5;
        locals.var_nsub_dn6 = assign11560_e6253_d_n6;
        locals.var_nsub_dn7 = assign11560_e6253_d_n7;
        locals.var_nsub_dn8 = assign11560_e6253_d_n8;
        locals.var_nsub_dn9 = assign11560_e6253_d_n9;
        locals.var_nsub_dn10 = assign11560_e6253_d_n10;
        locals.var_nsub_dn11 = assign11560_e6253_d_n11;
        locals.var_nsub_dn14 = assign11560_e6253_d_n14;
        locals.var_nsub_rv = 0.0;

        let (assign11570_e6268, assign11570_e6268_d_n0, assign11570_e6268_d_n2, assign11570_e6268_d_n4, assign11570_e6268_d_n5, assign11570_e6268_d_n6, assign11570_e6268_d_n7, assign11570_e6268_d_n8, assign11570_e6268_d_n9, assign11570_e6268_d_n10, assign11570_e6268_d_n11, assign11570_e6268_d_n14,) = {
    if (locals.var_guard271 == 0.0) {
        let assign11570_e6259: f64 = (locals.var_nsubps - locals.var_ef_nsubc);
        let assign11570_e6262: f64 = (p.p140 - locals.var_lgate);
        let assign11570_e6263: f64 = (assign11570_e6259 * assign11570_e6262);
        let assign11570_e6265: f64 = (assign11570_e6263 / p.p140);
        let assign11570_e6266: f64 = (locals.var_nsubps + assign11570_e6265);
        (assign11570_e6266, (locals.var_nsubps_dn0 + (((locals.var_nsubps_dn0 - locals.var_ef_nsubc_dn0) * assign11570_e6262) / p.p140)), (locals.var_nsubps_dn2 + (((locals.var_nsubps_dn2 - locals.var_ef_nsubc_dn2) * assign11570_e6262) / p.p140)), (locals.var_nsubps_dn4 + (((locals.var_nsubps_dn4 - locals.var_ef_nsubc_dn4) * assign11570_e6262) / p.p140)), (locals.var_nsubps_dn5 + (((locals.var_nsubps_dn5 - locals.var_ef_nsubc_dn5) * assign11570_e6262) / p.p140)), (locals.var_nsubps_dn6 + (((locals.var_nsubps_dn6 - locals.var_ef_nsubc_dn6) * assign11570_e6262) / p.p140)), (locals.var_nsubps_dn7 + (((locals.var_nsubps_dn7 - locals.var_ef_nsubc_dn7) * assign11570_e6262) / p.p140)), (locals.var_nsubps_dn8 + (((locals.var_nsubps_dn8 - locals.var_ef_nsubc_dn8) * assign11570_e6262) / p.p140)), (locals.var_nsubps_dn9 + (((locals.var_nsubps_dn9 - locals.var_ef_nsubc_dn9) * assign11570_e6262) / p.p140)), (locals.var_nsubps_dn10 + (((locals.var_nsubps_dn10 - locals.var_ef_nsubc_dn10) * assign11570_e6262) / p.p140)), (locals.var_nsubps_dn11 + (((locals.var_nsubps_dn11 - locals.var_ef_nsubc_dn11) * assign11570_e6262) / p.p140)), (locals.var_nsubps_dn14 + (((locals.var_nsubps_dn14 - locals.var_ef_nsubc_dn14) * assign11570_e6262) / p.p140)),)
    } else {
        (locals.var_nsub, locals.var_nsub_dn0, locals.var_nsub_dn2, locals.var_nsub_dn4, locals.var_nsub_dn5, locals.var_nsub_dn6, locals.var_nsub_dn7, locals.var_nsub_dn8, locals.var_nsub_dn9, locals.var_nsub_dn10, locals.var_nsub_dn11, locals.var_nsub_dn14,)
    }
};
        locals.var_nsub = assign11570_e6268;
        locals.var_nsub_dn0 = assign11570_e6268_d_n0;
        locals.var_nsub_dn2 = assign11570_e6268_d_n2;
        locals.var_nsub_dn4 = assign11570_e6268_d_n4;
        locals.var_nsub_dn5 = assign11570_e6268_d_n5;
        locals.var_nsub_dn6 = assign11570_e6268_d_n6;
        locals.var_nsub_dn7 = assign11570_e6268_d_n7;
        locals.var_nsub_dn8 = assign11570_e6268_d_n8;
        locals.var_nsub_dn9 = assign11570_e6268_d_n9;
        locals.var_nsub_dn10 = assign11570_e6268_d_n10;
        locals.var_nsub_dn11 = assign11570_e6268_d_n11;
        locals.var_nsub_dn14 = assign11570_e6268_d_n14;
        locals.var_nsub_rv = 0.0;

        let assign11580_e6271: f64 = (0.5 * locals.var_lgate);
        let assign11580_e6273: f64 = (assign11580_e6271 - p.p140);
        locals.var_t3 = assign11580_e6273;
        locals.var_t3_dn0 = 0.0;
        locals.var_t3_dn2 = 0.0;
        locals.var_t3_dn4 = 0.0;
        locals.var_t3_dn5 = 0.0;
        locals.var_t3_dn6 = 0.0;
        locals.var_t3_dn7 = 0.0;
        locals.var_t3_dn8 = 0.0;
        locals.var_t3_dn9 = 0.0;
        locals.var_t3_dn10 = 0.0;
        locals.var_t3_dn11 = 0.0;
        locals.var_t3_dn14 = 0.0;
        locals.var_t3_rv = 0.0;

        let assign11590_e6276: f64 = (locals.var_t3 - 1e-9);
        let assign11590_e6278: f64 = (assign11590_e6276 - 1e-10);
        locals.var_tmf1 = assign11590_e6278;
        locals.var_tmf1_dn0 = locals.var_t3_dn0;
        locals.var_tmf1_dn2 = locals.var_t3_dn2;
        locals.var_tmf1_dn4 = locals.var_t3_dn4;
        locals.var_tmf1_dn5 = locals.var_t3_dn5;
        locals.var_tmf1_dn6 = locals.var_t3_dn6;
        locals.var_tmf1_dn7 = locals.var_t3_dn7;
        locals.var_tmf1_dn8 = locals.var_t3_dn8;
        locals.var_tmf1_dn9 = locals.var_t3_dn9;
        locals.var_tmf1_dn10 = locals.var_t3_dn10;
        locals.var_tmf1_dn11 = locals.var_t3_dn11;
        locals.var_tmf1_dn14 = locals.var_t3_dn14;
        locals.var_tmf1_rv = 0.0;

        let assign11600_e6281: f64 = (4.0 * 1e-9);
        let assign11600_e6283: f64 = (assign11600_e6281 * 1e-10);
        locals.var_tmf2 = assign11600_e6283;
        locals.var_tmf2_dn0 = 0.0;
        locals.var_tmf2_dn2 = 0.0;
        locals.var_tmf2_dn4 = 0.0;
        locals.var_tmf2_dn5 = 0.0;
        locals.var_tmf2_dn6 = 0.0;
        locals.var_tmf2_dn7 = 0.0;
        locals.var_tmf2_dn8 = 0.0;
        locals.var_tmf2_dn9 = 0.0;
        locals.var_tmf2_dn10 = 0.0;
        locals.var_tmf2_dn11 = 0.0;
        locals.var_tmf2_dn14 = 0.0;
        locals.var_tmf2_rv = 0.0;

        let (assign11610_e6290, assign11610_e6290_d_n0, assign11610_e6290_d_n2, assign11610_e6290_d_n4, assign11610_e6290_d_n5, assign11610_e6290_d_n6, assign11610_e6290_d_n7, assign11610_e6290_d_n8, assign11610_e6290_d_n9, assign11610_e6290_d_n10, assign11610_e6290_d_n11, assign11610_e6290_d_n14,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    } else {
        let assign11610_e6289: f64 = (-locals.var_tmf2);
        (assign11610_e6289, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
    }
};
        locals.var_tmf2 = assign11610_e6290;
        locals.var_tmf2_dn0 = assign11610_e6290_d_n0;
        locals.var_tmf2_dn2 = assign11610_e6290_d_n2;
        locals.var_tmf2_dn4 = assign11610_e6290_d_n4;
        locals.var_tmf2_dn5 = assign11610_e6290_d_n5;
        locals.var_tmf2_dn6 = assign11610_e6290_d_n6;
        locals.var_tmf2_dn7 = assign11610_e6290_d_n7;
        locals.var_tmf2_dn8 = assign11610_e6290_d_n8;
        locals.var_tmf2_dn9 = assign11610_e6290_d_n9;
        locals.var_tmf2_dn10 = assign11610_e6290_d_n10;
        locals.var_tmf2_dn11 = assign11610_e6290_d_n11;
        locals.var_tmf2_dn14 = assign11610_e6290_d_n14;
        locals.var_tmf2_rv = 0.0;

        let assign11620_e6293: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign11620_e6295: f64 = (assign11620_e6293 + locals.var_tmf2);
        let assign11620_e6296: f64 = (assign11620_e6295).sqrt();
        locals.var_tmf2 = assign11620_e6296;
        locals.var_tmf2_dn0 = ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign11620_e6296));
        locals.var_tmf2_dn2 = ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign11620_e6296));
        locals.var_tmf2_dn4 = ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign11620_e6296));
        locals.var_tmf2_dn5 = ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign11620_e6296));
        locals.var_tmf2_dn6 = ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign11620_e6296));
        locals.var_tmf2_dn7 = ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign11620_e6296));
        locals.var_tmf2_dn8 = ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign11620_e6296));
        locals.var_tmf2_dn9 = ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign11620_e6296));
        locals.var_tmf2_dn10 = ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign11620_e6296));
        locals.var_tmf2_dn11 = ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign11620_e6296));
        locals.var_tmf2_dn14 = ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign11620_e6296));
        locals.var_tmf2_rv = 0.0;

        let assign11630_e6301: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign11630_e6302: f64 = (1.0 + assign11630_e6301);
        let assign11630_e6303: f64 = (0.5 * assign11630_e6302);
        locals.var_t0 = assign11630_e6303;
        locals.var_t0_dn0 = (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn2 = (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn4 = (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn5 = (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn6 = (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn7 = (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn8 = (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn9 = (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn10 = (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn11 = (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn14 = (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_rv = 0.0;

        let assign11640_e6308: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign11640_e6309: f64 = (0.5 * assign11640_e6308);
        let assign11640_e6310: f64 = (1e-9 + assign11640_e6309);
        locals.var_t3 = assign11640_e6310;
        locals.var_t3_dn0 = (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0));
        locals.var_t3_dn2 = (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2));
        locals.var_t3_dn4 = (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4));
        locals.var_t3_dn5 = (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5));
        locals.var_t3_dn6 = (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6));
        locals.var_t3_dn7 = (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7));
        locals.var_t3_dn8 = (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8));
        locals.var_t3_dn9 = (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9));
        locals.var_t3_dn10 = (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10));
        locals.var_t3_dn11 = (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11));
        locals.var_t3_dn14 = (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14));
        locals.var_t3_rv = 0.0;

        let assign11650_e6314: f64 = (1.0 / locals.var_t3);
        let assign11650_e6317: f64 = (1.0 / p.p220);
        let assign11650_e6318: f64 = (assign11650_e6314 + assign11650_e6317);
        let assign11650_e6319: f64 = (1.0 / assign11650_e6318);
        locals.var_t1 = assign11650_e6319;
        locals.var_t1_dn0 = (-((-(locals.var_t3_dn0 / (locals.var_t3 * locals.var_t3))) / (assign11650_e6318 * assign11650_e6318)));
        locals.var_t1_dn2 = (-((-(locals.var_t3_dn2 / (locals.var_t3 * locals.var_t3))) / (assign11650_e6318 * assign11650_e6318)));
        locals.var_t1_dn4 = (-((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) / (assign11650_e6318 * assign11650_e6318)));
        locals.var_t1_dn5 = (-((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) / (assign11650_e6318 * assign11650_e6318)));
        locals.var_t1_dn6 = (-((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) / (assign11650_e6318 * assign11650_e6318)));
        locals.var_t1_dn7 = (-((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) / (assign11650_e6318 * assign11650_e6318)));
        locals.var_t1_dn8 = (-((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) / (assign11650_e6318 * assign11650_e6318)));
        locals.var_t1_dn9 = (-((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) / (assign11650_e6318 * assign11650_e6318)));
        locals.var_t1_dn10 = (-((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) / (assign11650_e6318 * assign11650_e6318)));
        locals.var_t1_dn11 = (-((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) / (assign11650_e6318 * assign11650_e6318)));
        locals.var_t1_dn14 = (-((-(locals.var_t3_dn14 / (locals.var_t3 * locals.var_t3))) / (assign11650_e6318 * assign11650_e6318)));
        locals.var_t1_rv = 0.0;

        let (assign11660_e6325, assign11660_e6325_d_n0, assign11660_e6325_d_n2, assign11660_e6325_d_n4, assign11660_e6325_d_n5, assign11660_e6325_d_n6, assign11660_e6325_d_n7, assign11660_e6325_d_n8, assign11660_e6325_d_n9, assign11660_e6325_d_n10, assign11660_e6325_d_n11, assign11660_e6325_d_n14,) = {
    if (0.0 >= locals.var_t1) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t2 = assign11660_e6325;
        locals.var_t2_dn0 = assign11660_e6325_d_n0;
        locals.var_t2_dn2 = assign11660_e6325_d_n2;
        locals.var_t2_dn4 = assign11660_e6325_d_n4;
        locals.var_t2_dn5 = assign11660_e6325_d_n5;
        locals.var_t2_dn6 = assign11660_e6325_d_n6;
        locals.var_t2_dn7 = assign11660_e6325_d_n7;
        locals.var_t2_dn8 = assign11660_e6325_d_n8;
        locals.var_t2_dn9 = assign11660_e6325_d_n9;
        locals.var_t2_dn10 = assign11660_e6325_d_n10;
        locals.var_t2_dn11 = assign11660_e6325_d_n11;
        locals.var_t2_dn14 = assign11660_e6325_d_n14;
        locals.var_t2_rv = 0.0;

        let assign11670_e6330: f64 = (locals.var_npexte - locals.var_ef_nsubc);
        let assign11670_e6331: f64 = (locals.var_t2 * assign11670_e6330);
        let assign11670_e6333: f64 = (assign11670_e6331 / locals.var_lgate);
        let assign11670_e6334: f64 = (locals.var_nsub + assign11670_e6333);
        locals.var_nsub = assign11670_e6334;
        locals.var_nsub_dn0 = (locals.var_nsub_dn0 + (((locals.var_t2_dn0 * assign11670_e6330) + (locals.var_t2 * (locals.var_npexte_dn0 - locals.var_ef_nsubc_dn0))) / locals.var_lgate));
        locals.var_nsub_dn2 = (locals.var_nsub_dn2 + (((locals.var_t2_dn2 * assign11670_e6330) + (locals.var_t2 * (locals.var_npexte_dn2 - locals.var_ef_nsubc_dn2))) / locals.var_lgate));
        locals.var_nsub_dn4 = (locals.var_nsub_dn4 + (((locals.var_t2_dn4 * assign11670_e6330) + (locals.var_t2 * (locals.var_npexte_dn4 - locals.var_ef_nsubc_dn4))) / locals.var_lgate));
        locals.var_nsub_dn5 = (locals.var_nsub_dn5 + (((locals.var_t2_dn5 * assign11670_e6330) + (locals.var_t2 * (locals.var_npexte_dn5 - locals.var_ef_nsubc_dn5))) / locals.var_lgate));
        locals.var_nsub_dn6 = (locals.var_nsub_dn6 + (((locals.var_t2_dn6 * assign11670_e6330) + (locals.var_t2 * (locals.var_npexte_dn6 - locals.var_ef_nsubc_dn6))) / locals.var_lgate));
        locals.var_nsub_dn7 = (locals.var_nsub_dn7 + (((locals.var_t2_dn7 * assign11670_e6330) + (locals.var_t2 * (locals.var_npexte_dn7 - locals.var_ef_nsubc_dn7))) / locals.var_lgate));
        locals.var_nsub_dn8 = (locals.var_nsub_dn8 + (((locals.var_t2_dn8 * assign11670_e6330) + (locals.var_t2 * (locals.var_npexte_dn8 - locals.var_ef_nsubc_dn8))) / locals.var_lgate));
        locals.var_nsub_dn9 = (locals.var_nsub_dn9 + (((locals.var_t2_dn9 * assign11670_e6330) + (locals.var_t2 * (locals.var_npexte_dn9 - locals.var_ef_nsubc_dn9))) / locals.var_lgate));
        locals.var_nsub_dn10 = (locals.var_nsub_dn10 + (((locals.var_t2_dn10 * assign11670_e6330) + (locals.var_t2 * (locals.var_npexte_dn10 - locals.var_ef_nsubc_dn10))) / locals.var_lgate));
        locals.var_nsub_dn11 = (locals.var_nsub_dn11 + (((locals.var_t2_dn11 * assign11670_e6330) + (locals.var_t2 * (locals.var_npexte_dn11 - locals.var_ef_nsubc_dn11))) / locals.var_lgate));
        locals.var_nsub_dn14 = (locals.var_nsub_dn14 + (((locals.var_t2_dn14 * assign11670_e6330) + (locals.var_t2 * (locals.var_npexte_dn14 - locals.var_ef_nsubc_dn14))) / locals.var_lgate));
        locals.var_nsub_rv = 0.0;

        let assign11680_e6337: f64 = (1.6021918e-19 * locals.var_nsub);
        locals.var_q_nsub = assign11680_e6337;
        locals.var_q_nsub_dn0 = (1.6021918e-19 * locals.var_nsub_dn0);
        locals.var_q_nsub_dn2 = (1.6021918e-19 * locals.var_nsub_dn2);
        locals.var_q_nsub_dn4 = (1.6021918e-19 * locals.var_nsub_dn4);
        locals.var_q_nsub_dn5 = (1.6021918e-19 * locals.var_nsub_dn5);
        locals.var_q_nsub_dn6 = (1.6021918e-19 * locals.var_nsub_dn6);
        locals.var_q_nsub_dn7 = (1.6021918e-19 * locals.var_nsub_dn7);
        locals.var_q_nsub_dn8 = (1.6021918e-19 * locals.var_nsub_dn8);
        locals.var_q_nsub_dn9 = (1.6021918e-19 * locals.var_nsub_dn9);
        locals.var_q_nsub_dn10 = (1.6021918e-19 * locals.var_nsub_dn10);
        locals.var_q_nsub_dn11 = (1.6021918e-19 * locals.var_nsub_dn11);
        locals.var_q_nsub_dn14 = (1.6021918e-19 * locals.var_nsub_dn14);
        locals.var_q_nsub_rv = 0.0;

        let assign11690_e6340: f64 = (locals.var_q_nsub * 1.034943e-10);
        locals.var_qnsub_esi = assign11690_e6340;
        locals.var_qnsub_esi_dn0 = (locals.var_q_nsub_dn0 * 1.034943e-10);
        locals.var_qnsub_esi_dn2 = (locals.var_q_nsub_dn2 * 1.034943e-10);
        locals.var_qnsub_esi_dn4 = (locals.var_q_nsub_dn4 * 1.034943e-10);
        locals.var_qnsub_esi_dn5 = (locals.var_q_nsub_dn5 * 1.034943e-10);
        locals.var_qnsub_esi_dn6 = (locals.var_q_nsub_dn6 * 1.034943e-10);
        locals.var_qnsub_esi_dn7 = (locals.var_q_nsub_dn7 * 1.034943e-10);
        locals.var_qnsub_esi_dn8 = (locals.var_q_nsub_dn8 * 1.034943e-10);
        locals.var_qnsub_esi_dn9 = (locals.var_q_nsub_dn9 * 1.034943e-10);
        locals.var_qnsub_esi_dn10 = (locals.var_q_nsub_dn10 * 1.034943e-10);
        locals.var_qnsub_esi_dn11 = (locals.var_q_nsub_dn11 * 1.034943e-10);
        locals.var_qnsub_esi_dn14 = (locals.var_q_nsub_dn14 * 1.034943e-10);
        locals.var_qnsub_esi_rv = 0.0;

        let assign11700_e6343: f64 = (2.0 * locals.var_qnsub_esi);
        locals.var_qnsub_esi2 = assign11700_e6343;
        locals.var_qnsub_esi2_dn0 = (2.0 * locals.var_qnsub_esi_dn0);
        locals.var_qnsub_esi2_dn2 = (2.0 * locals.var_qnsub_esi_dn2);
        locals.var_qnsub_esi2_dn4 = (2.0 * locals.var_qnsub_esi_dn4);
        locals.var_qnsub_esi2_dn5 = (2.0 * locals.var_qnsub_esi_dn5);
        locals.var_qnsub_esi2_dn6 = (2.0 * locals.var_qnsub_esi_dn6);
        locals.var_qnsub_esi2_dn7 = (2.0 * locals.var_qnsub_esi_dn7);
        locals.var_qnsub_esi2_dn8 = (2.0 * locals.var_qnsub_esi_dn8);
        locals.var_qnsub_esi2_dn9 = (2.0 * locals.var_qnsub_esi_dn9);
        locals.var_qnsub_esi2_dn10 = (2.0 * locals.var_qnsub_esi_dn10);
        locals.var_qnsub_esi2_dn11 = (2.0 * locals.var_qnsub_esi_dn11);
        locals.var_qnsub_esi2_dn14 = (2.0 * locals.var_qnsub_esi_dn14);
        locals.var_qnsub_esi2_rv = 0.0;

        let assign11710_e6347: f64 = (2.0 * p.p140);
        let assign11710_e6352: f64 = if ((locals.var_lgate <= assign11710_e6347) && (p.p140 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard272 = assign11710_e6352;
        locals.var_guard272_rv = 0.0;

        let (assign11720_e6368, assign11720_e6368_d_n0, assign11720_e6368_d_n2, assign11720_e6368_d_n4, assign11720_e6368_d_n5, assign11720_e6368_d_n6, assign11720_e6368_d_n7, assign11720_e6368_d_n8, assign11720_e6368_d_n9, assign11720_e6368_d_n10, assign11720_e6368_d_n11, assign11720_e6368_d_n14,) = {
    if (locals.var_guard272 != 0.0) {
        let assign11720_e6356: f64 = (2.0 * locals.var_nsubps);
        let assign11720_e6359: f64 = (locals.var_nsubps - locals.var_ef_nsubc);
        let assign11720_e6361: f64 = (assign11720_e6359 * locals.var_lgate);
        let assign11720_e6363: f64 = (assign11720_e6361 / p.p140);
        let assign11720_e6364: f64 = (assign11720_e6356 - assign11720_e6363);
        let assign11720_e6366: f64 = (assign11720_e6364 - locals.var_ef_nsubc);
        (assign11720_e6366, (((2.0 * locals.var_nsubps_dn0) - (((locals.var_nsubps_dn0 - locals.var_ef_nsubc_dn0) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn0), (((2.0 * locals.var_nsubps_dn2) - (((locals.var_nsubps_dn2 - locals.var_ef_nsubc_dn2) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn2), (((2.0 * locals.var_nsubps_dn4) - (((locals.var_nsubps_dn4 - locals.var_ef_nsubc_dn4) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn4), (((2.0 * locals.var_nsubps_dn5) - (((locals.var_nsubps_dn5 - locals.var_ef_nsubc_dn5) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn5), (((2.0 * locals.var_nsubps_dn6) - (((locals.var_nsubps_dn6 - locals.var_ef_nsubc_dn6) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn6), (((2.0 * locals.var_nsubps_dn7) - (((locals.var_nsubps_dn7 - locals.var_ef_nsubc_dn7) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn7), (((2.0 * locals.var_nsubps_dn8) - (((locals.var_nsubps_dn8 - locals.var_ef_nsubc_dn8) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn8), (((2.0 * locals.var_nsubps_dn9) - (((locals.var_nsubps_dn9 - locals.var_ef_nsubc_dn9) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn9), (((2.0 * locals.var_nsubps_dn10) - (((locals.var_nsubps_dn10 - locals.var_ef_nsubc_dn10) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn10), (((2.0 * locals.var_nsubps_dn11) - (((locals.var_nsubps_dn11 - locals.var_ef_nsubc_dn11) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn11), (((2.0 * locals.var_nsubps_dn14) - (((locals.var_nsubps_dn14 - locals.var_ef_nsubc_dn14) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn14),)
    } else {
        (locals.var_nsubb, locals.var_nsubb_dn0, locals.var_nsubb_dn2, locals.var_nsubb_dn4, locals.var_nsubb_dn5, locals.var_nsubb_dn6, locals.var_nsubb_dn7, locals.var_nsubb_dn8, locals.var_nsubb_dn9, locals.var_nsubb_dn10, locals.var_nsubb_dn11, locals.var_nsubb_dn14,)
    }
};
        locals.var_nsubb = assign11720_e6368;
        locals.var_nsubb_dn0 = assign11720_e6368_d_n0;
        locals.var_nsubb_dn2 = assign11720_e6368_d_n2;
        locals.var_nsubb_dn4 = assign11720_e6368_d_n4;
        locals.var_nsubb_dn5 = assign11720_e6368_d_n5;
        locals.var_nsubb_dn6 = assign11720_e6368_d_n6;
        locals.var_nsubb_dn7 = assign11720_e6368_d_n7;
        locals.var_nsubb_dn8 = assign11720_e6368_d_n8;
        locals.var_nsubb_dn9 = assign11720_e6368_d_n9;
        locals.var_nsubb_dn10 = assign11720_e6368_d_n10;
        locals.var_nsubb_dn11 = assign11720_e6368_d_n11;
        locals.var_nsubb_dn14 = assign11720_e6368_d_n14;
        locals.var_nsubb_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_20(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign11730_e6375, assign11730_e6375_d_n0, assign11730_e6375_d_n2, assign11730_e6375_d_n4, assign11730_e6375_d_n5, assign11730_e6375_d_n6, assign11730_e6375_d_n7, assign11730_e6375_d_n8, assign11730_e6375_d_n9, assign11730_e6375_d_n10, assign11730_e6375_d_n11, assign11730_e6375_d_n14,) = {
    if (locals.var_guard272 != 0.0) {
        let assign11730_e6372: f64 = (locals.var_nsubb / locals.var_ef_nsubc);
        let assign11730_e6373: f64 = (assign11730_e6372).ln();
        (assign11730_e6373, ((((locals.var_nsubb_dn0 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn0)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11730_e6372), ((((locals.var_nsubb_dn2 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn2)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11730_e6372), ((((locals.var_nsubb_dn4 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn4)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11730_e6372), ((((locals.var_nsubb_dn5 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn5)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11730_e6372), ((((locals.var_nsubb_dn6 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn6)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11730_e6372), ((((locals.var_nsubb_dn7 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn7)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11730_e6372), ((((locals.var_nsubb_dn8 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn8)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11730_e6372), ((((locals.var_nsubb_dn9 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn9)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11730_e6372), ((((locals.var_nsubb_dn10 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn10)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11730_e6372), ((((locals.var_nsubb_dn11 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn11)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11730_e6372), ((((locals.var_nsubb_dn14 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn14)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11730_e6372),)
    } else {
        (locals.var_ptovr0, locals.var_ptovr0_dn0, locals.var_ptovr0_dn2, locals.var_ptovr0_dn4, locals.var_ptovr0_dn5, locals.var_ptovr0_dn6, locals.var_ptovr0_dn7, locals.var_ptovr0_dn8, locals.var_ptovr0_dn9, locals.var_ptovr0_dn10, locals.var_ptovr0_dn11, locals.var_ptovr0_dn14,)
    }
};
        locals.var_ptovr0 = assign11730_e6375;
        locals.var_ptovr0_dn0 = assign11730_e6375_d_n0;
        locals.var_ptovr0_dn2 = assign11730_e6375_d_n2;
        locals.var_ptovr0_dn4 = assign11730_e6375_d_n4;
        locals.var_ptovr0_dn5 = assign11730_e6375_d_n5;
        locals.var_ptovr0_dn6 = assign11730_e6375_d_n6;
        locals.var_ptovr0_dn7 = assign11730_e6375_d_n7;
        locals.var_ptovr0_dn8 = assign11730_e6375_d_n8;
        locals.var_ptovr0_dn9 = assign11730_e6375_d_n9;
        locals.var_ptovr0_dn10 = assign11730_e6375_d_n10;
        locals.var_ptovr0_dn11 = assign11730_e6375_d_n11;
        locals.var_ptovr0_dn14 = assign11730_e6375_d_n14;
        locals.var_ptovr0_rv = 0.0;

        let (assign11740_e6380, assign11740_e6380_d_n0, assign11740_e6380_d_n2, assign11740_e6380_d_n4, assign11740_e6380_d_n5, assign11740_e6380_d_n6, assign11740_e6380_d_n7, assign11740_e6380_d_n8, assign11740_e6380_d_n9, assign11740_e6380_d_n10, assign11740_e6380_d_n11, assign11740_e6380_d_n14,) = {
    if (locals.var_guard272 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ptovr0, locals.var_ptovr0_dn0, locals.var_ptovr0_dn2, locals.var_ptovr0_dn4, locals.var_ptovr0_dn5, locals.var_ptovr0_dn6, locals.var_ptovr0_dn7, locals.var_ptovr0_dn8, locals.var_ptovr0_dn9, locals.var_ptovr0_dn10, locals.var_ptovr0_dn11, locals.var_ptovr0_dn14,)
    }
};
        locals.var_ptovr0 = assign11740_e6380;
        locals.var_ptovr0_dn0 = assign11740_e6380_d_n0;
        locals.var_ptovr0_dn2 = assign11740_e6380_d_n2;
        locals.var_ptovr0_dn4 = assign11740_e6380_d_n4;
        locals.var_ptovr0_dn5 = assign11740_e6380_d_n5;
        locals.var_ptovr0_dn6 = assign11740_e6380_d_n6;
        locals.var_ptovr0_dn7 = assign11740_e6380_d_n7;
        locals.var_ptovr0_dn8 = assign11740_e6380_d_n8;
        locals.var_ptovr0_dn9 = assign11740_e6380_d_n9;
        locals.var_ptovr0_dn10 = assign11740_e6380_d_n10;
        locals.var_ptovr0_dn11 = assign11740_e6380_d_n11;
        locals.var_ptovr0_dn14 = assign11740_e6380_d_n14;
        locals.var_ptovr0_rv = 0.0;

        let assign11750_e6383: f64 = (2.0 * 1.6021918e-19);
        let assign11750_e6385: f64 = (assign11750_e6383 * locals.var_uc_nsti);
        let assign11750_e6387: f64 = (assign11750_e6385 * 1.034943e-10);
        let assign11750_e6388: f64 = (assign11750_e6387).sqrt();
        locals.var_costi00 = assign11750_e6388;
        locals.var_costi00_rv = 0.0;

        let assign11760_e6392: f64 = (locals.var_uc_nsti * locals.var_uc_nsti);
        let assign11760_e6393: f64 = (1.0 / assign11760_e6392);
        locals.var_nsti_p2 = assign11760_e6393;
        locals.var_nsti_p2_rv = 0.0;

        let assign11770_e6398: f64 = (locals.var_lg).powf(p.p231);
        let assign11770_e6399: f64 = (locals.var_uc_vover / assign11770_e6398);
        let assign11770_e6400: f64 = (1.0 + assign11770_e6399);
        let assign11770_e6405: f64 = (locals.var_wlg).powf(p.p239);
        let assign11770_e6406: f64 = (p.p238 / assign11770_e6405);
        let assign11770_e6407: f64 = (1.0 + assign11770_e6406);
        let assign11770_e6408: f64 = (assign11770_e6400 * assign11770_e6407);
        locals.var_vmax0 = assign11770_e6408;
        locals.var_vmax0_rv = 0.0;

        let assign11780_e6411: f64 = (2.0 / 38.68283);
        let assign11780_e6414: f64 = (locals.var_nsub / 1.04e16);
        let assign11780_e6415: f64 = (assign11780_e6414).ln();
        let assign11780_e6416: f64 = (assign11780_e6411 * assign11780_e6415);
        locals.var_pb20 = assign11780_e6416;
        locals.var_pb20_dn0 = (assign11780_e6411 * ((locals.var_nsub_dn0 / 1.04e16) / assign11780_e6414));
        locals.var_pb20_dn2 = (assign11780_e6411 * ((locals.var_nsub_dn2 / 1.04e16) / assign11780_e6414));
        locals.var_pb20_dn4 = (assign11780_e6411 * ((locals.var_nsub_dn4 / 1.04e16) / assign11780_e6414));
        locals.var_pb20_dn5 = (assign11780_e6411 * ((locals.var_nsub_dn5 / 1.04e16) / assign11780_e6414));
        locals.var_pb20_dn6 = (assign11780_e6411 * ((locals.var_nsub_dn6 / 1.04e16) / assign11780_e6414));
        locals.var_pb20_dn7 = (assign11780_e6411 * ((locals.var_nsub_dn7 / 1.04e16) / assign11780_e6414));
        locals.var_pb20_dn8 = (assign11780_e6411 * ((locals.var_nsub_dn8 / 1.04e16) / assign11780_e6414));
        locals.var_pb20_dn9 = (assign11780_e6411 * ((locals.var_nsub_dn9 / 1.04e16) / assign11780_e6414));
        locals.var_pb20_dn10 = (assign11780_e6411 * ((locals.var_nsub_dn10 / 1.04e16) / assign11780_e6414));
        locals.var_pb20_dn11 = (assign11780_e6411 * ((locals.var_nsub_dn11 / 1.04e16) / assign11780_e6414));
        locals.var_pb20_dn14 = (assign11780_e6411 * ((locals.var_nsub_dn14 / 1.04e16) / assign11780_e6414));
        locals.var_pb20_rv = 0.0;

        let assign11790_e6419: f64 = (2.0 / 38.68283);
        let assign11790_e6422: f64 = (locals.var_ef_nsubc / 1.04e16);
        let assign11790_e6423: f64 = (assign11790_e6422).ln();
        let assign11790_e6424: f64 = (assign11790_e6419 * assign11790_e6423);
        locals.var_pb2c = assign11790_e6424;
        locals.var_pb2c_dn0 = (assign11790_e6419 * ((locals.var_ef_nsubc_dn0 / 1.04e16) / assign11790_e6422));
        locals.var_pb2c_dn2 = (assign11790_e6419 * ((locals.var_ef_nsubc_dn2 / 1.04e16) / assign11790_e6422));
        locals.var_pb2c_dn4 = (assign11790_e6419 * ((locals.var_ef_nsubc_dn4 / 1.04e16) / assign11790_e6422));
        locals.var_pb2c_dn5 = (assign11790_e6419 * ((locals.var_ef_nsubc_dn5 / 1.04e16) / assign11790_e6422));
        locals.var_pb2c_dn6 = (assign11790_e6419 * ((locals.var_ef_nsubc_dn6 / 1.04e16) / assign11790_e6422));
        locals.var_pb2c_dn7 = (assign11790_e6419 * ((locals.var_ef_nsubc_dn7 / 1.04e16) / assign11790_e6422));
        locals.var_pb2c_dn8 = (assign11790_e6419 * ((locals.var_ef_nsubc_dn8 / 1.04e16) / assign11790_e6422));
        locals.var_pb2c_dn9 = (assign11790_e6419 * ((locals.var_ef_nsubc_dn9 / 1.04e16) / assign11790_e6422));
        locals.var_pb2c_dn10 = (assign11790_e6419 * ((locals.var_ef_nsubc_dn10 / 1.04e16) / assign11790_e6422));
        locals.var_pb2c_dn11 = (assign11790_e6419 * ((locals.var_ef_nsubc_dn11 / 1.04e16) / assign11790_e6422));
        locals.var_pb2c_dn14 = (assign11790_e6419 * ((locals.var_ef_nsubc_dn14 / 1.04e16) / assign11790_e6422));
        locals.var_pb2c_rv = 0.0;

        let assign11800_e6427: f64 = if p.p51 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard273 = assign11800_e6427;
        locals.var_guard273_rv = 0.0;

        let (assign11810_e6437, assign11810_e6437_d_n0, assign11810_e6437_d_n2, assign11810_e6437_d_n4, assign11810_e6437_d_n5, assign11810_e6437_d_n6, assign11810_e6437_d_n7, assign11810_e6437_d_n8, assign11810_e6437_d_n9, assign11810_e6437_d_n10, assign11810_e6437_d_n11, assign11810_e6437_d_n14,) = {
    if (locals.var_guard273 != 0.0) {
        let assign11810_e6433: f64 = (3.0 * p.p4);
        let assign11810_e6434: f64 = (locals.var_weff / assign11810_e6433);
        let assign11810_e6435: f64 = (p.p5 + assign11810_e6434);
        (assign11810_e6435, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign11810_e6437;
        locals.var_t1_dn0 = assign11810_e6437_d_n0;
        locals.var_t1_dn2 = assign11810_e6437_d_n2;
        locals.var_t1_dn4 = assign11810_e6437_d_n4;
        locals.var_t1_dn5 = assign11810_e6437_d_n5;
        locals.var_t1_dn6 = assign11810_e6437_d_n6;
        locals.var_t1_dn7 = assign11810_e6437_d_n7;
        locals.var_t1_dn8 = assign11810_e6437_d_n8;
        locals.var_t1_dn9 = assign11810_e6437_d_n9;
        locals.var_t1_dn10 = assign11810_e6437_d_n10;
        locals.var_t1_dn11 = assign11810_e6437_d_n11;
        locals.var_t1_dn14 = assign11810_e6437_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign11820_e6443, assign11820_e6443_d_n0, assign11820_e6443_d_n2, assign11820_e6443_d_n4, assign11820_e6443_d_n5, assign11820_e6443_d_n6, assign11820_e6443_d_n7, assign11820_e6443_d_n8, assign11820_e6443_d_n9, assign11820_e6443_d_n10, assign11820_e6443_d_n11, assign11820_e6443_d_n14,) = {
    if (locals.var_guard273 != 0.0) {
        let assign11820_e6441: f64 = (locals.var_lgate - p.p6);
        (assign11820_e6441, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign11820_e6443;
        locals.var_t2_dn0 = assign11820_e6443_d_n0;
        locals.var_t2_dn2 = assign11820_e6443_d_n2;
        locals.var_t2_dn4 = assign11820_e6443_d_n4;
        locals.var_t2_dn5 = assign11820_e6443_d_n5;
        locals.var_t2_dn6 = assign11820_e6443_d_n6;
        locals.var_t2_dn7 = assign11820_e6443_d_n7;
        locals.var_t2_dn8 = assign11820_e6443_d_n8;
        locals.var_t2_dn9 = assign11820_e6443_d_n9;
        locals.var_t2_dn10 = assign11820_e6443_d_n10;
        locals.var_t2_dn11 = assign11820_e6443_d_n11;
        locals.var_t2_dn14 = assign11820_e6443_d_n14;
        locals.var_t2_rv = 0.0;

        let assign11880_e6485: f64 = if p.p130 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard275 = assign11880_e6485;
        locals.var_guard275_rv = 0.0;

        let (assign11890_e6491,) = {
    if (locals.var_guard275 != 0.0) {
        let assign11890_e6489: f64 = (p.p130 * p.p2);
        (assign11890_e6489,)
    } else {
        (locals.var_rd0,)
    }
};
        locals.var_rd0 = assign11890_e6491;
        locals.var_rd0_rv = 0.0;

        let (assign11900_e6497,) = {
    if (locals.var_guard275 != 0.0) {
        let assign11900_e6495: f64 = (p.p130 * p.p3);
        (assign11900_e6495,)
    } else {
        (locals.var_rs0,)
    }
};
        locals.var_rs0 = assign11900_e6497;
        locals.var_rs0_rv = 0.0;

        let (assign11910_e6502,) = {
    if (locals.var_guard275 == 0.0) {
        (0.0,)
    } else {
        (locals.var_rd0,)
    }
};
        locals.var_rd0 = assign11910_e6502;
        locals.var_rd0_rv = 0.0;

        let (assign11920_e6507,) = {
    if (locals.var_guard275 == 0.0) {
        (0.0,)
    } else {
        (locals.var_rs0,)
    }
};
        locals.var_rs0 = assign11920_e6507;
        locals.var_rs0_rv = 0.0;

        let assign11930_e6510: f64 = if p.p131 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard276 = assign11930_e6510;
        locals.var_guard276_rv = 0.0;

        let (assign11940_e6516,) = {
    if (locals.var_guard276 != 0.0) {
        let assign11940_e6514: f64 = (p.p131 * p.p3);
        (assign11940_e6514,)
    } else {
        (locals.var_rs0,)
    }
};
        locals.var_rs0 = assign11940_e6516;
        locals.var_rs0_rv = 0.0;

        let (assign11950_e6521,) = {
    if (locals.var_guard276 == 0.0) {
        (0.0,)
    } else {
        (locals.var_rs0,)
    }
};
        locals.var_rs0 = assign11950_e6521;
        locals.var_rs0_rv = 0.0;

        let assign11960_e6524: f64 = if locals.var_uc_cordrift == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard277 = assign11960_e6524;
        locals.var_guard277_rv = 0.0;

        let assign11970_e6531: f64 = if ((locals.var_uc_rd > 0.0) || (locals.var_uc_rs > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard278 = assign11970_e6531;
        locals.var_guard278_rv = 0.0;

        let (assign11980_e6543,) = {
    if ((locals.var_guard277 != 0.0) && (locals.var_guard278 != 0.0)) {
        let assign11980_e6539: f64 = (locals.var_wlg).powf(p.p310);
        let assign11980_e6540: f64 = (p.p309 / assign11980_e6539);
        let assign11980_e6541: f64 = (1.0 + assign11980_e6540);
        (assign11980_e6541,)
    } else {
        (locals.var_rdtemp0,)
    }
};
        locals.var_rdtemp0 = assign11980_e6543;
        locals.var_rdtemp0_rv = 0.0;

        let assign11990_e6546: f64 = if locals.var_uc_rdvd != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard279 = assign11990_e6546;
        locals.var_guard279_rv = 0.0;

        let (assign12000_e6560, assign12000_e6560_d_n0, assign12000_e6560_d_n2, assign12000_e6560_d_n4, assign12000_e6560_d_n5, assign12000_e6560_d_n6, assign12000_e6560_d_n7, assign12000_e6560_d_n8, assign12000_e6560_d_n9, assign12000_e6560_d_n10, assign12000_e6560_d_n11, assign12000_e6560_d_n14,) = {
    if (((locals.var_guard277 != 0.0) && (locals.var_guard278 != 0.0)) && (locals.var_guard279 != 0.0)) {
        let assign12000_e6556: f64 = (locals.var_wlg).powf(p.p304);
        let assign12000_e6557: f64 = (p.p303 / assign12000_e6556);
        let assign12000_e6558: f64 = (1.0 + assign12000_e6557);
        (assign12000_e6558, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign12000_e6560;
        locals.var_t7_dn0 = assign12000_e6560_d_n0;
        locals.var_t7_dn2 = assign12000_e6560_d_n2;
        locals.var_t7_dn4 = assign12000_e6560_d_n4;
        locals.var_t7_dn5 = assign12000_e6560_d_n5;
        locals.var_t7_dn6 = assign12000_e6560_d_n6;
        locals.var_t7_dn7 = assign12000_e6560_d_n7;
        locals.var_t7_dn8 = assign12000_e6560_d_n8;
        locals.var_t7_dn9 = assign12000_e6560_d_n9;
        locals.var_t7_dn10 = assign12000_e6560_d_n10;
        locals.var_t7_dn11 = assign12000_e6560_d_n11;
        locals.var_t7_dn14 = assign12000_e6560_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign12010_e6573, assign12010_e6573_d_n0, assign12010_e6573_d_n2, assign12010_e6573_d_n4, assign12010_e6573_d_n5, assign12010_e6573_d_n6, assign12010_e6573_d_n7, assign12010_e6573_d_n8, assign12010_e6573_d_n9, assign12010_e6573_d_n10, assign12010_e6573_d_n11, assign12010_e6573_d_n14,) = {
    if (((locals.var_guard277 != 0.0) && (locals.var_guard278 != 0.0)) && (locals.var_guard279 != 0.0)) {
        let assign12010_e6567: f64 = (-p.p301);
        let assign12010_e6570: f64 = (locals.var_lg).powf(p.p302);
        let assign12010_e6571: f64 = (assign12010_e6567 * assign12010_e6570);
        (assign12010_e6571, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign12010_e6573;
        locals.var_t6_dn0 = assign12010_e6573_d_n0;
        locals.var_t6_dn2 = assign12010_e6573_d_n2;
        locals.var_t6_dn4 = assign12010_e6573_d_n4;
        locals.var_t6_dn5 = assign12010_e6573_d_n5;
        locals.var_t6_dn6 = assign12010_e6573_d_n6;
        locals.var_t6_dn7 = assign12010_e6573_d_n7;
        locals.var_t6_dn8 = assign12010_e6573_d_n8;
        locals.var_t6_dn9 = assign12010_e6573_d_n9;
        locals.var_t6_dn10 = assign12010_e6573_d_n10;
        locals.var_t6_dn11 = assign12010_e6573_d_n11;
        locals.var_t6_dn14 = assign12010_e6573_d_n14;
        locals.var_t6_rv = 0.0;

        let assign12020_e6576: f64 = if locals.var_t6 > 60.0 { 1.0 } else { 0.0 };
        locals.var_guard280 = assign12020_e6576;
        locals.var_guard280_rv = 0.0;

        let (assign12030_e6586, assign12030_e6586_d_n0, assign12030_e6586_d_n2, assign12030_e6586_d_n4, assign12030_e6586_d_n5, assign12030_e6586_d_n6, assign12030_e6586_d_n7, assign12030_e6586_d_n8, assign12030_e6586_d_n9, assign12030_e6586_d_n10, assign12030_e6586_d_n11, assign12030_e6586_d_n14,) = {
    if ((((locals.var_guard277 != 0.0) && (locals.var_guard278 != 0.0)) && (locals.var_guard279 != 0.0)) && (locals.var_guard280 != 0.0)) {
        (60.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign12030_e6586;
        locals.var_t6_dn0 = assign12030_e6586_d_n0;
        locals.var_t6_dn2 = assign12030_e6586_d_n2;
        locals.var_t6_dn4 = assign12030_e6586_d_n4;
        locals.var_t6_dn5 = assign12030_e6586_d_n5;
        locals.var_t6_dn6 = assign12030_e6586_d_n6;
        locals.var_t6_dn7 = assign12030_e6586_d_n7;
        locals.var_t6_dn8 = assign12030_e6586_d_n8;
        locals.var_t6_dn9 = assign12030_e6586_d_n9;
        locals.var_t6_dn10 = assign12030_e6586_d_n10;
        locals.var_t6_dn11 = assign12030_e6586_d_n11;
        locals.var_t6_dn14 = assign12030_e6586_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign12040_e6595, assign12040_e6595_d_n0, assign12040_e6595_d_n2, assign12040_e6595_d_n4, assign12040_e6595_d_n5, assign12040_e6595_d_n6, assign12040_e6595_d_n7, assign12040_e6595_d_n8, assign12040_e6595_d_n9, assign12040_e6595_d_n10, assign12040_e6595_d_n11, assign12040_e6595_d_n14,) = {
    if (((locals.var_guard277 != 0.0) && (locals.var_guard278 != 0.0)) && (locals.var_guard279 != 0.0)) {
        let assign12040_e6593: f64 = (locals.var_t6).exp();
        (assign12040_e6593, (assign12040_e6593 * locals.var_t6_dn0), (assign12040_e6593 * locals.var_t6_dn2), (assign12040_e6593 * locals.var_t6_dn4), (assign12040_e6593 * locals.var_t6_dn5), (assign12040_e6593 * locals.var_t6_dn6), (assign12040_e6593 * locals.var_t6_dn7), (assign12040_e6593 * locals.var_t6_dn8), (assign12040_e6593 * locals.var_t6_dn9), (assign12040_e6593 * locals.var_t6_dn10), (assign12040_e6593 * locals.var_t6_dn11), (assign12040_e6593 * locals.var_t6_dn14),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign12040_e6595;
        locals.var_t6_dn0 = assign12040_e6595_d_n0;
        locals.var_t6_dn2 = assign12040_e6595_d_n2;
        locals.var_t6_dn4 = assign12040_e6595_d_n4;
        locals.var_t6_dn5 = assign12040_e6595_d_n5;
        locals.var_t6_dn6 = assign12040_e6595_d_n6;
        locals.var_t6_dn7 = assign12040_e6595_d_n7;
        locals.var_t6_dn8 = assign12040_e6595_d_n8;
        locals.var_t6_dn9 = assign12040_e6595_d_n9;
        locals.var_t6_dn10 = assign12040_e6595_d_n10;
        locals.var_t6_dn11 = assign12040_e6595_d_n11;
        locals.var_t6_dn14 = assign12040_e6595_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign12050_e6605, assign12050_e6605_d_n0, assign12050_e6605_d_n2, assign12050_e6605_d_n4, assign12050_e6605_d_n5, assign12050_e6605_d_n6, assign12050_e6605_d_n7, assign12050_e6605_d_n8, assign12050_e6605_d_n9, assign12050_e6605_d_n10, assign12050_e6605_d_n11, assign12050_e6605_d_n14,) = {
    if (((locals.var_guard277 != 0.0) && (locals.var_guard278 != 0.0)) && (locals.var_guard279 != 0.0)) {
        let assign12050_e6603: f64 = (locals.var_t6 * locals.var_t7);
        (assign12050_e6603, ((locals.var_t6_dn0 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn0)), ((locals.var_t6_dn2 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn2)), ((locals.var_t6_dn4 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn4)), ((locals.var_t6_dn5 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn5)), ((locals.var_t6_dn6 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn6)), ((locals.var_t6_dn7 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn7)), ((locals.var_t6_dn8 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn8)), ((locals.var_t6_dn9 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn9)), ((locals.var_t6_dn10 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn10)), ((locals.var_t6_dn11 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn11)), ((locals.var_t6_dn14 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn14)),)
    } else {
        (locals.var_rdvdtemp0, locals.var_rdvdtemp0_dn0, locals.var_rdvdtemp0_dn2, locals.var_rdvdtemp0_dn4, locals.var_rdvdtemp0_dn5, locals.var_rdvdtemp0_dn6, locals.var_rdvdtemp0_dn7, locals.var_rdvdtemp0_dn8, locals.var_rdvdtemp0_dn9, locals.var_rdvdtemp0_dn10, locals.var_rdvdtemp0_dn11, locals.var_rdvdtemp0_dn14,)
    }
};
        locals.var_rdvdtemp0 = assign12050_e6605;
        locals.var_rdvdtemp0_dn0 = assign12050_e6605_d_n0;
        locals.var_rdvdtemp0_dn2 = assign12050_e6605_d_n2;
        locals.var_rdvdtemp0_dn4 = assign12050_e6605_d_n4;
        locals.var_rdvdtemp0_dn5 = assign12050_e6605_d_n5;
        locals.var_rdvdtemp0_dn6 = assign12050_e6605_d_n6;
        locals.var_rdvdtemp0_dn7 = assign12050_e6605_d_n7;
        locals.var_rdvdtemp0_dn8 = assign12050_e6605_d_n8;
        locals.var_rdvdtemp0_dn9 = assign12050_e6605_d_n9;
        locals.var_rdvdtemp0_dn10 = assign12050_e6605_d_n10;
        locals.var_rdvdtemp0_dn11 = assign12050_e6605_d_n11;
        locals.var_rdvdtemp0_dn14 = assign12050_e6605_d_n14;
        locals.var_rdvdtemp0_rv = 0.0;

        let (assign12060_e6614, assign12060_e6614_d_n0, assign12060_e6614_d_n2, assign12060_e6614_d_n4, assign12060_e6614_d_n5, assign12060_e6614_d_n6, assign12060_e6614_d_n7, assign12060_e6614_d_n8, assign12060_e6614_d_n9, assign12060_e6614_d_n10, assign12060_e6614_d_n11, assign12060_e6614_d_n14,) = {
    if (((locals.var_guard277 != 0.0) && (locals.var_guard278 != 0.0)) && (locals.var_guard279 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdvdtemp0, locals.var_rdvdtemp0_dn0, locals.var_rdvdtemp0_dn2, locals.var_rdvdtemp0_dn4, locals.var_rdvdtemp0_dn5, locals.var_rdvdtemp0_dn6, locals.var_rdvdtemp0_dn7, locals.var_rdvdtemp0_dn8, locals.var_rdvdtemp0_dn9, locals.var_rdvdtemp0_dn10, locals.var_rdvdtemp0_dn11, locals.var_rdvdtemp0_dn14,)
    }
};
        locals.var_rdvdtemp0 = assign12060_e6614;
        locals.var_rdvdtemp0_dn0 = assign12060_e6614_d_n0;
        locals.var_rdvdtemp0_dn2 = assign12060_e6614_d_n2;
        locals.var_rdvdtemp0_dn4 = assign12060_e6614_d_n4;
        locals.var_rdvdtemp0_dn5 = assign12060_e6614_d_n5;
        locals.var_rdvdtemp0_dn6 = assign12060_e6614_d_n6;
        locals.var_rdvdtemp0_dn7 = assign12060_e6614_d_n7;
        locals.var_rdvdtemp0_dn8 = assign12060_e6614_d_n8;
        locals.var_rdvdtemp0_dn9 = assign12060_e6614_d_n9;
        locals.var_rdvdtemp0_dn10 = assign12060_e6614_d_n10;
        locals.var_rdvdtemp0_dn11 = assign12060_e6614_d_n11;
        locals.var_rdvdtemp0_dn14 = assign12060_e6614_d_n14;
        locals.var_rdvdtemp0_rv = 0.0;

        let (assign12070_e6621,) = {
    if ((locals.var_guard277 != 0.0) && (locals.var_guard278 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdtemp0,)
    }
};
        locals.var_rdtemp0 = assign12070_e6621;
        locals.var_rdtemp0_rv = 0.0;

        let (assign12080_e6628, assign12080_e6628_d_n0, assign12080_e6628_d_n2, assign12080_e6628_d_n4, assign12080_e6628_d_n5, assign12080_e6628_d_n6, assign12080_e6628_d_n7, assign12080_e6628_d_n8, assign12080_e6628_d_n9, assign12080_e6628_d_n10, assign12080_e6628_d_n11, assign12080_e6628_d_n14,) = {
    if ((locals.var_guard277 != 0.0) && (locals.var_guard278 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdvdtemp0, locals.var_rdvdtemp0_dn0, locals.var_rdvdtemp0_dn2, locals.var_rdvdtemp0_dn4, locals.var_rdvdtemp0_dn5, locals.var_rdvdtemp0_dn6, locals.var_rdvdtemp0_dn7, locals.var_rdvdtemp0_dn8, locals.var_rdvdtemp0_dn9, locals.var_rdvdtemp0_dn10, locals.var_rdvdtemp0_dn11, locals.var_rdvdtemp0_dn14,)
    }
};
        locals.var_rdvdtemp0 = assign12080_e6628;
        locals.var_rdvdtemp0_dn0 = assign12080_e6628_d_n0;
        locals.var_rdvdtemp0_dn2 = assign12080_e6628_d_n2;
        locals.var_rdvdtemp0_dn4 = assign12080_e6628_d_n4;
        locals.var_rdvdtemp0_dn5 = assign12080_e6628_d_n5;
        locals.var_rdvdtemp0_dn6 = assign12080_e6628_d_n6;
        locals.var_rdvdtemp0_dn7 = assign12080_e6628_d_n7;
        locals.var_rdvdtemp0_dn8 = assign12080_e6628_d_n8;
        locals.var_rdvdtemp0_dn9 = assign12080_e6628_d_n9;
        locals.var_rdvdtemp0_dn10 = assign12080_e6628_d_n10;
        locals.var_rdvdtemp0_dn11 = assign12080_e6628_d_n11;
        locals.var_rdvdtemp0_dn14 = assign12080_e6628_d_n14;
        locals.var_rdvdtemp0_rv = 0.0;

        let assign12090_e6631: f64 = if locals.var_uc_rd23 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard281 = assign12090_e6631;
        locals.var_guard281_rv = 0.0;

        let (assign12100_e6643, assign12100_e6643_d_n0, assign12100_e6643_d_n2, assign12100_e6643_d_n4, assign12100_e6643_d_n5, assign12100_e6643_d_n6, assign12100_e6643_d_n7, assign12100_e6643_d_n8, assign12100_e6643_d_n9, assign12100_e6643_d_n10, assign12100_e6643_d_n11, assign12100_e6643_d_n14,) = {
    if ((locals.var_guard277 != 0.0) && (locals.var_guard281 != 0.0)) {
        let assign12100_e6639: f64 = (locals.var_wlg).powf(p.p308);
        let assign12100_e6640: f64 = (p.p307 / assign12100_e6639);
        let assign12100_e6641: f64 = (1.0 + assign12100_e6640);
        (assign12100_e6641, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign12100_e6643;
        locals.var_t2_dn0 = assign12100_e6643_d_n0;
        locals.var_t2_dn2 = assign12100_e6643_d_n2;
        locals.var_t2_dn4 = assign12100_e6643_d_n4;
        locals.var_t2_dn5 = assign12100_e6643_d_n5;
        locals.var_t2_dn6 = assign12100_e6643_d_n6;
        locals.var_t2_dn7 = assign12100_e6643_d_n7;
        locals.var_t2_dn8 = assign12100_e6643_d_n8;
        locals.var_t2_dn9 = assign12100_e6643_d_n9;
        locals.var_t2_dn10 = assign12100_e6643_d_n10;
        locals.var_t2_dn11 = assign12100_e6643_d_n11;
        locals.var_t2_dn14 = assign12100_e6643_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign12110_e6654, assign12110_e6654_d_n0, assign12110_e6654_d_n2, assign12110_e6654_d_n4, assign12110_e6654_d_n5, assign12110_e6654_d_n6, assign12110_e6654_d_n7, assign12110_e6654_d_n8, assign12110_e6654_d_n9, assign12110_e6654_d_n10, assign12110_e6654_d_n11, assign12110_e6654_d_n14,) = {
    if ((locals.var_guard277 != 0.0) && (locals.var_guard281 != 0.0)) {
        let assign12110_e6648: f64 = (-p.p305);
        let assign12110_e6651: f64 = (locals.var_lg).powf(p.p306);
        let assign12110_e6652: f64 = (assign12110_e6648 * assign12110_e6651);
        (assign12110_e6652, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign12110_e6654;
        locals.var_t1_dn0 = assign12110_e6654_d_n0;
        locals.var_t1_dn2 = assign12110_e6654_d_n2;
        locals.var_t1_dn4 = assign12110_e6654_d_n4;
        locals.var_t1_dn5 = assign12110_e6654_d_n5;
        locals.var_t1_dn6 = assign12110_e6654_d_n6;
        locals.var_t1_dn7 = assign12110_e6654_d_n7;
        locals.var_t1_dn8 = assign12110_e6654_d_n8;
        locals.var_t1_dn9 = assign12110_e6654_d_n9;
        locals.var_t1_dn10 = assign12110_e6654_d_n10;
        locals.var_t1_dn11 = assign12110_e6654_d_n11;
        locals.var_t1_dn14 = assign12110_e6654_d_n14;
        locals.var_t1_rv = 0.0;

        let assign12120_e6657: f64 = if locals.var_t1 > 60.0 { 1.0 } else { 0.0 };
        locals.var_guard282 = assign12120_e6657;
        locals.var_guard282_rv = 0.0;

        let (assign12130_e6665, assign12130_e6665_d_n0, assign12130_e6665_d_n2, assign12130_e6665_d_n4, assign12130_e6665_d_n5, assign12130_e6665_d_n6, assign12130_e6665_d_n7, assign12130_e6665_d_n8, assign12130_e6665_d_n9, assign12130_e6665_d_n10, assign12130_e6665_d_n11, assign12130_e6665_d_n14,) = {
    if (((locals.var_guard277 != 0.0) && (locals.var_guard281 != 0.0)) && (locals.var_guard282 != 0.0)) {
        (60.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign12130_e6665;
        locals.var_t1_dn0 = assign12130_e6665_d_n0;
        locals.var_t1_dn2 = assign12130_e6665_d_n2;
        locals.var_t1_dn4 = assign12130_e6665_d_n4;
        locals.var_t1_dn5 = assign12130_e6665_d_n5;
        locals.var_t1_dn6 = assign12130_e6665_d_n6;
        locals.var_t1_dn7 = assign12130_e6665_d_n7;
        locals.var_t1_dn8 = assign12130_e6665_d_n8;
        locals.var_t1_dn9 = assign12130_e6665_d_n9;
        locals.var_t1_dn10 = assign12130_e6665_d_n10;
        locals.var_t1_dn11 = assign12130_e6665_d_n11;
        locals.var_t1_dn14 = assign12130_e6665_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_21(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign12140_e6672, assign12140_e6672_d_n0, assign12140_e6672_d_n2, assign12140_e6672_d_n4, assign12140_e6672_d_n5, assign12140_e6672_d_n6, assign12140_e6672_d_n7, assign12140_e6672_d_n8, assign12140_e6672_d_n9, assign12140_e6672_d_n10, assign12140_e6672_d_n11, assign12140_e6672_d_n14,) = {
    if ((locals.var_guard277 != 0.0) && (locals.var_guard281 != 0.0)) {
        let assign12140_e6670: f64 = (locals.var_t1).exp();
        (assign12140_e6670, (assign12140_e6670 * locals.var_t1_dn0), (assign12140_e6670 * locals.var_t1_dn2), (assign12140_e6670 * locals.var_t1_dn4), (assign12140_e6670 * locals.var_t1_dn5), (assign12140_e6670 * locals.var_t1_dn6), (assign12140_e6670 * locals.var_t1_dn7), (assign12140_e6670 * locals.var_t1_dn8), (assign12140_e6670 * locals.var_t1_dn9), (assign12140_e6670 * locals.var_t1_dn10), (assign12140_e6670 * locals.var_t1_dn11), (assign12140_e6670 * locals.var_t1_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign12140_e6672;
        locals.var_t1_dn0 = assign12140_e6672_d_n0;
        locals.var_t1_dn2 = assign12140_e6672_d_n2;
        locals.var_t1_dn4 = assign12140_e6672_d_n4;
        locals.var_t1_dn5 = assign12140_e6672_d_n5;
        locals.var_t1_dn6 = assign12140_e6672_d_n6;
        locals.var_t1_dn7 = assign12140_e6672_d_n7;
        locals.var_t1_dn8 = assign12140_e6672_d_n8;
        locals.var_t1_dn9 = assign12140_e6672_d_n9;
        locals.var_t1_dn10 = assign12140_e6672_d_n10;
        locals.var_t1_dn11 = assign12140_e6672_d_n11;
        locals.var_t1_dn14 = assign12140_e6672_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign12150_e6682, assign12150_e6682_d_n0, assign12150_e6682_d_n2, assign12150_e6682_d_n4, assign12150_e6682_d_n5, assign12150_e6682_d_n6, assign12150_e6682_d_n7, assign12150_e6682_d_n8, assign12150_e6682_d_n9, assign12150_e6682_d_n10, assign12150_e6682_d_n11, assign12150_e6682_d_n14,) = {
    if ((locals.var_guard277 != 0.0) && (locals.var_guard281 != 0.0)) {
        let assign12150_e6678: f64 = (locals.var_uc_rd23 * locals.var_t2);
        let assign12150_e6680: f64 = (assign12150_e6678 * locals.var_t1);
        (assign12150_e6680, (((locals.var_uc_rd23 * locals.var_t2_dn0) * locals.var_t1) + (assign12150_e6678 * locals.var_t1_dn0)), (((locals.var_uc_rd23 * locals.var_t2_dn2) * locals.var_t1) + (assign12150_e6678 * locals.var_t1_dn2)), (((locals.var_uc_rd23 * locals.var_t2_dn4) * locals.var_t1) + (assign12150_e6678 * locals.var_t1_dn4)), (((locals.var_uc_rd23 * locals.var_t2_dn5) * locals.var_t1) + (assign12150_e6678 * locals.var_t1_dn5)), (((locals.var_uc_rd23 * locals.var_t2_dn6) * locals.var_t1) + (assign12150_e6678 * locals.var_t1_dn6)), (((locals.var_uc_rd23 * locals.var_t2_dn7) * locals.var_t1) + (assign12150_e6678 * locals.var_t1_dn7)), (((locals.var_uc_rd23 * locals.var_t2_dn8) * locals.var_t1) + (assign12150_e6678 * locals.var_t1_dn8)), (((locals.var_uc_rd23 * locals.var_t2_dn9) * locals.var_t1) + (assign12150_e6678 * locals.var_t1_dn9)), (((locals.var_uc_rd23 * locals.var_t2_dn10) * locals.var_t1) + (assign12150_e6678 * locals.var_t1_dn10)), (((locals.var_uc_rd23 * locals.var_t2_dn11) * locals.var_t1) + (assign12150_e6678 * locals.var_t1_dn11)), (((locals.var_uc_rd23 * locals.var_t2_dn14) * locals.var_t1) + (assign12150_e6678 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign12150_e6682;
        locals.var_t3_dn0 = assign12150_e6682_d_n0;
        locals.var_t3_dn2 = assign12150_e6682_d_n2;
        locals.var_t3_dn4 = assign12150_e6682_d_n4;
        locals.var_t3_dn5 = assign12150_e6682_d_n5;
        locals.var_t3_dn6 = assign12150_e6682_d_n6;
        locals.var_t3_dn7 = assign12150_e6682_d_n7;
        locals.var_t3_dn8 = assign12150_e6682_d_n8;
        locals.var_t3_dn9 = assign12150_e6682_d_n9;
        locals.var_t3_dn10 = assign12150_e6682_d_n10;
        locals.var_t3_dn11 = assign12150_e6682_d_n11;
        locals.var_t3_dn14 = assign12150_e6682_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign12160_e6705, assign12160_e6705_d_n0, assign12160_e6705_d_n2, assign12160_e6705_d_n4, assign12160_e6705_d_n5, assign12160_e6705_d_n6, assign12160_e6705_d_n7, assign12160_e6705_d_n8, assign12160_e6705_d_n9, assign12160_e6705_d_n10, assign12160_e6705_d_n11, assign12160_e6705_d_n14,) = {
    if ((locals.var_guard277 != 0.0) && (locals.var_guard281 != 0.0)) {
        let assign12160_e6690: f64 = (locals.var_t3 * locals.var_t3);
        let assign12160_e6693: f64 = (4.0 * 1e-6);
        let assign12160_e6695: f64 = (assign12160_e6693 / 100.0);
        let assign12160_e6697: f64 = (assign12160_e6695 * 1e-6);
        let assign12160_e6699: f64 = (assign12160_e6697 / 100.0);
        let assign12160_e6700: f64 = (assign12160_e6690 + assign12160_e6699);
        let assign12160_e6701: f64 = (assign12160_e6700).sqrt();
        let assign12160_e6702: f64 = (locals.var_t3 + assign12160_e6701);
        let assign12160_e6703: f64 = (0.5 * assign12160_e6702);
        (assign12160_e6703, (0.5 * (locals.var_t3_dn0 + (((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0)) / (2.0 * assign12160_e6701)))), (0.5 * (locals.var_t3_dn2 + (((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2)) / (2.0 * assign12160_e6701)))), (0.5 * (locals.var_t3_dn4 + (((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)) / (2.0 * assign12160_e6701)))), (0.5 * (locals.var_t3_dn5 + (((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)) / (2.0 * assign12160_e6701)))), (0.5 * (locals.var_t3_dn6 + (((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)) / (2.0 * assign12160_e6701)))), (0.5 * (locals.var_t3_dn7 + (((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7)) / (2.0 * assign12160_e6701)))), (0.5 * (locals.var_t3_dn8 + (((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)) / (2.0 * assign12160_e6701)))), (0.5 * (locals.var_t3_dn9 + (((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9)) / (2.0 * assign12160_e6701)))), (0.5 * (locals.var_t3_dn10 + (((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)) / (2.0 * assign12160_e6701)))), (0.5 * (locals.var_t3_dn11 + (((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11)) / (2.0 * assign12160_e6701)))), (0.5 * (locals.var_t3_dn14 + (((locals.var_t3_dn14 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn14)) / (2.0 * assign12160_e6701)))),)
    } else {
        (locals.var_rd23e, locals.var_rd23e_dn0, locals.var_rd23e_dn2, locals.var_rd23e_dn4, locals.var_rd23e_dn5, locals.var_rd23e_dn6, locals.var_rd23e_dn7, locals.var_rd23e_dn8, locals.var_rd23e_dn9, locals.var_rd23e_dn10, locals.var_rd23e_dn11, locals.var_rd23e_dn14,)
    }
};
        locals.var_rd23e = assign12160_e6705;
        locals.var_rd23e_dn0 = assign12160_e6705_d_n0;
        locals.var_rd23e_dn2 = assign12160_e6705_d_n2;
        locals.var_rd23e_dn4 = assign12160_e6705_d_n4;
        locals.var_rd23e_dn5 = assign12160_e6705_d_n5;
        locals.var_rd23e_dn6 = assign12160_e6705_d_n6;
        locals.var_rd23e_dn7 = assign12160_e6705_d_n7;
        locals.var_rd23e_dn8 = assign12160_e6705_d_n8;
        locals.var_rd23e_dn9 = assign12160_e6705_d_n9;
        locals.var_rd23e_dn10 = assign12160_e6705_d_n10;
        locals.var_rd23e_dn11 = assign12160_e6705_d_n11;
        locals.var_rd23e_dn14 = assign12160_e6705_d_n14;
        locals.var_rd23e_rv = 0.0;

        let (assign12170_e6712, assign12170_e6712_d_n0, assign12170_e6712_d_n2, assign12170_e6712_d_n4, assign12170_e6712_d_n5, assign12170_e6712_d_n6, assign12170_e6712_d_n7, assign12170_e6712_d_n8, assign12170_e6712_d_n9, assign12170_e6712_d_n10, assign12170_e6712_d_n11, assign12170_e6712_d_n14,) = {
    if ((locals.var_guard277 != 0.0) && (locals.var_guard281 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rd23e, locals.var_rd23e_dn0, locals.var_rd23e_dn2, locals.var_rd23e_dn4, locals.var_rd23e_dn5, locals.var_rd23e_dn6, locals.var_rd23e_dn7, locals.var_rd23e_dn8, locals.var_rd23e_dn9, locals.var_rd23e_dn10, locals.var_rd23e_dn11, locals.var_rd23e_dn14,)
    }
};
        locals.var_rd23e = assign12170_e6712;
        locals.var_rd23e_dn0 = assign12170_e6712_d_n0;
        locals.var_rd23e_dn2 = assign12170_e6712_d_n2;
        locals.var_rd23e_dn4 = assign12170_e6712_d_n4;
        locals.var_rd23e_dn5 = assign12170_e6712_d_n5;
        locals.var_rd23e_dn6 = assign12170_e6712_d_n6;
        locals.var_rd23e_dn7 = assign12170_e6712_d_n7;
        locals.var_rd23e_dn8 = assign12170_e6712_d_n8;
        locals.var_rd23e_dn9 = assign12170_e6712_d_n9;
        locals.var_rd23e_dn10 = assign12170_e6712_d_n10;
        locals.var_rd23e_dn11 = assign12170_e6712_d_n11;
        locals.var_rd23e_dn14 = assign12170_e6712_d_n14;
        locals.var_rd23e_rv = 0.0;

        let (assign12180_e6716,) = {
    if (locals.var_guard277 != 0.0) {
        (0.0,)
    } else {
        (locals.var_xmax,)
    }
};
        locals.var_xmax = assign12180_e6716;
        locals.var_xmax_rv = 0.0;

        let (assign12190_e6720,) = {
    if (locals.var_guard277 != 0.0) {
        (0.0,)
    } else {
        (locals.var_xmax_s,)
    }
};
        locals.var_xmax_s = assign12190_e6720;
        locals.var_xmax_s_rv = 0.0;

        let (assign12200_e6724,) = {
    if (locals.var_guard277 != 0.0) {
        (0.0,)
    } else {
        (locals.var_rdrvmaxwe,)
    }
};
        locals.var_rdrvmaxwe = assign12200_e6724;
        locals.var_rdrvmaxwe_rv = 0.0;

        let (assign12210_e6728,) = {
    if (locals.var_guard277 != 0.0) {
        (0.0,)
    } else {
        (locals.var_rdrvmaxle,)
    }
};
        locals.var_rdrvmaxle = assign12210_e6728;
        locals.var_rdrvmaxle_rv = 0.0;

        let (assign12220_e6732,) = {
    if (locals.var_guard277 != 0.0) {
        (0.0,)
    } else {
        (locals.var_rdrmuele,)
    }
};
        locals.var_rdrmuele = assign12220_e6732;
        locals.var_rdrmuele_rv = 0.0;

        let (assign12230_e6736, assign12230_e6736_d_n0, assign12230_e6736_d_n2, assign12230_e6736_d_n4, assign12230_e6736_d_n5, assign12230_e6736_d_n6, assign12230_e6736_d_n7, assign12230_e6736_d_n8, assign12230_e6736_d_n9, assign12230_e6736_d_n10, assign12230_e6736_d_n11, assign12230_e6736_d_n14,) = {
    if (locals.var_guard277 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdrmuevbs, locals.var_rdrmuevbs_dn0, locals.var_rdrmuevbs_dn2, locals.var_rdrmuevbs_dn4, locals.var_rdrmuevbs_dn5, locals.var_rdrmuevbs_dn6, locals.var_rdrmuevbs_dn7, locals.var_rdrmuevbs_dn8, locals.var_rdrmuevbs_dn9, locals.var_rdrmuevbs_dn10, locals.var_rdrmuevbs_dn11, locals.var_rdrmuevbs_dn14,)
    }
};
        locals.var_rdrmuevbs = assign12230_e6736;
        locals.var_rdrmuevbs_dn0 = assign12230_e6736_d_n0;
        locals.var_rdrmuevbs_dn2 = assign12230_e6736_d_n2;
        locals.var_rdrmuevbs_dn4 = assign12230_e6736_d_n4;
        locals.var_rdrmuevbs_dn5 = assign12230_e6736_d_n5;
        locals.var_rdrmuevbs_dn6 = assign12230_e6736_d_n6;
        locals.var_rdrmuevbs_dn7 = assign12230_e6736_d_n7;
        locals.var_rdrmuevbs_dn8 = assign12230_e6736_d_n8;
        locals.var_rdrmuevbs_dn9 = assign12230_e6736_d_n9;
        locals.var_rdrmuevbs_dn10 = assign12230_e6736_d_n10;
        locals.var_rdrmuevbs_dn11 = assign12230_e6736_d_n11;
        locals.var_rdrmuevbs_dn14 = assign12230_e6736_d_n14;
        locals.var_rdrmuevbs_rv = 0.0;

        let (assign12240_e6748,) = {
    if (locals.var_guard277 == 0.0) {
        let assign12240_e6741: f64 = (p.p419 * p.p419);
        let assign12240_e6744: f64 = (locals.var_uc_xldld * locals.var_uc_xldld);
        let assign12240_e6745: f64 = (assign12240_e6741 + assign12240_e6744);
        let assign12240_e6746: f64 = (assign12240_e6745).sqrt();
        (assign12240_e6746,)
    } else {
        (locals.var_xmax,)
    }
};
        locals.var_xmax = assign12240_e6748;
        locals.var_xmax_rv = 0.0;

        let (assign12250_e6760,) = {
    if (locals.var_guard277 == 0.0) {
        let assign12250_e6753: f64 = (p.p419 * p.p419);
        let assign12250_e6756: f64 = (p.p97 * p.p97);
        let assign12250_e6757: f64 = (assign12250_e6753 + assign12250_e6756);
        let assign12250_e6758: f64 = (assign12250_e6757).sqrt();
        (assign12250_e6758,)
    } else {
        (locals.var_xmax_s,)
    }
};
        locals.var_xmax_s = assign12250_e6760;
        locals.var_xmax_s_rv = 0.0;

        let (assign12260_e6771,) = {
    if (locals.var_guard277 == 0.0) {
        let assign12260_e6767: f64 = (locals.var_wg).powf(p.p425);
        let assign12260_e6768: f64 = (p.p424 / assign12260_e6767);
        let assign12260_e6769: f64 = (1.0 + assign12260_e6768);
        (assign12260_e6769,)
    } else {
        (locals.var_rdrvmaxwe,)
    }
};
        locals.var_rdrvmaxwe = assign12260_e6771;
        locals.var_rdrvmaxwe_rv = 0.0;

        let (assign12270_e6782,) = {
    if (locals.var_guard277 == 0.0) {
        let assign12270_e6778: f64 = (locals.var_lg).powf(p.p427);
        let assign12270_e6779: f64 = (p.p426 / assign12270_e6778);
        let assign12270_e6780: f64 = (1.0 + assign12270_e6779);
        (assign12270_e6780,)
    } else {
        (locals.var_rdrvmaxle,)
    }
};
        locals.var_rdrvmaxle = assign12270_e6782;
        locals.var_rdrvmaxle_rv = 0.0;

        let (assign12280_e6793,) = {
    if (locals.var_guard277 == 0.0) {
        let assign12280_e6789: f64 = (locals.var_lg).powf(p.p429);
        let assign12280_e6790: f64 = (p.p428 / assign12280_e6789);
        let assign12280_e6791: f64 = (1.0 + assign12280_e6790);
        (assign12280_e6791,)
    } else {
        (locals.var_rdrmuele,)
    }
};
        locals.var_rdrmuele = assign12280_e6793;
        locals.var_rdrmuele_rv = 0.0;

        let (assign12290_e6798, assign12290_e6798_d_n0, assign12290_e6798_d_n2, assign12290_e6798_d_n4, assign12290_e6798_d_n5, assign12290_e6798_d_n6, assign12290_e6798_d_n7, assign12290_e6798_d_n8, assign12290_e6798_d_n9, assign12290_e6798_d_n10, assign12290_e6798_d_n11, assign12290_e6798_d_n14,) = {
    if (locals.var_guard277 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdrmuevbs, locals.var_rdrmuevbs_dn0, locals.var_rdrmuevbs_dn2, locals.var_rdrmuevbs_dn4, locals.var_rdrmuevbs_dn5, locals.var_rdrmuevbs_dn6, locals.var_rdrmuevbs_dn7, locals.var_rdrmuevbs_dn8, locals.var_rdrmuevbs_dn9, locals.var_rdrmuevbs_dn10, locals.var_rdrmuevbs_dn11, locals.var_rdrmuevbs_dn14,)
    }
};
        locals.var_rdrmuevbs = assign12290_e6798;
        locals.var_rdrmuevbs_dn0 = assign12290_e6798_d_n0;
        locals.var_rdrmuevbs_dn2 = assign12290_e6798_d_n2;
        locals.var_rdrmuevbs_dn4 = assign12290_e6798_d_n4;
        locals.var_rdrmuevbs_dn5 = assign12290_e6798_d_n5;
        locals.var_rdrmuevbs_dn6 = assign12290_e6798_d_n6;
        locals.var_rdrmuevbs_dn7 = assign12290_e6798_d_n7;
        locals.var_rdrmuevbs_dn8 = assign12290_e6798_d_n8;
        locals.var_rdrmuevbs_dn9 = assign12290_e6798_d_n9;
        locals.var_rdrmuevbs_dn10 = assign12290_e6798_d_n10;
        locals.var_rdrmuevbs_dn11 = assign12290_e6798_d_n11;
        locals.var_rdrmuevbs_dn14 = assign12290_e6798_d_n14;
        locals.var_rdrmuevbs_rv = 0.0;

        let (assign12300_e6803,) = {
    if (locals.var_guard277 == 0.0) {
        (0.0,)
    } else {
        (locals.var_rdtemp0,)
    }
};
        locals.var_rdtemp0 = assign12300_e6803;
        locals.var_rdtemp0_rv = 0.0;

        let (assign12310_e6808, assign12310_e6808_d_n0, assign12310_e6808_d_n2, assign12310_e6808_d_n4, assign12310_e6808_d_n5, assign12310_e6808_d_n6, assign12310_e6808_d_n7, assign12310_e6808_d_n8, assign12310_e6808_d_n9, assign12310_e6808_d_n10, assign12310_e6808_d_n11, assign12310_e6808_d_n14,) = {
    if (locals.var_guard277 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdvdtemp0, locals.var_rdvdtemp0_dn0, locals.var_rdvdtemp0_dn2, locals.var_rdvdtemp0_dn4, locals.var_rdvdtemp0_dn5, locals.var_rdvdtemp0_dn6, locals.var_rdvdtemp0_dn7, locals.var_rdvdtemp0_dn8, locals.var_rdvdtemp0_dn9, locals.var_rdvdtemp0_dn10, locals.var_rdvdtemp0_dn11, locals.var_rdvdtemp0_dn14,)
    }
};
        locals.var_rdvdtemp0 = assign12310_e6808;
        locals.var_rdvdtemp0_dn0 = assign12310_e6808_d_n0;
        locals.var_rdvdtemp0_dn2 = assign12310_e6808_d_n2;
        locals.var_rdvdtemp0_dn4 = assign12310_e6808_d_n4;
        locals.var_rdvdtemp0_dn5 = assign12310_e6808_d_n5;
        locals.var_rdvdtemp0_dn6 = assign12310_e6808_d_n6;
        locals.var_rdvdtemp0_dn7 = assign12310_e6808_d_n7;
        locals.var_rdvdtemp0_dn8 = assign12310_e6808_d_n8;
        locals.var_rdvdtemp0_dn9 = assign12310_e6808_d_n9;
        locals.var_rdvdtemp0_dn10 = assign12310_e6808_d_n10;
        locals.var_rdvdtemp0_dn11 = assign12310_e6808_d_n11;
        locals.var_rdvdtemp0_dn14 = assign12310_e6808_d_n14;
        locals.var_rdvdtemp0_rv = 0.0;

        let (assign12320_e6813, assign12320_e6813_d_n0, assign12320_e6813_d_n2, assign12320_e6813_d_n4, assign12320_e6813_d_n5, assign12320_e6813_d_n6, assign12320_e6813_d_n7, assign12320_e6813_d_n8, assign12320_e6813_d_n9, assign12320_e6813_d_n10, assign12320_e6813_d_n11, assign12320_e6813_d_n14,) = {
    if (locals.var_guard277 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rd23e, locals.var_rd23e_dn0, locals.var_rd23e_dn2, locals.var_rd23e_dn4, locals.var_rd23e_dn5, locals.var_rd23e_dn6, locals.var_rd23e_dn7, locals.var_rd23e_dn8, locals.var_rd23e_dn9, locals.var_rd23e_dn10, locals.var_rd23e_dn11, locals.var_rd23e_dn14,)
    }
};
        locals.var_rd23e = assign12320_e6813;
        locals.var_rd23e_dn0 = assign12320_e6813_d_n0;
        locals.var_rd23e_dn2 = assign12320_e6813_d_n2;
        locals.var_rd23e_dn4 = assign12320_e6813_d_n4;
        locals.var_rd23e_dn5 = assign12320_e6813_d_n5;
        locals.var_rd23e_dn6 = assign12320_e6813_d_n6;
        locals.var_rd23e_dn7 = assign12320_e6813_d_n7;
        locals.var_rd23e_dn8 = assign12320_e6813_d_n8;
        locals.var_rd23e_dn9 = assign12320_e6813_d_n9;
        locals.var_rd23e_dn10 = assign12320_e6813_d_n10;
        locals.var_rd23e_dn11 = assign12320_e6813_d_n11;
        locals.var_rd23e_dn14 = assign12320_e6813_d_n14;
        locals.var_rd23e_rv = 0.0;

        let assign12330_e6816: f64 = if locals.var_uc_nover > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard283 = assign12330_e6816;
        locals.var_guard283_rv = 0.0;

        let (assign12340_e6826,) = {
    if (locals.var_guard283 != 0.0) {
        let assign12340_e6820: f64 = (2.0 * 1.034943e-10);
        let assign12340_e6823: f64 = (1.6021918e-19 * locals.var_uc_nover);
        let assign12340_e6824: f64 = (assign12340_e6820 / assign12340_e6823);
        (assign12340_e6824,)
    } else {
        (locals.var_kdep,)
    }
};
        locals.var_kdep = assign12340_e6826;
        locals.var_kdep_rv = 0.0;

        let (assign12350_e6842, assign12350_e6842_d_n0, assign12350_e6842_d_n2, assign12350_e6842_d_n4, assign12350_e6842_d_n5, assign12350_e6842_d_n6, assign12350_e6842_d_n7, assign12350_e6842_d_n8, assign12350_e6842_d_n9, assign12350_e6842_d_n10, assign12350_e6842_d_n11, assign12350_e6842_d_n14,) = {
    if (locals.var_guard283 != 0.0) {
        let assign12350_e6830: f64 = (2.0 * 1.034943e-10);
        let assign12350_e6832: f64 = (assign12350_e6830 / 1.6021918e-19);
        let assign12350_e6834: f64 = (assign12350_e6832 * locals.var_ef_nsubc);
        let assign12350_e6837: f64 = (locals.var_uc_nover + locals.var_ef_nsubc);
        let assign12350_e6838: f64 = (assign12350_e6834 / assign12350_e6837);
        let assign12350_e6840: f64 = (assign12350_e6838 / locals.var_uc_nover);
        (assign12350_e6840, (((((assign12350_e6832 * locals.var_ef_nsubc_dn0) * assign12350_e6837) - (assign12350_e6834 * locals.var_ef_nsubc_dn0)) / (assign12350_e6837 * assign12350_e6837)) / locals.var_uc_nover), (((((assign12350_e6832 * locals.var_ef_nsubc_dn2) * assign12350_e6837) - (assign12350_e6834 * locals.var_ef_nsubc_dn2)) / (assign12350_e6837 * assign12350_e6837)) / locals.var_uc_nover), (((((assign12350_e6832 * locals.var_ef_nsubc_dn4) * assign12350_e6837) - (assign12350_e6834 * locals.var_ef_nsubc_dn4)) / (assign12350_e6837 * assign12350_e6837)) / locals.var_uc_nover), (((((assign12350_e6832 * locals.var_ef_nsubc_dn5) * assign12350_e6837) - (assign12350_e6834 * locals.var_ef_nsubc_dn5)) / (assign12350_e6837 * assign12350_e6837)) / locals.var_uc_nover), (((((assign12350_e6832 * locals.var_ef_nsubc_dn6) * assign12350_e6837) - (assign12350_e6834 * locals.var_ef_nsubc_dn6)) / (assign12350_e6837 * assign12350_e6837)) / locals.var_uc_nover), (((((assign12350_e6832 * locals.var_ef_nsubc_dn7) * assign12350_e6837) - (assign12350_e6834 * locals.var_ef_nsubc_dn7)) / (assign12350_e6837 * assign12350_e6837)) / locals.var_uc_nover), (((((assign12350_e6832 * locals.var_ef_nsubc_dn8) * assign12350_e6837) - (assign12350_e6834 * locals.var_ef_nsubc_dn8)) / (assign12350_e6837 * assign12350_e6837)) / locals.var_uc_nover), (((((assign12350_e6832 * locals.var_ef_nsubc_dn9) * assign12350_e6837) - (assign12350_e6834 * locals.var_ef_nsubc_dn9)) / (assign12350_e6837 * assign12350_e6837)) / locals.var_uc_nover), (((((assign12350_e6832 * locals.var_ef_nsubc_dn10) * assign12350_e6837) - (assign12350_e6834 * locals.var_ef_nsubc_dn10)) / (assign12350_e6837 * assign12350_e6837)) / locals.var_uc_nover), (((((assign12350_e6832 * locals.var_ef_nsubc_dn11) * assign12350_e6837) - (assign12350_e6834 * locals.var_ef_nsubc_dn11)) / (assign12350_e6837 * assign12350_e6837)) / locals.var_uc_nover), (((((assign12350_e6832 * locals.var_ef_nsubc_dn14) * assign12350_e6837) - (assign12350_e6834 * locals.var_ef_nsubc_dn14)) / (assign12350_e6837 * assign12350_e6837)) / locals.var_uc_nover),)
    } else {
        (locals.var_kjunc, locals.var_kjunc_dn0, locals.var_kjunc_dn2, locals.var_kjunc_dn4, locals.var_kjunc_dn5, locals.var_kjunc_dn6, locals.var_kjunc_dn7, locals.var_kjunc_dn8, locals.var_kjunc_dn9, locals.var_kjunc_dn10, locals.var_kjunc_dn11, locals.var_kjunc_dn14,)
    }
};
        locals.var_kjunc = assign12350_e6842;
        locals.var_kjunc_dn0 = assign12350_e6842_d_n0;
        locals.var_kjunc_dn2 = assign12350_e6842_d_n2;
        locals.var_kjunc_dn4 = assign12350_e6842_d_n4;
        locals.var_kjunc_dn5 = assign12350_e6842_d_n5;
        locals.var_kjunc_dn6 = assign12350_e6842_d_n6;
        locals.var_kjunc_dn7 = assign12350_e6842_d_n7;
        locals.var_kjunc_dn8 = assign12350_e6842_d_n8;
        locals.var_kjunc_dn9 = assign12350_e6842_d_n9;
        locals.var_kjunc_dn10 = assign12350_e6842_d_n10;
        locals.var_kjunc_dn11 = assign12350_e6842_d_n11;
        locals.var_kjunc_dn14 = assign12350_e6842_d_n14;
        locals.var_kjunc_rv = 0.0;

        let (assign12360_e6847,) = {
    if (locals.var_guard283 == 0.0) {
        (0.0,)
    } else {
        (locals.var_kdep,)
    }
};
        locals.var_kdep = assign12360_e6847;
        locals.var_kdep_rv = 0.0;

        let (assign12370_e6852, assign12370_e6852_d_n0, assign12370_e6852_d_n2, assign12370_e6852_d_n4, assign12370_e6852_d_n5, assign12370_e6852_d_n6, assign12370_e6852_d_n7, assign12370_e6852_d_n8, assign12370_e6852_d_n9, assign12370_e6852_d_n10, assign12370_e6852_d_n11, assign12370_e6852_d_n14,) = {
    if (locals.var_guard283 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_kjunc, locals.var_kjunc_dn0, locals.var_kjunc_dn2, locals.var_kjunc_dn4, locals.var_kjunc_dn5, locals.var_kjunc_dn6, locals.var_kjunc_dn7, locals.var_kjunc_dn8, locals.var_kjunc_dn9, locals.var_kjunc_dn10, locals.var_kjunc_dn11, locals.var_kjunc_dn14,)
    }
};
        locals.var_kjunc = assign12370_e6852;
        locals.var_kjunc_dn0 = assign12370_e6852_d_n0;
        locals.var_kjunc_dn2 = assign12370_e6852_d_n2;
        locals.var_kjunc_dn4 = assign12370_e6852_d_n4;
        locals.var_kjunc_dn5 = assign12370_e6852_d_n5;
        locals.var_kjunc_dn6 = assign12370_e6852_d_n6;
        locals.var_kjunc_dn7 = assign12370_e6852_d_n7;
        locals.var_kjunc_dn8 = assign12370_e6852_d_n8;
        locals.var_kjunc_dn9 = assign12370_e6852_d_n9;
        locals.var_kjunc_dn10 = assign12370_e6852_d_n10;
        locals.var_kjunc_dn11 = assign12370_e6852_d_n11;
        locals.var_kjunc_dn14 = assign12370_e6852_d_n14;
        locals.var_kjunc_rv = 0.0;

        let assign12510_e6947: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard288 = assign12510_e6947;
        locals.var_guard288_rv = 0.0;

        let (assign12520_e6955, assign12520_e6955_d_n0, assign12520_e6955_d_n2, assign12520_e6955_d_n4, assign12520_e6955_d_n5, assign12520_e6955_d_n6, assign12520_e6955_d_n7, assign12520_e6955_d_n8, assign12520_e6955_d_n9, assign12520_e6955_d_n10, assign12520_e6955_d_n11, assign12520_e6955_d_n14,) = {
    if (locals.var_guard288 != 0.0) {
        let assign12520_e6951: f64 = (p.p108 * locals.var_lg);
        let assign12520_e6953: f64 = (assign12520_e6951 + p.p109);
        (assign12520_e6953, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign12520_e6955;
        locals.var_t1_dn0 = assign12520_e6955_d_n0;
        locals.var_t1_dn2 = assign12520_e6955_d_n2;
        locals.var_t1_dn4 = assign12520_e6955_d_n4;
        locals.var_t1_dn5 = assign12520_e6955_d_n5;
        locals.var_t1_dn6 = assign12520_e6955_d_n6;
        locals.var_t1_dn7 = assign12520_e6955_d_n7;
        locals.var_t1_dn8 = assign12520_e6955_d_n8;
        locals.var_t1_dn9 = assign12520_e6955_d_n9;
        locals.var_t1_dn10 = assign12520_e6955_d_n10;
        locals.var_t1_dn11 = assign12520_e6955_d_n11;
        locals.var_t1_dn14 = assign12520_e6955_d_n14;
        locals.var_t1_rv = 0.0;

        let assign12530_e6958: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard289 = assign12530_e6958;
        locals.var_guard289_rv = 0.0;

        let (assign12540_e6964, assign12540_e6964_d_n0, assign12540_e6964_d_n2, assign12540_e6964_d_n4, assign12540_e6964_d_n5, assign12540_e6964_d_n6, assign12540_e6964_d_n7, assign12540_e6964_d_n8, assign12540_e6964_d_n9, assign12540_e6964_d_n10, assign12540_e6964_d_n11, assign12540_e6964_d_n14,) = {
    if ((locals.var_guard288 != 0.0) && (locals.var_guard289 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign12540_e6964;
        locals.var_t1_dn0 = assign12540_e6964_d_n0;
        locals.var_t1_dn2 = assign12540_e6964_d_n2;
        locals.var_t1_dn4 = assign12540_e6964_d_n4;
        locals.var_t1_dn5 = assign12540_e6964_d_n5;
        locals.var_t1_dn6 = assign12540_e6964_d_n6;
        locals.var_t1_dn7 = assign12540_e6964_d_n7;
        locals.var_t1_dn8 = assign12540_e6964_d_n8;
        locals.var_t1_dn9 = assign12540_e6964_d_n9;
        locals.var_t1_dn10 = assign12540_e6964_d_n10;
        locals.var_t1_dn11 = assign12540_e6964_d_n11;
        locals.var_t1_dn14 = assign12540_e6964_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign12550_e6976, assign12550_e6976_d_n0, assign12550_e6976_d_n2, assign12550_e6976_d_n4, assign12550_e6976_d_n5, assign12550_e6976_d_n6, assign12550_e6976_d_n7, assign12550_e6976_d_n8, assign12550_e6976_d_n9, assign12550_e6976_d_n10, assign12550_e6976_d_n11, assign12550_e6976_d_n14,) = {
    if (locals.var_guard288 != 0.0) {
        let assign12550_e6968: f64 = (locals.var_t1 * p.p107);
        let assign12550_e6971: f64 = (locals.var_t1 + p.p107);
        let assign12550_e6972: f64 = (assign12550_e6968 / assign12550_e6971);
        let assign12550_e6974: f64 = (assign12550_e6972 + 1.0);
        (assign12550_e6974, ((((locals.var_t1_dn0 * p.p107) * assign12550_e6971) - (assign12550_e6968 * locals.var_t1_dn0)) / (assign12550_e6971 * assign12550_e6971)), ((((locals.var_t1_dn2 * p.p107) * assign12550_e6971) - (assign12550_e6968 * locals.var_t1_dn2)) / (assign12550_e6971 * assign12550_e6971)), ((((locals.var_t1_dn4 * p.p107) * assign12550_e6971) - (assign12550_e6968 * locals.var_t1_dn4)) / (assign12550_e6971 * assign12550_e6971)), ((((locals.var_t1_dn5 * p.p107) * assign12550_e6971) - (assign12550_e6968 * locals.var_t1_dn5)) / (assign12550_e6971 * assign12550_e6971)), ((((locals.var_t1_dn6 * p.p107) * assign12550_e6971) - (assign12550_e6968 * locals.var_t1_dn6)) / (assign12550_e6971 * assign12550_e6971)), ((((locals.var_t1_dn7 * p.p107) * assign12550_e6971) - (assign12550_e6968 * locals.var_t1_dn7)) / (assign12550_e6971 * assign12550_e6971)), ((((locals.var_t1_dn8 * p.p107) * assign12550_e6971) - (assign12550_e6968 * locals.var_t1_dn8)) / (assign12550_e6971 * assign12550_e6971)), ((((locals.var_t1_dn9 * p.p107) * assign12550_e6971) - (assign12550_e6968 * locals.var_t1_dn9)) / (assign12550_e6971 * assign12550_e6971)), ((((locals.var_t1_dn10 * p.p107) * assign12550_e6971) - (assign12550_e6968 * locals.var_t1_dn10)) / (assign12550_e6971 * assign12550_e6971)), ((((locals.var_t1_dn11 * p.p107) * assign12550_e6971) - (assign12550_e6968 * locals.var_t1_dn11)) / (assign12550_e6971 * assign12550_e6971)), ((((locals.var_t1_dn14 * p.p107) * assign12550_e6971) - (assign12550_e6968 * locals.var_t1_dn14)) / (assign12550_e6971 * assign12550_e6971)),)
    } else {
        (locals.var_ddlte, locals.var_ddlte_dn0, locals.var_ddlte_dn2, locals.var_ddlte_dn4, locals.var_ddlte_dn5, locals.var_ddlte_dn6, locals.var_ddlte_dn7, locals.var_ddlte_dn8, locals.var_ddlte_dn9, locals.var_ddlte_dn10, locals.var_ddlte_dn11, locals.var_ddlte_dn14,)
    }
};
        locals.var_ddlte = assign12550_e6976;
        locals.var_ddlte_dn0 = assign12550_e6976_d_n0;
        locals.var_ddlte_dn2 = assign12550_e6976_d_n2;
        locals.var_ddlte_dn4 = assign12550_e6976_d_n4;
        locals.var_ddlte_dn5 = assign12550_e6976_d_n5;
        locals.var_ddlte_dn6 = assign12550_e6976_d_n6;
        locals.var_ddlte_dn7 = assign12550_e6976_d_n7;
        locals.var_ddlte_dn8 = assign12550_e6976_d_n8;
        locals.var_ddlte_dn9 = assign12550_e6976_d_n9;
        locals.var_ddlte_dn10 = assign12550_e6976_d_n10;
        locals.var_ddlte_dn11 = assign12550_e6976_d_n11;
        locals.var_ddlte_dn14 = assign12550_e6976_d_n14;
        locals.var_ddlte_rv = 0.0;

        let (assign12560_e6983, assign12560_e6983_d_n0, assign12560_e6983_d_n2, assign12560_e6983_d_n4, assign12560_e6983_d_n5, assign12560_e6983_d_n6, assign12560_e6983_d_n7, assign12560_e6983_d_n8, assign12560_e6983_d_n9, assign12560_e6983_d_n10, assign12560_e6983_d_n11, assign12560_e6983_d_n14,) = {
    if (locals.var_guard288 == 0.0) {
        let assign12560_e6981: f64 = (p.p108 * locals.var_lg);
        (assign12560_e6981, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign12560_e6983;
        locals.var_t1_dn0 = assign12560_e6983_d_n0;
        locals.var_t1_dn2 = assign12560_e6983_d_n2;
        locals.var_t1_dn4 = assign12560_e6983_d_n4;
        locals.var_t1_dn5 = assign12560_e6983_d_n5;
        locals.var_t1_dn6 = assign12560_e6983_d_n6;
        locals.var_t1_dn7 = assign12560_e6983_d_n7;
        locals.var_t1_dn8 = assign12560_e6983_d_n8;
        locals.var_t1_dn9 = assign12560_e6983_d_n9;
        locals.var_t1_dn10 = assign12560_e6983_d_n10;
        locals.var_t1_dn11 = assign12560_e6983_d_n11;
        locals.var_t1_dn14 = assign12560_e6983_d_n14;
        locals.var_t1_rv = 0.0;

        let assign12570_e6986: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard290 = assign12570_e6986;
        locals.var_guard290_rv = 0.0;

        let (assign12580_e6993, assign12580_e6993_d_n0, assign12580_e6993_d_n2, assign12580_e6993_d_n4, assign12580_e6993_d_n5, assign12580_e6993_d_n6, assign12580_e6993_d_n7, assign12580_e6993_d_n8, assign12580_e6993_d_n9, assign12580_e6993_d_n10, assign12580_e6993_d_n11, assign12580_e6993_d_n14,) = {
    if ((locals.var_guard288 == 0.0) && (locals.var_guard290 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign12580_e6993;
        locals.var_t1_dn0 = assign12580_e6993_d_n0;
        locals.var_t1_dn2 = assign12580_e6993_d_n2;
        locals.var_t1_dn4 = assign12580_e6993_d_n4;
        locals.var_t1_dn5 = assign12580_e6993_d_n5;
        locals.var_t1_dn6 = assign12580_e6993_d_n6;
        locals.var_t1_dn7 = assign12580_e6993_d_n7;
        locals.var_t1_dn8 = assign12580_e6993_d_n8;
        locals.var_t1_dn9 = assign12580_e6993_d_n9;
        locals.var_t1_dn10 = assign12580_e6993_d_n10;
        locals.var_t1_dn11 = assign12580_e6993_d_n11;
        locals.var_t1_dn14 = assign12580_e6993_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_22(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign12590_e7008, assign12590_e7008_d_n0, assign12590_e7008_d_n2, assign12590_e7008_d_n4, assign12590_e7008_d_n5, assign12590_e7008_d_n6, assign12590_e7008_d_n7, assign12590_e7008_d_n8, assign12590_e7008_d_n9, assign12590_e7008_d_n10, assign12590_e7008_d_n11, assign12590_e7008_d_n14,) = {
    if (locals.var_guard288 == 0.0) {
        let assign12590_e6998: f64 = (locals.var_t1 * p.p107);
        let assign12590_e7001: f64 = (locals.var_t1 + p.p107);
        let assign12590_e7002: f64 = (assign12590_e6998 / assign12590_e7001);
        let assign12590_e7004: f64 = (assign12590_e7002 + p.p109);
        let assign12590_e7006: f64 = (assign12590_e7004 + 1e-25);
        (assign12590_e7006, ((((locals.var_t1_dn0 * p.p107) * assign12590_e7001) - (assign12590_e6998 * locals.var_t1_dn0)) / (assign12590_e7001 * assign12590_e7001)), ((((locals.var_t1_dn2 * p.p107) * assign12590_e7001) - (assign12590_e6998 * locals.var_t1_dn2)) / (assign12590_e7001 * assign12590_e7001)), ((((locals.var_t1_dn4 * p.p107) * assign12590_e7001) - (assign12590_e6998 * locals.var_t1_dn4)) / (assign12590_e7001 * assign12590_e7001)), ((((locals.var_t1_dn5 * p.p107) * assign12590_e7001) - (assign12590_e6998 * locals.var_t1_dn5)) / (assign12590_e7001 * assign12590_e7001)), ((((locals.var_t1_dn6 * p.p107) * assign12590_e7001) - (assign12590_e6998 * locals.var_t1_dn6)) / (assign12590_e7001 * assign12590_e7001)), ((((locals.var_t1_dn7 * p.p107) * assign12590_e7001) - (assign12590_e6998 * locals.var_t1_dn7)) / (assign12590_e7001 * assign12590_e7001)), ((((locals.var_t1_dn8 * p.p107) * assign12590_e7001) - (assign12590_e6998 * locals.var_t1_dn8)) / (assign12590_e7001 * assign12590_e7001)), ((((locals.var_t1_dn9 * p.p107) * assign12590_e7001) - (assign12590_e6998 * locals.var_t1_dn9)) / (assign12590_e7001 * assign12590_e7001)), ((((locals.var_t1_dn10 * p.p107) * assign12590_e7001) - (assign12590_e6998 * locals.var_t1_dn10)) / (assign12590_e7001 * assign12590_e7001)), ((((locals.var_t1_dn11 * p.p107) * assign12590_e7001) - (assign12590_e6998 * locals.var_t1_dn11)) / (assign12590_e7001 * assign12590_e7001)), ((((locals.var_t1_dn14 * p.p107) * assign12590_e7001) - (assign12590_e6998 * locals.var_t1_dn14)) / (assign12590_e7001 * assign12590_e7001)),)
    } else {
        (locals.var_ddlte, locals.var_ddlte_dn0, locals.var_ddlte_dn2, locals.var_ddlte_dn4, locals.var_ddlte_dn5, locals.var_ddlte_dn6, locals.var_ddlte_dn7, locals.var_ddlte_dn8, locals.var_ddlte_dn9, locals.var_ddlte_dn10, locals.var_ddlte_dn11, locals.var_ddlte_dn14,)
    }
};
        locals.var_ddlte = assign12590_e7008;
        locals.var_ddlte_dn0 = assign12590_e7008_d_n0;
        locals.var_ddlte_dn2 = assign12590_e7008_d_n2;
        locals.var_ddlte_dn4 = assign12590_e7008_d_n4;
        locals.var_ddlte_dn5 = assign12590_e7008_d_n5;
        locals.var_ddlte_dn6 = assign12590_e7008_d_n6;
        locals.var_ddlte_dn7 = assign12590_e7008_d_n7;
        locals.var_ddlte_dn8 = assign12590_e7008_d_n8;
        locals.var_ddlte_dn9 = assign12590_e7008_d_n9;
        locals.var_ddlte_dn10 = assign12590_e7008_d_n10;
        locals.var_ddlte_dn11 = assign12590_e7008_d_n11;
        locals.var_ddlte_dn14 = assign12590_e7008_d_n14;
        locals.var_ddlte_rv = 0.0;

        let assign12610_e7016: f64 = if locals.var_ddlte < 0.1 { 1.0 } else { 0.0 };
        locals.var_guard292 = assign12610_e7016;
        locals.var_guard292_rv = 0.0;

        let (assign12620_e7020, assign12620_e7020_d_n0, assign12620_e7020_d_n2, assign12620_e7020_d_n4, assign12620_e7020_d_n5, assign12620_e7020_d_n6, assign12620_e7020_d_n7, assign12620_e7020_d_n8, assign12620_e7020_d_n9, assign12620_e7020_d_n10, assign12620_e7020_d_n11, assign12620_e7020_d_n14,) = {
    if (locals.var_guard292 != 0.0) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ddlte, locals.var_ddlte_dn0, locals.var_ddlte_dn2, locals.var_ddlte_dn4, locals.var_ddlte_dn5, locals.var_ddlte_dn6, locals.var_ddlte_dn7, locals.var_ddlte_dn8, locals.var_ddlte_dn9, locals.var_ddlte_dn10, locals.var_ddlte_dn11, locals.var_ddlte_dn14,)
    }
};
        locals.var_ddlte = assign12620_e7020;
        locals.var_ddlte_dn0 = assign12620_e7020_d_n0;
        locals.var_ddlte_dn2 = assign12620_e7020_d_n2;
        locals.var_ddlte_dn4 = assign12620_e7020_d_n4;
        locals.var_ddlte_dn5 = assign12620_e7020_d_n5;
        locals.var_ddlte_dn6 = assign12620_e7020_d_n6;
        locals.var_ddlte_dn7 = assign12620_e7020_d_n7;
        locals.var_ddlte_dn8 = assign12620_e7020_d_n8;
        locals.var_ddlte_dn9 = assign12620_e7020_d_n9;
        locals.var_ddlte_dn10 = assign12620_e7020_d_n10;
        locals.var_ddlte_dn11 = assign12620_e7020_d_n11;
        locals.var_ddlte_dn14 = assign12620_e7020_d_n14;
        locals.var_ddlte_rv = 0.0;

        let (assign12630_e7026, assign12630_e7026_d_n0, assign12630_e7026_d_n2, assign12630_e7026_d_n4, assign12630_e7026_d_n5, assign12630_e7026_d_n6, assign12630_e7026_d_n7, assign12630_e7026_d_n8, assign12630_e7026_d_n9, assign12630_e7026_d_n10, assign12630_e7026_d_n11, assign12630_e7026_d_n14,) = {
    if (p.p23 != 0.0) {
        let assign12630_e7024: f64 = (locals.var_weff).powf(p.p201);
        (assign12630_e7024, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign12630_e7026;
        locals.var_t2_dn0 = assign12630_e7026_d_n0;
        locals.var_t2_dn2 = assign12630_e7026_d_n2;
        locals.var_t2_dn4 = assign12630_e7026_d_n4;
        locals.var_t2_dn5 = assign12630_e7026_d_n5;
        locals.var_t2_dn6 = assign12630_e7026_d_n6;
        locals.var_t2_dn7 = assign12630_e7026_d_n7;
        locals.var_t2_dn8 = assign12630_e7026_d_n8;
        locals.var_t2_dn9 = assign12630_e7026_d_n9;
        locals.var_t2_dn10 = assign12630_e7026_d_n10;
        locals.var_t2_dn11 = assign12630_e7026_d_n11;
        locals.var_t2_dn14 = assign12630_e7026_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign12640_e7044, assign12640_e7044_d_n0, assign12640_e7044_d_n2, assign12640_e7044_d_n4, assign12640_e7044_d_n5, assign12640_e7044_d_n6, assign12640_e7044_d_n7, assign12640_e7044_d_n8, assign12640_e7044_d_n9, assign12640_e7044_d_n10, assign12640_e7044_d_n11, assign12640_e7044_d_n14,) = {
    if (p.p23 != 0.0) {
        let assign12640_e7033: f64 = (locals.var_lgate).powf(p.p199);
        let assign12640_e7034: f64 = (locals.var_mks_svgsl / assign12640_e7033);
        let assign12640_e7035: f64 = (1.0 + assign12640_e7034);
        let assign12640_e7036: f64 = (locals.var_uc_svgs * assign12640_e7035);
        let assign12640_e7040: f64 = (locals.var_t2 + locals.var_mks_svgsw);
        let assign12640_e7041: f64 = (locals.var_t2 / assign12640_e7040);
        let assign12640_e7042: f64 = (assign12640_e7036 * assign12640_e7041);
        (assign12640_e7042, (assign12640_e7036 * (((locals.var_t2_dn0 * assign12640_e7040) - (locals.var_t2 * locals.var_t2_dn0)) / (assign12640_e7040 * assign12640_e7040))), (assign12640_e7036 * (((locals.var_t2_dn2 * assign12640_e7040) - (locals.var_t2 * locals.var_t2_dn2)) / (assign12640_e7040 * assign12640_e7040))), (assign12640_e7036 * (((locals.var_t2_dn4 * assign12640_e7040) - (locals.var_t2 * locals.var_t2_dn4)) / (assign12640_e7040 * assign12640_e7040))), (assign12640_e7036 * (((locals.var_t2_dn5 * assign12640_e7040) - (locals.var_t2 * locals.var_t2_dn5)) / (assign12640_e7040 * assign12640_e7040))), (assign12640_e7036 * (((locals.var_t2_dn6 * assign12640_e7040) - (locals.var_t2 * locals.var_t2_dn6)) / (assign12640_e7040 * assign12640_e7040))), (assign12640_e7036 * (((locals.var_t2_dn7 * assign12640_e7040) - (locals.var_t2 * locals.var_t2_dn7)) / (assign12640_e7040 * assign12640_e7040))), (assign12640_e7036 * (((locals.var_t2_dn8 * assign12640_e7040) - (locals.var_t2 * locals.var_t2_dn8)) / (assign12640_e7040 * assign12640_e7040))), (assign12640_e7036 * (((locals.var_t2_dn9 * assign12640_e7040) - (locals.var_t2 * locals.var_t2_dn9)) / (assign12640_e7040 * assign12640_e7040))), (assign12640_e7036 * (((locals.var_t2_dn10 * assign12640_e7040) - (locals.var_t2 * locals.var_t2_dn10)) / (assign12640_e7040 * assign12640_e7040))), (assign12640_e7036 * (((locals.var_t2_dn11 * assign12640_e7040) - (locals.var_t2 * locals.var_t2_dn11)) / (assign12640_e7040 * assign12640_e7040))), (assign12640_e7036 * (((locals.var_t2_dn14 * assign12640_e7040) - (locals.var_t2 * locals.var_t2_dn14)) / (assign12640_e7040 * assign12640_e7040))),)
    } else {
        (locals.var_vg2const, locals.var_vg2const_dn0, locals.var_vg2const_dn2, locals.var_vg2const_dn4, locals.var_vg2const_dn5, locals.var_vg2const_dn6, locals.var_vg2const_dn7, locals.var_vg2const_dn8, locals.var_vg2const_dn9, locals.var_vg2const_dn10, locals.var_vg2const_dn11, locals.var_vg2const_dn14,)
    }
};
        locals.var_vg2const = assign12640_e7044;
        locals.var_vg2const_dn0 = assign12640_e7044_d_n0;
        locals.var_vg2const_dn2 = assign12640_e7044_d_n2;
        locals.var_vg2const_dn4 = assign12640_e7044_d_n4;
        locals.var_vg2const_dn5 = assign12640_e7044_d_n5;
        locals.var_vg2const_dn6 = assign12640_e7044_d_n6;
        locals.var_vg2const_dn7 = assign12640_e7044_d_n7;
        locals.var_vg2const_dn8 = assign12640_e7044_d_n8;
        locals.var_vg2const_dn9 = assign12640_e7044_d_n9;
        locals.var_vg2const_dn10 = assign12640_e7044_d_n10;
        locals.var_vg2const_dn11 = assign12640_e7044_d_n11;
        locals.var_vg2const_dn14 = assign12640_e7044_d_n14;
        locals.var_vg2const_rv = 0.0;

        let (assign12650_e7056,) = {
    if (p.p23 != 0.0) {
        let assign12650_e7051: f64 = (locals.var_lgate).powf(p.p184);
        let assign12650_e7052: f64 = (locals.var_mks_svbsl / assign12650_e7051);
        let assign12650_e7053: f64 = (1.0 + assign12650_e7052);
        let assign12650_e7054: f64 = (locals.var_uc_svbs * assign12650_e7053);
        (assign12650_e7054,)
    } else {
        (locals.var_xvbs,)
    }
};
        locals.var_xvbs = assign12650_e7056;
        locals.var_xvbs_rv = 0.0;

        let (assign12660_e7068,) = {
    if (p.p23 != 0.0) {
        let assign12660_e7063: f64 = (locals.var_lgate).powf(p.p203);
        let assign12660_e7064: f64 = (locals.var_mks_slgl / assign12660_e7063);
        let assign12660_e7065: f64 = (1.0 + assign12660_e7064);
        let assign12660_e7066: f64 = (locals.var_mks_slg * assign12660_e7065);
        (assign12660_e7066,)
    } else {
        (locals.var_xgate,)
    }
};
        locals.var_xgate = assign12660_e7068;
        locals.var_xgate_rv = 0.0;

        let (assign12670_e7080,) = {
    if (p.p23 != 0.0) {
        let assign12670_e7075: f64 = (locals.var_lgate).powf(p.p191);
        let assign12670_e7076: f64 = (locals.var_mks_sub1l / assign12670_e7075);
        let assign12670_e7077: f64 = (1.0 + assign12670_e7076);
        let assign12670_e7078: f64 = (locals.var_uc_sub1 * assign12670_e7077);
        (assign12670_e7078,)
    } else {
        (locals.var_xsub1,)
    }
};
        locals.var_xsub1 = assign12670_e7080;
        locals.var_xsub1_rv = 0.0;

        let (assign12680_e7090,) = {
    if (p.p23 != 0.0) {
        let assign12680_e7086: f64 = (locals.var_mks_sub2l / locals.var_lgate);
        let assign12680_e7087: f64 = (1.0 + assign12680_e7086);
        let assign12680_e7088: f64 = (locals.var_uc_sub2 * assign12680_e7087);
        (assign12680_e7088,)
    } else {
        (locals.var_xsub2,)
    }
};
        locals.var_xsub2 = assign12680_e7090;
        locals.var_xsub2_rv = 0.0;

        let (assign12690_e7094,) = {
    if (p.p23 != 0.0) {
        (locals.var_xsub1,)
    } else {
        (locals.var_xsub1_1,)
    }
};
        locals.var_xsub1_1 = assign12690_e7094;
        locals.var_xsub1_1_rv = 0.0;

        let (assign12700_e7098,) = {
    if (p.p23 != 0.0) {
        (locals.var_xsub2,)
    } else {
        (locals.var_xsub2_1,)
    }
};
        locals.var_xsub2_1 = assign12700_e7098;
        locals.var_xsub2_1_rv = 0.0;

        let (assign12710_e7102, assign12710_e7102_d_n0, assign12710_e7102_d_n2, assign12710_e7102_d_n4, assign12710_e7102_d_n5, assign12710_e7102_d_n6, assign12710_e7102_d_n7, assign12710_e7102_d_n8, assign12710_e7102_d_n9, assign12710_e7102_d_n10, assign12710_e7102_d_n11, assign12710_e7102_d_n14,) = {
    if (p.p23 != 0.0) {
        (locals.var_vg2const, locals.var_vg2const_dn0, locals.var_vg2const_dn2, locals.var_vg2const_dn4, locals.var_vg2const_dn5, locals.var_vg2const_dn6, locals.var_vg2const_dn7, locals.var_vg2const_dn8, locals.var_vg2const_dn9, locals.var_vg2const_dn10, locals.var_vg2const_dn11, locals.var_vg2const_dn14,)
    } else {
        (locals.var_vg2const_1, locals.var_vg2const_1_dn0, locals.var_vg2const_1_dn2, locals.var_vg2const_1_dn4, locals.var_vg2const_1_dn5, locals.var_vg2const_1_dn6, locals.var_vg2const_1_dn7, locals.var_vg2const_1_dn8, locals.var_vg2const_1_dn9, locals.var_vg2const_1_dn10, locals.var_vg2const_1_dn11, locals.var_vg2const_1_dn14,)
    }
};
        locals.var_vg2const_1 = assign12710_e7102;
        locals.var_vg2const_1_dn0 = assign12710_e7102_d_n0;
        locals.var_vg2const_1_dn2 = assign12710_e7102_d_n2;
        locals.var_vg2const_1_dn4 = assign12710_e7102_d_n4;
        locals.var_vg2const_1_dn5 = assign12710_e7102_d_n5;
        locals.var_vg2const_1_dn6 = assign12710_e7102_d_n6;
        locals.var_vg2const_1_dn7 = assign12710_e7102_d_n7;
        locals.var_vg2const_1_dn8 = assign12710_e7102_d_n8;
        locals.var_vg2const_1_dn9 = assign12710_e7102_d_n9;
        locals.var_vg2const_1_dn10 = assign12710_e7102_d_n10;
        locals.var_vg2const_1_dn11 = assign12710_e7102_d_n11;
        locals.var_vg2const_1_dn14 = assign12710_e7102_d_n14;
        locals.var_vg2const_1_rv = 0.0;

        let (assign12720_e7106,) = {
    if (p.p23 != 0.0) {
        (locals.var_xvbs,)
    } else {
        (locals.var_xvbs_1,)
    }
};
        locals.var_xvbs_1 = assign12720_e7106;
        locals.var_xvbs_1_rv = 0.0;

        let (assign12730_e7110,) = {
    if (p.p23 != 0.0) {
        (locals.var_xgate,)
    } else {
        (locals.var_xgate_1,)
    }
};
        locals.var_xgate_1 = assign12730_e7110;
        locals.var_xgate_1_rv = 0.0;

        let (assign12740_e7124,) = {
    if ((p.p23 != 0.0) && (p.p46 != 0.0)) {
        let assign12740_e7119: f64 = (locals.var_lgate).powf(p.p191);
        let assign12740_e7120: f64 = (locals.var_mks_sub1l / assign12740_e7119);
        let assign12740_e7121: f64 = (1.0 + assign12740_e7120);
        let assign12740_e7122: f64 = (locals.var_uc_sub1snp * assign12740_e7121);
        (assign12740_e7122,)
    } else {
        (locals.var_xsub1_1,)
    }
};
        locals.var_xsub1_1 = assign12740_e7124;
        locals.var_xsub1_1_rv = 0.0;

        let (assign12750_e7136,) = {
    if ((p.p23 != 0.0) && (p.p46 != 0.0)) {
        let assign12750_e7132: f64 = (locals.var_mks_sub2l / locals.var_lgate);
        let assign12750_e7133: f64 = (1.0 + assign12750_e7132);
        let assign12750_e7134: f64 = (locals.var_uc_sub2snp * assign12750_e7133);
        (assign12750_e7134,)
    } else {
        (locals.var_xsub2_1,)
    }
};
        locals.var_xsub2_1 = assign12750_e7136;
        locals.var_xsub2_1_rv = 0.0;

        let (assign12760_e7148,) = {
    if (p.p23 != 0.0) {
        let assign12760_e7143: f64 = (locals.var_lg).powf(p.p103);
        let assign12760_e7144: f64 = (p.p102 / assign12760_e7143);
        let assign12760_e7145: f64 = (1.0 + assign12760_e7144);
        let assign12760_e7146: f64 = (p.p72 * assign12760_e7145);
        (assign12760_e7146,)
    } else {
        (locals.var_uc_subld1,)
    }
};
        locals.var_uc_subld1 = assign12760_e7148;
        locals.var_uc_subld1_rv = 0.0;

        let (assign12770_e7153, assign12770_e7153_d_n0, assign12770_e7153_d_n2, assign12770_e7153_d_n4, assign12770_e7153_d_n5, assign12770_e7153_d_n6, assign12770_e7153_d_n7, assign12770_e7153_d_n8, assign12770_e7153_d_n9, assign12770_e7153_d_n10, assign12770_e7153_d_n11, assign12770_e7153_d_n14,) = {
    if (p.p23 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vg2const, locals.var_vg2const_dn0, locals.var_vg2const_dn2, locals.var_vg2const_dn4, locals.var_vg2const_dn5, locals.var_vg2const_dn6, locals.var_vg2const_dn7, locals.var_vg2const_dn8, locals.var_vg2const_dn9, locals.var_vg2const_dn10, locals.var_vg2const_dn11, locals.var_vg2const_dn14,)
    }
};
        locals.var_vg2const = assign12770_e7153;
        locals.var_vg2const_dn0 = assign12770_e7153_d_n0;
        locals.var_vg2const_dn2 = assign12770_e7153_d_n2;
        locals.var_vg2const_dn4 = assign12770_e7153_d_n4;
        locals.var_vg2const_dn5 = assign12770_e7153_d_n5;
        locals.var_vg2const_dn6 = assign12770_e7153_d_n6;
        locals.var_vg2const_dn7 = assign12770_e7153_d_n7;
        locals.var_vg2const_dn8 = assign12770_e7153_d_n8;
        locals.var_vg2const_dn9 = assign12770_e7153_d_n9;
        locals.var_vg2const_dn10 = assign12770_e7153_d_n10;
        locals.var_vg2const_dn11 = assign12770_e7153_d_n11;
        locals.var_vg2const_dn14 = assign12770_e7153_d_n14;
        locals.var_vg2const_rv = 0.0;

        let (assign12780_e7158,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xvbs,)
    }
};
        locals.var_xvbs = assign12780_e7158;
        locals.var_xvbs_rv = 0.0;

        let (assign12790_e7163,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xgate,)
    }
};
        locals.var_xgate = assign12790_e7163;
        locals.var_xgate_rv = 0.0;

        let (assign12800_e7168,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xsub1,)
    }
};
        locals.var_xsub1 = assign12800_e7168;
        locals.var_xsub1_rv = 0.0;

        let (assign12810_e7173,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xsub2,)
    }
};
        locals.var_xsub2 = assign12810_e7173;
        locals.var_xsub2_rv = 0.0;

        let (assign12820_e7178,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_uc_subld1,)
    }
};
        locals.var_uc_subld1 = assign12820_e7178;
        locals.var_uc_subld1_rv = 0.0;

        let (assign12830_e7183, assign12830_e7183_d_n0, assign12830_e7183_d_n2, assign12830_e7183_d_n4, assign12830_e7183_d_n5, assign12830_e7183_d_n6, assign12830_e7183_d_n7, assign12830_e7183_d_n8, assign12830_e7183_d_n9, assign12830_e7183_d_n10, assign12830_e7183_d_n11, assign12830_e7183_d_n14,) = {
    if (p.p23 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vg2const_1, locals.var_vg2const_1_dn0, locals.var_vg2const_1_dn2, locals.var_vg2const_1_dn4, locals.var_vg2const_1_dn5, locals.var_vg2const_1_dn6, locals.var_vg2const_1_dn7, locals.var_vg2const_1_dn8, locals.var_vg2const_1_dn9, locals.var_vg2const_1_dn10, locals.var_vg2const_1_dn11, locals.var_vg2const_1_dn14,)
    }
};
        locals.var_vg2const_1 = assign12830_e7183;
        locals.var_vg2const_1_dn0 = assign12830_e7183_d_n0;
        locals.var_vg2const_1_dn2 = assign12830_e7183_d_n2;
        locals.var_vg2const_1_dn4 = assign12830_e7183_d_n4;
        locals.var_vg2const_1_dn5 = assign12830_e7183_d_n5;
        locals.var_vg2const_1_dn6 = assign12830_e7183_d_n6;
        locals.var_vg2const_1_dn7 = assign12830_e7183_d_n7;
        locals.var_vg2const_1_dn8 = assign12830_e7183_d_n8;
        locals.var_vg2const_1_dn9 = assign12830_e7183_d_n9;
        locals.var_vg2const_1_dn10 = assign12830_e7183_d_n10;
        locals.var_vg2const_1_dn11 = assign12830_e7183_d_n11;
        locals.var_vg2const_1_dn14 = assign12830_e7183_d_n14;
        locals.var_vg2const_1_rv = 0.0;

        let (assign12840_e7188,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xvbs_1,)
    }
};
        locals.var_xvbs_1 = assign12840_e7188;
        locals.var_xvbs_1_rv = 0.0;

        let (assign12850_e7193,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xgate_1,)
    }
};
        locals.var_xgate_1 = assign12850_e7193;
        locals.var_xgate_1_rv = 0.0;

        let (assign12860_e7198,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xsub1_1,)
    }
};
        locals.var_xsub1_1 = assign12860_e7198;
        locals.var_xsub1_1_rv = 0.0;

        let (assign12870_e7203,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xsub2_1,)
    }
};
        locals.var_xsub2_1 = assign12870_e7203;
        locals.var_xsub2_1_rv = 0.0;

        let (assign12880_e7217,) = {
    if (locals.var_uc_ibpc1 != 0.0) {
        let assign12880_e7212: f64 = (locals.var_lg).powf(p.p280);
        let assign12880_e7213: f64 = (p.p279 / assign12880_e7212);
        let assign12880_e7214: f64 = (1.0 + assign12880_e7213);
        let assign12880_e7215: f64 = (locals.var_uc_ibpc1 * assign12880_e7214);
        (assign12880_e7215,)
    } else {
        (0.0,)
    }
};
        locals.var_uc_ibpc1 = assign12880_e7217;
        locals.var_uc_ibpc1_rv = 0.0;

        let assign12890_e7221: f64 = (3.141592653589793 / 2.0);
        let assign12890_e7222: f64 = (3.453133e-11 / assign12890_e7221);
        let assign12890_e7224: f64 = (assign12890_e7222 * locals.var_weffcv_nf);
        let assign12890_e7228: f64 = (p.p225 / p.p95);
        let assign12890_e7229: f64 = (1.0 + assign12890_e7228);
        let assign12890_e7230: f64 = (assign12890_e7229).ln();
        let assign12890_e7231: f64 = (assign12890_e7224 * assign12890_e7230);
        locals.var_cfrng = assign12890_e7231;
        locals.var_cfrng_rv = 0.0;

        let (assign12900_e7245,) = {
    if (p.p134 != 0.0) {
        let assign12900_e7237: f64 = (1000000.0 * locals.var_weffcv_nf);
        let assign12900_e7239: f64 = (assign12900_e7237 * p.p134);
        let assign12900_e7242: f64 = (locals.var_lg).powf(p.p135);
        let assign12900_e7243: f64 = (assign12900_e7239 / assign12900_e7242);
        (assign12900_e7243,)
    } else {
        (0.0,)
    }
};
        locals.var_cqyb0 = assign12900_e7245;
        locals.var_cqyb0_rv = 0.0;

        let assign12910_e7249: f64 = (-p.p286);
        let assign12910_e7250: f64 = (locals.var_lg).powf(assign12910_e7249);
        let assign12910_e7251: f64 = (p.p283 * assign12910_e7250);
        locals.var_ptl0 = assign12910_e7251;
        locals.var_ptl0_rv = 0.0;

        let assign12920_e7255: f64 = (-p.p291);
        let assign12920_e7256: f64 = (locals.var_lg).powf(assign12920_e7255);
        let assign12920_e7257: f64 = (p.p290 * assign12920_e7256);
        locals.var_pt40 = assign12920_e7257;
        locals.var_pt40_rv = 0.0;

        let assign12930_e7261: f64 = (locals.var_lg + locals.var_uc_gdld);
        let assign12930_e7263: f64 = (-p.p288);
        let assign12930_e7264: f64 = (assign12930_e7261).powf(assign12930_e7263);
        let assign12930_e7265: f64 = (p.p287 * assign12930_e7264);
        locals.var_gdl0 = assign12930_e7265;
        locals.var_gdl0_rv = 0.0;

        let assign12940_e7269: f64 = (locals.var_mfactor * locals.var_weff_nf);
        let assign12940_e7270: f64 = (locals.var_uc_rth0 / assign12940_e7269);
        let assign12940_e7275: f64 = (locals.var_lg).powf(p.p318);
        let assign12940_e7276: f64 = (p.p317 / assign12940_e7275);
        let assign12940_e7277: f64 = (1.0 + assign12940_e7276);
        let assign12940_e7278: f64 = (assign12940_e7270 * assign12940_e7277);
        let assign12940_e7283: f64 = (locals.var_wg).powf(p.p316);
        let assign12940_e7284: f64 = (p.p315 / assign12940_e7283);
        let assign12940_e7285: f64 = (1.0 + assign12940_e7284);
        let assign12940_e7286: f64 = (assign12940_e7278 * assign12940_e7285);
        locals.var_rth = assign12940_e7286;
        locals.var_rth_dn0 = 0.0;
        locals.var_rth_dn2 = 0.0;
        locals.var_rth_dn4 = 0.0;
        locals.var_rth_dn5 = 0.0;
        locals.var_rth_dn6 = 0.0;
        locals.var_rth_dn7 = 0.0;
        locals.var_rth_dn8 = 0.0;
        locals.var_rth_dn9 = 0.0;
        locals.var_rth_dn10 = 0.0;
        locals.var_rth_dn11 = 0.0;
        locals.var_rth_dn14 = 0.0;
        locals.var_rth_rv = 0.0;

        let assign12960_e7296: f64 = (p.p7).powf(p.p327);
        let assign12960_e7297: f64 = (1.0 / assign12960_e7296);
        let assign12960_e7298: f64 = (locals.var_rth * assign12960_e7297);
        locals.var_rth = assign12960_e7298;
        locals.var_rth_dn0 = (locals.var_rth_dn0 * assign12960_e7297);
        locals.var_rth_dn2 = (locals.var_rth_dn2 * assign12960_e7297);
        locals.var_rth_dn4 = (locals.var_rth_dn4 * assign12960_e7297);
        locals.var_rth_dn5 = (locals.var_rth_dn5 * assign12960_e7297);
        locals.var_rth_dn6 = (locals.var_rth_dn6 * assign12960_e7297);
        locals.var_rth_dn7 = (locals.var_rth_dn7 * assign12960_e7297);
        locals.var_rth_dn8 = (locals.var_rth_dn8 * assign12960_e7297);
        locals.var_rth_dn9 = (locals.var_rth_dn9 * assign12960_e7297);
        locals.var_rth_dn10 = (locals.var_rth_dn10 * assign12960_e7297);
        locals.var_rth_dn11 = (locals.var_rth_dn11 * assign12960_e7297);
        locals.var_rth_dn14 = (locals.var_rth_dn14 * assign12960_e7297);
        locals.var_rth_rv = 0.0;

        let assign12970_e7302: f64 = (p.p7).powf(p.p327);
        let assign12970_e7303: f64 = (1.0 / assign12970_e7302);
        let assign12970_e7306: f64 = (locals.var_mfactor * locals.var_weff_nf);
        let assign12970_e7307: f64 = (assign12970_e7303 / assign12970_e7306);
        let assign12970_e7312: f64 = (locals.var_lg).powf(p.p318);
        let assign12970_e7313: f64 = (p.p317 / assign12970_e7312);
        let assign12970_e7314: f64 = (1.0 + assign12970_e7313);
        let assign12970_e7315: f64 = (assign12970_e7307 * assign12970_e7314);
        let assign12970_e7320: f64 = (locals.var_wg).powf(p.p316);
        let assign12970_e7321: f64 = (p.p315 / assign12970_e7320);
        let assign12970_e7322: f64 = (1.0 + assign12970_e7321);
        let assign12970_e7323: f64 = (assign12970_e7315 * assign12970_e7322);
        locals.var_rthtemp0 = assign12970_e7323;
        locals.var_rthtemp0_rv = 0.0;

        let assign12980_e7330: f64 = if ((p.p53 == 0.0) || (locals.var_uc_rth0 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard293 = assign12980_e7330;
        locals.var_guard293_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_23(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let (assign12990_e7334, assign12990_e7334_d_n0, assign12990_e7334_d_n2, assign12990_e7334_d_n4, assign12990_e7334_d_n5, assign12990_e7334_d_n6, assign12990_e7334_d_n7, assign12990_e7334_d_n8, assign12990_e7334_d_n9, assign12990_e7334_d_n10, assign12990_e7334_d_n11, assign12990_e7334_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn11, locals.var_cnst0over_dn14,)
    }
};
        locals.var_cnst0over = assign12990_e7334;
        locals.var_cnst0over_dn0 = assign12990_e7334_d_n0;
        locals.var_cnst0over_dn2 = assign12990_e7334_d_n2;
        locals.var_cnst0over_dn4 = assign12990_e7334_d_n4;
        locals.var_cnst0over_dn5 = assign12990_e7334_d_n5;
        locals.var_cnst0over_dn6 = assign12990_e7334_d_n6;
        locals.var_cnst0over_dn7 = assign12990_e7334_d_n7;
        locals.var_cnst0over_dn8 = assign12990_e7334_d_n8;
        locals.var_cnst0over_dn9 = assign12990_e7334_d_n9;
        locals.var_cnst0over_dn10 = assign12990_e7334_d_n10;
        locals.var_cnst0over_dn11 = assign12990_e7334_d_n11;
        locals.var_cnst0over_dn14 = assign12990_e7334_d_n14;
        locals.var_cnst0over_rv = 0.0;

        let (assign13000_e7338, assign13000_e7338_d_n0, assign13000_e7338_d_n2, assign13000_e7338_d_n4, assign13000_e7338_d_n5, assign13000_e7338_d_n6, assign13000_e7338_d_n7, assign13000_e7338_d_n8, assign13000_e7338_d_n9, assign13000_e7338_d_n10, assign13000_e7338_d_n11, assign13000_e7338_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn11, locals.var_cnst0overs_dn14,)
    }
};
        locals.var_cnst0overs = assign13000_e7338;
        locals.var_cnst0overs_dn0 = assign13000_e7338_d_n0;
        locals.var_cnst0overs_dn2 = assign13000_e7338_d_n2;
        locals.var_cnst0overs_dn4 = assign13000_e7338_d_n4;
        locals.var_cnst0overs_dn5 = assign13000_e7338_d_n5;
        locals.var_cnst0overs_dn6 = assign13000_e7338_d_n6;
        locals.var_cnst0overs_dn7 = assign13000_e7338_d_n7;
        locals.var_cnst0overs_dn8 = assign13000_e7338_d_n8;
        locals.var_cnst0overs_dn9 = assign13000_e7338_d_n9;
        locals.var_cnst0overs_dn10 = assign13000_e7338_d_n10;
        locals.var_cnst0overs_dn11 = assign13000_e7338_d_n11;
        locals.var_cnst0overs_dn14 = assign13000_e7338_d_n14;
        locals.var_cnst0overs_rv = 0.0;

        let (assign13010_e7344, assign13010_e7344_d_n0, assign13010_e7344_d_n2, assign13010_e7344_d_n4, assign13010_e7344_d_n5, assign13010_e7344_d_n6, assign13010_e7344_d_n7, assign13010_e7344_d_n8, assign13010_e7344_d_n9, assign13010_e7344_d_n10, assign13010_e7344_d_n11, assign13010_e7344_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign13010_e7340: f64 = ctx_temp;
        let assign13010_e7342: f64 = (assign13010_e7340 + p.p11);
        (assign13010_e7342, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    }
};
        locals.var_ttemp = assign13010_e7344;
        locals.var_ttemp_dn0 = assign13010_e7344_d_n0;
        locals.var_ttemp_dn2 = assign13010_e7344_d_n2;
        locals.var_ttemp_dn4 = assign13010_e7344_d_n4;
        locals.var_ttemp_dn5 = assign13010_e7344_d_n5;
        locals.var_ttemp_dn6 = assign13010_e7344_d_n6;
        locals.var_ttemp_dn7 = assign13010_e7344_d_n7;
        locals.var_ttemp_dn8 = assign13010_e7344_d_n8;
        locals.var_ttemp_dn9 = assign13010_e7344_d_n9;
        locals.var_ttemp_dn10 = assign13010_e7344_d_n10;
        locals.var_ttemp_dn11 = assign13010_e7344_d_n11;
        locals.var_ttemp_dn14 = assign13010_e7344_d_n14;
        locals.var_ttemp_rv = 0.0;

        let (assign13020_e7348, assign13020_e7348_d_n0, assign13020_e7348_d_n2, assign13020_e7348_d_n4, assign13020_e7348_d_n5, assign13020_e7348_d_n6, assign13020_e7348_d_n7, assign13020_e7348_d_n8, assign13020_e7348_d_n9, assign13020_e7348_d_n10, assign13020_e7348_d_n11, assign13020_e7348_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    } else {
        (locals.var_ttemp0, locals.var_ttemp0_dn0, locals.var_ttemp0_dn2, locals.var_ttemp0_dn4, locals.var_ttemp0_dn5, locals.var_ttemp0_dn6, locals.var_ttemp0_dn7, locals.var_ttemp0_dn8, locals.var_ttemp0_dn9, locals.var_ttemp0_dn10, locals.var_ttemp0_dn11, locals.var_ttemp0_dn14,)
    }
};
        locals.var_ttemp0 = assign13020_e7348;
        locals.var_ttemp0_dn0 = assign13020_e7348_d_n0;
        locals.var_ttemp0_dn2 = assign13020_e7348_d_n2;
        locals.var_ttemp0_dn4 = assign13020_e7348_d_n4;
        locals.var_ttemp0_dn5 = assign13020_e7348_d_n5;
        locals.var_ttemp0_dn6 = assign13020_e7348_d_n6;
        locals.var_ttemp0_dn7 = assign13020_e7348_d_n7;
        locals.var_ttemp0_dn8 = assign13020_e7348_d_n8;
        locals.var_ttemp0_dn9 = assign13020_e7348_d_n9;
        locals.var_ttemp0_dn10 = assign13020_e7348_d_n10;
        locals.var_ttemp0_dn11 = assign13020_e7348_d_n11;
        locals.var_ttemp0_dn14 = assign13020_e7348_d_n14;
        locals.var_ttemp0_rv = 0.0;

        let (assign13030_e7354, assign13030_e7354_d_n0, assign13030_e7354_d_n2, assign13030_e7354_d_n4, assign13030_e7354_d_n5, assign13030_e7354_d_n6, assign13030_e7354_d_n7, assign13030_e7354_d_n8, assign13030_e7354_d_n9, assign13030_e7354_d_n10, assign13030_e7354_d_n11, assign13030_e7354_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign13030_e7352: f64 = (locals.var_ttemp + locals.var_deltemp);
        (assign13030_e7352, (locals.var_ttemp_dn0 + locals.var_deltemp_dn0), (locals.var_ttemp_dn2 + locals.var_deltemp_dn2), (locals.var_ttemp_dn4 + locals.var_deltemp_dn4), (locals.var_ttemp_dn5 + locals.var_deltemp_dn5), (locals.var_ttemp_dn6 + locals.var_deltemp_dn6), (locals.var_ttemp_dn7 + locals.var_deltemp_dn7), (locals.var_ttemp_dn8 + locals.var_deltemp_dn8), (locals.var_ttemp_dn9 + locals.var_deltemp_dn9), (locals.var_ttemp_dn10 + locals.var_deltemp_dn10), (locals.var_ttemp_dn11 + locals.var_deltemp_dn11), (locals.var_ttemp_dn14 + locals.var_deltemp_dn14),)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    }
};
        locals.var_ttemp = assign13030_e7354;
        locals.var_ttemp_dn0 = assign13030_e7354_d_n0;
        locals.var_ttemp_dn2 = assign13030_e7354_d_n2;
        locals.var_ttemp_dn4 = assign13030_e7354_d_n4;
        locals.var_ttemp_dn5 = assign13030_e7354_d_n5;
        locals.var_ttemp_dn6 = assign13030_e7354_d_n6;
        locals.var_ttemp_dn7 = assign13030_e7354_d_n7;
        locals.var_ttemp_dn8 = assign13030_e7354_d_n8;
        locals.var_ttemp_dn9 = assign13030_e7354_d_n9;
        locals.var_ttemp_dn10 = assign13030_e7354_d_n10;
        locals.var_ttemp_dn11 = assign13030_e7354_d_n11;
        locals.var_ttemp_dn14 = assign13030_e7354_d_n14;
        locals.var_ttemp_rv = 0.0;

        let (assign13040_e7360, assign13040_e7360_d_n0, assign13040_e7360_d_n2, assign13040_e7360_d_n4, assign13040_e7360_d_n5, assign13040_e7360_d_n6, assign13040_e7360_d_n7, assign13040_e7360_d_n8, assign13040_e7360_d_n9, assign13040_e7360_d_n10, assign13040_e7360_d_n11, assign13040_e7360_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign13040_e7358: f64 = (locals.var_ttemp0 - locals.var_ktnom);
        (assign13040_e7358, locals.var_ttemp0_dn0, locals.var_ttemp0_dn2, locals.var_ttemp0_dn4, locals.var_ttemp0_dn5, locals.var_ttemp0_dn6, locals.var_ttemp0_dn7, locals.var_ttemp0_dn8, locals.var_ttemp0_dn9, locals.var_ttemp0_dn10, locals.var_ttemp0_dn11, locals.var_ttemp0_dn14,)
    } else {
        (locals.var_tdiff0, locals.var_tdiff0_dn0, locals.var_tdiff0_dn2, locals.var_tdiff0_dn4, locals.var_tdiff0_dn5, locals.var_tdiff0_dn6, locals.var_tdiff0_dn7, locals.var_tdiff0_dn8, locals.var_tdiff0_dn9, locals.var_tdiff0_dn10, locals.var_tdiff0_dn11, locals.var_tdiff0_dn14,)
    }
};
        locals.var_tdiff0 = assign13040_e7360;
        locals.var_tdiff0_dn0 = assign13040_e7360_d_n0;
        locals.var_tdiff0_dn2 = assign13040_e7360_d_n2;
        locals.var_tdiff0_dn4 = assign13040_e7360_d_n4;
        locals.var_tdiff0_dn5 = assign13040_e7360_d_n5;
        locals.var_tdiff0_dn6 = assign13040_e7360_d_n6;
        locals.var_tdiff0_dn7 = assign13040_e7360_d_n7;
        locals.var_tdiff0_dn8 = assign13040_e7360_d_n8;
        locals.var_tdiff0_dn9 = assign13040_e7360_d_n9;
        locals.var_tdiff0_dn10 = assign13040_e7360_d_n10;
        locals.var_tdiff0_dn11 = assign13040_e7360_d_n11;
        locals.var_tdiff0_dn14 = assign13040_e7360_d_n14;
        locals.var_tdiff0_rv = 0.0;

        let (assign13050_e7370, assign13050_e7370_d_n0, assign13050_e7370_d_n2, assign13050_e7370_d_n4, assign13050_e7370_d_n5, assign13050_e7370_d_n6, assign13050_e7370_d_n7, assign13050_e7370_d_n8, assign13050_e7370_d_n9, assign13050_e7370_d_n10, assign13050_e7370_d_n11, assign13050_e7370_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign13050_e7364: f64 = (locals.var_ttemp0 * locals.var_ttemp0);
        let assign13050_e7367: f64 = (locals.var_ktnom * locals.var_ktnom);
        let assign13050_e7368: f64 = (assign13050_e7364 - assign13050_e7367);
        (assign13050_e7368, ((locals.var_ttemp0_dn0 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn0)), ((locals.var_ttemp0_dn2 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn2)), ((locals.var_ttemp0_dn4 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn4)), ((locals.var_ttemp0_dn5 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn5)), ((locals.var_ttemp0_dn6 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn6)), ((locals.var_ttemp0_dn7 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn7)), ((locals.var_ttemp0_dn8 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn8)), ((locals.var_ttemp0_dn9 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn9)), ((locals.var_ttemp0_dn10 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn10)), ((locals.var_ttemp0_dn11 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn11)), ((locals.var_ttemp0_dn14 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn14)),)
    } else {
        (locals.var_tdiff0_2, locals.var_tdiff0_2_dn0, locals.var_tdiff0_2_dn2, locals.var_tdiff0_2_dn4, locals.var_tdiff0_2_dn5, locals.var_tdiff0_2_dn6, locals.var_tdiff0_2_dn7, locals.var_tdiff0_2_dn8, locals.var_tdiff0_2_dn9, locals.var_tdiff0_2_dn10, locals.var_tdiff0_2_dn11, locals.var_tdiff0_2_dn14,)
    }
};
        locals.var_tdiff0_2 = assign13050_e7370;
        locals.var_tdiff0_2_dn0 = assign13050_e7370_d_n0;
        locals.var_tdiff0_2_dn2 = assign13050_e7370_d_n2;
        locals.var_tdiff0_2_dn4 = assign13050_e7370_d_n4;
        locals.var_tdiff0_2_dn5 = assign13050_e7370_d_n5;
        locals.var_tdiff0_2_dn6 = assign13050_e7370_d_n6;
        locals.var_tdiff0_2_dn7 = assign13050_e7370_d_n7;
        locals.var_tdiff0_2_dn8 = assign13050_e7370_d_n8;
        locals.var_tdiff0_2_dn9 = assign13050_e7370_d_n9;
        locals.var_tdiff0_2_dn10 = assign13050_e7370_d_n10;
        locals.var_tdiff0_2_dn11 = assign13050_e7370_d_n11;
        locals.var_tdiff0_2_dn14 = assign13050_e7370_d_n14;
        locals.var_tdiff0_2_rv = 0.0;

        let (assign13060_e7376, assign13060_e7376_d_n0, assign13060_e7376_d_n2, assign13060_e7376_d_n4, assign13060_e7376_d_n5, assign13060_e7376_d_n6, assign13060_e7376_d_n7, assign13060_e7376_d_n8, assign13060_e7376_d_n9, assign13060_e7376_d_n10, assign13060_e7376_d_n11, assign13060_e7376_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign13060_e7374: f64 = (locals.var_ttemp - locals.var_ktnom);
        (assign13060_e7374, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    } else {
        (locals.var_tdiff, locals.var_tdiff_dn0, locals.var_tdiff_dn2, locals.var_tdiff_dn4, locals.var_tdiff_dn5, locals.var_tdiff_dn6, locals.var_tdiff_dn7, locals.var_tdiff_dn8, locals.var_tdiff_dn9, locals.var_tdiff_dn10, locals.var_tdiff_dn11, locals.var_tdiff_dn14,)
    }
};
        locals.var_tdiff = assign13060_e7376;
        locals.var_tdiff_dn0 = assign13060_e7376_d_n0;
        locals.var_tdiff_dn2 = assign13060_e7376_d_n2;
        locals.var_tdiff_dn4 = assign13060_e7376_d_n4;
        locals.var_tdiff_dn5 = assign13060_e7376_d_n5;
        locals.var_tdiff_dn6 = assign13060_e7376_d_n6;
        locals.var_tdiff_dn7 = assign13060_e7376_d_n7;
        locals.var_tdiff_dn8 = assign13060_e7376_d_n8;
        locals.var_tdiff_dn9 = assign13060_e7376_d_n9;
        locals.var_tdiff_dn10 = assign13060_e7376_d_n10;
        locals.var_tdiff_dn11 = assign13060_e7376_d_n11;
        locals.var_tdiff_dn14 = assign13060_e7376_d_n14;
        locals.var_tdiff_rv = 0.0;

        let (assign13070_e7386, assign13070_e7386_d_n0, assign13070_e7386_d_n2, assign13070_e7386_d_n4, assign13070_e7386_d_n5, assign13070_e7386_d_n6, assign13070_e7386_d_n7, assign13070_e7386_d_n8, assign13070_e7386_d_n9, assign13070_e7386_d_n10, assign13070_e7386_d_n11, assign13070_e7386_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign13070_e7380: f64 = (locals.var_ttemp * locals.var_ttemp);
        let assign13070_e7383: f64 = (locals.var_ktnom * locals.var_ktnom);
        let assign13070_e7384: f64 = (assign13070_e7380 - assign13070_e7383);
        (assign13070_e7384, ((locals.var_ttemp_dn0 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn0)), ((locals.var_ttemp_dn2 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn2)), ((locals.var_ttemp_dn4 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn4)), ((locals.var_ttemp_dn5 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn5)), ((locals.var_ttemp_dn6 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn6)), ((locals.var_ttemp_dn7 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn7)), ((locals.var_ttemp_dn8 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn8)), ((locals.var_ttemp_dn9 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn9)), ((locals.var_ttemp_dn10 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn10)), ((locals.var_ttemp_dn11 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn11)), ((locals.var_ttemp_dn14 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn14)),)
    } else {
        (locals.var_tdiff_2, locals.var_tdiff_2_dn0, locals.var_tdiff_2_dn2, locals.var_tdiff_2_dn4, locals.var_tdiff_2_dn5, locals.var_tdiff_2_dn6, locals.var_tdiff_2_dn7, locals.var_tdiff_2_dn8, locals.var_tdiff_2_dn9, locals.var_tdiff_2_dn10, locals.var_tdiff_2_dn11, locals.var_tdiff_2_dn14,)
    }
};
        locals.var_tdiff_2 = assign13070_e7386;
        locals.var_tdiff_2_dn0 = assign13070_e7386_d_n0;
        locals.var_tdiff_2_dn2 = assign13070_e7386_d_n2;
        locals.var_tdiff_2_dn4 = assign13070_e7386_d_n4;
        locals.var_tdiff_2_dn5 = assign13070_e7386_d_n5;
        locals.var_tdiff_2_dn6 = assign13070_e7386_d_n6;
        locals.var_tdiff_2_dn7 = assign13070_e7386_d_n7;
        locals.var_tdiff_2_dn8 = assign13070_e7386_d_n8;
        locals.var_tdiff_2_dn9 = assign13070_e7386_d_n9;
        locals.var_tdiff_2_dn10 = assign13070_e7386_d_n10;
        locals.var_tdiff_2_dn11 = assign13070_e7386_d_n11;
        locals.var_tdiff_2_dn14 = assign13070_e7386_d_n14;
        locals.var_tdiff_2_rv = 0.0;

        let (assign13080_e7392, assign13080_e7392_d_n0, assign13080_e7392_d_n2, assign13080_e7392_d_n4, assign13080_e7392_d_n5, assign13080_e7392_d_n6, assign13080_e7392_d_n7, assign13080_e7392_d_n8, assign13080_e7392_d_n9, assign13080_e7392_d_n10, assign13080_e7392_d_n11, assign13080_e7392_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign13080_e7390: f64 = (locals.var_ttemp / locals.var_ktnom);
        (assign13080_e7390, (locals.var_ttemp_dn0 / locals.var_ktnom), (locals.var_ttemp_dn2 / locals.var_ktnom), (locals.var_ttemp_dn4 / locals.var_ktnom), (locals.var_ttemp_dn5 / locals.var_ktnom), (locals.var_ttemp_dn6 / locals.var_ktnom), (locals.var_ttemp_dn7 / locals.var_ktnom), (locals.var_ttemp_dn8 / locals.var_ktnom), (locals.var_ttemp_dn9 / locals.var_ktnom), (locals.var_ttemp_dn10 / locals.var_ktnom), (locals.var_ttemp_dn11 / locals.var_ktnom), (locals.var_ttemp_dn14 / locals.var_ktnom),)
    } else {
        (locals.var_tratio, locals.var_tratio_dn0, locals.var_tratio_dn2, locals.var_tratio_dn4, locals.var_tratio_dn5, locals.var_tratio_dn6, locals.var_tratio_dn7, locals.var_tratio_dn8, locals.var_tratio_dn9, locals.var_tratio_dn10, locals.var_tratio_dn11, locals.var_tratio_dn14,)
    }
};
        locals.var_tratio = assign13080_e7392;
        locals.var_tratio_dn0 = assign13080_e7392_d_n0;
        locals.var_tratio_dn2 = assign13080_e7392_d_n2;
        locals.var_tratio_dn4 = assign13080_e7392_d_n4;
        locals.var_tratio_dn5 = assign13080_e7392_d_n5;
        locals.var_tratio_dn6 = assign13080_e7392_d_n6;
        locals.var_tratio_dn7 = assign13080_e7392_d_n7;
        locals.var_tratio_dn8 = assign13080_e7392_d_n8;
        locals.var_tratio_dn9 = assign13080_e7392_d_n9;
        locals.var_tratio_dn10 = assign13080_e7392_d_n10;
        locals.var_tratio_dn11 = assign13080_e7392_d_n11;
        locals.var_tratio_dn14 = assign13080_e7392_d_n14;
        locals.var_tratio_rv = 0.0;

        let (assign13090_e7397, assign13090_e7397_d_n0, assign13090_e7397_d_n2, assign13090_e7397_d_n4, assign13090_e7397_d_n5, assign13090_e7397_d_n6, assign13090_e7397_d_n7, assign13090_e7397_d_n8, assign13090_e7397_d_n9, assign13090_e7397_d_n10, assign13090_e7397_d_n11, assign13090_e7397_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign13090_e7395: f64 = (locals.var_tratio).ln();
        (assign13090_e7395, (locals.var_tratio_dn0 / locals.var_tratio), (locals.var_tratio_dn2 / locals.var_tratio), (locals.var_tratio_dn4 / locals.var_tratio), (locals.var_tratio_dn5 / locals.var_tratio), (locals.var_tratio_dn6 / locals.var_tratio), (locals.var_tratio_dn7 / locals.var_tratio), (locals.var_tratio_dn8 / locals.var_tratio), (locals.var_tratio_dn9 / locals.var_tratio), (locals.var_tratio_dn10 / locals.var_tratio), (locals.var_tratio_dn11 / locals.var_tratio), (locals.var_tratio_dn14 / locals.var_tratio),)
    } else {
        (locals.var_log_tratio, locals.var_log_tratio_dn0, locals.var_log_tratio_dn2, locals.var_log_tratio_dn4, locals.var_log_tratio_dn5, locals.var_log_tratio_dn6, locals.var_log_tratio_dn7, locals.var_log_tratio_dn8, locals.var_log_tratio_dn9, locals.var_log_tratio_dn10, locals.var_log_tratio_dn11, locals.var_log_tratio_dn14,)
    }
};
        locals.var_log_tratio = assign13090_e7397;
        locals.var_log_tratio_dn0 = assign13090_e7397_d_n0;
        locals.var_log_tratio_dn2 = assign13090_e7397_d_n2;
        locals.var_log_tratio_dn4 = assign13090_e7397_d_n4;
        locals.var_log_tratio_dn5 = assign13090_e7397_d_n5;
        locals.var_log_tratio_dn6 = assign13090_e7397_d_n6;
        locals.var_log_tratio_dn7 = assign13090_e7397_d_n7;
        locals.var_log_tratio_dn8 = assign13090_e7397_d_n8;
        locals.var_log_tratio_dn9 = assign13090_e7397_d_n9;
        locals.var_log_tratio_dn10 = assign13090_e7397_d_n10;
        locals.var_log_tratio_dn11 = assign13090_e7397_d_n11;
        locals.var_log_tratio_dn14 = assign13090_e7397_d_n14;
        locals.var_log_tratio_rv = 0.0;

        let (assign13100_e7409, assign13100_e7409_d_n0, assign13100_e7409_d_n2, assign13100_e7409_d_n4, assign13100_e7409_d_n5, assign13100_e7409_d_n6, assign13100_e7409_d_n7, assign13100_e7409_d_n8, assign13100_e7409_d_n9, assign13100_e7409_d_n10, assign13100_e7409_d_n11, assign13100_e7409_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign13100_e7402: f64 = (locals.var_uc_bgtmp1 * locals.var_tdiff);
        let assign13100_e7403: f64 = (locals.var_egtnom - assign13100_e7402);
        let assign13100_e7406: f64 = (locals.var_uc_bgtmp2 * locals.var_tdiff_2);
        let assign13100_e7407: f64 = (assign13100_e7403 - assign13100_e7406);
        (assign13100_e7407, ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn0)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn0)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn2)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn2)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn4)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn4)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn5)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn5)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn6)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn6)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn7)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn7)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn8)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn8)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn9)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn9)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn10)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn10)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn11)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn11)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn14)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn14)),)
    } else {
        (locals.var_eg, locals.var_eg_dn0, locals.var_eg_dn2, locals.var_eg_dn4, locals.var_eg_dn5, locals.var_eg_dn6, locals.var_eg_dn7, locals.var_eg_dn8, locals.var_eg_dn9, locals.var_eg_dn10, locals.var_eg_dn11, locals.var_eg_dn14,)
    }
};
        locals.var_eg = assign13100_e7409;
        locals.var_eg_dn0 = assign13100_e7409_d_n0;
        locals.var_eg_dn2 = assign13100_e7409_d_n2;
        locals.var_eg_dn4 = assign13100_e7409_d_n4;
        locals.var_eg_dn5 = assign13100_e7409_d_n5;
        locals.var_eg_dn6 = assign13100_e7409_d_n6;
        locals.var_eg_dn7 = assign13100_e7409_d_n7;
        locals.var_eg_dn8 = assign13100_e7409_d_n8;
        locals.var_eg_dn9 = assign13100_e7409_d_n9;
        locals.var_eg_dn10 = assign13100_e7409_d_n10;
        locals.var_eg_dn11 = assign13100_e7409_d_n11;
        locals.var_eg_dn14 = assign13100_e7409_d_n14;
        locals.var_eg_rv = 0.0;

        let (assign13110_e7414, assign13110_e7414_d_n0, assign13110_e7414_d_n2, assign13110_e7414_d_n4, assign13110_e7414_d_n5, assign13110_e7414_d_n6, assign13110_e7414_d_n7, assign13110_e7414_d_n8, assign13110_e7414_d_n9, assign13110_e7414_d_n10, assign13110_e7414_d_n11, assign13110_e7414_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign13110_e7412: f64 = (locals.var_eg).sqrt();
        (assign13110_e7412, (locals.var_eg_dn0 / (2.0 * assign13110_e7412)), (locals.var_eg_dn2 / (2.0 * assign13110_e7412)), (locals.var_eg_dn4 / (2.0 * assign13110_e7412)), (locals.var_eg_dn5 / (2.0 * assign13110_e7412)), (locals.var_eg_dn6 / (2.0 * assign13110_e7412)), (locals.var_eg_dn7 / (2.0 * assign13110_e7412)), (locals.var_eg_dn8 / (2.0 * assign13110_e7412)), (locals.var_eg_dn9 / (2.0 * assign13110_e7412)), (locals.var_eg_dn10 / (2.0 * assign13110_e7412)), (locals.var_eg_dn11 / (2.0 * assign13110_e7412)), (locals.var_eg_dn14 / (2.0 * assign13110_e7412)),)
    } else {
        (locals.var_sqrt_eg, locals.var_sqrt_eg_dn0, locals.var_sqrt_eg_dn2, locals.var_sqrt_eg_dn4, locals.var_sqrt_eg_dn5, locals.var_sqrt_eg_dn6, locals.var_sqrt_eg_dn7, locals.var_sqrt_eg_dn8, locals.var_sqrt_eg_dn9, locals.var_sqrt_eg_dn10, locals.var_sqrt_eg_dn11, locals.var_sqrt_eg_dn14,)
    }
};
        locals.var_sqrt_eg = assign13110_e7414;
        locals.var_sqrt_eg_dn0 = assign13110_e7414_d_n0;
        locals.var_sqrt_eg_dn2 = assign13110_e7414_d_n2;
        locals.var_sqrt_eg_dn4 = assign13110_e7414_d_n4;
        locals.var_sqrt_eg_dn5 = assign13110_e7414_d_n5;
        locals.var_sqrt_eg_dn6 = assign13110_e7414_d_n6;
        locals.var_sqrt_eg_dn7 = assign13110_e7414_d_n7;
        locals.var_sqrt_eg_dn8 = assign13110_e7414_d_n8;
        locals.var_sqrt_eg_dn9 = assign13110_e7414_d_n9;
        locals.var_sqrt_eg_dn10 = assign13110_e7414_d_n10;
        locals.var_sqrt_eg_dn11 = assign13110_e7414_d_n11;
        locals.var_sqrt_eg_dn14 = assign13110_e7414_d_n14;
        locals.var_sqrt_eg_rv = 0.0;

        let (assign13120_e7420, assign13120_e7420_d_n0, assign13120_e7420_d_n2, assign13120_e7420_d_n4, assign13120_e7420_d_n5, assign13120_e7420_d_n6, assign13120_e7420_d_n7, assign13120_e7420_d_n8, assign13120_e7420_d_n9, assign13120_e7420_d_n10, assign13120_e7420_d_n11, assign13120_e7420_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign13120_e7418: f64 = (1.0 / locals.var_ttemp);
        (assign13120_e7418, (-(locals.var_ttemp_dn0 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn2 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn4 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn5 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn6 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn7 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn8 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn9 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn10 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn11 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn14 / (locals.var_ttemp * locals.var_ttemp))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign13120_e7420;
        locals.var_t1_dn0 = assign13120_e7420_d_n0;
        locals.var_t1_dn2 = assign13120_e7420_d_n2;
        locals.var_t1_dn4 = assign13120_e7420_d_n4;
        locals.var_t1_dn5 = assign13120_e7420_d_n5;
        locals.var_t1_dn6 = assign13120_e7420_d_n6;
        locals.var_t1_dn7 = assign13120_e7420_d_n7;
        locals.var_t1_dn8 = assign13120_e7420_d_n8;
        locals.var_t1_dn9 = assign13120_e7420_d_n9;
        locals.var_t1_dn10 = assign13120_e7420_d_n10;
        locals.var_t1_dn11 = assign13120_e7420_d_n11;
        locals.var_t1_dn14 = assign13120_e7420_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign13130_e7426, assign13130_e7426_d_n0, assign13130_e7426_d_n2, assign13130_e7426_d_n4, assign13130_e7426_d_n5, assign13130_e7426_d_n6, assign13130_e7426_d_n7, assign13130_e7426_d_n8, assign13130_e7426_d_n9, assign13130_e7426_d_n10, assign13130_e7426_d_n11, assign13130_e7426_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign13130_e7424: f64 = (1.0 / locals.var_ktnom);
        (assign13130_e7424, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign13130_e7426;
        locals.var_t2_dn0 = assign13130_e7426_d_n0;
        locals.var_t2_dn2 = assign13130_e7426_d_n2;
        locals.var_t2_dn4 = assign13130_e7426_d_n4;
        locals.var_t2_dn5 = assign13130_e7426_d_n5;
        locals.var_t2_dn6 = assign13130_e7426_d_n6;
        locals.var_t2_dn7 = assign13130_e7426_d_n7;
        locals.var_t2_dn8 = assign13130_e7426_d_n8;
        locals.var_t2_dn9 = assign13130_e7426_d_n9;
        locals.var_t2_dn10 = assign13130_e7426_d_n10;
        locals.var_t2_dn11 = assign13130_e7426_d_n11;
        locals.var_t2_dn14 = assign13130_e7426_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign13140_e7448, assign13140_e7448_d_n0, assign13140_e7448_d_n2, assign13140_e7448_d_n4, assign13140_e7448_d_n5, assign13140_e7448_d_n6, assign13140_e7448_d_n7, assign13140_e7448_d_n8, assign13140_e7448_d_n9, assign13140_e7448_d_n10, assign13140_e7448_d_n11, assign13140_e7448_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign13140_e7430: f64 = (locals.var_egtnom + p.p259);
        let assign13140_e7434: f64 = (locals.var_t1 - locals.var_t2);
        let assign13140_e7435: f64 = (p.p260 * assign13140_e7434);
        let assign13140_e7436: f64 = (assign13140_e7430 + assign13140_e7435);
        let assign13140_e7440: f64 = (locals.var_t1 * locals.var_t1);
        let assign13140_e7443: f64 = (locals.var_t2 * locals.var_t2);
        let assign13140_e7444: f64 = (assign13140_e7440 - assign13140_e7443);
        let assign13140_e7445: f64 = (p.p261 * assign13140_e7444);
        let assign13140_e7446: f64 = (assign13140_e7436 + assign13140_e7445);
        (assign13140_e7446, ((p.p260 * (locals.var_t1_dn0 - locals.var_t2_dn0)) + (p.p261 * (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) - ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0))))), ((p.p260 * (locals.var_t1_dn2 - locals.var_t2_dn2)) + (p.p261 * (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) - ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2))))), ((p.p260 * (locals.var_t1_dn4 - locals.var_t2_dn4)) + (p.p261 * (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) - ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4))))), ((p.p260 * (locals.var_t1_dn5 - locals.var_t2_dn5)) + (p.p261 * (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) - ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5))))), ((p.p260 * (locals.var_t1_dn6 - locals.var_t2_dn6)) + (p.p261 * (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) - ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6))))), ((p.p260 * (locals.var_t1_dn7 - locals.var_t2_dn7)) + (p.p261 * (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) - ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7))))), ((p.p260 * (locals.var_t1_dn8 - locals.var_t2_dn8)) + (p.p261 * (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) - ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8))))), ((p.p260 * (locals.var_t1_dn9 - locals.var_t2_dn9)) + (p.p261 * (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) - ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9))))), ((p.p260 * (locals.var_t1_dn10 - locals.var_t2_dn10)) + (p.p261 * (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) - ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10))))), ((p.p260 * (locals.var_t1_dn11 - locals.var_t2_dn11)) + (p.p261 * (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) - ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11))))), ((p.p260 * (locals.var_t1_dn14 - locals.var_t2_dn14)) + (p.p261 * (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) - ((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign13140_e7448;
        locals.var_t3_dn0 = assign13140_e7448_d_n0;
        locals.var_t3_dn2 = assign13140_e7448_d_n2;
        locals.var_t3_dn4 = assign13140_e7448_d_n4;
        locals.var_t3_dn5 = assign13140_e7448_d_n5;
        locals.var_t3_dn6 = assign13140_e7448_d_n6;
        locals.var_t3_dn7 = assign13140_e7448_d_n7;
        locals.var_t3_dn8 = assign13140_e7448_d_n8;
        locals.var_t3_dn9 = assign13140_e7448_d_n9;
        locals.var_t3_dn10 = assign13140_e7448_d_n10;
        locals.var_t3_dn11 = assign13140_e7448_d_n11;
        locals.var_t3_dn14 = assign13140_e7448_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign13150_e7453, assign13150_e7453_d_n0, assign13150_e7453_d_n2, assign13150_e7453_d_n4, assign13150_e7453_d_n5, assign13150_e7453_d_n6, assign13150_e7453_d_n7, assign13150_e7453_d_n8, assign13150_e7453_d_n9, assign13150_e7453_d_n10, assign13150_e7453_d_n11, assign13150_e7453_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign13150_e7451: f64 = (locals.var_t3).sqrt();
        (assign13150_e7451, (locals.var_t3_dn0 / (2.0 * assign13150_e7451)), (locals.var_t3_dn2 / (2.0 * assign13150_e7451)), (locals.var_t3_dn4 / (2.0 * assign13150_e7451)), (locals.var_t3_dn5 / (2.0 * assign13150_e7451)), (locals.var_t3_dn6 / (2.0 * assign13150_e7451)), (locals.var_t3_dn7 / (2.0 * assign13150_e7451)), (locals.var_t3_dn8 / (2.0 * assign13150_e7451)), (locals.var_t3_dn9 / (2.0 * assign13150_e7451)), (locals.var_t3_dn10 / (2.0 * assign13150_e7451)), (locals.var_t3_dn11 / (2.0 * assign13150_e7451)), (locals.var_t3_dn14 / (2.0 * assign13150_e7451)),)
    } else {
        (locals.var_egp12, locals.var_egp12_dn0, locals.var_egp12_dn2, locals.var_egp12_dn4, locals.var_egp12_dn5, locals.var_egp12_dn6, locals.var_egp12_dn7, locals.var_egp12_dn8, locals.var_egp12_dn9, locals.var_egp12_dn10, locals.var_egp12_dn11, locals.var_egp12_dn14,)
    }
};
        locals.var_egp12 = assign13150_e7453;
        locals.var_egp12_dn0 = assign13150_e7453_d_n0;
        locals.var_egp12_dn2 = assign13150_e7453_d_n2;
        locals.var_egp12_dn4 = assign13150_e7453_d_n4;
        locals.var_egp12_dn5 = assign13150_e7453_d_n5;
        locals.var_egp12_dn6 = assign13150_e7453_d_n6;
        locals.var_egp12_dn7 = assign13150_e7453_d_n7;
        locals.var_egp12_dn8 = assign13150_e7453_d_n8;
        locals.var_egp12_dn9 = assign13150_e7453_d_n9;
        locals.var_egp12_dn10 = assign13150_e7453_d_n10;
        locals.var_egp12_dn11 = assign13150_e7453_d_n11;
        locals.var_egp12_dn14 = assign13150_e7453_d_n14;
        locals.var_egp12_rv = 0.0;

        let (assign13160_e7459, assign13160_e7459_d_n0, assign13160_e7459_d_n2, assign13160_e7459_d_n4, assign13160_e7459_d_n5, assign13160_e7459_d_n6, assign13160_e7459_d_n7, assign13160_e7459_d_n8, assign13160_e7459_d_n9, assign13160_e7459_d_n10, assign13160_e7459_d_n11, assign13160_e7459_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign13160_e7457: f64 = (locals.var_t3 * locals.var_egp12);
        (assign13160_e7457, ((locals.var_t3_dn0 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn0)), ((locals.var_t3_dn2 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn2)), ((locals.var_t3_dn4 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn4)), ((locals.var_t3_dn5 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn5)), ((locals.var_t3_dn6 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn6)), ((locals.var_t3_dn7 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn7)), ((locals.var_t3_dn8 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn8)), ((locals.var_t3_dn9 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn9)), ((locals.var_t3_dn10 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn10)), ((locals.var_t3_dn11 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn11)), ((locals.var_t3_dn14 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn14)),)
    } else {
        (locals.var_egp32, locals.var_egp32_dn0, locals.var_egp32_dn2, locals.var_egp32_dn4, locals.var_egp32_dn5, locals.var_egp32_dn6, locals.var_egp32_dn7, locals.var_egp32_dn8, locals.var_egp32_dn9, locals.var_egp32_dn10, locals.var_egp32_dn11, locals.var_egp32_dn14,)
    }
};
        locals.var_egp32 = assign13160_e7459;
        locals.var_egp32_dn0 = assign13160_e7459_d_n0;
        locals.var_egp32_dn2 = assign13160_e7459_d_n2;
        locals.var_egp32_dn4 = assign13160_e7459_d_n4;
        locals.var_egp32_dn5 = assign13160_e7459_d_n5;
        locals.var_egp32_dn6 = assign13160_e7459_d_n6;
        locals.var_egp32_dn7 = assign13160_e7459_d_n7;
        locals.var_egp32_dn8 = assign13160_e7459_d_n8;
        locals.var_egp32_dn9 = assign13160_e7459_d_n9;
        locals.var_egp32_dn10 = assign13160_e7459_d_n10;
        locals.var_egp32_dn11 = assign13160_e7459_d_n11;
        locals.var_egp32_dn14 = assign13160_e7459_d_n14;
        locals.var_egp32_rv = 0.0;

        let (assign13170_e7467, assign13170_e7467_d_n0, assign13170_e7467_d_n2, assign13170_e7467_d_n4, assign13170_e7467_d_n5, assign13170_e7467_d_n6, assign13170_e7467_d_n7, assign13170_e7467_d_n8, assign13170_e7467_d_n9, assign13170_e7467_d_n10, assign13170_e7467_d_n11, assign13170_e7467_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign13170_e7464: f64 = (1.3806226e-23 * locals.var_ttemp);
        let assign13170_e7465: f64 = (1.6021918e-19 / assign13170_e7464);
        (assign13170_e7465, (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn0)) / (assign13170_e7464 * assign13170_e7464))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn2)) / (assign13170_e7464 * assign13170_e7464))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn4)) / (assign13170_e7464 * assign13170_e7464))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn5)) / (assign13170_e7464 * assign13170_e7464))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn6)) / (assign13170_e7464 * assign13170_e7464))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn7)) / (assign13170_e7464 * assign13170_e7464))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn8)) / (assign13170_e7464 * assign13170_e7464))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn9)) / (assign13170_e7464 * assign13170_e7464))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn10)) / (assign13170_e7464 * assign13170_e7464))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn11)) / (assign13170_e7464 * assign13170_e7464))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn14)) / (assign13170_e7464 * assign13170_e7464))),)
    } else {
        (locals.var_beta, locals.var_beta_dn0, locals.var_beta_dn2, locals.var_beta_dn4, locals.var_beta_dn5, locals.var_beta_dn6, locals.var_beta_dn7, locals.var_beta_dn8, locals.var_beta_dn9, locals.var_beta_dn10, locals.var_beta_dn11, locals.var_beta_dn14,)
    }
};
        locals.var_beta = assign13170_e7467;
        locals.var_beta_dn0 = assign13170_e7467_d_n0;
        locals.var_beta_dn2 = assign13170_e7467_d_n2;
        locals.var_beta_dn4 = assign13170_e7467_d_n4;
        locals.var_beta_dn5 = assign13170_e7467_d_n5;
        locals.var_beta_dn6 = assign13170_e7467_d_n6;
        locals.var_beta_dn7 = assign13170_e7467_d_n7;
        locals.var_beta_dn8 = assign13170_e7467_d_n8;
        locals.var_beta_dn9 = assign13170_e7467_d_n9;
        locals.var_beta_dn10 = assign13170_e7467_d_n10;
        locals.var_beta_dn11 = assign13170_e7467_d_n11;
        locals.var_beta_dn14 = assign13170_e7467_d_n14;
        locals.var_beta_rv = 0.0;

        let (assign13180_e7473, assign13180_e7473_d_n0, assign13180_e7473_d_n2, assign13180_e7473_d_n4, assign13180_e7473_d_n5, assign13180_e7473_d_n6, assign13180_e7473_d_n7, assign13180_e7473_d_n8, assign13180_e7473_d_n9, assign13180_e7473_d_n10, assign13180_e7473_d_n11, assign13180_e7473_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign13180_e7471: f64 = (1.0 / locals.var_beta);
        (assign13180_e7471, (-(locals.var_beta_dn0 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn2 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn4 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn5 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn6 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn7 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn8 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn9 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn10 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn11 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn14 / (locals.var_beta * locals.var_beta))),)
    } else {
        (locals.var_beta_inv, locals.var_beta_inv_dn0, locals.var_beta_inv_dn2, locals.var_beta_inv_dn4, locals.var_beta_inv_dn5, locals.var_beta_inv_dn6, locals.var_beta_inv_dn7, locals.var_beta_inv_dn8, locals.var_beta_inv_dn9, locals.var_beta_inv_dn10, locals.var_beta_inv_dn11, locals.var_beta_inv_dn14,)
    }
};
        locals.var_beta_inv = assign13180_e7473;
        locals.var_beta_inv_dn0 = assign13180_e7473_d_n0;
        locals.var_beta_inv_dn2 = assign13180_e7473_d_n2;
        locals.var_beta_inv_dn4 = assign13180_e7473_d_n4;
        locals.var_beta_inv_dn5 = assign13180_e7473_d_n5;
        locals.var_beta_inv_dn6 = assign13180_e7473_d_n6;
        locals.var_beta_inv_dn7 = assign13180_e7473_d_n7;
        locals.var_beta_inv_dn8 = assign13180_e7473_d_n8;
        locals.var_beta_inv_dn9 = assign13180_e7473_d_n9;
        locals.var_beta_inv_dn10 = assign13180_e7473_d_n10;
        locals.var_beta_inv_dn11 = assign13180_e7473_d_n11;
        locals.var_beta_inv_dn14 = assign13180_e7473_d_n14;
        locals.var_beta_inv_rv = 0.0;

        let (assign13190_e7479, assign13190_e7479_d_n0, assign13190_e7479_d_n2, assign13190_e7479_d_n4, assign13190_e7479_d_n5, assign13190_e7479_d_n6, assign13190_e7479_d_n7, assign13190_e7479_d_n8, assign13190_e7479_d_n9, assign13190_e7479_d_n10, assign13190_e7479_d_n11, assign13190_e7479_d_n14,) = {
    if (locals.var_guard293 != 0.0) {
        let assign13190_e7477: f64 = (locals.var_beta * locals.var_beta);
        (assign13190_e7477, ((locals.var_beta_dn0 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn0)), ((locals.var_beta_dn2 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn2)), ((locals.var_beta_dn4 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn4)), ((locals.var_beta_dn5 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn5)), ((locals.var_beta_dn6 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn6)), ((locals.var_beta_dn7 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn7)), ((locals.var_beta_dn8 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn8)), ((locals.var_beta_dn9 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn9)), ((locals.var_beta_dn10 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn10)), ((locals.var_beta_dn11 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn11)), ((locals.var_beta_dn14 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn14)),)
    } else {
        (locals.var_beta2, locals.var_beta2_dn0, locals.var_beta2_dn2, locals.var_beta2_dn4, locals.var_beta2_dn5, locals.var_beta2_dn6, locals.var_beta2_dn7, locals.var_beta2_dn8, locals.var_beta2_dn9, locals.var_beta2_dn10, locals.var_beta2_dn11, locals.var_beta2_dn14,)
    }
};
        locals.var_beta2 = assign13190_e7479;
        locals.var_beta2_dn0 = assign13190_e7479_d_n0;
        locals.var_beta2_dn2 = assign13190_e7479_d_n2;
        locals.var_beta2_dn4 = assign13190_e7479_d_n4;
        locals.var_beta2_dn5 = assign13190_e7479_d_n5;
        locals.var_beta2_dn6 = assign13190_e7479_d_n6;
        locals.var_beta2_dn7 = assign13190_e7479_d_n7;
        locals.var_beta2_dn8 = assign13190_e7479_d_n8;
        locals.var_beta2_dn9 = assign13190_e7479_d_n9;
        locals.var_beta2_dn10 = assign13190_e7479_d_n10;
        locals.var_beta2_dn11 = assign13190_e7479_d_n11;
        locals.var_beta2_dn14 = assign13190_e7479_d_n14;
        locals.var_beta2_rv = 0.0;

        let (assign13200_e7487,) = {
    if (locals.var_guard293 != 0.0) {
        let assign13200_e7484: f64 = (1.3806226e-23 * locals.var_ktnom);
        let assign13200_e7485: f64 = (1.6021918e-19 / assign13200_e7484);
        (assign13200_e7485,)
    } else {
        (locals.var_betatnom,)
    }
};
        locals.var_betatnom = assign13200_e7487;
        locals.var_betatnom_rv = 0.0;

    }
}
