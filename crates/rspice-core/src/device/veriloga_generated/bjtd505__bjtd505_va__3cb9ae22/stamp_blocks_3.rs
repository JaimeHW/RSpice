#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_33(
        p: &Parameters,
        var_bjc: f64,
        var_bjc_db0: f64,
        var_bjc_db1: f64,
        var_bjc_dn0: f64,
        var_bjc_dn1: f64,
        var_bjc_dn10: f64,
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
        var_cjc_t_dn2: f64,
        var_cjc_t_dn3: f64,
        var_cjc_t_dn4: f64,
        var_cjc_t_dn5: f64,
        var_cjc_t_dn6: f64,
        var_cjc_t_dn7: f64,
        var_cjc_t_dn8: f64,
        var_cjc_t_dn9: f64,
        var_cje_t: f64,
        var_cje_t_db0: f64,
        var_cje_t_db1: f64,
        var_cje_t_dn0: f64,
        var_cje_t_dn1: f64,
        var_cje_t_dn10: f64,
        var_cje_t_dn2: f64,
        var_cje_t_dn3: f64,
        var_cje_t_dn4: f64,
        var_cje_t_dn5: f64,
        var_cje_t_dn6: f64,
        var_cje_t_dn7: f64,
        var_cje_t_dn8: f64,
        var_cje_t_dn9: f64,
        var_ik_t: f64,
        var_ik_t_db0: f64,
        var_ik_t_db1: f64,
        var_ik_t_dn0: f64,
        var_ik_t_dn1: f64,
        var_ik_t_dn10: f64,
        var_ik_t_dn2: f64,
        var_ik_t_dn3: f64,
        var_ik_t_dn4: f64,
        var_ik_t_dn5: f64,
        var_ik_t_dn6: f64,
        var_ik_t_dn7: f64,
        var_ik_t_dn8: f64,
        var_ik_t_dn9: f64,
        var_inv_vde_t: f64,
        var_inv_vde_t_db0: f64,
        var_inv_vde_t_db1: f64,
        var_inv_vde_t_dn0: f64,
        var_inv_vde_t_dn1: f64,
        var_inv_vde_t_dn10: f64,
        var_inv_vde_t_dn2: f64,
        var_inv_vde_t_dn3: f64,
        var_inv_vde_t_dn4: f64,
        var_inv_vde_t_dn5: f64,
        var_inv_vde_t_dn6: f64,
        var_inv_vde_t_dn7: f64,
        var_inv_vde_t_dn8: f64,
        var_inv_vde_t_dn9: f64,
        var_n0: f64,
        var_n0_db0: f64,
        var_n0_db1: f64,
        var_n0_dn0: f64,
        var_n0_dn1: f64,
        var_n0_dn10: f64,
        var_n0_dn2: f64,
        var_n0_dn3: f64,
        var_n0_dn4: f64,
        var_n0_dn5: f64,
        var_n0_dn6: f64,
        var_n0_dn7: f64,
        var_n0_dn8: f64,
        var_n0_dn9: f64,
        var_nb: f64,
        var_nb_db0: f64,
        var_nb_db1: f64,
        var_nb_dn0: f64,
        var_nb_dn1: f64,
        var_nb_dn10: f64,
        var_nb_dn2: f64,
        var_nb_dn3: f64,
        var_nb_dn4: f64,
        var_nb_dn5: f64,
        var_nb_dn6: f64,
        var_nb_dn7: f64,
        var_nb_dn8: f64,
        var_nb_dn9: f64,
        var_q1q: f64,
        var_q1q_db0: f64,
        var_q1q_db1: f64,
        var_q1q_dn0: f64,
        var_q1q_dn1: f64,
        var_q1q_dn10: f64,
        var_q1q_dn2: f64,
        var_q1q_dn3: f64,
        var_q1q_dn4: f64,
        var_q1q_dn5: f64,
        var_q1q_dn6: f64,
        var_q1q_dn7: f64,
        var_q1q_dn8: f64,
        var_q1q_dn9: f64,
        var_taub_t: f64,
        var_taub_t_db0: f64,
        var_taub_t_db1: f64,
        var_taub_t_dn0: f64,
        var_taub_t_dn1: f64,
        var_taub_t_dn10: f64,
        var_taub_t_dn2: f64,
        var_taub_t_dn3: f64,
        var_taub_t_dn4: f64,
        var_taub_t_dn5: f64,
        var_taub_t_dn6: f64,
        var_taub_t_dn7: f64,
        var_taub_t_dn8: f64,
        var_taub_t_dn9: f64,
        var_vb1c4: f64,
        var_vb1c4_db0: f64,
        var_vb1c4_db1: f64,
        var_vb1c4_dn0: f64,
        var_vb1c4_dn1: f64,
        var_vb1c4_dn10: f64,
        var_vb1c4_dn2: f64,
        var_vb1c4_dn3: f64,
        var_vb1c4_dn4: f64,
        var_vb1c4_dn5: f64,
        var_vb1c4_dn6: f64,
        var_vb1c4_dn7: f64,
        var_vb1c4_dn8: f64,
        var_vb1c4_dn9: f64,
        var_vb1e1: f64,
        var_vb1e1_db0: f64,
        var_vb1e1_db1: f64,
        var_vb1e1_dn0: f64,
        var_vb1e1_dn1: f64,
        var_vb1e1_dn10: f64,
        var_vb1e1_dn2: f64,
        var_vb1e1_dn3: f64,
        var_vb1e1_dn4: f64,
        var_vb1e1_dn5: f64,
        var_vb1e1_dn6: f64,
        var_vb1e1_dn7: f64,
        var_vb1e1_dn8: f64,
        var_vb1e1_dn9: f64,
        var_vbc3: f64,
        var_vbc3_db0: f64,
        var_vbc3_db1: f64,
        var_vbc3_dn0: f64,
        var_vbc3_dn1: f64,
        var_vbc3_dn10: f64,
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
        var_vdc_ctc_t_dn2: f64,
        var_vdc_ctc_t_dn3: f64,
        var_vdc_ctc_t_dn4: f64,
        var_vdc_ctc_t_dn5: f64,
        var_vdc_ctc_t_dn6: f64,
        var_vdc_ctc_t_dn7: f64,
        var_vdc_ctc_t_dn8: f64,
        var_vdc_ctc_t_dn9: f64,
        var_vde_t: f64,
        var_vde_t_db0: f64,
        var_vde_t_db1: f64,
        var_vde_t_dn0: f64,
        var_vde_t_dn1: f64,
        var_vde_t_dn10: f64,
        var_vde_t_dn2: f64,
        var_vde_t_dn3: f64,
        var_vde_t_dn4: f64,
        var_vde_t_dn5: f64,
        var_vde_t_dn6: f64,
        var_vde_t_dn7: f64,
        var_vde_t_dn8: f64,
        var_vde_t_dn9: f64,
        var_vfc: f64,
        var_vfc_db0: f64,
        var_vfc_db1: f64,
        var_vfc_dn0: f64,
        var_vfc_dn1: f64,
        var_vfc_dn10: f64,
        var_vfc_dn2: f64,
        var_vfc_dn3: f64,
        var_vfc_dn4: f64,
        var_vfc_dn5: f64,
        var_vfc_dn6: f64,
        var_vfc_dn7: f64,
        var_vfc_dn8: f64,
        var_vfc_dn9: f64,
        var_vje_s: f64,
        var_vje_s_db0: f64,
        var_vje_s_db1: f64,
        var_vje_s_dn0: f64,
        var_vje_s_dn1: f64,
        var_vje_s_dn10: f64,
        var_vje_s_dn2: f64,
        var_vje_s_dn3: f64,
        var_vje_s_dn4: f64,
        var_vje_s_dn5: f64,
        var_vje_s_dn6: f64,
        var_vje_s_dn7: f64,
        var_vje_s_dn8: f64,
        var_vje_s_dn9: f64,
        var_vtc: f64,
        var_vtc_db0: f64,
        var_vtc_db1: f64,
        var_vtc_dn0: f64,
        var_vtc_dn1: f64,
        var_vtc_dn10: f64,
        var_vtc_dn2: f64,
        var_vtc_dn3: f64,
        var_vtc_dn4: f64,
        var_vtc_dn5: f64,
        var_vtc_dn6: f64,
        var_vtc_dn7: f64,
        var_vtc_dn8: f64,
        var_vtc_dn9: f64,
        var_xp_t: f64,
        var_xp_t_db0: f64,
        var_xp_t_db1: f64,
        var_xp_t_dn0: f64,
        var_xp_t_dn1: f64,
        var_xp_t_dn10: f64,
        var_xp_t_dn2: f64,
        var_xp_t_dn3: f64,
        var_xp_t_dn4: f64,
        var_xp_t_dn5: f64,
        var_xp_t_dn6: f64,
        var_xp_t_dn7: f64,
        var_xp_t_dn8: f64,
        var_xp_t_dn9: f64,
        var_a_vdcctc_slot: &mut f64,
        var_a_vdcctc_db0_slot: &mut f64,
        var_a_vdcctc_db1_slot: &mut f64,
        var_a_vdcctc_dn0_slot: &mut f64,
        var_a_vdcctc_dn1_slot: &mut f64,
        var_a_vdcctc_dn10_slot: &mut f64,
        var_a_vdcctc_dn2_slot: &mut f64,
        var_a_vdcctc_dn3_slot: &mut f64,
        var_a_vdcctc_dn4_slot: &mut f64,
        var_a_vdcctc_dn5_slot: &mut f64,
        var_a_vdcctc_dn6_slot: &mut f64,
        var_a_vdcctc_dn7_slot: &mut f64,
        var_a_vdcctc_dn8_slot: &mut f64,
        var_a_vdcctc_dn9_slot: &mut f64,
        var_a_vdcctc_rdb0_slot: &mut f64,
        var_a_vdcctc_rdb1_slot: &mut f64,
        var_a_vdcctc_rdn0_slot: &mut f64,
        var_a_vdcctc_rdn1_slot: &mut f64,
        var_a_vdcctc_rdn10_slot: &mut f64,
        var_a_vdcctc_rdn2_slot: &mut f64,
        var_a_vdcctc_rdn3_slot: &mut f64,
        var_a_vdcctc_rdn4_slot: &mut f64,
        var_a_vdcctc_rdn5_slot: &mut f64,
        var_a_vdcctc_rdn6_slot: &mut f64,
        var_a_vdcctc_rdn7_slot: &mut f64,
        var_a_vdcctc_rdn8_slot: &mut f64,
        var_a_vdcctc_rdn9_slot: &mut f64,
        var_a_vdcctc_rv_slot: &mut f64,
        var_dxa_slot: &mut f64,
        var_dxa_db0_slot: &mut f64,
        var_dxa_db1_slot: &mut f64,
        var_dxa_dn0_slot: &mut f64,
        var_dxa_dn1_slot: &mut f64,
        var_dxa_dn10_slot: &mut f64,
        var_dxa_dn2_slot: &mut f64,
        var_dxa_dn3_slot: &mut f64,
        var_dxa_dn4_slot: &mut f64,
        var_dxa_dn5_slot: &mut f64,
        var_dxa_dn6_slot: &mut f64,
        var_dxa_dn7_slot: &mut f64,
        var_dxa_dn8_slot: &mut f64,
        var_dxa_dn9_slot: &mut f64,
        var_dxa_rdb0_slot: &mut f64,
        var_dxa_rdb1_slot: &mut f64,
        var_dxa_rdn0_slot: &mut f64,
        var_dxa_rdn1_slot: &mut f64,
        var_dxa_rdn10_slot: &mut f64,
        var_dxa_rdn2_slot: &mut f64,
        var_dxa_rdn3_slot: &mut f64,
        var_dxa_rdn4_slot: &mut f64,
        var_dxa_rdn5_slot: &mut f64,
        var_dxa_rdn6_slot: &mut f64,
        var_dxa_rdn7_slot: &mut f64,
        var_dxa_rdn8_slot: &mut f64,
        var_dxa_rdn9_slot: &mut f64,
        var_dxa_rv_slot: &mut f64,
        var_guard104_slot: &mut f64,
        var_guard104_db0_slot: &mut f64,
        var_guard104_db1_slot: &mut f64,
        var_guard104_dn0_slot: &mut f64,
        var_guard104_dn1_slot: &mut f64,
        var_guard104_dn10_slot: &mut f64,
        var_guard104_dn2_slot: &mut f64,
        var_guard104_dn3_slot: &mut f64,
        var_guard104_dn4_slot: &mut f64,
        var_guard104_dn5_slot: &mut f64,
        var_guard104_dn6_slot: &mut f64,
        var_guard104_dn7_slot: &mut f64,
        var_guard104_dn8_slot: &mut f64,
        var_guard104_dn9_slot: &mut f64,
        var_guard104_rdb0_slot: &mut f64,
        var_guard104_rdb1_slot: &mut f64,
        var_guard104_rdn0_slot: &mut f64,
        var_guard104_rdn1_slot: &mut f64,
        var_guard104_rdn10_slot: &mut f64,
        var_guard104_rdn2_slot: &mut f64,
        var_guard104_rdn3_slot: &mut f64,
        var_guard104_rdn4_slot: &mut f64,
        var_guard104_rdn5_slot: &mut f64,
        var_guard104_rdn6_slot: &mut f64,
        var_guard104_rdn7_slot: &mut f64,
        var_guard104_rdn8_slot: &mut f64,
        var_guard104_rdn9_slot: &mut f64,
        var_guard104_rv_slot: &mut f64,
        var_guard105_slot: &mut f64,
        var_guard105_db0_slot: &mut f64,
        var_guard105_db1_slot: &mut f64,
        var_guard105_dn0_slot: &mut f64,
        var_guard105_dn1_slot: &mut f64,
        var_guard105_dn10_slot: &mut f64,
        var_guard105_dn2_slot: &mut f64,
        var_guard105_dn3_slot: &mut f64,
        var_guard105_dn4_slot: &mut f64,
        var_guard105_dn5_slot: &mut f64,
        var_guard105_dn6_slot: &mut f64,
        var_guard105_dn7_slot: &mut f64,
        var_guard105_dn8_slot: &mut f64,
        var_guard105_dn9_slot: &mut f64,
        var_guard105_rdb0_slot: &mut f64,
        var_guard105_rdb1_slot: &mut f64,
        var_guard105_rdn0_slot: &mut f64,
        var_guard105_rdn1_slot: &mut f64,
        var_guard105_rdn10_slot: &mut f64,
        var_guard105_rdn2_slot: &mut f64,
        var_guard105_rdn3_slot: &mut f64,
        var_guard105_rdn4_slot: &mut f64,
        var_guard105_rdn5_slot: &mut f64,
        var_guard105_rdn6_slot: &mut f64,
        var_guard105_rdn7_slot: &mut f64,
        var_guard105_rdn8_slot: &mut f64,
        var_guard105_rdn9_slot: &mut f64,
        var_guard105_rv_slot: &mut f64,
        var_qb0_slot: &mut f64,
        var_qb0_db0_slot: &mut f64,
        var_qb0_db1_slot: &mut f64,
        var_qb0_dn0_slot: &mut f64,
        var_qb0_dn1_slot: &mut f64,
        var_qb0_dn10_slot: &mut f64,
        var_qb0_dn2_slot: &mut f64,
        var_qb0_dn3_slot: &mut f64,
        var_qb0_dn4_slot: &mut f64,
        var_qb0_dn5_slot: &mut f64,
        var_qb0_dn6_slot: &mut f64,
        var_qb0_dn7_slot: &mut f64,
        var_qb0_dn8_slot: &mut f64,
        var_qb0_dn9_slot: &mut f64,
        var_qb0_rdb0_slot: &mut f64,
        var_qb0_rdb1_slot: &mut f64,
        var_qb0_rdn0_slot: &mut f64,
        var_qb0_rdn1_slot: &mut f64,
        var_qb0_rdn10_slot: &mut f64,
        var_qb0_rdn2_slot: &mut f64,
        var_qb0_rdn3_slot: &mut f64,
        var_qb0_rdn4_slot: &mut f64,
        var_qb0_rdn5_slot: &mut f64,
        var_qb0_rdn6_slot: &mut f64,
        var_qb0_rdn7_slot: &mut f64,
        var_qb0_rdn8_slot: &mut f64,
        var_qb0_rdn9_slot: &mut f64,
        var_qb0_rv_slot: &mut f64,
        var_qbc_qs_slot: &mut f64,
        var_qbc_qs_db0_slot: &mut f64,
        var_qbc_qs_db1_slot: &mut f64,
        var_qbc_qs_dn0_slot: &mut f64,
        var_qbc_qs_dn1_slot: &mut f64,
        var_qbc_qs_dn10_slot: &mut f64,
        var_qbc_qs_dn2_slot: &mut f64,
        var_qbc_qs_dn3_slot: &mut f64,
        var_qbc_qs_dn4_slot: &mut f64,
        var_qbc_qs_dn5_slot: &mut f64,
        var_qbc_qs_dn6_slot: &mut f64,
        var_qbc_qs_dn7_slot: &mut f64,
        var_qbc_qs_dn8_slot: &mut f64,
        var_qbc_qs_dn9_slot: &mut f64,
        var_qbc_qs_rdb0_slot: &mut f64,
        var_qbc_qs_rdb1_slot: &mut f64,
        var_qbc_qs_rdn0_slot: &mut f64,
        var_qbc_qs_rdn1_slot: &mut f64,
        var_qbc_qs_rdn10_slot: &mut f64,
        var_qbc_qs_rdn2_slot: &mut f64,
        var_qbc_qs_rdn3_slot: &mut f64,
        var_qbc_qs_rdn4_slot: &mut f64,
        var_qbc_qs_rdn5_slot: &mut f64,
        var_qbc_qs_rdn6_slot: &mut f64,
        var_qbc_qs_rdn7_slot: &mut f64,
        var_qbc_qs_rdn8_slot: &mut f64,
        var_qbc_qs_rdn9_slot: &mut f64,
        var_qbc_qs_rv_slot: &mut f64,
        var_qbe_qs_slot: &mut f64,
        var_qbe_qs_db0_slot: &mut f64,
        var_qbe_qs_db1_slot: &mut f64,
        var_qbe_qs_dn0_slot: &mut f64,
        var_qbe_qs_dn1_slot: &mut f64,
        var_qbe_qs_dn10_slot: &mut f64,
        var_qbe_qs_dn2_slot: &mut f64,
        var_qbe_qs_dn3_slot: &mut f64,
        var_qbe_qs_dn4_slot: &mut f64,
        var_qbe_qs_dn5_slot: &mut f64,
        var_qbe_qs_dn6_slot: &mut f64,
        var_qbe_qs_dn7_slot: &mut f64,
        var_qbe_qs_dn8_slot: &mut f64,
        var_qbe_qs_dn9_slot: &mut f64,
        var_qbe_qs_rdb0_slot: &mut f64,
        var_qbe_qs_rdb1_slot: &mut f64,
        var_qbe_qs_rdn0_slot: &mut f64,
        var_qbe_qs_rdn1_slot: &mut f64,
        var_qbe_qs_rdn10_slot: &mut f64,
        var_qbe_qs_rdn2_slot: &mut f64,
        var_qbe_qs_rdn3_slot: &mut f64,
        var_qbe_qs_rdn4_slot: &mut f64,
        var_qbe_qs_rdn5_slot: &mut f64,
        var_qbe_qs_rdn6_slot: &mut f64,
        var_qbe_qs_rdn7_slot: &mut f64,
        var_qbe_qs_rdn8_slot: &mut f64,
        var_qbe_qs_rdn9_slot: &mut f64,
        var_qbe_qs_rv_slot: &mut f64,
        var_qtc_slot: &mut f64,
        var_qtc_db0_slot: &mut f64,
        var_qtc_db1_slot: &mut f64,
        var_qtc_dn0_slot: &mut f64,
        var_qtc_dn1_slot: &mut f64,
        var_qtc_dn10_slot: &mut f64,
        var_qtc_dn2_slot: &mut f64,
        var_qtc_dn3_slot: &mut f64,
        var_qtc_dn4_slot: &mut f64,
        var_qtc_dn5_slot: &mut f64,
        var_qtc_dn6_slot: &mut f64,
        var_qtc_dn7_slot: &mut f64,
        var_qtc_dn8_slot: &mut f64,
        var_qtc_dn9_slot: &mut f64,
        var_qtc_rdb0_slot: &mut f64,
        var_qtc_rdb1_slot: &mut f64,
        var_qtc_rdn0_slot: &mut f64,
        var_qtc_rdn1_slot: &mut f64,
        var_qtc_rdn10_slot: &mut f64,
        var_qtc_rdn2_slot: &mut f64,
        var_qtc_rdn3_slot: &mut f64,
        var_qtc_rdn4_slot: &mut f64,
        var_qtc_rdn5_slot: &mut f64,
        var_qtc_rdn6_slot: &mut f64,
        var_qtc_rdn7_slot: &mut f64,
        var_qtc_rdn8_slot: &mut f64,
        var_qtc_rdn9_slot: &mut f64,
        var_qtc_rv_slot: &mut f64,
        var_qte_s_slot: &mut f64,
        var_qte_s_db0_slot: &mut f64,
        var_qte_s_db1_slot: &mut f64,
        var_qte_s_dn0_slot: &mut f64,
        var_qte_s_dn1_slot: &mut f64,
        var_qte_s_dn10_slot: &mut f64,
        var_qte_s_dn2_slot: &mut f64,
        var_qte_s_dn3_slot: &mut f64,
        var_qte_s_dn4_slot: &mut f64,
        var_qte_s_dn5_slot: &mut f64,
        var_qte_s_dn6_slot: &mut f64,
        var_qte_s_dn7_slot: &mut f64,
        var_qte_s_dn8_slot: &mut f64,
        var_qte_s_dn9_slot: &mut f64,
        var_qte_s_rdb0_slot: &mut f64,
        var_qte_s_rdb1_slot: &mut f64,
        var_qte_s_rdn0_slot: &mut f64,
        var_qte_s_rdn1_slot: &mut f64,
        var_qte_s_rdn10_slot: &mut f64,
        var_qte_s_rdn2_slot: &mut f64,
        var_qte_s_rdn3_slot: &mut f64,
        var_qte_s_rdn4_slot: &mut f64,
        var_qte_s_rdn5_slot: &mut f64,
        var_qte_s_rdn6_slot: &mut f64,
        var_qte_s_rdn7_slot: &mut f64,
        var_qte_s_rdn8_slot: &mut f64,
        var_qte_s_rdn9_slot: &mut f64,
        var_qte_s_rv_slot: &mut f64,
        var_qtex_slot: &mut f64,
        var_qtex_db0_slot: &mut f64,
        var_qtex_db1_slot: &mut f64,
        var_qtex_dn0_slot: &mut f64,
        var_qtex_dn1_slot: &mut f64,
        var_qtex_dn10_slot: &mut f64,
        var_qtex_dn2_slot: &mut f64,
        var_qtex_dn3_slot: &mut f64,
        var_qtex_dn4_slot: &mut f64,
        var_qtex_dn5_slot: &mut f64,
        var_qtex_dn6_slot: &mut f64,
        var_qtex_dn7_slot: &mut f64,
        var_qtex_dn8_slot: &mut f64,
        var_qtex_dn9_slot: &mut f64,
        var_qtex_rdb0_slot: &mut f64,
        var_qtex_rdb1_slot: &mut f64,
        var_qtex_rdn0_slot: &mut f64,
        var_qtex_rdn1_slot: &mut f64,
        var_qtex_rdn10_slot: &mut f64,
        var_qtex_rdn2_slot: &mut f64,
        var_qtex_rdn3_slot: &mut f64,
        var_qtex_rdn4_slot: &mut f64,
        var_qtex_rdn5_slot: &mut f64,
        var_qtex_rdn6_slot: &mut f64,
        var_qtex_rdn7_slot: &mut f64,
        var_qtex_rdn8_slot: &mut f64,
        var_qtex_rdn9_slot: &mut f64,
        var_qtex_rv_slot: &mut f64,
        var_vjcex_slot: &mut f64,
        var_vjcex_db0_slot: &mut f64,
        var_vjcex_db1_slot: &mut f64,
        var_vjcex_dn0_slot: &mut f64,
        var_vjcex_dn1_slot: &mut f64,
        var_vjcex_dn10_slot: &mut f64,
        var_vjcex_dn2_slot: &mut f64,
        var_vjcex_dn3_slot: &mut f64,
        var_vjcex_dn4_slot: &mut f64,
        var_vjcex_dn5_slot: &mut f64,
        var_vjcex_dn6_slot: &mut f64,
        var_vjcex_dn7_slot: &mut f64,
        var_vjcex_dn8_slot: &mut f64,
        var_vjcex_dn9_slot: &mut f64,
        var_vjcex_rdb0_slot: &mut f64,
        var_vjcex_rdb1_slot: &mut f64,
        var_vjcex_rdn0_slot: &mut f64,
        var_vjcex_rdn1_slot: &mut f64,
        var_vjcex_rdn10_slot: &mut f64,
        var_vjcex_rdn2_slot: &mut f64,
        var_vjcex_rdn3_slot: &mut f64,
        var_vjcex_rdn4_slot: &mut f64,
        var_vjcex_rdn5_slot: &mut f64,
        var_vjcex_rdn6_slot: &mut f64,
        var_vjcex_rdn7_slot: &mut f64,
        var_vjcex_rdn8_slot: &mut f64,
        var_vjcex_rdn9_slot: &mut f64,
        var_vjcex_rv_slot: &mut f64,
        var_vtexv_slot: &mut f64,
        var_vtexv_db0_slot: &mut f64,
        var_vtexv_db1_slot: &mut f64,
        var_vtexv_dn0_slot: &mut f64,
        var_vtexv_dn1_slot: &mut f64,
        var_vtexv_dn10_slot: &mut f64,
        var_vtexv_dn2_slot: &mut f64,
        var_vtexv_dn3_slot: &mut f64,
        var_vtexv_dn4_slot: &mut f64,
        var_vtexv_dn5_slot: &mut f64,
        var_vtexv_dn6_slot: &mut f64,
        var_vtexv_dn7_slot: &mut f64,
        var_vtexv_dn8_slot: &mut f64,
        var_vtexv_dn9_slot: &mut f64,
        var_vtexv_rdb0_slot: &mut f64,
        var_vtexv_rdb1_slot: &mut f64,
        var_vtexv_rdn0_slot: &mut f64,
        var_vtexv_rdn1_slot: &mut f64,
        var_vtexv_rdn10_slot: &mut f64,
        var_vtexv_rdn2_slot: &mut f64,
        var_vtexv_rdn3_slot: &mut f64,
        var_vtexv_rdn4_slot: &mut f64,
        var_vtexv_rdn5_slot: &mut f64,
        var_vtexv_rdn6_slot: &mut f64,
        var_vtexv_rdn7_slot: &mut f64,
        var_vtexv_rdn8_slot: &mut f64,
        var_vtexv_rdn9_slot: &mut f64,
        var_vtexv_rv_slot: &mut f64,
    ) {
        let mut var_a_vdcctc: f64 = *var_a_vdcctc_slot;
        let mut var_a_vdcctc_db0: f64 = *var_a_vdcctc_db0_slot;
        let mut var_a_vdcctc_db1: f64 = *var_a_vdcctc_db1_slot;
        let mut var_a_vdcctc_dn0: f64 = *var_a_vdcctc_dn0_slot;
        let mut var_a_vdcctc_dn1: f64 = *var_a_vdcctc_dn1_slot;
        let mut var_a_vdcctc_dn10: f64 = *var_a_vdcctc_dn10_slot;
        let mut var_a_vdcctc_dn2: f64 = *var_a_vdcctc_dn2_slot;
        let mut var_a_vdcctc_dn3: f64 = *var_a_vdcctc_dn3_slot;
        let mut var_a_vdcctc_dn4: f64 = *var_a_vdcctc_dn4_slot;
        let mut var_a_vdcctc_dn5: f64 = *var_a_vdcctc_dn5_slot;
        let mut var_a_vdcctc_dn6: f64 = *var_a_vdcctc_dn6_slot;
        let mut var_a_vdcctc_dn7: f64 = *var_a_vdcctc_dn7_slot;
        let mut var_a_vdcctc_dn8: f64 = *var_a_vdcctc_dn8_slot;
        let mut var_a_vdcctc_dn9: f64 = *var_a_vdcctc_dn9_slot;
        let mut var_a_vdcctc_rdb0: f64 = *var_a_vdcctc_rdb0_slot;
        let mut var_a_vdcctc_rdb1: f64 = *var_a_vdcctc_rdb1_slot;
        let mut var_a_vdcctc_rdn0: f64 = *var_a_vdcctc_rdn0_slot;
        let mut var_a_vdcctc_rdn1: f64 = *var_a_vdcctc_rdn1_slot;
        let mut var_a_vdcctc_rdn10: f64 = *var_a_vdcctc_rdn10_slot;
        let mut var_a_vdcctc_rdn2: f64 = *var_a_vdcctc_rdn2_slot;
        let mut var_a_vdcctc_rdn3: f64 = *var_a_vdcctc_rdn3_slot;
        let mut var_a_vdcctc_rdn4: f64 = *var_a_vdcctc_rdn4_slot;
        let mut var_a_vdcctc_rdn5: f64 = *var_a_vdcctc_rdn5_slot;
        let mut var_a_vdcctc_rdn6: f64 = *var_a_vdcctc_rdn6_slot;
        let mut var_a_vdcctc_rdn7: f64 = *var_a_vdcctc_rdn7_slot;
        let mut var_a_vdcctc_rdn8: f64 = *var_a_vdcctc_rdn8_slot;
        let mut var_a_vdcctc_rdn9: f64 = *var_a_vdcctc_rdn9_slot;
        let mut var_a_vdcctc_rv: f64 = *var_a_vdcctc_rv_slot;
        let mut var_dxa: f64 = *var_dxa_slot;
        let mut var_dxa_db0: f64 = *var_dxa_db0_slot;
        let mut var_dxa_db1: f64 = *var_dxa_db1_slot;
        let mut var_dxa_dn0: f64 = *var_dxa_dn0_slot;
        let mut var_dxa_dn1: f64 = *var_dxa_dn1_slot;
        let mut var_dxa_dn10: f64 = *var_dxa_dn10_slot;
        let mut var_dxa_dn2: f64 = *var_dxa_dn2_slot;
        let mut var_dxa_dn3: f64 = *var_dxa_dn3_slot;
        let mut var_dxa_dn4: f64 = *var_dxa_dn4_slot;
        let mut var_dxa_dn5: f64 = *var_dxa_dn5_slot;
        let mut var_dxa_dn6: f64 = *var_dxa_dn6_slot;
        let mut var_dxa_dn7: f64 = *var_dxa_dn7_slot;
        let mut var_dxa_dn8: f64 = *var_dxa_dn8_slot;
        let mut var_dxa_dn9: f64 = *var_dxa_dn9_slot;
        let mut var_dxa_rdb0: f64 = *var_dxa_rdb0_slot;
        let mut var_dxa_rdb1: f64 = *var_dxa_rdb1_slot;
        let mut var_dxa_rdn0: f64 = *var_dxa_rdn0_slot;
        let mut var_dxa_rdn1: f64 = *var_dxa_rdn1_slot;
        let mut var_dxa_rdn10: f64 = *var_dxa_rdn10_slot;
        let mut var_dxa_rdn2: f64 = *var_dxa_rdn2_slot;
        let mut var_dxa_rdn3: f64 = *var_dxa_rdn3_slot;
        let mut var_dxa_rdn4: f64 = *var_dxa_rdn4_slot;
        let mut var_dxa_rdn5: f64 = *var_dxa_rdn5_slot;
        let mut var_dxa_rdn6: f64 = *var_dxa_rdn6_slot;
        let mut var_dxa_rdn7: f64 = *var_dxa_rdn7_slot;
        let mut var_dxa_rdn8: f64 = *var_dxa_rdn8_slot;
        let mut var_dxa_rdn9: f64 = *var_dxa_rdn9_slot;
        let mut var_dxa_rv: f64 = *var_dxa_rv_slot;
        let mut var_guard104: f64 = *var_guard104_slot;
        let mut var_guard104_db0: f64 = *var_guard104_db0_slot;
        let mut var_guard104_db1: f64 = *var_guard104_db1_slot;
        let mut var_guard104_dn0: f64 = *var_guard104_dn0_slot;
        let mut var_guard104_dn1: f64 = *var_guard104_dn1_slot;
        let mut var_guard104_dn10: f64 = *var_guard104_dn10_slot;
        let mut var_guard104_dn2: f64 = *var_guard104_dn2_slot;
        let mut var_guard104_dn3: f64 = *var_guard104_dn3_slot;
        let mut var_guard104_dn4: f64 = *var_guard104_dn4_slot;
        let mut var_guard104_dn5: f64 = *var_guard104_dn5_slot;
        let mut var_guard104_dn6: f64 = *var_guard104_dn6_slot;
        let mut var_guard104_dn7: f64 = *var_guard104_dn7_slot;
        let mut var_guard104_dn8: f64 = *var_guard104_dn8_slot;
        let mut var_guard104_dn9: f64 = *var_guard104_dn9_slot;
        let mut var_guard104_rdb0: f64 = *var_guard104_rdb0_slot;
        let mut var_guard104_rdb1: f64 = *var_guard104_rdb1_slot;
        let mut var_guard104_rdn0: f64 = *var_guard104_rdn0_slot;
        let mut var_guard104_rdn1: f64 = *var_guard104_rdn1_slot;
        let mut var_guard104_rdn10: f64 = *var_guard104_rdn10_slot;
        let mut var_guard104_rdn2: f64 = *var_guard104_rdn2_slot;
        let mut var_guard104_rdn3: f64 = *var_guard104_rdn3_slot;
        let mut var_guard104_rdn4: f64 = *var_guard104_rdn4_slot;
        let mut var_guard104_rdn5: f64 = *var_guard104_rdn5_slot;
        let mut var_guard104_rdn6: f64 = *var_guard104_rdn6_slot;
        let mut var_guard104_rdn7: f64 = *var_guard104_rdn7_slot;
        let mut var_guard104_rdn8: f64 = *var_guard104_rdn8_slot;
        let mut var_guard104_rdn9: f64 = *var_guard104_rdn9_slot;
        let mut var_guard104_rv: f64 = *var_guard104_rv_slot;
        let mut var_guard105: f64 = *var_guard105_slot;
        let mut var_guard105_db0: f64 = *var_guard105_db0_slot;
        let mut var_guard105_db1: f64 = *var_guard105_db1_slot;
        let mut var_guard105_dn0: f64 = *var_guard105_dn0_slot;
        let mut var_guard105_dn1: f64 = *var_guard105_dn1_slot;
        let mut var_guard105_dn10: f64 = *var_guard105_dn10_slot;
        let mut var_guard105_dn2: f64 = *var_guard105_dn2_slot;
        let mut var_guard105_dn3: f64 = *var_guard105_dn3_slot;
        let mut var_guard105_dn4: f64 = *var_guard105_dn4_slot;
        let mut var_guard105_dn5: f64 = *var_guard105_dn5_slot;
        let mut var_guard105_dn6: f64 = *var_guard105_dn6_slot;
        let mut var_guard105_dn7: f64 = *var_guard105_dn7_slot;
        let mut var_guard105_dn8: f64 = *var_guard105_dn8_slot;
        let mut var_guard105_dn9: f64 = *var_guard105_dn9_slot;
        let mut var_guard105_rdb0: f64 = *var_guard105_rdb0_slot;
        let mut var_guard105_rdb1: f64 = *var_guard105_rdb1_slot;
        let mut var_guard105_rdn0: f64 = *var_guard105_rdn0_slot;
        let mut var_guard105_rdn1: f64 = *var_guard105_rdn1_slot;
        let mut var_guard105_rdn10: f64 = *var_guard105_rdn10_slot;
        let mut var_guard105_rdn2: f64 = *var_guard105_rdn2_slot;
        let mut var_guard105_rdn3: f64 = *var_guard105_rdn3_slot;
        let mut var_guard105_rdn4: f64 = *var_guard105_rdn4_slot;
        let mut var_guard105_rdn5: f64 = *var_guard105_rdn5_slot;
        let mut var_guard105_rdn6: f64 = *var_guard105_rdn6_slot;
        let mut var_guard105_rdn7: f64 = *var_guard105_rdn7_slot;
        let mut var_guard105_rdn8: f64 = *var_guard105_rdn8_slot;
        let mut var_guard105_rdn9: f64 = *var_guard105_rdn9_slot;
        let mut var_guard105_rv: f64 = *var_guard105_rv_slot;
        let mut var_qb0: f64 = *var_qb0_slot;
        let mut var_qb0_db0: f64 = *var_qb0_db0_slot;
        let mut var_qb0_db1: f64 = *var_qb0_db1_slot;
        let mut var_qb0_dn0: f64 = *var_qb0_dn0_slot;
        let mut var_qb0_dn1: f64 = *var_qb0_dn1_slot;
        let mut var_qb0_dn10: f64 = *var_qb0_dn10_slot;
        let mut var_qb0_dn2: f64 = *var_qb0_dn2_slot;
        let mut var_qb0_dn3: f64 = *var_qb0_dn3_slot;
        let mut var_qb0_dn4: f64 = *var_qb0_dn4_slot;
        let mut var_qb0_dn5: f64 = *var_qb0_dn5_slot;
        let mut var_qb0_dn6: f64 = *var_qb0_dn6_slot;
        let mut var_qb0_dn7: f64 = *var_qb0_dn7_slot;
        let mut var_qb0_dn8: f64 = *var_qb0_dn8_slot;
        let mut var_qb0_dn9: f64 = *var_qb0_dn9_slot;
        let mut var_qb0_rdb0: f64 = *var_qb0_rdb0_slot;
        let mut var_qb0_rdb1: f64 = *var_qb0_rdb1_slot;
        let mut var_qb0_rdn0: f64 = *var_qb0_rdn0_slot;
        let mut var_qb0_rdn1: f64 = *var_qb0_rdn1_slot;
        let mut var_qb0_rdn10: f64 = *var_qb0_rdn10_slot;
        let mut var_qb0_rdn2: f64 = *var_qb0_rdn2_slot;
        let mut var_qb0_rdn3: f64 = *var_qb0_rdn3_slot;
        let mut var_qb0_rdn4: f64 = *var_qb0_rdn4_slot;
        let mut var_qb0_rdn5: f64 = *var_qb0_rdn5_slot;
        let mut var_qb0_rdn6: f64 = *var_qb0_rdn6_slot;
        let mut var_qb0_rdn7: f64 = *var_qb0_rdn7_slot;
        let mut var_qb0_rdn8: f64 = *var_qb0_rdn8_slot;
        let mut var_qb0_rdn9: f64 = *var_qb0_rdn9_slot;
        let mut var_qb0_rv: f64 = *var_qb0_rv_slot;
        let mut var_qbc_qs: f64 = *var_qbc_qs_slot;
        let mut var_qbc_qs_db0: f64 = *var_qbc_qs_db0_slot;
        let mut var_qbc_qs_db1: f64 = *var_qbc_qs_db1_slot;
        let mut var_qbc_qs_dn0: f64 = *var_qbc_qs_dn0_slot;
        let mut var_qbc_qs_dn1: f64 = *var_qbc_qs_dn1_slot;
        let mut var_qbc_qs_dn10: f64 = *var_qbc_qs_dn10_slot;
        let mut var_qbc_qs_dn2: f64 = *var_qbc_qs_dn2_slot;
        let mut var_qbc_qs_dn3: f64 = *var_qbc_qs_dn3_slot;
        let mut var_qbc_qs_dn4: f64 = *var_qbc_qs_dn4_slot;
        let mut var_qbc_qs_dn5: f64 = *var_qbc_qs_dn5_slot;
        let mut var_qbc_qs_dn6: f64 = *var_qbc_qs_dn6_slot;
        let mut var_qbc_qs_dn7: f64 = *var_qbc_qs_dn7_slot;
        let mut var_qbc_qs_dn8: f64 = *var_qbc_qs_dn8_slot;
        let mut var_qbc_qs_dn9: f64 = *var_qbc_qs_dn9_slot;
        let mut var_qbc_qs_rdb0: f64 = *var_qbc_qs_rdb0_slot;
        let mut var_qbc_qs_rdb1: f64 = *var_qbc_qs_rdb1_slot;
        let mut var_qbc_qs_rdn0: f64 = *var_qbc_qs_rdn0_slot;
        let mut var_qbc_qs_rdn1: f64 = *var_qbc_qs_rdn1_slot;
        let mut var_qbc_qs_rdn10: f64 = *var_qbc_qs_rdn10_slot;
        let mut var_qbc_qs_rdn2: f64 = *var_qbc_qs_rdn2_slot;
        let mut var_qbc_qs_rdn3: f64 = *var_qbc_qs_rdn3_slot;
        let mut var_qbc_qs_rdn4: f64 = *var_qbc_qs_rdn4_slot;
        let mut var_qbc_qs_rdn5: f64 = *var_qbc_qs_rdn5_slot;
        let mut var_qbc_qs_rdn6: f64 = *var_qbc_qs_rdn6_slot;
        let mut var_qbc_qs_rdn7: f64 = *var_qbc_qs_rdn7_slot;
        let mut var_qbc_qs_rdn8: f64 = *var_qbc_qs_rdn8_slot;
        let mut var_qbc_qs_rdn9: f64 = *var_qbc_qs_rdn9_slot;
        let mut var_qbc_qs_rv: f64 = *var_qbc_qs_rv_slot;
        let mut var_qbe_qs: f64 = *var_qbe_qs_slot;
        let mut var_qbe_qs_db0: f64 = *var_qbe_qs_db0_slot;
        let mut var_qbe_qs_db1: f64 = *var_qbe_qs_db1_slot;
        let mut var_qbe_qs_dn0: f64 = *var_qbe_qs_dn0_slot;
        let mut var_qbe_qs_dn1: f64 = *var_qbe_qs_dn1_slot;
        let mut var_qbe_qs_dn10: f64 = *var_qbe_qs_dn10_slot;
        let mut var_qbe_qs_dn2: f64 = *var_qbe_qs_dn2_slot;
        let mut var_qbe_qs_dn3: f64 = *var_qbe_qs_dn3_slot;
        let mut var_qbe_qs_dn4: f64 = *var_qbe_qs_dn4_slot;
        let mut var_qbe_qs_dn5: f64 = *var_qbe_qs_dn5_slot;
        let mut var_qbe_qs_dn6: f64 = *var_qbe_qs_dn6_slot;
        let mut var_qbe_qs_dn7: f64 = *var_qbe_qs_dn7_slot;
        let mut var_qbe_qs_dn8: f64 = *var_qbe_qs_dn8_slot;
        let mut var_qbe_qs_dn9: f64 = *var_qbe_qs_dn9_slot;
        let mut var_qbe_qs_rdb0: f64 = *var_qbe_qs_rdb0_slot;
        let mut var_qbe_qs_rdb1: f64 = *var_qbe_qs_rdb1_slot;
        let mut var_qbe_qs_rdn0: f64 = *var_qbe_qs_rdn0_slot;
        let mut var_qbe_qs_rdn1: f64 = *var_qbe_qs_rdn1_slot;
        let mut var_qbe_qs_rdn10: f64 = *var_qbe_qs_rdn10_slot;
        let mut var_qbe_qs_rdn2: f64 = *var_qbe_qs_rdn2_slot;
        let mut var_qbe_qs_rdn3: f64 = *var_qbe_qs_rdn3_slot;
        let mut var_qbe_qs_rdn4: f64 = *var_qbe_qs_rdn4_slot;
        let mut var_qbe_qs_rdn5: f64 = *var_qbe_qs_rdn5_slot;
        let mut var_qbe_qs_rdn6: f64 = *var_qbe_qs_rdn6_slot;
        let mut var_qbe_qs_rdn7: f64 = *var_qbe_qs_rdn7_slot;
        let mut var_qbe_qs_rdn8: f64 = *var_qbe_qs_rdn8_slot;
        let mut var_qbe_qs_rdn9: f64 = *var_qbe_qs_rdn9_slot;
        let mut var_qbe_qs_rv: f64 = *var_qbe_qs_rv_slot;
        let mut var_qtc: f64 = *var_qtc_slot;
        let mut var_qtc_db0: f64 = *var_qtc_db0_slot;
        let mut var_qtc_db1: f64 = *var_qtc_db1_slot;
        let mut var_qtc_dn0: f64 = *var_qtc_dn0_slot;
        let mut var_qtc_dn1: f64 = *var_qtc_dn1_slot;
        let mut var_qtc_dn10: f64 = *var_qtc_dn10_slot;
        let mut var_qtc_dn2: f64 = *var_qtc_dn2_slot;
        let mut var_qtc_dn3: f64 = *var_qtc_dn3_slot;
        let mut var_qtc_dn4: f64 = *var_qtc_dn4_slot;
        let mut var_qtc_dn5: f64 = *var_qtc_dn5_slot;
        let mut var_qtc_dn6: f64 = *var_qtc_dn6_slot;
        let mut var_qtc_dn7: f64 = *var_qtc_dn7_slot;
        let mut var_qtc_dn8: f64 = *var_qtc_dn8_slot;
        let mut var_qtc_dn9: f64 = *var_qtc_dn9_slot;
        let mut var_qtc_rdb0: f64 = *var_qtc_rdb0_slot;
        let mut var_qtc_rdb1: f64 = *var_qtc_rdb1_slot;
        let mut var_qtc_rdn0: f64 = *var_qtc_rdn0_slot;
        let mut var_qtc_rdn1: f64 = *var_qtc_rdn1_slot;
        let mut var_qtc_rdn10: f64 = *var_qtc_rdn10_slot;
        let mut var_qtc_rdn2: f64 = *var_qtc_rdn2_slot;
        let mut var_qtc_rdn3: f64 = *var_qtc_rdn3_slot;
        let mut var_qtc_rdn4: f64 = *var_qtc_rdn4_slot;
        let mut var_qtc_rdn5: f64 = *var_qtc_rdn5_slot;
        let mut var_qtc_rdn6: f64 = *var_qtc_rdn6_slot;
        let mut var_qtc_rdn7: f64 = *var_qtc_rdn7_slot;
        let mut var_qtc_rdn8: f64 = *var_qtc_rdn8_slot;
        let mut var_qtc_rdn9: f64 = *var_qtc_rdn9_slot;
        let mut var_qtc_rv: f64 = *var_qtc_rv_slot;
        let mut var_qte_s: f64 = *var_qte_s_slot;
        let mut var_qte_s_db0: f64 = *var_qte_s_db0_slot;
        let mut var_qte_s_db1: f64 = *var_qte_s_db1_slot;
        let mut var_qte_s_dn0: f64 = *var_qte_s_dn0_slot;
        let mut var_qte_s_dn1: f64 = *var_qte_s_dn1_slot;
        let mut var_qte_s_dn10: f64 = *var_qte_s_dn10_slot;
        let mut var_qte_s_dn2: f64 = *var_qte_s_dn2_slot;
        let mut var_qte_s_dn3: f64 = *var_qte_s_dn3_slot;
        let mut var_qte_s_dn4: f64 = *var_qte_s_dn4_slot;
        let mut var_qte_s_dn5: f64 = *var_qte_s_dn5_slot;
        let mut var_qte_s_dn6: f64 = *var_qte_s_dn6_slot;
        let mut var_qte_s_dn7: f64 = *var_qte_s_dn7_slot;
        let mut var_qte_s_dn8: f64 = *var_qte_s_dn8_slot;
        let mut var_qte_s_dn9: f64 = *var_qte_s_dn9_slot;
        let mut var_qte_s_rdb0: f64 = *var_qte_s_rdb0_slot;
        let mut var_qte_s_rdb1: f64 = *var_qte_s_rdb1_slot;
        let mut var_qte_s_rdn0: f64 = *var_qte_s_rdn0_slot;
        let mut var_qte_s_rdn1: f64 = *var_qte_s_rdn1_slot;
        let mut var_qte_s_rdn10: f64 = *var_qte_s_rdn10_slot;
        let mut var_qte_s_rdn2: f64 = *var_qte_s_rdn2_slot;
        let mut var_qte_s_rdn3: f64 = *var_qte_s_rdn3_slot;
        let mut var_qte_s_rdn4: f64 = *var_qte_s_rdn4_slot;
        let mut var_qte_s_rdn5: f64 = *var_qte_s_rdn5_slot;
        let mut var_qte_s_rdn6: f64 = *var_qte_s_rdn6_slot;
        let mut var_qte_s_rdn7: f64 = *var_qte_s_rdn7_slot;
        let mut var_qte_s_rdn8: f64 = *var_qte_s_rdn8_slot;
        let mut var_qte_s_rdn9: f64 = *var_qte_s_rdn9_slot;
        let mut var_qte_s_rv: f64 = *var_qte_s_rv_slot;
        let mut var_qtex: f64 = *var_qtex_slot;
        let mut var_qtex_db0: f64 = *var_qtex_db0_slot;
        let mut var_qtex_db1: f64 = *var_qtex_db1_slot;
        let mut var_qtex_dn0: f64 = *var_qtex_dn0_slot;
        let mut var_qtex_dn1: f64 = *var_qtex_dn1_slot;
        let mut var_qtex_dn10: f64 = *var_qtex_dn10_slot;
        let mut var_qtex_dn2: f64 = *var_qtex_dn2_slot;
        let mut var_qtex_dn3: f64 = *var_qtex_dn3_slot;
        let mut var_qtex_dn4: f64 = *var_qtex_dn4_slot;
        let mut var_qtex_dn5: f64 = *var_qtex_dn5_slot;
        let mut var_qtex_dn6: f64 = *var_qtex_dn6_slot;
        let mut var_qtex_dn7: f64 = *var_qtex_dn7_slot;
        let mut var_qtex_dn8: f64 = *var_qtex_dn8_slot;
        let mut var_qtex_dn9: f64 = *var_qtex_dn9_slot;
        let mut var_qtex_rdb0: f64 = *var_qtex_rdb0_slot;
        let mut var_qtex_rdb1: f64 = *var_qtex_rdb1_slot;
        let mut var_qtex_rdn0: f64 = *var_qtex_rdn0_slot;
        let mut var_qtex_rdn1: f64 = *var_qtex_rdn1_slot;
        let mut var_qtex_rdn10: f64 = *var_qtex_rdn10_slot;
        let mut var_qtex_rdn2: f64 = *var_qtex_rdn2_slot;
        let mut var_qtex_rdn3: f64 = *var_qtex_rdn3_slot;
        let mut var_qtex_rdn4: f64 = *var_qtex_rdn4_slot;
        let mut var_qtex_rdn5: f64 = *var_qtex_rdn5_slot;
        let mut var_qtex_rdn6: f64 = *var_qtex_rdn6_slot;
        let mut var_qtex_rdn7: f64 = *var_qtex_rdn7_slot;
        let mut var_qtex_rdn8: f64 = *var_qtex_rdn8_slot;
        let mut var_qtex_rdn9: f64 = *var_qtex_rdn9_slot;
        let mut var_qtex_rv: f64 = *var_qtex_rv_slot;
        let mut var_vjcex: f64 = *var_vjcex_slot;
        let mut var_vjcex_db0: f64 = *var_vjcex_db0_slot;
        let mut var_vjcex_db1: f64 = *var_vjcex_db1_slot;
        let mut var_vjcex_dn0: f64 = *var_vjcex_dn0_slot;
        let mut var_vjcex_dn1: f64 = *var_vjcex_dn1_slot;
        let mut var_vjcex_dn10: f64 = *var_vjcex_dn10_slot;
        let mut var_vjcex_dn2: f64 = *var_vjcex_dn2_slot;
        let mut var_vjcex_dn3: f64 = *var_vjcex_dn3_slot;
        let mut var_vjcex_dn4: f64 = *var_vjcex_dn4_slot;
        let mut var_vjcex_dn5: f64 = *var_vjcex_dn5_slot;
        let mut var_vjcex_dn6: f64 = *var_vjcex_dn6_slot;
        let mut var_vjcex_dn7: f64 = *var_vjcex_dn7_slot;
        let mut var_vjcex_dn8: f64 = *var_vjcex_dn8_slot;
        let mut var_vjcex_dn9: f64 = *var_vjcex_dn9_slot;
        let mut var_vjcex_rdb0: f64 = *var_vjcex_rdb0_slot;
        let mut var_vjcex_rdb1: f64 = *var_vjcex_rdb1_slot;
        let mut var_vjcex_rdn0: f64 = *var_vjcex_rdn0_slot;
        let mut var_vjcex_rdn1: f64 = *var_vjcex_rdn1_slot;
        let mut var_vjcex_rdn10: f64 = *var_vjcex_rdn10_slot;
        let mut var_vjcex_rdn2: f64 = *var_vjcex_rdn2_slot;
        let mut var_vjcex_rdn3: f64 = *var_vjcex_rdn3_slot;
        let mut var_vjcex_rdn4: f64 = *var_vjcex_rdn4_slot;
        let mut var_vjcex_rdn5: f64 = *var_vjcex_rdn5_slot;
        let mut var_vjcex_rdn6: f64 = *var_vjcex_rdn6_slot;
        let mut var_vjcex_rdn7: f64 = *var_vjcex_rdn7_slot;
        let mut var_vjcex_rdn8: f64 = *var_vjcex_rdn8_slot;
        let mut var_vjcex_rdn9: f64 = *var_vjcex_rdn9_slot;
        let mut var_vjcex_rv: f64 = *var_vjcex_rv_slot;
        let mut var_vtexv: f64 = *var_vtexv_slot;
        let mut var_vtexv_db0: f64 = *var_vtexv_db0_slot;
        let mut var_vtexv_db1: f64 = *var_vtexv_db1_slot;
        let mut var_vtexv_dn0: f64 = *var_vtexv_dn0_slot;
        let mut var_vtexv_dn1: f64 = *var_vtexv_dn1_slot;
        let mut var_vtexv_dn10: f64 = *var_vtexv_dn10_slot;
        let mut var_vtexv_dn2: f64 = *var_vtexv_dn2_slot;
        let mut var_vtexv_dn3: f64 = *var_vtexv_dn3_slot;
        let mut var_vtexv_dn4: f64 = *var_vtexv_dn4_slot;
        let mut var_vtexv_dn5: f64 = *var_vtexv_dn5_slot;
        let mut var_vtexv_dn6: f64 = *var_vtexv_dn6_slot;
        let mut var_vtexv_dn7: f64 = *var_vtexv_dn7_slot;
        let mut var_vtexv_dn8: f64 = *var_vtexv_dn8_slot;
        let mut var_vtexv_dn9: f64 = *var_vtexv_dn9_slot;
        let mut var_vtexv_rdb0: f64 = *var_vtexv_rdb0_slot;
        let mut var_vtexv_rdb1: f64 = *var_vtexv_rdb1_slot;
        let mut var_vtexv_rdn0: f64 = *var_vtexv_rdn0_slot;
        let mut var_vtexv_rdn1: f64 = *var_vtexv_rdn1_slot;
        let mut var_vtexv_rdn10: f64 = *var_vtexv_rdn10_slot;
        let mut var_vtexv_rdn2: f64 = *var_vtexv_rdn2_slot;
        let mut var_vtexv_rdn3: f64 = *var_vtexv_rdn3_slot;
        let mut var_vtexv_rdn4: f64 = *var_vtexv_rdn4_slot;
        let mut var_vtexv_rdn5: f64 = *var_vtexv_rdn5_slot;
        let mut var_vtexv_rdn6: f64 = *var_vtexv_rdn6_slot;
        let mut var_vtexv_rdn7: f64 = *var_vtexv_rdn7_slot;
        let mut var_vtexv_rdn8: f64 = *var_vtexv_rdn8_slot;
        let mut var_vtexv_rdn9: f64 = *var_vtexv_rdn9_slot;
        let mut var_vtexv_rv: f64 = *var_vtexv_rv_slot;

        let assign5680_e5733: f64 = (p.p67 * var_cje_t);
        let assign5680_e5737: f64 = (1.0 - p.p66);
        let assign5680_e5738: f64 = (var_vde_t / assign5680_e5737);
        let assign5680_e5743: f64 = (var_vje_s * var_inv_vde_t);
        let assign5680_e5744: f64 = (1.0 - assign5680_e5743);
        let assign5680_e5747: f64 = (1.0 - p.p66);
        let assign5680_e5748: f64 = (assign5680_e5744).powf(assign5680_e5747);
        let assign5680_e5749: f64 = (1.0 - assign5680_e5748);
        let assign5680_e5750: f64 = (assign5680_e5738 * assign5680_e5749);
        let assign5680_e5754: f64 = (var_vb1e1 - var_vje_s);
        let assign5680_e5755: f64 = (3.0 * assign5680_e5754);
        let assign5680_e5756: f64 = (assign5680_e5750 + assign5680_e5755);
        let assign5680_e5757: f64 = (assign5680_e5733 * assign5680_e5756);
        var_qte_s = assign5680_e5757;
        var_qte_s_dn0 = (((p.p67 * var_cje_t_dn0) * assign5680_e5756) + (assign5680_e5733 * ((((var_vde_t_dn0 / assign5680_e5737) * assign5680_e5749) + (assign5680_e5738 * (-if 0.0 == 0.0 && ((assign5680_e5747) as f64).is_finite() && ((assign5680_e5747) as f64).fract() == 0.0 { if assign5680_e5747 == 0.0 { 0.0 } else { (assign5680_e5747 * ((assign5680_e5744).powf(assign5680_e5747 - 1.0) * (-((var_vje_s_dn0 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn0))))) } } else { (assign5680_e5748 * (assign5680_e5747 * ((-((var_vje_s_dn0 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn0))) / assign5680_e5744))) }))) + (3.0 * (var_vb1e1_dn0 - var_vje_s_dn0)))));
        var_qte_s_dn1 = (((p.p67 * var_cje_t_dn1) * assign5680_e5756) + (assign5680_e5733 * ((((var_vde_t_dn1 / assign5680_e5737) * assign5680_e5749) + (assign5680_e5738 * (-if 0.0 == 0.0 && ((assign5680_e5747) as f64).is_finite() && ((assign5680_e5747) as f64).fract() == 0.0 { if assign5680_e5747 == 0.0 { 0.0 } else { (assign5680_e5747 * ((assign5680_e5744).powf(assign5680_e5747 - 1.0) * (-((var_vje_s_dn1 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn1))))) } } else { (assign5680_e5748 * (assign5680_e5747 * ((-((var_vje_s_dn1 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn1))) / assign5680_e5744))) }))) + (3.0 * (var_vb1e1_dn1 - var_vje_s_dn1)))));
        var_qte_s_dn2 = (((p.p67 * var_cje_t_dn2) * assign5680_e5756) + (assign5680_e5733 * ((((var_vde_t_dn2 / assign5680_e5737) * assign5680_e5749) + (assign5680_e5738 * (-if 0.0 == 0.0 && ((assign5680_e5747) as f64).is_finite() && ((assign5680_e5747) as f64).fract() == 0.0 { if assign5680_e5747 == 0.0 { 0.0 } else { (assign5680_e5747 * ((assign5680_e5744).powf(assign5680_e5747 - 1.0) * (-((var_vje_s_dn2 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn2))))) } } else { (assign5680_e5748 * (assign5680_e5747 * ((-((var_vje_s_dn2 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn2))) / assign5680_e5744))) }))) + (3.0 * (var_vb1e1_dn2 - var_vje_s_dn2)))));
        var_qte_s_dn3 = (((p.p67 * var_cje_t_dn3) * assign5680_e5756) + (assign5680_e5733 * ((((var_vde_t_dn3 / assign5680_e5737) * assign5680_e5749) + (assign5680_e5738 * (-if 0.0 == 0.0 && ((assign5680_e5747) as f64).is_finite() && ((assign5680_e5747) as f64).fract() == 0.0 { if assign5680_e5747 == 0.0 { 0.0 } else { (assign5680_e5747 * ((assign5680_e5744).powf(assign5680_e5747 - 1.0) * (-((var_vje_s_dn3 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn3))))) } } else { (assign5680_e5748 * (assign5680_e5747 * ((-((var_vje_s_dn3 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn3))) / assign5680_e5744))) }))) + (3.0 * (var_vb1e1_dn3 - var_vje_s_dn3)))));
        var_qte_s_dn4 = (((p.p67 * var_cje_t_dn4) * assign5680_e5756) + (assign5680_e5733 * ((((var_vde_t_dn4 / assign5680_e5737) * assign5680_e5749) + (assign5680_e5738 * (-if 0.0 == 0.0 && ((assign5680_e5747) as f64).is_finite() && ((assign5680_e5747) as f64).fract() == 0.0 { if assign5680_e5747 == 0.0 { 0.0 } else { (assign5680_e5747 * ((assign5680_e5744).powf(assign5680_e5747 - 1.0) * (-((var_vje_s_dn4 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn4))))) } } else { (assign5680_e5748 * (assign5680_e5747 * ((-((var_vje_s_dn4 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn4))) / assign5680_e5744))) }))) + (3.0 * (var_vb1e1_dn4 - var_vje_s_dn4)))));
        var_qte_s_dn5 = (((p.p67 * var_cje_t_dn5) * assign5680_e5756) + (assign5680_e5733 * ((((var_vde_t_dn5 / assign5680_e5737) * assign5680_e5749) + (assign5680_e5738 * (-if 0.0 == 0.0 && ((assign5680_e5747) as f64).is_finite() && ((assign5680_e5747) as f64).fract() == 0.0 { if assign5680_e5747 == 0.0 { 0.0 } else { (assign5680_e5747 * ((assign5680_e5744).powf(assign5680_e5747 - 1.0) * (-((var_vje_s_dn5 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn5))))) } } else { (assign5680_e5748 * (assign5680_e5747 * ((-((var_vje_s_dn5 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn5))) / assign5680_e5744))) }))) + (3.0 * (var_vb1e1_dn5 - var_vje_s_dn5)))));
        var_qte_s_dn6 = (((p.p67 * var_cje_t_dn6) * assign5680_e5756) + (assign5680_e5733 * ((((var_vde_t_dn6 / assign5680_e5737) * assign5680_e5749) + (assign5680_e5738 * (-if 0.0 == 0.0 && ((assign5680_e5747) as f64).is_finite() && ((assign5680_e5747) as f64).fract() == 0.0 { if assign5680_e5747 == 0.0 { 0.0 } else { (assign5680_e5747 * ((assign5680_e5744).powf(assign5680_e5747 - 1.0) * (-((var_vje_s_dn6 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn6))))) } } else { (assign5680_e5748 * (assign5680_e5747 * ((-((var_vje_s_dn6 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn6))) / assign5680_e5744))) }))) + (3.0 * (var_vb1e1_dn6 - var_vje_s_dn6)))));
        var_qte_s_dn7 = (((p.p67 * var_cje_t_dn7) * assign5680_e5756) + (assign5680_e5733 * ((((var_vde_t_dn7 / assign5680_e5737) * assign5680_e5749) + (assign5680_e5738 * (-if 0.0 == 0.0 && ((assign5680_e5747) as f64).is_finite() && ((assign5680_e5747) as f64).fract() == 0.0 { if assign5680_e5747 == 0.0 { 0.0 } else { (assign5680_e5747 * ((assign5680_e5744).powf(assign5680_e5747 - 1.0) * (-((var_vje_s_dn7 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn7))))) } } else { (assign5680_e5748 * (assign5680_e5747 * ((-((var_vje_s_dn7 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn7))) / assign5680_e5744))) }))) + (3.0 * (var_vb1e1_dn7 - var_vje_s_dn7)))));
        var_qte_s_dn8 = (((p.p67 * var_cje_t_dn8) * assign5680_e5756) + (assign5680_e5733 * ((((var_vde_t_dn8 / assign5680_e5737) * assign5680_e5749) + (assign5680_e5738 * (-if 0.0 == 0.0 && ((assign5680_e5747) as f64).is_finite() && ((assign5680_e5747) as f64).fract() == 0.0 { if assign5680_e5747 == 0.0 { 0.0 } else { (assign5680_e5747 * ((assign5680_e5744).powf(assign5680_e5747 - 1.0) * (-((var_vje_s_dn8 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn8))))) } } else { (assign5680_e5748 * (assign5680_e5747 * ((-((var_vje_s_dn8 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn8))) / assign5680_e5744))) }))) + (3.0 * (var_vb1e1_dn8 - var_vje_s_dn8)))));
        var_qte_s_dn9 = (((p.p67 * var_cje_t_dn9) * assign5680_e5756) + (assign5680_e5733 * ((((var_vde_t_dn9 / assign5680_e5737) * assign5680_e5749) + (assign5680_e5738 * (-if 0.0 == 0.0 && ((assign5680_e5747) as f64).is_finite() && ((assign5680_e5747) as f64).fract() == 0.0 { if assign5680_e5747 == 0.0 { 0.0 } else { (assign5680_e5747 * ((assign5680_e5744).powf(assign5680_e5747 - 1.0) * (-((var_vje_s_dn9 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn9))))) } } else { (assign5680_e5748 * (assign5680_e5747 * ((-((var_vje_s_dn9 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn9))) / assign5680_e5744))) }))) + (3.0 * (var_vb1e1_dn9 - var_vje_s_dn9)))));
        var_qte_s_dn10 = (((p.p67 * var_cje_t_dn10) * assign5680_e5756) + (assign5680_e5733 * ((((var_vde_t_dn10 / assign5680_e5737) * assign5680_e5749) + (assign5680_e5738 * (-if 0.0 == 0.0 && ((assign5680_e5747) as f64).is_finite() && ((assign5680_e5747) as f64).fract() == 0.0 { if assign5680_e5747 == 0.0 { 0.0 } else { (assign5680_e5747 * ((assign5680_e5744).powf(assign5680_e5747 - 1.0) * (-((var_vje_s_dn10 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn10))))) } } else { (assign5680_e5748 * (assign5680_e5747 * ((-((var_vje_s_dn10 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_dn10))) / assign5680_e5744))) }))) + (3.0 * (var_vb1e1_dn10 - var_vje_s_dn10)))));
        var_qte_s_db0 = (((p.p67 * var_cje_t_db0) * assign5680_e5756) + (assign5680_e5733 * ((((var_vde_t_db0 / assign5680_e5737) * assign5680_e5749) + (assign5680_e5738 * (-if 0.0 == 0.0 && ((assign5680_e5747) as f64).is_finite() && ((assign5680_e5747) as f64).fract() == 0.0 { if assign5680_e5747 == 0.0 { 0.0 } else { (assign5680_e5747 * ((assign5680_e5744).powf(assign5680_e5747 - 1.0) * (-((var_vje_s_db0 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_db0))))) } } else { (assign5680_e5748 * (assign5680_e5747 * ((-((var_vje_s_db0 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_db0))) / assign5680_e5744))) }))) + (3.0 * (var_vb1e1_db0 - var_vje_s_db0)))));
        var_qte_s_db1 = (((p.p67 * var_cje_t_db1) * assign5680_e5756) + (assign5680_e5733 * ((((var_vde_t_db1 / assign5680_e5737) * assign5680_e5749) + (assign5680_e5738 * (-if 0.0 == 0.0 && ((assign5680_e5747) as f64).is_finite() && ((assign5680_e5747) as f64).fract() == 0.0 { if assign5680_e5747 == 0.0 { 0.0 } else { (assign5680_e5747 * ((assign5680_e5744).powf(assign5680_e5747 - 1.0) * (-((var_vje_s_db1 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_db1))))) } } else { (assign5680_e5748 * (assign5680_e5747 * ((-((var_vje_s_db1 * var_inv_vde_t) + (var_vje_s * var_inv_vde_t_db1))) / assign5680_e5744))) }))) + (3.0 * (var_vb1e1_db1 - var_vje_s_db1)))));
        var_qte_s_rv = 0.0;
        var_qte_s_rdn0 = 0.0;
        var_qte_s_rdn1 = 0.0;
        var_qte_s_rdn2 = 0.0;
        var_qte_s_rdn3 = 0.0;
        var_qte_s_rdn4 = 0.0;
        var_qte_s_rdn5 = 0.0;
        var_qte_s_rdn6 = 0.0;
        var_qte_s_rdn7 = 0.0;
        var_qte_s_rdn8 = 0.0;
        var_qte_s_rdn9 = 0.0;
        var_qte_s_rdn10 = 0.0;
        var_qte_s_rdb0 = 0.0;
        var_qte_s_rdb1 = 0.0;

        let assign5690_e5760: f64 = (p.p76 * var_cjc_t);
        let assign5690_e5762: f64 = (assign5690_e5760 * var_vtc);
        var_qtc = assign5690_e5762;
        var_qtc_dn0 = (((p.p76 * var_cjc_t_dn0) * var_vtc) + (assign5690_e5760 * var_vtc_dn0));
        var_qtc_dn1 = (((p.p76 * var_cjc_t_dn1) * var_vtc) + (assign5690_e5760 * var_vtc_dn1));
        var_qtc_dn2 = (((p.p76 * var_cjc_t_dn2) * var_vtc) + (assign5690_e5760 * var_vtc_dn2));
        var_qtc_dn3 = (((p.p76 * var_cjc_t_dn3) * var_vtc) + (assign5690_e5760 * var_vtc_dn3));
        var_qtc_dn4 = (((p.p76 * var_cjc_t_dn4) * var_vtc) + (assign5690_e5760 * var_vtc_dn4));
        var_qtc_dn5 = (((p.p76 * var_cjc_t_dn5) * var_vtc) + (assign5690_e5760 * var_vtc_dn5));
        var_qtc_dn6 = (((p.p76 * var_cjc_t_dn6) * var_vtc) + (assign5690_e5760 * var_vtc_dn6));
        var_qtc_dn7 = (((p.p76 * var_cjc_t_dn7) * var_vtc) + (assign5690_e5760 * var_vtc_dn7));
        var_qtc_dn8 = (((p.p76 * var_cjc_t_dn8) * var_vtc) + (assign5690_e5760 * var_vtc_dn8));
        var_qtc_dn9 = (((p.p76 * var_cjc_t_dn9) * var_vtc) + (assign5690_e5760 * var_vtc_dn9));
        var_qtc_dn10 = (((p.p76 * var_cjc_t_dn10) * var_vtc) + (assign5690_e5760 * var_vtc_dn10));
        var_qtc_db0 = (((p.p76 * var_cjc_t_db0) * var_vtc) + (assign5690_e5760 * var_vtc_db0));
        var_qtc_db1 = (((p.p76 * var_cjc_t_db1) * var_vtc) + (assign5690_e5760 * var_vtc_db1));
        var_qtc_rv = 0.0;
        var_qtc_rdn0 = 0.0;
        var_qtc_rdn1 = 0.0;
        var_qtc_rdn2 = 0.0;
        var_qtc_rdn3 = 0.0;
        var_qtc_rdn4 = 0.0;
        var_qtc_rdn5 = 0.0;
        var_qtc_rdn6 = 0.0;
        var_qtc_rdn7 = 0.0;
        var_qtc_rdn8 = 0.0;
        var_qtc_rdn9 = 0.0;
        var_qtc_rdn10 = 0.0;
        var_qtc_rdb0 = 0.0;
        var_qtc_rdb1 = 0.0;

        let assign5700_e5765: f64 = (var_taub_t * var_ik_t);
        var_qb0 = assign5700_e5765;
        var_qb0_dn0 = ((var_taub_t_dn0 * var_ik_t) + (var_taub_t * var_ik_t_dn0));
        var_qb0_dn1 = ((var_taub_t_dn1 * var_ik_t) + (var_taub_t * var_ik_t_dn1));
        var_qb0_dn2 = ((var_taub_t_dn2 * var_ik_t) + (var_taub_t * var_ik_t_dn2));
        var_qb0_dn3 = ((var_taub_t_dn3 * var_ik_t) + (var_taub_t * var_ik_t_dn3));
        var_qb0_dn4 = ((var_taub_t_dn4 * var_ik_t) + (var_taub_t * var_ik_t_dn4));
        var_qb0_dn5 = ((var_taub_t_dn5 * var_ik_t) + (var_taub_t * var_ik_t_dn5));
        var_qb0_dn6 = ((var_taub_t_dn6 * var_ik_t) + (var_taub_t * var_ik_t_dn6));
        var_qb0_dn7 = ((var_taub_t_dn7 * var_ik_t) + (var_taub_t * var_ik_t_dn7));
        var_qb0_dn8 = ((var_taub_t_dn8 * var_ik_t) + (var_taub_t * var_ik_t_dn8));
        var_qb0_dn9 = ((var_taub_t_dn9 * var_ik_t) + (var_taub_t * var_ik_t_dn9));
        var_qb0_dn10 = ((var_taub_t_dn10 * var_ik_t) + (var_taub_t * var_ik_t_dn10));
        var_qb0_db0 = ((var_taub_t_db0 * var_ik_t) + (var_taub_t * var_ik_t_db0));
        var_qb0_db1 = ((var_taub_t_db1 * var_ik_t) + (var_taub_t * var_ik_t_db1));
        var_qb0_rv = 0.0;
        var_qb0_rdn0 = 0.0;
        var_qb0_rdn1 = 0.0;
        var_qb0_rdn2 = 0.0;
        var_qb0_rdn3 = 0.0;
        var_qb0_rdn4 = 0.0;
        var_qb0_rdn5 = 0.0;
        var_qb0_rdn6 = 0.0;
        var_qb0_rdn7 = 0.0;
        var_qb0_rdn8 = 0.0;
        var_qb0_rdn9 = 0.0;
        var_qb0_rdn10 = 0.0;
        var_qb0_rdb0 = 0.0;
        var_qb0_rdb1 = 0.0;

        let assign5710_e5768: f64 = (0.5 * var_qb0);
        let assign5710_e5770: f64 = (assign5710_e5768 * var_n0);
        let assign5710_e5772: f64 = (assign5710_e5770 * var_q1q);
        var_qbe_qs = assign5710_e5772;
        var_qbe_qs_dn0 = (((((0.5 * var_qb0_dn0) * var_n0) + (assign5710_e5768 * var_n0_dn0)) * var_q1q) + (assign5710_e5770 * var_q1q_dn0));
        var_qbe_qs_dn1 = (((((0.5 * var_qb0_dn1) * var_n0) + (assign5710_e5768 * var_n0_dn1)) * var_q1q) + (assign5710_e5770 * var_q1q_dn1));
        var_qbe_qs_dn2 = (((((0.5 * var_qb0_dn2) * var_n0) + (assign5710_e5768 * var_n0_dn2)) * var_q1q) + (assign5710_e5770 * var_q1q_dn2));
        var_qbe_qs_dn3 = (((((0.5 * var_qb0_dn3) * var_n0) + (assign5710_e5768 * var_n0_dn3)) * var_q1q) + (assign5710_e5770 * var_q1q_dn3));
        var_qbe_qs_dn4 = (((((0.5 * var_qb0_dn4) * var_n0) + (assign5710_e5768 * var_n0_dn4)) * var_q1q) + (assign5710_e5770 * var_q1q_dn4));
        var_qbe_qs_dn5 = (((((0.5 * var_qb0_dn5) * var_n0) + (assign5710_e5768 * var_n0_dn5)) * var_q1q) + (assign5710_e5770 * var_q1q_dn5));
        var_qbe_qs_dn6 = (((((0.5 * var_qb0_dn6) * var_n0) + (assign5710_e5768 * var_n0_dn6)) * var_q1q) + (assign5710_e5770 * var_q1q_dn6));
        var_qbe_qs_dn7 = (((((0.5 * var_qb0_dn7) * var_n0) + (assign5710_e5768 * var_n0_dn7)) * var_q1q) + (assign5710_e5770 * var_q1q_dn7));
        var_qbe_qs_dn8 = (((((0.5 * var_qb0_dn8) * var_n0) + (assign5710_e5768 * var_n0_dn8)) * var_q1q) + (assign5710_e5770 * var_q1q_dn8));
        var_qbe_qs_dn9 = (((((0.5 * var_qb0_dn9) * var_n0) + (assign5710_e5768 * var_n0_dn9)) * var_q1q) + (assign5710_e5770 * var_q1q_dn9));
        var_qbe_qs_dn10 = (((((0.5 * var_qb0_dn10) * var_n0) + (assign5710_e5768 * var_n0_dn10)) * var_q1q) + (assign5710_e5770 * var_q1q_dn10));
        var_qbe_qs_db0 = (((((0.5 * var_qb0_db0) * var_n0) + (assign5710_e5768 * var_n0_db0)) * var_q1q) + (assign5710_e5770 * var_q1q_db0));
        var_qbe_qs_db1 = (((((0.5 * var_qb0_db1) * var_n0) + (assign5710_e5768 * var_n0_db1)) * var_q1q) + (assign5710_e5770 * var_q1q_db1));
        var_qbe_qs_rv = 0.0;
        var_qbe_qs_rdn0 = 0.0;
        var_qbe_qs_rdn1 = 0.0;
        var_qbe_qs_rdn2 = 0.0;
        var_qbe_qs_rdn3 = 0.0;
        var_qbe_qs_rdn4 = 0.0;
        var_qbe_qs_rdn5 = 0.0;
        var_qbe_qs_rdn6 = 0.0;
        var_qbe_qs_rdn7 = 0.0;
        var_qbe_qs_rdn8 = 0.0;
        var_qbe_qs_rdn9 = 0.0;
        var_qbe_qs_rdn10 = 0.0;
        var_qbe_qs_rdb0 = 0.0;
        var_qbe_qs_rdb1 = 0.0;

        let assign5720_e5775: f64 = (0.5 * var_qb0);
        let assign5720_e5777: f64 = (assign5720_e5775 * var_nb);
        let assign5720_e5779: f64 = (assign5720_e5777 * var_q1q);
        var_qbc_qs = assign5720_e5779;
        var_qbc_qs_dn0 = (((((0.5 * var_qb0_dn0) * var_nb) + (assign5720_e5775 * var_nb_dn0)) * var_q1q) + (assign5720_e5777 * var_q1q_dn0));
        var_qbc_qs_dn1 = (((((0.5 * var_qb0_dn1) * var_nb) + (assign5720_e5775 * var_nb_dn1)) * var_q1q) + (assign5720_e5777 * var_q1q_dn1));
        var_qbc_qs_dn2 = (((((0.5 * var_qb0_dn2) * var_nb) + (assign5720_e5775 * var_nb_dn2)) * var_q1q) + (assign5720_e5777 * var_q1q_dn2));
        var_qbc_qs_dn3 = (((((0.5 * var_qb0_dn3) * var_nb) + (assign5720_e5775 * var_nb_dn3)) * var_q1q) + (assign5720_e5777 * var_q1q_dn3));
        var_qbc_qs_dn4 = (((((0.5 * var_qb0_dn4) * var_nb) + (assign5720_e5775 * var_nb_dn4)) * var_q1q) + (assign5720_e5777 * var_q1q_dn4));
        var_qbc_qs_dn5 = (((((0.5 * var_qb0_dn5) * var_nb) + (assign5720_e5775 * var_nb_dn5)) * var_q1q) + (assign5720_e5777 * var_q1q_dn5));
        var_qbc_qs_dn6 = (((((0.5 * var_qb0_dn6) * var_nb) + (assign5720_e5775 * var_nb_dn6)) * var_q1q) + (assign5720_e5777 * var_q1q_dn6));
        var_qbc_qs_dn7 = (((((0.5 * var_qb0_dn7) * var_nb) + (assign5720_e5775 * var_nb_dn7)) * var_q1q) + (assign5720_e5777 * var_q1q_dn7));
        var_qbc_qs_dn8 = (((((0.5 * var_qb0_dn8) * var_nb) + (assign5720_e5775 * var_nb_dn8)) * var_q1q) + (assign5720_e5777 * var_q1q_dn8));
        var_qbc_qs_dn9 = (((((0.5 * var_qb0_dn9) * var_nb) + (assign5720_e5775 * var_nb_dn9)) * var_q1q) + (assign5720_e5777 * var_q1q_dn9));
        var_qbc_qs_dn10 = (((((0.5 * var_qb0_dn10) * var_nb) + (assign5720_e5775 * var_nb_dn10)) * var_q1q) + (assign5720_e5777 * var_q1q_dn10));
        var_qbc_qs_db0 = (((((0.5 * var_qb0_db0) * var_nb) + (assign5720_e5775 * var_nb_db0)) * var_q1q) + (assign5720_e5777 * var_q1q_db0));
        var_qbc_qs_db1 = (((((0.5 * var_qb0_db1) * var_nb) + (assign5720_e5775 * var_nb_db1)) * var_q1q) + (assign5720_e5777 * var_q1q_db1));
        var_qbc_qs_rv = 0.0;
        var_qbc_qs_rdn0 = 0.0;
        var_qbc_qs_rdn1 = 0.0;
        var_qbc_qs_rdn2 = 0.0;
        var_qbc_qs_rdn3 = 0.0;
        var_qbc_qs_rdn4 = 0.0;
        var_qbc_qs_rdn5 = 0.0;
        var_qbc_qs_rdn6 = 0.0;
        var_qbc_qs_rdn7 = 0.0;
        var_qbc_qs_rdn8 = 0.0;
        var_qbc_qs_rdn9 = 0.0;
        var_qbc_qs_rdn10 = 0.0;
        var_qbc_qs_rdb0 = 0.0;
        var_qbc_qs_rdb1 = 0.0;

        let assign5730_e5782: f64 = (0.1 * var_vdc_ctc_t);
        var_a_vdcctc = assign5730_e5782;
        var_a_vdcctc_dn0 = (0.1 * var_vdc_ctc_t_dn0);
        var_a_vdcctc_dn1 = (0.1 * var_vdc_ctc_t_dn1);
        var_a_vdcctc_dn2 = (0.1 * var_vdc_ctc_t_dn2);
        var_a_vdcctc_dn3 = (0.1 * var_vdc_ctc_t_dn3);
        var_a_vdcctc_dn4 = (0.1 * var_vdc_ctc_t_dn4);
        var_a_vdcctc_dn5 = (0.1 * var_vdc_ctc_t_dn5);
        var_a_vdcctc_dn6 = (0.1 * var_vdc_ctc_t_dn6);
        var_a_vdcctc_dn7 = (0.1 * var_vdc_ctc_t_dn7);
        var_a_vdcctc_dn8 = (0.1 * var_vdc_ctc_t_dn8);
        var_a_vdcctc_dn9 = (0.1 * var_vdc_ctc_t_dn9);
        var_a_vdcctc_dn10 = (0.1 * var_vdc_ctc_t_dn10);
        var_a_vdcctc_db0 = (0.1 * var_vdc_ctc_t_db0);
        var_a_vdcctc_db1 = (0.1 * var_vdc_ctc_t_db1);
        var_a_vdcctc_rv = 0.0;
        var_a_vdcctc_rdn0 = 0.0;
        var_a_vdcctc_rdn1 = 0.0;
        var_a_vdcctc_rdn2 = 0.0;
        var_a_vdcctc_rdn3 = 0.0;
        var_a_vdcctc_rdn4 = 0.0;
        var_a_vdcctc_rdn5 = 0.0;
        var_a_vdcctc_rdn6 = 0.0;
        var_a_vdcctc_rdn7 = 0.0;
        var_a_vdcctc_rdn8 = 0.0;
        var_a_vdcctc_rdn9 = 0.0;
        var_a_vdcctc_rdn10 = 0.0;
        var_a_vdcctc_rdb0 = 0.0;
        var_a_vdcctc_rdb1 = 0.0;

        let assign5740_e5785: f64 = (var_vb1c4 - var_vfc);
        let assign5740_e5787: f64 = (assign5740_e5785 / var_a_vdcctc);
        var_dxa = assign5740_e5787;
        var_dxa_dn0 = ((((var_vb1c4_dn0 - var_vfc_dn0) * var_a_vdcctc) - (assign5740_e5785 * var_a_vdcctc_dn0)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn1 = ((((var_vb1c4_dn1 - var_vfc_dn1) * var_a_vdcctc) - (assign5740_e5785 * var_a_vdcctc_dn1)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn2 = ((((var_vb1c4_dn2 - var_vfc_dn2) * var_a_vdcctc) - (assign5740_e5785 * var_a_vdcctc_dn2)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn3 = ((((var_vb1c4_dn3 - var_vfc_dn3) * var_a_vdcctc) - (assign5740_e5785 * var_a_vdcctc_dn3)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn4 = ((((var_vb1c4_dn4 - var_vfc_dn4) * var_a_vdcctc) - (assign5740_e5785 * var_a_vdcctc_dn4)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn5 = ((((var_vb1c4_dn5 - var_vfc_dn5) * var_a_vdcctc) - (assign5740_e5785 * var_a_vdcctc_dn5)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn6 = ((((var_vb1c4_dn6 - var_vfc_dn6) * var_a_vdcctc) - (assign5740_e5785 * var_a_vdcctc_dn6)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn7 = ((((var_vb1c4_dn7 - var_vfc_dn7) * var_a_vdcctc) - (assign5740_e5785 * var_a_vdcctc_dn7)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn8 = ((((var_vb1c4_dn8 - var_vfc_dn8) * var_a_vdcctc) - (assign5740_e5785 * var_a_vdcctc_dn8)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn9 = ((((var_vb1c4_dn9 - var_vfc_dn9) * var_a_vdcctc) - (assign5740_e5785 * var_a_vdcctc_dn9)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn10 = ((((var_vb1c4_dn10 - var_vfc_dn10) * var_a_vdcctc) - (assign5740_e5785 * var_a_vdcctc_dn10)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_db0 = ((((var_vb1c4_db0 - var_vfc_db0) * var_a_vdcctc) - (assign5740_e5785 * var_a_vdcctc_db0)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_db1 = ((((var_vb1c4_db1 - var_vfc_db1) * var_a_vdcctc) - (assign5740_e5785 * var_a_vdcctc_db1)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_rv = 0.0;
        var_dxa_rdn0 = 0.0;
        var_dxa_rdn1 = 0.0;
        var_dxa_rdn2 = 0.0;
        var_dxa_rdn3 = 0.0;
        var_dxa_rdn4 = 0.0;
        var_dxa_rdn5 = 0.0;
        var_dxa_rdn6 = 0.0;
        var_dxa_rdn7 = 0.0;
        var_dxa_rdn8 = 0.0;
        var_dxa_rdn9 = 0.0;
        var_dxa_rdn10 = 0.0;
        var_dxa_rdb0 = 0.0;
        var_dxa_rdb1 = 0.0;

        let assign5750_e5790: f64 = if var_vb1c4 < var_vfc { 1.0 } else { 0.0 };
        var_guard104 = assign5750_e5790;
        var_guard104_dn0 = 0.0;
        var_guard104_dn1 = 0.0;
        var_guard104_dn2 = 0.0;
        var_guard104_dn3 = 0.0;
        var_guard104_dn4 = 0.0;
        var_guard104_dn5 = 0.0;
        var_guard104_dn6 = 0.0;
        var_guard104_dn7 = 0.0;
        var_guard104_dn8 = 0.0;
        var_guard104_dn9 = 0.0;
        var_guard104_dn10 = 0.0;
        var_guard104_db0 = 0.0;
        var_guard104_db1 = 0.0;
        var_guard104_rv = 0.0;
        var_guard104_rdn0 = 0.0;
        var_guard104_rdn1 = 0.0;
        var_guard104_rdn2 = 0.0;
        var_guard104_rdn3 = 0.0;
        var_guard104_rdn4 = 0.0;
        var_guard104_rdn5 = 0.0;
        var_guard104_rdn6 = 0.0;
        var_guard104_rdn7 = 0.0;
        var_guard104_rdn8 = 0.0;
        var_guard104_rdn9 = 0.0;
        var_guard104_rdn10 = 0.0;
        var_guard104_rdb0 = 0.0;
        var_guard104_rdb1 = 0.0;

        let (assign5760_e5802, assign5760_e5802_d_n0, assign5760_e5802_d_n1, assign5760_e5802_d_n2, assign5760_e5802_d_n3, assign5760_e5802_d_n4, assign5760_e5802_d_n5, assign5760_e5802_d_n6, assign5760_e5802_d_n7, assign5760_e5802_d_n8, assign5760_e5802_d_n9, assign5760_e5802_d_n10, assign5760_e5802_d_b0, assign5760_e5802_d_b1,) = {
    if (var_guard104 != 0.0) {
        let assign5760_e5796: f64 = (var_dxa).exp();
        let assign5760_e5797: f64 = (1.0 + assign5760_e5796);
        let assign5760_e5798: f64 = (assign5760_e5797).ln();
        let assign5760_e5799: f64 = (var_a_vdcctc * assign5760_e5798);
        let assign5760_e5800: f64 = (var_vb1c4 - assign5760_e5799);
        (assign5760_e5800, (var_vb1c4_dn0 - ((var_a_vdcctc_dn0 * assign5760_e5798) + (var_a_vdcctc * ((assign5760_e5796 * var_dxa_dn0) / assign5760_e5797)))), (var_vb1c4_dn1 - ((var_a_vdcctc_dn1 * assign5760_e5798) + (var_a_vdcctc * ((assign5760_e5796 * var_dxa_dn1) / assign5760_e5797)))), (var_vb1c4_dn2 - ((var_a_vdcctc_dn2 * assign5760_e5798) + (var_a_vdcctc * ((assign5760_e5796 * var_dxa_dn2) / assign5760_e5797)))), (var_vb1c4_dn3 - ((var_a_vdcctc_dn3 * assign5760_e5798) + (var_a_vdcctc * ((assign5760_e5796 * var_dxa_dn3) / assign5760_e5797)))), (var_vb1c4_dn4 - ((var_a_vdcctc_dn4 * assign5760_e5798) + (var_a_vdcctc * ((assign5760_e5796 * var_dxa_dn4) / assign5760_e5797)))), (var_vb1c4_dn5 - ((var_a_vdcctc_dn5 * assign5760_e5798) + (var_a_vdcctc * ((assign5760_e5796 * var_dxa_dn5) / assign5760_e5797)))), (var_vb1c4_dn6 - ((var_a_vdcctc_dn6 * assign5760_e5798) + (var_a_vdcctc * ((assign5760_e5796 * var_dxa_dn6) / assign5760_e5797)))), (var_vb1c4_dn7 - ((var_a_vdcctc_dn7 * assign5760_e5798) + (var_a_vdcctc * ((assign5760_e5796 * var_dxa_dn7) / assign5760_e5797)))), (var_vb1c4_dn8 - ((var_a_vdcctc_dn8 * assign5760_e5798) + (var_a_vdcctc * ((assign5760_e5796 * var_dxa_dn8) / assign5760_e5797)))), (var_vb1c4_dn9 - ((var_a_vdcctc_dn9 * assign5760_e5798) + (var_a_vdcctc * ((assign5760_e5796 * var_dxa_dn9) / assign5760_e5797)))), (var_vb1c4_dn10 - ((var_a_vdcctc_dn10 * assign5760_e5798) + (var_a_vdcctc * ((assign5760_e5796 * var_dxa_dn10) / assign5760_e5797)))), (var_vb1c4_db0 - ((var_a_vdcctc_db0 * assign5760_e5798) + (var_a_vdcctc * ((assign5760_e5796 * var_dxa_db0) / assign5760_e5797)))), (var_vb1c4_db1 - ((var_a_vdcctc_db1 * assign5760_e5798) + (var_a_vdcctc * ((assign5760_e5796 * var_dxa_db1) / assign5760_e5797)))),)
    } else {
        (var_vjcex, var_vjcex_dn0, var_vjcex_dn1, var_vjcex_dn2, var_vjcex_dn3, var_vjcex_dn4, var_vjcex_dn5, var_vjcex_dn6, var_vjcex_dn7, var_vjcex_dn8, var_vjcex_dn9, var_vjcex_dn10, var_vjcex_db0, var_vjcex_db1,)
    }
};
        var_vjcex = assign5760_e5802;
        var_vjcex_dn0 = assign5760_e5802_d_n0;
        var_vjcex_dn1 = assign5760_e5802_d_n1;
        var_vjcex_dn2 = assign5760_e5802_d_n2;
        var_vjcex_dn3 = assign5760_e5802_d_n3;
        var_vjcex_dn4 = assign5760_e5802_d_n4;
        var_vjcex_dn5 = assign5760_e5802_d_n5;
        var_vjcex_dn6 = assign5760_e5802_d_n6;
        var_vjcex_dn7 = assign5760_e5802_d_n7;
        var_vjcex_dn8 = assign5760_e5802_d_n8;
        var_vjcex_dn9 = assign5760_e5802_d_n9;
        var_vjcex_dn10 = assign5760_e5802_d_n10;
        var_vjcex_db0 = assign5760_e5802_d_b0;
        var_vjcex_db1 = assign5760_e5802_d_b1;
        var_vjcex_rv = 0.0;
        var_vjcex_rdn0 = 0.0;
        var_vjcex_rdn1 = 0.0;
        var_vjcex_rdn2 = 0.0;
        var_vjcex_rdn3 = 0.0;
        var_vjcex_rdn4 = 0.0;
        var_vjcex_rdn5 = 0.0;
        var_vjcex_rdn6 = 0.0;
        var_vjcex_rdn7 = 0.0;
        var_vjcex_rdn8 = 0.0;
        var_vjcex_rdn9 = 0.0;
        var_vjcex_rdn10 = 0.0;
        var_vjcex_rdb0 = 0.0;
        var_vjcex_rdb1 = 0.0;

        let (assign5770_e5816, assign5770_e5816_d_n0, assign5770_e5816_d_n1, assign5770_e5816_d_n2, assign5770_e5816_d_n3, assign5770_e5816_d_n4, assign5770_e5816_d_n5, assign5770_e5816_d_n6, assign5770_e5816_d_n7, assign5770_e5816_d_n8, assign5770_e5816_d_n9, assign5770_e5816_d_n10, assign5770_e5816_d_b0, assign5770_e5816_d_b1,) = {
    if (var_guard104 == 0.0) {
        let assign5770_e5809: f64 = (-var_dxa);
        let assign5770_e5810: f64 = (assign5770_e5809).exp();
        let assign5770_e5811: f64 = (1.0 + assign5770_e5810);
        let assign5770_e5812: f64 = (assign5770_e5811).ln();
        let assign5770_e5813: f64 = (var_a_vdcctc * assign5770_e5812);
        let assign5770_e5814: f64 = (var_vfc - assign5770_e5813);
        (assign5770_e5814, (var_vfc_dn0 - ((var_a_vdcctc_dn0 * assign5770_e5812) + (var_a_vdcctc * ((assign5770_e5810 * (-var_dxa_dn0)) / assign5770_e5811)))), (var_vfc_dn1 - ((var_a_vdcctc_dn1 * assign5770_e5812) + (var_a_vdcctc * ((assign5770_e5810 * (-var_dxa_dn1)) / assign5770_e5811)))), (var_vfc_dn2 - ((var_a_vdcctc_dn2 * assign5770_e5812) + (var_a_vdcctc * ((assign5770_e5810 * (-var_dxa_dn2)) / assign5770_e5811)))), (var_vfc_dn3 - ((var_a_vdcctc_dn3 * assign5770_e5812) + (var_a_vdcctc * ((assign5770_e5810 * (-var_dxa_dn3)) / assign5770_e5811)))), (var_vfc_dn4 - ((var_a_vdcctc_dn4 * assign5770_e5812) + (var_a_vdcctc * ((assign5770_e5810 * (-var_dxa_dn4)) / assign5770_e5811)))), (var_vfc_dn5 - ((var_a_vdcctc_dn5 * assign5770_e5812) + (var_a_vdcctc * ((assign5770_e5810 * (-var_dxa_dn5)) / assign5770_e5811)))), (var_vfc_dn6 - ((var_a_vdcctc_dn6 * assign5770_e5812) + (var_a_vdcctc * ((assign5770_e5810 * (-var_dxa_dn6)) / assign5770_e5811)))), (var_vfc_dn7 - ((var_a_vdcctc_dn7 * assign5770_e5812) + (var_a_vdcctc * ((assign5770_e5810 * (-var_dxa_dn7)) / assign5770_e5811)))), (var_vfc_dn8 - ((var_a_vdcctc_dn8 * assign5770_e5812) + (var_a_vdcctc * ((assign5770_e5810 * (-var_dxa_dn8)) / assign5770_e5811)))), (var_vfc_dn9 - ((var_a_vdcctc_dn9 * assign5770_e5812) + (var_a_vdcctc * ((assign5770_e5810 * (-var_dxa_dn9)) / assign5770_e5811)))), (var_vfc_dn10 - ((var_a_vdcctc_dn10 * assign5770_e5812) + (var_a_vdcctc * ((assign5770_e5810 * (-var_dxa_dn10)) / assign5770_e5811)))), (var_vfc_db0 - ((var_a_vdcctc_db0 * assign5770_e5812) + (var_a_vdcctc * ((assign5770_e5810 * (-var_dxa_db0)) / assign5770_e5811)))), (var_vfc_db1 - ((var_a_vdcctc_db1 * assign5770_e5812) + (var_a_vdcctc * ((assign5770_e5810 * (-var_dxa_db1)) / assign5770_e5811)))),)
    } else {
        (var_vjcex, var_vjcex_dn0, var_vjcex_dn1, var_vjcex_dn2, var_vjcex_dn3, var_vjcex_dn4, var_vjcex_dn5, var_vjcex_dn6, var_vjcex_dn7, var_vjcex_dn8, var_vjcex_dn9, var_vjcex_dn10, var_vjcex_db0, var_vjcex_db1,)
    }
};
        var_vjcex = assign5770_e5816;
        var_vjcex_dn0 = assign5770_e5816_d_n0;
        var_vjcex_dn1 = assign5770_e5816_d_n1;
        var_vjcex_dn2 = assign5770_e5816_d_n2;
        var_vjcex_dn3 = assign5770_e5816_d_n3;
        var_vjcex_dn4 = assign5770_e5816_d_n4;
        var_vjcex_dn5 = assign5770_e5816_d_n5;
        var_vjcex_dn6 = assign5770_e5816_d_n6;
        var_vjcex_dn7 = assign5770_e5816_d_n7;
        var_vjcex_dn8 = assign5770_e5816_d_n8;
        var_vjcex_dn9 = assign5770_e5816_d_n9;
        var_vjcex_dn10 = assign5770_e5816_d_n10;
        var_vjcex_db0 = assign5770_e5816_d_b0;
        var_vjcex_db1 = assign5770_e5816_d_b1;
        var_vjcex_rv = 0.0;
        var_vjcex_rdn0 = 0.0;
        var_vjcex_rdn1 = 0.0;
        var_vjcex_rdn2 = 0.0;
        var_vjcex_rdn3 = 0.0;
        var_vjcex_rdn4 = 0.0;
        var_vjcex_rdn5 = 0.0;
        var_vjcex_rdn6 = 0.0;
        var_vjcex_rdn7 = 0.0;
        var_vjcex_rdn8 = 0.0;
        var_vjcex_rdn9 = 0.0;
        var_vjcex_rdn10 = 0.0;
        var_vjcex_rdb0 = 0.0;
        var_vjcex_rdb1 = 0.0;

        let assign5780_e5820: f64 = (1.0 - p.p71);
        let assign5780_e5821: f64 = (var_vdc_ctc_t / assign5780_e5820);
        let assign5780_e5826: f64 = (var_vjcex / var_vdc_ctc_t);
        let assign5780_e5827: f64 = (1.0 - assign5780_e5826);
        let assign5780_e5830: f64 = (1.0 - p.p71);
        let assign5780_e5831: f64 = (assign5780_e5827).powf(assign5780_e5830);
        let assign5780_e5832: f64 = (1.0 - assign5780_e5831);
        let assign5780_e5833: f64 = (assign5780_e5821 * assign5780_e5832);
        let assign5780_e5837: f64 = (var_vb1c4 - var_vjcex);
        let assign5780_e5838: f64 = (var_bjc * assign5780_e5837);
        let assign5780_e5839: f64 = (assign5780_e5833 + assign5780_e5838);
        var_vtexv = assign5780_e5839;
        var_vtexv_dn0 = ((((var_vdc_ctc_t_dn0 / assign5780_e5820) * assign5780_e5832) + (assign5780_e5821 * (-if 0.0 == 0.0 && ((assign5780_e5830) as f64).is_finite() && ((assign5780_e5830) as f64).fract() == 0.0 { if assign5780_e5830 == 0.0 { 0.0 } else { (assign5780_e5830 * ((assign5780_e5827).powf(assign5780_e5830 - 1.0) * (-(((var_vjcex_dn0 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn0)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5780_e5831 * (assign5780_e5830 * ((-(((var_vjcex_dn0 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn0)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5780_e5827))) }))) + ((var_bjc_dn0 * assign5780_e5837) + (var_bjc * (var_vb1c4_dn0 - var_vjcex_dn0))));
        var_vtexv_dn1 = ((((var_vdc_ctc_t_dn1 / assign5780_e5820) * assign5780_e5832) + (assign5780_e5821 * (-if 0.0 == 0.0 && ((assign5780_e5830) as f64).is_finite() && ((assign5780_e5830) as f64).fract() == 0.0 { if assign5780_e5830 == 0.0 { 0.0 } else { (assign5780_e5830 * ((assign5780_e5827).powf(assign5780_e5830 - 1.0) * (-(((var_vjcex_dn1 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn1)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5780_e5831 * (assign5780_e5830 * ((-(((var_vjcex_dn1 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn1)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5780_e5827))) }))) + ((var_bjc_dn1 * assign5780_e5837) + (var_bjc * (var_vb1c4_dn1 - var_vjcex_dn1))));
        var_vtexv_dn2 = ((((var_vdc_ctc_t_dn2 / assign5780_e5820) * assign5780_e5832) + (assign5780_e5821 * (-if 0.0 == 0.0 && ((assign5780_e5830) as f64).is_finite() && ((assign5780_e5830) as f64).fract() == 0.0 { if assign5780_e5830 == 0.0 { 0.0 } else { (assign5780_e5830 * ((assign5780_e5827).powf(assign5780_e5830 - 1.0) * (-(((var_vjcex_dn2 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn2)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5780_e5831 * (assign5780_e5830 * ((-(((var_vjcex_dn2 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn2)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5780_e5827))) }))) + ((var_bjc_dn2 * assign5780_e5837) + (var_bjc * (var_vb1c4_dn2 - var_vjcex_dn2))));
        var_vtexv_dn3 = ((((var_vdc_ctc_t_dn3 / assign5780_e5820) * assign5780_e5832) + (assign5780_e5821 * (-if 0.0 == 0.0 && ((assign5780_e5830) as f64).is_finite() && ((assign5780_e5830) as f64).fract() == 0.0 { if assign5780_e5830 == 0.0 { 0.0 } else { (assign5780_e5830 * ((assign5780_e5827).powf(assign5780_e5830 - 1.0) * (-(((var_vjcex_dn3 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn3)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5780_e5831 * (assign5780_e5830 * ((-(((var_vjcex_dn3 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn3)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5780_e5827))) }))) + ((var_bjc_dn3 * assign5780_e5837) + (var_bjc * (var_vb1c4_dn3 - var_vjcex_dn3))));
        var_vtexv_dn4 = ((((var_vdc_ctc_t_dn4 / assign5780_e5820) * assign5780_e5832) + (assign5780_e5821 * (-if 0.0 == 0.0 && ((assign5780_e5830) as f64).is_finite() && ((assign5780_e5830) as f64).fract() == 0.0 { if assign5780_e5830 == 0.0 { 0.0 } else { (assign5780_e5830 * ((assign5780_e5827).powf(assign5780_e5830 - 1.0) * (-(((var_vjcex_dn4 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn4)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5780_e5831 * (assign5780_e5830 * ((-(((var_vjcex_dn4 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn4)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5780_e5827))) }))) + ((var_bjc_dn4 * assign5780_e5837) + (var_bjc * (var_vb1c4_dn4 - var_vjcex_dn4))));
        var_vtexv_dn5 = ((((var_vdc_ctc_t_dn5 / assign5780_e5820) * assign5780_e5832) + (assign5780_e5821 * (-if 0.0 == 0.0 && ((assign5780_e5830) as f64).is_finite() && ((assign5780_e5830) as f64).fract() == 0.0 { if assign5780_e5830 == 0.0 { 0.0 } else { (assign5780_e5830 * ((assign5780_e5827).powf(assign5780_e5830 - 1.0) * (-(((var_vjcex_dn5 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn5)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5780_e5831 * (assign5780_e5830 * ((-(((var_vjcex_dn5 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn5)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5780_e5827))) }))) + ((var_bjc_dn5 * assign5780_e5837) + (var_bjc * (var_vb1c4_dn5 - var_vjcex_dn5))));
        var_vtexv_dn6 = ((((var_vdc_ctc_t_dn6 / assign5780_e5820) * assign5780_e5832) + (assign5780_e5821 * (-if 0.0 == 0.0 && ((assign5780_e5830) as f64).is_finite() && ((assign5780_e5830) as f64).fract() == 0.0 { if assign5780_e5830 == 0.0 { 0.0 } else { (assign5780_e5830 * ((assign5780_e5827).powf(assign5780_e5830 - 1.0) * (-(((var_vjcex_dn6 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn6)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5780_e5831 * (assign5780_e5830 * ((-(((var_vjcex_dn6 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn6)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5780_e5827))) }))) + ((var_bjc_dn6 * assign5780_e5837) + (var_bjc * (var_vb1c4_dn6 - var_vjcex_dn6))));
        var_vtexv_dn7 = ((((var_vdc_ctc_t_dn7 / assign5780_e5820) * assign5780_e5832) + (assign5780_e5821 * (-if 0.0 == 0.0 && ((assign5780_e5830) as f64).is_finite() && ((assign5780_e5830) as f64).fract() == 0.0 { if assign5780_e5830 == 0.0 { 0.0 } else { (assign5780_e5830 * ((assign5780_e5827).powf(assign5780_e5830 - 1.0) * (-(((var_vjcex_dn7 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn7)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5780_e5831 * (assign5780_e5830 * ((-(((var_vjcex_dn7 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn7)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5780_e5827))) }))) + ((var_bjc_dn7 * assign5780_e5837) + (var_bjc * (var_vb1c4_dn7 - var_vjcex_dn7))));
        var_vtexv_dn8 = ((((var_vdc_ctc_t_dn8 / assign5780_e5820) * assign5780_e5832) + (assign5780_e5821 * (-if 0.0 == 0.0 && ((assign5780_e5830) as f64).is_finite() && ((assign5780_e5830) as f64).fract() == 0.0 { if assign5780_e5830 == 0.0 { 0.0 } else { (assign5780_e5830 * ((assign5780_e5827).powf(assign5780_e5830 - 1.0) * (-(((var_vjcex_dn8 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn8)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5780_e5831 * (assign5780_e5830 * ((-(((var_vjcex_dn8 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn8)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5780_e5827))) }))) + ((var_bjc_dn8 * assign5780_e5837) + (var_bjc * (var_vb1c4_dn8 - var_vjcex_dn8))));
        var_vtexv_dn9 = ((((var_vdc_ctc_t_dn9 / assign5780_e5820) * assign5780_e5832) + (assign5780_e5821 * (-if 0.0 == 0.0 && ((assign5780_e5830) as f64).is_finite() && ((assign5780_e5830) as f64).fract() == 0.0 { if assign5780_e5830 == 0.0 { 0.0 } else { (assign5780_e5830 * ((assign5780_e5827).powf(assign5780_e5830 - 1.0) * (-(((var_vjcex_dn9 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn9)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5780_e5831 * (assign5780_e5830 * ((-(((var_vjcex_dn9 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn9)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5780_e5827))) }))) + ((var_bjc_dn9 * assign5780_e5837) + (var_bjc * (var_vb1c4_dn9 - var_vjcex_dn9))));
        var_vtexv_dn10 = ((((var_vdc_ctc_t_dn10 / assign5780_e5820) * assign5780_e5832) + (assign5780_e5821 * (-if 0.0 == 0.0 && ((assign5780_e5830) as f64).is_finite() && ((assign5780_e5830) as f64).fract() == 0.0 { if assign5780_e5830 == 0.0 { 0.0 } else { (assign5780_e5830 * ((assign5780_e5827).powf(assign5780_e5830 - 1.0) * (-(((var_vjcex_dn10 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn10)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5780_e5831 * (assign5780_e5830 * ((-(((var_vjcex_dn10 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_dn10)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5780_e5827))) }))) + ((var_bjc_dn10 * assign5780_e5837) + (var_bjc * (var_vb1c4_dn10 - var_vjcex_dn10))));
        var_vtexv_db0 = ((((var_vdc_ctc_t_db0 / assign5780_e5820) * assign5780_e5832) + (assign5780_e5821 * (-if 0.0 == 0.0 && ((assign5780_e5830) as f64).is_finite() && ((assign5780_e5830) as f64).fract() == 0.0 { if assign5780_e5830 == 0.0 { 0.0 } else { (assign5780_e5830 * ((assign5780_e5827).powf(assign5780_e5830 - 1.0) * (-(((var_vjcex_db0 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_db0)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5780_e5831 * (assign5780_e5830 * ((-(((var_vjcex_db0 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_db0)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5780_e5827))) }))) + ((var_bjc_db0 * assign5780_e5837) + (var_bjc * (var_vb1c4_db0 - var_vjcex_db0))));
        var_vtexv_db1 = ((((var_vdc_ctc_t_db1 / assign5780_e5820) * assign5780_e5832) + (assign5780_e5821 * (-if 0.0 == 0.0 && ((assign5780_e5830) as f64).is_finite() && ((assign5780_e5830) as f64).fract() == 0.0 { if assign5780_e5830 == 0.0 { 0.0 } else { (assign5780_e5830 * ((assign5780_e5827).powf(assign5780_e5830 - 1.0) * (-(((var_vjcex_db1 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_db1)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5780_e5831 * (assign5780_e5830 * ((-(((var_vjcex_db1 * var_vdc_ctc_t) - (var_vjcex * var_vdc_ctc_t_db1)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5780_e5827))) }))) + ((var_bjc_db1 * assign5780_e5837) + (var_bjc * (var_vb1c4_db1 - var_vjcex_db1))));
        var_vtexv_rv = 0.0;
        var_vtexv_rdn0 = 0.0;
        var_vtexv_rdn1 = 0.0;
        var_vtexv_rdn2 = 0.0;
        var_vtexv_rdn3 = 0.0;
        var_vtexv_rdn4 = 0.0;
        var_vtexv_rdn5 = 0.0;
        var_vtexv_rdn6 = 0.0;
        var_vtexv_rdn7 = 0.0;
        var_vtexv_rdn8 = 0.0;
        var_vtexv_rdn9 = 0.0;
        var_vtexv_rdn10 = 0.0;
        var_vtexv_rdb0 = 0.0;
        var_vtexv_rdb1 = 0.0;

        let assign5790_e5843: f64 = (1.0 - var_xp_t);
        let assign5790_e5845: f64 = (assign5790_e5843 * var_vtexv);
        let assign5790_e5848: f64 = (var_xp_t * var_vb1c4);
        let assign5790_e5849: f64 = (assign5790_e5845 + assign5790_e5848);
        let assign5790_e5850: f64 = (var_cjc_t * assign5790_e5849);
        let assign5790_e5853: f64 = (1.0 - p.p76);
        let assign5790_e5854: f64 = (assign5790_e5850 * assign5790_e5853);
        let assign5790_e5857: f64 = (1.0 - p.p32);
        let assign5790_e5858: f64 = (assign5790_e5854 * assign5790_e5857);
        var_qtex = assign5790_e5858;
        var_qtex_dn0 = ((((var_cjc_t_dn0 * assign5790_e5849) + (var_cjc_t * ((((-var_xp_t_dn0) * var_vtexv) + (assign5790_e5843 * var_vtexv_dn0)) + ((var_xp_t_dn0 * var_vb1c4) + (var_xp_t * var_vb1c4_dn0))))) * assign5790_e5853) * assign5790_e5857);
        var_qtex_dn1 = ((((var_cjc_t_dn1 * assign5790_e5849) + (var_cjc_t * ((((-var_xp_t_dn1) * var_vtexv) + (assign5790_e5843 * var_vtexv_dn1)) + ((var_xp_t_dn1 * var_vb1c4) + (var_xp_t * var_vb1c4_dn1))))) * assign5790_e5853) * assign5790_e5857);
        var_qtex_dn2 = ((((var_cjc_t_dn2 * assign5790_e5849) + (var_cjc_t * ((((-var_xp_t_dn2) * var_vtexv) + (assign5790_e5843 * var_vtexv_dn2)) + ((var_xp_t_dn2 * var_vb1c4) + (var_xp_t * var_vb1c4_dn2))))) * assign5790_e5853) * assign5790_e5857);
        var_qtex_dn3 = ((((var_cjc_t_dn3 * assign5790_e5849) + (var_cjc_t * ((((-var_xp_t_dn3) * var_vtexv) + (assign5790_e5843 * var_vtexv_dn3)) + ((var_xp_t_dn3 * var_vb1c4) + (var_xp_t * var_vb1c4_dn3))))) * assign5790_e5853) * assign5790_e5857);
        var_qtex_dn4 = ((((var_cjc_t_dn4 * assign5790_e5849) + (var_cjc_t * ((((-var_xp_t_dn4) * var_vtexv) + (assign5790_e5843 * var_vtexv_dn4)) + ((var_xp_t_dn4 * var_vb1c4) + (var_xp_t * var_vb1c4_dn4))))) * assign5790_e5853) * assign5790_e5857);
        var_qtex_dn5 = ((((var_cjc_t_dn5 * assign5790_e5849) + (var_cjc_t * ((((-var_xp_t_dn5) * var_vtexv) + (assign5790_e5843 * var_vtexv_dn5)) + ((var_xp_t_dn5 * var_vb1c4) + (var_xp_t * var_vb1c4_dn5))))) * assign5790_e5853) * assign5790_e5857);
        var_qtex_dn6 = ((((var_cjc_t_dn6 * assign5790_e5849) + (var_cjc_t * ((((-var_xp_t_dn6) * var_vtexv) + (assign5790_e5843 * var_vtexv_dn6)) + ((var_xp_t_dn6 * var_vb1c4) + (var_xp_t * var_vb1c4_dn6))))) * assign5790_e5853) * assign5790_e5857);
        var_qtex_dn7 = ((((var_cjc_t_dn7 * assign5790_e5849) + (var_cjc_t * ((((-var_xp_t_dn7) * var_vtexv) + (assign5790_e5843 * var_vtexv_dn7)) + ((var_xp_t_dn7 * var_vb1c4) + (var_xp_t * var_vb1c4_dn7))))) * assign5790_e5853) * assign5790_e5857);
        var_qtex_dn8 = ((((var_cjc_t_dn8 * assign5790_e5849) + (var_cjc_t * ((((-var_xp_t_dn8) * var_vtexv) + (assign5790_e5843 * var_vtexv_dn8)) + ((var_xp_t_dn8 * var_vb1c4) + (var_xp_t * var_vb1c4_dn8))))) * assign5790_e5853) * assign5790_e5857);
        var_qtex_dn9 = ((((var_cjc_t_dn9 * assign5790_e5849) + (var_cjc_t * ((((-var_xp_t_dn9) * var_vtexv) + (assign5790_e5843 * var_vtexv_dn9)) + ((var_xp_t_dn9 * var_vb1c4) + (var_xp_t * var_vb1c4_dn9))))) * assign5790_e5853) * assign5790_e5857);
        var_qtex_dn10 = ((((var_cjc_t_dn10 * assign5790_e5849) + (var_cjc_t * ((((-var_xp_t_dn10) * var_vtexv) + (assign5790_e5843 * var_vtexv_dn10)) + ((var_xp_t_dn10 * var_vb1c4) + (var_xp_t * var_vb1c4_dn10))))) * assign5790_e5853) * assign5790_e5857);
        var_qtex_db0 = ((((var_cjc_t_db0 * assign5790_e5849) + (var_cjc_t * ((((-var_xp_t_db0) * var_vtexv) + (assign5790_e5843 * var_vtexv_db0)) + ((var_xp_t_db0 * var_vb1c4) + (var_xp_t * var_vb1c4_db0))))) * assign5790_e5853) * assign5790_e5857);
        var_qtex_db1 = ((((var_cjc_t_db1 * assign5790_e5849) + (var_cjc_t * ((((-var_xp_t_db1) * var_vtexv) + (assign5790_e5843 * var_vtexv_db1)) + ((var_xp_t_db1 * var_vb1c4) + (var_xp_t * var_vb1c4_db1))))) * assign5790_e5853) * assign5790_e5857);
        var_qtex_rv = 0.0;
        var_qtex_rdn0 = 0.0;
        var_qtex_rdn1 = 0.0;
        var_qtex_rdn2 = 0.0;
        var_qtex_rdn3 = 0.0;
        var_qtex_rdn4 = 0.0;
        var_qtex_rdn5 = 0.0;
        var_qtex_rdn6 = 0.0;
        var_qtex_rdn7 = 0.0;
        var_qtex_rdn8 = 0.0;
        var_qtex_rdn9 = 0.0;
        var_qtex_rdn10 = 0.0;
        var_qtex_rdb0 = 0.0;
        var_qtex_rdb1 = 0.0;

        let assign5800_e5861: f64 = (var_vbc3 - var_vfc);
        let assign5800_e5863: f64 = (assign5800_e5861 / var_a_vdcctc);
        var_dxa = assign5800_e5863;
        var_dxa_dn0 = ((((var_vbc3_dn0 - var_vfc_dn0) * var_a_vdcctc) - (assign5800_e5861 * var_a_vdcctc_dn0)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn1 = ((((var_vbc3_dn1 - var_vfc_dn1) * var_a_vdcctc) - (assign5800_e5861 * var_a_vdcctc_dn1)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn2 = ((((var_vbc3_dn2 - var_vfc_dn2) * var_a_vdcctc) - (assign5800_e5861 * var_a_vdcctc_dn2)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn3 = ((((var_vbc3_dn3 - var_vfc_dn3) * var_a_vdcctc) - (assign5800_e5861 * var_a_vdcctc_dn3)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn4 = ((((var_vbc3_dn4 - var_vfc_dn4) * var_a_vdcctc) - (assign5800_e5861 * var_a_vdcctc_dn4)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn5 = ((((var_vbc3_dn5 - var_vfc_dn5) * var_a_vdcctc) - (assign5800_e5861 * var_a_vdcctc_dn5)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn6 = ((((var_vbc3_dn6 - var_vfc_dn6) * var_a_vdcctc) - (assign5800_e5861 * var_a_vdcctc_dn6)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn7 = ((((var_vbc3_dn7 - var_vfc_dn7) * var_a_vdcctc) - (assign5800_e5861 * var_a_vdcctc_dn7)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn8 = ((((var_vbc3_dn8 - var_vfc_dn8) * var_a_vdcctc) - (assign5800_e5861 * var_a_vdcctc_dn8)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn9 = ((((var_vbc3_dn9 - var_vfc_dn9) * var_a_vdcctc) - (assign5800_e5861 * var_a_vdcctc_dn9)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_dn10 = ((((var_vbc3_dn10 - var_vfc_dn10) * var_a_vdcctc) - (assign5800_e5861 * var_a_vdcctc_dn10)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_db0 = ((((var_vbc3_db0 - var_vfc_db0) * var_a_vdcctc) - (assign5800_e5861 * var_a_vdcctc_db0)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_db1 = ((((var_vbc3_db1 - var_vfc_db1) * var_a_vdcctc) - (assign5800_e5861 * var_a_vdcctc_db1)) / (var_a_vdcctc * var_a_vdcctc));
        var_dxa_rv = 0.0;
        var_dxa_rdn0 = 0.0;
        var_dxa_rdn1 = 0.0;
        var_dxa_rdn2 = 0.0;
        var_dxa_rdn3 = 0.0;
        var_dxa_rdn4 = 0.0;
        var_dxa_rdn5 = 0.0;
        var_dxa_rdn6 = 0.0;
        var_dxa_rdn7 = 0.0;
        var_dxa_rdn8 = 0.0;
        var_dxa_rdn9 = 0.0;
        var_dxa_rdn10 = 0.0;
        var_dxa_rdb0 = 0.0;
        var_dxa_rdb1 = 0.0;

        let assign5810_e5866: f64 = if var_vbc3 < var_vfc { 1.0 } else { 0.0 };
        var_guard105 = assign5810_e5866;
        var_guard105_dn0 = 0.0;
        var_guard105_dn1 = 0.0;
        var_guard105_dn2 = 0.0;
        var_guard105_dn3 = 0.0;
        var_guard105_dn4 = 0.0;
        var_guard105_dn5 = 0.0;
        var_guard105_dn6 = 0.0;
        var_guard105_dn7 = 0.0;
        var_guard105_dn8 = 0.0;
        var_guard105_dn9 = 0.0;
        var_guard105_dn10 = 0.0;
        var_guard105_db0 = 0.0;
        var_guard105_db1 = 0.0;
        var_guard105_rv = 0.0;
        var_guard105_rdn0 = 0.0;
        var_guard105_rdn1 = 0.0;
        var_guard105_rdn2 = 0.0;
        var_guard105_rdn3 = 0.0;
        var_guard105_rdn4 = 0.0;
        var_guard105_rdn5 = 0.0;
        var_guard105_rdn6 = 0.0;
        var_guard105_rdn7 = 0.0;
        var_guard105_rdn8 = 0.0;
        var_guard105_rdn9 = 0.0;
        var_guard105_rdn10 = 0.0;
        var_guard105_rdb0 = 0.0;
        var_guard105_rdb1 = 0.0;

        *var_a_vdcctc_slot = var_a_vdcctc;
        *var_a_vdcctc_db0_slot = var_a_vdcctc_db0;
        *var_a_vdcctc_db1_slot = var_a_vdcctc_db1;
        *var_a_vdcctc_dn0_slot = var_a_vdcctc_dn0;
        *var_a_vdcctc_dn1_slot = var_a_vdcctc_dn1;
        *var_a_vdcctc_dn10_slot = var_a_vdcctc_dn10;
        *var_a_vdcctc_dn2_slot = var_a_vdcctc_dn2;
        *var_a_vdcctc_dn3_slot = var_a_vdcctc_dn3;
        *var_a_vdcctc_dn4_slot = var_a_vdcctc_dn4;
        *var_a_vdcctc_dn5_slot = var_a_vdcctc_dn5;
        *var_a_vdcctc_dn6_slot = var_a_vdcctc_dn6;
        *var_a_vdcctc_dn7_slot = var_a_vdcctc_dn7;
        *var_a_vdcctc_dn8_slot = var_a_vdcctc_dn8;
        *var_a_vdcctc_dn9_slot = var_a_vdcctc_dn9;
        *var_a_vdcctc_rdb0_slot = var_a_vdcctc_rdb0;
        *var_a_vdcctc_rdb1_slot = var_a_vdcctc_rdb1;
        *var_a_vdcctc_rdn0_slot = var_a_vdcctc_rdn0;
        *var_a_vdcctc_rdn1_slot = var_a_vdcctc_rdn1;
        *var_a_vdcctc_rdn10_slot = var_a_vdcctc_rdn10;
        *var_a_vdcctc_rdn2_slot = var_a_vdcctc_rdn2;
        *var_a_vdcctc_rdn3_slot = var_a_vdcctc_rdn3;
        *var_a_vdcctc_rdn4_slot = var_a_vdcctc_rdn4;
        *var_a_vdcctc_rdn5_slot = var_a_vdcctc_rdn5;
        *var_a_vdcctc_rdn6_slot = var_a_vdcctc_rdn6;
        *var_a_vdcctc_rdn7_slot = var_a_vdcctc_rdn7;
        *var_a_vdcctc_rdn8_slot = var_a_vdcctc_rdn8;
        *var_a_vdcctc_rdn9_slot = var_a_vdcctc_rdn9;
        *var_a_vdcctc_rv_slot = var_a_vdcctc_rv;
        *var_dxa_slot = var_dxa;
        *var_dxa_db0_slot = var_dxa_db0;
        *var_dxa_db1_slot = var_dxa_db1;
        *var_dxa_dn0_slot = var_dxa_dn0;
        *var_dxa_dn1_slot = var_dxa_dn1;
        *var_dxa_dn10_slot = var_dxa_dn10;
        *var_dxa_dn2_slot = var_dxa_dn2;
        *var_dxa_dn3_slot = var_dxa_dn3;
        *var_dxa_dn4_slot = var_dxa_dn4;
        *var_dxa_dn5_slot = var_dxa_dn5;
        *var_dxa_dn6_slot = var_dxa_dn6;
        *var_dxa_dn7_slot = var_dxa_dn7;
        *var_dxa_dn8_slot = var_dxa_dn8;
        *var_dxa_dn9_slot = var_dxa_dn9;
        *var_dxa_rdb0_slot = var_dxa_rdb0;
        *var_dxa_rdb1_slot = var_dxa_rdb1;
        *var_dxa_rdn0_slot = var_dxa_rdn0;
        *var_dxa_rdn1_slot = var_dxa_rdn1;
        *var_dxa_rdn10_slot = var_dxa_rdn10;
        *var_dxa_rdn2_slot = var_dxa_rdn2;
        *var_dxa_rdn3_slot = var_dxa_rdn3;
        *var_dxa_rdn4_slot = var_dxa_rdn4;
        *var_dxa_rdn5_slot = var_dxa_rdn5;
        *var_dxa_rdn6_slot = var_dxa_rdn6;
        *var_dxa_rdn7_slot = var_dxa_rdn7;
        *var_dxa_rdn8_slot = var_dxa_rdn8;
        *var_dxa_rdn9_slot = var_dxa_rdn9;
        *var_dxa_rv_slot = var_dxa_rv;
        *var_guard104_slot = var_guard104;
        *var_guard104_db0_slot = var_guard104_db0;
        *var_guard104_db1_slot = var_guard104_db1;
        *var_guard104_dn0_slot = var_guard104_dn0;
        *var_guard104_dn1_slot = var_guard104_dn1;
        *var_guard104_dn10_slot = var_guard104_dn10;
        *var_guard104_dn2_slot = var_guard104_dn2;
        *var_guard104_dn3_slot = var_guard104_dn3;
        *var_guard104_dn4_slot = var_guard104_dn4;
        *var_guard104_dn5_slot = var_guard104_dn5;
        *var_guard104_dn6_slot = var_guard104_dn6;
        *var_guard104_dn7_slot = var_guard104_dn7;
        *var_guard104_dn8_slot = var_guard104_dn8;
        *var_guard104_dn9_slot = var_guard104_dn9;
        *var_guard104_rdb0_slot = var_guard104_rdb0;
        *var_guard104_rdb1_slot = var_guard104_rdb1;
        *var_guard104_rdn0_slot = var_guard104_rdn0;
        *var_guard104_rdn1_slot = var_guard104_rdn1;
        *var_guard104_rdn10_slot = var_guard104_rdn10;
        *var_guard104_rdn2_slot = var_guard104_rdn2;
        *var_guard104_rdn3_slot = var_guard104_rdn3;
        *var_guard104_rdn4_slot = var_guard104_rdn4;
        *var_guard104_rdn5_slot = var_guard104_rdn5;
        *var_guard104_rdn6_slot = var_guard104_rdn6;
        *var_guard104_rdn7_slot = var_guard104_rdn7;
        *var_guard104_rdn8_slot = var_guard104_rdn8;
        *var_guard104_rdn9_slot = var_guard104_rdn9;
        *var_guard104_rv_slot = var_guard104_rv;
        *var_guard105_slot = var_guard105;
        *var_guard105_db0_slot = var_guard105_db0;
        *var_guard105_db1_slot = var_guard105_db1;
        *var_guard105_dn0_slot = var_guard105_dn0;
        *var_guard105_dn1_slot = var_guard105_dn1;
        *var_guard105_dn10_slot = var_guard105_dn10;
        *var_guard105_dn2_slot = var_guard105_dn2;
        *var_guard105_dn3_slot = var_guard105_dn3;
        *var_guard105_dn4_slot = var_guard105_dn4;
        *var_guard105_dn5_slot = var_guard105_dn5;
        *var_guard105_dn6_slot = var_guard105_dn6;
        *var_guard105_dn7_slot = var_guard105_dn7;
        *var_guard105_dn8_slot = var_guard105_dn8;
        *var_guard105_dn9_slot = var_guard105_dn9;
        *var_guard105_rdb0_slot = var_guard105_rdb0;
        *var_guard105_rdb1_slot = var_guard105_rdb1;
        *var_guard105_rdn0_slot = var_guard105_rdn0;
        *var_guard105_rdn1_slot = var_guard105_rdn1;
        *var_guard105_rdn10_slot = var_guard105_rdn10;
        *var_guard105_rdn2_slot = var_guard105_rdn2;
        *var_guard105_rdn3_slot = var_guard105_rdn3;
        *var_guard105_rdn4_slot = var_guard105_rdn4;
        *var_guard105_rdn5_slot = var_guard105_rdn5;
        *var_guard105_rdn6_slot = var_guard105_rdn6;
        *var_guard105_rdn7_slot = var_guard105_rdn7;
        *var_guard105_rdn8_slot = var_guard105_rdn8;
        *var_guard105_rdn9_slot = var_guard105_rdn9;
        *var_guard105_rv_slot = var_guard105_rv;
        *var_qb0_slot = var_qb0;
        *var_qb0_db0_slot = var_qb0_db0;
        *var_qb0_db1_slot = var_qb0_db1;
        *var_qb0_dn0_slot = var_qb0_dn0;
        *var_qb0_dn1_slot = var_qb0_dn1;
        *var_qb0_dn10_slot = var_qb0_dn10;
        *var_qb0_dn2_slot = var_qb0_dn2;
        *var_qb0_dn3_slot = var_qb0_dn3;
        *var_qb0_dn4_slot = var_qb0_dn4;
        *var_qb0_dn5_slot = var_qb0_dn5;
        *var_qb0_dn6_slot = var_qb0_dn6;
        *var_qb0_dn7_slot = var_qb0_dn7;
        *var_qb0_dn8_slot = var_qb0_dn8;
        *var_qb0_dn9_slot = var_qb0_dn9;
        *var_qb0_rdb0_slot = var_qb0_rdb0;
        *var_qb0_rdb1_slot = var_qb0_rdb1;
        *var_qb0_rdn0_slot = var_qb0_rdn0;
        *var_qb0_rdn1_slot = var_qb0_rdn1;
        *var_qb0_rdn10_slot = var_qb0_rdn10;
        *var_qb0_rdn2_slot = var_qb0_rdn2;
        *var_qb0_rdn3_slot = var_qb0_rdn3;
        *var_qb0_rdn4_slot = var_qb0_rdn4;
        *var_qb0_rdn5_slot = var_qb0_rdn5;
        *var_qb0_rdn6_slot = var_qb0_rdn6;
        *var_qb0_rdn7_slot = var_qb0_rdn7;
        *var_qb0_rdn8_slot = var_qb0_rdn8;
        *var_qb0_rdn9_slot = var_qb0_rdn9;
        *var_qb0_rv_slot = var_qb0_rv;
        *var_qbc_qs_slot = var_qbc_qs;
        *var_qbc_qs_db0_slot = var_qbc_qs_db0;
        *var_qbc_qs_db1_slot = var_qbc_qs_db1;
        *var_qbc_qs_dn0_slot = var_qbc_qs_dn0;
        *var_qbc_qs_dn1_slot = var_qbc_qs_dn1;
        *var_qbc_qs_dn10_slot = var_qbc_qs_dn10;
        *var_qbc_qs_dn2_slot = var_qbc_qs_dn2;
        *var_qbc_qs_dn3_slot = var_qbc_qs_dn3;
        *var_qbc_qs_dn4_slot = var_qbc_qs_dn4;
        *var_qbc_qs_dn5_slot = var_qbc_qs_dn5;
        *var_qbc_qs_dn6_slot = var_qbc_qs_dn6;
        *var_qbc_qs_dn7_slot = var_qbc_qs_dn7;
        *var_qbc_qs_dn8_slot = var_qbc_qs_dn8;
        *var_qbc_qs_dn9_slot = var_qbc_qs_dn9;
        *var_qbc_qs_rdb0_slot = var_qbc_qs_rdb0;
        *var_qbc_qs_rdb1_slot = var_qbc_qs_rdb1;
        *var_qbc_qs_rdn0_slot = var_qbc_qs_rdn0;
        *var_qbc_qs_rdn1_slot = var_qbc_qs_rdn1;
        *var_qbc_qs_rdn10_slot = var_qbc_qs_rdn10;
        *var_qbc_qs_rdn2_slot = var_qbc_qs_rdn2;
        *var_qbc_qs_rdn3_slot = var_qbc_qs_rdn3;
        *var_qbc_qs_rdn4_slot = var_qbc_qs_rdn4;
        *var_qbc_qs_rdn5_slot = var_qbc_qs_rdn5;
        *var_qbc_qs_rdn6_slot = var_qbc_qs_rdn6;
        *var_qbc_qs_rdn7_slot = var_qbc_qs_rdn7;
        *var_qbc_qs_rdn8_slot = var_qbc_qs_rdn8;
        *var_qbc_qs_rdn9_slot = var_qbc_qs_rdn9;
        *var_qbc_qs_rv_slot = var_qbc_qs_rv;
        *var_qbe_qs_slot = var_qbe_qs;
        *var_qbe_qs_db0_slot = var_qbe_qs_db0;
        *var_qbe_qs_db1_slot = var_qbe_qs_db1;
        *var_qbe_qs_dn0_slot = var_qbe_qs_dn0;
        *var_qbe_qs_dn1_slot = var_qbe_qs_dn1;
        *var_qbe_qs_dn10_slot = var_qbe_qs_dn10;
        *var_qbe_qs_dn2_slot = var_qbe_qs_dn2;
        *var_qbe_qs_dn3_slot = var_qbe_qs_dn3;
        *var_qbe_qs_dn4_slot = var_qbe_qs_dn4;
        *var_qbe_qs_dn5_slot = var_qbe_qs_dn5;
        *var_qbe_qs_dn6_slot = var_qbe_qs_dn6;
        *var_qbe_qs_dn7_slot = var_qbe_qs_dn7;
        *var_qbe_qs_dn8_slot = var_qbe_qs_dn8;
        *var_qbe_qs_dn9_slot = var_qbe_qs_dn9;
        *var_qbe_qs_rdb0_slot = var_qbe_qs_rdb0;
        *var_qbe_qs_rdb1_slot = var_qbe_qs_rdb1;
        *var_qbe_qs_rdn0_slot = var_qbe_qs_rdn0;
        *var_qbe_qs_rdn1_slot = var_qbe_qs_rdn1;
        *var_qbe_qs_rdn10_slot = var_qbe_qs_rdn10;
        *var_qbe_qs_rdn2_slot = var_qbe_qs_rdn2;
        *var_qbe_qs_rdn3_slot = var_qbe_qs_rdn3;
        *var_qbe_qs_rdn4_slot = var_qbe_qs_rdn4;
        *var_qbe_qs_rdn5_slot = var_qbe_qs_rdn5;
        *var_qbe_qs_rdn6_slot = var_qbe_qs_rdn6;
        *var_qbe_qs_rdn7_slot = var_qbe_qs_rdn7;
        *var_qbe_qs_rdn8_slot = var_qbe_qs_rdn8;
        *var_qbe_qs_rdn9_slot = var_qbe_qs_rdn9;
        *var_qbe_qs_rv_slot = var_qbe_qs_rv;
        *var_qtc_slot = var_qtc;
        *var_qtc_db0_slot = var_qtc_db0;
        *var_qtc_db1_slot = var_qtc_db1;
        *var_qtc_dn0_slot = var_qtc_dn0;
        *var_qtc_dn1_slot = var_qtc_dn1;
        *var_qtc_dn10_slot = var_qtc_dn10;
        *var_qtc_dn2_slot = var_qtc_dn2;
        *var_qtc_dn3_slot = var_qtc_dn3;
        *var_qtc_dn4_slot = var_qtc_dn4;
        *var_qtc_dn5_slot = var_qtc_dn5;
        *var_qtc_dn6_slot = var_qtc_dn6;
        *var_qtc_dn7_slot = var_qtc_dn7;
        *var_qtc_dn8_slot = var_qtc_dn8;
        *var_qtc_dn9_slot = var_qtc_dn9;
        *var_qtc_rdb0_slot = var_qtc_rdb0;
        *var_qtc_rdb1_slot = var_qtc_rdb1;
        *var_qtc_rdn0_slot = var_qtc_rdn0;
        *var_qtc_rdn1_slot = var_qtc_rdn1;
        *var_qtc_rdn10_slot = var_qtc_rdn10;
        *var_qtc_rdn2_slot = var_qtc_rdn2;
        *var_qtc_rdn3_slot = var_qtc_rdn3;
        *var_qtc_rdn4_slot = var_qtc_rdn4;
        *var_qtc_rdn5_slot = var_qtc_rdn5;
        *var_qtc_rdn6_slot = var_qtc_rdn6;
        *var_qtc_rdn7_slot = var_qtc_rdn7;
        *var_qtc_rdn8_slot = var_qtc_rdn8;
        *var_qtc_rdn9_slot = var_qtc_rdn9;
        *var_qtc_rv_slot = var_qtc_rv;
        *var_qte_s_slot = var_qte_s;
        *var_qte_s_db0_slot = var_qte_s_db0;
        *var_qte_s_db1_slot = var_qte_s_db1;
        *var_qte_s_dn0_slot = var_qte_s_dn0;
        *var_qte_s_dn1_slot = var_qte_s_dn1;
        *var_qte_s_dn10_slot = var_qte_s_dn10;
        *var_qte_s_dn2_slot = var_qte_s_dn2;
        *var_qte_s_dn3_slot = var_qte_s_dn3;
        *var_qte_s_dn4_slot = var_qte_s_dn4;
        *var_qte_s_dn5_slot = var_qte_s_dn5;
        *var_qte_s_dn6_slot = var_qte_s_dn6;
        *var_qte_s_dn7_slot = var_qte_s_dn7;
        *var_qte_s_dn8_slot = var_qte_s_dn8;
        *var_qte_s_dn9_slot = var_qte_s_dn9;
        *var_qte_s_rdb0_slot = var_qte_s_rdb0;
        *var_qte_s_rdb1_slot = var_qte_s_rdb1;
        *var_qte_s_rdn0_slot = var_qte_s_rdn0;
        *var_qte_s_rdn1_slot = var_qte_s_rdn1;
        *var_qte_s_rdn10_slot = var_qte_s_rdn10;
        *var_qte_s_rdn2_slot = var_qte_s_rdn2;
        *var_qte_s_rdn3_slot = var_qte_s_rdn3;
        *var_qte_s_rdn4_slot = var_qte_s_rdn4;
        *var_qte_s_rdn5_slot = var_qte_s_rdn5;
        *var_qte_s_rdn6_slot = var_qte_s_rdn6;
        *var_qte_s_rdn7_slot = var_qte_s_rdn7;
        *var_qte_s_rdn8_slot = var_qte_s_rdn8;
        *var_qte_s_rdn9_slot = var_qte_s_rdn9;
        *var_qte_s_rv_slot = var_qte_s_rv;
        *var_qtex_slot = var_qtex;
        *var_qtex_db0_slot = var_qtex_db0;
        *var_qtex_db1_slot = var_qtex_db1;
        *var_qtex_dn0_slot = var_qtex_dn0;
        *var_qtex_dn1_slot = var_qtex_dn1;
        *var_qtex_dn10_slot = var_qtex_dn10;
        *var_qtex_dn2_slot = var_qtex_dn2;
        *var_qtex_dn3_slot = var_qtex_dn3;
        *var_qtex_dn4_slot = var_qtex_dn4;
        *var_qtex_dn5_slot = var_qtex_dn5;
        *var_qtex_dn6_slot = var_qtex_dn6;
        *var_qtex_dn7_slot = var_qtex_dn7;
        *var_qtex_dn8_slot = var_qtex_dn8;
        *var_qtex_dn9_slot = var_qtex_dn9;
        *var_qtex_rdb0_slot = var_qtex_rdb0;
        *var_qtex_rdb1_slot = var_qtex_rdb1;
        *var_qtex_rdn0_slot = var_qtex_rdn0;
        *var_qtex_rdn1_slot = var_qtex_rdn1;
        *var_qtex_rdn10_slot = var_qtex_rdn10;
        *var_qtex_rdn2_slot = var_qtex_rdn2;
        *var_qtex_rdn3_slot = var_qtex_rdn3;
        *var_qtex_rdn4_slot = var_qtex_rdn4;
        *var_qtex_rdn5_slot = var_qtex_rdn5;
        *var_qtex_rdn6_slot = var_qtex_rdn6;
        *var_qtex_rdn7_slot = var_qtex_rdn7;
        *var_qtex_rdn8_slot = var_qtex_rdn8;
        *var_qtex_rdn9_slot = var_qtex_rdn9;
        *var_qtex_rv_slot = var_qtex_rv;
        *var_vjcex_slot = var_vjcex;
        *var_vjcex_db0_slot = var_vjcex_db0;
        *var_vjcex_db1_slot = var_vjcex_db1;
        *var_vjcex_dn0_slot = var_vjcex_dn0;
        *var_vjcex_dn1_slot = var_vjcex_dn1;
        *var_vjcex_dn10_slot = var_vjcex_dn10;
        *var_vjcex_dn2_slot = var_vjcex_dn2;
        *var_vjcex_dn3_slot = var_vjcex_dn3;
        *var_vjcex_dn4_slot = var_vjcex_dn4;
        *var_vjcex_dn5_slot = var_vjcex_dn5;
        *var_vjcex_dn6_slot = var_vjcex_dn6;
        *var_vjcex_dn7_slot = var_vjcex_dn7;
        *var_vjcex_dn8_slot = var_vjcex_dn8;
        *var_vjcex_dn9_slot = var_vjcex_dn9;
        *var_vjcex_rdb0_slot = var_vjcex_rdb0;
        *var_vjcex_rdb1_slot = var_vjcex_rdb1;
        *var_vjcex_rdn0_slot = var_vjcex_rdn0;
        *var_vjcex_rdn1_slot = var_vjcex_rdn1;
        *var_vjcex_rdn10_slot = var_vjcex_rdn10;
        *var_vjcex_rdn2_slot = var_vjcex_rdn2;
        *var_vjcex_rdn3_slot = var_vjcex_rdn3;
        *var_vjcex_rdn4_slot = var_vjcex_rdn4;
        *var_vjcex_rdn5_slot = var_vjcex_rdn5;
        *var_vjcex_rdn6_slot = var_vjcex_rdn6;
        *var_vjcex_rdn7_slot = var_vjcex_rdn7;
        *var_vjcex_rdn8_slot = var_vjcex_rdn8;
        *var_vjcex_rdn9_slot = var_vjcex_rdn9;
        *var_vjcex_rv_slot = var_vjcex_rv;
        *var_vtexv_slot = var_vtexv;
        *var_vtexv_db0_slot = var_vtexv_db0;
        *var_vtexv_db1_slot = var_vtexv_db1;
        *var_vtexv_dn0_slot = var_vtexv_dn0;
        *var_vtexv_dn1_slot = var_vtexv_dn1;
        *var_vtexv_dn10_slot = var_vtexv_dn10;
        *var_vtexv_dn2_slot = var_vtexv_dn2;
        *var_vtexv_dn3_slot = var_vtexv_dn3;
        *var_vtexv_dn4_slot = var_vtexv_dn4;
        *var_vtexv_dn5_slot = var_vtexv_dn5;
        *var_vtexv_dn6_slot = var_vtexv_dn6;
        *var_vtexv_dn7_slot = var_vtexv_dn7;
        *var_vtexv_dn8_slot = var_vtexv_dn8;
        *var_vtexv_dn9_slot = var_vtexv_dn9;
        *var_vtexv_rdb0_slot = var_vtexv_rdb0;
        *var_vtexv_rdb1_slot = var_vtexv_rdb1;
        *var_vtexv_rdn0_slot = var_vtexv_rdn0;
        *var_vtexv_rdn1_slot = var_vtexv_rdn1;
        *var_vtexv_rdn10_slot = var_vtexv_rdn10;
        *var_vtexv_rdn2_slot = var_vtexv_rdn2;
        *var_vtexv_rdn3_slot = var_vtexv_rdn3;
        *var_vtexv_rdn4_slot = var_vtexv_rdn4;
        *var_vtexv_rdn5_slot = var_vtexv_rdn5;
        *var_vtexv_rdn6_slot = var_vtexv_rdn6;
        *var_vtexv_rdn7_slot = var_vtexv_rdn7;
        *var_vtexv_rdn8_slot = var_vtexv_rdn8;
        *var_vtexv_rdn9_slot = var_vtexv_rdn9;
        *var_vtexv_rv_slot = var_vtexv_rv;
    }

    pub(super) fn stamp_reactive_block_34(
        p: &Parameters,
        var_a_vdcctc: f64,
        var_a_vdcctc_db0: f64,
        var_a_vdcctc_db1: f64,
        var_a_vdcctc_dn0: f64,
        var_a_vdcctc_dn1: f64,
        var_a_vdcctc_dn10: f64,
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
        var_cjc_t_dn2: f64,
        var_cjc_t_dn3: f64,
        var_cjc_t_dn4: f64,
        var_cjc_t_dn5: f64,
        var_cjc_t_dn6: f64,
        var_cjc_t_dn7: f64,
        var_cjc_t_dn8: f64,
        var_cjc_t_dn9: f64,
        var_dxa: f64,
        var_dxa_db0: f64,
        var_dxa_db1: f64,
        var_dxa_dn0: f64,
        var_dxa_dn1: f64,
        var_dxa_dn10: f64,
        var_dxa_dn2: f64,
        var_dxa_dn3: f64,
        var_dxa_dn4: f64,
        var_dxa_dn5: f64,
        var_dxa_dn6: f64,
        var_dxa_dn7: f64,
        var_dxa_dn8: f64,
        var_dxa_dn9: f64,
        var_guard105: f64,
        var_ik_t: f64,
        var_ik_t_db0: f64,
        var_ik_t_db1: f64,
        var_ik_t_dn0: f64,
        var_ik_t_dn1: f64,
        var_ik_t_dn10: f64,
        var_ik_t_dn2: f64,
        var_ik_t_dn3: f64,
        var_ik_t_dn4: f64,
        var_ik_t_dn5: f64,
        var_ik_t_dn6: f64,
        var_ik_t_dn7: f64,
        var_ik_t_dn8: f64,
        var_ik_t_dn9: f64,
        var_is_t: f64,
        var_is_t_db0: f64,
        var_is_t_db1: f64,
        var_is_t_dn0: f64,
        var_is_t_dn1: f64,
        var_is_t_dn10: f64,
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
        var_pwex_dn2: f64,
        var_pwex_dn3: f64,
        var_pwex_dn4: f64,
        var_pwex_dn5: f64,
        var_pwex_dn6: f64,
        var_pwex_dn7: f64,
        var_pwex_dn8: f64,
        var_pwex_dn9: f64,
        var_qb0: f64,
        var_qb0_db0: f64,
        var_qb0_db1: f64,
        var_qb0_dn0: f64,
        var_qb0_dn1: f64,
        var_qb0_dn10: f64,
        var_qb0_dn2: f64,
        var_qb0_dn3: f64,
        var_qb0_dn4: f64,
        var_qb0_dn5: f64,
        var_qb0_dn6: f64,
        var_qb0_dn7: f64,
        var_qb0_dn8: f64,
        var_qb0_dn9: f64,
        var_rcv_t: f64,
        var_rcv_t_db0: f64,
        var_rcv_t_db1: f64,
        var_rcv_t_dn0: f64,
        var_rcv_t_dn1: f64,
        var_rcv_t_dn10: f64,
        var_rcv_t_dn2: f64,
        var_rcv_t_dn3: f64,
        var_rcv_t_dn4: f64,
        var_rcv_t_dn5: f64,
        var_rcv_t_dn6: f64,
        var_rcv_t_dn7: f64,
        var_rcv_t_dn8: f64,
        var_rcv_t_dn9: f64,
        var_taub_t: f64,
        var_taub_t_db0: f64,
        var_taub_t_db1: f64,
        var_taub_t_dn0: f64,
        var_taub_t_dn1: f64,
        var_taub_t_dn10: f64,
        var_taub_t_dn2: f64,
        var_taub_t_dn3: f64,
        var_taub_t_dn4: f64,
        var_taub_t_dn5: f64,
        var_taub_t_dn6: f64,
        var_taub_t_dn7: f64,
        var_taub_t_dn8: f64,
        var_taub_t_dn9: f64,
        var_taue_t: f64,
        var_taue_t_db0: f64,
        var_taue_t_db1: f64,
        var_taue_t_dn0: f64,
        var_taue_t_dn1: f64,
        var_taue_t_dn10: f64,
        var_taue_t_dn2: f64,
        var_taue_t_dn3: f64,
        var_taue_t_dn4: f64,
        var_taue_t_dn5: f64,
        var_taue_t_dn6: f64,
        var_taue_t_dn7: f64,
        var_taue_t_dn8: f64,
        var_taue_t_dn9: f64,
        var_taur_t: f64,
        var_taur_t_db0: f64,
        var_taur_t_db1: f64,
        var_taur_t_dn0: f64,
        var_taur_t_dn1: f64,
        var_taur_t_dn10: f64,
        var_taur_t_dn2: f64,
        var_taur_t_dn3: f64,
        var_taur_t_dn4: f64,
        var_taur_t_dn5: f64,
        var_taur_t_dn6: f64,
        var_taur_t_dn7: f64,
        var_taur_t_dn8: f64,
        var_taur_t_dn9: f64,
        var_tepi_t: f64,
        var_tepi_t_db0: f64,
        var_tepi_t_db1: f64,
        var_tepi_t_dn0: f64,
        var_tepi_t_dn1: f64,
        var_tepi_t_dn10: f64,
        var_tepi_t_dn2: f64,
        var_tepi_t_dn3: f64,
        var_tepi_t_dn4: f64,
        var_tepi_t_dn5: f64,
        var_tepi_t_dn6: f64,
        var_tepi_t_dn7: f64,
        var_tepi_t_dn8: f64,
        var_tepi_t_dn9: f64,
        var_vb2e1: f64,
        var_vb2e1_db0: f64,
        var_vb2e1_db1: f64,
        var_vb2e1_dn0: f64,
        var_vb2e1_dn1: f64,
        var_vb2e1_dn10: f64,
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
        var_vdc_ctc_t_dn2: f64,
        var_vdc_ctc_t_dn3: f64,
        var_vdc_ctc_t_dn4: f64,
        var_vdc_ctc_t_dn5: f64,
        var_vdc_ctc_t_dn6: f64,
        var_vdc_ctc_t_dn7: f64,
        var_vdc_ctc_t_dn8: f64,
        var_vdc_ctc_t_dn9: f64,
        var_vfc: f64,
        var_vfc_db0: f64,
        var_vfc_db1: f64,
        var_vfc_dn0: f64,
        var_vfc_dn1: f64,
        var_vfc_dn10: f64,
        var_vfc_dn2: f64,
        var_vfc_dn3: f64,
        var_vfc_dn4: f64,
        var_vfc_dn5: f64,
        var_vfc_dn6: f64,
        var_vfc_dn7: f64,
        var_vfc_dn8: f64,
        var_vfc_dn9: f64,
        var_vt: f64,
        var_vt_db0: f64,
        var_vt_db1: f64,
        var_vt_dn0: f64,
        var_vt_dn1: f64,
        var_vt_dn10: f64,
        var_vt_dn2: f64,
        var_vt_dn3: f64,
        var_vt_dn4: f64,
        var_vt_dn5: f64,
        var_vt_dn6: f64,
        var_vt_dn7: f64,
        var_vt_dn8: f64,
        var_vt_dn9: f64,
        var_xi_w: f64,
        var_xi_w_db0: f64,
        var_xi_w_db1: f64,
        var_xi_w_dn0: f64,
        var_xi_w_dn1: f64,
        var_xi_w_dn10: f64,
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
        var_xp_t_dn2: f64,
        var_xp_t_dn3: f64,
        var_xp_t_dn4: f64,
        var_xp_t_dn5: f64,
        var_xp_t_dn6: f64,
        var_xp_t_dn7: f64,
        var_xp_t_dn8: f64,
        var_xp_t_dn9: f64,
        var_expl_slot: &mut f64,
        var_expl_db0_slot: &mut f64,
        var_expl_db1_slot: &mut f64,
        var_expl_dn0_slot: &mut f64,
        var_expl_dn1_slot: &mut f64,
        var_expl_dn10_slot: &mut f64,
        var_expl_dn2_slot: &mut f64,
        var_expl_dn3_slot: &mut f64,
        var_expl_dn4_slot: &mut f64,
        var_expl_dn5_slot: &mut f64,
        var_expl_dn6_slot: &mut f64,
        var_expl_dn7_slot: &mut f64,
        var_expl_dn8_slot: &mut f64,
        var_expl_dn9_slot: &mut f64,
        var_expl_rdb0_slot: &mut f64,
        var_expl_rdb1_slot: &mut f64,
        var_expl_rdn0_slot: &mut f64,
        var_expl_rdn1_slot: &mut f64,
        var_expl_rdn10_slot: &mut f64,
        var_expl_rdn2_slot: &mut f64,
        var_expl_rdn3_slot: &mut f64,
        var_expl_rdn4_slot: &mut f64,
        var_expl_rdn5_slot: &mut f64,
        var_expl_rdn6_slot: &mut f64,
        var_expl_rdn7_slot: &mut f64,
        var_expl_rdn8_slot: &mut f64,
        var_expl_rdn9_slot: &mut f64,
        var_expl_rv_slot: &mut f64,
        var_guard106_slot: &mut f64,
        var_guard106_db0_slot: &mut f64,
        var_guard106_db1_slot: &mut f64,
        var_guard106_dn0_slot: &mut f64,
        var_guard106_dn1_slot: &mut f64,
        var_guard106_dn10_slot: &mut f64,
        var_guard106_dn2_slot: &mut f64,
        var_guard106_dn3_slot: &mut f64,
        var_guard106_dn4_slot: &mut f64,
        var_guard106_dn5_slot: &mut f64,
        var_guard106_dn6_slot: &mut f64,
        var_guard106_dn7_slot: &mut f64,
        var_guard106_dn8_slot: &mut f64,
        var_guard106_dn9_slot: &mut f64,
        var_guard106_rdb0_slot: &mut f64,
        var_guard106_rdb1_slot: &mut f64,
        var_guard106_rdn0_slot: &mut f64,
        var_guard106_rdn1_slot: &mut f64,
        var_guard106_rdn10_slot: &mut f64,
        var_guard106_rdn2_slot: &mut f64,
        var_guard106_rdn3_slot: &mut f64,
        var_guard106_rdn4_slot: &mut f64,
        var_guard106_rdn5_slot: &mut f64,
        var_guard106_rdn6_slot: &mut f64,
        var_guard106_rdn7_slot: &mut f64,
        var_guard106_rdn8_slot: &mut f64,
        var_guard106_rdn9_slot: &mut f64,
        var_guard106_rv_slot: &mut f64,
        var_guard107_slot: &mut f64,
        var_guard107_db0_slot: &mut f64,
        var_guard107_db1_slot: &mut f64,
        var_guard107_dn0_slot: &mut f64,
        var_guard107_dn1_slot: &mut f64,
        var_guard107_dn10_slot: &mut f64,
        var_guard107_dn2_slot: &mut f64,
        var_guard107_dn3_slot: &mut f64,
        var_guard107_dn4_slot: &mut f64,
        var_guard107_dn5_slot: &mut f64,
        var_guard107_dn6_slot: &mut f64,
        var_guard107_dn7_slot: &mut f64,
        var_guard107_dn8_slot: &mut f64,
        var_guard107_dn9_slot: &mut f64,
        var_guard107_rdb0_slot: &mut f64,
        var_guard107_rdb1_slot: &mut f64,
        var_guard107_rdn0_slot: &mut f64,
        var_guard107_rdn1_slot: &mut f64,
        var_guard107_rdn10_slot: &mut f64,
        var_guard107_rdn2_slot: &mut f64,
        var_guard107_rdn3_slot: &mut f64,
        var_guard107_rdn4_slot: &mut f64,
        var_guard107_rdn5_slot: &mut f64,
        var_guard107_rdn6_slot: &mut f64,
        var_guard107_rdn7_slot: &mut f64,
        var_guard107_rdn8_slot: &mut f64,
        var_guard107_rdn9_slot: &mut f64,
        var_guard107_rv_slot: &mut f64,
        var_qe0_slot: &mut f64,
        var_qe0_db0_slot: &mut f64,
        var_qe0_db1_slot: &mut f64,
        var_qe0_dn0_slot: &mut f64,
        var_qe0_dn1_slot: &mut f64,
        var_qe0_dn10_slot: &mut f64,
        var_qe0_dn2_slot: &mut f64,
        var_qe0_dn3_slot: &mut f64,
        var_qe0_dn4_slot: &mut f64,
        var_qe0_dn5_slot: &mut f64,
        var_qe0_dn6_slot: &mut f64,
        var_qe0_dn7_slot: &mut f64,
        var_qe0_dn8_slot: &mut f64,
        var_qe0_dn9_slot: &mut f64,
        var_qe0_rdb0_slot: &mut f64,
        var_qe0_rdb1_slot: &mut f64,
        var_qe0_rdn0_slot: &mut f64,
        var_qe0_rdn1_slot: &mut f64,
        var_qe0_rdn10_slot: &mut f64,
        var_qe0_rdn2_slot: &mut f64,
        var_qe0_rdn3_slot: &mut f64,
        var_qe0_rdn4_slot: &mut f64,
        var_qe0_rdn5_slot: &mut f64,
        var_qe0_rdn6_slot: &mut f64,
        var_qe0_rdn7_slot: &mut f64,
        var_qe0_rdn8_slot: &mut f64,
        var_qe0_rdn9_slot: &mut f64,
        var_qe0_rv_slot: &mut f64,
        var_qe_qs_slot: &mut f64,
        var_qe_qs_db0_slot: &mut f64,
        var_qe_qs_db1_slot: &mut f64,
        var_qe_qs_dn0_slot: &mut f64,
        var_qe_qs_dn1_slot: &mut f64,
        var_qe_qs_dn10_slot: &mut f64,
        var_qe_qs_dn2_slot: &mut f64,
        var_qe_qs_dn3_slot: &mut f64,
        var_qe_qs_dn4_slot: &mut f64,
        var_qe_qs_dn5_slot: &mut f64,
        var_qe_qs_dn6_slot: &mut f64,
        var_qe_qs_dn7_slot: &mut f64,
        var_qe_qs_dn8_slot: &mut f64,
        var_qe_qs_dn9_slot: &mut f64,
        var_qe_qs_rdb0_slot: &mut f64,
        var_qe_qs_rdb1_slot: &mut f64,
        var_qe_qs_rdn0_slot: &mut f64,
        var_qe_qs_rdn1_slot: &mut f64,
        var_qe_qs_rdn10_slot: &mut f64,
        var_qe_qs_rdn2_slot: &mut f64,
        var_qe_qs_rdn3_slot: &mut f64,
        var_qe_qs_rdn4_slot: &mut f64,
        var_qe_qs_rdn5_slot: &mut f64,
        var_qe_qs_rdn6_slot: &mut f64,
        var_qe_qs_rdn7_slot: &mut f64,
        var_qe_qs_rdn8_slot: &mut f64,
        var_qe_qs_rdn9_slot: &mut f64,
        var_qe_qs_rv_slot: &mut f64,
        var_qepi_slot: &mut f64,
        var_qepi0_slot: &mut f64,
        var_qepi0_db0_slot: &mut f64,
        var_qepi0_db1_slot: &mut f64,
        var_qepi0_dn0_slot: &mut f64,
        var_qepi0_dn1_slot: &mut f64,
        var_qepi0_dn10_slot: &mut f64,
        var_qepi0_dn2_slot: &mut f64,
        var_qepi0_dn3_slot: &mut f64,
        var_qepi0_dn4_slot: &mut f64,
        var_qepi0_dn5_slot: &mut f64,
        var_qepi0_dn6_slot: &mut f64,
        var_qepi0_dn7_slot: &mut f64,
        var_qepi0_dn8_slot: &mut f64,
        var_qepi0_dn9_slot: &mut f64,
        var_qepi0_rdb0_slot: &mut f64,
        var_qepi0_rdb1_slot: &mut f64,
        var_qepi0_rdn0_slot: &mut f64,
        var_qepi0_rdn1_slot: &mut f64,
        var_qepi0_rdn10_slot: &mut f64,
        var_qepi0_rdn2_slot: &mut f64,
        var_qepi0_rdn3_slot: &mut f64,
        var_qepi0_rdn4_slot: &mut f64,
        var_qepi0_rdn5_slot: &mut f64,
        var_qepi0_rdn6_slot: &mut f64,
        var_qepi0_rdn7_slot: &mut f64,
        var_qepi0_rdn8_slot: &mut f64,
        var_qepi0_rdn9_slot: &mut f64,
        var_qepi0_rv_slot: &mut f64,
        var_qepi_db0_slot: &mut f64,
        var_qepi_db1_slot: &mut f64,
        var_qepi_dn0_slot: &mut f64,
        var_qepi_dn1_slot: &mut f64,
        var_qepi_dn10_slot: &mut f64,
        var_qepi_dn2_slot: &mut f64,
        var_qepi_dn3_slot: &mut f64,
        var_qepi_dn4_slot: &mut f64,
        var_qepi_dn5_slot: &mut f64,
        var_qepi_dn6_slot: &mut f64,
        var_qepi_dn7_slot: &mut f64,
        var_qepi_dn8_slot: &mut f64,
        var_qepi_dn9_slot: &mut f64,
        var_qepi_rdb0_slot: &mut f64,
        var_qepi_rdb1_slot: &mut f64,
        var_qepi_rdn0_slot: &mut f64,
        var_qepi_rdn1_slot: &mut f64,
        var_qepi_rdn10_slot: &mut f64,
        var_qepi_rdn2_slot: &mut f64,
        var_qepi_rdn3_slot: &mut f64,
        var_qepi_rdn4_slot: &mut f64,
        var_qepi_rdn5_slot: &mut f64,
        var_qepi_rdn6_slot: &mut f64,
        var_qepi_rdn7_slot: &mut f64,
        var_qepi_rdn8_slot: &mut f64,
        var_qepi_rdn9_slot: &mut f64,
        var_qepi_rv_slot: &mut f64,
        var_qex_slot: &mut f64,
        var_qex_db0_slot: &mut f64,
        var_qex_db1_slot: &mut f64,
        var_qex_dn0_slot: &mut f64,
        var_qex_dn1_slot: &mut f64,
        var_qex_dn10_slot: &mut f64,
        var_qex_dn2_slot: &mut f64,
        var_qex_dn3_slot: &mut f64,
        var_qex_dn4_slot: &mut f64,
        var_qex_dn5_slot: &mut f64,
        var_qex_dn6_slot: &mut f64,
        var_qex_dn7_slot: &mut f64,
        var_qex_dn8_slot: &mut f64,
        var_qex_dn9_slot: &mut f64,
        var_qex_rdb0_slot: &mut f64,
        var_qex_rdb1_slot: &mut f64,
        var_qex_rdn0_slot: &mut f64,
        var_qex_rdn1_slot: &mut f64,
        var_qex_rdn10_slot: &mut f64,
        var_qex_rdn2_slot: &mut f64,
        var_qex_rdn3_slot: &mut f64,
        var_qex_rdn4_slot: &mut f64,
        var_qex_rdn5_slot: &mut f64,
        var_qex_rdn6_slot: &mut f64,
        var_qex_rdn7_slot: &mut f64,
        var_qex_rdn8_slot: &mut f64,
        var_qex_rdn9_slot: &mut f64,
        var_qex_rv_slot: &mut f64,
        var_tmpexp_slot: &mut f64,
        var_tmpexp_db0_slot: &mut f64,
        var_tmpexp_db1_slot: &mut f64,
        var_tmpexp_dn0_slot: &mut f64,
        var_tmpexp_dn1_slot: &mut f64,
        var_tmpexp_dn10_slot: &mut f64,
        var_tmpexp_dn2_slot: &mut f64,
        var_tmpexp_dn3_slot: &mut f64,
        var_tmpexp_dn4_slot: &mut f64,
        var_tmpexp_dn5_slot: &mut f64,
        var_tmpexp_dn6_slot: &mut f64,
        var_tmpexp_dn7_slot: &mut f64,
        var_tmpexp_dn8_slot: &mut f64,
        var_tmpexp_dn9_slot: &mut f64,
        var_tmpexp_rdb0_slot: &mut f64,
        var_tmpexp_rdb1_slot: &mut f64,
        var_tmpexp_rdn0_slot: &mut f64,
        var_tmpexp_rdn1_slot: &mut f64,
        var_tmpexp_rdn10_slot: &mut f64,
        var_tmpexp_rdn2_slot: &mut f64,
        var_tmpexp_rdn3_slot: &mut f64,
        var_tmpexp_rdn4_slot: &mut f64,
        var_tmpexp_rdn5_slot: &mut f64,
        var_tmpexp_rdn6_slot: &mut f64,
        var_tmpexp_rdn7_slot: &mut f64,
        var_tmpexp_rdn8_slot: &mut f64,
        var_tmpexp_rdn9_slot: &mut f64,
        var_tmpexp_rv_slot: &mut f64,
        var_xqtex_slot: &mut f64,
        var_xqtex_db0_slot: &mut f64,
        var_xqtex_db1_slot: &mut f64,
        var_xqtex_dn0_slot: &mut f64,
        var_xqtex_dn1_slot: &mut f64,
        var_xqtex_dn10_slot: &mut f64,
        var_xqtex_dn2_slot: &mut f64,
        var_xqtex_dn3_slot: &mut f64,
        var_xqtex_dn4_slot: &mut f64,
        var_xqtex_dn5_slot: &mut f64,
        var_xqtex_dn6_slot: &mut f64,
        var_xqtex_dn7_slot: &mut f64,
        var_xqtex_dn8_slot: &mut f64,
        var_xqtex_dn9_slot: &mut f64,
        var_xqtex_rdb0_slot: &mut f64,
        var_xqtex_rdb1_slot: &mut f64,
        var_xqtex_rdn0_slot: &mut f64,
        var_xqtex_rdn1_slot: &mut f64,
        var_xqtex_rdn10_slot: &mut f64,
        var_xqtex_rdn2_slot: &mut f64,
        var_xqtex_rdn3_slot: &mut f64,
        var_xqtex_rdn4_slot: &mut f64,
        var_xqtex_rdn5_slot: &mut f64,
        var_xqtex_rdn6_slot: &mut f64,
        var_xqtex_rdn7_slot: &mut f64,
        var_xqtex_rdn8_slot: &mut f64,
        var_xqtex_rdn9_slot: &mut f64,
        var_xqtex_rv_slot: &mut f64,
        var_xvjcex_slot: &mut f64,
        var_xvjcex_db0_slot: &mut f64,
        var_xvjcex_db1_slot: &mut f64,
        var_xvjcex_dn0_slot: &mut f64,
        var_xvjcex_dn1_slot: &mut f64,
        var_xvjcex_dn10_slot: &mut f64,
        var_xvjcex_dn2_slot: &mut f64,
        var_xvjcex_dn3_slot: &mut f64,
        var_xvjcex_dn4_slot: &mut f64,
        var_xvjcex_dn5_slot: &mut f64,
        var_xvjcex_dn6_slot: &mut f64,
        var_xvjcex_dn7_slot: &mut f64,
        var_xvjcex_dn8_slot: &mut f64,
        var_xvjcex_dn9_slot: &mut f64,
        var_xvjcex_rdb0_slot: &mut f64,
        var_xvjcex_rdb1_slot: &mut f64,
        var_xvjcex_rdn0_slot: &mut f64,
        var_xvjcex_rdn1_slot: &mut f64,
        var_xvjcex_rdn10_slot: &mut f64,
        var_xvjcex_rdn2_slot: &mut f64,
        var_xvjcex_rdn3_slot: &mut f64,
        var_xvjcex_rdn4_slot: &mut f64,
        var_xvjcex_rdn5_slot: &mut f64,
        var_xvjcex_rdn6_slot: &mut f64,
        var_xvjcex_rdn7_slot: &mut f64,
        var_xvjcex_rdn8_slot: &mut f64,
        var_xvjcex_rdn9_slot: &mut f64,
        var_xvjcex_rv_slot: &mut f64,
        var_xvtexv_slot: &mut f64,
        var_xvtexv_db0_slot: &mut f64,
        var_xvtexv_db1_slot: &mut f64,
        var_xvtexv_dn0_slot: &mut f64,
        var_xvtexv_dn1_slot: &mut f64,
        var_xvtexv_dn10_slot: &mut f64,
        var_xvtexv_dn2_slot: &mut f64,
        var_xvtexv_dn3_slot: &mut f64,
        var_xvtexv_dn4_slot: &mut f64,
        var_xvtexv_dn5_slot: &mut f64,
        var_xvtexv_dn6_slot: &mut f64,
        var_xvtexv_dn7_slot: &mut f64,
        var_xvtexv_dn8_slot: &mut f64,
        var_xvtexv_dn9_slot: &mut f64,
        var_xvtexv_rdb0_slot: &mut f64,
        var_xvtexv_rdb1_slot: &mut f64,
        var_xvtexv_rdn0_slot: &mut f64,
        var_xvtexv_rdn1_slot: &mut f64,
        var_xvtexv_rdn10_slot: &mut f64,
        var_xvtexv_rdn2_slot: &mut f64,
        var_xvtexv_rdn3_slot: &mut f64,
        var_xvtexv_rdn4_slot: &mut f64,
        var_xvtexv_rdn5_slot: &mut f64,
        var_xvtexv_rdn6_slot: &mut f64,
        var_xvtexv_rdn7_slot: &mut f64,
        var_xvtexv_rdn8_slot: &mut f64,
        var_xvtexv_rdn9_slot: &mut f64,
        var_xvtexv_rv_slot: &mut f64,
    ) {
        let mut var_expl: f64 = *var_expl_slot;
        let mut var_expl_db0: f64 = *var_expl_db0_slot;
        let mut var_expl_db1: f64 = *var_expl_db1_slot;
        let mut var_expl_dn0: f64 = *var_expl_dn0_slot;
        let mut var_expl_dn1: f64 = *var_expl_dn1_slot;
        let mut var_expl_dn10: f64 = *var_expl_dn10_slot;
        let mut var_expl_dn2: f64 = *var_expl_dn2_slot;
        let mut var_expl_dn3: f64 = *var_expl_dn3_slot;
        let mut var_expl_dn4: f64 = *var_expl_dn4_slot;
        let mut var_expl_dn5: f64 = *var_expl_dn5_slot;
        let mut var_expl_dn6: f64 = *var_expl_dn6_slot;
        let mut var_expl_dn7: f64 = *var_expl_dn7_slot;
        let mut var_expl_dn8: f64 = *var_expl_dn8_slot;
        let mut var_expl_dn9: f64 = *var_expl_dn9_slot;
        let mut var_expl_rdb0: f64 = *var_expl_rdb0_slot;
        let mut var_expl_rdb1: f64 = *var_expl_rdb1_slot;
        let mut var_expl_rdn0: f64 = *var_expl_rdn0_slot;
        let mut var_expl_rdn1: f64 = *var_expl_rdn1_slot;
        let mut var_expl_rdn10: f64 = *var_expl_rdn10_slot;
        let mut var_expl_rdn2: f64 = *var_expl_rdn2_slot;
        let mut var_expl_rdn3: f64 = *var_expl_rdn3_slot;
        let mut var_expl_rdn4: f64 = *var_expl_rdn4_slot;
        let mut var_expl_rdn5: f64 = *var_expl_rdn5_slot;
        let mut var_expl_rdn6: f64 = *var_expl_rdn6_slot;
        let mut var_expl_rdn7: f64 = *var_expl_rdn7_slot;
        let mut var_expl_rdn8: f64 = *var_expl_rdn8_slot;
        let mut var_expl_rdn9: f64 = *var_expl_rdn9_slot;
        let mut var_expl_rv: f64 = *var_expl_rv_slot;
        let mut var_guard106: f64 = *var_guard106_slot;
        let mut var_guard106_db0: f64 = *var_guard106_db0_slot;
        let mut var_guard106_db1: f64 = *var_guard106_db1_slot;
        let mut var_guard106_dn0: f64 = *var_guard106_dn0_slot;
        let mut var_guard106_dn1: f64 = *var_guard106_dn1_slot;
        let mut var_guard106_dn10: f64 = *var_guard106_dn10_slot;
        let mut var_guard106_dn2: f64 = *var_guard106_dn2_slot;
        let mut var_guard106_dn3: f64 = *var_guard106_dn3_slot;
        let mut var_guard106_dn4: f64 = *var_guard106_dn4_slot;
        let mut var_guard106_dn5: f64 = *var_guard106_dn5_slot;
        let mut var_guard106_dn6: f64 = *var_guard106_dn6_slot;
        let mut var_guard106_dn7: f64 = *var_guard106_dn7_slot;
        let mut var_guard106_dn8: f64 = *var_guard106_dn8_slot;
        let mut var_guard106_dn9: f64 = *var_guard106_dn9_slot;
        let mut var_guard106_rdb0: f64 = *var_guard106_rdb0_slot;
        let mut var_guard106_rdb1: f64 = *var_guard106_rdb1_slot;
        let mut var_guard106_rdn0: f64 = *var_guard106_rdn0_slot;
        let mut var_guard106_rdn1: f64 = *var_guard106_rdn1_slot;
        let mut var_guard106_rdn10: f64 = *var_guard106_rdn10_slot;
        let mut var_guard106_rdn2: f64 = *var_guard106_rdn2_slot;
        let mut var_guard106_rdn3: f64 = *var_guard106_rdn3_slot;
        let mut var_guard106_rdn4: f64 = *var_guard106_rdn4_slot;
        let mut var_guard106_rdn5: f64 = *var_guard106_rdn5_slot;
        let mut var_guard106_rdn6: f64 = *var_guard106_rdn6_slot;
        let mut var_guard106_rdn7: f64 = *var_guard106_rdn7_slot;
        let mut var_guard106_rdn8: f64 = *var_guard106_rdn8_slot;
        let mut var_guard106_rdn9: f64 = *var_guard106_rdn9_slot;
        let mut var_guard106_rv: f64 = *var_guard106_rv_slot;
        let mut var_guard107: f64 = *var_guard107_slot;
        let mut var_guard107_db0: f64 = *var_guard107_db0_slot;
        let mut var_guard107_db1: f64 = *var_guard107_db1_slot;
        let mut var_guard107_dn0: f64 = *var_guard107_dn0_slot;
        let mut var_guard107_dn1: f64 = *var_guard107_dn1_slot;
        let mut var_guard107_dn10: f64 = *var_guard107_dn10_slot;
        let mut var_guard107_dn2: f64 = *var_guard107_dn2_slot;
        let mut var_guard107_dn3: f64 = *var_guard107_dn3_slot;
        let mut var_guard107_dn4: f64 = *var_guard107_dn4_slot;
        let mut var_guard107_dn5: f64 = *var_guard107_dn5_slot;
        let mut var_guard107_dn6: f64 = *var_guard107_dn6_slot;
        let mut var_guard107_dn7: f64 = *var_guard107_dn7_slot;
        let mut var_guard107_dn8: f64 = *var_guard107_dn8_slot;
        let mut var_guard107_dn9: f64 = *var_guard107_dn9_slot;
        let mut var_guard107_rdb0: f64 = *var_guard107_rdb0_slot;
        let mut var_guard107_rdb1: f64 = *var_guard107_rdb1_slot;
        let mut var_guard107_rdn0: f64 = *var_guard107_rdn0_slot;
        let mut var_guard107_rdn1: f64 = *var_guard107_rdn1_slot;
        let mut var_guard107_rdn10: f64 = *var_guard107_rdn10_slot;
        let mut var_guard107_rdn2: f64 = *var_guard107_rdn2_slot;
        let mut var_guard107_rdn3: f64 = *var_guard107_rdn3_slot;
        let mut var_guard107_rdn4: f64 = *var_guard107_rdn4_slot;
        let mut var_guard107_rdn5: f64 = *var_guard107_rdn5_slot;
        let mut var_guard107_rdn6: f64 = *var_guard107_rdn6_slot;
        let mut var_guard107_rdn7: f64 = *var_guard107_rdn7_slot;
        let mut var_guard107_rdn8: f64 = *var_guard107_rdn8_slot;
        let mut var_guard107_rdn9: f64 = *var_guard107_rdn9_slot;
        let mut var_guard107_rv: f64 = *var_guard107_rv_slot;
        let mut var_qe0: f64 = *var_qe0_slot;
        let mut var_qe0_db0: f64 = *var_qe0_db0_slot;
        let mut var_qe0_db1: f64 = *var_qe0_db1_slot;
        let mut var_qe0_dn0: f64 = *var_qe0_dn0_slot;
        let mut var_qe0_dn1: f64 = *var_qe0_dn1_slot;
        let mut var_qe0_dn10: f64 = *var_qe0_dn10_slot;
        let mut var_qe0_dn2: f64 = *var_qe0_dn2_slot;
        let mut var_qe0_dn3: f64 = *var_qe0_dn3_slot;
        let mut var_qe0_dn4: f64 = *var_qe0_dn4_slot;
        let mut var_qe0_dn5: f64 = *var_qe0_dn5_slot;
        let mut var_qe0_dn6: f64 = *var_qe0_dn6_slot;
        let mut var_qe0_dn7: f64 = *var_qe0_dn7_slot;
        let mut var_qe0_dn8: f64 = *var_qe0_dn8_slot;
        let mut var_qe0_dn9: f64 = *var_qe0_dn9_slot;
        let mut var_qe0_rdb0: f64 = *var_qe0_rdb0_slot;
        let mut var_qe0_rdb1: f64 = *var_qe0_rdb1_slot;
        let mut var_qe0_rdn0: f64 = *var_qe0_rdn0_slot;
        let mut var_qe0_rdn1: f64 = *var_qe0_rdn1_slot;
        let mut var_qe0_rdn10: f64 = *var_qe0_rdn10_slot;
        let mut var_qe0_rdn2: f64 = *var_qe0_rdn2_slot;
        let mut var_qe0_rdn3: f64 = *var_qe0_rdn3_slot;
        let mut var_qe0_rdn4: f64 = *var_qe0_rdn4_slot;
        let mut var_qe0_rdn5: f64 = *var_qe0_rdn5_slot;
        let mut var_qe0_rdn6: f64 = *var_qe0_rdn6_slot;
        let mut var_qe0_rdn7: f64 = *var_qe0_rdn7_slot;
        let mut var_qe0_rdn8: f64 = *var_qe0_rdn8_slot;
        let mut var_qe0_rdn9: f64 = *var_qe0_rdn9_slot;
        let mut var_qe0_rv: f64 = *var_qe0_rv_slot;
        let mut var_qe_qs: f64 = *var_qe_qs_slot;
        let mut var_qe_qs_db0: f64 = *var_qe_qs_db0_slot;
        let mut var_qe_qs_db1: f64 = *var_qe_qs_db1_slot;
        let mut var_qe_qs_dn0: f64 = *var_qe_qs_dn0_slot;
        let mut var_qe_qs_dn1: f64 = *var_qe_qs_dn1_slot;
        let mut var_qe_qs_dn10: f64 = *var_qe_qs_dn10_slot;
        let mut var_qe_qs_dn2: f64 = *var_qe_qs_dn2_slot;
        let mut var_qe_qs_dn3: f64 = *var_qe_qs_dn3_slot;
        let mut var_qe_qs_dn4: f64 = *var_qe_qs_dn4_slot;
        let mut var_qe_qs_dn5: f64 = *var_qe_qs_dn5_slot;
        let mut var_qe_qs_dn6: f64 = *var_qe_qs_dn6_slot;
        let mut var_qe_qs_dn7: f64 = *var_qe_qs_dn7_slot;
        let mut var_qe_qs_dn8: f64 = *var_qe_qs_dn8_slot;
        let mut var_qe_qs_dn9: f64 = *var_qe_qs_dn9_slot;
        let mut var_qe_qs_rdb0: f64 = *var_qe_qs_rdb0_slot;
        let mut var_qe_qs_rdb1: f64 = *var_qe_qs_rdb1_slot;
        let mut var_qe_qs_rdn0: f64 = *var_qe_qs_rdn0_slot;
        let mut var_qe_qs_rdn1: f64 = *var_qe_qs_rdn1_slot;
        let mut var_qe_qs_rdn10: f64 = *var_qe_qs_rdn10_slot;
        let mut var_qe_qs_rdn2: f64 = *var_qe_qs_rdn2_slot;
        let mut var_qe_qs_rdn3: f64 = *var_qe_qs_rdn3_slot;
        let mut var_qe_qs_rdn4: f64 = *var_qe_qs_rdn4_slot;
        let mut var_qe_qs_rdn5: f64 = *var_qe_qs_rdn5_slot;
        let mut var_qe_qs_rdn6: f64 = *var_qe_qs_rdn6_slot;
        let mut var_qe_qs_rdn7: f64 = *var_qe_qs_rdn7_slot;
        let mut var_qe_qs_rdn8: f64 = *var_qe_qs_rdn8_slot;
        let mut var_qe_qs_rdn9: f64 = *var_qe_qs_rdn9_slot;
        let mut var_qe_qs_rv: f64 = *var_qe_qs_rv_slot;
        let mut var_qepi: f64 = *var_qepi_slot;
        let mut var_qepi0: f64 = *var_qepi0_slot;
        let mut var_qepi0_db0: f64 = *var_qepi0_db0_slot;
        let mut var_qepi0_db1: f64 = *var_qepi0_db1_slot;
        let mut var_qepi0_dn0: f64 = *var_qepi0_dn0_slot;
        let mut var_qepi0_dn1: f64 = *var_qepi0_dn1_slot;
        let mut var_qepi0_dn10: f64 = *var_qepi0_dn10_slot;
        let mut var_qepi0_dn2: f64 = *var_qepi0_dn2_slot;
        let mut var_qepi0_dn3: f64 = *var_qepi0_dn3_slot;
        let mut var_qepi0_dn4: f64 = *var_qepi0_dn4_slot;
        let mut var_qepi0_dn5: f64 = *var_qepi0_dn5_slot;
        let mut var_qepi0_dn6: f64 = *var_qepi0_dn6_slot;
        let mut var_qepi0_dn7: f64 = *var_qepi0_dn7_slot;
        let mut var_qepi0_dn8: f64 = *var_qepi0_dn8_slot;
        let mut var_qepi0_dn9: f64 = *var_qepi0_dn9_slot;
        let mut var_qepi0_rdb0: f64 = *var_qepi0_rdb0_slot;
        let mut var_qepi0_rdb1: f64 = *var_qepi0_rdb1_slot;
        let mut var_qepi0_rdn0: f64 = *var_qepi0_rdn0_slot;
        let mut var_qepi0_rdn1: f64 = *var_qepi0_rdn1_slot;
        let mut var_qepi0_rdn10: f64 = *var_qepi0_rdn10_slot;
        let mut var_qepi0_rdn2: f64 = *var_qepi0_rdn2_slot;
        let mut var_qepi0_rdn3: f64 = *var_qepi0_rdn3_slot;
        let mut var_qepi0_rdn4: f64 = *var_qepi0_rdn4_slot;
        let mut var_qepi0_rdn5: f64 = *var_qepi0_rdn5_slot;
        let mut var_qepi0_rdn6: f64 = *var_qepi0_rdn6_slot;
        let mut var_qepi0_rdn7: f64 = *var_qepi0_rdn7_slot;
        let mut var_qepi0_rdn8: f64 = *var_qepi0_rdn8_slot;
        let mut var_qepi0_rdn9: f64 = *var_qepi0_rdn9_slot;
        let mut var_qepi0_rv: f64 = *var_qepi0_rv_slot;
        let mut var_qepi_db0: f64 = *var_qepi_db0_slot;
        let mut var_qepi_db1: f64 = *var_qepi_db1_slot;
        let mut var_qepi_dn0: f64 = *var_qepi_dn0_slot;
        let mut var_qepi_dn1: f64 = *var_qepi_dn1_slot;
        let mut var_qepi_dn10: f64 = *var_qepi_dn10_slot;
        let mut var_qepi_dn2: f64 = *var_qepi_dn2_slot;
        let mut var_qepi_dn3: f64 = *var_qepi_dn3_slot;
        let mut var_qepi_dn4: f64 = *var_qepi_dn4_slot;
        let mut var_qepi_dn5: f64 = *var_qepi_dn5_slot;
        let mut var_qepi_dn6: f64 = *var_qepi_dn6_slot;
        let mut var_qepi_dn7: f64 = *var_qepi_dn7_slot;
        let mut var_qepi_dn8: f64 = *var_qepi_dn8_slot;
        let mut var_qepi_dn9: f64 = *var_qepi_dn9_slot;
        let mut var_qepi_rdb0: f64 = *var_qepi_rdb0_slot;
        let mut var_qepi_rdb1: f64 = *var_qepi_rdb1_slot;
        let mut var_qepi_rdn0: f64 = *var_qepi_rdn0_slot;
        let mut var_qepi_rdn1: f64 = *var_qepi_rdn1_slot;
        let mut var_qepi_rdn10: f64 = *var_qepi_rdn10_slot;
        let mut var_qepi_rdn2: f64 = *var_qepi_rdn2_slot;
        let mut var_qepi_rdn3: f64 = *var_qepi_rdn3_slot;
        let mut var_qepi_rdn4: f64 = *var_qepi_rdn4_slot;
        let mut var_qepi_rdn5: f64 = *var_qepi_rdn5_slot;
        let mut var_qepi_rdn6: f64 = *var_qepi_rdn6_slot;
        let mut var_qepi_rdn7: f64 = *var_qepi_rdn7_slot;
        let mut var_qepi_rdn8: f64 = *var_qepi_rdn8_slot;
        let mut var_qepi_rdn9: f64 = *var_qepi_rdn9_slot;
        let mut var_qepi_rv: f64 = *var_qepi_rv_slot;
        let mut var_qex: f64 = *var_qex_slot;
        let mut var_qex_db0: f64 = *var_qex_db0_slot;
        let mut var_qex_db1: f64 = *var_qex_db1_slot;
        let mut var_qex_dn0: f64 = *var_qex_dn0_slot;
        let mut var_qex_dn1: f64 = *var_qex_dn1_slot;
        let mut var_qex_dn10: f64 = *var_qex_dn10_slot;
        let mut var_qex_dn2: f64 = *var_qex_dn2_slot;
        let mut var_qex_dn3: f64 = *var_qex_dn3_slot;
        let mut var_qex_dn4: f64 = *var_qex_dn4_slot;
        let mut var_qex_dn5: f64 = *var_qex_dn5_slot;
        let mut var_qex_dn6: f64 = *var_qex_dn6_slot;
        let mut var_qex_dn7: f64 = *var_qex_dn7_slot;
        let mut var_qex_dn8: f64 = *var_qex_dn8_slot;
        let mut var_qex_dn9: f64 = *var_qex_dn9_slot;
        let mut var_qex_rdb0: f64 = *var_qex_rdb0_slot;
        let mut var_qex_rdb1: f64 = *var_qex_rdb1_slot;
        let mut var_qex_rdn0: f64 = *var_qex_rdn0_slot;
        let mut var_qex_rdn1: f64 = *var_qex_rdn1_slot;
        let mut var_qex_rdn10: f64 = *var_qex_rdn10_slot;
        let mut var_qex_rdn2: f64 = *var_qex_rdn2_slot;
        let mut var_qex_rdn3: f64 = *var_qex_rdn3_slot;
        let mut var_qex_rdn4: f64 = *var_qex_rdn4_slot;
        let mut var_qex_rdn5: f64 = *var_qex_rdn5_slot;
        let mut var_qex_rdn6: f64 = *var_qex_rdn6_slot;
        let mut var_qex_rdn7: f64 = *var_qex_rdn7_slot;
        let mut var_qex_rdn8: f64 = *var_qex_rdn8_slot;
        let mut var_qex_rdn9: f64 = *var_qex_rdn9_slot;
        let mut var_qex_rv: f64 = *var_qex_rv_slot;
        let mut var_tmpexp: f64 = *var_tmpexp_slot;
        let mut var_tmpexp_db0: f64 = *var_tmpexp_db0_slot;
        let mut var_tmpexp_db1: f64 = *var_tmpexp_db1_slot;
        let mut var_tmpexp_dn0: f64 = *var_tmpexp_dn0_slot;
        let mut var_tmpexp_dn1: f64 = *var_tmpexp_dn1_slot;
        let mut var_tmpexp_dn10: f64 = *var_tmpexp_dn10_slot;
        let mut var_tmpexp_dn2: f64 = *var_tmpexp_dn2_slot;
        let mut var_tmpexp_dn3: f64 = *var_tmpexp_dn3_slot;
        let mut var_tmpexp_dn4: f64 = *var_tmpexp_dn4_slot;
        let mut var_tmpexp_dn5: f64 = *var_tmpexp_dn5_slot;
        let mut var_tmpexp_dn6: f64 = *var_tmpexp_dn6_slot;
        let mut var_tmpexp_dn7: f64 = *var_tmpexp_dn7_slot;
        let mut var_tmpexp_dn8: f64 = *var_tmpexp_dn8_slot;
        let mut var_tmpexp_dn9: f64 = *var_tmpexp_dn9_slot;
        let mut var_tmpexp_rdb0: f64 = *var_tmpexp_rdb0_slot;
        let mut var_tmpexp_rdb1: f64 = *var_tmpexp_rdb1_slot;
        let mut var_tmpexp_rdn0: f64 = *var_tmpexp_rdn0_slot;
        let mut var_tmpexp_rdn1: f64 = *var_tmpexp_rdn1_slot;
        let mut var_tmpexp_rdn10: f64 = *var_tmpexp_rdn10_slot;
        let mut var_tmpexp_rdn2: f64 = *var_tmpexp_rdn2_slot;
        let mut var_tmpexp_rdn3: f64 = *var_tmpexp_rdn3_slot;
        let mut var_tmpexp_rdn4: f64 = *var_tmpexp_rdn4_slot;
        let mut var_tmpexp_rdn5: f64 = *var_tmpexp_rdn5_slot;
        let mut var_tmpexp_rdn6: f64 = *var_tmpexp_rdn6_slot;
        let mut var_tmpexp_rdn7: f64 = *var_tmpexp_rdn7_slot;
        let mut var_tmpexp_rdn8: f64 = *var_tmpexp_rdn8_slot;
        let mut var_tmpexp_rdn9: f64 = *var_tmpexp_rdn9_slot;
        let mut var_tmpexp_rv: f64 = *var_tmpexp_rv_slot;
        let mut var_xqtex: f64 = *var_xqtex_slot;
        let mut var_xqtex_db0: f64 = *var_xqtex_db0_slot;
        let mut var_xqtex_db1: f64 = *var_xqtex_db1_slot;
        let mut var_xqtex_dn0: f64 = *var_xqtex_dn0_slot;
        let mut var_xqtex_dn1: f64 = *var_xqtex_dn1_slot;
        let mut var_xqtex_dn10: f64 = *var_xqtex_dn10_slot;
        let mut var_xqtex_dn2: f64 = *var_xqtex_dn2_slot;
        let mut var_xqtex_dn3: f64 = *var_xqtex_dn3_slot;
        let mut var_xqtex_dn4: f64 = *var_xqtex_dn4_slot;
        let mut var_xqtex_dn5: f64 = *var_xqtex_dn5_slot;
        let mut var_xqtex_dn6: f64 = *var_xqtex_dn6_slot;
        let mut var_xqtex_dn7: f64 = *var_xqtex_dn7_slot;
        let mut var_xqtex_dn8: f64 = *var_xqtex_dn8_slot;
        let mut var_xqtex_dn9: f64 = *var_xqtex_dn9_slot;
        let mut var_xqtex_rdb0: f64 = *var_xqtex_rdb0_slot;
        let mut var_xqtex_rdb1: f64 = *var_xqtex_rdb1_slot;
        let mut var_xqtex_rdn0: f64 = *var_xqtex_rdn0_slot;
        let mut var_xqtex_rdn1: f64 = *var_xqtex_rdn1_slot;
        let mut var_xqtex_rdn10: f64 = *var_xqtex_rdn10_slot;
        let mut var_xqtex_rdn2: f64 = *var_xqtex_rdn2_slot;
        let mut var_xqtex_rdn3: f64 = *var_xqtex_rdn3_slot;
        let mut var_xqtex_rdn4: f64 = *var_xqtex_rdn4_slot;
        let mut var_xqtex_rdn5: f64 = *var_xqtex_rdn5_slot;
        let mut var_xqtex_rdn6: f64 = *var_xqtex_rdn6_slot;
        let mut var_xqtex_rdn7: f64 = *var_xqtex_rdn7_slot;
        let mut var_xqtex_rdn8: f64 = *var_xqtex_rdn8_slot;
        let mut var_xqtex_rdn9: f64 = *var_xqtex_rdn9_slot;
        let mut var_xqtex_rv: f64 = *var_xqtex_rv_slot;
        let mut var_xvjcex: f64 = *var_xvjcex_slot;
        let mut var_xvjcex_db0: f64 = *var_xvjcex_db0_slot;
        let mut var_xvjcex_db1: f64 = *var_xvjcex_db1_slot;
        let mut var_xvjcex_dn0: f64 = *var_xvjcex_dn0_slot;
        let mut var_xvjcex_dn1: f64 = *var_xvjcex_dn1_slot;
        let mut var_xvjcex_dn10: f64 = *var_xvjcex_dn10_slot;
        let mut var_xvjcex_dn2: f64 = *var_xvjcex_dn2_slot;
        let mut var_xvjcex_dn3: f64 = *var_xvjcex_dn3_slot;
        let mut var_xvjcex_dn4: f64 = *var_xvjcex_dn4_slot;
        let mut var_xvjcex_dn5: f64 = *var_xvjcex_dn5_slot;
        let mut var_xvjcex_dn6: f64 = *var_xvjcex_dn6_slot;
        let mut var_xvjcex_dn7: f64 = *var_xvjcex_dn7_slot;
        let mut var_xvjcex_dn8: f64 = *var_xvjcex_dn8_slot;
        let mut var_xvjcex_dn9: f64 = *var_xvjcex_dn9_slot;
        let mut var_xvjcex_rdb0: f64 = *var_xvjcex_rdb0_slot;
        let mut var_xvjcex_rdb1: f64 = *var_xvjcex_rdb1_slot;
        let mut var_xvjcex_rdn0: f64 = *var_xvjcex_rdn0_slot;
        let mut var_xvjcex_rdn1: f64 = *var_xvjcex_rdn1_slot;
        let mut var_xvjcex_rdn10: f64 = *var_xvjcex_rdn10_slot;
        let mut var_xvjcex_rdn2: f64 = *var_xvjcex_rdn2_slot;
        let mut var_xvjcex_rdn3: f64 = *var_xvjcex_rdn3_slot;
        let mut var_xvjcex_rdn4: f64 = *var_xvjcex_rdn4_slot;
        let mut var_xvjcex_rdn5: f64 = *var_xvjcex_rdn5_slot;
        let mut var_xvjcex_rdn6: f64 = *var_xvjcex_rdn6_slot;
        let mut var_xvjcex_rdn7: f64 = *var_xvjcex_rdn7_slot;
        let mut var_xvjcex_rdn8: f64 = *var_xvjcex_rdn8_slot;
        let mut var_xvjcex_rdn9: f64 = *var_xvjcex_rdn9_slot;
        let mut var_xvjcex_rv: f64 = *var_xvjcex_rv_slot;
        let mut var_xvtexv: f64 = *var_xvtexv_slot;
        let mut var_xvtexv_db0: f64 = *var_xvtexv_db0_slot;
        let mut var_xvtexv_db1: f64 = *var_xvtexv_db1_slot;
        let mut var_xvtexv_dn0: f64 = *var_xvtexv_dn0_slot;
        let mut var_xvtexv_dn1: f64 = *var_xvtexv_dn1_slot;
        let mut var_xvtexv_dn10: f64 = *var_xvtexv_dn10_slot;
        let mut var_xvtexv_dn2: f64 = *var_xvtexv_dn2_slot;
        let mut var_xvtexv_dn3: f64 = *var_xvtexv_dn3_slot;
        let mut var_xvtexv_dn4: f64 = *var_xvtexv_dn4_slot;
        let mut var_xvtexv_dn5: f64 = *var_xvtexv_dn5_slot;
        let mut var_xvtexv_dn6: f64 = *var_xvtexv_dn6_slot;
        let mut var_xvtexv_dn7: f64 = *var_xvtexv_dn7_slot;
        let mut var_xvtexv_dn8: f64 = *var_xvtexv_dn8_slot;
        let mut var_xvtexv_dn9: f64 = *var_xvtexv_dn9_slot;
        let mut var_xvtexv_rdb0: f64 = *var_xvtexv_rdb0_slot;
        let mut var_xvtexv_rdb1: f64 = *var_xvtexv_rdb1_slot;
        let mut var_xvtexv_rdn0: f64 = *var_xvtexv_rdn0_slot;
        let mut var_xvtexv_rdn1: f64 = *var_xvtexv_rdn1_slot;
        let mut var_xvtexv_rdn10: f64 = *var_xvtexv_rdn10_slot;
        let mut var_xvtexv_rdn2: f64 = *var_xvtexv_rdn2_slot;
        let mut var_xvtexv_rdn3: f64 = *var_xvtexv_rdn3_slot;
        let mut var_xvtexv_rdn4: f64 = *var_xvtexv_rdn4_slot;
        let mut var_xvtexv_rdn5: f64 = *var_xvtexv_rdn5_slot;
        let mut var_xvtexv_rdn6: f64 = *var_xvtexv_rdn6_slot;
        let mut var_xvtexv_rdn7: f64 = *var_xvtexv_rdn7_slot;
        let mut var_xvtexv_rdn8: f64 = *var_xvtexv_rdn8_slot;
        let mut var_xvtexv_rdn9: f64 = *var_xvtexv_rdn9_slot;
        let mut var_xvtexv_rv: f64 = *var_xvtexv_rv_slot;

        let (assign5820_e5878, assign5820_e5878_d_n0, assign5820_e5878_d_n1, assign5820_e5878_d_n2, assign5820_e5878_d_n3, assign5820_e5878_d_n4, assign5820_e5878_d_n5, assign5820_e5878_d_n6, assign5820_e5878_d_n7, assign5820_e5878_d_n8, assign5820_e5878_d_n9, assign5820_e5878_d_n10, assign5820_e5878_d_b0, assign5820_e5878_d_b1,) = {
    if (var_guard105 != 0.0) {
        let assign5820_e5872: f64 = (var_dxa).exp();
        let assign5820_e5873: f64 = (1.0 + assign5820_e5872);
        let assign5820_e5874: f64 = (assign5820_e5873).ln();
        let assign5820_e5875: f64 = (var_a_vdcctc * assign5820_e5874);
        let assign5820_e5876: f64 = (var_vbc3 - assign5820_e5875);
        (assign5820_e5876, (var_vbc3_dn0 - ((var_a_vdcctc_dn0 * assign5820_e5874) + (var_a_vdcctc * ((assign5820_e5872 * var_dxa_dn0) / assign5820_e5873)))), (var_vbc3_dn1 - ((var_a_vdcctc_dn1 * assign5820_e5874) + (var_a_vdcctc * ((assign5820_e5872 * var_dxa_dn1) / assign5820_e5873)))), (var_vbc3_dn2 - ((var_a_vdcctc_dn2 * assign5820_e5874) + (var_a_vdcctc * ((assign5820_e5872 * var_dxa_dn2) / assign5820_e5873)))), (var_vbc3_dn3 - ((var_a_vdcctc_dn3 * assign5820_e5874) + (var_a_vdcctc * ((assign5820_e5872 * var_dxa_dn3) / assign5820_e5873)))), (var_vbc3_dn4 - ((var_a_vdcctc_dn4 * assign5820_e5874) + (var_a_vdcctc * ((assign5820_e5872 * var_dxa_dn4) / assign5820_e5873)))), (var_vbc3_dn5 - ((var_a_vdcctc_dn5 * assign5820_e5874) + (var_a_vdcctc * ((assign5820_e5872 * var_dxa_dn5) / assign5820_e5873)))), (var_vbc3_dn6 - ((var_a_vdcctc_dn6 * assign5820_e5874) + (var_a_vdcctc * ((assign5820_e5872 * var_dxa_dn6) / assign5820_e5873)))), (var_vbc3_dn7 - ((var_a_vdcctc_dn7 * assign5820_e5874) + (var_a_vdcctc * ((assign5820_e5872 * var_dxa_dn7) / assign5820_e5873)))), (var_vbc3_dn8 - ((var_a_vdcctc_dn8 * assign5820_e5874) + (var_a_vdcctc * ((assign5820_e5872 * var_dxa_dn8) / assign5820_e5873)))), (var_vbc3_dn9 - ((var_a_vdcctc_dn9 * assign5820_e5874) + (var_a_vdcctc * ((assign5820_e5872 * var_dxa_dn9) / assign5820_e5873)))), (var_vbc3_dn10 - ((var_a_vdcctc_dn10 * assign5820_e5874) + (var_a_vdcctc * ((assign5820_e5872 * var_dxa_dn10) / assign5820_e5873)))), (var_vbc3_db0 - ((var_a_vdcctc_db0 * assign5820_e5874) + (var_a_vdcctc * ((assign5820_e5872 * var_dxa_db0) / assign5820_e5873)))), (var_vbc3_db1 - ((var_a_vdcctc_db1 * assign5820_e5874) + (var_a_vdcctc * ((assign5820_e5872 * var_dxa_db1) / assign5820_e5873)))),)
    } else {
        (var_xvjcex, var_xvjcex_dn0, var_xvjcex_dn1, var_xvjcex_dn2, var_xvjcex_dn3, var_xvjcex_dn4, var_xvjcex_dn5, var_xvjcex_dn6, var_xvjcex_dn7, var_xvjcex_dn8, var_xvjcex_dn9, var_xvjcex_dn10, var_xvjcex_db0, var_xvjcex_db1,)
    }
};
        var_xvjcex = assign5820_e5878;
        var_xvjcex_dn0 = assign5820_e5878_d_n0;
        var_xvjcex_dn1 = assign5820_e5878_d_n1;
        var_xvjcex_dn2 = assign5820_e5878_d_n2;
        var_xvjcex_dn3 = assign5820_e5878_d_n3;
        var_xvjcex_dn4 = assign5820_e5878_d_n4;
        var_xvjcex_dn5 = assign5820_e5878_d_n5;
        var_xvjcex_dn6 = assign5820_e5878_d_n6;
        var_xvjcex_dn7 = assign5820_e5878_d_n7;
        var_xvjcex_dn8 = assign5820_e5878_d_n8;
        var_xvjcex_dn9 = assign5820_e5878_d_n9;
        var_xvjcex_dn10 = assign5820_e5878_d_n10;
        var_xvjcex_db0 = assign5820_e5878_d_b0;
        var_xvjcex_db1 = assign5820_e5878_d_b1;
        var_xvjcex_rv = 0.0;
        var_xvjcex_rdn0 = 0.0;
        var_xvjcex_rdn1 = 0.0;
        var_xvjcex_rdn2 = 0.0;
        var_xvjcex_rdn3 = 0.0;
        var_xvjcex_rdn4 = 0.0;
        var_xvjcex_rdn5 = 0.0;
        var_xvjcex_rdn6 = 0.0;
        var_xvjcex_rdn7 = 0.0;
        var_xvjcex_rdn8 = 0.0;
        var_xvjcex_rdn9 = 0.0;
        var_xvjcex_rdn10 = 0.0;
        var_xvjcex_rdb0 = 0.0;
        var_xvjcex_rdb1 = 0.0;

        let (assign5830_e5892, assign5830_e5892_d_n0, assign5830_e5892_d_n1, assign5830_e5892_d_n2, assign5830_e5892_d_n3, assign5830_e5892_d_n4, assign5830_e5892_d_n5, assign5830_e5892_d_n6, assign5830_e5892_d_n7, assign5830_e5892_d_n8, assign5830_e5892_d_n9, assign5830_e5892_d_n10, assign5830_e5892_d_b0, assign5830_e5892_d_b1,) = {
    if (var_guard105 == 0.0) {
        let assign5830_e5885: f64 = (-var_dxa);
        let assign5830_e5886: f64 = (assign5830_e5885).exp();
        let assign5830_e5887: f64 = (1.0 + assign5830_e5886);
        let assign5830_e5888: f64 = (assign5830_e5887).ln();
        let assign5830_e5889: f64 = (var_a_vdcctc * assign5830_e5888);
        let assign5830_e5890: f64 = (var_vfc - assign5830_e5889);
        (assign5830_e5890, (var_vfc_dn0 - ((var_a_vdcctc_dn0 * assign5830_e5888) + (var_a_vdcctc * ((assign5830_e5886 * (-var_dxa_dn0)) / assign5830_e5887)))), (var_vfc_dn1 - ((var_a_vdcctc_dn1 * assign5830_e5888) + (var_a_vdcctc * ((assign5830_e5886 * (-var_dxa_dn1)) / assign5830_e5887)))), (var_vfc_dn2 - ((var_a_vdcctc_dn2 * assign5830_e5888) + (var_a_vdcctc * ((assign5830_e5886 * (-var_dxa_dn2)) / assign5830_e5887)))), (var_vfc_dn3 - ((var_a_vdcctc_dn3 * assign5830_e5888) + (var_a_vdcctc * ((assign5830_e5886 * (-var_dxa_dn3)) / assign5830_e5887)))), (var_vfc_dn4 - ((var_a_vdcctc_dn4 * assign5830_e5888) + (var_a_vdcctc * ((assign5830_e5886 * (-var_dxa_dn4)) / assign5830_e5887)))), (var_vfc_dn5 - ((var_a_vdcctc_dn5 * assign5830_e5888) + (var_a_vdcctc * ((assign5830_e5886 * (-var_dxa_dn5)) / assign5830_e5887)))), (var_vfc_dn6 - ((var_a_vdcctc_dn6 * assign5830_e5888) + (var_a_vdcctc * ((assign5830_e5886 * (-var_dxa_dn6)) / assign5830_e5887)))), (var_vfc_dn7 - ((var_a_vdcctc_dn7 * assign5830_e5888) + (var_a_vdcctc * ((assign5830_e5886 * (-var_dxa_dn7)) / assign5830_e5887)))), (var_vfc_dn8 - ((var_a_vdcctc_dn8 * assign5830_e5888) + (var_a_vdcctc * ((assign5830_e5886 * (-var_dxa_dn8)) / assign5830_e5887)))), (var_vfc_dn9 - ((var_a_vdcctc_dn9 * assign5830_e5888) + (var_a_vdcctc * ((assign5830_e5886 * (-var_dxa_dn9)) / assign5830_e5887)))), (var_vfc_dn10 - ((var_a_vdcctc_dn10 * assign5830_e5888) + (var_a_vdcctc * ((assign5830_e5886 * (-var_dxa_dn10)) / assign5830_e5887)))), (var_vfc_db0 - ((var_a_vdcctc_db0 * assign5830_e5888) + (var_a_vdcctc * ((assign5830_e5886 * (-var_dxa_db0)) / assign5830_e5887)))), (var_vfc_db1 - ((var_a_vdcctc_db1 * assign5830_e5888) + (var_a_vdcctc * ((assign5830_e5886 * (-var_dxa_db1)) / assign5830_e5887)))),)
    } else {
        (var_xvjcex, var_xvjcex_dn0, var_xvjcex_dn1, var_xvjcex_dn2, var_xvjcex_dn3, var_xvjcex_dn4, var_xvjcex_dn5, var_xvjcex_dn6, var_xvjcex_dn7, var_xvjcex_dn8, var_xvjcex_dn9, var_xvjcex_dn10, var_xvjcex_db0, var_xvjcex_db1,)
    }
};
        var_xvjcex = assign5830_e5892;
        var_xvjcex_dn0 = assign5830_e5892_d_n0;
        var_xvjcex_dn1 = assign5830_e5892_d_n1;
        var_xvjcex_dn2 = assign5830_e5892_d_n2;
        var_xvjcex_dn3 = assign5830_e5892_d_n3;
        var_xvjcex_dn4 = assign5830_e5892_d_n4;
        var_xvjcex_dn5 = assign5830_e5892_d_n5;
        var_xvjcex_dn6 = assign5830_e5892_d_n6;
        var_xvjcex_dn7 = assign5830_e5892_d_n7;
        var_xvjcex_dn8 = assign5830_e5892_d_n8;
        var_xvjcex_dn9 = assign5830_e5892_d_n9;
        var_xvjcex_dn10 = assign5830_e5892_d_n10;
        var_xvjcex_db0 = assign5830_e5892_d_b0;
        var_xvjcex_db1 = assign5830_e5892_d_b1;
        var_xvjcex_rv = 0.0;
        var_xvjcex_rdn0 = 0.0;
        var_xvjcex_rdn1 = 0.0;
        var_xvjcex_rdn2 = 0.0;
        var_xvjcex_rdn3 = 0.0;
        var_xvjcex_rdn4 = 0.0;
        var_xvjcex_rdn5 = 0.0;
        var_xvjcex_rdn6 = 0.0;
        var_xvjcex_rdn7 = 0.0;
        var_xvjcex_rdn8 = 0.0;
        var_xvjcex_rdn9 = 0.0;
        var_xvjcex_rdn10 = 0.0;
        var_xvjcex_rdb0 = 0.0;
        var_xvjcex_rdb1 = 0.0;

        let assign5840_e5896: f64 = (1.0 - p.p71);
        let assign5840_e5897: f64 = (var_vdc_ctc_t / assign5840_e5896);
        let assign5840_e5902: f64 = (var_xvjcex / var_vdc_ctc_t);
        let assign5840_e5903: f64 = (1.0 - assign5840_e5902);
        let assign5840_e5906: f64 = (1.0 - p.p71);
        let assign5840_e5907: f64 = (assign5840_e5903).powf(assign5840_e5906);
        let assign5840_e5908: f64 = (1.0 - assign5840_e5907);
        let assign5840_e5909: f64 = (assign5840_e5897 * assign5840_e5908);
        let assign5840_e5913: f64 = (var_vbc3 - var_xvjcex);
        let assign5840_e5914: f64 = (var_bjc * assign5840_e5913);
        let assign5840_e5915: f64 = (assign5840_e5909 + assign5840_e5914);
        var_xvtexv = assign5840_e5915;
        var_xvtexv_dn0 = ((((var_vdc_ctc_t_dn0 / assign5840_e5896) * assign5840_e5908) + (assign5840_e5897 * (-if 0.0 == 0.0 && ((assign5840_e5906) as f64).is_finite() && ((assign5840_e5906) as f64).fract() == 0.0 { if assign5840_e5906 == 0.0 { 0.0 } else { (assign5840_e5906 * ((assign5840_e5903).powf(assign5840_e5906 - 1.0) * (-(((var_xvjcex_dn0 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn0)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5840_e5907 * (assign5840_e5906 * ((-(((var_xvjcex_dn0 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn0)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5840_e5903))) }))) + ((var_bjc_dn0 * assign5840_e5913) + (var_bjc * (var_vbc3_dn0 - var_xvjcex_dn0))));
        var_xvtexv_dn1 = ((((var_vdc_ctc_t_dn1 / assign5840_e5896) * assign5840_e5908) + (assign5840_e5897 * (-if 0.0 == 0.0 && ((assign5840_e5906) as f64).is_finite() && ((assign5840_e5906) as f64).fract() == 0.0 { if assign5840_e5906 == 0.0 { 0.0 } else { (assign5840_e5906 * ((assign5840_e5903).powf(assign5840_e5906 - 1.0) * (-(((var_xvjcex_dn1 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn1)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5840_e5907 * (assign5840_e5906 * ((-(((var_xvjcex_dn1 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn1)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5840_e5903))) }))) + ((var_bjc_dn1 * assign5840_e5913) + (var_bjc * (var_vbc3_dn1 - var_xvjcex_dn1))));
        var_xvtexv_dn2 = ((((var_vdc_ctc_t_dn2 / assign5840_e5896) * assign5840_e5908) + (assign5840_e5897 * (-if 0.0 == 0.0 && ((assign5840_e5906) as f64).is_finite() && ((assign5840_e5906) as f64).fract() == 0.0 { if assign5840_e5906 == 0.0 { 0.0 } else { (assign5840_e5906 * ((assign5840_e5903).powf(assign5840_e5906 - 1.0) * (-(((var_xvjcex_dn2 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn2)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5840_e5907 * (assign5840_e5906 * ((-(((var_xvjcex_dn2 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn2)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5840_e5903))) }))) + ((var_bjc_dn2 * assign5840_e5913) + (var_bjc * (var_vbc3_dn2 - var_xvjcex_dn2))));
        var_xvtexv_dn3 = ((((var_vdc_ctc_t_dn3 / assign5840_e5896) * assign5840_e5908) + (assign5840_e5897 * (-if 0.0 == 0.0 && ((assign5840_e5906) as f64).is_finite() && ((assign5840_e5906) as f64).fract() == 0.0 { if assign5840_e5906 == 0.0 { 0.0 } else { (assign5840_e5906 * ((assign5840_e5903).powf(assign5840_e5906 - 1.0) * (-(((var_xvjcex_dn3 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn3)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5840_e5907 * (assign5840_e5906 * ((-(((var_xvjcex_dn3 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn3)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5840_e5903))) }))) + ((var_bjc_dn3 * assign5840_e5913) + (var_bjc * (var_vbc3_dn3 - var_xvjcex_dn3))));
        var_xvtexv_dn4 = ((((var_vdc_ctc_t_dn4 / assign5840_e5896) * assign5840_e5908) + (assign5840_e5897 * (-if 0.0 == 0.0 && ((assign5840_e5906) as f64).is_finite() && ((assign5840_e5906) as f64).fract() == 0.0 { if assign5840_e5906 == 0.0 { 0.0 } else { (assign5840_e5906 * ((assign5840_e5903).powf(assign5840_e5906 - 1.0) * (-(((var_xvjcex_dn4 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn4)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5840_e5907 * (assign5840_e5906 * ((-(((var_xvjcex_dn4 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn4)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5840_e5903))) }))) + ((var_bjc_dn4 * assign5840_e5913) + (var_bjc * (var_vbc3_dn4 - var_xvjcex_dn4))));
        var_xvtexv_dn5 = ((((var_vdc_ctc_t_dn5 / assign5840_e5896) * assign5840_e5908) + (assign5840_e5897 * (-if 0.0 == 0.0 && ((assign5840_e5906) as f64).is_finite() && ((assign5840_e5906) as f64).fract() == 0.0 { if assign5840_e5906 == 0.0 { 0.0 } else { (assign5840_e5906 * ((assign5840_e5903).powf(assign5840_e5906 - 1.0) * (-(((var_xvjcex_dn5 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn5)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5840_e5907 * (assign5840_e5906 * ((-(((var_xvjcex_dn5 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn5)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5840_e5903))) }))) + ((var_bjc_dn5 * assign5840_e5913) + (var_bjc * (var_vbc3_dn5 - var_xvjcex_dn5))));
        var_xvtexv_dn6 = ((((var_vdc_ctc_t_dn6 / assign5840_e5896) * assign5840_e5908) + (assign5840_e5897 * (-if 0.0 == 0.0 && ((assign5840_e5906) as f64).is_finite() && ((assign5840_e5906) as f64).fract() == 0.0 { if assign5840_e5906 == 0.0 { 0.0 } else { (assign5840_e5906 * ((assign5840_e5903).powf(assign5840_e5906 - 1.0) * (-(((var_xvjcex_dn6 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn6)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5840_e5907 * (assign5840_e5906 * ((-(((var_xvjcex_dn6 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn6)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5840_e5903))) }))) + ((var_bjc_dn6 * assign5840_e5913) + (var_bjc * (var_vbc3_dn6 - var_xvjcex_dn6))));
        var_xvtexv_dn7 = ((((var_vdc_ctc_t_dn7 / assign5840_e5896) * assign5840_e5908) + (assign5840_e5897 * (-if 0.0 == 0.0 && ((assign5840_e5906) as f64).is_finite() && ((assign5840_e5906) as f64).fract() == 0.0 { if assign5840_e5906 == 0.0 { 0.0 } else { (assign5840_e5906 * ((assign5840_e5903).powf(assign5840_e5906 - 1.0) * (-(((var_xvjcex_dn7 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn7)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5840_e5907 * (assign5840_e5906 * ((-(((var_xvjcex_dn7 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn7)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5840_e5903))) }))) + ((var_bjc_dn7 * assign5840_e5913) + (var_bjc * (var_vbc3_dn7 - var_xvjcex_dn7))));
        var_xvtexv_dn8 = ((((var_vdc_ctc_t_dn8 / assign5840_e5896) * assign5840_e5908) + (assign5840_e5897 * (-if 0.0 == 0.0 && ((assign5840_e5906) as f64).is_finite() && ((assign5840_e5906) as f64).fract() == 0.0 { if assign5840_e5906 == 0.0 { 0.0 } else { (assign5840_e5906 * ((assign5840_e5903).powf(assign5840_e5906 - 1.0) * (-(((var_xvjcex_dn8 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn8)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5840_e5907 * (assign5840_e5906 * ((-(((var_xvjcex_dn8 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn8)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5840_e5903))) }))) + ((var_bjc_dn8 * assign5840_e5913) + (var_bjc * (var_vbc3_dn8 - var_xvjcex_dn8))));
        var_xvtexv_dn9 = ((((var_vdc_ctc_t_dn9 / assign5840_e5896) * assign5840_e5908) + (assign5840_e5897 * (-if 0.0 == 0.0 && ((assign5840_e5906) as f64).is_finite() && ((assign5840_e5906) as f64).fract() == 0.0 { if assign5840_e5906 == 0.0 { 0.0 } else { (assign5840_e5906 * ((assign5840_e5903).powf(assign5840_e5906 - 1.0) * (-(((var_xvjcex_dn9 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn9)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5840_e5907 * (assign5840_e5906 * ((-(((var_xvjcex_dn9 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn9)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5840_e5903))) }))) + ((var_bjc_dn9 * assign5840_e5913) + (var_bjc * (var_vbc3_dn9 - var_xvjcex_dn9))));
        var_xvtexv_dn10 = ((((var_vdc_ctc_t_dn10 / assign5840_e5896) * assign5840_e5908) + (assign5840_e5897 * (-if 0.0 == 0.0 && ((assign5840_e5906) as f64).is_finite() && ((assign5840_e5906) as f64).fract() == 0.0 { if assign5840_e5906 == 0.0 { 0.0 } else { (assign5840_e5906 * ((assign5840_e5903).powf(assign5840_e5906 - 1.0) * (-(((var_xvjcex_dn10 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn10)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5840_e5907 * (assign5840_e5906 * ((-(((var_xvjcex_dn10 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_dn10)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5840_e5903))) }))) + ((var_bjc_dn10 * assign5840_e5913) + (var_bjc * (var_vbc3_dn10 - var_xvjcex_dn10))));
        var_xvtexv_db0 = ((((var_vdc_ctc_t_db0 / assign5840_e5896) * assign5840_e5908) + (assign5840_e5897 * (-if 0.0 == 0.0 && ((assign5840_e5906) as f64).is_finite() && ((assign5840_e5906) as f64).fract() == 0.0 { if assign5840_e5906 == 0.0 { 0.0 } else { (assign5840_e5906 * ((assign5840_e5903).powf(assign5840_e5906 - 1.0) * (-(((var_xvjcex_db0 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_db0)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5840_e5907 * (assign5840_e5906 * ((-(((var_xvjcex_db0 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_db0)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5840_e5903))) }))) + ((var_bjc_db0 * assign5840_e5913) + (var_bjc * (var_vbc3_db0 - var_xvjcex_db0))));
        var_xvtexv_db1 = ((((var_vdc_ctc_t_db1 / assign5840_e5896) * assign5840_e5908) + (assign5840_e5897 * (-if 0.0 == 0.0 && ((assign5840_e5906) as f64).is_finite() && ((assign5840_e5906) as f64).fract() == 0.0 { if assign5840_e5906 == 0.0 { 0.0 } else { (assign5840_e5906 * ((assign5840_e5903).powf(assign5840_e5906 - 1.0) * (-(((var_xvjcex_db1 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_db1)) / (var_vdc_ctc_t * var_vdc_ctc_t))))) } } else { (assign5840_e5907 * (assign5840_e5906 * ((-(((var_xvjcex_db1 * var_vdc_ctc_t) - (var_xvjcex * var_vdc_ctc_t_db1)) / (var_vdc_ctc_t * var_vdc_ctc_t))) / assign5840_e5903))) }))) + ((var_bjc_db1 * assign5840_e5913) + (var_bjc * (var_vbc3_db1 - var_xvjcex_db1))));
        var_xvtexv_rv = 0.0;
        var_xvtexv_rdn0 = 0.0;
        var_xvtexv_rdn1 = 0.0;
        var_xvtexv_rdn2 = 0.0;
        var_xvtexv_rdn3 = 0.0;
        var_xvtexv_rdn4 = 0.0;
        var_xvtexv_rdn5 = 0.0;
        var_xvtexv_rdn6 = 0.0;
        var_xvtexv_rdn7 = 0.0;
        var_xvtexv_rdn8 = 0.0;
        var_xvtexv_rdn9 = 0.0;
        var_xvtexv_rdn10 = 0.0;
        var_xvtexv_rdb0 = 0.0;
        var_xvtexv_rdb1 = 0.0;

        let assign5850_e5919: f64 = (1.0 - var_xp_t);
        let assign5850_e5921: f64 = (assign5850_e5919 * var_xvtexv);
        let assign5850_e5924: f64 = (var_xp_t * var_vbc3);
        let assign5850_e5925: f64 = (assign5850_e5921 + assign5850_e5924);
        let assign5850_e5926: f64 = (var_cjc_t * assign5850_e5925);
        let assign5850_e5929: f64 = (1.0 - p.p76);
        let assign5850_e5930: f64 = (assign5850_e5926 * assign5850_e5929);
        let assign5850_e5932: f64 = (assign5850_e5930 * p.p32);
        var_xqtex = assign5850_e5932;
        var_xqtex_dn0 = ((((var_cjc_t_dn0 * assign5850_e5925) + (var_cjc_t * ((((-var_xp_t_dn0) * var_xvtexv) + (assign5850_e5919 * var_xvtexv_dn0)) + ((var_xp_t_dn0 * var_vbc3) + (var_xp_t * var_vbc3_dn0))))) * assign5850_e5929) * p.p32);
        var_xqtex_dn1 = ((((var_cjc_t_dn1 * assign5850_e5925) + (var_cjc_t * ((((-var_xp_t_dn1) * var_xvtexv) + (assign5850_e5919 * var_xvtexv_dn1)) + ((var_xp_t_dn1 * var_vbc3) + (var_xp_t * var_vbc3_dn1))))) * assign5850_e5929) * p.p32);
        var_xqtex_dn2 = ((((var_cjc_t_dn2 * assign5850_e5925) + (var_cjc_t * ((((-var_xp_t_dn2) * var_xvtexv) + (assign5850_e5919 * var_xvtexv_dn2)) + ((var_xp_t_dn2 * var_vbc3) + (var_xp_t * var_vbc3_dn2))))) * assign5850_e5929) * p.p32);
        var_xqtex_dn3 = ((((var_cjc_t_dn3 * assign5850_e5925) + (var_cjc_t * ((((-var_xp_t_dn3) * var_xvtexv) + (assign5850_e5919 * var_xvtexv_dn3)) + ((var_xp_t_dn3 * var_vbc3) + (var_xp_t * var_vbc3_dn3))))) * assign5850_e5929) * p.p32);
        var_xqtex_dn4 = ((((var_cjc_t_dn4 * assign5850_e5925) + (var_cjc_t * ((((-var_xp_t_dn4) * var_xvtexv) + (assign5850_e5919 * var_xvtexv_dn4)) + ((var_xp_t_dn4 * var_vbc3) + (var_xp_t * var_vbc3_dn4))))) * assign5850_e5929) * p.p32);
        var_xqtex_dn5 = ((((var_cjc_t_dn5 * assign5850_e5925) + (var_cjc_t * ((((-var_xp_t_dn5) * var_xvtexv) + (assign5850_e5919 * var_xvtexv_dn5)) + ((var_xp_t_dn5 * var_vbc3) + (var_xp_t * var_vbc3_dn5))))) * assign5850_e5929) * p.p32);
        var_xqtex_dn6 = ((((var_cjc_t_dn6 * assign5850_e5925) + (var_cjc_t * ((((-var_xp_t_dn6) * var_xvtexv) + (assign5850_e5919 * var_xvtexv_dn6)) + ((var_xp_t_dn6 * var_vbc3) + (var_xp_t * var_vbc3_dn6))))) * assign5850_e5929) * p.p32);
        var_xqtex_dn7 = ((((var_cjc_t_dn7 * assign5850_e5925) + (var_cjc_t * ((((-var_xp_t_dn7) * var_xvtexv) + (assign5850_e5919 * var_xvtexv_dn7)) + ((var_xp_t_dn7 * var_vbc3) + (var_xp_t * var_vbc3_dn7))))) * assign5850_e5929) * p.p32);
        var_xqtex_dn8 = ((((var_cjc_t_dn8 * assign5850_e5925) + (var_cjc_t * ((((-var_xp_t_dn8) * var_xvtexv) + (assign5850_e5919 * var_xvtexv_dn8)) + ((var_xp_t_dn8 * var_vbc3) + (var_xp_t * var_vbc3_dn8))))) * assign5850_e5929) * p.p32);
        var_xqtex_dn9 = ((((var_cjc_t_dn9 * assign5850_e5925) + (var_cjc_t * ((((-var_xp_t_dn9) * var_xvtexv) + (assign5850_e5919 * var_xvtexv_dn9)) + ((var_xp_t_dn9 * var_vbc3) + (var_xp_t * var_vbc3_dn9))))) * assign5850_e5929) * p.p32);
        var_xqtex_dn10 = ((((var_cjc_t_dn10 * assign5850_e5925) + (var_cjc_t * ((((-var_xp_t_dn10) * var_xvtexv) + (assign5850_e5919 * var_xvtexv_dn10)) + ((var_xp_t_dn10 * var_vbc3) + (var_xp_t * var_vbc3_dn10))))) * assign5850_e5929) * p.p32);
        var_xqtex_db0 = ((((var_cjc_t_db0 * assign5850_e5925) + (var_cjc_t * ((((-var_xp_t_db0) * var_xvtexv) + (assign5850_e5919 * var_xvtexv_db0)) + ((var_xp_t_db0 * var_vbc3) + (var_xp_t * var_vbc3_db0))))) * assign5850_e5929) * p.p32);
        var_xqtex_db1 = ((((var_cjc_t_db1 * assign5850_e5925) + (var_cjc_t * ((((-var_xp_t_db1) * var_xvtexv) + (assign5850_e5919 * var_xvtexv_db1)) + ((var_xp_t_db1 * var_vbc3) + (var_xp_t * var_vbc3_db1))))) * assign5850_e5929) * p.p32);
        var_xqtex_rv = 0.0;
        var_xqtex_rdn0 = 0.0;
        var_xqtex_rdn1 = 0.0;
        var_xqtex_rdn2 = 0.0;
        var_xqtex_rdn3 = 0.0;
        var_xqtex_rdn4 = 0.0;
        var_xqtex_rdn5 = 0.0;
        var_xqtex_rdn6 = 0.0;
        var_xqtex_rdn7 = 0.0;
        var_xqtex_rdn8 = 0.0;
        var_xqtex_rdn9 = 0.0;
        var_xqtex_rdn10 = 0.0;
        var_xqtex_rdb0 = 0.0;
        var_xqtex_rdb1 = 0.0;

        let assign5860_e5935: f64 = (var_taue_t * var_ik_t);
        let assign5860_e5938: f64 = (var_is_t / var_ik_t);
        let assign5860_e5941: f64 = (1.0 / p.p84);
        let assign5860_e5942: f64 = (assign5860_e5938).powf(assign5860_e5941);
        let assign5860_e5943: f64 = (assign5860_e5935 * assign5860_e5942);
        var_qe0 = assign5860_e5943;
        var_qe0_dn0 = ((((var_taue_t_dn0 * var_ik_t) + (var_taue_t * var_ik_t_dn0)) * assign5860_e5942) + (assign5860_e5935 * if 0.0 == 0.0 && ((assign5860_e5941) as f64).is_finite() && ((assign5860_e5941) as f64).fract() == 0.0 { if assign5860_e5941 == 0.0 { 0.0 } else { (assign5860_e5941 * ((assign5860_e5938).powf(assign5860_e5941 - 1.0) * (((var_is_t_dn0 * var_ik_t) - (var_is_t * var_ik_t_dn0)) / (var_ik_t * var_ik_t)))) } } else { (assign5860_e5942 * (assign5860_e5941 * ((((var_is_t_dn0 * var_ik_t) - (var_is_t * var_ik_t_dn0)) / (var_ik_t * var_ik_t)) / assign5860_e5938))) }));
        var_qe0_dn1 = ((((var_taue_t_dn1 * var_ik_t) + (var_taue_t * var_ik_t_dn1)) * assign5860_e5942) + (assign5860_e5935 * if 0.0 == 0.0 && ((assign5860_e5941) as f64).is_finite() && ((assign5860_e5941) as f64).fract() == 0.0 { if assign5860_e5941 == 0.0 { 0.0 } else { (assign5860_e5941 * ((assign5860_e5938).powf(assign5860_e5941 - 1.0) * (((var_is_t_dn1 * var_ik_t) - (var_is_t * var_ik_t_dn1)) / (var_ik_t * var_ik_t)))) } } else { (assign5860_e5942 * (assign5860_e5941 * ((((var_is_t_dn1 * var_ik_t) - (var_is_t * var_ik_t_dn1)) / (var_ik_t * var_ik_t)) / assign5860_e5938))) }));
        var_qe0_dn2 = ((((var_taue_t_dn2 * var_ik_t) + (var_taue_t * var_ik_t_dn2)) * assign5860_e5942) + (assign5860_e5935 * if 0.0 == 0.0 && ((assign5860_e5941) as f64).is_finite() && ((assign5860_e5941) as f64).fract() == 0.0 { if assign5860_e5941 == 0.0 { 0.0 } else { (assign5860_e5941 * ((assign5860_e5938).powf(assign5860_e5941 - 1.0) * (((var_is_t_dn2 * var_ik_t) - (var_is_t * var_ik_t_dn2)) / (var_ik_t * var_ik_t)))) } } else { (assign5860_e5942 * (assign5860_e5941 * ((((var_is_t_dn2 * var_ik_t) - (var_is_t * var_ik_t_dn2)) / (var_ik_t * var_ik_t)) / assign5860_e5938))) }));
        var_qe0_dn3 = ((((var_taue_t_dn3 * var_ik_t) + (var_taue_t * var_ik_t_dn3)) * assign5860_e5942) + (assign5860_e5935 * if 0.0 == 0.0 && ((assign5860_e5941) as f64).is_finite() && ((assign5860_e5941) as f64).fract() == 0.0 { if assign5860_e5941 == 0.0 { 0.0 } else { (assign5860_e5941 * ((assign5860_e5938).powf(assign5860_e5941 - 1.0) * (((var_is_t_dn3 * var_ik_t) - (var_is_t * var_ik_t_dn3)) / (var_ik_t * var_ik_t)))) } } else { (assign5860_e5942 * (assign5860_e5941 * ((((var_is_t_dn3 * var_ik_t) - (var_is_t * var_ik_t_dn3)) / (var_ik_t * var_ik_t)) / assign5860_e5938))) }));
        var_qe0_dn4 = ((((var_taue_t_dn4 * var_ik_t) + (var_taue_t * var_ik_t_dn4)) * assign5860_e5942) + (assign5860_e5935 * if 0.0 == 0.0 && ((assign5860_e5941) as f64).is_finite() && ((assign5860_e5941) as f64).fract() == 0.0 { if assign5860_e5941 == 0.0 { 0.0 } else { (assign5860_e5941 * ((assign5860_e5938).powf(assign5860_e5941 - 1.0) * (((var_is_t_dn4 * var_ik_t) - (var_is_t * var_ik_t_dn4)) / (var_ik_t * var_ik_t)))) } } else { (assign5860_e5942 * (assign5860_e5941 * ((((var_is_t_dn4 * var_ik_t) - (var_is_t * var_ik_t_dn4)) / (var_ik_t * var_ik_t)) / assign5860_e5938))) }));
        var_qe0_dn5 = ((((var_taue_t_dn5 * var_ik_t) + (var_taue_t * var_ik_t_dn5)) * assign5860_e5942) + (assign5860_e5935 * if 0.0 == 0.0 && ((assign5860_e5941) as f64).is_finite() && ((assign5860_e5941) as f64).fract() == 0.0 { if assign5860_e5941 == 0.0 { 0.0 } else { (assign5860_e5941 * ((assign5860_e5938).powf(assign5860_e5941 - 1.0) * (((var_is_t_dn5 * var_ik_t) - (var_is_t * var_ik_t_dn5)) / (var_ik_t * var_ik_t)))) } } else { (assign5860_e5942 * (assign5860_e5941 * ((((var_is_t_dn5 * var_ik_t) - (var_is_t * var_ik_t_dn5)) / (var_ik_t * var_ik_t)) / assign5860_e5938))) }));
        var_qe0_dn6 = ((((var_taue_t_dn6 * var_ik_t) + (var_taue_t * var_ik_t_dn6)) * assign5860_e5942) + (assign5860_e5935 * if 0.0 == 0.0 && ((assign5860_e5941) as f64).is_finite() && ((assign5860_e5941) as f64).fract() == 0.0 { if assign5860_e5941 == 0.0 { 0.0 } else { (assign5860_e5941 * ((assign5860_e5938).powf(assign5860_e5941 - 1.0) * (((var_is_t_dn6 * var_ik_t) - (var_is_t * var_ik_t_dn6)) / (var_ik_t * var_ik_t)))) } } else { (assign5860_e5942 * (assign5860_e5941 * ((((var_is_t_dn6 * var_ik_t) - (var_is_t * var_ik_t_dn6)) / (var_ik_t * var_ik_t)) / assign5860_e5938))) }));
        var_qe0_dn7 = ((((var_taue_t_dn7 * var_ik_t) + (var_taue_t * var_ik_t_dn7)) * assign5860_e5942) + (assign5860_e5935 * if 0.0 == 0.0 && ((assign5860_e5941) as f64).is_finite() && ((assign5860_e5941) as f64).fract() == 0.0 { if assign5860_e5941 == 0.0 { 0.0 } else { (assign5860_e5941 * ((assign5860_e5938).powf(assign5860_e5941 - 1.0) * (((var_is_t_dn7 * var_ik_t) - (var_is_t * var_ik_t_dn7)) / (var_ik_t * var_ik_t)))) } } else { (assign5860_e5942 * (assign5860_e5941 * ((((var_is_t_dn7 * var_ik_t) - (var_is_t * var_ik_t_dn7)) / (var_ik_t * var_ik_t)) / assign5860_e5938))) }));
        var_qe0_dn8 = ((((var_taue_t_dn8 * var_ik_t) + (var_taue_t * var_ik_t_dn8)) * assign5860_e5942) + (assign5860_e5935 * if 0.0 == 0.0 && ((assign5860_e5941) as f64).is_finite() && ((assign5860_e5941) as f64).fract() == 0.0 { if assign5860_e5941 == 0.0 { 0.0 } else { (assign5860_e5941 * ((assign5860_e5938).powf(assign5860_e5941 - 1.0) * (((var_is_t_dn8 * var_ik_t) - (var_is_t * var_ik_t_dn8)) / (var_ik_t * var_ik_t)))) } } else { (assign5860_e5942 * (assign5860_e5941 * ((((var_is_t_dn8 * var_ik_t) - (var_is_t * var_ik_t_dn8)) / (var_ik_t * var_ik_t)) / assign5860_e5938))) }));
        var_qe0_dn9 = ((((var_taue_t_dn9 * var_ik_t) + (var_taue_t * var_ik_t_dn9)) * assign5860_e5942) + (assign5860_e5935 * if 0.0 == 0.0 && ((assign5860_e5941) as f64).is_finite() && ((assign5860_e5941) as f64).fract() == 0.0 { if assign5860_e5941 == 0.0 { 0.0 } else { (assign5860_e5941 * ((assign5860_e5938).powf(assign5860_e5941 - 1.0) * (((var_is_t_dn9 * var_ik_t) - (var_is_t * var_ik_t_dn9)) / (var_ik_t * var_ik_t)))) } } else { (assign5860_e5942 * (assign5860_e5941 * ((((var_is_t_dn9 * var_ik_t) - (var_is_t * var_ik_t_dn9)) / (var_ik_t * var_ik_t)) / assign5860_e5938))) }));
        var_qe0_dn10 = ((((var_taue_t_dn10 * var_ik_t) + (var_taue_t * var_ik_t_dn10)) * assign5860_e5942) + (assign5860_e5935 * if 0.0 == 0.0 && ((assign5860_e5941) as f64).is_finite() && ((assign5860_e5941) as f64).fract() == 0.0 { if assign5860_e5941 == 0.0 { 0.0 } else { (assign5860_e5941 * ((assign5860_e5938).powf(assign5860_e5941 - 1.0) * (((var_is_t_dn10 * var_ik_t) - (var_is_t * var_ik_t_dn10)) / (var_ik_t * var_ik_t)))) } } else { (assign5860_e5942 * (assign5860_e5941 * ((((var_is_t_dn10 * var_ik_t) - (var_is_t * var_ik_t_dn10)) / (var_ik_t * var_ik_t)) / assign5860_e5938))) }));
        var_qe0_db0 = ((((var_taue_t_db0 * var_ik_t) + (var_taue_t * var_ik_t_db0)) * assign5860_e5942) + (assign5860_e5935 * if 0.0 == 0.0 && ((assign5860_e5941) as f64).is_finite() && ((assign5860_e5941) as f64).fract() == 0.0 { if assign5860_e5941 == 0.0 { 0.0 } else { (assign5860_e5941 * ((assign5860_e5938).powf(assign5860_e5941 - 1.0) * (((var_is_t_db0 * var_ik_t) - (var_is_t * var_ik_t_db0)) / (var_ik_t * var_ik_t)))) } } else { (assign5860_e5942 * (assign5860_e5941 * ((((var_is_t_db0 * var_ik_t) - (var_is_t * var_ik_t_db0)) / (var_ik_t * var_ik_t)) / assign5860_e5938))) }));
        var_qe0_db1 = ((((var_taue_t_db1 * var_ik_t) + (var_taue_t * var_ik_t_db1)) * assign5860_e5942) + (assign5860_e5935 * if 0.0 == 0.0 && ((assign5860_e5941) as f64).is_finite() && ((assign5860_e5941) as f64).fract() == 0.0 { if assign5860_e5941 == 0.0 { 0.0 } else { (assign5860_e5941 * ((assign5860_e5938).powf(assign5860_e5941 - 1.0) * (((var_is_t_db1 * var_ik_t) - (var_is_t * var_ik_t_db1)) / (var_ik_t * var_ik_t)))) } } else { (assign5860_e5942 * (assign5860_e5941 * ((((var_is_t_db1 * var_ik_t) - (var_is_t * var_ik_t_db1)) / (var_ik_t * var_ik_t)) / assign5860_e5938))) }));
        var_qe0_rv = 0.0;
        var_qe0_rdn0 = 0.0;
        var_qe0_rdn1 = 0.0;
        var_qe0_rdn2 = 0.0;
        var_qe0_rdn3 = 0.0;
        var_qe0_rdn4 = 0.0;
        var_qe0_rdn5 = 0.0;
        var_qe0_rdn6 = 0.0;
        var_qe0_rdn7 = 0.0;
        var_qe0_rdn8 = 0.0;
        var_qe0_rdn9 = 0.0;
        var_qe0_rdn10 = 0.0;
        var_qe0_rdb0 = 0.0;
        var_qe0_rdb1 = 0.0;

        let assign5870_e5947: f64 = (p.p84 * var_vt);
        let assign5870_e5948: f64 = (var_vb2e1 / assign5870_e5947);
        let assign5870_e5950: f64 = if assign5870_e5948 < p.p134 { 1.0 } else { 0.0 };
        var_guard106 = assign5870_e5950;
        var_guard106_dn0 = 0.0;
        var_guard106_dn1 = 0.0;
        var_guard106_dn2 = 0.0;
        var_guard106_dn3 = 0.0;
        var_guard106_dn4 = 0.0;
        var_guard106_dn5 = 0.0;
        var_guard106_dn6 = 0.0;
        var_guard106_dn7 = 0.0;
        var_guard106_dn8 = 0.0;
        var_guard106_dn9 = 0.0;
        var_guard106_dn10 = 0.0;
        var_guard106_db0 = 0.0;
        var_guard106_db1 = 0.0;
        var_guard106_rv = 0.0;
        var_guard106_rdn0 = 0.0;
        var_guard106_rdn1 = 0.0;
        var_guard106_rdn2 = 0.0;
        var_guard106_rdn3 = 0.0;
        var_guard106_rdn4 = 0.0;
        var_guard106_rdn5 = 0.0;
        var_guard106_rdn6 = 0.0;
        var_guard106_rdn7 = 0.0;
        var_guard106_rdn8 = 0.0;
        var_guard106_rdn9 = 0.0;
        var_guard106_rdn10 = 0.0;
        var_guard106_rdb0 = 0.0;
        var_guard106_rdb1 = 0.0;

        let (assign5880_e5959, assign5880_e5959_d_n0, assign5880_e5959_d_n1, assign5880_e5959_d_n2, assign5880_e5959_d_n3, assign5880_e5959_d_n4, assign5880_e5959_d_n5, assign5880_e5959_d_n6, assign5880_e5959_d_n7, assign5880_e5959_d_n8, assign5880_e5959_d_n9, assign5880_e5959_d_n10, assign5880_e5959_d_b0, assign5880_e5959_d_b1,) = {
    if (var_guard106 != 0.0) {
        let assign5880_e5955: f64 = (p.p84 * var_vt);
        let assign5880_e5956: f64 = (var_vb2e1 / assign5880_e5955);
        let assign5880_e5957: f64 = (assign5880_e5956).exp();
        (assign5880_e5957, (assign5880_e5957 * (((var_vb2e1_dn0 * assign5880_e5955) - (var_vb2e1 * (p.p84 * var_vt_dn0))) / (assign5880_e5955 * assign5880_e5955))), (assign5880_e5957 * (((var_vb2e1_dn1 * assign5880_e5955) - (var_vb2e1 * (p.p84 * var_vt_dn1))) / (assign5880_e5955 * assign5880_e5955))), (assign5880_e5957 * (((var_vb2e1_dn2 * assign5880_e5955) - (var_vb2e1 * (p.p84 * var_vt_dn2))) / (assign5880_e5955 * assign5880_e5955))), (assign5880_e5957 * (((var_vb2e1_dn3 * assign5880_e5955) - (var_vb2e1 * (p.p84 * var_vt_dn3))) / (assign5880_e5955 * assign5880_e5955))), (assign5880_e5957 * (((var_vb2e1_dn4 * assign5880_e5955) - (var_vb2e1 * (p.p84 * var_vt_dn4))) / (assign5880_e5955 * assign5880_e5955))), (assign5880_e5957 * (((var_vb2e1_dn5 * assign5880_e5955) - (var_vb2e1 * (p.p84 * var_vt_dn5))) / (assign5880_e5955 * assign5880_e5955))), (assign5880_e5957 * (((var_vb2e1_dn6 * assign5880_e5955) - (var_vb2e1 * (p.p84 * var_vt_dn6))) / (assign5880_e5955 * assign5880_e5955))), (assign5880_e5957 * (((var_vb2e1_dn7 * assign5880_e5955) - (var_vb2e1 * (p.p84 * var_vt_dn7))) / (assign5880_e5955 * assign5880_e5955))), (assign5880_e5957 * (((var_vb2e1_dn8 * assign5880_e5955) - (var_vb2e1 * (p.p84 * var_vt_dn8))) / (assign5880_e5955 * assign5880_e5955))), (assign5880_e5957 * (((var_vb2e1_dn9 * assign5880_e5955) - (var_vb2e1 * (p.p84 * var_vt_dn9))) / (assign5880_e5955 * assign5880_e5955))), (assign5880_e5957 * (((var_vb2e1_dn10 * assign5880_e5955) - (var_vb2e1 * (p.p84 * var_vt_dn10))) / (assign5880_e5955 * assign5880_e5955))), (assign5880_e5957 * (((var_vb2e1_db0 * assign5880_e5955) - (var_vb2e1 * (p.p84 * var_vt_db0))) / (assign5880_e5955 * assign5880_e5955))), (assign5880_e5957 * (((var_vb2e1_db1 * assign5880_e5955) - (var_vb2e1 * (p.p84 * var_vt_db1))) / (assign5880_e5955 * assign5880_e5955))),)
    } else {
        (var_tmpexp, var_tmpexp_dn0, var_tmpexp_dn1, var_tmpexp_dn2, var_tmpexp_dn3, var_tmpexp_dn4, var_tmpexp_dn5, var_tmpexp_dn6, var_tmpexp_dn7, var_tmpexp_dn8, var_tmpexp_dn9, var_tmpexp_dn10, var_tmpexp_db0, var_tmpexp_db1,)
    }
};
        var_tmpexp = assign5880_e5959;
        var_tmpexp_dn0 = assign5880_e5959_d_n0;
        var_tmpexp_dn1 = assign5880_e5959_d_n1;
        var_tmpexp_dn2 = assign5880_e5959_d_n2;
        var_tmpexp_dn3 = assign5880_e5959_d_n3;
        var_tmpexp_dn4 = assign5880_e5959_d_n4;
        var_tmpexp_dn5 = assign5880_e5959_d_n5;
        var_tmpexp_dn6 = assign5880_e5959_d_n6;
        var_tmpexp_dn7 = assign5880_e5959_d_n7;
        var_tmpexp_dn8 = assign5880_e5959_d_n8;
        var_tmpexp_dn9 = assign5880_e5959_d_n9;
        var_tmpexp_dn10 = assign5880_e5959_d_n10;
        var_tmpexp_db0 = assign5880_e5959_d_b0;
        var_tmpexp_db1 = assign5880_e5959_d_b1;
        var_tmpexp_rv = 0.0;
        var_tmpexp_rdn0 = 0.0;
        var_tmpexp_rdn1 = 0.0;
        var_tmpexp_rdn2 = 0.0;
        var_tmpexp_rdn3 = 0.0;
        var_tmpexp_rdn4 = 0.0;
        var_tmpexp_rdn5 = 0.0;
        var_tmpexp_rdn6 = 0.0;
        var_tmpexp_rdn7 = 0.0;
        var_tmpexp_rdn8 = 0.0;
        var_tmpexp_rdn9 = 0.0;
        var_tmpexp_rdn10 = 0.0;
        var_tmpexp_rdb0 = 0.0;
        var_tmpexp_rdb1 = 0.0;

        let (assign5890_e5965, assign5890_e5965_d_n0, assign5890_e5965_d_n1, assign5890_e5965_d_n2, assign5890_e5965_d_n3, assign5890_e5965_d_n4, assign5890_e5965_d_n5, assign5890_e5965_d_n6, assign5890_e5965_d_n7, assign5890_e5965_d_n8, assign5890_e5965_d_n9, assign5890_e5965_d_n10, assign5890_e5965_d_b0, assign5890_e5965_d_b1,) = {
    if (var_guard106 == 0.0) {
        let assign5890_e5963: f64 = (p.p134).exp();
        (assign5890_e5963, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_expl, var_expl_dn0, var_expl_dn1, var_expl_dn2, var_expl_dn3, var_expl_dn4, var_expl_dn5, var_expl_dn6, var_expl_dn7, var_expl_dn8, var_expl_dn9, var_expl_dn10, var_expl_db0, var_expl_db1,)
    }
};
        var_expl = assign5890_e5965;
        var_expl_dn0 = assign5890_e5965_d_n0;
        var_expl_dn1 = assign5890_e5965_d_n1;
        var_expl_dn2 = assign5890_e5965_d_n2;
        var_expl_dn3 = assign5890_e5965_d_n3;
        var_expl_dn4 = assign5890_e5965_d_n4;
        var_expl_dn5 = assign5890_e5965_d_n5;
        var_expl_dn6 = assign5890_e5965_d_n6;
        var_expl_dn7 = assign5890_e5965_d_n7;
        var_expl_dn8 = assign5890_e5965_d_n8;
        var_expl_dn9 = assign5890_e5965_d_n9;
        var_expl_dn10 = assign5890_e5965_d_n10;
        var_expl_db0 = assign5890_e5965_d_b0;
        var_expl_db1 = assign5890_e5965_d_b1;
        var_expl_rv = 0.0;
        var_expl_rdn0 = 0.0;
        var_expl_rdn1 = 0.0;
        var_expl_rdn2 = 0.0;
        var_expl_rdn3 = 0.0;
        var_expl_rdn4 = 0.0;
        var_expl_rdn5 = 0.0;
        var_expl_rdn6 = 0.0;
        var_expl_rdn7 = 0.0;
        var_expl_rdn8 = 0.0;
        var_expl_rdn9 = 0.0;
        var_expl_rdn10 = 0.0;
        var_expl_rdb0 = 0.0;
        var_expl_rdb1 = 0.0;

        let (assign5900_e5980, assign5900_e5980_d_n0, assign5900_e5980_d_n1, assign5900_e5980_d_n2, assign5900_e5980_d_n3, assign5900_e5980_d_n4, assign5900_e5980_d_n5, assign5900_e5980_d_n6, assign5900_e5980_d_n7, assign5900_e5980_d_n8, assign5900_e5980_d_n9, assign5900_e5980_d_n10, assign5900_e5980_d_b0, assign5900_e5980_d_b1,) = {
    if (var_guard106 == 0.0) {
        let assign5900_e5973: f64 = (p.p84 * var_vt);
        let assign5900_e5974: f64 = (var_vb2e1 / assign5900_e5973);
        let assign5900_e5976: f64 = (assign5900_e5974 - p.p134);
        let assign5900_e5977: f64 = (1.0 + assign5900_e5976);
        let assign5900_e5978: f64 = (var_expl * assign5900_e5977);
        (assign5900_e5978, ((var_expl_dn0 * assign5900_e5977) + (var_expl * (((var_vb2e1_dn0 * assign5900_e5973) - (var_vb2e1 * (p.p84 * var_vt_dn0))) / (assign5900_e5973 * assign5900_e5973)))), ((var_expl_dn1 * assign5900_e5977) + (var_expl * (((var_vb2e1_dn1 * assign5900_e5973) - (var_vb2e1 * (p.p84 * var_vt_dn1))) / (assign5900_e5973 * assign5900_e5973)))), ((var_expl_dn2 * assign5900_e5977) + (var_expl * (((var_vb2e1_dn2 * assign5900_e5973) - (var_vb2e1 * (p.p84 * var_vt_dn2))) / (assign5900_e5973 * assign5900_e5973)))), ((var_expl_dn3 * assign5900_e5977) + (var_expl * (((var_vb2e1_dn3 * assign5900_e5973) - (var_vb2e1 * (p.p84 * var_vt_dn3))) / (assign5900_e5973 * assign5900_e5973)))), ((var_expl_dn4 * assign5900_e5977) + (var_expl * (((var_vb2e1_dn4 * assign5900_e5973) - (var_vb2e1 * (p.p84 * var_vt_dn4))) / (assign5900_e5973 * assign5900_e5973)))), ((var_expl_dn5 * assign5900_e5977) + (var_expl * (((var_vb2e1_dn5 * assign5900_e5973) - (var_vb2e1 * (p.p84 * var_vt_dn5))) / (assign5900_e5973 * assign5900_e5973)))), ((var_expl_dn6 * assign5900_e5977) + (var_expl * (((var_vb2e1_dn6 * assign5900_e5973) - (var_vb2e1 * (p.p84 * var_vt_dn6))) / (assign5900_e5973 * assign5900_e5973)))), ((var_expl_dn7 * assign5900_e5977) + (var_expl * (((var_vb2e1_dn7 * assign5900_e5973) - (var_vb2e1 * (p.p84 * var_vt_dn7))) / (assign5900_e5973 * assign5900_e5973)))), ((var_expl_dn8 * assign5900_e5977) + (var_expl * (((var_vb2e1_dn8 * assign5900_e5973) - (var_vb2e1 * (p.p84 * var_vt_dn8))) / (assign5900_e5973 * assign5900_e5973)))), ((var_expl_dn9 * assign5900_e5977) + (var_expl * (((var_vb2e1_dn9 * assign5900_e5973) - (var_vb2e1 * (p.p84 * var_vt_dn9))) / (assign5900_e5973 * assign5900_e5973)))), ((var_expl_dn10 * assign5900_e5977) + (var_expl * (((var_vb2e1_dn10 * assign5900_e5973) - (var_vb2e1 * (p.p84 * var_vt_dn10))) / (assign5900_e5973 * assign5900_e5973)))), ((var_expl_db0 * assign5900_e5977) + (var_expl * (((var_vb2e1_db0 * assign5900_e5973) - (var_vb2e1 * (p.p84 * var_vt_db0))) / (assign5900_e5973 * assign5900_e5973)))), ((var_expl_db1 * assign5900_e5977) + (var_expl * (((var_vb2e1_db1 * assign5900_e5973) - (var_vb2e1 * (p.p84 * var_vt_db1))) / (assign5900_e5973 * assign5900_e5973)))),)
    } else {
        (var_tmpexp, var_tmpexp_dn0, var_tmpexp_dn1, var_tmpexp_dn2, var_tmpexp_dn3, var_tmpexp_dn4, var_tmpexp_dn5, var_tmpexp_dn6, var_tmpexp_dn7, var_tmpexp_dn8, var_tmpexp_dn9, var_tmpexp_dn10, var_tmpexp_db0, var_tmpexp_db1,)
    }
};
        var_tmpexp = assign5900_e5980;
        var_tmpexp_dn0 = assign5900_e5980_d_n0;
        var_tmpexp_dn1 = assign5900_e5980_d_n1;
        var_tmpexp_dn2 = assign5900_e5980_d_n2;
        var_tmpexp_dn3 = assign5900_e5980_d_n3;
        var_tmpexp_dn4 = assign5900_e5980_d_n4;
        var_tmpexp_dn5 = assign5900_e5980_d_n5;
        var_tmpexp_dn6 = assign5900_e5980_d_n6;
        var_tmpexp_dn7 = assign5900_e5980_d_n7;
        var_tmpexp_dn8 = assign5900_e5980_d_n8;
        var_tmpexp_dn9 = assign5900_e5980_d_n9;
        var_tmpexp_dn10 = assign5900_e5980_d_n10;
        var_tmpexp_db0 = assign5900_e5980_d_b0;
        var_tmpexp_db1 = assign5900_e5980_d_b1;
        var_tmpexp_rv = 0.0;
        var_tmpexp_rdn0 = 0.0;
        var_tmpexp_rdn1 = 0.0;
        var_tmpexp_rdn2 = 0.0;
        var_tmpexp_rdn3 = 0.0;
        var_tmpexp_rdn4 = 0.0;
        var_tmpexp_rdn5 = 0.0;
        var_tmpexp_rdn6 = 0.0;
        var_tmpexp_rdn7 = 0.0;
        var_tmpexp_rdn8 = 0.0;
        var_tmpexp_rdn9 = 0.0;
        var_tmpexp_rdn10 = 0.0;
        var_tmpexp_rdb0 = 0.0;
        var_tmpexp_rdb1 = 0.0;

        let assign5910_e5983: f64 = (var_qe0 * var_tmpexp);
        var_qe_qs = assign5910_e5983;
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
        var_qe_qs_db0 = ((var_qe0_db0 * var_tmpexp) + (var_qe0 * var_tmpexp_db0));
        var_qe_qs_db1 = ((var_qe0_db1 * var_tmpexp) + (var_qe0 * var_tmpexp_db1));
        var_qe_qs_rv = 0.0;
        var_qe_qs_rdn0 = 0.0;
        var_qe_qs_rdn1 = 0.0;
        var_qe_qs_rdn2 = 0.0;
        var_qe_qs_rdn3 = 0.0;
        var_qe_qs_rdn4 = 0.0;
        var_qe_qs_rdn5 = 0.0;
        var_qe_qs_rdn6 = 0.0;
        var_qe_qs_rdn7 = 0.0;
        var_qe_qs_rdn8 = 0.0;
        var_qe_qs_rdn9 = 0.0;
        var_qe_qs_rdn10 = 0.0;
        var_qe_qs_rdb0 = 0.0;
        var_qe_qs_rdb1 = 0.0;

        let assign5920_e5986: f64 = (4.0 * var_tepi_t);
        let assign5920_e5988: f64 = (assign5920_e5986 * var_vt);
        let assign5920_e5990: f64 = (assign5920_e5988 / var_rcv_t);
        var_qepi0 = assign5920_e5990;
        var_qepi0_dn0 = ((((((4.0 * var_tepi_t_dn0) * var_vt) + (assign5920_e5986 * var_vt_dn0)) * var_rcv_t) - (assign5920_e5988 * var_rcv_t_dn0)) / (var_rcv_t * var_rcv_t));
        var_qepi0_dn1 = ((((((4.0 * var_tepi_t_dn1) * var_vt) + (assign5920_e5986 * var_vt_dn1)) * var_rcv_t) - (assign5920_e5988 * var_rcv_t_dn1)) / (var_rcv_t * var_rcv_t));
        var_qepi0_dn2 = ((((((4.0 * var_tepi_t_dn2) * var_vt) + (assign5920_e5986 * var_vt_dn2)) * var_rcv_t) - (assign5920_e5988 * var_rcv_t_dn2)) / (var_rcv_t * var_rcv_t));
        var_qepi0_dn3 = ((((((4.0 * var_tepi_t_dn3) * var_vt) + (assign5920_e5986 * var_vt_dn3)) * var_rcv_t) - (assign5920_e5988 * var_rcv_t_dn3)) / (var_rcv_t * var_rcv_t));
        var_qepi0_dn4 = ((((((4.0 * var_tepi_t_dn4) * var_vt) + (assign5920_e5986 * var_vt_dn4)) * var_rcv_t) - (assign5920_e5988 * var_rcv_t_dn4)) / (var_rcv_t * var_rcv_t));
        var_qepi0_dn5 = ((((((4.0 * var_tepi_t_dn5) * var_vt) + (assign5920_e5986 * var_vt_dn5)) * var_rcv_t) - (assign5920_e5988 * var_rcv_t_dn5)) / (var_rcv_t * var_rcv_t));
        var_qepi0_dn6 = ((((((4.0 * var_tepi_t_dn6) * var_vt) + (assign5920_e5986 * var_vt_dn6)) * var_rcv_t) - (assign5920_e5988 * var_rcv_t_dn6)) / (var_rcv_t * var_rcv_t));
        var_qepi0_dn7 = ((((((4.0 * var_tepi_t_dn7) * var_vt) + (assign5920_e5986 * var_vt_dn7)) * var_rcv_t) - (assign5920_e5988 * var_rcv_t_dn7)) / (var_rcv_t * var_rcv_t));
        var_qepi0_dn8 = ((((((4.0 * var_tepi_t_dn8) * var_vt) + (assign5920_e5986 * var_vt_dn8)) * var_rcv_t) - (assign5920_e5988 * var_rcv_t_dn8)) / (var_rcv_t * var_rcv_t));
        var_qepi0_dn9 = ((((((4.0 * var_tepi_t_dn9) * var_vt) + (assign5920_e5986 * var_vt_dn9)) * var_rcv_t) - (assign5920_e5988 * var_rcv_t_dn9)) / (var_rcv_t * var_rcv_t));
        var_qepi0_dn10 = ((((((4.0 * var_tepi_t_dn10) * var_vt) + (assign5920_e5986 * var_vt_dn10)) * var_rcv_t) - (assign5920_e5988 * var_rcv_t_dn10)) / (var_rcv_t * var_rcv_t));
        var_qepi0_db0 = ((((((4.0 * var_tepi_t_db0) * var_vt) + (assign5920_e5986 * var_vt_db0)) * var_rcv_t) - (assign5920_e5988 * var_rcv_t_db0)) / (var_rcv_t * var_rcv_t));
        var_qepi0_db1 = ((((((4.0 * var_tepi_t_db1) * var_vt) + (assign5920_e5986 * var_vt_db1)) * var_rcv_t) - (assign5920_e5988 * var_rcv_t_db1)) / (var_rcv_t * var_rcv_t));
        var_qepi0_rv = 0.0;
        var_qepi0_rdn0 = 0.0;
        var_qepi0_rdn1 = 0.0;
        var_qepi0_rdn2 = 0.0;
        var_qepi0_rdn3 = 0.0;
        var_qepi0_rdn4 = 0.0;
        var_qepi0_rdn5 = 0.0;
        var_qepi0_rdn6 = 0.0;
        var_qepi0_rdn7 = 0.0;
        var_qepi0_rdn8 = 0.0;
        var_qepi0_rdn9 = 0.0;
        var_qepi0_rdn10 = 0.0;
        var_qepi0_rdb0 = 0.0;
        var_qepi0_rdb1 = 0.0;

        let assign5930_e5993: f64 = (0.5 * var_qepi0);
        let assign5930_e5995: f64 = (assign5930_e5993 * var_xi_w);
        let assign5930_e5998: f64 = (var_p0star + var_pw);
        let assign5930_e6000: f64 = (assign5930_e5998 + 2.0);
        let assign5930_e6001: f64 = (assign5930_e5995 * assign5930_e6000);
        var_qepi = assign5930_e6001;
        var_qepi_dn0 = (((((0.5 * var_qepi0_dn0) * var_xi_w) + (assign5930_e5993 * var_xi_w_dn0)) * assign5930_e6000) + (assign5930_e5995 * (var_p0star_dn0 + var_pw_dn0)));
        var_qepi_dn1 = (((((0.5 * var_qepi0_dn1) * var_xi_w) + (assign5930_e5993 * var_xi_w_dn1)) * assign5930_e6000) + (assign5930_e5995 * (var_p0star_dn1 + var_pw_dn1)));
        var_qepi_dn2 = (((((0.5 * var_qepi0_dn2) * var_xi_w) + (assign5930_e5993 * var_xi_w_dn2)) * assign5930_e6000) + (assign5930_e5995 * (var_p0star_dn2 + var_pw_dn2)));
        var_qepi_dn3 = (((((0.5 * var_qepi0_dn3) * var_xi_w) + (assign5930_e5993 * var_xi_w_dn3)) * assign5930_e6000) + (assign5930_e5995 * (var_p0star_dn3 + var_pw_dn3)));
        var_qepi_dn4 = (((((0.5 * var_qepi0_dn4) * var_xi_w) + (assign5930_e5993 * var_xi_w_dn4)) * assign5930_e6000) + (assign5930_e5995 * (var_p0star_dn4 + var_pw_dn4)));
        var_qepi_dn5 = (((((0.5 * var_qepi0_dn5) * var_xi_w) + (assign5930_e5993 * var_xi_w_dn5)) * assign5930_e6000) + (assign5930_e5995 * (var_p0star_dn5 + var_pw_dn5)));
        var_qepi_dn6 = (((((0.5 * var_qepi0_dn6) * var_xi_w) + (assign5930_e5993 * var_xi_w_dn6)) * assign5930_e6000) + (assign5930_e5995 * (var_p0star_dn6 + var_pw_dn6)));
        var_qepi_dn7 = (((((0.5 * var_qepi0_dn7) * var_xi_w) + (assign5930_e5993 * var_xi_w_dn7)) * assign5930_e6000) + (assign5930_e5995 * (var_p0star_dn7 + var_pw_dn7)));
        var_qepi_dn8 = (((((0.5 * var_qepi0_dn8) * var_xi_w) + (assign5930_e5993 * var_xi_w_dn8)) * assign5930_e6000) + (assign5930_e5995 * (var_p0star_dn8 + var_pw_dn8)));
        var_qepi_dn9 = (((((0.5 * var_qepi0_dn9) * var_xi_w) + (assign5930_e5993 * var_xi_w_dn9)) * assign5930_e6000) + (assign5930_e5995 * (var_p0star_dn9 + var_pw_dn9)));
        var_qepi_dn10 = (((((0.5 * var_qepi0_dn10) * var_xi_w) + (assign5930_e5993 * var_xi_w_dn10)) * assign5930_e6000) + (assign5930_e5995 * (var_p0star_dn10 + var_pw_dn10)));
        var_qepi_db0 = (((((0.5 * var_qepi0_db0) * var_xi_w) + (assign5930_e5993 * var_xi_w_db0)) * assign5930_e6000) + (assign5930_e5995 * (var_p0star_db0 + var_pw_db0)));
        var_qepi_db1 = (((((0.5 * var_qepi0_db1) * var_xi_w) + (assign5930_e5993 * var_xi_w_db1)) * assign5930_e6000) + (assign5930_e5995 * (var_p0star_db1 + var_pw_db1)));
        var_qepi_rv = 0.0;
        var_qepi_rdn0 = 0.0;
        var_qepi_rdn1 = 0.0;
        var_qepi_rdn2 = 0.0;
        var_qepi_rdn3 = 0.0;
        var_qepi_rdn4 = 0.0;
        var_qepi_rdn5 = 0.0;
        var_qepi_rdn6 = 0.0;
        var_qepi_rdn7 = 0.0;
        var_qepi_rdn8 = 0.0;
        var_qepi_rdn9 = 0.0;
        var_qepi_rdn10 = 0.0;
        var_qepi_rdb0 = 0.0;
        var_qepi_rdb1 = 0.0;

        let assign5940_e6004: f64 = if p.p78 == 0.0 { 1.0 } else { 0.0 };
        var_guard107 = assign5940_e6004;
        var_guard107_dn0 = 0.0;
        var_guard107_dn1 = 0.0;
        var_guard107_dn2 = 0.0;
        var_guard107_dn3 = 0.0;
        var_guard107_dn4 = 0.0;
        var_guard107_dn5 = 0.0;
        var_guard107_dn6 = 0.0;
        var_guard107_dn7 = 0.0;
        var_guard107_dn8 = 0.0;
        var_guard107_dn9 = 0.0;
        var_guard107_dn10 = 0.0;
        var_guard107_db0 = 0.0;
        var_guard107_db1 = 0.0;
        var_guard107_rv = 0.0;
        var_guard107_rdn0 = 0.0;
        var_guard107_rdn1 = 0.0;
        var_guard107_rdn2 = 0.0;
        var_guard107_rdn3 = 0.0;
        var_guard107_rdn4 = 0.0;
        var_guard107_rdn5 = 0.0;
        var_guard107_rdn6 = 0.0;
        var_guard107_rdn7 = 0.0;
        var_guard107_rdn8 = 0.0;
        var_guard107_rdn9 = 0.0;
        var_guard107_rdn10 = 0.0;
        var_guard107_rdb0 = 0.0;
        var_guard107_rdb1 = 0.0;

        let (assign5950_e6022, assign5950_e6022_d_n0, assign5950_e6022_d_n1, assign5950_e6022_d_n2, assign5950_e6022_d_n3, assign5950_e6022_d_n4, assign5950_e6022_d_n5, assign5950_e6022_d_n6, assign5950_e6022_d_n7, assign5950_e6022_d_n8, assign5950_e6022_d_n9, assign5950_e6022_d_n10, assign5950_e6022_d_b0, assign5950_e6022_d_b1,) = {
    if (var_guard107 != 0.0) {
        let assign5950_e6008: f64 = (var_taur_t * 0.5);
        let assign5950_e6011: f64 = (var_qb0 * var_nbex);
        let assign5950_e6014: f64 = (var_qepi0 * var_pwex);
        let assign5950_e6015: f64 = (assign5950_e6011 + assign5950_e6014);
        let assign5950_e6016: f64 = (assign5950_e6008 * assign5950_e6015);
        let assign5950_e6019: f64 = (var_taub_t + var_tepi_t);
        let assign5950_e6020: f64 = (assign5950_e6016 / assign5950_e6019);
        (assign5950_e6020, ((((((var_taur_t_dn0 * 0.5) * assign5950_e6015) + (assign5950_e6008 * (((var_qb0_dn0 * var_nbex) + (var_qb0 * var_nbex_dn0)) + ((var_qepi0_dn0 * var_pwex) + (var_qepi0 * var_pwex_dn0))))) * assign5950_e6019) - (assign5950_e6016 * (var_taub_t_dn0 + var_tepi_t_dn0))) / (assign5950_e6019 * assign5950_e6019)), ((((((var_taur_t_dn1 * 0.5) * assign5950_e6015) + (assign5950_e6008 * (((var_qb0_dn1 * var_nbex) + (var_qb0 * var_nbex_dn1)) + ((var_qepi0_dn1 * var_pwex) + (var_qepi0 * var_pwex_dn1))))) * assign5950_e6019) - (assign5950_e6016 * (var_taub_t_dn1 + var_tepi_t_dn1))) / (assign5950_e6019 * assign5950_e6019)), ((((((var_taur_t_dn2 * 0.5) * assign5950_e6015) + (assign5950_e6008 * (((var_qb0_dn2 * var_nbex) + (var_qb0 * var_nbex_dn2)) + ((var_qepi0_dn2 * var_pwex) + (var_qepi0 * var_pwex_dn2))))) * assign5950_e6019) - (assign5950_e6016 * (var_taub_t_dn2 + var_tepi_t_dn2))) / (assign5950_e6019 * assign5950_e6019)), ((((((var_taur_t_dn3 * 0.5) * assign5950_e6015) + (assign5950_e6008 * (((var_qb0_dn3 * var_nbex) + (var_qb0 * var_nbex_dn3)) + ((var_qepi0_dn3 * var_pwex) + (var_qepi0 * var_pwex_dn3))))) * assign5950_e6019) - (assign5950_e6016 * (var_taub_t_dn3 + var_tepi_t_dn3))) / (assign5950_e6019 * assign5950_e6019)), ((((((var_taur_t_dn4 * 0.5) * assign5950_e6015) + (assign5950_e6008 * (((var_qb0_dn4 * var_nbex) + (var_qb0 * var_nbex_dn4)) + ((var_qepi0_dn4 * var_pwex) + (var_qepi0 * var_pwex_dn4))))) * assign5950_e6019) - (assign5950_e6016 * (var_taub_t_dn4 + var_tepi_t_dn4))) / (assign5950_e6019 * assign5950_e6019)), ((((((var_taur_t_dn5 * 0.5) * assign5950_e6015) + (assign5950_e6008 * (((var_qb0_dn5 * var_nbex) + (var_qb0 * var_nbex_dn5)) + ((var_qepi0_dn5 * var_pwex) + (var_qepi0 * var_pwex_dn5))))) * assign5950_e6019) - (assign5950_e6016 * (var_taub_t_dn5 + var_tepi_t_dn5))) / (assign5950_e6019 * assign5950_e6019)), ((((((var_taur_t_dn6 * 0.5) * assign5950_e6015) + (assign5950_e6008 * (((var_qb0_dn6 * var_nbex) + (var_qb0 * var_nbex_dn6)) + ((var_qepi0_dn6 * var_pwex) + (var_qepi0 * var_pwex_dn6))))) * assign5950_e6019) - (assign5950_e6016 * (var_taub_t_dn6 + var_tepi_t_dn6))) / (assign5950_e6019 * assign5950_e6019)), ((((((var_taur_t_dn7 * 0.5) * assign5950_e6015) + (assign5950_e6008 * (((var_qb0_dn7 * var_nbex) + (var_qb0 * var_nbex_dn7)) + ((var_qepi0_dn7 * var_pwex) + (var_qepi0 * var_pwex_dn7))))) * assign5950_e6019) - (assign5950_e6016 * (var_taub_t_dn7 + var_tepi_t_dn7))) / (assign5950_e6019 * assign5950_e6019)), ((((((var_taur_t_dn8 * 0.5) * assign5950_e6015) + (assign5950_e6008 * (((var_qb0_dn8 * var_nbex) + (var_qb0 * var_nbex_dn8)) + ((var_qepi0_dn8 * var_pwex) + (var_qepi0 * var_pwex_dn8))))) * assign5950_e6019) - (assign5950_e6016 * (var_taub_t_dn8 + var_tepi_t_dn8))) / (assign5950_e6019 * assign5950_e6019)), ((((((var_taur_t_dn9 * 0.5) * assign5950_e6015) + (assign5950_e6008 * (((var_qb0_dn9 * var_nbex) + (var_qb0 * var_nbex_dn9)) + ((var_qepi0_dn9 * var_pwex) + (var_qepi0 * var_pwex_dn9))))) * assign5950_e6019) - (assign5950_e6016 * (var_taub_t_dn9 + var_tepi_t_dn9))) / (assign5950_e6019 * assign5950_e6019)), ((((((var_taur_t_dn10 * 0.5) * assign5950_e6015) + (assign5950_e6008 * (((var_qb0_dn10 * var_nbex) + (var_qb0 * var_nbex_dn10)) + ((var_qepi0_dn10 * var_pwex) + (var_qepi0 * var_pwex_dn10))))) * assign5950_e6019) - (assign5950_e6016 * (var_taub_t_dn10 + var_tepi_t_dn10))) / (assign5950_e6019 * assign5950_e6019)), ((((((var_taur_t_db0 * 0.5) * assign5950_e6015) + (assign5950_e6008 * (((var_qb0_db0 * var_nbex) + (var_qb0 * var_nbex_db0)) + ((var_qepi0_db0 * var_pwex) + (var_qepi0 * var_pwex_db0))))) * assign5950_e6019) - (assign5950_e6016 * (var_taub_t_db0 + var_tepi_t_db0))) / (assign5950_e6019 * assign5950_e6019)), ((((((var_taur_t_db1 * 0.5) * assign5950_e6015) + (assign5950_e6008 * (((var_qb0_db1 * var_nbex) + (var_qb0 * var_nbex_db1)) + ((var_qepi0_db1 * var_pwex) + (var_qepi0 * var_pwex_db1))))) * assign5950_e6019) - (assign5950_e6016 * (var_taub_t_db1 + var_tepi_t_db1))) / (assign5950_e6019 * assign5950_e6019)),)
    } else {
        (var_qex, var_qex_dn0, var_qex_dn1, var_qex_dn2, var_qex_dn3, var_qex_dn4, var_qex_dn5, var_qex_dn6, var_qex_dn7, var_qex_dn8, var_qex_dn9, var_qex_dn10, var_qex_db0, var_qex_db1,)
    }
};
        var_qex = assign5950_e6022;
        var_qex_dn0 = assign5950_e6022_d_n0;
        var_qex_dn1 = assign5950_e6022_d_n1;
        var_qex_dn2 = assign5950_e6022_d_n2;
        var_qex_dn3 = assign5950_e6022_d_n3;
        var_qex_dn4 = assign5950_e6022_d_n4;
        var_qex_dn5 = assign5950_e6022_d_n5;
        var_qex_dn6 = assign5950_e6022_d_n6;
        var_qex_dn7 = assign5950_e6022_d_n7;
        var_qex_dn8 = assign5950_e6022_d_n8;
        var_qex_dn9 = assign5950_e6022_d_n9;
        var_qex_dn10 = assign5950_e6022_d_n10;
        var_qex_db0 = assign5950_e6022_d_b0;
        var_qex_db1 = assign5950_e6022_d_b1;
        var_qex_rv = 0.0;
        var_qex_rdn0 = 0.0;
        var_qex_rdn1 = 0.0;
        var_qex_rdn2 = 0.0;
        var_qex_rdn3 = 0.0;
        var_qex_rdn4 = 0.0;
        var_qex_rdn5 = 0.0;
        var_qex_rdn6 = 0.0;
        var_qex_rdn7 = 0.0;
        var_qex_rdn8 = 0.0;
        var_qex_rdn9 = 0.0;
        var_qex_rdn10 = 0.0;
        var_qex_rdb0 = 0.0;
        var_qex_rdb1 = 0.0;

        *var_expl_slot = var_expl;
        *var_expl_db0_slot = var_expl_db0;
        *var_expl_db1_slot = var_expl_db1;
        *var_expl_dn0_slot = var_expl_dn0;
        *var_expl_dn1_slot = var_expl_dn1;
        *var_expl_dn10_slot = var_expl_dn10;
        *var_expl_dn2_slot = var_expl_dn2;
        *var_expl_dn3_slot = var_expl_dn3;
        *var_expl_dn4_slot = var_expl_dn4;
        *var_expl_dn5_slot = var_expl_dn5;
        *var_expl_dn6_slot = var_expl_dn6;
        *var_expl_dn7_slot = var_expl_dn7;
        *var_expl_dn8_slot = var_expl_dn8;
        *var_expl_dn9_slot = var_expl_dn9;
        *var_expl_rdb0_slot = var_expl_rdb0;
        *var_expl_rdb1_slot = var_expl_rdb1;
        *var_expl_rdn0_slot = var_expl_rdn0;
        *var_expl_rdn1_slot = var_expl_rdn1;
        *var_expl_rdn10_slot = var_expl_rdn10;
        *var_expl_rdn2_slot = var_expl_rdn2;
        *var_expl_rdn3_slot = var_expl_rdn3;
        *var_expl_rdn4_slot = var_expl_rdn4;
        *var_expl_rdn5_slot = var_expl_rdn5;
        *var_expl_rdn6_slot = var_expl_rdn6;
        *var_expl_rdn7_slot = var_expl_rdn7;
        *var_expl_rdn8_slot = var_expl_rdn8;
        *var_expl_rdn9_slot = var_expl_rdn9;
        *var_expl_rv_slot = var_expl_rv;
        *var_guard106_slot = var_guard106;
        *var_guard106_db0_slot = var_guard106_db0;
        *var_guard106_db1_slot = var_guard106_db1;
        *var_guard106_dn0_slot = var_guard106_dn0;
        *var_guard106_dn1_slot = var_guard106_dn1;
        *var_guard106_dn10_slot = var_guard106_dn10;
        *var_guard106_dn2_slot = var_guard106_dn2;
        *var_guard106_dn3_slot = var_guard106_dn3;
        *var_guard106_dn4_slot = var_guard106_dn4;
        *var_guard106_dn5_slot = var_guard106_dn5;
        *var_guard106_dn6_slot = var_guard106_dn6;
        *var_guard106_dn7_slot = var_guard106_dn7;
        *var_guard106_dn8_slot = var_guard106_dn8;
        *var_guard106_dn9_slot = var_guard106_dn9;
        *var_guard106_rdb0_slot = var_guard106_rdb0;
        *var_guard106_rdb1_slot = var_guard106_rdb1;
        *var_guard106_rdn0_slot = var_guard106_rdn0;
        *var_guard106_rdn1_slot = var_guard106_rdn1;
        *var_guard106_rdn10_slot = var_guard106_rdn10;
        *var_guard106_rdn2_slot = var_guard106_rdn2;
        *var_guard106_rdn3_slot = var_guard106_rdn3;
        *var_guard106_rdn4_slot = var_guard106_rdn4;
        *var_guard106_rdn5_slot = var_guard106_rdn5;
        *var_guard106_rdn6_slot = var_guard106_rdn6;
        *var_guard106_rdn7_slot = var_guard106_rdn7;
        *var_guard106_rdn8_slot = var_guard106_rdn8;
        *var_guard106_rdn9_slot = var_guard106_rdn9;
        *var_guard106_rv_slot = var_guard106_rv;
        *var_guard107_slot = var_guard107;
        *var_guard107_db0_slot = var_guard107_db0;
        *var_guard107_db1_slot = var_guard107_db1;
        *var_guard107_dn0_slot = var_guard107_dn0;
        *var_guard107_dn1_slot = var_guard107_dn1;
        *var_guard107_dn10_slot = var_guard107_dn10;
        *var_guard107_dn2_slot = var_guard107_dn2;
        *var_guard107_dn3_slot = var_guard107_dn3;
        *var_guard107_dn4_slot = var_guard107_dn4;
        *var_guard107_dn5_slot = var_guard107_dn5;
        *var_guard107_dn6_slot = var_guard107_dn6;
        *var_guard107_dn7_slot = var_guard107_dn7;
        *var_guard107_dn8_slot = var_guard107_dn8;
        *var_guard107_dn9_slot = var_guard107_dn9;
        *var_guard107_rdb0_slot = var_guard107_rdb0;
        *var_guard107_rdb1_slot = var_guard107_rdb1;
        *var_guard107_rdn0_slot = var_guard107_rdn0;
        *var_guard107_rdn1_slot = var_guard107_rdn1;
        *var_guard107_rdn10_slot = var_guard107_rdn10;
        *var_guard107_rdn2_slot = var_guard107_rdn2;
        *var_guard107_rdn3_slot = var_guard107_rdn3;
        *var_guard107_rdn4_slot = var_guard107_rdn4;
        *var_guard107_rdn5_slot = var_guard107_rdn5;
        *var_guard107_rdn6_slot = var_guard107_rdn6;
        *var_guard107_rdn7_slot = var_guard107_rdn7;
        *var_guard107_rdn8_slot = var_guard107_rdn8;
        *var_guard107_rdn9_slot = var_guard107_rdn9;
        *var_guard107_rv_slot = var_guard107_rv;
        *var_qe0_slot = var_qe0;
        *var_qe0_db0_slot = var_qe0_db0;
        *var_qe0_db1_slot = var_qe0_db1;
        *var_qe0_dn0_slot = var_qe0_dn0;
        *var_qe0_dn1_slot = var_qe0_dn1;
        *var_qe0_dn10_slot = var_qe0_dn10;
        *var_qe0_dn2_slot = var_qe0_dn2;
        *var_qe0_dn3_slot = var_qe0_dn3;
        *var_qe0_dn4_slot = var_qe0_dn4;
        *var_qe0_dn5_slot = var_qe0_dn5;
        *var_qe0_dn6_slot = var_qe0_dn6;
        *var_qe0_dn7_slot = var_qe0_dn7;
        *var_qe0_dn8_slot = var_qe0_dn8;
        *var_qe0_dn9_slot = var_qe0_dn9;
        *var_qe0_rdb0_slot = var_qe0_rdb0;
        *var_qe0_rdb1_slot = var_qe0_rdb1;
        *var_qe0_rdn0_slot = var_qe0_rdn0;
        *var_qe0_rdn1_slot = var_qe0_rdn1;
        *var_qe0_rdn10_slot = var_qe0_rdn10;
        *var_qe0_rdn2_slot = var_qe0_rdn2;
        *var_qe0_rdn3_slot = var_qe0_rdn3;
        *var_qe0_rdn4_slot = var_qe0_rdn4;
        *var_qe0_rdn5_slot = var_qe0_rdn5;
        *var_qe0_rdn6_slot = var_qe0_rdn6;
        *var_qe0_rdn7_slot = var_qe0_rdn7;
        *var_qe0_rdn8_slot = var_qe0_rdn8;
        *var_qe0_rdn9_slot = var_qe0_rdn9;
        *var_qe0_rv_slot = var_qe0_rv;
        *var_qe_qs_slot = var_qe_qs;
        *var_qe_qs_db0_slot = var_qe_qs_db0;
        *var_qe_qs_db1_slot = var_qe_qs_db1;
        *var_qe_qs_dn0_slot = var_qe_qs_dn0;
        *var_qe_qs_dn1_slot = var_qe_qs_dn1;
        *var_qe_qs_dn10_slot = var_qe_qs_dn10;
        *var_qe_qs_dn2_slot = var_qe_qs_dn2;
        *var_qe_qs_dn3_slot = var_qe_qs_dn3;
        *var_qe_qs_dn4_slot = var_qe_qs_dn4;
        *var_qe_qs_dn5_slot = var_qe_qs_dn5;
        *var_qe_qs_dn6_slot = var_qe_qs_dn6;
        *var_qe_qs_dn7_slot = var_qe_qs_dn7;
        *var_qe_qs_dn8_slot = var_qe_qs_dn8;
        *var_qe_qs_dn9_slot = var_qe_qs_dn9;
        *var_qe_qs_rdb0_slot = var_qe_qs_rdb0;
        *var_qe_qs_rdb1_slot = var_qe_qs_rdb1;
        *var_qe_qs_rdn0_slot = var_qe_qs_rdn0;
        *var_qe_qs_rdn1_slot = var_qe_qs_rdn1;
        *var_qe_qs_rdn10_slot = var_qe_qs_rdn10;
        *var_qe_qs_rdn2_slot = var_qe_qs_rdn2;
        *var_qe_qs_rdn3_slot = var_qe_qs_rdn3;
        *var_qe_qs_rdn4_slot = var_qe_qs_rdn4;
        *var_qe_qs_rdn5_slot = var_qe_qs_rdn5;
        *var_qe_qs_rdn6_slot = var_qe_qs_rdn6;
        *var_qe_qs_rdn7_slot = var_qe_qs_rdn7;
        *var_qe_qs_rdn8_slot = var_qe_qs_rdn8;
        *var_qe_qs_rdn9_slot = var_qe_qs_rdn9;
        *var_qe_qs_rv_slot = var_qe_qs_rv;
        *var_qepi_slot = var_qepi;
        *var_qepi0_slot = var_qepi0;
        *var_qepi0_db0_slot = var_qepi0_db0;
        *var_qepi0_db1_slot = var_qepi0_db1;
        *var_qepi0_dn0_slot = var_qepi0_dn0;
        *var_qepi0_dn1_slot = var_qepi0_dn1;
        *var_qepi0_dn10_slot = var_qepi0_dn10;
        *var_qepi0_dn2_slot = var_qepi0_dn2;
        *var_qepi0_dn3_slot = var_qepi0_dn3;
        *var_qepi0_dn4_slot = var_qepi0_dn4;
        *var_qepi0_dn5_slot = var_qepi0_dn5;
        *var_qepi0_dn6_slot = var_qepi0_dn6;
        *var_qepi0_dn7_slot = var_qepi0_dn7;
        *var_qepi0_dn8_slot = var_qepi0_dn8;
        *var_qepi0_dn9_slot = var_qepi0_dn9;
        *var_qepi0_rdb0_slot = var_qepi0_rdb0;
        *var_qepi0_rdb1_slot = var_qepi0_rdb1;
        *var_qepi0_rdn0_slot = var_qepi0_rdn0;
        *var_qepi0_rdn1_slot = var_qepi0_rdn1;
        *var_qepi0_rdn10_slot = var_qepi0_rdn10;
        *var_qepi0_rdn2_slot = var_qepi0_rdn2;
        *var_qepi0_rdn3_slot = var_qepi0_rdn3;
        *var_qepi0_rdn4_slot = var_qepi0_rdn4;
        *var_qepi0_rdn5_slot = var_qepi0_rdn5;
        *var_qepi0_rdn6_slot = var_qepi0_rdn6;
        *var_qepi0_rdn7_slot = var_qepi0_rdn7;
        *var_qepi0_rdn8_slot = var_qepi0_rdn8;
        *var_qepi0_rdn9_slot = var_qepi0_rdn9;
        *var_qepi0_rv_slot = var_qepi0_rv;
        *var_qepi_db0_slot = var_qepi_db0;
        *var_qepi_db1_slot = var_qepi_db1;
        *var_qepi_dn0_slot = var_qepi_dn0;
        *var_qepi_dn1_slot = var_qepi_dn1;
        *var_qepi_dn10_slot = var_qepi_dn10;
        *var_qepi_dn2_slot = var_qepi_dn2;
        *var_qepi_dn3_slot = var_qepi_dn3;
        *var_qepi_dn4_slot = var_qepi_dn4;
        *var_qepi_dn5_slot = var_qepi_dn5;
        *var_qepi_dn6_slot = var_qepi_dn6;
        *var_qepi_dn7_slot = var_qepi_dn7;
        *var_qepi_dn8_slot = var_qepi_dn8;
        *var_qepi_dn9_slot = var_qepi_dn9;
        *var_qepi_rdb0_slot = var_qepi_rdb0;
        *var_qepi_rdb1_slot = var_qepi_rdb1;
        *var_qepi_rdn0_slot = var_qepi_rdn0;
        *var_qepi_rdn1_slot = var_qepi_rdn1;
        *var_qepi_rdn10_slot = var_qepi_rdn10;
        *var_qepi_rdn2_slot = var_qepi_rdn2;
        *var_qepi_rdn3_slot = var_qepi_rdn3;
        *var_qepi_rdn4_slot = var_qepi_rdn4;
        *var_qepi_rdn5_slot = var_qepi_rdn5;
        *var_qepi_rdn6_slot = var_qepi_rdn6;
        *var_qepi_rdn7_slot = var_qepi_rdn7;
        *var_qepi_rdn8_slot = var_qepi_rdn8;
        *var_qepi_rdn9_slot = var_qepi_rdn9;
        *var_qepi_rv_slot = var_qepi_rv;
        *var_qex_slot = var_qex;
        *var_qex_db0_slot = var_qex_db0;
        *var_qex_db1_slot = var_qex_db1;
        *var_qex_dn0_slot = var_qex_dn0;
        *var_qex_dn1_slot = var_qex_dn1;
        *var_qex_dn10_slot = var_qex_dn10;
        *var_qex_dn2_slot = var_qex_dn2;
        *var_qex_dn3_slot = var_qex_dn3;
        *var_qex_dn4_slot = var_qex_dn4;
        *var_qex_dn5_slot = var_qex_dn5;
        *var_qex_dn6_slot = var_qex_dn6;
        *var_qex_dn7_slot = var_qex_dn7;
        *var_qex_dn8_slot = var_qex_dn8;
        *var_qex_dn9_slot = var_qex_dn9;
        *var_qex_rdb0_slot = var_qex_rdb0;
        *var_qex_rdb1_slot = var_qex_rdb1;
        *var_qex_rdn0_slot = var_qex_rdn0;
        *var_qex_rdn1_slot = var_qex_rdn1;
        *var_qex_rdn10_slot = var_qex_rdn10;
        *var_qex_rdn2_slot = var_qex_rdn2;
        *var_qex_rdn3_slot = var_qex_rdn3;
        *var_qex_rdn4_slot = var_qex_rdn4;
        *var_qex_rdn5_slot = var_qex_rdn5;
        *var_qex_rdn6_slot = var_qex_rdn6;
        *var_qex_rdn7_slot = var_qex_rdn7;
        *var_qex_rdn8_slot = var_qex_rdn8;
        *var_qex_rdn9_slot = var_qex_rdn9;
        *var_qex_rv_slot = var_qex_rv;
        *var_tmpexp_slot = var_tmpexp;
        *var_tmpexp_db0_slot = var_tmpexp_db0;
        *var_tmpexp_db1_slot = var_tmpexp_db1;
        *var_tmpexp_dn0_slot = var_tmpexp_dn0;
        *var_tmpexp_dn1_slot = var_tmpexp_dn1;
        *var_tmpexp_dn10_slot = var_tmpexp_dn10;
        *var_tmpexp_dn2_slot = var_tmpexp_dn2;
        *var_tmpexp_dn3_slot = var_tmpexp_dn3;
        *var_tmpexp_dn4_slot = var_tmpexp_dn4;
        *var_tmpexp_dn5_slot = var_tmpexp_dn5;
        *var_tmpexp_dn6_slot = var_tmpexp_dn6;
        *var_tmpexp_dn7_slot = var_tmpexp_dn7;
        *var_tmpexp_dn8_slot = var_tmpexp_dn8;
        *var_tmpexp_dn9_slot = var_tmpexp_dn9;
        *var_tmpexp_rdb0_slot = var_tmpexp_rdb0;
        *var_tmpexp_rdb1_slot = var_tmpexp_rdb1;
        *var_tmpexp_rdn0_slot = var_tmpexp_rdn0;
        *var_tmpexp_rdn1_slot = var_tmpexp_rdn1;
        *var_tmpexp_rdn10_slot = var_tmpexp_rdn10;
        *var_tmpexp_rdn2_slot = var_tmpexp_rdn2;
        *var_tmpexp_rdn3_slot = var_tmpexp_rdn3;
        *var_tmpexp_rdn4_slot = var_tmpexp_rdn4;
        *var_tmpexp_rdn5_slot = var_tmpexp_rdn5;
        *var_tmpexp_rdn6_slot = var_tmpexp_rdn6;
        *var_tmpexp_rdn7_slot = var_tmpexp_rdn7;
        *var_tmpexp_rdn8_slot = var_tmpexp_rdn8;
        *var_tmpexp_rdn9_slot = var_tmpexp_rdn9;
        *var_tmpexp_rv_slot = var_tmpexp_rv;
        *var_xqtex_slot = var_xqtex;
        *var_xqtex_db0_slot = var_xqtex_db0;
        *var_xqtex_db1_slot = var_xqtex_db1;
        *var_xqtex_dn0_slot = var_xqtex_dn0;
        *var_xqtex_dn1_slot = var_xqtex_dn1;
        *var_xqtex_dn10_slot = var_xqtex_dn10;
        *var_xqtex_dn2_slot = var_xqtex_dn2;
        *var_xqtex_dn3_slot = var_xqtex_dn3;
        *var_xqtex_dn4_slot = var_xqtex_dn4;
        *var_xqtex_dn5_slot = var_xqtex_dn5;
        *var_xqtex_dn6_slot = var_xqtex_dn6;
        *var_xqtex_dn7_slot = var_xqtex_dn7;
        *var_xqtex_dn8_slot = var_xqtex_dn8;
        *var_xqtex_dn9_slot = var_xqtex_dn9;
        *var_xqtex_rdb0_slot = var_xqtex_rdb0;
        *var_xqtex_rdb1_slot = var_xqtex_rdb1;
        *var_xqtex_rdn0_slot = var_xqtex_rdn0;
        *var_xqtex_rdn1_slot = var_xqtex_rdn1;
        *var_xqtex_rdn10_slot = var_xqtex_rdn10;
        *var_xqtex_rdn2_slot = var_xqtex_rdn2;
        *var_xqtex_rdn3_slot = var_xqtex_rdn3;
        *var_xqtex_rdn4_slot = var_xqtex_rdn4;
        *var_xqtex_rdn5_slot = var_xqtex_rdn5;
        *var_xqtex_rdn6_slot = var_xqtex_rdn6;
        *var_xqtex_rdn7_slot = var_xqtex_rdn7;
        *var_xqtex_rdn8_slot = var_xqtex_rdn8;
        *var_xqtex_rdn9_slot = var_xqtex_rdn9;
        *var_xqtex_rv_slot = var_xqtex_rv;
        *var_xvjcex_slot = var_xvjcex;
        *var_xvjcex_db0_slot = var_xvjcex_db0;
        *var_xvjcex_db1_slot = var_xvjcex_db1;
        *var_xvjcex_dn0_slot = var_xvjcex_dn0;
        *var_xvjcex_dn1_slot = var_xvjcex_dn1;
        *var_xvjcex_dn10_slot = var_xvjcex_dn10;
        *var_xvjcex_dn2_slot = var_xvjcex_dn2;
        *var_xvjcex_dn3_slot = var_xvjcex_dn3;
        *var_xvjcex_dn4_slot = var_xvjcex_dn4;
        *var_xvjcex_dn5_slot = var_xvjcex_dn5;
        *var_xvjcex_dn6_slot = var_xvjcex_dn6;
        *var_xvjcex_dn7_slot = var_xvjcex_dn7;
        *var_xvjcex_dn8_slot = var_xvjcex_dn8;
        *var_xvjcex_dn9_slot = var_xvjcex_dn9;
        *var_xvjcex_rdb0_slot = var_xvjcex_rdb0;
        *var_xvjcex_rdb1_slot = var_xvjcex_rdb1;
        *var_xvjcex_rdn0_slot = var_xvjcex_rdn0;
        *var_xvjcex_rdn1_slot = var_xvjcex_rdn1;
        *var_xvjcex_rdn10_slot = var_xvjcex_rdn10;
        *var_xvjcex_rdn2_slot = var_xvjcex_rdn2;
        *var_xvjcex_rdn3_slot = var_xvjcex_rdn3;
        *var_xvjcex_rdn4_slot = var_xvjcex_rdn4;
        *var_xvjcex_rdn5_slot = var_xvjcex_rdn5;
        *var_xvjcex_rdn6_slot = var_xvjcex_rdn6;
        *var_xvjcex_rdn7_slot = var_xvjcex_rdn7;
        *var_xvjcex_rdn8_slot = var_xvjcex_rdn8;
        *var_xvjcex_rdn9_slot = var_xvjcex_rdn9;
        *var_xvjcex_rv_slot = var_xvjcex_rv;
        *var_xvtexv_slot = var_xvtexv;
        *var_xvtexv_db0_slot = var_xvtexv_db0;
        *var_xvtexv_db1_slot = var_xvtexv_db1;
        *var_xvtexv_dn0_slot = var_xvtexv_dn0;
        *var_xvtexv_dn1_slot = var_xvtexv_dn1;
        *var_xvtexv_dn10_slot = var_xvtexv_dn10;
        *var_xvtexv_dn2_slot = var_xvtexv_dn2;
        *var_xvtexv_dn3_slot = var_xvtexv_dn3;
        *var_xvtexv_dn4_slot = var_xvtexv_dn4;
        *var_xvtexv_dn5_slot = var_xvtexv_dn5;
        *var_xvtexv_dn6_slot = var_xvtexv_dn6;
        *var_xvtexv_dn7_slot = var_xvtexv_dn7;
        *var_xvtexv_dn8_slot = var_xvtexv_dn8;
        *var_xvtexv_dn9_slot = var_xvtexv_dn9;
        *var_xvtexv_rdb0_slot = var_xvtexv_rdb0;
        *var_xvtexv_rdb1_slot = var_xvtexv_rdb1;
        *var_xvtexv_rdn0_slot = var_xvtexv_rdn0;
        *var_xvtexv_rdn1_slot = var_xvtexv_rdn1;
        *var_xvtexv_rdn10_slot = var_xvtexv_rdn10;
        *var_xvtexv_rdn2_slot = var_xvtexv_rdn2;
        *var_xvtexv_rdn3_slot = var_xvtexv_rdn3;
        *var_xvtexv_rdn4_slot = var_xvtexv_rdn4;
        *var_xvtexv_rdn5_slot = var_xvtexv_rdn5;
        *var_xvtexv_rdn6_slot = var_xvtexv_rdn6;
        *var_xvtexv_rdn7_slot = var_xvtexv_rdn7;
        *var_xvtexv_rdn8_slot = var_xvtexv_rdn8;
        *var_xvtexv_rdn9_slot = var_xvtexv_rdn9;
        *var_xvtexv_rv_slot = var_xvtexv_rv;
    }

    pub(super) fn stamp_reactive_block_35(
        p: &Parameters,
        var_evb1c4: f64,
        var_evb1c4_db0: f64,
        var_evb1c4_db1: f64,
        var_evb1c4_dn0: f64,
        var_evb1c4_dn1: f64,
        var_evb1c4_dn10: f64,
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
        var_evbc3vdc_dn2: f64,
        var_evbc3vdc_dn3: f64,
        var_evbc3vdc_dn4: f64,
        var_evbc3vdc_dn5: f64,
        var_evbc3vdc_dn6: f64,
        var_evbc3vdc_dn7: f64,
        var_evbc3vdc_dn8: f64,
        var_evbc3vdc_dn9: f64,
        var_guard107: f64,
        var_ibx_t: f64,
        var_ibx_t_db0: f64,
        var_ibx_t_db1: f64,
        var_ibx_t_dn0: f64,
        var_ibx_t_dn1: f64,
        var_ibx_t_dn10: f64,
        var_ibx_t_dn2: f64,
        var_ibx_t_dn3: f64,
        var_ibx_t_dn4: f64,
        var_ibx_t_dn5: f64,
        var_ibx_t_dn6: f64,
        var_ibx_t_dn7: f64,
        var_ibx_t_dn8: f64,
        var_ibx_t_dn9: f64,
        var_if0: f64,
        var_if0_db0: f64,
        var_if0_db1: f64,
        var_if0_dn0: f64,
        var_if0_dn1: f64,
        var_if0_dn10: f64,
        var_if0_dn2: f64,
        var_if0_dn3: f64,
        var_if0_dn4: f64,
        var_if0_dn5: f64,
        var_if0_dn6: f64,
        var_if0_dn7: f64,
        var_if0_dn8: f64,
        var_if0_dn9: f64,
        var_qb0: f64,
        var_qb0_db0: f64,
        var_qb0_db1: f64,
        var_qb0_dn0: f64,
        var_qb0_dn1: f64,
        var_qb0_dn10: f64,
        var_qb0_dn2: f64,
        var_qb0_dn3: f64,
        var_qb0_dn4: f64,
        var_qb0_dn5: f64,
        var_qb0_dn6: f64,
        var_qb0_dn7: f64,
        var_qb0_dn8: f64,
        var_qb0_dn9: f64,
        var_qepi0: f64,
        var_qepi0_db0: f64,
        var_qepi0_db1: f64,
        var_qepi0_dn0: f64,
        var_qepi0_dn1: f64,
        var_qepi0_dn10: f64,
        var_qepi0_dn2: f64,
        var_qepi0_dn3: f64,
        var_qepi0_dn4: f64,
        var_qepi0_dn5: f64,
        var_qepi0_dn6: f64,
        var_qepi0_dn7: f64,
        var_qepi0_dn8: f64,
        var_qepi0_dn9: f64,
        var_taub_t: f64,
        var_taub_t_db0: f64,
        var_taub_t_db1: f64,
        var_taub_t_dn0: f64,
        var_taub_t_dn1: f64,
        var_taub_t_dn10: f64,
        var_taub_t_dn2: f64,
        var_taub_t_dn3: f64,
        var_taub_t_dn4: f64,
        var_taub_t_dn5: f64,
        var_taub_t_dn6: f64,
        var_taub_t_dn7: f64,
        var_taub_t_dn8: f64,
        var_taub_t_dn9: f64,
        var_tauex_t: f64,
        var_tauex_t_db0: f64,
        var_tauex_t_db1: f64,
        var_tauex_t_dn0: f64,
        var_tauex_t_dn1: f64,
        var_tauex_t_dn10: f64,
        var_tauex_t_dn2: f64,
        var_tauex_t_dn3: f64,
        var_tauex_t_dn4: f64,
        var_tauex_t_dn5: f64,
        var_tauex_t_dn6: f64,
        var_tauex_t_dn7: f64,
        var_tauex_t_dn8: f64,
        var_tauex_t_dn9: f64,
        var_taur_t: f64,
        var_taur_t_db0: f64,
        var_taur_t_db1: f64,
        var_taur_t_dn0: f64,
        var_taur_t_dn1: f64,
        var_taur_t_dn10: f64,
        var_taur_t_dn2: f64,
        var_taur_t_dn3: f64,
        var_taur_t_dn4: f64,
        var_taur_t_dn5: f64,
        var_taur_t_dn6: f64,
        var_taur_t_dn7: f64,
        var_taur_t_dn8: f64,
        var_taur_t_dn9: f64,
        var_tepi_t: f64,
        var_tepi_t_db0: f64,
        var_tepi_t_db1: f64,
        var_tepi_t_dn0: f64,
        var_tepi_t_dn1: f64,
        var_tepi_t_dn10: f64,
        var_tepi_t_dn2: f64,
        var_tepi_t_dn3: f64,
        var_tepi_t_dn4: f64,
        var_tepi_t_dn5: f64,
        var_tepi_t_dn6: f64,
        var_tepi_t_dn7: f64,
        var_tepi_t_dn8: f64,
        var_tepi_t_dn9: f64,
        var_vb1c4: f64,
        var_vb1c4_db0: f64,
        var_vb1c4_db1: f64,
        var_vb1c4_dn0: f64,
        var_vb1c4_dn1: f64,
        var_vb1c4_dn10: f64,
        var_vb1c4_dn2: f64,
        var_vb1c4_dn3: f64,
        var_vb1c4_dn4: f64,
        var_vb1c4_dn5: f64,
        var_vb1c4_dn6: f64,
        var_vb1c4_dn7: f64,
        var_vb1c4_dn8: f64,
        var_vb1c4_dn9: f64,
        var_vdcex_t: f64,
        var_vdcex_t_db0: f64,
        var_vdcex_t_db1: f64,
        var_vdcex_t_dn0: f64,
        var_vdcex_t_dn1: f64,
        var_vdcex_t_dn10: f64,
        var_vdcex_t_dn2: f64,
        var_vdcex_t_dn3: f64,
        var_vdcex_t_dn4: f64,
        var_vdcex_t_dn5: f64,
        var_vdcex_t_dn6: f64,
        var_vdcex_t_dn7: f64,
        var_vdcex_t_dn8: f64,
        var_vdcex_t_dn9: f64,
        var_vtinv: f64,
        var_vtinv_db0: f64,
        var_vtinv_db1: f64,
        var_vtinv_dn0: f64,
        var_vtinv_dn1: f64,
        var_vtinv_dn10: f64,
        var_vtinv_dn2: f64,
        var_vtinv_dn3: f64,
        var_vtinv_dn4: f64,
        var_vtinv_dn5: f64,
        var_vtinv_dn6: f64,
        var_vtinv_dn7: f64,
        var_vtinv_dn8: f64,
        var_vtinv_dn9: f64,
        var_xext1: f64,
        var_xext1_db0: f64,
        var_xext1_db1: f64,
        var_xext1_dn0: f64,
        var_xext1_dn1: f64,
        var_xext1_dn10: f64,
        var_xext1_dn2: f64,
        var_xext1_dn3: f64,
        var_xext1_dn4: f64,
        var_xext1_dn5: f64,
        var_xext1_dn6: f64,
        var_xext1_dn7: f64,
        var_xext1_dn8: f64,
        var_xext1_dn9: f64,
        var_evb1c4vdcex_slot: &mut f64,
        var_evb1c4vdcex_db0_slot: &mut f64,
        var_evb1c4vdcex_db1_slot: &mut f64,
        var_evb1c4vdcex_dn0_slot: &mut f64,
        var_evb1c4vdcex_dn1_slot: &mut f64,
        var_evb1c4vdcex_dn10_slot: &mut f64,
        var_evb1c4vdcex_dn2_slot: &mut f64,
        var_evb1c4vdcex_dn3_slot: &mut f64,
        var_evb1c4vdcex_dn4_slot: &mut f64,
        var_evb1c4vdcex_dn5_slot: &mut f64,
        var_evb1c4vdcex_dn6_slot: &mut f64,
        var_evb1c4vdcex_dn7_slot: &mut f64,
        var_evb1c4vdcex_dn8_slot: &mut f64,
        var_evb1c4vdcex_dn9_slot: &mut f64,
        var_evb1c4vdcex_rdb0_slot: &mut f64,
        var_evb1c4vdcex_rdb1_slot: &mut f64,
        var_evb1c4vdcex_rdn0_slot: &mut f64,
        var_evb1c4vdcex_rdn1_slot: &mut f64,
        var_evb1c4vdcex_rdn10_slot: &mut f64,
        var_evb1c4vdcex_rdn2_slot: &mut f64,
        var_evb1c4vdcex_rdn3_slot: &mut f64,
        var_evb1c4vdcex_rdn4_slot: &mut f64,
        var_evb1c4vdcex_rdn5_slot: &mut f64,
        var_evb1c4vdcex_rdn6_slot: &mut f64,
        var_evb1c4vdcex_rdn7_slot: &mut f64,
        var_evb1c4vdcex_rdn8_slot: &mut f64,
        var_evb1c4vdcex_rdn9_slot: &mut f64,
        var_evb1c4vdcex_rv_slot: &mut f64,
        var_expl_slot: &mut f64,
        var_expl_db0_slot: &mut f64,
        var_expl_db1_slot: &mut f64,
        var_expl_dn0_slot: &mut f64,
        var_expl_dn1_slot: &mut f64,
        var_expl_dn10_slot: &mut f64,
        var_expl_dn2_slot: &mut f64,
        var_expl_dn3_slot: &mut f64,
        var_expl_dn4_slot: &mut f64,
        var_expl_dn5_slot: &mut f64,
        var_expl_dn6_slot: &mut f64,
        var_expl_dn7_slot: &mut f64,
        var_expl_dn8_slot: &mut f64,
        var_expl_dn9_slot: &mut f64,
        var_expl_rdb0_slot: &mut f64,
        var_expl_rdb1_slot: &mut f64,
        var_expl_rdn0_slot: &mut f64,
        var_expl_rdn1_slot: &mut f64,
        var_expl_rdn10_slot: &mut f64,
        var_expl_rdn2_slot: &mut f64,
        var_expl_rdn3_slot: &mut f64,
        var_expl_rdn4_slot: &mut f64,
        var_expl_rdn5_slot: &mut f64,
        var_expl_rdn6_slot: &mut f64,
        var_expl_rdn7_slot: &mut f64,
        var_expl_rdn8_slot: &mut f64,
        var_expl_rdn9_slot: &mut f64,
        var_expl_rv_slot: &mut f64,
        var_guard108_slot: &mut f64,
        var_guard108_db0_slot: &mut f64,
        var_guard108_db1_slot: &mut f64,
        var_guard108_dn0_slot: &mut f64,
        var_guard108_dn1_slot: &mut f64,
        var_guard108_dn10_slot: &mut f64,
        var_guard108_dn2_slot: &mut f64,
        var_guard108_dn3_slot: &mut f64,
        var_guard108_dn4_slot: &mut f64,
        var_guard108_dn5_slot: &mut f64,
        var_guard108_dn6_slot: &mut f64,
        var_guard108_dn7_slot: &mut f64,
        var_guard108_dn8_slot: &mut f64,
        var_guard108_dn9_slot: &mut f64,
        var_guard108_rdb0_slot: &mut f64,
        var_guard108_rdb1_slot: &mut f64,
        var_guard108_rdn0_slot: &mut f64,
        var_guard108_rdn1_slot: &mut f64,
        var_guard108_rdn10_slot: &mut f64,
        var_guard108_rdn2_slot: &mut f64,
        var_guard108_rdn3_slot: &mut f64,
        var_guard108_rdn4_slot: &mut f64,
        var_guard108_rdn5_slot: &mut f64,
        var_guard108_rdn6_slot: &mut f64,
        var_guard108_rdn7_slot: &mut f64,
        var_guard108_rdn8_slot: &mut f64,
        var_guard108_rdn9_slot: &mut f64,
        var_guard108_rv_slot: &mut f64,
        var_guard109_slot: &mut f64,
        var_guard109_db0_slot: &mut f64,
        var_guard109_db1_slot: &mut f64,
        var_guard109_dn0_slot: &mut f64,
        var_guard109_dn1_slot: &mut f64,
        var_guard109_dn10_slot: &mut f64,
        var_guard109_dn2_slot: &mut f64,
        var_guard109_dn3_slot: &mut f64,
        var_guard109_dn4_slot: &mut f64,
        var_guard109_dn5_slot: &mut f64,
        var_guard109_dn6_slot: &mut f64,
        var_guard109_dn7_slot: &mut f64,
        var_guard109_dn8_slot: &mut f64,
        var_guard109_dn9_slot: &mut f64,
        var_guard109_rdb0_slot: &mut f64,
        var_guard109_rdb1_slot: &mut f64,
        var_guard109_rdn0_slot: &mut f64,
        var_guard109_rdn1_slot: &mut f64,
        var_guard109_rdn10_slot: &mut f64,
        var_guard109_rdn2_slot: &mut f64,
        var_guard109_rdn3_slot: &mut f64,
        var_guard109_rdn4_slot: &mut f64,
        var_guard109_rdn5_slot: &mut f64,
        var_guard109_rdn6_slot: &mut f64,
        var_guard109_rdn7_slot: &mut f64,
        var_guard109_rdn8_slot: &mut f64,
        var_guard109_rdn9_slot: &mut f64,
        var_guard109_rv_slot: &mut f64,
        var_guard110_slot: &mut f64,
        var_guard110_db0_slot: &mut f64,
        var_guard110_db1_slot: &mut f64,
        var_guard110_dn0_slot: &mut f64,
        var_guard110_dn1_slot: &mut f64,
        var_guard110_dn10_slot: &mut f64,
        var_guard110_dn2_slot: &mut f64,
        var_guard110_dn3_slot: &mut f64,
        var_guard110_dn4_slot: &mut f64,
        var_guard110_dn5_slot: &mut f64,
        var_guard110_dn6_slot: &mut f64,
        var_guard110_dn7_slot: &mut f64,
        var_guard110_dn8_slot: &mut f64,
        var_guard110_dn9_slot: &mut f64,
        var_guard110_rdb0_slot: &mut f64,
        var_guard110_rdb1_slot: &mut f64,
        var_guard110_rdn0_slot: &mut f64,
        var_guard110_rdn1_slot: &mut f64,
        var_guard110_rdn10_slot: &mut f64,
        var_guard110_rdn2_slot: &mut f64,
        var_guard110_rdn3_slot: &mut f64,
        var_guard110_rdn4_slot: &mut f64,
        var_guard110_rdn5_slot: &mut f64,
        var_guard110_rdn6_slot: &mut f64,
        var_guard110_rdn7_slot: &mut f64,
        var_guard110_rdn8_slot: &mut f64,
        var_guard110_rdn9_slot: &mut f64,
        var_guard110_rv_slot: &mut f64,
        var_qex_slot: &mut f64,
        var_qex_db0_slot: &mut f64,
        var_qex_db1_slot: &mut f64,
        var_qex_dn0_slot: &mut f64,
        var_qex_dn1_slot: &mut f64,
        var_qex_dn10_slot: &mut f64,
        var_qex_dn2_slot: &mut f64,
        var_qex_dn3_slot: &mut f64,
        var_qex_dn4_slot: &mut f64,
        var_qex_dn5_slot: &mut f64,
        var_qex_dn6_slot: &mut f64,
        var_qex_dn7_slot: &mut f64,
        var_qex_dn8_slot: &mut f64,
        var_qex_dn9_slot: &mut f64,
        var_qex_rdb0_slot: &mut f64,
        var_qex_rdb1_slot: &mut f64,
        var_qex_rdn0_slot: &mut f64,
        var_qex_rdn1_slot: &mut f64,
        var_qex_rdn10_slot: &mut f64,
        var_qex_rdn2_slot: &mut f64,
        var_qex_rdn3_slot: &mut f64,
        var_qex_rdn4_slot: &mut f64,
        var_qex_rdn5_slot: &mut f64,
        var_qex_rdn6_slot: &mut f64,
        var_qex_rdn7_slot: &mut f64,
        var_qex_rdn8_slot: &mut f64,
        var_qex_rdn9_slot: &mut f64,
        var_qex_rv_slot: &mut f64,
        var_xg1_slot: &mut f64,
        var_xg1_db0_slot: &mut f64,
        var_xg1_db1_slot: &mut f64,
        var_xg1_dn0_slot: &mut f64,
        var_xg1_dn1_slot: &mut f64,
        var_xg1_dn10_slot: &mut f64,
        var_xg1_dn2_slot: &mut f64,
        var_xg1_dn3_slot: &mut f64,
        var_xg1_dn4_slot: &mut f64,
        var_xg1_dn5_slot: &mut f64,
        var_xg1_dn6_slot: &mut f64,
        var_xg1_dn7_slot: &mut f64,
        var_xg1_dn8_slot: &mut f64,
        var_xg1_dn9_slot: &mut f64,
        var_xg1_rdb0_slot: &mut f64,
        var_xg1_rdb1_slot: &mut f64,
        var_xg1_rdn0_slot: &mut f64,
        var_xg1_rdn1_slot: &mut f64,
        var_xg1_rdn10_slot: &mut f64,
        var_xg1_rdn2_slot: &mut f64,
        var_xg1_rdn3_slot: &mut f64,
        var_xg1_rdn4_slot: &mut f64,
        var_xg1_rdn5_slot: &mut f64,
        var_xg1_rdn6_slot: &mut f64,
        var_xg1_rdn7_slot: &mut f64,
        var_xg1_rdn8_slot: &mut f64,
        var_xg1_rdn9_slot: &mut f64,
        var_xg1_rv_slot: &mut f64,
        var_xg2_slot: &mut f64,
        var_xg2_db0_slot: &mut f64,
        var_xg2_db1_slot: &mut f64,
        var_xg2_dn0_slot: &mut f64,
        var_xg2_dn1_slot: &mut f64,
        var_xg2_dn10_slot: &mut f64,
        var_xg2_dn2_slot: &mut f64,
        var_xg2_dn3_slot: &mut f64,
        var_xg2_dn4_slot: &mut f64,
        var_xg2_dn5_slot: &mut f64,
        var_xg2_dn6_slot: &mut f64,
        var_xg2_dn7_slot: &mut f64,
        var_xg2_dn8_slot: &mut f64,
        var_xg2_dn9_slot: &mut f64,
        var_xg2_rdb0_slot: &mut f64,
        var_xg2_rdb1_slot: &mut f64,
        var_xg2_rdn0_slot: &mut f64,
        var_xg2_rdn1_slot: &mut f64,
        var_xg2_rdn10_slot: &mut f64,
        var_xg2_rdn2_slot: &mut f64,
        var_xg2_rdn3_slot: &mut f64,
        var_xg2_rdn4_slot: &mut f64,
        var_xg2_rdn5_slot: &mut f64,
        var_xg2_rdn6_slot: &mut f64,
        var_xg2_rdn7_slot: &mut f64,
        var_xg2_rdn8_slot: &mut f64,
        var_xg2_rdn9_slot: &mut f64,
        var_xg2_rv_slot: &mut f64,
        var_xnbex_slot: &mut f64,
        var_xnbex_db0_slot: &mut f64,
        var_xnbex_db1_slot: &mut f64,
        var_xnbex_dn0_slot: &mut f64,
        var_xnbex_dn1_slot: &mut f64,
        var_xnbex_dn10_slot: &mut f64,
        var_xnbex_dn2_slot: &mut f64,
        var_xnbex_dn3_slot: &mut f64,
        var_xnbex_dn4_slot: &mut f64,
        var_xnbex_dn5_slot: &mut f64,
        var_xnbex_dn6_slot: &mut f64,
        var_xnbex_dn7_slot: &mut f64,
        var_xnbex_dn8_slot: &mut f64,
        var_xnbex_dn9_slot: &mut f64,
        var_xnbex_rdb0_slot: &mut f64,
        var_xnbex_rdb1_slot: &mut f64,
        var_xnbex_rdn0_slot: &mut f64,
        var_xnbex_rdn1_slot: &mut f64,
        var_xnbex_rdn10_slot: &mut f64,
        var_xnbex_rdn2_slot: &mut f64,
        var_xnbex_rdn3_slot: &mut f64,
        var_xnbex_rdn4_slot: &mut f64,
        var_xnbex_rdn5_slot: &mut f64,
        var_xnbex_rdn6_slot: &mut f64,
        var_xnbex_rdn7_slot: &mut f64,
        var_xnbex_rdn8_slot: &mut f64,
        var_xnbex_rdn9_slot: &mut f64,
        var_xnbex_rv_slot: &mut f64,
        var_xpwex_slot: &mut f64,
        var_xpwex_db0_slot: &mut f64,
        var_xpwex_db1_slot: &mut f64,
        var_xpwex_dn0_slot: &mut f64,
        var_xpwex_dn1_slot: &mut f64,
        var_xpwex_dn10_slot: &mut f64,
        var_xpwex_dn2_slot: &mut f64,
        var_xpwex_dn3_slot: &mut f64,
        var_xpwex_dn4_slot: &mut f64,
        var_xpwex_dn5_slot: &mut f64,
        var_xpwex_dn6_slot: &mut f64,
        var_xpwex_dn7_slot: &mut f64,
        var_xpwex_dn8_slot: &mut f64,
        var_xpwex_dn9_slot: &mut f64,
        var_xpwex_rdb0_slot: &mut f64,
        var_xpwex_rdb1_slot: &mut f64,
        var_xpwex_rdn0_slot: &mut f64,
        var_xpwex_rdn1_slot: &mut f64,
        var_xpwex_rdn10_slot: &mut f64,
        var_xpwex_rdn2_slot: &mut f64,
        var_xpwex_rdn3_slot: &mut f64,
        var_xpwex_rdn4_slot: &mut f64,
        var_xpwex_rdn5_slot: &mut f64,
        var_xpwex_rdn6_slot: &mut f64,
        var_xpwex_rdn7_slot: &mut f64,
        var_xpwex_rdn8_slot: &mut f64,
        var_xpwex_rdn9_slot: &mut f64,
        var_xpwex_rv_slot: &mut f64,
        var_xqmex_slot: &mut f64,
        var_xqmex_db0_slot: &mut f64,
        var_xqmex_db1_slot: &mut f64,
        var_xqmex_dn0_slot: &mut f64,
        var_xqmex_dn1_slot: &mut f64,
        var_xqmex_dn10_slot: &mut f64,
        var_xqmex_dn2_slot: &mut f64,
        var_xqmex_dn3_slot: &mut f64,
        var_xqmex_dn4_slot: &mut f64,
        var_xqmex_dn5_slot: &mut f64,
        var_xqmex_dn6_slot: &mut f64,
        var_xqmex_dn7_slot: &mut f64,
        var_xqmex_dn8_slot: &mut f64,
        var_xqmex_dn9_slot: &mut f64,
        var_xqmex_rdb0_slot: &mut f64,
        var_xqmex_rdb1_slot: &mut f64,
        var_xqmex_rdn0_slot: &mut f64,
        var_xqmex_rdn1_slot: &mut f64,
        var_xqmex_rdn10_slot: &mut f64,
        var_xqmex_rdn2_slot: &mut f64,
        var_xqmex_rdn3_slot: &mut f64,
        var_xqmex_rdn4_slot: &mut f64,
        var_xqmex_rdn5_slot: &mut f64,
        var_xqmex_rdn6_slot: &mut f64,
        var_xqmex_rdn7_slot: &mut f64,
        var_xqmex_rdn8_slot: &mut f64,
        var_xqmex_rdn9_slot: &mut f64,
        var_xqmex_rv_slot: &mut f64,
    ) {
        let mut var_evb1c4vdcex: f64 = *var_evb1c4vdcex_slot;
        let mut var_evb1c4vdcex_db0: f64 = *var_evb1c4vdcex_db0_slot;
        let mut var_evb1c4vdcex_db1: f64 = *var_evb1c4vdcex_db1_slot;
        let mut var_evb1c4vdcex_dn0: f64 = *var_evb1c4vdcex_dn0_slot;
        let mut var_evb1c4vdcex_dn1: f64 = *var_evb1c4vdcex_dn1_slot;
        let mut var_evb1c4vdcex_dn10: f64 = *var_evb1c4vdcex_dn10_slot;
        let mut var_evb1c4vdcex_dn2: f64 = *var_evb1c4vdcex_dn2_slot;
        let mut var_evb1c4vdcex_dn3: f64 = *var_evb1c4vdcex_dn3_slot;
        let mut var_evb1c4vdcex_dn4: f64 = *var_evb1c4vdcex_dn4_slot;
        let mut var_evb1c4vdcex_dn5: f64 = *var_evb1c4vdcex_dn5_slot;
        let mut var_evb1c4vdcex_dn6: f64 = *var_evb1c4vdcex_dn6_slot;
        let mut var_evb1c4vdcex_dn7: f64 = *var_evb1c4vdcex_dn7_slot;
        let mut var_evb1c4vdcex_dn8: f64 = *var_evb1c4vdcex_dn8_slot;
        let mut var_evb1c4vdcex_dn9: f64 = *var_evb1c4vdcex_dn9_slot;
        let mut var_evb1c4vdcex_rdb0: f64 = *var_evb1c4vdcex_rdb0_slot;
        let mut var_evb1c4vdcex_rdb1: f64 = *var_evb1c4vdcex_rdb1_slot;
        let mut var_evb1c4vdcex_rdn0: f64 = *var_evb1c4vdcex_rdn0_slot;
        let mut var_evb1c4vdcex_rdn1: f64 = *var_evb1c4vdcex_rdn1_slot;
        let mut var_evb1c4vdcex_rdn10: f64 = *var_evb1c4vdcex_rdn10_slot;
        let mut var_evb1c4vdcex_rdn2: f64 = *var_evb1c4vdcex_rdn2_slot;
        let mut var_evb1c4vdcex_rdn3: f64 = *var_evb1c4vdcex_rdn3_slot;
        let mut var_evb1c4vdcex_rdn4: f64 = *var_evb1c4vdcex_rdn4_slot;
        let mut var_evb1c4vdcex_rdn5: f64 = *var_evb1c4vdcex_rdn5_slot;
        let mut var_evb1c4vdcex_rdn6: f64 = *var_evb1c4vdcex_rdn6_slot;
        let mut var_evb1c4vdcex_rdn7: f64 = *var_evb1c4vdcex_rdn7_slot;
        let mut var_evb1c4vdcex_rdn8: f64 = *var_evb1c4vdcex_rdn8_slot;
        let mut var_evb1c4vdcex_rdn9: f64 = *var_evb1c4vdcex_rdn9_slot;
        let mut var_evb1c4vdcex_rv: f64 = *var_evb1c4vdcex_rv_slot;
        let mut var_expl: f64 = *var_expl_slot;
        let mut var_expl_db0: f64 = *var_expl_db0_slot;
        let mut var_expl_db1: f64 = *var_expl_db1_slot;
        let mut var_expl_dn0: f64 = *var_expl_dn0_slot;
        let mut var_expl_dn1: f64 = *var_expl_dn1_slot;
        let mut var_expl_dn10: f64 = *var_expl_dn10_slot;
        let mut var_expl_dn2: f64 = *var_expl_dn2_slot;
        let mut var_expl_dn3: f64 = *var_expl_dn3_slot;
        let mut var_expl_dn4: f64 = *var_expl_dn4_slot;
        let mut var_expl_dn5: f64 = *var_expl_dn5_slot;
        let mut var_expl_dn6: f64 = *var_expl_dn6_slot;
        let mut var_expl_dn7: f64 = *var_expl_dn7_slot;
        let mut var_expl_dn8: f64 = *var_expl_dn8_slot;
        let mut var_expl_dn9: f64 = *var_expl_dn9_slot;
        let mut var_expl_rdb0: f64 = *var_expl_rdb0_slot;
        let mut var_expl_rdb1: f64 = *var_expl_rdb1_slot;
        let mut var_expl_rdn0: f64 = *var_expl_rdn0_slot;
        let mut var_expl_rdn1: f64 = *var_expl_rdn1_slot;
        let mut var_expl_rdn10: f64 = *var_expl_rdn10_slot;
        let mut var_expl_rdn2: f64 = *var_expl_rdn2_slot;
        let mut var_expl_rdn3: f64 = *var_expl_rdn3_slot;
        let mut var_expl_rdn4: f64 = *var_expl_rdn4_slot;
        let mut var_expl_rdn5: f64 = *var_expl_rdn5_slot;
        let mut var_expl_rdn6: f64 = *var_expl_rdn6_slot;
        let mut var_expl_rdn7: f64 = *var_expl_rdn7_slot;
        let mut var_expl_rdn8: f64 = *var_expl_rdn8_slot;
        let mut var_expl_rdn9: f64 = *var_expl_rdn9_slot;
        let mut var_expl_rv: f64 = *var_expl_rv_slot;
        let mut var_guard108: f64 = *var_guard108_slot;
        let mut var_guard108_db0: f64 = *var_guard108_db0_slot;
        let mut var_guard108_db1: f64 = *var_guard108_db1_slot;
        let mut var_guard108_dn0: f64 = *var_guard108_dn0_slot;
        let mut var_guard108_dn1: f64 = *var_guard108_dn1_slot;
        let mut var_guard108_dn10: f64 = *var_guard108_dn10_slot;
        let mut var_guard108_dn2: f64 = *var_guard108_dn2_slot;
        let mut var_guard108_dn3: f64 = *var_guard108_dn3_slot;
        let mut var_guard108_dn4: f64 = *var_guard108_dn4_slot;
        let mut var_guard108_dn5: f64 = *var_guard108_dn5_slot;
        let mut var_guard108_dn6: f64 = *var_guard108_dn6_slot;
        let mut var_guard108_dn7: f64 = *var_guard108_dn7_slot;
        let mut var_guard108_dn8: f64 = *var_guard108_dn8_slot;
        let mut var_guard108_dn9: f64 = *var_guard108_dn9_slot;
        let mut var_guard108_rdb0: f64 = *var_guard108_rdb0_slot;
        let mut var_guard108_rdb1: f64 = *var_guard108_rdb1_slot;
        let mut var_guard108_rdn0: f64 = *var_guard108_rdn0_slot;
        let mut var_guard108_rdn1: f64 = *var_guard108_rdn1_slot;
        let mut var_guard108_rdn10: f64 = *var_guard108_rdn10_slot;
        let mut var_guard108_rdn2: f64 = *var_guard108_rdn2_slot;
        let mut var_guard108_rdn3: f64 = *var_guard108_rdn3_slot;
        let mut var_guard108_rdn4: f64 = *var_guard108_rdn4_slot;
        let mut var_guard108_rdn5: f64 = *var_guard108_rdn5_slot;
        let mut var_guard108_rdn6: f64 = *var_guard108_rdn6_slot;
        let mut var_guard108_rdn7: f64 = *var_guard108_rdn7_slot;
        let mut var_guard108_rdn8: f64 = *var_guard108_rdn8_slot;
        let mut var_guard108_rdn9: f64 = *var_guard108_rdn9_slot;
        let mut var_guard108_rv: f64 = *var_guard108_rv_slot;
        let mut var_guard109: f64 = *var_guard109_slot;
        let mut var_guard109_db0: f64 = *var_guard109_db0_slot;
        let mut var_guard109_db1: f64 = *var_guard109_db1_slot;
        let mut var_guard109_dn0: f64 = *var_guard109_dn0_slot;
        let mut var_guard109_dn1: f64 = *var_guard109_dn1_slot;
        let mut var_guard109_dn10: f64 = *var_guard109_dn10_slot;
        let mut var_guard109_dn2: f64 = *var_guard109_dn2_slot;
        let mut var_guard109_dn3: f64 = *var_guard109_dn3_slot;
        let mut var_guard109_dn4: f64 = *var_guard109_dn4_slot;
        let mut var_guard109_dn5: f64 = *var_guard109_dn5_slot;
        let mut var_guard109_dn6: f64 = *var_guard109_dn6_slot;
        let mut var_guard109_dn7: f64 = *var_guard109_dn7_slot;
        let mut var_guard109_dn8: f64 = *var_guard109_dn8_slot;
        let mut var_guard109_dn9: f64 = *var_guard109_dn9_slot;
        let mut var_guard109_rdb0: f64 = *var_guard109_rdb0_slot;
        let mut var_guard109_rdb1: f64 = *var_guard109_rdb1_slot;
        let mut var_guard109_rdn0: f64 = *var_guard109_rdn0_slot;
        let mut var_guard109_rdn1: f64 = *var_guard109_rdn1_slot;
        let mut var_guard109_rdn10: f64 = *var_guard109_rdn10_slot;
        let mut var_guard109_rdn2: f64 = *var_guard109_rdn2_slot;
        let mut var_guard109_rdn3: f64 = *var_guard109_rdn3_slot;
        let mut var_guard109_rdn4: f64 = *var_guard109_rdn4_slot;
        let mut var_guard109_rdn5: f64 = *var_guard109_rdn5_slot;
        let mut var_guard109_rdn6: f64 = *var_guard109_rdn6_slot;
        let mut var_guard109_rdn7: f64 = *var_guard109_rdn7_slot;
        let mut var_guard109_rdn8: f64 = *var_guard109_rdn8_slot;
        let mut var_guard109_rdn9: f64 = *var_guard109_rdn9_slot;
        let mut var_guard109_rv: f64 = *var_guard109_rv_slot;
        let mut var_guard110: f64 = *var_guard110_slot;
        let mut var_guard110_db0: f64 = *var_guard110_db0_slot;
        let mut var_guard110_db1: f64 = *var_guard110_db1_slot;
        let mut var_guard110_dn0: f64 = *var_guard110_dn0_slot;
        let mut var_guard110_dn1: f64 = *var_guard110_dn1_slot;
        let mut var_guard110_dn10: f64 = *var_guard110_dn10_slot;
        let mut var_guard110_dn2: f64 = *var_guard110_dn2_slot;
        let mut var_guard110_dn3: f64 = *var_guard110_dn3_slot;
        let mut var_guard110_dn4: f64 = *var_guard110_dn4_slot;
        let mut var_guard110_dn5: f64 = *var_guard110_dn5_slot;
        let mut var_guard110_dn6: f64 = *var_guard110_dn6_slot;
        let mut var_guard110_dn7: f64 = *var_guard110_dn7_slot;
        let mut var_guard110_dn8: f64 = *var_guard110_dn8_slot;
        let mut var_guard110_dn9: f64 = *var_guard110_dn9_slot;
        let mut var_guard110_rdb0: f64 = *var_guard110_rdb0_slot;
        let mut var_guard110_rdb1: f64 = *var_guard110_rdb1_slot;
        let mut var_guard110_rdn0: f64 = *var_guard110_rdn0_slot;
        let mut var_guard110_rdn1: f64 = *var_guard110_rdn1_slot;
        let mut var_guard110_rdn10: f64 = *var_guard110_rdn10_slot;
        let mut var_guard110_rdn2: f64 = *var_guard110_rdn2_slot;
        let mut var_guard110_rdn3: f64 = *var_guard110_rdn3_slot;
        let mut var_guard110_rdn4: f64 = *var_guard110_rdn4_slot;
        let mut var_guard110_rdn5: f64 = *var_guard110_rdn5_slot;
        let mut var_guard110_rdn6: f64 = *var_guard110_rdn6_slot;
        let mut var_guard110_rdn7: f64 = *var_guard110_rdn7_slot;
        let mut var_guard110_rdn8: f64 = *var_guard110_rdn8_slot;
        let mut var_guard110_rdn9: f64 = *var_guard110_rdn9_slot;
        let mut var_guard110_rv: f64 = *var_guard110_rv_slot;
        let mut var_qex: f64 = *var_qex_slot;
        let mut var_qex_db0: f64 = *var_qex_db0_slot;
        let mut var_qex_db1: f64 = *var_qex_db1_slot;
        let mut var_qex_dn0: f64 = *var_qex_dn0_slot;
        let mut var_qex_dn1: f64 = *var_qex_dn1_slot;
        let mut var_qex_dn10: f64 = *var_qex_dn10_slot;
        let mut var_qex_dn2: f64 = *var_qex_dn2_slot;
        let mut var_qex_dn3: f64 = *var_qex_dn3_slot;
        let mut var_qex_dn4: f64 = *var_qex_dn4_slot;
        let mut var_qex_dn5: f64 = *var_qex_dn5_slot;
        let mut var_qex_dn6: f64 = *var_qex_dn6_slot;
        let mut var_qex_dn7: f64 = *var_qex_dn7_slot;
        let mut var_qex_dn8: f64 = *var_qex_dn8_slot;
        let mut var_qex_dn9: f64 = *var_qex_dn9_slot;
        let mut var_qex_rdb0: f64 = *var_qex_rdb0_slot;
        let mut var_qex_rdb1: f64 = *var_qex_rdb1_slot;
        let mut var_qex_rdn0: f64 = *var_qex_rdn0_slot;
        let mut var_qex_rdn1: f64 = *var_qex_rdn1_slot;
        let mut var_qex_rdn10: f64 = *var_qex_rdn10_slot;
        let mut var_qex_rdn2: f64 = *var_qex_rdn2_slot;
        let mut var_qex_rdn3: f64 = *var_qex_rdn3_slot;
        let mut var_qex_rdn4: f64 = *var_qex_rdn4_slot;
        let mut var_qex_rdn5: f64 = *var_qex_rdn5_slot;
        let mut var_qex_rdn6: f64 = *var_qex_rdn6_slot;
        let mut var_qex_rdn7: f64 = *var_qex_rdn7_slot;
        let mut var_qex_rdn8: f64 = *var_qex_rdn8_slot;
        let mut var_qex_rdn9: f64 = *var_qex_rdn9_slot;
        let mut var_qex_rv: f64 = *var_qex_rv_slot;
        let mut var_xg1: f64 = *var_xg1_slot;
        let mut var_xg1_db0: f64 = *var_xg1_db0_slot;
        let mut var_xg1_db1: f64 = *var_xg1_db1_slot;
        let mut var_xg1_dn0: f64 = *var_xg1_dn0_slot;
        let mut var_xg1_dn1: f64 = *var_xg1_dn1_slot;
        let mut var_xg1_dn10: f64 = *var_xg1_dn10_slot;
        let mut var_xg1_dn2: f64 = *var_xg1_dn2_slot;
        let mut var_xg1_dn3: f64 = *var_xg1_dn3_slot;
        let mut var_xg1_dn4: f64 = *var_xg1_dn4_slot;
        let mut var_xg1_dn5: f64 = *var_xg1_dn5_slot;
        let mut var_xg1_dn6: f64 = *var_xg1_dn6_slot;
        let mut var_xg1_dn7: f64 = *var_xg1_dn7_slot;
        let mut var_xg1_dn8: f64 = *var_xg1_dn8_slot;
        let mut var_xg1_dn9: f64 = *var_xg1_dn9_slot;
        let mut var_xg1_rdb0: f64 = *var_xg1_rdb0_slot;
        let mut var_xg1_rdb1: f64 = *var_xg1_rdb1_slot;
        let mut var_xg1_rdn0: f64 = *var_xg1_rdn0_slot;
        let mut var_xg1_rdn1: f64 = *var_xg1_rdn1_slot;
        let mut var_xg1_rdn10: f64 = *var_xg1_rdn10_slot;
        let mut var_xg1_rdn2: f64 = *var_xg1_rdn2_slot;
        let mut var_xg1_rdn3: f64 = *var_xg1_rdn3_slot;
        let mut var_xg1_rdn4: f64 = *var_xg1_rdn4_slot;
        let mut var_xg1_rdn5: f64 = *var_xg1_rdn5_slot;
        let mut var_xg1_rdn6: f64 = *var_xg1_rdn6_slot;
        let mut var_xg1_rdn7: f64 = *var_xg1_rdn7_slot;
        let mut var_xg1_rdn8: f64 = *var_xg1_rdn8_slot;
        let mut var_xg1_rdn9: f64 = *var_xg1_rdn9_slot;
        let mut var_xg1_rv: f64 = *var_xg1_rv_slot;
        let mut var_xg2: f64 = *var_xg2_slot;
        let mut var_xg2_db0: f64 = *var_xg2_db0_slot;
        let mut var_xg2_db1: f64 = *var_xg2_db1_slot;
        let mut var_xg2_dn0: f64 = *var_xg2_dn0_slot;
        let mut var_xg2_dn1: f64 = *var_xg2_dn1_slot;
        let mut var_xg2_dn10: f64 = *var_xg2_dn10_slot;
        let mut var_xg2_dn2: f64 = *var_xg2_dn2_slot;
        let mut var_xg2_dn3: f64 = *var_xg2_dn3_slot;
        let mut var_xg2_dn4: f64 = *var_xg2_dn4_slot;
        let mut var_xg2_dn5: f64 = *var_xg2_dn5_slot;
        let mut var_xg2_dn6: f64 = *var_xg2_dn6_slot;
        let mut var_xg2_dn7: f64 = *var_xg2_dn7_slot;
        let mut var_xg2_dn8: f64 = *var_xg2_dn8_slot;
        let mut var_xg2_dn9: f64 = *var_xg2_dn9_slot;
        let mut var_xg2_rdb0: f64 = *var_xg2_rdb0_slot;
        let mut var_xg2_rdb1: f64 = *var_xg2_rdb1_slot;
        let mut var_xg2_rdn0: f64 = *var_xg2_rdn0_slot;
        let mut var_xg2_rdn1: f64 = *var_xg2_rdn1_slot;
        let mut var_xg2_rdn10: f64 = *var_xg2_rdn10_slot;
        let mut var_xg2_rdn2: f64 = *var_xg2_rdn2_slot;
        let mut var_xg2_rdn3: f64 = *var_xg2_rdn3_slot;
        let mut var_xg2_rdn4: f64 = *var_xg2_rdn4_slot;
        let mut var_xg2_rdn5: f64 = *var_xg2_rdn5_slot;
        let mut var_xg2_rdn6: f64 = *var_xg2_rdn6_slot;
        let mut var_xg2_rdn7: f64 = *var_xg2_rdn7_slot;
        let mut var_xg2_rdn8: f64 = *var_xg2_rdn8_slot;
        let mut var_xg2_rdn9: f64 = *var_xg2_rdn9_slot;
        let mut var_xg2_rv: f64 = *var_xg2_rv_slot;
        let mut var_xnbex: f64 = *var_xnbex_slot;
        let mut var_xnbex_db0: f64 = *var_xnbex_db0_slot;
        let mut var_xnbex_db1: f64 = *var_xnbex_db1_slot;
        let mut var_xnbex_dn0: f64 = *var_xnbex_dn0_slot;
        let mut var_xnbex_dn1: f64 = *var_xnbex_dn1_slot;
        let mut var_xnbex_dn10: f64 = *var_xnbex_dn10_slot;
        let mut var_xnbex_dn2: f64 = *var_xnbex_dn2_slot;
        let mut var_xnbex_dn3: f64 = *var_xnbex_dn3_slot;
        let mut var_xnbex_dn4: f64 = *var_xnbex_dn4_slot;
        let mut var_xnbex_dn5: f64 = *var_xnbex_dn5_slot;
        let mut var_xnbex_dn6: f64 = *var_xnbex_dn6_slot;
        let mut var_xnbex_dn7: f64 = *var_xnbex_dn7_slot;
        let mut var_xnbex_dn8: f64 = *var_xnbex_dn8_slot;
        let mut var_xnbex_dn9: f64 = *var_xnbex_dn9_slot;
        let mut var_xnbex_rdb0: f64 = *var_xnbex_rdb0_slot;
        let mut var_xnbex_rdb1: f64 = *var_xnbex_rdb1_slot;
        let mut var_xnbex_rdn0: f64 = *var_xnbex_rdn0_slot;
        let mut var_xnbex_rdn1: f64 = *var_xnbex_rdn1_slot;
        let mut var_xnbex_rdn10: f64 = *var_xnbex_rdn10_slot;
        let mut var_xnbex_rdn2: f64 = *var_xnbex_rdn2_slot;
        let mut var_xnbex_rdn3: f64 = *var_xnbex_rdn3_slot;
        let mut var_xnbex_rdn4: f64 = *var_xnbex_rdn4_slot;
        let mut var_xnbex_rdn5: f64 = *var_xnbex_rdn5_slot;
        let mut var_xnbex_rdn6: f64 = *var_xnbex_rdn6_slot;
        let mut var_xnbex_rdn7: f64 = *var_xnbex_rdn7_slot;
        let mut var_xnbex_rdn8: f64 = *var_xnbex_rdn8_slot;
        let mut var_xnbex_rdn9: f64 = *var_xnbex_rdn9_slot;
        let mut var_xnbex_rv: f64 = *var_xnbex_rv_slot;
        let mut var_xpwex: f64 = *var_xpwex_slot;
        let mut var_xpwex_db0: f64 = *var_xpwex_db0_slot;
        let mut var_xpwex_db1: f64 = *var_xpwex_db1_slot;
        let mut var_xpwex_dn0: f64 = *var_xpwex_dn0_slot;
        let mut var_xpwex_dn1: f64 = *var_xpwex_dn1_slot;
        let mut var_xpwex_dn10: f64 = *var_xpwex_dn10_slot;
        let mut var_xpwex_dn2: f64 = *var_xpwex_dn2_slot;
        let mut var_xpwex_dn3: f64 = *var_xpwex_dn3_slot;
        let mut var_xpwex_dn4: f64 = *var_xpwex_dn4_slot;
        let mut var_xpwex_dn5: f64 = *var_xpwex_dn5_slot;
        let mut var_xpwex_dn6: f64 = *var_xpwex_dn6_slot;
        let mut var_xpwex_dn7: f64 = *var_xpwex_dn7_slot;
        let mut var_xpwex_dn8: f64 = *var_xpwex_dn8_slot;
        let mut var_xpwex_dn9: f64 = *var_xpwex_dn9_slot;
        let mut var_xpwex_rdb0: f64 = *var_xpwex_rdb0_slot;
        let mut var_xpwex_rdb1: f64 = *var_xpwex_rdb1_slot;
        let mut var_xpwex_rdn0: f64 = *var_xpwex_rdn0_slot;
        let mut var_xpwex_rdn1: f64 = *var_xpwex_rdn1_slot;
        let mut var_xpwex_rdn10: f64 = *var_xpwex_rdn10_slot;
        let mut var_xpwex_rdn2: f64 = *var_xpwex_rdn2_slot;
        let mut var_xpwex_rdn3: f64 = *var_xpwex_rdn3_slot;
        let mut var_xpwex_rdn4: f64 = *var_xpwex_rdn4_slot;
        let mut var_xpwex_rdn5: f64 = *var_xpwex_rdn5_slot;
        let mut var_xpwex_rdn6: f64 = *var_xpwex_rdn6_slot;
        let mut var_xpwex_rdn7: f64 = *var_xpwex_rdn7_slot;
        let mut var_xpwex_rdn8: f64 = *var_xpwex_rdn8_slot;
        let mut var_xpwex_rdn9: f64 = *var_xpwex_rdn9_slot;
        let mut var_xpwex_rv: f64 = *var_xpwex_rv_slot;
        let mut var_xqmex: f64 = *var_xqmex_slot;
        let mut var_xqmex_db0: f64 = *var_xqmex_db0_slot;
        let mut var_xqmex_db1: f64 = *var_xqmex_db1_slot;
        let mut var_xqmex_dn0: f64 = *var_xqmex_dn0_slot;
        let mut var_xqmex_dn1: f64 = *var_xqmex_dn1_slot;
        let mut var_xqmex_dn10: f64 = *var_xqmex_dn10_slot;
        let mut var_xqmex_dn2: f64 = *var_xqmex_dn2_slot;
        let mut var_xqmex_dn3: f64 = *var_xqmex_dn3_slot;
        let mut var_xqmex_dn4: f64 = *var_xqmex_dn4_slot;
        let mut var_xqmex_dn5: f64 = *var_xqmex_dn5_slot;
        let mut var_xqmex_dn6: f64 = *var_xqmex_dn6_slot;
        let mut var_xqmex_dn7: f64 = *var_xqmex_dn7_slot;
        let mut var_xqmex_dn8: f64 = *var_xqmex_dn8_slot;
        let mut var_xqmex_dn9: f64 = *var_xqmex_dn9_slot;
        let mut var_xqmex_rdb0: f64 = *var_xqmex_rdb0_slot;
        let mut var_xqmex_rdb1: f64 = *var_xqmex_rdb1_slot;
        let mut var_xqmex_rdn0: f64 = *var_xqmex_rdn0_slot;
        let mut var_xqmex_rdn1: f64 = *var_xqmex_rdn1_slot;
        let mut var_xqmex_rdn10: f64 = *var_xqmex_rdn10_slot;
        let mut var_xqmex_rdn2: f64 = *var_xqmex_rdn2_slot;
        let mut var_xqmex_rdn3: f64 = *var_xqmex_rdn3_slot;
        let mut var_xqmex_rdn4: f64 = *var_xqmex_rdn4_slot;
        let mut var_xqmex_rdn5: f64 = *var_xqmex_rdn5_slot;
        let mut var_xqmex_rdn6: f64 = *var_xqmex_rdn6_slot;
        let mut var_xqmex_rdn7: f64 = *var_xqmex_rdn7_slot;
        let mut var_xqmex_rdn8: f64 = *var_xqmex_rdn8_slot;
        let mut var_xqmex_rdn9: f64 = *var_xqmex_rdn9_slot;
        let mut var_xqmex_rv: f64 = *var_xqmex_rv_slot;

        let assign5960_e6025: f64 = (var_vb1c4 - var_vdcex_t);
        let assign5960_e6027: f64 = (assign5960_e6025 / p.p90);
        let assign5960_e6029: f64 = (assign5960_e6027 * var_vtinv);
        let assign5960_e6031: f64 = if assign5960_e6029 < p.p134 { 1.0 } else { 0.0 };
        var_guard108 = assign5960_e6031;
        var_guard108_dn0 = 0.0;
        var_guard108_dn1 = 0.0;
        var_guard108_dn2 = 0.0;
        var_guard108_dn3 = 0.0;
        var_guard108_dn4 = 0.0;
        var_guard108_dn5 = 0.0;
        var_guard108_dn6 = 0.0;
        var_guard108_dn7 = 0.0;
        var_guard108_dn8 = 0.0;
        var_guard108_dn9 = 0.0;
        var_guard108_dn10 = 0.0;
        var_guard108_db0 = 0.0;
        var_guard108_db1 = 0.0;
        var_guard108_rv = 0.0;
        var_guard108_rdn0 = 0.0;
        var_guard108_rdn1 = 0.0;
        var_guard108_rdn2 = 0.0;
        var_guard108_rdn3 = 0.0;
        var_guard108_rdn4 = 0.0;
        var_guard108_rdn5 = 0.0;
        var_guard108_rdn6 = 0.0;
        var_guard108_rdn7 = 0.0;
        var_guard108_rdn8 = 0.0;
        var_guard108_rdn9 = 0.0;
        var_guard108_rdn10 = 0.0;
        var_guard108_rdb0 = 0.0;
        var_guard108_rdb1 = 0.0;

        let (assign5970_e6045, assign5970_e6045_d_n0, assign5970_e6045_d_n1, assign5970_e6045_d_n2, assign5970_e6045_d_n3, assign5970_e6045_d_n4, assign5970_e6045_d_n5, assign5970_e6045_d_n6, assign5970_e6045_d_n7, assign5970_e6045_d_n8, assign5970_e6045_d_n9, assign5970_e6045_d_n10, assign5970_e6045_d_b0, assign5970_e6045_d_b1,) = {
    if ((var_guard107 == 0.0) && (var_guard108 != 0.0)) {
        let assign5970_e6038: f64 = (var_vb1c4 - var_vdcex_t);
        let assign5970_e6040: f64 = (assign5970_e6038 / p.p90);
        let assign5970_e6042: f64 = (assign5970_e6040 * var_vtinv);
        let assign5970_e6043: f64 = (assign5970_e6042).exp();
        (assign5970_e6043, (assign5970_e6043 * ((((var_vb1c4_dn0 - var_vdcex_t_dn0) / p.p90) * var_vtinv) + (assign5970_e6040 * var_vtinv_dn0))), (assign5970_e6043 * ((((var_vb1c4_dn1 - var_vdcex_t_dn1) / p.p90) * var_vtinv) + (assign5970_e6040 * var_vtinv_dn1))), (assign5970_e6043 * ((((var_vb1c4_dn2 - var_vdcex_t_dn2) / p.p90) * var_vtinv) + (assign5970_e6040 * var_vtinv_dn2))), (assign5970_e6043 * ((((var_vb1c4_dn3 - var_vdcex_t_dn3) / p.p90) * var_vtinv) + (assign5970_e6040 * var_vtinv_dn3))), (assign5970_e6043 * ((((var_vb1c4_dn4 - var_vdcex_t_dn4) / p.p90) * var_vtinv) + (assign5970_e6040 * var_vtinv_dn4))), (assign5970_e6043 * ((((var_vb1c4_dn5 - var_vdcex_t_dn5) / p.p90) * var_vtinv) + (assign5970_e6040 * var_vtinv_dn5))), (assign5970_e6043 * ((((var_vb1c4_dn6 - var_vdcex_t_dn6) / p.p90) * var_vtinv) + (assign5970_e6040 * var_vtinv_dn6))), (assign5970_e6043 * ((((var_vb1c4_dn7 - var_vdcex_t_dn7) / p.p90) * var_vtinv) + (assign5970_e6040 * var_vtinv_dn7))), (assign5970_e6043 * ((((var_vb1c4_dn8 - var_vdcex_t_dn8) / p.p90) * var_vtinv) + (assign5970_e6040 * var_vtinv_dn8))), (assign5970_e6043 * ((((var_vb1c4_dn9 - var_vdcex_t_dn9) / p.p90) * var_vtinv) + (assign5970_e6040 * var_vtinv_dn9))), (assign5970_e6043 * ((((var_vb1c4_dn10 - var_vdcex_t_dn10) / p.p90) * var_vtinv) + (assign5970_e6040 * var_vtinv_dn10))), (assign5970_e6043 * ((((var_vb1c4_db0 - var_vdcex_t_db0) / p.p90) * var_vtinv) + (assign5970_e6040 * var_vtinv_db0))), (assign5970_e6043 * ((((var_vb1c4_db1 - var_vdcex_t_db1) / p.p90) * var_vtinv) + (assign5970_e6040 * var_vtinv_db1))),)
    } else {
        (var_evb1c4vdcex, var_evb1c4vdcex_dn0, var_evb1c4vdcex_dn1, var_evb1c4vdcex_dn2, var_evb1c4vdcex_dn3, var_evb1c4vdcex_dn4, var_evb1c4vdcex_dn5, var_evb1c4vdcex_dn6, var_evb1c4vdcex_dn7, var_evb1c4vdcex_dn8, var_evb1c4vdcex_dn9, var_evb1c4vdcex_dn10, var_evb1c4vdcex_db0, var_evb1c4vdcex_db1,)
    }
};
        var_evb1c4vdcex = assign5970_e6045;
        var_evb1c4vdcex_dn0 = assign5970_e6045_d_n0;
        var_evb1c4vdcex_dn1 = assign5970_e6045_d_n1;
        var_evb1c4vdcex_dn2 = assign5970_e6045_d_n2;
        var_evb1c4vdcex_dn3 = assign5970_e6045_d_n3;
        var_evb1c4vdcex_dn4 = assign5970_e6045_d_n4;
        var_evb1c4vdcex_dn5 = assign5970_e6045_d_n5;
        var_evb1c4vdcex_dn6 = assign5970_e6045_d_n6;
        var_evb1c4vdcex_dn7 = assign5970_e6045_d_n7;
        var_evb1c4vdcex_dn8 = assign5970_e6045_d_n8;
        var_evb1c4vdcex_dn9 = assign5970_e6045_d_n9;
        var_evb1c4vdcex_dn10 = assign5970_e6045_d_n10;
        var_evb1c4vdcex_db0 = assign5970_e6045_d_b0;
        var_evb1c4vdcex_db1 = assign5970_e6045_d_b1;
        var_evb1c4vdcex_rv = 0.0;
        var_evb1c4vdcex_rdn0 = 0.0;
        var_evb1c4vdcex_rdn1 = 0.0;
        var_evb1c4vdcex_rdn2 = 0.0;
        var_evb1c4vdcex_rdn3 = 0.0;
        var_evb1c4vdcex_rdn4 = 0.0;
        var_evb1c4vdcex_rdn5 = 0.0;
        var_evb1c4vdcex_rdn6 = 0.0;
        var_evb1c4vdcex_rdn7 = 0.0;
        var_evb1c4vdcex_rdn8 = 0.0;
        var_evb1c4vdcex_rdn9 = 0.0;
        var_evb1c4vdcex_rdn10 = 0.0;
        var_evb1c4vdcex_rdb0 = 0.0;
        var_evb1c4vdcex_rdb1 = 0.0;

        let (assign5980_e6054, assign5980_e6054_d_n0, assign5980_e6054_d_n1, assign5980_e6054_d_n2, assign5980_e6054_d_n3, assign5980_e6054_d_n4, assign5980_e6054_d_n5, assign5980_e6054_d_n6, assign5980_e6054_d_n7, assign5980_e6054_d_n8, assign5980_e6054_d_n9, assign5980_e6054_d_n10, assign5980_e6054_d_b0, assign5980_e6054_d_b1,) = {
    if ((var_guard107 == 0.0) && (var_guard108 == 0.0)) {
        let assign5980_e6052: f64 = (p.p134).exp();
        (assign5980_e6052, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_expl, var_expl_dn0, var_expl_dn1, var_expl_dn2, var_expl_dn3, var_expl_dn4, var_expl_dn5, var_expl_dn6, var_expl_dn7, var_expl_dn8, var_expl_dn9, var_expl_dn10, var_expl_db0, var_expl_db1,)
    }
};
        var_expl = assign5980_e6054;
        var_expl_dn0 = assign5980_e6054_d_n0;
        var_expl_dn1 = assign5980_e6054_d_n1;
        var_expl_dn2 = assign5980_e6054_d_n2;
        var_expl_dn3 = assign5980_e6054_d_n3;
        var_expl_dn4 = assign5980_e6054_d_n4;
        var_expl_dn5 = assign5980_e6054_d_n5;
        var_expl_dn6 = assign5980_e6054_d_n6;
        var_expl_dn7 = assign5980_e6054_d_n7;
        var_expl_dn8 = assign5980_e6054_d_n8;
        var_expl_dn9 = assign5980_e6054_d_n9;
        var_expl_dn10 = assign5980_e6054_d_n10;
        var_expl_db0 = assign5980_e6054_d_b0;
        var_expl_db1 = assign5980_e6054_d_b1;
        var_expl_rv = 0.0;
        var_expl_rdn0 = 0.0;
        var_expl_rdn1 = 0.0;
        var_expl_rdn2 = 0.0;
        var_expl_rdn3 = 0.0;
        var_expl_rdn4 = 0.0;
        var_expl_rdn5 = 0.0;
        var_expl_rdn6 = 0.0;
        var_expl_rdn7 = 0.0;
        var_expl_rdn8 = 0.0;
        var_expl_rdn9 = 0.0;
        var_expl_rdn10 = 0.0;
        var_expl_rdb0 = 0.0;
        var_expl_rdb1 = 0.0;

        let (assign5990_e6074, assign5990_e6074_d_n0, assign5990_e6074_d_n1, assign5990_e6074_d_n2, assign5990_e6074_d_n3, assign5990_e6074_d_n4, assign5990_e6074_d_n5, assign5990_e6074_d_n6, assign5990_e6074_d_n7, assign5990_e6074_d_n8, assign5990_e6074_d_n9, assign5990_e6074_d_n10, assign5990_e6074_d_b0, assign5990_e6074_d_b1,) = {
    if ((var_guard107 == 0.0) && (var_guard108 == 0.0)) {
        let assign5990_e6064: f64 = (var_vb1c4 - var_vdcex_t);
        let assign5990_e6066: f64 = (assign5990_e6064 / p.p90);
        let assign5990_e6068: f64 = (assign5990_e6066 * var_vtinv);
        let assign5990_e6070: f64 = (assign5990_e6068 - p.p134);
        let assign5990_e6071: f64 = (1.0 + assign5990_e6070);
        let assign5990_e6072: f64 = (var_expl * assign5990_e6071);
        (assign5990_e6072, ((var_expl_dn0 * assign5990_e6071) + (var_expl * ((((var_vb1c4_dn0 - var_vdcex_t_dn0) / p.p90) * var_vtinv) + (assign5990_e6066 * var_vtinv_dn0)))), ((var_expl_dn1 * assign5990_e6071) + (var_expl * ((((var_vb1c4_dn1 - var_vdcex_t_dn1) / p.p90) * var_vtinv) + (assign5990_e6066 * var_vtinv_dn1)))), ((var_expl_dn2 * assign5990_e6071) + (var_expl * ((((var_vb1c4_dn2 - var_vdcex_t_dn2) / p.p90) * var_vtinv) + (assign5990_e6066 * var_vtinv_dn2)))), ((var_expl_dn3 * assign5990_e6071) + (var_expl * ((((var_vb1c4_dn3 - var_vdcex_t_dn3) / p.p90) * var_vtinv) + (assign5990_e6066 * var_vtinv_dn3)))), ((var_expl_dn4 * assign5990_e6071) + (var_expl * ((((var_vb1c4_dn4 - var_vdcex_t_dn4) / p.p90) * var_vtinv) + (assign5990_e6066 * var_vtinv_dn4)))), ((var_expl_dn5 * assign5990_e6071) + (var_expl * ((((var_vb1c4_dn5 - var_vdcex_t_dn5) / p.p90) * var_vtinv) + (assign5990_e6066 * var_vtinv_dn5)))), ((var_expl_dn6 * assign5990_e6071) + (var_expl * ((((var_vb1c4_dn6 - var_vdcex_t_dn6) / p.p90) * var_vtinv) + (assign5990_e6066 * var_vtinv_dn6)))), ((var_expl_dn7 * assign5990_e6071) + (var_expl * ((((var_vb1c4_dn7 - var_vdcex_t_dn7) / p.p90) * var_vtinv) + (assign5990_e6066 * var_vtinv_dn7)))), ((var_expl_dn8 * assign5990_e6071) + (var_expl * ((((var_vb1c4_dn8 - var_vdcex_t_dn8) / p.p90) * var_vtinv) + (assign5990_e6066 * var_vtinv_dn8)))), ((var_expl_dn9 * assign5990_e6071) + (var_expl * ((((var_vb1c4_dn9 - var_vdcex_t_dn9) / p.p90) * var_vtinv) + (assign5990_e6066 * var_vtinv_dn9)))), ((var_expl_dn10 * assign5990_e6071) + (var_expl * ((((var_vb1c4_dn10 - var_vdcex_t_dn10) / p.p90) * var_vtinv) + (assign5990_e6066 * var_vtinv_dn10)))), ((var_expl_db0 * assign5990_e6071) + (var_expl * ((((var_vb1c4_db0 - var_vdcex_t_db0) / p.p90) * var_vtinv) + (assign5990_e6066 * var_vtinv_db0)))), ((var_expl_db1 * assign5990_e6071) + (var_expl * ((((var_vb1c4_db1 - var_vdcex_t_db1) / p.p90) * var_vtinv) + (assign5990_e6066 * var_vtinv_db1)))),)
    } else {
        (var_evb1c4vdcex, var_evb1c4vdcex_dn0, var_evb1c4vdcex_dn1, var_evb1c4vdcex_dn2, var_evb1c4vdcex_dn3, var_evb1c4vdcex_dn4, var_evb1c4vdcex_dn5, var_evb1c4vdcex_dn6, var_evb1c4vdcex_dn7, var_evb1c4vdcex_dn8, var_evb1c4vdcex_dn9, var_evb1c4vdcex_dn10, var_evb1c4vdcex_db0, var_evb1c4vdcex_db1,)
    }
};
        var_evb1c4vdcex = assign5990_e6074;
        var_evb1c4vdcex_dn0 = assign5990_e6074_d_n0;
        var_evb1c4vdcex_dn1 = assign5990_e6074_d_n1;
        var_evb1c4vdcex_dn2 = assign5990_e6074_d_n2;
        var_evb1c4vdcex_dn3 = assign5990_e6074_d_n3;
        var_evb1c4vdcex_dn4 = assign5990_e6074_d_n4;
        var_evb1c4vdcex_dn5 = assign5990_e6074_d_n5;
        var_evb1c4vdcex_dn6 = assign5990_e6074_d_n6;
        var_evb1c4vdcex_dn7 = assign5990_e6074_d_n7;
        var_evb1c4vdcex_dn8 = assign5990_e6074_d_n8;
        var_evb1c4vdcex_dn9 = assign5990_e6074_d_n9;
        var_evb1c4vdcex_dn10 = assign5990_e6074_d_n10;
        var_evb1c4vdcex_db0 = assign5990_e6074_d_b0;
        var_evb1c4vdcex_db1 = assign5990_e6074_d_b1;
        var_evb1c4vdcex_rv = 0.0;
        var_evb1c4vdcex_rdn0 = 0.0;
        var_evb1c4vdcex_rdn1 = 0.0;
        var_evb1c4vdcex_rdn2 = 0.0;
        var_evb1c4vdcex_rdn3 = 0.0;
        var_evb1c4vdcex_rdn4 = 0.0;
        var_evb1c4vdcex_rdn5 = 0.0;
        var_evb1c4vdcex_rdn6 = 0.0;
        var_evb1c4vdcex_rdn7 = 0.0;
        var_evb1c4vdcex_rdn8 = 0.0;
        var_evb1c4vdcex_rdn9 = 0.0;
        var_evb1c4vdcex_rdn10 = 0.0;
        var_evb1c4vdcex_rdb0 = 0.0;
        var_evb1c4vdcex_rdb1 = 0.0;

        let (assign6000_e6094, assign6000_e6094_d_n0, assign6000_e6094_d_n1, assign6000_e6094_d_n2, assign6000_e6094_d_n3, assign6000_e6094_d_n4, assign6000_e6094_d_n5, assign6000_e6094_d_n6, assign6000_e6094_d_n7, assign6000_e6094_d_n8, assign6000_e6094_d_n9, assign6000_e6094_d_n10, assign6000_e6094_d_b0, assign6000_e6094_d_b1,) = {
    if (var_guard107 == 0.0) {
        let assign6000_e6079: f64 = (2.0 * var_ibx_t);
        let assign6000_e6081: f64 = (assign6000_e6079 * var_tauex_t);
        let assign6000_e6083: f64 = (assign6000_e6081 * var_evb1c4);
        let assign6000_e6088: f64 = (4.0 * var_evb1c4vdcex);
        let assign6000_e6089: f64 = (1.0 + assign6000_e6088);
        let assign6000_e6090: f64 = (assign6000_e6089).sqrt();
        let assign6000_e6091: f64 = (1.0 + assign6000_e6090);
        let assign6000_e6092: f64 = (assign6000_e6083 / assign6000_e6091);
        (assign6000_e6092, ((((((((2.0 * var_ibx_t_dn0) * var_tauex_t) + (assign6000_e6079 * var_tauex_t_dn0)) * var_evb1c4) + (assign6000_e6081 * var_evb1c4_dn0)) * assign6000_e6091) - (assign6000_e6083 * ((4.0 * var_evb1c4vdcex_dn0) / (2.0 * assign6000_e6090)))) / (assign6000_e6091 * assign6000_e6091)), ((((((((2.0 * var_ibx_t_dn1) * var_tauex_t) + (assign6000_e6079 * var_tauex_t_dn1)) * var_evb1c4) + (assign6000_e6081 * var_evb1c4_dn1)) * assign6000_e6091) - (assign6000_e6083 * ((4.0 * var_evb1c4vdcex_dn1) / (2.0 * assign6000_e6090)))) / (assign6000_e6091 * assign6000_e6091)), ((((((((2.0 * var_ibx_t_dn2) * var_tauex_t) + (assign6000_e6079 * var_tauex_t_dn2)) * var_evb1c4) + (assign6000_e6081 * var_evb1c4_dn2)) * assign6000_e6091) - (assign6000_e6083 * ((4.0 * var_evb1c4vdcex_dn2) / (2.0 * assign6000_e6090)))) / (assign6000_e6091 * assign6000_e6091)), ((((((((2.0 * var_ibx_t_dn3) * var_tauex_t) + (assign6000_e6079 * var_tauex_t_dn3)) * var_evb1c4) + (assign6000_e6081 * var_evb1c4_dn3)) * assign6000_e6091) - (assign6000_e6083 * ((4.0 * var_evb1c4vdcex_dn3) / (2.0 * assign6000_e6090)))) / (assign6000_e6091 * assign6000_e6091)), ((((((((2.0 * var_ibx_t_dn4) * var_tauex_t) + (assign6000_e6079 * var_tauex_t_dn4)) * var_evb1c4) + (assign6000_e6081 * var_evb1c4_dn4)) * assign6000_e6091) - (assign6000_e6083 * ((4.0 * var_evb1c4vdcex_dn4) / (2.0 * assign6000_e6090)))) / (assign6000_e6091 * assign6000_e6091)), ((((((((2.0 * var_ibx_t_dn5) * var_tauex_t) + (assign6000_e6079 * var_tauex_t_dn5)) * var_evb1c4) + (assign6000_e6081 * var_evb1c4_dn5)) * assign6000_e6091) - (assign6000_e6083 * ((4.0 * var_evb1c4vdcex_dn5) / (2.0 * assign6000_e6090)))) / (assign6000_e6091 * assign6000_e6091)), ((((((((2.0 * var_ibx_t_dn6) * var_tauex_t) + (assign6000_e6079 * var_tauex_t_dn6)) * var_evb1c4) + (assign6000_e6081 * var_evb1c4_dn6)) * assign6000_e6091) - (assign6000_e6083 * ((4.0 * var_evb1c4vdcex_dn6) / (2.0 * assign6000_e6090)))) / (assign6000_e6091 * assign6000_e6091)), ((((((((2.0 * var_ibx_t_dn7) * var_tauex_t) + (assign6000_e6079 * var_tauex_t_dn7)) * var_evb1c4) + (assign6000_e6081 * var_evb1c4_dn7)) * assign6000_e6091) - (assign6000_e6083 * ((4.0 * var_evb1c4vdcex_dn7) / (2.0 * assign6000_e6090)))) / (assign6000_e6091 * assign6000_e6091)), ((((((((2.0 * var_ibx_t_dn8) * var_tauex_t) + (assign6000_e6079 * var_tauex_t_dn8)) * var_evb1c4) + (assign6000_e6081 * var_evb1c4_dn8)) * assign6000_e6091) - (assign6000_e6083 * ((4.0 * var_evb1c4vdcex_dn8) / (2.0 * assign6000_e6090)))) / (assign6000_e6091 * assign6000_e6091)), ((((((((2.0 * var_ibx_t_dn9) * var_tauex_t) + (assign6000_e6079 * var_tauex_t_dn9)) * var_evb1c4) + (assign6000_e6081 * var_evb1c4_dn9)) * assign6000_e6091) - (assign6000_e6083 * ((4.0 * var_evb1c4vdcex_dn9) / (2.0 * assign6000_e6090)))) / (assign6000_e6091 * assign6000_e6091)), ((((((((2.0 * var_ibx_t_dn10) * var_tauex_t) + (assign6000_e6079 * var_tauex_t_dn10)) * var_evb1c4) + (assign6000_e6081 * var_evb1c4_dn10)) * assign6000_e6091) - (assign6000_e6083 * ((4.0 * var_evb1c4vdcex_dn10) / (2.0 * assign6000_e6090)))) / (assign6000_e6091 * assign6000_e6091)), ((((((((2.0 * var_ibx_t_db0) * var_tauex_t) + (assign6000_e6079 * var_tauex_t_db0)) * var_evb1c4) + (assign6000_e6081 * var_evb1c4_db0)) * assign6000_e6091) - (assign6000_e6083 * ((4.0 * var_evb1c4vdcex_db0) / (2.0 * assign6000_e6090)))) / (assign6000_e6091 * assign6000_e6091)), ((((((((2.0 * var_ibx_t_db1) * var_tauex_t) + (assign6000_e6079 * var_tauex_t_db1)) * var_evb1c4) + (assign6000_e6081 * var_evb1c4_db1)) * assign6000_e6091) - (assign6000_e6083 * ((4.0 * var_evb1c4vdcex_db1) / (2.0 * assign6000_e6090)))) / (assign6000_e6091 * assign6000_e6091)),)
    } else {
        (var_qex, var_qex_dn0, var_qex_dn1, var_qex_dn2, var_qex_dn3, var_qex_dn4, var_qex_dn5, var_qex_dn6, var_qex_dn7, var_qex_dn8, var_qex_dn9, var_qex_dn10, var_qex_db0, var_qex_db1,)
    }
};
        var_qex = assign6000_e6094;
        var_qex_dn0 = assign6000_e6094_d_n0;
        var_qex_dn1 = assign6000_e6094_d_n1;
        var_qex_dn2 = assign6000_e6094_d_n2;
        var_qex_dn3 = assign6000_e6094_d_n3;
        var_qex_dn4 = assign6000_e6094_d_n4;
        var_qex_dn5 = assign6000_e6094_d_n5;
        var_qex_dn6 = assign6000_e6094_d_n6;
        var_qex_dn7 = assign6000_e6094_d_n7;
        var_qex_dn8 = assign6000_e6094_d_n8;
        var_qex_dn9 = assign6000_e6094_d_n9;
        var_qex_dn10 = assign6000_e6094_d_n10;
        var_qex_db0 = assign6000_e6094_d_b0;
        var_qex_db1 = assign6000_e6094_d_b1;
        var_qex_rv = 0.0;
        var_qex_rdn0 = 0.0;
        var_qex_rdn1 = 0.0;
        var_qex_rdn2 = 0.0;
        var_qex_rdn3 = 0.0;
        var_qex_rdn4 = 0.0;
        var_qex_rdn5 = 0.0;
        var_qex_rdn6 = 0.0;
        var_qex_rdn7 = 0.0;
        var_qex_rdn8 = 0.0;
        var_qex_rdn9 = 0.0;
        var_qex_rdn10 = 0.0;
        var_qex_rdb0 = 0.0;
        var_qex_rdb1 = 0.0;

        let assign6010_e6105: f64 = if (((p.p5 == 1.0) || (p.p5 == 3.0)) && (p.p32 > 0.0)) { 1.0 } else { 0.0 };
        var_guard109 = assign6010_e6105;
        var_guard109_dn0 = 0.0;
        var_guard109_dn1 = 0.0;
        var_guard109_dn2 = 0.0;
        var_guard109_dn3 = 0.0;
        var_guard109_dn4 = 0.0;
        var_guard109_dn5 = 0.0;
        var_guard109_dn6 = 0.0;
        var_guard109_dn7 = 0.0;
        var_guard109_dn8 = 0.0;
        var_guard109_dn9 = 0.0;
        var_guard109_dn10 = 0.0;
        var_guard109_db0 = 0.0;
        var_guard109_db1 = 0.0;
        var_guard109_rv = 0.0;
        var_guard109_rdn0 = 0.0;
        var_guard109_rdn1 = 0.0;
        var_guard109_rdn2 = 0.0;
        var_guard109_rdn3 = 0.0;
        var_guard109_rdn4 = 0.0;
        var_guard109_rdn5 = 0.0;
        var_guard109_rdn6 = 0.0;
        var_guard109_rdn7 = 0.0;
        var_guard109_rdn8 = 0.0;
        var_guard109_rdn9 = 0.0;
        var_guard109_rdn10 = 0.0;
        var_guard109_rdb0 = 0.0;
        var_guard109_rdb1 = 0.0;

        let (assign6020_e6111, assign6020_e6111_d_n0, assign6020_e6111_d_n1, assign6020_e6111_d_n2, assign6020_e6111_d_n3, assign6020_e6111_d_n4, assign6020_e6111_d_n5, assign6020_e6111_d_n6, assign6020_e6111_d_n7, assign6020_e6111_d_n8, assign6020_e6111_d_n9, assign6020_e6111_d_n10, assign6020_e6111_d_b0, assign6020_e6111_d_b1,) = {
    if (var_guard109 != 0.0) {
        let assign6020_e6109: f64 = (var_qex * var_xext1);
        (assign6020_e6109, ((var_qex_dn0 * var_xext1) + (var_qex * var_xext1_dn0)), ((var_qex_dn1 * var_xext1) + (var_qex * var_xext1_dn1)), ((var_qex_dn2 * var_xext1) + (var_qex * var_xext1_dn2)), ((var_qex_dn3 * var_xext1) + (var_qex * var_xext1_dn3)), ((var_qex_dn4 * var_xext1) + (var_qex * var_xext1_dn4)), ((var_qex_dn5 * var_xext1) + (var_qex * var_xext1_dn5)), ((var_qex_dn6 * var_xext1) + (var_qex * var_xext1_dn6)), ((var_qex_dn7 * var_xext1) + (var_qex * var_xext1_dn7)), ((var_qex_dn8 * var_xext1) + (var_qex * var_xext1_dn8)), ((var_qex_dn9 * var_xext1) + (var_qex * var_xext1_dn9)), ((var_qex_dn10 * var_xext1) + (var_qex * var_xext1_dn10)), ((var_qex_db0 * var_xext1) + (var_qex * var_xext1_db0)), ((var_qex_db1 * var_xext1) + (var_qex * var_xext1_db1)),)
    } else {
        (var_qex, var_qex_dn0, var_qex_dn1, var_qex_dn2, var_qex_dn3, var_qex_dn4, var_qex_dn5, var_qex_dn6, var_qex_dn7, var_qex_dn8, var_qex_dn9, var_qex_dn10, var_qex_db0, var_qex_db1,)
    }
};
        var_qex = assign6020_e6111;
        var_qex_dn0 = assign6020_e6111_d_n0;
        var_qex_dn1 = assign6020_e6111_d_n1;
        var_qex_dn2 = assign6020_e6111_d_n2;
        var_qex_dn3 = assign6020_e6111_d_n3;
        var_qex_dn4 = assign6020_e6111_d_n4;
        var_qex_dn5 = assign6020_e6111_d_n5;
        var_qex_dn6 = assign6020_e6111_d_n6;
        var_qex_dn7 = assign6020_e6111_d_n7;
        var_qex_dn8 = assign6020_e6111_d_n8;
        var_qex_dn9 = assign6020_e6111_d_n9;
        var_qex_dn10 = assign6020_e6111_d_n10;
        var_qex_db0 = assign6020_e6111_d_b0;
        var_qex_db1 = assign6020_e6111_d_b1;
        var_qex_rv = 0.0;
        var_qex_rdn0 = 0.0;
        var_qex_rdn1 = 0.0;
        var_qex_rdn2 = 0.0;
        var_qex_rdn3 = 0.0;
        var_qex_rdn4 = 0.0;
        var_qex_rdn5 = 0.0;
        var_qex_rdn6 = 0.0;
        var_qex_rdn7 = 0.0;
        var_qex_rdn8 = 0.0;
        var_qex_rdn9 = 0.0;
        var_qex_rdn10 = 0.0;
        var_qex_rdb0 = 0.0;
        var_qex_rdb1 = 0.0;

        let assign6030_e6114: f64 = if p.p78 == 0.0 { 1.0 } else { 0.0 };
        var_guard110 = assign6030_e6114;
        var_guard110_dn0 = 0.0;
        var_guard110_dn1 = 0.0;
        var_guard110_dn2 = 0.0;
        var_guard110_dn3 = 0.0;
        var_guard110_dn4 = 0.0;
        var_guard110_dn5 = 0.0;
        var_guard110_dn6 = 0.0;
        var_guard110_dn7 = 0.0;
        var_guard110_dn8 = 0.0;
        var_guard110_dn9 = 0.0;
        var_guard110_dn10 = 0.0;
        var_guard110_db0 = 0.0;
        var_guard110_db1 = 0.0;
        var_guard110_rv = 0.0;
        var_guard110_rdn0 = 0.0;
        var_guard110_rdn1 = 0.0;
        var_guard110_rdn2 = 0.0;
        var_guard110_rdn3 = 0.0;
        var_guard110_rdn4 = 0.0;
        var_guard110_rdn5 = 0.0;
        var_guard110_rdn6 = 0.0;
        var_guard110_rdn7 = 0.0;
        var_guard110_rdn8 = 0.0;
        var_guard110_rdn9 = 0.0;
        var_guard110_rdn10 = 0.0;
        var_guard110_rdb0 = 0.0;
        var_guard110_rdb1 = 0.0;

        let (assign6040_e6122, assign6040_e6122_d_n0, assign6040_e6122_d_n1, assign6040_e6122_d_n2, assign6040_e6122_d_n3, assign6040_e6122_d_n4, assign6040_e6122_d_n5, assign6040_e6122_d_n6, assign6040_e6122_d_n7, assign6040_e6122_d_n8, assign6040_e6122_d_n9, assign6040_e6122_d_n10, assign6040_e6122_d_b0, assign6040_e6122_d_b1,) = {
    if ((var_guard109 != 0.0) && (var_guard110 != 0.0)) {
        let assign6040_e6120: f64 = (var_if0 * var_evbc3);
        (assign6040_e6120, ((var_if0_dn0 * var_evbc3) + (var_if0 * var_evbc3_dn0)), ((var_if0_dn1 * var_evbc3) + (var_if0 * var_evbc3_dn1)), ((var_if0_dn2 * var_evbc3) + (var_if0 * var_evbc3_dn2)), ((var_if0_dn3 * var_evbc3) + (var_if0 * var_evbc3_dn3)), ((var_if0_dn4 * var_evbc3) + (var_if0 * var_evbc3_dn4)), ((var_if0_dn5 * var_evbc3) + (var_if0 * var_evbc3_dn5)), ((var_if0_dn6 * var_evbc3) + (var_if0 * var_evbc3_dn6)), ((var_if0_dn7 * var_evbc3) + (var_if0 * var_evbc3_dn7)), ((var_if0_dn8 * var_evbc3) + (var_if0 * var_evbc3_dn8)), ((var_if0_dn9 * var_evbc3) + (var_if0 * var_evbc3_dn9)), ((var_if0_dn10 * var_evbc3) + (var_if0 * var_evbc3_dn10)), ((var_if0_db0 * var_evbc3) + (var_if0 * var_evbc3_db0)), ((var_if0_db1 * var_evbc3) + (var_if0 * var_evbc3_db1)),)
    } else {
        (var_xg1, var_xg1_dn0, var_xg1_dn1, var_xg1_dn2, var_xg1_dn3, var_xg1_dn4, var_xg1_dn5, var_xg1_dn6, var_xg1_dn7, var_xg1_dn8, var_xg1_dn9, var_xg1_dn10, var_xg1_db0, var_xg1_db1,)
    }
};
        var_xg1 = assign6040_e6122;
        var_xg1_dn0 = assign6040_e6122_d_n0;
        var_xg1_dn1 = assign6040_e6122_d_n1;
        var_xg1_dn2 = assign6040_e6122_d_n2;
        var_xg1_dn3 = assign6040_e6122_d_n3;
        var_xg1_dn4 = assign6040_e6122_d_n4;
        var_xg1_dn5 = assign6040_e6122_d_n5;
        var_xg1_dn6 = assign6040_e6122_d_n6;
        var_xg1_dn7 = assign6040_e6122_d_n7;
        var_xg1_dn8 = assign6040_e6122_d_n8;
        var_xg1_dn9 = assign6040_e6122_d_n9;
        var_xg1_dn10 = assign6040_e6122_d_n10;
        var_xg1_db0 = assign6040_e6122_d_b0;
        var_xg1_db1 = assign6040_e6122_d_b1;
        var_xg1_rv = 0.0;
        var_xg1_rdn0 = 0.0;
        var_xg1_rdn1 = 0.0;
        var_xg1_rdn2 = 0.0;
        var_xg1_rdn3 = 0.0;
        var_xg1_rdn4 = 0.0;
        var_xg1_rdn5 = 0.0;
        var_xg1_rdn6 = 0.0;
        var_xg1_rdn7 = 0.0;
        var_xg1_rdn8 = 0.0;
        var_xg1_rdn9 = 0.0;
        var_xg1_rdn10 = 0.0;
        var_xg1_rdb0 = 0.0;
        var_xg1_rdb1 = 0.0;

        let (assign6050_e6137, assign6050_e6137_d_n0, assign6050_e6137_d_n1, assign6050_e6137_d_n2, assign6050_e6137_d_n3, assign6050_e6137_d_n4, assign6050_e6137_d_n5, assign6050_e6137_d_n6, assign6050_e6137_d_n7, assign6050_e6137_d_n8, assign6050_e6137_d_n9, assign6050_e6137_d_n10, assign6050_e6137_d_b0, assign6050_e6137_d_b1,) = {
    if ((var_guard109 != 0.0) && (var_guard110 != 0.0)) {
        let assign6050_e6128: f64 = (var_xg1 - var_if0);
        let assign6050_e6132: f64 = (1.0 + var_xg1);
        let assign6050_e6133: f64 = (assign6050_e6132).sqrt();
        let assign6050_e6134: f64 = (1.0 + assign6050_e6133);
        let assign6050_e6135: f64 = (assign6050_e6128 / assign6050_e6134);
        (assign6050_e6135, ((((var_xg1_dn0 - var_if0_dn0) * assign6050_e6134) - (assign6050_e6128 * (var_xg1_dn0 / (2.0 * assign6050_e6133)))) / (assign6050_e6134 * assign6050_e6134)), ((((var_xg1_dn1 - var_if0_dn1) * assign6050_e6134) - (assign6050_e6128 * (var_xg1_dn1 / (2.0 * assign6050_e6133)))) / (assign6050_e6134 * assign6050_e6134)), ((((var_xg1_dn2 - var_if0_dn2) * assign6050_e6134) - (assign6050_e6128 * (var_xg1_dn2 / (2.0 * assign6050_e6133)))) / (assign6050_e6134 * assign6050_e6134)), ((((var_xg1_dn3 - var_if0_dn3) * assign6050_e6134) - (assign6050_e6128 * (var_xg1_dn3 / (2.0 * assign6050_e6133)))) / (assign6050_e6134 * assign6050_e6134)), ((((var_xg1_dn4 - var_if0_dn4) * assign6050_e6134) - (assign6050_e6128 * (var_xg1_dn4 / (2.0 * assign6050_e6133)))) / (assign6050_e6134 * assign6050_e6134)), ((((var_xg1_dn5 - var_if0_dn5) * assign6050_e6134) - (assign6050_e6128 * (var_xg1_dn5 / (2.0 * assign6050_e6133)))) / (assign6050_e6134 * assign6050_e6134)), ((((var_xg1_dn6 - var_if0_dn6) * assign6050_e6134) - (assign6050_e6128 * (var_xg1_dn6 / (2.0 * assign6050_e6133)))) / (assign6050_e6134 * assign6050_e6134)), ((((var_xg1_dn7 - var_if0_dn7) * assign6050_e6134) - (assign6050_e6128 * (var_xg1_dn7 / (2.0 * assign6050_e6133)))) / (assign6050_e6134 * assign6050_e6134)), ((((var_xg1_dn8 - var_if0_dn8) * assign6050_e6134) - (assign6050_e6128 * (var_xg1_dn8 / (2.0 * assign6050_e6133)))) / (assign6050_e6134 * assign6050_e6134)), ((((var_xg1_dn9 - var_if0_dn9) * assign6050_e6134) - (assign6050_e6128 * (var_xg1_dn9 / (2.0 * assign6050_e6133)))) / (assign6050_e6134 * assign6050_e6134)), ((((var_xg1_dn10 - var_if0_dn10) * assign6050_e6134) - (assign6050_e6128 * (var_xg1_dn10 / (2.0 * assign6050_e6133)))) / (assign6050_e6134 * assign6050_e6134)), ((((var_xg1_db0 - var_if0_db0) * assign6050_e6134) - (assign6050_e6128 * (var_xg1_db0 / (2.0 * assign6050_e6133)))) / (assign6050_e6134 * assign6050_e6134)), ((((var_xg1_db1 - var_if0_db1) * assign6050_e6134) - (assign6050_e6128 * (var_xg1_db1 / (2.0 * assign6050_e6133)))) / (assign6050_e6134 * assign6050_e6134)),)
    } else {
        (var_xnbex, var_xnbex_dn0, var_xnbex_dn1, var_xnbex_dn2, var_xnbex_dn3, var_xnbex_dn4, var_xnbex_dn5, var_xnbex_dn6, var_xnbex_dn7, var_xnbex_dn8, var_xnbex_dn9, var_xnbex_dn10, var_xnbex_db0, var_xnbex_db1,)
    }
};
        var_xnbex = assign6050_e6137;
        var_xnbex_dn0 = assign6050_e6137_d_n0;
        var_xnbex_dn1 = assign6050_e6137_d_n1;
        var_xnbex_dn2 = assign6050_e6137_d_n2;
        var_xnbex_dn3 = assign6050_e6137_d_n3;
        var_xnbex_dn4 = assign6050_e6137_d_n4;
        var_xnbex_dn5 = assign6050_e6137_d_n5;
        var_xnbex_dn6 = assign6050_e6137_d_n6;
        var_xnbex_dn7 = assign6050_e6137_d_n7;
        var_xnbex_dn8 = assign6050_e6137_d_n8;
        var_xnbex_dn9 = assign6050_e6137_d_n9;
        var_xnbex_dn10 = assign6050_e6137_d_n10;
        var_xnbex_db0 = assign6050_e6137_d_b0;
        var_xnbex_db1 = assign6050_e6137_d_b1;
        var_xnbex_rv = 0.0;
        var_xnbex_rdn0 = 0.0;
        var_xnbex_rdn1 = 0.0;
        var_xnbex_rdn2 = 0.0;
        var_xnbex_rdn3 = 0.0;
        var_xnbex_rdn4 = 0.0;
        var_xnbex_rdn5 = 0.0;
        var_xnbex_rdn6 = 0.0;
        var_xnbex_rdn7 = 0.0;
        var_xnbex_rdn8 = 0.0;
        var_xnbex_rdn9 = 0.0;
        var_xnbex_rdn10 = 0.0;
        var_xnbex_rdb0 = 0.0;
        var_xnbex_rdb1 = 0.0;

        let (assign6060_e6145, assign6060_e6145_d_n0, assign6060_e6145_d_n1, assign6060_e6145_d_n2, assign6060_e6145_d_n3, assign6060_e6145_d_n4, assign6060_e6145_d_n5, assign6060_e6145_d_n6, assign6060_e6145_d_n7, assign6060_e6145_d_n8, assign6060_e6145_d_n9, assign6060_e6145_d_n10, assign6060_e6145_d_b0, assign6060_e6145_d_b1,) = {
    if ((var_guard109 != 0.0) && (var_guard110 != 0.0)) {
        let assign6060_e6143: f64 = (4.0 * var_evbc3vdc);
        (assign6060_e6143, (4.0 * var_evbc3vdc_dn0), (4.0 * var_evbc3vdc_dn1), (4.0 * var_evbc3vdc_dn2), (4.0 * var_evbc3vdc_dn3), (4.0 * var_evbc3vdc_dn4), (4.0 * var_evbc3vdc_dn5), (4.0 * var_evbc3vdc_dn6), (4.0 * var_evbc3vdc_dn7), (4.0 * var_evbc3vdc_dn8), (4.0 * var_evbc3vdc_dn9), (4.0 * var_evbc3vdc_dn10), (4.0 * var_evbc3vdc_db0), (4.0 * var_evbc3vdc_db1),)
    } else {
        (var_xg2, var_xg2_dn0, var_xg2_dn1, var_xg2_dn2, var_xg2_dn3, var_xg2_dn4, var_xg2_dn5, var_xg2_dn6, var_xg2_dn7, var_xg2_dn8, var_xg2_dn9, var_xg2_dn10, var_xg2_db0, var_xg2_db1,)
    }
};
        var_xg2 = assign6060_e6145;
        var_xg2_dn0 = assign6060_e6145_d_n0;
        var_xg2_dn1 = assign6060_e6145_d_n1;
        var_xg2_dn2 = assign6060_e6145_d_n2;
        var_xg2_dn3 = assign6060_e6145_d_n3;
        var_xg2_dn4 = assign6060_e6145_d_n4;
        var_xg2_dn5 = assign6060_e6145_d_n5;
        var_xg2_dn6 = assign6060_e6145_d_n6;
        var_xg2_dn7 = assign6060_e6145_d_n7;
        var_xg2_dn8 = assign6060_e6145_d_n8;
        var_xg2_dn9 = assign6060_e6145_d_n9;
        var_xg2_dn10 = assign6060_e6145_d_n10;
        var_xg2_db0 = assign6060_e6145_d_b0;
        var_xg2_db1 = assign6060_e6145_d_b1;
        var_xg2_rv = 0.0;
        var_xg2_rdn0 = 0.0;
        var_xg2_rdn1 = 0.0;
        var_xg2_rdn2 = 0.0;
        var_xg2_rdn3 = 0.0;
        var_xg2_rdn4 = 0.0;
        var_xg2_rdn5 = 0.0;
        var_xg2_rdn6 = 0.0;
        var_xg2_rdn7 = 0.0;
        var_xg2_rdn8 = 0.0;
        var_xg2_rdn9 = 0.0;
        var_xg2_rdn10 = 0.0;
        var_xg2_rdb0 = 0.0;
        var_xg2_rdb1 = 0.0;

        let (assign6070_e6158, assign6070_e6158_d_n0, assign6070_e6158_d_n1, assign6070_e6158_d_n2, assign6070_e6158_d_n3, assign6070_e6158_d_n4, assign6070_e6158_d_n5, assign6070_e6158_d_n6, assign6070_e6158_d_n7, assign6070_e6158_d_n8, assign6070_e6158_d_n9, assign6070_e6158_d_n10, assign6070_e6158_d_b0, assign6070_e6158_d_b1,) = {
    if ((var_guard109 != 0.0) && (var_guard110 != 0.0)) {
        let assign6070_e6153: f64 = (1.0 + var_xg2);
        let assign6070_e6154: f64 = (assign6070_e6153).sqrt();
        let assign6070_e6155: f64 = (1.0 + assign6070_e6154);
        let assign6070_e6156: f64 = (var_xg2 / assign6070_e6155);
        (assign6070_e6156, (((var_xg2_dn0 * assign6070_e6155) - (var_xg2 * (var_xg2_dn0 / (2.0 * assign6070_e6154)))) / (assign6070_e6155 * assign6070_e6155)), (((var_xg2_dn1 * assign6070_e6155) - (var_xg2 * (var_xg2_dn1 / (2.0 * assign6070_e6154)))) / (assign6070_e6155 * assign6070_e6155)), (((var_xg2_dn2 * assign6070_e6155) - (var_xg2 * (var_xg2_dn2 / (2.0 * assign6070_e6154)))) / (assign6070_e6155 * assign6070_e6155)), (((var_xg2_dn3 * assign6070_e6155) - (var_xg2 * (var_xg2_dn3 / (2.0 * assign6070_e6154)))) / (assign6070_e6155 * assign6070_e6155)), (((var_xg2_dn4 * assign6070_e6155) - (var_xg2 * (var_xg2_dn4 / (2.0 * assign6070_e6154)))) / (assign6070_e6155 * assign6070_e6155)), (((var_xg2_dn5 * assign6070_e6155) - (var_xg2 * (var_xg2_dn5 / (2.0 * assign6070_e6154)))) / (assign6070_e6155 * assign6070_e6155)), (((var_xg2_dn6 * assign6070_e6155) - (var_xg2 * (var_xg2_dn6 / (2.0 * assign6070_e6154)))) / (assign6070_e6155 * assign6070_e6155)), (((var_xg2_dn7 * assign6070_e6155) - (var_xg2 * (var_xg2_dn7 / (2.0 * assign6070_e6154)))) / (assign6070_e6155 * assign6070_e6155)), (((var_xg2_dn8 * assign6070_e6155) - (var_xg2 * (var_xg2_dn8 / (2.0 * assign6070_e6154)))) / (assign6070_e6155 * assign6070_e6155)), (((var_xg2_dn9 * assign6070_e6155) - (var_xg2 * (var_xg2_dn9 / (2.0 * assign6070_e6154)))) / (assign6070_e6155 * assign6070_e6155)), (((var_xg2_dn10 * assign6070_e6155) - (var_xg2 * (var_xg2_dn10 / (2.0 * assign6070_e6154)))) / (assign6070_e6155 * assign6070_e6155)), (((var_xg2_db0 * assign6070_e6155) - (var_xg2 * (var_xg2_db0 / (2.0 * assign6070_e6154)))) / (assign6070_e6155 * assign6070_e6155)), (((var_xg2_db1 * assign6070_e6155) - (var_xg2 * (var_xg2_db1 / (2.0 * assign6070_e6154)))) / (assign6070_e6155 * assign6070_e6155)),)
    } else {
        (var_xpwex, var_xpwex_dn0, var_xpwex_dn1, var_xpwex_dn2, var_xpwex_dn3, var_xpwex_dn4, var_xpwex_dn5, var_xpwex_dn6, var_xpwex_dn7, var_xpwex_dn8, var_xpwex_dn9, var_xpwex_dn10, var_xpwex_db0, var_xpwex_db1,)
    }
};
        var_xpwex = assign6070_e6158;
        var_xpwex_dn0 = assign6070_e6158_d_n0;
        var_xpwex_dn1 = assign6070_e6158_d_n1;
        var_xpwex_dn2 = assign6070_e6158_d_n2;
        var_xpwex_dn3 = assign6070_e6158_d_n3;
        var_xpwex_dn4 = assign6070_e6158_d_n4;
        var_xpwex_dn5 = assign6070_e6158_d_n5;
        var_xpwex_dn6 = assign6070_e6158_d_n6;
        var_xpwex_dn7 = assign6070_e6158_d_n7;
        var_xpwex_dn8 = assign6070_e6158_d_n8;
        var_xpwex_dn9 = assign6070_e6158_d_n9;
        var_xpwex_dn10 = assign6070_e6158_d_n10;
        var_xpwex_db0 = assign6070_e6158_d_b0;
        var_xpwex_db1 = assign6070_e6158_d_b1;
        var_xpwex_rv = 0.0;
        var_xpwex_rdn0 = 0.0;
        var_xpwex_rdn1 = 0.0;
        var_xpwex_rdn2 = 0.0;
        var_xpwex_rdn3 = 0.0;
        var_xpwex_rdn4 = 0.0;
        var_xpwex_rdn5 = 0.0;
        var_xpwex_rdn6 = 0.0;
        var_xpwex_rdn7 = 0.0;
        var_xpwex_rdn8 = 0.0;
        var_xpwex_rdn9 = 0.0;
        var_xpwex_rdn10 = 0.0;
        var_xpwex_rdb0 = 0.0;
        var_xpwex_rdb1 = 0.0;

        let (assign6080_e6180, assign6080_e6180_d_n0, assign6080_e6180_d_n1, assign6080_e6180_d_n2, assign6080_e6180_d_n3, assign6080_e6180_d_n4, assign6080_e6180_d_n5, assign6080_e6180_d_n6, assign6080_e6180_d_n7, assign6080_e6180_d_n8, assign6080_e6180_d_n9, assign6080_e6180_d_n10, assign6080_e6180_d_b0, assign6080_e6180_d_b1,) = {
    if ((var_guard109 != 0.0) && (var_guard110 != 0.0)) {
        let assign6080_e6164: f64 = (0.5 * p.p32);
        let assign6080_e6166: f64 = (assign6080_e6164 * var_taur_t);
        let assign6080_e6169: f64 = (var_qb0 * var_xnbex);
        let assign6080_e6172: f64 = (var_qepi0 * var_xpwex);
        let assign6080_e6173: f64 = (assign6080_e6169 + assign6080_e6172);
        let assign6080_e6174: f64 = (assign6080_e6166 * assign6080_e6173);
        let assign6080_e6177: f64 = (var_taub_t + var_tepi_t);
        let assign6080_e6178: f64 = (assign6080_e6174 / assign6080_e6177);
        (assign6080_e6178, ((((((assign6080_e6164 * var_taur_t_dn0) * assign6080_e6173) + (assign6080_e6166 * (((var_qb0_dn0 * var_xnbex) + (var_qb0 * var_xnbex_dn0)) + ((var_qepi0_dn0 * var_xpwex) + (var_qepi0 * var_xpwex_dn0))))) * assign6080_e6177) - (assign6080_e6174 * (var_taub_t_dn0 + var_tepi_t_dn0))) / (assign6080_e6177 * assign6080_e6177)), ((((((assign6080_e6164 * var_taur_t_dn1) * assign6080_e6173) + (assign6080_e6166 * (((var_qb0_dn1 * var_xnbex) + (var_qb0 * var_xnbex_dn1)) + ((var_qepi0_dn1 * var_xpwex) + (var_qepi0 * var_xpwex_dn1))))) * assign6080_e6177) - (assign6080_e6174 * (var_taub_t_dn1 + var_tepi_t_dn1))) / (assign6080_e6177 * assign6080_e6177)), ((((((assign6080_e6164 * var_taur_t_dn2) * assign6080_e6173) + (assign6080_e6166 * (((var_qb0_dn2 * var_xnbex) + (var_qb0 * var_xnbex_dn2)) + ((var_qepi0_dn2 * var_xpwex) + (var_qepi0 * var_xpwex_dn2))))) * assign6080_e6177) - (assign6080_e6174 * (var_taub_t_dn2 + var_tepi_t_dn2))) / (assign6080_e6177 * assign6080_e6177)), ((((((assign6080_e6164 * var_taur_t_dn3) * assign6080_e6173) + (assign6080_e6166 * (((var_qb0_dn3 * var_xnbex) + (var_qb0 * var_xnbex_dn3)) + ((var_qepi0_dn3 * var_xpwex) + (var_qepi0 * var_xpwex_dn3))))) * assign6080_e6177) - (assign6080_e6174 * (var_taub_t_dn3 + var_tepi_t_dn3))) / (assign6080_e6177 * assign6080_e6177)), ((((((assign6080_e6164 * var_taur_t_dn4) * assign6080_e6173) + (assign6080_e6166 * (((var_qb0_dn4 * var_xnbex) + (var_qb0 * var_xnbex_dn4)) + ((var_qepi0_dn4 * var_xpwex) + (var_qepi0 * var_xpwex_dn4))))) * assign6080_e6177) - (assign6080_e6174 * (var_taub_t_dn4 + var_tepi_t_dn4))) / (assign6080_e6177 * assign6080_e6177)), ((((((assign6080_e6164 * var_taur_t_dn5) * assign6080_e6173) + (assign6080_e6166 * (((var_qb0_dn5 * var_xnbex) + (var_qb0 * var_xnbex_dn5)) + ((var_qepi0_dn5 * var_xpwex) + (var_qepi0 * var_xpwex_dn5))))) * assign6080_e6177) - (assign6080_e6174 * (var_taub_t_dn5 + var_tepi_t_dn5))) / (assign6080_e6177 * assign6080_e6177)), ((((((assign6080_e6164 * var_taur_t_dn6) * assign6080_e6173) + (assign6080_e6166 * (((var_qb0_dn6 * var_xnbex) + (var_qb0 * var_xnbex_dn6)) + ((var_qepi0_dn6 * var_xpwex) + (var_qepi0 * var_xpwex_dn6))))) * assign6080_e6177) - (assign6080_e6174 * (var_taub_t_dn6 + var_tepi_t_dn6))) / (assign6080_e6177 * assign6080_e6177)), ((((((assign6080_e6164 * var_taur_t_dn7) * assign6080_e6173) + (assign6080_e6166 * (((var_qb0_dn7 * var_xnbex) + (var_qb0 * var_xnbex_dn7)) + ((var_qepi0_dn7 * var_xpwex) + (var_qepi0 * var_xpwex_dn7))))) * assign6080_e6177) - (assign6080_e6174 * (var_taub_t_dn7 + var_tepi_t_dn7))) / (assign6080_e6177 * assign6080_e6177)), ((((((assign6080_e6164 * var_taur_t_dn8) * assign6080_e6173) + (assign6080_e6166 * (((var_qb0_dn8 * var_xnbex) + (var_qb0 * var_xnbex_dn8)) + ((var_qepi0_dn8 * var_xpwex) + (var_qepi0 * var_xpwex_dn8))))) * assign6080_e6177) - (assign6080_e6174 * (var_taub_t_dn8 + var_tepi_t_dn8))) / (assign6080_e6177 * assign6080_e6177)), ((((((assign6080_e6164 * var_taur_t_dn9) * assign6080_e6173) + (assign6080_e6166 * (((var_qb0_dn9 * var_xnbex) + (var_qb0 * var_xnbex_dn9)) + ((var_qepi0_dn9 * var_xpwex) + (var_qepi0 * var_xpwex_dn9))))) * assign6080_e6177) - (assign6080_e6174 * (var_taub_t_dn9 + var_tepi_t_dn9))) / (assign6080_e6177 * assign6080_e6177)), ((((((assign6080_e6164 * var_taur_t_dn10) * assign6080_e6173) + (assign6080_e6166 * (((var_qb0_dn10 * var_xnbex) + (var_qb0 * var_xnbex_dn10)) + ((var_qepi0_dn10 * var_xpwex) + (var_qepi0 * var_xpwex_dn10))))) * assign6080_e6177) - (assign6080_e6174 * (var_taub_t_dn10 + var_tepi_t_dn10))) / (assign6080_e6177 * assign6080_e6177)), ((((((assign6080_e6164 * var_taur_t_db0) * assign6080_e6173) + (assign6080_e6166 * (((var_qb0_db0 * var_xnbex) + (var_qb0 * var_xnbex_db0)) + ((var_qepi0_db0 * var_xpwex) + (var_qepi0 * var_xpwex_db0))))) * assign6080_e6177) - (assign6080_e6174 * (var_taub_t_db0 + var_tepi_t_db0))) / (assign6080_e6177 * assign6080_e6177)), ((((((assign6080_e6164 * var_taur_t_db1) * assign6080_e6173) + (assign6080_e6166 * (((var_qb0_db1 * var_xnbex) + (var_qb0 * var_xnbex_db1)) + ((var_qepi0_db1 * var_xpwex) + (var_qepi0 * var_xpwex_db1))))) * assign6080_e6177) - (assign6080_e6174 * (var_taub_t_db1 + var_tepi_t_db1))) / (assign6080_e6177 * assign6080_e6177)),)
    } else {
        (var_xqmex, var_xqmex_dn0, var_xqmex_dn1, var_xqmex_dn2, var_xqmex_dn3, var_xqmex_dn4, var_xqmex_dn5, var_xqmex_dn6, var_xqmex_dn7, var_xqmex_dn8, var_xqmex_dn9, var_xqmex_dn10, var_xqmex_db0, var_xqmex_db1,)
    }
};
        var_xqmex = assign6080_e6180;
        var_xqmex_dn0 = assign6080_e6180_d_n0;
        var_xqmex_dn1 = assign6080_e6180_d_n1;
        var_xqmex_dn2 = assign6080_e6180_d_n2;
        var_xqmex_dn3 = assign6080_e6180_d_n3;
        var_xqmex_dn4 = assign6080_e6180_d_n4;
        var_xqmex_dn5 = assign6080_e6180_d_n5;
        var_xqmex_dn6 = assign6080_e6180_d_n6;
        var_xqmex_dn7 = assign6080_e6180_d_n7;
        var_xqmex_dn8 = assign6080_e6180_d_n8;
        var_xqmex_dn9 = assign6080_e6180_d_n9;
        var_xqmex_dn10 = assign6080_e6180_d_n10;
        var_xqmex_db0 = assign6080_e6180_d_b0;
        var_xqmex_db1 = assign6080_e6180_d_b1;
        var_xqmex_rv = 0.0;
        var_xqmex_rdn0 = 0.0;
        var_xqmex_rdn1 = 0.0;
        var_xqmex_rdn2 = 0.0;
        var_xqmex_rdn3 = 0.0;
        var_xqmex_rdn4 = 0.0;
        var_xqmex_rdn5 = 0.0;
        var_xqmex_rdn6 = 0.0;
        var_xqmex_rdn7 = 0.0;
        var_xqmex_rdn8 = 0.0;
        var_xqmex_rdn9 = 0.0;
        var_xqmex_rdn10 = 0.0;
        var_xqmex_rdb0 = 0.0;
        var_xqmex_rdb1 = 0.0;

        *var_evb1c4vdcex_slot = var_evb1c4vdcex;
        *var_evb1c4vdcex_db0_slot = var_evb1c4vdcex_db0;
        *var_evb1c4vdcex_db1_slot = var_evb1c4vdcex_db1;
        *var_evb1c4vdcex_dn0_slot = var_evb1c4vdcex_dn0;
        *var_evb1c4vdcex_dn1_slot = var_evb1c4vdcex_dn1;
        *var_evb1c4vdcex_dn10_slot = var_evb1c4vdcex_dn10;
        *var_evb1c4vdcex_dn2_slot = var_evb1c4vdcex_dn2;
        *var_evb1c4vdcex_dn3_slot = var_evb1c4vdcex_dn3;
        *var_evb1c4vdcex_dn4_slot = var_evb1c4vdcex_dn4;
        *var_evb1c4vdcex_dn5_slot = var_evb1c4vdcex_dn5;
        *var_evb1c4vdcex_dn6_slot = var_evb1c4vdcex_dn6;
        *var_evb1c4vdcex_dn7_slot = var_evb1c4vdcex_dn7;
        *var_evb1c4vdcex_dn8_slot = var_evb1c4vdcex_dn8;
        *var_evb1c4vdcex_dn9_slot = var_evb1c4vdcex_dn9;
        *var_evb1c4vdcex_rdb0_slot = var_evb1c4vdcex_rdb0;
        *var_evb1c4vdcex_rdb1_slot = var_evb1c4vdcex_rdb1;
        *var_evb1c4vdcex_rdn0_slot = var_evb1c4vdcex_rdn0;
        *var_evb1c4vdcex_rdn1_slot = var_evb1c4vdcex_rdn1;
        *var_evb1c4vdcex_rdn10_slot = var_evb1c4vdcex_rdn10;
        *var_evb1c4vdcex_rdn2_slot = var_evb1c4vdcex_rdn2;
        *var_evb1c4vdcex_rdn3_slot = var_evb1c4vdcex_rdn3;
        *var_evb1c4vdcex_rdn4_slot = var_evb1c4vdcex_rdn4;
        *var_evb1c4vdcex_rdn5_slot = var_evb1c4vdcex_rdn5;
        *var_evb1c4vdcex_rdn6_slot = var_evb1c4vdcex_rdn6;
        *var_evb1c4vdcex_rdn7_slot = var_evb1c4vdcex_rdn7;
        *var_evb1c4vdcex_rdn8_slot = var_evb1c4vdcex_rdn8;
        *var_evb1c4vdcex_rdn9_slot = var_evb1c4vdcex_rdn9;
        *var_evb1c4vdcex_rv_slot = var_evb1c4vdcex_rv;
        *var_expl_slot = var_expl;
        *var_expl_db0_slot = var_expl_db0;
        *var_expl_db1_slot = var_expl_db1;
        *var_expl_dn0_slot = var_expl_dn0;
        *var_expl_dn1_slot = var_expl_dn1;
        *var_expl_dn10_slot = var_expl_dn10;
        *var_expl_dn2_slot = var_expl_dn2;
        *var_expl_dn3_slot = var_expl_dn3;
        *var_expl_dn4_slot = var_expl_dn4;
        *var_expl_dn5_slot = var_expl_dn5;
        *var_expl_dn6_slot = var_expl_dn6;
        *var_expl_dn7_slot = var_expl_dn7;
        *var_expl_dn8_slot = var_expl_dn8;
        *var_expl_dn9_slot = var_expl_dn9;
        *var_expl_rdb0_slot = var_expl_rdb0;
        *var_expl_rdb1_slot = var_expl_rdb1;
        *var_expl_rdn0_slot = var_expl_rdn0;
        *var_expl_rdn1_slot = var_expl_rdn1;
        *var_expl_rdn10_slot = var_expl_rdn10;
        *var_expl_rdn2_slot = var_expl_rdn2;
        *var_expl_rdn3_slot = var_expl_rdn3;
        *var_expl_rdn4_slot = var_expl_rdn4;
        *var_expl_rdn5_slot = var_expl_rdn5;
        *var_expl_rdn6_slot = var_expl_rdn6;
        *var_expl_rdn7_slot = var_expl_rdn7;
        *var_expl_rdn8_slot = var_expl_rdn8;
        *var_expl_rdn9_slot = var_expl_rdn9;
        *var_expl_rv_slot = var_expl_rv;
        *var_guard108_slot = var_guard108;
        *var_guard108_db0_slot = var_guard108_db0;
        *var_guard108_db1_slot = var_guard108_db1;
        *var_guard108_dn0_slot = var_guard108_dn0;
        *var_guard108_dn1_slot = var_guard108_dn1;
        *var_guard108_dn10_slot = var_guard108_dn10;
        *var_guard108_dn2_slot = var_guard108_dn2;
        *var_guard108_dn3_slot = var_guard108_dn3;
        *var_guard108_dn4_slot = var_guard108_dn4;
        *var_guard108_dn5_slot = var_guard108_dn5;
        *var_guard108_dn6_slot = var_guard108_dn6;
        *var_guard108_dn7_slot = var_guard108_dn7;
        *var_guard108_dn8_slot = var_guard108_dn8;
        *var_guard108_dn9_slot = var_guard108_dn9;
        *var_guard108_rdb0_slot = var_guard108_rdb0;
        *var_guard108_rdb1_slot = var_guard108_rdb1;
        *var_guard108_rdn0_slot = var_guard108_rdn0;
        *var_guard108_rdn1_slot = var_guard108_rdn1;
        *var_guard108_rdn10_slot = var_guard108_rdn10;
        *var_guard108_rdn2_slot = var_guard108_rdn2;
        *var_guard108_rdn3_slot = var_guard108_rdn3;
        *var_guard108_rdn4_slot = var_guard108_rdn4;
        *var_guard108_rdn5_slot = var_guard108_rdn5;
        *var_guard108_rdn6_slot = var_guard108_rdn6;
        *var_guard108_rdn7_slot = var_guard108_rdn7;
        *var_guard108_rdn8_slot = var_guard108_rdn8;
        *var_guard108_rdn9_slot = var_guard108_rdn9;
        *var_guard108_rv_slot = var_guard108_rv;
        *var_guard109_slot = var_guard109;
        *var_guard109_db0_slot = var_guard109_db0;
        *var_guard109_db1_slot = var_guard109_db1;
        *var_guard109_dn0_slot = var_guard109_dn0;
        *var_guard109_dn1_slot = var_guard109_dn1;
        *var_guard109_dn10_slot = var_guard109_dn10;
        *var_guard109_dn2_slot = var_guard109_dn2;
        *var_guard109_dn3_slot = var_guard109_dn3;
        *var_guard109_dn4_slot = var_guard109_dn4;
        *var_guard109_dn5_slot = var_guard109_dn5;
        *var_guard109_dn6_slot = var_guard109_dn6;
        *var_guard109_dn7_slot = var_guard109_dn7;
        *var_guard109_dn8_slot = var_guard109_dn8;
        *var_guard109_dn9_slot = var_guard109_dn9;
        *var_guard109_rdb0_slot = var_guard109_rdb0;
        *var_guard109_rdb1_slot = var_guard109_rdb1;
        *var_guard109_rdn0_slot = var_guard109_rdn0;
        *var_guard109_rdn1_slot = var_guard109_rdn1;
        *var_guard109_rdn10_slot = var_guard109_rdn10;
        *var_guard109_rdn2_slot = var_guard109_rdn2;
        *var_guard109_rdn3_slot = var_guard109_rdn3;
        *var_guard109_rdn4_slot = var_guard109_rdn4;
        *var_guard109_rdn5_slot = var_guard109_rdn5;
        *var_guard109_rdn6_slot = var_guard109_rdn6;
        *var_guard109_rdn7_slot = var_guard109_rdn7;
        *var_guard109_rdn8_slot = var_guard109_rdn8;
        *var_guard109_rdn9_slot = var_guard109_rdn9;
        *var_guard109_rv_slot = var_guard109_rv;
        *var_guard110_slot = var_guard110;
        *var_guard110_db0_slot = var_guard110_db0;
        *var_guard110_db1_slot = var_guard110_db1;
        *var_guard110_dn0_slot = var_guard110_dn0;
        *var_guard110_dn1_slot = var_guard110_dn1;
        *var_guard110_dn10_slot = var_guard110_dn10;
        *var_guard110_dn2_slot = var_guard110_dn2;
        *var_guard110_dn3_slot = var_guard110_dn3;
        *var_guard110_dn4_slot = var_guard110_dn4;
        *var_guard110_dn5_slot = var_guard110_dn5;
        *var_guard110_dn6_slot = var_guard110_dn6;
        *var_guard110_dn7_slot = var_guard110_dn7;
        *var_guard110_dn8_slot = var_guard110_dn8;
        *var_guard110_dn9_slot = var_guard110_dn9;
        *var_guard110_rdb0_slot = var_guard110_rdb0;
        *var_guard110_rdb1_slot = var_guard110_rdb1;
        *var_guard110_rdn0_slot = var_guard110_rdn0;
        *var_guard110_rdn1_slot = var_guard110_rdn1;
        *var_guard110_rdn10_slot = var_guard110_rdn10;
        *var_guard110_rdn2_slot = var_guard110_rdn2;
        *var_guard110_rdn3_slot = var_guard110_rdn3;
        *var_guard110_rdn4_slot = var_guard110_rdn4;
        *var_guard110_rdn5_slot = var_guard110_rdn5;
        *var_guard110_rdn6_slot = var_guard110_rdn6;
        *var_guard110_rdn7_slot = var_guard110_rdn7;
        *var_guard110_rdn8_slot = var_guard110_rdn8;
        *var_guard110_rdn9_slot = var_guard110_rdn9;
        *var_guard110_rv_slot = var_guard110_rv;
        *var_qex_slot = var_qex;
        *var_qex_db0_slot = var_qex_db0;
        *var_qex_db1_slot = var_qex_db1;
        *var_qex_dn0_slot = var_qex_dn0;
        *var_qex_dn1_slot = var_qex_dn1;
        *var_qex_dn10_slot = var_qex_dn10;
        *var_qex_dn2_slot = var_qex_dn2;
        *var_qex_dn3_slot = var_qex_dn3;
        *var_qex_dn4_slot = var_qex_dn4;
        *var_qex_dn5_slot = var_qex_dn5;
        *var_qex_dn6_slot = var_qex_dn6;
        *var_qex_dn7_slot = var_qex_dn7;
        *var_qex_dn8_slot = var_qex_dn8;
        *var_qex_dn9_slot = var_qex_dn9;
        *var_qex_rdb0_slot = var_qex_rdb0;
        *var_qex_rdb1_slot = var_qex_rdb1;
        *var_qex_rdn0_slot = var_qex_rdn0;
        *var_qex_rdn1_slot = var_qex_rdn1;
        *var_qex_rdn10_slot = var_qex_rdn10;
        *var_qex_rdn2_slot = var_qex_rdn2;
        *var_qex_rdn3_slot = var_qex_rdn3;
        *var_qex_rdn4_slot = var_qex_rdn4;
        *var_qex_rdn5_slot = var_qex_rdn5;
        *var_qex_rdn6_slot = var_qex_rdn6;
        *var_qex_rdn7_slot = var_qex_rdn7;
        *var_qex_rdn8_slot = var_qex_rdn8;
        *var_qex_rdn9_slot = var_qex_rdn9;
        *var_qex_rv_slot = var_qex_rv;
        *var_xg1_slot = var_xg1;
        *var_xg1_db0_slot = var_xg1_db0;
        *var_xg1_db1_slot = var_xg1_db1;
        *var_xg1_dn0_slot = var_xg1_dn0;
        *var_xg1_dn1_slot = var_xg1_dn1;
        *var_xg1_dn10_slot = var_xg1_dn10;
        *var_xg1_dn2_slot = var_xg1_dn2;
        *var_xg1_dn3_slot = var_xg1_dn3;
        *var_xg1_dn4_slot = var_xg1_dn4;
        *var_xg1_dn5_slot = var_xg1_dn5;
        *var_xg1_dn6_slot = var_xg1_dn6;
        *var_xg1_dn7_slot = var_xg1_dn7;
        *var_xg1_dn8_slot = var_xg1_dn8;
        *var_xg1_dn9_slot = var_xg1_dn9;
        *var_xg1_rdb0_slot = var_xg1_rdb0;
        *var_xg1_rdb1_slot = var_xg1_rdb1;
        *var_xg1_rdn0_slot = var_xg1_rdn0;
        *var_xg1_rdn1_slot = var_xg1_rdn1;
        *var_xg1_rdn10_slot = var_xg1_rdn10;
        *var_xg1_rdn2_slot = var_xg1_rdn2;
        *var_xg1_rdn3_slot = var_xg1_rdn3;
        *var_xg1_rdn4_slot = var_xg1_rdn4;
        *var_xg1_rdn5_slot = var_xg1_rdn5;
        *var_xg1_rdn6_slot = var_xg1_rdn6;
        *var_xg1_rdn7_slot = var_xg1_rdn7;
        *var_xg1_rdn8_slot = var_xg1_rdn8;
        *var_xg1_rdn9_slot = var_xg1_rdn9;
        *var_xg1_rv_slot = var_xg1_rv;
        *var_xg2_slot = var_xg2;
        *var_xg2_db0_slot = var_xg2_db0;
        *var_xg2_db1_slot = var_xg2_db1;
        *var_xg2_dn0_slot = var_xg2_dn0;
        *var_xg2_dn1_slot = var_xg2_dn1;
        *var_xg2_dn10_slot = var_xg2_dn10;
        *var_xg2_dn2_slot = var_xg2_dn2;
        *var_xg2_dn3_slot = var_xg2_dn3;
        *var_xg2_dn4_slot = var_xg2_dn4;
        *var_xg2_dn5_slot = var_xg2_dn5;
        *var_xg2_dn6_slot = var_xg2_dn6;
        *var_xg2_dn7_slot = var_xg2_dn7;
        *var_xg2_dn8_slot = var_xg2_dn8;
        *var_xg2_dn9_slot = var_xg2_dn9;
        *var_xg2_rdb0_slot = var_xg2_rdb0;
        *var_xg2_rdb1_slot = var_xg2_rdb1;
        *var_xg2_rdn0_slot = var_xg2_rdn0;
        *var_xg2_rdn1_slot = var_xg2_rdn1;
        *var_xg2_rdn10_slot = var_xg2_rdn10;
        *var_xg2_rdn2_slot = var_xg2_rdn2;
        *var_xg2_rdn3_slot = var_xg2_rdn3;
        *var_xg2_rdn4_slot = var_xg2_rdn4;
        *var_xg2_rdn5_slot = var_xg2_rdn5;
        *var_xg2_rdn6_slot = var_xg2_rdn6;
        *var_xg2_rdn7_slot = var_xg2_rdn7;
        *var_xg2_rdn8_slot = var_xg2_rdn8;
        *var_xg2_rdn9_slot = var_xg2_rdn9;
        *var_xg2_rv_slot = var_xg2_rv;
        *var_xnbex_slot = var_xnbex;
        *var_xnbex_db0_slot = var_xnbex_db0;
        *var_xnbex_db1_slot = var_xnbex_db1;
        *var_xnbex_dn0_slot = var_xnbex_dn0;
        *var_xnbex_dn1_slot = var_xnbex_dn1;
        *var_xnbex_dn10_slot = var_xnbex_dn10;
        *var_xnbex_dn2_slot = var_xnbex_dn2;
        *var_xnbex_dn3_slot = var_xnbex_dn3;
        *var_xnbex_dn4_slot = var_xnbex_dn4;
        *var_xnbex_dn5_slot = var_xnbex_dn5;
        *var_xnbex_dn6_slot = var_xnbex_dn6;
        *var_xnbex_dn7_slot = var_xnbex_dn7;
        *var_xnbex_dn8_slot = var_xnbex_dn8;
        *var_xnbex_dn9_slot = var_xnbex_dn9;
        *var_xnbex_rdb0_slot = var_xnbex_rdb0;
        *var_xnbex_rdb1_slot = var_xnbex_rdb1;
        *var_xnbex_rdn0_slot = var_xnbex_rdn0;
        *var_xnbex_rdn1_slot = var_xnbex_rdn1;
        *var_xnbex_rdn10_slot = var_xnbex_rdn10;
        *var_xnbex_rdn2_slot = var_xnbex_rdn2;
        *var_xnbex_rdn3_slot = var_xnbex_rdn3;
        *var_xnbex_rdn4_slot = var_xnbex_rdn4;
        *var_xnbex_rdn5_slot = var_xnbex_rdn5;
        *var_xnbex_rdn6_slot = var_xnbex_rdn6;
        *var_xnbex_rdn7_slot = var_xnbex_rdn7;
        *var_xnbex_rdn8_slot = var_xnbex_rdn8;
        *var_xnbex_rdn9_slot = var_xnbex_rdn9;
        *var_xnbex_rv_slot = var_xnbex_rv;
        *var_xpwex_slot = var_xpwex;
        *var_xpwex_db0_slot = var_xpwex_db0;
        *var_xpwex_db1_slot = var_xpwex_db1;
        *var_xpwex_dn0_slot = var_xpwex_dn0;
        *var_xpwex_dn1_slot = var_xpwex_dn1;
        *var_xpwex_dn10_slot = var_xpwex_dn10;
        *var_xpwex_dn2_slot = var_xpwex_dn2;
        *var_xpwex_dn3_slot = var_xpwex_dn3;
        *var_xpwex_dn4_slot = var_xpwex_dn4;
        *var_xpwex_dn5_slot = var_xpwex_dn5;
        *var_xpwex_dn6_slot = var_xpwex_dn6;
        *var_xpwex_dn7_slot = var_xpwex_dn7;
        *var_xpwex_dn8_slot = var_xpwex_dn8;
        *var_xpwex_dn9_slot = var_xpwex_dn9;
        *var_xpwex_rdb0_slot = var_xpwex_rdb0;
        *var_xpwex_rdb1_slot = var_xpwex_rdb1;
        *var_xpwex_rdn0_slot = var_xpwex_rdn0;
        *var_xpwex_rdn1_slot = var_xpwex_rdn1;
        *var_xpwex_rdn10_slot = var_xpwex_rdn10;
        *var_xpwex_rdn2_slot = var_xpwex_rdn2;
        *var_xpwex_rdn3_slot = var_xpwex_rdn3;
        *var_xpwex_rdn4_slot = var_xpwex_rdn4;
        *var_xpwex_rdn5_slot = var_xpwex_rdn5;
        *var_xpwex_rdn6_slot = var_xpwex_rdn6;
        *var_xpwex_rdn7_slot = var_xpwex_rdn7;
        *var_xpwex_rdn8_slot = var_xpwex_rdn8;
        *var_xpwex_rdn9_slot = var_xpwex_rdn9;
        *var_xpwex_rv_slot = var_xpwex_rv;
        *var_xqmex_slot = var_xqmex;
        *var_xqmex_db0_slot = var_xqmex_db0;
        *var_xqmex_db1_slot = var_xqmex_db1;
        *var_xqmex_dn0_slot = var_xqmex_dn0;
        *var_xqmex_dn1_slot = var_xqmex_dn1;
        *var_xqmex_dn10_slot = var_xqmex_dn10;
        *var_xqmex_dn2_slot = var_xqmex_dn2;
        *var_xqmex_dn3_slot = var_xqmex_dn3;
        *var_xqmex_dn4_slot = var_xqmex_dn4;
        *var_xqmex_dn5_slot = var_xqmex_dn5;
        *var_xqmex_dn6_slot = var_xqmex_dn6;
        *var_xqmex_dn7_slot = var_xqmex_dn7;
        *var_xqmex_dn8_slot = var_xqmex_dn8;
        *var_xqmex_dn9_slot = var_xqmex_dn9;
        *var_xqmex_rdb0_slot = var_xqmex_rdb0;
        *var_xqmex_rdb1_slot = var_xqmex_rdb1;
        *var_xqmex_rdn0_slot = var_xqmex_rdn0;
        *var_xqmex_rdn1_slot = var_xqmex_rdn1;
        *var_xqmex_rdn10_slot = var_xqmex_rdn10;
        *var_xqmex_rdn2_slot = var_xqmex_rdn2;
        *var_xqmex_rdn3_slot = var_xqmex_rdn3;
        *var_xqmex_rdn4_slot = var_xqmex_rdn4;
        *var_xqmex_rdn5_slot = var_xqmex_rdn5;
        *var_xqmex_rdn6_slot = var_xqmex_rdn6;
        *var_xqmex_rdn7_slot = var_xqmex_rdn7;
        *var_xqmex_rdn8_slot = var_xqmex_rdn8;
        *var_xqmex_rdn9_slot = var_xqmex_rdn9;
        *var_xqmex_rv_slot = var_xqmex_rv;
    }

    pub(super) fn stamp_reactive_block_36(
        p: &Parameters,
        var_a_vde: f64,
        var_a_vde_db0: f64,
        var_a_vde_db1: f64,
        var_a_vde_dn0: f64,
        var_a_vde_dn1: f64,
        var_a_vde_dn10: f64,
        var_a_vde_dn2: f64,
        var_a_vde_dn3: f64,
        var_a_vde_dn4: f64,
        var_a_vde_dn5: f64,
        var_a_vde_dn6: f64,
        var_a_vde_dn7: f64,
        var_a_vde_dn8: f64,
        var_a_vde_dn9: f64,
        var_evbc3: f64,
        var_evbc3_db0: f64,
        var_evbc3_db1: f64,
        var_evbc3_dn0: f64,
        var_evbc3_dn1: f64,
        var_evbc3_dn10: f64,
        var_evbc3_dn2: f64,
        var_evbc3_dn3: f64,
        var_evbc3_dn4: f64,
        var_evbc3_dn5: f64,
        var_evbc3_dn6: f64,
        var_evbc3_dn7: f64,
        var_evbc3_dn8: f64,
        var_evbc3_dn9: f64,
        var_fex: f64,
        var_fex_db0: f64,
        var_fex_db1: f64,
        var_fex_dn0: f64,
        var_fex_dn1: f64,
        var_fex_dn10: f64,
        var_fex_dn2: f64,
        var_fex_dn3: f64,
        var_fex_dn4: f64,
        var_fex_dn5: f64,
        var_fex_dn6: f64,
        var_fex_dn7: f64,
        var_fex_dn8: f64,
        var_fex_dn9: f64,
        var_guard109: f64,
        var_guard110: f64,
        var_ibx_t: f64,
        var_ibx_t_db0: f64,
        var_ibx_t_db1: f64,
        var_ibx_t_dn0: f64,
        var_ibx_t_dn1: f64,
        var_ibx_t_dn10: f64,
        var_ibx_t_dn2: f64,
        var_ibx_t_dn3: f64,
        var_ibx_t_dn4: f64,
        var_ibx_t_dn5: f64,
        var_ibx_t_dn6: f64,
        var_ibx_t_dn7: f64,
        var_ibx_t_dn8: f64,
        var_ibx_t_dn9: f64,
        var_inv_vde_t: f64,
        var_inv_vde_t_db0: f64,
        var_inv_vde_t_db1: f64,
        var_inv_vde_t_dn0: f64,
        var_inv_vde_t_dn1: f64,
        var_inv_vde_t_dn10: f64,
        var_inv_vde_t_dn2: f64,
        var_inv_vde_t_dn3: f64,
        var_inv_vde_t_dn4: f64,
        var_inv_vde_t_dn5: f64,
        var_inv_vde_t_dn6: f64,
        var_inv_vde_t_dn7: f64,
        var_inv_vde_t_dn8: f64,
        var_inv_vde_t_dn9: f64,
        var_tauex_t: f64,
        var_tauex_t_db0: f64,
        var_tauex_t_db1: f64,
        var_tauex_t_dn0: f64,
        var_tauex_t_dn1: f64,
        var_tauex_t_dn10: f64,
        var_tauex_t_dn2: f64,
        var_tauex_t_dn3: f64,
        var_tauex_t_dn4: f64,
        var_tauex_t_dn5: f64,
        var_tauex_t_dn6: f64,
        var_tauex_t_dn7: f64,
        var_tauex_t_dn8: f64,
        var_tauex_t_dn9: f64,
        var_vb2e1: f64,
        var_vb2e1_db0: f64,
        var_vb2e1_db1: f64,
        var_vb2e1_dn0: f64,
        var_vb2e1_dn1: f64,
        var_vb2e1_dn10: f64,
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
        var_vje_dn2: f64,
        var_vje_dn3: f64,
        var_vje_dn4: f64,
        var_vje_dn5: f64,
        var_vje_dn6: f64,
        var_vje_dn7: f64,
        var_vje_dn8: f64,
        var_vje_dn9: f64,
        var_vtinv: f64,
        var_vtinv_db0: f64,
        var_vtinv_db1: f64,
        var_vtinv_dn0: f64,
        var_vtinv_dn1: f64,
        var_vtinv_dn10: f64,
        var_vtinv_dn2: f64,
        var_vtinv_dn3: f64,
        var_vtinv_dn4: f64,
        var_vtinv_dn5: f64,
        var_vtinv_dn6: f64,
        var_vtinv_dn7: f64,
        var_vtinv_dn8: f64,
        var_vtinv_dn9: f64,
        var_dvjevb2e1_slot: &mut f64,
        var_dvjevb2e1_db0_slot: &mut f64,
        var_dvjevb2e1_db1_slot: &mut f64,
        var_dvjevb2e1_dn0_slot: &mut f64,
        var_dvjevb2e1_dn1_slot: &mut f64,
        var_dvjevb2e1_dn10_slot: &mut f64,
        var_dvjevb2e1_dn2_slot: &mut f64,
        var_dvjevb2e1_dn3_slot: &mut f64,
        var_dvjevb2e1_dn4_slot: &mut f64,
        var_dvjevb2e1_dn5_slot: &mut f64,
        var_dvjevb2e1_dn6_slot: &mut f64,
        var_dvjevb2e1_dn7_slot: &mut f64,
        var_dvjevb2e1_dn8_slot: &mut f64,
        var_dvjevb2e1_dn9_slot: &mut f64,
        var_dvjevb2e1_rdb0_slot: &mut f64,
        var_dvjevb2e1_rdb1_slot: &mut f64,
        var_dvjevb2e1_rdn0_slot: &mut f64,
        var_dvjevb2e1_rdn1_slot: &mut f64,
        var_dvjevb2e1_rdn10_slot: &mut f64,
        var_dvjevb2e1_rdn2_slot: &mut f64,
        var_dvjevb2e1_rdn3_slot: &mut f64,
        var_dvjevb2e1_rdn4_slot: &mut f64,
        var_dvjevb2e1_rdn5_slot: &mut f64,
        var_dvjevb2e1_rdn6_slot: &mut f64,
        var_dvjevb2e1_rdn7_slot: &mut f64,
        var_dvjevb2e1_rdn8_slot: &mut f64,
        var_dvjevb2e1_rdn9_slot: &mut f64,
        var_dvjevb2e1_rv_slot: &mut f64,
        var_dvtevb2e1_slot: &mut f64,
        var_dvtevb2e1_db0_slot: &mut f64,
        var_dvtevb2e1_db1_slot: &mut f64,
        var_dvtevb2e1_dn0_slot: &mut f64,
        var_dvtevb2e1_dn1_slot: &mut f64,
        var_dvtevb2e1_dn10_slot: &mut f64,
        var_dvtevb2e1_dn2_slot: &mut f64,
        var_dvtevb2e1_dn3_slot: &mut f64,
        var_dvtevb2e1_dn4_slot: &mut f64,
        var_dvtevb2e1_dn5_slot: &mut f64,
        var_dvtevb2e1_dn6_slot: &mut f64,
        var_dvtevb2e1_dn7_slot: &mut f64,
        var_dvtevb2e1_dn8_slot: &mut f64,
        var_dvtevb2e1_dn9_slot: &mut f64,
        var_dvtevb2e1_rdb0_slot: &mut f64,
        var_dvtevb2e1_rdb1_slot: &mut f64,
        var_dvtevb2e1_rdn0_slot: &mut f64,
        var_dvtevb2e1_rdn1_slot: &mut f64,
        var_dvtevb2e1_rdn10_slot: &mut f64,
        var_dvtevb2e1_rdn2_slot: &mut f64,
        var_dvtevb2e1_rdn3_slot: &mut f64,
        var_dvtevb2e1_rdn4_slot: &mut f64,
        var_dvtevb2e1_rdn5_slot: &mut f64,
        var_dvtevb2e1_rdn6_slot: &mut f64,
        var_dvtevb2e1_rdn7_slot: &mut f64,
        var_dvtevb2e1_rdn8_slot: &mut f64,
        var_dvtevb2e1_rdn9_slot: &mut f64,
        var_dvtevb2e1_rv_slot: &mut f64,
        var_dvtevje_slot: &mut f64,
        var_dvtevje_db0_slot: &mut f64,
        var_dvtevje_db1_slot: &mut f64,
        var_dvtevje_dn0_slot: &mut f64,
        var_dvtevje_dn1_slot: &mut f64,
        var_dvtevje_dn10_slot: &mut f64,
        var_dvtevje_dn2_slot: &mut f64,
        var_dvtevje_dn3_slot: &mut f64,
        var_dvtevje_dn4_slot: &mut f64,
        var_dvtevje_dn5_slot: &mut f64,
        var_dvtevje_dn6_slot: &mut f64,
        var_dvtevje_dn7_slot: &mut f64,
        var_dvtevje_dn8_slot: &mut f64,
        var_dvtevje_dn9_slot: &mut f64,
        var_dvtevje_rdb0_slot: &mut f64,
        var_dvtevje_rdb1_slot: &mut f64,
        var_dvtevje_rdn0_slot: &mut f64,
        var_dvtevje_rdn1_slot: &mut f64,
        var_dvtevje_rdn10_slot: &mut f64,
        var_dvtevje_rdn2_slot: &mut f64,
        var_dvtevje_rdn3_slot: &mut f64,
        var_dvtevje_rdn4_slot: &mut f64,
        var_dvtevje_rdn5_slot: &mut f64,
        var_dvtevje_rdn6_slot: &mut f64,
        var_dvtevje_rdn7_slot: &mut f64,
        var_dvtevje_rdn8_slot: &mut f64,
        var_dvtevje_rdn9_slot: &mut f64,
        var_dvtevje_rv_slot: &mut f64,
        var_evbc3vdcex_slot: &mut f64,
        var_evbc3vdcex_db0_slot: &mut f64,
        var_evbc3vdcex_db1_slot: &mut f64,
        var_evbc3vdcex_dn0_slot: &mut f64,
        var_evbc3vdcex_dn1_slot: &mut f64,
        var_evbc3vdcex_dn10_slot: &mut f64,
        var_evbc3vdcex_dn2_slot: &mut f64,
        var_evbc3vdcex_dn3_slot: &mut f64,
        var_evbc3vdcex_dn4_slot: &mut f64,
        var_evbc3vdcex_dn5_slot: &mut f64,
        var_evbc3vdcex_dn6_slot: &mut f64,
        var_evbc3vdcex_dn7_slot: &mut f64,
        var_evbc3vdcex_dn8_slot: &mut f64,
        var_evbc3vdcex_dn9_slot: &mut f64,
        var_evbc3vdcex_rdb0_slot: &mut f64,
        var_evbc3vdcex_rdb1_slot: &mut f64,
        var_evbc3vdcex_rdn0_slot: &mut f64,
        var_evbc3vdcex_rdn1_slot: &mut f64,
        var_evbc3vdcex_rdn10_slot: &mut f64,
        var_evbc3vdcex_rdn2_slot: &mut f64,
        var_evbc3vdcex_rdn3_slot: &mut f64,
        var_evbc3vdcex_rdn4_slot: &mut f64,
        var_evbc3vdcex_rdn5_slot: &mut f64,
        var_evbc3vdcex_rdn6_slot: &mut f64,
        var_evbc3vdcex_rdn7_slot: &mut f64,
        var_evbc3vdcex_rdn8_slot: &mut f64,
        var_evbc3vdcex_rdn9_slot: &mut f64,
        var_evbc3vdcex_rv_slot: &mut f64,
        var_expl_slot: &mut f64,
        var_expl_db0_slot: &mut f64,
        var_expl_db1_slot: &mut f64,
        var_expl_dn0_slot: &mut f64,
        var_expl_dn1_slot: &mut f64,
        var_expl_dn10_slot: &mut f64,
        var_expl_dn2_slot: &mut f64,
        var_expl_dn3_slot: &mut f64,
        var_expl_dn4_slot: &mut f64,
        var_expl_dn5_slot: &mut f64,
        var_expl_dn6_slot: &mut f64,
        var_expl_dn7_slot: &mut f64,
        var_expl_dn8_slot: &mut f64,
        var_expl_dn9_slot: &mut f64,
        var_expl_rdb0_slot: &mut f64,
        var_expl_rdb1_slot: &mut f64,
        var_expl_rdn0_slot: &mut f64,
        var_expl_rdn1_slot: &mut f64,
        var_expl_rdn10_slot: &mut f64,
        var_expl_rdn2_slot: &mut f64,
        var_expl_rdn3_slot: &mut f64,
        var_expl_rdn4_slot: &mut f64,
        var_expl_rdn5_slot: &mut f64,
        var_expl_rdn6_slot: &mut f64,
        var_expl_rdn7_slot: &mut f64,
        var_expl_rdn8_slot: &mut f64,
        var_expl_rdn9_slot: &mut f64,
        var_expl_rv_slot: &mut f64,
        var_guard111_slot: &mut f64,
        var_guard111_db0_slot: &mut f64,
        var_guard111_db1_slot: &mut f64,
        var_guard111_dn0_slot: &mut f64,
        var_guard111_dn1_slot: &mut f64,
        var_guard111_dn10_slot: &mut f64,
        var_guard111_dn2_slot: &mut f64,
        var_guard111_dn3_slot: &mut f64,
        var_guard111_dn4_slot: &mut f64,
        var_guard111_dn5_slot: &mut f64,
        var_guard111_dn6_slot: &mut f64,
        var_guard111_dn7_slot: &mut f64,
        var_guard111_dn8_slot: &mut f64,
        var_guard111_dn9_slot: &mut f64,
        var_guard111_rdb0_slot: &mut f64,
        var_guard111_rdb1_slot: &mut f64,
        var_guard111_rdn0_slot: &mut f64,
        var_guard111_rdn1_slot: &mut f64,
        var_guard111_rdn10_slot: &mut f64,
        var_guard111_rdn2_slot: &mut f64,
        var_guard111_rdn3_slot: &mut f64,
        var_guard111_rdn4_slot: &mut f64,
        var_guard111_rdn5_slot: &mut f64,
        var_guard111_rdn6_slot: &mut f64,
        var_guard111_rdn7_slot: &mut f64,
        var_guard111_rdn8_slot: &mut f64,
        var_guard111_rdn9_slot: &mut f64,
        var_guard111_rv_slot: &mut f64,
        var_guard112_slot: &mut f64,
        var_guard112_db0_slot: &mut f64,
        var_guard112_db1_slot: &mut f64,
        var_guard112_dn0_slot: &mut f64,
        var_guard112_dn1_slot: &mut f64,
        var_guard112_dn10_slot: &mut f64,
        var_guard112_dn2_slot: &mut f64,
        var_guard112_dn3_slot: &mut f64,
        var_guard112_dn4_slot: &mut f64,
        var_guard112_dn5_slot: &mut f64,
        var_guard112_dn6_slot: &mut f64,
        var_guard112_dn7_slot: &mut f64,
        var_guard112_dn8_slot: &mut f64,
        var_guard112_dn9_slot: &mut f64,
        var_guard112_rdb0_slot: &mut f64,
        var_guard112_rdb1_slot: &mut f64,
        var_guard112_rdn0_slot: &mut f64,
        var_guard112_rdn1_slot: &mut f64,
        var_guard112_rdn10_slot: &mut f64,
        var_guard112_rdn2_slot: &mut f64,
        var_guard112_rdn3_slot: &mut f64,
        var_guard112_rdn4_slot: &mut f64,
        var_guard112_rdn5_slot: &mut f64,
        var_guard112_rdn6_slot: &mut f64,
        var_guard112_rdn7_slot: &mut f64,
        var_guard112_rdn8_slot: &mut f64,
        var_guard112_rdn9_slot: &mut f64,
        var_guard112_rv_slot: &mut f64,
        var_guard113_slot: &mut f64,
        var_guard113_db0_slot: &mut f64,
        var_guard113_db1_slot: &mut f64,
        var_guard113_dn0_slot: &mut f64,
        var_guard113_dn1_slot: &mut f64,
        var_guard113_dn10_slot: &mut f64,
        var_guard113_dn2_slot: &mut f64,
        var_guard113_dn3_slot: &mut f64,
        var_guard113_dn4_slot: &mut f64,
        var_guard113_dn5_slot: &mut f64,
        var_guard113_dn6_slot: &mut f64,
        var_guard113_dn7_slot: &mut f64,
        var_guard113_dn8_slot: &mut f64,
        var_guard113_dn9_slot: &mut f64,
        var_guard113_rdb0_slot: &mut f64,
        var_guard113_rdb1_slot: &mut f64,
        var_guard113_rdn0_slot: &mut f64,
        var_guard113_rdn1_slot: &mut f64,
        var_guard113_rdn10_slot: &mut f64,
        var_guard113_rdn2_slot: &mut f64,
        var_guard113_rdn3_slot: &mut f64,
        var_guard113_rdn4_slot: &mut f64,
        var_guard113_rdn5_slot: &mut f64,
        var_guard113_rdn6_slot: &mut f64,
        var_guard113_rdn7_slot: &mut f64,
        var_guard113_rdn8_slot: &mut f64,
        var_guard113_rdn9_slot: &mut f64,
        var_guard113_rv_slot: &mut f64,
        var_vb2e1vfe_slot: &mut f64,
        var_vb2e1vfe_db0_slot: &mut f64,
        var_vb2e1vfe_db1_slot: &mut f64,
        var_vb2e1vfe_dn0_slot: &mut f64,
        var_vb2e1vfe_dn1_slot: &mut f64,
        var_vb2e1vfe_dn10_slot: &mut f64,
        var_vb2e1vfe_dn2_slot: &mut f64,
        var_vb2e1vfe_dn3_slot: &mut f64,
        var_vb2e1vfe_dn4_slot: &mut f64,
        var_vb2e1vfe_dn5_slot: &mut f64,
        var_vb2e1vfe_dn6_slot: &mut f64,
        var_vb2e1vfe_dn7_slot: &mut f64,
        var_vb2e1vfe_dn8_slot: &mut f64,
        var_vb2e1vfe_dn9_slot: &mut f64,
        var_vb2e1vfe_rdb0_slot: &mut f64,
        var_vb2e1vfe_rdb1_slot: &mut f64,
        var_vb2e1vfe_rdn0_slot: &mut f64,
        var_vb2e1vfe_rdn1_slot: &mut f64,
        var_vb2e1vfe_rdn10_slot: &mut f64,
        var_vb2e1vfe_rdn2_slot: &mut f64,
        var_vb2e1vfe_rdn3_slot: &mut f64,
        var_vb2e1vfe_rdn4_slot: &mut f64,
        var_vb2e1vfe_rdn5_slot: &mut f64,
        var_vb2e1vfe_rdn6_slot: &mut f64,
        var_vb2e1vfe_rdn7_slot: &mut f64,
        var_vb2e1vfe_rdn8_slot: &mut f64,
        var_vb2e1vfe_rdn9_slot: &mut f64,
        var_vb2e1vfe_rv_slot: &mut f64,
        var_xqex_slot: &mut f64,
        var_xqex_db0_slot: &mut f64,
        var_xqex_db1_slot: &mut f64,
        var_xqex_dn0_slot: &mut f64,
        var_xqex_dn1_slot: &mut f64,
        var_xqex_dn10_slot: &mut f64,
        var_xqex_dn2_slot: &mut f64,
        var_xqex_dn3_slot: &mut f64,
        var_xqex_dn4_slot: &mut f64,
        var_xqex_dn5_slot: &mut f64,
        var_xqex_dn6_slot: &mut f64,
        var_xqex_dn7_slot: &mut f64,
        var_xqex_dn8_slot: &mut f64,
        var_xqex_dn9_slot: &mut f64,
        var_xqex_rdb0_slot: &mut f64,
        var_xqex_rdb1_slot: &mut f64,
        var_xqex_rdn0_slot: &mut f64,
        var_xqex_rdn1_slot: &mut f64,
        var_xqex_rdn10_slot: &mut f64,
        var_xqex_rdn2_slot: &mut f64,
        var_xqex_rdn3_slot: &mut f64,
        var_xqex_rdn4_slot: &mut f64,
        var_xqex_rdn5_slot: &mut f64,
        var_xqex_rdn6_slot: &mut f64,
        var_xqex_rdn7_slot: &mut f64,
        var_xqex_rdn8_slot: &mut f64,
        var_xqex_rdn9_slot: &mut f64,
        var_xqex_rv_slot: &mut f64,
        var_xqmex_slot: &mut f64,
        var_xqmex_db0_slot: &mut f64,
        var_xqmex_db1_slot: &mut f64,
        var_xqmex_dn0_slot: &mut f64,
        var_xqmex_dn1_slot: &mut f64,
        var_xqmex_dn10_slot: &mut f64,
        var_xqmex_dn2_slot: &mut f64,
        var_xqmex_dn3_slot: &mut f64,
        var_xqmex_dn4_slot: &mut f64,
        var_xqmex_dn5_slot: &mut f64,
        var_xqmex_dn6_slot: &mut f64,
        var_xqmex_dn7_slot: &mut f64,
        var_xqmex_dn8_slot: &mut f64,
        var_xqmex_dn9_slot: &mut f64,
        var_xqmex_rdb0_slot: &mut f64,
        var_xqmex_rdb1_slot: &mut f64,
        var_xqmex_rdn0_slot: &mut f64,
        var_xqmex_rdn1_slot: &mut f64,
        var_xqmex_rdn10_slot: &mut f64,
        var_xqmex_rdn2_slot: &mut f64,
        var_xqmex_rdn3_slot: &mut f64,
        var_xqmex_rdn4_slot: &mut f64,
        var_xqmex_rdn5_slot: &mut f64,
        var_xqmex_rdn6_slot: &mut f64,
        var_xqmex_rdn7_slot: &mut f64,
        var_xqmex_rdn8_slot: &mut f64,
        var_xqmex_rdn9_slot: &mut f64,
        var_xqmex_rv_slot: &mut f64,
    ) {
        let mut var_dvjevb2e1: f64 = *var_dvjevb2e1_slot;
        let mut var_dvjevb2e1_db0: f64 = *var_dvjevb2e1_db0_slot;
        let mut var_dvjevb2e1_db1: f64 = *var_dvjevb2e1_db1_slot;
        let mut var_dvjevb2e1_dn0: f64 = *var_dvjevb2e1_dn0_slot;
        let mut var_dvjevb2e1_dn1: f64 = *var_dvjevb2e1_dn1_slot;
        let mut var_dvjevb2e1_dn10: f64 = *var_dvjevb2e1_dn10_slot;
        let mut var_dvjevb2e1_dn2: f64 = *var_dvjevb2e1_dn2_slot;
        let mut var_dvjevb2e1_dn3: f64 = *var_dvjevb2e1_dn3_slot;
        let mut var_dvjevb2e1_dn4: f64 = *var_dvjevb2e1_dn4_slot;
        let mut var_dvjevb2e1_dn5: f64 = *var_dvjevb2e1_dn5_slot;
        let mut var_dvjevb2e1_dn6: f64 = *var_dvjevb2e1_dn6_slot;
        let mut var_dvjevb2e1_dn7: f64 = *var_dvjevb2e1_dn7_slot;
        let mut var_dvjevb2e1_dn8: f64 = *var_dvjevb2e1_dn8_slot;
        let mut var_dvjevb2e1_dn9: f64 = *var_dvjevb2e1_dn9_slot;
        let mut var_dvjevb2e1_rdb0: f64 = *var_dvjevb2e1_rdb0_slot;
        let mut var_dvjevb2e1_rdb1: f64 = *var_dvjevb2e1_rdb1_slot;
        let mut var_dvjevb2e1_rdn0: f64 = *var_dvjevb2e1_rdn0_slot;
        let mut var_dvjevb2e1_rdn1: f64 = *var_dvjevb2e1_rdn1_slot;
        let mut var_dvjevb2e1_rdn10: f64 = *var_dvjevb2e1_rdn10_slot;
        let mut var_dvjevb2e1_rdn2: f64 = *var_dvjevb2e1_rdn2_slot;
        let mut var_dvjevb2e1_rdn3: f64 = *var_dvjevb2e1_rdn3_slot;
        let mut var_dvjevb2e1_rdn4: f64 = *var_dvjevb2e1_rdn4_slot;
        let mut var_dvjevb2e1_rdn5: f64 = *var_dvjevb2e1_rdn5_slot;
        let mut var_dvjevb2e1_rdn6: f64 = *var_dvjevb2e1_rdn6_slot;
        let mut var_dvjevb2e1_rdn7: f64 = *var_dvjevb2e1_rdn7_slot;
        let mut var_dvjevb2e1_rdn8: f64 = *var_dvjevb2e1_rdn8_slot;
        let mut var_dvjevb2e1_rdn9: f64 = *var_dvjevb2e1_rdn9_slot;
        let mut var_dvjevb2e1_rv: f64 = *var_dvjevb2e1_rv_slot;
        let mut var_dvtevb2e1: f64 = *var_dvtevb2e1_slot;
        let mut var_dvtevb2e1_db0: f64 = *var_dvtevb2e1_db0_slot;
        let mut var_dvtevb2e1_db1: f64 = *var_dvtevb2e1_db1_slot;
        let mut var_dvtevb2e1_dn0: f64 = *var_dvtevb2e1_dn0_slot;
        let mut var_dvtevb2e1_dn1: f64 = *var_dvtevb2e1_dn1_slot;
        let mut var_dvtevb2e1_dn10: f64 = *var_dvtevb2e1_dn10_slot;
        let mut var_dvtevb2e1_dn2: f64 = *var_dvtevb2e1_dn2_slot;
        let mut var_dvtevb2e1_dn3: f64 = *var_dvtevb2e1_dn3_slot;
        let mut var_dvtevb2e1_dn4: f64 = *var_dvtevb2e1_dn4_slot;
        let mut var_dvtevb2e1_dn5: f64 = *var_dvtevb2e1_dn5_slot;
        let mut var_dvtevb2e1_dn6: f64 = *var_dvtevb2e1_dn6_slot;
        let mut var_dvtevb2e1_dn7: f64 = *var_dvtevb2e1_dn7_slot;
        let mut var_dvtevb2e1_dn8: f64 = *var_dvtevb2e1_dn8_slot;
        let mut var_dvtevb2e1_dn9: f64 = *var_dvtevb2e1_dn9_slot;
        let mut var_dvtevb2e1_rdb0: f64 = *var_dvtevb2e1_rdb0_slot;
        let mut var_dvtevb2e1_rdb1: f64 = *var_dvtevb2e1_rdb1_slot;
        let mut var_dvtevb2e1_rdn0: f64 = *var_dvtevb2e1_rdn0_slot;
        let mut var_dvtevb2e1_rdn1: f64 = *var_dvtevb2e1_rdn1_slot;
        let mut var_dvtevb2e1_rdn10: f64 = *var_dvtevb2e1_rdn10_slot;
        let mut var_dvtevb2e1_rdn2: f64 = *var_dvtevb2e1_rdn2_slot;
        let mut var_dvtevb2e1_rdn3: f64 = *var_dvtevb2e1_rdn3_slot;
        let mut var_dvtevb2e1_rdn4: f64 = *var_dvtevb2e1_rdn4_slot;
        let mut var_dvtevb2e1_rdn5: f64 = *var_dvtevb2e1_rdn5_slot;
        let mut var_dvtevb2e1_rdn6: f64 = *var_dvtevb2e1_rdn6_slot;
        let mut var_dvtevb2e1_rdn7: f64 = *var_dvtevb2e1_rdn7_slot;
        let mut var_dvtevb2e1_rdn8: f64 = *var_dvtevb2e1_rdn8_slot;
        let mut var_dvtevb2e1_rdn9: f64 = *var_dvtevb2e1_rdn9_slot;
        let mut var_dvtevb2e1_rv: f64 = *var_dvtevb2e1_rv_slot;
        let mut var_dvtevje: f64 = *var_dvtevje_slot;
        let mut var_dvtevje_db0: f64 = *var_dvtevje_db0_slot;
        let mut var_dvtevje_db1: f64 = *var_dvtevje_db1_slot;
        let mut var_dvtevje_dn0: f64 = *var_dvtevje_dn0_slot;
        let mut var_dvtevje_dn1: f64 = *var_dvtevje_dn1_slot;
        let mut var_dvtevje_dn10: f64 = *var_dvtevje_dn10_slot;
        let mut var_dvtevje_dn2: f64 = *var_dvtevje_dn2_slot;
        let mut var_dvtevje_dn3: f64 = *var_dvtevje_dn3_slot;
        let mut var_dvtevje_dn4: f64 = *var_dvtevje_dn4_slot;
        let mut var_dvtevje_dn5: f64 = *var_dvtevje_dn5_slot;
        let mut var_dvtevje_dn6: f64 = *var_dvtevje_dn6_slot;
        let mut var_dvtevje_dn7: f64 = *var_dvtevje_dn7_slot;
        let mut var_dvtevje_dn8: f64 = *var_dvtevje_dn8_slot;
        let mut var_dvtevje_dn9: f64 = *var_dvtevje_dn9_slot;
        let mut var_dvtevje_rdb0: f64 = *var_dvtevje_rdb0_slot;
        let mut var_dvtevje_rdb1: f64 = *var_dvtevje_rdb1_slot;
        let mut var_dvtevje_rdn0: f64 = *var_dvtevje_rdn0_slot;
        let mut var_dvtevje_rdn1: f64 = *var_dvtevje_rdn1_slot;
        let mut var_dvtevje_rdn10: f64 = *var_dvtevje_rdn10_slot;
        let mut var_dvtevje_rdn2: f64 = *var_dvtevje_rdn2_slot;
        let mut var_dvtevje_rdn3: f64 = *var_dvtevje_rdn3_slot;
        let mut var_dvtevje_rdn4: f64 = *var_dvtevje_rdn4_slot;
        let mut var_dvtevje_rdn5: f64 = *var_dvtevje_rdn5_slot;
        let mut var_dvtevje_rdn6: f64 = *var_dvtevje_rdn6_slot;
        let mut var_dvtevje_rdn7: f64 = *var_dvtevje_rdn7_slot;
        let mut var_dvtevje_rdn8: f64 = *var_dvtevje_rdn8_slot;
        let mut var_dvtevje_rdn9: f64 = *var_dvtevje_rdn9_slot;
        let mut var_dvtevje_rv: f64 = *var_dvtevje_rv_slot;
        let mut var_evbc3vdcex: f64 = *var_evbc3vdcex_slot;
        let mut var_evbc3vdcex_db0: f64 = *var_evbc3vdcex_db0_slot;
        let mut var_evbc3vdcex_db1: f64 = *var_evbc3vdcex_db1_slot;
        let mut var_evbc3vdcex_dn0: f64 = *var_evbc3vdcex_dn0_slot;
        let mut var_evbc3vdcex_dn1: f64 = *var_evbc3vdcex_dn1_slot;
        let mut var_evbc3vdcex_dn10: f64 = *var_evbc3vdcex_dn10_slot;
        let mut var_evbc3vdcex_dn2: f64 = *var_evbc3vdcex_dn2_slot;
        let mut var_evbc3vdcex_dn3: f64 = *var_evbc3vdcex_dn3_slot;
        let mut var_evbc3vdcex_dn4: f64 = *var_evbc3vdcex_dn4_slot;
        let mut var_evbc3vdcex_dn5: f64 = *var_evbc3vdcex_dn5_slot;
        let mut var_evbc3vdcex_dn6: f64 = *var_evbc3vdcex_dn6_slot;
        let mut var_evbc3vdcex_dn7: f64 = *var_evbc3vdcex_dn7_slot;
        let mut var_evbc3vdcex_dn8: f64 = *var_evbc3vdcex_dn8_slot;
        let mut var_evbc3vdcex_dn9: f64 = *var_evbc3vdcex_dn9_slot;
        let mut var_evbc3vdcex_rdb0: f64 = *var_evbc3vdcex_rdb0_slot;
        let mut var_evbc3vdcex_rdb1: f64 = *var_evbc3vdcex_rdb1_slot;
        let mut var_evbc3vdcex_rdn0: f64 = *var_evbc3vdcex_rdn0_slot;
        let mut var_evbc3vdcex_rdn1: f64 = *var_evbc3vdcex_rdn1_slot;
        let mut var_evbc3vdcex_rdn10: f64 = *var_evbc3vdcex_rdn10_slot;
        let mut var_evbc3vdcex_rdn2: f64 = *var_evbc3vdcex_rdn2_slot;
        let mut var_evbc3vdcex_rdn3: f64 = *var_evbc3vdcex_rdn3_slot;
        let mut var_evbc3vdcex_rdn4: f64 = *var_evbc3vdcex_rdn4_slot;
        let mut var_evbc3vdcex_rdn5: f64 = *var_evbc3vdcex_rdn5_slot;
        let mut var_evbc3vdcex_rdn6: f64 = *var_evbc3vdcex_rdn6_slot;
        let mut var_evbc3vdcex_rdn7: f64 = *var_evbc3vdcex_rdn7_slot;
        let mut var_evbc3vdcex_rdn8: f64 = *var_evbc3vdcex_rdn8_slot;
        let mut var_evbc3vdcex_rdn9: f64 = *var_evbc3vdcex_rdn9_slot;
        let mut var_evbc3vdcex_rv: f64 = *var_evbc3vdcex_rv_slot;
        let mut var_expl: f64 = *var_expl_slot;
        let mut var_expl_db0: f64 = *var_expl_db0_slot;
        let mut var_expl_db1: f64 = *var_expl_db1_slot;
        let mut var_expl_dn0: f64 = *var_expl_dn0_slot;
        let mut var_expl_dn1: f64 = *var_expl_dn1_slot;
        let mut var_expl_dn10: f64 = *var_expl_dn10_slot;
        let mut var_expl_dn2: f64 = *var_expl_dn2_slot;
        let mut var_expl_dn3: f64 = *var_expl_dn3_slot;
        let mut var_expl_dn4: f64 = *var_expl_dn4_slot;
        let mut var_expl_dn5: f64 = *var_expl_dn5_slot;
        let mut var_expl_dn6: f64 = *var_expl_dn6_slot;
        let mut var_expl_dn7: f64 = *var_expl_dn7_slot;
        let mut var_expl_dn8: f64 = *var_expl_dn8_slot;
        let mut var_expl_dn9: f64 = *var_expl_dn9_slot;
        let mut var_expl_rdb0: f64 = *var_expl_rdb0_slot;
        let mut var_expl_rdb1: f64 = *var_expl_rdb1_slot;
        let mut var_expl_rdn0: f64 = *var_expl_rdn0_slot;
        let mut var_expl_rdn1: f64 = *var_expl_rdn1_slot;
        let mut var_expl_rdn10: f64 = *var_expl_rdn10_slot;
        let mut var_expl_rdn2: f64 = *var_expl_rdn2_slot;
        let mut var_expl_rdn3: f64 = *var_expl_rdn3_slot;
        let mut var_expl_rdn4: f64 = *var_expl_rdn4_slot;
        let mut var_expl_rdn5: f64 = *var_expl_rdn5_slot;
        let mut var_expl_rdn6: f64 = *var_expl_rdn6_slot;
        let mut var_expl_rdn7: f64 = *var_expl_rdn7_slot;
        let mut var_expl_rdn8: f64 = *var_expl_rdn8_slot;
        let mut var_expl_rdn9: f64 = *var_expl_rdn9_slot;
        let mut var_expl_rv: f64 = *var_expl_rv_slot;
        let mut var_guard111: f64 = *var_guard111_slot;
        let mut var_guard111_db0: f64 = *var_guard111_db0_slot;
        let mut var_guard111_db1: f64 = *var_guard111_db1_slot;
        let mut var_guard111_dn0: f64 = *var_guard111_dn0_slot;
        let mut var_guard111_dn1: f64 = *var_guard111_dn1_slot;
        let mut var_guard111_dn10: f64 = *var_guard111_dn10_slot;
        let mut var_guard111_dn2: f64 = *var_guard111_dn2_slot;
        let mut var_guard111_dn3: f64 = *var_guard111_dn3_slot;
        let mut var_guard111_dn4: f64 = *var_guard111_dn4_slot;
        let mut var_guard111_dn5: f64 = *var_guard111_dn5_slot;
        let mut var_guard111_dn6: f64 = *var_guard111_dn6_slot;
        let mut var_guard111_dn7: f64 = *var_guard111_dn7_slot;
        let mut var_guard111_dn8: f64 = *var_guard111_dn8_slot;
        let mut var_guard111_dn9: f64 = *var_guard111_dn9_slot;
        let mut var_guard111_rdb0: f64 = *var_guard111_rdb0_slot;
        let mut var_guard111_rdb1: f64 = *var_guard111_rdb1_slot;
        let mut var_guard111_rdn0: f64 = *var_guard111_rdn0_slot;
        let mut var_guard111_rdn1: f64 = *var_guard111_rdn1_slot;
        let mut var_guard111_rdn10: f64 = *var_guard111_rdn10_slot;
        let mut var_guard111_rdn2: f64 = *var_guard111_rdn2_slot;
        let mut var_guard111_rdn3: f64 = *var_guard111_rdn3_slot;
        let mut var_guard111_rdn4: f64 = *var_guard111_rdn4_slot;
        let mut var_guard111_rdn5: f64 = *var_guard111_rdn5_slot;
        let mut var_guard111_rdn6: f64 = *var_guard111_rdn6_slot;
        let mut var_guard111_rdn7: f64 = *var_guard111_rdn7_slot;
        let mut var_guard111_rdn8: f64 = *var_guard111_rdn8_slot;
        let mut var_guard111_rdn9: f64 = *var_guard111_rdn9_slot;
        let mut var_guard111_rv: f64 = *var_guard111_rv_slot;
        let mut var_guard112: f64 = *var_guard112_slot;
        let mut var_guard112_db0: f64 = *var_guard112_db0_slot;
        let mut var_guard112_db1: f64 = *var_guard112_db1_slot;
        let mut var_guard112_dn0: f64 = *var_guard112_dn0_slot;
        let mut var_guard112_dn1: f64 = *var_guard112_dn1_slot;
        let mut var_guard112_dn10: f64 = *var_guard112_dn10_slot;
        let mut var_guard112_dn2: f64 = *var_guard112_dn2_slot;
        let mut var_guard112_dn3: f64 = *var_guard112_dn3_slot;
        let mut var_guard112_dn4: f64 = *var_guard112_dn4_slot;
        let mut var_guard112_dn5: f64 = *var_guard112_dn5_slot;
        let mut var_guard112_dn6: f64 = *var_guard112_dn6_slot;
        let mut var_guard112_dn7: f64 = *var_guard112_dn7_slot;
        let mut var_guard112_dn8: f64 = *var_guard112_dn8_slot;
        let mut var_guard112_dn9: f64 = *var_guard112_dn9_slot;
        let mut var_guard112_rdb0: f64 = *var_guard112_rdb0_slot;
        let mut var_guard112_rdb1: f64 = *var_guard112_rdb1_slot;
        let mut var_guard112_rdn0: f64 = *var_guard112_rdn0_slot;
        let mut var_guard112_rdn1: f64 = *var_guard112_rdn1_slot;
        let mut var_guard112_rdn10: f64 = *var_guard112_rdn10_slot;
        let mut var_guard112_rdn2: f64 = *var_guard112_rdn2_slot;
        let mut var_guard112_rdn3: f64 = *var_guard112_rdn3_slot;
        let mut var_guard112_rdn4: f64 = *var_guard112_rdn4_slot;
        let mut var_guard112_rdn5: f64 = *var_guard112_rdn5_slot;
        let mut var_guard112_rdn6: f64 = *var_guard112_rdn6_slot;
        let mut var_guard112_rdn7: f64 = *var_guard112_rdn7_slot;
        let mut var_guard112_rdn8: f64 = *var_guard112_rdn8_slot;
        let mut var_guard112_rdn9: f64 = *var_guard112_rdn9_slot;
        let mut var_guard112_rv: f64 = *var_guard112_rv_slot;
        let mut var_guard113: f64 = *var_guard113_slot;
        let mut var_guard113_db0: f64 = *var_guard113_db0_slot;
        let mut var_guard113_db1: f64 = *var_guard113_db1_slot;
        let mut var_guard113_dn0: f64 = *var_guard113_dn0_slot;
        let mut var_guard113_dn1: f64 = *var_guard113_dn1_slot;
        let mut var_guard113_dn10: f64 = *var_guard113_dn10_slot;
        let mut var_guard113_dn2: f64 = *var_guard113_dn2_slot;
        let mut var_guard113_dn3: f64 = *var_guard113_dn3_slot;
        let mut var_guard113_dn4: f64 = *var_guard113_dn4_slot;
        let mut var_guard113_dn5: f64 = *var_guard113_dn5_slot;
        let mut var_guard113_dn6: f64 = *var_guard113_dn6_slot;
        let mut var_guard113_dn7: f64 = *var_guard113_dn7_slot;
        let mut var_guard113_dn8: f64 = *var_guard113_dn8_slot;
        let mut var_guard113_dn9: f64 = *var_guard113_dn9_slot;
        let mut var_guard113_rdb0: f64 = *var_guard113_rdb0_slot;
        let mut var_guard113_rdb1: f64 = *var_guard113_rdb1_slot;
        let mut var_guard113_rdn0: f64 = *var_guard113_rdn0_slot;
        let mut var_guard113_rdn1: f64 = *var_guard113_rdn1_slot;
        let mut var_guard113_rdn10: f64 = *var_guard113_rdn10_slot;
        let mut var_guard113_rdn2: f64 = *var_guard113_rdn2_slot;
        let mut var_guard113_rdn3: f64 = *var_guard113_rdn3_slot;
        let mut var_guard113_rdn4: f64 = *var_guard113_rdn4_slot;
        let mut var_guard113_rdn5: f64 = *var_guard113_rdn5_slot;
        let mut var_guard113_rdn6: f64 = *var_guard113_rdn6_slot;
        let mut var_guard113_rdn7: f64 = *var_guard113_rdn7_slot;
        let mut var_guard113_rdn8: f64 = *var_guard113_rdn8_slot;
        let mut var_guard113_rdn9: f64 = *var_guard113_rdn9_slot;
        let mut var_guard113_rv: f64 = *var_guard113_rv_slot;
        let mut var_vb2e1vfe: f64 = *var_vb2e1vfe_slot;
        let mut var_vb2e1vfe_db0: f64 = *var_vb2e1vfe_db0_slot;
        let mut var_vb2e1vfe_db1: f64 = *var_vb2e1vfe_db1_slot;
        let mut var_vb2e1vfe_dn0: f64 = *var_vb2e1vfe_dn0_slot;
        let mut var_vb2e1vfe_dn1: f64 = *var_vb2e1vfe_dn1_slot;
        let mut var_vb2e1vfe_dn10: f64 = *var_vb2e1vfe_dn10_slot;
        let mut var_vb2e1vfe_dn2: f64 = *var_vb2e1vfe_dn2_slot;
        let mut var_vb2e1vfe_dn3: f64 = *var_vb2e1vfe_dn3_slot;
        let mut var_vb2e1vfe_dn4: f64 = *var_vb2e1vfe_dn4_slot;
        let mut var_vb2e1vfe_dn5: f64 = *var_vb2e1vfe_dn5_slot;
        let mut var_vb2e1vfe_dn6: f64 = *var_vb2e1vfe_dn6_slot;
        let mut var_vb2e1vfe_dn7: f64 = *var_vb2e1vfe_dn7_slot;
        let mut var_vb2e1vfe_dn8: f64 = *var_vb2e1vfe_dn8_slot;
        let mut var_vb2e1vfe_dn9: f64 = *var_vb2e1vfe_dn9_slot;
        let mut var_vb2e1vfe_rdb0: f64 = *var_vb2e1vfe_rdb0_slot;
        let mut var_vb2e1vfe_rdb1: f64 = *var_vb2e1vfe_rdb1_slot;
        let mut var_vb2e1vfe_rdn0: f64 = *var_vb2e1vfe_rdn0_slot;
        let mut var_vb2e1vfe_rdn1: f64 = *var_vb2e1vfe_rdn1_slot;
        let mut var_vb2e1vfe_rdn10: f64 = *var_vb2e1vfe_rdn10_slot;
        let mut var_vb2e1vfe_rdn2: f64 = *var_vb2e1vfe_rdn2_slot;
        let mut var_vb2e1vfe_rdn3: f64 = *var_vb2e1vfe_rdn3_slot;
        let mut var_vb2e1vfe_rdn4: f64 = *var_vb2e1vfe_rdn4_slot;
        let mut var_vb2e1vfe_rdn5: f64 = *var_vb2e1vfe_rdn5_slot;
        let mut var_vb2e1vfe_rdn6: f64 = *var_vb2e1vfe_rdn6_slot;
        let mut var_vb2e1vfe_rdn7: f64 = *var_vb2e1vfe_rdn7_slot;
        let mut var_vb2e1vfe_rdn8: f64 = *var_vb2e1vfe_rdn8_slot;
        let mut var_vb2e1vfe_rdn9: f64 = *var_vb2e1vfe_rdn9_slot;
        let mut var_vb2e1vfe_rv: f64 = *var_vb2e1vfe_rv_slot;
        let mut var_xqex: f64 = *var_xqex_slot;
        let mut var_xqex_db0: f64 = *var_xqex_db0_slot;
        let mut var_xqex_db1: f64 = *var_xqex_db1_slot;
        let mut var_xqex_dn0: f64 = *var_xqex_dn0_slot;
        let mut var_xqex_dn1: f64 = *var_xqex_dn1_slot;
        let mut var_xqex_dn10: f64 = *var_xqex_dn10_slot;
        let mut var_xqex_dn2: f64 = *var_xqex_dn2_slot;
        let mut var_xqex_dn3: f64 = *var_xqex_dn3_slot;
        let mut var_xqex_dn4: f64 = *var_xqex_dn4_slot;
        let mut var_xqex_dn5: f64 = *var_xqex_dn5_slot;
        let mut var_xqex_dn6: f64 = *var_xqex_dn6_slot;
        let mut var_xqex_dn7: f64 = *var_xqex_dn7_slot;
        let mut var_xqex_dn8: f64 = *var_xqex_dn8_slot;
        let mut var_xqex_dn9: f64 = *var_xqex_dn9_slot;
        let mut var_xqex_rdb0: f64 = *var_xqex_rdb0_slot;
        let mut var_xqex_rdb1: f64 = *var_xqex_rdb1_slot;
        let mut var_xqex_rdn0: f64 = *var_xqex_rdn0_slot;
        let mut var_xqex_rdn1: f64 = *var_xqex_rdn1_slot;
        let mut var_xqex_rdn10: f64 = *var_xqex_rdn10_slot;
        let mut var_xqex_rdn2: f64 = *var_xqex_rdn2_slot;
        let mut var_xqex_rdn3: f64 = *var_xqex_rdn3_slot;
        let mut var_xqex_rdn4: f64 = *var_xqex_rdn4_slot;
        let mut var_xqex_rdn5: f64 = *var_xqex_rdn5_slot;
        let mut var_xqex_rdn6: f64 = *var_xqex_rdn6_slot;
        let mut var_xqex_rdn7: f64 = *var_xqex_rdn7_slot;
        let mut var_xqex_rdn8: f64 = *var_xqex_rdn8_slot;
        let mut var_xqex_rdn9: f64 = *var_xqex_rdn9_slot;
        let mut var_xqex_rv: f64 = *var_xqex_rv_slot;
        let mut var_xqmex: f64 = *var_xqmex_slot;
        let mut var_xqmex_db0: f64 = *var_xqmex_db0_slot;
        let mut var_xqmex_db1: f64 = *var_xqmex_db1_slot;
        let mut var_xqmex_dn0: f64 = *var_xqmex_dn0_slot;
        let mut var_xqmex_dn1: f64 = *var_xqmex_dn1_slot;
        let mut var_xqmex_dn10: f64 = *var_xqmex_dn10_slot;
        let mut var_xqmex_dn2: f64 = *var_xqmex_dn2_slot;
        let mut var_xqmex_dn3: f64 = *var_xqmex_dn3_slot;
        let mut var_xqmex_dn4: f64 = *var_xqmex_dn4_slot;
        let mut var_xqmex_dn5: f64 = *var_xqmex_dn5_slot;
        let mut var_xqmex_dn6: f64 = *var_xqmex_dn6_slot;
        let mut var_xqmex_dn7: f64 = *var_xqmex_dn7_slot;
        let mut var_xqmex_dn8: f64 = *var_xqmex_dn8_slot;
        let mut var_xqmex_dn9: f64 = *var_xqmex_dn9_slot;
        let mut var_xqmex_rdb0: f64 = *var_xqmex_rdb0_slot;
        let mut var_xqmex_rdb1: f64 = *var_xqmex_rdb1_slot;
        let mut var_xqmex_rdn0: f64 = *var_xqmex_rdn0_slot;
        let mut var_xqmex_rdn1: f64 = *var_xqmex_rdn1_slot;
        let mut var_xqmex_rdn10: f64 = *var_xqmex_rdn10_slot;
        let mut var_xqmex_rdn2: f64 = *var_xqmex_rdn2_slot;
        let mut var_xqmex_rdn3: f64 = *var_xqmex_rdn3_slot;
        let mut var_xqmex_rdn4: f64 = *var_xqmex_rdn4_slot;
        let mut var_xqmex_rdn5: f64 = *var_xqmex_rdn5_slot;
        let mut var_xqmex_rdn6: f64 = *var_xqmex_rdn6_slot;
        let mut var_xqmex_rdn7: f64 = *var_xqmex_rdn7_slot;
        let mut var_xqmex_rdn8: f64 = *var_xqmex_rdn8_slot;
        let mut var_xqmex_rdn9: f64 = *var_xqmex_rdn9_slot;
        let mut var_xqmex_rv: f64 = *var_xqmex_rv_slot;

        let assign6090_e6183: f64 = (var_vbc3 - var_vdcex_t);
        let assign6090_e6185: f64 = (assign6090_e6183 * var_vtinv);
        let assign6090_e6187: f64 = if assign6090_e6185 < p.p134 { 1.0 } else { 0.0 };
        var_guard111 = assign6090_e6187;
        var_guard111_dn0 = 0.0;
        var_guard111_dn1 = 0.0;
        var_guard111_dn2 = 0.0;
        var_guard111_dn3 = 0.0;
        var_guard111_dn4 = 0.0;
        var_guard111_dn5 = 0.0;
        var_guard111_dn6 = 0.0;
        var_guard111_dn7 = 0.0;
        var_guard111_dn8 = 0.0;
        var_guard111_dn9 = 0.0;
        var_guard111_dn10 = 0.0;
        var_guard111_db0 = 0.0;
        var_guard111_db1 = 0.0;
        var_guard111_rv = 0.0;
        var_guard111_rdn0 = 0.0;
        var_guard111_rdn1 = 0.0;
        var_guard111_rdn2 = 0.0;
        var_guard111_rdn3 = 0.0;
        var_guard111_rdn4 = 0.0;
        var_guard111_rdn5 = 0.0;
        var_guard111_rdn6 = 0.0;
        var_guard111_rdn7 = 0.0;
        var_guard111_rdn8 = 0.0;
        var_guard111_rdn9 = 0.0;
        var_guard111_rdn10 = 0.0;
        var_guard111_rdb0 = 0.0;
        var_guard111_rdb1 = 0.0;

        let (assign6100_e6201, assign6100_e6201_d_n0, assign6100_e6201_d_n1, assign6100_e6201_d_n2, assign6100_e6201_d_n3, assign6100_e6201_d_n4, assign6100_e6201_d_n5, assign6100_e6201_d_n6, assign6100_e6201_d_n7, assign6100_e6201_d_n8, assign6100_e6201_d_n9, assign6100_e6201_d_n10, assign6100_e6201_d_b0, assign6100_e6201_d_b1,) = {
    if (((var_guard109 != 0.0) && (var_guard110 == 0.0)) && (var_guard111 != 0.0)) {
        let assign6100_e6196: f64 = (var_vbc3 - var_vdcex_t);
        let assign6100_e6198: f64 = (assign6100_e6196 * var_vtinv);
        let assign6100_e6199: f64 = (assign6100_e6198).exp();
        (assign6100_e6199, (assign6100_e6199 * (((var_vbc3_dn0 - var_vdcex_t_dn0) * var_vtinv) + (assign6100_e6196 * var_vtinv_dn0))), (assign6100_e6199 * (((var_vbc3_dn1 - var_vdcex_t_dn1) * var_vtinv) + (assign6100_e6196 * var_vtinv_dn1))), (assign6100_e6199 * (((var_vbc3_dn2 - var_vdcex_t_dn2) * var_vtinv) + (assign6100_e6196 * var_vtinv_dn2))), (assign6100_e6199 * (((var_vbc3_dn3 - var_vdcex_t_dn3) * var_vtinv) + (assign6100_e6196 * var_vtinv_dn3))), (assign6100_e6199 * (((var_vbc3_dn4 - var_vdcex_t_dn4) * var_vtinv) + (assign6100_e6196 * var_vtinv_dn4))), (assign6100_e6199 * (((var_vbc3_dn5 - var_vdcex_t_dn5) * var_vtinv) + (assign6100_e6196 * var_vtinv_dn5))), (assign6100_e6199 * (((var_vbc3_dn6 - var_vdcex_t_dn6) * var_vtinv) + (assign6100_e6196 * var_vtinv_dn6))), (assign6100_e6199 * (((var_vbc3_dn7 - var_vdcex_t_dn7) * var_vtinv) + (assign6100_e6196 * var_vtinv_dn7))), (assign6100_e6199 * (((var_vbc3_dn8 - var_vdcex_t_dn8) * var_vtinv) + (assign6100_e6196 * var_vtinv_dn8))), (assign6100_e6199 * (((var_vbc3_dn9 - var_vdcex_t_dn9) * var_vtinv) + (assign6100_e6196 * var_vtinv_dn9))), (assign6100_e6199 * (((var_vbc3_dn10 - var_vdcex_t_dn10) * var_vtinv) + (assign6100_e6196 * var_vtinv_dn10))), (assign6100_e6199 * (((var_vbc3_db0 - var_vdcex_t_db0) * var_vtinv) + (assign6100_e6196 * var_vtinv_db0))), (assign6100_e6199 * (((var_vbc3_db1 - var_vdcex_t_db1) * var_vtinv) + (assign6100_e6196 * var_vtinv_db1))),)
    } else {
        (var_evbc3vdcex, var_evbc3vdcex_dn0, var_evbc3vdcex_dn1, var_evbc3vdcex_dn2, var_evbc3vdcex_dn3, var_evbc3vdcex_dn4, var_evbc3vdcex_dn5, var_evbc3vdcex_dn6, var_evbc3vdcex_dn7, var_evbc3vdcex_dn8, var_evbc3vdcex_dn9, var_evbc3vdcex_dn10, var_evbc3vdcex_db0, var_evbc3vdcex_db1,)
    }
};
        var_evbc3vdcex = assign6100_e6201;
        var_evbc3vdcex_dn0 = assign6100_e6201_d_n0;
        var_evbc3vdcex_dn1 = assign6100_e6201_d_n1;
        var_evbc3vdcex_dn2 = assign6100_e6201_d_n2;
        var_evbc3vdcex_dn3 = assign6100_e6201_d_n3;
        var_evbc3vdcex_dn4 = assign6100_e6201_d_n4;
        var_evbc3vdcex_dn5 = assign6100_e6201_d_n5;
        var_evbc3vdcex_dn6 = assign6100_e6201_d_n6;
        var_evbc3vdcex_dn7 = assign6100_e6201_d_n7;
        var_evbc3vdcex_dn8 = assign6100_e6201_d_n8;
        var_evbc3vdcex_dn9 = assign6100_e6201_d_n9;
        var_evbc3vdcex_dn10 = assign6100_e6201_d_n10;
        var_evbc3vdcex_db0 = assign6100_e6201_d_b0;
        var_evbc3vdcex_db1 = assign6100_e6201_d_b1;
        var_evbc3vdcex_rv = 0.0;
        var_evbc3vdcex_rdn0 = 0.0;
        var_evbc3vdcex_rdn1 = 0.0;
        var_evbc3vdcex_rdn2 = 0.0;
        var_evbc3vdcex_rdn3 = 0.0;
        var_evbc3vdcex_rdn4 = 0.0;
        var_evbc3vdcex_rdn5 = 0.0;
        var_evbc3vdcex_rdn6 = 0.0;
        var_evbc3vdcex_rdn7 = 0.0;
        var_evbc3vdcex_rdn8 = 0.0;
        var_evbc3vdcex_rdn9 = 0.0;
        var_evbc3vdcex_rdn10 = 0.0;
        var_evbc3vdcex_rdb0 = 0.0;
        var_evbc3vdcex_rdb1 = 0.0;

        let (assign6110_e6212, assign6110_e6212_d_n0, assign6110_e6212_d_n1, assign6110_e6212_d_n2, assign6110_e6212_d_n3, assign6110_e6212_d_n4, assign6110_e6212_d_n5, assign6110_e6212_d_n6, assign6110_e6212_d_n7, assign6110_e6212_d_n8, assign6110_e6212_d_n9, assign6110_e6212_d_n10, assign6110_e6212_d_b0, assign6110_e6212_d_b1,) = {
    if (((var_guard109 != 0.0) && (var_guard110 == 0.0)) && (var_guard111 == 0.0)) {
        let assign6110_e6210: f64 = (p.p134).exp();
        (assign6110_e6210, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_expl, var_expl_dn0, var_expl_dn1, var_expl_dn2, var_expl_dn3, var_expl_dn4, var_expl_dn5, var_expl_dn6, var_expl_dn7, var_expl_dn8, var_expl_dn9, var_expl_dn10, var_expl_db0, var_expl_db1,)
    }
};
        var_expl = assign6110_e6212;
        var_expl_dn0 = assign6110_e6212_d_n0;
        var_expl_dn1 = assign6110_e6212_d_n1;
        var_expl_dn2 = assign6110_e6212_d_n2;
        var_expl_dn3 = assign6110_e6212_d_n3;
        var_expl_dn4 = assign6110_e6212_d_n4;
        var_expl_dn5 = assign6110_e6212_d_n5;
        var_expl_dn6 = assign6110_e6212_d_n6;
        var_expl_dn7 = assign6110_e6212_d_n7;
        var_expl_dn8 = assign6110_e6212_d_n8;
        var_expl_dn9 = assign6110_e6212_d_n9;
        var_expl_dn10 = assign6110_e6212_d_n10;
        var_expl_db0 = assign6110_e6212_d_b0;
        var_expl_db1 = assign6110_e6212_d_b1;
        var_expl_rv = 0.0;
        var_expl_rdn0 = 0.0;
        var_expl_rdn1 = 0.0;
        var_expl_rdn2 = 0.0;
        var_expl_rdn3 = 0.0;
        var_expl_rdn4 = 0.0;
        var_expl_rdn5 = 0.0;
        var_expl_rdn6 = 0.0;
        var_expl_rdn7 = 0.0;
        var_expl_rdn8 = 0.0;
        var_expl_rdn9 = 0.0;
        var_expl_rdn10 = 0.0;
        var_expl_rdb0 = 0.0;
        var_expl_rdb1 = 0.0;

        let (assign6120_e6232, assign6120_e6232_d_n0, assign6120_e6232_d_n1, assign6120_e6232_d_n2, assign6120_e6232_d_n3, assign6120_e6232_d_n4, assign6120_e6232_d_n5, assign6120_e6232_d_n6, assign6120_e6232_d_n7, assign6120_e6232_d_n8, assign6120_e6232_d_n9, assign6120_e6232_d_n10, assign6120_e6232_d_b0, assign6120_e6232_d_b1,) = {
    if (((var_guard109 != 0.0) && (var_guard110 == 0.0)) && (var_guard111 == 0.0)) {
        let assign6120_e6224: f64 = (var_vbc3 - var_vdcex_t);
        let assign6120_e6226: f64 = (assign6120_e6224 * var_vtinv);
        let assign6120_e6228: f64 = (assign6120_e6226 - p.p134);
        let assign6120_e6229: f64 = (1.0 + assign6120_e6228);
        let assign6120_e6230: f64 = (var_expl * assign6120_e6229);
        (assign6120_e6230, ((var_expl_dn0 * assign6120_e6229) + (var_expl * (((var_vbc3_dn0 - var_vdcex_t_dn0) * var_vtinv) + (assign6120_e6224 * var_vtinv_dn0)))), ((var_expl_dn1 * assign6120_e6229) + (var_expl * (((var_vbc3_dn1 - var_vdcex_t_dn1) * var_vtinv) + (assign6120_e6224 * var_vtinv_dn1)))), ((var_expl_dn2 * assign6120_e6229) + (var_expl * (((var_vbc3_dn2 - var_vdcex_t_dn2) * var_vtinv) + (assign6120_e6224 * var_vtinv_dn2)))), ((var_expl_dn3 * assign6120_e6229) + (var_expl * (((var_vbc3_dn3 - var_vdcex_t_dn3) * var_vtinv) + (assign6120_e6224 * var_vtinv_dn3)))), ((var_expl_dn4 * assign6120_e6229) + (var_expl * (((var_vbc3_dn4 - var_vdcex_t_dn4) * var_vtinv) + (assign6120_e6224 * var_vtinv_dn4)))), ((var_expl_dn5 * assign6120_e6229) + (var_expl * (((var_vbc3_dn5 - var_vdcex_t_dn5) * var_vtinv) + (assign6120_e6224 * var_vtinv_dn5)))), ((var_expl_dn6 * assign6120_e6229) + (var_expl * (((var_vbc3_dn6 - var_vdcex_t_dn6) * var_vtinv) + (assign6120_e6224 * var_vtinv_dn6)))), ((var_expl_dn7 * assign6120_e6229) + (var_expl * (((var_vbc3_dn7 - var_vdcex_t_dn7) * var_vtinv) + (assign6120_e6224 * var_vtinv_dn7)))), ((var_expl_dn8 * assign6120_e6229) + (var_expl * (((var_vbc3_dn8 - var_vdcex_t_dn8) * var_vtinv) + (assign6120_e6224 * var_vtinv_dn8)))), ((var_expl_dn9 * assign6120_e6229) + (var_expl * (((var_vbc3_dn9 - var_vdcex_t_dn9) * var_vtinv) + (assign6120_e6224 * var_vtinv_dn9)))), ((var_expl_dn10 * assign6120_e6229) + (var_expl * (((var_vbc3_dn10 - var_vdcex_t_dn10) * var_vtinv) + (assign6120_e6224 * var_vtinv_dn10)))), ((var_expl_db0 * assign6120_e6229) + (var_expl * (((var_vbc3_db0 - var_vdcex_t_db0) * var_vtinv) + (assign6120_e6224 * var_vtinv_db0)))), ((var_expl_db1 * assign6120_e6229) + (var_expl * (((var_vbc3_db1 - var_vdcex_t_db1) * var_vtinv) + (assign6120_e6224 * var_vtinv_db1)))),)
    } else {
        (var_evbc3vdcex, var_evbc3vdcex_dn0, var_evbc3vdcex_dn1, var_evbc3vdcex_dn2, var_evbc3vdcex_dn3, var_evbc3vdcex_dn4, var_evbc3vdcex_dn5, var_evbc3vdcex_dn6, var_evbc3vdcex_dn7, var_evbc3vdcex_dn8, var_evbc3vdcex_dn9, var_evbc3vdcex_dn10, var_evbc3vdcex_db0, var_evbc3vdcex_db1,)
    }
};
        var_evbc3vdcex = assign6120_e6232;
        var_evbc3vdcex_dn0 = assign6120_e6232_d_n0;
        var_evbc3vdcex_dn1 = assign6120_e6232_d_n1;
        var_evbc3vdcex_dn2 = assign6120_e6232_d_n2;
        var_evbc3vdcex_dn3 = assign6120_e6232_d_n3;
        var_evbc3vdcex_dn4 = assign6120_e6232_d_n4;
        var_evbc3vdcex_dn5 = assign6120_e6232_d_n5;
        var_evbc3vdcex_dn6 = assign6120_e6232_d_n6;
        var_evbc3vdcex_dn7 = assign6120_e6232_d_n7;
        var_evbc3vdcex_dn8 = assign6120_e6232_d_n8;
        var_evbc3vdcex_dn9 = assign6120_e6232_d_n9;
        var_evbc3vdcex_dn10 = assign6120_e6232_d_n10;
        var_evbc3vdcex_db0 = assign6120_e6232_d_b0;
        var_evbc3vdcex_db1 = assign6120_e6232_d_b1;
        var_evbc3vdcex_rv = 0.0;
        var_evbc3vdcex_rdn0 = 0.0;
        var_evbc3vdcex_rdn1 = 0.0;
        var_evbc3vdcex_rdn2 = 0.0;
        var_evbc3vdcex_rdn3 = 0.0;
        var_evbc3vdcex_rdn4 = 0.0;
        var_evbc3vdcex_rdn5 = 0.0;
        var_evbc3vdcex_rdn6 = 0.0;
        var_evbc3vdcex_rdn7 = 0.0;
        var_evbc3vdcex_rdn8 = 0.0;
        var_evbc3vdcex_rdn9 = 0.0;
        var_evbc3vdcex_rdn10 = 0.0;
        var_evbc3vdcex_rdb0 = 0.0;
        var_evbc3vdcex_rdb1 = 0.0;

        let (assign6130_e6256, assign6130_e6256_d_n0, assign6130_e6256_d_n1, assign6130_e6256_d_n2, assign6130_e6256_d_n3, assign6130_e6256_d_n4, assign6130_e6256_d_n5, assign6130_e6256_d_n6, assign6130_e6256_d_n7, assign6130_e6256_d_n8, assign6130_e6256_d_n9, assign6130_e6256_d_n10, assign6130_e6256_d_b0, assign6130_e6256_d_b1,) = {
    if ((var_guard109 != 0.0) && (var_guard110 == 0.0)) {
        let assign6130_e6239: f64 = (2.0 * p.p32);
        let assign6130_e6241: f64 = (assign6130_e6239 * var_ibx_t);
        let assign6130_e6243: f64 = (assign6130_e6241 * var_tauex_t);
        let assign6130_e6245: f64 = (assign6130_e6243 * var_evbc3);
        let assign6130_e6250: f64 = (4.0 * var_evbc3vdcex);
        let assign6130_e6251: f64 = (1.0 + assign6130_e6250);
        let assign6130_e6252: f64 = (assign6130_e6251).sqrt();
        let assign6130_e6253: f64 = (1.0 + assign6130_e6252);
        let assign6130_e6254: f64 = (assign6130_e6245 / assign6130_e6253);
        (assign6130_e6254, ((((((((assign6130_e6239 * var_ibx_t_dn0) * var_tauex_t) + (assign6130_e6241 * var_tauex_t_dn0)) * var_evbc3) + (assign6130_e6243 * var_evbc3_dn0)) * assign6130_e6253) - (assign6130_e6245 * ((4.0 * var_evbc3vdcex_dn0) / (2.0 * assign6130_e6252)))) / (assign6130_e6253 * assign6130_e6253)), ((((((((assign6130_e6239 * var_ibx_t_dn1) * var_tauex_t) + (assign6130_e6241 * var_tauex_t_dn1)) * var_evbc3) + (assign6130_e6243 * var_evbc3_dn1)) * assign6130_e6253) - (assign6130_e6245 * ((4.0 * var_evbc3vdcex_dn1) / (2.0 * assign6130_e6252)))) / (assign6130_e6253 * assign6130_e6253)), ((((((((assign6130_e6239 * var_ibx_t_dn2) * var_tauex_t) + (assign6130_e6241 * var_tauex_t_dn2)) * var_evbc3) + (assign6130_e6243 * var_evbc3_dn2)) * assign6130_e6253) - (assign6130_e6245 * ((4.0 * var_evbc3vdcex_dn2) / (2.0 * assign6130_e6252)))) / (assign6130_e6253 * assign6130_e6253)), ((((((((assign6130_e6239 * var_ibx_t_dn3) * var_tauex_t) + (assign6130_e6241 * var_tauex_t_dn3)) * var_evbc3) + (assign6130_e6243 * var_evbc3_dn3)) * assign6130_e6253) - (assign6130_e6245 * ((4.0 * var_evbc3vdcex_dn3) / (2.0 * assign6130_e6252)))) / (assign6130_e6253 * assign6130_e6253)), ((((((((assign6130_e6239 * var_ibx_t_dn4) * var_tauex_t) + (assign6130_e6241 * var_tauex_t_dn4)) * var_evbc3) + (assign6130_e6243 * var_evbc3_dn4)) * assign6130_e6253) - (assign6130_e6245 * ((4.0 * var_evbc3vdcex_dn4) / (2.0 * assign6130_e6252)))) / (assign6130_e6253 * assign6130_e6253)), ((((((((assign6130_e6239 * var_ibx_t_dn5) * var_tauex_t) + (assign6130_e6241 * var_tauex_t_dn5)) * var_evbc3) + (assign6130_e6243 * var_evbc3_dn5)) * assign6130_e6253) - (assign6130_e6245 * ((4.0 * var_evbc3vdcex_dn5) / (2.0 * assign6130_e6252)))) / (assign6130_e6253 * assign6130_e6253)), ((((((((assign6130_e6239 * var_ibx_t_dn6) * var_tauex_t) + (assign6130_e6241 * var_tauex_t_dn6)) * var_evbc3) + (assign6130_e6243 * var_evbc3_dn6)) * assign6130_e6253) - (assign6130_e6245 * ((4.0 * var_evbc3vdcex_dn6) / (2.0 * assign6130_e6252)))) / (assign6130_e6253 * assign6130_e6253)), ((((((((assign6130_e6239 * var_ibx_t_dn7) * var_tauex_t) + (assign6130_e6241 * var_tauex_t_dn7)) * var_evbc3) + (assign6130_e6243 * var_evbc3_dn7)) * assign6130_e6253) - (assign6130_e6245 * ((4.0 * var_evbc3vdcex_dn7) / (2.0 * assign6130_e6252)))) / (assign6130_e6253 * assign6130_e6253)), ((((((((assign6130_e6239 * var_ibx_t_dn8) * var_tauex_t) + (assign6130_e6241 * var_tauex_t_dn8)) * var_evbc3) + (assign6130_e6243 * var_evbc3_dn8)) * assign6130_e6253) - (assign6130_e6245 * ((4.0 * var_evbc3vdcex_dn8) / (2.0 * assign6130_e6252)))) / (assign6130_e6253 * assign6130_e6253)), ((((((((assign6130_e6239 * var_ibx_t_dn9) * var_tauex_t) + (assign6130_e6241 * var_tauex_t_dn9)) * var_evbc3) + (assign6130_e6243 * var_evbc3_dn9)) * assign6130_e6253) - (assign6130_e6245 * ((4.0 * var_evbc3vdcex_dn9) / (2.0 * assign6130_e6252)))) / (assign6130_e6253 * assign6130_e6253)), ((((((((assign6130_e6239 * var_ibx_t_dn10) * var_tauex_t) + (assign6130_e6241 * var_tauex_t_dn10)) * var_evbc3) + (assign6130_e6243 * var_evbc3_dn10)) * assign6130_e6253) - (assign6130_e6245 * ((4.0 * var_evbc3vdcex_dn10) / (2.0 * assign6130_e6252)))) / (assign6130_e6253 * assign6130_e6253)), ((((((((assign6130_e6239 * var_ibx_t_db0) * var_tauex_t) + (assign6130_e6241 * var_tauex_t_db0)) * var_evbc3) + (assign6130_e6243 * var_evbc3_db0)) * assign6130_e6253) - (assign6130_e6245 * ((4.0 * var_evbc3vdcex_db0) / (2.0 * assign6130_e6252)))) / (assign6130_e6253 * assign6130_e6253)), ((((((((assign6130_e6239 * var_ibx_t_db1) * var_tauex_t) + (assign6130_e6241 * var_tauex_t_db1)) * var_evbc3) + (assign6130_e6243 * var_evbc3_db1)) * assign6130_e6253) - (assign6130_e6245 * ((4.0 * var_evbc3vdcex_db1) / (2.0 * assign6130_e6252)))) / (assign6130_e6253 * assign6130_e6253)),)
    } else {
        (var_xqmex, var_xqmex_dn0, var_xqmex_dn1, var_xqmex_dn2, var_xqmex_dn3, var_xqmex_dn4, var_xqmex_dn5, var_xqmex_dn6, var_xqmex_dn7, var_xqmex_dn8, var_xqmex_dn9, var_xqmex_dn10, var_xqmex_db0, var_xqmex_db1,)
    }
};
        var_xqmex = assign6130_e6256;
        var_xqmex_dn0 = assign6130_e6256_d_n0;
        var_xqmex_dn1 = assign6130_e6256_d_n1;
        var_xqmex_dn2 = assign6130_e6256_d_n2;
        var_xqmex_dn3 = assign6130_e6256_d_n3;
        var_xqmex_dn4 = assign6130_e6256_d_n4;
        var_xqmex_dn5 = assign6130_e6256_d_n5;
        var_xqmex_dn6 = assign6130_e6256_d_n6;
        var_xqmex_dn7 = assign6130_e6256_d_n7;
        var_xqmex_dn8 = assign6130_e6256_d_n8;
        var_xqmex_dn9 = assign6130_e6256_d_n9;
        var_xqmex_dn10 = assign6130_e6256_d_n10;
        var_xqmex_db0 = assign6130_e6256_d_b0;
        var_xqmex_db1 = assign6130_e6256_d_b1;
        var_xqmex_rv = 0.0;
        var_xqmex_rdn0 = 0.0;
        var_xqmex_rdn1 = 0.0;
        var_xqmex_rdn2 = 0.0;
        var_xqmex_rdn3 = 0.0;
        var_xqmex_rdn4 = 0.0;
        var_xqmex_rdn5 = 0.0;
        var_xqmex_rdn6 = 0.0;
        var_xqmex_rdn7 = 0.0;
        var_xqmex_rdn8 = 0.0;
        var_xqmex_rdn9 = 0.0;
        var_xqmex_rdn10 = 0.0;
        var_xqmex_rdb0 = 0.0;
        var_xqmex_rdb1 = 0.0;

        let (assign6140_e6262, assign6140_e6262_d_n0, assign6140_e6262_d_n1, assign6140_e6262_d_n2, assign6140_e6262_d_n3, assign6140_e6262_d_n4, assign6140_e6262_d_n5, assign6140_e6262_d_n6, assign6140_e6262_d_n7, assign6140_e6262_d_n8, assign6140_e6262_d_n9, assign6140_e6262_d_n10, assign6140_e6262_d_b0, assign6140_e6262_d_b1,) = {
    if (var_guard109 != 0.0) {
        let assign6140_e6260: f64 = (var_fex * var_xqmex);
        (assign6140_e6260, ((var_fex_dn0 * var_xqmex) + (var_fex * var_xqmex_dn0)), ((var_fex_dn1 * var_xqmex) + (var_fex * var_xqmex_dn1)), ((var_fex_dn2 * var_xqmex) + (var_fex * var_xqmex_dn2)), ((var_fex_dn3 * var_xqmex) + (var_fex * var_xqmex_dn3)), ((var_fex_dn4 * var_xqmex) + (var_fex * var_xqmex_dn4)), ((var_fex_dn5 * var_xqmex) + (var_fex * var_xqmex_dn5)), ((var_fex_dn6 * var_xqmex) + (var_fex * var_xqmex_dn6)), ((var_fex_dn7 * var_xqmex) + (var_fex * var_xqmex_dn7)), ((var_fex_dn8 * var_xqmex) + (var_fex * var_xqmex_dn8)), ((var_fex_dn9 * var_xqmex) + (var_fex * var_xqmex_dn9)), ((var_fex_dn10 * var_xqmex) + (var_fex * var_xqmex_dn10)), ((var_fex_db0 * var_xqmex) + (var_fex * var_xqmex_db0)), ((var_fex_db1 * var_xqmex) + (var_fex * var_xqmex_db1)),)
    } else {
        (var_xqex, var_xqex_dn0, var_xqex_dn1, var_xqex_dn2, var_xqex_dn3, var_xqex_dn4, var_xqex_dn5, var_xqex_dn6, var_xqex_dn7, var_xqex_dn8, var_xqex_dn9, var_xqex_dn10, var_xqex_db0, var_xqex_db1,)
    }
};
        var_xqex = assign6140_e6262;
        var_xqex_dn0 = assign6140_e6262_d_n0;
        var_xqex_dn1 = assign6140_e6262_d_n1;
        var_xqex_dn2 = assign6140_e6262_d_n2;
        var_xqex_dn3 = assign6140_e6262_d_n3;
        var_xqex_dn4 = assign6140_e6262_d_n4;
        var_xqex_dn5 = assign6140_e6262_d_n5;
        var_xqex_dn6 = assign6140_e6262_d_n6;
        var_xqex_dn7 = assign6140_e6262_d_n7;
        var_xqex_dn8 = assign6140_e6262_d_n8;
        var_xqex_dn9 = assign6140_e6262_d_n9;
        var_xqex_dn10 = assign6140_e6262_d_n10;
        var_xqex_db0 = assign6140_e6262_d_b0;
        var_xqex_db1 = assign6140_e6262_d_b1;
        var_xqex_rv = 0.0;
        var_xqex_rdn0 = 0.0;
        var_xqex_rdn1 = 0.0;
        var_xqex_rdn2 = 0.0;
        var_xqex_rdn3 = 0.0;
        var_xqex_rdn4 = 0.0;
        var_xqex_rdn5 = 0.0;
        var_xqex_rdn6 = 0.0;
        var_xqex_rdn7 = 0.0;
        var_xqex_rdn8 = 0.0;
        var_xqex_rdn9 = 0.0;
        var_xqex_rdn10 = 0.0;
        var_xqex_rdb0 = 0.0;
        var_xqex_rdb1 = 0.0;

        let assign6150_e6265: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        var_guard112 = assign6150_e6265;
        var_guard112_dn0 = 0.0;
        var_guard112_dn1 = 0.0;
        var_guard112_dn2 = 0.0;
        var_guard112_dn3 = 0.0;
        var_guard112_dn4 = 0.0;
        var_guard112_dn5 = 0.0;
        var_guard112_dn6 = 0.0;
        var_guard112_dn7 = 0.0;
        var_guard112_dn8 = 0.0;
        var_guard112_dn9 = 0.0;
        var_guard112_dn10 = 0.0;
        var_guard112_db0 = 0.0;
        var_guard112_db1 = 0.0;
        var_guard112_rv = 0.0;
        var_guard112_rdn0 = 0.0;
        var_guard112_rdn1 = 0.0;
        var_guard112_rdn2 = 0.0;
        var_guard112_rdn3 = 0.0;
        var_guard112_rdn4 = 0.0;
        var_guard112_rdn5 = 0.0;
        var_guard112_rdn6 = 0.0;
        var_guard112_rdn7 = 0.0;
        var_guard112_rdn8 = 0.0;
        var_guard112_rdn9 = 0.0;
        var_guard112_rdn10 = 0.0;
        var_guard112_rdb0 = 0.0;
        var_guard112_rdb1 = 0.0;

        let (assign6160_e6278, assign6160_e6278_d_n0, assign6160_e6278_d_n1, assign6160_e6278_d_n2, assign6160_e6278_d_n3, assign6160_e6278_d_n4, assign6160_e6278_d_n5, assign6160_e6278_d_n6, assign6160_e6278_d_n7, assign6160_e6278_d_n8, assign6160_e6278_d_n9, assign6160_e6278_d_n10, assign6160_e6278_d_b0, assign6160_e6278_d_b1,) = {
    if (var_guard112 != 0.0) {
        let assign6160_e6270: f64 = (var_vje * var_inv_vde_t);
        let assign6160_e6271: f64 = (1.0 - assign6160_e6270);
        let assign6160_e6273: f64 = (-p.p66);
        let assign6160_e6274: f64 = (assign6160_e6271).powf(assign6160_e6273);
        let assign6160_e6276: f64 = (assign6160_e6274 - 3.0);
        (assign6160_e6276, if 0.0 == 0.0 && ((assign6160_e6273) as f64).is_finite() && ((assign6160_e6273) as f64).fract() == 0.0 { if assign6160_e6273 == 0.0 { 0.0 } else { (assign6160_e6273 * ((assign6160_e6271).powf(assign6160_e6273 - 1.0) * (-((var_vje_dn0 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn0))))) } } else { (assign6160_e6274 * (assign6160_e6273 * ((-((var_vje_dn0 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn0))) / assign6160_e6271))) }, if 0.0 == 0.0 && ((assign6160_e6273) as f64).is_finite() && ((assign6160_e6273) as f64).fract() == 0.0 { if assign6160_e6273 == 0.0 { 0.0 } else { (assign6160_e6273 * ((assign6160_e6271).powf(assign6160_e6273 - 1.0) * (-((var_vje_dn1 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn1))))) } } else { (assign6160_e6274 * (assign6160_e6273 * ((-((var_vje_dn1 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn1))) / assign6160_e6271))) }, if 0.0 == 0.0 && ((assign6160_e6273) as f64).is_finite() && ((assign6160_e6273) as f64).fract() == 0.0 { if assign6160_e6273 == 0.0 { 0.0 } else { (assign6160_e6273 * ((assign6160_e6271).powf(assign6160_e6273 - 1.0) * (-((var_vje_dn2 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn2))))) } } else { (assign6160_e6274 * (assign6160_e6273 * ((-((var_vje_dn2 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn2))) / assign6160_e6271))) }, if 0.0 == 0.0 && ((assign6160_e6273) as f64).is_finite() && ((assign6160_e6273) as f64).fract() == 0.0 { if assign6160_e6273 == 0.0 { 0.0 } else { (assign6160_e6273 * ((assign6160_e6271).powf(assign6160_e6273 - 1.0) * (-((var_vje_dn3 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn3))))) } } else { (assign6160_e6274 * (assign6160_e6273 * ((-((var_vje_dn3 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn3))) / assign6160_e6271))) }, if 0.0 == 0.0 && ((assign6160_e6273) as f64).is_finite() && ((assign6160_e6273) as f64).fract() == 0.0 { if assign6160_e6273 == 0.0 { 0.0 } else { (assign6160_e6273 * ((assign6160_e6271).powf(assign6160_e6273 - 1.0) * (-((var_vje_dn4 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn4))))) } } else { (assign6160_e6274 * (assign6160_e6273 * ((-((var_vje_dn4 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn4))) / assign6160_e6271))) }, if 0.0 == 0.0 && ((assign6160_e6273) as f64).is_finite() && ((assign6160_e6273) as f64).fract() == 0.0 { if assign6160_e6273 == 0.0 { 0.0 } else { (assign6160_e6273 * ((assign6160_e6271).powf(assign6160_e6273 - 1.0) * (-((var_vje_dn5 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn5))))) } } else { (assign6160_e6274 * (assign6160_e6273 * ((-((var_vje_dn5 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn5))) / assign6160_e6271))) }, if 0.0 == 0.0 && ((assign6160_e6273) as f64).is_finite() && ((assign6160_e6273) as f64).fract() == 0.0 { if assign6160_e6273 == 0.0 { 0.0 } else { (assign6160_e6273 * ((assign6160_e6271).powf(assign6160_e6273 - 1.0) * (-((var_vje_dn6 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn6))))) } } else { (assign6160_e6274 * (assign6160_e6273 * ((-((var_vje_dn6 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn6))) / assign6160_e6271))) }, if 0.0 == 0.0 && ((assign6160_e6273) as f64).is_finite() && ((assign6160_e6273) as f64).fract() == 0.0 { if assign6160_e6273 == 0.0 { 0.0 } else { (assign6160_e6273 * ((assign6160_e6271).powf(assign6160_e6273 - 1.0) * (-((var_vje_dn7 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn7))))) } } else { (assign6160_e6274 * (assign6160_e6273 * ((-((var_vje_dn7 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn7))) / assign6160_e6271))) }, if 0.0 == 0.0 && ((assign6160_e6273) as f64).is_finite() && ((assign6160_e6273) as f64).fract() == 0.0 { if assign6160_e6273 == 0.0 { 0.0 } else { (assign6160_e6273 * ((assign6160_e6271).powf(assign6160_e6273 - 1.0) * (-((var_vje_dn8 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn8))))) } } else { (assign6160_e6274 * (assign6160_e6273 * ((-((var_vje_dn8 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn8))) / assign6160_e6271))) }, if 0.0 == 0.0 && ((assign6160_e6273) as f64).is_finite() && ((assign6160_e6273) as f64).fract() == 0.0 { if assign6160_e6273 == 0.0 { 0.0 } else { (assign6160_e6273 * ((assign6160_e6271).powf(assign6160_e6273 - 1.0) * (-((var_vje_dn9 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn9))))) } } else { (assign6160_e6274 * (assign6160_e6273 * ((-((var_vje_dn9 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn9))) / assign6160_e6271))) }, if 0.0 == 0.0 && ((assign6160_e6273) as f64).is_finite() && ((assign6160_e6273) as f64).fract() == 0.0 { if assign6160_e6273 == 0.0 { 0.0 } else { (assign6160_e6273 * ((assign6160_e6271).powf(assign6160_e6273 - 1.0) * (-((var_vje_dn10 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn10))))) } } else { (assign6160_e6274 * (assign6160_e6273 * ((-((var_vje_dn10 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn10))) / assign6160_e6271))) }, if 0.0 == 0.0 && ((assign6160_e6273) as f64).is_finite() && ((assign6160_e6273) as f64).fract() == 0.0 { if assign6160_e6273 == 0.0 { 0.0 } else { (assign6160_e6273 * ((assign6160_e6271).powf(assign6160_e6273 - 1.0) * (-((var_vje_db0 * var_inv_vde_t) + (var_vje * var_inv_vde_t_db0))))) } } else { (assign6160_e6274 * (assign6160_e6273 * ((-((var_vje_db0 * var_inv_vde_t) + (var_vje * var_inv_vde_t_db0))) / assign6160_e6271))) }, if 0.0 == 0.0 && ((assign6160_e6273) as f64).is_finite() && ((assign6160_e6273) as f64).fract() == 0.0 { if assign6160_e6273 == 0.0 { 0.0 } else { (assign6160_e6273 * ((assign6160_e6271).powf(assign6160_e6273 - 1.0) * (-((var_vje_db1 * var_inv_vde_t) + (var_vje * var_inv_vde_t_db1))))) } } else { (assign6160_e6274 * (assign6160_e6273 * ((-((var_vje_db1 * var_inv_vde_t) + (var_vje * var_inv_vde_t_db1))) / assign6160_e6271))) },)
    } else {
        (var_dvtevje, var_dvtevje_dn0, var_dvtevje_dn1, var_dvtevje_dn2, var_dvtevje_dn3, var_dvtevje_dn4, var_dvtevje_dn5, var_dvtevje_dn6, var_dvtevje_dn7, var_dvtevje_dn8, var_dvtevje_dn9, var_dvtevje_dn10, var_dvtevje_db0, var_dvtevje_db1,)
    }
};
        var_dvtevje = assign6160_e6278;
        var_dvtevje_dn0 = assign6160_e6278_d_n0;
        var_dvtevje_dn1 = assign6160_e6278_d_n1;
        var_dvtevje_dn2 = assign6160_e6278_d_n2;
        var_dvtevje_dn3 = assign6160_e6278_d_n3;
        var_dvtevje_dn4 = assign6160_e6278_d_n4;
        var_dvtevje_dn5 = assign6160_e6278_d_n5;
        var_dvtevje_dn6 = assign6160_e6278_d_n6;
        var_dvtevje_dn7 = assign6160_e6278_d_n7;
        var_dvtevje_dn8 = assign6160_e6278_d_n8;
        var_dvtevje_dn9 = assign6160_e6278_d_n9;
        var_dvtevje_dn10 = assign6160_e6278_d_n10;
        var_dvtevje_db0 = assign6160_e6278_d_b0;
        var_dvtevje_db1 = assign6160_e6278_d_b1;
        var_dvtevje_rv = 0.0;
        var_dvtevje_rdn0 = 0.0;
        var_dvtevje_rdn1 = 0.0;
        var_dvtevje_rdn2 = 0.0;
        var_dvtevje_rdn3 = 0.0;
        var_dvtevje_rdn4 = 0.0;
        var_dvtevje_rdn5 = 0.0;
        var_dvtevje_rdn6 = 0.0;
        var_dvtevje_rdn7 = 0.0;
        var_dvtevje_rdn8 = 0.0;
        var_dvtevje_rdn9 = 0.0;
        var_dvtevje_rdn10 = 0.0;
        var_dvtevje_rdb0 = 0.0;
        var_dvtevje_rdb1 = 0.0;

        let (assign6170_e6286, assign6170_e6286_d_n0, assign6170_e6286_d_n1, assign6170_e6286_d_n2, assign6170_e6286_d_n3, assign6170_e6286_d_n4, assign6170_e6286_d_n5, assign6170_e6286_d_n6, assign6170_e6286_d_n7, assign6170_e6286_d_n8, assign6170_e6286_d_n9, assign6170_e6286_d_n10, assign6170_e6286_d_b0, assign6170_e6286_d_b1,) = {
    if (var_guard112 != 0.0) {
        let assign6170_e6282: f64 = (var_vb2e1 - var_vfe);
        let assign6170_e6284: f64 = (assign6170_e6282 / var_a_vde);
        (assign6170_e6284, ((((var_vb2e1_dn0 - var_vfe_dn0) * var_a_vde) - (assign6170_e6282 * var_a_vde_dn0)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn1 - var_vfe_dn1) * var_a_vde) - (assign6170_e6282 * var_a_vde_dn1)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn2 - var_vfe_dn2) * var_a_vde) - (assign6170_e6282 * var_a_vde_dn2)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn3 - var_vfe_dn3) * var_a_vde) - (assign6170_e6282 * var_a_vde_dn3)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn4 - var_vfe_dn4) * var_a_vde) - (assign6170_e6282 * var_a_vde_dn4)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn5 - var_vfe_dn5) * var_a_vde) - (assign6170_e6282 * var_a_vde_dn5)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn6 - var_vfe_dn6) * var_a_vde) - (assign6170_e6282 * var_a_vde_dn6)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn7 - var_vfe_dn7) * var_a_vde) - (assign6170_e6282 * var_a_vde_dn7)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn8 - var_vfe_dn8) * var_a_vde) - (assign6170_e6282 * var_a_vde_dn8)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn9 - var_vfe_dn9) * var_a_vde) - (assign6170_e6282 * var_a_vde_dn9)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn10 - var_vfe_dn10) * var_a_vde) - (assign6170_e6282 * var_a_vde_dn10)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_db0 - var_vfe_db0) * var_a_vde) - (assign6170_e6282 * var_a_vde_db0)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_db1 - var_vfe_db1) * var_a_vde) - (assign6170_e6282 * var_a_vde_db1)) / (var_a_vde * var_a_vde)),)
    } else {
        (var_vb2e1vfe, var_vb2e1vfe_dn0, var_vb2e1vfe_dn1, var_vb2e1vfe_dn2, var_vb2e1vfe_dn3, var_vb2e1vfe_dn4, var_vb2e1vfe_dn5, var_vb2e1vfe_dn6, var_vb2e1vfe_dn7, var_vb2e1vfe_dn8, var_vb2e1vfe_dn9, var_vb2e1vfe_dn10, var_vb2e1vfe_db0, var_vb2e1vfe_db1,)
    }
};
        var_vb2e1vfe = assign6170_e6286;
        var_vb2e1vfe_dn0 = assign6170_e6286_d_n0;
        var_vb2e1vfe_dn1 = assign6170_e6286_d_n1;
        var_vb2e1vfe_dn2 = assign6170_e6286_d_n2;
        var_vb2e1vfe_dn3 = assign6170_e6286_d_n3;
        var_vb2e1vfe_dn4 = assign6170_e6286_d_n4;
        var_vb2e1vfe_dn5 = assign6170_e6286_d_n5;
        var_vb2e1vfe_dn6 = assign6170_e6286_d_n6;
        var_vb2e1vfe_dn7 = assign6170_e6286_d_n7;
        var_vb2e1vfe_dn8 = assign6170_e6286_d_n8;
        var_vb2e1vfe_dn9 = assign6170_e6286_d_n9;
        var_vb2e1vfe_dn10 = assign6170_e6286_d_n10;
        var_vb2e1vfe_db0 = assign6170_e6286_d_b0;
        var_vb2e1vfe_db1 = assign6170_e6286_d_b1;
        var_vb2e1vfe_rv = 0.0;
        var_vb2e1vfe_rdn0 = 0.0;
        var_vb2e1vfe_rdn1 = 0.0;
        var_vb2e1vfe_rdn2 = 0.0;
        var_vb2e1vfe_rdn3 = 0.0;
        var_vb2e1vfe_rdn4 = 0.0;
        var_vb2e1vfe_rdn5 = 0.0;
        var_vb2e1vfe_rdn6 = 0.0;
        var_vb2e1vfe_rdn7 = 0.0;
        var_vb2e1vfe_rdn8 = 0.0;
        var_vb2e1vfe_rdn9 = 0.0;
        var_vb2e1vfe_rdn10 = 0.0;
        var_vb2e1vfe_rdb0 = 0.0;
        var_vb2e1vfe_rdb1 = 0.0;

        let assign6180_e6289: f64 = if var_vb2e1vfe < 0.0 { 1.0 } else { 0.0 };
        var_guard113 = assign6180_e6289;
        var_guard113_dn0 = 0.0;
        var_guard113_dn1 = 0.0;
        var_guard113_dn2 = 0.0;
        var_guard113_dn3 = 0.0;
        var_guard113_dn4 = 0.0;
        var_guard113_dn5 = 0.0;
        var_guard113_dn6 = 0.0;
        var_guard113_dn7 = 0.0;
        var_guard113_dn8 = 0.0;
        var_guard113_dn9 = 0.0;
        var_guard113_dn10 = 0.0;
        var_guard113_db0 = 0.0;
        var_guard113_db1 = 0.0;
        var_guard113_rv = 0.0;
        var_guard113_rdn0 = 0.0;
        var_guard113_rdn1 = 0.0;
        var_guard113_rdn2 = 0.0;
        var_guard113_rdn3 = 0.0;
        var_guard113_rdn4 = 0.0;
        var_guard113_rdn5 = 0.0;
        var_guard113_rdn6 = 0.0;
        var_guard113_rdn7 = 0.0;
        var_guard113_rdn8 = 0.0;
        var_guard113_rdn9 = 0.0;
        var_guard113_rdn10 = 0.0;
        var_guard113_rdb0 = 0.0;
        var_guard113_rdb1 = 0.0;

        let (assign6190_e6300, assign6190_e6300_d_n0, assign6190_e6300_d_n1, assign6190_e6300_d_n2, assign6190_e6300_d_n3, assign6190_e6300_d_n4, assign6190_e6300_d_n5, assign6190_e6300_d_n6, assign6190_e6300_d_n7, assign6190_e6300_d_n8, assign6190_e6300_d_n9, assign6190_e6300_d_n10, assign6190_e6300_d_b0, assign6190_e6300_d_b1,) = {
    if ((var_guard112 != 0.0) && (var_guard113 != 0.0)) {
        let assign6190_e6296: f64 = (var_vb2e1vfe).exp();
        let assign6190_e6297: f64 = (1.0 + assign6190_e6296);
        let assign6190_e6298: f64 = (1.0 / assign6190_e6297);
        (assign6190_e6298, (-((assign6190_e6296 * var_vb2e1vfe_dn0) / (assign6190_e6297 * assign6190_e6297))), (-((assign6190_e6296 * var_vb2e1vfe_dn1) / (assign6190_e6297 * assign6190_e6297))), (-((assign6190_e6296 * var_vb2e1vfe_dn2) / (assign6190_e6297 * assign6190_e6297))), (-((assign6190_e6296 * var_vb2e1vfe_dn3) / (assign6190_e6297 * assign6190_e6297))), (-((assign6190_e6296 * var_vb2e1vfe_dn4) / (assign6190_e6297 * assign6190_e6297))), (-((assign6190_e6296 * var_vb2e1vfe_dn5) / (assign6190_e6297 * assign6190_e6297))), (-((assign6190_e6296 * var_vb2e1vfe_dn6) / (assign6190_e6297 * assign6190_e6297))), (-((assign6190_e6296 * var_vb2e1vfe_dn7) / (assign6190_e6297 * assign6190_e6297))), (-((assign6190_e6296 * var_vb2e1vfe_dn8) / (assign6190_e6297 * assign6190_e6297))), (-((assign6190_e6296 * var_vb2e1vfe_dn9) / (assign6190_e6297 * assign6190_e6297))), (-((assign6190_e6296 * var_vb2e1vfe_dn10) / (assign6190_e6297 * assign6190_e6297))), (-((assign6190_e6296 * var_vb2e1vfe_db0) / (assign6190_e6297 * assign6190_e6297))), (-((assign6190_e6296 * var_vb2e1vfe_db1) / (assign6190_e6297 * assign6190_e6297))),)
    } else {
        (var_dvjevb2e1, var_dvjevb2e1_dn0, var_dvjevb2e1_dn1, var_dvjevb2e1_dn2, var_dvjevb2e1_dn3, var_dvjevb2e1_dn4, var_dvjevb2e1_dn5, var_dvjevb2e1_dn6, var_dvjevb2e1_dn7, var_dvjevb2e1_dn8, var_dvjevb2e1_dn9, var_dvjevb2e1_dn10, var_dvjevb2e1_db0, var_dvjevb2e1_db1,)
    }
};
        var_dvjevb2e1 = assign6190_e6300;
        var_dvjevb2e1_dn0 = assign6190_e6300_d_n0;
        var_dvjevb2e1_dn1 = assign6190_e6300_d_n1;
        var_dvjevb2e1_dn2 = assign6190_e6300_d_n2;
        var_dvjevb2e1_dn3 = assign6190_e6300_d_n3;
        var_dvjevb2e1_dn4 = assign6190_e6300_d_n4;
        var_dvjevb2e1_dn5 = assign6190_e6300_d_n5;
        var_dvjevb2e1_dn6 = assign6190_e6300_d_n6;
        var_dvjevb2e1_dn7 = assign6190_e6300_d_n7;
        var_dvjevb2e1_dn8 = assign6190_e6300_d_n8;
        var_dvjevb2e1_dn9 = assign6190_e6300_d_n9;
        var_dvjevb2e1_dn10 = assign6190_e6300_d_n10;
        var_dvjevb2e1_db0 = assign6190_e6300_d_b0;
        var_dvjevb2e1_db1 = assign6190_e6300_d_b1;
        var_dvjevb2e1_rv = 0.0;
        var_dvjevb2e1_rdn0 = 0.0;
        var_dvjevb2e1_rdn1 = 0.0;
        var_dvjevb2e1_rdn2 = 0.0;
        var_dvjevb2e1_rdn3 = 0.0;
        var_dvjevb2e1_rdn4 = 0.0;
        var_dvjevb2e1_rdn5 = 0.0;
        var_dvjevb2e1_rdn6 = 0.0;
        var_dvjevb2e1_rdn7 = 0.0;
        var_dvjevb2e1_rdn8 = 0.0;
        var_dvjevb2e1_rdn9 = 0.0;
        var_dvjevb2e1_rdn10 = 0.0;
        var_dvjevb2e1_rdb0 = 0.0;
        var_dvjevb2e1_rdb1 = 0.0;

        let (assign6200_e6315, assign6200_e6315_d_n0, assign6200_e6315_d_n1, assign6200_e6315_d_n2, assign6200_e6315_d_n3, assign6200_e6315_d_n4, assign6200_e6315_d_n5, assign6200_e6315_d_n6, assign6200_e6315_d_n7, assign6200_e6315_d_n8, assign6200_e6315_d_n9, assign6200_e6315_d_n10, assign6200_e6315_d_b0, assign6200_e6315_d_b1,) = {
    if ((var_guard112 != 0.0) && (var_guard113 == 0.0)) {
        let assign6200_e6306: f64 = (-var_vb2e1vfe);
        let assign6200_e6307: f64 = (assign6200_e6306).exp();
        let assign6200_e6310: f64 = (-var_vb2e1vfe);
        let assign6200_e6311: f64 = (assign6200_e6310).exp();
        let assign6200_e6312: f64 = (1.0 + assign6200_e6311);
        let assign6200_e6313: f64 = (assign6200_e6307 / assign6200_e6312);
        (assign6200_e6313, ((((assign6200_e6307 * (-var_vb2e1vfe_dn0)) * assign6200_e6312) - (assign6200_e6307 * (assign6200_e6311 * (-var_vb2e1vfe_dn0)))) / (assign6200_e6312 * assign6200_e6312)), ((((assign6200_e6307 * (-var_vb2e1vfe_dn1)) * assign6200_e6312) - (assign6200_e6307 * (assign6200_e6311 * (-var_vb2e1vfe_dn1)))) / (assign6200_e6312 * assign6200_e6312)), ((((assign6200_e6307 * (-var_vb2e1vfe_dn2)) * assign6200_e6312) - (assign6200_e6307 * (assign6200_e6311 * (-var_vb2e1vfe_dn2)))) / (assign6200_e6312 * assign6200_e6312)), ((((assign6200_e6307 * (-var_vb2e1vfe_dn3)) * assign6200_e6312) - (assign6200_e6307 * (assign6200_e6311 * (-var_vb2e1vfe_dn3)))) / (assign6200_e6312 * assign6200_e6312)), ((((assign6200_e6307 * (-var_vb2e1vfe_dn4)) * assign6200_e6312) - (assign6200_e6307 * (assign6200_e6311 * (-var_vb2e1vfe_dn4)))) / (assign6200_e6312 * assign6200_e6312)), ((((assign6200_e6307 * (-var_vb2e1vfe_dn5)) * assign6200_e6312) - (assign6200_e6307 * (assign6200_e6311 * (-var_vb2e1vfe_dn5)))) / (assign6200_e6312 * assign6200_e6312)), ((((assign6200_e6307 * (-var_vb2e1vfe_dn6)) * assign6200_e6312) - (assign6200_e6307 * (assign6200_e6311 * (-var_vb2e1vfe_dn6)))) / (assign6200_e6312 * assign6200_e6312)), ((((assign6200_e6307 * (-var_vb2e1vfe_dn7)) * assign6200_e6312) - (assign6200_e6307 * (assign6200_e6311 * (-var_vb2e1vfe_dn7)))) / (assign6200_e6312 * assign6200_e6312)), ((((assign6200_e6307 * (-var_vb2e1vfe_dn8)) * assign6200_e6312) - (assign6200_e6307 * (assign6200_e6311 * (-var_vb2e1vfe_dn8)))) / (assign6200_e6312 * assign6200_e6312)), ((((assign6200_e6307 * (-var_vb2e1vfe_dn9)) * assign6200_e6312) - (assign6200_e6307 * (assign6200_e6311 * (-var_vb2e1vfe_dn9)))) / (assign6200_e6312 * assign6200_e6312)), ((((assign6200_e6307 * (-var_vb2e1vfe_dn10)) * assign6200_e6312) - (assign6200_e6307 * (assign6200_e6311 * (-var_vb2e1vfe_dn10)))) / (assign6200_e6312 * assign6200_e6312)), ((((assign6200_e6307 * (-var_vb2e1vfe_db0)) * assign6200_e6312) - (assign6200_e6307 * (assign6200_e6311 * (-var_vb2e1vfe_db0)))) / (assign6200_e6312 * assign6200_e6312)), ((((assign6200_e6307 * (-var_vb2e1vfe_db1)) * assign6200_e6312) - (assign6200_e6307 * (assign6200_e6311 * (-var_vb2e1vfe_db1)))) / (assign6200_e6312 * assign6200_e6312)),)
    } else {
        (var_dvjevb2e1, var_dvjevb2e1_dn0, var_dvjevb2e1_dn1, var_dvjevb2e1_dn2, var_dvjevb2e1_dn3, var_dvjevb2e1_dn4, var_dvjevb2e1_dn5, var_dvjevb2e1_dn6, var_dvjevb2e1_dn7, var_dvjevb2e1_dn8, var_dvjevb2e1_dn9, var_dvjevb2e1_dn10, var_dvjevb2e1_db0, var_dvjevb2e1_db1,)
    }
};
        var_dvjevb2e1 = assign6200_e6315;
        var_dvjevb2e1_dn0 = assign6200_e6315_d_n0;
        var_dvjevb2e1_dn1 = assign6200_e6315_d_n1;
        var_dvjevb2e1_dn2 = assign6200_e6315_d_n2;
        var_dvjevb2e1_dn3 = assign6200_e6315_d_n3;
        var_dvjevb2e1_dn4 = assign6200_e6315_d_n4;
        var_dvjevb2e1_dn5 = assign6200_e6315_d_n5;
        var_dvjevb2e1_dn6 = assign6200_e6315_d_n6;
        var_dvjevb2e1_dn7 = assign6200_e6315_d_n7;
        var_dvjevb2e1_dn8 = assign6200_e6315_d_n8;
        var_dvjevb2e1_dn9 = assign6200_e6315_d_n9;
        var_dvjevb2e1_dn10 = assign6200_e6315_d_n10;
        var_dvjevb2e1_db0 = assign6200_e6315_d_b0;
        var_dvjevb2e1_db1 = assign6200_e6315_d_b1;
        var_dvjevb2e1_rv = 0.0;
        var_dvjevb2e1_rdn0 = 0.0;
        var_dvjevb2e1_rdn1 = 0.0;
        var_dvjevb2e1_rdn2 = 0.0;
        var_dvjevb2e1_rdn3 = 0.0;
        var_dvjevb2e1_rdn4 = 0.0;
        var_dvjevb2e1_rdn5 = 0.0;
        var_dvjevb2e1_rdn6 = 0.0;
        var_dvjevb2e1_rdn7 = 0.0;
        var_dvjevb2e1_rdn8 = 0.0;
        var_dvjevb2e1_rdn9 = 0.0;
        var_dvjevb2e1_rdn10 = 0.0;
        var_dvjevb2e1_rdb0 = 0.0;
        var_dvjevb2e1_rdb1 = 0.0;

        let (assign6210_e6323, assign6210_e6323_d_n0, assign6210_e6323_d_n1, assign6210_e6323_d_n2, assign6210_e6323_d_n3, assign6210_e6323_d_n4, assign6210_e6323_d_n5, assign6210_e6323_d_n6, assign6210_e6323_d_n7, assign6210_e6323_d_n8, assign6210_e6323_d_n9, assign6210_e6323_d_n10, assign6210_e6323_d_b0, assign6210_e6323_d_b1,) = {
    if (var_guard112 != 0.0) {
        let assign6210_e6319: f64 = (var_dvtevje * var_dvjevb2e1);
        let assign6210_e6321: f64 = (assign6210_e6319 + 3.0);
        (assign6210_e6321, ((var_dvtevje_dn0 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn0)), ((var_dvtevje_dn1 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn1)), ((var_dvtevje_dn2 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn2)), ((var_dvtevje_dn3 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn3)), ((var_dvtevje_dn4 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn4)), ((var_dvtevje_dn5 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn5)), ((var_dvtevje_dn6 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn6)), ((var_dvtevje_dn7 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn7)), ((var_dvtevje_dn8 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn8)), ((var_dvtevje_dn9 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn9)), ((var_dvtevje_dn10 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn10)), ((var_dvtevje_db0 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_db0)), ((var_dvtevje_db1 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_db1)),)
    } else {
        (var_dvtevb2e1, var_dvtevb2e1_dn0, var_dvtevb2e1_dn1, var_dvtevb2e1_dn2, var_dvtevb2e1_dn3, var_dvtevb2e1_dn4, var_dvtevb2e1_dn5, var_dvtevb2e1_dn6, var_dvtevb2e1_dn7, var_dvtevb2e1_dn8, var_dvtevb2e1_dn9, var_dvtevb2e1_dn10, var_dvtevb2e1_db0, var_dvtevb2e1_db1,)
    }
};
        var_dvtevb2e1 = assign6210_e6323;
        var_dvtevb2e1_dn0 = assign6210_e6323_d_n0;
        var_dvtevb2e1_dn1 = assign6210_e6323_d_n1;
        var_dvtevb2e1_dn2 = assign6210_e6323_d_n2;
        var_dvtevb2e1_dn3 = assign6210_e6323_d_n3;
        var_dvtevb2e1_dn4 = assign6210_e6323_d_n4;
        var_dvtevb2e1_dn5 = assign6210_e6323_d_n5;
        var_dvtevb2e1_dn6 = assign6210_e6323_d_n6;
        var_dvtevb2e1_dn7 = assign6210_e6323_d_n7;
        var_dvtevb2e1_dn8 = assign6210_e6323_d_n8;
        var_dvtevb2e1_dn9 = assign6210_e6323_d_n9;
        var_dvtevb2e1_dn10 = assign6210_e6323_d_n10;
        var_dvtevb2e1_db0 = assign6210_e6323_d_b0;
        var_dvtevb2e1_db1 = assign6210_e6323_d_b1;
        var_dvtevb2e1_rv = 0.0;
        var_dvtevb2e1_rdn0 = 0.0;
        var_dvtevb2e1_rdn1 = 0.0;
        var_dvtevb2e1_rdn2 = 0.0;
        var_dvtevb2e1_rdn3 = 0.0;
        var_dvtevb2e1_rdn4 = 0.0;
        var_dvtevb2e1_rdn5 = 0.0;
        var_dvtevb2e1_rdn6 = 0.0;
        var_dvtevb2e1_rdn7 = 0.0;
        var_dvtevb2e1_rdn8 = 0.0;
        var_dvtevb2e1_rdn9 = 0.0;
        var_dvtevb2e1_rdn10 = 0.0;
        var_dvtevb2e1_rdb0 = 0.0;
        var_dvtevb2e1_rdb1 = 0.0;

        *var_dvjevb2e1_slot = var_dvjevb2e1;
        *var_dvjevb2e1_db0_slot = var_dvjevb2e1_db0;
        *var_dvjevb2e1_db1_slot = var_dvjevb2e1_db1;
        *var_dvjevb2e1_dn0_slot = var_dvjevb2e1_dn0;
        *var_dvjevb2e1_dn1_slot = var_dvjevb2e1_dn1;
        *var_dvjevb2e1_dn10_slot = var_dvjevb2e1_dn10;
        *var_dvjevb2e1_dn2_slot = var_dvjevb2e1_dn2;
        *var_dvjevb2e1_dn3_slot = var_dvjevb2e1_dn3;
        *var_dvjevb2e1_dn4_slot = var_dvjevb2e1_dn4;
        *var_dvjevb2e1_dn5_slot = var_dvjevb2e1_dn5;
        *var_dvjevb2e1_dn6_slot = var_dvjevb2e1_dn6;
        *var_dvjevb2e1_dn7_slot = var_dvjevb2e1_dn7;
        *var_dvjevb2e1_dn8_slot = var_dvjevb2e1_dn8;
        *var_dvjevb2e1_dn9_slot = var_dvjevb2e1_dn9;
        *var_dvjevb2e1_rdb0_slot = var_dvjevb2e1_rdb0;
        *var_dvjevb2e1_rdb1_slot = var_dvjevb2e1_rdb1;
        *var_dvjevb2e1_rdn0_slot = var_dvjevb2e1_rdn0;
        *var_dvjevb2e1_rdn1_slot = var_dvjevb2e1_rdn1;
        *var_dvjevb2e1_rdn10_slot = var_dvjevb2e1_rdn10;
        *var_dvjevb2e1_rdn2_slot = var_dvjevb2e1_rdn2;
        *var_dvjevb2e1_rdn3_slot = var_dvjevb2e1_rdn3;
        *var_dvjevb2e1_rdn4_slot = var_dvjevb2e1_rdn4;
        *var_dvjevb2e1_rdn5_slot = var_dvjevb2e1_rdn5;
        *var_dvjevb2e1_rdn6_slot = var_dvjevb2e1_rdn6;
        *var_dvjevb2e1_rdn7_slot = var_dvjevb2e1_rdn7;
        *var_dvjevb2e1_rdn8_slot = var_dvjevb2e1_rdn8;
        *var_dvjevb2e1_rdn9_slot = var_dvjevb2e1_rdn9;
        *var_dvjevb2e1_rv_slot = var_dvjevb2e1_rv;
        *var_dvtevb2e1_slot = var_dvtevb2e1;
        *var_dvtevb2e1_db0_slot = var_dvtevb2e1_db0;
        *var_dvtevb2e1_db1_slot = var_dvtevb2e1_db1;
        *var_dvtevb2e1_dn0_slot = var_dvtevb2e1_dn0;
        *var_dvtevb2e1_dn1_slot = var_dvtevb2e1_dn1;
        *var_dvtevb2e1_dn10_slot = var_dvtevb2e1_dn10;
        *var_dvtevb2e1_dn2_slot = var_dvtevb2e1_dn2;
        *var_dvtevb2e1_dn3_slot = var_dvtevb2e1_dn3;
        *var_dvtevb2e1_dn4_slot = var_dvtevb2e1_dn4;
        *var_dvtevb2e1_dn5_slot = var_dvtevb2e1_dn5;
        *var_dvtevb2e1_dn6_slot = var_dvtevb2e1_dn6;
        *var_dvtevb2e1_dn7_slot = var_dvtevb2e1_dn7;
        *var_dvtevb2e1_dn8_slot = var_dvtevb2e1_dn8;
        *var_dvtevb2e1_dn9_slot = var_dvtevb2e1_dn9;
        *var_dvtevb2e1_rdb0_slot = var_dvtevb2e1_rdb0;
        *var_dvtevb2e1_rdb1_slot = var_dvtevb2e1_rdb1;
        *var_dvtevb2e1_rdn0_slot = var_dvtevb2e1_rdn0;
        *var_dvtevb2e1_rdn1_slot = var_dvtevb2e1_rdn1;
        *var_dvtevb2e1_rdn10_slot = var_dvtevb2e1_rdn10;
        *var_dvtevb2e1_rdn2_slot = var_dvtevb2e1_rdn2;
        *var_dvtevb2e1_rdn3_slot = var_dvtevb2e1_rdn3;
        *var_dvtevb2e1_rdn4_slot = var_dvtevb2e1_rdn4;
        *var_dvtevb2e1_rdn5_slot = var_dvtevb2e1_rdn5;
        *var_dvtevb2e1_rdn6_slot = var_dvtevb2e1_rdn6;
        *var_dvtevb2e1_rdn7_slot = var_dvtevb2e1_rdn7;
        *var_dvtevb2e1_rdn8_slot = var_dvtevb2e1_rdn8;
        *var_dvtevb2e1_rdn9_slot = var_dvtevb2e1_rdn9;
        *var_dvtevb2e1_rv_slot = var_dvtevb2e1_rv;
        *var_dvtevje_slot = var_dvtevje;
        *var_dvtevje_db0_slot = var_dvtevje_db0;
        *var_dvtevje_db1_slot = var_dvtevje_db1;
        *var_dvtevje_dn0_slot = var_dvtevje_dn0;
        *var_dvtevje_dn1_slot = var_dvtevje_dn1;
        *var_dvtevje_dn10_slot = var_dvtevje_dn10;
        *var_dvtevje_dn2_slot = var_dvtevje_dn2;
        *var_dvtevje_dn3_slot = var_dvtevje_dn3;
        *var_dvtevje_dn4_slot = var_dvtevje_dn4;
        *var_dvtevje_dn5_slot = var_dvtevje_dn5;
        *var_dvtevje_dn6_slot = var_dvtevje_dn6;
        *var_dvtevje_dn7_slot = var_dvtevje_dn7;
        *var_dvtevje_dn8_slot = var_dvtevje_dn8;
        *var_dvtevje_dn9_slot = var_dvtevje_dn9;
        *var_dvtevje_rdb0_slot = var_dvtevje_rdb0;
        *var_dvtevje_rdb1_slot = var_dvtevje_rdb1;
        *var_dvtevje_rdn0_slot = var_dvtevje_rdn0;
        *var_dvtevje_rdn1_slot = var_dvtevje_rdn1;
        *var_dvtevje_rdn10_slot = var_dvtevje_rdn10;
        *var_dvtevje_rdn2_slot = var_dvtevje_rdn2;
        *var_dvtevje_rdn3_slot = var_dvtevje_rdn3;
        *var_dvtevje_rdn4_slot = var_dvtevje_rdn4;
        *var_dvtevje_rdn5_slot = var_dvtevje_rdn5;
        *var_dvtevje_rdn6_slot = var_dvtevje_rdn6;
        *var_dvtevje_rdn7_slot = var_dvtevje_rdn7;
        *var_dvtevje_rdn8_slot = var_dvtevje_rdn8;
        *var_dvtevje_rdn9_slot = var_dvtevje_rdn9;
        *var_dvtevje_rv_slot = var_dvtevje_rv;
        *var_evbc3vdcex_slot = var_evbc3vdcex;
        *var_evbc3vdcex_db0_slot = var_evbc3vdcex_db0;
        *var_evbc3vdcex_db1_slot = var_evbc3vdcex_db1;
        *var_evbc3vdcex_dn0_slot = var_evbc3vdcex_dn0;
        *var_evbc3vdcex_dn1_slot = var_evbc3vdcex_dn1;
        *var_evbc3vdcex_dn10_slot = var_evbc3vdcex_dn10;
        *var_evbc3vdcex_dn2_slot = var_evbc3vdcex_dn2;
        *var_evbc3vdcex_dn3_slot = var_evbc3vdcex_dn3;
        *var_evbc3vdcex_dn4_slot = var_evbc3vdcex_dn4;
        *var_evbc3vdcex_dn5_slot = var_evbc3vdcex_dn5;
        *var_evbc3vdcex_dn6_slot = var_evbc3vdcex_dn6;
        *var_evbc3vdcex_dn7_slot = var_evbc3vdcex_dn7;
        *var_evbc3vdcex_dn8_slot = var_evbc3vdcex_dn8;
        *var_evbc3vdcex_dn9_slot = var_evbc3vdcex_dn9;
        *var_evbc3vdcex_rdb0_slot = var_evbc3vdcex_rdb0;
        *var_evbc3vdcex_rdb1_slot = var_evbc3vdcex_rdb1;
        *var_evbc3vdcex_rdn0_slot = var_evbc3vdcex_rdn0;
        *var_evbc3vdcex_rdn1_slot = var_evbc3vdcex_rdn1;
        *var_evbc3vdcex_rdn10_slot = var_evbc3vdcex_rdn10;
        *var_evbc3vdcex_rdn2_slot = var_evbc3vdcex_rdn2;
        *var_evbc3vdcex_rdn3_slot = var_evbc3vdcex_rdn3;
        *var_evbc3vdcex_rdn4_slot = var_evbc3vdcex_rdn4;
        *var_evbc3vdcex_rdn5_slot = var_evbc3vdcex_rdn5;
        *var_evbc3vdcex_rdn6_slot = var_evbc3vdcex_rdn6;
        *var_evbc3vdcex_rdn7_slot = var_evbc3vdcex_rdn7;
        *var_evbc3vdcex_rdn8_slot = var_evbc3vdcex_rdn8;
        *var_evbc3vdcex_rdn9_slot = var_evbc3vdcex_rdn9;
        *var_evbc3vdcex_rv_slot = var_evbc3vdcex_rv;
        *var_expl_slot = var_expl;
        *var_expl_db0_slot = var_expl_db0;
        *var_expl_db1_slot = var_expl_db1;
        *var_expl_dn0_slot = var_expl_dn0;
        *var_expl_dn1_slot = var_expl_dn1;
        *var_expl_dn10_slot = var_expl_dn10;
        *var_expl_dn2_slot = var_expl_dn2;
        *var_expl_dn3_slot = var_expl_dn3;
        *var_expl_dn4_slot = var_expl_dn4;
        *var_expl_dn5_slot = var_expl_dn5;
        *var_expl_dn6_slot = var_expl_dn6;
        *var_expl_dn7_slot = var_expl_dn7;
        *var_expl_dn8_slot = var_expl_dn8;
        *var_expl_dn9_slot = var_expl_dn9;
        *var_expl_rdb0_slot = var_expl_rdb0;
        *var_expl_rdb1_slot = var_expl_rdb1;
        *var_expl_rdn0_slot = var_expl_rdn0;
        *var_expl_rdn1_slot = var_expl_rdn1;
        *var_expl_rdn10_slot = var_expl_rdn10;
        *var_expl_rdn2_slot = var_expl_rdn2;
        *var_expl_rdn3_slot = var_expl_rdn3;
        *var_expl_rdn4_slot = var_expl_rdn4;
        *var_expl_rdn5_slot = var_expl_rdn5;
        *var_expl_rdn6_slot = var_expl_rdn6;
        *var_expl_rdn7_slot = var_expl_rdn7;
        *var_expl_rdn8_slot = var_expl_rdn8;
        *var_expl_rdn9_slot = var_expl_rdn9;
        *var_expl_rv_slot = var_expl_rv;
        *var_guard111_slot = var_guard111;
        *var_guard111_db0_slot = var_guard111_db0;
        *var_guard111_db1_slot = var_guard111_db1;
        *var_guard111_dn0_slot = var_guard111_dn0;
        *var_guard111_dn1_slot = var_guard111_dn1;
        *var_guard111_dn10_slot = var_guard111_dn10;
        *var_guard111_dn2_slot = var_guard111_dn2;
        *var_guard111_dn3_slot = var_guard111_dn3;
        *var_guard111_dn4_slot = var_guard111_dn4;
        *var_guard111_dn5_slot = var_guard111_dn5;
        *var_guard111_dn6_slot = var_guard111_dn6;
        *var_guard111_dn7_slot = var_guard111_dn7;
        *var_guard111_dn8_slot = var_guard111_dn8;
        *var_guard111_dn9_slot = var_guard111_dn9;
        *var_guard111_rdb0_slot = var_guard111_rdb0;
        *var_guard111_rdb1_slot = var_guard111_rdb1;
        *var_guard111_rdn0_slot = var_guard111_rdn0;
        *var_guard111_rdn1_slot = var_guard111_rdn1;
        *var_guard111_rdn10_slot = var_guard111_rdn10;
        *var_guard111_rdn2_slot = var_guard111_rdn2;
        *var_guard111_rdn3_slot = var_guard111_rdn3;
        *var_guard111_rdn4_slot = var_guard111_rdn4;
        *var_guard111_rdn5_slot = var_guard111_rdn5;
        *var_guard111_rdn6_slot = var_guard111_rdn6;
        *var_guard111_rdn7_slot = var_guard111_rdn7;
        *var_guard111_rdn8_slot = var_guard111_rdn8;
        *var_guard111_rdn9_slot = var_guard111_rdn9;
        *var_guard111_rv_slot = var_guard111_rv;
        *var_guard112_slot = var_guard112;
        *var_guard112_db0_slot = var_guard112_db0;
        *var_guard112_db1_slot = var_guard112_db1;
        *var_guard112_dn0_slot = var_guard112_dn0;
        *var_guard112_dn1_slot = var_guard112_dn1;
        *var_guard112_dn10_slot = var_guard112_dn10;
        *var_guard112_dn2_slot = var_guard112_dn2;
        *var_guard112_dn3_slot = var_guard112_dn3;
        *var_guard112_dn4_slot = var_guard112_dn4;
        *var_guard112_dn5_slot = var_guard112_dn5;
        *var_guard112_dn6_slot = var_guard112_dn6;
        *var_guard112_dn7_slot = var_guard112_dn7;
        *var_guard112_dn8_slot = var_guard112_dn8;
        *var_guard112_dn9_slot = var_guard112_dn9;
        *var_guard112_rdb0_slot = var_guard112_rdb0;
        *var_guard112_rdb1_slot = var_guard112_rdb1;
        *var_guard112_rdn0_slot = var_guard112_rdn0;
        *var_guard112_rdn1_slot = var_guard112_rdn1;
        *var_guard112_rdn10_slot = var_guard112_rdn10;
        *var_guard112_rdn2_slot = var_guard112_rdn2;
        *var_guard112_rdn3_slot = var_guard112_rdn3;
        *var_guard112_rdn4_slot = var_guard112_rdn4;
        *var_guard112_rdn5_slot = var_guard112_rdn5;
        *var_guard112_rdn6_slot = var_guard112_rdn6;
        *var_guard112_rdn7_slot = var_guard112_rdn7;
        *var_guard112_rdn8_slot = var_guard112_rdn8;
        *var_guard112_rdn9_slot = var_guard112_rdn9;
        *var_guard112_rv_slot = var_guard112_rv;
        *var_guard113_slot = var_guard113;
        *var_guard113_db0_slot = var_guard113_db0;
        *var_guard113_db1_slot = var_guard113_db1;
        *var_guard113_dn0_slot = var_guard113_dn0;
        *var_guard113_dn1_slot = var_guard113_dn1;
        *var_guard113_dn10_slot = var_guard113_dn10;
        *var_guard113_dn2_slot = var_guard113_dn2;
        *var_guard113_dn3_slot = var_guard113_dn3;
        *var_guard113_dn4_slot = var_guard113_dn4;
        *var_guard113_dn5_slot = var_guard113_dn5;
        *var_guard113_dn6_slot = var_guard113_dn6;
        *var_guard113_dn7_slot = var_guard113_dn7;
        *var_guard113_dn8_slot = var_guard113_dn8;
        *var_guard113_dn9_slot = var_guard113_dn9;
        *var_guard113_rdb0_slot = var_guard113_rdb0;
        *var_guard113_rdb1_slot = var_guard113_rdb1;
        *var_guard113_rdn0_slot = var_guard113_rdn0;
        *var_guard113_rdn1_slot = var_guard113_rdn1;
        *var_guard113_rdn10_slot = var_guard113_rdn10;
        *var_guard113_rdn2_slot = var_guard113_rdn2;
        *var_guard113_rdn3_slot = var_guard113_rdn3;
        *var_guard113_rdn4_slot = var_guard113_rdn4;
        *var_guard113_rdn5_slot = var_guard113_rdn5;
        *var_guard113_rdn6_slot = var_guard113_rdn6;
        *var_guard113_rdn7_slot = var_guard113_rdn7;
        *var_guard113_rdn8_slot = var_guard113_rdn8;
        *var_guard113_rdn9_slot = var_guard113_rdn9;
        *var_guard113_rv_slot = var_guard113_rv;
        *var_vb2e1vfe_slot = var_vb2e1vfe;
        *var_vb2e1vfe_db0_slot = var_vb2e1vfe_db0;
        *var_vb2e1vfe_db1_slot = var_vb2e1vfe_db1;
        *var_vb2e1vfe_dn0_slot = var_vb2e1vfe_dn0;
        *var_vb2e1vfe_dn1_slot = var_vb2e1vfe_dn1;
        *var_vb2e1vfe_dn10_slot = var_vb2e1vfe_dn10;
        *var_vb2e1vfe_dn2_slot = var_vb2e1vfe_dn2;
        *var_vb2e1vfe_dn3_slot = var_vb2e1vfe_dn3;
        *var_vb2e1vfe_dn4_slot = var_vb2e1vfe_dn4;
        *var_vb2e1vfe_dn5_slot = var_vb2e1vfe_dn5;
        *var_vb2e1vfe_dn6_slot = var_vb2e1vfe_dn6;
        *var_vb2e1vfe_dn7_slot = var_vb2e1vfe_dn7;
        *var_vb2e1vfe_dn8_slot = var_vb2e1vfe_dn8;
        *var_vb2e1vfe_dn9_slot = var_vb2e1vfe_dn9;
        *var_vb2e1vfe_rdb0_slot = var_vb2e1vfe_rdb0;
        *var_vb2e1vfe_rdb1_slot = var_vb2e1vfe_rdb1;
        *var_vb2e1vfe_rdn0_slot = var_vb2e1vfe_rdn0;
        *var_vb2e1vfe_rdn1_slot = var_vb2e1vfe_rdn1;
        *var_vb2e1vfe_rdn10_slot = var_vb2e1vfe_rdn10;
        *var_vb2e1vfe_rdn2_slot = var_vb2e1vfe_rdn2;
        *var_vb2e1vfe_rdn3_slot = var_vb2e1vfe_rdn3;
        *var_vb2e1vfe_rdn4_slot = var_vb2e1vfe_rdn4;
        *var_vb2e1vfe_rdn5_slot = var_vb2e1vfe_rdn5;
        *var_vb2e1vfe_rdn6_slot = var_vb2e1vfe_rdn6;
        *var_vb2e1vfe_rdn7_slot = var_vb2e1vfe_rdn7;
        *var_vb2e1vfe_rdn8_slot = var_vb2e1vfe_rdn8;
        *var_vb2e1vfe_rdn9_slot = var_vb2e1vfe_rdn9;
        *var_vb2e1vfe_rv_slot = var_vb2e1vfe_rv;
        *var_xqex_slot = var_xqex;
        *var_xqex_db0_slot = var_xqex_db0;
        *var_xqex_db1_slot = var_xqex_db1;
        *var_xqex_dn0_slot = var_xqex_dn0;
        *var_xqex_dn1_slot = var_xqex_dn1;
        *var_xqex_dn10_slot = var_xqex_dn10;
        *var_xqex_dn2_slot = var_xqex_dn2;
        *var_xqex_dn3_slot = var_xqex_dn3;
        *var_xqex_dn4_slot = var_xqex_dn4;
        *var_xqex_dn5_slot = var_xqex_dn5;
        *var_xqex_dn6_slot = var_xqex_dn6;
        *var_xqex_dn7_slot = var_xqex_dn7;
        *var_xqex_dn8_slot = var_xqex_dn8;
        *var_xqex_dn9_slot = var_xqex_dn9;
        *var_xqex_rdb0_slot = var_xqex_rdb0;
        *var_xqex_rdb1_slot = var_xqex_rdb1;
        *var_xqex_rdn0_slot = var_xqex_rdn0;
        *var_xqex_rdn1_slot = var_xqex_rdn1;
        *var_xqex_rdn10_slot = var_xqex_rdn10;
        *var_xqex_rdn2_slot = var_xqex_rdn2;
        *var_xqex_rdn3_slot = var_xqex_rdn3;
        *var_xqex_rdn4_slot = var_xqex_rdn4;
        *var_xqex_rdn5_slot = var_xqex_rdn5;
        *var_xqex_rdn6_slot = var_xqex_rdn6;
        *var_xqex_rdn7_slot = var_xqex_rdn7;
        *var_xqex_rdn8_slot = var_xqex_rdn8;
        *var_xqex_rdn9_slot = var_xqex_rdn9;
        *var_xqex_rv_slot = var_xqex_rv;
        *var_xqmex_slot = var_xqmex;
        *var_xqmex_db0_slot = var_xqmex_db0;
        *var_xqmex_db1_slot = var_xqmex_db1;
        *var_xqmex_dn0_slot = var_xqmex_dn0;
        *var_xqmex_dn1_slot = var_xqmex_dn1;
        *var_xqmex_dn10_slot = var_xqmex_dn10;
        *var_xqmex_dn2_slot = var_xqmex_dn2;
        *var_xqmex_dn3_slot = var_xqmex_dn3;
        *var_xqmex_dn4_slot = var_xqmex_dn4;
        *var_xqmex_dn5_slot = var_xqmex_dn5;
        *var_xqmex_dn6_slot = var_xqmex_dn6;
        *var_xqmex_dn7_slot = var_xqmex_dn7;
        *var_xqmex_dn8_slot = var_xqmex_dn8;
        *var_xqmex_dn9_slot = var_xqmex_dn9;
        *var_xqmex_rdb0_slot = var_xqmex_rdb0;
        *var_xqmex_rdb1_slot = var_xqmex_rdb1;
        *var_xqmex_rdn0_slot = var_xqmex_rdn0;
        *var_xqmex_rdn1_slot = var_xqmex_rdn1;
        *var_xqmex_rdn10_slot = var_xqmex_rdn10;
        *var_xqmex_rdn2_slot = var_xqmex_rdn2;
        *var_xqmex_rdn3_slot = var_xqmex_rdn3;
        *var_xqmex_rdn4_slot = var_xqmex_rdn4;
        *var_xqmex_rdn5_slot = var_xqmex_rdn5;
        *var_xqmex_rdn6_slot = var_xqmex_rdn6;
        *var_xqmex_rdn7_slot = var_xqmex_rdn7;
        *var_xqmex_rdn8_slot = var_xqmex_rdn8;
        *var_xqmex_rdn9_slot = var_xqmex_rdn9;
        *var_xqmex_rv_slot = var_xqmex_rv;
    }

    pub(super) fn stamp_reactive_block_37(
        p: &Parameters,
        var_cje_t: f64,
        var_cje_t_db0: f64,
        var_cje_t_db1: f64,
        var_cje_t_dn0: f64,
        var_cje_t_dn1: f64,
        var_cje_t_dn10: f64,
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
        var_f1_dn2: f64,
        var_f1_dn3: f64,
        var_f1_dn4: f64,
        var_f1_dn5: f64,
        var_f1_dn6: f64,
        var_f1_dn7: f64,
        var_f1_dn8: f64,
        var_f1_dn9: f64,
        var_guard112: f64,
        var_if0: f64,
        var_if0_db0: f64,
        var_if0_db1: f64,
        var_if0_dn0: f64,
        var_if0_dn1: f64,
        var_if0_dn10: f64,
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
        var_q1q_dn2: f64,
        var_q1q_dn3: f64,
        var_q1q_dn4: f64,
        var_q1q_dn5: f64,
        var_q1q_dn6: f64,
        var_q1q_dn7: f64,
        var_q1q_dn8: f64,
        var_q1q_dn9: f64,
        var_qb0: f64,
        var_qb0_db0: f64,
        var_qb0_db1: f64,
        var_qb0_dn0: f64,
        var_qb0_dn1: f64,
        var_qb0_dn10: f64,
        var_qb0_dn2: f64,
        var_qb0_dn3: f64,
        var_qb0_dn4: f64,
        var_qb0_dn5: f64,
        var_qb0_dn6: f64,
        var_qb0_dn7: f64,
        var_qb0_dn8: f64,
        var_qb0_dn9: f64,
        var_qbc_qs: f64,
        var_qbc_qs_db0: f64,
        var_qbc_qs_db1: f64,
        var_qbc_qs_dn0: f64,
        var_qbc_qs_dn1: f64,
        var_qbc_qs_dn10: f64,
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
        var_qe_qs_dn2: f64,
        var_qe_qs_dn3: f64,
        var_qe_qs_dn4: f64,
        var_qe_qs_dn5: f64,
        var_qe_qs_dn6: f64,
        var_qe_qs_dn7: f64,
        var_qe_qs_dn8: f64,
        var_qe_qs_dn9: f64,
        var_vb1b2: f64,
        var_vb1b2_db0: f64,
        var_vb1b2_db1: f64,
        var_vb1b2_dn0: f64,
        var_vb1b2_dn1: f64,
        var_vb1b2_dn10: f64,
        var_vb1b2_dn2: f64,
        var_vb1b2_dn3: f64,
        var_vb1b2_dn4: f64,
        var_vb1b2_dn5: f64,
        var_vb1b2_dn6: f64,
        var_vb1b2_dn7: f64,
        var_vb1b2_dn8: f64,
        var_vb1b2_dn9: f64,
        var_vt: f64,
        var_vt_db0: f64,
        var_vt_db1: f64,
        var_vt_dn0: f64,
        var_vt_dn1: f64,
        var_vt_dn10: f64,
        var_vt_dn2: f64,
        var_vt_dn3: f64,
        var_vt_dn4: f64,
        var_vt_dn5: f64,
        var_vt_dn6: f64,
        var_vt_dn7: f64,
        var_vt_dn8: f64,
        var_vt_dn9: f64,
        var_vtinv: f64,
        var_vtinv_db0: f64,
        var_vtinv_db1: f64,
        var_vtinv_dn0: f64,
        var_vtinv_dn1: f64,
        var_vtinv_dn10: f64,
        var_vtinv_dn2: f64,
        var_vtinv_dn3: f64,
        var_vtinv_dn4: f64,
        var_vtinv_dn5: f64,
        var_vtinv_dn6: f64,
        var_vtinv_dn7: f64,
        var_vtinv_dn8: f64,
        var_vtinv_dn9: f64,
        var_dn0vb2e1_slot: &mut f64,
        var_dn0vb2e1_db0_slot: &mut f64,
        var_dn0vb2e1_db1_slot: &mut f64,
        var_dn0vb2e1_dn0_slot: &mut f64,
        var_dn0vb2e1_dn1_slot: &mut f64,
        var_dn0vb2e1_dn10_slot: &mut f64,
        var_dn0vb2e1_dn2_slot: &mut f64,
        var_dn0vb2e1_dn3_slot: &mut f64,
        var_dn0vb2e1_dn4_slot: &mut f64,
        var_dn0vb2e1_dn5_slot: &mut f64,
        var_dn0vb2e1_dn6_slot: &mut f64,
        var_dn0vb2e1_dn7_slot: &mut f64,
        var_dn0vb2e1_dn8_slot: &mut f64,
        var_dn0vb2e1_dn9_slot: &mut f64,
        var_dn0vb2e1_rdb0_slot: &mut f64,
        var_dn0vb2e1_rdb1_slot: &mut f64,
        var_dn0vb2e1_rdn0_slot: &mut f64,
        var_dn0vb2e1_rdn1_slot: &mut f64,
        var_dn0vb2e1_rdn10_slot: &mut f64,
        var_dn0vb2e1_rdn2_slot: &mut f64,
        var_dn0vb2e1_rdn3_slot: &mut f64,
        var_dn0vb2e1_rdn4_slot: &mut f64,
        var_dn0vb2e1_rdn5_slot: &mut f64,
        var_dn0vb2e1_rdn6_slot: &mut f64,
        var_dn0vb2e1_rdn7_slot: &mut f64,
        var_dn0vb2e1_rdn8_slot: &mut f64,
        var_dn0vb2e1_rdn9_slot: &mut f64,
        var_dn0vb2e1_rv_slot: &mut f64,
        var_dqbevb2e1_slot: &mut f64,
        var_dqbevb2e1_db0_slot: &mut f64,
        var_dqbevb2e1_db1_slot: &mut f64,
        var_dqbevb2e1_dn0_slot: &mut f64,
        var_dqbevb2e1_dn1_slot: &mut f64,
        var_dqbevb2e1_dn10_slot: &mut f64,
        var_dqbevb2e1_dn2_slot: &mut f64,
        var_dqbevb2e1_dn3_slot: &mut f64,
        var_dqbevb2e1_dn4_slot: &mut f64,
        var_dqbevb2e1_dn5_slot: &mut f64,
        var_dqbevb2e1_dn6_slot: &mut f64,
        var_dqbevb2e1_dn7_slot: &mut f64,
        var_dqbevb2e1_dn8_slot: &mut f64,
        var_dqbevb2e1_dn9_slot: &mut f64,
        var_dqbevb2e1_rdb0_slot: &mut f64,
        var_dqbevb2e1_rdb1_slot: &mut f64,
        var_dqbevb2e1_rdn0_slot: &mut f64,
        var_dqbevb2e1_rdn1_slot: &mut f64,
        var_dqbevb2e1_rdn10_slot: &mut f64,
        var_dqbevb2e1_rdn2_slot: &mut f64,
        var_dqbevb2e1_rdn3_slot: &mut f64,
        var_dqbevb2e1_rdn4_slot: &mut f64,
        var_dqbevb2e1_rdn5_slot: &mut f64,
        var_dqbevb2e1_rdn6_slot: &mut f64,
        var_dqbevb2e1_rdn7_slot: &mut f64,
        var_dqbevb2e1_rdn8_slot: &mut f64,
        var_dqbevb2e1_rdn9_slot: &mut f64,
        var_dqbevb2e1_rv_slot: &mut f64,
        var_dqevb2e1_slot: &mut f64,
        var_dqevb2e1_db0_slot: &mut f64,
        var_dqevb2e1_db1_slot: &mut f64,
        var_dqevb2e1_dn0_slot: &mut f64,
        var_dqevb2e1_dn1_slot: &mut f64,
        var_dqevb2e1_dn10_slot: &mut f64,
        var_dqevb2e1_dn2_slot: &mut f64,
        var_dqevb2e1_dn3_slot: &mut f64,
        var_dqevb2e1_dn4_slot: &mut f64,
        var_dqevb2e1_dn5_slot: &mut f64,
        var_dqevb2e1_dn6_slot: &mut f64,
        var_dqevb2e1_dn7_slot: &mut f64,
        var_dqevb2e1_dn8_slot: &mut f64,
        var_dqevb2e1_dn9_slot: &mut f64,
        var_dqevb2e1_rdb0_slot: &mut f64,
        var_dqevb2e1_rdb1_slot: &mut f64,
        var_dqevb2e1_rdn0_slot: &mut f64,
        var_dqevb2e1_rdn1_slot: &mut f64,
        var_dqevb2e1_rdn10_slot: &mut f64,
        var_dqevb2e1_rdn2_slot: &mut f64,
        var_dqevb2e1_rdn3_slot: &mut f64,
        var_dqevb2e1_rdn4_slot: &mut f64,
        var_dqevb2e1_rdn5_slot: &mut f64,
        var_dqevb2e1_rdn6_slot: &mut f64,
        var_dqevb2e1_rdn7_slot: &mut f64,
        var_dqevb2e1_rdn8_slot: &mut f64,
        var_dqevb2e1_rdn9_slot: &mut f64,
        var_dqevb2e1_rv_slot: &mut f64,
        var_dqtevb2e1_slot: &mut f64,
        var_dqtevb2e1_db0_slot: &mut f64,
        var_dqtevb2e1_db1_slot: &mut f64,
        var_dqtevb2e1_dn0_slot: &mut f64,
        var_dqtevb2e1_dn1_slot: &mut f64,
        var_dqtevb2e1_dn10_slot: &mut f64,
        var_dqtevb2e1_dn2_slot: &mut f64,
        var_dqtevb2e1_dn3_slot: &mut f64,
        var_dqtevb2e1_dn4_slot: &mut f64,
        var_dqtevb2e1_dn5_slot: &mut f64,
        var_dqtevb2e1_dn6_slot: &mut f64,
        var_dqtevb2e1_dn7_slot: &mut f64,
        var_dqtevb2e1_dn8_slot: &mut f64,
        var_dqtevb2e1_dn9_slot: &mut f64,
        var_dqtevb2e1_rdb0_slot: &mut f64,
        var_dqtevb2e1_rdb1_slot: &mut f64,
        var_dqtevb2e1_rdn0_slot: &mut f64,
        var_dqtevb2e1_rdn1_slot: &mut f64,
        var_dqtevb2e1_rdn10_slot: &mut f64,
        var_dqtevb2e1_rdn2_slot: &mut f64,
        var_dqtevb2e1_rdn3_slot: &mut f64,
        var_dqtevb2e1_rdn4_slot: &mut f64,
        var_dqtevb2e1_rdn5_slot: &mut f64,
        var_dqtevb2e1_rdn6_slot: &mut f64,
        var_dqtevb2e1_rdn7_slot: &mut f64,
        var_dqtevb2e1_rdn8_slot: &mut f64,
        var_dqtevb2e1_rdn9_slot: &mut f64,
        var_dqtevb2e1_rv_slot: &mut f64,
        var_in_n_slot: &mut f64,
        var_in_n_db0_slot: &mut f64,
        var_in_n_db1_slot: &mut f64,
        var_in_n_dn0_slot: &mut f64,
        var_in_n_dn1_slot: &mut f64,
        var_in_n_dn10_slot: &mut f64,
        var_in_n_dn2_slot: &mut f64,
        var_in_n_dn3_slot: &mut f64,
        var_in_n_dn4_slot: &mut f64,
        var_in_n_dn5_slot: &mut f64,
        var_in_n_dn6_slot: &mut f64,
        var_in_n_dn7_slot: &mut f64,
        var_in_n_dn8_slot: &mut f64,
        var_in_n_dn9_slot: &mut f64,
        var_in_n_rdb0_slot: &mut f64,
        var_in_n_rdb1_slot: &mut f64,
        var_in_n_rdn0_slot: &mut f64,
        var_in_n_rdn1_slot: &mut f64,
        var_in_n_rdn10_slot: &mut f64,
        var_in_n_rdn2_slot: &mut f64,
        var_in_n_rdn3_slot: &mut f64,
        var_in_n_rdn4_slot: &mut f64,
        var_in_n_rdn5_slot: &mut f64,
        var_in_n_rdn6_slot: &mut f64,
        var_in_n_rdn7_slot: &mut f64,
        var_in_n_rdn8_slot: &mut f64,
        var_in_n_rdn9_slot: &mut f64,
        var_in_n_rv_slot: &mut f64,
        var_qb1b2_slot: &mut f64,
        var_qb1b2_db0_slot: &mut f64,
        var_qb1b2_db1_slot: &mut f64,
        var_qb1b2_dn0_slot: &mut f64,
        var_qb1b2_dn1_slot: &mut f64,
        var_qb1b2_dn10_slot: &mut f64,
        var_qb1b2_dn2_slot: &mut f64,
        var_qb1b2_dn3_slot: &mut f64,
        var_qb1b2_dn4_slot: &mut f64,
        var_qb1b2_dn5_slot: &mut f64,
        var_qb1b2_dn6_slot: &mut f64,
        var_qb1b2_dn7_slot: &mut f64,
        var_qb1b2_dn8_slot: &mut f64,
        var_qb1b2_dn9_slot: &mut f64,
        var_qb1b2_rdb0_slot: &mut f64,
        var_qb1b2_rdb1_slot: &mut f64,
        var_qb1b2_rdn0_slot: &mut f64,
        var_qb1b2_rdn1_slot: &mut f64,
        var_qb1b2_rdn10_slot: &mut f64,
        var_qb1b2_rdn2_slot: &mut f64,
        var_qb1b2_rdn3_slot: &mut f64,
        var_qb1b2_rdn4_slot: &mut f64,
        var_qb1b2_rdn5_slot: &mut f64,
        var_qb1b2_rdn6_slot: &mut f64,
        var_qb1b2_rdn7_slot: &mut f64,
        var_qb1b2_rdn8_slot: &mut f64,
        var_qb1b2_rdn9_slot: &mut f64,
        var_qb1b2_rv_slot: &mut f64,
        var_qbc_slot: &mut f64,
        var_qbc_db0_slot: &mut f64,
        var_qbc_db1_slot: &mut f64,
        var_qbc_dn0_slot: &mut f64,
        var_qbc_dn1_slot: &mut f64,
        var_qbc_dn10_slot: &mut f64,
        var_qbc_dn2_slot: &mut f64,
        var_qbc_dn3_slot: &mut f64,
        var_qbc_dn4_slot: &mut f64,
        var_qbc_dn5_slot: &mut f64,
        var_qbc_dn6_slot: &mut f64,
        var_qbc_dn7_slot: &mut f64,
        var_qbc_dn8_slot: &mut f64,
        var_qbc_dn9_slot: &mut f64,
        var_qbc_rdb0_slot: &mut f64,
        var_qbc_rdb1_slot: &mut f64,
        var_qbc_rdn0_slot: &mut f64,
        var_qbc_rdn1_slot: &mut f64,
        var_qbc_rdn10_slot: &mut f64,
        var_qbc_rdn2_slot: &mut f64,
        var_qbc_rdn3_slot: &mut f64,
        var_qbc_rdn4_slot: &mut f64,
        var_qbc_rdn5_slot: &mut f64,
        var_qbc_rdn6_slot: &mut f64,
        var_qbc_rdn7_slot: &mut f64,
        var_qbc_rdn8_slot: &mut f64,
        var_qbc_rdn9_slot: &mut f64,
        var_qbc_rv_slot: &mut f64,
        var_qbe_slot: &mut f64,
        var_qbe_db0_slot: &mut f64,
        var_qbe_db1_slot: &mut f64,
        var_qbe_dn0_slot: &mut f64,
        var_qbe_dn1_slot: &mut f64,
        var_qbe_dn10_slot: &mut f64,
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
        var_qbe_qs_eff_dn2_slot: &mut f64,
        var_qbe_qs_eff_dn3_slot: &mut f64,
        var_qbe_qs_eff_dn4_slot: &mut f64,
        var_qbe_qs_eff_dn5_slot: &mut f64,
        var_qbe_qs_eff_dn6_slot: &mut f64,
        var_qbe_qs_eff_dn7_slot: &mut f64,
        var_qbe_qs_eff_dn8_slot: &mut f64,
        var_qbe_qs_eff_dn9_slot: &mut f64,
        var_qbe_qs_eff_rdb0_slot: &mut f64,
        var_qbe_qs_eff_rdb1_slot: &mut f64,
        var_qbe_qs_eff_rdn0_slot: &mut f64,
        var_qbe_qs_eff_rdn1_slot: &mut f64,
        var_qbe_qs_eff_rdn10_slot: &mut f64,
        var_qbe_qs_eff_rdn2_slot: &mut f64,
        var_qbe_qs_eff_rdn3_slot: &mut f64,
        var_qbe_qs_eff_rdn4_slot: &mut f64,
        var_qbe_qs_eff_rdn5_slot: &mut f64,
        var_qbe_qs_eff_rdn6_slot: &mut f64,
        var_qbe_qs_eff_rdn7_slot: &mut f64,
        var_qbe_qs_eff_rdn8_slot: &mut f64,
        var_qbe_qs_eff_rdn9_slot: &mut f64,
        var_qbe_qs_eff_rv_slot: &mut f64,
        var_qbe_rdb0_slot: &mut f64,
        var_qbe_rdb1_slot: &mut f64,
        var_qbe_rdn0_slot: &mut f64,
        var_qbe_rdn1_slot: &mut f64,
        var_qbe_rdn10_slot: &mut f64,
        var_qbe_rdn2_slot: &mut f64,
        var_qbe_rdn3_slot: &mut f64,
        var_qbe_rdn4_slot: &mut f64,
        var_qbe_rdn5_slot: &mut f64,
        var_qbe_rdn6_slot: &mut f64,
        var_qbe_rdn7_slot: &mut f64,
        var_qbe_rdn8_slot: &mut f64,
        var_qbe_rdn9_slot: &mut f64,
        var_qbe_rv_slot: &mut f64,
        var_qe_slot: &mut f64,
        var_qe_db0_slot: &mut f64,
        var_qe_db1_slot: &mut f64,
        var_qe_dn0_slot: &mut f64,
        var_qe_dn1_slot: &mut f64,
        var_qe_dn10_slot: &mut f64,
        var_qe_dn2_slot: &mut f64,
        var_qe_dn3_slot: &mut f64,
        var_qe_dn4_slot: &mut f64,
        var_qe_dn5_slot: &mut f64,
        var_qe_dn6_slot: &mut f64,
        var_qe_dn7_slot: &mut f64,
        var_qe_dn8_slot: &mut f64,
        var_qe_dn9_slot: &mut f64,
        var_qe_rdb0_slot: &mut f64,
        var_qe_rdb1_slot: &mut f64,
        var_qe_rdn0_slot: &mut f64,
        var_qe_rdn1_slot: &mut f64,
        var_qe_rdn10_slot: &mut f64,
        var_qe_rdn2_slot: &mut f64,
        var_qe_rdn3_slot: &mut f64,
        var_qe_rdn4_slot: &mut f64,
        var_qe_rdn5_slot: &mut f64,
        var_qe_rdn6_slot: &mut f64,
        var_qe_rdn7_slot: &mut f64,
        var_qe_rdn8_slot: &mut f64,
        var_qe_rdn9_slot: &mut f64,
        var_qe_rv_slot: &mut f64,
    ) {
        let mut var_dn0vb2e1: f64 = *var_dn0vb2e1_slot;
        let mut var_dn0vb2e1_db0: f64 = *var_dn0vb2e1_db0_slot;
        let mut var_dn0vb2e1_db1: f64 = *var_dn0vb2e1_db1_slot;
        let mut var_dn0vb2e1_dn0: f64 = *var_dn0vb2e1_dn0_slot;
        let mut var_dn0vb2e1_dn1: f64 = *var_dn0vb2e1_dn1_slot;
        let mut var_dn0vb2e1_dn10: f64 = *var_dn0vb2e1_dn10_slot;
        let mut var_dn0vb2e1_dn2: f64 = *var_dn0vb2e1_dn2_slot;
        let mut var_dn0vb2e1_dn3: f64 = *var_dn0vb2e1_dn3_slot;
        let mut var_dn0vb2e1_dn4: f64 = *var_dn0vb2e1_dn4_slot;
        let mut var_dn0vb2e1_dn5: f64 = *var_dn0vb2e1_dn5_slot;
        let mut var_dn0vb2e1_dn6: f64 = *var_dn0vb2e1_dn6_slot;
        let mut var_dn0vb2e1_dn7: f64 = *var_dn0vb2e1_dn7_slot;
        let mut var_dn0vb2e1_dn8: f64 = *var_dn0vb2e1_dn8_slot;
        let mut var_dn0vb2e1_dn9: f64 = *var_dn0vb2e1_dn9_slot;
        let mut var_dn0vb2e1_rdb0: f64 = *var_dn0vb2e1_rdb0_slot;
        let mut var_dn0vb2e1_rdb1: f64 = *var_dn0vb2e1_rdb1_slot;
        let mut var_dn0vb2e1_rdn0: f64 = *var_dn0vb2e1_rdn0_slot;
        let mut var_dn0vb2e1_rdn1: f64 = *var_dn0vb2e1_rdn1_slot;
        let mut var_dn0vb2e1_rdn10: f64 = *var_dn0vb2e1_rdn10_slot;
        let mut var_dn0vb2e1_rdn2: f64 = *var_dn0vb2e1_rdn2_slot;
        let mut var_dn0vb2e1_rdn3: f64 = *var_dn0vb2e1_rdn3_slot;
        let mut var_dn0vb2e1_rdn4: f64 = *var_dn0vb2e1_rdn4_slot;
        let mut var_dn0vb2e1_rdn5: f64 = *var_dn0vb2e1_rdn5_slot;
        let mut var_dn0vb2e1_rdn6: f64 = *var_dn0vb2e1_rdn6_slot;
        let mut var_dn0vb2e1_rdn7: f64 = *var_dn0vb2e1_rdn7_slot;
        let mut var_dn0vb2e1_rdn8: f64 = *var_dn0vb2e1_rdn8_slot;
        let mut var_dn0vb2e1_rdn9: f64 = *var_dn0vb2e1_rdn9_slot;
        let mut var_dn0vb2e1_rv: f64 = *var_dn0vb2e1_rv_slot;
        let mut var_dqbevb2e1: f64 = *var_dqbevb2e1_slot;
        let mut var_dqbevb2e1_db0: f64 = *var_dqbevb2e1_db0_slot;
        let mut var_dqbevb2e1_db1: f64 = *var_dqbevb2e1_db1_slot;
        let mut var_dqbevb2e1_dn0: f64 = *var_dqbevb2e1_dn0_slot;
        let mut var_dqbevb2e1_dn1: f64 = *var_dqbevb2e1_dn1_slot;
        let mut var_dqbevb2e1_dn10: f64 = *var_dqbevb2e1_dn10_slot;
        let mut var_dqbevb2e1_dn2: f64 = *var_dqbevb2e1_dn2_slot;
        let mut var_dqbevb2e1_dn3: f64 = *var_dqbevb2e1_dn3_slot;
        let mut var_dqbevb2e1_dn4: f64 = *var_dqbevb2e1_dn4_slot;
        let mut var_dqbevb2e1_dn5: f64 = *var_dqbevb2e1_dn5_slot;
        let mut var_dqbevb2e1_dn6: f64 = *var_dqbevb2e1_dn6_slot;
        let mut var_dqbevb2e1_dn7: f64 = *var_dqbevb2e1_dn7_slot;
        let mut var_dqbevb2e1_dn8: f64 = *var_dqbevb2e1_dn8_slot;
        let mut var_dqbevb2e1_dn9: f64 = *var_dqbevb2e1_dn9_slot;
        let mut var_dqbevb2e1_rdb0: f64 = *var_dqbevb2e1_rdb0_slot;
        let mut var_dqbevb2e1_rdb1: f64 = *var_dqbevb2e1_rdb1_slot;
        let mut var_dqbevb2e1_rdn0: f64 = *var_dqbevb2e1_rdn0_slot;
        let mut var_dqbevb2e1_rdn1: f64 = *var_dqbevb2e1_rdn1_slot;
        let mut var_dqbevb2e1_rdn10: f64 = *var_dqbevb2e1_rdn10_slot;
        let mut var_dqbevb2e1_rdn2: f64 = *var_dqbevb2e1_rdn2_slot;
        let mut var_dqbevb2e1_rdn3: f64 = *var_dqbevb2e1_rdn3_slot;
        let mut var_dqbevb2e1_rdn4: f64 = *var_dqbevb2e1_rdn4_slot;
        let mut var_dqbevb2e1_rdn5: f64 = *var_dqbevb2e1_rdn5_slot;
        let mut var_dqbevb2e1_rdn6: f64 = *var_dqbevb2e1_rdn6_slot;
        let mut var_dqbevb2e1_rdn7: f64 = *var_dqbevb2e1_rdn7_slot;
        let mut var_dqbevb2e1_rdn8: f64 = *var_dqbevb2e1_rdn8_slot;
        let mut var_dqbevb2e1_rdn9: f64 = *var_dqbevb2e1_rdn9_slot;
        let mut var_dqbevb2e1_rv: f64 = *var_dqbevb2e1_rv_slot;
        let mut var_dqevb2e1: f64 = *var_dqevb2e1_slot;
        let mut var_dqevb2e1_db0: f64 = *var_dqevb2e1_db0_slot;
        let mut var_dqevb2e1_db1: f64 = *var_dqevb2e1_db1_slot;
        let mut var_dqevb2e1_dn0: f64 = *var_dqevb2e1_dn0_slot;
        let mut var_dqevb2e1_dn1: f64 = *var_dqevb2e1_dn1_slot;
        let mut var_dqevb2e1_dn10: f64 = *var_dqevb2e1_dn10_slot;
        let mut var_dqevb2e1_dn2: f64 = *var_dqevb2e1_dn2_slot;
        let mut var_dqevb2e1_dn3: f64 = *var_dqevb2e1_dn3_slot;
        let mut var_dqevb2e1_dn4: f64 = *var_dqevb2e1_dn4_slot;
        let mut var_dqevb2e1_dn5: f64 = *var_dqevb2e1_dn5_slot;
        let mut var_dqevb2e1_dn6: f64 = *var_dqevb2e1_dn6_slot;
        let mut var_dqevb2e1_dn7: f64 = *var_dqevb2e1_dn7_slot;
        let mut var_dqevb2e1_dn8: f64 = *var_dqevb2e1_dn8_slot;
        let mut var_dqevb2e1_dn9: f64 = *var_dqevb2e1_dn9_slot;
        let mut var_dqevb2e1_rdb0: f64 = *var_dqevb2e1_rdb0_slot;
        let mut var_dqevb2e1_rdb1: f64 = *var_dqevb2e1_rdb1_slot;
        let mut var_dqevb2e1_rdn0: f64 = *var_dqevb2e1_rdn0_slot;
        let mut var_dqevb2e1_rdn1: f64 = *var_dqevb2e1_rdn1_slot;
        let mut var_dqevb2e1_rdn10: f64 = *var_dqevb2e1_rdn10_slot;
        let mut var_dqevb2e1_rdn2: f64 = *var_dqevb2e1_rdn2_slot;
        let mut var_dqevb2e1_rdn3: f64 = *var_dqevb2e1_rdn3_slot;
        let mut var_dqevb2e1_rdn4: f64 = *var_dqevb2e1_rdn4_slot;
        let mut var_dqevb2e1_rdn5: f64 = *var_dqevb2e1_rdn5_slot;
        let mut var_dqevb2e1_rdn6: f64 = *var_dqevb2e1_rdn6_slot;
        let mut var_dqevb2e1_rdn7: f64 = *var_dqevb2e1_rdn7_slot;
        let mut var_dqevb2e1_rdn8: f64 = *var_dqevb2e1_rdn8_slot;
        let mut var_dqevb2e1_rdn9: f64 = *var_dqevb2e1_rdn9_slot;
        let mut var_dqevb2e1_rv: f64 = *var_dqevb2e1_rv_slot;
        let mut var_dqtevb2e1: f64 = *var_dqtevb2e1_slot;
        let mut var_dqtevb2e1_db0: f64 = *var_dqtevb2e1_db0_slot;
        let mut var_dqtevb2e1_db1: f64 = *var_dqtevb2e1_db1_slot;
        let mut var_dqtevb2e1_dn0: f64 = *var_dqtevb2e1_dn0_slot;
        let mut var_dqtevb2e1_dn1: f64 = *var_dqtevb2e1_dn1_slot;
        let mut var_dqtevb2e1_dn10: f64 = *var_dqtevb2e1_dn10_slot;
        let mut var_dqtevb2e1_dn2: f64 = *var_dqtevb2e1_dn2_slot;
        let mut var_dqtevb2e1_dn3: f64 = *var_dqtevb2e1_dn3_slot;
        let mut var_dqtevb2e1_dn4: f64 = *var_dqtevb2e1_dn4_slot;
        let mut var_dqtevb2e1_dn5: f64 = *var_dqtevb2e1_dn5_slot;
        let mut var_dqtevb2e1_dn6: f64 = *var_dqtevb2e1_dn6_slot;
        let mut var_dqtevb2e1_dn7: f64 = *var_dqtevb2e1_dn7_slot;
        let mut var_dqtevb2e1_dn8: f64 = *var_dqtevb2e1_dn8_slot;
        let mut var_dqtevb2e1_dn9: f64 = *var_dqtevb2e1_dn9_slot;
        let mut var_dqtevb2e1_rdb0: f64 = *var_dqtevb2e1_rdb0_slot;
        let mut var_dqtevb2e1_rdb1: f64 = *var_dqtevb2e1_rdb1_slot;
        let mut var_dqtevb2e1_rdn0: f64 = *var_dqtevb2e1_rdn0_slot;
        let mut var_dqtevb2e1_rdn1: f64 = *var_dqtevb2e1_rdn1_slot;
        let mut var_dqtevb2e1_rdn10: f64 = *var_dqtevb2e1_rdn10_slot;
        let mut var_dqtevb2e1_rdn2: f64 = *var_dqtevb2e1_rdn2_slot;
        let mut var_dqtevb2e1_rdn3: f64 = *var_dqtevb2e1_rdn3_slot;
        let mut var_dqtevb2e1_rdn4: f64 = *var_dqtevb2e1_rdn4_slot;
        let mut var_dqtevb2e1_rdn5: f64 = *var_dqtevb2e1_rdn5_slot;
        let mut var_dqtevb2e1_rdn6: f64 = *var_dqtevb2e1_rdn6_slot;
        let mut var_dqtevb2e1_rdn7: f64 = *var_dqtevb2e1_rdn7_slot;
        let mut var_dqtevb2e1_rdn8: f64 = *var_dqtevb2e1_rdn8_slot;
        let mut var_dqtevb2e1_rdn9: f64 = *var_dqtevb2e1_rdn9_slot;
        let mut var_dqtevb2e1_rv: f64 = *var_dqtevb2e1_rv_slot;
        let mut var_in_n: f64 = *var_in_n_slot;
        let mut var_in_n_db0: f64 = *var_in_n_db0_slot;
        let mut var_in_n_db1: f64 = *var_in_n_db1_slot;
        let mut var_in_n_dn0: f64 = *var_in_n_dn0_slot;
        let mut var_in_n_dn1: f64 = *var_in_n_dn1_slot;
        let mut var_in_n_dn10: f64 = *var_in_n_dn10_slot;
        let mut var_in_n_dn2: f64 = *var_in_n_dn2_slot;
        let mut var_in_n_dn3: f64 = *var_in_n_dn3_slot;
        let mut var_in_n_dn4: f64 = *var_in_n_dn4_slot;
        let mut var_in_n_dn5: f64 = *var_in_n_dn5_slot;
        let mut var_in_n_dn6: f64 = *var_in_n_dn6_slot;
        let mut var_in_n_dn7: f64 = *var_in_n_dn7_slot;
        let mut var_in_n_dn8: f64 = *var_in_n_dn8_slot;
        let mut var_in_n_dn9: f64 = *var_in_n_dn9_slot;
        let mut var_in_n_rdb0: f64 = *var_in_n_rdb0_slot;
        let mut var_in_n_rdb1: f64 = *var_in_n_rdb1_slot;
        let mut var_in_n_rdn0: f64 = *var_in_n_rdn0_slot;
        let mut var_in_n_rdn1: f64 = *var_in_n_rdn1_slot;
        let mut var_in_n_rdn10: f64 = *var_in_n_rdn10_slot;
        let mut var_in_n_rdn2: f64 = *var_in_n_rdn2_slot;
        let mut var_in_n_rdn3: f64 = *var_in_n_rdn3_slot;
        let mut var_in_n_rdn4: f64 = *var_in_n_rdn4_slot;
        let mut var_in_n_rdn5: f64 = *var_in_n_rdn5_slot;
        let mut var_in_n_rdn6: f64 = *var_in_n_rdn6_slot;
        let mut var_in_n_rdn7: f64 = *var_in_n_rdn7_slot;
        let mut var_in_n_rdn8: f64 = *var_in_n_rdn8_slot;
        let mut var_in_n_rdn9: f64 = *var_in_n_rdn9_slot;
        let mut var_in_n_rv: f64 = *var_in_n_rv_slot;
        let mut var_qb1b2: f64 = *var_qb1b2_slot;
        let mut var_qb1b2_db0: f64 = *var_qb1b2_db0_slot;
        let mut var_qb1b2_db1: f64 = *var_qb1b2_db1_slot;
        let mut var_qb1b2_dn0: f64 = *var_qb1b2_dn0_slot;
        let mut var_qb1b2_dn1: f64 = *var_qb1b2_dn1_slot;
        let mut var_qb1b2_dn10: f64 = *var_qb1b2_dn10_slot;
        let mut var_qb1b2_dn2: f64 = *var_qb1b2_dn2_slot;
        let mut var_qb1b2_dn3: f64 = *var_qb1b2_dn3_slot;
        let mut var_qb1b2_dn4: f64 = *var_qb1b2_dn4_slot;
        let mut var_qb1b2_dn5: f64 = *var_qb1b2_dn5_slot;
        let mut var_qb1b2_dn6: f64 = *var_qb1b2_dn6_slot;
        let mut var_qb1b2_dn7: f64 = *var_qb1b2_dn7_slot;
        let mut var_qb1b2_dn8: f64 = *var_qb1b2_dn8_slot;
        let mut var_qb1b2_dn9: f64 = *var_qb1b2_dn9_slot;
        let mut var_qb1b2_rdb0: f64 = *var_qb1b2_rdb0_slot;
        let mut var_qb1b2_rdb1: f64 = *var_qb1b2_rdb1_slot;
        let mut var_qb1b2_rdn0: f64 = *var_qb1b2_rdn0_slot;
        let mut var_qb1b2_rdn1: f64 = *var_qb1b2_rdn1_slot;
        let mut var_qb1b2_rdn10: f64 = *var_qb1b2_rdn10_slot;
        let mut var_qb1b2_rdn2: f64 = *var_qb1b2_rdn2_slot;
        let mut var_qb1b2_rdn3: f64 = *var_qb1b2_rdn3_slot;
        let mut var_qb1b2_rdn4: f64 = *var_qb1b2_rdn4_slot;
        let mut var_qb1b2_rdn5: f64 = *var_qb1b2_rdn5_slot;
        let mut var_qb1b2_rdn6: f64 = *var_qb1b2_rdn6_slot;
        let mut var_qb1b2_rdn7: f64 = *var_qb1b2_rdn7_slot;
        let mut var_qb1b2_rdn8: f64 = *var_qb1b2_rdn8_slot;
        let mut var_qb1b2_rdn9: f64 = *var_qb1b2_rdn9_slot;
        let mut var_qb1b2_rv: f64 = *var_qb1b2_rv_slot;
        let mut var_qbc: f64 = *var_qbc_slot;
        let mut var_qbc_db0: f64 = *var_qbc_db0_slot;
        let mut var_qbc_db1: f64 = *var_qbc_db1_slot;
        let mut var_qbc_dn0: f64 = *var_qbc_dn0_slot;
        let mut var_qbc_dn1: f64 = *var_qbc_dn1_slot;
        let mut var_qbc_dn10: f64 = *var_qbc_dn10_slot;
        let mut var_qbc_dn2: f64 = *var_qbc_dn2_slot;
        let mut var_qbc_dn3: f64 = *var_qbc_dn3_slot;
        let mut var_qbc_dn4: f64 = *var_qbc_dn4_slot;
        let mut var_qbc_dn5: f64 = *var_qbc_dn5_slot;
        let mut var_qbc_dn6: f64 = *var_qbc_dn6_slot;
        let mut var_qbc_dn7: f64 = *var_qbc_dn7_slot;
        let mut var_qbc_dn8: f64 = *var_qbc_dn8_slot;
        let mut var_qbc_dn9: f64 = *var_qbc_dn9_slot;
        let mut var_qbc_rdb0: f64 = *var_qbc_rdb0_slot;
        let mut var_qbc_rdb1: f64 = *var_qbc_rdb1_slot;
        let mut var_qbc_rdn0: f64 = *var_qbc_rdn0_slot;
        let mut var_qbc_rdn1: f64 = *var_qbc_rdn1_slot;
        let mut var_qbc_rdn10: f64 = *var_qbc_rdn10_slot;
        let mut var_qbc_rdn2: f64 = *var_qbc_rdn2_slot;
        let mut var_qbc_rdn3: f64 = *var_qbc_rdn3_slot;
        let mut var_qbc_rdn4: f64 = *var_qbc_rdn4_slot;
        let mut var_qbc_rdn5: f64 = *var_qbc_rdn5_slot;
        let mut var_qbc_rdn6: f64 = *var_qbc_rdn6_slot;
        let mut var_qbc_rdn7: f64 = *var_qbc_rdn7_slot;
        let mut var_qbc_rdn8: f64 = *var_qbc_rdn8_slot;
        let mut var_qbc_rdn9: f64 = *var_qbc_rdn9_slot;
        let mut var_qbc_rv: f64 = *var_qbc_rv_slot;
        let mut var_qbe: f64 = *var_qbe_slot;
        let mut var_qbe_db0: f64 = *var_qbe_db0_slot;
        let mut var_qbe_db1: f64 = *var_qbe_db1_slot;
        let mut var_qbe_dn0: f64 = *var_qbe_dn0_slot;
        let mut var_qbe_dn1: f64 = *var_qbe_dn1_slot;
        let mut var_qbe_dn10: f64 = *var_qbe_dn10_slot;
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
        let mut var_qbe_qs_eff_dn2: f64 = *var_qbe_qs_eff_dn2_slot;
        let mut var_qbe_qs_eff_dn3: f64 = *var_qbe_qs_eff_dn3_slot;
        let mut var_qbe_qs_eff_dn4: f64 = *var_qbe_qs_eff_dn4_slot;
        let mut var_qbe_qs_eff_dn5: f64 = *var_qbe_qs_eff_dn5_slot;
        let mut var_qbe_qs_eff_dn6: f64 = *var_qbe_qs_eff_dn6_slot;
        let mut var_qbe_qs_eff_dn7: f64 = *var_qbe_qs_eff_dn7_slot;
        let mut var_qbe_qs_eff_dn8: f64 = *var_qbe_qs_eff_dn8_slot;
        let mut var_qbe_qs_eff_dn9: f64 = *var_qbe_qs_eff_dn9_slot;
        let mut var_qbe_qs_eff_rdb0: f64 = *var_qbe_qs_eff_rdb0_slot;
        let mut var_qbe_qs_eff_rdb1: f64 = *var_qbe_qs_eff_rdb1_slot;
        let mut var_qbe_qs_eff_rdn0: f64 = *var_qbe_qs_eff_rdn0_slot;
        let mut var_qbe_qs_eff_rdn1: f64 = *var_qbe_qs_eff_rdn1_slot;
        let mut var_qbe_qs_eff_rdn10: f64 = *var_qbe_qs_eff_rdn10_slot;
        let mut var_qbe_qs_eff_rdn2: f64 = *var_qbe_qs_eff_rdn2_slot;
        let mut var_qbe_qs_eff_rdn3: f64 = *var_qbe_qs_eff_rdn3_slot;
        let mut var_qbe_qs_eff_rdn4: f64 = *var_qbe_qs_eff_rdn4_slot;
        let mut var_qbe_qs_eff_rdn5: f64 = *var_qbe_qs_eff_rdn5_slot;
        let mut var_qbe_qs_eff_rdn6: f64 = *var_qbe_qs_eff_rdn6_slot;
        let mut var_qbe_qs_eff_rdn7: f64 = *var_qbe_qs_eff_rdn7_slot;
        let mut var_qbe_qs_eff_rdn8: f64 = *var_qbe_qs_eff_rdn8_slot;
        let mut var_qbe_qs_eff_rdn9: f64 = *var_qbe_qs_eff_rdn9_slot;
        let mut var_qbe_qs_eff_rv: f64 = *var_qbe_qs_eff_rv_slot;
        let mut var_qbe_rdb0: f64 = *var_qbe_rdb0_slot;
        let mut var_qbe_rdb1: f64 = *var_qbe_rdb1_slot;
        let mut var_qbe_rdn0: f64 = *var_qbe_rdn0_slot;
        let mut var_qbe_rdn1: f64 = *var_qbe_rdn1_slot;
        let mut var_qbe_rdn10: f64 = *var_qbe_rdn10_slot;
        let mut var_qbe_rdn2: f64 = *var_qbe_rdn2_slot;
        let mut var_qbe_rdn3: f64 = *var_qbe_rdn3_slot;
        let mut var_qbe_rdn4: f64 = *var_qbe_rdn4_slot;
        let mut var_qbe_rdn5: f64 = *var_qbe_rdn5_slot;
        let mut var_qbe_rdn6: f64 = *var_qbe_rdn6_slot;
        let mut var_qbe_rdn7: f64 = *var_qbe_rdn7_slot;
        let mut var_qbe_rdn8: f64 = *var_qbe_rdn8_slot;
        let mut var_qbe_rdn9: f64 = *var_qbe_rdn9_slot;
        let mut var_qbe_rv: f64 = *var_qbe_rv_slot;
        let mut var_qe: f64 = *var_qe_slot;
        let mut var_qe_db0: f64 = *var_qe_db0_slot;
        let mut var_qe_db1: f64 = *var_qe_db1_slot;
        let mut var_qe_dn0: f64 = *var_qe_dn0_slot;
        let mut var_qe_dn1: f64 = *var_qe_dn1_slot;
        let mut var_qe_dn10: f64 = *var_qe_dn10_slot;
        let mut var_qe_dn2: f64 = *var_qe_dn2_slot;
        let mut var_qe_dn3: f64 = *var_qe_dn3_slot;
        let mut var_qe_dn4: f64 = *var_qe_dn4_slot;
        let mut var_qe_dn5: f64 = *var_qe_dn5_slot;
        let mut var_qe_dn6: f64 = *var_qe_dn6_slot;
        let mut var_qe_dn7: f64 = *var_qe_dn7_slot;
        let mut var_qe_dn8: f64 = *var_qe_dn8_slot;
        let mut var_qe_dn9: f64 = *var_qe_dn9_slot;
        let mut var_qe_rdb0: f64 = *var_qe_rdb0_slot;
        let mut var_qe_rdb1: f64 = *var_qe_rdb1_slot;
        let mut var_qe_rdn0: f64 = *var_qe_rdn0_slot;
        let mut var_qe_rdn1: f64 = *var_qe_rdn1_slot;
        let mut var_qe_rdn10: f64 = *var_qe_rdn10_slot;
        let mut var_qe_rdn2: f64 = *var_qe_rdn2_slot;
        let mut var_qe_rdn3: f64 = *var_qe_rdn3_slot;
        let mut var_qe_rdn4: f64 = *var_qe_rdn4_slot;
        let mut var_qe_rdn5: f64 = *var_qe_rdn5_slot;
        let mut var_qe_rdn6: f64 = *var_qe_rdn6_slot;
        let mut var_qe_rdn7: f64 = *var_qe_rdn7_slot;
        let mut var_qe_rdn8: f64 = *var_qe_rdn8_slot;
        let mut var_qe_rdn9: f64 = *var_qe_rdn9_slot;
        let mut var_qe_rv: f64 = *var_qe_rv_slot;

        let (assign6220_e6333, assign6220_e6333_d_n0, assign6220_e6333_d_n1, assign6220_e6333_d_n2, assign6220_e6333_d_n3, assign6220_e6333_d_n4, assign6220_e6333_d_n5, assign6220_e6333_d_n6, assign6220_e6333_d_n7, assign6220_e6333_d_n8, assign6220_e6333_d_n9, assign6220_e6333_d_n10, assign6220_e6333_d_b0, assign6220_e6333_d_b1,) = {
    if (var_guard112 != 0.0) {
        let assign6220_e6327: f64 = (1.0 - p.p67);
        let assign6220_e6329: f64 = (assign6220_e6327 * var_cje_t);
        let assign6220_e6331: f64 = (assign6220_e6329 * var_dvtevb2e1);
        (assign6220_e6331, (((assign6220_e6327 * var_cje_t_dn0) * var_dvtevb2e1) + (assign6220_e6329 * var_dvtevb2e1_dn0)), (((assign6220_e6327 * var_cje_t_dn1) * var_dvtevb2e1) + (assign6220_e6329 * var_dvtevb2e1_dn1)), (((assign6220_e6327 * var_cje_t_dn2) * var_dvtevb2e1) + (assign6220_e6329 * var_dvtevb2e1_dn2)), (((assign6220_e6327 * var_cje_t_dn3) * var_dvtevb2e1) + (assign6220_e6329 * var_dvtevb2e1_dn3)), (((assign6220_e6327 * var_cje_t_dn4) * var_dvtevb2e1) + (assign6220_e6329 * var_dvtevb2e1_dn4)), (((assign6220_e6327 * var_cje_t_dn5) * var_dvtevb2e1) + (assign6220_e6329 * var_dvtevb2e1_dn5)), (((assign6220_e6327 * var_cje_t_dn6) * var_dvtevb2e1) + (assign6220_e6329 * var_dvtevb2e1_dn6)), (((assign6220_e6327 * var_cje_t_dn7) * var_dvtevb2e1) + (assign6220_e6329 * var_dvtevb2e1_dn7)), (((assign6220_e6327 * var_cje_t_dn8) * var_dvtevb2e1) + (assign6220_e6329 * var_dvtevb2e1_dn8)), (((assign6220_e6327 * var_cje_t_dn9) * var_dvtevb2e1) + (assign6220_e6329 * var_dvtevb2e1_dn9)), (((assign6220_e6327 * var_cje_t_dn10) * var_dvtevb2e1) + (assign6220_e6329 * var_dvtevb2e1_dn10)), (((assign6220_e6327 * var_cje_t_db0) * var_dvtevb2e1) + (assign6220_e6329 * var_dvtevb2e1_db0)), (((assign6220_e6327 * var_cje_t_db1) * var_dvtevb2e1) + (assign6220_e6329 * var_dvtevb2e1_db1)),)
    } else {
        (var_dqtevb2e1, var_dqtevb2e1_dn0, var_dqtevb2e1_dn1, var_dqtevb2e1_dn2, var_dqtevb2e1_dn3, var_dqtevb2e1_dn4, var_dqtevb2e1_dn5, var_dqtevb2e1_dn6, var_dqtevb2e1_dn7, var_dqtevb2e1_dn8, var_dqtevb2e1_dn9, var_dqtevb2e1_dn10, var_dqtevb2e1_db0, var_dqtevb2e1_db1,)
    }
};
        var_dqtevb2e1 = assign6220_e6333;
        var_dqtevb2e1_dn0 = assign6220_e6333_d_n0;
        var_dqtevb2e1_dn1 = assign6220_e6333_d_n1;
        var_dqtevb2e1_dn2 = assign6220_e6333_d_n2;
        var_dqtevb2e1_dn3 = assign6220_e6333_d_n3;
        var_dqtevb2e1_dn4 = assign6220_e6333_d_n4;
        var_dqtevb2e1_dn5 = assign6220_e6333_d_n5;
        var_dqtevb2e1_dn6 = assign6220_e6333_d_n6;
        var_dqtevb2e1_dn7 = assign6220_e6333_d_n7;
        var_dqtevb2e1_dn8 = assign6220_e6333_d_n8;
        var_dqtevb2e1_dn9 = assign6220_e6333_d_n9;
        var_dqtevb2e1_dn10 = assign6220_e6333_d_n10;
        var_dqtevb2e1_db0 = assign6220_e6333_d_b0;
        var_dqtevb2e1_db1 = assign6220_e6333_d_b1;
        var_dqtevb2e1_rv = 0.0;
        var_dqtevb2e1_rdn0 = 0.0;
        var_dqtevb2e1_rdn1 = 0.0;
        var_dqtevb2e1_rdn2 = 0.0;
        var_dqtevb2e1_rdn3 = 0.0;
        var_dqtevb2e1_rdn4 = 0.0;
        var_dqtevb2e1_rdn5 = 0.0;
        var_dqtevb2e1_rdn6 = 0.0;
        var_dqtevb2e1_rdn7 = 0.0;
        var_dqtevb2e1_rdn8 = 0.0;
        var_dqtevb2e1_rdn9 = 0.0;
        var_dqtevb2e1_rdn10 = 0.0;
        var_dqtevb2e1_rdb0 = 0.0;
        var_dqtevb2e1_rdb1 = 0.0;

        let (assign6230_e6350, assign6230_e6350_d_n0, assign6230_e6350_d_n1, assign6230_e6350_d_n2, assign6230_e6350_d_n3, assign6230_e6350_d_n4, assign6230_e6350_d_n5, assign6230_e6350_d_n6, assign6230_e6350_d_n7, assign6230_e6350_d_n8, assign6230_e6350_d_n9, assign6230_e6350_d_n10, assign6230_e6350_d_b0, assign6230_e6350_d_b1,) = {
    if (var_guard112 != 0.0) {
        let assign6230_e6337: f64 = (var_if0 * var_evb2e1);
        let assign6230_e6339: f64 = (assign6230_e6337 * var_vtinv);
        let assign6230_e6341: f64 = (assign6230_e6339 / var_nff_t);
        let assign6230_e6345: f64 = (1.0 + var_f1);
        let assign6230_e6346: f64 = (assign6230_e6345).sqrt();
        let assign6230_e6347: f64 = (0.5 / assign6230_e6346);
        let assign6230_e6348: f64 = (assign6230_e6341 * assign6230_e6347);
        (assign6230_e6348, (((((((((var_if0_dn0 * var_evb2e1) + (var_if0 * var_evb2e1_dn0)) * var_vtinv) + (assign6230_e6337 * var_vtinv_dn0)) * var_nff_t) - (assign6230_e6339 * var_nff_t_dn0)) / (var_nff_t * var_nff_t)) * assign6230_e6347) + (assign6230_e6341 * (-((0.5 * (var_f1_dn0 / (2.0 * assign6230_e6346))) / (assign6230_e6346 * assign6230_e6346))))), (((((((((var_if0_dn1 * var_evb2e1) + (var_if0 * var_evb2e1_dn1)) * var_vtinv) + (assign6230_e6337 * var_vtinv_dn1)) * var_nff_t) - (assign6230_e6339 * var_nff_t_dn1)) / (var_nff_t * var_nff_t)) * assign6230_e6347) + (assign6230_e6341 * (-((0.5 * (var_f1_dn1 / (2.0 * assign6230_e6346))) / (assign6230_e6346 * assign6230_e6346))))), (((((((((var_if0_dn2 * var_evb2e1) + (var_if0 * var_evb2e1_dn2)) * var_vtinv) + (assign6230_e6337 * var_vtinv_dn2)) * var_nff_t) - (assign6230_e6339 * var_nff_t_dn2)) / (var_nff_t * var_nff_t)) * assign6230_e6347) + (assign6230_e6341 * (-((0.5 * (var_f1_dn2 / (2.0 * assign6230_e6346))) / (assign6230_e6346 * assign6230_e6346))))), (((((((((var_if0_dn3 * var_evb2e1) + (var_if0 * var_evb2e1_dn3)) * var_vtinv) + (assign6230_e6337 * var_vtinv_dn3)) * var_nff_t) - (assign6230_e6339 * var_nff_t_dn3)) / (var_nff_t * var_nff_t)) * assign6230_e6347) + (assign6230_e6341 * (-((0.5 * (var_f1_dn3 / (2.0 * assign6230_e6346))) / (assign6230_e6346 * assign6230_e6346))))), (((((((((var_if0_dn4 * var_evb2e1) + (var_if0 * var_evb2e1_dn4)) * var_vtinv) + (assign6230_e6337 * var_vtinv_dn4)) * var_nff_t) - (assign6230_e6339 * var_nff_t_dn4)) / (var_nff_t * var_nff_t)) * assign6230_e6347) + (assign6230_e6341 * (-((0.5 * (var_f1_dn4 / (2.0 * assign6230_e6346))) / (assign6230_e6346 * assign6230_e6346))))), (((((((((var_if0_dn5 * var_evb2e1) + (var_if0 * var_evb2e1_dn5)) * var_vtinv) + (assign6230_e6337 * var_vtinv_dn5)) * var_nff_t) - (assign6230_e6339 * var_nff_t_dn5)) / (var_nff_t * var_nff_t)) * assign6230_e6347) + (assign6230_e6341 * (-((0.5 * (var_f1_dn5 / (2.0 * assign6230_e6346))) / (assign6230_e6346 * assign6230_e6346))))), (((((((((var_if0_dn6 * var_evb2e1) + (var_if0 * var_evb2e1_dn6)) * var_vtinv) + (assign6230_e6337 * var_vtinv_dn6)) * var_nff_t) - (assign6230_e6339 * var_nff_t_dn6)) / (var_nff_t * var_nff_t)) * assign6230_e6347) + (assign6230_e6341 * (-((0.5 * (var_f1_dn6 / (2.0 * assign6230_e6346))) / (assign6230_e6346 * assign6230_e6346))))), (((((((((var_if0_dn7 * var_evb2e1) + (var_if0 * var_evb2e1_dn7)) * var_vtinv) + (assign6230_e6337 * var_vtinv_dn7)) * var_nff_t) - (assign6230_e6339 * var_nff_t_dn7)) / (var_nff_t * var_nff_t)) * assign6230_e6347) + (assign6230_e6341 * (-((0.5 * (var_f1_dn7 / (2.0 * assign6230_e6346))) / (assign6230_e6346 * assign6230_e6346))))), (((((((((var_if0_dn8 * var_evb2e1) + (var_if0 * var_evb2e1_dn8)) * var_vtinv) + (assign6230_e6337 * var_vtinv_dn8)) * var_nff_t) - (assign6230_e6339 * var_nff_t_dn8)) / (var_nff_t * var_nff_t)) * assign6230_e6347) + (assign6230_e6341 * (-((0.5 * (var_f1_dn8 / (2.0 * assign6230_e6346))) / (assign6230_e6346 * assign6230_e6346))))), (((((((((var_if0_dn9 * var_evb2e1) + (var_if0 * var_evb2e1_dn9)) * var_vtinv) + (assign6230_e6337 * var_vtinv_dn9)) * var_nff_t) - (assign6230_e6339 * var_nff_t_dn9)) / (var_nff_t * var_nff_t)) * assign6230_e6347) + (assign6230_e6341 * (-((0.5 * (var_f1_dn9 / (2.0 * assign6230_e6346))) / (assign6230_e6346 * assign6230_e6346))))), (((((((((var_if0_dn10 * var_evb2e1) + (var_if0 * var_evb2e1_dn10)) * var_vtinv) + (assign6230_e6337 * var_vtinv_dn10)) * var_nff_t) - (assign6230_e6339 * var_nff_t_dn10)) / (var_nff_t * var_nff_t)) * assign6230_e6347) + (assign6230_e6341 * (-((0.5 * (var_f1_dn10 / (2.0 * assign6230_e6346))) / (assign6230_e6346 * assign6230_e6346))))), (((((((((var_if0_db0 * var_evb2e1) + (var_if0 * var_evb2e1_db0)) * var_vtinv) + (assign6230_e6337 * var_vtinv_db0)) * var_nff_t) - (assign6230_e6339 * var_nff_t_db0)) / (var_nff_t * var_nff_t)) * assign6230_e6347) + (assign6230_e6341 * (-((0.5 * (var_f1_db0 / (2.0 * assign6230_e6346))) / (assign6230_e6346 * assign6230_e6346))))), (((((((((var_if0_db1 * var_evb2e1) + (var_if0 * var_evb2e1_db1)) * var_vtinv) + (assign6230_e6337 * var_vtinv_db1)) * var_nff_t) - (assign6230_e6339 * var_nff_t_db1)) / (var_nff_t * var_nff_t)) * assign6230_e6347) + (assign6230_e6341 * (-((0.5 * (var_f1_db1 / (2.0 * assign6230_e6346))) / (assign6230_e6346 * assign6230_e6346))))),)
    } else {
        (var_dn0vb2e1, var_dn0vb2e1_dn0, var_dn0vb2e1_dn1, var_dn0vb2e1_dn2, var_dn0vb2e1_dn3, var_dn0vb2e1_dn4, var_dn0vb2e1_dn5, var_dn0vb2e1_dn6, var_dn0vb2e1_dn7, var_dn0vb2e1_dn8, var_dn0vb2e1_dn9, var_dn0vb2e1_dn10, var_dn0vb2e1_db0, var_dn0vb2e1_db1,)
    }
};
        var_dn0vb2e1 = assign6230_e6350;
        var_dn0vb2e1_dn0 = assign6230_e6350_d_n0;
        var_dn0vb2e1_dn1 = assign6230_e6350_d_n1;
        var_dn0vb2e1_dn2 = assign6230_e6350_d_n2;
        var_dn0vb2e1_dn3 = assign6230_e6350_d_n3;
        var_dn0vb2e1_dn4 = assign6230_e6350_d_n4;
        var_dn0vb2e1_dn5 = assign6230_e6350_d_n5;
        var_dn0vb2e1_dn6 = assign6230_e6350_d_n6;
        var_dn0vb2e1_dn7 = assign6230_e6350_d_n7;
        var_dn0vb2e1_dn8 = assign6230_e6350_d_n8;
        var_dn0vb2e1_dn9 = assign6230_e6350_d_n9;
        var_dn0vb2e1_dn10 = assign6230_e6350_d_n10;
        var_dn0vb2e1_db0 = assign6230_e6350_d_b0;
        var_dn0vb2e1_db1 = assign6230_e6350_d_b1;
        var_dn0vb2e1_rv = 0.0;
        var_dn0vb2e1_rdn0 = 0.0;
        var_dn0vb2e1_rdn1 = 0.0;
        var_dn0vb2e1_rdn2 = 0.0;
        var_dn0vb2e1_rdn3 = 0.0;
        var_dn0vb2e1_rdn4 = 0.0;
        var_dn0vb2e1_rdn5 = 0.0;
        var_dn0vb2e1_rdn6 = 0.0;
        var_dn0vb2e1_rdn7 = 0.0;
        var_dn0vb2e1_rdn8 = 0.0;
        var_dn0vb2e1_rdn9 = 0.0;
        var_dn0vb2e1_rdn10 = 0.0;
        var_dn0vb2e1_rdb0 = 0.0;
        var_dn0vb2e1_rdb1 = 0.0;

        let (assign6240_e6360, assign6240_e6360_d_n0, assign6240_e6360_d_n1, assign6240_e6360_d_n2, assign6240_e6360_d_n3, assign6240_e6360_d_n4, assign6240_e6360_d_n5, assign6240_e6360_d_n6, assign6240_e6360_d_n7, assign6240_e6360_d_n8, assign6240_e6360_d_n9, assign6240_e6360_d_n10, assign6240_e6360_d_b0, assign6240_e6360_d_b1,) = {
    if (var_guard112 != 0.0) {
        let assign6240_e6354: f64 = (0.5 * var_qb0);
        let assign6240_e6356: f64 = (assign6240_e6354 * var_q1q);
        let assign6240_e6358: f64 = (assign6240_e6356 * var_dn0vb2e1);
        (assign6240_e6358, (((((0.5 * var_qb0_dn0) * var_q1q) + (assign6240_e6354 * var_q1q_dn0)) * var_dn0vb2e1) + (assign6240_e6356 * var_dn0vb2e1_dn0)), (((((0.5 * var_qb0_dn1) * var_q1q) + (assign6240_e6354 * var_q1q_dn1)) * var_dn0vb2e1) + (assign6240_e6356 * var_dn0vb2e1_dn1)), (((((0.5 * var_qb0_dn2) * var_q1q) + (assign6240_e6354 * var_q1q_dn2)) * var_dn0vb2e1) + (assign6240_e6356 * var_dn0vb2e1_dn2)), (((((0.5 * var_qb0_dn3) * var_q1q) + (assign6240_e6354 * var_q1q_dn3)) * var_dn0vb2e1) + (assign6240_e6356 * var_dn0vb2e1_dn3)), (((((0.5 * var_qb0_dn4) * var_q1q) + (assign6240_e6354 * var_q1q_dn4)) * var_dn0vb2e1) + (assign6240_e6356 * var_dn0vb2e1_dn4)), (((((0.5 * var_qb0_dn5) * var_q1q) + (assign6240_e6354 * var_q1q_dn5)) * var_dn0vb2e1) + (assign6240_e6356 * var_dn0vb2e1_dn5)), (((((0.5 * var_qb0_dn6) * var_q1q) + (assign6240_e6354 * var_q1q_dn6)) * var_dn0vb2e1) + (assign6240_e6356 * var_dn0vb2e1_dn6)), (((((0.5 * var_qb0_dn7) * var_q1q) + (assign6240_e6354 * var_q1q_dn7)) * var_dn0vb2e1) + (assign6240_e6356 * var_dn0vb2e1_dn7)), (((((0.5 * var_qb0_dn8) * var_q1q) + (assign6240_e6354 * var_q1q_dn8)) * var_dn0vb2e1) + (assign6240_e6356 * var_dn0vb2e1_dn8)), (((((0.5 * var_qb0_dn9) * var_q1q) + (assign6240_e6354 * var_q1q_dn9)) * var_dn0vb2e1) + (assign6240_e6356 * var_dn0vb2e1_dn9)), (((((0.5 * var_qb0_dn10) * var_q1q) + (assign6240_e6354 * var_q1q_dn10)) * var_dn0vb2e1) + (assign6240_e6356 * var_dn0vb2e1_dn10)), (((((0.5 * var_qb0_db0) * var_q1q) + (assign6240_e6354 * var_q1q_db0)) * var_dn0vb2e1) + (assign6240_e6356 * var_dn0vb2e1_db0)), (((((0.5 * var_qb0_db1) * var_q1q) + (assign6240_e6354 * var_q1q_db1)) * var_dn0vb2e1) + (assign6240_e6356 * var_dn0vb2e1_db1)),)
    } else {
        (var_dqbevb2e1, var_dqbevb2e1_dn0, var_dqbevb2e1_dn1, var_dqbevb2e1_dn2, var_dqbevb2e1_dn3, var_dqbevb2e1_dn4, var_dqbevb2e1_dn5, var_dqbevb2e1_dn6, var_dqbevb2e1_dn7, var_dqbevb2e1_dn8, var_dqbevb2e1_dn9, var_dqbevb2e1_dn10, var_dqbevb2e1_db0, var_dqbevb2e1_db1,)
    }
};
        var_dqbevb2e1 = assign6240_e6360;
        var_dqbevb2e1_dn0 = assign6240_e6360_d_n0;
        var_dqbevb2e1_dn1 = assign6240_e6360_d_n1;
        var_dqbevb2e1_dn2 = assign6240_e6360_d_n2;
        var_dqbevb2e1_dn3 = assign6240_e6360_d_n3;
        var_dqbevb2e1_dn4 = assign6240_e6360_d_n4;
        var_dqbevb2e1_dn5 = assign6240_e6360_d_n5;
        var_dqbevb2e1_dn6 = assign6240_e6360_d_n6;
        var_dqbevb2e1_dn7 = assign6240_e6360_d_n7;
        var_dqbevb2e1_dn8 = assign6240_e6360_d_n8;
        var_dqbevb2e1_dn9 = assign6240_e6360_d_n9;
        var_dqbevb2e1_dn10 = assign6240_e6360_d_n10;
        var_dqbevb2e1_db0 = assign6240_e6360_d_b0;
        var_dqbevb2e1_db1 = assign6240_e6360_d_b1;
        var_dqbevb2e1_rv = 0.0;
        var_dqbevb2e1_rdn0 = 0.0;
        var_dqbevb2e1_rdn1 = 0.0;
        var_dqbevb2e1_rdn2 = 0.0;
        var_dqbevb2e1_rdn3 = 0.0;
        var_dqbevb2e1_rdn4 = 0.0;
        var_dqbevb2e1_rdn5 = 0.0;
        var_dqbevb2e1_rdn6 = 0.0;
        var_dqbevb2e1_rdn7 = 0.0;
        var_dqbevb2e1_rdn8 = 0.0;
        var_dqbevb2e1_rdn9 = 0.0;
        var_dqbevb2e1_rdn10 = 0.0;
        var_dqbevb2e1_rdb0 = 0.0;
        var_dqbevb2e1_rdb1 = 0.0;

        let (assign6250_e6368, assign6250_e6368_d_n0, assign6250_e6368_d_n1, assign6250_e6368_d_n2, assign6250_e6368_d_n3, assign6250_e6368_d_n4, assign6250_e6368_d_n5, assign6250_e6368_d_n6, assign6250_e6368_d_n7, assign6250_e6368_d_n8, assign6250_e6368_d_n9, assign6250_e6368_d_n10, assign6250_e6368_d_b0, assign6250_e6368_d_b1,) = {
    if (var_guard112 != 0.0) {
        let assign6250_e6365: f64 = (p.p84 * var_vt);
        let assign6250_e6366: f64 = (var_qe_qs / assign6250_e6365);
        (assign6250_e6366, (((var_qe_qs_dn0 * assign6250_e6365) - (var_qe_qs * (p.p84 * var_vt_dn0))) / (assign6250_e6365 * assign6250_e6365)), (((var_qe_qs_dn1 * assign6250_e6365) - (var_qe_qs * (p.p84 * var_vt_dn1))) / (assign6250_e6365 * assign6250_e6365)), (((var_qe_qs_dn2 * assign6250_e6365) - (var_qe_qs * (p.p84 * var_vt_dn2))) / (assign6250_e6365 * assign6250_e6365)), (((var_qe_qs_dn3 * assign6250_e6365) - (var_qe_qs * (p.p84 * var_vt_dn3))) / (assign6250_e6365 * assign6250_e6365)), (((var_qe_qs_dn4 * assign6250_e6365) - (var_qe_qs * (p.p84 * var_vt_dn4))) / (assign6250_e6365 * assign6250_e6365)), (((var_qe_qs_dn5 * assign6250_e6365) - (var_qe_qs * (p.p84 * var_vt_dn5))) / (assign6250_e6365 * assign6250_e6365)), (((var_qe_qs_dn6 * assign6250_e6365) - (var_qe_qs * (p.p84 * var_vt_dn6))) / (assign6250_e6365 * assign6250_e6365)), (((var_qe_qs_dn7 * assign6250_e6365) - (var_qe_qs * (p.p84 * var_vt_dn7))) / (assign6250_e6365 * assign6250_e6365)), (((var_qe_qs_dn8 * assign6250_e6365) - (var_qe_qs * (p.p84 * var_vt_dn8))) / (assign6250_e6365 * assign6250_e6365)), (((var_qe_qs_dn9 * assign6250_e6365) - (var_qe_qs * (p.p84 * var_vt_dn9))) / (assign6250_e6365 * assign6250_e6365)), (((var_qe_qs_dn10 * assign6250_e6365) - (var_qe_qs * (p.p84 * var_vt_dn10))) / (assign6250_e6365 * assign6250_e6365)), (((var_qe_qs_db0 * assign6250_e6365) - (var_qe_qs * (p.p84 * var_vt_db0))) / (assign6250_e6365 * assign6250_e6365)), (((var_qe_qs_db1 * assign6250_e6365) - (var_qe_qs * (p.p84 * var_vt_db1))) / (assign6250_e6365 * assign6250_e6365)),)
    } else {
        (var_dqevb2e1, var_dqevb2e1_dn0, var_dqevb2e1_dn1, var_dqevb2e1_dn2, var_dqevb2e1_dn3, var_dqevb2e1_dn4, var_dqevb2e1_dn5, var_dqevb2e1_dn6, var_dqevb2e1_dn7, var_dqevb2e1_dn8, var_dqevb2e1_dn9, var_dqevb2e1_dn10, var_dqevb2e1_db0, var_dqevb2e1_db1,)
    }
};
        var_dqevb2e1 = assign6250_e6368;
        var_dqevb2e1_dn0 = assign6250_e6368_d_n0;
        var_dqevb2e1_dn1 = assign6250_e6368_d_n1;
        var_dqevb2e1_dn2 = assign6250_e6368_d_n2;
        var_dqevb2e1_dn3 = assign6250_e6368_d_n3;
        var_dqevb2e1_dn4 = assign6250_e6368_d_n4;
        var_dqevb2e1_dn5 = assign6250_e6368_d_n5;
        var_dqevb2e1_dn6 = assign6250_e6368_d_n6;
        var_dqevb2e1_dn7 = assign6250_e6368_d_n7;
        var_dqevb2e1_dn8 = assign6250_e6368_d_n8;
        var_dqevb2e1_dn9 = assign6250_e6368_d_n9;
        var_dqevb2e1_dn10 = assign6250_e6368_d_n10;
        var_dqevb2e1_db0 = assign6250_e6368_d_b0;
        var_dqevb2e1_db1 = assign6250_e6368_d_b1;
        var_dqevb2e1_rv = 0.0;
        var_dqevb2e1_rdn0 = 0.0;
        var_dqevb2e1_rdn1 = 0.0;
        var_dqevb2e1_rdn2 = 0.0;
        var_dqevb2e1_rdn3 = 0.0;
        var_dqevb2e1_rdn4 = 0.0;
        var_dqevb2e1_rdn5 = 0.0;
        var_dqevb2e1_rdn6 = 0.0;
        var_dqevb2e1_rdn7 = 0.0;
        var_dqevb2e1_rdn8 = 0.0;
        var_dqevb2e1_rdn9 = 0.0;
        var_dqevb2e1_rdn10 = 0.0;
        var_dqevb2e1_rdb0 = 0.0;
        var_dqevb2e1_rdb1 = 0.0;

        let (assign6260_e6380, assign6260_e6380_d_n0, assign6260_e6380_d_n1, assign6260_e6380_d_n2, assign6260_e6380_d_n3, assign6260_e6380_d_n4, assign6260_e6380_d_n5, assign6260_e6380_d_n6, assign6260_e6380_d_n7, assign6260_e6380_d_n8, assign6260_e6380_d_n9, assign6260_e6380_d_n10, assign6260_e6380_d_b0, assign6260_e6380_d_b1,) = {
    if (var_guard112 != 0.0) {
        let assign6260_e6372: f64 = (0.2 * var_vb1b2);
        let assign6260_e6375: f64 = (var_dqtevb2e1 + var_dqbevb2e1);
        let assign6260_e6377: f64 = (assign6260_e6375 + var_dqevb2e1);
        let assign6260_e6378: f64 = (assign6260_e6372 * assign6260_e6377);
        (assign6260_e6378, (((0.2 * var_vb1b2_dn0) * assign6260_e6377) + (assign6260_e6372 * ((var_dqtevb2e1_dn0 + var_dqbevb2e1_dn0) + var_dqevb2e1_dn0))), (((0.2 * var_vb1b2_dn1) * assign6260_e6377) + (assign6260_e6372 * ((var_dqtevb2e1_dn1 + var_dqbevb2e1_dn1) + var_dqevb2e1_dn1))), (((0.2 * var_vb1b2_dn2) * assign6260_e6377) + (assign6260_e6372 * ((var_dqtevb2e1_dn2 + var_dqbevb2e1_dn2) + var_dqevb2e1_dn2))), (((0.2 * var_vb1b2_dn3) * assign6260_e6377) + (assign6260_e6372 * ((var_dqtevb2e1_dn3 + var_dqbevb2e1_dn3) + var_dqevb2e1_dn3))), (((0.2 * var_vb1b2_dn4) * assign6260_e6377) + (assign6260_e6372 * ((var_dqtevb2e1_dn4 + var_dqbevb2e1_dn4) + var_dqevb2e1_dn4))), (((0.2 * var_vb1b2_dn5) * assign6260_e6377) + (assign6260_e6372 * ((var_dqtevb2e1_dn5 + var_dqbevb2e1_dn5) + var_dqevb2e1_dn5))), (((0.2 * var_vb1b2_dn6) * assign6260_e6377) + (assign6260_e6372 * ((var_dqtevb2e1_dn6 + var_dqbevb2e1_dn6) + var_dqevb2e1_dn6))), (((0.2 * var_vb1b2_dn7) * assign6260_e6377) + (assign6260_e6372 * ((var_dqtevb2e1_dn7 + var_dqbevb2e1_dn7) + var_dqevb2e1_dn7))), (((0.2 * var_vb1b2_dn8) * assign6260_e6377) + (assign6260_e6372 * ((var_dqtevb2e1_dn8 + var_dqbevb2e1_dn8) + var_dqevb2e1_dn8))), (((0.2 * var_vb1b2_dn9) * assign6260_e6377) + (assign6260_e6372 * ((var_dqtevb2e1_dn9 + var_dqbevb2e1_dn9) + var_dqevb2e1_dn9))), (((0.2 * var_vb1b2_dn10) * assign6260_e6377) + (assign6260_e6372 * ((var_dqtevb2e1_dn10 + var_dqbevb2e1_dn10) + var_dqevb2e1_dn10))), (((0.2 * var_vb1b2_db0) * assign6260_e6377) + (assign6260_e6372 * ((var_dqtevb2e1_db0 + var_dqbevb2e1_db0) + var_dqevb2e1_db0))), (((0.2 * var_vb1b2_db1) * assign6260_e6377) + (assign6260_e6372 * ((var_dqtevb2e1_db1 + var_dqbevb2e1_db1) + var_dqevb2e1_db1))),)
    } else {
        (var_qb1b2, var_qb1b2_dn0, var_qb1b2_dn1, var_qb1b2_dn2, var_qb1b2_dn3, var_qb1b2_dn4, var_qb1b2_dn5, var_qb1b2_dn6, var_qb1b2_dn7, var_qb1b2_dn8, var_qb1b2_dn9, var_qb1b2_dn10, var_qb1b2_db0, var_qb1b2_db1,)
    }
};
        var_qb1b2 = assign6260_e6380;
        var_qb1b2_dn0 = assign6260_e6380_d_n0;
        var_qb1b2_dn1 = assign6260_e6380_d_n1;
        var_qb1b2_dn2 = assign6260_e6380_d_n2;
        var_qb1b2_dn3 = assign6260_e6380_d_n3;
        var_qb1b2_dn4 = assign6260_e6380_d_n4;
        var_qb1b2_dn5 = assign6260_e6380_d_n5;
        var_qb1b2_dn6 = assign6260_e6380_d_n6;
        var_qb1b2_dn7 = assign6260_e6380_d_n7;
        var_qb1b2_dn8 = assign6260_e6380_d_n8;
        var_qb1b2_dn9 = assign6260_e6380_d_n9;
        var_qb1b2_dn10 = assign6260_e6380_d_n10;
        var_qb1b2_db0 = assign6260_e6380_d_b0;
        var_qb1b2_db1 = assign6260_e6380_d_b1;
        var_qb1b2_rv = 0.0;
        var_qb1b2_rdn0 = 0.0;
        var_qb1b2_rdn1 = 0.0;
        var_qb1b2_rdn2 = 0.0;
        var_qb1b2_rdn3 = 0.0;
        var_qb1b2_rdn4 = 0.0;
        var_qb1b2_rdn5 = 0.0;
        var_qb1b2_rdn6 = 0.0;
        var_qb1b2_rdn7 = 0.0;
        var_qb1b2_rdn8 = 0.0;
        var_qb1b2_rdn9 = 0.0;
        var_qb1b2_rdn10 = 0.0;
        var_qb1b2_rdb0 = 0.0;
        var_qb1b2_rdb1 = 0.0;

        let (assign6270_e6388, assign6270_e6388_d_n0, assign6270_e6388_d_n1, assign6270_e6388_d_n2, assign6270_e6388_d_n3, assign6270_e6388_d_n4, assign6270_e6388_d_n5, assign6270_e6388_d_n6, assign6270_e6388_d_n7, assign6270_e6388_d_n8, assign6270_e6388_d_n9, assign6270_e6388_d_n10, assign6270_e6388_d_b0, assign6270_e6388_d_b1,) = {
    if (var_guard112 != 0.0) {
        let assign6270_e6384: f64 = (1.0 - p.p94);
        let assign6270_e6386: f64 = (assign6270_e6384 * var_qe_qs);
        (assign6270_e6386, (assign6270_e6384 * var_qe_qs_dn0), (assign6270_e6384 * var_qe_qs_dn1), (assign6270_e6384 * var_qe_qs_dn2), (assign6270_e6384 * var_qe_qs_dn3), (assign6270_e6384 * var_qe_qs_dn4), (assign6270_e6384 * var_qe_qs_dn5), (assign6270_e6384 * var_qe_qs_dn6), (assign6270_e6384 * var_qe_qs_dn7), (assign6270_e6384 * var_qe_qs_dn8), (assign6270_e6384 * var_qe_qs_dn9), (assign6270_e6384 * var_qe_qs_dn10), (assign6270_e6384 * var_qe_qs_db0), (assign6270_e6384 * var_qe_qs_db1),)
    } else {
        (var_qe, var_qe_dn0, var_qe_dn1, var_qe_dn2, var_qe_dn3, var_qe_dn4, var_qe_dn5, var_qe_dn6, var_qe_dn7, var_qe_dn8, var_qe_dn9, var_qe_dn10, var_qe_db0, var_qe_db1,)
    }
};
        var_qe = assign6270_e6388;
        var_qe_dn0 = assign6270_e6388_d_n0;
        var_qe_dn1 = assign6270_e6388_d_n1;
        var_qe_dn2 = assign6270_e6388_d_n2;
        var_qe_dn3 = assign6270_e6388_d_n3;
        var_qe_dn4 = assign6270_e6388_d_n4;
        var_qe_dn5 = assign6270_e6388_d_n5;
        var_qe_dn6 = assign6270_e6388_d_n6;
        var_qe_dn7 = assign6270_e6388_d_n7;
        var_qe_dn8 = assign6270_e6388_d_n8;
        var_qe_dn9 = assign6270_e6388_d_n9;
        var_qe_dn10 = assign6270_e6388_d_n10;
        var_qe_db0 = assign6270_e6388_d_b0;
        var_qe_db1 = assign6270_e6388_d_b1;
        var_qe_rv = 0.0;
        var_qe_rdn0 = 0.0;
        var_qe_rdn1 = 0.0;
        var_qe_rdn2 = 0.0;
        var_qe_rdn3 = 0.0;
        var_qe_rdn4 = 0.0;
        var_qe_rdn5 = 0.0;
        var_qe_rdn6 = 0.0;
        var_qe_rdn7 = 0.0;
        var_qe_rdn8 = 0.0;
        var_qe_rdn9 = 0.0;
        var_qe_rdn10 = 0.0;
        var_qe_rdb0 = 0.0;
        var_qe_rdb1 = 0.0;

        let (assign6280_e6396, assign6280_e6396_d_n0, assign6280_e6396_d_n1, assign6280_e6396_d_n2, assign6280_e6396_d_n3, assign6280_e6396_d_n4, assign6280_e6396_d_n5, assign6280_e6396_d_n6, assign6280_e6396_d_n7, assign6280_e6396_d_n8, assign6280_e6396_d_n9, assign6280_e6396_d_n10, assign6280_e6396_d_b0, assign6280_e6396_d_b1,) = {
    if (var_guard112 != 0.0) {
        let assign6280_e6393: f64 = (p.p94 * var_qe_qs);
        let assign6280_e6394: f64 = (var_qbe_qs + assign6280_e6393);
        (assign6280_e6394, (var_qbe_qs_dn0 + (p.p94 * var_qe_qs_dn0)), (var_qbe_qs_dn1 + (p.p94 * var_qe_qs_dn1)), (var_qbe_qs_dn2 + (p.p94 * var_qe_qs_dn2)), (var_qbe_qs_dn3 + (p.p94 * var_qe_qs_dn3)), (var_qbe_qs_dn4 + (p.p94 * var_qe_qs_dn4)), (var_qbe_qs_dn5 + (p.p94 * var_qe_qs_dn5)), (var_qbe_qs_dn6 + (p.p94 * var_qe_qs_dn6)), (var_qbe_qs_dn7 + (p.p94 * var_qe_qs_dn7)), (var_qbe_qs_dn8 + (p.p94 * var_qe_qs_dn8)), (var_qbe_qs_dn9 + (p.p94 * var_qe_qs_dn9)), (var_qbe_qs_dn10 + (p.p94 * var_qe_qs_dn10)), (var_qbe_qs_db0 + (p.p94 * var_qe_qs_db0)), (var_qbe_qs_db1 + (p.p94 * var_qe_qs_db1)),)
    } else {
        (var_qbe_qs_eff, var_qbe_qs_eff_dn0, var_qbe_qs_eff_dn1, var_qbe_qs_eff_dn2, var_qbe_qs_eff_dn3, var_qbe_qs_eff_dn4, var_qbe_qs_eff_dn5, var_qbe_qs_eff_dn6, var_qbe_qs_eff_dn7, var_qbe_qs_eff_dn8, var_qbe_qs_eff_dn9, var_qbe_qs_eff_dn10, var_qbe_qs_eff_db0, var_qbe_qs_eff_db1,)
    }
};
        var_qbe_qs_eff = assign6280_e6396;
        var_qbe_qs_eff_dn0 = assign6280_e6396_d_n0;
        var_qbe_qs_eff_dn1 = assign6280_e6396_d_n1;
        var_qbe_qs_eff_dn2 = assign6280_e6396_d_n2;
        var_qbe_qs_eff_dn3 = assign6280_e6396_d_n3;
        var_qbe_qs_eff_dn4 = assign6280_e6396_d_n4;
        var_qbe_qs_eff_dn5 = assign6280_e6396_d_n5;
        var_qbe_qs_eff_dn6 = assign6280_e6396_d_n6;
        var_qbe_qs_eff_dn7 = assign6280_e6396_d_n7;
        var_qbe_qs_eff_dn8 = assign6280_e6396_d_n8;
        var_qbe_qs_eff_dn9 = assign6280_e6396_d_n9;
        var_qbe_qs_eff_dn10 = assign6280_e6396_d_n10;
        var_qbe_qs_eff_db0 = assign6280_e6396_d_b0;
        var_qbe_qs_eff_db1 = assign6280_e6396_d_b1;
        var_qbe_qs_eff_rv = 0.0;
        var_qbe_qs_eff_rdn0 = 0.0;
        var_qbe_qs_eff_rdn1 = 0.0;
        var_qbe_qs_eff_rdn2 = 0.0;
        var_qbe_qs_eff_rdn3 = 0.0;
        var_qbe_qs_eff_rdn4 = 0.0;
        var_qbe_qs_eff_rdn5 = 0.0;
        var_qbe_qs_eff_rdn6 = 0.0;
        var_qbe_qs_eff_rdn7 = 0.0;
        var_qbe_qs_eff_rdn8 = 0.0;
        var_qbe_qs_eff_rdn9 = 0.0;
        var_qbe_qs_eff_rdn10 = 0.0;
        var_qbe_qs_eff_rdb0 = 0.0;
        var_qbe_qs_eff_rdb1 = 0.0;

        let (assign6290_e6404, assign6290_e6404_d_n0, assign6290_e6404_d_n1, assign6290_e6404_d_n2, assign6290_e6404_d_n3, assign6290_e6404_d_n4, assign6290_e6404_d_n5, assign6290_e6404_d_n6, assign6290_e6404_d_n7, assign6290_e6404_d_n8, assign6290_e6404_d_n9, assign6290_e6404_d_n10, assign6290_e6404_d_b0, assign6290_e6404_d_b1,) = {
    if (var_guard112 != 0.0) {
        let assign6290_e6400: f64 = (p.p93 * var_qbe_qs_eff);
        let assign6290_e6402: f64 = (assign6290_e6400 + var_qbc_qs);
        (assign6290_e6402, ((p.p93 * var_qbe_qs_eff_dn0) + var_qbc_qs_dn0), ((p.p93 * var_qbe_qs_eff_dn1) + var_qbc_qs_dn1), ((p.p93 * var_qbe_qs_eff_dn2) + var_qbc_qs_dn2), ((p.p93 * var_qbe_qs_eff_dn3) + var_qbc_qs_dn3), ((p.p93 * var_qbe_qs_eff_dn4) + var_qbc_qs_dn4), ((p.p93 * var_qbe_qs_eff_dn5) + var_qbc_qs_dn5), ((p.p93 * var_qbe_qs_eff_dn6) + var_qbc_qs_dn6), ((p.p93 * var_qbe_qs_eff_dn7) + var_qbc_qs_dn7), ((p.p93 * var_qbe_qs_eff_dn8) + var_qbc_qs_dn8), ((p.p93 * var_qbe_qs_eff_dn9) + var_qbc_qs_dn9), ((p.p93 * var_qbe_qs_eff_dn10) + var_qbc_qs_dn10), ((p.p93 * var_qbe_qs_eff_db0) + var_qbc_qs_db0), ((p.p93 * var_qbe_qs_eff_db1) + var_qbc_qs_db1),)
    } else {
        (var_qbc, var_qbc_dn0, var_qbc_dn1, var_qbc_dn2, var_qbc_dn3, var_qbc_dn4, var_qbc_dn5, var_qbc_dn6, var_qbc_dn7, var_qbc_dn8, var_qbc_dn9, var_qbc_dn10, var_qbc_db0, var_qbc_db1,)
    }
};
        var_qbc = assign6290_e6404;
        var_qbc_dn0 = assign6290_e6404_d_n0;
        var_qbc_dn1 = assign6290_e6404_d_n1;
        var_qbc_dn2 = assign6290_e6404_d_n2;
        var_qbc_dn3 = assign6290_e6404_d_n3;
        var_qbc_dn4 = assign6290_e6404_d_n4;
        var_qbc_dn5 = assign6290_e6404_d_n5;
        var_qbc_dn6 = assign6290_e6404_d_n6;
        var_qbc_dn7 = assign6290_e6404_d_n7;
        var_qbc_dn8 = assign6290_e6404_d_n8;
        var_qbc_dn9 = assign6290_e6404_d_n9;
        var_qbc_dn10 = assign6290_e6404_d_n10;
        var_qbc_db0 = assign6290_e6404_d_b0;
        var_qbc_db1 = assign6290_e6404_d_b1;
        var_qbc_rv = 0.0;
        var_qbc_rdn0 = 0.0;
        var_qbc_rdn1 = 0.0;
        var_qbc_rdn2 = 0.0;
        var_qbc_rdn3 = 0.0;
        var_qbc_rdn4 = 0.0;
        var_qbc_rdn5 = 0.0;
        var_qbc_rdn6 = 0.0;
        var_qbc_rdn7 = 0.0;
        var_qbc_rdn8 = 0.0;
        var_qbc_rdn9 = 0.0;
        var_qbc_rdn10 = 0.0;
        var_qbc_rdb0 = 0.0;
        var_qbc_rdb1 = 0.0;

        let (assign6300_e6412, assign6300_e6412_d_n0, assign6300_e6412_d_n1, assign6300_e6412_d_n2, assign6300_e6412_d_n3, assign6300_e6412_d_n4, assign6300_e6412_d_n5, assign6300_e6412_d_n6, assign6300_e6412_d_n7, assign6300_e6412_d_n8, assign6300_e6412_d_n9, assign6300_e6412_d_n10, assign6300_e6412_d_b0, assign6300_e6412_d_b1,) = {
    if (var_guard112 != 0.0) {
        let assign6300_e6408: f64 = (1.0 - p.p93);
        let assign6300_e6410: f64 = (assign6300_e6408 * var_qbe_qs_eff);
        (assign6300_e6410, (assign6300_e6408 * var_qbe_qs_eff_dn0), (assign6300_e6408 * var_qbe_qs_eff_dn1), (assign6300_e6408 * var_qbe_qs_eff_dn2), (assign6300_e6408 * var_qbe_qs_eff_dn3), (assign6300_e6408 * var_qbe_qs_eff_dn4), (assign6300_e6408 * var_qbe_qs_eff_dn5), (assign6300_e6408 * var_qbe_qs_eff_dn6), (assign6300_e6408 * var_qbe_qs_eff_dn7), (assign6300_e6408 * var_qbe_qs_eff_dn8), (assign6300_e6408 * var_qbe_qs_eff_dn9), (assign6300_e6408 * var_qbe_qs_eff_dn10), (assign6300_e6408 * var_qbe_qs_eff_db0), (assign6300_e6408 * var_qbe_qs_eff_db1),)
    } else {
        (var_qbe, var_qbe_dn0, var_qbe_dn1, var_qbe_dn2, var_qbe_dn3, var_qbe_dn4, var_qbe_dn5, var_qbe_dn6, var_qbe_dn7, var_qbe_dn8, var_qbe_dn9, var_qbe_dn10, var_qbe_db0, var_qbe_db1,)
    }
};
        var_qbe = assign6300_e6412;
        var_qbe_dn0 = assign6300_e6412_d_n0;
        var_qbe_dn1 = assign6300_e6412_d_n1;
        var_qbe_dn2 = assign6300_e6412_d_n2;
        var_qbe_dn3 = assign6300_e6412_d_n3;
        var_qbe_dn4 = assign6300_e6412_d_n4;
        var_qbe_dn5 = assign6300_e6412_d_n5;
        var_qbe_dn6 = assign6300_e6412_d_n6;
        var_qbe_dn7 = assign6300_e6412_d_n7;
        var_qbe_dn8 = assign6300_e6412_d_n8;
        var_qbe_dn9 = assign6300_e6412_d_n9;
        var_qbe_dn10 = assign6300_e6412_d_n10;
        var_qbe_db0 = assign6300_e6412_d_b0;
        var_qbe_db1 = assign6300_e6412_d_b1;
        var_qbe_rv = 0.0;
        var_qbe_rdn0 = 0.0;
        var_qbe_rdn1 = 0.0;
        var_qbe_rdn2 = 0.0;
        var_qbe_rdn3 = 0.0;
        var_qbe_rdn4 = 0.0;
        var_qbe_rdn5 = 0.0;
        var_qbe_rdn6 = 0.0;
        var_qbe_rdn7 = 0.0;
        var_qbe_rdn8 = 0.0;
        var_qbe_rdn9 = 0.0;
        var_qbe_rdn10 = 0.0;
        var_qbe_rdb0 = 0.0;
        var_qbe_rdb1 = 0.0;

        let (assign6310_e6417, assign6310_e6417_d_n0, assign6310_e6417_d_n1, assign6310_e6417_d_n2, assign6310_e6417_d_n3, assign6310_e6417_d_n4, assign6310_e6417_d_n5, assign6310_e6417_d_n6, assign6310_e6417_d_n7, assign6310_e6417_d_n8, assign6310_e6417_d_n9, assign6310_e6417_d_n10, assign6310_e6417_d_b0, assign6310_e6417_d_b1,) = {
    if (var_guard112 == 0.0) {
        (var_qbe_qs, var_qbe_qs_dn0, var_qbe_qs_dn1, var_qbe_qs_dn2, var_qbe_qs_dn3, var_qbe_qs_dn4, var_qbe_qs_dn5, var_qbe_qs_dn6, var_qbe_qs_dn7, var_qbe_qs_dn8, var_qbe_qs_dn9, var_qbe_qs_dn10, var_qbe_qs_db0, var_qbe_qs_db1,)
    } else {
        (var_qbe, var_qbe_dn0, var_qbe_dn1, var_qbe_dn2, var_qbe_dn3, var_qbe_dn4, var_qbe_dn5, var_qbe_dn6, var_qbe_dn7, var_qbe_dn8, var_qbe_dn9, var_qbe_dn10, var_qbe_db0, var_qbe_db1,)
    }
};
        var_qbe = assign6310_e6417;
        var_qbe_dn0 = assign6310_e6417_d_n0;
        var_qbe_dn1 = assign6310_e6417_d_n1;
        var_qbe_dn2 = assign6310_e6417_d_n2;
        var_qbe_dn3 = assign6310_e6417_d_n3;
        var_qbe_dn4 = assign6310_e6417_d_n4;
        var_qbe_dn5 = assign6310_e6417_d_n5;
        var_qbe_dn6 = assign6310_e6417_d_n6;
        var_qbe_dn7 = assign6310_e6417_d_n7;
        var_qbe_dn8 = assign6310_e6417_d_n8;
        var_qbe_dn9 = assign6310_e6417_d_n9;
        var_qbe_dn10 = assign6310_e6417_d_n10;
        var_qbe_db0 = assign6310_e6417_d_b0;
        var_qbe_db1 = assign6310_e6417_d_b1;
        var_qbe_rv = 0.0;
        var_qbe_rdn0 = 0.0;
        var_qbe_rdn1 = 0.0;
        var_qbe_rdn2 = 0.0;
        var_qbe_rdn3 = 0.0;
        var_qbe_rdn4 = 0.0;
        var_qbe_rdn5 = 0.0;
        var_qbe_rdn6 = 0.0;
        var_qbe_rdn7 = 0.0;
        var_qbe_rdn8 = 0.0;
        var_qbe_rdn9 = 0.0;
        var_qbe_rdn10 = 0.0;
        var_qbe_rdb0 = 0.0;
        var_qbe_rdb1 = 0.0;

        let (assign6320_e6422, assign6320_e6422_d_n0, assign6320_e6422_d_n1, assign6320_e6422_d_n2, assign6320_e6422_d_n3, assign6320_e6422_d_n4, assign6320_e6422_d_n5, assign6320_e6422_d_n6, assign6320_e6422_d_n7, assign6320_e6422_d_n8, assign6320_e6422_d_n9, assign6320_e6422_d_n10, assign6320_e6422_d_b0, assign6320_e6422_d_b1,) = {
    if (var_guard112 == 0.0) {
        (var_qbc_qs, var_qbc_qs_dn0, var_qbc_qs_dn1, var_qbc_qs_dn2, var_qbc_qs_dn3, var_qbc_qs_dn4, var_qbc_qs_dn5, var_qbc_qs_dn6, var_qbc_qs_dn7, var_qbc_qs_dn8, var_qbc_qs_dn9, var_qbc_qs_dn10, var_qbc_qs_db0, var_qbc_qs_db1,)
    } else {
        (var_qbc, var_qbc_dn0, var_qbc_dn1, var_qbc_dn2, var_qbc_dn3, var_qbc_dn4, var_qbc_dn5, var_qbc_dn6, var_qbc_dn7, var_qbc_dn8, var_qbc_dn9, var_qbc_dn10, var_qbc_db0, var_qbc_db1,)
    }
};
        var_qbc = assign6320_e6422;
        var_qbc_dn0 = assign6320_e6422_d_n0;
        var_qbc_dn1 = assign6320_e6422_d_n1;
        var_qbc_dn2 = assign6320_e6422_d_n2;
        var_qbc_dn3 = assign6320_e6422_d_n3;
        var_qbc_dn4 = assign6320_e6422_d_n4;
        var_qbc_dn5 = assign6320_e6422_d_n5;
        var_qbc_dn6 = assign6320_e6422_d_n6;
        var_qbc_dn7 = assign6320_e6422_d_n7;
        var_qbc_dn8 = assign6320_e6422_d_n8;
        var_qbc_dn9 = assign6320_e6422_d_n9;
        var_qbc_dn10 = assign6320_e6422_d_n10;
        var_qbc_db0 = assign6320_e6422_d_b0;
        var_qbc_db1 = assign6320_e6422_d_b1;
        var_qbc_rv = 0.0;
        var_qbc_rdn0 = 0.0;
        var_qbc_rdn1 = 0.0;
        var_qbc_rdn2 = 0.0;
        var_qbc_rdn3 = 0.0;
        var_qbc_rdn4 = 0.0;
        var_qbc_rdn5 = 0.0;
        var_qbc_rdn6 = 0.0;
        var_qbc_rdn7 = 0.0;
        var_qbc_rdn8 = 0.0;
        var_qbc_rdn9 = 0.0;
        var_qbc_rdn10 = 0.0;
        var_qbc_rdb0 = 0.0;
        var_qbc_rdb1 = 0.0;

        let (assign6330_e6427, assign6330_e6427_d_n0, assign6330_e6427_d_n1, assign6330_e6427_d_n2, assign6330_e6427_d_n3, assign6330_e6427_d_n4, assign6330_e6427_d_n5, assign6330_e6427_d_n6, assign6330_e6427_d_n7, assign6330_e6427_d_n8, assign6330_e6427_d_n9, assign6330_e6427_d_n10, assign6330_e6427_d_b0, assign6330_e6427_d_b1,) = {
    if (var_guard112 == 0.0) {
        (var_qe_qs, var_qe_qs_dn0, var_qe_qs_dn1, var_qe_qs_dn2, var_qe_qs_dn3, var_qe_qs_dn4, var_qe_qs_dn5, var_qe_qs_dn6, var_qe_qs_dn7, var_qe_qs_dn8, var_qe_qs_dn9, var_qe_qs_dn10, var_qe_qs_db0, var_qe_qs_db1,)
    } else {
        (var_qe, var_qe_dn0, var_qe_dn1, var_qe_dn2, var_qe_dn3, var_qe_dn4, var_qe_dn5, var_qe_dn6, var_qe_dn7, var_qe_dn8, var_qe_dn9, var_qe_dn10, var_qe_db0, var_qe_db1,)
    }
};
        var_qe = assign6330_e6427;
        var_qe_dn0 = assign6330_e6427_d_n0;
        var_qe_dn1 = assign6330_e6427_d_n1;
        var_qe_dn2 = assign6330_e6427_d_n2;
        var_qe_dn3 = assign6330_e6427_d_n3;
        var_qe_dn4 = assign6330_e6427_d_n4;
        var_qe_dn5 = assign6330_e6427_d_n5;
        var_qe_dn6 = assign6330_e6427_d_n6;
        var_qe_dn7 = assign6330_e6427_d_n7;
        var_qe_dn8 = assign6330_e6427_d_n8;
        var_qe_dn9 = assign6330_e6427_d_n9;
        var_qe_dn10 = assign6330_e6427_d_n10;
        var_qe_db0 = assign6330_e6427_d_b0;
        var_qe_db1 = assign6330_e6427_d_b1;
        var_qe_rv = 0.0;
        var_qe_rdn0 = 0.0;
        var_qe_rdn1 = 0.0;
        var_qe_rdn2 = 0.0;
        var_qe_rdn3 = 0.0;
        var_qe_rdn4 = 0.0;
        var_qe_rdn5 = 0.0;
        var_qe_rdn6 = 0.0;
        var_qe_rdn7 = 0.0;
        var_qe_rdn8 = 0.0;
        var_qe_rdn9 = 0.0;
        var_qe_rdn10 = 0.0;
        var_qe_rdb0 = 0.0;
        var_qe_rdb1 = 0.0;

        let assign6440_e6470: f64 = (var_if_ + var_ir);
        let assign6440_e6472: f64 = (assign6440_e6470 / var_qbi);
        var_in_n = assign6440_e6472;
        var_in_n_dn0 = ((((var_if__dn0 + var_ir_dn0) * var_qbi) - (assign6440_e6470 * var_qbi_dn0)) / (var_qbi * var_qbi));
        var_in_n_dn1 = ((((var_if__dn1 + var_ir_dn1) * var_qbi) - (assign6440_e6470 * var_qbi_dn1)) / (var_qbi * var_qbi));
        var_in_n_dn2 = ((((var_if__dn2 + var_ir_dn2) * var_qbi) - (assign6440_e6470 * var_qbi_dn2)) / (var_qbi * var_qbi));
        var_in_n_dn3 = ((((var_if__dn3 + var_ir_dn3) * var_qbi) - (assign6440_e6470 * var_qbi_dn3)) / (var_qbi * var_qbi));
        var_in_n_dn4 = ((((var_if__dn4 + var_ir_dn4) * var_qbi) - (assign6440_e6470 * var_qbi_dn4)) / (var_qbi * var_qbi));
        var_in_n_dn5 = ((((var_if__dn5 + var_ir_dn5) * var_qbi) - (assign6440_e6470 * var_qbi_dn5)) / (var_qbi * var_qbi));
        var_in_n_dn6 = ((((var_if__dn6 + var_ir_dn6) * var_qbi) - (assign6440_e6470 * var_qbi_dn6)) / (var_qbi * var_qbi));
        var_in_n_dn7 = ((((var_if__dn7 + var_ir_dn7) * var_qbi) - (assign6440_e6470 * var_qbi_dn7)) / (var_qbi * var_qbi));
        var_in_n_dn8 = ((((var_if__dn8 + var_ir_dn8) * var_qbi) - (assign6440_e6470 * var_qbi_dn8)) / (var_qbi * var_qbi));
        var_in_n_dn9 = ((((var_if__dn9 + var_ir_dn9) * var_qbi) - (assign6440_e6470 * var_qbi_dn9)) / (var_qbi * var_qbi));
        var_in_n_dn10 = ((((var_if__dn10 + var_ir_dn10) * var_qbi) - (assign6440_e6470 * var_qbi_dn10)) / (var_qbi * var_qbi));
        var_in_n_db0 = ((((var_if__db0 + var_ir_db0) * var_qbi) - (assign6440_e6470 * var_qbi_db0)) / (var_qbi * var_qbi));
        var_in_n_db1 = ((((var_if__db1 + var_ir_db1) * var_qbi) - (assign6440_e6470 * var_qbi_db1)) / (var_qbi * var_qbi));
        var_in_n_rv = 0.0;
        var_in_n_rdn0 = 0.0;
        var_in_n_rdn1 = 0.0;
        var_in_n_rdn2 = 0.0;
        var_in_n_rdn3 = 0.0;
        var_in_n_rdn4 = 0.0;
        var_in_n_rdn5 = 0.0;
        var_in_n_rdn6 = 0.0;
        var_in_n_rdn7 = 0.0;
        var_in_n_rdn8 = 0.0;
        var_in_n_rdn9 = 0.0;
        var_in_n_rdn10 = 0.0;
        var_in_n_rdb0 = 0.0;
        var_in_n_rdb1 = 0.0;

        *var_dn0vb2e1_slot = var_dn0vb2e1;
        *var_dn0vb2e1_db0_slot = var_dn0vb2e1_db0;
        *var_dn0vb2e1_db1_slot = var_dn0vb2e1_db1;
        *var_dn0vb2e1_dn0_slot = var_dn0vb2e1_dn0;
        *var_dn0vb2e1_dn1_slot = var_dn0vb2e1_dn1;
        *var_dn0vb2e1_dn10_slot = var_dn0vb2e1_dn10;
        *var_dn0vb2e1_dn2_slot = var_dn0vb2e1_dn2;
        *var_dn0vb2e1_dn3_slot = var_dn0vb2e1_dn3;
        *var_dn0vb2e1_dn4_slot = var_dn0vb2e1_dn4;
        *var_dn0vb2e1_dn5_slot = var_dn0vb2e1_dn5;
        *var_dn0vb2e1_dn6_slot = var_dn0vb2e1_dn6;
        *var_dn0vb2e1_dn7_slot = var_dn0vb2e1_dn7;
        *var_dn0vb2e1_dn8_slot = var_dn0vb2e1_dn8;
        *var_dn0vb2e1_dn9_slot = var_dn0vb2e1_dn9;
        *var_dn0vb2e1_rdb0_slot = var_dn0vb2e1_rdb0;
        *var_dn0vb2e1_rdb1_slot = var_dn0vb2e1_rdb1;
        *var_dn0vb2e1_rdn0_slot = var_dn0vb2e1_rdn0;
        *var_dn0vb2e1_rdn1_slot = var_dn0vb2e1_rdn1;
        *var_dn0vb2e1_rdn10_slot = var_dn0vb2e1_rdn10;
        *var_dn0vb2e1_rdn2_slot = var_dn0vb2e1_rdn2;
        *var_dn0vb2e1_rdn3_slot = var_dn0vb2e1_rdn3;
        *var_dn0vb2e1_rdn4_slot = var_dn0vb2e1_rdn4;
        *var_dn0vb2e1_rdn5_slot = var_dn0vb2e1_rdn5;
        *var_dn0vb2e1_rdn6_slot = var_dn0vb2e1_rdn6;
        *var_dn0vb2e1_rdn7_slot = var_dn0vb2e1_rdn7;
        *var_dn0vb2e1_rdn8_slot = var_dn0vb2e1_rdn8;
        *var_dn0vb2e1_rdn9_slot = var_dn0vb2e1_rdn9;
        *var_dn0vb2e1_rv_slot = var_dn0vb2e1_rv;
        *var_dqbevb2e1_slot = var_dqbevb2e1;
        *var_dqbevb2e1_db0_slot = var_dqbevb2e1_db0;
        *var_dqbevb2e1_db1_slot = var_dqbevb2e1_db1;
        *var_dqbevb2e1_dn0_slot = var_dqbevb2e1_dn0;
        *var_dqbevb2e1_dn1_slot = var_dqbevb2e1_dn1;
        *var_dqbevb2e1_dn10_slot = var_dqbevb2e1_dn10;
        *var_dqbevb2e1_dn2_slot = var_dqbevb2e1_dn2;
        *var_dqbevb2e1_dn3_slot = var_dqbevb2e1_dn3;
        *var_dqbevb2e1_dn4_slot = var_dqbevb2e1_dn4;
        *var_dqbevb2e1_dn5_slot = var_dqbevb2e1_dn5;
        *var_dqbevb2e1_dn6_slot = var_dqbevb2e1_dn6;
        *var_dqbevb2e1_dn7_slot = var_dqbevb2e1_dn7;
        *var_dqbevb2e1_dn8_slot = var_dqbevb2e1_dn8;
        *var_dqbevb2e1_dn9_slot = var_dqbevb2e1_dn9;
        *var_dqbevb2e1_rdb0_slot = var_dqbevb2e1_rdb0;
        *var_dqbevb2e1_rdb1_slot = var_dqbevb2e1_rdb1;
        *var_dqbevb2e1_rdn0_slot = var_dqbevb2e1_rdn0;
        *var_dqbevb2e1_rdn1_slot = var_dqbevb2e1_rdn1;
        *var_dqbevb2e1_rdn10_slot = var_dqbevb2e1_rdn10;
        *var_dqbevb2e1_rdn2_slot = var_dqbevb2e1_rdn2;
        *var_dqbevb2e1_rdn3_slot = var_dqbevb2e1_rdn3;
        *var_dqbevb2e1_rdn4_slot = var_dqbevb2e1_rdn4;
        *var_dqbevb2e1_rdn5_slot = var_dqbevb2e1_rdn5;
        *var_dqbevb2e1_rdn6_slot = var_dqbevb2e1_rdn6;
        *var_dqbevb2e1_rdn7_slot = var_dqbevb2e1_rdn7;
        *var_dqbevb2e1_rdn8_slot = var_dqbevb2e1_rdn8;
        *var_dqbevb2e1_rdn9_slot = var_dqbevb2e1_rdn9;
        *var_dqbevb2e1_rv_slot = var_dqbevb2e1_rv;
        *var_dqevb2e1_slot = var_dqevb2e1;
        *var_dqevb2e1_db0_slot = var_dqevb2e1_db0;
        *var_dqevb2e1_db1_slot = var_dqevb2e1_db1;
        *var_dqevb2e1_dn0_slot = var_dqevb2e1_dn0;
        *var_dqevb2e1_dn1_slot = var_dqevb2e1_dn1;
        *var_dqevb2e1_dn10_slot = var_dqevb2e1_dn10;
        *var_dqevb2e1_dn2_slot = var_dqevb2e1_dn2;
        *var_dqevb2e1_dn3_slot = var_dqevb2e1_dn3;
        *var_dqevb2e1_dn4_slot = var_dqevb2e1_dn4;
        *var_dqevb2e1_dn5_slot = var_dqevb2e1_dn5;
        *var_dqevb2e1_dn6_slot = var_dqevb2e1_dn6;
        *var_dqevb2e1_dn7_slot = var_dqevb2e1_dn7;
        *var_dqevb2e1_dn8_slot = var_dqevb2e1_dn8;
        *var_dqevb2e1_dn9_slot = var_dqevb2e1_dn9;
        *var_dqevb2e1_rdb0_slot = var_dqevb2e1_rdb0;
        *var_dqevb2e1_rdb1_slot = var_dqevb2e1_rdb1;
        *var_dqevb2e1_rdn0_slot = var_dqevb2e1_rdn0;
        *var_dqevb2e1_rdn1_slot = var_dqevb2e1_rdn1;
        *var_dqevb2e1_rdn10_slot = var_dqevb2e1_rdn10;
        *var_dqevb2e1_rdn2_slot = var_dqevb2e1_rdn2;
        *var_dqevb2e1_rdn3_slot = var_dqevb2e1_rdn3;
        *var_dqevb2e1_rdn4_slot = var_dqevb2e1_rdn4;
        *var_dqevb2e1_rdn5_slot = var_dqevb2e1_rdn5;
        *var_dqevb2e1_rdn6_slot = var_dqevb2e1_rdn6;
        *var_dqevb2e1_rdn7_slot = var_dqevb2e1_rdn7;
        *var_dqevb2e1_rdn8_slot = var_dqevb2e1_rdn8;
        *var_dqevb2e1_rdn9_slot = var_dqevb2e1_rdn9;
        *var_dqevb2e1_rv_slot = var_dqevb2e1_rv;
        *var_dqtevb2e1_slot = var_dqtevb2e1;
        *var_dqtevb2e1_db0_slot = var_dqtevb2e1_db0;
        *var_dqtevb2e1_db1_slot = var_dqtevb2e1_db1;
        *var_dqtevb2e1_dn0_slot = var_dqtevb2e1_dn0;
        *var_dqtevb2e1_dn1_slot = var_dqtevb2e1_dn1;
        *var_dqtevb2e1_dn10_slot = var_dqtevb2e1_dn10;
        *var_dqtevb2e1_dn2_slot = var_dqtevb2e1_dn2;
        *var_dqtevb2e1_dn3_slot = var_dqtevb2e1_dn3;
        *var_dqtevb2e1_dn4_slot = var_dqtevb2e1_dn4;
        *var_dqtevb2e1_dn5_slot = var_dqtevb2e1_dn5;
        *var_dqtevb2e1_dn6_slot = var_dqtevb2e1_dn6;
        *var_dqtevb2e1_dn7_slot = var_dqtevb2e1_dn7;
        *var_dqtevb2e1_dn8_slot = var_dqtevb2e1_dn8;
        *var_dqtevb2e1_dn9_slot = var_dqtevb2e1_dn9;
        *var_dqtevb2e1_rdb0_slot = var_dqtevb2e1_rdb0;
        *var_dqtevb2e1_rdb1_slot = var_dqtevb2e1_rdb1;
        *var_dqtevb2e1_rdn0_slot = var_dqtevb2e1_rdn0;
        *var_dqtevb2e1_rdn1_slot = var_dqtevb2e1_rdn1;
        *var_dqtevb2e1_rdn10_slot = var_dqtevb2e1_rdn10;
        *var_dqtevb2e1_rdn2_slot = var_dqtevb2e1_rdn2;
        *var_dqtevb2e1_rdn3_slot = var_dqtevb2e1_rdn3;
        *var_dqtevb2e1_rdn4_slot = var_dqtevb2e1_rdn4;
        *var_dqtevb2e1_rdn5_slot = var_dqtevb2e1_rdn5;
        *var_dqtevb2e1_rdn6_slot = var_dqtevb2e1_rdn6;
        *var_dqtevb2e1_rdn7_slot = var_dqtevb2e1_rdn7;
        *var_dqtevb2e1_rdn8_slot = var_dqtevb2e1_rdn8;
        *var_dqtevb2e1_rdn9_slot = var_dqtevb2e1_rdn9;
        *var_dqtevb2e1_rv_slot = var_dqtevb2e1_rv;
        *var_in_n_slot = var_in_n;
        *var_in_n_db0_slot = var_in_n_db0;
        *var_in_n_db1_slot = var_in_n_db1;
        *var_in_n_dn0_slot = var_in_n_dn0;
        *var_in_n_dn1_slot = var_in_n_dn1;
        *var_in_n_dn10_slot = var_in_n_dn10;
        *var_in_n_dn2_slot = var_in_n_dn2;
        *var_in_n_dn3_slot = var_in_n_dn3;
        *var_in_n_dn4_slot = var_in_n_dn4;
        *var_in_n_dn5_slot = var_in_n_dn5;
        *var_in_n_dn6_slot = var_in_n_dn6;
        *var_in_n_dn7_slot = var_in_n_dn7;
        *var_in_n_dn8_slot = var_in_n_dn8;
        *var_in_n_dn9_slot = var_in_n_dn9;
        *var_in_n_rdb0_slot = var_in_n_rdb0;
        *var_in_n_rdb1_slot = var_in_n_rdb1;
        *var_in_n_rdn0_slot = var_in_n_rdn0;
        *var_in_n_rdn1_slot = var_in_n_rdn1;
        *var_in_n_rdn10_slot = var_in_n_rdn10;
        *var_in_n_rdn2_slot = var_in_n_rdn2;
        *var_in_n_rdn3_slot = var_in_n_rdn3;
        *var_in_n_rdn4_slot = var_in_n_rdn4;
        *var_in_n_rdn5_slot = var_in_n_rdn5;
        *var_in_n_rdn6_slot = var_in_n_rdn6;
        *var_in_n_rdn7_slot = var_in_n_rdn7;
        *var_in_n_rdn8_slot = var_in_n_rdn8;
        *var_in_n_rdn9_slot = var_in_n_rdn9;
        *var_in_n_rv_slot = var_in_n_rv;
        *var_qb1b2_slot = var_qb1b2;
        *var_qb1b2_db0_slot = var_qb1b2_db0;
        *var_qb1b2_db1_slot = var_qb1b2_db1;
        *var_qb1b2_dn0_slot = var_qb1b2_dn0;
        *var_qb1b2_dn1_slot = var_qb1b2_dn1;
        *var_qb1b2_dn10_slot = var_qb1b2_dn10;
        *var_qb1b2_dn2_slot = var_qb1b2_dn2;
        *var_qb1b2_dn3_slot = var_qb1b2_dn3;
        *var_qb1b2_dn4_slot = var_qb1b2_dn4;
        *var_qb1b2_dn5_slot = var_qb1b2_dn5;
        *var_qb1b2_dn6_slot = var_qb1b2_dn6;
        *var_qb1b2_dn7_slot = var_qb1b2_dn7;
        *var_qb1b2_dn8_slot = var_qb1b2_dn8;
        *var_qb1b2_dn9_slot = var_qb1b2_dn9;
        *var_qb1b2_rdb0_slot = var_qb1b2_rdb0;
        *var_qb1b2_rdb1_slot = var_qb1b2_rdb1;
        *var_qb1b2_rdn0_slot = var_qb1b2_rdn0;
        *var_qb1b2_rdn1_slot = var_qb1b2_rdn1;
        *var_qb1b2_rdn10_slot = var_qb1b2_rdn10;
        *var_qb1b2_rdn2_slot = var_qb1b2_rdn2;
        *var_qb1b2_rdn3_slot = var_qb1b2_rdn3;
        *var_qb1b2_rdn4_slot = var_qb1b2_rdn4;
        *var_qb1b2_rdn5_slot = var_qb1b2_rdn5;
        *var_qb1b2_rdn6_slot = var_qb1b2_rdn6;
        *var_qb1b2_rdn7_slot = var_qb1b2_rdn7;
        *var_qb1b2_rdn8_slot = var_qb1b2_rdn8;
        *var_qb1b2_rdn9_slot = var_qb1b2_rdn9;
        *var_qb1b2_rv_slot = var_qb1b2_rv;
        *var_qbc_slot = var_qbc;
        *var_qbc_db0_slot = var_qbc_db0;
        *var_qbc_db1_slot = var_qbc_db1;
        *var_qbc_dn0_slot = var_qbc_dn0;
        *var_qbc_dn1_slot = var_qbc_dn1;
        *var_qbc_dn10_slot = var_qbc_dn10;
        *var_qbc_dn2_slot = var_qbc_dn2;
        *var_qbc_dn3_slot = var_qbc_dn3;
        *var_qbc_dn4_slot = var_qbc_dn4;
        *var_qbc_dn5_slot = var_qbc_dn5;
        *var_qbc_dn6_slot = var_qbc_dn6;
        *var_qbc_dn7_slot = var_qbc_dn7;
        *var_qbc_dn8_slot = var_qbc_dn8;
        *var_qbc_dn9_slot = var_qbc_dn9;
        *var_qbc_rdb0_slot = var_qbc_rdb0;
        *var_qbc_rdb1_slot = var_qbc_rdb1;
        *var_qbc_rdn0_slot = var_qbc_rdn0;
        *var_qbc_rdn1_slot = var_qbc_rdn1;
        *var_qbc_rdn10_slot = var_qbc_rdn10;
        *var_qbc_rdn2_slot = var_qbc_rdn2;
        *var_qbc_rdn3_slot = var_qbc_rdn3;
        *var_qbc_rdn4_slot = var_qbc_rdn4;
        *var_qbc_rdn5_slot = var_qbc_rdn5;
        *var_qbc_rdn6_slot = var_qbc_rdn6;
        *var_qbc_rdn7_slot = var_qbc_rdn7;
        *var_qbc_rdn8_slot = var_qbc_rdn8;
        *var_qbc_rdn9_slot = var_qbc_rdn9;
        *var_qbc_rv_slot = var_qbc_rv;
        *var_qbe_slot = var_qbe;
        *var_qbe_db0_slot = var_qbe_db0;
        *var_qbe_db1_slot = var_qbe_db1;
        *var_qbe_dn0_slot = var_qbe_dn0;
        *var_qbe_dn1_slot = var_qbe_dn1;
        *var_qbe_dn10_slot = var_qbe_dn10;
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
        *var_qbe_qs_eff_dn2_slot = var_qbe_qs_eff_dn2;
        *var_qbe_qs_eff_dn3_slot = var_qbe_qs_eff_dn3;
        *var_qbe_qs_eff_dn4_slot = var_qbe_qs_eff_dn4;
        *var_qbe_qs_eff_dn5_slot = var_qbe_qs_eff_dn5;
        *var_qbe_qs_eff_dn6_slot = var_qbe_qs_eff_dn6;
        *var_qbe_qs_eff_dn7_slot = var_qbe_qs_eff_dn7;
        *var_qbe_qs_eff_dn8_slot = var_qbe_qs_eff_dn8;
        *var_qbe_qs_eff_dn9_slot = var_qbe_qs_eff_dn9;
        *var_qbe_qs_eff_rdb0_slot = var_qbe_qs_eff_rdb0;
        *var_qbe_qs_eff_rdb1_slot = var_qbe_qs_eff_rdb1;
        *var_qbe_qs_eff_rdn0_slot = var_qbe_qs_eff_rdn0;
        *var_qbe_qs_eff_rdn1_slot = var_qbe_qs_eff_rdn1;
        *var_qbe_qs_eff_rdn10_slot = var_qbe_qs_eff_rdn10;
        *var_qbe_qs_eff_rdn2_slot = var_qbe_qs_eff_rdn2;
        *var_qbe_qs_eff_rdn3_slot = var_qbe_qs_eff_rdn3;
        *var_qbe_qs_eff_rdn4_slot = var_qbe_qs_eff_rdn4;
        *var_qbe_qs_eff_rdn5_slot = var_qbe_qs_eff_rdn5;
        *var_qbe_qs_eff_rdn6_slot = var_qbe_qs_eff_rdn6;
        *var_qbe_qs_eff_rdn7_slot = var_qbe_qs_eff_rdn7;
        *var_qbe_qs_eff_rdn8_slot = var_qbe_qs_eff_rdn8;
        *var_qbe_qs_eff_rdn9_slot = var_qbe_qs_eff_rdn9;
        *var_qbe_qs_eff_rv_slot = var_qbe_qs_eff_rv;
        *var_qbe_rdb0_slot = var_qbe_rdb0;
        *var_qbe_rdb1_slot = var_qbe_rdb1;
        *var_qbe_rdn0_slot = var_qbe_rdn0;
        *var_qbe_rdn1_slot = var_qbe_rdn1;
        *var_qbe_rdn10_slot = var_qbe_rdn10;
        *var_qbe_rdn2_slot = var_qbe_rdn2;
        *var_qbe_rdn3_slot = var_qbe_rdn3;
        *var_qbe_rdn4_slot = var_qbe_rdn4;
        *var_qbe_rdn5_slot = var_qbe_rdn5;
        *var_qbe_rdn6_slot = var_qbe_rdn6;
        *var_qbe_rdn7_slot = var_qbe_rdn7;
        *var_qbe_rdn8_slot = var_qbe_rdn8;
        *var_qbe_rdn9_slot = var_qbe_rdn9;
        *var_qbe_rv_slot = var_qbe_rv;
        *var_qe_slot = var_qe;
        *var_qe_db0_slot = var_qe_db0;
        *var_qe_db1_slot = var_qe_db1;
        *var_qe_dn0_slot = var_qe_dn0;
        *var_qe_dn1_slot = var_qe_dn1;
        *var_qe_dn10_slot = var_qe_dn10;
        *var_qe_dn2_slot = var_qe_dn2;
        *var_qe_dn3_slot = var_qe_dn3;
        *var_qe_dn4_slot = var_qe_dn4;
        *var_qe_dn5_slot = var_qe_dn5;
        *var_qe_dn6_slot = var_qe_dn6;
        *var_qe_dn7_slot = var_qe_dn7;
        *var_qe_dn8_slot = var_qe_dn8;
        *var_qe_dn9_slot = var_qe_dn9;
        *var_qe_rdb0_slot = var_qe_rdb0;
        *var_qe_rdb1_slot = var_qe_rdb1;
        *var_qe_rdn0_slot = var_qe_rdn0;
        *var_qe_rdn1_slot = var_qe_rdn1;
        *var_qe_rdn10_slot = var_qe_rdn10;
        *var_qe_rdn2_slot = var_qe_rdn2;
        *var_qe_rdn3_slot = var_qe_rdn3;
        *var_qe_rdn4_slot = var_qe_rdn4;
        *var_qe_rdn5_slot = var_qe_rdn5;
        *var_qe_rdn6_slot = var_qe_rdn6;
        *var_qe_rdn7_slot = var_qe_rdn7;
        *var_qe_rdn8_slot = var_qe_rdn8;
        *var_qe_rdn9_slot = var_qe_rdn9;
        *var_qe_rv_slot = var_qe_rv;
    }

    pub(super) fn stamp_reactive_block_38(
        p: &Parameters,
        var_in_n: f64,
        var_in_n_db0: f64,
        var_in_n_db1: f64,
        var_in_n_dn0: f64,
        var_in_n_dn1: f64,
        var_in_n_dn10: f64,
        var_in_n_dn2: f64,
        var_in_n_dn3: f64,
        var_in_n_dn4: f64,
        var_in_n_dn5: f64,
        var_in_n_dn6: f64,
        var_in_n_dn7: f64,
        var_in_n_dn8: f64,
        var_in_n_dn9: f64,
        var_q1q: f64,
        var_q1q_db0: f64,
        var_q1q_db1: f64,
        var_q1q_dn0: f64,
        var_q1q_dn1: f64,
        var_q1q_dn10: f64,
        var_q1q_dn2: f64,
        var_q1q_dn3: f64,
        var_q1q_dn4: f64,
        var_q1q_dn5: f64,
        var_q1q_dn6: f64,
        var_q1q_dn7: f64,
        var_q1q_dn8: f64,
        var_q1q_dn9: f64,
        var_qbc: f64,
        var_qbc_db0: f64,
        var_qbc_db1: f64,
        var_qbc_dn0: f64,
        var_qbc_dn1: f64,
        var_qbc_dn10: f64,
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
        var_qbe_dn2: f64,
        var_qbe_dn3: f64,
        var_qbe_dn4: f64,
        var_qbe_dn5: f64,
        var_qbe_dn6: f64,
        var_qbe_dn7: f64,
        var_qbe_dn8: f64,
        var_qbe_dn9: f64,
        var_qbi: f64,
        var_qbi_db0: f64,
        var_qbi_db1: f64,
        var_qbi_dn0: f64,
        var_qbi_dn1: f64,
        var_qbi_dn10: f64,
        var_qbi_dn2: f64,
        var_qbi_dn3: f64,
        var_qbi_dn4: f64,
        var_qbi_dn5: f64,
        var_qbi_dn6: f64,
        var_qbi_dn7: f64,
        var_qbi_dn8: f64,
        var_qbi_dn9: f64,
        var_taub_t: f64,
        var_taub_t_db0: f64,
        var_taub_t_db1: f64,
        var_taub_t_dn0: f64,
        var_taub_t_dn1: f64,
        var_taub_t_dn10: f64,
        var_taub_t_dn2: f64,
        var_taub_t_dn3: f64,
        var_taub_t_dn4: f64,
        var_taub_t_dn5: f64,
        var_taub_t_dn6: f64,
        var_taub_t_dn7: f64,
        var_taub_t_dn8: f64,
        var_taub_t_dn9: f64,
        var_guard118_slot: &mut f64,
        var_guard118_db0_slot: &mut f64,
        var_guard118_db1_slot: &mut f64,
        var_guard118_dn0_slot: &mut f64,
        var_guard118_dn1_slot: &mut f64,
        var_guard118_dn10_slot: &mut f64,
        var_guard118_dn2_slot: &mut f64,
        var_guard118_dn3_slot: &mut f64,
        var_guard118_dn4_slot: &mut f64,
        var_guard118_dn5_slot: &mut f64,
        var_guard118_dn6_slot: &mut f64,
        var_guard118_dn7_slot: &mut f64,
        var_guard118_dn8_slot: &mut f64,
        var_guard118_dn9_slot: &mut f64,
        var_guard118_rdb0_slot: &mut f64,
        var_guard118_rdb1_slot: &mut f64,
        var_guard118_rdn0_slot: &mut f64,
        var_guard118_rdn1_slot: &mut f64,
        var_guard118_rdn10_slot: &mut f64,
        var_guard118_rdn2_slot: &mut f64,
        var_guard118_rdn3_slot: &mut f64,
        var_guard118_rdn4_slot: &mut f64,
        var_guard118_rdn5_slot: &mut f64,
        var_guard118_rdn6_slot: &mut f64,
        var_guard118_rdn7_slot: &mut f64,
        var_guard118_rdn8_slot: &mut f64,
        var_guard118_rdn9_slot: &mut f64,
        var_guard118_rv_slot: &mut f64,
        var_guard119_slot: &mut f64,
        var_guard119_db0_slot: &mut f64,
        var_guard119_db1_slot: &mut f64,
        var_guard119_dn0_slot: &mut f64,
        var_guard119_dn1_slot: &mut f64,
        var_guard119_dn10_slot: &mut f64,
        var_guard119_dn2_slot: &mut f64,
        var_guard119_dn3_slot: &mut f64,
        var_guard119_dn4_slot: &mut f64,
        var_guard119_dn5_slot: &mut f64,
        var_guard119_dn6_slot: &mut f64,
        var_guard119_dn7_slot: &mut f64,
        var_guard119_dn8_slot: &mut f64,
        var_guard119_dn9_slot: &mut f64,
        var_guard119_rdb0_slot: &mut f64,
        var_guard119_rdb1_slot: &mut f64,
        var_guard119_rdn0_slot: &mut f64,
        var_guard119_rdn1_slot: &mut f64,
        var_guard119_rdn10_slot: &mut f64,
        var_guard119_rdn2_slot: &mut f64,
        var_guard119_rdn3_slot: &mut f64,
        var_guard119_rdn4_slot: &mut f64,
        var_guard119_rdn5_slot: &mut f64,
        var_guard119_rdn6_slot: &mut f64,
        var_guard119_rdn7_slot: &mut f64,
        var_guard119_rdn8_slot: &mut f64,
        var_guard119_rdn9_slot: &mut f64,
        var_guard119_rv_slot: &mut f64,
        var_guard120_slot: &mut f64,
        var_guard120_db0_slot: &mut f64,
        var_guard120_db1_slot: &mut f64,
        var_guard120_dn0_slot: &mut f64,
        var_guard120_dn1_slot: &mut f64,
        var_guard120_dn10_slot: &mut f64,
        var_guard120_dn2_slot: &mut f64,
        var_guard120_dn3_slot: &mut f64,
        var_guard120_dn4_slot: &mut f64,
        var_guard120_dn5_slot: &mut f64,
        var_guard120_dn6_slot: &mut f64,
        var_guard120_dn7_slot: &mut f64,
        var_guard120_dn8_slot: &mut f64,
        var_guard120_dn9_slot: &mut f64,
        var_guard120_rdb0_slot: &mut f64,
        var_guard120_rdb1_slot: &mut f64,
        var_guard120_rdn0_slot: &mut f64,
        var_guard120_rdn1_slot: &mut f64,
        var_guard120_rdn10_slot: &mut f64,
        var_guard120_rdn2_slot: &mut f64,
        var_guard120_rdn3_slot: &mut f64,
        var_guard120_rdn4_slot: &mut f64,
        var_guard120_rdn5_slot: &mut f64,
        var_guard120_rdn6_slot: &mut f64,
        var_guard120_rdn7_slot: &mut f64,
        var_guard120_rdn8_slot: &mut f64,
        var_guard120_rdn9_slot: &mut f64,
        var_guard120_rv_slot: &mut f64,
        var_taub_n_slot: &mut f64,
        var_taub_n_db0_slot: &mut f64,
        var_taub_n_db1_slot: &mut f64,
        var_taub_n_dn0_slot: &mut f64,
        var_taub_n_dn1_slot: &mut f64,
        var_taub_n_dn10_slot: &mut f64,
        var_taub_n_dn2_slot: &mut f64,
        var_taub_n_dn3_slot: &mut f64,
        var_taub_n_dn4_slot: &mut f64,
        var_taub_n_dn5_slot: &mut f64,
        var_taub_n_dn6_slot: &mut f64,
        var_taub_n_dn7_slot: &mut f64,
        var_taub_n_dn8_slot: &mut f64,
        var_taub_n_dn9_slot: &mut f64,
        var_taub_n_rdb0_slot: &mut f64,
        var_taub_n_rdb1_slot: &mut f64,
        var_taub_n_rdn0_slot: &mut f64,
        var_taub_n_rdn1_slot: &mut f64,
        var_taub_n_rdn10_slot: &mut f64,
        var_taub_n_rdn2_slot: &mut f64,
        var_taub_n_rdn3_slot: &mut f64,
        var_taub_n_rdn4_slot: &mut f64,
        var_taub_n_rdn5_slot: &mut f64,
        var_taub_n_rdn6_slot: &mut f64,
        var_taub_n_rdn7_slot: &mut f64,
        var_taub_n_rdn8_slot: &mut f64,
        var_taub_n_rdn9_slot: &mut f64,
        var_taub_n_rv_slot: &mut f64,
        var_taun_slot: &mut f64,
        var_taun_db0_slot: &mut f64,
        var_taun_db1_slot: &mut f64,
        var_taun_dn0_slot: &mut f64,
        var_taun_dn1_slot: &mut f64,
        var_taun_dn10_slot: &mut f64,
        var_taun_dn2_slot: &mut f64,
        var_taun_dn3_slot: &mut f64,
        var_taun_dn4_slot: &mut f64,
        var_taun_dn5_slot: &mut f64,
        var_taun_dn6_slot: &mut f64,
        var_taun_dn7_slot: &mut f64,
        var_taun_dn8_slot: &mut f64,
        var_taun_dn9_slot: &mut f64,
        var_taun_rdb0_slot: &mut f64,
        var_taun_rdb1_slot: &mut f64,
        var_taun_rdn0_slot: &mut f64,
        var_taun_rdn1_slot: &mut f64,
        var_taun_rdn10_slot: &mut f64,
        var_taun_rdn2_slot: &mut f64,
        var_taun_rdn3_slot: &mut f64,
        var_taun_rdn4_slot: &mut f64,
        var_taun_rdn5_slot: &mut f64,
        var_taun_rdn6_slot: &mut f64,
        var_taun_rdn7_slot: &mut f64,
        var_taun_rdn8_slot: &mut f64,
        var_taun_rdn9_slot: &mut f64,
        var_taun_rv_slot: &mut f64,
    ) {
        let mut var_guard118: f64 = *var_guard118_slot;
        let mut var_guard118_db0: f64 = *var_guard118_db0_slot;
        let mut var_guard118_db1: f64 = *var_guard118_db1_slot;
        let mut var_guard118_dn0: f64 = *var_guard118_dn0_slot;
        let mut var_guard118_dn1: f64 = *var_guard118_dn1_slot;
        let mut var_guard118_dn10: f64 = *var_guard118_dn10_slot;
        let mut var_guard118_dn2: f64 = *var_guard118_dn2_slot;
        let mut var_guard118_dn3: f64 = *var_guard118_dn3_slot;
        let mut var_guard118_dn4: f64 = *var_guard118_dn4_slot;
        let mut var_guard118_dn5: f64 = *var_guard118_dn5_slot;
        let mut var_guard118_dn6: f64 = *var_guard118_dn6_slot;
        let mut var_guard118_dn7: f64 = *var_guard118_dn7_slot;
        let mut var_guard118_dn8: f64 = *var_guard118_dn8_slot;
        let mut var_guard118_dn9: f64 = *var_guard118_dn9_slot;
        let mut var_guard118_rdb0: f64 = *var_guard118_rdb0_slot;
        let mut var_guard118_rdb1: f64 = *var_guard118_rdb1_slot;
        let mut var_guard118_rdn0: f64 = *var_guard118_rdn0_slot;
        let mut var_guard118_rdn1: f64 = *var_guard118_rdn1_slot;
        let mut var_guard118_rdn10: f64 = *var_guard118_rdn10_slot;
        let mut var_guard118_rdn2: f64 = *var_guard118_rdn2_slot;
        let mut var_guard118_rdn3: f64 = *var_guard118_rdn3_slot;
        let mut var_guard118_rdn4: f64 = *var_guard118_rdn4_slot;
        let mut var_guard118_rdn5: f64 = *var_guard118_rdn5_slot;
        let mut var_guard118_rdn6: f64 = *var_guard118_rdn6_slot;
        let mut var_guard118_rdn7: f64 = *var_guard118_rdn7_slot;
        let mut var_guard118_rdn8: f64 = *var_guard118_rdn8_slot;
        let mut var_guard118_rdn9: f64 = *var_guard118_rdn9_slot;
        let mut var_guard118_rv: f64 = *var_guard118_rv_slot;
        let mut var_guard119: f64 = *var_guard119_slot;
        let mut var_guard119_db0: f64 = *var_guard119_db0_slot;
        let mut var_guard119_db1: f64 = *var_guard119_db1_slot;
        let mut var_guard119_dn0: f64 = *var_guard119_dn0_slot;
        let mut var_guard119_dn1: f64 = *var_guard119_dn1_slot;
        let mut var_guard119_dn10: f64 = *var_guard119_dn10_slot;
        let mut var_guard119_dn2: f64 = *var_guard119_dn2_slot;
        let mut var_guard119_dn3: f64 = *var_guard119_dn3_slot;
        let mut var_guard119_dn4: f64 = *var_guard119_dn4_slot;
        let mut var_guard119_dn5: f64 = *var_guard119_dn5_slot;
        let mut var_guard119_dn6: f64 = *var_guard119_dn6_slot;
        let mut var_guard119_dn7: f64 = *var_guard119_dn7_slot;
        let mut var_guard119_dn8: f64 = *var_guard119_dn8_slot;
        let mut var_guard119_dn9: f64 = *var_guard119_dn9_slot;
        let mut var_guard119_rdb0: f64 = *var_guard119_rdb0_slot;
        let mut var_guard119_rdb1: f64 = *var_guard119_rdb1_slot;
        let mut var_guard119_rdn0: f64 = *var_guard119_rdn0_slot;
        let mut var_guard119_rdn1: f64 = *var_guard119_rdn1_slot;
        let mut var_guard119_rdn10: f64 = *var_guard119_rdn10_slot;
        let mut var_guard119_rdn2: f64 = *var_guard119_rdn2_slot;
        let mut var_guard119_rdn3: f64 = *var_guard119_rdn3_slot;
        let mut var_guard119_rdn4: f64 = *var_guard119_rdn4_slot;
        let mut var_guard119_rdn5: f64 = *var_guard119_rdn5_slot;
        let mut var_guard119_rdn6: f64 = *var_guard119_rdn6_slot;
        let mut var_guard119_rdn7: f64 = *var_guard119_rdn7_slot;
        let mut var_guard119_rdn8: f64 = *var_guard119_rdn8_slot;
        let mut var_guard119_rdn9: f64 = *var_guard119_rdn9_slot;
        let mut var_guard119_rv: f64 = *var_guard119_rv_slot;
        let mut var_guard120: f64 = *var_guard120_slot;
        let mut var_guard120_db0: f64 = *var_guard120_db0_slot;
        let mut var_guard120_db1: f64 = *var_guard120_db1_slot;
        let mut var_guard120_dn0: f64 = *var_guard120_dn0_slot;
        let mut var_guard120_dn1: f64 = *var_guard120_dn1_slot;
        let mut var_guard120_dn10: f64 = *var_guard120_dn10_slot;
        let mut var_guard120_dn2: f64 = *var_guard120_dn2_slot;
        let mut var_guard120_dn3: f64 = *var_guard120_dn3_slot;
        let mut var_guard120_dn4: f64 = *var_guard120_dn4_slot;
        let mut var_guard120_dn5: f64 = *var_guard120_dn5_slot;
        let mut var_guard120_dn6: f64 = *var_guard120_dn6_slot;
        let mut var_guard120_dn7: f64 = *var_guard120_dn7_slot;
        let mut var_guard120_dn8: f64 = *var_guard120_dn8_slot;
        let mut var_guard120_dn9: f64 = *var_guard120_dn9_slot;
        let mut var_guard120_rdb0: f64 = *var_guard120_rdb0_slot;
        let mut var_guard120_rdb1: f64 = *var_guard120_rdb1_slot;
        let mut var_guard120_rdn0: f64 = *var_guard120_rdn0_slot;
        let mut var_guard120_rdn1: f64 = *var_guard120_rdn1_slot;
        let mut var_guard120_rdn10: f64 = *var_guard120_rdn10_slot;
        let mut var_guard120_rdn2: f64 = *var_guard120_rdn2_slot;
        let mut var_guard120_rdn3: f64 = *var_guard120_rdn3_slot;
        let mut var_guard120_rdn4: f64 = *var_guard120_rdn4_slot;
        let mut var_guard120_rdn5: f64 = *var_guard120_rdn5_slot;
        let mut var_guard120_rdn6: f64 = *var_guard120_rdn6_slot;
        let mut var_guard120_rdn7: f64 = *var_guard120_rdn7_slot;
        let mut var_guard120_rdn8: f64 = *var_guard120_rdn8_slot;
        let mut var_guard120_rdn9: f64 = *var_guard120_rdn9_slot;
        let mut var_guard120_rv: f64 = *var_guard120_rv_slot;
        let mut var_taub_n: f64 = *var_taub_n_slot;
        let mut var_taub_n_db0: f64 = *var_taub_n_db0_slot;
        let mut var_taub_n_db1: f64 = *var_taub_n_db1_slot;
        let mut var_taub_n_dn0: f64 = *var_taub_n_dn0_slot;
        let mut var_taub_n_dn1: f64 = *var_taub_n_dn1_slot;
        let mut var_taub_n_dn10: f64 = *var_taub_n_dn10_slot;
        let mut var_taub_n_dn2: f64 = *var_taub_n_dn2_slot;
        let mut var_taub_n_dn3: f64 = *var_taub_n_dn3_slot;
        let mut var_taub_n_dn4: f64 = *var_taub_n_dn4_slot;
        let mut var_taub_n_dn5: f64 = *var_taub_n_dn5_slot;
        let mut var_taub_n_dn6: f64 = *var_taub_n_dn6_slot;
        let mut var_taub_n_dn7: f64 = *var_taub_n_dn7_slot;
        let mut var_taub_n_dn8: f64 = *var_taub_n_dn8_slot;
        let mut var_taub_n_dn9: f64 = *var_taub_n_dn9_slot;
        let mut var_taub_n_rdb0: f64 = *var_taub_n_rdb0_slot;
        let mut var_taub_n_rdb1: f64 = *var_taub_n_rdb1_slot;
        let mut var_taub_n_rdn0: f64 = *var_taub_n_rdn0_slot;
        let mut var_taub_n_rdn1: f64 = *var_taub_n_rdn1_slot;
        let mut var_taub_n_rdn10: f64 = *var_taub_n_rdn10_slot;
        let mut var_taub_n_rdn2: f64 = *var_taub_n_rdn2_slot;
        let mut var_taub_n_rdn3: f64 = *var_taub_n_rdn3_slot;
        let mut var_taub_n_rdn4: f64 = *var_taub_n_rdn4_slot;
        let mut var_taub_n_rdn5: f64 = *var_taub_n_rdn5_slot;
        let mut var_taub_n_rdn6: f64 = *var_taub_n_rdn6_slot;
        let mut var_taub_n_rdn7: f64 = *var_taub_n_rdn7_slot;
        let mut var_taub_n_rdn8: f64 = *var_taub_n_rdn8_slot;
        let mut var_taub_n_rdn9: f64 = *var_taub_n_rdn9_slot;
        let mut var_taub_n_rv: f64 = *var_taub_n_rv_slot;
        let mut var_taun: f64 = *var_taun_slot;
        let mut var_taun_db0: f64 = *var_taun_db0_slot;
        let mut var_taun_db1: f64 = *var_taun_db1_slot;
        let mut var_taun_dn0: f64 = *var_taun_dn0_slot;
        let mut var_taun_dn1: f64 = *var_taun_dn1_slot;
        let mut var_taun_dn10: f64 = *var_taun_dn10_slot;
        let mut var_taun_dn2: f64 = *var_taun_dn2_slot;
        let mut var_taun_dn3: f64 = *var_taun_dn3_slot;
        let mut var_taun_dn4: f64 = *var_taun_dn4_slot;
        let mut var_taun_dn5: f64 = *var_taun_dn5_slot;
        let mut var_taun_dn6: f64 = *var_taun_dn6_slot;
        let mut var_taun_dn7: f64 = *var_taun_dn7_slot;
        let mut var_taun_dn8: f64 = *var_taun_dn8_slot;
        let mut var_taun_dn9: f64 = *var_taun_dn9_slot;
        let mut var_taun_rdb0: f64 = *var_taun_rdb0_slot;
        let mut var_taun_rdb1: f64 = *var_taun_rdb1_slot;
        let mut var_taun_rdn0: f64 = *var_taun_rdn0_slot;
        let mut var_taun_rdn1: f64 = *var_taun_rdn1_slot;
        let mut var_taun_rdn10: f64 = *var_taun_rdn10_slot;
        let mut var_taun_rdn2: f64 = *var_taun_rdn2_slot;
        let mut var_taun_rdn3: f64 = *var_taun_rdn3_slot;
        let mut var_taun_rdn4: f64 = *var_taun_rdn4_slot;
        let mut var_taun_rdn5: f64 = *var_taun_rdn5_slot;
        let mut var_taun_rdn6: f64 = *var_taun_rdn6_slot;
        let mut var_taun_rdn7: f64 = *var_taun_rdn7_slot;
        let mut var_taun_rdn8: f64 = *var_taun_rdn8_slot;
        let mut var_taun_rdn9: f64 = *var_taun_rdn9_slot;
        let mut var_taun_rv: f64 = *var_taun_rv_slot;

        let assign6500_e6505: f64 = if var_in_n > 0.0 { 1.0 } else { 0.0 };
        var_guard118 = assign6500_e6505;
        var_guard118_dn0 = 0.0;
        var_guard118_dn1 = 0.0;
        var_guard118_dn2 = 0.0;
        var_guard118_dn3 = 0.0;
        var_guard118_dn4 = 0.0;
        var_guard118_dn5 = 0.0;
        var_guard118_dn6 = 0.0;
        var_guard118_dn7 = 0.0;
        var_guard118_dn8 = 0.0;
        var_guard118_dn9 = 0.0;
        var_guard118_dn10 = 0.0;
        var_guard118_db0 = 0.0;
        var_guard118_db1 = 0.0;
        var_guard118_rv = 0.0;
        var_guard118_rdn0 = 0.0;
        var_guard118_rdn1 = 0.0;
        var_guard118_rdn2 = 0.0;
        var_guard118_rdn3 = 0.0;
        var_guard118_rdn4 = 0.0;
        var_guard118_rdn5 = 0.0;
        var_guard118_rdn6 = 0.0;
        var_guard118_rdn7 = 0.0;
        var_guard118_rdn8 = 0.0;
        var_guard118_rdn9 = 0.0;
        var_guard118_rdn10 = 0.0;
        var_guard118_rdb0 = 0.0;
        var_guard118_rdb1 = 0.0;

        let (assign6510_e6513, assign6510_e6513_d_n0, assign6510_e6513_d_n1, assign6510_e6513_d_n2, assign6510_e6513_d_n3, assign6510_e6513_d_n4, assign6510_e6513_d_n5, assign6510_e6513_d_n6, assign6510_e6513_d_n7, assign6510_e6513_d_n8, assign6510_e6513_d_n9, assign6510_e6513_d_n10, assign6510_e6513_d_b0, assign6510_e6513_d_b1,) = {
    if (var_guard118 != 0.0) {
        let assign6510_e6509: f64 = (var_qbe + var_qbc);
        let assign6510_e6511: f64 = (assign6510_e6509 / var_in_n);
        (assign6510_e6511, ((((var_qbe_dn0 + var_qbc_dn0) * var_in_n) - (assign6510_e6509 * var_in_n_dn0)) / (var_in_n * var_in_n)), ((((var_qbe_dn1 + var_qbc_dn1) * var_in_n) - (assign6510_e6509 * var_in_n_dn1)) / (var_in_n * var_in_n)), ((((var_qbe_dn2 + var_qbc_dn2) * var_in_n) - (assign6510_e6509 * var_in_n_dn2)) / (var_in_n * var_in_n)), ((((var_qbe_dn3 + var_qbc_dn3) * var_in_n) - (assign6510_e6509 * var_in_n_dn3)) / (var_in_n * var_in_n)), ((((var_qbe_dn4 + var_qbc_dn4) * var_in_n) - (assign6510_e6509 * var_in_n_dn4)) / (var_in_n * var_in_n)), ((((var_qbe_dn5 + var_qbc_dn5) * var_in_n) - (assign6510_e6509 * var_in_n_dn5)) / (var_in_n * var_in_n)), ((((var_qbe_dn6 + var_qbc_dn6) * var_in_n) - (assign6510_e6509 * var_in_n_dn6)) / (var_in_n * var_in_n)), ((((var_qbe_dn7 + var_qbc_dn7) * var_in_n) - (assign6510_e6509 * var_in_n_dn7)) / (var_in_n * var_in_n)), ((((var_qbe_dn8 + var_qbc_dn8) * var_in_n) - (assign6510_e6509 * var_in_n_dn8)) / (var_in_n * var_in_n)), ((((var_qbe_dn9 + var_qbc_dn9) * var_in_n) - (assign6510_e6509 * var_in_n_dn9)) / (var_in_n * var_in_n)), ((((var_qbe_dn10 + var_qbc_dn10) * var_in_n) - (assign6510_e6509 * var_in_n_dn10)) / (var_in_n * var_in_n)), ((((var_qbe_db0 + var_qbc_db0) * var_in_n) - (assign6510_e6509 * var_in_n_db0)) / (var_in_n * var_in_n)), ((((var_qbe_db1 + var_qbc_db1) * var_in_n) - (assign6510_e6509 * var_in_n_db1)) / (var_in_n * var_in_n)),)
    } else {
        (var_taub_n, var_taub_n_dn0, var_taub_n_dn1, var_taub_n_dn2, var_taub_n_dn3, var_taub_n_dn4, var_taub_n_dn5, var_taub_n_dn6, var_taub_n_dn7, var_taub_n_dn8, var_taub_n_dn9, var_taub_n_dn10, var_taub_n_db0, var_taub_n_db1,)
    }
};
        var_taub_n = assign6510_e6513;
        var_taub_n_dn0 = assign6510_e6513_d_n0;
        var_taub_n_dn1 = assign6510_e6513_d_n1;
        var_taub_n_dn2 = assign6510_e6513_d_n2;
        var_taub_n_dn3 = assign6510_e6513_d_n3;
        var_taub_n_dn4 = assign6510_e6513_d_n4;
        var_taub_n_dn5 = assign6510_e6513_d_n5;
        var_taub_n_dn6 = assign6510_e6513_d_n6;
        var_taub_n_dn7 = assign6510_e6513_d_n7;
        var_taub_n_dn8 = assign6510_e6513_d_n8;
        var_taub_n_dn9 = assign6510_e6513_d_n9;
        var_taub_n_dn10 = assign6510_e6513_d_n10;
        var_taub_n_db0 = assign6510_e6513_d_b0;
        var_taub_n_db1 = assign6510_e6513_d_b1;
        var_taub_n_rv = 0.0;
        var_taub_n_rdn0 = 0.0;
        var_taub_n_rdn1 = 0.0;
        var_taub_n_rdn2 = 0.0;
        var_taub_n_rdn3 = 0.0;
        var_taub_n_rdn4 = 0.0;
        var_taub_n_rdn5 = 0.0;
        var_taub_n_rdn6 = 0.0;
        var_taub_n_rdn7 = 0.0;
        var_taub_n_rdn8 = 0.0;
        var_taub_n_rdn9 = 0.0;
        var_taub_n_rdn10 = 0.0;
        var_taub_n_rdb0 = 0.0;
        var_taub_n_rdb1 = 0.0;

        let (assign6520_e6522, assign6520_e6522_d_n0, assign6520_e6522_d_n1, assign6520_e6522_d_n2, assign6520_e6522_d_n3, assign6520_e6522_d_n4, assign6520_e6522_d_n5, assign6520_e6522_d_n6, assign6520_e6522_d_n7, assign6520_e6522_d_n8, assign6520_e6522_d_n9, assign6520_e6522_d_n10, assign6520_e6522_d_b0, assign6520_e6522_d_b1,) = {
    if (var_guard118 == 0.0) {
        let assign6520_e6518: f64 = (var_taub_t * var_q1q);
        let assign6520_e6520: f64 = (assign6520_e6518 * var_qbi);
        (assign6520_e6520, ((((var_taub_t_dn0 * var_q1q) + (var_taub_t * var_q1q_dn0)) * var_qbi) + (assign6520_e6518 * var_qbi_dn0)), ((((var_taub_t_dn1 * var_q1q) + (var_taub_t * var_q1q_dn1)) * var_qbi) + (assign6520_e6518 * var_qbi_dn1)), ((((var_taub_t_dn2 * var_q1q) + (var_taub_t * var_q1q_dn2)) * var_qbi) + (assign6520_e6518 * var_qbi_dn2)), ((((var_taub_t_dn3 * var_q1q) + (var_taub_t * var_q1q_dn3)) * var_qbi) + (assign6520_e6518 * var_qbi_dn3)), ((((var_taub_t_dn4 * var_q1q) + (var_taub_t * var_q1q_dn4)) * var_qbi) + (assign6520_e6518 * var_qbi_dn4)), ((((var_taub_t_dn5 * var_q1q) + (var_taub_t * var_q1q_dn5)) * var_qbi) + (assign6520_e6518 * var_qbi_dn5)), ((((var_taub_t_dn6 * var_q1q) + (var_taub_t * var_q1q_dn6)) * var_qbi) + (assign6520_e6518 * var_qbi_dn6)), ((((var_taub_t_dn7 * var_q1q) + (var_taub_t * var_q1q_dn7)) * var_qbi) + (assign6520_e6518 * var_qbi_dn7)), ((((var_taub_t_dn8 * var_q1q) + (var_taub_t * var_q1q_dn8)) * var_qbi) + (assign6520_e6518 * var_qbi_dn8)), ((((var_taub_t_dn9 * var_q1q) + (var_taub_t * var_q1q_dn9)) * var_qbi) + (assign6520_e6518 * var_qbi_dn9)), ((((var_taub_t_dn10 * var_q1q) + (var_taub_t * var_q1q_dn10)) * var_qbi) + (assign6520_e6518 * var_qbi_dn10)), ((((var_taub_t_db0 * var_q1q) + (var_taub_t * var_q1q_db0)) * var_qbi) + (assign6520_e6518 * var_qbi_db0)), ((((var_taub_t_db1 * var_q1q) + (var_taub_t * var_q1q_db1)) * var_qbi) + (assign6520_e6518 * var_qbi_db1)),)
    } else {
        (var_taub_n, var_taub_n_dn0, var_taub_n_dn1, var_taub_n_dn2, var_taub_n_dn3, var_taub_n_dn4, var_taub_n_dn5, var_taub_n_dn6, var_taub_n_dn7, var_taub_n_dn8, var_taub_n_dn9, var_taub_n_dn10, var_taub_n_db0, var_taub_n_db1,)
    }
};
        var_taub_n = assign6520_e6522;
        var_taub_n_dn0 = assign6520_e6522_d_n0;
        var_taub_n_dn1 = assign6520_e6522_d_n1;
        var_taub_n_dn2 = assign6520_e6522_d_n2;
        var_taub_n_dn3 = assign6520_e6522_d_n3;
        var_taub_n_dn4 = assign6520_e6522_d_n4;
        var_taub_n_dn5 = assign6520_e6522_d_n5;
        var_taub_n_dn6 = assign6520_e6522_d_n6;
        var_taub_n_dn7 = assign6520_e6522_d_n7;
        var_taub_n_dn8 = assign6520_e6522_d_n8;
        var_taub_n_dn9 = assign6520_e6522_d_n9;
        var_taub_n_dn10 = assign6520_e6522_d_n10;
        var_taub_n_db0 = assign6520_e6522_d_b0;
        var_taub_n_db1 = assign6520_e6522_d_b1;
        var_taub_n_rv = 0.0;
        var_taub_n_rdn0 = 0.0;
        var_taub_n_rdn1 = 0.0;
        var_taub_n_rdn2 = 0.0;
        var_taub_n_rdn3 = 0.0;
        var_taub_n_rdn4 = 0.0;
        var_taub_n_rdn5 = 0.0;
        var_taub_n_rdn6 = 0.0;
        var_taub_n_rdn7 = 0.0;
        var_taub_n_rdn8 = 0.0;
        var_taub_n_rdn9 = 0.0;
        var_taub_n_rdn10 = 0.0;
        var_taub_n_rdb0 = 0.0;
        var_taub_n_rdb1 = 0.0;

        let assign6530_e6525: f64 = if p.p130 == 1.0 { 1.0 } else { 0.0 };
        var_guard119 = assign6530_e6525;
        var_guard119_dn0 = 0.0;
        var_guard119_dn1 = 0.0;
        var_guard119_dn2 = 0.0;
        var_guard119_dn3 = 0.0;
        var_guard119_dn4 = 0.0;
        var_guard119_dn5 = 0.0;
        var_guard119_dn6 = 0.0;
        var_guard119_dn7 = 0.0;
        var_guard119_dn8 = 0.0;
        var_guard119_dn9 = 0.0;
        var_guard119_dn10 = 0.0;
        var_guard119_db0 = 0.0;
        var_guard119_db1 = 0.0;
        var_guard119_rv = 0.0;
        var_guard119_rdn0 = 0.0;
        var_guard119_rdn1 = 0.0;
        var_guard119_rdn2 = 0.0;
        var_guard119_rdn3 = 0.0;
        var_guard119_rdn4 = 0.0;
        var_guard119_rdn5 = 0.0;
        var_guard119_rdn6 = 0.0;
        var_guard119_rdn7 = 0.0;
        var_guard119_rdn8 = 0.0;
        var_guard119_rdn9 = 0.0;
        var_guard119_rdn10 = 0.0;
        var_guard119_rdb0 = 0.0;
        var_guard119_rdb1 = 0.0;

        let (assign6540_e6531, assign6540_e6531_d_n0, assign6540_e6531_d_n1, assign6540_e6531_d_n2, assign6540_e6531_d_n3, assign6540_e6531_d_n4, assign6540_e6531_d_n5, assign6540_e6531_d_n6, assign6540_e6531_d_n7, assign6540_e6531_d_n8, assign6540_e6531_d_n9, assign6540_e6531_d_n10, assign6540_e6531_d_b0, assign6540_e6531_d_b1,) = {
    if (var_guard119 != 0.0) {
        let assign6540_e6529: f64 = (p.p93 * var_taub_n);
        (assign6540_e6529, (p.p93 * var_taub_n_dn0), (p.p93 * var_taub_n_dn1), (p.p93 * var_taub_n_dn2), (p.p93 * var_taub_n_dn3), (p.p93 * var_taub_n_dn4), (p.p93 * var_taub_n_dn5), (p.p93 * var_taub_n_dn6), (p.p93 * var_taub_n_dn7), (p.p93 * var_taub_n_dn8), (p.p93 * var_taub_n_dn9), (p.p93 * var_taub_n_dn10), (p.p93 * var_taub_n_db0), (p.p93 * var_taub_n_db1),)
    } else {
        (var_taun, var_taun_dn0, var_taun_dn1, var_taun_dn2, var_taun_dn3, var_taun_dn4, var_taun_dn5, var_taun_dn6, var_taun_dn7, var_taun_dn8, var_taun_dn9, var_taun_dn10, var_taun_db0, var_taun_db1,)
    }
};
        var_taun = assign6540_e6531;
        var_taun_dn0 = assign6540_e6531_d_n0;
        var_taun_dn1 = assign6540_e6531_d_n1;
        var_taun_dn2 = assign6540_e6531_d_n2;
        var_taun_dn3 = assign6540_e6531_d_n3;
        var_taun_dn4 = assign6540_e6531_d_n4;
        var_taun_dn5 = assign6540_e6531_d_n5;
        var_taun_dn6 = assign6540_e6531_d_n6;
        var_taun_dn7 = assign6540_e6531_d_n7;
        var_taun_dn8 = assign6540_e6531_d_n8;
        var_taun_dn9 = assign6540_e6531_d_n9;
        var_taun_dn10 = assign6540_e6531_d_n10;
        var_taun_db0 = assign6540_e6531_d_b0;
        var_taun_db1 = assign6540_e6531_d_b1;
        var_taun_rv = 0.0;
        var_taun_rdn0 = 0.0;
        var_taun_rdn1 = 0.0;
        var_taun_rdn2 = 0.0;
        var_taun_rdn3 = 0.0;
        var_taun_rdn4 = 0.0;
        var_taun_rdn5 = 0.0;
        var_taun_rdn6 = 0.0;
        var_taun_rdn7 = 0.0;
        var_taun_rdn8 = 0.0;
        var_taun_rdn9 = 0.0;
        var_taun_rdn10 = 0.0;
        var_taun_rdb0 = 0.0;
        var_taun_rdb1 = 0.0;

        let assign6550_e6534: f64 = if p.p130 == 2.0 { 1.0 } else { 0.0 };
        var_guard120 = assign6550_e6534;
        var_guard120_dn0 = 0.0;
        var_guard120_dn1 = 0.0;
        var_guard120_dn2 = 0.0;
        var_guard120_dn3 = 0.0;
        var_guard120_dn4 = 0.0;
        var_guard120_dn5 = 0.0;
        var_guard120_dn6 = 0.0;
        var_guard120_dn7 = 0.0;
        var_guard120_dn8 = 0.0;
        var_guard120_dn9 = 0.0;
        var_guard120_dn10 = 0.0;
        var_guard120_db0 = 0.0;
        var_guard120_db1 = 0.0;
        var_guard120_rv = 0.0;
        var_guard120_rdn0 = 0.0;
        var_guard120_rdn1 = 0.0;
        var_guard120_rdn2 = 0.0;
        var_guard120_rdn3 = 0.0;
        var_guard120_rdn4 = 0.0;
        var_guard120_rdn5 = 0.0;
        var_guard120_rdn6 = 0.0;
        var_guard120_rdn7 = 0.0;
        var_guard120_rdn8 = 0.0;
        var_guard120_rdn9 = 0.0;
        var_guard120_rdn10 = 0.0;
        var_guard120_rdb0 = 0.0;
        var_guard120_rdb1 = 0.0;

        let (assign6560_e6543, assign6560_e6543_d_n0, assign6560_e6543_d_n1, assign6560_e6543_d_n2, assign6560_e6543_d_n3, assign6560_e6543_d_n4, assign6560_e6543_d_n5, assign6560_e6543_d_n6, assign6560_e6543_d_n7, assign6560_e6543_d_n8, assign6560_e6543_d_n9, assign6560_e6543_d_n10, assign6560_e6543_d_b0, assign6560_e6543_d_b1,) = {
    if ((var_guard119 == 0.0) && (var_guard120 != 0.0)) {
        let assign6560_e6541: f64 = (p.p131 * var_taub_n);
        (assign6560_e6541, (p.p131 * var_taub_n_dn0), (p.p131 * var_taub_n_dn1), (p.p131 * var_taub_n_dn2), (p.p131 * var_taub_n_dn3), (p.p131 * var_taub_n_dn4), (p.p131 * var_taub_n_dn5), (p.p131 * var_taub_n_dn6), (p.p131 * var_taub_n_dn7), (p.p131 * var_taub_n_dn8), (p.p131 * var_taub_n_dn9), (p.p131 * var_taub_n_dn10), (p.p131 * var_taub_n_db0), (p.p131 * var_taub_n_db1),)
    } else {
        (var_taun, var_taun_dn0, var_taun_dn1, var_taun_dn2, var_taun_dn3, var_taun_dn4, var_taun_dn5, var_taun_dn6, var_taun_dn7, var_taun_dn8, var_taun_dn9, var_taun_dn10, var_taun_db0, var_taun_db1,)
    }
};
        var_taun = assign6560_e6543;
        var_taun_dn0 = assign6560_e6543_d_n0;
        var_taun_dn1 = assign6560_e6543_d_n1;
        var_taun_dn2 = assign6560_e6543_d_n2;
        var_taun_dn3 = assign6560_e6543_d_n3;
        var_taun_dn4 = assign6560_e6543_d_n4;
        var_taun_dn5 = assign6560_e6543_d_n5;
        var_taun_dn6 = assign6560_e6543_d_n6;
        var_taun_dn7 = assign6560_e6543_d_n7;
        var_taun_dn8 = assign6560_e6543_d_n8;
        var_taun_dn9 = assign6560_e6543_d_n9;
        var_taun_dn10 = assign6560_e6543_d_n10;
        var_taun_db0 = assign6560_e6543_d_b0;
        var_taun_db1 = assign6560_e6543_d_b1;
        var_taun_rv = 0.0;
        var_taun_rdn0 = 0.0;
        var_taun_rdn1 = 0.0;
        var_taun_rdn2 = 0.0;
        var_taun_rdn3 = 0.0;
        var_taun_rdn4 = 0.0;
        var_taun_rdn5 = 0.0;
        var_taun_rdn6 = 0.0;
        var_taun_rdn7 = 0.0;
        var_taun_rdn8 = 0.0;
        var_taun_rdn9 = 0.0;
        var_taun_rdn10 = 0.0;
        var_taun_rdb0 = 0.0;
        var_taun_rdb1 = 0.0;

        let (assign6570_e6551, assign6570_e6551_d_n0, assign6570_e6551_d_n1, assign6570_e6551_d_n2, assign6570_e6551_d_n3, assign6570_e6551_d_n4, assign6570_e6551_d_n5, assign6570_e6551_d_n6, assign6570_e6551_d_n7, assign6570_e6551_d_n8, assign6570_e6551_d_n9, assign6570_e6551_d_n10, assign6570_e6551_d_b0, assign6570_e6551_d_b1,) = {
    if ((var_guard119 == 0.0) && (var_guard120 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_taun, var_taun_dn0, var_taun_dn1, var_taun_dn2, var_taun_dn3, var_taun_dn4, var_taun_dn5, var_taun_dn6, var_taun_dn7, var_taun_dn8, var_taun_dn9, var_taun_dn10, var_taun_db0, var_taun_db1,)
    }
};
        var_taun = assign6570_e6551;
        var_taun_dn0 = assign6570_e6551_d_n0;
        var_taun_dn1 = assign6570_e6551_d_n1;
        var_taun_dn2 = assign6570_e6551_d_n2;
        var_taun_dn3 = assign6570_e6551_d_n3;
        var_taun_dn4 = assign6570_e6551_d_n4;
        var_taun_dn5 = assign6570_e6551_d_n5;
        var_taun_dn6 = assign6570_e6551_d_n6;
        var_taun_dn7 = assign6570_e6551_d_n7;
        var_taun_dn8 = assign6570_e6551_d_n8;
        var_taun_dn9 = assign6570_e6551_d_n9;
        var_taun_dn10 = assign6570_e6551_d_n10;
        var_taun_db0 = assign6570_e6551_d_b0;
        var_taun_db1 = assign6570_e6551_d_b1;
        var_taun_rv = 0.0;
        var_taun_rdn0 = 0.0;
        var_taun_rdn1 = 0.0;
        var_taun_rdn2 = 0.0;
        var_taun_rdn3 = 0.0;
        var_taun_rdn4 = 0.0;
        var_taun_rdn5 = 0.0;
        var_taun_rdn6 = 0.0;
        var_taun_rdn7 = 0.0;
        var_taun_rdn8 = 0.0;
        var_taun_rdn9 = 0.0;
        var_taun_rdn10 = 0.0;
        var_taun_rdb0 = 0.0;
        var_taun_rdb1 = 0.0;

        *var_guard118_slot = var_guard118;
        *var_guard118_db0_slot = var_guard118_db0;
        *var_guard118_db1_slot = var_guard118_db1;
        *var_guard118_dn0_slot = var_guard118_dn0;
        *var_guard118_dn1_slot = var_guard118_dn1;
        *var_guard118_dn10_slot = var_guard118_dn10;
        *var_guard118_dn2_slot = var_guard118_dn2;
        *var_guard118_dn3_slot = var_guard118_dn3;
        *var_guard118_dn4_slot = var_guard118_dn4;
        *var_guard118_dn5_slot = var_guard118_dn5;
        *var_guard118_dn6_slot = var_guard118_dn6;
        *var_guard118_dn7_slot = var_guard118_dn7;
        *var_guard118_dn8_slot = var_guard118_dn8;
        *var_guard118_dn9_slot = var_guard118_dn9;
        *var_guard118_rdb0_slot = var_guard118_rdb0;
        *var_guard118_rdb1_slot = var_guard118_rdb1;
        *var_guard118_rdn0_slot = var_guard118_rdn0;
        *var_guard118_rdn1_slot = var_guard118_rdn1;
        *var_guard118_rdn10_slot = var_guard118_rdn10;
        *var_guard118_rdn2_slot = var_guard118_rdn2;
        *var_guard118_rdn3_slot = var_guard118_rdn3;
        *var_guard118_rdn4_slot = var_guard118_rdn4;
        *var_guard118_rdn5_slot = var_guard118_rdn5;
        *var_guard118_rdn6_slot = var_guard118_rdn6;
        *var_guard118_rdn7_slot = var_guard118_rdn7;
        *var_guard118_rdn8_slot = var_guard118_rdn8;
        *var_guard118_rdn9_slot = var_guard118_rdn9;
        *var_guard118_rv_slot = var_guard118_rv;
        *var_guard119_slot = var_guard119;
        *var_guard119_db0_slot = var_guard119_db0;
        *var_guard119_db1_slot = var_guard119_db1;
        *var_guard119_dn0_slot = var_guard119_dn0;
        *var_guard119_dn1_slot = var_guard119_dn1;
        *var_guard119_dn10_slot = var_guard119_dn10;
        *var_guard119_dn2_slot = var_guard119_dn2;
        *var_guard119_dn3_slot = var_guard119_dn3;
        *var_guard119_dn4_slot = var_guard119_dn4;
        *var_guard119_dn5_slot = var_guard119_dn5;
        *var_guard119_dn6_slot = var_guard119_dn6;
        *var_guard119_dn7_slot = var_guard119_dn7;
        *var_guard119_dn8_slot = var_guard119_dn8;
        *var_guard119_dn9_slot = var_guard119_dn9;
        *var_guard119_rdb0_slot = var_guard119_rdb0;
        *var_guard119_rdb1_slot = var_guard119_rdb1;
        *var_guard119_rdn0_slot = var_guard119_rdn0;
        *var_guard119_rdn1_slot = var_guard119_rdn1;
        *var_guard119_rdn10_slot = var_guard119_rdn10;
        *var_guard119_rdn2_slot = var_guard119_rdn2;
        *var_guard119_rdn3_slot = var_guard119_rdn3;
        *var_guard119_rdn4_slot = var_guard119_rdn4;
        *var_guard119_rdn5_slot = var_guard119_rdn5;
        *var_guard119_rdn6_slot = var_guard119_rdn6;
        *var_guard119_rdn7_slot = var_guard119_rdn7;
        *var_guard119_rdn8_slot = var_guard119_rdn8;
        *var_guard119_rdn9_slot = var_guard119_rdn9;
        *var_guard119_rv_slot = var_guard119_rv;
        *var_guard120_slot = var_guard120;
        *var_guard120_db0_slot = var_guard120_db0;
        *var_guard120_db1_slot = var_guard120_db1;
        *var_guard120_dn0_slot = var_guard120_dn0;
        *var_guard120_dn1_slot = var_guard120_dn1;
        *var_guard120_dn10_slot = var_guard120_dn10;
        *var_guard120_dn2_slot = var_guard120_dn2;
        *var_guard120_dn3_slot = var_guard120_dn3;
        *var_guard120_dn4_slot = var_guard120_dn4;
        *var_guard120_dn5_slot = var_guard120_dn5;
        *var_guard120_dn6_slot = var_guard120_dn6;
        *var_guard120_dn7_slot = var_guard120_dn7;
        *var_guard120_dn8_slot = var_guard120_dn8;
        *var_guard120_dn9_slot = var_guard120_dn9;
        *var_guard120_rdb0_slot = var_guard120_rdb0;
        *var_guard120_rdb1_slot = var_guard120_rdb1;
        *var_guard120_rdn0_slot = var_guard120_rdn0;
        *var_guard120_rdn1_slot = var_guard120_rdn1;
        *var_guard120_rdn10_slot = var_guard120_rdn10;
        *var_guard120_rdn2_slot = var_guard120_rdn2;
        *var_guard120_rdn3_slot = var_guard120_rdn3;
        *var_guard120_rdn4_slot = var_guard120_rdn4;
        *var_guard120_rdn5_slot = var_guard120_rdn5;
        *var_guard120_rdn6_slot = var_guard120_rdn6;
        *var_guard120_rdn7_slot = var_guard120_rdn7;
        *var_guard120_rdn8_slot = var_guard120_rdn8;
        *var_guard120_rdn9_slot = var_guard120_rdn9;
        *var_guard120_rv_slot = var_guard120_rv;
        *var_taub_n_slot = var_taub_n;
        *var_taub_n_db0_slot = var_taub_n_db0;
        *var_taub_n_db1_slot = var_taub_n_db1;
        *var_taub_n_dn0_slot = var_taub_n_dn0;
        *var_taub_n_dn1_slot = var_taub_n_dn1;
        *var_taub_n_dn10_slot = var_taub_n_dn10;
        *var_taub_n_dn2_slot = var_taub_n_dn2;
        *var_taub_n_dn3_slot = var_taub_n_dn3;
        *var_taub_n_dn4_slot = var_taub_n_dn4;
        *var_taub_n_dn5_slot = var_taub_n_dn5;
        *var_taub_n_dn6_slot = var_taub_n_dn6;
        *var_taub_n_dn7_slot = var_taub_n_dn7;
        *var_taub_n_dn8_slot = var_taub_n_dn8;
        *var_taub_n_dn9_slot = var_taub_n_dn9;
        *var_taub_n_rdb0_slot = var_taub_n_rdb0;
        *var_taub_n_rdb1_slot = var_taub_n_rdb1;
        *var_taub_n_rdn0_slot = var_taub_n_rdn0;
        *var_taub_n_rdn1_slot = var_taub_n_rdn1;
        *var_taub_n_rdn10_slot = var_taub_n_rdn10;
        *var_taub_n_rdn2_slot = var_taub_n_rdn2;
        *var_taub_n_rdn3_slot = var_taub_n_rdn3;
        *var_taub_n_rdn4_slot = var_taub_n_rdn4;
        *var_taub_n_rdn5_slot = var_taub_n_rdn5;
        *var_taub_n_rdn6_slot = var_taub_n_rdn6;
        *var_taub_n_rdn7_slot = var_taub_n_rdn7;
        *var_taub_n_rdn8_slot = var_taub_n_rdn8;
        *var_taub_n_rdn9_slot = var_taub_n_rdn9;
        *var_taub_n_rv_slot = var_taub_n_rv;
        *var_taun_slot = var_taun;
        *var_taun_db0_slot = var_taun_db0;
        *var_taun_db1_slot = var_taun_db1;
        *var_taun_dn0_slot = var_taun_dn0;
        *var_taun_dn1_slot = var_taun_dn1;
        *var_taun_dn10_slot = var_taun_dn10;
        *var_taun_dn2_slot = var_taun_dn2;
        *var_taun_dn3_slot = var_taun_dn3;
        *var_taun_dn4_slot = var_taun_dn4;
        *var_taun_dn5_slot = var_taun_dn5;
        *var_taun_dn6_slot = var_taun_dn6;
        *var_taun_dn7_slot = var_taun_dn7;
        *var_taun_dn8_slot = var_taun_dn8;
        *var_taun_dn9_slot = var_taun_dn9;
        *var_taun_rdb0_slot = var_taun_rdb0;
        *var_taun_rdb1_slot = var_taun_rdb1;
        *var_taun_rdn0_slot = var_taun_rdn0;
        *var_taun_rdn1_slot = var_taun_rdn1;
        *var_taun_rdn10_slot = var_taun_rdn10;
        *var_taun_rdn2_slot = var_taun_rdn2;
        *var_taun_rdn3_slot = var_taun_rdn3;
        *var_taun_rdn4_slot = var_taun_rdn4;
        *var_taun_rdn5_slot = var_taun_rdn5;
        *var_taun_rdn6_slot = var_taun_rdn6;
        *var_taun_rdn7_slot = var_taun_rdn7;
        *var_taun_rdn8_slot = var_taun_rdn8;
        *var_taun_rdn9_slot = var_taun_rdn9;
        *var_taun_rv_slot = var_taun_rv;
    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        var_qb1b2: f64,
        var_qb1b2_dn0: f64,
        var_qb1b2_dn1: f64,
        var_qb1b2_dn3: f64,
        var_qb1b2_dn4: f64,
        var_qb1b2_dn5: f64,
        var_qb1b2_dn6: f64,
        var_qb1b2_dn7: f64,
        var_qb1b2_dn8: f64,
        var_qb1b2_dn9: f64,
        var_qbc: f64,
        var_qbc_dn0: f64,
        var_qbc_dn1: f64,
        var_qbc_dn3: f64,
        var_qbc_dn4: f64,
        var_qbc_dn5: f64,
        var_qbc_dn6: f64,
        var_qbc_dn7: f64,
        var_qbc_dn8: f64,
        var_qbc_dn9: f64,
        var_qbe: f64,
        var_qbe_dn0: f64,
        var_qbe_dn1: f64,
        var_qbe_dn3: f64,
        var_qbe_dn4: f64,
        var_qbe_dn5: f64,
        var_qbe_dn6: f64,
        var_qbe_dn7: f64,
        var_qbe_dn8: f64,
        var_qbe_dn9: f64,
        var_qe: f64,
        var_qe_dn0: f64,
        var_qe_dn1: f64,
        var_qe_dn3: f64,
        var_qe_dn4: f64,
        var_qe_dn5: f64,
        var_qe_dn6: f64,
        var_qe_dn7: f64,
        var_qe_dn8: f64,
        var_qe_dn9: f64,
        var_qepi: f64,
        var_qepi_dn0: f64,
        var_qepi_dn1: f64,
        var_qepi_dn3: f64,
        var_qepi_dn4: f64,
        var_qepi_dn5: f64,
        var_qepi_dn6: f64,
        var_qepi_dn7: f64,
        var_qepi_dn8: f64,
        var_qepi_dn9: f64,
        var_qex: f64,
        var_qex_dn0: f64,
        var_qex_dn1: f64,
        var_qex_dn3: f64,
        var_qex_dn4: f64,
        var_qex_dn5: f64,
        var_qex_dn6: f64,
        var_qex_dn7: f64,
        var_qex_dn8: f64,
        var_qex_dn9: f64,
        var_qtc: f64,
        var_qtc_dn0: f64,
        var_qtc_dn1: f64,
        var_qtc_dn3: f64,
        var_qtc_dn4: f64,
        var_qtc_dn5: f64,
        var_qtc_dn6: f64,
        var_qtc_dn7: f64,
        var_qtc_dn8: f64,
        var_qtc_dn9: f64,
        var_qte: f64,
        var_qte_dn0: f64,
        var_qte_dn1: f64,
        var_qte_dn3: f64,
        var_qte_dn4: f64,
        var_qte_dn5: f64,
        var_qte_dn6: f64,
        var_qte_dn7: f64,
        var_qte_dn8: f64,
        var_qte_dn9: f64,
        var_qte_s: f64,
        var_qte_s_dn0: f64,
        var_qte_s_dn1: f64,
        var_qte_s_dn3: f64,
        var_qte_s_dn4: f64,
        var_qte_s_dn5: f64,
        var_qte_s_dn6: f64,
        var_qte_s_dn7: f64,
        var_qte_s_dn8: f64,
        var_qte_s_dn9: f64,
        var_qtex: f64,
        var_qtex_dn0: f64,
        var_qtex_dn1: f64,
        var_qtex_dn3: f64,
        var_qtex_dn4: f64,
        var_qtex_dn5: f64,
        var_qtex_dn6: f64,
        var_qtex_dn7: f64,
        var_qtex_dn8: f64,
        var_qtex_dn9: f64,
        var_taun: f64,
        var_taun_dn0: f64,
        var_taun_dn1: f64,
        var_taun_dn3: f64,
        var_taun_dn4: f64,
        var_taun_dn5: f64,
        var_taun_dn6: f64,
        var_taun_dn7: f64,
        var_taun_dn8: f64,
        var_taun_dn9: f64,
        var_vbc: f64,
        var_vbc_dn0: f64,
        var_vbc_dn1: f64,
        var_vbe: f64,
        var_vbe_dn1: f64,
        var_vbe_dn2: f64,
        var_xqex: f64,
        var_xqex_dn0: f64,
        var_xqex_dn1: f64,
        var_xqex_dn3: f64,
        var_xqex_dn4: f64,
        var_xqex_dn5: f64,
        var_xqex_dn6: f64,
        var_xqex_dn7: f64,
        var_xqex_dn8: f64,
        var_xqex_dn9: f64,
        var_xqtex: f64,
        var_xqtex_dn0: f64,
        var_xqtex_dn1: f64,
        var_xqtex_dn3: f64,
        var_xqtex_dn4: f64,
        var_xqtex_dn5: f64,
        var_xqtex_dn6: f64,
        var_xqtex_dn7: f64,
        var_xqtex_dn8: f64,
        var_xqtex_dn9: f64,
    ) {
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq10_e233: f64 = (var_qte + var_qbe);
        let eq10_e233_d_n0: f64 = (var_qte_dn0 + var_qbe_dn0);
        let eq10_e233_d_n1: f64 = (var_qte_dn1 + var_qbe_dn1);
        let eq10_e233_d_n3: f64 = (var_qte_dn3 + var_qbe_dn3);
        let eq10_e233_d_n4: f64 = (var_qte_dn4 + var_qbe_dn4);
        let eq10_e233_d_n5: f64 = (var_qte_dn5 + var_qbe_dn5);
        let eq10_e233_d_n6: f64 = (var_qte_dn6 + var_qbe_dn6);
        let eq10_e233_d_n7: f64 = (var_qte_dn7 + var_qbe_dn7);
        let eq10_e233_d_n8: f64 = (var_qte_dn8 + var_qbe_dn8);
        let eq10_e233_d_n9: f64 = (var_qte_dn9 + var_qbe_dn9);
        let eq10_e235: f64 = (eq10_e233 + var_qe);
        let eq10_e235_d_n0: f64 = (eq10_e233_d_n0 + var_qe_dn0);
        let eq10_e235_d_n1: f64 = (eq10_e233_d_n1 + var_qe_dn1);
        let eq10_e235_d_n3: f64 = (eq10_e233_d_n3 + var_qe_dn3);
        let eq10_e235_d_n4: f64 = (eq10_e233_d_n4 + var_qe_dn4);
        let eq10_e235_d_n5: f64 = (eq10_e233_d_n5 + var_qe_dn5);
        let eq10_e235_d_n6: f64 = (eq10_e233_d_n6 + var_qe_dn6);
        let eq10_e235_d_n7: f64 = (eq10_e233_d_n7 + var_qe_dn7);
        let eq10_e235_d_n8: f64 = (eq10_e233_d_n8 + var_qe_dn8);
        let eq10_e235_d_n9: f64 = (eq10_e233_d_n9 + var_qe_dn9);
        let eq10_e236: f64 = (p.p3 * eq10_e235);
        let eq10_e236_d_n0: f64 = (p.p3 * eq10_e235_d_n0);
        let eq10_e236_d_n1: f64 = (p.p3 * eq10_e235_d_n1);
        let eq10_e236_d_n3: f64 = (p.p3 * eq10_e235_d_n3);
        let eq10_e236_d_n4: f64 = (p.p3 * eq10_e235_d_n4);
        let eq10_e236_d_n5: f64 = (p.p3 * eq10_e235_d_n5);
        let eq10_e236_d_n6: f64 = (p.p3 * eq10_e235_d_n6);
        let eq10_e236_d_n7: f64 = (p.p3 * eq10_e235_d_n7);
        let eq10_e236_d_n8: f64 = (p.p3 * eq10_e235_d_n8);
        let eq10_e236_d_n9: f64 = (p.p3 * eq10_e235_d_n9);
        let eq10_e237: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq10_e236);
        let eq10_e239: f64 = (eq10_e237 * p.p1);
        let eq10_e239_d_n0: f64 = ((eq10_e236_d_n0 * ddt_scale) * p.p1);
        let eq10_e239_d_n1: f64 = ((eq10_e236_d_n1 * ddt_scale) * p.p1);
        let eq10_e239_d_n3: f64 = ((eq10_e236_d_n3 * ddt_scale) * p.p1);
        let eq10_e239_d_n4: f64 = ((eq10_e236_d_n4 * ddt_scale) * p.p1);
        let eq10_e239_d_n5: f64 = ((eq10_e236_d_n5 * ddt_scale) * p.p1);
        let eq10_e239_d_n6: f64 = ((eq10_e236_d_n6 * ddt_scale) * p.p1);
        let eq10_e239_d_n7: f64 = ((eq10_e236_d_n7 * ddt_scale) * p.p1);
        let eq10_e239_d_n8: f64 = ((eq10_e236_d_n8 * ddt_scale) * p.p1);
        let eq10_e239_d_n9: f64 = ((eq10_e236_d_n9 * ddt_scale) * p.p1);
        let eq10_value: f64 = eq10_e239;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(3),
            multiplicity * (eq10_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * (eq10_e239_d_n0), multiplicity * (eq10_e239_d_n1), multiplicity * (eq10_e239_d_n3), multiplicity * (eq10_e239_d_n4), multiplicity * (eq10_e239_d_n5), multiplicity * (eq10_e239_d_n6), multiplicity * (eq10_e239_d_n7), multiplicity * (eq10_e239_d_n8), multiplicity * (eq10_e239_d_n9)],
            [],
            [],
            1.0,
        );
        let eq11_e242: f64 = (p.p3 * var_qte_s);
        let eq11_e242_d_n0: f64 = (p.p3 * var_qte_s_dn0);
        let eq11_e242_d_n1: f64 = (p.p3 * var_qte_s_dn1);
        let eq11_e242_d_n3: f64 = (p.p3 * var_qte_s_dn3);
        let eq11_e242_d_n4: f64 = (p.p3 * var_qte_s_dn4);
        let eq11_e242_d_n5: f64 = (p.p3 * var_qte_s_dn5);
        let eq11_e242_d_n6: f64 = (p.p3 * var_qte_s_dn6);
        let eq11_e242_d_n7: f64 = (p.p3 * var_qte_s_dn7);
        let eq11_e242_d_n8: f64 = (p.p3 * var_qte_s_dn8);
        let eq11_e242_d_n9: f64 = (p.p3 * var_qte_s_dn9);
        let eq11_e243: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq11_e242);
        let eq11_e245: f64 = (eq11_e243 * p.p1);
        let eq11_e245_d_n0: f64 = ((eq11_e242_d_n0 * ddt_scale) * p.p1);
        let eq11_e245_d_n1: f64 = ((eq11_e242_d_n1 * ddt_scale) * p.p1);
        let eq11_e245_d_n3: f64 = ((eq11_e242_d_n3 * ddt_scale) * p.p1);
        let eq11_e245_d_n4: f64 = ((eq11_e242_d_n4 * ddt_scale) * p.p1);
        let eq11_e245_d_n5: f64 = ((eq11_e242_d_n5 * ddt_scale) * p.p1);
        let eq11_e245_d_n6: f64 = ((eq11_e242_d_n6 * ddt_scale) * p.p1);
        let eq11_e245_d_n7: f64 = ((eq11_e242_d_n7 * ddt_scale) * p.p1);
        let eq11_e245_d_n8: f64 = ((eq11_e242_d_n8 * ddt_scale) * p.p1);
        let eq11_e245_d_n9: f64 = ((eq11_e242_d_n9 * ddt_scale) * p.p1);
        let eq11_value: f64 = eq11_e245;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(4),
            Some(3),
            multiplicity * (eq11_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * (eq11_e245_d_n0), multiplicity * (eq11_e245_d_n1), multiplicity * (eq11_e245_d_n3), multiplicity * (eq11_e245_d_n4), multiplicity * (eq11_e245_d_n5), multiplicity * (eq11_e245_d_n6), multiplicity * (eq11_e245_d_n7), multiplicity * (eq11_e245_d_n8), multiplicity * (eq11_e245_d_n9)],
            [],
            [],
            1.0,
        );
        let eq12_e249: f64 = (var_qtc + var_qbc);
        let eq12_e249_d_n0: f64 = (var_qtc_dn0 + var_qbc_dn0);
        let eq12_e249_d_n1: f64 = (var_qtc_dn1 + var_qbc_dn1);
        let eq12_e249_d_n3: f64 = (var_qtc_dn3 + var_qbc_dn3);
        let eq12_e249_d_n4: f64 = (var_qtc_dn4 + var_qbc_dn4);
        let eq12_e249_d_n5: f64 = (var_qtc_dn5 + var_qbc_dn5);
        let eq12_e249_d_n6: f64 = (var_qtc_dn6 + var_qbc_dn6);
        let eq12_e249_d_n7: f64 = (var_qtc_dn7 + var_qbc_dn7);
        let eq12_e249_d_n8: f64 = (var_qtc_dn8 + var_qbc_dn8);
        let eq12_e249_d_n9: f64 = (var_qtc_dn9 + var_qbc_dn9);
        let eq12_e251: f64 = (eq12_e249 + var_qepi);
        let eq12_e251_d_n0: f64 = (eq12_e249_d_n0 + var_qepi_dn0);
        let eq12_e251_d_n1: f64 = (eq12_e249_d_n1 + var_qepi_dn1);
        let eq12_e251_d_n3: f64 = (eq12_e249_d_n3 + var_qepi_dn3);
        let eq12_e251_d_n4: f64 = (eq12_e249_d_n4 + var_qepi_dn4);
        let eq12_e251_d_n5: f64 = (eq12_e249_d_n5 + var_qepi_dn5);
        let eq12_e251_d_n6: f64 = (eq12_e249_d_n6 + var_qepi_dn6);
        let eq12_e251_d_n7: f64 = (eq12_e249_d_n7 + var_qepi_dn7);
        let eq12_e251_d_n8: f64 = (eq12_e249_d_n8 + var_qepi_dn8);
        let eq12_e251_d_n9: f64 = (eq12_e249_d_n9 + var_qepi_dn9);
        let eq12_e252: f64 = (p.p3 * eq12_e251);
        let eq12_e252_d_n0: f64 = (p.p3 * eq12_e251_d_n0);
        let eq12_e252_d_n1: f64 = (p.p3 * eq12_e251_d_n1);
        let eq12_e252_d_n3: f64 = (p.p3 * eq12_e251_d_n3);
        let eq12_e252_d_n4: f64 = (p.p3 * eq12_e251_d_n4);
        let eq12_e252_d_n5: f64 = (p.p3 * eq12_e251_d_n5);
        let eq12_e252_d_n6: f64 = (p.p3 * eq12_e251_d_n6);
        let eq12_e252_d_n7: f64 = (p.p3 * eq12_e251_d_n7);
        let eq12_e252_d_n8: f64 = (p.p3 * eq12_e251_d_n8);
        let eq12_e252_d_n9: f64 = (p.p3 * eq12_e251_d_n9);
        let eq12_e253: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq12_e252);
        let eq12_e255: f64 = (eq12_e253 * p.p1);
        let eq12_e255_d_n0: f64 = ((eq12_e252_d_n0 * ddt_scale) * p.p1);
        let eq12_e255_d_n1: f64 = ((eq12_e252_d_n1 * ddt_scale) * p.p1);
        let eq12_e255_d_n3: f64 = ((eq12_e252_d_n3 * ddt_scale) * p.p1);
        let eq12_e255_d_n4: f64 = ((eq12_e252_d_n4 * ddt_scale) * p.p1);
        let eq12_e255_d_n5: f64 = ((eq12_e252_d_n5 * ddt_scale) * p.p1);
        let eq12_e255_d_n6: f64 = ((eq12_e252_d_n6 * ddt_scale) * p.p1);
        let eq12_e255_d_n7: f64 = ((eq12_e252_d_n7 * ddt_scale) * p.p1);
        let eq12_e255_d_n8: f64 = ((eq12_e252_d_n8 * ddt_scale) * p.p1);
        let eq12_e255_d_n9: f64 = ((eq12_e252_d_n9 * ddt_scale) * p.p1);
        let eq12_value: f64 = eq12_e255;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(7),
            multiplicity * (eq12_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * (eq12_e255_d_n0), multiplicity * (eq12_e255_d_n1), multiplicity * (eq12_e255_d_n3), multiplicity * (eq12_e255_d_n4), multiplicity * (eq12_e255_d_n5), multiplicity * (eq12_e255_d_n6), multiplicity * (eq12_e255_d_n7), multiplicity * (eq12_e255_d_n8), multiplicity * (eq12_e255_d_n9)],
            [],
            [],
            1.0,
        );
        let eq13_e258: f64 = (p.p3 * var_qb1b2);
        let eq13_e258_d_n0: f64 = (p.p3 * var_qb1b2_dn0);
        let eq13_e258_d_n1: f64 = (p.p3 * var_qb1b2_dn1);
        let eq13_e258_d_n3: f64 = (p.p3 * var_qb1b2_dn3);
        let eq13_e258_d_n4: f64 = (p.p3 * var_qb1b2_dn4);
        let eq13_e258_d_n5: f64 = (p.p3 * var_qb1b2_dn5);
        let eq13_e258_d_n6: f64 = (p.p3 * var_qb1b2_dn6);
        let eq13_e258_d_n7: f64 = (p.p3 * var_qb1b2_dn7);
        let eq13_e258_d_n8: f64 = (p.p3 * var_qb1b2_dn8);
        let eq13_e258_d_n9: f64 = (p.p3 * var_qb1b2_dn9);
        let eq13_e259: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq13_e258);
        let eq13_e261: f64 = (eq13_e259 * p.p1);
        let eq13_e261_d_n0: f64 = ((eq13_e258_d_n0 * ddt_scale) * p.p1);
        let eq13_e261_d_n1: f64 = ((eq13_e258_d_n1 * ddt_scale) * p.p1);
        let eq13_e261_d_n3: f64 = ((eq13_e258_d_n3 * ddt_scale) * p.p1);
        let eq13_e261_d_n4: f64 = ((eq13_e258_d_n4 * ddt_scale) * p.p1);
        let eq13_e261_d_n5: f64 = ((eq13_e258_d_n5 * ddt_scale) * p.p1);
        let eq13_e261_d_n6: f64 = ((eq13_e258_d_n6 * ddt_scale) * p.p1);
        let eq13_e261_d_n7: f64 = ((eq13_e258_d_n7 * ddt_scale) * p.p1);
        let eq13_e261_d_n8: f64 = ((eq13_e258_d_n8 * ddt_scale) * p.p1);
        let eq13_e261_d_n9: f64 = ((eq13_e258_d_n9 * ddt_scale) * p.p1);
        let eq13_value: f64 = eq13_e261;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(4),
            Some(5),
            multiplicity * (eq13_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * (eq13_e261_d_n0), multiplicity * (eq13_e261_d_n1), multiplicity * (eq13_e261_d_n3), multiplicity * (eq13_e261_d_n4), multiplicity * (eq13_e261_d_n5), multiplicity * (eq13_e261_d_n6), multiplicity * (eq13_e261_d_n7), multiplicity * (eq13_e261_d_n8), multiplicity * (eq13_e261_d_n9)],
            [],
            [],
            1.0,
        );
        let eq14_e264: f64 = (p.p3 * p.p68);
        let eq14_e266: f64 = (eq14_e264 * var_vbe);
        let eq14_e266_d_n1: f64 = (eq14_e264 * var_vbe_dn1);
        let eq14_e266_d_n2: f64 = (eq14_e264 * var_vbe_dn2);
        let eq14_e267: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq14_e266);
        let eq14_e269: f64 = (eq14_e267 * p.p1);
        let eq14_e269_d_n1: f64 = ((eq14_e266_d_n1 * ddt_scale) * p.p1);
        let eq14_e269_d_n2: f64 = ((eq14_e266_d_n2 * ddt_scale) * p.p1);
        let eq14_value: f64 = eq14_e269;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (eq14_value),
            1,
            multiplicity * (eq14_e269_d_n1),
            2,
            multiplicity * (eq14_e269_d_n2),
        );
        let eq15_e272: f64 = (p.p3 * p.p77);
        let eq15_e274: f64 = (eq15_e272 * var_vbc);
        let eq15_e274_d_n0: f64 = (eq15_e272 * var_vbc_dn0);
        let eq15_e274_d_n1: f64 = (eq15_e272 * var_vbc_dn1);
        let eq15_e275: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq15_e274);
        let eq15_e277: f64 = (eq15_e275 * p.p1);
        let eq15_e277_d_n0: f64 = ((eq15_e274_d_n0 * ddt_scale) * p.p1);
        let eq15_e277_d_n1: f64 = ((eq15_e274_d_n1 * ddt_scale) * p.p1);
        let eq15_value: f64 = eq15_e277;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * (eq15_value),
            0,
            multiplicity * (eq15_e277_d_n0),
            1,
            multiplicity * (eq15_e277_d_n1),
        );
        let eq18_e293: f64 = (var_xqtex + var_xqex);
        let eq18_e293_d_n0: f64 = (var_xqtex_dn0 + var_xqex_dn0);
        let eq18_e293_d_n1: f64 = (var_xqtex_dn1 + var_xqex_dn1);
        let eq18_e293_d_n3: f64 = (var_xqtex_dn3 + var_xqex_dn3);
        let eq18_e293_d_n4: f64 = (var_xqtex_dn4 + var_xqex_dn4);
        let eq18_e293_d_n5: f64 = (var_xqtex_dn5 + var_xqex_dn5);
        let eq18_e293_d_n6: f64 = (var_xqtex_dn6 + var_xqex_dn6);
        let eq18_e293_d_n7: f64 = (var_xqtex_dn7 + var_xqex_dn7);
        let eq18_e293_d_n8: f64 = (var_xqtex_dn8 + var_xqex_dn8);
        let eq18_e293_d_n9: f64 = (var_xqtex_dn9 + var_xqex_dn9);
        let eq18_e294: f64 = (p.p3 * eq18_e293);
        let eq18_e294_d_n0: f64 = (p.p3 * eq18_e293_d_n0);
        let eq18_e294_d_n1: f64 = (p.p3 * eq18_e293_d_n1);
        let eq18_e294_d_n3: f64 = (p.p3 * eq18_e293_d_n3);
        let eq18_e294_d_n4: f64 = (p.p3 * eq18_e293_d_n4);
        let eq18_e294_d_n5: f64 = (p.p3 * eq18_e293_d_n5);
        let eq18_e294_d_n6: f64 = (p.p3 * eq18_e293_d_n6);
        let eq18_e294_d_n7: f64 = (p.p3 * eq18_e293_d_n7);
        let eq18_e294_d_n8: f64 = (p.p3 * eq18_e293_d_n8);
        let eq18_e294_d_n9: f64 = (p.p3 * eq18_e293_d_n9);
        let eq18_e295: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq18_e294);
        let eq18_e297: f64 = (eq18_e295 * p.p1);
        let eq18_e297_d_n0: f64 = ((eq18_e294_d_n0 * ddt_scale) * p.p1);
        let eq18_e297_d_n1: f64 = ((eq18_e294_d_n1 * ddt_scale) * p.p1);
        let eq18_e297_d_n3: f64 = ((eq18_e294_d_n3 * ddt_scale) * p.p1);
        let eq18_e297_d_n4: f64 = ((eq18_e294_d_n4 * ddt_scale) * p.p1);
        let eq18_e297_d_n5: f64 = ((eq18_e294_d_n5 * ddt_scale) * p.p1);
        let eq18_e297_d_n6: f64 = ((eq18_e294_d_n6 * ddt_scale) * p.p1);
        let eq18_e297_d_n7: f64 = ((eq18_e294_d_n7 * ddt_scale) * p.p1);
        let eq18_e297_d_n8: f64 = ((eq18_e294_d_n8 * ddt_scale) * p.p1);
        let eq18_e297_d_n9: f64 = ((eq18_e294_d_n9 * ddt_scale) * p.p1);
        let eq18_value: f64 = eq18_e297;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(1),
            Some(8),
            multiplicity * (eq18_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * (eq18_e297_d_n0), multiplicity * (eq18_e297_d_n1), multiplicity * (eq18_e297_d_n3), multiplicity * (eq18_e297_d_n4), multiplicity * (eq18_e297_d_n5), multiplicity * (eq18_e297_d_n6), multiplicity * (eq18_e297_d_n7), multiplicity * (eq18_e297_d_n8), multiplicity * (eq18_e297_d_n9)],
            [],
            [],
            1.0,
        );
        let eq20_e312: f64 = (var_qtex + var_qex);
        let eq20_e312_d_n0: f64 = (var_qtex_dn0 + var_qex_dn0);
        let eq20_e312_d_n1: f64 = (var_qtex_dn1 + var_qex_dn1);
        let eq20_e312_d_n3: f64 = (var_qtex_dn3 + var_qex_dn3);
        let eq20_e312_d_n4: f64 = (var_qtex_dn4 + var_qex_dn4);
        let eq20_e312_d_n5: f64 = (var_qtex_dn5 + var_qex_dn5);
        let eq20_e312_d_n6: f64 = (var_qtex_dn6 + var_qex_dn6);
        let eq20_e312_d_n7: f64 = (var_qtex_dn7 + var_qex_dn7);
        let eq20_e312_d_n8: f64 = (var_qtex_dn8 + var_qex_dn8);
        let eq20_e312_d_n9: f64 = (var_qtex_dn9 + var_qex_dn9);
        let eq20_e313: f64 = (p.p3 * eq20_e312);
        let eq20_e313_d_n0: f64 = (p.p3 * eq20_e312_d_n0);
        let eq20_e313_d_n1: f64 = (p.p3 * eq20_e312_d_n1);
        let eq20_e313_d_n3: f64 = (p.p3 * eq20_e312_d_n3);
        let eq20_e313_d_n4: f64 = (p.p3 * eq20_e312_d_n4);
        let eq20_e313_d_n5: f64 = (p.p3 * eq20_e312_d_n5);
        let eq20_e313_d_n6: f64 = (p.p3 * eq20_e312_d_n6);
        let eq20_e313_d_n7: f64 = (p.p3 * eq20_e312_d_n7);
        let eq20_e313_d_n8: f64 = (p.p3 * eq20_e312_d_n8);
        let eq20_e313_d_n9: f64 = (p.p3 * eq20_e312_d_n9);
        let eq20_e314: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq20_e313);
        let eq20_e316: f64 = (eq20_e314 * p.p1);
        let eq20_e316_d_n0: f64 = ((eq20_e313_d_n0 * ddt_scale) * p.p1);
        let eq20_e316_d_n1: f64 = ((eq20_e313_d_n1 * ddt_scale) * p.p1);
        let eq20_e316_d_n3: f64 = ((eq20_e313_d_n3 * ddt_scale) * p.p1);
        let eq20_e316_d_n4: f64 = ((eq20_e313_d_n4 * ddt_scale) * p.p1);
        let eq20_e316_d_n5: f64 = ((eq20_e313_d_n5 * ddt_scale) * p.p1);
        let eq20_e316_d_n6: f64 = ((eq20_e313_d_n6 * ddt_scale) * p.p1);
        let eq20_e316_d_n7: f64 = ((eq20_e313_d_n7 * ddt_scale) * p.p1);
        let eq20_e316_d_n8: f64 = ((eq20_e313_d_n8 * ddt_scale) * p.p1);
        let eq20_e316_d_n9: f64 = ((eq20_e313_d_n9 * ddt_scale) * p.p1);
        let eq20_value: f64 = eq20_e316;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(4),
            Some(9),
            multiplicity * (eq20_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [multiplicity * (eq20_e316_d_n0), multiplicity * (eq20_e316_d_n1), multiplicity * (eq20_e316_d_n3), multiplicity * (eq20_e316_d_n4), multiplicity * (eq20_e316_d_n5), multiplicity * (eq20_e316_d_n6), multiplicity * (eq20_e316_d_n7), multiplicity * (eq20_e316_d_n8), multiplicity * (eq20_e316_d_n9)],
            [],
            [],
            1.0,
        );
        let eq27_e355: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, (nv10 - 0.0));
        let eq27_e356: f64 = (var_taun * eq27_e355);
        let eq27_e356_d_n0: f64 = (var_taun_dn0 * eq27_e355);
        let eq27_e356_d_n1: f64 = (var_taun_dn1 * eq27_e355);
        let eq27_e356_d_n3: f64 = (var_taun_dn3 * eq27_e355);
        let eq27_e356_d_n4: f64 = (var_taun_dn4 * eq27_e355);
        let eq27_e356_d_n5: f64 = (var_taun_dn5 * eq27_e355);
        let eq27_e356_d_n6: f64 = (var_taun_dn6 * eq27_e355);
        let eq27_e356_d_n7: f64 = (var_taun_dn7 * eq27_e355);
        let eq27_e356_d_n8: f64 = (var_taun_dn8 * eq27_e355);
        let eq27_e356_d_n9: f64 = (var_taun_dn9 * eq27_e355);
        let eq27_value: f64 = eq27_e356;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(3),
            multiplicity * (eq27_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq27_e356_d_n0), multiplicity * (eq27_e356_d_n1), multiplicity * (eq27_e356_d_n3), multiplicity * (eq27_e356_d_n4), multiplicity * (eq27_e356_d_n5), multiplicity * (eq27_e356_d_n6), multiplicity * (eq27_e356_d_n7), multiplicity * (eq27_e356_d_n8), multiplicity * (eq27_e356_d_n9), multiplicity * ((var_taun * ddt_scale))],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_qb1b2: f64,
        var_qb1b2_db0: f64,
        var_qb1b2_db1: f64,
        var_qb1b2_dn0: f64,
        var_qb1b2_dn1: f64,
        var_qb1b2_dn10: f64,
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
        var_qtex_dn2: f64,
        var_qtex_dn3: f64,
        var_qtex_dn4: f64,
        var_qtex_dn5: f64,
        var_qtex_dn6: f64,
        var_qtex_dn7: f64,
        var_qtex_dn8: f64,
        var_qtex_dn9: f64,
        var_taun: f64,
        var_taun_db0: f64,
        var_taun_db1: f64,
        var_taun_dn0: f64,
        var_taun_dn1: f64,
        var_taun_dn10: f64,
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
        var_xqtex_dn2: f64,
        var_xqtex_dn3: f64,
        var_xqtex_dn4: f64,
        var_xqtex_dn5: f64,
        var_xqtex_dn6: f64,
        var_xqtex_dn7: f64,
        var_xqtex_dn8: f64,
        var_xqtex_dn9: f64,
    ) {
        let nv10 = ctx.node_voltage(nodes[10]);
        let eq10_e233: f64 = (var_qte + var_qbe);
        let eq10_e233_d_n0: f64 = (var_qte_dn0 + var_qbe_dn0);
        let eq10_e233_d_n1: f64 = (var_qte_dn1 + var_qbe_dn1);
        let eq10_e233_d_n2: f64 = (var_qte_dn2 + var_qbe_dn2);
        let eq10_e233_d_n3: f64 = (var_qte_dn3 + var_qbe_dn3);
        let eq10_e233_d_n4: f64 = (var_qte_dn4 + var_qbe_dn4);
        let eq10_e233_d_n5: f64 = (var_qte_dn5 + var_qbe_dn5);
        let eq10_e233_d_n6: f64 = (var_qte_dn6 + var_qbe_dn6);
        let eq10_e233_d_n7: f64 = (var_qte_dn7 + var_qbe_dn7);
        let eq10_e233_d_n8: f64 = (var_qte_dn8 + var_qbe_dn8);
        let eq10_e233_d_n9: f64 = (var_qte_dn9 + var_qbe_dn9);
        let eq10_e233_d_n10: f64 = (var_qte_dn10 + var_qbe_dn10);
        let eq10_e233_d_b0: f64 = (var_qte_db0 + var_qbe_db0);
        let eq10_e233_d_b1: f64 = (var_qte_db1 + var_qbe_db1);
        let eq10_e235: f64 = (eq10_e233 + var_qe);
        let eq10_e235_d_n0: f64 = (eq10_e233_d_n0 + var_qe_dn0);
        let eq10_e235_d_n1: f64 = (eq10_e233_d_n1 + var_qe_dn1);
        let eq10_e235_d_n2: f64 = (eq10_e233_d_n2 + var_qe_dn2);
        let eq10_e235_d_n3: f64 = (eq10_e233_d_n3 + var_qe_dn3);
        let eq10_e235_d_n4: f64 = (eq10_e233_d_n4 + var_qe_dn4);
        let eq10_e235_d_n5: f64 = (eq10_e233_d_n5 + var_qe_dn5);
        let eq10_e235_d_n6: f64 = (eq10_e233_d_n6 + var_qe_dn6);
        let eq10_e235_d_n7: f64 = (eq10_e233_d_n7 + var_qe_dn7);
        let eq10_e235_d_n8: f64 = (eq10_e233_d_n8 + var_qe_dn8);
        let eq10_e235_d_n9: f64 = (eq10_e233_d_n9 + var_qe_dn9);
        let eq10_e235_d_n10: f64 = (eq10_e233_d_n10 + var_qe_dn10);
        let eq10_e235_d_b0: f64 = (eq10_e233_d_b0 + var_qe_db0);
        let eq10_e235_d_b1: f64 = (eq10_e233_d_b1 + var_qe_db1);
        let eq10_e236: f64 = (p.p3 * eq10_e235);
        let eq10_e236_d_n0: f64 = (p.p3 * eq10_e235_d_n0);
        let eq10_e236_d_n1: f64 = (p.p3 * eq10_e235_d_n1);
        let eq10_e236_d_n2: f64 = (p.p3 * eq10_e235_d_n2);
        let eq10_e236_d_n3: f64 = (p.p3 * eq10_e235_d_n3);
        let eq10_e236_d_n4: f64 = (p.p3 * eq10_e235_d_n4);
        let eq10_e236_d_n5: f64 = (p.p3 * eq10_e235_d_n5);
        let eq10_e236_d_n6: f64 = (p.p3 * eq10_e235_d_n6);
        let eq10_e236_d_n7: f64 = (p.p3 * eq10_e235_d_n7);
        let eq10_e236_d_n8: f64 = (p.p3 * eq10_e235_d_n8);
        let eq10_e236_d_n9: f64 = (p.p3 * eq10_e235_d_n9);
        let eq10_e236_d_n10: f64 = (p.p3 * eq10_e235_d_n10);
        let eq10_e236_d_b0: f64 = (p.p3 * eq10_e235_d_b0);
        let eq10_e236_d_b1: f64 = (p.p3 * eq10_e235_d_b1);
        let eq10_e237_q: f64 = eq10_e236;
        let eq10_e239: f64 = (eq10_e236 * p.p1);
        let eq10_e239_d_n0: f64 = (eq10_e236_d_n0 * p.p1);
        let eq10_e239_d_n1: f64 = (eq10_e236_d_n1 * p.p1);
        let eq10_e239_d_n2: f64 = (eq10_e236_d_n2 * p.p1);
        let eq10_e239_d_n3: f64 = (eq10_e236_d_n3 * p.p1);
        let eq10_e239_d_n4: f64 = (eq10_e236_d_n4 * p.p1);
        let eq10_e239_d_n5: f64 = (eq10_e236_d_n5 * p.p1);
        let eq10_e239_d_n6: f64 = (eq10_e236_d_n6 * p.p1);
        let eq10_e239_d_n7: f64 = (eq10_e236_d_n7 * p.p1);
        let eq10_e239_d_n8: f64 = (eq10_e236_d_n8 * p.p1);
        let eq10_e239_d_n9: f64 = (eq10_e236_d_n9 * p.p1);
        let eq10_e239_d_n10: f64 = (eq10_e236_d_n10 * p.p1);
        let eq10_e239_d_b0: f64 = (eq10_e236_d_b0 * p.p1);
        let eq10_e239_d_b1: f64 = (eq10_e236_d_b1 * p.p1);
        let eq10_e239_q: f64 = (eq10_e237_q * p.p1);
        let eq10_reactive_node_derivatives: [f64; 11] = [eq10_e239_d_n0, eq10_e239_d_n1, eq10_e239_d_n2, eq10_e239_d_n3, eq10_e239_d_n4, eq10_e239_d_n5, eq10_e239_d_n6, eq10_e239_d_n7, eq10_e239_d_n8, eq10_e239_d_n9, eq10_e239_d_n10];
        let eq10_reactive_branch_derivatives: [f64; 2] = [eq10_e239_d_b0, eq10_e239_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[3]),
            nodes,
            &eq10_reactive_node_derivatives,
            branches,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );
        let eq11_e242: f64 = (p.p3 * var_qte_s);
        let eq11_e242_d_n0: f64 = (p.p3 * var_qte_s_dn0);
        let eq11_e242_d_n1: f64 = (p.p3 * var_qte_s_dn1);
        let eq11_e242_d_n2: f64 = (p.p3 * var_qte_s_dn2);
        let eq11_e242_d_n3: f64 = (p.p3 * var_qte_s_dn3);
        let eq11_e242_d_n4: f64 = (p.p3 * var_qte_s_dn4);
        let eq11_e242_d_n5: f64 = (p.p3 * var_qte_s_dn5);
        let eq11_e242_d_n6: f64 = (p.p3 * var_qte_s_dn6);
        let eq11_e242_d_n7: f64 = (p.p3 * var_qte_s_dn7);
        let eq11_e242_d_n8: f64 = (p.p3 * var_qte_s_dn8);
        let eq11_e242_d_n9: f64 = (p.p3 * var_qte_s_dn9);
        let eq11_e242_d_n10: f64 = (p.p3 * var_qte_s_dn10);
        let eq11_e242_d_b0: f64 = (p.p3 * var_qte_s_db0);
        let eq11_e242_d_b1: f64 = (p.p3 * var_qte_s_db1);
        let eq11_e243_q: f64 = eq11_e242;
        let eq11_e245: f64 = (eq11_e242 * p.p1);
        let eq11_e245_d_n0: f64 = (eq11_e242_d_n0 * p.p1);
        let eq11_e245_d_n1: f64 = (eq11_e242_d_n1 * p.p1);
        let eq11_e245_d_n2: f64 = (eq11_e242_d_n2 * p.p1);
        let eq11_e245_d_n3: f64 = (eq11_e242_d_n3 * p.p1);
        let eq11_e245_d_n4: f64 = (eq11_e242_d_n4 * p.p1);
        let eq11_e245_d_n5: f64 = (eq11_e242_d_n5 * p.p1);
        let eq11_e245_d_n6: f64 = (eq11_e242_d_n6 * p.p1);
        let eq11_e245_d_n7: f64 = (eq11_e242_d_n7 * p.p1);
        let eq11_e245_d_n8: f64 = (eq11_e242_d_n8 * p.p1);
        let eq11_e245_d_n9: f64 = (eq11_e242_d_n9 * p.p1);
        let eq11_e245_d_n10: f64 = (eq11_e242_d_n10 * p.p1);
        let eq11_e245_d_b0: f64 = (eq11_e242_d_b0 * p.p1);
        let eq11_e245_d_b1: f64 = (eq11_e242_d_b1 * p.p1);
        let eq11_e245_q: f64 = (eq11_e243_q * p.p1);
        let eq11_reactive_node_derivatives: [f64; 11] = [eq11_e245_d_n0, eq11_e245_d_n1, eq11_e245_d_n2, eq11_e245_d_n3, eq11_e245_d_n4, eq11_e245_d_n5, eq11_e245_d_n6, eq11_e245_d_n7, eq11_e245_d_n8, eq11_e245_d_n9, eq11_e245_d_n10];
        let eq11_reactive_branch_derivatives: [f64; 2] = [eq11_e245_d_b0, eq11_e245_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[3]),
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
        let eq12_e249: f64 = (var_qtc + var_qbc);
        let eq12_e249_d_n0: f64 = (var_qtc_dn0 + var_qbc_dn0);
        let eq12_e249_d_n1: f64 = (var_qtc_dn1 + var_qbc_dn1);
        let eq12_e249_d_n2: f64 = (var_qtc_dn2 + var_qbc_dn2);
        let eq12_e249_d_n3: f64 = (var_qtc_dn3 + var_qbc_dn3);
        let eq12_e249_d_n4: f64 = (var_qtc_dn4 + var_qbc_dn4);
        let eq12_e249_d_n5: f64 = (var_qtc_dn5 + var_qbc_dn5);
        let eq12_e249_d_n6: f64 = (var_qtc_dn6 + var_qbc_dn6);
        let eq12_e249_d_n7: f64 = (var_qtc_dn7 + var_qbc_dn7);
        let eq12_e249_d_n8: f64 = (var_qtc_dn8 + var_qbc_dn8);
        let eq12_e249_d_n9: f64 = (var_qtc_dn9 + var_qbc_dn9);
        let eq12_e249_d_n10: f64 = (var_qtc_dn10 + var_qbc_dn10);
        let eq12_e249_d_b0: f64 = (var_qtc_db0 + var_qbc_db0);
        let eq12_e249_d_b1: f64 = (var_qtc_db1 + var_qbc_db1);
        let eq12_e251: f64 = (eq12_e249 + var_qepi);
        let eq12_e251_d_n0: f64 = (eq12_e249_d_n0 + var_qepi_dn0);
        let eq12_e251_d_n1: f64 = (eq12_e249_d_n1 + var_qepi_dn1);
        let eq12_e251_d_n2: f64 = (eq12_e249_d_n2 + var_qepi_dn2);
        let eq12_e251_d_n3: f64 = (eq12_e249_d_n3 + var_qepi_dn3);
        let eq12_e251_d_n4: f64 = (eq12_e249_d_n4 + var_qepi_dn4);
        let eq12_e251_d_n5: f64 = (eq12_e249_d_n5 + var_qepi_dn5);
        let eq12_e251_d_n6: f64 = (eq12_e249_d_n6 + var_qepi_dn6);
        let eq12_e251_d_n7: f64 = (eq12_e249_d_n7 + var_qepi_dn7);
        let eq12_e251_d_n8: f64 = (eq12_e249_d_n8 + var_qepi_dn8);
        let eq12_e251_d_n9: f64 = (eq12_e249_d_n9 + var_qepi_dn9);
        let eq12_e251_d_n10: f64 = (eq12_e249_d_n10 + var_qepi_dn10);
        let eq12_e251_d_b0: f64 = (eq12_e249_d_b0 + var_qepi_db0);
        let eq12_e251_d_b1: f64 = (eq12_e249_d_b1 + var_qepi_db1);
        let eq12_e252: f64 = (p.p3 * eq12_e251);
        let eq12_e252_d_n0: f64 = (p.p3 * eq12_e251_d_n0);
        let eq12_e252_d_n1: f64 = (p.p3 * eq12_e251_d_n1);
        let eq12_e252_d_n2: f64 = (p.p3 * eq12_e251_d_n2);
        let eq12_e252_d_n3: f64 = (p.p3 * eq12_e251_d_n3);
        let eq12_e252_d_n4: f64 = (p.p3 * eq12_e251_d_n4);
        let eq12_e252_d_n5: f64 = (p.p3 * eq12_e251_d_n5);
        let eq12_e252_d_n6: f64 = (p.p3 * eq12_e251_d_n6);
        let eq12_e252_d_n7: f64 = (p.p3 * eq12_e251_d_n7);
        let eq12_e252_d_n8: f64 = (p.p3 * eq12_e251_d_n8);
        let eq12_e252_d_n9: f64 = (p.p3 * eq12_e251_d_n9);
        let eq12_e252_d_n10: f64 = (p.p3 * eq12_e251_d_n10);
        let eq12_e252_d_b0: f64 = (p.p3 * eq12_e251_d_b0);
        let eq12_e252_d_b1: f64 = (p.p3 * eq12_e251_d_b1);
        let eq12_e253_q: f64 = eq12_e252;
        let eq12_e255: f64 = (eq12_e252 * p.p1);
        let eq12_e255_d_n0: f64 = (eq12_e252_d_n0 * p.p1);
        let eq12_e255_d_n1: f64 = (eq12_e252_d_n1 * p.p1);
        let eq12_e255_d_n2: f64 = (eq12_e252_d_n2 * p.p1);
        let eq12_e255_d_n3: f64 = (eq12_e252_d_n3 * p.p1);
        let eq12_e255_d_n4: f64 = (eq12_e252_d_n4 * p.p1);
        let eq12_e255_d_n5: f64 = (eq12_e252_d_n5 * p.p1);
        let eq12_e255_d_n6: f64 = (eq12_e252_d_n6 * p.p1);
        let eq12_e255_d_n7: f64 = (eq12_e252_d_n7 * p.p1);
        let eq12_e255_d_n8: f64 = (eq12_e252_d_n8 * p.p1);
        let eq12_e255_d_n9: f64 = (eq12_e252_d_n9 * p.p1);
        let eq12_e255_d_n10: f64 = (eq12_e252_d_n10 * p.p1);
        let eq12_e255_d_b0: f64 = (eq12_e252_d_b0 * p.p1);
        let eq12_e255_d_b1: f64 = (eq12_e252_d_b1 * p.p1);
        let eq12_e255_q: f64 = (eq12_e253_q * p.p1);
        let eq12_reactive_node_derivatives: [f64; 11] = [eq12_e255_d_n0, eq12_e255_d_n1, eq12_e255_d_n2, eq12_e255_d_n3, eq12_e255_d_n4, eq12_e255_d_n5, eq12_e255_d_n6, eq12_e255_d_n7, eq12_e255_d_n8, eq12_e255_d_n9, eq12_e255_d_n10];
        let eq12_reactive_branch_derivatives: [f64; 2] = [eq12_e255_d_b0, eq12_e255_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );
        let eq13_e258: f64 = (p.p3 * var_qb1b2);
        let eq13_e258_d_n0: f64 = (p.p3 * var_qb1b2_dn0);
        let eq13_e258_d_n1: f64 = (p.p3 * var_qb1b2_dn1);
        let eq13_e258_d_n2: f64 = (p.p3 * var_qb1b2_dn2);
        let eq13_e258_d_n3: f64 = (p.p3 * var_qb1b2_dn3);
        let eq13_e258_d_n4: f64 = (p.p3 * var_qb1b2_dn4);
        let eq13_e258_d_n5: f64 = (p.p3 * var_qb1b2_dn5);
        let eq13_e258_d_n6: f64 = (p.p3 * var_qb1b2_dn6);
        let eq13_e258_d_n7: f64 = (p.p3 * var_qb1b2_dn7);
        let eq13_e258_d_n8: f64 = (p.p3 * var_qb1b2_dn8);
        let eq13_e258_d_n9: f64 = (p.p3 * var_qb1b2_dn9);
        let eq13_e258_d_n10: f64 = (p.p3 * var_qb1b2_dn10);
        let eq13_e258_d_b0: f64 = (p.p3 * var_qb1b2_db0);
        let eq13_e258_d_b1: f64 = (p.p3 * var_qb1b2_db1);
        let eq13_e259_q: f64 = eq13_e258;
        let eq13_e261: f64 = (eq13_e258 * p.p1);
        let eq13_e261_d_n0: f64 = (eq13_e258_d_n0 * p.p1);
        let eq13_e261_d_n1: f64 = (eq13_e258_d_n1 * p.p1);
        let eq13_e261_d_n2: f64 = (eq13_e258_d_n2 * p.p1);
        let eq13_e261_d_n3: f64 = (eq13_e258_d_n3 * p.p1);
        let eq13_e261_d_n4: f64 = (eq13_e258_d_n4 * p.p1);
        let eq13_e261_d_n5: f64 = (eq13_e258_d_n5 * p.p1);
        let eq13_e261_d_n6: f64 = (eq13_e258_d_n6 * p.p1);
        let eq13_e261_d_n7: f64 = (eq13_e258_d_n7 * p.p1);
        let eq13_e261_d_n8: f64 = (eq13_e258_d_n8 * p.p1);
        let eq13_e261_d_n9: f64 = (eq13_e258_d_n9 * p.p1);
        let eq13_e261_d_n10: f64 = (eq13_e258_d_n10 * p.p1);
        let eq13_e261_d_b0: f64 = (eq13_e258_d_b0 * p.p1);
        let eq13_e261_d_b1: f64 = (eq13_e258_d_b1 * p.p1);
        let eq13_e261_q: f64 = (eq13_e259_q * p.p1);
        let eq13_reactive_node_derivatives: [f64; 11] = [eq13_e261_d_n0, eq13_e261_d_n1, eq13_e261_d_n2, eq13_e261_d_n3, eq13_e261_d_n4, eq13_e261_d_n5, eq13_e261_d_n6, eq13_e261_d_n7, eq13_e261_d_n8, eq13_e261_d_n9, eq13_e261_d_n10];
        let eq13_reactive_branch_derivatives: [f64; 2] = [eq13_e261_d_b0, eq13_e261_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[5]),
            nodes,
            &eq13_reactive_node_derivatives,
            branches,
            &eq13_reactive_branch_derivatives,
            multiplicity,
        );
        let eq14_e264: f64 = (p.p3 * p.p68);
        let eq14_e266: f64 = (eq14_e264 * var_vbe);
        let eq14_e266_d_n0: f64 = (eq14_e264 * var_vbe_dn0);
        let eq14_e266_d_n1: f64 = (eq14_e264 * var_vbe_dn1);
        let eq14_e266_d_n2: f64 = (eq14_e264 * var_vbe_dn2);
        let eq14_e266_d_n3: f64 = (eq14_e264 * var_vbe_dn3);
        let eq14_e266_d_n4: f64 = (eq14_e264 * var_vbe_dn4);
        let eq14_e266_d_n5: f64 = (eq14_e264 * var_vbe_dn5);
        let eq14_e266_d_n6: f64 = (eq14_e264 * var_vbe_dn6);
        let eq14_e266_d_n7: f64 = (eq14_e264 * var_vbe_dn7);
        let eq14_e266_d_n8: f64 = (eq14_e264 * var_vbe_dn8);
        let eq14_e266_d_n9: f64 = (eq14_e264 * var_vbe_dn9);
        let eq14_e266_d_n10: f64 = (eq14_e264 * var_vbe_dn10);
        let eq14_e266_d_b0: f64 = (eq14_e264 * var_vbe_db0);
        let eq14_e266_d_b1: f64 = (eq14_e264 * var_vbe_db1);
        let eq14_e267_q: f64 = eq14_e266;
        let eq14_e269: f64 = (eq14_e266 * p.p1);
        let eq14_e269_d_n0: f64 = (eq14_e266_d_n0 * p.p1);
        let eq14_e269_d_n1: f64 = (eq14_e266_d_n1 * p.p1);
        let eq14_e269_d_n2: f64 = (eq14_e266_d_n2 * p.p1);
        let eq14_e269_d_n3: f64 = (eq14_e266_d_n3 * p.p1);
        let eq14_e269_d_n4: f64 = (eq14_e266_d_n4 * p.p1);
        let eq14_e269_d_n5: f64 = (eq14_e266_d_n5 * p.p1);
        let eq14_e269_d_n6: f64 = (eq14_e266_d_n6 * p.p1);
        let eq14_e269_d_n7: f64 = (eq14_e266_d_n7 * p.p1);
        let eq14_e269_d_n8: f64 = (eq14_e266_d_n8 * p.p1);
        let eq14_e269_d_n9: f64 = (eq14_e266_d_n9 * p.p1);
        let eq14_e269_d_n10: f64 = (eq14_e266_d_n10 * p.p1);
        let eq14_e269_d_b0: f64 = (eq14_e266_d_b0 * p.p1);
        let eq14_e269_d_b1: f64 = (eq14_e266_d_b1 * p.p1);
        let eq14_e269_q: f64 = (eq14_e267_q * p.p1);
        let eq14_reactive_node_derivatives: [f64; 11] = [eq14_e269_d_n0, eq14_e269_d_n1, eq14_e269_d_n2, eq14_e269_d_n3, eq14_e269_d_n4, eq14_e269_d_n5, eq14_e269_d_n6, eq14_e269_d_n7, eq14_e269_d_n8, eq14_e269_d_n9, eq14_e269_d_n10];
        let eq14_reactive_branch_derivatives: [f64; 2] = [eq14_e269_d_b0, eq14_e269_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes,
            &eq14_reactive_node_derivatives,
            branches,
            &eq14_reactive_branch_derivatives,
            multiplicity,
        );
        let eq15_e272: f64 = (p.p3 * p.p77);
        let eq15_e274: f64 = (eq15_e272 * var_vbc);
        let eq15_e274_d_n0: f64 = (eq15_e272 * var_vbc_dn0);
        let eq15_e274_d_n1: f64 = (eq15_e272 * var_vbc_dn1);
        let eq15_e274_d_n2: f64 = (eq15_e272 * var_vbc_dn2);
        let eq15_e274_d_n3: f64 = (eq15_e272 * var_vbc_dn3);
        let eq15_e274_d_n4: f64 = (eq15_e272 * var_vbc_dn4);
        let eq15_e274_d_n5: f64 = (eq15_e272 * var_vbc_dn5);
        let eq15_e274_d_n6: f64 = (eq15_e272 * var_vbc_dn6);
        let eq15_e274_d_n7: f64 = (eq15_e272 * var_vbc_dn7);
        let eq15_e274_d_n8: f64 = (eq15_e272 * var_vbc_dn8);
        let eq15_e274_d_n9: f64 = (eq15_e272 * var_vbc_dn9);
        let eq15_e274_d_n10: f64 = (eq15_e272 * var_vbc_dn10);
        let eq15_e274_d_b0: f64 = (eq15_e272 * var_vbc_db0);
        let eq15_e274_d_b1: f64 = (eq15_e272 * var_vbc_db1);
        let eq15_e275_q: f64 = eq15_e274;
        let eq15_e277: f64 = (eq15_e274 * p.p1);
        let eq15_e277_d_n0: f64 = (eq15_e274_d_n0 * p.p1);
        let eq15_e277_d_n1: f64 = (eq15_e274_d_n1 * p.p1);
        let eq15_e277_d_n2: f64 = (eq15_e274_d_n2 * p.p1);
        let eq15_e277_d_n3: f64 = (eq15_e274_d_n3 * p.p1);
        let eq15_e277_d_n4: f64 = (eq15_e274_d_n4 * p.p1);
        let eq15_e277_d_n5: f64 = (eq15_e274_d_n5 * p.p1);
        let eq15_e277_d_n6: f64 = (eq15_e274_d_n6 * p.p1);
        let eq15_e277_d_n7: f64 = (eq15_e274_d_n7 * p.p1);
        let eq15_e277_d_n8: f64 = (eq15_e274_d_n8 * p.p1);
        let eq15_e277_d_n9: f64 = (eq15_e274_d_n9 * p.p1);
        let eq15_e277_d_n10: f64 = (eq15_e274_d_n10 * p.p1);
        let eq15_e277_d_b0: f64 = (eq15_e274_d_b0 * p.p1);
        let eq15_e277_d_b1: f64 = (eq15_e274_d_b1 * p.p1);
        let eq15_e277_q: f64 = (eq15_e275_q * p.p1);
        let eq15_reactive_node_derivatives: [f64; 11] = [eq15_e277_d_n0, eq15_e277_d_n1, eq15_e277_d_n2, eq15_e277_d_n3, eq15_e277_d_n4, eq15_e277_d_n5, eq15_e277_d_n6, eq15_e277_d_n7, eq15_e277_d_n8, eq15_e277_d_n9, eq15_e277_d_n10];
        let eq15_reactive_branch_derivatives: [f64; 2] = [eq15_e277_d_b0, eq15_e277_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let eq18_e293: f64 = (var_xqtex + var_xqex);
        let eq18_e293_d_n0: f64 = (var_xqtex_dn0 + var_xqex_dn0);
        let eq18_e293_d_n1: f64 = (var_xqtex_dn1 + var_xqex_dn1);
        let eq18_e293_d_n2: f64 = (var_xqtex_dn2 + var_xqex_dn2);
        let eq18_e293_d_n3: f64 = (var_xqtex_dn3 + var_xqex_dn3);
        let eq18_e293_d_n4: f64 = (var_xqtex_dn4 + var_xqex_dn4);
        let eq18_e293_d_n5: f64 = (var_xqtex_dn5 + var_xqex_dn5);
        let eq18_e293_d_n6: f64 = (var_xqtex_dn6 + var_xqex_dn6);
        let eq18_e293_d_n7: f64 = (var_xqtex_dn7 + var_xqex_dn7);
        let eq18_e293_d_n8: f64 = (var_xqtex_dn8 + var_xqex_dn8);
        let eq18_e293_d_n9: f64 = (var_xqtex_dn9 + var_xqex_dn9);
        let eq18_e293_d_n10: f64 = (var_xqtex_dn10 + var_xqex_dn10);
        let eq18_e293_d_b0: f64 = (var_xqtex_db0 + var_xqex_db0);
        let eq18_e293_d_b1: f64 = (var_xqtex_db1 + var_xqex_db1);
        let eq18_e294: f64 = (p.p3 * eq18_e293);
        let eq18_e294_d_n0: f64 = (p.p3 * eq18_e293_d_n0);
        let eq18_e294_d_n1: f64 = (p.p3 * eq18_e293_d_n1);
        let eq18_e294_d_n2: f64 = (p.p3 * eq18_e293_d_n2);
        let eq18_e294_d_n3: f64 = (p.p3 * eq18_e293_d_n3);
        let eq18_e294_d_n4: f64 = (p.p3 * eq18_e293_d_n4);
        let eq18_e294_d_n5: f64 = (p.p3 * eq18_e293_d_n5);
        let eq18_e294_d_n6: f64 = (p.p3 * eq18_e293_d_n6);
        let eq18_e294_d_n7: f64 = (p.p3 * eq18_e293_d_n7);
        let eq18_e294_d_n8: f64 = (p.p3 * eq18_e293_d_n8);
        let eq18_e294_d_n9: f64 = (p.p3 * eq18_e293_d_n9);
        let eq18_e294_d_n10: f64 = (p.p3 * eq18_e293_d_n10);
        let eq18_e294_d_b0: f64 = (p.p3 * eq18_e293_d_b0);
        let eq18_e294_d_b1: f64 = (p.p3 * eq18_e293_d_b1);
        let eq18_e295_q: f64 = eq18_e294;
        let eq18_e297: f64 = (eq18_e294 * p.p1);
        let eq18_e297_d_n0: f64 = (eq18_e294_d_n0 * p.p1);
        let eq18_e297_d_n1: f64 = (eq18_e294_d_n1 * p.p1);
        let eq18_e297_d_n2: f64 = (eq18_e294_d_n2 * p.p1);
        let eq18_e297_d_n3: f64 = (eq18_e294_d_n3 * p.p1);
        let eq18_e297_d_n4: f64 = (eq18_e294_d_n4 * p.p1);
        let eq18_e297_d_n5: f64 = (eq18_e294_d_n5 * p.p1);
        let eq18_e297_d_n6: f64 = (eq18_e294_d_n6 * p.p1);
        let eq18_e297_d_n7: f64 = (eq18_e294_d_n7 * p.p1);
        let eq18_e297_d_n8: f64 = (eq18_e294_d_n8 * p.p1);
        let eq18_e297_d_n9: f64 = (eq18_e294_d_n9 * p.p1);
        let eq18_e297_d_n10: f64 = (eq18_e294_d_n10 * p.p1);
        let eq18_e297_d_b0: f64 = (eq18_e294_d_b0 * p.p1);
        let eq18_e297_d_b1: f64 = (eq18_e294_d_b1 * p.p1);
        let eq18_e297_q: f64 = (eq18_e295_q * p.p1);
        let eq18_reactive_node_derivatives: [f64; 11] = [eq18_e297_d_n0, eq18_e297_d_n1, eq18_e297_d_n2, eq18_e297_d_n3, eq18_e297_d_n4, eq18_e297_d_n5, eq18_e297_d_n6, eq18_e297_d_n7, eq18_e297_d_n8, eq18_e297_d_n9, eq18_e297_d_n10];
        let eq18_reactive_branch_derivatives: [f64; 2] = [eq18_e297_d_b0, eq18_e297_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[8]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let eq20_e312: f64 = (var_qtex + var_qex);
        let eq20_e312_d_n0: f64 = (var_qtex_dn0 + var_qex_dn0);
        let eq20_e312_d_n1: f64 = (var_qtex_dn1 + var_qex_dn1);
        let eq20_e312_d_n2: f64 = (var_qtex_dn2 + var_qex_dn2);
        let eq20_e312_d_n3: f64 = (var_qtex_dn3 + var_qex_dn3);
        let eq20_e312_d_n4: f64 = (var_qtex_dn4 + var_qex_dn4);
        let eq20_e312_d_n5: f64 = (var_qtex_dn5 + var_qex_dn5);
        let eq20_e312_d_n6: f64 = (var_qtex_dn6 + var_qex_dn6);
        let eq20_e312_d_n7: f64 = (var_qtex_dn7 + var_qex_dn7);
        let eq20_e312_d_n8: f64 = (var_qtex_dn8 + var_qex_dn8);
        let eq20_e312_d_n9: f64 = (var_qtex_dn9 + var_qex_dn9);
        let eq20_e312_d_n10: f64 = (var_qtex_dn10 + var_qex_dn10);
        let eq20_e312_d_b0: f64 = (var_qtex_db0 + var_qex_db0);
        let eq20_e312_d_b1: f64 = (var_qtex_db1 + var_qex_db1);
        let eq20_e313: f64 = (p.p3 * eq20_e312);
        let eq20_e313_d_n0: f64 = (p.p3 * eq20_e312_d_n0);
        let eq20_e313_d_n1: f64 = (p.p3 * eq20_e312_d_n1);
        let eq20_e313_d_n2: f64 = (p.p3 * eq20_e312_d_n2);
        let eq20_e313_d_n3: f64 = (p.p3 * eq20_e312_d_n3);
        let eq20_e313_d_n4: f64 = (p.p3 * eq20_e312_d_n4);
        let eq20_e313_d_n5: f64 = (p.p3 * eq20_e312_d_n5);
        let eq20_e313_d_n6: f64 = (p.p3 * eq20_e312_d_n6);
        let eq20_e313_d_n7: f64 = (p.p3 * eq20_e312_d_n7);
        let eq20_e313_d_n8: f64 = (p.p3 * eq20_e312_d_n8);
        let eq20_e313_d_n9: f64 = (p.p3 * eq20_e312_d_n9);
        let eq20_e313_d_n10: f64 = (p.p3 * eq20_e312_d_n10);
        let eq20_e313_d_b0: f64 = (p.p3 * eq20_e312_d_b0);
        let eq20_e313_d_b1: f64 = (p.p3 * eq20_e312_d_b1);
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
        let eq20_e316_d_b0: f64 = (eq20_e313_d_b0 * p.p1);
        let eq20_e316_d_b1: f64 = (eq20_e313_d_b1 * p.p1);
        let eq20_e316_q: f64 = (eq20_e314_q * p.p1);
        let eq20_reactive_node_derivatives: [f64; 11] = [eq20_e316_d_n0, eq20_e316_d_n1, eq20_e316_d_n2, eq20_e316_d_n3, eq20_e316_d_n4, eq20_e316_d_n5, eq20_e316_d_n6, eq20_e316_d_n7, eq20_e316_d_n8, eq20_e316_d_n9, eq20_e316_d_n10];
        let eq20_reactive_branch_derivatives: [f64; 2] = [eq20_e316_d_b0, eq20_e316_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[9]),
            nodes,
            &eq20_reactive_node_derivatives,
            branches,
            &eq20_reactive_branch_derivatives,
            multiplicity,
        );
        let eq27_e355_q: f64 = (nv10 - 0.0);
        let eq27_e356: f64 = (var_taun * (nv10 - 0.0));
        let eq27_e356_d_n0: f64 = (var_taun_dn0 * (nv10 - 0.0));
        let eq27_e356_d_n1: f64 = (var_taun_dn1 * (nv10 - 0.0));
        let eq27_e356_d_n2: f64 = (var_taun_dn2 * (nv10 - 0.0));
        let eq27_e356_d_n3: f64 = (var_taun_dn3 * (nv10 - 0.0));
        let eq27_e356_d_n4: f64 = (var_taun_dn4 * (nv10 - 0.0));
        let eq27_e356_d_n5: f64 = (var_taun_dn5 * (nv10 - 0.0));
        let eq27_e356_d_n6: f64 = (var_taun_dn6 * (nv10 - 0.0));
        let eq27_e356_d_n7: f64 = (var_taun_dn7 * (nv10 - 0.0));
        let eq27_e356_d_n8: f64 = (var_taun_dn8 * (nv10 - 0.0));
        let eq27_e356_d_n9: f64 = (var_taun_dn9 * (nv10 - 0.0));
        let eq27_e356_d_n10: f64 = ((var_taun_dn10 * (nv10 - 0.0)) + var_taun);
        let eq27_e356_d_b0: f64 = (var_taun_db0 * (nv10 - 0.0));
        let eq27_e356_d_b1: f64 = (var_taun_db1 * (nv10 - 0.0));
        let eq27_e356_q: f64 = (var_taun * eq27_e355_q);
        let eq27_e356_q_d_n0: f64 = (var_taun_dn0 * eq27_e355_q);
        let eq27_e356_q_d_n1: f64 = (var_taun_dn1 * eq27_e355_q);
        let eq27_e356_q_d_n2: f64 = (var_taun_dn2 * eq27_e355_q);
        let eq27_e356_q_d_n3: f64 = (var_taun_dn3 * eq27_e355_q);
        let eq27_e356_q_d_n4: f64 = (var_taun_dn4 * eq27_e355_q);
        let eq27_e356_q_d_n5: f64 = (var_taun_dn5 * eq27_e355_q);
        let eq27_e356_q_d_n6: f64 = (var_taun_dn6 * eq27_e355_q);
        let eq27_e356_q_d_n7: f64 = (var_taun_dn7 * eq27_e355_q);
        let eq27_e356_q_d_n8: f64 = (var_taun_dn8 * eq27_e355_q);
        let eq27_e356_q_d_n9: f64 = (var_taun_dn9 * eq27_e355_q);
        let eq27_e356_q_d_n10: f64 = ((var_taun_dn10 * eq27_e355_q) + var_taun);
        let eq27_e356_q_d_b0: f64 = (var_taun_db0 * eq27_e355_q);
        let eq27_e356_q_d_b1: f64 = (var_taun_db1 * eq27_e355_q);
        let eq27_reactive_node_derivatives: [f64; 11] = [eq27_e356_q_d_n0, eq27_e356_q_d_n1, eq27_e356_q_d_n2, eq27_e356_q_d_n3, eq27_e356_q_d_n4, eq27_e356_q_d_n5, eq27_e356_q_d_n6, eq27_e356_q_d_n7, eq27_e356_q_d_n8, eq27_e356_q_d_n9, eq27_e356_q_d_n10];
        let eq27_reactive_branch_derivatives: [f64; 2] = [eq27_e356_q_d_b0, eq27_e356_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[3]),
            nodes,
            &eq27_reactive_node_derivatives,
            branches,
            &eq27_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
