#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_176(
        p: &Parameters,
        var_cch: f64,
        var_cepi: f64,
        var_cg_fp4s: f64,
        var_ef1: f64,
        var_ef1_dn0: f64,
        var_ef1_dn1: f64,
        var_ef1_dn12: f64,
        var_ef1_dn14: f64,
        var_ef1_dn15: f64,
        var_ef1_dn16: f64,
        var_ef1_dn17: f64,
        var_ef1_dn18: f64,
        var_ef1_dn19: f64,
        var_ef1_dn2: f64,
        var_ef1_dn20: f64,
        var_ef1_dn21: f64,
        var_ef1_dn22: f64,
        var_ef1_dn3: f64,
        var_ef1_dn4: f64,
        var_ef1_dn5: f64,
        var_ef1_dn6: f64,
        var_ef1_dn7: f64,
        var_ef1_dn8: f64,
        var_ef1_dn9: f64,
        var_ef2: f64,
        var_ef2_dn0: f64,
        var_ef2_dn1: f64,
        var_ef2_dn12: f64,
        var_ef2_dn14: f64,
        var_ef2_dn15: f64,
        var_ef2_dn16: f64,
        var_ef2_dn17: f64,
        var_ef2_dn18: f64,
        var_ef2_dn19: f64,
        var_ef2_dn2: f64,
        var_ef2_dn20: f64,
        var_ef2_dn21: f64,
        var_ef2_dn22: f64,
        var_ef2_dn3: f64,
        var_ef2_dn4: f64,
        var_ef2_dn5: f64,
        var_ef2_dn6: f64,
        var_ef2_dn7: f64,
        var_ef2_dn8: f64,
        var_ef2_dn9: f64,
        var_guard504: f64,
        var_guard513: f64,
        var_guard516: f64,
        var_t42: f64,
        var_t42_dn0: f64,
        var_t42_dn1: f64,
        var_t42_dn12: f64,
        var_t42_dn14: f64,
        var_t42_dn15: f64,
        var_t42_dn16: f64,
        var_t42_dn17: f64,
        var_t42_dn18: f64,
        var_t42_dn19: f64,
        var_t42_dn2: f64,
        var_t42_dn20: f64,
        var_t42_dn21: f64,
        var_t42_dn22: f64,
        var_t42_dn3: f64,
        var_t42_dn4: f64,
        var_t42_dn5: f64,
        var_t42_dn6: f64,
        var_t42_dn7: f64,
        var_t42_dn8: f64,
        var_t42_dn9: f64,
        var_t5dg02: f64,
        var_t5dg02_dn0: f64,
        var_t5dg02_dn1: f64,
        var_t5dg02_dn12: f64,
        var_t5dg02_dn14: f64,
        var_t5dg02_dn15: f64,
        var_t5dg02_dn16: f64,
        var_t5dg02_dn17: f64,
        var_t5dg02_dn18: f64,
        var_t5dg02_dn19: f64,
        var_t5dg02_dn2: f64,
        var_t5dg02_dn20: f64,
        var_t5dg02_dn21: f64,
        var_t5dg02_dn22: f64,
        var_t5dg02_dn3: f64,
        var_t5dg02_dn4: f64,
        var_t5dg02_dn5: f64,
        var_t5dg02_dn6: f64,
        var_t5dg02_dn7: f64,
        var_t5dg02_dn8: f64,
        var_t5dg02_dn9: f64,
        var_t5ng02: f64,
        var_t5ng02_dn0: f64,
        var_t5ng02_dn1: f64,
        var_t5ng02_dn12: f64,
        var_t5ng02_dn14: f64,
        var_t5ng02_dn15: f64,
        var_t5ng02_dn16: f64,
        var_t5ng02_dn17: f64,
        var_t5ng02_dn18: f64,
        var_t5ng02_dn19: f64,
        var_t5ng02_dn2: f64,
        var_t5ng02_dn20: f64,
        var_t5ng02_dn21: f64,
        var_t5ng02_dn22: f64,
        var_t5ng02_dn3: f64,
        var_t5ng02_dn4: f64,
        var_t5ng02_dn5: f64,
        var_t5ng02_dn6: f64,
        var_t5ng02_dn7: f64,
        var_t5ng02_dn8: f64,
        var_t5ng02_dn9: f64,
        var_t5ng12: f64,
        var_t5ng12_dn0: f64,
        var_t5ng12_dn1: f64,
        var_t5ng12_dn12: f64,
        var_t5ng12_dn14: f64,
        var_t5ng12_dn15: f64,
        var_t5ng12_dn16: f64,
        var_t5ng12_dn17: f64,
        var_t5ng12_dn18: f64,
        var_t5ng12_dn19: f64,
        var_t5ng12_dn2: f64,
        var_t5ng12_dn20: f64,
        var_t5ng12_dn21: f64,
        var_t5ng12_dn22: f64,
        var_t5ng12_dn3: f64,
        var_t5ng12_dn4: f64,
        var_t5ng12_dn5: f64,
        var_t5ng12_dn6: f64,
        var_t5ng12_dn7: f64,
        var_t5ng12_dn8: f64,
        var_t5ng12_dn9: f64,
        var_tdev: f64,
        var_tdev_dn4: f64,
        var_tg12: f64,
        var_tg12_dn0: f64,
        var_tg12_dn1: f64,
        var_tg12_dn12: f64,
        var_tg12_dn14: f64,
        var_tg12_dn15: f64,
        var_tg12_dn16: f64,
        var_tg12_dn17: f64,
        var_tg12_dn18: f64,
        var_tg12_dn19: f64,
        var_tg12_dn2: f64,
        var_tg12_dn20: f64,
        var_tg12_dn21: f64,
        var_tg12_dn22: f64,
        var_tg12_dn3: f64,
        var_tg12_dn4: f64,
        var_tg12_dn5: f64,
        var_tg12_dn6: f64,
        var_tg12_dn7: f64,
        var_tg12_dn8: f64,
        var_tg12_dn9: f64,
        var_tnom: f64,
        var_vbs: f64,
        var_vbs_dn3: f64,
        var_vbs_dn7: f64,
        var_vbs_dn8: f64,
        var_vg0_fp4s: f64,
        var_vg0_fp4s_dn0: f64,
        var_vg0_fp4s_dn1: f64,
        var_vg0_fp4s_dn12: f64,
        var_vg0_fp4s_dn14: f64,
        var_vg0_fp4s_dn15: f64,
        var_vg0_fp4s_dn16: f64,
        var_vg0_fp4s_dn17: f64,
        var_vg0_fp4s_dn18: f64,
        var_vg0_fp4s_dn19: f64,
        var_vg0_fp4s_dn2: f64,
        var_vg0_fp4s_dn20: f64,
        var_vg0_fp4s_dn21: f64,
        var_vg0_fp4s_dn22: f64,
        var_vg0_fp4s_dn3: f64,
        var_vg0_fp4s_dn4: f64,
        var_vg0_fp4s_dn5: f64,
        var_vg0_fp4s_dn6: f64,
        var_vg0_fp4s_dn7: f64,
        var_vg0_fp4s_dn8: f64,
        var_vg0_fp4s_dn9: f64,
        var_ef3_slot: &mut f64,
        var_ef3_dn0_slot: &mut f64,
        var_ef3_dn1_slot: &mut f64,
        var_ef3_dn12_slot: &mut f64,
        var_ef3_dn14_slot: &mut f64,
        var_ef3_dn15_slot: &mut f64,
        var_ef3_dn16_slot: &mut f64,
        var_ef3_dn17_slot: &mut f64,
        var_ef3_dn18_slot: &mut f64,
        var_ef3_dn19_slot: &mut f64,
        var_ef3_dn2_slot: &mut f64,
        var_ef3_dn20_slot: &mut f64,
        var_ef3_dn21_slot: &mut f64,
        var_ef3_dn22_slot: &mut f64,
        var_ef3_dn3_slot: &mut f64,
        var_ef3_dn4_slot: &mut f64,
        var_ef3_dn5_slot: &mut f64,
        var_ef3_dn6_slot: &mut f64,
        var_ef3_dn7_slot: &mut f64,
        var_ef3_dn8_slot: &mut f64,
        var_ef3_dn9_slot: &mut f64,
        var_mu_eff_slot: &mut f64,
        var_mu_eff_dn0_slot: &mut f64,
        var_mu_eff_dn1_slot: &mut f64,
        var_mu_eff_dn12_slot: &mut f64,
        var_mu_eff_dn14_slot: &mut f64,
        var_mu_eff_dn15_slot: &mut f64,
        var_mu_eff_dn16_slot: &mut f64,
        var_mu_eff_dn17_slot: &mut f64,
        var_mu_eff_dn18_slot: &mut f64,
        var_mu_eff_dn19_slot: &mut f64,
        var_mu_eff_dn2_slot: &mut f64,
        var_mu_eff_dn20_slot: &mut f64,
        var_mu_eff_dn21_slot: &mut f64,
        var_mu_eff_dn22_slot: &mut f64,
        var_mu_eff_dn3_slot: &mut f64,
        var_mu_eff_dn4_slot: &mut f64,
        var_mu_eff_dn5_slot: &mut f64,
        var_mu_eff_dn6_slot: &mut f64,
        var_mu_eff_dn7_slot: &mut f64,
        var_mu_eff_dn8_slot: &mut f64,
        var_mu_eff_dn9_slot: &mut f64,
        var_mulf_tdev_slot: &mut f64,
        var_mulf_tdev_dn0_slot: &mut f64,
        var_mulf_tdev_dn1_slot: &mut f64,
        var_mulf_tdev_dn12_slot: &mut f64,
        var_mulf_tdev_dn14_slot: &mut f64,
        var_mulf_tdev_dn15_slot: &mut f64,
        var_mulf_tdev_dn16_slot: &mut f64,
        var_mulf_tdev_dn17_slot: &mut f64,
        var_mulf_tdev_dn18_slot: &mut f64,
        var_mulf_tdev_dn19_slot: &mut f64,
        var_mulf_tdev_dn2_slot: &mut f64,
        var_mulf_tdev_dn20_slot: &mut f64,
        var_mulf_tdev_dn21_slot: &mut f64,
        var_mulf_tdev_dn22_slot: &mut f64,
        var_mulf_tdev_dn3_slot: &mut f64,
        var_mulf_tdev_dn4_slot: &mut f64,
        var_mulf_tdev_dn5_slot: &mut f64,
        var_mulf_tdev_dn6_slot: &mut f64,
        var_mulf_tdev_dn7_slot: &mut f64,
        var_mulf_tdev_dn8_slot: &mut f64,
        var_mulf_tdev_dn9_slot: &mut f64,
        var_psis_fp4s_slot: &mut f64,
        var_psis_fp4s_dn0_slot: &mut f64,
        var_psis_fp4s_dn1_slot: &mut f64,
        var_psis_fp4s_dn12_slot: &mut f64,
        var_psis_fp4s_dn14_slot: &mut f64,
        var_psis_fp4s_dn15_slot: &mut f64,
        var_psis_fp4s_dn16_slot: &mut f64,
        var_psis_fp4s_dn17_slot: &mut f64,
        var_psis_fp4s_dn18_slot: &mut f64,
        var_psis_fp4s_dn19_slot: &mut f64,
        var_psis_fp4s_dn2_slot: &mut f64,
        var_psis_fp4s_dn20_slot: &mut f64,
        var_psis_fp4s_dn21_slot: &mut f64,
        var_psis_fp4s_dn22_slot: &mut f64,
        var_psis_fp4s_dn3_slot: &mut f64,
        var_psis_fp4s_dn4_slot: &mut f64,
        var_psis_fp4s_dn5_slot: &mut f64,
        var_psis_fp4s_dn6_slot: &mut f64,
        var_psis_fp4s_dn7_slot: &mut f64,
        var_psis_fp4s_dn8_slot: &mut f64,
        var_psis_fp4s_dn9_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn1_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn14_slot: &mut f64,
        var_t0_dn15_slot: &mut f64,
        var_t0_dn16_slot: &mut f64,
        var_t0_dn17_slot: &mut f64,
        var_t0_dn18_slot: &mut f64,
        var_t0_dn19_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn20_slot: &mut f64,
        var_t0_dn21_slot: &mut f64,
        var_t0_dn22_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn1_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn15_slot: &mut f64,
        var_t1_dn16_slot: &mut f64,
        var_t1_dn17_slot: &mut f64,
        var_t1_dn18_slot: &mut f64,
        var_t1_dn19_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn20_slot: &mut f64,
        var_t1_dn21_slot: &mut f64,
        var_t1_dn22_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t52_slot: &mut f64,
        var_t52_dn0_slot: &mut f64,
        var_t52_dn1_slot: &mut f64,
        var_t52_dn12_slot: &mut f64,
        var_t52_dn14_slot: &mut f64,
        var_t52_dn15_slot: &mut f64,
        var_t52_dn16_slot: &mut f64,
        var_t52_dn17_slot: &mut f64,
        var_t52_dn18_slot: &mut f64,
        var_t52_dn19_slot: &mut f64,
        var_t52_dn2_slot: &mut f64,
        var_t52_dn20_slot: &mut f64,
        var_t52_dn21_slot: &mut f64,
        var_t52_dn22_slot: &mut f64,
        var_t52_dn3_slot: &mut f64,
        var_t52_dn4_slot: &mut f64,
        var_t52_dn5_slot: &mut f64,
        var_t52_dn6_slot: &mut f64,
        var_t52_dn7_slot: &mut f64,
        var_t52_dn8_slot: &mut f64,
        var_t52_dn9_slot: &mut f64,
        var_t5dg12_slot: &mut f64,
        var_t5dg12_dn0_slot: &mut f64,
        var_t5dg12_dn1_slot: &mut f64,
        var_t5dg12_dn12_slot: &mut f64,
        var_t5dg12_dn14_slot: &mut f64,
        var_t5dg12_dn15_slot: &mut f64,
        var_t5dg12_dn16_slot: &mut f64,
        var_t5dg12_dn17_slot: &mut f64,
        var_t5dg12_dn18_slot: &mut f64,
        var_t5dg12_dn19_slot: &mut f64,
        var_t5dg12_dn2_slot: &mut f64,
        var_t5dg12_dn20_slot: &mut f64,
        var_t5dg12_dn21_slot: &mut f64,
        var_t5dg12_dn22_slot: &mut f64,
        var_t5dg12_dn3_slot: &mut f64,
        var_t5dg12_dn4_slot: &mut f64,
        var_t5dg12_dn5_slot: &mut f64,
        var_t5dg12_dn6_slot: &mut f64,
        var_t5dg12_dn7_slot: &mut f64,
        var_t5dg12_dn8_slot: &mut f64,
        var_t5dg12_dn9_slot: &mut f64,
        var_vds_fp4s_slot: &mut f64,
        var_vds_fp4s_dn21_slot: &mut f64,
        var_vds_fp4s_dn22_slot: &mut f64,
        var_vdsat_slot: &mut f64,
        var_vdsat_dn0_slot: &mut f64,
        var_vdsat_dn1_slot: &mut f64,
        var_vdsat_dn12_slot: &mut f64,
        var_vdsat_dn14_slot: &mut f64,
        var_vdsat_dn15_slot: &mut f64,
        var_vdsat_dn16_slot: &mut f64,
        var_vdsat_dn17_slot: &mut f64,
        var_vdsat_dn18_slot: &mut f64,
        var_vdsat_dn19_slot: &mut f64,
        var_vdsat_dn2_slot: &mut f64,
        var_vdsat_dn20_slot: &mut f64,
        var_vdsat_dn21_slot: &mut f64,
        var_vdsat_dn22_slot: &mut f64,
        var_vdsat_dn3_slot: &mut f64,
        var_vdsat_dn4_slot: &mut f64,
        var_vdsat_dn5_slot: &mut f64,
        var_vdsat_dn6_slot: &mut f64,
        var_vdsat_dn7_slot: &mut f64,
        var_vdsat_dn8_slot: &mut f64,
        var_vdsat_dn9_slot: &mut f64,
        var_vsat_tdev_slot: &mut f64,
        var_vsat_tdev_dn0_slot: &mut f64,
        var_vsat_tdev_dn1_slot: &mut f64,
        var_vsat_tdev_dn12_slot: &mut f64,
        var_vsat_tdev_dn14_slot: &mut f64,
        var_vsat_tdev_dn15_slot: &mut f64,
        var_vsat_tdev_dn16_slot: &mut f64,
        var_vsat_tdev_dn17_slot: &mut f64,
        var_vsat_tdev_dn18_slot: &mut f64,
        var_vsat_tdev_dn19_slot: &mut f64,
        var_vsat_tdev_dn2_slot: &mut f64,
        var_vsat_tdev_dn20_slot: &mut f64,
        var_vsat_tdev_dn21_slot: &mut f64,
        var_vsat_tdev_dn22_slot: &mut f64,
        var_vsat_tdev_dn3_slot: &mut f64,
        var_vsat_tdev_dn4_slot: &mut f64,
        var_vsat_tdev_dn5_slot: &mut f64,
        var_vsat_tdev_dn6_slot: &mut f64,
        var_vsat_tdev_dn7_slot: &mut f64,
        var_vsat_tdev_dn8_slot: &mut f64,
        var_vsat_tdev_dn9_slot: &mut f64,
    ) {
        let mut var_ef3: f64 = *var_ef3_slot;
        let mut var_ef3_dn0: f64 = *var_ef3_dn0_slot;
        let mut var_ef3_dn1: f64 = *var_ef3_dn1_slot;
        let mut var_ef3_dn12: f64 = *var_ef3_dn12_slot;
        let mut var_ef3_dn14: f64 = *var_ef3_dn14_slot;
        let mut var_ef3_dn15: f64 = *var_ef3_dn15_slot;
        let mut var_ef3_dn16: f64 = *var_ef3_dn16_slot;
        let mut var_ef3_dn17: f64 = *var_ef3_dn17_slot;
        let mut var_ef3_dn18: f64 = *var_ef3_dn18_slot;
        let mut var_ef3_dn19: f64 = *var_ef3_dn19_slot;
        let mut var_ef3_dn2: f64 = *var_ef3_dn2_slot;
        let mut var_ef3_dn20: f64 = *var_ef3_dn20_slot;
        let mut var_ef3_dn21: f64 = *var_ef3_dn21_slot;
        let mut var_ef3_dn22: f64 = *var_ef3_dn22_slot;
        let mut var_ef3_dn3: f64 = *var_ef3_dn3_slot;
        let mut var_ef3_dn4: f64 = *var_ef3_dn4_slot;
        let mut var_ef3_dn5: f64 = *var_ef3_dn5_slot;
        let mut var_ef3_dn6: f64 = *var_ef3_dn6_slot;
        let mut var_ef3_dn7: f64 = *var_ef3_dn7_slot;
        let mut var_ef3_dn8: f64 = *var_ef3_dn8_slot;
        let mut var_ef3_dn9: f64 = *var_ef3_dn9_slot;
        let mut var_mu_eff: f64 = *var_mu_eff_slot;
        let mut var_mu_eff_dn0: f64 = *var_mu_eff_dn0_slot;
        let mut var_mu_eff_dn1: f64 = *var_mu_eff_dn1_slot;
        let mut var_mu_eff_dn12: f64 = *var_mu_eff_dn12_slot;
        let mut var_mu_eff_dn14: f64 = *var_mu_eff_dn14_slot;
        let mut var_mu_eff_dn15: f64 = *var_mu_eff_dn15_slot;
        let mut var_mu_eff_dn16: f64 = *var_mu_eff_dn16_slot;
        let mut var_mu_eff_dn17: f64 = *var_mu_eff_dn17_slot;
        let mut var_mu_eff_dn18: f64 = *var_mu_eff_dn18_slot;
        let mut var_mu_eff_dn19: f64 = *var_mu_eff_dn19_slot;
        let mut var_mu_eff_dn2: f64 = *var_mu_eff_dn2_slot;
        let mut var_mu_eff_dn20: f64 = *var_mu_eff_dn20_slot;
        let mut var_mu_eff_dn21: f64 = *var_mu_eff_dn21_slot;
        let mut var_mu_eff_dn22: f64 = *var_mu_eff_dn22_slot;
        let mut var_mu_eff_dn3: f64 = *var_mu_eff_dn3_slot;
        let mut var_mu_eff_dn4: f64 = *var_mu_eff_dn4_slot;
        let mut var_mu_eff_dn5: f64 = *var_mu_eff_dn5_slot;
        let mut var_mu_eff_dn6: f64 = *var_mu_eff_dn6_slot;
        let mut var_mu_eff_dn7: f64 = *var_mu_eff_dn7_slot;
        let mut var_mu_eff_dn8: f64 = *var_mu_eff_dn8_slot;
        let mut var_mu_eff_dn9: f64 = *var_mu_eff_dn9_slot;
        let mut var_mulf_tdev: f64 = *var_mulf_tdev_slot;
        let mut var_mulf_tdev_dn0: f64 = *var_mulf_tdev_dn0_slot;
        let mut var_mulf_tdev_dn1: f64 = *var_mulf_tdev_dn1_slot;
        let mut var_mulf_tdev_dn12: f64 = *var_mulf_tdev_dn12_slot;
        let mut var_mulf_tdev_dn14: f64 = *var_mulf_tdev_dn14_slot;
        let mut var_mulf_tdev_dn15: f64 = *var_mulf_tdev_dn15_slot;
        let mut var_mulf_tdev_dn16: f64 = *var_mulf_tdev_dn16_slot;
        let mut var_mulf_tdev_dn17: f64 = *var_mulf_tdev_dn17_slot;
        let mut var_mulf_tdev_dn18: f64 = *var_mulf_tdev_dn18_slot;
        let mut var_mulf_tdev_dn19: f64 = *var_mulf_tdev_dn19_slot;
        let mut var_mulf_tdev_dn2: f64 = *var_mulf_tdev_dn2_slot;
        let mut var_mulf_tdev_dn20: f64 = *var_mulf_tdev_dn20_slot;
        let mut var_mulf_tdev_dn21: f64 = *var_mulf_tdev_dn21_slot;
        let mut var_mulf_tdev_dn22: f64 = *var_mulf_tdev_dn22_slot;
        let mut var_mulf_tdev_dn3: f64 = *var_mulf_tdev_dn3_slot;
        let mut var_mulf_tdev_dn4: f64 = *var_mulf_tdev_dn4_slot;
        let mut var_mulf_tdev_dn5: f64 = *var_mulf_tdev_dn5_slot;
        let mut var_mulf_tdev_dn6: f64 = *var_mulf_tdev_dn6_slot;
        let mut var_mulf_tdev_dn7: f64 = *var_mulf_tdev_dn7_slot;
        let mut var_mulf_tdev_dn8: f64 = *var_mulf_tdev_dn8_slot;
        let mut var_mulf_tdev_dn9: f64 = *var_mulf_tdev_dn9_slot;
        let mut var_psis_fp4s: f64 = *var_psis_fp4s_slot;
        let mut var_psis_fp4s_dn0: f64 = *var_psis_fp4s_dn0_slot;
        let mut var_psis_fp4s_dn1: f64 = *var_psis_fp4s_dn1_slot;
        let mut var_psis_fp4s_dn12: f64 = *var_psis_fp4s_dn12_slot;
        let mut var_psis_fp4s_dn14: f64 = *var_psis_fp4s_dn14_slot;
        let mut var_psis_fp4s_dn15: f64 = *var_psis_fp4s_dn15_slot;
        let mut var_psis_fp4s_dn16: f64 = *var_psis_fp4s_dn16_slot;
        let mut var_psis_fp4s_dn17: f64 = *var_psis_fp4s_dn17_slot;
        let mut var_psis_fp4s_dn18: f64 = *var_psis_fp4s_dn18_slot;
        let mut var_psis_fp4s_dn19: f64 = *var_psis_fp4s_dn19_slot;
        let mut var_psis_fp4s_dn2: f64 = *var_psis_fp4s_dn2_slot;
        let mut var_psis_fp4s_dn20: f64 = *var_psis_fp4s_dn20_slot;
        let mut var_psis_fp4s_dn21: f64 = *var_psis_fp4s_dn21_slot;
        let mut var_psis_fp4s_dn22: f64 = *var_psis_fp4s_dn22_slot;
        let mut var_psis_fp4s_dn3: f64 = *var_psis_fp4s_dn3_slot;
        let mut var_psis_fp4s_dn4: f64 = *var_psis_fp4s_dn4_slot;
        let mut var_psis_fp4s_dn5: f64 = *var_psis_fp4s_dn5_slot;
        let mut var_psis_fp4s_dn6: f64 = *var_psis_fp4s_dn6_slot;
        let mut var_psis_fp4s_dn7: f64 = *var_psis_fp4s_dn7_slot;
        let mut var_psis_fp4s_dn8: f64 = *var_psis_fp4s_dn8_slot;
        let mut var_psis_fp4s_dn9: f64 = *var_psis_fp4s_dn9_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn1: f64 = *var_t0_dn1_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn14: f64 = *var_t0_dn14_slot;
        let mut var_t0_dn15: f64 = *var_t0_dn15_slot;
        let mut var_t0_dn16: f64 = *var_t0_dn16_slot;
        let mut var_t0_dn17: f64 = *var_t0_dn17_slot;
        let mut var_t0_dn18: f64 = *var_t0_dn18_slot;
        let mut var_t0_dn19: f64 = *var_t0_dn19_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn20: f64 = *var_t0_dn20_slot;
        let mut var_t0_dn21: f64 = *var_t0_dn21_slot;
        let mut var_t0_dn22: f64 = *var_t0_dn22_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn1: f64 = *var_t1_dn1_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn15: f64 = *var_t1_dn15_slot;
        let mut var_t1_dn16: f64 = *var_t1_dn16_slot;
        let mut var_t1_dn17: f64 = *var_t1_dn17_slot;
        let mut var_t1_dn18: f64 = *var_t1_dn18_slot;
        let mut var_t1_dn19: f64 = *var_t1_dn19_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn20: f64 = *var_t1_dn20_slot;
        let mut var_t1_dn21: f64 = *var_t1_dn21_slot;
        let mut var_t1_dn22: f64 = *var_t1_dn22_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t52: f64 = *var_t52_slot;
        let mut var_t52_dn0: f64 = *var_t52_dn0_slot;
        let mut var_t52_dn1: f64 = *var_t52_dn1_slot;
        let mut var_t52_dn12: f64 = *var_t52_dn12_slot;
        let mut var_t52_dn14: f64 = *var_t52_dn14_slot;
        let mut var_t52_dn15: f64 = *var_t52_dn15_slot;
        let mut var_t52_dn16: f64 = *var_t52_dn16_slot;
        let mut var_t52_dn17: f64 = *var_t52_dn17_slot;
        let mut var_t52_dn18: f64 = *var_t52_dn18_slot;
        let mut var_t52_dn19: f64 = *var_t52_dn19_slot;
        let mut var_t52_dn2: f64 = *var_t52_dn2_slot;
        let mut var_t52_dn20: f64 = *var_t52_dn20_slot;
        let mut var_t52_dn21: f64 = *var_t52_dn21_slot;
        let mut var_t52_dn22: f64 = *var_t52_dn22_slot;
        let mut var_t52_dn3: f64 = *var_t52_dn3_slot;
        let mut var_t52_dn4: f64 = *var_t52_dn4_slot;
        let mut var_t52_dn5: f64 = *var_t52_dn5_slot;
        let mut var_t52_dn6: f64 = *var_t52_dn6_slot;
        let mut var_t52_dn7: f64 = *var_t52_dn7_slot;
        let mut var_t52_dn8: f64 = *var_t52_dn8_slot;
        let mut var_t52_dn9: f64 = *var_t52_dn9_slot;
        let mut var_t5dg12: f64 = *var_t5dg12_slot;
        let mut var_t5dg12_dn0: f64 = *var_t5dg12_dn0_slot;
        let mut var_t5dg12_dn1: f64 = *var_t5dg12_dn1_slot;
        let mut var_t5dg12_dn12: f64 = *var_t5dg12_dn12_slot;
        let mut var_t5dg12_dn14: f64 = *var_t5dg12_dn14_slot;
        let mut var_t5dg12_dn15: f64 = *var_t5dg12_dn15_slot;
        let mut var_t5dg12_dn16: f64 = *var_t5dg12_dn16_slot;
        let mut var_t5dg12_dn17: f64 = *var_t5dg12_dn17_slot;
        let mut var_t5dg12_dn18: f64 = *var_t5dg12_dn18_slot;
        let mut var_t5dg12_dn19: f64 = *var_t5dg12_dn19_slot;
        let mut var_t5dg12_dn2: f64 = *var_t5dg12_dn2_slot;
        let mut var_t5dg12_dn20: f64 = *var_t5dg12_dn20_slot;
        let mut var_t5dg12_dn21: f64 = *var_t5dg12_dn21_slot;
        let mut var_t5dg12_dn22: f64 = *var_t5dg12_dn22_slot;
        let mut var_t5dg12_dn3: f64 = *var_t5dg12_dn3_slot;
        let mut var_t5dg12_dn4: f64 = *var_t5dg12_dn4_slot;
        let mut var_t5dg12_dn5: f64 = *var_t5dg12_dn5_slot;
        let mut var_t5dg12_dn6: f64 = *var_t5dg12_dn6_slot;
        let mut var_t5dg12_dn7: f64 = *var_t5dg12_dn7_slot;
        let mut var_t5dg12_dn8: f64 = *var_t5dg12_dn8_slot;
        let mut var_t5dg12_dn9: f64 = *var_t5dg12_dn9_slot;
        let mut var_vds_fp4s: f64 = *var_vds_fp4s_slot;
        let mut var_vds_fp4s_dn21: f64 = *var_vds_fp4s_dn21_slot;
        let mut var_vds_fp4s_dn22: f64 = *var_vds_fp4s_dn22_slot;
        let mut var_vdsat: f64 = *var_vdsat_slot;
        let mut var_vdsat_dn0: f64 = *var_vdsat_dn0_slot;
        let mut var_vdsat_dn1: f64 = *var_vdsat_dn1_slot;
        let mut var_vdsat_dn12: f64 = *var_vdsat_dn12_slot;
        let mut var_vdsat_dn14: f64 = *var_vdsat_dn14_slot;
        let mut var_vdsat_dn15: f64 = *var_vdsat_dn15_slot;
        let mut var_vdsat_dn16: f64 = *var_vdsat_dn16_slot;
        let mut var_vdsat_dn17: f64 = *var_vdsat_dn17_slot;
        let mut var_vdsat_dn18: f64 = *var_vdsat_dn18_slot;
        let mut var_vdsat_dn19: f64 = *var_vdsat_dn19_slot;
        let mut var_vdsat_dn2: f64 = *var_vdsat_dn2_slot;
        let mut var_vdsat_dn20: f64 = *var_vdsat_dn20_slot;
        let mut var_vdsat_dn21: f64 = *var_vdsat_dn21_slot;
        let mut var_vdsat_dn22: f64 = *var_vdsat_dn22_slot;
        let mut var_vdsat_dn3: f64 = *var_vdsat_dn3_slot;
        let mut var_vdsat_dn4: f64 = *var_vdsat_dn4_slot;
        let mut var_vdsat_dn5: f64 = *var_vdsat_dn5_slot;
        let mut var_vdsat_dn6: f64 = *var_vdsat_dn6_slot;
        let mut var_vdsat_dn7: f64 = *var_vdsat_dn7_slot;
        let mut var_vdsat_dn8: f64 = *var_vdsat_dn8_slot;
        let mut var_vdsat_dn9: f64 = *var_vdsat_dn9_slot;
        let mut var_vsat_tdev: f64 = *var_vsat_tdev_slot;
        let mut var_vsat_tdev_dn0: f64 = *var_vsat_tdev_dn0_slot;
        let mut var_vsat_tdev_dn1: f64 = *var_vsat_tdev_dn1_slot;
        let mut var_vsat_tdev_dn12: f64 = *var_vsat_tdev_dn12_slot;
        let mut var_vsat_tdev_dn14: f64 = *var_vsat_tdev_dn14_slot;
        let mut var_vsat_tdev_dn15: f64 = *var_vsat_tdev_dn15_slot;
        let mut var_vsat_tdev_dn16: f64 = *var_vsat_tdev_dn16_slot;
        let mut var_vsat_tdev_dn17: f64 = *var_vsat_tdev_dn17_slot;
        let mut var_vsat_tdev_dn18: f64 = *var_vsat_tdev_dn18_slot;
        let mut var_vsat_tdev_dn19: f64 = *var_vsat_tdev_dn19_slot;
        let mut var_vsat_tdev_dn2: f64 = *var_vsat_tdev_dn2_slot;
        let mut var_vsat_tdev_dn20: f64 = *var_vsat_tdev_dn20_slot;
        let mut var_vsat_tdev_dn21: f64 = *var_vsat_tdev_dn21_slot;
        let mut var_vsat_tdev_dn22: f64 = *var_vsat_tdev_dn22_slot;
        let mut var_vsat_tdev_dn3: f64 = *var_vsat_tdev_dn3_slot;
        let mut var_vsat_tdev_dn4: f64 = *var_vsat_tdev_dn4_slot;
        let mut var_vsat_tdev_dn5: f64 = *var_vsat_tdev_dn5_slot;
        let mut var_vsat_tdev_dn6: f64 = *var_vsat_tdev_dn6_slot;
        let mut var_vsat_tdev_dn7: f64 = *var_vsat_tdev_dn7_slot;
        let mut var_vsat_tdev_dn8: f64 = *var_vsat_tdev_dn8_slot;
        let mut var_vsat_tdev_dn9: f64 = *var_vsat_tdev_dn9_slot;

        let (assign30010_e47356, assign30010_e47356_d_n0, assign30010_e47356_d_n1, assign30010_e47356_d_n2, assign30010_e47356_d_n3, assign30010_e47356_d_n4, assign30010_e47356_d_n5, assign30010_e47356_d_n6, assign30010_e47356_d_n7, assign30010_e47356_d_n8, assign30010_e47356_d_n9, assign30010_e47356_d_n12, assign30010_e47356_d_n14, assign30010_e47356_d_n15, assign30010_e47356_d_n16, assign30010_e47356_d_n17, assign30010_e47356_d_n18, assign30010_e47356_d_n19, assign30010_e47356_d_n20, assign30010_e47356_d_n21, assign30010_e47356_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard516 != 0.0)) {
        let assign30010_e47353: f64 = { let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign30010_e47354: f64 = (1.0 + assign30010_e47353);
        (assign30010_e47354, ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn0), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn1), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn2), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn3), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn4), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn5), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn6), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn7), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn8), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn9), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn12), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn14), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn15), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn16), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn17), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn18), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn19), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn20), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn21), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn22),)
    } else {
        (var_t5dg12, var_t5dg12_dn0, var_t5dg12_dn1, var_t5dg12_dn2, var_t5dg12_dn3, var_t5dg12_dn4, var_t5dg12_dn5, var_t5dg12_dn6, var_t5dg12_dn7, var_t5dg12_dn8, var_t5dg12_dn9, var_t5dg12_dn12, var_t5dg12_dn14, var_t5dg12_dn15, var_t5dg12_dn16, var_t5dg12_dn17, var_t5dg12_dn18, var_t5dg12_dn19, var_t5dg12_dn20, var_t5dg12_dn21, var_t5dg12_dn22,)
    }
};
        var_t5dg12 = assign30010_e47356;
        var_t5dg12_dn0 = assign30010_e47356_d_n0;
        var_t5dg12_dn1 = assign30010_e47356_d_n1;
        var_t5dg12_dn2 = assign30010_e47356_d_n2;
        var_t5dg12_dn3 = assign30010_e47356_d_n3;
        var_t5dg12_dn4 = assign30010_e47356_d_n4;
        var_t5dg12_dn5 = assign30010_e47356_d_n5;
        var_t5dg12_dn6 = assign30010_e47356_d_n6;
        var_t5dg12_dn7 = assign30010_e47356_d_n7;
        var_t5dg12_dn8 = assign30010_e47356_d_n8;
        var_t5dg12_dn9 = assign30010_e47356_d_n9;
        var_t5dg12_dn12 = assign30010_e47356_d_n12;
        var_t5dg12_dn14 = assign30010_e47356_d_n14;
        var_t5dg12_dn15 = assign30010_e47356_d_n15;
        var_t5dg12_dn16 = assign30010_e47356_d_n16;
        var_t5dg12_dn17 = assign30010_e47356_d_n17;
        var_t5dg12_dn18 = assign30010_e47356_d_n18;
        var_t5dg12_dn19 = assign30010_e47356_d_n19;
        var_t5dg12_dn20 = assign30010_e47356_d_n20;
        var_t5dg12_dn21 = assign30010_e47356_d_n21;
        var_t5dg12_dn22 = assign30010_e47356_d_n22;

        let (assign30020_e47376, assign30020_e47376_d_n0, assign30020_e47376_d_n1, assign30020_e47376_d_n2, assign30020_e47376_d_n3, assign30020_e47376_d_n4, assign30020_e47376_d_n5, assign30020_e47376_d_n6, assign30020_e47376_d_n7, assign30020_e47376_d_n8, assign30020_e47376_d_n9, assign30020_e47376_d_n12, assign30020_e47376_d_n14, assign30020_e47376_d_n15, assign30020_e47376_d_n16, assign30020_e47376_d_n17, assign30020_e47376_d_n18, assign30020_e47376_d_n19, assign30020_e47376_d_n20, assign30020_e47376_d_n21, assign30020_e47376_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard516 != 0.0)) {
        let assign30020_e47364: f64 = (-1.0);
        let assign30020_e47366: f64 = (assign30020_e47364 * var_cch);
        let assign30020_e47369: f64 = (var_t5ng02 / var_t5dg02);
        let assign30020_e47370: f64 = (assign30020_e47366 - assign30020_e47369);
        let assign30020_e47373: f64 = (var_t5ng12 / var_t5dg12);
        let assign30020_e47374: f64 = (assign30020_e47370 - assign30020_e47373);
        (assign30020_e47374, ((-(((var_t5ng02_dn0 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn0)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn0 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn0)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn1 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn1)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn1 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn1)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn2 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn2)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn2 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn2)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn3 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn3)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn3 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn3)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn4 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn4)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn4 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn4)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn5 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn5)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn5 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn5)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn6 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn6)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn6 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn6)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn7 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn7)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn7 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn7)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn8 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn8)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn8 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn8)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn9 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn9)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn9 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn9)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn12 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn12)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn12 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn12)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn14 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn14)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn14 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn14)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn15 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn15)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn15 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn15)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn16 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn16)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn16 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn16)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn17 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn17)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn17 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn17)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn18 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn18)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn18 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn18)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn19 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn19)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn19 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn19)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn20 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn20)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn20 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn20)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn21 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn21)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn21 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn21)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn22 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn22)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn22 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn22)) / (var_t5dg12 * var_t5dg12))),)
    } else {
        (var_t52, var_t52_dn0, var_t52_dn1, var_t52_dn2, var_t52_dn3, var_t52_dn4, var_t52_dn5, var_t52_dn6, var_t52_dn7, var_t52_dn8, var_t52_dn9, var_t52_dn12, var_t52_dn14, var_t52_dn15, var_t52_dn16, var_t52_dn17, var_t52_dn18, var_t52_dn19, var_t52_dn20, var_t52_dn21, var_t52_dn22,)
    }
};
        var_t52 = assign30020_e47376;
        var_t52_dn0 = assign30020_e47376_d_n0;
        var_t52_dn1 = assign30020_e47376_d_n1;
        var_t52_dn2 = assign30020_e47376_d_n2;
        var_t52_dn3 = assign30020_e47376_d_n3;
        var_t52_dn4 = assign30020_e47376_d_n4;
        var_t52_dn5 = assign30020_e47376_d_n5;
        var_t52_dn6 = assign30020_e47376_d_n6;
        var_t52_dn7 = assign30020_e47376_d_n7;
        var_t52_dn8 = assign30020_e47376_d_n8;
        var_t52_dn9 = assign30020_e47376_d_n9;
        var_t52_dn12 = assign30020_e47376_d_n12;
        var_t52_dn14 = assign30020_e47376_d_n14;
        var_t52_dn15 = assign30020_e47376_d_n15;
        var_t52_dn16 = assign30020_e47376_d_n16;
        var_t52_dn17 = assign30020_e47376_d_n17;
        var_t52_dn18 = assign30020_e47376_d_n18;
        var_t52_dn19 = assign30020_e47376_d_n19;
        var_t52_dn20 = assign30020_e47376_d_n20;
        var_t52_dn21 = assign30020_e47376_d_n21;
        var_t52_dn22 = assign30020_e47376_d_n22;

        let (assign30030_e47389, assign30030_e47389_d_n0, assign30030_e47389_d_n1, assign30030_e47389_d_n2, assign30030_e47389_d_n3, assign30030_e47389_d_n4, assign30030_e47389_d_n5, assign30030_e47389_d_n6, assign30030_e47389_d_n7, assign30030_e47389_d_n8, assign30030_e47389_d_n9, assign30030_e47389_d_n12, assign30030_e47389_d_n14, assign30030_e47389_d_n15, assign30030_e47389_d_n16, assign30030_e47389_d_n17, assign30030_e47389_d_n18, assign30030_e47389_d_n19, assign30030_e47389_d_n20, assign30030_e47389_d_n21, assign30030_e47389_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard516 != 0.0)) {
        let assign30030_e47386: f64 = (var_t42 / var_t52);
        let assign30030_e47387: f64 = (var_ef2 - assign30030_e47386);
        (assign30030_e47387, (var_ef2_dn0 - (((var_t42_dn0 * var_t52) - (var_t42 * var_t52_dn0)) / (var_t52 * var_t52))), (var_ef2_dn1 - (((var_t42_dn1 * var_t52) - (var_t42 * var_t52_dn1)) / (var_t52 * var_t52))), (var_ef2_dn2 - (((var_t42_dn2 * var_t52) - (var_t42 * var_t52_dn2)) / (var_t52 * var_t52))), (var_ef2_dn3 - (((var_t42_dn3 * var_t52) - (var_t42 * var_t52_dn3)) / (var_t52 * var_t52))), (var_ef2_dn4 - (((var_t42_dn4 * var_t52) - (var_t42 * var_t52_dn4)) / (var_t52 * var_t52))), (var_ef2_dn5 - (((var_t42_dn5 * var_t52) - (var_t42 * var_t52_dn5)) / (var_t52 * var_t52))), (var_ef2_dn6 - (((var_t42_dn6 * var_t52) - (var_t42 * var_t52_dn6)) / (var_t52 * var_t52))), (var_ef2_dn7 - (((var_t42_dn7 * var_t52) - (var_t42 * var_t52_dn7)) / (var_t52 * var_t52))), (var_ef2_dn8 - (((var_t42_dn8 * var_t52) - (var_t42 * var_t52_dn8)) / (var_t52 * var_t52))), (var_ef2_dn9 - (((var_t42_dn9 * var_t52) - (var_t42 * var_t52_dn9)) / (var_t52 * var_t52))), (var_ef2_dn12 - (((var_t42_dn12 * var_t52) - (var_t42 * var_t52_dn12)) / (var_t52 * var_t52))), (var_ef2_dn14 - (((var_t42_dn14 * var_t52) - (var_t42 * var_t52_dn14)) / (var_t52 * var_t52))), (var_ef2_dn15 - (((var_t42_dn15 * var_t52) - (var_t42 * var_t52_dn15)) / (var_t52 * var_t52))), (var_ef2_dn16 - (((var_t42_dn16 * var_t52) - (var_t42 * var_t52_dn16)) / (var_t52 * var_t52))), (var_ef2_dn17 - (((var_t42_dn17 * var_t52) - (var_t42 * var_t52_dn17)) / (var_t52 * var_t52))), (var_ef2_dn18 - (((var_t42_dn18 * var_t52) - (var_t42 * var_t52_dn18)) / (var_t52 * var_t52))), (var_ef2_dn19 - (((var_t42_dn19 * var_t52) - (var_t42 * var_t52_dn19)) / (var_t52 * var_t52))), (var_ef2_dn20 - (((var_t42_dn20 * var_t52) - (var_t42 * var_t52_dn20)) / (var_t52 * var_t52))), (var_ef2_dn21 - (((var_t42_dn21 * var_t52) - (var_t42 * var_t52_dn21)) / (var_t52 * var_t52))), (var_ef2_dn22 - (((var_t42_dn22 * var_t52) - (var_t42 * var_t52_dn22)) / (var_t52 * var_t52))),)
    } else {
        (var_ef3, var_ef3_dn0, var_ef3_dn1, var_ef3_dn2, var_ef3_dn3, var_ef3_dn4, var_ef3_dn5, var_ef3_dn6, var_ef3_dn7, var_ef3_dn8, var_ef3_dn9, var_ef3_dn12, var_ef3_dn14, var_ef3_dn15, var_ef3_dn16, var_ef3_dn17, var_ef3_dn18, var_ef3_dn19, var_ef3_dn20, var_ef3_dn21, var_ef3_dn22,)
    }
};
        var_ef3 = assign30030_e47389;
        var_ef3_dn0 = assign30030_e47389_d_n0;
        var_ef3_dn1 = assign30030_e47389_d_n1;
        var_ef3_dn2 = assign30030_e47389_d_n2;
        var_ef3_dn3 = assign30030_e47389_d_n3;
        var_ef3_dn4 = assign30030_e47389_d_n4;
        var_ef3_dn5 = assign30030_e47389_d_n5;
        var_ef3_dn6 = assign30030_e47389_d_n6;
        var_ef3_dn7 = assign30030_e47389_d_n7;
        var_ef3_dn8 = assign30030_e47389_d_n8;
        var_ef3_dn9 = assign30030_e47389_d_n9;
        var_ef3_dn12 = assign30030_e47389_d_n12;
        var_ef3_dn14 = assign30030_e47389_d_n14;
        var_ef3_dn15 = assign30030_e47389_d_n15;
        var_ef3_dn16 = assign30030_e47389_d_n16;
        var_ef3_dn17 = assign30030_e47389_d_n17;
        var_ef3_dn18 = assign30030_e47389_d_n18;
        var_ef3_dn19 = assign30030_e47389_d_n19;
        var_ef3_dn20 = assign30030_e47389_d_n20;
        var_ef3_dn21 = assign30030_e47389_d_n21;
        var_ef3_dn22 = assign30030_e47389_d_n22;

        let (assign30040_e47398, assign30040_e47398_d_n0, assign30040_e47398_d_n1, assign30040_e47398_d_n2, assign30040_e47398_d_n3, assign30040_e47398_d_n4, assign30040_e47398_d_n5, assign30040_e47398_d_n6, assign30040_e47398_d_n7, assign30040_e47398_d_n8, assign30040_e47398_d_n9, assign30040_e47398_d_n12, assign30040_e47398_d_n14, assign30040_e47398_d_n15, assign30040_e47398_d_n16, assign30040_e47398_d_n17, assign30040_e47398_d_n18, assign30040_e47398_d_n19, assign30040_e47398_d_n20, assign30040_e47398_d_n21, assign30040_e47398_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard516 != 0.0)) {
        (var_ef3, var_ef3_dn0, var_ef3_dn1, var_ef3_dn2, var_ef3_dn3, var_ef3_dn4, var_ef3_dn5, var_ef3_dn6, var_ef3_dn7, var_ef3_dn8, var_ef3_dn9, var_ef3_dn12, var_ef3_dn14, var_ef3_dn15, var_ef3_dn16, var_ef3_dn17, var_ef3_dn18, var_ef3_dn19, var_ef3_dn20, var_ef3_dn21, var_ef3_dn22,)
    } else {
        (var_psis_fp4s, var_psis_fp4s_dn0, var_psis_fp4s_dn1, var_psis_fp4s_dn2, var_psis_fp4s_dn3, var_psis_fp4s_dn4, var_psis_fp4s_dn5, var_psis_fp4s_dn6, var_psis_fp4s_dn7, var_psis_fp4s_dn8, var_psis_fp4s_dn9, var_psis_fp4s_dn12, var_psis_fp4s_dn14, var_psis_fp4s_dn15, var_psis_fp4s_dn16, var_psis_fp4s_dn17, var_psis_fp4s_dn18, var_psis_fp4s_dn19, var_psis_fp4s_dn20, var_psis_fp4s_dn21, var_psis_fp4s_dn22,)
    }
};
        var_psis_fp4s = assign30040_e47398;
        var_psis_fp4s_dn0 = assign30040_e47398_d_n0;
        var_psis_fp4s_dn1 = assign30040_e47398_d_n1;
        var_psis_fp4s_dn2 = assign30040_e47398_d_n2;
        var_psis_fp4s_dn3 = assign30040_e47398_d_n3;
        var_psis_fp4s_dn4 = assign30040_e47398_d_n4;
        var_psis_fp4s_dn5 = assign30040_e47398_d_n5;
        var_psis_fp4s_dn6 = assign30040_e47398_d_n6;
        var_psis_fp4s_dn7 = assign30040_e47398_d_n7;
        var_psis_fp4s_dn8 = assign30040_e47398_d_n8;
        var_psis_fp4s_dn9 = assign30040_e47398_d_n9;
        var_psis_fp4s_dn12 = assign30040_e47398_d_n12;
        var_psis_fp4s_dn14 = assign30040_e47398_d_n14;
        var_psis_fp4s_dn15 = assign30040_e47398_d_n15;
        var_psis_fp4s_dn16 = assign30040_e47398_d_n16;
        var_psis_fp4s_dn17 = assign30040_e47398_d_n17;
        var_psis_fp4s_dn18 = assign30040_e47398_d_n18;
        var_psis_fp4s_dn19 = assign30040_e47398_d_n19;
        var_psis_fp4s_dn20 = assign30040_e47398_d_n20;
        var_psis_fp4s_dn21 = assign30040_e47398_d_n21;
        var_psis_fp4s_dn22 = assign30040_e47398_d_n22;

        let (assign30050_e47408, assign30050_e47408_d_n0, assign30050_e47408_d_n1, assign30050_e47408_d_n2, assign30050_e47408_d_n3, assign30050_e47408_d_n4, assign30050_e47408_d_n5, assign30050_e47408_d_n6, assign30050_e47408_d_n7, assign30050_e47408_d_n8, assign30050_e47408_d_n9, assign30050_e47408_d_n12, assign30050_e47408_d_n14, assign30050_e47408_d_n15, assign30050_e47408_d_n16, assign30050_e47408_d_n17, assign30050_e47408_d_n18, assign30050_e47408_d_n19, assign30050_e47408_d_n20, assign30050_e47408_d_n21, assign30050_e47408_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard516 == 0.0)) {
        (var_ef1, var_ef1_dn0, var_ef1_dn1, var_ef1_dn2, var_ef1_dn3, var_ef1_dn4, var_ef1_dn5, var_ef1_dn6, var_ef1_dn7, var_ef1_dn8, var_ef1_dn9, var_ef1_dn12, var_ef1_dn14, var_ef1_dn15, var_ef1_dn16, var_ef1_dn17, var_ef1_dn18, var_ef1_dn19, var_ef1_dn20, var_ef1_dn21, var_ef1_dn22,)
    } else {
        (var_psis_fp4s, var_psis_fp4s_dn0, var_psis_fp4s_dn1, var_psis_fp4s_dn2, var_psis_fp4s_dn3, var_psis_fp4s_dn4, var_psis_fp4s_dn5, var_psis_fp4s_dn6, var_psis_fp4s_dn7, var_psis_fp4s_dn8, var_psis_fp4s_dn9, var_psis_fp4s_dn12, var_psis_fp4s_dn14, var_psis_fp4s_dn15, var_psis_fp4s_dn16, var_psis_fp4s_dn17, var_psis_fp4s_dn18, var_psis_fp4s_dn19, var_psis_fp4s_dn20, var_psis_fp4s_dn21, var_psis_fp4s_dn22,)
    }
};
        var_psis_fp4s = assign30050_e47408;
        var_psis_fp4s_dn0 = assign30050_e47408_d_n0;
        var_psis_fp4s_dn1 = assign30050_e47408_d_n1;
        var_psis_fp4s_dn2 = assign30050_e47408_d_n2;
        var_psis_fp4s_dn3 = assign30050_e47408_d_n3;
        var_psis_fp4s_dn4 = assign30050_e47408_d_n4;
        var_psis_fp4s_dn5 = assign30050_e47408_d_n5;
        var_psis_fp4s_dn6 = assign30050_e47408_d_n6;
        var_psis_fp4s_dn7 = assign30050_e47408_d_n7;
        var_psis_fp4s_dn8 = assign30050_e47408_d_n8;
        var_psis_fp4s_dn9 = assign30050_e47408_d_n9;
        var_psis_fp4s_dn12 = assign30050_e47408_d_n12;
        var_psis_fp4s_dn14 = assign30050_e47408_d_n14;
        var_psis_fp4s_dn15 = assign30050_e47408_d_n15;
        var_psis_fp4s_dn16 = assign30050_e47408_d_n16;
        var_psis_fp4s_dn17 = assign30050_e47408_d_n17;
        var_psis_fp4s_dn18 = assign30050_e47408_d_n18;
        var_psis_fp4s_dn19 = assign30050_e47408_d_n19;
        var_psis_fp4s_dn20 = assign30050_e47408_d_n20;
        var_psis_fp4s_dn21 = assign30050_e47408_d_n21;
        var_psis_fp4s_dn22 = assign30050_e47408_d_n22;

        let (assign30060_e47415, assign30060_e47415_d_n21, assign30060_e47415_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_vds_fp4s, var_vds_fp4s_dn21, var_vds_fp4s_dn22,)
    }
};
        var_vds_fp4s = assign30060_e47415;
        var_vds_fp4s_dn21 = assign30060_e47415_d_n21;
        var_vds_fp4s_dn22 = assign30060_e47415_d_n22;

        let (assign30070_e47428, assign30070_e47428_d_n0, assign30070_e47428_d_n1, assign30070_e47428_d_n2, assign30070_e47428_d_n3, assign30070_e47428_d_n4, assign30070_e47428_d_n5, assign30070_e47428_d_n6, assign30070_e47428_d_n7, assign30070_e47428_d_n8, assign30070_e47428_d_n9, assign30070_e47428_d_n12, assign30070_e47428_d_n14, assign30070_e47428_d_n15, assign30070_e47428_d_n16, assign30070_e47428_d_n17, assign30070_e47428_d_n18, assign30070_e47428_d_n19, assign30070_e47428_d_n20, assign30070_e47428_d_n21, assign30070_e47428_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30070_e47423: f64 = (var_tdev / var_tnom);
        let assign30070_e47425: f64 = (assign30070_e47423).powf(p.p20);
        let assign30070_e47426: f64 = (p.p202 * assign30070_e47425);
        (assign30070_e47426, 0.0, 0.0, 0.0, 0.0, (p.p202 * if 0.0 == 0.0 && ((p.p20) as f64).is_finite() && ((p.p20) as f64).fract() == 0.0 { if p.p20 == 0.0 { 0.0 } else { (p.p20 * ((assign30070_e47423).powf(p.p20 - 1.0) * (var_tdev_dn4 / var_tnom))) } } else { (assign30070_e47425 * (p.p20 * ((var_tdev_dn4 / var_tnom) / assign30070_e47423))) }), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_mulf_tdev, var_mulf_tdev_dn0, var_mulf_tdev_dn1, var_mulf_tdev_dn2, var_mulf_tdev_dn3, var_mulf_tdev_dn4, var_mulf_tdev_dn5, var_mulf_tdev_dn6, var_mulf_tdev_dn7, var_mulf_tdev_dn8, var_mulf_tdev_dn9, var_mulf_tdev_dn12, var_mulf_tdev_dn14, var_mulf_tdev_dn15, var_mulf_tdev_dn16, var_mulf_tdev_dn17, var_mulf_tdev_dn18, var_mulf_tdev_dn19, var_mulf_tdev_dn20, var_mulf_tdev_dn21, var_mulf_tdev_dn22,)
    }
};
        var_mulf_tdev = assign30070_e47428;
        var_mulf_tdev_dn0 = assign30070_e47428_d_n0;
        var_mulf_tdev_dn1 = assign30070_e47428_d_n1;
        var_mulf_tdev_dn2 = assign30070_e47428_d_n2;
        var_mulf_tdev_dn3 = assign30070_e47428_d_n3;
        var_mulf_tdev_dn4 = assign30070_e47428_d_n4;
        var_mulf_tdev_dn5 = assign30070_e47428_d_n5;
        var_mulf_tdev_dn6 = assign30070_e47428_d_n6;
        var_mulf_tdev_dn7 = assign30070_e47428_d_n7;
        var_mulf_tdev_dn8 = assign30070_e47428_d_n8;
        var_mulf_tdev_dn9 = assign30070_e47428_d_n9;
        var_mulf_tdev_dn12 = assign30070_e47428_d_n12;
        var_mulf_tdev_dn14 = assign30070_e47428_d_n14;
        var_mulf_tdev_dn15 = assign30070_e47428_d_n15;
        var_mulf_tdev_dn16 = assign30070_e47428_d_n16;
        var_mulf_tdev_dn17 = assign30070_e47428_d_n17;
        var_mulf_tdev_dn18 = assign30070_e47428_d_n18;
        var_mulf_tdev_dn19 = assign30070_e47428_d_n19;
        var_mulf_tdev_dn20 = assign30070_e47428_d_n20;
        var_mulf_tdev_dn21 = assign30070_e47428_d_n21;
        var_mulf_tdev_dn22 = assign30070_e47428_d_n22;

        let (assign30080_e47441, assign30080_e47441_d_n0, assign30080_e47441_d_n1, assign30080_e47441_d_n2, assign30080_e47441_d_n3, assign30080_e47441_d_n4, assign30080_e47441_d_n5, assign30080_e47441_d_n6, assign30080_e47441_d_n7, assign30080_e47441_d_n8, assign30080_e47441_d_n9, assign30080_e47441_d_n12, assign30080_e47441_d_n14, assign30080_e47441_d_n15, assign30080_e47441_d_n16, assign30080_e47441_d_n17, assign30080_e47441_d_n18, assign30080_e47441_d_n19, assign30080_e47441_d_n20, assign30080_e47441_d_n21, assign30080_e47441_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30080_e47436: f64 = (var_tdev / var_tnom);
        let assign30080_e47438: f64 = (assign30080_e47436).powf(p.p19);
        let assign30080_e47439: f64 = (p.p203 * assign30080_e47438);
        (assign30080_e47439, 0.0, 0.0, 0.0, 0.0, (p.p203 * if 0.0 == 0.0 && ((p.p19) as f64).is_finite() && ((p.p19) as f64).fract() == 0.0 { if p.p19 == 0.0 { 0.0 } else { (p.p19 * ((assign30080_e47436).powf(p.p19 - 1.0) * (var_tdev_dn4 / var_tnom))) } } else { (assign30080_e47438 * (p.p19 * ((var_tdev_dn4 / var_tnom) / assign30080_e47436))) }), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_vsat_tdev, var_vsat_tdev_dn0, var_vsat_tdev_dn1, var_vsat_tdev_dn2, var_vsat_tdev_dn3, var_vsat_tdev_dn4, var_vsat_tdev_dn5, var_vsat_tdev_dn6, var_vsat_tdev_dn7, var_vsat_tdev_dn8, var_vsat_tdev_dn9, var_vsat_tdev_dn12, var_vsat_tdev_dn14, var_vsat_tdev_dn15, var_vsat_tdev_dn16, var_vsat_tdev_dn17, var_vsat_tdev_dn18, var_vsat_tdev_dn19, var_vsat_tdev_dn20, var_vsat_tdev_dn21, var_vsat_tdev_dn22,)
    }
};
        var_vsat_tdev = assign30080_e47441;
        var_vsat_tdev_dn0 = assign30080_e47441_d_n0;
        var_vsat_tdev_dn1 = assign30080_e47441_d_n1;
        var_vsat_tdev_dn2 = assign30080_e47441_d_n2;
        var_vsat_tdev_dn3 = assign30080_e47441_d_n3;
        var_vsat_tdev_dn4 = assign30080_e47441_d_n4;
        var_vsat_tdev_dn5 = assign30080_e47441_d_n5;
        var_vsat_tdev_dn6 = assign30080_e47441_d_n6;
        var_vsat_tdev_dn7 = assign30080_e47441_d_n7;
        var_vsat_tdev_dn8 = assign30080_e47441_d_n8;
        var_vsat_tdev_dn9 = assign30080_e47441_d_n9;
        var_vsat_tdev_dn12 = assign30080_e47441_d_n12;
        var_vsat_tdev_dn14 = assign30080_e47441_d_n14;
        var_vsat_tdev_dn15 = assign30080_e47441_d_n15;
        var_vsat_tdev_dn16 = assign30080_e47441_d_n16;
        var_vsat_tdev_dn17 = assign30080_e47441_d_n17;
        var_vsat_tdev_dn18 = assign30080_e47441_d_n18;
        var_vsat_tdev_dn19 = assign30080_e47441_d_n19;
        var_vsat_tdev_dn20 = assign30080_e47441_d_n20;
        var_vsat_tdev_dn21 = assign30080_e47441_d_n21;
        var_vsat_tdev_dn22 = assign30080_e47441_d_n22;

        let (assign30090_e47455, assign30090_e47455_d_n0, assign30090_e47455_d_n1, assign30090_e47455_d_n2, assign30090_e47455_d_n3, assign30090_e47455_d_n4, assign30090_e47455_d_n5, assign30090_e47455_d_n6, assign30090_e47455_d_n7, assign30090_e47455_d_n8, assign30090_e47455_d_n9, assign30090_e47455_d_n12, assign30090_e47455_d_n14, assign30090_e47455_d_n15, assign30090_e47455_d_n16, assign30090_e47455_d_n17, assign30090_e47455_d_n18, assign30090_e47455_d_n19, assign30090_e47455_d_n20, assign30090_e47455_d_n21, assign30090_e47455_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30090_e47448: f64 = (var_cg_fp4s / p.p9);
        let assign30090_e47451: f64 = (var_vg0_fp4s - var_psis_fp4s);
        let assign30090_e47452: f64 = (assign30090_e47451).abs();
        let assign30090_e47453: f64 = (assign30090_e47448 * assign30090_e47452);
        (assign30090_e47453, (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (var_vg0_fp4s_dn0 - var_psis_fp4s_dn0) } else { (-(var_vg0_fp4s_dn0 - var_psis_fp4s_dn0)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (var_vg0_fp4s_dn1 - var_psis_fp4s_dn1) } else { (-(var_vg0_fp4s_dn1 - var_psis_fp4s_dn1)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (var_vg0_fp4s_dn2 - var_psis_fp4s_dn2) } else { (-(var_vg0_fp4s_dn2 - var_psis_fp4s_dn2)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (var_vg0_fp4s_dn3 - var_psis_fp4s_dn3) } else { (-(var_vg0_fp4s_dn3 - var_psis_fp4s_dn3)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (var_vg0_fp4s_dn4 - var_psis_fp4s_dn4) } else { (-(var_vg0_fp4s_dn4 - var_psis_fp4s_dn4)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (var_vg0_fp4s_dn5 - var_psis_fp4s_dn5) } else { (-(var_vg0_fp4s_dn5 - var_psis_fp4s_dn5)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (var_vg0_fp4s_dn6 - var_psis_fp4s_dn6) } else { (-(var_vg0_fp4s_dn6 - var_psis_fp4s_dn6)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (var_vg0_fp4s_dn7 - var_psis_fp4s_dn7) } else { (-(var_vg0_fp4s_dn7 - var_psis_fp4s_dn7)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (var_vg0_fp4s_dn8 - var_psis_fp4s_dn8) } else { (-(var_vg0_fp4s_dn8 - var_psis_fp4s_dn8)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (var_vg0_fp4s_dn9 - var_psis_fp4s_dn9) } else { (-(var_vg0_fp4s_dn9 - var_psis_fp4s_dn9)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (var_vg0_fp4s_dn12 - var_psis_fp4s_dn12) } else { (-(var_vg0_fp4s_dn12 - var_psis_fp4s_dn12)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (var_vg0_fp4s_dn14 - var_psis_fp4s_dn14) } else { (-(var_vg0_fp4s_dn14 - var_psis_fp4s_dn14)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (var_vg0_fp4s_dn15 - var_psis_fp4s_dn15) } else { (-(var_vg0_fp4s_dn15 - var_psis_fp4s_dn15)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (var_vg0_fp4s_dn16 - var_psis_fp4s_dn16) } else { (-(var_vg0_fp4s_dn16 - var_psis_fp4s_dn16)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (var_vg0_fp4s_dn17 - var_psis_fp4s_dn17) } else { (-(var_vg0_fp4s_dn17 - var_psis_fp4s_dn17)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (var_vg0_fp4s_dn18 - var_psis_fp4s_dn18) } else { (-(var_vg0_fp4s_dn18 - var_psis_fp4s_dn18)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (var_vg0_fp4s_dn19 - var_psis_fp4s_dn19) } else { (-(var_vg0_fp4s_dn19 - var_psis_fp4s_dn19)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (var_vg0_fp4s_dn20 - var_psis_fp4s_dn20) } else { (-(var_vg0_fp4s_dn20 - var_psis_fp4s_dn20)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (var_vg0_fp4s_dn21 - var_psis_fp4s_dn21) } else { (-(var_vg0_fp4s_dn21 - var_psis_fp4s_dn21)) }), (assign30090_e47448 * if assign30090_e47451 >= 0.0 { (var_vg0_fp4s_dn22 - var_psis_fp4s_dn22) } else { (-(var_vg0_fp4s_dn22 - var_psis_fp4s_dn22)) }),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn1, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn12, var_t0_dn14, var_t0_dn15, var_t0_dn16, var_t0_dn17, var_t0_dn18, var_t0_dn19, var_t0_dn20, var_t0_dn21, var_t0_dn22,)
    }
};
        var_t0 = assign30090_e47455;
        var_t0_dn0 = assign30090_e47455_d_n0;
        var_t0_dn1 = assign30090_e47455_d_n1;
        var_t0_dn2 = assign30090_e47455_d_n2;
        var_t0_dn3 = assign30090_e47455_d_n3;
        var_t0_dn4 = assign30090_e47455_d_n4;
        var_t0_dn5 = assign30090_e47455_d_n5;
        var_t0_dn6 = assign30090_e47455_d_n6;
        var_t0_dn7 = assign30090_e47455_d_n7;
        var_t0_dn8 = assign30090_e47455_d_n8;
        var_t0_dn9 = assign30090_e47455_d_n9;
        var_t0_dn12 = assign30090_e47455_d_n12;
        var_t0_dn14 = assign30090_e47455_d_n14;
        var_t0_dn15 = assign30090_e47455_d_n15;
        var_t0_dn16 = assign30090_e47455_d_n16;
        var_t0_dn17 = assign30090_e47455_d_n17;
        var_t0_dn18 = assign30090_e47455_d_n18;
        var_t0_dn19 = assign30090_e47455_d_n19;
        var_t0_dn20 = assign30090_e47455_d_n20;
        var_t0_dn21 = assign30090_e47455_d_n21;
        var_t0_dn22 = assign30090_e47455_d_n22;

        let (assign30100_e47469, assign30100_e47469_d_n0, assign30100_e47469_d_n1, assign30100_e47469_d_n2, assign30100_e47469_d_n3, assign30100_e47469_d_n4, assign30100_e47469_d_n5, assign30100_e47469_d_n6, assign30100_e47469_d_n7, assign30100_e47469_d_n8, assign30100_e47469_d_n9, assign30100_e47469_d_n12, assign30100_e47469_d_n14, assign30100_e47469_d_n15, assign30100_e47469_d_n16, assign30100_e47469_d_n17, assign30100_e47469_d_n18, assign30100_e47469_d_n19, assign30100_e47469_d_n20, assign30100_e47469_d_n21, assign30100_e47469_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30100_e47462: f64 = (var_cepi / p.p9);
        let assign30100_e47465: f64 = (var_vbs - var_psis_fp4s);
        let assign30100_e47466: f64 = (assign30100_e47465).abs();
        let assign30100_e47467: f64 = (assign30100_e47462 * assign30100_e47466);
        (assign30100_e47467, (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-var_psis_fp4s_dn0) } else { (-(-var_psis_fp4s_dn0)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-var_psis_fp4s_dn1) } else { (-(-var_psis_fp4s_dn1)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-var_psis_fp4s_dn2) } else { (-(-var_psis_fp4s_dn2)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (var_vbs_dn3 - var_psis_fp4s_dn3) } else { (-(var_vbs_dn3 - var_psis_fp4s_dn3)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-var_psis_fp4s_dn4) } else { (-(-var_psis_fp4s_dn4)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-var_psis_fp4s_dn5) } else { (-(-var_psis_fp4s_dn5)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-var_psis_fp4s_dn6) } else { (-(-var_psis_fp4s_dn6)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (var_vbs_dn7 - var_psis_fp4s_dn7) } else { (-(var_vbs_dn7 - var_psis_fp4s_dn7)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (var_vbs_dn8 - var_psis_fp4s_dn8) } else { (-(var_vbs_dn8 - var_psis_fp4s_dn8)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-var_psis_fp4s_dn9) } else { (-(-var_psis_fp4s_dn9)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-var_psis_fp4s_dn12) } else { (-(-var_psis_fp4s_dn12)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-var_psis_fp4s_dn14) } else { (-(-var_psis_fp4s_dn14)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-var_psis_fp4s_dn15) } else { (-(-var_psis_fp4s_dn15)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-var_psis_fp4s_dn16) } else { (-(-var_psis_fp4s_dn16)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-var_psis_fp4s_dn17) } else { (-(-var_psis_fp4s_dn17)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-var_psis_fp4s_dn18) } else { (-(-var_psis_fp4s_dn18)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-var_psis_fp4s_dn19) } else { (-(-var_psis_fp4s_dn19)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-var_psis_fp4s_dn20) } else { (-(-var_psis_fp4s_dn20)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-var_psis_fp4s_dn21) } else { (-(-var_psis_fp4s_dn21)) }), (assign30100_e47462 * if assign30100_e47465 >= 0.0 { (-var_psis_fp4s_dn22) } else { (-(-var_psis_fp4s_dn22)) }),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn1, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn12, var_t1_dn14, var_t1_dn15, var_t1_dn16, var_t1_dn17, var_t1_dn18, var_t1_dn19, var_t1_dn20, var_t1_dn21, var_t1_dn22,)
    }
};
        var_t1 = assign30100_e47469;
        var_t1_dn0 = assign30100_e47469_d_n0;
        var_t1_dn1 = assign30100_e47469_d_n1;
        var_t1_dn2 = assign30100_e47469_d_n2;
        var_t1_dn3 = assign30100_e47469_d_n3;
        var_t1_dn4 = assign30100_e47469_d_n4;
        var_t1_dn5 = assign30100_e47469_d_n5;
        var_t1_dn6 = assign30100_e47469_d_n6;
        var_t1_dn7 = assign30100_e47469_d_n7;
        var_t1_dn8 = assign30100_e47469_d_n8;
        var_t1_dn9 = assign30100_e47469_d_n9;
        var_t1_dn12 = assign30100_e47469_d_n12;
        var_t1_dn14 = assign30100_e47469_d_n14;
        var_t1_dn15 = assign30100_e47469_d_n15;
        var_t1_dn16 = assign30100_e47469_d_n16;
        var_t1_dn17 = assign30100_e47469_d_n17;
        var_t1_dn18 = assign30100_e47469_d_n18;
        var_t1_dn19 = assign30100_e47469_d_n19;
        var_t1_dn20 = assign30100_e47469_d_n20;
        var_t1_dn21 = assign30100_e47469_d_n21;
        var_t1_dn22 = assign30100_e47469_d_n22;

        let (assign30110_e47492, assign30110_e47492_d_n0, assign30110_e47492_d_n1, assign30110_e47492_d_n2, assign30110_e47492_d_n3, assign30110_e47492_d_n4, assign30110_e47492_d_n5, assign30110_e47492_d_n6, assign30110_e47492_d_n7, assign30110_e47492_d_n8, assign30110_e47492_d_n9, assign30110_e47492_d_n12, assign30110_e47492_d_n14, assign30110_e47492_d_n15, assign30110_e47492_d_n16, assign30110_e47492_d_n17, assign30110_e47492_d_n18, assign30110_e47492_d_n19, assign30110_e47492_d_n20, assign30110_e47492_d_n21, assign30110_e47492_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30110_e47478: f64 = (p.p14 * var_t0);
        let assign30110_e47479: f64 = (1.0 + assign30110_e47478);
        let assign30110_e47483: f64 = (var_t0 * var_t0);
        let assign30110_e47484: f64 = (p.p15 * assign30110_e47483);
        let assign30110_e47485: f64 = (assign30110_e47479 + assign30110_e47484);
        let assign30110_e47488: f64 = (p.p16 * var_t1);
        let assign30110_e47489: f64 = (assign30110_e47485 + assign30110_e47488);
        let assign30110_e47490: f64 = (var_mulf_tdev / assign30110_e47489);
        (assign30110_e47490, (((var_mulf_tdev_dn0 * assign30110_e47489) - (var_mulf_tdev * (((p.p14 * var_t0_dn0) + (p.p15 * ((var_t0_dn0 * var_t0) + (var_t0 * var_t0_dn0)))) + (p.p16 * var_t1_dn0)))) / (assign30110_e47489 * assign30110_e47489)), (((var_mulf_tdev_dn1 * assign30110_e47489) - (var_mulf_tdev * (((p.p14 * var_t0_dn1) + (p.p15 * ((var_t0_dn1 * var_t0) + (var_t0 * var_t0_dn1)))) + (p.p16 * var_t1_dn1)))) / (assign30110_e47489 * assign30110_e47489)), (((var_mulf_tdev_dn2 * assign30110_e47489) - (var_mulf_tdev * (((p.p14 * var_t0_dn2) + (p.p15 * ((var_t0_dn2 * var_t0) + (var_t0 * var_t0_dn2)))) + (p.p16 * var_t1_dn2)))) / (assign30110_e47489 * assign30110_e47489)), (((var_mulf_tdev_dn3 * assign30110_e47489) - (var_mulf_tdev * (((p.p14 * var_t0_dn3) + (p.p15 * ((var_t0_dn3 * var_t0) + (var_t0 * var_t0_dn3)))) + (p.p16 * var_t1_dn3)))) / (assign30110_e47489 * assign30110_e47489)), (((var_mulf_tdev_dn4 * assign30110_e47489) - (var_mulf_tdev * (((p.p14 * var_t0_dn4) + (p.p15 * ((var_t0_dn4 * var_t0) + (var_t0 * var_t0_dn4)))) + (p.p16 * var_t1_dn4)))) / (assign30110_e47489 * assign30110_e47489)), (((var_mulf_tdev_dn5 * assign30110_e47489) - (var_mulf_tdev * (((p.p14 * var_t0_dn5) + (p.p15 * ((var_t0_dn5 * var_t0) + (var_t0 * var_t0_dn5)))) + (p.p16 * var_t1_dn5)))) / (assign30110_e47489 * assign30110_e47489)), (((var_mulf_tdev_dn6 * assign30110_e47489) - (var_mulf_tdev * (((p.p14 * var_t0_dn6) + (p.p15 * ((var_t0_dn6 * var_t0) + (var_t0 * var_t0_dn6)))) + (p.p16 * var_t1_dn6)))) / (assign30110_e47489 * assign30110_e47489)), (((var_mulf_tdev_dn7 * assign30110_e47489) - (var_mulf_tdev * (((p.p14 * var_t0_dn7) + (p.p15 * ((var_t0_dn7 * var_t0) + (var_t0 * var_t0_dn7)))) + (p.p16 * var_t1_dn7)))) / (assign30110_e47489 * assign30110_e47489)), (((var_mulf_tdev_dn8 * assign30110_e47489) - (var_mulf_tdev * (((p.p14 * var_t0_dn8) + (p.p15 * ((var_t0_dn8 * var_t0) + (var_t0 * var_t0_dn8)))) + (p.p16 * var_t1_dn8)))) / (assign30110_e47489 * assign30110_e47489)), (((var_mulf_tdev_dn9 * assign30110_e47489) - (var_mulf_tdev * (((p.p14 * var_t0_dn9) + (p.p15 * ((var_t0_dn9 * var_t0) + (var_t0 * var_t0_dn9)))) + (p.p16 * var_t1_dn9)))) / (assign30110_e47489 * assign30110_e47489)), (((var_mulf_tdev_dn12 * assign30110_e47489) - (var_mulf_tdev * (((p.p14 * var_t0_dn12) + (p.p15 * ((var_t0_dn12 * var_t0) + (var_t0 * var_t0_dn12)))) + (p.p16 * var_t1_dn12)))) / (assign30110_e47489 * assign30110_e47489)), (((var_mulf_tdev_dn14 * assign30110_e47489) - (var_mulf_tdev * (((p.p14 * var_t0_dn14) + (p.p15 * ((var_t0_dn14 * var_t0) + (var_t0 * var_t0_dn14)))) + (p.p16 * var_t1_dn14)))) / (assign30110_e47489 * assign30110_e47489)), (((var_mulf_tdev_dn15 * assign30110_e47489) - (var_mulf_tdev * (((p.p14 * var_t0_dn15) + (p.p15 * ((var_t0_dn15 * var_t0) + (var_t0 * var_t0_dn15)))) + (p.p16 * var_t1_dn15)))) / (assign30110_e47489 * assign30110_e47489)), (((var_mulf_tdev_dn16 * assign30110_e47489) - (var_mulf_tdev * (((p.p14 * var_t0_dn16) + (p.p15 * ((var_t0_dn16 * var_t0) + (var_t0 * var_t0_dn16)))) + (p.p16 * var_t1_dn16)))) / (assign30110_e47489 * assign30110_e47489)), (((var_mulf_tdev_dn17 * assign30110_e47489) - (var_mulf_tdev * (((p.p14 * var_t0_dn17) + (p.p15 * ((var_t0_dn17 * var_t0) + (var_t0 * var_t0_dn17)))) + (p.p16 * var_t1_dn17)))) / (assign30110_e47489 * assign30110_e47489)), (((var_mulf_tdev_dn18 * assign30110_e47489) - (var_mulf_tdev * (((p.p14 * var_t0_dn18) + (p.p15 * ((var_t0_dn18 * var_t0) + (var_t0 * var_t0_dn18)))) + (p.p16 * var_t1_dn18)))) / (assign30110_e47489 * assign30110_e47489)), (((var_mulf_tdev_dn19 * assign30110_e47489) - (var_mulf_tdev * (((p.p14 * var_t0_dn19) + (p.p15 * ((var_t0_dn19 * var_t0) + (var_t0 * var_t0_dn19)))) + (p.p16 * var_t1_dn19)))) / (assign30110_e47489 * assign30110_e47489)), (((var_mulf_tdev_dn20 * assign30110_e47489) - (var_mulf_tdev * (((p.p14 * var_t0_dn20) + (p.p15 * ((var_t0_dn20 * var_t0) + (var_t0 * var_t0_dn20)))) + (p.p16 * var_t1_dn20)))) / (assign30110_e47489 * assign30110_e47489)), (((var_mulf_tdev_dn21 * assign30110_e47489) - (var_mulf_tdev * (((p.p14 * var_t0_dn21) + (p.p15 * ((var_t0_dn21 * var_t0) + (var_t0 * var_t0_dn21)))) + (p.p16 * var_t1_dn21)))) / (assign30110_e47489 * assign30110_e47489)), (((var_mulf_tdev_dn22 * assign30110_e47489) - (var_mulf_tdev * (((p.p14 * var_t0_dn22) + (p.p15 * ((var_t0_dn22 * var_t0) + (var_t0 * var_t0_dn22)))) + (p.p16 * var_t1_dn22)))) / (assign30110_e47489 * assign30110_e47489)),)
    } else {
        (var_mu_eff, var_mu_eff_dn0, var_mu_eff_dn1, var_mu_eff_dn2, var_mu_eff_dn3, var_mu_eff_dn4, var_mu_eff_dn5, var_mu_eff_dn6, var_mu_eff_dn7, var_mu_eff_dn8, var_mu_eff_dn9, var_mu_eff_dn12, var_mu_eff_dn14, var_mu_eff_dn15, var_mu_eff_dn16, var_mu_eff_dn17, var_mu_eff_dn18, var_mu_eff_dn19, var_mu_eff_dn20, var_mu_eff_dn21, var_mu_eff_dn22,)
    }
};
        var_mu_eff = assign30110_e47492;
        var_mu_eff_dn0 = assign30110_e47492_d_n0;
        var_mu_eff_dn1 = assign30110_e47492_d_n1;
        var_mu_eff_dn2 = assign30110_e47492_d_n2;
        var_mu_eff_dn3 = assign30110_e47492_d_n3;
        var_mu_eff_dn4 = assign30110_e47492_d_n4;
        var_mu_eff_dn5 = assign30110_e47492_d_n5;
        var_mu_eff_dn6 = assign30110_e47492_d_n6;
        var_mu_eff_dn7 = assign30110_e47492_d_n7;
        var_mu_eff_dn8 = assign30110_e47492_d_n8;
        var_mu_eff_dn9 = assign30110_e47492_d_n9;
        var_mu_eff_dn12 = assign30110_e47492_d_n12;
        var_mu_eff_dn14 = assign30110_e47492_d_n14;
        var_mu_eff_dn15 = assign30110_e47492_d_n15;
        var_mu_eff_dn16 = assign30110_e47492_d_n16;
        var_mu_eff_dn17 = assign30110_e47492_d_n17;
        var_mu_eff_dn18 = assign30110_e47492_d_n18;
        var_mu_eff_dn19 = assign30110_e47492_d_n19;
        var_mu_eff_dn20 = assign30110_e47492_d_n20;
        var_mu_eff_dn21 = assign30110_e47492_d_n21;
        var_mu_eff_dn22 = assign30110_e47492_d_n22;

        let (assign30120_e47503, assign30120_e47503_d_n0, assign30120_e47503_d_n1, assign30120_e47503_d_n2, assign30120_e47503_d_n3, assign30120_e47503_d_n4, assign30120_e47503_d_n5, assign30120_e47503_d_n6, assign30120_e47503_d_n7, assign30120_e47503_d_n8, assign30120_e47503_d_n9, assign30120_e47503_d_n12, assign30120_e47503_d_n14, assign30120_e47503_d_n15, assign30120_e47503_d_n16, assign30120_e47503_d_n17, assign30120_e47503_d_n18, assign30120_e47503_d_n19, assign30120_e47503_d_n20, assign30120_e47503_d_n21, assign30120_e47503_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30120_e47499: f64 = (2.0 * var_vsat_tdev);
        let assign30120_e47501: f64 = (assign30120_e47499 / var_mu_eff);
        (assign30120_e47501, ((((2.0 * var_vsat_tdev_dn0) * var_mu_eff) - (assign30120_e47499 * var_mu_eff_dn0)) / (var_mu_eff * var_mu_eff)), ((((2.0 * var_vsat_tdev_dn1) * var_mu_eff) - (assign30120_e47499 * var_mu_eff_dn1)) / (var_mu_eff * var_mu_eff)), ((((2.0 * var_vsat_tdev_dn2) * var_mu_eff) - (assign30120_e47499 * var_mu_eff_dn2)) / (var_mu_eff * var_mu_eff)), ((((2.0 * var_vsat_tdev_dn3) * var_mu_eff) - (assign30120_e47499 * var_mu_eff_dn3)) / (var_mu_eff * var_mu_eff)), ((((2.0 * var_vsat_tdev_dn4) * var_mu_eff) - (assign30120_e47499 * var_mu_eff_dn4)) / (var_mu_eff * var_mu_eff)), ((((2.0 * var_vsat_tdev_dn5) * var_mu_eff) - (assign30120_e47499 * var_mu_eff_dn5)) / (var_mu_eff * var_mu_eff)), ((((2.0 * var_vsat_tdev_dn6) * var_mu_eff) - (assign30120_e47499 * var_mu_eff_dn6)) / (var_mu_eff * var_mu_eff)), ((((2.0 * var_vsat_tdev_dn7) * var_mu_eff) - (assign30120_e47499 * var_mu_eff_dn7)) / (var_mu_eff * var_mu_eff)), ((((2.0 * var_vsat_tdev_dn8) * var_mu_eff) - (assign30120_e47499 * var_mu_eff_dn8)) / (var_mu_eff * var_mu_eff)), ((((2.0 * var_vsat_tdev_dn9) * var_mu_eff) - (assign30120_e47499 * var_mu_eff_dn9)) / (var_mu_eff * var_mu_eff)), ((((2.0 * var_vsat_tdev_dn12) * var_mu_eff) - (assign30120_e47499 * var_mu_eff_dn12)) / (var_mu_eff * var_mu_eff)), ((((2.0 * var_vsat_tdev_dn14) * var_mu_eff) - (assign30120_e47499 * var_mu_eff_dn14)) / (var_mu_eff * var_mu_eff)), ((((2.0 * var_vsat_tdev_dn15) * var_mu_eff) - (assign30120_e47499 * var_mu_eff_dn15)) / (var_mu_eff * var_mu_eff)), ((((2.0 * var_vsat_tdev_dn16) * var_mu_eff) - (assign30120_e47499 * var_mu_eff_dn16)) / (var_mu_eff * var_mu_eff)), ((((2.0 * var_vsat_tdev_dn17) * var_mu_eff) - (assign30120_e47499 * var_mu_eff_dn17)) / (var_mu_eff * var_mu_eff)), ((((2.0 * var_vsat_tdev_dn18) * var_mu_eff) - (assign30120_e47499 * var_mu_eff_dn18)) / (var_mu_eff * var_mu_eff)), ((((2.0 * var_vsat_tdev_dn19) * var_mu_eff) - (assign30120_e47499 * var_mu_eff_dn19)) / (var_mu_eff * var_mu_eff)), ((((2.0 * var_vsat_tdev_dn20) * var_mu_eff) - (assign30120_e47499 * var_mu_eff_dn20)) / (var_mu_eff * var_mu_eff)), ((((2.0 * var_vsat_tdev_dn21) * var_mu_eff) - (assign30120_e47499 * var_mu_eff_dn21)) / (var_mu_eff * var_mu_eff)), ((((2.0 * var_vsat_tdev_dn22) * var_mu_eff) - (assign30120_e47499 * var_mu_eff_dn22)) / (var_mu_eff * var_mu_eff)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn1, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn12, var_t0_dn14, var_t0_dn15, var_t0_dn16, var_t0_dn17, var_t0_dn18, var_t0_dn19, var_t0_dn20, var_t0_dn21, var_t0_dn22,)
    }
};
        var_t0 = assign30120_e47503;
        var_t0_dn0 = assign30120_e47503_d_n0;
        var_t0_dn1 = assign30120_e47503_d_n1;
        var_t0_dn2 = assign30120_e47503_d_n2;
        var_t0_dn3 = assign30120_e47503_d_n3;
        var_t0_dn4 = assign30120_e47503_d_n4;
        var_t0_dn5 = assign30120_e47503_d_n5;
        var_t0_dn6 = assign30120_e47503_d_n6;
        var_t0_dn7 = assign30120_e47503_d_n7;
        var_t0_dn8 = assign30120_e47503_d_n8;
        var_t0_dn9 = assign30120_e47503_d_n9;
        var_t0_dn12 = assign30120_e47503_d_n12;
        var_t0_dn14 = assign30120_e47503_d_n14;
        var_t0_dn15 = assign30120_e47503_d_n15;
        var_t0_dn16 = assign30120_e47503_d_n16;
        var_t0_dn17 = assign30120_e47503_d_n17;
        var_t0_dn18 = assign30120_e47503_d_n18;
        var_t0_dn19 = assign30120_e47503_d_n19;
        var_t0_dn20 = assign30120_e47503_d_n20;
        var_t0_dn21 = assign30120_e47503_d_n21;
        var_t0_dn22 = assign30120_e47503_d_n22;

        let (assign30130_e47525, assign30130_e47525_d_n0, assign30130_e47525_d_n1, assign30130_e47525_d_n2, assign30130_e47525_d_n3, assign30130_e47525_d_n4, assign30130_e47525_d_n5, assign30130_e47525_d_n6, assign30130_e47525_d_n7, assign30130_e47525_d_n8, assign30130_e47525_d_n9, assign30130_e47525_d_n12, assign30130_e47525_d_n14, assign30130_e47525_d_n15, assign30130_e47525_d_n16, assign30130_e47525_d_n17, assign30130_e47525_d_n18, assign30130_e47525_d_n19, assign30130_e47525_d_n20, assign30130_e47525_d_n21, assign30130_e47525_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30130_e47510: f64 = (0.5 * var_vg0_fp4s);
        let assign30130_e47514: f64 = (var_vg0_fp4s * var_vg0_fp4s);
        let assign30130_e47517: f64 = (4.0 * 0.3);
        let assign30130_e47519: f64 = (assign30130_e47517 * 0.3);
        let assign30130_e47520: f64 = (assign30130_e47514 + assign30130_e47519);
        let assign30130_e47521: f64 = (assign30130_e47520).sqrt();
        let assign30130_e47522: f64 = (0.5 * assign30130_e47521);
        let assign30130_e47523: f64 = (assign30130_e47510 + assign30130_e47522);
        (assign30130_e47523, ((0.5 * var_vg0_fp4s_dn0) + (0.5 * (((var_vg0_fp4s_dn0 * var_vg0_fp4s) + (var_vg0_fp4s * var_vg0_fp4s_dn0)) / (2.0 * assign30130_e47521)))), ((0.5 * var_vg0_fp4s_dn1) + (0.5 * (((var_vg0_fp4s_dn1 * var_vg0_fp4s) + (var_vg0_fp4s * var_vg0_fp4s_dn1)) / (2.0 * assign30130_e47521)))), ((0.5 * var_vg0_fp4s_dn2) + (0.5 * (((var_vg0_fp4s_dn2 * var_vg0_fp4s) + (var_vg0_fp4s * var_vg0_fp4s_dn2)) / (2.0 * assign30130_e47521)))), ((0.5 * var_vg0_fp4s_dn3) + (0.5 * (((var_vg0_fp4s_dn3 * var_vg0_fp4s) + (var_vg0_fp4s * var_vg0_fp4s_dn3)) / (2.0 * assign30130_e47521)))), ((0.5 * var_vg0_fp4s_dn4) + (0.5 * (((var_vg0_fp4s_dn4 * var_vg0_fp4s) + (var_vg0_fp4s * var_vg0_fp4s_dn4)) / (2.0 * assign30130_e47521)))), ((0.5 * var_vg0_fp4s_dn5) + (0.5 * (((var_vg0_fp4s_dn5 * var_vg0_fp4s) + (var_vg0_fp4s * var_vg0_fp4s_dn5)) / (2.0 * assign30130_e47521)))), ((0.5 * var_vg0_fp4s_dn6) + (0.5 * (((var_vg0_fp4s_dn6 * var_vg0_fp4s) + (var_vg0_fp4s * var_vg0_fp4s_dn6)) / (2.0 * assign30130_e47521)))), ((0.5 * var_vg0_fp4s_dn7) + (0.5 * (((var_vg0_fp4s_dn7 * var_vg0_fp4s) + (var_vg0_fp4s * var_vg0_fp4s_dn7)) / (2.0 * assign30130_e47521)))), ((0.5 * var_vg0_fp4s_dn8) + (0.5 * (((var_vg0_fp4s_dn8 * var_vg0_fp4s) + (var_vg0_fp4s * var_vg0_fp4s_dn8)) / (2.0 * assign30130_e47521)))), ((0.5 * var_vg0_fp4s_dn9) + (0.5 * (((var_vg0_fp4s_dn9 * var_vg0_fp4s) + (var_vg0_fp4s * var_vg0_fp4s_dn9)) / (2.0 * assign30130_e47521)))), ((0.5 * var_vg0_fp4s_dn12) + (0.5 * (((var_vg0_fp4s_dn12 * var_vg0_fp4s) + (var_vg0_fp4s * var_vg0_fp4s_dn12)) / (2.0 * assign30130_e47521)))), ((0.5 * var_vg0_fp4s_dn14) + (0.5 * (((var_vg0_fp4s_dn14 * var_vg0_fp4s) + (var_vg0_fp4s * var_vg0_fp4s_dn14)) / (2.0 * assign30130_e47521)))), ((0.5 * var_vg0_fp4s_dn15) + (0.5 * (((var_vg0_fp4s_dn15 * var_vg0_fp4s) + (var_vg0_fp4s * var_vg0_fp4s_dn15)) / (2.0 * assign30130_e47521)))), ((0.5 * var_vg0_fp4s_dn16) + (0.5 * (((var_vg0_fp4s_dn16 * var_vg0_fp4s) + (var_vg0_fp4s * var_vg0_fp4s_dn16)) / (2.0 * assign30130_e47521)))), ((0.5 * var_vg0_fp4s_dn17) + (0.5 * (((var_vg0_fp4s_dn17 * var_vg0_fp4s) + (var_vg0_fp4s * var_vg0_fp4s_dn17)) / (2.0 * assign30130_e47521)))), ((0.5 * var_vg0_fp4s_dn18) + (0.5 * (((var_vg0_fp4s_dn18 * var_vg0_fp4s) + (var_vg0_fp4s * var_vg0_fp4s_dn18)) / (2.0 * assign30130_e47521)))), ((0.5 * var_vg0_fp4s_dn19) + (0.5 * (((var_vg0_fp4s_dn19 * var_vg0_fp4s) + (var_vg0_fp4s * var_vg0_fp4s_dn19)) / (2.0 * assign30130_e47521)))), ((0.5 * var_vg0_fp4s_dn20) + (0.5 * (((var_vg0_fp4s_dn20 * var_vg0_fp4s) + (var_vg0_fp4s * var_vg0_fp4s_dn20)) / (2.0 * assign30130_e47521)))), ((0.5 * var_vg0_fp4s_dn21) + (0.5 * (((var_vg0_fp4s_dn21 * var_vg0_fp4s) + (var_vg0_fp4s * var_vg0_fp4s_dn21)) / (2.0 * assign30130_e47521)))), ((0.5 * var_vg0_fp4s_dn22) + (0.5 * (((var_vg0_fp4s_dn22 * var_vg0_fp4s) + (var_vg0_fp4s * var_vg0_fp4s_dn22)) / (2.0 * assign30130_e47521)))),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn1, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn12, var_t1_dn14, var_t1_dn15, var_t1_dn16, var_t1_dn17, var_t1_dn18, var_t1_dn19, var_t1_dn20, var_t1_dn21, var_t1_dn22,)
    }
};
        var_t1 = assign30130_e47525;
        var_t1_dn0 = assign30130_e47525_d_n0;
        var_t1_dn1 = assign30130_e47525_d_n1;
        var_t1_dn2 = assign30130_e47525_d_n2;
        var_t1_dn3 = assign30130_e47525_d_n3;
        var_t1_dn4 = assign30130_e47525_d_n4;
        var_t1_dn5 = assign30130_e47525_d_n5;
        var_t1_dn6 = assign30130_e47525_d_n6;
        var_t1_dn7 = assign30130_e47525_d_n7;
        var_t1_dn8 = assign30130_e47525_d_n8;
        var_t1_dn9 = assign30130_e47525_d_n9;
        var_t1_dn12 = assign30130_e47525_d_n12;
        var_t1_dn14 = assign30130_e47525_d_n14;
        var_t1_dn15 = assign30130_e47525_d_n15;
        var_t1_dn16 = assign30130_e47525_d_n16;
        var_t1_dn17 = assign30130_e47525_d_n17;
        var_t1_dn18 = assign30130_e47525_d_n18;
        var_t1_dn19 = assign30130_e47525_d_n19;
        var_t1_dn20 = assign30130_e47525_d_n20;
        var_t1_dn21 = assign30130_e47525_d_n21;
        var_t1_dn22 = assign30130_e47525_d_n22;

        let (assign30140_e47542, assign30140_e47542_d_n0, assign30140_e47542_d_n1, assign30140_e47542_d_n2, assign30140_e47542_d_n3, assign30140_e47542_d_n4, assign30140_e47542_d_n5, assign30140_e47542_d_n6, assign30140_e47542_d_n7, assign30140_e47542_d_n8, assign30140_e47542_d_n9, assign30140_e47542_d_n12, assign30140_e47542_d_n14, assign30140_e47542_d_n15, assign30140_e47542_d_n16, assign30140_e47542_d_n17, assign30140_e47542_d_n18, assign30140_e47542_d_n19, assign30140_e47542_d_n20, assign30140_e47542_d_n21, assign30140_e47542_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30140_e47532: f64 = (var_t0 * p.p200);
        let assign30140_e47534: f64 = (assign30140_e47532 * var_t1);
        let assign30140_e47537: f64 = (var_t0 * p.p200);
        let assign30140_e47539: f64 = (assign30140_e47537 + var_t1);
        let assign30140_e47540: f64 = (assign30140_e47534 / assign30140_e47539);
        (assign30140_e47540, ((((((var_t0_dn0 * p.p200) * var_t1) + (assign30140_e47532 * var_t1_dn0)) * assign30140_e47539) - (assign30140_e47534 * ((var_t0_dn0 * p.p200) + var_t1_dn0))) / (assign30140_e47539 * assign30140_e47539)), ((((((var_t0_dn1 * p.p200) * var_t1) + (assign30140_e47532 * var_t1_dn1)) * assign30140_e47539) - (assign30140_e47534 * ((var_t0_dn1 * p.p200) + var_t1_dn1))) / (assign30140_e47539 * assign30140_e47539)), ((((((var_t0_dn2 * p.p200) * var_t1) + (assign30140_e47532 * var_t1_dn2)) * assign30140_e47539) - (assign30140_e47534 * ((var_t0_dn2 * p.p200) + var_t1_dn2))) / (assign30140_e47539 * assign30140_e47539)), ((((((var_t0_dn3 * p.p200) * var_t1) + (assign30140_e47532 * var_t1_dn3)) * assign30140_e47539) - (assign30140_e47534 * ((var_t0_dn3 * p.p200) + var_t1_dn3))) / (assign30140_e47539 * assign30140_e47539)), ((((((var_t0_dn4 * p.p200) * var_t1) + (assign30140_e47532 * var_t1_dn4)) * assign30140_e47539) - (assign30140_e47534 * ((var_t0_dn4 * p.p200) + var_t1_dn4))) / (assign30140_e47539 * assign30140_e47539)), ((((((var_t0_dn5 * p.p200) * var_t1) + (assign30140_e47532 * var_t1_dn5)) * assign30140_e47539) - (assign30140_e47534 * ((var_t0_dn5 * p.p200) + var_t1_dn5))) / (assign30140_e47539 * assign30140_e47539)), ((((((var_t0_dn6 * p.p200) * var_t1) + (assign30140_e47532 * var_t1_dn6)) * assign30140_e47539) - (assign30140_e47534 * ((var_t0_dn6 * p.p200) + var_t1_dn6))) / (assign30140_e47539 * assign30140_e47539)), ((((((var_t0_dn7 * p.p200) * var_t1) + (assign30140_e47532 * var_t1_dn7)) * assign30140_e47539) - (assign30140_e47534 * ((var_t0_dn7 * p.p200) + var_t1_dn7))) / (assign30140_e47539 * assign30140_e47539)), ((((((var_t0_dn8 * p.p200) * var_t1) + (assign30140_e47532 * var_t1_dn8)) * assign30140_e47539) - (assign30140_e47534 * ((var_t0_dn8 * p.p200) + var_t1_dn8))) / (assign30140_e47539 * assign30140_e47539)), ((((((var_t0_dn9 * p.p200) * var_t1) + (assign30140_e47532 * var_t1_dn9)) * assign30140_e47539) - (assign30140_e47534 * ((var_t0_dn9 * p.p200) + var_t1_dn9))) / (assign30140_e47539 * assign30140_e47539)), ((((((var_t0_dn12 * p.p200) * var_t1) + (assign30140_e47532 * var_t1_dn12)) * assign30140_e47539) - (assign30140_e47534 * ((var_t0_dn12 * p.p200) + var_t1_dn12))) / (assign30140_e47539 * assign30140_e47539)), ((((((var_t0_dn14 * p.p200) * var_t1) + (assign30140_e47532 * var_t1_dn14)) * assign30140_e47539) - (assign30140_e47534 * ((var_t0_dn14 * p.p200) + var_t1_dn14))) / (assign30140_e47539 * assign30140_e47539)), ((((((var_t0_dn15 * p.p200) * var_t1) + (assign30140_e47532 * var_t1_dn15)) * assign30140_e47539) - (assign30140_e47534 * ((var_t0_dn15 * p.p200) + var_t1_dn15))) / (assign30140_e47539 * assign30140_e47539)), ((((((var_t0_dn16 * p.p200) * var_t1) + (assign30140_e47532 * var_t1_dn16)) * assign30140_e47539) - (assign30140_e47534 * ((var_t0_dn16 * p.p200) + var_t1_dn16))) / (assign30140_e47539 * assign30140_e47539)), ((((((var_t0_dn17 * p.p200) * var_t1) + (assign30140_e47532 * var_t1_dn17)) * assign30140_e47539) - (assign30140_e47534 * ((var_t0_dn17 * p.p200) + var_t1_dn17))) / (assign30140_e47539 * assign30140_e47539)), ((((((var_t0_dn18 * p.p200) * var_t1) + (assign30140_e47532 * var_t1_dn18)) * assign30140_e47539) - (assign30140_e47534 * ((var_t0_dn18 * p.p200) + var_t1_dn18))) / (assign30140_e47539 * assign30140_e47539)), ((((((var_t0_dn19 * p.p200) * var_t1) + (assign30140_e47532 * var_t1_dn19)) * assign30140_e47539) - (assign30140_e47534 * ((var_t0_dn19 * p.p200) + var_t1_dn19))) / (assign30140_e47539 * assign30140_e47539)), ((((((var_t0_dn20 * p.p200) * var_t1) + (assign30140_e47532 * var_t1_dn20)) * assign30140_e47539) - (assign30140_e47534 * ((var_t0_dn20 * p.p200) + var_t1_dn20))) / (assign30140_e47539 * assign30140_e47539)), ((((((var_t0_dn21 * p.p200) * var_t1) + (assign30140_e47532 * var_t1_dn21)) * assign30140_e47539) - (assign30140_e47534 * ((var_t0_dn21 * p.p200) + var_t1_dn21))) / (assign30140_e47539 * assign30140_e47539)), ((((((var_t0_dn22 * p.p200) * var_t1) + (assign30140_e47532 * var_t1_dn22)) * assign30140_e47539) - (assign30140_e47534 * ((var_t0_dn22 * p.p200) + var_t1_dn22))) / (assign30140_e47539 * assign30140_e47539)),)
    } else {
        (var_vdsat, var_vdsat_dn0, var_vdsat_dn1, var_vdsat_dn2, var_vdsat_dn3, var_vdsat_dn4, var_vdsat_dn5, var_vdsat_dn6, var_vdsat_dn7, var_vdsat_dn8, var_vdsat_dn9, var_vdsat_dn12, var_vdsat_dn14, var_vdsat_dn15, var_vdsat_dn16, var_vdsat_dn17, var_vdsat_dn18, var_vdsat_dn19, var_vdsat_dn20, var_vdsat_dn21, var_vdsat_dn22,)
    }
};
        var_vdsat = assign30140_e47542;
        var_vdsat_dn0 = assign30140_e47542_d_n0;
        var_vdsat_dn1 = assign30140_e47542_d_n1;
        var_vdsat_dn2 = assign30140_e47542_d_n2;
        var_vdsat_dn3 = assign30140_e47542_d_n3;
        var_vdsat_dn4 = assign30140_e47542_d_n4;
        var_vdsat_dn5 = assign30140_e47542_d_n5;
        var_vdsat_dn6 = assign30140_e47542_d_n6;
        var_vdsat_dn7 = assign30140_e47542_d_n7;
        var_vdsat_dn8 = assign30140_e47542_d_n8;
        var_vdsat_dn9 = assign30140_e47542_d_n9;
        var_vdsat_dn12 = assign30140_e47542_d_n12;
        var_vdsat_dn14 = assign30140_e47542_d_n14;
        var_vdsat_dn15 = assign30140_e47542_d_n15;
        var_vdsat_dn16 = assign30140_e47542_d_n16;
        var_vdsat_dn17 = assign30140_e47542_d_n17;
        var_vdsat_dn18 = assign30140_e47542_d_n18;
        var_vdsat_dn19 = assign30140_e47542_d_n19;
        var_vdsat_dn20 = assign30140_e47542_d_n20;
        var_vdsat_dn21 = assign30140_e47542_d_n21;
        var_vdsat_dn22 = assign30140_e47542_d_n22;

        let (assign30150_e47553, assign30150_e47553_d_n0, assign30150_e47553_d_n1, assign30150_e47553_d_n2, assign30150_e47553_d_n3, assign30150_e47553_d_n4, assign30150_e47553_d_n5, assign30150_e47553_d_n6, assign30150_e47553_d_n7, assign30150_e47553_d_n8, assign30150_e47553_d_n9, assign30150_e47553_d_n12, assign30150_e47553_d_n14, assign30150_e47553_d_n15, assign30150_e47553_d_n16, assign30150_e47553_d_n17, assign30150_e47553_d_n18, assign30150_e47553_d_n19, assign30150_e47553_d_n20, assign30150_e47553_d_n21, assign30150_e47553_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30150_e47549: f64 = (var_vds_fp4s / var_vdsat);
        let assign30150_e47551: f64 = (assign30150_e47549).powf(p.p18);
        (assign30150_e47551, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((var_vds_fp4s * var_vdsat_dn0) / (var_vdsat * var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((var_vds_fp4s * var_vdsat_dn0) / (var_vdsat * var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((var_vds_fp4s * var_vdsat_dn1) / (var_vdsat * var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((var_vds_fp4s * var_vdsat_dn1) / (var_vdsat * var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((var_vds_fp4s * var_vdsat_dn2) / (var_vdsat * var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((var_vds_fp4s * var_vdsat_dn2) / (var_vdsat * var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((var_vds_fp4s * var_vdsat_dn3) / (var_vdsat * var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((var_vds_fp4s * var_vdsat_dn3) / (var_vdsat * var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((var_vds_fp4s * var_vdsat_dn4) / (var_vdsat * var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((var_vds_fp4s * var_vdsat_dn4) / (var_vdsat * var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((var_vds_fp4s * var_vdsat_dn5) / (var_vdsat * var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((var_vds_fp4s * var_vdsat_dn5) / (var_vdsat * var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((var_vds_fp4s * var_vdsat_dn6) / (var_vdsat * var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((var_vds_fp4s * var_vdsat_dn6) / (var_vdsat * var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((var_vds_fp4s * var_vdsat_dn7) / (var_vdsat * var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((var_vds_fp4s * var_vdsat_dn7) / (var_vdsat * var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((var_vds_fp4s * var_vdsat_dn8) / (var_vdsat * var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((var_vds_fp4s * var_vdsat_dn8) / (var_vdsat * var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((var_vds_fp4s * var_vdsat_dn9) / (var_vdsat * var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((var_vds_fp4s * var_vdsat_dn9) / (var_vdsat * var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((var_vds_fp4s * var_vdsat_dn12) / (var_vdsat * var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((var_vds_fp4s * var_vdsat_dn12) / (var_vdsat * var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((var_vds_fp4s * var_vdsat_dn14) / (var_vdsat * var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((var_vds_fp4s * var_vdsat_dn14) / (var_vdsat * var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((var_vds_fp4s * var_vdsat_dn15) / (var_vdsat * var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((var_vds_fp4s * var_vdsat_dn15) / (var_vdsat * var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((var_vds_fp4s * var_vdsat_dn16) / (var_vdsat * var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((var_vds_fp4s * var_vdsat_dn16) / (var_vdsat * var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((var_vds_fp4s * var_vdsat_dn17) / (var_vdsat * var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((var_vds_fp4s * var_vdsat_dn17) / (var_vdsat * var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((var_vds_fp4s * var_vdsat_dn18) / (var_vdsat * var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((var_vds_fp4s * var_vdsat_dn18) / (var_vdsat * var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((var_vds_fp4s * var_vdsat_dn19) / (var_vdsat * var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((var_vds_fp4s * var_vdsat_dn19) / (var_vdsat * var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (-((var_vds_fp4s * var_vdsat_dn20) / (var_vdsat * var_vdsat))))) } } else { (assign30150_e47551 * (p.p18 * ((-((var_vds_fp4s * var_vdsat_dn20) / (var_vdsat * var_vdsat))) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (((var_vds_fp4s_dn21 * var_vdsat) - (var_vds_fp4s * var_vdsat_dn21)) / (var_vdsat * var_vdsat)))) } } else { (assign30150_e47551 * (p.p18 * ((((var_vds_fp4s_dn21 * var_vdsat) - (var_vds_fp4s * var_vdsat_dn21)) / (var_vdsat * var_vdsat)) / assign30150_e47549))) }, if 0.0 == 0.0 && ((p.p18) as f64).is_finite() && ((p.p18) as f64).fract() == 0.0 { if p.p18 == 0.0 { 0.0 } else { (p.p18 * ((assign30150_e47549).powf(p.p18 - 1.0) * (((var_vds_fp4s_dn22 * var_vdsat) - (var_vds_fp4s * var_vdsat_dn22)) / (var_vdsat * var_vdsat)))) } } else { (assign30150_e47551 * (p.p18 * ((((var_vds_fp4s_dn22 * var_vdsat) - (var_vds_fp4s * var_vdsat_dn22)) / (var_vdsat * var_vdsat)) / assign30150_e47549))) },)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn1, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn12, var_t0_dn14, var_t0_dn15, var_t0_dn16, var_t0_dn17, var_t0_dn18, var_t0_dn19, var_t0_dn20, var_t0_dn21, var_t0_dn22,)
    }
};
        var_t0 = assign30150_e47553;
        var_t0_dn0 = assign30150_e47553_d_n0;
        var_t0_dn1 = assign30150_e47553_d_n1;
        var_t0_dn2 = assign30150_e47553_d_n2;
        var_t0_dn3 = assign30150_e47553_d_n3;
        var_t0_dn4 = assign30150_e47553_d_n4;
        var_t0_dn5 = assign30150_e47553_d_n5;
        var_t0_dn6 = assign30150_e47553_d_n6;
        var_t0_dn7 = assign30150_e47553_d_n7;
        var_t0_dn8 = assign30150_e47553_d_n8;
        var_t0_dn9 = assign30150_e47553_d_n9;
        var_t0_dn12 = assign30150_e47553_d_n12;
        var_t0_dn14 = assign30150_e47553_d_n14;
        var_t0_dn15 = assign30150_e47553_d_n15;
        var_t0_dn16 = assign30150_e47553_d_n16;
        var_t0_dn17 = assign30150_e47553_d_n17;
        var_t0_dn18 = assign30150_e47553_d_n18;
        var_t0_dn19 = assign30150_e47553_d_n19;
        var_t0_dn20 = assign30150_e47553_d_n20;
        var_t0_dn21 = assign30150_e47553_d_n21;
        var_t0_dn22 = assign30150_e47553_d_n22;

        let (assign30160_e47567, assign30160_e47567_d_n0, assign30160_e47567_d_n1, assign30160_e47567_d_n2, assign30160_e47567_d_n3, assign30160_e47567_d_n4, assign30160_e47567_d_n5, assign30160_e47567_d_n6, assign30160_e47567_d_n7, assign30160_e47567_d_n8, assign30160_e47567_d_n9, assign30160_e47567_d_n12, assign30160_e47567_d_n14, assign30160_e47567_d_n15, assign30160_e47567_d_n16, assign30160_e47567_d_n17, assign30160_e47567_d_n18, assign30160_e47567_d_n19, assign30160_e47567_d_n20, assign30160_e47567_d_n21, assign30160_e47567_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30160_e47560: f64 = (1.0 + var_t0);
        let assign30160_e47562: f64 = (-1.0);
        let assign30160_e47564: f64 = (assign30160_e47562 / p.p18);
        let assign30160_e47565: f64 = (assign30160_e47560).powf(assign30160_e47564);
        (assign30160_e47565, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * var_t0_dn0)) } } else { (assign30160_e47565 * (assign30160_e47564 * (var_t0_dn0 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * var_t0_dn1)) } } else { (assign30160_e47565 * (assign30160_e47564 * (var_t0_dn1 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * var_t0_dn2)) } } else { (assign30160_e47565 * (assign30160_e47564 * (var_t0_dn2 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * var_t0_dn3)) } } else { (assign30160_e47565 * (assign30160_e47564 * (var_t0_dn3 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * var_t0_dn4)) } } else { (assign30160_e47565 * (assign30160_e47564 * (var_t0_dn4 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * var_t0_dn5)) } } else { (assign30160_e47565 * (assign30160_e47564 * (var_t0_dn5 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * var_t0_dn6)) } } else { (assign30160_e47565 * (assign30160_e47564 * (var_t0_dn6 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * var_t0_dn7)) } } else { (assign30160_e47565 * (assign30160_e47564 * (var_t0_dn7 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * var_t0_dn8)) } } else { (assign30160_e47565 * (assign30160_e47564 * (var_t0_dn8 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * var_t0_dn9)) } } else { (assign30160_e47565 * (assign30160_e47564 * (var_t0_dn9 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * var_t0_dn12)) } } else { (assign30160_e47565 * (assign30160_e47564 * (var_t0_dn12 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * var_t0_dn14)) } } else { (assign30160_e47565 * (assign30160_e47564 * (var_t0_dn14 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * var_t0_dn15)) } } else { (assign30160_e47565 * (assign30160_e47564 * (var_t0_dn15 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * var_t0_dn16)) } } else { (assign30160_e47565 * (assign30160_e47564 * (var_t0_dn16 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * var_t0_dn17)) } } else { (assign30160_e47565 * (assign30160_e47564 * (var_t0_dn17 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * var_t0_dn18)) } } else { (assign30160_e47565 * (assign30160_e47564 * (var_t0_dn18 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * var_t0_dn19)) } } else { (assign30160_e47565 * (assign30160_e47564 * (var_t0_dn19 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * var_t0_dn20)) } } else { (assign30160_e47565 * (assign30160_e47564 * (var_t0_dn20 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * var_t0_dn21)) } } else { (assign30160_e47565 * (assign30160_e47564 * (var_t0_dn21 / assign30160_e47560))) }, if 0.0 == 0.0 && ((assign30160_e47564) as f64).is_finite() && ((assign30160_e47564) as f64).fract() == 0.0 { if assign30160_e47564 == 0.0 { 0.0 } else { (assign30160_e47564 * ((assign30160_e47560).powf(assign30160_e47564 - 1.0) * var_t0_dn22)) } } else { (assign30160_e47565 * (assign30160_e47564 * (var_t0_dn22 / assign30160_e47560))) },)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn1, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn12, var_t1_dn14, var_t1_dn15, var_t1_dn16, var_t1_dn17, var_t1_dn18, var_t1_dn19, var_t1_dn20, var_t1_dn21, var_t1_dn22,)
    }
};
        var_t1 = assign30160_e47567;
        var_t1_dn0 = assign30160_e47567_d_n0;
        var_t1_dn1 = assign30160_e47567_d_n1;
        var_t1_dn2 = assign30160_e47567_d_n2;
        var_t1_dn3 = assign30160_e47567_d_n3;
        var_t1_dn4 = assign30160_e47567_d_n4;
        var_t1_dn5 = assign30160_e47567_d_n5;
        var_t1_dn6 = assign30160_e47567_d_n6;
        var_t1_dn7 = assign30160_e47567_d_n7;
        var_t1_dn8 = assign30160_e47567_d_n8;
        var_t1_dn9 = assign30160_e47567_d_n9;
        var_t1_dn12 = assign30160_e47567_d_n12;
        var_t1_dn14 = assign30160_e47567_d_n14;
        var_t1_dn15 = assign30160_e47567_d_n15;
        var_t1_dn16 = assign30160_e47567_d_n16;
        var_t1_dn17 = assign30160_e47567_d_n17;
        var_t1_dn18 = assign30160_e47567_d_n18;
        var_t1_dn19 = assign30160_e47567_d_n19;
        var_t1_dn20 = assign30160_e47567_d_n20;
        var_t1_dn21 = assign30160_e47567_d_n21;
        var_t1_dn22 = assign30160_e47567_d_n22;

        *var_ef3_slot = var_ef3;
        *var_ef3_dn0_slot = var_ef3_dn0;
        *var_ef3_dn1_slot = var_ef3_dn1;
        *var_ef3_dn12_slot = var_ef3_dn12;
        *var_ef3_dn14_slot = var_ef3_dn14;
        *var_ef3_dn15_slot = var_ef3_dn15;
        *var_ef3_dn16_slot = var_ef3_dn16;
        *var_ef3_dn17_slot = var_ef3_dn17;
        *var_ef3_dn18_slot = var_ef3_dn18;
        *var_ef3_dn19_slot = var_ef3_dn19;
        *var_ef3_dn2_slot = var_ef3_dn2;
        *var_ef3_dn20_slot = var_ef3_dn20;
        *var_ef3_dn21_slot = var_ef3_dn21;
        *var_ef3_dn22_slot = var_ef3_dn22;
        *var_ef3_dn3_slot = var_ef3_dn3;
        *var_ef3_dn4_slot = var_ef3_dn4;
        *var_ef3_dn5_slot = var_ef3_dn5;
        *var_ef3_dn6_slot = var_ef3_dn6;
        *var_ef3_dn7_slot = var_ef3_dn7;
        *var_ef3_dn8_slot = var_ef3_dn8;
        *var_ef3_dn9_slot = var_ef3_dn9;
        *var_mu_eff_slot = var_mu_eff;
        *var_mu_eff_dn0_slot = var_mu_eff_dn0;
        *var_mu_eff_dn1_slot = var_mu_eff_dn1;
        *var_mu_eff_dn12_slot = var_mu_eff_dn12;
        *var_mu_eff_dn14_slot = var_mu_eff_dn14;
        *var_mu_eff_dn15_slot = var_mu_eff_dn15;
        *var_mu_eff_dn16_slot = var_mu_eff_dn16;
        *var_mu_eff_dn17_slot = var_mu_eff_dn17;
        *var_mu_eff_dn18_slot = var_mu_eff_dn18;
        *var_mu_eff_dn19_slot = var_mu_eff_dn19;
        *var_mu_eff_dn2_slot = var_mu_eff_dn2;
        *var_mu_eff_dn20_slot = var_mu_eff_dn20;
        *var_mu_eff_dn21_slot = var_mu_eff_dn21;
        *var_mu_eff_dn22_slot = var_mu_eff_dn22;
        *var_mu_eff_dn3_slot = var_mu_eff_dn3;
        *var_mu_eff_dn4_slot = var_mu_eff_dn4;
        *var_mu_eff_dn5_slot = var_mu_eff_dn5;
        *var_mu_eff_dn6_slot = var_mu_eff_dn6;
        *var_mu_eff_dn7_slot = var_mu_eff_dn7;
        *var_mu_eff_dn8_slot = var_mu_eff_dn8;
        *var_mu_eff_dn9_slot = var_mu_eff_dn9;
        *var_mulf_tdev_slot = var_mulf_tdev;
        *var_mulf_tdev_dn0_slot = var_mulf_tdev_dn0;
        *var_mulf_tdev_dn1_slot = var_mulf_tdev_dn1;
        *var_mulf_tdev_dn12_slot = var_mulf_tdev_dn12;
        *var_mulf_tdev_dn14_slot = var_mulf_tdev_dn14;
        *var_mulf_tdev_dn15_slot = var_mulf_tdev_dn15;
        *var_mulf_tdev_dn16_slot = var_mulf_tdev_dn16;
        *var_mulf_tdev_dn17_slot = var_mulf_tdev_dn17;
        *var_mulf_tdev_dn18_slot = var_mulf_tdev_dn18;
        *var_mulf_tdev_dn19_slot = var_mulf_tdev_dn19;
        *var_mulf_tdev_dn2_slot = var_mulf_tdev_dn2;
        *var_mulf_tdev_dn20_slot = var_mulf_tdev_dn20;
        *var_mulf_tdev_dn21_slot = var_mulf_tdev_dn21;
        *var_mulf_tdev_dn22_slot = var_mulf_tdev_dn22;
        *var_mulf_tdev_dn3_slot = var_mulf_tdev_dn3;
        *var_mulf_tdev_dn4_slot = var_mulf_tdev_dn4;
        *var_mulf_tdev_dn5_slot = var_mulf_tdev_dn5;
        *var_mulf_tdev_dn6_slot = var_mulf_tdev_dn6;
        *var_mulf_tdev_dn7_slot = var_mulf_tdev_dn7;
        *var_mulf_tdev_dn8_slot = var_mulf_tdev_dn8;
        *var_mulf_tdev_dn9_slot = var_mulf_tdev_dn9;
        *var_psis_fp4s_slot = var_psis_fp4s;
        *var_psis_fp4s_dn0_slot = var_psis_fp4s_dn0;
        *var_psis_fp4s_dn1_slot = var_psis_fp4s_dn1;
        *var_psis_fp4s_dn12_slot = var_psis_fp4s_dn12;
        *var_psis_fp4s_dn14_slot = var_psis_fp4s_dn14;
        *var_psis_fp4s_dn15_slot = var_psis_fp4s_dn15;
        *var_psis_fp4s_dn16_slot = var_psis_fp4s_dn16;
        *var_psis_fp4s_dn17_slot = var_psis_fp4s_dn17;
        *var_psis_fp4s_dn18_slot = var_psis_fp4s_dn18;
        *var_psis_fp4s_dn19_slot = var_psis_fp4s_dn19;
        *var_psis_fp4s_dn2_slot = var_psis_fp4s_dn2;
        *var_psis_fp4s_dn20_slot = var_psis_fp4s_dn20;
        *var_psis_fp4s_dn21_slot = var_psis_fp4s_dn21;
        *var_psis_fp4s_dn22_slot = var_psis_fp4s_dn22;
        *var_psis_fp4s_dn3_slot = var_psis_fp4s_dn3;
        *var_psis_fp4s_dn4_slot = var_psis_fp4s_dn4;
        *var_psis_fp4s_dn5_slot = var_psis_fp4s_dn5;
        *var_psis_fp4s_dn6_slot = var_psis_fp4s_dn6;
        *var_psis_fp4s_dn7_slot = var_psis_fp4s_dn7;
        *var_psis_fp4s_dn8_slot = var_psis_fp4s_dn8;
        *var_psis_fp4s_dn9_slot = var_psis_fp4s_dn9;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn1_slot = var_t0_dn1;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn14_slot = var_t0_dn14;
        *var_t0_dn15_slot = var_t0_dn15;
        *var_t0_dn16_slot = var_t0_dn16;
        *var_t0_dn17_slot = var_t0_dn17;
        *var_t0_dn18_slot = var_t0_dn18;
        *var_t0_dn19_slot = var_t0_dn19;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn20_slot = var_t0_dn20;
        *var_t0_dn21_slot = var_t0_dn21;
        *var_t0_dn22_slot = var_t0_dn22;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn1_slot = var_t1_dn1;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn15_slot = var_t1_dn15;
        *var_t1_dn16_slot = var_t1_dn16;
        *var_t1_dn17_slot = var_t1_dn17;
        *var_t1_dn18_slot = var_t1_dn18;
        *var_t1_dn19_slot = var_t1_dn19;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn20_slot = var_t1_dn20;
        *var_t1_dn21_slot = var_t1_dn21;
        *var_t1_dn22_slot = var_t1_dn22;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t52_slot = var_t52;
        *var_t52_dn0_slot = var_t52_dn0;
        *var_t52_dn1_slot = var_t52_dn1;
        *var_t52_dn12_slot = var_t52_dn12;
        *var_t52_dn14_slot = var_t52_dn14;
        *var_t52_dn15_slot = var_t52_dn15;
        *var_t52_dn16_slot = var_t52_dn16;
        *var_t52_dn17_slot = var_t52_dn17;
        *var_t52_dn18_slot = var_t52_dn18;
        *var_t52_dn19_slot = var_t52_dn19;
        *var_t52_dn2_slot = var_t52_dn2;
        *var_t52_dn20_slot = var_t52_dn20;
        *var_t52_dn21_slot = var_t52_dn21;
        *var_t52_dn22_slot = var_t52_dn22;
        *var_t52_dn3_slot = var_t52_dn3;
        *var_t52_dn4_slot = var_t52_dn4;
        *var_t52_dn5_slot = var_t52_dn5;
        *var_t52_dn6_slot = var_t52_dn6;
        *var_t52_dn7_slot = var_t52_dn7;
        *var_t52_dn8_slot = var_t52_dn8;
        *var_t52_dn9_slot = var_t52_dn9;
        *var_t5dg12_slot = var_t5dg12;
        *var_t5dg12_dn0_slot = var_t5dg12_dn0;
        *var_t5dg12_dn1_slot = var_t5dg12_dn1;
        *var_t5dg12_dn12_slot = var_t5dg12_dn12;
        *var_t5dg12_dn14_slot = var_t5dg12_dn14;
        *var_t5dg12_dn15_slot = var_t5dg12_dn15;
        *var_t5dg12_dn16_slot = var_t5dg12_dn16;
        *var_t5dg12_dn17_slot = var_t5dg12_dn17;
        *var_t5dg12_dn18_slot = var_t5dg12_dn18;
        *var_t5dg12_dn19_slot = var_t5dg12_dn19;
        *var_t5dg12_dn2_slot = var_t5dg12_dn2;
        *var_t5dg12_dn20_slot = var_t5dg12_dn20;
        *var_t5dg12_dn21_slot = var_t5dg12_dn21;
        *var_t5dg12_dn22_slot = var_t5dg12_dn22;
        *var_t5dg12_dn3_slot = var_t5dg12_dn3;
        *var_t5dg12_dn4_slot = var_t5dg12_dn4;
        *var_t5dg12_dn5_slot = var_t5dg12_dn5;
        *var_t5dg12_dn6_slot = var_t5dg12_dn6;
        *var_t5dg12_dn7_slot = var_t5dg12_dn7;
        *var_t5dg12_dn8_slot = var_t5dg12_dn8;
        *var_t5dg12_dn9_slot = var_t5dg12_dn9;
        *var_vds_fp4s_slot = var_vds_fp4s;
        *var_vds_fp4s_dn21_slot = var_vds_fp4s_dn21;
        *var_vds_fp4s_dn22_slot = var_vds_fp4s_dn22;
        *var_vdsat_slot = var_vdsat;
        *var_vdsat_dn0_slot = var_vdsat_dn0;
        *var_vdsat_dn1_slot = var_vdsat_dn1;
        *var_vdsat_dn12_slot = var_vdsat_dn12;
        *var_vdsat_dn14_slot = var_vdsat_dn14;
        *var_vdsat_dn15_slot = var_vdsat_dn15;
        *var_vdsat_dn16_slot = var_vdsat_dn16;
        *var_vdsat_dn17_slot = var_vdsat_dn17;
        *var_vdsat_dn18_slot = var_vdsat_dn18;
        *var_vdsat_dn19_slot = var_vdsat_dn19;
        *var_vdsat_dn2_slot = var_vdsat_dn2;
        *var_vdsat_dn20_slot = var_vdsat_dn20;
        *var_vdsat_dn21_slot = var_vdsat_dn21;
        *var_vdsat_dn22_slot = var_vdsat_dn22;
        *var_vdsat_dn3_slot = var_vdsat_dn3;
        *var_vdsat_dn4_slot = var_vdsat_dn4;
        *var_vdsat_dn5_slot = var_vdsat_dn5;
        *var_vdsat_dn6_slot = var_vdsat_dn6;
        *var_vdsat_dn7_slot = var_vdsat_dn7;
        *var_vdsat_dn8_slot = var_vdsat_dn8;
        *var_vdsat_dn9_slot = var_vdsat_dn9;
        *var_vsat_tdev_slot = var_vsat_tdev;
        *var_vsat_tdev_dn0_slot = var_vsat_tdev_dn0;
        *var_vsat_tdev_dn1_slot = var_vsat_tdev_dn1;
        *var_vsat_tdev_dn12_slot = var_vsat_tdev_dn12;
        *var_vsat_tdev_dn14_slot = var_vsat_tdev_dn14;
        *var_vsat_tdev_dn15_slot = var_vsat_tdev_dn15;
        *var_vsat_tdev_dn16_slot = var_vsat_tdev_dn16;
        *var_vsat_tdev_dn17_slot = var_vsat_tdev_dn17;
        *var_vsat_tdev_dn18_slot = var_vsat_tdev_dn18;
        *var_vsat_tdev_dn19_slot = var_vsat_tdev_dn19;
        *var_vsat_tdev_dn2_slot = var_vsat_tdev_dn2;
        *var_vsat_tdev_dn20_slot = var_vsat_tdev_dn20;
        *var_vsat_tdev_dn21_slot = var_vsat_tdev_dn21;
        *var_vsat_tdev_dn22_slot = var_vsat_tdev_dn22;
        *var_vsat_tdev_dn3_slot = var_vsat_tdev_dn3;
        *var_vsat_tdev_dn4_slot = var_vsat_tdev_dn4;
        *var_vsat_tdev_dn5_slot = var_vsat_tdev_dn5;
        *var_vsat_tdev_dn6_slot = var_vsat_tdev_dn6;
        *var_vsat_tdev_dn7_slot = var_vsat_tdev_dn7;
        *var_vsat_tdev_dn8_slot = var_vsat_tdev_dn8;
        *var_vsat_tdev_dn9_slot = var_vsat_tdev_dn9;
    }

    pub(super) fn stamp_transient_block_177(
        p: &Parameters,
        var_alphad: f64,
        var_alphad_dn15: f64,
        var_alphad_dn16: f64,
        var_alphad_dn17: f64,
        var_alphad_dn18: f64,
        var_alphad_dn19: f64,
        var_alphad_dn20: f64,
        var_alphad_dn21: f64,
        var_alphad_dn22: f64,
        var_alphad_dn4: f64,
        var_alphad_dn6: f64,
        var_alphad_dn7: f64,
        var_alphad_dn8: f64,
        var_alphan: f64,
        var_alphan_dn15: f64,
        var_alphan_dn16: f64,
        var_alphan_dn17: f64,
        var_alphan_dn18: f64,
        var_alphan_dn19: f64,
        var_alphan_dn20: f64,
        var_alphan_dn21: f64,
        var_alphan_dn22: f64,
        var_alphan_dn4: f64,
        var_alphan_dn6: f64,
        var_alphan_dn7: f64,
        var_alphan_dn8: f64,
        var_beta: f64,
        var_beta_dn15: f64,
        var_beta_dn16: f64,
        var_beta_dn17: f64,
        var_beta_dn18: f64,
        var_beta_dn19: f64,
        var_beta_dn20: f64,
        var_beta_dn21: f64,
        var_beta_dn22: f64,
        var_beta_dn4: f64,
        var_beta_dn6: f64,
        var_beta_dn7: f64,
        var_beta_dn8: f64,
        var_cch: f64,
        var_guard504: f64,
        var_guard513: f64,
        var_vds_fp4s: f64,
        var_vds_fp4s_dn21: f64,
        var_vds_fp4s_dn22: f64,
        var_vg0_fp4s: f64,
        var_vg0_fp4s_dn0: f64,
        var_vg0_fp4s_dn1: f64,
        var_vg0_fp4s_dn12: f64,
        var_vg0_fp4s_dn14: f64,
        var_vg0_fp4s_dn15: f64,
        var_vg0_fp4s_dn16: f64,
        var_vg0_fp4s_dn17: f64,
        var_vg0_fp4s_dn18: f64,
        var_vg0_fp4s_dn19: f64,
        var_vg0_fp4s_dn2: f64,
        var_vg0_fp4s_dn20: f64,
        var_vg0_fp4s_dn21: f64,
        var_vg0_fp4s_dn22: f64,
        var_vg0_fp4s_dn3: f64,
        var_vg0_fp4s_dn4: f64,
        var_vg0_fp4s_dn5: f64,
        var_vg0_fp4s_dn6: f64,
        var_vg0_fp4s_dn7: f64,
        var_vg0_fp4s_dn8: f64,
        var_vg0_fp4s_dn9: f64,
        var_vtv: f64,
        var_vtv_dn15: f64,
        var_vtv_dn16: f64,
        var_vtv_dn17: f64,
        var_vtv_dn18: f64,
        var_vtv_dn19: f64,
        var_vtv_dn20: f64,
        var_vtv_dn21: f64,
        var_vtv_dn22: f64,
        var_vtv_dn4: f64,
        var_vtv_dn6: f64,
        var_vtv_dn7: f64,
        var_vtv_dn8: f64,
        var_dvgod_slot: &mut f64,
        var_dvgod_dn0_slot: &mut f64,
        var_dvgod_dn1_slot: &mut f64,
        var_dvgod_dn12_slot: &mut f64,
        var_dvgod_dn14_slot: &mut f64,
        var_dvgod_dn15_slot: &mut f64,
        var_dvgod_dn16_slot: &mut f64,
        var_dvgod_dn17_slot: &mut f64,
        var_dvgod_dn18_slot: &mut f64,
        var_dvgod_dn19_slot: &mut f64,
        var_dvgod_dn2_slot: &mut f64,
        var_dvgod_dn20_slot: &mut f64,
        var_dvgod_dn21_slot: &mut f64,
        var_dvgod_dn22_slot: &mut f64,
        var_dvgod_dn3_slot: &mut f64,
        var_dvgod_dn4_slot: &mut f64,
        var_dvgod_dn5_slot: &mut f64,
        var_dvgod_dn6_slot: &mut f64,
        var_dvgod_dn7_slot: &mut f64,
        var_dvgod_dn8_slot: &mut f64,
        var_dvgod_dn9_slot: &mut f64,
        var_dvgon_slot: &mut f64,
        var_dvgon_dn0_slot: &mut f64,
        var_dvgon_dn1_slot: &mut f64,
        var_dvgon_dn12_slot: &mut f64,
        var_dvgon_dn14_slot: &mut f64,
        var_dvgon_dn15_slot: &mut f64,
        var_dvgon_dn16_slot: &mut f64,
        var_dvgon_dn17_slot: &mut f64,
        var_dvgon_dn18_slot: &mut f64,
        var_dvgon_dn19_slot: &mut f64,
        var_dvgon_dn2_slot: &mut f64,
        var_dvgon_dn20_slot: &mut f64,
        var_dvgon_dn21_slot: &mut f64,
        var_dvgon_dn22_slot: &mut f64,
        var_dvgon_dn3_slot: &mut f64,
        var_dvgon_dn4_slot: &mut f64,
        var_dvgon_dn5_slot: &mut f64,
        var_dvgon_dn6_slot: &mut f64,
        var_dvgon_dn7_slot: &mut f64,
        var_dvgon_dn8_slot: &mut f64,
        var_dvgon_dn9_slot: &mut f64,
        var_ef1_slot: &mut f64,
        var_ef1_dn0_slot: &mut f64,
        var_ef1_dn1_slot: &mut f64,
        var_ef1_dn12_slot: &mut f64,
        var_ef1_dn14_slot: &mut f64,
        var_ef1_dn15_slot: &mut f64,
        var_ef1_dn16_slot: &mut f64,
        var_ef1_dn17_slot: &mut f64,
        var_ef1_dn18_slot: &mut f64,
        var_ef1_dn19_slot: &mut f64,
        var_ef1_dn2_slot: &mut f64,
        var_ef1_dn20_slot: &mut f64,
        var_ef1_dn21_slot: &mut f64,
        var_ef1_dn22_slot: &mut f64,
        var_ef1_dn3_slot: &mut f64,
        var_ef1_dn4_slot: &mut f64,
        var_ef1_dn5_slot: &mut f64,
        var_ef1_dn6_slot: &mut f64,
        var_ef1_dn7_slot: &mut f64,
        var_ef1_dn8_slot: &mut f64,
        var_ef1_dn9_slot: &mut f64,
        var_guard517_slot: &mut f64,
        var_guard518_slot: &mut f64,
        var_hx_slot: &mut f64,
        var_hx_dn0_slot: &mut f64,
        var_hx_dn1_slot: &mut f64,
        var_hx_dn12_slot: &mut f64,
        var_hx_dn14_slot: &mut f64,
        var_hx_dn15_slot: &mut f64,
        var_hx_dn16_slot: &mut f64,
        var_hx_dn17_slot: &mut f64,
        var_hx_dn18_slot: &mut f64,
        var_hx_dn19_slot: &mut f64,
        var_hx_dn2_slot: &mut f64,
        var_hx_dn20_slot: &mut f64,
        var_hx_dn21_slot: &mut f64,
        var_hx_dn22_slot: &mut f64,
        var_hx_dn3_slot: &mut f64,
        var_hx_dn4_slot: &mut f64,
        var_hx_dn5_slot: &mut f64,
        var_hx_dn6_slot: &mut f64,
        var_hx_dn7_slot: &mut f64,
        var_hx_dn8_slot: &mut f64,
        var_hx_dn9_slot: &mut f64,
        var_ndx_slot: &mut f64,
        var_ndx_dn0_slot: &mut f64,
        var_ndx_dn1_slot: &mut f64,
        var_ndx_dn12_slot: &mut f64,
        var_ndx_dn14_slot: &mut f64,
        var_ndx_dn15_slot: &mut f64,
        var_ndx_dn16_slot: &mut f64,
        var_ndx_dn17_slot: &mut f64,
        var_ndx_dn18_slot: &mut f64,
        var_ndx_dn19_slot: &mut f64,
        var_ndx_dn2_slot: &mut f64,
        var_ndx_dn20_slot: &mut f64,
        var_ndx_dn21_slot: &mut f64,
        var_ndx_dn22_slot: &mut f64,
        var_ndx_dn3_slot: &mut f64,
        var_ndx_dn4_slot: &mut f64,
        var_ndx_dn5_slot: &mut f64,
        var_ndx_dn6_slot: &mut f64,
        var_ndx_dn7_slot: &mut f64,
        var_ndx_dn8_slot: &mut f64,
        var_ndx_dn9_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn1_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn14_slot: &mut f64,
        var_t0_dn15_slot: &mut f64,
        var_t0_dn16_slot: &mut f64,
        var_t0_dn17_slot: &mut f64,
        var_t0_dn18_slot: &mut f64,
        var_t0_dn19_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn20_slot: &mut f64,
        var_t0_dn21_slot: &mut f64,
        var_t0_dn22_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn1_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn15_slot: &mut f64,
        var_t1_dn16_slot: &mut f64,
        var_t1_dn17_slot: &mut f64,
        var_t1_dn18_slot: &mut f64,
        var_t1_dn19_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn20_slot: &mut f64,
        var_t1_dn21_slot: &mut f64,
        var_t1_dn22_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn1_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn15_slot: &mut f64,
        var_t2_dn16_slot: &mut f64,
        var_t2_dn17_slot: &mut f64,
        var_t2_dn18_slot: &mut f64,
        var_t2_dn19_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn20_slot: &mut f64,
        var_t2_dn21_slot: &mut f64,
        var_t2_dn22_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_vdeff_slot: &mut f64,
        var_vdeff_dn0_slot: &mut f64,
        var_vdeff_dn1_slot: &mut f64,
        var_vdeff_dn12_slot: &mut f64,
        var_vdeff_dn14_slot: &mut f64,
        var_vdeff_dn15_slot: &mut f64,
        var_vdeff_dn16_slot: &mut f64,
        var_vdeff_dn17_slot: &mut f64,
        var_vdeff_dn18_slot: &mut f64,
        var_vdeff_dn19_slot: &mut f64,
        var_vdeff_dn2_slot: &mut f64,
        var_vdeff_dn20_slot: &mut f64,
        var_vdeff_dn21_slot: &mut f64,
        var_vdeff_dn22_slot: &mut f64,
        var_vdeff_dn3_slot: &mut f64,
        var_vdeff_dn4_slot: &mut f64,
        var_vdeff_dn5_slot: &mut f64,
        var_vdeff_dn6_slot: &mut f64,
        var_vdeff_dn7_slot: &mut f64,
        var_vdeff_dn8_slot: &mut f64,
        var_vdeff_dn9_slot: &mut f64,
        var_vgdeff_slot: &mut f64,
        var_vgdeff_dn0_slot: &mut f64,
        var_vgdeff_dn1_slot: &mut f64,
        var_vgdeff_dn12_slot: &mut f64,
        var_vgdeff_dn14_slot: &mut f64,
        var_vgdeff_dn15_slot: &mut f64,
        var_vgdeff_dn16_slot: &mut f64,
        var_vgdeff_dn17_slot: &mut f64,
        var_vgdeff_dn18_slot: &mut f64,
        var_vgdeff_dn19_slot: &mut f64,
        var_vgdeff_dn2_slot: &mut f64,
        var_vgdeff_dn20_slot: &mut f64,
        var_vgdeff_dn21_slot: &mut f64,
        var_vgdeff_dn22_slot: &mut f64,
        var_vgdeff_dn3_slot: &mut f64,
        var_vgdeff_dn4_slot: &mut f64,
        var_vgdeff_dn5_slot: &mut f64,
        var_vgdeff_dn6_slot: &mut f64,
        var_vgdeff_dn7_slot: &mut f64,
        var_vgdeff_dn8_slot: &mut f64,
        var_vgdeff_dn9_slot: &mut f64,
        var_vgod_slot: &mut f64,
        var_vgod_dn0_slot: &mut f64,
        var_vgod_dn1_slot: &mut f64,
        var_vgod_dn12_slot: &mut f64,
        var_vgod_dn14_slot: &mut f64,
        var_vgod_dn15_slot: &mut f64,
        var_vgod_dn16_slot: &mut f64,
        var_vgod_dn17_slot: &mut f64,
        var_vgod_dn18_slot: &mut f64,
        var_vgod_dn19_slot: &mut f64,
        var_vgod_dn2_slot: &mut f64,
        var_vgod_dn20_slot: &mut f64,
        var_vgod_dn21_slot: &mut f64,
        var_vgod_dn22_slot: &mut f64,
        var_vgod_dn3_slot: &mut f64,
        var_vgod_dn4_slot: &mut f64,
        var_vgod_dn5_slot: &mut f64,
        var_vgod_dn6_slot: &mut f64,
        var_vgod_dn7_slot: &mut f64,
        var_vgod_dn8_slot: &mut f64,
        var_vgod_dn9_slot: &mut f64,
        var_vgodp_slot: &mut f64,
        var_vgodp_dn0_slot: &mut f64,
        var_vgodp_dn1_slot: &mut f64,
        var_vgodp_dn12_slot: &mut f64,
        var_vgodp_dn14_slot: &mut f64,
        var_vgodp_dn15_slot: &mut f64,
        var_vgodp_dn16_slot: &mut f64,
        var_vgodp_dn17_slot: &mut f64,
        var_vgodp_dn18_slot: &mut f64,
        var_vgodp_dn19_slot: &mut f64,
        var_vgodp_dn2_slot: &mut f64,
        var_vgodp_dn20_slot: &mut f64,
        var_vgodp_dn21_slot: &mut f64,
        var_vgodp_dn22_slot: &mut f64,
        var_vgodp_dn3_slot: &mut f64,
        var_vgodp_dn4_slot: &mut f64,
        var_vgodp_dn5_slot: &mut f64,
        var_vgodp_dn6_slot: &mut f64,
        var_vgodp_dn7_slot: &mut f64,
        var_vgodp_dn8_slot: &mut f64,
        var_vgodp_dn9_slot: &mut f64,
        var_vgop_slot: &mut f64,
        var_vgop_dn0_slot: &mut f64,
        var_vgop_dn1_slot: &mut f64,
        var_vgop_dn12_slot: &mut f64,
        var_vgop_dn14_slot: &mut f64,
        var_vgop_dn15_slot: &mut f64,
        var_vgop_dn16_slot: &mut f64,
        var_vgop_dn17_slot: &mut f64,
        var_vgop_dn18_slot: &mut f64,
        var_vgop_dn19_slot: &mut f64,
        var_vgop_dn2_slot: &mut f64,
        var_vgop_dn20_slot: &mut f64,
        var_vgop_dn21_slot: &mut f64,
        var_vgop_dn22_slot: &mut f64,
        var_vgop_dn3_slot: &mut f64,
        var_vgop_dn4_slot: &mut f64,
        var_vgop_dn5_slot: &mut f64,
        var_vgop_dn6_slot: &mut f64,
        var_vgop_dn7_slot: &mut f64,
        var_vgop_dn8_slot: &mut f64,
        var_vgop_dn9_slot: &mut f64,
    ) {
        let mut var_dvgod: f64 = *var_dvgod_slot;
        let mut var_dvgod_dn0: f64 = *var_dvgod_dn0_slot;
        let mut var_dvgod_dn1: f64 = *var_dvgod_dn1_slot;
        let mut var_dvgod_dn12: f64 = *var_dvgod_dn12_slot;
        let mut var_dvgod_dn14: f64 = *var_dvgod_dn14_slot;
        let mut var_dvgod_dn15: f64 = *var_dvgod_dn15_slot;
        let mut var_dvgod_dn16: f64 = *var_dvgod_dn16_slot;
        let mut var_dvgod_dn17: f64 = *var_dvgod_dn17_slot;
        let mut var_dvgod_dn18: f64 = *var_dvgod_dn18_slot;
        let mut var_dvgod_dn19: f64 = *var_dvgod_dn19_slot;
        let mut var_dvgod_dn2: f64 = *var_dvgod_dn2_slot;
        let mut var_dvgod_dn20: f64 = *var_dvgod_dn20_slot;
        let mut var_dvgod_dn21: f64 = *var_dvgod_dn21_slot;
        let mut var_dvgod_dn22: f64 = *var_dvgod_dn22_slot;
        let mut var_dvgod_dn3: f64 = *var_dvgod_dn3_slot;
        let mut var_dvgod_dn4: f64 = *var_dvgod_dn4_slot;
        let mut var_dvgod_dn5: f64 = *var_dvgod_dn5_slot;
        let mut var_dvgod_dn6: f64 = *var_dvgod_dn6_slot;
        let mut var_dvgod_dn7: f64 = *var_dvgod_dn7_slot;
        let mut var_dvgod_dn8: f64 = *var_dvgod_dn8_slot;
        let mut var_dvgod_dn9: f64 = *var_dvgod_dn9_slot;
        let mut var_dvgon: f64 = *var_dvgon_slot;
        let mut var_dvgon_dn0: f64 = *var_dvgon_dn0_slot;
        let mut var_dvgon_dn1: f64 = *var_dvgon_dn1_slot;
        let mut var_dvgon_dn12: f64 = *var_dvgon_dn12_slot;
        let mut var_dvgon_dn14: f64 = *var_dvgon_dn14_slot;
        let mut var_dvgon_dn15: f64 = *var_dvgon_dn15_slot;
        let mut var_dvgon_dn16: f64 = *var_dvgon_dn16_slot;
        let mut var_dvgon_dn17: f64 = *var_dvgon_dn17_slot;
        let mut var_dvgon_dn18: f64 = *var_dvgon_dn18_slot;
        let mut var_dvgon_dn19: f64 = *var_dvgon_dn19_slot;
        let mut var_dvgon_dn2: f64 = *var_dvgon_dn2_slot;
        let mut var_dvgon_dn20: f64 = *var_dvgon_dn20_slot;
        let mut var_dvgon_dn21: f64 = *var_dvgon_dn21_slot;
        let mut var_dvgon_dn22: f64 = *var_dvgon_dn22_slot;
        let mut var_dvgon_dn3: f64 = *var_dvgon_dn3_slot;
        let mut var_dvgon_dn4: f64 = *var_dvgon_dn4_slot;
        let mut var_dvgon_dn5: f64 = *var_dvgon_dn5_slot;
        let mut var_dvgon_dn6: f64 = *var_dvgon_dn6_slot;
        let mut var_dvgon_dn7: f64 = *var_dvgon_dn7_slot;
        let mut var_dvgon_dn8: f64 = *var_dvgon_dn8_slot;
        let mut var_dvgon_dn9: f64 = *var_dvgon_dn9_slot;
        let mut var_ef1: f64 = *var_ef1_slot;
        let mut var_ef1_dn0: f64 = *var_ef1_dn0_slot;
        let mut var_ef1_dn1: f64 = *var_ef1_dn1_slot;
        let mut var_ef1_dn12: f64 = *var_ef1_dn12_slot;
        let mut var_ef1_dn14: f64 = *var_ef1_dn14_slot;
        let mut var_ef1_dn15: f64 = *var_ef1_dn15_slot;
        let mut var_ef1_dn16: f64 = *var_ef1_dn16_slot;
        let mut var_ef1_dn17: f64 = *var_ef1_dn17_slot;
        let mut var_ef1_dn18: f64 = *var_ef1_dn18_slot;
        let mut var_ef1_dn19: f64 = *var_ef1_dn19_slot;
        let mut var_ef1_dn2: f64 = *var_ef1_dn2_slot;
        let mut var_ef1_dn20: f64 = *var_ef1_dn20_slot;
        let mut var_ef1_dn21: f64 = *var_ef1_dn21_slot;
        let mut var_ef1_dn22: f64 = *var_ef1_dn22_slot;
        let mut var_ef1_dn3: f64 = *var_ef1_dn3_slot;
        let mut var_ef1_dn4: f64 = *var_ef1_dn4_slot;
        let mut var_ef1_dn5: f64 = *var_ef1_dn5_slot;
        let mut var_ef1_dn6: f64 = *var_ef1_dn6_slot;
        let mut var_ef1_dn7: f64 = *var_ef1_dn7_slot;
        let mut var_ef1_dn8: f64 = *var_ef1_dn8_slot;
        let mut var_ef1_dn9: f64 = *var_ef1_dn9_slot;
        let mut var_guard517: f64 = *var_guard517_slot;
        let mut var_guard518: f64 = *var_guard518_slot;
        let mut var_hx: f64 = *var_hx_slot;
        let mut var_hx_dn0: f64 = *var_hx_dn0_slot;
        let mut var_hx_dn1: f64 = *var_hx_dn1_slot;
        let mut var_hx_dn12: f64 = *var_hx_dn12_slot;
        let mut var_hx_dn14: f64 = *var_hx_dn14_slot;
        let mut var_hx_dn15: f64 = *var_hx_dn15_slot;
        let mut var_hx_dn16: f64 = *var_hx_dn16_slot;
        let mut var_hx_dn17: f64 = *var_hx_dn17_slot;
        let mut var_hx_dn18: f64 = *var_hx_dn18_slot;
        let mut var_hx_dn19: f64 = *var_hx_dn19_slot;
        let mut var_hx_dn2: f64 = *var_hx_dn2_slot;
        let mut var_hx_dn20: f64 = *var_hx_dn20_slot;
        let mut var_hx_dn21: f64 = *var_hx_dn21_slot;
        let mut var_hx_dn22: f64 = *var_hx_dn22_slot;
        let mut var_hx_dn3: f64 = *var_hx_dn3_slot;
        let mut var_hx_dn4: f64 = *var_hx_dn4_slot;
        let mut var_hx_dn5: f64 = *var_hx_dn5_slot;
        let mut var_hx_dn6: f64 = *var_hx_dn6_slot;
        let mut var_hx_dn7: f64 = *var_hx_dn7_slot;
        let mut var_hx_dn8: f64 = *var_hx_dn8_slot;
        let mut var_hx_dn9: f64 = *var_hx_dn9_slot;
        let mut var_ndx: f64 = *var_ndx_slot;
        let mut var_ndx_dn0: f64 = *var_ndx_dn0_slot;
        let mut var_ndx_dn1: f64 = *var_ndx_dn1_slot;
        let mut var_ndx_dn12: f64 = *var_ndx_dn12_slot;
        let mut var_ndx_dn14: f64 = *var_ndx_dn14_slot;
        let mut var_ndx_dn15: f64 = *var_ndx_dn15_slot;
        let mut var_ndx_dn16: f64 = *var_ndx_dn16_slot;
        let mut var_ndx_dn17: f64 = *var_ndx_dn17_slot;
        let mut var_ndx_dn18: f64 = *var_ndx_dn18_slot;
        let mut var_ndx_dn19: f64 = *var_ndx_dn19_slot;
        let mut var_ndx_dn2: f64 = *var_ndx_dn2_slot;
        let mut var_ndx_dn20: f64 = *var_ndx_dn20_slot;
        let mut var_ndx_dn21: f64 = *var_ndx_dn21_slot;
        let mut var_ndx_dn22: f64 = *var_ndx_dn22_slot;
        let mut var_ndx_dn3: f64 = *var_ndx_dn3_slot;
        let mut var_ndx_dn4: f64 = *var_ndx_dn4_slot;
        let mut var_ndx_dn5: f64 = *var_ndx_dn5_slot;
        let mut var_ndx_dn6: f64 = *var_ndx_dn6_slot;
        let mut var_ndx_dn7: f64 = *var_ndx_dn7_slot;
        let mut var_ndx_dn8: f64 = *var_ndx_dn8_slot;
        let mut var_ndx_dn9: f64 = *var_ndx_dn9_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn1: f64 = *var_t0_dn1_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn14: f64 = *var_t0_dn14_slot;
        let mut var_t0_dn15: f64 = *var_t0_dn15_slot;
        let mut var_t0_dn16: f64 = *var_t0_dn16_slot;
        let mut var_t0_dn17: f64 = *var_t0_dn17_slot;
        let mut var_t0_dn18: f64 = *var_t0_dn18_slot;
        let mut var_t0_dn19: f64 = *var_t0_dn19_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn20: f64 = *var_t0_dn20_slot;
        let mut var_t0_dn21: f64 = *var_t0_dn21_slot;
        let mut var_t0_dn22: f64 = *var_t0_dn22_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn1: f64 = *var_t1_dn1_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn15: f64 = *var_t1_dn15_slot;
        let mut var_t1_dn16: f64 = *var_t1_dn16_slot;
        let mut var_t1_dn17: f64 = *var_t1_dn17_slot;
        let mut var_t1_dn18: f64 = *var_t1_dn18_slot;
        let mut var_t1_dn19: f64 = *var_t1_dn19_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn20: f64 = *var_t1_dn20_slot;
        let mut var_t1_dn21: f64 = *var_t1_dn21_slot;
        let mut var_t1_dn22: f64 = *var_t1_dn22_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn1: f64 = *var_t2_dn1_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn15: f64 = *var_t2_dn15_slot;
        let mut var_t2_dn16: f64 = *var_t2_dn16_slot;
        let mut var_t2_dn17: f64 = *var_t2_dn17_slot;
        let mut var_t2_dn18: f64 = *var_t2_dn18_slot;
        let mut var_t2_dn19: f64 = *var_t2_dn19_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn20: f64 = *var_t2_dn20_slot;
        let mut var_t2_dn21: f64 = *var_t2_dn21_slot;
        let mut var_t2_dn22: f64 = *var_t2_dn22_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_vdeff: f64 = *var_vdeff_slot;
        let mut var_vdeff_dn0: f64 = *var_vdeff_dn0_slot;
        let mut var_vdeff_dn1: f64 = *var_vdeff_dn1_slot;
        let mut var_vdeff_dn12: f64 = *var_vdeff_dn12_slot;
        let mut var_vdeff_dn14: f64 = *var_vdeff_dn14_slot;
        let mut var_vdeff_dn15: f64 = *var_vdeff_dn15_slot;
        let mut var_vdeff_dn16: f64 = *var_vdeff_dn16_slot;
        let mut var_vdeff_dn17: f64 = *var_vdeff_dn17_slot;
        let mut var_vdeff_dn18: f64 = *var_vdeff_dn18_slot;
        let mut var_vdeff_dn19: f64 = *var_vdeff_dn19_slot;
        let mut var_vdeff_dn2: f64 = *var_vdeff_dn2_slot;
        let mut var_vdeff_dn20: f64 = *var_vdeff_dn20_slot;
        let mut var_vdeff_dn21: f64 = *var_vdeff_dn21_slot;
        let mut var_vdeff_dn22: f64 = *var_vdeff_dn22_slot;
        let mut var_vdeff_dn3: f64 = *var_vdeff_dn3_slot;
        let mut var_vdeff_dn4: f64 = *var_vdeff_dn4_slot;
        let mut var_vdeff_dn5: f64 = *var_vdeff_dn5_slot;
        let mut var_vdeff_dn6: f64 = *var_vdeff_dn6_slot;
        let mut var_vdeff_dn7: f64 = *var_vdeff_dn7_slot;
        let mut var_vdeff_dn8: f64 = *var_vdeff_dn8_slot;
        let mut var_vdeff_dn9: f64 = *var_vdeff_dn9_slot;
        let mut var_vgdeff: f64 = *var_vgdeff_slot;
        let mut var_vgdeff_dn0: f64 = *var_vgdeff_dn0_slot;
        let mut var_vgdeff_dn1: f64 = *var_vgdeff_dn1_slot;
        let mut var_vgdeff_dn12: f64 = *var_vgdeff_dn12_slot;
        let mut var_vgdeff_dn14: f64 = *var_vgdeff_dn14_slot;
        let mut var_vgdeff_dn15: f64 = *var_vgdeff_dn15_slot;
        let mut var_vgdeff_dn16: f64 = *var_vgdeff_dn16_slot;
        let mut var_vgdeff_dn17: f64 = *var_vgdeff_dn17_slot;
        let mut var_vgdeff_dn18: f64 = *var_vgdeff_dn18_slot;
        let mut var_vgdeff_dn19: f64 = *var_vgdeff_dn19_slot;
        let mut var_vgdeff_dn2: f64 = *var_vgdeff_dn2_slot;
        let mut var_vgdeff_dn20: f64 = *var_vgdeff_dn20_slot;
        let mut var_vgdeff_dn21: f64 = *var_vgdeff_dn21_slot;
        let mut var_vgdeff_dn22: f64 = *var_vgdeff_dn22_slot;
        let mut var_vgdeff_dn3: f64 = *var_vgdeff_dn3_slot;
        let mut var_vgdeff_dn4: f64 = *var_vgdeff_dn4_slot;
        let mut var_vgdeff_dn5: f64 = *var_vgdeff_dn5_slot;
        let mut var_vgdeff_dn6: f64 = *var_vgdeff_dn6_slot;
        let mut var_vgdeff_dn7: f64 = *var_vgdeff_dn7_slot;
        let mut var_vgdeff_dn8: f64 = *var_vgdeff_dn8_slot;
        let mut var_vgdeff_dn9: f64 = *var_vgdeff_dn9_slot;
        let mut var_vgod: f64 = *var_vgod_slot;
        let mut var_vgod_dn0: f64 = *var_vgod_dn0_slot;
        let mut var_vgod_dn1: f64 = *var_vgod_dn1_slot;
        let mut var_vgod_dn12: f64 = *var_vgod_dn12_slot;
        let mut var_vgod_dn14: f64 = *var_vgod_dn14_slot;
        let mut var_vgod_dn15: f64 = *var_vgod_dn15_slot;
        let mut var_vgod_dn16: f64 = *var_vgod_dn16_slot;
        let mut var_vgod_dn17: f64 = *var_vgod_dn17_slot;
        let mut var_vgod_dn18: f64 = *var_vgod_dn18_slot;
        let mut var_vgod_dn19: f64 = *var_vgod_dn19_slot;
        let mut var_vgod_dn2: f64 = *var_vgod_dn2_slot;
        let mut var_vgod_dn20: f64 = *var_vgod_dn20_slot;
        let mut var_vgod_dn21: f64 = *var_vgod_dn21_slot;
        let mut var_vgod_dn22: f64 = *var_vgod_dn22_slot;
        let mut var_vgod_dn3: f64 = *var_vgod_dn3_slot;
        let mut var_vgod_dn4: f64 = *var_vgod_dn4_slot;
        let mut var_vgod_dn5: f64 = *var_vgod_dn5_slot;
        let mut var_vgod_dn6: f64 = *var_vgod_dn6_slot;
        let mut var_vgod_dn7: f64 = *var_vgod_dn7_slot;
        let mut var_vgod_dn8: f64 = *var_vgod_dn8_slot;
        let mut var_vgod_dn9: f64 = *var_vgod_dn9_slot;
        let mut var_vgodp: f64 = *var_vgodp_slot;
        let mut var_vgodp_dn0: f64 = *var_vgodp_dn0_slot;
        let mut var_vgodp_dn1: f64 = *var_vgodp_dn1_slot;
        let mut var_vgodp_dn12: f64 = *var_vgodp_dn12_slot;
        let mut var_vgodp_dn14: f64 = *var_vgodp_dn14_slot;
        let mut var_vgodp_dn15: f64 = *var_vgodp_dn15_slot;
        let mut var_vgodp_dn16: f64 = *var_vgodp_dn16_slot;
        let mut var_vgodp_dn17: f64 = *var_vgodp_dn17_slot;
        let mut var_vgodp_dn18: f64 = *var_vgodp_dn18_slot;
        let mut var_vgodp_dn19: f64 = *var_vgodp_dn19_slot;
        let mut var_vgodp_dn2: f64 = *var_vgodp_dn2_slot;
        let mut var_vgodp_dn20: f64 = *var_vgodp_dn20_slot;
        let mut var_vgodp_dn21: f64 = *var_vgodp_dn21_slot;
        let mut var_vgodp_dn22: f64 = *var_vgodp_dn22_slot;
        let mut var_vgodp_dn3: f64 = *var_vgodp_dn3_slot;
        let mut var_vgodp_dn4: f64 = *var_vgodp_dn4_slot;
        let mut var_vgodp_dn5: f64 = *var_vgodp_dn5_slot;
        let mut var_vgodp_dn6: f64 = *var_vgodp_dn6_slot;
        let mut var_vgodp_dn7: f64 = *var_vgodp_dn7_slot;
        let mut var_vgodp_dn8: f64 = *var_vgodp_dn8_slot;
        let mut var_vgodp_dn9: f64 = *var_vgodp_dn9_slot;
        let mut var_vgop: f64 = *var_vgop_slot;
        let mut var_vgop_dn0: f64 = *var_vgop_dn0_slot;
        let mut var_vgop_dn1: f64 = *var_vgop_dn1_slot;
        let mut var_vgop_dn12: f64 = *var_vgop_dn12_slot;
        let mut var_vgop_dn14: f64 = *var_vgop_dn14_slot;
        let mut var_vgop_dn15: f64 = *var_vgop_dn15_slot;
        let mut var_vgop_dn16: f64 = *var_vgop_dn16_slot;
        let mut var_vgop_dn17: f64 = *var_vgop_dn17_slot;
        let mut var_vgop_dn18: f64 = *var_vgop_dn18_slot;
        let mut var_vgop_dn19: f64 = *var_vgop_dn19_slot;
        let mut var_vgop_dn2: f64 = *var_vgop_dn2_slot;
        let mut var_vgop_dn20: f64 = *var_vgop_dn20_slot;
        let mut var_vgop_dn21: f64 = *var_vgop_dn21_slot;
        let mut var_vgop_dn22: f64 = *var_vgop_dn22_slot;
        let mut var_vgop_dn3: f64 = *var_vgop_dn3_slot;
        let mut var_vgop_dn4: f64 = *var_vgop_dn4_slot;
        let mut var_vgop_dn5: f64 = *var_vgop_dn5_slot;
        let mut var_vgop_dn6: f64 = *var_vgop_dn6_slot;
        let mut var_vgop_dn7: f64 = *var_vgop_dn7_slot;
        let mut var_vgop_dn8: f64 = *var_vgop_dn8_slot;
        let mut var_vgop_dn9: f64 = *var_vgop_dn9_slot;

        let (assign30170_e47576, assign30170_e47576_d_n0, assign30170_e47576_d_n1, assign30170_e47576_d_n2, assign30170_e47576_d_n3, assign30170_e47576_d_n4, assign30170_e47576_d_n5, assign30170_e47576_d_n6, assign30170_e47576_d_n7, assign30170_e47576_d_n8, assign30170_e47576_d_n9, assign30170_e47576_d_n12, assign30170_e47576_d_n14, assign30170_e47576_d_n15, assign30170_e47576_d_n16, assign30170_e47576_d_n17, assign30170_e47576_d_n18, assign30170_e47576_d_n19, assign30170_e47576_d_n20, assign30170_e47576_d_n21, assign30170_e47576_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30170_e47574: f64 = (var_vds_fp4s * var_t1);
        (assign30170_e47574, (var_vds_fp4s * var_t1_dn0), (var_vds_fp4s * var_t1_dn1), (var_vds_fp4s * var_t1_dn2), (var_vds_fp4s * var_t1_dn3), (var_vds_fp4s * var_t1_dn4), (var_vds_fp4s * var_t1_dn5), (var_vds_fp4s * var_t1_dn6), (var_vds_fp4s * var_t1_dn7), (var_vds_fp4s * var_t1_dn8), (var_vds_fp4s * var_t1_dn9), (var_vds_fp4s * var_t1_dn12), (var_vds_fp4s * var_t1_dn14), (var_vds_fp4s * var_t1_dn15), (var_vds_fp4s * var_t1_dn16), (var_vds_fp4s * var_t1_dn17), (var_vds_fp4s * var_t1_dn18), (var_vds_fp4s * var_t1_dn19), (var_vds_fp4s * var_t1_dn20), ((var_vds_fp4s_dn21 * var_t1) + (var_vds_fp4s * var_t1_dn21)), ((var_vds_fp4s_dn22 * var_t1) + (var_vds_fp4s * var_t1_dn22)),)
    } else {
        (var_vdeff, var_vdeff_dn0, var_vdeff_dn1, var_vdeff_dn2, var_vdeff_dn3, var_vdeff_dn4, var_vdeff_dn5, var_vdeff_dn6, var_vdeff_dn7, var_vdeff_dn8, var_vdeff_dn9, var_vdeff_dn12, var_vdeff_dn14, var_vdeff_dn15, var_vdeff_dn16, var_vdeff_dn17, var_vdeff_dn18, var_vdeff_dn19, var_vdeff_dn20, var_vdeff_dn21, var_vdeff_dn22,)
    }
};
        var_vdeff = assign30170_e47576;
        var_vdeff_dn0 = assign30170_e47576_d_n0;
        var_vdeff_dn1 = assign30170_e47576_d_n1;
        var_vdeff_dn2 = assign30170_e47576_d_n2;
        var_vdeff_dn3 = assign30170_e47576_d_n3;
        var_vdeff_dn4 = assign30170_e47576_d_n4;
        var_vdeff_dn5 = assign30170_e47576_d_n5;
        var_vdeff_dn6 = assign30170_e47576_d_n6;
        var_vdeff_dn7 = assign30170_e47576_d_n7;
        var_vdeff_dn8 = assign30170_e47576_d_n8;
        var_vdeff_dn9 = assign30170_e47576_d_n9;
        var_vdeff_dn12 = assign30170_e47576_d_n12;
        var_vdeff_dn14 = assign30170_e47576_d_n14;
        var_vdeff_dn15 = assign30170_e47576_d_n15;
        var_vdeff_dn16 = assign30170_e47576_d_n16;
        var_vdeff_dn17 = assign30170_e47576_d_n17;
        var_vdeff_dn18 = assign30170_e47576_d_n18;
        var_vdeff_dn19 = assign30170_e47576_d_n19;
        var_vdeff_dn20 = assign30170_e47576_d_n20;
        var_vdeff_dn21 = assign30170_e47576_d_n21;
        var_vdeff_dn22 = assign30170_e47576_d_n22;

        let (assign30180_e47585, assign30180_e47585_d_n0, assign30180_e47585_d_n1, assign30180_e47585_d_n2, assign30180_e47585_d_n3, assign30180_e47585_d_n4, assign30180_e47585_d_n5, assign30180_e47585_d_n6, assign30180_e47585_d_n7, assign30180_e47585_d_n8, assign30180_e47585_d_n9, assign30180_e47585_d_n12, assign30180_e47585_d_n14, assign30180_e47585_d_n15, assign30180_e47585_d_n16, assign30180_e47585_d_n17, assign30180_e47585_d_n18, assign30180_e47585_d_n19, assign30180_e47585_d_n20, assign30180_e47585_d_n21, assign30180_e47585_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30180_e47583: f64 = (var_vg0_fp4s - var_vdeff);
        (assign30180_e47583, (var_vg0_fp4s_dn0 - var_vdeff_dn0), (var_vg0_fp4s_dn1 - var_vdeff_dn1), (var_vg0_fp4s_dn2 - var_vdeff_dn2), (var_vg0_fp4s_dn3 - var_vdeff_dn3), (var_vg0_fp4s_dn4 - var_vdeff_dn4), (var_vg0_fp4s_dn5 - var_vdeff_dn5), (var_vg0_fp4s_dn6 - var_vdeff_dn6), (var_vg0_fp4s_dn7 - var_vdeff_dn7), (var_vg0_fp4s_dn8 - var_vdeff_dn8), (var_vg0_fp4s_dn9 - var_vdeff_dn9), (var_vg0_fp4s_dn12 - var_vdeff_dn12), (var_vg0_fp4s_dn14 - var_vdeff_dn14), (var_vg0_fp4s_dn15 - var_vdeff_dn15), (var_vg0_fp4s_dn16 - var_vdeff_dn16), (var_vg0_fp4s_dn17 - var_vdeff_dn17), (var_vg0_fp4s_dn18 - var_vdeff_dn18), (var_vg0_fp4s_dn19 - var_vdeff_dn19), (var_vg0_fp4s_dn20 - var_vdeff_dn20), (var_vg0_fp4s_dn21 - var_vdeff_dn21), (var_vg0_fp4s_dn22 - var_vdeff_dn22),)
    } else {
        (var_vgdeff, var_vgdeff_dn0, var_vgdeff_dn1, var_vgdeff_dn2, var_vgdeff_dn3, var_vgdeff_dn4, var_vgdeff_dn5, var_vgdeff_dn6, var_vgdeff_dn7, var_vgdeff_dn8, var_vgdeff_dn9, var_vgdeff_dn12, var_vgdeff_dn14, var_vgdeff_dn15, var_vgdeff_dn16, var_vgdeff_dn17, var_vgdeff_dn18, var_vgdeff_dn19, var_vgdeff_dn20, var_vgdeff_dn21, var_vgdeff_dn22,)
    }
};
        var_vgdeff = assign30180_e47585;
        var_vgdeff_dn0 = assign30180_e47585_d_n0;
        var_vgdeff_dn1 = assign30180_e47585_d_n1;
        var_vgdeff_dn2 = assign30180_e47585_d_n2;
        var_vgdeff_dn3 = assign30180_e47585_d_n3;
        var_vgdeff_dn4 = assign30180_e47585_d_n4;
        var_vgdeff_dn5 = assign30180_e47585_d_n5;
        var_vgdeff_dn6 = assign30180_e47585_d_n6;
        var_vgdeff_dn7 = assign30180_e47585_d_n7;
        var_vgdeff_dn8 = assign30180_e47585_d_n8;
        var_vgdeff_dn9 = assign30180_e47585_d_n9;
        var_vgdeff_dn12 = assign30180_e47585_d_n12;
        var_vgdeff_dn14 = assign30180_e47585_d_n14;
        var_vgdeff_dn15 = assign30180_e47585_d_n15;
        var_vgdeff_dn16 = assign30180_e47585_d_n16;
        var_vgdeff_dn17 = assign30180_e47585_d_n17;
        var_vgdeff_dn18 = assign30180_e47585_d_n18;
        var_vgdeff_dn19 = assign30180_e47585_d_n19;
        var_vgdeff_dn20 = assign30180_e47585_d_n20;
        var_vgdeff_dn21 = assign30180_e47585_d_n21;
        var_vgdeff_dn22 = assign30180_e47585_d_n22;

        let (assign30190_e47592, assign30190_e47592_d_n0, assign30190_e47592_d_n1, assign30190_e47592_d_n2, assign30190_e47592_d_n3, assign30190_e47592_d_n4, assign30190_e47592_d_n5, assign30190_e47592_d_n6, assign30190_e47592_d_n7, assign30190_e47592_d_n8, assign30190_e47592_d_n9, assign30190_e47592_d_n12, assign30190_e47592_d_n14, assign30190_e47592_d_n15, assign30190_e47592_d_n16, assign30190_e47592_d_n17, assign30190_e47592_d_n18, assign30190_e47592_d_n19, assign30190_e47592_d_n20, assign30190_e47592_d_n21, assign30190_e47592_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        (var_vgdeff, var_vgdeff_dn0, var_vgdeff_dn1, var_vgdeff_dn2, var_vgdeff_dn3, var_vgdeff_dn4, var_vgdeff_dn5, var_vgdeff_dn6, var_vgdeff_dn7, var_vgdeff_dn8, var_vgdeff_dn9, var_vgdeff_dn12, var_vgdeff_dn14, var_vgdeff_dn15, var_vgdeff_dn16, var_vgdeff_dn17, var_vgdeff_dn18, var_vgdeff_dn19, var_vgdeff_dn20, var_vgdeff_dn21, var_vgdeff_dn22,)
    } else {
        (var_vgod, var_vgod_dn0, var_vgod_dn1, var_vgod_dn2, var_vgod_dn3, var_vgod_dn4, var_vgod_dn5, var_vgod_dn6, var_vgod_dn7, var_vgod_dn8, var_vgod_dn9, var_vgod_dn12, var_vgod_dn14, var_vgod_dn15, var_vgod_dn16, var_vgod_dn17, var_vgod_dn18, var_vgod_dn19, var_vgod_dn20, var_vgod_dn21, var_vgod_dn22,)
    }
};
        var_vgod = assign30190_e47592;
        var_vgod_dn0 = assign30190_e47592_d_n0;
        var_vgod_dn1 = assign30190_e47592_d_n1;
        var_vgod_dn2 = assign30190_e47592_d_n2;
        var_vgod_dn3 = assign30190_e47592_d_n3;
        var_vgod_dn4 = assign30190_e47592_d_n4;
        var_vgod_dn5 = assign30190_e47592_d_n5;
        var_vgod_dn6 = assign30190_e47592_d_n6;
        var_vgod_dn7 = assign30190_e47592_d_n7;
        var_vgod_dn8 = assign30190_e47592_d_n8;
        var_vgod_dn9 = assign30190_e47592_d_n9;
        var_vgod_dn12 = assign30190_e47592_d_n12;
        var_vgod_dn14 = assign30190_e47592_d_n14;
        var_vgod_dn15 = assign30190_e47592_d_n15;
        var_vgod_dn16 = assign30190_e47592_d_n16;
        var_vgod_dn17 = assign30190_e47592_d_n17;
        var_vgod_dn18 = assign30190_e47592_d_n18;
        var_vgod_dn19 = assign30190_e47592_d_n19;
        var_vgod_dn20 = assign30190_e47592_d_n20;
        var_vgod_dn21 = assign30190_e47592_d_n21;
        var_vgod_dn22 = assign30190_e47592_d_n22;

        let (assign30200_e47614, assign30200_e47614_d_n0, assign30200_e47614_d_n1, assign30200_e47614_d_n2, assign30200_e47614_d_n3, assign30200_e47614_d_n4, assign30200_e47614_d_n5, assign30200_e47614_d_n6, assign30200_e47614_d_n7, assign30200_e47614_d_n8, assign30200_e47614_d_n9, assign30200_e47614_d_n12, assign30200_e47614_d_n14, assign30200_e47614_d_n15, assign30200_e47614_d_n16, assign30200_e47614_d_n17, assign30200_e47614_d_n18, assign30200_e47614_d_n19, assign30200_e47614_d_n20, assign30200_e47614_d_n21, assign30200_e47614_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30200_e47599: f64 = (0.5 * var_vgod);
        let assign30200_e47603: f64 = (var_vgod * var_vgod);
        let assign30200_e47606: f64 = (4.0 * 0.3);
        let assign30200_e47608: f64 = (assign30200_e47606 * 0.3);
        let assign30200_e47609: f64 = (assign30200_e47603 + assign30200_e47608);
        let assign30200_e47610: f64 = (assign30200_e47609).sqrt();
        let assign30200_e47611: f64 = (0.5 * assign30200_e47610);
        let assign30200_e47612: f64 = (assign30200_e47599 + assign30200_e47611);
        (assign30200_e47612, ((0.5 * var_vgod_dn0) + (0.5 * (((var_vgod_dn0 * var_vgod) + (var_vgod * var_vgod_dn0)) / (2.0 * assign30200_e47610)))), ((0.5 * var_vgod_dn1) + (0.5 * (((var_vgod_dn1 * var_vgod) + (var_vgod * var_vgod_dn1)) / (2.0 * assign30200_e47610)))), ((0.5 * var_vgod_dn2) + (0.5 * (((var_vgod_dn2 * var_vgod) + (var_vgod * var_vgod_dn2)) / (2.0 * assign30200_e47610)))), ((0.5 * var_vgod_dn3) + (0.5 * (((var_vgod_dn3 * var_vgod) + (var_vgod * var_vgod_dn3)) / (2.0 * assign30200_e47610)))), ((0.5 * var_vgod_dn4) + (0.5 * (((var_vgod_dn4 * var_vgod) + (var_vgod * var_vgod_dn4)) / (2.0 * assign30200_e47610)))), ((0.5 * var_vgod_dn5) + (0.5 * (((var_vgod_dn5 * var_vgod) + (var_vgod * var_vgod_dn5)) / (2.0 * assign30200_e47610)))), ((0.5 * var_vgod_dn6) + (0.5 * (((var_vgod_dn6 * var_vgod) + (var_vgod * var_vgod_dn6)) / (2.0 * assign30200_e47610)))), ((0.5 * var_vgod_dn7) + (0.5 * (((var_vgod_dn7 * var_vgod) + (var_vgod * var_vgod_dn7)) / (2.0 * assign30200_e47610)))), ((0.5 * var_vgod_dn8) + (0.5 * (((var_vgod_dn8 * var_vgod) + (var_vgod * var_vgod_dn8)) / (2.0 * assign30200_e47610)))), ((0.5 * var_vgod_dn9) + (0.5 * (((var_vgod_dn9 * var_vgod) + (var_vgod * var_vgod_dn9)) / (2.0 * assign30200_e47610)))), ((0.5 * var_vgod_dn12) + (0.5 * (((var_vgod_dn12 * var_vgod) + (var_vgod * var_vgod_dn12)) / (2.0 * assign30200_e47610)))), ((0.5 * var_vgod_dn14) + (0.5 * (((var_vgod_dn14 * var_vgod) + (var_vgod * var_vgod_dn14)) / (2.0 * assign30200_e47610)))), ((0.5 * var_vgod_dn15) + (0.5 * (((var_vgod_dn15 * var_vgod) + (var_vgod * var_vgod_dn15)) / (2.0 * assign30200_e47610)))), ((0.5 * var_vgod_dn16) + (0.5 * (((var_vgod_dn16 * var_vgod) + (var_vgod * var_vgod_dn16)) / (2.0 * assign30200_e47610)))), ((0.5 * var_vgod_dn17) + (0.5 * (((var_vgod_dn17 * var_vgod) + (var_vgod * var_vgod_dn17)) / (2.0 * assign30200_e47610)))), ((0.5 * var_vgod_dn18) + (0.5 * (((var_vgod_dn18 * var_vgod) + (var_vgod * var_vgod_dn18)) / (2.0 * assign30200_e47610)))), ((0.5 * var_vgod_dn19) + (0.5 * (((var_vgod_dn19 * var_vgod) + (var_vgod * var_vgod_dn19)) / (2.0 * assign30200_e47610)))), ((0.5 * var_vgod_dn20) + (0.5 * (((var_vgod_dn20 * var_vgod) + (var_vgod * var_vgod_dn20)) / (2.0 * assign30200_e47610)))), ((0.5 * var_vgod_dn21) + (0.5 * (((var_vgod_dn21 * var_vgod) + (var_vgod * var_vgod_dn21)) / (2.0 * assign30200_e47610)))), ((0.5 * var_vgod_dn22) + (0.5 * (((var_vgod_dn22 * var_vgod) + (var_vgod * var_vgod_dn22)) / (2.0 * assign30200_e47610)))),)
    } else {
        (var_vgodp, var_vgodp_dn0, var_vgodp_dn1, var_vgodp_dn2, var_vgodp_dn3, var_vgodp_dn4, var_vgodp_dn5, var_vgodp_dn6, var_vgodp_dn7, var_vgodp_dn8, var_vgodp_dn9, var_vgodp_dn12, var_vgodp_dn14, var_vgodp_dn15, var_vgodp_dn16, var_vgodp_dn17, var_vgodp_dn18, var_vgodp_dn19, var_vgodp_dn20, var_vgodp_dn21, var_vgodp_dn22,)
    }
};
        var_vgodp = assign30200_e47614;
        var_vgodp_dn0 = assign30200_e47614_d_n0;
        var_vgodp_dn1 = assign30200_e47614_d_n1;
        var_vgodp_dn2 = assign30200_e47614_d_n2;
        var_vgodp_dn3 = assign30200_e47614_d_n3;
        var_vgodp_dn4 = assign30200_e47614_d_n4;
        var_vgodp_dn5 = assign30200_e47614_d_n5;
        var_vgodp_dn6 = assign30200_e47614_d_n6;
        var_vgodp_dn7 = assign30200_e47614_d_n7;
        var_vgodp_dn8 = assign30200_e47614_d_n8;
        var_vgodp_dn9 = assign30200_e47614_d_n9;
        var_vgodp_dn12 = assign30200_e47614_d_n12;
        var_vgodp_dn14 = assign30200_e47614_d_n14;
        var_vgodp_dn15 = assign30200_e47614_d_n15;
        var_vgodp_dn16 = assign30200_e47614_d_n16;
        var_vgodp_dn17 = assign30200_e47614_d_n17;
        var_vgodp_dn18 = assign30200_e47614_d_n18;
        var_vgodp_dn19 = assign30200_e47614_d_n19;
        var_vgodp_dn20 = assign30200_e47614_d_n20;
        var_vgodp_dn21 = assign30200_e47614_d_n21;
        var_vgodp_dn22 = assign30200_e47614_d_n22;

        let (assign30210_e47621, assign30210_e47621_d_n0, assign30210_e47621_d_n1, assign30210_e47621_d_n2, assign30210_e47621_d_n3, assign30210_e47621_d_n4, assign30210_e47621_d_n5, assign30210_e47621_d_n6, assign30210_e47621_d_n7, assign30210_e47621_d_n8, assign30210_e47621_d_n9, assign30210_e47621_d_n12, assign30210_e47621_d_n14, assign30210_e47621_d_n15, assign30210_e47621_d_n16, assign30210_e47621_d_n17, assign30210_e47621_d_n18, assign30210_e47621_d_n19, assign30210_e47621_d_n20, assign30210_e47621_d_n21, assign30210_e47621_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        (var_vgodp, var_vgodp_dn0, var_vgodp_dn1, var_vgodp_dn2, var_vgodp_dn3, var_vgodp_dn4, var_vgodp_dn5, var_vgodp_dn6, var_vgodp_dn7, var_vgodp_dn8, var_vgodp_dn9, var_vgodp_dn12, var_vgodp_dn14, var_vgodp_dn15, var_vgodp_dn16, var_vgodp_dn17, var_vgodp_dn18, var_vgodp_dn19, var_vgodp_dn20, var_vgodp_dn21, var_vgodp_dn22,)
    } else {
        (var_vgop, var_vgop_dn0, var_vgop_dn1, var_vgop_dn2, var_vgop_dn3, var_vgop_dn4, var_vgop_dn5, var_vgop_dn6, var_vgop_dn7, var_vgop_dn8, var_vgop_dn9, var_vgop_dn12, var_vgop_dn14, var_vgop_dn15, var_vgop_dn16, var_vgop_dn17, var_vgop_dn18, var_vgop_dn19, var_vgop_dn20, var_vgop_dn21, var_vgop_dn22,)
    }
};
        var_vgop = assign30210_e47621;
        var_vgop_dn0 = assign30210_e47621_d_n0;
        var_vgop_dn1 = assign30210_e47621_d_n1;
        var_vgop_dn2 = assign30210_e47621_d_n2;
        var_vgop_dn3 = assign30210_e47621_d_n3;
        var_vgop_dn4 = assign30210_e47621_d_n4;
        var_vgop_dn5 = assign30210_e47621_d_n5;
        var_vgop_dn6 = assign30210_e47621_d_n6;
        var_vgop_dn7 = assign30210_e47621_d_n7;
        var_vgop_dn8 = assign30210_e47621_d_n8;
        var_vgop_dn9 = assign30210_e47621_d_n9;
        var_vgop_dn12 = assign30210_e47621_d_n12;
        var_vgop_dn14 = assign30210_e47621_d_n14;
        var_vgop_dn15 = assign30210_e47621_d_n15;
        var_vgop_dn16 = assign30210_e47621_d_n16;
        var_vgop_dn17 = assign30210_e47621_d_n17;
        var_vgop_dn18 = assign30210_e47621_d_n18;
        var_vgop_dn19 = assign30210_e47621_d_n19;
        var_vgop_dn20 = assign30210_e47621_d_n20;
        var_vgop_dn21 = assign30210_e47621_d_n21;
        var_vgop_dn22 = assign30210_e47621_d_n22;

        let (assign30220_e47639, assign30220_e47639_d_n0, assign30220_e47639_d_n1, assign30220_e47639_d_n2, assign30220_e47639_d_n3, assign30220_e47639_d_n4, assign30220_e47639_d_n5, assign30220_e47639_d_n6, assign30220_e47639_d_n7, assign30220_e47639_d_n8, assign30220_e47639_d_n9, assign30220_e47639_d_n12, assign30220_e47639_d_n14, assign30220_e47639_d_n15, assign30220_e47639_d_n16, assign30220_e47639_d_n17, assign30220_e47639_d_n18, assign30220_e47639_d_n19, assign30220_e47639_d_n20, assign30220_e47639_d_n21, assign30220_e47639_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30220_e47628: f64 = (var_vgop * var_alphan);
        let assign30220_e47631: f64 = (var_vgop * var_vgop);
        let assign30220_e47634: f64 = (var_alphan * var_alphan);
        let assign30220_e47635: f64 = (assign30220_e47631 + assign30220_e47634);
        let assign30220_e47636: f64 = (assign30220_e47635).sqrt();
        let assign30220_e47637: f64 = (assign30220_e47628 / assign30220_e47636);
        (assign30220_e47637, ((((var_vgop_dn0 * var_alphan) * assign30220_e47636) - (assign30220_e47628 * (((var_vgop_dn0 * var_vgop) + (var_vgop * var_vgop_dn0)) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), ((((var_vgop_dn1 * var_alphan) * assign30220_e47636) - (assign30220_e47628 * (((var_vgop_dn1 * var_vgop) + (var_vgop * var_vgop_dn1)) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), ((((var_vgop_dn2 * var_alphan) * assign30220_e47636) - (assign30220_e47628 * (((var_vgop_dn2 * var_vgop) + (var_vgop * var_vgop_dn2)) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), ((((var_vgop_dn3 * var_alphan) * assign30220_e47636) - (assign30220_e47628 * (((var_vgop_dn3 * var_vgop) + (var_vgop * var_vgop_dn3)) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), (((((var_vgop_dn4 * var_alphan) + (var_vgop * var_alphan_dn4)) * assign30220_e47636) - (assign30220_e47628 * ((((var_vgop_dn4 * var_vgop) + (var_vgop * var_vgop_dn4)) + ((var_alphan_dn4 * var_alphan) + (var_alphan * var_alphan_dn4))) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), ((((var_vgop_dn5 * var_alphan) * assign30220_e47636) - (assign30220_e47628 * (((var_vgop_dn5 * var_vgop) + (var_vgop * var_vgop_dn5)) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), (((((var_vgop_dn6 * var_alphan) + (var_vgop * var_alphan_dn6)) * assign30220_e47636) - (assign30220_e47628 * ((((var_vgop_dn6 * var_vgop) + (var_vgop * var_vgop_dn6)) + ((var_alphan_dn6 * var_alphan) + (var_alphan * var_alphan_dn6))) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), (((((var_vgop_dn7 * var_alphan) + (var_vgop * var_alphan_dn7)) * assign30220_e47636) - (assign30220_e47628 * ((((var_vgop_dn7 * var_vgop) + (var_vgop * var_vgop_dn7)) + ((var_alphan_dn7 * var_alphan) + (var_alphan * var_alphan_dn7))) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), (((((var_vgop_dn8 * var_alphan) + (var_vgop * var_alphan_dn8)) * assign30220_e47636) - (assign30220_e47628 * ((((var_vgop_dn8 * var_vgop) + (var_vgop * var_vgop_dn8)) + ((var_alphan_dn8 * var_alphan) + (var_alphan * var_alphan_dn8))) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), ((((var_vgop_dn9 * var_alphan) * assign30220_e47636) - (assign30220_e47628 * (((var_vgop_dn9 * var_vgop) + (var_vgop * var_vgop_dn9)) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), ((((var_vgop_dn12 * var_alphan) * assign30220_e47636) - (assign30220_e47628 * (((var_vgop_dn12 * var_vgop) + (var_vgop * var_vgop_dn12)) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), ((((var_vgop_dn14 * var_alphan) * assign30220_e47636) - (assign30220_e47628 * (((var_vgop_dn14 * var_vgop) + (var_vgop * var_vgop_dn14)) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), (((((var_vgop_dn15 * var_alphan) + (var_vgop * var_alphan_dn15)) * assign30220_e47636) - (assign30220_e47628 * ((((var_vgop_dn15 * var_vgop) + (var_vgop * var_vgop_dn15)) + ((var_alphan_dn15 * var_alphan) + (var_alphan * var_alphan_dn15))) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), (((((var_vgop_dn16 * var_alphan) + (var_vgop * var_alphan_dn16)) * assign30220_e47636) - (assign30220_e47628 * ((((var_vgop_dn16 * var_vgop) + (var_vgop * var_vgop_dn16)) + ((var_alphan_dn16 * var_alphan) + (var_alphan * var_alphan_dn16))) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), (((((var_vgop_dn17 * var_alphan) + (var_vgop * var_alphan_dn17)) * assign30220_e47636) - (assign30220_e47628 * ((((var_vgop_dn17 * var_vgop) + (var_vgop * var_vgop_dn17)) + ((var_alphan_dn17 * var_alphan) + (var_alphan * var_alphan_dn17))) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), (((((var_vgop_dn18 * var_alphan) + (var_vgop * var_alphan_dn18)) * assign30220_e47636) - (assign30220_e47628 * ((((var_vgop_dn18 * var_vgop) + (var_vgop * var_vgop_dn18)) + ((var_alphan_dn18 * var_alphan) + (var_alphan * var_alphan_dn18))) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), (((((var_vgop_dn19 * var_alphan) + (var_vgop * var_alphan_dn19)) * assign30220_e47636) - (assign30220_e47628 * ((((var_vgop_dn19 * var_vgop) + (var_vgop * var_vgop_dn19)) + ((var_alphan_dn19 * var_alphan) + (var_alphan * var_alphan_dn19))) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), (((((var_vgop_dn20 * var_alphan) + (var_vgop * var_alphan_dn20)) * assign30220_e47636) - (assign30220_e47628 * ((((var_vgop_dn20 * var_vgop) + (var_vgop * var_vgop_dn20)) + ((var_alphan_dn20 * var_alphan) + (var_alphan * var_alphan_dn20))) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), (((((var_vgop_dn21 * var_alphan) + (var_vgop * var_alphan_dn21)) * assign30220_e47636) - (assign30220_e47628 * ((((var_vgop_dn21 * var_vgop) + (var_vgop * var_vgop_dn21)) + ((var_alphan_dn21 * var_alphan) + (var_alphan * var_alphan_dn21))) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)), (((((var_vgop_dn22 * var_alphan) + (var_vgop * var_alphan_dn22)) * assign30220_e47636) - (assign30220_e47628 * ((((var_vgop_dn22 * var_vgop) + (var_vgop * var_vgop_dn22)) + ((var_alphan_dn22 * var_alphan) + (var_alphan * var_alphan_dn22))) / (2.0 * assign30220_e47636)))) / (assign30220_e47636 * assign30220_e47636)),)
    } else {
        (var_dvgon, var_dvgon_dn0, var_dvgon_dn1, var_dvgon_dn2, var_dvgon_dn3, var_dvgon_dn4, var_dvgon_dn5, var_dvgon_dn6, var_dvgon_dn7, var_dvgon_dn8, var_dvgon_dn9, var_dvgon_dn12, var_dvgon_dn14, var_dvgon_dn15, var_dvgon_dn16, var_dvgon_dn17, var_dvgon_dn18, var_dvgon_dn19, var_dvgon_dn20, var_dvgon_dn21, var_dvgon_dn22,)
    }
};
        var_dvgon = assign30220_e47639;
        var_dvgon_dn0 = assign30220_e47639_d_n0;
        var_dvgon_dn1 = assign30220_e47639_d_n1;
        var_dvgon_dn2 = assign30220_e47639_d_n2;
        var_dvgon_dn3 = assign30220_e47639_d_n3;
        var_dvgon_dn4 = assign30220_e47639_d_n4;
        var_dvgon_dn5 = assign30220_e47639_d_n5;
        var_dvgon_dn6 = assign30220_e47639_d_n6;
        var_dvgon_dn7 = assign30220_e47639_d_n7;
        var_dvgon_dn8 = assign30220_e47639_d_n8;
        var_dvgon_dn9 = assign30220_e47639_d_n9;
        var_dvgon_dn12 = assign30220_e47639_d_n12;
        var_dvgon_dn14 = assign30220_e47639_d_n14;
        var_dvgon_dn15 = assign30220_e47639_d_n15;
        var_dvgon_dn16 = assign30220_e47639_d_n16;
        var_dvgon_dn17 = assign30220_e47639_d_n17;
        var_dvgon_dn18 = assign30220_e47639_d_n18;
        var_dvgon_dn19 = assign30220_e47639_d_n19;
        var_dvgon_dn20 = assign30220_e47639_d_n20;
        var_dvgon_dn21 = assign30220_e47639_d_n21;
        var_dvgon_dn22 = assign30220_e47639_d_n22;

        let (assign30230_e47657, assign30230_e47657_d_n0, assign30230_e47657_d_n1, assign30230_e47657_d_n2, assign30230_e47657_d_n3, assign30230_e47657_d_n4, assign30230_e47657_d_n5, assign30230_e47657_d_n6, assign30230_e47657_d_n7, assign30230_e47657_d_n8, assign30230_e47657_d_n9, assign30230_e47657_d_n12, assign30230_e47657_d_n14, assign30230_e47657_d_n15, assign30230_e47657_d_n16, assign30230_e47657_d_n17, assign30230_e47657_d_n18, assign30230_e47657_d_n19, assign30230_e47657_d_n20, assign30230_e47657_d_n21, assign30230_e47657_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30230_e47646: f64 = (var_vgop * var_alphad);
        let assign30230_e47649: f64 = (var_vgop * var_vgop);
        let assign30230_e47652: f64 = (var_alphad * var_alphad);
        let assign30230_e47653: f64 = (assign30230_e47649 + assign30230_e47652);
        let assign30230_e47654: f64 = (assign30230_e47653).sqrt();
        let assign30230_e47655: f64 = (assign30230_e47646 / assign30230_e47654);
        (assign30230_e47655, ((((var_vgop_dn0 * var_alphad) * assign30230_e47654) - (assign30230_e47646 * (((var_vgop_dn0 * var_vgop) + (var_vgop * var_vgop_dn0)) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), ((((var_vgop_dn1 * var_alphad) * assign30230_e47654) - (assign30230_e47646 * (((var_vgop_dn1 * var_vgop) + (var_vgop * var_vgop_dn1)) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), ((((var_vgop_dn2 * var_alphad) * assign30230_e47654) - (assign30230_e47646 * (((var_vgop_dn2 * var_vgop) + (var_vgop * var_vgop_dn2)) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), ((((var_vgop_dn3 * var_alphad) * assign30230_e47654) - (assign30230_e47646 * (((var_vgop_dn3 * var_vgop) + (var_vgop * var_vgop_dn3)) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), (((((var_vgop_dn4 * var_alphad) + (var_vgop * var_alphad_dn4)) * assign30230_e47654) - (assign30230_e47646 * ((((var_vgop_dn4 * var_vgop) + (var_vgop * var_vgop_dn4)) + ((var_alphad_dn4 * var_alphad) + (var_alphad * var_alphad_dn4))) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), ((((var_vgop_dn5 * var_alphad) * assign30230_e47654) - (assign30230_e47646 * (((var_vgop_dn5 * var_vgop) + (var_vgop * var_vgop_dn5)) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), (((((var_vgop_dn6 * var_alphad) + (var_vgop * var_alphad_dn6)) * assign30230_e47654) - (assign30230_e47646 * ((((var_vgop_dn6 * var_vgop) + (var_vgop * var_vgop_dn6)) + ((var_alphad_dn6 * var_alphad) + (var_alphad * var_alphad_dn6))) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), (((((var_vgop_dn7 * var_alphad) + (var_vgop * var_alphad_dn7)) * assign30230_e47654) - (assign30230_e47646 * ((((var_vgop_dn7 * var_vgop) + (var_vgop * var_vgop_dn7)) + ((var_alphad_dn7 * var_alphad) + (var_alphad * var_alphad_dn7))) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), (((((var_vgop_dn8 * var_alphad) + (var_vgop * var_alphad_dn8)) * assign30230_e47654) - (assign30230_e47646 * ((((var_vgop_dn8 * var_vgop) + (var_vgop * var_vgop_dn8)) + ((var_alphad_dn8 * var_alphad) + (var_alphad * var_alphad_dn8))) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), ((((var_vgop_dn9 * var_alphad) * assign30230_e47654) - (assign30230_e47646 * (((var_vgop_dn9 * var_vgop) + (var_vgop * var_vgop_dn9)) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), ((((var_vgop_dn12 * var_alphad) * assign30230_e47654) - (assign30230_e47646 * (((var_vgop_dn12 * var_vgop) + (var_vgop * var_vgop_dn12)) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), ((((var_vgop_dn14 * var_alphad) * assign30230_e47654) - (assign30230_e47646 * (((var_vgop_dn14 * var_vgop) + (var_vgop * var_vgop_dn14)) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), (((((var_vgop_dn15 * var_alphad) + (var_vgop * var_alphad_dn15)) * assign30230_e47654) - (assign30230_e47646 * ((((var_vgop_dn15 * var_vgop) + (var_vgop * var_vgop_dn15)) + ((var_alphad_dn15 * var_alphad) + (var_alphad * var_alphad_dn15))) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), (((((var_vgop_dn16 * var_alphad) + (var_vgop * var_alphad_dn16)) * assign30230_e47654) - (assign30230_e47646 * ((((var_vgop_dn16 * var_vgop) + (var_vgop * var_vgop_dn16)) + ((var_alphad_dn16 * var_alphad) + (var_alphad * var_alphad_dn16))) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), (((((var_vgop_dn17 * var_alphad) + (var_vgop * var_alphad_dn17)) * assign30230_e47654) - (assign30230_e47646 * ((((var_vgop_dn17 * var_vgop) + (var_vgop * var_vgop_dn17)) + ((var_alphad_dn17 * var_alphad) + (var_alphad * var_alphad_dn17))) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), (((((var_vgop_dn18 * var_alphad) + (var_vgop * var_alphad_dn18)) * assign30230_e47654) - (assign30230_e47646 * ((((var_vgop_dn18 * var_vgop) + (var_vgop * var_vgop_dn18)) + ((var_alphad_dn18 * var_alphad) + (var_alphad * var_alphad_dn18))) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), (((((var_vgop_dn19 * var_alphad) + (var_vgop * var_alphad_dn19)) * assign30230_e47654) - (assign30230_e47646 * ((((var_vgop_dn19 * var_vgop) + (var_vgop * var_vgop_dn19)) + ((var_alphad_dn19 * var_alphad) + (var_alphad * var_alphad_dn19))) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), (((((var_vgop_dn20 * var_alphad) + (var_vgop * var_alphad_dn20)) * assign30230_e47654) - (assign30230_e47646 * ((((var_vgop_dn20 * var_vgop) + (var_vgop * var_vgop_dn20)) + ((var_alphad_dn20 * var_alphad) + (var_alphad * var_alphad_dn20))) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), (((((var_vgop_dn21 * var_alphad) + (var_vgop * var_alphad_dn21)) * assign30230_e47654) - (assign30230_e47646 * ((((var_vgop_dn21 * var_vgop) + (var_vgop * var_vgop_dn21)) + ((var_alphad_dn21 * var_alphad) + (var_alphad * var_alphad_dn21))) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)), (((((var_vgop_dn22 * var_alphad) + (var_vgop * var_alphad_dn22)) * assign30230_e47654) - (assign30230_e47646 * ((((var_vgop_dn22 * var_vgop) + (var_vgop * var_vgop_dn22)) + ((var_alphad_dn22 * var_alphad) + (var_alphad * var_alphad_dn22))) / (2.0 * assign30230_e47654)))) / (assign30230_e47654 * assign30230_e47654)),)
    } else {
        (var_dvgod, var_dvgod_dn0, var_dvgod_dn1, var_dvgod_dn2, var_dvgod_dn3, var_dvgod_dn4, var_dvgod_dn5, var_dvgod_dn6, var_dvgod_dn7, var_dvgod_dn8, var_dvgod_dn9, var_dvgod_dn12, var_dvgod_dn14, var_dvgod_dn15, var_dvgod_dn16, var_dvgod_dn17, var_dvgod_dn18, var_dvgod_dn19, var_dvgod_dn20, var_dvgod_dn21, var_dvgod_dn22,)
    }
};
        var_dvgod = assign30230_e47657;
        var_dvgod_dn0 = assign30230_e47657_d_n0;
        var_dvgod_dn1 = assign30230_e47657_d_n1;
        var_dvgod_dn2 = assign30230_e47657_d_n2;
        var_dvgod_dn3 = assign30230_e47657_d_n3;
        var_dvgod_dn4 = assign30230_e47657_d_n4;
        var_dvgod_dn5 = assign30230_e47657_d_n5;
        var_dvgod_dn6 = assign30230_e47657_d_n6;
        var_dvgod_dn7 = assign30230_e47657_d_n7;
        var_dvgod_dn8 = assign30230_e47657_d_n8;
        var_dvgod_dn9 = assign30230_e47657_d_n9;
        var_dvgod_dn12 = assign30230_e47657_d_n12;
        var_dvgod_dn14 = assign30230_e47657_d_n14;
        var_dvgod_dn15 = assign30230_e47657_d_n15;
        var_dvgod_dn16 = assign30230_e47657_d_n16;
        var_dvgod_dn17 = assign30230_e47657_d_n17;
        var_dvgod_dn18 = assign30230_e47657_d_n18;
        var_dvgod_dn19 = assign30230_e47657_d_n19;
        var_dvgod_dn20 = assign30230_e47657_d_n20;
        var_dvgod_dn21 = assign30230_e47657_d_n21;
        var_dvgod_dn22 = assign30230_e47657_d_n22;

        let (assign30240_e47703, assign30240_e47703_d_n0, assign30240_e47703_d_n1, assign30240_e47703_d_n2, assign30240_e47703_d_n3, assign30240_e47703_d_n4, assign30240_e47703_d_n5, assign30240_e47703_d_n6, assign30240_e47703_d_n7, assign30240_e47703_d_n8, assign30240_e47703_d_n9, assign30240_e47703_d_n12, assign30240_e47703_d_n14, assign30240_e47703_d_n15, assign30240_e47703_d_n16, assign30240_e47703_d_n17, assign30240_e47703_d_n18, assign30240_e47703_d_n19, assign30240_e47703_d_n20, assign30240_e47703_d_n21, assign30240_e47703_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30240_e47667: f64 = (var_beta * var_dvgon);
        let assign30240_e47668: f64 = (assign30240_e47667).ln();
        let assign30240_e47669: f64 = (1.0 - assign30240_e47668);
        let assign30240_e47670: f64 = (var_vtv * assign30240_e47669);
        let assign30240_e47671: f64 = (var_vgop + assign30240_e47670);
        let assign30240_e47674: f64 = (p.p208 / 3.0);
        let assign30240_e47677: f64 = (var_cch * var_vgop);
        let assign30240_e47679: f64 = (assign30240_e47677).powf(0.6666666666666666);
        let assign30240_e47680: f64 = (assign30240_e47674 * assign30240_e47679);
        let assign30240_e47681: f64 = (assign30240_e47671 - assign30240_e47680);
        let assign30240_e47686: f64 = (var_vtv / var_dvgod);
        let assign30240_e47687: f64 = (1.0 + assign30240_e47686);
        let assign30240_e47688: f64 = (var_vgop * assign30240_e47687);
        let assign30240_e47691: f64 = (2.0 * p.p208);
        let assign30240_e47693: f64 = (assign30240_e47691 / 3.0);
        let assign30240_e47696: f64 = (var_cch * var_vgop);
        let assign30240_e47698: f64 = (assign30240_e47696).powf(0.6666666666666666);
        let assign30240_e47699: f64 = (assign30240_e47693 * assign30240_e47698);
        let assign30240_e47700: f64 = (assign30240_e47688 + assign30240_e47699);
        let assign30240_e47701: f64 = (assign30240_e47681 / assign30240_e47700);
        (assign30240_e47701, (((((var_vgop_dn0 + (var_vtv * (-((var_beta * var_dvgon_dn0) / assign30240_e47667)))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn0))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((var_cch * var_vgop_dn0) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((var_vgop_dn0 * assign30240_e47687) + (var_vgop * (-((var_vtv * var_dvgod_dn0) / (var_dvgod * var_dvgod))))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn0))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((var_cch * var_vgop_dn0) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((var_vgop_dn1 + (var_vtv * (-((var_beta * var_dvgon_dn1) / assign30240_e47667)))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn1))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((var_cch * var_vgop_dn1) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((var_vgop_dn1 * assign30240_e47687) + (var_vgop * (-((var_vtv * var_dvgod_dn1) / (var_dvgod * var_dvgod))))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn1))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((var_cch * var_vgop_dn1) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((var_vgop_dn2 + (var_vtv * (-((var_beta * var_dvgon_dn2) / assign30240_e47667)))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn2))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((var_cch * var_vgop_dn2) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((var_vgop_dn2 * assign30240_e47687) + (var_vgop * (-((var_vtv * var_dvgod_dn2) / (var_dvgod * var_dvgod))))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn2))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((var_cch * var_vgop_dn2) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((var_vgop_dn3 + (var_vtv * (-((var_beta * var_dvgon_dn3) / assign30240_e47667)))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn3))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((var_cch * var_vgop_dn3) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((var_vgop_dn3 * assign30240_e47687) + (var_vgop * (-((var_vtv * var_dvgod_dn3) / (var_dvgod * var_dvgod))))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn3))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((var_cch * var_vgop_dn3) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((var_vgop_dn4 + ((var_vtv_dn4 * assign30240_e47669) + (var_vtv * (-(((var_beta_dn4 * var_dvgon) + (var_beta * var_dvgon_dn4)) / assign30240_e47667))))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn4))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((var_cch * var_vgop_dn4) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((var_vgop_dn4 * assign30240_e47687) + (var_vgop * (((var_vtv_dn4 * var_dvgod) - (var_vtv * var_dvgod_dn4)) / (var_dvgod * var_dvgod)))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn4))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((var_cch * var_vgop_dn4) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((var_vgop_dn5 + (var_vtv * (-((var_beta * var_dvgon_dn5) / assign30240_e47667)))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn5))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((var_cch * var_vgop_dn5) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((var_vgop_dn5 * assign30240_e47687) + (var_vgop * (-((var_vtv * var_dvgod_dn5) / (var_dvgod * var_dvgod))))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn5))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((var_cch * var_vgop_dn5) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((var_vgop_dn6 + ((var_vtv_dn6 * assign30240_e47669) + (var_vtv * (-(((var_beta_dn6 * var_dvgon) + (var_beta * var_dvgon_dn6)) / assign30240_e47667))))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn6))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((var_cch * var_vgop_dn6) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((var_vgop_dn6 * assign30240_e47687) + (var_vgop * (((var_vtv_dn6 * var_dvgod) - (var_vtv * var_dvgod_dn6)) / (var_dvgod * var_dvgod)))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn6))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((var_cch * var_vgop_dn6) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((var_vgop_dn7 + ((var_vtv_dn7 * assign30240_e47669) + (var_vtv * (-(((var_beta_dn7 * var_dvgon) + (var_beta * var_dvgon_dn7)) / assign30240_e47667))))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn7))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((var_cch * var_vgop_dn7) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((var_vgop_dn7 * assign30240_e47687) + (var_vgop * (((var_vtv_dn7 * var_dvgod) - (var_vtv * var_dvgod_dn7)) / (var_dvgod * var_dvgod)))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn7))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((var_cch * var_vgop_dn7) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((var_vgop_dn8 + ((var_vtv_dn8 * assign30240_e47669) + (var_vtv * (-(((var_beta_dn8 * var_dvgon) + (var_beta * var_dvgon_dn8)) / assign30240_e47667))))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn8))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((var_cch * var_vgop_dn8) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((var_vgop_dn8 * assign30240_e47687) + (var_vgop * (((var_vtv_dn8 * var_dvgod) - (var_vtv * var_dvgod_dn8)) / (var_dvgod * var_dvgod)))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn8))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((var_cch * var_vgop_dn8) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((var_vgop_dn9 + (var_vtv * (-((var_beta * var_dvgon_dn9) / assign30240_e47667)))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn9))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((var_cch * var_vgop_dn9) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((var_vgop_dn9 * assign30240_e47687) + (var_vgop * (-((var_vtv * var_dvgod_dn9) / (var_dvgod * var_dvgod))))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn9))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((var_cch * var_vgop_dn9) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((var_vgop_dn12 + (var_vtv * (-((var_beta * var_dvgon_dn12) / assign30240_e47667)))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn12))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((var_cch * var_vgop_dn12) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((var_vgop_dn12 * assign30240_e47687) + (var_vgop * (-((var_vtv * var_dvgod_dn12) / (var_dvgod * var_dvgod))))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn12))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((var_cch * var_vgop_dn12) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((var_vgop_dn14 + (var_vtv * (-((var_beta * var_dvgon_dn14) / assign30240_e47667)))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn14))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((var_cch * var_vgop_dn14) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((var_vgop_dn14 * assign30240_e47687) + (var_vgop * (-((var_vtv * var_dvgod_dn14) / (var_dvgod * var_dvgod))))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn14))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((var_cch * var_vgop_dn14) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((var_vgop_dn15 + ((var_vtv_dn15 * assign30240_e47669) + (var_vtv * (-(((var_beta_dn15 * var_dvgon) + (var_beta * var_dvgon_dn15)) / assign30240_e47667))))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn15))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((var_cch * var_vgop_dn15) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((var_vgop_dn15 * assign30240_e47687) + (var_vgop * (((var_vtv_dn15 * var_dvgod) - (var_vtv * var_dvgod_dn15)) / (var_dvgod * var_dvgod)))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn15))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((var_cch * var_vgop_dn15) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((var_vgop_dn16 + ((var_vtv_dn16 * assign30240_e47669) + (var_vtv * (-(((var_beta_dn16 * var_dvgon) + (var_beta * var_dvgon_dn16)) / assign30240_e47667))))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn16))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((var_cch * var_vgop_dn16) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((var_vgop_dn16 * assign30240_e47687) + (var_vgop * (((var_vtv_dn16 * var_dvgod) - (var_vtv * var_dvgod_dn16)) / (var_dvgod * var_dvgod)))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn16))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((var_cch * var_vgop_dn16) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((var_vgop_dn17 + ((var_vtv_dn17 * assign30240_e47669) + (var_vtv * (-(((var_beta_dn17 * var_dvgon) + (var_beta * var_dvgon_dn17)) / assign30240_e47667))))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn17))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((var_cch * var_vgop_dn17) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((var_vgop_dn17 * assign30240_e47687) + (var_vgop * (((var_vtv_dn17 * var_dvgod) - (var_vtv * var_dvgod_dn17)) / (var_dvgod * var_dvgod)))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn17))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((var_cch * var_vgop_dn17) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((var_vgop_dn18 + ((var_vtv_dn18 * assign30240_e47669) + (var_vtv * (-(((var_beta_dn18 * var_dvgon) + (var_beta * var_dvgon_dn18)) / assign30240_e47667))))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn18))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((var_cch * var_vgop_dn18) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((var_vgop_dn18 * assign30240_e47687) + (var_vgop * (((var_vtv_dn18 * var_dvgod) - (var_vtv * var_dvgod_dn18)) / (var_dvgod * var_dvgod)))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn18))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((var_cch * var_vgop_dn18) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((var_vgop_dn19 + ((var_vtv_dn19 * assign30240_e47669) + (var_vtv * (-(((var_beta_dn19 * var_dvgon) + (var_beta * var_dvgon_dn19)) / assign30240_e47667))))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn19))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((var_cch * var_vgop_dn19) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((var_vgop_dn19 * assign30240_e47687) + (var_vgop * (((var_vtv_dn19 * var_dvgod) - (var_vtv * var_dvgod_dn19)) / (var_dvgod * var_dvgod)))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn19))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((var_cch * var_vgop_dn19) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((var_vgop_dn20 + ((var_vtv_dn20 * assign30240_e47669) + (var_vtv * (-(((var_beta_dn20 * var_dvgon) + (var_beta * var_dvgon_dn20)) / assign30240_e47667))))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn20))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((var_cch * var_vgop_dn20) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((var_vgop_dn20 * assign30240_e47687) + (var_vgop * (((var_vtv_dn20 * var_dvgod) - (var_vtv * var_dvgod_dn20)) / (var_dvgod * var_dvgod)))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn20))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((var_cch * var_vgop_dn20) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((var_vgop_dn21 + ((var_vtv_dn21 * assign30240_e47669) + (var_vtv * (-(((var_beta_dn21 * var_dvgon) + (var_beta * var_dvgon_dn21)) / assign30240_e47667))))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn21))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((var_cch * var_vgop_dn21) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((var_vgop_dn21 * assign30240_e47687) + (var_vgop * (((var_vtv_dn21 * var_dvgod) - (var_vtv * var_dvgod_dn21)) / (var_dvgod * var_dvgod)))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn21))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((var_cch * var_vgop_dn21) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)), (((((var_vgop_dn22 + ((var_vtv_dn22 * assign30240_e47669) + (var_vtv * (-(((var_beta_dn22 * var_dvgon) + (var_beta * var_dvgon_dn22)) / assign30240_e47667))))) - (assign30240_e47674 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47677).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn22))) } } else { (assign30240_e47679 * (0.6666666666666666 * ((var_cch * var_vgop_dn22) / assign30240_e47677))) })) * assign30240_e47700) - (assign30240_e47681 * (((var_vgop_dn22 * assign30240_e47687) + (var_vgop * (((var_vtv_dn22 * var_dvgod) - (var_vtv * var_dvgod_dn22)) / (var_dvgod * var_dvgod)))) + (assign30240_e47693 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((assign30240_e47696).powf(0.6666666666666666 - 1.0) * (var_cch * var_vgop_dn22))) } } else { (assign30240_e47698 * (0.6666666666666666 * ((var_cch * var_vgop_dn22) / assign30240_e47696))) })))) / (assign30240_e47700 * assign30240_e47700)),)
    } else {
        (var_hx, var_hx_dn0, var_hx_dn1, var_hx_dn2, var_hx_dn3, var_hx_dn4, var_hx_dn5, var_hx_dn6, var_hx_dn7, var_hx_dn8, var_hx_dn9, var_hx_dn12, var_hx_dn14, var_hx_dn15, var_hx_dn16, var_hx_dn17, var_hx_dn18, var_hx_dn19, var_hx_dn20, var_hx_dn21, var_hx_dn22,)
    }
};
        var_hx = assign30240_e47703;
        var_hx_dn0 = assign30240_e47703_d_n0;
        var_hx_dn1 = assign30240_e47703_d_n1;
        var_hx_dn2 = assign30240_e47703_d_n2;
        var_hx_dn3 = assign30240_e47703_d_n3;
        var_hx_dn4 = assign30240_e47703_d_n4;
        var_hx_dn5 = assign30240_e47703_d_n5;
        var_hx_dn6 = assign30240_e47703_d_n6;
        var_hx_dn7 = assign30240_e47703_d_n7;
        var_hx_dn8 = assign30240_e47703_d_n8;
        var_hx_dn9 = assign30240_e47703_d_n9;
        var_hx_dn12 = assign30240_e47703_d_n12;
        var_hx_dn14 = assign30240_e47703_d_n14;
        var_hx_dn15 = assign30240_e47703_d_n15;
        var_hx_dn16 = assign30240_e47703_d_n16;
        var_hx_dn17 = assign30240_e47703_d_n17;
        var_hx_dn18 = assign30240_e47703_d_n18;
        var_hx_dn19 = assign30240_e47703_d_n19;
        var_hx_dn20 = assign30240_e47703_d_n20;
        var_hx_dn21 = assign30240_e47703_d_n21;
        var_hx_dn22 = assign30240_e47703_d_n22;

        let (assign30250_e47714, assign30250_e47714_d_n0, assign30250_e47714_d_n1, assign30250_e47714_d_n2, assign30250_e47714_d_n3, assign30250_e47714_d_n4, assign30250_e47714_d_n5, assign30250_e47714_d_n6, assign30250_e47714_d_n7, assign30250_e47714_d_n8, assign30250_e47714_d_n9, assign30250_e47714_d_n12, assign30250_e47714_d_n14, assign30250_e47714_d_n15, assign30250_e47714_d_n16, assign30250_e47714_d_n17, assign30250_e47714_d_n18, assign30250_e47714_d_n19, assign30250_e47714_d_n20, assign30250_e47714_d_n21, assign30250_e47714_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30250_e47711: f64 = (2.0 * var_vtv);
        let assign30250_e47712: f64 = (var_vgod / assign30250_e47711);
        (assign30250_e47712, (var_vgod_dn0 / assign30250_e47711), (var_vgod_dn1 / assign30250_e47711), (var_vgod_dn2 / assign30250_e47711), (var_vgod_dn3 / assign30250_e47711), (((var_vgod_dn4 * assign30250_e47711) - (var_vgod * (2.0 * var_vtv_dn4))) / (assign30250_e47711 * assign30250_e47711)), (var_vgod_dn5 / assign30250_e47711), (((var_vgod_dn6 * assign30250_e47711) - (var_vgod * (2.0 * var_vtv_dn6))) / (assign30250_e47711 * assign30250_e47711)), (((var_vgod_dn7 * assign30250_e47711) - (var_vgod * (2.0 * var_vtv_dn7))) / (assign30250_e47711 * assign30250_e47711)), (((var_vgod_dn8 * assign30250_e47711) - (var_vgod * (2.0 * var_vtv_dn8))) / (assign30250_e47711 * assign30250_e47711)), (var_vgod_dn9 / assign30250_e47711), (var_vgod_dn12 / assign30250_e47711), (var_vgod_dn14 / assign30250_e47711), (((var_vgod_dn15 * assign30250_e47711) - (var_vgod * (2.0 * var_vtv_dn15))) / (assign30250_e47711 * assign30250_e47711)), (((var_vgod_dn16 * assign30250_e47711) - (var_vgod * (2.0 * var_vtv_dn16))) / (assign30250_e47711 * assign30250_e47711)), (((var_vgod_dn17 * assign30250_e47711) - (var_vgod * (2.0 * var_vtv_dn17))) / (assign30250_e47711 * assign30250_e47711)), (((var_vgod_dn18 * assign30250_e47711) - (var_vgod * (2.0 * var_vtv_dn18))) / (assign30250_e47711 * assign30250_e47711)), (((var_vgod_dn19 * assign30250_e47711) - (var_vgod * (2.0 * var_vtv_dn19))) / (assign30250_e47711 * assign30250_e47711)), (((var_vgod_dn20 * assign30250_e47711) - (var_vgod * (2.0 * var_vtv_dn20))) / (assign30250_e47711 * assign30250_e47711)), (((var_vgod_dn21 * assign30250_e47711) - (var_vgod * (2.0 * var_vtv_dn21))) / (assign30250_e47711 * assign30250_e47711)), (((var_vgod_dn22 * assign30250_e47711) - (var_vgod * (2.0 * var_vtv_dn22))) / (assign30250_e47711 * assign30250_e47711)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn1, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn12, var_t0_dn14, var_t0_dn15, var_t0_dn16, var_t0_dn17, var_t0_dn18, var_t0_dn19, var_t0_dn20, var_t0_dn21, var_t0_dn22,)
    }
};
        var_t0 = assign30250_e47714;
        var_t0_dn0 = assign30250_e47714_d_n0;
        var_t0_dn1 = assign30250_e47714_d_n1;
        var_t0_dn2 = assign30250_e47714_d_n2;
        var_t0_dn3 = assign30250_e47714_d_n3;
        var_t0_dn4 = assign30250_e47714_d_n4;
        var_t0_dn5 = assign30250_e47714_d_n5;
        var_t0_dn6 = assign30250_e47714_d_n6;
        var_t0_dn7 = assign30250_e47714_d_n7;
        var_t0_dn8 = assign30250_e47714_d_n8;
        var_t0_dn9 = assign30250_e47714_d_n9;
        var_t0_dn12 = assign30250_e47714_d_n12;
        var_t0_dn14 = assign30250_e47714_d_n14;
        var_t0_dn15 = assign30250_e47714_d_n15;
        var_t0_dn16 = assign30250_e47714_d_n16;
        var_t0_dn17 = assign30250_e47714_d_n17;
        var_t0_dn18 = assign30250_e47714_d_n18;
        var_t0_dn19 = assign30250_e47714_d_n19;
        var_t0_dn20 = assign30250_e47714_d_n20;
        var_t0_dn21 = assign30250_e47714_d_n21;
        var_t0_dn22 = assign30250_e47714_d_n22;

        let assign30260_e47717: f64 = if var_t0 < 200.0 { 1.0 } else { 0.0 };
        var_guard517 = assign30260_e47717;

        let (assign30270_e47729, assign30270_e47729_d_n0, assign30270_e47729_d_n1, assign30270_e47729_d_n2, assign30270_e47729_d_n3, assign30270_e47729_d_n4, assign30270_e47729_d_n5, assign30270_e47729_d_n6, assign30270_e47729_d_n7, assign30270_e47729_d_n8, assign30270_e47729_d_n9, assign30270_e47729_d_n12, assign30270_e47729_d_n14, assign30270_e47729_d_n15, assign30270_e47729_d_n16, assign30270_e47729_d_n17, assign30270_e47729_d_n18, assign30270_e47729_d_n19, assign30270_e47729_d_n20, assign30270_e47729_d_n21, assign30270_e47729_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard517 != 0.0)) {
        let assign30270_e47726: f64 = (var_t0 / 4.0);
        let assign30270_e47727: f64 = { let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign30270_e47727, ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn0 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn1 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn2 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn3 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn4 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn5 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn6 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn7 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn8 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn9 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn12 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn14 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn15 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn16 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn17 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn18 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn19 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn20 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn21 / 4.0)), ({ let limited_exp_arg = assign30270_e47726; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t0_dn22 / 4.0)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn1, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn12, var_t1_dn14, var_t1_dn15, var_t1_dn16, var_t1_dn17, var_t1_dn18, var_t1_dn19, var_t1_dn20, var_t1_dn21, var_t1_dn22,)
    }
};
        var_t1 = assign30270_e47729;
        var_t1_dn0 = assign30270_e47729_d_n0;
        var_t1_dn1 = assign30270_e47729_d_n1;
        var_t1_dn2 = assign30270_e47729_d_n2;
        var_t1_dn3 = assign30270_e47729_d_n3;
        var_t1_dn4 = assign30270_e47729_d_n4;
        var_t1_dn5 = assign30270_e47729_d_n5;
        var_t1_dn6 = assign30270_e47729_d_n6;
        var_t1_dn7 = assign30270_e47729_d_n7;
        var_t1_dn8 = assign30270_e47729_d_n8;
        var_t1_dn9 = assign30270_e47729_d_n9;
        var_t1_dn12 = assign30270_e47729_d_n12;
        var_t1_dn14 = assign30270_e47729_d_n14;
        var_t1_dn15 = assign30270_e47729_d_n15;
        var_t1_dn16 = assign30270_e47729_d_n16;
        var_t1_dn17 = assign30270_e47729_d_n17;
        var_t1_dn18 = assign30270_e47729_d_n18;
        var_t1_dn19 = assign30270_e47729_d_n19;
        var_t1_dn20 = assign30270_e47729_d_n20;
        var_t1_dn21 = assign30270_e47729_d_n21;
        var_t1_dn22 = assign30270_e47729_d_n22;

        let (assign30280_e47744, assign30280_e47744_d_n0, assign30280_e47744_d_n1, assign30280_e47744_d_n2, assign30280_e47744_d_n3, assign30280_e47744_d_n4, assign30280_e47744_d_n5, assign30280_e47744_d_n6, assign30280_e47744_d_n7, assign30280_e47744_d_n8, assign30280_e47744_d_n9, assign30280_e47744_d_n12, assign30280_e47744_d_n14, assign30280_e47744_d_n15, assign30280_e47744_d_n16, assign30280_e47744_d_n17, assign30280_e47744_d_n18, assign30280_e47744_d_n19, assign30280_e47744_d_n20, assign30280_e47744_d_n21, assign30280_e47744_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard517 != 0.0)) {
        let assign30280_e47737: f64 = (-3.0);
        let assign30280_e47739: f64 = (assign30280_e47737 * var_t0);
        let assign30280_e47741: f64 = (assign30280_e47739 / 4.0);
        let assign30280_e47742: f64 = { let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign30280_e47742, ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * var_t0_dn0) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * var_t0_dn1) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * var_t0_dn2) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * var_t0_dn3) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * var_t0_dn4) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * var_t0_dn5) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * var_t0_dn6) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * var_t0_dn7) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * var_t0_dn8) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * var_t0_dn9) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * var_t0_dn12) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * var_t0_dn14) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * var_t0_dn15) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * var_t0_dn16) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * var_t0_dn17) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * var_t0_dn18) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * var_t0_dn19) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * var_t0_dn20) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * var_t0_dn21) / 4.0)), ({ let limited_exp_arg = assign30280_e47741; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30280_e47737 * var_t0_dn22) / 4.0)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn1, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn12, var_t2_dn14, var_t2_dn15, var_t2_dn16, var_t2_dn17, var_t2_dn18, var_t2_dn19, var_t2_dn20, var_t2_dn21, var_t2_dn22,)
    }
};
        var_t2 = assign30280_e47744;
        var_t2_dn0 = assign30280_e47744_d_n0;
        var_t2_dn1 = assign30280_e47744_d_n1;
        var_t2_dn2 = assign30280_e47744_d_n2;
        var_t2_dn3 = assign30280_e47744_d_n3;
        var_t2_dn4 = assign30280_e47744_d_n4;
        var_t2_dn5 = assign30280_e47744_d_n5;
        var_t2_dn6 = assign30280_e47744_d_n6;
        var_t2_dn7 = assign30280_e47744_d_n7;
        var_t2_dn8 = assign30280_e47744_d_n8;
        var_t2_dn9 = assign30280_e47744_d_n9;
        var_t2_dn12 = assign30280_e47744_d_n12;
        var_t2_dn14 = assign30280_e47744_d_n14;
        var_t2_dn15 = assign30280_e47744_d_n15;
        var_t2_dn16 = assign30280_e47744_d_n16;
        var_t2_dn17 = assign30280_e47744_d_n17;
        var_t2_dn18 = assign30280_e47744_d_n18;
        var_t2_dn19 = assign30280_e47744_d_n19;
        var_t2_dn20 = assign30280_e47744_d_n20;
        var_t2_dn21 = assign30280_e47744_d_n21;
        var_t2_dn22 = assign30280_e47744_d_n22;

        let (assign30290_e47786, assign30290_e47786_d_n0, assign30290_e47786_d_n1, assign30290_e47786_d_n2, assign30290_e47786_d_n3, assign30290_e47786_d_n4, assign30290_e47786_d_n5, assign30290_e47786_d_n6, assign30290_e47786_d_n7, assign30290_e47786_d_n8, assign30290_e47786_d_n9, assign30290_e47786_d_n12, assign30290_e47786_d_n14, assign30290_e47786_d_n15, assign30290_e47786_d_n16, assign30290_e47786_d_n17, assign30290_e47786_d_n18, assign30290_e47786_d_n19, assign30290_e47786_d_n20, assign30290_e47786_d_n21, assign30290_e47786_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard517 != 0.0)) {
        let assign30290_e47753: f64 = (2.0 * var_vtv);
        let assign30290_e47755: f64 = (assign30290_e47753 * var_cch);
        let assign30290_e47758: f64 = (3.0 * var_t0);
        let assign30290_e47760: f64 = (assign30290_e47758 / 4.0);
        let assign30290_e47763: f64 = (var_t1 + var_t2);
        let assign30290_e47764: f64 = (assign30290_e47763).ln();
        let assign30290_e47765: f64 = (assign30290_e47760 + assign30290_e47764);
        let assign30290_e47766: f64 = (assign30290_e47755 * assign30290_e47765);
        let assign30290_e47769: f64 = (1.0 / var_hx);
        let assign30290_e47772: f64 = (var_cch / 3.24e17);
        let assign30290_e47774: f64 = (-1.0);
        let assign30290_e47776: f64 = (assign30290_e47774 * var_vgod);
        let assign30290_e47779: f64 = (2.0 * var_vtv);
        let assign30290_e47780: f64 = (assign30290_e47776 / assign30290_e47779);
        let assign30290_e47781: f64 = { let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign30290_e47782: f64 = (assign30290_e47772 * assign30290_e47781);
        let assign30290_e47783: f64 = (assign30290_e47769 + assign30290_e47782);
        let assign30290_e47784: f64 = (assign30290_e47766 / assign30290_e47783);
        (assign30290_e47784, ((((assign30290_e47755 * (((3.0 * var_t0_dn0) / 4.0) + ((var_t1_dn0 + var_t2_dn0) / assign30290_e47763))) * assign30290_e47783) - (assign30290_e47766 * ((-(var_hx_dn0 / (var_hx * var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30290_e47774 * var_vgod_dn0) / assign30290_e47779)))))) / (assign30290_e47783 * assign30290_e47783)), ((((assign30290_e47755 * (((3.0 * var_t0_dn1) / 4.0) + ((var_t1_dn1 + var_t2_dn1) / assign30290_e47763))) * assign30290_e47783) - (assign30290_e47766 * ((-(var_hx_dn1 / (var_hx * var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30290_e47774 * var_vgod_dn1) / assign30290_e47779)))))) / (assign30290_e47783 * assign30290_e47783)), ((((assign30290_e47755 * (((3.0 * var_t0_dn2) / 4.0) + ((var_t1_dn2 + var_t2_dn2) / assign30290_e47763))) * assign30290_e47783) - (assign30290_e47766 * ((-(var_hx_dn2 / (var_hx * var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30290_e47774 * var_vgod_dn2) / assign30290_e47779)))))) / (assign30290_e47783 * assign30290_e47783)), ((((assign30290_e47755 * (((3.0 * var_t0_dn3) / 4.0) + ((var_t1_dn3 + var_t2_dn3) / assign30290_e47763))) * assign30290_e47783) - (assign30290_e47766 * ((-(var_hx_dn3 / (var_hx * var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30290_e47774 * var_vgod_dn3) / assign30290_e47779)))))) / (assign30290_e47783 * assign30290_e47783)), (((((((2.0 * var_vtv_dn4) * var_cch) * assign30290_e47765) + (assign30290_e47755 * (((3.0 * var_t0_dn4) / 4.0) + ((var_t1_dn4 + var_t2_dn4) / assign30290_e47763)))) * assign30290_e47783) - (assign30290_e47766 * ((-(var_hx_dn4 / (var_hx * var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30290_e47774 * var_vgod_dn4) * assign30290_e47779) - (assign30290_e47776 * (2.0 * var_vtv_dn4))) / (assign30290_e47779 * assign30290_e47779))))))) / (assign30290_e47783 * assign30290_e47783)), ((((assign30290_e47755 * (((3.0 * var_t0_dn5) / 4.0) + ((var_t1_dn5 + var_t2_dn5) / assign30290_e47763))) * assign30290_e47783) - (assign30290_e47766 * ((-(var_hx_dn5 / (var_hx * var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30290_e47774 * var_vgod_dn5) / assign30290_e47779)))))) / (assign30290_e47783 * assign30290_e47783)), (((((((2.0 * var_vtv_dn6) * var_cch) * assign30290_e47765) + (assign30290_e47755 * (((3.0 * var_t0_dn6) / 4.0) + ((var_t1_dn6 + var_t2_dn6) / assign30290_e47763)))) * assign30290_e47783) - (assign30290_e47766 * ((-(var_hx_dn6 / (var_hx * var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30290_e47774 * var_vgod_dn6) * assign30290_e47779) - (assign30290_e47776 * (2.0 * var_vtv_dn6))) / (assign30290_e47779 * assign30290_e47779))))))) / (assign30290_e47783 * assign30290_e47783)), (((((((2.0 * var_vtv_dn7) * var_cch) * assign30290_e47765) + (assign30290_e47755 * (((3.0 * var_t0_dn7) / 4.0) + ((var_t1_dn7 + var_t2_dn7) / assign30290_e47763)))) * assign30290_e47783) - (assign30290_e47766 * ((-(var_hx_dn7 / (var_hx * var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30290_e47774 * var_vgod_dn7) * assign30290_e47779) - (assign30290_e47776 * (2.0 * var_vtv_dn7))) / (assign30290_e47779 * assign30290_e47779))))))) / (assign30290_e47783 * assign30290_e47783)), (((((((2.0 * var_vtv_dn8) * var_cch) * assign30290_e47765) + (assign30290_e47755 * (((3.0 * var_t0_dn8) / 4.0) + ((var_t1_dn8 + var_t2_dn8) / assign30290_e47763)))) * assign30290_e47783) - (assign30290_e47766 * ((-(var_hx_dn8 / (var_hx * var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30290_e47774 * var_vgod_dn8) * assign30290_e47779) - (assign30290_e47776 * (2.0 * var_vtv_dn8))) / (assign30290_e47779 * assign30290_e47779))))))) / (assign30290_e47783 * assign30290_e47783)), ((((assign30290_e47755 * (((3.0 * var_t0_dn9) / 4.0) + ((var_t1_dn9 + var_t2_dn9) / assign30290_e47763))) * assign30290_e47783) - (assign30290_e47766 * ((-(var_hx_dn9 / (var_hx * var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30290_e47774 * var_vgod_dn9) / assign30290_e47779)))))) / (assign30290_e47783 * assign30290_e47783)), ((((assign30290_e47755 * (((3.0 * var_t0_dn12) / 4.0) + ((var_t1_dn12 + var_t2_dn12) / assign30290_e47763))) * assign30290_e47783) - (assign30290_e47766 * ((-(var_hx_dn12 / (var_hx * var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30290_e47774 * var_vgod_dn12) / assign30290_e47779)))))) / (assign30290_e47783 * assign30290_e47783)), ((((assign30290_e47755 * (((3.0 * var_t0_dn14) / 4.0) + ((var_t1_dn14 + var_t2_dn14) / assign30290_e47763))) * assign30290_e47783) - (assign30290_e47766 * ((-(var_hx_dn14 / (var_hx * var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30290_e47774 * var_vgod_dn14) / assign30290_e47779)))))) / (assign30290_e47783 * assign30290_e47783)), (((((((2.0 * var_vtv_dn15) * var_cch) * assign30290_e47765) + (assign30290_e47755 * (((3.0 * var_t0_dn15) / 4.0) + ((var_t1_dn15 + var_t2_dn15) / assign30290_e47763)))) * assign30290_e47783) - (assign30290_e47766 * ((-(var_hx_dn15 / (var_hx * var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30290_e47774 * var_vgod_dn15) * assign30290_e47779) - (assign30290_e47776 * (2.0 * var_vtv_dn15))) / (assign30290_e47779 * assign30290_e47779))))))) / (assign30290_e47783 * assign30290_e47783)), (((((((2.0 * var_vtv_dn16) * var_cch) * assign30290_e47765) + (assign30290_e47755 * (((3.0 * var_t0_dn16) / 4.0) + ((var_t1_dn16 + var_t2_dn16) / assign30290_e47763)))) * assign30290_e47783) - (assign30290_e47766 * ((-(var_hx_dn16 / (var_hx * var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30290_e47774 * var_vgod_dn16) * assign30290_e47779) - (assign30290_e47776 * (2.0 * var_vtv_dn16))) / (assign30290_e47779 * assign30290_e47779))))))) / (assign30290_e47783 * assign30290_e47783)), (((((((2.0 * var_vtv_dn17) * var_cch) * assign30290_e47765) + (assign30290_e47755 * (((3.0 * var_t0_dn17) / 4.0) + ((var_t1_dn17 + var_t2_dn17) / assign30290_e47763)))) * assign30290_e47783) - (assign30290_e47766 * ((-(var_hx_dn17 / (var_hx * var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30290_e47774 * var_vgod_dn17) * assign30290_e47779) - (assign30290_e47776 * (2.0 * var_vtv_dn17))) / (assign30290_e47779 * assign30290_e47779))))))) / (assign30290_e47783 * assign30290_e47783)), (((((((2.0 * var_vtv_dn18) * var_cch) * assign30290_e47765) + (assign30290_e47755 * (((3.0 * var_t0_dn18) / 4.0) + ((var_t1_dn18 + var_t2_dn18) / assign30290_e47763)))) * assign30290_e47783) - (assign30290_e47766 * ((-(var_hx_dn18 / (var_hx * var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30290_e47774 * var_vgod_dn18) * assign30290_e47779) - (assign30290_e47776 * (2.0 * var_vtv_dn18))) / (assign30290_e47779 * assign30290_e47779))))))) / (assign30290_e47783 * assign30290_e47783)), (((((((2.0 * var_vtv_dn19) * var_cch) * assign30290_e47765) + (assign30290_e47755 * (((3.0 * var_t0_dn19) / 4.0) + ((var_t1_dn19 + var_t2_dn19) / assign30290_e47763)))) * assign30290_e47783) - (assign30290_e47766 * ((-(var_hx_dn19 / (var_hx * var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30290_e47774 * var_vgod_dn19) * assign30290_e47779) - (assign30290_e47776 * (2.0 * var_vtv_dn19))) / (assign30290_e47779 * assign30290_e47779))))))) / (assign30290_e47783 * assign30290_e47783)), (((((((2.0 * var_vtv_dn20) * var_cch) * assign30290_e47765) + (assign30290_e47755 * (((3.0 * var_t0_dn20) / 4.0) + ((var_t1_dn20 + var_t2_dn20) / assign30290_e47763)))) * assign30290_e47783) - (assign30290_e47766 * ((-(var_hx_dn20 / (var_hx * var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30290_e47774 * var_vgod_dn20) * assign30290_e47779) - (assign30290_e47776 * (2.0 * var_vtv_dn20))) / (assign30290_e47779 * assign30290_e47779))))))) / (assign30290_e47783 * assign30290_e47783)), (((((((2.0 * var_vtv_dn21) * var_cch) * assign30290_e47765) + (assign30290_e47755 * (((3.0 * var_t0_dn21) / 4.0) + ((var_t1_dn21 + var_t2_dn21) / assign30290_e47763)))) * assign30290_e47783) - (assign30290_e47766 * ((-(var_hx_dn21 / (var_hx * var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30290_e47774 * var_vgod_dn21) * assign30290_e47779) - (assign30290_e47776 * (2.0 * var_vtv_dn21))) / (assign30290_e47779 * assign30290_e47779))))))) / (assign30290_e47783 * assign30290_e47783)), (((((((2.0 * var_vtv_dn22) * var_cch) * assign30290_e47765) + (assign30290_e47755 * (((3.0 * var_t0_dn22) / 4.0) + ((var_t1_dn22 + var_t2_dn22) / assign30290_e47763)))) * assign30290_e47783) - (assign30290_e47766 * ((-(var_hx_dn22 / (var_hx * var_hx))) + (assign30290_e47772 * ({ let limited_exp_arg = assign30290_e47780; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30290_e47774 * var_vgod_dn22) * assign30290_e47779) - (assign30290_e47776 * (2.0 * var_vtv_dn22))) / (assign30290_e47779 * assign30290_e47779))))))) / (assign30290_e47783 * assign30290_e47783)),)
    } else {
        (var_ndx, var_ndx_dn0, var_ndx_dn1, var_ndx_dn2, var_ndx_dn3, var_ndx_dn4, var_ndx_dn5, var_ndx_dn6, var_ndx_dn7, var_ndx_dn8, var_ndx_dn9, var_ndx_dn12, var_ndx_dn14, var_ndx_dn15, var_ndx_dn16, var_ndx_dn17, var_ndx_dn18, var_ndx_dn19, var_ndx_dn20, var_ndx_dn21, var_ndx_dn22,)
    }
};
        var_ndx = assign30290_e47786;
        var_ndx_dn0 = assign30290_e47786_d_n0;
        var_ndx_dn1 = assign30290_e47786_d_n1;
        var_ndx_dn2 = assign30290_e47786_d_n2;
        var_ndx_dn3 = assign30290_e47786_d_n3;
        var_ndx_dn4 = assign30290_e47786_d_n4;
        var_ndx_dn5 = assign30290_e47786_d_n5;
        var_ndx_dn6 = assign30290_e47786_d_n6;
        var_ndx_dn7 = assign30290_e47786_d_n7;
        var_ndx_dn8 = assign30290_e47786_d_n8;
        var_ndx_dn9 = assign30290_e47786_d_n9;
        var_ndx_dn12 = assign30290_e47786_d_n12;
        var_ndx_dn14 = assign30290_e47786_d_n14;
        var_ndx_dn15 = assign30290_e47786_d_n15;
        var_ndx_dn16 = assign30290_e47786_d_n16;
        var_ndx_dn17 = assign30290_e47786_d_n17;
        var_ndx_dn18 = assign30290_e47786_d_n18;
        var_ndx_dn19 = assign30290_e47786_d_n19;
        var_ndx_dn20 = assign30290_e47786_d_n20;
        var_ndx_dn21 = assign30290_e47786_d_n21;
        var_ndx_dn22 = assign30290_e47786_d_n22;

        let (assign30300_e47824, assign30300_e47824_d_n0, assign30300_e47824_d_n1, assign30300_e47824_d_n2, assign30300_e47824_d_n3, assign30300_e47824_d_n4, assign30300_e47824_d_n5, assign30300_e47824_d_n6, assign30300_e47824_d_n7, assign30300_e47824_d_n8, assign30300_e47824_d_n9, assign30300_e47824_d_n12, assign30300_e47824_d_n14, assign30300_e47824_d_n15, assign30300_e47824_d_n16, assign30300_e47824_d_n17, assign30300_e47824_d_n18, assign30300_e47824_d_n19, assign30300_e47824_d_n20, assign30300_e47824_d_n21, assign30300_e47824_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard517 == 0.0)) {
        let assign30300_e47796: f64 = (2.0 * var_vtv);
        let assign30300_e47798: f64 = (assign30300_e47796 * var_cch);
        let assign30300_e47801: f64 = var_t0;
        let assign30300_e47803: f64 = assign30300_e47801;
        let assign30300_e47804: f64 = (assign30300_e47798 * assign30300_e47803);
        let assign30300_e47807: f64 = (1.0 / var_hx);
        let assign30300_e47810: f64 = (var_cch / 3.24e17);
        let assign30300_e47812: f64 = (-1.0);
        let assign30300_e47814: f64 = (assign30300_e47812 * var_vgod);
        let assign30300_e47817: f64 = (2.0 * var_vtv);
        let assign30300_e47818: f64 = (assign30300_e47814 / assign30300_e47817);
        let assign30300_e47819: f64 = { let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign30300_e47820: f64 = (assign30300_e47810 * assign30300_e47819);
        let assign30300_e47821: f64 = (assign30300_e47807 + assign30300_e47820);
        let assign30300_e47822: f64 = (assign30300_e47804 / assign30300_e47821);
        (assign30300_e47822, ((((assign30300_e47798 * var_t0_dn0) * assign30300_e47821) - (assign30300_e47804 * ((-(var_hx_dn0 / (var_hx * var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30300_e47812 * var_vgod_dn0) / assign30300_e47817)))))) / (assign30300_e47821 * assign30300_e47821)), ((((assign30300_e47798 * var_t0_dn1) * assign30300_e47821) - (assign30300_e47804 * ((-(var_hx_dn1 / (var_hx * var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30300_e47812 * var_vgod_dn1) / assign30300_e47817)))))) / (assign30300_e47821 * assign30300_e47821)), ((((assign30300_e47798 * var_t0_dn2) * assign30300_e47821) - (assign30300_e47804 * ((-(var_hx_dn2 / (var_hx * var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30300_e47812 * var_vgod_dn2) / assign30300_e47817)))))) / (assign30300_e47821 * assign30300_e47821)), ((((assign30300_e47798 * var_t0_dn3) * assign30300_e47821) - (assign30300_e47804 * ((-(var_hx_dn3 / (var_hx * var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30300_e47812 * var_vgod_dn3) / assign30300_e47817)))))) / (assign30300_e47821 * assign30300_e47821)), (((((((2.0 * var_vtv_dn4) * var_cch) * assign30300_e47803) + (assign30300_e47798 * var_t0_dn4)) * assign30300_e47821) - (assign30300_e47804 * ((-(var_hx_dn4 / (var_hx * var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30300_e47812 * var_vgod_dn4) * assign30300_e47817) - (assign30300_e47814 * (2.0 * var_vtv_dn4))) / (assign30300_e47817 * assign30300_e47817))))))) / (assign30300_e47821 * assign30300_e47821)), ((((assign30300_e47798 * var_t0_dn5) * assign30300_e47821) - (assign30300_e47804 * ((-(var_hx_dn5 / (var_hx * var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30300_e47812 * var_vgod_dn5) / assign30300_e47817)))))) / (assign30300_e47821 * assign30300_e47821)), (((((((2.0 * var_vtv_dn6) * var_cch) * assign30300_e47803) + (assign30300_e47798 * var_t0_dn6)) * assign30300_e47821) - (assign30300_e47804 * ((-(var_hx_dn6 / (var_hx * var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30300_e47812 * var_vgod_dn6) * assign30300_e47817) - (assign30300_e47814 * (2.0 * var_vtv_dn6))) / (assign30300_e47817 * assign30300_e47817))))))) / (assign30300_e47821 * assign30300_e47821)), (((((((2.0 * var_vtv_dn7) * var_cch) * assign30300_e47803) + (assign30300_e47798 * var_t0_dn7)) * assign30300_e47821) - (assign30300_e47804 * ((-(var_hx_dn7 / (var_hx * var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30300_e47812 * var_vgod_dn7) * assign30300_e47817) - (assign30300_e47814 * (2.0 * var_vtv_dn7))) / (assign30300_e47817 * assign30300_e47817))))))) / (assign30300_e47821 * assign30300_e47821)), (((((((2.0 * var_vtv_dn8) * var_cch) * assign30300_e47803) + (assign30300_e47798 * var_t0_dn8)) * assign30300_e47821) - (assign30300_e47804 * ((-(var_hx_dn8 / (var_hx * var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30300_e47812 * var_vgod_dn8) * assign30300_e47817) - (assign30300_e47814 * (2.0 * var_vtv_dn8))) / (assign30300_e47817 * assign30300_e47817))))))) / (assign30300_e47821 * assign30300_e47821)), ((((assign30300_e47798 * var_t0_dn9) * assign30300_e47821) - (assign30300_e47804 * ((-(var_hx_dn9 / (var_hx * var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30300_e47812 * var_vgod_dn9) / assign30300_e47817)))))) / (assign30300_e47821 * assign30300_e47821)), ((((assign30300_e47798 * var_t0_dn12) * assign30300_e47821) - (assign30300_e47804 * ((-(var_hx_dn12 / (var_hx * var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30300_e47812 * var_vgod_dn12) / assign30300_e47817)))))) / (assign30300_e47821 * assign30300_e47821)), ((((assign30300_e47798 * var_t0_dn14) * assign30300_e47821) - (assign30300_e47804 * ((-(var_hx_dn14 / (var_hx * var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((assign30300_e47812 * var_vgod_dn14) / assign30300_e47817)))))) / (assign30300_e47821 * assign30300_e47821)), (((((((2.0 * var_vtv_dn15) * var_cch) * assign30300_e47803) + (assign30300_e47798 * var_t0_dn15)) * assign30300_e47821) - (assign30300_e47804 * ((-(var_hx_dn15 / (var_hx * var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30300_e47812 * var_vgod_dn15) * assign30300_e47817) - (assign30300_e47814 * (2.0 * var_vtv_dn15))) / (assign30300_e47817 * assign30300_e47817))))))) / (assign30300_e47821 * assign30300_e47821)), (((((((2.0 * var_vtv_dn16) * var_cch) * assign30300_e47803) + (assign30300_e47798 * var_t0_dn16)) * assign30300_e47821) - (assign30300_e47804 * ((-(var_hx_dn16 / (var_hx * var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30300_e47812 * var_vgod_dn16) * assign30300_e47817) - (assign30300_e47814 * (2.0 * var_vtv_dn16))) / (assign30300_e47817 * assign30300_e47817))))))) / (assign30300_e47821 * assign30300_e47821)), (((((((2.0 * var_vtv_dn17) * var_cch) * assign30300_e47803) + (assign30300_e47798 * var_t0_dn17)) * assign30300_e47821) - (assign30300_e47804 * ((-(var_hx_dn17 / (var_hx * var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30300_e47812 * var_vgod_dn17) * assign30300_e47817) - (assign30300_e47814 * (2.0 * var_vtv_dn17))) / (assign30300_e47817 * assign30300_e47817))))))) / (assign30300_e47821 * assign30300_e47821)), (((((((2.0 * var_vtv_dn18) * var_cch) * assign30300_e47803) + (assign30300_e47798 * var_t0_dn18)) * assign30300_e47821) - (assign30300_e47804 * ((-(var_hx_dn18 / (var_hx * var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30300_e47812 * var_vgod_dn18) * assign30300_e47817) - (assign30300_e47814 * (2.0 * var_vtv_dn18))) / (assign30300_e47817 * assign30300_e47817))))))) / (assign30300_e47821 * assign30300_e47821)), (((((((2.0 * var_vtv_dn19) * var_cch) * assign30300_e47803) + (assign30300_e47798 * var_t0_dn19)) * assign30300_e47821) - (assign30300_e47804 * ((-(var_hx_dn19 / (var_hx * var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30300_e47812 * var_vgod_dn19) * assign30300_e47817) - (assign30300_e47814 * (2.0 * var_vtv_dn19))) / (assign30300_e47817 * assign30300_e47817))))))) / (assign30300_e47821 * assign30300_e47821)), (((((((2.0 * var_vtv_dn20) * var_cch) * assign30300_e47803) + (assign30300_e47798 * var_t0_dn20)) * assign30300_e47821) - (assign30300_e47804 * ((-(var_hx_dn20 / (var_hx * var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30300_e47812 * var_vgod_dn20) * assign30300_e47817) - (assign30300_e47814 * (2.0 * var_vtv_dn20))) / (assign30300_e47817 * assign30300_e47817))))))) / (assign30300_e47821 * assign30300_e47821)), (((((((2.0 * var_vtv_dn21) * var_cch) * assign30300_e47803) + (assign30300_e47798 * var_t0_dn21)) * assign30300_e47821) - (assign30300_e47804 * ((-(var_hx_dn21 / (var_hx * var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30300_e47812 * var_vgod_dn21) * assign30300_e47817) - (assign30300_e47814 * (2.0 * var_vtv_dn21))) / (assign30300_e47817 * assign30300_e47817))))))) / (assign30300_e47821 * assign30300_e47821)), (((((((2.0 * var_vtv_dn22) * var_cch) * assign30300_e47803) + (assign30300_e47798 * var_t0_dn22)) * assign30300_e47821) - (assign30300_e47804 * ((-(var_hx_dn22 / (var_hx * var_hx))) + (assign30300_e47810 * ({ let limited_exp_arg = assign30300_e47818; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((assign30300_e47812 * var_vgod_dn22) * assign30300_e47817) - (assign30300_e47814 * (2.0 * var_vtv_dn22))) / (assign30300_e47817 * assign30300_e47817))))))) / (assign30300_e47821 * assign30300_e47821)),)
    } else {
        (var_ndx, var_ndx_dn0, var_ndx_dn1, var_ndx_dn2, var_ndx_dn3, var_ndx_dn4, var_ndx_dn5, var_ndx_dn6, var_ndx_dn7, var_ndx_dn8, var_ndx_dn9, var_ndx_dn12, var_ndx_dn14, var_ndx_dn15, var_ndx_dn16, var_ndx_dn17, var_ndx_dn18, var_ndx_dn19, var_ndx_dn20, var_ndx_dn21, var_ndx_dn22,)
    }
};
        var_ndx = assign30300_e47824;
        var_ndx_dn0 = assign30300_e47824_d_n0;
        var_ndx_dn1 = assign30300_e47824_d_n1;
        var_ndx_dn2 = assign30300_e47824_d_n2;
        var_ndx_dn3 = assign30300_e47824_d_n3;
        var_ndx_dn4 = assign30300_e47824_d_n4;
        var_ndx_dn5 = assign30300_e47824_d_n5;
        var_ndx_dn6 = assign30300_e47824_d_n6;
        var_ndx_dn7 = assign30300_e47824_d_n7;
        var_ndx_dn8 = assign30300_e47824_d_n8;
        var_ndx_dn9 = assign30300_e47824_d_n9;
        var_ndx_dn12 = assign30300_e47824_d_n12;
        var_ndx_dn14 = assign30300_e47824_d_n14;
        var_ndx_dn15 = assign30300_e47824_d_n15;
        var_ndx_dn16 = assign30300_e47824_d_n16;
        var_ndx_dn17 = assign30300_e47824_d_n17;
        var_ndx_dn18 = assign30300_e47824_d_n18;
        var_ndx_dn19 = assign30300_e47824_d_n19;
        var_ndx_dn20 = assign30300_e47824_d_n20;
        var_ndx_dn21 = assign30300_e47824_d_n21;
        var_ndx_dn22 = assign30300_e47824_d_n22;

        let (assign30310_e47835, assign30310_e47835_d_n0, assign30310_e47835_d_n1, assign30310_e47835_d_n2, assign30310_e47835_d_n3, assign30310_e47835_d_n4, assign30310_e47835_d_n5, assign30310_e47835_d_n6, assign30310_e47835_d_n7, assign30310_e47835_d_n8, assign30310_e47835_d_n9, assign30310_e47835_d_n12, assign30310_e47835_d_n14, assign30310_e47835_d_n15, assign30310_e47835_d_n16, assign30310_e47835_d_n17, assign30310_e47835_d_n18, assign30310_e47835_d_n19, assign30310_e47835_d_n20, assign30310_e47835_d_n21, assign30310_e47835_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30310_e47832: f64 = (var_ndx / var_cch);
        let assign30310_e47833: f64 = (var_vgod - assign30310_e47832);
        (assign30310_e47833, (var_vgod_dn0 - (var_ndx_dn0 / var_cch)), (var_vgod_dn1 - (var_ndx_dn1 / var_cch)), (var_vgod_dn2 - (var_ndx_dn2 / var_cch)), (var_vgod_dn3 - (var_ndx_dn3 / var_cch)), (var_vgod_dn4 - (var_ndx_dn4 / var_cch)), (var_vgod_dn5 - (var_ndx_dn5 / var_cch)), (var_vgod_dn6 - (var_ndx_dn6 / var_cch)), (var_vgod_dn7 - (var_ndx_dn7 / var_cch)), (var_vgod_dn8 - (var_ndx_dn8 / var_cch)), (var_vgod_dn9 - (var_ndx_dn9 / var_cch)), (var_vgod_dn12 - (var_ndx_dn12 / var_cch)), (var_vgod_dn14 - (var_ndx_dn14 / var_cch)), (var_vgod_dn15 - (var_ndx_dn15 / var_cch)), (var_vgod_dn16 - (var_ndx_dn16 / var_cch)), (var_vgod_dn17 - (var_ndx_dn17 / var_cch)), (var_vgod_dn18 - (var_ndx_dn18 / var_cch)), (var_vgod_dn19 - (var_ndx_dn19 / var_cch)), (var_vgod_dn20 - (var_ndx_dn20 / var_cch)), (var_vgod_dn21 - (var_ndx_dn21 / var_cch)), (var_vgod_dn22 - (var_ndx_dn22 / var_cch)),)
    } else {
        (var_ef1, var_ef1_dn0, var_ef1_dn1, var_ef1_dn2, var_ef1_dn3, var_ef1_dn4, var_ef1_dn5, var_ef1_dn6, var_ef1_dn7, var_ef1_dn8, var_ef1_dn9, var_ef1_dn12, var_ef1_dn14, var_ef1_dn15, var_ef1_dn16, var_ef1_dn17, var_ef1_dn18, var_ef1_dn19, var_ef1_dn20, var_ef1_dn21, var_ef1_dn22,)
    }
};
        var_ef1 = assign30310_e47835;
        var_ef1_dn0 = assign30310_e47835_d_n0;
        var_ef1_dn1 = assign30310_e47835_d_n1;
        var_ef1_dn2 = assign30310_e47835_d_n2;
        var_ef1_dn3 = assign30310_e47835_d_n3;
        var_ef1_dn4 = assign30310_e47835_d_n4;
        var_ef1_dn5 = assign30310_e47835_d_n5;
        var_ef1_dn6 = assign30310_e47835_d_n6;
        var_ef1_dn7 = assign30310_e47835_d_n7;
        var_ef1_dn8 = assign30310_e47835_d_n8;
        var_ef1_dn9 = assign30310_e47835_d_n9;
        var_ef1_dn12 = assign30310_e47835_d_n12;
        var_ef1_dn14 = assign30310_e47835_d_n14;
        var_ef1_dn15 = assign30310_e47835_d_n15;
        var_ef1_dn16 = assign30310_e47835_d_n16;
        var_ef1_dn17 = assign30310_e47835_d_n17;
        var_ef1_dn18 = assign30310_e47835_d_n18;
        var_ef1_dn19 = assign30310_e47835_d_n19;
        var_ef1_dn20 = assign30310_e47835_d_n20;
        var_ef1_dn21 = assign30310_e47835_d_n21;
        var_ef1_dn22 = assign30310_e47835_d_n22;

        let assign30320_e47838: f64 = (var_ef1 - var_vgod);
        let assign30320_e47839: f64 = (assign30320_e47838).abs();
        let assign30320_e47841: f64 = if assign30320_e47839 > 1e-19 { 1.0 } else { 0.0 };
        var_guard518 = assign30320_e47841;

        *var_dvgod_slot = var_dvgod;
        *var_dvgod_dn0_slot = var_dvgod_dn0;
        *var_dvgod_dn1_slot = var_dvgod_dn1;
        *var_dvgod_dn12_slot = var_dvgod_dn12;
        *var_dvgod_dn14_slot = var_dvgod_dn14;
        *var_dvgod_dn15_slot = var_dvgod_dn15;
        *var_dvgod_dn16_slot = var_dvgod_dn16;
        *var_dvgod_dn17_slot = var_dvgod_dn17;
        *var_dvgod_dn18_slot = var_dvgod_dn18;
        *var_dvgod_dn19_slot = var_dvgod_dn19;
        *var_dvgod_dn2_slot = var_dvgod_dn2;
        *var_dvgod_dn20_slot = var_dvgod_dn20;
        *var_dvgod_dn21_slot = var_dvgod_dn21;
        *var_dvgod_dn22_slot = var_dvgod_dn22;
        *var_dvgod_dn3_slot = var_dvgod_dn3;
        *var_dvgod_dn4_slot = var_dvgod_dn4;
        *var_dvgod_dn5_slot = var_dvgod_dn5;
        *var_dvgod_dn6_slot = var_dvgod_dn6;
        *var_dvgod_dn7_slot = var_dvgod_dn7;
        *var_dvgod_dn8_slot = var_dvgod_dn8;
        *var_dvgod_dn9_slot = var_dvgod_dn9;
        *var_dvgon_slot = var_dvgon;
        *var_dvgon_dn0_slot = var_dvgon_dn0;
        *var_dvgon_dn1_slot = var_dvgon_dn1;
        *var_dvgon_dn12_slot = var_dvgon_dn12;
        *var_dvgon_dn14_slot = var_dvgon_dn14;
        *var_dvgon_dn15_slot = var_dvgon_dn15;
        *var_dvgon_dn16_slot = var_dvgon_dn16;
        *var_dvgon_dn17_slot = var_dvgon_dn17;
        *var_dvgon_dn18_slot = var_dvgon_dn18;
        *var_dvgon_dn19_slot = var_dvgon_dn19;
        *var_dvgon_dn2_slot = var_dvgon_dn2;
        *var_dvgon_dn20_slot = var_dvgon_dn20;
        *var_dvgon_dn21_slot = var_dvgon_dn21;
        *var_dvgon_dn22_slot = var_dvgon_dn22;
        *var_dvgon_dn3_slot = var_dvgon_dn3;
        *var_dvgon_dn4_slot = var_dvgon_dn4;
        *var_dvgon_dn5_slot = var_dvgon_dn5;
        *var_dvgon_dn6_slot = var_dvgon_dn6;
        *var_dvgon_dn7_slot = var_dvgon_dn7;
        *var_dvgon_dn8_slot = var_dvgon_dn8;
        *var_dvgon_dn9_slot = var_dvgon_dn9;
        *var_ef1_slot = var_ef1;
        *var_ef1_dn0_slot = var_ef1_dn0;
        *var_ef1_dn1_slot = var_ef1_dn1;
        *var_ef1_dn12_slot = var_ef1_dn12;
        *var_ef1_dn14_slot = var_ef1_dn14;
        *var_ef1_dn15_slot = var_ef1_dn15;
        *var_ef1_dn16_slot = var_ef1_dn16;
        *var_ef1_dn17_slot = var_ef1_dn17;
        *var_ef1_dn18_slot = var_ef1_dn18;
        *var_ef1_dn19_slot = var_ef1_dn19;
        *var_ef1_dn2_slot = var_ef1_dn2;
        *var_ef1_dn20_slot = var_ef1_dn20;
        *var_ef1_dn21_slot = var_ef1_dn21;
        *var_ef1_dn22_slot = var_ef1_dn22;
        *var_ef1_dn3_slot = var_ef1_dn3;
        *var_ef1_dn4_slot = var_ef1_dn4;
        *var_ef1_dn5_slot = var_ef1_dn5;
        *var_ef1_dn6_slot = var_ef1_dn6;
        *var_ef1_dn7_slot = var_ef1_dn7;
        *var_ef1_dn8_slot = var_ef1_dn8;
        *var_ef1_dn9_slot = var_ef1_dn9;
        *var_guard517_slot = var_guard517;
        *var_guard518_slot = var_guard518;
        *var_hx_slot = var_hx;
        *var_hx_dn0_slot = var_hx_dn0;
        *var_hx_dn1_slot = var_hx_dn1;
        *var_hx_dn12_slot = var_hx_dn12;
        *var_hx_dn14_slot = var_hx_dn14;
        *var_hx_dn15_slot = var_hx_dn15;
        *var_hx_dn16_slot = var_hx_dn16;
        *var_hx_dn17_slot = var_hx_dn17;
        *var_hx_dn18_slot = var_hx_dn18;
        *var_hx_dn19_slot = var_hx_dn19;
        *var_hx_dn2_slot = var_hx_dn2;
        *var_hx_dn20_slot = var_hx_dn20;
        *var_hx_dn21_slot = var_hx_dn21;
        *var_hx_dn22_slot = var_hx_dn22;
        *var_hx_dn3_slot = var_hx_dn3;
        *var_hx_dn4_slot = var_hx_dn4;
        *var_hx_dn5_slot = var_hx_dn5;
        *var_hx_dn6_slot = var_hx_dn6;
        *var_hx_dn7_slot = var_hx_dn7;
        *var_hx_dn8_slot = var_hx_dn8;
        *var_hx_dn9_slot = var_hx_dn9;
        *var_ndx_slot = var_ndx;
        *var_ndx_dn0_slot = var_ndx_dn0;
        *var_ndx_dn1_slot = var_ndx_dn1;
        *var_ndx_dn12_slot = var_ndx_dn12;
        *var_ndx_dn14_slot = var_ndx_dn14;
        *var_ndx_dn15_slot = var_ndx_dn15;
        *var_ndx_dn16_slot = var_ndx_dn16;
        *var_ndx_dn17_slot = var_ndx_dn17;
        *var_ndx_dn18_slot = var_ndx_dn18;
        *var_ndx_dn19_slot = var_ndx_dn19;
        *var_ndx_dn2_slot = var_ndx_dn2;
        *var_ndx_dn20_slot = var_ndx_dn20;
        *var_ndx_dn21_slot = var_ndx_dn21;
        *var_ndx_dn22_slot = var_ndx_dn22;
        *var_ndx_dn3_slot = var_ndx_dn3;
        *var_ndx_dn4_slot = var_ndx_dn4;
        *var_ndx_dn5_slot = var_ndx_dn5;
        *var_ndx_dn6_slot = var_ndx_dn6;
        *var_ndx_dn7_slot = var_ndx_dn7;
        *var_ndx_dn8_slot = var_ndx_dn8;
        *var_ndx_dn9_slot = var_ndx_dn9;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn1_slot = var_t0_dn1;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn14_slot = var_t0_dn14;
        *var_t0_dn15_slot = var_t0_dn15;
        *var_t0_dn16_slot = var_t0_dn16;
        *var_t0_dn17_slot = var_t0_dn17;
        *var_t0_dn18_slot = var_t0_dn18;
        *var_t0_dn19_slot = var_t0_dn19;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn20_slot = var_t0_dn20;
        *var_t0_dn21_slot = var_t0_dn21;
        *var_t0_dn22_slot = var_t0_dn22;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn1_slot = var_t1_dn1;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn15_slot = var_t1_dn15;
        *var_t1_dn16_slot = var_t1_dn16;
        *var_t1_dn17_slot = var_t1_dn17;
        *var_t1_dn18_slot = var_t1_dn18;
        *var_t1_dn19_slot = var_t1_dn19;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn20_slot = var_t1_dn20;
        *var_t1_dn21_slot = var_t1_dn21;
        *var_t1_dn22_slot = var_t1_dn22;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn1_slot = var_t2_dn1;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn15_slot = var_t2_dn15;
        *var_t2_dn16_slot = var_t2_dn16;
        *var_t2_dn17_slot = var_t2_dn17;
        *var_t2_dn18_slot = var_t2_dn18;
        *var_t2_dn19_slot = var_t2_dn19;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn20_slot = var_t2_dn20;
        *var_t2_dn21_slot = var_t2_dn21;
        *var_t2_dn22_slot = var_t2_dn22;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_vdeff_slot = var_vdeff;
        *var_vdeff_dn0_slot = var_vdeff_dn0;
        *var_vdeff_dn1_slot = var_vdeff_dn1;
        *var_vdeff_dn12_slot = var_vdeff_dn12;
        *var_vdeff_dn14_slot = var_vdeff_dn14;
        *var_vdeff_dn15_slot = var_vdeff_dn15;
        *var_vdeff_dn16_slot = var_vdeff_dn16;
        *var_vdeff_dn17_slot = var_vdeff_dn17;
        *var_vdeff_dn18_slot = var_vdeff_dn18;
        *var_vdeff_dn19_slot = var_vdeff_dn19;
        *var_vdeff_dn2_slot = var_vdeff_dn2;
        *var_vdeff_dn20_slot = var_vdeff_dn20;
        *var_vdeff_dn21_slot = var_vdeff_dn21;
        *var_vdeff_dn22_slot = var_vdeff_dn22;
        *var_vdeff_dn3_slot = var_vdeff_dn3;
        *var_vdeff_dn4_slot = var_vdeff_dn4;
        *var_vdeff_dn5_slot = var_vdeff_dn5;
        *var_vdeff_dn6_slot = var_vdeff_dn6;
        *var_vdeff_dn7_slot = var_vdeff_dn7;
        *var_vdeff_dn8_slot = var_vdeff_dn8;
        *var_vdeff_dn9_slot = var_vdeff_dn9;
        *var_vgdeff_slot = var_vgdeff;
        *var_vgdeff_dn0_slot = var_vgdeff_dn0;
        *var_vgdeff_dn1_slot = var_vgdeff_dn1;
        *var_vgdeff_dn12_slot = var_vgdeff_dn12;
        *var_vgdeff_dn14_slot = var_vgdeff_dn14;
        *var_vgdeff_dn15_slot = var_vgdeff_dn15;
        *var_vgdeff_dn16_slot = var_vgdeff_dn16;
        *var_vgdeff_dn17_slot = var_vgdeff_dn17;
        *var_vgdeff_dn18_slot = var_vgdeff_dn18;
        *var_vgdeff_dn19_slot = var_vgdeff_dn19;
        *var_vgdeff_dn2_slot = var_vgdeff_dn2;
        *var_vgdeff_dn20_slot = var_vgdeff_dn20;
        *var_vgdeff_dn21_slot = var_vgdeff_dn21;
        *var_vgdeff_dn22_slot = var_vgdeff_dn22;
        *var_vgdeff_dn3_slot = var_vgdeff_dn3;
        *var_vgdeff_dn4_slot = var_vgdeff_dn4;
        *var_vgdeff_dn5_slot = var_vgdeff_dn5;
        *var_vgdeff_dn6_slot = var_vgdeff_dn6;
        *var_vgdeff_dn7_slot = var_vgdeff_dn7;
        *var_vgdeff_dn8_slot = var_vgdeff_dn8;
        *var_vgdeff_dn9_slot = var_vgdeff_dn9;
        *var_vgod_slot = var_vgod;
        *var_vgod_dn0_slot = var_vgod_dn0;
        *var_vgod_dn1_slot = var_vgod_dn1;
        *var_vgod_dn12_slot = var_vgod_dn12;
        *var_vgod_dn14_slot = var_vgod_dn14;
        *var_vgod_dn15_slot = var_vgod_dn15;
        *var_vgod_dn16_slot = var_vgod_dn16;
        *var_vgod_dn17_slot = var_vgod_dn17;
        *var_vgod_dn18_slot = var_vgod_dn18;
        *var_vgod_dn19_slot = var_vgod_dn19;
        *var_vgod_dn2_slot = var_vgod_dn2;
        *var_vgod_dn20_slot = var_vgod_dn20;
        *var_vgod_dn21_slot = var_vgod_dn21;
        *var_vgod_dn22_slot = var_vgod_dn22;
        *var_vgod_dn3_slot = var_vgod_dn3;
        *var_vgod_dn4_slot = var_vgod_dn4;
        *var_vgod_dn5_slot = var_vgod_dn5;
        *var_vgod_dn6_slot = var_vgod_dn6;
        *var_vgod_dn7_slot = var_vgod_dn7;
        *var_vgod_dn8_slot = var_vgod_dn8;
        *var_vgod_dn9_slot = var_vgod_dn9;
        *var_vgodp_slot = var_vgodp;
        *var_vgodp_dn0_slot = var_vgodp_dn0;
        *var_vgodp_dn1_slot = var_vgodp_dn1;
        *var_vgodp_dn12_slot = var_vgodp_dn12;
        *var_vgodp_dn14_slot = var_vgodp_dn14;
        *var_vgodp_dn15_slot = var_vgodp_dn15;
        *var_vgodp_dn16_slot = var_vgodp_dn16;
        *var_vgodp_dn17_slot = var_vgodp_dn17;
        *var_vgodp_dn18_slot = var_vgodp_dn18;
        *var_vgodp_dn19_slot = var_vgodp_dn19;
        *var_vgodp_dn2_slot = var_vgodp_dn2;
        *var_vgodp_dn20_slot = var_vgodp_dn20;
        *var_vgodp_dn21_slot = var_vgodp_dn21;
        *var_vgodp_dn22_slot = var_vgodp_dn22;
        *var_vgodp_dn3_slot = var_vgodp_dn3;
        *var_vgodp_dn4_slot = var_vgodp_dn4;
        *var_vgodp_dn5_slot = var_vgodp_dn5;
        *var_vgodp_dn6_slot = var_vgodp_dn6;
        *var_vgodp_dn7_slot = var_vgodp_dn7;
        *var_vgodp_dn8_slot = var_vgodp_dn8;
        *var_vgodp_dn9_slot = var_vgodp_dn9;
        *var_vgop_slot = var_vgop;
        *var_vgop_dn0_slot = var_vgop_dn0;
        *var_vgop_dn1_slot = var_vgop_dn1;
        *var_vgop_dn12_slot = var_vgop_dn12;
        *var_vgop_dn14_slot = var_vgop_dn14;
        *var_vgop_dn15_slot = var_vgop_dn15;
        *var_vgop_dn16_slot = var_vgop_dn16;
        *var_vgop_dn17_slot = var_vgop_dn17;
        *var_vgop_dn18_slot = var_vgop_dn18;
        *var_vgop_dn19_slot = var_vgop_dn19;
        *var_vgop_dn2_slot = var_vgop_dn2;
        *var_vgop_dn20_slot = var_vgop_dn20;
        *var_vgop_dn21_slot = var_vgop_dn21;
        *var_vgop_dn22_slot = var_vgop_dn22;
        *var_vgop_dn3_slot = var_vgop_dn3;
        *var_vgop_dn4_slot = var_vgop_dn4;
        *var_vgop_dn5_slot = var_vgop_dn5;
        *var_vgop_dn6_slot = var_vgop_dn6;
        *var_vgop_dn7_slot = var_vgop_dn7;
        *var_vgop_dn8_slot = var_vgop_dn8;
        *var_vgop_dn9_slot = var_vgop_dn9;
    }

    pub(super) fn stamp_transient_block_178(
        p: &Parameters,
        var_cch: f64,
        var_ef1: f64,
        var_ef1_dn0: f64,
        var_ef1_dn1: f64,
        var_ef1_dn12: f64,
        var_ef1_dn14: f64,
        var_ef1_dn15: f64,
        var_ef1_dn16: f64,
        var_ef1_dn17: f64,
        var_ef1_dn18: f64,
        var_ef1_dn19: f64,
        var_ef1_dn2: f64,
        var_ef1_dn20: f64,
        var_ef1_dn21: f64,
        var_ef1_dn22: f64,
        var_ef1_dn3: f64,
        var_ef1_dn4: f64,
        var_ef1_dn5: f64,
        var_ef1_dn6: f64,
        var_ef1_dn7: f64,
        var_ef1_dn8: f64,
        var_ef1_dn9: f64,
        var_guard504: f64,
        var_guard513: f64,
        var_guard518: f64,
        var_vgod: f64,
        var_vgod_dn0: f64,
        var_vgod_dn1: f64,
        var_vgod_dn12: f64,
        var_vgod_dn14: f64,
        var_vgod_dn15: f64,
        var_vgod_dn16: f64,
        var_vgod_dn17: f64,
        var_vgod_dn18: f64,
        var_vgod_dn19: f64,
        var_vgod_dn2: f64,
        var_vgod_dn20: f64,
        var_vgod_dn21: f64,
        var_vgod_dn22: f64,
        var_vgod_dn3: f64,
        var_vgod_dn4: f64,
        var_vgod_dn5: f64,
        var_vgod_dn6: f64,
        var_vgod_dn7: f64,
        var_vgod_dn8: f64,
        var_vgod_dn9: f64,
        var_vtv: f64,
        var_vtv_dn15: f64,
        var_vtv_dn16: f64,
        var_vtv_dn17: f64,
        var_vtv_dn18: f64,
        var_vtv_dn19: f64,
        var_vtv_dn20: f64,
        var_vtv_dn21: f64,
        var_vtv_dn22: f64,
        var_vtv_dn4: f64,
        var_vtv_dn6: f64,
        var_vtv_dn7: f64,
        var_vtv_dn8: f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn1_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn14_slot: &mut f64,
        var_t0_dn15_slot: &mut f64,
        var_t0_dn16_slot: &mut f64,
        var_t0_dn17_slot: &mut f64,
        var_t0_dn18_slot: &mut f64,
        var_t0_dn19_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn20_slot: &mut f64,
        var_t0_dn21_slot: &mut f64,
        var_t0_dn22_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn1_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn15_slot: &mut f64,
        var_t1_dn16_slot: &mut f64,
        var_t1_dn17_slot: &mut f64,
        var_t1_dn18_slot: &mut f64,
        var_t1_dn19_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn20_slot: &mut f64,
        var_t1_dn21_slot: &mut f64,
        var_t1_dn22_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn1_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn15_slot: &mut f64,
        var_t2_dn16_slot: &mut f64,
        var_t2_dn17_slot: &mut f64,
        var_t2_dn18_slot: &mut f64,
        var_t2_dn19_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn20_slot: &mut f64,
        var_t2_dn21_slot: &mut f64,
        var_t2_dn22_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn1_slot: &mut f64,
        var_t4_dn12_slot: &mut f64,
        var_t4_dn14_slot: &mut f64,
        var_t4_dn15_slot: &mut f64,
        var_t4_dn16_slot: &mut f64,
        var_t4_dn17_slot: &mut f64,
        var_t4_dn18_slot: &mut f64,
        var_t4_dn19_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn20_slot: &mut f64,
        var_t4_dn21_slot: &mut f64,
        var_t4_dn22_slot: &mut f64,
        var_t4_dn3_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_dn9_slot: &mut f64,
        var_t5dg0_slot: &mut f64,
        var_t5dg0_dn0_slot: &mut f64,
        var_t5dg0_dn1_slot: &mut f64,
        var_t5dg0_dn12_slot: &mut f64,
        var_t5dg0_dn14_slot: &mut f64,
        var_t5dg0_dn15_slot: &mut f64,
        var_t5dg0_dn16_slot: &mut f64,
        var_t5dg0_dn17_slot: &mut f64,
        var_t5dg0_dn18_slot: &mut f64,
        var_t5dg0_dn19_slot: &mut f64,
        var_t5dg0_dn2_slot: &mut f64,
        var_t5dg0_dn20_slot: &mut f64,
        var_t5dg0_dn21_slot: &mut f64,
        var_t5dg0_dn22_slot: &mut f64,
        var_t5dg0_dn3_slot: &mut f64,
        var_t5dg0_dn4_slot: &mut f64,
        var_t5dg0_dn5_slot: &mut f64,
        var_t5dg0_dn6_slot: &mut f64,
        var_t5dg0_dn7_slot: &mut f64,
        var_t5dg0_dn8_slot: &mut f64,
        var_t5dg0_dn9_slot: &mut f64,
        var_t5ng0_slot: &mut f64,
        var_t5ng0_dn0_slot: &mut f64,
        var_t5ng0_dn1_slot: &mut f64,
        var_t5ng0_dn12_slot: &mut f64,
        var_t5ng0_dn14_slot: &mut f64,
        var_t5ng0_dn15_slot: &mut f64,
        var_t5ng0_dn16_slot: &mut f64,
        var_t5ng0_dn17_slot: &mut f64,
        var_t5ng0_dn18_slot: &mut f64,
        var_t5ng0_dn19_slot: &mut f64,
        var_t5ng0_dn2_slot: &mut f64,
        var_t5ng0_dn20_slot: &mut f64,
        var_t5ng0_dn21_slot: &mut f64,
        var_t5ng0_dn22_slot: &mut f64,
        var_t5ng0_dn3_slot: &mut f64,
        var_t5ng0_dn4_slot: &mut f64,
        var_t5ng0_dn5_slot: &mut f64,
        var_t5ng0_dn6_slot: &mut f64,
        var_t5ng0_dn7_slot: &mut f64,
        var_t5ng0_dn8_slot: &mut f64,
        var_t5ng0_dn9_slot: &mut f64,
        var_tg0_slot: &mut f64,
        var_tg0_dn0_slot: &mut f64,
        var_tg0_dn1_slot: &mut f64,
        var_tg0_dn12_slot: &mut f64,
        var_tg0_dn14_slot: &mut f64,
        var_tg0_dn15_slot: &mut f64,
        var_tg0_dn16_slot: &mut f64,
        var_tg0_dn17_slot: &mut f64,
        var_tg0_dn18_slot: &mut f64,
        var_tg0_dn19_slot: &mut f64,
        var_tg0_dn2_slot: &mut f64,
        var_tg0_dn20_slot: &mut f64,
        var_tg0_dn21_slot: &mut f64,
        var_tg0_dn22_slot: &mut f64,
        var_tg0_dn3_slot: &mut f64,
        var_tg0_dn4_slot: &mut f64,
        var_tg0_dn5_slot: &mut f64,
        var_tg0_dn6_slot: &mut f64,
        var_tg0_dn7_slot: &mut f64,
        var_tg0_dn8_slot: &mut f64,
        var_tg0_dn9_slot: &mut f64,
        var_tg1_slot: &mut f64,
        var_tg1_dn0_slot: &mut f64,
        var_tg1_dn1_slot: &mut f64,
        var_tg1_dn12_slot: &mut f64,
        var_tg1_dn14_slot: &mut f64,
        var_tg1_dn15_slot: &mut f64,
        var_tg1_dn16_slot: &mut f64,
        var_tg1_dn17_slot: &mut f64,
        var_tg1_dn18_slot: &mut f64,
        var_tg1_dn19_slot: &mut f64,
        var_tg1_dn2_slot: &mut f64,
        var_tg1_dn20_slot: &mut f64,
        var_tg1_dn21_slot: &mut f64,
        var_tg1_dn22_slot: &mut f64,
        var_tg1_dn3_slot: &mut f64,
        var_tg1_dn4_slot: &mut f64,
        var_tg1_dn5_slot: &mut f64,
        var_tg1_dn6_slot: &mut f64,
        var_tg1_dn7_slot: &mut f64,
        var_tg1_dn8_slot: &mut f64,
        var_tg1_dn9_slot: &mut f64,
        var_vgef1_slot: &mut f64,
        var_vgef1_dn0_slot: &mut f64,
        var_vgef1_dn1_slot: &mut f64,
        var_vgef1_dn12_slot: &mut f64,
        var_vgef1_dn14_slot: &mut f64,
        var_vgef1_dn15_slot: &mut f64,
        var_vgef1_dn16_slot: &mut f64,
        var_vgef1_dn17_slot: &mut f64,
        var_vgef1_dn18_slot: &mut f64,
        var_vgef1_dn19_slot: &mut f64,
        var_vgef1_dn2_slot: &mut f64,
        var_vgef1_dn20_slot: &mut f64,
        var_vgef1_dn21_slot: &mut f64,
        var_vgef1_dn22_slot: &mut f64,
        var_vgef1_dn3_slot: &mut f64,
        var_vgef1_dn4_slot: &mut f64,
        var_vgef1_dn5_slot: &mut f64,
        var_vgef1_dn6_slot: &mut f64,
        var_vgef1_dn7_slot: &mut f64,
        var_vgef1_dn8_slot: &mut f64,
        var_vgef1_dn9_slot: &mut f64,
        var_vgef23g0_slot: &mut f64,
        var_vgef23g0_dn0_slot: &mut f64,
        var_vgef23g0_dn1_slot: &mut f64,
        var_vgef23g0_dn12_slot: &mut f64,
        var_vgef23g0_dn14_slot: &mut f64,
        var_vgef23g0_dn15_slot: &mut f64,
        var_vgef23g0_dn16_slot: &mut f64,
        var_vgef23g0_dn17_slot: &mut f64,
        var_vgef23g0_dn18_slot: &mut f64,
        var_vgef23g0_dn19_slot: &mut f64,
        var_vgef23g0_dn2_slot: &mut f64,
        var_vgef23g0_dn20_slot: &mut f64,
        var_vgef23g0_dn21_slot: &mut f64,
        var_vgef23g0_dn22_slot: &mut f64,
        var_vgef23g0_dn3_slot: &mut f64,
        var_vgef23g0_dn4_slot: &mut f64,
        var_vgef23g0_dn5_slot: &mut f64,
        var_vgef23g0_dn6_slot: &mut f64,
        var_vgef23g0_dn7_slot: &mut f64,
        var_vgef23g0_dn8_slot: &mut f64,
        var_vgef23g0_dn9_slot: &mut f64,
        var_vgef23g1_slot: &mut f64,
        var_vgef23g1_dn0_slot: &mut f64,
        var_vgef23g1_dn1_slot: &mut f64,
        var_vgef23g1_dn12_slot: &mut f64,
        var_vgef23g1_dn14_slot: &mut f64,
        var_vgef23g1_dn15_slot: &mut f64,
        var_vgef23g1_dn16_slot: &mut f64,
        var_vgef23g1_dn17_slot: &mut f64,
        var_vgef23g1_dn18_slot: &mut f64,
        var_vgef23g1_dn19_slot: &mut f64,
        var_vgef23g1_dn2_slot: &mut f64,
        var_vgef23g1_dn20_slot: &mut f64,
        var_vgef23g1_dn21_slot: &mut f64,
        var_vgef23g1_dn22_slot: &mut f64,
        var_vgef23g1_dn3_slot: &mut f64,
        var_vgef23g1_dn4_slot: &mut f64,
        var_vgef23g1_dn5_slot: &mut f64,
        var_vgef23g1_dn6_slot: &mut f64,
        var_vgef23g1_dn7_slot: &mut f64,
        var_vgef23g1_dn8_slot: &mut f64,
        var_vgef23g1_dn9_slot: &mut f64,
        var_vgefm13g0_slot: &mut f64,
        var_vgefm13g0_dn0_slot: &mut f64,
        var_vgefm13g0_dn1_slot: &mut f64,
        var_vgefm13g0_dn12_slot: &mut f64,
        var_vgefm13g0_dn14_slot: &mut f64,
        var_vgefm13g0_dn15_slot: &mut f64,
        var_vgefm13g0_dn16_slot: &mut f64,
        var_vgefm13g0_dn17_slot: &mut f64,
        var_vgefm13g0_dn18_slot: &mut f64,
        var_vgefm13g0_dn19_slot: &mut f64,
        var_vgefm13g0_dn2_slot: &mut f64,
        var_vgefm13g0_dn20_slot: &mut f64,
        var_vgefm13g0_dn21_slot: &mut f64,
        var_vgefm13g0_dn22_slot: &mut f64,
        var_vgefm13g0_dn3_slot: &mut f64,
        var_vgefm13g0_dn4_slot: &mut f64,
        var_vgefm13g0_dn5_slot: &mut f64,
        var_vgefm13g0_dn6_slot: &mut f64,
        var_vgefm13g0_dn7_slot: &mut f64,
        var_vgefm13g0_dn8_slot: &mut f64,
        var_vgefm13g0_dn9_slot: &mut f64,
        var_vgefm13g1_slot: &mut f64,
        var_vgefm13g1_dn0_slot: &mut f64,
        var_vgefm13g1_dn1_slot: &mut f64,
        var_vgefm13g1_dn12_slot: &mut f64,
        var_vgefm13g1_dn14_slot: &mut f64,
        var_vgefm13g1_dn15_slot: &mut f64,
        var_vgefm13g1_dn16_slot: &mut f64,
        var_vgefm13g1_dn17_slot: &mut f64,
        var_vgefm13g1_dn18_slot: &mut f64,
        var_vgefm13g1_dn19_slot: &mut f64,
        var_vgefm13g1_dn2_slot: &mut f64,
        var_vgefm13g1_dn20_slot: &mut f64,
        var_vgefm13g1_dn21_slot: &mut f64,
        var_vgefm13g1_dn22_slot: &mut f64,
        var_vgefm13g1_dn3_slot: &mut f64,
        var_vgefm13g1_dn4_slot: &mut f64,
        var_vgefm13g1_dn5_slot: &mut f64,
        var_vgefm13g1_dn6_slot: &mut f64,
        var_vgefm13g1_dn7_slot: &mut f64,
        var_vgefm13g1_dn8_slot: &mut f64,
        var_vgefm13g1_dn9_slot: &mut f64,
    ) {
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn1: f64 = *var_t0_dn1_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn14: f64 = *var_t0_dn14_slot;
        let mut var_t0_dn15: f64 = *var_t0_dn15_slot;
        let mut var_t0_dn16: f64 = *var_t0_dn16_slot;
        let mut var_t0_dn17: f64 = *var_t0_dn17_slot;
        let mut var_t0_dn18: f64 = *var_t0_dn18_slot;
        let mut var_t0_dn19: f64 = *var_t0_dn19_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn20: f64 = *var_t0_dn20_slot;
        let mut var_t0_dn21: f64 = *var_t0_dn21_slot;
        let mut var_t0_dn22: f64 = *var_t0_dn22_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn1: f64 = *var_t1_dn1_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn15: f64 = *var_t1_dn15_slot;
        let mut var_t1_dn16: f64 = *var_t1_dn16_slot;
        let mut var_t1_dn17: f64 = *var_t1_dn17_slot;
        let mut var_t1_dn18: f64 = *var_t1_dn18_slot;
        let mut var_t1_dn19: f64 = *var_t1_dn19_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn20: f64 = *var_t1_dn20_slot;
        let mut var_t1_dn21: f64 = *var_t1_dn21_slot;
        let mut var_t1_dn22: f64 = *var_t1_dn22_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn1: f64 = *var_t2_dn1_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn15: f64 = *var_t2_dn15_slot;
        let mut var_t2_dn16: f64 = *var_t2_dn16_slot;
        let mut var_t2_dn17: f64 = *var_t2_dn17_slot;
        let mut var_t2_dn18: f64 = *var_t2_dn18_slot;
        let mut var_t2_dn19: f64 = *var_t2_dn19_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn20: f64 = *var_t2_dn20_slot;
        let mut var_t2_dn21: f64 = *var_t2_dn21_slot;
        let mut var_t2_dn22: f64 = *var_t2_dn22_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn1: f64 = *var_t4_dn1_slot;
        let mut var_t4_dn12: f64 = *var_t4_dn12_slot;
        let mut var_t4_dn14: f64 = *var_t4_dn14_slot;
        let mut var_t4_dn15: f64 = *var_t4_dn15_slot;
        let mut var_t4_dn16: f64 = *var_t4_dn16_slot;
        let mut var_t4_dn17: f64 = *var_t4_dn17_slot;
        let mut var_t4_dn18: f64 = *var_t4_dn18_slot;
        let mut var_t4_dn19: f64 = *var_t4_dn19_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn20: f64 = *var_t4_dn20_slot;
        let mut var_t4_dn21: f64 = *var_t4_dn21_slot;
        let mut var_t4_dn22: f64 = *var_t4_dn22_slot;
        let mut var_t4_dn3: f64 = *var_t4_dn3_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_dn9: f64 = *var_t4_dn9_slot;
        let mut var_t5dg0: f64 = *var_t5dg0_slot;
        let mut var_t5dg0_dn0: f64 = *var_t5dg0_dn0_slot;
        let mut var_t5dg0_dn1: f64 = *var_t5dg0_dn1_slot;
        let mut var_t5dg0_dn12: f64 = *var_t5dg0_dn12_slot;
        let mut var_t5dg0_dn14: f64 = *var_t5dg0_dn14_slot;
        let mut var_t5dg0_dn15: f64 = *var_t5dg0_dn15_slot;
        let mut var_t5dg0_dn16: f64 = *var_t5dg0_dn16_slot;
        let mut var_t5dg0_dn17: f64 = *var_t5dg0_dn17_slot;
        let mut var_t5dg0_dn18: f64 = *var_t5dg0_dn18_slot;
        let mut var_t5dg0_dn19: f64 = *var_t5dg0_dn19_slot;
        let mut var_t5dg0_dn2: f64 = *var_t5dg0_dn2_slot;
        let mut var_t5dg0_dn20: f64 = *var_t5dg0_dn20_slot;
        let mut var_t5dg0_dn21: f64 = *var_t5dg0_dn21_slot;
        let mut var_t5dg0_dn22: f64 = *var_t5dg0_dn22_slot;
        let mut var_t5dg0_dn3: f64 = *var_t5dg0_dn3_slot;
        let mut var_t5dg0_dn4: f64 = *var_t5dg0_dn4_slot;
        let mut var_t5dg0_dn5: f64 = *var_t5dg0_dn5_slot;
        let mut var_t5dg0_dn6: f64 = *var_t5dg0_dn6_slot;
        let mut var_t5dg0_dn7: f64 = *var_t5dg0_dn7_slot;
        let mut var_t5dg0_dn8: f64 = *var_t5dg0_dn8_slot;
        let mut var_t5dg0_dn9: f64 = *var_t5dg0_dn9_slot;
        let mut var_t5ng0: f64 = *var_t5ng0_slot;
        let mut var_t5ng0_dn0: f64 = *var_t5ng0_dn0_slot;
        let mut var_t5ng0_dn1: f64 = *var_t5ng0_dn1_slot;
        let mut var_t5ng0_dn12: f64 = *var_t5ng0_dn12_slot;
        let mut var_t5ng0_dn14: f64 = *var_t5ng0_dn14_slot;
        let mut var_t5ng0_dn15: f64 = *var_t5ng0_dn15_slot;
        let mut var_t5ng0_dn16: f64 = *var_t5ng0_dn16_slot;
        let mut var_t5ng0_dn17: f64 = *var_t5ng0_dn17_slot;
        let mut var_t5ng0_dn18: f64 = *var_t5ng0_dn18_slot;
        let mut var_t5ng0_dn19: f64 = *var_t5ng0_dn19_slot;
        let mut var_t5ng0_dn2: f64 = *var_t5ng0_dn2_slot;
        let mut var_t5ng0_dn20: f64 = *var_t5ng0_dn20_slot;
        let mut var_t5ng0_dn21: f64 = *var_t5ng0_dn21_slot;
        let mut var_t5ng0_dn22: f64 = *var_t5ng0_dn22_slot;
        let mut var_t5ng0_dn3: f64 = *var_t5ng0_dn3_slot;
        let mut var_t5ng0_dn4: f64 = *var_t5ng0_dn4_slot;
        let mut var_t5ng0_dn5: f64 = *var_t5ng0_dn5_slot;
        let mut var_t5ng0_dn6: f64 = *var_t5ng0_dn6_slot;
        let mut var_t5ng0_dn7: f64 = *var_t5ng0_dn7_slot;
        let mut var_t5ng0_dn8: f64 = *var_t5ng0_dn8_slot;
        let mut var_t5ng0_dn9: f64 = *var_t5ng0_dn9_slot;
        let mut var_tg0: f64 = *var_tg0_slot;
        let mut var_tg0_dn0: f64 = *var_tg0_dn0_slot;
        let mut var_tg0_dn1: f64 = *var_tg0_dn1_slot;
        let mut var_tg0_dn12: f64 = *var_tg0_dn12_slot;
        let mut var_tg0_dn14: f64 = *var_tg0_dn14_slot;
        let mut var_tg0_dn15: f64 = *var_tg0_dn15_slot;
        let mut var_tg0_dn16: f64 = *var_tg0_dn16_slot;
        let mut var_tg0_dn17: f64 = *var_tg0_dn17_slot;
        let mut var_tg0_dn18: f64 = *var_tg0_dn18_slot;
        let mut var_tg0_dn19: f64 = *var_tg0_dn19_slot;
        let mut var_tg0_dn2: f64 = *var_tg0_dn2_slot;
        let mut var_tg0_dn20: f64 = *var_tg0_dn20_slot;
        let mut var_tg0_dn21: f64 = *var_tg0_dn21_slot;
        let mut var_tg0_dn22: f64 = *var_tg0_dn22_slot;
        let mut var_tg0_dn3: f64 = *var_tg0_dn3_slot;
        let mut var_tg0_dn4: f64 = *var_tg0_dn4_slot;
        let mut var_tg0_dn5: f64 = *var_tg0_dn5_slot;
        let mut var_tg0_dn6: f64 = *var_tg0_dn6_slot;
        let mut var_tg0_dn7: f64 = *var_tg0_dn7_slot;
        let mut var_tg0_dn8: f64 = *var_tg0_dn8_slot;
        let mut var_tg0_dn9: f64 = *var_tg0_dn9_slot;
        let mut var_tg1: f64 = *var_tg1_slot;
        let mut var_tg1_dn0: f64 = *var_tg1_dn0_slot;
        let mut var_tg1_dn1: f64 = *var_tg1_dn1_slot;
        let mut var_tg1_dn12: f64 = *var_tg1_dn12_slot;
        let mut var_tg1_dn14: f64 = *var_tg1_dn14_slot;
        let mut var_tg1_dn15: f64 = *var_tg1_dn15_slot;
        let mut var_tg1_dn16: f64 = *var_tg1_dn16_slot;
        let mut var_tg1_dn17: f64 = *var_tg1_dn17_slot;
        let mut var_tg1_dn18: f64 = *var_tg1_dn18_slot;
        let mut var_tg1_dn19: f64 = *var_tg1_dn19_slot;
        let mut var_tg1_dn2: f64 = *var_tg1_dn2_slot;
        let mut var_tg1_dn20: f64 = *var_tg1_dn20_slot;
        let mut var_tg1_dn21: f64 = *var_tg1_dn21_slot;
        let mut var_tg1_dn22: f64 = *var_tg1_dn22_slot;
        let mut var_tg1_dn3: f64 = *var_tg1_dn3_slot;
        let mut var_tg1_dn4: f64 = *var_tg1_dn4_slot;
        let mut var_tg1_dn5: f64 = *var_tg1_dn5_slot;
        let mut var_tg1_dn6: f64 = *var_tg1_dn6_slot;
        let mut var_tg1_dn7: f64 = *var_tg1_dn7_slot;
        let mut var_tg1_dn8: f64 = *var_tg1_dn8_slot;
        let mut var_tg1_dn9: f64 = *var_tg1_dn9_slot;
        let mut var_vgef1: f64 = *var_vgef1_slot;
        let mut var_vgef1_dn0: f64 = *var_vgef1_dn0_slot;
        let mut var_vgef1_dn1: f64 = *var_vgef1_dn1_slot;
        let mut var_vgef1_dn12: f64 = *var_vgef1_dn12_slot;
        let mut var_vgef1_dn14: f64 = *var_vgef1_dn14_slot;
        let mut var_vgef1_dn15: f64 = *var_vgef1_dn15_slot;
        let mut var_vgef1_dn16: f64 = *var_vgef1_dn16_slot;
        let mut var_vgef1_dn17: f64 = *var_vgef1_dn17_slot;
        let mut var_vgef1_dn18: f64 = *var_vgef1_dn18_slot;
        let mut var_vgef1_dn19: f64 = *var_vgef1_dn19_slot;
        let mut var_vgef1_dn2: f64 = *var_vgef1_dn2_slot;
        let mut var_vgef1_dn20: f64 = *var_vgef1_dn20_slot;
        let mut var_vgef1_dn21: f64 = *var_vgef1_dn21_slot;
        let mut var_vgef1_dn22: f64 = *var_vgef1_dn22_slot;
        let mut var_vgef1_dn3: f64 = *var_vgef1_dn3_slot;
        let mut var_vgef1_dn4: f64 = *var_vgef1_dn4_slot;
        let mut var_vgef1_dn5: f64 = *var_vgef1_dn5_slot;
        let mut var_vgef1_dn6: f64 = *var_vgef1_dn6_slot;
        let mut var_vgef1_dn7: f64 = *var_vgef1_dn7_slot;
        let mut var_vgef1_dn8: f64 = *var_vgef1_dn8_slot;
        let mut var_vgef1_dn9: f64 = *var_vgef1_dn9_slot;
        let mut var_vgef23g0: f64 = *var_vgef23g0_slot;
        let mut var_vgef23g0_dn0: f64 = *var_vgef23g0_dn0_slot;
        let mut var_vgef23g0_dn1: f64 = *var_vgef23g0_dn1_slot;
        let mut var_vgef23g0_dn12: f64 = *var_vgef23g0_dn12_slot;
        let mut var_vgef23g0_dn14: f64 = *var_vgef23g0_dn14_slot;
        let mut var_vgef23g0_dn15: f64 = *var_vgef23g0_dn15_slot;
        let mut var_vgef23g0_dn16: f64 = *var_vgef23g0_dn16_slot;
        let mut var_vgef23g0_dn17: f64 = *var_vgef23g0_dn17_slot;
        let mut var_vgef23g0_dn18: f64 = *var_vgef23g0_dn18_slot;
        let mut var_vgef23g0_dn19: f64 = *var_vgef23g0_dn19_slot;
        let mut var_vgef23g0_dn2: f64 = *var_vgef23g0_dn2_slot;
        let mut var_vgef23g0_dn20: f64 = *var_vgef23g0_dn20_slot;
        let mut var_vgef23g0_dn21: f64 = *var_vgef23g0_dn21_slot;
        let mut var_vgef23g0_dn22: f64 = *var_vgef23g0_dn22_slot;
        let mut var_vgef23g0_dn3: f64 = *var_vgef23g0_dn3_slot;
        let mut var_vgef23g0_dn4: f64 = *var_vgef23g0_dn4_slot;
        let mut var_vgef23g0_dn5: f64 = *var_vgef23g0_dn5_slot;
        let mut var_vgef23g0_dn6: f64 = *var_vgef23g0_dn6_slot;
        let mut var_vgef23g0_dn7: f64 = *var_vgef23g0_dn7_slot;
        let mut var_vgef23g0_dn8: f64 = *var_vgef23g0_dn8_slot;
        let mut var_vgef23g0_dn9: f64 = *var_vgef23g0_dn9_slot;
        let mut var_vgef23g1: f64 = *var_vgef23g1_slot;
        let mut var_vgef23g1_dn0: f64 = *var_vgef23g1_dn0_slot;
        let mut var_vgef23g1_dn1: f64 = *var_vgef23g1_dn1_slot;
        let mut var_vgef23g1_dn12: f64 = *var_vgef23g1_dn12_slot;
        let mut var_vgef23g1_dn14: f64 = *var_vgef23g1_dn14_slot;
        let mut var_vgef23g1_dn15: f64 = *var_vgef23g1_dn15_slot;
        let mut var_vgef23g1_dn16: f64 = *var_vgef23g1_dn16_slot;
        let mut var_vgef23g1_dn17: f64 = *var_vgef23g1_dn17_slot;
        let mut var_vgef23g1_dn18: f64 = *var_vgef23g1_dn18_slot;
        let mut var_vgef23g1_dn19: f64 = *var_vgef23g1_dn19_slot;
        let mut var_vgef23g1_dn2: f64 = *var_vgef23g1_dn2_slot;
        let mut var_vgef23g1_dn20: f64 = *var_vgef23g1_dn20_slot;
        let mut var_vgef23g1_dn21: f64 = *var_vgef23g1_dn21_slot;
        let mut var_vgef23g1_dn22: f64 = *var_vgef23g1_dn22_slot;
        let mut var_vgef23g1_dn3: f64 = *var_vgef23g1_dn3_slot;
        let mut var_vgef23g1_dn4: f64 = *var_vgef23g1_dn4_slot;
        let mut var_vgef23g1_dn5: f64 = *var_vgef23g1_dn5_slot;
        let mut var_vgef23g1_dn6: f64 = *var_vgef23g1_dn6_slot;
        let mut var_vgef23g1_dn7: f64 = *var_vgef23g1_dn7_slot;
        let mut var_vgef23g1_dn8: f64 = *var_vgef23g1_dn8_slot;
        let mut var_vgef23g1_dn9: f64 = *var_vgef23g1_dn9_slot;
        let mut var_vgefm13g0: f64 = *var_vgefm13g0_slot;
        let mut var_vgefm13g0_dn0: f64 = *var_vgefm13g0_dn0_slot;
        let mut var_vgefm13g0_dn1: f64 = *var_vgefm13g0_dn1_slot;
        let mut var_vgefm13g0_dn12: f64 = *var_vgefm13g0_dn12_slot;
        let mut var_vgefm13g0_dn14: f64 = *var_vgefm13g0_dn14_slot;
        let mut var_vgefm13g0_dn15: f64 = *var_vgefm13g0_dn15_slot;
        let mut var_vgefm13g0_dn16: f64 = *var_vgefm13g0_dn16_slot;
        let mut var_vgefm13g0_dn17: f64 = *var_vgefm13g0_dn17_slot;
        let mut var_vgefm13g0_dn18: f64 = *var_vgefm13g0_dn18_slot;
        let mut var_vgefm13g0_dn19: f64 = *var_vgefm13g0_dn19_slot;
        let mut var_vgefm13g0_dn2: f64 = *var_vgefm13g0_dn2_slot;
        let mut var_vgefm13g0_dn20: f64 = *var_vgefm13g0_dn20_slot;
        let mut var_vgefm13g0_dn21: f64 = *var_vgefm13g0_dn21_slot;
        let mut var_vgefm13g0_dn22: f64 = *var_vgefm13g0_dn22_slot;
        let mut var_vgefm13g0_dn3: f64 = *var_vgefm13g0_dn3_slot;
        let mut var_vgefm13g0_dn4: f64 = *var_vgefm13g0_dn4_slot;
        let mut var_vgefm13g0_dn5: f64 = *var_vgefm13g0_dn5_slot;
        let mut var_vgefm13g0_dn6: f64 = *var_vgefm13g0_dn6_slot;
        let mut var_vgefm13g0_dn7: f64 = *var_vgefm13g0_dn7_slot;
        let mut var_vgefm13g0_dn8: f64 = *var_vgefm13g0_dn8_slot;
        let mut var_vgefm13g0_dn9: f64 = *var_vgefm13g0_dn9_slot;
        let mut var_vgefm13g1: f64 = *var_vgefm13g1_slot;
        let mut var_vgefm13g1_dn0: f64 = *var_vgefm13g1_dn0_slot;
        let mut var_vgefm13g1_dn1: f64 = *var_vgefm13g1_dn1_slot;
        let mut var_vgefm13g1_dn12: f64 = *var_vgefm13g1_dn12_slot;
        let mut var_vgefm13g1_dn14: f64 = *var_vgefm13g1_dn14_slot;
        let mut var_vgefm13g1_dn15: f64 = *var_vgefm13g1_dn15_slot;
        let mut var_vgefm13g1_dn16: f64 = *var_vgefm13g1_dn16_slot;
        let mut var_vgefm13g1_dn17: f64 = *var_vgefm13g1_dn17_slot;
        let mut var_vgefm13g1_dn18: f64 = *var_vgefm13g1_dn18_slot;
        let mut var_vgefm13g1_dn19: f64 = *var_vgefm13g1_dn19_slot;
        let mut var_vgefm13g1_dn2: f64 = *var_vgefm13g1_dn2_slot;
        let mut var_vgefm13g1_dn20: f64 = *var_vgefm13g1_dn20_slot;
        let mut var_vgefm13g1_dn21: f64 = *var_vgefm13g1_dn21_slot;
        let mut var_vgefm13g1_dn22: f64 = *var_vgefm13g1_dn22_slot;
        let mut var_vgefm13g1_dn3: f64 = *var_vgefm13g1_dn3_slot;
        let mut var_vgefm13g1_dn4: f64 = *var_vgefm13g1_dn4_slot;
        let mut var_vgefm13g1_dn5: f64 = *var_vgefm13g1_dn5_slot;
        let mut var_vgefm13g1_dn6: f64 = *var_vgefm13g1_dn6_slot;
        let mut var_vgefm13g1_dn7: f64 = *var_vgefm13g1_dn7_slot;
        let mut var_vgefm13g1_dn8: f64 = *var_vgefm13g1_dn8_slot;
        let mut var_vgefm13g1_dn9: f64 = *var_vgefm13g1_dn9_slot;

        let (assign30330_e47852, assign30330_e47852_d_n0, assign30330_e47852_d_n1, assign30330_e47852_d_n2, assign30330_e47852_d_n3, assign30330_e47852_d_n4, assign30330_e47852_d_n5, assign30330_e47852_d_n6, assign30330_e47852_d_n7, assign30330_e47852_d_n8, assign30330_e47852_d_n9, assign30330_e47852_d_n12, assign30330_e47852_d_n14, assign30330_e47852_d_n15, assign30330_e47852_d_n16, assign30330_e47852_d_n17, assign30330_e47852_d_n18, assign30330_e47852_d_n19, assign30330_e47852_d_n20, assign30330_e47852_d_n21, assign30330_e47852_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let assign30330_e47850: f64 = (var_vgod - var_ef1);
        (assign30330_e47850, (var_vgod_dn0 - var_ef1_dn0), (var_vgod_dn1 - var_ef1_dn1), (var_vgod_dn2 - var_ef1_dn2), (var_vgod_dn3 - var_ef1_dn3), (var_vgod_dn4 - var_ef1_dn4), (var_vgod_dn5 - var_ef1_dn5), (var_vgod_dn6 - var_ef1_dn6), (var_vgod_dn7 - var_ef1_dn7), (var_vgod_dn8 - var_ef1_dn8), (var_vgod_dn9 - var_ef1_dn9), (var_vgod_dn12 - var_ef1_dn12), (var_vgod_dn14 - var_ef1_dn14), (var_vgod_dn15 - var_ef1_dn15), (var_vgod_dn16 - var_ef1_dn16), (var_vgod_dn17 - var_ef1_dn17), (var_vgod_dn18 - var_ef1_dn18), (var_vgod_dn19 - var_ef1_dn19), (var_vgod_dn20 - var_ef1_dn20), (var_vgod_dn21 - var_ef1_dn21), (var_vgod_dn22 - var_ef1_dn22),)
    } else {
        (var_vgef1, var_vgef1_dn0, var_vgef1_dn1, var_vgef1_dn2, var_vgef1_dn3, var_vgef1_dn4, var_vgef1_dn5, var_vgef1_dn6, var_vgef1_dn7, var_vgef1_dn8, var_vgef1_dn9, var_vgef1_dn12, var_vgef1_dn14, var_vgef1_dn15, var_vgef1_dn16, var_vgef1_dn17, var_vgef1_dn18, var_vgef1_dn19, var_vgef1_dn20, var_vgef1_dn21, var_vgef1_dn22,)
    }
};
        var_vgef1 = assign30330_e47852;
        var_vgef1_dn0 = assign30330_e47852_d_n0;
        var_vgef1_dn1 = assign30330_e47852_d_n1;
        var_vgef1_dn2 = assign30330_e47852_d_n2;
        var_vgef1_dn3 = assign30330_e47852_d_n3;
        var_vgef1_dn4 = assign30330_e47852_d_n4;
        var_vgef1_dn5 = assign30330_e47852_d_n5;
        var_vgef1_dn6 = assign30330_e47852_d_n6;
        var_vgef1_dn7 = assign30330_e47852_d_n7;
        var_vgef1_dn8 = assign30330_e47852_d_n8;
        var_vgef1_dn9 = assign30330_e47852_d_n9;
        var_vgef1_dn12 = assign30330_e47852_d_n12;
        var_vgef1_dn14 = assign30330_e47852_d_n14;
        var_vgef1_dn15 = assign30330_e47852_d_n15;
        var_vgef1_dn16 = assign30330_e47852_d_n16;
        var_vgef1_dn17 = assign30330_e47852_d_n17;
        var_vgef1_dn18 = assign30330_e47852_d_n18;
        var_vgef1_dn19 = assign30330_e47852_d_n19;
        var_vgef1_dn20 = assign30330_e47852_d_n20;
        var_vgef1_dn21 = assign30330_e47852_d_n21;
        var_vgef1_dn22 = assign30330_e47852_d_n22;

        let (assign30340_e47876, assign30340_e47876_d_n0, assign30340_e47876_d_n1, assign30340_e47876_d_n2, assign30340_e47876_d_n3, assign30340_e47876_d_n4, assign30340_e47876_d_n5, assign30340_e47876_d_n6, assign30340_e47876_d_n7, assign30340_e47876_d_n8, assign30340_e47876_d_n9, assign30340_e47876_d_n12, assign30340_e47876_d_n14, assign30340_e47876_d_n15, assign30340_e47876_d_n16, assign30340_e47876_d_n17, assign30340_e47876_d_n18, assign30340_e47876_d_n19, assign30340_e47876_d_n20, assign30340_e47876_d_n21, assign30340_e47876_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let assign30340_e47861: f64 = (0.5 * var_vgef1);
        let assign30340_e47865: f64 = (var_vgef1 * var_vgef1);
        let assign30340_e47868: f64 = (4.0 * 1e-9);
        let assign30340_e47870: f64 = (assign30340_e47868 * 1e-9);
        let assign30340_e47871: f64 = (assign30340_e47865 + assign30340_e47870);
        let assign30340_e47872: f64 = (assign30340_e47871).sqrt();
        let assign30340_e47873: f64 = (0.5 * assign30340_e47872);
        let assign30340_e47874: f64 = (assign30340_e47861 + assign30340_e47873);
        (assign30340_e47874, ((0.5 * var_vgef1_dn0) + (0.5 * (((var_vgef1_dn0 * var_vgef1) + (var_vgef1 * var_vgef1_dn0)) / (2.0 * assign30340_e47872)))), ((0.5 * var_vgef1_dn1) + (0.5 * (((var_vgef1_dn1 * var_vgef1) + (var_vgef1 * var_vgef1_dn1)) / (2.0 * assign30340_e47872)))), ((0.5 * var_vgef1_dn2) + (0.5 * (((var_vgef1_dn2 * var_vgef1) + (var_vgef1 * var_vgef1_dn2)) / (2.0 * assign30340_e47872)))), ((0.5 * var_vgef1_dn3) + (0.5 * (((var_vgef1_dn3 * var_vgef1) + (var_vgef1 * var_vgef1_dn3)) / (2.0 * assign30340_e47872)))), ((0.5 * var_vgef1_dn4) + (0.5 * (((var_vgef1_dn4 * var_vgef1) + (var_vgef1 * var_vgef1_dn4)) / (2.0 * assign30340_e47872)))), ((0.5 * var_vgef1_dn5) + (0.5 * (((var_vgef1_dn5 * var_vgef1) + (var_vgef1 * var_vgef1_dn5)) / (2.0 * assign30340_e47872)))), ((0.5 * var_vgef1_dn6) + (0.5 * (((var_vgef1_dn6 * var_vgef1) + (var_vgef1 * var_vgef1_dn6)) / (2.0 * assign30340_e47872)))), ((0.5 * var_vgef1_dn7) + (0.5 * (((var_vgef1_dn7 * var_vgef1) + (var_vgef1 * var_vgef1_dn7)) / (2.0 * assign30340_e47872)))), ((0.5 * var_vgef1_dn8) + (0.5 * (((var_vgef1_dn8 * var_vgef1) + (var_vgef1 * var_vgef1_dn8)) / (2.0 * assign30340_e47872)))), ((0.5 * var_vgef1_dn9) + (0.5 * (((var_vgef1_dn9 * var_vgef1) + (var_vgef1 * var_vgef1_dn9)) / (2.0 * assign30340_e47872)))), ((0.5 * var_vgef1_dn12) + (0.5 * (((var_vgef1_dn12 * var_vgef1) + (var_vgef1 * var_vgef1_dn12)) / (2.0 * assign30340_e47872)))), ((0.5 * var_vgef1_dn14) + (0.5 * (((var_vgef1_dn14 * var_vgef1) + (var_vgef1 * var_vgef1_dn14)) / (2.0 * assign30340_e47872)))), ((0.5 * var_vgef1_dn15) + (0.5 * (((var_vgef1_dn15 * var_vgef1) + (var_vgef1 * var_vgef1_dn15)) / (2.0 * assign30340_e47872)))), ((0.5 * var_vgef1_dn16) + (0.5 * (((var_vgef1_dn16 * var_vgef1) + (var_vgef1 * var_vgef1_dn16)) / (2.0 * assign30340_e47872)))), ((0.5 * var_vgef1_dn17) + (0.5 * (((var_vgef1_dn17 * var_vgef1) + (var_vgef1 * var_vgef1_dn17)) / (2.0 * assign30340_e47872)))), ((0.5 * var_vgef1_dn18) + (0.5 * (((var_vgef1_dn18 * var_vgef1) + (var_vgef1 * var_vgef1_dn18)) / (2.0 * assign30340_e47872)))), ((0.5 * var_vgef1_dn19) + (0.5 * (((var_vgef1_dn19 * var_vgef1) + (var_vgef1 * var_vgef1_dn19)) / (2.0 * assign30340_e47872)))), ((0.5 * var_vgef1_dn20) + (0.5 * (((var_vgef1_dn20 * var_vgef1) + (var_vgef1 * var_vgef1_dn20)) / (2.0 * assign30340_e47872)))), ((0.5 * var_vgef1_dn21) + (0.5 * (((var_vgef1_dn21 * var_vgef1) + (var_vgef1 * var_vgef1_dn21)) / (2.0 * assign30340_e47872)))), ((0.5 * var_vgef1_dn22) + (0.5 * (((var_vgef1_dn22 * var_vgef1) + (var_vgef1 * var_vgef1_dn22)) / (2.0 * assign30340_e47872)))),)
    } else {
        (var_vgef1, var_vgef1_dn0, var_vgef1_dn1, var_vgef1_dn2, var_vgef1_dn3, var_vgef1_dn4, var_vgef1_dn5, var_vgef1_dn6, var_vgef1_dn7, var_vgef1_dn8, var_vgef1_dn9, var_vgef1_dn12, var_vgef1_dn14, var_vgef1_dn15, var_vgef1_dn16, var_vgef1_dn17, var_vgef1_dn18, var_vgef1_dn19, var_vgef1_dn20, var_vgef1_dn21, var_vgef1_dn22,)
    }
};
        var_vgef1 = assign30340_e47876;
        var_vgef1_dn0 = assign30340_e47876_d_n0;
        var_vgef1_dn1 = assign30340_e47876_d_n1;
        var_vgef1_dn2 = assign30340_e47876_d_n2;
        var_vgef1_dn3 = assign30340_e47876_d_n3;
        var_vgef1_dn4 = assign30340_e47876_d_n4;
        var_vgef1_dn5 = assign30340_e47876_d_n5;
        var_vgef1_dn6 = assign30340_e47876_d_n6;
        var_vgef1_dn7 = assign30340_e47876_d_n7;
        var_vgef1_dn8 = assign30340_e47876_d_n8;
        var_vgef1_dn9 = assign30340_e47876_d_n9;
        var_vgef1_dn12 = assign30340_e47876_d_n12;
        var_vgef1_dn14 = assign30340_e47876_d_n14;
        var_vgef1_dn15 = assign30340_e47876_d_n15;
        var_vgef1_dn16 = assign30340_e47876_d_n16;
        var_vgef1_dn17 = assign30340_e47876_d_n17;
        var_vgef1_dn18 = assign30340_e47876_d_n18;
        var_vgef1_dn19 = assign30340_e47876_d_n19;
        var_vgef1_dn20 = assign30340_e47876_d_n20;
        var_vgef1_dn21 = assign30340_e47876_d_n21;
        var_vgef1_dn22 = assign30340_e47876_d_n22;

        let (assign30350_e47887, assign30350_e47887_d_n0, assign30350_e47887_d_n1, assign30350_e47887_d_n2, assign30350_e47887_d_n3, assign30350_e47887_d_n4, assign30350_e47887_d_n5, assign30350_e47887_d_n6, assign30350_e47887_d_n7, assign30350_e47887_d_n8, assign30350_e47887_d_n9, assign30350_e47887_d_n12, assign30350_e47887_d_n14, assign30350_e47887_d_n15, assign30350_e47887_d_n16, assign30350_e47887_d_n17, assign30350_e47887_d_n18, assign30350_e47887_d_n19, assign30350_e47887_d_n20, assign30350_e47887_d_n21, assign30350_e47887_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let assign30350_e47885: f64 = (var_cch).powf(0.6666666666666666);
        (assign30350_e47885, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn1, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn12, var_t0_dn14, var_t0_dn15, var_t0_dn16, var_t0_dn17, var_t0_dn18, var_t0_dn19, var_t0_dn20, var_t0_dn21, var_t0_dn22,)
    }
};
        var_t0 = assign30350_e47887;
        var_t0_dn0 = assign30350_e47887_d_n0;
        var_t0_dn1 = assign30350_e47887_d_n1;
        var_t0_dn2 = assign30350_e47887_d_n2;
        var_t0_dn3 = assign30350_e47887_d_n3;
        var_t0_dn4 = assign30350_e47887_d_n4;
        var_t0_dn5 = assign30350_e47887_d_n5;
        var_t0_dn6 = assign30350_e47887_d_n6;
        var_t0_dn7 = assign30350_e47887_d_n7;
        var_t0_dn8 = assign30350_e47887_d_n8;
        var_t0_dn9 = assign30350_e47887_d_n9;
        var_t0_dn12 = assign30350_e47887_d_n12;
        var_t0_dn14 = assign30350_e47887_d_n14;
        var_t0_dn15 = assign30350_e47887_d_n15;
        var_t0_dn16 = assign30350_e47887_d_n16;
        var_t0_dn17 = assign30350_e47887_d_n17;
        var_t0_dn18 = assign30350_e47887_d_n18;
        var_t0_dn19 = assign30350_e47887_d_n19;
        var_t0_dn20 = assign30350_e47887_d_n20;
        var_t0_dn21 = assign30350_e47887_d_n21;
        var_t0_dn22 = assign30350_e47887_d_n22;

        let (assign30360_e47898, assign30360_e47898_d_n0, assign30360_e47898_d_n1, assign30360_e47898_d_n2, assign30360_e47898_d_n3, assign30360_e47898_d_n4, assign30360_e47898_d_n5, assign30360_e47898_d_n6, assign30360_e47898_d_n7, assign30360_e47898_d_n8, assign30360_e47898_d_n9, assign30360_e47898_d_n12, assign30360_e47898_d_n14, assign30360_e47898_d_n15, assign30360_e47898_d_n16, assign30360_e47898_d_n17, assign30360_e47898_d_n18, assign30360_e47898_d_n19, assign30360_e47898_d_n20, assign30360_e47898_d_n21, assign30360_e47898_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let assign30360_e47896: f64 = (var_vgef1).powf(0.6666666666666666);
        (assign30360_e47896, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef1).powf(0.6666666666666666 - 1.0) * var_vgef1_dn0)) } } else { (assign30360_e47896 * (0.6666666666666666 * (var_vgef1_dn0 / var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef1).powf(0.6666666666666666 - 1.0) * var_vgef1_dn1)) } } else { (assign30360_e47896 * (0.6666666666666666 * (var_vgef1_dn1 / var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef1).powf(0.6666666666666666 - 1.0) * var_vgef1_dn2)) } } else { (assign30360_e47896 * (0.6666666666666666 * (var_vgef1_dn2 / var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef1).powf(0.6666666666666666 - 1.0) * var_vgef1_dn3)) } } else { (assign30360_e47896 * (0.6666666666666666 * (var_vgef1_dn3 / var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef1).powf(0.6666666666666666 - 1.0) * var_vgef1_dn4)) } } else { (assign30360_e47896 * (0.6666666666666666 * (var_vgef1_dn4 / var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef1).powf(0.6666666666666666 - 1.0) * var_vgef1_dn5)) } } else { (assign30360_e47896 * (0.6666666666666666 * (var_vgef1_dn5 / var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef1).powf(0.6666666666666666 - 1.0) * var_vgef1_dn6)) } } else { (assign30360_e47896 * (0.6666666666666666 * (var_vgef1_dn6 / var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef1).powf(0.6666666666666666 - 1.0) * var_vgef1_dn7)) } } else { (assign30360_e47896 * (0.6666666666666666 * (var_vgef1_dn7 / var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef1).powf(0.6666666666666666 - 1.0) * var_vgef1_dn8)) } } else { (assign30360_e47896 * (0.6666666666666666 * (var_vgef1_dn8 / var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef1).powf(0.6666666666666666 - 1.0) * var_vgef1_dn9)) } } else { (assign30360_e47896 * (0.6666666666666666 * (var_vgef1_dn9 / var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef1).powf(0.6666666666666666 - 1.0) * var_vgef1_dn12)) } } else { (assign30360_e47896 * (0.6666666666666666 * (var_vgef1_dn12 / var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef1).powf(0.6666666666666666 - 1.0) * var_vgef1_dn14)) } } else { (assign30360_e47896 * (0.6666666666666666 * (var_vgef1_dn14 / var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef1).powf(0.6666666666666666 - 1.0) * var_vgef1_dn15)) } } else { (assign30360_e47896 * (0.6666666666666666 * (var_vgef1_dn15 / var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef1).powf(0.6666666666666666 - 1.0) * var_vgef1_dn16)) } } else { (assign30360_e47896 * (0.6666666666666666 * (var_vgef1_dn16 / var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef1).powf(0.6666666666666666 - 1.0) * var_vgef1_dn17)) } } else { (assign30360_e47896 * (0.6666666666666666 * (var_vgef1_dn17 / var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef1).powf(0.6666666666666666 - 1.0) * var_vgef1_dn18)) } } else { (assign30360_e47896 * (0.6666666666666666 * (var_vgef1_dn18 / var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef1).powf(0.6666666666666666 - 1.0) * var_vgef1_dn19)) } } else { (assign30360_e47896 * (0.6666666666666666 * (var_vgef1_dn19 / var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef1).powf(0.6666666666666666 - 1.0) * var_vgef1_dn20)) } } else { (assign30360_e47896 * (0.6666666666666666 * (var_vgef1_dn20 / var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef1).powf(0.6666666666666666 - 1.0) * var_vgef1_dn21)) } } else { (assign30360_e47896 * (0.6666666666666666 * (var_vgef1_dn21 / var_vgef1))) }, if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef1).powf(0.6666666666666666 - 1.0) * var_vgef1_dn22)) } } else { (assign30360_e47896 * (0.6666666666666666 * (var_vgef1_dn22 / var_vgef1))) },)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn1, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn12, var_t1_dn14, var_t1_dn15, var_t1_dn16, var_t1_dn17, var_t1_dn18, var_t1_dn19, var_t1_dn20, var_t1_dn21, var_t1_dn22,)
    }
};
        var_t1 = assign30360_e47898;
        var_t1_dn0 = assign30360_e47898_d_n0;
        var_t1_dn1 = assign30360_e47898_d_n1;
        var_t1_dn2 = assign30360_e47898_d_n2;
        var_t1_dn3 = assign30360_e47898_d_n3;
        var_t1_dn4 = assign30360_e47898_d_n4;
        var_t1_dn5 = assign30360_e47898_d_n5;
        var_t1_dn6 = assign30360_e47898_d_n6;
        var_t1_dn7 = assign30360_e47898_d_n7;
        var_t1_dn8 = assign30360_e47898_d_n8;
        var_t1_dn9 = assign30360_e47898_d_n9;
        var_t1_dn12 = assign30360_e47898_d_n12;
        var_t1_dn14 = assign30360_e47898_d_n14;
        var_t1_dn15 = assign30360_e47898_d_n15;
        var_t1_dn16 = assign30360_e47898_d_n16;
        var_t1_dn17 = assign30360_e47898_d_n17;
        var_t1_dn18 = assign30360_e47898_d_n18;
        var_t1_dn19 = assign30360_e47898_d_n19;
        var_t1_dn20 = assign30360_e47898_d_n20;
        var_t1_dn21 = assign30360_e47898_d_n21;
        var_t1_dn22 = assign30360_e47898_d_n22;

        let (assign30370_e47910, assign30370_e47910_d_n0, assign30370_e47910_d_n1, assign30370_e47910_d_n2, assign30370_e47910_d_n3, assign30370_e47910_d_n4, assign30370_e47910_d_n5, assign30370_e47910_d_n6, assign30370_e47910_d_n7, assign30370_e47910_d_n8, assign30370_e47910_d_n9, assign30370_e47910_d_n12, assign30370_e47910_d_n14, assign30370_e47910_d_n15, assign30370_e47910_d_n16, assign30370_e47910_d_n17, assign30370_e47910_d_n18, assign30370_e47910_d_n19, assign30370_e47910_d_n20, assign30370_e47910_d_n21, assign30370_e47910_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let assign30370_e47907: f64 = (-0.3333333333333333);
        let assign30370_e47908: f64 = (var_vgef1).powf(assign30370_e47907);
        (assign30370_e47908, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((var_vgef1).powf(assign30370_e47907 - 1.0) * var_vgef1_dn0)) } } else { (assign30370_e47908 * (assign30370_e47907 * (var_vgef1_dn0 / var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((var_vgef1).powf(assign30370_e47907 - 1.0) * var_vgef1_dn1)) } } else { (assign30370_e47908 * (assign30370_e47907 * (var_vgef1_dn1 / var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((var_vgef1).powf(assign30370_e47907 - 1.0) * var_vgef1_dn2)) } } else { (assign30370_e47908 * (assign30370_e47907 * (var_vgef1_dn2 / var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((var_vgef1).powf(assign30370_e47907 - 1.0) * var_vgef1_dn3)) } } else { (assign30370_e47908 * (assign30370_e47907 * (var_vgef1_dn3 / var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((var_vgef1).powf(assign30370_e47907 - 1.0) * var_vgef1_dn4)) } } else { (assign30370_e47908 * (assign30370_e47907 * (var_vgef1_dn4 / var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((var_vgef1).powf(assign30370_e47907 - 1.0) * var_vgef1_dn5)) } } else { (assign30370_e47908 * (assign30370_e47907 * (var_vgef1_dn5 / var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((var_vgef1).powf(assign30370_e47907 - 1.0) * var_vgef1_dn6)) } } else { (assign30370_e47908 * (assign30370_e47907 * (var_vgef1_dn6 / var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((var_vgef1).powf(assign30370_e47907 - 1.0) * var_vgef1_dn7)) } } else { (assign30370_e47908 * (assign30370_e47907 * (var_vgef1_dn7 / var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((var_vgef1).powf(assign30370_e47907 - 1.0) * var_vgef1_dn8)) } } else { (assign30370_e47908 * (assign30370_e47907 * (var_vgef1_dn8 / var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((var_vgef1).powf(assign30370_e47907 - 1.0) * var_vgef1_dn9)) } } else { (assign30370_e47908 * (assign30370_e47907 * (var_vgef1_dn9 / var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((var_vgef1).powf(assign30370_e47907 - 1.0) * var_vgef1_dn12)) } } else { (assign30370_e47908 * (assign30370_e47907 * (var_vgef1_dn12 / var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((var_vgef1).powf(assign30370_e47907 - 1.0) * var_vgef1_dn14)) } } else { (assign30370_e47908 * (assign30370_e47907 * (var_vgef1_dn14 / var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((var_vgef1).powf(assign30370_e47907 - 1.0) * var_vgef1_dn15)) } } else { (assign30370_e47908 * (assign30370_e47907 * (var_vgef1_dn15 / var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((var_vgef1).powf(assign30370_e47907 - 1.0) * var_vgef1_dn16)) } } else { (assign30370_e47908 * (assign30370_e47907 * (var_vgef1_dn16 / var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((var_vgef1).powf(assign30370_e47907 - 1.0) * var_vgef1_dn17)) } } else { (assign30370_e47908 * (assign30370_e47907 * (var_vgef1_dn17 / var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((var_vgef1).powf(assign30370_e47907 - 1.0) * var_vgef1_dn18)) } } else { (assign30370_e47908 * (assign30370_e47907 * (var_vgef1_dn18 / var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((var_vgef1).powf(assign30370_e47907 - 1.0) * var_vgef1_dn19)) } } else { (assign30370_e47908 * (assign30370_e47907 * (var_vgef1_dn19 / var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((var_vgef1).powf(assign30370_e47907 - 1.0) * var_vgef1_dn20)) } } else { (assign30370_e47908 * (assign30370_e47907 * (var_vgef1_dn20 / var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((var_vgef1).powf(assign30370_e47907 - 1.0) * var_vgef1_dn21)) } } else { (assign30370_e47908 * (assign30370_e47907 * (var_vgef1_dn21 / var_vgef1))) }, if 0.0 == 0.0 && ((assign30370_e47907) as f64).is_finite() && ((assign30370_e47907) as f64).fract() == 0.0 { if assign30370_e47907 == 0.0 { 0.0 } else { (assign30370_e47907 * ((var_vgef1).powf(assign30370_e47907 - 1.0) * var_vgef1_dn22)) } } else { (assign30370_e47908 * (assign30370_e47907 * (var_vgef1_dn22 / var_vgef1))) },)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn1, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn12, var_t2_dn14, var_t2_dn15, var_t2_dn16, var_t2_dn17, var_t2_dn18, var_t2_dn19, var_t2_dn20, var_t2_dn21, var_t2_dn22,)
    }
};
        var_t2 = assign30370_e47910;
        var_t2_dn0 = assign30370_e47910_d_n0;
        var_t2_dn1 = assign30370_e47910_d_n1;
        var_t2_dn2 = assign30370_e47910_d_n2;
        var_t2_dn3 = assign30370_e47910_d_n3;
        var_t2_dn4 = assign30370_e47910_d_n4;
        var_t2_dn5 = assign30370_e47910_d_n5;
        var_t2_dn6 = assign30370_e47910_d_n6;
        var_t2_dn7 = assign30370_e47910_d_n7;
        var_t2_dn8 = assign30370_e47910_d_n8;
        var_t2_dn9 = assign30370_e47910_d_n9;
        var_t2_dn12 = assign30370_e47910_d_n12;
        var_t2_dn14 = assign30370_e47910_d_n14;
        var_t2_dn15 = assign30370_e47910_d_n15;
        var_t2_dn16 = assign30370_e47910_d_n16;
        var_t2_dn17 = assign30370_e47910_d_n17;
        var_t2_dn18 = assign30370_e47910_d_n18;
        var_t2_dn19 = assign30370_e47910_d_n19;
        var_t2_dn20 = assign30370_e47910_d_n20;
        var_t2_dn21 = assign30370_e47910_d_n21;
        var_t2_dn22 = assign30370_e47910_d_n22;

        let (assign30380_e47923, assign30380_e47923_d_n0, assign30380_e47923_d_n1, assign30380_e47923_d_n2, assign30380_e47923_d_n3, assign30380_e47923_d_n4, assign30380_e47923_d_n5, assign30380_e47923_d_n6, assign30380_e47923_d_n7, assign30380_e47923_d_n8, assign30380_e47923_d_n9, assign30380_e47923_d_n12, assign30380_e47923_d_n14, assign30380_e47923_d_n15, assign30380_e47923_d_n16, assign30380_e47923_d_n17, assign30380_e47923_d_n18, assign30380_e47923_d_n19, assign30380_e47923_d_n20, assign30380_e47923_d_n21, assign30380_e47923_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let assign30380_e47919: f64 = (p.p208 * var_t0);
        let assign30380_e47921: f64 = (assign30380_e47919 * var_t1);
        (assign30380_e47921, (((p.p208 * var_t0_dn0) * var_t1) + (assign30380_e47919 * var_t1_dn0)), (((p.p208 * var_t0_dn1) * var_t1) + (assign30380_e47919 * var_t1_dn1)), (((p.p208 * var_t0_dn2) * var_t1) + (assign30380_e47919 * var_t1_dn2)), (((p.p208 * var_t0_dn3) * var_t1) + (assign30380_e47919 * var_t1_dn3)), (((p.p208 * var_t0_dn4) * var_t1) + (assign30380_e47919 * var_t1_dn4)), (((p.p208 * var_t0_dn5) * var_t1) + (assign30380_e47919 * var_t1_dn5)), (((p.p208 * var_t0_dn6) * var_t1) + (assign30380_e47919 * var_t1_dn6)), (((p.p208 * var_t0_dn7) * var_t1) + (assign30380_e47919 * var_t1_dn7)), (((p.p208 * var_t0_dn8) * var_t1) + (assign30380_e47919 * var_t1_dn8)), (((p.p208 * var_t0_dn9) * var_t1) + (assign30380_e47919 * var_t1_dn9)), (((p.p208 * var_t0_dn12) * var_t1) + (assign30380_e47919 * var_t1_dn12)), (((p.p208 * var_t0_dn14) * var_t1) + (assign30380_e47919 * var_t1_dn14)), (((p.p208 * var_t0_dn15) * var_t1) + (assign30380_e47919 * var_t1_dn15)), (((p.p208 * var_t0_dn16) * var_t1) + (assign30380_e47919 * var_t1_dn16)), (((p.p208 * var_t0_dn17) * var_t1) + (assign30380_e47919 * var_t1_dn17)), (((p.p208 * var_t0_dn18) * var_t1) + (assign30380_e47919 * var_t1_dn18)), (((p.p208 * var_t0_dn19) * var_t1) + (assign30380_e47919 * var_t1_dn19)), (((p.p208 * var_t0_dn20) * var_t1) + (assign30380_e47919 * var_t1_dn20)), (((p.p208 * var_t0_dn21) * var_t1) + (assign30380_e47919 * var_t1_dn21)), (((p.p208 * var_t0_dn22) * var_t1) + (assign30380_e47919 * var_t1_dn22)),)
    } else {
        (var_vgef23g0, var_vgef23g0_dn0, var_vgef23g0_dn1, var_vgef23g0_dn2, var_vgef23g0_dn3, var_vgef23g0_dn4, var_vgef23g0_dn5, var_vgef23g0_dn6, var_vgef23g0_dn7, var_vgef23g0_dn8, var_vgef23g0_dn9, var_vgef23g0_dn12, var_vgef23g0_dn14, var_vgef23g0_dn15, var_vgef23g0_dn16, var_vgef23g0_dn17, var_vgef23g0_dn18, var_vgef23g0_dn19, var_vgef23g0_dn20, var_vgef23g0_dn21, var_vgef23g0_dn22,)
    }
};
        var_vgef23g0 = assign30380_e47923;
        var_vgef23g0_dn0 = assign30380_e47923_d_n0;
        var_vgef23g0_dn1 = assign30380_e47923_d_n1;
        var_vgef23g0_dn2 = assign30380_e47923_d_n2;
        var_vgef23g0_dn3 = assign30380_e47923_d_n3;
        var_vgef23g0_dn4 = assign30380_e47923_d_n4;
        var_vgef23g0_dn5 = assign30380_e47923_d_n5;
        var_vgef23g0_dn6 = assign30380_e47923_d_n6;
        var_vgef23g0_dn7 = assign30380_e47923_d_n7;
        var_vgef23g0_dn8 = assign30380_e47923_d_n8;
        var_vgef23g0_dn9 = assign30380_e47923_d_n9;
        var_vgef23g0_dn12 = assign30380_e47923_d_n12;
        var_vgef23g0_dn14 = assign30380_e47923_d_n14;
        var_vgef23g0_dn15 = assign30380_e47923_d_n15;
        var_vgef23g0_dn16 = assign30380_e47923_d_n16;
        var_vgef23g0_dn17 = assign30380_e47923_d_n17;
        var_vgef23g0_dn18 = assign30380_e47923_d_n18;
        var_vgef23g0_dn19 = assign30380_e47923_d_n19;
        var_vgef23g0_dn20 = assign30380_e47923_d_n20;
        var_vgef23g0_dn21 = assign30380_e47923_d_n21;
        var_vgef23g0_dn22 = assign30380_e47923_d_n22;

        let (assign30390_e47936, assign30390_e47936_d_n0, assign30390_e47936_d_n1, assign30390_e47936_d_n2, assign30390_e47936_d_n3, assign30390_e47936_d_n4, assign30390_e47936_d_n5, assign30390_e47936_d_n6, assign30390_e47936_d_n7, assign30390_e47936_d_n8, assign30390_e47936_d_n9, assign30390_e47936_d_n12, assign30390_e47936_d_n14, assign30390_e47936_d_n15, assign30390_e47936_d_n16, assign30390_e47936_d_n17, assign30390_e47936_d_n18, assign30390_e47936_d_n19, assign30390_e47936_d_n20, assign30390_e47936_d_n21, assign30390_e47936_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let assign30390_e47932: f64 = (p.p209 * var_t0);
        let assign30390_e47934: f64 = (assign30390_e47932 * var_t1);
        (assign30390_e47934, (((p.p209 * var_t0_dn0) * var_t1) + (assign30390_e47932 * var_t1_dn0)), (((p.p209 * var_t0_dn1) * var_t1) + (assign30390_e47932 * var_t1_dn1)), (((p.p209 * var_t0_dn2) * var_t1) + (assign30390_e47932 * var_t1_dn2)), (((p.p209 * var_t0_dn3) * var_t1) + (assign30390_e47932 * var_t1_dn3)), (((p.p209 * var_t0_dn4) * var_t1) + (assign30390_e47932 * var_t1_dn4)), (((p.p209 * var_t0_dn5) * var_t1) + (assign30390_e47932 * var_t1_dn5)), (((p.p209 * var_t0_dn6) * var_t1) + (assign30390_e47932 * var_t1_dn6)), (((p.p209 * var_t0_dn7) * var_t1) + (assign30390_e47932 * var_t1_dn7)), (((p.p209 * var_t0_dn8) * var_t1) + (assign30390_e47932 * var_t1_dn8)), (((p.p209 * var_t0_dn9) * var_t1) + (assign30390_e47932 * var_t1_dn9)), (((p.p209 * var_t0_dn12) * var_t1) + (assign30390_e47932 * var_t1_dn12)), (((p.p209 * var_t0_dn14) * var_t1) + (assign30390_e47932 * var_t1_dn14)), (((p.p209 * var_t0_dn15) * var_t1) + (assign30390_e47932 * var_t1_dn15)), (((p.p209 * var_t0_dn16) * var_t1) + (assign30390_e47932 * var_t1_dn16)), (((p.p209 * var_t0_dn17) * var_t1) + (assign30390_e47932 * var_t1_dn17)), (((p.p209 * var_t0_dn18) * var_t1) + (assign30390_e47932 * var_t1_dn18)), (((p.p209 * var_t0_dn19) * var_t1) + (assign30390_e47932 * var_t1_dn19)), (((p.p209 * var_t0_dn20) * var_t1) + (assign30390_e47932 * var_t1_dn20)), (((p.p209 * var_t0_dn21) * var_t1) + (assign30390_e47932 * var_t1_dn21)), (((p.p209 * var_t0_dn22) * var_t1) + (assign30390_e47932 * var_t1_dn22)),)
    } else {
        (var_vgef23g1, var_vgef23g1_dn0, var_vgef23g1_dn1, var_vgef23g1_dn2, var_vgef23g1_dn3, var_vgef23g1_dn4, var_vgef23g1_dn5, var_vgef23g1_dn6, var_vgef23g1_dn7, var_vgef23g1_dn8, var_vgef23g1_dn9, var_vgef23g1_dn12, var_vgef23g1_dn14, var_vgef23g1_dn15, var_vgef23g1_dn16, var_vgef23g1_dn17, var_vgef23g1_dn18, var_vgef23g1_dn19, var_vgef23g1_dn20, var_vgef23g1_dn21, var_vgef23g1_dn22,)
    }
};
        var_vgef23g1 = assign30390_e47936;
        var_vgef23g1_dn0 = assign30390_e47936_d_n0;
        var_vgef23g1_dn1 = assign30390_e47936_d_n1;
        var_vgef23g1_dn2 = assign30390_e47936_d_n2;
        var_vgef23g1_dn3 = assign30390_e47936_d_n3;
        var_vgef23g1_dn4 = assign30390_e47936_d_n4;
        var_vgef23g1_dn5 = assign30390_e47936_d_n5;
        var_vgef23g1_dn6 = assign30390_e47936_d_n6;
        var_vgef23g1_dn7 = assign30390_e47936_d_n7;
        var_vgef23g1_dn8 = assign30390_e47936_d_n8;
        var_vgef23g1_dn9 = assign30390_e47936_d_n9;
        var_vgef23g1_dn12 = assign30390_e47936_d_n12;
        var_vgef23g1_dn14 = assign30390_e47936_d_n14;
        var_vgef23g1_dn15 = assign30390_e47936_d_n15;
        var_vgef23g1_dn16 = assign30390_e47936_d_n16;
        var_vgef23g1_dn17 = assign30390_e47936_d_n17;
        var_vgef23g1_dn18 = assign30390_e47936_d_n18;
        var_vgef23g1_dn19 = assign30390_e47936_d_n19;
        var_vgef23g1_dn20 = assign30390_e47936_d_n20;
        var_vgef23g1_dn21 = assign30390_e47936_d_n21;
        var_vgef23g1_dn22 = assign30390_e47936_d_n22;

        let (assign30400_e47951, assign30400_e47951_d_n0, assign30400_e47951_d_n1, assign30400_e47951_d_n2, assign30400_e47951_d_n3, assign30400_e47951_d_n4, assign30400_e47951_d_n5, assign30400_e47951_d_n6, assign30400_e47951_d_n7, assign30400_e47951_d_n8, assign30400_e47951_d_n9, assign30400_e47951_d_n12, assign30400_e47951_d_n14, assign30400_e47951_d_n15, assign30400_e47951_d_n16, assign30400_e47951_d_n17, assign30400_e47951_d_n18, assign30400_e47951_d_n19, assign30400_e47951_d_n20, assign30400_e47951_d_n21, assign30400_e47951_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / var_vtv;
        let assign30400_e47945: f64 = (var_ef1 * __rspice_inv_cse_0);
        let assign30400_e47948: f64 = (var_vgef23g0 * __rspice_inv_cse_0);
        let assign30400_e47949: f64 = (assign30400_e47945 - assign30400_e47948);
        (assign30400_e47949, ((var_ef1_dn0 / var_vtv) - (var_vgef23g0_dn0 / var_vtv)), ((var_ef1_dn1 / var_vtv) - (var_vgef23g0_dn1 / var_vtv)), ((var_ef1_dn2 / var_vtv) - (var_vgef23g0_dn2 / var_vtv)), ((var_ef1_dn3 / var_vtv) - (var_vgef23g0_dn3 / var_vtv)), ((((var_ef1_dn4 * var_vtv) - (var_ef1 * var_vtv_dn4)) / (var_vtv * var_vtv)) - (((var_vgef23g0_dn4 * var_vtv) - (var_vgef23g0 * var_vtv_dn4)) / (var_vtv * var_vtv))), ((var_ef1_dn5 / var_vtv) - (var_vgef23g0_dn5 / var_vtv)), ((((var_ef1_dn6 * var_vtv) - (var_ef1 * var_vtv_dn6)) / (var_vtv * var_vtv)) - (((var_vgef23g0_dn6 * var_vtv) - (var_vgef23g0 * var_vtv_dn6)) / (var_vtv * var_vtv))), ((((var_ef1_dn7 * var_vtv) - (var_ef1 * var_vtv_dn7)) / (var_vtv * var_vtv)) - (((var_vgef23g0_dn7 * var_vtv) - (var_vgef23g0 * var_vtv_dn7)) / (var_vtv * var_vtv))), ((((var_ef1_dn8 * var_vtv) - (var_ef1 * var_vtv_dn8)) / (var_vtv * var_vtv)) - (((var_vgef23g0_dn8 * var_vtv) - (var_vgef23g0 * var_vtv_dn8)) / (var_vtv * var_vtv))), ((var_ef1_dn9 / var_vtv) - (var_vgef23g0_dn9 / var_vtv)), ((var_ef1_dn12 / var_vtv) - (var_vgef23g0_dn12 / var_vtv)), ((var_ef1_dn14 / var_vtv) - (var_vgef23g0_dn14 / var_vtv)), ((((var_ef1_dn15 * var_vtv) - (var_ef1 * var_vtv_dn15)) / (var_vtv * var_vtv)) - (((var_vgef23g0_dn15 * var_vtv) - (var_vgef23g0 * var_vtv_dn15)) / (var_vtv * var_vtv))), ((((var_ef1_dn16 * var_vtv) - (var_ef1 * var_vtv_dn16)) / (var_vtv * var_vtv)) - (((var_vgef23g0_dn16 * var_vtv) - (var_vgef23g0 * var_vtv_dn16)) / (var_vtv * var_vtv))), ((((var_ef1_dn17 * var_vtv) - (var_ef1 * var_vtv_dn17)) / (var_vtv * var_vtv)) - (((var_vgef23g0_dn17 * var_vtv) - (var_vgef23g0 * var_vtv_dn17)) / (var_vtv * var_vtv))), ((((var_ef1_dn18 * var_vtv) - (var_ef1 * var_vtv_dn18)) / (var_vtv * var_vtv)) - (((var_vgef23g0_dn18 * var_vtv) - (var_vgef23g0 * var_vtv_dn18)) / (var_vtv * var_vtv))), ((((var_ef1_dn19 * var_vtv) - (var_ef1 * var_vtv_dn19)) / (var_vtv * var_vtv)) - (((var_vgef23g0_dn19 * var_vtv) - (var_vgef23g0 * var_vtv_dn19)) / (var_vtv * var_vtv))), ((((var_ef1_dn20 * var_vtv) - (var_ef1 * var_vtv_dn20)) / (var_vtv * var_vtv)) - (((var_vgef23g0_dn20 * var_vtv) - (var_vgef23g0 * var_vtv_dn20)) / (var_vtv * var_vtv))), ((((var_ef1_dn21 * var_vtv) - (var_ef1 * var_vtv_dn21)) / (var_vtv * var_vtv)) - (((var_vgef23g0_dn21 * var_vtv) - (var_vgef23g0 * var_vtv_dn21)) / (var_vtv * var_vtv))), ((((var_ef1_dn22 * var_vtv) - (var_ef1 * var_vtv_dn22)) / (var_vtv * var_vtv)) - (((var_vgef23g0_dn22 * var_vtv) - (var_vgef23g0 * var_vtv_dn22)) / (var_vtv * var_vtv))),)
    } else {
        (var_tg0, var_tg0_dn0, var_tg0_dn1, var_tg0_dn2, var_tg0_dn3, var_tg0_dn4, var_tg0_dn5, var_tg0_dn6, var_tg0_dn7, var_tg0_dn8, var_tg0_dn9, var_tg0_dn12, var_tg0_dn14, var_tg0_dn15, var_tg0_dn16, var_tg0_dn17, var_tg0_dn18, var_tg0_dn19, var_tg0_dn20, var_tg0_dn21, var_tg0_dn22,)
    }
};
        var_tg0 = assign30400_e47951;
        var_tg0_dn0 = assign30400_e47951_d_n0;
        var_tg0_dn1 = assign30400_e47951_d_n1;
        var_tg0_dn2 = assign30400_e47951_d_n2;
        var_tg0_dn3 = assign30400_e47951_d_n3;
        var_tg0_dn4 = assign30400_e47951_d_n4;
        var_tg0_dn5 = assign30400_e47951_d_n5;
        var_tg0_dn6 = assign30400_e47951_d_n6;
        var_tg0_dn7 = assign30400_e47951_d_n7;
        var_tg0_dn8 = assign30400_e47951_d_n8;
        var_tg0_dn9 = assign30400_e47951_d_n9;
        var_tg0_dn12 = assign30400_e47951_d_n12;
        var_tg0_dn14 = assign30400_e47951_d_n14;
        var_tg0_dn15 = assign30400_e47951_d_n15;
        var_tg0_dn16 = assign30400_e47951_d_n16;
        var_tg0_dn17 = assign30400_e47951_d_n17;
        var_tg0_dn18 = assign30400_e47951_d_n18;
        var_tg0_dn19 = assign30400_e47951_d_n19;
        var_tg0_dn20 = assign30400_e47951_d_n20;
        var_tg0_dn21 = assign30400_e47951_d_n21;
        var_tg0_dn22 = assign30400_e47951_d_n22;

        let (assign30410_e47966, assign30410_e47966_d_n0, assign30410_e47966_d_n1, assign30410_e47966_d_n2, assign30410_e47966_d_n3, assign30410_e47966_d_n4, assign30410_e47966_d_n5, assign30410_e47966_d_n6, assign30410_e47966_d_n7, assign30410_e47966_d_n8, assign30410_e47966_d_n9, assign30410_e47966_d_n12, assign30410_e47966_d_n14, assign30410_e47966_d_n15, assign30410_e47966_d_n16, assign30410_e47966_d_n17, assign30410_e47966_d_n18, assign30410_e47966_d_n19, assign30410_e47966_d_n20, assign30410_e47966_d_n21, assign30410_e47966_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let __rspice_inv_cse_1: f64 = 1.0 / var_vtv;
        let assign30410_e47960: f64 = (var_ef1 * __rspice_inv_cse_1);
        let assign30410_e47963: f64 = (var_vgef23g1 * __rspice_inv_cse_1);
        let assign30410_e47964: f64 = (assign30410_e47960 - assign30410_e47963);
        (assign30410_e47964, ((var_ef1_dn0 / var_vtv) - (var_vgef23g1_dn0 / var_vtv)), ((var_ef1_dn1 / var_vtv) - (var_vgef23g1_dn1 / var_vtv)), ((var_ef1_dn2 / var_vtv) - (var_vgef23g1_dn2 / var_vtv)), ((var_ef1_dn3 / var_vtv) - (var_vgef23g1_dn3 / var_vtv)), ((((var_ef1_dn4 * var_vtv) - (var_ef1 * var_vtv_dn4)) / (var_vtv * var_vtv)) - (((var_vgef23g1_dn4 * var_vtv) - (var_vgef23g1 * var_vtv_dn4)) / (var_vtv * var_vtv))), ((var_ef1_dn5 / var_vtv) - (var_vgef23g1_dn5 / var_vtv)), ((((var_ef1_dn6 * var_vtv) - (var_ef1 * var_vtv_dn6)) / (var_vtv * var_vtv)) - (((var_vgef23g1_dn6 * var_vtv) - (var_vgef23g1 * var_vtv_dn6)) / (var_vtv * var_vtv))), ((((var_ef1_dn7 * var_vtv) - (var_ef1 * var_vtv_dn7)) / (var_vtv * var_vtv)) - (((var_vgef23g1_dn7 * var_vtv) - (var_vgef23g1 * var_vtv_dn7)) / (var_vtv * var_vtv))), ((((var_ef1_dn8 * var_vtv) - (var_ef1 * var_vtv_dn8)) / (var_vtv * var_vtv)) - (((var_vgef23g1_dn8 * var_vtv) - (var_vgef23g1 * var_vtv_dn8)) / (var_vtv * var_vtv))), ((var_ef1_dn9 / var_vtv) - (var_vgef23g1_dn9 / var_vtv)), ((var_ef1_dn12 / var_vtv) - (var_vgef23g1_dn12 / var_vtv)), ((var_ef1_dn14 / var_vtv) - (var_vgef23g1_dn14 / var_vtv)), ((((var_ef1_dn15 * var_vtv) - (var_ef1 * var_vtv_dn15)) / (var_vtv * var_vtv)) - (((var_vgef23g1_dn15 * var_vtv) - (var_vgef23g1 * var_vtv_dn15)) / (var_vtv * var_vtv))), ((((var_ef1_dn16 * var_vtv) - (var_ef1 * var_vtv_dn16)) / (var_vtv * var_vtv)) - (((var_vgef23g1_dn16 * var_vtv) - (var_vgef23g1 * var_vtv_dn16)) / (var_vtv * var_vtv))), ((((var_ef1_dn17 * var_vtv) - (var_ef1 * var_vtv_dn17)) / (var_vtv * var_vtv)) - (((var_vgef23g1_dn17 * var_vtv) - (var_vgef23g1 * var_vtv_dn17)) / (var_vtv * var_vtv))), ((((var_ef1_dn18 * var_vtv) - (var_ef1 * var_vtv_dn18)) / (var_vtv * var_vtv)) - (((var_vgef23g1_dn18 * var_vtv) - (var_vgef23g1 * var_vtv_dn18)) / (var_vtv * var_vtv))), ((((var_ef1_dn19 * var_vtv) - (var_ef1 * var_vtv_dn19)) / (var_vtv * var_vtv)) - (((var_vgef23g1_dn19 * var_vtv) - (var_vgef23g1 * var_vtv_dn19)) / (var_vtv * var_vtv))), ((((var_ef1_dn20 * var_vtv) - (var_ef1 * var_vtv_dn20)) / (var_vtv * var_vtv)) - (((var_vgef23g1_dn20 * var_vtv) - (var_vgef23g1 * var_vtv_dn20)) / (var_vtv * var_vtv))), ((((var_ef1_dn21 * var_vtv) - (var_ef1 * var_vtv_dn21)) / (var_vtv * var_vtv)) - (((var_vgef23g1_dn21 * var_vtv) - (var_vgef23g1 * var_vtv_dn21)) / (var_vtv * var_vtv))), ((((var_ef1_dn22 * var_vtv) - (var_ef1 * var_vtv_dn22)) / (var_vtv * var_vtv)) - (((var_vgef23g1_dn22 * var_vtv) - (var_vgef23g1 * var_vtv_dn22)) / (var_vtv * var_vtv))),)
    } else {
        (var_tg1, var_tg1_dn0, var_tg1_dn1, var_tg1_dn2, var_tg1_dn3, var_tg1_dn4, var_tg1_dn5, var_tg1_dn6, var_tg1_dn7, var_tg1_dn8, var_tg1_dn9, var_tg1_dn12, var_tg1_dn14, var_tg1_dn15, var_tg1_dn16, var_tg1_dn17, var_tg1_dn18, var_tg1_dn19, var_tg1_dn20, var_tg1_dn21, var_tg1_dn22,)
    }
};
        var_tg1 = assign30410_e47966;
        var_tg1_dn0 = assign30410_e47966_d_n0;
        var_tg1_dn1 = assign30410_e47966_d_n1;
        var_tg1_dn2 = assign30410_e47966_d_n2;
        var_tg1_dn3 = assign30410_e47966_d_n3;
        var_tg1_dn4 = assign30410_e47966_d_n4;
        var_tg1_dn5 = assign30410_e47966_d_n5;
        var_tg1_dn6 = assign30410_e47966_d_n6;
        var_tg1_dn7 = assign30410_e47966_d_n7;
        var_tg1_dn8 = assign30410_e47966_d_n8;
        var_tg1_dn9 = assign30410_e47966_d_n9;
        var_tg1_dn12 = assign30410_e47966_d_n12;
        var_tg1_dn14 = assign30410_e47966_d_n14;
        var_tg1_dn15 = assign30410_e47966_d_n15;
        var_tg1_dn16 = assign30410_e47966_d_n16;
        var_tg1_dn17 = assign30410_e47966_d_n17;
        var_tg1_dn18 = assign30410_e47966_d_n18;
        var_tg1_dn19 = assign30410_e47966_d_n19;
        var_tg1_dn20 = assign30410_e47966_d_n20;
        var_tg1_dn21 = assign30410_e47966_d_n21;
        var_tg1_dn22 = assign30410_e47966_d_n22;

        let (assign30420_e48053, assign30420_e48053_d_n0, assign30420_e48053_d_n1, assign30420_e48053_d_n2, assign30420_e48053_d_n3, assign30420_e48053_d_n4, assign30420_e48053_d_n5, assign30420_e48053_d_n6, assign30420_e48053_d_n7, assign30420_e48053_d_n8, assign30420_e48053_d_n9, assign30420_e48053_d_n12, assign30420_e48053_d_n14, assign30420_e48053_d_n15, assign30420_e48053_d_n16, assign30420_e48053_d_n17, assign30420_e48053_d_n18, assign30420_e48053_d_n19, assign30420_e48053_d_n20, assign30420_e48053_d_n21, assign30420_e48053_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let assign30420_e47975: f64 = (var_cch * var_vgef1);
        let assign30420_e47978: f64 = (3.24e17 * var_vtv);
        let assign30420_e47985: f64 = (-37.0);
        let (assign30420_e48011, assign30420_e48011_d_n0, assign30420_e48011_d_n1, assign30420_e48011_d_n2, assign30420_e48011_d_n3, assign30420_e48011_d_n4, assign30420_e48011_d_n5, assign30420_e48011_d_n6, assign30420_e48011_d_n7, assign30420_e48011_d_n8, assign30420_e48011_d_n9, assign30420_e48011_d_n12, assign30420_e48011_d_n14, assign30420_e48011_d_n15, assign30420_e48011_d_n16, assign30420_e48011_d_n17, assign30420_e48011_d_n18, assign30420_e48011_d_n19, assign30420_e48011_d_n20, assign30420_e48011_d_n21, assign30420_e48011_d_n22,) = {
            if ((!(var_tg0 >= 37.0)) && (!(var_tg0 <= assign30420_e47985))) {
                let assign30420_e47990: f64 = (var_tg0).exp();
                let assign30420_e47992: f64 = (assign30420_e47990 + 1.0);
                let assign30420_e47993: f64 = (assign30420_e47992).ln();
                (assign30420_e47993, ((assign30420_e47990 * var_tg0_dn0) / assign30420_e47992), ((assign30420_e47990 * var_tg0_dn1) / assign30420_e47992), ((assign30420_e47990 * var_tg0_dn2) / assign30420_e47992), ((assign30420_e47990 * var_tg0_dn3) / assign30420_e47992), ((assign30420_e47990 * var_tg0_dn4) / assign30420_e47992), ((assign30420_e47990 * var_tg0_dn5) / assign30420_e47992), ((assign30420_e47990 * var_tg0_dn6) / assign30420_e47992), ((assign30420_e47990 * var_tg0_dn7) / assign30420_e47992), ((assign30420_e47990 * var_tg0_dn8) / assign30420_e47992), ((assign30420_e47990 * var_tg0_dn9) / assign30420_e47992), ((assign30420_e47990 * var_tg0_dn12) / assign30420_e47992), ((assign30420_e47990 * var_tg0_dn14) / assign30420_e47992), ((assign30420_e47990 * var_tg0_dn15) / assign30420_e47992), ((assign30420_e47990 * var_tg0_dn16) / assign30420_e47992), ((assign30420_e47990 * var_tg0_dn17) / assign30420_e47992), ((assign30420_e47990 * var_tg0_dn18) / assign30420_e47992), ((assign30420_e47990 * var_tg0_dn19) / assign30420_e47992), ((assign30420_e47990 * var_tg0_dn20) / assign30420_e47992), ((assign30420_e47990 * var_tg0_dn21) / assign30420_e47992), ((assign30420_e47990 * var_tg0_dn22) / assign30420_e47992),)
            } else {
                let assign30420_e48000: f64 = (-37.0);
                let (assign30420_e48010, assign30420_e48010_d_n0, assign30420_e48010_d_n1, assign30420_e48010_d_n2, assign30420_e48010_d_n3, assign30420_e48010_d_n4, assign30420_e48010_d_n5, assign30420_e48010_d_n6, assign30420_e48010_d_n7, assign30420_e48010_d_n8, assign30420_e48010_d_n9, assign30420_e48010_d_n12, assign30420_e48010_d_n14, assign30420_e48010_d_n15, assign30420_e48010_d_n16, assign30420_e48010_d_n17, assign30420_e48010_d_n18, assign30420_e48010_d_n19, assign30420_e48010_d_n20, assign30420_e48010_d_n21, assign30420_e48010_d_n22,) = {
                    if ((!(var_tg0 >= 37.0)) && (var_tg0 <= assign30420_e48000)) {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign30420_e48009, assign30420_e48009_d_n0, assign30420_e48009_d_n1, assign30420_e48009_d_n2, assign30420_e48009_d_n3, assign30420_e48009_d_n4, assign30420_e48009_d_n5, assign30420_e48009_d_n6, assign30420_e48009_d_n7, assign30420_e48009_d_n8, assign30420_e48009_d_n9, assign30420_e48009_d_n12, assign30420_e48009_d_n14, assign30420_e48009_d_n15, assign30420_e48009_d_n16, assign30420_e48009_d_n17, assign30420_e48009_d_n18, assign30420_e48009_d_n19, assign30420_e48009_d_n20, assign30420_e48009_d_n21, assign30420_e48009_d_n22,) = {
                            if (var_tg0 >= 37.0) {
                                (var_tg0, var_tg0_dn0, var_tg0_dn1, var_tg0_dn2, var_tg0_dn3, var_tg0_dn4, var_tg0_dn5, var_tg0_dn6, var_tg0_dn7, var_tg0_dn8, var_tg0_dn9, var_tg0_dn12, var_tg0_dn14, var_tg0_dn15, var_tg0_dn16, var_tg0_dn17, var_tg0_dn18, var_tg0_dn19, var_tg0_dn20, var_tg0_dn21, var_tg0_dn22,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign30420_e48009, assign30420_e48009_d_n0, assign30420_e48009_d_n1, assign30420_e48009_d_n2, assign30420_e48009_d_n3, assign30420_e48009_d_n4, assign30420_e48009_d_n5, assign30420_e48009_d_n6, assign30420_e48009_d_n7, assign30420_e48009_d_n8, assign30420_e48009_d_n9, assign30420_e48009_d_n12, assign30420_e48009_d_n14, assign30420_e48009_d_n15, assign30420_e48009_d_n16, assign30420_e48009_d_n17, assign30420_e48009_d_n18, assign30420_e48009_d_n19, assign30420_e48009_d_n20, assign30420_e48009_d_n21, assign30420_e48009_d_n22,)
                    }
                };
                (assign30420_e48010, assign30420_e48010_d_n0, assign30420_e48010_d_n1, assign30420_e48010_d_n2, assign30420_e48010_d_n3, assign30420_e48010_d_n4, assign30420_e48010_d_n5, assign30420_e48010_d_n6, assign30420_e48010_d_n7, assign30420_e48010_d_n8, assign30420_e48010_d_n9, assign30420_e48010_d_n12, assign30420_e48010_d_n14, assign30420_e48010_d_n15, assign30420_e48010_d_n16, assign30420_e48010_d_n17, assign30420_e48010_d_n18, assign30420_e48010_d_n19, assign30420_e48010_d_n20, assign30420_e48010_d_n21, assign30420_e48010_d_n22,)
            }
        };
        let assign30420_e48012: f64 = (assign30420_e47978 * assign30420_e48011);
        let assign30420_e48013: f64 = (assign30420_e47975 - assign30420_e48012);
        let assign30420_e48016: f64 = (3.24e17 * var_vtv);
        let assign30420_e48023: f64 = (-37.0);
        let (assign30420_e48049, assign30420_e48049_d_n0, assign30420_e48049_d_n1, assign30420_e48049_d_n2, assign30420_e48049_d_n3, assign30420_e48049_d_n4, assign30420_e48049_d_n5, assign30420_e48049_d_n6, assign30420_e48049_d_n7, assign30420_e48049_d_n8, assign30420_e48049_d_n9, assign30420_e48049_d_n12, assign30420_e48049_d_n14, assign30420_e48049_d_n15, assign30420_e48049_d_n16, assign30420_e48049_d_n17, assign30420_e48049_d_n18, assign30420_e48049_d_n19, assign30420_e48049_d_n20, assign30420_e48049_d_n21, assign30420_e48049_d_n22,) = {
            if ((!(var_tg1 >= 37.0)) && (!(var_tg1 <= assign30420_e48023))) {
                let assign30420_e48028: f64 = (var_tg1).exp();
                let assign30420_e48030: f64 = (assign30420_e48028 + 1.0);
                let assign30420_e48031: f64 = (assign30420_e48030).ln();
                (assign30420_e48031, ((assign30420_e48028 * var_tg1_dn0) / assign30420_e48030), ((assign30420_e48028 * var_tg1_dn1) / assign30420_e48030), ((assign30420_e48028 * var_tg1_dn2) / assign30420_e48030), ((assign30420_e48028 * var_tg1_dn3) / assign30420_e48030), ((assign30420_e48028 * var_tg1_dn4) / assign30420_e48030), ((assign30420_e48028 * var_tg1_dn5) / assign30420_e48030), ((assign30420_e48028 * var_tg1_dn6) / assign30420_e48030), ((assign30420_e48028 * var_tg1_dn7) / assign30420_e48030), ((assign30420_e48028 * var_tg1_dn8) / assign30420_e48030), ((assign30420_e48028 * var_tg1_dn9) / assign30420_e48030), ((assign30420_e48028 * var_tg1_dn12) / assign30420_e48030), ((assign30420_e48028 * var_tg1_dn14) / assign30420_e48030), ((assign30420_e48028 * var_tg1_dn15) / assign30420_e48030), ((assign30420_e48028 * var_tg1_dn16) / assign30420_e48030), ((assign30420_e48028 * var_tg1_dn17) / assign30420_e48030), ((assign30420_e48028 * var_tg1_dn18) / assign30420_e48030), ((assign30420_e48028 * var_tg1_dn19) / assign30420_e48030), ((assign30420_e48028 * var_tg1_dn20) / assign30420_e48030), ((assign30420_e48028 * var_tg1_dn21) / assign30420_e48030), ((assign30420_e48028 * var_tg1_dn22) / assign30420_e48030),)
            } else {
                let assign30420_e48038: f64 = (-37.0);
                let (assign30420_e48048, assign30420_e48048_d_n0, assign30420_e48048_d_n1, assign30420_e48048_d_n2, assign30420_e48048_d_n3, assign30420_e48048_d_n4, assign30420_e48048_d_n5, assign30420_e48048_d_n6, assign30420_e48048_d_n7, assign30420_e48048_d_n8, assign30420_e48048_d_n9, assign30420_e48048_d_n12, assign30420_e48048_d_n14, assign30420_e48048_d_n15, assign30420_e48048_d_n16, assign30420_e48048_d_n17, assign30420_e48048_d_n18, assign30420_e48048_d_n19, assign30420_e48048_d_n20, assign30420_e48048_d_n21, assign30420_e48048_d_n22,) = {
                    if ((!(var_tg1 >= 37.0)) && (var_tg1 <= assign30420_e48038)) {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign30420_e48047, assign30420_e48047_d_n0, assign30420_e48047_d_n1, assign30420_e48047_d_n2, assign30420_e48047_d_n3, assign30420_e48047_d_n4, assign30420_e48047_d_n5, assign30420_e48047_d_n6, assign30420_e48047_d_n7, assign30420_e48047_d_n8, assign30420_e48047_d_n9, assign30420_e48047_d_n12, assign30420_e48047_d_n14, assign30420_e48047_d_n15, assign30420_e48047_d_n16, assign30420_e48047_d_n17, assign30420_e48047_d_n18, assign30420_e48047_d_n19, assign30420_e48047_d_n20, assign30420_e48047_d_n21, assign30420_e48047_d_n22,) = {
                            if (var_tg1 >= 37.0) {
                                (var_tg1, var_tg1_dn0, var_tg1_dn1, var_tg1_dn2, var_tg1_dn3, var_tg1_dn4, var_tg1_dn5, var_tg1_dn6, var_tg1_dn7, var_tg1_dn8, var_tg1_dn9, var_tg1_dn12, var_tg1_dn14, var_tg1_dn15, var_tg1_dn16, var_tg1_dn17, var_tg1_dn18, var_tg1_dn19, var_tg1_dn20, var_tg1_dn21, var_tg1_dn22,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign30420_e48047, assign30420_e48047_d_n0, assign30420_e48047_d_n1, assign30420_e48047_d_n2, assign30420_e48047_d_n3, assign30420_e48047_d_n4, assign30420_e48047_d_n5, assign30420_e48047_d_n6, assign30420_e48047_d_n7, assign30420_e48047_d_n8, assign30420_e48047_d_n9, assign30420_e48047_d_n12, assign30420_e48047_d_n14, assign30420_e48047_d_n15, assign30420_e48047_d_n16, assign30420_e48047_d_n17, assign30420_e48047_d_n18, assign30420_e48047_d_n19, assign30420_e48047_d_n20, assign30420_e48047_d_n21, assign30420_e48047_d_n22,)
                    }
                };
                (assign30420_e48048, assign30420_e48048_d_n0, assign30420_e48048_d_n1, assign30420_e48048_d_n2, assign30420_e48048_d_n3, assign30420_e48048_d_n4, assign30420_e48048_d_n5, assign30420_e48048_d_n6, assign30420_e48048_d_n7, assign30420_e48048_d_n8, assign30420_e48048_d_n9, assign30420_e48048_d_n12, assign30420_e48048_d_n14, assign30420_e48048_d_n15, assign30420_e48048_d_n16, assign30420_e48048_d_n17, assign30420_e48048_d_n18, assign30420_e48048_d_n19, assign30420_e48048_d_n20, assign30420_e48048_d_n21, assign30420_e48048_d_n22,)
            }
        };
        let assign30420_e48050: f64 = (assign30420_e48016 * assign30420_e48049);
        let assign30420_e48051: f64 = (assign30420_e48013 - assign30420_e48050);
        (assign30420_e48051, (((var_cch * var_vgef1_dn0) - (assign30420_e47978 * assign30420_e48011_d_n0)) - (assign30420_e48016 * assign30420_e48049_d_n0)), (((var_cch * var_vgef1_dn1) - (assign30420_e47978 * assign30420_e48011_d_n1)) - (assign30420_e48016 * assign30420_e48049_d_n1)), (((var_cch * var_vgef1_dn2) - (assign30420_e47978 * assign30420_e48011_d_n2)) - (assign30420_e48016 * assign30420_e48049_d_n2)), (((var_cch * var_vgef1_dn3) - (assign30420_e47978 * assign30420_e48011_d_n3)) - (assign30420_e48016 * assign30420_e48049_d_n3)), (((var_cch * var_vgef1_dn4) - (((3.24e17 * var_vtv_dn4) * assign30420_e48011) + (assign30420_e47978 * assign30420_e48011_d_n4))) - (((3.24e17 * var_vtv_dn4) * assign30420_e48049) + (assign30420_e48016 * assign30420_e48049_d_n4))), (((var_cch * var_vgef1_dn5) - (assign30420_e47978 * assign30420_e48011_d_n5)) - (assign30420_e48016 * assign30420_e48049_d_n5)), (((var_cch * var_vgef1_dn6) - (((3.24e17 * var_vtv_dn6) * assign30420_e48011) + (assign30420_e47978 * assign30420_e48011_d_n6))) - (((3.24e17 * var_vtv_dn6) * assign30420_e48049) + (assign30420_e48016 * assign30420_e48049_d_n6))), (((var_cch * var_vgef1_dn7) - (((3.24e17 * var_vtv_dn7) * assign30420_e48011) + (assign30420_e47978 * assign30420_e48011_d_n7))) - (((3.24e17 * var_vtv_dn7) * assign30420_e48049) + (assign30420_e48016 * assign30420_e48049_d_n7))), (((var_cch * var_vgef1_dn8) - (((3.24e17 * var_vtv_dn8) * assign30420_e48011) + (assign30420_e47978 * assign30420_e48011_d_n8))) - (((3.24e17 * var_vtv_dn8) * assign30420_e48049) + (assign30420_e48016 * assign30420_e48049_d_n8))), (((var_cch * var_vgef1_dn9) - (assign30420_e47978 * assign30420_e48011_d_n9)) - (assign30420_e48016 * assign30420_e48049_d_n9)), (((var_cch * var_vgef1_dn12) - (assign30420_e47978 * assign30420_e48011_d_n12)) - (assign30420_e48016 * assign30420_e48049_d_n12)), (((var_cch * var_vgef1_dn14) - (assign30420_e47978 * assign30420_e48011_d_n14)) - (assign30420_e48016 * assign30420_e48049_d_n14)), (((var_cch * var_vgef1_dn15) - (((3.24e17 * var_vtv_dn15) * assign30420_e48011) + (assign30420_e47978 * assign30420_e48011_d_n15))) - (((3.24e17 * var_vtv_dn15) * assign30420_e48049) + (assign30420_e48016 * assign30420_e48049_d_n15))), (((var_cch * var_vgef1_dn16) - (((3.24e17 * var_vtv_dn16) * assign30420_e48011) + (assign30420_e47978 * assign30420_e48011_d_n16))) - (((3.24e17 * var_vtv_dn16) * assign30420_e48049) + (assign30420_e48016 * assign30420_e48049_d_n16))), (((var_cch * var_vgef1_dn17) - (((3.24e17 * var_vtv_dn17) * assign30420_e48011) + (assign30420_e47978 * assign30420_e48011_d_n17))) - (((3.24e17 * var_vtv_dn17) * assign30420_e48049) + (assign30420_e48016 * assign30420_e48049_d_n17))), (((var_cch * var_vgef1_dn18) - (((3.24e17 * var_vtv_dn18) * assign30420_e48011) + (assign30420_e47978 * assign30420_e48011_d_n18))) - (((3.24e17 * var_vtv_dn18) * assign30420_e48049) + (assign30420_e48016 * assign30420_e48049_d_n18))), (((var_cch * var_vgef1_dn19) - (((3.24e17 * var_vtv_dn19) * assign30420_e48011) + (assign30420_e47978 * assign30420_e48011_d_n19))) - (((3.24e17 * var_vtv_dn19) * assign30420_e48049) + (assign30420_e48016 * assign30420_e48049_d_n19))), (((var_cch * var_vgef1_dn20) - (((3.24e17 * var_vtv_dn20) * assign30420_e48011) + (assign30420_e47978 * assign30420_e48011_d_n20))) - (((3.24e17 * var_vtv_dn20) * assign30420_e48049) + (assign30420_e48016 * assign30420_e48049_d_n20))), (((var_cch * var_vgef1_dn21) - (((3.24e17 * var_vtv_dn21) * assign30420_e48011) + (assign30420_e47978 * assign30420_e48011_d_n21))) - (((3.24e17 * var_vtv_dn21) * assign30420_e48049) + (assign30420_e48016 * assign30420_e48049_d_n21))), (((var_cch * var_vgef1_dn22) - (((3.24e17 * var_vtv_dn22) * assign30420_e48011) + (assign30420_e47978 * assign30420_e48011_d_n22))) - (((3.24e17 * var_vtv_dn22) * assign30420_e48049) + (assign30420_e48016 * assign30420_e48049_d_n22))),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn1, var_t4_dn2, var_t4_dn3, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn12, var_t4_dn14, var_t4_dn15, var_t4_dn16, var_t4_dn17, var_t4_dn18, var_t4_dn19, var_t4_dn20, var_t4_dn21, var_t4_dn22,)
    }
};
        var_t4 = assign30420_e48053;
        var_t4_dn0 = assign30420_e48053_d_n0;
        var_t4_dn1 = assign30420_e48053_d_n1;
        var_t4_dn2 = assign30420_e48053_d_n2;
        var_t4_dn3 = assign30420_e48053_d_n3;
        var_t4_dn4 = assign30420_e48053_d_n4;
        var_t4_dn5 = assign30420_e48053_d_n5;
        var_t4_dn6 = assign30420_e48053_d_n6;
        var_t4_dn7 = assign30420_e48053_d_n7;
        var_t4_dn8 = assign30420_e48053_d_n8;
        var_t4_dn9 = assign30420_e48053_d_n9;
        var_t4_dn12 = assign30420_e48053_d_n12;
        var_t4_dn14 = assign30420_e48053_d_n14;
        var_t4_dn15 = assign30420_e48053_d_n15;
        var_t4_dn16 = assign30420_e48053_d_n16;
        var_t4_dn17 = assign30420_e48053_d_n17;
        var_t4_dn18 = assign30420_e48053_d_n18;
        var_t4_dn19 = assign30420_e48053_d_n19;
        var_t4_dn20 = assign30420_e48053_d_n20;
        var_t4_dn21 = assign30420_e48053_d_n21;
        var_t4_dn22 = assign30420_e48053_d_n22;

        let (assign30430_e48066, assign30430_e48066_d_n0, assign30430_e48066_d_n1, assign30430_e48066_d_n2, assign30430_e48066_d_n3, assign30430_e48066_d_n4, assign30430_e48066_d_n5, assign30430_e48066_d_n6, assign30430_e48066_d_n7, assign30430_e48066_d_n8, assign30430_e48066_d_n9, assign30430_e48066_d_n12, assign30430_e48066_d_n14, assign30430_e48066_d_n15, assign30430_e48066_d_n16, assign30430_e48066_d_n17, assign30430_e48066_d_n18, assign30430_e48066_d_n19, assign30430_e48066_d_n20, assign30430_e48066_d_n21, assign30430_e48066_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let assign30430_e48062: f64 = (p.p208 * var_t0);
        let assign30430_e48064: f64 = (assign30430_e48062 * var_t2);
        (assign30430_e48064, (((p.p208 * var_t0_dn0) * var_t2) + (assign30430_e48062 * var_t2_dn0)), (((p.p208 * var_t0_dn1) * var_t2) + (assign30430_e48062 * var_t2_dn1)), (((p.p208 * var_t0_dn2) * var_t2) + (assign30430_e48062 * var_t2_dn2)), (((p.p208 * var_t0_dn3) * var_t2) + (assign30430_e48062 * var_t2_dn3)), (((p.p208 * var_t0_dn4) * var_t2) + (assign30430_e48062 * var_t2_dn4)), (((p.p208 * var_t0_dn5) * var_t2) + (assign30430_e48062 * var_t2_dn5)), (((p.p208 * var_t0_dn6) * var_t2) + (assign30430_e48062 * var_t2_dn6)), (((p.p208 * var_t0_dn7) * var_t2) + (assign30430_e48062 * var_t2_dn7)), (((p.p208 * var_t0_dn8) * var_t2) + (assign30430_e48062 * var_t2_dn8)), (((p.p208 * var_t0_dn9) * var_t2) + (assign30430_e48062 * var_t2_dn9)), (((p.p208 * var_t0_dn12) * var_t2) + (assign30430_e48062 * var_t2_dn12)), (((p.p208 * var_t0_dn14) * var_t2) + (assign30430_e48062 * var_t2_dn14)), (((p.p208 * var_t0_dn15) * var_t2) + (assign30430_e48062 * var_t2_dn15)), (((p.p208 * var_t0_dn16) * var_t2) + (assign30430_e48062 * var_t2_dn16)), (((p.p208 * var_t0_dn17) * var_t2) + (assign30430_e48062 * var_t2_dn17)), (((p.p208 * var_t0_dn18) * var_t2) + (assign30430_e48062 * var_t2_dn18)), (((p.p208 * var_t0_dn19) * var_t2) + (assign30430_e48062 * var_t2_dn19)), (((p.p208 * var_t0_dn20) * var_t2) + (assign30430_e48062 * var_t2_dn20)), (((p.p208 * var_t0_dn21) * var_t2) + (assign30430_e48062 * var_t2_dn21)), (((p.p208 * var_t0_dn22) * var_t2) + (assign30430_e48062 * var_t2_dn22)),)
    } else {
        (var_vgefm13g0, var_vgefm13g0_dn0, var_vgefm13g0_dn1, var_vgefm13g0_dn2, var_vgefm13g0_dn3, var_vgefm13g0_dn4, var_vgefm13g0_dn5, var_vgefm13g0_dn6, var_vgefm13g0_dn7, var_vgefm13g0_dn8, var_vgefm13g0_dn9, var_vgefm13g0_dn12, var_vgefm13g0_dn14, var_vgefm13g0_dn15, var_vgefm13g0_dn16, var_vgefm13g0_dn17, var_vgefm13g0_dn18, var_vgefm13g0_dn19, var_vgefm13g0_dn20, var_vgefm13g0_dn21, var_vgefm13g0_dn22,)
    }
};
        var_vgefm13g0 = assign30430_e48066;
        var_vgefm13g0_dn0 = assign30430_e48066_d_n0;
        var_vgefm13g0_dn1 = assign30430_e48066_d_n1;
        var_vgefm13g0_dn2 = assign30430_e48066_d_n2;
        var_vgefm13g0_dn3 = assign30430_e48066_d_n3;
        var_vgefm13g0_dn4 = assign30430_e48066_d_n4;
        var_vgefm13g0_dn5 = assign30430_e48066_d_n5;
        var_vgefm13g0_dn6 = assign30430_e48066_d_n6;
        var_vgefm13g0_dn7 = assign30430_e48066_d_n7;
        var_vgefm13g0_dn8 = assign30430_e48066_d_n8;
        var_vgefm13g0_dn9 = assign30430_e48066_d_n9;
        var_vgefm13g0_dn12 = assign30430_e48066_d_n12;
        var_vgefm13g0_dn14 = assign30430_e48066_d_n14;
        var_vgefm13g0_dn15 = assign30430_e48066_d_n15;
        var_vgefm13g0_dn16 = assign30430_e48066_d_n16;
        var_vgefm13g0_dn17 = assign30430_e48066_d_n17;
        var_vgefm13g0_dn18 = assign30430_e48066_d_n18;
        var_vgefm13g0_dn19 = assign30430_e48066_d_n19;
        var_vgefm13g0_dn20 = assign30430_e48066_d_n20;
        var_vgefm13g0_dn21 = assign30430_e48066_d_n21;
        var_vgefm13g0_dn22 = assign30430_e48066_d_n22;

        let (assign30440_e48079, assign30440_e48079_d_n0, assign30440_e48079_d_n1, assign30440_e48079_d_n2, assign30440_e48079_d_n3, assign30440_e48079_d_n4, assign30440_e48079_d_n5, assign30440_e48079_d_n6, assign30440_e48079_d_n7, assign30440_e48079_d_n8, assign30440_e48079_d_n9, assign30440_e48079_d_n12, assign30440_e48079_d_n14, assign30440_e48079_d_n15, assign30440_e48079_d_n16, assign30440_e48079_d_n17, assign30440_e48079_d_n18, assign30440_e48079_d_n19, assign30440_e48079_d_n20, assign30440_e48079_d_n21, assign30440_e48079_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let assign30440_e48075: f64 = (p.p209 * var_t0);
        let assign30440_e48077: f64 = (assign30440_e48075 * var_t2);
        (assign30440_e48077, (((p.p209 * var_t0_dn0) * var_t2) + (assign30440_e48075 * var_t2_dn0)), (((p.p209 * var_t0_dn1) * var_t2) + (assign30440_e48075 * var_t2_dn1)), (((p.p209 * var_t0_dn2) * var_t2) + (assign30440_e48075 * var_t2_dn2)), (((p.p209 * var_t0_dn3) * var_t2) + (assign30440_e48075 * var_t2_dn3)), (((p.p209 * var_t0_dn4) * var_t2) + (assign30440_e48075 * var_t2_dn4)), (((p.p209 * var_t0_dn5) * var_t2) + (assign30440_e48075 * var_t2_dn5)), (((p.p209 * var_t0_dn6) * var_t2) + (assign30440_e48075 * var_t2_dn6)), (((p.p209 * var_t0_dn7) * var_t2) + (assign30440_e48075 * var_t2_dn7)), (((p.p209 * var_t0_dn8) * var_t2) + (assign30440_e48075 * var_t2_dn8)), (((p.p209 * var_t0_dn9) * var_t2) + (assign30440_e48075 * var_t2_dn9)), (((p.p209 * var_t0_dn12) * var_t2) + (assign30440_e48075 * var_t2_dn12)), (((p.p209 * var_t0_dn14) * var_t2) + (assign30440_e48075 * var_t2_dn14)), (((p.p209 * var_t0_dn15) * var_t2) + (assign30440_e48075 * var_t2_dn15)), (((p.p209 * var_t0_dn16) * var_t2) + (assign30440_e48075 * var_t2_dn16)), (((p.p209 * var_t0_dn17) * var_t2) + (assign30440_e48075 * var_t2_dn17)), (((p.p209 * var_t0_dn18) * var_t2) + (assign30440_e48075 * var_t2_dn18)), (((p.p209 * var_t0_dn19) * var_t2) + (assign30440_e48075 * var_t2_dn19)), (((p.p209 * var_t0_dn20) * var_t2) + (assign30440_e48075 * var_t2_dn20)), (((p.p209 * var_t0_dn21) * var_t2) + (assign30440_e48075 * var_t2_dn21)), (((p.p209 * var_t0_dn22) * var_t2) + (assign30440_e48075 * var_t2_dn22)),)
    } else {
        (var_vgefm13g1, var_vgefm13g1_dn0, var_vgefm13g1_dn1, var_vgefm13g1_dn2, var_vgefm13g1_dn3, var_vgefm13g1_dn4, var_vgefm13g1_dn5, var_vgefm13g1_dn6, var_vgefm13g1_dn7, var_vgefm13g1_dn8, var_vgefm13g1_dn9, var_vgefm13g1_dn12, var_vgefm13g1_dn14, var_vgefm13g1_dn15, var_vgefm13g1_dn16, var_vgefm13g1_dn17, var_vgefm13g1_dn18, var_vgefm13g1_dn19, var_vgefm13g1_dn20, var_vgefm13g1_dn21, var_vgefm13g1_dn22,)
    }
};
        var_vgefm13g1 = assign30440_e48079;
        var_vgefm13g1_dn0 = assign30440_e48079_d_n0;
        var_vgefm13g1_dn1 = assign30440_e48079_d_n1;
        var_vgefm13g1_dn2 = assign30440_e48079_d_n2;
        var_vgefm13g1_dn3 = assign30440_e48079_d_n3;
        var_vgefm13g1_dn4 = assign30440_e48079_d_n4;
        var_vgefm13g1_dn5 = assign30440_e48079_d_n5;
        var_vgefm13g1_dn6 = assign30440_e48079_d_n6;
        var_vgefm13g1_dn7 = assign30440_e48079_d_n7;
        var_vgefm13g1_dn8 = assign30440_e48079_d_n8;
        var_vgefm13g1_dn9 = assign30440_e48079_d_n9;
        var_vgefm13g1_dn12 = assign30440_e48079_d_n12;
        var_vgefm13g1_dn14 = assign30440_e48079_d_n14;
        var_vgefm13g1_dn15 = assign30440_e48079_d_n15;
        var_vgefm13g1_dn16 = assign30440_e48079_d_n16;
        var_vgefm13g1_dn17 = assign30440_e48079_d_n17;
        var_vgefm13g1_dn18 = assign30440_e48079_d_n18;
        var_vgefm13g1_dn19 = assign30440_e48079_d_n19;
        var_vgefm13g1_dn20 = assign30440_e48079_d_n20;
        var_vgefm13g1_dn21 = assign30440_e48079_d_n21;
        var_vgefm13g1_dn22 = assign30440_e48079_d_n22;

        let (assign30450_e48097, assign30450_e48097_d_n0, assign30450_e48097_d_n1, assign30450_e48097_d_n2, assign30450_e48097_d_n3, assign30450_e48097_d_n4, assign30450_e48097_d_n5, assign30450_e48097_d_n6, assign30450_e48097_d_n7, assign30450_e48097_d_n8, assign30450_e48097_d_n9, assign30450_e48097_d_n12, assign30450_e48097_d_n14, assign30450_e48097_d_n15, assign30450_e48097_d_n16, assign30450_e48097_d_n17, assign30450_e48097_d_n18, assign30450_e48097_d_n19, assign30450_e48097_d_n20, assign30450_e48097_d_n21, assign30450_e48097_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let assign30450_e48087: f64 = { let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign30450_e48089: f64 = (assign30450_e48087 * 3.24e17);
        let assign30450_e48093: f64 = (0.6666666666666666 * var_vgefm13g0);
        let assign30450_e48094: f64 = (1.0 + assign30450_e48093);
        let assign30450_e48095: f64 = (assign30450_e48089 * assign30450_e48094);
        (assign30450_e48095, (((({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn0) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * var_vgefm13g0_dn0))), (((({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn1) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * var_vgefm13g0_dn1))), (((({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn2) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * var_vgefm13g0_dn2))), (((({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn3) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * var_vgefm13g0_dn3))), (((({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn4) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * var_vgefm13g0_dn4))), (((({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn5) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * var_vgefm13g0_dn5))), (((({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn6) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * var_vgefm13g0_dn6))), (((({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn7) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * var_vgefm13g0_dn7))), (((({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn8) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * var_vgefm13g0_dn8))), (((({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn9) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * var_vgefm13g0_dn9))), (((({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn12) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * var_vgefm13g0_dn12))), (((({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn14) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * var_vgefm13g0_dn14))), (((({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn15) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * var_vgefm13g0_dn15))), (((({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn16) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * var_vgefm13g0_dn16))), (((({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn17) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * var_vgefm13g0_dn17))), (((({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn18) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * var_vgefm13g0_dn18))), (((({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn19) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * var_vgefm13g0_dn19))), (((({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn20) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * var_vgefm13g0_dn20))), (((({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn21) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * var_vgefm13g0_dn21))), (((({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn22) * 3.24e17) * assign30450_e48094) + (assign30450_e48089 * (0.6666666666666666 * var_vgefm13g0_dn22))),)
    } else {
        (var_t5ng0, var_t5ng0_dn0, var_t5ng0_dn1, var_t5ng0_dn2, var_t5ng0_dn3, var_t5ng0_dn4, var_t5ng0_dn5, var_t5ng0_dn6, var_t5ng0_dn7, var_t5ng0_dn8, var_t5ng0_dn9, var_t5ng0_dn12, var_t5ng0_dn14, var_t5ng0_dn15, var_t5ng0_dn16, var_t5ng0_dn17, var_t5ng0_dn18, var_t5ng0_dn19, var_t5ng0_dn20, var_t5ng0_dn21, var_t5ng0_dn22,)
    }
};
        var_t5ng0 = assign30450_e48097;
        var_t5ng0_dn0 = assign30450_e48097_d_n0;
        var_t5ng0_dn1 = assign30450_e48097_d_n1;
        var_t5ng0_dn2 = assign30450_e48097_d_n2;
        var_t5ng0_dn3 = assign30450_e48097_d_n3;
        var_t5ng0_dn4 = assign30450_e48097_d_n4;
        var_t5ng0_dn5 = assign30450_e48097_d_n5;
        var_t5ng0_dn6 = assign30450_e48097_d_n6;
        var_t5ng0_dn7 = assign30450_e48097_d_n7;
        var_t5ng0_dn8 = assign30450_e48097_d_n8;
        var_t5ng0_dn9 = assign30450_e48097_d_n9;
        var_t5ng0_dn12 = assign30450_e48097_d_n12;
        var_t5ng0_dn14 = assign30450_e48097_d_n14;
        var_t5ng0_dn15 = assign30450_e48097_d_n15;
        var_t5ng0_dn16 = assign30450_e48097_d_n16;
        var_t5ng0_dn17 = assign30450_e48097_d_n17;
        var_t5ng0_dn18 = assign30450_e48097_d_n18;
        var_t5ng0_dn19 = assign30450_e48097_d_n19;
        var_t5ng0_dn20 = assign30450_e48097_d_n20;
        var_t5ng0_dn21 = assign30450_e48097_d_n21;
        var_t5ng0_dn22 = assign30450_e48097_d_n22;

        let (assign30460_e48109, assign30460_e48109_d_n0, assign30460_e48109_d_n1, assign30460_e48109_d_n2, assign30460_e48109_d_n3, assign30460_e48109_d_n4, assign30460_e48109_d_n5, assign30460_e48109_d_n6, assign30460_e48109_d_n7, assign30460_e48109_d_n8, assign30460_e48109_d_n9, assign30460_e48109_d_n12, assign30460_e48109_d_n14, assign30460_e48109_d_n15, assign30460_e48109_d_n16, assign30460_e48109_d_n17, assign30460_e48109_d_n18, assign30460_e48109_d_n19, assign30460_e48109_d_n20, assign30460_e48109_d_n21, assign30460_e48109_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let assign30460_e48106: f64 = { let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign30460_e48107: f64 = (1.0 + assign30460_e48106);
        (assign30460_e48107, ({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn0), ({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn1), ({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn2), ({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn3), ({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn4), ({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn5), ({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn6), ({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn7), ({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn8), ({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn9), ({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn12), ({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn14), ({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn15), ({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn16), ({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn17), ({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn18), ({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn19), ({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn20), ({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn21), ({ let limited_exp_arg = var_tg0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg0_dn22),)
    } else {
        (var_t5dg0, var_t5dg0_dn0, var_t5dg0_dn1, var_t5dg0_dn2, var_t5dg0_dn3, var_t5dg0_dn4, var_t5dg0_dn5, var_t5dg0_dn6, var_t5dg0_dn7, var_t5dg0_dn8, var_t5dg0_dn9, var_t5dg0_dn12, var_t5dg0_dn14, var_t5dg0_dn15, var_t5dg0_dn16, var_t5dg0_dn17, var_t5dg0_dn18, var_t5dg0_dn19, var_t5dg0_dn20, var_t5dg0_dn21, var_t5dg0_dn22,)
    }
};
        var_t5dg0 = assign30460_e48109;
        var_t5dg0_dn0 = assign30460_e48109_d_n0;
        var_t5dg0_dn1 = assign30460_e48109_d_n1;
        var_t5dg0_dn2 = assign30460_e48109_d_n2;
        var_t5dg0_dn3 = assign30460_e48109_d_n3;
        var_t5dg0_dn4 = assign30460_e48109_d_n4;
        var_t5dg0_dn5 = assign30460_e48109_d_n5;
        var_t5dg0_dn6 = assign30460_e48109_d_n6;
        var_t5dg0_dn7 = assign30460_e48109_d_n7;
        var_t5dg0_dn8 = assign30460_e48109_d_n8;
        var_t5dg0_dn9 = assign30460_e48109_d_n9;
        var_t5dg0_dn12 = assign30460_e48109_d_n12;
        var_t5dg0_dn14 = assign30460_e48109_d_n14;
        var_t5dg0_dn15 = assign30460_e48109_d_n15;
        var_t5dg0_dn16 = assign30460_e48109_d_n16;
        var_t5dg0_dn17 = assign30460_e48109_d_n17;
        var_t5dg0_dn18 = assign30460_e48109_d_n18;
        var_t5dg0_dn19 = assign30460_e48109_d_n19;
        var_t5dg0_dn20 = assign30460_e48109_d_n20;
        var_t5dg0_dn21 = assign30460_e48109_d_n21;
        var_t5dg0_dn22 = assign30460_e48109_d_n22;

        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn1_slot = var_t0_dn1;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn14_slot = var_t0_dn14;
        *var_t0_dn15_slot = var_t0_dn15;
        *var_t0_dn16_slot = var_t0_dn16;
        *var_t0_dn17_slot = var_t0_dn17;
        *var_t0_dn18_slot = var_t0_dn18;
        *var_t0_dn19_slot = var_t0_dn19;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn20_slot = var_t0_dn20;
        *var_t0_dn21_slot = var_t0_dn21;
        *var_t0_dn22_slot = var_t0_dn22;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn1_slot = var_t1_dn1;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn15_slot = var_t1_dn15;
        *var_t1_dn16_slot = var_t1_dn16;
        *var_t1_dn17_slot = var_t1_dn17;
        *var_t1_dn18_slot = var_t1_dn18;
        *var_t1_dn19_slot = var_t1_dn19;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn20_slot = var_t1_dn20;
        *var_t1_dn21_slot = var_t1_dn21;
        *var_t1_dn22_slot = var_t1_dn22;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn1_slot = var_t2_dn1;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn15_slot = var_t2_dn15;
        *var_t2_dn16_slot = var_t2_dn16;
        *var_t2_dn17_slot = var_t2_dn17;
        *var_t2_dn18_slot = var_t2_dn18;
        *var_t2_dn19_slot = var_t2_dn19;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn20_slot = var_t2_dn20;
        *var_t2_dn21_slot = var_t2_dn21;
        *var_t2_dn22_slot = var_t2_dn22;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn1_slot = var_t4_dn1;
        *var_t4_dn12_slot = var_t4_dn12;
        *var_t4_dn14_slot = var_t4_dn14;
        *var_t4_dn15_slot = var_t4_dn15;
        *var_t4_dn16_slot = var_t4_dn16;
        *var_t4_dn17_slot = var_t4_dn17;
        *var_t4_dn18_slot = var_t4_dn18;
        *var_t4_dn19_slot = var_t4_dn19;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn20_slot = var_t4_dn20;
        *var_t4_dn21_slot = var_t4_dn21;
        *var_t4_dn22_slot = var_t4_dn22;
        *var_t4_dn3_slot = var_t4_dn3;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_dn9_slot = var_t4_dn9;
        *var_t5dg0_slot = var_t5dg0;
        *var_t5dg0_dn0_slot = var_t5dg0_dn0;
        *var_t5dg0_dn1_slot = var_t5dg0_dn1;
        *var_t5dg0_dn12_slot = var_t5dg0_dn12;
        *var_t5dg0_dn14_slot = var_t5dg0_dn14;
        *var_t5dg0_dn15_slot = var_t5dg0_dn15;
        *var_t5dg0_dn16_slot = var_t5dg0_dn16;
        *var_t5dg0_dn17_slot = var_t5dg0_dn17;
        *var_t5dg0_dn18_slot = var_t5dg0_dn18;
        *var_t5dg0_dn19_slot = var_t5dg0_dn19;
        *var_t5dg0_dn2_slot = var_t5dg0_dn2;
        *var_t5dg0_dn20_slot = var_t5dg0_dn20;
        *var_t5dg0_dn21_slot = var_t5dg0_dn21;
        *var_t5dg0_dn22_slot = var_t5dg0_dn22;
        *var_t5dg0_dn3_slot = var_t5dg0_dn3;
        *var_t5dg0_dn4_slot = var_t5dg0_dn4;
        *var_t5dg0_dn5_slot = var_t5dg0_dn5;
        *var_t5dg0_dn6_slot = var_t5dg0_dn6;
        *var_t5dg0_dn7_slot = var_t5dg0_dn7;
        *var_t5dg0_dn8_slot = var_t5dg0_dn8;
        *var_t5dg0_dn9_slot = var_t5dg0_dn9;
        *var_t5ng0_slot = var_t5ng0;
        *var_t5ng0_dn0_slot = var_t5ng0_dn0;
        *var_t5ng0_dn1_slot = var_t5ng0_dn1;
        *var_t5ng0_dn12_slot = var_t5ng0_dn12;
        *var_t5ng0_dn14_slot = var_t5ng0_dn14;
        *var_t5ng0_dn15_slot = var_t5ng0_dn15;
        *var_t5ng0_dn16_slot = var_t5ng0_dn16;
        *var_t5ng0_dn17_slot = var_t5ng0_dn17;
        *var_t5ng0_dn18_slot = var_t5ng0_dn18;
        *var_t5ng0_dn19_slot = var_t5ng0_dn19;
        *var_t5ng0_dn2_slot = var_t5ng0_dn2;
        *var_t5ng0_dn20_slot = var_t5ng0_dn20;
        *var_t5ng0_dn21_slot = var_t5ng0_dn21;
        *var_t5ng0_dn22_slot = var_t5ng0_dn22;
        *var_t5ng0_dn3_slot = var_t5ng0_dn3;
        *var_t5ng0_dn4_slot = var_t5ng0_dn4;
        *var_t5ng0_dn5_slot = var_t5ng0_dn5;
        *var_t5ng0_dn6_slot = var_t5ng0_dn6;
        *var_t5ng0_dn7_slot = var_t5ng0_dn7;
        *var_t5ng0_dn8_slot = var_t5ng0_dn8;
        *var_t5ng0_dn9_slot = var_t5ng0_dn9;
        *var_tg0_slot = var_tg0;
        *var_tg0_dn0_slot = var_tg0_dn0;
        *var_tg0_dn1_slot = var_tg0_dn1;
        *var_tg0_dn12_slot = var_tg0_dn12;
        *var_tg0_dn14_slot = var_tg0_dn14;
        *var_tg0_dn15_slot = var_tg0_dn15;
        *var_tg0_dn16_slot = var_tg0_dn16;
        *var_tg0_dn17_slot = var_tg0_dn17;
        *var_tg0_dn18_slot = var_tg0_dn18;
        *var_tg0_dn19_slot = var_tg0_dn19;
        *var_tg0_dn2_slot = var_tg0_dn2;
        *var_tg0_dn20_slot = var_tg0_dn20;
        *var_tg0_dn21_slot = var_tg0_dn21;
        *var_tg0_dn22_slot = var_tg0_dn22;
        *var_tg0_dn3_slot = var_tg0_dn3;
        *var_tg0_dn4_slot = var_tg0_dn4;
        *var_tg0_dn5_slot = var_tg0_dn5;
        *var_tg0_dn6_slot = var_tg0_dn6;
        *var_tg0_dn7_slot = var_tg0_dn7;
        *var_tg0_dn8_slot = var_tg0_dn8;
        *var_tg0_dn9_slot = var_tg0_dn9;
        *var_tg1_slot = var_tg1;
        *var_tg1_dn0_slot = var_tg1_dn0;
        *var_tg1_dn1_slot = var_tg1_dn1;
        *var_tg1_dn12_slot = var_tg1_dn12;
        *var_tg1_dn14_slot = var_tg1_dn14;
        *var_tg1_dn15_slot = var_tg1_dn15;
        *var_tg1_dn16_slot = var_tg1_dn16;
        *var_tg1_dn17_slot = var_tg1_dn17;
        *var_tg1_dn18_slot = var_tg1_dn18;
        *var_tg1_dn19_slot = var_tg1_dn19;
        *var_tg1_dn2_slot = var_tg1_dn2;
        *var_tg1_dn20_slot = var_tg1_dn20;
        *var_tg1_dn21_slot = var_tg1_dn21;
        *var_tg1_dn22_slot = var_tg1_dn22;
        *var_tg1_dn3_slot = var_tg1_dn3;
        *var_tg1_dn4_slot = var_tg1_dn4;
        *var_tg1_dn5_slot = var_tg1_dn5;
        *var_tg1_dn6_slot = var_tg1_dn6;
        *var_tg1_dn7_slot = var_tg1_dn7;
        *var_tg1_dn8_slot = var_tg1_dn8;
        *var_tg1_dn9_slot = var_tg1_dn9;
        *var_vgef1_slot = var_vgef1;
        *var_vgef1_dn0_slot = var_vgef1_dn0;
        *var_vgef1_dn1_slot = var_vgef1_dn1;
        *var_vgef1_dn12_slot = var_vgef1_dn12;
        *var_vgef1_dn14_slot = var_vgef1_dn14;
        *var_vgef1_dn15_slot = var_vgef1_dn15;
        *var_vgef1_dn16_slot = var_vgef1_dn16;
        *var_vgef1_dn17_slot = var_vgef1_dn17;
        *var_vgef1_dn18_slot = var_vgef1_dn18;
        *var_vgef1_dn19_slot = var_vgef1_dn19;
        *var_vgef1_dn2_slot = var_vgef1_dn2;
        *var_vgef1_dn20_slot = var_vgef1_dn20;
        *var_vgef1_dn21_slot = var_vgef1_dn21;
        *var_vgef1_dn22_slot = var_vgef1_dn22;
        *var_vgef1_dn3_slot = var_vgef1_dn3;
        *var_vgef1_dn4_slot = var_vgef1_dn4;
        *var_vgef1_dn5_slot = var_vgef1_dn5;
        *var_vgef1_dn6_slot = var_vgef1_dn6;
        *var_vgef1_dn7_slot = var_vgef1_dn7;
        *var_vgef1_dn8_slot = var_vgef1_dn8;
        *var_vgef1_dn9_slot = var_vgef1_dn9;
        *var_vgef23g0_slot = var_vgef23g0;
        *var_vgef23g0_dn0_slot = var_vgef23g0_dn0;
        *var_vgef23g0_dn1_slot = var_vgef23g0_dn1;
        *var_vgef23g0_dn12_slot = var_vgef23g0_dn12;
        *var_vgef23g0_dn14_slot = var_vgef23g0_dn14;
        *var_vgef23g0_dn15_slot = var_vgef23g0_dn15;
        *var_vgef23g0_dn16_slot = var_vgef23g0_dn16;
        *var_vgef23g0_dn17_slot = var_vgef23g0_dn17;
        *var_vgef23g0_dn18_slot = var_vgef23g0_dn18;
        *var_vgef23g0_dn19_slot = var_vgef23g0_dn19;
        *var_vgef23g0_dn2_slot = var_vgef23g0_dn2;
        *var_vgef23g0_dn20_slot = var_vgef23g0_dn20;
        *var_vgef23g0_dn21_slot = var_vgef23g0_dn21;
        *var_vgef23g0_dn22_slot = var_vgef23g0_dn22;
        *var_vgef23g0_dn3_slot = var_vgef23g0_dn3;
        *var_vgef23g0_dn4_slot = var_vgef23g0_dn4;
        *var_vgef23g0_dn5_slot = var_vgef23g0_dn5;
        *var_vgef23g0_dn6_slot = var_vgef23g0_dn6;
        *var_vgef23g0_dn7_slot = var_vgef23g0_dn7;
        *var_vgef23g0_dn8_slot = var_vgef23g0_dn8;
        *var_vgef23g0_dn9_slot = var_vgef23g0_dn9;
        *var_vgef23g1_slot = var_vgef23g1;
        *var_vgef23g1_dn0_slot = var_vgef23g1_dn0;
        *var_vgef23g1_dn1_slot = var_vgef23g1_dn1;
        *var_vgef23g1_dn12_slot = var_vgef23g1_dn12;
        *var_vgef23g1_dn14_slot = var_vgef23g1_dn14;
        *var_vgef23g1_dn15_slot = var_vgef23g1_dn15;
        *var_vgef23g1_dn16_slot = var_vgef23g1_dn16;
        *var_vgef23g1_dn17_slot = var_vgef23g1_dn17;
        *var_vgef23g1_dn18_slot = var_vgef23g1_dn18;
        *var_vgef23g1_dn19_slot = var_vgef23g1_dn19;
        *var_vgef23g1_dn2_slot = var_vgef23g1_dn2;
        *var_vgef23g1_dn20_slot = var_vgef23g1_dn20;
        *var_vgef23g1_dn21_slot = var_vgef23g1_dn21;
        *var_vgef23g1_dn22_slot = var_vgef23g1_dn22;
        *var_vgef23g1_dn3_slot = var_vgef23g1_dn3;
        *var_vgef23g1_dn4_slot = var_vgef23g1_dn4;
        *var_vgef23g1_dn5_slot = var_vgef23g1_dn5;
        *var_vgef23g1_dn6_slot = var_vgef23g1_dn6;
        *var_vgef23g1_dn7_slot = var_vgef23g1_dn7;
        *var_vgef23g1_dn8_slot = var_vgef23g1_dn8;
        *var_vgef23g1_dn9_slot = var_vgef23g1_dn9;
        *var_vgefm13g0_slot = var_vgefm13g0;
        *var_vgefm13g0_dn0_slot = var_vgefm13g0_dn0;
        *var_vgefm13g0_dn1_slot = var_vgefm13g0_dn1;
        *var_vgefm13g0_dn12_slot = var_vgefm13g0_dn12;
        *var_vgefm13g0_dn14_slot = var_vgefm13g0_dn14;
        *var_vgefm13g0_dn15_slot = var_vgefm13g0_dn15;
        *var_vgefm13g0_dn16_slot = var_vgefm13g0_dn16;
        *var_vgefm13g0_dn17_slot = var_vgefm13g0_dn17;
        *var_vgefm13g0_dn18_slot = var_vgefm13g0_dn18;
        *var_vgefm13g0_dn19_slot = var_vgefm13g0_dn19;
        *var_vgefm13g0_dn2_slot = var_vgefm13g0_dn2;
        *var_vgefm13g0_dn20_slot = var_vgefm13g0_dn20;
        *var_vgefm13g0_dn21_slot = var_vgefm13g0_dn21;
        *var_vgefm13g0_dn22_slot = var_vgefm13g0_dn22;
        *var_vgefm13g0_dn3_slot = var_vgefm13g0_dn3;
        *var_vgefm13g0_dn4_slot = var_vgefm13g0_dn4;
        *var_vgefm13g0_dn5_slot = var_vgefm13g0_dn5;
        *var_vgefm13g0_dn6_slot = var_vgefm13g0_dn6;
        *var_vgefm13g0_dn7_slot = var_vgefm13g0_dn7;
        *var_vgefm13g0_dn8_slot = var_vgefm13g0_dn8;
        *var_vgefm13g0_dn9_slot = var_vgefm13g0_dn9;
        *var_vgefm13g1_slot = var_vgefm13g1;
        *var_vgefm13g1_dn0_slot = var_vgefm13g1_dn0;
        *var_vgefm13g1_dn1_slot = var_vgefm13g1_dn1;
        *var_vgefm13g1_dn12_slot = var_vgefm13g1_dn12;
        *var_vgefm13g1_dn14_slot = var_vgefm13g1_dn14;
        *var_vgefm13g1_dn15_slot = var_vgefm13g1_dn15;
        *var_vgefm13g1_dn16_slot = var_vgefm13g1_dn16;
        *var_vgefm13g1_dn17_slot = var_vgefm13g1_dn17;
        *var_vgefm13g1_dn18_slot = var_vgefm13g1_dn18;
        *var_vgefm13g1_dn19_slot = var_vgefm13g1_dn19;
        *var_vgefm13g1_dn2_slot = var_vgefm13g1_dn2;
        *var_vgefm13g1_dn20_slot = var_vgefm13g1_dn20;
        *var_vgefm13g1_dn21_slot = var_vgefm13g1_dn21;
        *var_vgefm13g1_dn22_slot = var_vgefm13g1_dn22;
        *var_vgefm13g1_dn3_slot = var_vgefm13g1_dn3;
        *var_vgefm13g1_dn4_slot = var_vgefm13g1_dn4;
        *var_vgefm13g1_dn5_slot = var_vgefm13g1_dn5;
        *var_vgefm13g1_dn6_slot = var_vgefm13g1_dn6;
        *var_vgefm13g1_dn7_slot = var_vgefm13g1_dn7;
        *var_vgefm13g1_dn8_slot = var_vgefm13g1_dn8;
        *var_vgefm13g1_dn9_slot = var_vgefm13g1_dn9;
    }

    pub(super) fn stamp_transient_block_179(
        p: &Parameters,
        var_cch: f64,
        var_ef1: f64,
        var_ef1_dn0: f64,
        var_ef1_dn1: f64,
        var_ef1_dn12: f64,
        var_ef1_dn14: f64,
        var_ef1_dn15: f64,
        var_ef1_dn16: f64,
        var_ef1_dn17: f64,
        var_ef1_dn18: f64,
        var_ef1_dn19: f64,
        var_ef1_dn2: f64,
        var_ef1_dn20: f64,
        var_ef1_dn21: f64,
        var_ef1_dn22: f64,
        var_ef1_dn3: f64,
        var_ef1_dn4: f64,
        var_ef1_dn5: f64,
        var_ef1_dn6: f64,
        var_ef1_dn7: f64,
        var_ef1_dn8: f64,
        var_ef1_dn9: f64,
        var_guard504: f64,
        var_guard513: f64,
        var_guard518: f64,
        var_t0: f64,
        var_t0_dn0: f64,
        var_t0_dn1: f64,
        var_t0_dn12: f64,
        var_t0_dn14: f64,
        var_t0_dn15: f64,
        var_t0_dn16: f64,
        var_t0_dn17: f64,
        var_t0_dn18: f64,
        var_t0_dn19: f64,
        var_t0_dn2: f64,
        var_t0_dn20: f64,
        var_t0_dn21: f64,
        var_t0_dn22: f64,
        var_t0_dn3: f64,
        var_t0_dn4: f64,
        var_t0_dn5: f64,
        var_t0_dn6: f64,
        var_t0_dn7: f64,
        var_t0_dn8: f64,
        var_t0_dn9: f64,
        var_t4: f64,
        var_t4_dn0: f64,
        var_t4_dn1: f64,
        var_t4_dn12: f64,
        var_t4_dn14: f64,
        var_t4_dn15: f64,
        var_t4_dn16: f64,
        var_t4_dn17: f64,
        var_t4_dn18: f64,
        var_t4_dn19: f64,
        var_t4_dn2: f64,
        var_t4_dn20: f64,
        var_t4_dn21: f64,
        var_t4_dn22: f64,
        var_t4_dn3: f64,
        var_t4_dn4: f64,
        var_t4_dn5: f64,
        var_t4_dn6: f64,
        var_t4_dn7: f64,
        var_t4_dn8: f64,
        var_t4_dn9: f64,
        var_t5dg0: f64,
        var_t5dg0_dn0: f64,
        var_t5dg0_dn1: f64,
        var_t5dg0_dn12: f64,
        var_t5dg0_dn14: f64,
        var_t5dg0_dn15: f64,
        var_t5dg0_dn16: f64,
        var_t5dg0_dn17: f64,
        var_t5dg0_dn18: f64,
        var_t5dg0_dn19: f64,
        var_t5dg0_dn2: f64,
        var_t5dg0_dn20: f64,
        var_t5dg0_dn21: f64,
        var_t5dg0_dn22: f64,
        var_t5dg0_dn3: f64,
        var_t5dg0_dn4: f64,
        var_t5dg0_dn5: f64,
        var_t5dg0_dn6: f64,
        var_t5dg0_dn7: f64,
        var_t5dg0_dn8: f64,
        var_t5dg0_dn9: f64,
        var_t5ng0: f64,
        var_t5ng0_dn0: f64,
        var_t5ng0_dn1: f64,
        var_t5ng0_dn12: f64,
        var_t5ng0_dn14: f64,
        var_t5ng0_dn15: f64,
        var_t5ng0_dn16: f64,
        var_t5ng0_dn17: f64,
        var_t5ng0_dn18: f64,
        var_t5ng0_dn19: f64,
        var_t5ng0_dn2: f64,
        var_t5ng0_dn20: f64,
        var_t5ng0_dn21: f64,
        var_t5ng0_dn22: f64,
        var_t5ng0_dn3: f64,
        var_t5ng0_dn4: f64,
        var_t5ng0_dn5: f64,
        var_t5ng0_dn6: f64,
        var_t5ng0_dn7: f64,
        var_t5ng0_dn8: f64,
        var_t5ng0_dn9: f64,
        var_tg1: f64,
        var_tg1_dn0: f64,
        var_tg1_dn1: f64,
        var_tg1_dn12: f64,
        var_tg1_dn14: f64,
        var_tg1_dn15: f64,
        var_tg1_dn16: f64,
        var_tg1_dn17: f64,
        var_tg1_dn18: f64,
        var_tg1_dn19: f64,
        var_tg1_dn2: f64,
        var_tg1_dn20: f64,
        var_tg1_dn21: f64,
        var_tg1_dn22: f64,
        var_tg1_dn3: f64,
        var_tg1_dn4: f64,
        var_tg1_dn5: f64,
        var_tg1_dn6: f64,
        var_tg1_dn7: f64,
        var_tg1_dn8: f64,
        var_tg1_dn9: f64,
        var_vgefm13g1: f64,
        var_vgefm13g1_dn0: f64,
        var_vgefm13g1_dn1: f64,
        var_vgefm13g1_dn12: f64,
        var_vgefm13g1_dn14: f64,
        var_vgefm13g1_dn15: f64,
        var_vgefm13g1_dn16: f64,
        var_vgefm13g1_dn17: f64,
        var_vgefm13g1_dn18: f64,
        var_vgefm13g1_dn19: f64,
        var_vgefm13g1_dn2: f64,
        var_vgefm13g1_dn20: f64,
        var_vgefm13g1_dn21: f64,
        var_vgefm13g1_dn22: f64,
        var_vgefm13g1_dn3: f64,
        var_vgefm13g1_dn4: f64,
        var_vgefm13g1_dn5: f64,
        var_vgefm13g1_dn6: f64,
        var_vgefm13g1_dn7: f64,
        var_vgefm13g1_dn8: f64,
        var_vgefm13g1_dn9: f64,
        var_vgod: f64,
        var_vgod_dn0: f64,
        var_vgod_dn1: f64,
        var_vgod_dn12: f64,
        var_vgod_dn14: f64,
        var_vgod_dn15: f64,
        var_vgod_dn16: f64,
        var_vgod_dn17: f64,
        var_vgod_dn18: f64,
        var_vgod_dn19: f64,
        var_vgod_dn2: f64,
        var_vgod_dn20: f64,
        var_vgod_dn21: f64,
        var_vgod_dn22: f64,
        var_vgod_dn3: f64,
        var_vgod_dn4: f64,
        var_vgod_dn5: f64,
        var_vgod_dn6: f64,
        var_vgod_dn7: f64,
        var_vgod_dn8: f64,
        var_vgod_dn9: f64,
        var_vtv: f64,
        var_vtv_dn15: f64,
        var_vtv_dn16: f64,
        var_vtv_dn17: f64,
        var_vtv_dn18: f64,
        var_vtv_dn19: f64,
        var_vtv_dn20: f64,
        var_vtv_dn21: f64,
        var_vtv_dn22: f64,
        var_vtv_dn4: f64,
        var_vtv_dn6: f64,
        var_vtv_dn7: f64,
        var_vtv_dn8: f64,
        var_ef2_slot: &mut f64,
        var_ef2_dn0_slot: &mut f64,
        var_ef2_dn1_slot: &mut f64,
        var_ef2_dn12_slot: &mut f64,
        var_ef2_dn14_slot: &mut f64,
        var_ef2_dn15_slot: &mut f64,
        var_ef2_dn16_slot: &mut f64,
        var_ef2_dn17_slot: &mut f64,
        var_ef2_dn18_slot: &mut f64,
        var_ef2_dn19_slot: &mut f64,
        var_ef2_dn2_slot: &mut f64,
        var_ef2_dn20_slot: &mut f64,
        var_ef2_dn21_slot: &mut f64,
        var_ef2_dn22_slot: &mut f64,
        var_ef2_dn3_slot: &mut f64,
        var_ef2_dn4_slot: &mut f64,
        var_ef2_dn5_slot: &mut f64,
        var_ef2_dn6_slot: &mut f64,
        var_ef2_dn7_slot: &mut f64,
        var_ef2_dn8_slot: &mut f64,
        var_ef2_dn9_slot: &mut f64,
        var_t42_slot: &mut f64,
        var_t42_dn0_slot: &mut f64,
        var_t42_dn1_slot: &mut f64,
        var_t42_dn12_slot: &mut f64,
        var_t42_dn14_slot: &mut f64,
        var_t42_dn15_slot: &mut f64,
        var_t42_dn16_slot: &mut f64,
        var_t42_dn17_slot: &mut f64,
        var_t42_dn18_slot: &mut f64,
        var_t42_dn19_slot: &mut f64,
        var_t42_dn2_slot: &mut f64,
        var_t42_dn20_slot: &mut f64,
        var_t42_dn21_slot: &mut f64,
        var_t42_dn22_slot: &mut f64,
        var_t42_dn3_slot: &mut f64,
        var_t42_dn4_slot: &mut f64,
        var_t42_dn5_slot: &mut f64,
        var_t42_dn6_slot: &mut f64,
        var_t42_dn7_slot: &mut f64,
        var_t42_dn8_slot: &mut f64,
        var_t42_dn9_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn1_slot: &mut f64,
        var_t5_dn12_slot: &mut f64,
        var_t5_dn14_slot: &mut f64,
        var_t5_dn15_slot: &mut f64,
        var_t5_dn16_slot: &mut f64,
        var_t5_dn17_slot: &mut f64,
        var_t5_dn18_slot: &mut f64,
        var_t5_dn19_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn20_slot: &mut f64,
        var_t5_dn21_slot: &mut f64,
        var_t5_dn22_slot: &mut f64,
        var_t5_dn3_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t5_dn9_slot: &mut f64,
        var_t5dg1_slot: &mut f64,
        var_t5dg1_dn0_slot: &mut f64,
        var_t5dg1_dn1_slot: &mut f64,
        var_t5dg1_dn12_slot: &mut f64,
        var_t5dg1_dn14_slot: &mut f64,
        var_t5dg1_dn15_slot: &mut f64,
        var_t5dg1_dn16_slot: &mut f64,
        var_t5dg1_dn17_slot: &mut f64,
        var_t5dg1_dn18_slot: &mut f64,
        var_t5dg1_dn19_slot: &mut f64,
        var_t5dg1_dn2_slot: &mut f64,
        var_t5dg1_dn20_slot: &mut f64,
        var_t5dg1_dn21_slot: &mut f64,
        var_t5dg1_dn22_slot: &mut f64,
        var_t5dg1_dn3_slot: &mut f64,
        var_t5dg1_dn4_slot: &mut f64,
        var_t5dg1_dn5_slot: &mut f64,
        var_t5dg1_dn6_slot: &mut f64,
        var_t5dg1_dn7_slot: &mut f64,
        var_t5dg1_dn8_slot: &mut f64,
        var_t5dg1_dn9_slot: &mut f64,
        var_t5ng1_slot: &mut f64,
        var_t5ng1_dn0_slot: &mut f64,
        var_t5ng1_dn1_slot: &mut f64,
        var_t5ng1_dn12_slot: &mut f64,
        var_t5ng1_dn14_slot: &mut f64,
        var_t5ng1_dn15_slot: &mut f64,
        var_t5ng1_dn16_slot: &mut f64,
        var_t5ng1_dn17_slot: &mut f64,
        var_t5ng1_dn18_slot: &mut f64,
        var_t5ng1_dn19_slot: &mut f64,
        var_t5ng1_dn2_slot: &mut f64,
        var_t5ng1_dn20_slot: &mut f64,
        var_t5ng1_dn21_slot: &mut f64,
        var_t5ng1_dn22_slot: &mut f64,
        var_t5ng1_dn3_slot: &mut f64,
        var_t5ng1_dn4_slot: &mut f64,
        var_t5ng1_dn5_slot: &mut f64,
        var_t5ng1_dn6_slot: &mut f64,
        var_t5ng1_dn7_slot: &mut f64,
        var_t5ng1_dn8_slot: &mut f64,
        var_t5ng1_dn9_slot: &mut f64,
        var_tg02_slot: &mut f64,
        var_tg02_dn0_slot: &mut f64,
        var_tg02_dn1_slot: &mut f64,
        var_tg02_dn12_slot: &mut f64,
        var_tg02_dn14_slot: &mut f64,
        var_tg02_dn15_slot: &mut f64,
        var_tg02_dn16_slot: &mut f64,
        var_tg02_dn17_slot: &mut f64,
        var_tg02_dn18_slot: &mut f64,
        var_tg02_dn19_slot: &mut f64,
        var_tg02_dn2_slot: &mut f64,
        var_tg02_dn20_slot: &mut f64,
        var_tg02_dn21_slot: &mut f64,
        var_tg02_dn22_slot: &mut f64,
        var_tg02_dn3_slot: &mut f64,
        var_tg02_dn4_slot: &mut f64,
        var_tg02_dn5_slot: &mut f64,
        var_tg02_dn6_slot: &mut f64,
        var_tg02_dn7_slot: &mut f64,
        var_tg02_dn8_slot: &mut f64,
        var_tg02_dn9_slot: &mut f64,
        var_tg12_slot: &mut f64,
        var_tg12_dn0_slot: &mut f64,
        var_tg12_dn1_slot: &mut f64,
        var_tg12_dn12_slot: &mut f64,
        var_tg12_dn14_slot: &mut f64,
        var_tg12_dn15_slot: &mut f64,
        var_tg12_dn16_slot: &mut f64,
        var_tg12_dn17_slot: &mut f64,
        var_tg12_dn18_slot: &mut f64,
        var_tg12_dn19_slot: &mut f64,
        var_tg12_dn2_slot: &mut f64,
        var_tg12_dn20_slot: &mut f64,
        var_tg12_dn21_slot: &mut f64,
        var_tg12_dn22_slot: &mut f64,
        var_tg12_dn3_slot: &mut f64,
        var_tg12_dn4_slot: &mut f64,
        var_tg12_dn5_slot: &mut f64,
        var_tg12_dn6_slot: &mut f64,
        var_tg12_dn7_slot: &mut f64,
        var_tg12_dn8_slot: &mut f64,
        var_tg12_dn9_slot: &mut f64,
        var_vgef2_slot: &mut f64,
        var_vgef223g0_slot: &mut f64,
        var_vgef223g0_dn0_slot: &mut f64,
        var_vgef223g0_dn1_slot: &mut f64,
        var_vgef223g0_dn12_slot: &mut f64,
        var_vgef223g0_dn14_slot: &mut f64,
        var_vgef223g0_dn15_slot: &mut f64,
        var_vgef223g0_dn16_slot: &mut f64,
        var_vgef223g0_dn17_slot: &mut f64,
        var_vgef223g0_dn18_slot: &mut f64,
        var_vgef223g0_dn19_slot: &mut f64,
        var_vgef223g0_dn2_slot: &mut f64,
        var_vgef223g0_dn20_slot: &mut f64,
        var_vgef223g0_dn21_slot: &mut f64,
        var_vgef223g0_dn22_slot: &mut f64,
        var_vgef223g0_dn3_slot: &mut f64,
        var_vgef223g0_dn4_slot: &mut f64,
        var_vgef223g0_dn5_slot: &mut f64,
        var_vgef223g0_dn6_slot: &mut f64,
        var_vgef223g0_dn7_slot: &mut f64,
        var_vgef223g0_dn8_slot: &mut f64,
        var_vgef223g0_dn9_slot: &mut f64,
        var_vgef223g1_slot: &mut f64,
        var_vgef223g1_dn0_slot: &mut f64,
        var_vgef223g1_dn1_slot: &mut f64,
        var_vgef223g1_dn12_slot: &mut f64,
        var_vgef223g1_dn14_slot: &mut f64,
        var_vgef223g1_dn15_slot: &mut f64,
        var_vgef223g1_dn16_slot: &mut f64,
        var_vgef223g1_dn17_slot: &mut f64,
        var_vgef223g1_dn18_slot: &mut f64,
        var_vgef223g1_dn19_slot: &mut f64,
        var_vgef223g1_dn2_slot: &mut f64,
        var_vgef223g1_dn20_slot: &mut f64,
        var_vgef223g1_dn21_slot: &mut f64,
        var_vgef223g1_dn22_slot: &mut f64,
        var_vgef223g1_dn3_slot: &mut f64,
        var_vgef223g1_dn4_slot: &mut f64,
        var_vgef223g1_dn5_slot: &mut f64,
        var_vgef223g1_dn6_slot: &mut f64,
        var_vgef223g1_dn7_slot: &mut f64,
        var_vgef223g1_dn8_slot: &mut f64,
        var_vgef223g1_dn9_slot: &mut f64,
        var_vgef2_dn0_slot: &mut f64,
        var_vgef2_dn1_slot: &mut f64,
        var_vgef2_dn12_slot: &mut f64,
        var_vgef2_dn14_slot: &mut f64,
        var_vgef2_dn15_slot: &mut f64,
        var_vgef2_dn16_slot: &mut f64,
        var_vgef2_dn17_slot: &mut f64,
        var_vgef2_dn18_slot: &mut f64,
        var_vgef2_dn19_slot: &mut f64,
        var_vgef2_dn2_slot: &mut f64,
        var_vgef2_dn20_slot: &mut f64,
        var_vgef2_dn21_slot: &mut f64,
        var_vgef2_dn22_slot: &mut f64,
        var_vgef2_dn3_slot: &mut f64,
        var_vgef2_dn4_slot: &mut f64,
        var_vgef2_dn5_slot: &mut f64,
        var_vgef2_dn6_slot: &mut f64,
        var_vgef2_dn7_slot: &mut f64,
        var_vgef2_dn8_slot: &mut f64,
        var_vgef2_dn9_slot: &mut f64,
        var_vgefm213g0_slot: &mut f64,
        var_vgefm213g0_dn0_slot: &mut f64,
        var_vgefm213g0_dn1_slot: &mut f64,
        var_vgefm213g0_dn12_slot: &mut f64,
        var_vgefm213g0_dn14_slot: &mut f64,
        var_vgefm213g0_dn15_slot: &mut f64,
        var_vgefm213g0_dn16_slot: &mut f64,
        var_vgefm213g0_dn17_slot: &mut f64,
        var_vgefm213g0_dn18_slot: &mut f64,
        var_vgefm213g0_dn19_slot: &mut f64,
        var_vgefm213g0_dn2_slot: &mut f64,
        var_vgefm213g0_dn20_slot: &mut f64,
        var_vgefm213g0_dn21_slot: &mut f64,
        var_vgefm213g0_dn22_slot: &mut f64,
        var_vgefm213g0_dn3_slot: &mut f64,
        var_vgefm213g0_dn4_slot: &mut f64,
        var_vgefm213g0_dn5_slot: &mut f64,
        var_vgefm213g0_dn6_slot: &mut f64,
        var_vgefm213g0_dn7_slot: &mut f64,
        var_vgefm213g0_dn8_slot: &mut f64,
        var_vgefm213g0_dn9_slot: &mut f64,
        var_vgefm213g1_slot: &mut f64,
        var_vgefm213g1_dn0_slot: &mut f64,
        var_vgefm213g1_dn1_slot: &mut f64,
        var_vgefm213g1_dn12_slot: &mut f64,
        var_vgefm213g1_dn14_slot: &mut f64,
        var_vgefm213g1_dn15_slot: &mut f64,
        var_vgefm213g1_dn16_slot: &mut f64,
        var_vgefm213g1_dn17_slot: &mut f64,
        var_vgefm213g1_dn18_slot: &mut f64,
        var_vgefm213g1_dn19_slot: &mut f64,
        var_vgefm213g1_dn2_slot: &mut f64,
        var_vgefm213g1_dn20_slot: &mut f64,
        var_vgefm213g1_dn21_slot: &mut f64,
        var_vgefm213g1_dn22_slot: &mut f64,
        var_vgefm213g1_dn3_slot: &mut f64,
        var_vgefm213g1_dn4_slot: &mut f64,
        var_vgefm213g1_dn5_slot: &mut f64,
        var_vgefm213g1_dn6_slot: &mut f64,
        var_vgefm213g1_dn7_slot: &mut f64,
        var_vgefm213g1_dn8_slot: &mut f64,
        var_vgefm213g1_dn9_slot: &mut f64,
    ) {
        let mut var_ef2: f64 = *var_ef2_slot;
        let mut var_ef2_dn0: f64 = *var_ef2_dn0_slot;
        let mut var_ef2_dn1: f64 = *var_ef2_dn1_slot;
        let mut var_ef2_dn12: f64 = *var_ef2_dn12_slot;
        let mut var_ef2_dn14: f64 = *var_ef2_dn14_slot;
        let mut var_ef2_dn15: f64 = *var_ef2_dn15_slot;
        let mut var_ef2_dn16: f64 = *var_ef2_dn16_slot;
        let mut var_ef2_dn17: f64 = *var_ef2_dn17_slot;
        let mut var_ef2_dn18: f64 = *var_ef2_dn18_slot;
        let mut var_ef2_dn19: f64 = *var_ef2_dn19_slot;
        let mut var_ef2_dn2: f64 = *var_ef2_dn2_slot;
        let mut var_ef2_dn20: f64 = *var_ef2_dn20_slot;
        let mut var_ef2_dn21: f64 = *var_ef2_dn21_slot;
        let mut var_ef2_dn22: f64 = *var_ef2_dn22_slot;
        let mut var_ef2_dn3: f64 = *var_ef2_dn3_slot;
        let mut var_ef2_dn4: f64 = *var_ef2_dn4_slot;
        let mut var_ef2_dn5: f64 = *var_ef2_dn5_slot;
        let mut var_ef2_dn6: f64 = *var_ef2_dn6_slot;
        let mut var_ef2_dn7: f64 = *var_ef2_dn7_slot;
        let mut var_ef2_dn8: f64 = *var_ef2_dn8_slot;
        let mut var_ef2_dn9: f64 = *var_ef2_dn9_slot;
        let mut var_t42: f64 = *var_t42_slot;
        let mut var_t42_dn0: f64 = *var_t42_dn0_slot;
        let mut var_t42_dn1: f64 = *var_t42_dn1_slot;
        let mut var_t42_dn12: f64 = *var_t42_dn12_slot;
        let mut var_t42_dn14: f64 = *var_t42_dn14_slot;
        let mut var_t42_dn15: f64 = *var_t42_dn15_slot;
        let mut var_t42_dn16: f64 = *var_t42_dn16_slot;
        let mut var_t42_dn17: f64 = *var_t42_dn17_slot;
        let mut var_t42_dn18: f64 = *var_t42_dn18_slot;
        let mut var_t42_dn19: f64 = *var_t42_dn19_slot;
        let mut var_t42_dn2: f64 = *var_t42_dn2_slot;
        let mut var_t42_dn20: f64 = *var_t42_dn20_slot;
        let mut var_t42_dn21: f64 = *var_t42_dn21_slot;
        let mut var_t42_dn22: f64 = *var_t42_dn22_slot;
        let mut var_t42_dn3: f64 = *var_t42_dn3_slot;
        let mut var_t42_dn4: f64 = *var_t42_dn4_slot;
        let mut var_t42_dn5: f64 = *var_t42_dn5_slot;
        let mut var_t42_dn6: f64 = *var_t42_dn6_slot;
        let mut var_t42_dn7: f64 = *var_t42_dn7_slot;
        let mut var_t42_dn8: f64 = *var_t42_dn8_slot;
        let mut var_t42_dn9: f64 = *var_t42_dn9_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn1: f64 = *var_t5_dn1_slot;
        let mut var_t5_dn12: f64 = *var_t5_dn12_slot;
        let mut var_t5_dn14: f64 = *var_t5_dn14_slot;
        let mut var_t5_dn15: f64 = *var_t5_dn15_slot;
        let mut var_t5_dn16: f64 = *var_t5_dn16_slot;
        let mut var_t5_dn17: f64 = *var_t5_dn17_slot;
        let mut var_t5_dn18: f64 = *var_t5_dn18_slot;
        let mut var_t5_dn19: f64 = *var_t5_dn19_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn20: f64 = *var_t5_dn20_slot;
        let mut var_t5_dn21: f64 = *var_t5_dn21_slot;
        let mut var_t5_dn22: f64 = *var_t5_dn22_slot;
        let mut var_t5_dn3: f64 = *var_t5_dn3_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t5_dn9: f64 = *var_t5_dn9_slot;
        let mut var_t5dg1: f64 = *var_t5dg1_slot;
        let mut var_t5dg1_dn0: f64 = *var_t5dg1_dn0_slot;
        let mut var_t5dg1_dn1: f64 = *var_t5dg1_dn1_slot;
        let mut var_t5dg1_dn12: f64 = *var_t5dg1_dn12_slot;
        let mut var_t5dg1_dn14: f64 = *var_t5dg1_dn14_slot;
        let mut var_t5dg1_dn15: f64 = *var_t5dg1_dn15_slot;
        let mut var_t5dg1_dn16: f64 = *var_t5dg1_dn16_slot;
        let mut var_t5dg1_dn17: f64 = *var_t5dg1_dn17_slot;
        let mut var_t5dg1_dn18: f64 = *var_t5dg1_dn18_slot;
        let mut var_t5dg1_dn19: f64 = *var_t5dg1_dn19_slot;
        let mut var_t5dg1_dn2: f64 = *var_t5dg1_dn2_slot;
        let mut var_t5dg1_dn20: f64 = *var_t5dg1_dn20_slot;
        let mut var_t5dg1_dn21: f64 = *var_t5dg1_dn21_slot;
        let mut var_t5dg1_dn22: f64 = *var_t5dg1_dn22_slot;
        let mut var_t5dg1_dn3: f64 = *var_t5dg1_dn3_slot;
        let mut var_t5dg1_dn4: f64 = *var_t5dg1_dn4_slot;
        let mut var_t5dg1_dn5: f64 = *var_t5dg1_dn5_slot;
        let mut var_t5dg1_dn6: f64 = *var_t5dg1_dn6_slot;
        let mut var_t5dg1_dn7: f64 = *var_t5dg1_dn7_slot;
        let mut var_t5dg1_dn8: f64 = *var_t5dg1_dn8_slot;
        let mut var_t5dg1_dn9: f64 = *var_t5dg1_dn9_slot;
        let mut var_t5ng1: f64 = *var_t5ng1_slot;
        let mut var_t5ng1_dn0: f64 = *var_t5ng1_dn0_slot;
        let mut var_t5ng1_dn1: f64 = *var_t5ng1_dn1_slot;
        let mut var_t5ng1_dn12: f64 = *var_t5ng1_dn12_slot;
        let mut var_t5ng1_dn14: f64 = *var_t5ng1_dn14_slot;
        let mut var_t5ng1_dn15: f64 = *var_t5ng1_dn15_slot;
        let mut var_t5ng1_dn16: f64 = *var_t5ng1_dn16_slot;
        let mut var_t5ng1_dn17: f64 = *var_t5ng1_dn17_slot;
        let mut var_t5ng1_dn18: f64 = *var_t5ng1_dn18_slot;
        let mut var_t5ng1_dn19: f64 = *var_t5ng1_dn19_slot;
        let mut var_t5ng1_dn2: f64 = *var_t5ng1_dn2_slot;
        let mut var_t5ng1_dn20: f64 = *var_t5ng1_dn20_slot;
        let mut var_t5ng1_dn21: f64 = *var_t5ng1_dn21_slot;
        let mut var_t5ng1_dn22: f64 = *var_t5ng1_dn22_slot;
        let mut var_t5ng1_dn3: f64 = *var_t5ng1_dn3_slot;
        let mut var_t5ng1_dn4: f64 = *var_t5ng1_dn4_slot;
        let mut var_t5ng1_dn5: f64 = *var_t5ng1_dn5_slot;
        let mut var_t5ng1_dn6: f64 = *var_t5ng1_dn6_slot;
        let mut var_t5ng1_dn7: f64 = *var_t5ng1_dn7_slot;
        let mut var_t5ng1_dn8: f64 = *var_t5ng1_dn8_slot;
        let mut var_t5ng1_dn9: f64 = *var_t5ng1_dn9_slot;
        let mut var_tg02: f64 = *var_tg02_slot;
        let mut var_tg02_dn0: f64 = *var_tg02_dn0_slot;
        let mut var_tg02_dn1: f64 = *var_tg02_dn1_slot;
        let mut var_tg02_dn12: f64 = *var_tg02_dn12_slot;
        let mut var_tg02_dn14: f64 = *var_tg02_dn14_slot;
        let mut var_tg02_dn15: f64 = *var_tg02_dn15_slot;
        let mut var_tg02_dn16: f64 = *var_tg02_dn16_slot;
        let mut var_tg02_dn17: f64 = *var_tg02_dn17_slot;
        let mut var_tg02_dn18: f64 = *var_tg02_dn18_slot;
        let mut var_tg02_dn19: f64 = *var_tg02_dn19_slot;
        let mut var_tg02_dn2: f64 = *var_tg02_dn2_slot;
        let mut var_tg02_dn20: f64 = *var_tg02_dn20_slot;
        let mut var_tg02_dn21: f64 = *var_tg02_dn21_slot;
        let mut var_tg02_dn22: f64 = *var_tg02_dn22_slot;
        let mut var_tg02_dn3: f64 = *var_tg02_dn3_slot;
        let mut var_tg02_dn4: f64 = *var_tg02_dn4_slot;
        let mut var_tg02_dn5: f64 = *var_tg02_dn5_slot;
        let mut var_tg02_dn6: f64 = *var_tg02_dn6_slot;
        let mut var_tg02_dn7: f64 = *var_tg02_dn7_slot;
        let mut var_tg02_dn8: f64 = *var_tg02_dn8_slot;
        let mut var_tg02_dn9: f64 = *var_tg02_dn9_slot;
        let mut var_tg12: f64 = *var_tg12_slot;
        let mut var_tg12_dn0: f64 = *var_tg12_dn0_slot;
        let mut var_tg12_dn1: f64 = *var_tg12_dn1_slot;
        let mut var_tg12_dn12: f64 = *var_tg12_dn12_slot;
        let mut var_tg12_dn14: f64 = *var_tg12_dn14_slot;
        let mut var_tg12_dn15: f64 = *var_tg12_dn15_slot;
        let mut var_tg12_dn16: f64 = *var_tg12_dn16_slot;
        let mut var_tg12_dn17: f64 = *var_tg12_dn17_slot;
        let mut var_tg12_dn18: f64 = *var_tg12_dn18_slot;
        let mut var_tg12_dn19: f64 = *var_tg12_dn19_slot;
        let mut var_tg12_dn2: f64 = *var_tg12_dn2_slot;
        let mut var_tg12_dn20: f64 = *var_tg12_dn20_slot;
        let mut var_tg12_dn21: f64 = *var_tg12_dn21_slot;
        let mut var_tg12_dn22: f64 = *var_tg12_dn22_slot;
        let mut var_tg12_dn3: f64 = *var_tg12_dn3_slot;
        let mut var_tg12_dn4: f64 = *var_tg12_dn4_slot;
        let mut var_tg12_dn5: f64 = *var_tg12_dn5_slot;
        let mut var_tg12_dn6: f64 = *var_tg12_dn6_slot;
        let mut var_tg12_dn7: f64 = *var_tg12_dn7_slot;
        let mut var_tg12_dn8: f64 = *var_tg12_dn8_slot;
        let mut var_tg12_dn9: f64 = *var_tg12_dn9_slot;
        let mut var_vgef2: f64 = *var_vgef2_slot;
        let mut var_vgef223g0: f64 = *var_vgef223g0_slot;
        let mut var_vgef223g0_dn0: f64 = *var_vgef223g0_dn0_slot;
        let mut var_vgef223g0_dn1: f64 = *var_vgef223g0_dn1_slot;
        let mut var_vgef223g0_dn12: f64 = *var_vgef223g0_dn12_slot;
        let mut var_vgef223g0_dn14: f64 = *var_vgef223g0_dn14_slot;
        let mut var_vgef223g0_dn15: f64 = *var_vgef223g0_dn15_slot;
        let mut var_vgef223g0_dn16: f64 = *var_vgef223g0_dn16_slot;
        let mut var_vgef223g0_dn17: f64 = *var_vgef223g0_dn17_slot;
        let mut var_vgef223g0_dn18: f64 = *var_vgef223g0_dn18_slot;
        let mut var_vgef223g0_dn19: f64 = *var_vgef223g0_dn19_slot;
        let mut var_vgef223g0_dn2: f64 = *var_vgef223g0_dn2_slot;
        let mut var_vgef223g0_dn20: f64 = *var_vgef223g0_dn20_slot;
        let mut var_vgef223g0_dn21: f64 = *var_vgef223g0_dn21_slot;
        let mut var_vgef223g0_dn22: f64 = *var_vgef223g0_dn22_slot;
        let mut var_vgef223g0_dn3: f64 = *var_vgef223g0_dn3_slot;
        let mut var_vgef223g0_dn4: f64 = *var_vgef223g0_dn4_slot;
        let mut var_vgef223g0_dn5: f64 = *var_vgef223g0_dn5_slot;
        let mut var_vgef223g0_dn6: f64 = *var_vgef223g0_dn6_slot;
        let mut var_vgef223g0_dn7: f64 = *var_vgef223g0_dn7_slot;
        let mut var_vgef223g0_dn8: f64 = *var_vgef223g0_dn8_slot;
        let mut var_vgef223g0_dn9: f64 = *var_vgef223g0_dn9_slot;
        let mut var_vgef223g1: f64 = *var_vgef223g1_slot;
        let mut var_vgef223g1_dn0: f64 = *var_vgef223g1_dn0_slot;
        let mut var_vgef223g1_dn1: f64 = *var_vgef223g1_dn1_slot;
        let mut var_vgef223g1_dn12: f64 = *var_vgef223g1_dn12_slot;
        let mut var_vgef223g1_dn14: f64 = *var_vgef223g1_dn14_slot;
        let mut var_vgef223g1_dn15: f64 = *var_vgef223g1_dn15_slot;
        let mut var_vgef223g1_dn16: f64 = *var_vgef223g1_dn16_slot;
        let mut var_vgef223g1_dn17: f64 = *var_vgef223g1_dn17_slot;
        let mut var_vgef223g1_dn18: f64 = *var_vgef223g1_dn18_slot;
        let mut var_vgef223g1_dn19: f64 = *var_vgef223g1_dn19_slot;
        let mut var_vgef223g1_dn2: f64 = *var_vgef223g1_dn2_slot;
        let mut var_vgef223g1_dn20: f64 = *var_vgef223g1_dn20_slot;
        let mut var_vgef223g1_dn21: f64 = *var_vgef223g1_dn21_slot;
        let mut var_vgef223g1_dn22: f64 = *var_vgef223g1_dn22_slot;
        let mut var_vgef223g1_dn3: f64 = *var_vgef223g1_dn3_slot;
        let mut var_vgef223g1_dn4: f64 = *var_vgef223g1_dn4_slot;
        let mut var_vgef223g1_dn5: f64 = *var_vgef223g1_dn5_slot;
        let mut var_vgef223g1_dn6: f64 = *var_vgef223g1_dn6_slot;
        let mut var_vgef223g1_dn7: f64 = *var_vgef223g1_dn7_slot;
        let mut var_vgef223g1_dn8: f64 = *var_vgef223g1_dn8_slot;
        let mut var_vgef223g1_dn9: f64 = *var_vgef223g1_dn9_slot;
        let mut var_vgef2_dn0: f64 = *var_vgef2_dn0_slot;
        let mut var_vgef2_dn1: f64 = *var_vgef2_dn1_slot;
        let mut var_vgef2_dn12: f64 = *var_vgef2_dn12_slot;
        let mut var_vgef2_dn14: f64 = *var_vgef2_dn14_slot;
        let mut var_vgef2_dn15: f64 = *var_vgef2_dn15_slot;
        let mut var_vgef2_dn16: f64 = *var_vgef2_dn16_slot;
        let mut var_vgef2_dn17: f64 = *var_vgef2_dn17_slot;
        let mut var_vgef2_dn18: f64 = *var_vgef2_dn18_slot;
        let mut var_vgef2_dn19: f64 = *var_vgef2_dn19_slot;
        let mut var_vgef2_dn2: f64 = *var_vgef2_dn2_slot;
        let mut var_vgef2_dn20: f64 = *var_vgef2_dn20_slot;
        let mut var_vgef2_dn21: f64 = *var_vgef2_dn21_slot;
        let mut var_vgef2_dn22: f64 = *var_vgef2_dn22_slot;
        let mut var_vgef2_dn3: f64 = *var_vgef2_dn3_slot;
        let mut var_vgef2_dn4: f64 = *var_vgef2_dn4_slot;
        let mut var_vgef2_dn5: f64 = *var_vgef2_dn5_slot;
        let mut var_vgef2_dn6: f64 = *var_vgef2_dn6_slot;
        let mut var_vgef2_dn7: f64 = *var_vgef2_dn7_slot;
        let mut var_vgef2_dn8: f64 = *var_vgef2_dn8_slot;
        let mut var_vgef2_dn9: f64 = *var_vgef2_dn9_slot;
        let mut var_vgefm213g0: f64 = *var_vgefm213g0_slot;
        let mut var_vgefm213g0_dn0: f64 = *var_vgefm213g0_dn0_slot;
        let mut var_vgefm213g0_dn1: f64 = *var_vgefm213g0_dn1_slot;
        let mut var_vgefm213g0_dn12: f64 = *var_vgefm213g0_dn12_slot;
        let mut var_vgefm213g0_dn14: f64 = *var_vgefm213g0_dn14_slot;
        let mut var_vgefm213g0_dn15: f64 = *var_vgefm213g0_dn15_slot;
        let mut var_vgefm213g0_dn16: f64 = *var_vgefm213g0_dn16_slot;
        let mut var_vgefm213g0_dn17: f64 = *var_vgefm213g0_dn17_slot;
        let mut var_vgefm213g0_dn18: f64 = *var_vgefm213g0_dn18_slot;
        let mut var_vgefm213g0_dn19: f64 = *var_vgefm213g0_dn19_slot;
        let mut var_vgefm213g0_dn2: f64 = *var_vgefm213g0_dn2_slot;
        let mut var_vgefm213g0_dn20: f64 = *var_vgefm213g0_dn20_slot;
        let mut var_vgefm213g0_dn21: f64 = *var_vgefm213g0_dn21_slot;
        let mut var_vgefm213g0_dn22: f64 = *var_vgefm213g0_dn22_slot;
        let mut var_vgefm213g0_dn3: f64 = *var_vgefm213g0_dn3_slot;
        let mut var_vgefm213g0_dn4: f64 = *var_vgefm213g0_dn4_slot;
        let mut var_vgefm213g0_dn5: f64 = *var_vgefm213g0_dn5_slot;
        let mut var_vgefm213g0_dn6: f64 = *var_vgefm213g0_dn6_slot;
        let mut var_vgefm213g0_dn7: f64 = *var_vgefm213g0_dn7_slot;
        let mut var_vgefm213g0_dn8: f64 = *var_vgefm213g0_dn8_slot;
        let mut var_vgefm213g0_dn9: f64 = *var_vgefm213g0_dn9_slot;
        let mut var_vgefm213g1: f64 = *var_vgefm213g1_slot;
        let mut var_vgefm213g1_dn0: f64 = *var_vgefm213g1_dn0_slot;
        let mut var_vgefm213g1_dn1: f64 = *var_vgefm213g1_dn1_slot;
        let mut var_vgefm213g1_dn12: f64 = *var_vgefm213g1_dn12_slot;
        let mut var_vgefm213g1_dn14: f64 = *var_vgefm213g1_dn14_slot;
        let mut var_vgefm213g1_dn15: f64 = *var_vgefm213g1_dn15_slot;
        let mut var_vgefm213g1_dn16: f64 = *var_vgefm213g1_dn16_slot;
        let mut var_vgefm213g1_dn17: f64 = *var_vgefm213g1_dn17_slot;
        let mut var_vgefm213g1_dn18: f64 = *var_vgefm213g1_dn18_slot;
        let mut var_vgefm213g1_dn19: f64 = *var_vgefm213g1_dn19_slot;
        let mut var_vgefm213g1_dn2: f64 = *var_vgefm213g1_dn2_slot;
        let mut var_vgefm213g1_dn20: f64 = *var_vgefm213g1_dn20_slot;
        let mut var_vgefm213g1_dn21: f64 = *var_vgefm213g1_dn21_slot;
        let mut var_vgefm213g1_dn22: f64 = *var_vgefm213g1_dn22_slot;
        let mut var_vgefm213g1_dn3: f64 = *var_vgefm213g1_dn3_slot;
        let mut var_vgefm213g1_dn4: f64 = *var_vgefm213g1_dn4_slot;
        let mut var_vgefm213g1_dn5: f64 = *var_vgefm213g1_dn5_slot;
        let mut var_vgefm213g1_dn6: f64 = *var_vgefm213g1_dn6_slot;
        let mut var_vgefm213g1_dn7: f64 = *var_vgefm213g1_dn7_slot;
        let mut var_vgefm213g1_dn8: f64 = *var_vgefm213g1_dn8_slot;
        let mut var_vgefm213g1_dn9: f64 = *var_vgefm213g1_dn9_slot;

        let (assign30470_e48127, assign30470_e48127_d_n0, assign30470_e48127_d_n1, assign30470_e48127_d_n2, assign30470_e48127_d_n3, assign30470_e48127_d_n4, assign30470_e48127_d_n5, assign30470_e48127_d_n6, assign30470_e48127_d_n7, assign30470_e48127_d_n8, assign30470_e48127_d_n9, assign30470_e48127_d_n12, assign30470_e48127_d_n14, assign30470_e48127_d_n15, assign30470_e48127_d_n16, assign30470_e48127_d_n17, assign30470_e48127_d_n18, assign30470_e48127_d_n19, assign30470_e48127_d_n20, assign30470_e48127_d_n21, assign30470_e48127_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let assign30470_e48117: f64 = { let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign30470_e48119: f64 = (assign30470_e48117 * 3.24e17);
        let assign30470_e48123: f64 = (0.6666666666666666 * var_vgefm13g1);
        let assign30470_e48124: f64 = (1.0 + assign30470_e48123);
        let assign30470_e48125: f64 = (assign30470_e48119 * assign30470_e48124);
        (assign30470_e48125, (((({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn0) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * var_vgefm13g1_dn0))), (((({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn1) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * var_vgefm13g1_dn1))), (((({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn2) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * var_vgefm13g1_dn2))), (((({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn3) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * var_vgefm13g1_dn3))), (((({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn4) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * var_vgefm13g1_dn4))), (((({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn5) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * var_vgefm13g1_dn5))), (((({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn6) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * var_vgefm13g1_dn6))), (((({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn7) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * var_vgefm13g1_dn7))), (((({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn8) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * var_vgefm13g1_dn8))), (((({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn9) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * var_vgefm13g1_dn9))), (((({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn12) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * var_vgefm13g1_dn12))), (((({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn14) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * var_vgefm13g1_dn14))), (((({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn15) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * var_vgefm13g1_dn15))), (((({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn16) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * var_vgefm13g1_dn16))), (((({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn17) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * var_vgefm13g1_dn17))), (((({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn18) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * var_vgefm13g1_dn18))), (((({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn19) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * var_vgefm13g1_dn19))), (((({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn20) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * var_vgefm13g1_dn20))), (((({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn21) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * var_vgefm13g1_dn21))), (((({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn22) * 3.24e17) * assign30470_e48124) + (assign30470_e48119 * (0.6666666666666666 * var_vgefm13g1_dn22))),)
    } else {
        (var_t5ng1, var_t5ng1_dn0, var_t5ng1_dn1, var_t5ng1_dn2, var_t5ng1_dn3, var_t5ng1_dn4, var_t5ng1_dn5, var_t5ng1_dn6, var_t5ng1_dn7, var_t5ng1_dn8, var_t5ng1_dn9, var_t5ng1_dn12, var_t5ng1_dn14, var_t5ng1_dn15, var_t5ng1_dn16, var_t5ng1_dn17, var_t5ng1_dn18, var_t5ng1_dn19, var_t5ng1_dn20, var_t5ng1_dn21, var_t5ng1_dn22,)
    }
};
        var_t5ng1 = assign30470_e48127;
        var_t5ng1_dn0 = assign30470_e48127_d_n0;
        var_t5ng1_dn1 = assign30470_e48127_d_n1;
        var_t5ng1_dn2 = assign30470_e48127_d_n2;
        var_t5ng1_dn3 = assign30470_e48127_d_n3;
        var_t5ng1_dn4 = assign30470_e48127_d_n4;
        var_t5ng1_dn5 = assign30470_e48127_d_n5;
        var_t5ng1_dn6 = assign30470_e48127_d_n6;
        var_t5ng1_dn7 = assign30470_e48127_d_n7;
        var_t5ng1_dn8 = assign30470_e48127_d_n8;
        var_t5ng1_dn9 = assign30470_e48127_d_n9;
        var_t5ng1_dn12 = assign30470_e48127_d_n12;
        var_t5ng1_dn14 = assign30470_e48127_d_n14;
        var_t5ng1_dn15 = assign30470_e48127_d_n15;
        var_t5ng1_dn16 = assign30470_e48127_d_n16;
        var_t5ng1_dn17 = assign30470_e48127_d_n17;
        var_t5ng1_dn18 = assign30470_e48127_d_n18;
        var_t5ng1_dn19 = assign30470_e48127_d_n19;
        var_t5ng1_dn20 = assign30470_e48127_d_n20;
        var_t5ng1_dn21 = assign30470_e48127_d_n21;
        var_t5ng1_dn22 = assign30470_e48127_d_n22;

        let (assign30480_e48139, assign30480_e48139_d_n0, assign30480_e48139_d_n1, assign30480_e48139_d_n2, assign30480_e48139_d_n3, assign30480_e48139_d_n4, assign30480_e48139_d_n5, assign30480_e48139_d_n6, assign30480_e48139_d_n7, assign30480_e48139_d_n8, assign30480_e48139_d_n9, assign30480_e48139_d_n12, assign30480_e48139_d_n14, assign30480_e48139_d_n15, assign30480_e48139_d_n16, assign30480_e48139_d_n17, assign30480_e48139_d_n18, assign30480_e48139_d_n19, assign30480_e48139_d_n20, assign30480_e48139_d_n21, assign30480_e48139_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let assign30480_e48136: f64 = { let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign30480_e48137: f64 = (1.0 + assign30480_e48136);
        (assign30480_e48137, ({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn0), ({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn1), ({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn2), ({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn3), ({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn4), ({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn5), ({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn6), ({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn7), ({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn8), ({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn9), ({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn12), ({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn14), ({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn15), ({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn16), ({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn17), ({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn18), ({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn19), ({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn20), ({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn21), ({ let limited_exp_arg = var_tg1; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg1_dn22),)
    } else {
        (var_t5dg1, var_t5dg1_dn0, var_t5dg1_dn1, var_t5dg1_dn2, var_t5dg1_dn3, var_t5dg1_dn4, var_t5dg1_dn5, var_t5dg1_dn6, var_t5dg1_dn7, var_t5dg1_dn8, var_t5dg1_dn9, var_t5dg1_dn12, var_t5dg1_dn14, var_t5dg1_dn15, var_t5dg1_dn16, var_t5dg1_dn17, var_t5dg1_dn18, var_t5dg1_dn19, var_t5dg1_dn20, var_t5dg1_dn21, var_t5dg1_dn22,)
    }
};
        var_t5dg1 = assign30480_e48139;
        var_t5dg1_dn0 = assign30480_e48139_d_n0;
        var_t5dg1_dn1 = assign30480_e48139_d_n1;
        var_t5dg1_dn2 = assign30480_e48139_d_n2;
        var_t5dg1_dn3 = assign30480_e48139_d_n3;
        var_t5dg1_dn4 = assign30480_e48139_d_n4;
        var_t5dg1_dn5 = assign30480_e48139_d_n5;
        var_t5dg1_dn6 = assign30480_e48139_d_n6;
        var_t5dg1_dn7 = assign30480_e48139_d_n7;
        var_t5dg1_dn8 = assign30480_e48139_d_n8;
        var_t5dg1_dn9 = assign30480_e48139_d_n9;
        var_t5dg1_dn12 = assign30480_e48139_d_n12;
        var_t5dg1_dn14 = assign30480_e48139_d_n14;
        var_t5dg1_dn15 = assign30480_e48139_d_n15;
        var_t5dg1_dn16 = assign30480_e48139_d_n16;
        var_t5dg1_dn17 = assign30480_e48139_d_n17;
        var_t5dg1_dn18 = assign30480_e48139_d_n18;
        var_t5dg1_dn19 = assign30480_e48139_d_n19;
        var_t5dg1_dn20 = assign30480_e48139_d_n20;
        var_t5dg1_dn21 = assign30480_e48139_d_n21;
        var_t5dg1_dn22 = assign30480_e48139_d_n22;

        let (assign30490_e48159, assign30490_e48159_d_n0, assign30490_e48159_d_n1, assign30490_e48159_d_n2, assign30490_e48159_d_n3, assign30490_e48159_d_n4, assign30490_e48159_d_n5, assign30490_e48159_d_n6, assign30490_e48159_d_n7, assign30490_e48159_d_n8, assign30490_e48159_d_n9, assign30490_e48159_d_n12, assign30490_e48159_d_n14, assign30490_e48159_d_n15, assign30490_e48159_d_n16, assign30490_e48159_d_n17, assign30490_e48159_d_n18, assign30490_e48159_d_n19, assign30490_e48159_d_n20, assign30490_e48159_d_n21, assign30490_e48159_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let assign30490_e48147: f64 = (-1.0);
        let assign30490_e48149: f64 = (assign30490_e48147 * var_cch);
        let assign30490_e48152: f64 = (var_t5ng0 / var_t5dg0);
        let assign30490_e48153: f64 = (assign30490_e48149 - assign30490_e48152);
        let assign30490_e48156: f64 = (var_t5ng1 / var_t5dg1);
        let assign30490_e48157: f64 = (assign30490_e48153 - assign30490_e48156);
        (assign30490_e48157, ((-(((var_t5ng0_dn0 * var_t5dg0) - (var_t5ng0 * var_t5dg0_dn0)) / (var_t5dg0 * var_t5dg0))) - (((var_t5ng1_dn0 * var_t5dg1) - (var_t5ng1 * var_t5dg1_dn0)) / (var_t5dg1 * var_t5dg1))), ((-(((var_t5ng0_dn1 * var_t5dg0) - (var_t5ng0 * var_t5dg0_dn1)) / (var_t5dg0 * var_t5dg0))) - (((var_t5ng1_dn1 * var_t5dg1) - (var_t5ng1 * var_t5dg1_dn1)) / (var_t5dg1 * var_t5dg1))), ((-(((var_t5ng0_dn2 * var_t5dg0) - (var_t5ng0 * var_t5dg0_dn2)) / (var_t5dg0 * var_t5dg0))) - (((var_t5ng1_dn2 * var_t5dg1) - (var_t5ng1 * var_t5dg1_dn2)) / (var_t5dg1 * var_t5dg1))), ((-(((var_t5ng0_dn3 * var_t5dg0) - (var_t5ng0 * var_t5dg0_dn3)) / (var_t5dg0 * var_t5dg0))) - (((var_t5ng1_dn3 * var_t5dg1) - (var_t5ng1 * var_t5dg1_dn3)) / (var_t5dg1 * var_t5dg1))), ((-(((var_t5ng0_dn4 * var_t5dg0) - (var_t5ng0 * var_t5dg0_dn4)) / (var_t5dg0 * var_t5dg0))) - (((var_t5ng1_dn4 * var_t5dg1) - (var_t5ng1 * var_t5dg1_dn4)) / (var_t5dg1 * var_t5dg1))), ((-(((var_t5ng0_dn5 * var_t5dg0) - (var_t5ng0 * var_t5dg0_dn5)) / (var_t5dg0 * var_t5dg0))) - (((var_t5ng1_dn5 * var_t5dg1) - (var_t5ng1 * var_t5dg1_dn5)) / (var_t5dg1 * var_t5dg1))), ((-(((var_t5ng0_dn6 * var_t5dg0) - (var_t5ng0 * var_t5dg0_dn6)) / (var_t5dg0 * var_t5dg0))) - (((var_t5ng1_dn6 * var_t5dg1) - (var_t5ng1 * var_t5dg1_dn6)) / (var_t5dg1 * var_t5dg1))), ((-(((var_t5ng0_dn7 * var_t5dg0) - (var_t5ng0 * var_t5dg0_dn7)) / (var_t5dg0 * var_t5dg0))) - (((var_t5ng1_dn7 * var_t5dg1) - (var_t5ng1 * var_t5dg1_dn7)) / (var_t5dg1 * var_t5dg1))), ((-(((var_t5ng0_dn8 * var_t5dg0) - (var_t5ng0 * var_t5dg0_dn8)) / (var_t5dg0 * var_t5dg0))) - (((var_t5ng1_dn8 * var_t5dg1) - (var_t5ng1 * var_t5dg1_dn8)) / (var_t5dg1 * var_t5dg1))), ((-(((var_t5ng0_dn9 * var_t5dg0) - (var_t5ng0 * var_t5dg0_dn9)) / (var_t5dg0 * var_t5dg0))) - (((var_t5ng1_dn9 * var_t5dg1) - (var_t5ng1 * var_t5dg1_dn9)) / (var_t5dg1 * var_t5dg1))), ((-(((var_t5ng0_dn12 * var_t5dg0) - (var_t5ng0 * var_t5dg0_dn12)) / (var_t5dg0 * var_t5dg0))) - (((var_t5ng1_dn12 * var_t5dg1) - (var_t5ng1 * var_t5dg1_dn12)) / (var_t5dg1 * var_t5dg1))), ((-(((var_t5ng0_dn14 * var_t5dg0) - (var_t5ng0 * var_t5dg0_dn14)) / (var_t5dg0 * var_t5dg0))) - (((var_t5ng1_dn14 * var_t5dg1) - (var_t5ng1 * var_t5dg1_dn14)) / (var_t5dg1 * var_t5dg1))), ((-(((var_t5ng0_dn15 * var_t5dg0) - (var_t5ng0 * var_t5dg0_dn15)) / (var_t5dg0 * var_t5dg0))) - (((var_t5ng1_dn15 * var_t5dg1) - (var_t5ng1 * var_t5dg1_dn15)) / (var_t5dg1 * var_t5dg1))), ((-(((var_t5ng0_dn16 * var_t5dg0) - (var_t5ng0 * var_t5dg0_dn16)) / (var_t5dg0 * var_t5dg0))) - (((var_t5ng1_dn16 * var_t5dg1) - (var_t5ng1 * var_t5dg1_dn16)) / (var_t5dg1 * var_t5dg1))), ((-(((var_t5ng0_dn17 * var_t5dg0) - (var_t5ng0 * var_t5dg0_dn17)) / (var_t5dg0 * var_t5dg0))) - (((var_t5ng1_dn17 * var_t5dg1) - (var_t5ng1 * var_t5dg1_dn17)) / (var_t5dg1 * var_t5dg1))), ((-(((var_t5ng0_dn18 * var_t5dg0) - (var_t5ng0 * var_t5dg0_dn18)) / (var_t5dg0 * var_t5dg0))) - (((var_t5ng1_dn18 * var_t5dg1) - (var_t5ng1 * var_t5dg1_dn18)) / (var_t5dg1 * var_t5dg1))), ((-(((var_t5ng0_dn19 * var_t5dg0) - (var_t5ng0 * var_t5dg0_dn19)) / (var_t5dg0 * var_t5dg0))) - (((var_t5ng1_dn19 * var_t5dg1) - (var_t5ng1 * var_t5dg1_dn19)) / (var_t5dg1 * var_t5dg1))), ((-(((var_t5ng0_dn20 * var_t5dg0) - (var_t5ng0 * var_t5dg0_dn20)) / (var_t5dg0 * var_t5dg0))) - (((var_t5ng1_dn20 * var_t5dg1) - (var_t5ng1 * var_t5dg1_dn20)) / (var_t5dg1 * var_t5dg1))), ((-(((var_t5ng0_dn21 * var_t5dg0) - (var_t5ng0 * var_t5dg0_dn21)) / (var_t5dg0 * var_t5dg0))) - (((var_t5ng1_dn21 * var_t5dg1) - (var_t5ng1 * var_t5dg1_dn21)) / (var_t5dg1 * var_t5dg1))), ((-(((var_t5ng0_dn22 * var_t5dg0) - (var_t5ng0 * var_t5dg0_dn22)) / (var_t5dg0 * var_t5dg0))) - (((var_t5ng1_dn22 * var_t5dg1) - (var_t5ng1 * var_t5dg1_dn22)) / (var_t5dg1 * var_t5dg1))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn1, var_t5_dn2, var_t5_dn3, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn12, var_t5_dn14, var_t5_dn15, var_t5_dn16, var_t5_dn17, var_t5_dn18, var_t5_dn19, var_t5_dn20, var_t5_dn21, var_t5_dn22,)
    }
};
        var_t5 = assign30490_e48159;
        var_t5_dn0 = assign30490_e48159_d_n0;
        var_t5_dn1 = assign30490_e48159_d_n1;
        var_t5_dn2 = assign30490_e48159_d_n2;
        var_t5_dn3 = assign30490_e48159_d_n3;
        var_t5_dn4 = assign30490_e48159_d_n4;
        var_t5_dn5 = assign30490_e48159_d_n5;
        var_t5_dn6 = assign30490_e48159_d_n6;
        var_t5_dn7 = assign30490_e48159_d_n7;
        var_t5_dn8 = assign30490_e48159_d_n8;
        var_t5_dn9 = assign30490_e48159_d_n9;
        var_t5_dn12 = assign30490_e48159_d_n12;
        var_t5_dn14 = assign30490_e48159_d_n14;
        var_t5_dn15 = assign30490_e48159_d_n15;
        var_t5_dn16 = assign30490_e48159_d_n16;
        var_t5_dn17 = assign30490_e48159_d_n17;
        var_t5_dn18 = assign30490_e48159_d_n18;
        var_t5_dn19 = assign30490_e48159_d_n19;
        var_t5_dn20 = assign30490_e48159_d_n20;
        var_t5_dn21 = assign30490_e48159_d_n21;
        var_t5_dn22 = assign30490_e48159_d_n22;

        let (assign30500_e48172, assign30500_e48172_d_n0, assign30500_e48172_d_n1, assign30500_e48172_d_n2, assign30500_e48172_d_n3, assign30500_e48172_d_n4, assign30500_e48172_d_n5, assign30500_e48172_d_n6, assign30500_e48172_d_n7, assign30500_e48172_d_n8, assign30500_e48172_d_n9, assign30500_e48172_d_n12, assign30500_e48172_d_n14, assign30500_e48172_d_n15, assign30500_e48172_d_n16, assign30500_e48172_d_n17, assign30500_e48172_d_n18, assign30500_e48172_d_n19, assign30500_e48172_d_n20, assign30500_e48172_d_n21, assign30500_e48172_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let assign30500_e48169: f64 = (var_t4 / var_t5);
        let assign30500_e48170: f64 = (var_ef1 - assign30500_e48169);
        (assign30500_e48170, (var_ef1_dn0 - (((var_t4_dn0 * var_t5) - (var_t4 * var_t5_dn0)) / (var_t5 * var_t5))), (var_ef1_dn1 - (((var_t4_dn1 * var_t5) - (var_t4 * var_t5_dn1)) / (var_t5 * var_t5))), (var_ef1_dn2 - (((var_t4_dn2 * var_t5) - (var_t4 * var_t5_dn2)) / (var_t5 * var_t5))), (var_ef1_dn3 - (((var_t4_dn3 * var_t5) - (var_t4 * var_t5_dn3)) / (var_t5 * var_t5))), (var_ef1_dn4 - (((var_t4_dn4 * var_t5) - (var_t4 * var_t5_dn4)) / (var_t5 * var_t5))), (var_ef1_dn5 - (((var_t4_dn5 * var_t5) - (var_t4 * var_t5_dn5)) / (var_t5 * var_t5))), (var_ef1_dn6 - (((var_t4_dn6 * var_t5) - (var_t4 * var_t5_dn6)) / (var_t5 * var_t5))), (var_ef1_dn7 - (((var_t4_dn7 * var_t5) - (var_t4 * var_t5_dn7)) / (var_t5 * var_t5))), (var_ef1_dn8 - (((var_t4_dn8 * var_t5) - (var_t4 * var_t5_dn8)) / (var_t5 * var_t5))), (var_ef1_dn9 - (((var_t4_dn9 * var_t5) - (var_t4 * var_t5_dn9)) / (var_t5 * var_t5))), (var_ef1_dn12 - (((var_t4_dn12 * var_t5) - (var_t4 * var_t5_dn12)) / (var_t5 * var_t5))), (var_ef1_dn14 - (((var_t4_dn14 * var_t5) - (var_t4 * var_t5_dn14)) / (var_t5 * var_t5))), (var_ef1_dn15 - (((var_t4_dn15 * var_t5) - (var_t4 * var_t5_dn15)) / (var_t5 * var_t5))), (var_ef1_dn16 - (((var_t4_dn16 * var_t5) - (var_t4 * var_t5_dn16)) / (var_t5 * var_t5))), (var_ef1_dn17 - (((var_t4_dn17 * var_t5) - (var_t4 * var_t5_dn17)) / (var_t5 * var_t5))), (var_ef1_dn18 - (((var_t4_dn18 * var_t5) - (var_t4 * var_t5_dn18)) / (var_t5 * var_t5))), (var_ef1_dn19 - (((var_t4_dn19 * var_t5) - (var_t4 * var_t5_dn19)) / (var_t5 * var_t5))), (var_ef1_dn20 - (((var_t4_dn20 * var_t5) - (var_t4 * var_t5_dn20)) / (var_t5 * var_t5))), (var_ef1_dn21 - (((var_t4_dn21 * var_t5) - (var_t4 * var_t5_dn21)) / (var_t5 * var_t5))), (var_ef1_dn22 - (((var_t4_dn22 * var_t5) - (var_t4 * var_t5_dn22)) / (var_t5 * var_t5))),)
    } else {
        (var_ef2, var_ef2_dn0, var_ef2_dn1, var_ef2_dn2, var_ef2_dn3, var_ef2_dn4, var_ef2_dn5, var_ef2_dn6, var_ef2_dn7, var_ef2_dn8, var_ef2_dn9, var_ef2_dn12, var_ef2_dn14, var_ef2_dn15, var_ef2_dn16, var_ef2_dn17, var_ef2_dn18, var_ef2_dn19, var_ef2_dn20, var_ef2_dn21, var_ef2_dn22,)
    }
};
        var_ef2 = assign30500_e48172;
        var_ef2_dn0 = assign30500_e48172_d_n0;
        var_ef2_dn1 = assign30500_e48172_d_n1;
        var_ef2_dn2 = assign30500_e48172_d_n2;
        var_ef2_dn3 = assign30500_e48172_d_n3;
        var_ef2_dn4 = assign30500_e48172_d_n4;
        var_ef2_dn5 = assign30500_e48172_d_n5;
        var_ef2_dn6 = assign30500_e48172_d_n6;
        var_ef2_dn7 = assign30500_e48172_d_n7;
        var_ef2_dn8 = assign30500_e48172_d_n8;
        var_ef2_dn9 = assign30500_e48172_d_n9;
        var_ef2_dn12 = assign30500_e48172_d_n12;
        var_ef2_dn14 = assign30500_e48172_d_n14;
        var_ef2_dn15 = assign30500_e48172_d_n15;
        var_ef2_dn16 = assign30500_e48172_d_n16;
        var_ef2_dn17 = assign30500_e48172_d_n17;
        var_ef2_dn18 = assign30500_e48172_d_n18;
        var_ef2_dn19 = assign30500_e48172_d_n19;
        var_ef2_dn20 = assign30500_e48172_d_n20;
        var_ef2_dn21 = assign30500_e48172_d_n21;
        var_ef2_dn22 = assign30500_e48172_d_n22;

        let (assign30510_e48183, assign30510_e48183_d_n0, assign30510_e48183_d_n1, assign30510_e48183_d_n2, assign30510_e48183_d_n3, assign30510_e48183_d_n4, assign30510_e48183_d_n5, assign30510_e48183_d_n6, assign30510_e48183_d_n7, assign30510_e48183_d_n8, assign30510_e48183_d_n9, assign30510_e48183_d_n12, assign30510_e48183_d_n14, assign30510_e48183_d_n15, assign30510_e48183_d_n16, assign30510_e48183_d_n17, assign30510_e48183_d_n18, assign30510_e48183_d_n19, assign30510_e48183_d_n20, assign30510_e48183_d_n21, assign30510_e48183_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let assign30510_e48181: f64 = (var_vgod - var_ef2);
        (assign30510_e48181, (var_vgod_dn0 - var_ef2_dn0), (var_vgod_dn1 - var_ef2_dn1), (var_vgod_dn2 - var_ef2_dn2), (var_vgod_dn3 - var_ef2_dn3), (var_vgod_dn4 - var_ef2_dn4), (var_vgod_dn5 - var_ef2_dn5), (var_vgod_dn6 - var_ef2_dn6), (var_vgod_dn7 - var_ef2_dn7), (var_vgod_dn8 - var_ef2_dn8), (var_vgod_dn9 - var_ef2_dn9), (var_vgod_dn12 - var_ef2_dn12), (var_vgod_dn14 - var_ef2_dn14), (var_vgod_dn15 - var_ef2_dn15), (var_vgod_dn16 - var_ef2_dn16), (var_vgod_dn17 - var_ef2_dn17), (var_vgod_dn18 - var_ef2_dn18), (var_vgod_dn19 - var_ef2_dn19), (var_vgod_dn20 - var_ef2_dn20), (var_vgod_dn21 - var_ef2_dn21), (var_vgod_dn22 - var_ef2_dn22),)
    } else {
        (var_vgef2, var_vgef2_dn0, var_vgef2_dn1, var_vgef2_dn2, var_vgef2_dn3, var_vgef2_dn4, var_vgef2_dn5, var_vgef2_dn6, var_vgef2_dn7, var_vgef2_dn8, var_vgef2_dn9, var_vgef2_dn12, var_vgef2_dn14, var_vgef2_dn15, var_vgef2_dn16, var_vgef2_dn17, var_vgef2_dn18, var_vgef2_dn19, var_vgef2_dn20, var_vgef2_dn21, var_vgef2_dn22,)
    }
};
        var_vgef2 = assign30510_e48183;
        var_vgef2_dn0 = assign30510_e48183_d_n0;
        var_vgef2_dn1 = assign30510_e48183_d_n1;
        var_vgef2_dn2 = assign30510_e48183_d_n2;
        var_vgef2_dn3 = assign30510_e48183_d_n3;
        var_vgef2_dn4 = assign30510_e48183_d_n4;
        var_vgef2_dn5 = assign30510_e48183_d_n5;
        var_vgef2_dn6 = assign30510_e48183_d_n6;
        var_vgef2_dn7 = assign30510_e48183_d_n7;
        var_vgef2_dn8 = assign30510_e48183_d_n8;
        var_vgef2_dn9 = assign30510_e48183_d_n9;
        var_vgef2_dn12 = assign30510_e48183_d_n12;
        var_vgef2_dn14 = assign30510_e48183_d_n14;
        var_vgef2_dn15 = assign30510_e48183_d_n15;
        var_vgef2_dn16 = assign30510_e48183_d_n16;
        var_vgef2_dn17 = assign30510_e48183_d_n17;
        var_vgef2_dn18 = assign30510_e48183_d_n18;
        var_vgef2_dn19 = assign30510_e48183_d_n19;
        var_vgef2_dn20 = assign30510_e48183_d_n20;
        var_vgef2_dn21 = assign30510_e48183_d_n21;
        var_vgef2_dn22 = assign30510_e48183_d_n22;

        let (assign30520_e48207, assign30520_e48207_d_n0, assign30520_e48207_d_n1, assign30520_e48207_d_n2, assign30520_e48207_d_n3, assign30520_e48207_d_n4, assign30520_e48207_d_n5, assign30520_e48207_d_n6, assign30520_e48207_d_n7, assign30520_e48207_d_n8, assign30520_e48207_d_n9, assign30520_e48207_d_n12, assign30520_e48207_d_n14, assign30520_e48207_d_n15, assign30520_e48207_d_n16, assign30520_e48207_d_n17, assign30520_e48207_d_n18, assign30520_e48207_d_n19, assign30520_e48207_d_n20, assign30520_e48207_d_n21, assign30520_e48207_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let assign30520_e48192: f64 = (0.5 * var_vgef2);
        let assign30520_e48196: f64 = (var_vgef2 * var_vgef2);
        let assign30520_e48199: f64 = (4.0 * 1e-9);
        let assign30520_e48201: f64 = (assign30520_e48199 * 1e-9);
        let assign30520_e48202: f64 = (assign30520_e48196 + assign30520_e48201);
        let assign30520_e48203: f64 = (assign30520_e48202).sqrt();
        let assign30520_e48204: f64 = (0.5 * assign30520_e48203);
        let assign30520_e48205: f64 = (assign30520_e48192 + assign30520_e48204);
        (assign30520_e48205, ((0.5 * var_vgef2_dn0) + (0.5 * (((var_vgef2_dn0 * var_vgef2) + (var_vgef2 * var_vgef2_dn0)) / (2.0 * assign30520_e48203)))), ((0.5 * var_vgef2_dn1) + (0.5 * (((var_vgef2_dn1 * var_vgef2) + (var_vgef2 * var_vgef2_dn1)) / (2.0 * assign30520_e48203)))), ((0.5 * var_vgef2_dn2) + (0.5 * (((var_vgef2_dn2 * var_vgef2) + (var_vgef2 * var_vgef2_dn2)) / (2.0 * assign30520_e48203)))), ((0.5 * var_vgef2_dn3) + (0.5 * (((var_vgef2_dn3 * var_vgef2) + (var_vgef2 * var_vgef2_dn3)) / (2.0 * assign30520_e48203)))), ((0.5 * var_vgef2_dn4) + (0.5 * (((var_vgef2_dn4 * var_vgef2) + (var_vgef2 * var_vgef2_dn4)) / (2.0 * assign30520_e48203)))), ((0.5 * var_vgef2_dn5) + (0.5 * (((var_vgef2_dn5 * var_vgef2) + (var_vgef2 * var_vgef2_dn5)) / (2.0 * assign30520_e48203)))), ((0.5 * var_vgef2_dn6) + (0.5 * (((var_vgef2_dn6 * var_vgef2) + (var_vgef2 * var_vgef2_dn6)) / (2.0 * assign30520_e48203)))), ((0.5 * var_vgef2_dn7) + (0.5 * (((var_vgef2_dn7 * var_vgef2) + (var_vgef2 * var_vgef2_dn7)) / (2.0 * assign30520_e48203)))), ((0.5 * var_vgef2_dn8) + (0.5 * (((var_vgef2_dn8 * var_vgef2) + (var_vgef2 * var_vgef2_dn8)) / (2.0 * assign30520_e48203)))), ((0.5 * var_vgef2_dn9) + (0.5 * (((var_vgef2_dn9 * var_vgef2) + (var_vgef2 * var_vgef2_dn9)) / (2.0 * assign30520_e48203)))), ((0.5 * var_vgef2_dn12) + (0.5 * (((var_vgef2_dn12 * var_vgef2) + (var_vgef2 * var_vgef2_dn12)) / (2.0 * assign30520_e48203)))), ((0.5 * var_vgef2_dn14) + (0.5 * (((var_vgef2_dn14 * var_vgef2) + (var_vgef2 * var_vgef2_dn14)) / (2.0 * assign30520_e48203)))), ((0.5 * var_vgef2_dn15) + (0.5 * (((var_vgef2_dn15 * var_vgef2) + (var_vgef2 * var_vgef2_dn15)) / (2.0 * assign30520_e48203)))), ((0.5 * var_vgef2_dn16) + (0.5 * (((var_vgef2_dn16 * var_vgef2) + (var_vgef2 * var_vgef2_dn16)) / (2.0 * assign30520_e48203)))), ((0.5 * var_vgef2_dn17) + (0.5 * (((var_vgef2_dn17 * var_vgef2) + (var_vgef2 * var_vgef2_dn17)) / (2.0 * assign30520_e48203)))), ((0.5 * var_vgef2_dn18) + (0.5 * (((var_vgef2_dn18 * var_vgef2) + (var_vgef2 * var_vgef2_dn18)) / (2.0 * assign30520_e48203)))), ((0.5 * var_vgef2_dn19) + (0.5 * (((var_vgef2_dn19 * var_vgef2) + (var_vgef2 * var_vgef2_dn19)) / (2.0 * assign30520_e48203)))), ((0.5 * var_vgef2_dn20) + (0.5 * (((var_vgef2_dn20 * var_vgef2) + (var_vgef2 * var_vgef2_dn20)) / (2.0 * assign30520_e48203)))), ((0.5 * var_vgef2_dn21) + (0.5 * (((var_vgef2_dn21 * var_vgef2) + (var_vgef2 * var_vgef2_dn21)) / (2.0 * assign30520_e48203)))), ((0.5 * var_vgef2_dn22) + (0.5 * (((var_vgef2_dn22 * var_vgef2) + (var_vgef2 * var_vgef2_dn22)) / (2.0 * assign30520_e48203)))),)
    } else {
        (var_vgef2, var_vgef2_dn0, var_vgef2_dn1, var_vgef2_dn2, var_vgef2_dn3, var_vgef2_dn4, var_vgef2_dn5, var_vgef2_dn6, var_vgef2_dn7, var_vgef2_dn8, var_vgef2_dn9, var_vgef2_dn12, var_vgef2_dn14, var_vgef2_dn15, var_vgef2_dn16, var_vgef2_dn17, var_vgef2_dn18, var_vgef2_dn19, var_vgef2_dn20, var_vgef2_dn21, var_vgef2_dn22,)
    }
};
        var_vgef2 = assign30520_e48207;
        var_vgef2_dn0 = assign30520_e48207_d_n0;
        var_vgef2_dn1 = assign30520_e48207_d_n1;
        var_vgef2_dn2 = assign30520_e48207_d_n2;
        var_vgef2_dn3 = assign30520_e48207_d_n3;
        var_vgef2_dn4 = assign30520_e48207_d_n4;
        var_vgef2_dn5 = assign30520_e48207_d_n5;
        var_vgef2_dn6 = assign30520_e48207_d_n6;
        var_vgef2_dn7 = assign30520_e48207_d_n7;
        var_vgef2_dn8 = assign30520_e48207_d_n8;
        var_vgef2_dn9 = assign30520_e48207_d_n9;
        var_vgef2_dn12 = assign30520_e48207_d_n12;
        var_vgef2_dn14 = assign30520_e48207_d_n14;
        var_vgef2_dn15 = assign30520_e48207_d_n15;
        var_vgef2_dn16 = assign30520_e48207_d_n16;
        var_vgef2_dn17 = assign30520_e48207_d_n17;
        var_vgef2_dn18 = assign30520_e48207_d_n18;
        var_vgef2_dn19 = assign30520_e48207_d_n19;
        var_vgef2_dn20 = assign30520_e48207_d_n20;
        var_vgef2_dn21 = assign30520_e48207_d_n21;
        var_vgef2_dn22 = assign30520_e48207_d_n22;

        let (assign30530_e48222, assign30530_e48222_d_n0, assign30530_e48222_d_n1, assign30530_e48222_d_n2, assign30530_e48222_d_n3, assign30530_e48222_d_n4, assign30530_e48222_d_n5, assign30530_e48222_d_n6, assign30530_e48222_d_n7, assign30530_e48222_d_n8, assign30530_e48222_d_n9, assign30530_e48222_d_n12, assign30530_e48222_d_n14, assign30530_e48222_d_n15, assign30530_e48222_d_n16, assign30530_e48222_d_n17, assign30530_e48222_d_n18, assign30530_e48222_d_n19, assign30530_e48222_d_n20, assign30530_e48222_d_n21, assign30530_e48222_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let assign30530_e48216: f64 = (p.p208 * var_t0);
        let assign30530_e48219: f64 = (var_vgef2).powf(0.6666666666666666);
        let assign30530_e48220: f64 = (assign30530_e48216 * assign30530_e48219);
        (assign30530_e48220, (((p.p208 * var_t0_dn0) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn0)) } } else { (assign30530_e48219 * (0.6666666666666666 * (var_vgef2_dn0 / var_vgef2))) })), (((p.p208 * var_t0_dn1) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn1)) } } else { (assign30530_e48219 * (0.6666666666666666 * (var_vgef2_dn1 / var_vgef2))) })), (((p.p208 * var_t0_dn2) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn2)) } } else { (assign30530_e48219 * (0.6666666666666666 * (var_vgef2_dn2 / var_vgef2))) })), (((p.p208 * var_t0_dn3) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn3)) } } else { (assign30530_e48219 * (0.6666666666666666 * (var_vgef2_dn3 / var_vgef2))) })), (((p.p208 * var_t0_dn4) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn4)) } } else { (assign30530_e48219 * (0.6666666666666666 * (var_vgef2_dn4 / var_vgef2))) })), (((p.p208 * var_t0_dn5) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn5)) } } else { (assign30530_e48219 * (0.6666666666666666 * (var_vgef2_dn5 / var_vgef2))) })), (((p.p208 * var_t0_dn6) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn6)) } } else { (assign30530_e48219 * (0.6666666666666666 * (var_vgef2_dn6 / var_vgef2))) })), (((p.p208 * var_t0_dn7) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn7)) } } else { (assign30530_e48219 * (0.6666666666666666 * (var_vgef2_dn7 / var_vgef2))) })), (((p.p208 * var_t0_dn8) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn8)) } } else { (assign30530_e48219 * (0.6666666666666666 * (var_vgef2_dn8 / var_vgef2))) })), (((p.p208 * var_t0_dn9) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn9)) } } else { (assign30530_e48219 * (0.6666666666666666 * (var_vgef2_dn9 / var_vgef2))) })), (((p.p208 * var_t0_dn12) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn12)) } } else { (assign30530_e48219 * (0.6666666666666666 * (var_vgef2_dn12 / var_vgef2))) })), (((p.p208 * var_t0_dn14) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn14)) } } else { (assign30530_e48219 * (0.6666666666666666 * (var_vgef2_dn14 / var_vgef2))) })), (((p.p208 * var_t0_dn15) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn15)) } } else { (assign30530_e48219 * (0.6666666666666666 * (var_vgef2_dn15 / var_vgef2))) })), (((p.p208 * var_t0_dn16) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn16)) } } else { (assign30530_e48219 * (0.6666666666666666 * (var_vgef2_dn16 / var_vgef2))) })), (((p.p208 * var_t0_dn17) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn17)) } } else { (assign30530_e48219 * (0.6666666666666666 * (var_vgef2_dn17 / var_vgef2))) })), (((p.p208 * var_t0_dn18) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn18)) } } else { (assign30530_e48219 * (0.6666666666666666 * (var_vgef2_dn18 / var_vgef2))) })), (((p.p208 * var_t0_dn19) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn19)) } } else { (assign30530_e48219 * (0.6666666666666666 * (var_vgef2_dn19 / var_vgef2))) })), (((p.p208 * var_t0_dn20) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn20)) } } else { (assign30530_e48219 * (0.6666666666666666 * (var_vgef2_dn20 / var_vgef2))) })), (((p.p208 * var_t0_dn21) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn21)) } } else { (assign30530_e48219 * (0.6666666666666666 * (var_vgef2_dn21 / var_vgef2))) })), (((p.p208 * var_t0_dn22) * assign30530_e48219) + (assign30530_e48216 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn22)) } } else { (assign30530_e48219 * (0.6666666666666666 * (var_vgef2_dn22 / var_vgef2))) })),)
    } else {
        (var_vgef223g0, var_vgef223g0_dn0, var_vgef223g0_dn1, var_vgef223g0_dn2, var_vgef223g0_dn3, var_vgef223g0_dn4, var_vgef223g0_dn5, var_vgef223g0_dn6, var_vgef223g0_dn7, var_vgef223g0_dn8, var_vgef223g0_dn9, var_vgef223g0_dn12, var_vgef223g0_dn14, var_vgef223g0_dn15, var_vgef223g0_dn16, var_vgef223g0_dn17, var_vgef223g0_dn18, var_vgef223g0_dn19, var_vgef223g0_dn20, var_vgef223g0_dn21, var_vgef223g0_dn22,)
    }
};
        var_vgef223g0 = assign30530_e48222;
        var_vgef223g0_dn0 = assign30530_e48222_d_n0;
        var_vgef223g0_dn1 = assign30530_e48222_d_n1;
        var_vgef223g0_dn2 = assign30530_e48222_d_n2;
        var_vgef223g0_dn3 = assign30530_e48222_d_n3;
        var_vgef223g0_dn4 = assign30530_e48222_d_n4;
        var_vgef223g0_dn5 = assign30530_e48222_d_n5;
        var_vgef223g0_dn6 = assign30530_e48222_d_n6;
        var_vgef223g0_dn7 = assign30530_e48222_d_n7;
        var_vgef223g0_dn8 = assign30530_e48222_d_n8;
        var_vgef223g0_dn9 = assign30530_e48222_d_n9;
        var_vgef223g0_dn12 = assign30530_e48222_d_n12;
        var_vgef223g0_dn14 = assign30530_e48222_d_n14;
        var_vgef223g0_dn15 = assign30530_e48222_d_n15;
        var_vgef223g0_dn16 = assign30530_e48222_d_n16;
        var_vgef223g0_dn17 = assign30530_e48222_d_n17;
        var_vgef223g0_dn18 = assign30530_e48222_d_n18;
        var_vgef223g0_dn19 = assign30530_e48222_d_n19;
        var_vgef223g0_dn20 = assign30530_e48222_d_n20;
        var_vgef223g0_dn21 = assign30530_e48222_d_n21;
        var_vgef223g0_dn22 = assign30530_e48222_d_n22;

        let (assign30540_e48237, assign30540_e48237_d_n0, assign30540_e48237_d_n1, assign30540_e48237_d_n2, assign30540_e48237_d_n3, assign30540_e48237_d_n4, assign30540_e48237_d_n5, assign30540_e48237_d_n6, assign30540_e48237_d_n7, assign30540_e48237_d_n8, assign30540_e48237_d_n9, assign30540_e48237_d_n12, assign30540_e48237_d_n14, assign30540_e48237_d_n15, assign30540_e48237_d_n16, assign30540_e48237_d_n17, assign30540_e48237_d_n18, assign30540_e48237_d_n19, assign30540_e48237_d_n20, assign30540_e48237_d_n21, assign30540_e48237_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let assign30540_e48231: f64 = (p.p209 * var_t0);
        let assign30540_e48234: f64 = (var_vgef2).powf(0.6666666666666666);
        let assign30540_e48235: f64 = (assign30540_e48231 * assign30540_e48234);
        (assign30540_e48235, (((p.p209 * var_t0_dn0) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn0)) } } else { (assign30540_e48234 * (0.6666666666666666 * (var_vgef2_dn0 / var_vgef2))) })), (((p.p209 * var_t0_dn1) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn1)) } } else { (assign30540_e48234 * (0.6666666666666666 * (var_vgef2_dn1 / var_vgef2))) })), (((p.p209 * var_t0_dn2) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn2)) } } else { (assign30540_e48234 * (0.6666666666666666 * (var_vgef2_dn2 / var_vgef2))) })), (((p.p209 * var_t0_dn3) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn3)) } } else { (assign30540_e48234 * (0.6666666666666666 * (var_vgef2_dn3 / var_vgef2))) })), (((p.p209 * var_t0_dn4) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn4)) } } else { (assign30540_e48234 * (0.6666666666666666 * (var_vgef2_dn4 / var_vgef2))) })), (((p.p209 * var_t0_dn5) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn5)) } } else { (assign30540_e48234 * (0.6666666666666666 * (var_vgef2_dn5 / var_vgef2))) })), (((p.p209 * var_t0_dn6) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn6)) } } else { (assign30540_e48234 * (0.6666666666666666 * (var_vgef2_dn6 / var_vgef2))) })), (((p.p209 * var_t0_dn7) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn7)) } } else { (assign30540_e48234 * (0.6666666666666666 * (var_vgef2_dn7 / var_vgef2))) })), (((p.p209 * var_t0_dn8) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn8)) } } else { (assign30540_e48234 * (0.6666666666666666 * (var_vgef2_dn8 / var_vgef2))) })), (((p.p209 * var_t0_dn9) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn9)) } } else { (assign30540_e48234 * (0.6666666666666666 * (var_vgef2_dn9 / var_vgef2))) })), (((p.p209 * var_t0_dn12) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn12)) } } else { (assign30540_e48234 * (0.6666666666666666 * (var_vgef2_dn12 / var_vgef2))) })), (((p.p209 * var_t0_dn14) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn14)) } } else { (assign30540_e48234 * (0.6666666666666666 * (var_vgef2_dn14 / var_vgef2))) })), (((p.p209 * var_t0_dn15) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn15)) } } else { (assign30540_e48234 * (0.6666666666666666 * (var_vgef2_dn15 / var_vgef2))) })), (((p.p209 * var_t0_dn16) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn16)) } } else { (assign30540_e48234 * (0.6666666666666666 * (var_vgef2_dn16 / var_vgef2))) })), (((p.p209 * var_t0_dn17) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn17)) } } else { (assign30540_e48234 * (0.6666666666666666 * (var_vgef2_dn17 / var_vgef2))) })), (((p.p209 * var_t0_dn18) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn18)) } } else { (assign30540_e48234 * (0.6666666666666666 * (var_vgef2_dn18 / var_vgef2))) })), (((p.p209 * var_t0_dn19) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn19)) } } else { (assign30540_e48234 * (0.6666666666666666 * (var_vgef2_dn19 / var_vgef2))) })), (((p.p209 * var_t0_dn20) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn20)) } } else { (assign30540_e48234 * (0.6666666666666666 * (var_vgef2_dn20 / var_vgef2))) })), (((p.p209 * var_t0_dn21) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn21)) } } else { (assign30540_e48234 * (0.6666666666666666 * (var_vgef2_dn21 / var_vgef2))) })), (((p.p209 * var_t0_dn22) * assign30540_e48234) + (assign30540_e48231 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_vgef2).powf(0.6666666666666666 - 1.0) * var_vgef2_dn22)) } } else { (assign30540_e48234 * (0.6666666666666666 * (var_vgef2_dn22 / var_vgef2))) })),)
    } else {
        (var_vgef223g1, var_vgef223g1_dn0, var_vgef223g1_dn1, var_vgef223g1_dn2, var_vgef223g1_dn3, var_vgef223g1_dn4, var_vgef223g1_dn5, var_vgef223g1_dn6, var_vgef223g1_dn7, var_vgef223g1_dn8, var_vgef223g1_dn9, var_vgef223g1_dn12, var_vgef223g1_dn14, var_vgef223g1_dn15, var_vgef223g1_dn16, var_vgef223g1_dn17, var_vgef223g1_dn18, var_vgef223g1_dn19, var_vgef223g1_dn20, var_vgef223g1_dn21, var_vgef223g1_dn22,)
    }
};
        var_vgef223g1 = assign30540_e48237;
        var_vgef223g1_dn0 = assign30540_e48237_d_n0;
        var_vgef223g1_dn1 = assign30540_e48237_d_n1;
        var_vgef223g1_dn2 = assign30540_e48237_d_n2;
        var_vgef223g1_dn3 = assign30540_e48237_d_n3;
        var_vgef223g1_dn4 = assign30540_e48237_d_n4;
        var_vgef223g1_dn5 = assign30540_e48237_d_n5;
        var_vgef223g1_dn6 = assign30540_e48237_d_n6;
        var_vgef223g1_dn7 = assign30540_e48237_d_n7;
        var_vgef223g1_dn8 = assign30540_e48237_d_n8;
        var_vgef223g1_dn9 = assign30540_e48237_d_n9;
        var_vgef223g1_dn12 = assign30540_e48237_d_n12;
        var_vgef223g1_dn14 = assign30540_e48237_d_n14;
        var_vgef223g1_dn15 = assign30540_e48237_d_n15;
        var_vgef223g1_dn16 = assign30540_e48237_d_n16;
        var_vgef223g1_dn17 = assign30540_e48237_d_n17;
        var_vgef223g1_dn18 = assign30540_e48237_d_n18;
        var_vgef223g1_dn19 = assign30540_e48237_d_n19;
        var_vgef223g1_dn20 = assign30540_e48237_d_n20;
        var_vgef223g1_dn21 = assign30540_e48237_d_n21;
        var_vgef223g1_dn22 = assign30540_e48237_d_n22;

        let (assign30550_e48252, assign30550_e48252_d_n0, assign30550_e48252_d_n1, assign30550_e48252_d_n2, assign30550_e48252_d_n3, assign30550_e48252_d_n4, assign30550_e48252_d_n5, assign30550_e48252_d_n6, assign30550_e48252_d_n7, assign30550_e48252_d_n8, assign30550_e48252_d_n9, assign30550_e48252_d_n12, assign30550_e48252_d_n14, assign30550_e48252_d_n15, assign30550_e48252_d_n16, assign30550_e48252_d_n17, assign30550_e48252_d_n18, assign30550_e48252_d_n19, assign30550_e48252_d_n20, assign30550_e48252_d_n21, assign30550_e48252_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / var_vtv;
        let assign30550_e48246: f64 = (var_ef2 * __rspice_inv_cse_0);
        let assign30550_e48249: f64 = (var_vgef223g0 * __rspice_inv_cse_0);
        let assign30550_e48250: f64 = (assign30550_e48246 - assign30550_e48249);
        (assign30550_e48250, ((var_ef2_dn0 / var_vtv) - (var_vgef223g0_dn0 / var_vtv)), ((var_ef2_dn1 / var_vtv) - (var_vgef223g0_dn1 / var_vtv)), ((var_ef2_dn2 / var_vtv) - (var_vgef223g0_dn2 / var_vtv)), ((var_ef2_dn3 / var_vtv) - (var_vgef223g0_dn3 / var_vtv)), ((((var_ef2_dn4 * var_vtv) - (var_ef2 * var_vtv_dn4)) / (var_vtv * var_vtv)) - (((var_vgef223g0_dn4 * var_vtv) - (var_vgef223g0 * var_vtv_dn4)) / (var_vtv * var_vtv))), ((var_ef2_dn5 / var_vtv) - (var_vgef223g0_dn5 / var_vtv)), ((((var_ef2_dn6 * var_vtv) - (var_ef2 * var_vtv_dn6)) / (var_vtv * var_vtv)) - (((var_vgef223g0_dn6 * var_vtv) - (var_vgef223g0 * var_vtv_dn6)) / (var_vtv * var_vtv))), ((((var_ef2_dn7 * var_vtv) - (var_ef2 * var_vtv_dn7)) / (var_vtv * var_vtv)) - (((var_vgef223g0_dn7 * var_vtv) - (var_vgef223g0 * var_vtv_dn7)) / (var_vtv * var_vtv))), ((((var_ef2_dn8 * var_vtv) - (var_ef2 * var_vtv_dn8)) / (var_vtv * var_vtv)) - (((var_vgef223g0_dn8 * var_vtv) - (var_vgef223g0 * var_vtv_dn8)) / (var_vtv * var_vtv))), ((var_ef2_dn9 / var_vtv) - (var_vgef223g0_dn9 / var_vtv)), ((var_ef2_dn12 / var_vtv) - (var_vgef223g0_dn12 / var_vtv)), ((var_ef2_dn14 / var_vtv) - (var_vgef223g0_dn14 / var_vtv)), ((((var_ef2_dn15 * var_vtv) - (var_ef2 * var_vtv_dn15)) / (var_vtv * var_vtv)) - (((var_vgef223g0_dn15 * var_vtv) - (var_vgef223g0 * var_vtv_dn15)) / (var_vtv * var_vtv))), ((((var_ef2_dn16 * var_vtv) - (var_ef2 * var_vtv_dn16)) / (var_vtv * var_vtv)) - (((var_vgef223g0_dn16 * var_vtv) - (var_vgef223g0 * var_vtv_dn16)) / (var_vtv * var_vtv))), ((((var_ef2_dn17 * var_vtv) - (var_ef2 * var_vtv_dn17)) / (var_vtv * var_vtv)) - (((var_vgef223g0_dn17 * var_vtv) - (var_vgef223g0 * var_vtv_dn17)) / (var_vtv * var_vtv))), ((((var_ef2_dn18 * var_vtv) - (var_ef2 * var_vtv_dn18)) / (var_vtv * var_vtv)) - (((var_vgef223g0_dn18 * var_vtv) - (var_vgef223g0 * var_vtv_dn18)) / (var_vtv * var_vtv))), ((((var_ef2_dn19 * var_vtv) - (var_ef2 * var_vtv_dn19)) / (var_vtv * var_vtv)) - (((var_vgef223g0_dn19 * var_vtv) - (var_vgef223g0 * var_vtv_dn19)) / (var_vtv * var_vtv))), ((((var_ef2_dn20 * var_vtv) - (var_ef2 * var_vtv_dn20)) / (var_vtv * var_vtv)) - (((var_vgef223g0_dn20 * var_vtv) - (var_vgef223g0 * var_vtv_dn20)) / (var_vtv * var_vtv))), ((((var_ef2_dn21 * var_vtv) - (var_ef2 * var_vtv_dn21)) / (var_vtv * var_vtv)) - (((var_vgef223g0_dn21 * var_vtv) - (var_vgef223g0 * var_vtv_dn21)) / (var_vtv * var_vtv))), ((((var_ef2_dn22 * var_vtv) - (var_ef2 * var_vtv_dn22)) / (var_vtv * var_vtv)) - (((var_vgef223g0_dn22 * var_vtv) - (var_vgef223g0 * var_vtv_dn22)) / (var_vtv * var_vtv))),)
    } else {
        (var_tg02, var_tg02_dn0, var_tg02_dn1, var_tg02_dn2, var_tg02_dn3, var_tg02_dn4, var_tg02_dn5, var_tg02_dn6, var_tg02_dn7, var_tg02_dn8, var_tg02_dn9, var_tg02_dn12, var_tg02_dn14, var_tg02_dn15, var_tg02_dn16, var_tg02_dn17, var_tg02_dn18, var_tg02_dn19, var_tg02_dn20, var_tg02_dn21, var_tg02_dn22,)
    }
};
        var_tg02 = assign30550_e48252;
        var_tg02_dn0 = assign30550_e48252_d_n0;
        var_tg02_dn1 = assign30550_e48252_d_n1;
        var_tg02_dn2 = assign30550_e48252_d_n2;
        var_tg02_dn3 = assign30550_e48252_d_n3;
        var_tg02_dn4 = assign30550_e48252_d_n4;
        var_tg02_dn5 = assign30550_e48252_d_n5;
        var_tg02_dn6 = assign30550_e48252_d_n6;
        var_tg02_dn7 = assign30550_e48252_d_n7;
        var_tg02_dn8 = assign30550_e48252_d_n8;
        var_tg02_dn9 = assign30550_e48252_d_n9;
        var_tg02_dn12 = assign30550_e48252_d_n12;
        var_tg02_dn14 = assign30550_e48252_d_n14;
        var_tg02_dn15 = assign30550_e48252_d_n15;
        var_tg02_dn16 = assign30550_e48252_d_n16;
        var_tg02_dn17 = assign30550_e48252_d_n17;
        var_tg02_dn18 = assign30550_e48252_d_n18;
        var_tg02_dn19 = assign30550_e48252_d_n19;
        var_tg02_dn20 = assign30550_e48252_d_n20;
        var_tg02_dn21 = assign30550_e48252_d_n21;
        var_tg02_dn22 = assign30550_e48252_d_n22;

        let (assign30560_e48267, assign30560_e48267_d_n0, assign30560_e48267_d_n1, assign30560_e48267_d_n2, assign30560_e48267_d_n3, assign30560_e48267_d_n4, assign30560_e48267_d_n5, assign30560_e48267_d_n6, assign30560_e48267_d_n7, assign30560_e48267_d_n8, assign30560_e48267_d_n9, assign30560_e48267_d_n12, assign30560_e48267_d_n14, assign30560_e48267_d_n15, assign30560_e48267_d_n16, assign30560_e48267_d_n17, assign30560_e48267_d_n18, assign30560_e48267_d_n19, assign30560_e48267_d_n20, assign30560_e48267_d_n21, assign30560_e48267_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let __rspice_inv_cse_1: f64 = 1.0 / var_vtv;
        let assign30560_e48261: f64 = (var_ef2 * __rspice_inv_cse_1);
        let assign30560_e48264: f64 = (var_vgef223g1 * __rspice_inv_cse_1);
        let assign30560_e48265: f64 = (assign30560_e48261 - assign30560_e48264);
        (assign30560_e48265, ((var_ef2_dn0 / var_vtv) - (var_vgef223g1_dn0 / var_vtv)), ((var_ef2_dn1 / var_vtv) - (var_vgef223g1_dn1 / var_vtv)), ((var_ef2_dn2 / var_vtv) - (var_vgef223g1_dn2 / var_vtv)), ((var_ef2_dn3 / var_vtv) - (var_vgef223g1_dn3 / var_vtv)), ((((var_ef2_dn4 * var_vtv) - (var_ef2 * var_vtv_dn4)) / (var_vtv * var_vtv)) - (((var_vgef223g1_dn4 * var_vtv) - (var_vgef223g1 * var_vtv_dn4)) / (var_vtv * var_vtv))), ((var_ef2_dn5 / var_vtv) - (var_vgef223g1_dn5 / var_vtv)), ((((var_ef2_dn6 * var_vtv) - (var_ef2 * var_vtv_dn6)) / (var_vtv * var_vtv)) - (((var_vgef223g1_dn6 * var_vtv) - (var_vgef223g1 * var_vtv_dn6)) / (var_vtv * var_vtv))), ((((var_ef2_dn7 * var_vtv) - (var_ef2 * var_vtv_dn7)) / (var_vtv * var_vtv)) - (((var_vgef223g1_dn7 * var_vtv) - (var_vgef223g1 * var_vtv_dn7)) / (var_vtv * var_vtv))), ((((var_ef2_dn8 * var_vtv) - (var_ef2 * var_vtv_dn8)) / (var_vtv * var_vtv)) - (((var_vgef223g1_dn8 * var_vtv) - (var_vgef223g1 * var_vtv_dn8)) / (var_vtv * var_vtv))), ((var_ef2_dn9 / var_vtv) - (var_vgef223g1_dn9 / var_vtv)), ((var_ef2_dn12 / var_vtv) - (var_vgef223g1_dn12 / var_vtv)), ((var_ef2_dn14 / var_vtv) - (var_vgef223g1_dn14 / var_vtv)), ((((var_ef2_dn15 * var_vtv) - (var_ef2 * var_vtv_dn15)) / (var_vtv * var_vtv)) - (((var_vgef223g1_dn15 * var_vtv) - (var_vgef223g1 * var_vtv_dn15)) / (var_vtv * var_vtv))), ((((var_ef2_dn16 * var_vtv) - (var_ef2 * var_vtv_dn16)) / (var_vtv * var_vtv)) - (((var_vgef223g1_dn16 * var_vtv) - (var_vgef223g1 * var_vtv_dn16)) / (var_vtv * var_vtv))), ((((var_ef2_dn17 * var_vtv) - (var_ef2 * var_vtv_dn17)) / (var_vtv * var_vtv)) - (((var_vgef223g1_dn17 * var_vtv) - (var_vgef223g1 * var_vtv_dn17)) / (var_vtv * var_vtv))), ((((var_ef2_dn18 * var_vtv) - (var_ef2 * var_vtv_dn18)) / (var_vtv * var_vtv)) - (((var_vgef223g1_dn18 * var_vtv) - (var_vgef223g1 * var_vtv_dn18)) / (var_vtv * var_vtv))), ((((var_ef2_dn19 * var_vtv) - (var_ef2 * var_vtv_dn19)) / (var_vtv * var_vtv)) - (((var_vgef223g1_dn19 * var_vtv) - (var_vgef223g1 * var_vtv_dn19)) / (var_vtv * var_vtv))), ((((var_ef2_dn20 * var_vtv) - (var_ef2 * var_vtv_dn20)) / (var_vtv * var_vtv)) - (((var_vgef223g1_dn20 * var_vtv) - (var_vgef223g1 * var_vtv_dn20)) / (var_vtv * var_vtv))), ((((var_ef2_dn21 * var_vtv) - (var_ef2 * var_vtv_dn21)) / (var_vtv * var_vtv)) - (((var_vgef223g1_dn21 * var_vtv) - (var_vgef223g1 * var_vtv_dn21)) / (var_vtv * var_vtv))), ((((var_ef2_dn22 * var_vtv) - (var_ef2 * var_vtv_dn22)) / (var_vtv * var_vtv)) - (((var_vgef223g1_dn22 * var_vtv) - (var_vgef223g1 * var_vtv_dn22)) / (var_vtv * var_vtv))),)
    } else {
        (var_tg12, var_tg12_dn0, var_tg12_dn1, var_tg12_dn2, var_tg12_dn3, var_tg12_dn4, var_tg12_dn5, var_tg12_dn6, var_tg12_dn7, var_tg12_dn8, var_tg12_dn9, var_tg12_dn12, var_tg12_dn14, var_tg12_dn15, var_tg12_dn16, var_tg12_dn17, var_tg12_dn18, var_tg12_dn19, var_tg12_dn20, var_tg12_dn21, var_tg12_dn22,)
    }
};
        var_tg12 = assign30560_e48267;
        var_tg12_dn0 = assign30560_e48267_d_n0;
        var_tg12_dn1 = assign30560_e48267_d_n1;
        var_tg12_dn2 = assign30560_e48267_d_n2;
        var_tg12_dn3 = assign30560_e48267_d_n3;
        var_tg12_dn4 = assign30560_e48267_d_n4;
        var_tg12_dn5 = assign30560_e48267_d_n5;
        var_tg12_dn6 = assign30560_e48267_d_n6;
        var_tg12_dn7 = assign30560_e48267_d_n7;
        var_tg12_dn8 = assign30560_e48267_d_n8;
        var_tg12_dn9 = assign30560_e48267_d_n9;
        var_tg12_dn12 = assign30560_e48267_d_n12;
        var_tg12_dn14 = assign30560_e48267_d_n14;
        var_tg12_dn15 = assign30560_e48267_d_n15;
        var_tg12_dn16 = assign30560_e48267_d_n16;
        var_tg12_dn17 = assign30560_e48267_d_n17;
        var_tg12_dn18 = assign30560_e48267_d_n18;
        var_tg12_dn19 = assign30560_e48267_d_n19;
        var_tg12_dn20 = assign30560_e48267_d_n20;
        var_tg12_dn21 = assign30560_e48267_d_n21;
        var_tg12_dn22 = assign30560_e48267_d_n22;

        let (assign30570_e48354, assign30570_e48354_d_n0, assign30570_e48354_d_n1, assign30570_e48354_d_n2, assign30570_e48354_d_n3, assign30570_e48354_d_n4, assign30570_e48354_d_n5, assign30570_e48354_d_n6, assign30570_e48354_d_n7, assign30570_e48354_d_n8, assign30570_e48354_d_n9, assign30570_e48354_d_n12, assign30570_e48354_d_n14, assign30570_e48354_d_n15, assign30570_e48354_d_n16, assign30570_e48354_d_n17, assign30570_e48354_d_n18, assign30570_e48354_d_n19, assign30570_e48354_d_n20, assign30570_e48354_d_n21, assign30570_e48354_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let assign30570_e48276: f64 = (var_cch * var_vgef2);
        let assign30570_e48279: f64 = (3.24e17 * var_vtv);
        let assign30570_e48286: f64 = (-37.0);
        let (assign30570_e48312, assign30570_e48312_d_n0, assign30570_e48312_d_n1, assign30570_e48312_d_n2, assign30570_e48312_d_n3, assign30570_e48312_d_n4, assign30570_e48312_d_n5, assign30570_e48312_d_n6, assign30570_e48312_d_n7, assign30570_e48312_d_n8, assign30570_e48312_d_n9, assign30570_e48312_d_n12, assign30570_e48312_d_n14, assign30570_e48312_d_n15, assign30570_e48312_d_n16, assign30570_e48312_d_n17, assign30570_e48312_d_n18, assign30570_e48312_d_n19, assign30570_e48312_d_n20, assign30570_e48312_d_n21, assign30570_e48312_d_n22,) = {
            if ((!(var_tg02 >= 37.0)) && (!(var_tg02 <= assign30570_e48286))) {
                let assign30570_e48291: f64 = (var_tg02).exp();
                let assign30570_e48293: f64 = (assign30570_e48291 + 1.0);
                let assign30570_e48294: f64 = (assign30570_e48293).ln();
                (assign30570_e48294, ((assign30570_e48291 * var_tg02_dn0) / assign30570_e48293), ((assign30570_e48291 * var_tg02_dn1) / assign30570_e48293), ((assign30570_e48291 * var_tg02_dn2) / assign30570_e48293), ((assign30570_e48291 * var_tg02_dn3) / assign30570_e48293), ((assign30570_e48291 * var_tg02_dn4) / assign30570_e48293), ((assign30570_e48291 * var_tg02_dn5) / assign30570_e48293), ((assign30570_e48291 * var_tg02_dn6) / assign30570_e48293), ((assign30570_e48291 * var_tg02_dn7) / assign30570_e48293), ((assign30570_e48291 * var_tg02_dn8) / assign30570_e48293), ((assign30570_e48291 * var_tg02_dn9) / assign30570_e48293), ((assign30570_e48291 * var_tg02_dn12) / assign30570_e48293), ((assign30570_e48291 * var_tg02_dn14) / assign30570_e48293), ((assign30570_e48291 * var_tg02_dn15) / assign30570_e48293), ((assign30570_e48291 * var_tg02_dn16) / assign30570_e48293), ((assign30570_e48291 * var_tg02_dn17) / assign30570_e48293), ((assign30570_e48291 * var_tg02_dn18) / assign30570_e48293), ((assign30570_e48291 * var_tg02_dn19) / assign30570_e48293), ((assign30570_e48291 * var_tg02_dn20) / assign30570_e48293), ((assign30570_e48291 * var_tg02_dn21) / assign30570_e48293), ((assign30570_e48291 * var_tg02_dn22) / assign30570_e48293),)
            } else {
                let assign30570_e48301: f64 = (-37.0);
                let (assign30570_e48311, assign30570_e48311_d_n0, assign30570_e48311_d_n1, assign30570_e48311_d_n2, assign30570_e48311_d_n3, assign30570_e48311_d_n4, assign30570_e48311_d_n5, assign30570_e48311_d_n6, assign30570_e48311_d_n7, assign30570_e48311_d_n8, assign30570_e48311_d_n9, assign30570_e48311_d_n12, assign30570_e48311_d_n14, assign30570_e48311_d_n15, assign30570_e48311_d_n16, assign30570_e48311_d_n17, assign30570_e48311_d_n18, assign30570_e48311_d_n19, assign30570_e48311_d_n20, assign30570_e48311_d_n21, assign30570_e48311_d_n22,) = {
                    if ((!(var_tg02 >= 37.0)) && (var_tg02 <= assign30570_e48301)) {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign30570_e48310, assign30570_e48310_d_n0, assign30570_e48310_d_n1, assign30570_e48310_d_n2, assign30570_e48310_d_n3, assign30570_e48310_d_n4, assign30570_e48310_d_n5, assign30570_e48310_d_n6, assign30570_e48310_d_n7, assign30570_e48310_d_n8, assign30570_e48310_d_n9, assign30570_e48310_d_n12, assign30570_e48310_d_n14, assign30570_e48310_d_n15, assign30570_e48310_d_n16, assign30570_e48310_d_n17, assign30570_e48310_d_n18, assign30570_e48310_d_n19, assign30570_e48310_d_n20, assign30570_e48310_d_n21, assign30570_e48310_d_n22,) = {
                            if (var_tg02 >= 37.0) {
                                (var_tg02, var_tg02_dn0, var_tg02_dn1, var_tg02_dn2, var_tg02_dn3, var_tg02_dn4, var_tg02_dn5, var_tg02_dn6, var_tg02_dn7, var_tg02_dn8, var_tg02_dn9, var_tg02_dn12, var_tg02_dn14, var_tg02_dn15, var_tg02_dn16, var_tg02_dn17, var_tg02_dn18, var_tg02_dn19, var_tg02_dn20, var_tg02_dn21, var_tg02_dn22,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign30570_e48310, assign30570_e48310_d_n0, assign30570_e48310_d_n1, assign30570_e48310_d_n2, assign30570_e48310_d_n3, assign30570_e48310_d_n4, assign30570_e48310_d_n5, assign30570_e48310_d_n6, assign30570_e48310_d_n7, assign30570_e48310_d_n8, assign30570_e48310_d_n9, assign30570_e48310_d_n12, assign30570_e48310_d_n14, assign30570_e48310_d_n15, assign30570_e48310_d_n16, assign30570_e48310_d_n17, assign30570_e48310_d_n18, assign30570_e48310_d_n19, assign30570_e48310_d_n20, assign30570_e48310_d_n21, assign30570_e48310_d_n22,)
                    }
                };
                (assign30570_e48311, assign30570_e48311_d_n0, assign30570_e48311_d_n1, assign30570_e48311_d_n2, assign30570_e48311_d_n3, assign30570_e48311_d_n4, assign30570_e48311_d_n5, assign30570_e48311_d_n6, assign30570_e48311_d_n7, assign30570_e48311_d_n8, assign30570_e48311_d_n9, assign30570_e48311_d_n12, assign30570_e48311_d_n14, assign30570_e48311_d_n15, assign30570_e48311_d_n16, assign30570_e48311_d_n17, assign30570_e48311_d_n18, assign30570_e48311_d_n19, assign30570_e48311_d_n20, assign30570_e48311_d_n21, assign30570_e48311_d_n22,)
            }
        };
        let assign30570_e48313: f64 = (assign30570_e48279 * assign30570_e48312);
        let assign30570_e48314: f64 = (assign30570_e48276 - assign30570_e48313);
        let assign30570_e48317: f64 = (3.24e17 * var_vtv);
        let assign30570_e48324: f64 = (-37.0);
        let (assign30570_e48350, assign30570_e48350_d_n0, assign30570_e48350_d_n1, assign30570_e48350_d_n2, assign30570_e48350_d_n3, assign30570_e48350_d_n4, assign30570_e48350_d_n5, assign30570_e48350_d_n6, assign30570_e48350_d_n7, assign30570_e48350_d_n8, assign30570_e48350_d_n9, assign30570_e48350_d_n12, assign30570_e48350_d_n14, assign30570_e48350_d_n15, assign30570_e48350_d_n16, assign30570_e48350_d_n17, assign30570_e48350_d_n18, assign30570_e48350_d_n19, assign30570_e48350_d_n20, assign30570_e48350_d_n21, assign30570_e48350_d_n22,) = {
            if ((!(var_tg12 >= 37.0)) && (!(var_tg12 <= assign30570_e48324))) {
                let assign30570_e48329: f64 = (var_tg12).exp();
                let assign30570_e48331: f64 = (assign30570_e48329 + 1.0);
                let assign30570_e48332: f64 = (assign30570_e48331).ln();
                (assign30570_e48332, ((assign30570_e48329 * var_tg12_dn0) / assign30570_e48331), ((assign30570_e48329 * var_tg12_dn1) / assign30570_e48331), ((assign30570_e48329 * var_tg12_dn2) / assign30570_e48331), ((assign30570_e48329 * var_tg12_dn3) / assign30570_e48331), ((assign30570_e48329 * var_tg12_dn4) / assign30570_e48331), ((assign30570_e48329 * var_tg12_dn5) / assign30570_e48331), ((assign30570_e48329 * var_tg12_dn6) / assign30570_e48331), ((assign30570_e48329 * var_tg12_dn7) / assign30570_e48331), ((assign30570_e48329 * var_tg12_dn8) / assign30570_e48331), ((assign30570_e48329 * var_tg12_dn9) / assign30570_e48331), ((assign30570_e48329 * var_tg12_dn12) / assign30570_e48331), ((assign30570_e48329 * var_tg12_dn14) / assign30570_e48331), ((assign30570_e48329 * var_tg12_dn15) / assign30570_e48331), ((assign30570_e48329 * var_tg12_dn16) / assign30570_e48331), ((assign30570_e48329 * var_tg12_dn17) / assign30570_e48331), ((assign30570_e48329 * var_tg12_dn18) / assign30570_e48331), ((assign30570_e48329 * var_tg12_dn19) / assign30570_e48331), ((assign30570_e48329 * var_tg12_dn20) / assign30570_e48331), ((assign30570_e48329 * var_tg12_dn21) / assign30570_e48331), ((assign30570_e48329 * var_tg12_dn22) / assign30570_e48331),)
            } else {
                let assign30570_e48339: f64 = (-37.0);
                let (assign30570_e48349, assign30570_e48349_d_n0, assign30570_e48349_d_n1, assign30570_e48349_d_n2, assign30570_e48349_d_n3, assign30570_e48349_d_n4, assign30570_e48349_d_n5, assign30570_e48349_d_n6, assign30570_e48349_d_n7, assign30570_e48349_d_n8, assign30570_e48349_d_n9, assign30570_e48349_d_n12, assign30570_e48349_d_n14, assign30570_e48349_d_n15, assign30570_e48349_d_n16, assign30570_e48349_d_n17, assign30570_e48349_d_n18, assign30570_e48349_d_n19, assign30570_e48349_d_n20, assign30570_e48349_d_n21, assign30570_e48349_d_n22,) = {
                    if ((!(var_tg12 >= 37.0)) && (var_tg12 <= assign30570_e48339)) {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    } else {
                        let (assign30570_e48348, assign30570_e48348_d_n0, assign30570_e48348_d_n1, assign30570_e48348_d_n2, assign30570_e48348_d_n3, assign30570_e48348_d_n4, assign30570_e48348_d_n5, assign30570_e48348_d_n6, assign30570_e48348_d_n7, assign30570_e48348_d_n8, assign30570_e48348_d_n9, assign30570_e48348_d_n12, assign30570_e48348_d_n14, assign30570_e48348_d_n15, assign30570_e48348_d_n16, assign30570_e48348_d_n17, assign30570_e48348_d_n18, assign30570_e48348_d_n19, assign30570_e48348_d_n20, assign30570_e48348_d_n21, assign30570_e48348_d_n22,) = {
                            if (var_tg12 >= 37.0) {
                                (var_tg12, var_tg12_dn0, var_tg12_dn1, var_tg12_dn2, var_tg12_dn3, var_tg12_dn4, var_tg12_dn5, var_tg12_dn6, var_tg12_dn7, var_tg12_dn8, var_tg12_dn9, var_tg12_dn12, var_tg12_dn14, var_tg12_dn15, var_tg12_dn16, var_tg12_dn17, var_tg12_dn18, var_tg12_dn19, var_tg12_dn20, var_tg12_dn21, var_tg12_dn22,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign30570_e48348, assign30570_e48348_d_n0, assign30570_e48348_d_n1, assign30570_e48348_d_n2, assign30570_e48348_d_n3, assign30570_e48348_d_n4, assign30570_e48348_d_n5, assign30570_e48348_d_n6, assign30570_e48348_d_n7, assign30570_e48348_d_n8, assign30570_e48348_d_n9, assign30570_e48348_d_n12, assign30570_e48348_d_n14, assign30570_e48348_d_n15, assign30570_e48348_d_n16, assign30570_e48348_d_n17, assign30570_e48348_d_n18, assign30570_e48348_d_n19, assign30570_e48348_d_n20, assign30570_e48348_d_n21, assign30570_e48348_d_n22,)
                    }
                };
                (assign30570_e48349, assign30570_e48349_d_n0, assign30570_e48349_d_n1, assign30570_e48349_d_n2, assign30570_e48349_d_n3, assign30570_e48349_d_n4, assign30570_e48349_d_n5, assign30570_e48349_d_n6, assign30570_e48349_d_n7, assign30570_e48349_d_n8, assign30570_e48349_d_n9, assign30570_e48349_d_n12, assign30570_e48349_d_n14, assign30570_e48349_d_n15, assign30570_e48349_d_n16, assign30570_e48349_d_n17, assign30570_e48349_d_n18, assign30570_e48349_d_n19, assign30570_e48349_d_n20, assign30570_e48349_d_n21, assign30570_e48349_d_n22,)
            }
        };
        let assign30570_e48351: f64 = (assign30570_e48317 * assign30570_e48350);
        let assign30570_e48352: f64 = (assign30570_e48314 - assign30570_e48351);
        (assign30570_e48352, (((var_cch * var_vgef2_dn0) - (assign30570_e48279 * assign30570_e48312_d_n0)) - (assign30570_e48317 * assign30570_e48350_d_n0)), (((var_cch * var_vgef2_dn1) - (assign30570_e48279 * assign30570_e48312_d_n1)) - (assign30570_e48317 * assign30570_e48350_d_n1)), (((var_cch * var_vgef2_dn2) - (assign30570_e48279 * assign30570_e48312_d_n2)) - (assign30570_e48317 * assign30570_e48350_d_n2)), (((var_cch * var_vgef2_dn3) - (assign30570_e48279 * assign30570_e48312_d_n3)) - (assign30570_e48317 * assign30570_e48350_d_n3)), (((var_cch * var_vgef2_dn4) - (((3.24e17 * var_vtv_dn4) * assign30570_e48312) + (assign30570_e48279 * assign30570_e48312_d_n4))) - (((3.24e17 * var_vtv_dn4) * assign30570_e48350) + (assign30570_e48317 * assign30570_e48350_d_n4))), (((var_cch * var_vgef2_dn5) - (assign30570_e48279 * assign30570_e48312_d_n5)) - (assign30570_e48317 * assign30570_e48350_d_n5)), (((var_cch * var_vgef2_dn6) - (((3.24e17 * var_vtv_dn6) * assign30570_e48312) + (assign30570_e48279 * assign30570_e48312_d_n6))) - (((3.24e17 * var_vtv_dn6) * assign30570_e48350) + (assign30570_e48317 * assign30570_e48350_d_n6))), (((var_cch * var_vgef2_dn7) - (((3.24e17 * var_vtv_dn7) * assign30570_e48312) + (assign30570_e48279 * assign30570_e48312_d_n7))) - (((3.24e17 * var_vtv_dn7) * assign30570_e48350) + (assign30570_e48317 * assign30570_e48350_d_n7))), (((var_cch * var_vgef2_dn8) - (((3.24e17 * var_vtv_dn8) * assign30570_e48312) + (assign30570_e48279 * assign30570_e48312_d_n8))) - (((3.24e17 * var_vtv_dn8) * assign30570_e48350) + (assign30570_e48317 * assign30570_e48350_d_n8))), (((var_cch * var_vgef2_dn9) - (assign30570_e48279 * assign30570_e48312_d_n9)) - (assign30570_e48317 * assign30570_e48350_d_n9)), (((var_cch * var_vgef2_dn12) - (assign30570_e48279 * assign30570_e48312_d_n12)) - (assign30570_e48317 * assign30570_e48350_d_n12)), (((var_cch * var_vgef2_dn14) - (assign30570_e48279 * assign30570_e48312_d_n14)) - (assign30570_e48317 * assign30570_e48350_d_n14)), (((var_cch * var_vgef2_dn15) - (((3.24e17 * var_vtv_dn15) * assign30570_e48312) + (assign30570_e48279 * assign30570_e48312_d_n15))) - (((3.24e17 * var_vtv_dn15) * assign30570_e48350) + (assign30570_e48317 * assign30570_e48350_d_n15))), (((var_cch * var_vgef2_dn16) - (((3.24e17 * var_vtv_dn16) * assign30570_e48312) + (assign30570_e48279 * assign30570_e48312_d_n16))) - (((3.24e17 * var_vtv_dn16) * assign30570_e48350) + (assign30570_e48317 * assign30570_e48350_d_n16))), (((var_cch * var_vgef2_dn17) - (((3.24e17 * var_vtv_dn17) * assign30570_e48312) + (assign30570_e48279 * assign30570_e48312_d_n17))) - (((3.24e17 * var_vtv_dn17) * assign30570_e48350) + (assign30570_e48317 * assign30570_e48350_d_n17))), (((var_cch * var_vgef2_dn18) - (((3.24e17 * var_vtv_dn18) * assign30570_e48312) + (assign30570_e48279 * assign30570_e48312_d_n18))) - (((3.24e17 * var_vtv_dn18) * assign30570_e48350) + (assign30570_e48317 * assign30570_e48350_d_n18))), (((var_cch * var_vgef2_dn19) - (((3.24e17 * var_vtv_dn19) * assign30570_e48312) + (assign30570_e48279 * assign30570_e48312_d_n19))) - (((3.24e17 * var_vtv_dn19) * assign30570_e48350) + (assign30570_e48317 * assign30570_e48350_d_n19))), (((var_cch * var_vgef2_dn20) - (((3.24e17 * var_vtv_dn20) * assign30570_e48312) + (assign30570_e48279 * assign30570_e48312_d_n20))) - (((3.24e17 * var_vtv_dn20) * assign30570_e48350) + (assign30570_e48317 * assign30570_e48350_d_n20))), (((var_cch * var_vgef2_dn21) - (((3.24e17 * var_vtv_dn21) * assign30570_e48312) + (assign30570_e48279 * assign30570_e48312_d_n21))) - (((3.24e17 * var_vtv_dn21) * assign30570_e48350) + (assign30570_e48317 * assign30570_e48350_d_n21))), (((var_cch * var_vgef2_dn22) - (((3.24e17 * var_vtv_dn22) * assign30570_e48312) + (assign30570_e48279 * assign30570_e48312_d_n22))) - (((3.24e17 * var_vtv_dn22) * assign30570_e48350) + (assign30570_e48317 * assign30570_e48350_d_n22))),)
    } else {
        (var_t42, var_t42_dn0, var_t42_dn1, var_t42_dn2, var_t42_dn3, var_t42_dn4, var_t42_dn5, var_t42_dn6, var_t42_dn7, var_t42_dn8, var_t42_dn9, var_t42_dn12, var_t42_dn14, var_t42_dn15, var_t42_dn16, var_t42_dn17, var_t42_dn18, var_t42_dn19, var_t42_dn20, var_t42_dn21, var_t42_dn22,)
    }
};
        var_t42 = assign30570_e48354;
        var_t42_dn0 = assign30570_e48354_d_n0;
        var_t42_dn1 = assign30570_e48354_d_n1;
        var_t42_dn2 = assign30570_e48354_d_n2;
        var_t42_dn3 = assign30570_e48354_d_n3;
        var_t42_dn4 = assign30570_e48354_d_n4;
        var_t42_dn5 = assign30570_e48354_d_n5;
        var_t42_dn6 = assign30570_e48354_d_n6;
        var_t42_dn7 = assign30570_e48354_d_n7;
        var_t42_dn8 = assign30570_e48354_d_n8;
        var_t42_dn9 = assign30570_e48354_d_n9;
        var_t42_dn12 = assign30570_e48354_d_n12;
        var_t42_dn14 = assign30570_e48354_d_n14;
        var_t42_dn15 = assign30570_e48354_d_n15;
        var_t42_dn16 = assign30570_e48354_d_n16;
        var_t42_dn17 = assign30570_e48354_d_n17;
        var_t42_dn18 = assign30570_e48354_d_n18;
        var_t42_dn19 = assign30570_e48354_d_n19;
        var_t42_dn20 = assign30570_e48354_d_n20;
        var_t42_dn21 = assign30570_e48354_d_n21;
        var_t42_dn22 = assign30570_e48354_d_n22;

        let (assign30580_e48370, assign30580_e48370_d_n0, assign30580_e48370_d_n1, assign30580_e48370_d_n2, assign30580_e48370_d_n3, assign30580_e48370_d_n4, assign30580_e48370_d_n5, assign30580_e48370_d_n6, assign30580_e48370_d_n7, assign30580_e48370_d_n8, assign30580_e48370_d_n9, assign30580_e48370_d_n12, assign30580_e48370_d_n14, assign30580_e48370_d_n15, assign30580_e48370_d_n16, assign30580_e48370_d_n17, assign30580_e48370_d_n18, assign30580_e48370_d_n19, assign30580_e48370_d_n20, assign30580_e48370_d_n21, assign30580_e48370_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let assign30580_e48363: f64 = (p.p208 * var_t0);
        let assign30580_e48366: f64 = (-0.3333333333333333);
        let assign30580_e48367: f64 = (var_vgef2).powf(assign30580_e48366);
        let assign30580_e48368: f64 = (assign30580_e48363 * assign30580_e48367);
        (assign30580_e48368, (((p.p208 * var_t0_dn0) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((var_vgef2).powf(assign30580_e48366 - 1.0) * var_vgef2_dn0)) } } else { (assign30580_e48367 * (assign30580_e48366 * (var_vgef2_dn0 / var_vgef2))) })), (((p.p208 * var_t0_dn1) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((var_vgef2).powf(assign30580_e48366 - 1.0) * var_vgef2_dn1)) } } else { (assign30580_e48367 * (assign30580_e48366 * (var_vgef2_dn1 / var_vgef2))) })), (((p.p208 * var_t0_dn2) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((var_vgef2).powf(assign30580_e48366 - 1.0) * var_vgef2_dn2)) } } else { (assign30580_e48367 * (assign30580_e48366 * (var_vgef2_dn2 / var_vgef2))) })), (((p.p208 * var_t0_dn3) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((var_vgef2).powf(assign30580_e48366 - 1.0) * var_vgef2_dn3)) } } else { (assign30580_e48367 * (assign30580_e48366 * (var_vgef2_dn3 / var_vgef2))) })), (((p.p208 * var_t0_dn4) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((var_vgef2).powf(assign30580_e48366 - 1.0) * var_vgef2_dn4)) } } else { (assign30580_e48367 * (assign30580_e48366 * (var_vgef2_dn4 / var_vgef2))) })), (((p.p208 * var_t0_dn5) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((var_vgef2).powf(assign30580_e48366 - 1.0) * var_vgef2_dn5)) } } else { (assign30580_e48367 * (assign30580_e48366 * (var_vgef2_dn5 / var_vgef2))) })), (((p.p208 * var_t0_dn6) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((var_vgef2).powf(assign30580_e48366 - 1.0) * var_vgef2_dn6)) } } else { (assign30580_e48367 * (assign30580_e48366 * (var_vgef2_dn6 / var_vgef2))) })), (((p.p208 * var_t0_dn7) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((var_vgef2).powf(assign30580_e48366 - 1.0) * var_vgef2_dn7)) } } else { (assign30580_e48367 * (assign30580_e48366 * (var_vgef2_dn7 / var_vgef2))) })), (((p.p208 * var_t0_dn8) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((var_vgef2).powf(assign30580_e48366 - 1.0) * var_vgef2_dn8)) } } else { (assign30580_e48367 * (assign30580_e48366 * (var_vgef2_dn8 / var_vgef2))) })), (((p.p208 * var_t0_dn9) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((var_vgef2).powf(assign30580_e48366 - 1.0) * var_vgef2_dn9)) } } else { (assign30580_e48367 * (assign30580_e48366 * (var_vgef2_dn9 / var_vgef2))) })), (((p.p208 * var_t0_dn12) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((var_vgef2).powf(assign30580_e48366 - 1.0) * var_vgef2_dn12)) } } else { (assign30580_e48367 * (assign30580_e48366 * (var_vgef2_dn12 / var_vgef2))) })), (((p.p208 * var_t0_dn14) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((var_vgef2).powf(assign30580_e48366 - 1.0) * var_vgef2_dn14)) } } else { (assign30580_e48367 * (assign30580_e48366 * (var_vgef2_dn14 / var_vgef2))) })), (((p.p208 * var_t0_dn15) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((var_vgef2).powf(assign30580_e48366 - 1.0) * var_vgef2_dn15)) } } else { (assign30580_e48367 * (assign30580_e48366 * (var_vgef2_dn15 / var_vgef2))) })), (((p.p208 * var_t0_dn16) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((var_vgef2).powf(assign30580_e48366 - 1.0) * var_vgef2_dn16)) } } else { (assign30580_e48367 * (assign30580_e48366 * (var_vgef2_dn16 / var_vgef2))) })), (((p.p208 * var_t0_dn17) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((var_vgef2).powf(assign30580_e48366 - 1.0) * var_vgef2_dn17)) } } else { (assign30580_e48367 * (assign30580_e48366 * (var_vgef2_dn17 / var_vgef2))) })), (((p.p208 * var_t0_dn18) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((var_vgef2).powf(assign30580_e48366 - 1.0) * var_vgef2_dn18)) } } else { (assign30580_e48367 * (assign30580_e48366 * (var_vgef2_dn18 / var_vgef2))) })), (((p.p208 * var_t0_dn19) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((var_vgef2).powf(assign30580_e48366 - 1.0) * var_vgef2_dn19)) } } else { (assign30580_e48367 * (assign30580_e48366 * (var_vgef2_dn19 / var_vgef2))) })), (((p.p208 * var_t0_dn20) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((var_vgef2).powf(assign30580_e48366 - 1.0) * var_vgef2_dn20)) } } else { (assign30580_e48367 * (assign30580_e48366 * (var_vgef2_dn20 / var_vgef2))) })), (((p.p208 * var_t0_dn21) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((var_vgef2).powf(assign30580_e48366 - 1.0) * var_vgef2_dn21)) } } else { (assign30580_e48367 * (assign30580_e48366 * (var_vgef2_dn21 / var_vgef2))) })), (((p.p208 * var_t0_dn22) * assign30580_e48367) + (assign30580_e48363 * if 0.0 == 0.0 && ((assign30580_e48366) as f64).is_finite() && ((assign30580_e48366) as f64).fract() == 0.0 { if assign30580_e48366 == 0.0 { 0.0 } else { (assign30580_e48366 * ((var_vgef2).powf(assign30580_e48366 - 1.0) * var_vgef2_dn22)) } } else { (assign30580_e48367 * (assign30580_e48366 * (var_vgef2_dn22 / var_vgef2))) })),)
    } else {
        (var_vgefm213g0, var_vgefm213g0_dn0, var_vgefm213g0_dn1, var_vgefm213g0_dn2, var_vgefm213g0_dn3, var_vgefm213g0_dn4, var_vgefm213g0_dn5, var_vgefm213g0_dn6, var_vgefm213g0_dn7, var_vgefm213g0_dn8, var_vgefm213g0_dn9, var_vgefm213g0_dn12, var_vgefm213g0_dn14, var_vgefm213g0_dn15, var_vgefm213g0_dn16, var_vgefm213g0_dn17, var_vgefm213g0_dn18, var_vgefm213g0_dn19, var_vgefm213g0_dn20, var_vgefm213g0_dn21, var_vgefm213g0_dn22,)
    }
};
        var_vgefm213g0 = assign30580_e48370;
        var_vgefm213g0_dn0 = assign30580_e48370_d_n0;
        var_vgefm213g0_dn1 = assign30580_e48370_d_n1;
        var_vgefm213g0_dn2 = assign30580_e48370_d_n2;
        var_vgefm213g0_dn3 = assign30580_e48370_d_n3;
        var_vgefm213g0_dn4 = assign30580_e48370_d_n4;
        var_vgefm213g0_dn5 = assign30580_e48370_d_n5;
        var_vgefm213g0_dn6 = assign30580_e48370_d_n6;
        var_vgefm213g0_dn7 = assign30580_e48370_d_n7;
        var_vgefm213g0_dn8 = assign30580_e48370_d_n8;
        var_vgefm213g0_dn9 = assign30580_e48370_d_n9;
        var_vgefm213g0_dn12 = assign30580_e48370_d_n12;
        var_vgefm213g0_dn14 = assign30580_e48370_d_n14;
        var_vgefm213g0_dn15 = assign30580_e48370_d_n15;
        var_vgefm213g0_dn16 = assign30580_e48370_d_n16;
        var_vgefm213g0_dn17 = assign30580_e48370_d_n17;
        var_vgefm213g0_dn18 = assign30580_e48370_d_n18;
        var_vgefm213g0_dn19 = assign30580_e48370_d_n19;
        var_vgefm213g0_dn20 = assign30580_e48370_d_n20;
        var_vgefm213g0_dn21 = assign30580_e48370_d_n21;
        var_vgefm213g0_dn22 = assign30580_e48370_d_n22;

        let (assign30590_e48386, assign30590_e48386_d_n0, assign30590_e48386_d_n1, assign30590_e48386_d_n2, assign30590_e48386_d_n3, assign30590_e48386_d_n4, assign30590_e48386_d_n5, assign30590_e48386_d_n6, assign30590_e48386_d_n7, assign30590_e48386_d_n8, assign30590_e48386_d_n9, assign30590_e48386_d_n12, assign30590_e48386_d_n14, assign30590_e48386_d_n15, assign30590_e48386_d_n16, assign30590_e48386_d_n17, assign30590_e48386_d_n18, assign30590_e48386_d_n19, assign30590_e48386_d_n20, assign30590_e48386_d_n21, assign30590_e48386_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let assign30590_e48379: f64 = (p.p209 * var_t0);
        let assign30590_e48382: f64 = (-0.3333333333333333);
        let assign30590_e48383: f64 = (var_vgef2).powf(assign30590_e48382);
        let assign30590_e48384: f64 = (assign30590_e48379 * assign30590_e48383);
        (assign30590_e48384, (((p.p209 * var_t0_dn0) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((var_vgef2).powf(assign30590_e48382 - 1.0) * var_vgef2_dn0)) } } else { (assign30590_e48383 * (assign30590_e48382 * (var_vgef2_dn0 / var_vgef2))) })), (((p.p209 * var_t0_dn1) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((var_vgef2).powf(assign30590_e48382 - 1.0) * var_vgef2_dn1)) } } else { (assign30590_e48383 * (assign30590_e48382 * (var_vgef2_dn1 / var_vgef2))) })), (((p.p209 * var_t0_dn2) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((var_vgef2).powf(assign30590_e48382 - 1.0) * var_vgef2_dn2)) } } else { (assign30590_e48383 * (assign30590_e48382 * (var_vgef2_dn2 / var_vgef2))) })), (((p.p209 * var_t0_dn3) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((var_vgef2).powf(assign30590_e48382 - 1.0) * var_vgef2_dn3)) } } else { (assign30590_e48383 * (assign30590_e48382 * (var_vgef2_dn3 / var_vgef2))) })), (((p.p209 * var_t0_dn4) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((var_vgef2).powf(assign30590_e48382 - 1.0) * var_vgef2_dn4)) } } else { (assign30590_e48383 * (assign30590_e48382 * (var_vgef2_dn4 / var_vgef2))) })), (((p.p209 * var_t0_dn5) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((var_vgef2).powf(assign30590_e48382 - 1.0) * var_vgef2_dn5)) } } else { (assign30590_e48383 * (assign30590_e48382 * (var_vgef2_dn5 / var_vgef2))) })), (((p.p209 * var_t0_dn6) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((var_vgef2).powf(assign30590_e48382 - 1.0) * var_vgef2_dn6)) } } else { (assign30590_e48383 * (assign30590_e48382 * (var_vgef2_dn6 / var_vgef2))) })), (((p.p209 * var_t0_dn7) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((var_vgef2).powf(assign30590_e48382 - 1.0) * var_vgef2_dn7)) } } else { (assign30590_e48383 * (assign30590_e48382 * (var_vgef2_dn7 / var_vgef2))) })), (((p.p209 * var_t0_dn8) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((var_vgef2).powf(assign30590_e48382 - 1.0) * var_vgef2_dn8)) } } else { (assign30590_e48383 * (assign30590_e48382 * (var_vgef2_dn8 / var_vgef2))) })), (((p.p209 * var_t0_dn9) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((var_vgef2).powf(assign30590_e48382 - 1.0) * var_vgef2_dn9)) } } else { (assign30590_e48383 * (assign30590_e48382 * (var_vgef2_dn9 / var_vgef2))) })), (((p.p209 * var_t0_dn12) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((var_vgef2).powf(assign30590_e48382 - 1.0) * var_vgef2_dn12)) } } else { (assign30590_e48383 * (assign30590_e48382 * (var_vgef2_dn12 / var_vgef2))) })), (((p.p209 * var_t0_dn14) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((var_vgef2).powf(assign30590_e48382 - 1.0) * var_vgef2_dn14)) } } else { (assign30590_e48383 * (assign30590_e48382 * (var_vgef2_dn14 / var_vgef2))) })), (((p.p209 * var_t0_dn15) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((var_vgef2).powf(assign30590_e48382 - 1.0) * var_vgef2_dn15)) } } else { (assign30590_e48383 * (assign30590_e48382 * (var_vgef2_dn15 / var_vgef2))) })), (((p.p209 * var_t0_dn16) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((var_vgef2).powf(assign30590_e48382 - 1.0) * var_vgef2_dn16)) } } else { (assign30590_e48383 * (assign30590_e48382 * (var_vgef2_dn16 / var_vgef2))) })), (((p.p209 * var_t0_dn17) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((var_vgef2).powf(assign30590_e48382 - 1.0) * var_vgef2_dn17)) } } else { (assign30590_e48383 * (assign30590_e48382 * (var_vgef2_dn17 / var_vgef2))) })), (((p.p209 * var_t0_dn18) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((var_vgef2).powf(assign30590_e48382 - 1.0) * var_vgef2_dn18)) } } else { (assign30590_e48383 * (assign30590_e48382 * (var_vgef2_dn18 / var_vgef2))) })), (((p.p209 * var_t0_dn19) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((var_vgef2).powf(assign30590_e48382 - 1.0) * var_vgef2_dn19)) } } else { (assign30590_e48383 * (assign30590_e48382 * (var_vgef2_dn19 / var_vgef2))) })), (((p.p209 * var_t0_dn20) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((var_vgef2).powf(assign30590_e48382 - 1.0) * var_vgef2_dn20)) } } else { (assign30590_e48383 * (assign30590_e48382 * (var_vgef2_dn20 / var_vgef2))) })), (((p.p209 * var_t0_dn21) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((var_vgef2).powf(assign30590_e48382 - 1.0) * var_vgef2_dn21)) } } else { (assign30590_e48383 * (assign30590_e48382 * (var_vgef2_dn21 / var_vgef2))) })), (((p.p209 * var_t0_dn22) * assign30590_e48383) + (assign30590_e48379 * if 0.0 == 0.0 && ((assign30590_e48382) as f64).is_finite() && ((assign30590_e48382) as f64).fract() == 0.0 { if assign30590_e48382 == 0.0 { 0.0 } else { (assign30590_e48382 * ((var_vgef2).powf(assign30590_e48382 - 1.0) * var_vgef2_dn22)) } } else { (assign30590_e48383 * (assign30590_e48382 * (var_vgef2_dn22 / var_vgef2))) })),)
    } else {
        (var_vgefm213g1, var_vgefm213g1_dn0, var_vgefm213g1_dn1, var_vgefm213g1_dn2, var_vgefm213g1_dn3, var_vgefm213g1_dn4, var_vgefm213g1_dn5, var_vgefm213g1_dn6, var_vgefm213g1_dn7, var_vgefm213g1_dn8, var_vgefm213g1_dn9, var_vgefm213g1_dn12, var_vgefm213g1_dn14, var_vgefm213g1_dn15, var_vgefm213g1_dn16, var_vgefm213g1_dn17, var_vgefm213g1_dn18, var_vgefm213g1_dn19, var_vgefm213g1_dn20, var_vgefm213g1_dn21, var_vgefm213g1_dn22,)
    }
};
        var_vgefm213g1 = assign30590_e48386;
        var_vgefm213g1_dn0 = assign30590_e48386_d_n0;
        var_vgefm213g1_dn1 = assign30590_e48386_d_n1;
        var_vgefm213g1_dn2 = assign30590_e48386_d_n2;
        var_vgefm213g1_dn3 = assign30590_e48386_d_n3;
        var_vgefm213g1_dn4 = assign30590_e48386_d_n4;
        var_vgefm213g1_dn5 = assign30590_e48386_d_n5;
        var_vgefm213g1_dn6 = assign30590_e48386_d_n6;
        var_vgefm213g1_dn7 = assign30590_e48386_d_n7;
        var_vgefm213g1_dn8 = assign30590_e48386_d_n8;
        var_vgefm213g1_dn9 = assign30590_e48386_d_n9;
        var_vgefm213g1_dn12 = assign30590_e48386_d_n12;
        var_vgefm213g1_dn14 = assign30590_e48386_d_n14;
        var_vgefm213g1_dn15 = assign30590_e48386_d_n15;
        var_vgefm213g1_dn16 = assign30590_e48386_d_n16;
        var_vgefm213g1_dn17 = assign30590_e48386_d_n17;
        var_vgefm213g1_dn18 = assign30590_e48386_d_n18;
        var_vgefm213g1_dn19 = assign30590_e48386_d_n19;
        var_vgefm213g1_dn20 = assign30590_e48386_d_n20;
        var_vgefm213g1_dn21 = assign30590_e48386_d_n21;
        var_vgefm213g1_dn22 = assign30590_e48386_d_n22;

        *var_ef2_slot = var_ef2;
        *var_ef2_dn0_slot = var_ef2_dn0;
        *var_ef2_dn1_slot = var_ef2_dn1;
        *var_ef2_dn12_slot = var_ef2_dn12;
        *var_ef2_dn14_slot = var_ef2_dn14;
        *var_ef2_dn15_slot = var_ef2_dn15;
        *var_ef2_dn16_slot = var_ef2_dn16;
        *var_ef2_dn17_slot = var_ef2_dn17;
        *var_ef2_dn18_slot = var_ef2_dn18;
        *var_ef2_dn19_slot = var_ef2_dn19;
        *var_ef2_dn2_slot = var_ef2_dn2;
        *var_ef2_dn20_slot = var_ef2_dn20;
        *var_ef2_dn21_slot = var_ef2_dn21;
        *var_ef2_dn22_slot = var_ef2_dn22;
        *var_ef2_dn3_slot = var_ef2_dn3;
        *var_ef2_dn4_slot = var_ef2_dn4;
        *var_ef2_dn5_slot = var_ef2_dn5;
        *var_ef2_dn6_slot = var_ef2_dn6;
        *var_ef2_dn7_slot = var_ef2_dn7;
        *var_ef2_dn8_slot = var_ef2_dn8;
        *var_ef2_dn9_slot = var_ef2_dn9;
        *var_t42_slot = var_t42;
        *var_t42_dn0_slot = var_t42_dn0;
        *var_t42_dn1_slot = var_t42_dn1;
        *var_t42_dn12_slot = var_t42_dn12;
        *var_t42_dn14_slot = var_t42_dn14;
        *var_t42_dn15_slot = var_t42_dn15;
        *var_t42_dn16_slot = var_t42_dn16;
        *var_t42_dn17_slot = var_t42_dn17;
        *var_t42_dn18_slot = var_t42_dn18;
        *var_t42_dn19_slot = var_t42_dn19;
        *var_t42_dn2_slot = var_t42_dn2;
        *var_t42_dn20_slot = var_t42_dn20;
        *var_t42_dn21_slot = var_t42_dn21;
        *var_t42_dn22_slot = var_t42_dn22;
        *var_t42_dn3_slot = var_t42_dn3;
        *var_t42_dn4_slot = var_t42_dn4;
        *var_t42_dn5_slot = var_t42_dn5;
        *var_t42_dn6_slot = var_t42_dn6;
        *var_t42_dn7_slot = var_t42_dn7;
        *var_t42_dn8_slot = var_t42_dn8;
        *var_t42_dn9_slot = var_t42_dn9;
        *var_t5_slot = var_t5;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn1_slot = var_t5_dn1;
        *var_t5_dn12_slot = var_t5_dn12;
        *var_t5_dn14_slot = var_t5_dn14;
        *var_t5_dn15_slot = var_t5_dn15;
        *var_t5_dn16_slot = var_t5_dn16;
        *var_t5_dn17_slot = var_t5_dn17;
        *var_t5_dn18_slot = var_t5_dn18;
        *var_t5_dn19_slot = var_t5_dn19;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn20_slot = var_t5_dn20;
        *var_t5_dn21_slot = var_t5_dn21;
        *var_t5_dn22_slot = var_t5_dn22;
        *var_t5_dn3_slot = var_t5_dn3;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t5_dn9_slot = var_t5_dn9;
        *var_t5dg1_slot = var_t5dg1;
        *var_t5dg1_dn0_slot = var_t5dg1_dn0;
        *var_t5dg1_dn1_slot = var_t5dg1_dn1;
        *var_t5dg1_dn12_slot = var_t5dg1_dn12;
        *var_t5dg1_dn14_slot = var_t5dg1_dn14;
        *var_t5dg1_dn15_slot = var_t5dg1_dn15;
        *var_t5dg1_dn16_slot = var_t5dg1_dn16;
        *var_t5dg1_dn17_slot = var_t5dg1_dn17;
        *var_t5dg1_dn18_slot = var_t5dg1_dn18;
        *var_t5dg1_dn19_slot = var_t5dg1_dn19;
        *var_t5dg1_dn2_slot = var_t5dg1_dn2;
        *var_t5dg1_dn20_slot = var_t5dg1_dn20;
        *var_t5dg1_dn21_slot = var_t5dg1_dn21;
        *var_t5dg1_dn22_slot = var_t5dg1_dn22;
        *var_t5dg1_dn3_slot = var_t5dg1_dn3;
        *var_t5dg1_dn4_slot = var_t5dg1_dn4;
        *var_t5dg1_dn5_slot = var_t5dg1_dn5;
        *var_t5dg1_dn6_slot = var_t5dg1_dn6;
        *var_t5dg1_dn7_slot = var_t5dg1_dn7;
        *var_t5dg1_dn8_slot = var_t5dg1_dn8;
        *var_t5dg1_dn9_slot = var_t5dg1_dn9;
        *var_t5ng1_slot = var_t5ng1;
        *var_t5ng1_dn0_slot = var_t5ng1_dn0;
        *var_t5ng1_dn1_slot = var_t5ng1_dn1;
        *var_t5ng1_dn12_slot = var_t5ng1_dn12;
        *var_t5ng1_dn14_slot = var_t5ng1_dn14;
        *var_t5ng1_dn15_slot = var_t5ng1_dn15;
        *var_t5ng1_dn16_slot = var_t5ng1_dn16;
        *var_t5ng1_dn17_slot = var_t5ng1_dn17;
        *var_t5ng1_dn18_slot = var_t5ng1_dn18;
        *var_t5ng1_dn19_slot = var_t5ng1_dn19;
        *var_t5ng1_dn2_slot = var_t5ng1_dn2;
        *var_t5ng1_dn20_slot = var_t5ng1_dn20;
        *var_t5ng1_dn21_slot = var_t5ng1_dn21;
        *var_t5ng1_dn22_slot = var_t5ng1_dn22;
        *var_t5ng1_dn3_slot = var_t5ng1_dn3;
        *var_t5ng1_dn4_slot = var_t5ng1_dn4;
        *var_t5ng1_dn5_slot = var_t5ng1_dn5;
        *var_t5ng1_dn6_slot = var_t5ng1_dn6;
        *var_t5ng1_dn7_slot = var_t5ng1_dn7;
        *var_t5ng1_dn8_slot = var_t5ng1_dn8;
        *var_t5ng1_dn9_slot = var_t5ng1_dn9;
        *var_tg02_slot = var_tg02;
        *var_tg02_dn0_slot = var_tg02_dn0;
        *var_tg02_dn1_slot = var_tg02_dn1;
        *var_tg02_dn12_slot = var_tg02_dn12;
        *var_tg02_dn14_slot = var_tg02_dn14;
        *var_tg02_dn15_slot = var_tg02_dn15;
        *var_tg02_dn16_slot = var_tg02_dn16;
        *var_tg02_dn17_slot = var_tg02_dn17;
        *var_tg02_dn18_slot = var_tg02_dn18;
        *var_tg02_dn19_slot = var_tg02_dn19;
        *var_tg02_dn2_slot = var_tg02_dn2;
        *var_tg02_dn20_slot = var_tg02_dn20;
        *var_tg02_dn21_slot = var_tg02_dn21;
        *var_tg02_dn22_slot = var_tg02_dn22;
        *var_tg02_dn3_slot = var_tg02_dn3;
        *var_tg02_dn4_slot = var_tg02_dn4;
        *var_tg02_dn5_slot = var_tg02_dn5;
        *var_tg02_dn6_slot = var_tg02_dn6;
        *var_tg02_dn7_slot = var_tg02_dn7;
        *var_tg02_dn8_slot = var_tg02_dn8;
        *var_tg02_dn9_slot = var_tg02_dn9;
        *var_tg12_slot = var_tg12;
        *var_tg12_dn0_slot = var_tg12_dn0;
        *var_tg12_dn1_slot = var_tg12_dn1;
        *var_tg12_dn12_slot = var_tg12_dn12;
        *var_tg12_dn14_slot = var_tg12_dn14;
        *var_tg12_dn15_slot = var_tg12_dn15;
        *var_tg12_dn16_slot = var_tg12_dn16;
        *var_tg12_dn17_slot = var_tg12_dn17;
        *var_tg12_dn18_slot = var_tg12_dn18;
        *var_tg12_dn19_slot = var_tg12_dn19;
        *var_tg12_dn2_slot = var_tg12_dn2;
        *var_tg12_dn20_slot = var_tg12_dn20;
        *var_tg12_dn21_slot = var_tg12_dn21;
        *var_tg12_dn22_slot = var_tg12_dn22;
        *var_tg12_dn3_slot = var_tg12_dn3;
        *var_tg12_dn4_slot = var_tg12_dn4;
        *var_tg12_dn5_slot = var_tg12_dn5;
        *var_tg12_dn6_slot = var_tg12_dn6;
        *var_tg12_dn7_slot = var_tg12_dn7;
        *var_tg12_dn8_slot = var_tg12_dn8;
        *var_tg12_dn9_slot = var_tg12_dn9;
        *var_vgef2_slot = var_vgef2;
        *var_vgef223g0_slot = var_vgef223g0;
        *var_vgef223g0_dn0_slot = var_vgef223g0_dn0;
        *var_vgef223g0_dn1_slot = var_vgef223g0_dn1;
        *var_vgef223g0_dn12_slot = var_vgef223g0_dn12;
        *var_vgef223g0_dn14_slot = var_vgef223g0_dn14;
        *var_vgef223g0_dn15_slot = var_vgef223g0_dn15;
        *var_vgef223g0_dn16_slot = var_vgef223g0_dn16;
        *var_vgef223g0_dn17_slot = var_vgef223g0_dn17;
        *var_vgef223g0_dn18_slot = var_vgef223g0_dn18;
        *var_vgef223g0_dn19_slot = var_vgef223g0_dn19;
        *var_vgef223g0_dn2_slot = var_vgef223g0_dn2;
        *var_vgef223g0_dn20_slot = var_vgef223g0_dn20;
        *var_vgef223g0_dn21_slot = var_vgef223g0_dn21;
        *var_vgef223g0_dn22_slot = var_vgef223g0_dn22;
        *var_vgef223g0_dn3_slot = var_vgef223g0_dn3;
        *var_vgef223g0_dn4_slot = var_vgef223g0_dn4;
        *var_vgef223g0_dn5_slot = var_vgef223g0_dn5;
        *var_vgef223g0_dn6_slot = var_vgef223g0_dn6;
        *var_vgef223g0_dn7_slot = var_vgef223g0_dn7;
        *var_vgef223g0_dn8_slot = var_vgef223g0_dn8;
        *var_vgef223g0_dn9_slot = var_vgef223g0_dn9;
        *var_vgef223g1_slot = var_vgef223g1;
        *var_vgef223g1_dn0_slot = var_vgef223g1_dn0;
        *var_vgef223g1_dn1_slot = var_vgef223g1_dn1;
        *var_vgef223g1_dn12_slot = var_vgef223g1_dn12;
        *var_vgef223g1_dn14_slot = var_vgef223g1_dn14;
        *var_vgef223g1_dn15_slot = var_vgef223g1_dn15;
        *var_vgef223g1_dn16_slot = var_vgef223g1_dn16;
        *var_vgef223g1_dn17_slot = var_vgef223g1_dn17;
        *var_vgef223g1_dn18_slot = var_vgef223g1_dn18;
        *var_vgef223g1_dn19_slot = var_vgef223g1_dn19;
        *var_vgef223g1_dn2_slot = var_vgef223g1_dn2;
        *var_vgef223g1_dn20_slot = var_vgef223g1_dn20;
        *var_vgef223g1_dn21_slot = var_vgef223g1_dn21;
        *var_vgef223g1_dn22_slot = var_vgef223g1_dn22;
        *var_vgef223g1_dn3_slot = var_vgef223g1_dn3;
        *var_vgef223g1_dn4_slot = var_vgef223g1_dn4;
        *var_vgef223g1_dn5_slot = var_vgef223g1_dn5;
        *var_vgef223g1_dn6_slot = var_vgef223g1_dn6;
        *var_vgef223g1_dn7_slot = var_vgef223g1_dn7;
        *var_vgef223g1_dn8_slot = var_vgef223g1_dn8;
        *var_vgef223g1_dn9_slot = var_vgef223g1_dn9;
        *var_vgef2_dn0_slot = var_vgef2_dn0;
        *var_vgef2_dn1_slot = var_vgef2_dn1;
        *var_vgef2_dn12_slot = var_vgef2_dn12;
        *var_vgef2_dn14_slot = var_vgef2_dn14;
        *var_vgef2_dn15_slot = var_vgef2_dn15;
        *var_vgef2_dn16_slot = var_vgef2_dn16;
        *var_vgef2_dn17_slot = var_vgef2_dn17;
        *var_vgef2_dn18_slot = var_vgef2_dn18;
        *var_vgef2_dn19_slot = var_vgef2_dn19;
        *var_vgef2_dn2_slot = var_vgef2_dn2;
        *var_vgef2_dn20_slot = var_vgef2_dn20;
        *var_vgef2_dn21_slot = var_vgef2_dn21;
        *var_vgef2_dn22_slot = var_vgef2_dn22;
        *var_vgef2_dn3_slot = var_vgef2_dn3;
        *var_vgef2_dn4_slot = var_vgef2_dn4;
        *var_vgef2_dn5_slot = var_vgef2_dn5;
        *var_vgef2_dn6_slot = var_vgef2_dn6;
        *var_vgef2_dn7_slot = var_vgef2_dn7;
        *var_vgef2_dn8_slot = var_vgef2_dn8;
        *var_vgef2_dn9_slot = var_vgef2_dn9;
        *var_vgefm213g0_slot = var_vgefm213g0;
        *var_vgefm213g0_dn0_slot = var_vgefm213g0_dn0;
        *var_vgefm213g0_dn1_slot = var_vgefm213g0_dn1;
        *var_vgefm213g0_dn12_slot = var_vgefm213g0_dn12;
        *var_vgefm213g0_dn14_slot = var_vgefm213g0_dn14;
        *var_vgefm213g0_dn15_slot = var_vgefm213g0_dn15;
        *var_vgefm213g0_dn16_slot = var_vgefm213g0_dn16;
        *var_vgefm213g0_dn17_slot = var_vgefm213g0_dn17;
        *var_vgefm213g0_dn18_slot = var_vgefm213g0_dn18;
        *var_vgefm213g0_dn19_slot = var_vgefm213g0_dn19;
        *var_vgefm213g0_dn2_slot = var_vgefm213g0_dn2;
        *var_vgefm213g0_dn20_slot = var_vgefm213g0_dn20;
        *var_vgefm213g0_dn21_slot = var_vgefm213g0_dn21;
        *var_vgefm213g0_dn22_slot = var_vgefm213g0_dn22;
        *var_vgefm213g0_dn3_slot = var_vgefm213g0_dn3;
        *var_vgefm213g0_dn4_slot = var_vgefm213g0_dn4;
        *var_vgefm213g0_dn5_slot = var_vgefm213g0_dn5;
        *var_vgefm213g0_dn6_slot = var_vgefm213g0_dn6;
        *var_vgefm213g0_dn7_slot = var_vgefm213g0_dn7;
        *var_vgefm213g0_dn8_slot = var_vgefm213g0_dn8;
        *var_vgefm213g0_dn9_slot = var_vgefm213g0_dn9;
        *var_vgefm213g1_slot = var_vgefm213g1;
        *var_vgefm213g1_dn0_slot = var_vgefm213g1_dn0;
        *var_vgefm213g1_dn1_slot = var_vgefm213g1_dn1;
        *var_vgefm213g1_dn12_slot = var_vgefm213g1_dn12;
        *var_vgefm213g1_dn14_slot = var_vgefm213g1_dn14;
        *var_vgefm213g1_dn15_slot = var_vgefm213g1_dn15;
        *var_vgefm213g1_dn16_slot = var_vgefm213g1_dn16;
        *var_vgefm213g1_dn17_slot = var_vgefm213g1_dn17;
        *var_vgefm213g1_dn18_slot = var_vgefm213g1_dn18;
        *var_vgefm213g1_dn19_slot = var_vgefm213g1_dn19;
        *var_vgefm213g1_dn2_slot = var_vgefm213g1_dn2;
        *var_vgefm213g1_dn20_slot = var_vgefm213g1_dn20;
        *var_vgefm213g1_dn21_slot = var_vgefm213g1_dn21;
        *var_vgefm213g1_dn22_slot = var_vgefm213g1_dn22;
        *var_vgefm213g1_dn3_slot = var_vgefm213g1_dn3;
        *var_vgefm213g1_dn4_slot = var_vgefm213g1_dn4;
        *var_vgefm213g1_dn5_slot = var_vgefm213g1_dn5;
        *var_vgefm213g1_dn6_slot = var_vgefm213g1_dn6;
        *var_vgefm213g1_dn7_slot = var_vgefm213g1_dn7;
        *var_vgefm213g1_dn8_slot = var_vgefm213g1_dn8;
        *var_vgefm213g1_dn9_slot = var_vgefm213g1_dn9;
    }

    pub(super) fn stamp_transient_block_180(
        p: &Parameters,
        var_cch: f64,
        var_cg_fp4s: f64,
        var_ef1: f64,
        var_ef1_dn0: f64,
        var_ef1_dn1: f64,
        var_ef1_dn12: f64,
        var_ef1_dn14: f64,
        var_ef1_dn15: f64,
        var_ef1_dn16: f64,
        var_ef1_dn17: f64,
        var_ef1_dn18: f64,
        var_ef1_dn19: f64,
        var_ef1_dn2: f64,
        var_ef1_dn20: f64,
        var_ef1_dn21: f64,
        var_ef1_dn22: f64,
        var_ef1_dn3: f64,
        var_ef1_dn4: f64,
        var_ef1_dn5: f64,
        var_ef1_dn6: f64,
        var_ef1_dn7: f64,
        var_ef1_dn8: f64,
        var_ef1_dn9: f64,
        var_ef2: f64,
        var_ef2_dn0: f64,
        var_ef2_dn1: f64,
        var_ef2_dn12: f64,
        var_ef2_dn14: f64,
        var_ef2_dn15: f64,
        var_ef2_dn16: f64,
        var_ef2_dn17: f64,
        var_ef2_dn18: f64,
        var_ef2_dn19: f64,
        var_ef2_dn2: f64,
        var_ef2_dn20: f64,
        var_ef2_dn21: f64,
        var_ef2_dn22: f64,
        var_ef2_dn3: f64,
        var_ef2_dn4: f64,
        var_ef2_dn5: f64,
        var_ef2_dn6: f64,
        var_ef2_dn7: f64,
        var_ef2_dn8: f64,
        var_ef2_dn9: f64,
        var_guard504: f64,
        var_guard513: f64,
        var_guard518: f64,
        var_psis_fp4s: f64,
        var_psis_fp4s_dn0: f64,
        var_psis_fp4s_dn1: f64,
        var_psis_fp4s_dn12: f64,
        var_psis_fp4s_dn14: f64,
        var_psis_fp4s_dn15: f64,
        var_psis_fp4s_dn16: f64,
        var_psis_fp4s_dn17: f64,
        var_psis_fp4s_dn18: f64,
        var_psis_fp4s_dn19: f64,
        var_psis_fp4s_dn2: f64,
        var_psis_fp4s_dn20: f64,
        var_psis_fp4s_dn21: f64,
        var_psis_fp4s_dn22: f64,
        var_psis_fp4s_dn3: f64,
        var_psis_fp4s_dn4: f64,
        var_psis_fp4s_dn5: f64,
        var_psis_fp4s_dn6: f64,
        var_psis_fp4s_dn7: f64,
        var_psis_fp4s_dn8: f64,
        var_psis_fp4s_dn9: f64,
        var_t42: f64,
        var_t42_dn0: f64,
        var_t42_dn1: f64,
        var_t42_dn12: f64,
        var_t42_dn14: f64,
        var_t42_dn15: f64,
        var_t42_dn16: f64,
        var_t42_dn17: f64,
        var_t42_dn18: f64,
        var_t42_dn19: f64,
        var_t42_dn2: f64,
        var_t42_dn20: f64,
        var_t42_dn21: f64,
        var_t42_dn22: f64,
        var_t42_dn3: f64,
        var_t42_dn4: f64,
        var_t42_dn5: f64,
        var_t42_dn6: f64,
        var_t42_dn7: f64,
        var_t42_dn8: f64,
        var_t42_dn9: f64,
        var_tg02: f64,
        var_tg02_dn0: f64,
        var_tg02_dn1: f64,
        var_tg02_dn12: f64,
        var_tg02_dn14: f64,
        var_tg02_dn15: f64,
        var_tg02_dn16: f64,
        var_tg02_dn17: f64,
        var_tg02_dn18: f64,
        var_tg02_dn19: f64,
        var_tg02_dn2: f64,
        var_tg02_dn20: f64,
        var_tg02_dn21: f64,
        var_tg02_dn22: f64,
        var_tg02_dn3: f64,
        var_tg02_dn4: f64,
        var_tg02_dn5: f64,
        var_tg02_dn6: f64,
        var_tg02_dn7: f64,
        var_tg02_dn8: f64,
        var_tg02_dn9: f64,
        var_tg12: f64,
        var_tg12_dn0: f64,
        var_tg12_dn1: f64,
        var_tg12_dn12: f64,
        var_tg12_dn14: f64,
        var_tg12_dn15: f64,
        var_tg12_dn16: f64,
        var_tg12_dn17: f64,
        var_tg12_dn18: f64,
        var_tg12_dn19: f64,
        var_tg12_dn2: f64,
        var_tg12_dn20: f64,
        var_tg12_dn21: f64,
        var_tg12_dn22: f64,
        var_tg12_dn3: f64,
        var_tg12_dn4: f64,
        var_tg12_dn5: f64,
        var_tg12_dn6: f64,
        var_tg12_dn7: f64,
        var_tg12_dn8: f64,
        var_tg12_dn9: f64,
        var_vdeff: f64,
        var_vdeff_dn0: f64,
        var_vdeff_dn1: f64,
        var_vdeff_dn12: f64,
        var_vdeff_dn14: f64,
        var_vdeff_dn15: f64,
        var_vdeff_dn16: f64,
        var_vdeff_dn17: f64,
        var_vdeff_dn18: f64,
        var_vdeff_dn19: f64,
        var_vdeff_dn2: f64,
        var_vdeff_dn20: f64,
        var_vdeff_dn21: f64,
        var_vdeff_dn22: f64,
        var_vdeff_dn3: f64,
        var_vdeff_dn4: f64,
        var_vdeff_dn5: f64,
        var_vdeff_dn6: f64,
        var_vdeff_dn7: f64,
        var_vdeff_dn8: f64,
        var_vdeff_dn9: f64,
        var_vg0_fp4s: f64,
        var_vg0_fp4s_dn0: f64,
        var_vg0_fp4s_dn1: f64,
        var_vg0_fp4s_dn12: f64,
        var_vg0_fp4s_dn14: f64,
        var_vg0_fp4s_dn15: f64,
        var_vg0_fp4s_dn16: f64,
        var_vg0_fp4s_dn17: f64,
        var_vg0_fp4s_dn18: f64,
        var_vg0_fp4s_dn19: f64,
        var_vg0_fp4s_dn2: f64,
        var_vg0_fp4s_dn20: f64,
        var_vg0_fp4s_dn21: f64,
        var_vg0_fp4s_dn22: f64,
        var_vg0_fp4s_dn3: f64,
        var_vg0_fp4s_dn4: f64,
        var_vg0_fp4s_dn5: f64,
        var_vg0_fp4s_dn6: f64,
        var_vg0_fp4s_dn7: f64,
        var_vg0_fp4s_dn8: f64,
        var_vg0_fp4s_dn9: f64,
        var_vgefm213g0: f64,
        var_vgefm213g0_dn0: f64,
        var_vgefm213g0_dn1: f64,
        var_vgefm213g0_dn12: f64,
        var_vgefm213g0_dn14: f64,
        var_vgefm213g0_dn15: f64,
        var_vgefm213g0_dn16: f64,
        var_vgefm213g0_dn17: f64,
        var_vgefm213g0_dn18: f64,
        var_vgefm213g0_dn19: f64,
        var_vgefm213g0_dn2: f64,
        var_vgefm213g0_dn20: f64,
        var_vgefm213g0_dn21: f64,
        var_vgefm213g0_dn22: f64,
        var_vgefm213g0_dn3: f64,
        var_vgefm213g0_dn4: f64,
        var_vgefm213g0_dn5: f64,
        var_vgefm213g0_dn6: f64,
        var_vgefm213g0_dn7: f64,
        var_vgefm213g0_dn8: f64,
        var_vgefm213g0_dn9: f64,
        var_vgefm213g1: f64,
        var_vgefm213g1_dn0: f64,
        var_vgefm213g1_dn1: f64,
        var_vgefm213g1_dn12: f64,
        var_vgefm213g1_dn14: f64,
        var_vgefm213g1_dn15: f64,
        var_vgefm213g1_dn16: f64,
        var_vgefm213g1_dn17: f64,
        var_vgefm213g1_dn18: f64,
        var_vgefm213g1_dn19: f64,
        var_vgefm213g1_dn2: f64,
        var_vgefm213g1_dn20: f64,
        var_vgefm213g1_dn21: f64,
        var_vgefm213g1_dn22: f64,
        var_vgefm213g1_dn3: f64,
        var_vgefm213g1_dn4: f64,
        var_vgefm213g1_dn5: f64,
        var_vgefm213g1_dn6: f64,
        var_vgefm213g1_dn7: f64,
        var_vgefm213g1_dn8: f64,
        var_vgefm213g1_dn9: f64,
        var_vtv: f64,
        var_vtv_dn15: f64,
        var_vtv_dn16: f64,
        var_vtv_dn17: f64,
        var_vtv_dn18: f64,
        var_vtv_dn19: f64,
        var_vtv_dn20: f64,
        var_vtv_dn21: f64,
        var_vtv_dn22: f64,
        var_vtv_dn4: f64,
        var_vtv_dn6: f64,
        var_vtv_dn7: f64,
        var_vtv_dn8: f64,
        var_ef3_slot: &mut f64,
        var_ef3_dn0_slot: &mut f64,
        var_ef3_dn1_slot: &mut f64,
        var_ef3_dn12_slot: &mut f64,
        var_ef3_dn14_slot: &mut f64,
        var_ef3_dn15_slot: &mut f64,
        var_ef3_dn16_slot: &mut f64,
        var_ef3_dn17_slot: &mut f64,
        var_ef3_dn18_slot: &mut f64,
        var_ef3_dn19_slot: &mut f64,
        var_ef3_dn2_slot: &mut f64,
        var_ef3_dn20_slot: &mut f64,
        var_ef3_dn21_slot: &mut f64,
        var_ef3_dn22_slot: &mut f64,
        var_ef3_dn3_slot: &mut f64,
        var_ef3_dn4_slot: &mut f64,
        var_ef3_dn5_slot: &mut f64,
        var_ef3_dn6_slot: &mut f64,
        var_ef3_dn7_slot: &mut f64,
        var_ef3_dn8_slot: &mut f64,
        var_ef3_dn9_slot: &mut f64,
        var_psid_fp4s_slot: &mut f64,
        var_psid_fp4s_dn0_slot: &mut f64,
        var_psid_fp4s_dn1_slot: &mut f64,
        var_psid_fp4s_dn12_slot: &mut f64,
        var_psid_fp4s_dn14_slot: &mut f64,
        var_psid_fp4s_dn15_slot: &mut f64,
        var_psid_fp4s_dn16_slot: &mut f64,
        var_psid_fp4s_dn17_slot: &mut f64,
        var_psid_fp4s_dn18_slot: &mut f64,
        var_psid_fp4s_dn19_slot: &mut f64,
        var_psid_fp4s_dn2_slot: &mut f64,
        var_psid_fp4s_dn20_slot: &mut f64,
        var_psid_fp4s_dn21_slot: &mut f64,
        var_psid_fp4s_dn22_slot: &mut f64,
        var_psid_fp4s_dn3_slot: &mut f64,
        var_psid_fp4s_dn4_slot: &mut f64,
        var_psid_fp4s_dn5_slot: &mut f64,
        var_psid_fp4s_dn6_slot: &mut f64,
        var_psid_fp4s_dn7_slot: &mut f64,
        var_psid_fp4s_dn8_slot: &mut f64,
        var_psid_fp4s_dn9_slot: &mut f64,
        var_psim_fp4s_slot: &mut f64,
        var_psim_fp4s_dn0_slot: &mut f64,
        var_psim_fp4s_dn1_slot: &mut f64,
        var_psim_fp4s_dn12_slot: &mut f64,
        var_psim_fp4s_dn14_slot: &mut f64,
        var_psim_fp4s_dn15_slot: &mut f64,
        var_psim_fp4s_dn16_slot: &mut f64,
        var_psim_fp4s_dn17_slot: &mut f64,
        var_psim_fp4s_dn18_slot: &mut f64,
        var_psim_fp4s_dn19_slot: &mut f64,
        var_psim_fp4s_dn2_slot: &mut f64,
        var_psim_fp4s_dn20_slot: &mut f64,
        var_psim_fp4s_dn21_slot: &mut f64,
        var_psim_fp4s_dn22_slot: &mut f64,
        var_psim_fp4s_dn3_slot: &mut f64,
        var_psim_fp4s_dn4_slot: &mut f64,
        var_psim_fp4s_dn5_slot: &mut f64,
        var_psim_fp4s_dn6_slot: &mut f64,
        var_psim_fp4s_dn7_slot: &mut f64,
        var_psim_fp4s_dn8_slot: &mut f64,
        var_psim_fp4s_dn9_slot: &mut f64,
        var_psisd_fp4s_slot: &mut f64,
        var_psisd_fp4s_dn0_slot: &mut f64,
        var_psisd_fp4s_dn1_slot: &mut f64,
        var_psisd_fp4s_dn12_slot: &mut f64,
        var_psisd_fp4s_dn14_slot: &mut f64,
        var_psisd_fp4s_dn15_slot: &mut f64,
        var_psisd_fp4s_dn16_slot: &mut f64,
        var_psisd_fp4s_dn17_slot: &mut f64,
        var_psisd_fp4s_dn18_slot: &mut f64,
        var_psisd_fp4s_dn19_slot: &mut f64,
        var_psisd_fp4s_dn2_slot: &mut f64,
        var_psisd_fp4s_dn20_slot: &mut f64,
        var_psisd_fp4s_dn21_slot: &mut f64,
        var_psisd_fp4s_dn22_slot: &mut f64,
        var_psisd_fp4s_dn3_slot: &mut f64,
        var_psisd_fp4s_dn4_slot: &mut f64,
        var_psisd_fp4s_dn5_slot: &mut f64,
        var_psisd_fp4s_dn6_slot: &mut f64,
        var_psisd_fp4s_dn7_slot: &mut f64,
        var_psisd_fp4s_dn8_slot: &mut f64,
        var_psisd_fp4s_dn9_slot: &mut f64,
        var_t0_1_slot: &mut f64,
        var_t0_1_dn0_slot: &mut f64,
        var_t0_1_dn1_slot: &mut f64,
        var_t0_1_dn12_slot: &mut f64,
        var_t0_1_dn14_slot: &mut f64,
        var_t0_1_dn15_slot: &mut f64,
        var_t0_1_dn16_slot: &mut f64,
        var_t0_1_dn17_slot: &mut f64,
        var_t0_1_dn18_slot: &mut f64,
        var_t0_1_dn19_slot: &mut f64,
        var_t0_1_dn2_slot: &mut f64,
        var_t0_1_dn20_slot: &mut f64,
        var_t0_1_dn21_slot: &mut f64,
        var_t0_1_dn22_slot: &mut f64,
        var_t0_1_dn3_slot: &mut f64,
        var_t0_1_dn4_slot: &mut f64,
        var_t0_1_dn5_slot: &mut f64,
        var_t0_1_dn6_slot: &mut f64,
        var_t0_1_dn7_slot: &mut f64,
        var_t0_1_dn8_slot: &mut f64,
        var_t0_1_dn9_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_1_slot: &mut f64,
        var_t1_1_dn0_slot: &mut f64,
        var_t1_1_dn1_slot: &mut f64,
        var_t1_1_dn12_slot: &mut f64,
        var_t1_1_dn14_slot: &mut f64,
        var_t1_1_dn15_slot: &mut f64,
        var_t1_1_dn16_slot: &mut f64,
        var_t1_1_dn17_slot: &mut f64,
        var_t1_1_dn18_slot: &mut f64,
        var_t1_1_dn19_slot: &mut f64,
        var_t1_1_dn2_slot: &mut f64,
        var_t1_1_dn20_slot: &mut f64,
        var_t1_1_dn21_slot: &mut f64,
        var_t1_1_dn22_slot: &mut f64,
        var_t1_1_dn3_slot: &mut f64,
        var_t1_1_dn4_slot: &mut f64,
        var_t1_1_dn5_slot: &mut f64,
        var_t1_1_dn6_slot: &mut f64,
        var_t1_1_dn7_slot: &mut f64,
        var_t1_1_dn8_slot: &mut f64,
        var_t1_1_dn9_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn1_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn15_slot: &mut f64,
        var_t1_dn16_slot: &mut f64,
        var_t1_dn17_slot: &mut f64,
        var_t1_dn18_slot: &mut f64,
        var_t1_dn19_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn20_slot: &mut f64,
        var_t1_dn21_slot: &mut f64,
        var_t1_dn22_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn1_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn15_slot: &mut f64,
        var_t2_dn16_slot: &mut f64,
        var_t2_dn17_slot: &mut f64,
        var_t2_dn18_slot: &mut f64,
        var_t2_dn19_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn20_slot: &mut f64,
        var_t2_dn21_slot: &mut f64,
        var_t2_dn22_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn1_slot: &mut f64,
        var_t3_dn12_slot: &mut f64,
        var_t3_dn14_slot: &mut f64,
        var_t3_dn15_slot: &mut f64,
        var_t3_dn16_slot: &mut f64,
        var_t3_dn17_slot: &mut f64,
        var_t3_dn18_slot: &mut f64,
        var_t3_dn19_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn20_slot: &mut f64,
        var_t3_dn21_slot: &mut f64,
        var_t3_dn22_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t52_slot: &mut f64,
        var_t52_dn0_slot: &mut f64,
        var_t52_dn1_slot: &mut f64,
        var_t52_dn12_slot: &mut f64,
        var_t52_dn14_slot: &mut f64,
        var_t52_dn15_slot: &mut f64,
        var_t52_dn16_slot: &mut f64,
        var_t52_dn17_slot: &mut f64,
        var_t52_dn18_slot: &mut f64,
        var_t52_dn19_slot: &mut f64,
        var_t52_dn2_slot: &mut f64,
        var_t52_dn20_slot: &mut f64,
        var_t52_dn21_slot: &mut f64,
        var_t52_dn22_slot: &mut f64,
        var_t52_dn3_slot: &mut f64,
        var_t52_dn4_slot: &mut f64,
        var_t52_dn5_slot: &mut f64,
        var_t52_dn6_slot: &mut f64,
        var_t52_dn7_slot: &mut f64,
        var_t52_dn8_slot: &mut f64,
        var_t52_dn9_slot: &mut f64,
        var_t5dg02_slot: &mut f64,
        var_t5dg02_dn0_slot: &mut f64,
        var_t5dg02_dn1_slot: &mut f64,
        var_t5dg02_dn12_slot: &mut f64,
        var_t5dg02_dn14_slot: &mut f64,
        var_t5dg02_dn15_slot: &mut f64,
        var_t5dg02_dn16_slot: &mut f64,
        var_t5dg02_dn17_slot: &mut f64,
        var_t5dg02_dn18_slot: &mut f64,
        var_t5dg02_dn19_slot: &mut f64,
        var_t5dg02_dn2_slot: &mut f64,
        var_t5dg02_dn20_slot: &mut f64,
        var_t5dg02_dn21_slot: &mut f64,
        var_t5dg02_dn22_slot: &mut f64,
        var_t5dg02_dn3_slot: &mut f64,
        var_t5dg02_dn4_slot: &mut f64,
        var_t5dg02_dn5_slot: &mut f64,
        var_t5dg02_dn6_slot: &mut f64,
        var_t5dg02_dn7_slot: &mut f64,
        var_t5dg02_dn8_slot: &mut f64,
        var_t5dg02_dn9_slot: &mut f64,
        var_t5dg12_slot: &mut f64,
        var_t5dg12_dn0_slot: &mut f64,
        var_t5dg12_dn1_slot: &mut f64,
        var_t5dg12_dn12_slot: &mut f64,
        var_t5dg12_dn14_slot: &mut f64,
        var_t5dg12_dn15_slot: &mut f64,
        var_t5dg12_dn16_slot: &mut f64,
        var_t5dg12_dn17_slot: &mut f64,
        var_t5dg12_dn18_slot: &mut f64,
        var_t5dg12_dn19_slot: &mut f64,
        var_t5dg12_dn2_slot: &mut f64,
        var_t5dg12_dn20_slot: &mut f64,
        var_t5dg12_dn21_slot: &mut f64,
        var_t5dg12_dn22_slot: &mut f64,
        var_t5dg12_dn3_slot: &mut f64,
        var_t5dg12_dn4_slot: &mut f64,
        var_t5dg12_dn5_slot: &mut f64,
        var_t5dg12_dn6_slot: &mut f64,
        var_t5dg12_dn7_slot: &mut f64,
        var_t5dg12_dn8_slot: &mut f64,
        var_t5dg12_dn9_slot: &mut f64,
        var_t5ng02_slot: &mut f64,
        var_t5ng02_dn0_slot: &mut f64,
        var_t5ng02_dn1_slot: &mut f64,
        var_t5ng02_dn12_slot: &mut f64,
        var_t5ng02_dn14_slot: &mut f64,
        var_t5ng02_dn15_slot: &mut f64,
        var_t5ng02_dn16_slot: &mut f64,
        var_t5ng02_dn17_slot: &mut f64,
        var_t5ng02_dn18_slot: &mut f64,
        var_t5ng02_dn19_slot: &mut f64,
        var_t5ng02_dn2_slot: &mut f64,
        var_t5ng02_dn20_slot: &mut f64,
        var_t5ng02_dn21_slot: &mut f64,
        var_t5ng02_dn22_slot: &mut f64,
        var_t5ng02_dn3_slot: &mut f64,
        var_t5ng02_dn4_slot: &mut f64,
        var_t5ng02_dn5_slot: &mut f64,
        var_t5ng02_dn6_slot: &mut f64,
        var_t5ng02_dn7_slot: &mut f64,
        var_t5ng02_dn8_slot: &mut f64,
        var_t5ng02_dn9_slot: &mut f64,
        var_t5ng12_slot: &mut f64,
        var_t5ng12_dn0_slot: &mut f64,
        var_t5ng12_dn1_slot: &mut f64,
        var_t5ng12_dn12_slot: &mut f64,
        var_t5ng12_dn14_slot: &mut f64,
        var_t5ng12_dn15_slot: &mut f64,
        var_t5ng12_dn16_slot: &mut f64,
        var_t5ng12_dn17_slot: &mut f64,
        var_t5ng12_dn18_slot: &mut f64,
        var_t5ng12_dn19_slot: &mut f64,
        var_t5ng12_dn2_slot: &mut f64,
        var_t5ng12_dn20_slot: &mut f64,
        var_t5ng12_dn21_slot: &mut f64,
        var_t5ng12_dn22_slot: &mut f64,
        var_t5ng12_dn3_slot: &mut f64,
        var_t5ng12_dn4_slot: &mut f64,
        var_t5ng12_dn5_slot: &mut f64,
        var_t5ng12_dn6_slot: &mut f64,
        var_t5ng12_dn7_slot: &mut f64,
        var_t5ng12_dn8_slot: &mut f64,
        var_t5ng12_dn9_slot: &mut f64,
        var_xdcinv_slot: &mut f64,
        var_xdcinv_dn0_slot: &mut f64,
        var_xdcinv_dn1_slot: &mut f64,
        var_xdcinv_dn12_slot: &mut f64,
        var_xdcinv_dn14_slot: &mut f64,
        var_xdcinv_dn15_slot: &mut f64,
        var_xdcinv_dn16_slot: &mut f64,
        var_xdcinv_dn17_slot: &mut f64,
        var_xdcinv_dn18_slot: &mut f64,
        var_xdcinv_dn19_slot: &mut f64,
        var_xdcinv_dn2_slot: &mut f64,
        var_xdcinv_dn20_slot: &mut f64,
        var_xdcinv_dn21_slot: &mut f64,
        var_xdcinv_dn22_slot: &mut f64,
        var_xdcinv_dn3_slot: &mut f64,
        var_xdcinv_dn4_slot: &mut f64,
        var_xdcinv_dn5_slot: &mut f64,
        var_xdcinv_dn6_slot: &mut f64,
        var_xdcinv_dn7_slot: &mut f64,
        var_xdcinv_dn8_slot: &mut f64,
        var_xdcinv_dn9_slot: &mut f64,
    ) {
        let mut var_ef3: f64 = *var_ef3_slot;
        let mut var_ef3_dn0: f64 = *var_ef3_dn0_slot;
        let mut var_ef3_dn1: f64 = *var_ef3_dn1_slot;
        let mut var_ef3_dn12: f64 = *var_ef3_dn12_slot;
        let mut var_ef3_dn14: f64 = *var_ef3_dn14_slot;
        let mut var_ef3_dn15: f64 = *var_ef3_dn15_slot;
        let mut var_ef3_dn16: f64 = *var_ef3_dn16_slot;
        let mut var_ef3_dn17: f64 = *var_ef3_dn17_slot;
        let mut var_ef3_dn18: f64 = *var_ef3_dn18_slot;
        let mut var_ef3_dn19: f64 = *var_ef3_dn19_slot;
        let mut var_ef3_dn2: f64 = *var_ef3_dn2_slot;
        let mut var_ef3_dn20: f64 = *var_ef3_dn20_slot;
        let mut var_ef3_dn21: f64 = *var_ef3_dn21_slot;
        let mut var_ef3_dn22: f64 = *var_ef3_dn22_slot;
        let mut var_ef3_dn3: f64 = *var_ef3_dn3_slot;
        let mut var_ef3_dn4: f64 = *var_ef3_dn4_slot;
        let mut var_ef3_dn5: f64 = *var_ef3_dn5_slot;
        let mut var_ef3_dn6: f64 = *var_ef3_dn6_slot;
        let mut var_ef3_dn7: f64 = *var_ef3_dn7_slot;
        let mut var_ef3_dn8: f64 = *var_ef3_dn8_slot;
        let mut var_ef3_dn9: f64 = *var_ef3_dn9_slot;
        let mut var_psid_fp4s: f64 = *var_psid_fp4s_slot;
        let mut var_psid_fp4s_dn0: f64 = *var_psid_fp4s_dn0_slot;
        let mut var_psid_fp4s_dn1: f64 = *var_psid_fp4s_dn1_slot;
        let mut var_psid_fp4s_dn12: f64 = *var_psid_fp4s_dn12_slot;
        let mut var_psid_fp4s_dn14: f64 = *var_psid_fp4s_dn14_slot;
        let mut var_psid_fp4s_dn15: f64 = *var_psid_fp4s_dn15_slot;
        let mut var_psid_fp4s_dn16: f64 = *var_psid_fp4s_dn16_slot;
        let mut var_psid_fp4s_dn17: f64 = *var_psid_fp4s_dn17_slot;
        let mut var_psid_fp4s_dn18: f64 = *var_psid_fp4s_dn18_slot;
        let mut var_psid_fp4s_dn19: f64 = *var_psid_fp4s_dn19_slot;
        let mut var_psid_fp4s_dn2: f64 = *var_psid_fp4s_dn2_slot;
        let mut var_psid_fp4s_dn20: f64 = *var_psid_fp4s_dn20_slot;
        let mut var_psid_fp4s_dn21: f64 = *var_psid_fp4s_dn21_slot;
        let mut var_psid_fp4s_dn22: f64 = *var_psid_fp4s_dn22_slot;
        let mut var_psid_fp4s_dn3: f64 = *var_psid_fp4s_dn3_slot;
        let mut var_psid_fp4s_dn4: f64 = *var_psid_fp4s_dn4_slot;
        let mut var_psid_fp4s_dn5: f64 = *var_psid_fp4s_dn5_slot;
        let mut var_psid_fp4s_dn6: f64 = *var_psid_fp4s_dn6_slot;
        let mut var_psid_fp4s_dn7: f64 = *var_psid_fp4s_dn7_slot;
        let mut var_psid_fp4s_dn8: f64 = *var_psid_fp4s_dn8_slot;
        let mut var_psid_fp4s_dn9: f64 = *var_psid_fp4s_dn9_slot;
        let mut var_psim_fp4s: f64 = *var_psim_fp4s_slot;
        let mut var_psim_fp4s_dn0: f64 = *var_psim_fp4s_dn0_slot;
        let mut var_psim_fp4s_dn1: f64 = *var_psim_fp4s_dn1_slot;
        let mut var_psim_fp4s_dn12: f64 = *var_psim_fp4s_dn12_slot;
        let mut var_psim_fp4s_dn14: f64 = *var_psim_fp4s_dn14_slot;
        let mut var_psim_fp4s_dn15: f64 = *var_psim_fp4s_dn15_slot;
        let mut var_psim_fp4s_dn16: f64 = *var_psim_fp4s_dn16_slot;
        let mut var_psim_fp4s_dn17: f64 = *var_psim_fp4s_dn17_slot;
        let mut var_psim_fp4s_dn18: f64 = *var_psim_fp4s_dn18_slot;
        let mut var_psim_fp4s_dn19: f64 = *var_psim_fp4s_dn19_slot;
        let mut var_psim_fp4s_dn2: f64 = *var_psim_fp4s_dn2_slot;
        let mut var_psim_fp4s_dn20: f64 = *var_psim_fp4s_dn20_slot;
        let mut var_psim_fp4s_dn21: f64 = *var_psim_fp4s_dn21_slot;
        let mut var_psim_fp4s_dn22: f64 = *var_psim_fp4s_dn22_slot;
        let mut var_psim_fp4s_dn3: f64 = *var_psim_fp4s_dn3_slot;
        let mut var_psim_fp4s_dn4: f64 = *var_psim_fp4s_dn4_slot;
        let mut var_psim_fp4s_dn5: f64 = *var_psim_fp4s_dn5_slot;
        let mut var_psim_fp4s_dn6: f64 = *var_psim_fp4s_dn6_slot;
        let mut var_psim_fp4s_dn7: f64 = *var_psim_fp4s_dn7_slot;
        let mut var_psim_fp4s_dn8: f64 = *var_psim_fp4s_dn8_slot;
        let mut var_psim_fp4s_dn9: f64 = *var_psim_fp4s_dn9_slot;
        let mut var_psisd_fp4s: f64 = *var_psisd_fp4s_slot;
        let mut var_psisd_fp4s_dn0: f64 = *var_psisd_fp4s_dn0_slot;
        let mut var_psisd_fp4s_dn1: f64 = *var_psisd_fp4s_dn1_slot;
        let mut var_psisd_fp4s_dn12: f64 = *var_psisd_fp4s_dn12_slot;
        let mut var_psisd_fp4s_dn14: f64 = *var_psisd_fp4s_dn14_slot;
        let mut var_psisd_fp4s_dn15: f64 = *var_psisd_fp4s_dn15_slot;
        let mut var_psisd_fp4s_dn16: f64 = *var_psisd_fp4s_dn16_slot;
        let mut var_psisd_fp4s_dn17: f64 = *var_psisd_fp4s_dn17_slot;
        let mut var_psisd_fp4s_dn18: f64 = *var_psisd_fp4s_dn18_slot;
        let mut var_psisd_fp4s_dn19: f64 = *var_psisd_fp4s_dn19_slot;
        let mut var_psisd_fp4s_dn2: f64 = *var_psisd_fp4s_dn2_slot;
        let mut var_psisd_fp4s_dn20: f64 = *var_psisd_fp4s_dn20_slot;
        let mut var_psisd_fp4s_dn21: f64 = *var_psisd_fp4s_dn21_slot;
        let mut var_psisd_fp4s_dn22: f64 = *var_psisd_fp4s_dn22_slot;
        let mut var_psisd_fp4s_dn3: f64 = *var_psisd_fp4s_dn3_slot;
        let mut var_psisd_fp4s_dn4: f64 = *var_psisd_fp4s_dn4_slot;
        let mut var_psisd_fp4s_dn5: f64 = *var_psisd_fp4s_dn5_slot;
        let mut var_psisd_fp4s_dn6: f64 = *var_psisd_fp4s_dn6_slot;
        let mut var_psisd_fp4s_dn7: f64 = *var_psisd_fp4s_dn7_slot;
        let mut var_psisd_fp4s_dn8: f64 = *var_psisd_fp4s_dn8_slot;
        let mut var_psisd_fp4s_dn9: f64 = *var_psisd_fp4s_dn9_slot;
        let mut var_t0_1: f64 = *var_t0_1_slot;
        let mut var_t0_1_dn0: f64 = *var_t0_1_dn0_slot;
        let mut var_t0_1_dn1: f64 = *var_t0_1_dn1_slot;
        let mut var_t0_1_dn12: f64 = *var_t0_1_dn12_slot;
        let mut var_t0_1_dn14: f64 = *var_t0_1_dn14_slot;
        let mut var_t0_1_dn15: f64 = *var_t0_1_dn15_slot;
        let mut var_t0_1_dn16: f64 = *var_t0_1_dn16_slot;
        let mut var_t0_1_dn17: f64 = *var_t0_1_dn17_slot;
        let mut var_t0_1_dn18: f64 = *var_t0_1_dn18_slot;
        let mut var_t0_1_dn19: f64 = *var_t0_1_dn19_slot;
        let mut var_t0_1_dn2: f64 = *var_t0_1_dn2_slot;
        let mut var_t0_1_dn20: f64 = *var_t0_1_dn20_slot;
        let mut var_t0_1_dn21: f64 = *var_t0_1_dn21_slot;
        let mut var_t0_1_dn22: f64 = *var_t0_1_dn22_slot;
        let mut var_t0_1_dn3: f64 = *var_t0_1_dn3_slot;
        let mut var_t0_1_dn4: f64 = *var_t0_1_dn4_slot;
        let mut var_t0_1_dn5: f64 = *var_t0_1_dn5_slot;
        let mut var_t0_1_dn6: f64 = *var_t0_1_dn6_slot;
        let mut var_t0_1_dn7: f64 = *var_t0_1_dn7_slot;
        let mut var_t0_1_dn8: f64 = *var_t0_1_dn8_slot;
        let mut var_t0_1_dn9: f64 = *var_t0_1_dn9_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_1: f64 = *var_t1_1_slot;
        let mut var_t1_1_dn0: f64 = *var_t1_1_dn0_slot;
        let mut var_t1_1_dn1: f64 = *var_t1_1_dn1_slot;
        let mut var_t1_1_dn12: f64 = *var_t1_1_dn12_slot;
        let mut var_t1_1_dn14: f64 = *var_t1_1_dn14_slot;
        let mut var_t1_1_dn15: f64 = *var_t1_1_dn15_slot;
        let mut var_t1_1_dn16: f64 = *var_t1_1_dn16_slot;
        let mut var_t1_1_dn17: f64 = *var_t1_1_dn17_slot;
        let mut var_t1_1_dn18: f64 = *var_t1_1_dn18_slot;
        let mut var_t1_1_dn19: f64 = *var_t1_1_dn19_slot;
        let mut var_t1_1_dn2: f64 = *var_t1_1_dn2_slot;
        let mut var_t1_1_dn20: f64 = *var_t1_1_dn20_slot;
        let mut var_t1_1_dn21: f64 = *var_t1_1_dn21_slot;
        let mut var_t1_1_dn22: f64 = *var_t1_1_dn22_slot;
        let mut var_t1_1_dn3: f64 = *var_t1_1_dn3_slot;
        let mut var_t1_1_dn4: f64 = *var_t1_1_dn4_slot;
        let mut var_t1_1_dn5: f64 = *var_t1_1_dn5_slot;
        let mut var_t1_1_dn6: f64 = *var_t1_1_dn6_slot;
        let mut var_t1_1_dn7: f64 = *var_t1_1_dn7_slot;
        let mut var_t1_1_dn8: f64 = *var_t1_1_dn8_slot;
        let mut var_t1_1_dn9: f64 = *var_t1_1_dn9_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn1: f64 = *var_t1_dn1_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn15: f64 = *var_t1_dn15_slot;
        let mut var_t1_dn16: f64 = *var_t1_dn16_slot;
        let mut var_t1_dn17: f64 = *var_t1_dn17_slot;
        let mut var_t1_dn18: f64 = *var_t1_dn18_slot;
        let mut var_t1_dn19: f64 = *var_t1_dn19_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn20: f64 = *var_t1_dn20_slot;
        let mut var_t1_dn21: f64 = *var_t1_dn21_slot;
        let mut var_t1_dn22: f64 = *var_t1_dn22_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn1: f64 = *var_t2_dn1_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn15: f64 = *var_t2_dn15_slot;
        let mut var_t2_dn16: f64 = *var_t2_dn16_slot;
        let mut var_t2_dn17: f64 = *var_t2_dn17_slot;
        let mut var_t2_dn18: f64 = *var_t2_dn18_slot;
        let mut var_t2_dn19: f64 = *var_t2_dn19_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn20: f64 = *var_t2_dn20_slot;
        let mut var_t2_dn21: f64 = *var_t2_dn21_slot;
        let mut var_t2_dn22: f64 = *var_t2_dn22_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn1: f64 = *var_t3_dn1_slot;
        let mut var_t3_dn12: f64 = *var_t3_dn12_slot;
        let mut var_t3_dn14: f64 = *var_t3_dn14_slot;
        let mut var_t3_dn15: f64 = *var_t3_dn15_slot;
        let mut var_t3_dn16: f64 = *var_t3_dn16_slot;
        let mut var_t3_dn17: f64 = *var_t3_dn17_slot;
        let mut var_t3_dn18: f64 = *var_t3_dn18_slot;
        let mut var_t3_dn19: f64 = *var_t3_dn19_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn20: f64 = *var_t3_dn20_slot;
        let mut var_t3_dn21: f64 = *var_t3_dn21_slot;
        let mut var_t3_dn22: f64 = *var_t3_dn22_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t52: f64 = *var_t52_slot;
        let mut var_t52_dn0: f64 = *var_t52_dn0_slot;
        let mut var_t52_dn1: f64 = *var_t52_dn1_slot;
        let mut var_t52_dn12: f64 = *var_t52_dn12_slot;
        let mut var_t52_dn14: f64 = *var_t52_dn14_slot;
        let mut var_t52_dn15: f64 = *var_t52_dn15_slot;
        let mut var_t52_dn16: f64 = *var_t52_dn16_slot;
        let mut var_t52_dn17: f64 = *var_t52_dn17_slot;
        let mut var_t52_dn18: f64 = *var_t52_dn18_slot;
        let mut var_t52_dn19: f64 = *var_t52_dn19_slot;
        let mut var_t52_dn2: f64 = *var_t52_dn2_slot;
        let mut var_t52_dn20: f64 = *var_t52_dn20_slot;
        let mut var_t52_dn21: f64 = *var_t52_dn21_slot;
        let mut var_t52_dn22: f64 = *var_t52_dn22_slot;
        let mut var_t52_dn3: f64 = *var_t52_dn3_slot;
        let mut var_t52_dn4: f64 = *var_t52_dn4_slot;
        let mut var_t52_dn5: f64 = *var_t52_dn5_slot;
        let mut var_t52_dn6: f64 = *var_t52_dn6_slot;
        let mut var_t52_dn7: f64 = *var_t52_dn7_slot;
        let mut var_t52_dn8: f64 = *var_t52_dn8_slot;
        let mut var_t52_dn9: f64 = *var_t52_dn9_slot;
        let mut var_t5dg02: f64 = *var_t5dg02_slot;
        let mut var_t5dg02_dn0: f64 = *var_t5dg02_dn0_slot;
        let mut var_t5dg02_dn1: f64 = *var_t5dg02_dn1_slot;
        let mut var_t5dg02_dn12: f64 = *var_t5dg02_dn12_slot;
        let mut var_t5dg02_dn14: f64 = *var_t5dg02_dn14_slot;
        let mut var_t5dg02_dn15: f64 = *var_t5dg02_dn15_slot;
        let mut var_t5dg02_dn16: f64 = *var_t5dg02_dn16_slot;
        let mut var_t5dg02_dn17: f64 = *var_t5dg02_dn17_slot;
        let mut var_t5dg02_dn18: f64 = *var_t5dg02_dn18_slot;
        let mut var_t5dg02_dn19: f64 = *var_t5dg02_dn19_slot;
        let mut var_t5dg02_dn2: f64 = *var_t5dg02_dn2_slot;
        let mut var_t5dg02_dn20: f64 = *var_t5dg02_dn20_slot;
        let mut var_t5dg02_dn21: f64 = *var_t5dg02_dn21_slot;
        let mut var_t5dg02_dn22: f64 = *var_t5dg02_dn22_slot;
        let mut var_t5dg02_dn3: f64 = *var_t5dg02_dn3_slot;
        let mut var_t5dg02_dn4: f64 = *var_t5dg02_dn4_slot;
        let mut var_t5dg02_dn5: f64 = *var_t5dg02_dn5_slot;
        let mut var_t5dg02_dn6: f64 = *var_t5dg02_dn6_slot;
        let mut var_t5dg02_dn7: f64 = *var_t5dg02_dn7_slot;
        let mut var_t5dg02_dn8: f64 = *var_t5dg02_dn8_slot;
        let mut var_t5dg02_dn9: f64 = *var_t5dg02_dn9_slot;
        let mut var_t5dg12: f64 = *var_t5dg12_slot;
        let mut var_t5dg12_dn0: f64 = *var_t5dg12_dn0_slot;
        let mut var_t5dg12_dn1: f64 = *var_t5dg12_dn1_slot;
        let mut var_t5dg12_dn12: f64 = *var_t5dg12_dn12_slot;
        let mut var_t5dg12_dn14: f64 = *var_t5dg12_dn14_slot;
        let mut var_t5dg12_dn15: f64 = *var_t5dg12_dn15_slot;
        let mut var_t5dg12_dn16: f64 = *var_t5dg12_dn16_slot;
        let mut var_t5dg12_dn17: f64 = *var_t5dg12_dn17_slot;
        let mut var_t5dg12_dn18: f64 = *var_t5dg12_dn18_slot;
        let mut var_t5dg12_dn19: f64 = *var_t5dg12_dn19_slot;
        let mut var_t5dg12_dn2: f64 = *var_t5dg12_dn2_slot;
        let mut var_t5dg12_dn20: f64 = *var_t5dg12_dn20_slot;
        let mut var_t5dg12_dn21: f64 = *var_t5dg12_dn21_slot;
        let mut var_t5dg12_dn22: f64 = *var_t5dg12_dn22_slot;
        let mut var_t5dg12_dn3: f64 = *var_t5dg12_dn3_slot;
        let mut var_t5dg12_dn4: f64 = *var_t5dg12_dn4_slot;
        let mut var_t5dg12_dn5: f64 = *var_t5dg12_dn5_slot;
        let mut var_t5dg12_dn6: f64 = *var_t5dg12_dn6_slot;
        let mut var_t5dg12_dn7: f64 = *var_t5dg12_dn7_slot;
        let mut var_t5dg12_dn8: f64 = *var_t5dg12_dn8_slot;
        let mut var_t5dg12_dn9: f64 = *var_t5dg12_dn9_slot;
        let mut var_t5ng02: f64 = *var_t5ng02_slot;
        let mut var_t5ng02_dn0: f64 = *var_t5ng02_dn0_slot;
        let mut var_t5ng02_dn1: f64 = *var_t5ng02_dn1_slot;
        let mut var_t5ng02_dn12: f64 = *var_t5ng02_dn12_slot;
        let mut var_t5ng02_dn14: f64 = *var_t5ng02_dn14_slot;
        let mut var_t5ng02_dn15: f64 = *var_t5ng02_dn15_slot;
        let mut var_t5ng02_dn16: f64 = *var_t5ng02_dn16_slot;
        let mut var_t5ng02_dn17: f64 = *var_t5ng02_dn17_slot;
        let mut var_t5ng02_dn18: f64 = *var_t5ng02_dn18_slot;
        let mut var_t5ng02_dn19: f64 = *var_t5ng02_dn19_slot;
        let mut var_t5ng02_dn2: f64 = *var_t5ng02_dn2_slot;
        let mut var_t5ng02_dn20: f64 = *var_t5ng02_dn20_slot;
        let mut var_t5ng02_dn21: f64 = *var_t5ng02_dn21_slot;
        let mut var_t5ng02_dn22: f64 = *var_t5ng02_dn22_slot;
        let mut var_t5ng02_dn3: f64 = *var_t5ng02_dn3_slot;
        let mut var_t5ng02_dn4: f64 = *var_t5ng02_dn4_slot;
        let mut var_t5ng02_dn5: f64 = *var_t5ng02_dn5_slot;
        let mut var_t5ng02_dn6: f64 = *var_t5ng02_dn6_slot;
        let mut var_t5ng02_dn7: f64 = *var_t5ng02_dn7_slot;
        let mut var_t5ng02_dn8: f64 = *var_t5ng02_dn8_slot;
        let mut var_t5ng02_dn9: f64 = *var_t5ng02_dn9_slot;
        let mut var_t5ng12: f64 = *var_t5ng12_slot;
        let mut var_t5ng12_dn0: f64 = *var_t5ng12_dn0_slot;
        let mut var_t5ng12_dn1: f64 = *var_t5ng12_dn1_slot;
        let mut var_t5ng12_dn12: f64 = *var_t5ng12_dn12_slot;
        let mut var_t5ng12_dn14: f64 = *var_t5ng12_dn14_slot;
        let mut var_t5ng12_dn15: f64 = *var_t5ng12_dn15_slot;
        let mut var_t5ng12_dn16: f64 = *var_t5ng12_dn16_slot;
        let mut var_t5ng12_dn17: f64 = *var_t5ng12_dn17_slot;
        let mut var_t5ng12_dn18: f64 = *var_t5ng12_dn18_slot;
        let mut var_t5ng12_dn19: f64 = *var_t5ng12_dn19_slot;
        let mut var_t5ng12_dn2: f64 = *var_t5ng12_dn2_slot;
        let mut var_t5ng12_dn20: f64 = *var_t5ng12_dn20_slot;
        let mut var_t5ng12_dn21: f64 = *var_t5ng12_dn21_slot;
        let mut var_t5ng12_dn22: f64 = *var_t5ng12_dn22_slot;
        let mut var_t5ng12_dn3: f64 = *var_t5ng12_dn3_slot;
        let mut var_t5ng12_dn4: f64 = *var_t5ng12_dn4_slot;
        let mut var_t5ng12_dn5: f64 = *var_t5ng12_dn5_slot;
        let mut var_t5ng12_dn6: f64 = *var_t5ng12_dn6_slot;
        let mut var_t5ng12_dn7: f64 = *var_t5ng12_dn7_slot;
        let mut var_t5ng12_dn8: f64 = *var_t5ng12_dn8_slot;
        let mut var_t5ng12_dn9: f64 = *var_t5ng12_dn9_slot;
        let mut var_xdcinv: f64 = *var_xdcinv_slot;
        let mut var_xdcinv_dn0: f64 = *var_xdcinv_dn0_slot;
        let mut var_xdcinv_dn1: f64 = *var_xdcinv_dn1_slot;
        let mut var_xdcinv_dn12: f64 = *var_xdcinv_dn12_slot;
        let mut var_xdcinv_dn14: f64 = *var_xdcinv_dn14_slot;
        let mut var_xdcinv_dn15: f64 = *var_xdcinv_dn15_slot;
        let mut var_xdcinv_dn16: f64 = *var_xdcinv_dn16_slot;
        let mut var_xdcinv_dn17: f64 = *var_xdcinv_dn17_slot;
        let mut var_xdcinv_dn18: f64 = *var_xdcinv_dn18_slot;
        let mut var_xdcinv_dn19: f64 = *var_xdcinv_dn19_slot;
        let mut var_xdcinv_dn2: f64 = *var_xdcinv_dn2_slot;
        let mut var_xdcinv_dn20: f64 = *var_xdcinv_dn20_slot;
        let mut var_xdcinv_dn21: f64 = *var_xdcinv_dn21_slot;
        let mut var_xdcinv_dn22: f64 = *var_xdcinv_dn22_slot;
        let mut var_xdcinv_dn3: f64 = *var_xdcinv_dn3_slot;
        let mut var_xdcinv_dn4: f64 = *var_xdcinv_dn4_slot;
        let mut var_xdcinv_dn5: f64 = *var_xdcinv_dn5_slot;
        let mut var_xdcinv_dn6: f64 = *var_xdcinv_dn6_slot;
        let mut var_xdcinv_dn7: f64 = *var_xdcinv_dn7_slot;
        let mut var_xdcinv_dn8: f64 = *var_xdcinv_dn8_slot;
        let mut var_xdcinv_dn9: f64 = *var_xdcinv_dn9_slot;

        let (assign30600_e48404, assign30600_e48404_d_n0, assign30600_e48404_d_n1, assign30600_e48404_d_n2, assign30600_e48404_d_n3, assign30600_e48404_d_n4, assign30600_e48404_d_n5, assign30600_e48404_d_n6, assign30600_e48404_d_n7, assign30600_e48404_d_n8, assign30600_e48404_d_n9, assign30600_e48404_d_n12, assign30600_e48404_d_n14, assign30600_e48404_d_n15, assign30600_e48404_d_n16, assign30600_e48404_d_n17, assign30600_e48404_d_n18, assign30600_e48404_d_n19, assign30600_e48404_d_n20, assign30600_e48404_d_n21, assign30600_e48404_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let assign30600_e48394: f64 = { let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign30600_e48396: f64 = (assign30600_e48394 * 3.24e17);
        let assign30600_e48400: f64 = (0.6666666666666666 * var_vgefm213g0);
        let assign30600_e48401: f64 = (1.0 + assign30600_e48400);
        let assign30600_e48402: f64 = (assign30600_e48396 * assign30600_e48401);
        (assign30600_e48402, (((({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn0) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * var_vgefm213g0_dn0))), (((({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn1) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * var_vgefm213g0_dn1))), (((({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn2) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * var_vgefm213g0_dn2))), (((({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn3) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * var_vgefm213g0_dn3))), (((({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn4) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * var_vgefm213g0_dn4))), (((({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn5) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * var_vgefm213g0_dn5))), (((({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn6) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * var_vgefm213g0_dn6))), (((({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn7) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * var_vgefm213g0_dn7))), (((({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn8) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * var_vgefm213g0_dn8))), (((({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn9) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * var_vgefm213g0_dn9))), (((({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn12) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * var_vgefm213g0_dn12))), (((({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn14) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * var_vgefm213g0_dn14))), (((({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn15) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * var_vgefm213g0_dn15))), (((({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn16) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * var_vgefm213g0_dn16))), (((({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn17) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * var_vgefm213g0_dn17))), (((({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn18) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * var_vgefm213g0_dn18))), (((({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn19) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * var_vgefm213g0_dn19))), (((({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn20) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * var_vgefm213g0_dn20))), (((({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn21) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * var_vgefm213g0_dn21))), (((({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn22) * 3.24e17) * assign30600_e48401) + (assign30600_e48396 * (0.6666666666666666 * var_vgefm213g0_dn22))),)
    } else {
        (var_t5ng02, var_t5ng02_dn0, var_t5ng02_dn1, var_t5ng02_dn2, var_t5ng02_dn3, var_t5ng02_dn4, var_t5ng02_dn5, var_t5ng02_dn6, var_t5ng02_dn7, var_t5ng02_dn8, var_t5ng02_dn9, var_t5ng02_dn12, var_t5ng02_dn14, var_t5ng02_dn15, var_t5ng02_dn16, var_t5ng02_dn17, var_t5ng02_dn18, var_t5ng02_dn19, var_t5ng02_dn20, var_t5ng02_dn21, var_t5ng02_dn22,)
    }
};
        var_t5ng02 = assign30600_e48404;
        var_t5ng02_dn0 = assign30600_e48404_d_n0;
        var_t5ng02_dn1 = assign30600_e48404_d_n1;
        var_t5ng02_dn2 = assign30600_e48404_d_n2;
        var_t5ng02_dn3 = assign30600_e48404_d_n3;
        var_t5ng02_dn4 = assign30600_e48404_d_n4;
        var_t5ng02_dn5 = assign30600_e48404_d_n5;
        var_t5ng02_dn6 = assign30600_e48404_d_n6;
        var_t5ng02_dn7 = assign30600_e48404_d_n7;
        var_t5ng02_dn8 = assign30600_e48404_d_n8;
        var_t5ng02_dn9 = assign30600_e48404_d_n9;
        var_t5ng02_dn12 = assign30600_e48404_d_n12;
        var_t5ng02_dn14 = assign30600_e48404_d_n14;
        var_t5ng02_dn15 = assign30600_e48404_d_n15;
        var_t5ng02_dn16 = assign30600_e48404_d_n16;
        var_t5ng02_dn17 = assign30600_e48404_d_n17;
        var_t5ng02_dn18 = assign30600_e48404_d_n18;
        var_t5ng02_dn19 = assign30600_e48404_d_n19;
        var_t5ng02_dn20 = assign30600_e48404_d_n20;
        var_t5ng02_dn21 = assign30600_e48404_d_n21;
        var_t5ng02_dn22 = assign30600_e48404_d_n22;

        let (assign30610_e48416, assign30610_e48416_d_n0, assign30610_e48416_d_n1, assign30610_e48416_d_n2, assign30610_e48416_d_n3, assign30610_e48416_d_n4, assign30610_e48416_d_n5, assign30610_e48416_d_n6, assign30610_e48416_d_n7, assign30610_e48416_d_n8, assign30610_e48416_d_n9, assign30610_e48416_d_n12, assign30610_e48416_d_n14, assign30610_e48416_d_n15, assign30610_e48416_d_n16, assign30610_e48416_d_n17, assign30610_e48416_d_n18, assign30610_e48416_d_n19, assign30610_e48416_d_n20, assign30610_e48416_d_n21, assign30610_e48416_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let assign30610_e48413: f64 = { let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign30610_e48414: f64 = (1.0 + assign30610_e48413);
        (assign30610_e48414, ({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn0), ({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn1), ({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn2), ({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn3), ({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn4), ({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn5), ({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn6), ({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn7), ({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn8), ({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn9), ({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn12), ({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn14), ({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn15), ({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn16), ({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn17), ({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn18), ({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn19), ({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn20), ({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn21), ({ let limited_exp_arg = var_tg02; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg02_dn22),)
    } else {
        (var_t5dg02, var_t5dg02_dn0, var_t5dg02_dn1, var_t5dg02_dn2, var_t5dg02_dn3, var_t5dg02_dn4, var_t5dg02_dn5, var_t5dg02_dn6, var_t5dg02_dn7, var_t5dg02_dn8, var_t5dg02_dn9, var_t5dg02_dn12, var_t5dg02_dn14, var_t5dg02_dn15, var_t5dg02_dn16, var_t5dg02_dn17, var_t5dg02_dn18, var_t5dg02_dn19, var_t5dg02_dn20, var_t5dg02_dn21, var_t5dg02_dn22,)
    }
};
        var_t5dg02 = assign30610_e48416;
        var_t5dg02_dn0 = assign30610_e48416_d_n0;
        var_t5dg02_dn1 = assign30610_e48416_d_n1;
        var_t5dg02_dn2 = assign30610_e48416_d_n2;
        var_t5dg02_dn3 = assign30610_e48416_d_n3;
        var_t5dg02_dn4 = assign30610_e48416_d_n4;
        var_t5dg02_dn5 = assign30610_e48416_d_n5;
        var_t5dg02_dn6 = assign30610_e48416_d_n6;
        var_t5dg02_dn7 = assign30610_e48416_d_n7;
        var_t5dg02_dn8 = assign30610_e48416_d_n8;
        var_t5dg02_dn9 = assign30610_e48416_d_n9;
        var_t5dg02_dn12 = assign30610_e48416_d_n12;
        var_t5dg02_dn14 = assign30610_e48416_d_n14;
        var_t5dg02_dn15 = assign30610_e48416_d_n15;
        var_t5dg02_dn16 = assign30610_e48416_d_n16;
        var_t5dg02_dn17 = assign30610_e48416_d_n17;
        var_t5dg02_dn18 = assign30610_e48416_d_n18;
        var_t5dg02_dn19 = assign30610_e48416_d_n19;
        var_t5dg02_dn20 = assign30610_e48416_d_n20;
        var_t5dg02_dn21 = assign30610_e48416_d_n21;
        var_t5dg02_dn22 = assign30610_e48416_d_n22;

        let (assign30620_e48434, assign30620_e48434_d_n0, assign30620_e48434_d_n1, assign30620_e48434_d_n2, assign30620_e48434_d_n3, assign30620_e48434_d_n4, assign30620_e48434_d_n5, assign30620_e48434_d_n6, assign30620_e48434_d_n7, assign30620_e48434_d_n8, assign30620_e48434_d_n9, assign30620_e48434_d_n12, assign30620_e48434_d_n14, assign30620_e48434_d_n15, assign30620_e48434_d_n16, assign30620_e48434_d_n17, assign30620_e48434_d_n18, assign30620_e48434_d_n19, assign30620_e48434_d_n20, assign30620_e48434_d_n21, assign30620_e48434_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let assign30620_e48424: f64 = { let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign30620_e48426: f64 = (assign30620_e48424 * 3.24e17);
        let assign30620_e48430: f64 = (0.6666666666666666 * var_vgefm213g1);
        let assign30620_e48431: f64 = (1.0 + assign30620_e48430);
        let assign30620_e48432: f64 = (assign30620_e48426 * assign30620_e48431);
        (assign30620_e48432, (((({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn0) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * var_vgefm213g1_dn0))), (((({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn1) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * var_vgefm213g1_dn1))), (((({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn2) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * var_vgefm213g1_dn2))), (((({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn3) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * var_vgefm213g1_dn3))), (((({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn4) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * var_vgefm213g1_dn4))), (((({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn5) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * var_vgefm213g1_dn5))), (((({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn6) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * var_vgefm213g1_dn6))), (((({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn7) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * var_vgefm213g1_dn7))), (((({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn8) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * var_vgefm213g1_dn8))), (((({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn9) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * var_vgefm213g1_dn9))), (((({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn12) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * var_vgefm213g1_dn12))), (((({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn14) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * var_vgefm213g1_dn14))), (((({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn15) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * var_vgefm213g1_dn15))), (((({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn16) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * var_vgefm213g1_dn16))), (((({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn17) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * var_vgefm213g1_dn17))), (((({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn18) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * var_vgefm213g1_dn18))), (((({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn19) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * var_vgefm213g1_dn19))), (((({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn20) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * var_vgefm213g1_dn20))), (((({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn21) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * var_vgefm213g1_dn21))), (((({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn22) * 3.24e17) * assign30620_e48431) + (assign30620_e48426 * (0.6666666666666666 * var_vgefm213g1_dn22))),)
    } else {
        (var_t5ng12, var_t5ng12_dn0, var_t5ng12_dn1, var_t5ng12_dn2, var_t5ng12_dn3, var_t5ng12_dn4, var_t5ng12_dn5, var_t5ng12_dn6, var_t5ng12_dn7, var_t5ng12_dn8, var_t5ng12_dn9, var_t5ng12_dn12, var_t5ng12_dn14, var_t5ng12_dn15, var_t5ng12_dn16, var_t5ng12_dn17, var_t5ng12_dn18, var_t5ng12_dn19, var_t5ng12_dn20, var_t5ng12_dn21, var_t5ng12_dn22,)
    }
};
        var_t5ng12 = assign30620_e48434;
        var_t5ng12_dn0 = assign30620_e48434_d_n0;
        var_t5ng12_dn1 = assign30620_e48434_d_n1;
        var_t5ng12_dn2 = assign30620_e48434_d_n2;
        var_t5ng12_dn3 = assign30620_e48434_d_n3;
        var_t5ng12_dn4 = assign30620_e48434_d_n4;
        var_t5ng12_dn5 = assign30620_e48434_d_n5;
        var_t5ng12_dn6 = assign30620_e48434_d_n6;
        var_t5ng12_dn7 = assign30620_e48434_d_n7;
        var_t5ng12_dn8 = assign30620_e48434_d_n8;
        var_t5ng12_dn9 = assign30620_e48434_d_n9;
        var_t5ng12_dn12 = assign30620_e48434_d_n12;
        var_t5ng12_dn14 = assign30620_e48434_d_n14;
        var_t5ng12_dn15 = assign30620_e48434_d_n15;
        var_t5ng12_dn16 = assign30620_e48434_d_n16;
        var_t5ng12_dn17 = assign30620_e48434_d_n17;
        var_t5ng12_dn18 = assign30620_e48434_d_n18;
        var_t5ng12_dn19 = assign30620_e48434_d_n19;
        var_t5ng12_dn20 = assign30620_e48434_d_n20;
        var_t5ng12_dn21 = assign30620_e48434_d_n21;
        var_t5ng12_dn22 = assign30620_e48434_d_n22;

        let (assign30630_e48446, assign30630_e48446_d_n0, assign30630_e48446_d_n1, assign30630_e48446_d_n2, assign30630_e48446_d_n3, assign30630_e48446_d_n4, assign30630_e48446_d_n5, assign30630_e48446_d_n6, assign30630_e48446_d_n7, assign30630_e48446_d_n8, assign30630_e48446_d_n9, assign30630_e48446_d_n12, assign30630_e48446_d_n14, assign30630_e48446_d_n15, assign30630_e48446_d_n16, assign30630_e48446_d_n17, assign30630_e48446_d_n18, assign30630_e48446_d_n19, assign30630_e48446_d_n20, assign30630_e48446_d_n21, assign30630_e48446_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let assign30630_e48443: f64 = { let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign30630_e48444: f64 = (1.0 + assign30630_e48443);
        (assign30630_e48444, ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn0), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn1), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn2), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn3), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn4), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn5), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn6), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn7), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn8), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn9), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn12), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn14), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn15), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn16), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn17), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn18), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn19), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn20), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn21), ({ let limited_exp_arg = var_tg12; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * var_tg12_dn22),)
    } else {
        (var_t5dg12, var_t5dg12_dn0, var_t5dg12_dn1, var_t5dg12_dn2, var_t5dg12_dn3, var_t5dg12_dn4, var_t5dg12_dn5, var_t5dg12_dn6, var_t5dg12_dn7, var_t5dg12_dn8, var_t5dg12_dn9, var_t5dg12_dn12, var_t5dg12_dn14, var_t5dg12_dn15, var_t5dg12_dn16, var_t5dg12_dn17, var_t5dg12_dn18, var_t5dg12_dn19, var_t5dg12_dn20, var_t5dg12_dn21, var_t5dg12_dn22,)
    }
};
        var_t5dg12 = assign30630_e48446;
        var_t5dg12_dn0 = assign30630_e48446_d_n0;
        var_t5dg12_dn1 = assign30630_e48446_d_n1;
        var_t5dg12_dn2 = assign30630_e48446_d_n2;
        var_t5dg12_dn3 = assign30630_e48446_d_n3;
        var_t5dg12_dn4 = assign30630_e48446_d_n4;
        var_t5dg12_dn5 = assign30630_e48446_d_n5;
        var_t5dg12_dn6 = assign30630_e48446_d_n6;
        var_t5dg12_dn7 = assign30630_e48446_d_n7;
        var_t5dg12_dn8 = assign30630_e48446_d_n8;
        var_t5dg12_dn9 = assign30630_e48446_d_n9;
        var_t5dg12_dn12 = assign30630_e48446_d_n12;
        var_t5dg12_dn14 = assign30630_e48446_d_n14;
        var_t5dg12_dn15 = assign30630_e48446_d_n15;
        var_t5dg12_dn16 = assign30630_e48446_d_n16;
        var_t5dg12_dn17 = assign30630_e48446_d_n17;
        var_t5dg12_dn18 = assign30630_e48446_d_n18;
        var_t5dg12_dn19 = assign30630_e48446_d_n19;
        var_t5dg12_dn20 = assign30630_e48446_d_n20;
        var_t5dg12_dn21 = assign30630_e48446_d_n21;
        var_t5dg12_dn22 = assign30630_e48446_d_n22;

        let (assign30640_e48466, assign30640_e48466_d_n0, assign30640_e48466_d_n1, assign30640_e48466_d_n2, assign30640_e48466_d_n3, assign30640_e48466_d_n4, assign30640_e48466_d_n5, assign30640_e48466_d_n6, assign30640_e48466_d_n7, assign30640_e48466_d_n8, assign30640_e48466_d_n9, assign30640_e48466_d_n12, assign30640_e48466_d_n14, assign30640_e48466_d_n15, assign30640_e48466_d_n16, assign30640_e48466_d_n17, assign30640_e48466_d_n18, assign30640_e48466_d_n19, assign30640_e48466_d_n20, assign30640_e48466_d_n21, assign30640_e48466_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let assign30640_e48454: f64 = (-1.0);
        let assign30640_e48456: f64 = (assign30640_e48454 * var_cch);
        let assign30640_e48459: f64 = (var_t5ng02 / var_t5dg02);
        let assign30640_e48460: f64 = (assign30640_e48456 - assign30640_e48459);
        let assign30640_e48463: f64 = (var_t5ng12 / var_t5dg12);
        let assign30640_e48464: f64 = (assign30640_e48460 - assign30640_e48463);
        (assign30640_e48464, ((-(((var_t5ng02_dn0 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn0)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn0 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn0)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn1 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn1)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn1 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn1)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn2 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn2)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn2 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn2)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn3 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn3)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn3 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn3)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn4 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn4)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn4 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn4)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn5 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn5)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn5 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn5)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn6 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn6)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn6 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn6)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn7 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn7)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn7 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn7)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn8 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn8)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn8 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn8)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn9 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn9)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn9 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn9)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn12 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn12)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn12 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn12)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn14 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn14)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn14 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn14)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn15 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn15)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn15 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn15)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn16 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn16)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn16 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn16)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn17 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn17)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn17 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn17)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn18 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn18)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn18 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn18)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn19 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn19)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn19 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn19)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn20 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn20)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn20 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn20)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn21 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn21)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn21 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn21)) / (var_t5dg12 * var_t5dg12))), ((-(((var_t5ng02_dn22 * var_t5dg02) - (var_t5ng02 * var_t5dg02_dn22)) / (var_t5dg02 * var_t5dg02))) - (((var_t5ng12_dn22 * var_t5dg12) - (var_t5ng12 * var_t5dg12_dn22)) / (var_t5dg12 * var_t5dg12))),)
    } else {
        (var_t52, var_t52_dn0, var_t52_dn1, var_t52_dn2, var_t52_dn3, var_t52_dn4, var_t52_dn5, var_t52_dn6, var_t52_dn7, var_t52_dn8, var_t52_dn9, var_t52_dn12, var_t52_dn14, var_t52_dn15, var_t52_dn16, var_t52_dn17, var_t52_dn18, var_t52_dn19, var_t52_dn20, var_t52_dn21, var_t52_dn22,)
    }
};
        var_t52 = assign30640_e48466;
        var_t52_dn0 = assign30640_e48466_d_n0;
        var_t52_dn1 = assign30640_e48466_d_n1;
        var_t52_dn2 = assign30640_e48466_d_n2;
        var_t52_dn3 = assign30640_e48466_d_n3;
        var_t52_dn4 = assign30640_e48466_d_n4;
        var_t52_dn5 = assign30640_e48466_d_n5;
        var_t52_dn6 = assign30640_e48466_d_n6;
        var_t52_dn7 = assign30640_e48466_d_n7;
        var_t52_dn8 = assign30640_e48466_d_n8;
        var_t52_dn9 = assign30640_e48466_d_n9;
        var_t52_dn12 = assign30640_e48466_d_n12;
        var_t52_dn14 = assign30640_e48466_d_n14;
        var_t52_dn15 = assign30640_e48466_d_n15;
        var_t52_dn16 = assign30640_e48466_d_n16;
        var_t52_dn17 = assign30640_e48466_d_n17;
        var_t52_dn18 = assign30640_e48466_d_n18;
        var_t52_dn19 = assign30640_e48466_d_n19;
        var_t52_dn20 = assign30640_e48466_d_n20;
        var_t52_dn21 = assign30640_e48466_d_n21;
        var_t52_dn22 = assign30640_e48466_d_n22;

        let (assign30650_e48479, assign30650_e48479_d_n0, assign30650_e48479_d_n1, assign30650_e48479_d_n2, assign30650_e48479_d_n3, assign30650_e48479_d_n4, assign30650_e48479_d_n5, assign30650_e48479_d_n6, assign30650_e48479_d_n7, assign30650_e48479_d_n8, assign30650_e48479_d_n9, assign30650_e48479_d_n12, assign30650_e48479_d_n14, assign30650_e48479_d_n15, assign30650_e48479_d_n16, assign30650_e48479_d_n17, assign30650_e48479_d_n18, assign30650_e48479_d_n19, assign30650_e48479_d_n20, assign30650_e48479_d_n21, assign30650_e48479_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let assign30650_e48476: f64 = (var_t42 / var_t52);
        let assign30650_e48477: f64 = (var_ef2 - assign30650_e48476);
        (assign30650_e48477, (var_ef2_dn0 - (((var_t42_dn0 * var_t52) - (var_t42 * var_t52_dn0)) / (var_t52 * var_t52))), (var_ef2_dn1 - (((var_t42_dn1 * var_t52) - (var_t42 * var_t52_dn1)) / (var_t52 * var_t52))), (var_ef2_dn2 - (((var_t42_dn2 * var_t52) - (var_t42 * var_t52_dn2)) / (var_t52 * var_t52))), (var_ef2_dn3 - (((var_t42_dn3 * var_t52) - (var_t42 * var_t52_dn3)) / (var_t52 * var_t52))), (var_ef2_dn4 - (((var_t42_dn4 * var_t52) - (var_t42 * var_t52_dn4)) / (var_t52 * var_t52))), (var_ef2_dn5 - (((var_t42_dn5 * var_t52) - (var_t42 * var_t52_dn5)) / (var_t52 * var_t52))), (var_ef2_dn6 - (((var_t42_dn6 * var_t52) - (var_t42 * var_t52_dn6)) / (var_t52 * var_t52))), (var_ef2_dn7 - (((var_t42_dn7 * var_t52) - (var_t42 * var_t52_dn7)) / (var_t52 * var_t52))), (var_ef2_dn8 - (((var_t42_dn8 * var_t52) - (var_t42 * var_t52_dn8)) / (var_t52 * var_t52))), (var_ef2_dn9 - (((var_t42_dn9 * var_t52) - (var_t42 * var_t52_dn9)) / (var_t52 * var_t52))), (var_ef2_dn12 - (((var_t42_dn12 * var_t52) - (var_t42 * var_t52_dn12)) / (var_t52 * var_t52))), (var_ef2_dn14 - (((var_t42_dn14 * var_t52) - (var_t42 * var_t52_dn14)) / (var_t52 * var_t52))), (var_ef2_dn15 - (((var_t42_dn15 * var_t52) - (var_t42 * var_t52_dn15)) / (var_t52 * var_t52))), (var_ef2_dn16 - (((var_t42_dn16 * var_t52) - (var_t42 * var_t52_dn16)) / (var_t52 * var_t52))), (var_ef2_dn17 - (((var_t42_dn17 * var_t52) - (var_t42 * var_t52_dn17)) / (var_t52 * var_t52))), (var_ef2_dn18 - (((var_t42_dn18 * var_t52) - (var_t42 * var_t52_dn18)) / (var_t52 * var_t52))), (var_ef2_dn19 - (((var_t42_dn19 * var_t52) - (var_t42 * var_t52_dn19)) / (var_t52 * var_t52))), (var_ef2_dn20 - (((var_t42_dn20 * var_t52) - (var_t42 * var_t52_dn20)) / (var_t52 * var_t52))), (var_ef2_dn21 - (((var_t42_dn21 * var_t52) - (var_t42 * var_t52_dn21)) / (var_t52 * var_t52))), (var_ef2_dn22 - (((var_t42_dn22 * var_t52) - (var_t42 * var_t52_dn22)) / (var_t52 * var_t52))),)
    } else {
        (var_ef3, var_ef3_dn0, var_ef3_dn1, var_ef3_dn2, var_ef3_dn3, var_ef3_dn4, var_ef3_dn5, var_ef3_dn6, var_ef3_dn7, var_ef3_dn8, var_ef3_dn9, var_ef3_dn12, var_ef3_dn14, var_ef3_dn15, var_ef3_dn16, var_ef3_dn17, var_ef3_dn18, var_ef3_dn19, var_ef3_dn20, var_ef3_dn21, var_ef3_dn22,)
    }
};
        var_ef3 = assign30650_e48479;
        var_ef3_dn0 = assign30650_e48479_d_n0;
        var_ef3_dn1 = assign30650_e48479_d_n1;
        var_ef3_dn2 = assign30650_e48479_d_n2;
        var_ef3_dn3 = assign30650_e48479_d_n3;
        var_ef3_dn4 = assign30650_e48479_d_n4;
        var_ef3_dn5 = assign30650_e48479_d_n5;
        var_ef3_dn6 = assign30650_e48479_d_n6;
        var_ef3_dn7 = assign30650_e48479_d_n7;
        var_ef3_dn8 = assign30650_e48479_d_n8;
        var_ef3_dn9 = assign30650_e48479_d_n9;
        var_ef3_dn12 = assign30650_e48479_d_n12;
        var_ef3_dn14 = assign30650_e48479_d_n14;
        var_ef3_dn15 = assign30650_e48479_d_n15;
        var_ef3_dn16 = assign30650_e48479_d_n16;
        var_ef3_dn17 = assign30650_e48479_d_n17;
        var_ef3_dn18 = assign30650_e48479_d_n18;
        var_ef3_dn19 = assign30650_e48479_d_n19;
        var_ef3_dn20 = assign30650_e48479_d_n20;
        var_ef3_dn21 = assign30650_e48479_d_n21;
        var_ef3_dn22 = assign30650_e48479_d_n22;

        let (assign30660_e48490, assign30660_e48490_d_n0, assign30660_e48490_d_n1, assign30660_e48490_d_n2, assign30660_e48490_d_n3, assign30660_e48490_d_n4, assign30660_e48490_d_n5, assign30660_e48490_d_n6, assign30660_e48490_d_n7, assign30660_e48490_d_n8, assign30660_e48490_d_n9, assign30660_e48490_d_n12, assign30660_e48490_d_n14, assign30660_e48490_d_n15, assign30660_e48490_d_n16, assign30660_e48490_d_n17, assign30660_e48490_d_n18, assign30660_e48490_d_n19, assign30660_e48490_d_n20, assign30660_e48490_d_n21, assign30660_e48490_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 != 0.0)) {
        let assign30660_e48488: f64 = (var_ef3 + var_vdeff);
        (assign30660_e48488, (var_ef3_dn0 + var_vdeff_dn0), (var_ef3_dn1 + var_vdeff_dn1), (var_ef3_dn2 + var_vdeff_dn2), (var_ef3_dn3 + var_vdeff_dn3), (var_ef3_dn4 + var_vdeff_dn4), (var_ef3_dn5 + var_vdeff_dn5), (var_ef3_dn6 + var_vdeff_dn6), (var_ef3_dn7 + var_vdeff_dn7), (var_ef3_dn8 + var_vdeff_dn8), (var_ef3_dn9 + var_vdeff_dn9), (var_ef3_dn12 + var_vdeff_dn12), (var_ef3_dn14 + var_vdeff_dn14), (var_ef3_dn15 + var_vdeff_dn15), (var_ef3_dn16 + var_vdeff_dn16), (var_ef3_dn17 + var_vdeff_dn17), (var_ef3_dn18 + var_vdeff_dn18), (var_ef3_dn19 + var_vdeff_dn19), (var_ef3_dn20 + var_vdeff_dn20), (var_ef3_dn21 + var_vdeff_dn21), (var_ef3_dn22 + var_vdeff_dn22),)
    } else {
        (var_psid_fp4s, var_psid_fp4s_dn0, var_psid_fp4s_dn1, var_psid_fp4s_dn2, var_psid_fp4s_dn3, var_psid_fp4s_dn4, var_psid_fp4s_dn5, var_psid_fp4s_dn6, var_psid_fp4s_dn7, var_psid_fp4s_dn8, var_psid_fp4s_dn9, var_psid_fp4s_dn12, var_psid_fp4s_dn14, var_psid_fp4s_dn15, var_psid_fp4s_dn16, var_psid_fp4s_dn17, var_psid_fp4s_dn18, var_psid_fp4s_dn19, var_psid_fp4s_dn20, var_psid_fp4s_dn21, var_psid_fp4s_dn22,)
    }
};
        var_psid_fp4s = assign30660_e48490;
        var_psid_fp4s_dn0 = assign30660_e48490_d_n0;
        var_psid_fp4s_dn1 = assign30660_e48490_d_n1;
        var_psid_fp4s_dn2 = assign30660_e48490_d_n2;
        var_psid_fp4s_dn3 = assign30660_e48490_d_n3;
        var_psid_fp4s_dn4 = assign30660_e48490_d_n4;
        var_psid_fp4s_dn5 = assign30660_e48490_d_n5;
        var_psid_fp4s_dn6 = assign30660_e48490_d_n6;
        var_psid_fp4s_dn7 = assign30660_e48490_d_n7;
        var_psid_fp4s_dn8 = assign30660_e48490_d_n8;
        var_psid_fp4s_dn9 = assign30660_e48490_d_n9;
        var_psid_fp4s_dn12 = assign30660_e48490_d_n12;
        var_psid_fp4s_dn14 = assign30660_e48490_d_n14;
        var_psid_fp4s_dn15 = assign30660_e48490_d_n15;
        var_psid_fp4s_dn16 = assign30660_e48490_d_n16;
        var_psid_fp4s_dn17 = assign30660_e48490_d_n17;
        var_psid_fp4s_dn18 = assign30660_e48490_d_n18;
        var_psid_fp4s_dn19 = assign30660_e48490_d_n19;
        var_psid_fp4s_dn20 = assign30660_e48490_d_n20;
        var_psid_fp4s_dn21 = assign30660_e48490_d_n21;
        var_psid_fp4s_dn22 = assign30660_e48490_d_n22;

        let (assign30670_e48502, assign30670_e48502_d_n0, assign30670_e48502_d_n1, assign30670_e48502_d_n2, assign30670_e48502_d_n3, assign30670_e48502_d_n4, assign30670_e48502_d_n5, assign30670_e48502_d_n6, assign30670_e48502_d_n7, assign30670_e48502_d_n8, assign30670_e48502_d_n9, assign30670_e48502_d_n12, assign30670_e48502_d_n14, assign30670_e48502_d_n15, assign30670_e48502_d_n16, assign30670_e48502_d_n17, assign30670_e48502_d_n18, assign30670_e48502_d_n19, assign30670_e48502_d_n20, assign30670_e48502_d_n21, assign30670_e48502_d_n22,) = {
    if (((var_guard504 == 0.0) && (var_guard513 != 0.0)) && (var_guard518 == 0.0)) {
        let assign30670_e48500: f64 = (var_ef1 + var_vdeff);
        (assign30670_e48500, (var_ef1_dn0 + var_vdeff_dn0), (var_ef1_dn1 + var_vdeff_dn1), (var_ef1_dn2 + var_vdeff_dn2), (var_ef1_dn3 + var_vdeff_dn3), (var_ef1_dn4 + var_vdeff_dn4), (var_ef1_dn5 + var_vdeff_dn5), (var_ef1_dn6 + var_vdeff_dn6), (var_ef1_dn7 + var_vdeff_dn7), (var_ef1_dn8 + var_vdeff_dn8), (var_ef1_dn9 + var_vdeff_dn9), (var_ef1_dn12 + var_vdeff_dn12), (var_ef1_dn14 + var_vdeff_dn14), (var_ef1_dn15 + var_vdeff_dn15), (var_ef1_dn16 + var_vdeff_dn16), (var_ef1_dn17 + var_vdeff_dn17), (var_ef1_dn18 + var_vdeff_dn18), (var_ef1_dn19 + var_vdeff_dn19), (var_ef1_dn20 + var_vdeff_dn20), (var_ef1_dn21 + var_vdeff_dn21), (var_ef1_dn22 + var_vdeff_dn22),)
    } else {
        (var_psid_fp4s, var_psid_fp4s_dn0, var_psid_fp4s_dn1, var_psid_fp4s_dn2, var_psid_fp4s_dn3, var_psid_fp4s_dn4, var_psid_fp4s_dn5, var_psid_fp4s_dn6, var_psid_fp4s_dn7, var_psid_fp4s_dn8, var_psid_fp4s_dn9, var_psid_fp4s_dn12, var_psid_fp4s_dn14, var_psid_fp4s_dn15, var_psid_fp4s_dn16, var_psid_fp4s_dn17, var_psid_fp4s_dn18, var_psid_fp4s_dn19, var_psid_fp4s_dn20, var_psid_fp4s_dn21, var_psid_fp4s_dn22,)
    }
};
        var_psid_fp4s = assign30670_e48502;
        var_psid_fp4s_dn0 = assign30670_e48502_d_n0;
        var_psid_fp4s_dn1 = assign30670_e48502_d_n1;
        var_psid_fp4s_dn2 = assign30670_e48502_d_n2;
        var_psid_fp4s_dn3 = assign30670_e48502_d_n3;
        var_psid_fp4s_dn4 = assign30670_e48502_d_n4;
        var_psid_fp4s_dn5 = assign30670_e48502_d_n5;
        var_psid_fp4s_dn6 = assign30670_e48502_d_n6;
        var_psid_fp4s_dn7 = assign30670_e48502_d_n7;
        var_psid_fp4s_dn8 = assign30670_e48502_d_n8;
        var_psid_fp4s_dn9 = assign30670_e48502_d_n9;
        var_psid_fp4s_dn12 = assign30670_e48502_d_n12;
        var_psid_fp4s_dn14 = assign30670_e48502_d_n14;
        var_psid_fp4s_dn15 = assign30670_e48502_d_n15;
        var_psid_fp4s_dn16 = assign30670_e48502_d_n16;
        var_psid_fp4s_dn17 = assign30670_e48502_d_n17;
        var_psid_fp4s_dn18 = assign30670_e48502_d_n18;
        var_psid_fp4s_dn19 = assign30670_e48502_d_n19;
        var_psid_fp4s_dn20 = assign30670_e48502_d_n20;
        var_psid_fp4s_dn21 = assign30670_e48502_d_n21;
        var_psid_fp4s_dn22 = assign30670_e48502_d_n22;

        let (assign30680_e48513, assign30680_e48513_d_n0, assign30680_e48513_d_n1, assign30680_e48513_d_n2, assign30680_e48513_d_n3, assign30680_e48513_d_n4, assign30680_e48513_d_n5, assign30680_e48513_d_n6, assign30680_e48513_d_n7, assign30680_e48513_d_n8, assign30680_e48513_d_n9, assign30680_e48513_d_n12, assign30680_e48513_d_n14, assign30680_e48513_d_n15, assign30680_e48513_d_n16, assign30680_e48513_d_n17, assign30680_e48513_d_n18, assign30680_e48513_d_n19, assign30680_e48513_d_n20, assign30680_e48513_d_n21, assign30680_e48513_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30680_e48510: f64 = (var_psis_fp4s + var_psid_fp4s);
        let assign30680_e48511: f64 = (0.5 * assign30680_e48510);
        (assign30680_e48511, (0.5 * (var_psis_fp4s_dn0 + var_psid_fp4s_dn0)), (0.5 * (var_psis_fp4s_dn1 + var_psid_fp4s_dn1)), (0.5 * (var_psis_fp4s_dn2 + var_psid_fp4s_dn2)), (0.5 * (var_psis_fp4s_dn3 + var_psid_fp4s_dn3)), (0.5 * (var_psis_fp4s_dn4 + var_psid_fp4s_dn4)), (0.5 * (var_psis_fp4s_dn5 + var_psid_fp4s_dn5)), (0.5 * (var_psis_fp4s_dn6 + var_psid_fp4s_dn6)), (0.5 * (var_psis_fp4s_dn7 + var_psid_fp4s_dn7)), (0.5 * (var_psis_fp4s_dn8 + var_psid_fp4s_dn8)), (0.5 * (var_psis_fp4s_dn9 + var_psid_fp4s_dn9)), (0.5 * (var_psis_fp4s_dn12 + var_psid_fp4s_dn12)), (0.5 * (var_psis_fp4s_dn14 + var_psid_fp4s_dn14)), (0.5 * (var_psis_fp4s_dn15 + var_psid_fp4s_dn15)), (0.5 * (var_psis_fp4s_dn16 + var_psid_fp4s_dn16)), (0.5 * (var_psis_fp4s_dn17 + var_psid_fp4s_dn17)), (0.5 * (var_psis_fp4s_dn18 + var_psid_fp4s_dn18)), (0.5 * (var_psis_fp4s_dn19 + var_psid_fp4s_dn19)), (0.5 * (var_psis_fp4s_dn20 + var_psid_fp4s_dn20)), (0.5 * (var_psis_fp4s_dn21 + var_psid_fp4s_dn21)), (0.5 * (var_psis_fp4s_dn22 + var_psid_fp4s_dn22)),)
    } else {
        (var_psim_fp4s, var_psim_fp4s_dn0, var_psim_fp4s_dn1, var_psim_fp4s_dn2, var_psim_fp4s_dn3, var_psim_fp4s_dn4, var_psim_fp4s_dn5, var_psim_fp4s_dn6, var_psim_fp4s_dn7, var_psim_fp4s_dn8, var_psim_fp4s_dn9, var_psim_fp4s_dn12, var_psim_fp4s_dn14, var_psim_fp4s_dn15, var_psim_fp4s_dn16, var_psim_fp4s_dn17, var_psim_fp4s_dn18, var_psim_fp4s_dn19, var_psim_fp4s_dn20, var_psim_fp4s_dn21, var_psim_fp4s_dn22,)
    }
};
        var_psim_fp4s = assign30680_e48513;
        var_psim_fp4s_dn0 = assign30680_e48513_d_n0;
        var_psim_fp4s_dn1 = assign30680_e48513_d_n1;
        var_psim_fp4s_dn2 = assign30680_e48513_d_n2;
        var_psim_fp4s_dn3 = assign30680_e48513_d_n3;
        var_psim_fp4s_dn4 = assign30680_e48513_d_n4;
        var_psim_fp4s_dn5 = assign30680_e48513_d_n5;
        var_psim_fp4s_dn6 = assign30680_e48513_d_n6;
        var_psim_fp4s_dn7 = assign30680_e48513_d_n7;
        var_psim_fp4s_dn8 = assign30680_e48513_d_n8;
        var_psim_fp4s_dn9 = assign30680_e48513_d_n9;
        var_psim_fp4s_dn12 = assign30680_e48513_d_n12;
        var_psim_fp4s_dn14 = assign30680_e48513_d_n14;
        var_psim_fp4s_dn15 = assign30680_e48513_d_n15;
        var_psim_fp4s_dn16 = assign30680_e48513_d_n16;
        var_psim_fp4s_dn17 = assign30680_e48513_d_n17;
        var_psim_fp4s_dn18 = assign30680_e48513_d_n18;
        var_psim_fp4s_dn19 = assign30680_e48513_d_n19;
        var_psim_fp4s_dn20 = assign30680_e48513_d_n20;
        var_psim_fp4s_dn21 = assign30680_e48513_d_n21;
        var_psim_fp4s_dn22 = assign30680_e48513_d_n22;

        let (assign30690_e48522, assign30690_e48522_d_n0, assign30690_e48522_d_n1, assign30690_e48522_d_n2, assign30690_e48522_d_n3, assign30690_e48522_d_n4, assign30690_e48522_d_n5, assign30690_e48522_d_n6, assign30690_e48522_d_n7, assign30690_e48522_d_n8, assign30690_e48522_d_n9, assign30690_e48522_d_n12, assign30690_e48522_d_n14, assign30690_e48522_d_n15, assign30690_e48522_d_n16, assign30690_e48522_d_n17, assign30690_e48522_d_n18, assign30690_e48522_d_n19, assign30690_e48522_d_n20, assign30690_e48522_d_n21, assign30690_e48522_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30690_e48520: f64 = (var_psid_fp4s - var_psis_fp4s);
        (assign30690_e48520, (var_psid_fp4s_dn0 - var_psis_fp4s_dn0), (var_psid_fp4s_dn1 - var_psis_fp4s_dn1), (var_psid_fp4s_dn2 - var_psis_fp4s_dn2), (var_psid_fp4s_dn3 - var_psis_fp4s_dn3), (var_psid_fp4s_dn4 - var_psis_fp4s_dn4), (var_psid_fp4s_dn5 - var_psis_fp4s_dn5), (var_psid_fp4s_dn6 - var_psis_fp4s_dn6), (var_psid_fp4s_dn7 - var_psis_fp4s_dn7), (var_psid_fp4s_dn8 - var_psis_fp4s_dn8), (var_psid_fp4s_dn9 - var_psis_fp4s_dn9), (var_psid_fp4s_dn12 - var_psis_fp4s_dn12), (var_psid_fp4s_dn14 - var_psis_fp4s_dn14), (var_psid_fp4s_dn15 - var_psis_fp4s_dn15), (var_psid_fp4s_dn16 - var_psis_fp4s_dn16), (var_psid_fp4s_dn17 - var_psis_fp4s_dn17), (var_psid_fp4s_dn18 - var_psis_fp4s_dn18), (var_psid_fp4s_dn19 - var_psis_fp4s_dn19), (var_psid_fp4s_dn20 - var_psis_fp4s_dn20), (var_psid_fp4s_dn21 - var_psis_fp4s_dn21), (var_psid_fp4s_dn22 - var_psis_fp4s_dn22),)
    } else {
        (var_psisd_fp4s, var_psisd_fp4s_dn0, var_psisd_fp4s_dn1, var_psisd_fp4s_dn2, var_psisd_fp4s_dn3, var_psisd_fp4s_dn4, var_psisd_fp4s_dn5, var_psisd_fp4s_dn6, var_psisd_fp4s_dn7, var_psisd_fp4s_dn8, var_psisd_fp4s_dn9, var_psisd_fp4s_dn12, var_psisd_fp4s_dn14, var_psisd_fp4s_dn15, var_psisd_fp4s_dn16, var_psisd_fp4s_dn17, var_psisd_fp4s_dn18, var_psisd_fp4s_dn19, var_psisd_fp4s_dn20, var_psisd_fp4s_dn21, var_psisd_fp4s_dn22,)
    }
};
        var_psisd_fp4s = assign30690_e48522;
        var_psisd_fp4s_dn0 = assign30690_e48522_d_n0;
        var_psisd_fp4s_dn1 = assign30690_e48522_d_n1;
        var_psisd_fp4s_dn2 = assign30690_e48522_d_n2;
        var_psisd_fp4s_dn3 = assign30690_e48522_d_n3;
        var_psisd_fp4s_dn4 = assign30690_e48522_d_n4;
        var_psisd_fp4s_dn5 = assign30690_e48522_d_n5;
        var_psisd_fp4s_dn6 = assign30690_e48522_d_n6;
        var_psisd_fp4s_dn7 = assign30690_e48522_d_n7;
        var_psisd_fp4s_dn8 = assign30690_e48522_d_n8;
        var_psisd_fp4s_dn9 = assign30690_e48522_d_n9;
        var_psisd_fp4s_dn12 = assign30690_e48522_d_n12;
        var_psisd_fp4s_dn14 = assign30690_e48522_d_n14;
        var_psisd_fp4s_dn15 = assign30690_e48522_d_n15;
        var_psisd_fp4s_dn16 = assign30690_e48522_d_n16;
        var_psisd_fp4s_dn17 = assign30690_e48522_d_n17;
        var_psisd_fp4s_dn18 = assign30690_e48522_d_n18;
        var_psisd_fp4s_dn19 = assign30690_e48522_d_n19;
        var_psisd_fp4s_dn20 = assign30690_e48522_d_n20;
        var_psisd_fp4s_dn21 = assign30690_e48522_d_n21;
        var_psisd_fp4s_dn22 = assign30690_e48522_d_n22;

        let (assign30700_e48531, assign30700_e48531_d_n0, assign30700_e48531_d_n1, assign30700_e48531_d_n2, assign30700_e48531_d_n3, assign30700_e48531_d_n4, assign30700_e48531_d_n5, assign30700_e48531_d_n6, assign30700_e48531_d_n7, assign30700_e48531_d_n8, assign30700_e48531_d_n9, assign30700_e48531_d_n12, assign30700_e48531_d_n14, assign30700_e48531_d_n15, assign30700_e48531_d_n16, assign30700_e48531_d_n17, assign30700_e48531_d_n18, assign30700_e48531_d_n19, assign30700_e48531_d_n20, assign30700_e48531_d_n21, assign30700_e48531_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30700_e48529: f64 = (var_psid_fp4s - var_psis_fp4s);
        (assign30700_e48529, (var_psid_fp4s_dn0 - var_psis_fp4s_dn0), (var_psid_fp4s_dn1 - var_psis_fp4s_dn1), (var_psid_fp4s_dn2 - var_psis_fp4s_dn2), (var_psid_fp4s_dn3 - var_psis_fp4s_dn3), (var_psid_fp4s_dn4 - var_psis_fp4s_dn4), (var_psid_fp4s_dn5 - var_psis_fp4s_dn5), (var_psid_fp4s_dn6 - var_psis_fp4s_dn6), (var_psid_fp4s_dn7 - var_psis_fp4s_dn7), (var_psid_fp4s_dn8 - var_psis_fp4s_dn8), (var_psid_fp4s_dn9 - var_psis_fp4s_dn9), (var_psid_fp4s_dn12 - var_psis_fp4s_dn12), (var_psid_fp4s_dn14 - var_psis_fp4s_dn14), (var_psid_fp4s_dn15 - var_psis_fp4s_dn15), (var_psid_fp4s_dn16 - var_psis_fp4s_dn16), (var_psid_fp4s_dn17 - var_psis_fp4s_dn17), (var_psid_fp4s_dn18 - var_psis_fp4s_dn18), (var_psid_fp4s_dn19 - var_psis_fp4s_dn19), (var_psid_fp4s_dn20 - var_psis_fp4s_dn20), (var_psid_fp4s_dn21 - var_psis_fp4s_dn21), (var_psid_fp4s_dn22 - var_psis_fp4s_dn22),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn1, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn12, var_t1_dn14, var_t1_dn15, var_t1_dn16, var_t1_dn17, var_t1_dn18, var_t1_dn19, var_t1_dn20, var_t1_dn21, var_t1_dn22,)
    }
};
        var_t1 = assign30700_e48531;
        var_t1_dn0 = assign30700_e48531_d_n0;
        var_t1_dn1 = assign30700_e48531_d_n1;
        var_t1_dn2 = assign30700_e48531_d_n2;
        var_t1_dn3 = assign30700_e48531_d_n3;
        var_t1_dn4 = assign30700_e48531_d_n4;
        var_t1_dn5 = assign30700_e48531_d_n5;
        var_t1_dn6 = assign30700_e48531_d_n6;
        var_t1_dn7 = assign30700_e48531_d_n7;
        var_t1_dn8 = assign30700_e48531_d_n8;
        var_t1_dn9 = assign30700_e48531_d_n9;
        var_t1_dn12 = assign30700_e48531_d_n12;
        var_t1_dn14 = assign30700_e48531_d_n14;
        var_t1_dn15 = assign30700_e48531_d_n15;
        var_t1_dn16 = assign30700_e48531_d_n16;
        var_t1_dn17 = assign30700_e48531_d_n17;
        var_t1_dn18 = assign30700_e48531_d_n18;
        var_t1_dn19 = assign30700_e48531_d_n19;
        var_t1_dn20 = assign30700_e48531_d_n20;
        var_t1_dn21 = assign30700_e48531_d_n21;
        var_t1_dn22 = assign30700_e48531_d_n22;

        let (assign30710_e48542, assign30710_e48542_d_n0, assign30710_e48542_d_n1, assign30710_e48542_d_n2, assign30710_e48542_d_n3, assign30710_e48542_d_n4, assign30710_e48542_d_n5, assign30710_e48542_d_n6, assign30710_e48542_d_n7, assign30710_e48542_d_n8, assign30710_e48542_d_n9, assign30710_e48542_d_n12, assign30710_e48542_d_n14, assign30710_e48542_d_n15, assign30710_e48542_d_n16, assign30710_e48542_d_n17, assign30710_e48542_d_n18, assign30710_e48542_d_n19, assign30710_e48542_d_n20, assign30710_e48542_d_n21, assign30710_e48542_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30710_e48538: f64 = (var_vg0_fp4s + var_vtv);
        let assign30710_e48540: f64 = (assign30710_e48538 - var_psim_fp4s);
        (assign30710_e48540, (var_vg0_fp4s_dn0 - var_psim_fp4s_dn0), (var_vg0_fp4s_dn1 - var_psim_fp4s_dn1), (var_vg0_fp4s_dn2 - var_psim_fp4s_dn2), (var_vg0_fp4s_dn3 - var_psim_fp4s_dn3), ((var_vg0_fp4s_dn4 + var_vtv_dn4) - var_psim_fp4s_dn4), (var_vg0_fp4s_dn5 - var_psim_fp4s_dn5), ((var_vg0_fp4s_dn6 + var_vtv_dn6) - var_psim_fp4s_dn6), ((var_vg0_fp4s_dn7 + var_vtv_dn7) - var_psim_fp4s_dn7), ((var_vg0_fp4s_dn8 + var_vtv_dn8) - var_psim_fp4s_dn8), (var_vg0_fp4s_dn9 - var_psim_fp4s_dn9), (var_vg0_fp4s_dn12 - var_psim_fp4s_dn12), (var_vg0_fp4s_dn14 - var_psim_fp4s_dn14), ((var_vg0_fp4s_dn15 + var_vtv_dn15) - var_psim_fp4s_dn15), ((var_vg0_fp4s_dn16 + var_vtv_dn16) - var_psim_fp4s_dn16), ((var_vg0_fp4s_dn17 + var_vtv_dn17) - var_psim_fp4s_dn17), ((var_vg0_fp4s_dn18 + var_vtv_dn18) - var_psim_fp4s_dn18), ((var_vg0_fp4s_dn19 + var_vtv_dn19) - var_psim_fp4s_dn19), ((var_vg0_fp4s_dn20 + var_vtv_dn20) - var_psim_fp4s_dn20), ((var_vg0_fp4s_dn21 + var_vtv_dn21) - var_psim_fp4s_dn21), ((var_vg0_fp4s_dn22 + var_vtv_dn22) - var_psim_fp4s_dn22),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn1, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn12, var_t2_dn14, var_t2_dn15, var_t2_dn16, var_t2_dn17, var_t2_dn18, var_t2_dn19, var_t2_dn20, var_t2_dn21, var_t2_dn22,)
    }
};
        var_t2 = assign30710_e48542;
        var_t2_dn0 = assign30710_e48542_d_n0;
        var_t2_dn1 = assign30710_e48542_d_n1;
        var_t2_dn2 = assign30710_e48542_d_n2;
        var_t2_dn3 = assign30710_e48542_d_n3;
        var_t2_dn4 = assign30710_e48542_d_n4;
        var_t2_dn5 = assign30710_e48542_d_n5;
        var_t2_dn6 = assign30710_e48542_d_n6;
        var_t2_dn7 = assign30710_e48542_d_n7;
        var_t2_dn8 = assign30710_e48542_d_n8;
        var_t2_dn9 = assign30710_e48542_d_n9;
        var_t2_dn12 = assign30710_e48542_d_n12;
        var_t2_dn14 = assign30710_e48542_d_n14;
        var_t2_dn15 = assign30710_e48542_d_n15;
        var_t2_dn16 = assign30710_e48542_d_n16;
        var_t2_dn17 = assign30710_e48542_d_n17;
        var_t2_dn18 = assign30710_e48542_d_n18;
        var_t2_dn19 = assign30710_e48542_d_n19;
        var_t2_dn20 = assign30710_e48542_d_n20;
        var_t2_dn21 = assign30710_e48542_d_n21;
        var_t2_dn22 = assign30710_e48542_d_n22;

        let (assign30720_e48569, assign30720_e48569_d_n0, assign30720_e48569_d_n1, assign30720_e48569_d_n2, assign30720_e48569_d_n3, assign30720_e48569_d_n4, assign30720_e48569_d_n5, assign30720_e48569_d_n6, assign30720_e48569_d_n7, assign30720_e48569_d_n8, assign30720_e48569_d_n9, assign30720_e48569_d_n12, assign30720_e48569_d_n14, assign30720_e48569_d_n15, assign30720_e48569_d_n16, assign30720_e48569_d_n17, assign30720_e48569_d_n18, assign30720_e48569_d_n19, assign30720_e48569_d_n20, assign30720_e48569_d_n21, assign30720_e48569_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30720_e48549: f64 = (var_cg_fp4s * p.p4);
        let assign30720_e48551: f64 = (assign30720_e48549 * p.p5);
        let assign30720_e48553: f64 = (assign30720_e48551 * p.p200);
        let assign30720_e48556: f64 = (var_vg0_fp4s - var_psim_fp4s);
        let assign30720_e48559: f64 = (0.5 * var_t1);
        let assign30720_e48561: f64 = (assign30720_e48559 * var_t1);
        let assign30720_e48564: f64 = (6.0 * var_t2);
        let assign30720_e48565: f64 = (assign30720_e48561 / assign30720_e48564);
        let assign30720_e48566: f64 = (assign30720_e48556 + assign30720_e48565);
        let assign30720_e48567: f64 = (assign30720_e48553 * assign30720_e48566);
        (assign30720_e48567, (assign30720_e48553 * ((var_vg0_fp4s_dn0 - var_psim_fp4s_dn0) + ((((((0.5 * var_t1_dn0) * var_t1) + (assign30720_e48559 * var_t1_dn0)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * var_t2_dn0))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((var_vg0_fp4s_dn1 - var_psim_fp4s_dn1) + ((((((0.5 * var_t1_dn1) * var_t1) + (assign30720_e48559 * var_t1_dn1)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * var_t2_dn1))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((var_vg0_fp4s_dn2 - var_psim_fp4s_dn2) + ((((((0.5 * var_t1_dn2) * var_t1) + (assign30720_e48559 * var_t1_dn2)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * var_t2_dn2))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((var_vg0_fp4s_dn3 - var_psim_fp4s_dn3) + ((((((0.5 * var_t1_dn3) * var_t1) + (assign30720_e48559 * var_t1_dn3)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * var_t2_dn3))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((var_vg0_fp4s_dn4 - var_psim_fp4s_dn4) + ((((((0.5 * var_t1_dn4) * var_t1) + (assign30720_e48559 * var_t1_dn4)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * var_t2_dn4))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((var_vg0_fp4s_dn5 - var_psim_fp4s_dn5) + ((((((0.5 * var_t1_dn5) * var_t1) + (assign30720_e48559 * var_t1_dn5)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * var_t2_dn5))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((var_vg0_fp4s_dn6 - var_psim_fp4s_dn6) + ((((((0.5 * var_t1_dn6) * var_t1) + (assign30720_e48559 * var_t1_dn6)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * var_t2_dn6))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((var_vg0_fp4s_dn7 - var_psim_fp4s_dn7) + ((((((0.5 * var_t1_dn7) * var_t1) + (assign30720_e48559 * var_t1_dn7)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * var_t2_dn7))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((var_vg0_fp4s_dn8 - var_psim_fp4s_dn8) + ((((((0.5 * var_t1_dn8) * var_t1) + (assign30720_e48559 * var_t1_dn8)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * var_t2_dn8))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((var_vg0_fp4s_dn9 - var_psim_fp4s_dn9) + ((((((0.5 * var_t1_dn9) * var_t1) + (assign30720_e48559 * var_t1_dn9)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * var_t2_dn9))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((var_vg0_fp4s_dn12 - var_psim_fp4s_dn12) + ((((((0.5 * var_t1_dn12) * var_t1) + (assign30720_e48559 * var_t1_dn12)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * var_t2_dn12))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((var_vg0_fp4s_dn14 - var_psim_fp4s_dn14) + ((((((0.5 * var_t1_dn14) * var_t1) + (assign30720_e48559 * var_t1_dn14)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * var_t2_dn14))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((var_vg0_fp4s_dn15 - var_psim_fp4s_dn15) + ((((((0.5 * var_t1_dn15) * var_t1) + (assign30720_e48559 * var_t1_dn15)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * var_t2_dn15))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((var_vg0_fp4s_dn16 - var_psim_fp4s_dn16) + ((((((0.5 * var_t1_dn16) * var_t1) + (assign30720_e48559 * var_t1_dn16)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * var_t2_dn16))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((var_vg0_fp4s_dn17 - var_psim_fp4s_dn17) + ((((((0.5 * var_t1_dn17) * var_t1) + (assign30720_e48559 * var_t1_dn17)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * var_t2_dn17))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((var_vg0_fp4s_dn18 - var_psim_fp4s_dn18) + ((((((0.5 * var_t1_dn18) * var_t1) + (assign30720_e48559 * var_t1_dn18)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * var_t2_dn18))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((var_vg0_fp4s_dn19 - var_psim_fp4s_dn19) + ((((((0.5 * var_t1_dn19) * var_t1) + (assign30720_e48559 * var_t1_dn19)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * var_t2_dn19))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((var_vg0_fp4s_dn20 - var_psim_fp4s_dn20) + ((((((0.5 * var_t1_dn20) * var_t1) + (assign30720_e48559 * var_t1_dn20)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * var_t2_dn20))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((var_vg0_fp4s_dn21 - var_psim_fp4s_dn21) + ((((((0.5 * var_t1_dn21) * var_t1) + (assign30720_e48559 * var_t1_dn21)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * var_t2_dn21))) / (assign30720_e48564 * assign30720_e48564)))), (assign30720_e48553 * ((var_vg0_fp4s_dn22 - var_psim_fp4s_dn22) + ((((((0.5 * var_t1_dn22) * var_t1) + (assign30720_e48559 * var_t1_dn22)) * assign30720_e48564) - (assign30720_e48561 * (6.0 * var_t2_dn22))) / (assign30720_e48564 * assign30720_e48564)))),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn1, var_t3_dn2, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn12, var_t3_dn14, var_t3_dn15, var_t3_dn16, var_t3_dn17, var_t3_dn18, var_t3_dn19, var_t3_dn20, var_t3_dn21, var_t3_dn22,)
    }
};
        var_t3 = assign30720_e48569;
        var_t3_dn0 = assign30720_e48569_d_n0;
        var_t3_dn1 = assign30720_e48569_d_n1;
        var_t3_dn2 = assign30720_e48569_d_n2;
        var_t3_dn3 = assign30720_e48569_d_n3;
        var_t3_dn4 = assign30720_e48569_d_n4;
        var_t3_dn5 = assign30720_e48569_d_n5;
        var_t3_dn6 = assign30720_e48569_d_n6;
        var_t3_dn7 = assign30720_e48569_d_n7;
        var_t3_dn8 = assign30720_e48569_d_n8;
        var_t3_dn9 = assign30720_e48569_d_n9;
        var_t3_dn12 = assign30720_e48569_d_n12;
        var_t3_dn14 = assign30720_e48569_d_n14;
        var_t3_dn15 = assign30720_e48569_d_n15;
        var_t3_dn16 = assign30720_e48569_d_n16;
        var_t3_dn17 = assign30720_e48569_d_n17;
        var_t3_dn18 = assign30720_e48569_d_n18;
        var_t3_dn19 = assign30720_e48569_d_n19;
        var_t3_dn20 = assign30720_e48569_d_n20;
        var_t3_dn21 = assign30720_e48569_d_n21;
        var_t3_dn22 = assign30720_e48569_d_n22;

        let (assign30730_e48580, assign30730_e48580_d_n0, assign30730_e48580_d_n1, assign30730_e48580_d_n2, assign30730_e48580_d_n3, assign30730_e48580_d_n4, assign30730_e48580_d_n5, assign30730_e48580_d_n6, assign30730_e48580_d_n7, assign30730_e48580_d_n8, assign30730_e48580_d_n9, assign30730_e48580_d_n12, assign30730_e48580_d_n14, assign30730_e48580_d_n15, assign30730_e48580_d_n16, assign30730_e48580_d_n17, assign30730_e48580_d_n18, assign30730_e48580_d_n19, assign30730_e48580_d_n20, assign30730_e48580_d_n21, assign30730_e48580_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30730_e48577: f64 = (var_t3 / p.p245);
        let assign30730_e48578: f64 = (1e26 * assign30730_e48577);
        (assign30730_e48578, (1e26 * (var_t3_dn0 / p.p245)), (1e26 * (var_t3_dn1 / p.p245)), (1e26 * (var_t3_dn2 / p.p245)), (1e26 * (var_t3_dn3 / p.p245)), (1e26 * (var_t3_dn4 / p.p245)), (1e26 * (var_t3_dn5 / p.p245)), (1e26 * (var_t3_dn6 / p.p245)), (1e26 * (var_t3_dn7 / p.p245)), (1e26 * (var_t3_dn8 / p.p245)), (1e26 * (var_t3_dn9 / p.p245)), (1e26 * (var_t3_dn12 / p.p245)), (1e26 * (var_t3_dn14 / p.p245)), (1e26 * (var_t3_dn15 / p.p245)), (1e26 * (var_t3_dn16 / p.p245)), (1e26 * (var_t3_dn17 / p.p245)), (1e26 * (var_t3_dn18 / p.p245)), (1e26 * (var_t3_dn19 / p.p245)), (1e26 * (var_t3_dn20 / p.p245)), (1e26 * (var_t3_dn21 / p.p245)), (1e26 * (var_t3_dn22 / p.p245)),)
    } else {
        (var_t0_1, var_t0_1_dn0, var_t0_1_dn1, var_t0_1_dn2, var_t0_1_dn3, var_t0_1_dn4, var_t0_1_dn5, var_t0_1_dn6, var_t0_1_dn7, var_t0_1_dn8, var_t0_1_dn9, var_t0_1_dn12, var_t0_1_dn14, var_t0_1_dn15, var_t0_1_dn16, var_t0_1_dn17, var_t0_1_dn18, var_t0_1_dn19, var_t0_1_dn20, var_t0_1_dn21, var_t0_1_dn22,)
    }
};
        var_t0_1 = assign30730_e48580;
        var_t0_1_dn0 = assign30730_e48580_d_n0;
        var_t0_1_dn1 = assign30730_e48580_d_n1;
        var_t0_1_dn2 = assign30730_e48580_d_n2;
        var_t0_1_dn3 = assign30730_e48580_d_n3;
        var_t0_1_dn4 = assign30730_e48580_d_n4;
        var_t0_1_dn5 = assign30730_e48580_d_n5;
        var_t0_1_dn6 = assign30730_e48580_d_n6;
        var_t0_1_dn7 = assign30730_e48580_d_n7;
        var_t0_1_dn8 = assign30730_e48580_d_n8;
        var_t0_1_dn9 = assign30730_e48580_d_n9;
        var_t0_1_dn12 = assign30730_e48580_d_n12;
        var_t0_1_dn14 = assign30730_e48580_d_n14;
        var_t0_1_dn15 = assign30730_e48580_d_n15;
        var_t0_1_dn16 = assign30730_e48580_d_n16;
        var_t0_1_dn17 = assign30730_e48580_d_n17;
        var_t0_1_dn18 = assign30730_e48580_d_n18;
        var_t0_1_dn19 = assign30730_e48580_d_n19;
        var_t0_1_dn20 = assign30730_e48580_d_n20;
        var_t0_1_dn21 = assign30730_e48580_d_n21;
        var_t0_1_dn22 = assign30730_e48580_d_n22;

        let (assign30740_e48591, assign30740_e48591_d_n0, assign30740_e48591_d_n1, assign30740_e48591_d_n2, assign30740_e48591_d_n3, assign30740_e48591_d_n4, assign30740_e48591_d_n5, assign30740_e48591_d_n6, assign30740_e48591_d_n7, assign30740_e48591_d_n8, assign30740_e48591_d_n9, assign30740_e48591_d_n12, assign30740_e48591_d_n14, assign30740_e48591_d_n15, assign30740_e48591_d_n16, assign30740_e48591_d_n17, assign30740_e48591_d_n18, assign30740_e48591_d_n19, assign30740_e48591_d_n20, assign30740_e48591_d_n21, assign30740_e48591_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30740_e48588: f64 = (var_t0_1).powf(p.p244);
        let assign30740_e48589: f64 = (1.0 + assign30740_e48588);
        (assign30740_e48589, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((var_t0_1).powf(p.p244 - 1.0) * var_t0_1_dn0)) } } else { (assign30740_e48588 * (p.p244 * (var_t0_1_dn0 / var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((var_t0_1).powf(p.p244 - 1.0) * var_t0_1_dn1)) } } else { (assign30740_e48588 * (p.p244 * (var_t0_1_dn1 / var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((var_t0_1).powf(p.p244 - 1.0) * var_t0_1_dn2)) } } else { (assign30740_e48588 * (p.p244 * (var_t0_1_dn2 / var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((var_t0_1).powf(p.p244 - 1.0) * var_t0_1_dn3)) } } else { (assign30740_e48588 * (p.p244 * (var_t0_1_dn3 / var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((var_t0_1).powf(p.p244 - 1.0) * var_t0_1_dn4)) } } else { (assign30740_e48588 * (p.p244 * (var_t0_1_dn4 / var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((var_t0_1).powf(p.p244 - 1.0) * var_t0_1_dn5)) } } else { (assign30740_e48588 * (p.p244 * (var_t0_1_dn5 / var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((var_t0_1).powf(p.p244 - 1.0) * var_t0_1_dn6)) } } else { (assign30740_e48588 * (p.p244 * (var_t0_1_dn6 / var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((var_t0_1).powf(p.p244 - 1.0) * var_t0_1_dn7)) } } else { (assign30740_e48588 * (p.p244 * (var_t0_1_dn7 / var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((var_t0_1).powf(p.p244 - 1.0) * var_t0_1_dn8)) } } else { (assign30740_e48588 * (p.p244 * (var_t0_1_dn8 / var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((var_t0_1).powf(p.p244 - 1.0) * var_t0_1_dn9)) } } else { (assign30740_e48588 * (p.p244 * (var_t0_1_dn9 / var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((var_t0_1).powf(p.p244 - 1.0) * var_t0_1_dn12)) } } else { (assign30740_e48588 * (p.p244 * (var_t0_1_dn12 / var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((var_t0_1).powf(p.p244 - 1.0) * var_t0_1_dn14)) } } else { (assign30740_e48588 * (p.p244 * (var_t0_1_dn14 / var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((var_t0_1).powf(p.p244 - 1.0) * var_t0_1_dn15)) } } else { (assign30740_e48588 * (p.p244 * (var_t0_1_dn15 / var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((var_t0_1).powf(p.p244 - 1.0) * var_t0_1_dn16)) } } else { (assign30740_e48588 * (p.p244 * (var_t0_1_dn16 / var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((var_t0_1).powf(p.p244 - 1.0) * var_t0_1_dn17)) } } else { (assign30740_e48588 * (p.p244 * (var_t0_1_dn17 / var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((var_t0_1).powf(p.p244 - 1.0) * var_t0_1_dn18)) } } else { (assign30740_e48588 * (p.p244 * (var_t0_1_dn18 / var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((var_t0_1).powf(p.p244 - 1.0) * var_t0_1_dn19)) } } else { (assign30740_e48588 * (p.p244 * (var_t0_1_dn19 / var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((var_t0_1).powf(p.p244 - 1.0) * var_t0_1_dn20)) } } else { (assign30740_e48588 * (p.p244 * (var_t0_1_dn20 / var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((var_t0_1).powf(p.p244 - 1.0) * var_t0_1_dn21)) } } else { (assign30740_e48588 * (p.p244 * (var_t0_1_dn21 / var_t0_1))) }, if 0.0 == 0.0 && ((p.p244) as f64).is_finite() && ((p.p244) as f64).fract() == 0.0 { if p.p244 == 0.0 { 0.0 } else { (p.p244 * ((var_t0_1).powf(p.p244 - 1.0) * var_t0_1_dn22)) } } else { (assign30740_e48588 * (p.p244 * (var_t0_1_dn22 / var_t0_1))) },)
    } else {
        (var_t1_1, var_t1_1_dn0, var_t1_1_dn1, var_t1_1_dn2, var_t1_1_dn3, var_t1_1_dn4, var_t1_1_dn5, var_t1_1_dn6, var_t1_1_dn7, var_t1_1_dn8, var_t1_1_dn9, var_t1_1_dn12, var_t1_1_dn14, var_t1_1_dn15, var_t1_1_dn16, var_t1_1_dn17, var_t1_1_dn18, var_t1_1_dn19, var_t1_1_dn20, var_t1_1_dn21, var_t1_1_dn22,)
    }
};
        var_t1_1 = assign30740_e48591;
        var_t1_1_dn0 = assign30740_e48591_d_n0;
        var_t1_1_dn1 = assign30740_e48591_d_n1;
        var_t1_1_dn2 = assign30740_e48591_d_n2;
        var_t1_1_dn3 = assign30740_e48591_d_n3;
        var_t1_1_dn4 = assign30740_e48591_d_n4;
        var_t1_1_dn5 = assign30740_e48591_d_n5;
        var_t1_1_dn6 = assign30740_e48591_d_n6;
        var_t1_1_dn7 = assign30740_e48591_d_n7;
        var_t1_1_dn8 = assign30740_e48591_d_n8;
        var_t1_1_dn9 = assign30740_e48591_d_n9;
        var_t1_1_dn12 = assign30740_e48591_d_n12;
        var_t1_1_dn14 = assign30740_e48591_d_n14;
        var_t1_1_dn15 = assign30740_e48591_d_n15;
        var_t1_1_dn16 = assign30740_e48591_d_n16;
        var_t1_1_dn17 = assign30740_e48591_d_n17;
        var_t1_1_dn18 = assign30740_e48591_d_n18;
        var_t1_1_dn19 = assign30740_e48591_d_n19;
        var_t1_1_dn20 = assign30740_e48591_d_n20;
        var_t1_1_dn21 = assign30740_e48591_d_n21;
        var_t1_1_dn22 = assign30740_e48591_d_n22;

        let (assign30750_e48600, assign30750_e48600_d_n0, assign30750_e48600_d_n1, assign30750_e48600_d_n2, assign30750_e48600_d_n3, assign30750_e48600_d_n4, assign30750_e48600_d_n5, assign30750_e48600_d_n6, assign30750_e48600_d_n7, assign30750_e48600_d_n8, assign30750_e48600_d_n9, assign30750_e48600_d_n12, assign30750_e48600_d_n14, assign30750_e48600_d_n15, assign30750_e48600_d_n16, assign30750_e48600_d_n17, assign30750_e48600_d_n18, assign30750_e48600_d_n19, assign30750_e48600_d_n20, assign30750_e48600_d_n21, assign30750_e48600_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30750_e48598: f64 = (p.p243 / var_t1_1);
        (assign30750_e48598, (-((p.p243 * var_t1_1_dn0) / (var_t1_1 * var_t1_1))), (-((p.p243 * var_t1_1_dn1) / (var_t1_1 * var_t1_1))), (-((p.p243 * var_t1_1_dn2) / (var_t1_1 * var_t1_1))), (-((p.p243 * var_t1_1_dn3) / (var_t1_1 * var_t1_1))), (-((p.p243 * var_t1_1_dn4) / (var_t1_1 * var_t1_1))), (-((p.p243 * var_t1_1_dn5) / (var_t1_1 * var_t1_1))), (-((p.p243 * var_t1_1_dn6) / (var_t1_1 * var_t1_1))), (-((p.p243 * var_t1_1_dn7) / (var_t1_1 * var_t1_1))), (-((p.p243 * var_t1_1_dn8) / (var_t1_1 * var_t1_1))), (-((p.p243 * var_t1_1_dn9) / (var_t1_1 * var_t1_1))), (-((p.p243 * var_t1_1_dn12) / (var_t1_1 * var_t1_1))), (-((p.p243 * var_t1_1_dn14) / (var_t1_1 * var_t1_1))), (-((p.p243 * var_t1_1_dn15) / (var_t1_1 * var_t1_1))), (-((p.p243 * var_t1_1_dn16) / (var_t1_1 * var_t1_1))), (-((p.p243 * var_t1_1_dn17) / (var_t1_1 * var_t1_1))), (-((p.p243 * var_t1_1_dn18) / (var_t1_1 * var_t1_1))), (-((p.p243 * var_t1_1_dn19) / (var_t1_1 * var_t1_1))), (-((p.p243 * var_t1_1_dn20) / (var_t1_1 * var_t1_1))), (-((p.p243 * var_t1_1_dn21) / (var_t1_1 * var_t1_1))), (-((p.p243 * var_t1_1_dn22) / (var_t1_1 * var_t1_1))),)
    } else {
        (var_xdcinv, var_xdcinv_dn0, var_xdcinv_dn1, var_xdcinv_dn2, var_xdcinv_dn3, var_xdcinv_dn4, var_xdcinv_dn5, var_xdcinv_dn6, var_xdcinv_dn7, var_xdcinv_dn8, var_xdcinv_dn9, var_xdcinv_dn12, var_xdcinv_dn14, var_xdcinv_dn15, var_xdcinv_dn16, var_xdcinv_dn17, var_xdcinv_dn18, var_xdcinv_dn19, var_xdcinv_dn20, var_xdcinv_dn21, var_xdcinv_dn22,)
    }
};
        var_xdcinv = assign30750_e48600;
        var_xdcinv_dn0 = assign30750_e48600_d_n0;
        var_xdcinv_dn1 = assign30750_e48600_d_n1;
        var_xdcinv_dn2 = assign30750_e48600_d_n2;
        var_xdcinv_dn3 = assign30750_e48600_d_n3;
        var_xdcinv_dn4 = assign30750_e48600_d_n4;
        var_xdcinv_dn5 = assign30750_e48600_d_n5;
        var_xdcinv_dn6 = assign30750_e48600_d_n6;
        var_xdcinv_dn7 = assign30750_e48600_d_n7;
        var_xdcinv_dn8 = assign30750_e48600_d_n8;
        var_xdcinv_dn9 = assign30750_e48600_d_n9;
        var_xdcinv_dn12 = assign30750_e48600_d_n12;
        var_xdcinv_dn14 = assign30750_e48600_d_n14;
        var_xdcinv_dn15 = assign30750_e48600_d_n15;
        var_xdcinv_dn16 = assign30750_e48600_d_n16;
        var_xdcinv_dn17 = assign30750_e48600_d_n17;
        var_xdcinv_dn18 = assign30750_e48600_d_n18;
        var_xdcinv_dn19 = assign30750_e48600_d_n19;
        var_xdcinv_dn20 = assign30750_e48600_d_n20;
        var_xdcinv_dn21 = assign30750_e48600_d_n21;
        var_xdcinv_dn22 = assign30750_e48600_d_n22;

        *var_ef3_slot = var_ef3;
        *var_ef3_dn0_slot = var_ef3_dn0;
        *var_ef3_dn1_slot = var_ef3_dn1;
        *var_ef3_dn12_slot = var_ef3_dn12;
        *var_ef3_dn14_slot = var_ef3_dn14;
        *var_ef3_dn15_slot = var_ef3_dn15;
        *var_ef3_dn16_slot = var_ef3_dn16;
        *var_ef3_dn17_slot = var_ef3_dn17;
        *var_ef3_dn18_slot = var_ef3_dn18;
        *var_ef3_dn19_slot = var_ef3_dn19;
        *var_ef3_dn2_slot = var_ef3_dn2;
        *var_ef3_dn20_slot = var_ef3_dn20;
        *var_ef3_dn21_slot = var_ef3_dn21;
        *var_ef3_dn22_slot = var_ef3_dn22;
        *var_ef3_dn3_slot = var_ef3_dn3;
        *var_ef3_dn4_slot = var_ef3_dn4;
        *var_ef3_dn5_slot = var_ef3_dn5;
        *var_ef3_dn6_slot = var_ef3_dn6;
        *var_ef3_dn7_slot = var_ef3_dn7;
        *var_ef3_dn8_slot = var_ef3_dn8;
        *var_ef3_dn9_slot = var_ef3_dn9;
        *var_psid_fp4s_slot = var_psid_fp4s;
        *var_psid_fp4s_dn0_slot = var_psid_fp4s_dn0;
        *var_psid_fp4s_dn1_slot = var_psid_fp4s_dn1;
        *var_psid_fp4s_dn12_slot = var_psid_fp4s_dn12;
        *var_psid_fp4s_dn14_slot = var_psid_fp4s_dn14;
        *var_psid_fp4s_dn15_slot = var_psid_fp4s_dn15;
        *var_psid_fp4s_dn16_slot = var_psid_fp4s_dn16;
        *var_psid_fp4s_dn17_slot = var_psid_fp4s_dn17;
        *var_psid_fp4s_dn18_slot = var_psid_fp4s_dn18;
        *var_psid_fp4s_dn19_slot = var_psid_fp4s_dn19;
        *var_psid_fp4s_dn2_slot = var_psid_fp4s_dn2;
        *var_psid_fp4s_dn20_slot = var_psid_fp4s_dn20;
        *var_psid_fp4s_dn21_slot = var_psid_fp4s_dn21;
        *var_psid_fp4s_dn22_slot = var_psid_fp4s_dn22;
        *var_psid_fp4s_dn3_slot = var_psid_fp4s_dn3;
        *var_psid_fp4s_dn4_slot = var_psid_fp4s_dn4;
        *var_psid_fp4s_dn5_slot = var_psid_fp4s_dn5;
        *var_psid_fp4s_dn6_slot = var_psid_fp4s_dn6;
        *var_psid_fp4s_dn7_slot = var_psid_fp4s_dn7;
        *var_psid_fp4s_dn8_slot = var_psid_fp4s_dn8;
        *var_psid_fp4s_dn9_slot = var_psid_fp4s_dn9;
        *var_psim_fp4s_slot = var_psim_fp4s;
        *var_psim_fp4s_dn0_slot = var_psim_fp4s_dn0;
        *var_psim_fp4s_dn1_slot = var_psim_fp4s_dn1;
        *var_psim_fp4s_dn12_slot = var_psim_fp4s_dn12;
        *var_psim_fp4s_dn14_slot = var_psim_fp4s_dn14;
        *var_psim_fp4s_dn15_slot = var_psim_fp4s_dn15;
        *var_psim_fp4s_dn16_slot = var_psim_fp4s_dn16;
        *var_psim_fp4s_dn17_slot = var_psim_fp4s_dn17;
        *var_psim_fp4s_dn18_slot = var_psim_fp4s_dn18;
        *var_psim_fp4s_dn19_slot = var_psim_fp4s_dn19;
        *var_psim_fp4s_dn2_slot = var_psim_fp4s_dn2;
        *var_psim_fp4s_dn20_slot = var_psim_fp4s_dn20;
        *var_psim_fp4s_dn21_slot = var_psim_fp4s_dn21;
        *var_psim_fp4s_dn22_slot = var_psim_fp4s_dn22;
        *var_psim_fp4s_dn3_slot = var_psim_fp4s_dn3;
        *var_psim_fp4s_dn4_slot = var_psim_fp4s_dn4;
        *var_psim_fp4s_dn5_slot = var_psim_fp4s_dn5;
        *var_psim_fp4s_dn6_slot = var_psim_fp4s_dn6;
        *var_psim_fp4s_dn7_slot = var_psim_fp4s_dn7;
        *var_psim_fp4s_dn8_slot = var_psim_fp4s_dn8;
        *var_psim_fp4s_dn9_slot = var_psim_fp4s_dn9;
        *var_psisd_fp4s_slot = var_psisd_fp4s;
        *var_psisd_fp4s_dn0_slot = var_psisd_fp4s_dn0;
        *var_psisd_fp4s_dn1_slot = var_psisd_fp4s_dn1;
        *var_psisd_fp4s_dn12_slot = var_psisd_fp4s_dn12;
        *var_psisd_fp4s_dn14_slot = var_psisd_fp4s_dn14;
        *var_psisd_fp4s_dn15_slot = var_psisd_fp4s_dn15;
        *var_psisd_fp4s_dn16_slot = var_psisd_fp4s_dn16;
        *var_psisd_fp4s_dn17_slot = var_psisd_fp4s_dn17;
        *var_psisd_fp4s_dn18_slot = var_psisd_fp4s_dn18;
        *var_psisd_fp4s_dn19_slot = var_psisd_fp4s_dn19;
        *var_psisd_fp4s_dn2_slot = var_psisd_fp4s_dn2;
        *var_psisd_fp4s_dn20_slot = var_psisd_fp4s_dn20;
        *var_psisd_fp4s_dn21_slot = var_psisd_fp4s_dn21;
        *var_psisd_fp4s_dn22_slot = var_psisd_fp4s_dn22;
        *var_psisd_fp4s_dn3_slot = var_psisd_fp4s_dn3;
        *var_psisd_fp4s_dn4_slot = var_psisd_fp4s_dn4;
        *var_psisd_fp4s_dn5_slot = var_psisd_fp4s_dn5;
        *var_psisd_fp4s_dn6_slot = var_psisd_fp4s_dn6;
        *var_psisd_fp4s_dn7_slot = var_psisd_fp4s_dn7;
        *var_psisd_fp4s_dn8_slot = var_psisd_fp4s_dn8;
        *var_psisd_fp4s_dn9_slot = var_psisd_fp4s_dn9;
        *var_t0_1_slot = var_t0_1;
        *var_t0_1_dn0_slot = var_t0_1_dn0;
        *var_t0_1_dn1_slot = var_t0_1_dn1;
        *var_t0_1_dn12_slot = var_t0_1_dn12;
        *var_t0_1_dn14_slot = var_t0_1_dn14;
        *var_t0_1_dn15_slot = var_t0_1_dn15;
        *var_t0_1_dn16_slot = var_t0_1_dn16;
        *var_t0_1_dn17_slot = var_t0_1_dn17;
        *var_t0_1_dn18_slot = var_t0_1_dn18;
        *var_t0_1_dn19_slot = var_t0_1_dn19;
        *var_t0_1_dn2_slot = var_t0_1_dn2;
        *var_t0_1_dn20_slot = var_t0_1_dn20;
        *var_t0_1_dn21_slot = var_t0_1_dn21;
        *var_t0_1_dn22_slot = var_t0_1_dn22;
        *var_t0_1_dn3_slot = var_t0_1_dn3;
        *var_t0_1_dn4_slot = var_t0_1_dn4;
        *var_t0_1_dn5_slot = var_t0_1_dn5;
        *var_t0_1_dn6_slot = var_t0_1_dn6;
        *var_t0_1_dn7_slot = var_t0_1_dn7;
        *var_t0_1_dn8_slot = var_t0_1_dn8;
        *var_t0_1_dn9_slot = var_t0_1_dn9;
        *var_t1_slot = var_t1;
        *var_t1_1_slot = var_t1_1;
        *var_t1_1_dn0_slot = var_t1_1_dn0;
        *var_t1_1_dn1_slot = var_t1_1_dn1;
        *var_t1_1_dn12_slot = var_t1_1_dn12;
        *var_t1_1_dn14_slot = var_t1_1_dn14;
        *var_t1_1_dn15_slot = var_t1_1_dn15;
        *var_t1_1_dn16_slot = var_t1_1_dn16;
        *var_t1_1_dn17_slot = var_t1_1_dn17;
        *var_t1_1_dn18_slot = var_t1_1_dn18;
        *var_t1_1_dn19_slot = var_t1_1_dn19;
        *var_t1_1_dn2_slot = var_t1_1_dn2;
        *var_t1_1_dn20_slot = var_t1_1_dn20;
        *var_t1_1_dn21_slot = var_t1_1_dn21;
        *var_t1_1_dn22_slot = var_t1_1_dn22;
        *var_t1_1_dn3_slot = var_t1_1_dn3;
        *var_t1_1_dn4_slot = var_t1_1_dn4;
        *var_t1_1_dn5_slot = var_t1_1_dn5;
        *var_t1_1_dn6_slot = var_t1_1_dn6;
        *var_t1_1_dn7_slot = var_t1_1_dn7;
        *var_t1_1_dn8_slot = var_t1_1_dn8;
        *var_t1_1_dn9_slot = var_t1_1_dn9;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn1_slot = var_t1_dn1;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn15_slot = var_t1_dn15;
        *var_t1_dn16_slot = var_t1_dn16;
        *var_t1_dn17_slot = var_t1_dn17;
        *var_t1_dn18_slot = var_t1_dn18;
        *var_t1_dn19_slot = var_t1_dn19;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn20_slot = var_t1_dn20;
        *var_t1_dn21_slot = var_t1_dn21;
        *var_t1_dn22_slot = var_t1_dn22;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn1_slot = var_t2_dn1;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn15_slot = var_t2_dn15;
        *var_t2_dn16_slot = var_t2_dn16;
        *var_t2_dn17_slot = var_t2_dn17;
        *var_t2_dn18_slot = var_t2_dn18;
        *var_t2_dn19_slot = var_t2_dn19;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn20_slot = var_t2_dn20;
        *var_t2_dn21_slot = var_t2_dn21;
        *var_t2_dn22_slot = var_t2_dn22;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn1_slot = var_t3_dn1;
        *var_t3_dn12_slot = var_t3_dn12;
        *var_t3_dn14_slot = var_t3_dn14;
        *var_t3_dn15_slot = var_t3_dn15;
        *var_t3_dn16_slot = var_t3_dn16;
        *var_t3_dn17_slot = var_t3_dn17;
        *var_t3_dn18_slot = var_t3_dn18;
        *var_t3_dn19_slot = var_t3_dn19;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn20_slot = var_t3_dn20;
        *var_t3_dn21_slot = var_t3_dn21;
        *var_t3_dn22_slot = var_t3_dn22;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t52_slot = var_t52;
        *var_t52_dn0_slot = var_t52_dn0;
        *var_t52_dn1_slot = var_t52_dn1;
        *var_t52_dn12_slot = var_t52_dn12;
        *var_t52_dn14_slot = var_t52_dn14;
        *var_t52_dn15_slot = var_t52_dn15;
        *var_t52_dn16_slot = var_t52_dn16;
        *var_t52_dn17_slot = var_t52_dn17;
        *var_t52_dn18_slot = var_t52_dn18;
        *var_t52_dn19_slot = var_t52_dn19;
        *var_t52_dn2_slot = var_t52_dn2;
        *var_t52_dn20_slot = var_t52_dn20;
        *var_t52_dn21_slot = var_t52_dn21;
        *var_t52_dn22_slot = var_t52_dn22;
        *var_t52_dn3_slot = var_t52_dn3;
        *var_t52_dn4_slot = var_t52_dn4;
        *var_t52_dn5_slot = var_t52_dn5;
        *var_t52_dn6_slot = var_t52_dn6;
        *var_t52_dn7_slot = var_t52_dn7;
        *var_t52_dn8_slot = var_t52_dn8;
        *var_t52_dn9_slot = var_t52_dn9;
        *var_t5dg02_slot = var_t5dg02;
        *var_t5dg02_dn0_slot = var_t5dg02_dn0;
        *var_t5dg02_dn1_slot = var_t5dg02_dn1;
        *var_t5dg02_dn12_slot = var_t5dg02_dn12;
        *var_t5dg02_dn14_slot = var_t5dg02_dn14;
        *var_t5dg02_dn15_slot = var_t5dg02_dn15;
        *var_t5dg02_dn16_slot = var_t5dg02_dn16;
        *var_t5dg02_dn17_slot = var_t5dg02_dn17;
        *var_t5dg02_dn18_slot = var_t5dg02_dn18;
        *var_t5dg02_dn19_slot = var_t5dg02_dn19;
        *var_t5dg02_dn2_slot = var_t5dg02_dn2;
        *var_t5dg02_dn20_slot = var_t5dg02_dn20;
        *var_t5dg02_dn21_slot = var_t5dg02_dn21;
        *var_t5dg02_dn22_slot = var_t5dg02_dn22;
        *var_t5dg02_dn3_slot = var_t5dg02_dn3;
        *var_t5dg02_dn4_slot = var_t5dg02_dn4;
        *var_t5dg02_dn5_slot = var_t5dg02_dn5;
        *var_t5dg02_dn6_slot = var_t5dg02_dn6;
        *var_t5dg02_dn7_slot = var_t5dg02_dn7;
        *var_t5dg02_dn8_slot = var_t5dg02_dn8;
        *var_t5dg02_dn9_slot = var_t5dg02_dn9;
        *var_t5dg12_slot = var_t5dg12;
        *var_t5dg12_dn0_slot = var_t5dg12_dn0;
        *var_t5dg12_dn1_slot = var_t5dg12_dn1;
        *var_t5dg12_dn12_slot = var_t5dg12_dn12;
        *var_t5dg12_dn14_slot = var_t5dg12_dn14;
        *var_t5dg12_dn15_slot = var_t5dg12_dn15;
        *var_t5dg12_dn16_slot = var_t5dg12_dn16;
        *var_t5dg12_dn17_slot = var_t5dg12_dn17;
        *var_t5dg12_dn18_slot = var_t5dg12_dn18;
        *var_t5dg12_dn19_slot = var_t5dg12_dn19;
        *var_t5dg12_dn2_slot = var_t5dg12_dn2;
        *var_t5dg12_dn20_slot = var_t5dg12_dn20;
        *var_t5dg12_dn21_slot = var_t5dg12_dn21;
        *var_t5dg12_dn22_slot = var_t5dg12_dn22;
        *var_t5dg12_dn3_slot = var_t5dg12_dn3;
        *var_t5dg12_dn4_slot = var_t5dg12_dn4;
        *var_t5dg12_dn5_slot = var_t5dg12_dn5;
        *var_t5dg12_dn6_slot = var_t5dg12_dn6;
        *var_t5dg12_dn7_slot = var_t5dg12_dn7;
        *var_t5dg12_dn8_slot = var_t5dg12_dn8;
        *var_t5dg12_dn9_slot = var_t5dg12_dn9;
        *var_t5ng02_slot = var_t5ng02;
        *var_t5ng02_dn0_slot = var_t5ng02_dn0;
        *var_t5ng02_dn1_slot = var_t5ng02_dn1;
        *var_t5ng02_dn12_slot = var_t5ng02_dn12;
        *var_t5ng02_dn14_slot = var_t5ng02_dn14;
        *var_t5ng02_dn15_slot = var_t5ng02_dn15;
        *var_t5ng02_dn16_slot = var_t5ng02_dn16;
        *var_t5ng02_dn17_slot = var_t5ng02_dn17;
        *var_t5ng02_dn18_slot = var_t5ng02_dn18;
        *var_t5ng02_dn19_slot = var_t5ng02_dn19;
        *var_t5ng02_dn2_slot = var_t5ng02_dn2;
        *var_t5ng02_dn20_slot = var_t5ng02_dn20;
        *var_t5ng02_dn21_slot = var_t5ng02_dn21;
        *var_t5ng02_dn22_slot = var_t5ng02_dn22;
        *var_t5ng02_dn3_slot = var_t5ng02_dn3;
        *var_t5ng02_dn4_slot = var_t5ng02_dn4;
        *var_t5ng02_dn5_slot = var_t5ng02_dn5;
        *var_t5ng02_dn6_slot = var_t5ng02_dn6;
        *var_t5ng02_dn7_slot = var_t5ng02_dn7;
        *var_t5ng02_dn8_slot = var_t5ng02_dn8;
        *var_t5ng02_dn9_slot = var_t5ng02_dn9;
        *var_t5ng12_slot = var_t5ng12;
        *var_t5ng12_dn0_slot = var_t5ng12_dn0;
        *var_t5ng12_dn1_slot = var_t5ng12_dn1;
        *var_t5ng12_dn12_slot = var_t5ng12_dn12;
        *var_t5ng12_dn14_slot = var_t5ng12_dn14;
        *var_t5ng12_dn15_slot = var_t5ng12_dn15;
        *var_t5ng12_dn16_slot = var_t5ng12_dn16;
        *var_t5ng12_dn17_slot = var_t5ng12_dn17;
        *var_t5ng12_dn18_slot = var_t5ng12_dn18;
        *var_t5ng12_dn19_slot = var_t5ng12_dn19;
        *var_t5ng12_dn2_slot = var_t5ng12_dn2;
        *var_t5ng12_dn20_slot = var_t5ng12_dn20;
        *var_t5ng12_dn21_slot = var_t5ng12_dn21;
        *var_t5ng12_dn22_slot = var_t5ng12_dn22;
        *var_t5ng12_dn3_slot = var_t5ng12_dn3;
        *var_t5ng12_dn4_slot = var_t5ng12_dn4;
        *var_t5ng12_dn5_slot = var_t5ng12_dn5;
        *var_t5ng12_dn6_slot = var_t5ng12_dn6;
        *var_t5ng12_dn7_slot = var_t5ng12_dn7;
        *var_t5ng12_dn8_slot = var_t5ng12_dn8;
        *var_t5ng12_dn9_slot = var_t5ng12_dn9;
        *var_xdcinv_slot = var_xdcinv;
        *var_xdcinv_dn0_slot = var_xdcinv_dn0;
        *var_xdcinv_dn1_slot = var_xdcinv_dn1;
        *var_xdcinv_dn12_slot = var_xdcinv_dn12;
        *var_xdcinv_dn14_slot = var_xdcinv_dn14;
        *var_xdcinv_dn15_slot = var_xdcinv_dn15;
        *var_xdcinv_dn16_slot = var_xdcinv_dn16;
        *var_xdcinv_dn17_slot = var_xdcinv_dn17;
        *var_xdcinv_dn18_slot = var_xdcinv_dn18;
        *var_xdcinv_dn19_slot = var_xdcinv_dn19;
        *var_xdcinv_dn2_slot = var_xdcinv_dn2;
        *var_xdcinv_dn20_slot = var_xdcinv_dn20;
        *var_xdcinv_dn21_slot = var_xdcinv_dn21;
        *var_xdcinv_dn22_slot = var_xdcinv_dn22;
        *var_xdcinv_dn3_slot = var_xdcinv_dn3;
        *var_xdcinv_dn4_slot = var_xdcinv_dn4;
        *var_xdcinv_dn5_slot = var_xdcinv_dn5;
        *var_xdcinv_dn6_slot = var_xdcinv_dn6;
        *var_xdcinv_dn7_slot = var_xdcinv_dn7;
        *var_xdcinv_dn8_slot = var_xdcinv_dn8;
        *var_xdcinv_dn9_slot = var_xdcinv_dn9;
    }

    pub(super) fn stamp_transient_block_181(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_guard504: f64,
        var_guard513: f64,
        var_psid_fp4s: f64,
        var_psid_fp4s_dn0: f64,
        var_psid_fp4s_dn1: f64,
        var_psid_fp4s_dn12: f64,
        var_psid_fp4s_dn14: f64,
        var_psid_fp4s_dn15: f64,
        var_psid_fp4s_dn16: f64,
        var_psid_fp4s_dn17: f64,
        var_psid_fp4s_dn18: f64,
        var_psid_fp4s_dn19: f64,
        var_psid_fp4s_dn2: f64,
        var_psid_fp4s_dn20: f64,
        var_psid_fp4s_dn21: f64,
        var_psid_fp4s_dn22: f64,
        var_psid_fp4s_dn3: f64,
        var_psid_fp4s_dn4: f64,
        var_psid_fp4s_dn5: f64,
        var_psid_fp4s_dn6: f64,
        var_psid_fp4s_dn7: f64,
        var_psid_fp4s_dn8: f64,
        var_psid_fp4s_dn9: f64,
        var_psim_fp4s: f64,
        var_psim_fp4s_dn0: f64,
        var_psim_fp4s_dn1: f64,
        var_psim_fp4s_dn12: f64,
        var_psim_fp4s_dn14: f64,
        var_psim_fp4s_dn15: f64,
        var_psim_fp4s_dn16: f64,
        var_psim_fp4s_dn17: f64,
        var_psim_fp4s_dn18: f64,
        var_psim_fp4s_dn19: f64,
        var_psim_fp4s_dn2: f64,
        var_psim_fp4s_dn20: f64,
        var_psim_fp4s_dn21: f64,
        var_psim_fp4s_dn22: f64,
        var_psim_fp4s_dn3: f64,
        var_psim_fp4s_dn4: f64,
        var_psim_fp4s_dn5: f64,
        var_psim_fp4s_dn6: f64,
        var_psim_fp4s_dn7: f64,
        var_psim_fp4s_dn8: f64,
        var_psim_fp4s_dn9: f64,
        var_psis_fp4s: f64,
        var_psis_fp4s_dn0: f64,
        var_psis_fp4s_dn1: f64,
        var_psis_fp4s_dn12: f64,
        var_psis_fp4s_dn14: f64,
        var_psis_fp4s_dn15: f64,
        var_psis_fp4s_dn16: f64,
        var_psis_fp4s_dn17: f64,
        var_psis_fp4s_dn18: f64,
        var_psis_fp4s_dn19: f64,
        var_psis_fp4s_dn2: f64,
        var_psis_fp4s_dn20: f64,
        var_psis_fp4s_dn21: f64,
        var_psis_fp4s_dn22: f64,
        var_psis_fp4s_dn3: f64,
        var_psis_fp4s_dn4: f64,
        var_psis_fp4s_dn5: f64,
        var_psis_fp4s_dn6: f64,
        var_psis_fp4s_dn7: f64,
        var_psis_fp4s_dn8: f64,
        var_psis_fp4s_dn9: f64,
        var_psisd_fp4s: f64,
        var_psisd_fp4s_dn0: f64,
        var_psisd_fp4s_dn1: f64,
        var_psisd_fp4s_dn12: f64,
        var_psisd_fp4s_dn14: f64,
        var_psisd_fp4s_dn15: f64,
        var_psisd_fp4s_dn16: f64,
        var_psisd_fp4s_dn17: f64,
        var_psisd_fp4s_dn18: f64,
        var_psisd_fp4s_dn19: f64,
        var_psisd_fp4s_dn2: f64,
        var_psisd_fp4s_dn20: f64,
        var_psisd_fp4s_dn21: f64,
        var_psisd_fp4s_dn22: f64,
        var_psisd_fp4s_dn3: f64,
        var_psisd_fp4s_dn4: f64,
        var_psisd_fp4s_dn5: f64,
        var_psisd_fp4s_dn6: f64,
        var_psisd_fp4s_dn7: f64,
        var_psisd_fp4s_dn8: f64,
        var_psisd_fp4s_dn9: f64,
        var_tdev: f64,
        var_tdev_dn4: f64,
        var_tnom: f64,
        var_vg0_fp4s: f64,
        var_vg0_fp4s_dn0: f64,
        var_vg0_fp4s_dn1: f64,
        var_vg0_fp4s_dn12: f64,
        var_vg0_fp4s_dn14: f64,
        var_vg0_fp4s_dn15: f64,
        var_vg0_fp4s_dn16: f64,
        var_vg0_fp4s_dn17: f64,
        var_vg0_fp4s_dn18: f64,
        var_vg0_fp4s_dn19: f64,
        var_vg0_fp4s_dn2: f64,
        var_vg0_fp4s_dn20: f64,
        var_vg0_fp4s_dn21: f64,
        var_vg0_fp4s_dn22: f64,
        var_vg0_fp4s_dn3: f64,
        var_vg0_fp4s_dn4: f64,
        var_vg0_fp4s_dn5: f64,
        var_vg0_fp4s_dn6: f64,
        var_vg0_fp4s_dn7: f64,
        var_vg0_fp4s_dn8: f64,
        var_vg0_fp4s_dn9: f64,
        var_vtv: f64,
        var_vtv_dn15: f64,
        var_vtv_dn16: f64,
        var_vtv_dn17: f64,
        var_vtv_dn18: f64,
        var_vtv_dn19: f64,
        var_vtv_dn20: f64,
        var_vtv_dn21: f64,
        var_vtv_dn22: f64,
        var_vtv_dn4: f64,
        var_vtv_dn6: f64,
        var_vtv_dn7: f64,
        var_vtv_dn8: f64,
        var_xdcinv: f64,
        var_xdcinv_dn0: f64,
        var_xdcinv_dn1: f64,
        var_xdcinv_dn12: f64,
        var_xdcinv_dn14: f64,
        var_xdcinv_dn15: f64,
        var_xdcinv_dn16: f64,
        var_xdcinv_dn17: f64,
        var_xdcinv_dn18: f64,
        var_xdcinv_dn19: f64,
        var_xdcinv_dn2: f64,
        var_xdcinv_dn20: f64,
        var_xdcinv_dn21: f64,
        var_xdcinv_dn22: f64,
        var_xdcinv_dn3: f64,
        var_xdcinv_dn4: f64,
        var_xdcinv_dn5: f64,
        var_xdcinv_dn6: f64,
        var_xdcinv_dn7: f64,
        var_xdcinv_dn8: f64,
        var_xdcinv_dn9: f64,
        var_cg_qme_slot: &mut f64,
        var_cg_qme_dn0_slot: &mut f64,
        var_cg_qme_dn1_slot: &mut f64,
        var_cg_qme_dn12_slot: &mut f64,
        var_cg_qme_dn14_slot: &mut f64,
        var_cg_qme_dn15_slot: &mut f64,
        var_cg_qme_dn16_slot: &mut f64,
        var_cg_qme_dn17_slot: &mut f64,
        var_cg_qme_dn18_slot: &mut f64,
        var_cg_qme_dn19_slot: &mut f64,
        var_cg_qme_dn2_slot: &mut f64,
        var_cg_qme_dn20_slot: &mut f64,
        var_cg_qme_dn21_slot: &mut f64,
        var_cg_qme_dn22_slot: &mut f64,
        var_cg_qme_dn3_slot: &mut f64,
        var_cg_qme_dn4_slot: &mut f64,
        var_cg_qme_dn5_slot: &mut f64,
        var_cg_qme_dn6_slot: &mut f64,
        var_cg_qme_dn7_slot: &mut f64,
        var_cg_qme_dn8_slot: &mut f64,
        var_cg_qme_dn9_slot: &mut f64,
        var_cgdl_l_slot: &mut f64,
        var_cgdvar_slot: &mut f64,
        var_cgdvar_dn0_slot: &mut f64,
        var_cgdvar_dn2_slot: &mut f64,
        var_guard524_slot: &mut f64,
        var_isb_t_slot: &mut f64,
        var_isb_t_dn4_slot: &mut f64,
        var_nsb_t_slot: &mut f64,
        var_nsb_t_dn4_slot: &mut f64,
        var_qbdov_slot: &mut f64,
        var_qbdov_dn0_slot: &mut f64,
        var_qbdov_dn3_slot: &mut f64,
        var_qbgov_slot: &mut f64,
        var_qbgov_dn1_slot: &mut f64,
        var_qbgov_dn3_slot: &mut f64,
        var_qbsov_slot: &mut f64,
        var_qbsov_dn2_slot: &mut f64,
        var_qbsov_dn3_slot: &mut f64,
        var_qd_fp4s_slot: &mut f64,
        var_qd_fp4s_dn0_slot: &mut f64,
        var_qd_fp4s_dn1_slot: &mut f64,
        var_qd_fp4s_dn12_slot: &mut f64,
        var_qd_fp4s_dn14_slot: &mut f64,
        var_qd_fp4s_dn15_slot: &mut f64,
        var_qd_fp4s_dn16_slot: &mut f64,
        var_qd_fp4s_dn17_slot: &mut f64,
        var_qd_fp4s_dn18_slot: &mut f64,
        var_qd_fp4s_dn19_slot: &mut f64,
        var_qd_fp4s_dn2_slot: &mut f64,
        var_qd_fp4s_dn20_slot: &mut f64,
        var_qd_fp4s_dn21_slot: &mut f64,
        var_qd_fp4s_dn22_slot: &mut f64,
        var_qd_fp4s_dn3_slot: &mut f64,
        var_qd_fp4s_dn4_slot: &mut f64,
        var_qd_fp4s_dn5_slot: &mut f64,
        var_qd_fp4s_dn6_slot: &mut f64,
        var_qd_fp4s_dn7_slot: &mut f64,
        var_qd_fp4s_dn8_slot: &mut f64,
        var_qd_fp4s_dn9_slot: &mut f64,
        var_qdov_slot: &mut f64,
        var_qdov_dn0_slot: &mut f64,
        var_qdov_dn1_slot: &mut f64,
        var_qdov_dn10_slot: &mut f64,
        var_qdov_dn2_slot: &mut f64,
        var_qdsov_slot: &mut f64,
        var_qdsov_dn0_slot: &mut f64,
        var_qdsov_dn2_slot: &mut f64,
        var_qg_fp4s_slot: &mut f64,
        var_qg_fp4s_dn0_slot: &mut f64,
        var_qg_fp4s_dn1_slot: &mut f64,
        var_qg_fp4s_dn12_slot: &mut f64,
        var_qg_fp4s_dn14_slot: &mut f64,
        var_qg_fp4s_dn15_slot: &mut f64,
        var_qg_fp4s_dn16_slot: &mut f64,
        var_qg_fp4s_dn17_slot: &mut f64,
        var_qg_fp4s_dn18_slot: &mut f64,
        var_qg_fp4s_dn19_slot: &mut f64,
        var_qg_fp4s_dn2_slot: &mut f64,
        var_qg_fp4s_dn20_slot: &mut f64,
        var_qg_fp4s_dn21_slot: &mut f64,
        var_qg_fp4s_dn22_slot: &mut f64,
        var_qg_fp4s_dn3_slot: &mut f64,
        var_qg_fp4s_dn4_slot: &mut f64,
        var_qg_fp4s_dn5_slot: &mut f64,
        var_qg_fp4s_dn6_slot: &mut f64,
        var_qg_fp4s_dn7_slot: &mut f64,
        var_qg_fp4s_dn8_slot: &mut f64,
        var_qg_fp4s_dn9_slot: &mut f64,
        var_qsov_slot: &mut f64,
        var_qsov_dn1_slot: &mut f64,
        var_qsov_dn10_slot: &mut f64,
        var_qsov_dn2_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn1_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn14_slot: &mut f64,
        var_t0_dn15_slot: &mut f64,
        var_t0_dn16_slot: &mut f64,
        var_t0_dn17_slot: &mut f64,
        var_t0_dn18_slot: &mut f64,
        var_t0_dn19_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn20_slot: &mut f64,
        var_t0_dn21_slot: &mut f64,
        var_t0_dn22_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn1_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn15_slot: &mut f64,
        var_t1_dn16_slot: &mut f64,
        var_t1_dn17_slot: &mut f64,
        var_t1_dn18_slot: &mut f64,
        var_t1_dn19_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn20_slot: &mut f64,
        var_t1_dn21_slot: &mut f64,
        var_t1_dn22_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn1_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn15_slot: &mut f64,
        var_t2_dn16_slot: &mut f64,
        var_t2_dn17_slot: &mut f64,
        var_t2_dn18_slot: &mut f64,
        var_t2_dn19_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn20_slot: &mut f64,
        var_t2_dn21_slot: &mut f64,
        var_t2_dn22_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn1_slot: &mut f64,
        var_t3_dn12_slot: &mut f64,
        var_t3_dn14_slot: &mut f64,
        var_t3_dn15_slot: &mut f64,
        var_t3_dn16_slot: &mut f64,
        var_t3_dn17_slot: &mut f64,
        var_t3_dn18_slot: &mut f64,
        var_t3_dn19_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn20_slot: &mut f64,
        var_t3_dn21_slot: &mut f64,
        var_t3_dn22_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_vbidb_t_slot: &mut f64,
        var_vbidb_t_dn4_slot: &mut f64,
        var_vbisb_t_slot: &mut f64,
        var_vbisb_t_dn4_slot: &mut f64,
        var_vdseffcv_slot: &mut f64,
        var_vdseffcv_dn0_slot: &mut f64,
        var_vdseffcv_dn2_slot: &mut f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let mut var_cg_qme: f64 = *var_cg_qme_slot;
        let mut var_cg_qme_dn0: f64 = *var_cg_qme_dn0_slot;
        let mut var_cg_qme_dn1: f64 = *var_cg_qme_dn1_slot;
        let mut var_cg_qme_dn12: f64 = *var_cg_qme_dn12_slot;
        let mut var_cg_qme_dn14: f64 = *var_cg_qme_dn14_slot;
        let mut var_cg_qme_dn15: f64 = *var_cg_qme_dn15_slot;
        let mut var_cg_qme_dn16: f64 = *var_cg_qme_dn16_slot;
        let mut var_cg_qme_dn17: f64 = *var_cg_qme_dn17_slot;
        let mut var_cg_qme_dn18: f64 = *var_cg_qme_dn18_slot;
        let mut var_cg_qme_dn19: f64 = *var_cg_qme_dn19_slot;
        let mut var_cg_qme_dn2: f64 = *var_cg_qme_dn2_slot;
        let mut var_cg_qme_dn20: f64 = *var_cg_qme_dn20_slot;
        let mut var_cg_qme_dn21: f64 = *var_cg_qme_dn21_slot;
        let mut var_cg_qme_dn22: f64 = *var_cg_qme_dn22_slot;
        let mut var_cg_qme_dn3: f64 = *var_cg_qme_dn3_slot;
        let mut var_cg_qme_dn4: f64 = *var_cg_qme_dn4_slot;
        let mut var_cg_qme_dn5: f64 = *var_cg_qme_dn5_slot;
        let mut var_cg_qme_dn6: f64 = *var_cg_qme_dn6_slot;
        let mut var_cg_qme_dn7: f64 = *var_cg_qme_dn7_slot;
        let mut var_cg_qme_dn8: f64 = *var_cg_qme_dn8_slot;
        let mut var_cg_qme_dn9: f64 = *var_cg_qme_dn9_slot;
        let mut var_cgdl_l: f64 = *var_cgdl_l_slot;
        let mut var_cgdvar: f64 = *var_cgdvar_slot;
        let mut var_cgdvar_dn0: f64 = *var_cgdvar_dn0_slot;
        let mut var_cgdvar_dn2: f64 = *var_cgdvar_dn2_slot;
        let mut var_guard524: f64 = *var_guard524_slot;
        let mut var_isb_t: f64 = *var_isb_t_slot;
        let mut var_isb_t_dn4: f64 = *var_isb_t_dn4_slot;
        let mut var_nsb_t: f64 = *var_nsb_t_slot;
        let mut var_nsb_t_dn4: f64 = *var_nsb_t_dn4_slot;
        let mut var_qbdov: f64 = *var_qbdov_slot;
        let mut var_qbdov_dn0: f64 = *var_qbdov_dn0_slot;
        let mut var_qbdov_dn3: f64 = *var_qbdov_dn3_slot;
        let mut var_qbgov: f64 = *var_qbgov_slot;
        let mut var_qbgov_dn1: f64 = *var_qbgov_dn1_slot;
        let mut var_qbgov_dn3: f64 = *var_qbgov_dn3_slot;
        let mut var_qbsov: f64 = *var_qbsov_slot;
        let mut var_qbsov_dn2: f64 = *var_qbsov_dn2_slot;
        let mut var_qbsov_dn3: f64 = *var_qbsov_dn3_slot;
        let mut var_qd_fp4s: f64 = *var_qd_fp4s_slot;
        let mut var_qd_fp4s_dn0: f64 = *var_qd_fp4s_dn0_slot;
        let mut var_qd_fp4s_dn1: f64 = *var_qd_fp4s_dn1_slot;
        let mut var_qd_fp4s_dn12: f64 = *var_qd_fp4s_dn12_slot;
        let mut var_qd_fp4s_dn14: f64 = *var_qd_fp4s_dn14_slot;
        let mut var_qd_fp4s_dn15: f64 = *var_qd_fp4s_dn15_slot;
        let mut var_qd_fp4s_dn16: f64 = *var_qd_fp4s_dn16_slot;
        let mut var_qd_fp4s_dn17: f64 = *var_qd_fp4s_dn17_slot;
        let mut var_qd_fp4s_dn18: f64 = *var_qd_fp4s_dn18_slot;
        let mut var_qd_fp4s_dn19: f64 = *var_qd_fp4s_dn19_slot;
        let mut var_qd_fp4s_dn2: f64 = *var_qd_fp4s_dn2_slot;
        let mut var_qd_fp4s_dn20: f64 = *var_qd_fp4s_dn20_slot;
        let mut var_qd_fp4s_dn21: f64 = *var_qd_fp4s_dn21_slot;
        let mut var_qd_fp4s_dn22: f64 = *var_qd_fp4s_dn22_slot;
        let mut var_qd_fp4s_dn3: f64 = *var_qd_fp4s_dn3_slot;
        let mut var_qd_fp4s_dn4: f64 = *var_qd_fp4s_dn4_slot;
        let mut var_qd_fp4s_dn5: f64 = *var_qd_fp4s_dn5_slot;
        let mut var_qd_fp4s_dn6: f64 = *var_qd_fp4s_dn6_slot;
        let mut var_qd_fp4s_dn7: f64 = *var_qd_fp4s_dn7_slot;
        let mut var_qd_fp4s_dn8: f64 = *var_qd_fp4s_dn8_slot;
        let mut var_qd_fp4s_dn9: f64 = *var_qd_fp4s_dn9_slot;
        let mut var_qdov: f64 = *var_qdov_slot;
        let mut var_qdov_dn0: f64 = *var_qdov_dn0_slot;
        let mut var_qdov_dn1: f64 = *var_qdov_dn1_slot;
        let mut var_qdov_dn10: f64 = *var_qdov_dn10_slot;
        let mut var_qdov_dn2: f64 = *var_qdov_dn2_slot;
        let mut var_qdsov: f64 = *var_qdsov_slot;
        let mut var_qdsov_dn0: f64 = *var_qdsov_dn0_slot;
        let mut var_qdsov_dn2: f64 = *var_qdsov_dn2_slot;
        let mut var_qg_fp4s: f64 = *var_qg_fp4s_slot;
        let mut var_qg_fp4s_dn0: f64 = *var_qg_fp4s_dn0_slot;
        let mut var_qg_fp4s_dn1: f64 = *var_qg_fp4s_dn1_slot;
        let mut var_qg_fp4s_dn12: f64 = *var_qg_fp4s_dn12_slot;
        let mut var_qg_fp4s_dn14: f64 = *var_qg_fp4s_dn14_slot;
        let mut var_qg_fp4s_dn15: f64 = *var_qg_fp4s_dn15_slot;
        let mut var_qg_fp4s_dn16: f64 = *var_qg_fp4s_dn16_slot;
        let mut var_qg_fp4s_dn17: f64 = *var_qg_fp4s_dn17_slot;
        let mut var_qg_fp4s_dn18: f64 = *var_qg_fp4s_dn18_slot;
        let mut var_qg_fp4s_dn19: f64 = *var_qg_fp4s_dn19_slot;
        let mut var_qg_fp4s_dn2: f64 = *var_qg_fp4s_dn2_slot;
        let mut var_qg_fp4s_dn20: f64 = *var_qg_fp4s_dn20_slot;
        let mut var_qg_fp4s_dn21: f64 = *var_qg_fp4s_dn21_slot;
        let mut var_qg_fp4s_dn22: f64 = *var_qg_fp4s_dn22_slot;
        let mut var_qg_fp4s_dn3: f64 = *var_qg_fp4s_dn3_slot;
        let mut var_qg_fp4s_dn4: f64 = *var_qg_fp4s_dn4_slot;
        let mut var_qg_fp4s_dn5: f64 = *var_qg_fp4s_dn5_slot;
        let mut var_qg_fp4s_dn6: f64 = *var_qg_fp4s_dn6_slot;
        let mut var_qg_fp4s_dn7: f64 = *var_qg_fp4s_dn7_slot;
        let mut var_qg_fp4s_dn8: f64 = *var_qg_fp4s_dn8_slot;
        let mut var_qg_fp4s_dn9: f64 = *var_qg_fp4s_dn9_slot;
        let mut var_qsov: f64 = *var_qsov_slot;
        let mut var_qsov_dn1: f64 = *var_qsov_dn1_slot;
        let mut var_qsov_dn10: f64 = *var_qsov_dn10_slot;
        let mut var_qsov_dn2: f64 = *var_qsov_dn2_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn1: f64 = *var_t0_dn1_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn14: f64 = *var_t0_dn14_slot;
        let mut var_t0_dn15: f64 = *var_t0_dn15_slot;
        let mut var_t0_dn16: f64 = *var_t0_dn16_slot;
        let mut var_t0_dn17: f64 = *var_t0_dn17_slot;
        let mut var_t0_dn18: f64 = *var_t0_dn18_slot;
        let mut var_t0_dn19: f64 = *var_t0_dn19_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn20: f64 = *var_t0_dn20_slot;
        let mut var_t0_dn21: f64 = *var_t0_dn21_slot;
        let mut var_t0_dn22: f64 = *var_t0_dn22_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn1: f64 = *var_t1_dn1_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn15: f64 = *var_t1_dn15_slot;
        let mut var_t1_dn16: f64 = *var_t1_dn16_slot;
        let mut var_t1_dn17: f64 = *var_t1_dn17_slot;
        let mut var_t1_dn18: f64 = *var_t1_dn18_slot;
        let mut var_t1_dn19: f64 = *var_t1_dn19_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn20: f64 = *var_t1_dn20_slot;
        let mut var_t1_dn21: f64 = *var_t1_dn21_slot;
        let mut var_t1_dn22: f64 = *var_t1_dn22_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn1: f64 = *var_t2_dn1_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn15: f64 = *var_t2_dn15_slot;
        let mut var_t2_dn16: f64 = *var_t2_dn16_slot;
        let mut var_t2_dn17: f64 = *var_t2_dn17_slot;
        let mut var_t2_dn18: f64 = *var_t2_dn18_slot;
        let mut var_t2_dn19: f64 = *var_t2_dn19_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn20: f64 = *var_t2_dn20_slot;
        let mut var_t2_dn21: f64 = *var_t2_dn21_slot;
        let mut var_t2_dn22: f64 = *var_t2_dn22_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn1: f64 = *var_t3_dn1_slot;
        let mut var_t3_dn12: f64 = *var_t3_dn12_slot;
        let mut var_t3_dn14: f64 = *var_t3_dn14_slot;
        let mut var_t3_dn15: f64 = *var_t3_dn15_slot;
        let mut var_t3_dn16: f64 = *var_t3_dn16_slot;
        let mut var_t3_dn17: f64 = *var_t3_dn17_slot;
        let mut var_t3_dn18: f64 = *var_t3_dn18_slot;
        let mut var_t3_dn19: f64 = *var_t3_dn19_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn20: f64 = *var_t3_dn20_slot;
        let mut var_t3_dn21: f64 = *var_t3_dn21_slot;
        let mut var_t3_dn22: f64 = *var_t3_dn22_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_vbidb_t: f64 = *var_vbidb_t_slot;
        let mut var_vbidb_t_dn4: f64 = *var_vbidb_t_dn4_slot;
        let mut var_vbisb_t: f64 = *var_vbisb_t_slot;
        let mut var_vbisb_t_dn4: f64 = *var_vbisb_t_dn4_slot;
        let mut var_vdseffcv: f64 = *var_vdseffcv_slot;
        let mut var_vdseffcv_dn0: f64 = *var_vdseffcv_dn0_slot;
        let mut var_vdseffcv_dn2: f64 = *var_vdseffcv_dn2_slot;

        let (assign30760_e48611, assign30760_e48611_d_n0, assign30760_e48611_d_n1, assign30760_e48611_d_n2, assign30760_e48611_d_n3, assign30760_e48611_d_n4, assign30760_e48611_d_n5, assign30760_e48611_d_n6, assign30760_e48611_d_n7, assign30760_e48611_d_n8, assign30760_e48611_d_n9, assign30760_e48611_d_n12, assign30760_e48611_d_n14, assign30760_e48611_d_n15, assign30760_e48611_d_n16, assign30760_e48611_d_n17, assign30760_e48611_d_n18, assign30760_e48611_d_n19, assign30760_e48611_d_n20, assign30760_e48611_d_n21, assign30760_e48611_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30760_e48608: f64 = (p.p199 + var_xdcinv);
        let assign30760_e48609: f64 = (p.p9 / assign30760_e48608);
        (assign30760_e48609, (-((p.p9 * var_xdcinv_dn0) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * var_xdcinv_dn1) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * var_xdcinv_dn2) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * var_xdcinv_dn3) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * var_xdcinv_dn4) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * var_xdcinv_dn5) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * var_xdcinv_dn6) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * var_xdcinv_dn7) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * var_xdcinv_dn8) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * var_xdcinv_dn9) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * var_xdcinv_dn12) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * var_xdcinv_dn14) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * var_xdcinv_dn15) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * var_xdcinv_dn16) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * var_xdcinv_dn17) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * var_xdcinv_dn18) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * var_xdcinv_dn19) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * var_xdcinv_dn20) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * var_xdcinv_dn21) / (assign30760_e48608 * assign30760_e48608))), (-((p.p9 * var_xdcinv_dn22) / (assign30760_e48608 * assign30760_e48608))),)
    } else {
        (var_cg_qme, var_cg_qme_dn0, var_cg_qme_dn1, var_cg_qme_dn2, var_cg_qme_dn3, var_cg_qme_dn4, var_cg_qme_dn5, var_cg_qme_dn6, var_cg_qme_dn7, var_cg_qme_dn8, var_cg_qme_dn9, var_cg_qme_dn12, var_cg_qme_dn14, var_cg_qme_dn15, var_cg_qme_dn16, var_cg_qme_dn17, var_cg_qme_dn18, var_cg_qme_dn19, var_cg_qme_dn20, var_cg_qme_dn21, var_cg_qme_dn22,)
    }
};
        var_cg_qme = assign30760_e48611;
        var_cg_qme_dn0 = assign30760_e48611_d_n0;
        var_cg_qme_dn1 = assign30760_e48611_d_n1;
        var_cg_qme_dn2 = assign30760_e48611_d_n2;
        var_cg_qme_dn3 = assign30760_e48611_d_n3;
        var_cg_qme_dn4 = assign30760_e48611_d_n4;
        var_cg_qme_dn5 = assign30760_e48611_d_n5;
        var_cg_qme_dn6 = assign30760_e48611_d_n6;
        var_cg_qme_dn7 = assign30760_e48611_d_n7;
        var_cg_qme_dn8 = assign30760_e48611_d_n8;
        var_cg_qme_dn9 = assign30760_e48611_d_n9;
        var_cg_qme_dn12 = assign30760_e48611_d_n12;
        var_cg_qme_dn14 = assign30760_e48611_d_n14;
        var_cg_qme_dn15 = assign30760_e48611_d_n15;
        var_cg_qme_dn16 = assign30760_e48611_d_n16;
        var_cg_qme_dn17 = assign30760_e48611_d_n17;
        var_cg_qme_dn18 = assign30760_e48611_d_n18;
        var_cg_qme_dn19 = assign30760_e48611_d_n19;
        var_cg_qme_dn20 = assign30760_e48611_d_n20;
        var_cg_qme_dn21 = assign30760_e48611_d_n21;
        var_cg_qme_dn22 = assign30760_e48611_d_n22;

        let (assign30770_e48638, assign30770_e48638_d_n0, assign30770_e48638_d_n1, assign30770_e48638_d_n2, assign30770_e48638_d_n3, assign30770_e48638_d_n4, assign30770_e48638_d_n5, assign30770_e48638_d_n6, assign30770_e48638_d_n7, assign30770_e48638_d_n8, assign30770_e48638_d_n9, assign30770_e48638_d_n12, assign30770_e48638_d_n14, assign30770_e48638_d_n15, assign30770_e48638_d_n16, assign30770_e48638_d_n17, assign30770_e48638_d_n18, assign30770_e48638_d_n19, assign30770_e48638_d_n20, assign30770_e48638_d_n21, assign30770_e48638_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30770_e48618: f64 = (var_cg_qme * p.p4);
        let assign30770_e48620: f64 = (assign30770_e48618 * p.p5);
        let assign30770_e48622: f64 = (assign30770_e48620 * p.p200);
        let assign30770_e48625: f64 = (var_vg0_fp4s - var_psim_fp4s);
        let assign30770_e48628: f64 = (0.5 * var_t1);
        let assign30770_e48630: f64 = (assign30770_e48628 * var_t1);
        let assign30770_e48633: f64 = (6.0 * var_t2);
        let assign30770_e48634: f64 = (assign30770_e48630 / assign30770_e48633);
        let assign30770_e48635: f64 = (assign30770_e48625 + assign30770_e48634);
        let assign30770_e48636: f64 = (assign30770_e48622 * assign30770_e48635);
        (assign30770_e48636, (((((var_cg_qme_dn0 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((var_vg0_fp4s_dn0 - var_psim_fp4s_dn0) + ((((((0.5 * var_t1_dn0) * var_t1) + (assign30770_e48628 * var_t1_dn0)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * var_t2_dn0))) / (assign30770_e48633 * assign30770_e48633))))), (((((var_cg_qme_dn1 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((var_vg0_fp4s_dn1 - var_psim_fp4s_dn1) + ((((((0.5 * var_t1_dn1) * var_t1) + (assign30770_e48628 * var_t1_dn1)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * var_t2_dn1))) / (assign30770_e48633 * assign30770_e48633))))), (((((var_cg_qme_dn2 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((var_vg0_fp4s_dn2 - var_psim_fp4s_dn2) + ((((((0.5 * var_t1_dn2) * var_t1) + (assign30770_e48628 * var_t1_dn2)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * var_t2_dn2))) / (assign30770_e48633 * assign30770_e48633))))), (((((var_cg_qme_dn3 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((var_vg0_fp4s_dn3 - var_psim_fp4s_dn3) + ((((((0.5 * var_t1_dn3) * var_t1) + (assign30770_e48628 * var_t1_dn3)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * var_t2_dn3))) / (assign30770_e48633 * assign30770_e48633))))), (((((var_cg_qme_dn4 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((var_vg0_fp4s_dn4 - var_psim_fp4s_dn4) + ((((((0.5 * var_t1_dn4) * var_t1) + (assign30770_e48628 * var_t1_dn4)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * var_t2_dn4))) / (assign30770_e48633 * assign30770_e48633))))), (((((var_cg_qme_dn5 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((var_vg0_fp4s_dn5 - var_psim_fp4s_dn5) + ((((((0.5 * var_t1_dn5) * var_t1) + (assign30770_e48628 * var_t1_dn5)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * var_t2_dn5))) / (assign30770_e48633 * assign30770_e48633))))), (((((var_cg_qme_dn6 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((var_vg0_fp4s_dn6 - var_psim_fp4s_dn6) + ((((((0.5 * var_t1_dn6) * var_t1) + (assign30770_e48628 * var_t1_dn6)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * var_t2_dn6))) / (assign30770_e48633 * assign30770_e48633))))), (((((var_cg_qme_dn7 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((var_vg0_fp4s_dn7 - var_psim_fp4s_dn7) + ((((((0.5 * var_t1_dn7) * var_t1) + (assign30770_e48628 * var_t1_dn7)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * var_t2_dn7))) / (assign30770_e48633 * assign30770_e48633))))), (((((var_cg_qme_dn8 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((var_vg0_fp4s_dn8 - var_psim_fp4s_dn8) + ((((((0.5 * var_t1_dn8) * var_t1) + (assign30770_e48628 * var_t1_dn8)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * var_t2_dn8))) / (assign30770_e48633 * assign30770_e48633))))), (((((var_cg_qme_dn9 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((var_vg0_fp4s_dn9 - var_psim_fp4s_dn9) + ((((((0.5 * var_t1_dn9) * var_t1) + (assign30770_e48628 * var_t1_dn9)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * var_t2_dn9))) / (assign30770_e48633 * assign30770_e48633))))), (((((var_cg_qme_dn12 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((var_vg0_fp4s_dn12 - var_psim_fp4s_dn12) + ((((((0.5 * var_t1_dn12) * var_t1) + (assign30770_e48628 * var_t1_dn12)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * var_t2_dn12))) / (assign30770_e48633 * assign30770_e48633))))), (((((var_cg_qme_dn14 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((var_vg0_fp4s_dn14 - var_psim_fp4s_dn14) + ((((((0.5 * var_t1_dn14) * var_t1) + (assign30770_e48628 * var_t1_dn14)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * var_t2_dn14))) / (assign30770_e48633 * assign30770_e48633))))), (((((var_cg_qme_dn15 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((var_vg0_fp4s_dn15 - var_psim_fp4s_dn15) + ((((((0.5 * var_t1_dn15) * var_t1) + (assign30770_e48628 * var_t1_dn15)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * var_t2_dn15))) / (assign30770_e48633 * assign30770_e48633))))), (((((var_cg_qme_dn16 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((var_vg0_fp4s_dn16 - var_psim_fp4s_dn16) + ((((((0.5 * var_t1_dn16) * var_t1) + (assign30770_e48628 * var_t1_dn16)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * var_t2_dn16))) / (assign30770_e48633 * assign30770_e48633))))), (((((var_cg_qme_dn17 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((var_vg0_fp4s_dn17 - var_psim_fp4s_dn17) + ((((((0.5 * var_t1_dn17) * var_t1) + (assign30770_e48628 * var_t1_dn17)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * var_t2_dn17))) / (assign30770_e48633 * assign30770_e48633))))), (((((var_cg_qme_dn18 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((var_vg0_fp4s_dn18 - var_psim_fp4s_dn18) + ((((((0.5 * var_t1_dn18) * var_t1) + (assign30770_e48628 * var_t1_dn18)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * var_t2_dn18))) / (assign30770_e48633 * assign30770_e48633))))), (((((var_cg_qme_dn19 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((var_vg0_fp4s_dn19 - var_psim_fp4s_dn19) + ((((((0.5 * var_t1_dn19) * var_t1) + (assign30770_e48628 * var_t1_dn19)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * var_t2_dn19))) / (assign30770_e48633 * assign30770_e48633))))), (((((var_cg_qme_dn20 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((var_vg0_fp4s_dn20 - var_psim_fp4s_dn20) + ((((((0.5 * var_t1_dn20) * var_t1) + (assign30770_e48628 * var_t1_dn20)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * var_t2_dn20))) / (assign30770_e48633 * assign30770_e48633))))), (((((var_cg_qme_dn21 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((var_vg0_fp4s_dn21 - var_psim_fp4s_dn21) + ((((((0.5 * var_t1_dn21) * var_t1) + (assign30770_e48628 * var_t1_dn21)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * var_t2_dn21))) / (assign30770_e48633 * assign30770_e48633))))), (((((var_cg_qme_dn22 * p.p4) * p.p5) * p.p200) * assign30770_e48635) + (assign30770_e48622 * ((var_vg0_fp4s_dn22 - var_psim_fp4s_dn22) + ((((((0.5 * var_t1_dn22) * var_t1) + (assign30770_e48628 * var_t1_dn22)) * assign30770_e48633) - (assign30770_e48630 * (6.0 * var_t2_dn22))) / (assign30770_e48633 * assign30770_e48633))))),)
    } else {
        (var_qg_fp4s, var_qg_fp4s_dn0, var_qg_fp4s_dn1, var_qg_fp4s_dn2, var_qg_fp4s_dn3, var_qg_fp4s_dn4, var_qg_fp4s_dn5, var_qg_fp4s_dn6, var_qg_fp4s_dn7, var_qg_fp4s_dn8, var_qg_fp4s_dn9, var_qg_fp4s_dn12, var_qg_fp4s_dn14, var_qg_fp4s_dn15, var_qg_fp4s_dn16, var_qg_fp4s_dn17, var_qg_fp4s_dn18, var_qg_fp4s_dn19, var_qg_fp4s_dn20, var_qg_fp4s_dn21, var_qg_fp4s_dn22,)
    }
};
        var_qg_fp4s = assign30770_e48638;
        var_qg_fp4s_dn0 = assign30770_e48638_d_n0;
        var_qg_fp4s_dn1 = assign30770_e48638_d_n1;
        var_qg_fp4s_dn2 = assign30770_e48638_d_n2;
        var_qg_fp4s_dn3 = assign30770_e48638_d_n3;
        var_qg_fp4s_dn4 = assign30770_e48638_d_n4;
        var_qg_fp4s_dn5 = assign30770_e48638_d_n5;
        var_qg_fp4s_dn6 = assign30770_e48638_d_n6;
        var_qg_fp4s_dn7 = assign30770_e48638_d_n7;
        var_qg_fp4s_dn8 = assign30770_e48638_d_n8;
        var_qg_fp4s_dn9 = assign30770_e48638_d_n9;
        var_qg_fp4s_dn12 = assign30770_e48638_d_n12;
        var_qg_fp4s_dn14 = assign30770_e48638_d_n14;
        var_qg_fp4s_dn15 = assign30770_e48638_d_n15;
        var_qg_fp4s_dn16 = assign30770_e48638_d_n16;
        var_qg_fp4s_dn17 = assign30770_e48638_d_n17;
        var_qg_fp4s_dn18 = assign30770_e48638_d_n18;
        var_qg_fp4s_dn19 = assign30770_e48638_d_n19;
        var_qg_fp4s_dn20 = assign30770_e48638_d_n20;
        var_qg_fp4s_dn21 = assign30770_e48638_d_n21;
        var_qg_fp4s_dn22 = assign30770_e48638_d_n22;

        let (assign30780_e48649, assign30780_e48649_d_n0, assign30780_e48649_d_n1, assign30780_e48649_d_n2, assign30780_e48649_d_n3, assign30780_e48649_d_n4, assign30780_e48649_d_n5, assign30780_e48649_d_n6, assign30780_e48649_d_n7, assign30780_e48649_d_n8, assign30780_e48649_d_n9, assign30780_e48649_d_n12, assign30780_e48649_d_n14, assign30780_e48649_d_n15, assign30780_e48649_d_n16, assign30780_e48649_d_n17, assign30780_e48649_d_n18, assign30780_e48649_d_n19, assign30780_e48649_d_n20, assign30780_e48649_d_n21, assign30780_e48649_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30780_e48645: f64 = (var_vg0_fp4s + var_vtv);
        let assign30780_e48647: f64 = (assign30780_e48645 - var_psim_fp4s);
        (assign30780_e48647, (var_vg0_fp4s_dn0 - var_psim_fp4s_dn0), (var_vg0_fp4s_dn1 - var_psim_fp4s_dn1), (var_vg0_fp4s_dn2 - var_psim_fp4s_dn2), (var_vg0_fp4s_dn3 - var_psim_fp4s_dn3), ((var_vg0_fp4s_dn4 + var_vtv_dn4) - var_psim_fp4s_dn4), (var_vg0_fp4s_dn5 - var_psim_fp4s_dn5), ((var_vg0_fp4s_dn6 + var_vtv_dn6) - var_psim_fp4s_dn6), ((var_vg0_fp4s_dn7 + var_vtv_dn7) - var_psim_fp4s_dn7), ((var_vg0_fp4s_dn8 + var_vtv_dn8) - var_psim_fp4s_dn8), (var_vg0_fp4s_dn9 - var_psim_fp4s_dn9), (var_vg0_fp4s_dn12 - var_psim_fp4s_dn12), (var_vg0_fp4s_dn14 - var_psim_fp4s_dn14), ((var_vg0_fp4s_dn15 + var_vtv_dn15) - var_psim_fp4s_dn15), ((var_vg0_fp4s_dn16 + var_vtv_dn16) - var_psim_fp4s_dn16), ((var_vg0_fp4s_dn17 + var_vtv_dn17) - var_psim_fp4s_dn17), ((var_vg0_fp4s_dn18 + var_vtv_dn18) - var_psim_fp4s_dn18), ((var_vg0_fp4s_dn19 + var_vtv_dn19) - var_psim_fp4s_dn19), ((var_vg0_fp4s_dn20 + var_vtv_dn20) - var_psim_fp4s_dn20), ((var_vg0_fp4s_dn21 + var_vtv_dn21) - var_psim_fp4s_dn21), ((var_vg0_fp4s_dn22 + var_vtv_dn22) - var_psim_fp4s_dn22),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn1, var_t0_dn2, var_t0_dn3, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn12, var_t0_dn14, var_t0_dn15, var_t0_dn16, var_t0_dn17, var_t0_dn18, var_t0_dn19, var_t0_dn20, var_t0_dn21, var_t0_dn22,)
    }
};
        var_t0 = assign30780_e48649;
        var_t0_dn0 = assign30780_e48649_d_n0;
        var_t0_dn1 = assign30780_e48649_d_n1;
        var_t0_dn2 = assign30780_e48649_d_n2;
        var_t0_dn3 = assign30780_e48649_d_n3;
        var_t0_dn4 = assign30780_e48649_d_n4;
        var_t0_dn5 = assign30780_e48649_d_n5;
        var_t0_dn6 = assign30780_e48649_d_n6;
        var_t0_dn7 = assign30780_e48649_d_n7;
        var_t0_dn8 = assign30780_e48649_d_n8;
        var_t0_dn9 = assign30780_e48649_d_n9;
        var_t0_dn12 = assign30780_e48649_d_n12;
        var_t0_dn14 = assign30780_e48649_d_n14;
        var_t0_dn15 = assign30780_e48649_d_n15;
        var_t0_dn16 = assign30780_e48649_d_n16;
        var_t0_dn17 = assign30780_e48649_d_n17;
        var_t0_dn18 = assign30780_e48649_d_n18;
        var_t0_dn19 = assign30780_e48649_d_n19;
        var_t0_dn20 = assign30780_e48649_d_n20;
        var_t0_dn21 = assign30780_e48649_d_n21;
        var_t0_dn22 = assign30780_e48649_d_n22;

        let (assign30790_e48662, assign30790_e48662_d_n0, assign30790_e48662_d_n1, assign30790_e48662_d_n2, assign30790_e48662_d_n3, assign30790_e48662_d_n4, assign30790_e48662_d_n5, assign30790_e48662_d_n6, assign30790_e48662_d_n7, assign30790_e48662_d_n8, assign30790_e48662_d_n9, assign30790_e48662_d_n12, assign30790_e48662_d_n14, assign30790_e48662_d_n15, assign30790_e48662_d_n16, assign30790_e48662_d_n17, assign30790_e48662_d_n18, assign30790_e48662_d_n19, assign30790_e48662_d_n20, assign30790_e48662_d_n21, assign30790_e48662_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30790_e48657: f64 = (2.0 * var_psid_fp4s);
        let assign30790_e48658: f64 = (var_psis_fp4s + assign30790_e48657);
        let assign30790_e48660: f64 = (assign30790_e48658 / 3.0);
        (assign30790_e48660, ((var_psis_fp4s_dn0 + (2.0 * var_psid_fp4s_dn0)) / 3.0), ((var_psis_fp4s_dn1 + (2.0 * var_psid_fp4s_dn1)) / 3.0), ((var_psis_fp4s_dn2 + (2.0 * var_psid_fp4s_dn2)) / 3.0), ((var_psis_fp4s_dn3 + (2.0 * var_psid_fp4s_dn3)) / 3.0), ((var_psis_fp4s_dn4 + (2.0 * var_psid_fp4s_dn4)) / 3.0), ((var_psis_fp4s_dn5 + (2.0 * var_psid_fp4s_dn5)) / 3.0), ((var_psis_fp4s_dn6 + (2.0 * var_psid_fp4s_dn6)) / 3.0), ((var_psis_fp4s_dn7 + (2.0 * var_psid_fp4s_dn7)) / 3.0), ((var_psis_fp4s_dn8 + (2.0 * var_psid_fp4s_dn8)) / 3.0), ((var_psis_fp4s_dn9 + (2.0 * var_psid_fp4s_dn9)) / 3.0), ((var_psis_fp4s_dn12 + (2.0 * var_psid_fp4s_dn12)) / 3.0), ((var_psis_fp4s_dn14 + (2.0 * var_psid_fp4s_dn14)) / 3.0), ((var_psis_fp4s_dn15 + (2.0 * var_psid_fp4s_dn15)) / 3.0), ((var_psis_fp4s_dn16 + (2.0 * var_psid_fp4s_dn16)) / 3.0), ((var_psis_fp4s_dn17 + (2.0 * var_psid_fp4s_dn17)) / 3.0), ((var_psis_fp4s_dn18 + (2.0 * var_psid_fp4s_dn18)) / 3.0), ((var_psis_fp4s_dn19 + (2.0 * var_psid_fp4s_dn19)) / 3.0), ((var_psis_fp4s_dn20 + (2.0 * var_psid_fp4s_dn20)) / 3.0), ((var_psis_fp4s_dn21 + (2.0 * var_psid_fp4s_dn21)) / 3.0), ((var_psis_fp4s_dn22 + (2.0 * var_psid_fp4s_dn22)) / 3.0),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn1, var_t1_dn2, var_t1_dn3, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn12, var_t1_dn14, var_t1_dn15, var_t1_dn16, var_t1_dn17, var_t1_dn18, var_t1_dn19, var_t1_dn20, var_t1_dn21, var_t1_dn22,)
    }
};
        var_t1 = assign30790_e48662;
        var_t1_dn0 = assign30790_e48662_d_n0;
        var_t1_dn1 = assign30790_e48662_d_n1;
        var_t1_dn2 = assign30790_e48662_d_n2;
        var_t1_dn3 = assign30790_e48662_d_n3;
        var_t1_dn4 = assign30790_e48662_d_n4;
        var_t1_dn5 = assign30790_e48662_d_n5;
        var_t1_dn6 = assign30790_e48662_d_n6;
        var_t1_dn7 = assign30790_e48662_d_n7;
        var_t1_dn8 = assign30790_e48662_d_n8;
        var_t1_dn9 = assign30790_e48662_d_n9;
        var_t1_dn12 = assign30790_e48662_d_n12;
        var_t1_dn14 = assign30790_e48662_d_n14;
        var_t1_dn15 = assign30790_e48662_d_n15;
        var_t1_dn16 = assign30790_e48662_d_n16;
        var_t1_dn17 = assign30790_e48662_d_n17;
        var_t1_dn18 = assign30790_e48662_d_n18;
        var_t1_dn19 = assign30790_e48662_d_n19;
        var_t1_dn20 = assign30790_e48662_d_n20;
        var_t1_dn21 = assign30790_e48662_d_n21;
        var_t1_dn22 = assign30790_e48662_d_n22;

        let (assign30800_e48677, assign30800_e48677_d_n0, assign30800_e48677_d_n1, assign30800_e48677_d_n2, assign30800_e48677_d_n3, assign30800_e48677_d_n4, assign30800_e48677_d_n5, assign30800_e48677_d_n6, assign30800_e48677_d_n7, assign30800_e48677_d_n8, assign30800_e48677_d_n9, assign30800_e48677_d_n12, assign30800_e48677_d_n14, assign30800_e48677_d_n15, assign30800_e48677_d_n16, assign30800_e48677_d_n17, assign30800_e48677_d_n18, assign30800_e48677_d_n19, assign30800_e48677_d_n20, assign30800_e48677_d_n21, assign30800_e48677_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30800_e48669: f64 = (1.0 / 12.0);
        let assign30800_e48672: f64 = (var_psisd_fp4s * var_psisd_fp4s);
        let assign30800_e48673: f64 = (assign30800_e48669 * assign30800_e48672);
        let assign30800_e48675: f64 = (assign30800_e48673 / var_t0);
        (assign30800_e48675, ((((assign30800_e48669 * ((var_psisd_fp4s_dn0 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn0))) * var_t0) - (assign30800_e48673 * var_t0_dn0)) / (var_t0 * var_t0)), ((((assign30800_e48669 * ((var_psisd_fp4s_dn1 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn1))) * var_t0) - (assign30800_e48673 * var_t0_dn1)) / (var_t0 * var_t0)), ((((assign30800_e48669 * ((var_psisd_fp4s_dn2 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn2))) * var_t0) - (assign30800_e48673 * var_t0_dn2)) / (var_t0 * var_t0)), ((((assign30800_e48669 * ((var_psisd_fp4s_dn3 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn3))) * var_t0) - (assign30800_e48673 * var_t0_dn3)) / (var_t0 * var_t0)), ((((assign30800_e48669 * ((var_psisd_fp4s_dn4 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn4))) * var_t0) - (assign30800_e48673 * var_t0_dn4)) / (var_t0 * var_t0)), ((((assign30800_e48669 * ((var_psisd_fp4s_dn5 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn5))) * var_t0) - (assign30800_e48673 * var_t0_dn5)) / (var_t0 * var_t0)), ((((assign30800_e48669 * ((var_psisd_fp4s_dn6 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn6))) * var_t0) - (assign30800_e48673 * var_t0_dn6)) / (var_t0 * var_t0)), ((((assign30800_e48669 * ((var_psisd_fp4s_dn7 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn7))) * var_t0) - (assign30800_e48673 * var_t0_dn7)) / (var_t0 * var_t0)), ((((assign30800_e48669 * ((var_psisd_fp4s_dn8 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn8))) * var_t0) - (assign30800_e48673 * var_t0_dn8)) / (var_t0 * var_t0)), ((((assign30800_e48669 * ((var_psisd_fp4s_dn9 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn9))) * var_t0) - (assign30800_e48673 * var_t0_dn9)) / (var_t0 * var_t0)), ((((assign30800_e48669 * ((var_psisd_fp4s_dn12 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn12))) * var_t0) - (assign30800_e48673 * var_t0_dn12)) / (var_t0 * var_t0)), ((((assign30800_e48669 * ((var_psisd_fp4s_dn14 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn14))) * var_t0) - (assign30800_e48673 * var_t0_dn14)) / (var_t0 * var_t0)), ((((assign30800_e48669 * ((var_psisd_fp4s_dn15 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn15))) * var_t0) - (assign30800_e48673 * var_t0_dn15)) / (var_t0 * var_t0)), ((((assign30800_e48669 * ((var_psisd_fp4s_dn16 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn16))) * var_t0) - (assign30800_e48673 * var_t0_dn16)) / (var_t0 * var_t0)), ((((assign30800_e48669 * ((var_psisd_fp4s_dn17 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn17))) * var_t0) - (assign30800_e48673 * var_t0_dn17)) / (var_t0 * var_t0)), ((((assign30800_e48669 * ((var_psisd_fp4s_dn18 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn18))) * var_t0) - (assign30800_e48673 * var_t0_dn18)) / (var_t0 * var_t0)), ((((assign30800_e48669 * ((var_psisd_fp4s_dn19 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn19))) * var_t0) - (assign30800_e48673 * var_t0_dn19)) / (var_t0 * var_t0)), ((((assign30800_e48669 * ((var_psisd_fp4s_dn20 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn20))) * var_t0) - (assign30800_e48673 * var_t0_dn20)) / (var_t0 * var_t0)), ((((assign30800_e48669 * ((var_psisd_fp4s_dn21 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn21))) * var_t0) - (assign30800_e48673 * var_t0_dn21)) / (var_t0 * var_t0)), ((((assign30800_e48669 * ((var_psisd_fp4s_dn22 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn22))) * var_t0) - (assign30800_e48673 * var_t0_dn22)) / (var_t0 * var_t0)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn1, var_t2_dn2, var_t2_dn3, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn12, var_t2_dn14, var_t2_dn15, var_t2_dn16, var_t2_dn17, var_t2_dn18, var_t2_dn19, var_t2_dn20, var_t2_dn21, var_t2_dn22,)
    }
};
        var_t2 = assign30800_e48677;
        var_t2_dn0 = assign30800_e48677_d_n0;
        var_t2_dn1 = assign30800_e48677_d_n1;
        var_t2_dn2 = assign30800_e48677_d_n2;
        var_t2_dn3 = assign30800_e48677_d_n3;
        var_t2_dn4 = assign30800_e48677_d_n4;
        var_t2_dn5 = assign30800_e48677_d_n5;
        var_t2_dn6 = assign30800_e48677_d_n6;
        var_t2_dn7 = assign30800_e48677_d_n7;
        var_t2_dn8 = assign30800_e48677_d_n8;
        var_t2_dn9 = assign30800_e48677_d_n9;
        var_t2_dn12 = assign30800_e48677_d_n12;
        var_t2_dn14 = assign30800_e48677_d_n14;
        var_t2_dn15 = assign30800_e48677_d_n15;
        var_t2_dn16 = assign30800_e48677_d_n16;
        var_t2_dn17 = assign30800_e48677_d_n17;
        var_t2_dn18 = assign30800_e48677_d_n18;
        var_t2_dn19 = assign30800_e48677_d_n19;
        var_t2_dn20 = assign30800_e48677_d_n20;
        var_t2_dn21 = assign30800_e48677_d_n21;
        var_t2_dn22 = assign30800_e48677_d_n22;

        let (assign30810_e48696, assign30810_e48696_d_n0, assign30810_e48696_d_n1, assign30810_e48696_d_n2, assign30810_e48696_d_n3, assign30810_e48696_d_n4, assign30810_e48696_d_n5, assign30810_e48696_d_n6, assign30810_e48696_d_n7, assign30810_e48696_d_n8, assign30810_e48696_d_n9, assign30810_e48696_d_n12, assign30810_e48696_d_n14, assign30810_e48696_d_n15, assign30810_e48696_d_n16, assign30810_e48696_d_n17, assign30810_e48696_d_n18, assign30810_e48696_d_n19, assign30810_e48696_d_n20, assign30810_e48696_d_n21, assign30810_e48696_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30810_e48684: f64 = (1.0 / 120.0);
        let assign30810_e48687: f64 = (var_psisd_fp4s * var_psisd_fp4s);
        let assign30810_e48689: f64 = (assign30810_e48687 * var_psisd_fp4s);
        let assign30810_e48690: f64 = (assign30810_e48684 * assign30810_e48689);
        let assign30810_e48693: f64 = (var_t0 * var_t0);
        let assign30810_e48694: f64 = (assign30810_e48690 / assign30810_e48693);
        (assign30810_e48694, ((((assign30810_e48684 * ((((var_psisd_fp4s_dn0 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn0)) * var_psisd_fp4s) + (assign30810_e48687 * var_psisd_fp4s_dn0))) * assign30810_e48693) - (assign30810_e48690 * ((var_t0_dn0 * var_t0) + (var_t0 * var_t0_dn0)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((var_psisd_fp4s_dn1 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn1)) * var_psisd_fp4s) + (assign30810_e48687 * var_psisd_fp4s_dn1))) * assign30810_e48693) - (assign30810_e48690 * ((var_t0_dn1 * var_t0) + (var_t0 * var_t0_dn1)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((var_psisd_fp4s_dn2 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn2)) * var_psisd_fp4s) + (assign30810_e48687 * var_psisd_fp4s_dn2))) * assign30810_e48693) - (assign30810_e48690 * ((var_t0_dn2 * var_t0) + (var_t0 * var_t0_dn2)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((var_psisd_fp4s_dn3 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn3)) * var_psisd_fp4s) + (assign30810_e48687 * var_psisd_fp4s_dn3))) * assign30810_e48693) - (assign30810_e48690 * ((var_t0_dn3 * var_t0) + (var_t0 * var_t0_dn3)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((var_psisd_fp4s_dn4 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn4)) * var_psisd_fp4s) + (assign30810_e48687 * var_psisd_fp4s_dn4))) * assign30810_e48693) - (assign30810_e48690 * ((var_t0_dn4 * var_t0) + (var_t0 * var_t0_dn4)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((var_psisd_fp4s_dn5 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn5)) * var_psisd_fp4s) + (assign30810_e48687 * var_psisd_fp4s_dn5))) * assign30810_e48693) - (assign30810_e48690 * ((var_t0_dn5 * var_t0) + (var_t0 * var_t0_dn5)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((var_psisd_fp4s_dn6 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn6)) * var_psisd_fp4s) + (assign30810_e48687 * var_psisd_fp4s_dn6))) * assign30810_e48693) - (assign30810_e48690 * ((var_t0_dn6 * var_t0) + (var_t0 * var_t0_dn6)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((var_psisd_fp4s_dn7 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn7)) * var_psisd_fp4s) + (assign30810_e48687 * var_psisd_fp4s_dn7))) * assign30810_e48693) - (assign30810_e48690 * ((var_t0_dn7 * var_t0) + (var_t0 * var_t0_dn7)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((var_psisd_fp4s_dn8 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn8)) * var_psisd_fp4s) + (assign30810_e48687 * var_psisd_fp4s_dn8))) * assign30810_e48693) - (assign30810_e48690 * ((var_t0_dn8 * var_t0) + (var_t0 * var_t0_dn8)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((var_psisd_fp4s_dn9 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn9)) * var_psisd_fp4s) + (assign30810_e48687 * var_psisd_fp4s_dn9))) * assign30810_e48693) - (assign30810_e48690 * ((var_t0_dn9 * var_t0) + (var_t0 * var_t0_dn9)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((var_psisd_fp4s_dn12 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn12)) * var_psisd_fp4s) + (assign30810_e48687 * var_psisd_fp4s_dn12))) * assign30810_e48693) - (assign30810_e48690 * ((var_t0_dn12 * var_t0) + (var_t0 * var_t0_dn12)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((var_psisd_fp4s_dn14 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn14)) * var_psisd_fp4s) + (assign30810_e48687 * var_psisd_fp4s_dn14))) * assign30810_e48693) - (assign30810_e48690 * ((var_t0_dn14 * var_t0) + (var_t0 * var_t0_dn14)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((var_psisd_fp4s_dn15 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn15)) * var_psisd_fp4s) + (assign30810_e48687 * var_psisd_fp4s_dn15))) * assign30810_e48693) - (assign30810_e48690 * ((var_t0_dn15 * var_t0) + (var_t0 * var_t0_dn15)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((var_psisd_fp4s_dn16 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn16)) * var_psisd_fp4s) + (assign30810_e48687 * var_psisd_fp4s_dn16))) * assign30810_e48693) - (assign30810_e48690 * ((var_t0_dn16 * var_t0) + (var_t0 * var_t0_dn16)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((var_psisd_fp4s_dn17 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn17)) * var_psisd_fp4s) + (assign30810_e48687 * var_psisd_fp4s_dn17))) * assign30810_e48693) - (assign30810_e48690 * ((var_t0_dn17 * var_t0) + (var_t0 * var_t0_dn17)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((var_psisd_fp4s_dn18 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn18)) * var_psisd_fp4s) + (assign30810_e48687 * var_psisd_fp4s_dn18))) * assign30810_e48693) - (assign30810_e48690 * ((var_t0_dn18 * var_t0) + (var_t0 * var_t0_dn18)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((var_psisd_fp4s_dn19 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn19)) * var_psisd_fp4s) + (assign30810_e48687 * var_psisd_fp4s_dn19))) * assign30810_e48693) - (assign30810_e48690 * ((var_t0_dn19 * var_t0) + (var_t0 * var_t0_dn19)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((var_psisd_fp4s_dn20 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn20)) * var_psisd_fp4s) + (assign30810_e48687 * var_psisd_fp4s_dn20))) * assign30810_e48693) - (assign30810_e48690 * ((var_t0_dn20 * var_t0) + (var_t0 * var_t0_dn20)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((var_psisd_fp4s_dn21 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn21)) * var_psisd_fp4s) + (assign30810_e48687 * var_psisd_fp4s_dn21))) * assign30810_e48693) - (assign30810_e48690 * ((var_t0_dn21 * var_t0) + (var_t0 * var_t0_dn21)))) / (assign30810_e48693 * assign30810_e48693)), ((((assign30810_e48684 * ((((var_psisd_fp4s_dn22 * var_psisd_fp4s) + (var_psisd_fp4s * var_psisd_fp4s_dn22)) * var_psisd_fp4s) + (assign30810_e48687 * var_psisd_fp4s_dn22))) * assign30810_e48693) - (assign30810_e48690 * ((var_t0_dn22 * var_t0) + (var_t0 * var_t0_dn22)))) / (assign30810_e48693 * assign30810_e48693)),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn1, var_t3_dn2, var_t3_dn3, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn12, var_t3_dn14, var_t3_dn15, var_t3_dn16, var_t3_dn17, var_t3_dn18, var_t3_dn19, var_t3_dn20, var_t3_dn21, var_t3_dn22,)
    }
};
        var_t3 = assign30810_e48696;
        var_t3_dn0 = assign30810_e48696_d_n0;
        var_t3_dn1 = assign30810_e48696_d_n1;
        var_t3_dn2 = assign30810_e48696_d_n2;
        var_t3_dn3 = assign30810_e48696_d_n3;
        var_t3_dn4 = assign30810_e48696_d_n4;
        var_t3_dn5 = assign30810_e48696_d_n5;
        var_t3_dn6 = assign30810_e48696_d_n6;
        var_t3_dn7 = assign30810_e48696_d_n7;
        var_t3_dn8 = assign30810_e48696_d_n8;
        var_t3_dn9 = assign30810_e48696_d_n9;
        var_t3_dn12 = assign30810_e48696_d_n12;
        var_t3_dn14 = assign30810_e48696_d_n14;
        var_t3_dn15 = assign30810_e48696_d_n15;
        var_t3_dn16 = assign30810_e48696_d_n16;
        var_t3_dn17 = assign30810_e48696_d_n17;
        var_t3_dn18 = assign30810_e48696_d_n18;
        var_t3_dn19 = assign30810_e48696_d_n19;
        var_t3_dn20 = assign30810_e48696_d_n20;
        var_t3_dn21 = assign30810_e48696_d_n21;
        var_t3_dn22 = assign30810_e48696_d_n22;

        let (assign30820_e48720, assign30820_e48720_d_n0, assign30820_e48720_d_n1, assign30820_e48720_d_n2, assign30820_e48720_d_n3, assign30820_e48720_d_n4, assign30820_e48720_d_n5, assign30820_e48720_d_n6, assign30820_e48720_d_n7, assign30820_e48720_d_n8, assign30820_e48720_d_n9, assign30820_e48720_d_n12, assign30820_e48720_d_n14, assign30820_e48720_d_n15, assign30820_e48720_d_n16, assign30820_e48720_d_n17, assign30820_e48720_d_n18, assign30820_e48720_d_n19, assign30820_e48720_d_n20, assign30820_e48720_d_n21, assign30820_e48720_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 != 0.0)) {
        let assign30820_e48703: f64 = (var_cg_qme * p.p4);
        let assign30820_e48705: f64 = (assign30820_e48703 * p.p200);
        let assign30820_e48707: f64 = (assign30820_e48705 * p.p5);
        let assign30820_e48709: f64 = (assign30820_e48707 * 0.5);
        let assign30820_e48710: f64 = (-assign30820_e48709);
        let assign30820_e48713: f64 = (var_vg0_fp4s - var_t1);
        let assign30820_e48715: f64 = (assign30820_e48713 + var_t2);
        let assign30820_e48717: f64 = (assign30820_e48715 + var_t3);
        let assign30820_e48718: f64 = (assign30820_e48710 * assign30820_e48717);
        (assign30820_e48718, (((-((((var_cg_qme_dn0 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((var_vg0_fp4s_dn0 - var_t1_dn0) + var_t2_dn0) + var_t3_dn0))), (((-((((var_cg_qme_dn1 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((var_vg0_fp4s_dn1 - var_t1_dn1) + var_t2_dn1) + var_t3_dn1))), (((-((((var_cg_qme_dn2 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((var_vg0_fp4s_dn2 - var_t1_dn2) + var_t2_dn2) + var_t3_dn2))), (((-((((var_cg_qme_dn3 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((var_vg0_fp4s_dn3 - var_t1_dn3) + var_t2_dn3) + var_t3_dn3))), (((-((((var_cg_qme_dn4 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((var_vg0_fp4s_dn4 - var_t1_dn4) + var_t2_dn4) + var_t3_dn4))), (((-((((var_cg_qme_dn5 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((var_vg0_fp4s_dn5 - var_t1_dn5) + var_t2_dn5) + var_t3_dn5))), (((-((((var_cg_qme_dn6 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((var_vg0_fp4s_dn6 - var_t1_dn6) + var_t2_dn6) + var_t3_dn6))), (((-((((var_cg_qme_dn7 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((var_vg0_fp4s_dn7 - var_t1_dn7) + var_t2_dn7) + var_t3_dn7))), (((-((((var_cg_qme_dn8 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((var_vg0_fp4s_dn8 - var_t1_dn8) + var_t2_dn8) + var_t3_dn8))), (((-((((var_cg_qme_dn9 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((var_vg0_fp4s_dn9 - var_t1_dn9) + var_t2_dn9) + var_t3_dn9))), (((-((((var_cg_qme_dn12 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((var_vg0_fp4s_dn12 - var_t1_dn12) + var_t2_dn12) + var_t3_dn12))), (((-((((var_cg_qme_dn14 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((var_vg0_fp4s_dn14 - var_t1_dn14) + var_t2_dn14) + var_t3_dn14))), (((-((((var_cg_qme_dn15 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((var_vg0_fp4s_dn15 - var_t1_dn15) + var_t2_dn15) + var_t3_dn15))), (((-((((var_cg_qme_dn16 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((var_vg0_fp4s_dn16 - var_t1_dn16) + var_t2_dn16) + var_t3_dn16))), (((-((((var_cg_qme_dn17 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((var_vg0_fp4s_dn17 - var_t1_dn17) + var_t2_dn17) + var_t3_dn17))), (((-((((var_cg_qme_dn18 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((var_vg0_fp4s_dn18 - var_t1_dn18) + var_t2_dn18) + var_t3_dn18))), (((-((((var_cg_qme_dn19 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((var_vg0_fp4s_dn19 - var_t1_dn19) + var_t2_dn19) + var_t3_dn19))), (((-((((var_cg_qme_dn20 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((var_vg0_fp4s_dn20 - var_t1_dn20) + var_t2_dn20) + var_t3_dn20))), (((-((((var_cg_qme_dn21 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((var_vg0_fp4s_dn21 - var_t1_dn21) + var_t2_dn21) + var_t3_dn21))), (((-((((var_cg_qme_dn22 * p.p4) * p.p200) * p.p5) * 0.5)) * assign30820_e48717) + (assign30820_e48710 * (((var_vg0_fp4s_dn22 - var_t1_dn22) + var_t2_dn22) + var_t3_dn22))),)
    } else {
        (var_qd_fp4s, var_qd_fp4s_dn0, var_qd_fp4s_dn1, var_qd_fp4s_dn2, var_qd_fp4s_dn3, var_qd_fp4s_dn4, var_qd_fp4s_dn5, var_qd_fp4s_dn6, var_qd_fp4s_dn7, var_qd_fp4s_dn8, var_qd_fp4s_dn9, var_qd_fp4s_dn12, var_qd_fp4s_dn14, var_qd_fp4s_dn15, var_qd_fp4s_dn16, var_qd_fp4s_dn17, var_qd_fp4s_dn18, var_qd_fp4s_dn19, var_qd_fp4s_dn20, var_qd_fp4s_dn21, var_qd_fp4s_dn22,)
    }
};
        var_qd_fp4s = assign30820_e48720;
        var_qd_fp4s_dn0 = assign30820_e48720_d_n0;
        var_qd_fp4s_dn1 = assign30820_e48720_d_n1;
        var_qd_fp4s_dn2 = assign30820_e48720_d_n2;
        var_qd_fp4s_dn3 = assign30820_e48720_d_n3;
        var_qd_fp4s_dn4 = assign30820_e48720_d_n4;
        var_qd_fp4s_dn5 = assign30820_e48720_d_n5;
        var_qd_fp4s_dn6 = assign30820_e48720_d_n6;
        var_qd_fp4s_dn7 = assign30820_e48720_d_n7;
        var_qd_fp4s_dn8 = assign30820_e48720_d_n8;
        var_qd_fp4s_dn9 = assign30820_e48720_d_n9;
        var_qd_fp4s_dn12 = assign30820_e48720_d_n12;
        var_qd_fp4s_dn14 = assign30820_e48720_d_n14;
        var_qd_fp4s_dn15 = assign30820_e48720_d_n15;
        var_qd_fp4s_dn16 = assign30820_e48720_d_n16;
        var_qd_fp4s_dn17 = assign30820_e48720_d_n17;
        var_qd_fp4s_dn18 = assign30820_e48720_d_n18;
        var_qd_fp4s_dn19 = assign30820_e48720_d_n19;
        var_qd_fp4s_dn20 = assign30820_e48720_d_n20;
        var_qd_fp4s_dn21 = assign30820_e48720_d_n21;
        var_qd_fp4s_dn22 = assign30820_e48720_d_n22;

        let (assign30830_e48728, assign30830_e48728_d_n0, assign30830_e48728_d_n1, assign30830_e48728_d_n2, assign30830_e48728_d_n3, assign30830_e48728_d_n4, assign30830_e48728_d_n5, assign30830_e48728_d_n6, assign30830_e48728_d_n7, assign30830_e48728_d_n8, assign30830_e48728_d_n9, assign30830_e48728_d_n12, assign30830_e48728_d_n14, assign30830_e48728_d_n15, assign30830_e48728_d_n16, assign30830_e48728_d_n17, assign30830_e48728_d_n18, assign30830_e48728_d_n19, assign30830_e48728_d_n20, assign30830_e48728_d_n21, assign30830_e48728_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qg_fp4s, var_qg_fp4s_dn0, var_qg_fp4s_dn1, var_qg_fp4s_dn2, var_qg_fp4s_dn3, var_qg_fp4s_dn4, var_qg_fp4s_dn5, var_qg_fp4s_dn6, var_qg_fp4s_dn7, var_qg_fp4s_dn8, var_qg_fp4s_dn9, var_qg_fp4s_dn12, var_qg_fp4s_dn14, var_qg_fp4s_dn15, var_qg_fp4s_dn16, var_qg_fp4s_dn17, var_qg_fp4s_dn18, var_qg_fp4s_dn19, var_qg_fp4s_dn20, var_qg_fp4s_dn21, var_qg_fp4s_dn22,)
    }
};
        var_qg_fp4s = assign30830_e48728;
        var_qg_fp4s_dn0 = assign30830_e48728_d_n0;
        var_qg_fp4s_dn1 = assign30830_e48728_d_n1;
        var_qg_fp4s_dn2 = assign30830_e48728_d_n2;
        var_qg_fp4s_dn3 = assign30830_e48728_d_n3;
        var_qg_fp4s_dn4 = assign30830_e48728_d_n4;
        var_qg_fp4s_dn5 = assign30830_e48728_d_n5;
        var_qg_fp4s_dn6 = assign30830_e48728_d_n6;
        var_qg_fp4s_dn7 = assign30830_e48728_d_n7;
        var_qg_fp4s_dn8 = assign30830_e48728_d_n8;
        var_qg_fp4s_dn9 = assign30830_e48728_d_n9;
        var_qg_fp4s_dn12 = assign30830_e48728_d_n12;
        var_qg_fp4s_dn14 = assign30830_e48728_d_n14;
        var_qg_fp4s_dn15 = assign30830_e48728_d_n15;
        var_qg_fp4s_dn16 = assign30830_e48728_d_n16;
        var_qg_fp4s_dn17 = assign30830_e48728_d_n17;
        var_qg_fp4s_dn18 = assign30830_e48728_d_n18;
        var_qg_fp4s_dn19 = assign30830_e48728_d_n19;
        var_qg_fp4s_dn20 = assign30830_e48728_d_n20;
        var_qg_fp4s_dn21 = assign30830_e48728_d_n21;
        var_qg_fp4s_dn22 = assign30830_e48728_d_n22;

        let (assign30840_e48736, assign30840_e48736_d_n0, assign30840_e48736_d_n1, assign30840_e48736_d_n2, assign30840_e48736_d_n3, assign30840_e48736_d_n4, assign30840_e48736_d_n5, assign30840_e48736_d_n6, assign30840_e48736_d_n7, assign30840_e48736_d_n8, assign30840_e48736_d_n9, assign30840_e48736_d_n12, assign30840_e48736_d_n14, assign30840_e48736_d_n15, assign30840_e48736_d_n16, assign30840_e48736_d_n17, assign30840_e48736_d_n18, assign30840_e48736_d_n19, assign30840_e48736_d_n20, assign30840_e48736_d_n21, assign30840_e48736_d_n22,) = {
    if ((var_guard504 == 0.0) && (var_guard513 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qd_fp4s, var_qd_fp4s_dn0, var_qd_fp4s_dn1, var_qd_fp4s_dn2, var_qd_fp4s_dn3, var_qd_fp4s_dn4, var_qd_fp4s_dn5, var_qd_fp4s_dn6, var_qd_fp4s_dn7, var_qd_fp4s_dn8, var_qd_fp4s_dn9, var_qd_fp4s_dn12, var_qd_fp4s_dn14, var_qd_fp4s_dn15, var_qd_fp4s_dn16, var_qd_fp4s_dn17, var_qd_fp4s_dn18, var_qd_fp4s_dn19, var_qd_fp4s_dn20, var_qd_fp4s_dn21, var_qd_fp4s_dn22,)
    }
};
        var_qd_fp4s = assign30840_e48736;
        var_qd_fp4s_dn0 = assign30840_e48736_d_n0;
        var_qd_fp4s_dn1 = assign30840_e48736_d_n1;
        var_qd_fp4s_dn2 = assign30840_e48736_d_n2;
        var_qd_fp4s_dn3 = assign30840_e48736_d_n3;
        var_qd_fp4s_dn4 = assign30840_e48736_d_n4;
        var_qd_fp4s_dn5 = assign30840_e48736_d_n5;
        var_qd_fp4s_dn6 = assign30840_e48736_d_n6;
        var_qd_fp4s_dn7 = assign30840_e48736_d_n7;
        var_qd_fp4s_dn8 = assign30840_e48736_d_n8;
        var_qd_fp4s_dn9 = assign30840_e48736_d_n9;
        var_qd_fp4s_dn12 = assign30840_e48736_d_n12;
        var_qd_fp4s_dn14 = assign30840_e48736_d_n14;
        var_qd_fp4s_dn15 = assign30840_e48736_d_n15;
        var_qd_fp4s_dn16 = assign30840_e48736_d_n16;
        var_qd_fp4s_dn17 = assign30840_e48736_d_n17;
        var_qd_fp4s_dn18 = assign30840_e48736_d_n18;
        var_qd_fp4s_dn19 = assign30840_e48736_d_n19;
        var_qd_fp4s_dn20 = assign30840_e48736_d_n20;
        var_qd_fp4s_dn21 = assign30840_e48736_d_n21;
        var_qd_fp4s_dn22 = assign30840_e48736_d_n22;

        let assign30990_e48883: f64 = if p.p255 == 2.0 { 1.0 } else { 0.0 };
        var_guard524 = assign30990_e48883;

        let (assign31000_e48893, assign31000_e48893_d_n1, assign31000_e48893_d_n2, assign31000_e48893_d_n10,) = {
    if (var_guard524 != 0.0) {
        let assign31000_e48887: f64 = (p.p4 * p.p5);
        let assign31000_e48889: f64 = (assign31000_e48887 * p.p210);
        let assign31000_e48891: f64 = (assign31000_e48889 * (nv10 - nv2));
        (assign31000_e48891, 0.0, (-assign31000_e48889), assign31000_e48889,)
    } else {
        (var_qsov, var_qsov_dn1, var_qsov_dn2, var_qsov_dn10,)
    }
};
        var_qsov = assign31000_e48893;
        var_qsov_dn1 = assign31000_e48893_d_n1;
        var_qsov_dn2 = assign31000_e48893_d_n2;
        var_qsov_dn10 = assign31000_e48893_d_n10;

        let (assign31010_e48908, assign31010_e48908_d_n0, assign31010_e48908_d_n2,) = {
    if (var_guard524 != 0.0) {
        let assign31010_e48897: f64 = ((nv0 - nv2) * p.p214);
        let assign31010_e48900: f64 = ((nv0 - nv2) * (nv0 - nv2));
        let assign31010_e48903: f64 = (p.p214 * p.p214);
        let assign31010_e48904: f64 = (assign31010_e48900 + assign31010_e48903);
        let assign31010_e48905: f64 = (assign31010_e48904).sqrt();
        let assign31010_e48906: f64 = (assign31010_e48897 / assign31010_e48905);
        (assign31010_e48906, (((p.p214 * assign31010_e48905) - (assign31010_e48897 * (((nv0 - nv2) + (nv0 - nv2)) / (2.0 * assign31010_e48905)))) / (assign31010_e48905 * assign31010_e48905)), ((((-p.p214) * assign31010_e48905) - (assign31010_e48897 * (((-(nv0 - nv2)) + (-(nv0 - nv2))) / (2.0 * assign31010_e48905)))) / (assign31010_e48905 * assign31010_e48905)),)
    } else {
        (var_vdseffcv, var_vdseffcv_dn0, var_vdseffcv_dn2,)
    }
};
        var_vdseffcv = assign31010_e48908;
        var_vdseffcv_dn0 = assign31010_e48908_d_n0;
        var_vdseffcv_dn2 = assign31010_e48908_d_n2;

        let (assign31020_e48918,) = {
    if (var_guard524 != 0.0) {
        let assign31020_e48914: f64 = (2.0 * p.p214);
        let assign31020_e48915: f64 = (p.p211 / assign31020_e48914);
        let assign31020_e48916: f64 = (p.p213).min(assign31020_e48915);
        (assign31020_e48916,)
    } else {
        (var_cgdl_l,)
    }
};
        var_cgdl_l = assign31020_e48918;

        let (assign31030_e48934, assign31030_e48934_d_n0, assign31030_e48934_d_n2,) = {
    if (var_guard524 != 0.0) {
        let assign31030_e48922: f64 = (p.p4 * p.p5);
        let assign31030_e48924: f64 = (assign31030_e48922 * p.p211);
        let assign31030_e48927: f64 = (p.p4 * p.p5);
        let assign31030_e48929: f64 = (assign31030_e48927 * var_cgdl_l);
        let assign31030_e48931: f64 = (assign31030_e48929 * var_vdseffcv);
        let assign31030_e48932: f64 = (assign31030_e48924 - assign31030_e48931);
        (assign31030_e48932, (-(assign31030_e48929 * var_vdseffcv_dn0)), (-(assign31030_e48929 * var_vdseffcv_dn2)),)
    } else {
        (var_cgdvar, var_cgdvar_dn0, var_cgdvar_dn2,)
    }
};
        var_cgdvar = assign31030_e48934;
        var_cgdvar_dn0 = assign31030_e48934_d_n0;
        var_cgdvar_dn2 = assign31030_e48934_d_n2;

        let (assign31040_e48942, assign31040_e48942_d_n0, assign31040_e48942_d_n1, assign31040_e48942_d_n2, assign31040_e48942_d_n10,) = {
    if (var_guard524 != 0.0) {
        let assign31040_e48938: f64 = (var_cgdvar).max(0.0);
        let assign31040_e48940: f64 = (assign31040_e48938 * (nv10 - nv0));
        (assign31040_e48940, ((if var_cgdvar >= 0.0 { var_cgdvar_dn0 } else { 0.0 } * (nv10 - nv0)) + (-assign31040_e48938)), 0.0, (if var_cgdvar >= 0.0 { var_cgdvar_dn2 } else { 0.0 } * (nv10 - nv0)), assign31040_e48938,)
    } else {
        (var_qdov, var_qdov_dn0, var_qdov_dn1, var_qdov_dn2, var_qdov_dn10,)
    }
};
        var_qdov = assign31040_e48942;
        var_qdov_dn0 = assign31040_e48942_d_n0;
        var_qdov_dn1 = assign31040_e48942_d_n1;
        var_qdov_dn2 = assign31040_e48942_d_n2;
        var_qdov_dn10 = assign31040_e48942_d_n10;

        let (assign31050_e48953, assign31050_e48953_d_n1, assign31050_e48953_d_n2, assign31050_e48953_d_n10,) = {
    if (var_guard524 == 0.0) {
        let assign31050_e48947: f64 = (p.p4 * p.p5);
        let assign31050_e48949: f64 = (assign31050_e48947 * p.p210);
        let assign31050_e48951: f64 = (assign31050_e48949 * (nv1 - nv2));
        (assign31050_e48951, assign31050_e48949, (-assign31050_e48949), 0.0,)
    } else {
        (var_qsov, var_qsov_dn1, var_qsov_dn2, var_qsov_dn10,)
    }
};
        var_qsov = assign31050_e48953;
        var_qsov_dn1 = assign31050_e48953_d_n1;
        var_qsov_dn2 = assign31050_e48953_d_n2;
        var_qsov_dn10 = assign31050_e48953_d_n10;

        let (assign31060_e48969, assign31060_e48969_d_n0, assign31060_e48969_d_n2,) = {
    if (var_guard524 == 0.0) {
        let assign31060_e48958: f64 = ((nv0 - nv2) * p.p214);
        let assign31060_e48961: f64 = ((nv0 - nv2) * (nv0 - nv2));
        let assign31060_e48964: f64 = (p.p214 * p.p214);
        let assign31060_e48965: f64 = (assign31060_e48961 + assign31060_e48964);
        let assign31060_e48966: f64 = (assign31060_e48965).sqrt();
        let assign31060_e48967: f64 = (assign31060_e48958 / assign31060_e48966);
        (assign31060_e48967, (((p.p214 * assign31060_e48966) - (assign31060_e48958 * (((nv0 - nv2) + (nv0 - nv2)) / (2.0 * assign31060_e48966)))) / (assign31060_e48966 * assign31060_e48966)), ((((-p.p214) * assign31060_e48966) - (assign31060_e48958 * (((-(nv0 - nv2)) + (-(nv0 - nv2))) / (2.0 * assign31060_e48966)))) / (assign31060_e48966 * assign31060_e48966)),)
    } else {
        (var_vdseffcv, var_vdseffcv_dn0, var_vdseffcv_dn2,)
    }
};
        var_vdseffcv = assign31060_e48969;
        var_vdseffcv_dn0 = assign31060_e48969_d_n0;
        var_vdseffcv_dn2 = assign31060_e48969_d_n2;

        let (assign31070_e48980,) = {
    if (var_guard524 == 0.0) {
        let assign31070_e48976: f64 = (2.0 * p.p214);
        let assign31070_e48977: f64 = (p.p211 / assign31070_e48976);
        let assign31070_e48978: f64 = (p.p213).min(assign31070_e48977);
        (assign31070_e48978,)
    } else {
        (var_cgdl_l,)
    }
};
        var_cgdl_l = assign31070_e48980;

        let (assign31080_e48997, assign31080_e48997_d_n0, assign31080_e48997_d_n2,) = {
    if (var_guard524 == 0.0) {
        let assign31080_e48985: f64 = (p.p4 * p.p5);
        let assign31080_e48987: f64 = (assign31080_e48985 * p.p211);
        let assign31080_e48990: f64 = (p.p4 * p.p5);
        let assign31080_e48992: f64 = (assign31080_e48990 * var_cgdl_l);
        let assign31080_e48994: f64 = (assign31080_e48992 * var_vdseffcv);
        let assign31080_e48995: f64 = (assign31080_e48987 - assign31080_e48994);
        (assign31080_e48995, (-(assign31080_e48992 * var_vdseffcv_dn0)), (-(assign31080_e48992 * var_vdseffcv_dn2)),)
    } else {
        (var_cgdvar, var_cgdvar_dn0, var_cgdvar_dn2,)
    }
};
        var_cgdvar = assign31080_e48997;
        var_cgdvar_dn0 = assign31080_e48997_d_n0;
        var_cgdvar_dn2 = assign31080_e48997_d_n2;

        let (assign31090_e49006, assign31090_e49006_d_n0, assign31090_e49006_d_n1, assign31090_e49006_d_n2, assign31090_e49006_d_n10,) = {
    if (var_guard524 == 0.0) {
        let assign31090_e49002: f64 = (var_cgdvar).max(0.0);
        let assign31090_e49004: f64 = (assign31090_e49002 * (nv1 - nv0));
        (assign31090_e49004, ((if var_cgdvar >= 0.0 { var_cgdvar_dn0 } else { 0.0 } * (nv1 - nv0)) + (-assign31090_e49002)), assign31090_e49002, (if var_cgdvar >= 0.0 { var_cgdvar_dn2 } else { 0.0 } * (nv1 - nv0)), 0.0,)
    } else {
        (var_qdov, var_qdov_dn0, var_qdov_dn1, var_qdov_dn2, var_qdov_dn10,)
    }
};
        var_qdov = assign31090_e49006;
        var_qdov_dn0 = assign31090_e49006_d_n0;
        var_qdov_dn1 = assign31090_e49006_d_n1;
        var_qdov_dn2 = assign31090_e49006_d_n2;
        var_qdov_dn10 = assign31090_e49006_d_n10;

        let assign31100_e49009: f64 = (p.p4 * p.p5);
        let assign31100_e49011: f64 = (assign31100_e49009 * p.p212);
        let assign31100_e49013: f64 = (assign31100_e49011 * (nv0 - nv2));
        var_qdsov = assign31100_e49013;
        var_qdsov_dn0 = assign31100_e49011;
        var_qdsov_dn2 = (-assign31100_e49011);

        let assign31150_e49030: f64 = (p.p4 * p.p5);
        let assign31150_e49032: f64 = (assign31150_e49030 * p.p215);
        let assign31150_e49034: f64 = (assign31150_e49032 * (nv3 - nv0));
        var_qbdov = assign31150_e49034;
        var_qbdov_dn0 = (-assign31150_e49032);
        var_qbdov_dn3 = assign31150_e49032;

        let assign31160_e49037: f64 = (p.p4 * p.p5);
        let assign31160_e49039: f64 = (assign31160_e49037 * p.p216);
        let assign31160_e49041: f64 = (assign31160_e49039 * (nv3 - nv2));
        var_qbsov = assign31160_e49041;
        var_qbsov_dn2 = (-assign31160_e49039);
        var_qbsov_dn3 = assign31160_e49039;

        let assign31170_e49044: f64 = (p.p4 * p.p5);
        let assign31170_e49046: f64 = (assign31170_e49044 * p.p217);
        let assign31170_e49048: f64 = (assign31170_e49046 * (nv3 - nv1));
        var_qbgov = assign31170_e49048;
        var_qbgov_dn1 = (-assign31170_e49046);
        var_qbgov_dn3 = assign31170_e49046;

        let assign31180_e49052: f64 = (var_tdev / var_tnom);
        let assign31180_e49054: f64 = (assign31180_e49052 - 1.0);
        let assign31180_e49056: f64 = (assign31180_e49054 * p.p285);
        let assign31180_e49057: f64 = (p.p279 + assign31180_e49056);
        var_vbisb_t = assign31180_e49057;
        var_vbisb_t_dn4 = ((var_tdev_dn4 / var_tnom) * p.p285);

        let assign31190_e49061: f64 = (var_tdev / var_tnom);
        let assign31190_e49063: f64 = (assign31190_e49061 - 1.0);
        let assign31190_e49065: f64 = (assign31190_e49063 * p.p283);
        let assign31190_e49066: f64 = (p.p275 + assign31190_e49065);
        var_nsb_t = assign31190_e49066;
        var_nsb_t_dn4 = ((var_tdev_dn4 / var_tnom) * p.p283);

        let assign31200_e49071: f64 = (var_tdev / var_tnom);
        let assign31200_e49073: f64 = (assign31200_e49071 - 1.0);
        let assign31200_e49074: f64 = (p.p281 * assign31200_e49073);
        let assign31200_e49075: f64 = (assign31200_e49074).exp();
        let assign31200_e49076: f64 = (p.p277 * assign31200_e49075);
        var_isb_t = assign31200_e49076;
        var_isb_t_dn4 = (p.p277 * (assign31200_e49075 * (p.p281 * (var_tdev_dn4 / var_tnom))));

        let assign31210_e49080: f64 = (var_tdev / var_tnom);
        let assign31210_e49082: f64 = (assign31210_e49080 - 1.0);
        let assign31210_e49084: f64 = (assign31210_e49082 * p.p286);
        let assign31210_e49085: f64 = (p.p280 + assign31210_e49084);
        var_vbidb_t = assign31210_e49085;
        var_vbidb_t_dn4 = ((var_tdev_dn4 / var_tnom) * p.p286);

        *var_cg_qme_slot = var_cg_qme;
        *var_cg_qme_dn0_slot = var_cg_qme_dn0;
        *var_cg_qme_dn1_slot = var_cg_qme_dn1;
        *var_cg_qme_dn12_slot = var_cg_qme_dn12;
        *var_cg_qme_dn14_slot = var_cg_qme_dn14;
        *var_cg_qme_dn15_slot = var_cg_qme_dn15;
        *var_cg_qme_dn16_slot = var_cg_qme_dn16;
        *var_cg_qme_dn17_slot = var_cg_qme_dn17;
        *var_cg_qme_dn18_slot = var_cg_qme_dn18;
        *var_cg_qme_dn19_slot = var_cg_qme_dn19;
        *var_cg_qme_dn2_slot = var_cg_qme_dn2;
        *var_cg_qme_dn20_slot = var_cg_qme_dn20;
        *var_cg_qme_dn21_slot = var_cg_qme_dn21;
        *var_cg_qme_dn22_slot = var_cg_qme_dn22;
        *var_cg_qme_dn3_slot = var_cg_qme_dn3;
        *var_cg_qme_dn4_slot = var_cg_qme_dn4;
        *var_cg_qme_dn5_slot = var_cg_qme_dn5;
        *var_cg_qme_dn6_slot = var_cg_qme_dn6;
        *var_cg_qme_dn7_slot = var_cg_qme_dn7;
        *var_cg_qme_dn8_slot = var_cg_qme_dn8;
        *var_cg_qme_dn9_slot = var_cg_qme_dn9;
        *var_cgdl_l_slot = var_cgdl_l;
        *var_cgdvar_slot = var_cgdvar;
        *var_cgdvar_dn0_slot = var_cgdvar_dn0;
        *var_cgdvar_dn2_slot = var_cgdvar_dn2;
        *var_guard524_slot = var_guard524;
        *var_isb_t_slot = var_isb_t;
        *var_isb_t_dn4_slot = var_isb_t_dn4;
        *var_nsb_t_slot = var_nsb_t;
        *var_nsb_t_dn4_slot = var_nsb_t_dn4;
        *var_qbdov_slot = var_qbdov;
        *var_qbdov_dn0_slot = var_qbdov_dn0;
        *var_qbdov_dn3_slot = var_qbdov_dn3;
        *var_qbgov_slot = var_qbgov;
        *var_qbgov_dn1_slot = var_qbgov_dn1;
        *var_qbgov_dn3_slot = var_qbgov_dn3;
        *var_qbsov_slot = var_qbsov;
        *var_qbsov_dn2_slot = var_qbsov_dn2;
        *var_qbsov_dn3_slot = var_qbsov_dn3;
        *var_qd_fp4s_slot = var_qd_fp4s;
        *var_qd_fp4s_dn0_slot = var_qd_fp4s_dn0;
        *var_qd_fp4s_dn1_slot = var_qd_fp4s_dn1;
        *var_qd_fp4s_dn12_slot = var_qd_fp4s_dn12;
        *var_qd_fp4s_dn14_slot = var_qd_fp4s_dn14;
        *var_qd_fp4s_dn15_slot = var_qd_fp4s_dn15;
        *var_qd_fp4s_dn16_slot = var_qd_fp4s_dn16;
        *var_qd_fp4s_dn17_slot = var_qd_fp4s_dn17;
        *var_qd_fp4s_dn18_slot = var_qd_fp4s_dn18;
        *var_qd_fp4s_dn19_slot = var_qd_fp4s_dn19;
        *var_qd_fp4s_dn2_slot = var_qd_fp4s_dn2;
        *var_qd_fp4s_dn20_slot = var_qd_fp4s_dn20;
        *var_qd_fp4s_dn21_slot = var_qd_fp4s_dn21;
        *var_qd_fp4s_dn22_slot = var_qd_fp4s_dn22;
        *var_qd_fp4s_dn3_slot = var_qd_fp4s_dn3;
        *var_qd_fp4s_dn4_slot = var_qd_fp4s_dn4;
        *var_qd_fp4s_dn5_slot = var_qd_fp4s_dn5;
        *var_qd_fp4s_dn6_slot = var_qd_fp4s_dn6;
        *var_qd_fp4s_dn7_slot = var_qd_fp4s_dn7;
        *var_qd_fp4s_dn8_slot = var_qd_fp4s_dn8;
        *var_qd_fp4s_dn9_slot = var_qd_fp4s_dn9;
        *var_qdov_slot = var_qdov;
        *var_qdov_dn0_slot = var_qdov_dn0;
        *var_qdov_dn1_slot = var_qdov_dn1;
        *var_qdov_dn10_slot = var_qdov_dn10;
        *var_qdov_dn2_slot = var_qdov_dn2;
        *var_qdsov_slot = var_qdsov;
        *var_qdsov_dn0_slot = var_qdsov_dn0;
        *var_qdsov_dn2_slot = var_qdsov_dn2;
        *var_qg_fp4s_slot = var_qg_fp4s;
        *var_qg_fp4s_dn0_slot = var_qg_fp4s_dn0;
        *var_qg_fp4s_dn1_slot = var_qg_fp4s_dn1;
        *var_qg_fp4s_dn12_slot = var_qg_fp4s_dn12;
        *var_qg_fp4s_dn14_slot = var_qg_fp4s_dn14;
        *var_qg_fp4s_dn15_slot = var_qg_fp4s_dn15;
        *var_qg_fp4s_dn16_slot = var_qg_fp4s_dn16;
        *var_qg_fp4s_dn17_slot = var_qg_fp4s_dn17;
        *var_qg_fp4s_dn18_slot = var_qg_fp4s_dn18;
        *var_qg_fp4s_dn19_slot = var_qg_fp4s_dn19;
        *var_qg_fp4s_dn2_slot = var_qg_fp4s_dn2;
        *var_qg_fp4s_dn20_slot = var_qg_fp4s_dn20;
        *var_qg_fp4s_dn21_slot = var_qg_fp4s_dn21;
        *var_qg_fp4s_dn22_slot = var_qg_fp4s_dn22;
        *var_qg_fp4s_dn3_slot = var_qg_fp4s_dn3;
        *var_qg_fp4s_dn4_slot = var_qg_fp4s_dn4;
        *var_qg_fp4s_dn5_slot = var_qg_fp4s_dn5;
        *var_qg_fp4s_dn6_slot = var_qg_fp4s_dn6;
        *var_qg_fp4s_dn7_slot = var_qg_fp4s_dn7;
        *var_qg_fp4s_dn8_slot = var_qg_fp4s_dn8;
        *var_qg_fp4s_dn9_slot = var_qg_fp4s_dn9;
        *var_qsov_slot = var_qsov;
        *var_qsov_dn1_slot = var_qsov_dn1;
        *var_qsov_dn10_slot = var_qsov_dn10;
        *var_qsov_dn2_slot = var_qsov_dn2;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn1_slot = var_t0_dn1;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn14_slot = var_t0_dn14;
        *var_t0_dn15_slot = var_t0_dn15;
        *var_t0_dn16_slot = var_t0_dn16;
        *var_t0_dn17_slot = var_t0_dn17;
        *var_t0_dn18_slot = var_t0_dn18;
        *var_t0_dn19_slot = var_t0_dn19;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn20_slot = var_t0_dn20;
        *var_t0_dn21_slot = var_t0_dn21;
        *var_t0_dn22_slot = var_t0_dn22;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn1_slot = var_t1_dn1;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn15_slot = var_t1_dn15;
        *var_t1_dn16_slot = var_t1_dn16;
        *var_t1_dn17_slot = var_t1_dn17;
        *var_t1_dn18_slot = var_t1_dn18;
        *var_t1_dn19_slot = var_t1_dn19;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn20_slot = var_t1_dn20;
        *var_t1_dn21_slot = var_t1_dn21;
        *var_t1_dn22_slot = var_t1_dn22;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn1_slot = var_t2_dn1;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn15_slot = var_t2_dn15;
        *var_t2_dn16_slot = var_t2_dn16;
        *var_t2_dn17_slot = var_t2_dn17;
        *var_t2_dn18_slot = var_t2_dn18;
        *var_t2_dn19_slot = var_t2_dn19;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn20_slot = var_t2_dn20;
        *var_t2_dn21_slot = var_t2_dn21;
        *var_t2_dn22_slot = var_t2_dn22;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn1_slot = var_t3_dn1;
        *var_t3_dn12_slot = var_t3_dn12;
        *var_t3_dn14_slot = var_t3_dn14;
        *var_t3_dn15_slot = var_t3_dn15;
        *var_t3_dn16_slot = var_t3_dn16;
        *var_t3_dn17_slot = var_t3_dn17;
        *var_t3_dn18_slot = var_t3_dn18;
        *var_t3_dn19_slot = var_t3_dn19;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn20_slot = var_t3_dn20;
        *var_t3_dn21_slot = var_t3_dn21;
        *var_t3_dn22_slot = var_t3_dn22;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_vbidb_t_slot = var_vbidb_t;
        *var_vbidb_t_dn4_slot = var_vbidb_t_dn4;
        *var_vbisb_t_slot = var_vbisb_t;
        *var_vbisb_t_dn4_slot = var_vbisb_t_dn4;
        *var_vdseffcv_slot = var_vdseffcv;
        *var_vdseffcv_dn0_slot = var_vdseffcv_dn0;
        *var_vdseffcv_dn2_slot = var_vdseffcv_dn2;
    }

    pub(super) fn stamp_transient_block_182(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_isb_t: f64,
        var_isb_t_dn4: f64,
        var_nsb_t: f64,
        var_nsb_t_dn4: f64,
        var_tdev: f64,
        var_tdev_dn4: f64,
        var_tnom: f64,
        var_vbidb_t: f64,
        var_vbidb_t_dn4: f64,
        var_vbisb_t: f64,
        var_vbisb_t_dn4: f64,
        var_vth: f64,
        var_vth_dn4: f64,
        var_arg_slot: &mut f64,
        var_arg_dn0_slot: &mut f64,
        var_arg_dn2_slot: &mut f64,
        var_arg_dn3_slot: &mut f64,
        var_arg_dn4_slot: &mut f64,
        var_arg_dn7_slot: &mut f64,
        var_arg_dn8_slot: &mut f64,
        var_arg_dn9_slot: &mut f64,
        var_guard525_slot: &mut f64,
        var_guard526_slot: &mut f64,
        var_guard527_slot: &mut f64,
        var_guard528_slot: &mut f64,
        var_guard529_slot: &mut f64,
        var_guard530_slot: &mut f64,
        var_guard531_slot: &mut f64,
        var_guard532_slot: &mut f64,
        var_idb_slot: &mut f64,
        var_idb_dn0_slot: &mut f64,
        var_idb_dn1_slot: &mut f64,
        var_idb_dn12_slot: &mut f64,
        var_idb_dn14_slot: &mut f64,
        var_idb_dn15_slot: &mut f64,
        var_idb_dn16_slot: &mut f64,
        var_idb_dn17_slot: &mut f64,
        var_idb_dn18_slot: &mut f64,
        var_idb_dn19_slot: &mut f64,
        var_idb_dn2_slot: &mut f64,
        var_idb_dn20_slot: &mut f64,
        var_idb_dn21_slot: &mut f64,
        var_idb_dn22_slot: &mut f64,
        var_idb_dn3_slot: &mut f64,
        var_idb_dn4_slot: &mut f64,
        var_idb_dn5_slot: &mut f64,
        var_idb_dn6_slot: &mut f64,
        var_idb_dn7_slot: &mut f64,
        var_idb_dn8_slot: &mut f64,
        var_idb_dn9_slot: &mut f64,
        var_idb_t_slot: &mut f64,
        var_idb_t_dn4_slot: &mut f64,
        var_isb_slot: &mut f64,
        var_isb_dn0_slot: &mut f64,
        var_isb_dn1_slot: &mut f64,
        var_isb_dn12_slot: &mut f64,
        var_isb_dn14_slot: &mut f64,
        var_isb_dn15_slot: &mut f64,
        var_isb_dn16_slot: &mut f64,
        var_isb_dn17_slot: &mut f64,
        var_isb_dn18_slot: &mut f64,
        var_isb_dn19_slot: &mut f64,
        var_isb_dn2_slot: &mut f64,
        var_isb_dn20_slot: &mut f64,
        var_isb_dn21_slot: &mut f64,
        var_isb_dn22_slot: &mut f64,
        var_isb_dn3_slot: &mut f64,
        var_isb_dn4_slot: &mut f64,
        var_isb_dn5_slot: &mut f64,
        var_isb_dn6_slot: &mut f64,
        var_isb_dn7_slot: &mut f64,
        var_isb_dn8_slot: &mut f64,
        var_isb_dn9_slot: &mut f64,
        var_le_slot: &mut f64,
        var_le_dn0_slot: &mut f64,
        var_le_dn2_slot: &mut f64,
        var_le_dn3_slot: &mut f64,
        var_le_dn4_slot: &mut f64,
        var_le_dn7_slot: &mut f64,
        var_le_dn8_slot: &mut f64,
        var_le_dn9_slot: &mut f64,
        var_ndb_t_slot: &mut f64,
        var_ndb_t_dn4_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn1_slot: &mut f64,
        var_t3_dn12_slot: &mut f64,
        var_t3_dn14_slot: &mut f64,
        var_t3_dn15_slot: &mut f64,
        var_t3_dn16_slot: &mut f64,
        var_t3_dn17_slot: &mut f64,
        var_t3_dn18_slot: &mut f64,
        var_t3_dn19_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn20_slot: &mut f64,
        var_t3_dn21_slot: &mut f64,
        var_t3_dn22_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_vbdl_slot: &mut f64,
        var_vbdl_dn0_slot: &mut f64,
        var_vbdl_dn3_slot: &mut f64,
        var_vbdl_dn4_slot: &mut f64,
        var_vbsl_slot: &mut f64,
        var_vbsl_dn2_slot: &mut f64,
        var_vbsl_dn3_slot: &mut f64,
        var_vbsl_dn4_slot: &mut f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let mut var_arg: f64 = *var_arg_slot;
        let mut var_arg_dn0: f64 = *var_arg_dn0_slot;
        let mut var_arg_dn2: f64 = *var_arg_dn2_slot;
        let mut var_arg_dn3: f64 = *var_arg_dn3_slot;
        let mut var_arg_dn4: f64 = *var_arg_dn4_slot;
        let mut var_arg_dn7: f64 = *var_arg_dn7_slot;
        let mut var_arg_dn8: f64 = *var_arg_dn8_slot;
        let mut var_arg_dn9: f64 = *var_arg_dn9_slot;
        let mut var_guard525: f64 = *var_guard525_slot;
        let mut var_guard526: f64 = *var_guard526_slot;
        let mut var_guard527: f64 = *var_guard527_slot;
        let mut var_guard528: f64 = *var_guard528_slot;
        let mut var_guard529: f64 = *var_guard529_slot;
        let mut var_guard530: f64 = *var_guard530_slot;
        let mut var_guard531: f64 = *var_guard531_slot;
        let mut var_guard532: f64 = *var_guard532_slot;
        let mut var_idb: f64 = *var_idb_slot;
        let mut var_idb_dn0: f64 = *var_idb_dn0_slot;
        let mut var_idb_dn1: f64 = *var_idb_dn1_slot;
        let mut var_idb_dn12: f64 = *var_idb_dn12_slot;
        let mut var_idb_dn14: f64 = *var_idb_dn14_slot;
        let mut var_idb_dn15: f64 = *var_idb_dn15_slot;
        let mut var_idb_dn16: f64 = *var_idb_dn16_slot;
        let mut var_idb_dn17: f64 = *var_idb_dn17_slot;
        let mut var_idb_dn18: f64 = *var_idb_dn18_slot;
        let mut var_idb_dn19: f64 = *var_idb_dn19_slot;
        let mut var_idb_dn2: f64 = *var_idb_dn2_slot;
        let mut var_idb_dn20: f64 = *var_idb_dn20_slot;
        let mut var_idb_dn21: f64 = *var_idb_dn21_slot;
        let mut var_idb_dn22: f64 = *var_idb_dn22_slot;
        let mut var_idb_dn3: f64 = *var_idb_dn3_slot;
        let mut var_idb_dn4: f64 = *var_idb_dn4_slot;
        let mut var_idb_dn5: f64 = *var_idb_dn5_slot;
        let mut var_idb_dn6: f64 = *var_idb_dn6_slot;
        let mut var_idb_dn7: f64 = *var_idb_dn7_slot;
        let mut var_idb_dn8: f64 = *var_idb_dn8_slot;
        let mut var_idb_dn9: f64 = *var_idb_dn9_slot;
        let mut var_idb_t: f64 = *var_idb_t_slot;
        let mut var_idb_t_dn4: f64 = *var_idb_t_dn4_slot;
        let mut var_isb: f64 = *var_isb_slot;
        let mut var_isb_dn0: f64 = *var_isb_dn0_slot;
        let mut var_isb_dn1: f64 = *var_isb_dn1_slot;
        let mut var_isb_dn12: f64 = *var_isb_dn12_slot;
        let mut var_isb_dn14: f64 = *var_isb_dn14_slot;
        let mut var_isb_dn15: f64 = *var_isb_dn15_slot;
        let mut var_isb_dn16: f64 = *var_isb_dn16_slot;
        let mut var_isb_dn17: f64 = *var_isb_dn17_slot;
        let mut var_isb_dn18: f64 = *var_isb_dn18_slot;
        let mut var_isb_dn19: f64 = *var_isb_dn19_slot;
        let mut var_isb_dn2: f64 = *var_isb_dn2_slot;
        let mut var_isb_dn20: f64 = *var_isb_dn20_slot;
        let mut var_isb_dn21: f64 = *var_isb_dn21_slot;
        let mut var_isb_dn22: f64 = *var_isb_dn22_slot;
        let mut var_isb_dn3: f64 = *var_isb_dn3_slot;
        let mut var_isb_dn4: f64 = *var_isb_dn4_slot;
        let mut var_isb_dn5: f64 = *var_isb_dn5_slot;
        let mut var_isb_dn6: f64 = *var_isb_dn6_slot;
        let mut var_isb_dn7: f64 = *var_isb_dn7_slot;
        let mut var_isb_dn8: f64 = *var_isb_dn8_slot;
        let mut var_isb_dn9: f64 = *var_isb_dn9_slot;
        let mut var_le: f64 = *var_le_slot;
        let mut var_le_dn0: f64 = *var_le_dn0_slot;
        let mut var_le_dn2: f64 = *var_le_dn2_slot;
        let mut var_le_dn3: f64 = *var_le_dn3_slot;
        let mut var_le_dn4: f64 = *var_le_dn4_slot;
        let mut var_le_dn7: f64 = *var_le_dn7_slot;
        let mut var_le_dn8: f64 = *var_le_dn8_slot;
        let mut var_le_dn9: f64 = *var_le_dn9_slot;
        let mut var_ndb_t: f64 = *var_ndb_t_slot;
        let mut var_ndb_t_dn4: f64 = *var_ndb_t_dn4_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn1: f64 = *var_t3_dn1_slot;
        let mut var_t3_dn12: f64 = *var_t3_dn12_slot;
        let mut var_t3_dn14: f64 = *var_t3_dn14_slot;
        let mut var_t3_dn15: f64 = *var_t3_dn15_slot;
        let mut var_t3_dn16: f64 = *var_t3_dn16_slot;
        let mut var_t3_dn17: f64 = *var_t3_dn17_slot;
        let mut var_t3_dn18: f64 = *var_t3_dn18_slot;
        let mut var_t3_dn19: f64 = *var_t3_dn19_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn20: f64 = *var_t3_dn20_slot;
        let mut var_t3_dn21: f64 = *var_t3_dn21_slot;
        let mut var_t3_dn22: f64 = *var_t3_dn22_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_vbdl: f64 = *var_vbdl_slot;
        let mut var_vbdl_dn0: f64 = *var_vbdl_dn0_slot;
        let mut var_vbdl_dn3: f64 = *var_vbdl_dn3_slot;
        let mut var_vbdl_dn4: f64 = *var_vbdl_dn4_slot;
        let mut var_vbsl: f64 = *var_vbsl_slot;
        let mut var_vbsl_dn2: f64 = *var_vbsl_dn2_slot;
        let mut var_vbsl_dn3: f64 = *var_vbsl_dn3_slot;
        let mut var_vbsl_dn4: f64 = *var_vbsl_dn4_slot;

        let assign31220_e49089: f64 = (var_tdev / var_tnom);
        let assign31220_e49091: f64 = (assign31220_e49089 - 1.0);
        let assign31220_e49093: f64 = (assign31220_e49091 * p.p284);
        let assign31220_e49094: f64 = (p.p276 + assign31220_e49093);
        var_ndb_t = assign31220_e49094;
        var_ndb_t_dn4 = ((var_tdev_dn4 / var_tnom) * p.p284);

        let assign31230_e49099: f64 = (var_tdev / var_tnom);
        let assign31230_e49101: f64 = (assign31230_e49099 - 1.0);
        let assign31230_e49102: f64 = (p.p282 * assign31230_e49101);
        let assign31230_e49103: f64 = (assign31230_e49102).exp();
        let assign31230_e49104: f64 = (p.p278 * assign31230_e49103);
        var_idb_t = assign31230_e49104;
        var_idb_t_dn4 = (p.p278 * (assign31230_e49103 * (p.p282 * (var_tdev_dn4 / var_tnom))));

        let assign31240_e49107: f64 = (p.p4 * p.p5);
        let assign31240_e49109: f64 = (assign31240_e49107 * var_idb_t);
        var_t3 = assign31240_e49109;
        var_t3_dn0 = 0.0;
        var_t3_dn1 = 0.0;
        var_t3_dn2 = 0.0;
        var_t3_dn3 = 0.0;
        var_t3_dn4 = (assign31240_e49107 * var_idb_t_dn4);
        var_t3_dn5 = 0.0;
        var_t3_dn6 = 0.0;
        var_t3_dn7 = 0.0;
        var_t3_dn8 = 0.0;
        var_t3_dn9 = 0.0;
        var_t3_dn12 = 0.0;
        var_t3_dn14 = 0.0;
        var_t3_dn15 = 0.0;
        var_t3_dn16 = 0.0;
        var_t3_dn17 = 0.0;
        var_t3_dn18 = 0.0;
        var_t3_dn19 = 0.0;
        var_t3_dn20 = 0.0;
        var_t3_dn21 = 0.0;
        var_t3_dn22 = 0.0;

        let assign31250_e49112: f64 = ((nv0 - nv3) - var_vbidb_t);
        let assign31250_e49114: f64 = (assign31250_e49112).max(0.0);
        var_vbdl = assign31250_e49114;
        var_vbdl_dn0 = if assign31250_e49112 >= 0.0 { 1.0 } else { 0.0 };
        var_vbdl_dn3 = if assign31250_e49112 >= 0.0 { -1.0 } else { 0.0 };
        var_vbdl_dn4 = if assign31250_e49112 >= 0.0 { (-var_vbidb_t_dn4) } else { 0.0 };

        let assign31260_e49117: f64 = if var_t3 > 0.0 { 1.0 } else { 0.0 };
        var_guard525 = assign31260_e49117;

        let assign31270_e49120: f64 = if var_vbdl > 0.0 { 1.0 } else { 0.0 };
        var_guard526 = assign31270_e49120;

        let (assign31280_e49132, assign31280_e49132_d_n0, assign31280_e49132_d_n2, assign31280_e49132_d_n3, assign31280_e49132_d_n4, assign31280_e49132_d_n7, assign31280_e49132_d_n8, assign31280_e49132_d_n9,) = {
    if ((var_guard525 != 0.0) && (var_guard526 != 0.0)) {
        let assign31280_e49126: f64 = (var_vbdl).powf(1.0);
        let assign31280_e49129: f64 = (var_ndb_t * var_vth);
        let assign31280_e49130: f64 = (assign31280_e49126 / assign31280_e49129);
        (assign31280_e49130, (if 0.0 == 0.0 && ((1.0) as f64).is_finite() && ((1.0) as f64).fract() == 0.0 { if 1.0 == 0.0 { 0.0 } else { ((var_vbdl).powf(1.0 - 1.0) * var_vbdl_dn0) } } else { (assign31280_e49126 * (var_vbdl_dn0 / var_vbdl)) } / assign31280_e49129), 0.0, (if 0.0 == 0.0 && ((1.0) as f64).is_finite() && ((1.0) as f64).fract() == 0.0 { if 1.0 == 0.0 { 0.0 } else { ((var_vbdl).powf(1.0 - 1.0) * var_vbdl_dn3) } } else { (assign31280_e49126 * (var_vbdl_dn3 / var_vbdl)) } / assign31280_e49129), (((if 0.0 == 0.0 && ((1.0) as f64).is_finite() && ((1.0) as f64).fract() == 0.0 { if 1.0 == 0.0 { 0.0 } else { ((var_vbdl).powf(1.0 - 1.0) * var_vbdl_dn4) } } else { (assign31280_e49126 * (var_vbdl_dn4 / var_vbdl)) } * assign31280_e49129) - (assign31280_e49126 * ((var_ndb_t_dn4 * var_vth) + (var_ndb_t * var_vth_dn4)))) / (assign31280_e49129 * assign31280_e49129)), 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn7, var_arg_dn8, var_arg_dn9,)
    }
};
        var_arg = assign31280_e49132;
        var_arg_dn0 = assign31280_e49132_d_n0;
        var_arg_dn2 = assign31280_e49132_d_n2;
        var_arg_dn3 = assign31280_e49132_d_n3;
        var_arg_dn4 = assign31280_e49132_d_n4;
        var_arg_dn7 = assign31280_e49132_d_n7;
        var_arg_dn8 = assign31280_e49132_d_n8;
        var_arg_dn9 = assign31280_e49132_d_n9;

        let assign31290_e49135: f64 = if var_arg > 80.0 { 1.0 } else { 0.0 };
        var_guard527 = assign31290_e49135;

        let (assign31300_e49147, assign31300_e49147_d_n0, assign31300_e49147_d_n2, assign31300_e49147_d_n3, assign31300_e49147_d_n4, assign31300_e49147_d_n7, assign31300_e49147_d_n8, assign31300_e49147_d_n9,) = {
    if (((var_guard525 != 0.0) && (var_guard526 != 0.0)) && (var_guard527 != 0.0)) {
        let assign31300_e49144: f64 = (var_arg - 80.0);
        let assign31300_e49145: f64 = (1.0 + assign31300_e49144);
        (assign31300_e49145, var_arg_dn0, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn7, var_arg_dn8, var_arg_dn9,)
    } else {
        (var_le, var_le_dn0, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn7, var_le_dn8, var_le_dn9,)
    }
};
        var_le = assign31300_e49147;
        var_le_dn0 = assign31300_e49147_d_n0;
        var_le_dn2 = assign31300_e49147_d_n2;
        var_le_dn3 = assign31300_e49147_d_n3;
        var_le_dn4 = assign31300_e49147_d_n4;
        var_le_dn7 = assign31300_e49147_d_n7;
        var_le_dn8 = assign31300_e49147_d_n8;
        var_le_dn9 = assign31300_e49147_d_n9;

        let (assign31310_e49155, assign31310_e49155_d_n0, assign31310_e49155_d_n2, assign31310_e49155_d_n3, assign31310_e49155_d_n4, assign31310_e49155_d_n7, assign31310_e49155_d_n8, assign31310_e49155_d_n9,) = {
    if (((var_guard525 != 0.0) && (var_guard526 != 0.0)) && (var_guard527 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn7, var_arg_dn8, var_arg_dn9,)
    }
};
        var_arg = assign31310_e49155;
        var_arg_dn0 = assign31310_e49155_d_n0;
        var_arg_dn2 = assign31310_e49155_d_n2;
        var_arg_dn3 = assign31310_e49155_d_n3;
        var_arg_dn4 = assign31310_e49155_d_n4;
        var_arg_dn7 = assign31310_e49155_d_n7;
        var_arg_dn8 = assign31310_e49155_d_n8;
        var_arg_dn9 = assign31310_e49155_d_n9;

        let (assign31320_e49164, assign31320_e49164_d_n0, assign31320_e49164_d_n2, assign31320_e49164_d_n3, assign31320_e49164_d_n4, assign31320_e49164_d_n7, assign31320_e49164_d_n8, assign31320_e49164_d_n9,) = {
    if (((var_guard525 != 0.0) && (var_guard526 != 0.0)) && (var_guard527 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_le, var_le_dn0, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn7, var_le_dn8, var_le_dn9,)
    }
};
        var_le = assign31320_e49164;
        var_le_dn0 = assign31320_e49164_d_n0;
        var_le_dn2 = assign31320_e49164_d_n2;
        var_le_dn3 = assign31320_e49164_d_n3;
        var_le_dn4 = assign31320_e49164_d_n4;
        var_le_dn7 = assign31320_e49164_d_n7;
        var_le_dn8 = assign31320_e49164_d_n8;
        var_le_dn9 = assign31320_e49164_d_n9;

        let (assign31330_e49173, assign31330_e49173_d_n0, assign31330_e49173_d_n2, assign31330_e49173_d_n3, assign31330_e49173_d_n4, assign31330_e49173_d_n7, assign31330_e49173_d_n8, assign31330_e49173_d_n9,) = {
    if ((var_guard525 != 0.0) && (var_guard526 != 0.0)) {
        let assign31330_e49170: f64 = (var_arg).exp();
        let assign31330_e49171: f64 = (var_le * assign31330_e49170);
        (assign31330_e49171, ((var_le_dn0 * assign31330_e49170) + (var_le * (assign31330_e49170 * var_arg_dn0))), ((var_le_dn2 * assign31330_e49170) + (var_le * (assign31330_e49170 * var_arg_dn2))), ((var_le_dn3 * assign31330_e49170) + (var_le * (assign31330_e49170 * var_arg_dn3))), ((var_le_dn4 * assign31330_e49170) + (var_le * (assign31330_e49170 * var_arg_dn4))), ((var_le_dn7 * assign31330_e49170) + (var_le * (assign31330_e49170 * var_arg_dn7))), ((var_le_dn8 * assign31330_e49170) + (var_le * (assign31330_e49170 * var_arg_dn8))), ((var_le_dn9 * assign31330_e49170) + (var_le * (assign31330_e49170 * var_arg_dn9))),)
    } else {
        (var_le, var_le_dn0, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn7, var_le_dn8, var_le_dn9,)
    }
};
        var_le = assign31330_e49173;
        var_le_dn0 = assign31330_e49173_d_n0;
        var_le_dn2 = assign31330_e49173_d_n2;
        var_le_dn3 = assign31330_e49173_d_n3;
        var_le_dn4 = assign31330_e49173_d_n4;
        var_le_dn7 = assign31330_e49173_d_n7;
        var_le_dn8 = assign31330_e49173_d_n8;
        var_le_dn9 = assign31330_e49173_d_n9;

        let (assign31340_e49183, assign31340_e49183_d_n0, assign31340_e49183_d_n1, assign31340_e49183_d_n2, assign31340_e49183_d_n3, assign31340_e49183_d_n4, assign31340_e49183_d_n5, assign31340_e49183_d_n6, assign31340_e49183_d_n7, assign31340_e49183_d_n8, assign31340_e49183_d_n9, assign31340_e49183_d_n12, assign31340_e49183_d_n14, assign31340_e49183_d_n15, assign31340_e49183_d_n16, assign31340_e49183_d_n17, assign31340_e49183_d_n18, assign31340_e49183_d_n19, assign31340_e49183_d_n20, assign31340_e49183_d_n21, assign31340_e49183_d_n22,) = {
    if ((var_guard525 != 0.0) && (var_guard526 != 0.0)) {
        let assign31340_e49180: f64 = (var_le - 1.0);
        let assign31340_e49181: f64 = (var_t3 * assign31340_e49180);
        (assign31340_e49181, ((var_t3_dn0 * assign31340_e49180) + (var_t3 * var_le_dn0)), (var_t3_dn1 * assign31340_e49180), ((var_t3_dn2 * assign31340_e49180) + (var_t3 * var_le_dn2)), ((var_t3_dn3 * assign31340_e49180) + (var_t3 * var_le_dn3)), ((var_t3_dn4 * assign31340_e49180) + (var_t3 * var_le_dn4)), (var_t3_dn5 * assign31340_e49180), (var_t3_dn6 * assign31340_e49180), ((var_t3_dn7 * assign31340_e49180) + (var_t3 * var_le_dn7)), ((var_t3_dn8 * assign31340_e49180) + (var_t3 * var_le_dn8)), ((var_t3_dn9 * assign31340_e49180) + (var_t3 * var_le_dn9)), (var_t3_dn12 * assign31340_e49180), (var_t3_dn14 * assign31340_e49180), (var_t3_dn15 * assign31340_e49180), (var_t3_dn16 * assign31340_e49180), (var_t3_dn17 * assign31340_e49180), (var_t3_dn18 * assign31340_e49180), (var_t3_dn19 * assign31340_e49180), (var_t3_dn20 * assign31340_e49180), (var_t3_dn21 * assign31340_e49180), (var_t3_dn22 * assign31340_e49180),)
    } else {
        (var_idb, var_idb_dn0, var_idb_dn1, var_idb_dn2, var_idb_dn3, var_idb_dn4, var_idb_dn5, var_idb_dn6, var_idb_dn7, var_idb_dn8, var_idb_dn9, var_idb_dn12, var_idb_dn14, var_idb_dn15, var_idb_dn16, var_idb_dn17, var_idb_dn18, var_idb_dn19, var_idb_dn20, var_idb_dn21, var_idb_dn22,)
    }
};
        var_idb = assign31340_e49183;
        var_idb_dn0 = assign31340_e49183_d_n0;
        var_idb_dn1 = assign31340_e49183_d_n1;
        var_idb_dn2 = assign31340_e49183_d_n2;
        var_idb_dn3 = assign31340_e49183_d_n3;
        var_idb_dn4 = assign31340_e49183_d_n4;
        var_idb_dn5 = assign31340_e49183_d_n5;
        var_idb_dn6 = assign31340_e49183_d_n6;
        var_idb_dn7 = assign31340_e49183_d_n7;
        var_idb_dn8 = assign31340_e49183_d_n8;
        var_idb_dn9 = assign31340_e49183_d_n9;
        var_idb_dn12 = assign31340_e49183_d_n12;
        var_idb_dn14 = assign31340_e49183_d_n14;
        var_idb_dn15 = assign31340_e49183_d_n15;
        var_idb_dn16 = assign31340_e49183_d_n16;
        var_idb_dn17 = assign31340_e49183_d_n17;
        var_idb_dn18 = assign31340_e49183_d_n18;
        var_idb_dn19 = assign31340_e49183_d_n19;
        var_idb_dn20 = assign31340_e49183_d_n20;
        var_idb_dn21 = assign31340_e49183_d_n21;
        var_idb_dn22 = assign31340_e49183_d_n22;

        let (assign31350_e49194, assign31350_e49194_d_n0, assign31350_e49194_d_n2, assign31350_e49194_d_n3, assign31350_e49194_d_n4, assign31350_e49194_d_n7, assign31350_e49194_d_n8, assign31350_e49194_d_n9,) = {
    if ((var_guard525 != 0.0) && (var_guard526 == 0.0)) {
        let assign31350_e49191: f64 = (var_ndb_t * var_vth);
        let assign31350_e49192: f64 = (var_vbdl / assign31350_e49191);
        (assign31350_e49192, (var_vbdl_dn0 / assign31350_e49191), 0.0, (var_vbdl_dn3 / assign31350_e49191), (((var_vbdl_dn4 * assign31350_e49191) - (var_vbdl * ((var_ndb_t_dn4 * var_vth) + (var_ndb_t * var_vth_dn4)))) / (assign31350_e49191 * assign31350_e49191)), 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn7, var_arg_dn8, var_arg_dn9,)
    }
};
        var_arg = assign31350_e49194;
        var_arg_dn0 = assign31350_e49194_d_n0;
        var_arg_dn2 = assign31350_e49194_d_n2;
        var_arg_dn3 = assign31350_e49194_d_n3;
        var_arg_dn4 = assign31350_e49194_d_n4;
        var_arg_dn7 = assign31350_e49194_d_n7;
        var_arg_dn8 = assign31350_e49194_d_n8;
        var_arg_dn9 = assign31350_e49194_d_n9;

        let assign31360_e49197: f64 = if var_arg > 80.0 { 1.0 } else { 0.0 };
        var_guard528 = assign31360_e49197;

        let (assign31370_e49210, assign31370_e49210_d_n0, assign31370_e49210_d_n2, assign31370_e49210_d_n3, assign31370_e49210_d_n4, assign31370_e49210_d_n7, assign31370_e49210_d_n8, assign31370_e49210_d_n9,) = {
    if (((var_guard525 != 0.0) && (var_guard526 == 0.0)) && (var_guard528 != 0.0)) {
        let assign31370_e49207: f64 = (var_arg - 80.0);
        let assign31370_e49208: f64 = (1.0 + assign31370_e49207);
        (assign31370_e49208, var_arg_dn0, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn7, var_arg_dn8, var_arg_dn9,)
    } else {
        (var_le, var_le_dn0, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn7, var_le_dn8, var_le_dn9,)
    }
};
        var_le = assign31370_e49210;
        var_le_dn0 = assign31370_e49210_d_n0;
        var_le_dn2 = assign31370_e49210_d_n2;
        var_le_dn3 = assign31370_e49210_d_n3;
        var_le_dn4 = assign31370_e49210_d_n4;
        var_le_dn7 = assign31370_e49210_d_n7;
        var_le_dn8 = assign31370_e49210_d_n8;
        var_le_dn9 = assign31370_e49210_d_n9;

        let (assign31380_e49219, assign31380_e49219_d_n0, assign31380_e49219_d_n2, assign31380_e49219_d_n3, assign31380_e49219_d_n4, assign31380_e49219_d_n7, assign31380_e49219_d_n8, assign31380_e49219_d_n9,) = {
    if (((var_guard525 != 0.0) && (var_guard526 == 0.0)) && (var_guard528 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn7, var_arg_dn8, var_arg_dn9,)
    }
};
        var_arg = assign31380_e49219;
        var_arg_dn0 = assign31380_e49219_d_n0;
        var_arg_dn2 = assign31380_e49219_d_n2;
        var_arg_dn3 = assign31380_e49219_d_n3;
        var_arg_dn4 = assign31380_e49219_d_n4;
        var_arg_dn7 = assign31380_e49219_d_n7;
        var_arg_dn8 = assign31380_e49219_d_n8;
        var_arg_dn9 = assign31380_e49219_d_n9;

        let (assign31390_e49229, assign31390_e49229_d_n0, assign31390_e49229_d_n2, assign31390_e49229_d_n3, assign31390_e49229_d_n4, assign31390_e49229_d_n7, assign31390_e49229_d_n8, assign31390_e49229_d_n9,) = {
    if (((var_guard525 != 0.0) && (var_guard526 == 0.0)) && (var_guard528 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_le, var_le_dn0, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn7, var_le_dn8, var_le_dn9,)
    }
};
        var_le = assign31390_e49229;
        var_le_dn0 = assign31390_e49229_d_n0;
        var_le_dn2 = assign31390_e49229_d_n2;
        var_le_dn3 = assign31390_e49229_d_n3;
        var_le_dn4 = assign31390_e49229_d_n4;
        var_le_dn7 = assign31390_e49229_d_n7;
        var_le_dn8 = assign31390_e49229_d_n8;
        var_le_dn9 = assign31390_e49229_d_n9;

        let (assign31400_e49239, assign31400_e49239_d_n0, assign31400_e49239_d_n2, assign31400_e49239_d_n3, assign31400_e49239_d_n4, assign31400_e49239_d_n7, assign31400_e49239_d_n8, assign31400_e49239_d_n9,) = {
    if ((var_guard525 != 0.0) && (var_guard526 == 0.0)) {
        let assign31400_e49236: f64 = (var_arg).exp();
        let assign31400_e49237: f64 = (var_le * assign31400_e49236);
        (assign31400_e49237, ((var_le_dn0 * assign31400_e49236) + (var_le * (assign31400_e49236 * var_arg_dn0))), ((var_le_dn2 * assign31400_e49236) + (var_le * (assign31400_e49236 * var_arg_dn2))), ((var_le_dn3 * assign31400_e49236) + (var_le * (assign31400_e49236 * var_arg_dn3))), ((var_le_dn4 * assign31400_e49236) + (var_le * (assign31400_e49236 * var_arg_dn4))), ((var_le_dn7 * assign31400_e49236) + (var_le * (assign31400_e49236 * var_arg_dn7))), ((var_le_dn8 * assign31400_e49236) + (var_le * (assign31400_e49236 * var_arg_dn8))), ((var_le_dn9 * assign31400_e49236) + (var_le * (assign31400_e49236 * var_arg_dn9))),)
    } else {
        (var_le, var_le_dn0, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn7, var_le_dn8, var_le_dn9,)
    }
};
        var_le = assign31400_e49239;
        var_le_dn0 = assign31400_e49239_d_n0;
        var_le_dn2 = assign31400_e49239_d_n2;
        var_le_dn3 = assign31400_e49239_d_n3;
        var_le_dn4 = assign31400_e49239_d_n4;
        var_le_dn7 = assign31400_e49239_d_n7;
        var_le_dn8 = assign31400_e49239_d_n8;
        var_le_dn9 = assign31400_e49239_d_n9;

        let (assign31410_e49250, assign31410_e49250_d_n0, assign31410_e49250_d_n1, assign31410_e49250_d_n2, assign31410_e49250_d_n3, assign31410_e49250_d_n4, assign31410_e49250_d_n5, assign31410_e49250_d_n6, assign31410_e49250_d_n7, assign31410_e49250_d_n8, assign31410_e49250_d_n9, assign31410_e49250_d_n12, assign31410_e49250_d_n14, assign31410_e49250_d_n15, assign31410_e49250_d_n16, assign31410_e49250_d_n17, assign31410_e49250_d_n18, assign31410_e49250_d_n19, assign31410_e49250_d_n20, assign31410_e49250_d_n21, assign31410_e49250_d_n22,) = {
    if ((var_guard525 != 0.0) && (var_guard526 == 0.0)) {
        let assign31410_e49247: f64 = (var_le - 1.0);
        let assign31410_e49248: f64 = (var_t3 * assign31410_e49247);
        (assign31410_e49248, ((var_t3_dn0 * assign31410_e49247) + (var_t3 * var_le_dn0)), (var_t3_dn1 * assign31410_e49247), ((var_t3_dn2 * assign31410_e49247) + (var_t3 * var_le_dn2)), ((var_t3_dn3 * assign31410_e49247) + (var_t3 * var_le_dn3)), ((var_t3_dn4 * assign31410_e49247) + (var_t3 * var_le_dn4)), (var_t3_dn5 * assign31410_e49247), (var_t3_dn6 * assign31410_e49247), ((var_t3_dn7 * assign31410_e49247) + (var_t3 * var_le_dn7)), ((var_t3_dn8 * assign31410_e49247) + (var_t3 * var_le_dn8)), ((var_t3_dn9 * assign31410_e49247) + (var_t3 * var_le_dn9)), (var_t3_dn12 * assign31410_e49247), (var_t3_dn14 * assign31410_e49247), (var_t3_dn15 * assign31410_e49247), (var_t3_dn16 * assign31410_e49247), (var_t3_dn17 * assign31410_e49247), (var_t3_dn18 * assign31410_e49247), (var_t3_dn19 * assign31410_e49247), (var_t3_dn20 * assign31410_e49247), (var_t3_dn21 * assign31410_e49247), (var_t3_dn22 * assign31410_e49247),)
    } else {
        (var_idb, var_idb_dn0, var_idb_dn1, var_idb_dn2, var_idb_dn3, var_idb_dn4, var_idb_dn5, var_idb_dn6, var_idb_dn7, var_idb_dn8, var_idb_dn9, var_idb_dn12, var_idb_dn14, var_idb_dn15, var_idb_dn16, var_idb_dn17, var_idb_dn18, var_idb_dn19, var_idb_dn20, var_idb_dn21, var_idb_dn22,)
    }
};
        var_idb = assign31410_e49250;
        var_idb_dn0 = assign31410_e49250_d_n0;
        var_idb_dn1 = assign31410_e49250_d_n1;
        var_idb_dn2 = assign31410_e49250_d_n2;
        var_idb_dn3 = assign31410_e49250_d_n3;
        var_idb_dn4 = assign31410_e49250_d_n4;
        var_idb_dn5 = assign31410_e49250_d_n5;
        var_idb_dn6 = assign31410_e49250_d_n6;
        var_idb_dn7 = assign31410_e49250_d_n7;
        var_idb_dn8 = assign31410_e49250_d_n8;
        var_idb_dn9 = assign31410_e49250_d_n9;
        var_idb_dn12 = assign31410_e49250_d_n12;
        var_idb_dn14 = assign31410_e49250_d_n14;
        var_idb_dn15 = assign31410_e49250_d_n15;
        var_idb_dn16 = assign31410_e49250_d_n16;
        var_idb_dn17 = assign31410_e49250_d_n17;
        var_idb_dn18 = assign31410_e49250_d_n18;
        var_idb_dn19 = assign31410_e49250_d_n19;
        var_idb_dn20 = assign31410_e49250_d_n20;
        var_idb_dn21 = assign31410_e49250_d_n21;
        var_idb_dn22 = assign31410_e49250_d_n22;

        let (assign31420_e49255, assign31420_e49255_d_n0, assign31420_e49255_d_n1, assign31420_e49255_d_n2, assign31420_e49255_d_n3, assign31420_e49255_d_n4, assign31420_e49255_d_n5, assign31420_e49255_d_n6, assign31420_e49255_d_n7, assign31420_e49255_d_n8, assign31420_e49255_d_n9, assign31420_e49255_d_n12, assign31420_e49255_d_n14, assign31420_e49255_d_n15, assign31420_e49255_d_n16, assign31420_e49255_d_n17, assign31420_e49255_d_n18, assign31420_e49255_d_n19, assign31420_e49255_d_n20, assign31420_e49255_d_n21, assign31420_e49255_d_n22,) = {
    if (var_guard525 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_idb, var_idb_dn0, var_idb_dn1, var_idb_dn2, var_idb_dn3, var_idb_dn4, var_idb_dn5, var_idb_dn6, var_idb_dn7, var_idb_dn8, var_idb_dn9, var_idb_dn12, var_idb_dn14, var_idb_dn15, var_idb_dn16, var_idb_dn17, var_idb_dn18, var_idb_dn19, var_idb_dn20, var_idb_dn21, var_idb_dn22,)
    }
};
        var_idb = assign31420_e49255;
        var_idb_dn0 = assign31420_e49255_d_n0;
        var_idb_dn1 = assign31420_e49255_d_n1;
        var_idb_dn2 = assign31420_e49255_d_n2;
        var_idb_dn3 = assign31420_e49255_d_n3;
        var_idb_dn4 = assign31420_e49255_d_n4;
        var_idb_dn5 = assign31420_e49255_d_n5;
        var_idb_dn6 = assign31420_e49255_d_n6;
        var_idb_dn7 = assign31420_e49255_d_n7;
        var_idb_dn8 = assign31420_e49255_d_n8;
        var_idb_dn9 = assign31420_e49255_d_n9;
        var_idb_dn12 = assign31420_e49255_d_n12;
        var_idb_dn14 = assign31420_e49255_d_n14;
        var_idb_dn15 = assign31420_e49255_d_n15;
        var_idb_dn16 = assign31420_e49255_d_n16;
        var_idb_dn17 = assign31420_e49255_d_n17;
        var_idb_dn18 = assign31420_e49255_d_n18;
        var_idb_dn19 = assign31420_e49255_d_n19;
        var_idb_dn20 = assign31420_e49255_d_n20;
        var_idb_dn21 = assign31420_e49255_d_n21;
        var_idb_dn22 = assign31420_e49255_d_n22;

        let assign31430_e49258: f64 = ((nv2 - nv3) - var_vbisb_t);
        let assign31430_e49260: f64 = (assign31430_e49258).max(0.0);
        var_vbsl = assign31430_e49260;
        var_vbsl_dn2 = if assign31430_e49258 >= 0.0 { 1.0 } else { 0.0 };
        var_vbsl_dn3 = if assign31430_e49258 >= 0.0 { -1.0 } else { 0.0 };
        var_vbsl_dn4 = if assign31430_e49258 >= 0.0 { (-var_vbisb_t_dn4) } else { 0.0 };

        let assign31440_e49263: f64 = (p.p4 * p.p5);
        let assign31440_e49265: f64 = (assign31440_e49263 * var_isb_t);
        var_t3 = assign31440_e49265;
        var_t3_dn0 = 0.0;
        var_t3_dn1 = 0.0;
        var_t3_dn2 = 0.0;
        var_t3_dn3 = 0.0;
        var_t3_dn4 = (assign31440_e49263 * var_isb_t_dn4);
        var_t3_dn5 = 0.0;
        var_t3_dn6 = 0.0;
        var_t3_dn7 = 0.0;
        var_t3_dn8 = 0.0;
        var_t3_dn9 = 0.0;
        var_t3_dn12 = 0.0;
        var_t3_dn14 = 0.0;
        var_t3_dn15 = 0.0;
        var_t3_dn16 = 0.0;
        var_t3_dn17 = 0.0;
        var_t3_dn18 = 0.0;
        var_t3_dn19 = 0.0;
        var_t3_dn20 = 0.0;
        var_t3_dn21 = 0.0;
        var_t3_dn22 = 0.0;

        let assign31450_e49268: f64 = if var_t3 > 0.0 { 1.0 } else { 0.0 };
        var_guard529 = assign31450_e49268;

        let assign31460_e49271: f64 = if var_vbsl > 0.0 { 1.0 } else { 0.0 };
        var_guard530 = assign31460_e49271;

        let (assign31470_e49283, assign31470_e49283_d_n0, assign31470_e49283_d_n2, assign31470_e49283_d_n3, assign31470_e49283_d_n4, assign31470_e49283_d_n7, assign31470_e49283_d_n8, assign31470_e49283_d_n9,) = {
    if ((var_guard529 != 0.0) && (var_guard530 != 0.0)) {
        let assign31470_e49277: f64 = (var_vbsl).powf(1.0);
        let assign31470_e49280: f64 = (var_nsb_t * var_vth);
        let assign31470_e49281: f64 = (assign31470_e49277 / assign31470_e49280);
        (assign31470_e49281, 0.0, (if 0.0 == 0.0 && ((1.0) as f64).is_finite() && ((1.0) as f64).fract() == 0.0 { if 1.0 == 0.0 { 0.0 } else { ((var_vbsl).powf(1.0 - 1.0) * var_vbsl_dn2) } } else { (assign31470_e49277 * (var_vbsl_dn2 / var_vbsl)) } / assign31470_e49280), (if 0.0 == 0.0 && ((1.0) as f64).is_finite() && ((1.0) as f64).fract() == 0.0 { if 1.0 == 0.0 { 0.0 } else { ((var_vbsl).powf(1.0 - 1.0) * var_vbsl_dn3) } } else { (assign31470_e49277 * (var_vbsl_dn3 / var_vbsl)) } / assign31470_e49280), (((if 0.0 == 0.0 && ((1.0) as f64).is_finite() && ((1.0) as f64).fract() == 0.0 { if 1.0 == 0.0 { 0.0 } else { ((var_vbsl).powf(1.0 - 1.0) * var_vbsl_dn4) } } else { (assign31470_e49277 * (var_vbsl_dn4 / var_vbsl)) } * assign31470_e49280) - (assign31470_e49277 * ((var_nsb_t_dn4 * var_vth) + (var_nsb_t * var_vth_dn4)))) / (assign31470_e49280 * assign31470_e49280)), 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn7, var_arg_dn8, var_arg_dn9,)
    }
};
        var_arg = assign31470_e49283;
        var_arg_dn0 = assign31470_e49283_d_n0;
        var_arg_dn2 = assign31470_e49283_d_n2;
        var_arg_dn3 = assign31470_e49283_d_n3;
        var_arg_dn4 = assign31470_e49283_d_n4;
        var_arg_dn7 = assign31470_e49283_d_n7;
        var_arg_dn8 = assign31470_e49283_d_n8;
        var_arg_dn9 = assign31470_e49283_d_n9;

        let assign31480_e49286: f64 = if var_arg > 80.0 { 1.0 } else { 0.0 };
        var_guard531 = assign31480_e49286;

        let (assign31490_e49298, assign31490_e49298_d_n0, assign31490_e49298_d_n2, assign31490_e49298_d_n3, assign31490_e49298_d_n4, assign31490_e49298_d_n7, assign31490_e49298_d_n8, assign31490_e49298_d_n9,) = {
    if (((var_guard529 != 0.0) && (var_guard530 != 0.0)) && (var_guard531 != 0.0)) {
        let assign31490_e49295: f64 = (var_arg - 80.0);
        let assign31490_e49296: f64 = (1.0 + assign31490_e49295);
        (assign31490_e49296, var_arg_dn0, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn7, var_arg_dn8, var_arg_dn9,)
    } else {
        (var_le, var_le_dn0, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn7, var_le_dn8, var_le_dn9,)
    }
};
        var_le = assign31490_e49298;
        var_le_dn0 = assign31490_e49298_d_n0;
        var_le_dn2 = assign31490_e49298_d_n2;
        var_le_dn3 = assign31490_e49298_d_n3;
        var_le_dn4 = assign31490_e49298_d_n4;
        var_le_dn7 = assign31490_e49298_d_n7;
        var_le_dn8 = assign31490_e49298_d_n8;
        var_le_dn9 = assign31490_e49298_d_n9;

        let (assign31500_e49306, assign31500_e49306_d_n0, assign31500_e49306_d_n2, assign31500_e49306_d_n3, assign31500_e49306_d_n4, assign31500_e49306_d_n7, assign31500_e49306_d_n8, assign31500_e49306_d_n9,) = {
    if (((var_guard529 != 0.0) && (var_guard530 != 0.0)) && (var_guard531 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn7, var_arg_dn8, var_arg_dn9,)
    }
};
        var_arg = assign31500_e49306;
        var_arg_dn0 = assign31500_e49306_d_n0;
        var_arg_dn2 = assign31500_e49306_d_n2;
        var_arg_dn3 = assign31500_e49306_d_n3;
        var_arg_dn4 = assign31500_e49306_d_n4;
        var_arg_dn7 = assign31500_e49306_d_n7;
        var_arg_dn8 = assign31500_e49306_d_n8;
        var_arg_dn9 = assign31500_e49306_d_n9;

        let (assign31510_e49315, assign31510_e49315_d_n0, assign31510_e49315_d_n2, assign31510_e49315_d_n3, assign31510_e49315_d_n4, assign31510_e49315_d_n7, assign31510_e49315_d_n8, assign31510_e49315_d_n9,) = {
    if (((var_guard529 != 0.0) && (var_guard530 != 0.0)) && (var_guard531 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_le, var_le_dn0, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn7, var_le_dn8, var_le_dn9,)
    }
};
        var_le = assign31510_e49315;
        var_le_dn0 = assign31510_e49315_d_n0;
        var_le_dn2 = assign31510_e49315_d_n2;
        var_le_dn3 = assign31510_e49315_d_n3;
        var_le_dn4 = assign31510_e49315_d_n4;
        var_le_dn7 = assign31510_e49315_d_n7;
        var_le_dn8 = assign31510_e49315_d_n8;
        var_le_dn9 = assign31510_e49315_d_n9;

        let (assign31520_e49324, assign31520_e49324_d_n0, assign31520_e49324_d_n2, assign31520_e49324_d_n3, assign31520_e49324_d_n4, assign31520_e49324_d_n7, assign31520_e49324_d_n8, assign31520_e49324_d_n9,) = {
    if ((var_guard529 != 0.0) && (var_guard530 != 0.0)) {
        let assign31520_e49321: f64 = (var_arg).exp();
        let assign31520_e49322: f64 = (var_le * assign31520_e49321);
        (assign31520_e49322, ((var_le_dn0 * assign31520_e49321) + (var_le * (assign31520_e49321 * var_arg_dn0))), ((var_le_dn2 * assign31520_e49321) + (var_le * (assign31520_e49321 * var_arg_dn2))), ((var_le_dn3 * assign31520_e49321) + (var_le * (assign31520_e49321 * var_arg_dn3))), ((var_le_dn4 * assign31520_e49321) + (var_le * (assign31520_e49321 * var_arg_dn4))), ((var_le_dn7 * assign31520_e49321) + (var_le * (assign31520_e49321 * var_arg_dn7))), ((var_le_dn8 * assign31520_e49321) + (var_le * (assign31520_e49321 * var_arg_dn8))), ((var_le_dn9 * assign31520_e49321) + (var_le * (assign31520_e49321 * var_arg_dn9))),)
    } else {
        (var_le, var_le_dn0, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn7, var_le_dn8, var_le_dn9,)
    }
};
        var_le = assign31520_e49324;
        var_le_dn0 = assign31520_e49324_d_n0;
        var_le_dn2 = assign31520_e49324_d_n2;
        var_le_dn3 = assign31520_e49324_d_n3;
        var_le_dn4 = assign31520_e49324_d_n4;
        var_le_dn7 = assign31520_e49324_d_n7;
        var_le_dn8 = assign31520_e49324_d_n8;
        var_le_dn9 = assign31520_e49324_d_n9;

        let (assign31530_e49334, assign31530_e49334_d_n0, assign31530_e49334_d_n1, assign31530_e49334_d_n2, assign31530_e49334_d_n3, assign31530_e49334_d_n4, assign31530_e49334_d_n5, assign31530_e49334_d_n6, assign31530_e49334_d_n7, assign31530_e49334_d_n8, assign31530_e49334_d_n9, assign31530_e49334_d_n12, assign31530_e49334_d_n14, assign31530_e49334_d_n15, assign31530_e49334_d_n16, assign31530_e49334_d_n17, assign31530_e49334_d_n18, assign31530_e49334_d_n19, assign31530_e49334_d_n20, assign31530_e49334_d_n21, assign31530_e49334_d_n22,) = {
    if ((var_guard529 != 0.0) && (var_guard530 != 0.0)) {
        let assign31530_e49331: f64 = (var_le - 1.0);
        let assign31530_e49332: f64 = (var_t3 * assign31530_e49331);
        (assign31530_e49332, ((var_t3_dn0 * assign31530_e49331) + (var_t3 * var_le_dn0)), (var_t3_dn1 * assign31530_e49331), ((var_t3_dn2 * assign31530_e49331) + (var_t3 * var_le_dn2)), ((var_t3_dn3 * assign31530_e49331) + (var_t3 * var_le_dn3)), ((var_t3_dn4 * assign31530_e49331) + (var_t3 * var_le_dn4)), (var_t3_dn5 * assign31530_e49331), (var_t3_dn6 * assign31530_e49331), ((var_t3_dn7 * assign31530_e49331) + (var_t3 * var_le_dn7)), ((var_t3_dn8 * assign31530_e49331) + (var_t3 * var_le_dn8)), ((var_t3_dn9 * assign31530_e49331) + (var_t3 * var_le_dn9)), (var_t3_dn12 * assign31530_e49331), (var_t3_dn14 * assign31530_e49331), (var_t3_dn15 * assign31530_e49331), (var_t3_dn16 * assign31530_e49331), (var_t3_dn17 * assign31530_e49331), (var_t3_dn18 * assign31530_e49331), (var_t3_dn19 * assign31530_e49331), (var_t3_dn20 * assign31530_e49331), (var_t3_dn21 * assign31530_e49331), (var_t3_dn22 * assign31530_e49331),)
    } else {
        (var_isb, var_isb_dn0, var_isb_dn1, var_isb_dn2, var_isb_dn3, var_isb_dn4, var_isb_dn5, var_isb_dn6, var_isb_dn7, var_isb_dn8, var_isb_dn9, var_isb_dn12, var_isb_dn14, var_isb_dn15, var_isb_dn16, var_isb_dn17, var_isb_dn18, var_isb_dn19, var_isb_dn20, var_isb_dn21, var_isb_dn22,)
    }
};
        var_isb = assign31530_e49334;
        var_isb_dn0 = assign31530_e49334_d_n0;
        var_isb_dn1 = assign31530_e49334_d_n1;
        var_isb_dn2 = assign31530_e49334_d_n2;
        var_isb_dn3 = assign31530_e49334_d_n3;
        var_isb_dn4 = assign31530_e49334_d_n4;
        var_isb_dn5 = assign31530_e49334_d_n5;
        var_isb_dn6 = assign31530_e49334_d_n6;
        var_isb_dn7 = assign31530_e49334_d_n7;
        var_isb_dn8 = assign31530_e49334_d_n8;
        var_isb_dn9 = assign31530_e49334_d_n9;
        var_isb_dn12 = assign31530_e49334_d_n12;
        var_isb_dn14 = assign31530_e49334_d_n14;
        var_isb_dn15 = assign31530_e49334_d_n15;
        var_isb_dn16 = assign31530_e49334_d_n16;
        var_isb_dn17 = assign31530_e49334_d_n17;
        var_isb_dn18 = assign31530_e49334_d_n18;
        var_isb_dn19 = assign31530_e49334_d_n19;
        var_isb_dn20 = assign31530_e49334_d_n20;
        var_isb_dn21 = assign31530_e49334_d_n21;
        var_isb_dn22 = assign31530_e49334_d_n22;

        let (assign31540_e49345, assign31540_e49345_d_n0, assign31540_e49345_d_n2, assign31540_e49345_d_n3, assign31540_e49345_d_n4, assign31540_e49345_d_n7, assign31540_e49345_d_n8, assign31540_e49345_d_n9,) = {
    if ((var_guard529 != 0.0) && (var_guard530 == 0.0)) {
        let assign31540_e49342: f64 = (var_nsb_t * var_vth);
        let assign31540_e49343: f64 = (var_vbsl / assign31540_e49342);
        (assign31540_e49343, 0.0, (var_vbsl_dn2 / assign31540_e49342), (var_vbsl_dn3 / assign31540_e49342), (((var_vbsl_dn4 * assign31540_e49342) - (var_vbsl * ((var_nsb_t_dn4 * var_vth) + (var_nsb_t * var_vth_dn4)))) / (assign31540_e49342 * assign31540_e49342)), 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn7, var_arg_dn8, var_arg_dn9,)
    }
};
        var_arg = assign31540_e49345;
        var_arg_dn0 = assign31540_e49345_d_n0;
        var_arg_dn2 = assign31540_e49345_d_n2;
        var_arg_dn3 = assign31540_e49345_d_n3;
        var_arg_dn4 = assign31540_e49345_d_n4;
        var_arg_dn7 = assign31540_e49345_d_n7;
        var_arg_dn8 = assign31540_e49345_d_n8;
        var_arg_dn9 = assign31540_e49345_d_n9;

        let assign31550_e49348: f64 = if var_arg > 80.0 { 1.0 } else { 0.0 };
        var_guard532 = assign31550_e49348;

        *var_arg_slot = var_arg;
        *var_arg_dn0_slot = var_arg_dn0;
        *var_arg_dn2_slot = var_arg_dn2;
        *var_arg_dn3_slot = var_arg_dn3;
        *var_arg_dn4_slot = var_arg_dn4;
        *var_arg_dn7_slot = var_arg_dn7;
        *var_arg_dn8_slot = var_arg_dn8;
        *var_arg_dn9_slot = var_arg_dn9;
        *var_guard525_slot = var_guard525;
        *var_guard526_slot = var_guard526;
        *var_guard527_slot = var_guard527;
        *var_guard528_slot = var_guard528;
        *var_guard529_slot = var_guard529;
        *var_guard530_slot = var_guard530;
        *var_guard531_slot = var_guard531;
        *var_guard532_slot = var_guard532;
        *var_idb_slot = var_idb;
        *var_idb_dn0_slot = var_idb_dn0;
        *var_idb_dn1_slot = var_idb_dn1;
        *var_idb_dn12_slot = var_idb_dn12;
        *var_idb_dn14_slot = var_idb_dn14;
        *var_idb_dn15_slot = var_idb_dn15;
        *var_idb_dn16_slot = var_idb_dn16;
        *var_idb_dn17_slot = var_idb_dn17;
        *var_idb_dn18_slot = var_idb_dn18;
        *var_idb_dn19_slot = var_idb_dn19;
        *var_idb_dn2_slot = var_idb_dn2;
        *var_idb_dn20_slot = var_idb_dn20;
        *var_idb_dn21_slot = var_idb_dn21;
        *var_idb_dn22_slot = var_idb_dn22;
        *var_idb_dn3_slot = var_idb_dn3;
        *var_idb_dn4_slot = var_idb_dn4;
        *var_idb_dn5_slot = var_idb_dn5;
        *var_idb_dn6_slot = var_idb_dn6;
        *var_idb_dn7_slot = var_idb_dn7;
        *var_idb_dn8_slot = var_idb_dn8;
        *var_idb_dn9_slot = var_idb_dn9;
        *var_idb_t_slot = var_idb_t;
        *var_idb_t_dn4_slot = var_idb_t_dn4;
        *var_isb_slot = var_isb;
        *var_isb_dn0_slot = var_isb_dn0;
        *var_isb_dn1_slot = var_isb_dn1;
        *var_isb_dn12_slot = var_isb_dn12;
        *var_isb_dn14_slot = var_isb_dn14;
        *var_isb_dn15_slot = var_isb_dn15;
        *var_isb_dn16_slot = var_isb_dn16;
        *var_isb_dn17_slot = var_isb_dn17;
        *var_isb_dn18_slot = var_isb_dn18;
        *var_isb_dn19_slot = var_isb_dn19;
        *var_isb_dn2_slot = var_isb_dn2;
        *var_isb_dn20_slot = var_isb_dn20;
        *var_isb_dn21_slot = var_isb_dn21;
        *var_isb_dn22_slot = var_isb_dn22;
        *var_isb_dn3_slot = var_isb_dn3;
        *var_isb_dn4_slot = var_isb_dn4;
        *var_isb_dn5_slot = var_isb_dn5;
        *var_isb_dn6_slot = var_isb_dn6;
        *var_isb_dn7_slot = var_isb_dn7;
        *var_isb_dn8_slot = var_isb_dn8;
        *var_isb_dn9_slot = var_isb_dn9;
        *var_le_slot = var_le;
        *var_le_dn0_slot = var_le_dn0;
        *var_le_dn2_slot = var_le_dn2;
        *var_le_dn3_slot = var_le_dn3;
        *var_le_dn4_slot = var_le_dn4;
        *var_le_dn7_slot = var_le_dn7;
        *var_le_dn8_slot = var_le_dn8;
        *var_le_dn9_slot = var_le_dn9;
        *var_ndb_t_slot = var_ndb_t;
        *var_ndb_t_dn4_slot = var_ndb_t_dn4;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn1_slot = var_t3_dn1;
        *var_t3_dn12_slot = var_t3_dn12;
        *var_t3_dn14_slot = var_t3_dn14;
        *var_t3_dn15_slot = var_t3_dn15;
        *var_t3_dn16_slot = var_t3_dn16;
        *var_t3_dn17_slot = var_t3_dn17;
        *var_t3_dn18_slot = var_t3_dn18;
        *var_t3_dn19_slot = var_t3_dn19;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn20_slot = var_t3_dn20;
        *var_t3_dn21_slot = var_t3_dn21;
        *var_t3_dn22_slot = var_t3_dn22;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_vbdl_slot = var_vbdl;
        *var_vbdl_dn0_slot = var_vbdl_dn0;
        *var_vbdl_dn3_slot = var_vbdl_dn3;
        *var_vbdl_dn4_slot = var_vbdl_dn4;
        *var_vbsl_slot = var_vbsl;
        *var_vbsl_dn2_slot = var_vbsl_dn2;
        *var_vbsl_dn3_slot = var_vbsl_dn3;
        *var_vbsl_dn4_slot = var_vbsl_dn4;
    }

    pub(super) fn stamp_transient_block_183(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_guard529: f64,
        var_guard530: f64,
        var_guard532: f64,
        var_tdev: f64,
        var_tdev_dn4: f64,
        var_tnom: f64,
        var_vth: f64,
        var_vth_dn4: f64,
        var_arg_slot: &mut f64,
        var_arg_dn0_slot: &mut f64,
        var_arg_dn2_slot: &mut f64,
        var_arg_dn3_slot: &mut f64,
        var_arg_dn4_slot: &mut f64,
        var_arg_dn7_slot: &mut f64,
        var_arg_dn8_slot: &mut f64,
        var_arg_dn9_slot: &mut f64,
        var_guard535_slot: &mut f64,
        var_guard536_slot: &mut f64,
        var_guard537_slot: &mut f64,
        var_guard538_slot: &mut f64,
        var_guard539_slot: &mut f64,
        var_guard540_slot: &mut f64,
        var_guard541_slot: &mut f64,
        var_guard542_slot: &mut f64,
        var_guard543_slot: &mut f64,
        var_guard544_slot: &mut f64,
        var_guard545_slot: &mut f64,
        var_guard546_slot: &mut f64,
        var_guard547_slot: &mut f64,
        var_guard548_slot: &mut f64,
        var_guard549_slot: &mut f64,
        var_guard550_slot: &mut f64,
        var_guard551_slot: &mut f64,
        var_guard552_slot: &mut f64,
        var_guard553_slot: &mut f64,
        var_guard554_slot: &mut f64,
        var_guard555_slot: &mut f64,
        var_guard556_slot: &mut f64,
        var_guard557_slot: &mut f64,
        var_guard558_slot: &mut f64,
        var_guard559_slot: &mut f64,
        var_guard560_slot: &mut f64,
        var_guard561_slot: &mut f64,
        var_guard562_slot: &mut f64,
        var_guard563_slot: &mut f64,
        var_guard564_slot: &mut f64,
        var_guard565_slot: &mut f64,
        var_guard566_slot: &mut f64,
        var_guard567_slot: &mut f64,
        var_guard568_slot: &mut f64,
        var_guard569_slot: &mut f64,
        var_guard570_slot: &mut f64,
        var_guard571_slot: &mut f64,
        var_guard572_slot: &mut f64,
        var_guard573_slot: &mut f64,
        var_guard574_slot: &mut f64,
        var_guard575_slot: &mut f64,
        var_isb_slot: &mut f64,
        var_isb_dn0_slot: &mut f64,
        var_isb_dn1_slot: &mut f64,
        var_isb_dn12_slot: &mut f64,
        var_isb_dn14_slot: &mut f64,
        var_isb_dn15_slot: &mut f64,
        var_isb_dn16_slot: &mut f64,
        var_isb_dn17_slot: &mut f64,
        var_isb_dn18_slot: &mut f64,
        var_isb_dn19_slot: &mut f64,
        var_isb_dn2_slot: &mut f64,
        var_isb_dn20_slot: &mut f64,
        var_isb_dn21_slot: &mut f64,
        var_isb_dn22_slot: &mut f64,
        var_isb_dn3_slot: &mut f64,
        var_isb_dn4_slot: &mut f64,
        var_isb_dn5_slot: &mut f64,
        var_isb_dn6_slot: &mut f64,
        var_isb_dn7_slot: &mut f64,
        var_isb_dn8_slot: &mut f64,
        var_isb_dn9_slot: &mut f64,
        var_le_slot: &mut f64,
        var_le_dn0_slot: &mut f64,
        var_le_dn2_slot: &mut f64,
        var_le_dn3_slot: &mut f64,
        var_le_dn4_slot: &mut f64,
        var_le_dn7_slot: &mut f64,
        var_le_dn8_slot: &mut f64,
        var_le_dn9_slot: &mut f64,
        var_qfr_slot: &mut f64,
        var_qfr2_slot: &mut f64,
        var_qfr2_dn0_slot: &mut f64,
        var_qfr2_dn1_slot: &mut f64,
        var_qfr2_dn12_slot: &mut f64,
        var_qfr2_dn14_slot: &mut f64,
        var_qfr2_dn15_slot: &mut f64,
        var_qfr2_dn16_slot: &mut f64,
        var_qfr2_dn17_slot: &mut f64,
        var_qfr2_dn18_slot: &mut f64,
        var_qfr2_dn19_slot: &mut f64,
        var_qfr2_dn2_slot: &mut f64,
        var_qfr2_dn20_slot: &mut f64,
        var_qfr2_dn21_slot: &mut f64,
        var_qfr2_dn22_slot: &mut f64,
        var_qfr2_dn3_slot: &mut f64,
        var_qfr2_dn4_slot: &mut f64,
        var_qfr2_dn5_slot: &mut f64,
        var_qfr2_dn6_slot: &mut f64,
        var_qfr2_dn7_slot: &mut f64,
        var_qfr2_dn8_slot: &mut f64,
        var_qfr2_dn9_slot: &mut f64,
        var_qfr3_slot: &mut f64,
        var_qfr3_dn0_slot: &mut f64,
        var_qfr3_dn2_slot: &mut f64,
        var_qfr_dn0_slot: &mut f64,
        var_qfr_dn2_slot: &mut f64,
        var_qfr_dn4_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn1_slot: &mut f64,
        var_t0_dn12_slot: &mut f64,
        var_t0_dn14_slot: &mut f64,
        var_t0_dn15_slot: &mut f64,
        var_t0_dn16_slot: &mut f64,
        var_t0_dn17_slot: &mut f64,
        var_t0_dn18_slot: &mut f64,
        var_t0_dn19_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn20_slot: &mut f64,
        var_t0_dn21_slot: &mut f64,
        var_t0_dn22_slot: &mut f64,
        var_t0_dn3_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn1_slot: &mut f64,
        var_t1_dn12_slot: &mut f64,
        var_t1_dn14_slot: &mut f64,
        var_t1_dn15_slot: &mut f64,
        var_t1_dn16_slot: &mut f64,
        var_t1_dn17_slot: &mut f64,
        var_t1_dn18_slot: &mut f64,
        var_t1_dn19_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn20_slot: &mut f64,
        var_t1_dn21_slot: &mut f64,
        var_t1_dn22_slot: &mut f64,
        var_t1_dn3_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn1_slot: &mut f64,
        var_t2_dn12_slot: &mut f64,
        var_t2_dn14_slot: &mut f64,
        var_t2_dn15_slot: &mut f64,
        var_t2_dn16_slot: &mut f64,
        var_t2_dn17_slot: &mut f64,
        var_t2_dn18_slot: &mut f64,
        var_t2_dn19_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn20_slot: &mut f64,
        var_t2_dn21_slot: &mut f64,
        var_t2_dn22_slot: &mut f64,
        var_t2_dn3_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t3_slot: &mut f64,
        var_t3_dn0_slot: &mut f64,
        var_t3_dn1_slot: &mut f64,
        var_t3_dn12_slot: &mut f64,
        var_t3_dn14_slot: &mut f64,
        var_t3_dn15_slot: &mut f64,
        var_t3_dn16_slot: &mut f64,
        var_t3_dn17_slot: &mut f64,
        var_t3_dn18_slot: &mut f64,
        var_t3_dn19_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn20_slot: &mut f64,
        var_t3_dn21_slot: &mut f64,
        var_t3_dn22_slot: &mut f64,
        var_t3_dn3_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn1_slot: &mut f64,
        var_t4_dn12_slot: &mut f64,
        var_t4_dn14_slot: &mut f64,
        var_t4_dn15_slot: &mut f64,
        var_t4_dn16_slot: &mut f64,
        var_t4_dn17_slot: &mut f64,
        var_t4_dn18_slot: &mut f64,
        var_t4_dn19_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn20_slot: &mut f64,
        var_t4_dn21_slot: &mut f64,
        var_t4_dn22_slot: &mut f64,
        var_t4_dn3_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_dn9_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn1_slot: &mut f64,
        var_t6_dn12_slot: &mut f64,
        var_t6_dn14_slot: &mut f64,
        var_t6_dn15_slot: &mut f64,
        var_t6_dn16_slot: &mut f64,
        var_t6_dn17_slot: &mut f64,
        var_t6_dn18_slot: &mut f64,
        var_t6_dn19_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn20_slot: &mut f64,
        var_t6_dn21_slot: &mut f64,
        var_t6_dn22_slot: &mut f64,
        var_t6_dn3_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t6_dn9_slot: &mut f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let mut var_arg: f64 = *var_arg_slot;
        let mut var_arg_dn0: f64 = *var_arg_dn0_slot;
        let mut var_arg_dn2: f64 = *var_arg_dn2_slot;
        let mut var_arg_dn3: f64 = *var_arg_dn3_slot;
        let mut var_arg_dn4: f64 = *var_arg_dn4_slot;
        let mut var_arg_dn7: f64 = *var_arg_dn7_slot;
        let mut var_arg_dn8: f64 = *var_arg_dn8_slot;
        let mut var_arg_dn9: f64 = *var_arg_dn9_slot;
        let mut var_guard535: f64 = *var_guard535_slot;
        let mut var_guard536: f64 = *var_guard536_slot;
        let mut var_guard537: f64 = *var_guard537_slot;
        let mut var_guard538: f64 = *var_guard538_slot;
        let mut var_guard539: f64 = *var_guard539_slot;
        let mut var_guard540: f64 = *var_guard540_slot;
        let mut var_guard541: f64 = *var_guard541_slot;
        let mut var_guard542: f64 = *var_guard542_slot;
        let mut var_guard543: f64 = *var_guard543_slot;
        let mut var_guard544: f64 = *var_guard544_slot;
        let mut var_guard545: f64 = *var_guard545_slot;
        let mut var_guard546: f64 = *var_guard546_slot;
        let mut var_guard547: f64 = *var_guard547_slot;
        let mut var_guard548: f64 = *var_guard548_slot;
        let mut var_guard549: f64 = *var_guard549_slot;
        let mut var_guard550: f64 = *var_guard550_slot;
        let mut var_guard551: f64 = *var_guard551_slot;
        let mut var_guard552: f64 = *var_guard552_slot;
        let mut var_guard553: f64 = *var_guard553_slot;
        let mut var_guard554: f64 = *var_guard554_slot;
        let mut var_guard555: f64 = *var_guard555_slot;
        let mut var_guard556: f64 = *var_guard556_slot;
        let mut var_guard557: f64 = *var_guard557_slot;
        let mut var_guard558: f64 = *var_guard558_slot;
        let mut var_guard559: f64 = *var_guard559_slot;
        let mut var_guard560: f64 = *var_guard560_slot;
        let mut var_guard561: f64 = *var_guard561_slot;
        let mut var_guard562: f64 = *var_guard562_slot;
        let mut var_guard563: f64 = *var_guard563_slot;
        let mut var_guard564: f64 = *var_guard564_slot;
        let mut var_guard565: f64 = *var_guard565_slot;
        let mut var_guard566: f64 = *var_guard566_slot;
        let mut var_guard567: f64 = *var_guard567_slot;
        let mut var_guard568: f64 = *var_guard568_slot;
        let mut var_guard569: f64 = *var_guard569_slot;
        let mut var_guard570: f64 = *var_guard570_slot;
        let mut var_guard571: f64 = *var_guard571_slot;
        let mut var_guard572: f64 = *var_guard572_slot;
        let mut var_guard573: f64 = *var_guard573_slot;
        let mut var_guard574: f64 = *var_guard574_slot;
        let mut var_guard575: f64 = *var_guard575_slot;
        let mut var_isb: f64 = *var_isb_slot;
        let mut var_isb_dn0: f64 = *var_isb_dn0_slot;
        let mut var_isb_dn1: f64 = *var_isb_dn1_slot;
        let mut var_isb_dn12: f64 = *var_isb_dn12_slot;
        let mut var_isb_dn14: f64 = *var_isb_dn14_slot;
        let mut var_isb_dn15: f64 = *var_isb_dn15_slot;
        let mut var_isb_dn16: f64 = *var_isb_dn16_slot;
        let mut var_isb_dn17: f64 = *var_isb_dn17_slot;
        let mut var_isb_dn18: f64 = *var_isb_dn18_slot;
        let mut var_isb_dn19: f64 = *var_isb_dn19_slot;
        let mut var_isb_dn2: f64 = *var_isb_dn2_slot;
        let mut var_isb_dn20: f64 = *var_isb_dn20_slot;
        let mut var_isb_dn21: f64 = *var_isb_dn21_slot;
        let mut var_isb_dn22: f64 = *var_isb_dn22_slot;
        let mut var_isb_dn3: f64 = *var_isb_dn3_slot;
        let mut var_isb_dn4: f64 = *var_isb_dn4_slot;
        let mut var_isb_dn5: f64 = *var_isb_dn5_slot;
        let mut var_isb_dn6: f64 = *var_isb_dn6_slot;
        let mut var_isb_dn7: f64 = *var_isb_dn7_slot;
        let mut var_isb_dn8: f64 = *var_isb_dn8_slot;
        let mut var_isb_dn9: f64 = *var_isb_dn9_slot;
        let mut var_le: f64 = *var_le_slot;
        let mut var_le_dn0: f64 = *var_le_dn0_slot;
        let mut var_le_dn2: f64 = *var_le_dn2_slot;
        let mut var_le_dn3: f64 = *var_le_dn3_slot;
        let mut var_le_dn4: f64 = *var_le_dn4_slot;
        let mut var_le_dn7: f64 = *var_le_dn7_slot;
        let mut var_le_dn8: f64 = *var_le_dn8_slot;
        let mut var_le_dn9: f64 = *var_le_dn9_slot;
        let mut var_qfr: f64 = *var_qfr_slot;
        let mut var_qfr2: f64 = *var_qfr2_slot;
        let mut var_qfr2_dn0: f64 = *var_qfr2_dn0_slot;
        let mut var_qfr2_dn1: f64 = *var_qfr2_dn1_slot;
        let mut var_qfr2_dn12: f64 = *var_qfr2_dn12_slot;
        let mut var_qfr2_dn14: f64 = *var_qfr2_dn14_slot;
        let mut var_qfr2_dn15: f64 = *var_qfr2_dn15_slot;
        let mut var_qfr2_dn16: f64 = *var_qfr2_dn16_slot;
        let mut var_qfr2_dn17: f64 = *var_qfr2_dn17_slot;
        let mut var_qfr2_dn18: f64 = *var_qfr2_dn18_slot;
        let mut var_qfr2_dn19: f64 = *var_qfr2_dn19_slot;
        let mut var_qfr2_dn2: f64 = *var_qfr2_dn2_slot;
        let mut var_qfr2_dn20: f64 = *var_qfr2_dn20_slot;
        let mut var_qfr2_dn21: f64 = *var_qfr2_dn21_slot;
        let mut var_qfr2_dn22: f64 = *var_qfr2_dn22_slot;
        let mut var_qfr2_dn3: f64 = *var_qfr2_dn3_slot;
        let mut var_qfr2_dn4: f64 = *var_qfr2_dn4_slot;
        let mut var_qfr2_dn5: f64 = *var_qfr2_dn5_slot;
        let mut var_qfr2_dn6: f64 = *var_qfr2_dn6_slot;
        let mut var_qfr2_dn7: f64 = *var_qfr2_dn7_slot;
        let mut var_qfr2_dn8: f64 = *var_qfr2_dn8_slot;
        let mut var_qfr2_dn9: f64 = *var_qfr2_dn9_slot;
        let mut var_qfr3: f64 = *var_qfr3_slot;
        let mut var_qfr3_dn0: f64 = *var_qfr3_dn0_slot;
        let mut var_qfr3_dn2: f64 = *var_qfr3_dn2_slot;
        let mut var_qfr_dn0: f64 = *var_qfr_dn0_slot;
        let mut var_qfr_dn2: f64 = *var_qfr_dn2_slot;
        let mut var_qfr_dn4: f64 = *var_qfr_dn4_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn1: f64 = *var_t0_dn1_slot;
        let mut var_t0_dn12: f64 = *var_t0_dn12_slot;
        let mut var_t0_dn14: f64 = *var_t0_dn14_slot;
        let mut var_t0_dn15: f64 = *var_t0_dn15_slot;
        let mut var_t0_dn16: f64 = *var_t0_dn16_slot;
        let mut var_t0_dn17: f64 = *var_t0_dn17_slot;
        let mut var_t0_dn18: f64 = *var_t0_dn18_slot;
        let mut var_t0_dn19: f64 = *var_t0_dn19_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn20: f64 = *var_t0_dn20_slot;
        let mut var_t0_dn21: f64 = *var_t0_dn21_slot;
        let mut var_t0_dn22: f64 = *var_t0_dn22_slot;
        let mut var_t0_dn3: f64 = *var_t0_dn3_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn1: f64 = *var_t1_dn1_slot;
        let mut var_t1_dn12: f64 = *var_t1_dn12_slot;
        let mut var_t1_dn14: f64 = *var_t1_dn14_slot;
        let mut var_t1_dn15: f64 = *var_t1_dn15_slot;
        let mut var_t1_dn16: f64 = *var_t1_dn16_slot;
        let mut var_t1_dn17: f64 = *var_t1_dn17_slot;
        let mut var_t1_dn18: f64 = *var_t1_dn18_slot;
        let mut var_t1_dn19: f64 = *var_t1_dn19_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn20: f64 = *var_t1_dn20_slot;
        let mut var_t1_dn21: f64 = *var_t1_dn21_slot;
        let mut var_t1_dn22: f64 = *var_t1_dn22_slot;
        let mut var_t1_dn3: f64 = *var_t1_dn3_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn1: f64 = *var_t2_dn1_slot;
        let mut var_t2_dn12: f64 = *var_t2_dn12_slot;
        let mut var_t2_dn14: f64 = *var_t2_dn14_slot;
        let mut var_t2_dn15: f64 = *var_t2_dn15_slot;
        let mut var_t2_dn16: f64 = *var_t2_dn16_slot;
        let mut var_t2_dn17: f64 = *var_t2_dn17_slot;
        let mut var_t2_dn18: f64 = *var_t2_dn18_slot;
        let mut var_t2_dn19: f64 = *var_t2_dn19_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn20: f64 = *var_t2_dn20_slot;
        let mut var_t2_dn21: f64 = *var_t2_dn21_slot;
        let mut var_t2_dn22: f64 = *var_t2_dn22_slot;
        let mut var_t2_dn3: f64 = *var_t2_dn3_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t3: f64 = *var_t3_slot;
        let mut var_t3_dn0: f64 = *var_t3_dn0_slot;
        let mut var_t3_dn1: f64 = *var_t3_dn1_slot;
        let mut var_t3_dn12: f64 = *var_t3_dn12_slot;
        let mut var_t3_dn14: f64 = *var_t3_dn14_slot;
        let mut var_t3_dn15: f64 = *var_t3_dn15_slot;
        let mut var_t3_dn16: f64 = *var_t3_dn16_slot;
        let mut var_t3_dn17: f64 = *var_t3_dn17_slot;
        let mut var_t3_dn18: f64 = *var_t3_dn18_slot;
        let mut var_t3_dn19: f64 = *var_t3_dn19_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn20: f64 = *var_t3_dn20_slot;
        let mut var_t3_dn21: f64 = *var_t3_dn21_slot;
        let mut var_t3_dn22: f64 = *var_t3_dn22_slot;
        let mut var_t3_dn3: f64 = *var_t3_dn3_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn1: f64 = *var_t4_dn1_slot;
        let mut var_t4_dn12: f64 = *var_t4_dn12_slot;
        let mut var_t4_dn14: f64 = *var_t4_dn14_slot;
        let mut var_t4_dn15: f64 = *var_t4_dn15_slot;
        let mut var_t4_dn16: f64 = *var_t4_dn16_slot;
        let mut var_t4_dn17: f64 = *var_t4_dn17_slot;
        let mut var_t4_dn18: f64 = *var_t4_dn18_slot;
        let mut var_t4_dn19: f64 = *var_t4_dn19_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn20: f64 = *var_t4_dn20_slot;
        let mut var_t4_dn21: f64 = *var_t4_dn21_slot;
        let mut var_t4_dn22: f64 = *var_t4_dn22_slot;
        let mut var_t4_dn3: f64 = *var_t4_dn3_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_dn9: f64 = *var_t4_dn9_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn1: f64 = *var_t6_dn1_slot;
        let mut var_t6_dn12: f64 = *var_t6_dn12_slot;
        let mut var_t6_dn14: f64 = *var_t6_dn14_slot;
        let mut var_t6_dn15: f64 = *var_t6_dn15_slot;
        let mut var_t6_dn16: f64 = *var_t6_dn16_slot;
        let mut var_t6_dn17: f64 = *var_t6_dn17_slot;
        let mut var_t6_dn18: f64 = *var_t6_dn18_slot;
        let mut var_t6_dn19: f64 = *var_t6_dn19_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn20: f64 = *var_t6_dn20_slot;
        let mut var_t6_dn21: f64 = *var_t6_dn21_slot;
        let mut var_t6_dn22: f64 = *var_t6_dn22_slot;
        let mut var_t6_dn3: f64 = *var_t6_dn3_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t6_dn9: f64 = *var_t6_dn9_slot;

        let (assign31560_e49361, assign31560_e49361_d_n0, assign31560_e49361_d_n2, assign31560_e49361_d_n3, assign31560_e49361_d_n4, assign31560_e49361_d_n7, assign31560_e49361_d_n8, assign31560_e49361_d_n9,) = {
    if (((var_guard529 != 0.0) && (var_guard530 == 0.0)) && (var_guard532 != 0.0)) {
        let assign31560_e49358: f64 = (var_arg - 80.0);
        let assign31560_e49359: f64 = (1.0 + assign31560_e49358);
        (assign31560_e49359, var_arg_dn0, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn7, var_arg_dn8, var_arg_dn9,)
    } else {
        (var_le, var_le_dn0, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn7, var_le_dn8, var_le_dn9,)
    }
};
        var_le = assign31560_e49361;
        var_le_dn0 = assign31560_e49361_d_n0;
        var_le_dn2 = assign31560_e49361_d_n2;
        var_le_dn3 = assign31560_e49361_d_n3;
        var_le_dn4 = assign31560_e49361_d_n4;
        var_le_dn7 = assign31560_e49361_d_n7;
        var_le_dn8 = assign31560_e49361_d_n8;
        var_le_dn9 = assign31560_e49361_d_n9;

        let (assign31570_e49370, assign31570_e49370_d_n0, assign31570_e49370_d_n2, assign31570_e49370_d_n3, assign31570_e49370_d_n4, assign31570_e49370_d_n7, assign31570_e49370_d_n8, assign31570_e49370_d_n9,) = {
    if (((var_guard529 != 0.0) && (var_guard530 == 0.0)) && (var_guard532 != 0.0)) {
        (80.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn3, var_arg_dn4, var_arg_dn7, var_arg_dn8, var_arg_dn9,)
    }
};
        var_arg = assign31570_e49370;
        var_arg_dn0 = assign31570_e49370_d_n0;
        var_arg_dn2 = assign31570_e49370_d_n2;
        var_arg_dn3 = assign31570_e49370_d_n3;
        var_arg_dn4 = assign31570_e49370_d_n4;
        var_arg_dn7 = assign31570_e49370_d_n7;
        var_arg_dn8 = assign31570_e49370_d_n8;
        var_arg_dn9 = assign31570_e49370_d_n9;

        let (assign31580_e49380, assign31580_e49380_d_n0, assign31580_e49380_d_n2, assign31580_e49380_d_n3, assign31580_e49380_d_n4, assign31580_e49380_d_n7, assign31580_e49380_d_n8, assign31580_e49380_d_n9,) = {
    if (((var_guard529 != 0.0) && (var_guard530 == 0.0)) && (var_guard532 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_le, var_le_dn0, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn7, var_le_dn8, var_le_dn9,)
    }
};
        var_le = assign31580_e49380;
        var_le_dn0 = assign31580_e49380_d_n0;
        var_le_dn2 = assign31580_e49380_d_n2;
        var_le_dn3 = assign31580_e49380_d_n3;
        var_le_dn4 = assign31580_e49380_d_n4;
        var_le_dn7 = assign31580_e49380_d_n7;
        var_le_dn8 = assign31580_e49380_d_n8;
        var_le_dn9 = assign31580_e49380_d_n9;

        let (assign31590_e49390, assign31590_e49390_d_n0, assign31590_e49390_d_n2, assign31590_e49390_d_n3, assign31590_e49390_d_n4, assign31590_e49390_d_n7, assign31590_e49390_d_n8, assign31590_e49390_d_n9,) = {
    if ((var_guard529 != 0.0) && (var_guard530 == 0.0)) {
        let assign31590_e49387: f64 = (var_arg).exp();
        let assign31590_e49388: f64 = (var_le * assign31590_e49387);
        (assign31590_e49388, ((var_le_dn0 * assign31590_e49387) + (var_le * (assign31590_e49387 * var_arg_dn0))), ((var_le_dn2 * assign31590_e49387) + (var_le * (assign31590_e49387 * var_arg_dn2))), ((var_le_dn3 * assign31590_e49387) + (var_le * (assign31590_e49387 * var_arg_dn3))), ((var_le_dn4 * assign31590_e49387) + (var_le * (assign31590_e49387 * var_arg_dn4))), ((var_le_dn7 * assign31590_e49387) + (var_le * (assign31590_e49387 * var_arg_dn7))), ((var_le_dn8 * assign31590_e49387) + (var_le * (assign31590_e49387 * var_arg_dn8))), ((var_le_dn9 * assign31590_e49387) + (var_le * (assign31590_e49387 * var_arg_dn9))),)
    } else {
        (var_le, var_le_dn0, var_le_dn2, var_le_dn3, var_le_dn4, var_le_dn7, var_le_dn8, var_le_dn9,)
    }
};
        var_le = assign31590_e49390;
        var_le_dn0 = assign31590_e49390_d_n0;
        var_le_dn2 = assign31590_e49390_d_n2;
        var_le_dn3 = assign31590_e49390_d_n3;
        var_le_dn4 = assign31590_e49390_d_n4;
        var_le_dn7 = assign31590_e49390_d_n7;
        var_le_dn8 = assign31590_e49390_d_n8;
        var_le_dn9 = assign31590_e49390_d_n9;

        let (assign31600_e49401, assign31600_e49401_d_n0, assign31600_e49401_d_n1, assign31600_e49401_d_n2, assign31600_e49401_d_n3, assign31600_e49401_d_n4, assign31600_e49401_d_n5, assign31600_e49401_d_n6, assign31600_e49401_d_n7, assign31600_e49401_d_n8, assign31600_e49401_d_n9, assign31600_e49401_d_n12, assign31600_e49401_d_n14, assign31600_e49401_d_n15, assign31600_e49401_d_n16, assign31600_e49401_d_n17, assign31600_e49401_d_n18, assign31600_e49401_d_n19, assign31600_e49401_d_n20, assign31600_e49401_d_n21, assign31600_e49401_d_n22,) = {
    if ((var_guard529 != 0.0) && (var_guard530 == 0.0)) {
        let assign31600_e49398: f64 = (var_le - 1.0);
        let assign31600_e49399: f64 = (var_t3 * assign31600_e49398);
        (assign31600_e49399, ((var_t3_dn0 * assign31600_e49398) + (var_t3 * var_le_dn0)), (var_t3_dn1 * assign31600_e49398), ((var_t3_dn2 * assign31600_e49398) + (var_t3 * var_le_dn2)), ((var_t3_dn3 * assign31600_e49398) + (var_t3 * var_le_dn3)), ((var_t3_dn4 * assign31600_e49398) + (var_t3 * var_le_dn4)), (var_t3_dn5 * assign31600_e49398), (var_t3_dn6 * assign31600_e49398), ((var_t3_dn7 * assign31600_e49398) + (var_t3 * var_le_dn7)), ((var_t3_dn8 * assign31600_e49398) + (var_t3 * var_le_dn8)), ((var_t3_dn9 * assign31600_e49398) + (var_t3 * var_le_dn9)), (var_t3_dn12 * assign31600_e49398), (var_t3_dn14 * assign31600_e49398), (var_t3_dn15 * assign31600_e49398), (var_t3_dn16 * assign31600_e49398), (var_t3_dn17 * assign31600_e49398), (var_t3_dn18 * assign31600_e49398), (var_t3_dn19 * assign31600_e49398), (var_t3_dn20 * assign31600_e49398), (var_t3_dn21 * assign31600_e49398), (var_t3_dn22 * assign31600_e49398),)
    } else {
        (var_isb, var_isb_dn0, var_isb_dn1, var_isb_dn2, var_isb_dn3, var_isb_dn4, var_isb_dn5, var_isb_dn6, var_isb_dn7, var_isb_dn8, var_isb_dn9, var_isb_dn12, var_isb_dn14, var_isb_dn15, var_isb_dn16, var_isb_dn17, var_isb_dn18, var_isb_dn19, var_isb_dn20, var_isb_dn21, var_isb_dn22,)
    }
};
        var_isb = assign31600_e49401;
        var_isb_dn0 = assign31600_e49401_d_n0;
        var_isb_dn1 = assign31600_e49401_d_n1;
        var_isb_dn2 = assign31600_e49401_d_n2;
        var_isb_dn3 = assign31600_e49401_d_n3;
        var_isb_dn4 = assign31600_e49401_d_n4;
        var_isb_dn5 = assign31600_e49401_d_n5;
        var_isb_dn6 = assign31600_e49401_d_n6;
        var_isb_dn7 = assign31600_e49401_d_n7;
        var_isb_dn8 = assign31600_e49401_d_n8;
        var_isb_dn9 = assign31600_e49401_d_n9;
        var_isb_dn12 = assign31600_e49401_d_n12;
        var_isb_dn14 = assign31600_e49401_d_n14;
        var_isb_dn15 = assign31600_e49401_d_n15;
        var_isb_dn16 = assign31600_e49401_d_n16;
        var_isb_dn17 = assign31600_e49401_d_n17;
        var_isb_dn18 = assign31600_e49401_d_n18;
        var_isb_dn19 = assign31600_e49401_d_n19;
        var_isb_dn20 = assign31600_e49401_d_n20;
        var_isb_dn21 = assign31600_e49401_d_n21;
        var_isb_dn22 = assign31600_e49401_d_n22;

        let (assign31610_e49406, assign31610_e49406_d_n0, assign31610_e49406_d_n1, assign31610_e49406_d_n2, assign31610_e49406_d_n3, assign31610_e49406_d_n4, assign31610_e49406_d_n5, assign31610_e49406_d_n6, assign31610_e49406_d_n7, assign31610_e49406_d_n8, assign31610_e49406_d_n9, assign31610_e49406_d_n12, assign31610_e49406_d_n14, assign31610_e49406_d_n15, assign31610_e49406_d_n16, assign31610_e49406_d_n17, assign31610_e49406_d_n18, assign31610_e49406_d_n19, assign31610_e49406_d_n20, assign31610_e49406_d_n21, assign31610_e49406_d_n22,) = {
    if (var_guard529 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isb, var_isb_dn0, var_isb_dn1, var_isb_dn2, var_isb_dn3, var_isb_dn4, var_isb_dn5, var_isb_dn6, var_isb_dn7, var_isb_dn8, var_isb_dn9, var_isb_dn12, var_isb_dn14, var_isb_dn15, var_isb_dn16, var_isb_dn17, var_isb_dn18, var_isb_dn19, var_isb_dn20, var_isb_dn21, var_isb_dn22,)
    }
};
        var_isb = assign31610_e49406;
        var_isb_dn0 = assign31610_e49406_d_n0;
        var_isb_dn1 = assign31610_e49406_d_n1;
        var_isb_dn2 = assign31610_e49406_d_n2;
        var_isb_dn3 = assign31610_e49406_d_n3;
        var_isb_dn4 = assign31610_e49406_d_n4;
        var_isb_dn5 = assign31610_e49406_d_n5;
        var_isb_dn6 = assign31610_e49406_d_n6;
        var_isb_dn7 = assign31610_e49406_d_n7;
        var_isb_dn8 = assign31610_e49406_d_n8;
        var_isb_dn9 = assign31610_e49406_d_n9;
        var_isb_dn12 = assign31610_e49406_d_n12;
        var_isb_dn14 = assign31610_e49406_d_n14;
        var_isb_dn15 = assign31610_e49406_d_n15;
        var_isb_dn16 = assign31610_e49406_d_n16;
        var_isb_dn17 = assign31610_e49406_d_n17;
        var_isb_dn18 = assign31610_e49406_d_n18;
        var_isb_dn19 = assign31610_e49406_d_n19;
        var_isb_dn20 = assign31610_e49406_d_n20;
        var_isb_dn21 = assign31610_e49406_d_n21;
        var_isb_dn22 = assign31610_e49406_d_n22;

        let assign31720_e49545: f64 = if p.p255 == 2.0 { 1.0 } else { 0.0 };
        var_guard535 = assign31720_e49545;

        let assign31730_e49548: f64 = if p.p149 == 0.0 { 1.0 } else { 0.0 };
        var_guard536 = assign31730_e49548;

        let assign31740_e49551: f64 = if p.p150 != 0.0 { 1.0 } else { 0.0 };
        var_guard537 = assign31740_e49551;

        let assign31750_e49554: f64 = if p.p150 == 1.0 { 1.0 } else { 0.0 };
        var_guard538 = assign31750_e49554;

        let assign31760_e49557: f64 = if p.p150 != 0.0 { 1.0 } else { 0.0 };
        var_guard539 = assign31760_e49557;

        let assign31770_e49560: f64 = if p.p150 == 1.0 { 1.0 } else { 0.0 };
        var_guard540 = assign31770_e49560;

        let assign31780_e49563: f64 = if p.p149 == 0.0 { 1.0 } else { 0.0 };
        var_guard541 = assign31780_e49563;

        let assign31790_e49566: f64 = if p.p151 != 0.0 { 1.0 } else { 0.0 };
        var_guard542 = assign31790_e49566;

        let assign31800_e49569: f64 = if p.p151 == 1.0 { 1.0 } else { 0.0 };
        var_guard543 = assign31800_e49569;

        let assign31810_e49572: f64 = if p.p151 != 0.0 { 1.0 } else { 0.0 };
        var_guard544 = assign31810_e49572;

        let assign31820_e49575: f64 = if p.p151 == 1.0 { 1.0 } else { 0.0 };
        var_guard545 = assign31820_e49575;

        let assign31830_e49578: f64 = if p.p149 == 0.0 { 1.0 } else { 0.0 };
        var_guard546 = assign31830_e49578;

        let assign31840_e49581: f64 = if p.p152 != 0.0 { 1.0 } else { 0.0 };
        var_guard547 = assign31840_e49581;

        let assign31850_e49584: f64 = if p.p152 == 1.0 { 1.0 } else { 0.0 };
        var_guard548 = assign31850_e49584;

        let assign31860_e49587: f64 = if p.p152 != 0.0 { 1.0 } else { 0.0 };
        var_guard549 = assign31860_e49587;

        let assign31870_e49590: f64 = if p.p152 == 1.0 { 1.0 } else { 0.0 };
        var_guard550 = assign31870_e49590;

        let assign31880_e49593: f64 = if p.p149 == 0.0 { 1.0 } else { 0.0 };
        var_guard551 = assign31880_e49593;

        let assign31890_e49596: f64 = if p.p153 != 0.0 { 1.0 } else { 0.0 };
        var_guard552 = assign31890_e49596;

        let assign31900_e49599: f64 = if p.p153 == 1.0 { 1.0 } else { 0.0 };
        var_guard553 = assign31900_e49599;

        let assign31910_e49602: f64 = if p.p153 != 0.0 { 1.0 } else { 0.0 };
        var_guard554 = assign31910_e49602;

        let assign31920_e49605: f64 = if p.p153 == 1.0 { 1.0 } else { 0.0 };
        var_guard555 = assign31920_e49605;

        let assign31930_e49608: f64 = if p.p149 == 0.0 { 1.0 } else { 0.0 };
        var_guard556 = assign31930_e49608;

        let assign31940_e49611: f64 = if p.p154 != 0.0 { 1.0 } else { 0.0 };
        var_guard557 = assign31940_e49611;

        let assign31950_e49614: f64 = if p.p154 == 1.0 { 1.0 } else { 0.0 };
        var_guard558 = assign31950_e49614;

        let assign31960_e49617: f64 = if p.p154 != 0.0 { 1.0 } else { 0.0 };
        var_guard559 = assign31960_e49617;

        let assign31970_e49620: f64 = if p.p154 == 1.0 { 1.0 } else { 0.0 };
        var_guard560 = assign31970_e49620;

        let assign31980_e49623: f64 = if p.p149 == 0.0 { 1.0 } else { 0.0 };
        var_guard561 = assign31980_e49623;

        let assign31990_e49626: f64 = if p.p155 != 0.0 { 1.0 } else { 0.0 };
        var_guard562 = assign31990_e49626;

        let assign32000_e49629: f64 = if p.p155 == 1.0 { 1.0 } else { 0.0 };
        var_guard563 = assign32000_e49629;

        let assign32010_e49632: f64 = if p.p155 != 0.0 { 1.0 } else { 0.0 };
        var_guard564 = assign32010_e49632;

        let assign32020_e49635: f64 = if p.p155 == 1.0 { 1.0 } else { 0.0 };
        var_guard565 = assign32020_e49635;

        let assign32030_e49638: f64 = if p.p149 == 0.0 { 1.0 } else { 0.0 };
        var_guard566 = assign32030_e49638;

        let assign32040_e49641: f64 = if p.p156 != 0.0 { 1.0 } else { 0.0 };
        var_guard567 = assign32040_e49641;

        let assign32050_e49644: f64 = if p.p156 == 1.0 { 1.0 } else { 0.0 };
        var_guard568 = assign32050_e49644;

        let assign32060_e49647: f64 = if p.p156 != 0.0 { 1.0 } else { 0.0 };
        var_guard569 = assign32060_e49647;

        let assign32070_e49650: f64 = if p.p156 == 1.0 { 1.0 } else { 0.0 };
        var_guard570 = assign32070_e49650;

        let assign32080_e49653: f64 = if p.p149 == 0.0 { 1.0 } else { 0.0 };
        var_guard571 = assign32080_e49653;

        let assign32090_e49656: f64 = if p.p157 != 0.0 { 1.0 } else { 0.0 };
        var_guard572 = assign32090_e49656;

        let assign32100_e49659: f64 = if p.p157 == 1.0 { 1.0 } else { 0.0 };
        var_guard573 = assign32100_e49659;

        let assign32110_e49662: f64 = if p.p157 != 0.0 { 1.0 } else { 0.0 };
        var_guard574 = assign32110_e49662;

        let assign32120_e49665: f64 = if p.p157 == 1.0 { 1.0 } else { 0.0 };
        var_guard575 = assign32120_e49665;

        let assign32130_e49670: f64 = (var_tdev / var_tnom);
        let assign32130_e49672: f64 = (assign32130_e49670 - 1.0);
        let assign32130_e49674: f64 = (assign32130_e49672 * p.p227);
        let assign32130_e49675: f64 = (p.p220 + assign32130_e49674);
        let assign32130_e49677: f64 = (assign32130_e49675 * (nv0 - nv2));
        let assign32130_e49678: f64 = (p.p222 - assign32130_e49677);
        var_qfr = assign32130_e49678;
        var_qfr_dn0 = (-assign32130_e49675);
        var_qfr_dn2 = (-(-assign32130_e49675));
        var_qfr_dn4 = (-(((var_tdev_dn4 / var_tnom) * p.p227) * (nv0 - nv2)));

        let assign32140_e49681: f64 = (p.p4 * p.p5);
        let assign32140_e49684: f64 = (1e-25 + var_qfr);
        let assign32140_e49688: f64 = (1e-25 + var_qfr);
        let assign32140_e49691: f64 = (var_qfr - 1e-25);
        let assign32140_e49694: f64 = (var_qfr - 1e-25);
        let assign32140_e49695: f64 = (assign32140_e49691 * assign32140_e49694);
        let assign32140_e49697: f64 = (assign32140_e49695 + p.p221);
        let assign32140_e49698: f64 = (assign32140_e49697).sqrt();
        let assign32140_e49699: f64 = (assign32140_e49688 - assign32140_e49698);
        let assign32140_e49700: f64 = (0.5 * assign32140_e49699);
        let assign32140_e49701: f64 = (assign32140_e49684 - assign32140_e49700);
        let assign32140_e49702: f64 = (assign32140_e49681 * assign32140_e49701);
        var_qfr = assign32140_e49702;
        var_qfr_dn0 = (assign32140_e49681 * (var_qfr_dn0 - (0.5 * (var_qfr_dn0 - (((var_qfr_dn0 * assign32140_e49694) + (assign32140_e49691 * var_qfr_dn0)) / (2.0 * assign32140_e49698))))));
        var_qfr_dn2 = (assign32140_e49681 * (var_qfr_dn2 - (0.5 * (var_qfr_dn2 - (((var_qfr_dn2 * assign32140_e49694) + (assign32140_e49691 * var_qfr_dn2)) / (2.0 * assign32140_e49698))))));
        var_qfr_dn4 = (assign32140_e49681 * (var_qfr_dn4 - (0.5 * (var_qfr_dn4 - (((var_qfr_dn4 * assign32140_e49694) + (assign32140_e49691 * var_qfr_dn4)) / (2.0 * assign32140_e49698))))));

        let assign32150_e49707: f64 = (var_tdev / var_tnom);
        let assign32150_e49709: f64 = (assign32150_e49707 - 1.0);
        let assign32150_e49711: f64 = (assign32150_e49709 * p.p226);
        let assign32150_e49712: f64 = (p.p218 - assign32150_e49711);
        let assign32150_e49714: f64 = (assign32150_e49712 + 1e-18);
        let assign32150_e49718: f64 = (var_tdev / var_tnom);
        let assign32150_e49720: f64 = (assign32150_e49718 - 1.0);
        let assign32150_e49722: f64 = (assign32150_e49720 * p.p226);
        let assign32150_e49723: f64 = (p.p218 - assign32150_e49722);
        let assign32150_e49725: f64 = (assign32150_e49723 - 1e-18);
        let assign32150_e49729: f64 = (var_tdev / var_tnom);
        let assign32150_e49731: f64 = (assign32150_e49729 - 1.0);
        let assign32150_e49733: f64 = (assign32150_e49731 * p.p226);
        let assign32150_e49734: f64 = (p.p218 - assign32150_e49733);
        let assign32150_e49736: f64 = (assign32150_e49734 - 1e-18);
        let assign32150_e49737: f64 = (assign32150_e49725 * assign32150_e49736);
        let assign32150_e49740: f64 = (0.25 * 1e-19);
        let assign32150_e49742: f64 = (assign32150_e49740 * 1e-19);
        let assign32150_e49743: f64 = (assign32150_e49737 + assign32150_e49742);
        let assign32150_e49744: f64 = (assign32150_e49743).sqrt();
        let assign32150_e49745: f64 = (assign32150_e49714 + assign32150_e49744);
        let assign32150_e49746: f64 = (0.5 * assign32150_e49745);
        var_t0 = assign32150_e49746;
        var_t0_dn0 = 0.0;
        var_t0_dn1 = 0.0;
        var_t0_dn2 = 0.0;
        var_t0_dn3 = 0.0;
        var_t0_dn4 = (0.5 * ((-((var_tdev_dn4 / var_tnom) * p.p226)) + ((((-((var_tdev_dn4 / var_tnom) * p.p226)) * assign32150_e49736) + (assign32150_e49725 * (-((var_tdev_dn4 / var_tnom) * p.p226)))) / (2.0 * assign32150_e49744))));
        var_t0_dn5 = 0.0;
        var_t0_dn6 = 0.0;
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;
        var_t0_dn9 = 0.0;
        var_t0_dn12 = 0.0;
        var_t0_dn14 = 0.0;
        var_t0_dn15 = 0.0;
        var_t0_dn16 = 0.0;
        var_t0_dn17 = 0.0;
        var_t0_dn18 = 0.0;
        var_t0_dn19 = 0.0;
        var_t0_dn20 = 0.0;
        var_t0_dn21 = 0.0;
        var_t0_dn22 = 0.0;

        let assign32160_e49749: f64 = (p.p4 * p.p5);
        let assign32160_e49751: f64 = (assign32160_e49749 * var_t0);
        let assign32160_e49753: f64 = (assign32160_e49751 * (nv9 - nv2));
        var_qfr2 = assign32160_e49753;
        var_qfr2_dn0 = ((assign32160_e49749 * var_t0_dn0) * (nv9 - nv2));
        var_qfr2_dn1 = ((assign32160_e49749 * var_t0_dn1) * (nv9 - nv2));
        var_qfr2_dn2 = (((assign32160_e49749 * var_t0_dn2) * (nv9 - nv2)) + (-assign32160_e49751));
        var_qfr2_dn3 = ((assign32160_e49749 * var_t0_dn3) * (nv9 - nv2));
        var_qfr2_dn4 = ((assign32160_e49749 * var_t0_dn4) * (nv9 - nv2));
        var_qfr2_dn5 = ((assign32160_e49749 * var_t0_dn5) * (nv9 - nv2));
        var_qfr2_dn6 = ((assign32160_e49749 * var_t0_dn6) * (nv9 - nv2));
        var_qfr2_dn7 = ((assign32160_e49749 * var_t0_dn7) * (nv9 - nv2));
        var_qfr2_dn8 = ((assign32160_e49749 * var_t0_dn8) * (nv9 - nv2));
        var_qfr2_dn9 = (((assign32160_e49749 * var_t0_dn9) * (nv9 - nv2)) + assign32160_e49751);
        var_qfr2_dn12 = ((assign32160_e49749 * var_t0_dn12) * (nv9 - nv2));
        var_qfr2_dn14 = ((assign32160_e49749 * var_t0_dn14) * (nv9 - nv2));
        var_qfr2_dn15 = ((assign32160_e49749 * var_t0_dn15) * (nv9 - nv2));
        var_qfr2_dn16 = ((assign32160_e49749 * var_t0_dn16) * (nv9 - nv2));
        var_qfr2_dn17 = ((assign32160_e49749 * var_t0_dn17) * (nv9 - nv2));
        var_qfr2_dn18 = ((assign32160_e49749 * var_t0_dn18) * (nv9 - nv2));
        var_qfr2_dn19 = ((assign32160_e49749 * var_t0_dn19) * (nv9 - nv2));
        var_qfr2_dn20 = ((assign32160_e49749 * var_t0_dn20) * (nv9 - nv2));
        var_qfr2_dn21 = ((assign32160_e49749 * var_t0_dn21) * (nv9 - nv2));
        var_qfr2_dn22 = ((assign32160_e49749 * var_t0_dn22) * (nv9 - nv2));

        let assign32170_e49756: f64 = (p.p4 * p.p5);
        let assign32170_e49758: f64 = (assign32170_e49756 * p.p219);
        let assign32170_e49760: f64 = (assign32170_e49758 * (nv2 - nv0));
        var_qfr3 = assign32170_e49760;
        var_qfr3_dn0 = (-assign32170_e49758);
        var_qfr3_dn2 = assign32170_e49758;

        let assign32180_e49764: f64 = (var_tdev / var_tnom);
        let assign32180_e49766: f64 = (assign32180_e49764 - 1.0);
        let assign32180_e49768: f64 = (assign32180_e49766 * p.p225);
        let assign32180_e49769: f64 = (p.p224 - assign32180_e49768);
        let assign32180_e49772: f64 = (p.p229).ln();
        let assign32180_e49773: f64 = (-assign32180_e49772);
        let assign32180_e49775: f64 = (assign32180_e49773 / p.p228);
        let assign32180_e49776: f64 = { let limited_exp_arg = assign32180_e49775; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign32180_e49777: f64 = (1.0 - assign32180_e49776);
        let assign32180_e49778: f64 = (assign32180_e49769 * assign32180_e49777);
        var_t0 = assign32180_e49778;
        var_t0_dn0 = 0.0;
        var_t0_dn1 = 0.0;
        var_t0_dn2 = 0.0;
        var_t0_dn3 = 0.0;
        var_t0_dn4 = ((-((var_tdev_dn4 / var_tnom) * p.p225)) * assign32180_e49777);
        var_t0_dn5 = 0.0;
        var_t0_dn6 = 0.0;
        var_t0_dn7 = 0.0;
        var_t0_dn8 = 0.0;
        var_t0_dn9 = 0.0;
        var_t0_dn12 = 0.0;
        var_t0_dn14 = 0.0;
        var_t0_dn15 = 0.0;
        var_t0_dn16 = 0.0;
        var_t0_dn17 = 0.0;
        var_t0_dn18 = 0.0;
        var_t0_dn19 = 0.0;
        var_t0_dn20 = 0.0;
        var_t0_dn21 = 0.0;
        var_t0_dn22 = 0.0;

        let assign32190_e49781: f64 = (var_t0 - (nv2 - nv0));
        let assign32190_e49783: f64 = (assign32190_e49781 / var_vth);
        var_t1 = assign32190_e49783;
        var_t1_dn0 = ((var_t0_dn0 - -1.0) / var_vth);
        var_t1_dn1 = (var_t0_dn1 / var_vth);
        var_t1_dn2 = ((var_t0_dn2 - 1.0) / var_vth);
        var_t1_dn3 = (var_t0_dn3 / var_vth);
        var_t1_dn4 = (((var_t0_dn4 * var_vth) - (assign32190_e49781 * var_vth_dn4)) / (var_vth * var_vth));
        var_t1_dn5 = (var_t0_dn5 / var_vth);
        var_t1_dn6 = (var_t0_dn6 / var_vth);
        var_t1_dn7 = (var_t0_dn7 / var_vth);
        var_t1_dn8 = (var_t0_dn8 / var_vth);
        var_t1_dn9 = (var_t0_dn9 / var_vth);
        var_t1_dn12 = (var_t0_dn12 / var_vth);
        var_t1_dn14 = (var_t0_dn14 / var_vth);
        var_t1_dn15 = (var_t0_dn15 / var_vth);
        var_t1_dn16 = (var_t0_dn16 / var_vth);
        var_t1_dn17 = (var_t0_dn17 / var_vth);
        var_t1_dn18 = (var_t0_dn18 / var_vth);
        var_t1_dn19 = (var_t0_dn19 / var_vth);
        var_t1_dn20 = (var_t0_dn20 / var_vth);
        var_t1_dn21 = (var_t0_dn21 / var_vth);
        var_t1_dn22 = (var_t0_dn22 / var_vth);

        let assign32200_e49786: f64 = (p.p230 * var_t1);
        let assign32200_e49788: f64 = (assign32200_e49786 * var_t1);
        let assign32200_e49790: f64 = (assign32200_e49788 + 1.92);
        let assign32200_e49791: f64 = (assign32200_e49790).sqrt();
        var_t2 = assign32200_e49791;
        var_t2_dn0 = ((((p.p230 * var_t1_dn0) * var_t1) + (assign32200_e49786 * var_t1_dn0)) / (2.0 * assign32200_e49791));
        var_t2_dn1 = ((((p.p230 * var_t1_dn1) * var_t1) + (assign32200_e49786 * var_t1_dn1)) / (2.0 * assign32200_e49791));
        var_t2_dn2 = ((((p.p230 * var_t1_dn2) * var_t1) + (assign32200_e49786 * var_t1_dn2)) / (2.0 * assign32200_e49791));
        var_t2_dn3 = ((((p.p230 * var_t1_dn3) * var_t1) + (assign32200_e49786 * var_t1_dn3)) / (2.0 * assign32200_e49791));
        var_t2_dn4 = ((((p.p230 * var_t1_dn4) * var_t1) + (assign32200_e49786 * var_t1_dn4)) / (2.0 * assign32200_e49791));
        var_t2_dn5 = ((((p.p230 * var_t1_dn5) * var_t1) + (assign32200_e49786 * var_t1_dn5)) / (2.0 * assign32200_e49791));
        var_t2_dn6 = ((((p.p230 * var_t1_dn6) * var_t1) + (assign32200_e49786 * var_t1_dn6)) / (2.0 * assign32200_e49791));
        var_t2_dn7 = ((((p.p230 * var_t1_dn7) * var_t1) + (assign32200_e49786 * var_t1_dn7)) / (2.0 * assign32200_e49791));
        var_t2_dn8 = ((((p.p230 * var_t1_dn8) * var_t1) + (assign32200_e49786 * var_t1_dn8)) / (2.0 * assign32200_e49791));
        var_t2_dn9 = ((((p.p230 * var_t1_dn9) * var_t1) + (assign32200_e49786 * var_t1_dn9)) / (2.0 * assign32200_e49791));
        var_t2_dn12 = ((((p.p230 * var_t1_dn12) * var_t1) + (assign32200_e49786 * var_t1_dn12)) / (2.0 * assign32200_e49791));
        var_t2_dn14 = ((((p.p230 * var_t1_dn14) * var_t1) + (assign32200_e49786 * var_t1_dn14)) / (2.0 * assign32200_e49791));
        var_t2_dn15 = ((((p.p230 * var_t1_dn15) * var_t1) + (assign32200_e49786 * var_t1_dn15)) / (2.0 * assign32200_e49791));
        var_t2_dn16 = ((((p.p230 * var_t1_dn16) * var_t1) + (assign32200_e49786 * var_t1_dn16)) / (2.0 * assign32200_e49791));
        var_t2_dn17 = ((((p.p230 * var_t1_dn17) * var_t1) + (assign32200_e49786 * var_t1_dn17)) / (2.0 * assign32200_e49791));
        var_t2_dn18 = ((((p.p230 * var_t1_dn18) * var_t1) + (assign32200_e49786 * var_t1_dn18)) / (2.0 * assign32200_e49791));
        var_t2_dn19 = ((((p.p230 * var_t1_dn19) * var_t1) + (assign32200_e49786 * var_t1_dn19)) / (2.0 * assign32200_e49791));
        var_t2_dn20 = ((((p.p230 * var_t1_dn20) * var_t1) + (assign32200_e49786 * var_t1_dn20)) / (2.0 * assign32200_e49791));
        var_t2_dn21 = ((((p.p230 * var_t1_dn21) * var_t1) + (assign32200_e49786 * var_t1_dn21)) / (2.0 * assign32200_e49791));
        var_t2_dn22 = ((((p.p230 * var_t1_dn22) * var_t1) + (assign32200_e49786 * var_t1_dn22)) / (2.0 * assign32200_e49791));

        let assign32210_e49794: f64 = (var_t1 + var_t2);
        let assign32210_e49796: f64 = (assign32210_e49794 * 0.5);
        var_t3 = assign32210_e49796;
        var_t3_dn0 = ((var_t1_dn0 + var_t2_dn0) * 0.5);
        var_t3_dn1 = ((var_t1_dn1 + var_t2_dn1) * 0.5);
        var_t3_dn2 = ((var_t1_dn2 + var_t2_dn2) * 0.5);
        var_t3_dn3 = ((var_t1_dn3 + var_t2_dn3) * 0.5);
        var_t3_dn4 = ((var_t1_dn4 + var_t2_dn4) * 0.5);
        var_t3_dn5 = ((var_t1_dn5 + var_t2_dn5) * 0.5);
        var_t3_dn6 = ((var_t1_dn6 + var_t2_dn6) * 0.5);
        var_t3_dn7 = ((var_t1_dn7 + var_t2_dn7) * 0.5);
        var_t3_dn8 = ((var_t1_dn8 + var_t2_dn8) * 0.5);
        var_t3_dn9 = ((var_t1_dn9 + var_t2_dn9) * 0.5);
        var_t3_dn12 = ((var_t1_dn12 + var_t2_dn12) * 0.5);
        var_t3_dn14 = ((var_t1_dn14 + var_t2_dn14) * 0.5);
        var_t3_dn15 = ((var_t1_dn15 + var_t2_dn15) * 0.5);
        var_t3_dn16 = ((var_t1_dn16 + var_t2_dn16) * 0.5);
        var_t3_dn17 = ((var_t1_dn17 + var_t2_dn17) * 0.5);
        var_t3_dn18 = ((var_t1_dn18 + var_t2_dn18) * 0.5);
        var_t3_dn19 = ((var_t1_dn19 + var_t2_dn19) * 0.5);
        var_t3_dn20 = ((var_t1_dn20 + var_t2_dn20) * 0.5);
        var_t3_dn21 = ((var_t1_dn21 + var_t2_dn21) * 0.5);
        var_t3_dn22 = ((var_t1_dn22 + var_t2_dn22) * 0.5);

        let assign32220_e49800: f64 = (var_vth * var_t3);
        let assign32220_e49801: f64 = (var_t0 - assign32220_e49800);
        var_t4 = assign32220_e49801;
        var_t4_dn0 = (var_t0_dn0 - (var_vth * var_t3_dn0));
        var_t4_dn1 = (var_t0_dn1 - (var_vth * var_t3_dn1));
        var_t4_dn2 = (var_t0_dn2 - (var_vth * var_t3_dn2));
        var_t4_dn3 = (var_t0_dn3 - (var_vth * var_t3_dn3));
        var_t4_dn4 = (var_t0_dn4 - ((var_vth_dn4 * var_t3) + (var_vth * var_t3_dn4)));
        var_t4_dn5 = (var_t0_dn5 - (var_vth * var_t3_dn5));
        var_t4_dn6 = (var_t0_dn6 - (var_vth * var_t3_dn6));
        var_t4_dn7 = (var_t0_dn7 - (var_vth * var_t3_dn7));
        var_t4_dn8 = (var_t0_dn8 - (var_vth * var_t3_dn8));
        var_t4_dn9 = (var_t0_dn9 - (var_vth * var_t3_dn9));
        var_t4_dn12 = (var_t0_dn12 - (var_vth * var_t3_dn12));
        var_t4_dn14 = (var_t0_dn14 - (var_vth * var_t3_dn14));
        var_t4_dn15 = (var_t0_dn15 - (var_vth * var_t3_dn15));
        var_t4_dn16 = (var_t0_dn16 - (var_vth * var_t3_dn16));
        var_t4_dn17 = (var_t0_dn17 - (var_vth * var_t3_dn17));
        var_t4_dn18 = (var_t0_dn18 - (var_vth * var_t3_dn18));
        var_t4_dn19 = (var_t0_dn19 - (var_vth * var_t3_dn19));
        var_t4_dn20 = (var_t0_dn20 - (var_vth * var_t3_dn20));
        var_t4_dn21 = (var_t0_dn21 - (var_vth * var_t3_dn21));
        var_t4_dn22 = (var_t0_dn22 - (var_vth * var_t3_dn22));

        let assign32230_e49805: f64 = (var_t4 / p.p224);
        let assign32230_e49806: f64 = (1.0 - assign32230_e49805);
        let assign32230_e49807: f64 = (assign32230_e49806).ln();
        var_t6 = assign32230_e49807;
        var_t6_dn0 = ((-(var_t4_dn0 / p.p224)) / assign32230_e49806);
        var_t6_dn1 = ((-(var_t4_dn1 / p.p224)) / assign32230_e49806);
        var_t6_dn2 = ((-(var_t4_dn2 / p.p224)) / assign32230_e49806);
        var_t6_dn3 = ((-(var_t4_dn3 / p.p224)) / assign32230_e49806);
        var_t6_dn4 = ((-(var_t4_dn4 / p.p224)) / assign32230_e49806);
        var_t6_dn5 = ((-(var_t4_dn5 / p.p224)) / assign32230_e49806);
        var_t6_dn6 = ((-(var_t4_dn6 / p.p224)) / assign32230_e49806);
        var_t6_dn7 = ((-(var_t4_dn7 / p.p224)) / assign32230_e49806);
        var_t6_dn8 = ((-(var_t4_dn8 / p.p224)) / assign32230_e49806);
        var_t6_dn9 = ((-(var_t4_dn9 / p.p224)) / assign32230_e49806);
        var_t6_dn12 = ((-(var_t4_dn12 / p.p224)) / assign32230_e49806);
        var_t6_dn14 = ((-(var_t4_dn14 / p.p224)) / assign32230_e49806);
        var_t6_dn15 = ((-(var_t4_dn15 / p.p224)) / assign32230_e49806);
        var_t6_dn16 = ((-(var_t4_dn16 / p.p224)) / assign32230_e49806);
        var_t6_dn17 = ((-(var_t4_dn17 / p.p224)) / assign32230_e49806);
        var_t6_dn18 = ((-(var_t4_dn18 / p.p224)) / assign32230_e49806);
        var_t6_dn19 = ((-(var_t4_dn19 / p.p224)) / assign32230_e49806);
        var_t6_dn20 = ((-(var_t4_dn20 / p.p224)) / assign32230_e49806);
        var_t6_dn21 = ((-(var_t4_dn21 / p.p224)) / assign32230_e49806);
        var_t6_dn22 = ((-(var_t4_dn22 / p.p224)) / assign32230_e49806);

        *var_arg_slot = var_arg;
        *var_arg_dn0_slot = var_arg_dn0;
        *var_arg_dn2_slot = var_arg_dn2;
        *var_arg_dn3_slot = var_arg_dn3;
        *var_arg_dn4_slot = var_arg_dn4;
        *var_arg_dn7_slot = var_arg_dn7;
        *var_arg_dn8_slot = var_arg_dn8;
        *var_arg_dn9_slot = var_arg_dn9;
        *var_guard535_slot = var_guard535;
        *var_guard536_slot = var_guard536;
        *var_guard537_slot = var_guard537;
        *var_guard538_slot = var_guard538;
        *var_guard539_slot = var_guard539;
        *var_guard540_slot = var_guard540;
        *var_guard541_slot = var_guard541;
        *var_guard542_slot = var_guard542;
        *var_guard543_slot = var_guard543;
        *var_guard544_slot = var_guard544;
        *var_guard545_slot = var_guard545;
        *var_guard546_slot = var_guard546;
        *var_guard547_slot = var_guard547;
        *var_guard548_slot = var_guard548;
        *var_guard549_slot = var_guard549;
        *var_guard550_slot = var_guard550;
        *var_guard551_slot = var_guard551;
        *var_guard552_slot = var_guard552;
        *var_guard553_slot = var_guard553;
        *var_guard554_slot = var_guard554;
        *var_guard555_slot = var_guard555;
        *var_guard556_slot = var_guard556;
        *var_guard557_slot = var_guard557;
        *var_guard558_slot = var_guard558;
        *var_guard559_slot = var_guard559;
        *var_guard560_slot = var_guard560;
        *var_guard561_slot = var_guard561;
        *var_guard562_slot = var_guard562;
        *var_guard563_slot = var_guard563;
        *var_guard564_slot = var_guard564;
        *var_guard565_slot = var_guard565;
        *var_guard566_slot = var_guard566;
        *var_guard567_slot = var_guard567;
        *var_guard568_slot = var_guard568;
        *var_guard569_slot = var_guard569;
        *var_guard570_slot = var_guard570;
        *var_guard571_slot = var_guard571;
        *var_guard572_slot = var_guard572;
        *var_guard573_slot = var_guard573;
        *var_guard574_slot = var_guard574;
        *var_guard575_slot = var_guard575;
        *var_isb_slot = var_isb;
        *var_isb_dn0_slot = var_isb_dn0;
        *var_isb_dn1_slot = var_isb_dn1;
        *var_isb_dn12_slot = var_isb_dn12;
        *var_isb_dn14_slot = var_isb_dn14;
        *var_isb_dn15_slot = var_isb_dn15;
        *var_isb_dn16_slot = var_isb_dn16;
        *var_isb_dn17_slot = var_isb_dn17;
        *var_isb_dn18_slot = var_isb_dn18;
        *var_isb_dn19_slot = var_isb_dn19;
        *var_isb_dn2_slot = var_isb_dn2;
        *var_isb_dn20_slot = var_isb_dn20;
        *var_isb_dn21_slot = var_isb_dn21;
        *var_isb_dn22_slot = var_isb_dn22;
        *var_isb_dn3_slot = var_isb_dn3;
        *var_isb_dn4_slot = var_isb_dn4;
        *var_isb_dn5_slot = var_isb_dn5;
        *var_isb_dn6_slot = var_isb_dn6;
        *var_isb_dn7_slot = var_isb_dn7;
        *var_isb_dn8_slot = var_isb_dn8;
        *var_isb_dn9_slot = var_isb_dn9;
        *var_le_slot = var_le;
        *var_le_dn0_slot = var_le_dn0;
        *var_le_dn2_slot = var_le_dn2;
        *var_le_dn3_slot = var_le_dn3;
        *var_le_dn4_slot = var_le_dn4;
        *var_le_dn7_slot = var_le_dn7;
        *var_le_dn8_slot = var_le_dn8;
        *var_le_dn9_slot = var_le_dn9;
        *var_qfr_slot = var_qfr;
        *var_qfr2_slot = var_qfr2;
        *var_qfr2_dn0_slot = var_qfr2_dn0;
        *var_qfr2_dn1_slot = var_qfr2_dn1;
        *var_qfr2_dn12_slot = var_qfr2_dn12;
        *var_qfr2_dn14_slot = var_qfr2_dn14;
        *var_qfr2_dn15_slot = var_qfr2_dn15;
        *var_qfr2_dn16_slot = var_qfr2_dn16;
        *var_qfr2_dn17_slot = var_qfr2_dn17;
        *var_qfr2_dn18_slot = var_qfr2_dn18;
        *var_qfr2_dn19_slot = var_qfr2_dn19;
        *var_qfr2_dn2_slot = var_qfr2_dn2;
        *var_qfr2_dn20_slot = var_qfr2_dn20;
        *var_qfr2_dn21_slot = var_qfr2_dn21;
        *var_qfr2_dn22_slot = var_qfr2_dn22;
        *var_qfr2_dn3_slot = var_qfr2_dn3;
        *var_qfr2_dn4_slot = var_qfr2_dn4;
        *var_qfr2_dn5_slot = var_qfr2_dn5;
        *var_qfr2_dn6_slot = var_qfr2_dn6;
        *var_qfr2_dn7_slot = var_qfr2_dn7;
        *var_qfr2_dn8_slot = var_qfr2_dn8;
        *var_qfr2_dn9_slot = var_qfr2_dn9;
        *var_qfr3_slot = var_qfr3;
        *var_qfr3_dn0_slot = var_qfr3_dn0;
        *var_qfr3_dn2_slot = var_qfr3_dn2;
        *var_qfr_dn0_slot = var_qfr_dn0;
        *var_qfr_dn2_slot = var_qfr_dn2;
        *var_qfr_dn4_slot = var_qfr_dn4;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn1_slot = var_t0_dn1;
        *var_t0_dn12_slot = var_t0_dn12;
        *var_t0_dn14_slot = var_t0_dn14;
        *var_t0_dn15_slot = var_t0_dn15;
        *var_t0_dn16_slot = var_t0_dn16;
        *var_t0_dn17_slot = var_t0_dn17;
        *var_t0_dn18_slot = var_t0_dn18;
        *var_t0_dn19_slot = var_t0_dn19;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn20_slot = var_t0_dn20;
        *var_t0_dn21_slot = var_t0_dn21;
        *var_t0_dn22_slot = var_t0_dn22;
        *var_t0_dn3_slot = var_t0_dn3;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn1_slot = var_t1_dn1;
        *var_t1_dn12_slot = var_t1_dn12;
        *var_t1_dn14_slot = var_t1_dn14;
        *var_t1_dn15_slot = var_t1_dn15;
        *var_t1_dn16_slot = var_t1_dn16;
        *var_t1_dn17_slot = var_t1_dn17;
        *var_t1_dn18_slot = var_t1_dn18;
        *var_t1_dn19_slot = var_t1_dn19;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn20_slot = var_t1_dn20;
        *var_t1_dn21_slot = var_t1_dn21;
        *var_t1_dn22_slot = var_t1_dn22;
        *var_t1_dn3_slot = var_t1_dn3;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn1_slot = var_t2_dn1;
        *var_t2_dn12_slot = var_t2_dn12;
        *var_t2_dn14_slot = var_t2_dn14;
        *var_t2_dn15_slot = var_t2_dn15;
        *var_t2_dn16_slot = var_t2_dn16;
        *var_t2_dn17_slot = var_t2_dn17;
        *var_t2_dn18_slot = var_t2_dn18;
        *var_t2_dn19_slot = var_t2_dn19;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn20_slot = var_t2_dn20;
        *var_t2_dn21_slot = var_t2_dn21;
        *var_t2_dn22_slot = var_t2_dn22;
        *var_t2_dn3_slot = var_t2_dn3;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t3_slot = var_t3;
        *var_t3_dn0_slot = var_t3_dn0;
        *var_t3_dn1_slot = var_t3_dn1;
        *var_t3_dn12_slot = var_t3_dn12;
        *var_t3_dn14_slot = var_t3_dn14;
        *var_t3_dn15_slot = var_t3_dn15;
        *var_t3_dn16_slot = var_t3_dn16;
        *var_t3_dn17_slot = var_t3_dn17;
        *var_t3_dn18_slot = var_t3_dn18;
        *var_t3_dn19_slot = var_t3_dn19;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn20_slot = var_t3_dn20;
        *var_t3_dn21_slot = var_t3_dn21;
        *var_t3_dn22_slot = var_t3_dn22;
        *var_t3_dn3_slot = var_t3_dn3;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn1_slot = var_t4_dn1;
        *var_t4_dn12_slot = var_t4_dn12;
        *var_t4_dn14_slot = var_t4_dn14;
        *var_t4_dn15_slot = var_t4_dn15;
        *var_t4_dn16_slot = var_t4_dn16;
        *var_t4_dn17_slot = var_t4_dn17;
        *var_t4_dn18_slot = var_t4_dn18;
        *var_t4_dn19_slot = var_t4_dn19;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn20_slot = var_t4_dn20;
        *var_t4_dn21_slot = var_t4_dn21;
        *var_t4_dn22_slot = var_t4_dn22;
        *var_t4_dn3_slot = var_t4_dn3;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_dn9_slot = var_t4_dn9;
        *var_t6_slot = var_t6;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn1_slot = var_t6_dn1;
        *var_t6_dn12_slot = var_t6_dn12;
        *var_t6_dn14_slot = var_t6_dn14;
        *var_t6_dn15_slot = var_t6_dn15;
        *var_t6_dn16_slot = var_t6_dn16;
        *var_t6_dn17_slot = var_t6_dn17;
        *var_t6_dn18_slot = var_t6_dn18;
        *var_t6_dn19_slot = var_t6_dn19;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn20_slot = var_t6_dn20;
        *var_t6_dn21_slot = var_t6_dn21;
        *var_t6_dn22_slot = var_t6_dn22;
        *var_t6_dn3_slot = var_t6_dn3;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t6_dn9_slot = var_t6_dn9;
    }

    pub(super) fn stamp_transient_block_184(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_t4: f64,
        var_t4_dn0: f64,
        var_t4_dn1: f64,
        var_t4_dn12: f64,
        var_t4_dn14: f64,
        var_t4_dn15: f64,
        var_t4_dn16: f64,
        var_t4_dn17: f64,
        var_t4_dn18: f64,
        var_t4_dn19: f64,
        var_t4_dn2: f64,
        var_t4_dn20: f64,
        var_t4_dn21: f64,
        var_t4_dn22: f64,
        var_t4_dn3: f64,
        var_t4_dn4: f64,
        var_t4_dn5: f64,
        var_t4_dn6: f64,
        var_t4_dn7: f64,
        var_t4_dn8: f64,
        var_t4_dn9: f64,
        var_t6: f64,
        var_t6_dn0: f64,
        var_t6_dn1: f64,
        var_t6_dn12: f64,
        var_t6_dn14: f64,
        var_t6_dn15: f64,
        var_t6_dn16: f64,
        var_t6_dn17: f64,
        var_t6_dn18: f64,
        var_t6_dn19: f64,
        var_t6_dn2: f64,
        var_t6_dn20: f64,
        var_t6_dn21: f64,
        var_t6_dn22: f64,
        var_t6_dn3: f64,
        var_t6_dn4: f64,
        var_t6_dn5: f64,
        var_t6_dn6: f64,
        var_t6_dn7: f64,
        var_t6_dn8: f64,
        var_t6_dn9: f64,
        var_tdev: f64,
        var_tdev_dn4: f64,
        var_tnom: f64,
        var_guard576_slot: &mut f64,
        var_qdep_slot: &mut f64,
        var_qdep_dn0_slot: &mut f64,
        var_qdep_dn1_slot: &mut f64,
        var_qdep_dn12_slot: &mut f64,
        var_qdep_dn14_slot: &mut f64,
        var_qdep_dn15_slot: &mut f64,
        var_qdep_dn16_slot: &mut f64,
        var_qdep_dn17_slot: &mut f64,
        var_qdep_dn18_slot: &mut f64,
        var_qdep_dn19_slot: &mut f64,
        var_qdep_dn2_slot: &mut f64,
        var_qdep_dn20_slot: &mut f64,
        var_qdep_dn21_slot: &mut f64,
        var_qdep_dn22_slot: &mut f64,
        var_qdep_dn3_slot: &mut f64,
        var_qdep_dn4_slot: &mut f64,
        var_qdep_dn5_slot: &mut f64,
        var_qdep_dn6_slot: &mut f64,
        var_qdep_dn7_slot: &mut f64,
        var_qdep_dn8_slot: &mut f64,
        var_qdep_dn9_slot: &mut f64,
        var_t8_slot: &mut f64,
        var_t8_dn0_slot: &mut f64,
        var_t8_dn1_slot: &mut f64,
        var_t8_dn12_slot: &mut f64,
        var_t8_dn14_slot: &mut f64,
        var_t8_dn15_slot: &mut f64,
        var_t8_dn16_slot: &mut f64,
        var_t8_dn17_slot: &mut f64,
        var_t8_dn18_slot: &mut f64,
        var_t8_dn19_slot: &mut f64,
        var_t8_dn2_slot: &mut f64,
        var_t8_dn20_slot: &mut f64,
        var_t8_dn21_slot: &mut f64,
        var_t8_dn22_slot: &mut f64,
        var_t8_dn3_slot: &mut f64,
        var_t8_dn4_slot: &mut f64,
        var_t8_dn5_slot: &mut f64,
        var_t8_dn6_slot: &mut f64,
        var_t8_dn7_slot: &mut f64,
        var_t8_dn8_slot: &mut f64,
        var_t8_dn9_slot: &mut f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let mut var_guard576: f64 = *var_guard576_slot;
        let mut var_qdep: f64 = *var_qdep_slot;
        let mut var_qdep_dn0: f64 = *var_qdep_dn0_slot;
        let mut var_qdep_dn1: f64 = *var_qdep_dn1_slot;
        let mut var_qdep_dn12: f64 = *var_qdep_dn12_slot;
        let mut var_qdep_dn14: f64 = *var_qdep_dn14_slot;
        let mut var_qdep_dn15: f64 = *var_qdep_dn15_slot;
        let mut var_qdep_dn16: f64 = *var_qdep_dn16_slot;
        let mut var_qdep_dn17: f64 = *var_qdep_dn17_slot;
        let mut var_qdep_dn18: f64 = *var_qdep_dn18_slot;
        let mut var_qdep_dn19: f64 = *var_qdep_dn19_slot;
        let mut var_qdep_dn2: f64 = *var_qdep_dn2_slot;
        let mut var_qdep_dn20: f64 = *var_qdep_dn20_slot;
        let mut var_qdep_dn21: f64 = *var_qdep_dn21_slot;
        let mut var_qdep_dn22: f64 = *var_qdep_dn22_slot;
        let mut var_qdep_dn3: f64 = *var_qdep_dn3_slot;
        let mut var_qdep_dn4: f64 = *var_qdep_dn4_slot;
        let mut var_qdep_dn5: f64 = *var_qdep_dn5_slot;
        let mut var_qdep_dn6: f64 = *var_qdep_dn6_slot;
        let mut var_qdep_dn7: f64 = *var_qdep_dn7_slot;
        let mut var_qdep_dn8: f64 = *var_qdep_dn8_slot;
        let mut var_qdep_dn9: f64 = *var_qdep_dn9_slot;
        let mut var_t8: f64 = *var_t8_slot;
        let mut var_t8_dn0: f64 = *var_t8_dn0_slot;
        let mut var_t8_dn1: f64 = *var_t8_dn1_slot;
        let mut var_t8_dn12: f64 = *var_t8_dn12_slot;
        let mut var_t8_dn14: f64 = *var_t8_dn14_slot;
        let mut var_t8_dn15: f64 = *var_t8_dn15_slot;
        let mut var_t8_dn16: f64 = *var_t8_dn16_slot;
        let mut var_t8_dn17: f64 = *var_t8_dn17_slot;
        let mut var_t8_dn18: f64 = *var_t8_dn18_slot;
        let mut var_t8_dn19: f64 = *var_t8_dn19_slot;
        let mut var_t8_dn2: f64 = *var_t8_dn2_slot;
        let mut var_t8_dn20: f64 = *var_t8_dn20_slot;
        let mut var_t8_dn21: f64 = *var_t8_dn21_slot;
        let mut var_t8_dn22: f64 = *var_t8_dn22_slot;
        let mut var_t8_dn3: f64 = *var_t8_dn3_slot;
        let mut var_t8_dn4: f64 = *var_t8_dn4_slot;
        let mut var_t8_dn5: f64 = *var_t8_dn5_slot;
        let mut var_t8_dn6: f64 = *var_t8_dn6_slot;
        let mut var_t8_dn7: f64 = *var_t8_dn7_slot;
        let mut var_t8_dn8: f64 = *var_t8_dn8_slot;
        let mut var_t8_dn9: f64 = *var_t8_dn9_slot;

        let assign32240_e49812: f64 = (var_tdev / var_tnom);
        let assign32240_e49814: f64 = (assign32240_e49812 - 1.0);
        let assign32240_e49816: f64 = (assign32240_e49814 * p.p225);
        let assign32240_e49817: f64 = (p.p224 - assign32240_e49816);
        let assign32240_e49818: f64 = (p.p223 * assign32240_e49817);
        let assign32240_e49823: f64 = (1.0 - p.p228);
        let assign32240_e49824: f64 = (var_t6 * assign32240_e49823);
        let assign32240_e49825: f64 = { let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign32240_e49826: f64 = (1.0 - assign32240_e49825);
        let assign32240_e49827: f64 = (assign32240_e49818 * assign32240_e49826);
        let assign32240_e49830: f64 = (1.0 - p.p228);
        let assign32240_e49831: f64 = (assign32240_e49827 / assign32240_e49830);
        var_t8 = assign32240_e49831;
        var_t8_dn0 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t6_dn0 * assign32240_e49823)))) / assign32240_e49830);
        var_t8_dn1 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t6_dn1 * assign32240_e49823)))) / assign32240_e49830);
        var_t8_dn2 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t6_dn2 * assign32240_e49823)))) / assign32240_e49830);
        var_t8_dn3 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t6_dn3 * assign32240_e49823)))) / assign32240_e49830);
        var_t8_dn4 = ((((p.p223 * (-((var_tdev_dn4 / var_tnom) * p.p225))) * assign32240_e49826) + (assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t6_dn4 * assign32240_e49823))))) / assign32240_e49830);
        var_t8_dn5 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t6_dn5 * assign32240_e49823)))) / assign32240_e49830);
        var_t8_dn6 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t6_dn6 * assign32240_e49823)))) / assign32240_e49830);
        var_t8_dn7 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t6_dn7 * assign32240_e49823)))) / assign32240_e49830);
        var_t8_dn8 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t6_dn8 * assign32240_e49823)))) / assign32240_e49830);
        var_t8_dn9 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t6_dn9 * assign32240_e49823)))) / assign32240_e49830);
        var_t8_dn12 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t6_dn12 * assign32240_e49823)))) / assign32240_e49830);
        var_t8_dn14 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t6_dn14 * assign32240_e49823)))) / assign32240_e49830);
        var_t8_dn15 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t6_dn15 * assign32240_e49823)))) / assign32240_e49830);
        var_t8_dn16 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t6_dn16 * assign32240_e49823)))) / assign32240_e49830);
        var_t8_dn17 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t6_dn17 * assign32240_e49823)))) / assign32240_e49830);
        var_t8_dn18 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t6_dn18 * assign32240_e49823)))) / assign32240_e49830);
        var_t8_dn19 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t6_dn19 * assign32240_e49823)))) / assign32240_e49830);
        var_t8_dn20 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t6_dn20 * assign32240_e49823)))) / assign32240_e49830);
        var_t8_dn21 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t6_dn21 * assign32240_e49823)))) / assign32240_e49830);
        var_t8_dn22 = ((assign32240_e49818 * (-({ let limited_exp_arg = assign32240_e49824; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (var_t6_dn22 * assign32240_e49823)))) / assign32240_e49830);

        let assign32250_e49834: f64 = (p.p4 * p.p5);
        let assign32250_e49838: f64 = (p.p229 * p.p223);
        let assign32250_e49841: f64 = ((nv2 - nv0) - var_t4);
        let assign32250_e49842: f64 = (assign32250_e49838 * assign32250_e49841);
        let assign32250_e49843: f64 = (var_t8 + assign32250_e49842);
        let assign32250_e49844: f64 = (assign32250_e49834 * assign32250_e49843);
        var_qdep = assign32250_e49844;
        var_qdep_dn0 = (assign32250_e49834 * (var_t8_dn0 + (assign32250_e49838 * (-1.0 - var_t4_dn0))));
        var_qdep_dn1 = (assign32250_e49834 * (var_t8_dn1 + (assign32250_e49838 * (-var_t4_dn1))));
        var_qdep_dn2 = (assign32250_e49834 * (var_t8_dn2 + (assign32250_e49838 * (1.0 - var_t4_dn2))));
        var_qdep_dn3 = (assign32250_e49834 * (var_t8_dn3 + (assign32250_e49838 * (-var_t4_dn3))));
        var_qdep_dn4 = (assign32250_e49834 * (var_t8_dn4 + (assign32250_e49838 * (-var_t4_dn4))));
        var_qdep_dn5 = (assign32250_e49834 * (var_t8_dn5 + (assign32250_e49838 * (-var_t4_dn5))));
        var_qdep_dn6 = (assign32250_e49834 * (var_t8_dn6 + (assign32250_e49838 * (-var_t4_dn6))));
        var_qdep_dn7 = (assign32250_e49834 * (var_t8_dn7 + (assign32250_e49838 * (-var_t4_dn7))));
        var_qdep_dn8 = (assign32250_e49834 * (var_t8_dn8 + (assign32250_e49838 * (-var_t4_dn8))));
        var_qdep_dn9 = (assign32250_e49834 * (var_t8_dn9 + (assign32250_e49838 * (-var_t4_dn9))));
        var_qdep_dn12 = (assign32250_e49834 * (var_t8_dn12 + (assign32250_e49838 * (-var_t4_dn12))));
        var_qdep_dn14 = (assign32250_e49834 * (var_t8_dn14 + (assign32250_e49838 * (-var_t4_dn14))));
        var_qdep_dn15 = (assign32250_e49834 * (var_t8_dn15 + (assign32250_e49838 * (-var_t4_dn15))));
        var_qdep_dn16 = (assign32250_e49834 * (var_t8_dn16 + (assign32250_e49838 * (-var_t4_dn16))));
        var_qdep_dn17 = (assign32250_e49834 * (var_t8_dn17 + (assign32250_e49838 * (-var_t4_dn17))));
        var_qdep_dn18 = (assign32250_e49834 * (var_t8_dn18 + (assign32250_e49838 * (-var_t4_dn18))));
        var_qdep_dn19 = (assign32250_e49834 * (var_t8_dn19 + (assign32250_e49838 * (-var_t4_dn19))));
        var_qdep_dn20 = (assign32250_e49834 * (var_t8_dn20 + (assign32250_e49838 * (-var_t4_dn20))));
        var_qdep_dn21 = (assign32250_e49834 * (var_t8_dn21 + (assign32250_e49838 * (-var_t4_dn21))));
        var_qdep_dn22 = (assign32250_e49834 * (var_t8_dn22 + (assign32250_e49838 * (-var_t4_dn22))));

        let assign32260_e49851: f64 = if ((p.p31 == 1.0) && (p.p32 > 0.0)) { 1.0 } else { 0.0 };
        var_guard576 = assign32260_e49851;

        *var_guard576_slot = var_guard576;
        *var_qdep_slot = var_qdep;
        *var_qdep_dn0_slot = var_qdep_dn0;
        *var_qdep_dn1_slot = var_qdep_dn1;
        *var_qdep_dn12_slot = var_qdep_dn12;
        *var_qdep_dn14_slot = var_qdep_dn14;
        *var_qdep_dn15_slot = var_qdep_dn15;
        *var_qdep_dn16_slot = var_qdep_dn16;
        *var_qdep_dn17_slot = var_qdep_dn17;
        *var_qdep_dn18_slot = var_qdep_dn18;
        *var_qdep_dn19_slot = var_qdep_dn19;
        *var_qdep_dn2_slot = var_qdep_dn2;
        *var_qdep_dn20_slot = var_qdep_dn20;
        *var_qdep_dn21_slot = var_qdep_dn21;
        *var_qdep_dn22_slot = var_qdep_dn22;
        *var_qdep_dn3_slot = var_qdep_dn3;
        *var_qdep_dn4_slot = var_qdep_dn4;
        *var_qdep_dn5_slot = var_qdep_dn5;
        *var_qdep_dn6_slot = var_qdep_dn6;
        *var_qdep_dn7_slot = var_qdep_dn7;
        *var_qdep_dn8_slot = var_qdep_dn8;
        *var_qdep_dn9_slot = var_qdep_dn9;
        *var_t8_slot = var_t8;
        *var_t8_dn0_slot = var_t8_dn0;
        *var_t8_dn1_slot = var_t8_dn1;
        *var_t8_dn12_slot = var_t8_dn12;
        *var_t8_dn14_slot = var_t8_dn14;
        *var_t8_dn15_slot = var_t8_dn15;
        *var_t8_dn16_slot = var_t8_dn16;
        *var_t8_dn17_slot = var_t8_dn17;
        *var_t8_dn18_slot = var_t8_dn18;
        *var_t8_dn19_slot = var_t8_dn19;
        *var_t8_dn2_slot = var_t8_dn2;
        *var_t8_dn20_slot = var_t8_dn20;
        *var_t8_dn21_slot = var_t8_dn21;
        *var_t8_dn22_slot = var_t8_dn22;
        *var_t8_dn3_slot = var_t8_dn3;
        *var_t8_dn4_slot = var_t8_dn4;
        *var_t8_dn5_slot = var_t8_dn5;
        *var_t8_dn6_slot = var_t8_dn6;
        *var_t8_dn7_slot = var_t8_dn7;
        *var_t8_dn8_slot = var_t8_dn8;
        *var_t8_dn9_slot = var_t8_dn9;
    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.store_scalar(192, 0.0);

        s.store_scalar(193, 0.0);

        s.store_scalar(194, 0.0);

        s.store_scalar(195, 0.0);

        s.store_scalar(196, 0.0);

        s.store_scalar(197, 0.0);

        s.store_scalar(186, 1.0);

        s.store_scalar(213, 0.0);

        s.store_scalar(216, 0.0);

        s.store_scalar(94, 0.0);

        s.store_scalar(209, 0.0);

        s.store_scalar(211, 0.0);

        s.store_scalar(212, 0.0);

        s.store_scalar(222, 0.0);

        s.store_scalar(223, 0.0);

        s.store_scalar(224, 0.0);

        s.store_scalar(225, 0.0);

        s.store_scalar(226, 0.0);

        s.store_scalar(227, 0.0);

        s.store_scalar(228, 0.0);

        s.store_scalar(229, 0.0);

        s.store_scalar(230, 0.0);

        s.store_scalar(231, 0.0);

        s.store_scalar(234, 0.0);

        s.store_scalar(235, 0.0);

        s.store_scalar(236, 0.0);

        s.store_scalar(237, 0.0);

        s.store_scalar(238, 0.0);

        s.store_scalar(239, 0.0);

        s.store_scalar(240, 0.0);

        s.store_scalar(241, 0.0);

        s.store_scalar(242, 0.0);

        s.store_scalar(243, 0.0);

        s.store_scalar(246, 0.0);

        s.store_scalar(247, 0.0);

        s.store_scalar(248, 0.0);

        s.store_scalar(249, 0.0);

        s.store_scalar(250, 0.0);

        s.store_scalar(251, 0.0);

        s.store_scalar(252, 0.0);

        s.store_scalar(253, 0.0);

        s.store_scalar(254, 0.0);

        s.store_scalar(255, 0.0);

        s.store_scalar(258, 0.0);

        s.store_scalar(259, 0.0);

        s.store_scalar(260, 0.0);

        s.store_scalar(261, 0.0);

        s.store_scalar(262, 0.0);

        s.store_scalar(263, 0.0);

        s.store_scalar(264, 0.0);

        s.store_scalar(265, 0.0);

        s.store_scalar(266, 0.0);

        s.store_scalar(267, 0.0);

        s.store_scalar(270, 0.0);

        s.store_scalar(271, 0.0);

        s.store_scalar(272, 0.0);

        s.store_scalar(273, 0.0);

        s.store_scalar(274, 0.0);

        s.store_scalar(275, 0.0);

        s.store_scalar(276, 0.0);

        s.store_scalar(277, 0.0);

        s.store_scalar(278, 0.0);

        s.store_scalar(279, 0.0);

        s.store_scalar(282, 0.0);

        s.store_scalar(283, 0.0);

        s.store_scalar(284, 0.0);

        s.store_scalar(285, 0.0);

        s.store_scalar(286, 0.0);

        s.store_scalar(287, 0.0);

        s.store_scalar(288, 0.0);

        s.store_scalar(289, 0.0);

        s.store_scalar(290, 0.0);

        s.store_scalar(291, 0.0);

        s.store_scalar(294, 0.0);

        s.store_scalar(295, 0.0);

        s.store_scalar(296, 0.0);

        s.store_scalar(297, 0.0);

        s.store_scalar(298, 0.0);

        s.store_scalar(299, 0.0);

        s.store_scalar(300, 0.0);

        s.store_scalar(301, 0.0);

        s.store_scalar(302, 0.0);

        s.store_scalar(303, 0.0);

        s.store_scalar(306, 0.0);

        s.store_scalar(307, 0.0);

        s.store_scalar(308, 0.0);

        s.store_scalar(309, 0.0);

        s.store_scalar(310, 0.0);

        s.store_scalar(311, 0.0);

        s.store_scalar(312, 0.0);

        s.store_scalar(313, 0.0);

        s.store_scalar(314, 0.0);

        s.store_scalar(315, 0.0);

        s.store_scalar(182, 0.01);

        s.store_scalar(183, 0.01);

        s.store_scalar(48, 1.0);

        s.store_scalar(56, 1.0);

        s.store_scalar(64, 1.0);

        s.store_scalar(72, 1.0);

        s.store_scalar(52, 1.0);

        s.store_scalar(60, 1.0);

        s.store_scalar(68, 1.0);

        s.store_scalar(76, 1.0);

        s.store_scalar(321, 0.0);

        s.store_scalar(323, 0.0);

        s.store_scalar(326, 0.0);

        s.store_scalar(327, 0.0);

        s.store_scalar(328, 1.0);

        s.store_scalar(329, 1.0);

        s.store_scalar(339, 0.0);

        s.store_scalar(344, 0.0);

        s.store_scalar(345, 0.0);

        s.store_scalar(341, 0.0);

        s.store_scalar(340, 0.0);

        s.store_scalar(346, 0.0);

        s.store_scalar(366, 0.0);

        s.store_scalar(365, 0.0);

        s.store_scalar(361, p.p34);

        s.b[384] = (p.p149 == 1.0);
        s.store_scalar(384, if s.b[384] { 1.0 } else { 0.0 });

        s.b[385] = (s.v[361] == 0.0);
        s.store_scalar(385, if s.b[385] { 1.0 } else { 0.0 });

        if (s.b[384] && s.b[385]) {
            s.store_scalar(361, 1.0);
        }

        s.store_scalar(35, (p.p0 + 273.15));

        s.store_voltage(42, ctx, nodes, Some(7), Some(8));

        s.store_voltage(43, ctx, nodes, Some(9), Some(8));

        s.store_voltage(44, ctx, nodes, Some(9), Some(7));

        s.store_voltage(46, ctx, nodes, Some(3), Some(8));

        s.store_voltage(47, ctx, nodes, Some(3), Some(7));

        s.store_scalar(41, 1.0);

        s.b[386] = (s.v[42] < 0.0);
        s.store_scalar(386, if s.b[386] { 1.0 } else { 0.0 });

        if s.b[386] {
            s.store_scalar(41, (-1.0));
            s.store_mul(38, 41, 42);
            s.copy_ad(40, 44);
            s.copy_ad(45, 47);
        }

        if (!s.b[386]) {
            s.copy_ad(38, 42);
            s.copy_ad(40, 43);
            s.copy_ad(45, 46);
        }

        s.store_offset_sqrt_ad(140, A::offset(A::square(s.ad_value(38)), 0.01), (-0.1));

        s.store_offset_voltage(82, ctx, nodes, Some(4), None, ((ctx_temp) + (p.p274)));

        s.store_scale(36, 82, 8.617087e-5);

        s.b[387] = (p.p81 == 0.0);
        s.store_scalar(387, if s.b[387] { 1.0 } else { 0.0 });

        s.b[388] = (p.p81 == 1.0);
        s.store_scalar(388, if s.b[388] { 1.0 } else { 0.0 });

        s.b[389] = (p.p81 == 2.0);
        s.store_scalar(389, if s.b[389] { 1.0 } else { 0.0 });

        s.b[390] = (p.p81 == 3.0);
        s.store_scalar(390, if s.b[390] { 1.0 } else { 0.0 });

        s.b[391] = (p.p81 == 4.0);
        s.store_scalar(391, if s.b[391] { 1.0 } else { 0.0 });

        s.b[392] = (p.p81 == 5.0);
        s.store_scalar(392, if s.b[392] { 1.0 } else { 0.0 });

        if (s.b[388] && (!s.b[387])) {
            s.store_voltage(186, ctx, nodes, Some(5), None);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(186, 186, 0.5, 36, 0.5, 186, 36, ((0.25 * p.p128) * p.p128), 0.5);
            s.store_offset_scaled_ad(213, A::limited_exp(A::div_from_scalar((-1.0), s.ad_value(186))), p.p101, p.p100);
            s.store_offset_scaled_ad(216, A::limited_exp(A::div_from_scalar((-1.0), s.ad_value(186))), p.p103, p.p102);
        }

        if (s.b[389] && (!(s.b[387] || s.b[388]))) {
            s.store_scaled_voltage(209, ctx, nodes, Some(6), None, p.p113);
            s.store_scaled_voltage(211, ctx, nodes, Some(6), None, p.p114);
            s.store_scaled_voltage(212, ctx, nodes, Some(6), None, p.p115);
        }

        if (s.b[390] && (!((s.b[387] || s.b[388]) || s.b[389]))) {
            s.store_voltage(147, ctx, nodes, Some(0), Some(1));
            s.store_mul_div_from_scalar_ad_lhs(90, p.p124, A::scale_offset(s.ad_value(147), p.p123, 1.0), 147);
            s.store_scaled_offset(91, 147, (-p.p127), p.p125);
            s.store_exp_scaled_input_ad(136, A::offset(A::voltage(ctx, nodes, Some(1), Some(2)), (-p.p10)), ((-2.0) * 1.0 / (p.p122)));
            s.store_offset_scaled_ad(149, A::div(A::sub_from_scalar(1.0, s.ad_value(136)), A::offset(s.ad_value(136), 1.0)), ((p.p120 - 1e-9) * 0.5), ((((p.p120 - 1e-9) * 0.5)) + (1e-9)));
        }

        if (s.b[391] && (!(((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]))) {
            s.store_abs_voltage(136, ctx, nodes, Some(0), Some(2));
            s.store_abs_voltage(90, ctx, nodes, Some(1), Some(2));
            s.store_sub_voltage_abs_voltage(337, ctx, nodes, Some(12), None, Some(0), Some(2));
            s.store_scaled_add_sqrt_square_offset_rhs(337, 337, 337, ((0.25 * 1e-30) * 1e-30), 0.5);
            s.store_sub_voltage_abs_voltage(342, ctx, nodes, Some(14), None, Some(1), Some(2));
            s.store_scaled_add_sqrt_square_offset_rhs(342, 342, 342, ((0.25 * 1e-30) * 1e-30), 0.5);
            s.store_scale(136, 337, p.p89);
            s.store_sqrt_square_offset(90, 337, (p.p89 * p.p89));
            s.store_scaled_div(339, 136, 90, (((p.p91 * p.p10)) as f64).abs());
            s.store_scale(136, 342, p.p90);
            s.store_sqrt_square_offset(90, 342, (p.p90 * p.p90));
            s.store_scaled_div(344, 136, 90, (((p.p92 * p.p10)) as f64).abs());
            s.store_scale(136, 342, p.p90);
            s.store_sqrt_square_offset(90, 342, (p.p90 * p.p90));
            s.store_scaled_div(345, 136, 90, (((p.p93 * p.p13)) as f64).abs());
            s.store_scale(136, 342, p.p90);
            s.store_sqrt_square_offset(90, 342, (p.p90 * p.p90));
            s.store_scaled_div(346, 136, 90, (((p.p94 * p.p17)) as f64).abs());
            s.store_scale(136, 337, p.p89);
            s.store_sqrt_square_offset(90, 337, (p.p89 * p.p89));
            s.store_scaled_div(340, 136, 90, (((p.p95 * p.p36)) as f64).abs());
            s.store_scale(136, 337, p.p89);
            s.store_sqrt_square_offset(90, 337, (p.p89 * p.p89));
            s.store_scaled_div(341, 136, 90, (((p.p96 * p.p37)) as f64).abs());
        }

        if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
            s.store_voltage(337, ctx, nodes, Some(5), None);
            s.store_voltage(364, ctx, nodes, Some(6), None);
            s.store_scale(136, 337, p.p89);
            s.store_sqrt_square_offset(90, 337, (p.p89 * p.p89));
            s.store_scaled_div(339, 136, 90, (((p.p91 * p.p10)) as f64).abs());
            s.store_scale(136, 337, p.p89);
            s.store_sqrt_square_offset(90, 337, (p.p89 * p.p89));
            s.store_scaled_div(340, 136, 90, (((p.p95 * p.p36)) as f64).abs());
            s.store_scale(136, 337, p.p89);
            s.store_sqrt_square_offset(90, 337, (p.p89 * p.p89));
            s.store_scaled_div(341, 136, 90, (((p.p96 * p.p37)) as f64).abs());
            s.store_scale(136, 364, p.p90);
            s.store_sqrt_square_offset(90, 364, (p.p90 * p.p90));
        }

    }

    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[392] && (!((((s.b[387] || s.b[388]) || s.b[389]) || s.b[390]) || s.b[391]))) {
            s.store_scaled_div(344, 136, 90, (((p.p92 * p.p10)) as f64).abs());
            s.store_scale(136, 364, p.p90);
            s.store_sqrt_square_offset(90, 364, (p.p90 * p.p90));
            s.store_scaled_div(365, 136, 90, (((p.p147 * p.p36)) as f64).abs());
            s.store_scale(136, 364, p.p90);
            s.store_sqrt_square_offset(90, 364, (p.p90 * p.p90));
            s.store_scaled_div(366, 136, 90, (((p.p148 * p.p37)) as f64).abs());
        }

        s.store_scalar(80, (p.p9 / p.p1));

        s.store_scalar(81, (p.p9 / p.p2));

        s.store_offset_ad(146, A::mul_offset_lhs(s.ad_value(211), p.p27, s.ad_value(140)), (1.0 + p.p26));

        s.store_scaled_mul(83, 82, 146, 8.617087e-5);

        s.store_add_scaled_inputs3_offset_mixed_iia(87, 339, 1.0, 344, 1.0, A::div_scaled_product(A::sub(A::offset(s.ad_value(212), p.p22), s.ad_value(216)), s.ad_value(140), p.p23, A::sqrt_square_offset(s.ad_value(140), (p.p23 * p.p23)), 1.0), -1.0, p.p10);

        s.store_scale(334, 82, 1.0 / (s.v[35]));

        s.store_add_scaled_ad_lhs(88, A::add_scaled_inputs4_offset(s.ad_value(87), 1.0, s.ad_value(334), ((-1.0) * p.p24), s.ad_value(209), 1.0, s.ad_value(213), 1.0, ((-1.0) * ((-1.0) * p.p24))), 45, ((s.v[81] / (s.v[81] + s.v[80])) * p.p11));

        s.store_div_from_scalar_scaled_mul(136, p.p3, 83, 83, (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17));

        s.store_add_scaled_product_right_ad(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p30), 1.0);

        s.store_add_scaled_inputs4_mixed_iiai(160, 40, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(40), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);

        s.store_sub(37, 160, 88);

        s.store_div_from_scalar_scaled_input(84, s.v[80], 83, (1.602176634e-19 * 3.24e17));

        s.store_div_from_scalar(150, 2.718281828459045, 84);

        s.store_div_from_scalar(151, 1.0, 84);

        s.store_scalar(99, (s.v[80] / 1.602176634e-19));

        s.store_scaled_add_sqrt_square_offset_rhs(154, 37, 37, ((4.0 * 0.3) * 0.3), 0.5);

        s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);

        s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);

        let assign2600_ad_e4542: A = A::powf(A::scale(s.ad_value(154), s.v[99]), 0.6666666666666666);
        s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, assign2600_ad_e4542, (-(p.p28 / 3.0)), A::add_scaled_offset_product_rhs(assign2600_ad_e4542, ((2.0 * p.p28) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);

        s.store_div_scaled_inputs_indices(136, 37, 1.0, 83, 2.0);

        s.b[393] = (s.v[136] < 200.0);
        s.store_scalar(393, if s.b[393] { 1.0 } else { 0.0 });

        if s.b[393] {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product_mixed_iaa(153, 83, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), (2.0 * s.v[99]), A::add_scaled_inputs(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, A::limited_exp_div_scaled_inputs(s.ad_value(37), (-1.0), s.ad_value(83), 2.0), (s.v[99] / 3.24e17)), 1.0);
        }

        if (!s.b[393]) {
            s.store_div_scaled_product_add_scaled_denominator(153, 83, 136, ((2.0 * s.v[99]) * 1.0 / (1.0)), A::div_from_scalar(1.0, s.ad_value(152)), 1.0, A::limited_exp_div_scaled_inputs(s.ad_value(37), (-1.0), s.ad_value(83), 2.0), (s.v[99] / 3.24e17), 1.0);
        }

        s.store_sub_scaled_inputs(100, 37, 1.0, 153, 1.0 / (s.v[99]));

        s.b[394] = ((((s.v[100] - s.v[37])) as f64).abs() > 1e-19);
        s.store_scalar(394, if s.b[394] { 1.0 } else { 0.0 });

        if s.b[394] {
            s.store_sub(101, 37, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_scalar(136, ((s.v[99]) as f64).powf(0.6666666666666666));
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p28);
            s.store_scaled_mul(103, 136, 90, p.p29);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if s.b[394] {
            s.store_add_scaled_value_products(106, s.ad_value(101), s.v[99], s.ad_value(83), {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(105))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if s.b[394] {
            s.store_scaled_mul(107, 136, 91, p.p28);
            s.store_scaled_mul(108, 136, 91, p.p29);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_sub_ad(113, A::sub_from_scalar(((-1.0) * s.v[99]), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 37, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(116, 136, p.p28, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p29, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if s.b[394] {
            s.store_add_scaled_value_products(120, s.ad_value(115), s.v[99], s.ad_value(83), {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(119))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if s.b[394] {
            s.store_scaled_mul(121, 136, 137, p.p28);
            s.store_scaled_mul(122, 136, 137, p.p29);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_sub_ad(127, A::sub_from_scalar(((-1.0) * s.v[99]), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.copy_ad(129, 128);
        }

        if (!s.b[394]) {
            s.copy_ad(129, 100);
        }

        s.store_sub_from_scalar(347, p.p13, 345);

        s.store_sub_from_scalar(348, p.p17, 346);

        s.store_mul_powf_ad_rhs(97, 347, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20);

        s.store_mul_powf_ad_rhs(89, 348, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19);

        s.store_scaled_abs_ad(136, A::sub(s.ad_value(37), s.ad_value(129)), (s.v[80] / p.p9));

        s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));

        s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));

        s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);

        s.store_scaled_add_sqrt_square_offset_rhs(90, 37, 37, ((4.0 * 0.3) * 0.3), 0.5);

        s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p3, 136, p.p3, 90, 1.0, 1.0);

        s.store_powf_ad(136, A::div(s.ad_value(38), s.ad_value(85)), p.p18);

        s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p.p18));

        s.store_mul(86, 38, 90);

        s.store_sub(39, 37, 86);

        s.copy_ad(130, 39);

        s.store_scaled_add_sqrt_square_offset_rhs(131, 130, 130, ((4.0 * 0.3) * 0.3), 0.5);

        s.copy_ad(154, 131);

        s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);

        s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);

        let assign3240_ad_e5317: A = A::powf(A::scale(s.ad_value(154), s.v[99]), 0.6666666666666666);
        s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, assign3240_ad_e5317, (-(p.p28 / 3.0)), A::add_scaled_offset_product_rhs(assign3240_ad_e5317, ((2.0 * p.p28) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);

        s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);

        s.b[395] = (s.v[136] < 200.0);
        s.store_scalar(395, if s.b[395] { 1.0 } else { 0.0 });

        if s.b[395] {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product_mixed_iaa(156, 83, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), (2.0 * s.v[99]), A::add_scaled_inputs(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), (s.v[99] / 3.24e17)), 1.0);
        }

        if (!s.b[395]) {
            s.store_div_scaled_product_add_scaled_denominator(156, 83, 136, ((2.0 * s.v[99]) * 1.0 / (1.0)), A::div_from_scalar(1.0, s.ad_value(152)), 1.0, A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), (s.v[99] / 3.24e17), 1.0);
        }

        s.store_sub_scaled_inputs(100, 130, 1.0, 156, 1.0 / (s.v[99]));

        s.b[396] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.store_scalar(396, if s.b[396] { 1.0 } else { 0.0 });

        if s.b[396] {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_scalar(136, ((s.v[99]) as f64).powf(0.6666666666666666));
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p28);
            s.store_scaled_mul(103, 136, 90, p.p29);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if s.b[396] {
            s.store_add_scaled_value_products(106, s.ad_value(101), s.v[99], s.ad_value(83), {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(105))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if s.b[396] {
            s.store_scaled_mul(107, 136, 91, p.p28);
            s.store_scaled_mul(108, 136, 91, p.p29);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_sub_ad(113, A::sub_from_scalar(((-1.0) * s.v[99]), A::div(s.ad_value(109), s.ad_value(110))), A::div(s.ad_value(111), s.ad_value(112)));
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 130, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_mul_scaled_powf_rhs(116, 136, p.p28, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p29, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

    }

    pub(super) fn stamp_reactive_block_2(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[396] {
            s.store_add_scaled_value_products(120, s.ad_value(115), s.v[99], s.ad_value(83), {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(119))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if s.b[396] {
            s.store_mul_scaled_powf_rhs(121, 136, p.p28, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(122, 136, p.p29, 115, (-0.3333333333333333));
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_sub_ad(127, A::sub_from_scalar(((-1.0) * s.v[99]), A::div(s.ad_value(123), s.ad_value(124))), A::div(s.ad_value(125), s.ad_value(126)));
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.store_add(132, 128, 86);
        }

        if (!s.b[396]) {
            s.store_add(132, 100, 86);
        }

        s.store_scaled_add(133, 129, 132, 0.5);

        s.store_sub(134, 132, 129);

        s.store_mul_add_scaled_inputs3_offset_rhs(135, 134, s.ad_value(37), 1.0, s.ad_value(133), (-1.0), s.ad_value(83), 1.0, 0.0);

        s.store_scaled_abs_ad(136, A::sub(s.ad_value(37), s.ad_value(133)), (s.v[80] / p.p9));

        s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));

        s.store_div_add_scaled_inputs_rhs_mixed_ai(95, 97, A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, 90, p.p16);

        s.store_scale(96, 95, (s.v[80] * (p.p4 * (p.p5 * 1.0 / (p.p3)))));

        s.store_mul_offset_ad_rhs(98, 96, A::sub_scaled_inputs(s.ad_value(140), p.p21, s.ad_value(86), p.p21), 1.0);

        s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(134), (p.p25 * p.p25), s.ad_value(134)), 1.0);

        s.store_div(93, 98, 92);

        s.store_mul(94, 93, 135);

        s.store_sub(90, 132, 129);

        s.store_add_scaled_inputs3_indices(91, 37, 1.0, 83, 1.0, 133, -1.0);

        s.store_add_scaled_inputs3_mixed_iia(137, 37, (((s.v[80] * p.p4) * p.p5) * p.p3), 133, ((-1.0) * (((s.v[80] * p.p4) * p.p5) * p.p3)), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), (((s.v[80] * p.p4) * p.p5) * p.p3));

        s.store_scale(188, 137, (1.0 / (p.p233) * 1e26));

        s.store_offset_powf_ad(189, s.ad_value(188), p.p232, 1.0);

        s.store_div_from_scalar(190, p.p231, 189);

        s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p1);

        s.store_mul_add_scaled_inputs3_offset_rhs(161, 191, s.ad_value(37), ((p.p4 * p.p5) * p.p3), s.ad_value(133), (((-1.0)) * (((p.p4 * p.p5) * p.p3))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p3), 0.0);

        s.store_add_scaled_inputs3_indices(136, 37, 1.0, 83, 1.0, 133, -1.0);

        s.store_add_scaled_inputs(90, 129, 0.3333333333333333, 132, (2.0 * 0.3333333333333333));

        s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(134)), (1.0 / 12.0), 136, 1.0);

        s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(134)), 134, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);

        s.store_mul_add_scaled_inputs4_indices_rhs(165, 191, 37, (-(((p.p4 * p.p3) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p3) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p3) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p3) * p.p5) * 0.5)));

        s.store_sub_scaled_inputs(166, 161, (-1.0), 165, 1.0);

        s.b[401] = (s.v[41] < 0.0);
        s.store_scalar(401, if s.b[401] { 1.0 } else { 0.0 });

        if s.b[401] {
            s.copy_ad(90, 166);
            s.copy_ad(166, 165);
            s.copy_ad(165, 90);
        }

        s.b[402] = (p.p56 == 0.0);
        s.store_scalar(402, if s.b[402] { 1.0 } else { 0.0 });

        s.b[403] = (p.p56 == 1.0);
        s.store_scalar(403, if s.b[403] { 1.0 } else { 0.0 });

        s.b[404] = (p.p56 == 2.0);
        s.store_scalar(404, if s.b[404] { 1.0 } else { 0.0 });

        s.b[405] = (p.p56 == 3.0);
        s.store_scalar(405, if s.b[405] { 1.0 } else { 0.0 });

        s.b[406] = (p.p56 == 4.0);
        s.store_scalar(406, if s.b[406] { 1.0 } else { 0.0 });

        if (s.b[403] && (!s.b[402])) {
            s.store_div_scaled_inputs_mixed_ai(136, A::voltage(ctx, nodes, Some(9), Some(8)), 1.0, 82, (p.p57 * 8.617087e-5));
            s.store_offset_scaled(137, 82, ((1.0 / (s.v[35])) * (p.p71)), (((((-1.0)) * (p.p71))) + (p.p63)));
            s.store_div_scaled_inputs_mixed_ai(136, A::voltage(ctx, nodes, Some(9), Some(7)), 1.0, 82, (p.p60 * 8.617087e-5));
            s.store_offset_scaled(137, 82, ((1.0 / (s.v[35])) * (p.p72)), (((((-1.0)) * (p.p72))) + (p.p64)));
        }

        if (s.b[404] && (!(s.b[402] || s.b[403]))) {
            s.store_offset_scaled(326, 82, ((1.0 / (s.v[35])) * (p.p75)), (((((-1.0)) * (p.p75))) + (p.p67)));
            s.store_offset_scaled(328, 82, ((1.0 / (s.v[35])) * (p.p77)), (((((-1.0)) * (p.p77))) + (p.p57)));
            s.store_offset_scaled(330, 82, ((1.0 / (s.v[35])) * (p.p79)), (((((-1.0)) * (p.p79))) + (p.p61)));
            s.store_div_scaled_inputs2_mixed_aii(136, A::voltage(ctx, nodes, Some(9), Some(8)), 1.0, 326, (-1.0), 328, (8.617087e-5 * s.v[35]));
            s.store_scale_ad(137, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p71), p.p63);
            s.store_add_scaled_inputs3_sqrt_third_ad(321, A::voltage(ctx, nodes, Some(9), Some(8)), -1.0, A::voltage(ctx, nodes, Some(9), Some(8)), (-(-0.5)), A::offset(A::square(A::neg(A::voltage(ctx, nodes, Some(9), Some(8)))), 0.001), (-(-0.5)));
            s.store_offset_sqrt(136, 321, p.p69);
            s.store_div_scaled_inputs_indices(90, 136, 1.0, 330, (8.617087e-5 * s.v[35]));
            s.store_offset_scaled(327, 82, ((1.0 / (s.v[35])) * (p.p76)), (((((-1.0)) * (p.p76))) + (p.p68)));
            s.store_offset_scaled(329, 82, ((1.0 / (s.v[35])) * (p.p78)), (((((-1.0)) * (p.p78))) + (p.p60)));
            s.store_offset_scaled(331, 82, ((1.0 / (s.v[35])) * (p.p80)), (((((-1.0)) * (p.p80))) + (p.p62)));
            s.store_div_scaled_inputs2_mixed_aii(136, A::voltage(ctx, nodes, Some(9), Some(7)), 1.0, 327, (-1.0), 329, (8.617087e-5 * s.v[35]));
            s.store_scale_ad(137, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p72), p.p64);
            s.store_add_scaled_inputs3_sqrt_third_ad(323, A::voltage(ctx, nodes, Some(9), Some(7)), -1.0, A::voltage(ctx, nodes, Some(9), Some(7)), (-(-0.5)), A::offset(A::square(A::neg(A::voltage(ctx, nodes, Some(9), Some(7)))), 0.001), (-(-0.5)));
            s.store_offset_sqrt(136, 323, p.p70);
            s.store_div_scaled_inputs_indices(136, 136, 1.0, 331, (8.617087e-5 * s.v[35]));
        }

        if (s.b[405] && (!((s.b[402] || s.b[403]) || s.b[404]))) {
            s.store_offset_scaled(326, 82, ((1.0 / (s.v[35])) * (p.p75)), (((((-1.0)) * (p.p75))) + (p.p67)));
            s.store_offset_scaled(328, 82, ((1.0 / (s.v[35])) * (p.p77)), (((((-1.0)) * (p.p77))) + (p.p57)));
            s.store_offset_scaled(330, 82, ((1.0 / (s.v[35])) * (p.p79)), (((((-1.0)) * (p.p79))) + (p.p61)));
            s.store_scale_ad(137, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p71), (((p.p4 * p.p3) * p.p5) * p.p63));
            s.store_offset_scaled(327, 82, ((1.0 / (s.v[35])) * (p.p76)), (((((-1.0)) * (p.p76))) + (p.p68)));
            s.store_offset_scaled(329, 82, ((1.0 / (s.v[35])) * (p.p78)), (((((-1.0)) * (p.p78))) + (p.p60)));
            s.store_offset_scaled(331, 82, ((1.0 / (s.v[35])) * (p.p80)), (((((-1.0)) * (p.p80))) + (p.p62)));
            s.store_scale_ad(137, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p72), (((p.p4 * p.p3) * p.p5) * p.p64));
        }

        if (s.b[406] && (!(((s.b[402] || s.b[403]) || s.b[404]) || s.b[405]))) {
            s.store_offset_scaled(326, 82, ((1.0 / (s.v[35])) * (p.p75)), (((((-1.0)) * (p.p75))) + (p.p67)));
            s.store_offset_scaled(328, 82, ((1.0 / (s.v[35])) * (p.p77)), (((((-1.0)) * (p.p77))) + (p.p57)));
            s.store_offset_scaled(330, 82, ((1.0 / (s.v[35])) * (p.p79)), (((((-1.0)) * (p.p79))) + (p.p61)));
            s.store_scale_ad(137, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p71), (((p.p4 * p.p3) * p.p5) * p.p63));
            s.store_offset_scaled(327, 82, ((1.0 / (s.v[35])) * (p.p76)), (((((-1.0)) * (p.p76))) + (p.p68)));
            s.store_offset_scaled(329, 82, ((1.0 / (s.v[35])) * (p.p78)), (((((-1.0)) * (p.p78))) + (p.p60)));
            s.store_offset_scaled(331, 82, ((1.0 / (s.v[35])) * (p.p80)), (((((-1.0)) * (p.p80))) + (p.p62)));
            s.store_scale_ad(137, A::exp_scaled_input(A::scale_offset(s.ad_value(82), 1.0 / (s.v[35]), (-1.0)), p.p72), (((p.p4 * p.p3) * p.p5) * p.p64));
        }

        s.b[359] = param_given[45];
        s.store_scalar(359, if s.b[359] { 1.0 } else { 0.0 });

        s.b[360] = param_given[44];
        s.store_scalar(360, if s.b[360] { 1.0 } else { 0.0 });

        s.copy_ad(187, 154);

        s.b[424] = (s.v[361] == 1.0);
        s.store_scalar(424, if s.b[424] { 1.0 } else { 0.0 });

        if s.b[424] {
            s.store_add_scaled_inputs4_offset_indices(177, 82, ((-p.p36) * ((1.0 / (s.v[35])) * (p.p50))), 340, (-1.0), 365, -1.0, 45, ((p.p12 / 1.602176634e-19) * s.v[81]), (p.p36 + ((-p.p36) * (((-1.0)) * (p.p50)))));
            s.store_add_scaled_inputs3_offset_mixed_iia(177, 177, 1.0, 177, (-0.5), A::sqrt_square_offset(A::offset(s.ad_value(177), (-1.0)), 0.001), (-(-0.5)), (1.0 + (-0.5)));
            s.store_mul_scale_offset_rhs(172, 177, 187, ((p.p38) * (1.602176634e-19)), 1.602176634e-19);
            s.store_scaled_powf_ad(176, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p51, p.p35);
            s.store_scaled_mul(173, 172, 176, (p.p4 * p.p5));
        }

        s.b[425] = s.b[359];
        s.store_scalar(425, if s.b[425] { 1.0 } else { 0.0 });

        if (s.b[424] && s.b[425]) {
            s.store_scalar(350, (1.0 + p.p45));
            s.store_mul_sqrt_lhs(351, 350, 94);
            s.store_div(352, 351, 173);
            s.store_scale(353, 352, 2.0);
            s.store_add_ad_rhs(350, 350, A::square(s.ad_value(352)));
            s.store_add_ad(350, A::sqrt(A::sub(s.ad_value(350), s.ad_value(353))), A::sqrt(A::add(s.ad_value(350), s.ad_value(353))));
            s.store_div_scaled_inputs_indices(349, 351, 2.0, 350, 1.0);
            s.store_sub_from_scalar_div_indices(91, 1.0, 349, 173);
        }

        if (s.b[424] && (!s.b[425])) {
            s.store_abs_ad(182, A::div(s.ad_value(94), s.ad_value(173)));
            s.store_scaled_offset_ad(183, A::sub(A::offset(s.ad_value(182), 0.9), A::sqrt_square_offset(A::offset(s.ad_value(182), (-0.9)), (0.1 * 0.1))), (-(0.9 - ((((0.9 * 0.9) + (0.1 * 0.1))) as f64).sqrt())), 0.5);
            s.store_powf(136, 183, p.p42);
            s.store_sub_from_scalar(90, 1.0, 136);
            s.store_powf(91, 90, (1.0 / p.p42));
        }

        if s.b[424] {
            s.store_add_scaled_inputs4_offset_indices(177, 82, ((-p.p37) * ((1.0 / (s.v[35])) * (p.p50))), 341, (-1.0), 366, -1.0, 45, ((p.p12 / 1.602176634e-19) * s.v[81]), (p.p37 + ((-p.p37) * (((-1.0)) * (p.p50)))));
            s.store_add_scaled_inputs3_offset_mixed_iia(177, 177, 1.0, 177, (-0.5), A::sqrt_square_offset(A::offset(s.ad_value(177), (-1.0)), 0.001), (-(-0.5)), (1.0 + (-0.5)));
            s.store_mul_scale_offset_rhs(172, 177, 187, ((p.p39) * (1.602176634e-19)), 1.602176634e-19);
            s.store_scaled_mul(173, 172, 176, (p.p4 * p.p5));
        }

        s.b[426] = s.b[360];
        s.store_scalar(426, if s.b[426] { 1.0 } else { 0.0 });

        if (s.b[424] && s.b[426]) {
            s.store_scalar(350, (1.0 + p.p44));
            s.store_mul_sqrt_lhs(351, 350, 94);
            s.store_div(352, 351, 173);
            s.store_scale(353, 352, 2.0);
            s.store_add_ad_rhs(350, 350, A::square(s.ad_value(352)));
            s.store_add_ad(350, A::sqrt(A::sub(s.ad_value(350), s.ad_value(353))), A::sqrt(A::add(s.ad_value(350), s.ad_value(353))));
            s.store_div_scaled_inputs_indices(349, 351, 2.0, 350, 1.0);
            s.store_sub_from_scalar_div_indices(91, 1.0, 349, 173);
        }

        if (s.b[424] && (!s.b[426])) {
            s.store_abs_ad(182, A::div(s.ad_value(94), s.ad_value(173)));
            s.store_scaled_offset_ad(183, A::sub(A::offset(s.ad_value(182), 0.9), A::sqrt_square_offset(A::offset(s.ad_value(182), (-0.9)), (0.1 * 0.1))), (-(0.9 - ((((0.9 * 0.9) + (0.1 * 0.1))) as f64).sqrt())), 0.5);
            s.store_powf(136, 183, p.p43);
            s.store_sub_from_scalar(90, 1.0, 136);
            s.store_powf(91, 90, (1.0 / p.p43));
        }

        s.b[433] = (p.p149 == 0.0);
        s.store_scalar(433, if s.b[433] { 1.0 } else { 0.0 });

        s.b[434] = (p.p150 != 0.0);
        s.store_scalar(434, if s.b[434] { 1.0 } else { 0.0 });

        if (s.b[433] && s.b[434]) {
            s.store_voltage(49, ctx, nodes, Some(15), Some(7));
        }

        s.b[435] = (p.p150 == 1.0);
        s.store_scalar(435, if s.b[435] { 1.0 } else { 0.0 });

        if ((s.b[433] && s.b[434]) && s.b[435]) {
            s.store_voltage(50, ctx, nodes, Some(9), Some(7));
            s.store_voltage(51, ctx, nodes, Some(9), Some(15));
        }

        if ((s.b[433] && s.b[434]) && (!s.b[435])) {
            s.store_voltage(50, ctx, nodes, Some(2), Some(7));
            s.store_voltage(51, ctx, nodes, Some(2), Some(15));
        }

        if (s.b[433] && s.b[434]) {
            s.store_scalar(48, 1.0);
        }

        s.b[436] = (s.v[49] < 0.0);
        s.store_scalar(436, if s.b[436] { 1.0 } else { 0.0 });

        if ((s.b[433] && s.b[434]) && s.b[436]) {
            s.store_scalar(48, (-1.0));
            s.store_mul(231, 48, 49);
            s.copy_ad(230, 51);
        }

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[433] && s.b[434]) && (!s.b[436])) {
            s.copy_ad(231, 49);
            s.copy_ad(230, 50);
        }

        if (s.b[433] && s.b[434]) {
            s.store_offset_sqrt_ad(232, A::offset(A::square(s.ad_value(231)), 0.01), (-0.1));
            s.store_offset_scaled(146, 232, p.p166, (1.0 + p.p165));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_sub_ad(88, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p162)), (((((-1.0)) * (p.p162))) + (p.p159))), A::div_scaled_inputs(s.ad_value(232), (p.p168 * p.p167), A::sqrt_square_offset(s.ad_value(232), (p.p168 * p.p168)), 1.0));
            s.store_scalar(223, (p.p9 / p.p160));
            s.store_div_from_scalar_scaled_mul(136, p.p161, 83, 83, (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17));
            s.store_add_scaled_product_right_ad(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p158), 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(160, 230, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(230), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);
            s.store_sub(222, 160, 88);
            s.store_div_scaled_inputs_indices(84, 223, 1.0, 83, (1.602176634e-19 * 3.24e17));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 223, 6.241509074460763e18);
            s.store_scaled_add_sqrt_square_offset_rhs(154, 222, 222, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);
        }

        if (s.b[433] && s.b[434]) {
            let assign6440_ad_e9653: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, assign6440_ad_e9653, (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(assign6440_ad_e9653, ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
        }

        if (s.b[433] && s.b[434]) {
            s.store_div_scaled_inputs_indices(136, 222, 1.0, 83, 2.0);
        }

        s.b[437] = (s.v[136] < 200.0);
        s.store_scalar(437, if s.b[437] { 1.0 } else { 0.0 });

        if ((s.b[433] && s.b[434]) && s.b[437]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(222), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[433] && s.b[434]) && (!s.b[437])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(222), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[433] && s.b[434]) {
            s.store_sub_div_rhs_indices(100, 222, 153, 99);
        }

        s.b[438] = ((((s.v[100] - s.v[222])) as f64).abs() > 1e-19);
        s.store_scalar(438, if s.b[438] { 1.0 } else { 0.0 });

        if ((s.b[433] && s.b[434]) && s.b[438]) {
            s.store_sub(101, 222, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p169);
            s.store_scaled_mul(103, 136, 90, p.p170);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if ((s.b[433] && s.b[434]) && s.b[438]) {
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(105))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if ((s.b[433] && s.b[434]) && s.b[438]) {
            s.store_scaled_mul(107, 136, 91, p.p169);
            s.store_scaled_mul(108, 136, 91, p.p170);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 222, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(116, 136, p.p169, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p170, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if ((s.b[433] && s.b[434]) && s.b[438]) {
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(119))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if ((s.b[433] && s.b[434]) && s.b[438]) {
            s.store_scaled_mul(121, 136, 137, p.p169);
            s.store_scaled_mul(122, 136, 137, p.p170);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.copy_ad(224, 128);
        }

        if ((s.b[433] && s.b[434]) && (!s.b[438])) {
            s.copy_ad(224, 100);
        }

        if (s.b[433] && s.b[434]) {
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p163);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p164);
            s.store_mul_scaled_abs_ad_rhs(136, 223, 1.0 / (p.p9), A::sub(s.ad_value(222), s.ad_value(224)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(224)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(90, 222, 222, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p161, 136, p.p161, 90, 1.0, 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(231), s.ad_value(85)), p.p18);
            s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p.p18));
            s.store_mul(86, 231, 90);
            s.store_sub(39, 222, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_sqrt_square_offset_rhs(131, 130, 130, ((4.0 * 0.3) * 0.3), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);
        }

        if (s.b[433] && s.b[434]) {
            let assign7060_ad_e10682: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, assign7060_ad_e10682, (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(assign7060_ad_e10682, ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
        }

        if (s.b[433] && s.b[434]) {
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[439] = (s.v[136] < 200.0);
        s.store_scalar(439, if s.b[439] { 1.0 } else { 0.0 });

        if ((s.b[433] && s.b[434]) && s.b[439]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[433] && s.b[434]) && (!s.b[439])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[433] && s.b[434]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[440] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.store_scalar(440, if s.b[440] { 1.0 } else { 0.0 });

        if ((s.b[433] && s.b[434]) && s.b[440]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p169);
            s.store_scaled_mul(103, 136, 90, p.p170);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if ((s.b[433] && s.b[434]) && s.b[440]) {
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(105))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if ((s.b[433] && s.b[434]) && s.b[440]) {
            s.store_scaled_mul(107, 136, 91, p.p169);
            s.store_scaled_mul(108, 136, 91, p.p170);
        }

    }

    pub(super) fn stamp_reactive_block_4(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[433] && s.b[434]) && s.b[440]) {
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 130, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_mul_scaled_powf_rhs(116, 136, p.p169, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p170, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if ((s.b[433] && s.b[434]) && s.b[440]) {
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(119))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if ((s.b[433] && s.b[434]) && s.b[440]) {
            s.store_mul_scaled_powf_rhs(121, 136, p.p169, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(122, 136, p.p170, 115, (-0.3333333333333333));
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.store_add(225, 128, 86);
        }

        if ((s.b[433] && s.b[434]) && (!s.b[440])) {
            s.store_add(225, 100, 86);
        }

        if (s.b[433] && s.b[434]) {
            s.store_scaled_add(226, 224, 225, 0.5);
            s.store_sub(227, 225, 224);
            s.store_mul_add_scaled_inputs3_offset_rhs(135, 227, s.ad_value(222), 1.0, s.ad_value(226), (-1.0), s.ad_value(83), 1.0, 0.0);
            s.store_mul_scaled_abs_ad_rhs(136, 223, 1.0 / (p.p9), A::sub(s.ad_value(222), s.ad_value(226)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(129)), (s.v[81] / p.p9));
            s.store_div_add_scaled_inputs_rhs_mixed_ai(95, 97, A::add_scaled_product(A::scale_offset(s.ad_value(136), p.p14, 1.0), 1.0, s.ad_value(136), s.ad_value(136), p.p15), 1.0, 90, p.p16);
            s.store_scaled_mul(96, 95, 223, (p.p4 * (p.p5 * 1.0 / (p.p161))));
            s.store_mul_offset_ad_rhs(98, 96, A::sub_scaled_inputs(s.ad_value(232), p.p21, s.ad_value(86), p.p21), 1.0);
            s.store_sqrt_offset_ad(92, A::mul_scaled_lhs(s.ad_value(227), (p.p25 * p.p25), s.ad_value(227)), 1.0);
            s.store_div(93, 98, 92);
            s.store_sub(90, 225, 224);
            s.store_add_scaled_inputs3_indices(91, 222, 1.0, 83, 1.0, 226, -1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(137, 223, s.ad_value(222), ((p.p4 * p.p5) * p.p161), s.ad_value(226), (((-1.0)) * (((p.p4 * p.p5) * p.p161))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p161), 0.0);
            s.store_scale(188, 137, (1.0 / (p.p236) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p235, 1.0);
            s.store_div_from_scalar(190, p.p234, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p160);
            s.store_mul_add_scaled_inputs3_offset_rhs(228, 191, s.ad_value(222), ((p.p4 * p.p5) * p.p161), s.ad_value(226), (((-1.0)) * (((p.p4 * p.p5) * p.p161))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p161), 0.0);
            s.store_add_scaled_inputs3_indices(136, 222, 1.0, 83, 1.0, 226, -1.0);
            s.store_add_scaled_inputs(90, 224, 0.3333333333333333, 225, (2.0 * 0.3333333333333333));
            s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(227)), (1.0 / 12.0), 136, 1.0);
            s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(227)), 227, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_add_scaled_inputs4_indices_rhs(229, 191, 222, (-(((p.p4 * p.p161) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p161) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p161) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p161) * p.p5) * 0.5)));
        }

        s.b[441] = (s.v[48] < 0.0);
        s.store_scalar(441, if s.b[441] { 1.0 } else { 0.0 });

        if ((s.b[433] && s.b[434]) && s.b[441]) {
            s.store_sub_scaled_inputs(229, 228, (-1.0), 229, 1.0);
        }

        if (s.b[433] && (!s.b[434])) {
            s.store_scalar(228, 0.0);
            s.store_scalar(229, 0.0);
        }

        s.b[442] = (p.p150 != 0.0);
        s.store_scalar(442, if s.b[442] { 1.0 } else { 0.0 });

        s.b[443] = (p.p150 == 1.0);
        s.store_scalar(443, if s.b[443] { 1.0 } else { 0.0 });

        if (((!s.b[433]) && s.b[442]) && s.b[443]) {
            s.store_voltage(50, ctx, nodes, Some(9), Some(7));
        }

        if (((!s.b[433]) && s.b[442]) && (!s.b[443])) {
            s.store_voltage(50, ctx, nodes, Some(2), Some(7));
        }

        if ((!s.b[433]) && s.b[442]) {
            s.copy_ad(230, 50);
            s.store_scalar(146, (1.0 + p.p165));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_offset_scaled(88, 82, ((1.0 / (s.v[35])) * (p.p162)), (((((-1.0)) * (p.p162))) + (p.p159)));
            s.store_scalar(223, (p.p9 / p.p160));
            s.store_div_from_scalar_scaled_mul(136, p.p161, 83, 83, (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17));
            s.store_add_scaled_product_right_ad(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p158), 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(160, 230, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(230), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);
            s.store_sub(222, 160, 88);
            s.store_div_scaled_inputs_indices(84, 223, 1.0, 83, (1.602176634e-19 * 3.24e17));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 223, 6.241509074460763e18);
            s.store_scaled_add_sqrt_square_offset_rhs(154, 222, 222, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);
        }

        if ((!s.b[433]) && s.b[442]) {
            let assign7980_ad_e12080: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, assign7980_ad_e12080, (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(assign7980_ad_e12080, ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
        }

        if ((!s.b[433]) && s.b[442]) {
            s.store_div_scaled_inputs_indices(136, 222, 1.0, 83, 2.0);
        }

        s.b[444] = (s.v[136] < 200.0);
        s.store_scalar(444, if s.b[444] { 1.0 } else { 0.0 });

        if (((!s.b[433]) && s.b[442]) && s.b[444]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(222), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[433]) && s.b[442]) && (!s.b[444])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(222), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[433]) && s.b[442]) {
            s.store_sub_div_rhs_indices(100, 222, 153, 99);
        }

        s.b[445] = ((((s.v[100] - s.v[222])) as f64).abs() > 1e-19);
        s.store_scalar(445, if s.b[445] { 1.0 } else { 0.0 });

        if (((!s.b[433]) && s.b[442]) && s.b[445]) {
            s.store_sub(101, 222, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p169);
            s.store_scaled_mul(103, 136, 90, p.p170);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if (((!s.b[433]) && s.b[442]) && s.b[445]) {
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(105))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if (((!s.b[433]) && s.b[442]) && s.b[445]) {
            s.store_scaled_mul(107, 136, 91, p.p169);
            s.store_scaled_mul(108, 136, 91, p.p170);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 222, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(116, 136, p.p169, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p170, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

    }

    pub(super) fn stamp_reactive_block_5(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((!s.b[433]) && s.b[442]) && s.b[445]) {
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(119))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if (((!s.b[433]) && s.b[442]) && s.b[445]) {
            s.store_scaled_mul(121, 136, 137, p.p169);
            s.store_scaled_mul(122, 136, 137, p.p170);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.copy_ad(224, 128);
        }

        if (((!s.b[433]) && s.b[442]) && (!s.b[445])) {
            s.copy_ad(224, 100);
        }

        if ((!s.b[433]) && s.b[442]) {
            s.store_scalar(231, 0.0);
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p163);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p164);
            s.store_mul_scaled_abs_ad_rhs(136, 223, 1.0 / (p.p9), A::sub(s.ad_value(222), s.ad_value(224)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(224)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(90, 222, 222, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p161, 136, p.p161, 90, 1.0, 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(231), s.ad_value(85)), p.p18);
            s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p.p18));
            s.store_mul(86, 231, 90);
            s.store_sub(39, 222, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_sqrt_square_offset_rhs(131, 130, 130, ((4.0 * 0.3) * 0.3), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);
        }

        if ((!s.b[433]) && s.b[442]) {
            let assign8610_ad_e13176: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, assign8610_ad_e13176, (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(assign8610_ad_e13176, ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
        }

        if ((!s.b[433]) && s.b[442]) {
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[446] = (s.v[136] < 200.0);
        s.store_scalar(446, if s.b[446] { 1.0 } else { 0.0 });

        if (((!s.b[433]) && s.b[442]) && s.b[446]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (((!s.b[433]) && s.b[442]) && (!s.b[446])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((!s.b[433]) && s.b[442]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[447] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.store_scalar(447, if s.b[447] { 1.0 } else { 0.0 });

        if (((!s.b[433]) && s.b[442]) && s.b[447]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p169);
            s.store_scaled_mul(103, 136, 90, p.p170);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if (((!s.b[433]) && s.b[442]) && s.b[447]) {
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(105))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if (((!s.b[433]) && s.b[442]) && s.b[447]) {
            s.store_scaled_mul(107, 136, 91, p.p169);
            s.store_scaled_mul(108, 136, 91, p.p170);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 130, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_mul_scaled_powf_rhs(116, 136, p.p169, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p170, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if (((!s.b[433]) && s.b[442]) && s.b[447]) {
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(119))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if (((!s.b[433]) && s.b[442]) && s.b[447]) {
            s.store_mul_scaled_powf_rhs(121, 136, p.p169, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(122, 136, p.p170, 115, (-0.3333333333333333));
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.store_add(225, 128, 86);
        }

        if (((!s.b[433]) && s.b[442]) && (!s.b[447])) {
            s.store_add(225, 100, 86);
        }

        if ((!s.b[433]) && s.b[442]) {
            s.store_scaled_add(226, 224, 225, 0.5);
            s.store_sub(227, 225, 224);
            s.store_sub(90, 225, 224);
            s.store_add_scaled_inputs3_indices(91, 222, 1.0, 83, 1.0, 226, -1.0);
            s.store_mul_add_scaled_inputs3_offset_rhs(137, 223, s.ad_value(222), ((p.p4 * p.p5) * p.p161), s.ad_value(226), (((-1.0)) * (((p.p4 * p.p5) * p.p161))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p161), 0.0);
            s.store_scale(188, 137, (1.0 / (p.p236) * 1e26));
            s.store_offset_powf_ad(189, s.ad_value(188), p.p235, 1.0);
            s.store_div_from_scalar(190, p.p234, 189);
            s.store_div_from_scalar_offset_input(191, p.p9, 190, p.p160);
            s.store_mul_add_scaled_inputs3_offset_rhs(228, 191, s.ad_value(222), ((p.p4 * p.p5) * p.p161), s.ad_value(226), (((-1.0)) * (((p.p4 * p.p5) * p.p161))), A::div_scaled_product(s.ad_value(90), s.ad_value(90), 0.5, s.ad_value(91), 6.0), ((p.p4 * p.p5) * p.p161), 0.0);
            s.store_add_scaled_inputs3_indices(136, 222, 1.0, 83, 1.0, 226, -1.0);
            s.store_add_scaled_inputs(90, 224, 0.3333333333333333, 225, (2.0 * 0.3333333333333333));
            s.store_div_scaled_inputs_mixed_ai(91, A::square(s.ad_value(227)), (1.0 / 12.0), 136, 1.0);
            s.store_div_scaled_product_mixed_aia(137, A::square(s.ad_value(227)), 227, (1.0 / 120.0), A::square(s.ad_value(136)), 1.0);
            s.store_mul_add_scaled_inputs4_indices_rhs(229, 191, 222, (-(((p.p4 * p.p161) * p.p5) * 0.5)), 90, (((-1.0)) * ((-(((p.p4 * p.p161) * p.p5) * 0.5)))), 91, (-(((p.p4 * p.p161) * p.p5) * 0.5)), 137, (-(((p.p4 * p.p161) * p.p5) * 0.5)));
        }

        if ((!s.b[433]) && (!s.b[442])) {
            s.store_scalar(228, 0.0);
            s.store_scalar(229, 0.0);
        }

        s.b[448] = (p.p149 == 0.0);
        s.store_scalar(448, if s.b[448] { 1.0 } else { 0.0 });

        s.b[449] = (p.p151 != 0.0);
        s.store_scalar(449, if s.b[449] { 1.0 } else { 0.0 });

        if (s.b[448] && s.b[449]) {
            s.store_voltage(53, ctx, nodes, Some(8), Some(19));
        }

        s.b[450] = (p.p151 == 1.0);
        s.store_scalar(450, if s.b[450] { 1.0 } else { 0.0 });

        if ((s.b[448] && s.b[449]) && s.b[450]) {
            s.store_voltage(54, ctx, nodes, Some(9), Some(19));
            s.store_voltage(55, ctx, nodes, Some(9), Some(8));
        }

        if ((s.b[448] && s.b[449]) && (!s.b[450])) {
            s.store_voltage(54, ctx, nodes, Some(2), Some(19));
            s.store_voltage(55, ctx, nodes, Some(2), Some(8));
        }

        if (s.b[448] && s.b[449]) {
            s.store_scalar(52, 1.0);
        }

        s.b[451] = (s.v[53] < 0.0);
        s.store_scalar(451, if s.b[451] { 1.0 } else { 0.0 });

        if ((s.b[448] && s.b[449]) && s.b[451]) {
            s.store_scalar(52, (-1.0));
            s.store_mul(243, 52, 53);
        }

    }

    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[448] && s.b[449]) && s.b[451]) {
            s.copy_ad(242, 55);
        }

        if ((s.b[448] && s.b[449]) && (!s.b[451])) {
            s.copy_ad(243, 53);
            s.copy_ad(242, 54);
        }

        if (s.b[448] && s.b[449]) {
            s.store_offset_sqrt_ad(244, A::offset(A::square(s.ad_value(243)), 0.01), (-0.1));
            s.store_offset_scaled(146, 244, p.p166, (1.0 + p.p165));
            s.store_scaled_mul(83, 82, 146, 8.617087e-5);
            s.store_sub_ad(88, A::scale_offset(s.ad_value(82), ((1.0 / (s.v[35])) * (p.p162)), (((((-1.0)) * (p.p162))) + (p.p159))), A::div_scaled_inputs(s.ad_value(244), (p.p168 * p.p167), A::sqrt_square_offset(s.ad_value(244), (p.p168 * p.p168)), 1.0));
            s.store_scalar(235, (p.p9 / p.p160));
            s.store_div_from_scalar_scaled_mul(136, p.p161, 83, 83, (((2.0 * p.p4) * 1.602176634e-19) * 3.24e17));
            s.store_add_scaled_product_right_ad(159, 88, 1.0, 83, A::ln_scaled_input(s.ad_value(136), p.p158), 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(160, 242, 0.5, 159, ((-1.0) * 0.5), A::sqrt_square_offset(A::sub(s.ad_value(242), s.ad_value(159)), 0.0001), 0.5, 159, 1.0);
            s.store_sub(234, 160, 88);
            s.store_div_scaled_inputs_indices(84, 235, 1.0, 83, (1.602176634e-19 * 3.24e17));
            s.store_div_from_scalar(150, 2.718281828459045, 84);
            s.store_div_from_scalar(151, 1.0, 84);
            s.store_scale(99, 235, 6.241509074460763e18);
            s.store_scaled_add_sqrt_square_offset_rhs(154, 234, 234, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_sqrt_square_sum_denominator(155, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(130, 154, 151, 1.0, 154, 151, 1.0);
        }

        if (s.b[448] && s.b[449]) {
            let assign9530_ad_e14582: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(155)))), 1.0, assign9530_ad_e14582, (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(assign9530_ad_e14582, ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(130)), 1.0, 1.0), 1.0);
        }

        if (s.b[448] && s.b[449]) {
            s.store_div_scaled_inputs_indices(136, 234, 1.0, 83, 2.0);
        }

        s.b[452] = (s.v[136] < 200.0);
        s.store_scalar(452, if s.b[452] { 1.0 } else { 0.0 });

        if ((s.b[448] && s.b[449]) && s.b[452]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(153, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(234), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[448] && s.b[449]) && (!s.b[452])) {
            s.store_div_scaled_product3_mixed_iiia(153, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(234), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[448] && s.b[449]) {
            s.store_sub_div_rhs_indices(100, 234, 153, 99);
        }

        s.b[453] = ((((s.v[100] - s.v[234])) as f64).abs() > 1e-19);
        s.store_scalar(453, if s.b[453] { 1.0 } else { 0.0 });

        if ((s.b[448] && s.b[449]) && s.b[453]) {
            s.store_sub(101, 234, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p169);
            s.store_scaled_mul(103, 136, 90, p.p170);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if ((s.b[448] && s.b[449]) && s.b[453]) {
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(105))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if ((s.b[448] && s.b[449]) && s.b[453]) {
            s.store_scaled_mul(107, 136, 91, p.p169);
            s.store_scaled_mul(108, 136, 91, p.p170);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(109, 104, 107, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(110, 104, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(111, 105, 108, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(112, 105, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(113, 99, (-1.0), A::div(s.ad_value(109), s.ad_value(110)), (-1.0), A::div(s.ad_value(111), s.ad_value(112)), -1.0);
            s.store_sub_div_rhs_indices(114, 100, 106, 113);
            s.store_sub(115, 234, 114);
            s.store_scaled_add_sqrt_square_offset_rhs(115, 115, 115, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(137, 115, (-0.3333333333333333));
            s.store_mul_scaled_powf_rhs(116, 136, p.p169, 115, 0.6666666666666666);
            s.store_mul_scaled_powf_rhs(117, 136, p.p170, 115, 0.6666666666666666);
            s.store_sub_div_same_denominator(118, 114, 116, 83);
            s.store_sub_div_same_denominator(119, 114, 117, 83);
        }

        if ((s.b[448] && s.b[449]) && s.b[453]) {
            s.store_add_scaled_products3(120, s.ad_value(99), s.ad_value(115), 1.0, s.ad_value(83), {
                if ((!(s.v[118] >= 37.0)) && (!(s.v[118] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(118))
                } else {
                    {
                        if ((!(s.v[118] >= 37.0)) && (s.v[118] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[118] >= 37.0) {
                                    s.ad_value(118)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[119] >= 37.0)) && (!(s.v[119] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(119))
                } else {
                    {
                        if ((!(s.v[119] >= 37.0)) && (s.v[119] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[119] >= 37.0) {
                                    s.ad_value(119)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if ((s.b[448] && s.b[449]) && s.b[453]) {
            s.store_scaled_mul(121, 136, 137, p.p169);
            s.store_scaled_mul(122, 136, 137, p.p170);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(123, 118, 121, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(124, 118, 1.0);
            s.store_scaled_mul_limited_exp_scale_offset_rhs(125, 119, 122, 0.6666666666666666, 1.0, 3.24e17);
            s.store_offset_limited_exp(126, 119, 1.0);
            s.store_add_scaled_inputs3_mixed_iaa(127, 99, (-1.0), A::div(s.ad_value(123), s.ad_value(124)), (-1.0), A::div(s.ad_value(125), s.ad_value(126)), -1.0);
            s.store_sub_div_rhs_indices(128, 114, 120, 127);
            s.copy_ad(236, 128);
        }

        if ((s.b[448] && s.b[449]) && (!s.b[453])) {
            s.copy_ad(236, 100);
        }

        if (s.b[448] && s.b[449]) {
            s.store_scaled_powf_ad(97, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p20, p.p163);
            s.store_scaled_powf_ad(89, A::scale(s.ad_value(82), 1.0 / (s.v[35])), p.p19, p.p164);
            s.store_mul_scaled_abs_ad_rhs(136, 235, 1.0 / (p.p9), A::sub(s.ad_value(234), s.ad_value(236)));
            s.store_scaled_abs_ad(90, A::sub(s.ad_value(45), s.ad_value(236)), (s.v[81] / p.p9));
            s.store_div_ad_rhs(95, 97, A::add_scaled_inputs3_offset(s.ad_value(136), p.p14, A::square(s.ad_value(136)), p.p15, s.ad_value(90), p.p16, 1.0));
            s.store_div_scaled_inputs_indices(136, 89, 2.0, 95, 1.0);
            s.store_scaled_add_sqrt_square_offset_rhs(90, 234, 234, ((4.0 * 0.3) * 0.3), 0.5);
            s.store_div_scaled_product_add_scaled_denominator_indices(85, 136, 90, p.p161, 136, p.p161, 90, 1.0, 1.0);
            s.store_powf_ad(136, A::div(s.ad_value(243), s.ad_value(85)), p.p18);
            s.store_powf_offset_input(90, 136, 1.0, ((-1.0) / p.p18));
            s.store_mul(86, 243, 90);
            s.store_sub(39, 234, 86);
            s.copy_ad(130, 39);
            s.store_scaled_add_sqrt_square_offset_rhs(131, 130, 130, ((4.0 * 0.3) * 0.3), 0.5);
            s.copy_ad(154, 131);
            s.store_div_scaled_product_sqrt_square_sum_denominator(157, 154, 150, 1.0, 154, 150, 1.0);
            s.store_div_scaled_product_sqrt_square_sum_denominator(158, 154, 151, 1.0, 154, 151, 1.0);
        }

        if (s.b[448] && s.b[449]) {
            let assign10150_ad_e15611: A = A::powf(A::mul(s.ad_value(99), s.ad_value(154)), 0.6666666666666666);
            s.store_div_scaled_inputs3_mixed_iaaa(152, 154, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(83), 1.0, A::ln(A::mul(s.ad_value(84), s.ad_value(157)))), 1.0, assign10150_ad_e15611, (-(p.p169 / 3.0)), A::add_scaled_offset_product_rhs(assign10150_ad_e15611, ((2.0 * p.p169) / 3.0), s.ad_value(154), A::div(s.ad_value(83), s.ad_value(158)), 1.0, 1.0), 1.0);
        }

        if (s.b[448] && s.b[449]) {
            s.store_div_scaled_inputs_indices(136, 130, 1.0, 83, 2.0);
        }

        s.b[454] = (s.v[136] < 200.0);
        s.store_scalar(454, if s.b[454] { 1.0 } else { 0.0 });

        if ((s.b[448] && s.b[449]) && s.b[454]) {
            s.store_limited_exp_scaled_input(90, 136, 1.0 / (4.0));
            s.store_limited_exp_scaled_input(91, 136, ((-3.0) * 1.0 / (4.0)));
            s.store_div_scaled_product3_mixed_iiaa(156, 83, 99, A::add_scaled_inputs(s.ad_value(136), (3.0 * 0.25), A::ln(A::add(s.ad_value(90), s.ad_value(91))), 1.0), 2.0, A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if ((s.b[448] && s.b[449]) && (!s.b[454])) {
            s.store_div_scaled_product3_mixed_iiia(156, 83, 99, 136, (2.0 * 1.0 / (1.0)), A::add_scaled_product(A::div_from_scalar(1.0, s.ad_value(152)), 1.0, s.ad_value(99), A::limited_exp_div_scaled_inputs(s.ad_value(130), (-1.0), s.ad_value(83), 2.0), 1.0 / (3.24e17)), 1.0);
        }

        if (s.b[448] && s.b[449]) {
            s.store_sub_div_rhs_indices(100, 130, 156, 99);
        }

        s.b[455] = ((((s.v[100] - s.v[130])) as f64).abs() > 1e-19);
        s.store_scalar(455, if s.b[455] { 1.0 } else { 0.0 });

        if ((s.b[448] && s.b[449]) && s.b[455]) {
            s.store_sub(101, 130, 100);
            s.store_scaled_add_sqrt_square_offset_rhs(101, 101, 101, ((4.0 * 1e-9) * 1e-9), 0.5);
            s.store_powf(136, 99, 0.6666666666666666);
            s.store_powf(90, 101, 0.6666666666666666);
            s.store_powf(91, 101, (-0.3333333333333333));
            s.store_scaled_mul(102, 136, 90, p.p169);
            s.store_scaled_mul(103, 136, 90, p.p170);
            s.store_sub_div_same_denominator(104, 100, 102, 83);
            s.store_sub_div_same_denominator(105, 100, 103, 83);
        }

        if ((s.b[448] && s.b[449]) && s.b[455]) {
            s.store_add_scaled_products3(106, s.ad_value(99), s.ad_value(101), 1.0, s.ad_value(83), {
                if ((!(s.v[104] >= 37.0)) && (!(s.v[104] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(104))
                } else {
                    {
                        if ((!(s.v[104] >= 37.0)) && (s.v[104] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[104] >= 37.0) {
                                    s.ad_value(104)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17), s.ad_value(83), {
                if ((!(s.v[105] >= 37.0)) && (!(s.v[105] <= (-37.0)))) {
                    A::ln_one_plus_exp(s.ad_value(105))
                } else {
                    {
                        if ((!(s.v[105] >= 37.0)) && (s.v[105] <= (-37.0))) {
                            A::constant(0.0)
                        } else {
                            {
                                if (s.v[105] >= 37.0) {
                                    s.ad_value(105)
                                } else {
                                    A::constant(0.0)
                                }
                            }
                        }
                    }
                }
            }, (-3.24e17));
        }

        if ((s.b[448] && s.b[449]) && s.b[455]) {
            s.store_scaled_mul(107, 136, 91, p.p169);
        }

    }
}
