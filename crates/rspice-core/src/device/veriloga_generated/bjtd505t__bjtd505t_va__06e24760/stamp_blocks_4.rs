#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_39(
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
        var_qb0_db0: f64,
        var_qb0_db1: f64,
        var_qb0_dn0: f64,
        var_qb0_dn1: f64,
        var_qb0_dn10: f64,
        var_qb0_dn11: f64,
        var_qb0_dn2: f64,
        var_qb0_dn3: f64,
        var_qb0_dn4: f64,
        var_qb0_dn5: f64,
        var_qb0_dn6: f64,
        var_qb0_dn7: f64,
        var_qb0_dn8: f64,
        var_qb0_dn9: f64,
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
        var_vt: f64,
        var_vt_db0: f64,
        var_vt_db1: f64,
        var_vt_dn0: f64,
        var_vt_dn1: f64,
        var_vt_dn10: f64,
        var_vt_dn11: f64,
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
        var_vtinv_dn11: f64,
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
        var_dn0vb2e1_dn11_slot: &mut f64,
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
        var_dn0vb2e1_rdn11_slot: &mut f64,
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
        var_dqbevb2e1_dn11_slot: &mut f64,
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
        var_dqbevb2e1_rdn11_slot: &mut f64,
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
        var_dqevb2e1_dn11_slot: &mut f64,
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
        var_dqevb2e1_rdn11_slot: &mut f64,
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
        var_dqtevb2e1_dn11_slot: &mut f64,
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
        var_dqtevb2e1_rdn11_slot: &mut f64,
        var_dqtevb2e1_rdn2_slot: &mut f64,
        var_dqtevb2e1_rdn3_slot: &mut f64,
        var_dqtevb2e1_rdn4_slot: &mut f64,
        var_dqtevb2e1_rdn5_slot: &mut f64,
        var_dqtevb2e1_rdn6_slot: &mut f64,
        var_dqtevb2e1_rdn7_slot: &mut f64,
        var_dqtevb2e1_rdn8_slot: &mut f64,
        var_dqtevb2e1_rdn9_slot: &mut f64,
        var_dqtevb2e1_rv_slot: &mut f64,
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
        var_dvjevb2e1_rdb0_slot: &mut f64,
        var_dvjevb2e1_rdb1_slot: &mut f64,
        var_dvjevb2e1_rdn0_slot: &mut f64,
        var_dvjevb2e1_rdn1_slot: &mut f64,
        var_dvjevb2e1_rdn10_slot: &mut f64,
        var_dvjevb2e1_rdn11_slot: &mut f64,
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
        var_dvtevb2e1_dn11_slot: &mut f64,
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
        var_dvtevb2e1_rdn11_slot: &mut f64,
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
        var_dvtevje_dn11_slot: &mut f64,
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
        var_dvtevje_rdn11_slot: &mut f64,
        var_dvtevje_rdn2_slot: &mut f64,
        var_dvtevje_rdn3_slot: &mut f64,
        var_dvtevje_rdn4_slot: &mut f64,
        var_dvtevje_rdn5_slot: &mut f64,
        var_dvtevje_rdn6_slot: &mut f64,
        var_dvtevje_rdn7_slot: &mut f64,
        var_dvtevje_rdn8_slot: &mut f64,
        var_dvtevje_rdn9_slot: &mut f64,
        var_dvtevje_rv_slot: &mut f64,
        var_guard115_slot: &mut f64,
        var_guard115_db0_slot: &mut f64,
        var_guard115_db1_slot: &mut f64,
        var_guard115_dn0_slot: &mut f64,
        var_guard115_dn1_slot: &mut f64,
        var_guard115_dn10_slot: &mut f64,
        var_guard115_dn11_slot: &mut f64,
        var_guard115_dn2_slot: &mut f64,
        var_guard115_dn3_slot: &mut f64,
        var_guard115_dn4_slot: &mut f64,
        var_guard115_dn5_slot: &mut f64,
        var_guard115_dn6_slot: &mut f64,
        var_guard115_dn7_slot: &mut f64,
        var_guard115_dn8_slot: &mut f64,
        var_guard115_dn9_slot: &mut f64,
        var_guard115_rdb0_slot: &mut f64,
        var_guard115_rdb1_slot: &mut f64,
        var_guard115_rdn0_slot: &mut f64,
        var_guard115_rdn1_slot: &mut f64,
        var_guard115_rdn10_slot: &mut f64,
        var_guard115_rdn11_slot: &mut f64,
        var_guard115_rdn2_slot: &mut f64,
        var_guard115_rdn3_slot: &mut f64,
        var_guard115_rdn4_slot: &mut f64,
        var_guard115_rdn5_slot: &mut f64,
        var_guard115_rdn6_slot: &mut f64,
        var_guard115_rdn7_slot: &mut f64,
        var_guard115_rdn8_slot: &mut f64,
        var_guard115_rdn9_slot: &mut f64,
        var_guard115_rv_slot: &mut f64,
        var_guard116_slot: &mut f64,
        var_guard116_db0_slot: &mut f64,
        var_guard116_db1_slot: &mut f64,
        var_guard116_dn0_slot: &mut f64,
        var_guard116_dn1_slot: &mut f64,
        var_guard116_dn10_slot: &mut f64,
        var_guard116_dn11_slot: &mut f64,
        var_guard116_dn2_slot: &mut f64,
        var_guard116_dn3_slot: &mut f64,
        var_guard116_dn4_slot: &mut f64,
        var_guard116_dn5_slot: &mut f64,
        var_guard116_dn6_slot: &mut f64,
        var_guard116_dn7_slot: &mut f64,
        var_guard116_dn8_slot: &mut f64,
        var_guard116_dn9_slot: &mut f64,
        var_guard116_rdb0_slot: &mut f64,
        var_guard116_rdb1_slot: &mut f64,
        var_guard116_rdn0_slot: &mut f64,
        var_guard116_rdn1_slot: &mut f64,
        var_guard116_rdn10_slot: &mut f64,
        var_guard116_rdn11_slot: &mut f64,
        var_guard116_rdn2_slot: &mut f64,
        var_guard116_rdn3_slot: &mut f64,
        var_guard116_rdn4_slot: &mut f64,
        var_guard116_rdn5_slot: &mut f64,
        var_guard116_rdn6_slot: &mut f64,
        var_guard116_rdn7_slot: &mut f64,
        var_guard116_rdn8_slot: &mut f64,
        var_guard116_rdn9_slot: &mut f64,
        var_guard116_rv_slot: &mut f64,
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
        var_qb1b2_rdb0_slot: &mut f64,
        var_qb1b2_rdb1_slot: &mut f64,
        var_qb1b2_rdn0_slot: &mut f64,
        var_qb1b2_rdn1_slot: &mut f64,
        var_qb1b2_rdn10_slot: &mut f64,
        var_qb1b2_rdn11_slot: &mut f64,
        var_qb1b2_rdn2_slot: &mut f64,
        var_qb1b2_rdn3_slot: &mut f64,
        var_qb1b2_rdn4_slot: &mut f64,
        var_qb1b2_rdn5_slot: &mut f64,
        var_qb1b2_rdn6_slot: &mut f64,
        var_qb1b2_rdn7_slot: &mut f64,
        var_qb1b2_rdn8_slot: &mut f64,
        var_qb1b2_rdn9_slot: &mut f64,
        var_qb1b2_rv_slot: &mut f64,
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
        var_vb2e1vfe_rdb0_slot: &mut f64,
        var_vb2e1vfe_rdb1_slot: &mut f64,
        var_vb2e1vfe_rdn0_slot: &mut f64,
        var_vb2e1vfe_rdn1_slot: &mut f64,
        var_vb2e1vfe_rdn10_slot: &mut f64,
        var_vb2e1vfe_rdn11_slot: &mut f64,
        var_vb2e1vfe_rdn2_slot: &mut f64,
        var_vb2e1vfe_rdn3_slot: &mut f64,
        var_vb2e1vfe_rdn4_slot: &mut f64,
        var_vb2e1vfe_rdn5_slot: &mut f64,
        var_vb2e1vfe_rdn6_slot: &mut f64,
        var_vb2e1vfe_rdn7_slot: &mut f64,
        var_vb2e1vfe_rdn8_slot: &mut f64,
        var_vb2e1vfe_rdn9_slot: &mut f64,
        var_vb2e1vfe_rv_slot: &mut f64,
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
        let mut var_dn0vb2e1_rdb0: f64 = *var_dn0vb2e1_rdb0_slot;
        let mut var_dn0vb2e1_rdb1: f64 = *var_dn0vb2e1_rdb1_slot;
        let mut var_dn0vb2e1_rdn0: f64 = *var_dn0vb2e1_rdn0_slot;
        let mut var_dn0vb2e1_rdn1: f64 = *var_dn0vb2e1_rdn1_slot;
        let mut var_dn0vb2e1_rdn10: f64 = *var_dn0vb2e1_rdn10_slot;
        let mut var_dn0vb2e1_rdn11: f64 = *var_dn0vb2e1_rdn11_slot;
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
        let mut var_dqbevb2e1_dn11: f64 = *var_dqbevb2e1_dn11_slot;
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
        let mut var_dqbevb2e1_rdn11: f64 = *var_dqbevb2e1_rdn11_slot;
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
        let mut var_dqevb2e1_dn11: f64 = *var_dqevb2e1_dn11_slot;
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
        let mut var_dqevb2e1_rdn11: f64 = *var_dqevb2e1_rdn11_slot;
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
        let mut var_dqtevb2e1_dn11: f64 = *var_dqtevb2e1_dn11_slot;
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
        let mut var_dqtevb2e1_rdn11: f64 = *var_dqtevb2e1_rdn11_slot;
        let mut var_dqtevb2e1_rdn2: f64 = *var_dqtevb2e1_rdn2_slot;
        let mut var_dqtevb2e1_rdn3: f64 = *var_dqtevb2e1_rdn3_slot;
        let mut var_dqtevb2e1_rdn4: f64 = *var_dqtevb2e1_rdn4_slot;
        let mut var_dqtevb2e1_rdn5: f64 = *var_dqtevb2e1_rdn5_slot;
        let mut var_dqtevb2e1_rdn6: f64 = *var_dqtevb2e1_rdn6_slot;
        let mut var_dqtevb2e1_rdn7: f64 = *var_dqtevb2e1_rdn7_slot;
        let mut var_dqtevb2e1_rdn8: f64 = *var_dqtevb2e1_rdn8_slot;
        let mut var_dqtevb2e1_rdn9: f64 = *var_dqtevb2e1_rdn9_slot;
        let mut var_dqtevb2e1_rv: f64 = *var_dqtevb2e1_rv_slot;
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
        let mut var_dvjevb2e1_rdb0: f64 = *var_dvjevb2e1_rdb0_slot;
        let mut var_dvjevb2e1_rdb1: f64 = *var_dvjevb2e1_rdb1_slot;
        let mut var_dvjevb2e1_rdn0: f64 = *var_dvjevb2e1_rdn0_slot;
        let mut var_dvjevb2e1_rdn1: f64 = *var_dvjevb2e1_rdn1_slot;
        let mut var_dvjevb2e1_rdn10: f64 = *var_dvjevb2e1_rdn10_slot;
        let mut var_dvjevb2e1_rdn11: f64 = *var_dvjevb2e1_rdn11_slot;
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
        let mut var_dvtevb2e1_dn11: f64 = *var_dvtevb2e1_dn11_slot;
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
        let mut var_dvtevb2e1_rdn11: f64 = *var_dvtevb2e1_rdn11_slot;
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
        let mut var_dvtevje_dn11: f64 = *var_dvtevje_dn11_slot;
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
        let mut var_dvtevje_rdn11: f64 = *var_dvtevje_rdn11_slot;
        let mut var_dvtevje_rdn2: f64 = *var_dvtevje_rdn2_slot;
        let mut var_dvtevje_rdn3: f64 = *var_dvtevje_rdn3_slot;
        let mut var_dvtevje_rdn4: f64 = *var_dvtevje_rdn4_slot;
        let mut var_dvtevje_rdn5: f64 = *var_dvtevje_rdn5_slot;
        let mut var_dvtevje_rdn6: f64 = *var_dvtevje_rdn6_slot;
        let mut var_dvtevje_rdn7: f64 = *var_dvtevje_rdn7_slot;
        let mut var_dvtevje_rdn8: f64 = *var_dvtevje_rdn8_slot;
        let mut var_dvtevje_rdn9: f64 = *var_dvtevje_rdn9_slot;
        let mut var_dvtevje_rv: f64 = *var_dvtevje_rv_slot;
        let mut var_guard115: f64 = *var_guard115_slot;
        let mut var_guard115_db0: f64 = *var_guard115_db0_slot;
        let mut var_guard115_db1: f64 = *var_guard115_db1_slot;
        let mut var_guard115_dn0: f64 = *var_guard115_dn0_slot;
        let mut var_guard115_dn1: f64 = *var_guard115_dn1_slot;
        let mut var_guard115_dn10: f64 = *var_guard115_dn10_slot;
        let mut var_guard115_dn11: f64 = *var_guard115_dn11_slot;
        let mut var_guard115_dn2: f64 = *var_guard115_dn2_slot;
        let mut var_guard115_dn3: f64 = *var_guard115_dn3_slot;
        let mut var_guard115_dn4: f64 = *var_guard115_dn4_slot;
        let mut var_guard115_dn5: f64 = *var_guard115_dn5_slot;
        let mut var_guard115_dn6: f64 = *var_guard115_dn6_slot;
        let mut var_guard115_dn7: f64 = *var_guard115_dn7_slot;
        let mut var_guard115_dn8: f64 = *var_guard115_dn8_slot;
        let mut var_guard115_dn9: f64 = *var_guard115_dn9_slot;
        let mut var_guard115_rdb0: f64 = *var_guard115_rdb0_slot;
        let mut var_guard115_rdb1: f64 = *var_guard115_rdb1_slot;
        let mut var_guard115_rdn0: f64 = *var_guard115_rdn0_slot;
        let mut var_guard115_rdn1: f64 = *var_guard115_rdn1_slot;
        let mut var_guard115_rdn10: f64 = *var_guard115_rdn10_slot;
        let mut var_guard115_rdn11: f64 = *var_guard115_rdn11_slot;
        let mut var_guard115_rdn2: f64 = *var_guard115_rdn2_slot;
        let mut var_guard115_rdn3: f64 = *var_guard115_rdn3_slot;
        let mut var_guard115_rdn4: f64 = *var_guard115_rdn4_slot;
        let mut var_guard115_rdn5: f64 = *var_guard115_rdn5_slot;
        let mut var_guard115_rdn6: f64 = *var_guard115_rdn6_slot;
        let mut var_guard115_rdn7: f64 = *var_guard115_rdn7_slot;
        let mut var_guard115_rdn8: f64 = *var_guard115_rdn8_slot;
        let mut var_guard115_rdn9: f64 = *var_guard115_rdn9_slot;
        let mut var_guard115_rv: f64 = *var_guard115_rv_slot;
        let mut var_guard116: f64 = *var_guard116_slot;
        let mut var_guard116_db0: f64 = *var_guard116_db0_slot;
        let mut var_guard116_db1: f64 = *var_guard116_db1_slot;
        let mut var_guard116_dn0: f64 = *var_guard116_dn0_slot;
        let mut var_guard116_dn1: f64 = *var_guard116_dn1_slot;
        let mut var_guard116_dn10: f64 = *var_guard116_dn10_slot;
        let mut var_guard116_dn11: f64 = *var_guard116_dn11_slot;
        let mut var_guard116_dn2: f64 = *var_guard116_dn2_slot;
        let mut var_guard116_dn3: f64 = *var_guard116_dn3_slot;
        let mut var_guard116_dn4: f64 = *var_guard116_dn4_slot;
        let mut var_guard116_dn5: f64 = *var_guard116_dn5_slot;
        let mut var_guard116_dn6: f64 = *var_guard116_dn6_slot;
        let mut var_guard116_dn7: f64 = *var_guard116_dn7_slot;
        let mut var_guard116_dn8: f64 = *var_guard116_dn8_slot;
        let mut var_guard116_dn9: f64 = *var_guard116_dn9_slot;
        let mut var_guard116_rdb0: f64 = *var_guard116_rdb0_slot;
        let mut var_guard116_rdb1: f64 = *var_guard116_rdb1_slot;
        let mut var_guard116_rdn0: f64 = *var_guard116_rdn0_slot;
        let mut var_guard116_rdn1: f64 = *var_guard116_rdn1_slot;
        let mut var_guard116_rdn10: f64 = *var_guard116_rdn10_slot;
        let mut var_guard116_rdn11: f64 = *var_guard116_rdn11_slot;
        let mut var_guard116_rdn2: f64 = *var_guard116_rdn2_slot;
        let mut var_guard116_rdn3: f64 = *var_guard116_rdn3_slot;
        let mut var_guard116_rdn4: f64 = *var_guard116_rdn4_slot;
        let mut var_guard116_rdn5: f64 = *var_guard116_rdn5_slot;
        let mut var_guard116_rdn6: f64 = *var_guard116_rdn6_slot;
        let mut var_guard116_rdn7: f64 = *var_guard116_rdn7_slot;
        let mut var_guard116_rdn8: f64 = *var_guard116_rdn8_slot;
        let mut var_guard116_rdn9: f64 = *var_guard116_rdn9_slot;
        let mut var_guard116_rv: f64 = *var_guard116_rv_slot;
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
        let mut var_qb1b2_rdb0: f64 = *var_qb1b2_rdb0_slot;
        let mut var_qb1b2_rdb1: f64 = *var_qb1b2_rdb1_slot;
        let mut var_qb1b2_rdn0: f64 = *var_qb1b2_rdn0_slot;
        let mut var_qb1b2_rdn1: f64 = *var_qb1b2_rdn1_slot;
        let mut var_qb1b2_rdn10: f64 = *var_qb1b2_rdn10_slot;
        let mut var_qb1b2_rdn11: f64 = *var_qb1b2_rdn11_slot;
        let mut var_qb1b2_rdn2: f64 = *var_qb1b2_rdn2_slot;
        let mut var_qb1b2_rdn3: f64 = *var_qb1b2_rdn3_slot;
        let mut var_qb1b2_rdn4: f64 = *var_qb1b2_rdn4_slot;
        let mut var_qb1b2_rdn5: f64 = *var_qb1b2_rdn5_slot;
        let mut var_qb1b2_rdn6: f64 = *var_qb1b2_rdn6_slot;
        let mut var_qb1b2_rdn7: f64 = *var_qb1b2_rdn7_slot;
        let mut var_qb1b2_rdn8: f64 = *var_qb1b2_rdn8_slot;
        let mut var_qb1b2_rdn9: f64 = *var_qb1b2_rdn9_slot;
        let mut var_qb1b2_rv: f64 = *var_qb1b2_rv_slot;
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
        let mut var_vb2e1vfe_rdb0: f64 = *var_vb2e1vfe_rdb0_slot;
        let mut var_vb2e1vfe_rdb1: f64 = *var_vb2e1vfe_rdb1_slot;
        let mut var_vb2e1vfe_rdn0: f64 = *var_vb2e1vfe_rdn0_slot;
        let mut var_vb2e1vfe_rdn1: f64 = *var_vb2e1vfe_rdn1_slot;
        let mut var_vb2e1vfe_rdn10: f64 = *var_vb2e1vfe_rdn10_slot;
        let mut var_vb2e1vfe_rdn11: f64 = *var_vb2e1vfe_rdn11_slot;
        let mut var_vb2e1vfe_rdn2: f64 = *var_vb2e1vfe_rdn2_slot;
        let mut var_vb2e1vfe_rdn3: f64 = *var_vb2e1vfe_rdn3_slot;
        let mut var_vb2e1vfe_rdn4: f64 = *var_vb2e1vfe_rdn4_slot;
        let mut var_vb2e1vfe_rdn5: f64 = *var_vb2e1vfe_rdn5_slot;
        let mut var_vb2e1vfe_rdn6: f64 = *var_vb2e1vfe_rdn6_slot;
        let mut var_vb2e1vfe_rdn7: f64 = *var_vb2e1vfe_rdn7_slot;
        let mut var_vb2e1vfe_rdn8: f64 = *var_vb2e1vfe_rdn8_slot;
        let mut var_vb2e1vfe_rdn9: f64 = *var_vb2e1vfe_rdn9_slot;
        let mut var_vb2e1vfe_rv: f64 = *var_vb2e1vfe_rv_slot;

        let assign6250_e6417: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        var_guard115 = assign6250_e6417;
        var_guard115_dn0 = 0.0;
        var_guard115_dn1 = 0.0;
        var_guard115_dn2 = 0.0;
        var_guard115_dn3 = 0.0;
        var_guard115_dn4 = 0.0;
        var_guard115_dn5 = 0.0;
        var_guard115_dn6 = 0.0;
        var_guard115_dn7 = 0.0;
        var_guard115_dn8 = 0.0;
        var_guard115_dn9 = 0.0;
        var_guard115_dn10 = 0.0;
        var_guard115_dn11 = 0.0;
        var_guard115_db0 = 0.0;
        var_guard115_db1 = 0.0;
        var_guard115_rv = 0.0;
        var_guard115_rdn0 = 0.0;
        var_guard115_rdn1 = 0.0;
        var_guard115_rdn2 = 0.0;
        var_guard115_rdn3 = 0.0;
        var_guard115_rdn4 = 0.0;
        var_guard115_rdn5 = 0.0;
        var_guard115_rdn6 = 0.0;
        var_guard115_rdn7 = 0.0;
        var_guard115_rdn8 = 0.0;
        var_guard115_rdn9 = 0.0;
        var_guard115_rdn10 = 0.0;
        var_guard115_rdn11 = 0.0;
        var_guard115_rdb0 = 0.0;
        var_guard115_rdb1 = 0.0;

        let (assign6260_e6430, assign6260_e6430_d_n0, assign6260_e6430_d_n1, assign6260_e6430_d_n2, assign6260_e6430_d_n3, assign6260_e6430_d_n4, assign6260_e6430_d_n5, assign6260_e6430_d_n6, assign6260_e6430_d_n7, assign6260_e6430_d_n8, assign6260_e6430_d_n9, assign6260_e6430_d_n10, assign6260_e6430_d_n11, assign6260_e6430_d_b0, assign6260_e6430_d_b1,) = {
    if (var_guard115 != 0.0) {
        let assign6260_e6422: f64 = (var_vje * var_inv_vde_t);
        let assign6260_e6423: f64 = (1.0 - assign6260_e6422);
        let assign6260_e6425: f64 = (-p.p66);
        let assign6260_e6426: f64 = (assign6260_e6423).powf(assign6260_e6425);
        let assign6260_e6428: f64 = (assign6260_e6426 - 3.0);
        (assign6260_e6428, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((var_vje_dn0 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn0))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((var_vje_dn0 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn0))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((var_vje_dn1 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn1))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((var_vje_dn1 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn1))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((var_vje_dn2 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn2))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((var_vje_dn2 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn2))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((var_vje_dn3 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn3))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((var_vje_dn3 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn3))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((var_vje_dn4 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn4))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((var_vje_dn4 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn4))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((var_vje_dn5 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn5))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((var_vje_dn5 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn5))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((var_vje_dn6 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn6))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((var_vje_dn6 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn6))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((var_vje_dn7 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn7))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((var_vje_dn7 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn7))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((var_vje_dn8 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn8))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((var_vje_dn8 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn8))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((var_vje_dn9 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn9))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((var_vje_dn9 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn9))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((var_vje_dn10 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn10))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((var_vje_dn10 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn10))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((var_vje_dn11 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn11))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((var_vje_dn11 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn11))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((var_vje_db0 * var_inv_vde_t) + (var_vje * var_inv_vde_t_db0))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((var_vje_db0 * var_inv_vde_t) + (var_vje * var_inv_vde_t_db0))) / assign6260_e6423))) }, if 0.0 == 0.0 && ((assign6260_e6425) as f64).is_finite() && ((assign6260_e6425) as f64).fract() == 0.0 { if assign6260_e6425 == 0.0 { 0.0 } else { (assign6260_e6425 * ((assign6260_e6423).powf(assign6260_e6425 - 1.0) * (-((var_vje_db1 * var_inv_vde_t) + (var_vje * var_inv_vde_t_db1))))) } } else { (assign6260_e6426 * (assign6260_e6425 * ((-((var_vje_db1 * var_inv_vde_t) + (var_vje * var_inv_vde_t_db1))) / assign6260_e6423))) },)
    } else {
        (var_dvtevje, var_dvtevje_dn0, var_dvtevje_dn1, var_dvtevje_dn2, var_dvtevje_dn3, var_dvtevje_dn4, var_dvtevje_dn5, var_dvtevje_dn6, var_dvtevje_dn7, var_dvtevje_dn8, var_dvtevje_dn9, var_dvtevje_dn10, var_dvtevje_dn11, var_dvtevje_db0, var_dvtevje_db1,)
    }
};
        var_dvtevje = assign6260_e6430;
        var_dvtevje_dn0 = assign6260_e6430_d_n0;
        var_dvtevje_dn1 = assign6260_e6430_d_n1;
        var_dvtevje_dn2 = assign6260_e6430_d_n2;
        var_dvtevje_dn3 = assign6260_e6430_d_n3;
        var_dvtevje_dn4 = assign6260_e6430_d_n4;
        var_dvtevje_dn5 = assign6260_e6430_d_n5;
        var_dvtevje_dn6 = assign6260_e6430_d_n6;
        var_dvtevje_dn7 = assign6260_e6430_d_n7;
        var_dvtevje_dn8 = assign6260_e6430_d_n8;
        var_dvtevje_dn9 = assign6260_e6430_d_n9;
        var_dvtevje_dn10 = assign6260_e6430_d_n10;
        var_dvtevje_dn11 = assign6260_e6430_d_n11;
        var_dvtevje_db0 = assign6260_e6430_d_b0;
        var_dvtevje_db1 = assign6260_e6430_d_b1;
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
        var_dvtevje_rdn11 = 0.0;
        var_dvtevje_rdb0 = 0.0;
        var_dvtevje_rdb1 = 0.0;

        let (assign6270_e6438, assign6270_e6438_d_n0, assign6270_e6438_d_n1, assign6270_e6438_d_n2, assign6270_e6438_d_n3, assign6270_e6438_d_n4, assign6270_e6438_d_n5, assign6270_e6438_d_n6, assign6270_e6438_d_n7, assign6270_e6438_d_n8, assign6270_e6438_d_n9, assign6270_e6438_d_n10, assign6270_e6438_d_n11, assign6270_e6438_d_b0, assign6270_e6438_d_b1,) = {
    if (var_guard115 != 0.0) {
        let assign6270_e6434: f64 = (var_vb2e1 - var_vfe);
        let assign6270_e6436: f64 = (assign6270_e6434 / var_a_vde);
        (assign6270_e6436, ((((var_vb2e1_dn0 - var_vfe_dn0) * var_a_vde) - (assign6270_e6434 * var_a_vde_dn0)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn1 - var_vfe_dn1) * var_a_vde) - (assign6270_e6434 * var_a_vde_dn1)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn2 - var_vfe_dn2) * var_a_vde) - (assign6270_e6434 * var_a_vde_dn2)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn3 - var_vfe_dn3) * var_a_vde) - (assign6270_e6434 * var_a_vde_dn3)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn4 - var_vfe_dn4) * var_a_vde) - (assign6270_e6434 * var_a_vde_dn4)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn5 - var_vfe_dn5) * var_a_vde) - (assign6270_e6434 * var_a_vde_dn5)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn6 - var_vfe_dn6) * var_a_vde) - (assign6270_e6434 * var_a_vde_dn6)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn7 - var_vfe_dn7) * var_a_vde) - (assign6270_e6434 * var_a_vde_dn7)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn8 - var_vfe_dn8) * var_a_vde) - (assign6270_e6434 * var_a_vde_dn8)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn9 - var_vfe_dn9) * var_a_vde) - (assign6270_e6434 * var_a_vde_dn9)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn10 - var_vfe_dn10) * var_a_vde) - (assign6270_e6434 * var_a_vde_dn10)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn11 - var_vfe_dn11) * var_a_vde) - (assign6270_e6434 * var_a_vde_dn11)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_db0 - var_vfe_db0) * var_a_vde) - (assign6270_e6434 * var_a_vde_db0)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_db1 - var_vfe_db1) * var_a_vde) - (assign6270_e6434 * var_a_vde_db1)) / (var_a_vde * var_a_vde)),)
    } else {
        (var_vb2e1vfe, var_vb2e1vfe_dn0, var_vb2e1vfe_dn1, var_vb2e1vfe_dn2, var_vb2e1vfe_dn3, var_vb2e1vfe_dn4, var_vb2e1vfe_dn5, var_vb2e1vfe_dn6, var_vb2e1vfe_dn7, var_vb2e1vfe_dn8, var_vb2e1vfe_dn9, var_vb2e1vfe_dn10, var_vb2e1vfe_dn11, var_vb2e1vfe_db0, var_vb2e1vfe_db1,)
    }
};
        var_vb2e1vfe = assign6270_e6438;
        var_vb2e1vfe_dn0 = assign6270_e6438_d_n0;
        var_vb2e1vfe_dn1 = assign6270_e6438_d_n1;
        var_vb2e1vfe_dn2 = assign6270_e6438_d_n2;
        var_vb2e1vfe_dn3 = assign6270_e6438_d_n3;
        var_vb2e1vfe_dn4 = assign6270_e6438_d_n4;
        var_vb2e1vfe_dn5 = assign6270_e6438_d_n5;
        var_vb2e1vfe_dn6 = assign6270_e6438_d_n6;
        var_vb2e1vfe_dn7 = assign6270_e6438_d_n7;
        var_vb2e1vfe_dn8 = assign6270_e6438_d_n8;
        var_vb2e1vfe_dn9 = assign6270_e6438_d_n9;
        var_vb2e1vfe_dn10 = assign6270_e6438_d_n10;
        var_vb2e1vfe_dn11 = assign6270_e6438_d_n11;
        var_vb2e1vfe_db0 = assign6270_e6438_d_b0;
        var_vb2e1vfe_db1 = assign6270_e6438_d_b1;
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
        var_vb2e1vfe_rdn11 = 0.0;
        var_vb2e1vfe_rdb0 = 0.0;
        var_vb2e1vfe_rdb1 = 0.0;

        let assign6280_e6441: f64 = if var_vb2e1vfe < 0.0 { 1.0 } else { 0.0 };
        var_guard116 = assign6280_e6441;
        var_guard116_dn0 = 0.0;
        var_guard116_dn1 = 0.0;
        var_guard116_dn2 = 0.0;
        var_guard116_dn3 = 0.0;
        var_guard116_dn4 = 0.0;
        var_guard116_dn5 = 0.0;
        var_guard116_dn6 = 0.0;
        var_guard116_dn7 = 0.0;
        var_guard116_dn8 = 0.0;
        var_guard116_dn9 = 0.0;
        var_guard116_dn10 = 0.0;
        var_guard116_dn11 = 0.0;
        var_guard116_db0 = 0.0;
        var_guard116_db1 = 0.0;
        var_guard116_rv = 0.0;
        var_guard116_rdn0 = 0.0;
        var_guard116_rdn1 = 0.0;
        var_guard116_rdn2 = 0.0;
        var_guard116_rdn3 = 0.0;
        var_guard116_rdn4 = 0.0;
        var_guard116_rdn5 = 0.0;
        var_guard116_rdn6 = 0.0;
        var_guard116_rdn7 = 0.0;
        var_guard116_rdn8 = 0.0;
        var_guard116_rdn9 = 0.0;
        var_guard116_rdn10 = 0.0;
        var_guard116_rdn11 = 0.0;
        var_guard116_rdb0 = 0.0;
        var_guard116_rdb1 = 0.0;

        let (assign6290_e6452, assign6290_e6452_d_n0, assign6290_e6452_d_n1, assign6290_e6452_d_n2, assign6290_e6452_d_n3, assign6290_e6452_d_n4, assign6290_e6452_d_n5, assign6290_e6452_d_n6, assign6290_e6452_d_n7, assign6290_e6452_d_n8, assign6290_e6452_d_n9, assign6290_e6452_d_n10, assign6290_e6452_d_n11, assign6290_e6452_d_b0, assign6290_e6452_d_b1,) = {
    if ((var_guard115 != 0.0) && (var_guard116 != 0.0)) {
        let assign6290_e6448: f64 = (var_vb2e1vfe).exp();
        let assign6290_e6449: f64 = (1.0 + assign6290_e6448);
        let assign6290_e6450: f64 = (1.0 / assign6290_e6449);
        (assign6290_e6450, (-((assign6290_e6448 * var_vb2e1vfe_dn0) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * var_vb2e1vfe_dn1) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * var_vb2e1vfe_dn2) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * var_vb2e1vfe_dn3) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * var_vb2e1vfe_dn4) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * var_vb2e1vfe_dn5) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * var_vb2e1vfe_dn6) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * var_vb2e1vfe_dn7) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * var_vb2e1vfe_dn8) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * var_vb2e1vfe_dn9) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * var_vb2e1vfe_dn10) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * var_vb2e1vfe_dn11) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * var_vb2e1vfe_db0) / (assign6290_e6449 * assign6290_e6449))), (-((assign6290_e6448 * var_vb2e1vfe_db1) / (assign6290_e6449 * assign6290_e6449))),)
    } else {
        (var_dvjevb2e1, var_dvjevb2e1_dn0, var_dvjevb2e1_dn1, var_dvjevb2e1_dn2, var_dvjevb2e1_dn3, var_dvjevb2e1_dn4, var_dvjevb2e1_dn5, var_dvjevb2e1_dn6, var_dvjevb2e1_dn7, var_dvjevb2e1_dn8, var_dvjevb2e1_dn9, var_dvjevb2e1_dn10, var_dvjevb2e1_dn11, var_dvjevb2e1_db0, var_dvjevb2e1_db1,)
    }
};
        var_dvjevb2e1 = assign6290_e6452;
        var_dvjevb2e1_dn0 = assign6290_e6452_d_n0;
        var_dvjevb2e1_dn1 = assign6290_e6452_d_n1;
        var_dvjevb2e1_dn2 = assign6290_e6452_d_n2;
        var_dvjevb2e1_dn3 = assign6290_e6452_d_n3;
        var_dvjevb2e1_dn4 = assign6290_e6452_d_n4;
        var_dvjevb2e1_dn5 = assign6290_e6452_d_n5;
        var_dvjevb2e1_dn6 = assign6290_e6452_d_n6;
        var_dvjevb2e1_dn7 = assign6290_e6452_d_n7;
        var_dvjevb2e1_dn8 = assign6290_e6452_d_n8;
        var_dvjevb2e1_dn9 = assign6290_e6452_d_n9;
        var_dvjevb2e1_dn10 = assign6290_e6452_d_n10;
        var_dvjevb2e1_dn11 = assign6290_e6452_d_n11;
        var_dvjevb2e1_db0 = assign6290_e6452_d_b0;
        var_dvjevb2e1_db1 = assign6290_e6452_d_b1;
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
        var_dvjevb2e1_rdn11 = 0.0;
        var_dvjevb2e1_rdb0 = 0.0;
        var_dvjevb2e1_rdb1 = 0.0;

        let (assign6300_e6467, assign6300_e6467_d_n0, assign6300_e6467_d_n1, assign6300_e6467_d_n2, assign6300_e6467_d_n3, assign6300_e6467_d_n4, assign6300_e6467_d_n5, assign6300_e6467_d_n6, assign6300_e6467_d_n7, assign6300_e6467_d_n8, assign6300_e6467_d_n9, assign6300_e6467_d_n10, assign6300_e6467_d_n11, assign6300_e6467_d_b0, assign6300_e6467_d_b1,) = {
    if ((var_guard115 != 0.0) && (var_guard116 == 0.0)) {
        let assign6300_e6458: f64 = (-var_vb2e1vfe);
        let assign6300_e6459: f64 = (assign6300_e6458).exp();
        let assign6300_e6462: f64 = (-var_vb2e1vfe);
        let assign6300_e6463: f64 = (assign6300_e6462).exp();
        let assign6300_e6464: f64 = (1.0 + assign6300_e6463);
        let assign6300_e6465: f64 = (assign6300_e6459 / assign6300_e6464);
        (assign6300_e6465, ((((assign6300_e6459 * (-var_vb2e1vfe_dn0)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-var_vb2e1vfe_dn0)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-var_vb2e1vfe_dn1)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-var_vb2e1vfe_dn1)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-var_vb2e1vfe_dn2)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-var_vb2e1vfe_dn2)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-var_vb2e1vfe_dn3)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-var_vb2e1vfe_dn3)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-var_vb2e1vfe_dn4)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-var_vb2e1vfe_dn4)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-var_vb2e1vfe_dn5)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-var_vb2e1vfe_dn5)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-var_vb2e1vfe_dn6)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-var_vb2e1vfe_dn6)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-var_vb2e1vfe_dn7)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-var_vb2e1vfe_dn7)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-var_vb2e1vfe_dn8)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-var_vb2e1vfe_dn8)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-var_vb2e1vfe_dn9)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-var_vb2e1vfe_dn9)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-var_vb2e1vfe_dn10)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-var_vb2e1vfe_dn10)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-var_vb2e1vfe_dn11)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-var_vb2e1vfe_dn11)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-var_vb2e1vfe_db0)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-var_vb2e1vfe_db0)))) / (assign6300_e6464 * assign6300_e6464)), ((((assign6300_e6459 * (-var_vb2e1vfe_db1)) * assign6300_e6464) - (assign6300_e6459 * (assign6300_e6463 * (-var_vb2e1vfe_db1)))) / (assign6300_e6464 * assign6300_e6464)),)
    } else {
        (var_dvjevb2e1, var_dvjevb2e1_dn0, var_dvjevb2e1_dn1, var_dvjevb2e1_dn2, var_dvjevb2e1_dn3, var_dvjevb2e1_dn4, var_dvjevb2e1_dn5, var_dvjevb2e1_dn6, var_dvjevb2e1_dn7, var_dvjevb2e1_dn8, var_dvjevb2e1_dn9, var_dvjevb2e1_dn10, var_dvjevb2e1_dn11, var_dvjevb2e1_db0, var_dvjevb2e1_db1,)
    }
};
        var_dvjevb2e1 = assign6300_e6467;
        var_dvjevb2e1_dn0 = assign6300_e6467_d_n0;
        var_dvjevb2e1_dn1 = assign6300_e6467_d_n1;
        var_dvjevb2e1_dn2 = assign6300_e6467_d_n2;
        var_dvjevb2e1_dn3 = assign6300_e6467_d_n3;
        var_dvjevb2e1_dn4 = assign6300_e6467_d_n4;
        var_dvjevb2e1_dn5 = assign6300_e6467_d_n5;
        var_dvjevb2e1_dn6 = assign6300_e6467_d_n6;
        var_dvjevb2e1_dn7 = assign6300_e6467_d_n7;
        var_dvjevb2e1_dn8 = assign6300_e6467_d_n8;
        var_dvjevb2e1_dn9 = assign6300_e6467_d_n9;
        var_dvjevb2e1_dn10 = assign6300_e6467_d_n10;
        var_dvjevb2e1_dn11 = assign6300_e6467_d_n11;
        var_dvjevb2e1_db0 = assign6300_e6467_d_b0;
        var_dvjevb2e1_db1 = assign6300_e6467_d_b1;
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
        var_dvjevb2e1_rdn11 = 0.0;
        var_dvjevb2e1_rdb0 = 0.0;
        var_dvjevb2e1_rdb1 = 0.0;

        let (assign6310_e6475, assign6310_e6475_d_n0, assign6310_e6475_d_n1, assign6310_e6475_d_n2, assign6310_e6475_d_n3, assign6310_e6475_d_n4, assign6310_e6475_d_n5, assign6310_e6475_d_n6, assign6310_e6475_d_n7, assign6310_e6475_d_n8, assign6310_e6475_d_n9, assign6310_e6475_d_n10, assign6310_e6475_d_n11, assign6310_e6475_d_b0, assign6310_e6475_d_b1,) = {
    if (var_guard115 != 0.0) {
        let assign6310_e6471: f64 = (var_dvtevje * var_dvjevb2e1);
        let assign6310_e6473: f64 = (assign6310_e6471 + 3.0);
        (assign6310_e6473, ((var_dvtevje_dn0 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn0)), ((var_dvtevje_dn1 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn1)), ((var_dvtevje_dn2 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn2)), ((var_dvtevje_dn3 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn3)), ((var_dvtevje_dn4 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn4)), ((var_dvtevje_dn5 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn5)), ((var_dvtevje_dn6 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn6)), ((var_dvtevje_dn7 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn7)), ((var_dvtevje_dn8 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn8)), ((var_dvtevje_dn9 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn9)), ((var_dvtevje_dn10 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn10)), ((var_dvtevje_dn11 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn11)), ((var_dvtevje_db0 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_db0)), ((var_dvtevje_db1 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_db1)),)
    } else {
        (var_dvtevb2e1, var_dvtevb2e1_dn0, var_dvtevb2e1_dn1, var_dvtevb2e1_dn2, var_dvtevb2e1_dn3, var_dvtevb2e1_dn4, var_dvtevb2e1_dn5, var_dvtevb2e1_dn6, var_dvtevb2e1_dn7, var_dvtevb2e1_dn8, var_dvtevb2e1_dn9, var_dvtevb2e1_dn10, var_dvtevb2e1_dn11, var_dvtevb2e1_db0, var_dvtevb2e1_db1,)
    }
};
        var_dvtevb2e1 = assign6310_e6475;
        var_dvtevb2e1_dn0 = assign6310_e6475_d_n0;
        var_dvtevb2e1_dn1 = assign6310_e6475_d_n1;
        var_dvtevb2e1_dn2 = assign6310_e6475_d_n2;
        var_dvtevb2e1_dn3 = assign6310_e6475_d_n3;
        var_dvtevb2e1_dn4 = assign6310_e6475_d_n4;
        var_dvtevb2e1_dn5 = assign6310_e6475_d_n5;
        var_dvtevb2e1_dn6 = assign6310_e6475_d_n6;
        var_dvtevb2e1_dn7 = assign6310_e6475_d_n7;
        var_dvtevb2e1_dn8 = assign6310_e6475_d_n8;
        var_dvtevb2e1_dn9 = assign6310_e6475_d_n9;
        var_dvtevb2e1_dn10 = assign6310_e6475_d_n10;
        var_dvtevb2e1_dn11 = assign6310_e6475_d_n11;
        var_dvtevb2e1_db0 = assign6310_e6475_d_b0;
        var_dvtevb2e1_db1 = assign6310_e6475_d_b1;
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
        var_dvtevb2e1_rdn11 = 0.0;
        var_dvtevb2e1_rdb0 = 0.0;
        var_dvtevb2e1_rdb1 = 0.0;

        let (assign6320_e6485, assign6320_e6485_d_n0, assign6320_e6485_d_n1, assign6320_e6485_d_n2, assign6320_e6485_d_n3, assign6320_e6485_d_n4, assign6320_e6485_d_n5, assign6320_e6485_d_n6, assign6320_e6485_d_n7, assign6320_e6485_d_n8, assign6320_e6485_d_n9, assign6320_e6485_d_n10, assign6320_e6485_d_n11, assign6320_e6485_d_b0, assign6320_e6485_d_b1,) = {
    if (var_guard115 != 0.0) {
        let assign6320_e6479: f64 = (1.0 - p.p67);
        let assign6320_e6481: f64 = (assign6320_e6479 * var_cje_t);
        let assign6320_e6483: f64 = (assign6320_e6481 * var_dvtevb2e1);
        (assign6320_e6483, (((assign6320_e6479 * var_cje_t_dn0) * var_dvtevb2e1) + (assign6320_e6481 * var_dvtevb2e1_dn0)), (((assign6320_e6479 * var_cje_t_dn1) * var_dvtevb2e1) + (assign6320_e6481 * var_dvtevb2e1_dn1)), (((assign6320_e6479 * var_cje_t_dn2) * var_dvtevb2e1) + (assign6320_e6481 * var_dvtevb2e1_dn2)), (((assign6320_e6479 * var_cje_t_dn3) * var_dvtevb2e1) + (assign6320_e6481 * var_dvtevb2e1_dn3)), (((assign6320_e6479 * var_cje_t_dn4) * var_dvtevb2e1) + (assign6320_e6481 * var_dvtevb2e1_dn4)), (((assign6320_e6479 * var_cje_t_dn5) * var_dvtevb2e1) + (assign6320_e6481 * var_dvtevb2e1_dn5)), (((assign6320_e6479 * var_cje_t_dn6) * var_dvtevb2e1) + (assign6320_e6481 * var_dvtevb2e1_dn6)), (((assign6320_e6479 * var_cje_t_dn7) * var_dvtevb2e1) + (assign6320_e6481 * var_dvtevb2e1_dn7)), (((assign6320_e6479 * var_cje_t_dn8) * var_dvtevb2e1) + (assign6320_e6481 * var_dvtevb2e1_dn8)), (((assign6320_e6479 * var_cje_t_dn9) * var_dvtevb2e1) + (assign6320_e6481 * var_dvtevb2e1_dn9)), (((assign6320_e6479 * var_cje_t_dn10) * var_dvtevb2e1) + (assign6320_e6481 * var_dvtevb2e1_dn10)), (((assign6320_e6479 * var_cje_t_dn11) * var_dvtevb2e1) + (assign6320_e6481 * var_dvtevb2e1_dn11)), (((assign6320_e6479 * var_cje_t_db0) * var_dvtevb2e1) + (assign6320_e6481 * var_dvtevb2e1_db0)), (((assign6320_e6479 * var_cje_t_db1) * var_dvtevb2e1) + (assign6320_e6481 * var_dvtevb2e1_db1)),)
    } else {
        (var_dqtevb2e1, var_dqtevb2e1_dn0, var_dqtevb2e1_dn1, var_dqtevb2e1_dn2, var_dqtevb2e1_dn3, var_dqtevb2e1_dn4, var_dqtevb2e1_dn5, var_dqtevb2e1_dn6, var_dqtevb2e1_dn7, var_dqtevb2e1_dn8, var_dqtevb2e1_dn9, var_dqtevb2e1_dn10, var_dqtevb2e1_dn11, var_dqtevb2e1_db0, var_dqtevb2e1_db1,)
    }
};
        var_dqtevb2e1 = assign6320_e6485;
        var_dqtevb2e1_dn0 = assign6320_e6485_d_n0;
        var_dqtevb2e1_dn1 = assign6320_e6485_d_n1;
        var_dqtevb2e1_dn2 = assign6320_e6485_d_n2;
        var_dqtevb2e1_dn3 = assign6320_e6485_d_n3;
        var_dqtevb2e1_dn4 = assign6320_e6485_d_n4;
        var_dqtevb2e1_dn5 = assign6320_e6485_d_n5;
        var_dqtevb2e1_dn6 = assign6320_e6485_d_n6;
        var_dqtevb2e1_dn7 = assign6320_e6485_d_n7;
        var_dqtevb2e1_dn8 = assign6320_e6485_d_n8;
        var_dqtevb2e1_dn9 = assign6320_e6485_d_n9;
        var_dqtevb2e1_dn10 = assign6320_e6485_d_n10;
        var_dqtevb2e1_dn11 = assign6320_e6485_d_n11;
        var_dqtevb2e1_db0 = assign6320_e6485_d_b0;
        var_dqtevb2e1_db1 = assign6320_e6485_d_b1;
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
        var_dqtevb2e1_rdn11 = 0.0;
        var_dqtevb2e1_rdb0 = 0.0;
        var_dqtevb2e1_rdb1 = 0.0;

        let (assign6330_e6502, assign6330_e6502_d_n0, assign6330_e6502_d_n1, assign6330_e6502_d_n2, assign6330_e6502_d_n3, assign6330_e6502_d_n4, assign6330_e6502_d_n5, assign6330_e6502_d_n6, assign6330_e6502_d_n7, assign6330_e6502_d_n8, assign6330_e6502_d_n9, assign6330_e6502_d_n10, assign6330_e6502_d_n11, assign6330_e6502_d_b0, assign6330_e6502_d_b1,) = {
    if (var_guard115 != 0.0) {
        let assign6330_e6489: f64 = (var_if0 * var_evb2e1);
        let assign6330_e6491: f64 = (assign6330_e6489 * var_vtinv);
        let assign6330_e6493: f64 = (assign6330_e6491 / var_nff_t);
        let assign6330_e6497: f64 = (1.0 + var_f1);
        let assign6330_e6498: f64 = (assign6330_e6497).sqrt();
        let assign6330_e6499: f64 = (0.5 / assign6330_e6498);
        let assign6330_e6500: f64 = (assign6330_e6493 * assign6330_e6499);
        (assign6330_e6500, (((((((((var_if0_dn0 * var_evb2e1) + (var_if0 * var_evb2e1_dn0)) * var_vtinv) + (assign6330_e6489 * var_vtinv_dn0)) * var_nff_t) - (assign6330_e6491 * var_nff_t_dn0)) / (var_nff_t * var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (var_f1_dn0 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), (((((((((var_if0_dn1 * var_evb2e1) + (var_if0 * var_evb2e1_dn1)) * var_vtinv) + (assign6330_e6489 * var_vtinv_dn1)) * var_nff_t) - (assign6330_e6491 * var_nff_t_dn1)) / (var_nff_t * var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (var_f1_dn1 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), (((((((((var_if0_dn2 * var_evb2e1) + (var_if0 * var_evb2e1_dn2)) * var_vtinv) + (assign6330_e6489 * var_vtinv_dn2)) * var_nff_t) - (assign6330_e6491 * var_nff_t_dn2)) / (var_nff_t * var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (var_f1_dn2 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), (((((((((var_if0_dn3 * var_evb2e1) + (var_if0 * var_evb2e1_dn3)) * var_vtinv) + (assign6330_e6489 * var_vtinv_dn3)) * var_nff_t) - (assign6330_e6491 * var_nff_t_dn3)) / (var_nff_t * var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (var_f1_dn3 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), (((((((((var_if0_dn4 * var_evb2e1) + (var_if0 * var_evb2e1_dn4)) * var_vtinv) + (assign6330_e6489 * var_vtinv_dn4)) * var_nff_t) - (assign6330_e6491 * var_nff_t_dn4)) / (var_nff_t * var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (var_f1_dn4 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), (((((((((var_if0_dn5 * var_evb2e1) + (var_if0 * var_evb2e1_dn5)) * var_vtinv) + (assign6330_e6489 * var_vtinv_dn5)) * var_nff_t) - (assign6330_e6491 * var_nff_t_dn5)) / (var_nff_t * var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (var_f1_dn5 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), (((((((((var_if0_dn6 * var_evb2e1) + (var_if0 * var_evb2e1_dn6)) * var_vtinv) + (assign6330_e6489 * var_vtinv_dn6)) * var_nff_t) - (assign6330_e6491 * var_nff_t_dn6)) / (var_nff_t * var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (var_f1_dn6 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), (((((((((var_if0_dn7 * var_evb2e1) + (var_if0 * var_evb2e1_dn7)) * var_vtinv) + (assign6330_e6489 * var_vtinv_dn7)) * var_nff_t) - (assign6330_e6491 * var_nff_t_dn7)) / (var_nff_t * var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (var_f1_dn7 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), (((((((((var_if0_dn8 * var_evb2e1) + (var_if0 * var_evb2e1_dn8)) * var_vtinv) + (assign6330_e6489 * var_vtinv_dn8)) * var_nff_t) - (assign6330_e6491 * var_nff_t_dn8)) / (var_nff_t * var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (var_f1_dn8 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), (((((((((var_if0_dn9 * var_evb2e1) + (var_if0 * var_evb2e1_dn9)) * var_vtinv) + (assign6330_e6489 * var_vtinv_dn9)) * var_nff_t) - (assign6330_e6491 * var_nff_t_dn9)) / (var_nff_t * var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (var_f1_dn9 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), (((((((((var_if0_dn10 * var_evb2e1) + (var_if0 * var_evb2e1_dn10)) * var_vtinv) + (assign6330_e6489 * var_vtinv_dn10)) * var_nff_t) - (assign6330_e6491 * var_nff_t_dn10)) / (var_nff_t * var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (var_f1_dn10 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), (((((((((var_if0_dn11 * var_evb2e1) + (var_if0 * var_evb2e1_dn11)) * var_vtinv) + (assign6330_e6489 * var_vtinv_dn11)) * var_nff_t) - (assign6330_e6491 * var_nff_t_dn11)) / (var_nff_t * var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (var_f1_dn11 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), (((((((((var_if0_db0 * var_evb2e1) + (var_if0 * var_evb2e1_db0)) * var_vtinv) + (assign6330_e6489 * var_vtinv_db0)) * var_nff_t) - (assign6330_e6491 * var_nff_t_db0)) / (var_nff_t * var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (var_f1_db0 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))), (((((((((var_if0_db1 * var_evb2e1) + (var_if0 * var_evb2e1_db1)) * var_vtinv) + (assign6330_e6489 * var_vtinv_db1)) * var_nff_t) - (assign6330_e6491 * var_nff_t_db1)) / (var_nff_t * var_nff_t)) * assign6330_e6499) + (assign6330_e6493 * (-((0.5 * (var_f1_db1 / (2.0 * assign6330_e6498))) / (assign6330_e6498 * assign6330_e6498))))),)
    } else {
        (var_dn0vb2e1, var_dn0vb2e1_dn0, var_dn0vb2e1_dn1, var_dn0vb2e1_dn2, var_dn0vb2e1_dn3, var_dn0vb2e1_dn4, var_dn0vb2e1_dn5, var_dn0vb2e1_dn6, var_dn0vb2e1_dn7, var_dn0vb2e1_dn8, var_dn0vb2e1_dn9, var_dn0vb2e1_dn10, var_dn0vb2e1_dn11, var_dn0vb2e1_db0, var_dn0vb2e1_db1,)
    }
};
        var_dn0vb2e1 = assign6330_e6502;
        var_dn0vb2e1_dn0 = assign6330_e6502_d_n0;
        var_dn0vb2e1_dn1 = assign6330_e6502_d_n1;
        var_dn0vb2e1_dn2 = assign6330_e6502_d_n2;
        var_dn0vb2e1_dn3 = assign6330_e6502_d_n3;
        var_dn0vb2e1_dn4 = assign6330_e6502_d_n4;
        var_dn0vb2e1_dn5 = assign6330_e6502_d_n5;
        var_dn0vb2e1_dn6 = assign6330_e6502_d_n6;
        var_dn0vb2e1_dn7 = assign6330_e6502_d_n7;
        var_dn0vb2e1_dn8 = assign6330_e6502_d_n8;
        var_dn0vb2e1_dn9 = assign6330_e6502_d_n9;
        var_dn0vb2e1_dn10 = assign6330_e6502_d_n10;
        var_dn0vb2e1_dn11 = assign6330_e6502_d_n11;
        var_dn0vb2e1_db0 = assign6330_e6502_d_b0;
        var_dn0vb2e1_db1 = assign6330_e6502_d_b1;
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
        var_dn0vb2e1_rdn11 = 0.0;
        var_dn0vb2e1_rdb0 = 0.0;
        var_dn0vb2e1_rdb1 = 0.0;

        let (assign6340_e6512, assign6340_e6512_d_n0, assign6340_e6512_d_n1, assign6340_e6512_d_n2, assign6340_e6512_d_n3, assign6340_e6512_d_n4, assign6340_e6512_d_n5, assign6340_e6512_d_n6, assign6340_e6512_d_n7, assign6340_e6512_d_n8, assign6340_e6512_d_n9, assign6340_e6512_d_n10, assign6340_e6512_d_n11, assign6340_e6512_d_b0, assign6340_e6512_d_b1,) = {
    if (var_guard115 != 0.0) {
        let assign6340_e6506: f64 = (0.5 * var_qb0);
        let assign6340_e6508: f64 = (assign6340_e6506 * var_q1q);
        let assign6340_e6510: f64 = (assign6340_e6508 * var_dn0vb2e1);
        (assign6340_e6510, (((((0.5 * var_qb0_dn0) * var_q1q) + (assign6340_e6506 * var_q1q_dn0)) * var_dn0vb2e1) + (assign6340_e6508 * var_dn0vb2e1_dn0)), (((((0.5 * var_qb0_dn1) * var_q1q) + (assign6340_e6506 * var_q1q_dn1)) * var_dn0vb2e1) + (assign6340_e6508 * var_dn0vb2e1_dn1)), (((((0.5 * var_qb0_dn2) * var_q1q) + (assign6340_e6506 * var_q1q_dn2)) * var_dn0vb2e1) + (assign6340_e6508 * var_dn0vb2e1_dn2)), (((((0.5 * var_qb0_dn3) * var_q1q) + (assign6340_e6506 * var_q1q_dn3)) * var_dn0vb2e1) + (assign6340_e6508 * var_dn0vb2e1_dn3)), (((((0.5 * var_qb0_dn4) * var_q1q) + (assign6340_e6506 * var_q1q_dn4)) * var_dn0vb2e1) + (assign6340_e6508 * var_dn0vb2e1_dn4)), (((((0.5 * var_qb0_dn5) * var_q1q) + (assign6340_e6506 * var_q1q_dn5)) * var_dn0vb2e1) + (assign6340_e6508 * var_dn0vb2e1_dn5)), (((((0.5 * var_qb0_dn6) * var_q1q) + (assign6340_e6506 * var_q1q_dn6)) * var_dn0vb2e1) + (assign6340_e6508 * var_dn0vb2e1_dn6)), (((((0.5 * var_qb0_dn7) * var_q1q) + (assign6340_e6506 * var_q1q_dn7)) * var_dn0vb2e1) + (assign6340_e6508 * var_dn0vb2e1_dn7)), (((((0.5 * var_qb0_dn8) * var_q1q) + (assign6340_e6506 * var_q1q_dn8)) * var_dn0vb2e1) + (assign6340_e6508 * var_dn0vb2e1_dn8)), (((((0.5 * var_qb0_dn9) * var_q1q) + (assign6340_e6506 * var_q1q_dn9)) * var_dn0vb2e1) + (assign6340_e6508 * var_dn0vb2e1_dn9)), (((((0.5 * var_qb0_dn10) * var_q1q) + (assign6340_e6506 * var_q1q_dn10)) * var_dn0vb2e1) + (assign6340_e6508 * var_dn0vb2e1_dn10)), (((((0.5 * var_qb0_dn11) * var_q1q) + (assign6340_e6506 * var_q1q_dn11)) * var_dn0vb2e1) + (assign6340_e6508 * var_dn0vb2e1_dn11)), (((((0.5 * var_qb0_db0) * var_q1q) + (assign6340_e6506 * var_q1q_db0)) * var_dn0vb2e1) + (assign6340_e6508 * var_dn0vb2e1_db0)), (((((0.5 * var_qb0_db1) * var_q1q) + (assign6340_e6506 * var_q1q_db1)) * var_dn0vb2e1) + (assign6340_e6508 * var_dn0vb2e1_db1)),)
    } else {
        (var_dqbevb2e1, var_dqbevb2e1_dn0, var_dqbevb2e1_dn1, var_dqbevb2e1_dn2, var_dqbevb2e1_dn3, var_dqbevb2e1_dn4, var_dqbevb2e1_dn5, var_dqbevb2e1_dn6, var_dqbevb2e1_dn7, var_dqbevb2e1_dn8, var_dqbevb2e1_dn9, var_dqbevb2e1_dn10, var_dqbevb2e1_dn11, var_dqbevb2e1_db0, var_dqbevb2e1_db1,)
    }
};
        var_dqbevb2e1 = assign6340_e6512;
        var_dqbevb2e1_dn0 = assign6340_e6512_d_n0;
        var_dqbevb2e1_dn1 = assign6340_e6512_d_n1;
        var_dqbevb2e1_dn2 = assign6340_e6512_d_n2;
        var_dqbevb2e1_dn3 = assign6340_e6512_d_n3;
        var_dqbevb2e1_dn4 = assign6340_e6512_d_n4;
        var_dqbevb2e1_dn5 = assign6340_e6512_d_n5;
        var_dqbevb2e1_dn6 = assign6340_e6512_d_n6;
        var_dqbevb2e1_dn7 = assign6340_e6512_d_n7;
        var_dqbevb2e1_dn8 = assign6340_e6512_d_n8;
        var_dqbevb2e1_dn9 = assign6340_e6512_d_n9;
        var_dqbevb2e1_dn10 = assign6340_e6512_d_n10;
        var_dqbevb2e1_dn11 = assign6340_e6512_d_n11;
        var_dqbevb2e1_db0 = assign6340_e6512_d_b0;
        var_dqbevb2e1_db1 = assign6340_e6512_d_b1;
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
        var_dqbevb2e1_rdn11 = 0.0;
        var_dqbevb2e1_rdb0 = 0.0;
        var_dqbevb2e1_rdb1 = 0.0;

        let (assign6350_e6520, assign6350_e6520_d_n0, assign6350_e6520_d_n1, assign6350_e6520_d_n2, assign6350_e6520_d_n3, assign6350_e6520_d_n4, assign6350_e6520_d_n5, assign6350_e6520_d_n6, assign6350_e6520_d_n7, assign6350_e6520_d_n8, assign6350_e6520_d_n9, assign6350_e6520_d_n10, assign6350_e6520_d_n11, assign6350_e6520_d_b0, assign6350_e6520_d_b1,) = {
    if (var_guard115 != 0.0) {
        let assign6350_e6517: f64 = (p.p84 * var_vt);
        let assign6350_e6518: f64 = (var_qe_qs / assign6350_e6517);
        (assign6350_e6518, (((var_qe_qs_dn0 * assign6350_e6517) - (var_qe_qs * (p.p84 * var_vt_dn0))) / (assign6350_e6517 * assign6350_e6517)), (((var_qe_qs_dn1 * assign6350_e6517) - (var_qe_qs * (p.p84 * var_vt_dn1))) / (assign6350_e6517 * assign6350_e6517)), (((var_qe_qs_dn2 * assign6350_e6517) - (var_qe_qs * (p.p84 * var_vt_dn2))) / (assign6350_e6517 * assign6350_e6517)), (((var_qe_qs_dn3 * assign6350_e6517) - (var_qe_qs * (p.p84 * var_vt_dn3))) / (assign6350_e6517 * assign6350_e6517)), (((var_qe_qs_dn4 * assign6350_e6517) - (var_qe_qs * (p.p84 * var_vt_dn4))) / (assign6350_e6517 * assign6350_e6517)), (((var_qe_qs_dn5 * assign6350_e6517) - (var_qe_qs * (p.p84 * var_vt_dn5))) / (assign6350_e6517 * assign6350_e6517)), (((var_qe_qs_dn6 * assign6350_e6517) - (var_qe_qs * (p.p84 * var_vt_dn6))) / (assign6350_e6517 * assign6350_e6517)), (((var_qe_qs_dn7 * assign6350_e6517) - (var_qe_qs * (p.p84 * var_vt_dn7))) / (assign6350_e6517 * assign6350_e6517)), (((var_qe_qs_dn8 * assign6350_e6517) - (var_qe_qs * (p.p84 * var_vt_dn8))) / (assign6350_e6517 * assign6350_e6517)), (((var_qe_qs_dn9 * assign6350_e6517) - (var_qe_qs * (p.p84 * var_vt_dn9))) / (assign6350_e6517 * assign6350_e6517)), (((var_qe_qs_dn10 * assign6350_e6517) - (var_qe_qs * (p.p84 * var_vt_dn10))) / (assign6350_e6517 * assign6350_e6517)), (((var_qe_qs_dn11 * assign6350_e6517) - (var_qe_qs * (p.p84 * var_vt_dn11))) / (assign6350_e6517 * assign6350_e6517)), (((var_qe_qs_db0 * assign6350_e6517) - (var_qe_qs * (p.p84 * var_vt_db0))) / (assign6350_e6517 * assign6350_e6517)), (((var_qe_qs_db1 * assign6350_e6517) - (var_qe_qs * (p.p84 * var_vt_db1))) / (assign6350_e6517 * assign6350_e6517)),)
    } else {
        (var_dqevb2e1, var_dqevb2e1_dn0, var_dqevb2e1_dn1, var_dqevb2e1_dn2, var_dqevb2e1_dn3, var_dqevb2e1_dn4, var_dqevb2e1_dn5, var_dqevb2e1_dn6, var_dqevb2e1_dn7, var_dqevb2e1_dn8, var_dqevb2e1_dn9, var_dqevb2e1_dn10, var_dqevb2e1_dn11, var_dqevb2e1_db0, var_dqevb2e1_db1,)
    }
};
        var_dqevb2e1 = assign6350_e6520;
        var_dqevb2e1_dn0 = assign6350_e6520_d_n0;
        var_dqevb2e1_dn1 = assign6350_e6520_d_n1;
        var_dqevb2e1_dn2 = assign6350_e6520_d_n2;
        var_dqevb2e1_dn3 = assign6350_e6520_d_n3;
        var_dqevb2e1_dn4 = assign6350_e6520_d_n4;
        var_dqevb2e1_dn5 = assign6350_e6520_d_n5;
        var_dqevb2e1_dn6 = assign6350_e6520_d_n6;
        var_dqevb2e1_dn7 = assign6350_e6520_d_n7;
        var_dqevb2e1_dn8 = assign6350_e6520_d_n8;
        var_dqevb2e1_dn9 = assign6350_e6520_d_n9;
        var_dqevb2e1_dn10 = assign6350_e6520_d_n10;
        var_dqevb2e1_dn11 = assign6350_e6520_d_n11;
        var_dqevb2e1_db0 = assign6350_e6520_d_b0;
        var_dqevb2e1_db1 = assign6350_e6520_d_b1;
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
        var_dqevb2e1_rdn11 = 0.0;
        var_dqevb2e1_rdb0 = 0.0;
        var_dqevb2e1_rdb1 = 0.0;

        let (assign6360_e6532, assign6360_e6532_d_n0, assign6360_e6532_d_n1, assign6360_e6532_d_n2, assign6360_e6532_d_n3, assign6360_e6532_d_n4, assign6360_e6532_d_n5, assign6360_e6532_d_n6, assign6360_e6532_d_n7, assign6360_e6532_d_n8, assign6360_e6532_d_n9, assign6360_e6532_d_n10, assign6360_e6532_d_n11, assign6360_e6532_d_b0, assign6360_e6532_d_b1,) = {
    if (var_guard115 != 0.0) {
        let assign6360_e6524: f64 = (0.2 * var_vb1b2);
        let assign6360_e6527: f64 = (var_dqtevb2e1 + var_dqbevb2e1);
        let assign6360_e6529: f64 = (assign6360_e6527 + var_dqevb2e1);
        let assign6360_e6530: f64 = (assign6360_e6524 * assign6360_e6529);
        (assign6360_e6530, (((0.2 * var_vb1b2_dn0) * assign6360_e6529) + (assign6360_e6524 * ((var_dqtevb2e1_dn0 + var_dqbevb2e1_dn0) + var_dqevb2e1_dn0))), (((0.2 * var_vb1b2_dn1) * assign6360_e6529) + (assign6360_e6524 * ((var_dqtevb2e1_dn1 + var_dqbevb2e1_dn1) + var_dqevb2e1_dn1))), (((0.2 * var_vb1b2_dn2) * assign6360_e6529) + (assign6360_e6524 * ((var_dqtevb2e1_dn2 + var_dqbevb2e1_dn2) + var_dqevb2e1_dn2))), (((0.2 * var_vb1b2_dn3) * assign6360_e6529) + (assign6360_e6524 * ((var_dqtevb2e1_dn3 + var_dqbevb2e1_dn3) + var_dqevb2e1_dn3))), (((0.2 * var_vb1b2_dn4) * assign6360_e6529) + (assign6360_e6524 * ((var_dqtevb2e1_dn4 + var_dqbevb2e1_dn4) + var_dqevb2e1_dn4))), (((0.2 * var_vb1b2_dn5) * assign6360_e6529) + (assign6360_e6524 * ((var_dqtevb2e1_dn5 + var_dqbevb2e1_dn5) + var_dqevb2e1_dn5))), (((0.2 * var_vb1b2_dn6) * assign6360_e6529) + (assign6360_e6524 * ((var_dqtevb2e1_dn6 + var_dqbevb2e1_dn6) + var_dqevb2e1_dn6))), (((0.2 * var_vb1b2_dn7) * assign6360_e6529) + (assign6360_e6524 * ((var_dqtevb2e1_dn7 + var_dqbevb2e1_dn7) + var_dqevb2e1_dn7))), (((0.2 * var_vb1b2_dn8) * assign6360_e6529) + (assign6360_e6524 * ((var_dqtevb2e1_dn8 + var_dqbevb2e1_dn8) + var_dqevb2e1_dn8))), (((0.2 * var_vb1b2_dn9) * assign6360_e6529) + (assign6360_e6524 * ((var_dqtevb2e1_dn9 + var_dqbevb2e1_dn9) + var_dqevb2e1_dn9))), (((0.2 * var_vb1b2_dn10) * assign6360_e6529) + (assign6360_e6524 * ((var_dqtevb2e1_dn10 + var_dqbevb2e1_dn10) + var_dqevb2e1_dn10))), (((0.2 * var_vb1b2_dn11) * assign6360_e6529) + (assign6360_e6524 * ((var_dqtevb2e1_dn11 + var_dqbevb2e1_dn11) + var_dqevb2e1_dn11))), (((0.2 * var_vb1b2_db0) * assign6360_e6529) + (assign6360_e6524 * ((var_dqtevb2e1_db0 + var_dqbevb2e1_db0) + var_dqevb2e1_db0))), (((0.2 * var_vb1b2_db1) * assign6360_e6529) + (assign6360_e6524 * ((var_dqtevb2e1_db1 + var_dqbevb2e1_db1) + var_dqevb2e1_db1))),)
    } else {
        (var_qb1b2, var_qb1b2_dn0, var_qb1b2_dn1, var_qb1b2_dn2, var_qb1b2_dn3, var_qb1b2_dn4, var_qb1b2_dn5, var_qb1b2_dn6, var_qb1b2_dn7, var_qb1b2_dn8, var_qb1b2_dn9, var_qb1b2_dn10, var_qb1b2_dn11, var_qb1b2_db0, var_qb1b2_db1,)
    }
};
        var_qb1b2 = assign6360_e6532;
        var_qb1b2_dn0 = assign6360_e6532_d_n0;
        var_qb1b2_dn1 = assign6360_e6532_d_n1;
        var_qb1b2_dn2 = assign6360_e6532_d_n2;
        var_qb1b2_dn3 = assign6360_e6532_d_n3;
        var_qb1b2_dn4 = assign6360_e6532_d_n4;
        var_qb1b2_dn5 = assign6360_e6532_d_n5;
        var_qb1b2_dn6 = assign6360_e6532_d_n6;
        var_qb1b2_dn7 = assign6360_e6532_d_n7;
        var_qb1b2_dn8 = assign6360_e6532_d_n8;
        var_qb1b2_dn9 = assign6360_e6532_d_n9;
        var_qb1b2_dn10 = assign6360_e6532_d_n10;
        var_qb1b2_dn11 = assign6360_e6532_d_n11;
        var_qb1b2_db0 = assign6360_e6532_d_b0;
        var_qb1b2_db1 = assign6360_e6532_d_b1;
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
        var_qb1b2_rdn11 = 0.0;
        var_qb1b2_rdb0 = 0.0;
        var_qb1b2_rdb1 = 0.0;

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
        *var_dn0vb2e1_rdb0_slot = var_dn0vb2e1_rdb0;
        *var_dn0vb2e1_rdb1_slot = var_dn0vb2e1_rdb1;
        *var_dn0vb2e1_rdn0_slot = var_dn0vb2e1_rdn0;
        *var_dn0vb2e1_rdn1_slot = var_dn0vb2e1_rdn1;
        *var_dn0vb2e1_rdn10_slot = var_dn0vb2e1_rdn10;
        *var_dn0vb2e1_rdn11_slot = var_dn0vb2e1_rdn11;
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
        *var_dqbevb2e1_dn11_slot = var_dqbevb2e1_dn11;
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
        *var_dqbevb2e1_rdn11_slot = var_dqbevb2e1_rdn11;
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
        *var_dqevb2e1_dn11_slot = var_dqevb2e1_dn11;
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
        *var_dqevb2e1_rdn11_slot = var_dqevb2e1_rdn11;
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
        *var_dqtevb2e1_dn11_slot = var_dqtevb2e1_dn11;
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
        *var_dqtevb2e1_rdn11_slot = var_dqtevb2e1_rdn11;
        *var_dqtevb2e1_rdn2_slot = var_dqtevb2e1_rdn2;
        *var_dqtevb2e1_rdn3_slot = var_dqtevb2e1_rdn3;
        *var_dqtevb2e1_rdn4_slot = var_dqtevb2e1_rdn4;
        *var_dqtevb2e1_rdn5_slot = var_dqtevb2e1_rdn5;
        *var_dqtevb2e1_rdn6_slot = var_dqtevb2e1_rdn6;
        *var_dqtevb2e1_rdn7_slot = var_dqtevb2e1_rdn7;
        *var_dqtevb2e1_rdn8_slot = var_dqtevb2e1_rdn8;
        *var_dqtevb2e1_rdn9_slot = var_dqtevb2e1_rdn9;
        *var_dqtevb2e1_rv_slot = var_dqtevb2e1_rv;
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
        *var_dvjevb2e1_rdb0_slot = var_dvjevb2e1_rdb0;
        *var_dvjevb2e1_rdb1_slot = var_dvjevb2e1_rdb1;
        *var_dvjevb2e1_rdn0_slot = var_dvjevb2e1_rdn0;
        *var_dvjevb2e1_rdn1_slot = var_dvjevb2e1_rdn1;
        *var_dvjevb2e1_rdn10_slot = var_dvjevb2e1_rdn10;
        *var_dvjevb2e1_rdn11_slot = var_dvjevb2e1_rdn11;
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
        *var_dvtevb2e1_dn11_slot = var_dvtevb2e1_dn11;
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
        *var_dvtevb2e1_rdn11_slot = var_dvtevb2e1_rdn11;
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
        *var_dvtevje_dn11_slot = var_dvtevje_dn11;
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
        *var_dvtevje_rdn11_slot = var_dvtevje_rdn11;
        *var_dvtevje_rdn2_slot = var_dvtevje_rdn2;
        *var_dvtevje_rdn3_slot = var_dvtevje_rdn3;
        *var_dvtevje_rdn4_slot = var_dvtevje_rdn4;
        *var_dvtevje_rdn5_slot = var_dvtevje_rdn5;
        *var_dvtevje_rdn6_slot = var_dvtevje_rdn6;
        *var_dvtevje_rdn7_slot = var_dvtevje_rdn7;
        *var_dvtevje_rdn8_slot = var_dvtevje_rdn8;
        *var_dvtevje_rdn9_slot = var_dvtevje_rdn9;
        *var_dvtevje_rv_slot = var_dvtevje_rv;
        *var_guard115_slot = var_guard115;
        *var_guard115_db0_slot = var_guard115_db0;
        *var_guard115_db1_slot = var_guard115_db1;
        *var_guard115_dn0_slot = var_guard115_dn0;
        *var_guard115_dn1_slot = var_guard115_dn1;
        *var_guard115_dn10_slot = var_guard115_dn10;
        *var_guard115_dn11_slot = var_guard115_dn11;
        *var_guard115_dn2_slot = var_guard115_dn2;
        *var_guard115_dn3_slot = var_guard115_dn3;
        *var_guard115_dn4_slot = var_guard115_dn4;
        *var_guard115_dn5_slot = var_guard115_dn5;
        *var_guard115_dn6_slot = var_guard115_dn6;
        *var_guard115_dn7_slot = var_guard115_dn7;
        *var_guard115_dn8_slot = var_guard115_dn8;
        *var_guard115_dn9_slot = var_guard115_dn9;
        *var_guard115_rdb0_slot = var_guard115_rdb0;
        *var_guard115_rdb1_slot = var_guard115_rdb1;
        *var_guard115_rdn0_slot = var_guard115_rdn0;
        *var_guard115_rdn1_slot = var_guard115_rdn1;
        *var_guard115_rdn10_slot = var_guard115_rdn10;
        *var_guard115_rdn11_slot = var_guard115_rdn11;
        *var_guard115_rdn2_slot = var_guard115_rdn2;
        *var_guard115_rdn3_slot = var_guard115_rdn3;
        *var_guard115_rdn4_slot = var_guard115_rdn4;
        *var_guard115_rdn5_slot = var_guard115_rdn5;
        *var_guard115_rdn6_slot = var_guard115_rdn6;
        *var_guard115_rdn7_slot = var_guard115_rdn7;
        *var_guard115_rdn8_slot = var_guard115_rdn8;
        *var_guard115_rdn9_slot = var_guard115_rdn9;
        *var_guard115_rv_slot = var_guard115_rv;
        *var_guard116_slot = var_guard116;
        *var_guard116_db0_slot = var_guard116_db0;
        *var_guard116_db1_slot = var_guard116_db1;
        *var_guard116_dn0_slot = var_guard116_dn0;
        *var_guard116_dn1_slot = var_guard116_dn1;
        *var_guard116_dn10_slot = var_guard116_dn10;
        *var_guard116_dn11_slot = var_guard116_dn11;
        *var_guard116_dn2_slot = var_guard116_dn2;
        *var_guard116_dn3_slot = var_guard116_dn3;
        *var_guard116_dn4_slot = var_guard116_dn4;
        *var_guard116_dn5_slot = var_guard116_dn5;
        *var_guard116_dn6_slot = var_guard116_dn6;
        *var_guard116_dn7_slot = var_guard116_dn7;
        *var_guard116_dn8_slot = var_guard116_dn8;
        *var_guard116_dn9_slot = var_guard116_dn9;
        *var_guard116_rdb0_slot = var_guard116_rdb0;
        *var_guard116_rdb1_slot = var_guard116_rdb1;
        *var_guard116_rdn0_slot = var_guard116_rdn0;
        *var_guard116_rdn1_slot = var_guard116_rdn1;
        *var_guard116_rdn10_slot = var_guard116_rdn10;
        *var_guard116_rdn11_slot = var_guard116_rdn11;
        *var_guard116_rdn2_slot = var_guard116_rdn2;
        *var_guard116_rdn3_slot = var_guard116_rdn3;
        *var_guard116_rdn4_slot = var_guard116_rdn4;
        *var_guard116_rdn5_slot = var_guard116_rdn5;
        *var_guard116_rdn6_slot = var_guard116_rdn6;
        *var_guard116_rdn7_slot = var_guard116_rdn7;
        *var_guard116_rdn8_slot = var_guard116_rdn8;
        *var_guard116_rdn9_slot = var_guard116_rdn9;
        *var_guard116_rv_slot = var_guard116_rv;
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
        *var_qb1b2_rdb0_slot = var_qb1b2_rdb0;
        *var_qb1b2_rdb1_slot = var_qb1b2_rdb1;
        *var_qb1b2_rdn0_slot = var_qb1b2_rdn0;
        *var_qb1b2_rdn1_slot = var_qb1b2_rdn1;
        *var_qb1b2_rdn10_slot = var_qb1b2_rdn10;
        *var_qb1b2_rdn11_slot = var_qb1b2_rdn11;
        *var_qb1b2_rdn2_slot = var_qb1b2_rdn2;
        *var_qb1b2_rdn3_slot = var_qb1b2_rdn3;
        *var_qb1b2_rdn4_slot = var_qb1b2_rdn4;
        *var_qb1b2_rdn5_slot = var_qb1b2_rdn5;
        *var_qb1b2_rdn6_slot = var_qb1b2_rdn6;
        *var_qb1b2_rdn7_slot = var_qb1b2_rdn7;
        *var_qb1b2_rdn8_slot = var_qb1b2_rdn8;
        *var_qb1b2_rdn9_slot = var_qb1b2_rdn9;
        *var_qb1b2_rv_slot = var_qb1b2_rv;
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
        *var_vb2e1vfe_rdb0_slot = var_vb2e1vfe_rdb0;
        *var_vb2e1vfe_rdb1_slot = var_vb2e1vfe_rdb1;
        *var_vb2e1vfe_rdn0_slot = var_vb2e1vfe_rdn0;
        *var_vb2e1vfe_rdn1_slot = var_vb2e1vfe_rdn1;
        *var_vb2e1vfe_rdn10_slot = var_vb2e1vfe_rdn10;
        *var_vb2e1vfe_rdn11_slot = var_vb2e1vfe_rdn11;
        *var_vb2e1vfe_rdn2_slot = var_vb2e1vfe_rdn2;
        *var_vb2e1vfe_rdn3_slot = var_vb2e1vfe_rdn3;
        *var_vb2e1vfe_rdn4_slot = var_vb2e1vfe_rdn4;
        *var_vb2e1vfe_rdn5_slot = var_vb2e1vfe_rdn5;
        *var_vb2e1vfe_rdn6_slot = var_vb2e1vfe_rdn6;
        *var_vb2e1vfe_rdn7_slot = var_vb2e1vfe_rdn7;
        *var_vb2e1vfe_rdn8_slot = var_vb2e1vfe_rdn8;
        *var_vb2e1vfe_rdn9_slot = var_vb2e1vfe_rdn9;
        *var_vb2e1vfe_rv_slot = var_vb2e1vfe_rv;
    }

    pub(super) fn stamp_reactive_block_40(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_guard115: f64,
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
        var_taub_t_db0: f64,
        var_taub_t_db1: f64,
        var_taub_t_dn0: f64,
        var_taub_t_dn1: f64,
        var_taub_t_dn10: f64,
        var_taub_t_dn11: f64,
        var_taub_t_dn2: f64,
        var_taub_t_dn3: f64,
        var_taub_t_dn4: f64,
        var_taub_t_dn5: f64,
        var_taub_t_dn6: f64,
        var_taub_t_dn7: f64,
        var_taub_t_dn8: f64,
        var_taub_t_dn9: f64,
        var_guard124_slot: &mut f64,
        var_guard124_db0_slot: &mut f64,
        var_guard124_db1_slot: &mut f64,
        var_guard124_dn0_slot: &mut f64,
        var_guard124_dn1_slot: &mut f64,
        var_guard124_dn10_slot: &mut f64,
        var_guard124_dn11_slot: &mut f64,
        var_guard124_dn2_slot: &mut f64,
        var_guard124_dn3_slot: &mut f64,
        var_guard124_dn4_slot: &mut f64,
        var_guard124_dn5_slot: &mut f64,
        var_guard124_dn6_slot: &mut f64,
        var_guard124_dn7_slot: &mut f64,
        var_guard124_dn8_slot: &mut f64,
        var_guard124_dn9_slot: &mut f64,
        var_guard124_rdb0_slot: &mut f64,
        var_guard124_rdb1_slot: &mut f64,
        var_guard124_rdn0_slot: &mut f64,
        var_guard124_rdn1_slot: &mut f64,
        var_guard124_rdn10_slot: &mut f64,
        var_guard124_rdn11_slot: &mut f64,
        var_guard124_rdn2_slot: &mut f64,
        var_guard124_rdn3_slot: &mut f64,
        var_guard124_rdn4_slot: &mut f64,
        var_guard124_rdn5_slot: &mut f64,
        var_guard124_rdn6_slot: &mut f64,
        var_guard124_rdn7_slot: &mut f64,
        var_guard124_rdn8_slot: &mut f64,
        var_guard124_rdn9_slot: &mut f64,
        var_guard124_rv_slot: &mut f64,
        var_guard125_slot: &mut f64,
        var_guard125_db0_slot: &mut f64,
        var_guard125_db1_slot: &mut f64,
        var_guard125_dn0_slot: &mut f64,
        var_guard125_dn1_slot: &mut f64,
        var_guard125_dn10_slot: &mut f64,
        var_guard125_dn11_slot: &mut f64,
        var_guard125_dn2_slot: &mut f64,
        var_guard125_dn3_slot: &mut f64,
        var_guard125_dn4_slot: &mut f64,
        var_guard125_dn5_slot: &mut f64,
        var_guard125_dn6_slot: &mut f64,
        var_guard125_dn7_slot: &mut f64,
        var_guard125_dn8_slot: &mut f64,
        var_guard125_dn9_slot: &mut f64,
        var_guard125_rdb0_slot: &mut f64,
        var_guard125_rdb1_slot: &mut f64,
        var_guard125_rdn0_slot: &mut f64,
        var_guard125_rdn1_slot: &mut f64,
        var_guard125_rdn10_slot: &mut f64,
        var_guard125_rdn11_slot: &mut f64,
        var_guard125_rdn2_slot: &mut f64,
        var_guard125_rdn3_slot: &mut f64,
        var_guard125_rdn4_slot: &mut f64,
        var_guard125_rdn5_slot: &mut f64,
        var_guard125_rdn6_slot: &mut f64,
        var_guard125_rdn7_slot: &mut f64,
        var_guard125_rdn8_slot: &mut f64,
        var_guard125_rdn9_slot: &mut f64,
        var_guard125_rv_slot: &mut f64,
        var_i_cth_slot: &mut f64,
        var_i_cth_db0_slot: &mut f64,
        var_i_cth_db1_slot: &mut f64,
        var_i_cth_dn0_slot: &mut f64,
        var_i_cth_dn1_slot: &mut f64,
        var_i_cth_dn10_slot: &mut f64,
        var_i_cth_dn11_slot: &mut f64,
        var_i_cth_dn2_slot: &mut f64,
        var_i_cth_dn3_slot: &mut f64,
        var_i_cth_dn4_slot: &mut f64,
        var_i_cth_dn5_slot: &mut f64,
        var_i_cth_dn6_slot: &mut f64,
        var_i_cth_dn7_slot: &mut f64,
        var_i_cth_dn8_slot: &mut f64,
        var_i_cth_dn9_slot: &mut f64,
        var_i_cth_rdb0_slot: &mut f64,
        var_i_cth_rdb1_slot: &mut f64,
        var_i_cth_rdn0_slot: &mut f64,
        var_i_cth_rdn1_slot: &mut f64,
        var_i_cth_rdn10_slot: &mut f64,
        var_i_cth_rdn11_slot: &mut f64,
        var_i_cth_rdn2_slot: &mut f64,
        var_i_cth_rdn3_slot: &mut f64,
        var_i_cth_rdn4_slot: &mut f64,
        var_i_cth_rdn5_slot: &mut f64,
        var_i_cth_rdn6_slot: &mut f64,
        var_i_cth_rdn7_slot: &mut f64,
        var_i_cth_rdn8_slot: &mut f64,
        var_i_cth_rdn9_slot: &mut f64,
        var_i_cth_rv_slot: &mut f64,
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
        var_in_n_rdb0_slot: &mut f64,
        var_in_n_rdb1_slot: &mut f64,
        var_in_n_rdn0_slot: &mut f64,
        var_in_n_rdn1_slot: &mut f64,
        var_in_n_rdn10_slot: &mut f64,
        var_in_n_rdn11_slot: &mut f64,
        var_in_n_rdn2_slot: &mut f64,
        var_in_n_rdn3_slot: &mut f64,
        var_in_n_rdn4_slot: &mut f64,
        var_in_n_rdn5_slot: &mut f64,
        var_in_n_rdn6_slot: &mut f64,
        var_in_n_rdn7_slot: &mut f64,
        var_in_n_rdn8_slot: &mut f64,
        var_in_n_rdn9_slot: &mut f64,
        var_in_n_rv_slot: &mut f64,
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
        var_qbc_rdb0_slot: &mut f64,
        var_qbc_rdb1_slot: &mut f64,
        var_qbc_rdn0_slot: &mut f64,
        var_qbc_rdn1_slot: &mut f64,
        var_qbc_rdn10_slot: &mut f64,
        var_qbc_rdn11_slot: &mut f64,
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
        var_qbe_qs_eff_rdb0_slot: &mut f64,
        var_qbe_qs_eff_rdb1_slot: &mut f64,
        var_qbe_qs_eff_rdn0_slot: &mut f64,
        var_qbe_qs_eff_rdn1_slot: &mut f64,
        var_qbe_qs_eff_rdn10_slot: &mut f64,
        var_qbe_qs_eff_rdn11_slot: &mut f64,
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
        var_qbe_rdn11_slot: &mut f64,
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
        var_qe_dn11_slot: &mut f64,
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
        var_qe_rdn11_slot: &mut f64,
        var_qe_rdn2_slot: &mut f64,
        var_qe_rdn3_slot: &mut f64,
        var_qe_rdn4_slot: &mut f64,
        var_qe_rdn5_slot: &mut f64,
        var_qe_rdn6_slot: &mut f64,
        var_qe_rdn7_slot: &mut f64,
        var_qe_rdn8_slot: &mut f64,
        var_qe_rdn9_slot: &mut f64,
        var_qe_rv_slot: &mut f64,
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
        var_taub_n_rdb0_slot: &mut f64,
        var_taub_n_rdb1_slot: &mut f64,
        var_taub_n_rdn0_slot: &mut f64,
        var_taub_n_rdn1_slot: &mut f64,
        var_taub_n_rdn10_slot: &mut f64,
        var_taub_n_rdn11_slot: &mut f64,
        var_taub_n_rdn2_slot: &mut f64,
        var_taub_n_rdn3_slot: &mut f64,
        var_taub_n_rdn4_slot: &mut f64,
        var_taub_n_rdn5_slot: &mut f64,
        var_taub_n_rdn6_slot: &mut f64,
        var_taub_n_rdn7_slot: &mut f64,
        var_taub_n_rdn8_slot: &mut f64,
        var_taub_n_rdn9_slot: &mut f64,
        var_taub_n_rv_slot: &mut f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let mut var_guard124: f64 = *var_guard124_slot;
        let mut var_guard124_db0: f64 = *var_guard124_db0_slot;
        let mut var_guard124_db1: f64 = *var_guard124_db1_slot;
        let mut var_guard124_dn0: f64 = *var_guard124_dn0_slot;
        let mut var_guard124_dn1: f64 = *var_guard124_dn1_slot;
        let mut var_guard124_dn10: f64 = *var_guard124_dn10_slot;
        let mut var_guard124_dn11: f64 = *var_guard124_dn11_slot;
        let mut var_guard124_dn2: f64 = *var_guard124_dn2_slot;
        let mut var_guard124_dn3: f64 = *var_guard124_dn3_slot;
        let mut var_guard124_dn4: f64 = *var_guard124_dn4_slot;
        let mut var_guard124_dn5: f64 = *var_guard124_dn5_slot;
        let mut var_guard124_dn6: f64 = *var_guard124_dn6_slot;
        let mut var_guard124_dn7: f64 = *var_guard124_dn7_slot;
        let mut var_guard124_dn8: f64 = *var_guard124_dn8_slot;
        let mut var_guard124_dn9: f64 = *var_guard124_dn9_slot;
        let mut var_guard124_rdb0: f64 = *var_guard124_rdb0_slot;
        let mut var_guard124_rdb1: f64 = *var_guard124_rdb1_slot;
        let mut var_guard124_rdn0: f64 = *var_guard124_rdn0_slot;
        let mut var_guard124_rdn1: f64 = *var_guard124_rdn1_slot;
        let mut var_guard124_rdn10: f64 = *var_guard124_rdn10_slot;
        let mut var_guard124_rdn11: f64 = *var_guard124_rdn11_slot;
        let mut var_guard124_rdn2: f64 = *var_guard124_rdn2_slot;
        let mut var_guard124_rdn3: f64 = *var_guard124_rdn3_slot;
        let mut var_guard124_rdn4: f64 = *var_guard124_rdn4_slot;
        let mut var_guard124_rdn5: f64 = *var_guard124_rdn5_slot;
        let mut var_guard124_rdn6: f64 = *var_guard124_rdn6_slot;
        let mut var_guard124_rdn7: f64 = *var_guard124_rdn7_slot;
        let mut var_guard124_rdn8: f64 = *var_guard124_rdn8_slot;
        let mut var_guard124_rdn9: f64 = *var_guard124_rdn9_slot;
        let mut var_guard124_rv: f64 = *var_guard124_rv_slot;
        let mut var_guard125: f64 = *var_guard125_slot;
        let mut var_guard125_db0: f64 = *var_guard125_db0_slot;
        let mut var_guard125_db1: f64 = *var_guard125_db1_slot;
        let mut var_guard125_dn0: f64 = *var_guard125_dn0_slot;
        let mut var_guard125_dn1: f64 = *var_guard125_dn1_slot;
        let mut var_guard125_dn10: f64 = *var_guard125_dn10_slot;
        let mut var_guard125_dn11: f64 = *var_guard125_dn11_slot;
        let mut var_guard125_dn2: f64 = *var_guard125_dn2_slot;
        let mut var_guard125_dn3: f64 = *var_guard125_dn3_slot;
        let mut var_guard125_dn4: f64 = *var_guard125_dn4_slot;
        let mut var_guard125_dn5: f64 = *var_guard125_dn5_slot;
        let mut var_guard125_dn6: f64 = *var_guard125_dn6_slot;
        let mut var_guard125_dn7: f64 = *var_guard125_dn7_slot;
        let mut var_guard125_dn8: f64 = *var_guard125_dn8_slot;
        let mut var_guard125_dn9: f64 = *var_guard125_dn9_slot;
        let mut var_guard125_rdb0: f64 = *var_guard125_rdb0_slot;
        let mut var_guard125_rdb1: f64 = *var_guard125_rdb1_slot;
        let mut var_guard125_rdn0: f64 = *var_guard125_rdn0_slot;
        let mut var_guard125_rdn1: f64 = *var_guard125_rdn1_slot;
        let mut var_guard125_rdn10: f64 = *var_guard125_rdn10_slot;
        let mut var_guard125_rdn11: f64 = *var_guard125_rdn11_slot;
        let mut var_guard125_rdn2: f64 = *var_guard125_rdn2_slot;
        let mut var_guard125_rdn3: f64 = *var_guard125_rdn3_slot;
        let mut var_guard125_rdn4: f64 = *var_guard125_rdn4_slot;
        let mut var_guard125_rdn5: f64 = *var_guard125_rdn5_slot;
        let mut var_guard125_rdn6: f64 = *var_guard125_rdn6_slot;
        let mut var_guard125_rdn7: f64 = *var_guard125_rdn7_slot;
        let mut var_guard125_rdn8: f64 = *var_guard125_rdn8_slot;
        let mut var_guard125_rdn9: f64 = *var_guard125_rdn9_slot;
        let mut var_guard125_rv: f64 = *var_guard125_rv_slot;
        let mut var_i_cth: f64 = *var_i_cth_slot;
        let mut var_i_cth_db0: f64 = *var_i_cth_db0_slot;
        let mut var_i_cth_db1: f64 = *var_i_cth_db1_slot;
        let mut var_i_cth_dn0: f64 = *var_i_cth_dn0_slot;
        let mut var_i_cth_dn1: f64 = *var_i_cth_dn1_slot;
        let mut var_i_cth_dn10: f64 = *var_i_cth_dn10_slot;
        let mut var_i_cth_dn11: f64 = *var_i_cth_dn11_slot;
        let mut var_i_cth_dn2: f64 = *var_i_cth_dn2_slot;
        let mut var_i_cth_dn3: f64 = *var_i_cth_dn3_slot;
        let mut var_i_cth_dn4: f64 = *var_i_cth_dn4_slot;
        let mut var_i_cth_dn5: f64 = *var_i_cth_dn5_slot;
        let mut var_i_cth_dn6: f64 = *var_i_cth_dn6_slot;
        let mut var_i_cth_dn7: f64 = *var_i_cth_dn7_slot;
        let mut var_i_cth_dn8: f64 = *var_i_cth_dn8_slot;
        let mut var_i_cth_dn9: f64 = *var_i_cth_dn9_slot;
        let mut var_i_cth_rdb0: f64 = *var_i_cth_rdb0_slot;
        let mut var_i_cth_rdb1: f64 = *var_i_cth_rdb1_slot;
        let mut var_i_cth_rdn0: f64 = *var_i_cth_rdn0_slot;
        let mut var_i_cth_rdn1: f64 = *var_i_cth_rdn1_slot;
        let mut var_i_cth_rdn10: f64 = *var_i_cth_rdn10_slot;
        let mut var_i_cth_rdn11: f64 = *var_i_cth_rdn11_slot;
        let mut var_i_cth_rdn2: f64 = *var_i_cth_rdn2_slot;
        let mut var_i_cth_rdn3: f64 = *var_i_cth_rdn3_slot;
        let mut var_i_cth_rdn4: f64 = *var_i_cth_rdn4_slot;
        let mut var_i_cth_rdn5: f64 = *var_i_cth_rdn5_slot;
        let mut var_i_cth_rdn6: f64 = *var_i_cth_rdn6_slot;
        let mut var_i_cth_rdn7: f64 = *var_i_cth_rdn7_slot;
        let mut var_i_cth_rdn8: f64 = *var_i_cth_rdn8_slot;
        let mut var_i_cth_rdn9: f64 = *var_i_cth_rdn9_slot;
        let mut var_i_cth_rv: f64 = *var_i_cth_rv_slot;
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
        let mut var_in_n_rdb0: f64 = *var_in_n_rdb0_slot;
        let mut var_in_n_rdb1: f64 = *var_in_n_rdb1_slot;
        let mut var_in_n_rdn0: f64 = *var_in_n_rdn0_slot;
        let mut var_in_n_rdn1: f64 = *var_in_n_rdn1_slot;
        let mut var_in_n_rdn10: f64 = *var_in_n_rdn10_slot;
        let mut var_in_n_rdn11: f64 = *var_in_n_rdn11_slot;
        let mut var_in_n_rdn2: f64 = *var_in_n_rdn2_slot;
        let mut var_in_n_rdn3: f64 = *var_in_n_rdn3_slot;
        let mut var_in_n_rdn4: f64 = *var_in_n_rdn4_slot;
        let mut var_in_n_rdn5: f64 = *var_in_n_rdn5_slot;
        let mut var_in_n_rdn6: f64 = *var_in_n_rdn6_slot;
        let mut var_in_n_rdn7: f64 = *var_in_n_rdn7_slot;
        let mut var_in_n_rdn8: f64 = *var_in_n_rdn8_slot;
        let mut var_in_n_rdn9: f64 = *var_in_n_rdn9_slot;
        let mut var_in_n_rv: f64 = *var_in_n_rv_slot;
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
        let mut var_qbc_rdb0: f64 = *var_qbc_rdb0_slot;
        let mut var_qbc_rdb1: f64 = *var_qbc_rdb1_slot;
        let mut var_qbc_rdn0: f64 = *var_qbc_rdn0_slot;
        let mut var_qbc_rdn1: f64 = *var_qbc_rdn1_slot;
        let mut var_qbc_rdn10: f64 = *var_qbc_rdn10_slot;
        let mut var_qbc_rdn11: f64 = *var_qbc_rdn11_slot;
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
        let mut var_qbe_qs_eff_rdb0: f64 = *var_qbe_qs_eff_rdb0_slot;
        let mut var_qbe_qs_eff_rdb1: f64 = *var_qbe_qs_eff_rdb1_slot;
        let mut var_qbe_qs_eff_rdn0: f64 = *var_qbe_qs_eff_rdn0_slot;
        let mut var_qbe_qs_eff_rdn1: f64 = *var_qbe_qs_eff_rdn1_slot;
        let mut var_qbe_qs_eff_rdn10: f64 = *var_qbe_qs_eff_rdn10_slot;
        let mut var_qbe_qs_eff_rdn11: f64 = *var_qbe_qs_eff_rdn11_slot;
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
        let mut var_qbe_rdn11: f64 = *var_qbe_rdn11_slot;
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
        let mut var_qe_dn11: f64 = *var_qe_dn11_slot;
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
        let mut var_qe_rdn11: f64 = *var_qe_rdn11_slot;
        let mut var_qe_rdn2: f64 = *var_qe_rdn2_slot;
        let mut var_qe_rdn3: f64 = *var_qe_rdn3_slot;
        let mut var_qe_rdn4: f64 = *var_qe_rdn4_slot;
        let mut var_qe_rdn5: f64 = *var_qe_rdn5_slot;
        let mut var_qe_rdn6: f64 = *var_qe_rdn6_slot;
        let mut var_qe_rdn7: f64 = *var_qe_rdn7_slot;
        let mut var_qe_rdn8: f64 = *var_qe_rdn8_slot;
        let mut var_qe_rdn9: f64 = *var_qe_rdn9_slot;
        let mut var_qe_rv: f64 = *var_qe_rv_slot;
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
        let mut var_taub_n_rdb0: f64 = *var_taub_n_rdb0_slot;
        let mut var_taub_n_rdb1: f64 = *var_taub_n_rdb1_slot;
        let mut var_taub_n_rdn0: f64 = *var_taub_n_rdn0_slot;
        let mut var_taub_n_rdn1: f64 = *var_taub_n_rdn1_slot;
        let mut var_taub_n_rdn10: f64 = *var_taub_n_rdn10_slot;
        let mut var_taub_n_rdn11: f64 = *var_taub_n_rdn11_slot;
        let mut var_taub_n_rdn2: f64 = *var_taub_n_rdn2_slot;
        let mut var_taub_n_rdn3: f64 = *var_taub_n_rdn3_slot;
        let mut var_taub_n_rdn4: f64 = *var_taub_n_rdn4_slot;
        let mut var_taub_n_rdn5: f64 = *var_taub_n_rdn5_slot;
        let mut var_taub_n_rdn6: f64 = *var_taub_n_rdn6_slot;
        let mut var_taub_n_rdn7: f64 = *var_taub_n_rdn7_slot;
        let mut var_taub_n_rdn8: f64 = *var_taub_n_rdn8_slot;
        let mut var_taub_n_rdn9: f64 = *var_taub_n_rdn9_slot;
        let mut var_taub_n_rv: f64 = *var_taub_n_rv_slot;

        let (assign6370_e6540, assign6370_e6540_d_n0, assign6370_e6540_d_n1, assign6370_e6540_d_n2, assign6370_e6540_d_n3, assign6370_e6540_d_n4, assign6370_e6540_d_n5, assign6370_e6540_d_n6, assign6370_e6540_d_n7, assign6370_e6540_d_n8, assign6370_e6540_d_n9, assign6370_e6540_d_n10, assign6370_e6540_d_n11, assign6370_e6540_d_b0, assign6370_e6540_d_b1,) = {
    if (var_guard115 != 0.0) {
        let assign6370_e6536: f64 = (1.0 - p.p94);
        let assign6370_e6538: f64 = (assign6370_e6536 * var_qe_qs);
        (assign6370_e6538, (assign6370_e6536 * var_qe_qs_dn0), (assign6370_e6536 * var_qe_qs_dn1), (assign6370_e6536 * var_qe_qs_dn2), (assign6370_e6536 * var_qe_qs_dn3), (assign6370_e6536 * var_qe_qs_dn4), (assign6370_e6536 * var_qe_qs_dn5), (assign6370_e6536 * var_qe_qs_dn6), (assign6370_e6536 * var_qe_qs_dn7), (assign6370_e6536 * var_qe_qs_dn8), (assign6370_e6536 * var_qe_qs_dn9), (assign6370_e6536 * var_qe_qs_dn10), (assign6370_e6536 * var_qe_qs_dn11), (assign6370_e6536 * var_qe_qs_db0), (assign6370_e6536 * var_qe_qs_db1),)
    } else {
        (var_qe, var_qe_dn0, var_qe_dn1, var_qe_dn2, var_qe_dn3, var_qe_dn4, var_qe_dn5, var_qe_dn6, var_qe_dn7, var_qe_dn8, var_qe_dn9, var_qe_dn10, var_qe_dn11, var_qe_db0, var_qe_db1,)
    }
};
        var_qe = assign6370_e6540;
        var_qe_dn0 = assign6370_e6540_d_n0;
        var_qe_dn1 = assign6370_e6540_d_n1;
        var_qe_dn2 = assign6370_e6540_d_n2;
        var_qe_dn3 = assign6370_e6540_d_n3;
        var_qe_dn4 = assign6370_e6540_d_n4;
        var_qe_dn5 = assign6370_e6540_d_n5;
        var_qe_dn6 = assign6370_e6540_d_n6;
        var_qe_dn7 = assign6370_e6540_d_n7;
        var_qe_dn8 = assign6370_e6540_d_n8;
        var_qe_dn9 = assign6370_e6540_d_n9;
        var_qe_dn10 = assign6370_e6540_d_n10;
        var_qe_dn11 = assign6370_e6540_d_n11;
        var_qe_db0 = assign6370_e6540_d_b0;
        var_qe_db1 = assign6370_e6540_d_b1;
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
        var_qe_rdn11 = 0.0;
        var_qe_rdb0 = 0.0;
        var_qe_rdb1 = 0.0;

        let (assign6380_e6548, assign6380_e6548_d_n0, assign6380_e6548_d_n1, assign6380_e6548_d_n2, assign6380_e6548_d_n3, assign6380_e6548_d_n4, assign6380_e6548_d_n5, assign6380_e6548_d_n6, assign6380_e6548_d_n7, assign6380_e6548_d_n8, assign6380_e6548_d_n9, assign6380_e6548_d_n10, assign6380_e6548_d_n11, assign6380_e6548_d_b0, assign6380_e6548_d_b1,) = {
    if (var_guard115 != 0.0) {
        let assign6380_e6545: f64 = (p.p94 * var_qe_qs);
        let assign6380_e6546: f64 = (var_qbe_qs + assign6380_e6545);
        (assign6380_e6546, (var_qbe_qs_dn0 + (p.p94 * var_qe_qs_dn0)), (var_qbe_qs_dn1 + (p.p94 * var_qe_qs_dn1)), (var_qbe_qs_dn2 + (p.p94 * var_qe_qs_dn2)), (var_qbe_qs_dn3 + (p.p94 * var_qe_qs_dn3)), (var_qbe_qs_dn4 + (p.p94 * var_qe_qs_dn4)), (var_qbe_qs_dn5 + (p.p94 * var_qe_qs_dn5)), (var_qbe_qs_dn6 + (p.p94 * var_qe_qs_dn6)), (var_qbe_qs_dn7 + (p.p94 * var_qe_qs_dn7)), (var_qbe_qs_dn8 + (p.p94 * var_qe_qs_dn8)), (var_qbe_qs_dn9 + (p.p94 * var_qe_qs_dn9)), (var_qbe_qs_dn10 + (p.p94 * var_qe_qs_dn10)), (var_qbe_qs_dn11 + (p.p94 * var_qe_qs_dn11)), (var_qbe_qs_db0 + (p.p94 * var_qe_qs_db0)), (var_qbe_qs_db1 + (p.p94 * var_qe_qs_db1)),)
    } else {
        (var_qbe_qs_eff, var_qbe_qs_eff_dn0, var_qbe_qs_eff_dn1, var_qbe_qs_eff_dn2, var_qbe_qs_eff_dn3, var_qbe_qs_eff_dn4, var_qbe_qs_eff_dn5, var_qbe_qs_eff_dn6, var_qbe_qs_eff_dn7, var_qbe_qs_eff_dn8, var_qbe_qs_eff_dn9, var_qbe_qs_eff_dn10, var_qbe_qs_eff_dn11, var_qbe_qs_eff_db0, var_qbe_qs_eff_db1,)
    }
};
        var_qbe_qs_eff = assign6380_e6548;
        var_qbe_qs_eff_dn0 = assign6380_e6548_d_n0;
        var_qbe_qs_eff_dn1 = assign6380_e6548_d_n1;
        var_qbe_qs_eff_dn2 = assign6380_e6548_d_n2;
        var_qbe_qs_eff_dn3 = assign6380_e6548_d_n3;
        var_qbe_qs_eff_dn4 = assign6380_e6548_d_n4;
        var_qbe_qs_eff_dn5 = assign6380_e6548_d_n5;
        var_qbe_qs_eff_dn6 = assign6380_e6548_d_n6;
        var_qbe_qs_eff_dn7 = assign6380_e6548_d_n7;
        var_qbe_qs_eff_dn8 = assign6380_e6548_d_n8;
        var_qbe_qs_eff_dn9 = assign6380_e6548_d_n9;
        var_qbe_qs_eff_dn10 = assign6380_e6548_d_n10;
        var_qbe_qs_eff_dn11 = assign6380_e6548_d_n11;
        var_qbe_qs_eff_db0 = assign6380_e6548_d_b0;
        var_qbe_qs_eff_db1 = assign6380_e6548_d_b1;
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
        var_qbe_qs_eff_rdn11 = 0.0;
        var_qbe_qs_eff_rdb0 = 0.0;
        var_qbe_qs_eff_rdb1 = 0.0;

        let (assign6390_e6556, assign6390_e6556_d_n0, assign6390_e6556_d_n1, assign6390_e6556_d_n2, assign6390_e6556_d_n3, assign6390_e6556_d_n4, assign6390_e6556_d_n5, assign6390_e6556_d_n6, assign6390_e6556_d_n7, assign6390_e6556_d_n8, assign6390_e6556_d_n9, assign6390_e6556_d_n10, assign6390_e6556_d_n11, assign6390_e6556_d_b0, assign6390_e6556_d_b1,) = {
    if (var_guard115 != 0.0) {
        let assign6390_e6552: f64 = (p.p93 * var_qbe_qs_eff);
        let assign6390_e6554: f64 = (assign6390_e6552 + var_qbc_qs);
        (assign6390_e6554, ((p.p93 * var_qbe_qs_eff_dn0) + var_qbc_qs_dn0), ((p.p93 * var_qbe_qs_eff_dn1) + var_qbc_qs_dn1), ((p.p93 * var_qbe_qs_eff_dn2) + var_qbc_qs_dn2), ((p.p93 * var_qbe_qs_eff_dn3) + var_qbc_qs_dn3), ((p.p93 * var_qbe_qs_eff_dn4) + var_qbc_qs_dn4), ((p.p93 * var_qbe_qs_eff_dn5) + var_qbc_qs_dn5), ((p.p93 * var_qbe_qs_eff_dn6) + var_qbc_qs_dn6), ((p.p93 * var_qbe_qs_eff_dn7) + var_qbc_qs_dn7), ((p.p93 * var_qbe_qs_eff_dn8) + var_qbc_qs_dn8), ((p.p93 * var_qbe_qs_eff_dn9) + var_qbc_qs_dn9), ((p.p93 * var_qbe_qs_eff_dn10) + var_qbc_qs_dn10), ((p.p93 * var_qbe_qs_eff_dn11) + var_qbc_qs_dn11), ((p.p93 * var_qbe_qs_eff_db0) + var_qbc_qs_db0), ((p.p93 * var_qbe_qs_eff_db1) + var_qbc_qs_db1),)
    } else {
        (var_qbc, var_qbc_dn0, var_qbc_dn1, var_qbc_dn2, var_qbc_dn3, var_qbc_dn4, var_qbc_dn5, var_qbc_dn6, var_qbc_dn7, var_qbc_dn8, var_qbc_dn9, var_qbc_dn10, var_qbc_dn11, var_qbc_db0, var_qbc_db1,)
    }
};
        var_qbc = assign6390_e6556;
        var_qbc_dn0 = assign6390_e6556_d_n0;
        var_qbc_dn1 = assign6390_e6556_d_n1;
        var_qbc_dn2 = assign6390_e6556_d_n2;
        var_qbc_dn3 = assign6390_e6556_d_n3;
        var_qbc_dn4 = assign6390_e6556_d_n4;
        var_qbc_dn5 = assign6390_e6556_d_n5;
        var_qbc_dn6 = assign6390_e6556_d_n6;
        var_qbc_dn7 = assign6390_e6556_d_n7;
        var_qbc_dn8 = assign6390_e6556_d_n8;
        var_qbc_dn9 = assign6390_e6556_d_n9;
        var_qbc_dn10 = assign6390_e6556_d_n10;
        var_qbc_dn11 = assign6390_e6556_d_n11;
        var_qbc_db0 = assign6390_e6556_d_b0;
        var_qbc_db1 = assign6390_e6556_d_b1;
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
        var_qbc_rdn11 = 0.0;
        var_qbc_rdb0 = 0.0;
        var_qbc_rdb1 = 0.0;

        let (assign6400_e6564, assign6400_e6564_d_n0, assign6400_e6564_d_n1, assign6400_e6564_d_n2, assign6400_e6564_d_n3, assign6400_e6564_d_n4, assign6400_e6564_d_n5, assign6400_e6564_d_n6, assign6400_e6564_d_n7, assign6400_e6564_d_n8, assign6400_e6564_d_n9, assign6400_e6564_d_n10, assign6400_e6564_d_n11, assign6400_e6564_d_b0, assign6400_e6564_d_b1,) = {
    if (var_guard115 != 0.0) {
        let assign6400_e6560: f64 = (1.0 - p.p93);
        let assign6400_e6562: f64 = (assign6400_e6560 * var_qbe_qs_eff);
        (assign6400_e6562, (assign6400_e6560 * var_qbe_qs_eff_dn0), (assign6400_e6560 * var_qbe_qs_eff_dn1), (assign6400_e6560 * var_qbe_qs_eff_dn2), (assign6400_e6560 * var_qbe_qs_eff_dn3), (assign6400_e6560 * var_qbe_qs_eff_dn4), (assign6400_e6560 * var_qbe_qs_eff_dn5), (assign6400_e6560 * var_qbe_qs_eff_dn6), (assign6400_e6560 * var_qbe_qs_eff_dn7), (assign6400_e6560 * var_qbe_qs_eff_dn8), (assign6400_e6560 * var_qbe_qs_eff_dn9), (assign6400_e6560 * var_qbe_qs_eff_dn10), (assign6400_e6560 * var_qbe_qs_eff_dn11), (assign6400_e6560 * var_qbe_qs_eff_db0), (assign6400_e6560 * var_qbe_qs_eff_db1),)
    } else {
        (var_qbe, var_qbe_dn0, var_qbe_dn1, var_qbe_dn2, var_qbe_dn3, var_qbe_dn4, var_qbe_dn5, var_qbe_dn6, var_qbe_dn7, var_qbe_dn8, var_qbe_dn9, var_qbe_dn10, var_qbe_dn11, var_qbe_db0, var_qbe_db1,)
    }
};
        var_qbe = assign6400_e6564;
        var_qbe_dn0 = assign6400_e6564_d_n0;
        var_qbe_dn1 = assign6400_e6564_d_n1;
        var_qbe_dn2 = assign6400_e6564_d_n2;
        var_qbe_dn3 = assign6400_e6564_d_n3;
        var_qbe_dn4 = assign6400_e6564_d_n4;
        var_qbe_dn5 = assign6400_e6564_d_n5;
        var_qbe_dn6 = assign6400_e6564_d_n6;
        var_qbe_dn7 = assign6400_e6564_d_n7;
        var_qbe_dn8 = assign6400_e6564_d_n8;
        var_qbe_dn9 = assign6400_e6564_d_n9;
        var_qbe_dn10 = assign6400_e6564_d_n10;
        var_qbe_dn11 = assign6400_e6564_d_n11;
        var_qbe_db0 = assign6400_e6564_d_b0;
        var_qbe_db1 = assign6400_e6564_d_b1;
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
        var_qbe_rdn11 = 0.0;
        var_qbe_rdb0 = 0.0;
        var_qbe_rdb1 = 0.0;

        let (assign6410_e6569, assign6410_e6569_d_n0, assign6410_e6569_d_n1, assign6410_e6569_d_n2, assign6410_e6569_d_n3, assign6410_e6569_d_n4, assign6410_e6569_d_n5, assign6410_e6569_d_n6, assign6410_e6569_d_n7, assign6410_e6569_d_n8, assign6410_e6569_d_n9, assign6410_e6569_d_n10, assign6410_e6569_d_n11, assign6410_e6569_d_b0, assign6410_e6569_d_b1,) = {
    if (var_guard115 == 0.0) {
        (var_qbe_qs, var_qbe_qs_dn0, var_qbe_qs_dn1, var_qbe_qs_dn2, var_qbe_qs_dn3, var_qbe_qs_dn4, var_qbe_qs_dn5, var_qbe_qs_dn6, var_qbe_qs_dn7, var_qbe_qs_dn8, var_qbe_qs_dn9, var_qbe_qs_dn10, var_qbe_qs_dn11, var_qbe_qs_db0, var_qbe_qs_db1,)
    } else {
        (var_qbe, var_qbe_dn0, var_qbe_dn1, var_qbe_dn2, var_qbe_dn3, var_qbe_dn4, var_qbe_dn5, var_qbe_dn6, var_qbe_dn7, var_qbe_dn8, var_qbe_dn9, var_qbe_dn10, var_qbe_dn11, var_qbe_db0, var_qbe_db1,)
    }
};
        var_qbe = assign6410_e6569;
        var_qbe_dn0 = assign6410_e6569_d_n0;
        var_qbe_dn1 = assign6410_e6569_d_n1;
        var_qbe_dn2 = assign6410_e6569_d_n2;
        var_qbe_dn3 = assign6410_e6569_d_n3;
        var_qbe_dn4 = assign6410_e6569_d_n4;
        var_qbe_dn5 = assign6410_e6569_d_n5;
        var_qbe_dn6 = assign6410_e6569_d_n6;
        var_qbe_dn7 = assign6410_e6569_d_n7;
        var_qbe_dn8 = assign6410_e6569_d_n8;
        var_qbe_dn9 = assign6410_e6569_d_n9;
        var_qbe_dn10 = assign6410_e6569_d_n10;
        var_qbe_dn11 = assign6410_e6569_d_n11;
        var_qbe_db0 = assign6410_e6569_d_b0;
        var_qbe_db1 = assign6410_e6569_d_b1;
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
        var_qbe_rdn11 = 0.0;
        var_qbe_rdb0 = 0.0;
        var_qbe_rdb1 = 0.0;

        let (assign6420_e6574, assign6420_e6574_d_n0, assign6420_e6574_d_n1, assign6420_e6574_d_n2, assign6420_e6574_d_n3, assign6420_e6574_d_n4, assign6420_e6574_d_n5, assign6420_e6574_d_n6, assign6420_e6574_d_n7, assign6420_e6574_d_n8, assign6420_e6574_d_n9, assign6420_e6574_d_n10, assign6420_e6574_d_n11, assign6420_e6574_d_b0, assign6420_e6574_d_b1,) = {
    if (var_guard115 == 0.0) {
        (var_qbc_qs, var_qbc_qs_dn0, var_qbc_qs_dn1, var_qbc_qs_dn2, var_qbc_qs_dn3, var_qbc_qs_dn4, var_qbc_qs_dn5, var_qbc_qs_dn6, var_qbc_qs_dn7, var_qbc_qs_dn8, var_qbc_qs_dn9, var_qbc_qs_dn10, var_qbc_qs_dn11, var_qbc_qs_db0, var_qbc_qs_db1,)
    } else {
        (var_qbc, var_qbc_dn0, var_qbc_dn1, var_qbc_dn2, var_qbc_dn3, var_qbc_dn4, var_qbc_dn5, var_qbc_dn6, var_qbc_dn7, var_qbc_dn8, var_qbc_dn9, var_qbc_dn10, var_qbc_dn11, var_qbc_db0, var_qbc_db1,)
    }
};
        var_qbc = assign6420_e6574;
        var_qbc_dn0 = assign6420_e6574_d_n0;
        var_qbc_dn1 = assign6420_e6574_d_n1;
        var_qbc_dn2 = assign6420_e6574_d_n2;
        var_qbc_dn3 = assign6420_e6574_d_n3;
        var_qbc_dn4 = assign6420_e6574_d_n4;
        var_qbc_dn5 = assign6420_e6574_d_n5;
        var_qbc_dn6 = assign6420_e6574_d_n6;
        var_qbc_dn7 = assign6420_e6574_d_n7;
        var_qbc_dn8 = assign6420_e6574_d_n8;
        var_qbc_dn9 = assign6420_e6574_d_n9;
        var_qbc_dn10 = assign6420_e6574_d_n10;
        var_qbc_dn11 = assign6420_e6574_d_n11;
        var_qbc_db0 = assign6420_e6574_d_b0;
        var_qbc_db1 = assign6420_e6574_d_b1;
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
        var_qbc_rdn11 = 0.0;
        var_qbc_rdb0 = 0.0;
        var_qbc_rdb1 = 0.0;

        let (assign6430_e6579, assign6430_e6579_d_n0, assign6430_e6579_d_n1, assign6430_e6579_d_n2, assign6430_e6579_d_n3, assign6430_e6579_d_n4, assign6430_e6579_d_n5, assign6430_e6579_d_n6, assign6430_e6579_d_n7, assign6430_e6579_d_n8, assign6430_e6579_d_n9, assign6430_e6579_d_n10, assign6430_e6579_d_n11, assign6430_e6579_d_b0, assign6430_e6579_d_b1,) = {
    if (var_guard115 == 0.0) {
        (var_qe_qs, var_qe_qs_dn0, var_qe_qs_dn1, var_qe_qs_dn2, var_qe_qs_dn3, var_qe_qs_dn4, var_qe_qs_dn5, var_qe_qs_dn6, var_qe_qs_dn7, var_qe_qs_dn8, var_qe_qs_dn9, var_qe_qs_dn10, var_qe_qs_dn11, var_qe_qs_db0, var_qe_qs_db1,)
    } else {
        (var_qe, var_qe_dn0, var_qe_dn1, var_qe_dn2, var_qe_dn3, var_qe_dn4, var_qe_dn5, var_qe_dn6, var_qe_dn7, var_qe_dn8, var_qe_dn9, var_qe_dn10, var_qe_dn11, var_qe_db0, var_qe_db1,)
    }
};
        var_qe = assign6430_e6579;
        var_qe_dn0 = assign6430_e6579_d_n0;
        var_qe_dn1 = assign6430_e6579_d_n1;
        var_qe_dn2 = assign6430_e6579_d_n2;
        var_qe_dn3 = assign6430_e6579_d_n3;
        var_qe_dn4 = assign6430_e6579_d_n4;
        var_qe_dn5 = assign6430_e6579_d_n5;
        var_qe_dn6 = assign6430_e6579_d_n6;
        var_qe_dn7 = assign6430_e6579_d_n7;
        var_qe_dn8 = assign6430_e6579_d_n8;
        var_qe_dn9 = assign6430_e6579_d_n9;
        var_qe_dn10 = assign6430_e6579_d_n10;
        var_qe_dn11 = assign6430_e6579_d_n11;
        var_qe_db0 = assign6430_e6579_d_b0;
        var_qe_db1 = assign6430_e6579_d_b1;
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
        var_qe_rdn11 = 0.0;
        var_qe_rdb0 = 0.0;
        var_qe_rdb1 = 0.0;

        let assign6450_e6585: f64 = (p.p134 * (nv3 - 0.0));
        let assign6450_e6586_q: f64 = assign6450_e6585;
        let assign6450_e6588: f64 = (assign6450_e6585 * p.p1);
        let assign6450_e6588_q: f64 = (assign6450_e6586_q * p.p1);
        var_i_cth = assign6450_e6588;
        var_i_cth_dn0 = 0.0;
        var_i_cth_dn1 = 0.0;
        var_i_cth_dn2 = 0.0;
        var_i_cth_dn3 = (p.p134 * p.p1);
        var_i_cth_dn4 = 0.0;
        var_i_cth_dn5 = 0.0;
        var_i_cth_dn6 = 0.0;
        var_i_cth_dn7 = 0.0;
        var_i_cth_dn8 = 0.0;
        var_i_cth_dn9 = 0.0;
        var_i_cth_dn10 = 0.0;
        var_i_cth_dn11 = 0.0;
        var_i_cth_db0 = 0.0;
        var_i_cth_db1 = 0.0;
        var_i_cth_rv = assign6450_e6588_q;
        var_i_cth_rdn0 = 0.0;
        var_i_cth_rdn1 = 0.0;
        var_i_cth_rdn2 = 0.0;
        var_i_cth_rdn3 = (p.p134 * p.p1);
        var_i_cth_rdn4 = 0.0;
        var_i_cth_rdn5 = 0.0;
        var_i_cth_rdn6 = 0.0;
        var_i_cth_rdn7 = 0.0;
        var_i_cth_rdn8 = 0.0;
        var_i_cth_rdn9 = 0.0;
        var_i_cth_rdn10 = 0.0;
        var_i_cth_rdn11 = 0.0;
        var_i_cth_rdb0 = 0.0;
        var_i_cth_rdb1 = 0.0;

        let assign6630_e6704: f64 = (var_if_ + var_ir);
        let assign6630_e6706: f64 = (assign6630_e6704 / var_qbi);
        var_in_n = assign6630_e6706;
        var_in_n_dn0 = ((((var_if__dn0 + var_ir_dn0) * var_qbi) - (assign6630_e6704 * var_qbi_dn0)) / (var_qbi * var_qbi));
        var_in_n_dn1 = ((((var_if__dn1 + var_ir_dn1) * var_qbi) - (assign6630_e6704 * var_qbi_dn1)) / (var_qbi * var_qbi));
        var_in_n_dn2 = ((((var_if__dn2 + var_ir_dn2) * var_qbi) - (assign6630_e6704 * var_qbi_dn2)) / (var_qbi * var_qbi));
        var_in_n_dn3 = ((((var_if__dn3 + var_ir_dn3) * var_qbi) - (assign6630_e6704 * var_qbi_dn3)) / (var_qbi * var_qbi));
        var_in_n_dn4 = ((((var_if__dn4 + var_ir_dn4) * var_qbi) - (assign6630_e6704 * var_qbi_dn4)) / (var_qbi * var_qbi));
        var_in_n_dn5 = ((((var_if__dn5 + var_ir_dn5) * var_qbi) - (assign6630_e6704 * var_qbi_dn5)) / (var_qbi * var_qbi));
        var_in_n_dn6 = ((((var_if__dn6 + var_ir_dn6) * var_qbi) - (assign6630_e6704 * var_qbi_dn6)) / (var_qbi * var_qbi));
        var_in_n_dn7 = ((((var_if__dn7 + var_ir_dn7) * var_qbi) - (assign6630_e6704 * var_qbi_dn7)) / (var_qbi * var_qbi));
        var_in_n_dn8 = ((((var_if__dn8 + var_ir_dn8) * var_qbi) - (assign6630_e6704 * var_qbi_dn8)) / (var_qbi * var_qbi));
        var_in_n_dn9 = ((((var_if__dn9 + var_ir_dn9) * var_qbi) - (assign6630_e6704 * var_qbi_dn9)) / (var_qbi * var_qbi));
        var_in_n_dn10 = ((((var_if__dn10 + var_ir_dn10) * var_qbi) - (assign6630_e6704 * var_qbi_dn10)) / (var_qbi * var_qbi));
        var_in_n_dn11 = ((((var_if__dn11 + var_ir_dn11) * var_qbi) - (assign6630_e6704 * var_qbi_dn11)) / (var_qbi * var_qbi));
        var_in_n_db0 = ((((var_if__db0 + var_ir_db0) * var_qbi) - (assign6630_e6704 * var_qbi_db0)) / (var_qbi * var_qbi));
        var_in_n_db1 = ((((var_if__db1 + var_ir_db1) * var_qbi) - (assign6630_e6704 * var_qbi_db1)) / (var_qbi * var_qbi));
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
        var_in_n_rdn11 = 0.0;
        var_in_n_rdb0 = 0.0;
        var_in_n_rdb1 = 0.0;

        let assign6690_e6739: f64 = if var_in_n > 0.0 { 1.0 } else { 0.0 };
        var_guard124 = assign6690_e6739;
        var_guard124_dn0 = 0.0;
        var_guard124_dn1 = 0.0;
        var_guard124_dn2 = 0.0;
        var_guard124_dn3 = 0.0;
        var_guard124_dn4 = 0.0;
        var_guard124_dn5 = 0.0;
        var_guard124_dn6 = 0.0;
        var_guard124_dn7 = 0.0;
        var_guard124_dn8 = 0.0;
        var_guard124_dn9 = 0.0;
        var_guard124_dn10 = 0.0;
        var_guard124_dn11 = 0.0;
        var_guard124_db0 = 0.0;
        var_guard124_db1 = 0.0;
        var_guard124_rv = 0.0;
        var_guard124_rdn0 = 0.0;
        var_guard124_rdn1 = 0.0;
        var_guard124_rdn2 = 0.0;
        var_guard124_rdn3 = 0.0;
        var_guard124_rdn4 = 0.0;
        var_guard124_rdn5 = 0.0;
        var_guard124_rdn6 = 0.0;
        var_guard124_rdn7 = 0.0;
        var_guard124_rdn8 = 0.0;
        var_guard124_rdn9 = 0.0;
        var_guard124_rdn10 = 0.0;
        var_guard124_rdn11 = 0.0;
        var_guard124_rdb0 = 0.0;
        var_guard124_rdb1 = 0.0;

        let (assign6700_e6747, assign6700_e6747_d_n0, assign6700_e6747_d_n1, assign6700_e6747_d_n2, assign6700_e6747_d_n3, assign6700_e6747_d_n4, assign6700_e6747_d_n5, assign6700_e6747_d_n6, assign6700_e6747_d_n7, assign6700_e6747_d_n8, assign6700_e6747_d_n9, assign6700_e6747_d_n10, assign6700_e6747_d_n11, assign6700_e6747_d_b0, assign6700_e6747_d_b1,) = {
    if (var_guard124 != 0.0) {
        let assign6700_e6743: f64 = (var_qbe + var_qbc);
        let assign6700_e6745: f64 = (assign6700_e6743 / var_in_n);
        (assign6700_e6745, ((((var_qbe_dn0 + var_qbc_dn0) * var_in_n) - (assign6700_e6743 * var_in_n_dn0)) / (var_in_n * var_in_n)), ((((var_qbe_dn1 + var_qbc_dn1) * var_in_n) - (assign6700_e6743 * var_in_n_dn1)) / (var_in_n * var_in_n)), ((((var_qbe_dn2 + var_qbc_dn2) * var_in_n) - (assign6700_e6743 * var_in_n_dn2)) / (var_in_n * var_in_n)), ((((var_qbe_dn3 + var_qbc_dn3) * var_in_n) - (assign6700_e6743 * var_in_n_dn3)) / (var_in_n * var_in_n)), ((((var_qbe_dn4 + var_qbc_dn4) * var_in_n) - (assign6700_e6743 * var_in_n_dn4)) / (var_in_n * var_in_n)), ((((var_qbe_dn5 + var_qbc_dn5) * var_in_n) - (assign6700_e6743 * var_in_n_dn5)) / (var_in_n * var_in_n)), ((((var_qbe_dn6 + var_qbc_dn6) * var_in_n) - (assign6700_e6743 * var_in_n_dn6)) / (var_in_n * var_in_n)), ((((var_qbe_dn7 + var_qbc_dn7) * var_in_n) - (assign6700_e6743 * var_in_n_dn7)) / (var_in_n * var_in_n)), ((((var_qbe_dn8 + var_qbc_dn8) * var_in_n) - (assign6700_e6743 * var_in_n_dn8)) / (var_in_n * var_in_n)), ((((var_qbe_dn9 + var_qbc_dn9) * var_in_n) - (assign6700_e6743 * var_in_n_dn9)) / (var_in_n * var_in_n)), ((((var_qbe_dn10 + var_qbc_dn10) * var_in_n) - (assign6700_e6743 * var_in_n_dn10)) / (var_in_n * var_in_n)), ((((var_qbe_dn11 + var_qbc_dn11) * var_in_n) - (assign6700_e6743 * var_in_n_dn11)) / (var_in_n * var_in_n)), ((((var_qbe_db0 + var_qbc_db0) * var_in_n) - (assign6700_e6743 * var_in_n_db0)) / (var_in_n * var_in_n)), ((((var_qbe_db1 + var_qbc_db1) * var_in_n) - (assign6700_e6743 * var_in_n_db1)) / (var_in_n * var_in_n)),)
    } else {
        (var_taub_n, var_taub_n_dn0, var_taub_n_dn1, var_taub_n_dn2, var_taub_n_dn3, var_taub_n_dn4, var_taub_n_dn5, var_taub_n_dn6, var_taub_n_dn7, var_taub_n_dn8, var_taub_n_dn9, var_taub_n_dn10, var_taub_n_dn11, var_taub_n_db0, var_taub_n_db1,)
    }
};
        var_taub_n = assign6700_e6747;
        var_taub_n_dn0 = assign6700_e6747_d_n0;
        var_taub_n_dn1 = assign6700_e6747_d_n1;
        var_taub_n_dn2 = assign6700_e6747_d_n2;
        var_taub_n_dn3 = assign6700_e6747_d_n3;
        var_taub_n_dn4 = assign6700_e6747_d_n4;
        var_taub_n_dn5 = assign6700_e6747_d_n5;
        var_taub_n_dn6 = assign6700_e6747_d_n6;
        var_taub_n_dn7 = assign6700_e6747_d_n7;
        var_taub_n_dn8 = assign6700_e6747_d_n8;
        var_taub_n_dn9 = assign6700_e6747_d_n9;
        var_taub_n_dn10 = assign6700_e6747_d_n10;
        var_taub_n_dn11 = assign6700_e6747_d_n11;
        var_taub_n_db0 = assign6700_e6747_d_b0;
        var_taub_n_db1 = assign6700_e6747_d_b1;
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
        var_taub_n_rdn11 = 0.0;
        var_taub_n_rdb0 = 0.0;
        var_taub_n_rdb1 = 0.0;

        let (assign6710_e6756, assign6710_e6756_d_n0, assign6710_e6756_d_n1, assign6710_e6756_d_n2, assign6710_e6756_d_n3, assign6710_e6756_d_n4, assign6710_e6756_d_n5, assign6710_e6756_d_n6, assign6710_e6756_d_n7, assign6710_e6756_d_n8, assign6710_e6756_d_n9, assign6710_e6756_d_n10, assign6710_e6756_d_n11, assign6710_e6756_d_b0, assign6710_e6756_d_b1,) = {
    if (var_guard124 == 0.0) {
        let assign6710_e6752: f64 = (var_taub_t * var_q1q);
        let assign6710_e6754: f64 = (assign6710_e6752 * var_qbi);
        (assign6710_e6754, ((((var_taub_t_dn0 * var_q1q) + (var_taub_t * var_q1q_dn0)) * var_qbi) + (assign6710_e6752 * var_qbi_dn0)), ((((var_taub_t_dn1 * var_q1q) + (var_taub_t * var_q1q_dn1)) * var_qbi) + (assign6710_e6752 * var_qbi_dn1)), ((((var_taub_t_dn2 * var_q1q) + (var_taub_t * var_q1q_dn2)) * var_qbi) + (assign6710_e6752 * var_qbi_dn2)), ((((var_taub_t_dn3 * var_q1q) + (var_taub_t * var_q1q_dn3)) * var_qbi) + (assign6710_e6752 * var_qbi_dn3)), ((((var_taub_t_dn4 * var_q1q) + (var_taub_t * var_q1q_dn4)) * var_qbi) + (assign6710_e6752 * var_qbi_dn4)), ((((var_taub_t_dn5 * var_q1q) + (var_taub_t * var_q1q_dn5)) * var_qbi) + (assign6710_e6752 * var_qbi_dn5)), ((((var_taub_t_dn6 * var_q1q) + (var_taub_t * var_q1q_dn6)) * var_qbi) + (assign6710_e6752 * var_qbi_dn6)), ((((var_taub_t_dn7 * var_q1q) + (var_taub_t * var_q1q_dn7)) * var_qbi) + (assign6710_e6752 * var_qbi_dn7)), ((((var_taub_t_dn8 * var_q1q) + (var_taub_t * var_q1q_dn8)) * var_qbi) + (assign6710_e6752 * var_qbi_dn8)), ((((var_taub_t_dn9 * var_q1q) + (var_taub_t * var_q1q_dn9)) * var_qbi) + (assign6710_e6752 * var_qbi_dn9)), ((((var_taub_t_dn10 * var_q1q) + (var_taub_t * var_q1q_dn10)) * var_qbi) + (assign6710_e6752 * var_qbi_dn10)), ((((var_taub_t_dn11 * var_q1q) + (var_taub_t * var_q1q_dn11)) * var_qbi) + (assign6710_e6752 * var_qbi_dn11)), ((((var_taub_t_db0 * var_q1q) + (var_taub_t * var_q1q_db0)) * var_qbi) + (assign6710_e6752 * var_qbi_db0)), ((((var_taub_t_db1 * var_q1q) + (var_taub_t * var_q1q_db1)) * var_qbi) + (assign6710_e6752 * var_qbi_db1)),)
    } else {
        (var_taub_n, var_taub_n_dn0, var_taub_n_dn1, var_taub_n_dn2, var_taub_n_dn3, var_taub_n_dn4, var_taub_n_dn5, var_taub_n_dn6, var_taub_n_dn7, var_taub_n_dn8, var_taub_n_dn9, var_taub_n_dn10, var_taub_n_dn11, var_taub_n_db0, var_taub_n_db1,)
    }
};
        var_taub_n = assign6710_e6756;
        var_taub_n_dn0 = assign6710_e6756_d_n0;
        var_taub_n_dn1 = assign6710_e6756_d_n1;
        var_taub_n_dn2 = assign6710_e6756_d_n2;
        var_taub_n_dn3 = assign6710_e6756_d_n3;
        var_taub_n_dn4 = assign6710_e6756_d_n4;
        var_taub_n_dn5 = assign6710_e6756_d_n5;
        var_taub_n_dn6 = assign6710_e6756_d_n6;
        var_taub_n_dn7 = assign6710_e6756_d_n7;
        var_taub_n_dn8 = assign6710_e6756_d_n8;
        var_taub_n_dn9 = assign6710_e6756_d_n9;
        var_taub_n_dn10 = assign6710_e6756_d_n10;
        var_taub_n_dn11 = assign6710_e6756_d_n11;
        var_taub_n_db0 = assign6710_e6756_d_b0;
        var_taub_n_db1 = assign6710_e6756_d_b1;
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
        var_taub_n_rdn11 = 0.0;
        var_taub_n_rdb0 = 0.0;
        var_taub_n_rdb1 = 0.0;

        let assign6720_e6759: f64 = if p.p130 == 1.0 { 1.0 } else { 0.0 };
        var_guard125 = assign6720_e6759;
        var_guard125_dn0 = 0.0;
        var_guard125_dn1 = 0.0;
        var_guard125_dn2 = 0.0;
        var_guard125_dn3 = 0.0;
        var_guard125_dn4 = 0.0;
        var_guard125_dn5 = 0.0;
        var_guard125_dn6 = 0.0;
        var_guard125_dn7 = 0.0;
        var_guard125_dn8 = 0.0;
        var_guard125_dn9 = 0.0;
        var_guard125_dn10 = 0.0;
        var_guard125_dn11 = 0.0;
        var_guard125_db0 = 0.0;
        var_guard125_db1 = 0.0;
        var_guard125_rv = 0.0;
        var_guard125_rdn0 = 0.0;
        var_guard125_rdn1 = 0.0;
        var_guard125_rdn2 = 0.0;
        var_guard125_rdn3 = 0.0;
        var_guard125_rdn4 = 0.0;
        var_guard125_rdn5 = 0.0;
        var_guard125_rdn6 = 0.0;
        var_guard125_rdn7 = 0.0;
        var_guard125_rdn8 = 0.0;
        var_guard125_rdn9 = 0.0;
        var_guard125_rdn10 = 0.0;
        var_guard125_rdn11 = 0.0;
        var_guard125_rdb0 = 0.0;
        var_guard125_rdb1 = 0.0;

        *var_guard124_slot = var_guard124;
        *var_guard124_db0_slot = var_guard124_db0;
        *var_guard124_db1_slot = var_guard124_db1;
        *var_guard124_dn0_slot = var_guard124_dn0;
        *var_guard124_dn1_slot = var_guard124_dn1;
        *var_guard124_dn10_slot = var_guard124_dn10;
        *var_guard124_dn11_slot = var_guard124_dn11;
        *var_guard124_dn2_slot = var_guard124_dn2;
        *var_guard124_dn3_slot = var_guard124_dn3;
        *var_guard124_dn4_slot = var_guard124_dn4;
        *var_guard124_dn5_slot = var_guard124_dn5;
        *var_guard124_dn6_slot = var_guard124_dn6;
        *var_guard124_dn7_slot = var_guard124_dn7;
        *var_guard124_dn8_slot = var_guard124_dn8;
        *var_guard124_dn9_slot = var_guard124_dn9;
        *var_guard124_rdb0_slot = var_guard124_rdb0;
        *var_guard124_rdb1_slot = var_guard124_rdb1;
        *var_guard124_rdn0_slot = var_guard124_rdn0;
        *var_guard124_rdn1_slot = var_guard124_rdn1;
        *var_guard124_rdn10_slot = var_guard124_rdn10;
        *var_guard124_rdn11_slot = var_guard124_rdn11;
        *var_guard124_rdn2_slot = var_guard124_rdn2;
        *var_guard124_rdn3_slot = var_guard124_rdn3;
        *var_guard124_rdn4_slot = var_guard124_rdn4;
        *var_guard124_rdn5_slot = var_guard124_rdn5;
        *var_guard124_rdn6_slot = var_guard124_rdn6;
        *var_guard124_rdn7_slot = var_guard124_rdn7;
        *var_guard124_rdn8_slot = var_guard124_rdn8;
        *var_guard124_rdn9_slot = var_guard124_rdn9;
        *var_guard124_rv_slot = var_guard124_rv;
        *var_guard125_slot = var_guard125;
        *var_guard125_db0_slot = var_guard125_db0;
        *var_guard125_db1_slot = var_guard125_db1;
        *var_guard125_dn0_slot = var_guard125_dn0;
        *var_guard125_dn1_slot = var_guard125_dn1;
        *var_guard125_dn10_slot = var_guard125_dn10;
        *var_guard125_dn11_slot = var_guard125_dn11;
        *var_guard125_dn2_slot = var_guard125_dn2;
        *var_guard125_dn3_slot = var_guard125_dn3;
        *var_guard125_dn4_slot = var_guard125_dn4;
        *var_guard125_dn5_slot = var_guard125_dn5;
        *var_guard125_dn6_slot = var_guard125_dn6;
        *var_guard125_dn7_slot = var_guard125_dn7;
        *var_guard125_dn8_slot = var_guard125_dn8;
        *var_guard125_dn9_slot = var_guard125_dn9;
        *var_guard125_rdb0_slot = var_guard125_rdb0;
        *var_guard125_rdb1_slot = var_guard125_rdb1;
        *var_guard125_rdn0_slot = var_guard125_rdn0;
        *var_guard125_rdn1_slot = var_guard125_rdn1;
        *var_guard125_rdn10_slot = var_guard125_rdn10;
        *var_guard125_rdn11_slot = var_guard125_rdn11;
        *var_guard125_rdn2_slot = var_guard125_rdn2;
        *var_guard125_rdn3_slot = var_guard125_rdn3;
        *var_guard125_rdn4_slot = var_guard125_rdn4;
        *var_guard125_rdn5_slot = var_guard125_rdn5;
        *var_guard125_rdn6_slot = var_guard125_rdn6;
        *var_guard125_rdn7_slot = var_guard125_rdn7;
        *var_guard125_rdn8_slot = var_guard125_rdn8;
        *var_guard125_rdn9_slot = var_guard125_rdn9;
        *var_guard125_rv_slot = var_guard125_rv;
        *var_i_cth_slot = var_i_cth;
        *var_i_cth_db0_slot = var_i_cth_db0;
        *var_i_cth_db1_slot = var_i_cth_db1;
        *var_i_cth_dn0_slot = var_i_cth_dn0;
        *var_i_cth_dn1_slot = var_i_cth_dn1;
        *var_i_cth_dn10_slot = var_i_cth_dn10;
        *var_i_cth_dn11_slot = var_i_cth_dn11;
        *var_i_cth_dn2_slot = var_i_cth_dn2;
        *var_i_cth_dn3_slot = var_i_cth_dn3;
        *var_i_cth_dn4_slot = var_i_cth_dn4;
        *var_i_cth_dn5_slot = var_i_cth_dn5;
        *var_i_cth_dn6_slot = var_i_cth_dn6;
        *var_i_cth_dn7_slot = var_i_cth_dn7;
        *var_i_cth_dn8_slot = var_i_cth_dn8;
        *var_i_cth_dn9_slot = var_i_cth_dn9;
        *var_i_cth_rdb0_slot = var_i_cth_rdb0;
        *var_i_cth_rdb1_slot = var_i_cth_rdb1;
        *var_i_cth_rdn0_slot = var_i_cth_rdn0;
        *var_i_cth_rdn1_slot = var_i_cth_rdn1;
        *var_i_cth_rdn10_slot = var_i_cth_rdn10;
        *var_i_cth_rdn11_slot = var_i_cth_rdn11;
        *var_i_cth_rdn2_slot = var_i_cth_rdn2;
        *var_i_cth_rdn3_slot = var_i_cth_rdn3;
        *var_i_cth_rdn4_slot = var_i_cth_rdn4;
        *var_i_cth_rdn5_slot = var_i_cth_rdn5;
        *var_i_cth_rdn6_slot = var_i_cth_rdn6;
        *var_i_cth_rdn7_slot = var_i_cth_rdn7;
        *var_i_cth_rdn8_slot = var_i_cth_rdn8;
        *var_i_cth_rdn9_slot = var_i_cth_rdn9;
        *var_i_cth_rv_slot = var_i_cth_rv;
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
        *var_in_n_rdb0_slot = var_in_n_rdb0;
        *var_in_n_rdb1_slot = var_in_n_rdb1;
        *var_in_n_rdn0_slot = var_in_n_rdn0;
        *var_in_n_rdn1_slot = var_in_n_rdn1;
        *var_in_n_rdn10_slot = var_in_n_rdn10;
        *var_in_n_rdn11_slot = var_in_n_rdn11;
        *var_in_n_rdn2_slot = var_in_n_rdn2;
        *var_in_n_rdn3_slot = var_in_n_rdn3;
        *var_in_n_rdn4_slot = var_in_n_rdn4;
        *var_in_n_rdn5_slot = var_in_n_rdn5;
        *var_in_n_rdn6_slot = var_in_n_rdn6;
        *var_in_n_rdn7_slot = var_in_n_rdn7;
        *var_in_n_rdn8_slot = var_in_n_rdn8;
        *var_in_n_rdn9_slot = var_in_n_rdn9;
        *var_in_n_rv_slot = var_in_n_rv;
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
        *var_qbc_rdb0_slot = var_qbc_rdb0;
        *var_qbc_rdb1_slot = var_qbc_rdb1;
        *var_qbc_rdn0_slot = var_qbc_rdn0;
        *var_qbc_rdn1_slot = var_qbc_rdn1;
        *var_qbc_rdn10_slot = var_qbc_rdn10;
        *var_qbc_rdn11_slot = var_qbc_rdn11;
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
        *var_qbe_qs_eff_rdb0_slot = var_qbe_qs_eff_rdb0;
        *var_qbe_qs_eff_rdb1_slot = var_qbe_qs_eff_rdb1;
        *var_qbe_qs_eff_rdn0_slot = var_qbe_qs_eff_rdn0;
        *var_qbe_qs_eff_rdn1_slot = var_qbe_qs_eff_rdn1;
        *var_qbe_qs_eff_rdn10_slot = var_qbe_qs_eff_rdn10;
        *var_qbe_qs_eff_rdn11_slot = var_qbe_qs_eff_rdn11;
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
        *var_qbe_rdn11_slot = var_qbe_rdn11;
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
        *var_qe_dn11_slot = var_qe_dn11;
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
        *var_qe_rdn11_slot = var_qe_rdn11;
        *var_qe_rdn2_slot = var_qe_rdn2;
        *var_qe_rdn3_slot = var_qe_rdn3;
        *var_qe_rdn4_slot = var_qe_rdn4;
        *var_qe_rdn5_slot = var_qe_rdn5;
        *var_qe_rdn6_slot = var_qe_rdn6;
        *var_qe_rdn7_slot = var_qe_rdn7;
        *var_qe_rdn8_slot = var_qe_rdn8;
        *var_qe_rdn9_slot = var_qe_rdn9;
        *var_qe_rv_slot = var_qe_rv;
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
        *var_taub_n_rdb0_slot = var_taub_n_rdb0;
        *var_taub_n_rdb1_slot = var_taub_n_rdb1;
        *var_taub_n_rdn0_slot = var_taub_n_rdn0;
        *var_taub_n_rdn1_slot = var_taub_n_rdn1;
        *var_taub_n_rdn10_slot = var_taub_n_rdn10;
        *var_taub_n_rdn11_slot = var_taub_n_rdn11;
        *var_taub_n_rdn2_slot = var_taub_n_rdn2;
        *var_taub_n_rdn3_slot = var_taub_n_rdn3;
        *var_taub_n_rdn4_slot = var_taub_n_rdn4;
        *var_taub_n_rdn5_slot = var_taub_n_rdn5;
        *var_taub_n_rdn6_slot = var_taub_n_rdn6;
        *var_taub_n_rdn7_slot = var_taub_n_rdn7;
        *var_taub_n_rdn8_slot = var_taub_n_rdn8;
        *var_taub_n_rdn9_slot = var_taub_n_rdn9;
        *var_taub_n_rv_slot = var_taub_n_rv;
    }

    pub(super) fn stamp_reactive_block_41(
        p: &Parameters,
        var_guard125: f64,
        var_taub_n: f64,
        var_taub_n_db0: f64,
        var_taub_n_db1: f64,
        var_taub_n_dn0: f64,
        var_taub_n_dn1: f64,
        var_taub_n_dn10: f64,
        var_taub_n_dn11: f64,
        var_taub_n_dn2: f64,
        var_taub_n_dn3: f64,
        var_taub_n_dn4: f64,
        var_taub_n_dn5: f64,
        var_taub_n_dn6: f64,
        var_taub_n_dn7: f64,
        var_taub_n_dn8: f64,
        var_taub_n_dn9: f64,
        var_guard126_slot: &mut f64,
        var_guard126_db0_slot: &mut f64,
        var_guard126_db1_slot: &mut f64,
        var_guard126_dn0_slot: &mut f64,
        var_guard126_dn1_slot: &mut f64,
        var_guard126_dn10_slot: &mut f64,
        var_guard126_dn11_slot: &mut f64,
        var_guard126_dn2_slot: &mut f64,
        var_guard126_dn3_slot: &mut f64,
        var_guard126_dn4_slot: &mut f64,
        var_guard126_dn5_slot: &mut f64,
        var_guard126_dn6_slot: &mut f64,
        var_guard126_dn7_slot: &mut f64,
        var_guard126_dn8_slot: &mut f64,
        var_guard126_dn9_slot: &mut f64,
        var_guard126_rdb0_slot: &mut f64,
        var_guard126_rdb1_slot: &mut f64,
        var_guard126_rdn0_slot: &mut f64,
        var_guard126_rdn1_slot: &mut f64,
        var_guard126_rdn10_slot: &mut f64,
        var_guard126_rdn11_slot: &mut f64,
        var_guard126_rdn2_slot: &mut f64,
        var_guard126_rdn3_slot: &mut f64,
        var_guard126_rdn4_slot: &mut f64,
        var_guard126_rdn5_slot: &mut f64,
        var_guard126_rdn6_slot: &mut f64,
        var_guard126_rdn7_slot: &mut f64,
        var_guard126_rdn8_slot: &mut f64,
        var_guard126_rdn9_slot: &mut f64,
        var_guard126_rv_slot: &mut f64,
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
        var_taun_rdb0_slot: &mut f64,
        var_taun_rdb1_slot: &mut f64,
        var_taun_rdn0_slot: &mut f64,
        var_taun_rdn1_slot: &mut f64,
        var_taun_rdn10_slot: &mut f64,
        var_taun_rdn11_slot: &mut f64,
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
        let mut var_guard126: f64 = *var_guard126_slot;
        let mut var_guard126_db0: f64 = *var_guard126_db0_slot;
        let mut var_guard126_db1: f64 = *var_guard126_db1_slot;
        let mut var_guard126_dn0: f64 = *var_guard126_dn0_slot;
        let mut var_guard126_dn1: f64 = *var_guard126_dn1_slot;
        let mut var_guard126_dn10: f64 = *var_guard126_dn10_slot;
        let mut var_guard126_dn11: f64 = *var_guard126_dn11_slot;
        let mut var_guard126_dn2: f64 = *var_guard126_dn2_slot;
        let mut var_guard126_dn3: f64 = *var_guard126_dn3_slot;
        let mut var_guard126_dn4: f64 = *var_guard126_dn4_slot;
        let mut var_guard126_dn5: f64 = *var_guard126_dn5_slot;
        let mut var_guard126_dn6: f64 = *var_guard126_dn6_slot;
        let mut var_guard126_dn7: f64 = *var_guard126_dn7_slot;
        let mut var_guard126_dn8: f64 = *var_guard126_dn8_slot;
        let mut var_guard126_dn9: f64 = *var_guard126_dn9_slot;
        let mut var_guard126_rdb0: f64 = *var_guard126_rdb0_slot;
        let mut var_guard126_rdb1: f64 = *var_guard126_rdb1_slot;
        let mut var_guard126_rdn0: f64 = *var_guard126_rdn0_slot;
        let mut var_guard126_rdn1: f64 = *var_guard126_rdn1_slot;
        let mut var_guard126_rdn10: f64 = *var_guard126_rdn10_slot;
        let mut var_guard126_rdn11: f64 = *var_guard126_rdn11_slot;
        let mut var_guard126_rdn2: f64 = *var_guard126_rdn2_slot;
        let mut var_guard126_rdn3: f64 = *var_guard126_rdn3_slot;
        let mut var_guard126_rdn4: f64 = *var_guard126_rdn4_slot;
        let mut var_guard126_rdn5: f64 = *var_guard126_rdn5_slot;
        let mut var_guard126_rdn6: f64 = *var_guard126_rdn6_slot;
        let mut var_guard126_rdn7: f64 = *var_guard126_rdn7_slot;
        let mut var_guard126_rdn8: f64 = *var_guard126_rdn8_slot;
        let mut var_guard126_rdn9: f64 = *var_guard126_rdn9_slot;
        let mut var_guard126_rv: f64 = *var_guard126_rv_slot;
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
        let mut var_taun_rdb0: f64 = *var_taun_rdb0_slot;
        let mut var_taun_rdb1: f64 = *var_taun_rdb1_slot;
        let mut var_taun_rdn0: f64 = *var_taun_rdn0_slot;
        let mut var_taun_rdn1: f64 = *var_taun_rdn1_slot;
        let mut var_taun_rdn10: f64 = *var_taun_rdn10_slot;
        let mut var_taun_rdn11: f64 = *var_taun_rdn11_slot;
        let mut var_taun_rdn2: f64 = *var_taun_rdn2_slot;
        let mut var_taun_rdn3: f64 = *var_taun_rdn3_slot;
        let mut var_taun_rdn4: f64 = *var_taun_rdn4_slot;
        let mut var_taun_rdn5: f64 = *var_taun_rdn5_slot;
        let mut var_taun_rdn6: f64 = *var_taun_rdn6_slot;
        let mut var_taun_rdn7: f64 = *var_taun_rdn7_slot;
        let mut var_taun_rdn8: f64 = *var_taun_rdn8_slot;
        let mut var_taun_rdn9: f64 = *var_taun_rdn9_slot;
        let mut var_taun_rv: f64 = *var_taun_rv_slot;

        let (assign6730_e6765, assign6730_e6765_d_n0, assign6730_e6765_d_n1, assign6730_e6765_d_n2, assign6730_e6765_d_n3, assign6730_e6765_d_n4, assign6730_e6765_d_n5, assign6730_e6765_d_n6, assign6730_e6765_d_n7, assign6730_e6765_d_n8, assign6730_e6765_d_n9, assign6730_e6765_d_n10, assign6730_e6765_d_n11, assign6730_e6765_d_b0, assign6730_e6765_d_b1,) = {
    if (var_guard125 != 0.0) {
        let assign6730_e6763: f64 = (p.p93 * var_taub_n);
        (assign6730_e6763, (p.p93 * var_taub_n_dn0), (p.p93 * var_taub_n_dn1), (p.p93 * var_taub_n_dn2), (p.p93 * var_taub_n_dn3), (p.p93 * var_taub_n_dn4), (p.p93 * var_taub_n_dn5), (p.p93 * var_taub_n_dn6), (p.p93 * var_taub_n_dn7), (p.p93 * var_taub_n_dn8), (p.p93 * var_taub_n_dn9), (p.p93 * var_taub_n_dn10), (p.p93 * var_taub_n_dn11), (p.p93 * var_taub_n_db0), (p.p93 * var_taub_n_db1),)
    } else {
        (var_taun, var_taun_dn0, var_taun_dn1, var_taun_dn2, var_taun_dn3, var_taun_dn4, var_taun_dn5, var_taun_dn6, var_taun_dn7, var_taun_dn8, var_taun_dn9, var_taun_dn10, var_taun_dn11, var_taun_db0, var_taun_db1,)
    }
};
        var_taun = assign6730_e6765;
        var_taun_dn0 = assign6730_e6765_d_n0;
        var_taun_dn1 = assign6730_e6765_d_n1;
        var_taun_dn2 = assign6730_e6765_d_n2;
        var_taun_dn3 = assign6730_e6765_d_n3;
        var_taun_dn4 = assign6730_e6765_d_n4;
        var_taun_dn5 = assign6730_e6765_d_n5;
        var_taun_dn6 = assign6730_e6765_d_n6;
        var_taun_dn7 = assign6730_e6765_d_n7;
        var_taun_dn8 = assign6730_e6765_d_n8;
        var_taun_dn9 = assign6730_e6765_d_n9;
        var_taun_dn10 = assign6730_e6765_d_n10;
        var_taun_dn11 = assign6730_e6765_d_n11;
        var_taun_db0 = assign6730_e6765_d_b0;
        var_taun_db1 = assign6730_e6765_d_b1;
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
        var_taun_rdn11 = 0.0;
        var_taun_rdb0 = 0.0;
        var_taun_rdb1 = 0.0;

        let assign6740_e6768: f64 = if p.p130 == 2.0 { 1.0 } else { 0.0 };
        var_guard126 = assign6740_e6768;
        var_guard126_dn0 = 0.0;
        var_guard126_dn1 = 0.0;
        var_guard126_dn2 = 0.0;
        var_guard126_dn3 = 0.0;
        var_guard126_dn4 = 0.0;
        var_guard126_dn5 = 0.0;
        var_guard126_dn6 = 0.0;
        var_guard126_dn7 = 0.0;
        var_guard126_dn8 = 0.0;
        var_guard126_dn9 = 0.0;
        var_guard126_dn10 = 0.0;
        var_guard126_dn11 = 0.0;
        var_guard126_db0 = 0.0;
        var_guard126_db1 = 0.0;
        var_guard126_rv = 0.0;
        var_guard126_rdn0 = 0.0;
        var_guard126_rdn1 = 0.0;
        var_guard126_rdn2 = 0.0;
        var_guard126_rdn3 = 0.0;
        var_guard126_rdn4 = 0.0;
        var_guard126_rdn5 = 0.0;
        var_guard126_rdn6 = 0.0;
        var_guard126_rdn7 = 0.0;
        var_guard126_rdn8 = 0.0;
        var_guard126_rdn9 = 0.0;
        var_guard126_rdn10 = 0.0;
        var_guard126_rdn11 = 0.0;
        var_guard126_rdb0 = 0.0;
        var_guard126_rdb1 = 0.0;

        let (assign6750_e6777, assign6750_e6777_d_n0, assign6750_e6777_d_n1, assign6750_e6777_d_n2, assign6750_e6777_d_n3, assign6750_e6777_d_n4, assign6750_e6777_d_n5, assign6750_e6777_d_n6, assign6750_e6777_d_n7, assign6750_e6777_d_n8, assign6750_e6777_d_n9, assign6750_e6777_d_n10, assign6750_e6777_d_n11, assign6750_e6777_d_b0, assign6750_e6777_d_b1,) = {
    if ((var_guard125 == 0.0) && (var_guard126 != 0.0)) {
        let assign6750_e6775: f64 = (p.p131 * var_taub_n);
        (assign6750_e6775, (p.p131 * var_taub_n_dn0), (p.p131 * var_taub_n_dn1), (p.p131 * var_taub_n_dn2), (p.p131 * var_taub_n_dn3), (p.p131 * var_taub_n_dn4), (p.p131 * var_taub_n_dn5), (p.p131 * var_taub_n_dn6), (p.p131 * var_taub_n_dn7), (p.p131 * var_taub_n_dn8), (p.p131 * var_taub_n_dn9), (p.p131 * var_taub_n_dn10), (p.p131 * var_taub_n_dn11), (p.p131 * var_taub_n_db0), (p.p131 * var_taub_n_db1),)
    } else {
        (var_taun, var_taun_dn0, var_taun_dn1, var_taun_dn2, var_taun_dn3, var_taun_dn4, var_taun_dn5, var_taun_dn6, var_taun_dn7, var_taun_dn8, var_taun_dn9, var_taun_dn10, var_taun_dn11, var_taun_db0, var_taun_db1,)
    }
};
        var_taun = assign6750_e6777;
        var_taun_dn0 = assign6750_e6777_d_n0;
        var_taun_dn1 = assign6750_e6777_d_n1;
        var_taun_dn2 = assign6750_e6777_d_n2;
        var_taun_dn3 = assign6750_e6777_d_n3;
        var_taun_dn4 = assign6750_e6777_d_n4;
        var_taun_dn5 = assign6750_e6777_d_n5;
        var_taun_dn6 = assign6750_e6777_d_n6;
        var_taun_dn7 = assign6750_e6777_d_n7;
        var_taun_dn8 = assign6750_e6777_d_n8;
        var_taun_dn9 = assign6750_e6777_d_n9;
        var_taun_dn10 = assign6750_e6777_d_n10;
        var_taun_dn11 = assign6750_e6777_d_n11;
        var_taun_db0 = assign6750_e6777_d_b0;
        var_taun_db1 = assign6750_e6777_d_b1;
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
        var_taun_rdn11 = 0.0;
        var_taun_rdb0 = 0.0;
        var_taun_rdb1 = 0.0;

        let (assign6760_e6785, assign6760_e6785_d_n0, assign6760_e6785_d_n1, assign6760_e6785_d_n2, assign6760_e6785_d_n3, assign6760_e6785_d_n4, assign6760_e6785_d_n5, assign6760_e6785_d_n6, assign6760_e6785_d_n7, assign6760_e6785_d_n8, assign6760_e6785_d_n9, assign6760_e6785_d_n10, assign6760_e6785_d_n11, assign6760_e6785_d_b0, assign6760_e6785_d_b1,) = {
    if ((var_guard125 == 0.0) && (var_guard126 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_taun, var_taun_dn0, var_taun_dn1, var_taun_dn2, var_taun_dn3, var_taun_dn4, var_taun_dn5, var_taun_dn6, var_taun_dn7, var_taun_dn8, var_taun_dn9, var_taun_dn10, var_taun_dn11, var_taun_db0, var_taun_db1,)
    }
};
        var_taun = assign6760_e6785;
        var_taun_dn0 = assign6760_e6785_d_n0;
        var_taun_dn1 = assign6760_e6785_d_n1;
        var_taun_dn2 = assign6760_e6785_d_n2;
        var_taun_dn3 = assign6760_e6785_d_n3;
        var_taun_dn4 = assign6760_e6785_d_n4;
        var_taun_dn5 = assign6760_e6785_d_n5;
        var_taun_dn6 = assign6760_e6785_d_n6;
        var_taun_dn7 = assign6760_e6785_d_n7;
        var_taun_dn8 = assign6760_e6785_d_n8;
        var_taun_dn9 = assign6760_e6785_d_n9;
        var_taun_dn10 = assign6760_e6785_d_n10;
        var_taun_dn11 = assign6760_e6785_d_n11;
        var_taun_db0 = assign6760_e6785_d_b0;
        var_taun_db1 = assign6760_e6785_d_b1;
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
        var_taun_rdn11 = 0.0;
        var_taun_rdb0 = 0.0;
        var_taun_rdb1 = 0.0;

        *var_guard126_slot = var_guard126;
        *var_guard126_db0_slot = var_guard126_db0;
        *var_guard126_db1_slot = var_guard126_db1;
        *var_guard126_dn0_slot = var_guard126_dn0;
        *var_guard126_dn1_slot = var_guard126_dn1;
        *var_guard126_dn10_slot = var_guard126_dn10;
        *var_guard126_dn11_slot = var_guard126_dn11;
        *var_guard126_dn2_slot = var_guard126_dn2;
        *var_guard126_dn3_slot = var_guard126_dn3;
        *var_guard126_dn4_slot = var_guard126_dn4;
        *var_guard126_dn5_slot = var_guard126_dn5;
        *var_guard126_dn6_slot = var_guard126_dn6;
        *var_guard126_dn7_slot = var_guard126_dn7;
        *var_guard126_dn8_slot = var_guard126_dn8;
        *var_guard126_dn9_slot = var_guard126_dn9;
        *var_guard126_rdb0_slot = var_guard126_rdb0;
        *var_guard126_rdb1_slot = var_guard126_rdb1;
        *var_guard126_rdn0_slot = var_guard126_rdn0;
        *var_guard126_rdn1_slot = var_guard126_rdn1;
        *var_guard126_rdn10_slot = var_guard126_rdn10;
        *var_guard126_rdn11_slot = var_guard126_rdn11;
        *var_guard126_rdn2_slot = var_guard126_rdn2;
        *var_guard126_rdn3_slot = var_guard126_rdn3;
        *var_guard126_rdn4_slot = var_guard126_rdn4;
        *var_guard126_rdn5_slot = var_guard126_rdn5;
        *var_guard126_rdn6_slot = var_guard126_rdn6;
        *var_guard126_rdn7_slot = var_guard126_rdn7;
        *var_guard126_rdn8_slot = var_guard126_rdn8;
        *var_guard126_rdn9_slot = var_guard126_rdn9;
        *var_guard126_rv_slot = var_guard126_rv;
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
        *var_taun_rdb0_slot = var_taun_rdb0;
        *var_taun_rdb1_slot = var_taun_rdb1;
        *var_taun_rdn0_slot = var_taun_rdn0;
        *var_taun_rdn1_slot = var_taun_rdn1;
        *var_taun_rdn10_slot = var_taun_rdn10;
        *var_taun_rdn11_slot = var_taun_rdn11;
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
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        multiplicity: f64,
        var_gmin: f64,
        var_guard117: f64,
        var_iavl: f64,
        var_iavl_db0: f64,
        var_iavl_db1: f64,
        var_iavl_dn0: f64,
        var_iavl_dn1: f64,
        var_iavl_dn10: f64,
        var_iavl_dn11: f64,
        var_iavl_dn2: f64,
        var_iavl_dn3: f64,
        var_iavl_dn4: f64,
        var_iavl_dn5: f64,
        var_iavl_dn6: f64,
        var_iavl_dn7: f64,
        var_iavl_dn8: f64,
        var_iavl_dn9: f64,
        var_ib1: f64,
        var_ib1_db0: f64,
        var_ib1_db1: f64,
        var_ib1_dn0: f64,
        var_ib1_dn1: f64,
        var_ib1_dn10: f64,
        var_ib1_dn11: f64,
        var_ib1_dn2: f64,
        var_ib1_dn3: f64,
        var_ib1_dn4: f64,
        var_ib1_dn5: f64,
        var_ib1_dn6: f64,
        var_ib1_dn7: f64,
        var_ib1_dn8: f64,
        var_ib1_dn9: f64,
        var_ib1_s: f64,
        var_ib1_s_db0: f64,
        var_ib1_s_db1: f64,
        var_ib1_s_dn0: f64,
        var_ib1_s_dn1: f64,
        var_ib1_s_dn10: f64,
        var_ib1_s_dn11: f64,
        var_ib1_s_dn2: f64,
        var_ib1_s_dn3: f64,
        var_ib1_s_dn4: f64,
        var_ib1_s_dn5: f64,
        var_ib1_s_dn6: f64,
        var_ib1_s_dn7: f64,
        var_ib1_s_dn8: f64,
        var_ib1_s_dn9: f64,
        var_ib1b2: f64,
        var_ib1b2_db0: f64,
        var_ib1b2_db1: f64,
        var_ib1b2_dn0: f64,
        var_ib1b2_dn1: f64,
        var_ib1b2_dn10: f64,
        var_ib1b2_dn11: f64,
        var_ib1b2_dn2: f64,
        var_ib1b2_dn3: f64,
        var_ib1b2_dn4: f64,
        var_ib1b2_dn5: f64,
        var_ib1b2_dn6: f64,
        var_ib1b2_dn7: f64,
        var_ib1b2_dn8: f64,
        var_ib1b2_dn9: f64,
        var_ib2: f64,
        var_ib2_db0: f64,
        var_ib2_db1: f64,
        var_ib2_dn0: f64,
        var_ib2_dn1: f64,
        var_ib2_dn10: f64,
        var_ib2_dn11: f64,
        var_ib2_dn2: f64,
        var_ib2_dn3: f64,
        var_ib2_dn4: f64,
        var_ib2_dn5: f64,
        var_ib2_dn6: f64,
        var_ib2_dn7: f64,
        var_ib2_dn8: f64,
        var_ib2_dn9: f64,
        var_ib2_s: f64,
        var_ib2_s_db0: f64,
        var_ib2_s_db1: f64,
        var_ib2_s_dn0: f64,
        var_ib2_s_dn1: f64,
        var_ib2_s_dn10: f64,
        var_ib2_s_dn11: f64,
        var_ib2_s_dn2: f64,
        var_ib2_s_dn3: f64,
        var_ib2_s_dn4: f64,
        var_ib2_s_dn5: f64,
        var_ib2_s_dn6: f64,
        var_ib2_s_dn7: f64,
        var_ib2_s_dn8: f64,
        var_ib2_s_dn9: f64,
        var_ibrel: f64,
        var_ibrel_db0: f64,
        var_ibrel_db1: f64,
        var_ibrel_dn0: f64,
        var_ibrel_dn1: f64,
        var_ibrel_dn10: f64,
        var_ibrel_dn11: f64,
        var_ibrel_dn2: f64,
        var_ibrel_dn3: f64,
        var_ibrel_dn4: f64,
        var_ibrel_dn5: f64,
        var_ibrel_dn6: f64,
        var_ibrel_dn7: f64,
        var_ibrel_dn8: f64,
        var_ibrel_dn9: f64,
        var_ibtbt: f64,
        var_ibtbt_db0: f64,
        var_ibtbt_db1: f64,
        var_ibtbt_dn0: f64,
        var_ibtbt_dn1: f64,
        var_ibtbt_dn10: f64,
        var_ibtbt_dn11: f64,
        var_ibtbt_dn2: f64,
        var_ibtbt_dn3: f64,
        var_ibtbt_dn4: f64,
        var_ibtbt_dn5: f64,
        var_ibtbt_dn6: f64,
        var_ibtbt_dn7: f64,
        var_ibtbt_dn8: f64,
        var_ibtbt_dn9: f64,
        var_ic1c2: f64,
        var_ic1c2_db0: f64,
        var_ic1c2_db1: f64,
        var_ic1c2_dn0: f64,
        var_ic1c2_dn1: f64,
        var_ic1c2_dn10: f64,
        var_ic1c2_dn11: f64,
        var_ic1c2_dn2: f64,
        var_ic1c2_dn3: f64,
        var_ic1c2_dn4: f64,
        var_ic1c2_dn5: f64,
        var_ic1c2_dn6: f64,
        var_ic1c2_dn7: f64,
        var_ic1c2_dn8: f64,
        var_ic1c2_dn9: f64,
        var_in_: f64,
        var_in__db0: f64,
        var_in__db1: f64,
        var_in__dn0: f64,
        var_in__dn1: f64,
        var_in__dn10: f64,
        var_in__dn11: f64,
        var_in__dn2: f64,
        var_in__dn3: f64,
        var_in__dn4: f64,
        var_in__dn5: f64,
        var_in__dn6: f64,
        var_in__dn7: f64,
        var_in__dn8: f64,
        var_in__dn9: f64,
        var_itat: f64,
        var_itat_db0: f64,
        var_itat_db1: f64,
        var_itat_dn0: f64,
        var_itat_dn1: f64,
        var_itat_dn10: f64,
        var_itat_dn11: f64,
        var_itat_dn2: f64,
        var_itat_dn3: f64,
        var_itat_dn4: f64,
        var_itat_dn5: f64,
        var_itat_dn6: f64,
        var_itat_dn7: f64,
        var_itat_dn8: f64,
        var_itat_dn9: f64,
        var_iztcb: f64,
        var_iztcb_db0: f64,
        var_iztcb_db1: f64,
        var_iztcb_dn0: f64,
        var_iztcb_dn1: f64,
        var_iztcb_dn10: f64,
        var_iztcb_dn11: f64,
        var_iztcb_dn2: f64,
        var_iztcb_dn3: f64,
        var_iztcb_dn4: f64,
        var_iztcb_dn5: f64,
        var_iztcb_dn6: f64,
        var_iztcb_dn7: f64,
        var_iztcb_dn8: f64,
        var_iztcb_dn9: f64,
        var_izteb: f64,
        var_izteb_db0: f64,
        var_izteb_db1: f64,
        var_izteb_dn0: f64,
        var_izteb_dn1: f64,
        var_izteb_dn10: f64,
        var_izteb_dn11: f64,
        var_izteb_dn2: f64,
        var_izteb_dn3: f64,
        var_izteb_dn4: f64,
        var_izteb_dn5: f64,
        var_izteb_dn6: f64,
        var_izteb_dn7: f64,
        var_izteb_dn8: f64,
        var_izteb_dn9: f64,
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
    ) {
        let eq0_e154: f64 = (p.p3 * var_ic1c2);
        let eq0_e154_d_n0: f64 = (p.p3 * var_ic1c2_dn0);
        let eq0_e154_d_n1: f64 = (p.p3 * var_ic1c2_dn1);
        let eq0_e154_d_n2: f64 = (p.p3 * var_ic1c2_dn2);
        let eq0_e154_d_n3: f64 = (p.p3 * var_ic1c2_dn3);
        let eq0_e154_d_n4: f64 = (p.p3 * var_ic1c2_dn4);
        let eq0_e154_d_n5: f64 = (p.p3 * var_ic1c2_dn5);
        let eq0_e154_d_n6: f64 = (p.p3 * var_ic1c2_dn6);
        let eq0_e154_d_n7: f64 = (p.p3 * var_ic1c2_dn7);
        let eq0_e154_d_n8: f64 = (p.p3 * var_ic1c2_dn8);
        let eq0_e154_d_n9: f64 = (p.p3 * var_ic1c2_dn9);
        let eq0_e154_d_n10: f64 = (p.p3 * var_ic1c2_dn10);
        let eq0_e154_d_n11: f64 = (p.p3 * var_ic1c2_dn11);
        let eq0_e154_d_b0: f64 = (p.p3 * var_ic1c2_db0);
        let eq0_e154_d_b1: f64 = (p.p3 * var_ic1c2_db1);
        let eq0_e156: f64 = (eq0_e154 * p.p1);
        let eq0_e156_d_n0: f64 = (eq0_e154_d_n0 * p.p1);
        let eq0_e156_d_n1: f64 = (eq0_e154_d_n1 * p.p1);
        let eq0_e156_d_n2: f64 = (eq0_e154_d_n2 * p.p1);
        let eq0_e156_d_n3: f64 = (eq0_e154_d_n3 * p.p1);
        let eq0_e156_d_n4: f64 = (eq0_e154_d_n4 * p.p1);
        let eq0_e156_d_n5: f64 = (eq0_e154_d_n5 * p.p1);
        let eq0_e156_d_n6: f64 = (eq0_e154_d_n6 * p.p1);
        let eq0_e156_d_n7: f64 = (eq0_e154_d_n7 * p.p1);
        let eq0_e156_d_n8: f64 = (eq0_e154_d_n8 * p.p1);
        let eq0_e156_d_n9: f64 = (eq0_e154_d_n9 * p.p1);
        let eq0_e156_d_n10: f64 = (eq0_e154_d_n10 * p.p1);
        let eq0_e156_d_n11: f64 = (eq0_e154_d_n11 * p.p1);
        let eq0_e156_d_b0: f64 = (eq0_e154_d_b0 * p.p1);
        let eq0_e156_d_b1: f64 = (eq0_e154_d_b1 * p.p1);
        let eq0_value: f64 = eq0_e156;
        let eq0_node_derivatives: [f64; 12] = [eq0_e156_d_n0, eq0_e156_d_n1, eq0_e156_d_n2, eq0_e156_d_n3, eq0_e156_d_n4, eq0_e156_d_n5, eq0_e156_d_n6, eq0_e156_d_n7, eq0_e156_d_n8, eq0_e156_d_n9, eq0_e156_d_n10, eq0_e156_d_n11];
        let eq0_branch_derivatives: [f64; 2] = [eq0_e156_d_b0, eq0_e156_d_b1];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let eq1_e159: f64 = (p.p3 * var_in_);
        let eq1_e159_d_n0: f64 = (p.p3 * var_in__dn0);
        let eq1_e159_d_n1: f64 = (p.p3 * var_in__dn1);
        let eq1_e159_d_n2: f64 = (p.p3 * var_in__dn2);
        let eq1_e159_d_n3: f64 = (p.p3 * var_in__dn3);
        let eq1_e159_d_n4: f64 = (p.p3 * var_in__dn4);
        let eq1_e159_d_n5: f64 = (p.p3 * var_in__dn5);
        let eq1_e159_d_n6: f64 = (p.p3 * var_in__dn6);
        let eq1_e159_d_n7: f64 = (p.p3 * var_in__dn7);
        let eq1_e159_d_n8: f64 = (p.p3 * var_in__dn8);
        let eq1_e159_d_n9: f64 = (p.p3 * var_in__dn9);
        let eq1_e159_d_n10: f64 = (p.p3 * var_in__dn10);
        let eq1_e159_d_n11: f64 = (p.p3 * var_in__dn11);
        let eq1_e159_d_b0: f64 = (p.p3 * var_in__db0);
        let eq1_e159_d_b1: f64 = (p.p3 * var_in__db1);
        let eq1_e161: f64 = (eq1_e159 * p.p1);
        let eq1_e161_d_n0: f64 = (eq1_e159_d_n0 * p.p1);
        let eq1_e161_d_n1: f64 = (eq1_e159_d_n1 * p.p1);
        let eq1_e161_d_n2: f64 = (eq1_e159_d_n2 * p.p1);
        let eq1_e161_d_n3: f64 = (eq1_e159_d_n3 * p.p1);
        let eq1_e161_d_n4: f64 = (eq1_e159_d_n4 * p.p1);
        let eq1_e161_d_n5: f64 = (eq1_e159_d_n5 * p.p1);
        let eq1_e161_d_n6: f64 = (eq1_e159_d_n6 * p.p1);
        let eq1_e161_d_n7: f64 = (eq1_e159_d_n7 * p.p1);
        let eq1_e161_d_n8: f64 = (eq1_e159_d_n8 * p.p1);
        let eq1_e161_d_n9: f64 = (eq1_e159_d_n9 * p.p1);
        let eq1_e161_d_n10: f64 = (eq1_e159_d_n10 * p.p1);
        let eq1_e161_d_n11: f64 = (eq1_e159_d_n11 * p.p1);
        let eq1_e161_d_b0: f64 = (eq1_e159_d_b0 * p.p1);
        let eq1_e161_d_b1: f64 = (eq1_e159_d_b1 * p.p1);
        let eq1_value: f64 = eq1_e161;
        let eq1_node_derivatives: [f64; 12] = [eq1_e161_d_n0, eq1_e161_d_n1, eq1_e161_d_n2, eq1_e161_d_n3, eq1_e161_d_n4, eq1_e161_d_n5, eq1_e161_d_n6, eq1_e161_d_n7, eq1_e161_d_n8, eq1_e161_d_n9, eq1_e161_d_n10, eq1_e161_d_n11];
        let eq1_branch_derivatives: [f64; 2] = [eq1_e161_d_b0, eq1_e161_d_b1];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(4),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let eq2_e165: f64 = (var_ib1_s + var_ib2_s);
        let eq2_e165_d_n0: f64 = (var_ib1_s_dn0 + var_ib2_s_dn0);
        let eq2_e165_d_n1: f64 = (var_ib1_s_dn1 + var_ib2_s_dn1);
        let eq2_e165_d_n2: f64 = (var_ib1_s_dn2 + var_ib2_s_dn2);
        let eq2_e165_d_n3: f64 = (var_ib1_s_dn3 + var_ib2_s_dn3);
        let eq2_e165_d_n4: f64 = (var_ib1_s_dn4 + var_ib2_s_dn4);
        let eq2_e165_d_n5: f64 = (var_ib1_s_dn5 + var_ib2_s_dn5);
        let eq2_e165_d_n6: f64 = (var_ib1_s_dn6 + var_ib2_s_dn6);
        let eq2_e165_d_n7: f64 = (var_ib1_s_dn7 + var_ib2_s_dn7);
        let eq2_e165_d_n8: f64 = (var_ib1_s_dn8 + var_ib2_s_dn8);
        let eq2_e165_d_n9: f64 = (var_ib1_s_dn9 + var_ib2_s_dn9);
        let eq2_e165_d_n10: f64 = (var_ib1_s_dn10 + var_ib2_s_dn10);
        let eq2_e165_d_n11: f64 = (var_ib1_s_dn11 + var_ib2_s_dn11);
        let eq2_e165_d_b0: f64 = (var_ib1_s_db0 + var_ib2_s_db0);
        let eq2_e165_d_b1: f64 = (var_ib1_s_db1 + var_ib2_s_db1);
        let eq2_e167: f64 = (eq2_e165 + var_ibrel);
        let eq2_e167_d_n0: f64 = (eq2_e165_d_n0 + var_ibrel_dn0);
        let eq2_e167_d_n1: f64 = (eq2_e165_d_n1 + var_ibrel_dn1);
        let eq2_e167_d_n2: f64 = (eq2_e165_d_n2 + var_ibrel_dn2);
        let eq2_e167_d_n3: f64 = (eq2_e165_d_n3 + var_ibrel_dn3);
        let eq2_e167_d_n4: f64 = (eq2_e165_d_n4 + var_ibrel_dn4);
        let eq2_e167_d_n5: f64 = (eq2_e165_d_n5 + var_ibrel_dn5);
        let eq2_e167_d_n6: f64 = (eq2_e165_d_n6 + var_ibrel_dn6);
        let eq2_e167_d_n7: f64 = (eq2_e165_d_n7 + var_ibrel_dn7);
        let eq2_e167_d_n8: f64 = (eq2_e165_d_n8 + var_ibrel_dn8);
        let eq2_e167_d_n9: f64 = (eq2_e165_d_n9 + var_ibrel_dn9);
        let eq2_e167_d_n10: f64 = (eq2_e165_d_n10 + var_ibrel_dn10);
        let eq2_e167_d_n11: f64 = (eq2_e165_d_n11 + var_ibrel_dn11);
        let eq2_e167_d_b0: f64 = (eq2_e165_d_b0 + var_ibrel_db0);
        let eq2_e167_d_b1: f64 = (eq2_e165_d_b1 + var_ibrel_db1);
        let eq2_e168: f64 = (p.p3 * eq2_e167);
        let eq2_e168_d_n0: f64 = (p.p3 * eq2_e167_d_n0);
        let eq2_e168_d_n1: f64 = (p.p3 * eq2_e167_d_n1);
        let eq2_e168_d_n2: f64 = (p.p3 * eq2_e167_d_n2);
        let eq2_e168_d_n3: f64 = (p.p3 * eq2_e167_d_n3);
        let eq2_e168_d_n4: f64 = (p.p3 * eq2_e167_d_n4);
        let eq2_e168_d_n5: f64 = (p.p3 * eq2_e167_d_n5);
        let eq2_e168_d_n6: f64 = (p.p3 * eq2_e167_d_n6);
        let eq2_e168_d_n7: f64 = (p.p3 * eq2_e167_d_n7);
        let eq2_e168_d_n8: f64 = (p.p3 * eq2_e167_d_n8);
        let eq2_e168_d_n9: f64 = (p.p3 * eq2_e167_d_n9);
        let eq2_e168_d_n10: f64 = (p.p3 * eq2_e167_d_n10);
        let eq2_e168_d_n11: f64 = (p.p3 * eq2_e167_d_n11);
        let eq2_e168_d_b0: f64 = (p.p3 * eq2_e167_d_b0);
        let eq2_e168_d_b1: f64 = (p.p3 * eq2_e167_d_b1);
        let eq2_e170: f64 = (eq2_e168 * p.p1);
        let eq2_e170_d_n0: f64 = (eq2_e168_d_n0 * p.p1);
        let eq2_e170_d_n1: f64 = (eq2_e168_d_n1 * p.p1);
        let eq2_e170_d_n2: f64 = (eq2_e168_d_n2 * p.p1);
        let eq2_e170_d_n3: f64 = (eq2_e168_d_n3 * p.p1);
        let eq2_e170_d_n4: f64 = (eq2_e168_d_n4 * p.p1);
        let eq2_e170_d_n5: f64 = (eq2_e168_d_n5 * p.p1);
        let eq2_e170_d_n6: f64 = (eq2_e168_d_n6 * p.p1);
        let eq2_e170_d_n7: f64 = (eq2_e168_d_n7 * p.p1);
        let eq2_e170_d_n8: f64 = (eq2_e168_d_n8 * p.p1);
        let eq2_e170_d_n9: f64 = (eq2_e168_d_n9 * p.p1);
        let eq2_e170_d_n10: f64 = (eq2_e168_d_n10 * p.p1);
        let eq2_e170_d_n11: f64 = (eq2_e168_d_n11 * p.p1);
        let eq2_e170_d_b0: f64 = (eq2_e168_d_b0 * p.p1);
        let eq2_e170_d_b1: f64 = (eq2_e168_d_b1 * p.p1);
        let eq2_value: f64 = eq2_e170;
        let eq2_node_derivatives: [f64; 12] = [eq2_e170_d_n0, eq2_e170_d_n1, eq2_e170_d_n2, eq2_e170_d_n3, eq2_e170_d_n4, eq2_e170_d_n5, eq2_e170_d_n6, eq2_e170_d_n7, eq2_e170_d_n8, eq2_e170_d_n9, eq2_e170_d_n10, eq2_e170_d_n11];
        let eq2_branch_derivatives: [f64; 2] = [eq2_e170_d_b0, eq2_e170_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(4),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let eq3_e174: f64 = (var_ib1 + var_ib2);
        let eq3_e174_d_n0: f64 = (var_ib1_dn0 + var_ib2_dn0);
        let eq3_e174_d_n1: f64 = (var_ib1_dn1 + var_ib2_dn1);
        let eq3_e174_d_n2: f64 = (var_ib1_dn2 + var_ib2_dn2);
        let eq3_e174_d_n3: f64 = (var_ib1_dn3 + var_ib2_dn3);
        let eq3_e174_d_n4: f64 = (var_ib1_dn4 + var_ib2_dn4);
        let eq3_e174_d_n5: f64 = (var_ib1_dn5 + var_ib2_dn5);
        let eq3_e174_d_n6: f64 = (var_ib1_dn6 + var_ib2_dn6);
        let eq3_e174_d_n7: f64 = (var_ib1_dn7 + var_ib2_dn7);
        let eq3_e174_d_n8: f64 = (var_ib1_dn8 + var_ib2_dn8);
        let eq3_e174_d_n9: f64 = (var_ib1_dn9 + var_ib2_dn9);
        let eq3_e174_d_n10: f64 = (var_ib1_dn10 + var_ib2_dn10);
        let eq3_e174_d_n11: f64 = (var_ib1_dn11 + var_ib2_dn11);
        let eq3_e174_d_b0: f64 = (var_ib1_db0 + var_ib2_db0);
        let eq3_e174_d_b1: f64 = (var_ib1_db1 + var_ib2_db1);
        let eq3_e177: f64 = (var_gmin * var_vb2e1);
        let eq3_e177_d_n0: f64 = (var_gmin * var_vb2e1_dn0);
        let eq3_e177_d_n1: f64 = (var_gmin * var_vb2e1_dn1);
        let eq3_e177_d_n2: f64 = (var_gmin * var_vb2e1_dn2);
        let eq3_e177_d_n3: f64 = (var_gmin * var_vb2e1_dn3);
        let eq3_e177_d_n4: f64 = (var_gmin * var_vb2e1_dn4);
        let eq3_e177_d_n5: f64 = (var_gmin * var_vb2e1_dn5);
        let eq3_e177_d_n6: f64 = (var_gmin * var_vb2e1_dn6);
        let eq3_e177_d_n7: f64 = (var_gmin * var_vb2e1_dn7);
        let eq3_e177_d_n8: f64 = (var_gmin * var_vb2e1_dn8);
        let eq3_e177_d_n9: f64 = (var_gmin * var_vb2e1_dn9);
        let eq3_e177_d_n10: f64 = (var_gmin * var_vb2e1_dn10);
        let eq3_e177_d_n11: f64 = (var_gmin * var_vb2e1_dn11);
        let eq3_e177_d_b0: f64 = (var_gmin * var_vb2e1_db0);
        let eq3_e177_d_b1: f64 = (var_gmin * var_vb2e1_db1);
        let eq3_e178: f64 = (eq3_e174 + eq3_e177);
        let eq3_e178_d_n0: f64 = (eq3_e174_d_n0 + eq3_e177_d_n0);
        let eq3_e178_d_n1: f64 = (eq3_e174_d_n1 + eq3_e177_d_n1);
        let eq3_e178_d_n2: f64 = (eq3_e174_d_n2 + eq3_e177_d_n2);
        let eq3_e178_d_n3: f64 = (eq3_e174_d_n3 + eq3_e177_d_n3);
        let eq3_e178_d_n4: f64 = (eq3_e174_d_n4 + eq3_e177_d_n4);
        let eq3_e178_d_n5: f64 = (eq3_e174_d_n5 + eq3_e177_d_n5);
        let eq3_e178_d_n6: f64 = (eq3_e174_d_n6 + eq3_e177_d_n6);
        let eq3_e178_d_n7: f64 = (eq3_e174_d_n7 + eq3_e177_d_n7);
        let eq3_e178_d_n8: f64 = (eq3_e174_d_n8 + eq3_e177_d_n8);
        let eq3_e178_d_n9: f64 = (eq3_e174_d_n9 + eq3_e177_d_n9);
        let eq3_e178_d_n10: f64 = (eq3_e174_d_n10 + eq3_e177_d_n10);
        let eq3_e178_d_n11: f64 = (eq3_e174_d_n11 + eq3_e177_d_n11);
        let eq3_e178_d_b0: f64 = (eq3_e174_d_b0 + eq3_e177_d_b0);
        let eq3_e178_d_b1: f64 = (eq3_e174_d_b1 + eq3_e177_d_b1);
        let eq3_e180: f64 = (eq3_e178 - var_izteb);
        let eq3_e180_d_n0: f64 = (eq3_e178_d_n0 - var_izteb_dn0);
        let eq3_e180_d_n1: f64 = (eq3_e178_d_n1 - var_izteb_dn1);
        let eq3_e180_d_n2: f64 = (eq3_e178_d_n2 - var_izteb_dn2);
        let eq3_e180_d_n3: f64 = (eq3_e178_d_n3 - var_izteb_dn3);
        let eq3_e180_d_n4: f64 = (eq3_e178_d_n4 - var_izteb_dn4);
        let eq3_e180_d_n5: f64 = (eq3_e178_d_n5 - var_izteb_dn5);
        let eq3_e180_d_n6: f64 = (eq3_e178_d_n6 - var_izteb_dn6);
        let eq3_e180_d_n7: f64 = (eq3_e178_d_n7 - var_izteb_dn7);
        let eq3_e180_d_n8: f64 = (eq3_e178_d_n8 - var_izteb_dn8);
        let eq3_e180_d_n9: f64 = (eq3_e178_d_n9 - var_izteb_dn9);
        let eq3_e180_d_n10: f64 = (eq3_e178_d_n10 - var_izteb_dn10);
        let eq3_e180_d_n11: f64 = (eq3_e178_d_n11 - var_izteb_dn11);
        let eq3_e180_d_b0: f64 = (eq3_e178_d_b0 - var_izteb_db0);
        let eq3_e180_d_b1: f64 = (eq3_e178_d_b1 - var_izteb_db1);
        let eq3_e182: f64 = (eq3_e180 + var_ibtbt);
        let eq3_e182_d_n0: f64 = (eq3_e180_d_n0 + var_ibtbt_dn0);
        let eq3_e182_d_n1: f64 = (eq3_e180_d_n1 + var_ibtbt_dn1);
        let eq3_e182_d_n2: f64 = (eq3_e180_d_n2 + var_ibtbt_dn2);
        let eq3_e182_d_n3: f64 = (eq3_e180_d_n3 + var_ibtbt_dn3);
        let eq3_e182_d_n4: f64 = (eq3_e180_d_n4 + var_ibtbt_dn4);
        let eq3_e182_d_n5: f64 = (eq3_e180_d_n5 + var_ibtbt_dn5);
        let eq3_e182_d_n6: f64 = (eq3_e180_d_n6 + var_ibtbt_dn6);
        let eq3_e182_d_n7: f64 = (eq3_e180_d_n7 + var_ibtbt_dn7);
        let eq3_e182_d_n8: f64 = (eq3_e180_d_n8 + var_ibtbt_dn8);
        let eq3_e182_d_n9: f64 = (eq3_e180_d_n9 + var_ibtbt_dn9);
        let eq3_e182_d_n10: f64 = (eq3_e180_d_n10 + var_ibtbt_dn10);
        let eq3_e182_d_n11: f64 = (eq3_e180_d_n11 + var_ibtbt_dn11);
        let eq3_e182_d_b0: f64 = (eq3_e180_d_b0 + var_ibtbt_db0);
        let eq3_e182_d_b1: f64 = (eq3_e180_d_b1 + var_ibtbt_db1);
        let eq3_e184: f64 = (eq3_e182 + var_itat);
        let eq3_e184_d_n0: f64 = (eq3_e182_d_n0 + var_itat_dn0);
        let eq3_e184_d_n1: f64 = (eq3_e182_d_n1 + var_itat_dn1);
        let eq3_e184_d_n2: f64 = (eq3_e182_d_n2 + var_itat_dn2);
        let eq3_e184_d_n3: f64 = (eq3_e182_d_n3 + var_itat_dn3);
        let eq3_e184_d_n4: f64 = (eq3_e182_d_n4 + var_itat_dn4);
        let eq3_e184_d_n5: f64 = (eq3_e182_d_n5 + var_itat_dn5);
        let eq3_e184_d_n6: f64 = (eq3_e182_d_n6 + var_itat_dn6);
        let eq3_e184_d_n7: f64 = (eq3_e182_d_n7 + var_itat_dn7);
        let eq3_e184_d_n8: f64 = (eq3_e182_d_n8 + var_itat_dn8);
        let eq3_e184_d_n9: f64 = (eq3_e182_d_n9 + var_itat_dn9);
        let eq3_e184_d_n10: f64 = (eq3_e182_d_n10 + var_itat_dn10);
        let eq3_e184_d_n11: f64 = (eq3_e182_d_n11 + var_itat_dn11);
        let eq3_e184_d_b0: f64 = (eq3_e182_d_b0 + var_itat_db0);
        let eq3_e184_d_b1: f64 = (eq3_e182_d_b1 + var_itat_db1);
        let eq3_e185: f64 = (p.p3 * eq3_e184);
        let eq3_e185_d_n0: f64 = (p.p3 * eq3_e184_d_n0);
        let eq3_e185_d_n1: f64 = (p.p3 * eq3_e184_d_n1);
        let eq3_e185_d_n2: f64 = (p.p3 * eq3_e184_d_n2);
        let eq3_e185_d_n3: f64 = (p.p3 * eq3_e184_d_n3);
        let eq3_e185_d_n4: f64 = (p.p3 * eq3_e184_d_n4);
        let eq3_e185_d_n5: f64 = (p.p3 * eq3_e184_d_n5);
        let eq3_e185_d_n6: f64 = (p.p3 * eq3_e184_d_n6);
        let eq3_e185_d_n7: f64 = (p.p3 * eq3_e184_d_n7);
        let eq3_e185_d_n8: f64 = (p.p3 * eq3_e184_d_n8);
        let eq3_e185_d_n9: f64 = (p.p3 * eq3_e184_d_n9);
        let eq3_e185_d_n10: f64 = (p.p3 * eq3_e184_d_n10);
        let eq3_e185_d_n11: f64 = (p.p3 * eq3_e184_d_n11);
        let eq3_e185_d_b0: f64 = (p.p3 * eq3_e184_d_b0);
        let eq3_e185_d_b1: f64 = (p.p3 * eq3_e184_d_b1);
        let eq3_e187: f64 = (eq3_e185 * p.p1);
        let eq3_e187_d_n0: f64 = (eq3_e185_d_n0 * p.p1);
        let eq3_e187_d_n1: f64 = (eq3_e185_d_n1 * p.p1);
        let eq3_e187_d_n2: f64 = (eq3_e185_d_n2 * p.p1);
        let eq3_e187_d_n3: f64 = (eq3_e185_d_n3 * p.p1);
        let eq3_e187_d_n4: f64 = (eq3_e185_d_n4 * p.p1);
        let eq3_e187_d_n5: f64 = (eq3_e185_d_n5 * p.p1);
        let eq3_e187_d_n6: f64 = (eq3_e185_d_n6 * p.p1);
        let eq3_e187_d_n7: f64 = (eq3_e185_d_n7 * p.p1);
        let eq3_e187_d_n8: f64 = (eq3_e185_d_n8 * p.p1);
        let eq3_e187_d_n9: f64 = (eq3_e185_d_n9 * p.p1);
        let eq3_e187_d_n10: f64 = (eq3_e185_d_n10 * p.p1);
        let eq3_e187_d_n11: f64 = (eq3_e185_d_n11 * p.p1);
        let eq3_e187_d_b0: f64 = (eq3_e185_d_b0 * p.p1);
        let eq3_e187_d_b1: f64 = (eq3_e185_d_b1 * p.p1);
        let eq3_value: f64 = eq3_e187;
        let eq3_node_derivatives: [f64; 12] = [eq3_e187_d_n0, eq3_e187_d_n1, eq3_e187_d_n2, eq3_e187_d_n3, eq3_e187_d_n4, eq3_e187_d_n5, eq3_e187_d_n6, eq3_e187_d_n7, eq3_e187_d_n8, eq3_e187_d_n9, eq3_e187_d_n10, eq3_e187_d_n11];
        let eq3_branch_derivatives: [f64; 2] = [eq3_e187_d_b0, eq3_e187_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(4),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let (eq4_e196, eq4_e196_d_n0, eq4_e196_d_n1, eq4_e196_d_n2, eq4_e196_d_n3, eq4_e196_d_n4, eq4_e196_d_n5, eq4_e196_d_n6, eq4_e196_d_n7, eq4_e196_d_n8, eq4_e196_d_n9, eq4_e196_d_n10, eq4_e196_d_n11, eq4_e196_d_b0, eq4_e196_d_b1,) = {
    if (var_guard117 != 0.0) {
        let eq4_e191: f64 = (-var_iztcb);
        let eq4_e192: f64 = (p.p3 * eq4_e191);
        let eq4_e192_d_n0: f64 = (p.p3 * (-var_iztcb_dn0));
        let eq4_e192_d_n1: f64 = (p.p3 * (-var_iztcb_dn1));
        let eq4_e192_d_n2: f64 = (p.p3 * (-var_iztcb_dn2));
        let eq4_e192_d_n3: f64 = (p.p3 * (-var_iztcb_dn3));
        let eq4_e192_d_n4: f64 = (p.p3 * (-var_iztcb_dn4));
        let eq4_e192_d_n5: f64 = (p.p3 * (-var_iztcb_dn5));
        let eq4_e192_d_n6: f64 = (p.p3 * (-var_iztcb_dn6));
        let eq4_e192_d_n7: f64 = (p.p3 * (-var_iztcb_dn7));
        let eq4_e192_d_n8: f64 = (p.p3 * (-var_iztcb_dn8));
        let eq4_e192_d_n9: f64 = (p.p3 * (-var_iztcb_dn9));
        let eq4_e192_d_n10: f64 = (p.p3 * (-var_iztcb_dn10));
        let eq4_e192_d_n11: f64 = (p.p3 * (-var_iztcb_dn11));
        let eq4_e192_d_b0: f64 = (p.p3 * (-var_iztcb_db0));
        let eq4_e192_d_b1: f64 = (p.p3 * (-var_iztcb_db1));
        let eq4_e194: f64 = (eq4_e192 * p.p1);
        let eq4_e194_d_n0: f64 = (eq4_e192_d_n0 * p.p1);
        let eq4_e194_d_n1: f64 = (eq4_e192_d_n1 * p.p1);
        let eq4_e194_d_n2: f64 = (eq4_e192_d_n2 * p.p1);
        let eq4_e194_d_n3: f64 = (eq4_e192_d_n3 * p.p1);
        let eq4_e194_d_n4: f64 = (eq4_e192_d_n4 * p.p1);
        let eq4_e194_d_n5: f64 = (eq4_e192_d_n5 * p.p1);
        let eq4_e194_d_n6: f64 = (eq4_e192_d_n6 * p.p1);
        let eq4_e194_d_n7: f64 = (eq4_e192_d_n7 * p.p1);
        let eq4_e194_d_n8: f64 = (eq4_e192_d_n8 * p.p1);
        let eq4_e194_d_n9: f64 = (eq4_e192_d_n9 * p.p1);
        let eq4_e194_d_n10: f64 = (eq4_e192_d_n10 * p.p1);
        let eq4_e194_d_n11: f64 = (eq4_e192_d_n11 * p.p1);
        let eq4_e194_d_b0: f64 = (eq4_e192_d_b0 * p.p1);
        let eq4_e194_d_b1: f64 = (eq4_e192_d_b1 * p.p1);
        (eq4_e194, eq4_e194_d_n0, eq4_e194_d_n1, eq4_e194_d_n2, eq4_e194_d_n3, eq4_e194_d_n4, eq4_e194_d_n5, eq4_e194_d_n6, eq4_e194_d_n7, eq4_e194_d_n8, eq4_e194_d_n9, eq4_e194_d_n10, eq4_e194_d_n11, eq4_e194_d_b0, eq4_e194_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e196;
        let eq4_node_derivatives: [f64; 12] = [eq4_e196_d_n0, eq4_e196_d_n1, eq4_e196_d_n2, eq4_e196_d_n3, eq4_e196_d_n4, eq4_e196_d_n5, eq4_e196_d_n6, eq4_e196_d_n7, eq4_e196_d_n8, eq4_e196_d_n9, eq4_e196_d_n10, eq4_e196_d_n11];
        let eq4_branch_derivatives: [f64; 2] = [eq4_e196_d_b0, eq4_e196_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e206, eq5_e206_d_n0, eq5_e206_d_n1, eq5_e206_d_n2, eq5_e206_d_n3, eq5_e206_d_n4, eq5_e206_d_n5, eq5_e206_d_n6, eq5_e206_d_n7, eq5_e206_d_n8, eq5_e206_d_n9, eq5_e206_d_n10, eq5_e206_d_n11, eq5_e206_d_b0, eq5_e206_d_b1,) = {
    if (var_guard117 == 0.0) {
        let eq5_e201: f64 = (-var_iztcb);
        let eq5_e202: f64 = (p.p3 * eq5_e201);
        let eq5_e202_d_n0: f64 = (p.p3 * (-var_iztcb_dn0));
        let eq5_e202_d_n1: f64 = (p.p3 * (-var_iztcb_dn1));
        let eq5_e202_d_n2: f64 = (p.p3 * (-var_iztcb_dn2));
        let eq5_e202_d_n3: f64 = (p.p3 * (-var_iztcb_dn3));
        let eq5_e202_d_n4: f64 = (p.p3 * (-var_iztcb_dn4));
        let eq5_e202_d_n5: f64 = (p.p3 * (-var_iztcb_dn5));
        let eq5_e202_d_n6: f64 = (p.p3 * (-var_iztcb_dn6));
        let eq5_e202_d_n7: f64 = (p.p3 * (-var_iztcb_dn7));
        let eq5_e202_d_n8: f64 = (p.p3 * (-var_iztcb_dn8));
        let eq5_e202_d_n9: f64 = (p.p3 * (-var_iztcb_dn9));
        let eq5_e202_d_n10: f64 = (p.p3 * (-var_iztcb_dn10));
        let eq5_e202_d_n11: f64 = (p.p3 * (-var_iztcb_dn11));
        let eq5_e202_d_b0: f64 = (p.p3 * (-var_iztcb_db0));
        let eq5_e202_d_b1: f64 = (p.p3 * (-var_iztcb_db1));
        let eq5_e204: f64 = (eq5_e202 * p.p1);
        let eq5_e204_d_n0: f64 = (eq5_e202_d_n0 * p.p1);
        let eq5_e204_d_n1: f64 = (eq5_e202_d_n1 * p.p1);
        let eq5_e204_d_n2: f64 = (eq5_e202_d_n2 * p.p1);
        let eq5_e204_d_n3: f64 = (eq5_e202_d_n3 * p.p1);
        let eq5_e204_d_n4: f64 = (eq5_e202_d_n4 * p.p1);
        let eq5_e204_d_n5: f64 = (eq5_e202_d_n5 * p.p1);
        let eq5_e204_d_n6: f64 = (eq5_e202_d_n6 * p.p1);
        let eq5_e204_d_n7: f64 = (eq5_e202_d_n7 * p.p1);
        let eq5_e204_d_n8: f64 = (eq5_e202_d_n8 * p.p1);
        let eq5_e204_d_n9: f64 = (eq5_e202_d_n9 * p.p1);
        let eq5_e204_d_n10: f64 = (eq5_e202_d_n10 * p.p1);
        let eq5_e204_d_n11: f64 = (eq5_e202_d_n11 * p.p1);
        let eq5_e204_d_b0: f64 = (eq5_e202_d_b0 * p.p1);
        let eq5_e204_d_b1: f64 = (eq5_e202_d_b1 * p.p1);
        (eq5_e204, eq5_e204_d_n0, eq5_e204_d_n1, eq5_e204_d_n2, eq5_e204_d_n3, eq5_e204_d_n4, eq5_e204_d_n5, eq5_e204_d_n6, eq5_e204_d_n7, eq5_e204_d_n8, eq5_e204_d_n9, eq5_e204_d_n10, eq5_e204_d_n11, eq5_e204_d_b0, eq5_e204_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e206;
        let eq5_node_derivatives: [f64; 12] = [eq5_e206_d_n0, eq5_e206_d_n1, eq5_e206_d_n2, eq5_e206_d_n3, eq5_e206_d_n4, eq5_e206_d_n5, eq5_e206_d_n6, eq5_e206_d_n7, eq5_e206_d_n8, eq5_e206_d_n9, eq5_e206_d_n10, eq5_e206_d_n11];
        let eq5_branch_derivatives: [f64; 2] = [eq5_e206_d_b0, eq5_e206_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let eq6_e209: f64 = (p.p3 * var_ib1b2);
        let eq6_e209_d_n0: f64 = (p.p3 * var_ib1b2_dn0);
        let eq6_e209_d_n1: f64 = (p.p3 * var_ib1b2_dn1);
        let eq6_e209_d_n2: f64 = (p.p3 * var_ib1b2_dn2);
        let eq6_e209_d_n3: f64 = (p.p3 * var_ib1b2_dn3);
        let eq6_e209_d_n4: f64 = (p.p3 * var_ib1b2_dn4);
        let eq6_e209_d_n5: f64 = (p.p3 * var_ib1b2_dn5);
        let eq6_e209_d_n6: f64 = (p.p3 * var_ib1b2_dn6);
        let eq6_e209_d_n7: f64 = (p.p3 * var_ib1b2_dn7);
        let eq6_e209_d_n8: f64 = (p.p3 * var_ib1b2_dn8);
        let eq6_e209_d_n9: f64 = (p.p3 * var_ib1b2_dn9);
        let eq6_e209_d_n10: f64 = (p.p3 * var_ib1b2_dn10);
        let eq6_e209_d_n11: f64 = (p.p3 * var_ib1b2_dn11);
        let eq6_e209_d_b0: f64 = (p.p3 * var_ib1b2_db0);
        let eq6_e209_d_b1: f64 = (p.p3 * var_ib1b2_db1);
        let eq6_e211: f64 = (eq6_e209 * p.p1);
        let eq6_e211_d_n0: f64 = (eq6_e209_d_n0 * p.p1);
        let eq6_e211_d_n1: f64 = (eq6_e209_d_n1 * p.p1);
        let eq6_e211_d_n2: f64 = (eq6_e209_d_n2 * p.p1);
        let eq6_e211_d_n3: f64 = (eq6_e209_d_n3 * p.p1);
        let eq6_e211_d_n4: f64 = (eq6_e209_d_n4 * p.p1);
        let eq6_e211_d_n5: f64 = (eq6_e209_d_n5 * p.p1);
        let eq6_e211_d_n6: f64 = (eq6_e209_d_n6 * p.p1);
        let eq6_e211_d_n7: f64 = (eq6_e209_d_n7 * p.p1);
        let eq6_e211_d_n8: f64 = (eq6_e209_d_n8 * p.p1);
        let eq6_e211_d_n9: f64 = (eq6_e209_d_n9 * p.p1);
        let eq6_e211_d_n10: f64 = (eq6_e209_d_n10 * p.p1);
        let eq6_e211_d_n11: f64 = (eq6_e209_d_n11 * p.p1);
        let eq6_e211_d_b0: f64 = (eq6_e209_d_b0 * p.p1);
        let eq6_e211_d_b1: f64 = (eq6_e209_d_b1 * p.p1);
        let eq6_value: f64 = eq6_e211;
        let eq6_node_derivatives: [f64; 12] = [eq6_e211_d_n0, eq6_e211_d_n1, eq6_e211_d_n2, eq6_e211_d_n3, eq6_e211_d_n4, eq6_e211_d_n5, eq6_e211_d_n6, eq6_e211_d_n7, eq6_e211_d_n8, eq6_e211_d_n9, eq6_e211_d_n10, eq6_e211_d_n11];
        let eq6_branch_derivatives: [f64; 2] = [eq6_e211_d_b0, eq6_e211_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let eq7_e214: f64 = (-1.0);
        let eq7_e216: f64 = (eq7_e214 * var_iavl);
        let eq7_e216_d_n0: f64 = (eq7_e214 * var_iavl_dn0);
        let eq7_e216_d_n1: f64 = (eq7_e214 * var_iavl_dn1);
        let eq7_e216_d_n2: f64 = (eq7_e214 * var_iavl_dn2);
        let eq7_e216_d_n3: f64 = (eq7_e214 * var_iavl_dn3);
        let eq7_e216_d_n4: f64 = (eq7_e214 * var_iavl_dn4);
        let eq7_e216_d_n5: f64 = (eq7_e214 * var_iavl_dn5);
        let eq7_e216_d_n6: f64 = (eq7_e214 * var_iavl_dn6);
        let eq7_e216_d_n7: f64 = (eq7_e214 * var_iavl_dn7);
        let eq7_e216_d_n8: f64 = (eq7_e214 * var_iavl_dn8);
        let eq7_e216_d_n9: f64 = (eq7_e214 * var_iavl_dn9);
        let eq7_e216_d_n10: f64 = (eq7_e214 * var_iavl_dn10);
        let eq7_e216_d_n11: f64 = (eq7_e214 * var_iavl_dn11);
        let eq7_e216_d_b0: f64 = (eq7_e214 * var_iavl_db0);
        let eq7_e216_d_b1: f64 = (eq7_e214 * var_iavl_db1);
        let eq7_e217: f64 = (p.p3 * eq7_e216);
        let eq7_e217_d_n0: f64 = (p.p3 * eq7_e216_d_n0);
        let eq7_e217_d_n1: f64 = (p.p3 * eq7_e216_d_n1);
        let eq7_e217_d_n2: f64 = (p.p3 * eq7_e216_d_n2);
        let eq7_e217_d_n3: f64 = (p.p3 * eq7_e216_d_n3);
        let eq7_e217_d_n4: f64 = (p.p3 * eq7_e216_d_n4);
        let eq7_e217_d_n5: f64 = (p.p3 * eq7_e216_d_n5);
        let eq7_e217_d_n6: f64 = (p.p3 * eq7_e216_d_n6);
        let eq7_e217_d_n7: f64 = (p.p3 * eq7_e216_d_n7);
        let eq7_e217_d_n8: f64 = (p.p3 * eq7_e216_d_n8);
        let eq7_e217_d_n9: f64 = (p.p3 * eq7_e216_d_n9);
        let eq7_e217_d_n10: f64 = (p.p3 * eq7_e216_d_n10);
        let eq7_e217_d_n11: f64 = (p.p3 * eq7_e216_d_n11);
        let eq7_e217_d_b0: f64 = (p.p3 * eq7_e216_d_b0);
        let eq7_e217_d_b1: f64 = (p.p3 * eq7_e216_d_b1);
        let eq7_e219: f64 = (eq7_e217 * p.p1);
        let eq7_e219_d_n0: f64 = (eq7_e217_d_n0 * p.p1);
        let eq7_e219_d_n1: f64 = (eq7_e217_d_n1 * p.p1);
        let eq7_e219_d_n2: f64 = (eq7_e217_d_n2 * p.p1);
        let eq7_e219_d_n3: f64 = (eq7_e217_d_n3 * p.p1);
        let eq7_e219_d_n4: f64 = (eq7_e217_d_n4 * p.p1);
        let eq7_e219_d_n5: f64 = (eq7_e217_d_n5 * p.p1);
        let eq7_e219_d_n6: f64 = (eq7_e217_d_n6 * p.p1);
        let eq7_e219_d_n7: f64 = (eq7_e217_d_n7 * p.p1);
        let eq7_e219_d_n8: f64 = (eq7_e217_d_n8 * p.p1);
        let eq7_e219_d_n9: f64 = (eq7_e217_d_n9 * p.p1);
        let eq7_e219_d_n10: f64 = (eq7_e217_d_n10 * p.p1);
        let eq7_e219_d_n11: f64 = (eq7_e217_d_n11 * p.p1);
        let eq7_e219_d_b0: f64 = (eq7_e217_d_b0 * p.p1);
        let eq7_e219_d_b1: f64 = (eq7_e217_d_b1 * p.p1);
        let eq7_value: f64 = eq7_e219;
        let eq7_node_derivatives: [f64; 12] = [eq7_e219_d_n0, eq7_e219_d_n1, eq7_e219_d_n2, eq7_e219_d_n3, eq7_e219_d_n4, eq7_e219_d_n5, eq7_e219_d_n6, eq7_e219_d_n7, eq7_e219_d_n8, eq7_e219_d_n9, eq7_e219_d_n10, eq7_e219_d_n11];
        let eq7_branch_derivatives: [f64; 2] = [eq7_e219_d_b0, eq7_e219_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
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
        var_i_cth: f64,
        var_i_cth_db0: f64,
        var_i_cth_db1: f64,
        var_i_cth_dn0: f64,
        var_i_cth_dn1: f64,
        var_i_cth_dn10: f64,
        var_i_cth_dn11: f64,
        var_i_cth_dn2: f64,
        var_i_cth_dn3: f64,
        var_i_cth_dn4: f64,
        var_i_cth_dn5: f64,
        var_i_cth_dn6: f64,
        var_i_cth_dn7: f64,
        var_i_cth_dn8: f64,
        var_i_cth_dn9: f64,
        var_p_rth: f64,
        var_p_rth_db0: f64,
        var_p_rth_db1: f64,
        var_p_rth_dn0: f64,
        var_p_rth_dn1: f64,
        var_p_rth_dn10: f64,
        var_p_rth_dn11: f64,
        var_p_rth_dn2: f64,
        var_p_rth_dn3: f64,
        var_p_rth_dn4: f64,
        var_p_rth_dn5: f64,
        var_p_rth_dn6: f64,
        var_p_rth_dn7: f64,
        var_p_rth_dn8: f64,
        var_p_rth_dn9: f64,
        var_power: f64,
        var_power_db0: f64,
        var_power_db1: f64,
        var_power_dn0: f64,
        var_power_dn1: f64,
        var_power_dn10: f64,
        var_power_dn11: f64,
        var_power_dn2: f64,
        var_power_dn3: f64,
        var_power_dn4: f64,
        var_power_dn5: f64,
        var_power_dn6: f64,
        var_power_dn7: f64,
        var_power_dn8: f64,
        var_power_dn9: f64,
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
        var_rbc_t: f64,
        var_rbc_t_db0: f64,
        var_rbc_t_db1: f64,
        var_rbc_t_dn0: f64,
        var_rbc_t_dn1: f64,
        var_rbc_t_dn10: f64,
        var_rbc_t_dn11: f64,
        var_rbc_t_dn2: f64,
        var_rbc_t_dn3: f64,
        var_rbc_t_dn4: f64,
        var_rbc_t_dn5: f64,
        var_rbc_t_dn6: f64,
        var_rbc_t_dn7: f64,
        var_rbc_t_dn8: f64,
        var_rbc_t_dn9: f64,
        var_re_t: f64,
        var_re_t_db0: f64,
        var_re_t_db1: f64,
        var_re_t_dn0: f64,
        var_re_t_dn1: f64,
        var_re_t_dn10: f64,
        var_re_t_dn11: f64,
        var_re_t_dn2: f64,
        var_re_t_dn3: f64,
        var_re_t_dn4: f64,
        var_re_t_dn5: f64,
        var_re_t_dn6: f64,
        var_re_t_dn7: f64,
        var_re_t_dn8: f64,
        var_re_t_dn9: f64,
        var_vbb1: f64,
        var_vbb1_db0: f64,
        var_vbb1_db1: f64,
        var_vbb1_dn0: f64,
        var_vbb1_dn1: f64,
        var_vbb1_dn10: f64,
        var_vbb1_dn11: f64,
        var_vbb1_dn2: f64,
        var_vbb1_dn3: f64,
        var_vbb1_dn4: f64,
        var_vbb1_dn5: f64,
        var_vbb1_dn6: f64,
        var_vbb1_dn7: f64,
        var_vbb1_dn8: f64,
        var_vbb1_dn9: f64,
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
        var_vee1: f64,
        var_vee1_db0: f64,
        var_vee1_db1: f64,
        var_vee1_dn0: f64,
        var_vee1_dn1: f64,
        var_vee1_dn10: f64,
        var_vee1_dn11: f64,
        var_vee1_dn2: f64,
        var_vee1_dn3: f64,
        var_vee1_dn4: f64,
        var_vee1_dn5: f64,
        var_vee1_dn6: f64,
        var_vee1_dn7: f64,
        var_vee1_dn8: f64,
        var_vee1_dn9: f64,
    ) {
        let eq8_e222: f64 = (p.p3 * var_vee1);
        let eq8_e222_d_n0: f64 = (p.p3 * var_vee1_dn0);
        let eq8_e222_d_n1: f64 = (p.p3 * var_vee1_dn1);
        let eq8_e222_d_n2: f64 = (p.p3 * var_vee1_dn2);
        let eq8_e222_d_n3: f64 = (p.p3 * var_vee1_dn3);
        let eq8_e222_d_n4: f64 = (p.p3 * var_vee1_dn4);
        let eq8_e222_d_n5: f64 = (p.p3 * var_vee1_dn5);
        let eq8_e222_d_n6: f64 = (p.p3 * var_vee1_dn6);
        let eq8_e222_d_n7: f64 = (p.p3 * var_vee1_dn7);
        let eq8_e222_d_n8: f64 = (p.p3 * var_vee1_dn8);
        let eq8_e222_d_n9: f64 = (p.p3 * var_vee1_dn9);
        let eq8_e222_d_n10: f64 = (p.p3 * var_vee1_dn10);
        let eq8_e222_d_n11: f64 = (p.p3 * var_vee1_dn11);
        let eq8_e222_d_b0: f64 = (p.p3 * var_vee1_db0);
        let eq8_e222_d_b1: f64 = (p.p3 * var_vee1_db1);
        let eq8_e224: f64 = (eq8_e222 / var_re_t);
        let __rspice_inv_cse_0: f64 = 1.0 / (var_re_t * var_re_t);
        let eq8_e224_d_n0: f64 = (((eq8_e222_d_n0 * var_re_t) - (eq8_e222 * var_re_t_dn0)) * __rspice_inv_cse_0);
        let eq8_e224_d_n1: f64 = (((eq8_e222_d_n1 * var_re_t) - (eq8_e222 * var_re_t_dn1)) * __rspice_inv_cse_0);
        let eq8_e224_d_n2: f64 = (((eq8_e222_d_n2 * var_re_t) - (eq8_e222 * var_re_t_dn2)) * __rspice_inv_cse_0);
        let eq8_e224_d_n3: f64 = (((eq8_e222_d_n3 * var_re_t) - (eq8_e222 * var_re_t_dn3)) * __rspice_inv_cse_0);
        let eq8_e224_d_n4: f64 = (((eq8_e222_d_n4 * var_re_t) - (eq8_e222 * var_re_t_dn4)) * __rspice_inv_cse_0);
        let eq8_e224_d_n5: f64 = (((eq8_e222_d_n5 * var_re_t) - (eq8_e222 * var_re_t_dn5)) * __rspice_inv_cse_0);
        let eq8_e224_d_n6: f64 = (((eq8_e222_d_n6 * var_re_t) - (eq8_e222 * var_re_t_dn6)) * __rspice_inv_cse_0);
        let eq8_e224_d_n7: f64 = (((eq8_e222_d_n7 * var_re_t) - (eq8_e222 * var_re_t_dn7)) * __rspice_inv_cse_0);
        let eq8_e224_d_n8: f64 = (((eq8_e222_d_n8 * var_re_t) - (eq8_e222 * var_re_t_dn8)) * __rspice_inv_cse_0);
        let eq8_e224_d_n9: f64 = (((eq8_e222_d_n9 * var_re_t) - (eq8_e222 * var_re_t_dn9)) * __rspice_inv_cse_0);
        let eq8_e224_d_n10: f64 = (((eq8_e222_d_n10 * var_re_t) - (eq8_e222 * var_re_t_dn10)) * __rspice_inv_cse_0);
        let eq8_e224_d_n11: f64 = (((eq8_e222_d_n11 * var_re_t) - (eq8_e222 * var_re_t_dn11)) * __rspice_inv_cse_0);
        let eq8_e224_d_b0: f64 = (((eq8_e222_d_b0 * var_re_t) - (eq8_e222 * var_re_t_db0)) * __rspice_inv_cse_0);
        let eq8_e224_d_b1: f64 = (((eq8_e222_d_b1 * var_re_t) - (eq8_e222 * var_re_t_db1)) * __rspice_inv_cse_0);
        let eq8_e226: f64 = (eq8_e224 * p.p1);
        let eq8_e226_d_n0: f64 = (eq8_e224_d_n0 * p.p1);
        let eq8_e226_d_n1: f64 = (eq8_e224_d_n1 * p.p1);
        let eq8_e226_d_n2: f64 = (eq8_e224_d_n2 * p.p1);
        let eq8_e226_d_n3: f64 = (eq8_e224_d_n3 * p.p1);
        let eq8_e226_d_n4: f64 = (eq8_e224_d_n4 * p.p1);
        let eq8_e226_d_n5: f64 = (eq8_e224_d_n5 * p.p1);
        let eq8_e226_d_n6: f64 = (eq8_e224_d_n6 * p.p1);
        let eq8_e226_d_n7: f64 = (eq8_e224_d_n7 * p.p1);
        let eq8_e226_d_n8: f64 = (eq8_e224_d_n8 * p.p1);
        let eq8_e226_d_n9: f64 = (eq8_e224_d_n9 * p.p1);
        let eq8_e226_d_n10: f64 = (eq8_e224_d_n10 * p.p1);
        let eq8_e226_d_n11: f64 = (eq8_e224_d_n11 * p.p1);
        let eq8_e226_d_b0: f64 = (eq8_e224_d_b0 * p.p1);
        let eq8_e226_d_b1: f64 = (eq8_e224_d_b1 * p.p1);
        let eq8_value: f64 = eq8_e226;
        let eq8_node_derivatives: [f64; 12] = [eq8_e226_d_n0, eq8_e226_d_n1, eq8_e226_d_n2, eq8_e226_d_n3, eq8_e226_d_n4, eq8_e226_d_n5, eq8_e226_d_n6, eq8_e226_d_n7, eq8_e226_d_n8, eq8_e226_d_n9, eq8_e226_d_n10, eq8_e226_d_n11];
        let eq8_branch_derivatives: [f64; 2] = [eq8_e226_d_b0, eq8_e226_d_b1];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(4),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let eq9_e229: f64 = (p.p3 * var_vbb1);
        let eq9_e229_d_n0: f64 = (p.p3 * var_vbb1_dn0);
        let eq9_e229_d_n1: f64 = (p.p3 * var_vbb1_dn1);
        let eq9_e229_d_n2: f64 = (p.p3 * var_vbb1_dn2);
        let eq9_e229_d_n3: f64 = (p.p3 * var_vbb1_dn3);
        let eq9_e229_d_n4: f64 = (p.p3 * var_vbb1_dn4);
        let eq9_e229_d_n5: f64 = (p.p3 * var_vbb1_dn5);
        let eq9_e229_d_n6: f64 = (p.p3 * var_vbb1_dn6);
        let eq9_e229_d_n7: f64 = (p.p3 * var_vbb1_dn7);
        let eq9_e229_d_n8: f64 = (p.p3 * var_vbb1_dn8);
        let eq9_e229_d_n9: f64 = (p.p3 * var_vbb1_dn9);
        let eq9_e229_d_n10: f64 = (p.p3 * var_vbb1_dn10);
        let eq9_e229_d_n11: f64 = (p.p3 * var_vbb1_dn11);
        let eq9_e229_d_b0: f64 = (p.p3 * var_vbb1_db0);
        let eq9_e229_d_b1: f64 = (p.p3 * var_vbb1_db1);
        let eq9_e231: f64 = (eq9_e229 / var_rbc_t);
        let __rspice_inv_cse_1: f64 = 1.0 / (var_rbc_t * var_rbc_t);
        let eq9_e231_d_n0: f64 = (((eq9_e229_d_n0 * var_rbc_t) - (eq9_e229 * var_rbc_t_dn0)) * __rspice_inv_cse_1);
        let eq9_e231_d_n1: f64 = (((eq9_e229_d_n1 * var_rbc_t) - (eq9_e229 * var_rbc_t_dn1)) * __rspice_inv_cse_1);
        let eq9_e231_d_n2: f64 = (((eq9_e229_d_n2 * var_rbc_t) - (eq9_e229 * var_rbc_t_dn2)) * __rspice_inv_cse_1);
        let eq9_e231_d_n3: f64 = (((eq9_e229_d_n3 * var_rbc_t) - (eq9_e229 * var_rbc_t_dn3)) * __rspice_inv_cse_1);
        let eq9_e231_d_n4: f64 = (((eq9_e229_d_n4 * var_rbc_t) - (eq9_e229 * var_rbc_t_dn4)) * __rspice_inv_cse_1);
        let eq9_e231_d_n5: f64 = (((eq9_e229_d_n5 * var_rbc_t) - (eq9_e229 * var_rbc_t_dn5)) * __rspice_inv_cse_1);
        let eq9_e231_d_n6: f64 = (((eq9_e229_d_n6 * var_rbc_t) - (eq9_e229 * var_rbc_t_dn6)) * __rspice_inv_cse_1);
        let eq9_e231_d_n7: f64 = (((eq9_e229_d_n7 * var_rbc_t) - (eq9_e229 * var_rbc_t_dn7)) * __rspice_inv_cse_1);
        let eq9_e231_d_n8: f64 = (((eq9_e229_d_n8 * var_rbc_t) - (eq9_e229 * var_rbc_t_dn8)) * __rspice_inv_cse_1);
        let eq9_e231_d_n9: f64 = (((eq9_e229_d_n9 * var_rbc_t) - (eq9_e229 * var_rbc_t_dn9)) * __rspice_inv_cse_1);
        let eq9_e231_d_n10: f64 = (((eq9_e229_d_n10 * var_rbc_t) - (eq9_e229 * var_rbc_t_dn10)) * __rspice_inv_cse_1);
        let eq9_e231_d_n11: f64 = (((eq9_e229_d_n11 * var_rbc_t) - (eq9_e229 * var_rbc_t_dn11)) * __rspice_inv_cse_1);
        let eq9_e231_d_b0: f64 = (((eq9_e229_d_b0 * var_rbc_t) - (eq9_e229 * var_rbc_t_db0)) * __rspice_inv_cse_1);
        let eq9_e231_d_b1: f64 = (((eq9_e229_d_b1 * var_rbc_t) - (eq9_e229 * var_rbc_t_db1)) * __rspice_inv_cse_1);
        let eq9_e233: f64 = (eq9_e231 * p.p1);
        let eq9_e233_d_n0: f64 = (eq9_e231_d_n0 * p.p1);
        let eq9_e233_d_n1: f64 = (eq9_e231_d_n1 * p.p1);
        let eq9_e233_d_n2: f64 = (eq9_e231_d_n2 * p.p1);
        let eq9_e233_d_n3: f64 = (eq9_e231_d_n3 * p.p1);
        let eq9_e233_d_n4: f64 = (eq9_e231_d_n4 * p.p1);
        let eq9_e233_d_n5: f64 = (eq9_e231_d_n5 * p.p1);
        let eq9_e233_d_n6: f64 = (eq9_e231_d_n6 * p.p1);
        let eq9_e233_d_n7: f64 = (eq9_e231_d_n7 * p.p1);
        let eq9_e233_d_n8: f64 = (eq9_e231_d_n8 * p.p1);
        let eq9_e233_d_n9: f64 = (eq9_e231_d_n9 * p.p1);
        let eq9_e233_d_n10: f64 = (eq9_e231_d_n10 * p.p1);
        let eq9_e233_d_n11: f64 = (eq9_e231_d_n11 * p.p1);
        let eq9_e233_d_b0: f64 = (eq9_e231_d_b0 * p.p1);
        let eq9_e233_d_b1: f64 = (eq9_e231_d_b1 * p.p1);
        let eq9_value: f64 = eq9_e233;
        let eq9_node_derivatives: [f64; 12] = [eq9_e233_d_n0, eq9_e233_d_n1, eq9_e233_d_n2, eq9_e233_d_n3, eq9_e233_d_n4, eq9_e233_d_n5, eq9_e233_d_n6, eq9_e233_d_n7, eq9_e233_d_n8, eq9_e233_d_n9, eq9_e233_d_n10, eq9_e233_d_n11];
        let eq9_branch_derivatives: [f64; 2] = [eq9_e233_d_b0, eq9_e233_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(5),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let eq10_value: f64 = var_p_rth;
        let eq10_node_derivatives: [f64; 12] = [var_p_rth_dn0, var_p_rth_dn1, var_p_rth_dn2, var_p_rth_dn3, var_p_rth_dn4, var_p_rth_dn5, var_p_rth_dn6, var_p_rth_dn7, var_p_rth_dn8, var_p_rth_dn9, var_p_rth_dn10, var_p_rth_dn11];
        let eq10_branch_derivatives: [f64; 2] = [var_p_rth_db0, var_p_rth_db1];
        stamper.stamp_current_dense_local(
            Some(3),
            None,
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_value: f64 = var_i_cth;
        let eq11_node_derivatives: [f64; 12] = [var_i_cth_dn0, var_i_cth_dn1, var_i_cth_dn2, var_i_cth_dn3, var_i_cth_dn4, var_i_cth_dn5, var_i_cth_dn6, var_i_cth_dn7, var_i_cth_dn8, var_i_cth_dn9, var_i_cth_dn10, var_i_cth_dn11];
        let eq11_branch_derivatives: [f64; 2] = [var_i_cth_db0, var_i_cth_db1];
        stamper.stamp_current_dense_local(
            Some(3),
            None,
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e237: f64 = (-1.0);
        let eq12_e239: f64 = (eq12_e237 * var_power);
        let eq12_e239_d_n0: f64 = (eq12_e237 * var_power_dn0);
        let eq12_e239_d_n1: f64 = (eq12_e237 * var_power_dn1);
        let eq12_e239_d_n2: f64 = (eq12_e237 * var_power_dn2);
        let eq12_e239_d_n3: f64 = (eq12_e237 * var_power_dn3);
        let eq12_e239_d_n4: f64 = (eq12_e237 * var_power_dn4);
        let eq12_e239_d_n5: f64 = (eq12_e237 * var_power_dn5);
        let eq12_e239_d_n6: f64 = (eq12_e237 * var_power_dn6);
        let eq12_e239_d_n7: f64 = (eq12_e237 * var_power_dn7);
        let eq12_e239_d_n8: f64 = (eq12_e237 * var_power_dn8);
        let eq12_e239_d_n9: f64 = (eq12_e237 * var_power_dn9);
        let eq12_e239_d_n10: f64 = (eq12_e237 * var_power_dn10);
        let eq12_e239_d_n11: f64 = (eq12_e237 * var_power_dn11);
        let eq12_e239_d_b0: f64 = (eq12_e237 * var_power_db0);
        let eq12_e239_d_b1: f64 = (eq12_e237 * var_power_db1);
        let eq12_e241: f64 = (eq12_e239 * p.p1);
        let eq12_e241_d_n0: f64 = (eq12_e239_d_n0 * p.p1);
        let eq12_e241_d_n1: f64 = (eq12_e239_d_n1 * p.p1);
        let eq12_e241_d_n2: f64 = (eq12_e239_d_n2 * p.p1);
        let eq12_e241_d_n3: f64 = (eq12_e239_d_n3 * p.p1);
        let eq12_e241_d_n4: f64 = (eq12_e239_d_n4 * p.p1);
        let eq12_e241_d_n5: f64 = (eq12_e239_d_n5 * p.p1);
        let eq12_e241_d_n6: f64 = (eq12_e239_d_n6 * p.p1);
        let eq12_e241_d_n7: f64 = (eq12_e239_d_n7 * p.p1);
        let eq12_e241_d_n8: f64 = (eq12_e239_d_n8 * p.p1);
        let eq12_e241_d_n9: f64 = (eq12_e239_d_n9 * p.p1);
        let eq12_e241_d_n10: f64 = (eq12_e239_d_n10 * p.p1);
        let eq12_e241_d_n11: f64 = (eq12_e239_d_n11 * p.p1);
        let eq12_e241_d_b0: f64 = (eq12_e239_d_b0 * p.p1);
        let eq12_e241_d_b1: f64 = (eq12_e239_d_b1 * p.p1);
        let eq12_value: f64 = eq12_e241;
        let eq12_node_derivatives: [f64; 12] = [eq12_e241_d_n0, eq12_e241_d_n1, eq12_e241_d_n2, eq12_e241_d_n3, eq12_e241_d_n4, eq12_e241_d_n5, eq12_e241_d_n6, eq12_e241_d_n7, eq12_e241_d_n8, eq12_e241_d_n9, eq12_e241_d_n10, eq12_e241_d_n11];
        let eq12_branch_derivatives: [f64; 2] = [eq12_e241_d_b0, eq12_e241_d_b1];
        stamper.stamp_current_dense_local(
            Some(3),
            None,
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq13_e245: f64 = (var_qte + var_qbe);
        let eq13_e245_d_n0: f64 = (var_qte_dn0 + var_qbe_dn0);
        let eq13_e245_d_n1: f64 = (var_qte_dn1 + var_qbe_dn1);
        let eq13_e245_d_n2: f64 = (var_qte_dn2 + var_qbe_dn2);
        let eq13_e245_d_n3: f64 = (var_qte_dn3 + var_qbe_dn3);
        let eq13_e245_d_n4: f64 = (var_qte_dn4 + var_qbe_dn4);
        let eq13_e245_d_n5: f64 = (var_qte_dn5 + var_qbe_dn5);
        let eq13_e245_d_n6: f64 = (var_qte_dn6 + var_qbe_dn6);
        let eq13_e245_d_n7: f64 = (var_qte_dn7 + var_qbe_dn7);
        let eq13_e245_d_n8: f64 = (var_qte_dn8 + var_qbe_dn8);
        let eq13_e245_d_n9: f64 = (var_qte_dn9 + var_qbe_dn9);
        let eq13_e245_d_n10: f64 = (var_qte_dn10 + var_qbe_dn10);
        let eq13_e245_d_n11: f64 = (var_qte_dn11 + var_qbe_dn11);
        let eq13_e245_d_b0: f64 = (var_qte_db0 + var_qbe_db0);
        let eq13_e245_d_b1: f64 = (var_qte_db1 + var_qbe_db1);
        let eq13_e247: f64 = (eq13_e245 + var_qe);
        let eq13_e247_d_n0: f64 = (eq13_e245_d_n0 + var_qe_dn0);
        let eq13_e247_d_n1: f64 = (eq13_e245_d_n1 + var_qe_dn1);
        let eq13_e247_d_n2: f64 = (eq13_e245_d_n2 + var_qe_dn2);
        let eq13_e247_d_n3: f64 = (eq13_e245_d_n3 + var_qe_dn3);
        let eq13_e247_d_n4: f64 = (eq13_e245_d_n4 + var_qe_dn4);
        let eq13_e247_d_n5: f64 = (eq13_e245_d_n5 + var_qe_dn5);
        let eq13_e247_d_n6: f64 = (eq13_e245_d_n6 + var_qe_dn6);
        let eq13_e247_d_n7: f64 = (eq13_e245_d_n7 + var_qe_dn7);
        let eq13_e247_d_n8: f64 = (eq13_e245_d_n8 + var_qe_dn8);
        let eq13_e247_d_n9: f64 = (eq13_e245_d_n9 + var_qe_dn9);
        let eq13_e247_d_n10: f64 = (eq13_e245_d_n10 + var_qe_dn10);
        let eq13_e247_d_n11: f64 = (eq13_e245_d_n11 + var_qe_dn11);
        let eq13_e247_d_b0: f64 = (eq13_e245_d_b0 + var_qe_db0);
        let eq13_e247_d_b1: f64 = (eq13_e245_d_b1 + var_qe_db1);
        let eq13_e248: f64 = (p.p3 * eq13_e247);
        let eq13_e248_d_n0: f64 = (p.p3 * eq13_e247_d_n0);
        let eq13_e248_d_n1: f64 = (p.p3 * eq13_e247_d_n1);
        let eq13_e248_d_n2: f64 = (p.p3 * eq13_e247_d_n2);
        let eq13_e248_d_n3: f64 = (p.p3 * eq13_e247_d_n3);
        let eq13_e248_d_n4: f64 = (p.p3 * eq13_e247_d_n4);
        let eq13_e248_d_n5: f64 = (p.p3 * eq13_e247_d_n5);
        let eq13_e248_d_n6: f64 = (p.p3 * eq13_e247_d_n6);
        let eq13_e248_d_n7: f64 = (p.p3 * eq13_e247_d_n7);
        let eq13_e248_d_n8: f64 = (p.p3 * eq13_e247_d_n8);
        let eq13_e248_d_n9: f64 = (p.p3 * eq13_e247_d_n9);
        let eq13_e248_d_n10: f64 = (p.p3 * eq13_e247_d_n10);
        let eq13_e248_d_n11: f64 = (p.p3 * eq13_e247_d_n11);
        let eq13_e248_d_b0: f64 = (p.p3 * eq13_e247_d_b0);
        let eq13_e248_d_b1: f64 = (p.p3 * eq13_e247_d_b1);
        let eq13_e249: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq13_e248);
        let eq13_e251: f64 = (eq13_e249 * p.p1);
        let eq13_e251_d_n0: f64 = ((eq13_e248_d_n0 * ddt_scale) * p.p1);
        let eq13_e251_d_n1: f64 = ((eq13_e248_d_n1 * ddt_scale) * p.p1);
        let eq13_e251_d_n2: f64 = ((eq13_e248_d_n2 * ddt_scale) * p.p1);
        let eq13_e251_d_n3: f64 = ((eq13_e248_d_n3 * ddt_scale) * p.p1);
        let eq13_e251_d_n4: f64 = ((eq13_e248_d_n4 * ddt_scale) * p.p1);
        let eq13_e251_d_n5: f64 = ((eq13_e248_d_n5 * ddt_scale) * p.p1);
        let eq13_e251_d_n6: f64 = ((eq13_e248_d_n6 * ddt_scale) * p.p1);
        let eq13_e251_d_n7: f64 = ((eq13_e248_d_n7 * ddt_scale) * p.p1);
        let eq13_e251_d_n8: f64 = ((eq13_e248_d_n8 * ddt_scale) * p.p1);
        let eq13_e251_d_n9: f64 = ((eq13_e248_d_n9 * ddt_scale) * p.p1);
        let eq13_e251_d_n10: f64 = ((eq13_e248_d_n10 * ddt_scale) * p.p1);
        let eq13_e251_d_n11: f64 = ((eq13_e248_d_n11 * ddt_scale) * p.p1);
        let eq13_e251_d_b0: f64 = ((eq13_e248_d_b0 * ddt_scale) * p.p1);
        let eq13_e251_d_b1: f64 = ((eq13_e248_d_b1 * ddt_scale) * p.p1);
        let eq13_value: f64 = eq13_e251;
        let eq13_node_derivatives: [f64; 12] = [eq13_e251_d_n0, eq13_e251_d_n1, eq13_e251_d_n2, eq13_e251_d_n3, eq13_e251_d_n4, eq13_e251_d_n5, eq13_e251_d_n6, eq13_e251_d_n7, eq13_e251_d_n8, eq13_e251_d_n9, eq13_e251_d_n10, eq13_e251_d_n11];
        let eq13_branch_derivatives: [f64; 2] = [eq13_e251_d_b0, eq13_e251_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(4),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let eq14_e254: f64 = (p.p3 * var_qte_s);
        let eq14_e254_d_n0: f64 = (p.p3 * var_qte_s_dn0);
        let eq14_e254_d_n1: f64 = (p.p3 * var_qte_s_dn1);
        let eq14_e254_d_n2: f64 = (p.p3 * var_qte_s_dn2);
        let eq14_e254_d_n3: f64 = (p.p3 * var_qte_s_dn3);
        let eq14_e254_d_n4: f64 = (p.p3 * var_qte_s_dn4);
        let eq14_e254_d_n5: f64 = (p.p3 * var_qte_s_dn5);
        let eq14_e254_d_n6: f64 = (p.p3 * var_qte_s_dn6);
        let eq14_e254_d_n7: f64 = (p.p3 * var_qte_s_dn7);
        let eq14_e254_d_n8: f64 = (p.p3 * var_qte_s_dn8);
        let eq14_e254_d_n9: f64 = (p.p3 * var_qte_s_dn9);
        let eq14_e254_d_n10: f64 = (p.p3 * var_qte_s_dn10);
        let eq14_e254_d_n11: f64 = (p.p3 * var_qte_s_dn11);
        let eq14_e254_d_b0: f64 = (p.p3 * var_qte_s_db0);
        let eq14_e254_d_b1: f64 = (p.p3 * var_qte_s_db1);
        let eq14_e255: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq14_e254);
        let eq14_e257: f64 = (eq14_e255 * p.p1);
        let eq14_e257_d_n0: f64 = ((eq14_e254_d_n0 * ddt_scale) * p.p1);
        let eq14_e257_d_n1: f64 = ((eq14_e254_d_n1 * ddt_scale) * p.p1);
        let eq14_e257_d_n2: f64 = ((eq14_e254_d_n2 * ddt_scale) * p.p1);
        let eq14_e257_d_n3: f64 = ((eq14_e254_d_n3 * ddt_scale) * p.p1);
        let eq14_e257_d_n4: f64 = ((eq14_e254_d_n4 * ddt_scale) * p.p1);
        let eq14_e257_d_n5: f64 = ((eq14_e254_d_n5 * ddt_scale) * p.p1);
        let eq14_e257_d_n6: f64 = ((eq14_e254_d_n6 * ddt_scale) * p.p1);
        let eq14_e257_d_n7: f64 = ((eq14_e254_d_n7 * ddt_scale) * p.p1);
        let eq14_e257_d_n8: f64 = ((eq14_e254_d_n8 * ddt_scale) * p.p1);
        let eq14_e257_d_n9: f64 = ((eq14_e254_d_n9 * ddt_scale) * p.p1);
        let eq14_e257_d_n10: f64 = ((eq14_e254_d_n10 * ddt_scale) * p.p1);
        let eq14_e257_d_n11: f64 = ((eq14_e254_d_n11 * ddt_scale) * p.p1);
        let eq14_e257_d_b0: f64 = ((eq14_e254_d_b0 * ddt_scale) * p.p1);
        let eq14_e257_d_b1: f64 = ((eq14_e254_d_b1 * ddt_scale) * p.p1);
        let eq14_value: f64 = eq14_e257;
        let eq14_node_derivatives: [f64; 12] = [eq14_e257_d_n0, eq14_e257_d_n1, eq14_e257_d_n2, eq14_e257_d_n3, eq14_e257_d_n4, eq14_e257_d_n5, eq14_e257_d_n6, eq14_e257_d_n7, eq14_e257_d_n8, eq14_e257_d_n9, eq14_e257_d_n10, eq14_e257_d_n11];
        let eq14_branch_derivatives: [f64; 2] = [eq14_e257_d_b0, eq14_e257_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(4),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let eq15_e261: f64 = (var_qtc + var_qbc);
        let eq15_e261_d_n0: f64 = (var_qtc_dn0 + var_qbc_dn0);
        let eq15_e261_d_n1: f64 = (var_qtc_dn1 + var_qbc_dn1);
        let eq15_e261_d_n2: f64 = (var_qtc_dn2 + var_qbc_dn2);
        let eq15_e261_d_n3: f64 = (var_qtc_dn3 + var_qbc_dn3);
        let eq15_e261_d_n4: f64 = (var_qtc_dn4 + var_qbc_dn4);
        let eq15_e261_d_n5: f64 = (var_qtc_dn5 + var_qbc_dn5);
        let eq15_e261_d_n6: f64 = (var_qtc_dn6 + var_qbc_dn6);
        let eq15_e261_d_n7: f64 = (var_qtc_dn7 + var_qbc_dn7);
        let eq15_e261_d_n8: f64 = (var_qtc_dn8 + var_qbc_dn8);
        let eq15_e261_d_n9: f64 = (var_qtc_dn9 + var_qbc_dn9);
        let eq15_e261_d_n10: f64 = (var_qtc_dn10 + var_qbc_dn10);
        let eq15_e261_d_n11: f64 = (var_qtc_dn11 + var_qbc_dn11);
        let eq15_e261_d_b0: f64 = (var_qtc_db0 + var_qbc_db0);
        let eq15_e261_d_b1: f64 = (var_qtc_db1 + var_qbc_db1);
        let eq15_e263: f64 = (eq15_e261 + var_qepi);
        let eq15_e263_d_n0: f64 = (eq15_e261_d_n0 + var_qepi_dn0);
        let eq15_e263_d_n1: f64 = (eq15_e261_d_n1 + var_qepi_dn1);
        let eq15_e263_d_n2: f64 = (eq15_e261_d_n2 + var_qepi_dn2);
        let eq15_e263_d_n3: f64 = (eq15_e261_d_n3 + var_qepi_dn3);
        let eq15_e263_d_n4: f64 = (eq15_e261_d_n4 + var_qepi_dn4);
        let eq15_e263_d_n5: f64 = (eq15_e261_d_n5 + var_qepi_dn5);
        let eq15_e263_d_n6: f64 = (eq15_e261_d_n6 + var_qepi_dn6);
        let eq15_e263_d_n7: f64 = (eq15_e261_d_n7 + var_qepi_dn7);
        let eq15_e263_d_n8: f64 = (eq15_e261_d_n8 + var_qepi_dn8);
        let eq15_e263_d_n9: f64 = (eq15_e261_d_n9 + var_qepi_dn9);
        let eq15_e263_d_n10: f64 = (eq15_e261_d_n10 + var_qepi_dn10);
        let eq15_e263_d_n11: f64 = (eq15_e261_d_n11 + var_qepi_dn11);
        let eq15_e263_d_b0: f64 = (eq15_e261_d_b0 + var_qepi_db0);
        let eq15_e263_d_b1: f64 = (eq15_e261_d_b1 + var_qepi_db1);
        let eq15_e264: f64 = (p.p3 * eq15_e263);
        let eq15_e264_d_n0: f64 = (p.p3 * eq15_e263_d_n0);
        let eq15_e264_d_n1: f64 = (p.p3 * eq15_e263_d_n1);
        let eq15_e264_d_n2: f64 = (p.p3 * eq15_e263_d_n2);
        let eq15_e264_d_n3: f64 = (p.p3 * eq15_e263_d_n3);
        let eq15_e264_d_n4: f64 = (p.p3 * eq15_e263_d_n4);
        let eq15_e264_d_n5: f64 = (p.p3 * eq15_e263_d_n5);
        let eq15_e264_d_n6: f64 = (p.p3 * eq15_e263_d_n6);
        let eq15_e264_d_n7: f64 = (p.p3 * eq15_e263_d_n7);
        let eq15_e264_d_n8: f64 = (p.p3 * eq15_e263_d_n8);
        let eq15_e264_d_n9: f64 = (p.p3 * eq15_e263_d_n9);
        let eq15_e264_d_n10: f64 = (p.p3 * eq15_e263_d_n10);
        let eq15_e264_d_n11: f64 = (p.p3 * eq15_e263_d_n11);
        let eq15_e264_d_b0: f64 = (p.p3 * eq15_e263_d_b0);
        let eq15_e264_d_b1: f64 = (p.p3 * eq15_e263_d_b1);
        let eq15_e265: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq15_e264);
        let eq15_e267: f64 = (eq15_e265 * p.p1);
        let eq15_e267_d_n0: f64 = ((eq15_e264_d_n0 * ddt_scale) * p.p1);
        let eq15_e267_d_n1: f64 = ((eq15_e264_d_n1 * ddt_scale) * p.p1);
        let eq15_e267_d_n2: f64 = ((eq15_e264_d_n2 * ddt_scale) * p.p1);
        let eq15_e267_d_n3: f64 = ((eq15_e264_d_n3 * ddt_scale) * p.p1);
        let eq15_e267_d_n4: f64 = ((eq15_e264_d_n4 * ddt_scale) * p.p1);
        let eq15_e267_d_n5: f64 = ((eq15_e264_d_n5 * ddt_scale) * p.p1);
        let eq15_e267_d_n6: f64 = ((eq15_e264_d_n6 * ddt_scale) * p.p1);
        let eq15_e267_d_n7: f64 = ((eq15_e264_d_n7 * ddt_scale) * p.p1);
        let eq15_e267_d_n8: f64 = ((eq15_e264_d_n8 * ddt_scale) * p.p1);
        let eq15_e267_d_n9: f64 = ((eq15_e264_d_n9 * ddt_scale) * p.p1);
        let eq15_e267_d_n10: f64 = ((eq15_e264_d_n10 * ddt_scale) * p.p1);
        let eq15_e267_d_n11: f64 = ((eq15_e264_d_n11 * ddt_scale) * p.p1);
        let eq15_e267_d_b0: f64 = ((eq15_e264_d_b0 * ddt_scale) * p.p1);
        let eq15_e267_d_b1: f64 = ((eq15_e264_d_b1 * ddt_scale) * p.p1);
        let eq15_value: f64 = eq15_e267;
        let eq15_node_derivatives: [f64; 12] = [eq15_e267_d_n0, eq15_e267_d_n1, eq15_e267_d_n2, eq15_e267_d_n3, eq15_e267_d_n4, eq15_e267_d_n5, eq15_e267_d_n6, eq15_e267_d_n7, eq15_e267_d_n8, eq15_e267_d_n9, eq15_e267_d_n10, eq15_e267_d_n11];
        let eq15_branch_derivatives: [f64; 2] = [eq15_e267_d_b0, eq15_e267_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let eq16_e270: f64 = (p.p3 * var_qb1b2);
        let eq16_e270_d_n0: f64 = (p.p3 * var_qb1b2_dn0);
        let eq16_e270_d_n1: f64 = (p.p3 * var_qb1b2_dn1);
        let eq16_e270_d_n2: f64 = (p.p3 * var_qb1b2_dn2);
        let eq16_e270_d_n3: f64 = (p.p3 * var_qb1b2_dn3);
        let eq16_e270_d_n4: f64 = (p.p3 * var_qb1b2_dn4);
        let eq16_e270_d_n5: f64 = (p.p3 * var_qb1b2_dn5);
        let eq16_e270_d_n6: f64 = (p.p3 * var_qb1b2_dn6);
        let eq16_e270_d_n7: f64 = (p.p3 * var_qb1b2_dn7);
        let eq16_e270_d_n8: f64 = (p.p3 * var_qb1b2_dn8);
        let eq16_e270_d_n9: f64 = (p.p3 * var_qb1b2_dn9);
        let eq16_e270_d_n10: f64 = (p.p3 * var_qb1b2_dn10);
        let eq16_e270_d_n11: f64 = (p.p3 * var_qb1b2_dn11);
        let eq16_e270_d_b0: f64 = (p.p3 * var_qb1b2_db0);
        let eq16_e270_d_b1: f64 = (p.p3 * var_qb1b2_db1);
        let eq16_e271: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq16_e270);
        let eq16_e273: f64 = (eq16_e271 * p.p1);
        let eq16_e273_d_n0: f64 = ((eq16_e270_d_n0 * ddt_scale) * p.p1);
        let eq16_e273_d_n1: f64 = ((eq16_e270_d_n1 * ddt_scale) * p.p1);
        let eq16_e273_d_n2: f64 = ((eq16_e270_d_n2 * ddt_scale) * p.p1);
        let eq16_e273_d_n3: f64 = ((eq16_e270_d_n3 * ddt_scale) * p.p1);
        let eq16_e273_d_n4: f64 = ((eq16_e270_d_n4 * ddt_scale) * p.p1);
        let eq16_e273_d_n5: f64 = ((eq16_e270_d_n5 * ddt_scale) * p.p1);
        let eq16_e273_d_n6: f64 = ((eq16_e270_d_n6 * ddt_scale) * p.p1);
        let eq16_e273_d_n7: f64 = ((eq16_e270_d_n7 * ddt_scale) * p.p1);
        let eq16_e273_d_n8: f64 = ((eq16_e270_d_n8 * ddt_scale) * p.p1);
        let eq16_e273_d_n9: f64 = ((eq16_e270_d_n9 * ddt_scale) * p.p1);
        let eq16_e273_d_n10: f64 = ((eq16_e270_d_n10 * ddt_scale) * p.p1);
        let eq16_e273_d_n11: f64 = ((eq16_e270_d_n11 * ddt_scale) * p.p1);
        let eq16_e273_d_b0: f64 = ((eq16_e270_d_b0 * ddt_scale) * p.p1);
        let eq16_e273_d_b1: f64 = ((eq16_e270_d_b1 * ddt_scale) * p.p1);
        let eq16_value: f64 = eq16_e273;
        let eq16_node_derivatives: [f64; 12] = [eq16_e273_d_n0, eq16_e273_d_n1, eq16_e273_d_n2, eq16_e273_d_n3, eq16_e273_d_n4, eq16_e273_d_n5, eq16_e273_d_n6, eq16_e273_d_n7, eq16_e273_d_n8, eq16_e273_d_n9, eq16_e273_d_n10, eq16_e273_d_n11];
        let eq16_branch_derivatives: [f64; 2] = [eq16_e273_d_b0, eq16_e273_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq16_value),
            &eq16_node_derivatives,
            &eq16_branch_derivatives,
            multiplicity,
        );
        let eq17_e276: f64 = (p.p3 * p.p68);
        let eq17_e278: f64 = (eq17_e276 * var_vbe);
        let eq17_e278_d_n0: f64 = (eq17_e276 * var_vbe_dn0);
        let eq17_e278_d_n1: f64 = (eq17_e276 * var_vbe_dn1);
        let eq17_e278_d_n2: f64 = (eq17_e276 * var_vbe_dn2);
        let eq17_e278_d_n3: f64 = (eq17_e276 * var_vbe_dn3);
        let eq17_e278_d_n4: f64 = (eq17_e276 * var_vbe_dn4);
        let eq17_e278_d_n5: f64 = (eq17_e276 * var_vbe_dn5);
        let eq17_e278_d_n6: f64 = (eq17_e276 * var_vbe_dn6);
        let eq17_e278_d_n7: f64 = (eq17_e276 * var_vbe_dn7);
        let eq17_e278_d_n8: f64 = (eq17_e276 * var_vbe_dn8);
        let eq17_e278_d_n9: f64 = (eq17_e276 * var_vbe_dn9);
        let eq17_e278_d_n10: f64 = (eq17_e276 * var_vbe_dn10);
        let eq17_e278_d_n11: f64 = (eq17_e276 * var_vbe_dn11);
        let eq17_e278_d_b0: f64 = (eq17_e276 * var_vbe_db0);
        let eq17_e278_d_b1: f64 = (eq17_e276 * var_vbe_db1);
        let eq17_e279: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq17_e278);
        let eq17_e281: f64 = (eq17_e279 * p.p1);
        let eq17_e281_d_n0: f64 = ((eq17_e278_d_n0 * ddt_scale) * p.p1);
        let eq17_e281_d_n1: f64 = ((eq17_e278_d_n1 * ddt_scale) * p.p1);
        let eq17_e281_d_n2: f64 = ((eq17_e278_d_n2 * ddt_scale) * p.p1);
        let eq17_e281_d_n3: f64 = ((eq17_e278_d_n3 * ddt_scale) * p.p1);
        let eq17_e281_d_n4: f64 = ((eq17_e278_d_n4 * ddt_scale) * p.p1);
        let eq17_e281_d_n5: f64 = ((eq17_e278_d_n5 * ddt_scale) * p.p1);
        let eq17_e281_d_n6: f64 = ((eq17_e278_d_n6 * ddt_scale) * p.p1);
        let eq17_e281_d_n7: f64 = ((eq17_e278_d_n7 * ddt_scale) * p.p1);
        let eq17_e281_d_n8: f64 = ((eq17_e278_d_n8 * ddt_scale) * p.p1);
        let eq17_e281_d_n9: f64 = ((eq17_e278_d_n9 * ddt_scale) * p.p1);
        let eq17_e281_d_n10: f64 = ((eq17_e278_d_n10 * ddt_scale) * p.p1);
        let eq17_e281_d_n11: f64 = ((eq17_e278_d_n11 * ddt_scale) * p.p1);
        let eq17_e281_d_b0: f64 = ((eq17_e278_d_b0 * ddt_scale) * p.p1);
        let eq17_e281_d_b1: f64 = ((eq17_e278_d_b1 * ddt_scale) * p.p1);
        let eq17_value: f64 = eq17_e281;
        let eq17_node_derivatives: [f64; 12] = [eq17_e281_d_n0, eq17_e281_d_n1, eq17_e281_d_n2, eq17_e281_d_n3, eq17_e281_d_n4, eq17_e281_d_n5, eq17_e281_d_n6, eq17_e281_d_n7, eq17_e281_d_n8, eq17_e281_d_n9, eq17_e281_d_n10, eq17_e281_d_n11];
        let eq17_branch_derivatives: [f64; 2] = [eq17_e281_d_b0, eq17_e281_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(2),
            multiplicity * (eq17_value),
            &eq17_node_derivatives,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let eq18_e284: f64 = (p.p3 * p.p77);
        let eq18_e286: f64 = (eq18_e284 * var_vbc);
        let eq18_e286_d_n0: f64 = (eq18_e284 * var_vbc_dn0);
        let eq18_e286_d_n1: f64 = (eq18_e284 * var_vbc_dn1);
        let eq18_e286_d_n2: f64 = (eq18_e284 * var_vbc_dn2);
        let eq18_e286_d_n3: f64 = (eq18_e284 * var_vbc_dn3);
        let eq18_e286_d_n4: f64 = (eq18_e284 * var_vbc_dn4);
        let eq18_e286_d_n5: f64 = (eq18_e284 * var_vbc_dn5);
        let eq18_e286_d_n6: f64 = (eq18_e284 * var_vbc_dn6);
        let eq18_e286_d_n7: f64 = (eq18_e284 * var_vbc_dn7);
        let eq18_e286_d_n8: f64 = (eq18_e284 * var_vbc_dn8);
        let eq18_e286_d_n9: f64 = (eq18_e284 * var_vbc_dn9);
        let eq18_e286_d_n10: f64 = (eq18_e284 * var_vbc_dn10);
        let eq18_e286_d_n11: f64 = (eq18_e284 * var_vbc_dn11);
        let eq18_e286_d_b0: f64 = (eq18_e284 * var_vbc_db0);
        let eq18_e286_d_b1: f64 = (eq18_e284 * var_vbc_db1);
        let eq18_e287: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq18_e286);
        let eq18_e289: f64 = (eq18_e287 * p.p1);
        let eq18_e289_d_n0: f64 = ((eq18_e286_d_n0 * ddt_scale) * p.p1);
        let eq18_e289_d_n1: f64 = ((eq18_e286_d_n1 * ddt_scale) * p.p1);
        let eq18_e289_d_n2: f64 = ((eq18_e286_d_n2 * ddt_scale) * p.p1);
        let eq18_e289_d_n3: f64 = ((eq18_e286_d_n3 * ddt_scale) * p.p1);
        let eq18_e289_d_n4: f64 = ((eq18_e286_d_n4 * ddt_scale) * p.p1);
        let eq18_e289_d_n5: f64 = ((eq18_e286_d_n5 * ddt_scale) * p.p1);
        let eq18_e289_d_n6: f64 = ((eq18_e286_d_n6 * ddt_scale) * p.p1);
        let eq18_e289_d_n7: f64 = ((eq18_e286_d_n7 * ddt_scale) * p.p1);
        let eq18_e289_d_n8: f64 = ((eq18_e286_d_n8 * ddt_scale) * p.p1);
        let eq18_e289_d_n9: f64 = ((eq18_e286_d_n9 * ddt_scale) * p.p1);
        let eq18_e289_d_n10: f64 = ((eq18_e286_d_n10 * ddt_scale) * p.p1);
        let eq18_e289_d_n11: f64 = ((eq18_e286_d_n11 * ddt_scale) * p.p1);
        let eq18_e289_d_b0: f64 = ((eq18_e286_d_b0 * ddt_scale) * p.p1);
        let eq18_e289_d_b1: f64 = ((eq18_e286_d_b1 * ddt_scale) * p.p1);
        let eq18_value: f64 = eq18_e289;
        let eq18_node_derivatives: [f64; 12] = [eq18_e289_d_n0, eq18_e289_d_n1, eq18_e289_d_n2, eq18_e289_d_n3, eq18_e289_d_n4, eq18_e289_d_n5, eq18_e289_d_n6, eq18_e289_d_n7, eq18_e289_d_n8, eq18_e289_d_n9, eq18_e289_d_n10, eq18_e289_d_n11];
        let eq18_branch_derivatives: [f64; 2] = [eq18_e289_d_b0, eq18_e289_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(0),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
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
        var_gcc_ex_t: f64,
        var_gcc_ex_t_db0: f64,
        var_gcc_ex_t_db1: f64,
        var_gcc_ex_t_dn0: f64,
        var_gcc_ex_t_dn1: f64,
        var_gcc_ex_t_dn10: f64,
        var_gcc_ex_t_dn11: f64,
        var_gcc_ex_t_dn2: f64,
        var_gcc_ex_t_dn3: f64,
        var_gcc_ex_t_dn4: f64,
        var_gcc_ex_t_dn5: f64,
        var_gcc_ex_t_dn6: f64,
        var_gcc_ex_t_dn7: f64,
        var_gcc_ex_t_dn8: f64,
        var_gcc_ex_t_dn9: f64,
        var_gcc_in_t: f64,
        var_gcc_in_t_db0: f64,
        var_gcc_in_t_db1: f64,
        var_gcc_in_t_dn0: f64,
        var_gcc_in_t_dn1: f64,
        var_gcc_in_t_dn10: f64,
        var_gcc_in_t_dn11: f64,
        var_gcc_in_t_dn2: f64,
        var_gcc_in_t_dn3: f64,
        var_gcc_in_t_dn4: f64,
        var_gcc_in_t_dn5: f64,
        var_gcc_in_t_dn6: f64,
        var_gcc_in_t_dn7: f64,
        var_gcc_in_t_dn8: f64,
        var_gcc_in_t_dn9: f64,
        var_gcc_xx_t: f64,
        var_gcc_xx_t_db0: f64,
        var_gcc_xx_t_db1: f64,
        var_gcc_xx_t_dn0: f64,
        var_gcc_xx_t_dn1: f64,
        var_gcc_xx_t_dn10: f64,
        var_gcc_xx_t_dn11: f64,
        var_gcc_xx_t_dn2: f64,
        var_gcc_xx_t_dn3: f64,
        var_gcc_xx_t_dn4: f64,
        var_gcc_xx_t_dn5: f64,
        var_gcc_xx_t_dn6: f64,
        var_gcc_xx_t_dn7: f64,
        var_gcc_xx_t_dn8: f64,
        var_gcc_xx_t_dn9: f64,
        var_gem_n: f64,
        var_gem_n_db0: f64,
        var_gem_n_db1: f64,
        var_gem_n_dn0: f64,
        var_gem_n_dn1: f64,
        var_gem_n_dn10: f64,
        var_gem_n_dn11: f64,
        var_gem_n_dn2: f64,
        var_gem_n_dn3: f64,
        var_gem_n_dn4: f64,
        var_gem_n_dn5: f64,
        var_gem_n_dn6: f64,
        var_gem_n_dn7: f64,
        var_gem_n_dn8: f64,
        var_gem_n_dn9: f64,
        var_gmin: f64,
        var_guard121: f64,
        var_guard122: f64,
        var_ib3: f64,
        var_ib3_db0: f64,
        var_ib3_db1: f64,
        var_ib3_dn0: f64,
        var_ib3_dn1: f64,
        var_ib3_dn10: f64,
        var_ib3_dn11: f64,
        var_ib3_dn2: f64,
        var_ib3_dn3: f64,
        var_ib3_dn4: f64,
        var_ib3_dn5: f64,
        var_ib3_dn6: f64,
        var_ib3_dn7: f64,
        var_ib3_dn8: f64,
        var_ib3_dn9: f64,
        var_iex: f64,
        var_iex_db0: f64,
        var_iex_db1: f64,
        var_iex_dn0: f64,
        var_iex_dn1: f64,
        var_iex_dn10: f64,
        var_iex_dn11: f64,
        var_iex_dn2: f64,
        var_iex_dn3: f64,
        var_iex_dn4: f64,
        var_iex_dn5: f64,
        var_iex_dn6: f64,
        var_iex_dn7: f64,
        var_iex_dn8: f64,
        var_iex_dn9: f64,
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
        var_vc3c4: f64,
        var_vc3c4_db0: f64,
        var_vc3c4_db1: f64,
        var_vc3c4_dn0: f64,
        var_vc3c4_dn1: f64,
        var_vc3c4_dn10: f64,
        var_vc3c4_dn11: f64,
        var_vc3c4_dn2: f64,
        var_vc3c4_dn3: f64,
        var_vc3c4_dn4: f64,
        var_vc3c4_dn5: f64,
        var_vc3c4_dn6: f64,
        var_vc3c4_dn7: f64,
        var_vc3c4_dn8: f64,
        var_vc3c4_dn9: f64,
        var_vc4c1: f64,
        var_vc4c1_db0: f64,
        var_vc4c1_db1: f64,
        var_vc4c1_dn0: f64,
        var_vc4c1_dn1: f64,
        var_vc4c1_dn10: f64,
        var_vc4c1_dn11: f64,
        var_vc4c1_dn2: f64,
        var_vc4c1_dn3: f64,
        var_vc4c1_dn4: f64,
        var_vc4c1_dn5: f64,
        var_vc4c1_dn6: f64,
        var_vc4c1_dn7: f64,
        var_vc4c1_dn8: f64,
        var_vc4c1_dn9: f64,
        var_vcc3: f64,
        var_vcc3_db0: f64,
        var_vcc3_db1: f64,
        var_vcc3_dn0: f64,
        var_vcc3_dn1: f64,
        var_vcc3_dn10: f64,
        var_vcc3_dn11: f64,
        var_vcc3_dn2: f64,
        var_vcc3_dn3: f64,
        var_vcc3_dn4: f64,
        var_vcc3_dn5: f64,
        var_vcc3_dn6: f64,
        var_vcc3_dn7: f64,
        var_vcc3_dn8: f64,
        var_vcc3_dn9: f64,
        var_xiex: f64,
        var_xiex_db0: f64,
        var_xiex_db1: f64,
        var_xiex_dn0: f64,
        var_xiex_dn1: f64,
        var_xiex_dn10: f64,
        var_xiex_dn11: f64,
        var_xiex_dn2: f64,
        var_xiex_dn3: f64,
        var_xiex_dn4: f64,
        var_xiex_dn5: f64,
        var_xiex_dn6: f64,
        var_xiex_dn7: f64,
        var_xiex_dn8: f64,
        var_xiex_dn9: f64,
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
        let eq19_e292: f64 = (p.p3 * var_xiex);
        let eq19_e292_d_n0: f64 = (p.p3 * var_xiex_dn0);
        let eq19_e292_d_n1: f64 = (p.p3 * var_xiex_dn1);
        let eq19_e292_d_n2: f64 = (p.p3 * var_xiex_dn2);
        let eq19_e292_d_n3: f64 = (p.p3 * var_xiex_dn3);
        let eq19_e292_d_n4: f64 = (p.p3 * var_xiex_dn4);
        let eq19_e292_d_n5: f64 = (p.p3 * var_xiex_dn5);
        let eq19_e292_d_n6: f64 = (p.p3 * var_xiex_dn6);
        let eq19_e292_d_n7: f64 = (p.p3 * var_xiex_dn7);
        let eq19_e292_d_n8: f64 = (p.p3 * var_xiex_dn8);
        let eq19_e292_d_n9: f64 = (p.p3 * var_xiex_dn9);
        let eq19_e292_d_n10: f64 = (p.p3 * var_xiex_dn10);
        let eq19_e292_d_n11: f64 = (p.p3 * var_xiex_dn11);
        let eq19_e292_d_b0: f64 = (p.p3 * var_xiex_db0);
        let eq19_e292_d_b1: f64 = (p.p3 * var_xiex_db1);
        let eq19_e294: f64 = (eq19_e292 * p.p1);
        let eq19_e294_d_n0: f64 = (eq19_e292_d_n0 * p.p1);
        let eq19_e294_d_n1: f64 = (eq19_e292_d_n1 * p.p1);
        let eq19_e294_d_n2: f64 = (eq19_e292_d_n2 * p.p1);
        let eq19_e294_d_n3: f64 = (eq19_e292_d_n3 * p.p1);
        let eq19_e294_d_n4: f64 = (eq19_e292_d_n4 * p.p1);
        let eq19_e294_d_n5: f64 = (eq19_e292_d_n5 * p.p1);
        let eq19_e294_d_n6: f64 = (eq19_e292_d_n6 * p.p1);
        let eq19_e294_d_n7: f64 = (eq19_e292_d_n7 * p.p1);
        let eq19_e294_d_n8: f64 = (eq19_e292_d_n8 * p.p1);
        let eq19_e294_d_n9: f64 = (eq19_e292_d_n9 * p.p1);
        let eq19_e294_d_n10: f64 = (eq19_e292_d_n10 * p.p1);
        let eq19_e294_d_n11: f64 = (eq19_e292_d_n11 * p.p1);
        let eq19_e294_d_b0: f64 = (eq19_e292_d_b0 * p.p1);
        let eq19_e294_d_b1: f64 = (eq19_e292_d_b1 * p.p1);
        let eq19_value: f64 = eq19_e294;
        let eq19_node_derivatives: [f64; 12] = [eq19_e294_d_n0, eq19_e294_d_n1, eq19_e294_d_n2, eq19_e294_d_n3, eq19_e294_d_n4, eq19_e294_d_n5, eq19_e294_d_n6, eq19_e294_d_n7, eq19_e294_d_n8, eq19_e294_d_n9, eq19_e294_d_n10, eq19_e294_d_n11];
        let eq19_branch_derivatives: [f64; 2] = [eq19_e294_d_b0, eq19_e294_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(9),
            multiplicity * (eq19_value),
            &eq19_node_derivatives,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let eq20_e297: f64 = (p.p3 * var_vcc3);
        let eq20_e297_d_n0: f64 = (p.p3 * var_vcc3_dn0);
        let eq20_e297_d_n1: f64 = (p.p3 * var_vcc3_dn1);
        let eq20_e297_d_n2: f64 = (p.p3 * var_vcc3_dn2);
        let eq20_e297_d_n3: f64 = (p.p3 * var_vcc3_dn3);
        let eq20_e297_d_n4: f64 = (p.p3 * var_vcc3_dn4);
        let eq20_e297_d_n5: f64 = (p.p3 * var_vcc3_dn5);
        let eq20_e297_d_n6: f64 = (p.p3 * var_vcc3_dn6);
        let eq20_e297_d_n7: f64 = (p.p3 * var_vcc3_dn7);
        let eq20_e297_d_n8: f64 = (p.p3 * var_vcc3_dn8);
        let eq20_e297_d_n9: f64 = (p.p3 * var_vcc3_dn9);
        let eq20_e297_d_n10: f64 = (p.p3 * var_vcc3_dn10);
        let eq20_e297_d_n11: f64 = (p.p3 * var_vcc3_dn11);
        let eq20_e297_d_b0: f64 = (p.p3 * var_vcc3_db0);
        let eq20_e297_d_b1: f64 = (p.p3 * var_vcc3_db1);
        let eq20_e299: f64 = (eq20_e297 * var_gcc_xx_t);
        let eq20_e299_d_n0: f64 = ((eq20_e297_d_n0 * var_gcc_xx_t) + (eq20_e297 * var_gcc_xx_t_dn0));
        let eq20_e299_d_n1: f64 = ((eq20_e297_d_n1 * var_gcc_xx_t) + (eq20_e297 * var_gcc_xx_t_dn1));
        let eq20_e299_d_n2: f64 = ((eq20_e297_d_n2 * var_gcc_xx_t) + (eq20_e297 * var_gcc_xx_t_dn2));
        let eq20_e299_d_n3: f64 = ((eq20_e297_d_n3 * var_gcc_xx_t) + (eq20_e297 * var_gcc_xx_t_dn3));
        let eq20_e299_d_n4: f64 = ((eq20_e297_d_n4 * var_gcc_xx_t) + (eq20_e297 * var_gcc_xx_t_dn4));
        let eq20_e299_d_n5: f64 = ((eq20_e297_d_n5 * var_gcc_xx_t) + (eq20_e297 * var_gcc_xx_t_dn5));
        let eq20_e299_d_n6: f64 = ((eq20_e297_d_n6 * var_gcc_xx_t) + (eq20_e297 * var_gcc_xx_t_dn6));
        let eq20_e299_d_n7: f64 = ((eq20_e297_d_n7 * var_gcc_xx_t) + (eq20_e297 * var_gcc_xx_t_dn7));
        let eq20_e299_d_n8: f64 = ((eq20_e297_d_n8 * var_gcc_xx_t) + (eq20_e297 * var_gcc_xx_t_dn8));
        let eq20_e299_d_n9: f64 = ((eq20_e297_d_n9 * var_gcc_xx_t) + (eq20_e297 * var_gcc_xx_t_dn9));
        let eq20_e299_d_n10: f64 = ((eq20_e297_d_n10 * var_gcc_xx_t) + (eq20_e297 * var_gcc_xx_t_dn10));
        let eq20_e299_d_n11: f64 = ((eq20_e297_d_n11 * var_gcc_xx_t) + (eq20_e297 * var_gcc_xx_t_dn11));
        let eq20_e299_d_b0: f64 = ((eq20_e297_d_b0 * var_gcc_xx_t) + (eq20_e297 * var_gcc_xx_t_db0));
        let eq20_e299_d_b1: f64 = ((eq20_e297_d_b1 * var_gcc_xx_t) + (eq20_e297 * var_gcc_xx_t_db1));
        let eq20_e301: f64 = (eq20_e299 * p.p1);
        let eq20_e301_d_n0: f64 = (eq20_e299_d_n0 * p.p1);
        let eq20_e301_d_n1: f64 = (eq20_e299_d_n1 * p.p1);
        let eq20_e301_d_n2: f64 = (eq20_e299_d_n2 * p.p1);
        let eq20_e301_d_n3: f64 = (eq20_e299_d_n3 * p.p1);
        let eq20_e301_d_n4: f64 = (eq20_e299_d_n4 * p.p1);
        let eq20_e301_d_n5: f64 = (eq20_e299_d_n5 * p.p1);
        let eq20_e301_d_n6: f64 = (eq20_e299_d_n6 * p.p1);
        let eq20_e301_d_n7: f64 = (eq20_e299_d_n7 * p.p1);
        let eq20_e301_d_n8: f64 = (eq20_e299_d_n8 * p.p1);
        let eq20_e301_d_n9: f64 = (eq20_e299_d_n9 * p.p1);
        let eq20_e301_d_n10: f64 = (eq20_e299_d_n10 * p.p1);
        let eq20_e301_d_n11: f64 = (eq20_e299_d_n11 * p.p1);
        let eq20_e301_d_b0: f64 = (eq20_e299_d_b0 * p.p1);
        let eq20_e301_d_b1: f64 = (eq20_e299_d_b1 * p.p1);
        let eq20_value: f64 = eq20_e301;
        let eq20_node_derivatives: [f64; 12] = [eq20_e301_d_n0, eq20_e301_d_n1, eq20_e301_d_n2, eq20_e301_d_n3, eq20_e301_d_n4, eq20_e301_d_n5, eq20_e301_d_n6, eq20_e301_d_n7, eq20_e301_d_n8, eq20_e301_d_n9, eq20_e301_d_n10, eq20_e301_d_n11];
        let eq20_branch_derivatives: [f64; 2] = [eq20_e301_d_b0, eq20_e301_d_b1];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(9),
            multiplicity * (eq20_value),
            &eq20_node_derivatives,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let eq21_e305: f64 = (var_xqtex + var_xqex);
        let eq21_e305_d_n0: f64 = (var_xqtex_dn0 + var_xqex_dn0);
        let eq21_e305_d_n1: f64 = (var_xqtex_dn1 + var_xqex_dn1);
        let eq21_e305_d_n2: f64 = (var_xqtex_dn2 + var_xqex_dn2);
        let eq21_e305_d_n3: f64 = (var_xqtex_dn3 + var_xqex_dn3);
        let eq21_e305_d_n4: f64 = (var_xqtex_dn4 + var_xqex_dn4);
        let eq21_e305_d_n5: f64 = (var_xqtex_dn5 + var_xqex_dn5);
        let eq21_e305_d_n6: f64 = (var_xqtex_dn6 + var_xqex_dn6);
        let eq21_e305_d_n7: f64 = (var_xqtex_dn7 + var_xqex_dn7);
        let eq21_e305_d_n8: f64 = (var_xqtex_dn8 + var_xqex_dn8);
        let eq21_e305_d_n9: f64 = (var_xqtex_dn9 + var_xqex_dn9);
        let eq21_e305_d_n10: f64 = (var_xqtex_dn10 + var_xqex_dn10);
        let eq21_e305_d_n11: f64 = (var_xqtex_dn11 + var_xqex_dn11);
        let eq21_e305_d_b0: f64 = (var_xqtex_db0 + var_xqex_db0);
        let eq21_e305_d_b1: f64 = (var_xqtex_db1 + var_xqex_db1);
        let eq21_e306: f64 = (p.p3 * eq21_e305);
        let eq21_e306_d_n0: f64 = (p.p3 * eq21_e305_d_n0);
        let eq21_e306_d_n1: f64 = (p.p3 * eq21_e305_d_n1);
        let eq21_e306_d_n2: f64 = (p.p3 * eq21_e305_d_n2);
        let eq21_e306_d_n3: f64 = (p.p3 * eq21_e305_d_n3);
        let eq21_e306_d_n4: f64 = (p.p3 * eq21_e305_d_n4);
        let eq21_e306_d_n5: f64 = (p.p3 * eq21_e305_d_n5);
        let eq21_e306_d_n6: f64 = (p.p3 * eq21_e305_d_n6);
        let eq21_e306_d_n7: f64 = (p.p3 * eq21_e305_d_n7);
        let eq21_e306_d_n8: f64 = (p.p3 * eq21_e305_d_n8);
        let eq21_e306_d_n9: f64 = (p.p3 * eq21_e305_d_n9);
        let eq21_e306_d_n10: f64 = (p.p3 * eq21_e305_d_n10);
        let eq21_e306_d_n11: f64 = (p.p3 * eq21_e305_d_n11);
        let eq21_e306_d_b0: f64 = (p.p3 * eq21_e305_d_b0);
        let eq21_e306_d_b1: f64 = (p.p3 * eq21_e305_d_b1);
        let eq21_e307: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq21_e306);
        let eq21_e309: f64 = (eq21_e307 * p.p1);
        let eq21_e309_d_n0: f64 = ((eq21_e306_d_n0 * ddt_scale) * p.p1);
        let eq21_e309_d_n1: f64 = ((eq21_e306_d_n1 * ddt_scale) * p.p1);
        let eq21_e309_d_n2: f64 = ((eq21_e306_d_n2 * ddt_scale) * p.p1);
        let eq21_e309_d_n3: f64 = ((eq21_e306_d_n3 * ddt_scale) * p.p1);
        let eq21_e309_d_n4: f64 = ((eq21_e306_d_n4 * ddt_scale) * p.p1);
        let eq21_e309_d_n5: f64 = ((eq21_e306_d_n5 * ddt_scale) * p.p1);
        let eq21_e309_d_n6: f64 = ((eq21_e306_d_n6 * ddt_scale) * p.p1);
        let eq21_e309_d_n7: f64 = ((eq21_e306_d_n7 * ddt_scale) * p.p1);
        let eq21_e309_d_n8: f64 = ((eq21_e306_d_n8 * ddt_scale) * p.p1);
        let eq21_e309_d_n9: f64 = ((eq21_e306_d_n9 * ddt_scale) * p.p1);
        let eq21_e309_d_n10: f64 = ((eq21_e306_d_n10 * ddt_scale) * p.p1);
        let eq21_e309_d_n11: f64 = ((eq21_e306_d_n11 * ddt_scale) * p.p1);
        let eq21_e309_d_b0: f64 = ((eq21_e306_d_b0 * ddt_scale) * p.p1);
        let eq21_e309_d_b1: f64 = ((eq21_e306_d_b1 * ddt_scale) * p.p1);
        let eq21_value: f64 = eq21_e309;
        let eq21_node_derivatives: [f64; 12] = [eq21_e309_d_n0, eq21_e309_d_n1, eq21_e309_d_n2, eq21_e309_d_n3, eq21_e309_d_n4, eq21_e309_d_n5, eq21_e309_d_n6, eq21_e309_d_n7, eq21_e309_d_n8, eq21_e309_d_n9, eq21_e309_d_n10, eq21_e309_d_n11];
        let eq21_branch_derivatives: [f64; 2] = [eq21_e309_d_b0, eq21_e309_d_b1];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(9),
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let eq22_e314: f64 = (var_gmin * var_vb1c4);
        let eq22_e314_d_n0: f64 = (var_gmin * var_vb1c4_dn0);
        let eq22_e314_d_n1: f64 = (var_gmin * var_vb1c4_dn1);
        let eq22_e314_d_n2: f64 = (var_gmin * var_vb1c4_dn2);
        let eq22_e314_d_n3: f64 = (var_gmin * var_vb1c4_dn3);
        let eq22_e314_d_n4: f64 = (var_gmin * var_vb1c4_dn4);
        let eq22_e314_d_n5: f64 = (var_gmin * var_vb1c4_dn5);
        let eq22_e314_d_n6: f64 = (var_gmin * var_vb1c4_dn6);
        let eq22_e314_d_n7: f64 = (var_gmin * var_vb1c4_dn7);
        let eq22_e314_d_n8: f64 = (var_gmin * var_vb1c4_dn8);
        let eq22_e314_d_n9: f64 = (var_gmin * var_vb1c4_dn9);
        let eq22_e314_d_n10: f64 = (var_gmin * var_vb1c4_dn10);
        let eq22_e314_d_n11: f64 = (var_gmin * var_vb1c4_dn11);
        let eq22_e314_d_b0: f64 = (var_gmin * var_vb1c4_db0);
        let eq22_e314_d_b1: f64 = (var_gmin * var_vb1c4_db1);
        let eq22_e315: f64 = (var_ib3 + eq22_e314);
        let eq22_e315_d_n0: f64 = (var_ib3_dn0 + eq22_e314_d_n0);
        let eq22_e315_d_n1: f64 = (var_ib3_dn1 + eq22_e314_d_n1);
        let eq22_e315_d_n2: f64 = (var_ib3_dn2 + eq22_e314_d_n2);
        let eq22_e315_d_n3: f64 = (var_ib3_dn3 + eq22_e314_d_n3);
        let eq22_e315_d_n4: f64 = (var_ib3_dn4 + eq22_e314_d_n4);
        let eq22_e315_d_n5: f64 = (var_ib3_dn5 + eq22_e314_d_n5);
        let eq22_e315_d_n6: f64 = (var_ib3_dn6 + eq22_e314_d_n6);
        let eq22_e315_d_n7: f64 = (var_ib3_dn7 + eq22_e314_d_n7);
        let eq22_e315_d_n8: f64 = (var_ib3_dn8 + eq22_e314_d_n8);
        let eq22_e315_d_n9: f64 = (var_ib3_dn9 + eq22_e314_d_n9);
        let eq22_e315_d_n10: f64 = (var_ib3_dn10 + eq22_e314_d_n10);
        let eq22_e315_d_n11: f64 = (var_ib3_dn11 + eq22_e314_d_n11);
        let eq22_e315_d_b0: f64 = (var_ib3_db0 + eq22_e314_d_b0);
        let eq22_e315_d_b1: f64 = (var_ib3_db1 + eq22_e314_d_b1);
        let eq22_e317: f64 = (eq22_e315 + var_iex);
        let eq22_e317_d_n0: f64 = (eq22_e315_d_n0 + var_iex_dn0);
        let eq22_e317_d_n1: f64 = (eq22_e315_d_n1 + var_iex_dn1);
        let eq22_e317_d_n2: f64 = (eq22_e315_d_n2 + var_iex_dn2);
        let eq22_e317_d_n3: f64 = (eq22_e315_d_n3 + var_iex_dn3);
        let eq22_e317_d_n4: f64 = (eq22_e315_d_n4 + var_iex_dn4);
        let eq22_e317_d_n5: f64 = (eq22_e315_d_n5 + var_iex_dn5);
        let eq22_e317_d_n6: f64 = (eq22_e315_d_n6 + var_iex_dn6);
        let eq22_e317_d_n7: f64 = (eq22_e315_d_n7 + var_iex_dn7);
        let eq22_e317_d_n8: f64 = (eq22_e315_d_n8 + var_iex_dn8);
        let eq22_e317_d_n9: f64 = (eq22_e315_d_n9 + var_iex_dn9);
        let eq22_e317_d_n10: f64 = (eq22_e315_d_n10 + var_iex_dn10);
        let eq22_e317_d_n11: f64 = (eq22_e315_d_n11 + var_iex_dn11);
        let eq22_e317_d_b0: f64 = (eq22_e315_d_b0 + var_iex_db0);
        let eq22_e317_d_b1: f64 = (eq22_e315_d_b1 + var_iex_db1);
        let eq22_e318: f64 = (p.p3 * eq22_e317);
        let eq22_e318_d_n0: f64 = (p.p3 * eq22_e317_d_n0);
        let eq22_e318_d_n1: f64 = (p.p3 * eq22_e317_d_n1);
        let eq22_e318_d_n2: f64 = (p.p3 * eq22_e317_d_n2);
        let eq22_e318_d_n3: f64 = (p.p3 * eq22_e317_d_n3);
        let eq22_e318_d_n4: f64 = (p.p3 * eq22_e317_d_n4);
        let eq22_e318_d_n5: f64 = (p.p3 * eq22_e317_d_n5);
        let eq22_e318_d_n6: f64 = (p.p3 * eq22_e317_d_n6);
        let eq22_e318_d_n7: f64 = (p.p3 * eq22_e317_d_n7);
        let eq22_e318_d_n8: f64 = (p.p3 * eq22_e317_d_n8);
        let eq22_e318_d_n9: f64 = (p.p3 * eq22_e317_d_n9);
        let eq22_e318_d_n10: f64 = (p.p3 * eq22_e317_d_n10);
        let eq22_e318_d_n11: f64 = (p.p3 * eq22_e317_d_n11);
        let eq22_e318_d_b0: f64 = (p.p3 * eq22_e317_d_b0);
        let eq22_e318_d_b1: f64 = (p.p3 * eq22_e317_d_b1);
        let eq22_e320: f64 = (eq22_e318 * p.p1);
        let eq22_e320_d_n0: f64 = (eq22_e318_d_n0 * p.p1);
        let eq22_e320_d_n1: f64 = (eq22_e318_d_n1 * p.p1);
        let eq22_e320_d_n2: f64 = (eq22_e318_d_n2 * p.p1);
        let eq22_e320_d_n3: f64 = (eq22_e318_d_n3 * p.p1);
        let eq22_e320_d_n4: f64 = (eq22_e318_d_n4 * p.p1);
        let eq22_e320_d_n5: f64 = (eq22_e318_d_n5 * p.p1);
        let eq22_e320_d_n6: f64 = (eq22_e318_d_n6 * p.p1);
        let eq22_e320_d_n7: f64 = (eq22_e318_d_n7 * p.p1);
        let eq22_e320_d_n8: f64 = (eq22_e318_d_n8 * p.p1);
        let eq22_e320_d_n9: f64 = (eq22_e318_d_n9 * p.p1);
        let eq22_e320_d_n10: f64 = (eq22_e318_d_n10 * p.p1);
        let eq22_e320_d_n11: f64 = (eq22_e318_d_n11 * p.p1);
        let eq22_e320_d_b0: f64 = (eq22_e318_d_b0 * p.p1);
        let eq22_e320_d_b1: f64 = (eq22_e318_d_b1 * p.p1);
        let eq22_value: f64 = eq22_e320;
        let eq22_node_derivatives: [f64; 12] = [eq22_e320_d_n0, eq22_e320_d_n1, eq22_e320_d_n2, eq22_e320_d_n3, eq22_e320_d_n4, eq22_e320_d_n5, eq22_e320_d_n6, eq22_e320_d_n7, eq22_e320_d_n8, eq22_e320_d_n9, eq22_e320_d_n10, eq22_e320_d_n11];
        let eq22_branch_derivatives: [f64; 2] = [eq22_e320_d_b0, eq22_e320_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(10),
            multiplicity * (eq22_value),
            &eq22_node_derivatives,
            &eq22_branch_derivatives,
            multiplicity,
        );
        let eq23_e324: f64 = (var_qtex + var_qex);
        let eq23_e324_d_n0: f64 = (var_qtex_dn0 + var_qex_dn0);
        let eq23_e324_d_n1: f64 = (var_qtex_dn1 + var_qex_dn1);
        let eq23_e324_d_n2: f64 = (var_qtex_dn2 + var_qex_dn2);
        let eq23_e324_d_n3: f64 = (var_qtex_dn3 + var_qex_dn3);
        let eq23_e324_d_n4: f64 = (var_qtex_dn4 + var_qex_dn4);
        let eq23_e324_d_n5: f64 = (var_qtex_dn5 + var_qex_dn5);
        let eq23_e324_d_n6: f64 = (var_qtex_dn6 + var_qex_dn6);
        let eq23_e324_d_n7: f64 = (var_qtex_dn7 + var_qex_dn7);
        let eq23_e324_d_n8: f64 = (var_qtex_dn8 + var_qex_dn8);
        let eq23_e324_d_n9: f64 = (var_qtex_dn9 + var_qex_dn9);
        let eq23_e324_d_n10: f64 = (var_qtex_dn10 + var_qex_dn10);
        let eq23_e324_d_n11: f64 = (var_qtex_dn11 + var_qex_dn11);
        let eq23_e324_d_b0: f64 = (var_qtex_db0 + var_qex_db0);
        let eq23_e324_d_b1: f64 = (var_qtex_db1 + var_qex_db1);
        let eq23_e325: f64 = (p.p3 * eq23_e324);
        let eq23_e325_d_n0: f64 = (p.p3 * eq23_e324_d_n0);
        let eq23_e325_d_n1: f64 = (p.p3 * eq23_e324_d_n1);
        let eq23_e325_d_n2: f64 = (p.p3 * eq23_e324_d_n2);
        let eq23_e325_d_n3: f64 = (p.p3 * eq23_e324_d_n3);
        let eq23_e325_d_n4: f64 = (p.p3 * eq23_e324_d_n4);
        let eq23_e325_d_n5: f64 = (p.p3 * eq23_e324_d_n5);
        let eq23_e325_d_n6: f64 = (p.p3 * eq23_e324_d_n6);
        let eq23_e325_d_n7: f64 = (p.p3 * eq23_e324_d_n7);
        let eq23_e325_d_n8: f64 = (p.p3 * eq23_e324_d_n8);
        let eq23_e325_d_n9: f64 = (p.p3 * eq23_e324_d_n9);
        let eq23_e325_d_n10: f64 = (p.p3 * eq23_e324_d_n10);
        let eq23_e325_d_n11: f64 = (p.p3 * eq23_e324_d_n11);
        let eq23_e325_d_b0: f64 = (p.p3 * eq23_e324_d_b0);
        let eq23_e325_d_b1: f64 = (p.p3 * eq23_e324_d_b1);
        let eq23_e326: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq23_e325);
        let eq23_e328: f64 = (eq23_e326 * p.p1);
        let eq23_e328_d_n0: f64 = ((eq23_e325_d_n0 * ddt_scale) * p.p1);
        let eq23_e328_d_n1: f64 = ((eq23_e325_d_n1 * ddt_scale) * p.p1);
        let eq23_e328_d_n2: f64 = ((eq23_e325_d_n2 * ddt_scale) * p.p1);
        let eq23_e328_d_n3: f64 = ((eq23_e325_d_n3 * ddt_scale) * p.p1);
        let eq23_e328_d_n4: f64 = ((eq23_e325_d_n4 * ddt_scale) * p.p1);
        let eq23_e328_d_n5: f64 = ((eq23_e325_d_n5 * ddt_scale) * p.p1);
        let eq23_e328_d_n6: f64 = ((eq23_e325_d_n6 * ddt_scale) * p.p1);
        let eq23_e328_d_n7: f64 = ((eq23_e325_d_n7 * ddt_scale) * p.p1);
        let eq23_e328_d_n8: f64 = ((eq23_e325_d_n8 * ddt_scale) * p.p1);
        let eq23_e328_d_n9: f64 = ((eq23_e325_d_n9 * ddt_scale) * p.p1);
        let eq23_e328_d_n10: f64 = ((eq23_e325_d_n10 * ddt_scale) * p.p1);
        let eq23_e328_d_n11: f64 = ((eq23_e325_d_n11 * ddt_scale) * p.p1);
        let eq23_e328_d_b0: f64 = ((eq23_e325_d_b0 * ddt_scale) * p.p1);
        let eq23_e328_d_b1: f64 = ((eq23_e325_d_b1 * ddt_scale) * p.p1);
        let eq23_value: f64 = eq23_e328;
        let eq23_node_derivatives: [f64; 12] = [eq23_e328_d_n0, eq23_e328_d_n1, eq23_e328_d_n2, eq23_e328_d_n3, eq23_e328_d_n4, eq23_e328_d_n5, eq23_e328_d_n6, eq23_e328_d_n7, eq23_e328_d_n8, eq23_e328_d_n9, eq23_e328_d_n10, eq23_e328_d_n11];
        let eq23_branch_derivatives: [f64; 2] = [eq23_e328_d_b0, eq23_e328_d_b1];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(10),
            multiplicity * (eq23_value),
            &eq23_node_derivatives,
            &eq23_branch_derivatives,
            multiplicity,
        );
        let (eq24_e338, eq24_e338_d_n0, eq24_e338_d_n1, eq24_e338_d_n2, eq24_e338_d_n3, eq24_e338_d_n4, eq24_e338_d_n5, eq24_e338_d_n6, eq24_e338_d_n7, eq24_e338_d_n8, eq24_e338_d_n9, eq24_e338_d_n10, eq24_e338_d_n11, eq24_e338_d_b0, eq24_e338_d_b1,) = {
    if (var_guard121 != 0.0) {
        let eq24_e332: f64 = (p.p3 * var_vc3c4);
        let eq24_e332_d_n0: f64 = (p.p3 * var_vc3c4_dn0);
        let eq24_e332_d_n1: f64 = (p.p3 * var_vc3c4_dn1);
        let eq24_e332_d_n2: f64 = (p.p3 * var_vc3c4_dn2);
        let eq24_e332_d_n3: f64 = (p.p3 * var_vc3c4_dn3);
        let eq24_e332_d_n4: f64 = (p.p3 * var_vc3c4_dn4);
        let eq24_e332_d_n5: f64 = (p.p3 * var_vc3c4_dn5);
        let eq24_e332_d_n6: f64 = (p.p3 * var_vc3c4_dn6);
        let eq24_e332_d_n7: f64 = (p.p3 * var_vc3c4_dn7);
        let eq24_e332_d_n8: f64 = (p.p3 * var_vc3c4_dn8);
        let eq24_e332_d_n9: f64 = (p.p3 * var_vc3c4_dn9);
        let eq24_e332_d_n10: f64 = (p.p3 * var_vc3c4_dn10);
        let eq24_e332_d_n11: f64 = (p.p3 * var_vc3c4_dn11);
        let eq24_e332_d_b0: f64 = (p.p3 * var_vc3c4_db0);
        let eq24_e332_d_b1: f64 = (p.p3 * var_vc3c4_db1);
        let eq24_e334: f64 = (eq24_e332 * var_gcc_ex_t);
        let eq24_e334_d_n0: f64 = ((eq24_e332_d_n0 * var_gcc_ex_t) + (eq24_e332 * var_gcc_ex_t_dn0));
        let eq24_e334_d_n1: f64 = ((eq24_e332_d_n1 * var_gcc_ex_t) + (eq24_e332 * var_gcc_ex_t_dn1));
        let eq24_e334_d_n2: f64 = ((eq24_e332_d_n2 * var_gcc_ex_t) + (eq24_e332 * var_gcc_ex_t_dn2));
        let eq24_e334_d_n3: f64 = ((eq24_e332_d_n3 * var_gcc_ex_t) + (eq24_e332 * var_gcc_ex_t_dn3));
        let eq24_e334_d_n4: f64 = ((eq24_e332_d_n4 * var_gcc_ex_t) + (eq24_e332 * var_gcc_ex_t_dn4));
        let eq24_e334_d_n5: f64 = ((eq24_e332_d_n5 * var_gcc_ex_t) + (eq24_e332 * var_gcc_ex_t_dn5));
        let eq24_e334_d_n6: f64 = ((eq24_e332_d_n6 * var_gcc_ex_t) + (eq24_e332 * var_gcc_ex_t_dn6));
        let eq24_e334_d_n7: f64 = ((eq24_e332_d_n7 * var_gcc_ex_t) + (eq24_e332 * var_gcc_ex_t_dn7));
        let eq24_e334_d_n8: f64 = ((eq24_e332_d_n8 * var_gcc_ex_t) + (eq24_e332 * var_gcc_ex_t_dn8));
        let eq24_e334_d_n9: f64 = ((eq24_e332_d_n9 * var_gcc_ex_t) + (eq24_e332 * var_gcc_ex_t_dn9));
        let eq24_e334_d_n10: f64 = ((eq24_e332_d_n10 * var_gcc_ex_t) + (eq24_e332 * var_gcc_ex_t_dn10));
        let eq24_e334_d_n11: f64 = ((eq24_e332_d_n11 * var_gcc_ex_t) + (eq24_e332 * var_gcc_ex_t_dn11));
        let eq24_e334_d_b0: f64 = ((eq24_e332_d_b0 * var_gcc_ex_t) + (eq24_e332 * var_gcc_ex_t_db0));
        let eq24_e334_d_b1: f64 = ((eq24_e332_d_b1 * var_gcc_ex_t) + (eq24_e332 * var_gcc_ex_t_db1));
        let eq24_e336: f64 = (eq24_e334 * p.p1);
        let eq24_e336_d_n0: f64 = (eq24_e334_d_n0 * p.p1);
        let eq24_e336_d_n1: f64 = (eq24_e334_d_n1 * p.p1);
        let eq24_e336_d_n2: f64 = (eq24_e334_d_n2 * p.p1);
        let eq24_e336_d_n3: f64 = (eq24_e334_d_n3 * p.p1);
        let eq24_e336_d_n4: f64 = (eq24_e334_d_n4 * p.p1);
        let eq24_e336_d_n5: f64 = (eq24_e334_d_n5 * p.p1);
        let eq24_e336_d_n6: f64 = (eq24_e334_d_n6 * p.p1);
        let eq24_e336_d_n7: f64 = (eq24_e334_d_n7 * p.p1);
        let eq24_e336_d_n8: f64 = (eq24_e334_d_n8 * p.p1);
        let eq24_e336_d_n9: f64 = (eq24_e334_d_n9 * p.p1);
        let eq24_e336_d_n10: f64 = (eq24_e334_d_n10 * p.p1);
        let eq24_e336_d_n11: f64 = (eq24_e334_d_n11 * p.p1);
        let eq24_e336_d_b0: f64 = (eq24_e334_d_b0 * p.p1);
        let eq24_e336_d_b1: f64 = (eq24_e334_d_b1 * p.p1);
        (eq24_e336, eq24_e336_d_n0, eq24_e336_d_n1, eq24_e336_d_n2, eq24_e336_d_n3, eq24_e336_d_n4, eq24_e336_d_n5, eq24_e336_d_n6, eq24_e336_d_n7, eq24_e336_d_n8, eq24_e336_d_n9, eq24_e336_d_n10, eq24_e336_d_n11, eq24_e336_d_b0, eq24_e336_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e338;
        let eq24_node_derivatives: [f64; 12] = [eq24_e338_d_n0, eq24_e338_d_n1, eq24_e338_d_n2, eq24_e338_d_n3, eq24_e338_d_n4, eq24_e338_d_n5, eq24_e338_d_n6, eq24_e338_d_n7, eq24_e338_d_n8, eq24_e338_d_n9, eq24_e338_d_n10, eq24_e338_d_n11];
        let eq24_branch_derivatives: [f64; 2] = [eq24_e338_d_b0, eq24_e338_d_b1];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(10),
            multiplicity * (eq24_value),
            &eq24_node_derivatives,
            &eq24_branch_derivatives,
            multiplicity,
        );
        let (eq26_e353, eq26_e353_d_n0, eq26_e353_d_n1, eq26_e353_d_n2, eq26_e353_d_n3, eq26_e353_d_n4, eq26_e353_d_n5, eq26_e353_d_n6, eq26_e353_d_n7, eq26_e353_d_n8, eq26_e353_d_n9, eq26_e353_d_n10, eq26_e353_d_n11, eq26_e353_d_b0, eq26_e353_d_b1,) = {
    if (var_guard122 != 0.0) {
        let eq26_e347: f64 = (p.p3 * var_vc4c1);
        let eq26_e347_d_n0: f64 = (p.p3 * var_vc4c1_dn0);
        let eq26_e347_d_n1: f64 = (p.p3 * var_vc4c1_dn1);
        let eq26_e347_d_n2: f64 = (p.p3 * var_vc4c1_dn2);
        let eq26_e347_d_n3: f64 = (p.p3 * var_vc4c1_dn3);
        let eq26_e347_d_n4: f64 = (p.p3 * var_vc4c1_dn4);
        let eq26_e347_d_n5: f64 = (p.p3 * var_vc4c1_dn5);
        let eq26_e347_d_n6: f64 = (p.p3 * var_vc4c1_dn6);
        let eq26_e347_d_n7: f64 = (p.p3 * var_vc4c1_dn7);
        let eq26_e347_d_n8: f64 = (p.p3 * var_vc4c1_dn8);
        let eq26_e347_d_n9: f64 = (p.p3 * var_vc4c1_dn9);
        let eq26_e347_d_n10: f64 = (p.p3 * var_vc4c1_dn10);
        let eq26_e347_d_n11: f64 = (p.p3 * var_vc4c1_dn11);
        let eq26_e347_d_b0: f64 = (p.p3 * var_vc4c1_db0);
        let eq26_e347_d_b1: f64 = (p.p3 * var_vc4c1_db1);
        let eq26_e349: f64 = (eq26_e347 * var_gcc_in_t);
        let eq26_e349_d_n0: f64 = ((eq26_e347_d_n0 * var_gcc_in_t) + (eq26_e347 * var_gcc_in_t_dn0));
        let eq26_e349_d_n1: f64 = ((eq26_e347_d_n1 * var_gcc_in_t) + (eq26_e347 * var_gcc_in_t_dn1));
        let eq26_e349_d_n2: f64 = ((eq26_e347_d_n2 * var_gcc_in_t) + (eq26_e347 * var_gcc_in_t_dn2));
        let eq26_e349_d_n3: f64 = ((eq26_e347_d_n3 * var_gcc_in_t) + (eq26_e347 * var_gcc_in_t_dn3));
        let eq26_e349_d_n4: f64 = ((eq26_e347_d_n4 * var_gcc_in_t) + (eq26_e347 * var_gcc_in_t_dn4));
        let eq26_e349_d_n5: f64 = ((eq26_e347_d_n5 * var_gcc_in_t) + (eq26_e347 * var_gcc_in_t_dn5));
        let eq26_e349_d_n6: f64 = ((eq26_e347_d_n6 * var_gcc_in_t) + (eq26_e347 * var_gcc_in_t_dn6));
        let eq26_e349_d_n7: f64 = ((eq26_e347_d_n7 * var_gcc_in_t) + (eq26_e347 * var_gcc_in_t_dn7));
        let eq26_e349_d_n8: f64 = ((eq26_e347_d_n8 * var_gcc_in_t) + (eq26_e347 * var_gcc_in_t_dn8));
        let eq26_e349_d_n9: f64 = ((eq26_e347_d_n9 * var_gcc_in_t) + (eq26_e347 * var_gcc_in_t_dn9));
        let eq26_e349_d_n10: f64 = ((eq26_e347_d_n10 * var_gcc_in_t) + (eq26_e347 * var_gcc_in_t_dn10));
        let eq26_e349_d_n11: f64 = ((eq26_e347_d_n11 * var_gcc_in_t) + (eq26_e347 * var_gcc_in_t_dn11));
        let eq26_e349_d_b0: f64 = ((eq26_e347_d_b0 * var_gcc_in_t) + (eq26_e347 * var_gcc_in_t_db0));
        let eq26_e349_d_b1: f64 = ((eq26_e347_d_b1 * var_gcc_in_t) + (eq26_e347 * var_gcc_in_t_db1));
        let eq26_e351: f64 = (eq26_e349 * p.p1);
        let eq26_e351_d_n0: f64 = (eq26_e349_d_n0 * p.p1);
        let eq26_e351_d_n1: f64 = (eq26_e349_d_n1 * p.p1);
        let eq26_e351_d_n2: f64 = (eq26_e349_d_n2 * p.p1);
        let eq26_e351_d_n3: f64 = (eq26_e349_d_n3 * p.p1);
        let eq26_e351_d_n4: f64 = (eq26_e349_d_n4 * p.p1);
        let eq26_e351_d_n5: f64 = (eq26_e349_d_n5 * p.p1);
        let eq26_e351_d_n6: f64 = (eq26_e349_d_n6 * p.p1);
        let eq26_e351_d_n7: f64 = (eq26_e349_d_n7 * p.p1);
        let eq26_e351_d_n8: f64 = (eq26_e349_d_n8 * p.p1);
        let eq26_e351_d_n9: f64 = (eq26_e349_d_n9 * p.p1);
        let eq26_e351_d_n10: f64 = (eq26_e349_d_n10 * p.p1);
        let eq26_e351_d_n11: f64 = (eq26_e349_d_n11 * p.p1);
        let eq26_e351_d_b0: f64 = (eq26_e349_d_b0 * p.p1);
        let eq26_e351_d_b1: f64 = (eq26_e349_d_b1 * p.p1);
        (eq26_e351, eq26_e351_d_n0, eq26_e351_d_n1, eq26_e351_d_n2, eq26_e351_d_n3, eq26_e351_d_n4, eq26_e351_d_n5, eq26_e351_d_n6, eq26_e351_d_n7, eq26_e351_d_n8, eq26_e351_d_n9, eq26_e351_d_n10, eq26_e351_d_n11, eq26_e351_d_b0, eq26_e351_d_b1,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e353;
        let eq26_node_derivatives: [f64; 12] = [eq26_e353_d_n0, eq26_e353_d_n1, eq26_e353_d_n2, eq26_e353_d_n3, eq26_e353_d_n4, eq26_e353_d_n5, eq26_e353_d_n6, eq26_e353_d_n7, eq26_e353_d_n8, eq26_e353_d_n9, eq26_e353_d_n10, eq26_e353_d_n11];
        let eq26_branch_derivatives: [f64; 2] = [eq26_e353_d_b0, eq26_e353_d_b1];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(7),
            multiplicity * (eq26_value),
            &eq26_node_derivatives,
            &eq26_branch_derivatives,
            multiplicity,
        );
        let eq30_e367: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, (nv11 - 0.0));
        let eq30_e368: f64 = (var_taun * eq30_e367);
        let eq30_e368_d_n0: f64 = (var_taun_dn0 * eq30_e367);
        let eq30_e368_d_n1: f64 = (var_taun_dn1 * eq30_e367);
        let eq30_e368_d_n2: f64 = (var_taun_dn2 * eq30_e367);
        let eq30_e368_d_n3: f64 = (var_taun_dn3 * eq30_e367);
        let eq30_e368_d_n4: f64 = (var_taun_dn4 * eq30_e367);
        let eq30_e368_d_n5: f64 = (var_taun_dn5 * eq30_e367);
        let eq30_e368_d_n6: f64 = (var_taun_dn6 * eq30_e367);
        let eq30_e368_d_n7: f64 = (var_taun_dn7 * eq30_e367);
        let eq30_e368_d_n8: f64 = (var_taun_dn8 * eq30_e367);
        let eq30_e368_d_n9: f64 = (var_taun_dn9 * eq30_e367);
        let eq30_e368_d_n10: f64 = (var_taun_dn10 * eq30_e367);
        let eq30_e368_d_n11: f64 = ((var_taun_dn11 * eq30_e367) + (var_taun * ddt_scale));
        let eq30_e368_d_b0: f64 = (var_taun_db0 * eq30_e367);
        let eq30_e368_d_b1: f64 = (var_taun_db1 * eq30_e367);
        let eq30_value: f64 = eq30_e368;
        let eq30_node_derivatives: [f64; 12] = [eq30_e368_d_n0, eq30_e368_d_n1, eq30_e368_d_n2, eq30_e368_d_n3, eq30_e368_d_n4, eq30_e368_d_n5, eq30_e368_d_n6, eq30_e368_d_n7, eq30_e368_d_n8, eq30_e368_d_n9, eq30_e368_d_n10, eq30_e368_d_n11];
        let eq30_branch_derivatives: [f64; 2] = [eq30_e368_d_b0, eq30_e368_d_b1];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(4),
            multiplicity * (eq30_value),
            &eq30_node_derivatives,
            &eq30_branch_derivatives,
            multiplicity,
        );
        let eq31_e371: f64 = (var_gem_n * (nv11 - 0.0));
        let eq31_e371_d_n0: f64 = (var_gem_n_dn0 * (nv11 - 0.0));
        let eq31_e371_d_n1: f64 = (var_gem_n_dn1 * (nv11 - 0.0));
        let eq31_e371_d_n2: f64 = (var_gem_n_dn2 * (nv11 - 0.0));
        let eq31_e371_d_n3: f64 = (var_gem_n_dn3 * (nv11 - 0.0));
        let eq31_e371_d_n4: f64 = (var_gem_n_dn4 * (nv11 - 0.0));
        let eq31_e371_d_n5: f64 = (var_gem_n_dn5 * (nv11 - 0.0));
        let eq31_e371_d_n6: f64 = (var_gem_n_dn6 * (nv11 - 0.0));
        let eq31_e371_d_n7: f64 = (var_gem_n_dn7 * (nv11 - 0.0));
        let eq31_e371_d_n8: f64 = (var_gem_n_dn8 * (nv11 - 0.0));
        let eq31_e371_d_n9: f64 = (var_gem_n_dn9 * (nv11 - 0.0));
        let eq31_e371_d_n10: f64 = (var_gem_n_dn10 * (nv11 - 0.0));
        let eq31_e371_d_n11: f64 = ((var_gem_n_dn11 * (nv11 - 0.0)) + var_gem_n);
        let eq31_e371_d_b0: f64 = (var_gem_n_db0 * (nv11 - 0.0));
        let eq31_e371_d_b1: f64 = (var_gem_n_db1 * (nv11 - 0.0));
        let eq31_value: f64 = eq31_e371;
        let eq31_node_derivatives: [f64; 12] = [eq31_e371_d_n0, eq31_e371_d_n1, eq31_e371_d_n2, eq31_e371_d_n3, eq31_e371_d_n4, eq31_e371_d_n5, eq31_e371_d_n6, eq31_e371_d_n7, eq31_e371_d_n8, eq31_e371_d_n9, eq31_e371_d_n10, eq31_e371_d_n11];
        let eq31_branch_derivatives: [f64; 2] = [eq31_e371_d_b0, eq31_e371_d_b1];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq31_value),
            &eq31_node_derivatives,
            &eq31_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_i_cth_rdb0: f64,
        var_i_cth_rdb1: f64,
        var_i_cth_rdn0: f64,
        var_i_cth_rdn1: f64,
        var_i_cth_rdn10: f64,
        var_i_cth_rdn11: f64,
        var_i_cth_rdn2: f64,
        var_i_cth_rdn3: f64,
        var_i_cth_rdn4: f64,
        var_i_cth_rdn5: f64,
        var_i_cth_rdn6: f64,
        var_i_cth_rdn7: f64,
        var_i_cth_rdn8: f64,
        var_i_cth_rdn9: f64,
        var_i_cth_rv: f64,
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
        let eq11_e235_q: f64 = var_i_cth_rv;
        let eq11_reactive_node_derivatives: [f64; 12] = [var_i_cth_rdn0, var_i_cth_rdn1, var_i_cth_rdn2, var_i_cth_rdn3, var_i_cth_rdn4, var_i_cth_rdn5, var_i_cth_rdn6, var_i_cth_rdn7, var_i_cth_rdn8, var_i_cth_rdn9, var_i_cth_rdn10, var_i_cth_rdn11];
        let eq11_reactive_branch_derivatives: [f64; 2] = [var_i_cth_rdb0, var_i_cth_rdb1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            None,
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
        let eq13_e245: f64 = (var_qte + var_qbe);
        let eq13_e245_d_n0: f64 = (var_qte_dn0 + var_qbe_dn0);
        let eq13_e245_d_n1: f64 = (var_qte_dn1 + var_qbe_dn1);
        let eq13_e245_d_n2: f64 = (var_qte_dn2 + var_qbe_dn2);
        let eq13_e245_d_n3: f64 = (var_qte_dn3 + var_qbe_dn3);
        let eq13_e245_d_n4: f64 = (var_qte_dn4 + var_qbe_dn4);
        let eq13_e245_d_n5: f64 = (var_qte_dn5 + var_qbe_dn5);
        let eq13_e245_d_n6: f64 = (var_qte_dn6 + var_qbe_dn6);
        let eq13_e245_d_n7: f64 = (var_qte_dn7 + var_qbe_dn7);
        let eq13_e245_d_n8: f64 = (var_qte_dn8 + var_qbe_dn8);
        let eq13_e245_d_n9: f64 = (var_qte_dn9 + var_qbe_dn9);
        let eq13_e245_d_n10: f64 = (var_qte_dn10 + var_qbe_dn10);
        let eq13_e245_d_n11: f64 = (var_qte_dn11 + var_qbe_dn11);
        let eq13_e245_d_b0: f64 = (var_qte_db0 + var_qbe_db0);
        let eq13_e245_d_b1: f64 = (var_qte_db1 + var_qbe_db1);
        let eq13_e247: f64 = (eq13_e245 + var_qe);
        let eq13_e247_d_n0: f64 = (eq13_e245_d_n0 + var_qe_dn0);
        let eq13_e247_d_n1: f64 = (eq13_e245_d_n1 + var_qe_dn1);
        let eq13_e247_d_n2: f64 = (eq13_e245_d_n2 + var_qe_dn2);
        let eq13_e247_d_n3: f64 = (eq13_e245_d_n3 + var_qe_dn3);
        let eq13_e247_d_n4: f64 = (eq13_e245_d_n4 + var_qe_dn4);
        let eq13_e247_d_n5: f64 = (eq13_e245_d_n5 + var_qe_dn5);
        let eq13_e247_d_n6: f64 = (eq13_e245_d_n6 + var_qe_dn6);
        let eq13_e247_d_n7: f64 = (eq13_e245_d_n7 + var_qe_dn7);
        let eq13_e247_d_n8: f64 = (eq13_e245_d_n8 + var_qe_dn8);
        let eq13_e247_d_n9: f64 = (eq13_e245_d_n9 + var_qe_dn9);
        let eq13_e247_d_n10: f64 = (eq13_e245_d_n10 + var_qe_dn10);
        let eq13_e247_d_n11: f64 = (eq13_e245_d_n11 + var_qe_dn11);
        let eq13_e247_d_b0: f64 = (eq13_e245_d_b0 + var_qe_db0);
        let eq13_e247_d_b1: f64 = (eq13_e245_d_b1 + var_qe_db1);
        let eq13_e248: f64 = (p.p3 * eq13_e247);
        let eq13_e248_d_n0: f64 = (p.p3 * eq13_e247_d_n0);
        let eq13_e248_d_n1: f64 = (p.p3 * eq13_e247_d_n1);
        let eq13_e248_d_n2: f64 = (p.p3 * eq13_e247_d_n2);
        let eq13_e248_d_n3: f64 = (p.p3 * eq13_e247_d_n3);
        let eq13_e248_d_n4: f64 = (p.p3 * eq13_e247_d_n4);
        let eq13_e248_d_n5: f64 = (p.p3 * eq13_e247_d_n5);
        let eq13_e248_d_n6: f64 = (p.p3 * eq13_e247_d_n6);
        let eq13_e248_d_n7: f64 = (p.p3 * eq13_e247_d_n7);
        let eq13_e248_d_n8: f64 = (p.p3 * eq13_e247_d_n8);
        let eq13_e248_d_n9: f64 = (p.p3 * eq13_e247_d_n9);
        let eq13_e248_d_n10: f64 = (p.p3 * eq13_e247_d_n10);
        let eq13_e248_d_n11: f64 = (p.p3 * eq13_e247_d_n11);
        let eq13_e248_d_b0: f64 = (p.p3 * eq13_e247_d_b0);
        let eq13_e248_d_b1: f64 = (p.p3 * eq13_e247_d_b1);
        let eq13_e249_q: f64 = eq13_e248;
        let eq13_e251: f64 = (eq13_e248 * p.p1);
        let eq13_e251_d_n0: f64 = (eq13_e248_d_n0 * p.p1);
        let eq13_e251_d_n1: f64 = (eq13_e248_d_n1 * p.p1);
        let eq13_e251_d_n2: f64 = (eq13_e248_d_n2 * p.p1);
        let eq13_e251_d_n3: f64 = (eq13_e248_d_n3 * p.p1);
        let eq13_e251_d_n4: f64 = (eq13_e248_d_n4 * p.p1);
        let eq13_e251_d_n5: f64 = (eq13_e248_d_n5 * p.p1);
        let eq13_e251_d_n6: f64 = (eq13_e248_d_n6 * p.p1);
        let eq13_e251_d_n7: f64 = (eq13_e248_d_n7 * p.p1);
        let eq13_e251_d_n8: f64 = (eq13_e248_d_n8 * p.p1);
        let eq13_e251_d_n9: f64 = (eq13_e248_d_n9 * p.p1);
        let eq13_e251_d_n10: f64 = (eq13_e248_d_n10 * p.p1);
        let eq13_e251_d_n11: f64 = (eq13_e248_d_n11 * p.p1);
        let eq13_e251_d_b0: f64 = (eq13_e248_d_b0 * p.p1);
        let eq13_e251_d_b1: f64 = (eq13_e248_d_b1 * p.p1);
        let eq13_e251_q: f64 = (eq13_e249_q * p.p1);
        let eq13_reactive_node_derivatives: [f64; 12] = [eq13_e251_d_n0, eq13_e251_d_n1, eq13_e251_d_n2, eq13_e251_d_n3, eq13_e251_d_n4, eq13_e251_d_n5, eq13_e251_d_n6, eq13_e251_d_n7, eq13_e251_d_n8, eq13_e251_d_n9, eq13_e251_d_n10, eq13_e251_d_n11];
        let eq13_reactive_branch_derivatives: [f64; 2] = [eq13_e251_d_b0, eq13_e251_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            nodes,
            &eq13_reactive_node_derivatives,
            branches,
            &eq13_reactive_branch_derivatives,
            multiplicity,
        );
        let eq14_e254: f64 = (p.p3 * var_qte_s);
        let eq14_e254_d_n0: f64 = (p.p3 * var_qte_s_dn0);
        let eq14_e254_d_n1: f64 = (p.p3 * var_qte_s_dn1);
        let eq14_e254_d_n2: f64 = (p.p3 * var_qte_s_dn2);
        let eq14_e254_d_n3: f64 = (p.p3 * var_qte_s_dn3);
        let eq14_e254_d_n4: f64 = (p.p3 * var_qte_s_dn4);
        let eq14_e254_d_n5: f64 = (p.p3 * var_qte_s_dn5);
        let eq14_e254_d_n6: f64 = (p.p3 * var_qte_s_dn6);
        let eq14_e254_d_n7: f64 = (p.p3 * var_qte_s_dn7);
        let eq14_e254_d_n8: f64 = (p.p3 * var_qte_s_dn8);
        let eq14_e254_d_n9: f64 = (p.p3 * var_qte_s_dn9);
        let eq14_e254_d_n10: f64 = (p.p3 * var_qte_s_dn10);
        let eq14_e254_d_n11: f64 = (p.p3 * var_qte_s_dn11);
        let eq14_e254_d_b0: f64 = (p.p3 * var_qte_s_db0);
        let eq14_e254_d_b1: f64 = (p.p3 * var_qte_s_db1);
        let eq14_e255_q: f64 = eq14_e254;
        let eq14_e257: f64 = (eq14_e254 * p.p1);
        let eq14_e257_d_n0: f64 = (eq14_e254_d_n0 * p.p1);
        let eq14_e257_d_n1: f64 = (eq14_e254_d_n1 * p.p1);
        let eq14_e257_d_n2: f64 = (eq14_e254_d_n2 * p.p1);
        let eq14_e257_d_n3: f64 = (eq14_e254_d_n3 * p.p1);
        let eq14_e257_d_n4: f64 = (eq14_e254_d_n4 * p.p1);
        let eq14_e257_d_n5: f64 = (eq14_e254_d_n5 * p.p1);
        let eq14_e257_d_n6: f64 = (eq14_e254_d_n6 * p.p1);
        let eq14_e257_d_n7: f64 = (eq14_e254_d_n7 * p.p1);
        let eq14_e257_d_n8: f64 = (eq14_e254_d_n8 * p.p1);
        let eq14_e257_d_n9: f64 = (eq14_e254_d_n9 * p.p1);
        let eq14_e257_d_n10: f64 = (eq14_e254_d_n10 * p.p1);
        let eq14_e257_d_n11: f64 = (eq14_e254_d_n11 * p.p1);
        let eq14_e257_d_b0: f64 = (eq14_e254_d_b0 * p.p1);
        let eq14_e257_d_b1: f64 = (eq14_e254_d_b1 * p.p1);
        let eq14_e257_q: f64 = (eq14_e255_q * p.p1);
        let eq14_reactive_node_derivatives: [f64; 12] = [eq14_e257_d_n0, eq14_e257_d_n1, eq14_e257_d_n2, eq14_e257_d_n3, eq14_e257_d_n4, eq14_e257_d_n5, eq14_e257_d_n6, eq14_e257_d_n7, eq14_e257_d_n8, eq14_e257_d_n9, eq14_e257_d_n10, eq14_e257_d_n11];
        let eq14_reactive_branch_derivatives: [f64; 2] = [eq14_e257_d_b0, eq14_e257_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            nodes,
            &eq14_reactive_node_derivatives,
            branches,
            &eq14_reactive_branch_derivatives,
            multiplicity,
        );
        let eq15_e261: f64 = (var_qtc + var_qbc);
        let eq15_e261_d_n0: f64 = (var_qtc_dn0 + var_qbc_dn0);
        let eq15_e261_d_n1: f64 = (var_qtc_dn1 + var_qbc_dn1);
        let eq15_e261_d_n2: f64 = (var_qtc_dn2 + var_qbc_dn2);
        let eq15_e261_d_n3: f64 = (var_qtc_dn3 + var_qbc_dn3);
        let eq15_e261_d_n4: f64 = (var_qtc_dn4 + var_qbc_dn4);
        let eq15_e261_d_n5: f64 = (var_qtc_dn5 + var_qbc_dn5);
        let eq15_e261_d_n6: f64 = (var_qtc_dn6 + var_qbc_dn6);
        let eq15_e261_d_n7: f64 = (var_qtc_dn7 + var_qbc_dn7);
        let eq15_e261_d_n8: f64 = (var_qtc_dn8 + var_qbc_dn8);
        let eq15_e261_d_n9: f64 = (var_qtc_dn9 + var_qbc_dn9);
        let eq15_e261_d_n10: f64 = (var_qtc_dn10 + var_qbc_dn10);
        let eq15_e261_d_n11: f64 = (var_qtc_dn11 + var_qbc_dn11);
        let eq15_e261_d_b0: f64 = (var_qtc_db0 + var_qbc_db0);
        let eq15_e261_d_b1: f64 = (var_qtc_db1 + var_qbc_db1);
        let eq15_e263: f64 = (eq15_e261 + var_qepi);
        let eq15_e263_d_n0: f64 = (eq15_e261_d_n0 + var_qepi_dn0);
        let eq15_e263_d_n1: f64 = (eq15_e261_d_n1 + var_qepi_dn1);
        let eq15_e263_d_n2: f64 = (eq15_e261_d_n2 + var_qepi_dn2);
        let eq15_e263_d_n3: f64 = (eq15_e261_d_n3 + var_qepi_dn3);
        let eq15_e263_d_n4: f64 = (eq15_e261_d_n4 + var_qepi_dn4);
        let eq15_e263_d_n5: f64 = (eq15_e261_d_n5 + var_qepi_dn5);
        let eq15_e263_d_n6: f64 = (eq15_e261_d_n6 + var_qepi_dn6);
        let eq15_e263_d_n7: f64 = (eq15_e261_d_n7 + var_qepi_dn7);
        let eq15_e263_d_n8: f64 = (eq15_e261_d_n8 + var_qepi_dn8);
        let eq15_e263_d_n9: f64 = (eq15_e261_d_n9 + var_qepi_dn9);
        let eq15_e263_d_n10: f64 = (eq15_e261_d_n10 + var_qepi_dn10);
        let eq15_e263_d_n11: f64 = (eq15_e261_d_n11 + var_qepi_dn11);
        let eq15_e263_d_b0: f64 = (eq15_e261_d_b0 + var_qepi_db0);
        let eq15_e263_d_b1: f64 = (eq15_e261_d_b1 + var_qepi_db1);
        let eq15_e264: f64 = (p.p3 * eq15_e263);
        let eq15_e264_d_n0: f64 = (p.p3 * eq15_e263_d_n0);
        let eq15_e264_d_n1: f64 = (p.p3 * eq15_e263_d_n1);
        let eq15_e264_d_n2: f64 = (p.p3 * eq15_e263_d_n2);
        let eq15_e264_d_n3: f64 = (p.p3 * eq15_e263_d_n3);
        let eq15_e264_d_n4: f64 = (p.p3 * eq15_e263_d_n4);
        let eq15_e264_d_n5: f64 = (p.p3 * eq15_e263_d_n5);
        let eq15_e264_d_n6: f64 = (p.p3 * eq15_e263_d_n6);
        let eq15_e264_d_n7: f64 = (p.p3 * eq15_e263_d_n7);
        let eq15_e264_d_n8: f64 = (p.p3 * eq15_e263_d_n8);
        let eq15_e264_d_n9: f64 = (p.p3 * eq15_e263_d_n9);
        let eq15_e264_d_n10: f64 = (p.p3 * eq15_e263_d_n10);
        let eq15_e264_d_n11: f64 = (p.p3 * eq15_e263_d_n11);
        let eq15_e264_d_b0: f64 = (p.p3 * eq15_e263_d_b0);
        let eq15_e264_d_b1: f64 = (p.p3 * eq15_e263_d_b1);
        let eq15_e265_q: f64 = eq15_e264;
        let eq15_e267: f64 = (eq15_e264 * p.p1);
        let eq15_e267_d_n0: f64 = (eq15_e264_d_n0 * p.p1);
        let eq15_e267_d_n1: f64 = (eq15_e264_d_n1 * p.p1);
        let eq15_e267_d_n2: f64 = (eq15_e264_d_n2 * p.p1);
        let eq15_e267_d_n3: f64 = (eq15_e264_d_n3 * p.p1);
        let eq15_e267_d_n4: f64 = (eq15_e264_d_n4 * p.p1);
        let eq15_e267_d_n5: f64 = (eq15_e264_d_n5 * p.p1);
        let eq15_e267_d_n6: f64 = (eq15_e264_d_n6 * p.p1);
        let eq15_e267_d_n7: f64 = (eq15_e264_d_n7 * p.p1);
        let eq15_e267_d_n8: f64 = (eq15_e264_d_n8 * p.p1);
        let eq15_e267_d_n9: f64 = (eq15_e264_d_n9 * p.p1);
        let eq15_e267_d_n10: f64 = (eq15_e264_d_n10 * p.p1);
        let eq15_e267_d_n11: f64 = (eq15_e264_d_n11 * p.p1);
        let eq15_e267_d_b0: f64 = (eq15_e264_d_b0 * p.p1);
        let eq15_e267_d_b1: f64 = (eq15_e264_d_b1 * p.p1);
        let eq15_e267_q: f64 = (eq15_e265_q * p.p1);
        let eq15_reactive_node_derivatives: [f64; 12] = [eq15_e267_d_n0, eq15_e267_d_n1, eq15_e267_d_n2, eq15_e267_d_n3, eq15_e267_d_n4, eq15_e267_d_n5, eq15_e267_d_n6, eq15_e267_d_n7, eq15_e267_d_n8, eq15_e267_d_n9, eq15_e267_d_n10, eq15_e267_d_n11];
        let eq15_reactive_branch_derivatives: [f64; 2] = [eq15_e267_d_b0, eq15_e267_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let eq16_e270: f64 = (p.p3 * var_qb1b2);
        let eq16_e270_d_n0: f64 = (p.p3 * var_qb1b2_dn0);
        let eq16_e270_d_n1: f64 = (p.p3 * var_qb1b2_dn1);
        let eq16_e270_d_n2: f64 = (p.p3 * var_qb1b2_dn2);
        let eq16_e270_d_n3: f64 = (p.p3 * var_qb1b2_dn3);
        let eq16_e270_d_n4: f64 = (p.p3 * var_qb1b2_dn4);
        let eq16_e270_d_n5: f64 = (p.p3 * var_qb1b2_dn5);
        let eq16_e270_d_n6: f64 = (p.p3 * var_qb1b2_dn6);
        let eq16_e270_d_n7: f64 = (p.p3 * var_qb1b2_dn7);
        let eq16_e270_d_n8: f64 = (p.p3 * var_qb1b2_dn8);
        let eq16_e270_d_n9: f64 = (p.p3 * var_qb1b2_dn9);
        let eq16_e270_d_n10: f64 = (p.p3 * var_qb1b2_dn10);
        let eq16_e270_d_n11: f64 = (p.p3 * var_qb1b2_dn11);
        let eq16_e270_d_b0: f64 = (p.p3 * var_qb1b2_db0);
        let eq16_e270_d_b1: f64 = (p.p3 * var_qb1b2_db1);
        let eq16_e271_q: f64 = eq16_e270;
        let eq16_e273: f64 = (eq16_e270 * p.p1);
        let eq16_e273_d_n0: f64 = (eq16_e270_d_n0 * p.p1);
        let eq16_e273_d_n1: f64 = (eq16_e270_d_n1 * p.p1);
        let eq16_e273_d_n2: f64 = (eq16_e270_d_n2 * p.p1);
        let eq16_e273_d_n3: f64 = (eq16_e270_d_n3 * p.p1);
        let eq16_e273_d_n4: f64 = (eq16_e270_d_n4 * p.p1);
        let eq16_e273_d_n5: f64 = (eq16_e270_d_n5 * p.p1);
        let eq16_e273_d_n6: f64 = (eq16_e270_d_n6 * p.p1);
        let eq16_e273_d_n7: f64 = (eq16_e270_d_n7 * p.p1);
        let eq16_e273_d_n8: f64 = (eq16_e270_d_n8 * p.p1);
        let eq16_e273_d_n9: f64 = (eq16_e270_d_n9 * p.p1);
        let eq16_e273_d_n10: f64 = (eq16_e270_d_n10 * p.p1);
        let eq16_e273_d_n11: f64 = (eq16_e270_d_n11 * p.p1);
        let eq16_e273_d_b0: f64 = (eq16_e270_d_b0 * p.p1);
        let eq16_e273_d_b1: f64 = (eq16_e270_d_b1 * p.p1);
        let eq16_e273_q: f64 = (eq16_e271_q * p.p1);
        let eq16_reactive_node_derivatives: [f64; 12] = [eq16_e273_d_n0, eq16_e273_d_n1, eq16_e273_d_n2, eq16_e273_d_n3, eq16_e273_d_n4, eq16_e273_d_n5, eq16_e273_d_n6, eq16_e273_d_n7, eq16_e273_d_n8, eq16_e273_d_n9, eq16_e273_d_n10, eq16_e273_d_n11];
        let eq16_reactive_branch_derivatives: [f64; 2] = [eq16_e273_d_b0, eq16_e273_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq16_reactive_node_derivatives,
            branches,
            &eq16_reactive_branch_derivatives,
            multiplicity,
        );
        let eq17_e276: f64 = (p.p3 * p.p68);
        let eq17_e278: f64 = (eq17_e276 * var_vbe);
        let eq17_e278_d_n0: f64 = (eq17_e276 * var_vbe_dn0);
        let eq17_e278_d_n1: f64 = (eq17_e276 * var_vbe_dn1);
        let eq17_e278_d_n2: f64 = (eq17_e276 * var_vbe_dn2);
        let eq17_e278_d_n3: f64 = (eq17_e276 * var_vbe_dn3);
        let eq17_e278_d_n4: f64 = (eq17_e276 * var_vbe_dn4);
        let eq17_e278_d_n5: f64 = (eq17_e276 * var_vbe_dn5);
        let eq17_e278_d_n6: f64 = (eq17_e276 * var_vbe_dn6);
        let eq17_e278_d_n7: f64 = (eq17_e276 * var_vbe_dn7);
        let eq17_e278_d_n8: f64 = (eq17_e276 * var_vbe_dn8);
        let eq17_e278_d_n9: f64 = (eq17_e276 * var_vbe_dn9);
        let eq17_e278_d_n10: f64 = (eq17_e276 * var_vbe_dn10);
        let eq17_e278_d_n11: f64 = (eq17_e276 * var_vbe_dn11);
        let eq17_e278_d_b0: f64 = (eq17_e276 * var_vbe_db0);
        let eq17_e278_d_b1: f64 = (eq17_e276 * var_vbe_db1);
        let eq17_e279_q: f64 = eq17_e278;
        let eq17_e281: f64 = (eq17_e278 * p.p1);
        let eq17_e281_d_n0: f64 = (eq17_e278_d_n0 * p.p1);
        let eq17_e281_d_n1: f64 = (eq17_e278_d_n1 * p.p1);
        let eq17_e281_d_n2: f64 = (eq17_e278_d_n2 * p.p1);
        let eq17_e281_d_n3: f64 = (eq17_e278_d_n3 * p.p1);
        let eq17_e281_d_n4: f64 = (eq17_e278_d_n4 * p.p1);
        let eq17_e281_d_n5: f64 = (eq17_e278_d_n5 * p.p1);
        let eq17_e281_d_n6: f64 = (eq17_e278_d_n6 * p.p1);
        let eq17_e281_d_n7: f64 = (eq17_e278_d_n7 * p.p1);
        let eq17_e281_d_n8: f64 = (eq17_e278_d_n8 * p.p1);
        let eq17_e281_d_n9: f64 = (eq17_e278_d_n9 * p.p1);
        let eq17_e281_d_n10: f64 = (eq17_e278_d_n10 * p.p1);
        let eq17_e281_d_n11: f64 = (eq17_e278_d_n11 * p.p1);
        let eq17_e281_d_b0: f64 = (eq17_e278_d_b0 * p.p1);
        let eq17_e281_d_b1: f64 = (eq17_e278_d_b1 * p.p1);
        let eq17_e281_q: f64 = (eq17_e279_q * p.p1);
        let eq17_reactive_node_derivatives: [f64; 12] = [eq17_e281_d_n0, eq17_e281_d_n1, eq17_e281_d_n2, eq17_e281_d_n3, eq17_e281_d_n4, eq17_e281_d_n5, eq17_e281_d_n6, eq17_e281_d_n7, eq17_e281_d_n8, eq17_e281_d_n9, eq17_e281_d_n10, eq17_e281_d_n11];
        let eq17_reactive_branch_derivatives: [f64; 2] = [eq17_e281_d_b0, eq17_e281_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes,
            &eq17_reactive_node_derivatives,
            branches,
            &eq17_reactive_branch_derivatives,
            multiplicity,
        );
        let eq18_e284: f64 = (p.p3 * p.p77);
        let eq18_e286: f64 = (eq18_e284 * var_vbc);
        let eq18_e286_d_n0: f64 = (eq18_e284 * var_vbc_dn0);
        let eq18_e286_d_n1: f64 = (eq18_e284 * var_vbc_dn1);
        let eq18_e286_d_n2: f64 = (eq18_e284 * var_vbc_dn2);
        let eq18_e286_d_n3: f64 = (eq18_e284 * var_vbc_dn3);
        let eq18_e286_d_n4: f64 = (eq18_e284 * var_vbc_dn4);
        let eq18_e286_d_n5: f64 = (eq18_e284 * var_vbc_dn5);
        let eq18_e286_d_n6: f64 = (eq18_e284 * var_vbc_dn6);
        let eq18_e286_d_n7: f64 = (eq18_e284 * var_vbc_dn7);
        let eq18_e286_d_n8: f64 = (eq18_e284 * var_vbc_dn8);
        let eq18_e286_d_n9: f64 = (eq18_e284 * var_vbc_dn9);
        let eq18_e286_d_n10: f64 = (eq18_e284 * var_vbc_dn10);
        let eq18_e286_d_n11: f64 = (eq18_e284 * var_vbc_dn11);
        let eq18_e286_d_b0: f64 = (eq18_e284 * var_vbc_db0);
        let eq18_e286_d_b1: f64 = (eq18_e284 * var_vbc_db1);
        let eq18_e287_q: f64 = eq18_e286;
        let eq18_e289: f64 = (eq18_e286 * p.p1);
        let eq18_e289_d_n0: f64 = (eq18_e286_d_n0 * p.p1);
        let eq18_e289_d_n1: f64 = (eq18_e286_d_n1 * p.p1);
        let eq18_e289_d_n2: f64 = (eq18_e286_d_n2 * p.p1);
        let eq18_e289_d_n3: f64 = (eq18_e286_d_n3 * p.p1);
        let eq18_e289_d_n4: f64 = (eq18_e286_d_n4 * p.p1);
        let eq18_e289_d_n5: f64 = (eq18_e286_d_n5 * p.p1);
        let eq18_e289_d_n6: f64 = (eq18_e286_d_n6 * p.p1);
        let eq18_e289_d_n7: f64 = (eq18_e286_d_n7 * p.p1);
        let eq18_e289_d_n8: f64 = (eq18_e286_d_n8 * p.p1);
        let eq18_e289_d_n9: f64 = (eq18_e286_d_n9 * p.p1);
        let eq18_e289_d_n10: f64 = (eq18_e286_d_n10 * p.p1);
        let eq18_e289_d_n11: f64 = (eq18_e286_d_n11 * p.p1);
        let eq18_e289_d_b0: f64 = (eq18_e286_d_b0 * p.p1);
        let eq18_e289_d_b1: f64 = (eq18_e286_d_b1 * p.p1);
        let eq18_e289_q: f64 = (eq18_e287_q * p.p1);
        let eq18_reactive_node_derivatives: [f64; 12] = [eq18_e289_d_n0, eq18_e289_d_n1, eq18_e289_d_n2, eq18_e289_d_n3, eq18_e289_d_n4, eq18_e289_d_n5, eq18_e289_d_n6, eq18_e289_d_n7, eq18_e289_d_n8, eq18_e289_d_n9, eq18_e289_d_n10, eq18_e289_d_n11];
        let eq18_reactive_branch_derivatives: [f64; 2] = [eq18_e289_d_b0, eq18_e289_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let eq21_e305: f64 = (var_xqtex + var_xqex);
        let eq21_e305_d_n0: f64 = (var_xqtex_dn0 + var_xqex_dn0);
        let eq21_e305_d_n1: f64 = (var_xqtex_dn1 + var_xqex_dn1);
        let eq21_e305_d_n2: f64 = (var_xqtex_dn2 + var_xqex_dn2);
        let eq21_e305_d_n3: f64 = (var_xqtex_dn3 + var_xqex_dn3);
        let eq21_e305_d_n4: f64 = (var_xqtex_dn4 + var_xqex_dn4);
        let eq21_e305_d_n5: f64 = (var_xqtex_dn5 + var_xqex_dn5);
        let eq21_e305_d_n6: f64 = (var_xqtex_dn6 + var_xqex_dn6);
        let eq21_e305_d_n7: f64 = (var_xqtex_dn7 + var_xqex_dn7);
        let eq21_e305_d_n8: f64 = (var_xqtex_dn8 + var_xqex_dn8);
        let eq21_e305_d_n9: f64 = (var_xqtex_dn9 + var_xqex_dn9);
        let eq21_e305_d_n10: f64 = (var_xqtex_dn10 + var_xqex_dn10);
        let eq21_e305_d_n11: f64 = (var_xqtex_dn11 + var_xqex_dn11);
        let eq21_e305_d_b0: f64 = (var_xqtex_db0 + var_xqex_db0);
        let eq21_e305_d_b1: f64 = (var_xqtex_db1 + var_xqex_db1);
        let eq21_e306: f64 = (p.p3 * eq21_e305);
        let eq21_e306_d_n0: f64 = (p.p3 * eq21_e305_d_n0);
        let eq21_e306_d_n1: f64 = (p.p3 * eq21_e305_d_n1);
        let eq21_e306_d_n2: f64 = (p.p3 * eq21_e305_d_n2);
        let eq21_e306_d_n3: f64 = (p.p3 * eq21_e305_d_n3);
        let eq21_e306_d_n4: f64 = (p.p3 * eq21_e305_d_n4);
        let eq21_e306_d_n5: f64 = (p.p3 * eq21_e305_d_n5);
        let eq21_e306_d_n6: f64 = (p.p3 * eq21_e305_d_n6);
        let eq21_e306_d_n7: f64 = (p.p3 * eq21_e305_d_n7);
        let eq21_e306_d_n8: f64 = (p.p3 * eq21_e305_d_n8);
        let eq21_e306_d_n9: f64 = (p.p3 * eq21_e305_d_n9);
        let eq21_e306_d_n10: f64 = (p.p3 * eq21_e305_d_n10);
        let eq21_e306_d_n11: f64 = (p.p3 * eq21_e305_d_n11);
        let eq21_e306_d_b0: f64 = (p.p3 * eq21_e305_d_b0);
        let eq21_e306_d_b1: f64 = (p.p3 * eq21_e305_d_b1);
        let eq21_e307_q: f64 = eq21_e306;
        let eq21_e309: f64 = (eq21_e306 * p.p1);
        let eq21_e309_d_n0: f64 = (eq21_e306_d_n0 * p.p1);
        let eq21_e309_d_n1: f64 = (eq21_e306_d_n1 * p.p1);
        let eq21_e309_d_n2: f64 = (eq21_e306_d_n2 * p.p1);
        let eq21_e309_d_n3: f64 = (eq21_e306_d_n3 * p.p1);
        let eq21_e309_d_n4: f64 = (eq21_e306_d_n4 * p.p1);
        let eq21_e309_d_n5: f64 = (eq21_e306_d_n5 * p.p1);
        let eq21_e309_d_n6: f64 = (eq21_e306_d_n6 * p.p1);
        let eq21_e309_d_n7: f64 = (eq21_e306_d_n7 * p.p1);
        let eq21_e309_d_n8: f64 = (eq21_e306_d_n8 * p.p1);
        let eq21_e309_d_n9: f64 = (eq21_e306_d_n9 * p.p1);
        let eq21_e309_d_n10: f64 = (eq21_e306_d_n10 * p.p1);
        let eq21_e309_d_n11: f64 = (eq21_e306_d_n11 * p.p1);
        let eq21_e309_d_b0: f64 = (eq21_e306_d_b0 * p.p1);
        let eq21_e309_d_b1: f64 = (eq21_e306_d_b1 * p.p1);
        let eq21_e309_q: f64 = (eq21_e307_q * p.p1);
        let eq21_reactive_node_derivatives: [f64; 12] = [eq21_e309_d_n0, eq21_e309_d_n1, eq21_e309_d_n2, eq21_e309_d_n3, eq21_e309_d_n4, eq21_e309_d_n5, eq21_e309_d_n6, eq21_e309_d_n7, eq21_e309_d_n8, eq21_e309_d_n9, eq21_e309_d_n10, eq21_e309_d_n11];
        let eq21_reactive_branch_derivatives: [f64; 2] = [eq21_e309_d_b0, eq21_e309_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[9]),
            nodes,
            &eq21_reactive_node_derivatives,
            branches,
            &eq21_reactive_branch_derivatives,
            multiplicity,
        );
        let eq23_e324: f64 = (var_qtex + var_qex);
        let eq23_e324_d_n0: f64 = (var_qtex_dn0 + var_qex_dn0);
        let eq23_e324_d_n1: f64 = (var_qtex_dn1 + var_qex_dn1);
        let eq23_e324_d_n2: f64 = (var_qtex_dn2 + var_qex_dn2);
        let eq23_e324_d_n3: f64 = (var_qtex_dn3 + var_qex_dn3);
        let eq23_e324_d_n4: f64 = (var_qtex_dn4 + var_qex_dn4);
        let eq23_e324_d_n5: f64 = (var_qtex_dn5 + var_qex_dn5);
        let eq23_e324_d_n6: f64 = (var_qtex_dn6 + var_qex_dn6);
        let eq23_e324_d_n7: f64 = (var_qtex_dn7 + var_qex_dn7);
        let eq23_e324_d_n8: f64 = (var_qtex_dn8 + var_qex_dn8);
        let eq23_e324_d_n9: f64 = (var_qtex_dn9 + var_qex_dn9);
        let eq23_e324_d_n10: f64 = (var_qtex_dn10 + var_qex_dn10);
        let eq23_e324_d_n11: f64 = (var_qtex_dn11 + var_qex_dn11);
        let eq23_e324_d_b0: f64 = (var_qtex_db0 + var_qex_db0);
        let eq23_e324_d_b1: f64 = (var_qtex_db1 + var_qex_db1);
        let eq23_e325: f64 = (p.p3 * eq23_e324);
        let eq23_e325_d_n0: f64 = (p.p3 * eq23_e324_d_n0);
        let eq23_e325_d_n1: f64 = (p.p3 * eq23_e324_d_n1);
        let eq23_e325_d_n2: f64 = (p.p3 * eq23_e324_d_n2);
        let eq23_e325_d_n3: f64 = (p.p3 * eq23_e324_d_n3);
        let eq23_e325_d_n4: f64 = (p.p3 * eq23_e324_d_n4);
        let eq23_e325_d_n5: f64 = (p.p3 * eq23_e324_d_n5);
        let eq23_e325_d_n6: f64 = (p.p3 * eq23_e324_d_n6);
        let eq23_e325_d_n7: f64 = (p.p3 * eq23_e324_d_n7);
        let eq23_e325_d_n8: f64 = (p.p3 * eq23_e324_d_n8);
        let eq23_e325_d_n9: f64 = (p.p3 * eq23_e324_d_n9);
        let eq23_e325_d_n10: f64 = (p.p3 * eq23_e324_d_n10);
        let eq23_e325_d_n11: f64 = (p.p3 * eq23_e324_d_n11);
        let eq23_e325_d_b0: f64 = (p.p3 * eq23_e324_d_b0);
        let eq23_e325_d_b1: f64 = (p.p3 * eq23_e324_d_b1);
        let eq23_e326_q: f64 = eq23_e325;
        let eq23_e328: f64 = (eq23_e325 * p.p1);
        let eq23_e328_d_n0: f64 = (eq23_e325_d_n0 * p.p1);
        let eq23_e328_d_n1: f64 = (eq23_e325_d_n1 * p.p1);
        let eq23_e328_d_n2: f64 = (eq23_e325_d_n2 * p.p1);
        let eq23_e328_d_n3: f64 = (eq23_e325_d_n3 * p.p1);
        let eq23_e328_d_n4: f64 = (eq23_e325_d_n4 * p.p1);
        let eq23_e328_d_n5: f64 = (eq23_e325_d_n5 * p.p1);
        let eq23_e328_d_n6: f64 = (eq23_e325_d_n6 * p.p1);
        let eq23_e328_d_n7: f64 = (eq23_e325_d_n7 * p.p1);
        let eq23_e328_d_n8: f64 = (eq23_e325_d_n8 * p.p1);
        let eq23_e328_d_n9: f64 = (eq23_e325_d_n9 * p.p1);
        let eq23_e328_d_n10: f64 = (eq23_e325_d_n10 * p.p1);
        let eq23_e328_d_n11: f64 = (eq23_e325_d_n11 * p.p1);
        let eq23_e328_d_b0: f64 = (eq23_e325_d_b0 * p.p1);
        let eq23_e328_d_b1: f64 = (eq23_e325_d_b1 * p.p1);
        let eq23_e328_q: f64 = (eq23_e326_q * p.p1);
        let eq23_reactive_node_derivatives: [f64; 12] = [eq23_e328_d_n0, eq23_e328_d_n1, eq23_e328_d_n2, eq23_e328_d_n3, eq23_e328_d_n4, eq23_e328_d_n5, eq23_e328_d_n6, eq23_e328_d_n7, eq23_e328_d_n8, eq23_e328_d_n9, eq23_e328_d_n10, eq23_e328_d_n11];
        let eq23_reactive_branch_derivatives: [f64; 2] = [eq23_e328_d_b0, eq23_e328_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[10]),
            nodes,
            &eq23_reactive_node_derivatives,
            branches,
            &eq23_reactive_branch_derivatives,
            multiplicity,
        );
        let eq30_e367_q: f64 = (nv11 - 0.0);
        let eq30_e368: f64 = (var_taun * (nv11 - 0.0));
        let eq30_e368_d_n0: f64 = (var_taun_dn0 * (nv11 - 0.0));
        let eq30_e368_d_n1: f64 = (var_taun_dn1 * (nv11 - 0.0));
        let eq30_e368_d_n2: f64 = (var_taun_dn2 * (nv11 - 0.0));
        let eq30_e368_d_n3: f64 = (var_taun_dn3 * (nv11 - 0.0));
        let eq30_e368_d_n4: f64 = (var_taun_dn4 * (nv11 - 0.0));
        let eq30_e368_d_n5: f64 = (var_taun_dn5 * (nv11 - 0.0));
        let eq30_e368_d_n6: f64 = (var_taun_dn6 * (nv11 - 0.0));
        let eq30_e368_d_n7: f64 = (var_taun_dn7 * (nv11 - 0.0));
        let eq30_e368_d_n8: f64 = (var_taun_dn8 * (nv11 - 0.0));
        let eq30_e368_d_n9: f64 = (var_taun_dn9 * (nv11 - 0.0));
        let eq30_e368_d_n10: f64 = (var_taun_dn10 * (nv11 - 0.0));
        let eq30_e368_d_n11: f64 = ((var_taun_dn11 * (nv11 - 0.0)) + var_taun);
        let eq30_e368_d_b0: f64 = (var_taun_db0 * (nv11 - 0.0));
        let eq30_e368_d_b1: f64 = (var_taun_db1 * (nv11 - 0.0));
        let eq30_e368_q: f64 = (var_taun * eq30_e367_q);
        let eq30_e368_q_d_n0: f64 = (var_taun_dn0 * eq30_e367_q);
        let eq30_e368_q_d_n1: f64 = (var_taun_dn1 * eq30_e367_q);
        let eq30_e368_q_d_n2: f64 = (var_taun_dn2 * eq30_e367_q);
        let eq30_e368_q_d_n3: f64 = (var_taun_dn3 * eq30_e367_q);
        let eq30_e368_q_d_n4: f64 = (var_taun_dn4 * eq30_e367_q);
        let eq30_e368_q_d_n5: f64 = (var_taun_dn5 * eq30_e367_q);
        let eq30_e368_q_d_n6: f64 = (var_taun_dn6 * eq30_e367_q);
        let eq30_e368_q_d_n7: f64 = (var_taun_dn7 * eq30_e367_q);
        let eq30_e368_q_d_n8: f64 = (var_taun_dn8 * eq30_e367_q);
        let eq30_e368_q_d_n9: f64 = (var_taun_dn9 * eq30_e367_q);
        let eq30_e368_q_d_n10: f64 = (var_taun_dn10 * eq30_e367_q);
        let eq30_e368_q_d_n11: f64 = ((var_taun_dn11 * eq30_e367_q) + var_taun);
        let eq30_e368_q_d_b0: f64 = (var_taun_db0 * eq30_e367_q);
        let eq30_e368_q_d_b1: f64 = (var_taun_db1 * eq30_e367_q);
        let eq30_reactive_node_derivatives: [f64; 12] = [eq30_e368_q_d_n0, eq30_e368_q_d_n1, eq30_e368_q_d_n2, eq30_e368_q_d_n3, eq30_e368_q_d_n4, eq30_e368_q_d_n5, eq30_e368_q_d_n6, eq30_e368_q_d_n7, eq30_e368_q_d_n8, eq30_e368_q_d_n9, eq30_e368_q_d_n10, eq30_e368_q_d_n11];
        let eq30_reactive_branch_derivatives: [f64; 2] = [eq30_e368_q_d_b0, eq30_e368_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            nodes,
            &eq30_reactive_node_derivatives,
            branches,
            &eq30_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
