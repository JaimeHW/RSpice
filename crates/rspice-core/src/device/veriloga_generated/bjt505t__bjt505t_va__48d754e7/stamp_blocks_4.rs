#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_42(
        p: &Parameters,
        var_evbc3: f64,
        var_evbc3_db0: f64,
        var_evbc3_db1: f64,
        var_evbc3_dn0: f64,
        var_evbc3_dn1: f64,
        var_evbc3_dn10: f64,
        var_evbc3_dn11: f64,
        var_evbc3_dn12: f64,
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
        var_evbc3vdc_dn12: f64,
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
        var_fex_dn12: f64,
        var_fex_dn2: f64,
        var_fex_dn3: f64,
        var_fex_dn4: f64,
        var_fex_dn5: f64,
        var_fex_dn6: f64,
        var_fex_dn7: f64,
        var_fex_dn8: f64,
        var_fex_dn9: f64,
        var_guard120: f64,
        var_ibx_t: f64,
        var_ibx_t_db0: f64,
        var_ibx_t_db1: f64,
        var_ibx_t_dn0: f64,
        var_ibx_t_dn1: f64,
        var_ibx_t_dn10: f64,
        var_ibx_t_dn11: f64,
        var_ibx_t_dn12: f64,
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
        var_if0_dn11: f64,
        var_if0_dn12: f64,
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
        var_qb0_dn11: f64,
        var_qb0_dn12: f64,
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
        var_qepi0_dn11: f64,
        var_qepi0_dn12: f64,
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
        var_taub_t_dn11: f64,
        var_taub_t_dn12: f64,
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
        var_tauex_t_dn11: f64,
        var_tauex_t_dn12: f64,
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
        var_taur_t_dn11: f64,
        var_taur_t_dn12: f64,
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
        var_tepi_t_dn11: f64,
        var_tepi_t_dn12: f64,
        var_tepi_t_dn2: f64,
        var_tepi_t_dn3: f64,
        var_tepi_t_dn4: f64,
        var_tepi_t_dn5: f64,
        var_tepi_t_dn6: f64,
        var_tepi_t_dn7: f64,
        var_tepi_t_dn8: f64,
        var_tepi_t_dn9: f64,
        var_vbc3: f64,
        var_vbc3_db0: f64,
        var_vbc3_db1: f64,
        var_vbc3_dn0: f64,
        var_vbc3_dn1: f64,
        var_vbc3_dn10: f64,
        var_vbc3_dn11: f64,
        var_vbc3_dn12: f64,
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
        var_vdcex_t_dn12: f64,
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
        var_vtinv_dn11: f64,
        var_vtinv_dn12: f64,
        var_vtinv_dn2: f64,
        var_vtinv_dn3: f64,
        var_vtinv_dn4: f64,
        var_vtinv_dn5: f64,
        var_vtinv_dn6: f64,
        var_vtinv_dn7: f64,
        var_vtinv_dn8: f64,
        var_vtinv_dn9: f64,
        var_evbc3vdcex_slot: &mut f64,
        var_evbc3vdcex_db0_slot: &mut f64,
        var_evbc3vdcex_db1_slot: &mut f64,
        var_evbc3vdcex_dn0_slot: &mut f64,
        var_evbc3vdcex_dn1_slot: &mut f64,
        var_evbc3vdcex_dn10_slot: &mut f64,
        var_evbc3vdcex_dn11_slot: &mut f64,
        var_evbc3vdcex_dn12_slot: &mut f64,
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
        var_evbc3vdcex_rdn11_slot: &mut f64,
        var_evbc3vdcex_rdn12_slot: &mut f64,
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
        var_expl_dn11_slot: &mut f64,
        var_expl_dn12_slot: &mut f64,
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
        var_expl_rdn11_slot: &mut f64,
        var_expl_rdn12_slot: &mut f64,
        var_expl_rdn2_slot: &mut f64,
        var_expl_rdn3_slot: &mut f64,
        var_expl_rdn4_slot: &mut f64,
        var_expl_rdn5_slot: &mut f64,
        var_expl_rdn6_slot: &mut f64,
        var_expl_rdn7_slot: &mut f64,
        var_expl_rdn8_slot: &mut f64,
        var_expl_rdn9_slot: &mut f64,
        var_expl_rv_slot: &mut f64,
        var_guard121_slot: &mut f64,
        var_guard121_db0_slot: &mut f64,
        var_guard121_db1_slot: &mut f64,
        var_guard121_dn0_slot: &mut f64,
        var_guard121_dn1_slot: &mut f64,
        var_guard121_dn10_slot: &mut f64,
        var_guard121_dn11_slot: &mut f64,
        var_guard121_dn12_slot: &mut f64,
        var_guard121_dn2_slot: &mut f64,
        var_guard121_dn3_slot: &mut f64,
        var_guard121_dn4_slot: &mut f64,
        var_guard121_dn5_slot: &mut f64,
        var_guard121_dn6_slot: &mut f64,
        var_guard121_dn7_slot: &mut f64,
        var_guard121_dn8_slot: &mut f64,
        var_guard121_dn9_slot: &mut f64,
        var_guard121_rdb0_slot: &mut f64,
        var_guard121_rdb1_slot: &mut f64,
        var_guard121_rdn0_slot: &mut f64,
        var_guard121_rdn1_slot: &mut f64,
        var_guard121_rdn10_slot: &mut f64,
        var_guard121_rdn11_slot: &mut f64,
        var_guard121_rdn12_slot: &mut f64,
        var_guard121_rdn2_slot: &mut f64,
        var_guard121_rdn3_slot: &mut f64,
        var_guard121_rdn4_slot: &mut f64,
        var_guard121_rdn5_slot: &mut f64,
        var_guard121_rdn6_slot: &mut f64,
        var_guard121_rdn7_slot: &mut f64,
        var_guard121_rdn8_slot: &mut f64,
        var_guard121_rdn9_slot: &mut f64,
        var_guard121_rv_slot: &mut f64,
        var_guard122_slot: &mut f64,
        var_guard122_db0_slot: &mut f64,
        var_guard122_db1_slot: &mut f64,
        var_guard122_dn0_slot: &mut f64,
        var_guard122_dn1_slot: &mut f64,
        var_guard122_dn10_slot: &mut f64,
        var_guard122_dn11_slot: &mut f64,
        var_guard122_dn12_slot: &mut f64,
        var_guard122_dn2_slot: &mut f64,
        var_guard122_dn3_slot: &mut f64,
        var_guard122_dn4_slot: &mut f64,
        var_guard122_dn5_slot: &mut f64,
        var_guard122_dn6_slot: &mut f64,
        var_guard122_dn7_slot: &mut f64,
        var_guard122_dn8_slot: &mut f64,
        var_guard122_dn9_slot: &mut f64,
        var_guard122_rdb0_slot: &mut f64,
        var_guard122_rdb1_slot: &mut f64,
        var_guard122_rdn0_slot: &mut f64,
        var_guard122_rdn1_slot: &mut f64,
        var_guard122_rdn10_slot: &mut f64,
        var_guard122_rdn11_slot: &mut f64,
        var_guard122_rdn12_slot: &mut f64,
        var_guard122_rdn2_slot: &mut f64,
        var_guard122_rdn3_slot: &mut f64,
        var_guard122_rdn4_slot: &mut f64,
        var_guard122_rdn5_slot: &mut f64,
        var_guard122_rdn6_slot: &mut f64,
        var_guard122_rdn7_slot: &mut f64,
        var_guard122_rdn8_slot: &mut f64,
        var_guard122_rdn9_slot: &mut f64,
        var_guard122_rv_slot: &mut f64,
        var_xg1_slot: &mut f64,
        var_xg1_db0_slot: &mut f64,
        var_xg1_db1_slot: &mut f64,
        var_xg1_dn0_slot: &mut f64,
        var_xg1_dn1_slot: &mut f64,
        var_xg1_dn10_slot: &mut f64,
        var_xg1_dn11_slot: &mut f64,
        var_xg1_dn12_slot: &mut f64,
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
        var_xg1_rdn11_slot: &mut f64,
        var_xg1_rdn12_slot: &mut f64,
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
        var_xg2_dn11_slot: &mut f64,
        var_xg2_dn12_slot: &mut f64,
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
        var_xg2_rdn11_slot: &mut f64,
        var_xg2_rdn12_slot: &mut f64,
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
        var_xnbex_dn11_slot: &mut f64,
        var_xnbex_dn12_slot: &mut f64,
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
        var_xnbex_rdn11_slot: &mut f64,
        var_xnbex_rdn12_slot: &mut f64,
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
        var_xpwex_dn11_slot: &mut f64,
        var_xpwex_dn12_slot: &mut f64,
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
        var_xpwex_rdn11_slot: &mut f64,
        var_xpwex_rdn12_slot: &mut f64,
        var_xpwex_rdn2_slot: &mut f64,
        var_xpwex_rdn3_slot: &mut f64,
        var_xpwex_rdn4_slot: &mut f64,
        var_xpwex_rdn5_slot: &mut f64,
        var_xpwex_rdn6_slot: &mut f64,
        var_xpwex_rdn7_slot: &mut f64,
        var_xpwex_rdn8_slot: &mut f64,
        var_xpwex_rdn9_slot: &mut f64,
        var_xpwex_rv_slot: &mut f64,
        var_xqex_slot: &mut f64,
        var_xqex_db0_slot: &mut f64,
        var_xqex_db1_slot: &mut f64,
        var_xqex_dn0_slot: &mut f64,
        var_xqex_dn1_slot: &mut f64,
        var_xqex_dn10_slot: &mut f64,
        var_xqex_dn11_slot: &mut f64,
        var_xqex_dn12_slot: &mut f64,
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
        var_xqex_rdn11_slot: &mut f64,
        var_xqex_rdn12_slot: &mut f64,
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
        var_xqmex_dn11_slot: &mut f64,
        var_xqmex_dn12_slot: &mut f64,
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
        var_xqmex_rdn11_slot: &mut f64,
        var_xqmex_rdn12_slot: &mut f64,
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
        let mut var_evbc3vdcex: f64 = *var_evbc3vdcex_slot;
        let mut var_evbc3vdcex_db0: f64 = *var_evbc3vdcex_db0_slot;
        let mut var_evbc3vdcex_db1: f64 = *var_evbc3vdcex_db1_slot;
        let mut var_evbc3vdcex_dn0: f64 = *var_evbc3vdcex_dn0_slot;
        let mut var_evbc3vdcex_dn1: f64 = *var_evbc3vdcex_dn1_slot;
        let mut var_evbc3vdcex_dn10: f64 = *var_evbc3vdcex_dn10_slot;
        let mut var_evbc3vdcex_dn11: f64 = *var_evbc3vdcex_dn11_slot;
        let mut var_evbc3vdcex_dn12: f64 = *var_evbc3vdcex_dn12_slot;
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
        let mut var_evbc3vdcex_rdn11: f64 = *var_evbc3vdcex_rdn11_slot;
        let mut var_evbc3vdcex_rdn12: f64 = *var_evbc3vdcex_rdn12_slot;
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
        let mut var_expl_dn11: f64 = *var_expl_dn11_slot;
        let mut var_expl_dn12: f64 = *var_expl_dn12_slot;
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
        let mut var_expl_rdn11: f64 = *var_expl_rdn11_slot;
        let mut var_expl_rdn12: f64 = *var_expl_rdn12_slot;
        let mut var_expl_rdn2: f64 = *var_expl_rdn2_slot;
        let mut var_expl_rdn3: f64 = *var_expl_rdn3_slot;
        let mut var_expl_rdn4: f64 = *var_expl_rdn4_slot;
        let mut var_expl_rdn5: f64 = *var_expl_rdn5_slot;
        let mut var_expl_rdn6: f64 = *var_expl_rdn6_slot;
        let mut var_expl_rdn7: f64 = *var_expl_rdn7_slot;
        let mut var_expl_rdn8: f64 = *var_expl_rdn8_slot;
        let mut var_expl_rdn9: f64 = *var_expl_rdn9_slot;
        let mut var_expl_rv: f64 = *var_expl_rv_slot;
        let mut var_guard121: f64 = *var_guard121_slot;
        let mut var_guard121_db0: f64 = *var_guard121_db0_slot;
        let mut var_guard121_db1: f64 = *var_guard121_db1_slot;
        let mut var_guard121_dn0: f64 = *var_guard121_dn0_slot;
        let mut var_guard121_dn1: f64 = *var_guard121_dn1_slot;
        let mut var_guard121_dn10: f64 = *var_guard121_dn10_slot;
        let mut var_guard121_dn11: f64 = *var_guard121_dn11_slot;
        let mut var_guard121_dn12: f64 = *var_guard121_dn12_slot;
        let mut var_guard121_dn2: f64 = *var_guard121_dn2_slot;
        let mut var_guard121_dn3: f64 = *var_guard121_dn3_slot;
        let mut var_guard121_dn4: f64 = *var_guard121_dn4_slot;
        let mut var_guard121_dn5: f64 = *var_guard121_dn5_slot;
        let mut var_guard121_dn6: f64 = *var_guard121_dn6_slot;
        let mut var_guard121_dn7: f64 = *var_guard121_dn7_slot;
        let mut var_guard121_dn8: f64 = *var_guard121_dn8_slot;
        let mut var_guard121_dn9: f64 = *var_guard121_dn9_slot;
        let mut var_guard121_rdb0: f64 = *var_guard121_rdb0_slot;
        let mut var_guard121_rdb1: f64 = *var_guard121_rdb1_slot;
        let mut var_guard121_rdn0: f64 = *var_guard121_rdn0_slot;
        let mut var_guard121_rdn1: f64 = *var_guard121_rdn1_slot;
        let mut var_guard121_rdn10: f64 = *var_guard121_rdn10_slot;
        let mut var_guard121_rdn11: f64 = *var_guard121_rdn11_slot;
        let mut var_guard121_rdn12: f64 = *var_guard121_rdn12_slot;
        let mut var_guard121_rdn2: f64 = *var_guard121_rdn2_slot;
        let mut var_guard121_rdn3: f64 = *var_guard121_rdn3_slot;
        let mut var_guard121_rdn4: f64 = *var_guard121_rdn4_slot;
        let mut var_guard121_rdn5: f64 = *var_guard121_rdn5_slot;
        let mut var_guard121_rdn6: f64 = *var_guard121_rdn6_slot;
        let mut var_guard121_rdn7: f64 = *var_guard121_rdn7_slot;
        let mut var_guard121_rdn8: f64 = *var_guard121_rdn8_slot;
        let mut var_guard121_rdn9: f64 = *var_guard121_rdn9_slot;
        let mut var_guard121_rv: f64 = *var_guard121_rv_slot;
        let mut var_guard122: f64 = *var_guard122_slot;
        let mut var_guard122_db0: f64 = *var_guard122_db0_slot;
        let mut var_guard122_db1: f64 = *var_guard122_db1_slot;
        let mut var_guard122_dn0: f64 = *var_guard122_dn0_slot;
        let mut var_guard122_dn1: f64 = *var_guard122_dn1_slot;
        let mut var_guard122_dn10: f64 = *var_guard122_dn10_slot;
        let mut var_guard122_dn11: f64 = *var_guard122_dn11_slot;
        let mut var_guard122_dn12: f64 = *var_guard122_dn12_slot;
        let mut var_guard122_dn2: f64 = *var_guard122_dn2_slot;
        let mut var_guard122_dn3: f64 = *var_guard122_dn3_slot;
        let mut var_guard122_dn4: f64 = *var_guard122_dn4_slot;
        let mut var_guard122_dn5: f64 = *var_guard122_dn5_slot;
        let mut var_guard122_dn6: f64 = *var_guard122_dn6_slot;
        let mut var_guard122_dn7: f64 = *var_guard122_dn7_slot;
        let mut var_guard122_dn8: f64 = *var_guard122_dn8_slot;
        let mut var_guard122_dn9: f64 = *var_guard122_dn9_slot;
        let mut var_guard122_rdb0: f64 = *var_guard122_rdb0_slot;
        let mut var_guard122_rdb1: f64 = *var_guard122_rdb1_slot;
        let mut var_guard122_rdn0: f64 = *var_guard122_rdn0_slot;
        let mut var_guard122_rdn1: f64 = *var_guard122_rdn1_slot;
        let mut var_guard122_rdn10: f64 = *var_guard122_rdn10_slot;
        let mut var_guard122_rdn11: f64 = *var_guard122_rdn11_slot;
        let mut var_guard122_rdn12: f64 = *var_guard122_rdn12_slot;
        let mut var_guard122_rdn2: f64 = *var_guard122_rdn2_slot;
        let mut var_guard122_rdn3: f64 = *var_guard122_rdn3_slot;
        let mut var_guard122_rdn4: f64 = *var_guard122_rdn4_slot;
        let mut var_guard122_rdn5: f64 = *var_guard122_rdn5_slot;
        let mut var_guard122_rdn6: f64 = *var_guard122_rdn6_slot;
        let mut var_guard122_rdn7: f64 = *var_guard122_rdn7_slot;
        let mut var_guard122_rdn8: f64 = *var_guard122_rdn8_slot;
        let mut var_guard122_rdn9: f64 = *var_guard122_rdn9_slot;
        let mut var_guard122_rv: f64 = *var_guard122_rv_slot;
        let mut var_xg1: f64 = *var_xg1_slot;
        let mut var_xg1_db0: f64 = *var_xg1_db0_slot;
        let mut var_xg1_db1: f64 = *var_xg1_db1_slot;
        let mut var_xg1_dn0: f64 = *var_xg1_dn0_slot;
        let mut var_xg1_dn1: f64 = *var_xg1_dn1_slot;
        let mut var_xg1_dn10: f64 = *var_xg1_dn10_slot;
        let mut var_xg1_dn11: f64 = *var_xg1_dn11_slot;
        let mut var_xg1_dn12: f64 = *var_xg1_dn12_slot;
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
        let mut var_xg1_rdn11: f64 = *var_xg1_rdn11_slot;
        let mut var_xg1_rdn12: f64 = *var_xg1_rdn12_slot;
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
        let mut var_xg2_dn11: f64 = *var_xg2_dn11_slot;
        let mut var_xg2_dn12: f64 = *var_xg2_dn12_slot;
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
        let mut var_xg2_rdn11: f64 = *var_xg2_rdn11_slot;
        let mut var_xg2_rdn12: f64 = *var_xg2_rdn12_slot;
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
        let mut var_xnbex_dn11: f64 = *var_xnbex_dn11_slot;
        let mut var_xnbex_dn12: f64 = *var_xnbex_dn12_slot;
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
        let mut var_xnbex_rdn11: f64 = *var_xnbex_rdn11_slot;
        let mut var_xnbex_rdn12: f64 = *var_xnbex_rdn12_slot;
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
        let mut var_xpwex_dn11: f64 = *var_xpwex_dn11_slot;
        let mut var_xpwex_dn12: f64 = *var_xpwex_dn12_slot;
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
        let mut var_xpwex_rdn11: f64 = *var_xpwex_rdn11_slot;
        let mut var_xpwex_rdn12: f64 = *var_xpwex_rdn12_slot;
        let mut var_xpwex_rdn2: f64 = *var_xpwex_rdn2_slot;
        let mut var_xpwex_rdn3: f64 = *var_xpwex_rdn3_slot;
        let mut var_xpwex_rdn4: f64 = *var_xpwex_rdn4_slot;
        let mut var_xpwex_rdn5: f64 = *var_xpwex_rdn5_slot;
        let mut var_xpwex_rdn6: f64 = *var_xpwex_rdn6_slot;
        let mut var_xpwex_rdn7: f64 = *var_xpwex_rdn7_slot;
        let mut var_xpwex_rdn8: f64 = *var_xpwex_rdn8_slot;
        let mut var_xpwex_rdn9: f64 = *var_xpwex_rdn9_slot;
        let mut var_xpwex_rv: f64 = *var_xpwex_rv_slot;
        let mut var_xqex: f64 = *var_xqex_slot;
        let mut var_xqex_db0: f64 = *var_xqex_db0_slot;
        let mut var_xqex_db1: f64 = *var_xqex_db1_slot;
        let mut var_xqex_dn0: f64 = *var_xqex_dn0_slot;
        let mut var_xqex_dn1: f64 = *var_xqex_dn1_slot;
        let mut var_xqex_dn10: f64 = *var_xqex_dn10_slot;
        let mut var_xqex_dn11: f64 = *var_xqex_dn11_slot;
        let mut var_xqex_dn12: f64 = *var_xqex_dn12_slot;
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
        let mut var_xqex_rdn11: f64 = *var_xqex_rdn11_slot;
        let mut var_xqex_rdn12: f64 = *var_xqex_rdn12_slot;
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
        let mut var_xqmex_dn11: f64 = *var_xqmex_dn11_slot;
        let mut var_xqmex_dn12: f64 = *var_xqmex_dn12_slot;
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
        let mut var_xqmex_rdn11: f64 = *var_xqmex_rdn11_slot;
        let mut var_xqmex_rdn12: f64 = *var_xqmex_rdn12_slot;
        let mut var_xqmex_rdn2: f64 = *var_xqmex_rdn2_slot;
        let mut var_xqmex_rdn3: f64 = *var_xqmex_rdn3_slot;
        let mut var_xqmex_rdn4: f64 = *var_xqmex_rdn4_slot;
        let mut var_xqmex_rdn5: f64 = *var_xqmex_rdn5_slot;
        let mut var_xqmex_rdn6: f64 = *var_xqmex_rdn6_slot;
        let mut var_xqmex_rdn7: f64 = *var_xqmex_rdn7_slot;
        let mut var_xqmex_rdn8: f64 = *var_xqmex_rdn8_slot;
        let mut var_xqmex_rdn9: f64 = *var_xqmex_rdn9_slot;
        let mut var_xqmex_rv: f64 = *var_xqmex_rv_slot;

        let assign6590_e6855: f64 = if p.p79 == 0.0 { 1.0 } else { 0.0 };
        var_guard121 = assign6590_e6855;
        var_guard121_dn0 = 0.0;
        var_guard121_dn1 = 0.0;
        var_guard121_dn2 = 0.0;
        var_guard121_dn3 = 0.0;
        var_guard121_dn4 = 0.0;
        var_guard121_dn5 = 0.0;
        var_guard121_dn6 = 0.0;
        var_guard121_dn7 = 0.0;
        var_guard121_dn8 = 0.0;
        var_guard121_dn9 = 0.0;
        var_guard121_dn10 = 0.0;
        var_guard121_dn11 = 0.0;
        var_guard121_dn12 = 0.0;
        var_guard121_db0 = 0.0;
        var_guard121_db1 = 0.0;
        var_guard121_rv = 0.0;
        var_guard121_rdn0 = 0.0;
        var_guard121_rdn1 = 0.0;
        var_guard121_rdn2 = 0.0;
        var_guard121_rdn3 = 0.0;
        var_guard121_rdn4 = 0.0;
        var_guard121_rdn5 = 0.0;
        var_guard121_rdn6 = 0.0;
        var_guard121_rdn7 = 0.0;
        var_guard121_rdn8 = 0.0;
        var_guard121_rdn9 = 0.0;
        var_guard121_rdn10 = 0.0;
        var_guard121_rdn11 = 0.0;
        var_guard121_rdn12 = 0.0;
        var_guard121_rdb0 = 0.0;
        var_guard121_rdb1 = 0.0;

        let (assign6600_e6863, assign6600_e6863_d_n0, assign6600_e6863_d_n1, assign6600_e6863_d_n2, assign6600_e6863_d_n3, assign6600_e6863_d_n4, assign6600_e6863_d_n5, assign6600_e6863_d_n6, assign6600_e6863_d_n7, assign6600_e6863_d_n8, assign6600_e6863_d_n9, assign6600_e6863_d_n10, assign6600_e6863_d_n11, assign6600_e6863_d_n12, assign6600_e6863_d_b0, assign6600_e6863_d_b1,) = {
    if ((var_guard120 != 0.0) && (var_guard121 != 0.0)) {
        let assign6600_e6861: f64 = (var_if0 * var_evbc3);
        (assign6600_e6861, ((var_if0_dn0 * var_evbc3) + (var_if0 * var_evbc3_dn0)), ((var_if0_dn1 * var_evbc3) + (var_if0 * var_evbc3_dn1)), ((var_if0_dn2 * var_evbc3) + (var_if0 * var_evbc3_dn2)), ((var_if0_dn3 * var_evbc3) + (var_if0 * var_evbc3_dn3)), ((var_if0_dn4 * var_evbc3) + (var_if0 * var_evbc3_dn4)), ((var_if0_dn5 * var_evbc3) + (var_if0 * var_evbc3_dn5)), ((var_if0_dn6 * var_evbc3) + (var_if0 * var_evbc3_dn6)), ((var_if0_dn7 * var_evbc3) + (var_if0 * var_evbc3_dn7)), ((var_if0_dn8 * var_evbc3) + (var_if0 * var_evbc3_dn8)), ((var_if0_dn9 * var_evbc3) + (var_if0 * var_evbc3_dn9)), ((var_if0_dn10 * var_evbc3) + (var_if0 * var_evbc3_dn10)), ((var_if0_dn11 * var_evbc3) + (var_if0 * var_evbc3_dn11)), ((var_if0_dn12 * var_evbc3) + (var_if0 * var_evbc3_dn12)), ((var_if0_db0 * var_evbc3) + (var_if0 * var_evbc3_db0)), ((var_if0_db1 * var_evbc3) + (var_if0 * var_evbc3_db1)),)
    } else {
        (var_xg1, var_xg1_dn0, var_xg1_dn1, var_xg1_dn2, var_xg1_dn3, var_xg1_dn4, var_xg1_dn5, var_xg1_dn6, var_xg1_dn7, var_xg1_dn8, var_xg1_dn9, var_xg1_dn10, var_xg1_dn11, var_xg1_dn12, var_xg1_db0, var_xg1_db1,)
    }
};
        var_xg1 = assign6600_e6863;
        var_xg1_dn0 = assign6600_e6863_d_n0;
        var_xg1_dn1 = assign6600_e6863_d_n1;
        var_xg1_dn2 = assign6600_e6863_d_n2;
        var_xg1_dn3 = assign6600_e6863_d_n3;
        var_xg1_dn4 = assign6600_e6863_d_n4;
        var_xg1_dn5 = assign6600_e6863_d_n5;
        var_xg1_dn6 = assign6600_e6863_d_n6;
        var_xg1_dn7 = assign6600_e6863_d_n7;
        var_xg1_dn8 = assign6600_e6863_d_n8;
        var_xg1_dn9 = assign6600_e6863_d_n9;
        var_xg1_dn10 = assign6600_e6863_d_n10;
        var_xg1_dn11 = assign6600_e6863_d_n11;
        var_xg1_dn12 = assign6600_e6863_d_n12;
        var_xg1_db0 = assign6600_e6863_d_b0;
        var_xg1_db1 = assign6600_e6863_d_b1;
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
        var_xg1_rdn11 = 0.0;
        var_xg1_rdn12 = 0.0;
        var_xg1_rdb0 = 0.0;
        var_xg1_rdb1 = 0.0;

        let (assign6610_e6878, assign6610_e6878_d_n0, assign6610_e6878_d_n1, assign6610_e6878_d_n2, assign6610_e6878_d_n3, assign6610_e6878_d_n4, assign6610_e6878_d_n5, assign6610_e6878_d_n6, assign6610_e6878_d_n7, assign6610_e6878_d_n8, assign6610_e6878_d_n9, assign6610_e6878_d_n10, assign6610_e6878_d_n11, assign6610_e6878_d_n12, assign6610_e6878_d_b0, assign6610_e6878_d_b1,) = {
    if ((var_guard120 != 0.0) && (var_guard121 != 0.0)) {
        let assign6610_e6869: f64 = (var_xg1 - var_if0);
        let assign6610_e6873: f64 = (1.0 + var_xg1);
        let assign6610_e6874: f64 = (assign6610_e6873).sqrt();
        let assign6610_e6875: f64 = (1.0 + assign6610_e6874);
        let assign6610_e6876: f64 = (assign6610_e6869 / assign6610_e6875);
        (assign6610_e6876, ((((var_xg1_dn0 - var_if0_dn0) * assign6610_e6875) - (assign6610_e6869 * (var_xg1_dn0 / (2.0 * assign6610_e6874)))) / (assign6610_e6875 * assign6610_e6875)), ((((var_xg1_dn1 - var_if0_dn1) * assign6610_e6875) - (assign6610_e6869 * (var_xg1_dn1 / (2.0 * assign6610_e6874)))) / (assign6610_e6875 * assign6610_e6875)), ((((var_xg1_dn2 - var_if0_dn2) * assign6610_e6875) - (assign6610_e6869 * (var_xg1_dn2 / (2.0 * assign6610_e6874)))) / (assign6610_e6875 * assign6610_e6875)), ((((var_xg1_dn3 - var_if0_dn3) * assign6610_e6875) - (assign6610_e6869 * (var_xg1_dn3 / (2.0 * assign6610_e6874)))) / (assign6610_e6875 * assign6610_e6875)), ((((var_xg1_dn4 - var_if0_dn4) * assign6610_e6875) - (assign6610_e6869 * (var_xg1_dn4 / (2.0 * assign6610_e6874)))) / (assign6610_e6875 * assign6610_e6875)), ((((var_xg1_dn5 - var_if0_dn5) * assign6610_e6875) - (assign6610_e6869 * (var_xg1_dn5 / (2.0 * assign6610_e6874)))) / (assign6610_e6875 * assign6610_e6875)), ((((var_xg1_dn6 - var_if0_dn6) * assign6610_e6875) - (assign6610_e6869 * (var_xg1_dn6 / (2.0 * assign6610_e6874)))) / (assign6610_e6875 * assign6610_e6875)), ((((var_xg1_dn7 - var_if0_dn7) * assign6610_e6875) - (assign6610_e6869 * (var_xg1_dn7 / (2.0 * assign6610_e6874)))) / (assign6610_e6875 * assign6610_e6875)), ((((var_xg1_dn8 - var_if0_dn8) * assign6610_e6875) - (assign6610_e6869 * (var_xg1_dn8 / (2.0 * assign6610_e6874)))) / (assign6610_e6875 * assign6610_e6875)), ((((var_xg1_dn9 - var_if0_dn9) * assign6610_e6875) - (assign6610_e6869 * (var_xg1_dn9 / (2.0 * assign6610_e6874)))) / (assign6610_e6875 * assign6610_e6875)), ((((var_xg1_dn10 - var_if0_dn10) * assign6610_e6875) - (assign6610_e6869 * (var_xg1_dn10 / (2.0 * assign6610_e6874)))) / (assign6610_e6875 * assign6610_e6875)), ((((var_xg1_dn11 - var_if0_dn11) * assign6610_e6875) - (assign6610_e6869 * (var_xg1_dn11 / (2.0 * assign6610_e6874)))) / (assign6610_e6875 * assign6610_e6875)), ((((var_xg1_dn12 - var_if0_dn12) * assign6610_e6875) - (assign6610_e6869 * (var_xg1_dn12 / (2.0 * assign6610_e6874)))) / (assign6610_e6875 * assign6610_e6875)), ((((var_xg1_db0 - var_if0_db0) * assign6610_e6875) - (assign6610_e6869 * (var_xg1_db0 / (2.0 * assign6610_e6874)))) / (assign6610_e6875 * assign6610_e6875)), ((((var_xg1_db1 - var_if0_db1) * assign6610_e6875) - (assign6610_e6869 * (var_xg1_db1 / (2.0 * assign6610_e6874)))) / (assign6610_e6875 * assign6610_e6875)),)
    } else {
        (var_xnbex, var_xnbex_dn0, var_xnbex_dn1, var_xnbex_dn2, var_xnbex_dn3, var_xnbex_dn4, var_xnbex_dn5, var_xnbex_dn6, var_xnbex_dn7, var_xnbex_dn8, var_xnbex_dn9, var_xnbex_dn10, var_xnbex_dn11, var_xnbex_dn12, var_xnbex_db0, var_xnbex_db1,)
    }
};
        var_xnbex = assign6610_e6878;
        var_xnbex_dn0 = assign6610_e6878_d_n0;
        var_xnbex_dn1 = assign6610_e6878_d_n1;
        var_xnbex_dn2 = assign6610_e6878_d_n2;
        var_xnbex_dn3 = assign6610_e6878_d_n3;
        var_xnbex_dn4 = assign6610_e6878_d_n4;
        var_xnbex_dn5 = assign6610_e6878_d_n5;
        var_xnbex_dn6 = assign6610_e6878_d_n6;
        var_xnbex_dn7 = assign6610_e6878_d_n7;
        var_xnbex_dn8 = assign6610_e6878_d_n8;
        var_xnbex_dn9 = assign6610_e6878_d_n9;
        var_xnbex_dn10 = assign6610_e6878_d_n10;
        var_xnbex_dn11 = assign6610_e6878_d_n11;
        var_xnbex_dn12 = assign6610_e6878_d_n12;
        var_xnbex_db0 = assign6610_e6878_d_b0;
        var_xnbex_db1 = assign6610_e6878_d_b1;
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
        var_xnbex_rdn11 = 0.0;
        var_xnbex_rdn12 = 0.0;
        var_xnbex_rdb0 = 0.0;
        var_xnbex_rdb1 = 0.0;

        let (assign6620_e6886, assign6620_e6886_d_n0, assign6620_e6886_d_n1, assign6620_e6886_d_n2, assign6620_e6886_d_n3, assign6620_e6886_d_n4, assign6620_e6886_d_n5, assign6620_e6886_d_n6, assign6620_e6886_d_n7, assign6620_e6886_d_n8, assign6620_e6886_d_n9, assign6620_e6886_d_n10, assign6620_e6886_d_n11, assign6620_e6886_d_n12, assign6620_e6886_d_b0, assign6620_e6886_d_b1,) = {
    if ((var_guard120 != 0.0) && (var_guard121 != 0.0)) {
        let assign6620_e6884: f64 = (4.0 * var_evbc3vdc);
        (assign6620_e6884, (4.0 * var_evbc3vdc_dn0), (4.0 * var_evbc3vdc_dn1), (4.0 * var_evbc3vdc_dn2), (4.0 * var_evbc3vdc_dn3), (4.0 * var_evbc3vdc_dn4), (4.0 * var_evbc3vdc_dn5), (4.0 * var_evbc3vdc_dn6), (4.0 * var_evbc3vdc_dn7), (4.0 * var_evbc3vdc_dn8), (4.0 * var_evbc3vdc_dn9), (4.0 * var_evbc3vdc_dn10), (4.0 * var_evbc3vdc_dn11), (4.0 * var_evbc3vdc_dn12), (4.0 * var_evbc3vdc_db0), (4.0 * var_evbc3vdc_db1),)
    } else {
        (var_xg2, var_xg2_dn0, var_xg2_dn1, var_xg2_dn2, var_xg2_dn3, var_xg2_dn4, var_xg2_dn5, var_xg2_dn6, var_xg2_dn7, var_xg2_dn8, var_xg2_dn9, var_xg2_dn10, var_xg2_dn11, var_xg2_dn12, var_xg2_db0, var_xg2_db1,)
    }
};
        var_xg2 = assign6620_e6886;
        var_xg2_dn0 = assign6620_e6886_d_n0;
        var_xg2_dn1 = assign6620_e6886_d_n1;
        var_xg2_dn2 = assign6620_e6886_d_n2;
        var_xg2_dn3 = assign6620_e6886_d_n3;
        var_xg2_dn4 = assign6620_e6886_d_n4;
        var_xg2_dn5 = assign6620_e6886_d_n5;
        var_xg2_dn6 = assign6620_e6886_d_n6;
        var_xg2_dn7 = assign6620_e6886_d_n7;
        var_xg2_dn8 = assign6620_e6886_d_n8;
        var_xg2_dn9 = assign6620_e6886_d_n9;
        var_xg2_dn10 = assign6620_e6886_d_n10;
        var_xg2_dn11 = assign6620_e6886_d_n11;
        var_xg2_dn12 = assign6620_e6886_d_n12;
        var_xg2_db0 = assign6620_e6886_d_b0;
        var_xg2_db1 = assign6620_e6886_d_b1;
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
        var_xg2_rdn11 = 0.0;
        var_xg2_rdn12 = 0.0;
        var_xg2_rdb0 = 0.0;
        var_xg2_rdb1 = 0.0;

        let (assign6630_e6899, assign6630_e6899_d_n0, assign6630_e6899_d_n1, assign6630_e6899_d_n2, assign6630_e6899_d_n3, assign6630_e6899_d_n4, assign6630_e6899_d_n5, assign6630_e6899_d_n6, assign6630_e6899_d_n7, assign6630_e6899_d_n8, assign6630_e6899_d_n9, assign6630_e6899_d_n10, assign6630_e6899_d_n11, assign6630_e6899_d_n12, assign6630_e6899_d_b0, assign6630_e6899_d_b1,) = {
    if ((var_guard120 != 0.0) && (var_guard121 != 0.0)) {
        let assign6630_e6894: f64 = (1.0 + var_xg2);
        let assign6630_e6895: f64 = (assign6630_e6894).sqrt();
        let assign6630_e6896: f64 = (1.0 + assign6630_e6895);
        let assign6630_e6897: f64 = (var_xg2 / assign6630_e6896);
        (assign6630_e6897, (((var_xg2_dn0 * assign6630_e6896) - (var_xg2 * (var_xg2_dn0 / (2.0 * assign6630_e6895)))) / (assign6630_e6896 * assign6630_e6896)), (((var_xg2_dn1 * assign6630_e6896) - (var_xg2 * (var_xg2_dn1 / (2.0 * assign6630_e6895)))) / (assign6630_e6896 * assign6630_e6896)), (((var_xg2_dn2 * assign6630_e6896) - (var_xg2 * (var_xg2_dn2 / (2.0 * assign6630_e6895)))) / (assign6630_e6896 * assign6630_e6896)), (((var_xg2_dn3 * assign6630_e6896) - (var_xg2 * (var_xg2_dn3 / (2.0 * assign6630_e6895)))) / (assign6630_e6896 * assign6630_e6896)), (((var_xg2_dn4 * assign6630_e6896) - (var_xg2 * (var_xg2_dn4 / (2.0 * assign6630_e6895)))) / (assign6630_e6896 * assign6630_e6896)), (((var_xg2_dn5 * assign6630_e6896) - (var_xg2 * (var_xg2_dn5 / (2.0 * assign6630_e6895)))) / (assign6630_e6896 * assign6630_e6896)), (((var_xg2_dn6 * assign6630_e6896) - (var_xg2 * (var_xg2_dn6 / (2.0 * assign6630_e6895)))) / (assign6630_e6896 * assign6630_e6896)), (((var_xg2_dn7 * assign6630_e6896) - (var_xg2 * (var_xg2_dn7 / (2.0 * assign6630_e6895)))) / (assign6630_e6896 * assign6630_e6896)), (((var_xg2_dn8 * assign6630_e6896) - (var_xg2 * (var_xg2_dn8 / (2.0 * assign6630_e6895)))) / (assign6630_e6896 * assign6630_e6896)), (((var_xg2_dn9 * assign6630_e6896) - (var_xg2 * (var_xg2_dn9 / (2.0 * assign6630_e6895)))) / (assign6630_e6896 * assign6630_e6896)), (((var_xg2_dn10 * assign6630_e6896) - (var_xg2 * (var_xg2_dn10 / (2.0 * assign6630_e6895)))) / (assign6630_e6896 * assign6630_e6896)), (((var_xg2_dn11 * assign6630_e6896) - (var_xg2 * (var_xg2_dn11 / (2.0 * assign6630_e6895)))) / (assign6630_e6896 * assign6630_e6896)), (((var_xg2_dn12 * assign6630_e6896) - (var_xg2 * (var_xg2_dn12 / (2.0 * assign6630_e6895)))) / (assign6630_e6896 * assign6630_e6896)), (((var_xg2_db0 * assign6630_e6896) - (var_xg2 * (var_xg2_db0 / (2.0 * assign6630_e6895)))) / (assign6630_e6896 * assign6630_e6896)), (((var_xg2_db1 * assign6630_e6896) - (var_xg2 * (var_xg2_db1 / (2.0 * assign6630_e6895)))) / (assign6630_e6896 * assign6630_e6896)),)
    } else {
        (var_xpwex, var_xpwex_dn0, var_xpwex_dn1, var_xpwex_dn2, var_xpwex_dn3, var_xpwex_dn4, var_xpwex_dn5, var_xpwex_dn6, var_xpwex_dn7, var_xpwex_dn8, var_xpwex_dn9, var_xpwex_dn10, var_xpwex_dn11, var_xpwex_dn12, var_xpwex_db0, var_xpwex_db1,)
    }
};
        var_xpwex = assign6630_e6899;
        var_xpwex_dn0 = assign6630_e6899_d_n0;
        var_xpwex_dn1 = assign6630_e6899_d_n1;
        var_xpwex_dn2 = assign6630_e6899_d_n2;
        var_xpwex_dn3 = assign6630_e6899_d_n3;
        var_xpwex_dn4 = assign6630_e6899_d_n4;
        var_xpwex_dn5 = assign6630_e6899_d_n5;
        var_xpwex_dn6 = assign6630_e6899_d_n6;
        var_xpwex_dn7 = assign6630_e6899_d_n7;
        var_xpwex_dn8 = assign6630_e6899_d_n8;
        var_xpwex_dn9 = assign6630_e6899_d_n9;
        var_xpwex_dn10 = assign6630_e6899_d_n10;
        var_xpwex_dn11 = assign6630_e6899_d_n11;
        var_xpwex_dn12 = assign6630_e6899_d_n12;
        var_xpwex_db0 = assign6630_e6899_d_b0;
        var_xpwex_db1 = assign6630_e6899_d_b1;
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
        var_xpwex_rdn11 = 0.0;
        var_xpwex_rdn12 = 0.0;
        var_xpwex_rdb0 = 0.0;
        var_xpwex_rdb1 = 0.0;

        let (assign6640_e6921, assign6640_e6921_d_n0, assign6640_e6921_d_n1, assign6640_e6921_d_n2, assign6640_e6921_d_n3, assign6640_e6921_d_n4, assign6640_e6921_d_n5, assign6640_e6921_d_n6, assign6640_e6921_d_n7, assign6640_e6921_d_n8, assign6640_e6921_d_n9, assign6640_e6921_d_n10, assign6640_e6921_d_n11, assign6640_e6921_d_n12, assign6640_e6921_d_b0, assign6640_e6921_d_b1,) = {
    if ((var_guard120 != 0.0) && (var_guard121 != 0.0)) {
        let assign6640_e6905: f64 = (0.5 * p.p33);
        let assign6640_e6907: f64 = (assign6640_e6905 * var_taur_t);
        let assign6640_e6910: f64 = (var_qb0 * var_xnbex);
        let assign6640_e6913: f64 = (var_qepi0 * var_xpwex);
        let assign6640_e6914: f64 = (assign6640_e6910 + assign6640_e6913);
        let assign6640_e6915: f64 = (assign6640_e6907 * assign6640_e6914);
        let assign6640_e6918: f64 = (var_taub_t + var_tepi_t);
        let assign6640_e6919: f64 = (assign6640_e6915 / assign6640_e6918);
        (assign6640_e6919, ((((((assign6640_e6905 * var_taur_t_dn0) * assign6640_e6914) + (assign6640_e6907 * (((var_qb0_dn0 * var_xnbex) + (var_qb0 * var_xnbex_dn0)) + ((var_qepi0_dn0 * var_xpwex) + (var_qepi0 * var_xpwex_dn0))))) * assign6640_e6918) - (assign6640_e6915 * (var_taub_t_dn0 + var_tepi_t_dn0))) / (assign6640_e6918 * assign6640_e6918)), ((((((assign6640_e6905 * var_taur_t_dn1) * assign6640_e6914) + (assign6640_e6907 * (((var_qb0_dn1 * var_xnbex) + (var_qb0 * var_xnbex_dn1)) + ((var_qepi0_dn1 * var_xpwex) + (var_qepi0 * var_xpwex_dn1))))) * assign6640_e6918) - (assign6640_e6915 * (var_taub_t_dn1 + var_tepi_t_dn1))) / (assign6640_e6918 * assign6640_e6918)), ((((((assign6640_e6905 * var_taur_t_dn2) * assign6640_e6914) + (assign6640_e6907 * (((var_qb0_dn2 * var_xnbex) + (var_qb0 * var_xnbex_dn2)) + ((var_qepi0_dn2 * var_xpwex) + (var_qepi0 * var_xpwex_dn2))))) * assign6640_e6918) - (assign6640_e6915 * (var_taub_t_dn2 + var_tepi_t_dn2))) / (assign6640_e6918 * assign6640_e6918)), ((((((assign6640_e6905 * var_taur_t_dn3) * assign6640_e6914) + (assign6640_e6907 * (((var_qb0_dn3 * var_xnbex) + (var_qb0 * var_xnbex_dn3)) + ((var_qepi0_dn3 * var_xpwex) + (var_qepi0 * var_xpwex_dn3))))) * assign6640_e6918) - (assign6640_e6915 * (var_taub_t_dn3 + var_tepi_t_dn3))) / (assign6640_e6918 * assign6640_e6918)), ((((((assign6640_e6905 * var_taur_t_dn4) * assign6640_e6914) + (assign6640_e6907 * (((var_qb0_dn4 * var_xnbex) + (var_qb0 * var_xnbex_dn4)) + ((var_qepi0_dn4 * var_xpwex) + (var_qepi0 * var_xpwex_dn4))))) * assign6640_e6918) - (assign6640_e6915 * (var_taub_t_dn4 + var_tepi_t_dn4))) / (assign6640_e6918 * assign6640_e6918)), ((((((assign6640_e6905 * var_taur_t_dn5) * assign6640_e6914) + (assign6640_e6907 * (((var_qb0_dn5 * var_xnbex) + (var_qb0 * var_xnbex_dn5)) + ((var_qepi0_dn5 * var_xpwex) + (var_qepi0 * var_xpwex_dn5))))) * assign6640_e6918) - (assign6640_e6915 * (var_taub_t_dn5 + var_tepi_t_dn5))) / (assign6640_e6918 * assign6640_e6918)), ((((((assign6640_e6905 * var_taur_t_dn6) * assign6640_e6914) + (assign6640_e6907 * (((var_qb0_dn6 * var_xnbex) + (var_qb0 * var_xnbex_dn6)) + ((var_qepi0_dn6 * var_xpwex) + (var_qepi0 * var_xpwex_dn6))))) * assign6640_e6918) - (assign6640_e6915 * (var_taub_t_dn6 + var_tepi_t_dn6))) / (assign6640_e6918 * assign6640_e6918)), ((((((assign6640_e6905 * var_taur_t_dn7) * assign6640_e6914) + (assign6640_e6907 * (((var_qb0_dn7 * var_xnbex) + (var_qb0 * var_xnbex_dn7)) + ((var_qepi0_dn7 * var_xpwex) + (var_qepi0 * var_xpwex_dn7))))) * assign6640_e6918) - (assign6640_e6915 * (var_taub_t_dn7 + var_tepi_t_dn7))) / (assign6640_e6918 * assign6640_e6918)), ((((((assign6640_e6905 * var_taur_t_dn8) * assign6640_e6914) + (assign6640_e6907 * (((var_qb0_dn8 * var_xnbex) + (var_qb0 * var_xnbex_dn8)) + ((var_qepi0_dn8 * var_xpwex) + (var_qepi0 * var_xpwex_dn8))))) * assign6640_e6918) - (assign6640_e6915 * (var_taub_t_dn8 + var_tepi_t_dn8))) / (assign6640_e6918 * assign6640_e6918)), ((((((assign6640_e6905 * var_taur_t_dn9) * assign6640_e6914) + (assign6640_e6907 * (((var_qb0_dn9 * var_xnbex) + (var_qb0 * var_xnbex_dn9)) + ((var_qepi0_dn9 * var_xpwex) + (var_qepi0 * var_xpwex_dn9))))) * assign6640_e6918) - (assign6640_e6915 * (var_taub_t_dn9 + var_tepi_t_dn9))) / (assign6640_e6918 * assign6640_e6918)), ((((((assign6640_e6905 * var_taur_t_dn10) * assign6640_e6914) + (assign6640_e6907 * (((var_qb0_dn10 * var_xnbex) + (var_qb0 * var_xnbex_dn10)) + ((var_qepi0_dn10 * var_xpwex) + (var_qepi0 * var_xpwex_dn10))))) * assign6640_e6918) - (assign6640_e6915 * (var_taub_t_dn10 + var_tepi_t_dn10))) / (assign6640_e6918 * assign6640_e6918)), ((((((assign6640_e6905 * var_taur_t_dn11) * assign6640_e6914) + (assign6640_e6907 * (((var_qb0_dn11 * var_xnbex) + (var_qb0 * var_xnbex_dn11)) + ((var_qepi0_dn11 * var_xpwex) + (var_qepi0 * var_xpwex_dn11))))) * assign6640_e6918) - (assign6640_e6915 * (var_taub_t_dn11 + var_tepi_t_dn11))) / (assign6640_e6918 * assign6640_e6918)), ((((((assign6640_e6905 * var_taur_t_dn12) * assign6640_e6914) + (assign6640_e6907 * (((var_qb0_dn12 * var_xnbex) + (var_qb0 * var_xnbex_dn12)) + ((var_qepi0_dn12 * var_xpwex) + (var_qepi0 * var_xpwex_dn12))))) * assign6640_e6918) - (assign6640_e6915 * (var_taub_t_dn12 + var_tepi_t_dn12))) / (assign6640_e6918 * assign6640_e6918)), ((((((assign6640_e6905 * var_taur_t_db0) * assign6640_e6914) + (assign6640_e6907 * (((var_qb0_db0 * var_xnbex) + (var_qb0 * var_xnbex_db0)) + ((var_qepi0_db0 * var_xpwex) + (var_qepi0 * var_xpwex_db0))))) * assign6640_e6918) - (assign6640_e6915 * (var_taub_t_db0 + var_tepi_t_db0))) / (assign6640_e6918 * assign6640_e6918)), ((((((assign6640_e6905 * var_taur_t_db1) * assign6640_e6914) + (assign6640_e6907 * (((var_qb0_db1 * var_xnbex) + (var_qb0 * var_xnbex_db1)) + ((var_qepi0_db1 * var_xpwex) + (var_qepi0 * var_xpwex_db1))))) * assign6640_e6918) - (assign6640_e6915 * (var_taub_t_db1 + var_tepi_t_db1))) / (assign6640_e6918 * assign6640_e6918)),)
    } else {
        (var_xqmex, var_xqmex_dn0, var_xqmex_dn1, var_xqmex_dn2, var_xqmex_dn3, var_xqmex_dn4, var_xqmex_dn5, var_xqmex_dn6, var_xqmex_dn7, var_xqmex_dn8, var_xqmex_dn9, var_xqmex_dn10, var_xqmex_dn11, var_xqmex_dn12, var_xqmex_db0, var_xqmex_db1,)
    }
};
        var_xqmex = assign6640_e6921;
        var_xqmex_dn0 = assign6640_e6921_d_n0;
        var_xqmex_dn1 = assign6640_e6921_d_n1;
        var_xqmex_dn2 = assign6640_e6921_d_n2;
        var_xqmex_dn3 = assign6640_e6921_d_n3;
        var_xqmex_dn4 = assign6640_e6921_d_n4;
        var_xqmex_dn5 = assign6640_e6921_d_n5;
        var_xqmex_dn6 = assign6640_e6921_d_n6;
        var_xqmex_dn7 = assign6640_e6921_d_n7;
        var_xqmex_dn8 = assign6640_e6921_d_n8;
        var_xqmex_dn9 = assign6640_e6921_d_n9;
        var_xqmex_dn10 = assign6640_e6921_d_n10;
        var_xqmex_dn11 = assign6640_e6921_d_n11;
        var_xqmex_dn12 = assign6640_e6921_d_n12;
        var_xqmex_db0 = assign6640_e6921_d_b0;
        var_xqmex_db1 = assign6640_e6921_d_b1;
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
        var_xqmex_rdn11 = 0.0;
        var_xqmex_rdn12 = 0.0;
        var_xqmex_rdb0 = 0.0;
        var_xqmex_rdb1 = 0.0;

        let assign6650_e6924: f64 = (var_vbc3 - var_vdcex_t);
        let assign6650_e6926: f64 = (assign6650_e6924 * var_vtinv);
        let assign6650_e6928: f64 = if assign6650_e6926 < p.p151 { 1.0 } else { 0.0 };
        var_guard122 = assign6650_e6928;
        var_guard122_dn0 = 0.0;
        var_guard122_dn1 = 0.0;
        var_guard122_dn2 = 0.0;
        var_guard122_dn3 = 0.0;
        var_guard122_dn4 = 0.0;
        var_guard122_dn5 = 0.0;
        var_guard122_dn6 = 0.0;
        var_guard122_dn7 = 0.0;
        var_guard122_dn8 = 0.0;
        var_guard122_dn9 = 0.0;
        var_guard122_dn10 = 0.0;
        var_guard122_dn11 = 0.0;
        var_guard122_dn12 = 0.0;
        var_guard122_db0 = 0.0;
        var_guard122_db1 = 0.0;
        var_guard122_rv = 0.0;
        var_guard122_rdn0 = 0.0;
        var_guard122_rdn1 = 0.0;
        var_guard122_rdn2 = 0.0;
        var_guard122_rdn3 = 0.0;
        var_guard122_rdn4 = 0.0;
        var_guard122_rdn5 = 0.0;
        var_guard122_rdn6 = 0.0;
        var_guard122_rdn7 = 0.0;
        var_guard122_rdn8 = 0.0;
        var_guard122_rdn9 = 0.0;
        var_guard122_rdn10 = 0.0;
        var_guard122_rdn11 = 0.0;
        var_guard122_rdn12 = 0.0;
        var_guard122_rdb0 = 0.0;
        var_guard122_rdb1 = 0.0;

        let (assign6660_e6942, assign6660_e6942_d_n0, assign6660_e6942_d_n1, assign6660_e6942_d_n2, assign6660_e6942_d_n3, assign6660_e6942_d_n4, assign6660_e6942_d_n5, assign6660_e6942_d_n6, assign6660_e6942_d_n7, assign6660_e6942_d_n8, assign6660_e6942_d_n9, assign6660_e6942_d_n10, assign6660_e6942_d_n11, assign6660_e6942_d_n12, assign6660_e6942_d_b0, assign6660_e6942_d_b1,) = {
    if (((var_guard120 != 0.0) && (var_guard121 == 0.0)) && (var_guard122 != 0.0)) {
        let assign6660_e6937: f64 = (var_vbc3 - var_vdcex_t);
        let assign6660_e6939: f64 = (assign6660_e6937 * var_vtinv);
        let assign6660_e6940: f64 = (assign6660_e6939).exp();
        (assign6660_e6940, (assign6660_e6940 * (((var_vbc3_dn0 - var_vdcex_t_dn0) * var_vtinv) + (assign6660_e6937 * var_vtinv_dn0))), (assign6660_e6940 * (((var_vbc3_dn1 - var_vdcex_t_dn1) * var_vtinv) + (assign6660_e6937 * var_vtinv_dn1))), (assign6660_e6940 * (((var_vbc3_dn2 - var_vdcex_t_dn2) * var_vtinv) + (assign6660_e6937 * var_vtinv_dn2))), (assign6660_e6940 * (((var_vbc3_dn3 - var_vdcex_t_dn3) * var_vtinv) + (assign6660_e6937 * var_vtinv_dn3))), (assign6660_e6940 * (((var_vbc3_dn4 - var_vdcex_t_dn4) * var_vtinv) + (assign6660_e6937 * var_vtinv_dn4))), (assign6660_e6940 * (((var_vbc3_dn5 - var_vdcex_t_dn5) * var_vtinv) + (assign6660_e6937 * var_vtinv_dn5))), (assign6660_e6940 * (((var_vbc3_dn6 - var_vdcex_t_dn6) * var_vtinv) + (assign6660_e6937 * var_vtinv_dn6))), (assign6660_e6940 * (((var_vbc3_dn7 - var_vdcex_t_dn7) * var_vtinv) + (assign6660_e6937 * var_vtinv_dn7))), (assign6660_e6940 * (((var_vbc3_dn8 - var_vdcex_t_dn8) * var_vtinv) + (assign6660_e6937 * var_vtinv_dn8))), (assign6660_e6940 * (((var_vbc3_dn9 - var_vdcex_t_dn9) * var_vtinv) + (assign6660_e6937 * var_vtinv_dn9))), (assign6660_e6940 * (((var_vbc3_dn10 - var_vdcex_t_dn10) * var_vtinv) + (assign6660_e6937 * var_vtinv_dn10))), (assign6660_e6940 * (((var_vbc3_dn11 - var_vdcex_t_dn11) * var_vtinv) + (assign6660_e6937 * var_vtinv_dn11))), (assign6660_e6940 * (((var_vbc3_dn12 - var_vdcex_t_dn12) * var_vtinv) + (assign6660_e6937 * var_vtinv_dn12))), (assign6660_e6940 * (((var_vbc3_db0 - var_vdcex_t_db0) * var_vtinv) + (assign6660_e6937 * var_vtinv_db0))), (assign6660_e6940 * (((var_vbc3_db1 - var_vdcex_t_db1) * var_vtinv) + (assign6660_e6937 * var_vtinv_db1))),)
    } else {
        (var_evbc3vdcex, var_evbc3vdcex_dn0, var_evbc3vdcex_dn1, var_evbc3vdcex_dn2, var_evbc3vdcex_dn3, var_evbc3vdcex_dn4, var_evbc3vdcex_dn5, var_evbc3vdcex_dn6, var_evbc3vdcex_dn7, var_evbc3vdcex_dn8, var_evbc3vdcex_dn9, var_evbc3vdcex_dn10, var_evbc3vdcex_dn11, var_evbc3vdcex_dn12, var_evbc3vdcex_db0, var_evbc3vdcex_db1,)
    }
};
        var_evbc3vdcex = assign6660_e6942;
        var_evbc3vdcex_dn0 = assign6660_e6942_d_n0;
        var_evbc3vdcex_dn1 = assign6660_e6942_d_n1;
        var_evbc3vdcex_dn2 = assign6660_e6942_d_n2;
        var_evbc3vdcex_dn3 = assign6660_e6942_d_n3;
        var_evbc3vdcex_dn4 = assign6660_e6942_d_n4;
        var_evbc3vdcex_dn5 = assign6660_e6942_d_n5;
        var_evbc3vdcex_dn6 = assign6660_e6942_d_n6;
        var_evbc3vdcex_dn7 = assign6660_e6942_d_n7;
        var_evbc3vdcex_dn8 = assign6660_e6942_d_n8;
        var_evbc3vdcex_dn9 = assign6660_e6942_d_n9;
        var_evbc3vdcex_dn10 = assign6660_e6942_d_n10;
        var_evbc3vdcex_dn11 = assign6660_e6942_d_n11;
        var_evbc3vdcex_dn12 = assign6660_e6942_d_n12;
        var_evbc3vdcex_db0 = assign6660_e6942_d_b0;
        var_evbc3vdcex_db1 = assign6660_e6942_d_b1;
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
        var_evbc3vdcex_rdn11 = 0.0;
        var_evbc3vdcex_rdn12 = 0.0;
        var_evbc3vdcex_rdb0 = 0.0;
        var_evbc3vdcex_rdb1 = 0.0;

        let (assign6670_e6953, assign6670_e6953_d_n0, assign6670_e6953_d_n1, assign6670_e6953_d_n2, assign6670_e6953_d_n3, assign6670_e6953_d_n4, assign6670_e6953_d_n5, assign6670_e6953_d_n6, assign6670_e6953_d_n7, assign6670_e6953_d_n8, assign6670_e6953_d_n9, assign6670_e6953_d_n10, assign6670_e6953_d_n11, assign6670_e6953_d_n12, assign6670_e6953_d_b0, assign6670_e6953_d_b1,) = {
    if (((var_guard120 != 0.0) && (var_guard121 == 0.0)) && (var_guard122 == 0.0)) {
        let assign6670_e6951: f64 = (p.p151).exp();
        (assign6670_e6951, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_expl, var_expl_dn0, var_expl_dn1, var_expl_dn2, var_expl_dn3, var_expl_dn4, var_expl_dn5, var_expl_dn6, var_expl_dn7, var_expl_dn8, var_expl_dn9, var_expl_dn10, var_expl_dn11, var_expl_dn12, var_expl_db0, var_expl_db1,)
    }
};
        var_expl = assign6670_e6953;
        var_expl_dn0 = assign6670_e6953_d_n0;
        var_expl_dn1 = assign6670_e6953_d_n1;
        var_expl_dn2 = assign6670_e6953_d_n2;
        var_expl_dn3 = assign6670_e6953_d_n3;
        var_expl_dn4 = assign6670_e6953_d_n4;
        var_expl_dn5 = assign6670_e6953_d_n5;
        var_expl_dn6 = assign6670_e6953_d_n6;
        var_expl_dn7 = assign6670_e6953_d_n7;
        var_expl_dn8 = assign6670_e6953_d_n8;
        var_expl_dn9 = assign6670_e6953_d_n9;
        var_expl_dn10 = assign6670_e6953_d_n10;
        var_expl_dn11 = assign6670_e6953_d_n11;
        var_expl_dn12 = assign6670_e6953_d_n12;
        var_expl_db0 = assign6670_e6953_d_b0;
        var_expl_db1 = assign6670_e6953_d_b1;
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
        var_expl_rdn11 = 0.0;
        var_expl_rdn12 = 0.0;
        var_expl_rdb0 = 0.0;
        var_expl_rdb1 = 0.0;

        let (assign6680_e6973, assign6680_e6973_d_n0, assign6680_e6973_d_n1, assign6680_e6973_d_n2, assign6680_e6973_d_n3, assign6680_e6973_d_n4, assign6680_e6973_d_n5, assign6680_e6973_d_n6, assign6680_e6973_d_n7, assign6680_e6973_d_n8, assign6680_e6973_d_n9, assign6680_e6973_d_n10, assign6680_e6973_d_n11, assign6680_e6973_d_n12, assign6680_e6973_d_b0, assign6680_e6973_d_b1,) = {
    if (((var_guard120 != 0.0) && (var_guard121 == 0.0)) && (var_guard122 == 0.0)) {
        let assign6680_e6965: f64 = (var_vbc3 - var_vdcex_t);
        let assign6680_e6967: f64 = (assign6680_e6965 * var_vtinv);
        let assign6680_e6969: f64 = (assign6680_e6967 - p.p151);
        let assign6680_e6970: f64 = (1.0 + assign6680_e6969);
        let assign6680_e6971: f64 = (var_expl * assign6680_e6970);
        (assign6680_e6971, ((var_expl_dn0 * assign6680_e6970) + (var_expl * (((var_vbc3_dn0 - var_vdcex_t_dn0) * var_vtinv) + (assign6680_e6965 * var_vtinv_dn0)))), ((var_expl_dn1 * assign6680_e6970) + (var_expl * (((var_vbc3_dn1 - var_vdcex_t_dn1) * var_vtinv) + (assign6680_e6965 * var_vtinv_dn1)))), ((var_expl_dn2 * assign6680_e6970) + (var_expl * (((var_vbc3_dn2 - var_vdcex_t_dn2) * var_vtinv) + (assign6680_e6965 * var_vtinv_dn2)))), ((var_expl_dn3 * assign6680_e6970) + (var_expl * (((var_vbc3_dn3 - var_vdcex_t_dn3) * var_vtinv) + (assign6680_e6965 * var_vtinv_dn3)))), ((var_expl_dn4 * assign6680_e6970) + (var_expl * (((var_vbc3_dn4 - var_vdcex_t_dn4) * var_vtinv) + (assign6680_e6965 * var_vtinv_dn4)))), ((var_expl_dn5 * assign6680_e6970) + (var_expl * (((var_vbc3_dn5 - var_vdcex_t_dn5) * var_vtinv) + (assign6680_e6965 * var_vtinv_dn5)))), ((var_expl_dn6 * assign6680_e6970) + (var_expl * (((var_vbc3_dn6 - var_vdcex_t_dn6) * var_vtinv) + (assign6680_e6965 * var_vtinv_dn6)))), ((var_expl_dn7 * assign6680_e6970) + (var_expl * (((var_vbc3_dn7 - var_vdcex_t_dn7) * var_vtinv) + (assign6680_e6965 * var_vtinv_dn7)))), ((var_expl_dn8 * assign6680_e6970) + (var_expl * (((var_vbc3_dn8 - var_vdcex_t_dn8) * var_vtinv) + (assign6680_e6965 * var_vtinv_dn8)))), ((var_expl_dn9 * assign6680_e6970) + (var_expl * (((var_vbc3_dn9 - var_vdcex_t_dn9) * var_vtinv) + (assign6680_e6965 * var_vtinv_dn9)))), ((var_expl_dn10 * assign6680_e6970) + (var_expl * (((var_vbc3_dn10 - var_vdcex_t_dn10) * var_vtinv) + (assign6680_e6965 * var_vtinv_dn10)))), ((var_expl_dn11 * assign6680_e6970) + (var_expl * (((var_vbc3_dn11 - var_vdcex_t_dn11) * var_vtinv) + (assign6680_e6965 * var_vtinv_dn11)))), ((var_expl_dn12 * assign6680_e6970) + (var_expl * (((var_vbc3_dn12 - var_vdcex_t_dn12) * var_vtinv) + (assign6680_e6965 * var_vtinv_dn12)))), ((var_expl_db0 * assign6680_e6970) + (var_expl * (((var_vbc3_db0 - var_vdcex_t_db0) * var_vtinv) + (assign6680_e6965 * var_vtinv_db0)))), ((var_expl_db1 * assign6680_e6970) + (var_expl * (((var_vbc3_db1 - var_vdcex_t_db1) * var_vtinv) + (assign6680_e6965 * var_vtinv_db1)))),)
    } else {
        (var_evbc3vdcex, var_evbc3vdcex_dn0, var_evbc3vdcex_dn1, var_evbc3vdcex_dn2, var_evbc3vdcex_dn3, var_evbc3vdcex_dn4, var_evbc3vdcex_dn5, var_evbc3vdcex_dn6, var_evbc3vdcex_dn7, var_evbc3vdcex_dn8, var_evbc3vdcex_dn9, var_evbc3vdcex_dn10, var_evbc3vdcex_dn11, var_evbc3vdcex_dn12, var_evbc3vdcex_db0, var_evbc3vdcex_db1,)
    }
};
        var_evbc3vdcex = assign6680_e6973;
        var_evbc3vdcex_dn0 = assign6680_e6973_d_n0;
        var_evbc3vdcex_dn1 = assign6680_e6973_d_n1;
        var_evbc3vdcex_dn2 = assign6680_e6973_d_n2;
        var_evbc3vdcex_dn3 = assign6680_e6973_d_n3;
        var_evbc3vdcex_dn4 = assign6680_e6973_d_n4;
        var_evbc3vdcex_dn5 = assign6680_e6973_d_n5;
        var_evbc3vdcex_dn6 = assign6680_e6973_d_n6;
        var_evbc3vdcex_dn7 = assign6680_e6973_d_n7;
        var_evbc3vdcex_dn8 = assign6680_e6973_d_n8;
        var_evbc3vdcex_dn9 = assign6680_e6973_d_n9;
        var_evbc3vdcex_dn10 = assign6680_e6973_d_n10;
        var_evbc3vdcex_dn11 = assign6680_e6973_d_n11;
        var_evbc3vdcex_dn12 = assign6680_e6973_d_n12;
        var_evbc3vdcex_db0 = assign6680_e6973_d_b0;
        var_evbc3vdcex_db1 = assign6680_e6973_d_b1;
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
        var_evbc3vdcex_rdn11 = 0.0;
        var_evbc3vdcex_rdn12 = 0.0;
        var_evbc3vdcex_rdb0 = 0.0;
        var_evbc3vdcex_rdb1 = 0.0;

        let (assign6690_e6997, assign6690_e6997_d_n0, assign6690_e6997_d_n1, assign6690_e6997_d_n2, assign6690_e6997_d_n3, assign6690_e6997_d_n4, assign6690_e6997_d_n5, assign6690_e6997_d_n6, assign6690_e6997_d_n7, assign6690_e6997_d_n8, assign6690_e6997_d_n9, assign6690_e6997_d_n10, assign6690_e6997_d_n11, assign6690_e6997_d_n12, assign6690_e6997_d_b0, assign6690_e6997_d_b1,) = {
    if ((var_guard120 != 0.0) && (var_guard121 == 0.0)) {
        let assign6690_e6980: f64 = (2.0 * p.p33);
        let assign6690_e6982: f64 = (assign6690_e6980 * var_ibx_t);
        let assign6690_e6984: f64 = (assign6690_e6982 * var_tauex_t);
        let assign6690_e6986: f64 = (assign6690_e6984 * var_evbc3);
        let assign6690_e6991: f64 = (4.0 * var_evbc3vdcex);
        let assign6690_e6992: f64 = (1.0 + assign6690_e6991);
        let assign6690_e6993: f64 = (assign6690_e6992).sqrt();
        let assign6690_e6994: f64 = (1.0 + assign6690_e6993);
        let assign6690_e6995: f64 = (assign6690_e6986 / assign6690_e6994);
        (assign6690_e6995, ((((((((assign6690_e6980 * var_ibx_t_dn0) * var_tauex_t) + (assign6690_e6982 * var_tauex_t_dn0)) * var_evbc3) + (assign6690_e6984 * var_evbc3_dn0)) * assign6690_e6994) - (assign6690_e6986 * ((4.0 * var_evbc3vdcex_dn0) / (2.0 * assign6690_e6993)))) / (assign6690_e6994 * assign6690_e6994)), ((((((((assign6690_e6980 * var_ibx_t_dn1) * var_tauex_t) + (assign6690_e6982 * var_tauex_t_dn1)) * var_evbc3) + (assign6690_e6984 * var_evbc3_dn1)) * assign6690_e6994) - (assign6690_e6986 * ((4.0 * var_evbc3vdcex_dn1) / (2.0 * assign6690_e6993)))) / (assign6690_e6994 * assign6690_e6994)), ((((((((assign6690_e6980 * var_ibx_t_dn2) * var_tauex_t) + (assign6690_e6982 * var_tauex_t_dn2)) * var_evbc3) + (assign6690_e6984 * var_evbc3_dn2)) * assign6690_e6994) - (assign6690_e6986 * ((4.0 * var_evbc3vdcex_dn2) / (2.0 * assign6690_e6993)))) / (assign6690_e6994 * assign6690_e6994)), ((((((((assign6690_e6980 * var_ibx_t_dn3) * var_tauex_t) + (assign6690_e6982 * var_tauex_t_dn3)) * var_evbc3) + (assign6690_e6984 * var_evbc3_dn3)) * assign6690_e6994) - (assign6690_e6986 * ((4.0 * var_evbc3vdcex_dn3) / (2.0 * assign6690_e6993)))) / (assign6690_e6994 * assign6690_e6994)), ((((((((assign6690_e6980 * var_ibx_t_dn4) * var_tauex_t) + (assign6690_e6982 * var_tauex_t_dn4)) * var_evbc3) + (assign6690_e6984 * var_evbc3_dn4)) * assign6690_e6994) - (assign6690_e6986 * ((4.0 * var_evbc3vdcex_dn4) / (2.0 * assign6690_e6993)))) / (assign6690_e6994 * assign6690_e6994)), ((((((((assign6690_e6980 * var_ibx_t_dn5) * var_tauex_t) + (assign6690_e6982 * var_tauex_t_dn5)) * var_evbc3) + (assign6690_e6984 * var_evbc3_dn5)) * assign6690_e6994) - (assign6690_e6986 * ((4.0 * var_evbc3vdcex_dn5) / (2.0 * assign6690_e6993)))) / (assign6690_e6994 * assign6690_e6994)), ((((((((assign6690_e6980 * var_ibx_t_dn6) * var_tauex_t) + (assign6690_e6982 * var_tauex_t_dn6)) * var_evbc3) + (assign6690_e6984 * var_evbc3_dn6)) * assign6690_e6994) - (assign6690_e6986 * ((4.0 * var_evbc3vdcex_dn6) / (2.0 * assign6690_e6993)))) / (assign6690_e6994 * assign6690_e6994)), ((((((((assign6690_e6980 * var_ibx_t_dn7) * var_tauex_t) + (assign6690_e6982 * var_tauex_t_dn7)) * var_evbc3) + (assign6690_e6984 * var_evbc3_dn7)) * assign6690_e6994) - (assign6690_e6986 * ((4.0 * var_evbc3vdcex_dn7) / (2.0 * assign6690_e6993)))) / (assign6690_e6994 * assign6690_e6994)), ((((((((assign6690_e6980 * var_ibx_t_dn8) * var_tauex_t) + (assign6690_e6982 * var_tauex_t_dn8)) * var_evbc3) + (assign6690_e6984 * var_evbc3_dn8)) * assign6690_e6994) - (assign6690_e6986 * ((4.0 * var_evbc3vdcex_dn8) / (2.0 * assign6690_e6993)))) / (assign6690_e6994 * assign6690_e6994)), ((((((((assign6690_e6980 * var_ibx_t_dn9) * var_tauex_t) + (assign6690_e6982 * var_tauex_t_dn9)) * var_evbc3) + (assign6690_e6984 * var_evbc3_dn9)) * assign6690_e6994) - (assign6690_e6986 * ((4.0 * var_evbc3vdcex_dn9) / (2.0 * assign6690_e6993)))) / (assign6690_e6994 * assign6690_e6994)), ((((((((assign6690_e6980 * var_ibx_t_dn10) * var_tauex_t) + (assign6690_e6982 * var_tauex_t_dn10)) * var_evbc3) + (assign6690_e6984 * var_evbc3_dn10)) * assign6690_e6994) - (assign6690_e6986 * ((4.0 * var_evbc3vdcex_dn10) / (2.0 * assign6690_e6993)))) / (assign6690_e6994 * assign6690_e6994)), ((((((((assign6690_e6980 * var_ibx_t_dn11) * var_tauex_t) + (assign6690_e6982 * var_tauex_t_dn11)) * var_evbc3) + (assign6690_e6984 * var_evbc3_dn11)) * assign6690_e6994) - (assign6690_e6986 * ((4.0 * var_evbc3vdcex_dn11) / (2.0 * assign6690_e6993)))) / (assign6690_e6994 * assign6690_e6994)), ((((((((assign6690_e6980 * var_ibx_t_dn12) * var_tauex_t) + (assign6690_e6982 * var_tauex_t_dn12)) * var_evbc3) + (assign6690_e6984 * var_evbc3_dn12)) * assign6690_e6994) - (assign6690_e6986 * ((4.0 * var_evbc3vdcex_dn12) / (2.0 * assign6690_e6993)))) / (assign6690_e6994 * assign6690_e6994)), ((((((((assign6690_e6980 * var_ibx_t_db0) * var_tauex_t) + (assign6690_e6982 * var_tauex_t_db0)) * var_evbc3) + (assign6690_e6984 * var_evbc3_db0)) * assign6690_e6994) - (assign6690_e6986 * ((4.0 * var_evbc3vdcex_db0) / (2.0 * assign6690_e6993)))) / (assign6690_e6994 * assign6690_e6994)), ((((((((assign6690_e6980 * var_ibx_t_db1) * var_tauex_t) + (assign6690_e6982 * var_tauex_t_db1)) * var_evbc3) + (assign6690_e6984 * var_evbc3_db1)) * assign6690_e6994) - (assign6690_e6986 * ((4.0 * var_evbc3vdcex_db1) / (2.0 * assign6690_e6993)))) / (assign6690_e6994 * assign6690_e6994)),)
    } else {
        (var_xqmex, var_xqmex_dn0, var_xqmex_dn1, var_xqmex_dn2, var_xqmex_dn3, var_xqmex_dn4, var_xqmex_dn5, var_xqmex_dn6, var_xqmex_dn7, var_xqmex_dn8, var_xqmex_dn9, var_xqmex_dn10, var_xqmex_dn11, var_xqmex_dn12, var_xqmex_db0, var_xqmex_db1,)
    }
};
        var_xqmex = assign6690_e6997;
        var_xqmex_dn0 = assign6690_e6997_d_n0;
        var_xqmex_dn1 = assign6690_e6997_d_n1;
        var_xqmex_dn2 = assign6690_e6997_d_n2;
        var_xqmex_dn3 = assign6690_e6997_d_n3;
        var_xqmex_dn4 = assign6690_e6997_d_n4;
        var_xqmex_dn5 = assign6690_e6997_d_n5;
        var_xqmex_dn6 = assign6690_e6997_d_n6;
        var_xqmex_dn7 = assign6690_e6997_d_n7;
        var_xqmex_dn8 = assign6690_e6997_d_n8;
        var_xqmex_dn9 = assign6690_e6997_d_n9;
        var_xqmex_dn10 = assign6690_e6997_d_n10;
        var_xqmex_dn11 = assign6690_e6997_d_n11;
        var_xqmex_dn12 = assign6690_e6997_d_n12;
        var_xqmex_db0 = assign6690_e6997_d_b0;
        var_xqmex_db1 = assign6690_e6997_d_b1;
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
        var_xqmex_rdn11 = 0.0;
        var_xqmex_rdn12 = 0.0;
        var_xqmex_rdb0 = 0.0;
        var_xqmex_rdb1 = 0.0;

        let (assign6700_e7003, assign6700_e7003_d_n0, assign6700_e7003_d_n1, assign6700_e7003_d_n2, assign6700_e7003_d_n3, assign6700_e7003_d_n4, assign6700_e7003_d_n5, assign6700_e7003_d_n6, assign6700_e7003_d_n7, assign6700_e7003_d_n8, assign6700_e7003_d_n9, assign6700_e7003_d_n10, assign6700_e7003_d_n11, assign6700_e7003_d_n12, assign6700_e7003_d_b0, assign6700_e7003_d_b1,) = {
    if (var_guard120 != 0.0) {
        let assign6700_e7001: f64 = (var_fex * var_xqmex);
        (assign6700_e7001, ((var_fex_dn0 * var_xqmex) + (var_fex * var_xqmex_dn0)), ((var_fex_dn1 * var_xqmex) + (var_fex * var_xqmex_dn1)), ((var_fex_dn2 * var_xqmex) + (var_fex * var_xqmex_dn2)), ((var_fex_dn3 * var_xqmex) + (var_fex * var_xqmex_dn3)), ((var_fex_dn4 * var_xqmex) + (var_fex * var_xqmex_dn4)), ((var_fex_dn5 * var_xqmex) + (var_fex * var_xqmex_dn5)), ((var_fex_dn6 * var_xqmex) + (var_fex * var_xqmex_dn6)), ((var_fex_dn7 * var_xqmex) + (var_fex * var_xqmex_dn7)), ((var_fex_dn8 * var_xqmex) + (var_fex * var_xqmex_dn8)), ((var_fex_dn9 * var_xqmex) + (var_fex * var_xqmex_dn9)), ((var_fex_dn10 * var_xqmex) + (var_fex * var_xqmex_dn10)), ((var_fex_dn11 * var_xqmex) + (var_fex * var_xqmex_dn11)), ((var_fex_dn12 * var_xqmex) + (var_fex * var_xqmex_dn12)), ((var_fex_db0 * var_xqmex) + (var_fex * var_xqmex_db0)), ((var_fex_db1 * var_xqmex) + (var_fex * var_xqmex_db1)),)
    } else {
        (var_xqex, var_xqex_dn0, var_xqex_dn1, var_xqex_dn2, var_xqex_dn3, var_xqex_dn4, var_xqex_dn5, var_xqex_dn6, var_xqex_dn7, var_xqex_dn8, var_xqex_dn9, var_xqex_dn10, var_xqex_dn11, var_xqex_dn12, var_xqex_db0, var_xqex_db1,)
    }
};
        var_xqex = assign6700_e7003;
        var_xqex_dn0 = assign6700_e7003_d_n0;
        var_xqex_dn1 = assign6700_e7003_d_n1;
        var_xqex_dn2 = assign6700_e7003_d_n2;
        var_xqex_dn3 = assign6700_e7003_d_n3;
        var_xqex_dn4 = assign6700_e7003_d_n4;
        var_xqex_dn5 = assign6700_e7003_d_n5;
        var_xqex_dn6 = assign6700_e7003_d_n6;
        var_xqex_dn7 = assign6700_e7003_d_n7;
        var_xqex_dn8 = assign6700_e7003_d_n8;
        var_xqex_dn9 = assign6700_e7003_d_n9;
        var_xqex_dn10 = assign6700_e7003_d_n10;
        var_xqex_dn11 = assign6700_e7003_d_n11;
        var_xqex_dn12 = assign6700_e7003_d_n12;
        var_xqex_db0 = assign6700_e7003_d_b0;
        var_xqex_db1 = assign6700_e7003_d_b1;
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
        var_xqex_rdn11 = 0.0;
        var_xqex_rdn12 = 0.0;
        var_xqex_rdb0 = 0.0;
        var_xqex_rdb1 = 0.0;

        *var_evbc3vdcex_slot = var_evbc3vdcex;
        *var_evbc3vdcex_db0_slot = var_evbc3vdcex_db0;
        *var_evbc3vdcex_db1_slot = var_evbc3vdcex_db1;
        *var_evbc3vdcex_dn0_slot = var_evbc3vdcex_dn0;
        *var_evbc3vdcex_dn1_slot = var_evbc3vdcex_dn1;
        *var_evbc3vdcex_dn10_slot = var_evbc3vdcex_dn10;
        *var_evbc3vdcex_dn11_slot = var_evbc3vdcex_dn11;
        *var_evbc3vdcex_dn12_slot = var_evbc3vdcex_dn12;
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
        *var_evbc3vdcex_rdn11_slot = var_evbc3vdcex_rdn11;
        *var_evbc3vdcex_rdn12_slot = var_evbc3vdcex_rdn12;
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
        *var_expl_dn11_slot = var_expl_dn11;
        *var_expl_dn12_slot = var_expl_dn12;
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
        *var_expl_rdn11_slot = var_expl_rdn11;
        *var_expl_rdn12_slot = var_expl_rdn12;
        *var_expl_rdn2_slot = var_expl_rdn2;
        *var_expl_rdn3_slot = var_expl_rdn3;
        *var_expl_rdn4_slot = var_expl_rdn4;
        *var_expl_rdn5_slot = var_expl_rdn5;
        *var_expl_rdn6_slot = var_expl_rdn6;
        *var_expl_rdn7_slot = var_expl_rdn7;
        *var_expl_rdn8_slot = var_expl_rdn8;
        *var_expl_rdn9_slot = var_expl_rdn9;
        *var_expl_rv_slot = var_expl_rv;
        *var_guard121_slot = var_guard121;
        *var_guard121_db0_slot = var_guard121_db0;
        *var_guard121_db1_slot = var_guard121_db1;
        *var_guard121_dn0_slot = var_guard121_dn0;
        *var_guard121_dn1_slot = var_guard121_dn1;
        *var_guard121_dn10_slot = var_guard121_dn10;
        *var_guard121_dn11_slot = var_guard121_dn11;
        *var_guard121_dn12_slot = var_guard121_dn12;
        *var_guard121_dn2_slot = var_guard121_dn2;
        *var_guard121_dn3_slot = var_guard121_dn3;
        *var_guard121_dn4_slot = var_guard121_dn4;
        *var_guard121_dn5_slot = var_guard121_dn5;
        *var_guard121_dn6_slot = var_guard121_dn6;
        *var_guard121_dn7_slot = var_guard121_dn7;
        *var_guard121_dn8_slot = var_guard121_dn8;
        *var_guard121_dn9_slot = var_guard121_dn9;
        *var_guard121_rdb0_slot = var_guard121_rdb0;
        *var_guard121_rdb1_slot = var_guard121_rdb1;
        *var_guard121_rdn0_slot = var_guard121_rdn0;
        *var_guard121_rdn1_slot = var_guard121_rdn1;
        *var_guard121_rdn10_slot = var_guard121_rdn10;
        *var_guard121_rdn11_slot = var_guard121_rdn11;
        *var_guard121_rdn12_slot = var_guard121_rdn12;
        *var_guard121_rdn2_slot = var_guard121_rdn2;
        *var_guard121_rdn3_slot = var_guard121_rdn3;
        *var_guard121_rdn4_slot = var_guard121_rdn4;
        *var_guard121_rdn5_slot = var_guard121_rdn5;
        *var_guard121_rdn6_slot = var_guard121_rdn6;
        *var_guard121_rdn7_slot = var_guard121_rdn7;
        *var_guard121_rdn8_slot = var_guard121_rdn8;
        *var_guard121_rdn9_slot = var_guard121_rdn9;
        *var_guard121_rv_slot = var_guard121_rv;
        *var_guard122_slot = var_guard122;
        *var_guard122_db0_slot = var_guard122_db0;
        *var_guard122_db1_slot = var_guard122_db1;
        *var_guard122_dn0_slot = var_guard122_dn0;
        *var_guard122_dn1_slot = var_guard122_dn1;
        *var_guard122_dn10_slot = var_guard122_dn10;
        *var_guard122_dn11_slot = var_guard122_dn11;
        *var_guard122_dn12_slot = var_guard122_dn12;
        *var_guard122_dn2_slot = var_guard122_dn2;
        *var_guard122_dn3_slot = var_guard122_dn3;
        *var_guard122_dn4_slot = var_guard122_dn4;
        *var_guard122_dn5_slot = var_guard122_dn5;
        *var_guard122_dn6_slot = var_guard122_dn6;
        *var_guard122_dn7_slot = var_guard122_dn7;
        *var_guard122_dn8_slot = var_guard122_dn8;
        *var_guard122_dn9_slot = var_guard122_dn9;
        *var_guard122_rdb0_slot = var_guard122_rdb0;
        *var_guard122_rdb1_slot = var_guard122_rdb1;
        *var_guard122_rdn0_slot = var_guard122_rdn0;
        *var_guard122_rdn1_slot = var_guard122_rdn1;
        *var_guard122_rdn10_slot = var_guard122_rdn10;
        *var_guard122_rdn11_slot = var_guard122_rdn11;
        *var_guard122_rdn12_slot = var_guard122_rdn12;
        *var_guard122_rdn2_slot = var_guard122_rdn2;
        *var_guard122_rdn3_slot = var_guard122_rdn3;
        *var_guard122_rdn4_slot = var_guard122_rdn4;
        *var_guard122_rdn5_slot = var_guard122_rdn5;
        *var_guard122_rdn6_slot = var_guard122_rdn6;
        *var_guard122_rdn7_slot = var_guard122_rdn7;
        *var_guard122_rdn8_slot = var_guard122_rdn8;
        *var_guard122_rdn9_slot = var_guard122_rdn9;
        *var_guard122_rv_slot = var_guard122_rv;
        *var_xg1_slot = var_xg1;
        *var_xg1_db0_slot = var_xg1_db0;
        *var_xg1_db1_slot = var_xg1_db1;
        *var_xg1_dn0_slot = var_xg1_dn0;
        *var_xg1_dn1_slot = var_xg1_dn1;
        *var_xg1_dn10_slot = var_xg1_dn10;
        *var_xg1_dn11_slot = var_xg1_dn11;
        *var_xg1_dn12_slot = var_xg1_dn12;
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
        *var_xg1_rdn11_slot = var_xg1_rdn11;
        *var_xg1_rdn12_slot = var_xg1_rdn12;
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
        *var_xg2_dn11_slot = var_xg2_dn11;
        *var_xg2_dn12_slot = var_xg2_dn12;
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
        *var_xg2_rdn11_slot = var_xg2_rdn11;
        *var_xg2_rdn12_slot = var_xg2_rdn12;
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
        *var_xnbex_dn11_slot = var_xnbex_dn11;
        *var_xnbex_dn12_slot = var_xnbex_dn12;
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
        *var_xnbex_rdn11_slot = var_xnbex_rdn11;
        *var_xnbex_rdn12_slot = var_xnbex_rdn12;
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
        *var_xpwex_dn11_slot = var_xpwex_dn11;
        *var_xpwex_dn12_slot = var_xpwex_dn12;
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
        *var_xpwex_rdn11_slot = var_xpwex_rdn11;
        *var_xpwex_rdn12_slot = var_xpwex_rdn12;
        *var_xpwex_rdn2_slot = var_xpwex_rdn2;
        *var_xpwex_rdn3_slot = var_xpwex_rdn3;
        *var_xpwex_rdn4_slot = var_xpwex_rdn4;
        *var_xpwex_rdn5_slot = var_xpwex_rdn5;
        *var_xpwex_rdn6_slot = var_xpwex_rdn6;
        *var_xpwex_rdn7_slot = var_xpwex_rdn7;
        *var_xpwex_rdn8_slot = var_xpwex_rdn8;
        *var_xpwex_rdn9_slot = var_xpwex_rdn9;
        *var_xpwex_rv_slot = var_xpwex_rv;
        *var_xqex_slot = var_xqex;
        *var_xqex_db0_slot = var_xqex_db0;
        *var_xqex_db1_slot = var_xqex_db1;
        *var_xqex_dn0_slot = var_xqex_dn0;
        *var_xqex_dn1_slot = var_xqex_dn1;
        *var_xqex_dn10_slot = var_xqex_dn10;
        *var_xqex_dn11_slot = var_xqex_dn11;
        *var_xqex_dn12_slot = var_xqex_dn12;
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
        *var_xqex_rdn11_slot = var_xqex_rdn11;
        *var_xqex_rdn12_slot = var_xqex_rdn12;
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
        *var_xqmex_dn11_slot = var_xqmex_dn11;
        *var_xqmex_dn12_slot = var_xqmex_dn12;
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
        *var_xqmex_rdn11_slot = var_xqmex_rdn11;
        *var_xqmex_rdn12_slot = var_xqmex_rdn12;
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

    pub(super) fn stamp_reactive_block_43(
        p: &Parameters,
        var_a_vde: f64,
        var_a_vde_db0: f64,
        var_a_vde_db1: f64,
        var_a_vde_dn0: f64,
        var_a_vde_dn1: f64,
        var_a_vde_dn10: f64,
        var_a_vde_dn11: f64,
        var_a_vde_dn12: f64,
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
        var_cje_t_dn12: f64,
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
        var_evb2e1_dn12: f64,
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
        var_f1_dn12: f64,
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
        var_if0_dn12: f64,
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
        var_inv_vde_t_dn12: f64,
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
        var_nff_t_dn12: f64,
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
        var_q1q_dn12: f64,
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
        var_qb0_dn12: f64,
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
        var_qe_qs_dn12: f64,
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
        var_vb1b2_dn12: f64,
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
        var_vb2e1_dn12: f64,
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
        var_vfe_dn12: f64,
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
        var_vje_dn12: f64,
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
        var_vt_dn12: f64,
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
        var_vtinv_dn12: f64,
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
        var_dn0vb2e1_dn12_slot: &mut f64,
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
        var_dn0vb2e1_rdn12_slot: &mut f64,
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
        var_dqbevb2e1_dn12_slot: &mut f64,
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
        var_dqbevb2e1_rdn12_slot: &mut f64,
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
        var_dqevb2e1_dn12_slot: &mut f64,
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
        var_dqevb2e1_rdn12_slot: &mut f64,
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
        var_dqtevb2e1_dn12_slot: &mut f64,
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
        var_dqtevb2e1_rdn12_slot: &mut f64,
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
        var_dvjevb2e1_dn12_slot: &mut f64,
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
        var_dvjevb2e1_rdn12_slot: &mut f64,
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
        var_dvtevb2e1_dn12_slot: &mut f64,
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
        var_dvtevb2e1_rdn12_slot: &mut f64,
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
        var_dvtevje_dn12_slot: &mut f64,
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
        var_dvtevje_rdn12_slot: &mut f64,
        var_dvtevje_rdn2_slot: &mut f64,
        var_dvtevje_rdn3_slot: &mut f64,
        var_dvtevje_rdn4_slot: &mut f64,
        var_dvtevje_rdn5_slot: &mut f64,
        var_dvtevje_rdn6_slot: &mut f64,
        var_dvtevje_rdn7_slot: &mut f64,
        var_dvtevje_rdn8_slot: &mut f64,
        var_dvtevje_rdn9_slot: &mut f64,
        var_dvtevje_rv_slot: &mut f64,
        var_guard123_slot: &mut f64,
        var_guard123_db0_slot: &mut f64,
        var_guard123_db1_slot: &mut f64,
        var_guard123_dn0_slot: &mut f64,
        var_guard123_dn1_slot: &mut f64,
        var_guard123_dn10_slot: &mut f64,
        var_guard123_dn11_slot: &mut f64,
        var_guard123_dn12_slot: &mut f64,
        var_guard123_dn2_slot: &mut f64,
        var_guard123_dn3_slot: &mut f64,
        var_guard123_dn4_slot: &mut f64,
        var_guard123_dn5_slot: &mut f64,
        var_guard123_dn6_slot: &mut f64,
        var_guard123_dn7_slot: &mut f64,
        var_guard123_dn8_slot: &mut f64,
        var_guard123_dn9_slot: &mut f64,
        var_guard123_rdb0_slot: &mut f64,
        var_guard123_rdb1_slot: &mut f64,
        var_guard123_rdn0_slot: &mut f64,
        var_guard123_rdn1_slot: &mut f64,
        var_guard123_rdn10_slot: &mut f64,
        var_guard123_rdn11_slot: &mut f64,
        var_guard123_rdn12_slot: &mut f64,
        var_guard123_rdn2_slot: &mut f64,
        var_guard123_rdn3_slot: &mut f64,
        var_guard123_rdn4_slot: &mut f64,
        var_guard123_rdn5_slot: &mut f64,
        var_guard123_rdn6_slot: &mut f64,
        var_guard123_rdn7_slot: &mut f64,
        var_guard123_rdn8_slot: &mut f64,
        var_guard123_rdn9_slot: &mut f64,
        var_guard123_rv_slot: &mut f64,
        var_guard124_slot: &mut f64,
        var_guard124_db0_slot: &mut f64,
        var_guard124_db1_slot: &mut f64,
        var_guard124_dn0_slot: &mut f64,
        var_guard124_dn1_slot: &mut f64,
        var_guard124_dn10_slot: &mut f64,
        var_guard124_dn11_slot: &mut f64,
        var_guard124_dn12_slot: &mut f64,
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
        var_guard124_rdn12_slot: &mut f64,
        var_guard124_rdn2_slot: &mut f64,
        var_guard124_rdn3_slot: &mut f64,
        var_guard124_rdn4_slot: &mut f64,
        var_guard124_rdn5_slot: &mut f64,
        var_guard124_rdn6_slot: &mut f64,
        var_guard124_rdn7_slot: &mut f64,
        var_guard124_rdn8_slot: &mut f64,
        var_guard124_rdn9_slot: &mut f64,
        var_guard124_rv_slot: &mut f64,
        var_qb1b2_slot: &mut f64,
        var_qb1b2_db0_slot: &mut f64,
        var_qb1b2_db1_slot: &mut f64,
        var_qb1b2_dn0_slot: &mut f64,
        var_qb1b2_dn1_slot: &mut f64,
        var_qb1b2_dn10_slot: &mut f64,
        var_qb1b2_dn11_slot: &mut f64,
        var_qb1b2_dn12_slot: &mut f64,
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
        var_qb1b2_rdn12_slot: &mut f64,
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
        var_vb2e1vfe_dn12_slot: &mut f64,
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
        var_vb2e1vfe_rdn12_slot: &mut f64,
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
        let mut var_dn0vb2e1_dn12: f64 = *var_dn0vb2e1_dn12_slot;
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
        let mut var_dn0vb2e1_rdn12: f64 = *var_dn0vb2e1_rdn12_slot;
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
        let mut var_dqbevb2e1_dn12: f64 = *var_dqbevb2e1_dn12_slot;
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
        let mut var_dqbevb2e1_rdn12: f64 = *var_dqbevb2e1_rdn12_slot;
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
        let mut var_dqevb2e1_dn12: f64 = *var_dqevb2e1_dn12_slot;
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
        let mut var_dqevb2e1_rdn12: f64 = *var_dqevb2e1_rdn12_slot;
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
        let mut var_dqtevb2e1_dn12: f64 = *var_dqtevb2e1_dn12_slot;
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
        let mut var_dqtevb2e1_rdn12: f64 = *var_dqtevb2e1_rdn12_slot;
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
        let mut var_dvjevb2e1_dn12: f64 = *var_dvjevb2e1_dn12_slot;
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
        let mut var_dvjevb2e1_rdn12: f64 = *var_dvjevb2e1_rdn12_slot;
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
        let mut var_dvtevb2e1_dn12: f64 = *var_dvtevb2e1_dn12_slot;
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
        let mut var_dvtevb2e1_rdn12: f64 = *var_dvtevb2e1_rdn12_slot;
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
        let mut var_dvtevje_dn12: f64 = *var_dvtevje_dn12_slot;
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
        let mut var_dvtevje_rdn12: f64 = *var_dvtevje_rdn12_slot;
        let mut var_dvtevje_rdn2: f64 = *var_dvtevje_rdn2_slot;
        let mut var_dvtevje_rdn3: f64 = *var_dvtevje_rdn3_slot;
        let mut var_dvtevje_rdn4: f64 = *var_dvtevje_rdn4_slot;
        let mut var_dvtevje_rdn5: f64 = *var_dvtevje_rdn5_slot;
        let mut var_dvtevje_rdn6: f64 = *var_dvtevje_rdn6_slot;
        let mut var_dvtevje_rdn7: f64 = *var_dvtevje_rdn7_slot;
        let mut var_dvtevje_rdn8: f64 = *var_dvtevje_rdn8_slot;
        let mut var_dvtevje_rdn9: f64 = *var_dvtevje_rdn9_slot;
        let mut var_dvtevje_rv: f64 = *var_dvtevje_rv_slot;
        let mut var_guard123: f64 = *var_guard123_slot;
        let mut var_guard123_db0: f64 = *var_guard123_db0_slot;
        let mut var_guard123_db1: f64 = *var_guard123_db1_slot;
        let mut var_guard123_dn0: f64 = *var_guard123_dn0_slot;
        let mut var_guard123_dn1: f64 = *var_guard123_dn1_slot;
        let mut var_guard123_dn10: f64 = *var_guard123_dn10_slot;
        let mut var_guard123_dn11: f64 = *var_guard123_dn11_slot;
        let mut var_guard123_dn12: f64 = *var_guard123_dn12_slot;
        let mut var_guard123_dn2: f64 = *var_guard123_dn2_slot;
        let mut var_guard123_dn3: f64 = *var_guard123_dn3_slot;
        let mut var_guard123_dn4: f64 = *var_guard123_dn4_slot;
        let mut var_guard123_dn5: f64 = *var_guard123_dn5_slot;
        let mut var_guard123_dn6: f64 = *var_guard123_dn6_slot;
        let mut var_guard123_dn7: f64 = *var_guard123_dn7_slot;
        let mut var_guard123_dn8: f64 = *var_guard123_dn8_slot;
        let mut var_guard123_dn9: f64 = *var_guard123_dn9_slot;
        let mut var_guard123_rdb0: f64 = *var_guard123_rdb0_slot;
        let mut var_guard123_rdb1: f64 = *var_guard123_rdb1_slot;
        let mut var_guard123_rdn0: f64 = *var_guard123_rdn0_slot;
        let mut var_guard123_rdn1: f64 = *var_guard123_rdn1_slot;
        let mut var_guard123_rdn10: f64 = *var_guard123_rdn10_slot;
        let mut var_guard123_rdn11: f64 = *var_guard123_rdn11_slot;
        let mut var_guard123_rdn12: f64 = *var_guard123_rdn12_slot;
        let mut var_guard123_rdn2: f64 = *var_guard123_rdn2_slot;
        let mut var_guard123_rdn3: f64 = *var_guard123_rdn3_slot;
        let mut var_guard123_rdn4: f64 = *var_guard123_rdn4_slot;
        let mut var_guard123_rdn5: f64 = *var_guard123_rdn5_slot;
        let mut var_guard123_rdn6: f64 = *var_guard123_rdn6_slot;
        let mut var_guard123_rdn7: f64 = *var_guard123_rdn7_slot;
        let mut var_guard123_rdn8: f64 = *var_guard123_rdn8_slot;
        let mut var_guard123_rdn9: f64 = *var_guard123_rdn9_slot;
        let mut var_guard123_rv: f64 = *var_guard123_rv_slot;
        let mut var_guard124: f64 = *var_guard124_slot;
        let mut var_guard124_db0: f64 = *var_guard124_db0_slot;
        let mut var_guard124_db1: f64 = *var_guard124_db1_slot;
        let mut var_guard124_dn0: f64 = *var_guard124_dn0_slot;
        let mut var_guard124_dn1: f64 = *var_guard124_dn1_slot;
        let mut var_guard124_dn10: f64 = *var_guard124_dn10_slot;
        let mut var_guard124_dn11: f64 = *var_guard124_dn11_slot;
        let mut var_guard124_dn12: f64 = *var_guard124_dn12_slot;
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
        let mut var_guard124_rdn12: f64 = *var_guard124_rdn12_slot;
        let mut var_guard124_rdn2: f64 = *var_guard124_rdn2_slot;
        let mut var_guard124_rdn3: f64 = *var_guard124_rdn3_slot;
        let mut var_guard124_rdn4: f64 = *var_guard124_rdn4_slot;
        let mut var_guard124_rdn5: f64 = *var_guard124_rdn5_slot;
        let mut var_guard124_rdn6: f64 = *var_guard124_rdn6_slot;
        let mut var_guard124_rdn7: f64 = *var_guard124_rdn7_slot;
        let mut var_guard124_rdn8: f64 = *var_guard124_rdn8_slot;
        let mut var_guard124_rdn9: f64 = *var_guard124_rdn9_slot;
        let mut var_guard124_rv: f64 = *var_guard124_rv_slot;
        let mut var_qb1b2: f64 = *var_qb1b2_slot;
        let mut var_qb1b2_db0: f64 = *var_qb1b2_db0_slot;
        let mut var_qb1b2_db1: f64 = *var_qb1b2_db1_slot;
        let mut var_qb1b2_dn0: f64 = *var_qb1b2_dn0_slot;
        let mut var_qb1b2_dn1: f64 = *var_qb1b2_dn1_slot;
        let mut var_qb1b2_dn10: f64 = *var_qb1b2_dn10_slot;
        let mut var_qb1b2_dn11: f64 = *var_qb1b2_dn11_slot;
        let mut var_qb1b2_dn12: f64 = *var_qb1b2_dn12_slot;
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
        let mut var_qb1b2_rdn12: f64 = *var_qb1b2_rdn12_slot;
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
        let mut var_vb2e1vfe_dn12: f64 = *var_vb2e1vfe_dn12_slot;
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
        let mut var_vb2e1vfe_rdn12: f64 = *var_vb2e1vfe_rdn12_slot;
        let mut var_vb2e1vfe_rdn2: f64 = *var_vb2e1vfe_rdn2_slot;
        let mut var_vb2e1vfe_rdn3: f64 = *var_vb2e1vfe_rdn3_slot;
        let mut var_vb2e1vfe_rdn4: f64 = *var_vb2e1vfe_rdn4_slot;
        let mut var_vb2e1vfe_rdn5: f64 = *var_vb2e1vfe_rdn5_slot;
        let mut var_vb2e1vfe_rdn6: f64 = *var_vb2e1vfe_rdn6_slot;
        let mut var_vb2e1vfe_rdn7: f64 = *var_vb2e1vfe_rdn7_slot;
        let mut var_vb2e1vfe_rdn8: f64 = *var_vb2e1vfe_rdn8_slot;
        let mut var_vb2e1vfe_rdn9: f64 = *var_vb2e1vfe_rdn9_slot;
        let mut var_vb2e1vfe_rv: f64 = *var_vb2e1vfe_rv_slot;

        let assign6710_e7006: f64 = if p.p6 == 1.0 { 1.0 } else { 0.0 };
        var_guard123 = assign6710_e7006;
        var_guard123_dn0 = 0.0;
        var_guard123_dn1 = 0.0;
        var_guard123_dn2 = 0.0;
        var_guard123_dn3 = 0.0;
        var_guard123_dn4 = 0.0;
        var_guard123_dn5 = 0.0;
        var_guard123_dn6 = 0.0;
        var_guard123_dn7 = 0.0;
        var_guard123_dn8 = 0.0;
        var_guard123_dn9 = 0.0;
        var_guard123_dn10 = 0.0;
        var_guard123_dn11 = 0.0;
        var_guard123_dn12 = 0.0;
        var_guard123_db0 = 0.0;
        var_guard123_db1 = 0.0;
        var_guard123_rv = 0.0;
        var_guard123_rdn0 = 0.0;
        var_guard123_rdn1 = 0.0;
        var_guard123_rdn2 = 0.0;
        var_guard123_rdn3 = 0.0;
        var_guard123_rdn4 = 0.0;
        var_guard123_rdn5 = 0.0;
        var_guard123_rdn6 = 0.0;
        var_guard123_rdn7 = 0.0;
        var_guard123_rdn8 = 0.0;
        var_guard123_rdn9 = 0.0;
        var_guard123_rdn10 = 0.0;
        var_guard123_rdn11 = 0.0;
        var_guard123_rdn12 = 0.0;
        var_guard123_rdb0 = 0.0;
        var_guard123_rdb1 = 0.0;

        let (assign6720_e7019, assign6720_e7019_d_n0, assign6720_e7019_d_n1, assign6720_e7019_d_n2, assign6720_e7019_d_n3, assign6720_e7019_d_n4, assign6720_e7019_d_n5, assign6720_e7019_d_n6, assign6720_e7019_d_n7, assign6720_e7019_d_n8, assign6720_e7019_d_n9, assign6720_e7019_d_n10, assign6720_e7019_d_n11, assign6720_e7019_d_n12, assign6720_e7019_d_b0, assign6720_e7019_d_b1,) = {
    if (var_guard123 != 0.0) {
        let assign6720_e7011: f64 = (var_vje * var_inv_vde_t);
        let assign6720_e7012: f64 = (1.0 - assign6720_e7011);
        let assign6720_e7014: f64 = (-p.p67);
        let assign6720_e7015: f64 = (assign6720_e7012).powf(assign6720_e7014);
        let assign6720_e7017: f64 = (assign6720_e7015 - 3.0);
        (assign6720_e7017, if 0.0 == 0.0 && ((assign6720_e7014) as f64).is_finite() && ((assign6720_e7014) as f64).fract() == 0.0 { if assign6720_e7014 == 0.0 { 0.0 } else { (assign6720_e7014 * ((assign6720_e7012).powf(assign6720_e7014 - 1.0) * (-((var_vje_dn0 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn0))))) } } else { (assign6720_e7015 * (assign6720_e7014 * ((-((var_vje_dn0 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn0))) / assign6720_e7012))) }, if 0.0 == 0.0 && ((assign6720_e7014) as f64).is_finite() && ((assign6720_e7014) as f64).fract() == 0.0 { if assign6720_e7014 == 0.0 { 0.0 } else { (assign6720_e7014 * ((assign6720_e7012).powf(assign6720_e7014 - 1.0) * (-((var_vje_dn1 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn1))))) } } else { (assign6720_e7015 * (assign6720_e7014 * ((-((var_vje_dn1 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn1))) / assign6720_e7012))) }, if 0.0 == 0.0 && ((assign6720_e7014) as f64).is_finite() && ((assign6720_e7014) as f64).fract() == 0.0 { if assign6720_e7014 == 0.0 { 0.0 } else { (assign6720_e7014 * ((assign6720_e7012).powf(assign6720_e7014 - 1.0) * (-((var_vje_dn2 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn2))))) } } else { (assign6720_e7015 * (assign6720_e7014 * ((-((var_vje_dn2 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn2))) / assign6720_e7012))) }, if 0.0 == 0.0 && ((assign6720_e7014) as f64).is_finite() && ((assign6720_e7014) as f64).fract() == 0.0 { if assign6720_e7014 == 0.0 { 0.0 } else { (assign6720_e7014 * ((assign6720_e7012).powf(assign6720_e7014 - 1.0) * (-((var_vje_dn3 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn3))))) } } else { (assign6720_e7015 * (assign6720_e7014 * ((-((var_vje_dn3 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn3))) / assign6720_e7012))) }, if 0.0 == 0.0 && ((assign6720_e7014) as f64).is_finite() && ((assign6720_e7014) as f64).fract() == 0.0 { if assign6720_e7014 == 0.0 { 0.0 } else { (assign6720_e7014 * ((assign6720_e7012).powf(assign6720_e7014 - 1.0) * (-((var_vje_dn4 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn4))))) } } else { (assign6720_e7015 * (assign6720_e7014 * ((-((var_vje_dn4 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn4))) / assign6720_e7012))) }, if 0.0 == 0.0 && ((assign6720_e7014) as f64).is_finite() && ((assign6720_e7014) as f64).fract() == 0.0 { if assign6720_e7014 == 0.0 { 0.0 } else { (assign6720_e7014 * ((assign6720_e7012).powf(assign6720_e7014 - 1.0) * (-((var_vje_dn5 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn5))))) } } else { (assign6720_e7015 * (assign6720_e7014 * ((-((var_vje_dn5 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn5))) / assign6720_e7012))) }, if 0.0 == 0.0 && ((assign6720_e7014) as f64).is_finite() && ((assign6720_e7014) as f64).fract() == 0.0 { if assign6720_e7014 == 0.0 { 0.0 } else { (assign6720_e7014 * ((assign6720_e7012).powf(assign6720_e7014 - 1.0) * (-((var_vje_dn6 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn6))))) } } else { (assign6720_e7015 * (assign6720_e7014 * ((-((var_vje_dn6 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn6))) / assign6720_e7012))) }, if 0.0 == 0.0 && ((assign6720_e7014) as f64).is_finite() && ((assign6720_e7014) as f64).fract() == 0.0 { if assign6720_e7014 == 0.0 { 0.0 } else { (assign6720_e7014 * ((assign6720_e7012).powf(assign6720_e7014 - 1.0) * (-((var_vje_dn7 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn7))))) } } else { (assign6720_e7015 * (assign6720_e7014 * ((-((var_vje_dn7 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn7))) / assign6720_e7012))) }, if 0.0 == 0.0 && ((assign6720_e7014) as f64).is_finite() && ((assign6720_e7014) as f64).fract() == 0.0 { if assign6720_e7014 == 0.0 { 0.0 } else { (assign6720_e7014 * ((assign6720_e7012).powf(assign6720_e7014 - 1.0) * (-((var_vje_dn8 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn8))))) } } else { (assign6720_e7015 * (assign6720_e7014 * ((-((var_vje_dn8 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn8))) / assign6720_e7012))) }, if 0.0 == 0.0 && ((assign6720_e7014) as f64).is_finite() && ((assign6720_e7014) as f64).fract() == 0.0 { if assign6720_e7014 == 0.0 { 0.0 } else { (assign6720_e7014 * ((assign6720_e7012).powf(assign6720_e7014 - 1.0) * (-((var_vje_dn9 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn9))))) } } else { (assign6720_e7015 * (assign6720_e7014 * ((-((var_vje_dn9 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn9))) / assign6720_e7012))) }, if 0.0 == 0.0 && ((assign6720_e7014) as f64).is_finite() && ((assign6720_e7014) as f64).fract() == 0.0 { if assign6720_e7014 == 0.0 { 0.0 } else { (assign6720_e7014 * ((assign6720_e7012).powf(assign6720_e7014 - 1.0) * (-((var_vje_dn10 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn10))))) } } else { (assign6720_e7015 * (assign6720_e7014 * ((-((var_vje_dn10 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn10))) / assign6720_e7012))) }, if 0.0 == 0.0 && ((assign6720_e7014) as f64).is_finite() && ((assign6720_e7014) as f64).fract() == 0.0 { if assign6720_e7014 == 0.0 { 0.0 } else { (assign6720_e7014 * ((assign6720_e7012).powf(assign6720_e7014 - 1.0) * (-((var_vje_dn11 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn11))))) } } else { (assign6720_e7015 * (assign6720_e7014 * ((-((var_vje_dn11 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn11))) / assign6720_e7012))) }, if 0.0 == 0.0 && ((assign6720_e7014) as f64).is_finite() && ((assign6720_e7014) as f64).fract() == 0.0 { if assign6720_e7014 == 0.0 { 0.0 } else { (assign6720_e7014 * ((assign6720_e7012).powf(assign6720_e7014 - 1.0) * (-((var_vje_dn12 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn12))))) } } else { (assign6720_e7015 * (assign6720_e7014 * ((-((var_vje_dn12 * var_inv_vde_t) + (var_vje * var_inv_vde_t_dn12))) / assign6720_e7012))) }, if 0.0 == 0.0 && ((assign6720_e7014) as f64).is_finite() && ((assign6720_e7014) as f64).fract() == 0.0 { if assign6720_e7014 == 0.0 { 0.0 } else { (assign6720_e7014 * ((assign6720_e7012).powf(assign6720_e7014 - 1.0) * (-((var_vje_db0 * var_inv_vde_t) + (var_vje * var_inv_vde_t_db0))))) } } else { (assign6720_e7015 * (assign6720_e7014 * ((-((var_vje_db0 * var_inv_vde_t) + (var_vje * var_inv_vde_t_db0))) / assign6720_e7012))) }, if 0.0 == 0.0 && ((assign6720_e7014) as f64).is_finite() && ((assign6720_e7014) as f64).fract() == 0.0 { if assign6720_e7014 == 0.0 { 0.0 } else { (assign6720_e7014 * ((assign6720_e7012).powf(assign6720_e7014 - 1.0) * (-((var_vje_db1 * var_inv_vde_t) + (var_vje * var_inv_vde_t_db1))))) } } else { (assign6720_e7015 * (assign6720_e7014 * ((-((var_vje_db1 * var_inv_vde_t) + (var_vje * var_inv_vde_t_db1))) / assign6720_e7012))) },)
    } else {
        (var_dvtevje, var_dvtevje_dn0, var_dvtevje_dn1, var_dvtevje_dn2, var_dvtevje_dn3, var_dvtevje_dn4, var_dvtevje_dn5, var_dvtevje_dn6, var_dvtevje_dn7, var_dvtevje_dn8, var_dvtevje_dn9, var_dvtevje_dn10, var_dvtevje_dn11, var_dvtevje_dn12, var_dvtevje_db0, var_dvtevje_db1,)
    }
};
        var_dvtevje = assign6720_e7019;
        var_dvtevje_dn0 = assign6720_e7019_d_n0;
        var_dvtevje_dn1 = assign6720_e7019_d_n1;
        var_dvtevje_dn2 = assign6720_e7019_d_n2;
        var_dvtevje_dn3 = assign6720_e7019_d_n3;
        var_dvtevje_dn4 = assign6720_e7019_d_n4;
        var_dvtevje_dn5 = assign6720_e7019_d_n5;
        var_dvtevje_dn6 = assign6720_e7019_d_n6;
        var_dvtevje_dn7 = assign6720_e7019_d_n7;
        var_dvtevje_dn8 = assign6720_e7019_d_n8;
        var_dvtevje_dn9 = assign6720_e7019_d_n9;
        var_dvtevje_dn10 = assign6720_e7019_d_n10;
        var_dvtevje_dn11 = assign6720_e7019_d_n11;
        var_dvtevje_dn12 = assign6720_e7019_d_n12;
        var_dvtevje_db0 = assign6720_e7019_d_b0;
        var_dvtevje_db1 = assign6720_e7019_d_b1;
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
        var_dvtevje_rdn12 = 0.0;
        var_dvtevje_rdb0 = 0.0;
        var_dvtevje_rdb1 = 0.0;

        let (assign6730_e7027, assign6730_e7027_d_n0, assign6730_e7027_d_n1, assign6730_e7027_d_n2, assign6730_e7027_d_n3, assign6730_e7027_d_n4, assign6730_e7027_d_n5, assign6730_e7027_d_n6, assign6730_e7027_d_n7, assign6730_e7027_d_n8, assign6730_e7027_d_n9, assign6730_e7027_d_n10, assign6730_e7027_d_n11, assign6730_e7027_d_n12, assign6730_e7027_d_b0, assign6730_e7027_d_b1,) = {
    if (var_guard123 != 0.0) {
        let assign6730_e7023: f64 = (var_vb2e1 - var_vfe);
        let assign6730_e7025: f64 = (assign6730_e7023 / var_a_vde);
        (assign6730_e7025, ((((var_vb2e1_dn0 - var_vfe_dn0) * var_a_vde) - (assign6730_e7023 * var_a_vde_dn0)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn1 - var_vfe_dn1) * var_a_vde) - (assign6730_e7023 * var_a_vde_dn1)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn2 - var_vfe_dn2) * var_a_vde) - (assign6730_e7023 * var_a_vde_dn2)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn3 - var_vfe_dn3) * var_a_vde) - (assign6730_e7023 * var_a_vde_dn3)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn4 - var_vfe_dn4) * var_a_vde) - (assign6730_e7023 * var_a_vde_dn4)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn5 - var_vfe_dn5) * var_a_vde) - (assign6730_e7023 * var_a_vde_dn5)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn6 - var_vfe_dn6) * var_a_vde) - (assign6730_e7023 * var_a_vde_dn6)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn7 - var_vfe_dn7) * var_a_vde) - (assign6730_e7023 * var_a_vde_dn7)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn8 - var_vfe_dn8) * var_a_vde) - (assign6730_e7023 * var_a_vde_dn8)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn9 - var_vfe_dn9) * var_a_vde) - (assign6730_e7023 * var_a_vde_dn9)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn10 - var_vfe_dn10) * var_a_vde) - (assign6730_e7023 * var_a_vde_dn10)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn11 - var_vfe_dn11) * var_a_vde) - (assign6730_e7023 * var_a_vde_dn11)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_dn12 - var_vfe_dn12) * var_a_vde) - (assign6730_e7023 * var_a_vde_dn12)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_db0 - var_vfe_db0) * var_a_vde) - (assign6730_e7023 * var_a_vde_db0)) / (var_a_vde * var_a_vde)), ((((var_vb2e1_db1 - var_vfe_db1) * var_a_vde) - (assign6730_e7023 * var_a_vde_db1)) / (var_a_vde * var_a_vde)),)
    } else {
        (var_vb2e1vfe, var_vb2e1vfe_dn0, var_vb2e1vfe_dn1, var_vb2e1vfe_dn2, var_vb2e1vfe_dn3, var_vb2e1vfe_dn4, var_vb2e1vfe_dn5, var_vb2e1vfe_dn6, var_vb2e1vfe_dn7, var_vb2e1vfe_dn8, var_vb2e1vfe_dn9, var_vb2e1vfe_dn10, var_vb2e1vfe_dn11, var_vb2e1vfe_dn12, var_vb2e1vfe_db0, var_vb2e1vfe_db1,)
    }
};
        var_vb2e1vfe = assign6730_e7027;
        var_vb2e1vfe_dn0 = assign6730_e7027_d_n0;
        var_vb2e1vfe_dn1 = assign6730_e7027_d_n1;
        var_vb2e1vfe_dn2 = assign6730_e7027_d_n2;
        var_vb2e1vfe_dn3 = assign6730_e7027_d_n3;
        var_vb2e1vfe_dn4 = assign6730_e7027_d_n4;
        var_vb2e1vfe_dn5 = assign6730_e7027_d_n5;
        var_vb2e1vfe_dn6 = assign6730_e7027_d_n6;
        var_vb2e1vfe_dn7 = assign6730_e7027_d_n7;
        var_vb2e1vfe_dn8 = assign6730_e7027_d_n8;
        var_vb2e1vfe_dn9 = assign6730_e7027_d_n9;
        var_vb2e1vfe_dn10 = assign6730_e7027_d_n10;
        var_vb2e1vfe_dn11 = assign6730_e7027_d_n11;
        var_vb2e1vfe_dn12 = assign6730_e7027_d_n12;
        var_vb2e1vfe_db0 = assign6730_e7027_d_b0;
        var_vb2e1vfe_db1 = assign6730_e7027_d_b1;
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
        var_vb2e1vfe_rdn12 = 0.0;
        var_vb2e1vfe_rdb0 = 0.0;
        var_vb2e1vfe_rdb1 = 0.0;

        let assign6740_e7030: f64 = if var_vb2e1vfe < 0.0 { 1.0 } else { 0.0 };
        var_guard124 = assign6740_e7030;
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
        var_guard124_dn12 = 0.0;
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
        var_guard124_rdn12 = 0.0;
        var_guard124_rdb0 = 0.0;
        var_guard124_rdb1 = 0.0;

        let (assign6750_e7041, assign6750_e7041_d_n0, assign6750_e7041_d_n1, assign6750_e7041_d_n2, assign6750_e7041_d_n3, assign6750_e7041_d_n4, assign6750_e7041_d_n5, assign6750_e7041_d_n6, assign6750_e7041_d_n7, assign6750_e7041_d_n8, assign6750_e7041_d_n9, assign6750_e7041_d_n10, assign6750_e7041_d_n11, assign6750_e7041_d_n12, assign6750_e7041_d_b0, assign6750_e7041_d_b1,) = {
    if ((var_guard123 != 0.0) && (var_guard124 != 0.0)) {
        let assign6750_e7037: f64 = (var_vb2e1vfe).exp();
        let assign6750_e7038: f64 = (1.0 + assign6750_e7037);
        let assign6750_e7039: f64 = (1.0 / assign6750_e7038);
        (assign6750_e7039, (-((assign6750_e7037 * var_vb2e1vfe_dn0) / (assign6750_e7038 * assign6750_e7038))), (-((assign6750_e7037 * var_vb2e1vfe_dn1) / (assign6750_e7038 * assign6750_e7038))), (-((assign6750_e7037 * var_vb2e1vfe_dn2) / (assign6750_e7038 * assign6750_e7038))), (-((assign6750_e7037 * var_vb2e1vfe_dn3) / (assign6750_e7038 * assign6750_e7038))), (-((assign6750_e7037 * var_vb2e1vfe_dn4) / (assign6750_e7038 * assign6750_e7038))), (-((assign6750_e7037 * var_vb2e1vfe_dn5) / (assign6750_e7038 * assign6750_e7038))), (-((assign6750_e7037 * var_vb2e1vfe_dn6) / (assign6750_e7038 * assign6750_e7038))), (-((assign6750_e7037 * var_vb2e1vfe_dn7) / (assign6750_e7038 * assign6750_e7038))), (-((assign6750_e7037 * var_vb2e1vfe_dn8) / (assign6750_e7038 * assign6750_e7038))), (-((assign6750_e7037 * var_vb2e1vfe_dn9) / (assign6750_e7038 * assign6750_e7038))), (-((assign6750_e7037 * var_vb2e1vfe_dn10) / (assign6750_e7038 * assign6750_e7038))), (-((assign6750_e7037 * var_vb2e1vfe_dn11) / (assign6750_e7038 * assign6750_e7038))), (-((assign6750_e7037 * var_vb2e1vfe_dn12) / (assign6750_e7038 * assign6750_e7038))), (-((assign6750_e7037 * var_vb2e1vfe_db0) / (assign6750_e7038 * assign6750_e7038))), (-((assign6750_e7037 * var_vb2e1vfe_db1) / (assign6750_e7038 * assign6750_e7038))),)
    } else {
        (var_dvjevb2e1, var_dvjevb2e1_dn0, var_dvjevb2e1_dn1, var_dvjevb2e1_dn2, var_dvjevb2e1_dn3, var_dvjevb2e1_dn4, var_dvjevb2e1_dn5, var_dvjevb2e1_dn6, var_dvjevb2e1_dn7, var_dvjevb2e1_dn8, var_dvjevb2e1_dn9, var_dvjevb2e1_dn10, var_dvjevb2e1_dn11, var_dvjevb2e1_dn12, var_dvjevb2e1_db0, var_dvjevb2e1_db1,)
    }
};
        var_dvjevb2e1 = assign6750_e7041;
        var_dvjevb2e1_dn0 = assign6750_e7041_d_n0;
        var_dvjevb2e1_dn1 = assign6750_e7041_d_n1;
        var_dvjevb2e1_dn2 = assign6750_e7041_d_n2;
        var_dvjevb2e1_dn3 = assign6750_e7041_d_n3;
        var_dvjevb2e1_dn4 = assign6750_e7041_d_n4;
        var_dvjevb2e1_dn5 = assign6750_e7041_d_n5;
        var_dvjevb2e1_dn6 = assign6750_e7041_d_n6;
        var_dvjevb2e1_dn7 = assign6750_e7041_d_n7;
        var_dvjevb2e1_dn8 = assign6750_e7041_d_n8;
        var_dvjevb2e1_dn9 = assign6750_e7041_d_n9;
        var_dvjevb2e1_dn10 = assign6750_e7041_d_n10;
        var_dvjevb2e1_dn11 = assign6750_e7041_d_n11;
        var_dvjevb2e1_dn12 = assign6750_e7041_d_n12;
        var_dvjevb2e1_db0 = assign6750_e7041_d_b0;
        var_dvjevb2e1_db1 = assign6750_e7041_d_b1;
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
        var_dvjevb2e1_rdn12 = 0.0;
        var_dvjevb2e1_rdb0 = 0.0;
        var_dvjevb2e1_rdb1 = 0.0;

        let (assign6760_e7056, assign6760_e7056_d_n0, assign6760_e7056_d_n1, assign6760_e7056_d_n2, assign6760_e7056_d_n3, assign6760_e7056_d_n4, assign6760_e7056_d_n5, assign6760_e7056_d_n6, assign6760_e7056_d_n7, assign6760_e7056_d_n8, assign6760_e7056_d_n9, assign6760_e7056_d_n10, assign6760_e7056_d_n11, assign6760_e7056_d_n12, assign6760_e7056_d_b0, assign6760_e7056_d_b1,) = {
    if ((var_guard123 != 0.0) && (var_guard124 == 0.0)) {
        let assign6760_e7047: f64 = (-var_vb2e1vfe);
        let assign6760_e7048: f64 = (assign6760_e7047).exp();
        let assign6760_e7051: f64 = (-var_vb2e1vfe);
        let assign6760_e7052: f64 = (assign6760_e7051).exp();
        let assign6760_e7053: f64 = (1.0 + assign6760_e7052);
        let assign6760_e7054: f64 = (assign6760_e7048 / assign6760_e7053);
        (assign6760_e7054, ((((assign6760_e7048 * (-var_vb2e1vfe_dn0)) * assign6760_e7053) - (assign6760_e7048 * (assign6760_e7052 * (-var_vb2e1vfe_dn0)))) / (assign6760_e7053 * assign6760_e7053)), ((((assign6760_e7048 * (-var_vb2e1vfe_dn1)) * assign6760_e7053) - (assign6760_e7048 * (assign6760_e7052 * (-var_vb2e1vfe_dn1)))) / (assign6760_e7053 * assign6760_e7053)), ((((assign6760_e7048 * (-var_vb2e1vfe_dn2)) * assign6760_e7053) - (assign6760_e7048 * (assign6760_e7052 * (-var_vb2e1vfe_dn2)))) / (assign6760_e7053 * assign6760_e7053)), ((((assign6760_e7048 * (-var_vb2e1vfe_dn3)) * assign6760_e7053) - (assign6760_e7048 * (assign6760_e7052 * (-var_vb2e1vfe_dn3)))) / (assign6760_e7053 * assign6760_e7053)), ((((assign6760_e7048 * (-var_vb2e1vfe_dn4)) * assign6760_e7053) - (assign6760_e7048 * (assign6760_e7052 * (-var_vb2e1vfe_dn4)))) / (assign6760_e7053 * assign6760_e7053)), ((((assign6760_e7048 * (-var_vb2e1vfe_dn5)) * assign6760_e7053) - (assign6760_e7048 * (assign6760_e7052 * (-var_vb2e1vfe_dn5)))) / (assign6760_e7053 * assign6760_e7053)), ((((assign6760_e7048 * (-var_vb2e1vfe_dn6)) * assign6760_e7053) - (assign6760_e7048 * (assign6760_e7052 * (-var_vb2e1vfe_dn6)))) / (assign6760_e7053 * assign6760_e7053)), ((((assign6760_e7048 * (-var_vb2e1vfe_dn7)) * assign6760_e7053) - (assign6760_e7048 * (assign6760_e7052 * (-var_vb2e1vfe_dn7)))) / (assign6760_e7053 * assign6760_e7053)), ((((assign6760_e7048 * (-var_vb2e1vfe_dn8)) * assign6760_e7053) - (assign6760_e7048 * (assign6760_e7052 * (-var_vb2e1vfe_dn8)))) / (assign6760_e7053 * assign6760_e7053)), ((((assign6760_e7048 * (-var_vb2e1vfe_dn9)) * assign6760_e7053) - (assign6760_e7048 * (assign6760_e7052 * (-var_vb2e1vfe_dn9)))) / (assign6760_e7053 * assign6760_e7053)), ((((assign6760_e7048 * (-var_vb2e1vfe_dn10)) * assign6760_e7053) - (assign6760_e7048 * (assign6760_e7052 * (-var_vb2e1vfe_dn10)))) / (assign6760_e7053 * assign6760_e7053)), ((((assign6760_e7048 * (-var_vb2e1vfe_dn11)) * assign6760_e7053) - (assign6760_e7048 * (assign6760_e7052 * (-var_vb2e1vfe_dn11)))) / (assign6760_e7053 * assign6760_e7053)), ((((assign6760_e7048 * (-var_vb2e1vfe_dn12)) * assign6760_e7053) - (assign6760_e7048 * (assign6760_e7052 * (-var_vb2e1vfe_dn12)))) / (assign6760_e7053 * assign6760_e7053)), ((((assign6760_e7048 * (-var_vb2e1vfe_db0)) * assign6760_e7053) - (assign6760_e7048 * (assign6760_e7052 * (-var_vb2e1vfe_db0)))) / (assign6760_e7053 * assign6760_e7053)), ((((assign6760_e7048 * (-var_vb2e1vfe_db1)) * assign6760_e7053) - (assign6760_e7048 * (assign6760_e7052 * (-var_vb2e1vfe_db1)))) / (assign6760_e7053 * assign6760_e7053)),)
    } else {
        (var_dvjevb2e1, var_dvjevb2e1_dn0, var_dvjevb2e1_dn1, var_dvjevb2e1_dn2, var_dvjevb2e1_dn3, var_dvjevb2e1_dn4, var_dvjevb2e1_dn5, var_dvjevb2e1_dn6, var_dvjevb2e1_dn7, var_dvjevb2e1_dn8, var_dvjevb2e1_dn9, var_dvjevb2e1_dn10, var_dvjevb2e1_dn11, var_dvjevb2e1_dn12, var_dvjevb2e1_db0, var_dvjevb2e1_db1,)
    }
};
        var_dvjevb2e1 = assign6760_e7056;
        var_dvjevb2e1_dn0 = assign6760_e7056_d_n0;
        var_dvjevb2e1_dn1 = assign6760_e7056_d_n1;
        var_dvjevb2e1_dn2 = assign6760_e7056_d_n2;
        var_dvjevb2e1_dn3 = assign6760_e7056_d_n3;
        var_dvjevb2e1_dn4 = assign6760_e7056_d_n4;
        var_dvjevb2e1_dn5 = assign6760_e7056_d_n5;
        var_dvjevb2e1_dn6 = assign6760_e7056_d_n6;
        var_dvjevb2e1_dn7 = assign6760_e7056_d_n7;
        var_dvjevb2e1_dn8 = assign6760_e7056_d_n8;
        var_dvjevb2e1_dn9 = assign6760_e7056_d_n9;
        var_dvjevb2e1_dn10 = assign6760_e7056_d_n10;
        var_dvjevb2e1_dn11 = assign6760_e7056_d_n11;
        var_dvjevb2e1_dn12 = assign6760_e7056_d_n12;
        var_dvjevb2e1_db0 = assign6760_e7056_d_b0;
        var_dvjevb2e1_db1 = assign6760_e7056_d_b1;
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
        var_dvjevb2e1_rdn12 = 0.0;
        var_dvjevb2e1_rdb0 = 0.0;
        var_dvjevb2e1_rdb1 = 0.0;

        let (assign6770_e7064, assign6770_e7064_d_n0, assign6770_e7064_d_n1, assign6770_e7064_d_n2, assign6770_e7064_d_n3, assign6770_e7064_d_n4, assign6770_e7064_d_n5, assign6770_e7064_d_n6, assign6770_e7064_d_n7, assign6770_e7064_d_n8, assign6770_e7064_d_n9, assign6770_e7064_d_n10, assign6770_e7064_d_n11, assign6770_e7064_d_n12, assign6770_e7064_d_b0, assign6770_e7064_d_b1,) = {
    if (var_guard123 != 0.0) {
        let assign6770_e7060: f64 = (var_dvtevje * var_dvjevb2e1);
        let assign6770_e7062: f64 = (assign6770_e7060 + 3.0);
        (assign6770_e7062, ((var_dvtevje_dn0 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn0)), ((var_dvtevje_dn1 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn1)), ((var_dvtevje_dn2 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn2)), ((var_dvtevje_dn3 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn3)), ((var_dvtevje_dn4 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn4)), ((var_dvtevje_dn5 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn5)), ((var_dvtevje_dn6 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn6)), ((var_dvtevje_dn7 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn7)), ((var_dvtevje_dn8 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn8)), ((var_dvtevje_dn9 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn9)), ((var_dvtevje_dn10 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn10)), ((var_dvtevje_dn11 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn11)), ((var_dvtevje_dn12 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_dn12)), ((var_dvtevje_db0 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_db0)), ((var_dvtevje_db1 * var_dvjevb2e1) + (var_dvtevje * var_dvjevb2e1_db1)),)
    } else {
        (var_dvtevb2e1, var_dvtevb2e1_dn0, var_dvtevb2e1_dn1, var_dvtevb2e1_dn2, var_dvtevb2e1_dn3, var_dvtevb2e1_dn4, var_dvtevb2e1_dn5, var_dvtevb2e1_dn6, var_dvtevb2e1_dn7, var_dvtevb2e1_dn8, var_dvtevb2e1_dn9, var_dvtevb2e1_dn10, var_dvtevb2e1_dn11, var_dvtevb2e1_dn12, var_dvtevb2e1_db0, var_dvtevb2e1_db1,)
    }
};
        var_dvtevb2e1 = assign6770_e7064;
        var_dvtevb2e1_dn0 = assign6770_e7064_d_n0;
        var_dvtevb2e1_dn1 = assign6770_e7064_d_n1;
        var_dvtevb2e1_dn2 = assign6770_e7064_d_n2;
        var_dvtevb2e1_dn3 = assign6770_e7064_d_n3;
        var_dvtevb2e1_dn4 = assign6770_e7064_d_n4;
        var_dvtevb2e1_dn5 = assign6770_e7064_d_n5;
        var_dvtevb2e1_dn6 = assign6770_e7064_d_n6;
        var_dvtevb2e1_dn7 = assign6770_e7064_d_n7;
        var_dvtevb2e1_dn8 = assign6770_e7064_d_n8;
        var_dvtevb2e1_dn9 = assign6770_e7064_d_n9;
        var_dvtevb2e1_dn10 = assign6770_e7064_d_n10;
        var_dvtevb2e1_dn11 = assign6770_e7064_d_n11;
        var_dvtevb2e1_dn12 = assign6770_e7064_d_n12;
        var_dvtevb2e1_db0 = assign6770_e7064_d_b0;
        var_dvtevb2e1_db1 = assign6770_e7064_d_b1;
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
        var_dvtevb2e1_rdn12 = 0.0;
        var_dvtevb2e1_rdb0 = 0.0;
        var_dvtevb2e1_rdb1 = 0.0;

        let (assign6780_e7074, assign6780_e7074_d_n0, assign6780_e7074_d_n1, assign6780_e7074_d_n2, assign6780_e7074_d_n3, assign6780_e7074_d_n4, assign6780_e7074_d_n5, assign6780_e7074_d_n6, assign6780_e7074_d_n7, assign6780_e7074_d_n8, assign6780_e7074_d_n9, assign6780_e7074_d_n10, assign6780_e7074_d_n11, assign6780_e7074_d_n12, assign6780_e7074_d_b0, assign6780_e7074_d_b1,) = {
    if (var_guard123 != 0.0) {
        let assign6780_e7068: f64 = (1.0 - p.p68);
        let assign6780_e7070: f64 = (assign6780_e7068 * var_cje_t);
        let assign6780_e7072: f64 = (assign6780_e7070 * var_dvtevb2e1);
        (assign6780_e7072, (((assign6780_e7068 * var_cje_t_dn0) * var_dvtevb2e1) + (assign6780_e7070 * var_dvtevb2e1_dn0)), (((assign6780_e7068 * var_cje_t_dn1) * var_dvtevb2e1) + (assign6780_e7070 * var_dvtevb2e1_dn1)), (((assign6780_e7068 * var_cje_t_dn2) * var_dvtevb2e1) + (assign6780_e7070 * var_dvtevb2e1_dn2)), (((assign6780_e7068 * var_cje_t_dn3) * var_dvtevb2e1) + (assign6780_e7070 * var_dvtevb2e1_dn3)), (((assign6780_e7068 * var_cje_t_dn4) * var_dvtevb2e1) + (assign6780_e7070 * var_dvtevb2e1_dn4)), (((assign6780_e7068 * var_cje_t_dn5) * var_dvtevb2e1) + (assign6780_e7070 * var_dvtevb2e1_dn5)), (((assign6780_e7068 * var_cje_t_dn6) * var_dvtevb2e1) + (assign6780_e7070 * var_dvtevb2e1_dn6)), (((assign6780_e7068 * var_cje_t_dn7) * var_dvtevb2e1) + (assign6780_e7070 * var_dvtevb2e1_dn7)), (((assign6780_e7068 * var_cje_t_dn8) * var_dvtevb2e1) + (assign6780_e7070 * var_dvtevb2e1_dn8)), (((assign6780_e7068 * var_cje_t_dn9) * var_dvtevb2e1) + (assign6780_e7070 * var_dvtevb2e1_dn9)), (((assign6780_e7068 * var_cje_t_dn10) * var_dvtevb2e1) + (assign6780_e7070 * var_dvtevb2e1_dn10)), (((assign6780_e7068 * var_cje_t_dn11) * var_dvtevb2e1) + (assign6780_e7070 * var_dvtevb2e1_dn11)), (((assign6780_e7068 * var_cje_t_dn12) * var_dvtevb2e1) + (assign6780_e7070 * var_dvtevb2e1_dn12)), (((assign6780_e7068 * var_cje_t_db0) * var_dvtevb2e1) + (assign6780_e7070 * var_dvtevb2e1_db0)), (((assign6780_e7068 * var_cje_t_db1) * var_dvtevb2e1) + (assign6780_e7070 * var_dvtevb2e1_db1)),)
    } else {
        (var_dqtevb2e1, var_dqtevb2e1_dn0, var_dqtevb2e1_dn1, var_dqtevb2e1_dn2, var_dqtevb2e1_dn3, var_dqtevb2e1_dn4, var_dqtevb2e1_dn5, var_dqtevb2e1_dn6, var_dqtevb2e1_dn7, var_dqtevb2e1_dn8, var_dqtevb2e1_dn9, var_dqtevb2e1_dn10, var_dqtevb2e1_dn11, var_dqtevb2e1_dn12, var_dqtevb2e1_db0, var_dqtevb2e1_db1,)
    }
};
        var_dqtevb2e1 = assign6780_e7074;
        var_dqtevb2e1_dn0 = assign6780_e7074_d_n0;
        var_dqtevb2e1_dn1 = assign6780_e7074_d_n1;
        var_dqtevb2e1_dn2 = assign6780_e7074_d_n2;
        var_dqtevb2e1_dn3 = assign6780_e7074_d_n3;
        var_dqtevb2e1_dn4 = assign6780_e7074_d_n4;
        var_dqtevb2e1_dn5 = assign6780_e7074_d_n5;
        var_dqtevb2e1_dn6 = assign6780_e7074_d_n6;
        var_dqtevb2e1_dn7 = assign6780_e7074_d_n7;
        var_dqtevb2e1_dn8 = assign6780_e7074_d_n8;
        var_dqtevb2e1_dn9 = assign6780_e7074_d_n9;
        var_dqtevb2e1_dn10 = assign6780_e7074_d_n10;
        var_dqtevb2e1_dn11 = assign6780_e7074_d_n11;
        var_dqtevb2e1_dn12 = assign6780_e7074_d_n12;
        var_dqtevb2e1_db0 = assign6780_e7074_d_b0;
        var_dqtevb2e1_db1 = assign6780_e7074_d_b1;
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
        var_dqtevb2e1_rdn12 = 0.0;
        var_dqtevb2e1_rdb0 = 0.0;
        var_dqtevb2e1_rdb1 = 0.0;

        let (assign6790_e7091, assign6790_e7091_d_n0, assign6790_e7091_d_n1, assign6790_e7091_d_n2, assign6790_e7091_d_n3, assign6790_e7091_d_n4, assign6790_e7091_d_n5, assign6790_e7091_d_n6, assign6790_e7091_d_n7, assign6790_e7091_d_n8, assign6790_e7091_d_n9, assign6790_e7091_d_n10, assign6790_e7091_d_n11, assign6790_e7091_d_n12, assign6790_e7091_d_b0, assign6790_e7091_d_b1,) = {
    if (var_guard123 != 0.0) {
        let assign6790_e7078: f64 = (var_if0 * var_evb2e1);
        let assign6790_e7080: f64 = (assign6790_e7078 * var_vtinv);
        let assign6790_e7082: f64 = (assign6790_e7080 / var_nff_t);
        let assign6790_e7086: f64 = (1.0 + var_f1);
        let assign6790_e7087: f64 = (assign6790_e7086).sqrt();
        let assign6790_e7088: f64 = (0.5 / assign6790_e7087);
        let assign6790_e7089: f64 = (assign6790_e7082 * assign6790_e7088);
        (assign6790_e7089, (((((((((var_if0_dn0 * var_evb2e1) + (var_if0 * var_evb2e1_dn0)) * var_vtinv) + (assign6790_e7078 * var_vtinv_dn0)) * var_nff_t) - (assign6790_e7080 * var_nff_t_dn0)) / (var_nff_t * var_nff_t)) * assign6790_e7088) + (assign6790_e7082 * (-((0.5 * (var_f1_dn0 / (2.0 * assign6790_e7087))) / (assign6790_e7087 * assign6790_e7087))))), (((((((((var_if0_dn1 * var_evb2e1) + (var_if0 * var_evb2e1_dn1)) * var_vtinv) + (assign6790_e7078 * var_vtinv_dn1)) * var_nff_t) - (assign6790_e7080 * var_nff_t_dn1)) / (var_nff_t * var_nff_t)) * assign6790_e7088) + (assign6790_e7082 * (-((0.5 * (var_f1_dn1 / (2.0 * assign6790_e7087))) / (assign6790_e7087 * assign6790_e7087))))), (((((((((var_if0_dn2 * var_evb2e1) + (var_if0 * var_evb2e1_dn2)) * var_vtinv) + (assign6790_e7078 * var_vtinv_dn2)) * var_nff_t) - (assign6790_e7080 * var_nff_t_dn2)) / (var_nff_t * var_nff_t)) * assign6790_e7088) + (assign6790_e7082 * (-((0.5 * (var_f1_dn2 / (2.0 * assign6790_e7087))) / (assign6790_e7087 * assign6790_e7087))))), (((((((((var_if0_dn3 * var_evb2e1) + (var_if0 * var_evb2e1_dn3)) * var_vtinv) + (assign6790_e7078 * var_vtinv_dn3)) * var_nff_t) - (assign6790_e7080 * var_nff_t_dn3)) / (var_nff_t * var_nff_t)) * assign6790_e7088) + (assign6790_e7082 * (-((0.5 * (var_f1_dn3 / (2.0 * assign6790_e7087))) / (assign6790_e7087 * assign6790_e7087))))), (((((((((var_if0_dn4 * var_evb2e1) + (var_if0 * var_evb2e1_dn4)) * var_vtinv) + (assign6790_e7078 * var_vtinv_dn4)) * var_nff_t) - (assign6790_e7080 * var_nff_t_dn4)) / (var_nff_t * var_nff_t)) * assign6790_e7088) + (assign6790_e7082 * (-((0.5 * (var_f1_dn4 / (2.0 * assign6790_e7087))) / (assign6790_e7087 * assign6790_e7087))))), (((((((((var_if0_dn5 * var_evb2e1) + (var_if0 * var_evb2e1_dn5)) * var_vtinv) + (assign6790_e7078 * var_vtinv_dn5)) * var_nff_t) - (assign6790_e7080 * var_nff_t_dn5)) / (var_nff_t * var_nff_t)) * assign6790_e7088) + (assign6790_e7082 * (-((0.5 * (var_f1_dn5 / (2.0 * assign6790_e7087))) / (assign6790_e7087 * assign6790_e7087))))), (((((((((var_if0_dn6 * var_evb2e1) + (var_if0 * var_evb2e1_dn6)) * var_vtinv) + (assign6790_e7078 * var_vtinv_dn6)) * var_nff_t) - (assign6790_e7080 * var_nff_t_dn6)) / (var_nff_t * var_nff_t)) * assign6790_e7088) + (assign6790_e7082 * (-((0.5 * (var_f1_dn6 / (2.0 * assign6790_e7087))) / (assign6790_e7087 * assign6790_e7087))))), (((((((((var_if0_dn7 * var_evb2e1) + (var_if0 * var_evb2e1_dn7)) * var_vtinv) + (assign6790_e7078 * var_vtinv_dn7)) * var_nff_t) - (assign6790_e7080 * var_nff_t_dn7)) / (var_nff_t * var_nff_t)) * assign6790_e7088) + (assign6790_e7082 * (-((0.5 * (var_f1_dn7 / (2.0 * assign6790_e7087))) / (assign6790_e7087 * assign6790_e7087))))), (((((((((var_if0_dn8 * var_evb2e1) + (var_if0 * var_evb2e1_dn8)) * var_vtinv) + (assign6790_e7078 * var_vtinv_dn8)) * var_nff_t) - (assign6790_e7080 * var_nff_t_dn8)) / (var_nff_t * var_nff_t)) * assign6790_e7088) + (assign6790_e7082 * (-((0.5 * (var_f1_dn8 / (2.0 * assign6790_e7087))) / (assign6790_e7087 * assign6790_e7087))))), (((((((((var_if0_dn9 * var_evb2e1) + (var_if0 * var_evb2e1_dn9)) * var_vtinv) + (assign6790_e7078 * var_vtinv_dn9)) * var_nff_t) - (assign6790_e7080 * var_nff_t_dn9)) / (var_nff_t * var_nff_t)) * assign6790_e7088) + (assign6790_e7082 * (-((0.5 * (var_f1_dn9 / (2.0 * assign6790_e7087))) / (assign6790_e7087 * assign6790_e7087))))), (((((((((var_if0_dn10 * var_evb2e1) + (var_if0 * var_evb2e1_dn10)) * var_vtinv) + (assign6790_e7078 * var_vtinv_dn10)) * var_nff_t) - (assign6790_e7080 * var_nff_t_dn10)) / (var_nff_t * var_nff_t)) * assign6790_e7088) + (assign6790_e7082 * (-((0.5 * (var_f1_dn10 / (2.0 * assign6790_e7087))) / (assign6790_e7087 * assign6790_e7087))))), (((((((((var_if0_dn11 * var_evb2e1) + (var_if0 * var_evb2e1_dn11)) * var_vtinv) + (assign6790_e7078 * var_vtinv_dn11)) * var_nff_t) - (assign6790_e7080 * var_nff_t_dn11)) / (var_nff_t * var_nff_t)) * assign6790_e7088) + (assign6790_e7082 * (-((0.5 * (var_f1_dn11 / (2.0 * assign6790_e7087))) / (assign6790_e7087 * assign6790_e7087))))), (((((((((var_if0_dn12 * var_evb2e1) + (var_if0 * var_evb2e1_dn12)) * var_vtinv) + (assign6790_e7078 * var_vtinv_dn12)) * var_nff_t) - (assign6790_e7080 * var_nff_t_dn12)) / (var_nff_t * var_nff_t)) * assign6790_e7088) + (assign6790_e7082 * (-((0.5 * (var_f1_dn12 / (2.0 * assign6790_e7087))) / (assign6790_e7087 * assign6790_e7087))))), (((((((((var_if0_db0 * var_evb2e1) + (var_if0 * var_evb2e1_db0)) * var_vtinv) + (assign6790_e7078 * var_vtinv_db0)) * var_nff_t) - (assign6790_e7080 * var_nff_t_db0)) / (var_nff_t * var_nff_t)) * assign6790_e7088) + (assign6790_e7082 * (-((0.5 * (var_f1_db0 / (2.0 * assign6790_e7087))) / (assign6790_e7087 * assign6790_e7087))))), (((((((((var_if0_db1 * var_evb2e1) + (var_if0 * var_evb2e1_db1)) * var_vtinv) + (assign6790_e7078 * var_vtinv_db1)) * var_nff_t) - (assign6790_e7080 * var_nff_t_db1)) / (var_nff_t * var_nff_t)) * assign6790_e7088) + (assign6790_e7082 * (-((0.5 * (var_f1_db1 / (2.0 * assign6790_e7087))) / (assign6790_e7087 * assign6790_e7087))))),)
    } else {
        (var_dn0vb2e1, var_dn0vb2e1_dn0, var_dn0vb2e1_dn1, var_dn0vb2e1_dn2, var_dn0vb2e1_dn3, var_dn0vb2e1_dn4, var_dn0vb2e1_dn5, var_dn0vb2e1_dn6, var_dn0vb2e1_dn7, var_dn0vb2e1_dn8, var_dn0vb2e1_dn9, var_dn0vb2e1_dn10, var_dn0vb2e1_dn11, var_dn0vb2e1_dn12, var_dn0vb2e1_db0, var_dn0vb2e1_db1,)
    }
};
        var_dn0vb2e1 = assign6790_e7091;
        var_dn0vb2e1_dn0 = assign6790_e7091_d_n0;
        var_dn0vb2e1_dn1 = assign6790_e7091_d_n1;
        var_dn0vb2e1_dn2 = assign6790_e7091_d_n2;
        var_dn0vb2e1_dn3 = assign6790_e7091_d_n3;
        var_dn0vb2e1_dn4 = assign6790_e7091_d_n4;
        var_dn0vb2e1_dn5 = assign6790_e7091_d_n5;
        var_dn0vb2e1_dn6 = assign6790_e7091_d_n6;
        var_dn0vb2e1_dn7 = assign6790_e7091_d_n7;
        var_dn0vb2e1_dn8 = assign6790_e7091_d_n8;
        var_dn0vb2e1_dn9 = assign6790_e7091_d_n9;
        var_dn0vb2e1_dn10 = assign6790_e7091_d_n10;
        var_dn0vb2e1_dn11 = assign6790_e7091_d_n11;
        var_dn0vb2e1_dn12 = assign6790_e7091_d_n12;
        var_dn0vb2e1_db0 = assign6790_e7091_d_b0;
        var_dn0vb2e1_db1 = assign6790_e7091_d_b1;
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
        var_dn0vb2e1_rdn12 = 0.0;
        var_dn0vb2e1_rdb0 = 0.0;
        var_dn0vb2e1_rdb1 = 0.0;

        let (assign6800_e7101, assign6800_e7101_d_n0, assign6800_e7101_d_n1, assign6800_e7101_d_n2, assign6800_e7101_d_n3, assign6800_e7101_d_n4, assign6800_e7101_d_n5, assign6800_e7101_d_n6, assign6800_e7101_d_n7, assign6800_e7101_d_n8, assign6800_e7101_d_n9, assign6800_e7101_d_n10, assign6800_e7101_d_n11, assign6800_e7101_d_n12, assign6800_e7101_d_b0, assign6800_e7101_d_b1,) = {
    if (var_guard123 != 0.0) {
        let assign6800_e7095: f64 = (0.5 * var_qb0);
        let assign6800_e7097: f64 = (assign6800_e7095 * var_q1q);
        let assign6800_e7099: f64 = (assign6800_e7097 * var_dn0vb2e1);
        (assign6800_e7099, (((((0.5 * var_qb0_dn0) * var_q1q) + (assign6800_e7095 * var_q1q_dn0)) * var_dn0vb2e1) + (assign6800_e7097 * var_dn0vb2e1_dn0)), (((((0.5 * var_qb0_dn1) * var_q1q) + (assign6800_e7095 * var_q1q_dn1)) * var_dn0vb2e1) + (assign6800_e7097 * var_dn0vb2e1_dn1)), (((((0.5 * var_qb0_dn2) * var_q1q) + (assign6800_e7095 * var_q1q_dn2)) * var_dn0vb2e1) + (assign6800_e7097 * var_dn0vb2e1_dn2)), (((((0.5 * var_qb0_dn3) * var_q1q) + (assign6800_e7095 * var_q1q_dn3)) * var_dn0vb2e1) + (assign6800_e7097 * var_dn0vb2e1_dn3)), (((((0.5 * var_qb0_dn4) * var_q1q) + (assign6800_e7095 * var_q1q_dn4)) * var_dn0vb2e1) + (assign6800_e7097 * var_dn0vb2e1_dn4)), (((((0.5 * var_qb0_dn5) * var_q1q) + (assign6800_e7095 * var_q1q_dn5)) * var_dn0vb2e1) + (assign6800_e7097 * var_dn0vb2e1_dn5)), (((((0.5 * var_qb0_dn6) * var_q1q) + (assign6800_e7095 * var_q1q_dn6)) * var_dn0vb2e1) + (assign6800_e7097 * var_dn0vb2e1_dn6)), (((((0.5 * var_qb0_dn7) * var_q1q) + (assign6800_e7095 * var_q1q_dn7)) * var_dn0vb2e1) + (assign6800_e7097 * var_dn0vb2e1_dn7)), (((((0.5 * var_qb0_dn8) * var_q1q) + (assign6800_e7095 * var_q1q_dn8)) * var_dn0vb2e1) + (assign6800_e7097 * var_dn0vb2e1_dn8)), (((((0.5 * var_qb0_dn9) * var_q1q) + (assign6800_e7095 * var_q1q_dn9)) * var_dn0vb2e1) + (assign6800_e7097 * var_dn0vb2e1_dn9)), (((((0.5 * var_qb0_dn10) * var_q1q) + (assign6800_e7095 * var_q1q_dn10)) * var_dn0vb2e1) + (assign6800_e7097 * var_dn0vb2e1_dn10)), (((((0.5 * var_qb0_dn11) * var_q1q) + (assign6800_e7095 * var_q1q_dn11)) * var_dn0vb2e1) + (assign6800_e7097 * var_dn0vb2e1_dn11)), (((((0.5 * var_qb0_dn12) * var_q1q) + (assign6800_e7095 * var_q1q_dn12)) * var_dn0vb2e1) + (assign6800_e7097 * var_dn0vb2e1_dn12)), (((((0.5 * var_qb0_db0) * var_q1q) + (assign6800_e7095 * var_q1q_db0)) * var_dn0vb2e1) + (assign6800_e7097 * var_dn0vb2e1_db0)), (((((0.5 * var_qb0_db1) * var_q1q) + (assign6800_e7095 * var_q1q_db1)) * var_dn0vb2e1) + (assign6800_e7097 * var_dn0vb2e1_db1)),)
    } else {
        (var_dqbevb2e1, var_dqbevb2e1_dn0, var_dqbevb2e1_dn1, var_dqbevb2e1_dn2, var_dqbevb2e1_dn3, var_dqbevb2e1_dn4, var_dqbevb2e1_dn5, var_dqbevb2e1_dn6, var_dqbevb2e1_dn7, var_dqbevb2e1_dn8, var_dqbevb2e1_dn9, var_dqbevb2e1_dn10, var_dqbevb2e1_dn11, var_dqbevb2e1_dn12, var_dqbevb2e1_db0, var_dqbevb2e1_db1,)
    }
};
        var_dqbevb2e1 = assign6800_e7101;
        var_dqbevb2e1_dn0 = assign6800_e7101_d_n0;
        var_dqbevb2e1_dn1 = assign6800_e7101_d_n1;
        var_dqbevb2e1_dn2 = assign6800_e7101_d_n2;
        var_dqbevb2e1_dn3 = assign6800_e7101_d_n3;
        var_dqbevb2e1_dn4 = assign6800_e7101_d_n4;
        var_dqbevb2e1_dn5 = assign6800_e7101_d_n5;
        var_dqbevb2e1_dn6 = assign6800_e7101_d_n6;
        var_dqbevb2e1_dn7 = assign6800_e7101_d_n7;
        var_dqbevb2e1_dn8 = assign6800_e7101_d_n8;
        var_dqbevb2e1_dn9 = assign6800_e7101_d_n9;
        var_dqbevb2e1_dn10 = assign6800_e7101_d_n10;
        var_dqbevb2e1_dn11 = assign6800_e7101_d_n11;
        var_dqbevb2e1_dn12 = assign6800_e7101_d_n12;
        var_dqbevb2e1_db0 = assign6800_e7101_d_b0;
        var_dqbevb2e1_db1 = assign6800_e7101_d_b1;
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
        var_dqbevb2e1_rdn12 = 0.0;
        var_dqbevb2e1_rdb0 = 0.0;
        var_dqbevb2e1_rdb1 = 0.0;

        let (assign6810_e7109, assign6810_e7109_d_n0, assign6810_e7109_d_n1, assign6810_e7109_d_n2, assign6810_e7109_d_n3, assign6810_e7109_d_n4, assign6810_e7109_d_n5, assign6810_e7109_d_n6, assign6810_e7109_d_n7, assign6810_e7109_d_n8, assign6810_e7109_d_n9, assign6810_e7109_d_n10, assign6810_e7109_d_n11, assign6810_e7109_d_n12, assign6810_e7109_d_b0, assign6810_e7109_d_b1,) = {
    if (var_guard123 != 0.0) {
        let assign6810_e7106: f64 = (p.p85 * var_vt);
        let assign6810_e7107: f64 = (var_qe_qs / assign6810_e7106);
        (assign6810_e7107, (((var_qe_qs_dn0 * assign6810_e7106) - (var_qe_qs * (p.p85 * var_vt_dn0))) / (assign6810_e7106 * assign6810_e7106)), (((var_qe_qs_dn1 * assign6810_e7106) - (var_qe_qs * (p.p85 * var_vt_dn1))) / (assign6810_e7106 * assign6810_e7106)), (((var_qe_qs_dn2 * assign6810_e7106) - (var_qe_qs * (p.p85 * var_vt_dn2))) / (assign6810_e7106 * assign6810_e7106)), (((var_qe_qs_dn3 * assign6810_e7106) - (var_qe_qs * (p.p85 * var_vt_dn3))) / (assign6810_e7106 * assign6810_e7106)), (((var_qe_qs_dn4 * assign6810_e7106) - (var_qe_qs * (p.p85 * var_vt_dn4))) / (assign6810_e7106 * assign6810_e7106)), (((var_qe_qs_dn5 * assign6810_e7106) - (var_qe_qs * (p.p85 * var_vt_dn5))) / (assign6810_e7106 * assign6810_e7106)), (((var_qe_qs_dn6 * assign6810_e7106) - (var_qe_qs * (p.p85 * var_vt_dn6))) / (assign6810_e7106 * assign6810_e7106)), (((var_qe_qs_dn7 * assign6810_e7106) - (var_qe_qs * (p.p85 * var_vt_dn7))) / (assign6810_e7106 * assign6810_e7106)), (((var_qe_qs_dn8 * assign6810_e7106) - (var_qe_qs * (p.p85 * var_vt_dn8))) / (assign6810_e7106 * assign6810_e7106)), (((var_qe_qs_dn9 * assign6810_e7106) - (var_qe_qs * (p.p85 * var_vt_dn9))) / (assign6810_e7106 * assign6810_e7106)), (((var_qe_qs_dn10 * assign6810_e7106) - (var_qe_qs * (p.p85 * var_vt_dn10))) / (assign6810_e7106 * assign6810_e7106)), (((var_qe_qs_dn11 * assign6810_e7106) - (var_qe_qs * (p.p85 * var_vt_dn11))) / (assign6810_e7106 * assign6810_e7106)), (((var_qe_qs_dn12 * assign6810_e7106) - (var_qe_qs * (p.p85 * var_vt_dn12))) / (assign6810_e7106 * assign6810_e7106)), (((var_qe_qs_db0 * assign6810_e7106) - (var_qe_qs * (p.p85 * var_vt_db0))) / (assign6810_e7106 * assign6810_e7106)), (((var_qe_qs_db1 * assign6810_e7106) - (var_qe_qs * (p.p85 * var_vt_db1))) / (assign6810_e7106 * assign6810_e7106)),)
    } else {
        (var_dqevb2e1, var_dqevb2e1_dn0, var_dqevb2e1_dn1, var_dqevb2e1_dn2, var_dqevb2e1_dn3, var_dqevb2e1_dn4, var_dqevb2e1_dn5, var_dqevb2e1_dn6, var_dqevb2e1_dn7, var_dqevb2e1_dn8, var_dqevb2e1_dn9, var_dqevb2e1_dn10, var_dqevb2e1_dn11, var_dqevb2e1_dn12, var_dqevb2e1_db0, var_dqevb2e1_db1,)
    }
};
        var_dqevb2e1 = assign6810_e7109;
        var_dqevb2e1_dn0 = assign6810_e7109_d_n0;
        var_dqevb2e1_dn1 = assign6810_e7109_d_n1;
        var_dqevb2e1_dn2 = assign6810_e7109_d_n2;
        var_dqevb2e1_dn3 = assign6810_e7109_d_n3;
        var_dqevb2e1_dn4 = assign6810_e7109_d_n4;
        var_dqevb2e1_dn5 = assign6810_e7109_d_n5;
        var_dqevb2e1_dn6 = assign6810_e7109_d_n6;
        var_dqevb2e1_dn7 = assign6810_e7109_d_n7;
        var_dqevb2e1_dn8 = assign6810_e7109_d_n8;
        var_dqevb2e1_dn9 = assign6810_e7109_d_n9;
        var_dqevb2e1_dn10 = assign6810_e7109_d_n10;
        var_dqevb2e1_dn11 = assign6810_e7109_d_n11;
        var_dqevb2e1_dn12 = assign6810_e7109_d_n12;
        var_dqevb2e1_db0 = assign6810_e7109_d_b0;
        var_dqevb2e1_db1 = assign6810_e7109_d_b1;
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
        var_dqevb2e1_rdn12 = 0.0;
        var_dqevb2e1_rdb0 = 0.0;
        var_dqevb2e1_rdb1 = 0.0;

        let (assign6820_e7121, assign6820_e7121_d_n0, assign6820_e7121_d_n1, assign6820_e7121_d_n2, assign6820_e7121_d_n3, assign6820_e7121_d_n4, assign6820_e7121_d_n5, assign6820_e7121_d_n6, assign6820_e7121_d_n7, assign6820_e7121_d_n8, assign6820_e7121_d_n9, assign6820_e7121_d_n10, assign6820_e7121_d_n11, assign6820_e7121_d_n12, assign6820_e7121_d_b0, assign6820_e7121_d_b1,) = {
    if (var_guard123 != 0.0) {
        let assign6820_e7113: f64 = (0.2 * var_vb1b2);
        let assign6820_e7116: f64 = (var_dqtevb2e1 + var_dqbevb2e1);
        let assign6820_e7118: f64 = (assign6820_e7116 + var_dqevb2e1);
        let assign6820_e7119: f64 = (assign6820_e7113 * assign6820_e7118);
        (assign6820_e7119, (((0.2 * var_vb1b2_dn0) * assign6820_e7118) + (assign6820_e7113 * ((var_dqtevb2e1_dn0 + var_dqbevb2e1_dn0) + var_dqevb2e1_dn0))), (((0.2 * var_vb1b2_dn1) * assign6820_e7118) + (assign6820_e7113 * ((var_dqtevb2e1_dn1 + var_dqbevb2e1_dn1) + var_dqevb2e1_dn1))), (((0.2 * var_vb1b2_dn2) * assign6820_e7118) + (assign6820_e7113 * ((var_dqtevb2e1_dn2 + var_dqbevb2e1_dn2) + var_dqevb2e1_dn2))), (((0.2 * var_vb1b2_dn3) * assign6820_e7118) + (assign6820_e7113 * ((var_dqtevb2e1_dn3 + var_dqbevb2e1_dn3) + var_dqevb2e1_dn3))), (((0.2 * var_vb1b2_dn4) * assign6820_e7118) + (assign6820_e7113 * ((var_dqtevb2e1_dn4 + var_dqbevb2e1_dn4) + var_dqevb2e1_dn4))), (((0.2 * var_vb1b2_dn5) * assign6820_e7118) + (assign6820_e7113 * ((var_dqtevb2e1_dn5 + var_dqbevb2e1_dn5) + var_dqevb2e1_dn5))), (((0.2 * var_vb1b2_dn6) * assign6820_e7118) + (assign6820_e7113 * ((var_dqtevb2e1_dn6 + var_dqbevb2e1_dn6) + var_dqevb2e1_dn6))), (((0.2 * var_vb1b2_dn7) * assign6820_e7118) + (assign6820_e7113 * ((var_dqtevb2e1_dn7 + var_dqbevb2e1_dn7) + var_dqevb2e1_dn7))), (((0.2 * var_vb1b2_dn8) * assign6820_e7118) + (assign6820_e7113 * ((var_dqtevb2e1_dn8 + var_dqbevb2e1_dn8) + var_dqevb2e1_dn8))), (((0.2 * var_vb1b2_dn9) * assign6820_e7118) + (assign6820_e7113 * ((var_dqtevb2e1_dn9 + var_dqbevb2e1_dn9) + var_dqevb2e1_dn9))), (((0.2 * var_vb1b2_dn10) * assign6820_e7118) + (assign6820_e7113 * ((var_dqtevb2e1_dn10 + var_dqbevb2e1_dn10) + var_dqevb2e1_dn10))), (((0.2 * var_vb1b2_dn11) * assign6820_e7118) + (assign6820_e7113 * ((var_dqtevb2e1_dn11 + var_dqbevb2e1_dn11) + var_dqevb2e1_dn11))), (((0.2 * var_vb1b2_dn12) * assign6820_e7118) + (assign6820_e7113 * ((var_dqtevb2e1_dn12 + var_dqbevb2e1_dn12) + var_dqevb2e1_dn12))), (((0.2 * var_vb1b2_db0) * assign6820_e7118) + (assign6820_e7113 * ((var_dqtevb2e1_db0 + var_dqbevb2e1_db0) + var_dqevb2e1_db0))), (((0.2 * var_vb1b2_db1) * assign6820_e7118) + (assign6820_e7113 * ((var_dqtevb2e1_db1 + var_dqbevb2e1_db1) + var_dqevb2e1_db1))),)
    } else {
        (var_qb1b2, var_qb1b2_dn0, var_qb1b2_dn1, var_qb1b2_dn2, var_qb1b2_dn3, var_qb1b2_dn4, var_qb1b2_dn5, var_qb1b2_dn6, var_qb1b2_dn7, var_qb1b2_dn8, var_qb1b2_dn9, var_qb1b2_dn10, var_qb1b2_dn11, var_qb1b2_dn12, var_qb1b2_db0, var_qb1b2_db1,)
    }
};
        var_qb1b2 = assign6820_e7121;
        var_qb1b2_dn0 = assign6820_e7121_d_n0;
        var_qb1b2_dn1 = assign6820_e7121_d_n1;
        var_qb1b2_dn2 = assign6820_e7121_d_n2;
        var_qb1b2_dn3 = assign6820_e7121_d_n3;
        var_qb1b2_dn4 = assign6820_e7121_d_n4;
        var_qb1b2_dn5 = assign6820_e7121_d_n5;
        var_qb1b2_dn6 = assign6820_e7121_d_n6;
        var_qb1b2_dn7 = assign6820_e7121_d_n7;
        var_qb1b2_dn8 = assign6820_e7121_d_n8;
        var_qb1b2_dn9 = assign6820_e7121_d_n9;
        var_qb1b2_dn10 = assign6820_e7121_d_n10;
        var_qb1b2_dn11 = assign6820_e7121_d_n11;
        var_qb1b2_dn12 = assign6820_e7121_d_n12;
        var_qb1b2_db0 = assign6820_e7121_d_b0;
        var_qb1b2_db1 = assign6820_e7121_d_b1;
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
        var_qb1b2_rdn12 = 0.0;
        var_qb1b2_rdb0 = 0.0;
        var_qb1b2_rdb1 = 0.0;

        *var_dn0vb2e1_slot = var_dn0vb2e1;
        *var_dn0vb2e1_db0_slot = var_dn0vb2e1_db0;
        *var_dn0vb2e1_db1_slot = var_dn0vb2e1_db1;
        *var_dn0vb2e1_dn0_slot = var_dn0vb2e1_dn0;
        *var_dn0vb2e1_dn1_slot = var_dn0vb2e1_dn1;
        *var_dn0vb2e1_dn10_slot = var_dn0vb2e1_dn10;
        *var_dn0vb2e1_dn11_slot = var_dn0vb2e1_dn11;
        *var_dn0vb2e1_dn12_slot = var_dn0vb2e1_dn12;
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
        *var_dn0vb2e1_rdn12_slot = var_dn0vb2e1_rdn12;
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
        *var_dqbevb2e1_dn12_slot = var_dqbevb2e1_dn12;
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
        *var_dqbevb2e1_rdn12_slot = var_dqbevb2e1_rdn12;
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
        *var_dqevb2e1_dn12_slot = var_dqevb2e1_dn12;
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
        *var_dqevb2e1_rdn12_slot = var_dqevb2e1_rdn12;
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
        *var_dqtevb2e1_dn12_slot = var_dqtevb2e1_dn12;
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
        *var_dqtevb2e1_rdn12_slot = var_dqtevb2e1_rdn12;
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
        *var_dvjevb2e1_dn12_slot = var_dvjevb2e1_dn12;
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
        *var_dvjevb2e1_rdn12_slot = var_dvjevb2e1_rdn12;
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
        *var_dvtevb2e1_dn12_slot = var_dvtevb2e1_dn12;
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
        *var_dvtevb2e1_rdn12_slot = var_dvtevb2e1_rdn12;
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
        *var_dvtevje_dn12_slot = var_dvtevje_dn12;
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
        *var_dvtevje_rdn12_slot = var_dvtevje_rdn12;
        *var_dvtevje_rdn2_slot = var_dvtevje_rdn2;
        *var_dvtevje_rdn3_slot = var_dvtevje_rdn3;
        *var_dvtevje_rdn4_slot = var_dvtevje_rdn4;
        *var_dvtevje_rdn5_slot = var_dvtevje_rdn5;
        *var_dvtevje_rdn6_slot = var_dvtevje_rdn6;
        *var_dvtevje_rdn7_slot = var_dvtevje_rdn7;
        *var_dvtevje_rdn8_slot = var_dvtevje_rdn8;
        *var_dvtevje_rdn9_slot = var_dvtevje_rdn9;
        *var_dvtevje_rv_slot = var_dvtevje_rv;
        *var_guard123_slot = var_guard123;
        *var_guard123_db0_slot = var_guard123_db0;
        *var_guard123_db1_slot = var_guard123_db1;
        *var_guard123_dn0_slot = var_guard123_dn0;
        *var_guard123_dn1_slot = var_guard123_dn1;
        *var_guard123_dn10_slot = var_guard123_dn10;
        *var_guard123_dn11_slot = var_guard123_dn11;
        *var_guard123_dn12_slot = var_guard123_dn12;
        *var_guard123_dn2_slot = var_guard123_dn2;
        *var_guard123_dn3_slot = var_guard123_dn3;
        *var_guard123_dn4_slot = var_guard123_dn4;
        *var_guard123_dn5_slot = var_guard123_dn5;
        *var_guard123_dn6_slot = var_guard123_dn6;
        *var_guard123_dn7_slot = var_guard123_dn7;
        *var_guard123_dn8_slot = var_guard123_dn8;
        *var_guard123_dn9_slot = var_guard123_dn9;
        *var_guard123_rdb0_slot = var_guard123_rdb0;
        *var_guard123_rdb1_slot = var_guard123_rdb1;
        *var_guard123_rdn0_slot = var_guard123_rdn0;
        *var_guard123_rdn1_slot = var_guard123_rdn1;
        *var_guard123_rdn10_slot = var_guard123_rdn10;
        *var_guard123_rdn11_slot = var_guard123_rdn11;
        *var_guard123_rdn12_slot = var_guard123_rdn12;
        *var_guard123_rdn2_slot = var_guard123_rdn2;
        *var_guard123_rdn3_slot = var_guard123_rdn3;
        *var_guard123_rdn4_slot = var_guard123_rdn4;
        *var_guard123_rdn5_slot = var_guard123_rdn5;
        *var_guard123_rdn6_slot = var_guard123_rdn6;
        *var_guard123_rdn7_slot = var_guard123_rdn7;
        *var_guard123_rdn8_slot = var_guard123_rdn8;
        *var_guard123_rdn9_slot = var_guard123_rdn9;
        *var_guard123_rv_slot = var_guard123_rv;
        *var_guard124_slot = var_guard124;
        *var_guard124_db0_slot = var_guard124_db0;
        *var_guard124_db1_slot = var_guard124_db1;
        *var_guard124_dn0_slot = var_guard124_dn0;
        *var_guard124_dn1_slot = var_guard124_dn1;
        *var_guard124_dn10_slot = var_guard124_dn10;
        *var_guard124_dn11_slot = var_guard124_dn11;
        *var_guard124_dn12_slot = var_guard124_dn12;
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
        *var_guard124_rdn12_slot = var_guard124_rdn12;
        *var_guard124_rdn2_slot = var_guard124_rdn2;
        *var_guard124_rdn3_slot = var_guard124_rdn3;
        *var_guard124_rdn4_slot = var_guard124_rdn4;
        *var_guard124_rdn5_slot = var_guard124_rdn5;
        *var_guard124_rdn6_slot = var_guard124_rdn6;
        *var_guard124_rdn7_slot = var_guard124_rdn7;
        *var_guard124_rdn8_slot = var_guard124_rdn8;
        *var_guard124_rdn9_slot = var_guard124_rdn9;
        *var_guard124_rv_slot = var_guard124_rv;
        *var_qb1b2_slot = var_qb1b2;
        *var_qb1b2_db0_slot = var_qb1b2_db0;
        *var_qb1b2_db1_slot = var_qb1b2_db1;
        *var_qb1b2_dn0_slot = var_qb1b2_dn0;
        *var_qb1b2_dn1_slot = var_qb1b2_dn1;
        *var_qb1b2_dn10_slot = var_qb1b2_dn10;
        *var_qb1b2_dn11_slot = var_qb1b2_dn11;
        *var_qb1b2_dn12_slot = var_qb1b2_dn12;
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
        *var_qb1b2_rdn12_slot = var_qb1b2_rdn12;
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
        *var_vb2e1vfe_dn12_slot = var_vb2e1vfe_dn12;
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
        *var_vb2e1vfe_rdn12_slot = var_vb2e1vfe_rdn12;
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

    pub(super) fn stamp_reactive_block_44(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_guard123: f64,
        var_if_: f64,
        var_if__db0: f64,
        var_if__db1: f64,
        var_if__dn0: f64,
        var_if__dn1: f64,
        var_if__dn10: f64,
        var_if__dn11: f64,
        var_if__dn12: f64,
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
        var_ir_dn12: f64,
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
        var_q1q_dn12: f64,
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
        var_qbc_qs_dn12: f64,
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
        var_qbe_qs_dn12: f64,
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
        var_qbi_dn12: f64,
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
        var_qe_qs_dn12: f64,
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
        var_taub_t_dn12: f64,
        var_taub_t_dn2: f64,
        var_taub_t_dn3: f64,
        var_taub_t_dn4: f64,
        var_taub_t_dn5: f64,
        var_taub_t_dn6: f64,
        var_taub_t_dn7: f64,
        var_taub_t_dn8: f64,
        var_taub_t_dn9: f64,
        var_guard132_slot: &mut f64,
        var_guard132_db0_slot: &mut f64,
        var_guard132_db1_slot: &mut f64,
        var_guard132_dn0_slot: &mut f64,
        var_guard132_dn1_slot: &mut f64,
        var_guard132_dn10_slot: &mut f64,
        var_guard132_dn11_slot: &mut f64,
        var_guard132_dn12_slot: &mut f64,
        var_guard132_dn2_slot: &mut f64,
        var_guard132_dn3_slot: &mut f64,
        var_guard132_dn4_slot: &mut f64,
        var_guard132_dn5_slot: &mut f64,
        var_guard132_dn6_slot: &mut f64,
        var_guard132_dn7_slot: &mut f64,
        var_guard132_dn8_slot: &mut f64,
        var_guard132_dn9_slot: &mut f64,
        var_guard132_rdb0_slot: &mut f64,
        var_guard132_rdb1_slot: &mut f64,
        var_guard132_rdn0_slot: &mut f64,
        var_guard132_rdn1_slot: &mut f64,
        var_guard132_rdn10_slot: &mut f64,
        var_guard132_rdn11_slot: &mut f64,
        var_guard132_rdn12_slot: &mut f64,
        var_guard132_rdn2_slot: &mut f64,
        var_guard132_rdn3_slot: &mut f64,
        var_guard132_rdn4_slot: &mut f64,
        var_guard132_rdn5_slot: &mut f64,
        var_guard132_rdn6_slot: &mut f64,
        var_guard132_rdn7_slot: &mut f64,
        var_guard132_rdn8_slot: &mut f64,
        var_guard132_rdn9_slot: &mut f64,
        var_guard132_rv_slot: &mut f64,
        var_guard133_slot: &mut f64,
        var_guard133_db0_slot: &mut f64,
        var_guard133_db1_slot: &mut f64,
        var_guard133_dn0_slot: &mut f64,
        var_guard133_dn1_slot: &mut f64,
        var_guard133_dn10_slot: &mut f64,
        var_guard133_dn11_slot: &mut f64,
        var_guard133_dn12_slot: &mut f64,
        var_guard133_dn2_slot: &mut f64,
        var_guard133_dn3_slot: &mut f64,
        var_guard133_dn4_slot: &mut f64,
        var_guard133_dn5_slot: &mut f64,
        var_guard133_dn6_slot: &mut f64,
        var_guard133_dn7_slot: &mut f64,
        var_guard133_dn8_slot: &mut f64,
        var_guard133_dn9_slot: &mut f64,
        var_guard133_rdb0_slot: &mut f64,
        var_guard133_rdb1_slot: &mut f64,
        var_guard133_rdn0_slot: &mut f64,
        var_guard133_rdn1_slot: &mut f64,
        var_guard133_rdn10_slot: &mut f64,
        var_guard133_rdn11_slot: &mut f64,
        var_guard133_rdn12_slot: &mut f64,
        var_guard133_rdn2_slot: &mut f64,
        var_guard133_rdn3_slot: &mut f64,
        var_guard133_rdn4_slot: &mut f64,
        var_guard133_rdn5_slot: &mut f64,
        var_guard133_rdn6_slot: &mut f64,
        var_guard133_rdn7_slot: &mut f64,
        var_guard133_rdn8_slot: &mut f64,
        var_guard133_rdn9_slot: &mut f64,
        var_guard133_rv_slot: &mut f64,
        var_i_cth_slot: &mut f64,
        var_i_cth_db0_slot: &mut f64,
        var_i_cth_db1_slot: &mut f64,
        var_i_cth_dn0_slot: &mut f64,
        var_i_cth_dn1_slot: &mut f64,
        var_i_cth_dn10_slot: &mut f64,
        var_i_cth_dn11_slot: &mut f64,
        var_i_cth_dn12_slot: &mut f64,
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
        var_i_cth_rdn12_slot: &mut f64,
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
        var_in_n_dn12_slot: &mut f64,
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
        var_in_n_rdn12_slot: &mut f64,
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
        var_qbc_dn12_slot: &mut f64,
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
        var_qbc_rdn12_slot: &mut f64,
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
        var_qbe_dn12_slot: &mut f64,
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
        var_qbe_qs_eff_dn12_slot: &mut f64,
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
        var_qbe_qs_eff_rdn12_slot: &mut f64,
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
        var_qbe_rdn12_slot: &mut f64,
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
        var_qe_dn12_slot: &mut f64,
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
        var_qe_rdn12_slot: &mut f64,
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
        var_taub_n_dn12_slot: &mut f64,
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
        var_taub_n_rdn12_slot: &mut f64,
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let mut var_guard132: f64 = *var_guard132_slot;
        let mut var_guard132_db0: f64 = *var_guard132_db0_slot;
        let mut var_guard132_db1: f64 = *var_guard132_db1_slot;
        let mut var_guard132_dn0: f64 = *var_guard132_dn0_slot;
        let mut var_guard132_dn1: f64 = *var_guard132_dn1_slot;
        let mut var_guard132_dn10: f64 = *var_guard132_dn10_slot;
        let mut var_guard132_dn11: f64 = *var_guard132_dn11_slot;
        let mut var_guard132_dn12: f64 = *var_guard132_dn12_slot;
        let mut var_guard132_dn2: f64 = *var_guard132_dn2_slot;
        let mut var_guard132_dn3: f64 = *var_guard132_dn3_slot;
        let mut var_guard132_dn4: f64 = *var_guard132_dn4_slot;
        let mut var_guard132_dn5: f64 = *var_guard132_dn5_slot;
        let mut var_guard132_dn6: f64 = *var_guard132_dn6_slot;
        let mut var_guard132_dn7: f64 = *var_guard132_dn7_slot;
        let mut var_guard132_dn8: f64 = *var_guard132_dn8_slot;
        let mut var_guard132_dn9: f64 = *var_guard132_dn9_slot;
        let mut var_guard132_rdb0: f64 = *var_guard132_rdb0_slot;
        let mut var_guard132_rdb1: f64 = *var_guard132_rdb1_slot;
        let mut var_guard132_rdn0: f64 = *var_guard132_rdn0_slot;
        let mut var_guard132_rdn1: f64 = *var_guard132_rdn1_slot;
        let mut var_guard132_rdn10: f64 = *var_guard132_rdn10_slot;
        let mut var_guard132_rdn11: f64 = *var_guard132_rdn11_slot;
        let mut var_guard132_rdn12: f64 = *var_guard132_rdn12_slot;
        let mut var_guard132_rdn2: f64 = *var_guard132_rdn2_slot;
        let mut var_guard132_rdn3: f64 = *var_guard132_rdn3_slot;
        let mut var_guard132_rdn4: f64 = *var_guard132_rdn4_slot;
        let mut var_guard132_rdn5: f64 = *var_guard132_rdn5_slot;
        let mut var_guard132_rdn6: f64 = *var_guard132_rdn6_slot;
        let mut var_guard132_rdn7: f64 = *var_guard132_rdn7_slot;
        let mut var_guard132_rdn8: f64 = *var_guard132_rdn8_slot;
        let mut var_guard132_rdn9: f64 = *var_guard132_rdn9_slot;
        let mut var_guard132_rv: f64 = *var_guard132_rv_slot;
        let mut var_guard133: f64 = *var_guard133_slot;
        let mut var_guard133_db0: f64 = *var_guard133_db0_slot;
        let mut var_guard133_db1: f64 = *var_guard133_db1_slot;
        let mut var_guard133_dn0: f64 = *var_guard133_dn0_slot;
        let mut var_guard133_dn1: f64 = *var_guard133_dn1_slot;
        let mut var_guard133_dn10: f64 = *var_guard133_dn10_slot;
        let mut var_guard133_dn11: f64 = *var_guard133_dn11_slot;
        let mut var_guard133_dn12: f64 = *var_guard133_dn12_slot;
        let mut var_guard133_dn2: f64 = *var_guard133_dn2_slot;
        let mut var_guard133_dn3: f64 = *var_guard133_dn3_slot;
        let mut var_guard133_dn4: f64 = *var_guard133_dn4_slot;
        let mut var_guard133_dn5: f64 = *var_guard133_dn5_slot;
        let mut var_guard133_dn6: f64 = *var_guard133_dn6_slot;
        let mut var_guard133_dn7: f64 = *var_guard133_dn7_slot;
        let mut var_guard133_dn8: f64 = *var_guard133_dn8_slot;
        let mut var_guard133_dn9: f64 = *var_guard133_dn9_slot;
        let mut var_guard133_rdb0: f64 = *var_guard133_rdb0_slot;
        let mut var_guard133_rdb1: f64 = *var_guard133_rdb1_slot;
        let mut var_guard133_rdn0: f64 = *var_guard133_rdn0_slot;
        let mut var_guard133_rdn1: f64 = *var_guard133_rdn1_slot;
        let mut var_guard133_rdn10: f64 = *var_guard133_rdn10_slot;
        let mut var_guard133_rdn11: f64 = *var_guard133_rdn11_slot;
        let mut var_guard133_rdn12: f64 = *var_guard133_rdn12_slot;
        let mut var_guard133_rdn2: f64 = *var_guard133_rdn2_slot;
        let mut var_guard133_rdn3: f64 = *var_guard133_rdn3_slot;
        let mut var_guard133_rdn4: f64 = *var_guard133_rdn4_slot;
        let mut var_guard133_rdn5: f64 = *var_guard133_rdn5_slot;
        let mut var_guard133_rdn6: f64 = *var_guard133_rdn6_slot;
        let mut var_guard133_rdn7: f64 = *var_guard133_rdn7_slot;
        let mut var_guard133_rdn8: f64 = *var_guard133_rdn8_slot;
        let mut var_guard133_rdn9: f64 = *var_guard133_rdn9_slot;
        let mut var_guard133_rv: f64 = *var_guard133_rv_slot;
        let mut var_i_cth: f64 = *var_i_cth_slot;
        let mut var_i_cth_db0: f64 = *var_i_cth_db0_slot;
        let mut var_i_cth_db1: f64 = *var_i_cth_db1_slot;
        let mut var_i_cth_dn0: f64 = *var_i_cth_dn0_slot;
        let mut var_i_cth_dn1: f64 = *var_i_cth_dn1_slot;
        let mut var_i_cth_dn10: f64 = *var_i_cth_dn10_slot;
        let mut var_i_cth_dn11: f64 = *var_i_cth_dn11_slot;
        let mut var_i_cth_dn12: f64 = *var_i_cth_dn12_slot;
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
        let mut var_i_cth_rdn12: f64 = *var_i_cth_rdn12_slot;
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
        let mut var_in_n_dn12: f64 = *var_in_n_dn12_slot;
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
        let mut var_in_n_rdn12: f64 = *var_in_n_rdn12_slot;
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
        let mut var_qbc_dn12: f64 = *var_qbc_dn12_slot;
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
        let mut var_qbc_rdn12: f64 = *var_qbc_rdn12_slot;
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
        let mut var_qbe_dn12: f64 = *var_qbe_dn12_slot;
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
        let mut var_qbe_qs_eff_dn12: f64 = *var_qbe_qs_eff_dn12_slot;
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
        let mut var_qbe_qs_eff_rdn12: f64 = *var_qbe_qs_eff_rdn12_slot;
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
        let mut var_qbe_rdn12: f64 = *var_qbe_rdn12_slot;
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
        let mut var_qe_dn12: f64 = *var_qe_dn12_slot;
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
        let mut var_qe_rdn12: f64 = *var_qe_rdn12_slot;
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
        let mut var_taub_n_dn12: f64 = *var_taub_n_dn12_slot;
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
        let mut var_taub_n_rdn12: f64 = *var_taub_n_rdn12_slot;
        let mut var_taub_n_rdn2: f64 = *var_taub_n_rdn2_slot;
        let mut var_taub_n_rdn3: f64 = *var_taub_n_rdn3_slot;
        let mut var_taub_n_rdn4: f64 = *var_taub_n_rdn4_slot;
        let mut var_taub_n_rdn5: f64 = *var_taub_n_rdn5_slot;
        let mut var_taub_n_rdn6: f64 = *var_taub_n_rdn6_slot;
        let mut var_taub_n_rdn7: f64 = *var_taub_n_rdn7_slot;
        let mut var_taub_n_rdn8: f64 = *var_taub_n_rdn8_slot;
        let mut var_taub_n_rdn9: f64 = *var_taub_n_rdn9_slot;
        let mut var_taub_n_rv: f64 = *var_taub_n_rv_slot;

        let (assign6830_e7129, assign6830_e7129_d_n0, assign6830_e7129_d_n1, assign6830_e7129_d_n2, assign6830_e7129_d_n3, assign6830_e7129_d_n4, assign6830_e7129_d_n5, assign6830_e7129_d_n6, assign6830_e7129_d_n7, assign6830_e7129_d_n8, assign6830_e7129_d_n9, assign6830_e7129_d_n10, assign6830_e7129_d_n11, assign6830_e7129_d_n12, assign6830_e7129_d_b0, assign6830_e7129_d_b1,) = {
    if (var_guard123 != 0.0) {
        let assign6830_e7125: f64 = (1.0 - p.p95);
        let assign6830_e7127: f64 = (assign6830_e7125 * var_qe_qs);
        (assign6830_e7127, (assign6830_e7125 * var_qe_qs_dn0), (assign6830_e7125 * var_qe_qs_dn1), (assign6830_e7125 * var_qe_qs_dn2), (assign6830_e7125 * var_qe_qs_dn3), (assign6830_e7125 * var_qe_qs_dn4), (assign6830_e7125 * var_qe_qs_dn5), (assign6830_e7125 * var_qe_qs_dn6), (assign6830_e7125 * var_qe_qs_dn7), (assign6830_e7125 * var_qe_qs_dn8), (assign6830_e7125 * var_qe_qs_dn9), (assign6830_e7125 * var_qe_qs_dn10), (assign6830_e7125 * var_qe_qs_dn11), (assign6830_e7125 * var_qe_qs_dn12), (assign6830_e7125 * var_qe_qs_db0), (assign6830_e7125 * var_qe_qs_db1),)
    } else {
        (var_qe, var_qe_dn0, var_qe_dn1, var_qe_dn2, var_qe_dn3, var_qe_dn4, var_qe_dn5, var_qe_dn6, var_qe_dn7, var_qe_dn8, var_qe_dn9, var_qe_dn10, var_qe_dn11, var_qe_dn12, var_qe_db0, var_qe_db1,)
    }
};
        var_qe = assign6830_e7129;
        var_qe_dn0 = assign6830_e7129_d_n0;
        var_qe_dn1 = assign6830_e7129_d_n1;
        var_qe_dn2 = assign6830_e7129_d_n2;
        var_qe_dn3 = assign6830_e7129_d_n3;
        var_qe_dn4 = assign6830_e7129_d_n4;
        var_qe_dn5 = assign6830_e7129_d_n5;
        var_qe_dn6 = assign6830_e7129_d_n6;
        var_qe_dn7 = assign6830_e7129_d_n7;
        var_qe_dn8 = assign6830_e7129_d_n8;
        var_qe_dn9 = assign6830_e7129_d_n9;
        var_qe_dn10 = assign6830_e7129_d_n10;
        var_qe_dn11 = assign6830_e7129_d_n11;
        var_qe_dn12 = assign6830_e7129_d_n12;
        var_qe_db0 = assign6830_e7129_d_b0;
        var_qe_db1 = assign6830_e7129_d_b1;
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
        var_qe_rdn12 = 0.0;
        var_qe_rdb0 = 0.0;
        var_qe_rdb1 = 0.0;

        let (assign6840_e7137, assign6840_e7137_d_n0, assign6840_e7137_d_n1, assign6840_e7137_d_n2, assign6840_e7137_d_n3, assign6840_e7137_d_n4, assign6840_e7137_d_n5, assign6840_e7137_d_n6, assign6840_e7137_d_n7, assign6840_e7137_d_n8, assign6840_e7137_d_n9, assign6840_e7137_d_n10, assign6840_e7137_d_n11, assign6840_e7137_d_n12, assign6840_e7137_d_b0, assign6840_e7137_d_b1,) = {
    if (var_guard123 != 0.0) {
        let assign6840_e7134: f64 = (p.p95 * var_qe_qs);
        let assign6840_e7135: f64 = (var_qbe_qs + assign6840_e7134);
        (assign6840_e7135, (var_qbe_qs_dn0 + (p.p95 * var_qe_qs_dn0)), (var_qbe_qs_dn1 + (p.p95 * var_qe_qs_dn1)), (var_qbe_qs_dn2 + (p.p95 * var_qe_qs_dn2)), (var_qbe_qs_dn3 + (p.p95 * var_qe_qs_dn3)), (var_qbe_qs_dn4 + (p.p95 * var_qe_qs_dn4)), (var_qbe_qs_dn5 + (p.p95 * var_qe_qs_dn5)), (var_qbe_qs_dn6 + (p.p95 * var_qe_qs_dn6)), (var_qbe_qs_dn7 + (p.p95 * var_qe_qs_dn7)), (var_qbe_qs_dn8 + (p.p95 * var_qe_qs_dn8)), (var_qbe_qs_dn9 + (p.p95 * var_qe_qs_dn9)), (var_qbe_qs_dn10 + (p.p95 * var_qe_qs_dn10)), (var_qbe_qs_dn11 + (p.p95 * var_qe_qs_dn11)), (var_qbe_qs_dn12 + (p.p95 * var_qe_qs_dn12)), (var_qbe_qs_db0 + (p.p95 * var_qe_qs_db0)), (var_qbe_qs_db1 + (p.p95 * var_qe_qs_db1)),)
    } else {
        (var_qbe_qs_eff, var_qbe_qs_eff_dn0, var_qbe_qs_eff_dn1, var_qbe_qs_eff_dn2, var_qbe_qs_eff_dn3, var_qbe_qs_eff_dn4, var_qbe_qs_eff_dn5, var_qbe_qs_eff_dn6, var_qbe_qs_eff_dn7, var_qbe_qs_eff_dn8, var_qbe_qs_eff_dn9, var_qbe_qs_eff_dn10, var_qbe_qs_eff_dn11, var_qbe_qs_eff_dn12, var_qbe_qs_eff_db0, var_qbe_qs_eff_db1,)
    }
};
        var_qbe_qs_eff = assign6840_e7137;
        var_qbe_qs_eff_dn0 = assign6840_e7137_d_n0;
        var_qbe_qs_eff_dn1 = assign6840_e7137_d_n1;
        var_qbe_qs_eff_dn2 = assign6840_e7137_d_n2;
        var_qbe_qs_eff_dn3 = assign6840_e7137_d_n3;
        var_qbe_qs_eff_dn4 = assign6840_e7137_d_n4;
        var_qbe_qs_eff_dn5 = assign6840_e7137_d_n5;
        var_qbe_qs_eff_dn6 = assign6840_e7137_d_n6;
        var_qbe_qs_eff_dn7 = assign6840_e7137_d_n7;
        var_qbe_qs_eff_dn8 = assign6840_e7137_d_n8;
        var_qbe_qs_eff_dn9 = assign6840_e7137_d_n9;
        var_qbe_qs_eff_dn10 = assign6840_e7137_d_n10;
        var_qbe_qs_eff_dn11 = assign6840_e7137_d_n11;
        var_qbe_qs_eff_dn12 = assign6840_e7137_d_n12;
        var_qbe_qs_eff_db0 = assign6840_e7137_d_b0;
        var_qbe_qs_eff_db1 = assign6840_e7137_d_b1;
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
        var_qbe_qs_eff_rdn12 = 0.0;
        var_qbe_qs_eff_rdb0 = 0.0;
        var_qbe_qs_eff_rdb1 = 0.0;

        let (assign6850_e7145, assign6850_e7145_d_n0, assign6850_e7145_d_n1, assign6850_e7145_d_n2, assign6850_e7145_d_n3, assign6850_e7145_d_n4, assign6850_e7145_d_n5, assign6850_e7145_d_n6, assign6850_e7145_d_n7, assign6850_e7145_d_n8, assign6850_e7145_d_n9, assign6850_e7145_d_n10, assign6850_e7145_d_n11, assign6850_e7145_d_n12, assign6850_e7145_d_b0, assign6850_e7145_d_b1,) = {
    if (var_guard123 != 0.0) {
        let assign6850_e7141: f64 = (p.p94 * var_qbe_qs_eff);
        let assign6850_e7143: f64 = (assign6850_e7141 + var_qbc_qs);
        (assign6850_e7143, ((p.p94 * var_qbe_qs_eff_dn0) + var_qbc_qs_dn0), ((p.p94 * var_qbe_qs_eff_dn1) + var_qbc_qs_dn1), ((p.p94 * var_qbe_qs_eff_dn2) + var_qbc_qs_dn2), ((p.p94 * var_qbe_qs_eff_dn3) + var_qbc_qs_dn3), ((p.p94 * var_qbe_qs_eff_dn4) + var_qbc_qs_dn4), ((p.p94 * var_qbe_qs_eff_dn5) + var_qbc_qs_dn5), ((p.p94 * var_qbe_qs_eff_dn6) + var_qbc_qs_dn6), ((p.p94 * var_qbe_qs_eff_dn7) + var_qbc_qs_dn7), ((p.p94 * var_qbe_qs_eff_dn8) + var_qbc_qs_dn8), ((p.p94 * var_qbe_qs_eff_dn9) + var_qbc_qs_dn9), ((p.p94 * var_qbe_qs_eff_dn10) + var_qbc_qs_dn10), ((p.p94 * var_qbe_qs_eff_dn11) + var_qbc_qs_dn11), ((p.p94 * var_qbe_qs_eff_dn12) + var_qbc_qs_dn12), ((p.p94 * var_qbe_qs_eff_db0) + var_qbc_qs_db0), ((p.p94 * var_qbe_qs_eff_db1) + var_qbc_qs_db1),)
    } else {
        (var_qbc, var_qbc_dn0, var_qbc_dn1, var_qbc_dn2, var_qbc_dn3, var_qbc_dn4, var_qbc_dn5, var_qbc_dn6, var_qbc_dn7, var_qbc_dn8, var_qbc_dn9, var_qbc_dn10, var_qbc_dn11, var_qbc_dn12, var_qbc_db0, var_qbc_db1,)
    }
};
        var_qbc = assign6850_e7145;
        var_qbc_dn0 = assign6850_e7145_d_n0;
        var_qbc_dn1 = assign6850_e7145_d_n1;
        var_qbc_dn2 = assign6850_e7145_d_n2;
        var_qbc_dn3 = assign6850_e7145_d_n3;
        var_qbc_dn4 = assign6850_e7145_d_n4;
        var_qbc_dn5 = assign6850_e7145_d_n5;
        var_qbc_dn6 = assign6850_e7145_d_n6;
        var_qbc_dn7 = assign6850_e7145_d_n7;
        var_qbc_dn8 = assign6850_e7145_d_n8;
        var_qbc_dn9 = assign6850_e7145_d_n9;
        var_qbc_dn10 = assign6850_e7145_d_n10;
        var_qbc_dn11 = assign6850_e7145_d_n11;
        var_qbc_dn12 = assign6850_e7145_d_n12;
        var_qbc_db0 = assign6850_e7145_d_b0;
        var_qbc_db1 = assign6850_e7145_d_b1;
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
        var_qbc_rdn12 = 0.0;
        var_qbc_rdb0 = 0.0;
        var_qbc_rdb1 = 0.0;

        let (assign6860_e7153, assign6860_e7153_d_n0, assign6860_e7153_d_n1, assign6860_e7153_d_n2, assign6860_e7153_d_n3, assign6860_e7153_d_n4, assign6860_e7153_d_n5, assign6860_e7153_d_n6, assign6860_e7153_d_n7, assign6860_e7153_d_n8, assign6860_e7153_d_n9, assign6860_e7153_d_n10, assign6860_e7153_d_n11, assign6860_e7153_d_n12, assign6860_e7153_d_b0, assign6860_e7153_d_b1,) = {
    if (var_guard123 != 0.0) {
        let assign6860_e7149: f64 = (1.0 - p.p94);
        let assign6860_e7151: f64 = (assign6860_e7149 * var_qbe_qs_eff);
        (assign6860_e7151, (assign6860_e7149 * var_qbe_qs_eff_dn0), (assign6860_e7149 * var_qbe_qs_eff_dn1), (assign6860_e7149 * var_qbe_qs_eff_dn2), (assign6860_e7149 * var_qbe_qs_eff_dn3), (assign6860_e7149 * var_qbe_qs_eff_dn4), (assign6860_e7149 * var_qbe_qs_eff_dn5), (assign6860_e7149 * var_qbe_qs_eff_dn6), (assign6860_e7149 * var_qbe_qs_eff_dn7), (assign6860_e7149 * var_qbe_qs_eff_dn8), (assign6860_e7149 * var_qbe_qs_eff_dn9), (assign6860_e7149 * var_qbe_qs_eff_dn10), (assign6860_e7149 * var_qbe_qs_eff_dn11), (assign6860_e7149 * var_qbe_qs_eff_dn12), (assign6860_e7149 * var_qbe_qs_eff_db0), (assign6860_e7149 * var_qbe_qs_eff_db1),)
    } else {
        (var_qbe, var_qbe_dn0, var_qbe_dn1, var_qbe_dn2, var_qbe_dn3, var_qbe_dn4, var_qbe_dn5, var_qbe_dn6, var_qbe_dn7, var_qbe_dn8, var_qbe_dn9, var_qbe_dn10, var_qbe_dn11, var_qbe_dn12, var_qbe_db0, var_qbe_db1,)
    }
};
        var_qbe = assign6860_e7153;
        var_qbe_dn0 = assign6860_e7153_d_n0;
        var_qbe_dn1 = assign6860_e7153_d_n1;
        var_qbe_dn2 = assign6860_e7153_d_n2;
        var_qbe_dn3 = assign6860_e7153_d_n3;
        var_qbe_dn4 = assign6860_e7153_d_n4;
        var_qbe_dn5 = assign6860_e7153_d_n5;
        var_qbe_dn6 = assign6860_e7153_d_n6;
        var_qbe_dn7 = assign6860_e7153_d_n7;
        var_qbe_dn8 = assign6860_e7153_d_n8;
        var_qbe_dn9 = assign6860_e7153_d_n9;
        var_qbe_dn10 = assign6860_e7153_d_n10;
        var_qbe_dn11 = assign6860_e7153_d_n11;
        var_qbe_dn12 = assign6860_e7153_d_n12;
        var_qbe_db0 = assign6860_e7153_d_b0;
        var_qbe_db1 = assign6860_e7153_d_b1;
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
        var_qbe_rdn12 = 0.0;
        var_qbe_rdb0 = 0.0;
        var_qbe_rdb1 = 0.0;

        let (assign6870_e7158, assign6870_e7158_d_n0, assign6870_e7158_d_n1, assign6870_e7158_d_n2, assign6870_e7158_d_n3, assign6870_e7158_d_n4, assign6870_e7158_d_n5, assign6870_e7158_d_n6, assign6870_e7158_d_n7, assign6870_e7158_d_n8, assign6870_e7158_d_n9, assign6870_e7158_d_n10, assign6870_e7158_d_n11, assign6870_e7158_d_n12, assign6870_e7158_d_b0, assign6870_e7158_d_b1,) = {
    if (var_guard123 == 0.0) {
        (var_qbe_qs, var_qbe_qs_dn0, var_qbe_qs_dn1, var_qbe_qs_dn2, var_qbe_qs_dn3, var_qbe_qs_dn4, var_qbe_qs_dn5, var_qbe_qs_dn6, var_qbe_qs_dn7, var_qbe_qs_dn8, var_qbe_qs_dn9, var_qbe_qs_dn10, var_qbe_qs_dn11, var_qbe_qs_dn12, var_qbe_qs_db0, var_qbe_qs_db1,)
    } else {
        (var_qbe, var_qbe_dn0, var_qbe_dn1, var_qbe_dn2, var_qbe_dn3, var_qbe_dn4, var_qbe_dn5, var_qbe_dn6, var_qbe_dn7, var_qbe_dn8, var_qbe_dn9, var_qbe_dn10, var_qbe_dn11, var_qbe_dn12, var_qbe_db0, var_qbe_db1,)
    }
};
        var_qbe = assign6870_e7158;
        var_qbe_dn0 = assign6870_e7158_d_n0;
        var_qbe_dn1 = assign6870_e7158_d_n1;
        var_qbe_dn2 = assign6870_e7158_d_n2;
        var_qbe_dn3 = assign6870_e7158_d_n3;
        var_qbe_dn4 = assign6870_e7158_d_n4;
        var_qbe_dn5 = assign6870_e7158_d_n5;
        var_qbe_dn6 = assign6870_e7158_d_n6;
        var_qbe_dn7 = assign6870_e7158_d_n7;
        var_qbe_dn8 = assign6870_e7158_d_n8;
        var_qbe_dn9 = assign6870_e7158_d_n9;
        var_qbe_dn10 = assign6870_e7158_d_n10;
        var_qbe_dn11 = assign6870_e7158_d_n11;
        var_qbe_dn12 = assign6870_e7158_d_n12;
        var_qbe_db0 = assign6870_e7158_d_b0;
        var_qbe_db1 = assign6870_e7158_d_b1;
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
        var_qbe_rdn12 = 0.0;
        var_qbe_rdb0 = 0.0;
        var_qbe_rdb1 = 0.0;

        let (assign6880_e7163, assign6880_e7163_d_n0, assign6880_e7163_d_n1, assign6880_e7163_d_n2, assign6880_e7163_d_n3, assign6880_e7163_d_n4, assign6880_e7163_d_n5, assign6880_e7163_d_n6, assign6880_e7163_d_n7, assign6880_e7163_d_n8, assign6880_e7163_d_n9, assign6880_e7163_d_n10, assign6880_e7163_d_n11, assign6880_e7163_d_n12, assign6880_e7163_d_b0, assign6880_e7163_d_b1,) = {
    if (var_guard123 == 0.0) {
        (var_qbc_qs, var_qbc_qs_dn0, var_qbc_qs_dn1, var_qbc_qs_dn2, var_qbc_qs_dn3, var_qbc_qs_dn4, var_qbc_qs_dn5, var_qbc_qs_dn6, var_qbc_qs_dn7, var_qbc_qs_dn8, var_qbc_qs_dn9, var_qbc_qs_dn10, var_qbc_qs_dn11, var_qbc_qs_dn12, var_qbc_qs_db0, var_qbc_qs_db1,)
    } else {
        (var_qbc, var_qbc_dn0, var_qbc_dn1, var_qbc_dn2, var_qbc_dn3, var_qbc_dn4, var_qbc_dn5, var_qbc_dn6, var_qbc_dn7, var_qbc_dn8, var_qbc_dn9, var_qbc_dn10, var_qbc_dn11, var_qbc_dn12, var_qbc_db0, var_qbc_db1,)
    }
};
        var_qbc = assign6880_e7163;
        var_qbc_dn0 = assign6880_e7163_d_n0;
        var_qbc_dn1 = assign6880_e7163_d_n1;
        var_qbc_dn2 = assign6880_e7163_d_n2;
        var_qbc_dn3 = assign6880_e7163_d_n3;
        var_qbc_dn4 = assign6880_e7163_d_n4;
        var_qbc_dn5 = assign6880_e7163_d_n5;
        var_qbc_dn6 = assign6880_e7163_d_n6;
        var_qbc_dn7 = assign6880_e7163_d_n7;
        var_qbc_dn8 = assign6880_e7163_d_n8;
        var_qbc_dn9 = assign6880_e7163_d_n9;
        var_qbc_dn10 = assign6880_e7163_d_n10;
        var_qbc_dn11 = assign6880_e7163_d_n11;
        var_qbc_dn12 = assign6880_e7163_d_n12;
        var_qbc_db0 = assign6880_e7163_d_b0;
        var_qbc_db1 = assign6880_e7163_d_b1;
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
        var_qbc_rdn12 = 0.0;
        var_qbc_rdb0 = 0.0;
        var_qbc_rdb1 = 0.0;

        let (assign6890_e7168, assign6890_e7168_d_n0, assign6890_e7168_d_n1, assign6890_e7168_d_n2, assign6890_e7168_d_n3, assign6890_e7168_d_n4, assign6890_e7168_d_n5, assign6890_e7168_d_n6, assign6890_e7168_d_n7, assign6890_e7168_d_n8, assign6890_e7168_d_n9, assign6890_e7168_d_n10, assign6890_e7168_d_n11, assign6890_e7168_d_n12, assign6890_e7168_d_b0, assign6890_e7168_d_b1,) = {
    if (var_guard123 == 0.0) {
        (var_qe_qs, var_qe_qs_dn0, var_qe_qs_dn1, var_qe_qs_dn2, var_qe_qs_dn3, var_qe_qs_dn4, var_qe_qs_dn5, var_qe_qs_dn6, var_qe_qs_dn7, var_qe_qs_dn8, var_qe_qs_dn9, var_qe_qs_dn10, var_qe_qs_dn11, var_qe_qs_dn12, var_qe_qs_db0, var_qe_qs_db1,)
    } else {
        (var_qe, var_qe_dn0, var_qe_dn1, var_qe_dn2, var_qe_dn3, var_qe_dn4, var_qe_dn5, var_qe_dn6, var_qe_dn7, var_qe_dn8, var_qe_dn9, var_qe_dn10, var_qe_dn11, var_qe_dn12, var_qe_db0, var_qe_db1,)
    }
};
        var_qe = assign6890_e7168;
        var_qe_dn0 = assign6890_e7168_d_n0;
        var_qe_dn1 = assign6890_e7168_d_n1;
        var_qe_dn2 = assign6890_e7168_d_n2;
        var_qe_dn3 = assign6890_e7168_d_n3;
        var_qe_dn4 = assign6890_e7168_d_n4;
        var_qe_dn5 = assign6890_e7168_d_n5;
        var_qe_dn6 = assign6890_e7168_d_n6;
        var_qe_dn7 = assign6890_e7168_d_n7;
        var_qe_dn8 = assign6890_e7168_d_n8;
        var_qe_dn9 = assign6890_e7168_d_n9;
        var_qe_dn10 = assign6890_e7168_d_n10;
        var_qe_dn11 = assign6890_e7168_d_n11;
        var_qe_dn12 = assign6890_e7168_d_n12;
        var_qe_db0 = assign6890_e7168_d_b0;
        var_qe_db1 = assign6890_e7168_d_b1;
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
        var_qe_rdn12 = 0.0;
        var_qe_rdb0 = 0.0;
        var_qe_rdb1 = 0.0;

        let assign6910_e7174: f64 = (p.p147 * (nv4 - 0.0));
        let assign6910_e7175_q: f64 = assign6910_e7174;
        let assign6910_e7177: f64 = (assign6910_e7174 * p.p1);
        let assign6910_e7177_q: f64 = (assign6910_e7175_q * p.p1);
        var_i_cth = assign6910_e7177;
        var_i_cth_dn0 = 0.0;
        var_i_cth_dn1 = 0.0;
        var_i_cth_dn2 = 0.0;
        var_i_cth_dn3 = 0.0;
        var_i_cth_dn4 = (p.p147 * p.p1);
        var_i_cth_dn5 = 0.0;
        var_i_cth_dn6 = 0.0;
        var_i_cth_dn7 = 0.0;
        var_i_cth_dn8 = 0.0;
        var_i_cth_dn9 = 0.0;
        var_i_cth_dn10 = 0.0;
        var_i_cth_dn11 = 0.0;
        var_i_cth_dn12 = 0.0;
        var_i_cth_db0 = 0.0;
        var_i_cth_db1 = 0.0;
        var_i_cth_rv = assign6910_e7177_q;
        var_i_cth_rdn0 = 0.0;
        var_i_cth_rdn1 = 0.0;
        var_i_cth_rdn2 = 0.0;
        var_i_cth_rdn3 = 0.0;
        var_i_cth_rdn4 = (p.p147 * p.p1);
        var_i_cth_rdn5 = 0.0;
        var_i_cth_rdn6 = 0.0;
        var_i_cth_rdn7 = 0.0;
        var_i_cth_rdn8 = 0.0;
        var_i_cth_rdn9 = 0.0;
        var_i_cth_rdn10 = 0.0;
        var_i_cth_rdn11 = 0.0;
        var_i_cth_rdn12 = 0.0;
        var_i_cth_rdb0 = 0.0;
        var_i_cth_rdb1 = 0.0;

        let assign7090_e7293: f64 = (var_if_ + var_ir);
        let assign7090_e7295: f64 = (assign7090_e7293 / var_qbi);
        var_in_n = assign7090_e7295;
        var_in_n_dn0 = ((((var_if__dn0 + var_ir_dn0) * var_qbi) - (assign7090_e7293 * var_qbi_dn0)) / (var_qbi * var_qbi));
        var_in_n_dn1 = ((((var_if__dn1 + var_ir_dn1) * var_qbi) - (assign7090_e7293 * var_qbi_dn1)) / (var_qbi * var_qbi));
        var_in_n_dn2 = ((((var_if__dn2 + var_ir_dn2) * var_qbi) - (assign7090_e7293 * var_qbi_dn2)) / (var_qbi * var_qbi));
        var_in_n_dn3 = ((((var_if__dn3 + var_ir_dn3) * var_qbi) - (assign7090_e7293 * var_qbi_dn3)) / (var_qbi * var_qbi));
        var_in_n_dn4 = ((((var_if__dn4 + var_ir_dn4) * var_qbi) - (assign7090_e7293 * var_qbi_dn4)) / (var_qbi * var_qbi));
        var_in_n_dn5 = ((((var_if__dn5 + var_ir_dn5) * var_qbi) - (assign7090_e7293 * var_qbi_dn5)) / (var_qbi * var_qbi));
        var_in_n_dn6 = ((((var_if__dn6 + var_ir_dn6) * var_qbi) - (assign7090_e7293 * var_qbi_dn6)) / (var_qbi * var_qbi));
        var_in_n_dn7 = ((((var_if__dn7 + var_ir_dn7) * var_qbi) - (assign7090_e7293 * var_qbi_dn7)) / (var_qbi * var_qbi));
        var_in_n_dn8 = ((((var_if__dn8 + var_ir_dn8) * var_qbi) - (assign7090_e7293 * var_qbi_dn8)) / (var_qbi * var_qbi));
        var_in_n_dn9 = ((((var_if__dn9 + var_ir_dn9) * var_qbi) - (assign7090_e7293 * var_qbi_dn9)) / (var_qbi * var_qbi));
        var_in_n_dn10 = ((((var_if__dn10 + var_ir_dn10) * var_qbi) - (assign7090_e7293 * var_qbi_dn10)) / (var_qbi * var_qbi));
        var_in_n_dn11 = ((((var_if__dn11 + var_ir_dn11) * var_qbi) - (assign7090_e7293 * var_qbi_dn11)) / (var_qbi * var_qbi));
        var_in_n_dn12 = ((((var_if__dn12 + var_ir_dn12) * var_qbi) - (assign7090_e7293 * var_qbi_dn12)) / (var_qbi * var_qbi));
        var_in_n_db0 = ((((var_if__db0 + var_ir_db0) * var_qbi) - (assign7090_e7293 * var_qbi_db0)) / (var_qbi * var_qbi));
        var_in_n_db1 = ((((var_if__db1 + var_ir_db1) * var_qbi) - (assign7090_e7293 * var_qbi_db1)) / (var_qbi * var_qbi));
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
        var_in_n_rdn12 = 0.0;
        var_in_n_rdb0 = 0.0;
        var_in_n_rdb1 = 0.0;

        let assign7150_e7328: f64 = if var_in_n > 0.0 { 1.0 } else { 0.0 };
        var_guard132 = assign7150_e7328;
        var_guard132_dn0 = 0.0;
        var_guard132_dn1 = 0.0;
        var_guard132_dn2 = 0.0;
        var_guard132_dn3 = 0.0;
        var_guard132_dn4 = 0.0;
        var_guard132_dn5 = 0.0;
        var_guard132_dn6 = 0.0;
        var_guard132_dn7 = 0.0;
        var_guard132_dn8 = 0.0;
        var_guard132_dn9 = 0.0;
        var_guard132_dn10 = 0.0;
        var_guard132_dn11 = 0.0;
        var_guard132_dn12 = 0.0;
        var_guard132_db0 = 0.0;
        var_guard132_db1 = 0.0;
        var_guard132_rv = 0.0;
        var_guard132_rdn0 = 0.0;
        var_guard132_rdn1 = 0.0;
        var_guard132_rdn2 = 0.0;
        var_guard132_rdn3 = 0.0;
        var_guard132_rdn4 = 0.0;
        var_guard132_rdn5 = 0.0;
        var_guard132_rdn6 = 0.0;
        var_guard132_rdn7 = 0.0;
        var_guard132_rdn8 = 0.0;
        var_guard132_rdn9 = 0.0;
        var_guard132_rdn10 = 0.0;
        var_guard132_rdn11 = 0.0;
        var_guard132_rdn12 = 0.0;
        var_guard132_rdb0 = 0.0;
        var_guard132_rdb1 = 0.0;

        let (assign7160_e7336, assign7160_e7336_d_n0, assign7160_e7336_d_n1, assign7160_e7336_d_n2, assign7160_e7336_d_n3, assign7160_e7336_d_n4, assign7160_e7336_d_n5, assign7160_e7336_d_n6, assign7160_e7336_d_n7, assign7160_e7336_d_n8, assign7160_e7336_d_n9, assign7160_e7336_d_n10, assign7160_e7336_d_n11, assign7160_e7336_d_n12, assign7160_e7336_d_b0, assign7160_e7336_d_b1,) = {
    if (var_guard132 != 0.0) {
        let assign7160_e7332: f64 = (var_qbe + var_qbc);
        let assign7160_e7334: f64 = (assign7160_e7332 / var_in_n);
        (assign7160_e7334, ((((var_qbe_dn0 + var_qbc_dn0) * var_in_n) - (assign7160_e7332 * var_in_n_dn0)) / (var_in_n * var_in_n)), ((((var_qbe_dn1 + var_qbc_dn1) * var_in_n) - (assign7160_e7332 * var_in_n_dn1)) / (var_in_n * var_in_n)), ((((var_qbe_dn2 + var_qbc_dn2) * var_in_n) - (assign7160_e7332 * var_in_n_dn2)) / (var_in_n * var_in_n)), ((((var_qbe_dn3 + var_qbc_dn3) * var_in_n) - (assign7160_e7332 * var_in_n_dn3)) / (var_in_n * var_in_n)), ((((var_qbe_dn4 + var_qbc_dn4) * var_in_n) - (assign7160_e7332 * var_in_n_dn4)) / (var_in_n * var_in_n)), ((((var_qbe_dn5 + var_qbc_dn5) * var_in_n) - (assign7160_e7332 * var_in_n_dn5)) / (var_in_n * var_in_n)), ((((var_qbe_dn6 + var_qbc_dn6) * var_in_n) - (assign7160_e7332 * var_in_n_dn6)) / (var_in_n * var_in_n)), ((((var_qbe_dn7 + var_qbc_dn7) * var_in_n) - (assign7160_e7332 * var_in_n_dn7)) / (var_in_n * var_in_n)), ((((var_qbe_dn8 + var_qbc_dn8) * var_in_n) - (assign7160_e7332 * var_in_n_dn8)) / (var_in_n * var_in_n)), ((((var_qbe_dn9 + var_qbc_dn9) * var_in_n) - (assign7160_e7332 * var_in_n_dn9)) / (var_in_n * var_in_n)), ((((var_qbe_dn10 + var_qbc_dn10) * var_in_n) - (assign7160_e7332 * var_in_n_dn10)) / (var_in_n * var_in_n)), ((((var_qbe_dn11 + var_qbc_dn11) * var_in_n) - (assign7160_e7332 * var_in_n_dn11)) / (var_in_n * var_in_n)), ((((var_qbe_dn12 + var_qbc_dn12) * var_in_n) - (assign7160_e7332 * var_in_n_dn12)) / (var_in_n * var_in_n)), ((((var_qbe_db0 + var_qbc_db0) * var_in_n) - (assign7160_e7332 * var_in_n_db0)) / (var_in_n * var_in_n)), ((((var_qbe_db1 + var_qbc_db1) * var_in_n) - (assign7160_e7332 * var_in_n_db1)) / (var_in_n * var_in_n)),)
    } else {
        (var_taub_n, var_taub_n_dn0, var_taub_n_dn1, var_taub_n_dn2, var_taub_n_dn3, var_taub_n_dn4, var_taub_n_dn5, var_taub_n_dn6, var_taub_n_dn7, var_taub_n_dn8, var_taub_n_dn9, var_taub_n_dn10, var_taub_n_dn11, var_taub_n_dn12, var_taub_n_db0, var_taub_n_db1,)
    }
};
        var_taub_n = assign7160_e7336;
        var_taub_n_dn0 = assign7160_e7336_d_n0;
        var_taub_n_dn1 = assign7160_e7336_d_n1;
        var_taub_n_dn2 = assign7160_e7336_d_n2;
        var_taub_n_dn3 = assign7160_e7336_d_n3;
        var_taub_n_dn4 = assign7160_e7336_d_n4;
        var_taub_n_dn5 = assign7160_e7336_d_n5;
        var_taub_n_dn6 = assign7160_e7336_d_n6;
        var_taub_n_dn7 = assign7160_e7336_d_n7;
        var_taub_n_dn8 = assign7160_e7336_d_n8;
        var_taub_n_dn9 = assign7160_e7336_d_n9;
        var_taub_n_dn10 = assign7160_e7336_d_n10;
        var_taub_n_dn11 = assign7160_e7336_d_n11;
        var_taub_n_dn12 = assign7160_e7336_d_n12;
        var_taub_n_db0 = assign7160_e7336_d_b0;
        var_taub_n_db1 = assign7160_e7336_d_b1;
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
        var_taub_n_rdn12 = 0.0;
        var_taub_n_rdb0 = 0.0;
        var_taub_n_rdb1 = 0.0;

        let (assign7170_e7345, assign7170_e7345_d_n0, assign7170_e7345_d_n1, assign7170_e7345_d_n2, assign7170_e7345_d_n3, assign7170_e7345_d_n4, assign7170_e7345_d_n5, assign7170_e7345_d_n6, assign7170_e7345_d_n7, assign7170_e7345_d_n8, assign7170_e7345_d_n9, assign7170_e7345_d_n10, assign7170_e7345_d_n11, assign7170_e7345_d_n12, assign7170_e7345_d_b0, assign7170_e7345_d_b1,) = {
    if (var_guard132 == 0.0) {
        let assign7170_e7341: f64 = (var_taub_t * var_q1q);
        let assign7170_e7343: f64 = (assign7170_e7341 * var_qbi);
        (assign7170_e7343, ((((var_taub_t_dn0 * var_q1q) + (var_taub_t * var_q1q_dn0)) * var_qbi) + (assign7170_e7341 * var_qbi_dn0)), ((((var_taub_t_dn1 * var_q1q) + (var_taub_t * var_q1q_dn1)) * var_qbi) + (assign7170_e7341 * var_qbi_dn1)), ((((var_taub_t_dn2 * var_q1q) + (var_taub_t * var_q1q_dn2)) * var_qbi) + (assign7170_e7341 * var_qbi_dn2)), ((((var_taub_t_dn3 * var_q1q) + (var_taub_t * var_q1q_dn3)) * var_qbi) + (assign7170_e7341 * var_qbi_dn3)), ((((var_taub_t_dn4 * var_q1q) + (var_taub_t * var_q1q_dn4)) * var_qbi) + (assign7170_e7341 * var_qbi_dn4)), ((((var_taub_t_dn5 * var_q1q) + (var_taub_t * var_q1q_dn5)) * var_qbi) + (assign7170_e7341 * var_qbi_dn5)), ((((var_taub_t_dn6 * var_q1q) + (var_taub_t * var_q1q_dn6)) * var_qbi) + (assign7170_e7341 * var_qbi_dn6)), ((((var_taub_t_dn7 * var_q1q) + (var_taub_t * var_q1q_dn7)) * var_qbi) + (assign7170_e7341 * var_qbi_dn7)), ((((var_taub_t_dn8 * var_q1q) + (var_taub_t * var_q1q_dn8)) * var_qbi) + (assign7170_e7341 * var_qbi_dn8)), ((((var_taub_t_dn9 * var_q1q) + (var_taub_t * var_q1q_dn9)) * var_qbi) + (assign7170_e7341 * var_qbi_dn9)), ((((var_taub_t_dn10 * var_q1q) + (var_taub_t * var_q1q_dn10)) * var_qbi) + (assign7170_e7341 * var_qbi_dn10)), ((((var_taub_t_dn11 * var_q1q) + (var_taub_t * var_q1q_dn11)) * var_qbi) + (assign7170_e7341 * var_qbi_dn11)), ((((var_taub_t_dn12 * var_q1q) + (var_taub_t * var_q1q_dn12)) * var_qbi) + (assign7170_e7341 * var_qbi_dn12)), ((((var_taub_t_db0 * var_q1q) + (var_taub_t * var_q1q_db0)) * var_qbi) + (assign7170_e7341 * var_qbi_db0)), ((((var_taub_t_db1 * var_q1q) + (var_taub_t * var_q1q_db1)) * var_qbi) + (assign7170_e7341 * var_qbi_db1)),)
    } else {
        (var_taub_n, var_taub_n_dn0, var_taub_n_dn1, var_taub_n_dn2, var_taub_n_dn3, var_taub_n_dn4, var_taub_n_dn5, var_taub_n_dn6, var_taub_n_dn7, var_taub_n_dn8, var_taub_n_dn9, var_taub_n_dn10, var_taub_n_dn11, var_taub_n_dn12, var_taub_n_db0, var_taub_n_db1,)
    }
};
        var_taub_n = assign7170_e7345;
        var_taub_n_dn0 = assign7170_e7345_d_n0;
        var_taub_n_dn1 = assign7170_e7345_d_n1;
        var_taub_n_dn2 = assign7170_e7345_d_n2;
        var_taub_n_dn3 = assign7170_e7345_d_n3;
        var_taub_n_dn4 = assign7170_e7345_d_n4;
        var_taub_n_dn5 = assign7170_e7345_d_n5;
        var_taub_n_dn6 = assign7170_e7345_d_n6;
        var_taub_n_dn7 = assign7170_e7345_d_n7;
        var_taub_n_dn8 = assign7170_e7345_d_n8;
        var_taub_n_dn9 = assign7170_e7345_d_n9;
        var_taub_n_dn10 = assign7170_e7345_d_n10;
        var_taub_n_dn11 = assign7170_e7345_d_n11;
        var_taub_n_dn12 = assign7170_e7345_d_n12;
        var_taub_n_db0 = assign7170_e7345_d_b0;
        var_taub_n_db1 = assign7170_e7345_d_b1;
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
        var_taub_n_rdn12 = 0.0;
        var_taub_n_rdb0 = 0.0;
        var_taub_n_rdb1 = 0.0;

        let assign7180_e7348: f64 = if p.p131 == 1.0 { 1.0 } else { 0.0 };
        var_guard133 = assign7180_e7348;
        var_guard133_dn0 = 0.0;
        var_guard133_dn1 = 0.0;
        var_guard133_dn2 = 0.0;
        var_guard133_dn3 = 0.0;
        var_guard133_dn4 = 0.0;
        var_guard133_dn5 = 0.0;
        var_guard133_dn6 = 0.0;
        var_guard133_dn7 = 0.0;
        var_guard133_dn8 = 0.0;
        var_guard133_dn9 = 0.0;
        var_guard133_dn10 = 0.0;
        var_guard133_dn11 = 0.0;
        var_guard133_dn12 = 0.0;
        var_guard133_db0 = 0.0;
        var_guard133_db1 = 0.0;
        var_guard133_rv = 0.0;
        var_guard133_rdn0 = 0.0;
        var_guard133_rdn1 = 0.0;
        var_guard133_rdn2 = 0.0;
        var_guard133_rdn3 = 0.0;
        var_guard133_rdn4 = 0.0;
        var_guard133_rdn5 = 0.0;
        var_guard133_rdn6 = 0.0;
        var_guard133_rdn7 = 0.0;
        var_guard133_rdn8 = 0.0;
        var_guard133_rdn9 = 0.0;
        var_guard133_rdn10 = 0.0;
        var_guard133_rdn11 = 0.0;
        var_guard133_rdn12 = 0.0;
        var_guard133_rdb0 = 0.0;
        var_guard133_rdb1 = 0.0;

        *var_guard132_slot = var_guard132;
        *var_guard132_db0_slot = var_guard132_db0;
        *var_guard132_db1_slot = var_guard132_db1;
        *var_guard132_dn0_slot = var_guard132_dn0;
        *var_guard132_dn1_slot = var_guard132_dn1;
        *var_guard132_dn10_slot = var_guard132_dn10;
        *var_guard132_dn11_slot = var_guard132_dn11;
        *var_guard132_dn12_slot = var_guard132_dn12;
        *var_guard132_dn2_slot = var_guard132_dn2;
        *var_guard132_dn3_slot = var_guard132_dn3;
        *var_guard132_dn4_slot = var_guard132_dn4;
        *var_guard132_dn5_slot = var_guard132_dn5;
        *var_guard132_dn6_slot = var_guard132_dn6;
        *var_guard132_dn7_slot = var_guard132_dn7;
        *var_guard132_dn8_slot = var_guard132_dn8;
        *var_guard132_dn9_slot = var_guard132_dn9;
        *var_guard132_rdb0_slot = var_guard132_rdb0;
        *var_guard132_rdb1_slot = var_guard132_rdb1;
        *var_guard132_rdn0_slot = var_guard132_rdn0;
        *var_guard132_rdn1_slot = var_guard132_rdn1;
        *var_guard132_rdn10_slot = var_guard132_rdn10;
        *var_guard132_rdn11_slot = var_guard132_rdn11;
        *var_guard132_rdn12_slot = var_guard132_rdn12;
        *var_guard132_rdn2_slot = var_guard132_rdn2;
        *var_guard132_rdn3_slot = var_guard132_rdn3;
        *var_guard132_rdn4_slot = var_guard132_rdn4;
        *var_guard132_rdn5_slot = var_guard132_rdn5;
        *var_guard132_rdn6_slot = var_guard132_rdn6;
        *var_guard132_rdn7_slot = var_guard132_rdn7;
        *var_guard132_rdn8_slot = var_guard132_rdn8;
        *var_guard132_rdn9_slot = var_guard132_rdn9;
        *var_guard132_rv_slot = var_guard132_rv;
        *var_guard133_slot = var_guard133;
        *var_guard133_db0_slot = var_guard133_db0;
        *var_guard133_db1_slot = var_guard133_db1;
        *var_guard133_dn0_slot = var_guard133_dn0;
        *var_guard133_dn1_slot = var_guard133_dn1;
        *var_guard133_dn10_slot = var_guard133_dn10;
        *var_guard133_dn11_slot = var_guard133_dn11;
        *var_guard133_dn12_slot = var_guard133_dn12;
        *var_guard133_dn2_slot = var_guard133_dn2;
        *var_guard133_dn3_slot = var_guard133_dn3;
        *var_guard133_dn4_slot = var_guard133_dn4;
        *var_guard133_dn5_slot = var_guard133_dn5;
        *var_guard133_dn6_slot = var_guard133_dn6;
        *var_guard133_dn7_slot = var_guard133_dn7;
        *var_guard133_dn8_slot = var_guard133_dn8;
        *var_guard133_dn9_slot = var_guard133_dn9;
        *var_guard133_rdb0_slot = var_guard133_rdb0;
        *var_guard133_rdb1_slot = var_guard133_rdb1;
        *var_guard133_rdn0_slot = var_guard133_rdn0;
        *var_guard133_rdn1_slot = var_guard133_rdn1;
        *var_guard133_rdn10_slot = var_guard133_rdn10;
        *var_guard133_rdn11_slot = var_guard133_rdn11;
        *var_guard133_rdn12_slot = var_guard133_rdn12;
        *var_guard133_rdn2_slot = var_guard133_rdn2;
        *var_guard133_rdn3_slot = var_guard133_rdn3;
        *var_guard133_rdn4_slot = var_guard133_rdn4;
        *var_guard133_rdn5_slot = var_guard133_rdn5;
        *var_guard133_rdn6_slot = var_guard133_rdn6;
        *var_guard133_rdn7_slot = var_guard133_rdn7;
        *var_guard133_rdn8_slot = var_guard133_rdn8;
        *var_guard133_rdn9_slot = var_guard133_rdn9;
        *var_guard133_rv_slot = var_guard133_rv;
        *var_i_cth_slot = var_i_cth;
        *var_i_cth_db0_slot = var_i_cth_db0;
        *var_i_cth_db1_slot = var_i_cth_db1;
        *var_i_cth_dn0_slot = var_i_cth_dn0;
        *var_i_cth_dn1_slot = var_i_cth_dn1;
        *var_i_cth_dn10_slot = var_i_cth_dn10;
        *var_i_cth_dn11_slot = var_i_cth_dn11;
        *var_i_cth_dn12_slot = var_i_cth_dn12;
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
        *var_i_cth_rdn12_slot = var_i_cth_rdn12;
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
        *var_in_n_dn12_slot = var_in_n_dn12;
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
        *var_in_n_rdn12_slot = var_in_n_rdn12;
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
        *var_qbc_dn12_slot = var_qbc_dn12;
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
        *var_qbc_rdn12_slot = var_qbc_rdn12;
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
        *var_qbe_dn12_slot = var_qbe_dn12;
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
        *var_qbe_qs_eff_dn12_slot = var_qbe_qs_eff_dn12;
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
        *var_qbe_qs_eff_rdn12_slot = var_qbe_qs_eff_rdn12;
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
        *var_qbe_rdn12_slot = var_qbe_rdn12;
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
        *var_qe_dn12_slot = var_qe_dn12;
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
        *var_qe_rdn12_slot = var_qe_rdn12;
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
        *var_taub_n_dn12_slot = var_taub_n_dn12;
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
        *var_taub_n_rdn12_slot = var_taub_n_rdn12;
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

    pub(super) fn stamp_reactive_block_45(
        p: &Parameters,
        var_guard133: f64,
        var_taub_n: f64,
        var_taub_n_db0: f64,
        var_taub_n_db1: f64,
        var_taub_n_dn0: f64,
        var_taub_n_dn1: f64,
        var_taub_n_dn10: f64,
        var_taub_n_dn11: f64,
        var_taub_n_dn12: f64,
        var_taub_n_dn2: f64,
        var_taub_n_dn3: f64,
        var_taub_n_dn4: f64,
        var_taub_n_dn5: f64,
        var_taub_n_dn6: f64,
        var_taub_n_dn7: f64,
        var_taub_n_dn8: f64,
        var_taub_n_dn9: f64,
        var_guard134_slot: &mut f64,
        var_guard134_db0_slot: &mut f64,
        var_guard134_db1_slot: &mut f64,
        var_guard134_dn0_slot: &mut f64,
        var_guard134_dn1_slot: &mut f64,
        var_guard134_dn10_slot: &mut f64,
        var_guard134_dn11_slot: &mut f64,
        var_guard134_dn12_slot: &mut f64,
        var_guard134_dn2_slot: &mut f64,
        var_guard134_dn3_slot: &mut f64,
        var_guard134_dn4_slot: &mut f64,
        var_guard134_dn5_slot: &mut f64,
        var_guard134_dn6_slot: &mut f64,
        var_guard134_dn7_slot: &mut f64,
        var_guard134_dn8_slot: &mut f64,
        var_guard134_dn9_slot: &mut f64,
        var_guard134_rdb0_slot: &mut f64,
        var_guard134_rdb1_slot: &mut f64,
        var_guard134_rdn0_slot: &mut f64,
        var_guard134_rdn1_slot: &mut f64,
        var_guard134_rdn10_slot: &mut f64,
        var_guard134_rdn11_slot: &mut f64,
        var_guard134_rdn12_slot: &mut f64,
        var_guard134_rdn2_slot: &mut f64,
        var_guard134_rdn3_slot: &mut f64,
        var_guard134_rdn4_slot: &mut f64,
        var_guard134_rdn5_slot: &mut f64,
        var_guard134_rdn6_slot: &mut f64,
        var_guard134_rdn7_slot: &mut f64,
        var_guard134_rdn8_slot: &mut f64,
        var_guard134_rdn9_slot: &mut f64,
        var_guard134_rv_slot: &mut f64,
        var_taun_slot: &mut f64,
        var_taun_db0_slot: &mut f64,
        var_taun_db1_slot: &mut f64,
        var_taun_dn0_slot: &mut f64,
        var_taun_dn1_slot: &mut f64,
        var_taun_dn10_slot: &mut f64,
        var_taun_dn11_slot: &mut f64,
        var_taun_dn12_slot: &mut f64,
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
        var_taun_rdn12_slot: &mut f64,
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
        let mut var_guard134: f64 = *var_guard134_slot;
        let mut var_guard134_db0: f64 = *var_guard134_db0_slot;
        let mut var_guard134_db1: f64 = *var_guard134_db1_slot;
        let mut var_guard134_dn0: f64 = *var_guard134_dn0_slot;
        let mut var_guard134_dn1: f64 = *var_guard134_dn1_slot;
        let mut var_guard134_dn10: f64 = *var_guard134_dn10_slot;
        let mut var_guard134_dn11: f64 = *var_guard134_dn11_slot;
        let mut var_guard134_dn12: f64 = *var_guard134_dn12_slot;
        let mut var_guard134_dn2: f64 = *var_guard134_dn2_slot;
        let mut var_guard134_dn3: f64 = *var_guard134_dn3_slot;
        let mut var_guard134_dn4: f64 = *var_guard134_dn4_slot;
        let mut var_guard134_dn5: f64 = *var_guard134_dn5_slot;
        let mut var_guard134_dn6: f64 = *var_guard134_dn6_slot;
        let mut var_guard134_dn7: f64 = *var_guard134_dn7_slot;
        let mut var_guard134_dn8: f64 = *var_guard134_dn8_slot;
        let mut var_guard134_dn9: f64 = *var_guard134_dn9_slot;
        let mut var_guard134_rdb0: f64 = *var_guard134_rdb0_slot;
        let mut var_guard134_rdb1: f64 = *var_guard134_rdb1_slot;
        let mut var_guard134_rdn0: f64 = *var_guard134_rdn0_slot;
        let mut var_guard134_rdn1: f64 = *var_guard134_rdn1_slot;
        let mut var_guard134_rdn10: f64 = *var_guard134_rdn10_slot;
        let mut var_guard134_rdn11: f64 = *var_guard134_rdn11_slot;
        let mut var_guard134_rdn12: f64 = *var_guard134_rdn12_slot;
        let mut var_guard134_rdn2: f64 = *var_guard134_rdn2_slot;
        let mut var_guard134_rdn3: f64 = *var_guard134_rdn3_slot;
        let mut var_guard134_rdn4: f64 = *var_guard134_rdn4_slot;
        let mut var_guard134_rdn5: f64 = *var_guard134_rdn5_slot;
        let mut var_guard134_rdn6: f64 = *var_guard134_rdn6_slot;
        let mut var_guard134_rdn7: f64 = *var_guard134_rdn7_slot;
        let mut var_guard134_rdn8: f64 = *var_guard134_rdn8_slot;
        let mut var_guard134_rdn9: f64 = *var_guard134_rdn9_slot;
        let mut var_guard134_rv: f64 = *var_guard134_rv_slot;
        let mut var_taun: f64 = *var_taun_slot;
        let mut var_taun_db0: f64 = *var_taun_db0_slot;
        let mut var_taun_db1: f64 = *var_taun_db1_slot;
        let mut var_taun_dn0: f64 = *var_taun_dn0_slot;
        let mut var_taun_dn1: f64 = *var_taun_dn1_slot;
        let mut var_taun_dn10: f64 = *var_taun_dn10_slot;
        let mut var_taun_dn11: f64 = *var_taun_dn11_slot;
        let mut var_taun_dn12: f64 = *var_taun_dn12_slot;
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
        let mut var_taun_rdn12: f64 = *var_taun_rdn12_slot;
        let mut var_taun_rdn2: f64 = *var_taun_rdn2_slot;
        let mut var_taun_rdn3: f64 = *var_taun_rdn3_slot;
        let mut var_taun_rdn4: f64 = *var_taun_rdn4_slot;
        let mut var_taun_rdn5: f64 = *var_taun_rdn5_slot;
        let mut var_taun_rdn6: f64 = *var_taun_rdn6_slot;
        let mut var_taun_rdn7: f64 = *var_taun_rdn7_slot;
        let mut var_taun_rdn8: f64 = *var_taun_rdn8_slot;
        let mut var_taun_rdn9: f64 = *var_taun_rdn9_slot;
        let mut var_taun_rv: f64 = *var_taun_rv_slot;

        let (assign7190_e7354, assign7190_e7354_d_n0, assign7190_e7354_d_n1, assign7190_e7354_d_n2, assign7190_e7354_d_n3, assign7190_e7354_d_n4, assign7190_e7354_d_n5, assign7190_e7354_d_n6, assign7190_e7354_d_n7, assign7190_e7354_d_n8, assign7190_e7354_d_n9, assign7190_e7354_d_n10, assign7190_e7354_d_n11, assign7190_e7354_d_n12, assign7190_e7354_d_b0, assign7190_e7354_d_b1,) = {
    if (var_guard133 != 0.0) {
        let assign7190_e7352: f64 = (p.p94 * var_taub_n);
        (assign7190_e7352, (p.p94 * var_taub_n_dn0), (p.p94 * var_taub_n_dn1), (p.p94 * var_taub_n_dn2), (p.p94 * var_taub_n_dn3), (p.p94 * var_taub_n_dn4), (p.p94 * var_taub_n_dn5), (p.p94 * var_taub_n_dn6), (p.p94 * var_taub_n_dn7), (p.p94 * var_taub_n_dn8), (p.p94 * var_taub_n_dn9), (p.p94 * var_taub_n_dn10), (p.p94 * var_taub_n_dn11), (p.p94 * var_taub_n_dn12), (p.p94 * var_taub_n_db0), (p.p94 * var_taub_n_db1),)
    } else {
        (var_taun, var_taun_dn0, var_taun_dn1, var_taun_dn2, var_taun_dn3, var_taun_dn4, var_taun_dn5, var_taun_dn6, var_taun_dn7, var_taun_dn8, var_taun_dn9, var_taun_dn10, var_taun_dn11, var_taun_dn12, var_taun_db0, var_taun_db1,)
    }
};
        var_taun = assign7190_e7354;
        var_taun_dn0 = assign7190_e7354_d_n0;
        var_taun_dn1 = assign7190_e7354_d_n1;
        var_taun_dn2 = assign7190_e7354_d_n2;
        var_taun_dn3 = assign7190_e7354_d_n3;
        var_taun_dn4 = assign7190_e7354_d_n4;
        var_taun_dn5 = assign7190_e7354_d_n5;
        var_taun_dn6 = assign7190_e7354_d_n6;
        var_taun_dn7 = assign7190_e7354_d_n7;
        var_taun_dn8 = assign7190_e7354_d_n8;
        var_taun_dn9 = assign7190_e7354_d_n9;
        var_taun_dn10 = assign7190_e7354_d_n10;
        var_taun_dn11 = assign7190_e7354_d_n11;
        var_taun_dn12 = assign7190_e7354_d_n12;
        var_taun_db0 = assign7190_e7354_d_b0;
        var_taun_db1 = assign7190_e7354_d_b1;
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
        var_taun_rdn12 = 0.0;
        var_taun_rdb0 = 0.0;
        var_taun_rdb1 = 0.0;

        let assign7200_e7357: f64 = if p.p131 == 2.0 { 1.0 } else { 0.0 };
        var_guard134 = assign7200_e7357;
        var_guard134_dn0 = 0.0;
        var_guard134_dn1 = 0.0;
        var_guard134_dn2 = 0.0;
        var_guard134_dn3 = 0.0;
        var_guard134_dn4 = 0.0;
        var_guard134_dn5 = 0.0;
        var_guard134_dn6 = 0.0;
        var_guard134_dn7 = 0.0;
        var_guard134_dn8 = 0.0;
        var_guard134_dn9 = 0.0;
        var_guard134_dn10 = 0.0;
        var_guard134_dn11 = 0.0;
        var_guard134_dn12 = 0.0;
        var_guard134_db0 = 0.0;
        var_guard134_db1 = 0.0;
        var_guard134_rv = 0.0;
        var_guard134_rdn0 = 0.0;
        var_guard134_rdn1 = 0.0;
        var_guard134_rdn2 = 0.0;
        var_guard134_rdn3 = 0.0;
        var_guard134_rdn4 = 0.0;
        var_guard134_rdn5 = 0.0;
        var_guard134_rdn6 = 0.0;
        var_guard134_rdn7 = 0.0;
        var_guard134_rdn8 = 0.0;
        var_guard134_rdn9 = 0.0;
        var_guard134_rdn10 = 0.0;
        var_guard134_rdn11 = 0.0;
        var_guard134_rdn12 = 0.0;
        var_guard134_rdb0 = 0.0;
        var_guard134_rdb1 = 0.0;

        let (assign7210_e7366, assign7210_e7366_d_n0, assign7210_e7366_d_n1, assign7210_e7366_d_n2, assign7210_e7366_d_n3, assign7210_e7366_d_n4, assign7210_e7366_d_n5, assign7210_e7366_d_n6, assign7210_e7366_d_n7, assign7210_e7366_d_n8, assign7210_e7366_d_n9, assign7210_e7366_d_n10, assign7210_e7366_d_n11, assign7210_e7366_d_n12, assign7210_e7366_d_b0, assign7210_e7366_d_b1,) = {
    if ((var_guard133 == 0.0) && (var_guard134 != 0.0)) {
        let assign7210_e7364: f64 = (p.p132 * var_taub_n);
        (assign7210_e7364, (p.p132 * var_taub_n_dn0), (p.p132 * var_taub_n_dn1), (p.p132 * var_taub_n_dn2), (p.p132 * var_taub_n_dn3), (p.p132 * var_taub_n_dn4), (p.p132 * var_taub_n_dn5), (p.p132 * var_taub_n_dn6), (p.p132 * var_taub_n_dn7), (p.p132 * var_taub_n_dn8), (p.p132 * var_taub_n_dn9), (p.p132 * var_taub_n_dn10), (p.p132 * var_taub_n_dn11), (p.p132 * var_taub_n_dn12), (p.p132 * var_taub_n_db0), (p.p132 * var_taub_n_db1),)
    } else {
        (var_taun, var_taun_dn0, var_taun_dn1, var_taun_dn2, var_taun_dn3, var_taun_dn4, var_taun_dn5, var_taun_dn6, var_taun_dn7, var_taun_dn8, var_taun_dn9, var_taun_dn10, var_taun_dn11, var_taun_dn12, var_taun_db0, var_taun_db1,)
    }
};
        var_taun = assign7210_e7366;
        var_taun_dn0 = assign7210_e7366_d_n0;
        var_taun_dn1 = assign7210_e7366_d_n1;
        var_taun_dn2 = assign7210_e7366_d_n2;
        var_taun_dn3 = assign7210_e7366_d_n3;
        var_taun_dn4 = assign7210_e7366_d_n4;
        var_taun_dn5 = assign7210_e7366_d_n5;
        var_taun_dn6 = assign7210_e7366_d_n6;
        var_taun_dn7 = assign7210_e7366_d_n7;
        var_taun_dn8 = assign7210_e7366_d_n8;
        var_taun_dn9 = assign7210_e7366_d_n9;
        var_taun_dn10 = assign7210_e7366_d_n10;
        var_taun_dn11 = assign7210_e7366_d_n11;
        var_taun_dn12 = assign7210_e7366_d_n12;
        var_taun_db0 = assign7210_e7366_d_b0;
        var_taun_db1 = assign7210_e7366_d_b1;
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
        var_taun_rdn12 = 0.0;
        var_taun_rdb0 = 0.0;
        var_taun_rdb1 = 0.0;

        let (assign7220_e7374, assign7220_e7374_d_n0, assign7220_e7374_d_n1, assign7220_e7374_d_n2, assign7220_e7374_d_n3, assign7220_e7374_d_n4, assign7220_e7374_d_n5, assign7220_e7374_d_n6, assign7220_e7374_d_n7, assign7220_e7374_d_n8, assign7220_e7374_d_n9, assign7220_e7374_d_n10, assign7220_e7374_d_n11, assign7220_e7374_d_n12, assign7220_e7374_d_b0, assign7220_e7374_d_b1,) = {
    if ((var_guard133 == 0.0) && (var_guard134 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_taun, var_taun_dn0, var_taun_dn1, var_taun_dn2, var_taun_dn3, var_taun_dn4, var_taun_dn5, var_taun_dn6, var_taun_dn7, var_taun_dn8, var_taun_dn9, var_taun_dn10, var_taun_dn11, var_taun_dn12, var_taun_db0, var_taun_db1,)
    }
};
        var_taun = assign7220_e7374;
        var_taun_dn0 = assign7220_e7374_d_n0;
        var_taun_dn1 = assign7220_e7374_d_n1;
        var_taun_dn2 = assign7220_e7374_d_n2;
        var_taun_dn3 = assign7220_e7374_d_n3;
        var_taun_dn4 = assign7220_e7374_d_n4;
        var_taun_dn5 = assign7220_e7374_d_n5;
        var_taun_dn6 = assign7220_e7374_d_n6;
        var_taun_dn7 = assign7220_e7374_d_n7;
        var_taun_dn8 = assign7220_e7374_d_n8;
        var_taun_dn9 = assign7220_e7374_d_n9;
        var_taun_dn10 = assign7220_e7374_d_n10;
        var_taun_dn11 = assign7220_e7374_d_n11;
        var_taun_dn12 = assign7220_e7374_d_n12;
        var_taun_db0 = assign7220_e7374_d_b0;
        var_taun_db1 = assign7220_e7374_d_b1;
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
        var_taun_rdn12 = 0.0;
        var_taun_rdb0 = 0.0;
        var_taun_rdb1 = 0.0;

        *var_guard134_slot = var_guard134;
        *var_guard134_db0_slot = var_guard134_db0;
        *var_guard134_db1_slot = var_guard134_db1;
        *var_guard134_dn0_slot = var_guard134_dn0;
        *var_guard134_dn1_slot = var_guard134_dn1;
        *var_guard134_dn10_slot = var_guard134_dn10;
        *var_guard134_dn11_slot = var_guard134_dn11;
        *var_guard134_dn12_slot = var_guard134_dn12;
        *var_guard134_dn2_slot = var_guard134_dn2;
        *var_guard134_dn3_slot = var_guard134_dn3;
        *var_guard134_dn4_slot = var_guard134_dn4;
        *var_guard134_dn5_slot = var_guard134_dn5;
        *var_guard134_dn6_slot = var_guard134_dn6;
        *var_guard134_dn7_slot = var_guard134_dn7;
        *var_guard134_dn8_slot = var_guard134_dn8;
        *var_guard134_dn9_slot = var_guard134_dn9;
        *var_guard134_rdb0_slot = var_guard134_rdb0;
        *var_guard134_rdb1_slot = var_guard134_rdb1;
        *var_guard134_rdn0_slot = var_guard134_rdn0;
        *var_guard134_rdn1_slot = var_guard134_rdn1;
        *var_guard134_rdn10_slot = var_guard134_rdn10;
        *var_guard134_rdn11_slot = var_guard134_rdn11;
        *var_guard134_rdn12_slot = var_guard134_rdn12;
        *var_guard134_rdn2_slot = var_guard134_rdn2;
        *var_guard134_rdn3_slot = var_guard134_rdn3;
        *var_guard134_rdn4_slot = var_guard134_rdn4;
        *var_guard134_rdn5_slot = var_guard134_rdn5;
        *var_guard134_rdn6_slot = var_guard134_rdn6;
        *var_guard134_rdn7_slot = var_guard134_rdn7;
        *var_guard134_rdn8_slot = var_guard134_rdn8;
        *var_guard134_rdn9_slot = var_guard134_rdn9;
        *var_guard134_rv_slot = var_guard134_rv;
        *var_taun_slot = var_taun;
        *var_taun_db0_slot = var_taun_db0;
        *var_taun_db1_slot = var_taun_db1;
        *var_taun_dn0_slot = var_taun_dn0;
        *var_taun_dn1_slot = var_taun_dn1;
        *var_taun_dn10_slot = var_taun_dn10;
        *var_taun_dn11_slot = var_taun_dn11;
        *var_taun_dn12_slot = var_taun_dn12;
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
        *var_taun_rdn12_slot = var_taun_rdn12;
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
        var_guard125: f64,
        var_ib1: f64,
        var_ib1_dn0: f64,
        var_ib1_dn1: f64,
        var_ib1_dn10: f64,
        var_ib1_dn11: f64,
        var_ib1_dn3: f64,
        var_ib1_dn4: f64,
        var_ib1_dn5: f64,
        var_ib1_dn6: f64,
        var_ib1_dn7: f64,
        var_ib1_dn8: f64,
        var_ib1_dn9: f64,
        var_ib1_s: f64,
        var_ib1_s_dn0: f64,
        var_ib1_s_dn1: f64,
        var_ib1_s_dn10: f64,
        var_ib1_s_dn11: f64,
        var_ib1_s_dn3: f64,
        var_ib1_s_dn4: f64,
        var_ib1_s_dn5: f64,
        var_ib1_s_dn6: f64,
        var_ib1_s_dn7: f64,
        var_ib1_s_dn8: f64,
        var_ib1_s_dn9: f64,
        var_ib1b2: f64,
        var_ib1b2_dn0: f64,
        var_ib1b2_dn1: f64,
        var_ib1b2_dn10: f64,
        var_ib1b2_dn11: f64,
        var_ib1b2_dn3: f64,
        var_ib1b2_dn4: f64,
        var_ib1b2_dn5: f64,
        var_ib1b2_dn6: f64,
        var_ib1b2_dn7: f64,
        var_ib1b2_dn8: f64,
        var_ib1b2_dn9: f64,
        var_ib2: f64,
        var_ib2_dn0: f64,
        var_ib2_dn1: f64,
        var_ib2_dn10: f64,
        var_ib2_dn11: f64,
        var_ib2_dn3: f64,
        var_ib2_dn4: f64,
        var_ib2_dn5: f64,
        var_ib2_dn6: f64,
        var_ib2_dn7: f64,
        var_ib2_dn8: f64,
        var_ib2_dn9: f64,
        var_ib2_s: f64,
        var_ib2_s_dn0: f64,
        var_ib2_s_dn1: f64,
        var_ib2_s_dn10: f64,
        var_ib2_s_dn11: f64,
        var_ib2_s_dn3: f64,
        var_ib2_s_dn4: f64,
        var_ib2_s_dn5: f64,
        var_ib2_s_dn6: f64,
        var_ib2_s_dn7: f64,
        var_ib2_s_dn8: f64,
        var_ib2_s_dn9: f64,
        var_ibrel: f64,
        var_ibrel_dn0: f64,
        var_ibrel_dn1: f64,
        var_ibrel_dn10: f64,
        var_ibrel_dn11: f64,
        var_ibrel_dn3: f64,
        var_ibrel_dn4: f64,
        var_ibrel_dn5: f64,
        var_ibrel_dn6: f64,
        var_ibrel_dn7: f64,
        var_ibrel_dn8: f64,
        var_ibrel_dn9: f64,
        var_ibtbt: f64,
        var_ibtbt_dn0: f64,
        var_ibtbt_dn1: f64,
        var_ibtbt_dn10: f64,
        var_ibtbt_dn11: f64,
        var_ibtbt_dn3: f64,
        var_ibtbt_dn4: f64,
        var_ibtbt_dn5: f64,
        var_ibtbt_dn6: f64,
        var_ibtbt_dn7: f64,
        var_ibtbt_dn8: f64,
        var_ibtbt_dn9: f64,
        var_ic1c2: f64,
        var_ic1c2_dn0: f64,
        var_ic1c2_dn1: f64,
        var_ic1c2_dn10: f64,
        var_ic1c2_dn11: f64,
        var_ic1c2_dn3: f64,
        var_ic1c2_dn4: f64,
        var_ic1c2_dn5: f64,
        var_ic1c2_dn6: f64,
        var_ic1c2_dn7: f64,
        var_ic1c2_dn8: f64,
        var_ic1c2_dn9: f64,
        var_in_: f64,
        var_in__dn0: f64,
        var_in__dn1: f64,
        var_in__dn10: f64,
        var_in__dn11: f64,
        var_in__dn3: f64,
        var_in__dn4: f64,
        var_in__dn5: f64,
        var_in__dn6: f64,
        var_in__dn7: f64,
        var_in__dn8: f64,
        var_in__dn9: f64,
        var_isf: f64,
        var_isf_dn3: f64,
        var_isf_dn4: f64,
        var_isf_dn8: f64,
        var_isub: f64,
        var_isub_dn11: f64,
        var_isub_dn3: f64,
        var_isub_dn4: f64,
        var_isub_dn6: f64,
        var_isub_dn7: f64,
        var_isub_dn8: f64,
        var_isub_dn9: f64,
        var_isub_int: f64,
        var_isub_int_dn3: f64,
        var_isub_int_dn4: f64,
        var_isub_int_dn7: f64,
        var_isub_int_dn8: f64,
        var_isub_int_dn9: f64,
        var_itat: f64,
        var_itat_dn0: f64,
        var_itat_dn1: f64,
        var_itat_dn10: f64,
        var_itat_dn11: f64,
        var_itat_dn3: f64,
        var_itat_dn4: f64,
        var_itat_dn5: f64,
        var_itat_dn6: f64,
        var_itat_dn7: f64,
        var_itat_dn8: f64,
        var_itat_dn9: f64,
        var_iztcb: f64,
        var_iztcb_dn0: f64,
        var_iztcb_dn1: f64,
        var_iztcb_dn10: f64,
        var_iztcb_dn11: f64,
        var_iztcb_dn3: f64,
        var_iztcb_dn4: f64,
        var_iztcb_dn5: f64,
        var_iztcb_dn6: f64,
        var_iztcb_dn7: f64,
        var_iztcb_dn8: f64,
        var_iztcb_dn9: f64,
        var_izteb: f64,
        var_izteb_dn0: f64,
        var_izteb_dn1: f64,
        var_izteb_dn10: f64,
        var_izteb_dn11: f64,
        var_izteb_dn3: f64,
        var_izteb_dn4: f64,
        var_izteb_dn5: f64,
        var_izteb_dn6: f64,
        var_izteb_dn7: f64,
        var_izteb_dn8: f64,
        var_izteb_dn9: f64,
        var_vb2e1: f64,
        var_vb2e1_dn5: f64,
        var_vb2e1_dn7: f64,
        var_xisub: f64,
        var_xisub_dn0: f64,
        var_xisub_dn1: f64,
        var_xisub_dn10: f64,
        var_xisub_dn11: f64,
        var_xisub_dn3: f64,
        var_xisub_dn4: f64,
        var_xisub_dn5: f64,
        var_xisub_dn6: f64,
        var_xisub_dn7: f64,
        var_xisub_dn8: f64,
        var_xisub_dn9: f64,
    ) {
        let eq0_e167: f64 = (p.p3 * var_ic1c2);
        let eq0_e167_d_n0: f64 = (p.p3 * var_ic1c2_dn0);
        let eq0_e167_d_n1: f64 = (p.p3 * var_ic1c2_dn1);
        let eq0_e167_d_n3: f64 = (p.p3 * var_ic1c2_dn3);
        let eq0_e167_d_n4: f64 = (p.p3 * var_ic1c2_dn4);
        let eq0_e167_d_n5: f64 = (p.p3 * var_ic1c2_dn5);
        let eq0_e167_d_n6: f64 = (p.p3 * var_ic1c2_dn6);
        let eq0_e167_d_n7: f64 = (p.p3 * var_ic1c2_dn7);
        let eq0_e167_d_n8: f64 = (p.p3 * var_ic1c2_dn8);
        let eq0_e167_d_n9: f64 = (p.p3 * var_ic1c2_dn9);
        let eq0_e167_d_n10: f64 = (p.p3 * var_ic1c2_dn10);
        let eq0_e167_d_n11: f64 = (p.p3 * var_ic1c2_dn11);
        let eq0_e169: f64 = (eq0_e167 * p.p1);
        let eq0_e169_d_n0: f64 = (eq0_e167_d_n0 * p.p1);
        let eq0_e169_d_n1: f64 = (eq0_e167_d_n1 * p.p1);
        let eq0_e169_d_n3: f64 = (eq0_e167_d_n3 * p.p1);
        let eq0_e169_d_n4: f64 = (eq0_e167_d_n4 * p.p1);
        let eq0_e169_d_n5: f64 = (eq0_e167_d_n5 * p.p1);
        let eq0_e169_d_n6: f64 = (eq0_e167_d_n6 * p.p1);
        let eq0_e169_d_n7: f64 = (eq0_e167_d_n7 * p.p1);
        let eq0_e169_d_n8: f64 = (eq0_e167_d_n8 * p.p1);
        let eq0_e169_d_n9: f64 = (eq0_e167_d_n9 * p.p1);
        let eq0_e169_d_n10: f64 = (eq0_e167_d_n10 * p.p1);
        let eq0_e169_d_n11: f64 = (eq0_e167_d_n11 * p.p1);
        let eq0_value: f64 = eq0_e169;
        let eq0_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq0_node_derivatives: [f64; 11] = [eq0_e169_d_n0, eq0_e169_d_n1, eq0_e169_d_n3, eq0_e169_d_n4, eq0_e169_d_n5, eq0_e169_d_n6, eq0_e169_d_n7, eq0_e169_d_n8, eq0_e169_d_n9, eq0_e169_d_n10, eq0_e169_d_n11];
        let eq0_branch_derivative_indices: [usize; 0] = [];
        let eq0_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(8),
            Some(9),
            multiplicity * (eq0_value),
            &eq0_node_derivative_indices,
            &eq0_node_derivatives,
            &eq0_branch_derivative_indices,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let eq1_e172: f64 = (p.p3 * var_in_);
        let eq1_e172_d_n0: f64 = (p.p3 * var_in__dn0);
        let eq1_e172_d_n1: f64 = (p.p3 * var_in__dn1);
        let eq1_e172_d_n3: f64 = (p.p3 * var_in__dn3);
        let eq1_e172_d_n4: f64 = (p.p3 * var_in__dn4);
        let eq1_e172_d_n5: f64 = (p.p3 * var_in__dn5);
        let eq1_e172_d_n6: f64 = (p.p3 * var_in__dn6);
        let eq1_e172_d_n7: f64 = (p.p3 * var_in__dn7);
        let eq1_e172_d_n8: f64 = (p.p3 * var_in__dn8);
        let eq1_e172_d_n9: f64 = (p.p3 * var_in__dn9);
        let eq1_e172_d_n10: f64 = (p.p3 * var_in__dn10);
        let eq1_e172_d_n11: f64 = (p.p3 * var_in__dn11);
        let eq1_e174: f64 = (eq1_e172 * p.p1);
        let eq1_e174_d_n0: f64 = (eq1_e172_d_n0 * p.p1);
        let eq1_e174_d_n1: f64 = (eq1_e172_d_n1 * p.p1);
        let eq1_e174_d_n3: f64 = (eq1_e172_d_n3 * p.p1);
        let eq1_e174_d_n4: f64 = (eq1_e172_d_n4 * p.p1);
        let eq1_e174_d_n5: f64 = (eq1_e172_d_n5 * p.p1);
        let eq1_e174_d_n6: f64 = (eq1_e172_d_n6 * p.p1);
        let eq1_e174_d_n7: f64 = (eq1_e172_d_n7 * p.p1);
        let eq1_e174_d_n8: f64 = (eq1_e172_d_n8 * p.p1);
        let eq1_e174_d_n9: f64 = (eq1_e172_d_n9 * p.p1);
        let eq1_e174_d_n10: f64 = (eq1_e172_d_n10 * p.p1);
        let eq1_e174_d_n11: f64 = (eq1_e172_d_n11 * p.p1);
        let eq1_value: f64 = eq1_e174;
        let eq1_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq1_node_derivatives: [f64; 11] = [eq1_e174_d_n0, eq1_e174_d_n1, eq1_e174_d_n3, eq1_e174_d_n4, eq1_e174_d_n5, eq1_e174_d_n6, eq1_e174_d_n7, eq1_e174_d_n8, eq1_e174_d_n9, eq1_e174_d_n10, eq1_e174_d_n11];
        let eq1_branch_derivative_indices: [usize; 0] = [];
        let eq1_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(5),
            multiplicity * (eq1_value),
            &eq1_node_derivative_indices,
            &eq1_node_derivatives,
            &eq1_branch_derivative_indices,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let eq2_e178: f64 = (var_ib1_s + var_ib2_s);
        let eq2_e178_d_n0: f64 = (var_ib1_s_dn0 + var_ib2_s_dn0);
        let eq2_e178_d_n1: f64 = (var_ib1_s_dn1 + var_ib2_s_dn1);
        let eq2_e178_d_n3: f64 = (var_ib1_s_dn3 + var_ib2_s_dn3);
        let eq2_e178_d_n4: f64 = (var_ib1_s_dn4 + var_ib2_s_dn4);
        let eq2_e178_d_n5: f64 = (var_ib1_s_dn5 + var_ib2_s_dn5);
        let eq2_e178_d_n6: f64 = (var_ib1_s_dn6 + var_ib2_s_dn6);
        let eq2_e178_d_n7: f64 = (var_ib1_s_dn7 + var_ib2_s_dn7);
        let eq2_e178_d_n8: f64 = (var_ib1_s_dn8 + var_ib2_s_dn8);
        let eq2_e178_d_n9: f64 = (var_ib1_s_dn9 + var_ib2_s_dn9);
        let eq2_e178_d_n10: f64 = (var_ib1_s_dn10 + var_ib2_s_dn10);
        let eq2_e178_d_n11: f64 = (var_ib1_s_dn11 + var_ib2_s_dn11);
        let eq2_e180: f64 = (eq2_e178 + var_ibrel);
        let eq2_e180_d_n0: f64 = (eq2_e178_d_n0 + var_ibrel_dn0);
        let eq2_e180_d_n1: f64 = (eq2_e178_d_n1 + var_ibrel_dn1);
        let eq2_e180_d_n3: f64 = (eq2_e178_d_n3 + var_ibrel_dn3);
        let eq2_e180_d_n4: f64 = (eq2_e178_d_n4 + var_ibrel_dn4);
        let eq2_e180_d_n5: f64 = (eq2_e178_d_n5 + var_ibrel_dn5);
        let eq2_e180_d_n6: f64 = (eq2_e178_d_n6 + var_ibrel_dn6);
        let eq2_e180_d_n7: f64 = (eq2_e178_d_n7 + var_ibrel_dn7);
        let eq2_e180_d_n8: f64 = (eq2_e178_d_n8 + var_ibrel_dn8);
        let eq2_e180_d_n9: f64 = (eq2_e178_d_n9 + var_ibrel_dn9);
        let eq2_e180_d_n10: f64 = (eq2_e178_d_n10 + var_ibrel_dn10);
        let eq2_e180_d_n11: f64 = (eq2_e178_d_n11 + var_ibrel_dn11);
        let eq2_e181: f64 = (p.p3 * eq2_e180);
        let eq2_e181_d_n0: f64 = (p.p3 * eq2_e180_d_n0);
        let eq2_e181_d_n1: f64 = (p.p3 * eq2_e180_d_n1);
        let eq2_e181_d_n3: f64 = (p.p3 * eq2_e180_d_n3);
        let eq2_e181_d_n4: f64 = (p.p3 * eq2_e180_d_n4);
        let eq2_e181_d_n5: f64 = (p.p3 * eq2_e180_d_n5);
        let eq2_e181_d_n6: f64 = (p.p3 * eq2_e180_d_n6);
        let eq2_e181_d_n7: f64 = (p.p3 * eq2_e180_d_n7);
        let eq2_e181_d_n8: f64 = (p.p3 * eq2_e180_d_n8);
        let eq2_e181_d_n9: f64 = (p.p3 * eq2_e180_d_n9);
        let eq2_e181_d_n10: f64 = (p.p3 * eq2_e180_d_n10);
        let eq2_e181_d_n11: f64 = (p.p3 * eq2_e180_d_n11);
        let eq2_e183: f64 = (eq2_e181 * p.p1);
        let eq2_e183_d_n0: f64 = (eq2_e181_d_n0 * p.p1);
        let eq2_e183_d_n1: f64 = (eq2_e181_d_n1 * p.p1);
        let eq2_e183_d_n3: f64 = (eq2_e181_d_n3 * p.p1);
        let eq2_e183_d_n4: f64 = (eq2_e181_d_n4 * p.p1);
        let eq2_e183_d_n5: f64 = (eq2_e181_d_n5 * p.p1);
        let eq2_e183_d_n6: f64 = (eq2_e181_d_n6 * p.p1);
        let eq2_e183_d_n7: f64 = (eq2_e181_d_n7 * p.p1);
        let eq2_e183_d_n8: f64 = (eq2_e181_d_n8 * p.p1);
        let eq2_e183_d_n9: f64 = (eq2_e181_d_n9 * p.p1);
        let eq2_e183_d_n10: f64 = (eq2_e181_d_n10 * p.p1);
        let eq2_e183_d_n11: f64 = (eq2_e181_d_n11 * p.p1);
        let eq2_value: f64 = eq2_e183;
        let eq2_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq2_node_derivatives: [f64; 11] = [eq2_e183_d_n0, eq2_e183_d_n1, eq2_e183_d_n3, eq2_e183_d_n4, eq2_e183_d_n5, eq2_e183_d_n6, eq2_e183_d_n7, eq2_e183_d_n8, eq2_e183_d_n9, eq2_e183_d_n10, eq2_e183_d_n11];
        let eq2_branch_derivative_indices: [usize; 0] = [];
        let eq2_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq2_value),
            &eq2_node_derivative_indices,
            &eq2_node_derivatives,
            &eq2_branch_derivative_indices,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let eq3_e187: f64 = (var_ib1 + var_ib2);
        let eq3_e187_d_n0: f64 = (var_ib1_dn0 + var_ib2_dn0);
        let eq3_e187_d_n1: f64 = (var_ib1_dn1 + var_ib2_dn1);
        let eq3_e187_d_n3: f64 = (var_ib1_dn3 + var_ib2_dn3);
        let eq3_e187_d_n4: f64 = (var_ib1_dn4 + var_ib2_dn4);
        let eq3_e187_d_n5: f64 = (var_ib1_dn5 + var_ib2_dn5);
        let eq3_e187_d_n6: f64 = (var_ib1_dn6 + var_ib2_dn6);
        let eq3_e187_d_n7: f64 = (var_ib1_dn7 + var_ib2_dn7);
        let eq3_e187_d_n8: f64 = (var_ib1_dn8 + var_ib2_dn8);
        let eq3_e187_d_n9: f64 = (var_ib1_dn9 + var_ib2_dn9);
        let eq3_e187_d_n10: f64 = (var_ib1_dn10 + var_ib2_dn10);
        let eq3_e187_d_n11: f64 = (var_ib1_dn11 + var_ib2_dn11);
        let eq3_e190: f64 = (var_gmin * var_vb2e1);
        let eq3_e190_d_n5: f64 = (var_gmin * var_vb2e1_dn5);
        let eq3_e190_d_n7: f64 = (var_gmin * var_vb2e1_dn7);
        let eq3_e191: f64 = (eq3_e187 + eq3_e190);
        let eq3_e191_d_n5: f64 = (eq3_e187_d_n5 + eq3_e190_d_n5);
        let eq3_e191_d_n7: f64 = (eq3_e187_d_n7 + eq3_e190_d_n7);
        let eq3_e193: f64 = (eq3_e191 - var_izteb);
        let eq3_e193_d_n0: f64 = (eq3_e187_d_n0 - var_izteb_dn0);
        let eq3_e193_d_n1: f64 = (eq3_e187_d_n1 - var_izteb_dn1);
        let eq3_e193_d_n3: f64 = (eq3_e187_d_n3 - var_izteb_dn3);
        let eq3_e193_d_n4: f64 = (eq3_e187_d_n4 - var_izteb_dn4);
        let eq3_e193_d_n5: f64 = (eq3_e191_d_n5 - var_izteb_dn5);
        let eq3_e193_d_n6: f64 = (eq3_e187_d_n6 - var_izteb_dn6);
        let eq3_e193_d_n7: f64 = (eq3_e191_d_n7 - var_izteb_dn7);
        let eq3_e193_d_n8: f64 = (eq3_e187_d_n8 - var_izteb_dn8);
        let eq3_e193_d_n9: f64 = (eq3_e187_d_n9 - var_izteb_dn9);
        let eq3_e193_d_n10: f64 = (eq3_e187_d_n10 - var_izteb_dn10);
        let eq3_e193_d_n11: f64 = (eq3_e187_d_n11 - var_izteb_dn11);
        let eq3_e195: f64 = (eq3_e193 + var_ibtbt);
        let eq3_e195_d_n0: f64 = (eq3_e193_d_n0 + var_ibtbt_dn0);
        let eq3_e195_d_n1: f64 = (eq3_e193_d_n1 + var_ibtbt_dn1);
        let eq3_e195_d_n3: f64 = (eq3_e193_d_n3 + var_ibtbt_dn3);
        let eq3_e195_d_n4: f64 = (eq3_e193_d_n4 + var_ibtbt_dn4);
        let eq3_e195_d_n5: f64 = (eq3_e193_d_n5 + var_ibtbt_dn5);
        let eq3_e195_d_n6: f64 = (eq3_e193_d_n6 + var_ibtbt_dn6);
        let eq3_e195_d_n7: f64 = (eq3_e193_d_n7 + var_ibtbt_dn7);
        let eq3_e195_d_n8: f64 = (eq3_e193_d_n8 + var_ibtbt_dn8);
        let eq3_e195_d_n9: f64 = (eq3_e193_d_n9 + var_ibtbt_dn9);
        let eq3_e195_d_n10: f64 = (eq3_e193_d_n10 + var_ibtbt_dn10);
        let eq3_e195_d_n11: f64 = (eq3_e193_d_n11 + var_ibtbt_dn11);
        let eq3_e197: f64 = (eq3_e195 + var_itat);
        let eq3_e197_d_n0: f64 = (eq3_e195_d_n0 + var_itat_dn0);
        let eq3_e197_d_n1: f64 = (eq3_e195_d_n1 + var_itat_dn1);
        let eq3_e197_d_n3: f64 = (eq3_e195_d_n3 + var_itat_dn3);
        let eq3_e197_d_n4: f64 = (eq3_e195_d_n4 + var_itat_dn4);
        let eq3_e197_d_n5: f64 = (eq3_e195_d_n5 + var_itat_dn5);
        let eq3_e197_d_n6: f64 = (eq3_e195_d_n6 + var_itat_dn6);
        let eq3_e197_d_n7: f64 = (eq3_e195_d_n7 + var_itat_dn7);
        let eq3_e197_d_n8: f64 = (eq3_e195_d_n8 + var_itat_dn8);
        let eq3_e197_d_n9: f64 = (eq3_e195_d_n9 + var_itat_dn9);
        let eq3_e197_d_n10: f64 = (eq3_e195_d_n10 + var_itat_dn10);
        let eq3_e197_d_n11: f64 = (eq3_e195_d_n11 + var_itat_dn11);
        let eq3_e198: f64 = (p.p3 * eq3_e197);
        let eq3_e198_d_n0: f64 = (p.p3 * eq3_e197_d_n0);
        let eq3_e198_d_n1: f64 = (p.p3 * eq3_e197_d_n1);
        let eq3_e198_d_n3: f64 = (p.p3 * eq3_e197_d_n3);
        let eq3_e198_d_n4: f64 = (p.p3 * eq3_e197_d_n4);
        let eq3_e198_d_n5: f64 = (p.p3 * eq3_e197_d_n5);
        let eq3_e198_d_n6: f64 = (p.p3 * eq3_e197_d_n6);
        let eq3_e198_d_n7: f64 = (p.p3 * eq3_e197_d_n7);
        let eq3_e198_d_n8: f64 = (p.p3 * eq3_e197_d_n8);
        let eq3_e198_d_n9: f64 = (p.p3 * eq3_e197_d_n9);
        let eq3_e198_d_n10: f64 = (p.p3 * eq3_e197_d_n10);
        let eq3_e198_d_n11: f64 = (p.p3 * eq3_e197_d_n11);
        let eq3_e200: f64 = (eq3_e198 * p.p1);
        let eq3_e200_d_n0: f64 = (eq3_e198_d_n0 * p.p1);
        let eq3_e200_d_n1: f64 = (eq3_e198_d_n1 * p.p1);
        let eq3_e200_d_n3: f64 = (eq3_e198_d_n3 * p.p1);
        let eq3_e200_d_n4: f64 = (eq3_e198_d_n4 * p.p1);
        let eq3_e200_d_n5: f64 = (eq3_e198_d_n5 * p.p1);
        let eq3_e200_d_n6: f64 = (eq3_e198_d_n6 * p.p1);
        let eq3_e200_d_n7: f64 = (eq3_e198_d_n7 * p.p1);
        let eq3_e200_d_n8: f64 = (eq3_e198_d_n8 * p.p1);
        let eq3_e200_d_n9: f64 = (eq3_e198_d_n9 * p.p1);
        let eq3_e200_d_n10: f64 = (eq3_e198_d_n10 * p.p1);
        let eq3_e200_d_n11: f64 = (eq3_e198_d_n11 * p.p1);
        let eq3_value: f64 = eq3_e200;
        let eq3_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq3_node_derivatives: [f64; 11] = [eq3_e200_d_n0, eq3_e200_d_n1, eq3_e200_d_n3, eq3_e200_d_n4, eq3_e200_d_n5, eq3_e200_d_n6, eq3_e200_d_n7, eq3_e200_d_n8, eq3_e200_d_n9, eq3_e200_d_n10, eq3_e200_d_n11];
        let eq3_branch_derivative_indices: [usize; 0] = [];
        let eq3_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq3_value),
            &eq3_node_derivative_indices,
            &eq3_node_derivatives,
            &eq3_branch_derivative_indices,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let (eq4_e209, eq4_e209_d_n0, eq4_e209_d_n1, eq4_e209_d_n3, eq4_e209_d_n4, eq4_e209_d_n5, eq4_e209_d_n6, eq4_e209_d_n7, eq4_e209_d_n8, eq4_e209_d_n9, eq4_e209_d_n10, eq4_e209_d_n11,) = {
    if (var_guard125 != 0.0) {
        let eq4_e204: f64 = (-var_iztcb);
        let eq4_e205: f64 = (p.p3 * eq4_e204);
        let eq4_e205_d_n0: f64 = (p.p3 * (-var_iztcb_dn0));
        let eq4_e205_d_n1: f64 = (p.p3 * (-var_iztcb_dn1));
        let eq4_e205_d_n3: f64 = (p.p3 * (-var_iztcb_dn3));
        let eq4_e205_d_n4: f64 = (p.p3 * (-var_iztcb_dn4));
        let eq4_e205_d_n5: f64 = (p.p3 * (-var_iztcb_dn5));
        let eq4_e205_d_n6: f64 = (p.p3 * (-var_iztcb_dn6));
        let eq4_e205_d_n7: f64 = (p.p3 * (-var_iztcb_dn7));
        let eq4_e205_d_n8: f64 = (p.p3 * (-var_iztcb_dn8));
        let eq4_e205_d_n9: f64 = (p.p3 * (-var_iztcb_dn9));
        let eq4_e205_d_n10: f64 = (p.p3 * (-var_iztcb_dn10));
        let eq4_e205_d_n11: f64 = (p.p3 * (-var_iztcb_dn11));
        let eq4_e207: f64 = (eq4_e205 * p.p1);
        let eq4_e207_d_n0: f64 = (eq4_e205_d_n0 * p.p1);
        let eq4_e207_d_n1: f64 = (eq4_e205_d_n1 * p.p1);
        let eq4_e207_d_n3: f64 = (eq4_e205_d_n3 * p.p1);
        let eq4_e207_d_n4: f64 = (eq4_e205_d_n4 * p.p1);
        let eq4_e207_d_n5: f64 = (eq4_e205_d_n5 * p.p1);
        let eq4_e207_d_n6: f64 = (eq4_e205_d_n6 * p.p1);
        let eq4_e207_d_n7: f64 = (eq4_e205_d_n7 * p.p1);
        let eq4_e207_d_n8: f64 = (eq4_e205_d_n8 * p.p1);
        let eq4_e207_d_n9: f64 = (eq4_e205_d_n9 * p.p1);
        let eq4_e207_d_n10: f64 = (eq4_e205_d_n10 * p.p1);
        let eq4_e207_d_n11: f64 = (eq4_e205_d_n11 * p.p1);
        (eq4_e207, eq4_e207_d_n0, eq4_e207_d_n1, eq4_e207_d_n3, eq4_e207_d_n4, eq4_e207_d_n5, eq4_e207_d_n6, eq4_e207_d_n7, eq4_e207_d_n8, eq4_e207_d_n9, eq4_e207_d_n10, eq4_e207_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e209;
        let eq4_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq4_node_derivatives: [f64; 11] = [eq4_e209_d_n0, eq4_e209_d_n1, eq4_e209_d_n3, eq4_e209_d_n4, eq4_e209_d_n5, eq4_e209_d_n6, eq4_e209_d_n7, eq4_e209_d_n8, eq4_e209_d_n9, eq4_e209_d_n10, eq4_e209_d_n11];
        let eq4_branch_derivative_indices: [usize; 0] = [];
        let eq4_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq4_value),
            &eq4_node_derivative_indices,
            &eq4_node_derivatives,
            &eq4_branch_derivative_indices,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e219, eq5_e219_d_n0, eq5_e219_d_n1, eq5_e219_d_n3, eq5_e219_d_n4, eq5_e219_d_n5, eq5_e219_d_n6, eq5_e219_d_n7, eq5_e219_d_n8, eq5_e219_d_n9, eq5_e219_d_n10, eq5_e219_d_n11,) = {
    if (var_guard125 == 0.0) {
        let eq5_e214: f64 = (-var_iztcb);
        let eq5_e215: f64 = (p.p3 * eq5_e214);
        let eq5_e215_d_n0: f64 = (p.p3 * (-var_iztcb_dn0));
        let eq5_e215_d_n1: f64 = (p.p3 * (-var_iztcb_dn1));
        let eq5_e215_d_n3: f64 = (p.p3 * (-var_iztcb_dn3));
        let eq5_e215_d_n4: f64 = (p.p3 * (-var_iztcb_dn4));
        let eq5_e215_d_n5: f64 = (p.p3 * (-var_iztcb_dn5));
        let eq5_e215_d_n6: f64 = (p.p3 * (-var_iztcb_dn6));
        let eq5_e215_d_n7: f64 = (p.p3 * (-var_iztcb_dn7));
        let eq5_e215_d_n8: f64 = (p.p3 * (-var_iztcb_dn8));
        let eq5_e215_d_n9: f64 = (p.p3 * (-var_iztcb_dn9));
        let eq5_e215_d_n10: f64 = (p.p3 * (-var_iztcb_dn10));
        let eq5_e215_d_n11: f64 = (p.p3 * (-var_iztcb_dn11));
        let eq5_e217: f64 = (eq5_e215 * p.p1);
        let eq5_e217_d_n0: f64 = (eq5_e215_d_n0 * p.p1);
        let eq5_e217_d_n1: f64 = (eq5_e215_d_n1 * p.p1);
        let eq5_e217_d_n3: f64 = (eq5_e215_d_n3 * p.p1);
        let eq5_e217_d_n4: f64 = (eq5_e215_d_n4 * p.p1);
        let eq5_e217_d_n5: f64 = (eq5_e215_d_n5 * p.p1);
        let eq5_e217_d_n6: f64 = (eq5_e215_d_n6 * p.p1);
        let eq5_e217_d_n7: f64 = (eq5_e215_d_n7 * p.p1);
        let eq5_e217_d_n8: f64 = (eq5_e215_d_n8 * p.p1);
        let eq5_e217_d_n9: f64 = (eq5_e215_d_n9 * p.p1);
        let eq5_e217_d_n10: f64 = (eq5_e215_d_n10 * p.p1);
        let eq5_e217_d_n11: f64 = (eq5_e215_d_n11 * p.p1);
        (eq5_e217, eq5_e217_d_n0, eq5_e217_d_n1, eq5_e217_d_n3, eq5_e217_d_n4, eq5_e217_d_n5, eq5_e217_d_n6, eq5_e217_d_n7, eq5_e217_d_n8, eq5_e217_d_n9, eq5_e217_d_n10, eq5_e217_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e219;
        let eq5_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq5_node_derivatives: [f64; 11] = [eq5_e219_d_n0, eq5_e219_d_n1, eq5_e219_d_n3, eq5_e219_d_n4, eq5_e219_d_n5, eq5_e219_d_n6, eq5_e219_d_n7, eq5_e219_d_n8, eq5_e219_d_n9, eq5_e219_d_n10, eq5_e219_d_n11];
        let eq5_branch_derivative_indices: [usize; 0] = [];
        let eq5_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq5_value),
            &eq5_node_derivative_indices,
            &eq5_node_derivatives,
            &eq5_branch_derivative_indices,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let eq6_e222: f64 = (p.p3 * var_isub);
        let eq6_e222_d_n3: f64 = (p.p3 * var_isub_dn3);
        let eq6_e222_d_n4: f64 = (p.p3 * var_isub_dn4);
        let eq6_e222_d_n6: f64 = (p.p3 * var_isub_dn6);
        let eq6_e222_d_n7: f64 = (p.p3 * var_isub_dn7);
        let eq6_e222_d_n8: f64 = (p.p3 * var_isub_dn8);
        let eq6_e222_d_n9: f64 = (p.p3 * var_isub_dn9);
        let eq6_e222_d_n11: f64 = (p.p3 * var_isub_dn11);
        let eq6_e224: f64 = (eq6_e222 * p.p1);
        let eq6_e224_d_n3: f64 = (eq6_e222_d_n3 * p.p1);
        let eq6_e224_d_n4: f64 = (eq6_e222_d_n4 * p.p1);
        let eq6_e224_d_n6: f64 = (eq6_e222_d_n6 * p.p1);
        let eq6_e224_d_n7: f64 = (eq6_e222_d_n7 * p.p1);
        let eq6_e224_d_n8: f64 = (eq6_e222_d_n8 * p.p1);
        let eq6_e224_d_n9: f64 = (eq6_e222_d_n9 * p.p1);
        let eq6_e224_d_n11: f64 = (eq6_e222_d_n11 * p.p1);
        let eq6_value: f64 = eq6_e224;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(6),
            Some(3),
            multiplicity * (eq6_value),
            [3, 4, 6, 7, 8, 9, 11],
            [multiplicity * (eq6_e224_d_n3), multiplicity * (eq6_e224_d_n4), multiplicity * (eq6_e224_d_n6), multiplicity * (eq6_e224_d_n7), multiplicity * (eq6_e224_d_n8), multiplicity * (eq6_e224_d_n9), multiplicity * (eq6_e224_d_n11)],
            [],
            [],
            1.0,
        );
        let eq7_e227: f64 = (p.p3 * var_isub_int);
        let eq7_e227_d_n3: f64 = (p.p3 * var_isub_int_dn3);
        let eq7_e227_d_n4: f64 = (p.p3 * var_isub_int_dn4);
        let eq7_e227_d_n7: f64 = (p.p3 * var_isub_int_dn7);
        let eq7_e227_d_n8: f64 = (p.p3 * var_isub_int_dn8);
        let eq7_e227_d_n9: f64 = (p.p3 * var_isub_int_dn9);
        let eq7_e229: f64 = (eq7_e227 * p.p1);
        let eq7_e229_d_n3: f64 = (eq7_e227_d_n3 * p.p1);
        let eq7_e229_d_n4: f64 = (eq7_e227_d_n4 * p.p1);
        let eq7_e229_d_n7: f64 = (eq7_e227_d_n7 * p.p1);
        let eq7_e229_d_n8: f64 = (eq7_e227_d_n8 * p.p1);
        let eq7_e229_d_n9: f64 = (eq7_e227_d_n9 * p.p1);
        let eq7_value: f64 = eq7_e229;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(3),
            multiplicity * (eq7_value),
            [3, 4, 7, 8, 9],
            [multiplicity * (eq7_e229_d_n3), multiplicity * (eq7_e229_d_n4), multiplicity * (eq7_e229_d_n7), multiplicity * (eq7_e229_d_n8), multiplicity * (eq7_e229_d_n9)],
            [],
            [],
            1.0,
        );
        let eq8_e232: f64 = (p.p3 * var_xisub);
        let eq8_e232_d_n0: f64 = (p.p3 * var_xisub_dn0);
        let eq8_e232_d_n1: f64 = (p.p3 * var_xisub_dn1);
        let eq8_e232_d_n3: f64 = (p.p3 * var_xisub_dn3);
        let eq8_e232_d_n4: f64 = (p.p3 * var_xisub_dn4);
        let eq8_e232_d_n5: f64 = (p.p3 * var_xisub_dn5);
        let eq8_e232_d_n6: f64 = (p.p3 * var_xisub_dn6);
        let eq8_e232_d_n7: f64 = (p.p3 * var_xisub_dn7);
        let eq8_e232_d_n8: f64 = (p.p3 * var_xisub_dn8);
        let eq8_e232_d_n9: f64 = (p.p3 * var_xisub_dn9);
        let eq8_e232_d_n10: f64 = (p.p3 * var_xisub_dn10);
        let eq8_e232_d_n11: f64 = (p.p3 * var_xisub_dn11);
        let eq8_e234: f64 = (eq8_e232 * p.p1);
        let eq8_e234_d_n0: f64 = (eq8_e232_d_n0 * p.p1);
        let eq8_e234_d_n1: f64 = (eq8_e232_d_n1 * p.p1);
        let eq8_e234_d_n3: f64 = (eq8_e232_d_n3 * p.p1);
        let eq8_e234_d_n4: f64 = (eq8_e232_d_n4 * p.p1);
        let eq8_e234_d_n5: f64 = (eq8_e232_d_n5 * p.p1);
        let eq8_e234_d_n6: f64 = (eq8_e232_d_n6 * p.p1);
        let eq8_e234_d_n7: f64 = (eq8_e232_d_n7 * p.p1);
        let eq8_e234_d_n8: f64 = (eq8_e232_d_n8 * p.p1);
        let eq8_e234_d_n9: f64 = (eq8_e232_d_n9 * p.p1);
        let eq8_e234_d_n10: f64 = (eq8_e232_d_n10 * p.p1);
        let eq8_e234_d_n11: f64 = (eq8_e232_d_n11 * p.p1);
        let eq8_value: f64 = eq8_e234;
        let eq8_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq8_node_derivatives: [f64; 11] = [eq8_e234_d_n0, eq8_e234_d_n1, eq8_e234_d_n3, eq8_e234_d_n4, eq8_e234_d_n5, eq8_e234_d_n6, eq8_e234_d_n7, eq8_e234_d_n8, eq8_e234_d_n9, eq8_e234_d_n10, eq8_e234_d_n11];
        let eq8_branch_derivative_indices: [usize; 0] = [];
        let eq8_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(1),
            Some(3),
            multiplicity * (eq8_value),
            &eq8_node_derivative_indices,
            &eq8_node_derivatives,
            &eq8_branch_derivative_indices,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let eq9_e237: f64 = (p.p3 * var_isf);
        let eq9_e237_d_n3: f64 = (p.p3 * var_isf_dn3);
        let eq9_e237_d_n4: f64 = (p.p3 * var_isf_dn4);
        let eq9_e237_d_n8: f64 = (p.p3 * var_isf_dn8);
        let eq9_e239: f64 = (eq9_e237 * p.p1);
        let eq9_e239_d_n3: f64 = (eq9_e237_d_n3 * p.p1);
        let eq9_e239_d_n4: f64 = (eq9_e237_d_n4 * p.p1);
        let eq9_e239_d_n8: f64 = (eq9_e237_d_n8 * p.p1);
        let eq9_value: f64 = eq9_e239;
        stamper.stamp_current_node3_local(
            Some(3),
            Some(8),
            multiplicity * (eq9_value),
            3,
            multiplicity * (eq9_e239_d_n3),
            4,
            multiplicity * (eq9_e239_d_n4),
            8,
            multiplicity * (eq9_e239_d_n8),
        );
        let eq10_e242: f64 = (p.p3 * var_ib1b2);
        let eq10_e242_d_n0: f64 = (p.p3 * var_ib1b2_dn0);
        let eq10_e242_d_n1: f64 = (p.p3 * var_ib1b2_dn1);
        let eq10_e242_d_n3: f64 = (p.p3 * var_ib1b2_dn3);
        let eq10_e242_d_n4: f64 = (p.p3 * var_ib1b2_dn4);
        let eq10_e242_d_n5: f64 = (p.p3 * var_ib1b2_dn5);
        let eq10_e242_d_n6: f64 = (p.p3 * var_ib1b2_dn6);
        let eq10_e242_d_n7: f64 = (p.p3 * var_ib1b2_dn7);
        let eq10_e242_d_n8: f64 = (p.p3 * var_ib1b2_dn8);
        let eq10_e242_d_n9: f64 = (p.p3 * var_ib1b2_dn9);
        let eq10_e242_d_n10: f64 = (p.p3 * var_ib1b2_dn10);
        let eq10_e242_d_n11: f64 = (p.p3 * var_ib1b2_dn11);
        let eq10_e244: f64 = (eq10_e242 * p.p1);
        let eq10_e244_d_n0: f64 = (eq10_e242_d_n0 * p.p1);
        let eq10_e244_d_n1: f64 = (eq10_e242_d_n1 * p.p1);
        let eq10_e244_d_n3: f64 = (eq10_e242_d_n3 * p.p1);
        let eq10_e244_d_n4: f64 = (eq10_e242_d_n4 * p.p1);
        let eq10_e244_d_n5: f64 = (eq10_e242_d_n5 * p.p1);
        let eq10_e244_d_n6: f64 = (eq10_e242_d_n6 * p.p1);
        let eq10_e244_d_n7: f64 = (eq10_e242_d_n7 * p.p1);
        let eq10_e244_d_n8: f64 = (eq10_e242_d_n8 * p.p1);
        let eq10_e244_d_n9: f64 = (eq10_e242_d_n9 * p.p1);
        let eq10_e244_d_n10: f64 = (eq10_e242_d_n10 * p.p1);
        let eq10_e244_d_n11: f64 = (eq10_e242_d_n11 * p.p1);
        let eq10_value: f64 = eq10_e244;
        let eq10_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq10_node_derivatives: [f64; 11] = [eq10_e244_d_n0, eq10_e244_d_n1, eq10_e244_d_n3, eq10_e244_d_n4, eq10_e244_d_n5, eq10_e244_d_n6, eq10_e244_d_n7, eq10_e244_d_n8, eq10_e244_d_n9, eq10_e244_d_n10, eq10_e244_d_n11];
        let eq10_branch_derivative_indices: [usize; 0] = [];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq10_value),
            &eq10_node_derivative_indices,
            &eq10_node_derivatives,
            &eq10_branch_derivative_indices,
            &eq10_branch_derivatives,
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
        var_i_cth_dn4: f64,
        var_iavl: f64,
        var_iavl_dn0: f64,
        var_iavl_dn1: f64,
        var_iavl_dn10: f64,
        var_iavl_dn11: f64,
        var_iavl_dn3: f64,
        var_iavl_dn4: f64,
        var_iavl_dn5: f64,
        var_iavl_dn6: f64,
        var_iavl_dn7: f64,
        var_iavl_dn8: f64,
        var_iavl_dn9: f64,
        var_p_rth: f64,
        var_p_rth_dn4: f64,
        var_power: f64,
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
        var_qb1b2_dn0: f64,
        var_qb1b2_dn1: f64,
        var_qb1b2_dn10: f64,
        var_qb1b2_dn11: f64,
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
        var_qbc_dn11: f64,
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
        var_qbe_dn11: f64,
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
        var_qe_dn11: f64,
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
        var_qepi_dn11: f64,
        var_qepi_dn3: f64,
        var_qepi_dn4: f64,
        var_qepi_dn5: f64,
        var_qepi_dn6: f64,
        var_qepi_dn7: f64,
        var_qepi_dn8: f64,
        var_qepi_dn9: f64,
        var_qtc: f64,
        var_qtc_dn0: f64,
        var_qtc_dn1: f64,
        var_qtc_dn10: f64,
        var_qtc_dn11: f64,
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
        var_qte_dn11: f64,
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
        var_qte_s_dn11: f64,
        var_qte_s_dn3: f64,
        var_qte_s_dn4: f64,
        var_qte_s_dn5: f64,
        var_qte_s_dn6: f64,
        var_qte_s_dn7: f64,
        var_qte_s_dn8: f64,
        var_qte_s_dn9: f64,
        var_qts: f64,
        var_qts_dn0: f64,
        var_qts_dn1: f64,
        var_qts_dn10: f64,
        var_qts_dn11: f64,
        var_qts_dn3: f64,
        var_qts_dn4: f64,
        var_qts_dn5: f64,
        var_qts_dn6: f64,
        var_qts_dn7: f64,
        var_qts_dn8: f64,
        var_qts_dn9: f64,
        var_rbc_t: f64,
        var_rbc_t_dn4: f64,
        var_re_t: f64,
        var_re_t_dn4: f64,
        var_vbb1: f64,
        var_vbb1_dn1: f64,
        var_vbb1_dn6: f64,
        var_vbc: f64,
        var_vbc_dn0: f64,
        var_vbc_dn1: f64,
        var_vbe: f64,
        var_vbe_dn1: f64,
        var_vbe_dn2: f64,
        var_vee1: f64,
        var_vee1_dn2: f64,
        var_vee1_dn5: f64,
        var_xiex: f64,
        var_xiex_dn0: f64,
        var_xiex_dn1: f64,
        var_xiex_dn10: f64,
        var_xiex_dn11: f64,
        var_xiex_dn3: f64,
        var_xiex_dn4: f64,
        var_xiex_dn5: f64,
        var_xiex_dn6: f64,
        var_xiex_dn7: f64,
        var_xiex_dn8: f64,
        var_xiex_dn9: f64,
    ) {
        let eq11_e247: f64 = (-1.0);
        let eq11_e249: f64 = (eq11_e247 * var_iavl);
        let eq11_e249_d_n0: f64 = (eq11_e247 * var_iavl_dn0);
        let eq11_e249_d_n1: f64 = (eq11_e247 * var_iavl_dn1);
        let eq11_e249_d_n3: f64 = (eq11_e247 * var_iavl_dn3);
        let eq11_e249_d_n4: f64 = (eq11_e247 * var_iavl_dn4);
        let eq11_e249_d_n5: f64 = (eq11_e247 * var_iavl_dn5);
        let eq11_e249_d_n6: f64 = (eq11_e247 * var_iavl_dn6);
        let eq11_e249_d_n7: f64 = (eq11_e247 * var_iavl_dn7);
        let eq11_e249_d_n8: f64 = (eq11_e247 * var_iavl_dn8);
        let eq11_e249_d_n9: f64 = (eq11_e247 * var_iavl_dn9);
        let eq11_e249_d_n10: f64 = (eq11_e247 * var_iavl_dn10);
        let eq11_e249_d_n11: f64 = (eq11_e247 * var_iavl_dn11);
        let eq11_e250: f64 = (p.p3 * eq11_e249);
        let eq11_e250_d_n0: f64 = (p.p3 * eq11_e249_d_n0);
        let eq11_e250_d_n1: f64 = (p.p3 * eq11_e249_d_n1);
        let eq11_e250_d_n3: f64 = (p.p3 * eq11_e249_d_n3);
        let eq11_e250_d_n4: f64 = (p.p3 * eq11_e249_d_n4);
        let eq11_e250_d_n5: f64 = (p.p3 * eq11_e249_d_n5);
        let eq11_e250_d_n6: f64 = (p.p3 * eq11_e249_d_n6);
        let eq11_e250_d_n7: f64 = (p.p3 * eq11_e249_d_n7);
        let eq11_e250_d_n8: f64 = (p.p3 * eq11_e249_d_n8);
        let eq11_e250_d_n9: f64 = (p.p3 * eq11_e249_d_n9);
        let eq11_e250_d_n10: f64 = (p.p3 * eq11_e249_d_n10);
        let eq11_e250_d_n11: f64 = (p.p3 * eq11_e249_d_n11);
        let eq11_e252: f64 = (eq11_e250 * p.p1);
        let eq11_e252_d_n0: f64 = (eq11_e250_d_n0 * p.p1);
        let eq11_e252_d_n1: f64 = (eq11_e250_d_n1 * p.p1);
        let eq11_e252_d_n3: f64 = (eq11_e250_d_n3 * p.p1);
        let eq11_e252_d_n4: f64 = (eq11_e250_d_n4 * p.p1);
        let eq11_e252_d_n5: f64 = (eq11_e250_d_n5 * p.p1);
        let eq11_e252_d_n6: f64 = (eq11_e250_d_n6 * p.p1);
        let eq11_e252_d_n7: f64 = (eq11_e250_d_n7 * p.p1);
        let eq11_e252_d_n8: f64 = (eq11_e250_d_n8 * p.p1);
        let eq11_e252_d_n9: f64 = (eq11_e250_d_n9 * p.p1);
        let eq11_e252_d_n10: f64 = (eq11_e250_d_n10 * p.p1);
        let eq11_e252_d_n11: f64 = (eq11_e250_d_n11 * p.p1);
        let eq11_value: f64 = eq11_e252;
        let eq11_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq11_node_derivatives: [f64; 11] = [eq11_e252_d_n0, eq11_e252_d_n1, eq11_e252_d_n3, eq11_e252_d_n4, eq11_e252_d_n5, eq11_e252_d_n6, eq11_e252_d_n7, eq11_e252_d_n8, eq11_e252_d_n9, eq11_e252_d_n10, eq11_e252_d_n11];
        let eq11_branch_derivative_indices: [usize; 0] = [];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq11_value),
            &eq11_node_derivative_indices,
            &eq11_node_derivatives,
            &eq11_branch_derivative_indices,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e255: f64 = (p.p3 * var_vee1);
        let eq12_e255_d_n2: f64 = (p.p3 * var_vee1_dn2);
        let eq12_e255_d_n5: f64 = (p.p3 * var_vee1_dn5);
        let __rspice_inv_cse_0: f64 = 1.0 / var_re_t;
        let eq12_e257: f64 = (eq12_e255 * __rspice_inv_cse_0);
        let eq12_e257_d_n2: f64 = (eq12_e255_d_n2 * __rspice_inv_cse_0);
        let eq12_e257_d_n4: f64 = (-((eq12_e255 * var_re_t_dn4) / (var_re_t * var_re_t)));
        let eq12_e257_d_n5: f64 = (eq12_e255_d_n5 / var_re_t);
        let eq12_e259: f64 = (eq12_e257 * p.p1);
        let eq12_e259_d_n2: f64 = (eq12_e257_d_n2 * p.p1);
        let eq12_e259_d_n4: f64 = (eq12_e257_d_n4 * p.p1);
        let eq12_e259_d_n5: f64 = (eq12_e257_d_n5 * p.p1);
        let eq12_value: f64 = eq12_e259;
        stamper.stamp_current_node3_local(
            Some(2),
            Some(5),
            multiplicity * (eq12_value),
            2,
            multiplicity * (eq12_e259_d_n2),
            4,
            multiplicity * (eq12_e259_d_n4),
            5,
            multiplicity * (eq12_e259_d_n5),
        );
        let eq13_e262: f64 = (p.p3 * var_vbb1);
        let eq13_e262_d_n1: f64 = (p.p3 * var_vbb1_dn1);
        let eq13_e262_d_n6: f64 = (p.p3 * var_vbb1_dn6);
        let __rspice_inv_cse_1: f64 = 1.0 / var_rbc_t;
        let eq13_e264: f64 = (eq13_e262 * __rspice_inv_cse_1);
        let eq13_e264_d_n1: f64 = (eq13_e262_d_n1 * __rspice_inv_cse_1);
        let eq13_e264_d_n4: f64 = (-((eq13_e262 * var_rbc_t_dn4) / (var_rbc_t * var_rbc_t)));
        let eq13_e264_d_n6: f64 = (eq13_e262_d_n6 / var_rbc_t);
        let eq13_e266: f64 = (eq13_e264 * p.p1);
        let eq13_e266_d_n1: f64 = (eq13_e264_d_n1 * p.p1);
        let eq13_e266_d_n4: f64 = (eq13_e264_d_n4 * p.p1);
        let eq13_e266_d_n6: f64 = (eq13_e264_d_n6 * p.p1);
        let eq13_value: f64 = eq13_e266;
        stamper.stamp_current_node3_local(
            Some(1),
            Some(6),
            multiplicity * (eq13_value),
            1,
            multiplicity * (eq13_e266_d_n1),
            4,
            multiplicity * (eq13_e266_d_n4),
            6,
            multiplicity * (eq13_e266_d_n6),
        );
        let eq14_value: f64 = var_p_rth;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq14_value),
            4,
            multiplicity * (var_p_rth_dn4),
        );
        let eq15_value: f64 = var_i_cth;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq15_value),
            4,
            multiplicity * (var_i_cth_dn4),
        );
        let eq16_e270: f64 = (-1.0);
        let eq16_e272: f64 = (eq16_e270 * var_power);
        let eq16_e272_d_n0: f64 = (eq16_e270 * var_power_dn0);
        let eq16_e272_d_n1: f64 = (eq16_e270 * var_power_dn1);
        let eq16_e272_d_n2: f64 = (eq16_e270 * var_power_dn2);
        let eq16_e272_d_n3: f64 = (eq16_e270 * var_power_dn3);
        let eq16_e272_d_n4: f64 = (eq16_e270 * var_power_dn4);
        let eq16_e272_d_n5: f64 = (eq16_e270 * var_power_dn5);
        let eq16_e272_d_n6: f64 = (eq16_e270 * var_power_dn6);
        let eq16_e272_d_n7: f64 = (eq16_e270 * var_power_dn7);
        let eq16_e272_d_n8: f64 = (eq16_e270 * var_power_dn8);
        let eq16_e272_d_n9: f64 = (eq16_e270 * var_power_dn9);
        let eq16_e272_d_n10: f64 = (eq16_e270 * var_power_dn10);
        let eq16_e272_d_n11: f64 = (eq16_e270 * var_power_dn11);
        let eq16_e274: f64 = (eq16_e272 * p.p1);
        let eq16_e274_d_n0: f64 = (eq16_e272_d_n0 * p.p1);
        let eq16_e274_d_n1: f64 = (eq16_e272_d_n1 * p.p1);
        let eq16_e274_d_n2: f64 = (eq16_e272_d_n2 * p.p1);
        let eq16_e274_d_n3: f64 = (eq16_e272_d_n3 * p.p1);
        let eq16_e274_d_n4: f64 = (eq16_e272_d_n4 * p.p1);
        let eq16_e274_d_n5: f64 = (eq16_e272_d_n5 * p.p1);
        let eq16_e274_d_n6: f64 = (eq16_e272_d_n6 * p.p1);
        let eq16_e274_d_n7: f64 = (eq16_e272_d_n7 * p.p1);
        let eq16_e274_d_n8: f64 = (eq16_e272_d_n8 * p.p1);
        let eq16_e274_d_n9: f64 = (eq16_e272_d_n9 * p.p1);
        let eq16_e274_d_n10: f64 = (eq16_e272_d_n10 * p.p1);
        let eq16_e274_d_n11: f64 = (eq16_e272_d_n11 * p.p1);
        let eq16_value: f64 = eq16_e274;
        let eq16_node_derivative_indices: [usize; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq16_node_derivatives: [f64; 12] = [eq16_e274_d_n0, eq16_e274_d_n1, eq16_e274_d_n2, eq16_e274_d_n3, eq16_e274_d_n4, eq16_e274_d_n5, eq16_e274_d_n6, eq16_e274_d_n7, eq16_e274_d_n8, eq16_e274_d_n9, eq16_e274_d_n10, eq16_e274_d_n11];
        let eq16_branch_derivative_indices: [usize; 0] = [];
        let eq16_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq16_value),
            &eq16_node_derivative_indices,
            &eq16_node_derivatives,
            &eq16_branch_derivative_indices,
            &eq16_branch_derivatives,
            multiplicity,
        );
        let eq17_e278: f64 = (var_qte + var_qbe);
        let eq17_e278_d_n0: f64 = (var_qte_dn0 + var_qbe_dn0);
        let eq17_e278_d_n1: f64 = (var_qte_dn1 + var_qbe_dn1);
        let eq17_e278_d_n3: f64 = (var_qte_dn3 + var_qbe_dn3);
        let eq17_e278_d_n4: f64 = (var_qte_dn4 + var_qbe_dn4);
        let eq17_e278_d_n5: f64 = (var_qte_dn5 + var_qbe_dn5);
        let eq17_e278_d_n6: f64 = (var_qte_dn6 + var_qbe_dn6);
        let eq17_e278_d_n7: f64 = (var_qte_dn7 + var_qbe_dn7);
        let eq17_e278_d_n8: f64 = (var_qte_dn8 + var_qbe_dn8);
        let eq17_e278_d_n9: f64 = (var_qte_dn9 + var_qbe_dn9);
        let eq17_e278_d_n10: f64 = (var_qte_dn10 + var_qbe_dn10);
        let eq17_e278_d_n11: f64 = (var_qte_dn11 + var_qbe_dn11);
        let eq17_e280: f64 = (eq17_e278 + var_qe);
        let eq17_e280_d_n0: f64 = (eq17_e278_d_n0 + var_qe_dn0);
        let eq17_e280_d_n1: f64 = (eq17_e278_d_n1 + var_qe_dn1);
        let eq17_e280_d_n3: f64 = (eq17_e278_d_n3 + var_qe_dn3);
        let eq17_e280_d_n4: f64 = (eq17_e278_d_n4 + var_qe_dn4);
        let eq17_e280_d_n5: f64 = (eq17_e278_d_n5 + var_qe_dn5);
        let eq17_e280_d_n6: f64 = (eq17_e278_d_n6 + var_qe_dn6);
        let eq17_e280_d_n7: f64 = (eq17_e278_d_n7 + var_qe_dn7);
        let eq17_e280_d_n8: f64 = (eq17_e278_d_n8 + var_qe_dn8);
        let eq17_e280_d_n9: f64 = (eq17_e278_d_n9 + var_qe_dn9);
        let eq17_e280_d_n10: f64 = (eq17_e278_d_n10 + var_qe_dn10);
        let eq17_e280_d_n11: f64 = (eq17_e278_d_n11 + var_qe_dn11);
        let eq17_e281: f64 = (p.p3 * eq17_e280);
        let eq17_e281_d_n0: f64 = (p.p3 * eq17_e280_d_n0);
        let eq17_e281_d_n1: f64 = (p.p3 * eq17_e280_d_n1);
        let eq17_e281_d_n3: f64 = (p.p3 * eq17_e280_d_n3);
        let eq17_e281_d_n4: f64 = (p.p3 * eq17_e280_d_n4);
        let eq17_e281_d_n5: f64 = (p.p3 * eq17_e280_d_n5);
        let eq17_e281_d_n6: f64 = (p.p3 * eq17_e280_d_n6);
        let eq17_e281_d_n7: f64 = (p.p3 * eq17_e280_d_n7);
        let eq17_e281_d_n8: f64 = (p.p3 * eq17_e280_d_n8);
        let eq17_e281_d_n9: f64 = (p.p3 * eq17_e280_d_n9);
        let eq17_e281_d_n10: f64 = (p.p3 * eq17_e280_d_n10);
        let eq17_e281_d_n11: f64 = (p.p3 * eq17_e280_d_n11);
        let eq17_e282: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq17_e281);
        let eq17_e284: f64 = (eq17_e282 * p.p1);
        let eq17_e284_d_n0: f64 = ((eq17_e281_d_n0 * ddt_scale) * p.p1);
        let eq17_e284_d_n1: f64 = ((eq17_e281_d_n1 * ddt_scale) * p.p1);
        let eq17_e284_d_n3: f64 = ((eq17_e281_d_n3 * ddt_scale) * p.p1);
        let eq17_e284_d_n4: f64 = ((eq17_e281_d_n4 * ddt_scale) * p.p1);
        let eq17_e284_d_n5: f64 = ((eq17_e281_d_n5 * ddt_scale) * p.p1);
        let eq17_e284_d_n6: f64 = ((eq17_e281_d_n6 * ddt_scale) * p.p1);
        let eq17_e284_d_n7: f64 = ((eq17_e281_d_n7 * ddt_scale) * p.p1);
        let eq17_e284_d_n8: f64 = ((eq17_e281_d_n8 * ddt_scale) * p.p1);
        let eq17_e284_d_n9: f64 = ((eq17_e281_d_n9 * ddt_scale) * p.p1);
        let eq17_e284_d_n10: f64 = ((eq17_e281_d_n10 * ddt_scale) * p.p1);
        let eq17_e284_d_n11: f64 = ((eq17_e281_d_n11 * ddt_scale) * p.p1);
        let eq17_value: f64 = eq17_e284;
        let eq17_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq17_node_derivatives: [f64; 11] = [eq17_e284_d_n0, eq17_e284_d_n1, eq17_e284_d_n3, eq17_e284_d_n4, eq17_e284_d_n5, eq17_e284_d_n6, eq17_e284_d_n7, eq17_e284_d_n8, eq17_e284_d_n9, eq17_e284_d_n10, eq17_e284_d_n11];
        let eq17_branch_derivative_indices: [usize; 0] = [];
        let eq17_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq17_value),
            &eq17_node_derivative_indices,
            &eq17_node_derivatives,
            &eq17_branch_derivative_indices,
            &eq17_branch_derivatives,
            multiplicity,
        );
        let eq18_e287: f64 = (p.p3 * var_qte_s);
        let eq18_e287_d_n0: f64 = (p.p3 * var_qte_s_dn0);
        let eq18_e287_d_n1: f64 = (p.p3 * var_qte_s_dn1);
        let eq18_e287_d_n3: f64 = (p.p3 * var_qte_s_dn3);
        let eq18_e287_d_n4: f64 = (p.p3 * var_qte_s_dn4);
        let eq18_e287_d_n5: f64 = (p.p3 * var_qte_s_dn5);
        let eq18_e287_d_n6: f64 = (p.p3 * var_qte_s_dn6);
        let eq18_e287_d_n7: f64 = (p.p3 * var_qte_s_dn7);
        let eq18_e287_d_n8: f64 = (p.p3 * var_qte_s_dn8);
        let eq18_e287_d_n9: f64 = (p.p3 * var_qte_s_dn9);
        let eq18_e287_d_n10: f64 = (p.p3 * var_qte_s_dn10);
        let eq18_e287_d_n11: f64 = (p.p3 * var_qte_s_dn11);
        let eq18_e288: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq18_e287);
        let eq18_e290: f64 = (eq18_e288 * p.p1);
        let eq18_e290_d_n0: f64 = ((eq18_e287_d_n0 * ddt_scale) * p.p1);
        let eq18_e290_d_n1: f64 = ((eq18_e287_d_n1 * ddt_scale) * p.p1);
        let eq18_e290_d_n3: f64 = ((eq18_e287_d_n3 * ddt_scale) * p.p1);
        let eq18_e290_d_n4: f64 = ((eq18_e287_d_n4 * ddt_scale) * p.p1);
        let eq18_e290_d_n5: f64 = ((eq18_e287_d_n5 * ddt_scale) * p.p1);
        let eq18_e290_d_n6: f64 = ((eq18_e287_d_n6 * ddt_scale) * p.p1);
        let eq18_e290_d_n7: f64 = ((eq18_e287_d_n7 * ddt_scale) * p.p1);
        let eq18_e290_d_n8: f64 = ((eq18_e287_d_n8 * ddt_scale) * p.p1);
        let eq18_e290_d_n9: f64 = ((eq18_e287_d_n9 * ddt_scale) * p.p1);
        let eq18_e290_d_n10: f64 = ((eq18_e287_d_n10 * ddt_scale) * p.p1);
        let eq18_e290_d_n11: f64 = ((eq18_e287_d_n11 * ddt_scale) * p.p1);
        let eq18_value: f64 = eq18_e290;
        let eq18_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq18_node_derivatives: [f64; 11] = [eq18_e290_d_n0, eq18_e290_d_n1, eq18_e290_d_n3, eq18_e290_d_n4, eq18_e290_d_n5, eq18_e290_d_n6, eq18_e290_d_n7, eq18_e290_d_n8, eq18_e290_d_n9, eq18_e290_d_n10, eq18_e290_d_n11];
        let eq18_branch_derivative_indices: [usize; 0] = [];
        let eq18_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq18_value),
            &eq18_node_derivative_indices,
            &eq18_node_derivatives,
            &eq18_branch_derivative_indices,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let eq19_e294: f64 = (var_qtc + var_qbc);
        let eq19_e294_d_n0: f64 = (var_qtc_dn0 + var_qbc_dn0);
        let eq19_e294_d_n1: f64 = (var_qtc_dn1 + var_qbc_dn1);
        let eq19_e294_d_n3: f64 = (var_qtc_dn3 + var_qbc_dn3);
        let eq19_e294_d_n4: f64 = (var_qtc_dn4 + var_qbc_dn4);
        let eq19_e294_d_n5: f64 = (var_qtc_dn5 + var_qbc_dn5);
        let eq19_e294_d_n6: f64 = (var_qtc_dn6 + var_qbc_dn6);
        let eq19_e294_d_n7: f64 = (var_qtc_dn7 + var_qbc_dn7);
        let eq19_e294_d_n8: f64 = (var_qtc_dn8 + var_qbc_dn8);
        let eq19_e294_d_n9: f64 = (var_qtc_dn9 + var_qbc_dn9);
        let eq19_e294_d_n10: f64 = (var_qtc_dn10 + var_qbc_dn10);
        let eq19_e294_d_n11: f64 = (var_qtc_dn11 + var_qbc_dn11);
        let eq19_e296: f64 = (eq19_e294 + var_qepi);
        let eq19_e296_d_n0: f64 = (eq19_e294_d_n0 + var_qepi_dn0);
        let eq19_e296_d_n1: f64 = (eq19_e294_d_n1 + var_qepi_dn1);
        let eq19_e296_d_n3: f64 = (eq19_e294_d_n3 + var_qepi_dn3);
        let eq19_e296_d_n4: f64 = (eq19_e294_d_n4 + var_qepi_dn4);
        let eq19_e296_d_n5: f64 = (eq19_e294_d_n5 + var_qepi_dn5);
        let eq19_e296_d_n6: f64 = (eq19_e294_d_n6 + var_qepi_dn6);
        let eq19_e296_d_n7: f64 = (eq19_e294_d_n7 + var_qepi_dn7);
        let eq19_e296_d_n8: f64 = (eq19_e294_d_n8 + var_qepi_dn8);
        let eq19_e296_d_n9: f64 = (eq19_e294_d_n9 + var_qepi_dn9);
        let eq19_e296_d_n10: f64 = (eq19_e294_d_n10 + var_qepi_dn10);
        let eq19_e296_d_n11: f64 = (eq19_e294_d_n11 + var_qepi_dn11);
        let eq19_e297: f64 = (p.p3 * eq19_e296);
        let eq19_e297_d_n0: f64 = (p.p3 * eq19_e296_d_n0);
        let eq19_e297_d_n1: f64 = (p.p3 * eq19_e296_d_n1);
        let eq19_e297_d_n3: f64 = (p.p3 * eq19_e296_d_n3);
        let eq19_e297_d_n4: f64 = (p.p3 * eq19_e296_d_n4);
        let eq19_e297_d_n5: f64 = (p.p3 * eq19_e296_d_n5);
        let eq19_e297_d_n6: f64 = (p.p3 * eq19_e296_d_n6);
        let eq19_e297_d_n7: f64 = (p.p3 * eq19_e296_d_n7);
        let eq19_e297_d_n8: f64 = (p.p3 * eq19_e296_d_n8);
        let eq19_e297_d_n9: f64 = (p.p3 * eq19_e296_d_n9);
        let eq19_e297_d_n10: f64 = (p.p3 * eq19_e296_d_n10);
        let eq19_e297_d_n11: f64 = (p.p3 * eq19_e296_d_n11);
        let eq19_e298: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq19_e297);
        let eq19_e300: f64 = (eq19_e298 * p.p1);
        let eq19_e300_d_n0: f64 = ((eq19_e297_d_n0 * ddt_scale) * p.p1);
        let eq19_e300_d_n1: f64 = ((eq19_e297_d_n1 * ddt_scale) * p.p1);
        let eq19_e300_d_n3: f64 = ((eq19_e297_d_n3 * ddt_scale) * p.p1);
        let eq19_e300_d_n4: f64 = ((eq19_e297_d_n4 * ddt_scale) * p.p1);
        let eq19_e300_d_n5: f64 = ((eq19_e297_d_n5 * ddt_scale) * p.p1);
        let eq19_e300_d_n6: f64 = ((eq19_e297_d_n6 * ddt_scale) * p.p1);
        let eq19_e300_d_n7: f64 = ((eq19_e297_d_n7 * ddt_scale) * p.p1);
        let eq19_e300_d_n8: f64 = ((eq19_e297_d_n8 * ddt_scale) * p.p1);
        let eq19_e300_d_n9: f64 = ((eq19_e297_d_n9 * ddt_scale) * p.p1);
        let eq19_e300_d_n10: f64 = ((eq19_e297_d_n10 * ddt_scale) * p.p1);
        let eq19_e300_d_n11: f64 = ((eq19_e297_d_n11 * ddt_scale) * p.p1);
        let eq19_value: f64 = eq19_e300;
        let eq19_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq19_node_derivatives: [f64; 11] = [eq19_e300_d_n0, eq19_e300_d_n1, eq19_e300_d_n3, eq19_e300_d_n4, eq19_e300_d_n5, eq19_e300_d_n6, eq19_e300_d_n7, eq19_e300_d_n8, eq19_e300_d_n9, eq19_e300_d_n10, eq19_e300_d_n11];
        let eq19_branch_derivative_indices: [usize; 0] = [];
        let eq19_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq19_value),
            &eq19_node_derivative_indices,
            &eq19_node_derivatives,
            &eq19_branch_derivative_indices,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let eq20_e303: f64 = (p.p3 * var_qts);
        let eq20_e303_d_n0: f64 = (p.p3 * var_qts_dn0);
        let eq20_e303_d_n1: f64 = (p.p3 * var_qts_dn1);
        let eq20_e303_d_n3: f64 = (p.p3 * var_qts_dn3);
        let eq20_e303_d_n4: f64 = (p.p3 * var_qts_dn4);
        let eq20_e303_d_n5: f64 = (p.p3 * var_qts_dn5);
        let eq20_e303_d_n6: f64 = (p.p3 * var_qts_dn6);
        let eq20_e303_d_n7: f64 = (p.p3 * var_qts_dn7);
        let eq20_e303_d_n8: f64 = (p.p3 * var_qts_dn8);
        let eq20_e303_d_n9: f64 = (p.p3 * var_qts_dn9);
        let eq20_e303_d_n10: f64 = (p.p3 * var_qts_dn10);
        let eq20_e303_d_n11: f64 = (p.p3 * var_qts_dn11);
        let eq20_e304: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq20_e303);
        let eq20_e306: f64 = (eq20_e304 * p.p1);
        let eq20_e306_d_n0: f64 = ((eq20_e303_d_n0 * ddt_scale) * p.p1);
        let eq20_e306_d_n1: f64 = ((eq20_e303_d_n1 * ddt_scale) * p.p1);
        let eq20_e306_d_n3: f64 = ((eq20_e303_d_n3 * ddt_scale) * p.p1);
        let eq20_e306_d_n4: f64 = ((eq20_e303_d_n4 * ddt_scale) * p.p1);
        let eq20_e306_d_n5: f64 = ((eq20_e303_d_n5 * ddt_scale) * p.p1);
        let eq20_e306_d_n6: f64 = ((eq20_e303_d_n6 * ddt_scale) * p.p1);
        let eq20_e306_d_n7: f64 = ((eq20_e303_d_n7 * ddt_scale) * p.p1);
        let eq20_e306_d_n8: f64 = ((eq20_e303_d_n8 * ddt_scale) * p.p1);
        let eq20_e306_d_n9: f64 = ((eq20_e303_d_n9 * ddt_scale) * p.p1);
        let eq20_e306_d_n10: f64 = ((eq20_e303_d_n10 * ddt_scale) * p.p1);
        let eq20_e306_d_n11: f64 = ((eq20_e303_d_n11 * ddt_scale) * p.p1);
        let eq20_value: f64 = eq20_e306;
        let eq20_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq20_node_derivatives: [f64; 11] = [eq20_e306_d_n0, eq20_e306_d_n1, eq20_e306_d_n3, eq20_e306_d_n4, eq20_e306_d_n5, eq20_e306_d_n6, eq20_e306_d_n7, eq20_e306_d_n8, eq20_e306_d_n9, eq20_e306_d_n10, eq20_e306_d_n11];
        let eq20_branch_derivative_indices: [usize; 0] = [];
        let eq20_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            Some(8),
            multiplicity * (eq20_value),
            &eq20_node_derivative_indices,
            &eq20_node_derivatives,
            &eq20_branch_derivative_indices,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let eq21_e309: f64 = (p.p3 * var_qb1b2);
        let eq21_e309_d_n0: f64 = (p.p3 * var_qb1b2_dn0);
        let eq21_e309_d_n1: f64 = (p.p3 * var_qb1b2_dn1);
        let eq21_e309_d_n3: f64 = (p.p3 * var_qb1b2_dn3);
        let eq21_e309_d_n4: f64 = (p.p3 * var_qb1b2_dn4);
        let eq21_e309_d_n5: f64 = (p.p3 * var_qb1b2_dn5);
        let eq21_e309_d_n6: f64 = (p.p3 * var_qb1b2_dn6);
        let eq21_e309_d_n7: f64 = (p.p3 * var_qb1b2_dn7);
        let eq21_e309_d_n8: f64 = (p.p3 * var_qb1b2_dn8);
        let eq21_e309_d_n9: f64 = (p.p3 * var_qb1b2_dn9);
        let eq21_e309_d_n10: f64 = (p.p3 * var_qb1b2_dn10);
        let eq21_e309_d_n11: f64 = (p.p3 * var_qb1b2_dn11);
        let eq21_e310: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq21_e309);
        let eq21_e312: f64 = (eq21_e310 * p.p1);
        let eq21_e312_d_n0: f64 = ((eq21_e309_d_n0 * ddt_scale) * p.p1);
        let eq21_e312_d_n1: f64 = ((eq21_e309_d_n1 * ddt_scale) * p.p1);
        let eq21_e312_d_n3: f64 = ((eq21_e309_d_n3 * ddt_scale) * p.p1);
        let eq21_e312_d_n4: f64 = ((eq21_e309_d_n4 * ddt_scale) * p.p1);
        let eq21_e312_d_n5: f64 = ((eq21_e309_d_n5 * ddt_scale) * p.p1);
        let eq21_e312_d_n6: f64 = ((eq21_e309_d_n6 * ddt_scale) * p.p1);
        let eq21_e312_d_n7: f64 = ((eq21_e309_d_n7 * ddt_scale) * p.p1);
        let eq21_e312_d_n8: f64 = ((eq21_e309_d_n8 * ddt_scale) * p.p1);
        let eq21_e312_d_n9: f64 = ((eq21_e309_d_n9 * ddt_scale) * p.p1);
        let eq21_e312_d_n10: f64 = ((eq21_e309_d_n10 * ddt_scale) * p.p1);
        let eq21_e312_d_n11: f64 = ((eq21_e309_d_n11 * ddt_scale) * p.p1);
        let eq21_value: f64 = eq21_e312;
        let eq21_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq21_node_derivatives: [f64; 11] = [eq21_e312_d_n0, eq21_e312_d_n1, eq21_e312_d_n3, eq21_e312_d_n4, eq21_e312_d_n5, eq21_e312_d_n6, eq21_e312_d_n7, eq21_e312_d_n8, eq21_e312_d_n9, eq21_e312_d_n10, eq21_e312_d_n11];
        let eq21_branch_derivative_indices: [usize; 0] = [];
        let eq21_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq21_value),
            &eq21_node_derivative_indices,
            &eq21_node_derivatives,
            &eq21_branch_derivative_indices,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let eq22_e315: f64 = (p.p3 * p.p69);
        let eq22_e317: f64 = (eq22_e315 * var_vbe);
        let eq22_e317_d_n1: f64 = (eq22_e315 * var_vbe_dn1);
        let eq22_e317_d_n2: f64 = (eq22_e315 * var_vbe_dn2);
        let eq22_e318: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq22_e317);
        let eq22_e320: f64 = (eq22_e318 * p.p1);
        let eq22_e320_d_n1: f64 = ((eq22_e317_d_n1 * ddt_scale) * p.p1);
        let eq22_e320_d_n2: f64 = ((eq22_e317_d_n2 * ddt_scale) * p.p1);
        let eq22_value: f64 = eq22_e320;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (eq22_value),
            1,
            multiplicity * (eq22_e320_d_n1),
            2,
            multiplicity * (eq22_e320_d_n2),
        );
        let eq23_e323: f64 = (p.p3 * p.p78);
        let eq23_e325: f64 = (eq23_e323 * var_vbc);
        let eq23_e325_d_n0: f64 = (eq23_e323 * var_vbc_dn0);
        let eq23_e325_d_n1: f64 = (eq23_e323 * var_vbc_dn1);
        let eq23_e326: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq23_e325);
        let eq23_e328: f64 = (eq23_e326 * p.p1);
        let eq23_e328_d_n0: f64 = ((eq23_e325_d_n0 * ddt_scale) * p.p1);
        let eq23_e328_d_n1: f64 = ((eq23_e325_d_n1 * ddt_scale) * p.p1);
        let eq23_value: f64 = eq23_e328;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * (eq23_value),
            0,
            multiplicity * (eq23_e328_d_n0),
            1,
            multiplicity * (eq23_e328_d_n1),
        );
        let eq24_e331: f64 = (p.p3 * var_xiex);
        let eq24_e331_d_n0: f64 = (p.p3 * var_xiex_dn0);
        let eq24_e331_d_n1: f64 = (p.p3 * var_xiex_dn1);
        let eq24_e331_d_n3: f64 = (p.p3 * var_xiex_dn3);
        let eq24_e331_d_n4: f64 = (p.p3 * var_xiex_dn4);
        let eq24_e331_d_n5: f64 = (p.p3 * var_xiex_dn5);
        let eq24_e331_d_n6: f64 = (p.p3 * var_xiex_dn6);
        let eq24_e331_d_n7: f64 = (p.p3 * var_xiex_dn7);
        let eq24_e331_d_n8: f64 = (p.p3 * var_xiex_dn8);
        let eq24_e331_d_n9: f64 = (p.p3 * var_xiex_dn9);
        let eq24_e331_d_n10: f64 = (p.p3 * var_xiex_dn10);
        let eq24_e331_d_n11: f64 = (p.p3 * var_xiex_dn11);
        let eq24_e333: f64 = (eq24_e331 * p.p1);
        let eq24_e333_d_n0: f64 = (eq24_e331_d_n0 * p.p1);
        let eq24_e333_d_n1: f64 = (eq24_e331_d_n1 * p.p1);
        let eq24_e333_d_n3: f64 = (eq24_e331_d_n3 * p.p1);
        let eq24_e333_d_n4: f64 = (eq24_e331_d_n4 * p.p1);
        let eq24_e333_d_n5: f64 = (eq24_e331_d_n5 * p.p1);
        let eq24_e333_d_n6: f64 = (eq24_e331_d_n6 * p.p1);
        let eq24_e333_d_n7: f64 = (eq24_e331_d_n7 * p.p1);
        let eq24_e333_d_n8: f64 = (eq24_e331_d_n8 * p.p1);
        let eq24_e333_d_n9: f64 = (eq24_e331_d_n9 * p.p1);
        let eq24_e333_d_n10: f64 = (eq24_e331_d_n10 * p.p1);
        let eq24_e333_d_n11: f64 = (eq24_e331_d_n11 * p.p1);
        let eq24_value: f64 = eq24_e333;
        let eq24_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq24_node_derivatives: [f64; 11] = [eq24_e333_d_n0, eq24_e333_d_n1, eq24_e333_d_n3, eq24_e333_d_n4, eq24_e333_d_n5, eq24_e333_d_n6, eq24_e333_d_n7, eq24_e333_d_n8, eq24_e333_d_n9, eq24_e333_d_n10, eq24_e333_d_n11];
        let eq24_branch_derivative_indices: [usize; 0] = [];
        let eq24_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(1),
            Some(10),
            multiplicity * (eq24_value),
            &eq24_node_derivative_indices,
            &eq24_node_derivatives,
            &eq24_branch_derivative_indices,
            &eq24_branch_derivatives,
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
        var_gcc_ex_t_dn4: f64,
        var_gcc_in_t: f64,
        var_gcc_in_t_dn4: f64,
        var_gcc_xx_t: f64,
        var_gcc_xx_t_dn4: f64,
        var_gem_n: f64,
        var_gem_n_dn0: f64,
        var_gem_n_dn1: f64,
        var_gem_n_dn10: f64,
        var_gem_n_dn11: f64,
        var_gem_n_dn3: f64,
        var_gem_n_dn4: f64,
        var_gem_n_dn5: f64,
        var_gem_n_dn6: f64,
        var_gem_n_dn7: f64,
        var_gem_n_dn8: f64,
        var_gem_n_dn9: f64,
        var_gmin: f64,
        var_guard129: f64,
        var_guard130: f64,
        var_ib3: f64,
        var_ib3_dn0: f64,
        var_ib3_dn1: f64,
        var_ib3_dn10: f64,
        var_ib3_dn11: f64,
        var_ib3_dn3: f64,
        var_ib3_dn4: f64,
        var_ib3_dn5: f64,
        var_ib3_dn6: f64,
        var_ib3_dn7: f64,
        var_ib3_dn8: f64,
        var_ib3_dn9: f64,
        var_iex: f64,
        var_iex_dn0: f64,
        var_iex_dn1: f64,
        var_iex_dn10: f64,
        var_iex_dn11: f64,
        var_iex_dn3: f64,
        var_iex_dn4: f64,
        var_iex_dn5: f64,
        var_iex_dn6: f64,
        var_iex_dn7: f64,
        var_iex_dn8: f64,
        var_iex_dn9: f64,
        var_qex: f64,
        var_qex_dn0: f64,
        var_qex_dn1: f64,
        var_qex_dn10: f64,
        var_qex_dn11: f64,
        var_qex_dn3: f64,
        var_qex_dn4: f64,
        var_qex_dn5: f64,
        var_qex_dn6: f64,
        var_qex_dn7: f64,
        var_qex_dn8: f64,
        var_qex_dn9: f64,
        var_qtex: f64,
        var_qtex_dn0: f64,
        var_qtex_dn1: f64,
        var_qtex_dn10: f64,
        var_qtex_dn11: f64,
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
        var_taun_dn10: f64,
        var_taun_dn11: f64,
        var_taun_dn3: f64,
        var_taun_dn4: f64,
        var_taun_dn5: f64,
        var_taun_dn6: f64,
        var_taun_dn7: f64,
        var_taun_dn8: f64,
        var_taun_dn9: f64,
        var_vb1c4: f64,
        var_vb1c4_dn11: f64,
        var_vb1c4_dn6: f64,
        var_vb1c4_dn7: f64,
        var_vb1c4_dn8: f64,
        var_vb1c4_dn9: f64,
        var_vc3c4: f64,
        var_vc3c4_dn10: f64,
        var_vc3c4_dn11: f64,
        var_vc4c1: f64,
        var_vc4c1_dn11: f64,
        var_vc4c1_dn8: f64,
        var_vcc3: f64,
        var_vcc3_dn0: f64,
        var_vcc3_dn1: f64,
        var_vcc3_dn10: f64,
        var_vcc3_dn11: f64,
        var_vcc3_dn6: f64,
        var_vcc3_dn7: f64,
        var_vcc3_dn8: f64,
        var_vcc3_dn9: f64,
        var_xqex: f64,
        var_xqex_dn0: f64,
        var_xqex_dn1: f64,
        var_xqex_dn10: f64,
        var_xqex_dn11: f64,
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
        var_xqtex_dn11: f64,
        var_xqtex_dn3: f64,
        var_xqtex_dn4: f64,
        var_xqtex_dn5: f64,
        var_xqtex_dn6: f64,
        var_xqtex_dn7: f64,
        var_xqtex_dn8: f64,
        var_xqtex_dn9: f64,
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);
        let eq25_e336: f64 = (p.p3 * var_vcc3);
        let eq25_e336_d_n0: f64 = (p.p3 * var_vcc3_dn0);
        let eq25_e336_d_n1: f64 = (p.p3 * var_vcc3_dn1);
        let eq25_e336_d_n6: f64 = (p.p3 * var_vcc3_dn6);
        let eq25_e336_d_n7: f64 = (p.p3 * var_vcc3_dn7);
        let eq25_e336_d_n8: f64 = (p.p3 * var_vcc3_dn8);
        let eq25_e336_d_n9: f64 = (p.p3 * var_vcc3_dn9);
        let eq25_e336_d_n10: f64 = (p.p3 * var_vcc3_dn10);
        let eq25_e336_d_n11: f64 = (p.p3 * var_vcc3_dn11);
        let eq25_e338: f64 = (eq25_e336 * var_gcc_xx_t);
        let eq25_e338_d_n0: f64 = (eq25_e336_d_n0 * var_gcc_xx_t);
        let eq25_e338_d_n1: f64 = (eq25_e336_d_n1 * var_gcc_xx_t);
        let eq25_e338_d_n4: f64 = (eq25_e336 * var_gcc_xx_t_dn4);
        let eq25_e338_d_n6: f64 = (eq25_e336_d_n6 * var_gcc_xx_t);
        let eq25_e338_d_n7: f64 = (eq25_e336_d_n7 * var_gcc_xx_t);
        let eq25_e338_d_n8: f64 = (eq25_e336_d_n8 * var_gcc_xx_t);
        let eq25_e338_d_n9: f64 = (eq25_e336_d_n9 * var_gcc_xx_t);
        let eq25_e338_d_n10: f64 = (eq25_e336_d_n10 * var_gcc_xx_t);
        let eq25_e338_d_n11: f64 = (eq25_e336_d_n11 * var_gcc_xx_t);
        let eq25_e340: f64 = (eq25_e338 * p.p1);
        let eq25_e340_d_n0: f64 = (eq25_e338_d_n0 * p.p1);
        let eq25_e340_d_n1: f64 = (eq25_e338_d_n1 * p.p1);
        let eq25_e340_d_n4: f64 = (eq25_e338_d_n4 * p.p1);
        let eq25_e340_d_n6: f64 = (eq25_e338_d_n6 * p.p1);
        let eq25_e340_d_n7: f64 = (eq25_e338_d_n7 * p.p1);
        let eq25_e340_d_n8: f64 = (eq25_e338_d_n8 * p.p1);
        let eq25_e340_d_n9: f64 = (eq25_e338_d_n9 * p.p1);
        let eq25_e340_d_n10: f64 = (eq25_e338_d_n10 * p.p1);
        let eq25_e340_d_n11: f64 = (eq25_e338_d_n11 * p.p1);
        let eq25_value: f64 = eq25_e340;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(0),
            Some(10),
            multiplicity * (eq25_value),
            [0, 1, 4, 6, 7, 8, 9, 10, 11],
            [multiplicity * (eq25_e340_d_n0), multiplicity * (eq25_e340_d_n1), multiplicity * (eq25_e340_d_n4), multiplicity * (eq25_e340_d_n6), multiplicity * (eq25_e340_d_n7), multiplicity * (eq25_e340_d_n8), multiplicity * (eq25_e340_d_n9), multiplicity * (eq25_e340_d_n10), multiplicity * (eq25_e340_d_n11)],
            [],
            [],
            1.0,
        );
        let eq26_e344: f64 = (var_xqtex + var_xqex);
        let eq26_e344_d_n0: f64 = (var_xqtex_dn0 + var_xqex_dn0);
        let eq26_e344_d_n1: f64 = (var_xqtex_dn1 + var_xqex_dn1);
        let eq26_e344_d_n3: f64 = (var_xqtex_dn3 + var_xqex_dn3);
        let eq26_e344_d_n4: f64 = (var_xqtex_dn4 + var_xqex_dn4);
        let eq26_e344_d_n5: f64 = (var_xqtex_dn5 + var_xqex_dn5);
        let eq26_e344_d_n6: f64 = (var_xqtex_dn6 + var_xqex_dn6);
        let eq26_e344_d_n7: f64 = (var_xqtex_dn7 + var_xqex_dn7);
        let eq26_e344_d_n8: f64 = (var_xqtex_dn8 + var_xqex_dn8);
        let eq26_e344_d_n9: f64 = (var_xqtex_dn9 + var_xqex_dn9);
        let eq26_e344_d_n10: f64 = (var_xqtex_dn10 + var_xqex_dn10);
        let eq26_e344_d_n11: f64 = (var_xqtex_dn11 + var_xqex_dn11);
        let eq26_e345: f64 = (p.p3 * eq26_e344);
        let eq26_e345_d_n0: f64 = (p.p3 * eq26_e344_d_n0);
        let eq26_e345_d_n1: f64 = (p.p3 * eq26_e344_d_n1);
        let eq26_e345_d_n3: f64 = (p.p3 * eq26_e344_d_n3);
        let eq26_e345_d_n4: f64 = (p.p3 * eq26_e344_d_n4);
        let eq26_e345_d_n5: f64 = (p.p3 * eq26_e344_d_n5);
        let eq26_e345_d_n6: f64 = (p.p3 * eq26_e344_d_n6);
        let eq26_e345_d_n7: f64 = (p.p3 * eq26_e344_d_n7);
        let eq26_e345_d_n8: f64 = (p.p3 * eq26_e344_d_n8);
        let eq26_e345_d_n9: f64 = (p.p3 * eq26_e344_d_n9);
        let eq26_e345_d_n10: f64 = (p.p3 * eq26_e344_d_n10);
        let eq26_e345_d_n11: f64 = (p.p3 * eq26_e344_d_n11);
        let eq26_e346: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq26_e345);
        let eq26_e348: f64 = (eq26_e346 * p.p1);
        let eq26_e348_d_n0: f64 = ((eq26_e345_d_n0 * ddt_scale) * p.p1);
        let eq26_e348_d_n1: f64 = ((eq26_e345_d_n1 * ddt_scale) * p.p1);
        let eq26_e348_d_n3: f64 = ((eq26_e345_d_n3 * ddt_scale) * p.p1);
        let eq26_e348_d_n4: f64 = ((eq26_e345_d_n4 * ddt_scale) * p.p1);
        let eq26_e348_d_n5: f64 = ((eq26_e345_d_n5 * ddt_scale) * p.p1);
        let eq26_e348_d_n6: f64 = ((eq26_e345_d_n6 * ddt_scale) * p.p1);
        let eq26_e348_d_n7: f64 = ((eq26_e345_d_n7 * ddt_scale) * p.p1);
        let eq26_e348_d_n8: f64 = ((eq26_e345_d_n8 * ddt_scale) * p.p1);
        let eq26_e348_d_n9: f64 = ((eq26_e345_d_n9 * ddt_scale) * p.p1);
        let eq26_e348_d_n10: f64 = ((eq26_e345_d_n10 * ddt_scale) * p.p1);
        let eq26_e348_d_n11: f64 = ((eq26_e345_d_n11 * ddt_scale) * p.p1);
        let eq26_value: f64 = eq26_e348;
        let eq26_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq26_node_derivatives: [f64; 11] = [eq26_e348_d_n0, eq26_e348_d_n1, eq26_e348_d_n3, eq26_e348_d_n4, eq26_e348_d_n5, eq26_e348_d_n6, eq26_e348_d_n7, eq26_e348_d_n8, eq26_e348_d_n9, eq26_e348_d_n10, eq26_e348_d_n11];
        let eq26_branch_derivative_indices: [usize; 0] = [];
        let eq26_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(1),
            Some(10),
            multiplicity * (eq26_value),
            &eq26_node_derivative_indices,
            &eq26_node_derivatives,
            &eq26_branch_derivative_indices,
            &eq26_branch_derivatives,
            multiplicity,
        );
        let eq27_e353: f64 = (var_gmin * var_vb1c4);
        let eq27_e353_d_n6: f64 = (var_gmin * var_vb1c4_dn6);
        let eq27_e353_d_n7: f64 = (var_gmin * var_vb1c4_dn7);
        let eq27_e353_d_n8: f64 = (var_gmin * var_vb1c4_dn8);
        let eq27_e353_d_n9: f64 = (var_gmin * var_vb1c4_dn9);
        let eq27_e353_d_n11: f64 = (var_gmin * var_vb1c4_dn11);
        let eq27_e354: f64 = (var_ib3 + eq27_e353);
        let eq27_e354_d_n6: f64 = (var_ib3_dn6 + eq27_e353_d_n6);
        let eq27_e354_d_n7: f64 = (var_ib3_dn7 + eq27_e353_d_n7);
        let eq27_e354_d_n8: f64 = (var_ib3_dn8 + eq27_e353_d_n8);
        let eq27_e354_d_n9: f64 = (var_ib3_dn9 + eq27_e353_d_n9);
        let eq27_e354_d_n11: f64 = (var_ib3_dn11 + eq27_e353_d_n11);
        let eq27_e356: f64 = (eq27_e354 + var_iex);
        let eq27_e356_d_n0: f64 = (var_ib3_dn0 + var_iex_dn0);
        let eq27_e356_d_n1: f64 = (var_ib3_dn1 + var_iex_dn1);
        let eq27_e356_d_n3: f64 = (var_ib3_dn3 + var_iex_dn3);
        let eq27_e356_d_n4: f64 = (var_ib3_dn4 + var_iex_dn4);
        let eq27_e356_d_n5: f64 = (var_ib3_dn5 + var_iex_dn5);
        let eq27_e356_d_n6: f64 = (eq27_e354_d_n6 + var_iex_dn6);
        let eq27_e356_d_n7: f64 = (eq27_e354_d_n7 + var_iex_dn7);
        let eq27_e356_d_n8: f64 = (eq27_e354_d_n8 + var_iex_dn8);
        let eq27_e356_d_n9: f64 = (eq27_e354_d_n9 + var_iex_dn9);
        let eq27_e356_d_n10: f64 = (var_ib3_dn10 + var_iex_dn10);
        let eq27_e356_d_n11: f64 = (eq27_e354_d_n11 + var_iex_dn11);
        let eq27_e357: f64 = (p.p3 * eq27_e356);
        let eq27_e357_d_n0: f64 = (p.p3 * eq27_e356_d_n0);
        let eq27_e357_d_n1: f64 = (p.p3 * eq27_e356_d_n1);
        let eq27_e357_d_n3: f64 = (p.p3 * eq27_e356_d_n3);
        let eq27_e357_d_n4: f64 = (p.p3 * eq27_e356_d_n4);
        let eq27_e357_d_n5: f64 = (p.p3 * eq27_e356_d_n5);
        let eq27_e357_d_n6: f64 = (p.p3 * eq27_e356_d_n6);
        let eq27_e357_d_n7: f64 = (p.p3 * eq27_e356_d_n7);
        let eq27_e357_d_n8: f64 = (p.p3 * eq27_e356_d_n8);
        let eq27_e357_d_n9: f64 = (p.p3 * eq27_e356_d_n9);
        let eq27_e357_d_n10: f64 = (p.p3 * eq27_e356_d_n10);
        let eq27_e357_d_n11: f64 = (p.p3 * eq27_e356_d_n11);
        let eq27_e359: f64 = (eq27_e357 * p.p1);
        let eq27_e359_d_n0: f64 = (eq27_e357_d_n0 * p.p1);
        let eq27_e359_d_n1: f64 = (eq27_e357_d_n1 * p.p1);
        let eq27_e359_d_n3: f64 = (eq27_e357_d_n3 * p.p1);
        let eq27_e359_d_n4: f64 = (eq27_e357_d_n4 * p.p1);
        let eq27_e359_d_n5: f64 = (eq27_e357_d_n5 * p.p1);
        let eq27_e359_d_n6: f64 = (eq27_e357_d_n6 * p.p1);
        let eq27_e359_d_n7: f64 = (eq27_e357_d_n7 * p.p1);
        let eq27_e359_d_n8: f64 = (eq27_e357_d_n8 * p.p1);
        let eq27_e359_d_n9: f64 = (eq27_e357_d_n9 * p.p1);
        let eq27_e359_d_n10: f64 = (eq27_e357_d_n10 * p.p1);
        let eq27_e359_d_n11: f64 = (eq27_e357_d_n11 * p.p1);
        let eq27_value: f64 = eq27_e359;
        let eq27_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq27_node_derivatives: [f64; 11] = [eq27_e359_d_n0, eq27_e359_d_n1, eq27_e359_d_n3, eq27_e359_d_n4, eq27_e359_d_n5, eq27_e359_d_n6, eq27_e359_d_n7, eq27_e359_d_n8, eq27_e359_d_n9, eq27_e359_d_n10, eq27_e359_d_n11];
        let eq27_branch_derivative_indices: [usize; 0] = [];
        let eq27_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(11),
            multiplicity * (eq27_value),
            &eq27_node_derivative_indices,
            &eq27_node_derivatives,
            &eq27_branch_derivative_indices,
            &eq27_branch_derivatives,
            multiplicity,
        );
        let eq28_e363: f64 = (var_qtex + var_qex);
        let eq28_e363_d_n0: f64 = (var_qtex_dn0 + var_qex_dn0);
        let eq28_e363_d_n1: f64 = (var_qtex_dn1 + var_qex_dn1);
        let eq28_e363_d_n3: f64 = (var_qtex_dn3 + var_qex_dn3);
        let eq28_e363_d_n4: f64 = (var_qtex_dn4 + var_qex_dn4);
        let eq28_e363_d_n5: f64 = (var_qtex_dn5 + var_qex_dn5);
        let eq28_e363_d_n6: f64 = (var_qtex_dn6 + var_qex_dn6);
        let eq28_e363_d_n7: f64 = (var_qtex_dn7 + var_qex_dn7);
        let eq28_e363_d_n8: f64 = (var_qtex_dn8 + var_qex_dn8);
        let eq28_e363_d_n9: f64 = (var_qtex_dn9 + var_qex_dn9);
        let eq28_e363_d_n10: f64 = (var_qtex_dn10 + var_qex_dn10);
        let eq28_e363_d_n11: f64 = (var_qtex_dn11 + var_qex_dn11);
        let eq28_e364: f64 = (p.p3 * eq28_e363);
        let eq28_e364_d_n0: f64 = (p.p3 * eq28_e363_d_n0);
        let eq28_e364_d_n1: f64 = (p.p3 * eq28_e363_d_n1);
        let eq28_e364_d_n3: f64 = (p.p3 * eq28_e363_d_n3);
        let eq28_e364_d_n4: f64 = (p.p3 * eq28_e363_d_n4);
        let eq28_e364_d_n5: f64 = (p.p3 * eq28_e363_d_n5);
        let eq28_e364_d_n6: f64 = (p.p3 * eq28_e363_d_n6);
        let eq28_e364_d_n7: f64 = (p.p3 * eq28_e363_d_n7);
        let eq28_e364_d_n8: f64 = (p.p3 * eq28_e363_d_n8);
        let eq28_e364_d_n9: f64 = (p.p3 * eq28_e363_d_n9);
        let eq28_e364_d_n10: f64 = (p.p3 * eq28_e363_d_n10);
        let eq28_e364_d_n11: f64 = (p.p3 * eq28_e363_d_n11);
        let eq28_e365: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq28_e364);
        let eq28_e367: f64 = (eq28_e365 * p.p1);
        let eq28_e367_d_n0: f64 = ((eq28_e364_d_n0 * ddt_scale) * p.p1);
        let eq28_e367_d_n1: f64 = ((eq28_e364_d_n1 * ddt_scale) * p.p1);
        let eq28_e367_d_n3: f64 = ((eq28_e364_d_n3 * ddt_scale) * p.p1);
        let eq28_e367_d_n4: f64 = ((eq28_e364_d_n4 * ddt_scale) * p.p1);
        let eq28_e367_d_n5: f64 = ((eq28_e364_d_n5 * ddt_scale) * p.p1);
        let eq28_e367_d_n6: f64 = ((eq28_e364_d_n6 * ddt_scale) * p.p1);
        let eq28_e367_d_n7: f64 = ((eq28_e364_d_n7 * ddt_scale) * p.p1);
        let eq28_e367_d_n8: f64 = ((eq28_e364_d_n8 * ddt_scale) * p.p1);
        let eq28_e367_d_n9: f64 = ((eq28_e364_d_n9 * ddt_scale) * p.p1);
        let eq28_e367_d_n10: f64 = ((eq28_e364_d_n10 * ddt_scale) * p.p1);
        let eq28_e367_d_n11: f64 = ((eq28_e364_d_n11 * ddt_scale) * p.p1);
        let eq28_value: f64 = eq28_e367;
        let eq28_node_derivative_indices: [usize; 11] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let eq28_node_derivatives: [f64; 11] = [eq28_e367_d_n0, eq28_e367_d_n1, eq28_e367_d_n3, eq28_e367_d_n4, eq28_e367_d_n5, eq28_e367_d_n6, eq28_e367_d_n7, eq28_e367_d_n8, eq28_e367_d_n9, eq28_e367_d_n10, eq28_e367_d_n11];
        let eq28_branch_derivative_indices: [usize; 0] = [];
        let eq28_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(11),
            multiplicity * (eq28_value),
            &eq28_node_derivative_indices,
            &eq28_node_derivatives,
            &eq28_branch_derivative_indices,
            &eq28_branch_derivatives,
            multiplicity,
        );
        let (eq29_e377, eq29_e377_d_n4, eq29_e377_d_n10, eq29_e377_d_n11,) = {
    if (var_guard129 != 0.0) {
        let eq29_e371: f64 = (p.p3 * var_vc3c4);
        let eq29_e371_d_n10: f64 = (p.p3 * var_vc3c4_dn10);
        let eq29_e371_d_n11: f64 = (p.p3 * var_vc3c4_dn11);
        let eq29_e373: f64 = (eq29_e371 * var_gcc_ex_t);
        let eq29_e373_d_n4: f64 = (eq29_e371 * var_gcc_ex_t_dn4);
        let eq29_e373_d_n10: f64 = (eq29_e371_d_n10 * var_gcc_ex_t);
        let eq29_e373_d_n11: f64 = (eq29_e371_d_n11 * var_gcc_ex_t);
        let eq29_e375: f64 = (eq29_e373 * p.p1);
        let eq29_e375_d_n4: f64 = (eq29_e373_d_n4 * p.p1);
        let eq29_e375_d_n10: f64 = (eq29_e373_d_n10 * p.p1);
        let eq29_e375_d_n11: f64 = (eq29_e373_d_n11 * p.p1);
        (eq29_e375, eq29_e375_d_n4, eq29_e375_d_n10, eq29_e375_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e377;
        stamper.stamp_current_node3_local(
            Some(10),
            Some(11),
            multiplicity * (eq29_value),
            4,
            multiplicity * (eq29_e377_d_n4),
            10,
            multiplicity * (eq29_e377_d_n10),
            11,
            multiplicity * (eq29_e377_d_n11),
        );
        let (eq31_e392, eq31_e392_d_n4, eq31_e392_d_n8, eq31_e392_d_n11,) = {
    if (var_guard130 != 0.0) {
        let eq31_e386: f64 = (p.p3 * var_vc4c1);
        let eq31_e386_d_n8: f64 = (p.p3 * var_vc4c1_dn8);
        let eq31_e386_d_n11: f64 = (p.p3 * var_vc4c1_dn11);
        let eq31_e388: f64 = (eq31_e386 * var_gcc_in_t);
        let eq31_e388_d_n4: f64 = (eq31_e386 * var_gcc_in_t_dn4);
        let eq31_e388_d_n8: f64 = (eq31_e386_d_n8 * var_gcc_in_t);
        let eq31_e388_d_n11: f64 = (eq31_e386_d_n11 * var_gcc_in_t);
        let eq31_e390: f64 = (eq31_e388 * p.p1);
        let eq31_e390_d_n4: f64 = (eq31_e388_d_n4 * p.p1);
        let eq31_e390_d_n8: f64 = (eq31_e388_d_n8 * p.p1);
        let eq31_e390_d_n11: f64 = (eq31_e388_d_n11 * p.p1);
        (eq31_e390, eq31_e390_d_n4, eq31_e390_d_n8, eq31_e390_d_n11,)
    } else {
        (0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e392;
        stamper.stamp_current_node3_local(
            Some(11),
            Some(8),
            multiplicity * (eq31_value),
            4,
            multiplicity * (eq31_e392_d_n4),
            8,
            multiplicity * (eq31_e392_d_n8),
            11,
            multiplicity * (eq31_e392_d_n11),
        );
        let eq35_e406: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, (nv12 - 0.0));
        let eq35_e407: f64 = (var_taun * eq35_e406);
        let eq35_e407_d_n0: f64 = (var_taun_dn0 * eq35_e406);
        let eq35_e407_d_n1: f64 = (var_taun_dn1 * eq35_e406);
        let eq35_e407_d_n3: f64 = (var_taun_dn3 * eq35_e406);
        let eq35_e407_d_n4: f64 = (var_taun_dn4 * eq35_e406);
        let eq35_e407_d_n5: f64 = (var_taun_dn5 * eq35_e406);
        let eq35_e407_d_n6: f64 = (var_taun_dn6 * eq35_e406);
        let eq35_e407_d_n7: f64 = (var_taun_dn7 * eq35_e406);
        let eq35_e407_d_n8: f64 = (var_taun_dn8 * eq35_e406);
        let eq35_e407_d_n9: f64 = (var_taun_dn9 * eq35_e406);
        let eq35_e407_d_n10: f64 = (var_taun_dn10 * eq35_e406);
        let eq35_e407_d_n11: f64 = (var_taun_dn11 * eq35_e406);
        let eq35_value: f64 = eq35_e407;
        let eq35_node_derivative_indices: [usize; 12] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let eq35_node_derivatives: [f64; 12] = [eq35_e407_d_n0, eq35_e407_d_n1, eq35_e407_d_n3, eq35_e407_d_n4, eq35_e407_d_n5, eq35_e407_d_n6, eq35_e407_d_n7, eq35_e407_d_n8, eq35_e407_d_n9, eq35_e407_d_n10, eq35_e407_d_n11, (var_taun * ddt_scale)];
        let eq35_branch_derivative_indices: [usize; 0] = [];
        let eq35_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(5),
            multiplicity * (eq35_value),
            &eq35_node_derivative_indices,
            &eq35_node_derivatives,
            &eq35_branch_derivative_indices,
            &eq35_branch_derivatives,
            multiplicity,
        );
        let eq36_e410: f64 = (var_gem_n * (nv12 - 0.0));
        let eq36_e410_d_n0: f64 = (var_gem_n_dn0 * (nv12 - 0.0));
        let eq36_e410_d_n1: f64 = (var_gem_n_dn1 * (nv12 - 0.0));
        let eq36_e410_d_n3: f64 = (var_gem_n_dn3 * (nv12 - 0.0));
        let eq36_e410_d_n4: f64 = (var_gem_n_dn4 * (nv12 - 0.0));
        let eq36_e410_d_n5: f64 = (var_gem_n_dn5 * (nv12 - 0.0));
        let eq36_e410_d_n6: f64 = (var_gem_n_dn6 * (nv12 - 0.0));
        let eq36_e410_d_n7: f64 = (var_gem_n_dn7 * (nv12 - 0.0));
        let eq36_e410_d_n8: f64 = (var_gem_n_dn8 * (nv12 - 0.0));
        let eq36_e410_d_n9: f64 = (var_gem_n_dn9 * (nv12 - 0.0));
        let eq36_e410_d_n10: f64 = (var_gem_n_dn10 * (nv12 - 0.0));
        let eq36_e410_d_n11: f64 = (var_gem_n_dn11 * (nv12 - 0.0));
        let eq36_value: f64 = eq36_e410;
        let eq36_node_derivative_indices: [usize; 12] = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let eq36_node_derivatives: [f64; 12] = [eq36_e410_d_n0, eq36_e410_d_n1, eq36_e410_d_n3, eq36_e410_d_n4, eq36_e410_d_n5, eq36_e410_d_n6, eq36_e410_d_n7, eq36_e410_d_n8, eq36_e410_d_n9, eq36_e410_d_n10, eq36_e410_d_n11, var_gem_n];
        let eq36_branch_derivative_indices: [usize; 0] = [];
        let eq36_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq36_value),
            &eq36_node_derivative_indices,
            &eq36_node_derivatives,
            &eq36_branch_derivative_indices,
            &eq36_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
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
        var_i_cth_rdn12: f64,
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
        var_qb1b2_dn12: f64,
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
        var_qbc_dn12: f64,
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
        var_qbe_dn12: f64,
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
        var_qe_dn12: f64,
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
        var_qepi_dn12: f64,
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
        var_qtc_dn12: f64,
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
        var_qte_dn12: f64,
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
        var_qte_s_dn12: f64,
        var_qte_s_dn2: f64,
        var_qte_s_dn3: f64,
        var_qte_s_dn4: f64,
        var_qte_s_dn5: f64,
        var_qte_s_dn6: f64,
        var_qte_s_dn7: f64,
        var_qte_s_dn8: f64,
        var_qte_s_dn9: f64,
        var_qts: f64,
        var_qts_db0: f64,
        var_qts_db1: f64,
        var_qts_dn0: f64,
        var_qts_dn1: f64,
        var_qts_dn10: f64,
        var_qts_dn11: f64,
        var_qts_dn12: f64,
        var_qts_dn2: f64,
        var_qts_dn3: f64,
        var_qts_dn4: f64,
        var_qts_dn5: f64,
        var_qts_dn6: f64,
        var_qts_dn7: f64,
        var_qts_dn8: f64,
        var_qts_dn9: f64,
        var_vbc: f64,
        var_vbc_db0: f64,
        var_vbc_db1: f64,
        var_vbc_dn0: f64,
        var_vbc_dn1: f64,
        var_vbc_dn10: f64,
        var_vbc_dn11: f64,
        var_vbc_dn12: f64,
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
        var_vbe_dn12: f64,
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
        var_xqex_dn12: f64,
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
        var_xqtex_dn12: f64,
        var_xqtex_dn2: f64,
        var_xqtex_dn3: f64,
        var_xqtex_dn4: f64,
        var_xqtex_dn5: f64,
        var_xqtex_dn6: f64,
        var_xqtex_dn7: f64,
        var_xqtex_dn8: f64,
        var_xqtex_dn9: f64,
    ) {
        let eq15_e268_q: f64 = var_i_cth_rv;
        let eq15_reactive_node_derivatives: [f64; 13] = [var_i_cth_rdn0, var_i_cth_rdn1, var_i_cth_rdn2, var_i_cth_rdn3, var_i_cth_rdn4, var_i_cth_rdn5, var_i_cth_rdn6, var_i_cth_rdn7, var_i_cth_rdn8, var_i_cth_rdn9, var_i_cth_rdn10, var_i_cth_rdn11, var_i_cth_rdn12];
        let eq15_reactive_branch_derivatives: [f64; 2] = [var_i_cth_rdb0, var_i_cth_rdb1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq15_reactive_node_derivatives,
            branches,
            &eq15_reactive_branch_derivatives,
            multiplicity,
        );
        let eq17_e278: f64 = (var_qte + var_qbe);
        let eq17_e278_d_n0: f64 = (var_qte_dn0 + var_qbe_dn0);
        let eq17_e278_d_n1: f64 = (var_qte_dn1 + var_qbe_dn1);
        let eq17_e278_d_n2: f64 = (var_qte_dn2 + var_qbe_dn2);
        let eq17_e278_d_n3: f64 = (var_qte_dn3 + var_qbe_dn3);
        let eq17_e278_d_n4: f64 = (var_qte_dn4 + var_qbe_dn4);
        let eq17_e278_d_n5: f64 = (var_qte_dn5 + var_qbe_dn5);
        let eq17_e278_d_n6: f64 = (var_qte_dn6 + var_qbe_dn6);
        let eq17_e278_d_n7: f64 = (var_qte_dn7 + var_qbe_dn7);
        let eq17_e278_d_n8: f64 = (var_qte_dn8 + var_qbe_dn8);
        let eq17_e278_d_n9: f64 = (var_qte_dn9 + var_qbe_dn9);
        let eq17_e278_d_n10: f64 = (var_qte_dn10 + var_qbe_dn10);
        let eq17_e278_d_n11: f64 = (var_qte_dn11 + var_qbe_dn11);
        let eq17_e278_d_n12: f64 = (var_qte_dn12 + var_qbe_dn12);
        let eq17_e278_d_b0: f64 = (var_qte_db0 + var_qbe_db0);
        let eq17_e278_d_b1: f64 = (var_qte_db1 + var_qbe_db1);
        let eq17_e280: f64 = (eq17_e278 + var_qe);
        let eq17_e280_d_n0: f64 = (eq17_e278_d_n0 + var_qe_dn0);
        let eq17_e280_d_n1: f64 = (eq17_e278_d_n1 + var_qe_dn1);
        let eq17_e280_d_n2: f64 = (eq17_e278_d_n2 + var_qe_dn2);
        let eq17_e280_d_n3: f64 = (eq17_e278_d_n3 + var_qe_dn3);
        let eq17_e280_d_n4: f64 = (eq17_e278_d_n4 + var_qe_dn4);
        let eq17_e280_d_n5: f64 = (eq17_e278_d_n5 + var_qe_dn5);
        let eq17_e280_d_n6: f64 = (eq17_e278_d_n6 + var_qe_dn6);
        let eq17_e280_d_n7: f64 = (eq17_e278_d_n7 + var_qe_dn7);
        let eq17_e280_d_n8: f64 = (eq17_e278_d_n8 + var_qe_dn8);
        let eq17_e280_d_n9: f64 = (eq17_e278_d_n9 + var_qe_dn9);
        let eq17_e280_d_n10: f64 = (eq17_e278_d_n10 + var_qe_dn10);
        let eq17_e280_d_n11: f64 = (eq17_e278_d_n11 + var_qe_dn11);
        let eq17_e280_d_n12: f64 = (eq17_e278_d_n12 + var_qe_dn12);
        let eq17_e280_d_b0: f64 = (eq17_e278_d_b0 + var_qe_db0);
        let eq17_e280_d_b1: f64 = (eq17_e278_d_b1 + var_qe_db1);
        let eq17_e281: f64 = (p.p3 * eq17_e280);
        let eq17_e281_d_n0: f64 = (p.p3 * eq17_e280_d_n0);
        let eq17_e281_d_n1: f64 = (p.p3 * eq17_e280_d_n1);
        let eq17_e281_d_n2: f64 = (p.p3 * eq17_e280_d_n2);
        let eq17_e281_d_n3: f64 = (p.p3 * eq17_e280_d_n3);
        let eq17_e281_d_n4: f64 = (p.p3 * eq17_e280_d_n4);
        let eq17_e281_d_n5: f64 = (p.p3 * eq17_e280_d_n5);
        let eq17_e281_d_n6: f64 = (p.p3 * eq17_e280_d_n6);
        let eq17_e281_d_n7: f64 = (p.p3 * eq17_e280_d_n7);
        let eq17_e281_d_n8: f64 = (p.p3 * eq17_e280_d_n8);
        let eq17_e281_d_n9: f64 = (p.p3 * eq17_e280_d_n9);
        let eq17_e281_d_n10: f64 = (p.p3 * eq17_e280_d_n10);
        let eq17_e281_d_n11: f64 = (p.p3 * eq17_e280_d_n11);
        let eq17_e281_d_n12: f64 = (p.p3 * eq17_e280_d_n12);
        let eq17_e281_d_b0: f64 = (p.p3 * eq17_e280_d_b0);
        let eq17_e281_d_b1: f64 = (p.p3 * eq17_e280_d_b1);
        let eq17_e282_q: f64 = eq17_e281;
        let eq17_e284: f64 = (eq17_e281 * p.p1);
        let eq17_e284_d_n0: f64 = (eq17_e281_d_n0 * p.p1);
        let eq17_e284_d_n1: f64 = (eq17_e281_d_n1 * p.p1);
        let eq17_e284_d_n2: f64 = (eq17_e281_d_n2 * p.p1);
        let eq17_e284_d_n3: f64 = (eq17_e281_d_n3 * p.p1);
        let eq17_e284_d_n4: f64 = (eq17_e281_d_n4 * p.p1);
        let eq17_e284_d_n5: f64 = (eq17_e281_d_n5 * p.p1);
        let eq17_e284_d_n6: f64 = (eq17_e281_d_n6 * p.p1);
        let eq17_e284_d_n7: f64 = (eq17_e281_d_n7 * p.p1);
        let eq17_e284_d_n8: f64 = (eq17_e281_d_n8 * p.p1);
        let eq17_e284_d_n9: f64 = (eq17_e281_d_n9 * p.p1);
        let eq17_e284_d_n10: f64 = (eq17_e281_d_n10 * p.p1);
        let eq17_e284_d_n11: f64 = (eq17_e281_d_n11 * p.p1);
        let eq17_e284_d_n12: f64 = (eq17_e281_d_n12 * p.p1);
        let eq17_e284_d_b0: f64 = (eq17_e281_d_b0 * p.p1);
        let eq17_e284_d_b1: f64 = (eq17_e281_d_b1 * p.p1);
        let eq17_e284_q: f64 = (eq17_e282_q * p.p1);
        let eq17_reactive_node_derivatives: [f64; 13] = [eq17_e284_d_n0, eq17_e284_d_n1, eq17_e284_d_n2, eq17_e284_d_n3, eq17_e284_d_n4, eq17_e284_d_n5, eq17_e284_d_n6, eq17_e284_d_n7, eq17_e284_d_n8, eq17_e284_d_n9, eq17_e284_d_n10, eq17_e284_d_n11, eq17_e284_d_n12];
        let eq17_reactive_branch_derivatives: [f64; 2] = [eq17_e284_d_b0, eq17_e284_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes,
            &eq17_reactive_node_derivatives,
            branches,
            &eq17_reactive_branch_derivatives,
            multiplicity,
        );
        let eq18_e287: f64 = (p.p3 * var_qte_s);
        let eq18_e287_d_n0: f64 = (p.p3 * var_qte_s_dn0);
        let eq18_e287_d_n1: f64 = (p.p3 * var_qte_s_dn1);
        let eq18_e287_d_n2: f64 = (p.p3 * var_qte_s_dn2);
        let eq18_e287_d_n3: f64 = (p.p3 * var_qte_s_dn3);
        let eq18_e287_d_n4: f64 = (p.p3 * var_qte_s_dn4);
        let eq18_e287_d_n5: f64 = (p.p3 * var_qte_s_dn5);
        let eq18_e287_d_n6: f64 = (p.p3 * var_qte_s_dn6);
        let eq18_e287_d_n7: f64 = (p.p3 * var_qte_s_dn7);
        let eq18_e287_d_n8: f64 = (p.p3 * var_qte_s_dn8);
        let eq18_e287_d_n9: f64 = (p.p3 * var_qte_s_dn9);
        let eq18_e287_d_n10: f64 = (p.p3 * var_qte_s_dn10);
        let eq18_e287_d_n11: f64 = (p.p3 * var_qte_s_dn11);
        let eq18_e287_d_n12: f64 = (p.p3 * var_qte_s_dn12);
        let eq18_e287_d_b0: f64 = (p.p3 * var_qte_s_db0);
        let eq18_e287_d_b1: f64 = (p.p3 * var_qte_s_db1);
        let eq18_e288_q: f64 = eq18_e287;
        let eq18_e290: f64 = (eq18_e287 * p.p1);
        let eq18_e290_d_n0: f64 = (eq18_e287_d_n0 * p.p1);
        let eq18_e290_d_n1: f64 = (eq18_e287_d_n1 * p.p1);
        let eq18_e290_d_n2: f64 = (eq18_e287_d_n2 * p.p1);
        let eq18_e290_d_n3: f64 = (eq18_e287_d_n3 * p.p1);
        let eq18_e290_d_n4: f64 = (eq18_e287_d_n4 * p.p1);
        let eq18_e290_d_n5: f64 = (eq18_e287_d_n5 * p.p1);
        let eq18_e290_d_n6: f64 = (eq18_e287_d_n6 * p.p1);
        let eq18_e290_d_n7: f64 = (eq18_e287_d_n7 * p.p1);
        let eq18_e290_d_n8: f64 = (eq18_e287_d_n8 * p.p1);
        let eq18_e290_d_n9: f64 = (eq18_e287_d_n9 * p.p1);
        let eq18_e290_d_n10: f64 = (eq18_e287_d_n10 * p.p1);
        let eq18_e290_d_n11: f64 = (eq18_e287_d_n11 * p.p1);
        let eq18_e290_d_n12: f64 = (eq18_e287_d_n12 * p.p1);
        let eq18_e290_d_b0: f64 = (eq18_e287_d_b0 * p.p1);
        let eq18_e290_d_b1: f64 = (eq18_e287_d_b1 * p.p1);
        let eq18_e290_q: f64 = (eq18_e288_q * p.p1);
        let eq18_reactive_node_derivatives: [f64; 13] = [eq18_e290_d_n0, eq18_e290_d_n1, eq18_e290_d_n2, eq18_e290_d_n3, eq18_e290_d_n4, eq18_e290_d_n5, eq18_e290_d_n6, eq18_e290_d_n7, eq18_e290_d_n8, eq18_e290_d_n9, eq18_e290_d_n10, eq18_e290_d_n11, eq18_e290_d_n12];
        let eq18_reactive_branch_derivatives: [f64; 2] = [eq18_e290_d_b0, eq18_e290_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[5]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let eq19_e294: f64 = (var_qtc + var_qbc);
        let eq19_e294_d_n0: f64 = (var_qtc_dn0 + var_qbc_dn0);
        let eq19_e294_d_n1: f64 = (var_qtc_dn1 + var_qbc_dn1);
        let eq19_e294_d_n2: f64 = (var_qtc_dn2 + var_qbc_dn2);
        let eq19_e294_d_n3: f64 = (var_qtc_dn3 + var_qbc_dn3);
        let eq19_e294_d_n4: f64 = (var_qtc_dn4 + var_qbc_dn4);
        let eq19_e294_d_n5: f64 = (var_qtc_dn5 + var_qbc_dn5);
        let eq19_e294_d_n6: f64 = (var_qtc_dn6 + var_qbc_dn6);
        let eq19_e294_d_n7: f64 = (var_qtc_dn7 + var_qbc_dn7);
        let eq19_e294_d_n8: f64 = (var_qtc_dn8 + var_qbc_dn8);
        let eq19_e294_d_n9: f64 = (var_qtc_dn9 + var_qbc_dn9);
        let eq19_e294_d_n10: f64 = (var_qtc_dn10 + var_qbc_dn10);
        let eq19_e294_d_n11: f64 = (var_qtc_dn11 + var_qbc_dn11);
        let eq19_e294_d_n12: f64 = (var_qtc_dn12 + var_qbc_dn12);
        let eq19_e294_d_b0: f64 = (var_qtc_db0 + var_qbc_db0);
        let eq19_e294_d_b1: f64 = (var_qtc_db1 + var_qbc_db1);
        let eq19_e296: f64 = (eq19_e294 + var_qepi);
        let eq19_e296_d_n0: f64 = (eq19_e294_d_n0 + var_qepi_dn0);
        let eq19_e296_d_n1: f64 = (eq19_e294_d_n1 + var_qepi_dn1);
        let eq19_e296_d_n2: f64 = (eq19_e294_d_n2 + var_qepi_dn2);
        let eq19_e296_d_n3: f64 = (eq19_e294_d_n3 + var_qepi_dn3);
        let eq19_e296_d_n4: f64 = (eq19_e294_d_n4 + var_qepi_dn4);
        let eq19_e296_d_n5: f64 = (eq19_e294_d_n5 + var_qepi_dn5);
        let eq19_e296_d_n6: f64 = (eq19_e294_d_n6 + var_qepi_dn6);
        let eq19_e296_d_n7: f64 = (eq19_e294_d_n7 + var_qepi_dn7);
        let eq19_e296_d_n8: f64 = (eq19_e294_d_n8 + var_qepi_dn8);
        let eq19_e296_d_n9: f64 = (eq19_e294_d_n9 + var_qepi_dn9);
        let eq19_e296_d_n10: f64 = (eq19_e294_d_n10 + var_qepi_dn10);
        let eq19_e296_d_n11: f64 = (eq19_e294_d_n11 + var_qepi_dn11);
        let eq19_e296_d_n12: f64 = (eq19_e294_d_n12 + var_qepi_dn12);
        let eq19_e296_d_b0: f64 = (eq19_e294_d_b0 + var_qepi_db0);
        let eq19_e296_d_b1: f64 = (eq19_e294_d_b1 + var_qepi_db1);
        let eq19_e297: f64 = (p.p3 * eq19_e296);
        let eq19_e297_d_n0: f64 = (p.p3 * eq19_e296_d_n0);
        let eq19_e297_d_n1: f64 = (p.p3 * eq19_e296_d_n1);
        let eq19_e297_d_n2: f64 = (p.p3 * eq19_e296_d_n2);
        let eq19_e297_d_n3: f64 = (p.p3 * eq19_e296_d_n3);
        let eq19_e297_d_n4: f64 = (p.p3 * eq19_e296_d_n4);
        let eq19_e297_d_n5: f64 = (p.p3 * eq19_e296_d_n5);
        let eq19_e297_d_n6: f64 = (p.p3 * eq19_e296_d_n6);
        let eq19_e297_d_n7: f64 = (p.p3 * eq19_e296_d_n7);
        let eq19_e297_d_n8: f64 = (p.p3 * eq19_e296_d_n8);
        let eq19_e297_d_n9: f64 = (p.p3 * eq19_e296_d_n9);
        let eq19_e297_d_n10: f64 = (p.p3 * eq19_e296_d_n10);
        let eq19_e297_d_n11: f64 = (p.p3 * eq19_e296_d_n11);
        let eq19_e297_d_n12: f64 = (p.p3 * eq19_e296_d_n12);
        let eq19_e297_d_b0: f64 = (p.p3 * eq19_e296_d_b0);
        let eq19_e297_d_b1: f64 = (p.p3 * eq19_e296_d_b1);
        let eq19_e298_q: f64 = eq19_e297;
        let eq19_e300: f64 = (eq19_e297 * p.p1);
        let eq19_e300_d_n0: f64 = (eq19_e297_d_n0 * p.p1);
        let eq19_e300_d_n1: f64 = (eq19_e297_d_n1 * p.p1);
        let eq19_e300_d_n2: f64 = (eq19_e297_d_n2 * p.p1);
        let eq19_e300_d_n3: f64 = (eq19_e297_d_n3 * p.p1);
        let eq19_e300_d_n4: f64 = (eq19_e297_d_n4 * p.p1);
        let eq19_e300_d_n5: f64 = (eq19_e297_d_n5 * p.p1);
        let eq19_e300_d_n6: f64 = (eq19_e297_d_n6 * p.p1);
        let eq19_e300_d_n7: f64 = (eq19_e297_d_n7 * p.p1);
        let eq19_e300_d_n8: f64 = (eq19_e297_d_n8 * p.p1);
        let eq19_e300_d_n9: f64 = (eq19_e297_d_n9 * p.p1);
        let eq19_e300_d_n10: f64 = (eq19_e297_d_n10 * p.p1);
        let eq19_e300_d_n11: f64 = (eq19_e297_d_n11 * p.p1);
        let eq19_e300_d_n12: f64 = (eq19_e297_d_n12 * p.p1);
        let eq19_e300_d_b0: f64 = (eq19_e297_d_b0 * p.p1);
        let eq19_e300_d_b1: f64 = (eq19_e297_d_b1 * p.p1);
        let eq19_e300_q: f64 = (eq19_e298_q * p.p1);
        let eq19_reactive_node_derivatives: [f64; 13] = [eq19_e300_d_n0, eq19_e300_d_n1, eq19_e300_d_n2, eq19_e300_d_n3, eq19_e300_d_n4, eq19_e300_d_n5, eq19_e300_d_n6, eq19_e300_d_n7, eq19_e300_d_n8, eq19_e300_d_n9, eq19_e300_d_n10, eq19_e300_d_n11, eq19_e300_d_n12];
        let eq19_reactive_branch_derivatives: [f64; 2] = [eq19_e300_d_b0, eq19_e300_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let eq20_e303: f64 = (p.p3 * var_qts);
        let eq20_e303_d_n0: f64 = (p.p3 * var_qts_dn0);
        let eq20_e303_d_n1: f64 = (p.p3 * var_qts_dn1);
        let eq20_e303_d_n2: f64 = (p.p3 * var_qts_dn2);
        let eq20_e303_d_n3: f64 = (p.p3 * var_qts_dn3);
        let eq20_e303_d_n4: f64 = (p.p3 * var_qts_dn4);
        let eq20_e303_d_n5: f64 = (p.p3 * var_qts_dn5);
        let eq20_e303_d_n6: f64 = (p.p3 * var_qts_dn6);
        let eq20_e303_d_n7: f64 = (p.p3 * var_qts_dn7);
        let eq20_e303_d_n8: f64 = (p.p3 * var_qts_dn8);
        let eq20_e303_d_n9: f64 = (p.p3 * var_qts_dn9);
        let eq20_e303_d_n10: f64 = (p.p3 * var_qts_dn10);
        let eq20_e303_d_n11: f64 = (p.p3 * var_qts_dn11);
        let eq20_e303_d_n12: f64 = (p.p3 * var_qts_dn12);
        let eq20_e303_d_b0: f64 = (p.p3 * var_qts_db0);
        let eq20_e303_d_b1: f64 = (p.p3 * var_qts_db1);
        let eq20_e304_q: f64 = eq20_e303;
        let eq20_e306: f64 = (eq20_e303 * p.p1);
        let eq20_e306_d_n0: f64 = (eq20_e303_d_n0 * p.p1);
        let eq20_e306_d_n1: f64 = (eq20_e303_d_n1 * p.p1);
        let eq20_e306_d_n2: f64 = (eq20_e303_d_n2 * p.p1);
        let eq20_e306_d_n3: f64 = (eq20_e303_d_n3 * p.p1);
        let eq20_e306_d_n4: f64 = (eq20_e303_d_n4 * p.p1);
        let eq20_e306_d_n5: f64 = (eq20_e303_d_n5 * p.p1);
        let eq20_e306_d_n6: f64 = (eq20_e303_d_n6 * p.p1);
        let eq20_e306_d_n7: f64 = (eq20_e303_d_n7 * p.p1);
        let eq20_e306_d_n8: f64 = (eq20_e303_d_n8 * p.p1);
        let eq20_e306_d_n9: f64 = (eq20_e303_d_n9 * p.p1);
        let eq20_e306_d_n10: f64 = (eq20_e303_d_n10 * p.p1);
        let eq20_e306_d_n11: f64 = (eq20_e303_d_n11 * p.p1);
        let eq20_e306_d_n12: f64 = (eq20_e303_d_n12 * p.p1);
        let eq20_e306_d_b0: f64 = (eq20_e303_d_b0 * p.p1);
        let eq20_e306_d_b1: f64 = (eq20_e303_d_b1 * p.p1);
        let eq20_e306_q: f64 = (eq20_e304_q * p.p1);
        let eq20_reactive_node_derivatives: [f64; 13] = [eq20_e306_d_n0, eq20_e306_d_n1, eq20_e306_d_n2, eq20_e306_d_n3, eq20_e306_d_n4, eq20_e306_d_n5, eq20_e306_d_n6, eq20_e306_d_n7, eq20_e306_d_n8, eq20_e306_d_n9, eq20_e306_d_n10, eq20_e306_d_n11, eq20_e306_d_n12];
        let eq20_reactive_branch_derivatives: [f64; 2] = [eq20_e306_d_b0, eq20_e306_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[3]),
            Some(nodes[8]),
            nodes,
            &eq20_reactive_node_derivatives,
            branches,
            &eq20_reactive_branch_derivatives,
            multiplicity,
        );
        let eq21_e309: f64 = (p.p3 * var_qb1b2);
        let eq21_e309_d_n0: f64 = (p.p3 * var_qb1b2_dn0);
        let eq21_e309_d_n1: f64 = (p.p3 * var_qb1b2_dn1);
        let eq21_e309_d_n2: f64 = (p.p3 * var_qb1b2_dn2);
        let eq21_e309_d_n3: f64 = (p.p3 * var_qb1b2_dn3);
        let eq21_e309_d_n4: f64 = (p.p3 * var_qb1b2_dn4);
        let eq21_e309_d_n5: f64 = (p.p3 * var_qb1b2_dn5);
        let eq21_e309_d_n6: f64 = (p.p3 * var_qb1b2_dn6);
        let eq21_e309_d_n7: f64 = (p.p3 * var_qb1b2_dn7);
        let eq21_e309_d_n8: f64 = (p.p3 * var_qb1b2_dn8);
        let eq21_e309_d_n9: f64 = (p.p3 * var_qb1b2_dn9);
        let eq21_e309_d_n10: f64 = (p.p3 * var_qb1b2_dn10);
        let eq21_e309_d_n11: f64 = (p.p3 * var_qb1b2_dn11);
        let eq21_e309_d_n12: f64 = (p.p3 * var_qb1b2_dn12);
        let eq21_e309_d_b0: f64 = (p.p3 * var_qb1b2_db0);
        let eq21_e309_d_b1: f64 = (p.p3 * var_qb1b2_db1);
        let eq21_e310_q: f64 = eq21_e309;
        let eq21_e312: f64 = (eq21_e309 * p.p1);
        let eq21_e312_d_n0: f64 = (eq21_e309_d_n0 * p.p1);
        let eq21_e312_d_n1: f64 = (eq21_e309_d_n1 * p.p1);
        let eq21_e312_d_n2: f64 = (eq21_e309_d_n2 * p.p1);
        let eq21_e312_d_n3: f64 = (eq21_e309_d_n3 * p.p1);
        let eq21_e312_d_n4: f64 = (eq21_e309_d_n4 * p.p1);
        let eq21_e312_d_n5: f64 = (eq21_e309_d_n5 * p.p1);
        let eq21_e312_d_n6: f64 = (eq21_e309_d_n6 * p.p1);
        let eq21_e312_d_n7: f64 = (eq21_e309_d_n7 * p.p1);
        let eq21_e312_d_n8: f64 = (eq21_e309_d_n8 * p.p1);
        let eq21_e312_d_n9: f64 = (eq21_e309_d_n9 * p.p1);
        let eq21_e312_d_n10: f64 = (eq21_e309_d_n10 * p.p1);
        let eq21_e312_d_n11: f64 = (eq21_e309_d_n11 * p.p1);
        let eq21_e312_d_n12: f64 = (eq21_e309_d_n12 * p.p1);
        let eq21_e312_d_b0: f64 = (eq21_e309_d_b0 * p.p1);
        let eq21_e312_d_b1: f64 = (eq21_e309_d_b1 * p.p1);
        let eq21_e312_q: f64 = (eq21_e310_q * p.p1);
        let eq21_reactive_node_derivatives: [f64; 13] = [eq21_e312_d_n0, eq21_e312_d_n1, eq21_e312_d_n2, eq21_e312_d_n3, eq21_e312_d_n4, eq21_e312_d_n5, eq21_e312_d_n6, eq21_e312_d_n7, eq21_e312_d_n8, eq21_e312_d_n9, eq21_e312_d_n10, eq21_e312_d_n11, eq21_e312_d_n12];
        let eq21_reactive_branch_derivatives: [f64; 2] = [eq21_e312_d_b0, eq21_e312_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq21_reactive_node_derivatives,
            branches,
            &eq21_reactive_branch_derivatives,
            multiplicity,
        );
        let eq22_e315: f64 = (p.p3 * p.p69);
        let eq22_e317: f64 = (eq22_e315 * var_vbe);
        let eq22_e317_d_n0: f64 = (eq22_e315 * var_vbe_dn0);
        let eq22_e317_d_n1: f64 = (eq22_e315 * var_vbe_dn1);
        let eq22_e317_d_n2: f64 = (eq22_e315 * var_vbe_dn2);
        let eq22_e317_d_n3: f64 = (eq22_e315 * var_vbe_dn3);
        let eq22_e317_d_n4: f64 = (eq22_e315 * var_vbe_dn4);
        let eq22_e317_d_n5: f64 = (eq22_e315 * var_vbe_dn5);
        let eq22_e317_d_n6: f64 = (eq22_e315 * var_vbe_dn6);
        let eq22_e317_d_n7: f64 = (eq22_e315 * var_vbe_dn7);
        let eq22_e317_d_n8: f64 = (eq22_e315 * var_vbe_dn8);
        let eq22_e317_d_n9: f64 = (eq22_e315 * var_vbe_dn9);
        let eq22_e317_d_n10: f64 = (eq22_e315 * var_vbe_dn10);
        let eq22_e317_d_n11: f64 = (eq22_e315 * var_vbe_dn11);
        let eq22_e317_d_n12: f64 = (eq22_e315 * var_vbe_dn12);
        let eq22_e317_d_b0: f64 = (eq22_e315 * var_vbe_db0);
        let eq22_e317_d_b1: f64 = (eq22_e315 * var_vbe_db1);
        let eq22_e318_q: f64 = eq22_e317;
        let eq22_e320: f64 = (eq22_e317 * p.p1);
        let eq22_e320_d_n0: f64 = (eq22_e317_d_n0 * p.p1);
        let eq22_e320_d_n1: f64 = (eq22_e317_d_n1 * p.p1);
        let eq22_e320_d_n2: f64 = (eq22_e317_d_n2 * p.p1);
        let eq22_e320_d_n3: f64 = (eq22_e317_d_n3 * p.p1);
        let eq22_e320_d_n4: f64 = (eq22_e317_d_n4 * p.p1);
        let eq22_e320_d_n5: f64 = (eq22_e317_d_n5 * p.p1);
        let eq22_e320_d_n6: f64 = (eq22_e317_d_n6 * p.p1);
        let eq22_e320_d_n7: f64 = (eq22_e317_d_n7 * p.p1);
        let eq22_e320_d_n8: f64 = (eq22_e317_d_n8 * p.p1);
        let eq22_e320_d_n9: f64 = (eq22_e317_d_n9 * p.p1);
        let eq22_e320_d_n10: f64 = (eq22_e317_d_n10 * p.p1);
        let eq22_e320_d_n11: f64 = (eq22_e317_d_n11 * p.p1);
        let eq22_e320_d_n12: f64 = (eq22_e317_d_n12 * p.p1);
        let eq22_e320_d_b0: f64 = (eq22_e317_d_b0 * p.p1);
        let eq22_e320_d_b1: f64 = (eq22_e317_d_b1 * p.p1);
        let eq22_e320_q: f64 = (eq22_e318_q * p.p1);
        let eq22_reactive_node_derivatives: [f64; 13] = [eq22_e320_d_n0, eq22_e320_d_n1, eq22_e320_d_n2, eq22_e320_d_n3, eq22_e320_d_n4, eq22_e320_d_n5, eq22_e320_d_n6, eq22_e320_d_n7, eq22_e320_d_n8, eq22_e320_d_n9, eq22_e320_d_n10, eq22_e320_d_n11, eq22_e320_d_n12];
        let eq22_reactive_branch_derivatives: [f64; 2] = [eq22_e320_d_b0, eq22_e320_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes,
            &eq22_reactive_node_derivatives,
            branches,
            &eq22_reactive_branch_derivatives,
            multiplicity,
        );
        let eq23_e323: f64 = (p.p3 * p.p78);
        let eq23_e325: f64 = (eq23_e323 * var_vbc);
        let eq23_e325_d_n0: f64 = (eq23_e323 * var_vbc_dn0);
        let eq23_e325_d_n1: f64 = (eq23_e323 * var_vbc_dn1);
        let eq23_e325_d_n2: f64 = (eq23_e323 * var_vbc_dn2);
        let eq23_e325_d_n3: f64 = (eq23_e323 * var_vbc_dn3);
        let eq23_e325_d_n4: f64 = (eq23_e323 * var_vbc_dn4);
        let eq23_e325_d_n5: f64 = (eq23_e323 * var_vbc_dn5);
        let eq23_e325_d_n6: f64 = (eq23_e323 * var_vbc_dn6);
        let eq23_e325_d_n7: f64 = (eq23_e323 * var_vbc_dn7);
        let eq23_e325_d_n8: f64 = (eq23_e323 * var_vbc_dn8);
        let eq23_e325_d_n9: f64 = (eq23_e323 * var_vbc_dn9);
        let eq23_e325_d_n10: f64 = (eq23_e323 * var_vbc_dn10);
        let eq23_e325_d_n11: f64 = (eq23_e323 * var_vbc_dn11);
        let eq23_e325_d_n12: f64 = (eq23_e323 * var_vbc_dn12);
        let eq23_e325_d_b0: f64 = (eq23_e323 * var_vbc_db0);
        let eq23_e325_d_b1: f64 = (eq23_e323 * var_vbc_db1);
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
        let eq23_e328_d_n12: f64 = (eq23_e325_d_n12 * p.p1);
        let eq23_e328_d_b0: f64 = (eq23_e325_d_b0 * p.p1);
        let eq23_e328_d_b1: f64 = (eq23_e325_d_b1 * p.p1);
        let eq23_e328_q: f64 = (eq23_e326_q * p.p1);
        let eq23_reactive_node_derivatives: [f64; 13] = [eq23_e328_d_n0, eq23_e328_d_n1, eq23_e328_d_n2, eq23_e328_d_n3, eq23_e328_d_n4, eq23_e328_d_n5, eq23_e328_d_n6, eq23_e328_d_n7, eq23_e328_d_n8, eq23_e328_d_n9, eq23_e328_d_n10, eq23_e328_d_n11, eq23_e328_d_n12];
        let eq23_reactive_branch_derivatives: [f64; 2] = [eq23_e328_d_b0, eq23_e328_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes,
            &eq23_reactive_node_derivatives,
            branches,
            &eq23_reactive_branch_derivatives,
            multiplicity,
        );
        let eq26_e344: f64 = (var_xqtex + var_xqex);
        let eq26_e344_d_n0: f64 = (var_xqtex_dn0 + var_xqex_dn0);
        let eq26_e344_d_n1: f64 = (var_xqtex_dn1 + var_xqex_dn1);
        let eq26_e344_d_n2: f64 = (var_xqtex_dn2 + var_xqex_dn2);
        let eq26_e344_d_n3: f64 = (var_xqtex_dn3 + var_xqex_dn3);
        let eq26_e344_d_n4: f64 = (var_xqtex_dn4 + var_xqex_dn4);
        let eq26_e344_d_n5: f64 = (var_xqtex_dn5 + var_xqex_dn5);
        let eq26_e344_d_n6: f64 = (var_xqtex_dn6 + var_xqex_dn6);
        let eq26_e344_d_n7: f64 = (var_xqtex_dn7 + var_xqex_dn7);
        let eq26_e344_d_n8: f64 = (var_xqtex_dn8 + var_xqex_dn8);
        let eq26_e344_d_n9: f64 = (var_xqtex_dn9 + var_xqex_dn9);
        let eq26_e344_d_n10: f64 = (var_xqtex_dn10 + var_xqex_dn10);
        let eq26_e344_d_n11: f64 = (var_xqtex_dn11 + var_xqex_dn11);
        let eq26_e344_d_n12: f64 = (var_xqtex_dn12 + var_xqex_dn12);
        let eq26_e344_d_b0: f64 = (var_xqtex_db0 + var_xqex_db0);
        let eq26_e344_d_b1: f64 = (var_xqtex_db1 + var_xqex_db1);
        let eq26_e345: f64 = (p.p3 * eq26_e344);
        let eq26_e345_d_n0: f64 = (p.p3 * eq26_e344_d_n0);
        let eq26_e345_d_n1: f64 = (p.p3 * eq26_e344_d_n1);
        let eq26_e345_d_n2: f64 = (p.p3 * eq26_e344_d_n2);
        let eq26_e345_d_n3: f64 = (p.p3 * eq26_e344_d_n3);
        let eq26_e345_d_n4: f64 = (p.p3 * eq26_e344_d_n4);
        let eq26_e345_d_n5: f64 = (p.p3 * eq26_e344_d_n5);
        let eq26_e345_d_n6: f64 = (p.p3 * eq26_e344_d_n6);
        let eq26_e345_d_n7: f64 = (p.p3 * eq26_e344_d_n7);
        let eq26_e345_d_n8: f64 = (p.p3 * eq26_e344_d_n8);
        let eq26_e345_d_n9: f64 = (p.p3 * eq26_e344_d_n9);
        let eq26_e345_d_n10: f64 = (p.p3 * eq26_e344_d_n10);
        let eq26_e345_d_n11: f64 = (p.p3 * eq26_e344_d_n11);
        let eq26_e345_d_n12: f64 = (p.p3 * eq26_e344_d_n12);
        let eq26_e345_d_b0: f64 = (p.p3 * eq26_e344_d_b0);
        let eq26_e345_d_b1: f64 = (p.p3 * eq26_e344_d_b1);
        let eq26_e346_q: f64 = eq26_e345;
        let eq26_e348: f64 = (eq26_e345 * p.p1);
        let eq26_e348_d_n0: f64 = (eq26_e345_d_n0 * p.p1);
        let eq26_e348_d_n1: f64 = (eq26_e345_d_n1 * p.p1);
        let eq26_e348_d_n2: f64 = (eq26_e345_d_n2 * p.p1);
        let eq26_e348_d_n3: f64 = (eq26_e345_d_n3 * p.p1);
        let eq26_e348_d_n4: f64 = (eq26_e345_d_n4 * p.p1);
        let eq26_e348_d_n5: f64 = (eq26_e345_d_n5 * p.p1);
        let eq26_e348_d_n6: f64 = (eq26_e345_d_n6 * p.p1);
        let eq26_e348_d_n7: f64 = (eq26_e345_d_n7 * p.p1);
        let eq26_e348_d_n8: f64 = (eq26_e345_d_n8 * p.p1);
        let eq26_e348_d_n9: f64 = (eq26_e345_d_n9 * p.p1);
        let eq26_e348_d_n10: f64 = (eq26_e345_d_n10 * p.p1);
        let eq26_e348_d_n11: f64 = (eq26_e345_d_n11 * p.p1);
        let eq26_e348_d_n12: f64 = (eq26_e345_d_n12 * p.p1);
        let eq26_e348_d_b0: f64 = (eq26_e345_d_b0 * p.p1);
        let eq26_e348_d_b1: f64 = (eq26_e345_d_b1 * p.p1);
        let eq26_e348_q: f64 = (eq26_e346_q * p.p1);
        let eq26_reactive_node_derivatives: [f64; 13] = [eq26_e348_d_n0, eq26_e348_d_n1, eq26_e348_d_n2, eq26_e348_d_n3, eq26_e348_d_n4, eq26_e348_d_n5, eq26_e348_d_n6, eq26_e348_d_n7, eq26_e348_d_n8, eq26_e348_d_n9, eq26_e348_d_n10, eq26_e348_d_n11, eq26_e348_d_n12];
        let eq26_reactive_branch_derivatives: [f64; 2] = [eq26_e348_d_b0, eq26_e348_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[10]),
            nodes,
            &eq26_reactive_node_derivatives,
            branches,
            &eq26_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_qex: f64,
        var_qex_db0: f64,
        var_qex_db1: f64,
        var_qex_dn0: f64,
        var_qex_dn1: f64,
        var_qex_dn10: f64,
        var_qex_dn11: f64,
        var_qex_dn12: f64,
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
        var_qtex_dn12: f64,
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
        var_taun_dn12: f64,
        var_taun_dn2: f64,
        var_taun_dn3: f64,
        var_taun_dn4: f64,
        var_taun_dn5: f64,
        var_taun_dn6: f64,
        var_taun_dn7: f64,
        var_taun_dn8: f64,
        var_taun_dn9: f64,
    ) {
        let nv12 = ctx.node_voltage(nodes[12]);
        let eq28_e363: f64 = (var_qtex + var_qex);
        let eq28_e363_d_n0: f64 = (var_qtex_dn0 + var_qex_dn0);
        let eq28_e363_d_n1: f64 = (var_qtex_dn1 + var_qex_dn1);
        let eq28_e363_d_n2: f64 = (var_qtex_dn2 + var_qex_dn2);
        let eq28_e363_d_n3: f64 = (var_qtex_dn3 + var_qex_dn3);
        let eq28_e363_d_n4: f64 = (var_qtex_dn4 + var_qex_dn4);
        let eq28_e363_d_n5: f64 = (var_qtex_dn5 + var_qex_dn5);
        let eq28_e363_d_n6: f64 = (var_qtex_dn6 + var_qex_dn6);
        let eq28_e363_d_n7: f64 = (var_qtex_dn7 + var_qex_dn7);
        let eq28_e363_d_n8: f64 = (var_qtex_dn8 + var_qex_dn8);
        let eq28_e363_d_n9: f64 = (var_qtex_dn9 + var_qex_dn9);
        let eq28_e363_d_n10: f64 = (var_qtex_dn10 + var_qex_dn10);
        let eq28_e363_d_n11: f64 = (var_qtex_dn11 + var_qex_dn11);
        let eq28_e363_d_n12: f64 = (var_qtex_dn12 + var_qex_dn12);
        let eq28_e363_d_b0: f64 = (var_qtex_db0 + var_qex_db0);
        let eq28_e363_d_b1: f64 = (var_qtex_db1 + var_qex_db1);
        let eq28_e364: f64 = (p.p3 * eq28_e363);
        let eq28_e364_d_n0: f64 = (p.p3 * eq28_e363_d_n0);
        let eq28_e364_d_n1: f64 = (p.p3 * eq28_e363_d_n1);
        let eq28_e364_d_n2: f64 = (p.p3 * eq28_e363_d_n2);
        let eq28_e364_d_n3: f64 = (p.p3 * eq28_e363_d_n3);
        let eq28_e364_d_n4: f64 = (p.p3 * eq28_e363_d_n4);
        let eq28_e364_d_n5: f64 = (p.p3 * eq28_e363_d_n5);
        let eq28_e364_d_n6: f64 = (p.p3 * eq28_e363_d_n6);
        let eq28_e364_d_n7: f64 = (p.p3 * eq28_e363_d_n7);
        let eq28_e364_d_n8: f64 = (p.p3 * eq28_e363_d_n8);
        let eq28_e364_d_n9: f64 = (p.p3 * eq28_e363_d_n9);
        let eq28_e364_d_n10: f64 = (p.p3 * eq28_e363_d_n10);
        let eq28_e364_d_n11: f64 = (p.p3 * eq28_e363_d_n11);
        let eq28_e364_d_n12: f64 = (p.p3 * eq28_e363_d_n12);
        let eq28_e364_d_b0: f64 = (p.p3 * eq28_e363_d_b0);
        let eq28_e364_d_b1: f64 = (p.p3 * eq28_e363_d_b1);
        let eq28_e365_q: f64 = eq28_e364;
        let eq28_e367: f64 = (eq28_e364 * p.p1);
        let eq28_e367_d_n0: f64 = (eq28_e364_d_n0 * p.p1);
        let eq28_e367_d_n1: f64 = (eq28_e364_d_n1 * p.p1);
        let eq28_e367_d_n2: f64 = (eq28_e364_d_n2 * p.p1);
        let eq28_e367_d_n3: f64 = (eq28_e364_d_n3 * p.p1);
        let eq28_e367_d_n4: f64 = (eq28_e364_d_n4 * p.p1);
        let eq28_e367_d_n5: f64 = (eq28_e364_d_n5 * p.p1);
        let eq28_e367_d_n6: f64 = (eq28_e364_d_n6 * p.p1);
        let eq28_e367_d_n7: f64 = (eq28_e364_d_n7 * p.p1);
        let eq28_e367_d_n8: f64 = (eq28_e364_d_n8 * p.p1);
        let eq28_e367_d_n9: f64 = (eq28_e364_d_n9 * p.p1);
        let eq28_e367_d_n10: f64 = (eq28_e364_d_n10 * p.p1);
        let eq28_e367_d_n11: f64 = (eq28_e364_d_n11 * p.p1);
        let eq28_e367_d_n12: f64 = (eq28_e364_d_n12 * p.p1);
        let eq28_e367_d_b0: f64 = (eq28_e364_d_b0 * p.p1);
        let eq28_e367_d_b1: f64 = (eq28_e364_d_b1 * p.p1);
        let eq28_e367_q: f64 = (eq28_e365_q * p.p1);
        let eq28_reactive_node_derivatives: [f64; 13] = [eq28_e367_d_n0, eq28_e367_d_n1, eq28_e367_d_n2, eq28_e367_d_n3, eq28_e367_d_n4, eq28_e367_d_n5, eq28_e367_d_n6, eq28_e367_d_n7, eq28_e367_d_n8, eq28_e367_d_n9, eq28_e367_d_n10, eq28_e367_d_n11, eq28_e367_d_n12];
        let eq28_reactive_branch_derivatives: [f64; 2] = [eq28_e367_d_b0, eq28_e367_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[11]),
            nodes,
            &eq28_reactive_node_derivatives,
            branches,
            &eq28_reactive_branch_derivatives,
            multiplicity,
        );
        let eq35_e406_q: f64 = (nv12 - 0.0);
        let eq35_e407: f64 = (var_taun * (nv12 - 0.0));
        let eq35_e407_d_n0: f64 = (var_taun_dn0 * (nv12 - 0.0));
        let eq35_e407_d_n1: f64 = (var_taun_dn1 * (nv12 - 0.0));
        let eq35_e407_d_n2: f64 = (var_taun_dn2 * (nv12 - 0.0));
        let eq35_e407_d_n3: f64 = (var_taun_dn3 * (nv12 - 0.0));
        let eq35_e407_d_n4: f64 = (var_taun_dn4 * (nv12 - 0.0));
        let eq35_e407_d_n5: f64 = (var_taun_dn5 * (nv12 - 0.0));
        let eq35_e407_d_n6: f64 = (var_taun_dn6 * (nv12 - 0.0));
        let eq35_e407_d_n7: f64 = (var_taun_dn7 * (nv12 - 0.0));
        let eq35_e407_d_n8: f64 = (var_taun_dn8 * (nv12 - 0.0));
        let eq35_e407_d_n9: f64 = (var_taun_dn9 * (nv12 - 0.0));
        let eq35_e407_d_n10: f64 = (var_taun_dn10 * (nv12 - 0.0));
        let eq35_e407_d_n11: f64 = (var_taun_dn11 * (nv12 - 0.0));
        let eq35_e407_d_n12: f64 = ((var_taun_dn12 * (nv12 - 0.0)) + var_taun);
        let eq35_e407_d_b0: f64 = (var_taun_db0 * (nv12 - 0.0));
        let eq35_e407_d_b1: f64 = (var_taun_db1 * (nv12 - 0.0));
        let eq35_e407_q: f64 = (var_taun * eq35_e406_q);
        let eq35_e407_q_d_n0: f64 = (var_taun_dn0 * eq35_e406_q);
        let eq35_e407_q_d_n1: f64 = (var_taun_dn1 * eq35_e406_q);
        let eq35_e407_q_d_n2: f64 = (var_taun_dn2 * eq35_e406_q);
        let eq35_e407_q_d_n3: f64 = (var_taun_dn3 * eq35_e406_q);
        let eq35_e407_q_d_n4: f64 = (var_taun_dn4 * eq35_e406_q);
        let eq35_e407_q_d_n5: f64 = (var_taun_dn5 * eq35_e406_q);
        let eq35_e407_q_d_n6: f64 = (var_taun_dn6 * eq35_e406_q);
        let eq35_e407_q_d_n7: f64 = (var_taun_dn7 * eq35_e406_q);
        let eq35_e407_q_d_n8: f64 = (var_taun_dn8 * eq35_e406_q);
        let eq35_e407_q_d_n9: f64 = (var_taun_dn9 * eq35_e406_q);
        let eq35_e407_q_d_n10: f64 = (var_taun_dn10 * eq35_e406_q);
        let eq35_e407_q_d_n11: f64 = (var_taun_dn11 * eq35_e406_q);
        let eq35_e407_q_d_n12: f64 = ((var_taun_dn12 * eq35_e406_q) + var_taun);
        let eq35_e407_q_d_b0: f64 = (var_taun_db0 * eq35_e406_q);
        let eq35_e407_q_d_b1: f64 = (var_taun_db1 * eq35_e406_q);
        let eq35_reactive_node_derivatives: [f64; 13] = [eq35_e407_q_d_n0, eq35_e407_q_d_n1, eq35_e407_q_d_n2, eq35_e407_q_d_n3, eq35_e407_q_d_n4, eq35_e407_q_d_n5, eq35_e407_q_d_n6, eq35_e407_q_d_n7, eq35_e407_q_d_n8, eq35_e407_q_d_n9, eq35_e407_q_d_n10, eq35_e407_q_d_n11, eq35_e407_q_d_n12];
        let eq35_reactive_branch_derivatives: [f64; 2] = [eq35_e407_q_d_b0, eq35_e407_q_d_b1];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[5]),
            nodes,
            &eq35_reactive_node_derivatives,
            branches,
            &eq35_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
