#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        p: &Parameters,
        var_a_vdcctc: f64,
        var_a_vdcctc_db0: f64,
        var_a_vdcctc_db1: f64,
        var_a_vdcctc_dn0: f64,
        var_a_vdcctc_dn1: f64,
        var_a_vdcctc_dn10: f64,
        var_a_vdcctc_dn11: f64,
        var_a_vdcctc_dn2: f64,
        var_a_vdcctc_dn3: f64,
        var_a_vdcctc_dn4: f64,
        var_a_vdcctc_dn5: f64,
        var_a_vdcctc_dn6: f64,
        var_a_vdcctc_dn7: f64,
        var_a_vdcctc_dn8: f64,
        var_a_vdcctc_dn9: f64,
        var_bjc: f64,
        var_bjc_db0: f64,
        var_bjc_db1: f64,
        var_bjc_dn0: f64,
        var_bjc_dn1: f64,
        var_bjc_dn10: f64,
        var_bjc_dn11: f64,
        var_bjc_dn2: f64,
        var_bjc_dn3: f64,
        var_bjc_dn4: f64,
        var_bjc_dn5: f64,
        var_bjc_dn6: f64,
        var_bjc_dn7: f64,
        var_bjc_dn8: f64,
        var_bjc_dn9: f64,
        var_cjc_t: f64,
        var_cjc_t_db0: f64,
        var_cjc_t_db1: f64,
        var_cjc_t_dn0: f64,
        var_cjc_t_dn1: f64,
        var_cjc_t_dn10: f64,
        var_cjc_t_dn11: f64,
        var_cjc_t_dn2: f64,
        var_cjc_t_dn3: f64,
        var_cjc_t_dn4: f64,
        var_cjc_t_dn5: f64,
        var_cjc_t_dn6: f64,
        var_cjc_t_dn7: f64,
        var_cjc_t_dn8: f64,
        var_cjc_t_dn9: f64,
        var_cjs_t: f64,
        var_cjs_t_db0: f64,
        var_cjs_t_db1: f64,
        var_cjs_t_dn0: f64,
        var_cjs_t_dn1: f64,
        var_cjs_t_dn10: f64,
        var_cjs_t_dn11: f64,
        var_cjs_t_dn2: f64,
        var_cjs_t_dn3: f64,
        var_cjs_t_dn4: f64,
        var_cjs_t_dn5: f64,
        var_cjs_t_dn6: f64,
        var_cjs_t_dn7: f64,
        var_cjs_t_dn8: f64,
        var_cjs_t_dn9: f64,
        var_ik_t: f64,
        var_is_t: f64,
        var_is_t_db0: f64,
        var_is_t_db1: f64,
        var_is_t_dn0: f64,
        var_is_t_dn1: f64,
        var_is_t_dn10: f64,
        var_is_t_dn11: f64,
        var_is_t_dn2: f64,
        var_is_t_dn3: f64,
        var_is_t_dn4: f64,
        var_is_t_dn5: f64,
        var_is_t_dn6: f64,
        var_is_t_dn7: f64,
        var_is_t_dn8: f64,
        var_is_t_dn9: f64,
        var_nbex: f64,
        var_nbex_db0: f64,
        var_nbex_db1: f64,
        var_nbex_dn0: f64,
        var_nbex_dn1: f64,
        var_nbex_dn10: f64,
        var_nbex_dn11: f64,
        var_nbex_dn2: f64,
        var_nbex_dn3: f64,
        var_nbex_dn4: f64,
        var_nbex_dn5: f64,
        var_nbex_dn6: f64,
        var_nbex_dn7: f64,
        var_nbex_dn8: f64,
        var_nbex_dn9: f64,
        var_p0star: f64,
        var_p0star_db0: f64,
        var_p0star_db1: f64,
        var_p0star_dn0: f64,
        var_p0star_dn1: f64,
        var_p0star_dn10: f64,
        var_p0star_dn11: f64,
        var_p0star_dn2: f64,
        var_p0star_dn3: f64,
        var_p0star_dn4: f64,
        var_p0star_dn5: f64,
        var_p0star_dn6: f64,
        var_p0star_dn7: f64,
        var_p0star_dn8: f64,
        var_p0star_dn9: f64,
        var_pw: f64,
        var_pw_db0: f64,
        var_pw_db1: f64,
        var_pw_dn0: f64,
        var_pw_dn1: f64,
        var_pw_dn10: f64,
        var_pw_dn11: f64,
        var_pw_dn2: f64,
        var_pw_dn3: f64,
        var_pw_dn4: f64,
        var_pw_dn5: f64,
        var_pw_dn6: f64,
        var_pw_dn7: f64,
        var_pw_dn8: f64,
        var_pw_dn9: f64,
        var_pwex: f64,
        var_pwex_db0: f64,
        var_pwex_db1: f64,
        var_pwex_dn0: f64,
        var_pwex_dn1: f64,
        var_pwex_dn10: f64,
        var_pwex_dn11: f64,
        var_pwex_dn2: f64,
        var_pwex_dn3: f64,
        var_pwex_dn4: f64,
        var_pwex_dn5: f64,
        var_pwex_dn6: f64,
        var_pwex_dn7: f64,
        var_pwex_dn8: f64,
        var_pwex_dn9: f64,
        var_qb0: f64,
        var_rcv_t: f64,
        var_taub_t: f64,
        var_taue_t: f64,
        var_taur_t: f64,
        var_tepi_t: f64,
        var_vb1c4: f64,
        var_vb1c4_db0: f64,
        var_vb1c4_db1: f64,
        var_vb1c4_dn0: f64,
        var_vb1c4_dn1: f64,
        var_vb1c4_dn10: f64,
        var_vb1c4_dn11: f64,
        var_vb1c4_dn2: f64,
        var_vb1c4_dn3: f64,
        var_vb1c4_dn4: f64,
        var_vb1c4_dn5: f64,
        var_vb1c4_dn6: f64,
        var_vb1c4_dn7: f64,
        var_vb1c4_dn8: f64,
        var_vb1c4_dn9: f64,
        var_vb2e1: f64,
        var_vb2e1_db0: f64,
        var_vb2e1_db1: f64,
        var_vb2e1_dn0: f64,
        var_vb2e1_dn1: f64,
        var_vb2e1_dn10: f64,
        var_vb2e1_dn11: f64,
        var_vb2e1_dn2: f64,
        var_vb2e1_dn3: f64,
        var_vb2e1_dn4: f64,
        var_vb2e1_dn5: f64,
        var_vb2e1_dn6: f64,
        var_vb2e1_dn7: f64,
        var_vb2e1_dn8: f64,
        var_vb2e1_dn9: f64,
        var_vbc3: f64,
        var_vbc3_db0: f64,
        var_vbc3_db1: f64,
        var_vbc3_dn0: f64,
        var_vbc3_dn1: f64,
        var_vbc3_dn10: f64,
        var_vbc3_dn11: f64,
        var_vbc3_dn2: f64,
        var_vbc3_dn3: f64,
        var_vbc3_dn4: f64,
        var_vbc3_dn5: f64,
        var_vbc3_dn6: f64,
        var_vbc3_dn7: f64,
        var_vbc3_dn8: f64,
        var_vbc3_dn9: f64,
        var_vdc_ctc_t: f64,
        var_vdc_ctc_t_db0: f64,
        var_vdc_ctc_t_db1: f64,
        var_vdc_ctc_t_dn0: f64,
        var_vdc_ctc_t_dn1: f64,
        var_vdc_ctc_t_dn10: f64,
        var_vdc_ctc_t_dn11: f64,
        var_vdc_ctc_t_dn2: f64,
        var_vdc_ctc_t_dn3: f64,
        var_vdc_ctc_t_dn4: f64,
        var_vdc_ctc_t_dn5: f64,
        var_vdc_ctc_t_dn6: f64,
        var_vdc_ctc_t_dn7: f64,
        var_vdc_ctc_t_dn8: f64,
        var_vdc_ctc_t_dn9: f64,
        var_vdcex_t: f64,
        var_vdcex_t_db0: f64,
        var_vdcex_t_db1: f64,
        var_vdcex_t_dn0: f64,
        var_vdcex_t_dn1: f64,
        var_vdcex_t_dn10: f64,
        var_vdcex_t_dn11: f64,
        var_vdcex_t_dn2: f64,
        var_vdcex_t_dn3: f64,
        var_vdcex_t_dn4: f64,
        var_vdcex_t_dn5: f64,
        var_vdcex_t_dn6: f64,
        var_vdcex_t_dn7: f64,
        var_vdcex_t_dn8: f64,
        var_vdcex_t_dn9: f64,
        var_vds_t: f64,
        var_vds_t_db0: f64,
        var_vds_t_db1: f64,
        var_vds_t_dn0: f64,
        var_vds_t_dn1: f64,
        var_vds_t_dn10: f64,
        var_vds_t_dn11: f64,
        var_vds_t_dn2: f64,
        var_vds_t_dn3: f64,
        var_vds_t_dn4: f64,
        var_vds_t_dn5: f64,
        var_vds_t_dn6: f64,
        var_vds_t_dn7: f64,
        var_vds_t_dn8: f64,
        var_vds_t_dn9: f64,
        var_vfc: f64,
        var_vfc_db0: f64,
        var_vfc_db1: f64,
        var_vfc_dn0: f64,
        var_vfc_dn1: f64,
        var_vfc_dn10: f64,
        var_vfc_dn11: f64,
        var_vfc_dn2: f64,
        var_vfc_dn3: f64,
        var_vfc_dn4: f64,
        var_vfc_dn5: f64,
        var_vfc_dn6: f64,
        var_vfc_dn7: f64,
        var_vfc_dn8: f64,
        var_vfc_dn9: f64,
        var_vsc1: f64,
        var_vsc1_db0: f64,
        var_vsc1_db1: f64,
        var_vsc1_dn0: f64,
        var_vsc1_dn1: f64,
        var_vsc1_dn10: f64,
        var_vsc1_dn11: f64,
        var_vsc1_dn2: f64,
        var_vsc1_dn3: f64,
        var_vsc1_dn4: f64,
        var_vsc1_dn5: f64,
        var_vsc1_dn6: f64,
        var_vsc1_dn7: f64,
        var_vsc1_dn8: f64,
        var_vsc1_dn9: f64,
        var_vt: f64,
        var_vtexv: f64,
        var_vtexv_db0: f64,
        var_vtexv_db1: f64,
        var_vtexv_dn0: f64,
        var_vtexv_dn1: f64,
        var_vtexv_dn10: f64,
        var_vtexv_dn11: f64,
        var_vtexv_dn2: f64,
        var_vtexv_dn3: f64,
        var_vtexv_dn4: f64,
        var_vtexv_dn5: f64,
        var_vtexv_dn6: f64,
        var_vtexv_dn7: f64,
        var_vtexv_dn8: f64,
        var_vtexv_dn9: f64,
        var_vtinv: f64,
        var_xi_w: f64,
        var_xi_w_db0: f64,
        var_xi_w_db1: f64,
        var_xi_w_dn0: f64,
        var_xi_w_dn1: f64,
        var_xi_w_dn10: f64,
        var_xi_w_dn11: f64,
        var_xi_w_dn2: f64,
        var_xi_w_dn3: f64,
        var_xi_w_dn4: f64,
        var_xi_w_dn5: f64,
        var_xi_w_dn6: f64,
        var_xi_w_dn7: f64,
        var_xi_w_dn8: f64,
        var_xi_w_dn9: f64,
        var_xp_t: f64,
        var_xp_t_db0: f64,
        var_xp_t_db1: f64,
        var_xp_t_dn0: f64,
        var_xp_t_dn1: f64,
        var_xp_t_dn10: f64,
        var_xp_t_dn11: f64,
        var_xp_t_dn2: f64,
        var_xp_t_dn3: f64,
        var_xp_t_dn4: f64,
        var_xp_t_dn5: f64,
        var_xp_t_dn6: f64,
        var_xp_t_dn7: f64,
        var_xp_t_dn8: f64,
        var_xp_t_dn9: f64,
        var_a_vds_slot: &mut f64,
        var_a_vds_db0_slot: &mut f64,
        var_a_vds_db1_slot: &mut f64,
        var_a_vds_dn0_slot: &mut f64,
        var_a_vds_dn1_slot: &mut f64,
        var_a_vds_dn10_slot: &mut f64,
        var_a_vds_dn11_slot: &mut f64,
        var_a_vds_dn2_slot: &mut f64,
        var_a_vds_dn3_slot: &mut f64,
        var_a_vds_dn4_slot: &mut f64,
        var_a_vds_dn5_slot: &mut f64,
        var_a_vds_dn6_slot: &mut f64,
        var_a_vds_dn7_slot: &mut f64,
        var_a_vds_dn8_slot: &mut f64,
        var_a_vds_dn9_slot: &mut f64,
        var_dxa_slot: &mut f64,
        var_dxa_db0_slot: &mut f64,
        var_dxa_db1_slot: &mut f64,
        var_dxa_dn0_slot: &mut f64,
        var_dxa_dn1_slot: &mut f64,
        var_dxa_dn10_slot: &mut f64,
        var_dxa_dn11_slot: &mut f64,
        var_dxa_dn2_slot: &mut f64,
        var_dxa_dn3_slot: &mut f64,
        var_dxa_dn4_slot: &mut f64,
        var_dxa_dn5_slot: &mut f64,
        var_dxa_dn6_slot: &mut f64,
        var_dxa_dn7_slot: &mut f64,
        var_dxa_dn8_slot: &mut f64,
        var_dxa_dn9_slot: &mut f64,
        var_evb1c4vdcex_slot: &mut f64,
        var_evb1c4vdcex_db0_slot: &mut f64,
        var_evb1c4vdcex_db1_slot: &mut f64,
        var_evb1c4vdcex_dn0_slot: &mut f64,
        var_evb1c4vdcex_dn1_slot: &mut f64,
        var_evb1c4vdcex_dn10_slot: &mut f64,
        var_evb1c4vdcex_dn11_slot: &mut f64,
        var_evb1c4vdcex_dn2_slot: &mut f64,
        var_evb1c4vdcex_dn3_slot: &mut f64,
        var_evb1c4vdcex_dn4_slot: &mut f64,
        var_evb1c4vdcex_dn5_slot: &mut f64,
        var_evb1c4vdcex_dn6_slot: &mut f64,
        var_evb1c4vdcex_dn7_slot: &mut f64,
        var_evb1c4vdcex_dn8_slot: &mut f64,
        var_evb1c4vdcex_dn9_slot: &mut f64,
        var_expl_slot: &mut f64,
        var_guard112_slot: &mut f64,
        var_guard113_slot: &mut f64,
        var_guard114_slot: &mut f64,
        var_guard115_slot: &mut f64,
        var_guard116_slot: &mut f64,
        var_qe0_slot: &mut f64,
        var_qe0_db0_slot: &mut f64,
        var_qe0_db1_slot: &mut f64,
        var_qe0_dn0_slot: &mut f64,
        var_qe0_dn1_slot: &mut f64,
        var_qe0_dn10_slot: &mut f64,
        var_qe0_dn11_slot: &mut f64,
        var_qe0_dn2_slot: &mut f64,
        var_qe0_dn3_slot: &mut f64,
        var_qe0_dn4_slot: &mut f64,
        var_qe0_dn5_slot: &mut f64,
        var_qe0_dn6_slot: &mut f64,
        var_qe0_dn7_slot: &mut f64,
        var_qe0_dn8_slot: &mut f64,
        var_qe0_dn9_slot: &mut f64,
        var_qe_qs_slot: &mut f64,
        var_qe_qs_db0_slot: &mut f64,
        var_qe_qs_db1_slot: &mut f64,
        var_qe_qs_dn0_slot: &mut f64,
        var_qe_qs_dn1_slot: &mut f64,
        var_qe_qs_dn10_slot: &mut f64,
        var_qe_qs_dn11_slot: &mut f64,
        var_qe_qs_dn2_slot: &mut f64,
        var_qe_qs_dn3_slot: &mut f64,
        var_qe_qs_dn4_slot: &mut f64,
        var_qe_qs_dn5_slot: &mut f64,
        var_qe_qs_dn6_slot: &mut f64,
        var_qe_qs_dn7_slot: &mut f64,
        var_qe_qs_dn8_slot: &mut f64,
        var_qe_qs_dn9_slot: &mut f64,
        var_qepi_slot: &mut f64,
        var_qepi0_slot: &mut f64,
        var_qepi_db0_slot: &mut f64,
        var_qepi_db1_slot: &mut f64,
        var_qepi_dn0_slot: &mut f64,
        var_qepi_dn1_slot: &mut f64,
        var_qepi_dn10_slot: &mut f64,
        var_qepi_dn11_slot: &mut f64,
        var_qepi_dn2_slot: &mut f64,
        var_qepi_dn3_slot: &mut f64,
        var_qepi_dn4_slot: &mut f64,
        var_qepi_dn5_slot: &mut f64,
        var_qepi_dn6_slot: &mut f64,
        var_qepi_dn7_slot: &mut f64,
        var_qepi_dn8_slot: &mut f64,
        var_qepi_dn9_slot: &mut f64,
        var_qex_slot: &mut f64,
        var_qex_db0_slot: &mut f64,
        var_qex_db1_slot: &mut f64,
        var_qex_dn0_slot: &mut f64,
        var_qex_dn1_slot: &mut f64,
        var_qex_dn10_slot: &mut f64,
        var_qex_dn11_slot: &mut f64,
        var_qex_dn2_slot: &mut f64,
        var_qex_dn3_slot: &mut f64,
        var_qex_dn4_slot: &mut f64,
        var_qex_dn5_slot: &mut f64,
        var_qex_dn6_slot: &mut f64,
        var_qex_dn7_slot: &mut f64,
        var_qex_dn8_slot: &mut f64,
        var_qex_dn9_slot: &mut f64,
        var_qtex_slot: &mut f64,
        var_qtex_db0_slot: &mut f64,
        var_qtex_db1_slot: &mut f64,
        var_qtex_dn0_slot: &mut f64,
        var_qtex_dn1_slot: &mut f64,
        var_qtex_dn10_slot: &mut f64,
        var_qtex_dn11_slot: &mut f64,
        var_qtex_dn2_slot: &mut f64,
        var_qtex_dn3_slot: &mut f64,
        var_qtex_dn4_slot: &mut f64,
        var_qtex_dn5_slot: &mut f64,
        var_qtex_dn6_slot: &mut f64,
        var_qtex_dn7_slot: &mut f64,
        var_qtex_dn8_slot: &mut f64,
        var_qtex_dn9_slot: &mut f64,
        var_qts_slot: &mut f64,
        var_qts_db0_slot: &mut f64,
        var_qts_db1_slot: &mut f64,
        var_qts_dn0_slot: &mut f64,
        var_qts_dn1_slot: &mut f64,
        var_qts_dn10_slot: &mut f64,
        var_qts_dn11_slot: &mut f64,
        var_qts_dn2_slot: &mut f64,
        var_qts_dn3_slot: &mut f64,
        var_qts_dn4_slot: &mut f64,
        var_qts_dn5_slot: &mut f64,
        var_qts_dn6_slot: &mut f64,
        var_qts_dn7_slot: &mut f64,
        var_qts_dn8_slot: &mut f64,
        var_qts_dn9_slot: &mut f64,
        var_tmpexp_slot: &mut f64,
        var_tmpexp_db0_slot: &mut f64,
        var_tmpexp_db1_slot: &mut f64,
        var_tmpexp_dn0_slot: &mut f64,
        var_tmpexp_dn1_slot: &mut f64,
        var_tmpexp_dn10_slot: &mut f64,
        var_tmpexp_dn11_slot: &mut f64,
        var_tmpexp_dn2_slot: &mut f64,
        var_tmpexp_dn3_slot: &mut f64,
        var_tmpexp_dn4_slot: &mut f64,
        var_tmpexp_dn5_slot: &mut f64,
        var_tmpexp_dn6_slot: &mut f64,
        var_tmpexp_dn7_slot: &mut f64,
        var_tmpexp_dn8_slot: &mut f64,
        var_tmpexp_dn9_slot: &mut f64,
        var_vfs_slot: &mut f64,
        var_vfs_db0_slot: &mut f64,
        var_vfs_db1_slot: &mut f64,
        var_vfs_dn0_slot: &mut f64,
        var_vfs_dn1_slot: &mut f64,
        var_vfs_dn10_slot: &mut f64,
        var_vfs_dn11_slot: &mut f64,
        var_vfs_dn2_slot: &mut f64,
        var_vfs_dn3_slot: &mut f64,
        var_vfs_dn4_slot: &mut f64,
        var_vfs_dn5_slot: &mut f64,
        var_vfs_dn6_slot: &mut f64,
        var_vfs_dn7_slot: &mut f64,
        var_vfs_dn8_slot: &mut f64,
        var_vfs_dn9_slot: &mut f64,
        var_vjs_slot: &mut f64,
        var_vjs_db0_slot: &mut f64,
        var_vjs_db1_slot: &mut f64,
        var_vjs_dn0_slot: &mut f64,
        var_vjs_dn1_slot: &mut f64,
        var_vjs_dn10_slot: &mut f64,
        var_vjs_dn11_slot: &mut f64,
        var_vjs_dn2_slot: &mut f64,
        var_vjs_dn3_slot: &mut f64,
        var_vjs_dn4_slot: &mut f64,
        var_vjs_dn5_slot: &mut f64,
        var_vjs_dn6_slot: &mut f64,
        var_vjs_dn7_slot: &mut f64,
        var_vjs_dn8_slot: &mut f64,
        var_vjs_dn9_slot: &mut f64,
        var_xqtex_slot: &mut f64,
        var_xqtex_db0_slot: &mut f64,
        var_xqtex_db1_slot: &mut f64,
        var_xqtex_dn0_slot: &mut f64,
        var_xqtex_dn1_slot: &mut f64,
        var_xqtex_dn10_slot: &mut f64,
        var_xqtex_dn11_slot: &mut f64,
        var_xqtex_dn2_slot: &mut f64,
        var_xqtex_dn3_slot: &mut f64,
        var_xqtex_dn4_slot: &mut f64,
        var_xqtex_dn5_slot: &mut f64,
        var_xqtex_dn6_slot: &mut f64,
        var_xqtex_dn7_slot: &mut f64,
        var_xqtex_dn8_slot: &mut f64,
        var_xqtex_dn9_slot: &mut f64,
        var_xvjcex_slot: &mut f64,
        var_xvjcex_db0_slot: &mut f64,
        var_xvjcex_db1_slot: &mut f64,
        var_xvjcex_dn0_slot: &mut f64,
        var_xvjcex_dn1_slot: &mut f64,
        var_xvjcex_dn10_slot: &mut f64,
        var_xvjcex_dn11_slot: &mut f64,
        var_xvjcex_dn2_slot: &mut f64,
        var_xvjcex_dn3_slot: &mut f64,
        var_xvjcex_dn4_slot: &mut f64,
        var_xvjcex_dn5_slot: &mut f64,
        var_xvjcex_dn6_slot: &mut f64,
        var_xvjcex_dn7_slot: &mut f64,
        var_xvjcex_dn8_slot: &mut f64,
        var_xvjcex_dn9_slot: &mut f64,
        var_xvtexv_slot: &mut f64,
        var_xvtexv_db0_slot: &mut f64,
        var_xvtexv_db1_slot: &mut f64,
        var_xvtexv_dn0_slot: &mut f64,
        var_xvtexv_dn1_slot: &mut f64,
        var_xvtexv_dn10_slot: &mut f64,
        var_xvtexv_dn11_slot: &mut f64,
        var_xvtexv_dn2_slot: &mut f64,
        var_xvtexv_dn3_slot: &mut f64,
        var_xvtexv_dn4_slot: &mut f64,
        var_xvtexv_dn5_slot: &mut f64,
        var_xvtexv_dn6_slot: &mut f64,
        var_xvtexv_dn7_slot: &mut f64,
        var_xvtexv_dn8_slot: &mut f64,
        var_xvtexv_dn9_slot: &mut f64,
    ) {
        let mut var_a_vds: f64 = *var_a_vds_slot;
        let mut var_a_vds_db0: f64 = *var_a_vds_db0_slot;
        let mut var_a_vds_db1: f64 = *var_a_vds_db1_slot;
        let mut var_a_vds_dn0: f64 = *var_a_vds_dn0_slot;
        let mut var_a_vds_dn1: f64 = *var_a_vds_dn1_slot;
        let mut var_a_vds_dn10: f64 = *var_a_vds_dn10_slot;
        let mut var_a_vds_dn11: f64 = *var_a_vds_dn11_slot;
        let mut var_a_vds_dn2: f64 = *var_a_vds_dn2_slot;
        let mut var_a_vds_dn3: f64 = *var_a_vds_dn3_slot;
        let mut var_a_vds_dn4: f64 = *var_a_vds_dn4_slot;
        let mut var_a_vds_dn5: f64 = *var_a_vds_dn5_slot;
        let mut var_a_vds_dn6: f64 = *var_a_vds_dn6_slot;
        let mut var_a_vds_dn7: f64 = *var_a_vds_dn7_slot;
        let mut var_a_vds_dn8: f64 = *var_a_vds_dn8_slot;
        let mut var_a_vds_dn9: f64 = *var_a_vds_dn9_slot;
        let mut var_dxa: f64 = *var_dxa_slot;
        let mut var_dxa_db0: f64 = *var_dxa_db0_slot;
        let mut var_dxa_db1: f64 = *var_dxa_db1_slot;
        let mut var_dxa_dn0: f64 = *var_dxa_dn0_slot;
        let mut var_dxa_dn1: f64 = *var_dxa_dn1_slot;
        let mut var_dxa_dn10: f64 = *var_dxa_dn10_slot;
        let mut var_dxa_dn11: f64 = *var_dxa_dn11_slot;
        let mut var_dxa_dn2: f64 = *var_dxa_dn2_slot;
        let mut var_dxa_dn3: f64 = *var_dxa_dn3_slot;
        let mut var_dxa_dn4: f64 = *var_dxa_dn4_slot;
        let mut var_dxa_dn5: f64 = *var_dxa_dn5_slot;
        let mut var_dxa_dn6: f64 = *var_dxa_dn6_slot;
        let mut var_dxa_dn7: f64 = *var_dxa_dn7_slot;
        let mut var_dxa_dn8: f64 = *var_dxa_dn8_slot;
        let mut var_dxa_dn9: f64 = *var_dxa_dn9_slot;
        let mut var_evb1c4vdcex: f64 = *var_evb1c4vdcex_slot;
        let mut var_evb1c4vdcex_db0: f64 = *var_evb1c4vdcex_db0_slot;
        let mut var_evb1c4vdcex_db1: f64 = *var_evb1c4vdcex_db1_slot;
        let mut var_evb1c4vdcex_dn0: f64 = *var_evb1c4vdcex_dn0_slot;
        let mut var_evb1c4vdcex_dn1: f64 = *var_evb1c4vdcex_dn1_slot;
        let mut var_evb1c4vdcex_dn10: f64 = *var_evb1c4vdcex_dn10_slot;
        let mut var_evb1c4vdcex_dn11: f64 = *var_evb1c4vdcex_dn11_slot;
        let mut var_evb1c4vdcex_dn2: f64 = *var_evb1c4vdcex_dn2_slot;
        let mut var_evb1c4vdcex_dn3: f64 = *var_evb1c4vdcex_dn3_slot;
        let mut var_evb1c4vdcex_dn4: f64 = *var_evb1c4vdcex_dn4_slot;
        let mut var_evb1c4vdcex_dn5: f64 = *var_evb1c4vdcex_dn5_slot;
        let mut var_evb1c4vdcex_dn6: f64 = *var_evb1c4vdcex_dn6_slot;
        let mut var_evb1c4vdcex_dn7: f64 = *var_evb1c4vdcex_dn7_slot;
        let mut var_evb1c4vdcex_dn8: f64 = *var_evb1c4vdcex_dn8_slot;
        let mut var_evb1c4vdcex_dn9: f64 = *var_evb1c4vdcex_dn9_slot;
        let mut var_expl: f64 = *var_expl_slot;
        let mut var_guard112: f64 = *var_guard112_slot;
        let mut var_guard113: f64 = *var_guard113_slot;
        let mut var_guard114: f64 = *var_guard114_slot;
        let mut var_guard115: f64 = *var_guard115_slot;
        let mut var_guard116: f64 = *var_guard116_slot;
        let mut var_qe0: f64 = *var_qe0_slot;
        let mut var_qe0_db0: f64 = *var_qe0_db0_slot;
        let mut var_qe0_db1: f64 = *var_qe0_db1_slot;
        let mut var_qe0_dn0: f64 = *var_qe0_dn0_slot;
        let mut var_qe0_dn1: f64 = *var_qe0_dn1_slot;
        let mut var_qe0_dn10: f64 = *var_qe0_dn10_slot;
        let mut var_qe0_dn11: f64 = *var_qe0_dn11_slot;
        let mut var_qe0_dn2: f64 = *var_qe0_dn2_slot;
        let mut var_qe0_dn3: f64 = *var_qe0_dn3_slot;
        let mut var_qe0_dn4: f64 = *var_qe0_dn4_slot;
        let mut var_qe0_dn5: f64 = *var_qe0_dn5_slot;
        let mut var_qe0_dn6: f64 = *var_qe0_dn6_slot;
        let mut var_qe0_dn7: f64 = *var_qe0_dn7_slot;
        let mut var_qe0_dn8: f64 = *var_qe0_dn8_slot;
        let mut var_qe0_dn9: f64 = *var_qe0_dn9_slot;
        let mut var_qe_qs: f64 = *var_qe_qs_slot;
        let mut var_qe_qs_db0: f64 = *var_qe_qs_db0_slot;
        let mut var_qe_qs_db1: f64 = *var_qe_qs_db1_slot;
        let mut var_qe_qs_dn0: f64 = *var_qe_qs_dn0_slot;
        let mut var_qe_qs_dn1: f64 = *var_qe_qs_dn1_slot;
        let mut var_qe_qs_dn10: f64 = *var_qe_qs_dn10_slot;
        let mut var_qe_qs_dn11: f64 = *var_qe_qs_dn11_slot;
        let mut var_qe_qs_dn2: f64 = *var_qe_qs_dn2_slot;
        let mut var_qe_qs_dn3: f64 = *var_qe_qs_dn3_slot;
        let mut var_qe_qs_dn4: f64 = *var_qe_qs_dn4_slot;
        let mut var_qe_qs_dn5: f64 = *var_qe_qs_dn5_slot;
        let mut var_qe_qs_dn6: f64 = *var_qe_qs_dn6_slot;
        let mut var_qe_qs_dn7: f64 = *var_qe_qs_dn7_slot;
        let mut var_qe_qs_dn8: f64 = *var_qe_qs_dn8_slot;
        let mut var_qe_qs_dn9: f64 = *var_qe_qs_dn9_slot;
        let mut var_qepi: f64 = *var_qepi_slot;
        let mut var_qepi0: f64 = *var_qepi0_slot;
        let mut var_qepi_db0: f64 = *var_qepi_db0_slot;
        let mut var_qepi_db1: f64 = *var_qepi_db1_slot;
        let mut var_qepi_dn0: f64 = *var_qepi_dn0_slot;
        let mut var_qepi_dn1: f64 = *var_qepi_dn1_slot;
        let mut var_qepi_dn10: f64 = *var_qepi_dn10_slot;
        let mut var_qepi_dn11: f64 = *var_qepi_dn11_slot;
        let mut var_qepi_dn2: f64 = *var_qepi_dn2_slot;
        let mut var_qepi_dn3: f64 = *var_qepi_dn3_slot;
        let mut var_qepi_dn4: f64 = *var_qepi_dn4_slot;
        let mut var_qepi_dn5: f64 = *var_qepi_dn5_slot;
        let mut var_qepi_dn6: f64 = *var_qepi_dn6_slot;
        let mut var_qepi_dn7: f64 = *var_qepi_dn7_slot;
        let mut var_qepi_dn8: f64 = *var_qepi_dn8_slot;
        let mut var_qepi_dn9: f64 = *var_qepi_dn9_slot;
        let mut var_qex: f64 = *var_qex_slot;
        let mut var_qex_db0: f64 = *var_qex_db0_slot;
        let mut var_qex_db1: f64 = *var_qex_db1_slot;
        let mut var_qex_dn0: f64 = *var_qex_dn0_slot;
        let mut var_qex_dn1: f64 = *var_qex_dn1_slot;
        let mut var_qex_dn10: f64 = *var_qex_dn10_slot;
        let mut var_qex_dn11: f64 = *var_qex_dn11_slot;
        let mut var_qex_dn2: f64 = *var_qex_dn2_slot;
        let mut var_qex_dn3: f64 = *var_qex_dn3_slot;
        let mut var_qex_dn4: f64 = *var_qex_dn4_slot;
        let mut var_qex_dn5: f64 = *var_qex_dn5_slot;
        let mut var_qex_dn6: f64 = *var_qex_dn6_slot;
        let mut var_qex_dn7: f64 = *var_qex_dn7_slot;
        let mut var_qex_dn8: f64 = *var_qex_dn8_slot;
        let mut var_qex_dn9: f64 = *var_qex_dn9_slot;
        let mut var_qtex: f64 = *var_qtex_slot;
        let mut var_qtex_db0: f64 = *var_qtex_db0_slot;
        let mut var_qtex_db1: f64 = *var_qtex_db1_slot;
        let mut var_qtex_dn0: f64 = *var_qtex_dn0_slot;
        let mut var_qtex_dn1: f64 = *var_qtex_dn1_slot;
        let mut var_qtex_dn10: f64 = *var_qtex_dn10_slot;
        let mut var_qtex_dn11: f64 = *var_qtex_dn11_slot;
        let mut var_qtex_dn2: f64 = *var_qtex_dn2_slot;
        let mut var_qtex_dn3: f64 = *var_qtex_dn3_slot;
        let mut var_qtex_dn4: f64 = *var_qtex_dn4_slot;
        let mut var_qtex_dn5: f64 = *var_qtex_dn5_slot;
        let mut var_qtex_dn6: f64 = *var_qtex_dn6_slot;
        let mut var_qtex_dn7: f64 = *var_qtex_dn7_slot;
        let mut var_qtex_dn8: f64 = *var_qtex_dn8_slot;
        let mut var_qtex_dn9: f64 = *var_qtex_dn9_slot;
        let mut var_qts: f64 = *var_qts_slot;
        let mut var_qts_db0: f64 = *var_qts_db0_slot;
        let mut var_qts_db1: f64 = *var_qts_db1_slot;
        let mut var_qts_dn0: f64 = *var_qts_dn0_slot;
        let mut var_qts_dn1: f64 = *var_qts_dn1_slot;
        let mut var_qts_dn10: f64 = *var_qts_dn10_slot;
        let mut var_qts_dn11: f64 = *var_qts_dn11_slot;
        let mut var_qts_dn2: f64 = *var_qts_dn2_slot;
        let mut var_qts_dn3: f64 = *var_qts_dn3_slot;
        let mut var_qts_dn4: f64 = *var_qts_dn4_slot;
        let mut var_qts_dn5: f64 = *var_qts_dn5_slot;
        let mut var_qts_dn6: f64 = *var_qts_dn6_slot;
        let mut var_qts_dn7: f64 = *var_qts_dn7_slot;
        let mut var_qts_dn8: f64 = *var_qts_dn8_slot;
        let mut var_qts_dn9: f64 = *var_qts_dn9_slot;
        let mut var_tmpexp: f64 = *var_tmpexp_slot;
        let mut var_tmpexp_db0: f64 = *var_tmpexp_db0_slot;
        let mut var_tmpexp_db1: f64 = *var_tmpexp_db1_slot;
        let mut var_tmpexp_dn0: f64 = *var_tmpexp_dn0_slot;
        let mut var_tmpexp_dn1: f64 = *var_tmpexp_dn1_slot;
        let mut var_tmpexp_dn10: f64 = *var_tmpexp_dn10_slot;
        let mut var_tmpexp_dn11: f64 = *var_tmpexp_dn11_slot;
        let mut var_tmpexp_dn2: f64 = *var_tmpexp_dn2_slot;
        let mut var_tmpexp_dn3: f64 = *var_tmpexp_dn3_slot;
        let mut var_tmpexp_dn4: f64 = *var_tmpexp_dn4_slot;
        let mut var_tmpexp_dn5: f64 = *var_tmpexp_dn5_slot;
        let mut var_tmpexp_dn6: f64 = *var_tmpexp_dn6_slot;
        let mut var_tmpexp_dn7: f64 = *var_tmpexp_dn7_slot;
        let mut var_tmpexp_dn8: f64 = *var_tmpexp_dn8_slot;
        let mut var_tmpexp_dn9: f64 = *var_tmpexp_dn9_slot;
        let mut var_vfs: f64 = *var_vfs_slot;
        let mut var_vfs_db0: f64 = *var_vfs_db0_slot;
        let mut var_vfs_db1: f64 = *var_vfs_db1_slot;
        let mut var_vfs_dn0: f64 = *var_vfs_dn0_slot;
        let mut var_vfs_dn1: f64 = *var_vfs_dn1_slot;
        let mut var_vfs_dn10: f64 = *var_vfs_dn10_slot;
        let mut var_vfs_dn11: f64 = *var_vfs_dn11_slot;
        let mut var_vfs_dn2: f64 = *var_vfs_dn2_slot;
        let mut var_vfs_dn3: f64 = *var_vfs_dn3_slot;
        let mut var_vfs_dn4: f64 = *var_vfs_dn4_slot;
        let mut var_vfs_dn5: f64 = *var_vfs_dn5_slot;
        let mut var_vfs_dn6: f64 = *var_vfs_dn6_slot;
        let mut var_vfs_dn7: f64 = *var_vfs_dn7_slot;
        let mut var_vfs_dn8: f64 = *var_vfs_dn8_slot;
        let mut var_vfs_dn9: f64 = *var_vfs_dn9_slot;
        let mut var_vjs: f64 = *var_vjs_slot;
        let mut var_vjs_db0: f64 = *var_vjs_db0_slot;
        let mut var_vjs_db1: f64 = *var_vjs_db1_slot;
        let mut var_vjs_dn0: f64 = *var_vjs_dn0_slot;
        let mut var_vjs_dn1: f64 = *var_vjs_dn1_slot;
        let mut var_vjs_dn10: f64 = *var_vjs_dn10_slot;
        let mut var_vjs_dn11: f64 = *var_vjs_dn11_slot;
        let mut var_vjs_dn2: f64 = *var_vjs_dn2_slot;
        let mut var_vjs_dn3: f64 = *var_vjs_dn3_slot;
        let mut var_vjs_dn4: f64 = *var_vjs_dn4_slot;
        let mut var_vjs_dn5: f64 = *var_vjs_dn5_slot;
        let mut var_vjs_dn6: f64 = *var_vjs_dn6_slot;
        let mut var_vjs_dn7: f64 = *var_vjs_dn7_slot;
        let mut var_vjs_dn8: f64 = *var_vjs_dn8_slot;
        let mut var_vjs_dn9: f64 = *var_vjs_dn9_slot;
        let mut var_xqtex: f64 = *var_xqtex_slot;
        let mut var_xqtex_db0: f64 = *var_xqtex_db0_slot;
        let mut var_xqtex_db1: f64 = *var_xqtex_db1_slot;
        let mut var_xqtex_dn0: f64 = *var_xqtex_dn0_slot;
        let mut var_xqtex_dn1: f64 = *var_xqtex_dn1_slot;
        let mut var_xqtex_dn10: f64 = *var_xqtex_dn10_slot;
        let mut var_xqtex_dn11: f64 = *var_xqtex_dn11_slot;
        let mut var_xqtex_dn2: f64 = *var_xqtex_dn2_slot;
        let mut var_xqtex_dn3: f64 = *var_xqtex_dn3_slot;
        let mut var_xqtex_dn4: f64 = *var_xqtex_dn4_slot;
        let mut var_xqtex_dn5: f64 = *var_xqtex_dn5_slot;
        let mut var_xqtex_dn6: f64 = *var_xqtex_dn6_slot;
        let mut var_xqtex_dn7: f64 = *var_xqtex_dn7_slot;
        let mut var_xqtex_dn8: f64 = *var_xqtex_dn8_slot;
        let mut var_xqtex_dn9: f64 = *var_xqtex_dn9_slot;
        let mut var_xvjcex: f64 = *var_xvjcex_slot;
        let mut var_xvjcex_db0: f64 = *var_xvjcex_db0_slot;
        let mut var_xvjcex_db1: f64 = *var_xvjcex_db1_slot;
        let mut var_xvjcex_dn0: f64 = *var_xvjcex_dn0_slot;
        let mut var_xvjcex_dn1: f64 = *var_xvjcex_dn1_slot;
        let mut var_xvjcex_dn10: f64 = *var_xvjcex_dn10_slot;
        let mut var_xvjcex_dn11: f64 = *var_xvjcex_dn11_slot;
        let mut var_xvjcex_dn2: f64 = *var_xvjcex_dn2_slot;
        let mut var_xvjcex_dn3: f64 = *var_xvjcex_dn3_slot;
        let mut var_xvjcex_dn4: f64 = *var_xvjcex_dn4_slot;
        let mut var_xvjcex_dn5: f64 = *var_xvjcex_dn5_slot;
        let mut var_xvjcex_dn6: f64 = *var_xvjcex_dn6_slot;
        let mut var_xvjcex_dn7: f64 = *var_xvjcex_dn7_slot;
        let mut var_xvjcex_dn8: f64 = *var_xvjcex_dn8_slot;
        let mut var_xvjcex_dn9: f64 = *var_xvjcex_dn9_slot;
        let mut var_xvtexv: f64 = *var_xvtexv_slot;
        let mut var_xvtexv_db0: f64 = *var_xvtexv_db0_slot;
        let mut var_xvtexv_db1: f64 = *var_xvtexv_db1_slot;
        let mut var_xvtexv_dn0: f64 = *var_xvtexv_dn0_slot;
        let mut var_xvtexv_dn1: f64 = *var_xvtexv_dn1_slot;
        let mut var_xvtexv_dn10: f64 = *var_xvtexv_dn10_slot;
        let mut var_xvtexv_dn11: f64 = *var_xvtexv_dn11_slot;
        let mut var_xvtexv_dn2: f64 = *var_xvtexv_dn2_slot;
        let mut var_xvtexv_dn3: f64 = *var_xvtexv_dn3_slot;
        let mut var_xvtexv_dn4: f64 = *var_xvtexv_dn4_slot;
        let mut var_xvtexv_dn5: f64 = *var_xvtexv_dn5_slot;
        let mut var_xvtexv_dn6: f64 = *var_xvtexv_dn6_slot;
        let mut var_xvtexv_dn7: f64 = *var_xvtexv_dn7_slot;
        let mut var_xvtexv_dn8: f64 = *var_xvtexv_dn8_slot;
        let mut var_xvtexv_dn9: f64 = *var_xvtexv_dn9_slot;

        let assign6180_e6338: f64 = (1.0 - var_xp_t);
        let assign6180_e6340: f64 = (assign6180_e6338 * var_vtexv);
        let assign6180_e6343: f64 = (var_xp_t * var_vb1c4);
        let assign6180_e6344: f64 = (assign6180_e6340 + assign6180_e6343);
        let assign6180_e6345: f64 = (var_cjc_t * assign6180_e6344);
        let assign6180_e6348: f64 = (1.0 - p.p77);
        let assign6180_e6349: f64 = (assign6180_e6345 * assign6180_e6348);
        let assign6180_e6352: f64 = (1.0 - p.p33);
        let assign6180_e6353: f64 = (assign6180_e6349 * assign6180_e6352);
        var_qtex = assign6180_e6353;
        var_qtex_dn0 = ((((var_cjc_t_dn0 * assign6180_e6344) + (var_cjc_t * ((((-var_xp_t_dn0) * var_vtexv) + (assign6180_e6338 * var_vtexv_dn0)) + ((var_xp_t_dn0 * var_vb1c4) + (var_xp_t * var_vb1c4_dn0))))) * assign6180_e6348) * assign6180_e6352);
        var_qtex_dn1 = ((((var_cjc_t_dn1 * assign6180_e6344) + (var_cjc_t * ((((-var_xp_t_dn1) * var_vtexv) + (assign6180_e6338 * var_vtexv_dn1)) + ((var_xp_t_dn1 * var_vb1c4) + (var_xp_t * var_vb1c4_dn1))))) * assign6180_e6348) * assign6180_e6352);
        var_qtex_dn2 = ((((var_cjc_t_dn2 * assign6180_e6344) + (var_cjc_t * ((((-var_xp_t_dn2) * var_vtexv) + (assign6180_e6338 * var_vtexv_dn2)) + ((var_xp_t_dn2 * var_vb1c4) + (var_xp_t * var_vb1c4_dn2))))) * assign6180_e6348) * assign6180_e6352);
        var_qtex_dn3 = ((((var_cjc_t_dn3 * assign6180_e6344) + (var_cjc_t * ((((-var_xp_t_dn3) * var_vtexv) + (assign6180_e6338 * var_vtexv_dn3)) + ((var_xp_t_dn3 * var_vb1c4) + (var_xp_t * var_vb1c4_dn3))))) * assign6180_e6348) * assign6180_e6352);
        var_qtex_dn4 = ((((var_cjc_t_dn4 * assign6180_e6344) + (var_cjc_t * ((((-var_xp_t_dn4) * var_vtexv) + (assign6180_e6338 * var_vtexv_dn4)) + ((var_xp_t_dn4 * var_vb1c4) + (var_xp_t * var_vb1c4_dn4))))) * assign6180_e6348) * assign6180_e6352);
        var_qtex_dn5 = ((((var_cjc_t_dn5 * assign6180_e6344) + (var_cjc_t * ((((-var_xp_t_dn5) * var_vtexv) + (assign6180_e6338 * var_vtexv_dn5)) + ((var_xp_t_dn5 * var_vb1c4) + (var_xp_t * var_vb1c4_dn5))))) * assign6180_e6348) * assign6180_e6352);
        var_qtex_dn6 = ((((var_cjc_t_dn6 * assign6180_e6344) + (var_cjc_t * ((((-var_xp_t_dn6) * var_vtexv) + (assign6180_e6338 * var_vtexv_dn6)) + ((var_xp_t_dn6 * var_vb1c4) + (var_xp_t * var_vb1c4_dn6))))) * assign6180_e6348) * assign6180_e6352);
        var_qtex_dn7 = ((((var_cjc_t_dn7 * assign6180_e6344) + (var_cjc_t * ((((-var_xp_t_dn7) * var_vtexv) + (assign6180_e6338 * var_vtexv_dn7)) + ((var_xp_t_dn7 * var_vb1c4) + (var_xp_t * var_vb1c4_dn7))))) * assign6180_e6348) * assign6180_e6352);
        var_qtex_dn8 = ((((var_cjc_t_dn8 * assign6180_e6344) + (var_cjc_t * ((((-var_xp_t_dn8) * var_vtexv) + (assign6180_e6338 * var_vtexv_dn8)) + ((var_xp_t_dn8 * var_vb1c4) + (var_xp_t * var_vb1c4_dn8))))) * assign6180_e6348) * assign6180_e6352);
        var_qtex_dn9 = ((((var_cjc_t_dn9 * assign6180_e6344) + (var_cjc_t * ((((-var_xp_t_dn9) * var_vtexv) + (assign6180_e6338 * var_vtexv_dn9)) + ((var_xp_t_dn9 * var_vb1c4) + (var_xp_t * var_vb1c4_dn9))))) * assign6180_e6348) * assign6180_e6352);
        var_qtex_dn10 = ((((var_cjc_t_dn10 * assign6180_e6344) + (var_cjc_t * ((((-var_xp_t_dn10) * var_vtexv) + (assign6180_e6338 * var_vtexv_dn10)) + ((var_xp_t_dn10 * var_vb1c4) + (var_xp_t * var_vb1c4_dn10))))) * assign6180_e6348) * assign6180_e6352);
        var_qtex_dn11 = ((((var_cjc_t_dn11 * assign6180_e6344) + (var_cjc_t * ((((-var_xp_t_dn11) * var_vtexv) + (assign6180_e6338 * var_vtexv_dn11)) + ((var_xp_t_dn11 * var_vb1c4) + (var_xp_t * var_vb1c4_dn11))))) * assign6180_e6348) * assign6180_e6352);
        var_qtex_db0 = ((((var_cjc_t_db0 * assign6180_e6344) + (var_cjc_t * ((((-var_xp_t_db0) * var_vtexv) + (assign6180_e6338 * var_vtexv_db0)) + ((var_xp_t_db0 * var_vb1c4) + (var_xp_t * var_vb1c4_db0))))) * assign6180_e6348) * assign6180_e6352);
        var_qtex_db1 = ((((var_cjc_t_db1 * assign6180_e6344) + (var_cjc_t * ((((-var_xp_t_db1) * var_vtexv) + (assign6180_e6338 * var_vtexv_db1)) + ((var_xp_t_db1 * var_vb1c4) + (var_xp_t * var_vb1c4_db1))))) * assign6180_e6348) * assign6180_e6352);

        let assign6190_e6356: f64 = (var_vbc3 - var_vfc);
        let assign6190_e6358: f64 = (assign6190_e6356 / var_a_vdcctc);
        var_dxa = assign6190_e6358;
        var_dxa_dn0 = ((((var_vbc3_dn0 - var_vfc_dn0) * var_a_vdcctc) - (assign6190_e6356 * var_a_vdcctc_dn0)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn1 = ((((var_vbc3_dn1 - var_vfc_dn1) * var_a_vdcctc) - (assign6190_e6356 * var_a_vdcctc_dn1)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn2 = ((((var_vbc3_dn2 - var_vfc_dn2) * var_a_vdcctc) - (assign6190_e6356 * var_a_vdcctc_dn2)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn3 = ((((var_vbc3_dn3 - var_vfc_dn3) * var_a_vdcctc) - (assign6190_e6356 * var_a_vdcctc_dn3)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn4 = ((((var_vbc3_dn4 - var_vfc_dn4) * var_a_vdcctc) - (assign6190_e6356 * var_a_vdcctc_dn4)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn5 = ((((var_vbc3_dn5 - var_vfc_dn5) * var_a_vdcctc) - (assign6190_e6356 * var_a_vdcctc_dn5)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn6 = ((((var_vbc3_dn6 - var_vfc_dn6) * var_a_vdcctc) - (assign6190_e6356 * var_a_vdcctc_dn6)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn7 = ((((var_vbc3_dn7 - var_vfc_dn7) * var_a_vdcctc) - (assign6190_e6356 * var_a_vdcctc_dn7)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn8 = ((((var_vbc3_dn8 - var_vfc_dn8) * var_a_vdcctc) - (assign6190_e6356 * var_a_vdcctc_dn8)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn9 = ((((var_vbc3_dn9 - var_vfc_dn9) * var_a_vdcctc) - (assign6190_e6356 * var_a_vdcctc_dn9)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn10 = ((((var_vbc3_dn10 - var_vfc_dn10) * var_a_vdcctc) - (assign6190_e6356 * var_a_vdcctc_dn10)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn11 = ((((var_vbc3_dn11 - var_vfc_dn11) * var_a_vdcctc) - (assign6190_e6356 * var_a_vdcctc_dn11)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_db0 = ((((var_vbc3_db0 - var_vfc_db0) * var_a_vdcctc) - (assign6190_e6356 * var_a_vdcctc_db0)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_db1 = ((((var_vbc3_db1 - var_vfc_db1) * var_a_vdcctc) - (assign6190_e6356 * var_a_vdcctc_db1)) / (var_a_vdcctc * var_a_vdcctc));

        let assign6200_e6361: f64 = if var_vbc3 < var_vfc { 1.0 } else { 0.0 };
        var_guard112 = assign6200_e6361;

        let (assign6210_e6373, assign6210_e6373_d_n0, assign6210_e6373_d_n1, assign6210_e6373_d_n2, assign6210_e6373_d_n3, assign6210_e6373_d_n4, assign6210_e6373_d_n5, assign6210_e6373_d_n6, assign6210_e6373_d_n7, assign6210_e6373_d_n8, assign6210_e6373_d_n9, assign6210_e6373_d_n10, assign6210_e6373_d_n11, assign6210_e6373_d_b0, assign6210_e6373_d_b1,) = {
    if (var_guard112 != 0.0) {
        let assign6210_e6367: f64 = (var_dxa).exp();
        let assign6210_e6368: f64 = (1.0 + assign6210_e6367);
        let assign6210_e6369: f64 = (assign6210_e6368).ln();
        let assign6210_e6370: f64 = (var_a_vdcctc * assign6210_e6369);
        let assign6210_e6371: f64 = (var_vbc3 - assign6210_e6370);
        (assign6210_e6371, (var_vbc3_dn0 - ((var_a_vdcctc_dn0 * assign6210_e6369) + (var_a_vdcctc * ((assign6210_e6367 * var_dxa_dn0) / assign6210_e6368)))), (var_vbc3_dn1 - ((var_a_vdcctc_dn1 * assign6210_e6369) + (var_a_vdcctc * ((assign6210_e6367 * var_dxa_dn1) / assign6210_e6368)))), (var_vbc3_dn2 - ((var_a_vdcctc_dn2 * assign6210_e6369) + (var_a_vdcctc * ((assign6210_e6367 * var_dxa_dn2) / assign6210_e6368)))), (var_vbc3_dn3 - ((var_a_vdcctc_dn3 * assign6210_e6369) + (var_a_vdcctc * ((assign6210_e6367 * var_dxa_dn3) / assign6210_e6368)))), (var_vbc3_dn4 - ((var_a_vdcctc_dn4 * assign6210_e6369) + (var_a_vdcctc * ((assign6210_e6367 * var_dxa_dn4) / assign6210_e6368)))), (var_vbc3_dn5 - ((var_a_vdcctc_dn5 * assign6210_e6369) + (var_a_vdcctc * ((assign6210_e6367 * var_dxa_dn5) / assign6210_e6368)))), (var_vbc3_dn6 - ((var_a_vdcctc_dn6 * assign6210_e6369) + (var_a_vdcctc * ((assign6210_e6367 * var_dxa_dn6) / assign6210_e6368)))), (var_vbc3_dn7 - ((var_a_vdcctc_dn7 * assign6210_e6369) + (var_a_vdcctc * ((assign6210_e6367 * var_dxa_dn7) / assign6210_e6368)))), (var_vbc3_dn8 - ((var_a_vdcctc_dn8 * assign6210_e6369) + (var_a_vdcctc * ((assign6210_e6367 * var_dxa_dn8) / assign6210_e6368)))), (var_vbc3_dn9 - ((var_a_vdcctc_dn9 * assign6210_e6369) + (var_a_vdcctc * ((assign6210_e6367 * var_dxa_dn9) / assign6210_e6368)))), (var_vbc3_dn10 - ((var_a_vdcctc_dn10 * assign6210_e6369) + (var_a_vdcctc * ((assign6210_e6367 * var_dxa_dn10) / assign6210_e6368)))), (var_vbc3_dn11 - ((var_a_vdcctc_dn11 * assign6210_e6369) + (var_a_vdcctc * ((assign6210_e6367 * var_dxa_dn11) / assign6210_e6368)))), (var_vbc3_db0 - ((var_a_vdcctc_db0 * assign6210_e6369) + (var_a_vdcctc * ((assign6210_e6367 * var_dxa_db0) / assign6210_e6368)))), (var_vbc3_db1 - ((var_a_vdcctc_db1 * assign6210_e6369) + (var_a_vdcctc * ((assign6210_e6367 * var_dxa_db1) / assign6210_e6368)))),)
    } else {
        (var_xvjcex, var_xvjcex_dn0, var_xvjcex_dn1, var_xvjcex_dn2, var_xvjcex_dn3, var_xvjcex_dn4, var_xvjcex_dn5, var_xvjcex_dn6, var_xvjcex_dn7, var_xvjcex_dn8, var_xvjcex_dn9, var_xvjcex_dn10, var_xvjcex_dn11, var_xvjcex_db0, var_xvjcex_db1,)
    }
};
        var_xvjcex = assign6210_e6373;
        var_xvjcex_dn0 = assign6210_e6373_d_n0;
        var_xvjcex_dn1 = assign6210_e6373_d_n1;
        var_xvjcex_dn2 = assign6210_e6373_d_n2;
        var_xvjcex_dn3 = assign6210_e6373_d_n3;
        var_xvjcex_dn4 = assign6210_e6373_d_n4;
        var_xvjcex_dn5 = assign6210_e6373_d_n5;
        var_xvjcex_dn6 = assign6210_e6373_d_n6;
        var_xvjcex_dn7 = assign6210_e6373_d_n7;
        var_xvjcex_dn8 = assign6210_e6373_d_n8;
        var_xvjcex_dn9 = assign6210_e6373_d_n9;
        var_xvjcex_dn10 = assign6210_e6373_d_n10;
        var_xvjcex_dn11 = assign6210_e6373_d_n11;
        var_xvjcex_db0 = assign6210_e6373_d_b0;
        var_xvjcex_db1 = assign6210_e6373_d_b1;

        let (assign6220_e6387, assign6220_e6387_d_n0, assign6220_e6387_d_n1, assign6220_e6387_d_n2, assign6220_e6387_d_n3, assign6220_e6387_d_n4, assign6220_e6387_d_n5, assign6220_e6387_d_n6, assign6220_e6387_d_n7, assign6220_e6387_d_n8, assign6220_e6387_d_n9, assign6220_e6387_d_n10, assign6220_e6387_d_n11, assign6220_e6387_d_b0, assign6220_e6387_d_b1,) = {
    if (var_guard112 == 0.0) {
        let assign6220_e6380: f64 = (-var_dxa);
        let assign6220_e6381: f64 = (assign6220_e6380).exp();
        let assign6220_e6382: f64 = (1.0 + assign6220_e6381);
        let assign6220_e6383: f64 = (assign6220_e6382).ln();
        let assign6220_e6384: f64 = (var_a_vdcctc * assign6220_e6383);
        let assign6220_e6385: f64 = (var_vfc - assign6220_e6384);
        (assign6220_e6385, (var_vfc_dn0 - ((var_a_vdcctc_dn0 * assign6220_e6383) + (var_a_vdcctc * ((assign6220_e6381 * (-var_dxa_dn0)) / assign6220_e6382)))), (var_vfc_dn1 - ((var_a_vdcctc_dn1 * assign6220_e6383) + (var_a_vdcctc * ((assign6220_e6381 * (-var_dxa_dn1)) / assign6220_e6382)))), (var_vfc_dn2 - ((var_a_vdcctc_dn2 * assign6220_e6383) + (var_a_vdcctc * ((assign6220_e6381 * (-var_dxa_dn2)) / assign6220_e6382)))), (var_vfc_dn3 - ((var_a_vdcctc_dn3 * assign6220_e6383) + (var_a_vdcctc * ((assign6220_e6381 * (-var_dxa_dn3)) / assign6220_e6382)))), (var_vfc_dn4 - ((var_a_vdcctc_dn4 * assign6220_e6383) + (var_a_vdcctc * ((assign6220_e6381 * (-var_dxa_dn4)) / assign6220_e6382)))), (var_vfc_dn5 - ((var_a_vdcctc_dn5 * assign6220_e6383) + (var_a_vdcctc * ((assign6220_e6381 * (-var_dxa_dn5)) / assign6220_e6382)))), (var_vfc_dn6 - ((var_a_vdcctc_dn6 * assign6220_e6383) + (var_a_vdcctc * ((assign6220_e6381 * (-var_dxa_dn6)) / assign6220_e6382)))), (var_vfc_dn7 - ((var_a_vdcctc_dn7 * assign6220_e6383) + (var_a_vdcctc * ((assign6220_e6381 * (-var_dxa_dn7)) / assign6220_e6382)))), (var_vfc_dn8 - ((var_a_vdcctc_dn8 * assign6220_e6383) + (var_a_vdcctc * ((assign6220_e6381 * (-var_dxa_dn8)) / assign6220_e6382)))), (var_vfc_dn9 - ((var_a_vdcctc_dn9 * assign6220_e6383) + (var_a_vdcctc * ((assign6220_e6381 * (-var_dxa_dn9)) / assign6220_e6382)))), (var_vfc_dn10 - ((var_a_vdcctc_dn10 * assign6220_e6383) + (var_a_vdcctc * ((assign6220_e6381 * (-var_dxa_dn10)) / assign6220_e6382)))), (var_vfc_dn11 - ((var_a_vdcctc_dn11 * assign6220_e6383) + (var_a_vdcctc * ((assign6220_e6381 * (-var_dxa_dn11)) / assign6220_e6382)))), (var_vfc_db0 - ((var_a_vdcctc_db0 * assign6220_e6383) + (var_a_vdcctc * ((assign6220_e6381 * (-var_dxa_db0)) / assign6220_e6382)))), (var_vfc_db1 - ((var_a_vdcctc_db1 * assign6220_e6383) + (var_a_vdcctc * ((assign6220_e6381 * (-var_dxa_db1)) / assign6220_e6382)))),)
    } else {
        (var_xvjcex, var_xvjcex_dn0, var_xvjcex_dn1, var_xvjcex_dn2, var_xvjcex_dn3, var_xvjcex_dn4, var_xvjcex_dn5, var_xvjcex_dn6, var_xvjcex_dn7, var_xvjcex_dn8, var_xvjcex_dn9, var_xvjcex_dn10, var_xvjcex_dn11, var_xvjcex_db0, var_xvjcex_db1,)
    }
};
        var_xvjcex = assign6220_e6387;
        var_xvjcex_dn0 = assign6220_e6387_d_n0;
        var_xvjcex_dn1 = assign6220_e6387_d_n1;
        var_xvjcex_dn2 = assign6220_e6387_d_n2;
        var_xvjcex_dn3 = assign6220_e6387_d_n3;
        var_xvjcex_dn4 = assign6220_e6387_d_n4;
        var_xvjcex_dn5 = assign6220_e6387_d_n5;
        var_xvjcex_dn6 = assign6220_e6387_d_n6;
        var_xvjcex_dn7 = assign6220_e6387_d_n7;
        var_xvjcex_dn8 = assign6220_e6387_d_n8;
        var_xvjcex_dn9 = assign6220_e6387_d_n9;
        var_xvjcex_dn10 = assign6220_e6387_d_n10;
        var_xvjcex_dn11 = assign6220_e6387_d_n11;
        var_xvjcex_db0 = assign6220_e6387_d_b0;
        var_xvjcex_db1 = assign6220_e6387_d_b1;

        let assign6230_e6391: f64 = (1.0 - p.p72);
        let assign6230_e6392: f64 = (var_vdc_ctc_t / assign6230_e6391);
        let assign6230_e6397: f64 = (var_xvjcex / var_vdc_ctc_t);
        let assign6230_e6398: f64 = (1.0 - assign6230_e6397);
        let assign6230_e6401: f64 = (1.0 - p.p72);
        let assign6230_e6402: f64 = (assign6230_e6398).powf(assign6230_e6401);
        let assign6230_e6403: f64 = (1.0 - assign6230_e6402);
        let assign6230_e6404: f64 = (assign6230_e6392 * assign6230_e6403);
        let assign6230_e6408: f64 = (var_vbc3 - var_xvjcex);
        let assign6230_e6409: f64 = (var_bjc * assign6230_e6408);
        let assign6230_e6410: f64 = (assign6230_e6404 + assign6230_e6409);
        var_xvtexv = assign6230_e6410;
        var_xvtexv_dn0 = ((((var_vdc_ctc_t_dn0 / assign6230_e6391) * assign6230_e6403) + (assign6230_e6392 * (-if 0.0 == 0.0 && ((assign6230_e6401) as f64).is_finite() && ((assign6230_e6401) as f64).fract() == 0.0 { if assign6230_e6401 == 0.0 { 0.0 } else { (assign6230_e6401 * ((assign6230_e6398).powf(assign6230_e6401 - 1.0) * (-(((var_xvjcex_dn0 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn0)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign6230_e6402 * (assign6230_e6401 * ((-(((var_xvjcex_dn0 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn0)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign6230_e6398))) }))) + ((var_bjc_dn0 * assign6230_e6408) + (var_bjc * (var_vbc3_dn0 - var_xvjcex_dn0))));
        var_xvtexv_dn1 = ((((var_vdc_ctc_t_dn1 / assign6230_e6391) * assign6230_e6403) + (assign6230_e6392 * (-if 0.0 == 0.0 && ((assign6230_e6401) as f64).is_finite() && ((assign6230_e6401) as f64).fract() == 0.0 { if assign6230_e6401 == 0.0 { 0.0 } else { (assign6230_e6401 * ((assign6230_e6398).powf(assign6230_e6401 - 1.0) * (-(((var_xvjcex_dn1 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn1)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign6230_e6402 * (assign6230_e6401 * ((-(((var_xvjcex_dn1 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn1)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign6230_e6398))) }))) + ((var_bjc_dn1 * assign6230_e6408) + (var_bjc * (var_vbc3_dn1 - var_xvjcex_dn1))));
        var_xvtexv_dn2 = ((((var_vdc_ctc_t_dn2 / assign6230_e6391) * assign6230_e6403) + (assign6230_e6392 * (-if 0.0 == 0.0 && ((assign6230_e6401) as f64).is_finite() && ((assign6230_e6401) as f64).fract() == 0.0 { if assign6230_e6401 == 0.0 { 0.0 } else { (assign6230_e6401 * ((assign6230_e6398).powf(assign6230_e6401 - 1.0) * (-(((var_xvjcex_dn2 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn2)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign6230_e6402 * (assign6230_e6401 * ((-(((var_xvjcex_dn2 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn2)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign6230_e6398))) }))) + ((var_bjc_dn2 * assign6230_e6408) + (var_bjc * (var_vbc3_dn2 - var_xvjcex_dn2))));
        var_xvtexv_dn3 = ((((var_vdc_ctc_t_dn3 / assign6230_e6391) * assign6230_e6403) + (assign6230_e6392 * (-if 0.0 == 0.0 && ((assign6230_e6401) as f64).is_finite() && ((assign6230_e6401) as f64).fract() == 0.0 { if assign6230_e6401 == 0.0 { 0.0 } else { (assign6230_e6401 * ((assign6230_e6398).powf(assign6230_e6401 - 1.0) * (-(((var_xvjcex_dn3 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn3)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign6230_e6402 * (assign6230_e6401 * ((-(((var_xvjcex_dn3 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn3)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign6230_e6398))) }))) + ((var_bjc_dn3 * assign6230_e6408) + (var_bjc * (var_vbc3_dn3 - var_xvjcex_dn3))));
        var_xvtexv_dn4 = ((((var_vdc_ctc_t_dn4 / assign6230_e6391) * assign6230_e6403) + (assign6230_e6392 * (-if 0.0 == 0.0 && ((assign6230_e6401) as f64).is_finite() && ((assign6230_e6401) as f64).fract() == 0.0 { if assign6230_e6401 == 0.0 { 0.0 } else { (assign6230_e6401 * ((assign6230_e6398).powf(assign6230_e6401 - 1.0) * (-(((var_xvjcex_dn4 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn4)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign6230_e6402 * (assign6230_e6401 * ((-(((var_xvjcex_dn4 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn4)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign6230_e6398))) }))) + ((var_bjc_dn4 * assign6230_e6408) + (var_bjc * (var_vbc3_dn4 - var_xvjcex_dn4))));
        var_xvtexv_dn5 = ((((var_vdc_ctc_t_dn5 / assign6230_e6391) * assign6230_e6403) + (assign6230_e6392 * (-if 0.0 == 0.0 && ((assign6230_e6401) as f64).is_finite() && ((assign6230_e6401) as f64).fract() == 0.0 { if assign6230_e6401 == 0.0 { 0.0 } else { (assign6230_e6401 * ((assign6230_e6398).powf(assign6230_e6401 - 1.0) * (-(((var_xvjcex_dn5 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn5)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign6230_e6402 * (assign6230_e6401 * ((-(((var_xvjcex_dn5 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn5)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign6230_e6398))) }))) + ((var_bjc_dn5 * assign6230_e6408) + (var_bjc * (var_vbc3_dn5 - var_xvjcex_dn5))));
        var_xvtexv_dn6 = ((((var_vdc_ctc_t_dn6 / assign6230_e6391) * assign6230_e6403) + (assign6230_e6392 * (-if 0.0 == 0.0 && ((assign6230_e6401) as f64).is_finite() && ((assign6230_e6401) as f64).fract() == 0.0 { if assign6230_e6401 == 0.0 { 0.0 } else { (assign6230_e6401 * ((assign6230_e6398).powf(assign6230_e6401 - 1.0) * (-(((var_xvjcex_dn6 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn6)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign6230_e6402 * (assign6230_e6401 * ((-(((var_xvjcex_dn6 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn6)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign6230_e6398))) }))) + ((var_bjc_dn6 * assign6230_e6408) + (var_bjc * (var_vbc3_dn6 - var_xvjcex_dn6))));
        var_xvtexv_dn7 = ((((var_vdc_ctc_t_dn7 / assign6230_e6391) * assign6230_e6403) + (assign6230_e6392 * (-if 0.0 == 0.0 && ((assign6230_e6401) as f64).is_finite() && ((assign6230_e6401) as f64).fract() == 0.0 { if assign6230_e6401 == 0.0 { 0.0 } else { (assign6230_e6401 * ((assign6230_e6398).powf(assign6230_e6401 - 1.0) * (-(((var_xvjcex_dn7 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn7)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign6230_e6402 * (assign6230_e6401 * ((-(((var_xvjcex_dn7 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn7)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign6230_e6398))) }))) + ((var_bjc_dn7 * assign6230_e6408) + (var_bjc * (var_vbc3_dn7 - var_xvjcex_dn7))));
        var_xvtexv_dn8 = ((((var_vdc_ctc_t_dn8 / assign6230_e6391) * assign6230_e6403) + (assign6230_e6392 * (-if 0.0 == 0.0 && ((assign6230_e6401) as f64).is_finite() && ((assign6230_e6401) as f64).fract() == 0.0 { if assign6230_e6401 == 0.0 { 0.0 } else { (assign6230_e6401 * ((assign6230_e6398).powf(assign6230_e6401 - 1.0) * (-(((var_xvjcex_dn8 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn8)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign6230_e6402 * (assign6230_e6401 * ((-(((var_xvjcex_dn8 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn8)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign6230_e6398))) }))) + ((var_bjc_dn8 * assign6230_e6408) + (var_bjc * (var_vbc3_dn8 - var_xvjcex_dn8))));
        var_xvtexv_dn9 = ((((var_vdc_ctc_t_dn9 / assign6230_e6391) * assign6230_e6403) + (assign6230_e6392 * (-if 0.0 == 0.0 && ((assign6230_e6401) as f64).is_finite() && ((assign6230_e6401) as f64).fract() == 0.0 { if assign6230_e6401 == 0.0 { 0.0 } else { (assign6230_e6401 * ((assign6230_e6398).powf(assign6230_e6401 - 1.0) * (-(((var_xvjcex_dn9 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn9)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign6230_e6402 * (assign6230_e6401 * ((-(((var_xvjcex_dn9 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn9)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign6230_e6398))) }))) + ((var_bjc_dn9 * assign6230_e6408) + (var_bjc * (var_vbc3_dn9 - var_xvjcex_dn9))));
        var_xvtexv_dn10 = ((((var_vdc_ctc_t_dn10 / assign6230_e6391) * assign6230_e6403) + (assign6230_e6392 * (-if 0.0 == 0.0 && ((assign6230_e6401) as f64).is_finite() && ((assign6230_e6401) as f64).fract() == 0.0 { if assign6230_e6401 == 0.0 { 0.0 } else { (assign6230_e6401 * ((assign6230_e6398).powf(assign6230_e6401 - 1.0) * (-(((var_xvjcex_dn10 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn10)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign6230_e6402 * (assign6230_e6401 * ((-(((var_xvjcex_dn10 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn10)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign6230_e6398))) }))) + ((var_bjc_dn10 * assign6230_e6408) + (var_bjc * (var_vbc3_dn10 - var_xvjcex_dn10))));
        var_xvtexv_dn11 = ((((var_vdc_ctc_t_dn11 / assign6230_e6391) * assign6230_e6403) + (assign6230_e6392 * (-if 0.0 == 0.0 && ((assign6230_e6401) as f64).is_finite() && ((assign6230_e6401) as f64).fract() == 0.0 { if assign6230_e6401 == 0.0 { 0.0 } else { (assign6230_e6401 * ((assign6230_e6398).powf(assign6230_e6401 - 1.0) * (-(((var_xvjcex_dn11 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn11)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign6230_e6402 * (assign6230_e6401 * ((-(((var_xvjcex_dn11 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn11)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign6230_e6398))) }))) + ((var_bjc_dn11 * assign6230_e6408) + (var_bjc * (var_vbc3_dn11 - var_xvjcex_dn11))));
        var_xvtexv_db0 = ((((var_vdc_ctc_t_db0 / assign6230_e6391) * assign6230_e6403) + (assign6230_e6392 * (-if 0.0 == 0.0 && ((assign6230_e6401) as f64).is_finite() && ((assign6230_e6401) as f64).fract() == 0.0 { if assign6230_e6401 == 0.0 { 0.0 } else { (assign6230_e6401 * ((assign6230_e6398).powf(assign6230_e6401 - 1.0) * (-(((var_xvjcex_db0 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_db0)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign6230_e6402 * (assign6230_e6401 * ((-(((var_xvjcex_db0 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_db0)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign6230_e6398))) }))) + ((var_bjc_db0 * assign6230_e6408) + (var_bjc * (var_vbc3_db0 - var_xvjcex_db0))));
        var_xvtexv_db1 = ((((var_vdc_ctc_t_db1 / assign6230_e6391) * assign6230_e6403) + (assign6230_e6392 * (-if 0.0 == 0.0 && ((assign6230_e6401) as f64).is_finite() && ((assign6230_e6401) as f64).fract() == 0.0 { if assign6230_e6401 == 0.0 { 0.0 } else { (assign6230_e6401 * ((assign6230_e6398).powf(assign6230_e6401 - 1.0) * (-(((var_xvjcex_db1 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_db1)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign6230_e6402 * (assign6230_e6401 * ((-(((var_xvjcex_db1 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_db1)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign6230_e6398))) }))) + ((var_bjc_db1 * assign6230_e6408) + (var_bjc * (var_vbc3_db1 - var_xvjcex_db1))));

        let assign6240_e6414: f64 = (1.0 - var_xp_t);
        let assign6240_e6416: f64 = (assign6240_e6414 * var_xvtexv);
        let assign6240_e6419: f64 = (var_xp_t * var_vbc3);
        let assign6240_e6420: f64 = (assign6240_e6416 + assign6240_e6419);
        let assign6240_e6421: f64 = (var_cjc_t * assign6240_e6420);
        let assign6240_e6424: f64 = (1.0 - p.p77);
        let assign6240_e6425: f64 = (assign6240_e6421 * assign6240_e6424);
        let assign6240_e6427: f64 = (assign6240_e6425 * p.p33);
        var_xqtex = assign6240_e6427;
        var_xqtex_dn0 = ((((var_cjc_t_dn0 * assign6240_e6420) + (var_cjc_t * ((((-var_xp_t_dn0) * var_xvtexv) + (assign6240_e6414 * var_xvtexv_dn0)) + ((var_xp_t_dn0 * var_vbc3) + (var_xp_t * var_vbc3_dn0))))) * assign6240_e6424) * p.p33);
        var_xqtex_dn1 = ((((var_cjc_t_dn1 * assign6240_e6420) + (var_cjc_t * ((((-var_xp_t_dn1) * var_xvtexv) + (assign6240_e6414 * var_xvtexv_dn1)) + ((var_xp_t_dn1 * var_vbc3) + (var_xp_t * var_vbc3_dn1))))) * assign6240_e6424) * p.p33);
        var_xqtex_dn2 = ((((var_cjc_t_dn2 * assign6240_e6420) + (var_cjc_t * ((((-var_xp_t_dn2) * var_xvtexv) + (assign6240_e6414 * var_xvtexv_dn2)) + ((var_xp_t_dn2 * var_vbc3) + (var_xp_t * var_vbc3_dn2))))) * assign6240_e6424) * p.p33);
        var_xqtex_dn3 = ((((var_cjc_t_dn3 * assign6240_e6420) + (var_cjc_t * ((((-var_xp_t_dn3) * var_xvtexv) + (assign6240_e6414 * var_xvtexv_dn3)) + ((var_xp_t_dn3 * var_vbc3) + (var_xp_t * var_vbc3_dn3))))) * assign6240_e6424) * p.p33);
        var_xqtex_dn4 = ((((var_cjc_t_dn4 * assign6240_e6420) + (var_cjc_t * ((((-var_xp_t_dn4) * var_xvtexv) + (assign6240_e6414 * var_xvtexv_dn4)) + ((var_xp_t_dn4 * var_vbc3) + (var_xp_t * var_vbc3_dn4))))) * assign6240_e6424) * p.p33);
        var_xqtex_dn5 = ((((var_cjc_t_dn5 * assign6240_e6420) + (var_cjc_t * ((((-var_xp_t_dn5) * var_xvtexv) + (assign6240_e6414 * var_xvtexv_dn5)) + ((var_xp_t_dn5 * var_vbc3) + (var_xp_t * var_vbc3_dn5))))) * assign6240_e6424) * p.p33);
        var_xqtex_dn6 = ((((var_cjc_t_dn6 * assign6240_e6420) + (var_cjc_t * ((((-var_xp_t_dn6) * var_xvtexv) + (assign6240_e6414 * var_xvtexv_dn6)) + ((var_xp_t_dn6 * var_vbc3) + (var_xp_t * var_vbc3_dn6))))) * assign6240_e6424) * p.p33);
        var_xqtex_dn7 = ((((var_cjc_t_dn7 * assign6240_e6420) + (var_cjc_t * ((((-var_xp_t_dn7) * var_xvtexv) + (assign6240_e6414 * var_xvtexv_dn7)) + ((var_xp_t_dn7 * var_vbc3) + (var_xp_t * var_vbc3_dn7))))) * assign6240_e6424) * p.p33);
        var_xqtex_dn8 = ((((var_cjc_t_dn8 * assign6240_e6420) + (var_cjc_t * ((((-var_xp_t_dn8) * var_xvtexv) + (assign6240_e6414 * var_xvtexv_dn8)) + ((var_xp_t_dn8 * var_vbc3) + (var_xp_t * var_vbc3_dn8))))) * assign6240_e6424) * p.p33);
        var_xqtex_dn9 = ((((var_cjc_t_dn9 * assign6240_e6420) + (var_cjc_t * ((((-var_xp_t_dn9) * var_xvtexv) + (assign6240_e6414 * var_xvtexv_dn9)) + ((var_xp_t_dn9 * var_vbc3) + (var_xp_t * var_vbc3_dn9))))) * assign6240_e6424) * p.p33);
        var_xqtex_dn10 = ((((var_cjc_t_dn10 * assign6240_e6420) + (var_cjc_t * ((((-var_xp_t_dn10) * var_xvtexv) + (assign6240_e6414 * var_xvtexv_dn10)) + ((var_xp_t_dn10 * var_vbc3) + (var_xp_t * var_vbc3_dn10))))) * assign6240_e6424) * p.p33);
        var_xqtex_dn11 = ((((var_cjc_t_dn11 * assign6240_e6420) + (var_cjc_t * ((((-var_xp_t_dn11) * var_xvtexv) + (assign6240_e6414 * var_xvtexv_dn11)) + ((var_xp_t_dn11 * var_vbc3) + (var_xp_t * var_vbc3_dn11))))) * assign6240_e6424) * p.p33);
        var_xqtex_db0 = ((((var_cjc_t_db0 * assign6240_e6420) + (var_cjc_t * ((((-var_xp_t_db0) * var_xvtexv) + (assign6240_e6414 * var_xvtexv_db0)) + ((var_xp_t_db0 * var_vbc3) + (var_xp_t * var_vbc3_db0))))) * assign6240_e6424) * p.p33);
        var_xqtex_db1 = ((((var_cjc_t_db1 * assign6240_e6420) + (var_cjc_t * ((((-var_xp_t_db1) * var_xvtexv) + (assign6240_e6414 * var_xvtexv_db1)) + ((var_xp_t_db1 * var_vbc3) + (var_xp_t * var_vbc3_db1))))) * assign6240_e6424) * p.p33);

        let assign6250_e6430: f64 = (0.1 * var_vds_t);
        var_a_vds = assign6250_e6430;
        var_a_vds_dn0 = (0.1 * var_vds_t_dn0);
        var_a_vds_dn1 = (0.1 * var_vds_t_dn1);
        var_a_vds_dn2 = (0.1 * var_vds_t_dn2);
        var_a_vds_dn3 = (0.1 * var_vds_t_dn3);
        var_a_vds_dn4 = (0.1 * var_vds_t_dn4);
        var_a_vds_dn5 = (0.1 * var_vds_t_dn5);
        var_a_vds_dn6 = (0.1 * var_vds_t_dn6);
        var_a_vds_dn7 = (0.1 * var_vds_t_dn7);
        var_a_vds_dn8 = (0.1 * var_vds_t_dn8);
        var_a_vds_dn9 = (0.1 * var_vds_t_dn9);
        var_a_vds_dn10 = (0.1 * var_vds_t_dn10);
        var_a_vds_dn11 = (0.1 * var_vds_t_dn11);
        var_a_vds_db0 = (0.1 * var_vds_t_db0);
        var_a_vds_db1 = (0.1 * var_vds_t_db1);

        let assign6260_e6435: f64 = (-1.0);
        let assign6260_e6437: f64 = (assign6260_e6435 / p.p139);
        let assign6260_e6438: f64 = (2.0_f64).powf(assign6260_e6437);
        let assign6260_e6439: f64 = (1.0 - assign6260_e6438);
        let assign6260_e6440: f64 = (var_vds_t * assign6260_e6439);
        var_vfs = assign6260_e6440;
        var_vfs_dn0 = (var_vds_t_dn0 * assign6260_e6439);
        var_vfs_dn1 = (var_vds_t_dn1 * assign6260_e6439);
        var_vfs_dn2 = (var_vds_t_dn2 * assign6260_e6439);
        var_vfs_dn3 = (var_vds_t_dn3 * assign6260_e6439);
        var_vfs_dn4 = (var_vds_t_dn4 * assign6260_e6439);
        var_vfs_dn5 = (var_vds_t_dn5 * assign6260_e6439);
        var_vfs_dn6 = (var_vds_t_dn6 * assign6260_e6439);
        var_vfs_dn7 = (var_vds_t_dn7 * assign6260_e6439);
        var_vfs_dn8 = (var_vds_t_dn8 * assign6260_e6439);
        var_vfs_dn9 = (var_vds_t_dn9 * assign6260_e6439);
        var_vfs_dn10 = (var_vds_t_dn10 * assign6260_e6439);
        var_vfs_dn11 = (var_vds_t_dn11 * assign6260_e6439);
        var_vfs_db0 = (var_vds_t_db0 * assign6260_e6439);
        var_vfs_db1 = (var_vds_t_db1 * assign6260_e6439);

        let assign6270_e6443: f64 = (var_vsc1 - var_vfs);
        let assign6270_e6445: f64 = (assign6270_e6443 / var_a_vds);
        var_dxa = assign6270_e6445;
        var_dxa_dn0 = ((((var_vsc1_dn0 - var_vfs_dn0) * var_a_vds) - (assign6270_e6443 * var_a_vds_dn0)) / (var_a_vds * var_a_vds));
        var_dxa_dn1 = ((((var_vsc1_dn1 - var_vfs_dn1) * var_a_vds) - (assign6270_e6443 * var_a_vds_dn1)) / (var_a_vds * var_a_vds));
        var_dxa_dn2 = ((((var_vsc1_dn2 - var_vfs_dn2) * var_a_vds) - (assign6270_e6443 * var_a_vds_dn2)) / (var_a_vds * var_a_vds));
        var_dxa_dn3 = ((((var_vsc1_dn3 - var_vfs_dn3) * var_a_vds) - (assign6270_e6443 * var_a_vds_dn3)) / (var_a_vds * var_a_vds));
        var_dxa_dn4 = ((((var_vsc1_dn4 - var_vfs_dn4) * var_a_vds) - (assign6270_e6443 * var_a_vds_dn4)) / (var_a_vds * var_a_vds));
        var_dxa_dn5 = ((((var_vsc1_dn5 - var_vfs_dn5) * var_a_vds) - (assign6270_e6443 * var_a_vds_dn5)) / (var_a_vds * var_a_vds));
        var_dxa_dn6 = ((((var_vsc1_dn6 - var_vfs_dn6) * var_a_vds) - (assign6270_e6443 * var_a_vds_dn6)) / (var_a_vds * var_a_vds));
        var_dxa_dn7 = ((((var_vsc1_dn7 - var_vfs_dn7) * var_a_vds) - (assign6270_e6443 * var_a_vds_dn7)) / (var_a_vds * var_a_vds));
        var_dxa_dn8 = ((((var_vsc1_dn8 - var_vfs_dn8) * var_a_vds) - (assign6270_e6443 * var_a_vds_dn8)) / (var_a_vds * var_a_vds));
        var_dxa_dn9 = ((((var_vsc1_dn9 - var_vfs_dn9) * var_a_vds) - (assign6270_e6443 * var_a_vds_dn9)) / (var_a_vds * var_a_vds));
        var_dxa_dn10 = ((((var_vsc1_dn10 - var_vfs_dn10) * var_a_vds) - (assign6270_e6443 * var_a_vds_dn10)) / (var_a_vds * var_a_vds));
        var_dxa_dn11 = ((((var_vsc1_dn11 - var_vfs_dn11) * var_a_vds) - (assign6270_e6443 * var_a_vds_dn11)) / (var_a_vds * var_a_vds));
        var_dxa_db0 = ((((var_vsc1_db0 - var_vfs_db0) * var_a_vds) - (assign6270_e6443 * var_a_vds_db0)) / (var_a_vds * var_a_vds));
        var_dxa_db1 = ((((var_vsc1_db1 - var_vfs_db1) * var_a_vds) - (assign6270_e6443 * var_a_vds_db1)) / (var_a_vds * var_a_vds));

        let assign6280_e6448: f64 = if var_vsc1 < var_vfs { 1.0 } else { 0.0 };
        var_guard113 = assign6280_e6448;

        let (assign6290_e6460, assign6290_e6460_d_n0, assign6290_e6460_d_n1, assign6290_e6460_d_n2, assign6290_e6460_d_n3, assign6290_e6460_d_n4, assign6290_e6460_d_n5, assign6290_e6460_d_n6, assign6290_e6460_d_n7, assign6290_e6460_d_n8, assign6290_e6460_d_n9, assign6290_e6460_d_n10, assign6290_e6460_d_n11, assign6290_e6460_d_b0, assign6290_e6460_d_b1,) = {
    if (var_guard113 != 0.0) {
        let assign6290_e6454: f64 = (var_dxa).exp();
        let assign6290_e6455: f64 = (1.0 + assign6290_e6454);
        let assign6290_e6456: f64 = (assign6290_e6455).ln();
        let assign6290_e6457: f64 = (var_a_vds * assign6290_e6456);
        let assign6290_e6458: f64 = (var_vsc1 - assign6290_e6457);
        (assign6290_e6458, (var_vsc1_dn0 - ((var_a_vds_dn0 * assign6290_e6456) + (var_a_vds * ((assign6290_e6454 * var_dxa_dn0) / assign6290_e6455)))), (var_vsc1_dn1 - ((var_a_vds_dn1 * assign6290_e6456) + (var_a_vds * ((assign6290_e6454 * var_dxa_dn1) / assign6290_e6455)))), (var_vsc1_dn2 - ((var_a_vds_dn2 * assign6290_e6456) + (var_a_vds * ((assign6290_e6454 * var_dxa_dn2) / assign6290_e6455)))), (var_vsc1_dn3 - ((var_a_vds_dn3 * assign6290_e6456) + (var_a_vds * ((assign6290_e6454 * var_dxa_dn3) / assign6290_e6455)))), (var_vsc1_dn4 - ((var_a_vds_dn4 * assign6290_e6456) + (var_a_vds * ((assign6290_e6454 * var_dxa_dn4) / assign6290_e6455)))), (var_vsc1_dn5 - ((var_a_vds_dn5 * assign6290_e6456) + (var_a_vds * ((assign6290_e6454 * var_dxa_dn5) / assign6290_e6455)))), (var_vsc1_dn6 - ((var_a_vds_dn6 * assign6290_e6456) + (var_a_vds * ((assign6290_e6454 * var_dxa_dn6) / assign6290_e6455)))), (var_vsc1_dn7 - ((var_a_vds_dn7 * assign6290_e6456) + (var_a_vds * ((assign6290_e6454 * var_dxa_dn7) / assign6290_e6455)))), (var_vsc1_dn8 - ((var_a_vds_dn8 * assign6290_e6456) + (var_a_vds * ((assign6290_e6454 * var_dxa_dn8) / assign6290_e6455)))), (var_vsc1_dn9 - ((var_a_vds_dn9 * assign6290_e6456) + (var_a_vds * ((assign6290_e6454 * var_dxa_dn9) / assign6290_e6455)))), (var_vsc1_dn10 - ((var_a_vds_dn10 * assign6290_e6456) + (var_a_vds * ((assign6290_e6454 * var_dxa_dn10) / assign6290_e6455)))), (var_vsc1_dn11 - ((var_a_vds_dn11 * assign6290_e6456) + (var_a_vds * ((assign6290_e6454 * var_dxa_dn11) / assign6290_e6455)))), (var_vsc1_db0 - ((var_a_vds_db0 * assign6290_e6456) + (var_a_vds * ((assign6290_e6454 * var_dxa_db0) / assign6290_e6455)))), (var_vsc1_db1 - ((var_a_vds_db1 * assign6290_e6456) + (var_a_vds * ((assign6290_e6454 * var_dxa_db1) / assign6290_e6455)))),)
    } else {
        (var_vjs, var_vjs_dn0, var_vjs_dn1, var_vjs_dn2, var_vjs_dn3, var_vjs_dn4, var_vjs_dn5, var_vjs_dn6, var_vjs_dn7, var_vjs_dn8, var_vjs_dn9, var_vjs_dn10, var_vjs_dn11, var_vjs_db0, var_vjs_db1,)
    }
};
        var_vjs = assign6290_e6460;
        var_vjs_dn0 = assign6290_e6460_d_n0;
        var_vjs_dn1 = assign6290_e6460_d_n1;
        var_vjs_dn2 = assign6290_e6460_d_n2;
        var_vjs_dn3 = assign6290_e6460_d_n3;
        var_vjs_dn4 = assign6290_e6460_d_n4;
        var_vjs_dn5 = assign6290_e6460_d_n5;
        var_vjs_dn6 = assign6290_e6460_d_n6;
        var_vjs_dn7 = assign6290_e6460_d_n7;
        var_vjs_dn8 = assign6290_e6460_d_n8;
        var_vjs_dn9 = assign6290_e6460_d_n9;
        var_vjs_dn10 = assign6290_e6460_d_n10;
        var_vjs_dn11 = assign6290_e6460_d_n11;
        var_vjs_db0 = assign6290_e6460_d_b0;
        var_vjs_db1 = assign6290_e6460_d_b1;

        let (assign6300_e6474, assign6300_e6474_d_n0, assign6300_e6474_d_n1, assign6300_e6474_d_n2, assign6300_e6474_d_n3, assign6300_e6474_d_n4, assign6300_e6474_d_n5, assign6300_e6474_d_n6, assign6300_e6474_d_n7, assign6300_e6474_d_n8, assign6300_e6474_d_n9, assign6300_e6474_d_n10, assign6300_e6474_d_n11, assign6300_e6474_d_b0, assign6300_e6474_d_b1,) = {
    if (var_guard113 == 0.0) {
        let assign6300_e6467: f64 = (-var_dxa);
        let assign6300_e6468: f64 = (assign6300_e6467).exp();
        let assign6300_e6469: f64 = (1.0 + assign6300_e6468);
        let assign6300_e6470: f64 = (assign6300_e6469).ln();
        let assign6300_e6471: f64 = (var_a_vds * assign6300_e6470);
        let assign6300_e6472: f64 = (var_vfs - assign6300_e6471);
        (assign6300_e6472, (var_vfs_dn0 - ((var_a_vds_dn0 * assign6300_e6470) + (var_a_vds * ((assign6300_e6468 * (-var_dxa_dn0)) / assign6300_e6469)))), (var_vfs_dn1 - ((var_a_vds_dn1 * assign6300_e6470) + (var_a_vds * ((assign6300_e6468 * (-var_dxa_dn1)) / assign6300_e6469)))), (var_vfs_dn2 - ((var_a_vds_dn2 * assign6300_e6470) + (var_a_vds * ((assign6300_e6468 * (-var_dxa_dn2)) / assign6300_e6469)))), (var_vfs_dn3 - ((var_a_vds_dn3 * assign6300_e6470) + (var_a_vds * ((assign6300_e6468 * (-var_dxa_dn3)) / assign6300_e6469)))), (var_vfs_dn4 - ((var_a_vds_dn4 * assign6300_e6470) + (var_a_vds * ((assign6300_e6468 * (-var_dxa_dn4)) / assign6300_e6469)))), (var_vfs_dn5 - ((var_a_vds_dn5 * assign6300_e6470) + (var_a_vds * ((assign6300_e6468 * (-var_dxa_dn5)) / assign6300_e6469)))), (var_vfs_dn6 - ((var_a_vds_dn6 * assign6300_e6470) + (var_a_vds * ((assign6300_e6468 * (-var_dxa_dn6)) / assign6300_e6469)))), (var_vfs_dn7 - ((var_a_vds_dn7 * assign6300_e6470) + (var_a_vds * ((assign6300_e6468 * (-var_dxa_dn7)) / assign6300_e6469)))), (var_vfs_dn8 - ((var_a_vds_dn8 * assign6300_e6470) + (var_a_vds * ((assign6300_e6468 * (-var_dxa_dn8)) / assign6300_e6469)))), (var_vfs_dn9 - ((var_a_vds_dn9 * assign6300_e6470) + (var_a_vds * ((assign6300_e6468 * (-var_dxa_dn9)) / assign6300_e6469)))), (var_vfs_dn10 - ((var_a_vds_dn10 * assign6300_e6470) + (var_a_vds * ((assign6300_e6468 * (-var_dxa_dn10)) / assign6300_e6469)))), (var_vfs_dn11 - ((var_a_vds_dn11 * assign6300_e6470) + (var_a_vds * ((assign6300_e6468 * (-var_dxa_dn11)) / assign6300_e6469)))), (var_vfs_db0 - ((var_a_vds_db0 * assign6300_e6470) + (var_a_vds * ((assign6300_e6468 * (-var_dxa_db0)) / assign6300_e6469)))), (var_vfs_db1 - ((var_a_vds_db1 * assign6300_e6470) + (var_a_vds * ((assign6300_e6468 * (-var_dxa_db1)) / assign6300_e6469)))),)
    } else {
        (var_vjs, var_vjs_dn0, var_vjs_dn1, var_vjs_dn2, var_vjs_dn3, var_vjs_dn4, var_vjs_dn5, var_vjs_dn6, var_vjs_dn7, var_vjs_dn8, var_vjs_dn9, var_vjs_dn10, var_vjs_dn11, var_vjs_db0, var_vjs_db1,)
    }
};
        var_vjs = assign6300_e6474;
        var_vjs_dn0 = assign6300_e6474_d_n0;
        var_vjs_dn1 = assign6300_e6474_d_n1;
        var_vjs_dn2 = assign6300_e6474_d_n2;
        var_vjs_dn3 = assign6300_e6474_d_n3;
        var_vjs_dn4 = assign6300_e6474_d_n4;
        var_vjs_dn5 = assign6300_e6474_d_n5;
        var_vjs_dn6 = assign6300_e6474_d_n6;
        var_vjs_dn7 = assign6300_e6474_d_n7;
        var_vjs_dn8 = assign6300_e6474_d_n8;
        var_vjs_dn9 = assign6300_e6474_d_n9;
        var_vjs_dn10 = assign6300_e6474_d_n10;
        var_vjs_dn11 = assign6300_e6474_d_n11;
        var_vjs_db0 = assign6300_e6474_d_b0;
        var_vjs_db1 = assign6300_e6474_d_b1;

        let assign6310_e6479: f64 = (1.0 - p.p139);
        let assign6310_e6480: f64 = (var_vds_t / assign6310_e6479);
        let assign6310_e6485: f64 = (var_vjs / var_vds_t);
        let assign6310_e6486: f64 = (1.0 - assign6310_e6485);
        let assign6310_e6489: f64 = (1.0 - p.p139);
        let assign6310_e6490: f64 = (assign6310_e6486).powf(assign6310_e6489);
        let assign6310_e6491: f64 = (1.0 - assign6310_e6490);
        let assign6310_e6492: f64 = (assign6310_e6480 * assign6310_e6491);
        let assign6310_e6496: f64 = (var_vsc1 - var_vjs);
        let assign6310_e6497: f64 = (2.0 * assign6310_e6496);
        let assign6310_e6498: f64 = (assign6310_e6492 + assign6310_e6497);
        let assign6310_e6499: f64 = (var_cjs_t * assign6310_e6498);
        var_qts = assign6310_e6499;
        var_qts_dn0 = ((var_cjs_t_dn0 * assign6310_e6498) + (var_cjs_t * ((((var_vds_t_dn0 / assign6310_e6479) * assign6310_e6491) + (assign6310_e6480 * (-if 0.0 == 0.0 && ((assign6310_e6489) as f64).is_finite() && ((assign6310_e6489) as f64).fract() == 0.0 { if assign6310_e6489 == 0.0 { 0.0 } else { (assign6310_e6489 * ((assign6310_e6486).powf(assign6310_e6489 - 1.0) * (-(((var_vjs_dn0 * var_vds_t) - (var_vjs * var_vds_t_dn0)) / (var_vds_t * var_vds_t))))) } } else { (assign6310_e6490 * (assign6310_e6489 * ((-(((var_vjs_dn0 * var_vds_t) - (var_vjs * var_vds_t_dn0)) / (var_vds_t * var_vds_t))) / assign6310_e6486))) }))) + (2.0 * (var_vsc1_dn0 - var_vjs_dn0)))));
        var_qts_dn1 = ((var_cjs_t_dn1 * assign6310_e6498) + (var_cjs_t * ((((var_vds_t_dn1 / assign6310_e6479) * assign6310_e6491) + (assign6310_e6480 * (-if 0.0 == 0.0 && ((assign6310_e6489) as f64).is_finite() && ((assign6310_e6489) as f64).fract() == 0.0 { if assign6310_e6489 == 0.0 { 0.0 } else { (assign6310_e6489 * ((assign6310_e6486).powf(assign6310_e6489 - 1.0) * (-(((var_vjs_dn1 * var_vds_t) - (var_vjs * var_vds_t_dn1)) / (var_vds_t * var_vds_t))))) } } else { (assign6310_e6490 * (assign6310_e6489 * ((-(((var_vjs_dn1 * var_vds_t) - (var_vjs * var_vds_t_dn1)) / (var_vds_t * var_vds_t))) / assign6310_e6486))) }))) + (2.0 * (var_vsc1_dn1 - var_vjs_dn1)))));
        var_qts_dn2 = ((var_cjs_t_dn2 * assign6310_e6498) + (var_cjs_t * ((((var_vds_t_dn2 / assign6310_e6479) * assign6310_e6491) + (assign6310_e6480 * (-if 0.0 == 0.0 && ((assign6310_e6489) as f64).is_finite() && ((assign6310_e6489) as f64).fract() == 0.0 { if assign6310_e6489 == 0.0 { 0.0 } else { (assign6310_e6489 * ((assign6310_e6486).powf(assign6310_e6489 - 1.0) * (-(((var_vjs_dn2 * var_vds_t) - (var_vjs * var_vds_t_dn2)) / (var_vds_t * var_vds_t))))) } } else { (assign6310_e6490 * (assign6310_e6489 * ((-(((var_vjs_dn2 * var_vds_t) - (var_vjs * var_vds_t_dn2)) / (var_vds_t * var_vds_t))) / assign6310_e6486))) }))) + (2.0 * (var_vsc1_dn2 - var_vjs_dn2)))));
        var_qts_dn3 = ((var_cjs_t_dn3 * assign6310_e6498) + (var_cjs_t * ((((var_vds_t_dn3 / assign6310_e6479) * assign6310_e6491) + (assign6310_e6480 * (-if 0.0 == 0.0 && ((assign6310_e6489) as f64).is_finite() && ((assign6310_e6489) as f64).fract() == 0.0 { if assign6310_e6489 == 0.0 { 0.0 } else { (assign6310_e6489 * ((assign6310_e6486).powf(assign6310_e6489 - 1.0) * (-(((var_vjs_dn3 * var_vds_t) - (var_vjs * var_vds_t_dn3)) / (var_vds_t * var_vds_t))))) } } else { (assign6310_e6490 * (assign6310_e6489 * ((-(((var_vjs_dn3 * var_vds_t) - (var_vjs * var_vds_t_dn3)) / (var_vds_t * var_vds_t))) / assign6310_e6486))) }))) + (2.0 * (var_vsc1_dn3 - var_vjs_dn3)))));
        var_qts_dn4 = ((var_cjs_t_dn4 * assign6310_e6498) + (var_cjs_t * ((((var_vds_t_dn4 / assign6310_e6479) * assign6310_e6491) + (assign6310_e6480 * (-if 0.0 == 0.0 && ((assign6310_e6489) as f64).is_finite() && ((assign6310_e6489) as f64).fract() == 0.0 { if assign6310_e6489 == 0.0 { 0.0 } else { (assign6310_e6489 * ((assign6310_e6486).powf(assign6310_e6489 - 1.0) * (-(((var_vjs_dn4 * var_vds_t) - (var_vjs * var_vds_t_dn4)) / (var_vds_t * var_vds_t))))) } } else { (assign6310_e6490 * (assign6310_e6489 * ((-(((var_vjs_dn4 * var_vds_t) - (var_vjs * var_vds_t_dn4)) / (var_vds_t * var_vds_t))) / assign6310_e6486))) }))) + (2.0 * (var_vsc1_dn4 - var_vjs_dn4)))));
        var_qts_dn5 = ((var_cjs_t_dn5 * assign6310_e6498) + (var_cjs_t * ((((var_vds_t_dn5 / assign6310_e6479) * assign6310_e6491) + (assign6310_e6480 * (-if 0.0 == 0.0 && ((assign6310_e6489) as f64).is_finite() && ((assign6310_e6489) as f64).fract() == 0.0 { if assign6310_e6489 == 0.0 { 0.0 } else { (assign6310_e6489 * ((assign6310_e6486).powf(assign6310_e6489 - 1.0) * (-(((var_vjs_dn5 * var_vds_t) - (var_vjs * var_vds_t_dn5)) / (var_vds_t * var_vds_t))))) } } else { (assign6310_e6490 * (assign6310_e6489 * ((-(((var_vjs_dn5 * var_vds_t) - (var_vjs * var_vds_t_dn5)) / (var_vds_t * var_vds_t))) / assign6310_e6486))) }))) + (2.0 * (var_vsc1_dn5 - var_vjs_dn5)))));
        var_qts_dn6 = ((var_cjs_t_dn6 * assign6310_e6498) + (var_cjs_t * ((((var_vds_t_dn6 / assign6310_e6479) * assign6310_e6491) + (assign6310_e6480 * (-if 0.0 == 0.0 && ((assign6310_e6489) as f64).is_finite() && ((assign6310_e6489) as f64).fract() == 0.0 { if assign6310_e6489 == 0.0 { 0.0 } else { (assign6310_e6489 * ((assign6310_e6486).powf(assign6310_e6489 - 1.0) * (-(((var_vjs_dn6 * var_vds_t) - (var_vjs * var_vds_t_dn6)) / (var_vds_t * var_vds_t))))) } } else { (assign6310_e6490 * (assign6310_e6489 * ((-(((var_vjs_dn6 * var_vds_t) - (var_vjs * var_vds_t_dn6)) / (var_vds_t * var_vds_t))) / assign6310_e6486))) }))) + (2.0 * (var_vsc1_dn6 - var_vjs_dn6)))));
        var_qts_dn7 = ((var_cjs_t_dn7 * assign6310_e6498) + (var_cjs_t * ((((var_vds_t_dn7 / assign6310_e6479) * assign6310_e6491) + (assign6310_e6480 * (-if 0.0 == 0.0 && ((assign6310_e6489) as f64).is_finite() && ((assign6310_e6489) as f64).fract() == 0.0 { if assign6310_e6489 == 0.0 { 0.0 } else { (assign6310_e6489 * ((assign6310_e6486).powf(assign6310_e6489 - 1.0) * (-(((var_vjs_dn7 * var_vds_t) - (var_vjs * var_vds_t_dn7)) / (var_vds_t * var_vds_t))))) } } else { (assign6310_e6490 * (assign6310_e6489 * ((-(((var_vjs_dn7 * var_vds_t) - (var_vjs * var_vds_t_dn7)) / (var_vds_t * var_vds_t))) / assign6310_e6486))) }))) + (2.0 * (var_vsc1_dn7 - var_vjs_dn7)))));
        var_qts_dn8 = ((var_cjs_t_dn8 * assign6310_e6498) + (var_cjs_t * ((((var_vds_t_dn8 / assign6310_e6479) * assign6310_e6491) + (assign6310_e6480 * (-if 0.0 == 0.0 && ((assign6310_e6489) as f64).is_finite() && ((assign6310_e6489) as f64).fract() == 0.0 { if assign6310_e6489 == 0.0 { 0.0 } else { (assign6310_e6489 * ((assign6310_e6486).powf(assign6310_e6489 - 1.0) * (-(((var_vjs_dn8 * var_vds_t) - (var_vjs * var_vds_t_dn8)) / (var_vds_t * var_vds_t))))) } } else { (assign6310_e6490 * (assign6310_e6489 * ((-(((var_vjs_dn8 * var_vds_t) - (var_vjs * var_vds_t_dn8)) / (var_vds_t * var_vds_t))) / assign6310_e6486))) }))) + (2.0 * (var_vsc1_dn8 - var_vjs_dn8)))));
        var_qts_dn9 = ((var_cjs_t_dn9 * assign6310_e6498) + (var_cjs_t * ((((var_vds_t_dn9 / assign6310_e6479) * assign6310_e6491) + (assign6310_e6480 * (-if 0.0 == 0.0 && ((assign6310_e6489) as f64).is_finite() && ((assign6310_e6489) as f64).fract() == 0.0 { if assign6310_e6489 == 0.0 { 0.0 } else { (assign6310_e6489 * ((assign6310_e6486).powf(assign6310_e6489 - 1.0) * (-(((var_vjs_dn9 * var_vds_t) - (var_vjs * var_vds_t_dn9)) / (var_vds_t * var_vds_t))))) } } else { (assign6310_e6490 * (assign6310_e6489 * ((-(((var_vjs_dn9 * var_vds_t) - (var_vjs * var_vds_t_dn9)) / (var_vds_t * var_vds_t))) / assign6310_e6486))) }))) + (2.0 * (var_vsc1_dn9 - var_vjs_dn9)))));
        var_qts_dn10 = ((var_cjs_t_dn10 * assign6310_e6498) + (var_cjs_t * ((((var_vds_t_dn10 / assign6310_e6479) * assign6310_e6491) + (assign6310_e6480 * (-if 0.0 == 0.0 && ((assign6310_e6489) as f64).is_finite() && ((assign6310_e6489) as f64).fract() == 0.0 { if assign6310_e6489 == 0.0 { 0.0 } else { (assign6310_e6489 * ((assign6310_e6486).powf(assign6310_e6489 - 1.0) * (-(((var_vjs_dn10 * var_vds_t) - (var_vjs * var_vds_t_dn10)) / (var_vds_t * var_vds_t))))) } } else { (assign6310_e6490 * (assign6310_e6489 * ((-(((var_vjs_dn10 * var_vds_t) - (var_vjs * var_vds_t_dn10)) / (var_vds_t * var_vds_t))) / assign6310_e6486))) }))) + (2.0 * (var_vsc1_dn10 - var_vjs_dn10)))));
        var_qts_dn11 = ((var_cjs_t_dn11 * assign6310_e6498) + (var_cjs_t * ((((var_vds_t_dn11 / assign6310_e6479) * assign6310_e6491) + (assign6310_e6480 * (-if 0.0 == 0.0 && ((assign6310_e6489) as f64).is_finite() && ((assign6310_e6489) as f64).fract() == 0.0 { if assign6310_e6489 == 0.0 { 0.0 } else { (assign6310_e6489 * ((assign6310_e6486).powf(assign6310_e6489 - 1.0) * (-(((var_vjs_dn11 * var_vds_t) - (var_vjs * var_vds_t_dn11)) / (var_vds_t * var_vds_t))))) } } else { (assign6310_e6490 * (assign6310_e6489 * ((-(((var_vjs_dn11 * var_vds_t) - (var_vjs * var_vds_t_dn11)) / (var_vds_t * var_vds_t))) / assign6310_e6486))) }))) + (2.0 * (var_vsc1_dn11 - var_vjs_dn11)))));
        var_qts_db0 = ((var_cjs_t_db0 * assign6310_e6498) + (var_cjs_t * ((((var_vds_t_db0 / assign6310_e6479) * assign6310_e6491) + (assign6310_e6480 * (-if 0.0 == 0.0 && ((assign6310_e6489) as f64).is_finite() && ((assign6310_e6489) as f64).fract() == 0.0 { if assign6310_e6489 == 0.0 { 0.0 } else { (assign6310_e6489 * ((assign6310_e6486).powf(assign6310_e6489 - 1.0) * (-(((var_vjs_db0 * var_vds_t) - (var_vjs * var_vds_t_db0)) / (var_vds_t * var_vds_t))))) } } else { (assign6310_e6490 * (assign6310_e6489 * ((-(((var_vjs_db0 * var_vds_t) - (var_vjs * var_vds_t_db0)) / (var_vds_t * var_vds_t))) / assign6310_e6486))) }))) + (2.0 * (var_vsc1_db0 - var_vjs_db0)))));
        var_qts_db1 = ((var_cjs_t_db1 * assign6310_e6498) + (var_cjs_t * ((((var_vds_t_db1 / assign6310_e6479) * assign6310_e6491) + (assign6310_e6480 * (-if 0.0 == 0.0 && ((assign6310_e6489) as f64).is_finite() && ((assign6310_e6489) as f64).fract() == 0.0 { if assign6310_e6489 == 0.0 { 0.0 } else { (assign6310_e6489 * ((assign6310_e6486).powf(assign6310_e6489 - 1.0) * (-(((var_vjs_db1 * var_vds_t) - (var_vjs * var_vds_t_db1)) / (var_vds_t * var_vds_t))))) } } else { (assign6310_e6490 * (assign6310_e6489 * ((-(((var_vjs_db1 * var_vds_t) - (var_vjs * var_vds_t_db1)) / (var_vds_t * var_vds_t))) / assign6310_e6486))) }))) + (2.0 * (var_vsc1_db1 - var_vjs_db1)))));

        let assign6320_e6502: f64 = (var_taue_t * var_ik_t);
        let assign6320_e6505: f64 = (var_is_t / var_ik_t);
        let assign6320_e6508: f64 = (1.0 / p.p85);
        let assign6320_e6509: f64 = (assign6320_e6505).powf(assign6320_e6508);
        let assign6320_e6510: f64 = (assign6320_e6502 * assign6320_e6509);
        var_qe0 = assign6320_e6510;
        var_qe0_dn0 = (assign6320_e6502 * if 0.0 == 0.0 && ((assign6320_e6508) as f64).is_finite() && ((assign6320_e6508) as f64).fract() == 0.0 { if assign6320_e6508 == 0.0 { 0.0 } else { (assign6320_e6508 * ((assign6320_e6505).powf(assign6320_e6508 - 1.0) * (var_is_t_dn0 / var_ik_t))) } } else { (assign6320_e6509 * (assign6320_e6508 * ((var_is_t_dn0 / var_ik_t) / assign6320_e6505))) });
        var_qe0_dn1 = (assign6320_e6502 * if 0.0 == 0.0 && ((assign6320_e6508) as f64).is_finite() && ((assign6320_e6508) as f64).fract() == 0.0 { if assign6320_e6508 == 0.0 { 0.0 } else { (assign6320_e6508 * ((assign6320_e6505).powf(assign6320_e6508 - 1.0) * (var_is_t_dn1 / var_ik_t))) } } else { (assign6320_e6509 * (assign6320_e6508 * ((var_is_t_dn1 / var_ik_t) / assign6320_e6505))) });
        var_qe0_dn2 = (assign6320_e6502 * if 0.0 == 0.0 && ((assign6320_e6508) as f64).is_finite() && ((assign6320_e6508) as f64).fract() == 0.0 { if assign6320_e6508 == 0.0 { 0.0 } else { (assign6320_e6508 * ((assign6320_e6505).powf(assign6320_e6508 - 1.0) * (var_is_t_dn2 / var_ik_t))) } } else { (assign6320_e6509 * (assign6320_e6508 * ((var_is_t_dn2 / var_ik_t) / assign6320_e6505))) });
        var_qe0_dn3 = (assign6320_e6502 * if 0.0 == 0.0 && ((assign6320_e6508) as f64).is_finite() && ((assign6320_e6508) as f64).fract() == 0.0 { if assign6320_e6508 == 0.0 { 0.0 } else { (assign6320_e6508 * ((assign6320_e6505).powf(assign6320_e6508 - 1.0) * (var_is_t_dn3 / var_ik_t))) } } else { (assign6320_e6509 * (assign6320_e6508 * ((var_is_t_dn3 / var_ik_t) / assign6320_e6505))) });
        var_qe0_dn4 = (assign6320_e6502 * if 0.0 == 0.0 && ((assign6320_e6508) as f64).is_finite() && ((assign6320_e6508) as f64).fract() == 0.0 { if assign6320_e6508 == 0.0 { 0.0 } else { (assign6320_e6508 * ((assign6320_e6505).powf(assign6320_e6508 - 1.0) * (var_is_t_dn4 / var_ik_t))) } } else { (assign6320_e6509 * (assign6320_e6508 * ((var_is_t_dn4 / var_ik_t) / assign6320_e6505))) });
        var_qe0_dn5 = (assign6320_e6502 * if 0.0 == 0.0 && ((assign6320_e6508) as f64).is_finite() && ((assign6320_e6508) as f64).fract() == 0.0 { if assign6320_e6508 == 0.0 { 0.0 } else { (assign6320_e6508 * ((assign6320_e6505).powf(assign6320_e6508 - 1.0) * (var_is_t_dn5 / var_ik_t))) } } else { (assign6320_e6509 * (assign6320_e6508 * ((var_is_t_dn5 / var_ik_t) / assign6320_e6505))) });
        var_qe0_dn6 = (assign6320_e6502 * if 0.0 == 0.0 && ((assign6320_e6508) as f64).is_finite() && ((assign6320_e6508) as f64).fract() == 0.0 { if assign6320_e6508 == 0.0 { 0.0 } else { (assign6320_e6508 * ((assign6320_e6505).powf(assign6320_e6508 - 1.0) * (var_is_t_dn6 / var_ik_t))) } } else { (assign6320_e6509 * (assign6320_e6508 * ((var_is_t_dn6 / var_ik_t) / assign6320_e6505))) });
        var_qe0_dn7 = (assign6320_e6502 * if 0.0 == 0.0 && ((assign6320_e6508) as f64).is_finite() && ((assign6320_e6508) as f64).fract() == 0.0 { if assign6320_e6508 == 0.0 { 0.0 } else { (assign6320_e6508 * ((assign6320_e6505).powf(assign6320_e6508 - 1.0) * (var_is_t_dn7 / var_ik_t))) } } else { (assign6320_e6509 * (assign6320_e6508 * ((var_is_t_dn7 / var_ik_t) / assign6320_e6505))) });
        var_qe0_dn8 = (assign6320_e6502 * if 0.0 == 0.0 && ((assign6320_e6508) as f64).is_finite() && ((assign6320_e6508) as f64).fract() == 0.0 { if assign6320_e6508 == 0.0 { 0.0 } else { (assign6320_e6508 * ((assign6320_e6505).powf(assign6320_e6508 - 1.0) * (var_is_t_dn8 / var_ik_t))) } } else { (assign6320_e6509 * (assign6320_e6508 * ((var_is_t_dn8 / var_ik_t) / assign6320_e6505))) });
        var_qe0_dn9 = (assign6320_e6502 * if 0.0 == 0.0 && ((assign6320_e6508) as f64).is_finite() && ((assign6320_e6508) as f64).fract() == 0.0 { if assign6320_e6508 == 0.0 { 0.0 } else { (assign6320_e6508 * ((assign6320_e6505).powf(assign6320_e6508 - 1.0) * (var_is_t_dn9 / var_ik_t))) } } else { (assign6320_e6509 * (assign6320_e6508 * ((var_is_t_dn9 / var_ik_t) / assign6320_e6505))) });
        var_qe0_dn10 = (assign6320_e6502 * if 0.0 == 0.0 && ((assign6320_e6508) as f64).is_finite() && ((assign6320_e6508) as f64).fract() == 0.0 { if assign6320_e6508 == 0.0 { 0.0 } else { (assign6320_e6508 * ((assign6320_e6505).powf(assign6320_e6508 - 1.0) * (var_is_t_dn10 / var_ik_t))) } } else { (assign6320_e6509 * (assign6320_e6508 * ((var_is_t_dn10 / var_ik_t) / assign6320_e6505))) });
        var_qe0_dn11 = (assign6320_e6502 * if 0.0 == 0.0 && ((assign6320_e6508) as f64).is_finite() && ((assign6320_e6508) as f64).fract() == 0.0 { if assign6320_e6508 == 0.0 { 0.0 } else { (assign6320_e6508 * ((assign6320_e6505).powf(assign6320_e6508 - 1.0) * (var_is_t_dn11 / var_ik_t))) } } else { (assign6320_e6509 * (assign6320_e6508 * ((var_is_t_dn11 / var_ik_t) / assign6320_e6505))) });
        var_qe0_db0 = (assign6320_e6502 * if 0.0 == 0.0 && ((assign6320_e6508) as f64).is_finite() && ((assign6320_e6508) as f64).fract() == 0.0 { if assign6320_e6508 == 0.0 { 0.0 } else { (assign6320_e6508 * ((assign6320_e6505).powf(assign6320_e6508 - 1.0) * (var_is_t_db0 / var_ik_t))) } } else { (assign6320_e6509 * (assign6320_e6508 * ((var_is_t_db0 / var_ik_t) / assign6320_e6505))) });
        var_qe0_db1 = (assign6320_e6502 * if 0.0 == 0.0 && ((assign6320_e6508) as f64).is_finite() && ((assign6320_e6508) as f64).fract() == 0.0 { if assign6320_e6508 == 0.0 { 0.0 } else { (assign6320_e6508 * ((assign6320_e6505).powf(assign6320_e6508 - 1.0) * (var_is_t_db1 / var_ik_t))) } } else { (assign6320_e6509 * (assign6320_e6508 * ((var_is_t_db1 / var_ik_t) / assign6320_e6505))) });

        let assign6330_e6514: f64 = (p.p85 * var_vt);
        let assign6330_e6515: f64 = (var_vb2e1 / assign6330_e6514);
        let assign6330_e6517: f64 = if assign6330_e6515 < p.p147 { 1.0 } else { 0.0 };
        var_guard114 = assign6330_e6517;

        let (assign6340_e6526, assign6340_e6526_d_n0, assign6340_e6526_d_n1, assign6340_e6526_d_n2, assign6340_e6526_d_n3, assign6340_e6526_d_n4, assign6340_e6526_d_n5, assign6340_e6526_d_n6, assign6340_e6526_d_n7, assign6340_e6526_d_n8, assign6340_e6526_d_n9, assign6340_e6526_d_n10, assign6340_e6526_d_n11, assign6340_e6526_d_b0, assign6340_e6526_d_b1,) = {
    if (var_guard114 != 0.0) {
        let assign6340_e6522: f64 = (p.p85 * var_vt);
        let assign6340_e6523: f64 = (var_vb2e1 / assign6340_e6522);
        let assign6340_e6524: f64 = (assign6340_e6523).exp();
        (assign6340_e6524, (assign6340_e6524 * (var_vb2e1_dn0 / assign6340_e6522)), (assign6340_e6524 * (var_vb2e1_dn1 / assign6340_e6522)), (assign6340_e6524 * (var_vb2e1_dn2 / assign6340_e6522)), (assign6340_e6524 * (var_vb2e1_dn3 / assign6340_e6522)), (assign6340_e6524 * (var_vb2e1_dn4 / assign6340_e6522)), (assign6340_e6524 * (var_vb2e1_dn5 / assign6340_e6522)), (assign6340_e6524 * (var_vb2e1_dn6 / assign6340_e6522)), (assign6340_e6524 * (var_vb2e1_dn7 / assign6340_e6522)), (assign6340_e6524 * (var_vb2e1_dn8 / assign6340_e6522)), (assign6340_e6524 * (var_vb2e1_dn9 / assign6340_e6522)), (assign6340_e6524 * (var_vb2e1_dn10 / assign6340_e6522)), (assign6340_e6524 * (var_vb2e1_dn11 / assign6340_e6522)), (assign6340_e6524 * (var_vb2e1_db0 / assign6340_e6522)), (assign6340_e6524 * (var_vb2e1_db1 / assign6340_e6522)),)
    } else {
        (var_tmpexp, var_tmpexp_dn0, var_tmpexp_dn1, var_tmpexp_dn2, var_tmpexp_dn3, var_tmpexp_dn4, var_tmpexp_dn5, var_tmpexp_dn6, var_tmpexp_dn7, var_tmpexp_dn8, var_tmpexp_dn9, var_tmpexp_dn10, var_tmpexp_dn11, var_tmpexp_db0, var_tmpexp_db1,)
    }
};
        var_tmpexp = assign6340_e6526;
        var_tmpexp_dn0 = assign6340_e6526_d_n0;
        var_tmpexp_dn1 = assign6340_e6526_d_n1;
        var_tmpexp_dn2 = assign6340_e6526_d_n2;
        var_tmpexp_dn3 = assign6340_e6526_d_n3;
        var_tmpexp_dn4 = assign6340_e6526_d_n4;
        var_tmpexp_dn5 = assign6340_e6526_d_n5;
        var_tmpexp_dn6 = assign6340_e6526_d_n6;
        var_tmpexp_dn7 = assign6340_e6526_d_n7;
        var_tmpexp_dn8 = assign6340_e6526_d_n8;
        var_tmpexp_dn9 = assign6340_e6526_d_n9;
        var_tmpexp_dn10 = assign6340_e6526_d_n10;
        var_tmpexp_dn11 = assign6340_e6526_d_n11;
        var_tmpexp_db0 = assign6340_e6526_d_b0;
        var_tmpexp_db1 = assign6340_e6526_d_b1;

        let (assign6350_e6532,) = {
    if (var_guard114 == 0.0) {
        let assign6350_e6530: f64 = (p.p147).exp();
        (assign6350_e6530,)
    } else {
        (var_expl,)
    }
};
        var_expl = assign6350_e6532;

        let (assign6360_e6547, assign6360_e6547_d_n0, assign6360_e6547_d_n1, assign6360_e6547_d_n2, assign6360_e6547_d_n3, assign6360_e6547_d_n4, assign6360_e6547_d_n5, assign6360_e6547_d_n6, assign6360_e6547_d_n7, assign6360_e6547_d_n8, assign6360_e6547_d_n9, assign6360_e6547_d_n10, assign6360_e6547_d_n11, assign6360_e6547_d_b0, assign6360_e6547_d_b1,) = {
    if (var_guard114 == 0.0) {
        let assign6360_e6540: f64 = (p.p85 * var_vt);
        let assign6360_e6541: f64 = (var_vb2e1 / assign6360_e6540);
        let assign6360_e6543: f64 = (assign6360_e6541 - p.p147);
        let assign6360_e6544: f64 = (1.0 + assign6360_e6543);
        let assign6360_e6545: f64 = (var_expl * assign6360_e6544);
        (assign6360_e6545, (var_expl * (var_vb2e1_dn0 / assign6360_e6540)), (var_expl * (var_vb2e1_dn1 / assign6360_e6540)), (var_expl * (var_vb2e1_dn2 / assign6360_e6540)), (var_expl * (var_vb2e1_dn3 / assign6360_e6540)), (var_expl * (var_vb2e1_dn4 / assign6360_e6540)), (var_expl * (var_vb2e1_dn5 / assign6360_e6540)), (var_expl * (var_vb2e1_dn6 / assign6360_e6540)), (var_expl * (var_vb2e1_dn7 / assign6360_e6540)), (var_expl * (var_vb2e1_dn8 / assign6360_e6540)), (var_expl * (var_vb2e1_dn9 / assign6360_e6540)), (var_expl * (var_vb2e1_dn10 / assign6360_e6540)), (var_expl * (var_vb2e1_dn11 / assign6360_e6540)), (var_expl * (var_vb2e1_db0 / assign6360_e6540)), (var_expl * (var_vb2e1_db1 / assign6360_e6540)),)
    } else {
        (var_tmpexp, var_tmpexp_dn0, var_tmpexp_dn1, var_tmpexp_dn2, var_tmpexp_dn3, var_tmpexp_dn4, var_tmpexp_dn5, var_tmpexp_dn6, var_tmpexp_dn7, var_tmpexp_dn8, var_tmpexp_dn9, var_tmpexp_dn10, var_tmpexp_dn11, var_tmpexp_db0, var_tmpexp_db1,)
    }
};
        var_tmpexp = assign6360_e6547;
        var_tmpexp_dn0 = assign6360_e6547_d_n0;
        var_tmpexp_dn1 = assign6360_e6547_d_n1;
        var_tmpexp_dn2 = assign6360_e6547_d_n2;
        var_tmpexp_dn3 = assign6360_e6547_d_n3;
        var_tmpexp_dn4 = assign6360_e6547_d_n4;
        var_tmpexp_dn5 = assign6360_e6547_d_n5;
        var_tmpexp_dn6 = assign6360_e6547_d_n6;
        var_tmpexp_dn7 = assign6360_e6547_d_n7;
        var_tmpexp_dn8 = assign6360_e6547_d_n8;
        var_tmpexp_dn9 = assign6360_e6547_d_n9;
        var_tmpexp_dn10 = assign6360_e6547_d_n10;
        var_tmpexp_dn11 = assign6360_e6547_d_n11;
        var_tmpexp_db0 = assign6360_e6547_d_b0;
        var_tmpexp_db1 = assign6360_e6547_d_b1;

        let assign6370_e6550: f64 = (var_qe0 * var_tmpexp);
        var_qe_qs = assign6370_e6550;
        var_qe_qs_dn0 = ((var_qe0_dn0 * var_tmpexp) + (var_qe0 * var_tmpexp_dn0));
        var_qe_qs_dn1 = ((var_qe0_dn1 * var_tmpexp) + (var_qe0 * var_tmpexp_dn1));
        var_qe_qs_dn2 = ((var_qe0_dn2 * var_tmpexp) + (var_qe0 * var_tmpexp_dn2));
        var_qe_qs_dn3 = ((var_qe0_dn3 * var_tmpexp) + (var_qe0 * var_tmpexp_dn3));
        var_qe_qs_dn4 = ((var_qe0_dn4 * var_tmpexp) + (var_qe0 * var_tmpexp_dn4));
        var_qe_qs_dn5 = ((var_qe0_dn5 * var_tmpexp) + (var_qe0 * var_tmpexp_dn5));
        var_qe_qs_dn6 = ((var_qe0_dn6 * var_tmpexp) + (var_qe0 * var_tmpexp_dn6));
        var_qe_qs_dn7 = ((var_qe0_dn7 * var_tmpexp) + (var_qe0 * var_tmpexp_dn7));
        var_qe_qs_dn8 = ((var_qe0_dn8 * var_tmpexp) + (var_qe0 * var_tmpexp_dn8));
        var_qe_qs_dn9 = ((var_qe0_dn9 * var_tmpexp) + (var_qe0 * var_tmpexp_dn9));
        var_qe_qs_dn10 = ((var_qe0_dn10 * var_tmpexp) + (var_qe0 * var_tmpexp_dn10));
        var_qe_qs_dn11 = ((var_qe0_dn11 * var_tmpexp) + (var_qe0 * var_tmpexp_dn11));
        var_qe_qs_db0 = ((var_qe0_db0 * var_tmpexp) + (var_qe0 * var_tmpexp_db0));
        var_qe_qs_db1 = ((var_qe0_db1 * var_tmpexp) + (var_qe0 * var_tmpexp_db1));

        let assign6380_e6553: f64 = (4.0 * var_tepi_t);
        let assign6380_e6555: f64 = (assign6380_e6553 * var_vt);
        let assign6380_e6557: f64 = (assign6380_e6555 / var_rcv_t);
        var_qepi0 = assign6380_e6557;

        let assign6390_e6560: f64 = (0.5 * var_qepi0);
        let assign6390_e6562: f64 = (assign6390_e6560 * var_xi_w);
        let assign6390_e6565: f64 = (var_p0star + var_pw);
        let assign6390_e6567: f64 = (assign6390_e6565 + 2.0);
        let assign6390_e6568: f64 = (assign6390_e6562 * assign6390_e6567);
        var_qepi = assign6390_e6568;
        var_qepi_dn0 = (((assign6390_e6560 * var_xi_w_dn0) * assign6390_e6567) + (assign6390_e6562 * (var_p0star_dn0 + var_pw_dn0)));
        var_qepi_dn1 = (((assign6390_e6560 * var_xi_w_dn1) * assign6390_e6567) + (assign6390_e6562 * (var_p0star_dn1 + var_pw_dn1)));
        var_qepi_dn2 = (((assign6390_e6560 * var_xi_w_dn2) * assign6390_e6567) + (assign6390_e6562 * (var_p0star_dn2 + var_pw_dn2)));
        var_qepi_dn3 = (((assign6390_e6560 * var_xi_w_dn3) * assign6390_e6567) + (assign6390_e6562 * (var_p0star_dn3 + var_pw_dn3)));
        var_qepi_dn4 = (((assign6390_e6560 * var_xi_w_dn4) * assign6390_e6567) + (assign6390_e6562 * (var_p0star_dn4 + var_pw_dn4)));
        var_qepi_dn5 = (((assign6390_e6560 * var_xi_w_dn5) * assign6390_e6567) + (assign6390_e6562 * (var_p0star_dn5 + var_pw_dn5)));
        var_qepi_dn6 = (((assign6390_e6560 * var_xi_w_dn6) * assign6390_e6567) + (assign6390_e6562 * (var_p0star_dn6 + var_pw_dn6)));
        var_qepi_dn7 = (((assign6390_e6560 * var_xi_w_dn7) * assign6390_e6567) + (assign6390_e6562 * (var_p0star_dn7 + var_pw_dn7)));
        var_qepi_dn8 = (((assign6390_e6560 * var_xi_w_dn8) * assign6390_e6567) + (assign6390_e6562 * (var_p0star_dn8 + var_pw_dn8)));
        var_qepi_dn9 = (((assign6390_e6560 * var_xi_w_dn9) * assign6390_e6567) + (assign6390_e6562 * (var_p0star_dn9 + var_pw_dn9)));
        var_qepi_dn10 = (((assign6390_e6560 * var_xi_w_dn10) * assign6390_e6567) + (assign6390_e6562 * (var_p0star_dn10 + var_pw_dn10)));
        var_qepi_dn11 = (((assign6390_e6560 * var_xi_w_dn11) * assign6390_e6567) + (assign6390_e6562 * (var_p0star_dn11 + var_pw_dn11)));
        var_qepi_db0 = (((assign6390_e6560 * var_xi_w_db0) * assign6390_e6567) + (assign6390_e6562 * (var_p0star_db0 + var_pw_db0)));
        var_qepi_db1 = (((assign6390_e6560 * var_xi_w_db1) * assign6390_e6567) + (assign6390_e6562 * (var_p0star_db1 + var_pw_db1)));

        let assign6400_e6571: f64 = if p.p79 == 0.0 { 1.0 } else { 0.0 };
        var_guard115 = assign6400_e6571;

        let (assign6410_e6589, assign6410_e6589_d_n0, assign6410_e6589_d_n1, assign6410_e6589_d_n2, assign6410_e6589_d_n3, assign6410_e6589_d_n4, assign6410_e6589_d_n5, assign6410_e6589_d_n6, assign6410_e6589_d_n7, assign6410_e6589_d_n8, assign6410_e6589_d_n9, assign6410_e6589_d_n10, assign6410_e6589_d_n11, assign6410_e6589_d_b0, assign6410_e6589_d_b1,) = {
    if (var_guard115 != 0.0) {
        let assign6410_e6575: f64 = (var_taur_t * 0.5);
        let assign6410_e6578: f64 = (var_qb0 * var_nbex);
        let assign6410_e6581: f64 = (var_qepi0 * var_pwex);
        let assign6410_e6582: f64 = (assign6410_e6578 + assign6410_e6581);
        let assign6410_e6583: f64 = (assign6410_e6575 * assign6410_e6582);
        let assign6410_e6586: f64 = (var_taub_t + var_tepi_t);
        let assign6410_e6587: f64 = (assign6410_e6583 / assign6410_e6586);
        (assign6410_e6587, ((assign6410_e6575 * ((var_qb0 * var_nbex_dn0) + (var_qepi0 * var_pwex_dn0))) / assign6410_e6586), ((assign6410_e6575 * ((var_qb0 * var_nbex_dn1) + (var_qepi0 * var_pwex_dn1))) / assign6410_e6586), ((assign6410_e6575 * ((var_qb0 * var_nbex_dn2) + (var_qepi0 * var_pwex_dn2))) / assign6410_e6586), ((assign6410_e6575 * ((var_qb0 * var_nbex_dn3) + (var_qepi0 * var_pwex_dn3))) / assign6410_e6586), ((assign6410_e6575 * ((var_qb0 * var_nbex_dn4) + (var_qepi0 * var_pwex_dn4))) / assign6410_e6586), ((assign6410_e6575 * ((var_qb0 * var_nbex_dn5) + (var_qepi0 * var_pwex_dn5))) / assign6410_e6586), ((assign6410_e6575 * ((var_qb0 * var_nbex_dn6) + (var_qepi0 * var_pwex_dn6))) / assign6410_e6586), ((assign6410_e6575 * ((var_qb0 * var_nbex_dn7) + (var_qepi0 * var_pwex_dn7))) / assign6410_e6586), ((assign6410_e6575 * ((var_qb0 * var_nbex_dn8) + (var_qepi0 * var_pwex_dn8))) / assign6410_e6586), ((assign6410_e6575 * ((var_qb0 * var_nbex_dn9) + (var_qepi0 * var_pwex_dn9))) / assign6410_e6586), ((assign6410_e6575 * ((var_qb0 * var_nbex_dn10) + (var_qepi0 * var_pwex_dn10))) / assign6410_e6586), ((assign6410_e6575 * ((var_qb0 * var_nbex_dn11) + (var_qepi0 * var_pwex_dn11))) / assign6410_e6586), ((assign6410_e6575 * ((var_qb0 * var_nbex_db0) + (var_qepi0 * var_pwex_db0))) / assign6410_e6586), ((assign6410_e6575 * ((var_qb0 * var_nbex_db1) + (var_qepi0 * var_pwex_db1))) / assign6410_e6586),)
    } else {
        (var_qex, var_qex_dn0, var_qex_dn1, var_qex_dn2, var_qex_dn3, var_qex_dn4, var_qex_dn5, var_qex_dn6, var_qex_dn7, var_qex_dn8, var_qex_dn9, var_qex_dn10, var_qex_dn11, var_qex_db0, var_qex_db1,)
    }
};
        var_qex = assign6410_e6589;
        var_qex_dn0 = assign6410_e6589_d_n0;
        var_qex_dn1 = assign6410_e6589_d_n1;
        var_qex_dn2 = assign6410_e6589_d_n2;
        var_qex_dn3 = assign6410_e6589_d_n3;
        var_qex_dn4 = assign6410_e6589_d_n4;
        var_qex_dn5 = assign6410_e6589_d_n5;
        var_qex_dn6 = assign6410_e6589_d_n6;
        var_qex_dn7 = assign6410_e6589_d_n7;
        var_qex_dn8 = assign6410_e6589_d_n8;
        var_qex_dn9 = assign6410_e6589_d_n9;
        var_qex_dn10 = assign6410_e6589_d_n10;
        var_qex_dn11 = assign6410_e6589_d_n11;
        var_qex_db0 = assign6410_e6589_d_b0;
        var_qex_db1 = assign6410_e6589_d_b1;

        let assign6420_e6592: f64 = (var_vb1c4 - var_vdcex_t);
        let assign6420_e6594: f64 = (assign6420_e6592 / p.p91);
        let assign6420_e6596: f64 = (assign6420_e6594 * var_vtinv);
        let assign6420_e6598: f64 = if assign6420_e6596 < p.p147 { 1.0 } else { 0.0 };
        var_guard116 = assign6420_e6598;

        let (assign6430_e6612, assign6430_e6612_d_n0, assign6430_e6612_d_n1, assign6430_e6612_d_n2, assign6430_e6612_d_n3, assign6430_e6612_d_n4, assign6430_e6612_d_n5, assign6430_e6612_d_n6, assign6430_e6612_d_n7, assign6430_e6612_d_n8, assign6430_e6612_d_n9, assign6430_e6612_d_n10, assign6430_e6612_d_n11, assign6430_e6612_d_b0, assign6430_e6612_d_b1,) = {
    if ((var_guard115 == 0.0) && (var_guard116 != 0.0)) {
        let assign6430_e6605: f64 = (var_vb1c4 - var_vdcex_t);
        let assign6430_e6607: f64 = (assign6430_e6605 / p.p91);
        let assign6430_e6609: f64 = (assign6430_e6607 * var_vtinv);
        let assign6430_e6610: f64 = (assign6430_e6609).exp();
        (assign6430_e6610, (assign6430_e6610 * (((var_vb1c4_dn0 - var_vdcex_t_dn0) / p.p91) * var_vtinv)), (assign6430_e6610 * (((var_vb1c4_dn1 - var_vdcex_t_dn1) / p.p91) * var_vtinv)), (assign6430_e6610 * (((var_vb1c4_dn2 - var_vdcex_t_dn2) / p.p91) * var_vtinv)), (assign6430_e6610 * (((var_vb1c4_dn3 - var_vdcex_t_dn3) / p.p91) * var_vtinv)), (assign6430_e6610 * (((var_vb1c4_dn4 - var_vdcex_t_dn4) / p.p91) * var_vtinv)), (assign6430_e6610 * (((var_vb1c4_dn5 - var_vdcex_t_dn5) / p.p91) * var_vtinv)), (assign6430_e6610 * (((var_vb1c4_dn6 - var_vdcex_t_dn6) / p.p91) * var_vtinv)), (assign6430_e6610 * (((var_vb1c4_dn7 - var_vdcex_t_dn7) / p.p91) * var_vtinv)), (assign6430_e6610 * (((var_vb1c4_dn8 - var_vdcex_t_dn8) / p.p91) * var_vtinv)), (assign6430_e6610 * (((var_vb1c4_dn9 - var_vdcex_t_dn9) / p.p91) * var_vtinv)), (assign6430_e6610 * (((var_vb1c4_dn10 - var_vdcex_t_dn10) / p.p91) * var_vtinv)), (assign6430_e6610 * (((var_vb1c4_dn11 - var_vdcex_t_dn11) / p.p91) * var_vtinv)), (assign6430_e6610 * (((var_vb1c4_db0 - var_vdcex_t_db0) / p.p91) * var_vtinv)), (assign6430_e6610 * (((var_vb1c4_db1 - var_vdcex_t_db1) / p.p91) * var_vtinv)),)
    } else {
        (var_evb1c4vdcex, var_evb1c4vdcex_dn0, var_evb1c4vdcex_dn1, var_evb1c4vdcex_dn2, var_evb1c4vdcex_dn3, var_evb1c4vdcex_dn4, var_evb1c4vdcex_dn5, var_evb1c4vdcex_dn6, var_evb1c4vdcex_dn7, var_evb1c4vdcex_dn8, var_evb1c4vdcex_dn9, var_evb1c4vdcex_dn10, var_evb1c4vdcex_dn11, var_evb1c4vdcex_db0, var_evb1c4vdcex_db1,)
    }
};
        var_evb1c4vdcex = assign6430_e6612;
        var_evb1c4vdcex_dn0 = assign6430_e6612_d_n0;
        var_evb1c4vdcex_dn1 = assign6430_e6612_d_n1;
        var_evb1c4vdcex_dn2 = assign6430_e6612_d_n2;
        var_evb1c4vdcex_dn3 = assign6430_e6612_d_n3;
        var_evb1c4vdcex_dn4 = assign6430_e6612_d_n4;
        var_evb1c4vdcex_dn5 = assign6430_e6612_d_n5;
        var_evb1c4vdcex_dn6 = assign6430_e6612_d_n6;
        var_evb1c4vdcex_dn7 = assign6430_e6612_d_n7;
        var_evb1c4vdcex_dn8 = assign6430_e6612_d_n8;
        var_evb1c4vdcex_dn9 = assign6430_e6612_d_n9;
        var_evb1c4vdcex_dn10 = assign6430_e6612_d_n10;
        var_evb1c4vdcex_dn11 = assign6430_e6612_d_n11;
        var_evb1c4vdcex_db0 = assign6430_e6612_d_b0;
        var_evb1c4vdcex_db1 = assign6430_e6612_d_b1;

        let (assign6440_e6621,) = {
    if ((var_guard115 == 0.0) && (var_guard116 == 0.0)) {
        let assign6440_e6619: f64 = (p.p147).exp();
        (assign6440_e6619,)
    } else {
        (var_expl,)
    }
};
        var_expl = assign6440_e6621;


        *var_a_vds_slot = var_a_vds;
        *var_a_vds_db0_slot = var_a_vds_db0;
        *var_a_vds_db1_slot = var_a_vds_db1;
        *var_a_vds_dn0_slot = var_a_vds_dn0;
        *var_a_vds_dn1_slot = var_a_vds_dn1;
        *var_a_vds_dn10_slot = var_a_vds_dn10;
        *var_a_vds_dn11_slot = var_a_vds_dn11;
        *var_a_vds_dn2_slot = var_a_vds_dn2;
        *var_a_vds_dn3_slot = var_a_vds_dn3;
        *var_a_vds_dn4_slot = var_a_vds_dn4;
        *var_a_vds_dn5_slot = var_a_vds_dn5;
        *var_a_vds_dn6_slot = var_a_vds_dn6;
        *var_a_vds_dn7_slot = var_a_vds_dn7;
        *var_a_vds_dn8_slot = var_a_vds_dn8;
        *var_a_vds_dn9_slot = var_a_vds_dn9;
        *var_dxa_slot = var_dxa;
        *var_dxa_db0_slot = var_dxa_db0;
        *var_dxa_db1_slot = var_dxa_db1;
        *var_dxa_dn0_slot = var_dxa_dn0;
        *var_dxa_dn1_slot = var_dxa_dn1;
        *var_dxa_dn10_slot = var_dxa_dn10;
        *var_dxa_dn11_slot = var_dxa_dn11;
        *var_dxa_dn2_slot = var_dxa_dn2;
        *var_dxa_dn3_slot = var_dxa_dn3;
        *var_dxa_dn4_slot = var_dxa_dn4;
        *var_dxa_dn5_slot = var_dxa_dn5;
        *var_dxa_dn6_slot = var_dxa_dn6;
        *var_dxa_dn7_slot = var_dxa_dn7;
        *var_dxa_dn8_slot = var_dxa_dn8;
        *var_dxa_dn9_slot = var_dxa_dn9;
        *var_evb1c4vdcex_slot = var_evb1c4vdcex;
        *var_evb1c4vdcex_db0_slot = var_evb1c4vdcex_db0;
        *var_evb1c4vdcex_db1_slot = var_evb1c4vdcex_db1;
        *var_evb1c4vdcex_dn0_slot = var_evb1c4vdcex_dn0;
        *var_evb1c4vdcex_dn1_slot = var_evb1c4vdcex_dn1;
        *var_evb1c4vdcex_dn10_slot = var_evb1c4vdcex_dn10;
        *var_evb1c4vdcex_dn11_slot = var_evb1c4vdcex_dn11;
        *var_evb1c4vdcex_dn2_slot = var_evb1c4vdcex_dn2;
        *var_evb1c4vdcex_dn3_slot = var_evb1c4vdcex_dn3;
        *var_evb1c4vdcex_dn4_slot = var_evb1c4vdcex_dn4;
        *var_evb1c4vdcex_dn5_slot = var_evb1c4vdcex_dn5;
        *var_evb1c4vdcex_dn6_slot = var_evb1c4vdcex_dn6;
        *var_evb1c4vdcex_dn7_slot = var_evb1c4vdcex_dn7;
        *var_evb1c4vdcex_dn8_slot = var_evb1c4vdcex_dn8;
        *var_evb1c4vdcex_dn9_slot = var_evb1c4vdcex_dn9;
        *var_expl_slot = var_expl;
        *var_guard112_slot = var_guard112;
        *var_guard113_slot = var_guard113;
        *var_guard114_slot = var_guard114;
        *var_guard115_slot = var_guard115;
        *var_guard116_slot = var_guard116;
        *var_qe0_slot = var_qe0;
        *var_qe0_db0_slot = var_qe0_db0;
        *var_qe0_db1_slot = var_qe0_db1;
        *var_qe0_dn0_slot = var_qe0_dn0;
        *var_qe0_dn1_slot = var_qe0_dn1;
        *var_qe0_dn10_slot = var_qe0_dn10;
        *var_qe0_dn11_slot = var_qe0_dn11;
        *var_qe0_dn2_slot = var_qe0_dn2;
        *var_qe0_dn3_slot = var_qe0_dn3;
        *var_qe0_dn4_slot = var_qe0_dn4;
        *var_qe0_dn5_slot = var_qe0_dn5;
        *var_qe0_dn6_slot = var_qe0_dn6;
        *var_qe0_dn7_slot = var_qe0_dn7;
        *var_qe0_dn8_slot = var_qe0_dn8;
        *var_qe0_dn9_slot = var_qe0_dn9;
        *var_qe_qs_slot = var_qe_qs;
        *var_qe_qs_db0_slot = var_qe_qs_db0;
        *var_qe_qs_db1_slot = var_qe_qs_db1;
        *var_qe_qs_dn0_slot = var_qe_qs_dn0;
        *var_qe_qs_dn1_slot = var_qe_qs_dn1;
        *var_qe_qs_dn10_slot = var_qe_qs_dn10;
        *var_qe_qs_dn11_slot = var_qe_qs_dn11;
        *var_qe_qs_dn2_slot = var_qe_qs_dn2;
        *var_qe_qs_dn3_slot = var_qe_qs_dn3;
        *var_qe_qs_dn4_slot = var_qe_qs_dn4;
        *var_qe_qs_dn5_slot = var_qe_qs_dn5;
        *var_qe_qs_dn6_slot = var_qe_qs_dn6;
        *var_qe_qs_dn7_slot = var_qe_qs_dn7;
        *var_qe_qs_dn8_slot = var_qe_qs_dn8;
        *var_qe_qs_dn9_slot = var_qe_qs_dn9;
        *var_qepi_slot = var_qepi;
        *var_qepi0_slot = var_qepi0;
        *var_qepi_db0_slot = var_qepi_db0;
        *var_qepi_db1_slot = var_qepi_db1;
        *var_qepi_dn0_slot = var_qepi_dn0;
        *var_qepi_dn1_slot = var_qepi_dn1;
        *var_qepi_dn10_slot = var_qepi_dn10;
        *var_qepi_dn11_slot = var_qepi_dn11;
        *var_qepi_dn2_slot = var_qepi_dn2;
        *var_qepi_dn3_slot = var_qepi_dn3;
        *var_qepi_dn4_slot = var_qepi_dn4;
        *var_qepi_dn5_slot = var_qepi_dn5;
        *var_qepi_dn6_slot = var_qepi_dn6;
        *var_qepi_dn7_slot = var_qepi_dn7;
        *var_qepi_dn8_slot = var_qepi_dn8;
        *var_qepi_dn9_slot = var_qepi_dn9;
        *var_qex_slot = var_qex;
        *var_qex_db0_slot = var_qex_db0;
        *var_qex_db1_slot = var_qex_db1;
        *var_qex_dn0_slot = var_qex_dn0;
        *var_qex_dn1_slot = var_qex_dn1;
        *var_qex_dn10_slot = var_qex_dn10;
        *var_qex_dn11_slot = var_qex_dn11;
        *var_qex_dn2_slot = var_qex_dn2;
        *var_qex_dn3_slot = var_qex_dn3;
        *var_qex_dn4_slot = var_qex_dn4;
        *var_qex_dn5_slot = var_qex_dn5;
        *var_qex_dn6_slot = var_qex_dn6;
        *var_qex_dn7_slot = var_qex_dn7;
        *var_qex_dn8_slot = var_qex_dn8;
        *var_qex_dn9_slot = var_qex_dn9;
        *var_qtex_slot = var_qtex;
        *var_qtex_db0_slot = var_qtex_db0;
        *var_qtex_db1_slot = var_qtex_db1;
        *var_qtex_dn0_slot = var_qtex_dn0;
        *var_qtex_dn1_slot = var_qtex_dn1;
        *var_qtex_dn10_slot = var_qtex_dn10;
        *var_qtex_dn11_slot = var_qtex_dn11;
        *var_qtex_dn2_slot = var_qtex_dn2;
        *var_qtex_dn3_slot = var_qtex_dn3;
        *var_qtex_dn4_slot = var_qtex_dn4;
        *var_qtex_dn5_slot = var_qtex_dn5;
        *var_qtex_dn6_slot = var_qtex_dn6;
        *var_qtex_dn7_slot = var_qtex_dn7;
        *var_qtex_dn8_slot = var_qtex_dn8;
        *var_qtex_dn9_slot = var_qtex_dn9;
        *var_qts_slot = var_qts;
        *var_qts_db0_slot = var_qts_db0;
        *var_qts_db1_slot = var_qts_db1;
        *var_qts_dn0_slot = var_qts_dn0;
        *var_qts_dn1_slot = var_qts_dn1;
        *var_qts_dn10_slot = var_qts_dn10;
        *var_qts_dn11_slot = var_qts_dn11;
        *var_qts_dn2_slot = var_qts_dn2;
        *var_qts_dn3_slot = var_qts_dn3;
        *var_qts_dn4_slot = var_qts_dn4;
        *var_qts_dn5_slot = var_qts_dn5;
        *var_qts_dn6_slot = var_qts_dn6;
        *var_qts_dn7_slot = var_qts_dn7;
        *var_qts_dn8_slot = var_qts_dn8;
        *var_qts_dn9_slot = var_qts_dn9;
        *var_tmpexp_slot = var_tmpexp;
        *var_tmpexp_db0_slot = var_tmpexp_db0;
        *var_tmpexp_db1_slot = var_tmpexp_db1;
        *var_tmpexp_dn0_slot = var_tmpexp_dn0;
        *var_tmpexp_dn1_slot = var_tmpexp_dn1;
        *var_tmpexp_dn10_slot = var_tmpexp_dn10;
        *var_tmpexp_dn11_slot = var_tmpexp_dn11;
        *var_tmpexp_dn2_slot = var_tmpexp_dn2;
        *var_tmpexp_dn3_slot = var_tmpexp_dn3;
        *var_tmpexp_dn4_slot = var_tmpexp_dn4;
        *var_tmpexp_dn5_slot = var_tmpexp_dn5;
        *var_tmpexp_dn6_slot = var_tmpexp_dn6;
        *var_tmpexp_dn7_slot = var_tmpexp_dn7;
        *var_tmpexp_dn8_slot = var_tmpexp_dn8;
        *var_tmpexp_dn9_slot = var_tmpexp_dn9;
        *var_vfs_slot = var_vfs;
        *var_vfs_db0_slot = var_vfs_db0;
        *var_vfs_db1_slot = var_vfs_db1;
        *var_vfs_dn0_slot = var_vfs_dn0;
        *var_vfs_dn1_slot = var_vfs_dn1;
        *var_vfs_dn10_slot = var_vfs_dn10;
        *var_vfs_dn11_slot = var_vfs_dn11;
        *var_vfs_dn2_slot = var_vfs_dn2;
        *var_vfs_dn3_slot = var_vfs_dn3;
        *var_vfs_dn4_slot = var_vfs_dn4;
        *var_vfs_dn5_slot = var_vfs_dn5;
        *var_vfs_dn6_slot = var_vfs_dn6;
        *var_vfs_dn7_slot = var_vfs_dn7;
        *var_vfs_dn8_slot = var_vfs_dn8;
        *var_vfs_dn9_slot = var_vfs_dn9;
        *var_vjs_slot = var_vjs;
        *var_vjs_db0_slot = var_vjs_db0;
        *var_vjs_db1_slot = var_vjs_db1;
        *var_vjs_dn0_slot = var_vjs_dn0;
        *var_vjs_dn1_slot = var_vjs_dn1;
        *var_vjs_dn10_slot = var_vjs_dn10;
        *var_vjs_dn11_slot = var_vjs_dn11;
        *var_vjs_dn2_slot = var_vjs_dn2;
        *var_vjs_dn3_slot = var_vjs_dn3;
        *var_vjs_dn4_slot = var_vjs_dn4;
        *var_vjs_dn5_slot = var_vjs_dn5;
        *var_vjs_dn6_slot = var_vjs_dn6;
        *var_vjs_dn7_slot = var_vjs_dn7;
        *var_vjs_dn8_slot = var_vjs_dn8;
        *var_vjs_dn9_slot = var_vjs_dn9;
        *var_xqtex_slot = var_xqtex;
        *var_xqtex_db0_slot = var_xqtex_db0;
        *var_xqtex_db1_slot = var_xqtex_db1;
        *var_xqtex_dn0_slot = var_xqtex_dn0;
        *var_xqtex_dn1_slot = var_xqtex_dn1;
        *var_xqtex_dn10_slot = var_xqtex_dn10;
        *var_xqtex_dn11_slot = var_xqtex_dn11;
        *var_xqtex_dn2_slot = var_xqtex_dn2;
        *var_xqtex_dn3_slot = var_xqtex_dn3;
        *var_xqtex_dn4_slot = var_xqtex_dn4;
        *var_xqtex_dn5_slot = var_xqtex_dn5;
        *var_xqtex_dn6_slot = var_xqtex_dn6;
        *var_xqtex_dn7_slot = var_xqtex_dn7;
        *var_xqtex_dn8_slot = var_xqtex_dn8;
        *var_xqtex_dn9_slot = var_xqtex_dn9;
        *var_xvjcex_slot = var_xvjcex;
        *var_xvjcex_db0_slot = var_xvjcex_db0;
        *var_xvjcex_db1_slot = var_xvjcex_db1;
        *var_xvjcex_dn0_slot = var_xvjcex_dn0;
        *var_xvjcex_dn1_slot = var_xvjcex_dn1;
        *var_xvjcex_dn10_slot = var_xvjcex_dn10;
        *var_xvjcex_dn11_slot = var_xvjcex_dn11;
        *var_xvjcex_dn2_slot = var_xvjcex_dn2;
        *var_xvjcex_dn3_slot = var_xvjcex_dn3;
        *var_xvjcex_dn4_slot = var_xvjcex_dn4;
        *var_xvjcex_dn5_slot = var_xvjcex_dn5;
        *var_xvjcex_dn6_slot = var_xvjcex_dn6;
        *var_xvjcex_dn7_slot = var_xvjcex_dn7;
        *var_xvjcex_dn8_slot = var_xvjcex_dn8;
        *var_xvjcex_dn9_slot = var_xvjcex_dn9;
        *var_xvtexv_slot = var_xvtexv;
        *var_xvtexv_db0_slot = var_xvtexv_db0;
        *var_xvtexv_db1_slot = var_xvtexv_db1;
        *var_xvtexv_dn0_slot = var_xvtexv_dn0;
        *var_xvtexv_dn1_slot = var_xvtexv_dn1;
        *var_xvtexv_dn10_slot = var_xvtexv_dn10;
        *var_xvtexv_dn11_slot = var_xvtexv_dn11;
        *var_xvtexv_dn2_slot = var_xvtexv_dn2;
        *var_xvtexv_dn3_slot = var_xvtexv_dn3;
        *var_xvtexv_dn4_slot = var_xvtexv_dn4;
        *var_xvtexv_dn5_slot = var_xvtexv_dn5;
        *var_xvtexv_dn6_slot = var_xvtexv_dn6;
        *var_xvtexv_dn7_slot = var_xvtexv_dn7;
        *var_xvtexv_dn8_slot = var_xvtexv_dn8;
        *var_xvtexv_dn9_slot = var_xvtexv_dn9;
    }

    pub(super) fn stamp_transient_block_17(
        p: &Parameters,
        var_a_vde: f64,
        var_a_vde_db0: f64,
        var_a_vde_db1: f64,
        var_a_vde_dn0: f64,
        var_a_vde_dn1: f64,
        var_a_vde_dn10: f64,
        var_a_vde_dn11: f64,
        var_a_vde_dn2: f64,
        var_a_vde_dn3: f64,
        var_a_vde_dn4: f64,
        var_a_vde_dn5: f64,
        var_a_vde_dn6: f64,
        var_a_vde_dn7: f64,
        var_a_vde_dn8: f64,
        var_a_vde_dn9: f64,
        var_evb1c4: f64,
        var_evb1c4_db0: f64,
        var_evb1c4_db1: f64,
        var_evb1c4_dn0: f64,
        var_evb1c4_dn1: f64,
        var_evb1c4_dn10: f64,
        var_evb1c4_dn11: f64,
        var_evb1c4_dn2: f64,
        var_evb1c4_dn3: f64,
        var_evb1c4_dn4: f64,
        var_evb1c4_dn5: f64,
        var_evb1c4_dn6: f64,
        var_evb1c4_dn7: f64,
        var_evb1c4_dn8: f64,
        var_evb1c4_dn9: f64,
        var_evbc3: f64,
        var_evbc3_db0: f64,
        var_evbc3_db1: f64,
        var_evbc3_dn0: f64,
        var_evbc3_dn1: f64,
        var_evbc3_dn10: f64,
        var_evbc3_dn11: f64,
        var_evbc3_dn2: f64,
        var_evbc3_dn3: f64,
        var_evbc3_dn4: f64,
        var_evbc3_dn5: f64,
        var_evbc3_dn6: f64,
        var_evbc3_dn7: f64,
        var_evbc3_dn8: f64,
        var_evbc3_dn9: f64,
        var_evbc3vdc: f64,
        var_evbc3vdc_db0: f64,
        var_evbc3vdc_db1: f64,
        var_evbc3vdc_dn0: f64,
        var_evbc3vdc_dn1: f64,
        var_evbc3vdc_dn10: f64,
        var_evbc3vdc_dn11: f64,
        var_evbc3vdc_dn2: f64,
        var_evbc3vdc_dn3: f64,
        var_evbc3vdc_dn4: f64,
        var_evbc3vdc_dn5: f64,
        var_evbc3vdc_dn6: f64,
        var_evbc3vdc_dn7: f64,
        var_evbc3vdc_dn8: f64,
        var_evbc3vdc_dn9: f64,
        var_fex: f64,
        var_fex_db0: f64,
        var_fex_db1: f64,
        var_fex_dn0: f64,
        var_fex_dn1: f64,
        var_fex_dn10: f64,
        var_fex_dn11: f64,
        var_fex_dn2: f64,
        var_fex_dn3: f64,
        var_fex_dn4: f64,
        var_fex_dn5: f64,
        var_fex_dn6: f64,
        var_fex_dn7: f64,
        var_fex_dn8: f64,
        var_fex_dn9: f64,
        var_guard115: f64,
        var_guard116: f64,
        var_ibx_t: f64,
        var_if0: f64,
        var_if0_db0: f64,
        var_if0_db1: f64,
        var_if0_dn0: f64,
        var_if0_dn1: f64,
        var_if0_dn10: f64,
        var_if0_dn11: f64,
        var_if0_dn2: f64,
        var_if0_dn3: f64,
        var_if0_dn4: f64,
        var_if0_dn5: f64,
        var_if0_dn6: f64,
        var_if0_dn7: f64,
        var_if0_dn8: f64,
        var_if0_dn9: f64,
        var_inv_vde_t: f64,
        var_inv_vde_t_db0: f64,
        var_inv_vde_t_db1: f64,
        var_inv_vde_t_dn0: f64,
        var_inv_vde_t_dn1: f64,
        var_inv_vde_t_dn10: f64,
        var_inv_vde_t_dn11: f64,
        var_inv_vde_t_dn2: f64,
        var_inv_vde_t_dn3: f64,
        var_inv_vde_t_dn4: f64,
        var_inv_vde_t_dn5: f64,
        var_inv_vde_t_dn6: f64,
        var_inv_vde_t_dn7: f64,
        var_inv_vde_t_dn8: f64,
        var_inv_vde_t_dn9: f64,
        var_qb0: f64,
        var_qepi0: f64,
        var_taub_t: f64,
        var_tauex_t: f64,
        var_taur_t: f64,
        var_tepi_t: f64,
        var_vb1c4: f64,
        var_vb1c4_db0: f64,
        var_vb1c4_db1: f64,
        var_vb1c4_dn0: f64,
        var_vb1c4_dn1: f64,
        var_vb1c4_dn10: f64,
        var_vb1c4_dn11: f64,
        var_vb1c4_dn2: f64,
        var_vb1c4_dn3: f64,
        var_vb1c4_dn4: f64,
        var_vb1c4_dn5: f64,
        var_vb1c4_dn6: f64,
        var_vb1c4_dn7: f64,
        var_vb1c4_dn8: f64,
        var_vb1c4_dn9: f64,
        var_vb2e1: f64,
        var_vb2e1_db0: f64,
        var_vb2e1_db1: f64,
        var_vb2e1_dn0: f64,
        var_vb2e1_dn1: f64,
        var_vb2e1_dn10: f64,
        var_vb2e1_dn11: f64,
        var_vb2e1_dn2: f64,
        var_vb2e1_dn3: f64,
        var_vb2e1_dn4: f64,
        var_vb2e1_dn5: f64,
        var_vb2e1_dn6: f64,
        var_vb2e1_dn7: f64,
        var_vb2e1_dn8: f64,
        var_vb2e1_dn9: f64,
        var_vbc3: f64,
        var_vbc3_db0: f64,
        var_vbc3_db1: f64,
        var_vbc3_dn0: f64,
        var_vbc3_dn1: f64,
        var_vbc3_dn10: f64,
        var_vbc3_dn11: f64,
        var_vbc3_dn2: f64,
        var_vbc3_dn3: f64,
        var_vbc3_dn4: f64,
        var_vbc3_dn5: f64,
        var_vbc3_dn6: f64,
        var_vbc3_dn7: f64,
        var_vbc3_dn8: f64,
        var_vbc3_dn9: f64,
        var_vdcex_t: f64,
        var_vdcex_t_db0: f64,
        var_vdcex_t_db1: f64,
        var_vdcex_t_dn0: f64,
        var_vdcex_t_dn1: f64,
        var_vdcex_t_dn10: f64,
        var_vdcex_t_dn11: f64,
        var_vdcex_t_dn2: f64,
        var_vdcex_t_dn3: f64,
        var_vdcex_t_dn4: f64,
        var_vdcex_t_dn5: f64,
        var_vdcex_t_dn6: f64,
        var_vdcex_t_dn7: f64,
        var_vdcex_t_dn8: f64,
        var_vdcex_t_dn9: f64,
        var_vfe: f64,
        var_vfe_db0: f64,
        var_vfe_db1: f64,
        var_vfe_dn0: f64,
        var_vfe_dn1: f64,
        var_vfe_dn10: f64,
        var_vfe_dn11: f64,
        var_vfe_dn2: f64,
        var_vfe_dn3: f64,
        var_vfe_dn4: f64,
        var_vfe_dn5: f64,
        var_vfe_dn6: f64,
        var_vfe_dn7: f64,
        var_vfe_dn8: f64,
        var_vfe_dn9: f64,
        var_vje: f64,
        var_vje_db0: f64,
        var_vje_db1: f64,
        var_vje_dn0: f64,
        var_vje_dn1: f64,
        var_vje_dn10: f64,
        var_vje_dn11: f64,
        var_vje_dn2: f64,
        var_vje_dn3: f64,
        var_vje_dn4: f64,
        var_vje_dn5: f64,
        var_vje_dn6: f64,
        var_vje_dn7: f64,
        var_vje_dn8: f64,
        var_vje_dn9: f64,
        var_vtinv: f64,
        var_xext1: f64,
        var_dvjevb2e1_slot: &mut f64,
        var_dvjevb2e1_db0_slot: &mut f64,
        var_dvjevb2e1_db1_slot: &mut f64,
        var_dvjevb2e1_dn0_slot: &mut f64,
        var_dvjevb2e1_dn1_slot: &mut f64,
        var_dvjevb2e1_dn10_slot: &mut f64,
        var_dvjevb2e1_dn11_slot: &mut f64,
        var_dvjevb2e1_dn2_slot: &mut f64,
        var_dvjevb2e1_dn3_slot: &mut f64,
        var_dvjevb2e1_dn4_slot: &mut f64,
        var_dvjevb2e1_dn5_slot: &mut f64,
        var_dvjevb2e1_dn6_slot: &mut f64,
        var_dvjevb2e1_dn7_slot: &mut f64,
        var_dvjevb2e1_dn8_slot: &mut f64,
        var_dvjevb2e1_dn9_slot: &mut f64,
        var_dvtevb2e1_slot: &mut f64,
        var_dvtevb2e1_db0_slot: &mut f64,
        var_dvtevb2e1_db1_slot: &mut f64,
        var_dvtevb2e1_dn0_slot: &mut f64,
        var_dvtevb2e1_dn1_slot: &mut f64,
        var_dvtevb2e1_dn10_slot: &mut f64,
        var_dvtevb2e1_dn11_slot: &mut f64,
        var_dvtevb2e1_dn2_slot: &mut f64,
        var_dvtevb2e1_dn3_slot: &mut f64,
        var_dvtevb2e1_dn4_slot: &mut f64,
        var_dvtevb2e1_dn5_slot: &mut f64,
        var_dvtevb2e1_dn6_slot: &mut f64,
        var_dvtevb2e1_dn7_slot: &mut f64,
        var_dvtevb2e1_dn8_slot: &mut f64,
        var_dvtevb2e1_dn9_slot: &mut f64,
        var_dvtevje_slot: &mut f64,
        var_dvtevje_db0_slot: &mut f64,
        var_dvtevje_db1_slot: &mut f64,
        var_dvtevje_dn0_slot: &mut f64,
        var_dvtevje_dn1_slot: &mut f64,
        var_dvtevje_dn10_slot: &mut f64,
        var_dvtevje_dn11_slot: &mut f64,
        var_dvtevje_dn2_slot: &mut f64,
        var_dvtevje_dn3_slot: &mut f64,
        var_dvtevje_dn4_slot: &mut f64,
        var_dvtevje_dn5_slot: &mut f64,
        var_dvtevje_dn6_slot: &mut f64,
        var_dvtevje_dn7_slot: &mut f64,
        var_dvtevje_dn8_slot: &mut f64,
        var_dvtevje_dn9_slot: &mut f64,
        var_evb1c4vdcex_slot: &mut f64,
        var_evb1c4vdcex_db0_slot: &mut f64,
        var_evb1c4vdcex_db1_slot: &mut f64,
        var_evb1c4vdcex_dn0_slot: &mut f64,
        var_evb1c4vdcex_dn1_slot: &mut f64,
        var_evb1c4vdcex_dn10_slot: &mut f64,
        var_evb1c4vdcex_dn11_slot: &mut f64,
        var_evb1c4vdcex_dn2_slot: &mut f64,
        var_evb1c4vdcex_dn3_slot: &mut f64,
        var_evb1c4vdcex_dn4_slot: &mut f64,
        var_evb1c4vdcex_dn5_slot: &mut f64,
        var_evb1c4vdcex_dn6_slot: &mut f64,
        var_evb1c4vdcex_dn7_slot: &mut f64,
        var_evb1c4vdcex_dn8_slot: &mut f64,
        var_evb1c4vdcex_dn9_slot: &mut f64,
        var_evbc3vdcex_slot: &mut f64,
        var_evbc3vdcex_db0_slot: &mut f64,
        var_evbc3vdcex_db1_slot: &mut f64,
        var_evbc3vdcex_dn0_slot: &mut f64,
        var_evbc3vdcex_dn1_slot: &mut f64,
        var_evbc3vdcex_dn10_slot: &mut f64,
        var_evbc3vdcex_dn11_slot: &mut f64,
        var_evbc3vdcex_dn2_slot: &mut f64,
        var_evbc3vdcex_dn3_slot: &mut f64,
        var_evbc3vdcex_dn4_slot: &mut f64,
        var_evbc3vdcex_dn5_slot: &mut f64,
        var_evbc3vdcex_dn6_slot: &mut f64,
        var_evbc3vdcex_dn7_slot: &mut f64,
        var_evbc3vdcex_dn8_slot: &mut f64,
        var_evbc3vdcex_dn9_slot: &mut f64,
        var_expl_slot: &mut f64,
        var_guard117_slot: &mut f64,
        var_guard118_slot: &mut f64,
        var_guard119_slot: &mut f64,
        var_guard120_slot: &mut f64,
        var_guard121_slot: &mut f64,
        var_qex_slot: &mut f64,
        var_qex_db0_slot: &mut f64,
        var_qex_db1_slot: &mut f64,
        var_qex_dn0_slot: &mut f64,
        var_qex_dn1_slot: &mut f64,
        var_qex_dn10_slot: &mut f64,
        var_qex_dn11_slot: &mut f64,
        var_qex_dn2_slot: &mut f64,
        var_qex_dn3_slot: &mut f64,
        var_qex_dn4_slot: &mut f64,
        var_qex_dn5_slot: &mut f64,
        var_qex_dn6_slot: &mut f64,
        var_qex_dn7_slot: &mut f64,
        var_qex_dn8_slot: &mut f64,
        var_qex_dn9_slot: &mut f64,
        var_vb2e1vfe_slot: &mut f64,
        var_vb2e1vfe_db0_slot: &mut f64,
        var_vb2e1vfe_db1_slot: &mut f64,
        var_vb2e1vfe_dn0_slot: &mut f64,
        var_vb2e1vfe_dn1_slot: &mut f64,
        var_vb2e1vfe_dn10_slot: &mut f64,
        var_vb2e1vfe_dn11_slot: &mut f64,
        var_vb2e1vfe_dn2_slot: &mut f64,
        var_vb2e1vfe_dn3_slot: &mut f64,
        var_vb2e1vfe_dn4_slot: &mut f64,
        var_vb2e1vfe_dn5_slot: &mut f64,
        var_vb2e1vfe_dn6_slot: &mut f64,
        var_vb2e1vfe_dn7_slot: &mut f64,
        var_vb2e1vfe_dn8_slot: &mut f64,
        var_vb2e1vfe_dn9_slot: &mut f64,
        var_xg1_slot: &mut f64,
        var_xg1_db0_slot: &mut f64,
        var_xg1_db1_slot: &mut f64,
        var_xg1_dn0_slot: &mut f64,
        var_xg1_dn1_slot: &mut f64,
        var_xg1_dn10_slot: &mut f64,
        var_xg1_dn11_slot: &mut f64,
        var_xg1_dn2_slot: &mut f64,
        var_xg1_dn3_slot: &mut f64,
        var_xg1_dn4_slot: &mut f64,
        var_xg1_dn5_slot: &mut f64,
        var_xg1_dn6_slot: &mut f64,
        var_xg1_dn7_slot: &mut f64,
        var_xg1_dn8_slot: &mut f64,
        var_xg1_dn9_slot: &mut f64,
        var_xg2_slot: &mut f64,
        var_xg2_db0_slot: &mut f64,
        var_xg2_db1_slot: &mut f64,
        var_xg2_dn0_slot: &mut f64,
        var_xg2_dn1_slot: &mut f64,
        var_xg2_dn10_slot: &mut f64,
        var_xg2_dn11_slot: &mut f64,
        var_xg2_dn2_slot: &mut f64,
        var_xg2_dn3_slot: &mut f64,
        var_xg2_dn4_slot: &mut f64,
        var_xg2_dn5_slot: &mut f64,
        var_xg2_dn6_slot: &mut f64,
        var_xg2_dn7_slot: &mut f64,
        var_xg2_dn8_slot: &mut f64,
        var_xg2_dn9_slot: &mut f64,
        var_xnbex_slot: &mut f64,
        var_xnbex_db0_slot: &mut f64,
        var_xnbex_db1_slot: &mut f64,
        var_xnbex_dn0_slot: &mut f64,
        var_xnbex_dn1_slot: &mut f64,
        var_xnbex_dn10_slot: &mut f64,
        var_xnbex_dn11_slot: &mut f64,
        var_xnbex_dn2_slot: &mut f64,
        var_xnbex_dn3_slot: &mut f64,
        var_xnbex_dn4_slot: &mut f64,
        var_xnbex_dn5_slot: &mut f64,
        var_xnbex_dn6_slot: &mut f64,
        var_xnbex_dn7_slot: &mut f64,
        var_xnbex_dn8_slot: &mut f64,
        var_xnbex_dn9_slot: &mut f64,
        var_xpwex_slot: &mut f64,
        var_xpwex_db0_slot: &mut f64,
        var_xpwex_db1_slot: &mut f64,
        var_xpwex_dn0_slot: &mut f64,
        var_xpwex_dn1_slot: &mut f64,
        var_xpwex_dn10_slot: &mut f64,
        var_xpwex_dn11_slot: &mut f64,
        var_xpwex_dn2_slot: &mut f64,
        var_xpwex_dn3_slot: &mut f64,
        var_xpwex_dn4_slot: &mut f64,
        var_xpwex_dn5_slot: &mut f64,
        var_xpwex_dn6_slot: &mut f64,
        var_xpwex_dn7_slot: &mut f64,
        var_xpwex_dn8_slot: &mut f64,
        var_xpwex_dn9_slot: &mut f64,
        var_xqex_slot: &mut f64,
        var_xqex_db0_slot: &mut f64,
        var_xqex_db1_slot: &mut f64,
        var_xqex_dn0_slot: &mut f64,
        var_xqex_dn1_slot: &mut f64,
        var_xqex_dn10_slot: &mut f64,
        var_xqex_dn11_slot: &mut f64,
        var_xqex_dn2_slot: &mut f64,
        var_xqex_dn3_slot: &mut f64,
        var_xqex_dn4_slot: &mut f64,
        var_xqex_dn5_slot: &mut f64,
        var_xqex_dn6_slot: &mut f64,
        var_xqex_dn7_slot: &mut f64,
        var_xqex_dn8_slot: &mut f64,
        var_xqex_dn9_slot: &mut f64,
        var_xqmex_slot: &mut f64,
        var_xqmex_db0_slot: &mut f64,
        var_xqmex_db1_slot: &mut f64,
        var_xqmex_dn0_slot: &mut f64,
        var_xqmex_dn1_slot: &mut f64,
        var_xqmex_dn10_slot: &mut f64,
        var_xqmex_dn11_slot: &mut f64,
        var_xqmex_dn2_slot: &mut f64,
        var_xqmex_dn3_slot: &mut f64,
        var_xqmex_dn4_slot: &mut f64,
        var_xqmex_dn5_slot: &mut f64,
        var_xqmex_dn6_slot: &mut f64,
        var_xqmex_dn7_slot: &mut f64,
        var_xqmex_dn8_slot: &mut f64,
        var_xqmex_dn9_slot: &mut f64,
    ) {
        let mut var_dvjevb2e1: f64 = *var_dvjevb2e1_slot;
        let mut var_dvjevb2e1_db0: f64 = *var_dvjevb2e1_db0_slot;
        let mut var_dvjevb2e1_db1: f64 = *var_dvjevb2e1_db1_slot;
        let mut var_dvjevb2e1_dn0: f64 = *var_dvjevb2e1_dn0_slot;
        let mut var_dvjevb2e1_dn1: f64 = *var_dvjevb2e1_dn1_slot;
        let mut var_dvjevb2e1_dn10: f64 = *var_dvjevb2e1_dn10_slot;
        let mut var_dvjevb2e1_dn11: f64 = *var_dvjevb2e1_dn11_slot;
        let mut var_dvjevb2e1_dn2: f64 = *var_dvjevb2e1_dn2_slot;
        let mut var_dvjevb2e1_dn3: f64 = *var_dvjevb2e1_dn3_slot;
        let mut var_dvjevb2e1_dn4: f64 = *var_dvjevb2e1_dn4_slot;
        let mut var_dvjevb2e1_dn5: f64 = *var_dvjevb2e1_dn5_slot;
        let mut var_dvjevb2e1_dn6: f64 = *var_dvjevb2e1_dn6_slot;
        let mut var_dvjevb2e1_dn7: f64 = *var_dvjevb2e1_dn7_slot;
        let mut var_dvjevb2e1_dn8: f64 = *var_dvjevb2e1_dn8_slot;
        let mut var_dvjevb2e1_dn9: f64 = *var_dvjevb2e1_dn9_slot;
        let mut var_dvtevb2e1: f64 = *var_dvtevb2e1_slot;
        let mut var_dvtevb2e1_db0: f64 = *var_dvtevb2e1_db0_slot;
        let mut var_dvtevb2e1_db1: f64 = *var_dvtevb2e1_db1_slot;
        let mut var_dvtevb2e1_dn0: f64 = *var_dvtevb2e1_dn0_slot;
        let mut var_dvtevb2e1_dn1: f64 = *var_dvtevb2e1_dn1_slot;
        let mut var_dvtevb2e1_dn10: f64 = *var_dvtevb2e1_dn10_slot;
        let mut var_dvtevb2e1_dn11: f64 = *var_dvtevb2e1_dn11_slot;
        let mut var_dvtevb2e1_dn2: f64 = *var_dvtevb2e1_dn2_slot;
        let mut var_dvtevb2e1_dn3: f64 = *var_dvtevb2e1_dn3_slot;
        let mut var_dvtevb2e1_dn4: f64 = *var_dvtevb2e1_dn4_slot;
        let mut var_dvtevb2e1_dn5: f64 = *var_dvtevb2e1_dn5_slot;
        let mut var_dvtevb2e1_dn6: f64 = *var_dvtevb2e1_dn6_slot;
        let mut var_dvtevb2e1_dn7: f64 = *var_dvtevb2e1_dn7_slot;
        let mut var_dvtevb2e1_dn8: f64 = *var_dvtevb2e1_dn8_slot;
        let mut var_dvtevb2e1_dn9: f64 = *var_dvtevb2e1_dn9_slot;
        let mut var_dvtevje: f64 = *var_dvtevje_slot;
        let mut var_dvtevje_db0: f64 = *var_dvtevje_db0_slot;
        let mut var_dvtevje_db1: f64 = *var_dvtevje_db1_slot;
        let mut var_dvtevje_dn0: f64 = *var_dvtevje_dn0_slot;
        let mut var_dvtevje_dn1: f64 = *var_dvtevje_dn1_slot;
        let mut var_dvtevje_dn10: f64 = *var_dvtevje_dn10_slot;
        let mut var_dvtevje_dn11: f64 = *var_dvtevje_dn11_slot;
        let mut var_dvtevje_dn2: f64 = *var_dvtevje_dn2_slot;
        let mut var_dvtevje_dn3: f64 = *var_dvtevje_dn3_slot;
        let mut var_dvtevje_dn4: f64 = *var_dvtevje_dn4_slot;
        let mut var_dvtevje_dn5: f64 = *var_dvtevje_dn5_slot;
        let mut var_dvtevje_dn6: f64 = *var_dvtevje_dn6_slot;
        let mut var_dvtevje_dn7: f64 = *var_dvtevje_dn7_slot;
        let mut var_dvtevje_dn8: f64 = *var_dvtevje_dn8_slot;
        let mut var_dvtevje_dn9: f64 = *var_dvtevje_dn9_slot;
        let mut var_evb1c4vdcex: f64 = *var_evb1c4vdcex_slot;
        let mut var_evb1c4vdcex_db0: f64 = *var_evb1c4vdcex_db0_slot;
        let mut var_evb1c4vdcex_db1: f64 = *var_evb1c4vdcex_db1_slot;
        let mut var_evb1c4vdcex_dn0: f64 = *var_evb1c4vdcex_dn0_slot;
        let mut var_evb1c4vdcex_dn1: f64 = *var_evb1c4vdcex_dn1_slot;
        let mut var_evb1c4vdcex_dn10: f64 = *var_evb1c4vdcex_dn10_slot;
        let mut var_evb1c4vdcex_dn11: f64 = *var_evb1c4vdcex_dn11_slot;
        let mut var_evb1c4vdcex_dn2: f64 = *var_evb1c4vdcex_dn2_slot;
        let mut var_evb1c4vdcex_dn3: f64 = *var_evb1c4vdcex_dn3_slot;
        let mut var_evb1c4vdcex_dn4: f64 = *var_evb1c4vdcex_dn4_slot;
        let mut var_evb1c4vdcex_dn5: f64 = *var_evb1c4vdcex_dn5_slot;
        let mut var_evb1c4vdcex_dn6: f64 = *var_evb1c4vdcex_dn6_slot;
        let mut var_evb1c4vdcex_dn7: f64 = *var_evb1c4vdcex_dn7_slot;
        let mut var_evb1c4vdcex_dn8: f64 = *var_evb1c4vdcex_dn8_slot;
        let mut var_evb1c4vdcex_dn9: f64 = *var_evb1c4vdcex_dn9_slot;
        let mut var_evbc3vdcex: f64 = *var_evbc3vdcex_slot;
        let mut var_evbc3vdcex_db0: f64 = *var_evbc3vdcex_db0_slot;
        let mut var_evbc3vdcex_db1: f64 = *var_evbc3vdcex_db1_slot;
        let mut var_evbc3vdcex_dn0: f64 = *var_evbc3vdcex_dn0_slot;
        let mut var_evbc3vdcex_dn1: f64 = *var_evbc3vdcex_dn1_slot;
        let mut var_evbc3vdcex_dn10: f64 = *var_evbc3vdcex_dn10_slot;
        let mut var_evbc3vdcex_dn11: f64 = *var_evbc3vdcex_dn11_slot;
        let mut var_evbc3vdcex_dn2: f64 = *var_evbc3vdcex_dn2_slot;
        let mut var_evbc3vdcex_dn3: f64 = *var_evbc3vdcex_dn3_slot;
        let mut var_evbc3vdcex_dn4: f64 = *var_evbc3vdcex_dn4_slot;
        let mut var_evbc3vdcex_dn5: f64 = *var_evbc3vdcex_dn5_slot;
        let mut var_evbc3vdcex_dn6: f64 = *var_evbc3vdcex_dn6_slot;
        let mut var_evbc3vdcex_dn7: f64 = *var_evbc3vdcex_dn7_slot;
        let mut var_evbc3vdcex_dn8: f64 = *var_evbc3vdcex_dn8_slot;
        let mut var_evbc3vdcex_dn9: f64 = *var_evbc3vdcex_dn9_slot;
        let mut var_expl: f64 = *var_expl_slot;
        let mut var_guard117: f64 = *var_guard117_slot;
        let mut var_guard118: f64 = *var_guard118_slot;
        let mut var_guard119: f64 = *var_guard119_slot;
        let mut var_guard120: f64 = *var_guard120_slot;
        let mut var_guard121: f64 = *var_guard121_slot;
        let mut var_qex: f64 = *var_qex_slot;
        let mut var_qex_db0: f64 = *var_qex_db0_slot;
        let mut var_qex_db1: f64 = *var_qex_db1_slot;
        let mut var_qex_dn0: f64 = *var_qex_dn0_slot;
        let mut var_qex_dn1: f64 = *var_qex_dn1_slot;
        let mut var_qex_dn10: f64 = *var_qex_dn10_slot;
        let mut var_qex_dn11: f64 = *var_qex_dn11_slot;
        let mut var_qex_dn2: f64 = *var_qex_dn2_slot;
        let mut var_qex_dn3: f64 = *var_qex_dn3_slot;
        let mut var_qex_dn4: f64 = *var_qex_dn4_slot;
        let mut var_qex_dn5: f64 = *var_qex_dn5_slot;
        let mut var_qex_dn6: f64 = *var_qex_dn6_slot;
        let mut var_qex_dn7: f64 = *var_qex_dn7_slot;
        let mut var_qex_dn8: f64 = *var_qex_dn8_slot;
        let mut var_qex_dn9: f64 = *var_qex_dn9_slot;
        let mut var_vb2e1vfe: f64 = *var_vb2e1vfe_slot;
        let mut var_vb2e1vfe_db0: f64 = *var_vb2e1vfe_db0_slot;
        let mut var_vb2e1vfe_db1: f64 = *var_vb2e1vfe_db1_slot;
        let mut var_vb2e1vfe_dn0: f64 = *var_vb2e1vfe_dn0_slot;
        let mut var_vb2e1vfe_dn1: f64 = *var_vb2e1vfe_dn1_slot;
        let mut var_vb2e1vfe_dn10: f64 = *var_vb2e1vfe_dn10_slot;
        let mut var_vb2e1vfe_dn11: f64 = *var_vb2e1vfe_dn11_slot;
        let mut var_vb2e1vfe_dn2: f64 = *var_vb2e1vfe_dn2_slot;
        let mut var_vb2e1vfe_dn3: f64 = *var_vb2e1vfe_dn3_slot;
        let mut var_vb2e1vfe_dn4: f64 = *var_vb2e1vfe_dn4_slot;
        let mut var_vb2e1vfe_dn5: f64 = *var_vb2e1vfe_dn5_slot;
        let mut var_vb2e1vfe_dn6: f64 = *var_vb2e1vfe_dn6_slot;
        let mut var_vb2e1vfe_dn7: f64 = *var_vb2e1vfe_dn7_slot;
        let mut var_vb2e1vfe_dn8: f64 = *var_vb2e1vfe_dn8_slot;
        let mut var_vb2e1vfe_dn9: f64 = *var_vb2e1vfe_dn9_slot;
        let mut var_xg1: f64 = *var_xg1_slot;
        let mut var_xg1_db0: f64 = *var_xg1_db0_slot;
        let mut var_xg1_db1: f64 = *var_xg1_db1_slot;
        let mut var_xg1_dn0: f64 = *var_xg1_dn0_slot;
        let mut var_xg1_dn1: f64 = *var_xg1_dn1_slot;
        let mut var_xg1_dn10: f64 = *var_xg1_dn10_slot;
        let mut var_xg1_dn11: f64 = *var_xg1_dn11_slot;
        let mut var_xg1_dn2: f64 = *var_xg1_dn2_slot;
        let mut var_xg1_dn3: f64 = *var_xg1_dn3_slot;
        let mut var_xg1_dn4: f64 = *var_xg1_dn4_slot;
        let mut var_xg1_dn5: f64 = *var_xg1_dn5_slot;
        let mut var_xg1_dn6: f64 = *var_xg1_dn6_slot;
        let mut var_xg1_dn7: f64 = *var_xg1_dn7_slot;
        let mut var_xg1_dn8: f64 = *var_xg1_dn8_slot;
        let mut var_xg1_dn9: f64 = *var_xg1_dn9_slot;
        let mut var_xg2: f64 = *var_xg2_slot;
        let mut var_xg2_db0: f64 = *var_xg2_db0_slot;
        let mut var_xg2_db1: f64 = *var_xg2_db1_slot;
        let mut var_xg2_dn0: f64 = *var_xg2_dn0_slot;
        let mut var_xg2_dn1: f64 = *var_xg2_dn1_slot;
        let mut var_xg2_dn10: f64 = *var_xg2_dn10_slot;
        let mut var_xg2_dn11: f64 = *var_xg2_dn11_slot;
        let mut var_xg2_dn2: f64 = *var_xg2_dn2_slot;
        let mut var_xg2_dn3: f64 = *var_xg2_dn3_slot;
        let mut var_xg2_dn4: f64 = *var_xg2_dn4_slot;
        let mut var_xg2_dn5: f64 = *var_xg2_dn5_slot;
        let mut var_xg2_dn6: f64 = *var_xg2_dn6_slot;
        let mut var_xg2_dn7: f64 = *var_xg2_dn7_slot;
        let mut var_xg2_dn8: f64 = *var_xg2_dn8_slot;
        let mut var_xg2_dn9: f64 = *var_xg2_dn9_slot;
        let mut var_xnbex: f64 = *var_xnbex_slot;
        let mut var_xnbex_db0: f64 = *var_xnbex_db0_slot;
        let mut var_xnbex_db1: f64 = *var_xnbex_db1_slot;
        let mut var_xnbex_dn0: f64 = *var_xnbex_dn0_slot;
        let mut var_xnbex_dn1: f64 = *var_xnbex_dn1_slot;
        let mut var_xnbex_dn10: f64 = *var_xnbex_dn10_slot;
        let mut var_xnbex_dn11: f64 = *var_xnbex_dn11_slot;
        let mut var_xnbex_dn2: f64 = *var_xnbex_dn2_slot;
        let mut var_xnbex_dn3: f64 = *var_xnbex_dn3_slot;
        let mut var_xnbex_dn4: f64 = *var_xnbex_dn4_slot;
        let mut var_xnbex_dn5: f64 = *var_xnbex_dn5_slot;
        let mut var_xnbex_dn6: f64 = *var_xnbex_dn6_slot;
        let mut var_xnbex_dn7: f64 = *var_xnbex_dn7_slot;
        let mut var_xnbex_dn8: f64 = *var_xnbex_dn8_slot;
        let mut var_xnbex_dn9: f64 = *var_xnbex_dn9_slot;
        let mut var_xpwex: f64 = *var_xpwex_slot;
        let mut var_xpwex_db0: f64 = *var_xpwex_db0_slot;
        let mut var_xpwex_db1: f64 = *var_xpwex_db1_slot;
        let mut var_xpwex_dn0: f64 = *var_xpwex_dn0_slot;
        let mut var_xpwex_dn1: f64 = *var_xpwex_dn1_slot;
        let mut var_xpwex_dn10: f64 = *var_xpwex_dn10_slot;
        let mut var_xpwex_dn11: f64 = *var_xpwex_dn11_slot;
        let mut var_xpwex_dn2: f64 = *var_xpwex_dn2_slot;
        let mut var_xpwex_dn3: f64 = *var_xpwex_dn3_slot;
        let mut var_xpwex_dn4: f64 = *var_xpwex_dn4_slot;
        let mut var_xpwex_dn5: f64 = *var_xpwex_dn5_slot;
        let mut var_xpwex_dn6: f64 = *var_xpwex_dn6_slot;
        let mut var_xpwex_dn7: f64 = *var_xpwex_dn7_slot;
        let mut var_xpwex_dn8: f64 = *var_xpwex_dn8_slot;
        let mut var_xpwex_dn9: f64 = *var_xpwex_dn9_slot;
        let mut var_xqex: f64 = *var_xqex_slot;
        let mut var_xqex_db0: f64 = *var_xqex_db0_slot;
        let mut var_xqex_db1: f64 = *var_xqex_db1_slot;
        let mut var_xqex_dn0: f64 = *var_xqex_dn0_slot;
        let mut var_xqex_dn1: f64 = *var_xqex_dn1_slot;
        let mut var_xqex_dn10: f64 = *var_xqex_dn10_slot;
        let mut var_xqex_dn11: f64 = *var_xqex_dn11_slot;
        let mut var_xqex_dn2: f64 = *var_xqex_dn2_slot;
        let mut var_xqex_dn3: f64 = *var_xqex_dn3_slot;
        let mut var_xqex_dn4: f64 = *var_xqex_dn4_slot;
        let mut var_xqex_dn5: f64 = *var_xqex_dn5_slot;
        let mut var_xqex_dn6: f64 = *var_xqex_dn6_slot;
        let mut var_xqex_dn7: f64 = *var_xqex_dn7_slot;
        let mut var_xqex_dn8: f64 = *var_xqex_dn8_slot;
        let mut var_xqex_dn9: f64 = *var_xqex_dn9_slot;
        let mut var_xqmex: f64 = *var_xqmex_slot;
        let mut var_xqmex_db0: f64 = *var_xqmex_db0_slot;
        let mut var_xqmex_db1: f64 = *var_xqmex_db1_slot;
        let mut var_xqmex_dn0: f64 = *var_xqmex_dn0_slot;
        let mut var_xqmex_dn1: f64 = *var_xqmex_dn1_slot;
        let mut var_xqmex_dn10: f64 = *var_xqmex_dn10_slot;
        let mut var_xqmex_dn11: f64 = *var_xqmex_dn11_slot;
        let mut var_xqmex_dn2: f64 = *var_xqmex_dn2_slot;
        let mut var_xqmex_dn3: f64 = *var_xqmex_dn3_slot;
        let mut var_xqmex_dn4: f64 = *var_xqmex_dn4_slot;
        let mut var_xqmex_dn5: f64 = *var_xqmex_dn5_slot;
        let mut var_xqmex_dn6: f64 = *var_xqmex_dn6_slot;
        let mut var_xqmex_dn7: f64 = *var_xqmex_dn7_slot;
        let mut var_xqmex_dn8: f64 = *var_xqmex_dn8_slot;
        let mut var_xqmex_dn9: f64 = *var_xqmex_dn9_slot;

        let (assign6450_e6641, assign6450_e6641_d_n0, assign6450_e6641_d_n1, assign6450_e6641_d_n2, assign6450_e6641_d_n3, assign6450_e6641_d_n4, assign6450_e6641_d_n5, assign6450_e6641_d_n6, assign6450_e6641_d_n7, assign6450_e6641_d_n8, assign6450_e6641_d_n9, assign6450_e6641_d_n10, assign6450_e6641_d_n11, assign6450_e6641_d_b0, assign6450_e6641_d_b1,) = {
    if ((var_guard115 == 0.0) && (var_guard116 == 0.0)) {
        let assign6450_e6631: f64 = (var_vb1c4 - var_vdcex_t);
        let assign6450_e6633: f64 = (assign6450_e6631 / p.p91);
        let assign6450_e6635: f64 = (assign6450_e6633 * var_vtinv);
        let assign6450_e6637: f64 = (assign6450_e6635 - p.p147);
        let assign6450_e6638: f64 = (1.0 + assign6450_e6637);
        let assign6450_e6639: f64 = (var_expl * assign6450_e6638);
        (assign6450_e6639, (var_expl * (((var_vb1c4_dn0 - var_vdcex_t_dn0) / p.p91) * var_vtinv)), (var_expl * (((var_vb1c4_dn1 - var_vdcex_t_dn1) / p.p91) * var_vtinv)), (var_expl * (((var_vb1c4_dn2 - var_vdcex_t_dn2) / p.p91) * var_vtinv)), (var_expl * (((var_vb1c4_dn3 - var_vdcex_t_dn3) / p.p91) * var_vtinv)), (var_expl * (((var_vb1c4_dn4 - var_vdcex_t_dn4) / p.p91) * var_vtinv)), (var_expl * (((var_vb1c4_dn5 - var_vdcex_t_dn5) / p.p91) * var_vtinv)), (var_expl * (((var_vb1c4_dn6 - var_vdcex_t_dn6) / p.p91) * var_vtinv)), (var_expl * (((var_vb1c4_dn7 - var_vdcex_t_dn7) / p.p91) * var_vtinv)), (var_expl * (((var_vb1c4_dn8 - var_vdcex_t_dn8) / p.p91) * var_vtinv)), (var_expl * (((var_vb1c4_dn9 - var_vdcex_t_dn9) / p.p91) * var_vtinv)), (var_expl * (((var_vb1c4_dn10 - var_vdcex_t_dn10) / p.p91) * var_vtinv)), (var_expl * (((var_vb1c4_dn11 - var_vdcex_t_dn11) / p.p91) * var_vtinv)), (var_expl * (((var_vb1c4_db0 - var_vdcex_t_db0) / p.p91) * var_vtinv)), (var_expl * (((var_vb1c4_db1 - var_vdcex_t_db1) / p.p91) * var_vtinv)),)
    } else {
        (var_evb1c4vdcex, var_evb1c4vdcex_dn0, var_evb1c4vdcex_dn1, var_evb1c4vdcex_dn2, var_evb1c4vdcex_dn3, var_evb1c4vdcex_dn4, var_evb1c4vdcex_dn5, var_evb1c4vdcex_dn6, var_evb1c4vdcex_dn7, var_evb1c4vdcex_dn8, var_evb1c4vdcex_dn9, var_evb1c4vdcex_dn10, var_evb1c4vdcex_dn11, var_evb1c4vdcex_db0, var_evb1c4vdcex_db1,)
    }
};
        var_evb1c4vdcex = assign6450_e6641;
        var_evb1c4vdcex_dn0 = assign6450_e6641_d_n0;
        var_evb1c4vdcex_dn1 = assign6450_e6641_d_n1;
        var_evb1c4vdcex_dn2 = assign6450_e6641_d_n2;
        var_evb1c4vdcex_dn3 = assign6450_e6641_d_n3;
        var_evb1c4vdcex_dn4 = assign6450_e6641_d_n4;
        var_evb1c4vdcex_dn5 = assign6450_e6641_d_n5;
        var_evb1c4vdcex_dn6 = assign6450_e6641_d_n6;
        var_evb1c4vdcex_dn7 = assign6450_e6641_d_n7;
        var_evb1c4vdcex_dn8 = assign6450_e6641_d_n8;
        var_evb1c4vdcex_dn9 = assign6450_e6641_d_n9;
        var_evb1c4vdcex_dn10 = assign6450_e6641_d_n10;
        var_evb1c4vdcex_dn11 = assign6450_e6641_d_n11;
        var_evb1c4vdcex_db0 = assign6450_e6641_d_b0;
        var_evb1c4vdcex_db1 = assign6450_e6641_d_b1;

        let (assign6460_e6661, assign6460_e6661_d_n0, assign6460_e6661_d_n1, assign6460_e6661_d_n2, assign6460_e6661_d_n3, assign6460_e6661_d_n4, assign6460_e6661_d_n5, assign6460_e6661_d_n6, assign6460_e6661_d_n7, assign6460_e6661_d_n8, assign6460_e6661_d_n9, assign6460_e6661_d_n10, assign6460_e6661_d_n11, assign6460_e6661_d_b0, assign6460_e6661_d_b1,) = {
    if (var_guard115 == 0.0) {
        let assign6460_e6646: f64 = (2.0 * var_ibx_t);
        let assign6460_e6648: f64 = (assign6460_e6646 * var_tauex_t);
        let assign6460_e6650: f64 = (assign6460_e6648 * var_evb1c4);
        let assign6460_e6655: f64 = (4.0 * var_evb1c4vdcex);
        let assign6460_e6656: f64 = (1.0 + assign6460_e6655);
        let assign6460_e6657: f64 = (assign6460_e6656).sqrt();
        let assign6460_e6658: f64 = (1.0 + assign6460_e6657);
        let assign6460_e6659: f64 = (assign6460_e6650 / assign6460_e6658);
        (assign6460_e6659, ((((assign6460_e6648 * var_evb1c4_dn0) * assign6460_e6658) - (assign6460_e6650 * ((4.0 * var_evb1c4vdcex_dn0) / (2.0 * assign6460_e6657)))) / (assign6460_e6658 * assign6460_e6658)), ((((assign6460_e6648 * var_evb1c4_dn1) * assign6460_e6658) - (assign6460_e6650 * ((4.0 * var_evb1c4vdcex_dn1) / (2.0 * assign6460_e6657)))) / (assign6460_e6658 * assign6460_e6658)), ((((assign6460_e6648 * var_evb1c4_dn2) * assign6460_e6658) - (assign6460_e6650 * ((4.0 * var_evb1c4vdcex_dn2) / (2.0 * assign6460_e6657)))) / (assign6460_e6658 * assign6460_e6658)), ((((assign6460_e6648 * var_evb1c4_dn3) * assign6460_e6658) - (assign6460_e6650 * ((4.0 * var_evb1c4vdcex_dn3) / (2.0 * assign6460_e6657)))) / (assign6460_e6658 * assign6460_e6658)), ((((assign6460_e6648 * var_evb1c4_dn4) * assign6460_e6658) - (assign6460_e6650 * ((4.0 * var_evb1c4vdcex_dn4) / (2.0 * assign6460_e6657)))) / (assign6460_e6658 * assign6460_e6658)), ((((assign6460_e6648 * var_evb1c4_dn5) * assign6460_e6658) - (assign6460_e6650 * ((4.0 * var_evb1c4vdcex_dn5) / (2.0 * assign6460_e6657)))) / (assign6460_e6658 * assign6460_e6658)), ((((assign6460_e6648 * var_evb1c4_dn6) * assign6460_e6658) - (assign6460_e6650 * ((4.0 * var_evb1c4vdcex_dn6) / (2.0 * assign6460_e6657)))) / (assign6460_e6658 * assign6460_e6658)), ((((assign6460_e6648 * var_evb1c4_dn7) * assign6460_e6658) - (assign6460_e6650 * ((4.0 * var_evb1c4vdcex_dn7) / (2.0 * assign6460_e6657)))) / (assign6460_e6658 * assign6460_e6658)), ((((assign6460_e6648 * var_evb1c4_dn8) * assign6460_e6658) - (assign6460_e6650 * ((4.0 * var_evb1c4vdcex_dn8) / (2.0 * assign6460_e6657)))) / (assign6460_e6658 * assign6460_e6658)), ((((assign6460_e6648 * var_evb1c4_dn9) * assign6460_e6658) - (assign6460_e6650 * ((4.0 * var_evb1c4vdcex_dn9) / (2.0 * assign6460_e6657)))) / (assign6460_e6658 * assign6460_e6658)), ((((assign6460_e6648 * var_evb1c4_dn10) * assign6460_e6658) - (assign6460_e6650 * ((4.0 * var_evb1c4vdcex_dn10) / (2.0 * assign6460_e6657)))) / (assign6460_e6658 * assign6460_e6658)), ((((assign6460_e6648 * var_evb1c4_dn11) * assign6460_e6658) - (assign6460_e6650 * ((4.0 * var_evb1c4vdcex_dn11) / (2.0 * assign6460_e6657)))) / (assign6460_e6658 * assign6460_e6658)), ((((assign6460_e6648 * var_evb1c4_db0) * assign6460_e6658) - (assign6460_e6650 * ((4.0 * var_evb1c4vdcex_db0) / (2.0 * assign6460_e6657)))) / (assign6460_e6658 * assign6460_e6658)), ((((assign6460_e6648 * var_evb1c4_db1) * assign6460_e6658) - (assign6460_e6650 * ((4.0 * var_evb1c4vdcex_db1) / (2.0 * assign6460_e6657)))) / (assign6460_e6658 * assign6460_e6658)),)
    } else {
        (var_qex, var_qex_dn0, var_qex_dn1, var_qex_dn2, var_qex_dn3, var_qex_dn4, var_qex_dn5, var_qex_dn6, var_qex_dn7, var_qex_dn8, var_qex_dn9, var_qex_dn10, var_qex_dn11, var_qex_db0, var_qex_db1,)
    }
};
        var_qex = assign6460_e6661;
        var_qex_dn0 = assign6460_e6661_d_n0;
        var_qex_dn1 = assign6460_e6661_d_n1;
        var_qex_dn2 = assign6460_e6661_d_n2;
        var_qex_dn3 = assign6460_e6661_d_n3;
        var_qex_dn4 = assign6460_e6661_d_n4;
        var_qex_dn5 = assign6460_e6661_d_n5;
        var_qex_dn6 = assign6460_e6661_d_n6;
        var_qex_dn7 = assign6460_e6661_d_n7;
        var_qex_dn8 = assign6460_e6661_d_n8;
        var_qex_dn9 = assign6460_e6661_d_n9;
        var_qex_dn10 = assign6460_e6661_d_n10;
        var_qex_dn11 = assign6460_e6661_d_n11;
        var_qex_db0 = assign6460_e6661_d_b0;
        var_qex_db1 = assign6460_e6661_d_b1;

        let assign6470_e6672: f64 = if (((p.p5 == 1.0) || (p.p5 == 3.0)) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };
        var_guard117 = assign6470_e6672;

        let (assign6480_e6678, assign6480_e6678_d_n0, assign6480_e6678_d_n1, assign6480_e6678_d_n2, assign6480_e6678_d_n3, assign6480_e6678_d_n4, assign6480_e6678_d_n5, assign6480_e6678_d_n6, assign6480_e6678_d_n7, assign6480_e6678_d_n8, assign6480_e6678_d_n9, assign6480_e6678_d_n10, assign6480_e6678_d_n11, assign6480_e6678_d_b0, assign6480_e6678_d_b1,) = {
    if (var_guard117 != 0.0) {
        let assign6480_e6676: f64 = (var_qex * var_xext1);
        (assign6480_e6676, (var_qex_dn0 * var_xext1), (var_qex_dn1 * var_xext1), (var_qex_dn2 * var_xext1), (var_qex_dn3 * var_xext1), (var_qex_dn4 * var_xext1), (var_qex_dn5 * var_xext1), (var_qex_dn6 * var_xext1), (var_qex_dn7 * var_xext1), (var_qex_dn8 * var_xext1), (var_qex_dn9 * var_xext1), (var_qex_dn10 * var_xext1), (var_qex_dn11 * var_xext1), (var_qex_db0 * var_xext1), (var_qex_db1 * var_xext1),)
    } else {
        (var_qex, var_qex_dn0, var_qex_dn1, var_qex_dn2, var_qex_dn3, var_qex_dn4, var_qex_dn5, var_qex_dn6, var_qex_dn7, var_qex_dn8, var_qex_dn9, var_qex_dn10, var_qex_dn11, var_qex_db0, var_qex_db1,)
    }
};
        var_qex = assign6480_e6678;
        var_qex_dn0 = assign6480_e6678_d_n0;
        var_qex_dn1 = assign6480_e6678_d_n1;
        var_qex_dn2 = assign6480_e6678_d_n2;
        var_qex_dn3 = assign6480_e6678_d_n3;
        var_qex_dn4 = assign6480_e6678_d_n4;
        var_qex_dn5 = assign6480_e6678_d_n5;
        var_qex_dn6 = assign6480_e6678_d_n6;
        var_qex_dn7 = assign6480_e6678_d_n7;
        var_qex_dn8 = assign6480_e6678_d_n8;
        var_qex_dn9 = assign6480_e6678_d_n9;
        var_qex_dn10 = assign6480_e6678_d_n10;
        var_qex_dn11 = assign6480_e6678_d_n11;
        var_qex_db0 = assign6480_e6678_d_b0;
        var_qex_db1 = assign6480_e6678_d_b1;

        let assign6490_e6681: f64 = if p.p79 == 0.0 { 1.0 } else { 0.0 };
        var_guard118 = assign6490_e6681;

        let (assign6500_e6689, assign6500_e6689_d_n0, assign6500_e6689_d_n1, assign6500_e6689_d_n2, assign6500_e6689_d_n3, assign6500_e6689_d_n4, assign6500_e6689_d_n5, assign6500_e6689_d_n6, assign6500_e6689_d_n7, assign6500_e6689_d_n8, assign6500_e6689_d_n9, assign6500_e6689_d_n10, assign6500_e6689_d_n11, assign6500_e6689_d_b0, assign6500_e6689_d_b1,) = {
    if ((var_guard117 != 0.0) && (var_guard118 != 0.0)) {
        let assign6500_e6687: f64 = (var_if0 * var_evbc3);
        (assign6500_e6687, ((var_if0_dn0 * var_evbc3) + (var_if0 * var_evbc3_dn0)), ((var_if0_dn1 * var_evbc3) + (var_if0 * var_evbc3_dn1)), ((var_if0_dn2 * var_evbc3) + (var_if0 * var_evbc3_dn2)), ((var_if0_dn3 * var_evbc3) + (var_if0 * var_evbc3_dn3)), ((var_if0_dn4 * var_evbc3) + (var_if0 * var_evbc3_dn4)), ((var_if0_dn5 * var_evbc3) + (var_if0 * var_evbc3_dn5)), ((var_if0_dn6 * var_evbc3) + (var_if0 * var_evbc3_dn6)), ((var_if0_dn7 * var_evbc3) + (var_if0 * var_evbc3_dn7)), ((var_if0_dn8 * var_evbc3) + (var_if0 * var_evbc3_dn8)), ((var_if0_dn9 * var_evbc3) + (var_if0 * var_evbc3_dn9)), ((var_if0_dn10 * var_evbc3) + (var_if0 * var_evbc3_dn10)), ((var_if0_dn11 * var_evbc3) + (var_if0 * var_evbc3_dn11)), ((var_if0_db0 * var_evbc3) + (var_if0 * var_evbc3_db0)), ((var_if0_db1 * var_evbc3) + (var_if0 * var_evbc3_db1)),)
    } else {
        (var_xg1, var_xg1_dn0, var_xg1_dn1, var_xg1_dn2, var_xg1_dn3, var_xg1_dn4, var_xg1_dn5, var_xg1_dn6, var_xg1_dn7, var_xg1_dn8, var_xg1_dn9, var_xg1_dn10, var_xg1_dn11, var_xg1_db0, var_xg1_db1,)
    }
};
        var_xg1 = assign6500_e6689;
        var_xg1_dn0 = assign6500_e6689_d_n0;
        var_xg1_dn1 = assign6500_e6689_d_n1;
        var_xg1_dn2 = assign6500_e6689_d_n2;
        var_xg1_dn3 = assign6500_e6689_d_n3;
        var_xg1_dn4 = assign6500_e6689_d_n4;
        var_xg1_dn5 = assign6500_e6689_d_n5;
        var_xg1_dn6 = assign6500_e6689_d_n6;
        var_xg1_dn7 = assign6500_e6689_d_n7;
        var_xg1_dn8 = assign6500_e6689_d_n8;
        var_xg1_dn9 = assign6500_e6689_d_n9;
        var_xg1_dn10 = assign6500_e6689_d_n10;
        var_xg1_dn11 = assign6500_e6689_d_n11;
        var_xg1_db0 = assign6500_e6689_d_b0;
        var_xg1_db1 = assign6500_e6689_d_b1;

        let (assign6510_e6704, assign6510_e6704_d_n0, assign6510_e6704_d_n1, assign6510_e6704_d_n2, assign6510_e6704_d_n3, assign6510_e6704_d_n4, assign6510_e6704_d_n5, assign6510_e6704_d_n6, assign6510_e6704_d_n7, assign6510_e6704_d_n8, assign6510_e6704_d_n9, assign6510_e6704_d_n10, assign6510_e6704_d_n11, assign6510_e6704_d_b0, assign6510_e6704_d_b1,) = {
    if ((var_guard117 != 0.0) && (var_guard118 != 0.0)) {
        let assign6510_e6695: f64 = (var_xg1 - var_if0);
        let assign6510_e6699: f64 = (1.0 + var_xg1);
        let assign6510_e6700: f64 = (assign6510_e6699).sqrt();
        let assign6510_e6701: f64 = (1.0 + assign6510_e6700);
        let assign6510_e6702: f64 = (assign6510_e6695 / assign6510_e6701);
        (assign6510_e6702, ((((var_xg1_dn0 - var_if0_dn0) * assign6510_e6701) - (assign6510_e6695 * (var_xg1_dn0 / (2.0 * assign6510_e6700)))) / (assign6510_e6701 * assign6510_e6701)), ((((var_xg1_dn1 - var_if0_dn1) * assign6510_e6701) - (assign6510_e6695 * (var_xg1_dn1 / (2.0 * assign6510_e6700)))) / (assign6510_e6701 * assign6510_e6701)), ((((var_xg1_dn2 - var_if0_dn2) * assign6510_e6701) - (assign6510_e6695 * (var_xg1_dn2 / (2.0 * assign6510_e6700)))) / (assign6510_e6701 * assign6510_e6701)), ((((var_xg1_dn3 - var_if0_dn3) * assign6510_e6701) - (assign6510_e6695 * (var_xg1_dn3 / (2.0 * assign6510_e6700)))) / (assign6510_e6701 * assign6510_e6701)), ((((var_xg1_dn4 - var_if0_dn4) * assign6510_e6701) - (assign6510_e6695 * (var_xg1_dn4 / (2.0 * assign6510_e6700)))) / (assign6510_e6701 * assign6510_e6701)), ((((var_xg1_dn5 - var_if0_dn5) * assign6510_e6701) - (assign6510_e6695 * (var_xg1_dn5 / (2.0 * assign6510_e6700)))) / (assign6510_e6701 * assign6510_e6701)), ((((var_xg1_dn6 - var_if0_dn6) * assign6510_e6701) - (assign6510_e6695 * (var_xg1_dn6 / (2.0 * assign6510_e6700)))) / (assign6510_e6701 * assign6510_e6701)), ((((var_xg1_dn7 - var_if0_dn7) * assign6510_e6701) - (assign6510_e6695 * (var_xg1_dn7 / (2.0 * assign6510_e6700)))) / (assign6510_e6701 * assign6510_e6701)), ((((var_xg1_dn8 - var_if0_dn8) * assign6510_e6701) - (assign6510_e6695 * (var_xg1_dn8 / (2.0 * assign6510_e6700)))) / (assign6510_e6701 * assign6510_e6701)), ((((var_xg1_dn9 - var_if0_dn9) * assign6510_e6701) - (assign6510_e6695 * (var_xg1_dn9 / (2.0 * assign6510_e6700)))) / (assign6510_e6701 * assign6510_e6701)), ((((var_xg1_dn10 - var_if0_dn10) * assign6510_e6701) - (assign6510_e6695 * (var_xg1_dn10 / (2.0 * assign6510_e6700)))) / (assign6510_e6701 * assign6510_e6701)), ((((var_xg1_dn11 - var_if0_dn11) * assign6510_e6701) - (assign6510_e6695 * (var_xg1_dn11 / (2.0 * assign6510_e6700)))) / (assign6510_e6701 * assign6510_e6701)), ((((var_xg1_db0 - var_if0_db0) * assign6510_e6701) - (assign6510_e6695 * (var_xg1_db0 / (2.0 * assign6510_e6700)))) / (assign6510_e6701 * assign6510_e6701)), ((((var_xg1_db1 - var_if0_db1) * assign6510_e6701) - (assign6510_e6695 * (var_xg1_db1 / (2.0 * assign6510_e6700)))) / (assign6510_e6701 * assign6510_e6701)),)
    } else {
        (var_xnbex, var_xnbex_dn0, var_xnbex_dn1, var_xnbex_dn2, var_xnbex_dn3, var_xnbex_dn4, var_xnbex_dn5, var_xnbex_dn6, var_xnbex_dn7, var_xnbex_dn8, var_xnbex_dn9, var_xnbex_dn10, var_xnbex_dn11, var_xnbex_db0, var_xnbex_db1,)
    }
};
        var_xnbex = assign6510_e6704;
        var_xnbex_dn0 = assign6510_e6704_d_n0;
        var_xnbex_dn1 = assign6510_e6704_d_n1;
        var_xnbex_dn2 = assign6510_e6704_d_n2;
        var_xnbex_dn3 = assign6510_e6704_d_n3;
        var_xnbex_dn4 = assign6510_e6704_d_n4;
        var_xnbex_dn5 = assign6510_e6704_d_n5;
        var_xnbex_dn6 = assign6510_e6704_d_n6;
        var_xnbex_dn7 = assign6510_e6704_d_n7;
        var_xnbex_dn8 = assign6510_e6704_d_n8;
        var_xnbex_dn9 = assign6510_e6704_d_n9;
        var_xnbex_dn10 = assign6510_e6704_d_n10;
        var_xnbex_dn11 = assign6510_e6704_d_n11;
        var_xnbex_db0 = assign6510_e6704_d_b0;
        var_xnbex_db1 = assign6510_e6704_d_b1;

        let (assign6520_e6712, assign6520_e6712_d_n0, assign6520_e6712_d_n1, assign6520_e6712_d_n2, assign6520_e6712_d_n3, assign6520_e6712_d_n4, assign6520_e6712_d_n5, assign6520_e6712_d_n6, assign6520_e6712_d_n7, assign6520_e6712_d_n8, assign6520_e6712_d_n9, assign6520_e6712_d_n10, assign6520_e6712_d_n11, assign6520_e6712_d_b0, assign6520_e6712_d_b1,) = {
    if ((var_guard117 != 0.0) && (var_guard118 != 0.0)) {
        let assign6520_e6710: f64 = (4.0 * var_evbc3vdc);
        (assign6520_e6710, (4.0 * var_evbc3vdc_dn0), (4.0 * var_evbc3vdc_dn1), (4.0 * var_evbc3vdc_dn2), (4.0 * var_evbc3vdc_dn3), (4.0 * var_evbc3vdc_dn4), (4.0 * var_evbc3vdc_dn5), (4.0 * var_evbc3vdc_dn6), (4.0 * var_evbc3vdc_dn7), (4.0 * var_evbc3vdc_dn8), (4.0 * var_evbc3vdc_dn9), (4.0 * var_evbc3vdc_dn10), (4.0 * var_evbc3vdc_dn11), (4.0 * var_evbc3vdc_db0), (4.0 * var_evbc3vdc_db1),)
    } else {
        (var_xg2, var_xg2_dn0, var_xg2_dn1, var_xg2_dn2, var_xg2_dn3, var_xg2_dn4, var_xg2_dn5, var_xg2_dn6, var_xg2_dn7, var_xg2_dn8, var_xg2_dn9, var_xg2_dn10, var_xg2_dn11, var_xg2_db0, var_xg2_db1,)
    }
};
        var_xg2 = assign6520_e6712;
        var_xg2_dn0 = assign6520_e6712_d_n0;
        var_xg2_dn1 = assign6520_e6712_d_n1;
        var_xg2_dn2 = assign6520_e6712_d_n2;
        var_xg2_dn3 = assign6520_e6712_d_n3;
        var_xg2_dn4 = assign6520_e6712_d_n4;
        var_xg2_dn5 = assign6520_e6712_d_n5;
        var_xg2_dn6 = assign6520_e6712_d_n6;
        var_xg2_dn7 = assign6520_e6712_d_n7;
        var_xg2_dn8 = assign6520_e6712_d_n8;
        var_xg2_dn9 = assign6520_e6712_d_n9;
        var_xg2_dn10 = assign6520_e6712_d_n10;
        var_xg2_dn11 = assign6520_e6712_d_n11;
        var_xg2_db0 = assign6520_e6712_d_b0;
        var_xg2_db1 = assign6520_e6712_d_b1;

        let (assign6530_e6725, assign6530_e6725_d_n0, assign6530_e6725_d_n1, assign6530_e6725_d_n2, assign6530_e6725_d_n3, assign6530_e6725_d_n4, assign6530_e6725_d_n5, assign6530_e6725_d_n6, assign6530_e6725_d_n7, assign6530_e6725_d_n8, assign6530_e6725_d_n9, assign6530_e6725_d_n10, assign6530_e6725_d_n11, assign6530_e6725_d_b0, assign6530_e6725_d_b1,) = {
    if ((var_guard117 != 0.0) && (var_guard118 != 0.0)) {
        let assign6530_e6720: f64 = (1.0 + var_xg2);
        let assign6530_e6721: f64 = (assign6530_e6720).sqrt();
        let assign6530_e6722: f64 = (1.0 + assign6530_e6721);
        let assign6530_e6723: f64 = (var_xg2 / assign6530_e6722);
        (assign6530_e6723, (((var_xg2_dn0 * assign6530_e6722) - (var_xg2 * (var_xg2_dn0 / (2.0 * assign6530_e6721)))) / (assign6530_e6722 * assign6530_e6722)), (((var_xg2_dn1 * assign6530_e6722) - (var_xg2 * (var_xg2_dn1 / (2.0 * assign6530_e6721)))) / (assign6530_e6722 * assign6530_e6722)), (((var_xg2_dn2 * assign6530_e6722) - (var_xg2 * (var_xg2_dn2 / (2.0 * assign6530_e6721)))) / (assign6530_e6722 * assign6530_e6722)), (((var_xg2_dn3 * assign6530_e6722) - (var_xg2 * (var_xg2_dn3 / (2.0 * assign6530_e6721)))) / (assign6530_e6722 * assign6530_e6722)), (((var_xg2_dn4 * assign6530_e6722) - (var_xg2 * (var_xg2_dn4 / (2.0 * assign6530_e6721)))) / (assign6530_e6722 * assign6530_e6722)), (((var_xg2_dn5 * assign6530_e6722) - (var_xg2 * (var_xg2_dn5 / (2.0 * assign6530_e6721)))) / (assign6530_e6722 * assign6530_e6722)), (((var_xg2_dn6 * assign6530_e6722) - (var_xg2 * (var_xg2_dn6 / (2.0 * assign6530_e6721)))) / (assign6530_e6722 * assign6530_e6722)), (((var_xg2_dn7 * assign6530_e6722) - (var_xg2 * (var_xg2_dn7 / (2.0 * assign6530_e6721)))) / (assign6530_e6722 * assign6530_e6722)), (((var_xg2_dn8 * assign6530_e6722) - (var_xg2 * (var_xg2_dn8 / (2.0 * assign6530_e6721)))) / (assign6530_e6722 * assign6530_e6722)), (((var_xg2_dn9 * assign6530_e6722) - (var_xg2 * (var_xg2_dn9 / (2.0 * assign6530_e6721)))) / (assign6530_e6722 * assign6530_e6722)), (((var_xg2_dn10 * assign6530_e6722) - (var_xg2 * (var_xg2_dn10 / (2.0 * assign6530_e6721)))) / (assign6530_e6722 * assign6530_e6722)), (((var_xg2_dn11 * assign6530_e6722) - (var_xg2 * (var_xg2_dn11 / (2.0 * assign6530_e6721)))) / (assign6530_e6722 * assign6530_e6722)), (((var_xg2_db0 * assign6530_e6722) - (var_xg2 * (var_xg2_db0 / (2.0 * assign6530_e6721)))) / (assign6530_e6722 * assign6530_e6722)), (((var_xg2_db1 * assign6530_e6722) - (var_xg2 * (var_xg2_db1 / (2.0 * assign6530_e6721)))) / (assign6530_e6722 * assign6530_e6722)),)
    } else {
        (var_xpwex, var_xpwex_dn0, var_xpwex_dn1, var_xpwex_dn2, var_xpwex_dn3, var_xpwex_dn4, var_xpwex_dn5, var_xpwex_dn6, var_xpwex_dn7, var_xpwex_dn8, var_xpwex_dn9, var_xpwex_dn10, var_xpwex_dn11, var_xpwex_db0, var_xpwex_db1,)
    }
};
        var_xpwex = assign6530_e6725;
        var_xpwex_dn0 = assign6530_e6725_d_n0;
        var_xpwex_dn1 = assign6530_e6725_d_n1;
        var_xpwex_dn2 = assign6530_e6725_d_n2;
        var_xpwex_dn3 = assign6530_e6725_d_n3;
        var_xpwex_dn4 = assign6530_e6725_d_n4;
        var_xpwex_dn5 = assign6530_e6725_d_n5;
        var_xpwex_dn6 = assign6530_e6725_d_n6;
        var_xpwex_dn7 = assign6530_e6725_d_n7;
        var_xpwex_dn8 = assign6530_e6725_d_n8;
        var_xpwex_dn9 = assign6530_e6725_d_n9;
        var_xpwex_dn10 = assign6530_e6725_d_n10;
        var_xpwex_dn11 = assign6530_e6725_d_n11;
        var_xpwex_db0 = assign6530_e6725_d_b0;
        var_xpwex_db1 = assign6530_e6725_d_b1;

        let (assign6540_e6747, assign6540_e6747_d_n0, assign6540_e6747_d_n1, assign6540_e6747_d_n2, assign6540_e6747_d_n3, assign6540_e6747_d_n4, assign6540_e6747_d_n5, assign6540_e6747_d_n6, assign6540_e6747_d_n7, assign6540_e6747_d_n8, assign6540_e6747_d_n9, assign6540_e6747_d_n10, assign6540_e6747_d_n11, assign6540_e6747_d_b0, assign6540_e6747_d_b1,) = {
    if ((var_guard117 != 0.0) && (var_guard118 != 0.0)) {
        let assign6540_e6731: f64 = (0.5 * p.p33);
        let assign6540_e6733: f64 = (assign6540_e6731 * var_taur_t);
        let assign6540_e6736: f64 = (var_qb0 * var_xnbex);
        let assign6540_e6739: f64 = (var_qepi0 * var_xpwex);
        let assign6540_e6740: f64 = (assign6540_e6736 + assign6540_e6739);
        let assign6540_e6741: f64 = (assign6540_e6733 * assign6540_e6740);
        let assign6540_e6744: f64 = (var_taub_t + var_tepi_t);
        let assign6540_e6745: f64 = (assign6540_e6741 / assign6540_e6744);
        (assign6540_e6745, ((assign6540_e6733 * ((var_qb0 * var_xnbex_dn0) + (var_qepi0 * var_xpwex_dn0))) / assign6540_e6744), ((assign6540_e6733 * ((var_qb0 * var_xnbex_dn1) + (var_qepi0 * var_xpwex_dn1))) / assign6540_e6744), ((assign6540_e6733 * ((var_qb0 * var_xnbex_dn2) + (var_qepi0 * var_xpwex_dn2))) / assign6540_e6744), ((assign6540_e6733 * ((var_qb0 * var_xnbex_dn3) + (var_qepi0 * var_xpwex_dn3))) / assign6540_e6744), ((assign6540_e6733 * ((var_qb0 * var_xnbex_dn4) + (var_qepi0 * var_xpwex_dn4))) / assign6540_e6744), ((assign6540_e6733 * ((var_qb0 * var_xnbex_dn5) + (var_qepi0 * var_xpwex_dn5))) / assign6540_e6744), ((assign6540_e6733 * ((var_qb0 * var_xnbex_dn6) + (var_qepi0 * var_xpwex_dn6))) / assign6540_e6744), ((assign6540_e6733 * ((var_qb0 * var_xnbex_dn7) + (var_qepi0 * var_xpwex_dn7))) / assign6540_e6744), ((assign6540_e6733 * ((var_qb0 * var_xnbex_dn8) + (var_qepi0 * var_xpwex_dn8))) / assign6540_e6744), ((assign6540_e6733 * ((var_qb0 * var_xnbex_dn9) + (var_qepi0 * var_xpwex_dn9))) / assign6540_e6744), ((assign6540_e6733 * ((var_qb0 * var_xnbex_dn10) + (var_qepi0 * var_xpwex_dn10))) / assign6540_e6744), ((assign6540_e6733 * ((var_qb0 * var_xnbex_dn11) + (var_qepi0 * var_xpwex_dn11))) / assign6540_e6744), ((assign6540_e6733 * ((var_qb0 * var_xnbex_db0) + (var_qepi0 * var_xpwex_db0))) / assign6540_e6744), ((assign6540_e6733 * ((var_qb0 * var_xnbex_db1) + (var_qepi0 * var_xpwex_db1))) / assign6540_e6744),)
    } else {
        (var_xqmex, var_xqmex_dn0, var_xqmex_dn1, var_xqmex_dn2, var_xqmex_dn3, var_xqmex_dn4, var_xqmex_dn5, var_xqmex_dn6, var_xqmex_dn7, var_xqmex_dn8, var_xqmex_dn9, var_xqmex_dn10, var_xqmex_dn11, var_xqmex_db0, var_xqmex_db1,)
    }
};
        var_xqmex = assign6540_e6747;
        var_xqmex_dn0 = assign6540_e6747_d_n0;
        var_xqmex_dn1 = assign6540_e6747_d_n1;
        var_xqmex_dn2 = assign6540_e6747_d_n2;
        var_xqmex_dn3 = assign6540_e6747_d_n3;
        var_xqmex_dn4 = assign6540_e6747_d_n4;
        var_xqmex_dn5 = assign6540_e6747_d_n5;
        var_xqmex_dn6 = assign6540_e6747_d_n6;
        var_xqmex_dn7 = assign6540_e6747_d_n7;
        var_xqmex_dn8 = assign6540_e6747_d_n8;
        var_xqmex_dn9 = assign6540_e6747_d_n9;
        var_xqmex_dn10 = assign6540_e6747_d_n10;
        var_xqmex_dn11 = assign6540_e6747_d_n11;
        var_xqmex_db0 = assign6540_e6747_d_b0;
        var_xqmex_db1 = assign6540_e6747_d_b1;

        let assign6550_e6750: f64 = (var_vbc3 - var_vdcex_t);
        let assign6550_e6752: f64 = (assign6550_e6750 * var_vtinv);
        let assign6550_e6754: f64 = if assign6550_e6752 < p.p147 { 1.0 } else { 0.0 };
        var_guard119 = assign6550_e6754;

        let (assign6560_e6768, assign6560_e6768_d_n0, assign6560_e6768_d_n1, assign6560_e6768_d_n2, assign6560_e6768_d_n3, assign6560_e6768_d_n4, assign6560_e6768_d_n5, assign6560_e6768_d_n6, assign6560_e6768_d_n7, assign6560_e6768_d_n8, assign6560_e6768_d_n9, assign6560_e6768_d_n10, assign6560_e6768_d_n11, assign6560_e6768_d_b0, assign6560_e6768_d_b1,) = {
    if (((var_guard117 != 0.0) && (var_guard118 == 0.0)) && (var_guard119 != 0.0)) {
        let assign6560_e6763: f64 = (var_vbc3 - var_vdcex_t);
        let assign6560_e6765: f64 = (assign6560_e6763 * var_vtinv);
        let assign6560_e6766: f64 = (assign6560_e6765).exp();
        (assign6560_e6766, (assign6560_e6766 * ((var_vbc3_dn0 - var_vdcex_t_dn0) * var_vtinv)), (assign6560_e6766 * ((var_vbc3_dn1 - var_vdcex_t_dn1) * var_vtinv)), (assign6560_e6766 * ((var_vbc3_dn2 - var_vdcex_t_dn2) * var_vtinv)), (assign6560_e6766 * ((var_vbc3_dn3 - var_vdcex_t_dn3) * var_vtinv)), (assign6560_e6766 * ((var_vbc3_dn4 - var_vdcex_t_dn4) * var_vtinv)), (assign6560_e6766 * ((var_vbc3_dn5 - var_vdcex_t_dn5) * var_vtinv)), (assign6560_e6766 * ((var_vbc3_dn6 - var_vdcex_t_dn6) * var_vtinv)), (assign6560_e6766 * ((var_vbc3_dn7 - var_vdcex_t_dn7) * var_vtinv)), (assign6560_e6766 * ((var_vbc3_dn8 - var_vdcex_t_dn8) * var_vtinv)), (assign6560_e6766 * ((var_vbc3_dn9 - var_vdcex_t_dn9) * var_vtinv)), (assign6560_e6766 * ((var_vbc3_dn10 - var_vdcex_t_dn10) * var_vtinv)), (assign6560_e6766 * ((var_vbc3_dn11 - var_vdcex_t_dn11) * var_vtinv)), (assign6560_e6766 * ((var_vbc3_db0 - var_vdcex_t_db0) * var_vtinv)), (assign6560_e6766 * ((var_vbc3_db1 - var_vdcex_t_db1) * var_vtinv)),)
    } else {
        (var_evbc3vdcex, var_evbc3vdcex_dn0, var_evbc3vdcex_dn1, var_evbc3vdcex_dn2, var_evbc3vdcex_dn3, var_evbc3vdcex_dn4, var_evbc3vdcex_dn5, var_evbc3vdcex_dn6, var_evbc3vdcex_dn7, var_evbc3vdcex_dn8, var_evbc3vdcex_dn9, var_evbc3vdcex_dn10, var_evbc3vdcex_dn11, var_evbc3vdcex_db0, var_evbc3vdcex_db1,)
    }
};
        var_evbc3vdcex = assign6560_e6768;
        var_evbc3vdcex_dn0 = assign6560_e6768_d_n0;
        var_evbc3vdcex_dn1 = assign6560_e6768_d_n1;
        var_evbc3vdcex_dn2 = assign6560_e6768_d_n2;
        var_evbc3vdcex_dn3 = assign6560_e6768_d_n3;
        var_evbc3vdcex_dn4 = assign6560_e6768_d_n4;
        var_evbc3vdcex_dn5 = assign6560_e6768_d_n5;
        var_evbc3vdcex_dn6 = assign6560_e6768_d_n6;
        var_evbc3vdcex_dn7 = assign6560_e6768_d_n7;
        var_evbc3vdcex_dn8 = assign6560_e6768_d_n8;
        var_evbc3vdcex_dn9 = assign6560_e6768_d_n9;
        var_evbc3vdcex_dn10 = assign6560_e6768_d_n10;
        var_evbc3vdcex_dn11 = assign6560_e6768_d_n11;
        var_evbc3vdcex_db0 = assign6560_e6768_d_b0;
        var_evbc3vdcex_db1 = assign6560_e6768_d_b1;

        let (assign6570_e6779,) = {
    if (((var_guard117 != 0.0) && (var_guard118 == 0.0)) && (var_guard119 == 0.0)) {
        let assign6570_e6777: f64 = (p.p147).exp();
        (assign6570_e6777,)
    } else {
        (var_expl,)
    }
};
        var_expl = assign6570_e6779;

        let (assign6580_e6799, assign6580_e6799_d_n0, assign6580_e6799_d_n1, assign6580_e6799_d_n2, assign6580_e6799_d_n3, assign6580_e6799_d_n4, assign6580_e6799_d_n5, assign6580_e6799_d_n6, assign6580_e6799_d_n7, assign6580_e6799_d_n8, assign6580_e6799_d_n9, assign6580_e6799_d_n10, assign6580_e6799_d_n11, assign6580_e6799_d_b0, assign6580_e6799_d_b1,) = {
    if (((var_guard117 != 0.0) && (var_guard118 == 0.0)) && (var_guard119 == 0.0)) {
        let assign6580_e6791: f64 = (var_vbc3 - var_vdcex_t);
        let assign6580_e6793: f64 = (assign6580_e6791 * var_vtinv);
        let assign6580_e6795: f64 = (assign6580_e6793 - p.p147);
        let assign6580_e6796: f64 = (1.0 + assign6580_e6795);
        let assign6580_e6797: f64 = (var_expl * assign6580_e6796);
        (assign6580_e6797, (var_expl * ((var_vbc3_dn0 - var_vdcex_t_dn0) * var_vtinv)), (var_expl * ((var_vbc3_dn1 - var_vdcex_t_dn1) * var_vtinv)), (var_expl * ((var_vbc3_dn2 - var_vdcex_t_dn2) * var_vtinv)), (var_expl * ((var_vbc3_dn3 - var_vdcex_t_dn3) * var_vtinv)), (var_expl * ((var_vbc3_dn4 - var_vdcex_t_dn4) * var_vtinv)), (var_expl * ((var_vbc3_dn5 - var_vdcex_t_dn5) * var_vtinv)), (var_expl * ((var_vbc3_dn6 - var_vdcex_t_dn6) * var_vtinv)), (var_expl * ((var_vbc3_dn7 - var_vdcex_t_dn7) * var_vtinv)), (var_expl * ((var_vbc3_dn8 - var_vdcex_t_dn8) * var_vtinv)), (var_expl * ((var_vbc3_dn9 - var_vdcex_t_dn9) * var_vtinv)), (var_expl * ((var_vbc3_dn10 - var_vdcex_t_dn10) * var_vtinv)), (var_expl * ((var_vbc3_dn11 - var_vdcex_t_dn11) * var_vtinv)), (var_expl * ((var_vbc3_db0 - var_vdcex_t_db0) * var_vtinv)), (var_expl * ((var_vbc3_db1 - var_vdcex_t_db1) * var_vtinv)),)
    } else {
        (var_evbc3vdcex, var_evbc3vdcex_dn0, var_evbc3vdcex_dn1, var_evbc3vdcex_dn2, var_evbc3vdcex_dn3, var_evbc3vdcex_dn4, var_evbc3vdcex_dn5, var_evbc3vdcex_dn6, var_evbc3vdcex_dn7, var_evbc3vdcex_dn8, var_evbc3vdcex_dn9, var_evbc3vdcex_dn10, var_evbc3vdcex_dn11, var_evbc3vdcex_db0, var_evbc3vdcex_db1,)
    }
};
        var_evbc3vdcex = assign6580_e6799;
        var_evbc3vdcex_dn0 = assign6580_e6799_d_n0;
        var_evbc3vdcex_dn1 = assign6580_e6799_d_n1;
        var_evbc3vdcex_dn2 = assign6580_e6799_d_n2;
        var_evbc3vdcex_dn3 = assign6580_e6799_d_n3;
        var_evbc3vdcex_dn4 = assign6580_e6799_d_n4;
        var_evbc3vdcex_dn5 = assign6580_e6799_d_n5;
        var_evbc3vdcex_dn6 = assign6580_e6799_d_n6;
        var_evbc3vdcex_dn7 = assign6580_e6799_d_n7;
        var_evbc3vdcex_dn8 = assign6580_e6799_d_n8;
        var_evbc3vdcex_dn9 = assign6580_e6799_d_n9;
        var_evbc3vdcex_dn10 = assign6580_e6799_d_n10;
        var_evbc3vdcex_dn11 = assign6580_e6799_d_n11;
        var_evbc3vdcex_db0 = assign6580_e6799_d_b0;
        var_evbc3vdcex_db1 = assign6580_e6799_d_b1;

        let (assign6590_e6823, assign6590_e6823_d_n0, assign6590_e6823_d_n1, assign6590_e6823_d_n2, assign6590_e6823_d_n3, assign6590_e6823_d_n4, assign6590_e6823_d_n5, assign6590_e6823_d_n6, assign6590_e6823_d_n7, assign6590_e6823_d_n8, assign6590_e6823_d_n9, assign6590_e6823_d_n10, assign6590_e6823_d_n11, assign6590_e6823_d_b0, assign6590_e6823_d_b1,) = {
    if ((var_guard117 != 0.0) && (var_guard118 == 0.0)) {
        let assign6590_e6806: f64 = (2.0 * p.p33);
        let assign6590_e6808: f64 = (assign6590_e6806 * var_ibx_t);
        let assign6590_e6810: f64 = (assign6590_e6808 * var_tauex_t);
        let assign6590_e6812: f64 = (assign6590_e6810 * var_evbc3);
        let assign6590_e6817: f64 = (4.0 * var_evbc3vdcex);
        let assign6590_e6818: f64 = (1.0 + assign6590_e6817);
        let assign6590_e6819: f64 = (assign6590_e6818).sqrt();
        let assign6590_e6820: f64 = (1.0 + assign6590_e6819);
        let assign6590_e6821: f64 = (assign6590_e6812 / assign6590_e6820);
        (assign6590_e6821, ((((assign6590_e6810 * var_evbc3_dn0) * assign6590_e6820) - (assign6590_e6812 * ((4.0 * var_evbc3vdcex_dn0) / (2.0 * assign6590_e6819)))) / (assign6590_e6820 * assign6590_e6820)), ((((assign6590_e6810 * var_evbc3_dn1) * assign6590_e6820) - (assign6590_e6812 * ((4.0 * var_evbc3vdcex_dn1) / (2.0 * assign6590_e6819)))) / (assign6590_e6820 * assign6590_e6820)), ((((assign6590_e6810 * var_evbc3_dn2) * assign6590_e6820) - (assign6590_e6812 * ((4.0 * var_evbc3vdcex_dn2) / (2.0 * assign6590_e6819)))) / (assign6590_e6820 * assign6590_e6820)), ((((assign6590_e6810 * var_evbc3_dn3) * assign6590_e6820) - (assign6590_e6812 * ((4.0 * var_evbc3vdcex_dn3) / (2.0 * assign6590_e6819)))) / (assign6590_e6820 * assign6590_e6820)), ((((assign6590_e6810 * var_evbc3_dn4) * assign6590_e6820) - (assign6590_e6812 * ((4.0 * var_evbc3vdcex_dn4) / (2.0 * assign6590_e6819)))) / (assign6590_e6820 * assign6590_e6820)), ((((assign6590_e6810 * var_evbc3_dn5) * assign6590_e6820) - (assign6590_e6812 * ((4.0 * var_evbc3vdcex_dn5) / (2.0 * assign6590_e6819)))) / (assign6590_e6820 * assign6590_e6820)), ((((assign6590_e6810 * var_evbc3_dn6) * assign6590_e6820) - (assign6590_e6812 * ((4.0 * var_evbc3vdcex_dn6) / (2.0 * assign6590_e6819)))) / (assign6590_e6820 * assign6590_e6820)), ((((assign6590_e6810 * var_evbc3_dn7) * assign6590_e6820) - (assign6590_e6812 * ((4.0 * var_evbc3vdcex_dn7) / (2.0 * assign6590_e6819)))) / (assign6590_e6820 * assign6590_e6820)), ((((assign6590_e6810 * var_evbc3_dn8) * assign6590_e6820) - (assign6590_e6812 * ((4.0 * var_evbc3vdcex_dn8) / (2.0 * assign6590_e6819)))) / (assign6590_e6820 * assign6590_e6820)), ((((assign6590_e6810 * var_evbc3_dn9) * assign6590_e6820) - (assign6590_e6812 * ((4.0 * var_evbc3vdcex_dn9) / (2.0 * assign6590_e6819)))) / (assign6590_e6820 * assign6590_e6820)), ((((assign6590_e6810 * var_evbc3_dn10) * assign6590_e6820) - (assign6590_e6812 * ((4.0 * var_evbc3vdcex_dn10) / (2.0 * assign6590_e6819)))) / (assign6590_e6820 * assign6590_e6820)), ((((assign6590_e6810 * var_evbc3_dn11) * assign6590_e6820) - (assign6590_e6812 * ((4.0 * var_evbc3vdcex_dn11) / (2.0 * assign6590_e6819)))) / (assign6590_e6820 * assign6590_e6820)), ((((assign6590_e6810 * var_evbc3_db0) * assign6590_e6820) - (assign6590_e6812 * ((4.0 * var_evbc3vdcex_db0) / (2.0 * assign6590_e6819)))) / (assign6590_e6820 * assign6590_e6820)), ((((assign6590_e6810 * var_evbc3_db1) * assign6590_e6820) - (assign6590_e6812 * ((4.0 * var_evbc3vdcex_db1) / (2.0 * assign6590_e6819)))) / (assign6590_e6820 * assign6590_e6820)),)
    } else {
        (var_xqmex, var_xqmex_dn0, var_xqmex_dn1, var_xqmex_dn2, var_xqmex_dn3, var_xqmex_dn4, var_xqmex_dn5, var_xqmex_dn6, var_xqmex_dn7, var_xqmex_dn8, var_xqmex_dn9, var_xqmex_dn10, var_xqmex_dn11, var_xqmex_db0, var_xqmex_db1,)
    }
};
        var_xqmex = assign6590_e6823;
        var_xqmex_dn0 = assign6590_e6823_d_n0;
        var_xqmex_dn1 = assign6590_e6823_d_n1;
        var_xqmex_dn2 = assign6590_e6823_d_n2;
        var_xqmex_dn3 = assign6590_e6823_d_n3;
        var_xqmex_dn4 = assign6590_e6823_d_n4;
        var_xqmex_dn5 = assign6590_e6823_d_n5;
        var_xqmex_dn6 = assign6590_e6823_d_n6;
        var_xqmex_dn7 = assign6590_e6823_d_n7;
        var_xqmex_dn8 = assign6590_e6823_d_n8;
        var_xqmex_dn9 = assign6590_e6823_d_n9;
        var_xqmex_dn10 = assign6590_e6823_d_n10;
        var_xqmex_dn11 = assign6590_e6823_d_n11;
        var_xqmex_db0 = assign6590_e6823_d_b0;
        var_xqmex_db1 = assign6590_e6823_d_b1;

        let (assign6600_e6829, assign6600_e6829_d_n0, assign6600_e6829_d_n1, assign6600_e6829_d_n2, assign6600_e6829_d_n3, assign6600_e6829_d_n4, assign6600_e6829_d_n5, assign6600_e6829_d_n6, assign6600_e6829_d_n7, assign6600_e6829_d_n8, assign6600_e6829_d_n9, assign6600_e6829_d_n10, assign6600_e6829_d_n11, assign6600_e6829_d_b0, assign6600_e6829_d_b1,) = {
    if (var_guard117 != 0.0) {
        let assign6600_e6827: f64 = (var_fex * var_xqmex);
        (assign6600_e6827, ((var_fex_dn0 * var_xqmex) + (var_fex * var_xqmex_dn0)), ((var_fex_dn1 * var_xqmex) + (var_fex * var_xqmex_dn1)), ((var_fex_dn2 * var_xqmex) + (var_fex * var_xqmex_dn2)), ((var_fex_dn3 * var_xqmex) + (var_fex * var_xqmex_dn3)), ((var_fex_dn4 * var_xqmex) + (var_fex * var_xqmex_dn4)), ((var_fex_dn5 * var_xqmex) + (var_fex * var_xqmex_dn5)), ((var_fex_dn6 * var_xqmex) + (var_fex * var_xqmex_dn6)), ((var_fex_dn7 * var_xqmex) + (var_fex * var_xqmex_dn7)), ((var_fex_dn8 * var_xqmex) + (var_fex * var_xqmex_dn8)), ((var_fex_dn9 * var_xqmex) + (var_fex * var_xqmex_dn9)), ((var_fex_dn10 * var_xqmex) + (var_fex * var_xqmex_dn10)), ((var_fex_dn11 * var_xqmex) + (var_fex * var_xqmex_dn11)), ((var_fex_db0 * var_xqmex) + (var_fex * var_xqmex_db0)), ((var_fex_db1 * var_xqmex) + (var_fex * var_xqmex_db1)),)
    } else {
        (var_xqex, var_xqex_dn0, var_xqex_dn1, var_xqex_dn2, var_xqex_dn3, var_xqex_dn4, var_xqex_dn5, var_xqex_dn6, var_xqex_dn7, var_xqex_dn8, var_xqex_dn9, var_xqex_dn10, var_xqex_dn11, var_xqex_db0, var_xqex_db1,)
    }
};
        var_xqex = assign6600_e6829;
        var_xqex_dn0 = assign6600_e6829_d_n0;
        var_xqex_dn1 = assign6600_e6829_d_n1;
        var_xqex_dn2 = assign6600_e6829_d_n2;
        var_xqex_dn3 = assign6600_e6829_d_n3;
        var_xqex_dn4 = assign6600_e6829_d_n4;
        var_xqex_dn5 = assign6600_e6829_d_n5;
        var_xqex_dn6 = assign6600_e6829_d_n6;
        var_xqex_dn7 = assign6600_e6829_d_n7;
        var_xqex_dn8 = assign6600_e6829_d_n8;
        var_xqex_dn9 = assign6600_e6829_d_n9;
        var_xqex_dn10 = assign6600_e6829_d_n10;
        var_xqex_dn11 = assign6600_e6829_d_n11;
        var_xqex_db0 = assign6600_e6829_d_b0;
        var_xqex_db1 = assign6600_e6829_d_b1;

        let assign6610_e6832: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        var_guard120 = assign6610_e6832;

        let (assign6620_e6845, assign6620_e6845_d_n0, assign6620_e6845_d_n1, assign6620_e6845_d_n2, assign6620_e6845_d_n3, assign6620_e6845_d_n4, assign6620_e6845_d_n5, assign6620_e6845_d_n6, assign6620_e6845_d_n7, assign6620_e6845_d_n8, assign6620_e6845_d_n9, assign6620_e6845_d_n10, assign6620_e6845_d_n11, assign6620_e6845_d_b0, assign6620_e6845_d_b1,) = {
    if (var_guard120 != 0.0) {
        let assign6620_e6837: f64 = (var_vje * var_inv_vde_t);
        let assign6620_e6838: f64 = (1.0 - assign6620_e6837);
        let assign6620_e6840: f64 = (-p.p67);
        let assign6620_e6841: f64 = (assign6620_e6838).powf(assign6620_e6840);
        let assign6620_e6843: f64 = (assign6620_e6841 - 3.0);
        (assign6620_e6843, if 0.0 == 0.0 && ((assign6620_e6840) as f64).is_finite() && ((assign6620_e6840) as f64).fract() == 0.0 { if assign6620_e6840 == 0.0 { 0.0 } else { (assign6620_e6840 * ((assign6620_e6838).powf(assign6620_e6840 - 1.0) * (-((var_vje_dn0 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn0))))) } } else { (assign6620_e6841 * (assign6620_e6840 * ((-((var_vje_dn0 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn0))) / assign6620_e6838))) }, if 0.0 == 0.0 && ((assign6620_e6840) as f64).is_finite() && ((assign6620_e6840) as f64).fract() == 0.0 { if assign6620_e6840 == 0.0 { 0.0 } else { (assign6620_e6840 * ((assign6620_e6838).powf(assign6620_e6840 - 1.0) * (-((var_vje_dn1 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn1))))) } } else { (assign6620_e6841 * (assign6620_e6840 * ((-((var_vje_dn1 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn1))) / assign6620_e6838))) }, if 0.0 == 0.0 && ((assign6620_e6840) as f64).is_finite() && ((assign6620_e6840) as f64).fract() == 0.0 { if assign6620_e6840 == 0.0 { 0.0 } else { (assign6620_e6840 * ((assign6620_e6838).powf(assign6620_e6840 - 1.0) * (-((var_vje_dn2 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn2))))) } } else { (assign6620_e6841 * (assign6620_e6840 * ((-((var_vje_dn2 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn2))) / assign6620_e6838))) }, if 0.0 == 0.0 && ((assign6620_e6840) as f64).is_finite() && ((assign6620_e6840) as f64).fract() == 0.0 { if assign6620_e6840 == 0.0 { 0.0 } else { (assign6620_e6840 * ((assign6620_e6838).powf(assign6620_e6840 - 1.0) * (-((var_vje_dn3 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn3))))) } } else { (assign6620_e6841 * (assign6620_e6840 * ((-((var_vje_dn3 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn3))) / assign6620_e6838))) }, if 0.0 == 0.0 && ((assign6620_e6840) as f64).is_finite() && ((assign6620_e6840) as f64).fract() == 0.0 { if assign6620_e6840 == 0.0 { 0.0 } else { (assign6620_e6840 * ((assign6620_e6838).powf(assign6620_e6840 - 1.0) * (-((var_vje_dn4 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn4))))) } } else { (assign6620_e6841 * (assign6620_e6840 * ((-((var_vje_dn4 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn4))) / assign6620_e6838))) }, if 0.0 == 0.0 && ((assign6620_e6840) as f64).is_finite() && ((assign6620_e6840) as f64).fract() == 0.0 { if assign6620_e6840 == 0.0 { 0.0 } else { (assign6620_e6840 * ((assign6620_e6838).powf(assign6620_e6840 - 1.0) * (-((var_vje_dn5 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn5))))) } } else { (assign6620_e6841 * (assign6620_e6840 * ((-((var_vje_dn5 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn5))) / assign6620_e6838))) }, if 0.0 == 0.0 && ((assign6620_e6840) as f64).is_finite() && ((assign6620_e6840) as f64).fract() == 0.0 { if assign6620_e6840 == 0.0 { 0.0 } else { (assign6620_e6840 * ((assign6620_e6838).powf(assign6620_e6840 - 1.0) * (-((var_vje_dn6 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn6))))) } } else { (assign6620_e6841 * (assign6620_e6840 * ((-((var_vje_dn6 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn6))) / assign6620_e6838))) }, if 0.0 == 0.0 && ((assign6620_e6840) as f64).is_finite() && ((assign6620_e6840) as f64).fract() == 0.0 { if assign6620_e6840 == 0.0 { 0.0 } else { (assign6620_e6840 * ((assign6620_e6838).powf(assign6620_e6840 - 1.0) * (-((var_vje_dn7 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn7))))) } } else { (assign6620_e6841 * (assign6620_e6840 * ((-((var_vje_dn7 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn7))) / assign6620_e6838))) }, if 0.0 == 0.0 && ((assign6620_e6840) as f64).is_finite() && ((assign6620_e6840) as f64).fract() == 0.0 { if assign6620_e6840 == 0.0 { 0.0 } else { (assign6620_e6840 * ((assign6620_e6838).powf(assign6620_e6840 - 1.0) * (-((var_vje_dn8 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn8))))) } } else { (assign6620_e6841 * (assign6620_e6840 * ((-((var_vje_dn8 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn8))) / assign6620_e6838))) }, if 0.0 == 0.0 && ((assign6620_e6840) as f64).is_finite() && ((assign6620_e6840) as f64).fract() == 0.0 { if assign6620_e6840 == 0.0 { 0.0 } else { (assign6620_e6840 * ((assign6620_e6838).powf(assign6620_e6840 - 1.0) * (-((var_vje_dn9 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn9))))) } } else { (assign6620_e6841 * (assign6620_e6840 * ((-((var_vje_dn9 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn9))) / assign6620_e6838))) }, if 0.0 == 0.0 && ((assign6620_e6840) as f64).is_finite() && ((assign6620_e6840) as f64).fract() == 0.0 { if assign6620_e6840 == 0.0 { 0.0 } else { (assign6620_e6840 * ((assign6620_e6838).powf(assign6620_e6840 - 1.0) * (-((var_vje_dn10 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn10))))) } } else { (assign6620_e6841 * (assign6620_e6840 * ((-((var_vje_dn10 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn10))) / assign6620_e6838))) }, if 0.0 == 0.0 && ((assign6620_e6840) as f64).is_finite() && ((assign6620_e6840) as f64).fract() == 0.0 { if assign6620_e6840 == 0.0 { 0.0 } else { (assign6620_e6840 * ((assign6620_e6838).powf(assign6620_e6840 - 1.0) * (-((var_vje_dn11 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn11))))) } } else { (assign6620_e6841 * (assign6620_e6840 * ((-((var_vje_dn11 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn11))) / assign6620_e6838))) }, if 0.0 == 0.0 && ((assign6620_e6840) as f64).is_finite() && ((assign6620_e6840) as f64).fract() == 0.0 { if assign6620_e6840 == 0.0 { 0.0 } else { (assign6620_e6840 * ((assign6620_e6838).powf(assign6620_e6840 - 1.0) * (-((var_vje_db0 * var_inv_vde_t) + (var_vje * var_inv_vde_t_db0))))) } } else { (assign6620_e6841 * (assign6620_e6840 * ((-((var_vje_db0 * var_inv_vde_t) + (var_vje * var_inv_vde_t_db0))) / assign6620_e6838))) }, if 0.0 == 0.0 && ((assign6620_e6840) as f64).is_finite() && ((assign6620_e6840) as f64).fract() == 0.0 { if assign6620_e6840 == 0.0 { 0.0 } else { (assign6620_e6840 * ((assign6620_e6838).powf(assign6620_e6840 - 1.0) * (-((var_vje_db1 * var_inv_vde_t) + (var_vje * var_inv_vde_t_db1))))) } } else { (assign6620_e6841 * (assign6620_e6840 * ((-((var_vje_db1 * var_inv_vde_t) + (var_vje * var_inv_vde_t_db1))) / assign6620_e6838))) },)
    } else {
        (var_dvtevje, var_dvtevje_dn0, var_dvtevje_dn1, var_dvtevje_dn2, var_dvtevje_dn3, var_dvtevje_dn4, var_dvtevje_dn5, var_dvtevje_dn6, var_dvtevje_dn7, var_dvtevje_dn8, var_dvtevje_dn9, var_dvtevje_dn10, var_dvtevje_dn11, var_dvtevje_db0, var_dvtevje_db1,)
    }
};
        var_dvtevje = assign6620_e6845;
        var_dvtevje_dn0 = assign6620_e6845_d_n0;
        var_dvtevje_dn1 = assign6620_e6845_d_n1;
        var_dvtevje_dn2 = assign6620_e6845_d_n2;
        var_dvtevje_dn3 = assign6620_e6845_d_n3;
        var_dvtevje_dn4 = assign6620_e6845_d_n4;
        var_dvtevje_dn5 = assign6620_e6845_d_n5;
        var_dvtevje_dn6 = assign6620_e6845_d_n6;
        var_dvtevje_dn7 = assign6620_e6845_d_n7;
        var_dvtevje_dn8 = assign6620_e6845_d_n8;
        var_dvtevje_dn9 = assign6620_e6845_d_n9;
        var_dvtevje_dn10 = assign6620_e6845_d_n10;
        var_dvtevje_dn11 = assign6620_e6845_d_n11;
        var_dvtevje_db0 = assign6620_e6845_d_b0;
        var_dvtevje_db1 = assign6620_e6845_d_b1;

        let (assign6630_e6853, assign6630_e6853_d_n0, assign6630_e6853_d_n1, assign6630_e6853_d_n2, assign6630_e6853_d_n3, assign6630_e6853_d_n4, assign6630_e6853_d_n5, assign6630_e6853_d_n6, assign6630_e6853_d_n7, assign6630_e6853_d_n8, assign6630_e6853_d_n9, assign6630_e6853_d_n10, assign6630_e6853_d_n11, assign6630_e6853_d_b0, assign6630_e6853_d_b1,) = {
    if (var_guard120 != 0.0) {
        let assign6630_e6849: f64 = (var_vb2e1 - var_vfe);
        let assign6630_e6851: f64 = (assign6630_e6849 / var_a_vde);
        (assign6630_e6851, ((((var_vb2e1_dn0 - var_vfe_dn0) * var_a_vde) - (assign6630_e6849 * var_a_vde_dn0)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn1 - var_vfe_dn1) * var_a_vde) - (assign6630_e6849 * var_a_vde_dn1)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn2 - var_vfe_dn2) * var_a_vde) - (assign6630_e6849 * var_a_vde_dn2)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn3 - var_vfe_dn3) * var_a_vde) - (assign6630_e6849 * var_a_vde_dn3)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn4 - var_vfe_dn4) * var_a_vde) - (assign6630_e6849 * var_a_vde_dn4)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn5 - var_vfe_dn5) * var_a_vde) - (assign6630_e6849 * var_a_vde_dn5)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn6 - var_vfe_dn6) * var_a_vde) - (assign6630_e6849 * var_a_vde_dn6)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn7 - var_vfe_dn7) * var_a_vde) - (assign6630_e6849 * var_a_vde_dn7)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn8 - var_vfe_dn8) * var_a_vde) - (assign6630_e6849 * var_a_vde_dn8)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn9 - var_vfe_dn9) * var_a_vde) - (assign6630_e6849 * var_a_vde_dn9)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn10 - var_vfe_dn10) * var_a_vde) - (assign6630_e6849 * var_a_vde_dn10)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn11 - var_vfe_dn11) * var_a_vde) - (assign6630_e6849 * var_a_vde_dn11)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_db0 - var_vfe_db0) * var_a_vde) - (assign6630_e6849 * var_a_vde_db0)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_db1 - var_vfe_db1) * var_a_vde) - (assign6630_e6849 * var_a_vde_db1)) / (var_a_vde * var_a_vde)),)
    } else {
        (var_vb2e1vfe, var_vb2e1vfe_dn0, var_vb2e1vfe_dn1, var_vb2e1vfe_dn2, var_vb2e1vfe_dn3, var_vb2e1vfe_dn4, var_vb2e1vfe_dn5, var_vb2e1vfe_dn6, var_vb2e1vfe_dn7, var_vb2e1vfe_dn8, var_vb2e1vfe_dn9, var_vb2e1vfe_dn10, var_vb2e1vfe_dn11, var_vb2e1vfe_db0, var_vb2e1vfe_db1,)
    }
};
        var_vb2e1vfe = assign6630_e6853;
        var_vb2e1vfe_dn0 = assign6630_e6853_d_n0;
        var_vb2e1vfe_dn1 = assign6630_e6853_d_n1;
        var_vb2e1vfe_dn2 = assign6630_e6853_d_n2;
        var_vb2e1vfe_dn3 = assign6630_e6853_d_n3;
        var_vb2e1vfe_dn4 = assign6630_e6853_d_n4;
        var_vb2e1vfe_dn5 = assign6630_e6853_d_n5;
        var_vb2e1vfe_dn6 = assign6630_e6853_d_n6;
        var_vb2e1vfe_dn7 = assign6630_e6853_d_n7;
        var_vb2e1vfe_dn8 = assign6630_e6853_d_n8;
        var_vb2e1vfe_dn9 = assign6630_e6853_d_n9;
        var_vb2e1vfe_dn10 = assign6630_e6853_d_n10;
        var_vb2e1vfe_dn11 = assign6630_e6853_d_n11;
        var_vb2e1vfe_db0 = assign6630_e6853_d_b0;
        var_vb2e1vfe_db1 = assign6630_e6853_d_b1;

        let assign6640_e6856: f64 = if var_vb2e1vfe < 0.0 { 1.0 } else { 0.0 };
        var_guard121 = assign6640_e6856;

        let (assign6650_e6867, assign6650_e6867_d_n0, assign6650_e6867_d_n1, assign6650_e6867_d_n2, assign6650_e6867_d_n3, assign6650_e6867_d_n4, assign6650_e6867_d_n5, assign6650_e6867_d_n6, assign6650_e6867_d_n7, assign6650_e6867_d_n8, assign6650_e6867_d_n9, assign6650_e6867_d_n10, assign6650_e6867_d_n11, assign6650_e6867_d_b0, assign6650_e6867_d_b1,) = {
    if ((var_guard120 != 0.0) && (var_guard121 != 0.0)) {
        let assign6650_e6863: f64 = (var_vb2e1vfe).exp();
        let assign6650_e6864: f64 = (1.0 + assign6650_e6863);
        let assign6650_e6865: f64 = (1.0 / assign6650_e6864);
        (assign6650_e6865, (-((assign6650_e6863 * var_vb2e1vfe_dn0) / (assign6650_e6864 * assign6650_e6864))), (-((assign6650_e6863 * var_vb2e1vfe_dn1) / (assign6650_e6864 * assign6650_e6864))), (-((assign6650_e6863 * var_vb2e1vfe_dn2) / (assign6650_e6864 * assign6650_e6864))), (-((assign6650_e6863 * var_vb2e1vfe_dn3) / (assign6650_e6864 * assign6650_e6864))), (-((assign6650_e6863 * var_vb2e1vfe_dn4) / (assign6650_e6864 * assign6650_e6864))), (-((assign6650_e6863 * var_vb2e1vfe_dn5) / (assign6650_e6864 * assign6650_e6864))), (-((assign6650_e6863 * var_vb2e1vfe_dn6) / (assign6650_e6864 * assign6650_e6864))), (-((assign6650_e6863 * var_vb2e1vfe_dn7) / (assign6650_e6864 * assign6650_e6864))), (-((assign6650_e6863 * var_vb2e1vfe_dn8) / (assign6650_e6864 * assign6650_e6864))), (-((assign6650_e6863 * var_vb2e1vfe_dn9) / (assign6650_e6864 * assign6650_e6864))), (-((assign6650_e6863 * var_vb2e1vfe_dn10) / (assign6650_e6864 * assign6650_e6864))), (-((assign6650_e6863 * var_vb2e1vfe_dn11) / (assign6650_e6864 * assign6650_e6864))), (-((assign6650_e6863 * var_vb2e1vfe_db0) / (assign6650_e6864 * assign6650_e6864))), (-((assign6650_e6863 * var_vb2e1vfe_db1) / (assign6650_e6864 * assign6650_e6864))),)
    } else {
        (var_dvjevb2e1, var_dvjevb2e1_dn0, var_dvjevb2e1_dn1, var_dvjevb2e1_dn2, var_dvjevb2e1_dn3, var_dvjevb2e1_dn4, var_dvjevb2e1_dn5, var_dvjevb2e1_dn6, var_dvjevb2e1_dn7, var_dvjevb2e1_dn8, var_dvjevb2e1_dn9, var_dvjevb2e1_dn10, var_dvjevb2e1_dn11, var_dvjevb2e1_db0, var_dvjevb2e1_db1,)
    }
};
        var_dvjevb2e1 = assign6650_e6867;
        var_dvjevb2e1_dn0 = assign6650_e6867_d_n0;
        var_dvjevb2e1_dn1 = assign6650_e6867_d_n1;
        var_dvjevb2e1_dn2 = assign6650_e6867_d_n2;
        var_dvjevb2e1_dn3 = assign6650_e6867_d_n3;
        var_dvjevb2e1_dn4 = assign6650_e6867_d_n4;
        var_dvjevb2e1_dn5 = assign6650_e6867_d_n5;
        var_dvjevb2e1_dn6 = assign6650_e6867_d_n6;
        var_dvjevb2e1_dn7 = assign6650_e6867_d_n7;
        var_dvjevb2e1_dn8 = assign6650_e6867_d_n8;
        var_dvjevb2e1_dn9 = assign6650_e6867_d_n9;
        var_dvjevb2e1_dn10 = assign6650_e6867_d_n10;
        var_dvjevb2e1_dn11 = assign6650_e6867_d_n11;
        var_dvjevb2e1_db0 = assign6650_e6867_d_b0;
        var_dvjevb2e1_db1 = assign6650_e6867_d_b1;

        let (assign6660_e6882, assign6660_e6882_d_n0, assign6660_e6882_d_n1, assign6660_e6882_d_n2, assign6660_e6882_d_n3, assign6660_e6882_d_n4, assign6660_e6882_d_n5, assign6660_e6882_d_n6, assign6660_e6882_d_n7, assign6660_e6882_d_n8, assign6660_e6882_d_n9, assign6660_e6882_d_n10, assign6660_e6882_d_n11, assign6660_e6882_d_b0, assign6660_e6882_d_b1,) = {
    if ((var_guard120 != 0.0) && (var_guard121 == 0.0)) {
        let assign6660_e6873: f64 = (-var_vb2e1vfe);
        let assign6660_e6874: f64 = (assign6660_e6873).exp();
        let assign6660_e6877: f64 = (-var_vb2e1vfe);
        let assign6660_e6878: f64 = (assign6660_e6877).exp();
        let assign6660_e6879: f64 = (1.0 + assign6660_e6878);
        let assign6660_e6880: f64 = (assign6660_e6874 / assign6660_e6879);
        (assign6660_e6880, ((((assign6660_e6874 * (-var_vb2e1vfe_dn0)) * assign6660_e6879) - (assign6660_e6874 * (assign6660_e6878 * (-var_vb2e1vfe_dn0)))) / (assign6660_e6879 * assign6660_e6879)), ((((assign6660_e6874 * (-var_vb2e1vfe_dn1)) * assign6660_e6879) - (assign6660_e6874 * (assign6660_e6878 * (-var_vb2e1vfe_dn1)))) / (assign6660_e6879 * assign6660_e6879)), ((((assign6660_e6874 * (-var_vb2e1vfe_dn2)) * assign6660_e6879) - (assign6660_e6874 * (assign6660_e6878 * (-var_vb2e1vfe_dn2)))) / (assign6660_e6879 * assign6660_e6879)), ((((assign6660_e6874 * (-var_vb2e1vfe_dn3)) * assign6660_e6879) - (assign6660_e6874 * (assign6660_e6878 * (-var_vb2e1vfe_dn3)))) / (assign6660_e6879 * assign6660_e6879)), ((((assign6660_e6874 * (-var_vb2e1vfe_dn4)) * assign6660_e6879) - (assign6660_e6874 * (assign6660_e6878 * (-var_vb2e1vfe_dn4)))) / (assign6660_e6879 * assign6660_e6879)), ((((assign6660_e6874 * (-var_vb2e1vfe_dn5)) * assign6660_e6879) - (assign6660_e6874 * (assign6660_e6878 * (-var_vb2e1vfe_dn5)))) / (assign6660_e6879 * assign6660_e6879)), ((((assign6660_e6874 * (-var_vb2e1vfe_dn6)) * assign6660_e6879) - (assign6660_e6874 * (assign6660_e6878 * (-var_vb2e1vfe_dn6)))) / (assign6660_e6879 * assign6660_e6879)), ((((assign6660_e6874 * (-var_vb2e1vfe_dn7)) * assign6660_e6879) - (assign6660_e6874 * (assign6660_e6878 * (-var_vb2e1vfe_dn7)))) / (assign6660_e6879 * assign6660_e6879)), ((((assign6660_e6874 * (-var_vb2e1vfe_dn8)) * assign6660_e6879) - (assign6660_e6874 * (assign6660_e6878 * (-var_vb2e1vfe_dn8)))) / (assign6660_e6879 * assign6660_e6879)), ((((assign6660_e6874 * (-var_vb2e1vfe_dn9)) * assign6660_e6879) - (assign6660_e6874 * (assign6660_e6878 * (-var_vb2e1vfe_dn9)))) / (assign6660_e6879 * assign6660_e6879)), ((((assign6660_e6874 * (-var_vb2e1vfe_dn10)) * assign6660_e6879) - (assign6660_e6874 * (assign6660_e6878 * (-var_vb2e1vfe_dn10)))) / (assign6660_e6879 * assign6660_e6879)), ((((assign6660_e6874 * (-var_vb2e1vfe_dn11)) * assign6660_e6879) - (assign6660_e6874 * (assign6660_e6878 * (-var_vb2e1vfe_dn11)))) / (assign6660_e6879 * assign6660_e6879)), ((((assign6660_e6874 * (-var_vb2e1vfe_db0)) * assign6660_e6879) - (assign6660_e6874 * (assign6660_e6878 * (-var_vb2e1vfe_db0)))) / (assign6660_e6879 * assign6660_e6879)), ((((assign6660_e6874 * (-var_vb2e1vfe_db1)) * assign6660_e6879) - (assign6660_e6874 * (assign6660_e6878 * (-var_vb2e1vfe_db1)))) / (assign6660_e6879 * assign6660_e6879)),)
    } else {
        (var_dvjevb2e1, var_dvjevb2e1_dn0, var_dvjevb2e1_dn1, var_dvjevb2e1_dn2, var_dvjevb2e1_dn3, var_dvjevb2e1_dn4, var_dvjevb2e1_dn5, var_dvjevb2e1_dn6, var_dvjevb2e1_dn7, var_dvjevb2e1_dn8, var_dvjevb2e1_dn9, var_dvjevb2e1_dn10, var_dvjevb2e1_dn11, var_dvjevb2e1_db0, var_dvjevb2e1_db1,)
    }
};
        var_dvjevb2e1 = assign6660_e6882;
        var_dvjevb2e1_dn0 = assign6660_e6882_d_n0;
        var_dvjevb2e1_dn1 = assign6660_e6882_d_n1;
        var_dvjevb2e1_dn2 = assign6660_e6882_d_n2;
        var_dvjevb2e1_dn3 = assign6660_e6882_d_n3;
        var_dvjevb2e1_dn4 = assign6660_e6882_d_n4;
        var_dvjevb2e1_dn5 = assign6660_e6882_d_n5;
        var_dvjevb2e1_dn6 = assign6660_e6882_d_n6;
        var_dvjevb2e1_dn7 = assign6660_e6882_d_n7;
        var_dvjevb2e1_dn8 = assign6660_e6882_d_n8;
        var_dvjevb2e1_dn9 = assign6660_e6882_d_n9;
        var_dvjevb2e1_dn10 = assign6660_e6882_d_n10;
        var_dvjevb2e1_dn11 = assign6660_e6882_d_n11;
        var_dvjevb2e1_db0 = assign6660_e6882_d_b0;
        var_dvjevb2e1_db1 = assign6660_e6882_d_b1;

        let (assign6670_e6890, assign6670_e6890_d_n0, assign6670_e6890_d_n1, assign6670_e6890_d_n2, assign6670_e6890_d_n3, assign6670_e6890_d_n4, assign6670_e6890_d_n5, assign6670_e6890_d_n6, assign6670_e6890_d_n7, assign6670_e6890_d_n8, assign6670_e6890_d_n9, assign6670_e6890_d_n10, assign6670_e6890_d_n11, assign6670_e6890_d_b0, assign6670_e6890_d_b1,) = {
    if (var_guard120 != 0.0) {
        let assign6670_e6886: f64 = (var_dvtevje * var_dvjevb2e1);
        let assign6670_e6888: f64 = (assign6670_e6886 + 3.0);
        (assign6670_e6888, ((var_dvtevje_dn0 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn0)), ((var_dvtevje_dn1 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn1)), ((var_dvtevje_dn2 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn2)), ((var_dvtevje_dn3 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn3)), ((var_dvtevje_dn4 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn4)), ((var_dvtevje_dn5 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn5)), ((var_dvtevje_dn6 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn6)), ((var_dvtevje_dn7 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn7)), ((var_dvtevje_dn8 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn8)), ((var_dvtevje_dn9 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn9)), ((var_dvtevje_dn10 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn10)), ((var_dvtevje_dn11 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn11)), ((var_dvtevje_db0 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_db0)), ((var_dvtevje_db1 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_db1)),)
    } else {
        (var_dvtevb2e1, var_dvtevb2e1_dn0, var_dvtevb2e1_dn1, var_dvtevb2e1_dn2, var_dvtevb2e1_dn3, var_dvtevb2e1_dn4, var_dvtevb2e1_dn5, var_dvtevb2e1_dn6, var_dvtevb2e1_dn7, var_dvtevb2e1_dn8, var_dvtevb2e1_dn9, var_dvtevb2e1_dn10, var_dvtevb2e1_dn11, var_dvtevb2e1_db0, var_dvtevb2e1_db1,)
    }
};
        var_dvtevb2e1 = assign6670_e6890;
        var_dvtevb2e1_dn0 = assign6670_e6890_d_n0;
        var_dvtevb2e1_dn1 = assign6670_e6890_d_n1;
        var_dvtevb2e1_dn2 = assign6670_e6890_d_n2;
        var_dvtevb2e1_dn3 = assign6670_e6890_d_n3;
        var_dvtevb2e1_dn4 = assign6670_e6890_d_n4;
        var_dvtevb2e1_dn5 = assign6670_e6890_d_n5;
        var_dvtevb2e1_dn6 = assign6670_e6890_d_n6;
        var_dvtevb2e1_dn7 = assign6670_e6890_d_n7;
        var_dvtevb2e1_dn8 = assign6670_e6890_d_n8;
        var_dvtevb2e1_dn9 = assign6670_e6890_d_n9;
        var_dvtevb2e1_dn10 = assign6670_e6890_d_n10;
        var_dvtevb2e1_dn11 = assign6670_e6890_d_n11;
        var_dvtevb2e1_db0 = assign6670_e6890_d_b0;
        var_dvtevb2e1_db1 = assign6670_e6890_d_b1;


        *var_dvjevb2e1_slot = var_dvjevb2e1;
        *var_dvjevb2e1_db0_slot = var_dvjevb2e1_db0;
        *var_dvjevb2e1_db1_slot = var_dvjevb2e1_db1;
        *var_dvjevb2e1_dn0_slot = var_dvjevb2e1_dn0;
        *var_dvjevb2e1_dn1_slot = var_dvjevb2e1_dn1;
        *var_dvjevb2e1_dn10_slot = var_dvjevb2e1_dn10;
        *var_dvjevb2e1_dn11_slot = var_dvjevb2e1_dn11;
        *var_dvjevb2e1_dn2_slot = var_dvjevb2e1_dn2;
        *var_dvjevb2e1_dn3_slot = var_dvjevb2e1_dn3;
        *var_dvjevb2e1_dn4_slot = var_dvjevb2e1_dn4;
        *var_dvjevb2e1_dn5_slot = var_dvjevb2e1_dn5;
        *var_dvjevb2e1_dn6_slot = var_dvjevb2e1_dn6;
        *var_dvjevb2e1_dn7_slot = var_dvjevb2e1_dn7;
        *var_dvjevb2e1_dn8_slot = var_dvjevb2e1_dn8;
        *var_dvjevb2e1_dn9_slot = var_dvjevb2e1_dn9;
        *var_dvtevb2e1_slot = var_dvtevb2e1;
        *var_dvtevb2e1_db0_slot = var_dvtevb2e1_db0;
        *var_dvtevb2e1_db1_slot = var_dvtevb2e1_db1;
        *var_dvtevb2e1_dn0_slot = var_dvtevb2e1_dn0;
        *var_dvtevb2e1_dn1_slot = var_dvtevb2e1_dn1;
        *var_dvtevb2e1_dn10_slot = var_dvtevb2e1_dn10;
        *var_dvtevb2e1_dn11_slot = var_dvtevb2e1_dn11;
        *var_dvtevb2e1_dn2_slot = var_dvtevb2e1_dn2;
        *var_dvtevb2e1_dn3_slot = var_dvtevb2e1_dn3;
        *var_dvtevb2e1_dn4_slot = var_dvtevb2e1_dn4;
        *var_dvtevb2e1_dn5_slot = var_dvtevb2e1_dn5;
        *var_dvtevb2e1_dn6_slot = var_dvtevb2e1_dn6;
        *var_dvtevb2e1_dn7_slot = var_dvtevb2e1_dn7;
        *var_dvtevb2e1_dn8_slot = var_dvtevb2e1_dn8;
        *var_dvtevb2e1_dn9_slot = var_dvtevb2e1_dn9;
        *var_dvtevje_slot = var_dvtevje;
        *var_dvtevje_db0_slot = var_dvtevje_db0;
        *var_dvtevje_db1_slot = var_dvtevje_db1;
        *var_dvtevje_dn0_slot = var_dvtevje_dn0;
        *var_dvtevje_dn1_slot = var_dvtevje_dn1;
        *var_dvtevje_dn10_slot = var_dvtevje_dn10;
        *var_dvtevje_dn11_slot = var_dvtevje_dn11;
        *var_dvtevje_dn2_slot = var_dvtevje_dn2;
        *var_dvtevje_dn3_slot = var_dvtevje_dn3;
        *var_dvtevje_dn4_slot = var_dvtevje_dn4;
        *var_dvtevje_dn5_slot = var_dvtevje_dn5;
        *var_dvtevje_dn6_slot = var_dvtevje_dn6;
        *var_dvtevje_dn7_slot = var_dvtevje_dn7;
        *var_dvtevje_dn8_slot = var_dvtevje_dn8;
        *var_dvtevje_dn9_slot = var_dvtevje_dn9;
        *var_evb1c4vdcex_slot = var_evb1c4vdcex;
        *var_evb1c4vdcex_db0_slot = var_evb1c4vdcex_db0;
        *var_evb1c4vdcex_db1_slot = var_evb1c4vdcex_db1;
        *var_evb1c4vdcex_dn0_slot = var_evb1c4vdcex_dn0;
        *var_evb1c4vdcex_dn1_slot = var_evb1c4vdcex_dn1;
        *var_evb1c4vdcex_dn10_slot = var_evb1c4vdcex_dn10;
        *var_evb1c4vdcex_dn11_slot = var_evb1c4vdcex_dn11;
        *var_evb1c4vdcex_dn2_slot = var_evb1c4vdcex_dn2;
        *var_evb1c4vdcex_dn3_slot = var_evb1c4vdcex_dn3;
        *var_evb1c4vdcex_dn4_slot = var_evb1c4vdcex_dn4;
        *var_evb1c4vdcex_dn5_slot = var_evb1c4vdcex_dn5;
        *var_evb1c4vdcex_dn6_slot = var_evb1c4vdcex_dn6;
        *var_evb1c4vdcex_dn7_slot = var_evb1c4vdcex_dn7;
        *var_evb1c4vdcex_dn8_slot = var_evb1c4vdcex_dn8;
        *var_evb1c4vdcex_dn9_slot = var_evb1c4vdcex_dn9;
        *var_evbc3vdcex_slot = var_evbc3vdcex;
        *var_evbc3vdcex_db0_slot = var_evbc3vdcex_db0;
        *var_evbc3vdcex_db1_slot = var_evbc3vdcex_db1;
        *var_evbc3vdcex_dn0_slot = var_evbc3vdcex_dn0;
        *var_evbc3vdcex_dn1_slot = var_evbc3vdcex_dn1;
        *var_evbc3vdcex_dn10_slot = var_evbc3vdcex_dn10;
        *var_evbc3vdcex_dn11_slot = var_evbc3vdcex_dn11;
        *var_evbc3vdcex_dn2_slot = var_evbc3vdcex_dn2;
        *var_evbc3vdcex_dn3_slot = var_evbc3vdcex_dn3;
        *var_evbc3vdcex_dn4_slot = var_evbc3vdcex_dn4;
        *var_evbc3vdcex_dn5_slot = var_evbc3vdcex_dn5;
        *var_evbc3vdcex_dn6_slot = var_evbc3vdcex_dn6;
        *var_evbc3vdcex_dn7_slot = var_evbc3vdcex_dn7;
        *var_evbc3vdcex_dn8_slot = var_evbc3vdcex_dn8;
        *var_evbc3vdcex_dn9_slot = var_evbc3vdcex_dn9;
        *var_expl_slot = var_expl;
        *var_guard117_slot = var_guard117;
        *var_guard118_slot = var_guard118;
        *var_guard119_slot = var_guard119;
        *var_guard120_slot = var_guard120;
        *var_guard121_slot = var_guard121;
        *var_qex_slot = var_qex;
        *var_qex_db0_slot = var_qex_db0;
        *var_qex_db1_slot = var_qex_db1;
        *var_qex_dn0_slot = var_qex_dn0;
        *var_qex_dn1_slot = var_qex_dn1;
        *var_qex_dn10_slot = var_qex_dn10;
        *var_qex_dn11_slot = var_qex_dn11;
        *var_qex_dn2_slot = var_qex_dn2;
        *var_qex_dn3_slot = var_qex_dn3;
        *var_qex_dn4_slot = var_qex_dn4;
        *var_qex_dn5_slot = var_qex_dn5;
        *var_qex_dn6_slot = var_qex_dn6;
        *var_qex_dn7_slot = var_qex_dn7;
        *var_qex_dn8_slot = var_qex_dn8;
        *var_qex_dn9_slot = var_qex_dn9;
        *var_vb2e1vfe_slot = var_vb2e1vfe;
        *var_vb2e1vfe_db0_slot = var_vb2e1vfe_db0;
        *var_vb2e1vfe_db1_slot = var_vb2e1vfe_db1;
        *var_vb2e1vfe_dn0_slot = var_vb2e1vfe_dn0;
        *var_vb2e1vfe_dn1_slot = var_vb2e1vfe_dn1;
        *var_vb2e1vfe_dn10_slot = var_vb2e1vfe_dn10;
        *var_vb2e1vfe_dn11_slot = var_vb2e1vfe_dn11;
        *var_vb2e1vfe_dn2_slot = var_vb2e1vfe_dn2;
        *var_vb2e1vfe_dn3_slot = var_vb2e1vfe_dn3;
        *var_vb2e1vfe_dn4_slot = var_vb2e1vfe_dn4;
        *var_vb2e1vfe_dn5_slot = var_vb2e1vfe_dn5;
        *var_vb2e1vfe_dn6_slot = var_vb2e1vfe_dn6;
        *var_vb2e1vfe_dn7_slot = var_vb2e1vfe_dn7;
        *var_vb2e1vfe_dn8_slot = var_vb2e1vfe_dn8;
        *var_vb2e1vfe_dn9_slot = var_vb2e1vfe_dn9;
        *var_xg1_slot = var_xg1;
        *var_xg1_db0_slot = var_xg1_db0;
        *var_xg1_db1_slot = var_xg1_db1;
        *var_xg1_dn0_slot = var_xg1_dn0;
        *var_xg1_dn1_slot = var_xg1_dn1;
        *var_xg1_dn10_slot = var_xg1_dn10;
        *var_xg1_dn11_slot = var_xg1_dn11;
        *var_xg1_dn2_slot = var_xg1_dn2;
        *var_xg1_dn3_slot = var_xg1_dn3;
        *var_xg1_dn4_slot = var_xg1_dn4;
        *var_xg1_dn5_slot = var_xg1_dn5;
        *var_xg1_dn6_slot = var_xg1_dn6;
        *var_xg1_dn7_slot = var_xg1_dn7;
        *var_xg1_dn8_slot = var_xg1_dn8;
        *var_xg1_dn9_slot = var_xg1_dn9;
        *var_xg2_slot = var_xg2;
        *var_xg2_db0_slot = var_xg2_db0;
        *var_xg2_db1_slot = var_xg2_db1;
        *var_xg2_dn0_slot = var_xg2_dn0;
        *var_xg2_dn1_slot = var_xg2_dn1;
        *var_xg2_dn10_slot = var_xg2_dn10;
        *var_xg2_dn11_slot = var_xg2_dn11;
        *var_xg2_dn2_slot = var_xg2_dn2;
        *var_xg2_dn3_slot = var_xg2_dn3;
        *var_xg2_dn4_slot = var_xg2_dn4;
        *var_xg2_dn5_slot = var_xg2_dn5;
        *var_xg2_dn6_slot = var_xg2_dn6;
        *var_xg2_dn7_slot = var_xg2_dn7;
        *var_xg2_dn8_slot = var_xg2_dn8;
        *var_xg2_dn9_slot = var_xg2_dn9;
        *var_xnbex_slot = var_xnbex;
        *var_xnbex_db0_slot = var_xnbex_db0;
        *var_xnbex_db1_slot = var_xnbex_db1;
        *var_xnbex_dn0_slot = var_xnbex_dn0;
        *var_xnbex_dn1_slot = var_xnbex_dn1;
        *var_xnbex_dn10_slot = var_xnbex_dn10;
        *var_xnbex_dn11_slot = var_xnbex_dn11;
        *var_xnbex_dn2_slot = var_xnbex_dn2;
        *var_xnbex_dn3_slot = var_xnbex_dn3;
        *var_xnbex_dn4_slot = var_xnbex_dn4;
        *var_xnbex_dn5_slot = var_xnbex_dn5;
        *var_xnbex_dn6_slot = var_xnbex_dn6;
        *var_xnbex_dn7_slot = var_xnbex_dn7;
        *var_xnbex_dn8_slot = var_xnbex_dn8;
        *var_xnbex_dn9_slot = var_xnbex_dn9;
        *var_xpwex_slot = var_xpwex;
        *var_xpwex_db0_slot = var_xpwex_db0;
        *var_xpwex_db1_slot = var_xpwex_db1;
        *var_xpwex_dn0_slot = var_xpwex_dn0;
        *var_xpwex_dn1_slot = var_xpwex_dn1;
        *var_xpwex_dn10_slot = var_xpwex_dn10;
        *var_xpwex_dn11_slot = var_xpwex_dn11;
        *var_xpwex_dn2_slot = var_xpwex_dn2;
        *var_xpwex_dn3_slot = var_xpwex_dn3;
        *var_xpwex_dn4_slot = var_xpwex_dn4;
        *var_xpwex_dn5_slot = var_xpwex_dn5;
        *var_xpwex_dn6_slot = var_xpwex_dn6;
        *var_xpwex_dn7_slot = var_xpwex_dn7;
        *var_xpwex_dn8_slot = var_xpwex_dn8;
        *var_xpwex_dn9_slot = var_xpwex_dn9;
        *var_xqex_slot = var_xqex;
        *var_xqex_db0_slot = var_xqex_db0;
        *var_xqex_db1_slot = var_xqex_db1;
        *var_xqex_dn0_slot = var_xqex_dn0;
        *var_xqex_dn1_slot = var_xqex_dn1;
        *var_xqex_dn10_slot = var_xqex_dn10;
        *var_xqex_dn11_slot = var_xqex_dn11;
        *var_xqex_dn2_slot = var_xqex_dn2;
        *var_xqex_dn3_slot = var_xqex_dn3;
        *var_xqex_dn4_slot = var_xqex_dn4;
        *var_xqex_dn5_slot = var_xqex_dn5;
        *var_xqex_dn6_slot = var_xqex_dn6;
        *var_xqex_dn7_slot = var_xqex_dn7;
        *var_xqex_dn8_slot = var_xqex_dn8;
        *var_xqex_dn9_slot = var_xqex_dn9;
        *var_xqmex_slot = var_xqmex;
        *var_xqmex_db0_slot = var_xqmex_db0;
        *var_xqmex_db1_slot = var_xqmex_db1;
        *var_xqmex_dn0_slot = var_xqmex_dn0;
        *var_xqmex_dn1_slot = var_xqmex_dn1;
        *var_xqmex_dn10_slot = var_xqmex_dn10;
        *var_xqmex_dn11_slot = var_xqmex_dn11;
        *var_xqmex_dn2_slot = var_xqmex_dn2;
        *var_xqmex_dn3_slot = var_xqmex_dn3;
        *var_xqmex_dn4_slot = var_xqmex_dn4;
        *var_xqmex_dn5_slot = var_xqmex_dn5;
        *var_xqmex_dn6_slot = var_xqmex_dn6;
        *var_xqmex_dn7_slot = var_xqmex_dn7;
        *var_xqmex_dn8_slot = var_xqmex_dn8;
        *var_xqmex_dn9_slot = var_xqmex_dn9;
    }

    pub(super) fn stamp_transient_block_18(
        p: &Parameters,
        var_cje_t: f64,
        var_cje_t_db0: f64,
        var_cje_t_db1: f64,
        var_cje_t_dn0: f64,
        var_cje_t_dn1: f64,
        var_cje_t_dn10: f64,
        var_cje_t_dn11: f64,
        var_cje_t_dn2: f64,
        var_cje_t_dn3: f64,
        var_cje_t_dn4: f64,
        var_cje_t_dn5: f64,
        var_cje_t_dn6: f64,
        var_cje_t_dn7: f64,
        var_cje_t_dn8: f64,
        var_cje_t_dn9: f64,
        var_dvtevb2e1: f64,
        var_dvtevb2e1_db0: f64,
        var_dvtevb2e1_db1: f64,
        var_dvtevb2e1_dn0: f64,
        var_dvtevb2e1_dn1: f64,
        var_dvtevb2e1_dn10: f64,
        var_dvtevb2e1_dn11: f64,
        var_dvtevb2e1_dn2: f64,
        var_dvtevb2e1_dn3: f64,
        var_dvtevb2e1_dn4: f64,
        var_dvtevb2e1_dn5: f64,
        var_dvtevb2e1_dn6: f64,
        var_dvtevb2e1_dn7: f64,
        var_dvtevb2e1_dn8: f64,
        var_dvtevb2e1_dn9: f64,
        var_evb2e1: f64,
        var_evb2e1_db0: f64,
        var_evb2e1_db1: f64,
        var_evb2e1_dn0: f64,
        var_evb2e1_dn1: f64,
        var_evb2e1_dn10: f64,
        var_evb2e1_dn11: f64,
        var_evb2e1_dn2: f64,
        var_evb2e1_dn3: f64,
        var_evb2e1_dn4: f64,
        var_evb2e1_dn5: f64,
        var_evb2e1_dn6: f64,
        var_evb2e1_dn7: f64,
        var_evb2e1_dn8: f64,
        var_evb2e1_dn9: f64,
        var_f1: f64,
        var_f1_db0: f64,
        var_f1_db1: f64,
        var_f1_dn0: f64,
        var_f1_dn1: f64,
        var_f1_dn10: f64,
        var_f1_dn11: f64,
        var_f1_dn2: f64,
        var_f1_dn3: f64,
        var_f1_dn4: f64,
        var_f1_dn5: f64,
        var_f1_dn6: f64,
        var_f1_dn7: f64,
        var_f1_dn8: f64,
        var_f1_dn9: f64,
        var_guard120: f64,
        var_if0: f64,
        var_if0_db0: f64,
        var_if0_db1: f64,
        var_if0_dn0: f64,
        var_if0_dn1: f64,
        var_if0_dn10: f64,
        var_if0_dn11: f64,
        var_if0_dn2: f64,
        var_if0_dn3: f64,
        var_if0_dn4: f64,
        var_if0_dn5: f64,
        var_if0_dn6: f64,
        var_if0_dn7: f64,
        var_if0_dn8: f64,
        var_if0_dn9: f64,
        var_if_: f64,
        var_if__db0: f64,
        var_if__db1: f64,
        var_if__dn0: f64,
        var_if__dn1: f64,
        var_if__dn10: f64,
        var_if__dn11: f64,
        var_if__dn2: f64,
        var_if__dn3: f64,
        var_if__dn4: f64,
        var_if__dn5: f64,
        var_if__dn6: f64,
        var_if__dn7: f64,
        var_if__dn8: f64,
        var_if__dn9: f64,
        var_ir: f64,
        var_ir_db0: f64,
        var_ir_db1: f64,
        var_ir_dn0: f64,
        var_ir_dn1: f64,
        var_ir_dn10: f64,
        var_ir_dn11: f64,
        var_ir_dn2: f64,
        var_ir_dn3: f64,
        var_ir_dn4: f64,
        var_ir_dn5: f64,
        var_ir_dn6: f64,
        var_ir_dn7: f64,
        var_ir_dn8: f64,
        var_ir_dn9: f64,
        var_nff_t: f64,
        var_nff_t_db0: f64,
        var_nff_t_db1: f64,
        var_nff_t_dn0: f64,
        var_nff_t_dn1: f64,
        var_nff_t_dn10: f64,
        var_nff_t_dn11: f64,
        var_nff_t_dn2: f64,
        var_nff_t_dn3: f64,
        var_nff_t_dn4: f64,
        var_nff_t_dn5: f64,
        var_nff_t_dn6: f64,
        var_nff_t_dn7: f64,
        var_nff_t_dn8: f64,
        var_nff_t_dn9: f64,
        var_q1q: f64,
        var_q1q_db0: f64,
        var_q1q_db1: f64,
        var_q1q_dn0: f64,
        var_q1q_dn1: f64,
        var_q1q_dn10: f64,
        var_q1q_dn11: f64,
        var_q1q_dn2: f64,
        var_q1q_dn3: f64,
        var_q1q_dn4: f64,
        var_q1q_dn5: f64,
        var_q1q_dn6: f64,
        var_q1q_dn7: f64,
        var_q1q_dn8: f64,
        var_q1q_dn9: f64,
        var_qb0: f64,
        var_qbc_qs: f64,
        var_qbc_qs_db0: f64,
        var_qbc_qs_db1: f64,
        var_qbc_qs_dn0: f64,
        var_qbc_qs_dn1: f64,
        var_qbc_qs_dn10: f64,
        var_qbc_qs_dn11: f64,
        var_qbc_qs_dn2: f64,
        var_qbc_qs_dn3: f64,
        var_qbc_qs_dn4: f64,
        var_qbc_qs_dn5: f64,
        var_qbc_qs_dn6: f64,
        var_qbc_qs_dn7: f64,
        var_qbc_qs_dn8: f64,
        var_qbc_qs_dn9: f64,
        var_qbe_qs: f64,
        var_qbe_qs_db0: f64,
        var_qbe_qs_db1: f64,
        var_qbe_qs_dn0: f64,
        var_qbe_qs_dn1: f64,
        var_qbe_qs_dn10: f64,
        var_qbe_qs_dn11: f64,
        var_qbe_qs_dn2: f64,
        var_qbe_qs_dn3: f64,
        var_qbe_qs_dn4: f64,
        var_qbe_qs_dn5: f64,
        var_qbe_qs_dn6: f64,
        var_qbe_qs_dn7: f64,
        var_qbe_qs_dn8: f64,
        var_qbe_qs_dn9: f64,
        var_qbi: f64,
        var_qbi_db0: f64,
        var_qbi_db1: f64,
        var_qbi_dn0: f64,
        var_qbi_dn1: f64,
        var_qbi_dn10: f64,
        var_qbi_dn11: f64,
        var_qbi_dn2: f64,
        var_qbi_dn3: f64,
        var_qbi_dn4: f64,
        var_qbi_dn5: f64,
        var_qbi_dn6: f64,
        var_qbi_dn7: f64,
        var_qbi_dn8: f64,
        var_qbi_dn9: f64,
        var_qe_qs: f64,
        var_qe_qs_db0: f64,
        var_qe_qs_db1: f64,
        var_qe_qs_dn0: f64,
        var_qe_qs_dn1: f64,
        var_qe_qs_dn10: f64,
        var_qe_qs_dn11: f64,
        var_qe_qs_dn2: f64,
        var_qe_qs_dn3: f64,
        var_qe_qs_dn4: f64,
        var_qe_qs_dn5: f64,
        var_qe_qs_dn6: f64,
        var_qe_qs_dn7: f64,
        var_qe_qs_dn8: f64,
        var_qe_qs_dn9: f64,
        var_taub_t: f64,
        var_vb1b2: f64,
        var_vb1b2_db0: f64,
        var_vb1b2_db1: f64,
        var_vb1b2_dn0: f64,
        var_vb1b2_dn1: f64,
        var_vb1b2_dn10: f64,
        var_vb1b2_dn11: f64,
        var_vb1b2_dn2: f64,
        var_vb1b2_dn3: f64,
        var_vb1b2_dn4: f64,
        var_vb1b2_dn5: f64,
        var_vb1b2_dn6: f64,
        var_vb1b2_dn7: f64,
        var_vb1b2_dn8: f64,
        var_vb1b2_dn9: f64,
        var_vt: f64,
        var_vtinv: f64,
        var_dn0vb2e1_slot: &mut f64,
        var_dn0vb2e1_db0_slot: &mut f64,
        var_dn0vb2e1_db1_slot: &mut f64,
        var_dn0vb2e1_dn0_slot: &mut f64,
        var_dn0vb2e1_dn1_slot: &mut f64,
        var_dn0vb2e1_dn10_slot: &mut f64,
        var_dn0vb2e1_dn11_slot: &mut f64,
        var_dn0vb2e1_dn2_slot: &mut f64,
        var_dn0vb2e1_dn3_slot: &mut f64,
        var_dn0vb2e1_dn4_slot: &mut f64,
        var_dn0vb2e1_dn5_slot: &mut f64,
        var_dn0vb2e1_dn6_slot: &mut f64,
        var_dn0vb2e1_dn7_slot: &mut f64,
        var_dn0vb2e1_dn8_slot: &mut f64,
        var_dn0vb2e1_dn9_slot: &mut f64,
        var_dqbevb2e1_slot: &mut f64,
        var_dqbevb2e1_db0_slot: &mut f64,
        var_dqbevb2e1_db1_slot: &mut f64,
        var_dqbevb2e1_dn0_slot: &mut f64,
        var_dqbevb2e1_dn1_slot: &mut f64,
        var_dqbevb2e1_dn10_slot: &mut f64,
        var_dqbevb2e1_dn11_slot: &mut f64,
        var_dqbevb2e1_dn2_slot: &mut f64,
        var_dqbevb2e1_dn3_slot: &mut f64,
        var_dqbevb2e1_dn4_slot: &mut f64,
        var_dqbevb2e1_dn5_slot: &mut f64,
        var_dqbevb2e1_dn6_slot: &mut f64,
        var_dqbevb2e1_dn7_slot: &mut f64,
        var_dqbevb2e1_dn8_slot: &mut f64,
        var_dqbevb2e1_dn9_slot: &mut f64,
        var_dqevb2e1_slot: &mut f64,
        var_dqevb2e1_db0_slot: &mut f64,
        var_dqevb2e1_db1_slot: &mut f64,
        var_dqevb2e1_dn0_slot: &mut f64,
        var_dqevb2e1_dn1_slot: &mut f64,
        var_dqevb2e1_dn10_slot: &mut f64,
        var_dqevb2e1_dn11_slot: &mut f64,
        var_dqevb2e1_dn2_slot: &mut f64,
        var_dqevb2e1_dn3_slot: &mut f64,
        var_dqevb2e1_dn4_slot: &mut f64,
        var_dqevb2e1_dn5_slot: &mut f64,
        var_dqevb2e1_dn6_slot: &mut f64,
        var_dqevb2e1_dn7_slot: &mut f64,
        var_dqevb2e1_dn8_slot: &mut f64,
        var_dqevb2e1_dn9_slot: &mut f64,
        var_dqtevb2e1_slot: &mut f64,
        var_dqtevb2e1_db0_slot: &mut f64,
        var_dqtevb2e1_db1_slot: &mut f64,
        var_dqtevb2e1_dn0_slot: &mut f64,
        var_dqtevb2e1_dn1_slot: &mut f64,
        var_dqtevb2e1_dn10_slot: &mut f64,
        var_dqtevb2e1_dn11_slot: &mut f64,
        var_dqtevb2e1_dn2_slot: &mut f64,
        var_dqtevb2e1_dn3_slot: &mut f64,
        var_dqtevb2e1_dn4_slot: &mut f64,
        var_dqtevb2e1_dn5_slot: &mut f64,
        var_dqtevb2e1_dn6_slot: &mut f64,
        var_dqtevb2e1_dn7_slot: &mut f64,
        var_dqtevb2e1_dn8_slot: &mut f64,
        var_dqtevb2e1_dn9_slot: &mut f64,
        var_guard126_slot: &mut f64,
        var_guard127_slot: &mut f64,
        var_guard128_slot: &mut f64,
        var_in_n_slot: &mut f64,
        var_in_n_db0_slot: &mut f64,
        var_in_n_db1_slot: &mut f64,
        var_in_n_dn0_slot: &mut f64,
        var_in_n_dn1_slot: &mut f64,
        var_in_n_dn10_slot: &mut f64,
        var_in_n_dn11_slot: &mut f64,
        var_in_n_dn2_slot: &mut f64,
        var_in_n_dn3_slot: &mut f64,
        var_in_n_dn4_slot: &mut f64,
        var_in_n_dn5_slot: &mut f64,
        var_in_n_dn6_slot: &mut f64,
        var_in_n_dn7_slot: &mut f64,
        var_in_n_dn8_slot: &mut f64,
        var_in_n_dn9_slot: &mut f64,
        var_qb1b2_slot: &mut f64,
        var_qb1b2_db0_slot: &mut f64,
        var_qb1b2_db1_slot: &mut f64,
        var_qb1b2_dn0_slot: &mut f64,
        var_qb1b2_dn1_slot: &mut f64,
        var_qb1b2_dn10_slot: &mut f64,
        var_qb1b2_dn11_slot: &mut f64,
        var_qb1b2_dn2_slot: &mut f64,
        var_qb1b2_dn3_slot: &mut f64,
        var_qb1b2_dn4_slot: &mut f64,
        var_qb1b2_dn5_slot: &mut f64,
        var_qb1b2_dn6_slot: &mut f64,
        var_qb1b2_dn7_slot: &mut f64,
        var_qb1b2_dn8_slot: &mut f64,
        var_qb1b2_dn9_slot: &mut f64,
        var_qbc_slot: &mut f64,
        var_qbc_db0_slot: &mut f64,
        var_qbc_db1_slot: &mut f64,
        var_qbc_dn0_slot: &mut f64,
        var_qbc_dn1_slot: &mut f64,
        var_qbc_dn10_slot: &mut f64,
        var_qbc_dn11_slot: &mut f64,
        var_qbc_dn2_slot: &mut f64,
        var_qbc_dn3_slot: &mut f64,
        var_qbc_dn4_slot: &mut f64,
        var_qbc_dn5_slot: &mut f64,
        var_qbc_dn6_slot: &mut f64,
        var_qbc_dn7_slot: &mut f64,
        var_qbc_dn8_slot: &mut f64,
        var_qbc_dn9_slot: &mut f64,
        var_qbe_slot: &mut f64,
        var_qbe_db0_slot: &mut f64,
        var_qbe_db1_slot: &mut f64,
        var_qbe_dn0_slot: &mut f64,
        var_qbe_dn1_slot: &mut f64,
        var_qbe_dn10_slot: &mut f64,
        var_qbe_dn11_slot: &mut f64,
        var_qbe_dn2_slot: &mut f64,
        var_qbe_dn3_slot: &mut f64,
        var_qbe_dn4_slot: &mut f64,
        var_qbe_dn5_slot: &mut f64,
        var_qbe_dn6_slot: &mut f64,
        var_qbe_dn7_slot: &mut f64,
        var_qbe_dn8_slot: &mut f64,
        var_qbe_dn9_slot: &mut f64,
        var_qbe_qs_eff_slot: &mut f64,
        var_qbe_qs_eff_db0_slot: &mut f64,
        var_qbe_qs_eff_db1_slot: &mut f64,
        var_qbe_qs_eff_dn0_slot: &mut f64,
        var_qbe_qs_eff_dn1_slot: &mut f64,
        var_qbe_qs_eff_dn10_slot: &mut f64,
        var_qbe_qs_eff_dn11_slot: &mut f64,
        var_qbe_qs_eff_dn2_slot: &mut f64,
        var_qbe_qs_eff_dn3_slot: &mut f64,
        var_qbe_qs_eff_dn4_slot: &mut f64,
        var_qbe_qs_eff_dn5_slot: &mut f64,
        var_qbe_qs_eff_dn6_slot: &mut f64,
        var_qbe_qs_eff_dn7_slot: &mut f64,
        var_qbe_qs_eff_dn8_slot: &mut f64,
        var_qbe_qs_eff_dn9_slot: &mut f64,
        var_qe_slot: &mut f64,
        var_qe_db0_slot: &mut f64,
        var_qe_db1_slot: &mut f64,
        var_qe_dn0_slot: &mut f64,
        var_qe_dn1_slot: &mut f64,
        var_qe_dn10_slot: &mut f64,
        var_qe_dn11_slot: &mut f64,
        var_qe_dn2_slot: &mut f64,
        var_qe_dn3_slot: &mut f64,
        var_qe_dn4_slot: &mut f64,
        var_qe_dn5_slot: &mut f64,
        var_qe_dn6_slot: &mut f64,
        var_qe_dn7_slot: &mut f64,
        var_qe_dn8_slot: &mut f64,
        var_qe_dn9_slot: &mut f64,
        var_taub_n_slot: &mut f64,
        var_taub_n_db0_slot: &mut f64,
        var_taub_n_db1_slot: &mut f64,
        var_taub_n_dn0_slot: &mut f64,
        var_taub_n_dn1_slot: &mut f64,
        var_taub_n_dn10_slot: &mut f64,
        var_taub_n_dn11_slot: &mut f64,
        var_taub_n_dn2_slot: &mut f64,
        var_taub_n_dn3_slot: &mut f64,
        var_taub_n_dn4_slot: &mut f64,
        var_taub_n_dn5_slot: &mut f64,
        var_taub_n_dn6_slot: &mut f64,
        var_taub_n_dn7_slot: &mut f64,
        var_taub_n_dn8_slot: &mut f64,
        var_taub_n_dn9_slot: &mut f64,
        var_taun_slot: &mut f64,
        var_taun_db0_slot: &mut f64,
        var_taun_db1_slot: &mut f64,
        var_taun_dn0_slot: &mut f64,
        var_taun_dn1_slot: &mut f64,
        var_taun_dn10_slot: &mut f64,
        var_taun_dn11_slot: &mut f64,
        var_taun_dn2_slot: &mut f64,
        var_taun_dn3_slot: &mut f64,
        var_taun_dn4_slot: &mut f64,
        var_taun_dn5_slot: &mut f64,
        var_taun_dn6_slot: &mut f64,
        var_taun_dn7_slot: &mut f64,
        var_taun_dn8_slot: &mut f64,
        var_taun_dn9_slot: &mut f64,
    ) {
        let mut var_dn0vb2e1: f64 = *var_dn0vb2e1_slot;
        let mut var_dn0vb2e1_db0: f64 = *var_dn0vb2e1_db0_slot;
        let mut var_dn0vb2e1_db1: f64 = *var_dn0vb2e1_db1_slot;
        let mut var_dn0vb2e1_dn0: f64 = *var_dn0vb2e1_dn0_slot;
        let mut var_dn0vb2e1_dn1: f64 = *var_dn0vb2e1_dn1_slot;
        let mut var_dn0vb2e1_dn10: f64 = *var_dn0vb2e1_dn10_slot;
        let mut var_dn0vb2e1_dn11: f64 = *var_dn0vb2e1_dn11_slot;
        let mut var_dn0vb2e1_dn2: f64 = *var_dn0vb2e1_dn2_slot;
        let mut var_dn0vb2e1_dn3: f64 = *var_dn0vb2e1_dn3_slot;
        let mut var_dn0vb2e1_dn4: f64 = *var_dn0vb2e1_dn4_slot;
        let mut var_dn0vb2e1_dn5: f64 = *var_dn0vb2e1_dn5_slot;
        let mut var_dn0vb2e1_dn6: f64 = *var_dn0vb2e1_dn6_slot;
        let mut var_dn0vb2e1_dn7: f64 = *var_dn0vb2e1_dn7_slot;
        let mut var_dn0vb2e1_dn8: f64 = *var_dn0vb2e1_dn8_slot;
        let mut var_dn0vb2e1_dn9: f64 = *var_dn0vb2e1_dn9_slot;
        let mut var_dqbevb2e1: f64 = *var_dqbevb2e1_slot;
        let mut var_dqbevb2e1_db0: f64 = *var_dqbevb2e1_db0_slot;
        let mut var_dqbevb2e1_db1: f64 = *var_dqbevb2e1_db1_slot;
        let mut var_dqbevb2e1_dn0: f64 = *var_dqbevb2e1_dn0_slot;
        let mut var_dqbevb2e1_dn1: f64 = *var_dqbevb2e1_dn1_slot;
        let mut var_dqbevb2e1_dn10: f64 = *var_dqbevb2e1_dn10_slot;
        let mut var_dqbevb2e1_dn11: f64 = *var_dqbevb2e1_dn11_slot;
        let mut var_dqbevb2e1_dn2: f64 = *var_dqbevb2e1_dn2_slot;
        let mut var_dqbevb2e1_dn3: f64 = *var_dqbevb2e1_dn3_slot;
        let mut var_dqbevb2e1_dn4: f64 = *var_dqbevb2e1_dn4_slot;
        let mut var_dqbevb2e1_dn5: f64 = *var_dqbevb2e1_dn5_slot;
        let mut var_dqbevb2e1_dn6: f64 = *var_dqbevb2e1_dn6_slot;
        let mut var_dqbevb2e1_dn7: f64 = *var_dqbevb2e1_dn7_slot;
        let mut var_dqbevb2e1_dn8: f64 = *var_dqbevb2e1_dn8_slot;
        let mut var_dqbevb2e1_dn9: f64 = *var_dqbevb2e1_dn9_slot;
        let mut var_dqevb2e1: f64 = *var_dqevb2e1_slot;
        let mut var_dqevb2e1_db0: f64 = *var_dqevb2e1_db0_slot;
        let mut var_dqevb2e1_db1: f64 = *var_dqevb2e1_db1_slot;
        let mut var_dqevb2e1_dn0: f64 = *var_dqevb2e1_dn0_slot;
        let mut var_dqevb2e1_dn1: f64 = *var_dqevb2e1_dn1_slot;
        let mut var_dqevb2e1_dn10: f64 = *var_dqevb2e1_dn10_slot;
        let mut var_dqevb2e1_dn11: f64 = *var_dqevb2e1_dn11_slot;
        let mut var_dqevb2e1_dn2: f64 = *var_dqevb2e1_dn2_slot;
        let mut var_dqevb2e1_dn3: f64 = *var_dqevb2e1_dn3_slot;
        let mut var_dqevb2e1_dn4: f64 = *var_dqevb2e1_dn4_slot;
        let mut var_dqevb2e1_dn5: f64 = *var_dqevb2e1_dn5_slot;
        let mut var_dqevb2e1_dn6: f64 = *var_dqevb2e1_dn6_slot;
        let mut var_dqevb2e1_dn7: f64 = *var_dqevb2e1_dn7_slot;
        let mut var_dqevb2e1_dn8: f64 = *var_dqevb2e1_dn8_slot;
        let mut var_dqevb2e1_dn9: f64 = *var_dqevb2e1_dn9_slot;
        let mut var_dqtevb2e1: f64 = *var_dqtevb2e1_slot;
        let mut var_dqtevb2e1_db0: f64 = *var_dqtevb2e1_db0_slot;
        let mut var_dqtevb2e1_db1: f64 = *var_dqtevb2e1_db1_slot;
        let mut var_dqtevb2e1_dn0: f64 = *var_dqtevb2e1_dn0_slot;
        let mut var_dqtevb2e1_dn1: f64 = *var_dqtevb2e1_dn1_slot;
        let mut var_dqtevb2e1_dn10: f64 = *var_dqtevb2e1_dn10_slot;
        let mut var_dqtevb2e1_dn11: f64 = *var_dqtevb2e1_dn11_slot;
        let mut var_dqtevb2e1_dn2: f64 = *var_dqtevb2e1_dn2_slot;
        let mut var_dqtevb2e1_dn3: f64 = *var_dqtevb2e1_dn3_slot;
        let mut var_dqtevb2e1_dn4: f64 = *var_dqtevb2e1_dn4_slot;
        let mut var_dqtevb2e1_dn5: f64 = *var_dqtevb2e1_dn5_slot;
        let mut var_dqtevb2e1_dn6: f64 = *var_dqtevb2e1_dn6_slot;
        let mut var_dqtevb2e1_dn7: f64 = *var_dqtevb2e1_dn7_slot;
        let mut var_dqtevb2e1_dn8: f64 = *var_dqtevb2e1_dn8_slot;
        let mut var_dqtevb2e1_dn9: f64 = *var_dqtevb2e1_dn9_slot;
        let mut var_guard126: f64 = *var_guard126_slot;
        let mut var_guard127: f64 = *var_guard127_slot;
        let mut var_guard128: f64 = *var_guard128_slot;
        let mut var_in_n: f64 = *var_in_n_slot;
        let mut var_in_n_db0: f64 = *var_in_n_db0_slot;
        let mut var_in_n_db1: f64 = *var_in_n_db1_slot;
        let mut var_in_n_dn0: f64 = *var_in_n_dn0_slot;
        let mut var_in_n_dn1: f64 = *var_in_n_dn1_slot;
        let mut var_in_n_dn10: f64 = *var_in_n_dn10_slot;
        let mut var_in_n_dn11: f64 = *var_in_n_dn11_slot;
        let mut var_in_n_dn2: f64 = *var_in_n_dn2_slot;
        let mut var_in_n_dn3: f64 = *var_in_n_dn3_slot;
        let mut var_in_n_dn4: f64 = *var_in_n_dn4_slot;
        let mut var_in_n_dn5: f64 = *var_in_n_dn5_slot;
        let mut var_in_n_dn6: f64 = *var_in_n_dn6_slot;
        let mut var_in_n_dn7: f64 = *var_in_n_dn7_slot;
        let mut var_in_n_dn8: f64 = *var_in_n_dn8_slot;
        let mut var_in_n_dn9: f64 = *var_in_n_dn9_slot;
        let mut var_qb1b2: f64 = *var_qb1b2_slot;
        let mut var_qb1b2_db0: f64 = *var_qb1b2_db0_slot;
        let mut var_qb1b2_db1: f64 = *var_qb1b2_db1_slot;
        let mut var_qb1b2_dn0: f64 = *var_qb1b2_dn0_slot;
        let mut var_qb1b2_dn1: f64 = *var_qb1b2_dn1_slot;
        let mut var_qb1b2_dn10: f64 = *var_qb1b2_dn10_slot;
        let mut var_qb1b2_dn11: f64 = *var_qb1b2_dn11_slot;
        let mut var_qb1b2_dn2: f64 = *var_qb1b2_dn2_slot;
        let mut var_qb1b2_dn3: f64 = *var_qb1b2_dn3_slot;
        let mut var_qb1b2_dn4: f64 = *var_qb1b2_dn4_slot;
        let mut var_qb1b2_dn5: f64 = *var_qb1b2_dn5_slot;
        let mut var_qb1b2_dn6: f64 = *var_qb1b2_dn6_slot;
        let mut var_qb1b2_dn7: f64 = *var_qb1b2_dn7_slot;
        let mut var_qb1b2_dn8: f64 = *var_qb1b2_dn8_slot;
        let mut var_qb1b2_dn9: f64 = *var_qb1b2_dn9_slot;
        let mut var_qbc: f64 = *var_qbc_slot;
        let mut var_qbc_db0: f64 = *var_qbc_db0_slot;
        let mut var_qbc_db1: f64 = *var_qbc_db1_slot;
        let mut var_qbc_dn0: f64 = *var_qbc_dn0_slot;
        let mut var_qbc_dn1: f64 = *var_qbc_dn1_slot;
        let mut var_qbc_dn10: f64 = *var_qbc_dn10_slot;
        let mut var_qbc_dn11: f64 = *var_qbc_dn11_slot;
        let mut var_qbc_dn2: f64 = *var_qbc_dn2_slot;
        let mut var_qbc_dn3: f64 = *var_qbc_dn3_slot;
        let mut var_qbc_dn4: f64 = *var_qbc_dn4_slot;
        let mut var_qbc_dn5: f64 = *var_qbc_dn5_slot;
        let mut var_qbc_dn6: f64 = *var_qbc_dn6_slot;
        let mut var_qbc_dn7: f64 = *var_qbc_dn7_slot;
        let mut var_qbc_dn8: f64 = *var_qbc_dn8_slot;
        let mut var_qbc_dn9: f64 = *var_qbc_dn9_slot;
        let mut var_qbe: f64 = *var_qbe_slot;
        let mut var_qbe_db0: f64 = *var_qbe_db0_slot;
        let mut var_qbe_db1: f64 = *var_qbe_db1_slot;
        let mut var_qbe_dn0: f64 = *var_qbe_dn0_slot;
        let mut var_qbe_dn1: f64 = *var_qbe_dn1_slot;
        let mut var_qbe_dn10: f64 = *var_qbe_dn10_slot;
        let mut var_qbe_dn11: f64 = *var_qbe_dn11_slot;
        let mut var_qbe_dn2: f64 = *var_qbe_dn2_slot;
        let mut var_qbe_dn3: f64 = *var_qbe_dn3_slot;
        let mut var_qbe_dn4: f64 = *var_qbe_dn4_slot;
        let mut var_qbe_dn5: f64 = *var_qbe_dn5_slot;
        let mut var_qbe_dn6: f64 = *var_qbe_dn6_slot;
        let mut var_qbe_dn7: f64 = *var_qbe_dn7_slot;
        let mut var_qbe_dn8: f64 = *var_qbe_dn8_slot;
        let mut var_qbe_dn9: f64 = *var_qbe_dn9_slot;
        let mut var_qbe_qs_eff: f64 = *var_qbe_qs_eff_slot;
        let mut var_qbe_qs_eff_db0: f64 = *var_qbe_qs_eff_db0_slot;
        let mut var_qbe_qs_eff_db1: f64 = *var_qbe_qs_eff_db1_slot;
        let mut var_qbe_qs_eff_dn0: f64 = *var_qbe_qs_eff_dn0_slot;
        let mut var_qbe_qs_eff_dn1: f64 = *var_qbe_qs_eff_dn1_slot;
        let mut var_qbe_qs_eff_dn10: f64 = *var_qbe_qs_eff_dn10_slot;
        let mut var_qbe_qs_eff_dn11: f64 = *var_qbe_qs_eff_dn11_slot;
        let mut var_qbe_qs_eff_dn2: f64 = *var_qbe_qs_eff_dn2_slot;
        let mut var_qbe_qs_eff_dn3: f64 = *var_qbe_qs_eff_dn3_slot;
        let mut var_qbe_qs_eff_dn4: f64 = *var_qbe_qs_eff_dn4_slot;
        let mut var_qbe_qs_eff_dn5: f64 = *var_qbe_qs_eff_dn5_slot;
        let mut var_qbe_qs_eff_dn6: f64 = *var_qbe_qs_eff_dn6_slot;
        let mut var_qbe_qs_eff_dn7: f64 = *var_qbe_qs_eff_dn7_slot;
        let mut var_qbe_qs_eff_dn8: f64 = *var_qbe_qs_eff_dn8_slot;
        let mut var_qbe_qs_eff_dn9: f64 = *var_qbe_qs_eff_dn9_slot;
        let mut var_qe: f64 = *var_qe_slot;
        let mut var_qe_db0: f64 = *var_qe_db0_slot;
        let mut var_qe_db1: f64 = *var_qe_db1_slot;
        let mut var_qe_dn0: f64 = *var_qe_dn0_slot;
        let mut var_qe_dn1: f64 = *var_qe_dn1_slot;
        let mut var_qe_dn10: f64 = *var_qe_dn10_slot;
        let mut var_qe_dn11: f64 = *var_qe_dn11_slot;
        let mut var_qe_dn2: f64 = *var_qe_dn2_slot;
        let mut var_qe_dn3: f64 = *var_qe_dn3_slot;
        let mut var_qe_dn4: f64 = *var_qe_dn4_slot;
        let mut var_qe_dn5: f64 = *var_qe_dn5_slot;
        let mut var_qe_dn6: f64 = *var_qe_dn6_slot;
        let mut var_qe_dn7: f64 = *var_qe_dn7_slot;
        let mut var_qe_dn8: f64 = *var_qe_dn8_slot;
        let mut var_qe_dn9: f64 = *var_qe_dn9_slot;
        let mut var_taub_n: f64 = *var_taub_n_slot;
        let mut var_taub_n_db0: f64 = *var_taub_n_db0_slot;
        let mut var_taub_n_db1: f64 = *var_taub_n_db1_slot;
        let mut var_taub_n_dn0: f64 = *var_taub_n_dn0_slot;
        let mut var_taub_n_dn1: f64 = *var_taub_n_dn1_slot;
        let mut var_taub_n_dn10: f64 = *var_taub_n_dn10_slot;
        let mut var_taub_n_dn11: f64 = *var_taub_n_dn11_slot;
        let mut var_taub_n_dn2: f64 = *var_taub_n_dn2_slot;
        let mut var_taub_n_dn3: f64 = *var_taub_n_dn3_slot;
        let mut var_taub_n_dn4: f64 = *var_taub_n_dn4_slot;
        let mut var_taub_n_dn5: f64 = *var_taub_n_dn5_slot;
        let mut var_taub_n_dn6: f64 = *var_taub_n_dn6_slot;
        let mut var_taub_n_dn7: f64 = *var_taub_n_dn7_slot;
        let mut var_taub_n_dn8: f64 = *var_taub_n_dn8_slot;
        let mut var_taub_n_dn9: f64 = *var_taub_n_dn9_slot;
        let mut var_taun: f64 = *var_taun_slot;
        let mut var_taun_db0: f64 = *var_taun_db0_slot;
        let mut var_taun_db1: f64 = *var_taun_db1_slot;
        let mut var_taun_dn0: f64 = *var_taun_dn0_slot;
        let mut var_taun_dn1: f64 = *var_taun_dn1_slot;
        let mut var_taun_dn10: f64 = *var_taun_dn10_slot;
        let mut var_taun_dn11: f64 = *var_taun_dn11_slot;
        let mut var_taun_dn2: f64 = *var_taun_dn2_slot;
        let mut var_taun_dn3: f64 = *var_taun_dn3_slot;
        let mut var_taun_dn4: f64 = *var_taun_dn4_slot;
        let mut var_taun_dn5: f64 = *var_taun_dn5_slot;
        let mut var_taun_dn6: f64 = *var_taun_dn6_slot;
        let mut var_taun_dn7: f64 = *var_taun_dn7_slot;
        let mut var_taun_dn8: f64 = *var_taun_dn8_slot;
        let mut var_taun_dn9: f64 = *var_taun_dn9_slot;

        let (assign6680_e6900, assign6680_e6900_d_n0, assign6680_e6900_d_n1, assign6680_e6900_d_n2, assign6680_e6900_d_n3, assign6680_e6900_d_n4, assign6680_e6900_d_n5, assign6680_e6900_d_n6, assign6680_e6900_d_n7, assign6680_e6900_d_n8, assign6680_e6900_d_n9, assign6680_e6900_d_n10, assign6680_e6900_d_n11, assign6680_e6900_d_b0, assign6680_e6900_d_b1,) = {
    if (var_guard120 != 0.0) {
        let assign6680_e6894: f64 = (1.0 - p.p68);
        let assign6680_e6896: f64 = (assign6680_e6894 * var_cje_t);
        let assign6680_e6898: f64 = (assign6680_e6896 * var_dvtevb2e1);
        (assign6680_e6898, (((assign6680_e6894 * var_cje_t_dn0) * var_dvtevb2e1) + (assign6680_e6896 * var_dvtevb2e1_dn0)), (((assign6680_e6894 * var_cje_t_dn1) * var_dvtevb2e1) + (assign6680_e6896 * var_dvtevb2e1_dn1)), (((assign6680_e6894 * var_cje_t_dn2) * var_dvtevb2e1) + (assign6680_e6896 * var_dvtevb2e1_dn2)), (((assign6680_e6894 * var_cje_t_dn3) * var_dvtevb2e1) + (assign6680_e6896 * var_dvtevb2e1_dn3)), (((assign6680_e6894 * var_cje_t_dn4) * var_dvtevb2e1) + (assign6680_e6896 * var_dvtevb2e1_dn4)), (((assign6680_e6894 * var_cje_t_dn5) * var_dvtevb2e1) + (assign6680_e6896 * var_dvtevb2e1_dn5)), (((assign6680_e6894 * var_cje_t_dn6) * var_dvtevb2e1) + (assign6680_e6896 * var_dvtevb2e1_dn6)), (((assign6680_e6894 * var_cje_t_dn7) * var_dvtevb2e1) + (assign6680_e6896 * var_dvtevb2e1_dn7)), (((assign6680_e6894 * var_cje_t_dn8) * var_dvtevb2e1) + (assign6680_e6896 * var_dvtevb2e1_dn8)), (((assign6680_e6894 * var_cje_t_dn9) * var_dvtevb2e1) + (assign6680_e6896 * var_dvtevb2e1_dn9)), (((assign6680_e6894 * var_cje_t_dn10) * var_dvtevb2e1) + (assign6680_e6896 * var_dvtevb2e1_dn10)), (((assign6680_e6894 * var_cje_t_dn11) * var_dvtevb2e1) + (assign6680_e6896 * var_dvtevb2e1_dn11)), (((assign6680_e6894 * var_cje_t_db0) * var_dvtevb2e1) + (assign6680_e6896 * var_dvtevb2e1_db0)), (((assign6680_e6894 * var_cje_t_db1) * var_dvtevb2e1) + (assign6680_e6896 * var_dvtevb2e1_db1)),)
    } else {
        (var_dqtevb2e1, var_dqtevb2e1_dn0, var_dqtevb2e1_dn1, var_dqtevb2e1_dn2, var_dqtevb2e1_dn3, var_dqtevb2e1_dn4, var_dqtevb2e1_dn5, var_dqtevb2e1_dn6, var_dqtevb2e1_dn7, var_dqtevb2e1_dn8, var_dqtevb2e1_dn9, var_dqtevb2e1_dn10, var_dqtevb2e1_dn11, var_dqtevb2e1_db0, var_dqtevb2e1_db1,)
    }
};
        var_dqtevb2e1 = assign6680_e6900;
        var_dqtevb2e1_dn0 = assign6680_e6900_d_n0;
        var_dqtevb2e1_dn1 = assign6680_e6900_d_n1;
        var_dqtevb2e1_dn2 = assign6680_e6900_d_n2;
        var_dqtevb2e1_dn3 = assign6680_e6900_d_n3;
        var_dqtevb2e1_dn4 = assign6680_e6900_d_n4;
        var_dqtevb2e1_dn5 = assign6680_e6900_d_n5;
        var_dqtevb2e1_dn6 = assign6680_e6900_d_n6;
        var_dqtevb2e1_dn7 = assign6680_e6900_d_n7;
        var_dqtevb2e1_dn8 = assign6680_e6900_d_n8;
        var_dqtevb2e1_dn9 = assign6680_e6900_d_n9;
        var_dqtevb2e1_dn10 = assign6680_e6900_d_n10;
        var_dqtevb2e1_dn11 = assign6680_e6900_d_n11;
        var_dqtevb2e1_db0 = assign6680_e6900_d_b0;
        var_dqtevb2e1_db1 = assign6680_e6900_d_b1;

        let (assign6690_e6917, assign6690_e6917_d_n0, assign6690_e6917_d_n1, assign6690_e6917_d_n2, assign6690_e6917_d_n3, assign6690_e6917_d_n4, assign6690_e6917_d_n5, assign6690_e6917_d_n6, assign6690_e6917_d_n7, assign6690_e6917_d_n8, assign6690_e6917_d_n9, assign6690_e6917_d_n10, assign6690_e6917_d_n11, assign6690_e6917_d_b0, assign6690_e6917_d_b1,) = {
    if (var_guard120 != 0.0) {
        let assign6690_e6904: f64 = (var_if0 * var_evb2e1);
        let assign6690_e6906: f64 = (assign6690_e6904 * var_vtinv);
        let assign6690_e6908: f64 = (assign6690_e6906 / var_nff_t);
        let assign6690_e6912: f64 = (1.0 + var_f1);
        let assign6690_e6913: f64 = (assign6690_e6912).sqrt();
        let assign6690_e6914: f64 = (0.5 / assign6690_e6913);
        let assign6690_e6915: f64 = (assign6690_e6908 * assign6690_e6914);
        (assign6690_e6915, ((((((((var_if0_dn0 * var_evb2e1) + (var_if0 * var_evb2e1_dn0)) * var_vtinv) * var_nff_t) - (assign6690_e6906 * var_nff_t_dn0)) / (var_nff_t * var_nff_t)) * assign6690_e6914) + (assign6690_e6908 * (-((0.5 * (var_f1_dn0 / (2.0 * assign6690_e6913))) / (assign6690_e6913 * assign6690_e6913))))), ((((((((var_if0_dn1 * var_evb2e1) + (var_if0 * var_evb2e1_dn1)) * var_vtinv) * var_nff_t) - (assign6690_e6906 * var_nff_t_dn1)) / (var_nff_t * var_nff_t)) * assign6690_e6914) + (assign6690_e6908 * (-((0.5 * (var_f1_dn1 / (2.0 * assign6690_e6913))) / (assign6690_e6913 * assign6690_e6913))))), ((((((((var_if0_dn2 * var_evb2e1) + (var_if0 * var_evb2e1_dn2)) * var_vtinv) * var_nff_t) - (assign6690_e6906 * var_nff_t_dn2)) / (var_nff_t * var_nff_t)) * assign6690_e6914) + (assign6690_e6908 * (-((0.5 * (var_f1_dn2 / (2.0 * assign6690_e6913))) / (assign6690_e6913 * assign6690_e6913))))), ((((((((var_if0_dn3 * var_evb2e1) + (var_if0 * var_evb2e1_dn3)) * var_vtinv) * var_nff_t) - (assign6690_e6906 * var_nff_t_dn3)) / (var_nff_t * var_nff_t)) * assign6690_e6914) + (assign6690_e6908 * (-((0.5 * (var_f1_dn3 / (2.0 * assign6690_e6913))) / (assign6690_e6913 * assign6690_e6913))))), ((((((((var_if0_dn4 * var_evb2e1) + (var_if0 * var_evb2e1_dn4)) * var_vtinv) * var_nff_t) - (assign6690_e6906 * var_nff_t_dn4)) / (var_nff_t * var_nff_t)) * assign6690_e6914) + (assign6690_e6908 * (-((0.5 * (var_f1_dn4 / (2.0 * assign6690_e6913))) / (assign6690_e6913 * assign6690_e6913))))), ((((((((var_if0_dn5 * var_evb2e1) + (var_if0 * var_evb2e1_dn5)) * var_vtinv) * var_nff_t) - (assign6690_e6906 * var_nff_t_dn5)) / (var_nff_t * var_nff_t)) * assign6690_e6914) + (assign6690_e6908 * (-((0.5 * (var_f1_dn5 / (2.0 * assign6690_e6913))) / (assign6690_e6913 * assign6690_e6913))))), ((((((((var_if0_dn6 * var_evb2e1) + (var_if0 * var_evb2e1_dn6)) * var_vtinv) * var_nff_t) - (assign6690_e6906 * var_nff_t_dn6)) / (var_nff_t * var_nff_t)) * assign6690_e6914) + (assign6690_e6908 * (-((0.5 * (var_f1_dn6 / (2.0 * assign6690_e6913))) / (assign6690_e6913 * assign6690_e6913))))), ((((((((var_if0_dn7 * var_evb2e1) + (var_if0 * var_evb2e1_dn7)) * var_vtinv) * var_nff_t) - (assign6690_e6906 * var_nff_t_dn7)) / (var_nff_t * var_nff_t)) * assign6690_e6914) + (assign6690_e6908 * (-((0.5 * (var_f1_dn7 / (2.0 * assign6690_e6913))) / (assign6690_e6913 * assign6690_e6913))))), ((((((((var_if0_dn8 * var_evb2e1) + (var_if0 * var_evb2e1_dn8)) * var_vtinv) * var_nff_t) - (assign6690_e6906 * var_nff_t_dn8)) / (var_nff_t * var_nff_t)) * assign6690_e6914) + (assign6690_e6908 * (-((0.5 * (var_f1_dn8 / (2.0 * assign6690_e6913))) / (assign6690_e6913 * assign6690_e6913))))), ((((((((var_if0_dn9 * var_evb2e1) + (var_if0 * var_evb2e1_dn9)) * var_vtinv) * var_nff_t) - (assign6690_e6906 * var_nff_t_dn9)) / (var_nff_t * var_nff_t)) * assign6690_e6914) + (assign6690_e6908 * (-((0.5 * (var_f1_dn9 / (2.0 * assign6690_e6913))) / (assign6690_e6913 * assign6690_e6913))))), ((((((((var_if0_dn10 * var_evb2e1) + (var_if0 * var_evb2e1_dn10)) * var_vtinv) * var_nff_t) - (assign6690_e6906 * var_nff_t_dn10)) / (var_nff_t * var_nff_t)) * assign6690_e6914) + (assign6690_e6908 * (-((0.5 * (var_f1_dn10 / (2.0 * assign6690_e6913))) / (assign6690_e6913 * assign6690_e6913))))), ((((((((var_if0_dn11 * var_evb2e1) + (var_if0 * var_evb2e1_dn11)) * var_vtinv) * var_nff_t) - (assign6690_e6906 * var_nff_t_dn11)) / (var_nff_t * var_nff_t)) * assign6690_e6914) + (assign6690_e6908 * (-((0.5 * (var_f1_dn11 / (2.0 * assign6690_e6913))) / (assign6690_e6913 * assign6690_e6913))))), ((((((((var_if0_db0 * var_evb2e1) + (var_if0 * var_evb2e1_db0)) * var_vtinv) * var_nff_t) - (assign6690_e6906 * var_nff_t_db0)) / (var_nff_t * var_nff_t)) * assign6690_e6914) + (assign6690_e6908 * (-((0.5 * (var_f1_db0 / (2.0 * assign6690_e6913))) / (assign6690_e6913 * assign6690_e6913))))), ((((((((var_if0_db1 * var_evb2e1) + (var_if0 * var_evb2e1_db1)) * var_vtinv) * var_nff_t) - (assign6690_e6906 * var_nff_t_db1)) / (var_nff_t * var_nff_t)) * assign6690_e6914) + (assign6690_e6908 * (-((0.5 * (var_f1_db1 / (2.0 * assign6690_e6913))) / (assign6690_e6913 * assign6690_e6913))))),)
    } else {
        (var_dn0vb2e1, var_dn0vb2e1_dn0, var_dn0vb2e1_dn1, var_dn0vb2e1_dn2, var_dn0vb2e1_dn3, var_dn0vb2e1_dn4, var_dn0vb2e1_dn5, var_dn0vb2e1_dn6, var_dn0vb2e1_dn7, var_dn0vb2e1_dn8, var_dn0vb2e1_dn9, var_dn0vb2e1_dn10, var_dn0vb2e1_dn11, var_dn0vb2e1_db0, var_dn0vb2e1_db1,)
    }
};
        var_dn0vb2e1 = assign6690_e6917;
        var_dn0vb2e1_dn0 = assign6690_e6917_d_n0;
        var_dn0vb2e1_dn1 = assign6690_e6917_d_n1;
        var_dn0vb2e1_dn2 = assign6690_e6917_d_n2;
        var_dn0vb2e1_dn3 = assign6690_e6917_d_n3;
        var_dn0vb2e1_dn4 = assign6690_e6917_d_n4;
        var_dn0vb2e1_dn5 = assign6690_e6917_d_n5;
        var_dn0vb2e1_dn6 = assign6690_e6917_d_n6;
        var_dn0vb2e1_dn7 = assign6690_e6917_d_n7;
        var_dn0vb2e1_dn8 = assign6690_e6917_d_n8;
        var_dn0vb2e1_dn9 = assign6690_e6917_d_n9;
        var_dn0vb2e1_dn10 = assign6690_e6917_d_n10;
        var_dn0vb2e1_dn11 = assign6690_e6917_d_n11;
        var_dn0vb2e1_db0 = assign6690_e6917_d_b0;
        var_dn0vb2e1_db1 = assign6690_e6917_d_b1;

        let (assign6700_e6927, assign6700_e6927_d_n0, assign6700_e6927_d_n1, assign6700_e6927_d_n2, assign6700_e6927_d_n3, assign6700_e6927_d_n4, assign6700_e6927_d_n5, assign6700_e6927_d_n6, assign6700_e6927_d_n7, assign6700_e6927_d_n8, assign6700_e6927_d_n9, assign6700_e6927_d_n10, assign6700_e6927_d_n11, assign6700_e6927_d_b0, assign6700_e6927_d_b1,) = {
    if (var_guard120 != 0.0) {
        let assign6700_e6921: f64 = (0.5 * var_qb0);
        let assign6700_e6923: f64 = (assign6700_e6921 * var_q1q);
        let assign6700_e6925: f64 = (assign6700_e6923 * var_dn0vb2e1);
        (assign6700_e6925, (((assign6700_e6921 * var_q1q_dn0) * var_dn0vb2e1) + (assign6700_e6923 * var_dn0vb2e1_dn0)), (((assign6700_e6921 * var_q1q_dn1) * var_dn0vb2e1) + (assign6700_e6923 * var_dn0vb2e1_dn1)), (((assign6700_e6921 * var_q1q_dn2) * var_dn0vb2e1) + (assign6700_e6923 * var_dn0vb2e1_dn2)), (((assign6700_e6921 * var_q1q_dn3) * var_dn0vb2e1) + (assign6700_e6923 * var_dn0vb2e1_dn3)), (((assign6700_e6921 * var_q1q_dn4) * var_dn0vb2e1) + (assign6700_e6923 * var_dn0vb2e1_dn4)), (((assign6700_e6921 * var_q1q_dn5) * var_dn0vb2e1) + (assign6700_e6923 * var_dn0vb2e1_dn5)), (((assign6700_e6921 * var_q1q_dn6) * var_dn0vb2e1) + (assign6700_e6923 * var_dn0vb2e1_dn6)), (((assign6700_e6921 * var_q1q_dn7) * var_dn0vb2e1) + (assign6700_e6923 * var_dn0vb2e1_dn7)), (((assign6700_e6921 * var_q1q_dn8) * var_dn0vb2e1) + (assign6700_e6923 * var_dn0vb2e1_dn8)), (((assign6700_e6921 * var_q1q_dn9) * var_dn0vb2e1) + (assign6700_e6923 * var_dn0vb2e1_dn9)), (((assign6700_e6921 * var_q1q_dn10) * var_dn0vb2e1) + (assign6700_e6923 * var_dn0vb2e1_dn10)), (((assign6700_e6921 * var_q1q_dn11) * var_dn0vb2e1) + (assign6700_e6923 * var_dn0vb2e1_dn11)), (((assign6700_e6921 * var_q1q_db0) * var_dn0vb2e1) + (assign6700_e6923 * var_dn0vb2e1_db0)), (((assign6700_e6921 * var_q1q_db1) * var_dn0vb2e1) + (assign6700_e6923 * var_dn0vb2e1_db1)),)
    } else {
        (var_dqbevb2e1, var_dqbevb2e1_dn0, var_dqbevb2e1_dn1, var_dqbevb2e1_dn2, var_dqbevb2e1_dn3, var_dqbevb2e1_dn4, var_dqbevb2e1_dn5, var_dqbevb2e1_dn6, var_dqbevb2e1_dn7, var_dqbevb2e1_dn8, var_dqbevb2e1_dn9, var_dqbevb2e1_dn10, var_dqbevb2e1_dn11, var_dqbevb2e1_db0, var_dqbevb2e1_db1,)
    }
};
        var_dqbevb2e1 = assign6700_e6927;
        var_dqbevb2e1_dn0 = assign6700_e6927_d_n0;
        var_dqbevb2e1_dn1 = assign6700_e6927_d_n1;
        var_dqbevb2e1_dn2 = assign6700_e6927_d_n2;
        var_dqbevb2e1_dn3 = assign6700_e6927_d_n3;
        var_dqbevb2e1_dn4 = assign6700_e6927_d_n4;
        var_dqbevb2e1_dn5 = assign6700_e6927_d_n5;
        var_dqbevb2e1_dn6 = assign6700_e6927_d_n6;
        var_dqbevb2e1_dn7 = assign6700_e6927_d_n7;
        var_dqbevb2e1_dn8 = assign6700_e6927_d_n8;
        var_dqbevb2e1_dn9 = assign6700_e6927_d_n9;
        var_dqbevb2e1_dn10 = assign6700_e6927_d_n10;
        var_dqbevb2e1_dn11 = assign6700_e6927_d_n11;
        var_dqbevb2e1_db0 = assign6700_e6927_d_b0;
        var_dqbevb2e1_db1 = assign6700_e6927_d_b1;

        let (assign6710_e6935, assign6710_e6935_d_n0, assign6710_e6935_d_n1, assign6710_e6935_d_n2, assign6710_e6935_d_n3, assign6710_e6935_d_n4, assign6710_e6935_d_n5, assign6710_e6935_d_n6, assign6710_e6935_d_n7, assign6710_e6935_d_n8, assign6710_e6935_d_n9, assign6710_e6935_d_n10, assign6710_e6935_d_n11, assign6710_e6935_d_b0, assign6710_e6935_d_b1,) = {
    if (var_guard120 != 0.0) {
        let assign6710_e6932: f64 = (p.p85 * var_vt);
        let assign6710_e6933: f64 = (var_qe_qs / assign6710_e6932);
        (assign6710_e6933, (var_qe_qs_dn0 / assign6710_e6932), (var_qe_qs_dn1 / assign6710_e6932), (var_qe_qs_dn2 / assign6710_e6932), (var_qe_qs_dn3 / assign6710_e6932), (var_qe_qs_dn4 / assign6710_e6932), (var_qe_qs_dn5 / assign6710_e6932), (var_qe_qs_dn6 / assign6710_e6932), (var_qe_qs_dn7 / assign6710_e6932), (var_qe_qs_dn8 / assign6710_e6932), (var_qe_qs_dn9 / assign6710_e6932), (var_qe_qs_dn10 / assign6710_e6932), (var_qe_qs_dn11 / assign6710_e6932), (var_qe_qs_db0 / assign6710_e6932), (var_qe_qs_db1 / assign6710_e6932),)
    } else {
        (var_dqevb2e1, var_dqevb2e1_dn0, var_dqevb2e1_dn1, var_dqevb2e1_dn2, var_dqevb2e1_dn3, var_dqevb2e1_dn4, var_dqevb2e1_dn5, var_dqevb2e1_dn6, var_dqevb2e1_dn7, var_dqevb2e1_dn8, var_dqevb2e1_dn9, var_dqevb2e1_dn10, var_dqevb2e1_dn11, var_dqevb2e1_db0, var_dqevb2e1_db1,)
    }
};
        var_dqevb2e1 = assign6710_e6935;
        var_dqevb2e1_dn0 = assign6710_e6935_d_n0;
        var_dqevb2e1_dn1 = assign6710_e6935_d_n1;
        var_dqevb2e1_dn2 = assign6710_e6935_d_n2;
        var_dqevb2e1_dn3 = assign6710_e6935_d_n3;
        var_dqevb2e1_dn4 = assign6710_e6935_d_n4;
        var_dqevb2e1_dn5 = assign6710_e6935_d_n5;
        var_dqevb2e1_dn6 = assign6710_e6935_d_n6;
        var_dqevb2e1_dn7 = assign6710_e6935_d_n7;
        var_dqevb2e1_dn8 = assign6710_e6935_d_n8;
        var_dqevb2e1_dn9 = assign6710_e6935_d_n9;
        var_dqevb2e1_dn10 = assign6710_e6935_d_n10;
        var_dqevb2e1_dn11 = assign6710_e6935_d_n11;
        var_dqevb2e1_db0 = assign6710_e6935_d_b0;
        var_dqevb2e1_db1 = assign6710_e6935_d_b1;

        let (assign6720_e6947, assign6720_e6947_d_n0, assign6720_e6947_d_n1, assign6720_e6947_d_n2, assign6720_e6947_d_n3, assign6720_e6947_d_n4, assign6720_e6947_d_n5, assign6720_e6947_d_n6, assign6720_e6947_d_n7, assign6720_e6947_d_n8, assign6720_e6947_d_n9, assign6720_e6947_d_n10, assign6720_e6947_d_n11, assign6720_e6947_d_b0, assign6720_e6947_d_b1,) = {
    if (var_guard120 != 0.0) {
        let assign6720_e6939: f64 = (0.2 * var_vb1b2);
        let assign6720_e6942: f64 = (var_dqtevb2e1 + var_dqbevb2e1);
        let assign6720_e6944: f64 = (assign6720_e6942 + var_dqevb2e1);
        let assign6720_e6945: f64 = (assign6720_e6939 * assign6720_e6944);
        (assign6720_e6945, (((0.2 * var_vb1b2_dn0) * assign6720_e6944) + (assign6720_e6939 * ((var_dqtevb2e1_dn0 + var_dqbevb2e1_dn0) + var_dqevb2e1_dn0))), (((0.2 * var_vb1b2_dn1) * assign6720_e6944) + (assign6720_e6939 * ((var_dqtevb2e1_dn1 + var_dqbevb2e1_dn1) + var_dqevb2e1_dn1))), (((0.2 * var_vb1b2_dn2) * assign6720_e6944) + (assign6720_e6939 * ((var_dqtevb2e1_dn2 + var_dqbevb2e1_dn2) + var_dqevb2e1_dn2))), (((0.2 * var_vb1b2_dn3) * assign6720_e6944) + (assign6720_e6939 * ((var_dqtevb2e1_dn3 + var_dqbevb2e1_dn3) + var_dqevb2e1_dn3))), (((0.2 * var_vb1b2_dn4) * assign6720_e6944) + (assign6720_e6939 * ((var_dqtevb2e1_dn4 + var_dqbevb2e1_dn4) + var_dqevb2e1_dn4))), (((0.2 * var_vb1b2_dn5) * assign6720_e6944) + (assign6720_e6939 * ((var_dqtevb2e1_dn5 + var_dqbevb2e1_dn5) + var_dqevb2e1_dn5))), (((0.2 * var_vb1b2_dn6) * assign6720_e6944) + (assign6720_e6939 * ((var_dqtevb2e1_dn6 + var_dqbevb2e1_dn6) + var_dqevb2e1_dn6))), (((0.2 * var_vb1b2_dn7) * assign6720_e6944) + (assign6720_e6939 * ((var_dqtevb2e1_dn7 + var_dqbevb2e1_dn7) + var_dqevb2e1_dn7))), (((0.2 * var_vb1b2_dn8) * assign6720_e6944) + (assign6720_e6939 * ((var_dqtevb2e1_dn8 + var_dqbevb2e1_dn8) + var_dqevb2e1_dn8))), (((0.2 * var_vb1b2_dn9) * assign6720_e6944) + (assign6720_e6939 * ((var_dqtevb2e1_dn9 + var_dqbevb2e1_dn9) + var_dqevb2e1_dn9))), (((0.2 * var_vb1b2_dn10) * assign6720_e6944) + (assign6720_e6939 * ((var_dqtevb2e1_dn10 + var_dqbevb2e1_dn10) + var_dqevb2e1_dn10))), (((0.2 * var_vb1b2_dn11) * assign6720_e6944) + (assign6720_e6939 * ((var_dqtevb2e1_dn11 + var_dqbevb2e1_dn11) + var_dqevb2e1_dn11))), (((0.2 * var_vb1b2_db0) * assign6720_e6944) + (assign6720_e6939 * ((var_dqtevb2e1_db0 + var_dqbevb2e1_db0) + var_dqevb2e1_db0))), (((0.2 * var_vb1b2_db1) * assign6720_e6944) + (assign6720_e6939 * ((var_dqtevb2e1_db1 + var_dqbevb2e1_db1) + var_dqevb2e1_db1))),)
    } else {
        (var_qb1b2, var_qb1b2_dn0, var_qb1b2_dn1, var_qb1b2_dn2, var_qb1b2_dn3, var_qb1b2_dn4, var_qb1b2_dn5, var_qb1b2_dn6, var_qb1b2_dn7, var_qb1b2_dn8, var_qb1b2_dn9, var_qb1b2_dn10, var_qb1b2_dn11, var_qb1b2_db0, var_qb1b2_db1,)
    }
};
        var_qb1b2 = assign6720_e6947;
        var_qb1b2_dn0 = assign6720_e6947_d_n0;
        var_qb1b2_dn1 = assign6720_e6947_d_n1;
        var_qb1b2_dn2 = assign6720_e6947_d_n2;
        var_qb1b2_dn3 = assign6720_e6947_d_n3;
        var_qb1b2_dn4 = assign6720_e6947_d_n4;
        var_qb1b2_dn5 = assign6720_e6947_d_n5;
        var_qb1b2_dn6 = assign6720_e6947_d_n6;
        var_qb1b2_dn7 = assign6720_e6947_d_n7;
        var_qb1b2_dn8 = assign6720_e6947_d_n8;
        var_qb1b2_dn9 = assign6720_e6947_d_n9;
        var_qb1b2_dn10 = assign6720_e6947_d_n10;
        var_qb1b2_dn11 = assign6720_e6947_d_n11;
        var_qb1b2_db0 = assign6720_e6947_d_b0;
        var_qb1b2_db1 = assign6720_e6947_d_b1;

        let (assign6730_e6955, assign6730_e6955_d_n0, assign6730_e6955_d_n1, assign6730_e6955_d_n2, assign6730_e6955_d_n3, assign6730_e6955_d_n4, assign6730_e6955_d_n5, assign6730_e6955_d_n6, assign6730_e6955_d_n7, assign6730_e6955_d_n8, assign6730_e6955_d_n9, assign6730_e6955_d_n10, assign6730_e6955_d_n11, assign6730_e6955_d_b0, assign6730_e6955_d_b1,) = {
    if (var_guard120 != 0.0) {
        let assign6730_e6951: f64 = (1.0 - p.p95);
        let assign6730_e6953: f64 = (assign6730_e6951 * var_qe_qs);
        (assign6730_e6953, (assign6730_e6951 * var_qe_qs_dn0), (assign6730_e6951 * var_qe_qs_dn1), (assign6730_e6951 * var_qe_qs_dn2), (assign6730_e6951 * var_qe_qs_dn3), (assign6730_e6951 * var_qe_qs_dn4), (assign6730_e6951 * var_qe_qs_dn5), (assign6730_e6951 * var_qe_qs_dn6), (assign6730_e6951 * var_qe_qs_dn7), (assign6730_e6951 * var_qe_qs_dn8), (assign6730_e6951 * var_qe_qs_dn9), (assign6730_e6951 * var_qe_qs_dn10), (assign6730_e6951 * var_qe_qs_dn11), (assign6730_e6951 * var_qe_qs_db0), (assign6730_e6951 * var_qe_qs_db1),)
    } else {
        (var_qe, var_qe_dn0, var_qe_dn1, var_qe_dn2, var_qe_dn3, var_qe_dn4, var_qe_dn5, var_qe_dn6, var_qe_dn7, var_qe_dn8, var_qe_dn9, var_qe_dn10, var_qe_dn11, var_qe_db0, var_qe_db1,)
    }
};
        var_qe = assign6730_e6955;
        var_qe_dn0 = assign6730_e6955_d_n0;
        var_qe_dn1 = assign6730_e6955_d_n1;
        var_qe_dn2 = assign6730_e6955_d_n2;
        var_qe_dn3 = assign6730_e6955_d_n3;
        var_qe_dn4 = assign6730_e6955_d_n4;
        var_qe_dn5 = assign6730_e6955_d_n5;
        var_qe_dn6 = assign6730_e6955_d_n6;
        var_qe_dn7 = assign6730_e6955_d_n7;
        var_qe_dn8 = assign6730_e6955_d_n8;
        var_qe_dn9 = assign6730_e6955_d_n9;
        var_qe_dn10 = assign6730_e6955_d_n10;
        var_qe_dn11 = assign6730_e6955_d_n11;
        var_qe_db0 = assign6730_e6955_d_b0;
        var_qe_db1 = assign6730_e6955_d_b1;

        let (assign6740_e6963, assign6740_e6963_d_n0, assign6740_e6963_d_n1, assign6740_e6963_d_n2, assign6740_e6963_d_n3, assign6740_e6963_d_n4, assign6740_e6963_d_n5, assign6740_e6963_d_n6, assign6740_e6963_d_n7, assign6740_e6963_d_n8, assign6740_e6963_d_n9, assign6740_e6963_d_n10, assign6740_e6963_d_n11, assign6740_e6963_d_b0, assign6740_e6963_d_b1,) = {
    if (var_guard120 != 0.0) {
        let assign6740_e6960: f64 = (p.p95 * var_qe_qs);
        let assign6740_e6961: f64 = (var_qbe_qs + assign6740_e6960);
        (assign6740_e6961, (var_qbe_qs_dn0 + (p.p95 * var_qe_qs_dn0)), (var_qbe_qs_dn1 + (p.p95 * var_qe_qs_dn1)), (var_qbe_qs_dn2 + (p.p95 * var_qe_qs_dn2)), (var_qbe_qs_dn3 + (p.p95 * var_qe_qs_dn3)), (var_qbe_qs_dn4 + (p.p95 * var_qe_qs_dn4)), (var_qbe_qs_dn5 + (p.p95 * var_qe_qs_dn5)), (var_qbe_qs_dn6 + (p.p95 * var_qe_qs_dn6)), (var_qbe_qs_dn7 + (p.p95 * var_qe_qs_dn7)), (var_qbe_qs_dn8 + (p.p95 * var_qe_qs_dn8)), (var_qbe_qs_dn9 + (p.p95 * var_qe_qs_dn9)), (var_qbe_qs_dn10 + (p.p95 * var_qe_qs_dn10)), (var_qbe_qs_dn11 + (p.p95 * var_qe_qs_dn11)), (var_qbe_qs_db0 + (p.p95 * var_qe_qs_db0)), (var_qbe_qs_db1 + (p.p95 * var_qe_qs_db1)),)
    } else {
        (var_qbe_qs_eff, var_qbe_qs_eff_dn0, var_qbe_qs_eff_dn1, var_qbe_qs_eff_dn2, var_qbe_qs_eff_dn3, var_qbe_qs_eff_dn4, var_qbe_qs_eff_dn5, var_qbe_qs_eff_dn6, var_qbe_qs_eff_dn7, var_qbe_qs_eff_dn8, var_qbe_qs_eff_dn9, var_qbe_qs_eff_dn10, var_qbe_qs_eff_dn11, var_qbe_qs_eff_db0, var_qbe_qs_eff_db1,)
    }
};
        var_qbe_qs_eff = assign6740_e6963;
        var_qbe_qs_eff_dn0 = assign6740_e6963_d_n0;
        var_qbe_qs_eff_dn1 = assign6740_e6963_d_n1;
        var_qbe_qs_eff_dn2 = assign6740_e6963_d_n2;
        var_qbe_qs_eff_dn3 = assign6740_e6963_d_n3;
        var_qbe_qs_eff_dn4 = assign6740_e6963_d_n4;
        var_qbe_qs_eff_dn5 = assign6740_e6963_d_n5;
        var_qbe_qs_eff_dn6 = assign6740_e6963_d_n6;
        var_qbe_qs_eff_dn7 = assign6740_e6963_d_n7;
        var_qbe_qs_eff_dn8 = assign6740_e6963_d_n8;
        var_qbe_qs_eff_dn9 = assign6740_e6963_d_n9;
        var_qbe_qs_eff_dn10 = assign6740_e6963_d_n10;
        var_qbe_qs_eff_dn11 = assign6740_e6963_d_n11;
        var_qbe_qs_eff_db0 = assign6740_e6963_d_b0;
        var_qbe_qs_eff_db1 = assign6740_e6963_d_b1;

        let (assign6750_e6971, assign6750_e6971_d_n0, assign6750_e6971_d_n1, assign6750_e6971_d_n2, assign6750_e6971_d_n3, assign6750_e6971_d_n4, assign6750_e6971_d_n5, assign6750_e6971_d_n6, assign6750_e6971_d_n7, assign6750_e6971_d_n8, assign6750_e6971_d_n9, assign6750_e6971_d_n10, assign6750_e6971_d_n11, assign6750_e6971_d_b0, assign6750_e6971_d_b1,) = {
    if (var_guard120 != 0.0) {
        let assign6750_e6967: f64 = (p.p94 * var_qbe_qs_eff);
        let assign6750_e6969: f64 = (assign6750_e6967 + var_qbc_qs);
        (assign6750_e6969, ((p.p94 * var_qbe_qs_eff_dn0) + var_qbc_qs_dn0), ((p.p94 * var_qbe_qs_eff_dn1) + var_qbc_qs_dn1), ((p.p94 * var_qbe_qs_eff_dn2) + var_qbc_qs_dn2), ((p.p94 * var_qbe_qs_eff_dn3) + var_qbc_qs_dn3), ((p.p94 * var_qbe_qs_eff_dn4) + var_qbc_qs_dn4), ((p.p94 * var_qbe_qs_eff_dn5) + var_qbc_qs_dn5), ((p.p94 * var_qbe_qs_eff_dn6) + var_qbc_qs_dn6), ((p.p94 * var_qbe_qs_eff_dn7) + var_qbc_qs_dn7), ((p.p94 * var_qbe_qs_eff_dn8) + var_qbc_qs_dn8), ((p.p94 * var_qbe_qs_eff_dn9) + var_qbc_qs_dn9), ((p.p94 * var_qbe_qs_eff_dn10) + var_qbc_qs_dn10), ((p.p94 * var_qbe_qs_eff_dn11) + var_qbc_qs_dn11), ((p.p94 * var_qbe_qs_eff_db0) + var_qbc_qs_db0), ((p.p94 * var_qbe_qs_eff_db1) + var_qbc_qs_db1),)
    } else {
        (var_qbc, var_qbc_dn0, var_qbc_dn1, var_qbc_dn2, var_qbc_dn3, var_qbc_dn4, var_qbc_dn5, var_qbc_dn6, var_qbc_dn7, var_qbc_dn8, var_qbc_dn9, var_qbc_dn10, var_qbc_dn11, var_qbc_db0, var_qbc_db1,)
    }
};
        var_qbc = assign6750_e6971;
        var_qbc_dn0 = assign6750_e6971_d_n0;
        var_qbc_dn1 = assign6750_e6971_d_n1;
        var_qbc_dn2 = assign6750_e6971_d_n2;
        var_qbc_dn3 = assign6750_e6971_d_n3;
        var_qbc_dn4 = assign6750_e6971_d_n4;
        var_qbc_dn5 = assign6750_e6971_d_n5;
        var_qbc_dn6 = assign6750_e6971_d_n6;
        var_qbc_dn7 = assign6750_e6971_d_n7;
        var_qbc_dn8 = assign6750_e6971_d_n8;
        var_qbc_dn9 = assign6750_e6971_d_n9;
        var_qbc_dn10 = assign6750_e6971_d_n10;
        var_qbc_dn11 = assign6750_e6971_d_n11;
        var_qbc_db0 = assign6750_e6971_d_b0;
        var_qbc_db1 = assign6750_e6971_d_b1;

        let (assign6760_e6979, assign6760_e6979_d_n0, assign6760_e6979_d_n1, assign6760_e6979_d_n2, assign6760_e6979_d_n3, assign6760_e6979_d_n4, assign6760_e6979_d_n5, assign6760_e6979_d_n6, assign6760_e6979_d_n7, assign6760_e6979_d_n8, assign6760_e6979_d_n9, assign6760_e6979_d_n10, assign6760_e6979_d_n11, assign6760_e6979_d_b0, assign6760_e6979_d_b1,) = {
    if (var_guard120 != 0.0) {
        let assign6760_e6975: f64 = (1.0 - p.p94);
        let assign6760_e6977: f64 = (assign6760_e6975 * var_qbe_qs_eff);
        (assign6760_e6977, (assign6760_e6975 * var_qbe_qs_eff_dn0), (assign6760_e6975 * var_qbe_qs_eff_dn1), (assign6760_e6975 * var_qbe_qs_eff_dn2), (assign6760_e6975 * var_qbe_qs_eff_dn3), (assign6760_e6975 * var_qbe_qs_eff_dn4), (assign6760_e6975 * var_qbe_qs_eff_dn5), (assign6760_e6975 * var_qbe_qs_eff_dn6), (assign6760_e6975 * var_qbe_qs_eff_dn7), (assign6760_e6975 * var_qbe_qs_eff_dn8), (assign6760_e6975 * var_qbe_qs_eff_dn9), (assign6760_e6975 * var_qbe_qs_eff_dn10), (assign6760_e6975 * var_qbe_qs_eff_dn11), (assign6760_e6975 * var_qbe_qs_eff_db0), (assign6760_e6975 * var_qbe_qs_eff_db1),)
    } else {
        (var_qbe, var_qbe_dn0, var_qbe_dn1, var_qbe_dn2, var_qbe_dn3, var_qbe_dn4, var_qbe_dn5, var_qbe_dn6, var_qbe_dn7, var_qbe_dn8, var_qbe_dn9, var_qbe_dn10, var_qbe_dn11, var_qbe_db0, var_qbe_db1,)
    }
};
        var_qbe = assign6760_e6979;
        var_qbe_dn0 = assign6760_e6979_d_n0;
        var_qbe_dn1 = assign6760_e6979_d_n1;
        var_qbe_dn2 = assign6760_e6979_d_n2;
        var_qbe_dn3 = assign6760_e6979_d_n3;
        var_qbe_dn4 = assign6760_e6979_d_n4;
        var_qbe_dn5 = assign6760_e6979_d_n5;
        var_qbe_dn6 = assign6760_e6979_d_n6;
        var_qbe_dn7 = assign6760_e6979_d_n7;
        var_qbe_dn8 = assign6760_e6979_d_n8;
        var_qbe_dn9 = assign6760_e6979_d_n9;
        var_qbe_dn10 = assign6760_e6979_d_n10;
        var_qbe_dn11 = assign6760_e6979_d_n11;
        var_qbe_db0 = assign6760_e6979_d_b0;
        var_qbe_db1 = assign6760_e6979_d_b1;

        let (assign6770_e6984, assign6770_e6984_d_n0, assign6770_e6984_d_n1, assign6770_e6984_d_n2, assign6770_e6984_d_n3, assign6770_e6984_d_n4, assign6770_e6984_d_n5, assign6770_e6984_d_n6, assign6770_e6984_d_n7, assign6770_e6984_d_n8, assign6770_e6984_d_n9, assign6770_e6984_d_n10, assign6770_e6984_d_n11, assign6770_e6984_d_b0, assign6770_e6984_d_b1,) = {
    if (var_guard120 == 0.0) {
        (var_qbe_qs, var_qbe_qs_dn0, var_qbe_qs_dn1, var_qbe_qs_dn2, var_qbe_qs_dn3, var_qbe_qs_dn4, var_qbe_qs_dn5, var_qbe_qs_dn6, var_qbe_qs_dn7, var_qbe_qs_dn8, var_qbe_qs_dn9, var_qbe_qs_dn10, var_qbe_qs_dn11, var_qbe_qs_db0, var_qbe_qs_db1,)
    } else {
        (var_qbe, var_qbe_dn0, var_qbe_dn1, var_qbe_dn2, var_qbe_dn3, var_qbe_dn4, var_qbe_dn5, var_qbe_dn6, var_qbe_dn7, var_qbe_dn8, var_qbe_dn9, var_qbe_dn10, var_qbe_dn11, var_qbe_db0, var_qbe_db1,)
    }
};
        var_qbe = assign6770_e6984;
        var_qbe_dn0 = assign6770_e6984_d_n0;
        var_qbe_dn1 = assign6770_e6984_d_n1;
        var_qbe_dn2 = assign6770_e6984_d_n2;
        var_qbe_dn3 = assign6770_e6984_d_n3;
        var_qbe_dn4 = assign6770_e6984_d_n4;
        var_qbe_dn5 = assign6770_e6984_d_n5;
        var_qbe_dn6 = assign6770_e6984_d_n6;
        var_qbe_dn7 = assign6770_e6984_d_n7;
        var_qbe_dn8 = assign6770_e6984_d_n8;
        var_qbe_dn9 = assign6770_e6984_d_n9;
        var_qbe_dn10 = assign6770_e6984_d_n10;
        var_qbe_dn11 = assign6770_e6984_d_n11;
        var_qbe_db0 = assign6770_e6984_d_b0;
        var_qbe_db1 = assign6770_e6984_d_b1;

        let (assign6780_e6989, assign6780_e6989_d_n0, assign6780_e6989_d_n1, assign6780_e6989_d_n2, assign6780_e6989_d_n3, assign6780_e6989_d_n4, assign6780_e6989_d_n5, assign6780_e6989_d_n6, assign6780_e6989_d_n7, assign6780_e6989_d_n8, assign6780_e6989_d_n9, assign6780_e6989_d_n10, assign6780_e6989_d_n11, assign6780_e6989_d_b0, assign6780_e6989_d_b1,) = {
    if (var_guard120 == 0.0) {
        (var_qbc_qs, var_qbc_qs_dn0, var_qbc_qs_dn1, var_qbc_qs_dn2, var_qbc_qs_dn3, var_qbc_qs_dn4, var_qbc_qs_dn5, var_qbc_qs_dn6, var_qbc_qs_dn7, var_qbc_qs_dn8, var_qbc_qs_dn9, var_qbc_qs_dn10, var_qbc_qs_dn11, var_qbc_qs_db0, var_qbc_qs_db1,)
    } else {
        (var_qbc, var_qbc_dn0, var_qbc_dn1, var_qbc_dn2, var_qbc_dn3, var_qbc_dn4, var_qbc_dn5, var_qbc_dn6, var_qbc_dn7, var_qbc_dn8, var_qbc_dn9, var_qbc_dn10, var_qbc_dn11, var_qbc_db0, var_qbc_db1,)
    }
};
        var_qbc = assign6780_e6989;
        var_qbc_dn0 = assign6780_e6989_d_n0;
        var_qbc_dn1 = assign6780_e6989_d_n1;
        var_qbc_dn2 = assign6780_e6989_d_n2;
        var_qbc_dn3 = assign6780_e6989_d_n3;
        var_qbc_dn4 = assign6780_e6989_d_n4;
        var_qbc_dn5 = assign6780_e6989_d_n5;
        var_qbc_dn6 = assign6780_e6989_d_n6;
        var_qbc_dn7 = assign6780_e6989_d_n7;
        var_qbc_dn8 = assign6780_e6989_d_n8;
        var_qbc_dn9 = assign6780_e6989_d_n9;
        var_qbc_dn10 = assign6780_e6989_d_n10;
        var_qbc_dn11 = assign6780_e6989_d_n11;
        var_qbc_db0 = assign6780_e6989_d_b0;
        var_qbc_db1 = assign6780_e6989_d_b1;

        let (assign6790_e6994, assign6790_e6994_d_n0, assign6790_e6994_d_n1, assign6790_e6994_d_n2, assign6790_e6994_d_n3, assign6790_e6994_d_n4, assign6790_e6994_d_n5, assign6790_e6994_d_n6, assign6790_e6994_d_n7, assign6790_e6994_d_n8, assign6790_e6994_d_n9, assign6790_e6994_d_n10, assign6790_e6994_d_n11, assign6790_e6994_d_b0, assign6790_e6994_d_b1,) = {
    if (var_guard120 == 0.0) {
        (var_qe_qs, var_qe_qs_dn0, var_qe_qs_dn1, var_qe_qs_dn2, var_qe_qs_dn3, var_qe_qs_dn4, var_qe_qs_dn5, var_qe_qs_dn6, var_qe_qs_dn7, var_qe_qs_dn8, var_qe_qs_dn9, var_qe_qs_dn10, var_qe_qs_dn11, var_qe_qs_db0, var_qe_qs_db1,)
    } else {
        (var_qe, var_qe_dn0, var_qe_dn1, var_qe_dn2, var_qe_dn3, var_qe_dn4, var_qe_dn5, var_qe_dn6, var_qe_dn7, var_qe_dn8, var_qe_dn9, var_qe_dn10, var_qe_dn11, var_qe_db0, var_qe_db1,)
    }
};
        var_qe = assign6790_e6994;
        var_qe_dn0 = assign6790_e6994_d_n0;
        var_qe_dn1 = assign6790_e6994_d_n1;
        var_qe_dn2 = assign6790_e6994_d_n2;
        var_qe_dn3 = assign6790_e6994_d_n3;
        var_qe_dn4 = assign6790_e6994_d_n4;
        var_qe_dn5 = assign6790_e6994_d_n5;
        var_qe_dn6 = assign6790_e6994_d_n6;
        var_qe_dn7 = assign6790_e6994_d_n7;
        var_qe_dn8 = assign6790_e6994_d_n8;
        var_qe_dn9 = assign6790_e6994_d_n9;
        var_qe_dn10 = assign6790_e6994_d_n10;
        var_qe_dn11 = assign6790_e6994_d_n11;
        var_qe_db0 = assign6790_e6994_d_b0;
        var_qe_db1 = assign6790_e6994_d_b1;

        let assign6900_e7037: f64 = (var_if_ + var_ir);
        let assign6900_e7039: f64 = (assign6900_e7037 / var_qbi);
        var_in_n = assign6900_e7039;
        var_in_n_dn0 = ((((var_if__dn0 + var_ir_dn0) * var_qbi) - (assign6900_e7037 * var_qbi_dn0)) / (var_qbi * var_qbi));
        var_in_n_dn1 = ((((var_if__dn1 + var_ir_dn1) * var_qbi) - (assign6900_e7037 * var_qbi_dn1)) / (var_qbi * var_qbi));
        var_in_n_dn2 = ((((var_if__dn2 + var_ir_dn2) * var_qbi) - (assign6900_e7037 * var_qbi_dn2)) / (var_qbi * var_qbi));
        var_in_n_dn3 = ((((var_if__dn3 + var_ir_dn3) * var_qbi) - (assign6900_e7037 * var_qbi_dn3)) / (var_qbi * var_qbi));
        var_in_n_dn4 = ((((var_if__dn4 + var_ir_dn4) * var_qbi) - (assign6900_e7037 * var_qbi_dn4)) / (var_qbi * var_qbi));
        var_in_n_dn5 = ((((var_if__dn5 + var_ir_dn5) * var_qbi) - (assign6900_e7037 * var_qbi_dn5)) / (var_qbi * var_qbi));
        var_in_n_dn6 = ((((var_if__dn6 + var_ir_dn6) * var_qbi) - (assign6900_e7037 * var_qbi_dn6)) / (var_qbi * var_qbi));
        var_in_n_dn7 = ((((var_if__dn7 + var_ir_dn7) * var_qbi) - (assign6900_e7037 * var_qbi_dn7)) / (var_qbi * var_qbi));
        var_in_n_dn8 = ((((var_if__dn8 + var_ir_dn8) * var_qbi) - (assign6900_e7037 * var_qbi_dn8)) / (var_qbi * var_qbi));
        var_in_n_dn9 = ((((var_if__dn9 + var_ir_dn9) * var_qbi) - (assign6900_e7037 * var_qbi_dn9)) / (var_qbi * var_qbi));
        var_in_n_dn10 = ((((var_if__dn10 + var_ir_dn10) * var_qbi) - (assign6900_e7037 * var_qbi_dn10)) / (var_qbi * var_qbi));
        var_in_n_dn11 = ((((var_if__dn11 + var_ir_dn11) * var_qbi) - (assign6900_e7037 * var_qbi_dn11)) / (var_qbi * var_qbi));
        var_in_n_db0 = ((((var_if__db0 + var_ir_db0) * var_qbi) - (assign6900_e7037 * var_qbi_db0)) / (var_qbi * var_qbi));
        var_in_n_db1 = ((((var_if__db1 + var_ir_db1) * var_qbi) - (assign6900_e7037 * var_qbi_db1)) / (var_qbi * var_qbi));

        let assign6960_e7072: f64 = if var_in_n > 0.0 { 1.0 } else { 0.0 };
        var_guard126 = assign6960_e7072;

        let (assign6970_e7080, assign6970_e7080_d_n0, assign6970_e7080_d_n1, assign6970_e7080_d_n2, assign6970_e7080_d_n3, assign6970_e7080_d_n4, assign6970_e7080_d_n5, assign6970_e7080_d_n6, assign6970_e7080_d_n7, assign6970_e7080_d_n8, assign6970_e7080_d_n9, assign6970_e7080_d_n10, assign6970_e7080_d_n11, assign6970_e7080_d_b0, assign6970_e7080_d_b1,) = {
    if (var_guard126 != 0.0) {
        let assign6970_e7076: f64 = (var_qbe + var_qbc);
        let assign6970_e7078: f64 = (assign6970_e7076 / var_in_n);
        (assign6970_e7078, ((((var_qbe_dn0 + var_qbc_dn0) * var_in_n) - (assign6970_e7076 * var_in_n_dn0)) / (var_in_n * var_in_n)), ((((var_qbe_dn1 + var_qbc_dn1) * var_in_n) - (assign6970_e7076 * var_in_n_dn1)) / (var_in_n * var_in_n)), ((((var_qbe_dn2 + var_qbc_dn2) * var_in_n) - (assign6970_e7076 * var_in_n_dn2)) / (var_in_n * var_in_n)), ((((var_qbe_dn3 + var_qbc_dn3) * var_in_n) - (assign6970_e7076 * var_in_n_dn3)) / (var_in_n * var_in_n)), ((((var_qbe_dn4 + var_qbc_dn4) * var_in_n) - (assign6970_e7076 * var_in_n_dn4)) / (var_in_n * var_in_n)), ((((var_qbe_dn5 + var_qbc_dn5) * var_in_n) - (assign6970_e7076 * var_in_n_dn5)) / (var_in_n * var_in_n)), ((((var_qbe_dn6 + var_qbc_dn6) * var_in_n) - (assign6970_e7076 * var_in_n_dn6)) / (var_in_n * var_in_n)), ((((var_qbe_dn7 + var_qbc_dn7) * var_in_n) - (assign6970_e7076 * var_in_n_dn7)) / (var_in_n * var_in_n)), ((((var_qbe_dn8 + var_qbc_dn8) * var_in_n) - (assign6970_e7076 * var_in_n_dn8)) / (var_in_n * var_in_n)), ((((var_qbe_dn9 + var_qbc_dn9) * var_in_n) - (assign6970_e7076 * var_in_n_dn9)) / (var_in_n * var_in_n)), ((((var_qbe_dn10 + var_qbc_dn10) * var_in_n) - (assign6970_e7076 * var_in_n_dn10)) / (var_in_n * var_in_n)), ((((var_qbe_dn11 + var_qbc_dn11) * var_in_n) - (assign6970_e7076 * var_in_n_dn11)) / (var_in_n * var_in_n)), ((((var_qbe_db0 + var_qbc_db0) * var_in_n) - (assign6970_e7076 * var_in_n_db0)) / (var_in_n * var_in_n)), ((((var_qbe_db1 + var_qbc_db1) * var_in_n) - (assign6970_e7076 * var_in_n_db1)) / (var_in_n * var_in_n)),)
    } else {
        (var_taub_n, var_taub_n_dn0, var_taub_n_dn1, var_taub_n_dn2, var_taub_n_dn3, var_taub_n_dn4, var_taub_n_dn5, var_taub_n_dn6, var_taub_n_dn7, var_taub_n_dn8, var_taub_n_dn9, var_taub_n_dn10, var_taub_n_dn11, var_taub_n_db0, var_taub_n_db1,)
    }
};
        var_taub_n = assign6970_e7080;
        var_taub_n_dn0 = assign6970_e7080_d_n0;
        var_taub_n_dn1 = assign6970_e7080_d_n1;
        var_taub_n_dn2 = assign6970_e7080_d_n2;
        var_taub_n_dn3 = assign6970_e7080_d_n3;
        var_taub_n_dn4 = assign6970_e7080_d_n4;
        var_taub_n_dn5 = assign6970_e7080_d_n5;
        var_taub_n_dn6 = assign6970_e7080_d_n6;
        var_taub_n_dn7 = assign6970_e7080_d_n7;
        var_taub_n_dn8 = assign6970_e7080_d_n8;
        var_taub_n_dn9 = assign6970_e7080_d_n9;
        var_taub_n_dn10 = assign6970_e7080_d_n10;
        var_taub_n_dn11 = assign6970_e7080_d_n11;
        var_taub_n_db0 = assign6970_e7080_d_b0;
        var_taub_n_db1 = assign6970_e7080_d_b1;

        let (assign6980_e7089, assign6980_e7089_d_n0, assign6980_e7089_d_n1, assign6980_e7089_d_n2, assign6980_e7089_d_n3, assign6980_e7089_d_n4, assign6980_e7089_d_n5, assign6980_e7089_d_n6, assign6980_e7089_d_n7, assign6980_e7089_d_n8, assign6980_e7089_d_n9, assign6980_e7089_d_n10, assign6980_e7089_d_n11, assign6980_e7089_d_b0, assign6980_e7089_d_b1,) = {
    if (var_guard126 == 0.0) {
        let assign6980_e7085: f64 = (var_taub_t * var_q1q);
        let assign6980_e7087: f64 = (assign6980_e7085 * var_qbi);
        (assign6980_e7087, (((var_taub_t * var_q1q_dn0) * var_qbi) + (assign6980_e7085 * var_qbi_dn0)), (((var_taub_t * var_q1q_dn1) * var_qbi) + (assign6980_e7085 * var_qbi_dn1)), (((var_taub_t * var_q1q_dn2) * var_qbi) + (assign6980_e7085 * var_qbi_dn2)), (((var_taub_t * var_q1q_dn3) * var_qbi) + (assign6980_e7085 * var_qbi_dn3)), (((var_taub_t * var_q1q_dn4) * var_qbi) + (assign6980_e7085 * var_qbi_dn4)), (((var_taub_t * var_q1q_dn5) * var_qbi) + (assign6980_e7085 * var_qbi_dn5)), (((var_taub_t * var_q1q_dn6) * var_qbi) + (assign6980_e7085 * var_qbi_dn6)), (((var_taub_t * var_q1q_dn7) * var_qbi) + (assign6980_e7085 * var_qbi_dn7)), (((var_taub_t * var_q1q_dn8) * var_qbi) + (assign6980_e7085 * var_qbi_dn8)), (((var_taub_t * var_q1q_dn9) * var_qbi) + (assign6980_e7085 * var_qbi_dn9)), (((var_taub_t * var_q1q_dn10) * var_qbi) + (assign6980_e7085 * var_qbi_dn10)), (((var_taub_t * var_q1q_dn11) * var_qbi) + (assign6980_e7085 * var_qbi_dn11)), (((var_taub_t * var_q1q_db0) * var_qbi) + (assign6980_e7085 * var_qbi_db0)), (((var_taub_t * var_q1q_db1) * var_qbi) + (assign6980_e7085 * var_qbi_db1)),)
    } else {
        (var_taub_n, var_taub_n_dn0, var_taub_n_dn1, var_taub_n_dn2, var_taub_n_dn3, var_taub_n_dn4, var_taub_n_dn5, var_taub_n_dn6, var_taub_n_dn7, var_taub_n_dn8, var_taub_n_dn9, var_taub_n_dn10, var_taub_n_dn11, var_taub_n_db0, var_taub_n_db1,)
    }
};
        var_taub_n = assign6980_e7089;
        var_taub_n_dn0 = assign6980_e7089_d_n0;
        var_taub_n_dn1 = assign6980_e7089_d_n1;
        var_taub_n_dn2 = assign6980_e7089_d_n2;
        var_taub_n_dn3 = assign6980_e7089_d_n3;
        var_taub_n_dn4 = assign6980_e7089_d_n4;
        var_taub_n_dn5 = assign6980_e7089_d_n5;
        var_taub_n_dn6 = assign6980_e7089_d_n6;
        var_taub_n_dn7 = assign6980_e7089_d_n7;
        var_taub_n_dn8 = assign6980_e7089_d_n8;
        var_taub_n_dn9 = assign6980_e7089_d_n9;
        var_taub_n_dn10 = assign6980_e7089_d_n10;
        var_taub_n_dn11 = assign6980_e7089_d_n11;
        var_taub_n_db0 = assign6980_e7089_d_b0;
        var_taub_n_db1 = assign6980_e7089_d_b1;

        let assign6990_e7092: f64 = if p.p131 == 1.0 { 1.0 } else { 0.0 };
        var_guard127 = assign6990_e7092;

        let (assign7000_e7098, assign7000_e7098_d_n0, assign7000_e7098_d_n1, assign7000_e7098_d_n2, assign7000_e7098_d_n3, assign7000_e7098_d_n4, assign7000_e7098_d_n5, assign7000_e7098_d_n6, assign7000_e7098_d_n7, assign7000_e7098_d_n8, assign7000_e7098_d_n9, assign7000_e7098_d_n10, assign7000_e7098_d_n11, assign7000_e7098_d_b0, assign7000_e7098_d_b1,) = {
    if (var_guard127 != 0.0) {
        let assign7000_e7096: f64 = (p.p94 * var_taub_n);
        (assign7000_e7096, (p.p94 * var_taub_n_dn0), (p.p94 * var_taub_n_dn1), (p.p94 * var_taub_n_dn2), (p.p94 * var_taub_n_dn3), (p.p94 * var_taub_n_dn4), (p.p94 * var_taub_n_dn5), (p.p94 * var_taub_n_dn6), (p.p94 * var_taub_n_dn7), (p.p94 * var_taub_n_dn8), (p.p94 * var_taub_n_dn9), (p.p94 * var_taub_n_dn10), (p.p94 * var_taub_n_dn11), (p.p94 * var_taub_n_db0), (p.p94 * var_taub_n_db1),)
    } else {
        (var_taun, var_taun_dn0, var_taun_dn1, var_taun_dn2, var_taun_dn3, var_taun_dn4, var_taun_dn5, var_taun_dn6, var_taun_dn7, var_taun_dn8, var_taun_dn9, var_taun_dn10, var_taun_dn11, var_taun_db0, var_taun_db1,)
    }
};
        var_taun = assign7000_e7098;
        var_taun_dn0 = assign7000_e7098_d_n0;
        var_taun_dn1 = assign7000_e7098_d_n1;
        var_taun_dn2 = assign7000_e7098_d_n2;
        var_taun_dn3 = assign7000_e7098_d_n3;
        var_taun_dn4 = assign7000_e7098_d_n4;
        var_taun_dn5 = assign7000_e7098_d_n5;
        var_taun_dn6 = assign7000_e7098_d_n6;
        var_taun_dn7 = assign7000_e7098_d_n7;
        var_taun_dn8 = assign7000_e7098_d_n8;
        var_taun_dn9 = assign7000_e7098_d_n9;
        var_taun_dn10 = assign7000_e7098_d_n10;
        var_taun_dn11 = assign7000_e7098_d_n11;
        var_taun_db0 = assign7000_e7098_d_b0;
        var_taun_db1 = assign7000_e7098_d_b1;

        let assign7010_e7101: f64 = if p.p131 == 2.0 { 1.0 } else { 0.0 };
        var_guard128 = assign7010_e7101;

        let (assign7020_e7110, assign7020_e7110_d_n0, assign7020_e7110_d_n1, assign7020_e7110_d_n2, assign7020_e7110_d_n3, assign7020_e7110_d_n4, assign7020_e7110_d_n5, assign7020_e7110_d_n6, assign7020_e7110_d_n7, assign7020_e7110_d_n8, assign7020_e7110_d_n9, assign7020_e7110_d_n10, assign7020_e7110_d_n11, assign7020_e7110_d_b0, assign7020_e7110_d_b1,) = {
    if ((var_guard127 == 0.0) && (var_guard128 != 0.0)) {
        let assign7020_e7108: f64 = (p.p132 * var_taub_n);
        (assign7020_e7108, (p.p132 * var_taub_n_dn0), (p.p132 * var_taub_n_dn1), (p.p132 * var_taub_n_dn2), (p.p132 * var_taub_n_dn3), (p.p132 * var_taub_n_dn4), (p.p132 * var_taub_n_dn5), (p.p132 * var_taub_n_dn6), (p.p132 * var_taub_n_dn7), (p.p132 * var_taub_n_dn8), (p.p132 * var_taub_n_dn9), (p.p132 * var_taub_n_dn10), (p.p132 * var_taub_n_dn11), (p.p132 * var_taub_n_db0), (p.p132 * var_taub_n_db1),)
    } else {
        (var_taun, var_taun_dn0, var_taun_dn1, var_taun_dn2, var_taun_dn3, var_taun_dn4, var_taun_dn5, var_taun_dn6, var_taun_dn7, var_taun_dn8, var_taun_dn9, var_taun_dn10, var_taun_dn11, var_taun_db0, var_taun_db1,)
    }
};
        var_taun = assign7020_e7110;
        var_taun_dn0 = assign7020_e7110_d_n0;
        var_taun_dn1 = assign7020_e7110_d_n1;
        var_taun_dn2 = assign7020_e7110_d_n2;
        var_taun_dn3 = assign7020_e7110_d_n3;
        var_taun_dn4 = assign7020_e7110_d_n4;
        var_taun_dn5 = assign7020_e7110_d_n5;
        var_taun_dn6 = assign7020_e7110_d_n6;
        var_taun_dn7 = assign7020_e7110_d_n7;
        var_taun_dn8 = assign7020_e7110_d_n8;
        var_taun_dn9 = assign7020_e7110_d_n9;
        var_taun_dn10 = assign7020_e7110_d_n10;
        var_taun_dn11 = assign7020_e7110_d_n11;
        var_taun_db0 = assign7020_e7110_d_b0;
        var_taun_db1 = assign7020_e7110_d_b1;

        let (assign7030_e7118, assign7030_e7118_d_n0, assign7030_e7118_d_n1, assign7030_e7118_d_n2, assign7030_e7118_d_n3, assign7030_e7118_d_n4, assign7030_e7118_d_n5, assign7030_e7118_d_n6, assign7030_e7118_d_n7, assign7030_e7118_d_n8, assign7030_e7118_d_n9, assign7030_e7118_d_n10, assign7030_e7118_d_n11, assign7030_e7118_d_b0, assign7030_e7118_d_b1,) = {
    if ((var_guard127 == 0.0) && (var_guard128 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_taun, var_taun_dn0, var_taun_dn1, var_taun_dn2, var_taun_dn3, var_taun_dn4, var_taun_dn5, var_taun_dn6, var_taun_dn7, var_taun_dn8, var_taun_dn9, var_taun_dn10, var_taun_dn11, var_taun_db0, var_taun_db1,)
    }
};
        var_taun = assign7030_e7118;
        var_taun_dn0 = assign7030_e7118_d_n0;
        var_taun_dn1 = assign7030_e7118_d_n1;
        var_taun_dn2 = assign7030_e7118_d_n2;
        var_taun_dn3 = assign7030_e7118_d_n3;
        var_taun_dn4 = assign7030_e7118_d_n4;
        var_taun_dn5 = assign7030_e7118_d_n5;
        var_taun_dn6 = assign7030_e7118_d_n6;
        var_taun_dn7 = assign7030_e7118_d_n7;
        var_taun_dn8 = assign7030_e7118_d_n8;
        var_taun_dn9 = assign7030_e7118_d_n9;
        var_taun_dn10 = assign7030_e7118_d_n10;
        var_taun_dn11 = assign7030_e7118_d_n11;
        var_taun_db0 = assign7030_e7118_d_b0;
        var_taun_db1 = assign7030_e7118_d_b1;


        *var_dn0vb2e1_slot = var_dn0vb2e1;
        *var_dn0vb2e1_db0_slot = var_dn0vb2e1_db0;
        *var_dn0vb2e1_db1_slot = var_dn0vb2e1_db1;
        *var_dn0vb2e1_dn0_slot = var_dn0vb2e1_dn0;
        *var_dn0vb2e1_dn1_slot = var_dn0vb2e1_dn1;
        *var_dn0vb2e1_dn10_slot = var_dn0vb2e1_dn10;
        *var_dn0vb2e1_dn11_slot = var_dn0vb2e1_dn11;
        *var_dn0vb2e1_dn2_slot = var_dn0vb2e1_dn2;
        *var_dn0vb2e1_dn3_slot = var_dn0vb2e1_dn3;
        *var_dn0vb2e1_dn4_slot = var_dn0vb2e1_dn4;
        *var_dn0vb2e1_dn5_slot = var_dn0vb2e1_dn5;
        *var_dn0vb2e1_dn6_slot = var_dn0vb2e1_dn6;
        *var_dn0vb2e1_dn7_slot = var_dn0vb2e1_dn7;
        *var_dn0vb2e1_dn8_slot = var_dn0vb2e1_dn8;
        *var_dn0vb2e1_dn9_slot = var_dn0vb2e1_dn9;
        *var_dqbevb2e1_slot = var_dqbevb2e1;
        *var_dqbevb2e1_db0_slot = var_dqbevb2e1_db0;
        *var_dqbevb2e1_db1_slot = var_dqbevb2e1_db1;
        *var_dqbevb2e1_dn0_slot = var_dqbevb2e1_dn0;
        *var_dqbevb2e1_dn1_slot = var_dqbevb2e1_dn1;
        *var_dqbevb2e1_dn10_slot = var_dqbevb2e1_dn10;
        *var_dqbevb2e1_dn11_slot = var_dqbevb2e1_dn11;
        *var_dqbevb2e1_dn2_slot = var_dqbevb2e1_dn2;
        *var_dqbevb2e1_dn3_slot = var_dqbevb2e1_dn3;
        *var_dqbevb2e1_dn4_slot = var_dqbevb2e1_dn4;
        *var_dqbevb2e1_dn5_slot = var_dqbevb2e1_dn5;
        *var_dqbevb2e1_dn6_slot = var_dqbevb2e1_dn6;
        *var_dqbevb2e1_dn7_slot = var_dqbevb2e1_dn7;
        *var_dqbevb2e1_dn8_slot = var_dqbevb2e1_dn8;
        *var_dqbevb2e1_dn9_slot = var_dqbevb2e1_dn9;
        *var_dqevb2e1_slot = var_dqevb2e1;
        *var_dqevb2e1_db0_slot = var_dqevb2e1_db0;
        *var_dqevb2e1_db1_slot = var_dqevb2e1_db1;
        *var_dqevb2e1_dn0_slot = var_dqevb2e1_dn0;
        *var_dqevb2e1_dn1_slot = var_dqevb2e1_dn1;
        *var_dqevb2e1_dn10_slot = var_dqevb2e1_dn10;
        *var_dqevb2e1_dn11_slot = var_dqevb2e1_dn11;
        *var_dqevb2e1_dn2_slot = var_dqevb2e1_dn2;
        *var_dqevb2e1_dn3_slot = var_dqevb2e1_dn3;
        *var_dqevb2e1_dn4_slot = var_dqevb2e1_dn4;
        *var_dqevb2e1_dn5_slot = var_dqevb2e1_dn5;
        *var_dqevb2e1_dn6_slot = var_dqevb2e1_dn6;
        *var_dqevb2e1_dn7_slot = var_dqevb2e1_dn7;
        *var_dqevb2e1_dn8_slot = var_dqevb2e1_dn8;
        *var_dqevb2e1_dn9_slot = var_dqevb2e1_dn9;
        *var_dqtevb2e1_slot = var_dqtevb2e1;
        *var_dqtevb2e1_db0_slot = var_dqtevb2e1_db0;
        *var_dqtevb2e1_db1_slot = var_dqtevb2e1_db1;
        *var_dqtevb2e1_dn0_slot = var_dqtevb2e1_dn0;
        *var_dqtevb2e1_dn1_slot = var_dqtevb2e1_dn1;
        *var_dqtevb2e1_dn10_slot = var_dqtevb2e1_dn10;
        *var_dqtevb2e1_dn11_slot = var_dqtevb2e1_dn11;
        *var_dqtevb2e1_dn2_slot = var_dqtevb2e1_dn2;
        *var_dqtevb2e1_dn3_slot = var_dqtevb2e1_dn3;
        *var_dqtevb2e1_dn4_slot = var_dqtevb2e1_dn4;
        *var_dqtevb2e1_dn5_slot = var_dqtevb2e1_dn5;
        *var_dqtevb2e1_dn6_slot = var_dqtevb2e1_dn6;
        *var_dqtevb2e1_dn7_slot = var_dqtevb2e1_dn7;
        *var_dqtevb2e1_dn8_slot = var_dqtevb2e1_dn8;
        *var_dqtevb2e1_dn9_slot = var_dqtevb2e1_dn9;
        *var_guard126_slot = var_guard126;
        *var_guard127_slot = var_guard127;
        *var_guard128_slot = var_guard128;
        *var_in_n_slot = var_in_n;
        *var_in_n_db0_slot = var_in_n_db0;
        *var_in_n_db1_slot = var_in_n_db1;
        *var_in_n_dn0_slot = var_in_n_dn0;
        *var_in_n_dn1_slot = var_in_n_dn1;
        *var_in_n_dn10_slot = var_in_n_dn10;
        *var_in_n_dn11_slot = var_in_n_dn11;
        *var_in_n_dn2_slot = var_in_n_dn2;
        *var_in_n_dn3_slot = var_in_n_dn3;
        *var_in_n_dn4_slot = var_in_n_dn4;
        *var_in_n_dn5_slot = var_in_n_dn5;
        *var_in_n_dn6_slot = var_in_n_dn6;
        *var_in_n_dn7_slot = var_in_n_dn7;
        *var_in_n_dn8_slot = var_in_n_dn8;
        *var_in_n_dn9_slot = var_in_n_dn9;
        *var_qb1b2_slot = var_qb1b2;
        *var_qb1b2_db0_slot = var_qb1b2_db0;
        *var_qb1b2_db1_slot = var_qb1b2_db1;
        *var_qb1b2_dn0_slot = var_qb1b2_dn0;
        *var_qb1b2_dn1_slot = var_qb1b2_dn1;
        *var_qb1b2_dn10_slot = var_qb1b2_dn10;
        *var_qb1b2_dn11_slot = var_qb1b2_dn11;
        *var_qb1b2_dn2_slot = var_qb1b2_dn2;
        *var_qb1b2_dn3_slot = var_qb1b2_dn3;
        *var_qb1b2_dn4_slot = var_qb1b2_dn4;
        *var_qb1b2_dn5_slot = var_qb1b2_dn5;
        *var_qb1b2_dn6_slot = var_qb1b2_dn6;
        *var_qb1b2_dn7_slot = var_qb1b2_dn7;
        *var_qb1b2_dn8_slot = var_qb1b2_dn8;
        *var_qb1b2_dn9_slot = var_qb1b2_dn9;
        *var_qbc_slot = var_qbc;
        *var_qbc_db0_slot = var_qbc_db0;
        *var_qbc_db1_slot = var_qbc_db1;
        *var_qbc_dn0_slot = var_qbc_dn0;
        *var_qbc_dn1_slot = var_qbc_dn1;
        *var_qbc_dn10_slot = var_qbc_dn10;
        *var_qbc_dn11_slot = var_qbc_dn11;
        *var_qbc_dn2_slot = var_qbc_dn2;
        *var_qbc_dn3_slot = var_qbc_dn3;
        *var_qbc_dn4_slot = var_qbc_dn4;
        *var_qbc_dn5_slot = var_qbc_dn5;
        *var_qbc_dn6_slot = var_qbc_dn6;
        *var_qbc_dn7_slot = var_qbc_dn7;
        *var_qbc_dn8_slot = var_qbc_dn8;
        *var_qbc_dn9_slot = var_qbc_dn9;
        *var_qbe_slot = var_qbe;
        *var_qbe_db0_slot = var_qbe_db0;
        *var_qbe_db1_slot = var_qbe_db1;
        *var_qbe_dn0_slot = var_qbe_dn0;
        *var_qbe_dn1_slot = var_qbe_dn1;
        *var_qbe_dn10_slot = var_qbe_dn10;
        *var_qbe_dn11_slot = var_qbe_dn11;
        *var_qbe_dn2_slot = var_qbe_dn2;
        *var_qbe_dn3_slot = var_qbe_dn3;
        *var_qbe_dn4_slot = var_qbe_dn4;
        *var_qbe_dn5_slot = var_qbe_dn5;
        *var_qbe_dn6_slot = var_qbe_dn6;
        *var_qbe_dn7_slot = var_qbe_dn7;
        *var_qbe_dn8_slot = var_qbe_dn8;
        *var_qbe_dn9_slot = var_qbe_dn9;
        *var_qbe_qs_eff_slot = var_qbe_qs_eff;
        *var_qbe_qs_eff_db0_slot = var_qbe_qs_eff_db0;
        *var_qbe_qs_eff_db1_slot = var_qbe_qs_eff_db1;
        *var_qbe_qs_eff_dn0_slot = var_qbe_qs_eff_dn0;
        *var_qbe_qs_eff_dn1_slot = var_qbe_qs_eff_dn1;
        *var_qbe_qs_eff_dn10_slot = var_qbe_qs_eff_dn10;
        *var_qbe_qs_eff_dn11_slot = var_qbe_qs_eff_dn11;
        *var_qbe_qs_eff_dn2_slot = var_qbe_qs_eff_dn2;
        *var_qbe_qs_eff_dn3_slot = var_qbe_qs_eff_dn3;
        *var_qbe_qs_eff_dn4_slot = var_qbe_qs_eff_dn4;
        *var_qbe_qs_eff_dn5_slot = var_qbe_qs_eff_dn5;
        *var_qbe_qs_eff_dn6_slot = var_qbe_qs_eff_dn6;
        *var_qbe_qs_eff_dn7_slot = var_qbe_qs_eff_dn7;
        *var_qbe_qs_eff_dn8_slot = var_qbe_qs_eff_dn8;
        *var_qbe_qs_eff_dn9_slot = var_qbe_qs_eff_dn9;
        *var_qe_slot = var_qe;
        *var_qe_db0_slot = var_qe_db0;
        *var_qe_db1_slot = var_qe_db1;
        *var_qe_dn0_slot = var_qe_dn0;
        *var_qe_dn1_slot = var_qe_dn1;
        *var_qe_dn10_slot = var_qe_dn10;
        *var_qe_dn11_slot = var_qe_dn11;
        *var_qe_dn2_slot = var_qe_dn2;
        *var_qe_dn3_slot = var_qe_dn3;
        *var_qe_dn4_slot = var_qe_dn4;
        *var_qe_dn5_slot = var_qe_dn5;
        *var_qe_dn6_slot = var_qe_dn6;
        *var_qe_dn7_slot = var_qe_dn7;
        *var_qe_dn8_slot = var_qe_dn8;
        *var_qe_dn9_slot = var_qe_dn9;
        *var_taub_n_slot = var_taub_n;
        *var_taub_n_db0_slot = var_taub_n_db0;
        *var_taub_n_db1_slot = var_taub_n_db1;
        *var_taub_n_dn0_slot = var_taub_n_dn0;
        *var_taub_n_dn1_slot = var_taub_n_dn1;
        *var_taub_n_dn10_slot = var_taub_n_dn10;
        *var_taub_n_dn11_slot = var_taub_n_dn11;
        *var_taub_n_dn2_slot = var_taub_n_dn2;
        *var_taub_n_dn3_slot = var_taub_n_dn3;
        *var_taub_n_dn4_slot = var_taub_n_dn4;
        *var_taub_n_dn5_slot = var_taub_n_dn5;
        *var_taub_n_dn6_slot = var_taub_n_dn6;
        *var_taub_n_dn7_slot = var_taub_n_dn7;
        *var_taub_n_dn8_slot = var_taub_n_dn8;
        *var_taub_n_dn9_slot = var_taub_n_dn9;
        *var_taun_slot = var_taun;
        *var_taun_db0_slot = var_taun_db0;
        *var_taun_db1_slot = var_taun_db1;
        *var_taun_dn0_slot = var_taun_dn0;
        *var_taun_dn1_slot = var_taun_dn1;
        *var_taun_dn10_slot = var_taun_dn10;
        *var_taun_dn11_slot = var_taun_dn11;
        *var_taun_dn2_slot = var_taun_dn2;
        *var_taun_dn3_slot = var_taun_dn3;
        *var_taun_dn4_slot = var_taun_dn4;
        *var_taun_dn5_slot = var_taun_dn5;
        *var_taun_dn6_slot = var_taun_dn6;
        *var_taun_dn7_slot = var_taun_dn7;
        *var_taun_dn8_slot = var_taun_dn8;
        *var_taun_dn9_slot = var_taun_dn9;
    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.b[476] = (p.p3 == 1.0);
        s.v[476] = if s.b[476] { 1.0 } else { 0.0 };

        if s.b[476] {
            s.store_scalar(0, 70300000.0);
            s.store_scalar(1, 123000000.0);
        }

        if (!s.b[476]) {
            s.store_scalar(0, 158000000.0);
            s.store_scalar(1, 204000000.0);
        }

        s.v[157] = (1.0 - p.p33);

        s.v[3] = (p.p4 + 273.15);

        s.v[5] = (ctx_temp + p.p0);

        s.b[477] = (p.p150 == 0.0);
        s.v[477] = if s.b[477] { 1.0 } else { 0.0 };

        if s.b[477] {
            s.store_scalar(339, 1e-12);
        }

        if (!s.b[477]) {
            s.store_scalar(339, p.p150);
        }

        s.store_scale(340, 339, p.p1);

        s.v[52] = 0.001;

        s.v[336] = 0.001;

        s.v[62] = ((2.0) as f64).powf((2.0 - p.p67));

        s.v[279] = (((p.p114 + (((p.p115 * s.v[3]) * s.v[3]) / (s.v[3] + p.p116))) - 0.05) / 0.1);

        s.b[479] = ((p.p114 + (((p.p115 * s.v[3]) * s.v[3]) / (s.v[3] + p.p116))) < 0.05);
        s.v[479] = if s.b[479] { 1.0 } else { 0.0 };

        if s.b[479] {
            s.store_scalar(74, (0.05 + (0.1 * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[479]) {
            s.store_scalar(74, ((p.p114 + (((p.p115 * s.v[3]) * s.v[3]) / (s.v[3] + p.p116))) + (0.1 * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[71] = p.p114;

        s.v[72] = (1.0 / s.v[71]);

        s.v[75] = p.p71;

        s.v[76] = p.p72;

        s.v[79] = ((2.0) as f64).powf((2.0 - s.v[76]));

        s.v[279] = (((p.p117 + (((p.p118 * s.v[3]) * s.v[3]) / (s.v[3] + p.p119))) - 0.05) / 0.1);

        s.b[480] = ((p.p117 + (((p.p118 * s.v[3]) * s.v[3]) / (s.v[3] + p.p119))) < 0.05);
        s.v[480] = if s.b[480] { 1.0 } else { 0.0 };

        if s.b[480] {
            s.store_scalar(88, (0.05 + (0.1 * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[480]) {
            s.store_scalar(88, ((p.p117 + (((p.p118 * s.v[3]) * s.v[3]) / (s.v[3] + p.p119))) + (0.1 * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[87] = p.p117;

        s.v[86] = (1.0 / s.v[87]);

        s.v[175] = 1.0;

        s.v[207] = 0.0;

        s.v[242] = 0.0;

        s.v[222] = 0.0;

        s.v[42] = 0.0;

        s.v[11] = 0.0;

        s.v[2] = (s.v[5] + s.v[11]);

        s.v[4] = (s.v[2] / s.v[3]);

        s.v[6] = (8.617086918058125e-5 * s.v[2]);

        s.v[7] = (8.617086918058125e-5 * s.v[3]);

        s.v[8] = (1.0 / s.v[6]);

        s.v[9] = (1.0 / s.v[7]);

        s.v[10] = (s.v[8] - s.v[9]);

        s.v[12] = (s.v[2] - s.v[3]);

        s.v[274] = ((s.v[4]) as f64).ln();

        s.store_scaled_offset(279, 74, (((-(((p.p115 * s.v[2]) * s.v[2]) / (s.v[2] + p.p116)))) + ((-0.05))), 10.0);

        s.b[481] = ((s.v[74] - (((p.p115 * s.v[2]) * s.v[2]) / (s.v[2] + p.p116))) < 0.05);
        s.v[481] = if s.b[481] { 1.0 } else { 0.0 };

        if s.b[481] {
            s.store_offset_scaled_ad(70, A::ln_one_plus_exp(s.ad_value(279)), 0.1, 0.05);
        }

        if (!s.b[481]) {
            s.store_add_scaled_inputs_ad(70, A::offset(s.ad_value(74), (-(((p.p115 * s.v[2]) * s.v[2]) / (s.v[2] + p.p116)))), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), 0.1);
        }

        s.store_scaled_offset(279, 88, (((-(((p.p118 * s.v[2]) * s.v[2]) / (s.v[2] + p.p119)))) + ((-0.05))), 10.0);

        s.b[482] = ((s.v[88] - (((p.p118 * s.v[2]) * s.v[2]) / (s.v[2] + p.p119))) < 0.05);
        s.v[482] = if s.b[482] { 1.0 } else { 0.0 };

        if s.b[482] {
            s.store_offset_scaled_ad(85, A::ln_one_plus_exp(s.ad_value(279)), 0.1, 0.05);
        }

        if (!s.b[482]) {
            s.store_add_scaled_inputs_ad(85, A::offset(s.ad_value(88), (-(((p.p118 * s.v[2]) * s.v[2]) / (s.v[2] + p.p119)))), 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), 0.1);
        }

        s.v[13] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p66 * s.v[4])) + ((1.0 - s.v[4]) * p.p105));

        s.v[279] = ((0.05 - s.v[13]) / s.v[6]);

        s.b[483] = (0.05 < s.v[13]);
        s.v[483] = if s.b[483] { 1.0 } else { 0.0 };

        if s.b[483] {
            s.store_scalar(14, (s.v[13] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[483]) {
            s.store_scalar(14, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[15] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p64 * s.v[4])) + ((1.0 - s.v[4]) * p.p110));

        s.v[279] = ((0.05 - s.v[15]) / s.v[6]);

        s.b[484] = (0.05 < s.v[15]);
        s.v[484] = if s.b[484] { 1.0 } else { 0.0 };

        if s.b[484] {
            s.store_scalar(16, (s.v[15] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[484]) {
            s.store_scalar(16, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[21] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p80 * s.v[4])) + ((1.0 - s.v[4]) * p.p110));

        s.v[279] = ((0.05 - s.v[21]) / s.v[6]);

        s.b[485] = (0.05 < s.v[21]);
        s.v[485] = if s.b[485] { 1.0 } else { 0.0 };

        if s.b[485] {
            s.store_scalar(22, (s.v[21] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[485]) {
            s.store_scalar(22, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[18] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p71 * s.v[4])) + ((1.0 - s.v[4]) * p.p110));

        s.v[279] = ((0.05 - s.v[18]) / s.v[6]);

        s.b[486] = (0.05 < s.v[18]);
        s.v[486] = if s.b[486] { 1.0 } else { 0.0 };

        if s.b[486] {
            s.store_scalar(17, (s.v[18] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[486]) {
            s.store_scalar(17, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[20] = (((((-3.0) * s.v[6]) * s.v[274]) + (s.v[75] * s.v[4])) + ((1.0 - s.v[4]) * p.p110));

        s.v[279] = ((0.05 - s.v[20]) / s.v[6]);

        s.b[487] = (0.05 < s.v[20]);
        s.v[487] = if s.b[487] { 1.0 } else { 0.0 };

        if s.b[487] {
            s.store_scalar(19, (s.v[20] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[487]) {
            s.store_scalar(19, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[56] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p27 * s.v[4])) + ((1.0 - s.v[4]) * p.p109));

        s.v[279] = ((0.05 - s.v[56]) / s.v[6]);

        s.b[488] = (0.05 < s.v[56]);
        s.v[488] = if s.b[488] { 1.0 } else { 0.0 };

        if s.b[488] {
            s.store_scalar(55, (s.v[56] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[488]) {
            s.store_scalar(55, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.v[101] = (((((-3.0) * s.v[6]) * s.v[274]) + (p.p138 * s.v[4])) + ((1.0 - s.v[4]) * p.p140));

        s.v[279] = ((0.05 - s.v[101]) / s.v[6]);

        s.b[489] = (0.05 < s.v[101]);
        s.v[489] = if s.b[489] { 1.0 } else { 0.0 };

        if s.b[489] {
            s.store_scalar(102, (s.v[101] + (s.v[6] * (((1.0 + ((s.v[279]) as f64).exp())) as f64).ln())));
        }

        if (!s.b[489]) {
            s.store_scalar(102, (0.05 + (s.v[6] * (((1.0 + (((-s.v[279])) as f64).exp())) as f64).ln())));
        }

        s.store_div_from_scalar(65, 1.0, 14);

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_scaled_input(73, 65, p.p66, p.p67);

        s.store_powf_scaled_input(90, 67, s.v[75], s.v[76]);

        s.store_scale(23, 73, p.p65);

        s.store_scaled_powf_ad(103, A::div_from_scalar(p.p138, s.ad_value(102)), p.p139, p.p137);

        s.store_offset_scaled_ad(26, A::powf(A::div_from_scalar(p.p71, s.ad_value(17)), p.p72), (1.0 - p.p75), p.p75);

        s.store_div_from_scalar(27, 1.0, 26);

        s.store_scale(24, 26, p.p70);

        s.store_scale(25, 27, p.p75);

        s.v[28] = (p.p54 * (((s.v[274] * p.p97)) as f64).exp());

        s.b[490] = (s.v[28] < s.v[340]);
        s.v[490] = if s.b[490] { 1.0 } else { 0.0 };

        if s.b[490] {
            s.copy_ad(28, 340);
        }

        s.v[29] = (p.p56 * (((s.v[274] * (p.p98 - p.p96))) as f64).exp());

        s.v[30] = (p.p55 * (((s.v[274] * p.p101)) as f64).exp());

        s.b[491] = (s.v[30] < s.v[340]);
        s.v[491] = if s.b[491] { 1.0 } else { 0.0 };

        if s.b[491] {
            s.copy_ad(30, 340);
        }

        s.v[32] = (p.p57 * (((s.v[274] * p.p102)) as f64).exp());

        s.v[31] = (p.p60 * (((s.v[274] * p.p99)) as f64).exp());

        s.b[492] = (p.p122 != 0.0);
        s.v[492] = if s.b[492] { 1.0 } else { 0.0 };

        if s.b[492] {
            s.store_scalar(50, (p.p10 * (1.0 + (s.v[12] * p.p122))));
            s.store_scaled_offset(279, 50, (-1.0), 1.0 / (s.v[52]));
        }

        s.b[493] = (s.v[50] < 1.0);
        s.v[493] = if s.b[493] { 1.0 } else { 0.0 };

        if (s.b[492] && s.b[493]) {
            s.store_offset_scaled_ad(50, A::ln_one_plus_exp(s.ad_value(279)), s.v[52], 1.0);
        }

        if (s.b[492] && (!s.b[493])) {
            s.store_add_scaled_inputs_ad_rhs(50, 50, 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), s.v[52]);
        }

        if s.b[492] {
            s.store_offset(48, 50, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!s.b[492]) {
            s.store_scalar(48, p.p10);
        }

        s.b[494] = (p.p123 != 0.0);
        s.v[494] = if s.b[494] { 1.0 } else { 0.0 };

        if s.b[494] {
            s.store_scalar(51, (p.p11 * (1.0 + (s.v[12] * p.p123))));
            s.store_scaled_offset(279, 51, (-1.0), 1.0 / (s.v[52]));
        }

        s.b[495] = (s.v[51] < 1.0);
        s.v[495] = if s.b[495] { 1.0 } else { 0.0 };

        if (s.b[494] && s.b[495]) {
            s.store_offset_scaled_ad(51, A::ln_one_plus_exp(s.ad_value(279)), s.v[52], 1.0);
        }

        if (s.b[494] && (!s.b[495])) {
            s.store_add_scaled_inputs_ad_rhs(51, 51, 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), s.v[52]);
        }

        if s.b[494] {
            s.store_offset(49, 51, (-(s.v[52] * 0.6931471805599453)));
        }

        if (!s.b[494]) {
            s.store_scalar(49, p.p11);
        }

        s.v[335] = (p.p43 * (1.0 + (p.p124 * s.v[12])));

        s.v[281] = (s.v[336] * s.v[336]);

        s.v[282] = (s.v[335] * s.v[335]);

        s.b[496] = (s.v[335] < 0.0);
        s.v[496] = if s.b[496] { 1.0 } else { 0.0 };

        if s.b[496] {
            s.store_scalar(334, ((0.5 * s.v[281]) / ((((s.v[282] + s.v[281])) as f64).sqrt() - s.v[335])));
        }

        if (!s.b[496]) {
            s.store_scalar(334, (0.5 * ((((s.v[282] + s.v[281])) as f64).sqrt() + s.v[335])));
        }

        s.store_scaled_mul_ad(35, A::exp(A::div_from_scalar((s.v[274] * (((4.0 - p.p98) - p.p96) + p.p121)), s.ad_value(48))), A::exp(A::div_from_scalar(((-p.p105) * s.v[10]), s.ad_value(48))), p.p9);

        s.v[36] = (p.p12 * (((s.v[274] * (1.0 - p.p98))) as f64).exp());

        s.v[37] = (p.p30 * (((s.v[274] * (1.0 - p.p103))) as f64).exp());

        s.v[42] = ((p.p16 * ((((s.v[274] * ((4.0 - p.p97) + p.p121)) / p.p17)) as f64).exp()) * (((((-p.p111) * s.v[10]) / p.p17)) as f64).exp());

        s.v[43] = ((p.p29 * (((s.v[274] * ((4.0 - p.p103) + p.p121))) as f64).exp()) * ((((-p.p112) * s.v[10])) as f64).exp());

        s.store_powf_scaled_input(275, 70, s.v[72], (-0.5));

        s.store_div_from_scalar(276, 1.0, 73);

        s.store_mul_ad_affine_product_lhs(61, A::mul3_scaled_output(s.ad_value(70), s.ad_value(70), s.ad_value(275), p.p35), s.ad_value(276), (p.p66 * (s.v[72] * s.v[72])), 0.0, 65);

        s.store_div_from_scalar(67, 1.0, 19);

        s.store_powf_scaled_input(277, 85, s.v[86], (-0.5));

        s.store_div_from_scalar(278, 1.0, 90);

        s.store_mul_ad_affine_product_lhs(83, A::mul3_scaled_output(s.ad_value(85), s.ad_value(85), s.ad_value(277), p.p37), s.ad_value(278), (s.v[75] * (s.v[86] * s.v[86])), 0.0, 67);

        s.v[275] = (((s.v[274] * p.p96)) as f64).exp();

        s.store_scale(40, 27, (p.p14 * s.v[275]));

        s.store_scale(41, 276, (p.p13 * s.v[275]));

        s.v[104] = ((p.p133 * (((s.v[274] * (4.0 - p.p141))) as f64).exp()) * ((((-p.p140) * s.v[10])) as f64).exp());

        s.v[106] = (p.p135 * (((s.v[274] * (1.0 - p.p141))) as f64).exp());

        s.v[93] = ((p.p86 * (((s.v[274] * (p.p98 - 2.0))) as f64).exp()) * ((((-p.p120) * s.v[10])) as f64).exp());

        s.v[94] = (p.p87 * (((s.v[274] * ((p.p96 + p.p98) - 1.0))) as f64).exp());

        s.v[95] = (p.p88 * (((s.v[274] * (p.p99 - 1.0))) as f64).exp());

        s.v[96] = ((p.p89 * (s.v[94] + s.v[95])) / (p.p87 + p.p88));

        s.v[97] = (p.p90 * (((s.v[274] * (p.p100 - 1.0))) as f64).exp());

        s.v[100] = (s.v[2] - 300.0);

        s.b[498] = (s.v[2] < 525.0);
        s.v[498] = if s.b[498] { 1.0 } else { 0.0 };

        if s.b[498] {
            s.store_scale(98, 1, ((1.0 + (0.00072 * s.v[100])) - ((1.6e-6 * s.v[100]) * s.v[100])));
        }

        if (!s.b[498]) {
            s.store_scale(98, 1, 1.081);
        }

        s.v[99] = (p.p92 * (((s.v[274] * p.p96)) as f64).exp());

        s.store_scaled_voltage(244, ctx, nodes, Some(6), Some(7), p.p3);

        s.store_scaled_voltage(245, ctx, nodes, Some(6), Some(8), p.p3);

        s.store_scaled_voltage(246, ctx, nodes, Some(6), Some(4), p.p3);

        s.store_scaled_voltage(247, ctx, nodes, Some(5), Some(4), p.p3);

        s.store_scaled_voltage(248, ctx, nodes, Some(5), Some(6), p.p3);

        s.store_scaled_voltage(253, ctx, nodes, Some(3), Some(7), p.p3);

        s.store_scaled_voltage(250, ctx, nodes, Some(7), Some(8), p.p3);

        s.store_scaled_voltage(260, ctx, nodes, Some(1), Some(5), p.p3);

        s.store_scaled_voltage(263, ctx, nodes, Some(1), Some(2), p.p3);

        s.store_scaled_voltage(264, ctx, nodes, Some(1), Some(0), p.p3);

        s.store_scaled_voltage(252, ctx, nodes, Some(10), Some(7), p.p3);

        s.store_scaled_voltage(251, ctx, nodes, Some(9), Some(10), p.p3);

        s.store_add_scaled_inputs4_indices(249, 248, 1.0, 245, 1.0, 250, -1.0, 252, -1.0);

        s.store_add_scaled_inputs4_indices(262, 260, 1.0, 264, (-1.0), 249, 1.0, 251, -1.0);

        s.store_add(261, 264, 262);

        s.store_sub(255, 253, 252);

        s.store_sub(254, 255, 251);

        s.b[505] = ((s.v[245] * s.v[8]) < p.p147);
        s.v[505] = if s.b[505] { 1.0 } else { 0.0 };

        if s.b[505] {
            s.store_exp_scaled_input(265, 245, s.v[8]);
        }

        if (!s.b[505]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_scale_offset_rhs(265, 295, 245, s.v[8], (((-p.p147)) + (1.0)));
        }

        s.b[506] = (((s.v[246] * s.v[8]) / s.v[48]) < p.p147);
        s.v[506] = if s.b[506] { 1.0 } else { 0.0 };

        if s.b[506] {
            s.store_ad_value(266, A::exp_div_scaled_inputs(s.ad_value(246), s.v[8], s.ad_value(48), 1.0));
        }

        if (!s.b[506]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(266, 295, A::div_scaled_inputs(s.ad_value(246), s.v[8], s.ad_value(48), 1.0), (((-p.p147)) + (1.0)));
        }

        s.b[507] = ((s.v[249] * s.v[8]) < p.p147);
        s.v[507] = if s.b[507] { 1.0 } else { 0.0 };

        if s.b[507] {
            s.store_exp_scaled_input(268, 249, s.v[8]);
        }

        if (!s.b[507]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_scale_offset_rhs(268, 295, 249, s.v[8], (((-p.p147)) + (1.0)));
        }

        s.b[508] = ((s.v[248] * s.v[8]) < p.p147);
        s.v[508] = if s.b[508] { 1.0 } else { 0.0 };

        if (!s.b[508]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

    }

    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[509] = ((s.v[261] * s.v[8]) < p.p147);
        s.v[509] = if s.b[509] { 1.0 } else { 0.0 };

        if s.b[509] {
            s.store_exp_scaled_input(269, 261, s.v[8]);
        }

        if (!s.b[509]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_scale_offset_rhs(269, 295, 261, s.v[8], (((-p.p147)) + (1.0)));
        }

        s.b[510] = ((s.v[253] * s.v[8]) < p.p147);
        s.v[510] = if s.b[510] { 1.0 } else { 0.0 };

        if (!s.b[510]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        s.b[511] = ((s.v[254] * s.v[8]) < p.p147);
        s.v[511] = if s.b[511] { 1.0 } else { 0.0 };

        if s.b[511] {
            s.store_exp_scaled_input(257, 254, s.v[8]);
        }

        if (!s.b[511]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_scale_offset_rhs(257, 295, 254, s.v[8], (((-p.p147)) + (1.0)));
        }

        s.b[512] = ((s.v[255] * s.v[8]) < p.p147);
        s.v[512] = if s.b[512] { 1.0 } else { 0.0 };

        if (!s.b[512]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        s.b[513] = (((s.v[261] - s.v[16]) * s.v[8]) < p.p147);
        s.v[513] = if s.b[513] { 1.0 } else { 0.0 };

        if s.b[513] {
            s.store_exp_scaled_input_ad(272, A::sub(s.ad_value(261), s.ad_value(16)), s.v[8]);
        }

        if (!s.b[513]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(272, 295, A::sub_scaled_inputs(s.ad_value(261), s.v[8], s.ad_value(16), s.v[8]), (((-p.p147)) + (1.0)));
        }

        s.b[514] = (((s.v[249] - s.v[16]) * s.v[8]) < p.p147);
        s.v[514] = if s.b[514] { 1.0 } else { 0.0 };

        if s.b[514] {
            s.store_exp_scaled_input_ad(270, A::sub(s.ad_value(249), s.ad_value(16)), s.v[8]);
        }

        if (!s.b[514]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(270, 295, A::sub_scaled_inputs(s.ad_value(249), s.v[8], s.ad_value(16), s.v[8]), (((-p.p147)) + (1.0)));
        }

        s.b[515] = (((s.v[245] - s.v[16]) * s.v[8]) < p.p147);
        s.v[515] = if s.b[515] { 1.0 } else { 0.0 };

        if s.b[515] {
            s.store_exp_scaled_input_ad(271, A::sub(s.ad_value(245), s.ad_value(16)), s.v[8]);
        }

        if (!s.b[515]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(271, 295, A::sub_scaled_inputs(s.ad_value(245), s.v[8], s.ad_value(16), s.v[8]), (((-p.p147)) + (1.0)));
        }

        s.b[516] = (((s.v[244] - s.v[16]) * s.v[8]) < p.p147);
        s.v[516] = if s.b[516] { 1.0 } else { 0.0 };

        if s.b[516] {
            s.store_exp_scaled_input_ad(273, A::sub(s.ad_value(244), s.ad_value(16)), s.v[8]);
        }

        if (!s.b[516]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(273, 295, A::sub_scaled_inputs(s.ad_value(244), s.v[8], s.ad_value(16), s.v[8]), (((-p.p147)) + (1.0)));
        }

        s.store_sqrt_offset_scaled_input(111, 271, 4.0, 1.0);

        s.store_sqrt_offset_scaled_input(112, 273, 4.0, 1.0);

        s.store_div_scaled_value_offset_denominator(113, s.ad_value(273), 2.0, s.ad_value(112), 1.0, 1.0);

        s.b[517] = (s.v[113] < p.p149);
        s.v[517] = if s.b[517] { 1.0 } else { 0.0 };

        if s.b[517] {
            s.store_scalar(113, p.p149);
        }

        s.store_add_scaled_inputs3_mixed_iia(114, 111, s.v[6], 112, ((-1.0) * s.v[6]), A::ln(A::div_scaled_offset_numerator(s.ad_value(111), 1.0, 1.0, A::offset(s.ad_value(112), 1.0), 1.0)), (-s.v[6]));

        s.store_scaled_add(115, 114, 250, 1.0 / (s.v[31]));

        s.b[518] = (s.v[115] > 0.0);
        s.v[518] = if s.b[518] { 1.0 } else { 0.0 };

        s.b[519] = (s.v[244] < 100.0);
        s.v[519] = if s.b[519] { 1.0 } else { 0.0 };

        if (s.b[518] && s.b[519]) {
            s.copy_ad(297, 244);
        }

        if (s.b[518] && (!s.b[519])) {
            s.store_offset_ln_ad(297, A::offset(s.ad_value(244), (((-100.0)) + (1.0))), 100.0);
        }

        if s.b[518] {
            s.store_add_scaled_inputs3_mixed_iai(116, 16, 1.0, A::ln(A::scale_offset(s.ad_value(115), (0.5 * (s.v[31] * s.v[8])), 1.0)), (2.0 * s.v[6]), 297, -1.0);
            s.store_scale(292, 16, 0.2);
            s.store_square(281, 292);
            s.store_square(282, 116);
        }

        s.b[520] = (s.v[116] < 0.0);
        s.v[520] = if s.b[520] { 1.0 } else { 0.0 };

        if (s.b[518] && s.b[520]) {
            s.store_div_scaled_inputs_mixed_ia(117, 281, 0.5, A::sub(A::sqrt(A::add(s.ad_value(282), s.ad_value(281))), s.ad_value(116)), 1.0);
        }

        if (s.b[518] && (!s.b[520])) {
            s.store_scaled_add_ad_lhs(117, A::sqrt(A::add(s.ad_value(282), s.ad_value(281))), 116, 0.5);
        }

        if s.b[518] {
            s.store_div_scaled_product_offset_rhs(118, s.ad_value(117), s.ad_value(117), (p.p62 * p.p61), 1.0, A::scaled_offset(s.ad_value(117), (p.p62 * s.v[31]), p.p61), 1.0);
            s.store_div(285, 115, 118);
            s.store_scaled_offset(279, 285, (-1.0), 1.0 / (p.p63));
        }

        s.b[521] = (s.v[285] < 1.0);
        s.v[521] = if s.b[521] { 1.0 } else { 0.0 };

        if (s.b[518] && s.b[521]) {
            s.store_offset_scaled_ad(283, A::ln_one_plus_exp(s.ad_value(279)), p.p63, 1.0);
        }

        if (s.b[518] && (!s.b[521])) {
            s.store_add_scaled_inputs_ad_rhs(283, 285, 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), p.p63);
        }

        if s.b[518] {
            s.store_scale(119, 283, 1.0 / ((1.0 + (p.p63 * (((1.0 + ((((-1.0) / p.p63)) as f64).exp())) as f64).ln()))));
            s.store_scale(120, 117, 1.0 / ((p.p62 * p.p61)));
            s.store_div_scaled_offset_numerator(121, A::sqrt(A::offset(A::mul3_scaled_output(s.ad_value(119), s.ad_value(120), A::offset(s.ad_value(120), 1.0), 4.0), 1.0)), 1.0, 1.0, A::mul_scaled_lhs(s.ad_value(119), 2.0, A::offset(s.ad_value(120), 1.0)), 1.0);
            s.store_div_ad(122, A::add_scaled_sub_value_product(1.0, s.ad_value(121), 1.0, s.ad_value(113), s.ad_value(121), 1.0), A::offset(A::mul(s.ad_value(113), s.ad_value(121)), 1.0));
            s.store_scaled_mul(124, 115, 122, ((0.5 * s.v[31]) * s.v[8]));
            s.store_add_scaled_offset_product_rhs_mixed_iia(286, 124, 2.0, 113, A::add(s.ad_value(113), s.ad_value(124)), 1.0, 1.0);
            s.store_scaled_offset(125, 124, (-1.0), 0.5);
            s.store_add_ad_lhs(280, A::square(s.ad_value(125)), 286);
        }

        s.b[522] = (s.v[124] >= 1.0);
        s.v[522] = if s.b[522] { 1.0 } else { 0.0 };

        if (s.b[518] && s.b[522]) {
            s.store_add_ad_rhs(126, 125, A::sqrt(s.ad_value(280)));
        }

        if (s.b[518] && (!s.b[522])) {
            s.store_div_add_scaled_inputs_rhs_mixed_ai(126, 286, A::sqrt(s.ad_value(280)), 1.0, 125, -1.0);
        }

        s.b[523] = (s.v[126] < p.p148);
        s.v[523] = if s.b[523] { 1.0 } else { 0.0 };

        if (s.b[518] && s.b[523]) {
            s.store_scalar(126, p.p148);
        }

        if s.b[518] {
            s.store_mul_ad_product_rhs(128, 126, A::offset(s.ad_value(126), 1.0), A::exp_scaled_input(s.ad_value(16), s.v[8]));
            s.store_scaled_offset(130, 115, (-p.p62), (0.5 * p.p61));
            s.store_scale(131, 115, ((p.p61 * s.v[31]) * p.p62));
            s.store_add_ad_rhs(132, 130, A::sqrt(A::add(A::square(s.ad_value(130)), s.ad_value(131))));
        }

        s.b[524] = (p.p73 == 0.0);
        s.v[524] = if s.b[524] { 1.0 } else { 0.0 };

        if (s.b[518] && s.b[524]) {
            s.store_scale(133, 17, 0.1);
        }

        if (s.b[518] && (!s.b[524])) {
            s.store_mul_offset_ad_rhs(133, 17, A::div_scaled_inputs(s.ad_value(115), 2.0, A::add(s.ad_value(115), s.ad_value(118)), 1.0), 0.1);
        }

        if s.b[518] {
            s.store_div_scaled_value_offset_denominator(134, s.ad_value(115), p.p62, s.ad_value(115), p.p62, 1.0);
            s.store_div_from_scalar_offset_input(210, p.p62, 115, p.p62);
        }

        if (!s.b[518]) {
            s.store_scalar(118, 0.0);
            s.store_div_scaled_value_offset_denominator(126, s.ad_value(271), 2.0, s.ad_value(111), 1.0, 1.0);
            s.copy_ad(128, 265);
        }

        s.b[525] = ((((s.v[250]) as f64).abs() < (1e-5 * s.v[6])) || (((s.v[114]) as f64).abs() < ((1e-40 * s.v[6]) * (s.v[111] + s.v[112]))));
        s.v[525] = if s.b[525] { 1.0 } else { 0.0 };

        if ((!s.b[518]) && s.b[525]) {
            s.store_scaled_add(135, 126, 113, 0.5);
            s.store_div_scaled_value_offset_denominator(122, s.ad_value(135), 1.0, s.ad_value(135), 1.0, 1.0);
        }

        if ((!s.b[518]) && (!s.b[525])) {
            s.store_div_ad_rhs(122, 114, A::add_scaled_inputs3(s.ad_value(114), 1.0, s.ad_value(245), 1.0, s.ad_value(244), -1.0));
        }

        if (!s.b[518]) {
            s.copy_ad(132, 250);
            s.store_scale(133, 17, 0.1);
            s.copy_ad(134, 115);
            s.store_sub_from_scalar_scaled_input(210, 1.0, 134, 1.0 / (p.p62));
        }

        s.store_scale(136, 14, (1.0 - ((3.0) as f64).powf(((-1.0) / p.p67))));

        s.store_scale(293, 14, 0.1);

        s.store_div_scaled_inputs2_indices(279, 246, 1.0, 136, (-1.0), 293, 1.0);

        s.b[526] = (s.v[246] < s.v[136]);
        s.v[526] = if s.b[526] { 1.0 } else { 0.0 };

        if s.b[526] {
            s.store_add_scaled_product_right_ln_one_plus_exp_scaled_input(137, 246, 1.0, 293, 279, 1.0, (-1.0));
        }

        if (!s.b[526]) {
            s.store_add_scaled_product_right_ln_one_plus_exp_scaled_input(137, 136, 1.0, 293, 279, -1.0, (-1.0));
        }

        s.store_powf_ad(59, A::sub_from_scalar(1.0, A::mul(s.ad_value(137), s.ad_value(65))), (1.0 - p.p67));

        s.store_add_scaled_inputs3_mixed_aii(138, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(14), 1.0, s.ad_value(59), 1.0 / ((1.0 - p.p67))), 1.0, 246, 3.0, 137, (-3.0));

        s.b[527] = (p.p74 == 1.0);
        s.v[527] = if s.b[527] { 1.0 } else { 0.0 };

        if s.b[527] {
            s.copy_ad(139, 244);
        }

        s.b[528] = (p.p74 == 2.0);
        s.v[528] = if s.b[528] { 1.0 } else { 0.0 };

        if ((!s.b[527]) && s.b[528]) {
            s.store_add(139, 244, 132);
        }

        if ((!s.b[527]) && (!s.b[528])) {
            s.copy_ad(139, 245);
        }

        s.store_div_ad(140, A::sub_from_scalar(2.0, s.ad_value(25)), A::sub_from_scalar(1.0, s.ad_value(25)));

        s.store_mul_sub_from_scalar_ad_rhs(141, 17, 1.0, A::powf(s.ad_value(140), ((-1.0) / p.p72)));

        s.store_div_scaled_inputs2_indices(279, 139, 1.0, 141, (-1.0), 133, 1.0);

        s.b[529] = (s.v[139] < s.v[141]);
        s.v[529] = if s.b[529] { 1.0 } else { 0.0 };

        if s.b[529] {
            s.store_add_scaled_product_right_ln_one_plus_exp_scaled_input(142, 139, 1.0, 133, 279, 1.0, (-1.0));
        }

        if (!s.b[529]) {
            s.store_add_scaled_product_right_ln_one_plus_exp_scaled_input(142, 141, 1.0, 133, 279, -1.0, (-1.0));
        }

        s.store_powf(143, 210, p.p76);

        s.store_add_ad(144, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::mul(s.ad_value(143), A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(142), s.ad_value(17))), (1.0 - p.p72))), 1.0 / ((1.0 - p.p72))), A::mul3(s.ad_value(143), s.ad_value(140), A::sub(s.ad_value(139), s.ad_value(142))));

        s.store_add_scaled_product_value_ad(145, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(144)), 1.0, 25, 244, 1.0);

        s.store_scale(146, 35, (4.0 * 1.0 / (s.v[36])));

        s.store_mul(147, 146, 266);

        s.store_div_scaled_value_offset_denominator(149, s.ad_value(147), 1.0, A::sqrt(A::offset(s.ad_value(147), 1.0)), 1.0, 1.0);

        s.store_pow_ad(129, s.ad_value(128), A::div_from_scalar(1.0, s.ad_value(49)));

        s.store_mul(148, 146, 129);

        s.store_div_scaled_value_offset_denominator(150, s.ad_value(148), 1.0, A::sqrt(A::offset(s.ad_value(148), 1.0)), 1.0, 1.0);

        s.b[530] = (p.p92 == 0.0);
        s.v[530] = if s.b[530] { 1.0 } else { 0.0 };

        if s.b[530] {
            s.store_add_ad(151, A::offset(A::div(s.ad_value(138), s.ad_value(41)), 1.0), A::div(s.ad_value(145), s.ad_value(40)));
        }

        if (!s.b[530]) {
            s.store_offset_scaled_div(289, 138, 41, (s.v[99] * s.v[8]), (s.v[99] * s.v[8]));
            s.store_div_scaled_inputs_indices(290, 145, (-(s.v[99] * s.v[8])), 40, 1.0);
            s.store_scaled_sub_ad(151, A::exp(s.ad_value(289)), A::exp(s.ad_value(290)), 1.0 / (((((s.v[99] * s.v[8])) as f64).exp() - 1.0)));
        }

        s.v[281] = (0.1 * 0.1);

        s.store_square(282, 151);

        s.b[531] = (s.v[151] < 0.0);
        s.v[531] = if s.b[531] { 1.0 } else { 0.0 };

        if s.b[531] {
            s.store_div_from_scalar_sub_ad(152, (0.5 * s.v[281]), A::sqrt(A::offset(s.ad_value(282), s.v[281])), s.ad_value(151));
        }

        if (!s.b[531]) {
            s.store_scaled_add_ad_lhs(152, A::sqrt(A::offset(s.ad_value(282), s.v[281])), 151, 0.5);
        }

        s.store_mul_offset_ad_rhs(153, 152, A::add_scaled_inputs(s.ad_value(149), 0.5, s.ad_value(150), 0.5), 1.0);

        s.store_scaled_mul(154, 35, 129, p.p15);

        s.store_mul(155, 35, 266);

        s.store_div_scaled_inputs2_indices(156, 155, 1.0, 154, (-1.0), 153, 1.0);

        s.store_scale(279, 246, 10000.0);

        s.b[532] = (s.v[246] < 0.0);
        s.v[532] = if s.b[532] { 1.0 } else { 0.0 };

        if s.b[532] {
            s.store_scaled_ln_one_plus_exp(296, 279, 0.0001);
        }

        if (!s.b[532]) {
            s.store_add_scaled_inputs_ad_rhs(296, 246, 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), 0.0001);
        }

        s.store_scale(298, 296, 1.0 / (p.p152));

        s.b[533] = (s.v[298] < p.p147);
        s.v[533] = if s.b[533] { 1.0 } else { 0.0 };

        if (!s.b[533]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        s.store_scaled_offset(279, 246, (-p.p154), 1000.0);

        s.b[535] = (((s.v[246] * s.v[8]) / p.p17) < p.p147);
        s.v[535] = if s.b[535] { 1.0 } else { 0.0 };

        if s.b[535] {
            s.store_exp_scaled_input(296, 246, (s.v[8] * 1.0 / (p.p17)));
        }

        if (!s.b[535]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_scale_offset_rhs(296, 295, 246, (s.v[8] * 1.0 / (p.p17)), (((-p.p147)) + (1.0)));
        }

        s.b[536] = (p.p24 == 1.0);
        s.v[536] = if s.b[536] { 1.0 } else { 0.0 };

        s.b[537] = (((s.v[246] - s.v[55]) * s.v[8]) < p.p147);
        s.v[537] = if s.b[537] { 1.0 } else { 0.0 };

        if (s.b[536] && s.b[537]) {
            s.store_exp_scaled_input_ad(298, A::sub(s.ad_value(246), s.ad_value(55)), s.v[8]);
        }

        if (s.b[536] && (!s.b[537])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(298, 295, A::sub_scaled_inputs(s.ad_value(246), s.v[8], s.ad_value(55), s.v[8]), (((-p.p147)) + (1.0)));
        }

        s.b[538] = (((s.v[156] / s.v[35]) - 1000.0) < 40.0);
        s.v[538] = if s.b[538] { 1.0 } else { 0.0 };

        if (s.b[536] && (!s.b[538])) {
            s.store_scalar(295, ((40.0) as f64).exp());
        }

        s.b[540] = (((s.v[247] * s.v[8]) / p.p19) < p.p147);
        s.v[540] = if s.b[540] { 1.0 } else { 0.0 };

        if s.b[540] {
            s.store_exp_scaled_input(296, 247, (s.v[8] * 1.0 / (p.p19)));
        }

        if (!s.b[540]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_scale_offset_rhs(296, 295, 247, (s.v[8] * 1.0 / (p.p19)), (((-p.p147)) + (1.0)));
        }

        s.b[541] = (p.p24 == 1.0);
        s.v[541] = if s.b[541] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[542] = (((s.v[247] - s.v[55]) * s.v[8]) < p.p147);
        s.v[542] = if s.b[542] { 1.0 } else { 0.0 };

        if (s.b[541] && s.b[542]) {
            s.store_exp_scaled_input_ad(298, A::sub(s.ad_value(247), s.ad_value(55)), s.v[8]);
        }

        if (s.b[541] && (!s.b[542])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(298, 295, A::sub_scaled_inputs(s.ad_value(247), s.v[8], s.ad_value(55), s.v[8]), (((-p.p147)) + (1.0)));
        }

        s.b[543] = (((s.v[246] * s.v[8]) / p.p21) < p.p147);
        s.v[543] = if s.b[543] { 1.0 } else { 0.0 };

        if s.b[543] {
            s.store_exp_scaled_input(296, 246, (s.v[8] * 1.0 / (p.p21)));
        }

        if (!s.b[543]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_scale_offset_rhs(296, 295, 246, (s.v[8] * 1.0 / (p.p21)), (((-p.p147)) + (1.0)));
        }

        s.b[544] = (((s.v[247] * s.v[8]) / p.p23) < p.p147);
        s.v[544] = if s.b[544] { 1.0 } else { 0.0 };

        if s.b[544] {
            s.store_exp_scaled_input(296, 247, (s.v[8] * 1.0 / (p.p23)));
        }

        if (!s.b[544]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_scale_offset_rhs(296, 295, 247, (s.v[8] * 1.0 / (p.p23)), (((-p.p147)) + (1.0)));
        }

        s.b[545] = (((s.v[249] * s.v[8]) / p.p32) < p.p147);
        s.v[545] = if s.b[545] { 1.0 } else { 0.0 };

        if s.b[545] {
            s.store_exp_scaled_input(296, 249, (s.v[8] * 1.0 / (p.p32)));
        }

        if (!s.b[545]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_scale_offset_rhs(296, 295, 249, (s.v[8] * 1.0 / (p.p32)), (((-p.p147)) + (1.0)));
        }

        s.b[546] = (((s.v[247] * s.v[8]) / p.p146) < p.p147);
        s.v[546] = if s.b[546] { 1.0 } else { 0.0 };

        if s.b[546] {
            s.store_exp_scaled_input(296, 247, (s.v[8] * 1.0 / (p.p146)));
        }

        if (!s.b[546]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_scale_offset_rhs(296, 295, 247, (s.v[8] * 1.0 / (p.p146)), (((-p.p147)) + (1.0)));
        }

        s.b[547] = (((p.p34 > 0.0) && (p.p35 > 0.0)) && (s.v[246] < 0.0));
        s.v[547] = if s.b[547] { 1.0 } else { 0.0 };

        s.b[548] = ((s.v[61] * (1.0 - (s.v[62] / (2.0 * s.v[59])))) < p.p147);
        s.v[548] = if s.b[548] { 1.0 } else { 0.0 };

        if (s.b[547] && (!s.b[548])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if s.b[547] {
            s.store_mul(275, 246, 65);
            s.store_scaled_mul_ad(60, A::powf(A::sqrt_square_offset(s.ad_value(275), 1e-30), ((-2.0) - p.p67)), A::sub(A::scale_offset(A::scale(s.ad_value(275), (3.0 * (p.p67 - 1.0))), (-p.p67), (((1.0 - (p.p67 * p.p67))) * (p.p67))), A::mul3_scaled_output(s.ad_value(275), s.ad_value(275), A::offset(s.ad_value(275), (p.p67 - 1.0)), 6.0)), 0.16666666666666666);
            s.store_div_scaled_product_by_product(275, s.ad_value(246), s.ad_value(61), s.v[62], s.ad_value(70), s.ad_value(60), 1.0);
        }

        s.b[549] = (s.v[275] < (-0.001));
        s.v[549] = if s.b[549] { 1.0 } else { 0.0 };

        s.b[550] = (s.v[275] < p.p147);
        s.v[550] = if s.b[550] { 1.0 } else { 0.0 };

        if ((s.b[547] && s.b[549]) && (!s.b[550])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        s.b[551] = (((p.p36 > 0.0) && (p.p37 > 0.0)) && (s.v[244] < 0.0));
        s.v[551] = if s.b[551] { 1.0 } else { 0.0 };

        if s.b[551] {
            s.store_powf_ad(77, A::sub_from_scalar(1.0, A::mul(s.ad_value(244), s.ad_value(67))), (1.0 - s.v[76]));
        }

        s.b[552] = ((s.v[83] * (1.0 - (s.v[79] / (2.0 * s.v[77])))) < p.p147);
        s.v[552] = if s.b[552] { 1.0 } else { 0.0 };

        if (s.b[551] && (!s.b[552])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        if s.b[551] {
            s.store_mul(277, 244, 67);
            s.store_scaled_mul_ad(80, A::powf(A::sqrt_square_offset(s.ad_value(277), 1e-30), ((-2.0) - s.v[76])), A::sub(A::scale_offset(A::scale(s.ad_value(277), (3.0 * (s.v[76] - 1.0))), (-s.v[76]), (((1.0 - (s.v[76] * s.v[76]))) * (s.v[76]))), A::mul3_scaled_output(s.ad_value(277), s.ad_value(277), A::offset(s.ad_value(277), (s.v[76] - 1.0)), 6.0)), 0.16666666666666666);
            s.store_div_scaled_product_by_product(277, s.ad_value(244), s.ad_value(83), s.v[79], s.ad_value(85), s.ad_value(80), 1.0);
        }

        s.b[553] = (s.v[277] < (-0.001));
        s.v[553] = if s.b[553] { 1.0 } else { 0.0 };

        s.b[554] = (s.v[277] < p.p147);
        s.v[554] = if s.b[554] { 1.0 } else { 0.0 };

        if ((s.b[551] && s.b[553]) && (!s.b[554])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
        }

        s.store_mul(165, 146, 268);

        s.store_scale(166, 270, 4.0);

        s.store_div_scaled_inputs2_mixed_iia(168, 165, 1.0, 146, (-1.0), A::offset(A::sqrt(A::offset(s.ad_value(165), 1.0)), 1.0), 1.0);

        s.store_div_scaled_value_offset_denominator(167, s.ad_value(166), 1.0, A::sqrt(A::offset(s.ad_value(166), 1.0)), 1.0, 1.0);

        s.b[556] = ((p.p5 > 0.0) && (p.p33 > 0.0));
        s.v[556] = if s.b[556] { 1.0 } else { 0.0 };

        if s.b[556] {
            s.store_div_scaled_offset_numerator(171, s.ad_value(269), ((p.p33 * 2.0) * s.v[43]), ((-1.0) * ((p.p33 * 2.0) * s.v[43])), A::offset(A::sqrt(A::scale_offset(s.ad_value(269), ((4.0 * s.v[43]) / s.v[37]), 1.0)), 1.0), 1.0);
        }

        s.b[557] = (p.p8 == 1.0);
        s.v[557] = if s.b[557] { 1.0 } else { 0.0 };

        if (s.b[556] && s.b[557]) {
            s.store_div_scaled_inputs2_mixed_iia(172, 269, ((((1.0 - p.p143) * p.p33) * 2.0) * s.v[104]), 257, (-((((1.0 - p.p143) * p.p33) * 2.0) * s.v[104])), A::offset(A::sqrt(A::offset(A::add_scaled_inputs(s.ad_value(269), ((4.0 * s.v[104]) / s.v[106]), s.ad_value(257), (p.p144 * ((4.0 * s.v[104]) / s.v[106]))), 1.0)), 1.0), 1.0);
        }

        if (s.b[556] && (!s.b[557])) {
            s.store_div_scaled_offset_numerator(172, s.ad_value(269), ((((1.0 - p.p143) * p.p33) * 2.0) * s.v[104]), ((-1.0) * ((((1.0 - p.p143) * p.p33) * 2.0) * s.v[104])), A::offset(A::sqrt(A::scale_offset(s.ad_value(269), ((4.0 * s.v[104]) / s.v[106]), 1.0)), 1.0), 1.0);
        }

        s.b[558] = (p.p5 == 1.0);
        s.v[558] = if s.b[558] { 1.0 } else { 0.0 };

        if (s.b[556] && s.b[558]) {
            s.store_scalar(291, ((p.p33 * (s.v[43] + s.v[104])) * s.v[32]));
            s.store_offset_scaled_ad(173, A::ln_scaled_input(s.ad_value(291), s.v[8]), (-s.v[6]), ((2.0) * (s.v[6])));
            s.store_sub(284, 261, 173);
            s.store_scalar(281, (0.11 * 0.11));
            s.store_square(282, 284);
        }

        s.b[559] = (s.v[284] < 0.0);
        s.v[559] = if s.b[559] { 1.0 } else { 0.0 };

        if ((s.b[556] && s.b[558]) && s.b[559]) {
            s.store_div_scaled_inputs_mixed_ia(174, 281, 0.5, A::sub(A::sqrt(A::add(s.ad_value(282), s.ad_value(281))), s.ad_value(284)), 1.0);
        }

        if ((s.b[556] && s.b[558]) && (!s.b[559])) {
            s.store_scaled_add_ad_lhs(174, A::sqrt(A::add(s.ad_value(282), s.ad_value(281))), 284, 0.5);
        }

        if (s.b[556] && s.b[558]) {
            s.store_div_ad_rhs(175, 174, A::add_scaled_inputs4(s.ad_value(291), 1.0, s.ad_value(171), s.v[32], s.ad_value(172), s.v[32], s.ad_value(174), 1.0));
        }

        if (s.b[556] && (!s.b[558])) {
            s.store_scalar(173, 0.0);
            s.store_scalar(284, 0.0);
            s.store_scalar(174, 0.0);
            s.store_scalar(175, 1.0);
        }

        s.b[560] = (p.p84 == 1.0);
        s.v[560] = if s.b[560] { 1.0 } else { 0.0 };

        if s.b[560] {
            s.store_add(347, 248, 244);
            s.store_scalar(281, (1e-6 * 1e-6));
            s.store_scaled_mul(282, 347, 347, ((-1.0) * (-1.0)));
        }

        s.store_add_ad(183, A::offset(A::div(s.ad_value(138), s.ad_value(41)), 1.0), A::div(s.ad_value(145), s.ad_value(40)));

        s.v[281] = (0.1 * 0.1);

        s.store_square(282, 183);

        s.b[563] = (s.v[183] < 0.0);
        s.v[563] = if s.b[563] { 1.0 } else { 0.0 };

        if s.b[563] {
            s.store_div_from_scalar_sub_ad(184, (0.5 * s.v[281]), A::sqrt(A::offset(s.ad_value(282), s.v[281])), s.ad_value(183));
        }

        if (!s.b[563]) {
            s.store_scaled_add_ad_lhs(184, A::sqrt(A::offset(s.ad_value(282), s.v[281])), 183, 0.5);
        }

        s.store_mul_offset_ad_rhs(185, 184, A::add_scaled_inputs(s.ad_value(149), 0.5, s.ad_value(150), 0.5), 1.0);

        s.store_div_from_scalar(187, s.v[29], 185);

        s.b[564] = (s.v[187] < s.v[340]);
        s.v[564] = if s.b[564] { 1.0 } else { 0.0 };

        if s.b[564] {
            s.copy_ad(187, 340);
        }

        s.store_scale(186, 187, 3.0);

        s.b[565] = (s.v[156] > 0.0);
        s.v[565] = if s.b[565] { 1.0 } else { 0.0 };

        s.b[566] = (p.p39 == 1.0);
        s.v[566] = if s.b[566] { 1.0 } else { 0.0 };

        s.b[567] = (s.v[244] < p.p44);
        s.v[567] = if s.b[567] { 1.0 } else { 0.0 };

        s.b[568] = (((-s.v[156]) / p.p42) < p.p147);
        s.v[568] = if s.b[568] { 1.0 } else { 0.0 };

        if (((s.b[565] && s.b[566]) && s.b[567]) && s.b[568]) {
            s.store_exp_scaled_input(332, 156, (-1.0 / (p.p42)));
        }

        if (((s.b[565] && s.b[566]) && s.b[567]) && (!s.b[568])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_scale_offset_rhs(332, 295, 156, (-1.0 / (p.p42)), (((-p.p147)) + (1.0)));
        }

        if ((s.b[565] && s.b[566]) && s.b[567]) {
            s.store_mul_sub_from_scalar_lhs(333, p.p44, 244, 332);
        }

        s.b[569] = (((-s.v[334]) * ((s.v[333]) as f64).powf(p.p41)) < p.p147);
        s.v[569] = if s.b[569] { 1.0 } else { 0.0 };

        if (((s.b[565] && s.b[566]) && s.b[567]) && s.b[569]) {
            s.store_exp_mul_scaled_lhs_mixed_ia(337, 334, -1.0, A::powf(s.ad_value(333), p.p41));
        }

        if (((s.b[565] && s.b[566]) && s.b[567]) && (!s.b[569])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(337, 295, A::mul_scaled_lhs(s.ad_value(334), -1.0, A::powf(s.ad_value(333), p.p41)), (((-p.p147)) + (1.0)));
        }

        if ((s.b[565] && s.b[566]) && s.b[567]) {
            s.store_mul_ad_product_lhs_mixed_ai(207, A::div_from_scalar(p.p40, s.ad_value(334)), 333, 337);
        }

        s.b[570] = (p.p39 == 2.0);
        s.v[570] = if s.b[570] { 1.0 } else { 0.0 };

        s.b[571] = (s.v[244] < s.v[16]);
        s.v[571] = if s.b[571] { 1.0 } else { 0.0 };

        if (((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) {
            s.store_scalar(196, ((2.0 * p.p46) / (p.p45 * p.p45)));
            s.store_div_scaled_inputs2_indices(280, 16, 1.0, 244, (-1.0), 210, 1.0);
            s.store_sqrt_div_scaled_inputs(197, 280, 2.0, 196, 1.0);
        }

        s.b[572] = (p.p7 == 0.0);
        s.v[572] = if s.b[572] { 1.0 } else { 0.0 };

        if ((((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) && s.b[572]) {
            s.store_scalar(198, p.p45);
        }

        if ((((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) && (!s.b[572])) {
            s.store_sub_from_scalar_scaled_input(123, 1.0, 122, 0.5);
            s.store_scaled_mul(198, 123, 123, p.p45);
        }

        if (((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) {
            s.store_div_scaled_product_sqrt_square_sum_denominator(199, 197, 198, 1.0, 197, 198, 1.0);
            s.store_div_scaled_inputs2_indices(200, 16, 1.0, 244, (-1.0), 199, 1.0);
            s.store_add_product3_rhs_indices(201, 200, 199, 196, 210, 0.5);
        }

        s.b[573] = (p.p7 == 0.0);
        s.v[573] = if s.b[573] { 1.0 } else { 0.0 };

        if ((((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) && s.b[573]) {
            s.copy_ad(202, 201);
        }

        if ((((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) && (!s.b[573])) {
            s.store_offset_scaled(203, 122, ((2.0) * ((2.0 * p.p47))), (((2.0 * p.p47)) + (1.0)));
            s.store_scalar(204, ((1.0 + p.p47) / (1.0 + (2.0 * p.p47))));
            s.store_sub_ad_rhs(205, 200, A::mul3_scaled_output(s.ad_value(199), s.ad_value(196), A::sub(s.ad_value(204), A::div_scaled_inputs(s.ad_value(156), 1.0, s.ad_value(203), p.p62)), 0.5));
            s.store_add_ad(280, A::square(A::sub(s.ad_value(205), s.ad_value(201))), A::mul3_scaled_output(s.ad_value(200), s.ad_value(200), s.ad_value(134), (0.1 * 1.0 / (p.p62))));
            s.store_add_scaled_inputs3_sqrt_third_indices(202, 205, 0.5, 201, 0.5, 280, 0.5);
        }

        if (((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) {
            s.store_div_scaled_inputs2_indices(287, 202, 1.0, 200, (-1.0), 202, 1.0);
        }

        s.b[574] = (((s.v[287]) as f64).abs() > 1e-7);
        s.v[574] = if s.b[574] { 1.0 } else { 0.0 };

        if ((((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) && s.b[574]) {
            s.store_div_scaled_inputs_indices(206, 199, 0.5, 287, 1.0);
            s.store_mul_product3_mixed_aaii(207, A::sub(A::exp_div_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(202), 1.0), A::exp(A::mul_offset_rhs(A::div_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(202), 1.0), A::div(s.ad_value(198), s.ad_value(206)), 1.0))), A::div(s.ad_value(0), s.ad_value(98)), 202, 206, 1.0);
        }

        if ((((s.b[565] && (!s.b[566])) && s.b[570]) && s.b[571]) && (!s.b[574])) {
            s.store_mul_ad_product_rhs_mixed_ia(207, 0, 198, A::exp_div_scaled_inputs(s.ad_value(98), -1.0, s.ad_value(202), 1.0));
        }

        s.b[575] = (p.p39 == 3.0);
        s.v[575] = if s.b[575] { 1.0 } else { 0.0 };

        s.b[576] = (s.v[244] < p.p44);
        s.v[576] = if s.b[576] { 1.0 } else { 0.0 };

        if ((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) {
            s.store_mul_powf(211, A::powf(A::sub_from_scalar(1.0, A::div_scaled_value_offset_denominator(s.ad_value(156), 1.0, s.ad_value(156), p.p48, 1.0)), p.p49), A::sub_from_scalar(p.p44, s.ad_value(244)), p.p41);
        }

        s.b[577] = (p.p7 == 0.0);
        s.v[577] = if s.b[577] { 1.0 } else { 0.0 };

        if (((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) && s.b[577]) {
            s.copy_ad(212, 211);
        }

        if (((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) && (!s.b[577])) {
            s.store_scaled_offset(213, 156, (-p.p52), 1.0 / (p.p48));
            s.store_scaled_offset(279, 213, (-1.0), 1.0 / (p.p51));
        }

        s.b[578] = (s.v[213] < 1.0);
        s.v[578] = if s.b[578] { 1.0 } else { 0.0 };

        if ((((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) && (!s.b[577])) && s.b[578]) {
            s.store_offset_scaled_ad(214, A::ln_one_plus_exp(s.ad_value(279)), p.p51, 1.0);
        }

        if ((((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) && (!s.b[577])) && (!s.b[578])) {
            s.store_add_scaled_inputs_ad_rhs(214, 213, 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), p.p51);
        }

        if (((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) && (!s.b[577])) {
            s.store_mul_powf_ad_rhs(212, 211, s.ad_value(214), p.p50);
        }

        s.b[579] = (((-s.v[334]) * s.v[212]) < p.p147);
        s.v[579] = if s.b[579] { 1.0 } else { 0.0 };

        if (((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) && s.b[579]) {
            s.store_exp_mul_scaled_lhs_indices(337, 334, -1.0, 212);
        }

        if (((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) && (!s.b[579])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(337, 295, A::mul_scaled_lhs(s.ad_value(334), -1.0, s.ad_value(212)), (((-p.p147)) + (1.0)));
        }

        if ((((s.b[565] && (!s.b[566])) && (!s.b[570])) && s.b[575]) && s.b[576]) {
            s.store_mul_ad_lhs(207, A::mul_sub_from_scalar_rhs(A::div_from_scalar(p.p40, s.ad_value(334)), p.p44, s.ad_value(244)), 337);
        }

        s.b[580] = (s.v[207] > 0.0);
        s.v[580] = if s.b[580] { 1.0 } else { 0.0 };

        s.b[581] = (p.p53 == 1.0);
        s.v[581] = if s.b[581] { 1.0 } else { 0.0 };

        if ((s.b[565] && s.b[580]) && s.b[581]) {
            s.store_add_scaled_inputs3(208, A::div_from_scalar(s.v[6], A::mul(s.ad_value(156), A::add(s.ad_value(30), s.ad_value(186)))), 1.0, A::div(s.ad_value(153), s.ad_value(35)), s.v[42], A::div(s.ad_value(28), A::add(s.ad_value(30), s.ad_value(186))), 1.0);
        }

        s.b[582] = (p.p39 == 3.0);
        s.v[582] = if s.b[582] { 1.0 } else { 0.0 };

        if (((s.b[565] && s.b[580]) && s.b[581]) && s.b[582]) {
            s.store_scaled_sub(279, 207, 208, 1000000.0);
        }

        s.b[583] = (s.v[207] < s.v[208]);
        s.v[583] = if s.b[583] { 1.0 } else { 0.0 };

        if ((((s.b[565] && s.b[580]) && s.b[581]) && s.b[582]) && s.b[583]) {
            s.store_sub_scaled_inputs_ad_rhs(207, 207, 1.0, A::ln_one_plus_exp(s.ad_value(279)), 1e-6);
        }

        if ((((s.b[565] && s.b[580]) && s.b[581]) && s.b[582]) && (!s.b[583])) {
            s.store_sub_scaled_inputs_ad_rhs(207, 208, 1.0, A::ln_one_plus_exp(A::neg(s.ad_value(279))), 1e-6);
        }

        s.store_scaled_mul(215, 23, 138, (1.0 - p.p68));

        s.store_div_scaled_inputs2_indices(279, 247, 1.0, 136, (-1.0), 293, 1.0);

        s.b[585] = (s.v[247] < s.v[136]);
        s.v[585] = if s.b[585] { 1.0 } else { 0.0 };

        if s.b[585] {
            s.store_add_scaled_product_right_ln_one_plus_exp_scaled_input(216, 247, 1.0, 293, 279, 1.0, (-1.0));
        }

        if (!s.b[585]) {
            s.store_add_scaled_product_right_ln_one_plus_exp_scaled_input(216, 136, 1.0, 293, 279, -1.0, (-1.0));
        }

        s.store_mul_add_scaled_inputs3_offset_rhs(217, 23, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(14), 1.0, A::powf(A::sub_from_scalar(1.0, A::mul(s.ad_value(216), s.ad_value(65))), (1.0 - p.p67)), 1.0 / ((1.0 - p.p67))), p.p68, s.ad_value(247), ((3.0) * (p.p68)), s.ad_value(216), (((-3.0)) * (p.p68)), 0.0);

        s.store_scaled_mul(218, 24, 145, p.p77);

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.v[219] = (s.v[94] * s.v[36]);

        s.store_scaled_mul(223, 149, 184, (0.5 * s.v[219]));

        s.store_scaled_mul(224, 150, 184, (0.5 * s.v[219]));

        s.store_scale(294, 17, 0.1);

        s.store_div_scaled_inputs2_indices(279, 249, 1.0, 141, (-1.0), 294, 1.0);

        s.b[586] = (s.v[249] < s.v[141]);
        s.v[586] = if s.b[586] { 1.0 } else { 0.0 };

        if s.b[586] {
            s.store_add_scaled_product_right_ln_one_plus_exp_scaled_input(225, 249, 1.0, 294, 279, 1.0, (-1.0));
        }

        if (!s.b[586]) {
            s.store_add_scaled_product_right_ln_one_plus_exp_scaled_input(225, 141, 1.0, 294, 279, -1.0, (-1.0));
        }

        s.store_add_scaled_product_mixed_aia(226, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(225), s.ad_value(17))), (1.0 - p.p72)), 1.0 / ((1.0 - p.p72))), 1.0, 140, A::sub(s.ad_value(249), s.ad_value(225)), 1.0);

        s.store_mul_add_scaled_product_rhs(227, 24, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(226)), ((1.0 - p.p77) * (1.0 - p.p33)), s.ad_value(25), s.ad_value(249), ((1.0 - p.p77) * (1.0 - p.p33)));

        s.store_div_scaled_inputs2_indices(279, 261, 1.0, 141, (-1.0), 294, 1.0);

        s.b[587] = (s.v[261] < s.v[141]);
        s.v[587] = if s.b[587] { 1.0 } else { 0.0 };

        if s.b[587] {
            s.store_add_scaled_product_right_ln_one_plus_exp_scaled_input(228, 261, 1.0, 294, 279, 1.0, (-1.0));
        }

        if (!s.b[587]) {
            s.store_add_scaled_product_right_ln_one_plus_exp_scaled_input(228, 141, 1.0, 294, 279, -1.0, (-1.0));
        }

        s.store_add_scaled_product_mixed_aia(229, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(17), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(228), s.ad_value(17))), (1.0 - p.p72)), 1.0 / ((1.0 - p.p72))), 1.0, 140, A::sub(s.ad_value(261), s.ad_value(228)), 1.0);

        s.store_mul_add_scaled_product_rhs(230, 24, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(25), s.ad_value(229)), ((1.0 - p.p77) * p.p33), s.ad_value(25), s.ad_value(261), ((1.0 - p.p77) * p.p33));

        s.store_scale(301, 102, 0.1);

        s.store_scale(231, 102, (1.0 - ((2.0) as f64).powf(((-1.0) / p.p139))));

        s.store_div_scaled_inputs2_indices(279, 253, 1.0, 231, (-1.0), 301, 1.0);

        s.b[588] = (s.v[253] < s.v[231]);
        s.v[588] = if s.b[588] { 1.0 } else { 0.0 };

        if s.b[588] {
            s.store_add_scaled_product_right_ln_one_plus_exp_scaled_input(232, 253, 1.0, 301, 279, 1.0, (-1.0));
        }

        if (!s.b[588]) {
            s.store_add_scaled_product_right_ln_one_plus_exp_scaled_input(232, 231, 1.0, 301, 279, -1.0, (-1.0));
        }

        s.store_mul_add_scaled_inputs3_offset_rhs(233, 103, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(102), 1.0, A::powf(A::sub_from_scalar(1.0, A::div(s.ad_value(232), s.ad_value(102))), (1.0 - p.p139)), 1.0 / ((1.0 - p.p139))), 1.0, s.ad_value(253), 2.0, s.ad_value(232), (-2.0), 0.0);

        s.store_scaled_powf_ad(234, A::scale(s.ad_value(35), 1.0 / (s.v[36])), (1.0 / p.p85), (s.v[93] * s.v[36]));

        s.b[589] = ((s.v[246] / (p.p85 * s.v[6])) < p.p147);
        s.v[589] = if s.b[589] { 1.0 } else { 0.0 };

        if s.b[589] {
            s.store_exp_scaled_input(296, 246, 1.0 / ((p.p85 * s.v[6])));
        }

        if (!s.b[589]) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_scale_offset_rhs(296, 295, 246, 1.0 / ((p.p85 * s.v[6])), (((-p.p147)) + (1.0)));
        }

        s.store_mul(236, 234, 296);

        s.v[237] = (((4.0 * s.v[95]) * s.v[6]) / s.v[31]);

        s.store_mul_scaled_offset_ad_rhs(238, 122, (0.5 * s.v[237]), A::add(s.ad_value(126), s.ad_value(113)), 2.0);

        s.b[590] = (p.p79 == 0.0);
        s.v[590] = if s.b[590] { 1.0 } else { 0.0 };

        if s.b[590] {
            s.store_add_scaled_inputs(243, 168, (s.v[219] * ((s.v[96] * 0.5) * 1.0 / ((s.v[94] + s.v[95])))), 167, (s.v[237] * ((s.v[96] * 0.5) * 1.0 / ((s.v[94] + s.v[95])))));
        }

        s.b[591] = ((((s.v[249] - s.v[22]) / p.p91) * s.v[8]) < p.p147);
        s.v[591] = if s.b[591] { 1.0 } else { 0.0 };

        if ((!s.b[590]) && s.b[591]) {
            s.store_exp_scaled_input_ad(177, A::sub(s.ad_value(249), s.ad_value(22)), (1.0 / (p.p91) * s.v[8]));
        }

        if ((!s.b[590]) && (!s.b[591])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(177, 295, A::sub_scaled_inputs(s.ad_value(249), (1.0 / (p.p91) * s.v[8]), s.ad_value(22), (1.0 / (p.p91) * s.v[8])), (((-p.p147)) + (1.0)));
        }

        if (!s.b[590]) {
            s.store_div_scaled_value_offset_denominator(243, s.ad_value(268), ((2.0 * s.v[43]) * s.v[97]), A::sqrt(A::scale_offset(s.ad_value(177), 4.0, 1.0)), 1.0, 1.0);
        }

        s.b[592] = (((p.p5 == 1.0) || (p.p5 == 3.0)) && (p.p33 > 0.0));
        s.v[592] = if s.b[592] { 1.0 } else { 0.0 };

        if s.b[592] {
            s.store_scale(243, 243, s.v[157]);
        }

        s.b[593] = (p.p79 == 0.0);
        s.v[593] = if s.b[593] { 1.0 } else { 0.0 };

        if (s.b[592] && s.b[593]) {
            s.store_mul(169, 146, 269);
            s.store_div_scaled_inputs2_mixed_iia(170, 169, 1.0, 146, (-1.0), A::offset(A::sqrt(A::offset(s.ad_value(169), 1.0)), 1.0), 1.0);
            s.store_scale(239, 272, 4.0);
            s.store_div_scaled_value_offset_denominator(240, s.ad_value(239), 1.0, A::sqrt(A::offset(s.ad_value(239), 1.0)), 1.0, 1.0);
            s.store_add_scaled_inputs(241, 170, (s.v[219] * (((0.5 * p.p33) * s.v[96]) * 1.0 / ((s.v[94] + s.v[95])))), 240, (s.v[237] * (((0.5 * p.p33) * s.v[96]) * 1.0 / ((s.v[94] + s.v[95])))));
        }

        s.b[594] = (((s.v[261] - s.v[22]) * s.v[8]) < p.p147);
        s.v[594] = if s.b[594] { 1.0 } else { 0.0 };

        if ((s.b[592] && (!s.b[593])) && s.b[594]) {
            s.store_exp_scaled_input_ad(178, A::sub(s.ad_value(261), s.ad_value(22)), s.v[8]);
        }

        if ((s.b[592] && (!s.b[593])) && (!s.b[594])) {
            s.store_scalar(295, ((p.p147) as f64).exp());
            s.store_mul_offset_ad_rhs(178, 295, A::sub_scaled_inputs(s.ad_value(261), s.v[8], s.ad_value(22), s.v[8]), (((-p.p147)) + (1.0)));
        }

        if (s.b[592] && (!s.b[593])) {
            s.store_div_scaled_value_offset_denominator(241, s.ad_value(269), (((2.0 * p.p33) * s.v[43]) * s.v[97]), A::sqrt(A::scale_offset(s.ad_value(178), 4.0, 1.0)), 1.0, 1.0);
        }

        if s.b[592] {
            s.store_mul(242, 175, 241);
        }

        s.b[595] = (p.p6 == 1.0);
        s.v[595] = if s.b[595] { 1.0 } else { 0.0 };

        if s.b[595] {
            s.store_offset_powf_ad(190, A::sub_from_scalar(1.0, A::mul(s.ad_value(137), s.ad_value(65))), (-p.p67), (-3.0));
            s.store_div_scaled_inputs2_indices(288, 246, 1.0, 136, (-1.0), 293, 1.0);
        }

        s.b[596] = (s.v[288] < 0.0);
        s.v[596] = if s.b[596] { 1.0 } else { 0.0 };

        if (s.b[595] && s.b[596]) {
            s.store_div_from_scalar_offset_ad(191, 1.0, A::exp(s.ad_value(288)), 1.0);
        }

        if (s.b[595] && (!s.b[596])) {
            let assign6660_ad_e6874: A = A::exp_scaled_input(s.ad_value(288), -1.0);
            s.store_div_ad(191, assign6660_ad_e6874, A::offset(assign6660_ad_e6874, 1.0));
        }

        if s.b[595] {
            s.store_offset_mul(189, 190, 191, 3.0);
            s.store_scaled_mul(192, 23, 189, (1.0 - p.p68));
            s.store_mul_div_scaled_product_mixed_aiii(195, A::div_from_scalar(0.5, A::sqrt(A::offset(s.ad_value(147), 1.0))), 146, 266, s.v[8], 48, 1.0);
            s.store_scaled_mul(193, 184, 195, (0.5 * s.v[219]));
            s.store_scale(194, 236, 1.0 / ((p.p85 * s.v[6])));
            s.store_mul_add_scaled_inputs3_offset_rhs(222, 248, s.ad_value(192), 0.2, s.ad_value(193), 0.2, s.ad_value(194), 0.2, 0.0);
            s.store_scale(235, 236, (1.0 - p.p95));
            s.store_add_scaled_inputs(331, 223, 1.0, 236, p.p95);
            s.store_add_scaled_inputs(221, 331, p.p94, 224, 1.0);
            s.store_scale(220, 331, (1.0 - p.p94));
        }

        if (!s.b[595]) {
            s.copy_ad(220, 223);
            s.copy_ad(221, 224);
            s.copy_ad(235, 236);
        }

        s.store_div_scaled_inputs2_indices(327, 155, 1.0, 154, 1.0, 153, 1.0);

        s.b[601] = (s.v[327] > 0.0);
        s.v[601] = if s.b[601] { 1.0 } else { 0.0 };

        if s.b[601] {
            s.store_div_scaled_inputs2_indices(329, 220, 1.0, 221, 1.0, 327, 1.0);
        }

        if (!s.b[601]) {
            s.store_scaled_mul(329, 184, 153, s.v[94]);
        }

        s.b[602] = (p.p131 == 1.0);
        s.v[602] = if s.b[602] { 1.0 } else { 0.0 };

        if s.b[602] {
            s.store_scale(330, 329, p.p94);
        }

        s.b[603] = (p.p131 == 2.0);
        s.v[603] = if s.b[603] { 1.0 } else { 0.0 };

        if ((!s.b[602]) && s.b[603]) {
            s.store_scale(330, 329, p.p132);
        }

        if ((!s.b[602]) && (!s.b[603])) {
            s.store_scalar(330, 0.0);
        }

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        var_qb1b2: f64,
        var_qb1b2_db0: f64,
        var_qb1b2_db1: f64,
        var_qb1b2_dn0: f64,
        var_qb1b2_dn1: f64,
        var_qb1b2_dn10: f64,
        var_qb1b2_dn11: f64,
        var_qb1b2_dn2: f64,
        var_qb1b2_dn3: f64,
        var_qb1b2_dn4: f64,
        var_qb1b2_dn5: f64,
        var_qb1b2_dn6: f64,
        var_qb1b2_dn7: f64,
        var_qb1b2_dn8: f64,
        var_qb1b2_dn9: f64,
        var_qbc: f64,
        var_qbc_db0: f64,
        var_qbc_db1: f64,
        var_qbc_dn0: f64,
        var_qbc_dn1: f64,
        var_qbc_dn10: f64,
        var_qbc_dn11: f64,
        var_qbc_dn2: f64,
        var_qbc_dn3: f64,
        var_qbc_dn4: f64,
        var_qbc_dn5: f64,
        var_qbc_dn6: f64,
        var_qbc_dn7: f64,
        var_qbc_dn8: f64,
        var_qbc_dn9: f64,
        var_qbe: f64,
        var_qbe_db0: f64,
        var_qbe_db1: f64,
        var_qbe_dn0: f64,
        var_qbe_dn1: f64,
        var_qbe_dn10: f64,
        var_qbe_dn11: f64,
        var_qbe_dn2: f64,
        var_qbe_dn3: f64,
        var_qbe_dn4: f64,
        var_qbe_dn5: f64,
        var_qbe_dn6: f64,
        var_qbe_dn7: f64,
        var_qbe_dn8: f64,
        var_qbe_dn9: f64,
        var_qe: f64,
        var_qe_db0: f64,
        var_qe_db1: f64,
        var_qe_dn0: f64,
        var_qe_dn1: f64,
        var_qe_dn10: f64,
        var_qe_dn11: f64,
        var_qe_dn2: f64,
        var_qe_dn3: f64,
        var_qe_dn4: f64,
        var_qe_dn5: f64,
        var_qe_dn6: f64,
        var_qe_dn7: f64,
        var_qe_dn8: f64,
        var_qe_dn9: f64,
        var_qepi: f64,
        var_qepi_db0: f64,
        var_qepi_db1: f64,
        var_qepi_dn0: f64,
        var_qepi_dn1: f64,
        var_qepi_dn10: f64,
        var_qepi_dn11: f64,
        var_qepi_dn2: f64,
        var_qepi_dn3: f64,
        var_qepi_dn4: f64,
        var_qepi_dn5: f64,
        var_qepi_dn6: f64,
        var_qepi_dn7: f64,
        var_qepi_dn8: f64,
        var_qepi_dn9: f64,
        var_qex: f64,
        var_qex_db0: f64,
        var_qex_db1: f64,
        var_qex_dn0: f64,
        var_qex_dn1: f64,
        var_qex_dn10: f64,
        var_qex_dn11: f64,
        var_qex_dn2: f64,
        var_qex_dn3: f64,
        var_qex_dn4: f64,
        var_qex_dn5: f64,
        var_qex_dn6: f64,
        var_qex_dn7: f64,
        var_qex_dn8: f64,
        var_qex_dn9: f64,
        var_qtc: f64,
        var_qtc_db0: f64,
        var_qtc_db1: f64,
        var_qtc_dn0: f64,
        var_qtc_dn1: f64,
        var_qtc_dn10: f64,
        var_qtc_dn11: f64,
        var_qtc_dn2: f64,
        var_qtc_dn3: f64,
        var_qtc_dn4: f64,
        var_qtc_dn5: f64,
        var_qtc_dn6: f64,
        var_qtc_dn7: f64,
        var_qtc_dn8: f64,
        var_qtc_dn9: f64,
        var_qte: f64,
        var_qte_db0: f64,
        var_qte_db1: f64,
        var_qte_dn0: f64,
        var_qte_dn1: f64,
        var_qte_dn10: f64,
        var_qte_dn11: f64,
        var_qte_dn2: f64,
        var_qte_dn3: f64,
        var_qte_dn4: f64,
        var_qte_dn5: f64,
        var_qte_dn6: f64,
        var_qte_dn7: f64,
        var_qte_dn8: f64,
        var_qte_dn9: f64,
        var_qte_s: f64,
        var_qte_s_db0: f64,
        var_qte_s_db1: f64,
        var_qte_s_dn0: f64,
        var_qte_s_dn1: f64,
        var_qte_s_dn10: f64,
        var_qte_s_dn11: f64,
        var_qte_s_dn2: f64,
        var_qte_s_dn3: f64,
        var_qte_s_dn4: f64,
        var_qte_s_dn5: f64,
        var_qte_s_dn6: f64,
        var_qte_s_dn7: f64,
        var_qte_s_dn8: f64,
        var_qte_s_dn9: f64,
        var_qtex: f64,
        var_qtex_db0: f64,
        var_qtex_db1: f64,
        var_qtex_dn0: f64,
        var_qtex_dn1: f64,
        var_qtex_dn10: f64,
        var_qtex_dn11: f64,
        var_qtex_dn2: f64,
        var_qtex_dn3: f64,
        var_qtex_dn4: f64,
        var_qtex_dn5: f64,
        var_qtex_dn6: f64,
        var_qtex_dn7: f64,
        var_qtex_dn8: f64,
        var_qtex_dn9: f64,
        var_qts: f64,
        var_qts_db0: f64,
        var_qts_db1: f64,
        var_qts_dn0: f64,
        var_qts_dn1: f64,
        var_qts_dn10: f64,
        var_qts_dn11: f64,
        var_qts_dn2: f64,
        var_qts_dn3: f64,
        var_qts_dn4: f64,
        var_qts_dn5: f64,
        var_qts_dn6: f64,
        var_qts_dn7: f64,
        var_qts_dn8: f64,
        var_qts_dn9: f64,
        var_taun: f64,
        var_taun_db0: f64,
        var_taun_db1: f64,
        var_taun_dn0: f64,
        var_taun_dn1: f64,
        var_taun_dn10: f64,
        var_taun_dn11: f64,
        var_taun_dn2: f64,
        var_taun_dn3: f64,
        var_taun_dn4: f64,
        var_taun_dn5: f64,
        var_taun_dn6: f64,
        var_taun_dn7: f64,
        var_taun_dn8: f64,
        var_taun_dn9: f64,
        var_vbc: f64,
        var_vbc_db0: f64,
        var_vbc_db1: f64,
        var_vbc_dn0: f64,
        var_vbc_dn1: f64,
        var_vbc_dn10: f64,
        var_vbc_dn11: f64,
        var_vbc_dn2: f64,
        var_vbc_dn3: f64,
        var_vbc_dn4: f64,
        var_vbc_dn5: f64,
        var_vbc_dn6: f64,
        var_vbc_dn7: f64,
        var_vbc_dn8: f64,
        var_vbc_dn9: f64,
        var_vbe: f64,
        var_vbe_db0: f64,
        var_vbe_db1: f64,
        var_vbe_dn0: f64,
        var_vbe_dn1: f64,
        var_vbe_dn10: f64,
        var_vbe_dn11: f64,
        var_vbe_dn2: f64,
        var_vbe_dn3: f64,
        var_vbe_dn4: f64,
        var_vbe_dn5: f64,
        var_vbe_dn6: f64,
        var_vbe_dn7: f64,
        var_vbe_dn8: f64,
        var_vbe_dn9: f64,
        var_xqex: f64,
        var_xqex_db0: f64,
        var_xqex_db1: f64,
        var_xqex_dn0: f64,
        var_xqex_dn1: f64,
        var_xqex_dn10: f64,
        var_xqex_dn11: f64,
        var_xqex_dn2: f64,
        var_xqex_dn3: f64,
        var_xqex_dn4: f64,
        var_xqex_dn5: f64,
        var_xqex_dn6: f64,
        var_xqex_dn7: f64,
        var_xqex_dn8: f64,
        var_xqex_dn9: f64,
        var_xqtex: f64,
        var_xqtex_db0: f64,
        var_xqtex_db1: f64,
        var_xqtex_dn0: f64,
        var_xqtex_dn1: f64,
        var_xqtex_dn10: f64,
        var_xqtex_dn11: f64,
        var_xqtex_dn2: f64,
        var_xqtex_dn3: f64,
        var_xqtex_dn4: f64,
        var_xqtex_dn5: f64,
        var_xqtex_dn6: f64,
        var_xqtex_dn7: f64,
        var_xqtex_dn8: f64,
        var_xqtex_dn9: f64,
    ) {
        let nv11 = ctx.node_voltage(nodes[11]);
        let eq14_e266: f64 = (var_qte + var_qbe);
        let eq14_e266_d_n0: f64 = (var_qte_dn0 + var_qbe_dn0);
        let eq14_e266_d_n1: f64 = (var_qte_dn1 + var_qbe_dn1);
        let eq14_e266_d_n2: f64 = (var_qte_dn2 + var_qbe_dn2);
        let eq14_e266_d_n3: f64 = (var_qte_dn3 + var_qbe_dn3);
        let eq14_e266_d_n4: f64 = (var_qte_dn4 + var_qbe_dn4);
        let eq14_e266_d_n5: f64 = (var_qte_dn5 + var_qbe_dn5);
        let eq14_e266_d_n6: f64 = (var_qte_dn6 + var_qbe_dn6);
        let eq14_e266_d_n7: f64 = (var_qte_dn7 + var_qbe_dn7);
        let eq14_e266_d_n8: f64 = (var_qte_dn8 + var_qbe_dn8);
        let eq14_e266_d_n9: f64 = (var_qte_dn9 + var_qbe_dn9);
        let eq14_e266_d_n10: f64 = (var_qte_dn10 + var_qbe_dn10);
        let eq14_e266_d_n11: f64 = (var_qte_dn11 + var_qbe_dn11);
        let eq14_e266_d_b0: f64 = (var_qte_db0 + var_qbe_db0);
        let eq14_e266_d_b1: f64 = (var_qte_db1 + var_qbe_db1);
        let eq14_e268: f64 = (eq14_e266 + var_qe);
        let eq14_e268_d_n0: f64 = (eq14_e266_d_n0 + var_qe_dn0);
        let eq14_e268_d_n1: f64 = (eq14_e266_d_n1 + var_qe_dn1);
        let eq14_e268_d_n2: f64 = (eq14_e266_d_n2 + var_qe_dn2);
        let eq14_e268_d_n3: f64 = (eq14_e266_d_n3 + var_qe_dn3);
        let eq14_e268_d_n4: f64 = (eq14_e266_d_n4 + var_qe_dn4);
        let eq14_e268_d_n5: f64 = (eq14_e266_d_n5 + var_qe_dn5);
        let eq14_e268_d_n6: f64 = (eq14_e266_d_n6 + var_qe_dn6);
        let eq14_e268_d_n7: f64 = (eq14_e266_d_n7 + var_qe_dn7);
        let eq14_e268_d_n8: f64 = (eq14_e266_d_n8 + var_qe_dn8);
        let eq14_e268_d_n9: f64 = (eq14_e266_d_n9 + var_qe_dn9);
        let eq14_e268_d_n10: f64 = (eq14_e266_d_n10 + var_qe_dn10);
        let eq14_e268_d_n11: f64 = (eq14_e266_d_n11 + var_qe_dn11);
        let eq14_e268_d_b0: f64 = (eq14_e266_d_b0 + var_qe_db0);
        let eq14_e268_d_b1: f64 = (eq14_e266_d_b1 + var_qe_db1);
        let eq14_e269: f64 = (p.p3 * eq14_e268);
        let eq14_e269_d_n0: f64 = (p.p3 * eq14_e268_d_n0);
        let eq14_e269_d_n1: f64 = (p.p3 * eq14_e268_d_n1);
        let eq14_e269_d_n2: f64 = (p.p3 * eq14_e268_d_n2);
        let eq14_e269_d_n3: f64 = (p.p3 * eq14_e268_d_n3);
        let eq14_e269_d_n4: f64 = (p.p3 * eq14_e268_d_n4);
        let eq14_e269_d_n5: f64 = (p.p3 * eq14_e268_d_n5);
        let eq14_e269_d_n6: f64 = (p.p3 * eq14_e268_d_n6);
        let eq14_e269_d_n7: f64 = (p.p3 * eq14_e268_d_n7);
        let eq14_e269_d_n8: f64 = (p.p3 * eq14_e268_d_n8);
        let eq14_e269_d_n9: f64 = (p.p3 * eq14_e268_d_n9);
        let eq14_e269_d_n10: f64 = (p.p3 * eq14_e268_d_n10);
        let eq14_e269_d_n11: f64 = (p.p3 * eq14_e268_d_n11);
        let eq14_e269_d_b0: f64 = (p.p3 * eq14_e268_d_b0);
        let eq14_e269_d_b1: f64 = (p.p3 * eq14_e268_d_b1);
        let eq14_e270: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, eq14_e269);
        let eq14_e272: f64 = (eq14_e270 * p.p1);
        let eq14_e272_d_n0: f64 = ((eq14_e269_d_n0 * ddt_scale) * p.p1);
        let eq14_e272_d_n1: f64 = ((eq14_e269_d_n1 * ddt_scale) * p.p1);
        let eq14_e272_d_n2: f64 = ((eq14_e269_d_n2 * ddt_scale) * p.p1);
        let eq14_e272_d_n3: f64 = ((eq14_e269_d_n3 * ddt_scale) * p.p1);
        let eq14_e272_d_n4: f64 = ((eq14_e269_d_n4 * ddt_scale) * p.p1);
        let eq14_e272_d_n5: f64 = ((eq14_e269_d_n5 * ddt_scale) * p.p1);
        let eq14_e272_d_n6: f64 = ((eq14_e269_d_n6 * ddt_scale) * p.p1);
        let eq14_e272_d_n7: f64 = ((eq14_e269_d_n7 * ddt_scale) * p.p1);
        let eq14_e272_d_n8: f64 = ((eq14_e269_d_n8 * ddt_scale) * p.p1);
        let eq14_e272_d_n9: f64 = ((eq14_e269_d_n9 * ddt_scale) * p.p1);
        let eq14_e272_d_n10: f64 = ((eq14_e269_d_n10 * ddt_scale) * p.p1);
        let eq14_e272_d_n11: f64 = ((eq14_e269_d_n11 * ddt_scale) * p.p1);
        let eq14_e272_d_b0: f64 = ((eq14_e269_d_b0 * ddt_scale) * p.p1);
        let eq14_e272_d_b1: f64 = ((eq14_e269_d_b1 * ddt_scale) * p.p1);
        let eq14_value: f64 = eq14_e272;
        let eq14_node_derivatives: [f64; 12] = [eq14_e272_d_n0, eq14_e272_d_n1, eq14_e272_d_n2, eq14_e272_d_n3, eq14_e272_d_n4, eq14_e272_d_n5, eq14_e272_d_n6, eq14_e272_d_n7, eq14_e272_d_n8, eq14_e272_d_n9, eq14_e272_d_n10, eq14_e272_d_n11];
        let eq14_branch_derivatives: [f64; 2] = [eq14_e272_d_b0, eq14_e272_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(4),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let eq15_e275: f64 = (p.p3 * var_qte_s);
        let eq15_e275_d_n0: f64 = (p.p3 * var_qte_s_dn0);
        let eq15_e275_d_n1: f64 = (p.p3 * var_qte_s_dn1);
        let eq15_e275_d_n2: f64 = (p.p3 * var_qte_s_dn2);
        let eq15_e275_d_n3: f64 = (p.p3 * var_qte_s_dn3);
        let eq15_e275_d_n4: f64 = (p.p3 * var_qte_s_dn4);
        let eq15_e275_d_n5: f64 = (p.p3 * var_qte_s_dn5);
        let eq15_e275_d_n6: f64 = (p.p3 * var_qte_s_dn6);
        let eq15_e275_d_n7: f64 = (p.p3 * var_qte_s_dn7);
        let eq15_e275_d_n8: f64 = (p.p3 * var_qte_s_dn8);
        let eq15_e275_d_n9: f64 = (p.p3 * var_qte_s_dn9);
        let eq15_e275_d_n10: f64 = (p.p3 * var_qte_s_dn10);
        let eq15_e275_d_n11: f64 = (p.p3 * var_qte_s_dn11);
        let eq15_e275_d_b0: f64 = (p.p3 * var_qte_s_db0);
        let eq15_e275_d_b1: f64 = (p.p3 * var_qte_s_db1);
        let eq15_e276: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, eq15_e275);
        let eq15_e278: f64 = (eq15_e276 * p.p1);
        let eq15_e278_d_n0: f64 = ((eq15_e275_d_n0 * ddt_scale) * p.p1);
        let eq15_e278_d_n1: f64 = ((eq15_e275_d_n1 * ddt_scale) * p.p1);
        let eq15_e278_d_n2: f64 = ((eq15_e275_d_n2 * ddt_scale) * p.p1);
        let eq15_e278_d_n3: f64 = ((eq15_e275_d_n3 * ddt_scale) * p.p1);
        let eq15_e278_d_n4: f64 = ((eq15_e275_d_n4 * ddt_scale) * p.p1);
        let eq15_e278_d_n5: f64 = ((eq15_e275_d_n5 * ddt_scale) * p.p1);
        let eq15_e278_d_n6: f64 = ((eq15_e275_d_n6 * ddt_scale) * p.p1);
        let eq15_e278_d_n7: f64 = ((eq15_e275_d_n7 * ddt_scale) * p.p1);
        let eq15_e278_d_n8: f64 = ((eq15_e275_d_n8 * ddt_scale) * p.p1);
        let eq15_e278_d_n9: f64 = ((eq15_e275_d_n9 * ddt_scale) * p.p1);
        let eq15_e278_d_n10: f64 = ((eq15_e275_d_n10 * ddt_scale) * p.p1);
        let eq15_e278_d_n11: f64 = ((eq15_e275_d_n11 * ddt_scale) * p.p1);
        let eq15_e278_d_b0: f64 = ((eq15_e275_d_b0 * ddt_scale) * p.p1);
        let eq15_e278_d_b1: f64 = ((eq15_e275_d_b1 * ddt_scale) * p.p1);
        let eq15_value: f64 = eq15_e278;
        let eq15_node_derivatives: [f64; 12] = [eq15_e278_d_n0, eq15_e278_d_n1, eq15_e278_d_n2, eq15_e278_d_n3, eq15_e278_d_n4, eq15_e278_d_n5, eq15_e278_d_n6, eq15_e278_d_n7, eq15_e278_d_n8, eq15_e278_d_n9, eq15_e278_d_n10, eq15_e278_d_n11];
        let eq15_branch_derivatives: [f64; 2] = [eq15_e278_d_b0, eq15_e278_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(4),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let eq16_e282: f64 = (var_qtc + var_qbc);
        let eq16_e282_d_n0: f64 = (var_qtc_dn0 + var_qbc_dn0);
        let eq16_e282_d_n1: f64 = (var_qtc_dn1 + var_qbc_dn1);
        let eq16_e282_d_n2: f64 = (var_qtc_dn2 + var_qbc_dn2);
        let eq16_e282_d_n3: f64 = (var_qtc_dn3 + var_qbc_dn3);
        let eq16_e282_d_n4: f64 = (var_qtc_dn4 + var_qbc_dn4);
        let eq16_e282_d_n5: f64 = (var_qtc_dn5 + var_qbc_dn5);
        let eq16_e282_d_n6: f64 = (var_qtc_dn6 + var_qbc_dn6);
        let eq16_e282_d_n7: f64 = (var_qtc_dn7 + var_qbc_dn7);
        let eq16_e282_d_n8: f64 = (var_qtc_dn8 + var_qbc_dn8);
        let eq16_e282_d_n9: f64 = (var_qtc_dn9 + var_qbc_dn9);
        let eq16_e282_d_n10: f64 = (var_qtc_dn10 + var_qbc_dn10);
        let eq16_e282_d_n11: f64 = (var_qtc_dn11 + var_qbc_dn11);
        let eq16_e282_d_b0: f64 = (var_qtc_db0 + var_qbc_db0);
        let eq16_e282_d_b1: f64 = (var_qtc_db1 + var_qbc_db1);
        let eq16_e284: f64 = (eq16_e282 + var_qepi);
        let eq16_e284_d_n0: f64 = (eq16_e282_d_n0 + var_qepi_dn0);
        let eq16_e284_d_n1: f64 = (eq16_e282_d_n1 + var_qepi_dn1);
        let eq16_e284_d_n2: f64 = (eq16_e282_d_n2 + var_qepi_dn2);
        let eq16_e284_d_n3: f64 = (eq16_e282_d_n3 + var_qepi_dn3);
        let eq16_e284_d_n4: f64 = (eq16_e282_d_n4 + var_qepi_dn4);
        let eq16_e284_d_n5: f64 = (eq16_e282_d_n5 + var_qepi_dn5);
        let eq16_e284_d_n6: f64 = (eq16_e282_d_n6 + var_qepi_dn6);
        let eq16_e284_d_n7: f64 = (eq16_e282_d_n7 + var_qepi_dn7);
        let eq16_e284_d_n8: f64 = (eq16_e282_d_n8 + var_qepi_dn8);
        let eq16_e284_d_n9: f64 = (eq16_e282_d_n9 + var_qepi_dn9);
        let eq16_e284_d_n10: f64 = (eq16_e282_d_n10 + var_qepi_dn10);
        let eq16_e284_d_n11: f64 = (eq16_e282_d_n11 + var_qepi_dn11);
        let eq16_e284_d_b0: f64 = (eq16_e282_d_b0 + var_qepi_db0);
        let eq16_e284_d_b1: f64 = (eq16_e282_d_b1 + var_qepi_db1);
        let eq16_e285: f64 = (p.p3 * eq16_e284);
        let eq16_e285_d_n0: f64 = (p.p3 * eq16_e284_d_n0);
        let eq16_e285_d_n1: f64 = (p.p3 * eq16_e284_d_n1);
        let eq16_e285_d_n2: f64 = (p.p3 * eq16_e284_d_n2);
        let eq16_e285_d_n3: f64 = (p.p3 * eq16_e284_d_n3);
        let eq16_e285_d_n4: f64 = (p.p3 * eq16_e284_d_n4);
        let eq16_e285_d_n5: f64 = (p.p3 * eq16_e284_d_n5);
        let eq16_e285_d_n6: f64 = (p.p3 * eq16_e284_d_n6);
        let eq16_e285_d_n7: f64 = (p.p3 * eq16_e284_d_n7);
        let eq16_e285_d_n8: f64 = (p.p3 * eq16_e284_d_n8);
        let eq16_e285_d_n9: f64 = (p.p3 * eq16_e284_d_n9);
        let eq16_e285_d_n10: f64 = (p.p3 * eq16_e284_d_n10);
        let eq16_e285_d_n11: f64 = (p.p3 * eq16_e284_d_n11);
        let eq16_e285_d_b0: f64 = (p.p3 * eq16_e284_d_b0);
        let eq16_e285_d_b1: f64 = (p.p3 * eq16_e284_d_b1);
        let eq16_e286: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, eq16_e285);
        let eq16_e288: f64 = (eq16_e286 * p.p1);
        let eq16_e288_d_n0: f64 = ((eq16_e285_d_n0 * ddt_scale) * p.p1);
        let eq16_e288_d_n1: f64 = ((eq16_e285_d_n1 * ddt_scale) * p.p1);
        let eq16_e288_d_n2: f64 = ((eq16_e285_d_n2 * ddt_scale) * p.p1);
        let eq16_e288_d_n3: f64 = ((eq16_e285_d_n3 * ddt_scale) * p.p1);
        let eq16_e288_d_n4: f64 = ((eq16_e285_d_n4 * ddt_scale) * p.p1);
        let eq16_e288_d_n5: f64 = ((eq16_e285_d_n5 * ddt_scale) * p.p1);
        let eq16_e288_d_n6: f64 = ((eq16_e285_d_n6 * ddt_scale) * p.p1);
        let eq16_e288_d_n7: f64 = ((eq16_e285_d_n7 * ddt_scale) * p.p1);
        let eq16_e288_d_n8: f64 = ((eq16_e285_d_n8 * ddt_scale) * p.p1);
        let eq16_e288_d_n9: f64 = ((eq16_e285_d_n9 * ddt_scale) * p.p1);
        let eq16_e288_d_n10: f64 = ((eq16_e285_d_n10 * ddt_scale) * p.p1);
        let eq16_e288_d_n11: f64 = ((eq16_e285_d_n11 * ddt_scale) * p.p1);
        let eq16_e288_d_b0: f64 = ((eq16_e285_d_b0 * ddt_scale) * p.p1);
        let eq16_e288_d_b1: f64 = ((eq16_e285_d_b1 * ddt_scale) * p.p1);
        let eq16_value: f64 = eq16_e288;
        let eq16_node_derivatives: [f64; 12] = [eq16_e288_d_n0, eq16_e288_d_n1, eq16_e288_d_n2, eq16_e288_d_n3, eq16_e288_d_n4, eq16_e288_d_n5, eq16_e288_d_n6, eq16_e288_d_n7, eq16_e288_d_n8, eq16_e288_d_n9, eq16_e288_d_n10, eq16_e288_d_n11];
        let eq16_branch_derivatives: [f64; 2] = [eq16_e288_d_b0, eq16_e288_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq16_value),
            &eq16_node_derivatives,
            &eq16_branch_derivatives,
            multiplicity,
        );
        let eq17_e291: f64 = (p.p3 * var_qts);
        let eq17_e291_d_n0: f64 = (p.p3 * var_qts_dn0);
        let eq17_e291_d_n1: f64 = (p.p3 * var_qts_dn1);
        let eq17_e291_d_n2: f64 = (p.p3 * var_qts_dn2);
        let eq17_e291_d_n3: f64 = (p.p3 * var_qts_dn3);
        let eq17_e291_d_n4: f64 = (p.p3 * var_qts_dn4);
        let eq17_e291_d_n5: f64 = (p.p3 * var_qts_dn5);
        let eq17_e291_d_n6: f64 = (p.p3 * var_qts_dn6);
        let eq17_e291_d_n7: f64 = (p.p3 * var_qts_dn7);
        let eq17_e291_d_n8: f64 = (p.p3 * var_qts_dn8);
        let eq17_e291_d_n9: f64 = (p.p3 * var_qts_dn9);
        let eq17_e291_d_n10: f64 = (p.p3 * var_qts_dn10);
        let eq17_e291_d_n11: f64 = (p.p3 * var_qts_dn11);
        let eq17_e291_d_b0: f64 = (p.p3 * var_qts_db0);
        let eq17_e291_d_b1: f64 = (p.p3 * var_qts_db1);
        let eq17_e292: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, eq17_e291);
        let eq17_e294: f64 = (eq17_e292 * p.p1);
        let eq17_e294_d_n0: f64 = ((eq17_e291_d_n0 * ddt_scale) * p.p1);
        let eq17_e294_d_n1: f64 = ((eq17_e291_d_n1 * ddt_scale) * p.p1);
        let eq17_e294_d_n2: f64 = ((eq17_e291_d_n2 * ddt_scale) * p.p1);
        let eq17_e294_d_n3: f64 = ((eq17_e291_d_n3 * ddt_scale) * p.p1);
        let eq17_e294_d_n4: f64 = ((eq17_e291_d_n4 * ddt_scale) * p.p1);
        let eq17_e294_d_n5: f64 = ((eq17_e291_d_n5 * ddt_scale) * p.p1);
        let eq17_e294_d_n6: f64 = ((eq17_e291_d_n6 * ddt_scale) * p.p1);
        let eq17_e294_d_n7: f64 = ((eq17_e291_d_n7 * ddt_scale) * p.p1);
        let eq17_e294_d_n8: f64 = ((eq17_e291_d_n8 * ddt_scale) * p.p1);
        let eq17_e294_d_n9: f64 = ((eq17_e291_d_n9 * ddt_scale) * p.p1);
        let eq17_e294_d_n10: f64 = ((eq17_e291_d_n10 * ddt_scale) * p.p1);
        let eq17_e294_d_n11: f64 = ((eq17_e291_d_n11 * ddt_scale) * p.p1);
        let eq17_e294_d_b0: f64 = ((eq17_e291_d_b0 * ddt_scale) * p.p1);
        let eq17_e294_d_b1: f64 = ((eq17_e291_d_b1 * ddt_scale) * p.p1);
        let eq17_value: f64 = eq17_e294;
        let eq17_node_derivatives: [f64; 12] = [eq17_e294_d_n0, eq17_e294_d_n1, eq17_e294_d_n2, eq17_e294_d_n3, eq17_e294_d_n4, eq17_e294_d_n5, eq17_e294_d_n6, eq17_e294_d_n7, eq17_e294_d_n8, eq17_e294_d_n9, eq17_e294_d_n10, eq17_e294_d_n11];
        let eq17_branch_derivatives: [f64; 2] = [eq17_e294_d_b0, eq17_e294_d_b1];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(7),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let eq18_e297: f64 = (p.p3 * var_qb1b2);
        let eq18_e297_d_n0: f64 = (p.p3 * var_qb1b2_dn0);
        let eq18_e297_d_n1: f64 = (p.p3 * var_qb1b2_dn1);
        let eq18_e297_d_n2: f64 = (p.p3 * var_qb1b2_dn2);
        let eq18_e297_d_n3: f64 = (p.p3 * var_qb1b2_dn3);
        let eq18_e297_d_n4: f64 = (p.p3 * var_qb1b2_dn4);
        let eq18_e297_d_n5: f64 = (p.p3 * var_qb1b2_dn5);
        let eq18_e297_d_n6: f64 = (p.p3 * var_qb1b2_dn6);
        let eq18_e297_d_n7: f64 = (p.p3 * var_qb1b2_dn7);
        let eq18_e297_d_n8: f64 = (p.p3 * var_qb1b2_dn8);
        let eq18_e297_d_n9: f64 = (p.p3 * var_qb1b2_dn9);
        let eq18_e297_d_n10: f64 = (p.p3 * var_qb1b2_dn10);
        let eq18_e297_d_n11: f64 = (p.p3 * var_qb1b2_dn11);
        let eq18_e297_d_b0: f64 = (p.p3 * var_qb1b2_db0);
        let eq18_e297_d_b1: f64 = (p.p3 * var_qb1b2_db1);
        let eq18_e298: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, eq18_e297);
        let eq18_e300: f64 = (eq18_e298 * p.p1);
        let eq18_e300_d_n0: f64 = ((eq18_e297_d_n0 * ddt_scale) * p.p1);
        let eq18_e300_d_n1: f64 = ((eq18_e297_d_n1 * ddt_scale) * p.p1);
        let eq18_e300_d_n2: f64 = ((eq18_e297_d_n2 * ddt_scale) * p.p1);
        let eq18_e300_d_n3: f64 = ((eq18_e297_d_n3 * ddt_scale) * p.p1);
        let eq18_e300_d_n4: f64 = ((eq18_e297_d_n4 * ddt_scale) * p.p1);
        let eq18_e300_d_n5: f64 = ((eq18_e297_d_n5 * ddt_scale) * p.p1);
        let eq18_e300_d_n6: f64 = ((eq18_e297_d_n6 * ddt_scale) * p.p1);
        let eq18_e300_d_n7: f64 = ((eq18_e297_d_n7 * ddt_scale) * p.p1);
        let eq18_e300_d_n8: f64 = ((eq18_e297_d_n8 * ddt_scale) * p.p1);
        let eq18_e300_d_n9: f64 = ((eq18_e297_d_n9 * ddt_scale) * p.p1);
        let eq18_e300_d_n10: f64 = ((eq18_e297_d_n10 * ddt_scale) * p.p1);
        let eq18_e300_d_n11: f64 = ((eq18_e297_d_n11 * ddt_scale) * p.p1);
        let eq18_e300_d_b0: f64 = ((eq18_e297_d_b0 * ddt_scale) * p.p1);
        let eq18_e300_d_b1: f64 = ((eq18_e297_d_b1 * ddt_scale) * p.p1);
        let eq18_value: f64 = eq18_e300;
        let eq18_node_derivatives: [f64; 12] = [eq18_e300_d_n0, eq18_e300_d_n1, eq18_e300_d_n2, eq18_e300_d_n3, eq18_e300_d_n4, eq18_e300_d_n5, eq18_e300_d_n6, eq18_e300_d_n7, eq18_e300_d_n8, eq18_e300_d_n9, eq18_e300_d_n10, eq18_e300_d_n11];
        let eq18_branch_derivatives: [f64; 2] = [eq18_e300_d_b0, eq18_e300_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let eq19_e303: f64 = (p.p3 * p.p69);
        let eq19_e305: f64 = (eq19_e303 * var_vbe);
        let eq19_e305_d_n0: f64 = (eq19_e303 * var_vbe_dn0);
        let eq19_e305_d_n1: f64 = (eq19_e303 * var_vbe_dn1);
        let eq19_e305_d_n2: f64 = (eq19_e303 * var_vbe_dn2);
        let eq19_e305_d_n3: f64 = (eq19_e303 * var_vbe_dn3);
        let eq19_e305_d_n4: f64 = (eq19_e303 * var_vbe_dn4);
        let eq19_e305_d_n5: f64 = (eq19_e303 * var_vbe_dn5);
        let eq19_e305_d_n6: f64 = (eq19_e303 * var_vbe_dn6);
        let eq19_e305_d_n7: f64 = (eq19_e303 * var_vbe_dn7);
        let eq19_e305_d_n8: f64 = (eq19_e303 * var_vbe_dn8);
        let eq19_e305_d_n9: f64 = (eq19_e303 * var_vbe_dn9);
        let eq19_e305_d_n10: f64 = (eq19_e303 * var_vbe_dn10);
        let eq19_e305_d_n11: f64 = (eq19_e303 * var_vbe_dn11);
        let eq19_e305_d_b0: f64 = (eq19_e303 * var_vbe_db0);
        let eq19_e305_d_b1: f64 = (eq19_e303 * var_vbe_db1);
        let eq19_e306: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, eq19_e305);
        let eq19_e308: f64 = (eq19_e306 * p.p1);
        let eq19_e308_d_n0: f64 = ((eq19_e305_d_n0 * ddt_scale) * p.p1);
        let eq19_e308_d_n1: f64 = ((eq19_e305_d_n1 * ddt_scale) * p.p1);
        let eq19_e308_d_n2: f64 = ((eq19_e305_d_n2 * ddt_scale) * p.p1);
        let eq19_e308_d_n3: f64 = ((eq19_e305_d_n3 * ddt_scale) * p.p1);
        let eq19_e308_d_n4: f64 = ((eq19_e305_d_n4 * ddt_scale) * p.p1);
        let eq19_e308_d_n5: f64 = ((eq19_e305_d_n5 * ddt_scale) * p.p1);
        let eq19_e308_d_n6: f64 = ((eq19_e305_d_n6 * ddt_scale) * p.p1);
        let eq19_e308_d_n7: f64 = ((eq19_e305_d_n7 * ddt_scale) * p.p1);
        let eq19_e308_d_n8: f64 = ((eq19_e305_d_n8 * ddt_scale) * p.p1);
        let eq19_e308_d_n9: f64 = ((eq19_e305_d_n9 * ddt_scale) * p.p1);
        let eq19_e308_d_n10: f64 = ((eq19_e305_d_n10 * ddt_scale) * p.p1);
        let eq19_e308_d_n11: f64 = ((eq19_e305_d_n11 * ddt_scale) * p.p1);
        let eq19_e308_d_b0: f64 = ((eq19_e305_d_b0 * ddt_scale) * p.p1);
        let eq19_e308_d_b1: f64 = ((eq19_e305_d_b1 * ddt_scale) * p.p1);
        let eq19_value: f64 = eq19_e308;
        let eq19_node_derivatives: [f64; 12] = [eq19_e308_d_n0, eq19_e308_d_n1, eq19_e308_d_n2, eq19_e308_d_n3, eq19_e308_d_n4, eq19_e308_d_n5, eq19_e308_d_n6, eq19_e308_d_n7, eq19_e308_d_n8, eq19_e308_d_n9, eq19_e308_d_n10, eq19_e308_d_n11];
        let eq19_branch_derivatives: [f64; 2] = [eq19_e308_d_b0, eq19_e308_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(2),
            multiplicity * (eq19_value),
            &eq19_node_derivatives,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let eq20_e311: f64 = (p.p3 * p.p78);
        let eq20_e313: f64 = (eq20_e311 * var_vbc);
        let eq20_e313_d_n0: f64 = (eq20_e311 * var_vbc_dn0);
        let eq20_e313_d_n1: f64 = (eq20_e311 * var_vbc_dn1);
        let eq20_e313_d_n2: f64 = (eq20_e311 * var_vbc_dn2);
        let eq20_e313_d_n3: f64 = (eq20_e311 * var_vbc_dn3);
        let eq20_e313_d_n4: f64 = (eq20_e311 * var_vbc_dn4);
        let eq20_e313_d_n5: f64 = (eq20_e311 * var_vbc_dn5);
        let eq20_e313_d_n6: f64 = (eq20_e311 * var_vbc_dn6);
        let eq20_e313_d_n7: f64 = (eq20_e311 * var_vbc_dn7);
        let eq20_e313_d_n8: f64 = (eq20_e311 * var_vbc_dn8);
        let eq20_e313_d_n9: f64 = (eq20_e311 * var_vbc_dn9);
        let eq20_e313_d_n10: f64 = (eq20_e311 * var_vbc_dn10);
        let eq20_e313_d_n11: f64 = (eq20_e311 * var_vbc_dn11);
        let eq20_e313_d_b0: f64 = (eq20_e311 * var_vbc_db0);
        let eq20_e313_d_b1: f64 = (eq20_e311 * var_vbc_db1);
        let eq20_e314: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, eq20_e313);
        let eq20_e316: f64 = (eq20_e314 * p.p1);
        let eq20_e316_d_n0: f64 = ((eq20_e313_d_n0 * ddt_scale) * p.p1);
        let eq20_e316_d_n1: f64 = ((eq20_e313_d_n1 * ddt_scale) * p.p1);
        let eq20_e316_d_n2: f64 = ((eq20_e313_d_n2 * ddt_scale) * p.p1);
        let eq20_e316_d_n3: f64 = ((eq20_e313_d_n3 * ddt_scale) * p.p1);
        let eq20_e316_d_n4: f64 = ((eq20_e313_d_n4 * ddt_scale) * p.p1);
        let eq20_e316_d_n5: f64 = ((eq20_e313_d_n5 * ddt_scale) * p.p1);
        let eq20_e316_d_n6: f64 = ((eq20_e313_d_n6 * ddt_scale) * p.p1);
        let eq20_e316_d_n7: f64 = ((eq20_e313_d_n7 * ddt_scale) * p.p1);
        let eq20_e316_d_n8: f64 = ((eq20_e313_d_n8 * ddt_scale) * p.p1);
        let eq20_e316_d_n9: f64 = ((eq20_e313_d_n9 * ddt_scale) * p.p1);
        let eq20_e316_d_n10: f64 = ((eq20_e313_d_n10 * ddt_scale) * p.p1);
        let eq20_e316_d_n11: f64 = ((eq20_e313_d_n11 * ddt_scale) * p.p1);
        let eq20_e316_d_b0: f64 = ((eq20_e313_d_b0 * ddt_scale) * p.p1);
        let eq20_e316_d_b1: f64 = ((eq20_e313_d_b1 * ddt_scale) * p.p1);
        let eq20_value: f64 = eq20_e316;
        let eq20_node_derivatives: [f64; 12] = [eq20_e316_d_n0, eq20_e316_d_n1, eq20_e316_d_n2, eq20_e316_d_n3, eq20_e316_d_n4, eq20_e316_d_n5, eq20_e316_d_n6, eq20_e316_d_n7, eq20_e316_d_n8, eq20_e316_d_n9, eq20_e316_d_n10, eq20_e316_d_n11];
        let eq20_branch_derivatives: [f64; 2] = [eq20_e316_d_b0, eq20_e316_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(0),
            multiplicity * (eq20_value),
            &eq20_node_derivatives,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let eq23_e332: f64 = (var_xqtex + var_xqex);
        let eq23_e332_d_n0: f64 = (var_xqtex_dn0 + var_xqex_dn0);
        let eq23_e332_d_n1: f64 = (var_xqtex_dn1 + var_xqex_dn1);
        let eq23_e332_d_n2: f64 = (var_xqtex_dn2 + var_xqex_dn2);
        let eq23_e332_d_n3: f64 = (var_xqtex_dn3 + var_xqex_dn3);
        let eq23_e332_d_n4: f64 = (var_xqtex_dn4 + var_xqex_dn4);
        let eq23_e332_d_n5: f64 = (var_xqtex_dn5 + var_xqex_dn5);
        let eq23_e332_d_n6: f64 = (var_xqtex_dn6 + var_xqex_dn6);
        let eq23_e332_d_n7: f64 = (var_xqtex_dn7 + var_xqex_dn7);
        let eq23_e332_d_n8: f64 = (var_xqtex_dn8 + var_xqex_dn8);
        let eq23_e332_d_n9: f64 = (var_xqtex_dn9 + var_xqex_dn9);
        let eq23_e332_d_n10: f64 = (var_xqtex_dn10 + var_xqex_dn10);
        let eq23_e332_d_n11: f64 = (var_xqtex_dn11 + var_xqex_dn11);
        let eq23_e332_d_b0: f64 = (var_xqtex_db0 + var_xqex_db0);
        let eq23_e332_d_b1: f64 = (var_xqtex_db1 + var_xqex_db1);
        let eq23_e333: f64 = (p.p3 * eq23_e332);
        let eq23_e333_d_n0: f64 = (p.p3 * eq23_e332_d_n0);
        let eq23_e333_d_n1: f64 = (p.p3 * eq23_e332_d_n1);
        let eq23_e333_d_n2: f64 = (p.p3 * eq23_e332_d_n2);
        let eq23_e333_d_n3: f64 = (p.p3 * eq23_e332_d_n3);
        let eq23_e333_d_n4: f64 = (p.p3 * eq23_e332_d_n4);
        let eq23_e333_d_n5: f64 = (p.p3 * eq23_e332_d_n5);
        let eq23_e333_d_n6: f64 = (p.p3 * eq23_e332_d_n6);
        let eq23_e333_d_n7: f64 = (p.p3 * eq23_e332_d_n7);
        let eq23_e333_d_n8: f64 = (p.p3 * eq23_e332_d_n8);
        let eq23_e333_d_n9: f64 = (p.p3 * eq23_e332_d_n9);
        let eq23_e333_d_n10: f64 = (p.p3 * eq23_e332_d_n10);
        let eq23_e333_d_n11: f64 = (p.p3 * eq23_e332_d_n11);
        let eq23_e333_d_b0: f64 = (p.p3 * eq23_e332_d_b0);
        let eq23_e333_d_b1: f64 = (p.p3 * eq23_e332_d_b1);
        let eq23_e334: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, eq23_e333);
        let eq23_e336: f64 = (eq23_e334 * p.p1);
        let eq23_e336_d_n0: f64 = ((eq23_e333_d_n0 * ddt_scale) * p.p1);
        let eq23_e336_d_n1: f64 = ((eq23_e333_d_n1 * ddt_scale) * p.p1);
        let eq23_e336_d_n2: f64 = ((eq23_e333_d_n2 * ddt_scale) * p.p1);
        let eq23_e336_d_n3: f64 = ((eq23_e333_d_n3 * ddt_scale) * p.p1);
        let eq23_e336_d_n4: f64 = ((eq23_e333_d_n4 * ddt_scale) * p.p1);
        let eq23_e336_d_n5: f64 = ((eq23_e333_d_n5 * ddt_scale) * p.p1);
        let eq23_e336_d_n6: f64 = ((eq23_e333_d_n6 * ddt_scale) * p.p1);
        let eq23_e336_d_n7: f64 = ((eq23_e333_d_n7 * ddt_scale) * p.p1);
        let eq23_e336_d_n8: f64 = ((eq23_e333_d_n8 * ddt_scale) * p.p1);
        let eq23_e336_d_n9: f64 = ((eq23_e333_d_n9 * ddt_scale) * p.p1);
        let eq23_e336_d_n10: f64 = ((eq23_e333_d_n10 * ddt_scale) * p.p1);
        let eq23_e336_d_n11: f64 = ((eq23_e333_d_n11 * ddt_scale) * p.p1);
        let eq23_e336_d_b0: f64 = ((eq23_e333_d_b0 * ddt_scale) * p.p1);
        let eq23_e336_d_b1: f64 = ((eq23_e333_d_b1 * ddt_scale) * p.p1);
        let eq23_value: f64 = eq23_e336;
        let eq23_node_derivatives: [f64; 12] = [eq23_e336_d_n0, eq23_e336_d_n1, eq23_e336_d_n2, eq23_e336_d_n3, eq23_e336_d_n4, eq23_e336_d_n5, eq23_e336_d_n6, eq23_e336_d_n7, eq23_e336_d_n8, eq23_e336_d_n9, eq23_e336_d_n10, eq23_e336_d_n11];
        let eq23_branch_derivatives: [f64; 2] = [eq23_e336_d_b0, eq23_e336_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(9),
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &eq23_branch_derivatives,
            multiplicity,
        );
        let eq25_e351: f64 = (var_qtex + var_qex);
        let eq25_e351_d_n0: f64 = (var_qtex_dn0 + var_qex_dn0);
        let eq25_e351_d_n1: f64 = (var_qtex_dn1 + var_qex_dn1);
        let eq25_e351_d_n2: f64 = (var_qtex_dn2 + var_qex_dn2);
        let eq25_e351_d_n3: f64 = (var_qtex_dn3 + var_qex_dn3);
        let eq25_e351_d_n4: f64 = (var_qtex_dn4 + var_qex_dn4);
        let eq25_e351_d_n5: f64 = (var_qtex_dn5 + var_qex_dn5);
        let eq25_e351_d_n6: f64 = (var_qtex_dn6 + var_qex_dn6);
        let eq25_e351_d_n7: f64 = (var_qtex_dn7 + var_qex_dn7);
        let eq25_e351_d_n8: f64 = (var_qtex_dn8 + var_qex_dn8);
        let eq25_e351_d_n9: f64 = (var_qtex_dn9 + var_qex_dn9);
        let eq25_e351_d_n10: f64 = (var_qtex_dn10 + var_qex_dn10);
        let eq25_e351_d_n11: f64 = (var_qtex_dn11 + var_qex_dn11);
        let eq25_e351_d_b0: f64 = (var_qtex_db0 + var_qex_db0);
        let eq25_e351_d_b1: f64 = (var_qtex_db1 + var_qex_db1);
        let eq25_e352: f64 = (p.p3 * eq25_e351);
        let eq25_e352_d_n0: f64 = (p.p3 * eq25_e351_d_n0);
        let eq25_e352_d_n1: f64 = (p.p3 * eq25_e351_d_n1);
        let eq25_e352_d_n2: f64 = (p.p3 * eq25_e351_d_n2);
        let eq25_e352_d_n3: f64 = (p.p3 * eq25_e351_d_n3);
        let eq25_e352_d_n4: f64 = (p.p3 * eq25_e351_d_n4);
        let eq25_e352_d_n5: f64 = (p.p3 * eq25_e351_d_n5);
        let eq25_e352_d_n6: f64 = (p.p3 * eq25_e351_d_n6);
        let eq25_e352_d_n7: f64 = (p.p3 * eq25_e351_d_n7);
        let eq25_e352_d_n8: f64 = (p.p3 * eq25_e351_d_n8);
        let eq25_e352_d_n9: f64 = (p.p3 * eq25_e351_d_n9);
        let eq25_e352_d_n10: f64 = (p.p3 * eq25_e351_d_n10);
        let eq25_e352_d_n11: f64 = (p.p3 * eq25_e351_d_n11);
        let eq25_e352_d_b0: f64 = (p.p3 * eq25_e351_d_b0);
        let eq25_e352_d_b1: f64 = (p.p3 * eq25_e351_d_b1);
        let eq25_e353: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, eq25_e352);
        let eq25_e355: f64 = (eq25_e353 * p.p1);
        let eq25_e355_d_n0: f64 = ((eq25_e352_d_n0 * ddt_scale) * p.p1);
        let eq25_e355_d_n1: f64 = ((eq25_e352_d_n1 * ddt_scale) * p.p1);
        let eq25_e355_d_n2: f64 = ((eq25_e352_d_n2 * ddt_scale) * p.p1);
        let eq25_e355_d_n3: f64 = ((eq25_e352_d_n3 * ddt_scale) * p.p1);
        let eq25_e355_d_n4: f64 = ((eq25_e352_d_n4 * ddt_scale) * p.p1);
        let eq25_e355_d_n5: f64 = ((eq25_e352_d_n5 * ddt_scale) * p.p1);
        let eq25_e355_d_n6: f64 = ((eq25_e352_d_n6 * ddt_scale) * p.p1);
        let eq25_e355_d_n7: f64 = ((eq25_e352_d_n7 * ddt_scale) * p.p1);
        let eq25_e355_d_n8: f64 = ((eq25_e352_d_n8 * ddt_scale) * p.p1);
        let eq25_e355_d_n9: f64 = ((eq25_e352_d_n9 * ddt_scale) * p.p1);
        let eq25_e355_d_n10: f64 = ((eq25_e352_d_n10 * ddt_scale) * p.p1);
        let eq25_e355_d_n11: f64 = ((eq25_e352_d_n11 * ddt_scale) * p.p1);
        let eq25_e355_d_b0: f64 = ((eq25_e352_d_b0 * ddt_scale) * p.p1);
        let eq25_e355_d_b1: f64 = ((eq25_e352_d_b1 * ddt_scale) * p.p1);
        let eq25_value: f64 = eq25_e355;
        let eq25_node_derivatives: [f64; 12] = [eq25_e355_d_n0, eq25_e355_d_n1, eq25_e355_d_n2, eq25_e355_d_n3, eq25_e355_d_n4, eq25_e355_d_n5, eq25_e355_d_n6, eq25_e355_d_n7, eq25_e355_d_n8, eq25_e355_d_n9, eq25_e355_d_n10, eq25_e355_d_n11];
        let eq25_branch_derivatives: [f64; 2] = [eq25_e355_d_b0, eq25_e355_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(10),
            multiplicity * (eq25_value),
            &eq25_node_derivatives,
            &eq25_branch_derivatives,
            multiplicity,
        );
        let eq32_e394: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 9, (nv11 - 0.0));
        let eq32_e395: f64 = (var_taun * eq32_e394);
        let eq32_e395_d_n0: f64 = (var_taun_dn0 * eq32_e394);
        let eq32_e395_d_n1: f64 = (var_taun_dn1 * eq32_e394);
        let eq32_e395_d_n2: f64 = (var_taun_dn2 * eq32_e394);
        let eq32_e395_d_n3: f64 = (var_taun_dn3 * eq32_e394);
        let eq32_e395_d_n4: f64 = (var_taun_dn4 * eq32_e394);
        let eq32_e395_d_n5: f64 = (var_taun_dn5 * eq32_e394);
        let eq32_e395_d_n6: f64 = (var_taun_dn6 * eq32_e394);
        let eq32_e395_d_n7: f64 = (var_taun_dn7 * eq32_e394);
        let eq32_e395_d_n8: f64 = (var_taun_dn8 * eq32_e394);
        let eq32_e395_d_n9: f64 = (var_taun_dn9 * eq32_e394);
        let eq32_e395_d_n10: f64 = (var_taun_dn10 * eq32_e394);
        let eq32_e395_d_n11: f64 = ((var_taun_dn11 * eq32_e394) + (var_taun * ddt_scale));
        let eq32_e395_d_b0: f64 = (var_taun_db0 * eq32_e394);
        let eq32_e395_d_b1: f64 = (var_taun_db1 * eq32_e394);
        let eq32_value: f64 = eq32_e395;
        let eq32_node_derivatives: [f64; 12] = [eq32_e395_d_n0, eq32_e395_d_n1, eq32_e395_d_n2, eq32_e395_d_n3, eq32_e395_d_n4, eq32_e395_d_n5, eq32_e395_d_n6, eq32_e395_d_n7, eq32_e395_d_n8, eq32_e395_d_n9, eq32_e395_d_n10, eq32_e395_d_n11];
        let eq32_branch_derivatives: [f64; 2] = [eq32_e395_d_b0, eq32_e395_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(4),
            multiplicity * (eq32_value),
            &eq32_node_derivatives,
            &eq32_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let eq14_e266: f64 = (s.v[215] + s.v[220]);
        let eq14_e266_d_n0: f64 = (s.dn[215][0] + s.dn[220][0]);
        let eq14_e266_d_n1: f64 = (s.dn[215][1] + s.dn[220][1]);
        let eq14_e266_d_n2: f64 = (s.dn[215][2] + s.dn[220][2]);
        let eq14_e266_d_n3: f64 = (s.dn[215][3] + s.dn[220][3]);
        let eq14_e266_d_n4: f64 = (s.dn[215][4] + s.dn[220][4]);
        let eq14_e266_d_n5: f64 = (s.dn[215][5] + s.dn[220][5]);
        let eq14_e266_d_n6: f64 = (s.dn[215][6] + s.dn[220][6]);
        let eq14_e266_d_n7: f64 = (s.dn[215][7] + s.dn[220][7]);
        let eq14_e266_d_n8: f64 = (s.dn[215][8] + s.dn[220][8]);
        let eq14_e266_d_n9: f64 = (s.dn[215][9] + s.dn[220][9]);
        let eq14_e266_d_n10: f64 = (s.dn[215][10] + s.dn[220][10]);
        let eq14_e266_d_n11: f64 = (s.dn[215][11] + s.dn[220][11]);
        let eq14_e266_d_b0: f64 = (s.db[215][0] + s.db[220][0]);
        let eq14_e266_d_b1: f64 = (s.db[215][1] + s.db[220][1]);
        let eq14_e268: f64 = (eq14_e266 + s.v[235]);
        let eq14_e268_d_n0: f64 = (eq14_e266_d_n0 + s.dn[235][0]);
        let eq14_e268_d_n1: f64 = (eq14_e266_d_n1 + s.dn[235][1]);
        let eq14_e268_d_n2: f64 = (eq14_e266_d_n2 + s.dn[235][2]);
        let eq14_e268_d_n3: f64 = (eq14_e266_d_n3 + s.dn[235][3]);
        let eq14_e268_d_n4: f64 = (eq14_e266_d_n4 + s.dn[235][4]);
        let eq14_e268_d_n5: f64 = (eq14_e266_d_n5 + s.dn[235][5]);
        let eq14_e268_d_n6: f64 = (eq14_e266_d_n6 + s.dn[235][6]);
        let eq14_e268_d_n7: f64 = (eq14_e266_d_n7 + s.dn[235][7]);
        let eq14_e268_d_n8: f64 = (eq14_e266_d_n8 + s.dn[235][8]);
        let eq14_e268_d_n9: f64 = (eq14_e266_d_n9 + s.dn[235][9]);
        let eq14_e268_d_n10: f64 = (eq14_e266_d_n10 + s.dn[235][10]);
        let eq14_e268_d_n11: f64 = (eq14_e266_d_n11 + s.dn[235][11]);
        let eq14_e268_d_b0: f64 = (eq14_e266_d_b0 + s.db[235][0]);
        let eq14_e268_d_b1: f64 = (eq14_e266_d_b1 + s.db[235][1]);
        let eq14_e269: f64 = (p.p3 * eq14_e268);
        let eq14_e269_d_n0: f64 = (p.p3 * eq14_e268_d_n0);
        let eq14_e269_d_n1: f64 = (p.p3 * eq14_e268_d_n1);
        let eq14_e269_d_n2: f64 = (p.p3 * eq14_e268_d_n2);
        let eq14_e269_d_n3: f64 = (p.p3 * eq14_e268_d_n3);
        let eq14_e269_d_n4: f64 = (p.p3 * eq14_e268_d_n4);
        let eq14_e269_d_n5: f64 = (p.p3 * eq14_e268_d_n5);
        let eq14_e269_d_n6: f64 = (p.p3 * eq14_e268_d_n6);
        let eq14_e269_d_n7: f64 = (p.p3 * eq14_e268_d_n7);
        let eq14_e269_d_n8: f64 = (p.p3 * eq14_e268_d_n8);
        let eq14_e269_d_n9: f64 = (p.p3 * eq14_e268_d_n9);
        let eq14_e269_d_n10: f64 = (p.p3 * eq14_e268_d_n10);
        let eq14_e269_d_n11: f64 = (p.p3 * eq14_e268_d_n11);
        let eq14_e269_d_b0: f64 = (p.p3 * eq14_e268_d_b0);
        let eq14_e269_d_b1: f64 = (p.p3 * eq14_e268_d_b1);
        let eq14_e270_q: f64 = eq14_e269;
        let eq14_e272: f64 = (eq14_e269 * p.p1);
        let eq14_e272_d_n0: f64 = (eq14_e269_d_n0 * p.p1);
        let eq14_e272_d_n1: f64 = (eq14_e269_d_n1 * p.p1);
        let eq14_e272_d_n2: f64 = (eq14_e269_d_n2 * p.p1);
        let eq14_e272_d_n3: f64 = (eq14_e269_d_n3 * p.p1);
        let eq14_e272_d_n4: f64 = (eq14_e269_d_n4 * p.p1);
        let eq14_e272_d_n5: f64 = (eq14_e269_d_n5 * p.p1);
        let eq14_e272_d_n6: f64 = (eq14_e269_d_n6 * p.p1);
        let eq14_e272_d_n7: f64 = (eq14_e269_d_n7 * p.p1);
        let eq14_e272_d_n8: f64 = (eq14_e269_d_n8 * p.p1);
        let eq14_e272_d_n9: f64 = (eq14_e269_d_n9 * p.p1);
        let eq14_e272_d_n10: f64 = (eq14_e269_d_n10 * p.p1);
        let eq14_e272_d_n11: f64 = (eq14_e269_d_n11 * p.p1);
        let eq14_e272_d_b0: f64 = (eq14_e269_d_b0 * p.p1);
        let eq14_e272_d_b1: f64 = (eq14_e269_d_b1 * p.p1);
        let eq14_e272_q: f64 = (eq14_e270_q * p.p1);
        let eq14_reactive_node_derivatives: [f64; 12] = [eq14_e272_d_n0, eq14_e272_d_n1, eq14_e272_d_n2, eq14_e272_d_n3, eq14_e272_d_n4, eq14_e272_d_n5, eq14_e272_d_n6, eq14_e272_d_n7, eq14_e272_d_n8, eq14_e272_d_n9, eq14_e272_d_n10, eq14_e272_d_n11];
        let eq14_reactive_branch_derivatives: [f64; 2] = [eq14_e272_d_b0, eq14_e272_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            nodes,
            &eq14_reactive_node_derivatives,
            branches,
            &eq14_reactive_branch_derivatives,
            multiplicity,
        );
        let eq15_e275: f64 = (p.p3 * s.v[217]);
        let eq15_e275_d_n0: f64 = (p.p3 * s.dn[217][0]);
        let eq15_e275_d_n1: f64 = (p.p3 * s.dn[217][1]);
        let eq15_e275_d_n2: f64 = (p.p3 * s.dn[217][2]);
        let eq15_e275_d_n3: f64 = (p.p3 * s.dn[217][3]);
        let eq15_e275_d_n4: f64 = (p.p3 * s.dn[217][4]);
        let eq15_e275_d_n5: f64 = (p.p3 * s.dn[217][5]);
        let eq15_e275_d_n6: f64 = (p.p3 * s.dn[217][6]);
        let eq15_e275_d_n7: f64 = (p.p3 * s.dn[217][7]);
        let eq15_e275_d_n8: f64 = (p.p3 * s.dn[217][8]);
        let eq15_e275_d_n9: f64 = (p.p3 * s.dn[217][9]);
        let eq15_e275_d_n10: f64 = (p.p3 * s.dn[217][10]);
        let eq15_e275_d_n11: f64 = (p.p3 * s.dn[217][11]);
        let eq15_e275_d_b0: f64 = (p.p3 * s.db[217][0]);
        let eq15_e275_d_b1: f64 = (p.p3 * s.db[217][1]);
        let eq15_e276_q: f64 = eq15_e275;
        let eq15_e278: f64 = (eq15_e275 * p.p1);
        let eq15_e278_d_n0: f64 = (eq15_e275_d_n0 * p.p1);
        let eq15_e278_d_n1: f64 = (eq15_e275_d_n1 * p.p1);
        let eq15_e278_d_n2: f64 = (eq15_e275_d_n2 * p.p1);
        let eq15_e278_d_n3: f64 = (eq15_e275_d_n3 * p.p1);
        let eq15_e278_d_n4: f64 = (eq15_e275_d_n4 * p.p1);
        let eq15_e278_d_n5: f64 = (eq15_e275_d_n5 * p.p1);
        let eq15_e278_d_n6: f64 = (eq15_e275_d_n6 * p.p1);
        let eq15_e278_d_n7: f64 = (eq15_e275_d_n7 * p.p1);
        let eq15_e278_d_n8: f64 = (eq15_e275_d_n8 * p.p1);
        let eq15_e278_d_n9: f64 = (eq15_e275_d_n9 * p.p1);
        let eq15_e278_d_n10: f64 = (eq15_e275_d_n10 * p.p1);
        let eq15_e278_d_n11: f64 = (eq15_e275_d_n11 * p.p1);
        let eq15_e278_d_b0: f64 = (eq15_e275_d_b0 * p.p1);
        let eq15_e278_d_b1: f64 = (eq15_e275_d_b1 * p.p1);
        let eq15_e278_q: f64 = (eq15_e276_q * p.p1);
        let eq15_reactive_node_derivatives: [f64; 12] = [eq15_e278_d_n0, eq15_e278_d_n1, eq15_e278_d_n2, eq15_e278_d_n3, eq15_e278_d_n4, eq15_e278_d_n5, eq15_e278_d_n6, eq15_e278_d_n7, eq15_e278_d_n8, eq15_e278_d_n9, eq15_e278_d_n10, eq15_e278_d_n11];
        let eq15_reactive_branch_derivatives: [f64; 2] = [eq15_e278_d_b0, eq15_e278_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let eq16_e282: f64 = (s.v[218] + s.v[221]);
        let eq16_e282_d_n0: f64 = (s.dn[218][0] + s.dn[221][0]);
        let eq16_e282_d_n1: f64 = (s.dn[218][1] + s.dn[221][1]);
        let eq16_e282_d_n2: f64 = (s.dn[218][2] + s.dn[221][2]);
        let eq16_e282_d_n3: f64 = (s.dn[218][3] + s.dn[221][3]);
        let eq16_e282_d_n4: f64 = (s.dn[218][4] + s.dn[221][4]);
        let eq16_e282_d_n5: f64 = (s.dn[218][5] + s.dn[221][5]);
        let eq16_e282_d_n6: f64 = (s.dn[218][6] + s.dn[221][6]);
        let eq16_e282_d_n7: f64 = (s.dn[218][7] + s.dn[221][7]);
        let eq16_e282_d_n8: f64 = (s.dn[218][8] + s.dn[221][8]);
        let eq16_e282_d_n9: f64 = (s.dn[218][9] + s.dn[221][9]);
        let eq16_e282_d_n10: f64 = (s.dn[218][10] + s.dn[221][10]);
        let eq16_e282_d_n11: f64 = (s.dn[218][11] + s.dn[221][11]);
        let eq16_e282_d_b0: f64 = (s.db[218][0] + s.db[221][0]);
        let eq16_e282_d_b1: f64 = (s.db[218][1] + s.db[221][1]);
        let eq16_e284: f64 = (eq16_e282 + s.v[238]);
        let eq16_e284_d_n0: f64 = (eq16_e282_d_n0 + s.dn[238][0]);
        let eq16_e284_d_n1: f64 = (eq16_e282_d_n1 + s.dn[238][1]);
        let eq16_e284_d_n2: f64 = (eq16_e282_d_n2 + s.dn[238][2]);
        let eq16_e284_d_n3: f64 = (eq16_e282_d_n3 + s.dn[238][3]);
        let eq16_e284_d_n4: f64 = (eq16_e282_d_n4 + s.dn[238][4]);
        let eq16_e284_d_n5: f64 = (eq16_e282_d_n5 + s.dn[238][5]);
        let eq16_e284_d_n6: f64 = (eq16_e282_d_n6 + s.dn[238][6]);
        let eq16_e284_d_n7: f64 = (eq16_e282_d_n7 + s.dn[238][7]);
        let eq16_e284_d_n8: f64 = (eq16_e282_d_n8 + s.dn[238][8]);
        let eq16_e284_d_n9: f64 = (eq16_e282_d_n9 + s.dn[238][9]);
        let eq16_e284_d_n10: f64 = (eq16_e282_d_n10 + s.dn[238][10]);
        let eq16_e284_d_n11: f64 = (eq16_e282_d_n11 + s.dn[238][11]);
        let eq16_e284_d_b0: f64 = (eq16_e282_d_b0 + s.db[238][0]);
        let eq16_e284_d_b1: f64 = (eq16_e282_d_b1 + s.db[238][1]);
        let eq16_e285: f64 = (p.p3 * eq16_e284);
        let eq16_e285_d_n0: f64 = (p.p3 * eq16_e284_d_n0);
        let eq16_e285_d_n1: f64 = (p.p3 * eq16_e284_d_n1);
        let eq16_e285_d_n2: f64 = (p.p3 * eq16_e284_d_n2);
        let eq16_e285_d_n3: f64 = (p.p3 * eq16_e284_d_n3);
        let eq16_e285_d_n4: f64 = (p.p3 * eq16_e284_d_n4);
        let eq16_e285_d_n5: f64 = (p.p3 * eq16_e284_d_n5);
        let eq16_e285_d_n6: f64 = (p.p3 * eq16_e284_d_n6);
        let eq16_e285_d_n7: f64 = (p.p3 * eq16_e284_d_n7);
        let eq16_e285_d_n8: f64 = (p.p3 * eq16_e284_d_n8);
        let eq16_e285_d_n9: f64 = (p.p3 * eq16_e284_d_n9);
        let eq16_e285_d_n10: f64 = (p.p3 * eq16_e284_d_n10);
        let eq16_e285_d_n11: f64 = (p.p3 * eq16_e284_d_n11);
        let eq16_e285_d_b0: f64 = (p.p3 * eq16_e284_d_b0);
        let eq16_e285_d_b1: f64 = (p.p3 * eq16_e284_d_b1);
        let eq16_e286_q: f64 = eq16_e285;
        let eq16_e288: f64 = (eq16_e285 * p.p1);
        let eq16_e288_d_n0: f64 = (eq16_e285_d_n0 * p.p1);
        let eq16_e288_d_n1: f64 = (eq16_e285_d_n1 * p.p1);
        let eq16_e288_d_n2: f64 = (eq16_e285_d_n2 * p.p1);
        let eq16_e288_d_n3: f64 = (eq16_e285_d_n3 * p.p1);
        let eq16_e288_d_n4: f64 = (eq16_e285_d_n4 * p.p1);
        let eq16_e288_d_n5: f64 = (eq16_e285_d_n5 * p.p1);
        let eq16_e288_d_n6: f64 = (eq16_e285_d_n6 * p.p1);
        let eq16_e288_d_n7: f64 = (eq16_e285_d_n7 * p.p1);
        let eq16_e288_d_n8: f64 = (eq16_e285_d_n8 * p.p1);
        let eq16_e288_d_n9: f64 = (eq16_e285_d_n9 * p.p1);
        let eq16_e288_d_n10: f64 = (eq16_e285_d_n10 * p.p1);
        let eq16_e288_d_n11: f64 = (eq16_e285_d_n11 * p.p1);
        let eq16_e288_d_b0: f64 = (eq16_e285_d_b0 * p.p1);
        let eq16_e288_d_b1: f64 = (eq16_e285_d_b1 * p.p1);
        let eq16_e288_q: f64 = (eq16_e286_q * p.p1);
        let eq16_reactive_node_derivatives: [f64; 12] = [eq16_e288_d_n0, eq16_e288_d_n1, eq16_e288_d_n2, eq16_e288_d_n3, eq16_e288_d_n4, eq16_e288_d_n5, eq16_e288_d_n6, eq16_e288_d_n7, eq16_e288_d_n8, eq16_e288_d_n9, eq16_e288_d_n10, eq16_e288_d_n11];
        let eq16_reactive_branch_derivatives: [f64; 2] = [eq16_e288_d_b0, eq16_e288_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            nodes,
            &eq16_reactive_node_derivatives,
            branches,
            &eq16_reactive_branch_derivatives,
            multiplicity,
        );
        let eq17_e291: f64 = (p.p3 * s.v[233]);
        let eq17_e291_d_n0: f64 = (p.p3 * s.dn[233][0]);
        let eq17_e291_d_n1: f64 = (p.p3 * s.dn[233][1]);
        let eq17_e291_d_n2: f64 = (p.p3 * s.dn[233][2]);
        let eq17_e291_d_n3: f64 = (p.p3 * s.dn[233][3]);
        let eq17_e291_d_n4: f64 = (p.p3 * s.dn[233][4]);
        let eq17_e291_d_n5: f64 = (p.p3 * s.dn[233][5]);
        let eq17_e291_d_n6: f64 = (p.p3 * s.dn[233][6]);
        let eq17_e291_d_n7: f64 = (p.p3 * s.dn[233][7]);
        let eq17_e291_d_n8: f64 = (p.p3 * s.dn[233][8]);
        let eq17_e291_d_n9: f64 = (p.p3 * s.dn[233][9]);
        let eq17_e291_d_n10: f64 = (p.p3 * s.dn[233][10]);
        let eq17_e291_d_n11: f64 = (p.p3 * s.dn[233][11]);
        let eq17_e291_d_b0: f64 = (p.p3 * s.db[233][0]);
        let eq17_e291_d_b1: f64 = (p.p3 * s.db[233][1]);
        let eq17_e292_q: f64 = eq17_e291;
        let eq17_e294: f64 = (eq17_e291 * p.p1);
        let eq17_e294_d_n0: f64 = (eq17_e291_d_n0 * p.p1);
        let eq17_e294_d_n1: f64 = (eq17_e291_d_n1 * p.p1);
        let eq17_e294_d_n2: f64 = (eq17_e291_d_n2 * p.p1);
        let eq17_e294_d_n3: f64 = (eq17_e291_d_n3 * p.p1);
        let eq17_e294_d_n4: f64 = (eq17_e291_d_n4 * p.p1);
        let eq17_e294_d_n5: f64 = (eq17_e291_d_n5 * p.p1);
        let eq17_e294_d_n6: f64 = (eq17_e291_d_n6 * p.p1);
        let eq17_e294_d_n7: f64 = (eq17_e291_d_n7 * p.p1);
        let eq17_e294_d_n8: f64 = (eq17_e291_d_n8 * p.p1);
        let eq17_e294_d_n9: f64 = (eq17_e291_d_n9 * p.p1);
        let eq17_e294_d_n10: f64 = (eq17_e291_d_n10 * p.p1);
        let eq17_e294_d_n11: f64 = (eq17_e291_d_n11 * p.p1);
        let eq17_e294_d_b0: f64 = (eq17_e291_d_b0 * p.p1);
        let eq17_e294_d_b1: f64 = (eq17_e291_d_b1 * p.p1);
        let eq17_e294_q: f64 = (eq17_e292_q * p.p1);
        let eq17_reactive_node_derivatives: [f64; 12] = [eq17_e294_d_n0, eq17_e294_d_n1, eq17_e294_d_n2, eq17_e294_d_n3, eq17_e294_d_n4, eq17_e294_d_n5, eq17_e294_d_n6, eq17_e294_d_n7, eq17_e294_d_n8, eq17_e294_d_n9, eq17_e294_d_n10, eq17_e294_d_n11];
        let eq17_reactive_branch_derivatives: [f64; 2] = [eq17_e294_d_b0, eq17_e294_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            nodes,
            &eq17_reactive_node_derivatives,
            branches,
            &eq17_reactive_branch_derivatives,
            multiplicity,
        );
        let eq18_e297: f64 = (p.p3 * s.v[222]);
        let eq18_e297_d_n0: f64 = (p.p3 * s.dn[222][0]);
        let eq18_e297_d_n1: f64 = (p.p3 * s.dn[222][1]);
        let eq18_e297_d_n2: f64 = (p.p3 * s.dn[222][2]);
        let eq18_e297_d_n3: f64 = (p.p3 * s.dn[222][3]);
        let eq18_e297_d_n4: f64 = (p.p3 * s.dn[222][4]);
        let eq18_e297_d_n5: f64 = (p.p3 * s.dn[222][5]);
        let eq18_e297_d_n6: f64 = (p.p3 * s.dn[222][6]);
        let eq18_e297_d_n7: f64 = (p.p3 * s.dn[222][7]);
        let eq18_e297_d_n8: f64 = (p.p3 * s.dn[222][8]);
        let eq18_e297_d_n9: f64 = (p.p3 * s.dn[222][9]);
        let eq18_e297_d_n10: f64 = (p.p3 * s.dn[222][10]);
        let eq18_e297_d_n11: f64 = (p.p3 * s.dn[222][11]);
        let eq18_e297_d_b0: f64 = (p.p3 * s.db[222][0]);
        let eq18_e297_d_b1: f64 = (p.p3 * s.db[222][1]);
        let eq18_e298_q: f64 = eq18_e297;
        let eq18_e300: f64 = (eq18_e297 * p.p1);
        let eq18_e300_d_n0: f64 = (eq18_e297_d_n0 * p.p1);
        let eq18_e300_d_n1: f64 = (eq18_e297_d_n1 * p.p1);
        let eq18_e300_d_n2: f64 = (eq18_e297_d_n2 * p.p1);
        let eq18_e300_d_n3: f64 = (eq18_e297_d_n3 * p.p1);
        let eq18_e300_d_n4: f64 = (eq18_e297_d_n4 * p.p1);
        let eq18_e300_d_n5: f64 = (eq18_e297_d_n5 * p.p1);
        let eq18_e300_d_n6: f64 = (eq18_e297_d_n6 * p.p1);
        let eq18_e300_d_n7: f64 = (eq18_e297_d_n7 * p.p1);
        let eq18_e300_d_n8: f64 = (eq18_e297_d_n8 * p.p1);
        let eq18_e300_d_n9: f64 = (eq18_e297_d_n9 * p.p1);
        let eq18_e300_d_n10: f64 = (eq18_e297_d_n10 * p.p1);
        let eq18_e300_d_n11: f64 = (eq18_e297_d_n11 * p.p1);
        let eq18_e300_d_b0: f64 = (eq18_e297_d_b0 * p.p1);
        let eq18_e300_d_b1: f64 = (eq18_e297_d_b1 * p.p1);
        let eq18_e300_q: f64 = (eq18_e298_q * p.p1);
        let eq18_reactive_node_derivatives: [f64; 12] = [eq18_e300_d_n0, eq18_e300_d_n1, eq18_e300_d_n2, eq18_e300_d_n3, eq18_e300_d_n4, eq18_e300_d_n5, eq18_e300_d_n6, eq18_e300_d_n7, eq18_e300_d_n8, eq18_e300_d_n9, eq18_e300_d_n10, eq18_e300_d_n11];
        let eq18_reactive_branch_derivatives: [f64; 2] = [eq18_e300_d_b0, eq18_e300_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let eq19_e303: f64 = (p.p3 * p.p69);
        let eq19_e305: f64 = (eq19_e303 * s.v[263]);
        let eq19_e305_d_n0: f64 = (eq19_e303 * s.dn[263][0]);
        let eq19_e305_d_n1: f64 = (eq19_e303 * s.dn[263][1]);
        let eq19_e305_d_n2: f64 = (eq19_e303 * s.dn[263][2]);
        let eq19_e305_d_n3: f64 = (eq19_e303 * s.dn[263][3]);
        let eq19_e305_d_n4: f64 = (eq19_e303 * s.dn[263][4]);
        let eq19_e305_d_n5: f64 = (eq19_e303 * s.dn[263][5]);
        let eq19_e305_d_n6: f64 = (eq19_e303 * s.dn[263][6]);
        let eq19_e305_d_n7: f64 = (eq19_e303 * s.dn[263][7]);
        let eq19_e305_d_n8: f64 = (eq19_e303 * s.dn[263][8]);
        let eq19_e305_d_n9: f64 = (eq19_e303 * s.dn[263][9]);
        let eq19_e305_d_n10: f64 = (eq19_e303 * s.dn[263][10]);
        let eq19_e305_d_n11: f64 = (eq19_e303 * s.dn[263][11]);
        let eq19_e305_d_b0: f64 = (eq19_e303 * s.db[263][0]);
        let eq19_e305_d_b1: f64 = (eq19_e303 * s.db[263][1]);
        let eq19_e306_q: f64 = eq19_e305;
        let eq19_e308: f64 = (eq19_e305 * p.p1);
        let eq19_e308_d_n0: f64 = (eq19_e305_d_n0 * p.p1);
        let eq19_e308_d_n1: f64 = (eq19_e305_d_n1 * p.p1);
        let eq19_e308_d_n2: f64 = (eq19_e305_d_n2 * p.p1);
        let eq19_e308_d_n3: f64 = (eq19_e305_d_n3 * p.p1);
        let eq19_e308_d_n4: f64 = (eq19_e305_d_n4 * p.p1);
        let eq19_e308_d_n5: f64 = (eq19_e305_d_n5 * p.p1);
        let eq19_e308_d_n6: f64 = (eq19_e305_d_n6 * p.p1);
        let eq19_e308_d_n7: f64 = (eq19_e305_d_n7 * p.p1);
        let eq19_e308_d_n8: f64 = (eq19_e305_d_n8 * p.p1);
        let eq19_e308_d_n9: f64 = (eq19_e305_d_n9 * p.p1);
        let eq19_e308_d_n10: f64 = (eq19_e305_d_n10 * p.p1);
        let eq19_e308_d_n11: f64 = (eq19_e305_d_n11 * p.p1);
        let eq19_e308_d_b0: f64 = (eq19_e305_d_b0 * p.p1);
        let eq19_e308_d_b1: f64 = (eq19_e305_d_b1 * p.p1);
        let eq19_e308_q: f64 = (eq19_e306_q * p.p1);
        let eq19_reactive_node_derivatives: [f64; 12] = [eq19_e308_d_n0, eq19_e308_d_n1, eq19_e308_d_n2, eq19_e308_d_n3, eq19_e308_d_n4, eq19_e308_d_n5, eq19_e308_d_n6, eq19_e308_d_n7, eq19_e308_d_n8, eq19_e308_d_n9, eq19_e308_d_n10, eq19_e308_d_n11];
        let eq19_reactive_branch_derivatives: [f64; 2] = [eq19_e308_d_b0, eq19_e308_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let eq20_e311: f64 = (p.p3 * p.p78);
        let eq20_e313: f64 = (eq20_e311 * s.v[264]);
        let eq20_e313_d_n0: f64 = (eq20_e311 * s.dn[264][0]);
        let eq20_e313_d_n1: f64 = (eq20_e311 * s.dn[264][1]);
        let eq20_e313_d_n2: f64 = (eq20_e311 * s.dn[264][2]);
        let eq20_e313_d_n3: f64 = (eq20_e311 * s.dn[264][3]);
        let eq20_e313_d_n4: f64 = (eq20_e311 * s.dn[264][4]);
        let eq20_e313_d_n5: f64 = (eq20_e311 * s.dn[264][5]);
        let eq20_e313_d_n6: f64 = (eq20_e311 * s.dn[264][6]);
        let eq20_e313_d_n7: f64 = (eq20_e311 * s.dn[264][7]);
        let eq20_e313_d_n8: f64 = (eq20_e311 * s.dn[264][8]);
        let eq20_e313_d_n9: f64 = (eq20_e311 * s.dn[264][9]);
        let eq20_e313_d_n10: f64 = (eq20_e311 * s.dn[264][10]);
        let eq20_e313_d_n11: f64 = (eq20_e311 * s.dn[264][11]);
        let eq20_e313_d_b0: f64 = (eq20_e311 * s.db[264][0]);
        let eq20_e313_d_b1: f64 = (eq20_e311 * s.db[264][1]);
        let eq20_e314_q: f64 = eq20_e313;
        let eq20_e316: f64 = (eq20_e313 * p.p1);
        let eq20_e316_d_n0: f64 = (eq20_e313_d_n0 * p.p1);
        let eq20_e316_d_n1: f64 = (eq20_e313_d_n1 * p.p1);
        let eq20_e316_d_n2: f64 = (eq20_e313_d_n2 * p.p1);
        let eq20_e316_d_n3: f64 = (eq20_e313_d_n3 * p.p1);
        let eq20_e316_d_n4: f64 = (eq20_e313_d_n4 * p.p1);
        let eq20_e316_d_n5: f64 = (eq20_e313_d_n5 * p.p1);
        let eq20_e316_d_n6: f64 = (eq20_e313_d_n6 * p.p1);
        let eq20_e316_d_n7: f64 = (eq20_e313_d_n7 * p.p1);
        let eq20_e316_d_n8: f64 = (eq20_e313_d_n8 * p.p1);
        let eq20_e316_d_n9: f64 = (eq20_e313_d_n9 * p.p1);
        let eq20_e316_d_n10: f64 = (eq20_e313_d_n10 * p.p1);
        let eq20_e316_d_n11: f64 = (eq20_e313_d_n11 * p.p1);
        let eq20_e316_d_b0: f64 = (eq20_e313_d_b0 * p.p1);
        let eq20_e316_d_b1: f64 = (eq20_e313_d_b1 * p.p1);
        let eq20_e316_q: f64 = (eq20_e314_q * p.p1);
        let eq20_reactive_node_derivatives: [f64; 12] = [eq20_e316_d_n0, eq20_e316_d_n1, eq20_e316_d_n2, eq20_e316_d_n3, eq20_e316_d_n4, eq20_e316_d_n5, eq20_e316_d_n6, eq20_e316_d_n7, eq20_e316_d_n8, eq20_e316_d_n9, eq20_e316_d_n10, eq20_e316_d_n11];
        let eq20_reactive_branch_derivatives: [f64; 2] = [eq20_e316_d_b0, eq20_e316_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes,
            &eq20_reactive_node_derivatives,
            branches,
            &eq20_reactive_branch_derivatives,
            multiplicity,
        );
        let eq23_e332: f64 = (s.v[230] + s.v[242]);
        let eq23_e332_d_n0: f64 = (s.dn[230][0] + s.dn[242][0]);
        let eq23_e332_d_n1: f64 = (s.dn[230][1] + s.dn[242][1]);
        let eq23_e332_d_n2: f64 = (s.dn[230][2] + s.dn[242][2]);
        let eq23_e332_d_n3: f64 = (s.dn[230][3] + s.dn[242][3]);
        let eq23_e332_d_n4: f64 = (s.dn[230][4] + s.dn[242][4]);
        let eq23_e332_d_n5: f64 = (s.dn[230][5] + s.dn[242][5]);
        let eq23_e332_d_n6: f64 = (s.dn[230][6] + s.dn[242][6]);
        let eq23_e332_d_n7: f64 = (s.dn[230][7] + s.dn[242][7]);
        let eq23_e332_d_n8: f64 = (s.dn[230][8] + s.dn[242][8]);
        let eq23_e332_d_n9: f64 = (s.dn[230][9] + s.dn[242][9]);
        let eq23_e332_d_n10: f64 = (s.dn[230][10] + s.dn[242][10]);
        let eq23_e332_d_n11: f64 = (s.dn[230][11] + s.dn[242][11]);
        let eq23_e332_d_b0: f64 = (s.db[230][0] + s.db[242][0]);
        let eq23_e332_d_b1: f64 = (s.db[230][1] + s.db[242][1]);
        let eq23_e333: f64 = (p.p3 * eq23_e332);
        let eq23_e333_d_n0: f64 = (p.p3 * eq23_e332_d_n0);
        let eq23_e333_d_n1: f64 = (p.p3 * eq23_e332_d_n1);
        let eq23_e333_d_n2: f64 = (p.p3 * eq23_e332_d_n2);
        let eq23_e333_d_n3: f64 = (p.p3 * eq23_e332_d_n3);
        let eq23_e333_d_n4: f64 = (p.p3 * eq23_e332_d_n4);
        let eq23_e333_d_n5: f64 = (p.p3 * eq23_e332_d_n5);
        let eq23_e333_d_n6: f64 = (p.p3 * eq23_e332_d_n6);
        let eq23_e333_d_n7: f64 = (p.p3 * eq23_e332_d_n7);
        let eq23_e333_d_n8: f64 = (p.p3 * eq23_e332_d_n8);
        let eq23_e333_d_n9: f64 = (p.p3 * eq23_e332_d_n9);
        let eq23_e333_d_n10: f64 = (p.p3 * eq23_e332_d_n10);
        let eq23_e333_d_n11: f64 = (p.p3 * eq23_e332_d_n11);
        let eq23_e333_d_b0: f64 = (p.p3 * eq23_e332_d_b0);
        let eq23_e333_d_b1: f64 = (p.p3 * eq23_e332_d_b1);
        let eq23_e334_q: f64 = eq23_e333;
        let eq23_e336: f64 = (eq23_e333 * p.p1);
        let eq23_e336_d_n0: f64 = (eq23_e333_d_n0 * p.p1);
        let eq23_e336_d_n1: f64 = (eq23_e333_d_n1 * p.p1);
        let eq23_e336_d_n2: f64 = (eq23_e333_d_n2 * p.p1);
        let eq23_e336_d_n3: f64 = (eq23_e333_d_n3 * p.p1);
        let eq23_e336_d_n4: f64 = (eq23_e333_d_n4 * p.p1);
        let eq23_e336_d_n5: f64 = (eq23_e333_d_n5 * p.p1);
        let eq23_e336_d_n6: f64 = (eq23_e333_d_n6 * p.p1);
        let eq23_e336_d_n7: f64 = (eq23_e333_d_n7 * p.p1);
        let eq23_e336_d_n8: f64 = (eq23_e333_d_n8 * p.p1);
        let eq23_e336_d_n9: f64 = (eq23_e333_d_n9 * p.p1);
        let eq23_e336_d_n10: f64 = (eq23_e333_d_n10 * p.p1);
        let eq23_e336_d_n11: f64 = (eq23_e333_d_n11 * p.p1);
        let eq23_e336_d_b0: f64 = (eq23_e333_d_b0 * p.p1);
        let eq23_e336_d_b1: f64 = (eq23_e333_d_b1 * p.p1);
        let eq23_e336_q: f64 = (eq23_e334_q * p.p1);
        let eq23_reactive_node_derivatives: [f64; 12] = [eq23_e336_d_n0, eq23_e336_d_n1, eq23_e336_d_n2, eq23_e336_d_n3, eq23_e336_d_n4, eq23_e336_d_n5, eq23_e336_d_n6, eq23_e336_d_n7, eq23_e336_d_n8, eq23_e336_d_n9, eq23_e336_d_n10, eq23_e336_d_n11];
        let eq23_reactive_branch_derivatives: [f64; 2] = [eq23_e336_d_b0, eq23_e336_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[9]),
            nodes,
            &eq23_reactive_node_derivatives,
            branches,
            &eq23_reactive_branch_derivatives,
            multiplicity,
        );
        let eq25_e351: f64 = (s.v[227] + s.v[243]);
        let eq25_e351_d_n0: f64 = (s.dn[227][0] + s.dn[243][0]);
        let eq25_e351_d_n1: f64 = (s.dn[227][1] + s.dn[243][1]);
        let eq25_e351_d_n2: f64 = (s.dn[227][2] + s.dn[243][2]);
        let eq25_e351_d_n3: f64 = (s.dn[227][3] + s.dn[243][3]);
        let eq25_e351_d_n4: f64 = (s.dn[227][4] + s.dn[243][4]);
        let eq25_e351_d_n5: f64 = (s.dn[227][5] + s.dn[243][5]);
        let eq25_e351_d_n6: f64 = (s.dn[227][6] + s.dn[243][6]);
        let eq25_e351_d_n7: f64 = (s.dn[227][7] + s.dn[243][7]);
        let eq25_e351_d_n8: f64 = (s.dn[227][8] + s.dn[243][8]);
        let eq25_e351_d_n9: f64 = (s.dn[227][9] + s.dn[243][9]);
        let eq25_e351_d_n10: f64 = (s.dn[227][10] + s.dn[243][10]);
        let eq25_e351_d_n11: f64 = (s.dn[227][11] + s.dn[243][11]);
        let eq25_e351_d_b0: f64 = (s.db[227][0] + s.db[243][0]);
        let eq25_e351_d_b1: f64 = (s.db[227][1] + s.db[243][1]);
        let eq25_e352: f64 = (p.p3 * eq25_e351);
        let eq25_e352_d_n0: f64 = (p.p3 * eq25_e351_d_n0);
        let eq25_e352_d_n1: f64 = (p.p3 * eq25_e351_d_n1);
        let eq25_e352_d_n2: f64 = (p.p3 * eq25_e351_d_n2);
        let eq25_e352_d_n3: f64 = (p.p3 * eq25_e351_d_n3);
        let eq25_e352_d_n4: f64 = (p.p3 * eq25_e351_d_n4);
        let eq25_e352_d_n5: f64 = (p.p3 * eq25_e351_d_n5);
        let eq25_e352_d_n6: f64 = (p.p3 * eq25_e351_d_n6);
        let eq25_e352_d_n7: f64 = (p.p3 * eq25_e351_d_n7);
        let eq25_e352_d_n8: f64 = (p.p3 * eq25_e351_d_n8);
        let eq25_e352_d_n9: f64 = (p.p3 * eq25_e351_d_n9);
        let eq25_e352_d_n10: f64 = (p.p3 * eq25_e351_d_n10);
        let eq25_e352_d_n11: f64 = (p.p3 * eq25_e351_d_n11);
        let eq25_e352_d_b0: f64 = (p.p3 * eq25_e351_d_b0);
        let eq25_e352_d_b1: f64 = (p.p3 * eq25_e351_d_b1);
        let eq25_e353_q: f64 = eq25_e352;
        let eq25_e355: f64 = (eq25_e352 * p.p1);
        let eq25_e355_d_n0: f64 = (eq25_e352_d_n0 * p.p1);
        let eq25_e355_d_n1: f64 = (eq25_e352_d_n1 * p.p1);
        let eq25_e355_d_n2: f64 = (eq25_e352_d_n2 * p.p1);
        let eq25_e355_d_n3: f64 = (eq25_e352_d_n3 * p.p1);
        let eq25_e355_d_n4: f64 = (eq25_e352_d_n4 * p.p1);
        let eq25_e355_d_n5: f64 = (eq25_e352_d_n5 * p.p1);
        let eq25_e355_d_n6: f64 = (eq25_e352_d_n6 * p.p1);
        let eq25_e355_d_n7: f64 = (eq25_e352_d_n7 * p.p1);
        let eq25_e355_d_n8: f64 = (eq25_e352_d_n8 * p.p1);
        let eq25_e355_d_n9: f64 = (eq25_e352_d_n9 * p.p1);
        let eq25_e355_d_n10: f64 = (eq25_e352_d_n10 * p.p1);
        let eq25_e355_d_n11: f64 = (eq25_e352_d_n11 * p.p1);
        let eq25_e355_d_b0: f64 = (eq25_e352_d_b0 * p.p1);
        let eq25_e355_d_b1: f64 = (eq25_e352_d_b1 * p.p1);
        let eq25_e355_q: f64 = (eq25_e353_q * p.p1);
        let eq25_reactive_node_derivatives: [f64; 12] = [eq25_e355_d_n0, eq25_e355_d_n1, eq25_e355_d_n2, eq25_e355_d_n3, eq25_e355_d_n4, eq25_e355_d_n5, eq25_e355_d_n6, eq25_e355_d_n7, eq25_e355_d_n8, eq25_e355_d_n9, eq25_e355_d_n10, eq25_e355_d_n11];
        let eq25_reactive_branch_derivatives: [f64; 2] = [eq25_e355_d_b0, eq25_e355_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[10]),
            nodes,
            &eq25_reactive_node_derivatives,
            branches,
            &eq25_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv11 = ctx.node_voltage(nodes[11]);
        let eq32_e394_q: f64 = (nv11 - 0.0);
        let eq32_e395: f64 = (s.v[330] * (nv11 - 0.0));
        let eq32_e395_d_n0: f64 = (s.dn[330][0] * (nv11 - 0.0));
        let eq32_e395_d_n1: f64 = (s.dn[330][1] * (nv11 - 0.0));
        let eq32_e395_d_n2: f64 = (s.dn[330][2] * (nv11 - 0.0));
        let eq32_e395_d_n3: f64 = (s.dn[330][3] * (nv11 - 0.0));
        let eq32_e395_d_n4: f64 = (s.dn[330][4] * (nv11 - 0.0));
        let eq32_e395_d_n5: f64 = (s.dn[330][5] * (nv11 - 0.0));
        let eq32_e395_d_n6: f64 = (s.dn[330][6] * (nv11 - 0.0));
        let eq32_e395_d_n7: f64 = (s.dn[330][7] * (nv11 - 0.0));
        let eq32_e395_d_n8: f64 = (s.dn[330][8] * (nv11 - 0.0));
        let eq32_e395_d_n9: f64 = (s.dn[330][9] * (nv11 - 0.0));
        let eq32_e395_d_n10: f64 = (s.dn[330][10] * (nv11 - 0.0));
        let eq32_e395_d_n11: f64 = ((s.dn[330][11] * (nv11 - 0.0)) + s.v[330]);
        let eq32_e395_d_b0: f64 = (s.db[330][0] * (nv11 - 0.0));
        let eq32_e395_d_b1: f64 = (s.db[330][1] * (nv11 - 0.0));
        let eq32_e395_q: f64 = (s.v[330] * eq32_e394_q);
        let eq32_e395_q_d_n0: f64 = (s.dn[330][0] * eq32_e394_q);
        let eq32_e395_q_d_n1: f64 = (s.dn[330][1] * eq32_e394_q);
        let eq32_e395_q_d_n2: f64 = (s.dn[330][2] * eq32_e394_q);
        let eq32_e395_q_d_n3: f64 = (s.dn[330][3] * eq32_e394_q);
        let eq32_e395_q_d_n4: f64 = (s.dn[330][4] * eq32_e394_q);
        let eq32_e395_q_d_n5: f64 = (s.dn[330][5] * eq32_e394_q);
        let eq32_e395_q_d_n6: f64 = (s.dn[330][6] * eq32_e394_q);
        let eq32_e395_q_d_n7: f64 = (s.dn[330][7] * eq32_e394_q);
        let eq32_e395_q_d_n8: f64 = (s.dn[330][8] * eq32_e394_q);
        let eq32_e395_q_d_n9: f64 = (s.dn[330][9] * eq32_e394_q);
        let eq32_e395_q_d_n10: f64 = (s.dn[330][10] * eq32_e394_q);
        let eq32_e395_q_d_n11: f64 = ((s.dn[330][11] * eq32_e394_q) + s.v[330]);
        let eq32_e395_q_d_b0: f64 = (s.db[330][0] * eq32_e394_q);
        let eq32_e395_q_d_b1: f64 = (s.db[330][1] * eq32_e394_q);
        let eq32_reactive_node_derivatives: [f64; 12] = [eq32_e395_q_d_n0, eq32_e395_q_d_n1, eq32_e395_q_d_n2, eq32_e395_q_d_n3, eq32_e395_q_d_n4, eq32_e395_q_d_n5, eq32_e395_q_d_n6, eq32_e395_q_d_n7, eq32_e395_q_d_n8, eq32_e395_q_d_n9, eq32_e395_q_d_n10, eq32_e395_q_d_n11];
        let eq32_reactive_branch_derivatives: [f64; 2] = [eq32_e395_q_d_b0, eq32_e395_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            nodes,
            &eq32_reactive_node_derivatives,
            branches,
            &eq32_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
