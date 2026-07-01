#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_16(
        p: &Parameters,
        var_a_vde: f64,
        var_a_vde_dn0: f64,
        var_a_vde_dn1: f64,
        var_a_vde_dn10: f64,
        var_a_vde_dn3: f64,
        var_a_vde_dn4: f64,
        var_a_vde_dn5: f64,
        var_a_vde_dn6: f64,
        var_a_vde_dn7: f64,
        var_a_vde_dn8: f64,
        var_a_vde_dn9: f64,
        var_cje_t: f64,
        var_cje_t_dn0: f64,
        var_cje_t_dn1: f64,
        var_cje_t_dn10: f64,
        var_cje_t_dn3: f64,
        var_cje_t_dn4: f64,
        var_cje_t_dn5: f64,
        var_cje_t_dn6: f64,
        var_cje_t_dn7: f64,
        var_cje_t_dn8: f64,
        var_cje_t_dn9: f64,
        var_evb2e1: f64,
        var_evb2e1_dn0: f64,
        var_evb2e1_dn1: f64,
        var_evb2e1_dn10: f64,
        var_evb2e1_dn3: f64,
        var_evb2e1_dn4: f64,
        var_evb2e1_dn5: f64,
        var_evb2e1_dn6: f64,
        var_evb2e1_dn7: f64,
        var_evb2e1_dn8: f64,
        var_evb2e1_dn9: f64,
        var_evbc3: f64,
        var_evbc3_dn0: f64,
        var_evbc3_dn1: f64,
        var_evbc3_dn10: f64,
        var_evbc3_dn5: f64,
        var_evbc3_dn6: f64,
        var_evbc3_dn7: f64,
        var_evbc3_dn8: f64,
        var_evbc3_dn9: f64,
        var_evbc3vdc: f64,
        var_evbc3vdc_dn0: f64,
        var_evbc3vdc_dn1: f64,
        var_evbc3vdc_dn10: f64,
        var_evbc3vdc_dn3: f64,
        var_evbc3vdc_dn4: f64,
        var_evbc3vdc_dn5: f64,
        var_evbc3vdc_dn6: f64,
        var_evbc3vdc_dn7: f64,
        var_evbc3vdc_dn8: f64,
        var_evbc3vdc_dn9: f64,
        var_f1: f64,
        var_f1_dn0: f64,
        var_f1_dn1: f64,
        var_f1_dn10: f64,
        var_f1_dn3: f64,
        var_f1_dn4: f64,
        var_f1_dn5: f64,
        var_f1_dn6: f64,
        var_f1_dn7: f64,
        var_f1_dn8: f64,
        var_f1_dn9: f64,
        var_fex: f64,
        var_fex_dn0: f64,
        var_fex_dn1: f64,
        var_fex_dn10: f64,
        var_fex_dn3: f64,
        var_fex_dn4: f64,
        var_fex_dn5: f64,
        var_fex_dn6: f64,
        var_fex_dn7: f64,
        var_fex_dn8: f64,
        var_fex_dn9: f64,
        var_guard117: f64,
        var_guard118: f64,
        var_ibx_t: f64,
        var_if0: f64,
        var_if0_dn0: f64,
        var_if0_dn1: f64,
        var_if0_dn10: f64,
        var_if0_dn3: f64,
        var_if0_dn4: f64,
        var_if0_dn5: f64,
        var_if0_dn6: f64,
        var_if0_dn7: f64,
        var_if0_dn8: f64,
        var_if0_dn9: f64,
        var_inv_vde_t: f64,
        var_inv_vde_t_dn0: f64,
        var_inv_vde_t_dn1: f64,
        var_inv_vde_t_dn10: f64,
        var_inv_vde_t_dn3: f64,
        var_inv_vde_t_dn4: f64,
        var_inv_vde_t_dn5: f64,
        var_inv_vde_t_dn6: f64,
        var_inv_vde_t_dn7: f64,
        var_inv_vde_t_dn8: f64,
        var_inv_vde_t_dn9: f64,
        var_nff_t: f64,
        var_nff_t_dn0: f64,
        var_nff_t_dn1: f64,
        var_nff_t_dn10: f64,
        var_nff_t_dn3: f64,
        var_nff_t_dn4: f64,
        var_nff_t_dn5: f64,
        var_nff_t_dn6: f64,
        var_nff_t_dn7: f64,
        var_nff_t_dn8: f64,
        var_nff_t_dn9: f64,
        var_q1q: f64,
        var_q1q_dn0: f64,
        var_q1q_dn1: f64,
        var_q1q_dn10: f64,
        var_q1q_dn3: f64,
        var_q1q_dn4: f64,
        var_q1q_dn5: f64,
        var_q1q_dn6: f64,
        var_q1q_dn7: f64,
        var_q1q_dn8: f64,
        var_q1q_dn9: f64,
        var_qb0: f64,
        var_qbe_qs: f64,
        var_qbe_qs_dn0: f64,
        var_qbe_qs_dn1: f64,
        var_qbe_qs_dn10: f64,
        var_qbe_qs_dn3: f64,
        var_qbe_qs_dn4: f64,
        var_qbe_qs_dn5: f64,
        var_qbe_qs_dn6: f64,
        var_qbe_qs_dn7: f64,
        var_qbe_qs_dn8: f64,
        var_qbe_qs_dn9: f64,
        var_qe_qs: f64,
        var_qe_qs_dn0: f64,
        var_qe_qs_dn1: f64,
        var_qe_qs_dn10: f64,
        var_qe_qs_dn3: f64,
        var_qe_qs_dn4: f64,
        var_qe_qs_dn5: f64,
        var_qe_qs_dn6: f64,
        var_qe_qs_dn7: f64,
        var_qe_qs_dn8: f64,
        var_qe_qs_dn9: f64,
        var_qepi0: f64,
        var_taub_t: f64,
        var_tauex_t: f64,
        var_taur_t: f64,
        var_tepi_t: f64,
        var_vb1b2: f64,
        var_vb1b2_dn5: f64,
        var_vb1b2_dn6: f64,
        var_vb2e1: f64,
        var_vb2e1_dn4: f64,
        var_vb2e1_dn6: f64,
        var_vbc3: f64,
        var_vbc3_dn0: f64,
        var_vbc3_dn1: f64,
        var_vbc3_dn10: f64,
        var_vbc3_dn5: f64,
        var_vbc3_dn6: f64,
        var_vbc3_dn7: f64,
        var_vbc3_dn8: f64,
        var_vbc3_dn9: f64,
        var_vdcex_t: f64,
        var_vdcex_t_dn0: f64,
        var_vdcex_t_dn1: f64,
        var_vdcex_t_dn10: f64,
        var_vdcex_t_dn3: f64,
        var_vdcex_t_dn4: f64,
        var_vdcex_t_dn5: f64,
        var_vdcex_t_dn6: f64,
        var_vdcex_t_dn7: f64,
        var_vdcex_t_dn8: f64,
        var_vdcex_t_dn9: f64,
        var_vfe: f64,
        var_vfe_dn0: f64,
        var_vfe_dn1: f64,
        var_vfe_dn10: f64,
        var_vfe_dn3: f64,
        var_vfe_dn4: f64,
        var_vfe_dn5: f64,
        var_vfe_dn6: f64,
        var_vfe_dn7: f64,
        var_vfe_dn8: f64,
        var_vfe_dn9: f64,
        var_vje: f64,
        var_vje_dn0: f64,
        var_vje_dn1: f64,
        var_vje_dn10: f64,
        var_vje_dn3: f64,
        var_vje_dn4: f64,
        var_vje_dn5: f64,
        var_vje_dn6: f64,
        var_vje_dn7: f64,
        var_vje_dn8: f64,
        var_vje_dn9: f64,
        var_vt: f64,
        var_vtinv: f64,
        var_xg1: f64,
        var_xg1_dn0: f64,
        var_xg1_dn1: f64,
        var_xg1_dn10: f64,
        var_xg1_dn3: f64,
        var_xg1_dn4: f64,
        var_xg1_dn5: f64,
        var_xg1_dn6: f64,
        var_xg1_dn7: f64,
        var_xg1_dn8: f64,
        var_xg1_dn9: f64,
        var_dn0vb2e1_slot: &mut f64,
        var_dn0vb2e1_dn0_slot: &mut f64,
        var_dn0vb2e1_dn1_slot: &mut f64,
        var_dn0vb2e1_dn10_slot: &mut f64,
        var_dn0vb2e1_dn3_slot: &mut f64,
        var_dn0vb2e1_dn4_slot: &mut f64,
        var_dn0vb2e1_dn5_slot: &mut f64,
        var_dn0vb2e1_dn6_slot: &mut f64,
        var_dn0vb2e1_dn7_slot: &mut f64,
        var_dn0vb2e1_dn8_slot: &mut f64,
        var_dn0vb2e1_dn9_slot: &mut f64,
        var_dn0vb2e1_rv_slot: &mut f64,
        var_dqbevb2e1_slot: &mut f64,
        var_dqbevb2e1_dn0_slot: &mut f64,
        var_dqbevb2e1_dn1_slot: &mut f64,
        var_dqbevb2e1_dn10_slot: &mut f64,
        var_dqbevb2e1_dn3_slot: &mut f64,
        var_dqbevb2e1_dn4_slot: &mut f64,
        var_dqbevb2e1_dn5_slot: &mut f64,
        var_dqbevb2e1_dn6_slot: &mut f64,
        var_dqbevb2e1_dn7_slot: &mut f64,
        var_dqbevb2e1_dn8_slot: &mut f64,
        var_dqbevb2e1_dn9_slot: &mut f64,
        var_dqbevb2e1_rv_slot: &mut f64,
        var_dqevb2e1_slot: &mut f64,
        var_dqevb2e1_dn0_slot: &mut f64,
        var_dqevb2e1_dn1_slot: &mut f64,
        var_dqevb2e1_dn10_slot: &mut f64,
        var_dqevb2e1_dn3_slot: &mut f64,
        var_dqevb2e1_dn4_slot: &mut f64,
        var_dqevb2e1_dn5_slot: &mut f64,
        var_dqevb2e1_dn6_slot: &mut f64,
        var_dqevb2e1_dn7_slot: &mut f64,
        var_dqevb2e1_dn8_slot: &mut f64,
        var_dqevb2e1_dn9_slot: &mut f64,
        var_dqevb2e1_rv_slot: &mut f64,
        var_dqtevb2e1_slot: &mut f64,
        var_dqtevb2e1_dn0_slot: &mut f64,
        var_dqtevb2e1_dn1_slot: &mut f64,
        var_dqtevb2e1_dn10_slot: &mut f64,
        var_dqtevb2e1_dn3_slot: &mut f64,
        var_dqtevb2e1_dn4_slot: &mut f64,
        var_dqtevb2e1_dn5_slot: &mut f64,
        var_dqtevb2e1_dn6_slot: &mut f64,
        var_dqtevb2e1_dn7_slot: &mut f64,
        var_dqtevb2e1_dn8_slot: &mut f64,
        var_dqtevb2e1_dn9_slot: &mut f64,
        var_dqtevb2e1_rv_slot: &mut f64,
        var_dvjevb2e1_slot: &mut f64,
        var_dvjevb2e1_dn0_slot: &mut f64,
        var_dvjevb2e1_dn1_slot: &mut f64,
        var_dvjevb2e1_dn10_slot: &mut f64,
        var_dvjevb2e1_dn3_slot: &mut f64,
        var_dvjevb2e1_dn4_slot: &mut f64,
        var_dvjevb2e1_dn5_slot: &mut f64,
        var_dvjevb2e1_dn6_slot: &mut f64,
        var_dvjevb2e1_dn7_slot: &mut f64,
        var_dvjevb2e1_dn8_slot: &mut f64,
        var_dvjevb2e1_dn9_slot: &mut f64,
        var_dvjevb2e1_rv_slot: &mut f64,
        var_dvtevb2e1_slot: &mut f64,
        var_dvtevb2e1_dn0_slot: &mut f64,
        var_dvtevb2e1_dn1_slot: &mut f64,
        var_dvtevb2e1_dn10_slot: &mut f64,
        var_dvtevb2e1_dn3_slot: &mut f64,
        var_dvtevb2e1_dn4_slot: &mut f64,
        var_dvtevb2e1_dn5_slot: &mut f64,
        var_dvtevb2e1_dn6_slot: &mut f64,
        var_dvtevb2e1_dn7_slot: &mut f64,
        var_dvtevb2e1_dn8_slot: &mut f64,
        var_dvtevb2e1_dn9_slot: &mut f64,
        var_dvtevb2e1_rv_slot: &mut f64,
        var_dvtevje_slot: &mut f64,
        var_dvtevje_dn0_slot: &mut f64,
        var_dvtevje_dn1_slot: &mut f64,
        var_dvtevje_dn10_slot: &mut f64,
        var_dvtevje_dn3_slot: &mut f64,
        var_dvtevje_dn4_slot: &mut f64,
        var_dvtevje_dn5_slot: &mut f64,
        var_dvtevje_dn6_slot: &mut f64,
        var_dvtevje_dn7_slot: &mut f64,
        var_dvtevje_dn8_slot: &mut f64,
        var_dvtevje_dn9_slot: &mut f64,
        var_dvtevje_rv_slot: &mut f64,
        var_evbc3vdcex_slot: &mut f64,
        var_evbc3vdcex_dn0_slot: &mut f64,
        var_evbc3vdcex_dn1_slot: &mut f64,
        var_evbc3vdcex_dn10_slot: &mut f64,
        var_evbc3vdcex_dn3_slot: &mut f64,
        var_evbc3vdcex_dn4_slot: &mut f64,
        var_evbc3vdcex_dn5_slot: &mut f64,
        var_evbc3vdcex_dn6_slot: &mut f64,
        var_evbc3vdcex_dn7_slot: &mut f64,
        var_evbc3vdcex_dn8_slot: &mut f64,
        var_evbc3vdcex_dn9_slot: &mut f64,
        var_evbc3vdcex_rv_slot: &mut f64,
        var_expl_slot: &mut f64,
        var_expl_rv_slot: &mut f64,
        var_guard119_slot: &mut f64,
        var_guard119_rv_slot: &mut f64,
        var_guard120_slot: &mut f64,
        var_guard120_rv_slot: &mut f64,
        var_guard121_slot: &mut f64,
        var_guard121_rv_slot: &mut f64,
        var_qb1b2_slot: &mut f64,
        var_qb1b2_dn0_slot: &mut f64,
        var_qb1b2_dn1_slot: &mut f64,
        var_qb1b2_dn10_slot: &mut f64,
        var_qb1b2_dn3_slot: &mut f64,
        var_qb1b2_dn4_slot: &mut f64,
        var_qb1b2_dn5_slot: &mut f64,
        var_qb1b2_dn6_slot: &mut f64,
        var_qb1b2_dn7_slot: &mut f64,
        var_qb1b2_dn8_slot: &mut f64,
        var_qb1b2_dn9_slot: &mut f64,
        var_qb1b2_rv_slot: &mut f64,
        var_qbe_qs_eff_slot: &mut f64,
        var_qbe_qs_eff_dn0_slot: &mut f64,
        var_qbe_qs_eff_dn1_slot: &mut f64,
        var_qbe_qs_eff_dn10_slot: &mut f64,
        var_qbe_qs_eff_dn3_slot: &mut f64,
        var_qbe_qs_eff_dn4_slot: &mut f64,
        var_qbe_qs_eff_dn5_slot: &mut f64,
        var_qbe_qs_eff_dn6_slot: &mut f64,
        var_qbe_qs_eff_dn7_slot: &mut f64,
        var_qbe_qs_eff_dn8_slot: &mut f64,
        var_qbe_qs_eff_dn9_slot: &mut f64,
        var_qbe_qs_eff_rv_slot: &mut f64,
        var_qe_slot: &mut f64,
        var_qe_dn0_slot: &mut f64,
        var_qe_dn1_slot: &mut f64,
        var_qe_dn10_slot: &mut f64,
        var_qe_dn3_slot: &mut f64,
        var_qe_dn4_slot: &mut f64,
        var_qe_dn5_slot: &mut f64,
        var_qe_dn6_slot: &mut f64,
        var_qe_dn7_slot: &mut f64,
        var_qe_dn8_slot: &mut f64,
        var_qe_dn9_slot: &mut f64,
        var_qe_rv_slot: &mut f64,
        var_vb2e1vfe_slot: &mut f64,
        var_vb2e1vfe_dn0_slot: &mut f64,
        var_vb2e1vfe_dn1_slot: &mut f64,
        var_vb2e1vfe_dn10_slot: &mut f64,
        var_vb2e1vfe_dn3_slot: &mut f64,
        var_vb2e1vfe_dn4_slot: &mut f64,
        var_vb2e1vfe_dn5_slot: &mut f64,
        var_vb2e1vfe_dn6_slot: &mut f64,
        var_vb2e1vfe_dn7_slot: &mut f64,
        var_vb2e1vfe_dn8_slot: &mut f64,
        var_vb2e1vfe_dn9_slot: &mut f64,
        var_vb2e1vfe_rv_slot: &mut f64,
        var_xg2_slot: &mut f64,
        var_xg2_dn0_slot: &mut f64,
        var_xg2_dn1_slot: &mut f64,
        var_xg2_dn10_slot: &mut f64,
        var_xg2_dn3_slot: &mut f64,
        var_xg2_dn4_slot: &mut f64,
        var_xg2_dn5_slot: &mut f64,
        var_xg2_dn6_slot: &mut f64,
        var_xg2_dn7_slot: &mut f64,
        var_xg2_dn8_slot: &mut f64,
        var_xg2_dn9_slot: &mut f64,
        var_xg2_rv_slot: &mut f64,
        var_xnbex_slot: &mut f64,
        var_xnbex_dn0_slot: &mut f64,
        var_xnbex_dn1_slot: &mut f64,
        var_xnbex_dn10_slot: &mut f64,
        var_xnbex_dn3_slot: &mut f64,
        var_xnbex_dn4_slot: &mut f64,
        var_xnbex_dn5_slot: &mut f64,
        var_xnbex_dn6_slot: &mut f64,
        var_xnbex_dn7_slot: &mut f64,
        var_xnbex_dn8_slot: &mut f64,
        var_xnbex_dn9_slot: &mut f64,
        var_xnbex_rv_slot: &mut f64,
        var_xpwex_slot: &mut f64,
        var_xpwex_dn0_slot: &mut f64,
        var_xpwex_dn1_slot: &mut f64,
        var_xpwex_dn10_slot: &mut f64,
        var_xpwex_dn3_slot: &mut f64,
        var_xpwex_dn4_slot: &mut f64,
        var_xpwex_dn5_slot: &mut f64,
        var_xpwex_dn6_slot: &mut f64,
        var_xpwex_dn7_slot: &mut f64,
        var_xpwex_dn8_slot: &mut f64,
        var_xpwex_dn9_slot: &mut f64,
        var_xpwex_rv_slot: &mut f64,
        var_xqex_slot: &mut f64,
        var_xqex_dn0_slot: &mut f64,
        var_xqex_dn1_slot: &mut f64,
        var_xqex_dn10_slot: &mut f64,
        var_xqex_dn3_slot: &mut f64,
        var_xqex_dn4_slot: &mut f64,
        var_xqex_dn5_slot: &mut f64,
        var_xqex_dn6_slot: &mut f64,
        var_xqex_dn7_slot: &mut f64,
        var_xqex_dn8_slot: &mut f64,
        var_xqex_dn9_slot: &mut f64,
        var_xqex_rv_slot: &mut f64,
        var_xqmex_slot: &mut f64,
        var_xqmex_dn0_slot: &mut f64,
        var_xqmex_dn1_slot: &mut f64,
        var_xqmex_dn10_slot: &mut f64,
        var_xqmex_dn3_slot: &mut f64,
        var_xqmex_dn4_slot: &mut f64,
        var_xqmex_dn5_slot: &mut f64,
        var_xqmex_dn6_slot: &mut f64,
        var_xqmex_dn7_slot: &mut f64,
        var_xqmex_dn8_slot: &mut f64,
        var_xqmex_dn9_slot: &mut f64,
        var_xqmex_rv_slot: &mut f64,
    ) {
        let mut var_dn0vb2e1: f64 = *var_dn0vb2e1_slot;
        let mut var_dn0vb2e1_dn0: f64 = *var_dn0vb2e1_dn0_slot;
        let mut var_dn0vb2e1_dn1: f64 = *var_dn0vb2e1_dn1_slot;
        let mut var_dn0vb2e1_dn10: f64 = *var_dn0vb2e1_dn10_slot;
        let mut var_dn0vb2e1_dn3: f64 = *var_dn0vb2e1_dn3_slot;
        let mut var_dn0vb2e1_dn4: f64 = *var_dn0vb2e1_dn4_slot;
        let mut var_dn0vb2e1_dn5: f64 = *var_dn0vb2e1_dn5_slot;
        let mut var_dn0vb2e1_dn6: f64 = *var_dn0vb2e1_dn6_slot;
        let mut var_dn0vb2e1_dn7: f64 = *var_dn0vb2e1_dn7_slot;
        let mut var_dn0vb2e1_dn8: f64 = *var_dn0vb2e1_dn8_slot;
        let mut var_dn0vb2e1_dn9: f64 = *var_dn0vb2e1_dn9_slot;
        let mut var_dn0vb2e1_rv: f64 = *var_dn0vb2e1_rv_slot;
        let mut var_dqbevb2e1: f64 = *var_dqbevb2e1_slot;
        let mut var_dqbevb2e1_dn0: f64 = *var_dqbevb2e1_dn0_slot;
        let mut var_dqbevb2e1_dn1: f64 = *var_dqbevb2e1_dn1_slot;
        let mut var_dqbevb2e1_dn10: f64 = *var_dqbevb2e1_dn10_slot;
        let mut var_dqbevb2e1_dn3: f64 = *var_dqbevb2e1_dn3_slot;
        let mut var_dqbevb2e1_dn4: f64 = *var_dqbevb2e1_dn4_slot;
        let mut var_dqbevb2e1_dn5: f64 = *var_dqbevb2e1_dn5_slot;
        let mut var_dqbevb2e1_dn6: f64 = *var_dqbevb2e1_dn6_slot;
        let mut var_dqbevb2e1_dn7: f64 = *var_dqbevb2e1_dn7_slot;
        let mut var_dqbevb2e1_dn8: f64 = *var_dqbevb2e1_dn8_slot;
        let mut var_dqbevb2e1_dn9: f64 = *var_dqbevb2e1_dn9_slot;
        let mut var_dqbevb2e1_rv: f64 = *var_dqbevb2e1_rv_slot;
        let mut var_dqevb2e1: f64 = *var_dqevb2e1_slot;
        let mut var_dqevb2e1_dn0: f64 = *var_dqevb2e1_dn0_slot;
        let mut var_dqevb2e1_dn1: f64 = *var_dqevb2e1_dn1_slot;
        let mut var_dqevb2e1_dn10: f64 = *var_dqevb2e1_dn10_slot;
        let mut var_dqevb2e1_dn3: f64 = *var_dqevb2e1_dn3_slot;
        let mut var_dqevb2e1_dn4: f64 = *var_dqevb2e1_dn4_slot;
        let mut var_dqevb2e1_dn5: f64 = *var_dqevb2e1_dn5_slot;
        let mut var_dqevb2e1_dn6: f64 = *var_dqevb2e1_dn6_slot;
        let mut var_dqevb2e1_dn7: f64 = *var_dqevb2e1_dn7_slot;
        let mut var_dqevb2e1_dn8: f64 = *var_dqevb2e1_dn8_slot;
        let mut var_dqevb2e1_dn9: f64 = *var_dqevb2e1_dn9_slot;
        let mut var_dqevb2e1_rv: f64 = *var_dqevb2e1_rv_slot;
        let mut var_dqtevb2e1: f64 = *var_dqtevb2e1_slot;
        let mut var_dqtevb2e1_dn0: f64 = *var_dqtevb2e1_dn0_slot;
        let mut var_dqtevb2e1_dn1: f64 = *var_dqtevb2e1_dn1_slot;
        let mut var_dqtevb2e1_dn10: f64 = *var_dqtevb2e1_dn10_slot;
        let mut var_dqtevb2e1_dn3: f64 = *var_dqtevb2e1_dn3_slot;
        let mut var_dqtevb2e1_dn4: f64 = *var_dqtevb2e1_dn4_slot;
        let mut var_dqtevb2e1_dn5: f64 = *var_dqtevb2e1_dn5_slot;
        let mut var_dqtevb2e1_dn6: f64 = *var_dqtevb2e1_dn6_slot;
        let mut var_dqtevb2e1_dn7: f64 = *var_dqtevb2e1_dn7_slot;
        let mut var_dqtevb2e1_dn8: f64 = *var_dqtevb2e1_dn8_slot;
        let mut var_dqtevb2e1_dn9: f64 = *var_dqtevb2e1_dn9_slot;
        let mut var_dqtevb2e1_rv: f64 = *var_dqtevb2e1_rv_slot;
        let mut var_dvjevb2e1: f64 = *var_dvjevb2e1_slot;
        let mut var_dvjevb2e1_dn0: f64 = *var_dvjevb2e1_dn0_slot;
        let mut var_dvjevb2e1_dn1: f64 = *var_dvjevb2e1_dn1_slot;
        let mut var_dvjevb2e1_dn10: f64 = *var_dvjevb2e1_dn10_slot;
        let mut var_dvjevb2e1_dn3: f64 = *var_dvjevb2e1_dn3_slot;
        let mut var_dvjevb2e1_dn4: f64 = *var_dvjevb2e1_dn4_slot;
        let mut var_dvjevb2e1_dn5: f64 = *var_dvjevb2e1_dn5_slot;
        let mut var_dvjevb2e1_dn6: f64 = *var_dvjevb2e1_dn6_slot;
        let mut var_dvjevb2e1_dn7: f64 = *var_dvjevb2e1_dn7_slot;
        let mut var_dvjevb2e1_dn8: f64 = *var_dvjevb2e1_dn8_slot;
        let mut var_dvjevb2e1_dn9: f64 = *var_dvjevb2e1_dn9_slot;
        let mut var_dvjevb2e1_rv: f64 = *var_dvjevb2e1_rv_slot;
        let mut var_dvtevb2e1: f64 = *var_dvtevb2e1_slot;
        let mut var_dvtevb2e1_dn0: f64 = *var_dvtevb2e1_dn0_slot;
        let mut var_dvtevb2e1_dn1: f64 = *var_dvtevb2e1_dn1_slot;
        let mut var_dvtevb2e1_dn10: f64 = *var_dvtevb2e1_dn10_slot;
        let mut var_dvtevb2e1_dn3: f64 = *var_dvtevb2e1_dn3_slot;
        let mut var_dvtevb2e1_dn4: f64 = *var_dvtevb2e1_dn4_slot;
        let mut var_dvtevb2e1_dn5: f64 = *var_dvtevb2e1_dn5_slot;
        let mut var_dvtevb2e1_dn6: f64 = *var_dvtevb2e1_dn6_slot;
        let mut var_dvtevb2e1_dn7: f64 = *var_dvtevb2e1_dn7_slot;
        let mut var_dvtevb2e1_dn8: f64 = *var_dvtevb2e1_dn8_slot;
        let mut var_dvtevb2e1_dn9: f64 = *var_dvtevb2e1_dn9_slot;
        let mut var_dvtevb2e1_rv: f64 = *var_dvtevb2e1_rv_slot;
        let mut var_dvtevje: f64 = *var_dvtevje_slot;
        let mut var_dvtevje_dn0: f64 = *var_dvtevje_dn0_slot;
        let mut var_dvtevje_dn1: f64 = *var_dvtevje_dn1_slot;
        let mut var_dvtevje_dn10: f64 = *var_dvtevje_dn10_slot;
        let mut var_dvtevje_dn3: f64 = *var_dvtevje_dn3_slot;
        let mut var_dvtevje_dn4: f64 = *var_dvtevje_dn4_slot;
        let mut var_dvtevje_dn5: f64 = *var_dvtevje_dn5_slot;
        let mut var_dvtevje_dn6: f64 = *var_dvtevje_dn6_slot;
        let mut var_dvtevje_dn7: f64 = *var_dvtevje_dn7_slot;
        let mut var_dvtevje_dn8: f64 = *var_dvtevje_dn8_slot;
        let mut var_dvtevje_dn9: f64 = *var_dvtevje_dn9_slot;
        let mut var_dvtevje_rv: f64 = *var_dvtevje_rv_slot;
        let mut var_evbc3vdcex: f64 = *var_evbc3vdcex_slot;
        let mut var_evbc3vdcex_dn0: f64 = *var_evbc3vdcex_dn0_slot;
        let mut var_evbc3vdcex_dn1: f64 = *var_evbc3vdcex_dn1_slot;
        let mut var_evbc3vdcex_dn10: f64 = *var_evbc3vdcex_dn10_slot;
        let mut var_evbc3vdcex_dn3: f64 = *var_evbc3vdcex_dn3_slot;
        let mut var_evbc3vdcex_dn4: f64 = *var_evbc3vdcex_dn4_slot;
        let mut var_evbc3vdcex_dn5: f64 = *var_evbc3vdcex_dn5_slot;
        let mut var_evbc3vdcex_dn6: f64 = *var_evbc3vdcex_dn6_slot;
        let mut var_evbc3vdcex_dn7: f64 = *var_evbc3vdcex_dn7_slot;
        let mut var_evbc3vdcex_dn8: f64 = *var_evbc3vdcex_dn8_slot;
        let mut var_evbc3vdcex_dn9: f64 = *var_evbc3vdcex_dn9_slot;
        let mut var_evbc3vdcex_rv: f64 = *var_evbc3vdcex_rv_slot;
        let mut var_expl: f64 = *var_expl_slot;
        let mut var_expl_rv: f64 = *var_expl_rv_slot;
        let mut var_guard119: f64 = *var_guard119_slot;
        let mut var_guard119_rv: f64 = *var_guard119_rv_slot;
        let mut var_guard120: f64 = *var_guard120_slot;
        let mut var_guard120_rv: f64 = *var_guard120_rv_slot;
        let mut var_guard121: f64 = *var_guard121_slot;
        let mut var_guard121_rv: f64 = *var_guard121_rv_slot;
        let mut var_qb1b2: f64 = *var_qb1b2_slot;
        let mut var_qb1b2_dn0: f64 = *var_qb1b2_dn0_slot;
        let mut var_qb1b2_dn1: f64 = *var_qb1b2_dn1_slot;
        let mut var_qb1b2_dn10: f64 = *var_qb1b2_dn10_slot;
        let mut var_qb1b2_dn3: f64 = *var_qb1b2_dn3_slot;
        let mut var_qb1b2_dn4: f64 = *var_qb1b2_dn4_slot;
        let mut var_qb1b2_dn5: f64 = *var_qb1b2_dn5_slot;
        let mut var_qb1b2_dn6: f64 = *var_qb1b2_dn6_slot;
        let mut var_qb1b2_dn7: f64 = *var_qb1b2_dn7_slot;
        let mut var_qb1b2_dn8: f64 = *var_qb1b2_dn8_slot;
        let mut var_qb1b2_dn9: f64 = *var_qb1b2_dn9_slot;
        let mut var_qb1b2_rv: f64 = *var_qb1b2_rv_slot;
        let mut var_qbe_qs_eff: f64 = *var_qbe_qs_eff_slot;
        let mut var_qbe_qs_eff_dn0: f64 = *var_qbe_qs_eff_dn0_slot;
        let mut var_qbe_qs_eff_dn1: f64 = *var_qbe_qs_eff_dn1_slot;
        let mut var_qbe_qs_eff_dn10: f64 = *var_qbe_qs_eff_dn10_slot;
        let mut var_qbe_qs_eff_dn3: f64 = *var_qbe_qs_eff_dn3_slot;
        let mut var_qbe_qs_eff_dn4: f64 = *var_qbe_qs_eff_dn4_slot;
        let mut var_qbe_qs_eff_dn5: f64 = *var_qbe_qs_eff_dn5_slot;
        let mut var_qbe_qs_eff_dn6: f64 = *var_qbe_qs_eff_dn6_slot;
        let mut var_qbe_qs_eff_dn7: f64 = *var_qbe_qs_eff_dn7_slot;
        let mut var_qbe_qs_eff_dn8: f64 = *var_qbe_qs_eff_dn8_slot;
        let mut var_qbe_qs_eff_dn9: f64 = *var_qbe_qs_eff_dn9_slot;
        let mut var_qbe_qs_eff_rv: f64 = *var_qbe_qs_eff_rv_slot;
        let mut var_qe: f64 = *var_qe_slot;
        let mut var_qe_dn0: f64 = *var_qe_dn0_slot;
        let mut var_qe_dn1: f64 = *var_qe_dn1_slot;
        let mut var_qe_dn10: f64 = *var_qe_dn10_slot;
        let mut var_qe_dn3: f64 = *var_qe_dn3_slot;
        let mut var_qe_dn4: f64 = *var_qe_dn4_slot;
        let mut var_qe_dn5: f64 = *var_qe_dn5_slot;
        let mut var_qe_dn6: f64 = *var_qe_dn6_slot;
        let mut var_qe_dn7: f64 = *var_qe_dn7_slot;
        let mut var_qe_dn8: f64 = *var_qe_dn8_slot;
        let mut var_qe_dn9: f64 = *var_qe_dn9_slot;
        let mut var_qe_rv: f64 = *var_qe_rv_slot;
        let mut var_vb2e1vfe: f64 = *var_vb2e1vfe_slot;
        let mut var_vb2e1vfe_dn0: f64 = *var_vb2e1vfe_dn0_slot;
        let mut var_vb2e1vfe_dn1: f64 = *var_vb2e1vfe_dn1_slot;
        let mut var_vb2e1vfe_dn10: f64 = *var_vb2e1vfe_dn10_slot;
        let mut var_vb2e1vfe_dn3: f64 = *var_vb2e1vfe_dn3_slot;
        let mut var_vb2e1vfe_dn4: f64 = *var_vb2e1vfe_dn4_slot;
        let mut var_vb2e1vfe_dn5: f64 = *var_vb2e1vfe_dn5_slot;
        let mut var_vb2e1vfe_dn6: f64 = *var_vb2e1vfe_dn6_slot;
        let mut var_vb2e1vfe_dn7: f64 = *var_vb2e1vfe_dn7_slot;
        let mut var_vb2e1vfe_dn8: f64 = *var_vb2e1vfe_dn8_slot;
        let mut var_vb2e1vfe_dn9: f64 = *var_vb2e1vfe_dn9_slot;
        let mut var_vb2e1vfe_rv: f64 = *var_vb2e1vfe_rv_slot;
        let mut var_xg2: f64 = *var_xg2_slot;
        let mut var_xg2_dn0: f64 = *var_xg2_dn0_slot;
        let mut var_xg2_dn1: f64 = *var_xg2_dn1_slot;
        let mut var_xg2_dn10: f64 = *var_xg2_dn10_slot;
        let mut var_xg2_dn3: f64 = *var_xg2_dn3_slot;
        let mut var_xg2_dn4: f64 = *var_xg2_dn4_slot;
        let mut var_xg2_dn5: f64 = *var_xg2_dn5_slot;
        let mut var_xg2_dn6: f64 = *var_xg2_dn6_slot;
        let mut var_xg2_dn7: f64 = *var_xg2_dn7_slot;
        let mut var_xg2_dn8: f64 = *var_xg2_dn8_slot;
        let mut var_xg2_dn9: f64 = *var_xg2_dn9_slot;
        let mut var_xg2_rv: f64 = *var_xg2_rv_slot;
        let mut var_xnbex: f64 = *var_xnbex_slot;
        let mut var_xnbex_dn0: f64 = *var_xnbex_dn0_slot;
        let mut var_xnbex_dn1: f64 = *var_xnbex_dn1_slot;
        let mut var_xnbex_dn10: f64 = *var_xnbex_dn10_slot;
        let mut var_xnbex_dn3: f64 = *var_xnbex_dn3_slot;
        let mut var_xnbex_dn4: f64 = *var_xnbex_dn4_slot;
        let mut var_xnbex_dn5: f64 = *var_xnbex_dn5_slot;
        let mut var_xnbex_dn6: f64 = *var_xnbex_dn6_slot;
        let mut var_xnbex_dn7: f64 = *var_xnbex_dn7_slot;
        let mut var_xnbex_dn8: f64 = *var_xnbex_dn8_slot;
        let mut var_xnbex_dn9: f64 = *var_xnbex_dn9_slot;
        let mut var_xnbex_rv: f64 = *var_xnbex_rv_slot;
        let mut var_xpwex: f64 = *var_xpwex_slot;
        let mut var_xpwex_dn0: f64 = *var_xpwex_dn0_slot;
        let mut var_xpwex_dn1: f64 = *var_xpwex_dn1_slot;
        let mut var_xpwex_dn10: f64 = *var_xpwex_dn10_slot;
        let mut var_xpwex_dn3: f64 = *var_xpwex_dn3_slot;
        let mut var_xpwex_dn4: f64 = *var_xpwex_dn4_slot;
        let mut var_xpwex_dn5: f64 = *var_xpwex_dn5_slot;
        let mut var_xpwex_dn6: f64 = *var_xpwex_dn6_slot;
        let mut var_xpwex_dn7: f64 = *var_xpwex_dn7_slot;
        let mut var_xpwex_dn8: f64 = *var_xpwex_dn8_slot;
        let mut var_xpwex_dn9: f64 = *var_xpwex_dn9_slot;
        let mut var_xpwex_rv: f64 = *var_xpwex_rv_slot;
        let mut var_xqex: f64 = *var_xqex_slot;
        let mut var_xqex_dn0: f64 = *var_xqex_dn0_slot;
        let mut var_xqex_dn1: f64 = *var_xqex_dn1_slot;
        let mut var_xqex_dn10: f64 = *var_xqex_dn10_slot;
        let mut var_xqex_dn3: f64 = *var_xqex_dn3_slot;
        let mut var_xqex_dn4: f64 = *var_xqex_dn4_slot;
        let mut var_xqex_dn5: f64 = *var_xqex_dn5_slot;
        let mut var_xqex_dn6: f64 = *var_xqex_dn6_slot;
        let mut var_xqex_dn7: f64 = *var_xqex_dn7_slot;
        let mut var_xqex_dn8: f64 = *var_xqex_dn8_slot;
        let mut var_xqex_dn9: f64 = *var_xqex_dn9_slot;
        let mut var_xqex_rv: f64 = *var_xqex_rv_slot;
        let mut var_xqmex: f64 = *var_xqmex_slot;
        let mut var_xqmex_dn0: f64 = *var_xqmex_dn0_slot;
        let mut var_xqmex_dn1: f64 = *var_xqmex_dn1_slot;
        let mut var_xqmex_dn10: f64 = *var_xqmex_dn10_slot;
        let mut var_xqmex_dn3: f64 = *var_xqmex_dn3_slot;
        let mut var_xqmex_dn4: f64 = *var_xqmex_dn4_slot;
        let mut var_xqmex_dn5: f64 = *var_xqmex_dn5_slot;
        let mut var_xqmex_dn6: f64 = *var_xqmex_dn6_slot;
        let mut var_xqmex_dn7: f64 = *var_xqmex_dn7_slot;
        let mut var_xqmex_dn8: f64 = *var_xqmex_dn8_slot;
        let mut var_xqmex_dn9: f64 = *var_xqmex_dn9_slot;
        let mut var_xqmex_rv: f64 = *var_xqmex_rv_slot;

        let (assign6510_e6704, assign6510_e6704_d_n0, assign6510_e6704_d_n1, assign6510_e6704_d_n3, assign6510_e6704_d_n4, assign6510_e6704_d_n5, assign6510_e6704_d_n6, assign6510_e6704_d_n7, assign6510_e6704_d_n8, assign6510_e6704_d_n9, assign6510_e6704_d_n10,) = {
    if ((var_guard117 != 0.0) && (var_guard118 != 0.0)) {
        let assign6510_e6695: f64 = (var_xg1 - var_if0);
        let assign6510_e6699: f64 = (1.0 + var_xg1);
        let assign6510_e6700: f64 = (assign6510_e6699).sqrt();
        let assign6510_e6701: f64 = (1.0 + assign6510_e6700);
        let assign6510_e6702: f64 = (assign6510_e6695 / assign6510_e6701);
        (assign6510_e6702, ((((var_xg1_dn0 - var_if0_dn0) * assign6510_e6701) - (assign6510_e6695 * (var_xg1_dn0 / (2.0 * assign6510_e6700)))) / (assign6510_e6701 * assign6510_e6701)), ((((var_xg1_dn1 - var_if0_dn1) * assign6510_e6701) - (assign6510_e6695 * (var_xg1_dn1 / (2.0 * assign6510_e6700)))) / (assign6510_e6701 * assign6510_e6701)), ((((var_xg1_dn3 - var_if0_dn3) * assign6510_e6701) - (assign6510_e6695 * (var_xg1_dn3 / (2.0 * assign6510_e6700)))) / (assign6510_e6701 * assign6510_e6701)), ((((var_xg1_dn4 - var_if0_dn4) * assign6510_e6701) - (assign6510_e6695 * (var_xg1_dn4 / (2.0 * assign6510_e6700)))) / (assign6510_e6701 * assign6510_e6701)), ((((var_xg1_dn5 - var_if0_dn5) * assign6510_e6701) - (assign6510_e6695 * (var_xg1_dn5 / (2.0 * assign6510_e6700)))) / (assign6510_e6701 * assign6510_e6701)), ((((var_xg1_dn6 - var_if0_dn6) * assign6510_e6701) - (assign6510_e6695 * (var_xg1_dn6 / (2.0 * assign6510_e6700)))) / (assign6510_e6701 * assign6510_e6701)), ((((var_xg1_dn7 - var_if0_dn7) * assign6510_e6701) - (assign6510_e6695 * (var_xg1_dn7 / (2.0 * assign6510_e6700)))) / (assign6510_e6701 * assign6510_e6701)), ((((var_xg1_dn8 - var_if0_dn8) * assign6510_e6701) - (assign6510_e6695 * (var_xg1_dn8 / (2.0 * assign6510_e6700)))) / (assign6510_e6701 * assign6510_e6701)), ((((var_xg1_dn9 - var_if0_dn9) * assign6510_e6701) - (assign6510_e6695 * (var_xg1_dn9 / (2.0 * assign6510_e6700)))) / (assign6510_e6701 * assign6510_e6701)), ((((var_xg1_dn10 - var_if0_dn10) * assign6510_e6701) - (assign6510_e6695 * (var_xg1_dn10 / (2.0 * assign6510_e6700)))) / (assign6510_e6701 * assign6510_e6701)),)
    } else {
        (var_xnbex, var_xnbex_dn0, var_xnbex_dn1, var_xnbex_dn3, var_xnbex_dn4, var_xnbex_dn5, var_xnbex_dn6, var_xnbex_dn7, var_xnbex_dn8, var_xnbex_dn9, var_xnbex_dn10,)
    }
};
        var_xnbex = assign6510_e6704;
        var_xnbex_dn0 = assign6510_e6704_d_n0;
        var_xnbex_dn1 = assign6510_e6704_d_n1;
        var_xnbex_dn3 = assign6510_e6704_d_n3;
        var_xnbex_dn4 = assign6510_e6704_d_n4;
        var_xnbex_dn5 = assign6510_e6704_d_n5;
        var_xnbex_dn6 = assign6510_e6704_d_n6;
        var_xnbex_dn7 = assign6510_e6704_d_n7;
        var_xnbex_dn8 = assign6510_e6704_d_n8;
        var_xnbex_dn9 = assign6510_e6704_d_n9;
        var_xnbex_dn10 = assign6510_e6704_d_n10;
        var_xnbex_rv = 0.0;

        let (assign6520_e6712, assign6520_e6712_d_n0, assign6520_e6712_d_n1, assign6520_e6712_d_n3, assign6520_e6712_d_n4, assign6520_e6712_d_n5, assign6520_e6712_d_n6, assign6520_e6712_d_n7, assign6520_e6712_d_n8, assign6520_e6712_d_n9, assign6520_e6712_d_n10,) = {
    if ((var_guard117 != 0.0) && (var_guard118 != 0.0)) {
        let assign6520_e6710: f64 = (4.0 * var_evbc3vdc);
        (assign6520_e6710, (4.0 * var_evbc3vdc_dn0), (4.0 * var_evbc3vdc_dn1), (4.0 * var_evbc3vdc_dn3), (4.0 * var_evbc3vdc_dn4), (4.0 * var_evbc3vdc_dn5), (4.0 * var_evbc3vdc_dn6), (4.0 * var_evbc3vdc_dn7), (4.0 * var_evbc3vdc_dn8), (4.0 * var_evbc3vdc_dn9), (4.0 * var_evbc3vdc_dn10),)
    } else {
        (var_xg2, var_xg2_dn0, var_xg2_dn1, var_xg2_dn3, var_xg2_dn4, var_xg2_dn5, var_xg2_dn6, var_xg2_dn7, var_xg2_dn8, var_xg2_dn9, var_xg2_dn10,)
    }
};
        var_xg2 = assign6520_e6712;
        var_xg2_dn0 = assign6520_e6712_d_n0;
        var_xg2_dn1 = assign6520_e6712_d_n1;
        var_xg2_dn3 = assign6520_e6712_d_n3;
        var_xg2_dn4 = assign6520_e6712_d_n4;
        var_xg2_dn5 = assign6520_e6712_d_n5;
        var_xg2_dn6 = assign6520_e6712_d_n6;
        var_xg2_dn7 = assign6520_e6712_d_n7;
        var_xg2_dn8 = assign6520_e6712_d_n8;
        var_xg2_dn9 = assign6520_e6712_d_n9;
        var_xg2_dn10 = assign6520_e6712_d_n10;
        var_xg2_rv = 0.0;

        let (assign6530_e6725, assign6530_e6725_d_n0, assign6530_e6725_d_n1, assign6530_e6725_d_n3, assign6530_e6725_d_n4, assign6530_e6725_d_n5, assign6530_e6725_d_n6, assign6530_e6725_d_n7, assign6530_e6725_d_n8, assign6530_e6725_d_n9, assign6530_e6725_d_n10,) = {
    if ((var_guard117 != 0.0) && (var_guard118 != 0.0)) {
        let assign6530_e6720: f64 = (1.0 + var_xg2);
        let assign6530_e6721: f64 = (assign6530_e6720).sqrt();
        let assign6530_e6722: f64 = (1.0 + assign6530_e6721);
        let assign6530_e6723: f64 = (var_xg2 / assign6530_e6722);
        (assign6530_e6723, (((var_xg2_dn0 * assign6530_e6722) - (var_xg2 * (var_xg2_dn0 / (2.0 * assign6530_e6721)))) / (assign6530_e6722 * assign6530_e6722)), (((var_xg2_dn1 * assign6530_e6722) - (var_xg2 * (var_xg2_dn1 / (2.0 * assign6530_e6721)))) / (assign6530_e6722 * assign6530_e6722)), (((var_xg2_dn3 * assign6530_e6722) - (var_xg2 * (var_xg2_dn3 / (2.0 * assign6530_e6721)))) / (assign6530_e6722 * assign6530_e6722)), (((var_xg2_dn4 * assign6530_e6722) - (var_xg2 * (var_xg2_dn4 / (2.0 * assign6530_e6721)))) / (assign6530_e6722 * assign6530_e6722)), (((var_xg2_dn5 * assign6530_e6722) - (var_xg2 * (var_xg2_dn5 / (2.0 * assign6530_e6721)))) / (assign6530_e6722 * assign6530_e6722)), (((var_xg2_dn6 * assign6530_e6722) - (var_xg2 * (var_xg2_dn6 / (2.0 * assign6530_e6721)))) / (assign6530_e6722 * assign6530_e6722)), (((var_xg2_dn7 * assign6530_e6722) - (var_xg2 * (var_xg2_dn7 / (2.0 * assign6530_e6721)))) / (assign6530_e6722 * assign6530_e6722)), (((var_xg2_dn8 * assign6530_e6722) - (var_xg2 * (var_xg2_dn8 / (2.0 * assign6530_e6721)))) / (assign6530_e6722 * assign6530_e6722)), (((var_xg2_dn9 * assign6530_e6722) - (var_xg2 * (var_xg2_dn9 / (2.0 * assign6530_e6721)))) / (assign6530_e6722 * assign6530_e6722)), (((var_xg2_dn10 * assign6530_e6722) - (var_xg2 * (var_xg2_dn10 / (2.0 * assign6530_e6721)))) / (assign6530_e6722 * assign6530_e6722)),)
    } else {
        (var_xpwex, var_xpwex_dn0, var_xpwex_dn1, var_xpwex_dn3, var_xpwex_dn4, var_xpwex_dn5, var_xpwex_dn6, var_xpwex_dn7, var_xpwex_dn8, var_xpwex_dn9, var_xpwex_dn10,)
    }
};
        var_xpwex = assign6530_e6725;
        var_xpwex_dn0 = assign6530_e6725_d_n0;
        var_xpwex_dn1 = assign6530_e6725_d_n1;
        var_xpwex_dn3 = assign6530_e6725_d_n3;
        var_xpwex_dn4 = assign6530_e6725_d_n4;
        var_xpwex_dn5 = assign6530_e6725_d_n5;
        var_xpwex_dn6 = assign6530_e6725_d_n6;
        var_xpwex_dn7 = assign6530_e6725_d_n7;
        var_xpwex_dn8 = assign6530_e6725_d_n8;
        var_xpwex_dn9 = assign6530_e6725_d_n9;
        var_xpwex_dn10 = assign6530_e6725_d_n10;
        var_xpwex_rv = 0.0;

        let (assign6540_e6747, assign6540_e6747_d_n0, assign6540_e6747_d_n1, assign6540_e6747_d_n3, assign6540_e6747_d_n4, assign6540_e6747_d_n5, assign6540_e6747_d_n6, assign6540_e6747_d_n7, assign6540_e6747_d_n8, assign6540_e6747_d_n9, assign6540_e6747_d_n10,) = {
    if ((var_guard117 != 0.0) && (var_guard118 != 0.0)) {
        let assign6540_e6731: f64 = (0.5 * p.p33);
        let assign6540_e6733: f64 = (assign6540_e6731 * var_taur_t);
        let assign6540_e6736: f64 = (var_qb0 * var_xnbex);
        let assign6540_e6739: f64 = (var_qepi0 * var_xpwex);
        let assign6540_e6740: f64 = (assign6540_e6736 + assign6540_e6739);
        let assign6540_e6741: f64 = (assign6540_e6733 * assign6540_e6740);
        let assign6540_e6744: f64 = (var_taub_t + var_tepi_t);
        let assign6540_e6745: f64 = (assign6540_e6741 / assign6540_e6744);
        (assign6540_e6745, ((assign6540_e6733 * ((var_qb0 * var_xnbex_dn0) + (var_qepi0 * var_xpwex_dn0))) / assign6540_e6744), ((assign6540_e6733 * ((var_qb0 * var_xnbex_dn1) + (var_qepi0 * var_xpwex_dn1))) / assign6540_e6744), ((assign6540_e6733 * ((var_qb0 * var_xnbex_dn3) + (var_qepi0 * var_xpwex_dn3))) / assign6540_e6744), ((assign6540_e6733 * ((var_qb0 * var_xnbex_dn4) + (var_qepi0 * var_xpwex_dn4))) / assign6540_e6744), ((assign6540_e6733 * ((var_qb0 * var_xnbex_dn5) + (var_qepi0 * var_xpwex_dn5))) / assign6540_e6744), ((assign6540_e6733 * ((var_qb0 * var_xnbex_dn6) + (var_qepi0 * var_xpwex_dn6))) / assign6540_e6744), ((assign6540_e6733 * ((var_qb0 * var_xnbex_dn7) + (var_qepi0 * var_xpwex_dn7))) / assign6540_e6744), ((assign6540_e6733 * ((var_qb0 * var_xnbex_dn8) + (var_qepi0 * var_xpwex_dn8))) / assign6540_e6744), ((assign6540_e6733 * ((var_qb0 * var_xnbex_dn9) + (var_qepi0 * var_xpwex_dn9))) / assign6540_e6744), ((assign6540_e6733 * ((var_qb0 * var_xnbex_dn10) + (var_qepi0 * var_xpwex_dn10))) / assign6540_e6744),)
    } else {
        (var_xqmex, var_xqmex_dn0, var_xqmex_dn1, var_xqmex_dn3, var_xqmex_dn4, var_xqmex_dn5, var_xqmex_dn6, var_xqmex_dn7, var_xqmex_dn8, var_xqmex_dn9, var_xqmex_dn10,)
    }
};
        var_xqmex = assign6540_e6747;
        var_xqmex_dn0 = assign6540_e6747_d_n0;
        var_xqmex_dn1 = assign6540_e6747_d_n1;
        var_xqmex_dn3 = assign6540_e6747_d_n3;
        var_xqmex_dn4 = assign6540_e6747_d_n4;
        var_xqmex_dn5 = assign6540_e6747_d_n5;
        var_xqmex_dn6 = assign6540_e6747_d_n6;
        var_xqmex_dn7 = assign6540_e6747_d_n7;
        var_xqmex_dn8 = assign6540_e6747_d_n8;
        var_xqmex_dn9 = assign6540_e6747_d_n9;
        var_xqmex_dn10 = assign6540_e6747_d_n10;
        var_xqmex_rv = 0.0;

        let assign6550_e6750: f64 = (var_vbc3 - var_vdcex_t);
        let assign6550_e6752: f64 = (assign6550_e6750 * var_vtinv);
        let assign6550_e6754: f64 = if assign6550_e6752 < p.p147 { 1.0 } else { 0.0 };
        var_guard119 = assign6550_e6754;
        var_guard119_rv = 0.0;

        let (assign6560_e6768, assign6560_e6768_d_n0, assign6560_e6768_d_n1, assign6560_e6768_d_n3, assign6560_e6768_d_n4, assign6560_e6768_d_n5, assign6560_e6768_d_n6, assign6560_e6768_d_n7, assign6560_e6768_d_n8, assign6560_e6768_d_n9, assign6560_e6768_d_n10,) = {
    if (((var_guard117 != 0.0) && (var_guard118 == 0.0)) && (var_guard119 != 0.0)) {
        let assign6560_e6763: f64 = (var_vbc3 - var_vdcex_t);
        let assign6560_e6765: f64 = (assign6560_e6763 * var_vtinv);
        let assign6560_e6766: f64 = (assign6560_e6765).exp();
        (assign6560_e6766, (assign6560_e6766 * ((var_vbc3_dn0 - var_vdcex_t_dn0) * var_vtinv)), (assign6560_e6766 * ((var_vbc3_dn1 - var_vdcex_t_dn1) * var_vtinv)), (assign6560_e6766 * ((-var_vdcex_t_dn3) * var_vtinv)), (assign6560_e6766 * ((-var_vdcex_t_dn4) * var_vtinv)), (assign6560_e6766 * ((var_vbc3_dn5 - var_vdcex_t_dn5) * var_vtinv)), (assign6560_e6766 * ((var_vbc3_dn6 - var_vdcex_t_dn6) * var_vtinv)), (assign6560_e6766 * ((var_vbc3_dn7 - var_vdcex_t_dn7) * var_vtinv)), (assign6560_e6766 * ((var_vbc3_dn8 - var_vdcex_t_dn8) * var_vtinv)), (assign6560_e6766 * ((var_vbc3_dn9 - var_vdcex_t_dn9) * var_vtinv)), (assign6560_e6766 * ((var_vbc3_dn10 - var_vdcex_t_dn10) * var_vtinv)),)
    } else {
        (var_evbc3vdcex, var_evbc3vdcex_dn0, var_evbc3vdcex_dn1, var_evbc3vdcex_dn3, var_evbc3vdcex_dn4, var_evbc3vdcex_dn5, var_evbc3vdcex_dn6, var_evbc3vdcex_dn7, var_evbc3vdcex_dn8, var_evbc3vdcex_dn9, var_evbc3vdcex_dn10,)
    }
};
        var_evbc3vdcex = assign6560_e6768;
        var_evbc3vdcex_dn0 = assign6560_e6768_d_n0;
        var_evbc3vdcex_dn1 = assign6560_e6768_d_n1;
        var_evbc3vdcex_dn3 = assign6560_e6768_d_n3;
        var_evbc3vdcex_dn4 = assign6560_e6768_d_n4;
        var_evbc3vdcex_dn5 = assign6560_e6768_d_n5;
        var_evbc3vdcex_dn6 = assign6560_e6768_d_n6;
        var_evbc3vdcex_dn7 = assign6560_e6768_d_n7;
        var_evbc3vdcex_dn8 = assign6560_e6768_d_n8;
        var_evbc3vdcex_dn9 = assign6560_e6768_d_n9;
        var_evbc3vdcex_dn10 = assign6560_e6768_d_n10;
        var_evbc3vdcex_rv = 0.0;

        let (assign6570_e6779,) = {
    if (((var_guard117 != 0.0) && (var_guard118 == 0.0)) && (var_guard119 == 0.0)) {
        let assign6570_e6777: f64 = (p.p147).exp();
        (assign6570_e6777,)
    } else {
        (var_expl,)
    }
};
        var_expl = assign6570_e6779;
        var_expl_rv = 0.0;

        let (assign6580_e6799, assign6580_e6799_d_n0, assign6580_e6799_d_n1, assign6580_e6799_d_n3, assign6580_e6799_d_n4, assign6580_e6799_d_n5, assign6580_e6799_d_n6, assign6580_e6799_d_n7, assign6580_e6799_d_n8, assign6580_e6799_d_n9, assign6580_e6799_d_n10,) = {
    if (((var_guard117 != 0.0) && (var_guard118 == 0.0)) && (var_guard119 == 0.0)) {
        let assign6580_e6791: f64 = (var_vbc3 - var_vdcex_t);
        let assign6580_e6793: f64 = (assign6580_e6791 * var_vtinv);
        let assign6580_e6795: f64 = (assign6580_e6793 - p.p147);
        let assign6580_e6796: f64 = (1.0 + assign6580_e6795);
        let assign6580_e6797: f64 = (var_expl * assign6580_e6796);
        (assign6580_e6797, (var_expl * ((var_vbc3_dn0 - var_vdcex_t_dn0) * var_vtinv)), (var_expl * ((var_vbc3_dn1 - var_vdcex_t_dn1) * var_vtinv)), (var_expl * ((-var_vdcex_t_dn3) * var_vtinv)), (var_expl * ((-var_vdcex_t_dn4) * var_vtinv)), (var_expl * ((var_vbc3_dn5 - var_vdcex_t_dn5) * var_vtinv)), (var_expl * ((var_vbc3_dn6 - var_vdcex_t_dn6) * var_vtinv)), (var_expl * ((var_vbc3_dn7 - var_vdcex_t_dn7) * var_vtinv)), (var_expl * ((var_vbc3_dn8 - var_vdcex_t_dn8) * var_vtinv)), (var_expl * ((var_vbc3_dn9 - var_vdcex_t_dn9) * var_vtinv)), (var_expl * ((var_vbc3_dn10 - var_vdcex_t_dn10) * var_vtinv)),)
    } else {
        (var_evbc3vdcex, var_evbc3vdcex_dn0, var_evbc3vdcex_dn1, var_evbc3vdcex_dn3, var_evbc3vdcex_dn4, var_evbc3vdcex_dn5, var_evbc3vdcex_dn6, var_evbc3vdcex_dn7, var_evbc3vdcex_dn8, var_evbc3vdcex_dn9, var_evbc3vdcex_dn10,)
    }
};
        var_evbc3vdcex = assign6580_e6799;
        var_evbc3vdcex_dn0 = assign6580_e6799_d_n0;
        var_evbc3vdcex_dn1 = assign6580_e6799_d_n1;
        var_evbc3vdcex_dn3 = assign6580_e6799_d_n3;
        var_evbc3vdcex_dn4 = assign6580_e6799_d_n4;
        var_evbc3vdcex_dn5 = assign6580_e6799_d_n5;
        var_evbc3vdcex_dn6 = assign6580_e6799_d_n6;
        var_evbc3vdcex_dn7 = assign6580_e6799_d_n7;
        var_evbc3vdcex_dn8 = assign6580_e6799_d_n8;
        var_evbc3vdcex_dn9 = assign6580_e6799_d_n9;
        var_evbc3vdcex_dn10 = assign6580_e6799_d_n10;
        var_evbc3vdcex_rv = 0.0;

        let (assign6590_e6823, assign6590_e6823_d_n0, assign6590_e6823_d_n1, assign6590_e6823_d_n3, assign6590_e6823_d_n4, assign6590_e6823_d_n5, assign6590_e6823_d_n6, assign6590_e6823_d_n7, assign6590_e6823_d_n8, assign6590_e6823_d_n9, assign6590_e6823_d_n10,) = {
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
        (assign6590_e6821, ((((assign6590_e6810 * var_evbc3_dn0) * assign6590_e6820) - (assign6590_e6812 * ((4.0 * var_evbc3vdcex_dn0) / (2.0 * assign6590_e6819)))) / (assign6590_e6820 * assign6590_e6820)), ((((assign6590_e6810 * var_evbc3_dn1) * assign6590_e6820) - (assign6590_e6812 * ((4.0 * var_evbc3vdcex_dn1) / (2.0 * assign6590_e6819)))) / (assign6590_e6820 * assign6590_e6820)), (-((assign6590_e6812 * ((4.0 * var_evbc3vdcex_dn3) / (2.0 * assign6590_e6819))) / (assign6590_e6820 * assign6590_e6820))), (-((assign6590_e6812 * ((4.0 * var_evbc3vdcex_dn4) / (2.0 * assign6590_e6819))) / (assign6590_e6820 * assign6590_e6820))), ((((assign6590_e6810 * var_evbc3_dn5) * assign6590_e6820) - (assign6590_e6812 * ((4.0 * var_evbc3vdcex_dn5) / (2.0 * assign6590_e6819)))) / (assign6590_e6820 * assign6590_e6820)), ((((assign6590_e6810 * var_evbc3_dn6) * assign6590_e6820) - (assign6590_e6812 * ((4.0 * var_evbc3vdcex_dn6) / (2.0 * assign6590_e6819)))) / (assign6590_e6820 * assign6590_e6820)), ((((assign6590_e6810 * var_evbc3_dn7) * assign6590_e6820) - (assign6590_e6812 * ((4.0 * var_evbc3vdcex_dn7) / (2.0 * assign6590_e6819)))) / (assign6590_e6820 * assign6590_e6820)), ((((assign6590_e6810 * var_evbc3_dn8) * assign6590_e6820) - (assign6590_e6812 * ((4.0 * var_evbc3vdcex_dn8) / (2.0 * assign6590_e6819)))) / (assign6590_e6820 * assign6590_e6820)), ((((assign6590_e6810 * var_evbc3_dn9) * assign6590_e6820) - (assign6590_e6812 * ((4.0 * var_evbc3vdcex_dn9) / (2.0 * assign6590_e6819)))) / (assign6590_e6820 * assign6590_e6820)), ((((assign6590_e6810 * var_evbc3_dn10) * assign6590_e6820) - (assign6590_e6812 * ((4.0 * var_evbc3vdcex_dn10) / (2.0 * assign6590_e6819)))) / (assign6590_e6820 * assign6590_e6820)),)
    } else {
        (var_xqmex, var_xqmex_dn0, var_xqmex_dn1, var_xqmex_dn3, var_xqmex_dn4, var_xqmex_dn5, var_xqmex_dn6, var_xqmex_dn7, var_xqmex_dn8, var_xqmex_dn9, var_xqmex_dn10,)
    }
};
        var_xqmex = assign6590_e6823;
        var_xqmex_dn0 = assign6590_e6823_d_n0;
        var_xqmex_dn1 = assign6590_e6823_d_n1;
        var_xqmex_dn3 = assign6590_e6823_d_n3;
        var_xqmex_dn4 = assign6590_e6823_d_n4;
        var_xqmex_dn5 = assign6590_e6823_d_n5;
        var_xqmex_dn6 = assign6590_e6823_d_n6;
        var_xqmex_dn7 = assign6590_e6823_d_n7;
        var_xqmex_dn8 = assign6590_e6823_d_n8;
        var_xqmex_dn9 = assign6590_e6823_d_n9;
        var_xqmex_dn10 = assign6590_e6823_d_n10;
        var_xqmex_rv = 0.0;

        let (assign6600_e6829, assign6600_e6829_d_n0, assign6600_e6829_d_n1, assign6600_e6829_d_n3, assign6600_e6829_d_n4, assign6600_e6829_d_n5, assign6600_e6829_d_n6, assign6600_e6829_d_n7, assign6600_e6829_d_n8, assign6600_e6829_d_n9, assign6600_e6829_d_n10,) = {
    if (var_guard117 != 0.0) {
        let assign6600_e6827: f64 = (var_fex * var_xqmex);
        (assign6600_e6827, ((var_fex_dn0 * var_xqmex) + (var_fex * var_xqmex_dn0)), ((var_fex_dn1 * var_xqmex) + (var_fex * var_xqmex_dn1)), ((var_fex_dn3 * var_xqmex) + (var_fex * var_xqmex_dn3)), ((var_fex_dn4 * var_xqmex) + (var_fex * var_xqmex_dn4)), ((var_fex_dn5 * var_xqmex) + (var_fex * var_xqmex_dn5)), ((var_fex_dn6 * var_xqmex) + (var_fex * var_xqmex_dn6)), ((var_fex_dn7 * var_xqmex) + (var_fex * var_xqmex_dn7)), ((var_fex_dn8 * var_xqmex) + (var_fex * var_xqmex_dn8)), ((var_fex_dn9 * var_xqmex) + (var_fex * var_xqmex_dn9)), ((var_fex_dn10 * var_xqmex) + (var_fex * var_xqmex_dn10)),)
    } else {
        (var_xqex, var_xqex_dn0, var_xqex_dn1, var_xqex_dn3, var_xqex_dn4, var_xqex_dn5, var_xqex_dn6, var_xqex_dn7, var_xqex_dn8, var_xqex_dn9, var_xqex_dn10,)
    }
};
        var_xqex = assign6600_e6829;
        var_xqex_dn0 = assign6600_e6829_d_n0;
        var_xqex_dn1 = assign6600_e6829_d_n1;
        var_xqex_dn3 = assign6600_e6829_d_n3;
        var_xqex_dn4 = assign6600_e6829_d_n4;
        var_xqex_dn5 = assign6600_e6829_d_n5;
        var_xqex_dn6 = assign6600_e6829_d_n6;
        var_xqex_dn7 = assign6600_e6829_d_n7;
        var_xqex_dn8 = assign6600_e6829_d_n8;
        var_xqex_dn9 = assign6600_e6829_d_n9;
        var_xqex_dn10 = assign6600_e6829_d_n10;
        var_xqex_rv = 0.0;

        let assign6610_e6832: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        var_guard120 = assign6610_e6832;
        var_guard120_rv = 0.0;

        let (assign6620_e6845, assign6620_e6845_d_n0, assign6620_e6845_d_n1, assign6620_e6845_d_n3, assign6620_e6845_d_n4, assign6620_e6845_d_n5, assign6620_e6845_d_n6, assign6620_e6845_d_n7, assign6620_e6845_d_n8, assign6620_e6845_d_n9, assign6620_e6845_d_n10,) = {
    if (var_guard120 != 0.0) {
        let assign6620_e6837: f64 = (var_vje * var_inv_vde_t);
        let assign6620_e6838: f64 = (1.0 - assign6620_e6837);
        let assign6620_e6840: f64 = (-p.p67);
        let assign6620_e6841: f64 = (assign6620_e6838).powf(assign6620_e6840);
        let assign6620_e6843: f64 = (assign6620_e6841 - 3.0);
        (assign6620_e6843, if 0.0 == 0.0 && ((assign6620_e6840) as f64).is_finite() && ((assign6620_e6840) as f64).fract() == 0.0 { if assign6620_e6840 == 0.0 { 0.0 } else { (assign6620_e6840 * ((assign6620_e6838).powf(assign6620_e6840 - 1.0) * (-((var_vje_dn0 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn0))))) } } else { (assign6620_e6841 * (assign6620_e6840 * ((-((var_vje_dn0 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn0))) / assign6620_e6838))) }, if 0.0 == 0.0 && ((assign6620_e6840) as f64).is_finite() && ((assign6620_e6840) as f64).fract() == 0.0 { if assign6620_e6840 == 0.0 { 0.0 } else { (assign6620_e6840 * ((assign6620_e6838).powf(assign6620_e6840 - 1.0) * (-((var_vje_dn1 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn1))))) } } else { (assign6620_e6841 * (assign6620_e6840 * ((-((var_vje_dn1 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn1))) / assign6620_e6838))) }, if 0.0 == 0.0 && ((assign6620_e6840) as f64).is_finite() && ((assign6620_e6840) as f64).fract() == 0.0 { if assign6620_e6840 == 0.0 { 0.0 } else { (assign6620_e6840 * ((assign6620_e6838).powf(assign6620_e6840 - 1.0) * (-((var_vje_dn3 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn3))))) } } else { (assign6620_e6841 * (assign6620_e6840 * ((-((var_vje_dn3 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn3))) / assign6620_e6838))) }, if 0.0 == 0.0 && ((assign6620_e6840) as f64).is_finite() && ((assign6620_e6840) as f64).fract() == 0.0 { if assign6620_e6840 == 0.0 { 0.0 } else { (assign6620_e6840 * ((assign6620_e6838).powf(assign6620_e6840 - 1.0) * (-((var_vje_dn4 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn4))))) } } else { (assign6620_e6841 * (assign6620_e6840 * ((-((var_vje_dn4 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn4))) / assign6620_e6838))) }, if 0.0 == 0.0 && ((assign6620_e6840) as f64).is_finite() && ((assign6620_e6840) as f64).fract() == 0.0 { if assign6620_e6840 == 0.0 { 0.0 } else { (assign6620_e6840 * ((assign6620_e6838).powf(assign6620_e6840 - 1.0) * (-((var_vje_dn5 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn5))))) } } else { (assign6620_e6841 * (assign6620_e6840 * ((-((var_vje_dn5 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn5))) / assign6620_e6838))) }, if 0.0 == 0.0 && ((assign6620_e6840) as f64).is_finite() && ((assign6620_e6840) as f64).fract() == 0.0 { if assign6620_e6840 == 0.0 { 0.0 } else { (assign6620_e6840 * ((assign6620_e6838).powf(assign6620_e6840 - 1.0) * (-((var_vje_dn6 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn6))))) } } else { (assign6620_e6841 * (assign6620_e6840 * ((-((var_vje_dn6 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn6))) / assign6620_e6838))) }, if 0.0 == 0.0 && ((assign6620_e6840) as f64).is_finite() && ((assign6620_e6840) as f64).fract() == 0.0 { if assign6620_e6840 == 0.0 { 0.0 } else { (assign6620_e6840 * ((assign6620_e6838).powf(assign6620_e6840 - 1.0) * (-((var_vje_dn7 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn7))))) } } else { (assign6620_e6841 * (assign6620_e6840 * ((-((var_vje_dn7 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn7))) / assign6620_e6838))) }, if 0.0 == 0.0 && ((assign6620_e6840) as f64).is_finite() && ((assign6620_e6840) as f64).fract() == 0.0 { if assign6620_e6840 == 0.0 { 0.0 } else { (assign6620_e6840 * ((assign6620_e6838).powf(assign6620_e6840 - 1.0) * (-((var_vje_dn8 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn8))))) } } else { (assign6620_e6841 * (assign6620_e6840 * ((-((var_vje_dn8 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn8))) / assign6620_e6838))) }, if 0.0 == 0.0 && ((assign6620_e6840) as f64).is_finite() && ((assign6620_e6840) as f64).fract() == 0.0 { if assign6620_e6840 == 0.0 { 0.0 } else { (assign6620_e6840 * ((assign6620_e6838).powf(assign6620_e6840 - 1.0) * (-((var_vje_dn9 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn9))))) } } else { (assign6620_e6841 * (assign6620_e6840 * ((-((var_vje_dn9 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn9))) / assign6620_e6838))) }, if 0.0 == 0.0 && ((assign6620_e6840) as f64).is_finite() && ((assign6620_e6840) as f64).fract() == 0.0 { if assign6620_e6840 == 0.0 { 0.0 } else { (assign6620_e6840 * ((assign6620_e6838).powf(assign6620_e6840 - 1.0) * (-((var_vje_dn10 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn10))))) } } else { (assign6620_e6841 * (assign6620_e6840 * ((-((var_vje_dn10 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn10))) / assign6620_e6838))) },)
    } else {
        (var_dvtevje, var_dvtevje_dn0, var_dvtevje_dn1, var_dvtevje_dn3, var_dvtevje_dn4, var_dvtevje_dn5, var_dvtevje_dn6, var_dvtevje_dn7, var_dvtevje_dn8, var_dvtevje_dn9, var_dvtevje_dn10,)
    }
};
        var_dvtevje = assign6620_e6845;
        var_dvtevje_dn0 = assign6620_e6845_d_n0;
        var_dvtevje_dn1 = assign6620_e6845_d_n1;
        var_dvtevje_dn3 = assign6620_e6845_d_n3;
        var_dvtevje_dn4 = assign6620_e6845_d_n4;
        var_dvtevje_dn5 = assign6620_e6845_d_n5;
        var_dvtevje_dn6 = assign6620_e6845_d_n6;
        var_dvtevje_dn7 = assign6620_e6845_d_n7;
        var_dvtevje_dn8 = assign6620_e6845_d_n8;
        var_dvtevje_dn9 = assign6620_e6845_d_n9;
        var_dvtevje_dn10 = assign6620_e6845_d_n10;
        var_dvtevje_rv = 0.0;

        let (assign6630_e6853, assign6630_e6853_d_n0, assign6630_e6853_d_n1, assign6630_e6853_d_n3, assign6630_e6853_d_n4, assign6630_e6853_d_n5, assign6630_e6853_d_n6, assign6630_e6853_d_n7, assign6630_e6853_d_n8, assign6630_e6853_d_n9, assign6630_e6853_d_n10,) = {
    if (var_guard120 != 0.0) {
        let assign6630_e6849: f64 = (var_vb2e1 - var_vfe);
        let assign6630_e6851: f64 = (assign6630_e6849 / var_a_vde);
        (assign6630_e6851, ((((-var_vfe_dn0) * var_a_vde) - (assign6630_e6849 * var_a_vde_dn0)) / (var_a_vde * var_a_vde)), ((((-var_vfe_dn1) * var_a_vde) - (assign6630_e6849 * var_a_vde_dn1)) / (var_a_vde * var_a_vde)), ((((-var_vfe_dn3) * var_a_vde) - (assign6630_e6849 * var_a_vde_dn3)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn4 - var_vfe_dn4) * var_a_vde) - (assign6630_e6849 * var_a_vde_dn4)) / (var_a_vde * var_a_vde)), ((((-var_vfe_dn5) * var_a_vde) - (assign6630_e6849 * var_a_vde_dn5)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn6 - var_vfe_dn6) * var_a_vde) - (assign6630_e6849 * var_a_vde_dn6)) / (var_a_vde * var_a_vde)), ((((-var_vfe_dn7) * var_a_vde) - (assign6630_e6849 * var_a_vde_dn7)) / (var_a_vde * var_a_vde)), ((((-var_vfe_dn8) * var_a_vde) - (assign6630_e6849 * var_a_vde_dn8)) / (var_a_vde * var_a_vde)), ((((-var_vfe_dn9) * var_a_vde) - (assign6630_e6849 * var_a_vde_dn9)) / (var_a_vde * var_a_vde)), ((((-var_vfe_dn10) * var_a_vde) - (assign6630_e6849 * var_a_vde_dn10)) / (var_a_vde * var_a_vde)),)
    } else {
        (var_vb2e1vfe, var_vb2e1vfe_dn0, var_vb2e1vfe_dn1, var_vb2e1vfe_dn3, var_vb2e1vfe_dn4, var_vb2e1vfe_dn5, var_vb2e1vfe_dn6, var_vb2e1vfe_dn7, var_vb2e1vfe_dn8, var_vb2e1vfe_dn9, var_vb2e1vfe_dn10,)
    }
};
        var_vb2e1vfe = assign6630_e6853;
        var_vb2e1vfe_dn0 = assign6630_e6853_d_n0;
        var_vb2e1vfe_dn1 = assign6630_e6853_d_n1;
        var_vb2e1vfe_dn3 = assign6630_e6853_d_n3;
        var_vb2e1vfe_dn4 = assign6630_e6853_d_n4;
        var_vb2e1vfe_dn5 = assign6630_e6853_d_n5;
        var_vb2e1vfe_dn6 = assign6630_e6853_d_n6;
        var_vb2e1vfe_dn7 = assign6630_e6853_d_n7;
        var_vb2e1vfe_dn8 = assign6630_e6853_d_n8;
        var_vb2e1vfe_dn9 = assign6630_e6853_d_n9;
        var_vb2e1vfe_dn10 = assign6630_e6853_d_n10;
        var_vb2e1vfe_rv = 0.0;

        let assign6640_e6856: f64 = if var_vb2e1vfe < 0.0 { 1.0 } else { 0.0 };
        var_guard121 = assign6640_e6856;
        var_guard121_rv = 0.0;

        let (assign6650_e6867, assign6650_e6867_d_n0, assign6650_e6867_d_n1, assign6650_e6867_d_n3, assign6650_e6867_d_n4, assign6650_e6867_d_n5, assign6650_e6867_d_n6, assign6650_e6867_d_n7, assign6650_e6867_d_n8, assign6650_e6867_d_n9, assign6650_e6867_d_n10,) = {
    if ((var_guard120 != 0.0) && (var_guard121 != 0.0)) {
        let assign6650_e6863: f64 = (var_vb2e1vfe).exp();
        let assign6650_e6864: f64 = (1.0 + assign6650_e6863);
        let assign6650_e6865: f64 = (1.0 / assign6650_e6864);
        (assign6650_e6865, (-((assign6650_e6863 * var_vb2e1vfe_dn0) / (assign6650_e6864 * assign6650_e6864))), (-((assign6650_e6863 * var_vb2e1vfe_dn1) / (assign6650_e6864 * assign6650_e6864))), (-((assign6650_e6863 * var_vb2e1vfe_dn3) / (assign6650_e6864 * assign6650_e6864))), (-((assign6650_e6863 * var_vb2e1vfe_dn4) / (assign6650_e6864 * assign6650_e6864))), (-((assign6650_e6863 * var_vb2e1vfe_dn5) / (assign6650_e6864 * assign6650_e6864))), (-((assign6650_e6863 * var_vb2e1vfe_dn6) / (assign6650_e6864 * assign6650_e6864))), (-((assign6650_e6863 * var_vb2e1vfe_dn7) / (assign6650_e6864 * assign6650_e6864))), (-((assign6650_e6863 * var_vb2e1vfe_dn8) / (assign6650_e6864 * assign6650_e6864))), (-((assign6650_e6863 * var_vb2e1vfe_dn9) / (assign6650_e6864 * assign6650_e6864))), (-((assign6650_e6863 * var_vb2e1vfe_dn10) / (assign6650_e6864 * assign6650_e6864))),)
    } else {
        (var_dvjevb2e1, var_dvjevb2e1_dn0, var_dvjevb2e1_dn1, var_dvjevb2e1_dn3, var_dvjevb2e1_dn4, var_dvjevb2e1_dn5, var_dvjevb2e1_dn6, var_dvjevb2e1_dn7, var_dvjevb2e1_dn8, var_dvjevb2e1_dn9, var_dvjevb2e1_dn10,)
    }
};
        var_dvjevb2e1 = assign6650_e6867;
        var_dvjevb2e1_dn0 = assign6650_e6867_d_n0;
        var_dvjevb2e1_dn1 = assign6650_e6867_d_n1;
        var_dvjevb2e1_dn3 = assign6650_e6867_d_n3;
        var_dvjevb2e1_dn4 = assign6650_e6867_d_n4;
        var_dvjevb2e1_dn5 = assign6650_e6867_d_n5;
        var_dvjevb2e1_dn6 = assign6650_e6867_d_n6;
        var_dvjevb2e1_dn7 = assign6650_e6867_d_n7;
        var_dvjevb2e1_dn8 = assign6650_e6867_d_n8;
        var_dvjevb2e1_dn9 = assign6650_e6867_d_n9;
        var_dvjevb2e1_dn10 = assign6650_e6867_d_n10;
        var_dvjevb2e1_rv = 0.0;

        let (assign6660_e6882, assign6660_e6882_d_n0, assign6660_e6882_d_n1, assign6660_e6882_d_n3, assign6660_e6882_d_n4, assign6660_e6882_d_n5, assign6660_e6882_d_n6, assign6660_e6882_d_n7, assign6660_e6882_d_n8, assign6660_e6882_d_n9, assign6660_e6882_d_n10,) = {
    if ((var_guard120 != 0.0) && (var_guard121 == 0.0)) {
        let assign6660_e6873: f64 = (-var_vb2e1vfe);
        let assign6660_e6874: f64 = (assign6660_e6873).exp();
        let assign6660_e6877: f64 = (-var_vb2e1vfe);
        let assign6660_e6878: f64 = (assign6660_e6877).exp();
        let assign6660_e6879: f64 = (1.0 + assign6660_e6878);
        let assign6660_e6880: f64 = (assign6660_e6874 / assign6660_e6879);
        (assign6660_e6880, ((((assign6660_e6874 * (-var_vb2e1vfe_dn0)) * assign6660_e6879) - (assign6660_e6874 * (assign6660_e6878 * (-var_vb2e1vfe_dn0)))) / (assign6660_e6879 * assign6660_e6879)), ((((assign6660_e6874 * (-var_vb2e1vfe_dn1)) * assign6660_e6879) - (assign6660_e6874 * (assign6660_e6878 * (-var_vb2e1vfe_dn1)))) / (assign6660_e6879 * assign6660_e6879)), ((((assign6660_e6874 * (-var_vb2e1vfe_dn3)) * assign6660_e6879) - (assign6660_e6874 * (assign6660_e6878 * (-var_vb2e1vfe_dn3)))) / (assign6660_e6879 * assign6660_e6879)), ((((assign6660_e6874 * (-var_vb2e1vfe_dn4)) * assign6660_e6879) - (assign6660_e6874 * (assign6660_e6878 * (-var_vb2e1vfe_dn4)))) / (assign6660_e6879 * assign6660_e6879)), ((((assign6660_e6874 * (-var_vb2e1vfe_dn5)) * assign6660_e6879) - (assign6660_e6874 * (assign6660_e6878 * (-var_vb2e1vfe_dn5)))) / (assign6660_e6879 * assign6660_e6879)), ((((assign6660_e6874 * (-var_vb2e1vfe_dn6)) * assign6660_e6879) - (assign6660_e6874 * (assign6660_e6878 * (-var_vb2e1vfe_dn6)))) / (assign6660_e6879 * assign6660_e6879)), ((((assign6660_e6874 * (-var_vb2e1vfe_dn7)) * assign6660_e6879) - (assign6660_e6874 * (assign6660_e6878 * (-var_vb2e1vfe_dn7)))) / (assign6660_e6879 * assign6660_e6879)), ((((assign6660_e6874 * (-var_vb2e1vfe_dn8)) * assign6660_e6879) - (assign6660_e6874 * (assign6660_e6878 * (-var_vb2e1vfe_dn8)))) / (assign6660_e6879 * assign6660_e6879)), ((((assign6660_e6874 * (-var_vb2e1vfe_dn9)) * assign6660_e6879) - (assign6660_e6874 * (assign6660_e6878 * (-var_vb2e1vfe_dn9)))) / (assign6660_e6879 * assign6660_e6879)), ((((assign6660_e6874 * (-var_vb2e1vfe_dn10)) * assign6660_e6879) - (assign6660_e6874 * (assign6660_e6878 * (-var_vb2e1vfe_dn10)))) / (assign6660_e6879 * assign6660_e6879)),)
    } else {
        (var_dvjevb2e1, var_dvjevb2e1_dn0, var_dvjevb2e1_dn1, var_dvjevb2e1_dn3, var_dvjevb2e1_dn4, var_dvjevb2e1_dn5, var_dvjevb2e1_dn6, var_dvjevb2e1_dn7, var_dvjevb2e1_dn8, var_dvjevb2e1_dn9, var_dvjevb2e1_dn10,)
    }
};
        var_dvjevb2e1 = assign6660_e6882;
        var_dvjevb2e1_dn0 = assign6660_e6882_d_n0;
        var_dvjevb2e1_dn1 = assign6660_e6882_d_n1;
        var_dvjevb2e1_dn3 = assign6660_e6882_d_n3;
        var_dvjevb2e1_dn4 = assign6660_e6882_d_n4;
        var_dvjevb2e1_dn5 = assign6660_e6882_d_n5;
        var_dvjevb2e1_dn6 = assign6660_e6882_d_n6;
        var_dvjevb2e1_dn7 = assign6660_e6882_d_n7;
        var_dvjevb2e1_dn8 = assign6660_e6882_d_n8;
        var_dvjevb2e1_dn9 = assign6660_e6882_d_n9;
        var_dvjevb2e1_dn10 = assign6660_e6882_d_n10;
        var_dvjevb2e1_rv = 0.0;

        let (assign6670_e6890, assign6670_e6890_d_n0, assign6670_e6890_d_n1, assign6670_e6890_d_n3, assign6670_e6890_d_n4, assign6670_e6890_d_n5, assign6670_e6890_d_n6, assign6670_e6890_d_n7, assign6670_e6890_d_n8, assign6670_e6890_d_n9, assign6670_e6890_d_n10,) = {
    if (var_guard120 != 0.0) {
        let assign6670_e6886: f64 = (var_dvtevje * var_dvjevb2e1);
        let assign6670_e6888: f64 = (assign6670_e6886 + 3.0);
        (assign6670_e6888, ((var_dvtevje_dn0 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn0)), ((var_dvtevje_dn1 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn1)), ((var_dvtevje_dn3 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn3)), ((var_dvtevje_dn4 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn4)), ((var_dvtevje_dn5 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn5)), ((var_dvtevje_dn6 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn6)), ((var_dvtevje_dn7 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn7)), ((var_dvtevje_dn8 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn8)), ((var_dvtevje_dn9 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn9)), ((var_dvtevje_dn10 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn10)),)
    } else {
        (var_dvtevb2e1, var_dvtevb2e1_dn0, var_dvtevb2e1_dn1, var_dvtevb2e1_dn3, var_dvtevb2e1_dn4, var_dvtevb2e1_dn5, var_dvtevb2e1_dn6, var_dvtevb2e1_dn7, var_dvtevb2e1_dn8, var_dvtevb2e1_dn9, var_dvtevb2e1_dn10,)
    }
};
        var_dvtevb2e1 = assign6670_e6890;
        var_dvtevb2e1_dn0 = assign6670_e6890_d_n0;
        var_dvtevb2e1_dn1 = assign6670_e6890_d_n1;
        var_dvtevb2e1_dn3 = assign6670_e6890_d_n3;
        var_dvtevb2e1_dn4 = assign6670_e6890_d_n4;
        var_dvtevb2e1_dn5 = assign6670_e6890_d_n5;
        var_dvtevb2e1_dn6 = assign6670_e6890_d_n6;
        var_dvtevb2e1_dn7 = assign6670_e6890_d_n7;
        var_dvtevb2e1_dn8 = assign6670_e6890_d_n8;
        var_dvtevb2e1_dn9 = assign6670_e6890_d_n9;
        var_dvtevb2e1_dn10 = assign6670_e6890_d_n10;
        var_dvtevb2e1_rv = 0.0;

        let (assign6680_e6900, assign6680_e6900_d_n0, assign6680_e6900_d_n1, assign6680_e6900_d_n3, assign6680_e6900_d_n4, assign6680_e6900_d_n5, assign6680_e6900_d_n6, assign6680_e6900_d_n7, assign6680_e6900_d_n8, assign6680_e6900_d_n9, assign6680_e6900_d_n10,) = {
    if (var_guard120 != 0.0) {
        let assign6680_e6894: f64 = (1.0 - p.p68);
        let assign6680_e6896: f64 = (assign6680_e6894 * var_cje_t);
        let assign6680_e6898: f64 = (assign6680_e6896 * var_dvtevb2e1);
        (assign6680_e6898, (((assign6680_e6894 * var_cje_t_dn0) * var_dvtevb2e1) + (assign6680_e6896 * var_dvtevb2e1_dn0)), (((assign6680_e6894 * var_cje_t_dn1) * var_dvtevb2e1) + (assign6680_e6896 * var_dvtevb2e1_dn1)), (((assign6680_e6894 * var_cje_t_dn3) * var_dvtevb2e1) + (assign6680_e6896 * var_dvtevb2e1_dn3)), (((assign6680_e6894 * var_cje_t_dn4) * var_dvtevb2e1) + (assign6680_e6896 * var_dvtevb2e1_dn4)), (((assign6680_e6894 * var_cje_t_dn5) * var_dvtevb2e1) + (assign6680_e6896 * var_dvtevb2e1_dn5)), (((assign6680_e6894 * var_cje_t_dn6) * var_dvtevb2e1) + (assign6680_e6896 * var_dvtevb2e1_dn6)), (((assign6680_e6894 * var_cje_t_dn7) * var_dvtevb2e1) + (assign6680_e6896 * var_dvtevb2e1_dn7)), (((assign6680_e6894 * var_cje_t_dn8) * var_dvtevb2e1) + (assign6680_e6896 * var_dvtevb2e1_dn8)), (((assign6680_e6894 * var_cje_t_dn9) * var_dvtevb2e1) + (assign6680_e6896 * var_dvtevb2e1_dn9)), (((assign6680_e6894 * var_cje_t_dn10) * var_dvtevb2e1) + (assign6680_e6896 * var_dvtevb2e1_dn10)),)
    } else {
        (var_dqtevb2e1, var_dqtevb2e1_dn0, var_dqtevb2e1_dn1, var_dqtevb2e1_dn3, var_dqtevb2e1_dn4, var_dqtevb2e1_dn5, var_dqtevb2e1_dn6, var_dqtevb2e1_dn7, var_dqtevb2e1_dn8, var_dqtevb2e1_dn9, var_dqtevb2e1_dn10,)
    }
};
        var_dqtevb2e1 = assign6680_e6900;
        var_dqtevb2e1_dn0 = assign6680_e6900_d_n0;
        var_dqtevb2e1_dn1 = assign6680_e6900_d_n1;
        var_dqtevb2e1_dn3 = assign6680_e6900_d_n3;
        var_dqtevb2e1_dn4 = assign6680_e6900_d_n4;
        var_dqtevb2e1_dn5 = assign6680_e6900_d_n5;
        var_dqtevb2e1_dn6 = assign6680_e6900_d_n6;
        var_dqtevb2e1_dn7 = assign6680_e6900_d_n7;
        var_dqtevb2e1_dn8 = assign6680_e6900_d_n8;
        var_dqtevb2e1_dn9 = assign6680_e6900_d_n9;
        var_dqtevb2e1_dn10 = assign6680_e6900_d_n10;
        var_dqtevb2e1_rv = 0.0;

        let (assign6690_e6917, assign6690_e6917_d_n0, assign6690_e6917_d_n1, assign6690_e6917_d_n3, assign6690_e6917_d_n4, assign6690_e6917_d_n5, assign6690_e6917_d_n6, assign6690_e6917_d_n7, assign6690_e6917_d_n8, assign6690_e6917_d_n9, assign6690_e6917_d_n10,) = {
    if (var_guard120 != 0.0) {
        let assign6690_e6904: f64 = (var_if0 * var_evb2e1);
        let assign6690_e6906: f64 = (assign6690_e6904 * var_vtinv);
        let assign6690_e6908: f64 = (assign6690_e6906 / var_nff_t);
        let assign6690_e6912: f64 = (1.0 + var_f1);
        let assign6690_e6913: f64 = (assign6690_e6912).sqrt();
        let assign6690_e6914: f64 = (0.5 / assign6690_e6913);
        let assign6690_e6915: f64 = (assign6690_e6908 * assign6690_e6914);
        (assign6690_e6915, ((((((((var_if0_dn0 * var_evb2e1) + (var_if0 * var_evb2e1_dn0)) * var_vtinv) * var_nff_t) - (assign6690_e6906 * var_nff_t_dn0)) / (var_nff_t * var_nff_t)) * assign6690_e6914) + (assign6690_e6908 * (-((0.5 * (var_f1_dn0 / (2.0 * assign6690_e6913))) / (assign6690_e6913 * assign6690_e6913))))), ((((((((var_if0_dn1 * var_evb2e1) + (var_if0 * var_evb2e1_dn1)) * var_vtinv) * var_nff_t) - (assign6690_e6906 * var_nff_t_dn1)) / (var_nff_t * var_nff_t)) * assign6690_e6914) + (assign6690_e6908 * (-((0.5 * (var_f1_dn1 / (2.0 * assign6690_e6913))) / (assign6690_e6913 * assign6690_e6913))))), ((((((((var_if0_dn3 * var_evb2e1) + (var_if0 * var_evb2e1_dn3)) * var_vtinv) * var_nff_t) - (assign6690_e6906 * var_nff_t_dn3)) / (var_nff_t * var_nff_t)) * assign6690_e6914) + (assign6690_e6908 * (-((0.5 * (var_f1_dn3 / (2.0 * assign6690_e6913))) / (assign6690_e6913 * assign6690_e6913))))), ((((((((var_if0_dn4 * var_evb2e1) + (var_if0 * var_evb2e1_dn4)) * var_vtinv) * var_nff_t) - (assign6690_e6906 * var_nff_t_dn4)) / (var_nff_t * var_nff_t)) * assign6690_e6914) + (assign6690_e6908 * (-((0.5 * (var_f1_dn4 / (2.0 * assign6690_e6913))) / (assign6690_e6913 * assign6690_e6913))))), ((((((((var_if0_dn5 * var_evb2e1) + (var_if0 * var_evb2e1_dn5)) * var_vtinv) * var_nff_t) - (assign6690_e6906 * var_nff_t_dn5)) / (var_nff_t * var_nff_t)) * assign6690_e6914) + (assign6690_e6908 * (-((0.5 * (var_f1_dn5 / (2.0 * assign6690_e6913))) / (assign6690_e6913 * assign6690_e6913))))), ((((((((var_if0_dn6 * var_evb2e1) + (var_if0 * var_evb2e1_dn6)) * var_vtinv) * var_nff_t) - (assign6690_e6906 * var_nff_t_dn6)) / (var_nff_t * var_nff_t)) * assign6690_e6914) + (assign6690_e6908 * (-((0.5 * (var_f1_dn6 / (2.0 * assign6690_e6913))) / (assign6690_e6913 * assign6690_e6913))))), ((((((((var_if0_dn7 * var_evb2e1) + (var_if0 * var_evb2e1_dn7)) * var_vtinv) * var_nff_t) - (assign6690_e6906 * var_nff_t_dn7)) / (var_nff_t * var_nff_t)) * assign6690_e6914) + (assign6690_e6908 * (-((0.5 * (var_f1_dn7 / (2.0 * assign6690_e6913))) / (assign6690_e6913 * assign6690_e6913))))), ((((((((var_if0_dn8 * var_evb2e1) + (var_if0 * var_evb2e1_dn8)) * var_vtinv) * var_nff_t) - (assign6690_e6906 * var_nff_t_dn8)) / (var_nff_t * var_nff_t)) * assign6690_e6914) + (assign6690_e6908 * (-((0.5 * (var_f1_dn8 / (2.0 * assign6690_e6913))) / (assign6690_e6913 * assign6690_e6913))))), ((((((((var_if0_dn9 * var_evb2e1) + (var_if0 * var_evb2e1_dn9)) * var_vtinv) * var_nff_t) - (assign6690_e6906 * var_nff_t_dn9)) / (var_nff_t * var_nff_t)) * assign6690_e6914) + (assign6690_e6908 * (-((0.5 * (var_f1_dn9 / (2.0 * assign6690_e6913))) / (assign6690_e6913 * assign6690_e6913))))), ((((((((var_if0_dn10 * var_evb2e1) + (var_if0 * var_evb2e1_dn10)) * var_vtinv) * var_nff_t) - (assign6690_e6906 * var_nff_t_dn10)) / (var_nff_t * var_nff_t)) * assign6690_e6914) + (assign6690_e6908 * (-((0.5 * (var_f1_dn10 / (2.0 * assign6690_e6913))) / (assign6690_e6913 * assign6690_e6913))))),)
    } else {
        (var_dn0vb2e1, var_dn0vb2e1_dn0, var_dn0vb2e1_dn1, var_dn0vb2e1_dn3, var_dn0vb2e1_dn4, var_dn0vb2e1_dn5, var_dn0vb2e1_dn6, var_dn0vb2e1_dn7, var_dn0vb2e1_dn8, var_dn0vb2e1_dn9, var_dn0vb2e1_dn10,)
    }
};
        var_dn0vb2e1 = assign6690_e6917;
        var_dn0vb2e1_dn0 = assign6690_e6917_d_n0;
        var_dn0vb2e1_dn1 = assign6690_e6917_d_n1;
        var_dn0vb2e1_dn3 = assign6690_e6917_d_n3;
        var_dn0vb2e1_dn4 = assign6690_e6917_d_n4;
        var_dn0vb2e1_dn5 = assign6690_e6917_d_n5;
        var_dn0vb2e1_dn6 = assign6690_e6917_d_n6;
        var_dn0vb2e1_dn7 = assign6690_e6917_d_n7;
        var_dn0vb2e1_dn8 = assign6690_e6917_d_n8;
        var_dn0vb2e1_dn9 = assign6690_e6917_d_n9;
        var_dn0vb2e1_dn10 = assign6690_e6917_d_n10;
        var_dn0vb2e1_rv = 0.0;

        let (assign6700_e6927, assign6700_e6927_d_n0, assign6700_e6927_d_n1, assign6700_e6927_d_n3, assign6700_e6927_d_n4, assign6700_e6927_d_n5, assign6700_e6927_d_n6, assign6700_e6927_d_n7, assign6700_e6927_d_n8, assign6700_e6927_d_n9, assign6700_e6927_d_n10,) = {
    if (var_guard120 != 0.0) {
        let assign6700_e6921: f64 = (0.5 * var_qb0);
        let assign6700_e6923: f64 = (assign6700_e6921 * var_q1q);
        let assign6700_e6925: f64 = (assign6700_e6923 * var_dn0vb2e1);
        (assign6700_e6925, (((assign6700_e6921 * var_q1q_dn0) * var_dn0vb2e1) + (assign6700_e6923 * var_dn0vb2e1_dn0)), (((assign6700_e6921 * var_q1q_dn1) * var_dn0vb2e1) + (assign6700_e6923 * var_dn0vb2e1_dn1)), (((assign6700_e6921 * var_q1q_dn3) * var_dn0vb2e1) + (assign6700_e6923 * var_dn0vb2e1_dn3)), (((assign6700_e6921 * var_q1q_dn4) * var_dn0vb2e1) + (assign6700_e6923 * var_dn0vb2e1_dn4)), (((assign6700_e6921 * var_q1q_dn5) * var_dn0vb2e1) + (assign6700_e6923 * var_dn0vb2e1_dn5)), (((assign6700_e6921 * var_q1q_dn6) * var_dn0vb2e1) + (assign6700_e6923 * var_dn0vb2e1_dn6)), (((assign6700_e6921 * var_q1q_dn7) * var_dn0vb2e1) + (assign6700_e6923 * var_dn0vb2e1_dn7)), (((assign6700_e6921 * var_q1q_dn8) * var_dn0vb2e1) + (assign6700_e6923 * var_dn0vb2e1_dn8)), (((assign6700_e6921 * var_q1q_dn9) * var_dn0vb2e1) + (assign6700_e6923 * var_dn0vb2e1_dn9)), (((assign6700_e6921 * var_q1q_dn10) * var_dn0vb2e1) + (assign6700_e6923 * var_dn0vb2e1_dn10)),)
    } else {
        (var_dqbevb2e1, var_dqbevb2e1_dn0, var_dqbevb2e1_dn1, var_dqbevb2e1_dn3, var_dqbevb2e1_dn4, var_dqbevb2e1_dn5, var_dqbevb2e1_dn6, var_dqbevb2e1_dn7, var_dqbevb2e1_dn8, var_dqbevb2e1_dn9, var_dqbevb2e1_dn10,)
    }
};
        var_dqbevb2e1 = assign6700_e6927;
        var_dqbevb2e1_dn0 = assign6700_e6927_d_n0;
        var_dqbevb2e1_dn1 = assign6700_e6927_d_n1;
        var_dqbevb2e1_dn3 = assign6700_e6927_d_n3;
        var_dqbevb2e1_dn4 = assign6700_e6927_d_n4;
        var_dqbevb2e1_dn5 = assign6700_e6927_d_n5;
        var_dqbevb2e1_dn6 = assign6700_e6927_d_n6;
        var_dqbevb2e1_dn7 = assign6700_e6927_d_n7;
        var_dqbevb2e1_dn8 = assign6700_e6927_d_n8;
        var_dqbevb2e1_dn9 = assign6700_e6927_d_n9;
        var_dqbevb2e1_dn10 = assign6700_e6927_d_n10;
        var_dqbevb2e1_rv = 0.0;

        let (assign6710_e6935, assign6710_e6935_d_n0, assign6710_e6935_d_n1, assign6710_e6935_d_n3, assign6710_e6935_d_n4, assign6710_e6935_d_n5, assign6710_e6935_d_n6, assign6710_e6935_d_n7, assign6710_e6935_d_n8, assign6710_e6935_d_n9, assign6710_e6935_d_n10,) = {
    if (var_guard120 != 0.0) {
        let assign6710_e6932: f64 = (p.p85 * var_vt);
        let assign6710_e6933: f64 = (var_qe_qs / assign6710_e6932);
        (assign6710_e6933, (var_qe_qs_dn0 / assign6710_e6932), (var_qe_qs_dn1 / assign6710_e6932), (var_qe_qs_dn3 / assign6710_e6932), (var_qe_qs_dn4 / assign6710_e6932), (var_qe_qs_dn5 / assign6710_e6932), (var_qe_qs_dn6 / assign6710_e6932), (var_qe_qs_dn7 / assign6710_e6932), (var_qe_qs_dn8 / assign6710_e6932), (var_qe_qs_dn9 / assign6710_e6932), (var_qe_qs_dn10 / assign6710_e6932),)
    } else {
        (var_dqevb2e1, var_dqevb2e1_dn0, var_dqevb2e1_dn1, var_dqevb2e1_dn3, var_dqevb2e1_dn4, var_dqevb2e1_dn5, var_dqevb2e1_dn6, var_dqevb2e1_dn7, var_dqevb2e1_dn8, var_dqevb2e1_dn9, var_dqevb2e1_dn10,)
    }
};
        var_dqevb2e1 = assign6710_e6935;
        var_dqevb2e1_dn0 = assign6710_e6935_d_n0;
        var_dqevb2e1_dn1 = assign6710_e6935_d_n1;
        var_dqevb2e1_dn3 = assign6710_e6935_d_n3;
        var_dqevb2e1_dn4 = assign6710_e6935_d_n4;
        var_dqevb2e1_dn5 = assign6710_e6935_d_n5;
        var_dqevb2e1_dn6 = assign6710_e6935_d_n6;
        var_dqevb2e1_dn7 = assign6710_e6935_d_n7;
        var_dqevb2e1_dn8 = assign6710_e6935_d_n8;
        var_dqevb2e1_dn9 = assign6710_e6935_d_n9;
        var_dqevb2e1_dn10 = assign6710_e6935_d_n10;
        var_dqevb2e1_rv = 0.0;

        let (assign6720_e6947, assign6720_e6947_d_n0, assign6720_e6947_d_n1, assign6720_e6947_d_n3, assign6720_e6947_d_n4, assign6720_e6947_d_n5, assign6720_e6947_d_n6, assign6720_e6947_d_n7, assign6720_e6947_d_n8, assign6720_e6947_d_n9, assign6720_e6947_d_n10,) = {
    if (var_guard120 != 0.0) {
        let assign6720_e6939: f64 = (0.2 * var_vb1b2);
        let assign6720_e6942: f64 = (var_dqtevb2e1 + var_dqbevb2e1);
        let assign6720_e6944: f64 = (assign6720_e6942 + var_dqevb2e1);
        let assign6720_e6945: f64 = (assign6720_e6939 * assign6720_e6944);
        (assign6720_e6945, (assign6720_e6939 * ((var_dqtevb2e1_dn0 + var_dqbevb2e1_dn0) + var_dqevb2e1_dn0)), (assign6720_e6939 * ((var_dqtevb2e1_dn1 + var_dqbevb2e1_dn1) + var_dqevb2e1_dn1)), (assign6720_e6939 * ((var_dqtevb2e1_dn3 + var_dqbevb2e1_dn3) + var_dqevb2e1_dn3)), (assign6720_e6939 * ((var_dqtevb2e1_dn4 + var_dqbevb2e1_dn4) + var_dqevb2e1_dn4)), (((0.2 * var_vb1b2_dn5) * assign6720_e6944) + (assign6720_e6939 * ((var_dqtevb2e1_dn5 + var_dqbevb2e1_dn5) + var_dqevb2e1_dn5))), (((0.2 * var_vb1b2_dn6) * assign6720_e6944) + (assign6720_e6939 * ((var_dqtevb2e1_dn6 + var_dqbevb2e1_dn6) + var_dqevb2e1_dn6))), (assign6720_e6939 * ((var_dqtevb2e1_dn7 + var_dqbevb2e1_dn7) + var_dqevb2e1_dn7)), (assign6720_e6939 * ((var_dqtevb2e1_dn8 + var_dqbevb2e1_dn8) + var_dqevb2e1_dn8)), (assign6720_e6939 * ((var_dqtevb2e1_dn9 + var_dqbevb2e1_dn9) + var_dqevb2e1_dn9)), (assign6720_e6939 * ((var_dqtevb2e1_dn10 + var_dqbevb2e1_dn10) + var_dqevb2e1_dn10)),)
    } else {
        (var_qb1b2, var_qb1b2_dn0, var_qb1b2_dn1, var_qb1b2_dn3, var_qb1b2_dn4, var_qb1b2_dn5, var_qb1b2_dn6, var_qb1b2_dn7, var_qb1b2_dn8, var_qb1b2_dn9, var_qb1b2_dn10,)
    }
};
        var_qb1b2 = assign6720_e6947;
        var_qb1b2_dn0 = assign6720_e6947_d_n0;
        var_qb1b2_dn1 = assign6720_e6947_d_n1;
        var_qb1b2_dn3 = assign6720_e6947_d_n3;
        var_qb1b2_dn4 = assign6720_e6947_d_n4;
        var_qb1b2_dn5 = assign6720_e6947_d_n5;
        var_qb1b2_dn6 = assign6720_e6947_d_n6;
        var_qb1b2_dn7 = assign6720_e6947_d_n7;
        var_qb1b2_dn8 = assign6720_e6947_d_n8;
        var_qb1b2_dn9 = assign6720_e6947_d_n9;
        var_qb1b2_dn10 = assign6720_e6947_d_n10;
        var_qb1b2_rv = 0.0;

        let (assign6730_e6955, assign6730_e6955_d_n0, assign6730_e6955_d_n1, assign6730_e6955_d_n3, assign6730_e6955_d_n4, assign6730_e6955_d_n5, assign6730_e6955_d_n6, assign6730_e6955_d_n7, assign6730_e6955_d_n8, assign6730_e6955_d_n9, assign6730_e6955_d_n10,) = {
    if (var_guard120 != 0.0) {
        let assign6730_e6951: f64 = (1.0 - p.p95);
        let assign6730_e6953: f64 = (assign6730_e6951 * var_qe_qs);
        (assign6730_e6953, (assign6730_e6951 * var_qe_qs_dn0), (assign6730_e6951 * var_qe_qs_dn1), (assign6730_e6951 * var_qe_qs_dn3), (assign6730_e6951 * var_qe_qs_dn4), (assign6730_e6951 * var_qe_qs_dn5), (assign6730_e6951 * var_qe_qs_dn6), (assign6730_e6951 * var_qe_qs_dn7), (assign6730_e6951 * var_qe_qs_dn8), (assign6730_e6951 * var_qe_qs_dn9), (assign6730_e6951 * var_qe_qs_dn10),)
    } else {
        (var_qe, var_qe_dn0, var_qe_dn1, var_qe_dn3, var_qe_dn4, var_qe_dn5, var_qe_dn6, var_qe_dn7, var_qe_dn8, var_qe_dn9, var_qe_dn10,)
    }
};
        var_qe = assign6730_e6955;
        var_qe_dn0 = assign6730_e6955_d_n0;
        var_qe_dn1 = assign6730_e6955_d_n1;
        var_qe_dn3 = assign6730_e6955_d_n3;
        var_qe_dn4 = assign6730_e6955_d_n4;
        var_qe_dn5 = assign6730_e6955_d_n5;
        var_qe_dn6 = assign6730_e6955_d_n6;
        var_qe_dn7 = assign6730_e6955_d_n7;
        var_qe_dn8 = assign6730_e6955_d_n8;
        var_qe_dn9 = assign6730_e6955_d_n9;
        var_qe_dn10 = assign6730_e6955_d_n10;
        var_qe_rv = 0.0;

        let (assign6740_e6963, assign6740_e6963_d_n0, assign6740_e6963_d_n1, assign6740_e6963_d_n3, assign6740_e6963_d_n4, assign6740_e6963_d_n5, assign6740_e6963_d_n6, assign6740_e6963_d_n7, assign6740_e6963_d_n8, assign6740_e6963_d_n9, assign6740_e6963_d_n10,) = {
    if (var_guard120 != 0.0) {
        let assign6740_e6960: f64 = (p.p95 * var_qe_qs);
        let assign6740_e6961: f64 = (var_qbe_qs + assign6740_e6960);
        (assign6740_e6961, (var_qbe_qs_dn0 + (p.p95 * var_qe_qs_dn0)), (var_qbe_qs_dn1 + (p.p95 * var_qe_qs_dn1)), (var_qbe_qs_dn3 + (p.p95 * var_qe_qs_dn3)), (var_qbe_qs_dn4 + (p.p95 * var_qe_qs_dn4)), (var_qbe_qs_dn5 + (p.p95 * var_qe_qs_dn5)), (var_qbe_qs_dn6 + (p.p95 * var_qe_qs_dn6)), (var_qbe_qs_dn7 + (p.p95 * var_qe_qs_dn7)), (var_qbe_qs_dn8 + (p.p95 * var_qe_qs_dn8)), (var_qbe_qs_dn9 + (p.p95 * var_qe_qs_dn9)), (var_qbe_qs_dn10 + (p.p95 * var_qe_qs_dn10)),)
    } else {
        (var_qbe_qs_eff, var_qbe_qs_eff_dn0, var_qbe_qs_eff_dn1, var_qbe_qs_eff_dn3, var_qbe_qs_eff_dn4, var_qbe_qs_eff_dn5, var_qbe_qs_eff_dn6, var_qbe_qs_eff_dn7, var_qbe_qs_eff_dn8, var_qbe_qs_eff_dn9, var_qbe_qs_eff_dn10,)
    }
};
        var_qbe_qs_eff = assign6740_e6963;
        var_qbe_qs_eff_dn0 = assign6740_e6963_d_n0;
        var_qbe_qs_eff_dn1 = assign6740_e6963_d_n1;
        var_qbe_qs_eff_dn3 = assign6740_e6963_d_n3;
        var_qbe_qs_eff_dn4 = assign6740_e6963_d_n4;
        var_qbe_qs_eff_dn5 = assign6740_e6963_d_n5;
        var_qbe_qs_eff_dn6 = assign6740_e6963_d_n6;
        var_qbe_qs_eff_dn7 = assign6740_e6963_d_n7;
        var_qbe_qs_eff_dn8 = assign6740_e6963_d_n8;
        var_qbe_qs_eff_dn9 = assign6740_e6963_d_n9;
        var_qbe_qs_eff_dn10 = assign6740_e6963_d_n10;
        var_qbe_qs_eff_rv = 0.0;

        *var_dn0vb2e1_slot = var_dn0vb2e1;
        *var_dn0vb2e1_dn0_slot = var_dn0vb2e1_dn0;
        *var_dn0vb2e1_dn1_slot = var_dn0vb2e1_dn1;
        *var_dn0vb2e1_dn10_slot = var_dn0vb2e1_dn10;
        *var_dn0vb2e1_dn3_slot = var_dn0vb2e1_dn3;
        *var_dn0vb2e1_dn4_slot = var_dn0vb2e1_dn4;
        *var_dn0vb2e1_dn5_slot = var_dn0vb2e1_dn5;
        *var_dn0vb2e1_dn6_slot = var_dn0vb2e1_dn6;
        *var_dn0vb2e1_dn7_slot = var_dn0vb2e1_dn7;
        *var_dn0vb2e1_dn8_slot = var_dn0vb2e1_dn8;
        *var_dn0vb2e1_dn9_slot = var_dn0vb2e1_dn9;
        *var_dn0vb2e1_rv_slot = var_dn0vb2e1_rv;
        *var_dqbevb2e1_slot = var_dqbevb2e1;
        *var_dqbevb2e1_dn0_slot = var_dqbevb2e1_dn0;
        *var_dqbevb2e1_dn1_slot = var_dqbevb2e1_dn1;
        *var_dqbevb2e1_dn10_slot = var_dqbevb2e1_dn10;
        *var_dqbevb2e1_dn3_slot = var_dqbevb2e1_dn3;
        *var_dqbevb2e1_dn4_slot = var_dqbevb2e1_dn4;
        *var_dqbevb2e1_dn5_slot = var_dqbevb2e1_dn5;
        *var_dqbevb2e1_dn6_slot = var_dqbevb2e1_dn6;
        *var_dqbevb2e1_dn7_slot = var_dqbevb2e1_dn7;
        *var_dqbevb2e1_dn8_slot = var_dqbevb2e1_dn8;
        *var_dqbevb2e1_dn9_slot = var_dqbevb2e1_dn9;
        *var_dqbevb2e1_rv_slot = var_dqbevb2e1_rv;
        *var_dqevb2e1_slot = var_dqevb2e1;
        *var_dqevb2e1_dn0_slot = var_dqevb2e1_dn0;
        *var_dqevb2e1_dn1_slot = var_dqevb2e1_dn1;
        *var_dqevb2e1_dn10_slot = var_dqevb2e1_dn10;
        *var_dqevb2e1_dn3_slot = var_dqevb2e1_dn3;
        *var_dqevb2e1_dn4_slot = var_dqevb2e1_dn4;
        *var_dqevb2e1_dn5_slot = var_dqevb2e1_dn5;
        *var_dqevb2e1_dn6_slot = var_dqevb2e1_dn6;
        *var_dqevb2e1_dn7_slot = var_dqevb2e1_dn7;
        *var_dqevb2e1_dn8_slot = var_dqevb2e1_dn8;
        *var_dqevb2e1_dn9_slot = var_dqevb2e1_dn9;
        *var_dqevb2e1_rv_slot = var_dqevb2e1_rv;
        *var_dqtevb2e1_slot = var_dqtevb2e1;
        *var_dqtevb2e1_dn0_slot = var_dqtevb2e1_dn0;
        *var_dqtevb2e1_dn1_slot = var_dqtevb2e1_dn1;
        *var_dqtevb2e1_dn10_slot = var_dqtevb2e1_dn10;
        *var_dqtevb2e1_dn3_slot = var_dqtevb2e1_dn3;
        *var_dqtevb2e1_dn4_slot = var_dqtevb2e1_dn4;
        *var_dqtevb2e1_dn5_slot = var_dqtevb2e1_dn5;
        *var_dqtevb2e1_dn6_slot = var_dqtevb2e1_dn6;
        *var_dqtevb2e1_dn7_slot = var_dqtevb2e1_dn7;
        *var_dqtevb2e1_dn8_slot = var_dqtevb2e1_dn8;
        *var_dqtevb2e1_dn9_slot = var_dqtevb2e1_dn9;
        *var_dqtevb2e1_rv_slot = var_dqtevb2e1_rv;
        *var_dvjevb2e1_slot = var_dvjevb2e1;
        *var_dvjevb2e1_dn0_slot = var_dvjevb2e1_dn0;
        *var_dvjevb2e1_dn1_slot = var_dvjevb2e1_dn1;
        *var_dvjevb2e1_dn10_slot = var_dvjevb2e1_dn10;
        *var_dvjevb2e1_dn3_slot = var_dvjevb2e1_dn3;
        *var_dvjevb2e1_dn4_slot = var_dvjevb2e1_dn4;
        *var_dvjevb2e1_dn5_slot = var_dvjevb2e1_dn5;
        *var_dvjevb2e1_dn6_slot = var_dvjevb2e1_dn6;
        *var_dvjevb2e1_dn7_slot = var_dvjevb2e1_dn7;
        *var_dvjevb2e1_dn8_slot = var_dvjevb2e1_dn8;
        *var_dvjevb2e1_dn9_slot = var_dvjevb2e1_dn9;
        *var_dvjevb2e1_rv_slot = var_dvjevb2e1_rv;
        *var_dvtevb2e1_slot = var_dvtevb2e1;
        *var_dvtevb2e1_dn0_slot = var_dvtevb2e1_dn0;
        *var_dvtevb2e1_dn1_slot = var_dvtevb2e1_dn1;
        *var_dvtevb2e1_dn10_slot = var_dvtevb2e1_dn10;
        *var_dvtevb2e1_dn3_slot = var_dvtevb2e1_dn3;
        *var_dvtevb2e1_dn4_slot = var_dvtevb2e1_dn4;
        *var_dvtevb2e1_dn5_slot = var_dvtevb2e1_dn5;
        *var_dvtevb2e1_dn6_slot = var_dvtevb2e1_dn6;
        *var_dvtevb2e1_dn7_slot = var_dvtevb2e1_dn7;
        *var_dvtevb2e1_dn8_slot = var_dvtevb2e1_dn8;
        *var_dvtevb2e1_dn9_slot = var_dvtevb2e1_dn9;
        *var_dvtevb2e1_rv_slot = var_dvtevb2e1_rv;
        *var_dvtevje_slot = var_dvtevje;
        *var_dvtevje_dn0_slot = var_dvtevje_dn0;
        *var_dvtevje_dn1_slot = var_dvtevje_dn1;
        *var_dvtevje_dn10_slot = var_dvtevje_dn10;
        *var_dvtevje_dn3_slot = var_dvtevje_dn3;
        *var_dvtevje_dn4_slot = var_dvtevje_dn4;
        *var_dvtevje_dn5_slot = var_dvtevje_dn5;
        *var_dvtevje_dn6_slot = var_dvtevje_dn6;
        *var_dvtevje_dn7_slot = var_dvtevje_dn7;
        *var_dvtevje_dn8_slot = var_dvtevje_dn8;
        *var_dvtevje_dn9_slot = var_dvtevje_dn9;
        *var_dvtevje_rv_slot = var_dvtevje_rv;
        *var_evbc3vdcex_slot = var_evbc3vdcex;
        *var_evbc3vdcex_dn0_slot = var_evbc3vdcex_dn0;
        *var_evbc3vdcex_dn1_slot = var_evbc3vdcex_dn1;
        *var_evbc3vdcex_dn10_slot = var_evbc3vdcex_dn10;
        *var_evbc3vdcex_dn3_slot = var_evbc3vdcex_dn3;
        *var_evbc3vdcex_dn4_slot = var_evbc3vdcex_dn4;
        *var_evbc3vdcex_dn5_slot = var_evbc3vdcex_dn5;
        *var_evbc3vdcex_dn6_slot = var_evbc3vdcex_dn6;
        *var_evbc3vdcex_dn7_slot = var_evbc3vdcex_dn7;
        *var_evbc3vdcex_dn8_slot = var_evbc3vdcex_dn8;
        *var_evbc3vdcex_dn9_slot = var_evbc3vdcex_dn9;
        *var_evbc3vdcex_rv_slot = var_evbc3vdcex_rv;
        *var_expl_slot = var_expl;
        *var_expl_rv_slot = var_expl_rv;
        *var_guard119_slot = var_guard119;
        *var_guard119_rv_slot = var_guard119_rv;
        *var_guard120_slot = var_guard120;
        *var_guard120_rv_slot = var_guard120_rv;
        *var_guard121_slot = var_guard121;
        *var_guard121_rv_slot = var_guard121_rv;
        *var_qb1b2_slot = var_qb1b2;
        *var_qb1b2_dn0_slot = var_qb1b2_dn0;
        *var_qb1b2_dn1_slot = var_qb1b2_dn1;
        *var_qb1b2_dn10_slot = var_qb1b2_dn10;
        *var_qb1b2_dn3_slot = var_qb1b2_dn3;
        *var_qb1b2_dn4_slot = var_qb1b2_dn4;
        *var_qb1b2_dn5_slot = var_qb1b2_dn5;
        *var_qb1b2_dn6_slot = var_qb1b2_dn6;
        *var_qb1b2_dn7_slot = var_qb1b2_dn7;
        *var_qb1b2_dn8_slot = var_qb1b2_dn8;
        *var_qb1b2_dn9_slot = var_qb1b2_dn9;
        *var_qb1b2_rv_slot = var_qb1b2_rv;
        *var_qbe_qs_eff_slot = var_qbe_qs_eff;
        *var_qbe_qs_eff_dn0_slot = var_qbe_qs_eff_dn0;
        *var_qbe_qs_eff_dn1_slot = var_qbe_qs_eff_dn1;
        *var_qbe_qs_eff_dn10_slot = var_qbe_qs_eff_dn10;
        *var_qbe_qs_eff_dn3_slot = var_qbe_qs_eff_dn3;
        *var_qbe_qs_eff_dn4_slot = var_qbe_qs_eff_dn4;
        *var_qbe_qs_eff_dn5_slot = var_qbe_qs_eff_dn5;
        *var_qbe_qs_eff_dn6_slot = var_qbe_qs_eff_dn6;
        *var_qbe_qs_eff_dn7_slot = var_qbe_qs_eff_dn7;
        *var_qbe_qs_eff_dn8_slot = var_qbe_qs_eff_dn8;
        *var_qbe_qs_eff_dn9_slot = var_qbe_qs_eff_dn9;
        *var_qbe_qs_eff_rv_slot = var_qbe_qs_eff_rv;
        *var_qe_slot = var_qe;
        *var_qe_dn0_slot = var_qe_dn0;
        *var_qe_dn1_slot = var_qe_dn1;
        *var_qe_dn10_slot = var_qe_dn10;
        *var_qe_dn3_slot = var_qe_dn3;
        *var_qe_dn4_slot = var_qe_dn4;
        *var_qe_dn5_slot = var_qe_dn5;
        *var_qe_dn6_slot = var_qe_dn6;
        *var_qe_dn7_slot = var_qe_dn7;
        *var_qe_dn8_slot = var_qe_dn8;
        *var_qe_dn9_slot = var_qe_dn9;
        *var_qe_rv_slot = var_qe_rv;
        *var_vb2e1vfe_slot = var_vb2e1vfe;
        *var_vb2e1vfe_dn0_slot = var_vb2e1vfe_dn0;
        *var_vb2e1vfe_dn1_slot = var_vb2e1vfe_dn1;
        *var_vb2e1vfe_dn10_slot = var_vb2e1vfe_dn10;
        *var_vb2e1vfe_dn3_slot = var_vb2e1vfe_dn3;
        *var_vb2e1vfe_dn4_slot = var_vb2e1vfe_dn4;
        *var_vb2e1vfe_dn5_slot = var_vb2e1vfe_dn5;
        *var_vb2e1vfe_dn6_slot = var_vb2e1vfe_dn6;
        *var_vb2e1vfe_dn7_slot = var_vb2e1vfe_dn7;
        *var_vb2e1vfe_dn8_slot = var_vb2e1vfe_dn8;
        *var_vb2e1vfe_dn9_slot = var_vb2e1vfe_dn9;
        *var_vb2e1vfe_rv_slot = var_vb2e1vfe_rv;
        *var_xg2_slot = var_xg2;
        *var_xg2_dn0_slot = var_xg2_dn0;
        *var_xg2_dn1_slot = var_xg2_dn1;
        *var_xg2_dn10_slot = var_xg2_dn10;
        *var_xg2_dn3_slot = var_xg2_dn3;
        *var_xg2_dn4_slot = var_xg2_dn4;
        *var_xg2_dn5_slot = var_xg2_dn5;
        *var_xg2_dn6_slot = var_xg2_dn6;
        *var_xg2_dn7_slot = var_xg2_dn7;
        *var_xg2_dn8_slot = var_xg2_dn8;
        *var_xg2_dn9_slot = var_xg2_dn9;
        *var_xg2_rv_slot = var_xg2_rv;
        *var_xnbex_slot = var_xnbex;
        *var_xnbex_dn0_slot = var_xnbex_dn0;
        *var_xnbex_dn1_slot = var_xnbex_dn1;
        *var_xnbex_dn10_slot = var_xnbex_dn10;
        *var_xnbex_dn3_slot = var_xnbex_dn3;
        *var_xnbex_dn4_slot = var_xnbex_dn4;
        *var_xnbex_dn5_slot = var_xnbex_dn5;
        *var_xnbex_dn6_slot = var_xnbex_dn6;
        *var_xnbex_dn7_slot = var_xnbex_dn7;
        *var_xnbex_dn8_slot = var_xnbex_dn8;
        *var_xnbex_dn9_slot = var_xnbex_dn9;
        *var_xnbex_rv_slot = var_xnbex_rv;
        *var_xpwex_slot = var_xpwex;
        *var_xpwex_dn0_slot = var_xpwex_dn0;
        *var_xpwex_dn1_slot = var_xpwex_dn1;
        *var_xpwex_dn10_slot = var_xpwex_dn10;
        *var_xpwex_dn3_slot = var_xpwex_dn3;
        *var_xpwex_dn4_slot = var_xpwex_dn4;
        *var_xpwex_dn5_slot = var_xpwex_dn5;
        *var_xpwex_dn6_slot = var_xpwex_dn6;
        *var_xpwex_dn7_slot = var_xpwex_dn7;
        *var_xpwex_dn8_slot = var_xpwex_dn8;
        *var_xpwex_dn9_slot = var_xpwex_dn9;
        *var_xpwex_rv_slot = var_xpwex_rv;
        *var_xqex_slot = var_xqex;
        *var_xqex_dn0_slot = var_xqex_dn0;
        *var_xqex_dn1_slot = var_xqex_dn1;
        *var_xqex_dn10_slot = var_xqex_dn10;
        *var_xqex_dn3_slot = var_xqex_dn3;
        *var_xqex_dn4_slot = var_xqex_dn4;
        *var_xqex_dn5_slot = var_xqex_dn5;
        *var_xqex_dn6_slot = var_xqex_dn6;
        *var_xqex_dn7_slot = var_xqex_dn7;
        *var_xqex_dn8_slot = var_xqex_dn8;
        *var_xqex_dn9_slot = var_xqex_dn9;
        *var_xqex_rv_slot = var_xqex_rv;
        *var_xqmex_slot = var_xqmex;
        *var_xqmex_dn0_slot = var_xqmex_dn0;
        *var_xqmex_dn1_slot = var_xqmex_dn1;
        *var_xqmex_dn10_slot = var_xqmex_dn10;
        *var_xqmex_dn3_slot = var_xqmex_dn3;
        *var_xqmex_dn4_slot = var_xqmex_dn4;
        *var_xqmex_dn5_slot = var_xqmex_dn5;
        *var_xqmex_dn6_slot = var_xqmex_dn6;
        *var_xqmex_dn7_slot = var_xqmex_dn7;
        *var_xqmex_dn8_slot = var_xqmex_dn8;
        *var_xqmex_dn9_slot = var_xqmex_dn9;
        *var_xqmex_rv_slot = var_xqmex_rv;
    }

    pub(super) fn stamp_reactive_block_17(
        p: &Parameters,
        var_guard120: f64,
        var_if_: f64,
        var_if__dn0: f64,
        var_if__dn1: f64,
        var_if__dn10: f64,
        var_if__dn3: f64,
        var_if__dn4: f64,
        var_if__dn5: f64,
        var_if__dn6: f64,
        var_if__dn7: f64,
        var_if__dn8: f64,
        var_if__dn9: f64,
        var_ir: f64,
        var_ir_dn0: f64,
        var_ir_dn1: f64,
        var_ir_dn10: f64,
        var_ir_dn3: f64,
        var_ir_dn4: f64,
        var_ir_dn5: f64,
        var_ir_dn6: f64,
        var_ir_dn7: f64,
        var_ir_dn8: f64,
        var_ir_dn9: f64,
        var_q1q: f64,
        var_q1q_dn0: f64,
        var_q1q_dn1: f64,
        var_q1q_dn10: f64,
        var_q1q_dn3: f64,
        var_q1q_dn4: f64,
        var_q1q_dn5: f64,
        var_q1q_dn6: f64,
        var_q1q_dn7: f64,
        var_q1q_dn8: f64,
        var_q1q_dn9: f64,
        var_qbc_qs: f64,
        var_qbc_qs_dn0: f64,
        var_qbc_qs_dn1: f64,
        var_qbc_qs_dn10: f64,
        var_qbc_qs_dn3: f64,
        var_qbc_qs_dn4: f64,
        var_qbc_qs_dn5: f64,
        var_qbc_qs_dn6: f64,
        var_qbc_qs_dn7: f64,
        var_qbc_qs_dn8: f64,
        var_qbc_qs_dn9: f64,
        var_qbe_qs: f64,
        var_qbe_qs_dn0: f64,
        var_qbe_qs_dn1: f64,
        var_qbe_qs_dn10: f64,
        var_qbe_qs_dn3: f64,
        var_qbe_qs_dn4: f64,
        var_qbe_qs_dn5: f64,
        var_qbe_qs_dn6: f64,
        var_qbe_qs_dn7: f64,
        var_qbe_qs_dn8: f64,
        var_qbe_qs_dn9: f64,
        var_qbe_qs_eff: f64,
        var_qbe_qs_eff_dn0: f64,
        var_qbe_qs_eff_dn1: f64,
        var_qbe_qs_eff_dn10: f64,
        var_qbe_qs_eff_dn3: f64,
        var_qbe_qs_eff_dn4: f64,
        var_qbe_qs_eff_dn5: f64,
        var_qbe_qs_eff_dn6: f64,
        var_qbe_qs_eff_dn7: f64,
        var_qbe_qs_eff_dn8: f64,
        var_qbe_qs_eff_dn9: f64,
        var_qbi: f64,
        var_qbi_dn0: f64,
        var_qbi_dn1: f64,
        var_qbi_dn10: f64,
        var_qbi_dn3: f64,
        var_qbi_dn4: f64,
        var_qbi_dn5: f64,
        var_qbi_dn6: f64,
        var_qbi_dn7: f64,
        var_qbi_dn8: f64,
        var_qbi_dn9: f64,
        var_qe_qs: f64,
        var_qe_qs_dn0: f64,
        var_qe_qs_dn1: f64,
        var_qe_qs_dn10: f64,
        var_qe_qs_dn3: f64,
        var_qe_qs_dn4: f64,
        var_qe_qs_dn5: f64,
        var_qe_qs_dn6: f64,
        var_qe_qs_dn7: f64,
        var_qe_qs_dn8: f64,
        var_qe_qs_dn9: f64,
        var_taub_t: f64,
        var_guard126_slot: &mut f64,
        var_guard126_rv_slot: &mut f64,
        var_guard127_slot: &mut f64,
        var_guard127_rv_slot: &mut f64,
        var_guard128_slot: &mut f64,
        var_guard128_rv_slot: &mut f64,
        var_in_n_slot: &mut f64,
        var_in_n_dn0_slot: &mut f64,
        var_in_n_dn1_slot: &mut f64,
        var_in_n_dn10_slot: &mut f64,
        var_in_n_dn3_slot: &mut f64,
        var_in_n_dn4_slot: &mut f64,
        var_in_n_dn5_slot: &mut f64,
        var_in_n_dn6_slot: &mut f64,
        var_in_n_dn7_slot: &mut f64,
        var_in_n_dn8_slot: &mut f64,
        var_in_n_dn9_slot: &mut f64,
        var_in_n_rv_slot: &mut f64,
        var_qbc_slot: &mut f64,
        var_qbc_dn0_slot: &mut f64,
        var_qbc_dn1_slot: &mut f64,
        var_qbc_dn10_slot: &mut f64,
        var_qbc_dn3_slot: &mut f64,
        var_qbc_dn4_slot: &mut f64,
        var_qbc_dn5_slot: &mut f64,
        var_qbc_dn6_slot: &mut f64,
        var_qbc_dn7_slot: &mut f64,
        var_qbc_dn8_slot: &mut f64,
        var_qbc_dn9_slot: &mut f64,
        var_qbc_rv_slot: &mut f64,
        var_qbe_slot: &mut f64,
        var_qbe_dn0_slot: &mut f64,
        var_qbe_dn1_slot: &mut f64,
        var_qbe_dn10_slot: &mut f64,
        var_qbe_dn3_slot: &mut f64,
        var_qbe_dn4_slot: &mut f64,
        var_qbe_dn5_slot: &mut f64,
        var_qbe_dn6_slot: &mut f64,
        var_qbe_dn7_slot: &mut f64,
        var_qbe_dn8_slot: &mut f64,
        var_qbe_dn9_slot: &mut f64,
        var_qbe_rv_slot: &mut f64,
        var_qe_slot: &mut f64,
        var_qe_dn0_slot: &mut f64,
        var_qe_dn1_slot: &mut f64,
        var_qe_dn10_slot: &mut f64,
        var_qe_dn3_slot: &mut f64,
        var_qe_dn4_slot: &mut f64,
        var_qe_dn5_slot: &mut f64,
        var_qe_dn6_slot: &mut f64,
        var_qe_dn7_slot: &mut f64,
        var_qe_dn8_slot: &mut f64,
        var_qe_dn9_slot: &mut f64,
        var_qe_rv_slot: &mut f64,
        var_taub_n_slot: &mut f64,
        var_taub_n_dn0_slot: &mut f64,
        var_taub_n_dn1_slot: &mut f64,
        var_taub_n_dn10_slot: &mut f64,
        var_taub_n_dn3_slot: &mut f64,
        var_taub_n_dn4_slot: &mut f64,
        var_taub_n_dn5_slot: &mut f64,
        var_taub_n_dn6_slot: &mut f64,
        var_taub_n_dn7_slot: &mut f64,
        var_taub_n_dn8_slot: &mut f64,
        var_taub_n_dn9_slot: &mut f64,
        var_taub_n_rv_slot: &mut f64,
        var_taun_slot: &mut f64,
        var_taun_dn0_slot: &mut f64,
        var_taun_dn1_slot: &mut f64,
        var_taun_dn10_slot: &mut f64,
        var_taun_dn3_slot: &mut f64,
        var_taun_dn4_slot: &mut f64,
        var_taun_dn5_slot: &mut f64,
        var_taun_dn6_slot: &mut f64,
        var_taun_dn7_slot: &mut f64,
        var_taun_dn8_slot: &mut f64,
        var_taun_dn9_slot: &mut f64,
        var_taun_rv_slot: &mut f64,
    ) {
        let mut var_guard126: f64 = *var_guard126_slot;
        let mut var_guard126_rv: f64 = *var_guard126_rv_slot;
        let mut var_guard127: f64 = *var_guard127_slot;
        let mut var_guard127_rv: f64 = *var_guard127_rv_slot;
        let mut var_guard128: f64 = *var_guard128_slot;
        let mut var_guard128_rv: f64 = *var_guard128_rv_slot;
        let mut var_in_n: f64 = *var_in_n_slot;
        let mut var_in_n_dn0: f64 = *var_in_n_dn0_slot;
        let mut var_in_n_dn1: f64 = *var_in_n_dn1_slot;
        let mut var_in_n_dn10: f64 = *var_in_n_dn10_slot;
        let mut var_in_n_dn3: f64 = *var_in_n_dn3_slot;
        let mut var_in_n_dn4: f64 = *var_in_n_dn4_slot;
        let mut var_in_n_dn5: f64 = *var_in_n_dn5_slot;
        let mut var_in_n_dn6: f64 = *var_in_n_dn6_slot;
        let mut var_in_n_dn7: f64 = *var_in_n_dn7_slot;
        let mut var_in_n_dn8: f64 = *var_in_n_dn8_slot;
        let mut var_in_n_dn9: f64 = *var_in_n_dn9_slot;
        let mut var_in_n_rv: f64 = *var_in_n_rv_slot;
        let mut var_qbc: f64 = *var_qbc_slot;
        let mut var_qbc_dn0: f64 = *var_qbc_dn0_slot;
        let mut var_qbc_dn1: f64 = *var_qbc_dn1_slot;
        let mut var_qbc_dn10: f64 = *var_qbc_dn10_slot;
        let mut var_qbc_dn3: f64 = *var_qbc_dn3_slot;
        let mut var_qbc_dn4: f64 = *var_qbc_dn4_slot;
        let mut var_qbc_dn5: f64 = *var_qbc_dn5_slot;
        let mut var_qbc_dn6: f64 = *var_qbc_dn6_slot;
        let mut var_qbc_dn7: f64 = *var_qbc_dn7_slot;
        let mut var_qbc_dn8: f64 = *var_qbc_dn8_slot;
        let mut var_qbc_dn9: f64 = *var_qbc_dn9_slot;
        let mut var_qbc_rv: f64 = *var_qbc_rv_slot;
        let mut var_qbe: f64 = *var_qbe_slot;
        let mut var_qbe_dn0: f64 = *var_qbe_dn0_slot;
        let mut var_qbe_dn1: f64 = *var_qbe_dn1_slot;
        let mut var_qbe_dn10: f64 = *var_qbe_dn10_slot;
        let mut var_qbe_dn3: f64 = *var_qbe_dn3_slot;
        let mut var_qbe_dn4: f64 = *var_qbe_dn4_slot;
        let mut var_qbe_dn5: f64 = *var_qbe_dn5_slot;
        let mut var_qbe_dn6: f64 = *var_qbe_dn6_slot;
        let mut var_qbe_dn7: f64 = *var_qbe_dn7_slot;
        let mut var_qbe_dn8: f64 = *var_qbe_dn8_slot;
        let mut var_qbe_dn9: f64 = *var_qbe_dn9_slot;
        let mut var_qbe_rv: f64 = *var_qbe_rv_slot;
        let mut var_qe: f64 = *var_qe_slot;
        let mut var_qe_dn0: f64 = *var_qe_dn0_slot;
        let mut var_qe_dn1: f64 = *var_qe_dn1_slot;
        let mut var_qe_dn10: f64 = *var_qe_dn10_slot;
        let mut var_qe_dn3: f64 = *var_qe_dn3_slot;
        let mut var_qe_dn4: f64 = *var_qe_dn4_slot;
        let mut var_qe_dn5: f64 = *var_qe_dn5_slot;
        let mut var_qe_dn6: f64 = *var_qe_dn6_slot;
        let mut var_qe_dn7: f64 = *var_qe_dn7_slot;
        let mut var_qe_dn8: f64 = *var_qe_dn8_slot;
        let mut var_qe_dn9: f64 = *var_qe_dn9_slot;
        let mut var_qe_rv: f64 = *var_qe_rv_slot;
        let mut var_taub_n: f64 = *var_taub_n_slot;
        let mut var_taub_n_dn0: f64 = *var_taub_n_dn0_slot;
        let mut var_taub_n_dn1: f64 = *var_taub_n_dn1_slot;
        let mut var_taub_n_dn10: f64 = *var_taub_n_dn10_slot;
        let mut var_taub_n_dn3: f64 = *var_taub_n_dn3_slot;
        let mut var_taub_n_dn4: f64 = *var_taub_n_dn4_slot;
        let mut var_taub_n_dn5: f64 = *var_taub_n_dn5_slot;
        let mut var_taub_n_dn6: f64 = *var_taub_n_dn6_slot;
        let mut var_taub_n_dn7: f64 = *var_taub_n_dn7_slot;
        let mut var_taub_n_dn8: f64 = *var_taub_n_dn8_slot;
        let mut var_taub_n_dn9: f64 = *var_taub_n_dn9_slot;
        let mut var_taub_n_rv: f64 = *var_taub_n_rv_slot;
        let mut var_taun: f64 = *var_taun_slot;
        let mut var_taun_dn0: f64 = *var_taun_dn0_slot;
        let mut var_taun_dn1: f64 = *var_taun_dn1_slot;
        let mut var_taun_dn10: f64 = *var_taun_dn10_slot;
        let mut var_taun_dn3: f64 = *var_taun_dn3_slot;
        let mut var_taun_dn4: f64 = *var_taun_dn4_slot;
        let mut var_taun_dn5: f64 = *var_taun_dn5_slot;
        let mut var_taun_dn6: f64 = *var_taun_dn6_slot;
        let mut var_taun_dn7: f64 = *var_taun_dn7_slot;
        let mut var_taun_dn8: f64 = *var_taun_dn8_slot;
        let mut var_taun_dn9: f64 = *var_taun_dn9_slot;
        let mut var_taun_rv: f64 = *var_taun_rv_slot;

        let (assign6750_e6971, assign6750_e6971_d_n0, assign6750_e6971_d_n1, assign6750_e6971_d_n3, assign6750_e6971_d_n4, assign6750_e6971_d_n5, assign6750_e6971_d_n6, assign6750_e6971_d_n7, assign6750_e6971_d_n8, assign6750_e6971_d_n9, assign6750_e6971_d_n10,) = {
    if (var_guard120 != 0.0) {
        let assign6750_e6967: f64 = (p.p94 * var_qbe_qs_eff);
        let assign6750_e6969: f64 = (assign6750_e6967 + var_qbc_qs);
        (assign6750_e6969, ((p.p94 * var_qbe_qs_eff_dn0) + var_qbc_qs_dn0), ((p.p94 * var_qbe_qs_eff_dn1) + var_qbc_qs_dn1), ((p.p94 * var_qbe_qs_eff_dn3) + var_qbc_qs_dn3), ((p.p94 * var_qbe_qs_eff_dn4) + var_qbc_qs_dn4), ((p.p94 * var_qbe_qs_eff_dn5) + var_qbc_qs_dn5), ((p.p94 * var_qbe_qs_eff_dn6) + var_qbc_qs_dn6), ((p.p94 * var_qbe_qs_eff_dn7) + var_qbc_qs_dn7), ((p.p94 * var_qbe_qs_eff_dn8) + var_qbc_qs_dn8), ((p.p94 * var_qbe_qs_eff_dn9) + var_qbc_qs_dn9), ((p.p94 * var_qbe_qs_eff_dn10) + var_qbc_qs_dn10),)
    } else {
        (var_qbc, var_qbc_dn0, var_qbc_dn1, var_qbc_dn3, var_qbc_dn4, var_qbc_dn5, var_qbc_dn6, var_qbc_dn7, var_qbc_dn8, var_qbc_dn9, var_qbc_dn10,)
    }
};
        var_qbc = assign6750_e6971;
        var_qbc_dn0 = assign6750_e6971_d_n0;
        var_qbc_dn1 = assign6750_e6971_d_n1;
        var_qbc_dn3 = assign6750_e6971_d_n3;
        var_qbc_dn4 = assign6750_e6971_d_n4;
        var_qbc_dn5 = assign6750_e6971_d_n5;
        var_qbc_dn6 = assign6750_e6971_d_n6;
        var_qbc_dn7 = assign6750_e6971_d_n7;
        var_qbc_dn8 = assign6750_e6971_d_n8;
        var_qbc_dn9 = assign6750_e6971_d_n9;
        var_qbc_dn10 = assign6750_e6971_d_n10;
        var_qbc_rv = 0.0;

        let (assign6760_e6979, assign6760_e6979_d_n0, assign6760_e6979_d_n1, assign6760_e6979_d_n3, assign6760_e6979_d_n4, assign6760_e6979_d_n5, assign6760_e6979_d_n6, assign6760_e6979_d_n7, assign6760_e6979_d_n8, assign6760_e6979_d_n9, assign6760_e6979_d_n10,) = {
    if (var_guard120 != 0.0) {
        let assign6760_e6975: f64 = (1.0 - p.p94);
        let assign6760_e6977: f64 = (assign6760_e6975 * var_qbe_qs_eff);
        (assign6760_e6977, (assign6760_e6975 * var_qbe_qs_eff_dn0), (assign6760_e6975 * var_qbe_qs_eff_dn1), (assign6760_e6975 * var_qbe_qs_eff_dn3), (assign6760_e6975 * var_qbe_qs_eff_dn4), (assign6760_e6975 * var_qbe_qs_eff_dn5), (assign6760_e6975 * var_qbe_qs_eff_dn6), (assign6760_e6975 * var_qbe_qs_eff_dn7), (assign6760_e6975 * var_qbe_qs_eff_dn8), (assign6760_e6975 * var_qbe_qs_eff_dn9), (assign6760_e6975 * var_qbe_qs_eff_dn10),)
    } else {
        (var_qbe, var_qbe_dn0, var_qbe_dn1, var_qbe_dn3, var_qbe_dn4, var_qbe_dn5, var_qbe_dn6, var_qbe_dn7, var_qbe_dn8, var_qbe_dn9, var_qbe_dn10,)
    }
};
        var_qbe = assign6760_e6979;
        var_qbe_dn0 = assign6760_e6979_d_n0;
        var_qbe_dn1 = assign6760_e6979_d_n1;
        var_qbe_dn3 = assign6760_e6979_d_n3;
        var_qbe_dn4 = assign6760_e6979_d_n4;
        var_qbe_dn5 = assign6760_e6979_d_n5;
        var_qbe_dn6 = assign6760_e6979_d_n6;
        var_qbe_dn7 = assign6760_e6979_d_n7;
        var_qbe_dn8 = assign6760_e6979_d_n8;
        var_qbe_dn9 = assign6760_e6979_d_n9;
        var_qbe_dn10 = assign6760_e6979_d_n10;
        var_qbe_rv = 0.0;

        let (assign6770_e6984, assign6770_e6984_d_n0, assign6770_e6984_d_n1, assign6770_e6984_d_n3, assign6770_e6984_d_n4, assign6770_e6984_d_n5, assign6770_e6984_d_n6, assign6770_e6984_d_n7, assign6770_e6984_d_n8, assign6770_e6984_d_n9, assign6770_e6984_d_n10,) = {
    if (var_guard120 == 0.0) {
        (var_qbe_qs, var_qbe_qs_dn0, var_qbe_qs_dn1, var_qbe_qs_dn3, var_qbe_qs_dn4, var_qbe_qs_dn5, var_qbe_qs_dn6, var_qbe_qs_dn7, var_qbe_qs_dn8, var_qbe_qs_dn9, var_qbe_qs_dn10,)
    } else {
        (var_qbe, var_qbe_dn0, var_qbe_dn1, var_qbe_dn3, var_qbe_dn4, var_qbe_dn5, var_qbe_dn6, var_qbe_dn7, var_qbe_dn8, var_qbe_dn9, var_qbe_dn10,)
    }
};
        var_qbe = assign6770_e6984;
        var_qbe_dn0 = assign6770_e6984_d_n0;
        var_qbe_dn1 = assign6770_e6984_d_n1;
        var_qbe_dn3 = assign6770_e6984_d_n3;
        var_qbe_dn4 = assign6770_e6984_d_n4;
        var_qbe_dn5 = assign6770_e6984_d_n5;
        var_qbe_dn6 = assign6770_e6984_d_n6;
        var_qbe_dn7 = assign6770_e6984_d_n7;
        var_qbe_dn8 = assign6770_e6984_d_n8;
        var_qbe_dn9 = assign6770_e6984_d_n9;
        var_qbe_dn10 = assign6770_e6984_d_n10;
        var_qbe_rv = 0.0;

        let (assign6780_e6989, assign6780_e6989_d_n0, assign6780_e6989_d_n1, assign6780_e6989_d_n3, assign6780_e6989_d_n4, assign6780_e6989_d_n5, assign6780_e6989_d_n6, assign6780_e6989_d_n7, assign6780_e6989_d_n8, assign6780_e6989_d_n9, assign6780_e6989_d_n10,) = {
    if (var_guard120 == 0.0) {
        (var_qbc_qs, var_qbc_qs_dn0, var_qbc_qs_dn1, var_qbc_qs_dn3, var_qbc_qs_dn4, var_qbc_qs_dn5, var_qbc_qs_dn6, var_qbc_qs_dn7, var_qbc_qs_dn8, var_qbc_qs_dn9, var_qbc_qs_dn10,)
    } else {
        (var_qbc, var_qbc_dn0, var_qbc_dn1, var_qbc_dn3, var_qbc_dn4, var_qbc_dn5, var_qbc_dn6, var_qbc_dn7, var_qbc_dn8, var_qbc_dn9, var_qbc_dn10,)
    }
};
        var_qbc = assign6780_e6989;
        var_qbc_dn0 = assign6780_e6989_d_n0;
        var_qbc_dn1 = assign6780_e6989_d_n1;
        var_qbc_dn3 = assign6780_e6989_d_n3;
        var_qbc_dn4 = assign6780_e6989_d_n4;
        var_qbc_dn5 = assign6780_e6989_d_n5;
        var_qbc_dn6 = assign6780_e6989_d_n6;
        var_qbc_dn7 = assign6780_e6989_d_n7;
        var_qbc_dn8 = assign6780_e6989_d_n8;
        var_qbc_dn9 = assign6780_e6989_d_n9;
        var_qbc_dn10 = assign6780_e6989_d_n10;
        var_qbc_rv = 0.0;

        let (assign6790_e6994, assign6790_e6994_d_n0, assign6790_e6994_d_n1, assign6790_e6994_d_n3, assign6790_e6994_d_n4, assign6790_e6994_d_n5, assign6790_e6994_d_n6, assign6790_e6994_d_n7, assign6790_e6994_d_n8, assign6790_e6994_d_n9, assign6790_e6994_d_n10,) = {
    if (var_guard120 == 0.0) {
        (var_qe_qs, var_qe_qs_dn0, var_qe_qs_dn1, var_qe_qs_dn3, var_qe_qs_dn4, var_qe_qs_dn5, var_qe_qs_dn6, var_qe_qs_dn7, var_qe_qs_dn8, var_qe_qs_dn9, var_qe_qs_dn10,)
    } else {
        (var_qe, var_qe_dn0, var_qe_dn1, var_qe_dn3, var_qe_dn4, var_qe_dn5, var_qe_dn6, var_qe_dn7, var_qe_dn8, var_qe_dn9, var_qe_dn10,)
    }
};
        var_qe = assign6790_e6994;
        var_qe_dn0 = assign6790_e6994_d_n0;
        var_qe_dn1 = assign6790_e6994_d_n1;
        var_qe_dn3 = assign6790_e6994_d_n3;
        var_qe_dn4 = assign6790_e6994_d_n4;
        var_qe_dn5 = assign6790_e6994_d_n5;
        var_qe_dn6 = assign6790_e6994_d_n6;
        var_qe_dn7 = assign6790_e6994_d_n7;
        var_qe_dn8 = assign6790_e6994_d_n8;
        var_qe_dn9 = assign6790_e6994_d_n9;
        var_qe_dn10 = assign6790_e6994_d_n10;
        var_qe_rv = 0.0;

        let assign6900_e7037: f64 = (var_if_ + var_ir);
        let assign6900_e7039: f64 = (assign6900_e7037 / var_qbi);
        var_in_n = assign6900_e7039;
        var_in_n_dn0 = ((((var_if__dn0 + var_ir_dn0) * var_qbi) - (assign6900_e7037 * var_qbi_dn0)) / (var_qbi * var_qbi));
        var_in_n_dn1 = ((((var_if__dn1 + var_ir_dn1) * var_qbi) - (assign6900_e7037 * var_qbi_dn1)) / (var_qbi * var_qbi));
        var_in_n_dn3 = ((((var_if__dn3 + var_ir_dn3) * var_qbi) - (assign6900_e7037 * var_qbi_dn3)) / (var_qbi * var_qbi));
        var_in_n_dn4 = ((((var_if__dn4 + var_ir_dn4) * var_qbi) - (assign6900_e7037 * var_qbi_dn4)) / (var_qbi * var_qbi));
        var_in_n_dn5 = ((((var_if__dn5 + var_ir_dn5) * var_qbi) - (assign6900_e7037 * var_qbi_dn5)) / (var_qbi * var_qbi));
        var_in_n_dn6 = ((((var_if__dn6 + var_ir_dn6) * var_qbi) - (assign6900_e7037 * var_qbi_dn6)) / (var_qbi * var_qbi));
        var_in_n_dn7 = ((((var_if__dn7 + var_ir_dn7) * var_qbi) - (assign6900_e7037 * var_qbi_dn7)) / (var_qbi * var_qbi));
        var_in_n_dn8 = ((((var_if__dn8 + var_ir_dn8) * var_qbi) - (assign6900_e7037 * var_qbi_dn8)) / (var_qbi * var_qbi));
        var_in_n_dn9 = ((((var_if__dn9 + var_ir_dn9) * var_qbi) - (assign6900_e7037 * var_qbi_dn9)) / (var_qbi * var_qbi));
        var_in_n_dn10 = ((((var_if__dn10 + var_ir_dn10) * var_qbi) - (assign6900_e7037 * var_qbi_dn10)) / (var_qbi * var_qbi));
        var_in_n_rv = 0.0;

        let assign6960_e7072: f64 = if var_in_n > 0.0 { 1.0 } else { 0.0 };
        var_guard126 = assign6960_e7072;
        var_guard126_rv = 0.0;

        let (assign6970_e7080, assign6970_e7080_d_n0, assign6970_e7080_d_n1, assign6970_e7080_d_n3, assign6970_e7080_d_n4, assign6970_e7080_d_n5, assign6970_e7080_d_n6, assign6970_e7080_d_n7, assign6970_e7080_d_n8, assign6970_e7080_d_n9, assign6970_e7080_d_n10,) = {
    if (var_guard126 != 0.0) {
        let assign6970_e7076: f64 = (var_qbe + var_qbc);
        let assign6970_e7078: f64 = (assign6970_e7076 / var_in_n);
        (assign6970_e7078, ((((var_qbe_dn0 + var_qbc_dn0) * var_in_n) - (assign6970_e7076 * var_in_n_dn0)) / (var_in_n * var_in_n)), ((((var_qbe_dn1 + var_qbc_dn1) * var_in_n) - (assign6970_e7076 * var_in_n_dn1)) / (var_in_n * var_in_n)), ((((var_qbe_dn3 + var_qbc_dn3) * var_in_n) - (assign6970_e7076 * var_in_n_dn3)) / (var_in_n * var_in_n)), ((((var_qbe_dn4 + var_qbc_dn4) * var_in_n) - (assign6970_e7076 * var_in_n_dn4)) / (var_in_n * var_in_n)), ((((var_qbe_dn5 + var_qbc_dn5) * var_in_n) - (assign6970_e7076 * var_in_n_dn5)) / (var_in_n * var_in_n)), ((((var_qbe_dn6 + var_qbc_dn6) * var_in_n) - (assign6970_e7076 * var_in_n_dn6)) / (var_in_n * var_in_n)), ((((var_qbe_dn7 + var_qbc_dn7) * var_in_n) - (assign6970_e7076 * var_in_n_dn7)) / (var_in_n * var_in_n)), ((((var_qbe_dn8 + var_qbc_dn8) * var_in_n) - (assign6970_e7076 * var_in_n_dn8)) / (var_in_n * var_in_n)), ((((var_qbe_dn9 + var_qbc_dn9) * var_in_n) - (assign6970_e7076 * var_in_n_dn9)) / (var_in_n * var_in_n)), ((((var_qbe_dn10 + var_qbc_dn10) * var_in_n) - (assign6970_e7076 * var_in_n_dn10)) / (var_in_n * var_in_n)),)
    } else {
        (var_taub_n, var_taub_n_dn0, var_taub_n_dn1, var_taub_n_dn3, var_taub_n_dn4, var_taub_n_dn5, var_taub_n_dn6, var_taub_n_dn7, var_taub_n_dn8, var_taub_n_dn9, var_taub_n_dn10,)
    }
};
        var_taub_n = assign6970_e7080;
        var_taub_n_dn0 = assign6970_e7080_d_n0;
        var_taub_n_dn1 = assign6970_e7080_d_n1;
        var_taub_n_dn3 = assign6970_e7080_d_n3;
        var_taub_n_dn4 = assign6970_e7080_d_n4;
        var_taub_n_dn5 = assign6970_e7080_d_n5;
        var_taub_n_dn6 = assign6970_e7080_d_n6;
        var_taub_n_dn7 = assign6970_e7080_d_n7;
        var_taub_n_dn8 = assign6970_e7080_d_n8;
        var_taub_n_dn9 = assign6970_e7080_d_n9;
        var_taub_n_dn10 = assign6970_e7080_d_n10;
        var_taub_n_rv = 0.0;

        let (assign6980_e7089, assign6980_e7089_d_n0, assign6980_e7089_d_n1, assign6980_e7089_d_n3, assign6980_e7089_d_n4, assign6980_e7089_d_n5, assign6980_e7089_d_n6, assign6980_e7089_d_n7, assign6980_e7089_d_n8, assign6980_e7089_d_n9, assign6980_e7089_d_n10,) = {
    if (var_guard126 == 0.0) {
        let assign6980_e7085: f64 = (var_taub_t * var_q1q);
        let assign6980_e7087: f64 = (assign6980_e7085 * var_qbi);
        (assign6980_e7087, (((var_taub_t * var_q1q_dn0) * var_qbi) + (assign6980_e7085 * var_qbi_dn0)), (((var_taub_t * var_q1q_dn1) * var_qbi) + (assign6980_e7085 * var_qbi_dn1)), (((var_taub_t * var_q1q_dn3) * var_qbi) + (assign6980_e7085 * var_qbi_dn3)), (((var_taub_t * var_q1q_dn4) * var_qbi) + (assign6980_e7085 * var_qbi_dn4)), (((var_taub_t * var_q1q_dn5) * var_qbi) + (assign6980_e7085 * var_qbi_dn5)), (((var_taub_t * var_q1q_dn6) * var_qbi) + (assign6980_e7085 * var_qbi_dn6)), (((var_taub_t * var_q1q_dn7) * var_qbi) + (assign6980_e7085 * var_qbi_dn7)), (((var_taub_t * var_q1q_dn8) * var_qbi) + (assign6980_e7085 * var_qbi_dn8)), (((var_taub_t * var_q1q_dn9) * var_qbi) + (assign6980_e7085 * var_qbi_dn9)), (((var_taub_t * var_q1q_dn10) * var_qbi) + (assign6980_e7085 * var_qbi_dn10)),)
    } else {
        (var_taub_n, var_taub_n_dn0, var_taub_n_dn1, var_taub_n_dn3, var_taub_n_dn4, var_taub_n_dn5, var_taub_n_dn6, var_taub_n_dn7, var_taub_n_dn8, var_taub_n_dn9, var_taub_n_dn10,)
    }
};
        var_taub_n = assign6980_e7089;
        var_taub_n_dn0 = assign6980_e7089_d_n0;
        var_taub_n_dn1 = assign6980_e7089_d_n1;
        var_taub_n_dn3 = assign6980_e7089_d_n3;
        var_taub_n_dn4 = assign6980_e7089_d_n4;
        var_taub_n_dn5 = assign6980_e7089_d_n5;
        var_taub_n_dn6 = assign6980_e7089_d_n6;
        var_taub_n_dn7 = assign6980_e7089_d_n7;
        var_taub_n_dn8 = assign6980_e7089_d_n8;
        var_taub_n_dn9 = assign6980_e7089_d_n9;
        var_taub_n_dn10 = assign6980_e7089_d_n10;
        var_taub_n_rv = 0.0;

        let assign6990_e7092: f64 = if p.p131 == 1.0 { 1.0 } else { 0.0 };
        var_guard127 = assign6990_e7092;
        var_guard127_rv = 0.0;

        let (assign7000_e7098, assign7000_e7098_d_n0, assign7000_e7098_d_n1, assign7000_e7098_d_n3, assign7000_e7098_d_n4, assign7000_e7098_d_n5, assign7000_e7098_d_n6, assign7000_e7098_d_n7, assign7000_e7098_d_n8, assign7000_e7098_d_n9, assign7000_e7098_d_n10,) = {
    if (var_guard127 != 0.0) {
        let assign7000_e7096: f64 = (p.p94 * var_taub_n);
        (assign7000_e7096, (p.p94 * var_taub_n_dn0), (p.p94 * var_taub_n_dn1), (p.p94 * var_taub_n_dn3), (p.p94 * var_taub_n_dn4), (p.p94 * var_taub_n_dn5), (p.p94 * var_taub_n_dn6), (p.p94 * var_taub_n_dn7), (p.p94 * var_taub_n_dn8), (p.p94 * var_taub_n_dn9), (p.p94 * var_taub_n_dn10),)
    } else {
        (var_taun, var_taun_dn0, var_taun_dn1, var_taun_dn3, var_taun_dn4, var_taun_dn5, var_taun_dn6, var_taun_dn7, var_taun_dn8, var_taun_dn9, var_taun_dn10,)
    }
};
        var_taun = assign7000_e7098;
        var_taun_dn0 = assign7000_e7098_d_n0;
        var_taun_dn1 = assign7000_e7098_d_n1;
        var_taun_dn3 = assign7000_e7098_d_n3;
        var_taun_dn4 = assign7000_e7098_d_n4;
        var_taun_dn5 = assign7000_e7098_d_n5;
        var_taun_dn6 = assign7000_e7098_d_n6;
        var_taun_dn7 = assign7000_e7098_d_n7;
        var_taun_dn8 = assign7000_e7098_d_n8;
        var_taun_dn9 = assign7000_e7098_d_n9;
        var_taun_dn10 = assign7000_e7098_d_n10;
        var_taun_rv = 0.0;

        let assign7010_e7101: f64 = if p.p131 == 2.0 { 1.0 } else { 0.0 };
        var_guard128 = assign7010_e7101;
        var_guard128_rv = 0.0;

        let (assign7020_e7110, assign7020_e7110_d_n0, assign7020_e7110_d_n1, assign7020_e7110_d_n3, assign7020_e7110_d_n4, assign7020_e7110_d_n5, assign7020_e7110_d_n6, assign7020_e7110_d_n7, assign7020_e7110_d_n8, assign7020_e7110_d_n9, assign7020_e7110_d_n10,) = {
    if ((var_guard127 == 0.0) && (var_guard128 != 0.0)) {
        let assign7020_e7108: f64 = (p.p132 * var_taub_n);
        (assign7020_e7108, (p.p132 * var_taub_n_dn0), (p.p132 * var_taub_n_dn1), (p.p132 * var_taub_n_dn3), (p.p132 * var_taub_n_dn4), (p.p132 * var_taub_n_dn5), (p.p132 * var_taub_n_dn6), (p.p132 * var_taub_n_dn7), (p.p132 * var_taub_n_dn8), (p.p132 * var_taub_n_dn9), (p.p132 * var_taub_n_dn10),)
    } else {
        (var_taun, var_taun_dn0, var_taun_dn1, var_taun_dn3, var_taun_dn4, var_taun_dn5, var_taun_dn6, var_taun_dn7, var_taun_dn8, var_taun_dn9, var_taun_dn10,)
    }
};
        var_taun = assign7020_e7110;
        var_taun_dn0 = assign7020_e7110_d_n0;
        var_taun_dn1 = assign7020_e7110_d_n1;
        var_taun_dn3 = assign7020_e7110_d_n3;
        var_taun_dn4 = assign7020_e7110_d_n4;
        var_taun_dn5 = assign7020_e7110_d_n5;
        var_taun_dn6 = assign7020_e7110_d_n6;
        var_taun_dn7 = assign7020_e7110_d_n7;
        var_taun_dn8 = assign7020_e7110_d_n8;
        var_taun_dn9 = assign7020_e7110_d_n9;
        var_taun_dn10 = assign7020_e7110_d_n10;
        var_taun_rv = 0.0;

        let (assign7030_e7118, assign7030_e7118_d_n0, assign7030_e7118_d_n1, assign7030_e7118_d_n3, assign7030_e7118_d_n4, assign7030_e7118_d_n5, assign7030_e7118_d_n6, assign7030_e7118_d_n7, assign7030_e7118_d_n8, assign7030_e7118_d_n9, assign7030_e7118_d_n10,) = {
    if ((var_guard127 == 0.0) && (var_guard128 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_taun, var_taun_dn0, var_taun_dn1, var_taun_dn3, var_taun_dn4, var_taun_dn5, var_taun_dn6, var_taun_dn7, var_taun_dn8, var_taun_dn9, var_taun_dn10,)
    }
};
        var_taun = assign7030_e7118;
        var_taun_dn0 = assign7030_e7118_d_n0;
        var_taun_dn1 = assign7030_e7118_d_n1;
        var_taun_dn3 = assign7030_e7118_d_n3;
        var_taun_dn4 = assign7030_e7118_d_n4;
        var_taun_dn5 = assign7030_e7118_d_n5;
        var_taun_dn6 = assign7030_e7118_d_n6;
        var_taun_dn7 = assign7030_e7118_d_n7;
        var_taun_dn8 = assign7030_e7118_d_n8;
        var_taun_dn9 = assign7030_e7118_d_n9;
        var_taun_dn10 = assign7030_e7118_d_n10;
        var_taun_rv = 0.0;

        *var_guard126_slot = var_guard126;
        *var_guard126_rv_slot = var_guard126_rv;
        *var_guard127_slot = var_guard127;
        *var_guard127_rv_slot = var_guard127_rv;
        *var_guard128_slot = var_guard128;
        *var_guard128_rv_slot = var_guard128_rv;
        *var_in_n_slot = var_in_n;
        *var_in_n_dn0_slot = var_in_n_dn0;
        *var_in_n_dn1_slot = var_in_n_dn1;
        *var_in_n_dn10_slot = var_in_n_dn10;
        *var_in_n_dn3_slot = var_in_n_dn3;
        *var_in_n_dn4_slot = var_in_n_dn4;
        *var_in_n_dn5_slot = var_in_n_dn5;
        *var_in_n_dn6_slot = var_in_n_dn6;
        *var_in_n_dn7_slot = var_in_n_dn7;
        *var_in_n_dn8_slot = var_in_n_dn8;
        *var_in_n_dn9_slot = var_in_n_dn9;
        *var_in_n_rv_slot = var_in_n_rv;
        *var_qbc_slot = var_qbc;
        *var_qbc_dn0_slot = var_qbc_dn0;
        *var_qbc_dn1_slot = var_qbc_dn1;
        *var_qbc_dn10_slot = var_qbc_dn10;
        *var_qbc_dn3_slot = var_qbc_dn3;
        *var_qbc_dn4_slot = var_qbc_dn4;
        *var_qbc_dn5_slot = var_qbc_dn5;
        *var_qbc_dn6_slot = var_qbc_dn6;
        *var_qbc_dn7_slot = var_qbc_dn7;
        *var_qbc_dn8_slot = var_qbc_dn8;
        *var_qbc_dn9_slot = var_qbc_dn9;
        *var_qbc_rv_slot = var_qbc_rv;
        *var_qbe_slot = var_qbe;
        *var_qbe_dn0_slot = var_qbe_dn0;
        *var_qbe_dn1_slot = var_qbe_dn1;
        *var_qbe_dn10_slot = var_qbe_dn10;
        *var_qbe_dn3_slot = var_qbe_dn3;
        *var_qbe_dn4_slot = var_qbe_dn4;
        *var_qbe_dn5_slot = var_qbe_dn5;
        *var_qbe_dn6_slot = var_qbe_dn6;
        *var_qbe_dn7_slot = var_qbe_dn7;
        *var_qbe_dn8_slot = var_qbe_dn8;
        *var_qbe_dn9_slot = var_qbe_dn9;
        *var_qbe_rv_slot = var_qbe_rv;
        *var_qe_slot = var_qe;
        *var_qe_dn0_slot = var_qe_dn0;
        *var_qe_dn1_slot = var_qe_dn1;
        *var_qe_dn10_slot = var_qe_dn10;
        *var_qe_dn3_slot = var_qe_dn3;
        *var_qe_dn4_slot = var_qe_dn4;
        *var_qe_dn5_slot = var_qe_dn5;
        *var_qe_dn6_slot = var_qe_dn6;
        *var_qe_dn7_slot = var_qe_dn7;
        *var_qe_dn8_slot = var_qe_dn8;
        *var_qe_dn9_slot = var_qe_dn9;
        *var_qe_rv_slot = var_qe_rv;
        *var_taub_n_slot = var_taub_n;
        *var_taub_n_dn0_slot = var_taub_n_dn0;
        *var_taub_n_dn1_slot = var_taub_n_dn1;
        *var_taub_n_dn10_slot = var_taub_n_dn10;
        *var_taub_n_dn3_slot = var_taub_n_dn3;
        *var_taub_n_dn4_slot = var_taub_n_dn4;
        *var_taub_n_dn5_slot = var_taub_n_dn5;
        *var_taub_n_dn6_slot = var_taub_n_dn6;
        *var_taub_n_dn7_slot = var_taub_n_dn7;
        *var_taub_n_dn8_slot = var_taub_n_dn8;
        *var_taub_n_dn9_slot = var_taub_n_dn9;
        *var_taub_n_rv_slot = var_taub_n_rv;
        *var_taun_slot = var_taun;
        *var_taun_dn0_slot = var_taun_dn0;
        *var_taun_dn1_slot = var_taun_dn1;
        *var_taun_dn10_slot = var_taun_dn10;
        *var_taun_dn3_slot = var_taun_dn3;
        *var_taun_dn4_slot = var_taun_dn4;
        *var_taun_dn5_slot = var_taun_dn5;
        *var_taun_dn6_slot = var_taun_dn6;
        *var_taun_dn7_slot = var_taun_dn7;
        *var_taun_dn8_slot = var_taun_dn8;
        *var_taun_dn9_slot = var_taun_dn9;
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
        var_qb1b2_dn10: f64,
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
        var_qbc_dn10: f64,
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
        var_qbe_dn10: f64,
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
        var_qe_dn10: f64,
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
        var_qepi_dn10: f64,
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
        var_qex_dn10: f64,
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
        var_qtc_dn10: f64,
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
        var_qte_dn10: f64,
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
        var_qte_s_dn10: f64,
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
        var_qtex_dn10: f64,
        var_qtex_dn3: f64,
        var_qtex_dn4: f64,
        var_qtex_dn5: f64,
        var_qtex_dn6: f64,
        var_qtex_dn7: f64,
        var_qtex_dn8: f64,
        var_qtex_dn9: f64,
        var_qts: f64,
        var_qts_dn0: f64,
        var_qts_dn1: f64,
        var_qts_dn10: f64,
        var_qts_dn3: f64,
        var_qts_dn4: f64,
        var_qts_dn5: f64,
        var_qts_dn6: f64,
        var_qts_dn7: f64,
        var_qts_dn8: f64,
        var_qts_dn9: f64,
        var_taun: f64,
        var_taun_dn0: f64,
        var_taun_dn1: f64,
        var_taun_dn10: f64,
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
        var_xqex_dn10: f64,
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
        var_xqtex_dn10: f64,
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
        let eq14_e266_d_n3: f64 = (var_qte_dn3 + var_qbe_dn3);
        let eq14_e266_d_n4: f64 = (var_qte_dn4 + var_qbe_dn4);
        let eq14_e266_d_n5: f64 = (var_qte_dn5 + var_qbe_dn5);
        let eq14_e266_d_n6: f64 = (var_qte_dn6 + var_qbe_dn6);
        let eq14_e266_d_n7: f64 = (var_qte_dn7 + var_qbe_dn7);
        let eq14_e266_d_n8: f64 = (var_qte_dn8 + var_qbe_dn8);
        let eq14_e266_d_n9: f64 = (var_qte_dn9 + var_qbe_dn9);
        let eq14_e266_d_n10: f64 = (var_qte_dn10 + var_qbe_dn10);
        let eq14_e268: f64 = (eq14_e266 + var_qe);
        let eq14_e268_d_n0: f64 = (eq14_e266_d_n0 + var_qe_dn0);
        let eq14_e268_d_n1: f64 = (eq14_e266_d_n1 + var_qe_dn1);
        let eq14_e268_d_n3: f64 = (eq14_e266_d_n3 + var_qe_dn3);
        let eq14_e268_d_n4: f64 = (eq14_e266_d_n4 + var_qe_dn4);
        let eq14_e268_d_n5: f64 = (eq14_e266_d_n5 + var_qe_dn5);
        let eq14_e268_d_n6: f64 = (eq14_e266_d_n6 + var_qe_dn6);
        let eq14_e268_d_n7: f64 = (eq14_e266_d_n7 + var_qe_dn7);
        let eq14_e268_d_n8: f64 = (eq14_e266_d_n8 + var_qe_dn8);
        let eq14_e268_d_n9: f64 = (eq14_e266_d_n9 + var_qe_dn9);
        let eq14_e268_d_n10: f64 = (eq14_e266_d_n10 + var_qe_dn10);
        let eq14_e269: f64 = (p.p3 * eq14_e268);
        let eq14_e269_d_n0: f64 = (p.p3 * eq14_e268_d_n0);
        let eq14_e269_d_n1: f64 = (p.p3 * eq14_e268_d_n1);
        let eq14_e269_d_n3: f64 = (p.p3 * eq14_e268_d_n3);
        let eq14_e269_d_n4: f64 = (p.p3 * eq14_e268_d_n4);
        let eq14_e269_d_n5: f64 = (p.p3 * eq14_e268_d_n5);
        let eq14_e269_d_n6: f64 = (p.p3 * eq14_e268_d_n6);
        let eq14_e269_d_n7: f64 = (p.p3 * eq14_e268_d_n7);
        let eq14_e269_d_n8: f64 = (p.p3 * eq14_e268_d_n8);
        let eq14_e269_d_n9: f64 = (p.p3 * eq14_e268_d_n9);
        let eq14_e269_d_n10: f64 = (p.p3 * eq14_e268_d_n10);
        let eq14_e270: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq14_e269);
        let eq14_e272: f64 = (eq14_e270 * p.p1);
        let eq14_e272_d_n0: f64 = ((eq14_e269_d_n0 * ddt_scale) * p.p1);
        let eq14_e272_d_n1: f64 = ((eq14_e269_d_n1 * ddt_scale) * p.p1);
        let eq14_e272_d_n3: f64 = ((eq14_e269_d_n3 * ddt_scale) * p.p1);
        let eq14_e272_d_n4: f64 = ((eq14_e269_d_n4 * ddt_scale) * p.p1);
        let eq14_e272_d_n5: f64 = ((eq14_e269_d_n5 * ddt_scale) * p.p1);
        let eq14_e272_d_n6: f64 = ((eq14_e269_d_n6 * ddt_scale) * p.p1);
        let eq14_e272_d_n7: f64 = ((eq14_e269_d_n7 * ddt_scale) * p.p1);
        let eq14_e272_d_n8: f64 = ((eq14_e269_d_n8 * ddt_scale) * p.p1);
        let eq14_e272_d_n9: f64 = ((eq14_e269_d_n9 * ddt_scale) * p.p1);
        let eq14_e272_d_n10: f64 = ((eq14_e269_d_n10 * ddt_scale) * p.p1);
        let eq14_value: f64 = eq14_e272;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(4),
            multiplicity * (eq14_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq14_e272_d_n0), multiplicity * (eq14_e272_d_n1), multiplicity * (eq14_e272_d_n3), multiplicity * (eq14_e272_d_n4), multiplicity * (eq14_e272_d_n5), multiplicity * (eq14_e272_d_n6), multiplicity * (eq14_e272_d_n7), multiplicity * (eq14_e272_d_n8), multiplicity * (eq14_e272_d_n9), multiplicity * (eq14_e272_d_n10)],
            [],
            [],
            1.0,
        );
        let eq15_e275: f64 = (p.p3 * var_qte_s);
        let eq15_e275_d_n0: f64 = (p.p3 * var_qte_s_dn0);
        let eq15_e275_d_n1: f64 = (p.p3 * var_qte_s_dn1);
        let eq15_e275_d_n3: f64 = (p.p3 * var_qte_s_dn3);
        let eq15_e275_d_n4: f64 = (p.p3 * var_qte_s_dn4);
        let eq15_e275_d_n5: f64 = (p.p3 * var_qte_s_dn5);
        let eq15_e275_d_n6: f64 = (p.p3 * var_qte_s_dn6);
        let eq15_e275_d_n7: f64 = (p.p3 * var_qte_s_dn7);
        let eq15_e275_d_n8: f64 = (p.p3 * var_qte_s_dn8);
        let eq15_e275_d_n9: f64 = (p.p3 * var_qte_s_dn9);
        let eq15_e275_d_n10: f64 = (p.p3 * var_qte_s_dn10);
        let eq15_e276: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq15_e275);
        let eq15_e278: f64 = (eq15_e276 * p.p1);
        let eq15_e278_d_n0: f64 = ((eq15_e275_d_n0 * ddt_scale) * p.p1);
        let eq15_e278_d_n1: f64 = ((eq15_e275_d_n1 * ddt_scale) * p.p1);
        let eq15_e278_d_n3: f64 = ((eq15_e275_d_n3 * ddt_scale) * p.p1);
        let eq15_e278_d_n4: f64 = ((eq15_e275_d_n4 * ddt_scale) * p.p1);
        let eq15_e278_d_n5: f64 = ((eq15_e275_d_n5 * ddt_scale) * p.p1);
        let eq15_e278_d_n6: f64 = ((eq15_e275_d_n6 * ddt_scale) * p.p1);
        let eq15_e278_d_n7: f64 = ((eq15_e275_d_n7 * ddt_scale) * p.p1);
        let eq15_e278_d_n8: f64 = ((eq15_e275_d_n8 * ddt_scale) * p.p1);
        let eq15_e278_d_n9: f64 = ((eq15_e275_d_n9 * ddt_scale) * p.p1);
        let eq15_e278_d_n10: f64 = ((eq15_e275_d_n10 * ddt_scale) * p.p1);
        let eq15_value: f64 = eq15_e278;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(4),
            multiplicity * (eq15_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq15_e278_d_n0), multiplicity * (eq15_e278_d_n1), multiplicity * (eq15_e278_d_n3), multiplicity * (eq15_e278_d_n4), multiplicity * (eq15_e278_d_n5), multiplicity * (eq15_e278_d_n6), multiplicity * (eq15_e278_d_n7), multiplicity * (eq15_e278_d_n8), multiplicity * (eq15_e278_d_n9), multiplicity * (eq15_e278_d_n10)],
            [],
            [],
            1.0,
        );
        let eq16_e282: f64 = (var_qtc + var_qbc);
        let eq16_e282_d_n0: f64 = (var_qtc_dn0 + var_qbc_dn0);
        let eq16_e282_d_n1: f64 = (var_qtc_dn1 + var_qbc_dn1);
        let eq16_e282_d_n3: f64 = (var_qtc_dn3 + var_qbc_dn3);
        let eq16_e282_d_n4: f64 = (var_qtc_dn4 + var_qbc_dn4);
        let eq16_e282_d_n5: f64 = (var_qtc_dn5 + var_qbc_dn5);
        let eq16_e282_d_n6: f64 = (var_qtc_dn6 + var_qbc_dn6);
        let eq16_e282_d_n7: f64 = (var_qtc_dn7 + var_qbc_dn7);
        let eq16_e282_d_n8: f64 = (var_qtc_dn8 + var_qbc_dn8);
        let eq16_e282_d_n9: f64 = (var_qtc_dn9 + var_qbc_dn9);
        let eq16_e282_d_n10: f64 = (var_qtc_dn10 + var_qbc_dn10);
        let eq16_e284: f64 = (eq16_e282 + var_qepi);
        let eq16_e284_d_n0: f64 = (eq16_e282_d_n0 + var_qepi_dn0);
        let eq16_e284_d_n1: f64 = (eq16_e282_d_n1 + var_qepi_dn1);
        let eq16_e284_d_n3: f64 = (eq16_e282_d_n3 + var_qepi_dn3);
        let eq16_e284_d_n4: f64 = (eq16_e282_d_n4 + var_qepi_dn4);
        let eq16_e284_d_n5: f64 = (eq16_e282_d_n5 + var_qepi_dn5);
        let eq16_e284_d_n6: f64 = (eq16_e282_d_n6 + var_qepi_dn6);
        let eq16_e284_d_n7: f64 = (eq16_e282_d_n7 + var_qepi_dn7);
        let eq16_e284_d_n8: f64 = (eq16_e282_d_n8 + var_qepi_dn8);
        let eq16_e284_d_n9: f64 = (eq16_e282_d_n9 + var_qepi_dn9);
        let eq16_e284_d_n10: f64 = (eq16_e282_d_n10 + var_qepi_dn10);
        let eq16_e285: f64 = (p.p3 * eq16_e284);
        let eq16_e285_d_n0: f64 = (p.p3 * eq16_e284_d_n0);
        let eq16_e285_d_n1: f64 = (p.p3 * eq16_e284_d_n1);
        let eq16_e285_d_n3: f64 = (p.p3 * eq16_e284_d_n3);
        let eq16_e285_d_n4: f64 = (p.p3 * eq16_e284_d_n4);
        let eq16_e285_d_n5: f64 = (p.p3 * eq16_e284_d_n5);
        let eq16_e285_d_n6: f64 = (p.p3 * eq16_e284_d_n6);
        let eq16_e285_d_n7: f64 = (p.p3 * eq16_e284_d_n7);
        let eq16_e285_d_n8: f64 = (p.p3 * eq16_e284_d_n8);
        let eq16_e285_d_n9: f64 = (p.p3 * eq16_e284_d_n9);
        let eq16_e285_d_n10: f64 = (p.p3 * eq16_e284_d_n10);
        let eq16_e286: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq16_e285);
        let eq16_e288: f64 = (eq16_e286 * p.p1);
        let eq16_e288_d_n0: f64 = ((eq16_e285_d_n0 * ddt_scale) * p.p1);
        let eq16_e288_d_n1: f64 = ((eq16_e285_d_n1 * ddt_scale) * p.p1);
        let eq16_e288_d_n3: f64 = ((eq16_e285_d_n3 * ddt_scale) * p.p1);
        let eq16_e288_d_n4: f64 = ((eq16_e285_d_n4 * ddt_scale) * p.p1);
        let eq16_e288_d_n5: f64 = ((eq16_e285_d_n5 * ddt_scale) * p.p1);
        let eq16_e288_d_n6: f64 = ((eq16_e285_d_n6 * ddt_scale) * p.p1);
        let eq16_e288_d_n7: f64 = ((eq16_e285_d_n7 * ddt_scale) * p.p1);
        let eq16_e288_d_n8: f64 = ((eq16_e285_d_n8 * ddt_scale) * p.p1);
        let eq16_e288_d_n9: f64 = ((eq16_e285_d_n9 * ddt_scale) * p.p1);
        let eq16_e288_d_n10: f64 = ((eq16_e285_d_n10 * ddt_scale) * p.p1);
        let eq16_value: f64 = eq16_e288;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq16_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq16_e288_d_n0), multiplicity * (eq16_e288_d_n1), multiplicity * (eq16_e288_d_n3), multiplicity * (eq16_e288_d_n4), multiplicity * (eq16_e288_d_n5), multiplicity * (eq16_e288_d_n6), multiplicity * (eq16_e288_d_n7), multiplicity * (eq16_e288_d_n8), multiplicity * (eq16_e288_d_n9), multiplicity * (eq16_e288_d_n10)],
            [],
            [],
            1.0,
        );
        let eq17_e291: f64 = (p.p3 * var_qts);
        let eq17_e291_d_n0: f64 = (p.p3 * var_qts_dn0);
        let eq17_e291_d_n1: f64 = (p.p3 * var_qts_dn1);
        let eq17_e291_d_n3: f64 = (p.p3 * var_qts_dn3);
        let eq17_e291_d_n4: f64 = (p.p3 * var_qts_dn4);
        let eq17_e291_d_n5: f64 = (p.p3 * var_qts_dn5);
        let eq17_e291_d_n6: f64 = (p.p3 * var_qts_dn6);
        let eq17_e291_d_n7: f64 = (p.p3 * var_qts_dn7);
        let eq17_e291_d_n8: f64 = (p.p3 * var_qts_dn8);
        let eq17_e291_d_n9: f64 = (p.p3 * var_qts_dn9);
        let eq17_e291_d_n10: f64 = (p.p3 * var_qts_dn10);
        let eq17_e292: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq17_e291);
        let eq17_e294: f64 = (eq17_e292 * p.p1);
        let eq17_e294_d_n0: f64 = ((eq17_e291_d_n0 * ddt_scale) * p.p1);
        let eq17_e294_d_n1: f64 = ((eq17_e291_d_n1 * ddt_scale) * p.p1);
        let eq17_e294_d_n3: f64 = ((eq17_e291_d_n3 * ddt_scale) * p.p1);
        let eq17_e294_d_n4: f64 = ((eq17_e291_d_n4 * ddt_scale) * p.p1);
        let eq17_e294_d_n5: f64 = ((eq17_e291_d_n5 * ddt_scale) * p.p1);
        let eq17_e294_d_n6: f64 = ((eq17_e291_d_n6 * ddt_scale) * p.p1);
        let eq17_e294_d_n7: f64 = ((eq17_e291_d_n7 * ddt_scale) * p.p1);
        let eq17_e294_d_n8: f64 = ((eq17_e291_d_n8 * ddt_scale) * p.p1);
        let eq17_e294_d_n9: f64 = ((eq17_e291_d_n9 * ddt_scale) * p.p1);
        let eq17_e294_d_n10: f64 = ((eq17_e291_d_n10 * ddt_scale) * p.p1);
        let eq17_value: f64 = eq17_e294;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(3),
            Some(7),
            multiplicity * (eq17_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq17_e294_d_n0), multiplicity * (eq17_e294_d_n1), multiplicity * (eq17_e294_d_n3), multiplicity * (eq17_e294_d_n4), multiplicity * (eq17_e294_d_n5), multiplicity * (eq17_e294_d_n6), multiplicity * (eq17_e294_d_n7), multiplicity * (eq17_e294_d_n8), multiplicity * (eq17_e294_d_n9), multiplicity * (eq17_e294_d_n10)],
            [],
            [],
            1.0,
        );
        let eq18_e297: f64 = (p.p3 * var_qb1b2);
        let eq18_e297_d_n0: f64 = (p.p3 * var_qb1b2_dn0);
        let eq18_e297_d_n1: f64 = (p.p3 * var_qb1b2_dn1);
        let eq18_e297_d_n3: f64 = (p.p3 * var_qb1b2_dn3);
        let eq18_e297_d_n4: f64 = (p.p3 * var_qb1b2_dn4);
        let eq18_e297_d_n5: f64 = (p.p3 * var_qb1b2_dn5);
        let eq18_e297_d_n6: f64 = (p.p3 * var_qb1b2_dn6);
        let eq18_e297_d_n7: f64 = (p.p3 * var_qb1b2_dn7);
        let eq18_e297_d_n8: f64 = (p.p3 * var_qb1b2_dn8);
        let eq18_e297_d_n9: f64 = (p.p3 * var_qb1b2_dn9);
        let eq18_e297_d_n10: f64 = (p.p3 * var_qb1b2_dn10);
        let eq18_e298: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq18_e297);
        let eq18_e300: f64 = (eq18_e298 * p.p1);
        let eq18_e300_d_n0: f64 = ((eq18_e297_d_n0 * ddt_scale) * p.p1);
        let eq18_e300_d_n1: f64 = ((eq18_e297_d_n1 * ddt_scale) * p.p1);
        let eq18_e300_d_n3: f64 = ((eq18_e297_d_n3 * ddt_scale) * p.p1);
        let eq18_e300_d_n4: f64 = ((eq18_e297_d_n4 * ddt_scale) * p.p1);
        let eq18_e300_d_n5: f64 = ((eq18_e297_d_n5 * ddt_scale) * p.p1);
        let eq18_e300_d_n6: f64 = ((eq18_e297_d_n6 * ddt_scale) * p.p1);
        let eq18_e300_d_n7: f64 = ((eq18_e297_d_n7 * ddt_scale) * p.p1);
        let eq18_e300_d_n8: f64 = ((eq18_e297_d_n8 * ddt_scale) * p.p1);
        let eq18_e300_d_n9: f64 = ((eq18_e297_d_n9 * ddt_scale) * p.p1);
        let eq18_e300_d_n10: f64 = ((eq18_e297_d_n10 * ddt_scale) * p.p1);
        let eq18_value: f64 = eq18_e300;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq18_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq18_e300_d_n0), multiplicity * (eq18_e300_d_n1), multiplicity * (eq18_e300_d_n3), multiplicity * (eq18_e300_d_n4), multiplicity * (eq18_e300_d_n5), multiplicity * (eq18_e300_d_n6), multiplicity * (eq18_e300_d_n7), multiplicity * (eq18_e300_d_n8), multiplicity * (eq18_e300_d_n9), multiplicity * (eq18_e300_d_n10)],
            [],
            [],
            1.0,
        );
        let eq19_e303: f64 = (p.p3 * p.p69);
        let eq19_e305: f64 = (eq19_e303 * var_vbe);
        let eq19_e305_d_n1: f64 = (eq19_e303 * var_vbe_dn1);
        let eq19_e305_d_n2: f64 = (eq19_e303 * var_vbe_dn2);
        let eq19_e306: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq19_e305);
        let eq19_e308: f64 = (eq19_e306 * p.p1);
        let eq19_e308_d_n1: f64 = ((eq19_e305_d_n1 * ddt_scale) * p.p1);
        let eq19_e308_d_n2: f64 = ((eq19_e305_d_n2 * ddt_scale) * p.p1);
        let eq19_value: f64 = eq19_e308;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (eq19_value),
            1,
            multiplicity * (eq19_e308_d_n1),
            2,
            multiplicity * (eq19_e308_d_n2),
        );
        let eq20_e311: f64 = (p.p3 * p.p78);
        let eq20_e313: f64 = (eq20_e311 * var_vbc);
        let eq20_e313_d_n0: f64 = (eq20_e311 * var_vbc_dn0);
        let eq20_e313_d_n1: f64 = (eq20_e311 * var_vbc_dn1);
        let eq20_e314: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq20_e313);
        let eq20_e316: f64 = (eq20_e314 * p.p1);
        let eq20_e316_d_n0: f64 = ((eq20_e313_d_n0 * ddt_scale) * p.p1);
        let eq20_e316_d_n1: f64 = ((eq20_e313_d_n1 * ddt_scale) * p.p1);
        let eq20_value: f64 = eq20_e316;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * (eq20_value),
            0,
            multiplicity * (eq20_e316_d_n0),
            1,
            multiplicity * (eq20_e316_d_n1),
        );
        let eq23_e332: f64 = (var_xqtex + var_xqex);
        let eq23_e332_d_n0: f64 = (var_xqtex_dn0 + var_xqex_dn0);
        let eq23_e332_d_n1: f64 = (var_xqtex_dn1 + var_xqex_dn1);
        let eq23_e332_d_n3: f64 = (var_xqtex_dn3 + var_xqex_dn3);
        let eq23_e332_d_n4: f64 = (var_xqtex_dn4 + var_xqex_dn4);
        let eq23_e332_d_n5: f64 = (var_xqtex_dn5 + var_xqex_dn5);
        let eq23_e332_d_n6: f64 = (var_xqtex_dn6 + var_xqex_dn6);
        let eq23_e332_d_n7: f64 = (var_xqtex_dn7 + var_xqex_dn7);
        let eq23_e332_d_n8: f64 = (var_xqtex_dn8 + var_xqex_dn8);
        let eq23_e332_d_n9: f64 = (var_xqtex_dn9 + var_xqex_dn9);
        let eq23_e332_d_n10: f64 = (var_xqtex_dn10 + var_xqex_dn10);
        let eq23_e333: f64 = (p.p3 * eq23_e332);
        let eq23_e333_d_n0: f64 = (p.p3 * eq23_e332_d_n0);
        let eq23_e333_d_n1: f64 = (p.p3 * eq23_e332_d_n1);
        let eq23_e333_d_n3: f64 = (p.p3 * eq23_e332_d_n3);
        let eq23_e333_d_n4: f64 = (p.p3 * eq23_e332_d_n4);
        let eq23_e333_d_n5: f64 = (p.p3 * eq23_e332_d_n5);
        let eq23_e333_d_n6: f64 = (p.p3 * eq23_e332_d_n6);
        let eq23_e333_d_n7: f64 = (p.p3 * eq23_e332_d_n7);
        let eq23_e333_d_n8: f64 = (p.p3 * eq23_e332_d_n8);
        let eq23_e333_d_n9: f64 = (p.p3 * eq23_e332_d_n9);
        let eq23_e333_d_n10: f64 = (p.p3 * eq23_e332_d_n10);
        let eq23_e334: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq23_e333);
        let eq23_e336: f64 = (eq23_e334 * p.p1);
        let eq23_e336_d_n0: f64 = ((eq23_e333_d_n0 * ddt_scale) * p.p1);
        let eq23_e336_d_n1: f64 = ((eq23_e333_d_n1 * ddt_scale) * p.p1);
        let eq23_e336_d_n3: f64 = ((eq23_e333_d_n3 * ddt_scale) * p.p1);
        let eq23_e336_d_n4: f64 = ((eq23_e333_d_n4 * ddt_scale) * p.p1);
        let eq23_e336_d_n5: f64 = ((eq23_e333_d_n5 * ddt_scale) * p.p1);
        let eq23_e336_d_n6: f64 = ((eq23_e333_d_n6 * ddt_scale) * p.p1);
        let eq23_e336_d_n7: f64 = ((eq23_e333_d_n7 * ddt_scale) * p.p1);
        let eq23_e336_d_n8: f64 = ((eq23_e333_d_n8 * ddt_scale) * p.p1);
        let eq23_e336_d_n9: f64 = ((eq23_e333_d_n9 * ddt_scale) * p.p1);
        let eq23_e336_d_n10: f64 = ((eq23_e333_d_n10 * ddt_scale) * p.p1);
        let eq23_value: f64 = eq23_e336;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(9),
            multiplicity * (eq23_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq23_e336_d_n0), multiplicity * (eq23_e336_d_n1), multiplicity * (eq23_e336_d_n3), multiplicity * (eq23_e336_d_n4), multiplicity * (eq23_e336_d_n5), multiplicity * (eq23_e336_d_n6), multiplicity * (eq23_e336_d_n7), multiplicity * (eq23_e336_d_n8), multiplicity * (eq23_e336_d_n9), multiplicity * (eq23_e336_d_n10)],
            [],
            [],
            1.0,
        );
        let eq25_e351: f64 = (var_qtex + var_qex);
        let eq25_e351_d_n0: f64 = (var_qtex_dn0 + var_qex_dn0);
        let eq25_e351_d_n1: f64 = (var_qtex_dn1 + var_qex_dn1);
        let eq25_e351_d_n3: f64 = (var_qtex_dn3 + var_qex_dn3);
        let eq25_e351_d_n4: f64 = (var_qtex_dn4 + var_qex_dn4);
        let eq25_e351_d_n5: f64 = (var_qtex_dn5 + var_qex_dn5);
        let eq25_e351_d_n6: f64 = (var_qtex_dn6 + var_qex_dn6);
        let eq25_e351_d_n7: f64 = (var_qtex_dn7 + var_qex_dn7);
        let eq25_e351_d_n8: f64 = (var_qtex_dn8 + var_qex_dn8);
        let eq25_e351_d_n9: f64 = (var_qtex_dn9 + var_qex_dn9);
        let eq25_e351_d_n10: f64 = (var_qtex_dn10 + var_qex_dn10);
        let eq25_e352: f64 = (p.p3 * eq25_e351);
        let eq25_e352_d_n0: f64 = (p.p3 * eq25_e351_d_n0);
        let eq25_e352_d_n1: f64 = (p.p3 * eq25_e351_d_n1);
        let eq25_e352_d_n3: f64 = (p.p3 * eq25_e351_d_n3);
        let eq25_e352_d_n4: f64 = (p.p3 * eq25_e351_d_n4);
        let eq25_e352_d_n5: f64 = (p.p3 * eq25_e351_d_n5);
        let eq25_e352_d_n6: f64 = (p.p3 * eq25_e351_d_n6);
        let eq25_e352_d_n7: f64 = (p.p3 * eq25_e351_d_n7);
        let eq25_e352_d_n8: f64 = (p.p3 * eq25_e351_d_n8);
        let eq25_e352_d_n9: f64 = (p.p3 * eq25_e351_d_n9);
        let eq25_e352_d_n10: f64 = (p.p3 * eq25_e351_d_n10);
        let eq25_e353: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq25_e352);
        let eq25_e355: f64 = (eq25_e353 * p.p1);
        let eq25_e355_d_n0: f64 = ((eq25_e352_d_n0 * ddt_scale) * p.p1);
        let eq25_e355_d_n1: f64 = ((eq25_e352_d_n1 * ddt_scale) * p.p1);
        let eq25_e355_d_n3: f64 = ((eq25_e352_d_n3 * ddt_scale) * p.p1);
        let eq25_e355_d_n4: f64 = ((eq25_e352_d_n4 * ddt_scale) * p.p1);
        let eq25_e355_d_n5: f64 = ((eq25_e352_d_n5 * ddt_scale) * p.p1);
        let eq25_e355_d_n6: f64 = ((eq25_e352_d_n6 * ddt_scale) * p.p1);
        let eq25_e355_d_n7: f64 = ((eq25_e352_d_n7 * ddt_scale) * p.p1);
        let eq25_e355_d_n8: f64 = ((eq25_e352_d_n8 * ddt_scale) * p.p1);
        let eq25_e355_d_n9: f64 = ((eq25_e352_d_n9 * ddt_scale) * p.p1);
        let eq25_e355_d_n10: f64 = ((eq25_e352_d_n10 * ddt_scale) * p.p1);
        let eq25_value: f64 = eq25_e355;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(10),
            multiplicity * (eq25_value),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [multiplicity * (eq25_e355_d_n0), multiplicity * (eq25_e355_d_n1), multiplicity * (eq25_e355_d_n3), multiplicity * (eq25_e355_d_n4), multiplicity * (eq25_e355_d_n5), multiplicity * (eq25_e355_d_n6), multiplicity * (eq25_e355_d_n7), multiplicity * (eq25_e355_d_n8), multiplicity * (eq25_e355_d_n9), multiplicity * (eq25_e355_d_n10)],
            [],
            [],
            1.0,
        );
        let eq32_e394: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, (nv11 - 0.0));
        let eq32_e395: f64 = (var_taun * eq32_e394);
        let eq32_e395_d_n0: f64 = (var_taun_dn0 * eq32_e394);
        let eq32_e395_d_n1: f64 = (var_taun_dn1 * eq32_e394);
        let eq32_e395_d_n3: f64 = (var_taun_dn3 * eq32_e394);
        let eq32_e395_d_n4: f64 = (var_taun_dn4 * eq32_e394);
        let eq32_e395_d_n5: f64 = (var_taun_dn5 * eq32_e394);
        let eq32_e395_d_n6: f64 = (var_taun_dn6 * eq32_e394);
        let eq32_e395_d_n7: f64 = (var_taun_dn7 * eq32_e394);
        let eq32_e395_d_n8: f64 = (var_taun_dn8 * eq32_e394);
        let eq32_e395_d_n9: f64 = (var_taun_dn9 * eq32_e394);
        let eq32_e395_d_n10: f64 = (var_taun_dn10 * eq32_e394);
        let eq32_value: f64 = eq32_e395;
        let eq32_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq32_node_derivatives: [f64; 11] = [eq32_e395_d_n0, eq32_e395_d_n1, eq32_e395_d_n3, eq32_e395_d_n4, eq32_e395_d_n5, eq32_e395_d_n6, eq32_e395_d_n7, eq32_e395_d_n8, eq32_e395_d_n9, eq32_e395_d_n10, (var_taun * ddt_scale)];
        let eq32_branch_derivative_indices: [usize; 0] = [];
        let eq32_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(4),
            multiplicity * (eq32_value),
            &eq32_node_derivative_indices,
            &eq32_node_derivatives,
            &eq32_branch_derivative_indices,
            &eq32_branch_derivatives,
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
        var_qb1b2: f64,
        var_qb1b2_dn0: f64,
        var_qb1b2_dn1: f64,
        var_qb1b2_dn10: f64,
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
        var_qbc_dn10: f64,
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
        var_qbe_dn10: f64,
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
        var_qe_dn10: f64,
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
        var_qepi_dn10: f64,
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
        var_qex_dn10: f64,
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
        var_qtc_dn10: f64,
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
        var_qte_dn10: f64,
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
        var_qte_s_dn10: f64,
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
        var_qtex_dn10: f64,
        var_qtex_dn3: f64,
        var_qtex_dn4: f64,
        var_qtex_dn5: f64,
        var_qtex_dn6: f64,
        var_qtex_dn7: f64,
        var_qtex_dn8: f64,
        var_qtex_dn9: f64,
        var_qts: f64,
        var_qts_dn0: f64,
        var_qts_dn1: f64,
        var_qts_dn10: f64,
        var_qts_dn3: f64,
        var_qts_dn4: f64,
        var_qts_dn5: f64,
        var_qts_dn6: f64,
        var_qts_dn7: f64,
        var_qts_dn8: f64,
        var_qts_dn9: f64,
        var_taun: f64,
        var_taun_dn0: f64,
        var_taun_dn1: f64,
        var_taun_dn10: f64,
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
        var_xqex_dn10: f64,
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
        var_xqtex_dn10: f64,
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
        let eq14_e266_d_n3: f64 = (var_qte_dn3 + var_qbe_dn3);
        let eq14_e266_d_n4: f64 = (var_qte_dn4 + var_qbe_dn4);
        let eq14_e266_d_n5: f64 = (var_qte_dn5 + var_qbe_dn5);
        let eq14_e266_d_n6: f64 = (var_qte_dn6 + var_qbe_dn6);
        let eq14_e266_d_n7: f64 = (var_qte_dn7 + var_qbe_dn7);
        let eq14_e266_d_n8: f64 = (var_qte_dn8 + var_qbe_dn8);
        let eq14_e266_d_n9: f64 = (var_qte_dn9 + var_qbe_dn9);
        let eq14_e266_d_n10: f64 = (var_qte_dn10 + var_qbe_dn10);
        let eq14_e268: f64 = (eq14_e266 + var_qe);
        let eq14_e268_d_n0: f64 = (eq14_e266_d_n0 + var_qe_dn0);
        let eq14_e268_d_n1: f64 = (eq14_e266_d_n1 + var_qe_dn1);
        let eq14_e268_d_n3: f64 = (eq14_e266_d_n3 + var_qe_dn3);
        let eq14_e268_d_n4: f64 = (eq14_e266_d_n4 + var_qe_dn4);
        let eq14_e268_d_n5: f64 = (eq14_e266_d_n5 + var_qe_dn5);
        let eq14_e268_d_n6: f64 = (eq14_e266_d_n6 + var_qe_dn6);
        let eq14_e268_d_n7: f64 = (eq14_e266_d_n7 + var_qe_dn7);
        let eq14_e268_d_n8: f64 = (eq14_e266_d_n8 + var_qe_dn8);
        let eq14_e268_d_n9: f64 = (eq14_e266_d_n9 + var_qe_dn9);
        let eq14_e268_d_n10: f64 = (eq14_e266_d_n10 + var_qe_dn10);
        let eq14_e269: f64 = (p.p3 * eq14_e268);
        let eq14_e269_d_n0: f64 = (p.p3 * eq14_e268_d_n0);
        let eq14_e269_d_n1: f64 = (p.p3 * eq14_e268_d_n1);
        let eq14_e269_d_n3: f64 = (p.p3 * eq14_e268_d_n3);
        let eq14_e269_d_n4: f64 = (p.p3 * eq14_e268_d_n4);
        let eq14_e269_d_n5: f64 = (p.p3 * eq14_e268_d_n5);
        let eq14_e269_d_n6: f64 = (p.p3 * eq14_e268_d_n6);
        let eq14_e269_d_n7: f64 = (p.p3 * eq14_e268_d_n7);
        let eq14_e269_d_n8: f64 = (p.p3 * eq14_e268_d_n8);
        let eq14_e269_d_n9: f64 = (p.p3 * eq14_e268_d_n9);
        let eq14_e269_d_n10: f64 = (p.p3 * eq14_e268_d_n10);
        let eq14_e270_q: f64 = eq14_e269;
        let eq14_e272: f64 = (eq14_e269 * p.p1);
        let eq14_e272_d_n0: f64 = (eq14_e269_d_n0 * p.p1);
        let eq14_e272_d_n1: f64 = (eq14_e269_d_n1 * p.p1);
        let eq14_e272_d_n3: f64 = (eq14_e269_d_n3 * p.p1);
        let eq14_e272_d_n4: f64 = (eq14_e269_d_n4 * p.p1);
        let eq14_e272_d_n5: f64 = (eq14_e269_d_n5 * p.p1);
        let eq14_e272_d_n6: f64 = (eq14_e269_d_n6 * p.p1);
        let eq14_e272_d_n7: f64 = (eq14_e269_d_n7 * p.p1);
        let eq14_e272_d_n8: f64 = (eq14_e269_d_n8 * p.p1);
        let eq14_e272_d_n9: f64 = (eq14_e269_d_n9 * p.p1);
        let eq14_e272_d_n10: f64 = (eq14_e269_d_n10 * p.p1);
        let eq14_e272_q: f64 = (eq14_e270_q * p.p1);
        let eq14_reactive_node_derivatives: [f64; 12] = [eq14_e272_d_n0, eq14_e272_d_n1, 0.0, eq14_e272_d_n3, eq14_e272_d_n4, eq14_e272_d_n5, eq14_e272_d_n6, eq14_e272_d_n7, eq14_e272_d_n8, eq14_e272_d_n9, eq14_e272_d_n10, 0.0];
        let eq14_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            nodes,
            &eq14_reactive_node_derivatives,
            branches,
            &eq14_reactive_branch_derivatives,
            multiplicity,
        );
        let eq15_e275: f64 = (p.p3 * var_qte_s);
        let eq15_e275_d_n0: f64 = (p.p3 * var_qte_s_dn0);
        let eq15_e275_d_n1: f64 = (p.p3 * var_qte_s_dn1);
        let eq15_e275_d_n3: f64 = (p.p3 * var_qte_s_dn3);
        let eq15_e275_d_n4: f64 = (p.p3 * var_qte_s_dn4);
        let eq15_e275_d_n5: f64 = (p.p3 * var_qte_s_dn5);
        let eq15_e275_d_n6: f64 = (p.p3 * var_qte_s_dn6);
        let eq15_e275_d_n7: f64 = (p.p3 * var_qte_s_dn7);
        let eq15_e275_d_n8: f64 = (p.p3 * var_qte_s_dn8);
        let eq15_e275_d_n9: f64 = (p.p3 * var_qte_s_dn9);
        let eq15_e275_d_n10: f64 = (p.p3 * var_qte_s_dn10);
        let eq15_e276_q: f64 = eq15_e275;
        let eq15_e278: f64 = (eq15_e275 * p.p1);
        let eq15_e278_d_n0: f64 = (eq15_e275_d_n0 * p.p1);
        let eq15_e278_d_n1: f64 = (eq15_e275_d_n1 * p.p1);
        let eq15_e278_d_n3: f64 = (eq15_e275_d_n3 * p.p1);
        let eq15_e278_d_n4: f64 = (eq15_e275_d_n4 * p.p1);
        let eq15_e278_d_n5: f64 = (eq15_e275_d_n5 * p.p1);
        let eq15_e278_d_n6: f64 = (eq15_e275_d_n6 * p.p1);
        let eq15_e278_d_n7: f64 = (eq15_e275_d_n7 * p.p1);
        let eq15_e278_d_n8: f64 = (eq15_e275_d_n8 * p.p1);
        let eq15_e278_d_n9: f64 = (eq15_e275_d_n9 * p.p1);
        let eq15_e278_d_n10: f64 = (eq15_e275_d_n10 * p.p1);
        let eq15_e278_q: f64 = (eq15_e276_q * p.p1);
        let eq15_reactive_node_derivatives: [f64; 12] = [eq15_e278_d_n0, eq15_e278_d_n1, 0.0, eq15_e278_d_n3, eq15_e278_d_n4, eq15_e278_d_n5, eq15_e278_d_n6, eq15_e278_d_n7, eq15_e278_d_n8, eq15_e278_d_n9, eq15_e278_d_n10, 0.0];
        let eq15_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[4]),
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let eq16_e282: f64 = (var_qtc + var_qbc);
        let eq16_e282_d_n0: f64 = (var_qtc_dn0 + var_qbc_dn0);
        let eq16_e282_d_n1: f64 = (var_qtc_dn1 + var_qbc_dn1);
        let eq16_e282_d_n3: f64 = (var_qtc_dn3 + var_qbc_dn3);
        let eq16_e282_d_n4: f64 = (var_qtc_dn4 + var_qbc_dn4);
        let eq16_e282_d_n5: f64 = (var_qtc_dn5 + var_qbc_dn5);
        let eq16_e282_d_n6: f64 = (var_qtc_dn6 + var_qbc_dn6);
        let eq16_e282_d_n7: f64 = (var_qtc_dn7 + var_qbc_dn7);
        let eq16_e282_d_n8: f64 = (var_qtc_dn8 + var_qbc_dn8);
        let eq16_e282_d_n9: f64 = (var_qtc_dn9 + var_qbc_dn9);
        let eq16_e282_d_n10: f64 = (var_qtc_dn10 + var_qbc_dn10);
        let eq16_e284: f64 = (eq16_e282 + var_qepi);
        let eq16_e284_d_n0: f64 = (eq16_e282_d_n0 + var_qepi_dn0);
        let eq16_e284_d_n1: f64 = (eq16_e282_d_n1 + var_qepi_dn1);
        let eq16_e284_d_n3: f64 = (eq16_e282_d_n3 + var_qepi_dn3);
        let eq16_e284_d_n4: f64 = (eq16_e282_d_n4 + var_qepi_dn4);
        let eq16_e284_d_n5: f64 = (eq16_e282_d_n5 + var_qepi_dn5);
        let eq16_e284_d_n6: f64 = (eq16_e282_d_n6 + var_qepi_dn6);
        let eq16_e284_d_n7: f64 = (eq16_e282_d_n7 + var_qepi_dn7);
        let eq16_e284_d_n8: f64 = (eq16_e282_d_n8 + var_qepi_dn8);
        let eq16_e284_d_n9: f64 = (eq16_e282_d_n9 + var_qepi_dn9);
        let eq16_e284_d_n10: f64 = (eq16_e282_d_n10 + var_qepi_dn10);
        let eq16_e285: f64 = (p.p3 * eq16_e284);
        let eq16_e285_d_n0: f64 = (p.p3 * eq16_e284_d_n0);
        let eq16_e285_d_n1: f64 = (p.p3 * eq16_e284_d_n1);
        let eq16_e285_d_n3: f64 = (p.p3 * eq16_e284_d_n3);
        let eq16_e285_d_n4: f64 = (p.p3 * eq16_e284_d_n4);
        let eq16_e285_d_n5: f64 = (p.p3 * eq16_e284_d_n5);
        let eq16_e285_d_n6: f64 = (p.p3 * eq16_e284_d_n6);
        let eq16_e285_d_n7: f64 = (p.p3 * eq16_e284_d_n7);
        let eq16_e285_d_n8: f64 = (p.p3 * eq16_e284_d_n8);
        let eq16_e285_d_n9: f64 = (p.p3 * eq16_e284_d_n9);
        let eq16_e285_d_n10: f64 = (p.p3 * eq16_e284_d_n10);
        let eq16_e286_q: f64 = eq16_e285;
        let eq16_e288: f64 = (eq16_e285 * p.p1);
        let eq16_e288_d_n0: f64 = (eq16_e285_d_n0 * p.p1);
        let eq16_e288_d_n1: f64 = (eq16_e285_d_n1 * p.p1);
        let eq16_e288_d_n3: f64 = (eq16_e285_d_n3 * p.p1);
        let eq16_e288_d_n4: f64 = (eq16_e285_d_n4 * p.p1);
        let eq16_e288_d_n5: f64 = (eq16_e285_d_n5 * p.p1);
        let eq16_e288_d_n6: f64 = (eq16_e285_d_n6 * p.p1);
        let eq16_e288_d_n7: f64 = (eq16_e285_d_n7 * p.p1);
        let eq16_e288_d_n8: f64 = (eq16_e285_d_n8 * p.p1);
        let eq16_e288_d_n9: f64 = (eq16_e285_d_n9 * p.p1);
        let eq16_e288_d_n10: f64 = (eq16_e285_d_n10 * p.p1);
        let eq16_e288_q: f64 = (eq16_e286_q * p.p1);
        let eq16_reactive_node_derivatives: [f64; 12] = [eq16_e288_d_n0, eq16_e288_d_n1, 0.0, eq16_e288_d_n3, eq16_e288_d_n4, eq16_e288_d_n5, eq16_e288_d_n6, eq16_e288_d_n7, eq16_e288_d_n8, eq16_e288_d_n9, eq16_e288_d_n10, 0.0];
        let eq16_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            nodes,
            &eq16_reactive_node_derivatives,
            branches,
            &eq16_reactive_branch_derivatives,
            multiplicity,
        );
        let eq17_e291: f64 = (p.p3 * var_qts);
        let eq17_e291_d_n0: f64 = (p.p3 * var_qts_dn0);
        let eq17_e291_d_n1: f64 = (p.p3 * var_qts_dn1);
        let eq17_e291_d_n3: f64 = (p.p3 * var_qts_dn3);
        let eq17_e291_d_n4: f64 = (p.p3 * var_qts_dn4);
        let eq17_e291_d_n5: f64 = (p.p3 * var_qts_dn5);
        let eq17_e291_d_n6: f64 = (p.p3 * var_qts_dn6);
        let eq17_e291_d_n7: f64 = (p.p3 * var_qts_dn7);
        let eq17_e291_d_n8: f64 = (p.p3 * var_qts_dn8);
        let eq17_e291_d_n9: f64 = (p.p3 * var_qts_dn9);
        let eq17_e291_d_n10: f64 = (p.p3 * var_qts_dn10);
        let eq17_e292_q: f64 = eq17_e291;
        let eq17_e294: f64 = (eq17_e291 * p.p1);
        let eq17_e294_d_n0: f64 = (eq17_e291_d_n0 * p.p1);
        let eq17_e294_d_n1: f64 = (eq17_e291_d_n1 * p.p1);
        let eq17_e294_d_n3: f64 = (eq17_e291_d_n3 * p.p1);
        let eq17_e294_d_n4: f64 = (eq17_e291_d_n4 * p.p1);
        let eq17_e294_d_n5: f64 = (eq17_e291_d_n5 * p.p1);
        let eq17_e294_d_n6: f64 = (eq17_e291_d_n6 * p.p1);
        let eq17_e294_d_n7: f64 = (eq17_e291_d_n7 * p.p1);
        let eq17_e294_d_n8: f64 = (eq17_e291_d_n8 * p.p1);
        let eq17_e294_d_n9: f64 = (eq17_e291_d_n9 * p.p1);
        let eq17_e294_d_n10: f64 = (eq17_e291_d_n10 * p.p1);
        let eq17_e294_q: f64 = (eq17_e292_q * p.p1);
        let eq17_reactive_node_derivatives: [f64; 12] = [eq17_e294_d_n0, eq17_e294_d_n1, 0.0, eq17_e294_d_n3, eq17_e294_d_n4, eq17_e294_d_n5, eq17_e294_d_n6, eq17_e294_d_n7, eq17_e294_d_n8, eq17_e294_d_n9, eq17_e294_d_n10, 0.0];
        let eq17_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[7]),
            nodes,
            &eq17_reactive_node_derivatives,
            branches,
            &eq17_reactive_branch_derivatives,
            multiplicity,
        );
        let eq18_e297: f64 = (p.p3 * var_qb1b2);
        let eq18_e297_d_n0: f64 = (p.p3 * var_qb1b2_dn0);
        let eq18_e297_d_n1: f64 = (p.p3 * var_qb1b2_dn1);
        let eq18_e297_d_n3: f64 = (p.p3 * var_qb1b2_dn3);
        let eq18_e297_d_n4: f64 = (p.p3 * var_qb1b2_dn4);
        let eq18_e297_d_n5: f64 = (p.p3 * var_qb1b2_dn5);
        let eq18_e297_d_n6: f64 = (p.p3 * var_qb1b2_dn6);
        let eq18_e297_d_n7: f64 = (p.p3 * var_qb1b2_dn7);
        let eq18_e297_d_n8: f64 = (p.p3 * var_qb1b2_dn8);
        let eq18_e297_d_n9: f64 = (p.p3 * var_qb1b2_dn9);
        let eq18_e297_d_n10: f64 = (p.p3 * var_qb1b2_dn10);
        let eq18_e298_q: f64 = eq18_e297;
        let eq18_e300: f64 = (eq18_e297 * p.p1);
        let eq18_e300_d_n0: f64 = (eq18_e297_d_n0 * p.p1);
        let eq18_e300_d_n1: f64 = (eq18_e297_d_n1 * p.p1);
        let eq18_e300_d_n3: f64 = (eq18_e297_d_n3 * p.p1);
        let eq18_e300_d_n4: f64 = (eq18_e297_d_n4 * p.p1);
        let eq18_e300_d_n5: f64 = (eq18_e297_d_n5 * p.p1);
        let eq18_e300_d_n6: f64 = (eq18_e297_d_n6 * p.p1);
        let eq18_e300_d_n7: f64 = (eq18_e297_d_n7 * p.p1);
        let eq18_e300_d_n8: f64 = (eq18_e297_d_n8 * p.p1);
        let eq18_e300_d_n9: f64 = (eq18_e297_d_n9 * p.p1);
        let eq18_e300_d_n10: f64 = (eq18_e297_d_n10 * p.p1);
        let eq18_e300_q: f64 = (eq18_e298_q * p.p1);
        let eq18_reactive_node_derivatives: [f64; 12] = [eq18_e300_d_n0, eq18_e300_d_n1, 0.0, eq18_e300_d_n3, eq18_e300_d_n4, eq18_e300_d_n5, eq18_e300_d_n6, eq18_e300_d_n7, eq18_e300_d_n8, eq18_e300_d_n9, eq18_e300_d_n10, 0.0];
        let eq18_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
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
        let eq19_e305: f64 = (eq19_e303 * var_vbe);
        let eq19_e305_d_n1: f64 = (eq19_e303 * var_vbe_dn1);
        let eq19_e305_d_n2: f64 = (eq19_e303 * var_vbe_dn2);
        let eq19_e306_q: f64 = eq19_e305;
        let eq19_e308: f64 = (eq19_e305 * p.p1);
        let eq19_e308_d_n1: f64 = (eq19_e305_d_n1 * p.p1);
        let eq19_e308_d_n2: f64 = (eq19_e305_d_n2 * p.p1);
        let eq19_e308_q: f64 = (eq19_e306_q * p.p1);
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * (eq19_e308_d_n1),
            nodes[2],
            multiplicity * (eq19_e308_d_n2),
        );
        let eq20_e311: f64 = (p.p3 * p.p78);
        let eq20_e313: f64 = (eq20_e311 * var_vbc);
        let eq20_e313_d_n0: f64 = (eq20_e311 * var_vbc_dn0);
        let eq20_e313_d_n1: f64 = (eq20_e311 * var_vbc_dn1);
        let eq20_e314_q: f64 = eq20_e313;
        let eq20_e316: f64 = (eq20_e313 * p.p1);
        let eq20_e316_d_n0: f64 = (eq20_e313_d_n0 * p.p1);
        let eq20_e316_d_n1: f64 = (eq20_e313_d_n1 * p.p1);
        let eq20_e316_q: f64 = (eq20_e314_q * p.p1);
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * (eq20_e316_d_n0),
            nodes[1],
            multiplicity * (eq20_e316_d_n1),
        );
        let eq23_e332: f64 = (var_xqtex + var_xqex);
        let eq23_e332_d_n0: f64 = (var_xqtex_dn0 + var_xqex_dn0);
        let eq23_e332_d_n1: f64 = (var_xqtex_dn1 + var_xqex_dn1);
        let eq23_e332_d_n3: f64 = (var_xqtex_dn3 + var_xqex_dn3);
        let eq23_e332_d_n4: f64 = (var_xqtex_dn4 + var_xqex_dn4);
        let eq23_e332_d_n5: f64 = (var_xqtex_dn5 + var_xqex_dn5);
        let eq23_e332_d_n6: f64 = (var_xqtex_dn6 + var_xqex_dn6);
        let eq23_e332_d_n7: f64 = (var_xqtex_dn7 + var_xqex_dn7);
        let eq23_e332_d_n8: f64 = (var_xqtex_dn8 + var_xqex_dn8);
        let eq23_e332_d_n9: f64 = (var_xqtex_dn9 + var_xqex_dn9);
        let eq23_e332_d_n10: f64 = (var_xqtex_dn10 + var_xqex_dn10);
        let eq23_e333: f64 = (p.p3 * eq23_e332);
        let eq23_e333_d_n0: f64 = (p.p3 * eq23_e332_d_n0);
        let eq23_e333_d_n1: f64 = (p.p3 * eq23_e332_d_n1);
        let eq23_e333_d_n3: f64 = (p.p3 * eq23_e332_d_n3);
        let eq23_e333_d_n4: f64 = (p.p3 * eq23_e332_d_n4);
        let eq23_e333_d_n5: f64 = (p.p3 * eq23_e332_d_n5);
        let eq23_e333_d_n6: f64 = (p.p3 * eq23_e332_d_n6);
        let eq23_e333_d_n7: f64 = (p.p3 * eq23_e332_d_n7);
        let eq23_e333_d_n8: f64 = (p.p3 * eq23_e332_d_n8);
        let eq23_e333_d_n9: f64 = (p.p3 * eq23_e332_d_n9);
        let eq23_e333_d_n10: f64 = (p.p3 * eq23_e332_d_n10);
        let eq23_e334_q: f64 = eq23_e333;
        let eq23_e336: f64 = (eq23_e333 * p.p1);
        let eq23_e336_d_n0: f64 = (eq23_e333_d_n0 * p.p1);
        let eq23_e336_d_n1: f64 = (eq23_e333_d_n1 * p.p1);
        let eq23_e336_d_n3: f64 = (eq23_e333_d_n3 * p.p1);
        let eq23_e336_d_n4: f64 = (eq23_e333_d_n4 * p.p1);
        let eq23_e336_d_n5: f64 = (eq23_e333_d_n5 * p.p1);
        let eq23_e336_d_n6: f64 = (eq23_e333_d_n6 * p.p1);
        let eq23_e336_d_n7: f64 = (eq23_e333_d_n7 * p.p1);
        let eq23_e336_d_n8: f64 = (eq23_e333_d_n8 * p.p1);
        let eq23_e336_d_n9: f64 = (eq23_e333_d_n9 * p.p1);
        let eq23_e336_d_n10: f64 = (eq23_e333_d_n10 * p.p1);
        let eq23_e336_q: f64 = (eq23_e334_q * p.p1);
        let eq23_reactive_node_derivatives: [f64; 12] = [eq23_e336_d_n0, eq23_e336_d_n1, 0.0, eq23_e336_d_n3, eq23_e336_d_n4, eq23_e336_d_n5, eq23_e336_d_n6, eq23_e336_d_n7, eq23_e336_d_n8, eq23_e336_d_n9, eq23_e336_d_n10, 0.0];
        let eq23_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[9]),
            nodes,
            &eq23_reactive_node_derivatives,
            branches,
            &eq23_reactive_branch_derivatives,
            multiplicity,
        );
        let eq25_e351: f64 = (var_qtex + var_qex);
        let eq25_e351_d_n0: f64 = (var_qtex_dn0 + var_qex_dn0);
        let eq25_e351_d_n1: f64 = (var_qtex_dn1 + var_qex_dn1);
        let eq25_e351_d_n3: f64 = (var_qtex_dn3 + var_qex_dn3);
        let eq25_e351_d_n4: f64 = (var_qtex_dn4 + var_qex_dn4);
        let eq25_e351_d_n5: f64 = (var_qtex_dn5 + var_qex_dn5);
        let eq25_e351_d_n6: f64 = (var_qtex_dn6 + var_qex_dn6);
        let eq25_e351_d_n7: f64 = (var_qtex_dn7 + var_qex_dn7);
        let eq25_e351_d_n8: f64 = (var_qtex_dn8 + var_qex_dn8);
        let eq25_e351_d_n9: f64 = (var_qtex_dn9 + var_qex_dn9);
        let eq25_e351_d_n10: f64 = (var_qtex_dn10 + var_qex_dn10);
        let eq25_e352: f64 = (p.p3 * eq25_e351);
        let eq25_e352_d_n0: f64 = (p.p3 * eq25_e351_d_n0);
        let eq25_e352_d_n1: f64 = (p.p3 * eq25_e351_d_n1);
        let eq25_e352_d_n3: f64 = (p.p3 * eq25_e351_d_n3);
        let eq25_e352_d_n4: f64 = (p.p3 * eq25_e351_d_n4);
        let eq25_e352_d_n5: f64 = (p.p3 * eq25_e351_d_n5);
        let eq25_e352_d_n6: f64 = (p.p3 * eq25_e351_d_n6);
        let eq25_e352_d_n7: f64 = (p.p3 * eq25_e351_d_n7);
        let eq25_e352_d_n8: f64 = (p.p3 * eq25_e351_d_n8);
        let eq25_e352_d_n9: f64 = (p.p3 * eq25_e351_d_n9);
        let eq25_e352_d_n10: f64 = (p.p3 * eq25_e351_d_n10);
        let eq25_e353_q: f64 = eq25_e352;
        let eq25_e355: f64 = (eq25_e352 * p.p1);
        let eq25_e355_d_n0: f64 = (eq25_e352_d_n0 * p.p1);
        let eq25_e355_d_n1: f64 = (eq25_e352_d_n1 * p.p1);
        let eq25_e355_d_n3: f64 = (eq25_e352_d_n3 * p.p1);
        let eq25_e355_d_n4: f64 = (eq25_e352_d_n4 * p.p1);
        let eq25_e355_d_n5: f64 = (eq25_e352_d_n5 * p.p1);
        let eq25_e355_d_n6: f64 = (eq25_e352_d_n6 * p.p1);
        let eq25_e355_d_n7: f64 = (eq25_e352_d_n7 * p.p1);
        let eq25_e355_d_n8: f64 = (eq25_e352_d_n8 * p.p1);
        let eq25_e355_d_n9: f64 = (eq25_e352_d_n9 * p.p1);
        let eq25_e355_d_n10: f64 = (eq25_e352_d_n10 * p.p1);
        let eq25_e355_q: f64 = (eq25_e353_q * p.p1);
        let eq25_reactive_node_derivatives: [f64; 12] = [eq25_e355_d_n0, eq25_e355_d_n1, 0.0, eq25_e355_d_n3, eq25_e355_d_n4, eq25_e355_d_n5, eq25_e355_d_n6, eq25_e355_d_n7, eq25_e355_d_n8, eq25_e355_d_n9, eq25_e355_d_n10, 0.0];
        let eq25_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[10]),
            nodes,
            &eq25_reactive_node_derivatives,
            branches,
            &eq25_reactive_branch_derivatives,
            multiplicity,
        );
        let eq32_e394_q: f64 = (nv11 - 0.0);
        let eq32_e395: f64 = (var_taun * (nv11 - 0.0));
        let eq32_e395_d_n0: f64 = (var_taun_dn0 * (nv11 - 0.0));
        let eq32_e395_d_n1: f64 = (var_taun_dn1 * (nv11 - 0.0));
        let eq32_e395_d_n3: f64 = (var_taun_dn3 * (nv11 - 0.0));
        let eq32_e395_d_n4: f64 = (var_taun_dn4 * (nv11 - 0.0));
        let eq32_e395_d_n5: f64 = (var_taun_dn5 * (nv11 - 0.0));
        let eq32_e395_d_n6: f64 = (var_taun_dn6 * (nv11 - 0.0));
        let eq32_e395_d_n7: f64 = (var_taun_dn7 * (nv11 - 0.0));
        let eq32_e395_d_n8: f64 = (var_taun_dn8 * (nv11 - 0.0));
        let eq32_e395_d_n9: f64 = (var_taun_dn9 * (nv11 - 0.0));
        let eq32_e395_d_n10: f64 = (var_taun_dn10 * (nv11 - 0.0));
        let eq32_e395_q: f64 = (var_taun * eq32_e394_q);
        let eq32_e395_q_d_n0: f64 = (var_taun_dn0 * eq32_e394_q);
        let eq32_e395_q_d_n1: f64 = (var_taun_dn1 * eq32_e394_q);
        let eq32_e395_q_d_n3: f64 = (var_taun_dn3 * eq32_e394_q);
        let eq32_e395_q_d_n4: f64 = (var_taun_dn4 * eq32_e394_q);
        let eq32_e395_q_d_n5: f64 = (var_taun_dn5 * eq32_e394_q);
        let eq32_e395_q_d_n6: f64 = (var_taun_dn6 * eq32_e394_q);
        let eq32_e395_q_d_n7: f64 = (var_taun_dn7 * eq32_e394_q);
        let eq32_e395_q_d_n8: f64 = (var_taun_dn8 * eq32_e394_q);
        let eq32_e395_q_d_n9: f64 = (var_taun_dn9 * eq32_e394_q);
        let eq32_e395_q_d_n10: f64 = (var_taun_dn10 * eq32_e394_q);
        let eq32_reactive_node_derivatives: [f64; 12] = [eq32_e395_q_d_n0, eq32_e395_q_d_n1, 0.0, eq32_e395_q_d_n3, eq32_e395_q_d_n4, eq32_e395_q_d_n5, eq32_e395_q_d_n6, eq32_e395_q_d_n7, eq32_e395_q_d_n8, eq32_e395_q_d_n9, eq32_e395_q_d_n10, var_taun];
        let eq32_reactive_branch_derivatives: [f64; 2] = [0.0, 0.0];
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
