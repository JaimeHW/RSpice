#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_214(
        var_arg: f64,
        var_arg_dn0: f64,
        var_arg_dn10: f64,
        var_arg_dn13: f64,
        var_arg_dn2: f64,
        var_arg_dn4: f64,
        var_arg_dn5: f64,
        var_arg_dn6: f64,
        var_arg_dn7: f64,
        var_arg_dn8: f64,
        var_arg_dn9: f64,
        var_guard1430: f64,
        var_guard1461: f64,
        var_guard1462: f64,
        var_guard1463: f64,
        var_guard443: f64,
        var_ps0: f64,
        var_ps0_dn0: f64,
        var_ps0_dn10: f64,
        var_ps0_dn13: f64,
        var_ps0_dn2: f64,
        var_ps0_dn4: f64,
        var_ps0_dn5: f64,
        var_ps0_dn6: f64,
        var_ps0_dn7: f64,
        var_ps0_dn8: f64,
        var_ps0_dn9: f64,
        var_psl: f64,
        var_psl_dn0: f64,
        var_psl_dn10: f64,
        var_psl_dn13: f64,
        var_psl_dn2: f64,
        var_psl_dn4: f64,
        var_psl_dn5: f64,
        var_psl_dn6: f64,
        var_psl_dn7: f64,
        var_psl_dn8: f64,
        var_psl_dn9: f64,
        var_q_nsub: f64,
        var_q_nsub_dn0: f64,
        var_q_nsub_dn10: f64,
        var_q_nsub_dn13: f64,
        var_q_nsub_dn2: f64,
        var_q_nsub_dn4: f64,
        var_q_nsub_dn5: f64,
        var_q_nsub_dn6: f64,
        var_q_nsub_dn7: f64,
        var_q_nsub_dn8: f64,
        var_q_nsub_dn9: f64,
        var_qn0: f64,
        var_qn0_dn0: f64,
        var_qn0_dn10: f64,
        var_qn0_dn13: f64,
        var_qn0_dn2: f64,
        var_qn0_dn4: f64,
        var_qn0_dn5: f64,
        var_qn0_dn6: f64,
        var_qn0_dn7: f64,
        var_qn0_dn8: f64,
        var_qn0_dn9: f64,
        var_tmf1: f64,
        var_tmf1_dn0: f64,
        var_tmf1_dn10: f64,
        var_tmf1_dn13: f64,
        var_tmf1_dn2: f64,
        var_tmf1_dn4: f64,
        var_tmf1_dn5: f64,
        var_tmf1_dn6: f64,
        var_tmf1_dn7: f64,
        var_tmf1_dn8: f64,
        var_tmf1_dn9: f64,
        var_uc_clm2: f64,
        var_uc_clm2_dn0: f64,
        var_uc_clm2_dn10: f64,
        var_uc_clm2_dn13: f64,
        var_uc_clm2_dn2: f64,
        var_uc_clm2_dn4: f64,
        var_uc_clm2_dn5: f64,
        var_uc_clm2_dn6: f64,
        var_uc_clm2_dn7: f64,
        var_uc_clm2_dn8: f64,
        var_uc_clm2_dn9: f64,
        var_uc_clm3: f64,
        var_vbscl__blk435: f64,
        var_vbscl__blk435_dn0: f64,
        var_vbscl__blk435_dn10: f64,
        var_vbscl__blk435_dn13: f64,
        var_vbscl__blk435_dn2: f64,
        var_vbscl__blk435_dn4: f64,
        var_vbscl__blk435_dn5: f64,
        var_vbscl__blk435_dn6: f64,
        var_vbscl__blk435_dn7: f64,
        var_vbscl__blk435_dn8: f64,
        var_vbscl__blk435_dn9: f64,
        var_vds: f64,
        var_vds_dn0: f64,
        var_vds_dn10: f64,
        var_vds_dn13: f64,
        var_vds_dn2: f64,
        var_vds_dn4: f64,
        var_vds_dn5: f64,
        var_vds_dn6: f64,
        var_vds_dn7: f64,
        var_vds_dn8: f64,
        var_vds_dn9: f64,
        var_wdpl: f64,
        var_wdpl_dn0: f64,
        var_wdpl_dn10: f64,
        var_wdpl_dn13: f64,
        var_wdpl_dn2: f64,
        var_wdpl_dn4: f64,
        var_wdpl_dn5: f64,
        var_wdpl_dn6: f64,
        var_wdpl_dn7: f64,
        var_wdpl_dn8: f64,
        var_wdpl_dn9: f64,
        var_xmp: f64,
        var_xmp_dn0: f64,
        var_xmp_dn10: f64,
        var_xmp_dn13: f64,
        var_xmp_dn2: f64,
        var_xmp_dn4: f64,
        var_xmp_dn5: f64,
        var_xmp_dn6: f64,
        var_xmp_dn7: f64,
        var_xmp_dn8: f64,
        var_xmp_dn9: f64,
        var_dnm_slot: &mut f64,
        var_dnm_dn0_slot: &mut f64,
        var_dnm_dn10_slot: &mut f64,
        var_dnm_dn13_slot: &mut f64,
        var_dnm_dn2_slot: &mut f64,
        var_dnm_dn4_slot: &mut f64,
        var_dnm_dn5_slot: &mut f64,
        var_dnm_dn6_slot: &mut f64,
        var_dnm_dn7_slot: &mut f64,
        var_dnm_dn8_slot: &mut f64,
        var_dnm_dn9_slot: &mut f64,
        var_dnm_rv_slot: &mut f64,
        var_guard1464_slot: &mut f64,
        var_guard1464_rv_slot: &mut f64,
        var_guard1465_slot: &mut f64,
        var_guard1465_rv_slot: &mut f64,
        var_guard1466_slot: &mut f64,
        var_guard1466_rv_slot: &mut f64,
        var_guard1467_slot: &mut f64,
        var_guard1467_rv_slot: &mut f64,
        var_guard1468_slot: &mut f64,
        var_guard1468_rv_slot: &mut f64,
        var_m0_slot: &mut f64,
        var_m0_rv_slot: &mut f64,
        var_mm_slot: &mut f64,
        var_mm_rv_slot: &mut f64,
        var_psdl_slot: &mut f64,
        var_psdl_dn0_slot: &mut f64,
        var_psdl_dn10_slot: &mut f64,
        var_psdl_dn13_slot: &mut f64,
        var_psdl_dn2_slot: &mut f64,
        var_psdl_dn4_slot: &mut f64,
        var_psdl_dn5_slot: &mut f64,
        var_psdl_dn6_slot: &mut f64,
        var_psdl_dn7_slot: &mut f64,
        var_psdl_dn8_slot: &mut f64,
        var_psdl_dn9_slot: &mut f64,
        var_psdl_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn13_slot: &mut f64,
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
        var_t1_dn13_slot: &mut f64,
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
        var_t2_dn13_slot: &mut f64,
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
        var_t3_dn13_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn10_slot: &mut f64,
        var_t5_dn13_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t5_dn9_slot: &mut f64,
        var_t5_rv_slot: &mut f64,
        var_t8_slot: &mut f64,
        var_t8_dn0_slot: &mut f64,
        var_t8_dn10_slot: &mut f64,
        var_t8_dn13_slot: &mut f64,
        var_t8_dn2_slot: &mut f64,
        var_t8_dn4_slot: &mut f64,
        var_t8_dn5_slot: &mut f64,
        var_t8_dn6_slot: &mut f64,
        var_t8_dn7_slot: &mut f64,
        var_t8_dn8_slot: &mut f64,
        var_t8_dn9_slot: &mut f64,
        var_t8_rv_slot: &mut f64,
        var_t9_slot: &mut f64,
        var_t9_dn0_slot: &mut f64,
        var_t9_dn10_slot: &mut f64,
        var_t9_dn13_slot: &mut f64,
        var_t9_dn2_slot: &mut f64,
        var_t9_dn4_slot: &mut f64,
        var_t9_dn5_slot: &mut f64,
        var_t9_dn6_slot: &mut f64,
        var_t9_dn7_slot: &mut f64,
        var_t9_dn8_slot: &mut f64,
        var_t9_dn9_slot: &mut f64,
        var_t9_rv_slot: &mut f64,
        var_tmf0_slot: &mut f64,
        var_tmf0_dn0_slot: &mut f64,
        var_tmf0_dn10_slot: &mut f64,
        var_tmf0_dn13_slot: &mut f64,
        var_tmf0_dn2_slot: &mut f64,
        var_tmf0_dn4_slot: &mut f64,
        var_tmf0_dn5_slot: &mut f64,
        var_tmf0_dn6_slot: &mut f64,
        var_tmf0_dn7_slot: &mut f64,
        var_tmf0_dn8_slot: &mut f64,
        var_tmf0_dn9_slot: &mut f64,
        var_tmf0_rv_slot: &mut f64,
        var_wd_slot: &mut f64,
        var_wd_dn0_slot: &mut f64,
        var_wd_dn10_slot: &mut f64,
        var_wd_dn13_slot: &mut f64,
        var_wd_dn2_slot: &mut f64,
        var_wd_dn4_slot: &mut f64,
        var_wd_dn5_slot: &mut f64,
        var_wd_dn6_slot: &mut f64,
        var_wd_dn7_slot: &mut f64,
        var_wd_dn8_slot: &mut f64,
        var_wd_dn9_slot: &mut f64,
        var_wd_rv_slot: &mut f64,
    ) {
        let mut var_dnm: f64 = *var_dnm_slot;
        let mut var_dnm_dn0: f64 = *var_dnm_dn0_slot;
        let mut var_dnm_dn10: f64 = *var_dnm_dn10_slot;
        let mut var_dnm_dn13: f64 = *var_dnm_dn13_slot;
        let mut var_dnm_dn2: f64 = *var_dnm_dn2_slot;
        let mut var_dnm_dn4: f64 = *var_dnm_dn4_slot;
        let mut var_dnm_dn5: f64 = *var_dnm_dn5_slot;
        let mut var_dnm_dn6: f64 = *var_dnm_dn6_slot;
        let mut var_dnm_dn7: f64 = *var_dnm_dn7_slot;
        let mut var_dnm_dn8: f64 = *var_dnm_dn8_slot;
        let mut var_dnm_dn9: f64 = *var_dnm_dn9_slot;
        let mut var_dnm_rv: f64 = *var_dnm_rv_slot;
        let mut var_guard1464: f64 = *var_guard1464_slot;
        let mut var_guard1464_rv: f64 = *var_guard1464_rv_slot;
        let mut var_guard1465: f64 = *var_guard1465_slot;
        let mut var_guard1465_rv: f64 = *var_guard1465_rv_slot;
        let mut var_guard1466: f64 = *var_guard1466_slot;
        let mut var_guard1466_rv: f64 = *var_guard1466_rv_slot;
        let mut var_guard1467: f64 = *var_guard1467_slot;
        let mut var_guard1467_rv: f64 = *var_guard1467_rv_slot;
        let mut var_guard1468: f64 = *var_guard1468_slot;
        let mut var_guard1468_rv: f64 = *var_guard1468_rv_slot;
        let mut var_m0: f64 = *var_m0_slot;
        let mut var_m0_rv: f64 = *var_m0_rv_slot;
        let mut var_mm: f64 = *var_mm_slot;
        let mut var_mm_rv: f64 = *var_mm_rv_slot;
        let mut var_psdl: f64 = *var_psdl_slot;
        let mut var_psdl_dn0: f64 = *var_psdl_dn0_slot;
        let mut var_psdl_dn10: f64 = *var_psdl_dn10_slot;
        let mut var_psdl_dn13: f64 = *var_psdl_dn13_slot;
        let mut var_psdl_dn2: f64 = *var_psdl_dn2_slot;
        let mut var_psdl_dn4: f64 = *var_psdl_dn4_slot;
        let mut var_psdl_dn5: f64 = *var_psdl_dn5_slot;
        let mut var_psdl_dn6: f64 = *var_psdl_dn6_slot;
        let mut var_psdl_dn7: f64 = *var_psdl_dn7_slot;
        let mut var_psdl_dn8: f64 = *var_psdl_dn8_slot;
        let mut var_psdl_dn9: f64 = *var_psdl_dn9_slot;
        let mut var_psdl_rv: f64 = *var_psdl_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn13: f64 = *var_t0_dn13_slot;
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
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
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
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
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
        let mut var_t3_dn13: f64 = *var_t3_dn13_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn10: f64 = *var_t5_dn10_slot;
        let mut var_t5_dn13: f64 = *var_t5_dn13_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t5_dn9: f64 = *var_t5_dn9_slot;
        let mut var_t5_rv: f64 = *var_t5_rv_slot;
        let mut var_t8: f64 = *var_t8_slot;
        let mut var_t8_dn0: f64 = *var_t8_dn0_slot;
        let mut var_t8_dn10: f64 = *var_t8_dn10_slot;
        let mut var_t8_dn13: f64 = *var_t8_dn13_slot;
        let mut var_t8_dn2: f64 = *var_t8_dn2_slot;
        let mut var_t8_dn4: f64 = *var_t8_dn4_slot;
        let mut var_t8_dn5: f64 = *var_t8_dn5_slot;
        let mut var_t8_dn6: f64 = *var_t8_dn6_slot;
        let mut var_t8_dn7: f64 = *var_t8_dn7_slot;
        let mut var_t8_dn8: f64 = *var_t8_dn8_slot;
        let mut var_t8_dn9: f64 = *var_t8_dn9_slot;
        let mut var_t8_rv: f64 = *var_t8_rv_slot;
        let mut var_t9: f64 = *var_t9_slot;
        let mut var_t9_dn0: f64 = *var_t9_dn0_slot;
        let mut var_t9_dn10: f64 = *var_t9_dn10_slot;
        let mut var_t9_dn13: f64 = *var_t9_dn13_slot;
        let mut var_t9_dn2: f64 = *var_t9_dn2_slot;
        let mut var_t9_dn4: f64 = *var_t9_dn4_slot;
        let mut var_t9_dn5: f64 = *var_t9_dn5_slot;
        let mut var_t9_dn6: f64 = *var_t9_dn6_slot;
        let mut var_t9_dn7: f64 = *var_t9_dn7_slot;
        let mut var_t9_dn8: f64 = *var_t9_dn8_slot;
        let mut var_t9_dn9: f64 = *var_t9_dn9_slot;
        let mut var_t9_rv: f64 = *var_t9_rv_slot;
        let mut var_tmf0: f64 = *var_tmf0_slot;
        let mut var_tmf0_dn0: f64 = *var_tmf0_dn0_slot;
        let mut var_tmf0_dn10: f64 = *var_tmf0_dn10_slot;
        let mut var_tmf0_dn13: f64 = *var_tmf0_dn13_slot;
        let mut var_tmf0_dn2: f64 = *var_tmf0_dn2_slot;
        let mut var_tmf0_dn4: f64 = *var_tmf0_dn4_slot;
        let mut var_tmf0_dn5: f64 = *var_tmf0_dn5_slot;
        let mut var_tmf0_dn6: f64 = *var_tmf0_dn6_slot;
        let mut var_tmf0_dn7: f64 = *var_tmf0_dn7_slot;
        let mut var_tmf0_dn8: f64 = *var_tmf0_dn8_slot;
        let mut var_tmf0_dn9: f64 = *var_tmf0_dn9_slot;
        let mut var_tmf0_rv: f64 = *var_tmf0_rv_slot;
        let mut var_wd: f64 = *var_wd_slot;
        let mut var_wd_dn0: f64 = *var_wd_dn0_slot;
        let mut var_wd_dn10: f64 = *var_wd_dn10_slot;
        let mut var_wd_dn13: f64 = *var_wd_dn13_slot;
        let mut var_wd_dn2: f64 = *var_wd_dn2_slot;
        let mut var_wd_dn4: f64 = *var_wd_dn4_slot;
        let mut var_wd_dn5: f64 = *var_wd_dn5_slot;
        let mut var_wd_dn6: f64 = *var_wd_dn6_slot;
        let mut var_wd_dn7: f64 = *var_wd_dn7_slot;
        let mut var_wd_dn8: f64 = *var_wd_dn8_slot;
        let mut var_wd_dn9: f64 = *var_wd_dn9_slot;
        let mut var_wd_rv: f64 = *var_wd_rv_slot;

        let (assign59890_e93425, assign59890_e93425_d_n0, assign59890_e93425_d_n2, assign59890_e93425_d_n4, assign59890_e93425_d_n5, assign59890_e93425_d_n6, assign59890_e93425_d_n7, assign59890_e93425_d_n8, assign59890_e93425_d_n9, assign59890_e93425_d_n10, assign59890_e93425_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 != 0.0)) && (var_guard1463 != 0.0)) {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_dn10, var_arg_dn13,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
        var_dnm = assign59890_e93425;
        var_dnm_dn0 = assign59890_e93425_d_n0;
        var_dnm_dn2 = assign59890_e93425_d_n2;
        var_dnm_dn4 = assign59890_e93425_d_n4;
        var_dnm_dn5 = assign59890_e93425_d_n5;
        var_dnm_dn6 = assign59890_e93425_d_n6;
        var_dnm_dn7 = assign59890_e93425_d_n7;
        var_dnm_dn8 = assign59890_e93425_d_n8;
        var_dnm_dn9 = assign59890_e93425_d_n9;
        var_dnm_dn10 = assign59890_e93425_d_n10;
        var_dnm_dn13 = assign59890_e93425_d_n13;
        var_dnm_rv = 0.0;

        let assign59900_e93440: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        var_guard1464 = assign59900_e93440;
        var_guard1464_rv = 0.0;

        let assign59910_e93443: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard1465 = assign59910_e93443;
        var_guard1465_rv = 0.0;

        let (assign59920_e93460,) = {
    if (((((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 != 0.0)) && (var_guard1463 != 0.0)) && (var_guard1464 != 0.0)) && (var_guard1465 != 0.0)) {
        (1.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign59920_e93460;
        var_mm_rv = 0.0;

        let assign59930_e93463: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        var_guard1466 = assign59930_e93463;
        var_guard1466_rv = 0.0;

        let (assign59940_e93483,) = {
    if ((((((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 != 0.0)) && (var_guard1463 != 0.0)) && (var_guard1464 != 0.0)) && (var_guard1465 == 0.0)) && (var_guard1466 != 0.0)) {
        (2.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign59940_e93483;
        var_mm_rv = 0.0;

        let assign59950_e93486: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        var_guard1467 = assign59950_e93486;
        var_guard1467_rv = 0.0;

        let (assign59960_e93509,) = {
    if (((((((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 != 0.0)) && (var_guard1463 != 0.0)) && (var_guard1464 != 0.0)) && (var_guard1465 == 0.0)) && (var_guard1466 == 0.0)) && (var_guard1467 != 0.0)) {
        (3.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign59960_e93509;
        var_mm_rv = 0.0;

        let assign59970_e93512: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        var_guard1468 = assign59970_e93512;
        var_guard1468_rv = 0.0;

        let (assign59980_e93538,) = {
    if ((((((((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 != 0.0)) && (var_guard1463 != 0.0)) && (var_guard1464 != 0.0)) && (var_guard1465 == 0.0)) && (var_guard1466 == 0.0)) && (var_guard1467 == 0.0)) && (var_guard1468 != 0.0)) {
        (4.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign59980_e93538;
        var_mm_rv = 0.0;

        let (assign59990_e93553,) = {
    if ((((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 != 0.0)) && (var_guard1463 != 0.0)) && (var_guard1464 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign59990_e93553;
        var_m0_rv = 0.0;

        let mut assign60000_loop_guard: usize = 0;
        while {
            let assign60000_cond_e93569: f64 = if (((((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 != 0.0)) && (var_guard1463 != 0.0)) && (var_guard1464 != 0.0)) && (var_m0 < var_mm)) { 1.0 } else { 0.0 };
            assign60000_cond_e93569 != 0.0
        } {
            assign60000_loop_guard += 1;
            assert!(assign60000_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign60000_body0_e93585, assign60000_body0_e93585_d_n0, assign60000_body0_e93585_d_n2, assign60000_body0_e93585_d_n4, assign60000_body0_e93585_d_n5, assign60000_body0_e93585_d_n6, assign60000_body0_e93585_d_n7, assign60000_body0_e93585_d_n8, assign60000_body0_e93585_d_n9, assign60000_body0_e93585_d_n10, assign60000_body0_e93585_d_n13,) = {
    if ((((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 != 0.0)) && (var_guard1463 != 0.0)) && (var_guard1464 != 0.0)) {
        let assign60000_body0_e93583: f64 = (var_dnm).sqrt();
        (assign60000_body0_e93583, (var_dnm_dn0 / (2.0 * assign60000_body0_e93583)), (var_dnm_dn2 / (2.0 * assign60000_body0_e93583)), (var_dnm_dn4 / (2.0 * assign60000_body0_e93583)), (var_dnm_dn5 / (2.0 * assign60000_body0_e93583)), (var_dnm_dn6 / (2.0 * assign60000_body0_e93583)), (var_dnm_dn7 / (2.0 * assign60000_body0_e93583)), (var_dnm_dn8 / (2.0 * assign60000_body0_e93583)), (var_dnm_dn9 / (2.0 * assign60000_body0_e93583)), (var_dnm_dn10 / (2.0 * assign60000_body0_e93583)), (var_dnm_dn13 / (2.0 * assign60000_body0_e93583)),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
            var_dnm = assign60000_body0_e93585;
            var_dnm_dn0 = assign60000_body0_e93585_d_n0;
            var_dnm_dn2 = assign60000_body0_e93585_d_n2;
            var_dnm_dn4 = assign60000_body0_e93585_d_n4;
            var_dnm_dn5 = assign60000_body0_e93585_d_n5;
            var_dnm_dn6 = assign60000_body0_e93585_d_n6;
            var_dnm_dn7 = assign60000_body0_e93585_d_n7;
            var_dnm_dn8 = assign60000_body0_e93585_d_n8;
            var_dnm_dn9 = assign60000_body0_e93585_d_n9;
            var_dnm_dn10 = assign60000_body0_e93585_d_n10;
            var_dnm_dn13 = assign60000_body0_e93585_d_n13;
            var_dnm_rv = 0.0;
            let (assign60000_body1_e93602,) = {
    if ((((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 != 0.0)) && (var_guard1463 != 0.0)) && (var_guard1464 != 0.0)) {
        let assign60000_body1_e93600: f64 = (var_m0 + 1.0);
        (assign60000_body1_e93600,)
    } else {
        (var_m0,)
    }
};
            var_m0 = assign60000_body1_e93602;
            var_m0_rv = 0.0;
        }

        let (assign60010_e93629, assign60010_e93629_d_n0, assign60010_e93629_d_n2, assign60010_e93629_d_n4, assign60010_e93629_d_n5, assign60010_e93629_d_n6, assign60010_e93629_d_n7, assign60010_e93629_d_n8, assign60010_e93629_d_n9, assign60010_e93629_d_n10, assign60010_e93629_d_n13,) = {
    if ((((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 != 0.0)) && (var_guard1463 != 0.0)) && (var_guard1464 == 0.0)) {
        let (assign60010_e93627, assign60010_e93627_d_n0, assign60010_e93627_d_n2, assign60010_e93627_d_n4, assign60010_e93627_d_n5, assign60010_e93627_d_n6, assign60010_e93627_d_n7, assign60010_e93627_d_n8, assign60010_e93627_d_n9, assign60010_e93627_d_n10, assign60010_e93627_d_n13,) = {
            if (var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign60010_e93624: f64 = (2.0 * 2.0);
                let assign60010_e93625: f64 = (1.0 / assign60010_e93624);
                let assign60010_e93626: f64 = (var_dnm).powf(assign60010_e93625);
                (assign60010_e93626, if 0.0 == 0.0 && ((assign60010_e93625) as f64).is_finite() && ((assign60010_e93625) as f64).fract() == 0.0 { if assign60010_e93625 == 0.0 { 0.0 } else { (assign60010_e93625 * ((var_dnm).powf(assign60010_e93625 - 1.0) * var_dnm_dn0)) } } else { (assign60010_e93626 * (assign60010_e93625 * (var_dnm_dn0 / var_dnm))) }, if 0.0 == 0.0 && ((assign60010_e93625) as f64).is_finite() && ((assign60010_e93625) as f64).fract() == 0.0 { if assign60010_e93625 == 0.0 { 0.0 } else { (assign60010_e93625 * ((var_dnm).powf(assign60010_e93625 - 1.0) * var_dnm_dn2)) } } else { (assign60010_e93626 * (assign60010_e93625 * (var_dnm_dn2 / var_dnm))) }, if 0.0 == 0.0 && ((assign60010_e93625) as f64).is_finite() && ((assign60010_e93625) as f64).fract() == 0.0 { if assign60010_e93625 == 0.0 { 0.0 } else { (assign60010_e93625 * ((var_dnm).powf(assign60010_e93625 - 1.0) * var_dnm_dn4)) } } else { (assign60010_e93626 * (assign60010_e93625 * (var_dnm_dn4 / var_dnm))) }, if 0.0 == 0.0 && ((assign60010_e93625) as f64).is_finite() && ((assign60010_e93625) as f64).fract() == 0.0 { if assign60010_e93625 == 0.0 { 0.0 } else { (assign60010_e93625 * ((var_dnm).powf(assign60010_e93625 - 1.0) * var_dnm_dn5)) } } else { (assign60010_e93626 * (assign60010_e93625 * (var_dnm_dn5 / var_dnm))) }, if 0.0 == 0.0 && ((assign60010_e93625) as f64).is_finite() && ((assign60010_e93625) as f64).fract() == 0.0 { if assign60010_e93625 == 0.0 { 0.0 } else { (assign60010_e93625 * ((var_dnm).powf(assign60010_e93625 - 1.0) * var_dnm_dn6)) } } else { (assign60010_e93626 * (assign60010_e93625 * (var_dnm_dn6 / var_dnm))) }, if 0.0 == 0.0 && ((assign60010_e93625) as f64).is_finite() && ((assign60010_e93625) as f64).fract() == 0.0 { if assign60010_e93625 == 0.0 { 0.0 } else { (assign60010_e93625 * ((var_dnm).powf(assign60010_e93625 - 1.0) * var_dnm_dn7)) } } else { (assign60010_e93626 * (assign60010_e93625 * (var_dnm_dn7 / var_dnm))) }, if 0.0 == 0.0 && ((assign60010_e93625) as f64).is_finite() && ((assign60010_e93625) as f64).fract() == 0.0 { if assign60010_e93625 == 0.0 { 0.0 } else { (assign60010_e93625 * ((var_dnm).powf(assign60010_e93625 - 1.0) * var_dnm_dn8)) } } else { (assign60010_e93626 * (assign60010_e93625 * (var_dnm_dn8 / var_dnm))) }, if 0.0 == 0.0 && ((assign60010_e93625) as f64).is_finite() && ((assign60010_e93625) as f64).fract() == 0.0 { if assign60010_e93625 == 0.0 { 0.0 } else { (assign60010_e93625 * ((var_dnm).powf(assign60010_e93625 - 1.0) * var_dnm_dn9)) } } else { (assign60010_e93626 * (assign60010_e93625 * (var_dnm_dn9 / var_dnm))) }, if 0.0 == 0.0 && ((assign60010_e93625) as f64).is_finite() && ((assign60010_e93625) as f64).fract() == 0.0 { if assign60010_e93625 == 0.0 { 0.0 } else { (assign60010_e93625 * ((var_dnm).powf(assign60010_e93625 - 1.0) * var_dnm_dn10)) } } else { (assign60010_e93626 * (assign60010_e93625 * (var_dnm_dn10 / var_dnm))) }, if 0.0 == 0.0 && ((assign60010_e93625) as f64).is_finite() && ((assign60010_e93625) as f64).fract() == 0.0 { if assign60010_e93625 == 0.0 { 0.0 } else { (assign60010_e93625 * ((var_dnm).powf(assign60010_e93625 - 1.0) * var_dnm_dn13)) } } else { (assign60010_e93626 * (assign60010_e93625 * (var_dnm_dn13 / var_dnm))) },)
            }
        };
        (assign60010_e93627, assign60010_e93627_d_n0, assign60010_e93627_d_n2, assign60010_e93627_d_n4, assign60010_e93627_d_n5, assign60010_e93627_d_n6, assign60010_e93627_d_n7, assign60010_e93627_d_n8, assign60010_e93627_d_n9, assign60010_e93627_d_n10, assign60010_e93627_d_n13,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
        var_dnm = assign60010_e93629;
        var_dnm_dn0 = assign60010_e93629_d_n0;
        var_dnm_dn2 = assign60010_e93629_d_n2;
        var_dnm_dn4 = assign60010_e93629_d_n4;
        var_dnm_dn5 = assign60010_e93629_d_n5;
        var_dnm_dn6 = assign60010_e93629_d_n6;
        var_dnm_dn7 = assign60010_e93629_d_n7;
        var_dnm_dn8 = assign60010_e93629_d_n8;
        var_dnm_dn9 = assign60010_e93629_d_n9;
        var_dnm_dn10 = assign60010_e93629_d_n10;
        var_dnm_dn13 = assign60010_e93629_d_n13;
        var_dnm_rv = 0.0;

        let (assign60020_e93644, assign60020_e93644_d_n0, assign60020_e93644_d_n2, assign60020_e93644_d_n4, assign60020_e93644_d_n5, assign60020_e93644_d_n6, assign60020_e93644_d_n7, assign60020_e93644_d_n8, assign60020_e93644_d_n9, assign60020_e93644_d_n10, assign60020_e93644_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 != 0.0)) && (var_guard1463 != 0.0)) {
        let assign60020_e93642: f64 = (1.0 / var_dnm);
        (assign60020_e93642, (-(var_dnm_dn0 / (var_dnm * var_dnm))), (-(var_dnm_dn2 / (var_dnm * var_dnm))), (-(var_dnm_dn4 / (var_dnm * var_dnm))), (-(var_dnm_dn5 / (var_dnm * var_dnm))), (-(var_dnm_dn6 / (var_dnm * var_dnm))), (-(var_dnm_dn7 / (var_dnm * var_dnm))), (-(var_dnm_dn8 / (var_dnm * var_dnm))), (-(var_dnm_dn9 / (var_dnm * var_dnm))), (-(var_dnm_dn10 / (var_dnm * var_dnm))), (-(var_dnm_dn13 / (var_dnm * var_dnm))),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
        var_dnm = assign60020_e93644;
        var_dnm_dn0 = assign60020_e93644_d_n0;
        var_dnm_dn2 = assign60020_e93644_d_n2;
        var_dnm_dn4 = assign60020_e93644_d_n4;
        var_dnm_dn5 = assign60020_e93644_d_n5;
        var_dnm_dn6 = assign60020_e93644_d_n6;
        var_dnm_dn7 = assign60020_e93644_d_n7;
        var_dnm_dn8 = assign60020_e93644_d_n8;
        var_dnm_dn9 = assign60020_e93644_d_n9;
        var_dnm_dn10 = assign60020_e93644_d_n10;
        var_dnm_dn13 = assign60020_e93644_d_n13;
        var_dnm_rv = 0.0;

        let (assign60030_e93663, assign60030_e93663_d_n0, assign60030_e93663_d_n2, assign60030_e93663_d_n4, assign60030_e93663_d_n5, assign60030_e93663_d_n6, assign60030_e93663_d_n7, assign60030_e93663_d_n8, assign60030_e93663_d_n9, assign60030_e93663_d_n10, assign60030_e93663_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 != 0.0)) && (var_guard1463 != 0.0)) {
        let assign60030_e93658: f64 = (10.0 * 2.220446049250313e-16);
        let assign60030_e93659: f64 = (var_tmf1 * assign60030_e93658);
        let assign60030_e93661: f64 = (assign60030_e93659 * var_dnm);
        (assign60030_e93661, (((var_tmf1_dn0 * assign60030_e93658) * var_dnm) + (assign60030_e93659 * var_dnm_dn0)), (((var_tmf1_dn2 * assign60030_e93658) * var_dnm) + (assign60030_e93659 * var_dnm_dn2)), (((var_tmf1_dn4 * assign60030_e93658) * var_dnm) + (assign60030_e93659 * var_dnm_dn4)), (((var_tmf1_dn5 * assign60030_e93658) * var_dnm) + (assign60030_e93659 * var_dnm_dn5)), (((var_tmf1_dn6 * assign60030_e93658) * var_dnm) + (assign60030_e93659 * var_dnm_dn6)), (((var_tmf1_dn7 * assign60030_e93658) * var_dnm) + (assign60030_e93659 * var_dnm_dn7)), (((var_tmf1_dn8 * assign60030_e93658) * var_dnm) + (assign60030_e93659 * var_dnm_dn8)), (((var_tmf1_dn9 * assign60030_e93658) * var_dnm) + (assign60030_e93659 * var_dnm_dn9)), (((var_tmf1_dn10 * assign60030_e93658) * var_dnm) + (assign60030_e93659 * var_dnm_dn10)), (((var_tmf1_dn13 * assign60030_e93658) * var_dnm) + (assign60030_e93659 * var_dnm_dn13)),)
    } else {
        (var_tmf0, var_tmf0_dn0, var_tmf0_dn2, var_tmf0_dn4, var_tmf0_dn5, var_tmf0_dn6, var_tmf0_dn7, var_tmf0_dn8, var_tmf0_dn9, var_tmf0_dn10, var_tmf0_dn13,)
    }
};
        var_tmf0 = assign60030_e93663;
        var_tmf0_dn0 = assign60030_e93663_d_n0;
        var_tmf0_dn2 = assign60030_e93663_d_n2;
        var_tmf0_dn4 = assign60030_e93663_d_n4;
        var_tmf0_dn5 = assign60030_e93663_d_n5;
        var_tmf0_dn6 = assign60030_e93663_d_n6;
        var_tmf0_dn7 = assign60030_e93663_d_n7;
        var_tmf0_dn8 = assign60030_e93663_d_n8;
        var_tmf0_dn9 = assign60030_e93663_d_n9;
        var_tmf0_dn10 = assign60030_e93663_d_n10;
        var_tmf0_dn13 = assign60030_e93663_d_n13;
        var_tmf0_rv = 0.0;

        let (assign60040_e93684, assign60040_e93684_d_n0, assign60040_e93684_d_n2, assign60040_e93684_d_n4, assign60040_e93684_d_n5, assign60040_e93684_d_n6, assign60040_e93684_d_n7, assign60040_e93684_d_n8, assign60040_e93684_d_n9, assign60040_e93684_d_n10, assign60040_e93684_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 != 0.0)) && (var_guard1463 != 0.0)) {
        let assign60040_e93676: f64 = (10.0 * 2.220446049250313e-16);
        let assign60040_e93678: f64 = (assign60040_e93676 * var_xmp);
        let assign60040_e93680: f64 = (assign60040_e93678 * var_dnm);
        let assign60040_e93682: f64 = (assign60040_e93680 / var_arg);
        (assign60040_e93682, ((((((assign60040_e93676 * var_xmp_dn0) * var_dnm) + (assign60040_e93678 * var_dnm_dn0)) * var_arg) - (assign60040_e93680 * var_arg_dn0)) / (var_arg * var_arg)), ((((((assign60040_e93676 * var_xmp_dn2) * var_dnm) + (assign60040_e93678 * var_dnm_dn2)) * var_arg) - (assign60040_e93680 * var_arg_dn2)) / (var_arg * var_arg)), ((((((assign60040_e93676 * var_xmp_dn4) * var_dnm) + (assign60040_e93678 * var_dnm_dn4)) * var_arg) - (assign60040_e93680 * var_arg_dn4)) / (var_arg * var_arg)), ((((((assign60040_e93676 * var_xmp_dn5) * var_dnm) + (assign60040_e93678 * var_dnm_dn5)) * var_arg) - (assign60040_e93680 * var_arg_dn5)) / (var_arg * var_arg)), ((((((assign60040_e93676 * var_xmp_dn6) * var_dnm) + (assign60040_e93678 * var_dnm_dn6)) * var_arg) - (assign60040_e93680 * var_arg_dn6)) / (var_arg * var_arg)), ((((((assign60040_e93676 * var_xmp_dn7) * var_dnm) + (assign60040_e93678 * var_dnm_dn7)) * var_arg) - (assign60040_e93680 * var_arg_dn7)) / (var_arg * var_arg)), ((((((assign60040_e93676 * var_xmp_dn8) * var_dnm) + (assign60040_e93678 * var_dnm_dn8)) * var_arg) - (assign60040_e93680 * var_arg_dn8)) / (var_arg * var_arg)), ((((((assign60040_e93676 * var_xmp_dn9) * var_dnm) + (assign60040_e93678 * var_dnm_dn9)) * var_arg) - (assign60040_e93680 * var_arg_dn9)) / (var_arg * var_arg)), ((((((assign60040_e93676 * var_xmp_dn10) * var_dnm) + (assign60040_e93678 * var_dnm_dn10)) * var_arg) - (assign60040_e93680 * var_arg_dn10)) / (var_arg * var_arg)), ((((((assign60040_e93676 * var_xmp_dn13) * var_dnm) + (assign60040_e93678 * var_dnm_dn13)) * var_arg) - (assign60040_e93680 * var_arg_dn13)) / (var_arg * var_arg)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign60040_e93684;
        var_t0_dn0 = assign60040_e93684_d_n0;
        var_t0_dn2 = assign60040_e93684_d_n2;
        var_t0_dn4 = assign60040_e93684_d_n4;
        var_t0_dn5 = assign60040_e93684_d_n5;
        var_t0_dn6 = assign60040_e93684_d_n6;
        var_t0_dn7 = assign60040_e93684_d_n7;
        var_t0_dn8 = assign60040_e93684_d_n8;
        var_t0_dn9 = assign60040_e93684_d_n9;
        var_t0_dn10 = assign60040_e93684_d_n10;
        var_t0_dn13 = assign60040_e93684_d_n13;
        var_t0_rv = 0.0;

        let (assign60050_e93709, assign60050_e93709_d_n0, assign60050_e93709_d_n2, assign60050_e93709_d_n4, assign60050_e93709_d_n5, assign60050_e93709_d_n6, assign60050_e93709_d_n7, assign60050_e93709_d_n8, assign60050_e93709_d_n9, assign60050_e93709_d_n10, assign60050_e93709_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 != 0.0)) && (var_guard1463 != 0.0)) {
        let assign60050_e93697: f64 = (var_ps0 + var_vds);
        let assign60050_e93700: f64 = (10.0 * 2.220446049250313e-16);
        let assign60050_e93701: f64 = (assign60050_e93697 - assign60050_e93700);
        let assign60050_e93704: f64 = (10.0 * 2.220446049250313e-16);
        let assign60050_e93705: f64 = (assign60050_e93701 - assign60050_e93704);
        let assign60050_e93707: f64 = (assign60050_e93705 + var_tmf0);
        (assign60050_e93707, ((var_ps0_dn0 + var_vds_dn0) + var_tmf0_dn0), ((var_ps0_dn2 + var_vds_dn2) + var_tmf0_dn2), ((var_ps0_dn4 + var_vds_dn4) + var_tmf0_dn4), ((var_ps0_dn5 + var_vds_dn5) + var_tmf0_dn5), ((var_ps0_dn6 + var_vds_dn6) + var_tmf0_dn6), ((var_ps0_dn7 + var_vds_dn7) + var_tmf0_dn7), ((var_ps0_dn8 + var_vds_dn8) + var_tmf0_dn8), ((var_ps0_dn9 + var_vds_dn9) + var_tmf0_dn9), ((var_ps0_dn10 + var_vds_dn10) + var_tmf0_dn10), ((var_ps0_dn13 + var_vds_dn13) + var_tmf0_dn13),)
    } else {
        (var_psdl, var_psdl_dn0, var_psdl_dn2, var_psdl_dn4, var_psdl_dn5, var_psdl_dn6, var_psdl_dn7, var_psdl_dn8, var_psdl_dn9, var_psdl_dn10, var_psdl_dn13,)
    }
};
        var_psdl = assign60050_e93709;
        var_psdl_dn0 = assign60050_e93709_d_n0;
        var_psdl_dn2 = assign60050_e93709_d_n2;
        var_psdl_dn4 = assign60050_e93709_d_n4;
        var_psdl_dn5 = assign60050_e93709_d_n5;
        var_psdl_dn6 = assign60050_e93709_d_n6;
        var_psdl_dn7 = assign60050_e93709_d_n7;
        var_psdl_dn8 = assign60050_e93709_d_n8;
        var_psdl_dn9 = assign60050_e93709_d_n9;
        var_psdl_dn10 = assign60050_e93709_d_n10;
        var_psdl_dn13 = assign60050_e93709_d_n13;
        var_psdl_rv = 0.0;

        let (assign60060_e93722, assign60060_e93722_d_n0, assign60060_e93722_d_n2, assign60060_e93722_d_n4, assign60060_e93722_d_n5, assign60060_e93722_d_n6, assign60060_e93722_d_n7, assign60060_e93722_d_n8, assign60060_e93722_d_n9, assign60060_e93722_d_n10, assign60060_e93722_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 != 0.0)) && (var_guard1463 != 0.0)) {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign60060_e93722;
        var_t0_dn0 = assign60060_e93722_d_n0;
        var_t0_dn2 = assign60060_e93722_d_n2;
        var_t0_dn4 = assign60060_e93722_d_n4;
        var_t0_dn5 = assign60060_e93722_d_n5;
        var_t0_dn6 = assign60060_e93722_d_n6;
        var_t0_dn7 = assign60060_e93722_d_n7;
        var_t0_dn8 = assign60060_e93722_d_n8;
        var_t0_dn9 = assign60060_e93722_d_n9;
        var_t0_dn10 = assign60060_e93722_d_n10;
        var_t0_dn13 = assign60060_e93722_d_n13;
        var_t0_rv = 0.0;

        let (assign60070_e93736, assign60070_e93736_d_n0, assign60070_e93736_d_n2, assign60070_e93736_d_n4, assign60070_e93736_d_n5, assign60070_e93736_d_n6, assign60070_e93736_d_n7, assign60070_e93736_d_n8, assign60070_e93736_d_n9, assign60070_e93736_d_n10, assign60070_e93736_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 != 0.0)) && (var_guard1463 == 0.0)) {
        (var_psdl, var_psdl_dn0, var_psdl_dn2, var_psdl_dn4, var_psdl_dn5, var_psdl_dn6, var_psdl_dn7, var_psdl_dn8, var_psdl_dn9, var_psdl_dn10, var_psdl_dn13,)
    } else {
        (var_psdl, var_psdl_dn0, var_psdl_dn2, var_psdl_dn4, var_psdl_dn5, var_psdl_dn6, var_psdl_dn7, var_psdl_dn8, var_psdl_dn9, var_psdl_dn10, var_psdl_dn13,)
    }
};
        var_psdl = assign60070_e93736;
        var_psdl_dn0 = assign60070_e93736_d_n0;
        var_psdl_dn2 = assign60070_e93736_d_n2;
        var_psdl_dn4 = assign60070_e93736_d_n4;
        var_psdl_dn5 = assign60070_e93736_d_n5;
        var_psdl_dn6 = assign60070_e93736_d_n6;
        var_psdl_dn7 = assign60070_e93736_d_n7;
        var_psdl_dn8 = assign60070_e93736_d_n8;
        var_psdl_dn9 = assign60070_e93736_d_n9;
        var_psdl_dn10 = assign60070_e93736_d_n10;
        var_psdl_dn13 = assign60070_e93736_d_n13;
        var_psdl_rv = 0.0;

        let (assign60080_e93750, assign60080_e93750_d_n0, assign60080_e93750_d_n2, assign60080_e93750_d_n4, assign60080_e93750_d_n5, assign60080_e93750_d_n6, assign60080_e93750_d_n7, assign60080_e93750_d_n8, assign60080_e93750_d_n9, assign60080_e93750_d_n10, assign60080_e93750_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 != 0.0)) && (var_guard1463 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign60080_e93750;
        var_t0_dn0 = assign60080_e93750_d_n0;
        var_t0_dn2 = assign60080_e93750_d_n2;
        var_t0_dn4 = assign60080_e93750_d_n4;
        var_t0_dn5 = assign60080_e93750_d_n5;
        var_t0_dn6 = assign60080_e93750_d_n6;
        var_t0_dn7 = assign60080_e93750_d_n7;
        var_t0_dn8 = assign60080_e93750_d_n8;
        var_t0_dn9 = assign60080_e93750_d_n9;
        var_t0_dn10 = assign60080_e93750_d_n10;
        var_t0_dn13 = assign60080_e93750_d_n13;
        var_t0_rv = 0.0;

        let (assign60090_e93762, assign60090_e93762_d_n0, assign60090_e93762_d_n2, assign60090_e93762_d_n4, assign60090_e93762_d_n5, assign60090_e93762_d_n6, assign60090_e93762_d_n7, assign60090_e93762_d_n8, assign60090_e93762_d_n9, assign60090_e93762_d_n10, assign60090_e93762_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        (var_wdpl, var_wdpl_dn0, var_wdpl_dn2, var_wdpl_dn4, var_wdpl_dn5, var_wdpl_dn6, var_wdpl_dn7, var_wdpl_dn8, var_wdpl_dn9, var_wdpl_dn10, var_wdpl_dn13,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign60090_e93762;
        var_t1_dn0 = assign60090_e93762_d_n0;
        var_t1_dn2 = assign60090_e93762_d_n2;
        var_t1_dn4 = assign60090_e93762_d_n4;
        var_t1_dn5 = assign60090_e93762_d_n5;
        var_t1_dn6 = assign60090_e93762_d_n6;
        var_t1_dn7 = assign60090_e93762_d_n7;
        var_t1_dn8 = assign60090_e93762_d_n8;
        var_t1_dn9 = assign60090_e93762_d_n9;
        var_t1_dn10 = assign60090_e93762_d_n10;
        var_t1_dn13 = assign60090_e93762_d_n13;
        var_t1_rv = 0.0;

        let (assign60100_e93777, assign60100_e93777_d_n0, assign60100_e93777_d_n2, assign60100_e93777_d_n4, assign60100_e93777_d_n5, assign60100_e93777_d_n6, assign60100_e93777_d_n7, assign60100_e93777_d_n8, assign60100_e93777_d_n9, assign60100_e93777_d_n10, assign60100_e93777_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let assign60100_e93774: f64 = (var_psl - var_vbscl__blk435);
        let assign60100_e93775: f64 = (assign60100_e93774).sqrt();
        (assign60100_e93775, ((var_psl_dn0 - var_vbscl__blk435_dn0) / (2.0 * assign60100_e93775)), ((var_psl_dn2 - var_vbscl__blk435_dn2) / (2.0 * assign60100_e93775)), ((var_psl_dn4 - var_vbscl__blk435_dn4) / (2.0 * assign60100_e93775)), ((var_psl_dn5 - var_vbscl__blk435_dn5) / (2.0 * assign60100_e93775)), ((var_psl_dn6 - var_vbscl__blk435_dn6) / (2.0 * assign60100_e93775)), ((var_psl_dn7 - var_vbscl__blk435_dn7) / (2.0 * assign60100_e93775)), ((var_psl_dn8 - var_vbscl__blk435_dn8) / (2.0 * assign60100_e93775)), ((var_psl_dn9 - var_vbscl__blk435_dn9) / (2.0 * assign60100_e93775)), ((var_psl_dn10 - var_vbscl__blk435_dn10) / (2.0 * assign60100_e93775)), ((var_psl_dn13 - var_vbscl__blk435_dn13) / (2.0 * assign60100_e93775)),)
    } else {
        (var_t8, var_t8_dn0, var_t8_dn2, var_t8_dn4, var_t8_dn5, var_t8_dn6, var_t8_dn7, var_t8_dn8, var_t8_dn9, var_t8_dn10, var_t8_dn13,)
    }
};
        var_t8 = assign60100_e93777;
        var_t8_dn0 = assign60100_e93777_d_n0;
        var_t8_dn2 = assign60100_e93777_d_n2;
        var_t8_dn4 = assign60100_e93777_d_n4;
        var_t8_dn5 = assign60100_e93777_d_n5;
        var_t8_dn6 = assign60100_e93777_d_n6;
        var_t8_dn7 = assign60100_e93777_d_n7;
        var_t8_dn8 = assign60100_e93777_d_n8;
        var_t8_dn9 = assign60100_e93777_d_n9;
        var_t8_dn10 = assign60100_e93777_d_n10;
        var_t8_dn13 = assign60100_e93777_d_n13;
        var_t8_rv = 0.0;

        let (assign60110_e93791, assign60110_e93791_d_n0, assign60110_e93791_d_n2, assign60110_e93791_d_n4, assign60110_e93791_d_n5, assign60110_e93791_d_n6, assign60110_e93791_d_n7, assign60110_e93791_d_n8, assign60110_e93791_d_n9, assign60110_e93791_d_n10, assign60110_e93791_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let assign60110_e93789: f64 = (var_t1 * var_t8);
        (assign60110_e93789, ((var_t1_dn0 * var_t8) + (var_t1 * var_t8_dn0)), ((var_t1_dn2 * var_t8) + (var_t1 * var_t8_dn2)), ((var_t1_dn4 * var_t8) + (var_t1 * var_t8_dn4)), ((var_t1_dn5 * var_t8) + (var_t1 * var_t8_dn5)), ((var_t1_dn6 * var_t8) + (var_t1 * var_t8_dn6)), ((var_t1_dn7 * var_t8) + (var_t1 * var_t8_dn7)), ((var_t1_dn8 * var_t8) + (var_t1 * var_t8_dn8)), ((var_t1_dn9 * var_t8) + (var_t1 * var_t8_dn9)), ((var_t1_dn10 * var_t8) + (var_t1 * var_t8_dn10)), ((var_t1_dn13 * var_t8) + (var_t1 * var_t8_dn13)),)
    } else {
        (var_wd, var_wd_dn0, var_wd_dn2, var_wd_dn4, var_wd_dn5, var_wd_dn6, var_wd_dn7, var_wd_dn8, var_wd_dn9, var_wd_dn10, var_wd_dn13,)
    }
};
        var_wd = assign60110_e93791;
        var_wd_dn0 = assign60110_e93791_d_n0;
        var_wd_dn2 = assign60110_e93791_d_n2;
        var_wd_dn4 = assign60110_e93791_d_n4;
        var_wd_dn5 = assign60110_e93791_d_n5;
        var_wd_dn6 = assign60110_e93791_d_n6;
        var_wd_dn7 = assign60110_e93791_d_n7;
        var_wd_dn8 = assign60110_e93791_d_n8;
        var_wd_dn9 = assign60110_e93791_d_n9;
        var_wd_dn10 = assign60110_e93791_d_n10;
        var_wd_dn13 = assign60110_e93791_d_n13;
        var_wd_rv = 0.0;

        let (assign60120_e93807, assign60120_e93807_d_n0, assign60120_e93807_d_n2, assign60120_e93807_d_n4, assign60120_e93807_d_n5, assign60120_e93807_d_n6, assign60120_e93807_d_n7, assign60120_e93807_d_n8, assign60120_e93807_d_n9, assign60120_e93807_d_n10, assign60120_e93807_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let assign60120_e93803: f64 = (0.5 * var_t1);
        let assign60120_e93805: f64 = (assign60120_e93803 / var_t8);
        (assign60120_e93805, ((((0.5 * var_t1_dn0) * var_t8) - (assign60120_e93803 * var_t8_dn0)) / (var_t8 * var_t8)), ((((0.5 * var_t1_dn2) * var_t8) - (assign60120_e93803 * var_t8_dn2)) / (var_t8 * var_t8)), ((((0.5 * var_t1_dn4) * var_t8) - (assign60120_e93803 * var_t8_dn4)) / (var_t8 * var_t8)), ((((0.5 * var_t1_dn5) * var_t8) - (assign60120_e93803 * var_t8_dn5)) / (var_t8 * var_t8)), ((((0.5 * var_t1_dn6) * var_t8) - (assign60120_e93803 * var_t8_dn6)) / (var_t8 * var_t8)), ((((0.5 * var_t1_dn7) * var_t8) - (assign60120_e93803 * var_t8_dn7)) / (var_t8 * var_t8)), ((((0.5 * var_t1_dn8) * var_t8) - (assign60120_e93803 * var_t8_dn8)) / (var_t8 * var_t8)), ((((0.5 * var_t1_dn9) * var_t8) - (assign60120_e93803 * var_t8_dn9)) / (var_t8 * var_t8)), ((((0.5 * var_t1_dn10) * var_t8) - (assign60120_e93803 * var_t8_dn10)) / (var_t8 * var_t8)), ((((0.5 * var_t1_dn13) * var_t8) - (assign60120_e93803 * var_t8_dn13)) / (var_t8 * var_t8)),)
    } else {
        (var_t9, var_t9_dn0, var_t9_dn2, var_t9_dn4, var_t9_dn5, var_t9_dn6, var_t9_dn7, var_t9_dn8, var_t9_dn9, var_t9_dn10, var_t9_dn13,)
    }
};
        var_t9 = assign60120_e93807;
        var_t9_dn0 = assign60120_e93807_d_n0;
        var_t9_dn2 = assign60120_e93807_d_n2;
        var_t9_dn4 = assign60120_e93807_d_n4;
        var_t9_dn5 = assign60120_e93807_d_n5;
        var_t9_dn6 = assign60120_e93807_d_n6;
        var_t9_dn7 = assign60120_e93807_d_n7;
        var_t9_dn8 = assign60120_e93807_d_n8;
        var_t9_dn9 = assign60120_e93807_d_n9;
        var_t9_dn10 = assign60120_e93807_d_n10;
        var_t9_dn13 = assign60120_e93807_d_n13;
        var_t9_rv = 0.0;

        let (assign60130_e93821, assign60130_e93821_d_n0, assign60130_e93821_d_n2, assign60130_e93821_d_n4, assign60130_e93821_d_n5, assign60130_e93821_d_n6, assign60130_e93821_d_n7, assign60130_e93821_d_n8, assign60130_e93821_d_n9, assign60130_e93821_d_n10, assign60130_e93821_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let assign60130_e93819: f64 = (1.0 / var_wd);
        (assign60130_e93819, (-(var_wd_dn0 / (var_wd * var_wd))), (-(var_wd_dn2 / (var_wd * var_wd))), (-(var_wd_dn4 / (var_wd * var_wd))), (-(var_wd_dn5 / (var_wd * var_wd))), (-(var_wd_dn6 / (var_wd * var_wd))), (-(var_wd_dn7 / (var_wd * var_wd))), (-(var_wd_dn8 / (var_wd * var_wd))), (-(var_wd_dn9 / (var_wd * var_wd))), (-(var_wd_dn10 / (var_wd * var_wd))), (-(var_wd_dn13 / (var_wd * var_wd))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign60130_e93821;
        var_t0_dn0 = assign60130_e93821_d_n0;
        var_t0_dn2 = assign60130_e93821_d_n2;
        var_t0_dn4 = assign60130_e93821_d_n4;
        var_t0_dn5 = assign60130_e93821_d_n5;
        var_t0_dn6 = assign60130_e93821_d_n6;
        var_t0_dn7 = assign60130_e93821_d_n7;
        var_t0_dn8 = assign60130_e93821_d_n8;
        var_t0_dn9 = assign60130_e93821_d_n9;
        var_t0_dn10 = assign60130_e93821_d_n10;
        var_t0_dn13 = assign60130_e93821_d_n13;
        var_t0_rv = 0.0;

        let (assign60140_e93835, assign60140_e93835_d_n0, assign60140_e93835_d_n2, assign60140_e93835_d_n4, assign60140_e93835_d_n5, assign60140_e93835_d_n6, assign60140_e93835_d_n7, assign60140_e93835_d_n8, assign60140_e93835_d_n9, assign60140_e93835_d_n10, assign60140_e93835_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let assign60140_e93833: f64 = (var_qn0 * var_t0);
        (assign60140_e93833, ((var_qn0_dn0 * var_t0) + (var_qn0 * var_t0_dn0)), ((var_qn0_dn2 * var_t0) + (var_qn0 * var_t0_dn2)), ((var_qn0_dn4 * var_t0) + (var_qn0 * var_t0_dn4)), ((var_qn0_dn5 * var_t0) + (var_qn0 * var_t0_dn5)), ((var_qn0_dn6 * var_t0) + (var_qn0 * var_t0_dn6)), ((var_qn0_dn7 * var_t0) + (var_qn0 * var_t0_dn7)), ((var_qn0_dn8 * var_t0) + (var_qn0 * var_t0_dn8)), ((var_qn0_dn9 * var_t0) + (var_qn0 * var_t0_dn9)), ((var_qn0_dn10 * var_t0) + (var_qn0 * var_t0_dn10)), ((var_qn0_dn13 * var_t0) + (var_qn0 * var_t0_dn13)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign60140_e93835;
        var_t1_dn0 = assign60140_e93835_d_n0;
        var_t1_dn2 = assign60140_e93835_d_n2;
        var_t1_dn4 = assign60140_e93835_d_n4;
        var_t1_dn5 = assign60140_e93835_d_n5;
        var_t1_dn6 = assign60140_e93835_d_n6;
        var_t1_dn7 = assign60140_e93835_d_n7;
        var_t1_dn8 = assign60140_e93835_d_n8;
        var_t1_dn9 = assign60140_e93835_d_n9;
        var_t1_dn10 = assign60140_e93835_d_n10;
        var_t1_dn13 = assign60140_e93835_d_n13;
        var_t1_rv = 0.0;

        let (assign60150_e93849, assign60150_e93849_d_n0, assign60150_e93849_d_n2, assign60150_e93849_d_n4, assign60150_e93849_d_n5, assign60150_e93849_d_n6, assign60150_e93849_d_n7, assign60150_e93849_d_n8, assign60150_e93849_d_n9, assign60150_e93849_d_n10, assign60150_e93849_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let assign60150_e93847: f64 = (var_uc_clm3 * var_t1);
        (assign60150_e93847, (var_uc_clm3 * var_t1_dn0), (var_uc_clm3 * var_t1_dn2), (var_uc_clm3 * var_t1_dn4), (var_uc_clm3 * var_t1_dn5), (var_uc_clm3 * var_t1_dn6), (var_uc_clm3 * var_t1_dn7), (var_uc_clm3 * var_t1_dn8), (var_uc_clm3 * var_t1_dn9), (var_uc_clm3 * var_t1_dn10), (var_uc_clm3 * var_t1_dn13),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign60150_e93849;
        var_t2_dn0 = assign60150_e93849_d_n0;
        var_t2_dn2 = assign60150_e93849_d_n2;
        var_t2_dn4 = assign60150_e93849_d_n4;
        var_t2_dn5 = assign60150_e93849_d_n5;
        var_t2_dn6 = assign60150_e93849_d_n6;
        var_t2_dn7 = assign60150_e93849_d_n7;
        var_t2_dn8 = assign60150_e93849_d_n8;
        var_t2_dn9 = assign60150_e93849_d_n9;
        var_t2_dn10 = assign60150_e93849_d_n10;
        var_t2_dn13 = assign60150_e93849_d_n13;
        var_t2_rv = 0.0;

        let (assign60160_e93863, assign60160_e93863_d_n0, assign60160_e93863_d_n2, assign60160_e93863_d_n4, assign60160_e93863_d_n5, assign60160_e93863_d_n6, assign60160_e93863_d_n7, assign60160_e93863_d_n8, assign60160_e93863_d_n9, assign60160_e93863_d_n10, assign60160_e93863_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let assign60160_e93861: f64 = (var_uc_clm3 * var_t0);
        (assign60160_e93861, (var_uc_clm3 * var_t0_dn0), (var_uc_clm3 * var_t0_dn2), (var_uc_clm3 * var_t0_dn4), (var_uc_clm3 * var_t0_dn5), (var_uc_clm3 * var_t0_dn6), (var_uc_clm3 * var_t0_dn7), (var_uc_clm3 * var_t0_dn8), (var_uc_clm3 * var_t0_dn9), (var_uc_clm3 * var_t0_dn10), (var_uc_clm3 * var_t0_dn13),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn13,)
    }
};
        var_t3 = assign60160_e93863;
        var_t3_dn0 = assign60160_e93863_d_n0;
        var_t3_dn2 = assign60160_e93863_d_n2;
        var_t3_dn4 = assign60160_e93863_d_n4;
        var_t3_dn5 = assign60160_e93863_d_n5;
        var_t3_dn6 = assign60160_e93863_d_n6;
        var_t3_dn7 = assign60160_e93863_d_n7;
        var_t3_dn8 = assign60160_e93863_d_n8;
        var_t3_dn9 = assign60160_e93863_d_n9;
        var_t3_dn10 = assign60160_e93863_d_n10;
        var_t3_dn13 = assign60160_e93863_d_n13;
        var_t3_rv = 0.0;

        let (assign60170_e93879, assign60170_e93879_d_n0, assign60170_e93879_d_n2, assign60170_e93879_d_n4, assign60170_e93879_d_n5, assign60170_e93879_d_n6, assign60170_e93879_d_n7, assign60170_e93879_d_n8, assign60170_e93879_d_n9, assign60170_e93879_d_n10, assign60170_e93879_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let assign60170_e93875: f64 = (var_uc_clm2 * var_q_nsub);
        let assign60170_e93877: f64 = (assign60170_e93875 + var_t2);
        (assign60170_e93877, (((var_uc_clm2_dn0 * var_q_nsub) + (var_uc_clm2 * var_q_nsub_dn0)) + var_t2_dn0), (((var_uc_clm2_dn2 * var_q_nsub) + (var_uc_clm2 * var_q_nsub_dn2)) + var_t2_dn2), (((var_uc_clm2_dn4 * var_q_nsub) + (var_uc_clm2 * var_q_nsub_dn4)) + var_t2_dn4), (((var_uc_clm2_dn5 * var_q_nsub) + (var_uc_clm2 * var_q_nsub_dn5)) + var_t2_dn5), (((var_uc_clm2_dn6 * var_q_nsub) + (var_uc_clm2 * var_q_nsub_dn6)) + var_t2_dn6), (((var_uc_clm2_dn7 * var_q_nsub) + (var_uc_clm2 * var_q_nsub_dn7)) + var_t2_dn7), (((var_uc_clm2_dn8 * var_q_nsub) + (var_uc_clm2 * var_q_nsub_dn8)) + var_t2_dn8), (((var_uc_clm2_dn9 * var_q_nsub) + (var_uc_clm2 * var_q_nsub_dn9)) + var_t2_dn9), (((var_uc_clm2_dn10 * var_q_nsub) + (var_uc_clm2 * var_q_nsub_dn10)) + var_t2_dn10), (((var_uc_clm2_dn13 * var_q_nsub) + (var_uc_clm2 * var_q_nsub_dn13)) + var_t2_dn13),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn13,)
    }
};
        var_t5 = assign60170_e93879;
        var_t5_dn0 = assign60170_e93879_d_n0;
        var_t5_dn2 = assign60170_e93879_d_n2;
        var_t5_dn4 = assign60170_e93879_d_n4;
        var_t5_dn5 = assign60170_e93879_d_n5;
        var_t5_dn6 = assign60170_e93879_d_n6;
        var_t5_dn7 = assign60170_e93879_d_n7;
        var_t5_dn8 = assign60170_e93879_d_n8;
        var_t5_dn9 = assign60170_e93879_d_n9;
        var_t5_dn10 = assign60170_e93879_d_n10;
        var_t5_dn13 = assign60170_e93879_d_n13;
        var_t5_rv = 0.0;

        *var_dnm_slot = var_dnm;
        *var_dnm_dn0_slot = var_dnm_dn0;
        *var_dnm_dn10_slot = var_dnm_dn10;
        *var_dnm_dn13_slot = var_dnm_dn13;
        *var_dnm_dn2_slot = var_dnm_dn2;
        *var_dnm_dn4_slot = var_dnm_dn4;
        *var_dnm_dn5_slot = var_dnm_dn5;
        *var_dnm_dn6_slot = var_dnm_dn6;
        *var_dnm_dn7_slot = var_dnm_dn7;
        *var_dnm_dn8_slot = var_dnm_dn8;
        *var_dnm_dn9_slot = var_dnm_dn9;
        *var_dnm_rv_slot = var_dnm_rv;
        *var_guard1464_slot = var_guard1464;
        *var_guard1464_rv_slot = var_guard1464_rv;
        *var_guard1465_slot = var_guard1465;
        *var_guard1465_rv_slot = var_guard1465_rv;
        *var_guard1466_slot = var_guard1466;
        *var_guard1466_rv_slot = var_guard1466_rv;
        *var_guard1467_slot = var_guard1467;
        *var_guard1467_rv_slot = var_guard1467_rv;
        *var_guard1468_slot = var_guard1468;
        *var_guard1468_rv_slot = var_guard1468_rv;
        *var_m0_slot = var_m0;
        *var_m0_rv_slot = var_m0_rv;
        *var_mm_slot = var_mm;
        *var_mm_rv_slot = var_mm_rv;
        *var_psdl_slot = var_psdl;
        *var_psdl_dn0_slot = var_psdl_dn0;
        *var_psdl_dn10_slot = var_psdl_dn10;
        *var_psdl_dn13_slot = var_psdl_dn13;
        *var_psdl_dn2_slot = var_psdl_dn2;
        *var_psdl_dn4_slot = var_psdl_dn4;
        *var_psdl_dn5_slot = var_psdl_dn5;
        *var_psdl_dn6_slot = var_psdl_dn6;
        *var_psdl_dn7_slot = var_psdl_dn7;
        *var_psdl_dn8_slot = var_psdl_dn8;
        *var_psdl_dn9_slot = var_psdl_dn9;
        *var_psdl_rv_slot = var_psdl_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn13_slot = var_t0_dn13;
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
        *var_t1_dn13_slot = var_t1_dn13;
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
        *var_t2_dn13_slot = var_t2_dn13;
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
        *var_t3_dn13_slot = var_t3_dn13;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t3_rv_slot = var_t3_rv;
        *var_t5_slot = var_t5;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn10_slot = var_t5_dn10;
        *var_t5_dn13_slot = var_t5_dn13;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t5_dn9_slot = var_t5_dn9;
        *var_t5_rv_slot = var_t5_rv;
        *var_t8_slot = var_t8;
        *var_t8_dn0_slot = var_t8_dn0;
        *var_t8_dn10_slot = var_t8_dn10;
        *var_t8_dn13_slot = var_t8_dn13;
        *var_t8_dn2_slot = var_t8_dn2;
        *var_t8_dn4_slot = var_t8_dn4;
        *var_t8_dn5_slot = var_t8_dn5;
        *var_t8_dn6_slot = var_t8_dn6;
        *var_t8_dn7_slot = var_t8_dn7;
        *var_t8_dn8_slot = var_t8_dn8;
        *var_t8_dn9_slot = var_t8_dn9;
        *var_t8_rv_slot = var_t8_rv;
        *var_t9_slot = var_t9;
        *var_t9_dn0_slot = var_t9_dn0;
        *var_t9_dn10_slot = var_t9_dn10;
        *var_t9_dn13_slot = var_t9_dn13;
        *var_t9_dn2_slot = var_t9_dn2;
        *var_t9_dn4_slot = var_t9_dn4;
        *var_t9_dn5_slot = var_t9_dn5;
        *var_t9_dn6_slot = var_t9_dn6;
        *var_t9_dn7_slot = var_t9_dn7;
        *var_t9_dn8_slot = var_t9_dn8;
        *var_t9_dn9_slot = var_t9_dn9;
        *var_t9_rv_slot = var_t9_rv;
        *var_tmf0_slot = var_tmf0;
        *var_tmf0_dn0_slot = var_tmf0_dn0;
        *var_tmf0_dn10_slot = var_tmf0_dn10;
        *var_tmf0_dn13_slot = var_tmf0_dn13;
        *var_tmf0_dn2_slot = var_tmf0_dn2;
        *var_tmf0_dn4_slot = var_tmf0_dn4;
        *var_tmf0_dn5_slot = var_tmf0_dn5;
        *var_tmf0_dn6_slot = var_tmf0_dn6;
        *var_tmf0_dn7_slot = var_tmf0_dn7;
        *var_tmf0_dn8_slot = var_tmf0_dn8;
        *var_tmf0_dn9_slot = var_tmf0_dn9;
        *var_tmf0_rv_slot = var_tmf0_rv;
        *var_wd_slot = var_wd;
        *var_wd_dn0_slot = var_wd_dn0;
        *var_wd_dn10_slot = var_wd_dn10;
        *var_wd_dn13_slot = var_wd_dn13;
        *var_wd_dn2_slot = var_wd_dn2;
        *var_wd_dn4_slot = var_wd_dn4;
        *var_wd_dn5_slot = var_wd_dn5;
        *var_wd_dn6_slot = var_wd_dn6;
        *var_wd_dn7_slot = var_wd_dn7;
        *var_wd_dn8_slot = var_wd_dn8;
        *var_wd_dn9_slot = var_wd_dn9;
        *var_wd_rv_slot = var_wd_rv;
    }

    pub(super) fn stamp_reactive_block_215(
        var_guard1430: f64,
        var_guard1461: f64,
        var_guard1462: f64,
        var_guard443: f64,
        var_ps0: f64,
        var_ps0_dn0: f64,
        var_ps0_dn10: f64,
        var_ps0_dn13: f64,
        var_ps0_dn2: f64,
        var_ps0_dn4: f64,
        var_ps0_dn5: f64,
        var_ps0_dn6: f64,
        var_ps0_dn7: f64,
        var_ps0_dn8: f64,
        var_ps0_dn9: f64,
        var_psl: f64,
        var_psl_dn0: f64,
        var_psl_dn10: f64,
        var_psl_dn13: f64,
        var_psl_dn2: f64,
        var_psl_dn4: f64,
        var_psl_dn5: f64,
        var_psl_dn6: f64,
        var_psl_dn7: f64,
        var_psl_dn8: f64,
        var_psl_dn9: f64,
        var_t5: f64,
        var_t5_dn0: f64,
        var_t5_dn10: f64,
        var_t5_dn13: f64,
        var_t5_dn2: f64,
        var_t5_dn4: f64,
        var_t5_dn5: f64,
        var_t5_dn6: f64,
        var_t5_dn7: f64,
        var_t5_dn8: f64,
        var_t5_dn9: f64,
        var_uc_clm1: f64,
        var_vds: f64,
        var_vds_dn0: f64,
        var_vds_dn10: f64,
        var_vds_dn13: f64,
        var_vds_dn2: f64,
        var_vds_dn4: f64,
        var_vds_dn5: f64,
        var_vds_dn6: f64,
        var_vds_dn7: f64,
        var_vds_dn8: f64,
        var_vds_dn9: f64,
        var_arg_slot: &mut f64,
        var_arg_dn0_slot: &mut f64,
        var_arg_dn10_slot: &mut f64,
        var_arg_dn13_slot: &mut f64,
        var_arg_dn2_slot: &mut f64,
        var_arg_dn4_slot: &mut f64,
        var_arg_dn5_slot: &mut f64,
        var_arg_dn6_slot: &mut f64,
        var_arg_dn7_slot: &mut f64,
        var_arg_dn8_slot: &mut f64,
        var_arg_dn9_slot: &mut f64,
        var_arg_rv_slot: &mut f64,
        var_dnm_slot: &mut f64,
        var_dnm_dn0_slot: &mut f64,
        var_dnm_dn10_slot: &mut f64,
        var_dnm_dn13_slot: &mut f64,
        var_dnm_dn2_slot: &mut f64,
        var_dnm_dn4_slot: &mut f64,
        var_dnm_dn5_slot: &mut f64,
        var_dnm_dn6_slot: &mut f64,
        var_dnm_dn7_slot: &mut f64,
        var_dnm_dn8_slot: &mut f64,
        var_dnm_dn9_slot: &mut f64,
        var_dnm_rv_slot: &mut f64,
        var_guard1469_slot: &mut f64,
        var_guard1469_rv_slot: &mut f64,
        var_guard1470_slot: &mut f64,
        var_guard1470_rv_slot: &mut f64,
        var_guard1471_slot: &mut f64,
        var_guard1471_rv_slot: &mut f64,
        var_guard1472_slot: &mut f64,
        var_guard1472_rv_slot: &mut f64,
        var_guard1473_slot: &mut f64,
        var_guard1473_rv_slot: &mut f64,
        var_guard1474_slot: &mut f64,
        var_guard1474_rv_slot: &mut f64,
        var_m0_slot: &mut f64,
        var_m0_rv_slot: &mut f64,
        var_mm_slot: &mut f64,
        var_mm_rv_slot: &mut f64,
        var_psdl_slot: &mut f64,
        var_psdl_dn0_slot: &mut f64,
        var_psdl_dn10_slot: &mut f64,
        var_psdl_dn13_slot: &mut f64,
        var_psdl_dn2_slot: &mut f64,
        var_psdl_dn4_slot: &mut f64,
        var_psdl_dn5_slot: &mut f64,
        var_psdl_dn6_slot: &mut f64,
        var_psdl_dn7_slot: &mut f64,
        var_psdl_dn8_slot: &mut f64,
        var_psdl_dn9_slot: &mut f64,
        var_psdl_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn13_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn13_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_dn9_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn13_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn7_slot: &mut f64,
        var_tmf1_dn8_slot: &mut f64,
        var_tmf1_dn9_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_x2_slot: &mut f64,
        var_x2_dn0_slot: &mut f64,
        var_x2_dn10_slot: &mut f64,
        var_x2_dn13_slot: &mut f64,
        var_x2_dn2_slot: &mut f64,
        var_x2_dn4_slot: &mut f64,
        var_x2_dn5_slot: &mut f64,
        var_x2_dn6_slot: &mut f64,
        var_x2_dn7_slot: &mut f64,
        var_x2_dn8_slot: &mut f64,
        var_x2_dn9_slot: &mut f64,
        var_x2_rv_slot: &mut f64,
        var_xmax2_slot: &mut f64,
        var_xmax2_dn0_slot: &mut f64,
        var_xmax2_dn10_slot: &mut f64,
        var_xmax2_dn13_slot: &mut f64,
        var_xmax2_dn2_slot: &mut f64,
        var_xmax2_dn4_slot: &mut f64,
        var_xmax2_dn5_slot: &mut f64,
        var_xmax2_dn6_slot: &mut f64,
        var_xmax2_dn7_slot: &mut f64,
        var_xmax2_dn8_slot: &mut f64,
        var_xmax2_dn9_slot: &mut f64,
        var_xmax2_rv_slot: &mut f64,
        var_xmp_slot: &mut f64,
        var_xmp_dn0_slot: &mut f64,
        var_xmp_dn10_slot: &mut f64,
        var_xmp_dn13_slot: &mut f64,
        var_xmp_dn2_slot: &mut f64,
        var_xmp_dn4_slot: &mut f64,
        var_xmp_dn5_slot: &mut f64,
        var_xmp_dn6_slot: &mut f64,
        var_xmp_dn7_slot: &mut f64,
        var_xmp_dn8_slot: &mut f64,
        var_xmp_dn9_slot: &mut f64,
        var_xmp_rv_slot: &mut f64,
        var_xp_slot: &mut f64,
        var_xp_dn0_slot: &mut f64,
        var_xp_dn10_slot: &mut f64,
        var_xp_dn13_slot: &mut f64,
        var_xp_dn2_slot: &mut f64,
        var_xp_dn4_slot: &mut f64,
        var_xp_dn5_slot: &mut f64,
        var_xp_dn6_slot: &mut f64,
        var_xp_dn7_slot: &mut f64,
        var_xp_dn8_slot: &mut f64,
        var_xp_dn9_slot: &mut f64,
        var_xp_rv_slot: &mut f64,
    ) {
        let mut var_arg: f64 = *var_arg_slot;
        let mut var_arg_dn0: f64 = *var_arg_dn0_slot;
        let mut var_arg_dn10: f64 = *var_arg_dn10_slot;
        let mut var_arg_dn13: f64 = *var_arg_dn13_slot;
        let mut var_arg_dn2: f64 = *var_arg_dn2_slot;
        let mut var_arg_dn4: f64 = *var_arg_dn4_slot;
        let mut var_arg_dn5: f64 = *var_arg_dn5_slot;
        let mut var_arg_dn6: f64 = *var_arg_dn6_slot;
        let mut var_arg_dn7: f64 = *var_arg_dn7_slot;
        let mut var_arg_dn8: f64 = *var_arg_dn8_slot;
        let mut var_arg_dn9: f64 = *var_arg_dn9_slot;
        let mut var_arg_rv: f64 = *var_arg_rv_slot;
        let mut var_dnm: f64 = *var_dnm_slot;
        let mut var_dnm_dn0: f64 = *var_dnm_dn0_slot;
        let mut var_dnm_dn10: f64 = *var_dnm_dn10_slot;
        let mut var_dnm_dn13: f64 = *var_dnm_dn13_slot;
        let mut var_dnm_dn2: f64 = *var_dnm_dn2_slot;
        let mut var_dnm_dn4: f64 = *var_dnm_dn4_slot;
        let mut var_dnm_dn5: f64 = *var_dnm_dn5_slot;
        let mut var_dnm_dn6: f64 = *var_dnm_dn6_slot;
        let mut var_dnm_dn7: f64 = *var_dnm_dn7_slot;
        let mut var_dnm_dn8: f64 = *var_dnm_dn8_slot;
        let mut var_dnm_dn9: f64 = *var_dnm_dn9_slot;
        let mut var_dnm_rv: f64 = *var_dnm_rv_slot;
        let mut var_guard1469: f64 = *var_guard1469_slot;
        let mut var_guard1469_rv: f64 = *var_guard1469_rv_slot;
        let mut var_guard1470: f64 = *var_guard1470_slot;
        let mut var_guard1470_rv: f64 = *var_guard1470_rv_slot;
        let mut var_guard1471: f64 = *var_guard1471_slot;
        let mut var_guard1471_rv: f64 = *var_guard1471_rv_slot;
        let mut var_guard1472: f64 = *var_guard1472_slot;
        let mut var_guard1472_rv: f64 = *var_guard1472_rv_slot;
        let mut var_guard1473: f64 = *var_guard1473_slot;
        let mut var_guard1473_rv: f64 = *var_guard1473_rv_slot;
        let mut var_guard1474: f64 = *var_guard1474_slot;
        let mut var_guard1474_rv: f64 = *var_guard1474_rv_slot;
        let mut var_m0: f64 = *var_m0_slot;
        let mut var_m0_rv: f64 = *var_m0_rv_slot;
        let mut var_mm: f64 = *var_mm_slot;
        let mut var_mm_rv: f64 = *var_mm_rv_slot;
        let mut var_psdl: f64 = *var_psdl_slot;
        let mut var_psdl_dn0: f64 = *var_psdl_dn0_slot;
        let mut var_psdl_dn10: f64 = *var_psdl_dn10_slot;
        let mut var_psdl_dn13: f64 = *var_psdl_dn13_slot;
        let mut var_psdl_dn2: f64 = *var_psdl_dn2_slot;
        let mut var_psdl_dn4: f64 = *var_psdl_dn4_slot;
        let mut var_psdl_dn5: f64 = *var_psdl_dn5_slot;
        let mut var_psdl_dn6: f64 = *var_psdl_dn6_slot;
        let mut var_psdl_dn7: f64 = *var_psdl_dn7_slot;
        let mut var_psdl_dn8: f64 = *var_psdl_dn8_slot;
        let mut var_psdl_dn9: f64 = *var_psdl_dn9_slot;
        let mut var_psdl_rv: f64 = *var_psdl_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn13: f64 = *var_t4_dn13_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_dn9: f64 = *var_t4_dn9_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn13: f64 = *var_tmf1_dn13_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn7: f64 = *var_tmf1_dn7_slot;
        let mut var_tmf1_dn8: f64 = *var_tmf1_dn8_slot;
        let mut var_tmf1_dn9: f64 = *var_tmf1_dn9_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_x2: f64 = *var_x2_slot;
        let mut var_x2_dn0: f64 = *var_x2_dn0_slot;
        let mut var_x2_dn10: f64 = *var_x2_dn10_slot;
        let mut var_x2_dn13: f64 = *var_x2_dn13_slot;
        let mut var_x2_dn2: f64 = *var_x2_dn2_slot;
        let mut var_x2_dn4: f64 = *var_x2_dn4_slot;
        let mut var_x2_dn5: f64 = *var_x2_dn5_slot;
        let mut var_x2_dn6: f64 = *var_x2_dn6_slot;
        let mut var_x2_dn7: f64 = *var_x2_dn7_slot;
        let mut var_x2_dn8: f64 = *var_x2_dn8_slot;
        let mut var_x2_dn9: f64 = *var_x2_dn9_slot;
        let mut var_x2_rv: f64 = *var_x2_rv_slot;
        let mut var_xmax2: f64 = *var_xmax2_slot;
        let mut var_xmax2_dn0: f64 = *var_xmax2_dn0_slot;
        let mut var_xmax2_dn10: f64 = *var_xmax2_dn10_slot;
        let mut var_xmax2_dn13: f64 = *var_xmax2_dn13_slot;
        let mut var_xmax2_dn2: f64 = *var_xmax2_dn2_slot;
        let mut var_xmax2_dn4: f64 = *var_xmax2_dn4_slot;
        let mut var_xmax2_dn5: f64 = *var_xmax2_dn5_slot;
        let mut var_xmax2_dn6: f64 = *var_xmax2_dn6_slot;
        let mut var_xmax2_dn7: f64 = *var_xmax2_dn7_slot;
        let mut var_xmax2_dn8: f64 = *var_xmax2_dn8_slot;
        let mut var_xmax2_dn9: f64 = *var_xmax2_dn9_slot;
        let mut var_xmax2_rv: f64 = *var_xmax2_rv_slot;
        let mut var_xmp: f64 = *var_xmp_slot;
        let mut var_xmp_dn0: f64 = *var_xmp_dn0_slot;
        let mut var_xmp_dn10: f64 = *var_xmp_dn10_slot;
        let mut var_xmp_dn13: f64 = *var_xmp_dn13_slot;
        let mut var_xmp_dn2: f64 = *var_xmp_dn2_slot;
        let mut var_xmp_dn4: f64 = *var_xmp_dn4_slot;
        let mut var_xmp_dn5: f64 = *var_xmp_dn5_slot;
        let mut var_xmp_dn6: f64 = *var_xmp_dn6_slot;
        let mut var_xmp_dn7: f64 = *var_xmp_dn7_slot;
        let mut var_xmp_dn8: f64 = *var_xmp_dn8_slot;
        let mut var_xmp_dn9: f64 = *var_xmp_dn9_slot;
        let mut var_xmp_rv: f64 = *var_xmp_rv_slot;
        let mut var_xp: f64 = *var_xp_slot;
        let mut var_xp_dn0: f64 = *var_xp_dn0_slot;
        let mut var_xp_dn10: f64 = *var_xp_dn10_slot;
        let mut var_xp_dn13: f64 = *var_xp_dn13_slot;
        let mut var_xp_dn2: f64 = *var_xp_dn2_slot;
        let mut var_xp_dn4: f64 = *var_xp_dn4_slot;
        let mut var_xp_dn5: f64 = *var_xp_dn5_slot;
        let mut var_xp_dn6: f64 = *var_xp_dn6_slot;
        let mut var_xp_dn7: f64 = *var_xp_dn7_slot;
        let mut var_xp_dn8: f64 = *var_xp_dn8_slot;
        let mut var_xp_dn9: f64 = *var_xp_dn9_slot;
        let mut var_xp_rv: f64 = *var_xp_rv_slot;

        let (assign60180_e93893, assign60180_e93893_d_n0, assign60180_e93893_d_n2, assign60180_e93893_d_n4, assign60180_e93893_d_n5, assign60180_e93893_d_n6, assign60180_e93893_d_n7, assign60180_e93893_d_n8, assign60180_e93893_d_n9, assign60180_e93893_d_n10, assign60180_e93893_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let assign60180_e93891: f64 = (1.0 / var_t5);
        (assign60180_e93891, (-(var_t5_dn0 / (var_t5 * var_t5))), (-(var_t5_dn2 / (var_t5 * var_t5))), (-(var_t5_dn4 / (var_t5 * var_t5))), (-(var_t5_dn5 / (var_t5 * var_t5))), (-(var_t5_dn6 / (var_t5 * var_t5))), (-(var_t5_dn7 / (var_t5 * var_t5))), (-(var_t5_dn8 / (var_t5 * var_t5))), (-(var_t5_dn9 / (var_t5 * var_t5))), (-(var_t5_dn10 / (var_t5 * var_t5))), (-(var_t5_dn13 / (var_t5 * var_t5))),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign60180_e93893;
        var_t1_dn0 = assign60180_e93893_d_n0;
        var_t1_dn2 = assign60180_e93893_d_n2;
        var_t1_dn4 = assign60180_e93893_d_n4;
        var_t1_dn5 = assign60180_e93893_d_n5;
        var_t1_dn6 = assign60180_e93893_d_n6;
        var_t1_dn7 = assign60180_e93893_d_n7;
        var_t1_dn8 = assign60180_e93893_d_n8;
        var_t1_dn9 = assign60180_e93893_d_n9;
        var_t1_dn10 = assign60180_e93893_d_n10;
        var_t1_dn13 = assign60180_e93893_d_n13;
        var_t1_rv = 0.0;

        let (assign60190_e93907, assign60190_e93907_d_n0, assign60190_e93907_d_n2, assign60190_e93907_d_n4, assign60190_e93907_d_n5, assign60190_e93907_d_n6, assign60190_e93907_d_n7, assign60190_e93907_d_n8, assign60190_e93907_d_n9, assign60190_e93907_d_n10, assign60190_e93907_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let assign60190_e93905: f64 = (1.034943e-10 * var_t1);
        (assign60190_e93905, (1.034943e-10 * var_t1_dn0), (1.034943e-10 * var_t1_dn2), (1.034943e-10 * var_t1_dn4), (1.034943e-10 * var_t1_dn5), (1.034943e-10 * var_t1_dn6), (1.034943e-10 * var_t1_dn7), (1.034943e-10 * var_t1_dn8), (1.034943e-10 * var_t1_dn9), (1.034943e-10 * var_t1_dn10), (1.034943e-10 * var_t1_dn13),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn13,)
    }
};
        var_t4 = assign60190_e93907;
        var_t4_dn0 = assign60190_e93907_d_n0;
        var_t4_dn2 = assign60190_e93907_d_n2;
        var_t4_dn4 = assign60190_e93907_d_n4;
        var_t4_dn5 = assign60190_e93907_d_n5;
        var_t4_dn6 = assign60190_e93907_d_n6;
        var_t4_dn7 = assign60190_e93907_d_n7;
        var_t4_dn8 = assign60190_e93907_d_n8;
        var_t4_dn9 = assign60190_e93907_d_n9;
        var_t4_dn10 = assign60190_e93907_d_n10;
        var_t4_dn13 = assign60190_e93907_d_n13;
        var_t4_rv = 0.0;

        let (assign60200_e93921, assign60200_e93921_d_n0, assign60200_e93921_d_n2, assign60200_e93921_d_n4, assign60200_e93921_d_n5, assign60200_e93921_d_n6, assign60200_e93921_d_n7, assign60200_e93921_d_n8, assign60200_e93921_d_n9, assign60200_e93921_d_n10, assign60200_e93921_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let assign60200_e93919: f64 = (1.0 - var_uc_clm1);
        (assign60200_e93919, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign60200_e93921;
        var_t1_dn0 = assign60200_e93921_d_n0;
        var_t1_dn2 = assign60200_e93921_d_n2;
        var_t1_dn4 = assign60200_e93921_d_n4;
        var_t1_dn5 = assign60200_e93921_d_n5;
        var_t1_dn6 = assign60200_e93921_d_n6;
        var_t1_dn7 = assign60200_e93921_d_n7;
        var_t1_dn8 = assign60200_e93921_d_n8;
        var_t1_dn9 = assign60200_e93921_d_n9;
        var_t1_dn10 = assign60200_e93921_d_n10;
        var_t1_dn13 = assign60200_e93921_d_n13;
        var_t1_rv = 0.0;

        let (assign60210_e93941, assign60210_e93941_d_n0, assign60210_e93941_d_n2, assign60210_e93941_d_n4, assign60210_e93941_d_n5, assign60210_e93941_d_n6, assign60210_e93941_d_n7, assign60210_e93941_d_n8, assign60210_e93941_d_n9, assign60210_e93941_d_n10, assign60210_e93941_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let assign60210_e93934: f64 = (var_vds + var_ps0);
        let assign60210_e93935: f64 = (var_uc_clm1 * assign60210_e93934);
        let assign60210_e93938: f64 = (var_t1 * var_psl);
        let assign60210_e93939: f64 = (assign60210_e93935 + assign60210_e93938);
        (assign60210_e93939, ((var_uc_clm1 * (var_vds_dn0 + var_ps0_dn0)) + ((var_t1_dn0 * var_psl) + (var_t1 * var_psl_dn0))), ((var_uc_clm1 * (var_vds_dn2 + var_ps0_dn2)) + ((var_t1_dn2 * var_psl) + (var_t1 * var_psl_dn2))), ((var_uc_clm1 * (var_vds_dn4 + var_ps0_dn4)) + ((var_t1_dn4 * var_psl) + (var_t1 * var_psl_dn4))), ((var_uc_clm1 * (var_vds_dn5 + var_ps0_dn5)) + ((var_t1_dn5 * var_psl) + (var_t1 * var_psl_dn5))), ((var_uc_clm1 * (var_vds_dn6 + var_ps0_dn6)) + ((var_t1_dn6 * var_psl) + (var_t1 * var_psl_dn6))), ((var_uc_clm1 * (var_vds_dn7 + var_ps0_dn7)) + ((var_t1_dn7 * var_psl) + (var_t1 * var_psl_dn7))), ((var_uc_clm1 * (var_vds_dn8 + var_ps0_dn8)) + ((var_t1_dn8 * var_psl) + (var_t1 * var_psl_dn8))), ((var_uc_clm1 * (var_vds_dn9 + var_ps0_dn9)) + ((var_t1_dn9 * var_psl) + (var_t1 * var_psl_dn9))), ((var_uc_clm1 * (var_vds_dn10 + var_ps0_dn10)) + ((var_t1_dn10 * var_psl) + (var_t1 * var_psl_dn10))), ((var_uc_clm1 * (var_vds_dn13 + var_ps0_dn13)) + ((var_t1_dn13 * var_psl) + (var_t1 * var_psl_dn13))),)
    } else {
        (var_psdl, var_psdl_dn0, var_psdl_dn2, var_psdl_dn4, var_psdl_dn5, var_psdl_dn6, var_psdl_dn7, var_psdl_dn8, var_psdl_dn9, var_psdl_dn10, var_psdl_dn13,)
    }
};
        var_psdl = assign60210_e93941;
        var_psdl_dn0 = assign60210_e93941_d_n0;
        var_psdl_dn2 = assign60210_e93941_d_n2;
        var_psdl_dn4 = assign60210_e93941_d_n4;
        var_psdl_dn5 = assign60210_e93941_d_n5;
        var_psdl_dn6 = assign60210_e93941_d_n6;
        var_psdl_dn7 = assign60210_e93941_d_n7;
        var_psdl_dn8 = assign60210_e93941_d_n8;
        var_psdl_dn9 = assign60210_e93941_d_n9;
        var_psdl_dn10 = assign60210_e93941_d_n10;
        var_psdl_dn13 = assign60210_e93941_d_n13;
        var_psdl_rv = 0.0;

        let assign60220_e93945: f64 = (var_ps0 + var_vds);
        let assign60220_e93948: f64 = (10.0 * 2.220446049250313e-16);
        let assign60220_e93949: f64 = (assign60220_e93945 - assign60220_e93948);
        let assign60220_e93952: f64 = (10.0 * 2.220446049250313e-16);
        let assign60220_e93953: f64 = (assign60220_e93949 - assign60220_e93952);
        let assign60220_e93957: f64 = (10.0 * 2.220446049250313e-16);
        let assign60220_e93960: f64 = if ((var_psdl > assign60220_e93953) && (assign60220_e93957 >= 0.0)) { 1.0 } else { 0.0 };
        var_guard1469 = assign60220_e93960;
        var_guard1469_rv = 0.0;

        let (assign60230_e93986, assign60230_e93986_d_n0, assign60230_e93986_d_n2, assign60230_e93986_d_n4, assign60230_e93986_d_n5, assign60230_e93986_d_n6, assign60230_e93986_d_n7, assign60230_e93986_d_n8, assign60230_e93986_d_n9, assign60230_e93986_d_n10, assign60230_e93986_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 != 0.0)) {
        let assign60230_e93975: f64 = (var_ps0 + var_vds);
        let assign60230_e93978: f64 = (10.0 * 2.220446049250313e-16);
        let assign60230_e93979: f64 = (assign60230_e93975 - assign60230_e93978);
        let assign60230_e93980: f64 = (var_psdl - assign60230_e93979);
        let assign60230_e93983: f64 = (10.0 * 2.220446049250313e-16);
        let assign60230_e93984: f64 = (assign60230_e93980 + assign60230_e93983);
        (assign60230_e93984, (var_psdl_dn0 - (var_ps0_dn0 + var_vds_dn0)), (var_psdl_dn2 - (var_ps0_dn2 + var_vds_dn2)), (var_psdl_dn4 - (var_ps0_dn4 + var_vds_dn4)), (var_psdl_dn5 - (var_ps0_dn5 + var_vds_dn5)), (var_psdl_dn6 - (var_ps0_dn6 + var_vds_dn6)), (var_psdl_dn7 - (var_ps0_dn7 + var_vds_dn7)), (var_psdl_dn8 - (var_ps0_dn8 + var_vds_dn8)), (var_psdl_dn9 - (var_ps0_dn9 + var_vds_dn9)), (var_psdl_dn10 - (var_ps0_dn10 + var_vds_dn10)), (var_psdl_dn13 - (var_ps0_dn13 + var_vds_dn13)),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn13,)
    }
};
        var_tmf1 = assign60230_e93986;
        var_tmf1_dn0 = assign60230_e93986_d_n0;
        var_tmf1_dn2 = assign60230_e93986_d_n2;
        var_tmf1_dn4 = assign60230_e93986_d_n4;
        var_tmf1_dn5 = assign60230_e93986_d_n5;
        var_tmf1_dn6 = assign60230_e93986_d_n6;
        var_tmf1_dn7 = assign60230_e93986_d_n7;
        var_tmf1_dn8 = assign60230_e93986_d_n8;
        var_tmf1_dn9 = assign60230_e93986_d_n9;
        var_tmf1_dn10 = assign60230_e93986_d_n10;
        var_tmf1_dn13 = assign60230_e93986_d_n13;
        var_tmf1_rv = 0.0;

        let (assign60240_e94002, assign60240_e94002_d_n0, assign60240_e94002_d_n2, assign60240_e94002_d_n4, assign60240_e94002_d_n5, assign60240_e94002_d_n6, assign60240_e94002_d_n7, assign60240_e94002_d_n8, assign60240_e94002_d_n9, assign60240_e94002_d_n10, assign60240_e94002_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 != 0.0)) {
        let assign60240_e94000: f64 = (var_tmf1 * var_tmf1);
        (assign60240_e94000, ((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)), ((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)), ((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)), ((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)), ((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)), ((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)), ((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)), ((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)), ((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)), ((var_tmf1_dn13 * var_tmf1) + (var_tmf1 * var_tmf1_dn13)),)
    } else {
        (var_x2, var_x2_dn0, var_x2_dn2, var_x2_dn4, var_x2_dn5, var_x2_dn6, var_x2_dn7, var_x2_dn8, var_x2_dn9, var_x2_dn10, var_x2_dn13,)
    }
};
        var_x2 = assign60240_e94002;
        var_x2_dn0 = assign60240_e94002_d_n0;
        var_x2_dn2 = assign60240_e94002_d_n2;
        var_x2_dn4 = assign60240_e94002_d_n4;
        var_x2_dn5 = assign60240_e94002_d_n5;
        var_x2_dn6 = assign60240_e94002_d_n6;
        var_x2_dn7 = assign60240_e94002_d_n7;
        var_x2_dn8 = assign60240_e94002_d_n8;
        var_x2_dn9 = assign60240_e94002_d_n9;
        var_x2_dn10 = assign60240_e94002_d_n10;
        var_x2_dn13 = assign60240_e94002_d_n13;
        var_x2_rv = 0.0;

        let (assign60250_e94022, assign60250_e94022_d_n0, assign60250_e94022_d_n2, assign60250_e94022_d_n4, assign60250_e94022_d_n5, assign60250_e94022_d_n6, assign60250_e94022_d_n7, assign60250_e94022_d_n8, assign60250_e94022_d_n9, assign60250_e94022_d_n10, assign60250_e94022_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 != 0.0)) {
        let assign60250_e94016: f64 = (10.0 * 2.220446049250313e-16);
        let assign60250_e94019: f64 = (10.0 * 2.220446049250313e-16);
        let assign60250_e94020: f64 = (assign60250_e94016 * assign60250_e94019);
        (assign60250_e94020, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmax2, var_xmax2_dn0, var_xmax2_dn2, var_xmax2_dn4, var_xmax2_dn5, var_xmax2_dn6, var_xmax2_dn7, var_xmax2_dn8, var_xmax2_dn9, var_xmax2_dn10, var_xmax2_dn13,)
    }
};
        var_xmax2 = assign60250_e94022;
        var_xmax2_dn0 = assign60250_e94022_d_n0;
        var_xmax2_dn2 = assign60250_e94022_d_n2;
        var_xmax2_dn4 = assign60250_e94022_d_n4;
        var_xmax2_dn5 = assign60250_e94022_d_n5;
        var_xmax2_dn6 = assign60250_e94022_d_n6;
        var_xmax2_dn7 = assign60250_e94022_d_n7;
        var_xmax2_dn8 = assign60250_e94022_d_n8;
        var_xmax2_dn9 = assign60250_e94022_d_n9;
        var_xmax2_dn10 = assign60250_e94022_d_n10;
        var_xmax2_dn13 = assign60250_e94022_d_n13;
        var_xmax2_rv = 0.0;

        let (assign60260_e94036, assign60260_e94036_d_n0, assign60260_e94036_d_n2, assign60260_e94036_d_n4, assign60260_e94036_d_n5, assign60260_e94036_d_n6, assign60260_e94036_d_n7, assign60260_e94036_d_n8, assign60260_e94036_d_n9, assign60260_e94036_d_n10, assign60260_e94036_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn7, var_xp_dn8, var_xp_dn9, var_xp_dn10, var_xp_dn13,)
    }
};
        var_xp = assign60260_e94036;
        var_xp_dn0 = assign60260_e94036_d_n0;
        var_xp_dn2 = assign60260_e94036_d_n2;
        var_xp_dn4 = assign60260_e94036_d_n4;
        var_xp_dn5 = assign60260_e94036_d_n5;
        var_xp_dn6 = assign60260_e94036_d_n6;
        var_xp_dn7 = assign60260_e94036_d_n7;
        var_xp_dn8 = assign60260_e94036_d_n8;
        var_xp_dn9 = assign60260_e94036_d_n9;
        var_xp_dn10 = assign60260_e94036_d_n10;
        var_xp_dn13 = assign60260_e94036_d_n13;
        var_xp_rv = 0.0;

        let (assign60270_e94050, assign60270_e94050_d_n0, assign60270_e94050_d_n2, assign60270_e94050_d_n4, assign60270_e94050_d_n5, assign60270_e94050_d_n6, assign60270_e94050_d_n7, assign60270_e94050_d_n8, assign60270_e94050_d_n9, assign60270_e94050_d_n10, assign60270_e94050_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn7, var_xmp_dn8, var_xmp_dn9, var_xmp_dn10, var_xmp_dn13,)
    }
};
        var_xmp = assign60270_e94050;
        var_xmp_dn0 = assign60270_e94050_d_n0;
        var_xmp_dn2 = assign60270_e94050_d_n2;
        var_xmp_dn4 = assign60270_e94050_d_n4;
        var_xmp_dn5 = assign60270_e94050_d_n5;
        var_xmp_dn6 = assign60270_e94050_d_n6;
        var_xmp_dn7 = assign60270_e94050_d_n7;
        var_xmp_dn8 = assign60270_e94050_d_n8;
        var_xmp_dn9 = assign60270_e94050_d_n9;
        var_xmp_dn10 = assign60270_e94050_d_n10;
        var_xmp_dn13 = assign60270_e94050_d_n13;
        var_xmp_rv = 0.0;

        let (assign60280_e94064,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign60280_e94064;
        var_m0_rv = 0.0;

        let (assign60290_e94078,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 != 0.0)) {
        (0.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign60290_e94078;
        var_mm_rv = 0.0;

        let (assign60300_e94092, assign60300_e94092_d_n0, assign60300_e94092_d_n2, assign60300_e94092_d_n4, assign60300_e94092_d_n5, assign60300_e94092_d_n6, assign60300_e94092_d_n7, assign60300_e94092_d_n8, assign60300_e94092_d_n9, assign60300_e94092_d_n10, assign60300_e94092_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_dn10, var_arg_dn13,)
    }
};
        var_arg = assign60300_e94092;
        var_arg_dn0 = assign60300_e94092_d_n0;
        var_arg_dn2 = assign60300_e94092_d_n2;
        var_arg_dn4 = assign60300_e94092_d_n4;
        var_arg_dn5 = assign60300_e94092_d_n5;
        var_arg_dn6 = assign60300_e94092_d_n6;
        var_arg_dn7 = assign60300_e94092_d_n7;
        var_arg_dn8 = assign60300_e94092_d_n8;
        var_arg_dn9 = assign60300_e94092_d_n9;
        var_arg_dn10 = assign60300_e94092_d_n10;
        var_arg_dn13 = assign60300_e94092_d_n13;
        var_arg_rv = 0.0;

        let (assign60310_e94106, assign60310_e94106_d_n0, assign60310_e94106_d_n2, assign60310_e94106_d_n4, assign60310_e94106_d_n5, assign60310_e94106_d_n6, assign60310_e94106_d_n7, assign60310_e94106_d_n8, assign60310_e94106_d_n9, assign60310_e94106_d_n10, assign60310_e94106_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
        var_dnm = assign60310_e94106;
        var_dnm_dn0 = assign60310_e94106_d_n0;
        var_dnm_dn2 = assign60310_e94106_d_n2;
        var_dnm_dn4 = assign60310_e94106_d_n4;
        var_dnm_dn5 = assign60310_e94106_d_n5;
        var_dnm_dn6 = assign60310_e94106_d_n6;
        var_dnm_dn7 = assign60310_e94106_d_n7;
        var_dnm_dn8 = assign60310_e94106_d_n8;
        var_dnm_dn9 = assign60310_e94106_d_n9;
        var_dnm_dn10 = assign60310_e94106_d_n10;
        var_dnm_dn13 = assign60310_e94106_d_n13;
        var_dnm_rv = 0.0;

        let (assign60320_e94122, assign60320_e94122_d_n0, assign60320_e94122_d_n2, assign60320_e94122_d_n4, assign60320_e94122_d_n5, assign60320_e94122_d_n6, assign60320_e94122_d_n7, assign60320_e94122_d_n8, assign60320_e94122_d_n9, assign60320_e94122_d_n10, assign60320_e94122_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 != 0.0)) {
        let assign60320_e94120: f64 = (var_xp * var_x2);
        (assign60320_e94120, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn4 * var_x2) + (var_xp * var_x2_dn4)), ((var_xp_dn5 * var_x2) + (var_xp * var_x2_dn5)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn8 * var_x2) + (var_xp * var_x2_dn8)), ((var_xp_dn9 * var_x2) + (var_xp * var_x2_dn9)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn13 * var_x2) + (var_xp * var_x2_dn13)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn7, var_xp_dn8, var_xp_dn9, var_xp_dn10, var_xp_dn13,)
    }
};
        var_xp = assign60320_e94122;
        var_xp_dn0 = assign60320_e94122_d_n0;
        var_xp_dn2 = assign60320_e94122_d_n2;
        var_xp_dn4 = assign60320_e94122_d_n4;
        var_xp_dn5 = assign60320_e94122_d_n5;
        var_xp_dn6 = assign60320_e94122_d_n6;
        var_xp_dn7 = assign60320_e94122_d_n7;
        var_xp_dn8 = assign60320_e94122_d_n8;
        var_xp_dn9 = assign60320_e94122_d_n9;
        var_xp_dn10 = assign60320_e94122_d_n10;
        var_xp_dn13 = assign60320_e94122_d_n13;
        var_xp_rv = 0.0;

        let (assign60330_e94138, assign60330_e94138_d_n0, assign60330_e94138_d_n2, assign60330_e94138_d_n4, assign60330_e94138_d_n5, assign60330_e94138_d_n6, assign60330_e94138_d_n7, assign60330_e94138_d_n8, assign60330_e94138_d_n9, assign60330_e94138_d_n10, assign60330_e94138_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 != 0.0)) {
        let assign60330_e94136: f64 = (var_xmp * var_xmax2);
        (assign60330_e94136, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn4 * var_xmax2) + (var_xmp * var_xmax2_dn4)), ((var_xmp_dn5 * var_xmax2) + (var_xmp * var_xmax2_dn5)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn8 * var_xmax2) + (var_xmp * var_xmax2_dn8)), ((var_xmp_dn9 * var_xmax2) + (var_xmp * var_xmax2_dn9)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn13 * var_xmax2) + (var_xmp * var_xmax2_dn13)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn7, var_xmp_dn8, var_xmp_dn9, var_xmp_dn10, var_xmp_dn13,)
    }
};
        var_xmp = assign60330_e94138;
        var_xmp_dn0 = assign60330_e94138_d_n0;
        var_xmp_dn2 = assign60330_e94138_d_n2;
        var_xmp_dn4 = assign60330_e94138_d_n4;
        var_xmp_dn5 = assign60330_e94138_d_n5;
        var_xmp_dn6 = assign60330_e94138_d_n6;
        var_xmp_dn7 = assign60330_e94138_d_n7;
        var_xmp_dn8 = assign60330_e94138_d_n8;
        var_xmp_dn9 = assign60330_e94138_d_n9;
        var_xmp_dn10 = assign60330_e94138_d_n10;
        var_xmp_dn13 = assign60330_e94138_d_n13;
        var_xmp_rv = 0.0;

        let (assign60340_e94154, assign60340_e94154_d_n0, assign60340_e94154_d_n2, assign60340_e94154_d_n4, assign60340_e94154_d_n5, assign60340_e94154_d_n6, assign60340_e94154_d_n7, assign60340_e94154_d_n8, assign60340_e94154_d_n9, assign60340_e94154_d_n10, assign60340_e94154_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 != 0.0)) {
        let assign60340_e94152: f64 = (var_xp * var_x2);
        (assign60340_e94152, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn4 * var_x2) + (var_xp * var_x2_dn4)), ((var_xp_dn5 * var_x2) + (var_xp * var_x2_dn5)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn8 * var_x2) + (var_xp * var_x2_dn8)), ((var_xp_dn9 * var_x2) + (var_xp * var_x2_dn9)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn13 * var_x2) + (var_xp * var_x2_dn13)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn7, var_xp_dn8, var_xp_dn9, var_xp_dn10, var_xp_dn13,)
    }
};
        var_xp = assign60340_e94154;
        var_xp_dn0 = assign60340_e94154_d_n0;
        var_xp_dn2 = assign60340_e94154_d_n2;
        var_xp_dn4 = assign60340_e94154_d_n4;
        var_xp_dn5 = assign60340_e94154_d_n5;
        var_xp_dn6 = assign60340_e94154_d_n6;
        var_xp_dn7 = assign60340_e94154_d_n7;
        var_xp_dn8 = assign60340_e94154_d_n8;
        var_xp_dn9 = assign60340_e94154_d_n9;
        var_xp_dn10 = assign60340_e94154_d_n10;
        var_xp_dn13 = assign60340_e94154_d_n13;
        var_xp_rv = 0.0;

        let (assign60350_e94170, assign60350_e94170_d_n0, assign60350_e94170_d_n2, assign60350_e94170_d_n4, assign60350_e94170_d_n5, assign60350_e94170_d_n6, assign60350_e94170_d_n7, assign60350_e94170_d_n8, assign60350_e94170_d_n9, assign60350_e94170_d_n10, assign60350_e94170_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 != 0.0)) {
        let assign60350_e94168: f64 = (var_xmp * var_xmax2);
        (assign60350_e94168, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn4 * var_xmax2) + (var_xmp * var_xmax2_dn4)), ((var_xmp_dn5 * var_xmax2) + (var_xmp * var_xmax2_dn5)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn8 * var_xmax2) + (var_xmp * var_xmax2_dn8)), ((var_xmp_dn9 * var_xmax2) + (var_xmp * var_xmax2_dn9)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn13 * var_xmax2) + (var_xmp * var_xmax2_dn13)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn7, var_xmp_dn8, var_xmp_dn9, var_xmp_dn10, var_xmp_dn13,)
    }
};
        var_xmp = assign60350_e94170;
        var_xmp_dn0 = assign60350_e94170_d_n0;
        var_xmp_dn2 = assign60350_e94170_d_n2;
        var_xmp_dn4 = assign60350_e94170_d_n4;
        var_xmp_dn5 = assign60350_e94170_d_n5;
        var_xmp_dn6 = assign60350_e94170_d_n6;
        var_xmp_dn7 = assign60350_e94170_d_n7;
        var_xmp_dn8 = assign60350_e94170_d_n8;
        var_xmp_dn9 = assign60350_e94170_d_n9;
        var_xmp_dn10 = assign60350_e94170_d_n10;
        var_xmp_dn13 = assign60350_e94170_d_n13;
        var_xmp_rv = 0.0;

        let (assign60360_e94186, assign60360_e94186_d_n0, assign60360_e94186_d_n2, assign60360_e94186_d_n4, assign60360_e94186_d_n5, assign60360_e94186_d_n6, assign60360_e94186_d_n7, assign60360_e94186_d_n8, assign60360_e94186_d_n9, assign60360_e94186_d_n10, assign60360_e94186_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 != 0.0)) {
        let assign60360_e94184: f64 = (var_xp + var_xmp);
        (assign60360_e94184, (var_xp_dn0 + var_xmp_dn0), (var_xp_dn2 + var_xmp_dn2), (var_xp_dn4 + var_xmp_dn4), (var_xp_dn5 + var_xmp_dn5), (var_xp_dn6 + var_xmp_dn6), (var_xp_dn7 + var_xmp_dn7), (var_xp_dn8 + var_xmp_dn8), (var_xp_dn9 + var_xmp_dn9), (var_xp_dn10 + var_xmp_dn10), (var_xp_dn13 + var_xmp_dn13),)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_dn10, var_arg_dn13,)
    }
};
        var_arg = assign60360_e94186;
        var_arg_dn0 = assign60360_e94186_d_n0;
        var_arg_dn2 = assign60360_e94186_d_n2;
        var_arg_dn4 = assign60360_e94186_d_n4;
        var_arg_dn5 = assign60360_e94186_d_n5;
        var_arg_dn6 = assign60360_e94186_d_n6;
        var_arg_dn7 = assign60360_e94186_d_n7;
        var_arg_dn8 = assign60360_e94186_d_n8;
        var_arg_dn9 = assign60360_e94186_d_n9;
        var_arg_dn10 = assign60360_e94186_d_n10;
        var_arg_dn13 = assign60360_e94186_d_n13;
        var_arg_rv = 0.0;

        let (assign60370_e94200, assign60370_e94200_d_n0, assign60370_e94200_d_n2, assign60370_e94200_d_n4, assign60370_e94200_d_n5, assign60370_e94200_d_n6, assign60370_e94200_d_n7, assign60370_e94200_d_n8, assign60370_e94200_d_n9, assign60370_e94200_d_n10, assign60370_e94200_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 != 0.0)) {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_dn10, var_arg_dn13,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
        var_dnm = assign60370_e94200;
        var_dnm_dn0 = assign60370_e94200_d_n0;
        var_dnm_dn2 = assign60370_e94200_d_n2;
        var_dnm_dn4 = assign60370_e94200_d_n4;
        var_dnm_dn5 = assign60370_e94200_d_n5;
        var_dnm_dn6 = assign60370_e94200_d_n6;
        var_dnm_dn7 = assign60370_e94200_d_n7;
        var_dnm_dn8 = assign60370_e94200_d_n8;
        var_dnm_dn9 = assign60370_e94200_d_n9;
        var_dnm_dn10 = assign60370_e94200_d_n10;
        var_dnm_dn13 = assign60370_e94200_d_n13;
        var_dnm_rv = 0.0;

        let assign60380_e94215: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        var_guard1470 = assign60380_e94215;
        var_guard1470_rv = 0.0;

        let assign60390_e94218: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard1471 = assign60390_e94218;
        var_guard1471_rv = 0.0;

        let (assign60400_e94236,) = {
    if (((((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 != 0.0)) && (var_guard1470 != 0.0)) && (var_guard1471 != 0.0)) {
        (1.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign60400_e94236;
        var_mm_rv = 0.0;

        let assign60410_e94239: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        var_guard1472 = assign60410_e94239;
        var_guard1472_rv = 0.0;

        let (assign60420_e94260,) = {
    if ((((((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 != 0.0)) && (var_guard1470 != 0.0)) && (var_guard1471 == 0.0)) && (var_guard1472 != 0.0)) {
        (2.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign60420_e94260;
        var_mm_rv = 0.0;

        let assign60430_e94263: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        var_guard1473 = assign60430_e94263;
        var_guard1473_rv = 0.0;

        let (assign60440_e94287,) = {
    if (((((((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 != 0.0)) && (var_guard1470 != 0.0)) && (var_guard1471 == 0.0)) && (var_guard1472 == 0.0)) && (var_guard1473 != 0.0)) {
        (3.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign60440_e94287;
        var_mm_rv = 0.0;

        let assign60450_e94290: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        var_guard1474 = assign60450_e94290;
        var_guard1474_rv = 0.0;

        let (assign60460_e94317,) = {
    if ((((((((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 != 0.0)) && (var_guard1470 != 0.0)) && (var_guard1471 == 0.0)) && (var_guard1472 == 0.0)) && (var_guard1473 == 0.0)) && (var_guard1474 != 0.0)) {
        (4.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign60460_e94317;
        var_mm_rv = 0.0;

        let (assign60470_e94333,) = {
    if ((((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 != 0.0)) && (var_guard1470 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign60470_e94333;
        var_m0_rv = 0.0;

        let mut assign60480_loop_guard: usize = 0;
        while {
            let assign60480_cond_e94350: f64 = if (((((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 != 0.0)) && (var_guard1470 != 0.0)) && (var_m0 < var_mm)) { 1.0 } else { 0.0 };
            assign60480_cond_e94350 != 0.0
        } {
            assign60480_loop_guard += 1;
            assert!(assign60480_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign60480_body0_e94367, assign60480_body0_e94367_d_n0, assign60480_body0_e94367_d_n2, assign60480_body0_e94367_d_n4, assign60480_body0_e94367_d_n5, assign60480_body0_e94367_d_n6, assign60480_body0_e94367_d_n7, assign60480_body0_e94367_d_n8, assign60480_body0_e94367_d_n9, assign60480_body0_e94367_d_n10, assign60480_body0_e94367_d_n13,) = {
    if ((((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 != 0.0)) && (var_guard1470 != 0.0)) {
        let assign60480_body0_e94365: f64 = (var_dnm).sqrt();
        (assign60480_body0_e94365, (var_dnm_dn0 / (2.0 * assign60480_body0_e94365)), (var_dnm_dn2 / (2.0 * assign60480_body0_e94365)), (var_dnm_dn4 / (2.0 * assign60480_body0_e94365)), (var_dnm_dn5 / (2.0 * assign60480_body0_e94365)), (var_dnm_dn6 / (2.0 * assign60480_body0_e94365)), (var_dnm_dn7 / (2.0 * assign60480_body0_e94365)), (var_dnm_dn8 / (2.0 * assign60480_body0_e94365)), (var_dnm_dn9 / (2.0 * assign60480_body0_e94365)), (var_dnm_dn10 / (2.0 * assign60480_body0_e94365)), (var_dnm_dn13 / (2.0 * assign60480_body0_e94365)),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
            var_dnm = assign60480_body0_e94367;
            var_dnm_dn0 = assign60480_body0_e94367_d_n0;
            var_dnm_dn2 = assign60480_body0_e94367_d_n2;
            var_dnm_dn4 = assign60480_body0_e94367_d_n4;
            var_dnm_dn5 = assign60480_body0_e94367_d_n5;
            var_dnm_dn6 = assign60480_body0_e94367_d_n6;
            var_dnm_dn7 = assign60480_body0_e94367_d_n7;
            var_dnm_dn8 = assign60480_body0_e94367_d_n8;
            var_dnm_dn9 = assign60480_body0_e94367_d_n9;
            var_dnm_dn10 = assign60480_body0_e94367_d_n10;
            var_dnm_dn13 = assign60480_body0_e94367_d_n13;
            var_dnm_rv = 0.0;
            let (assign60480_body1_e94385,) = {
    if ((((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 != 0.0)) && (var_guard1470 != 0.0)) {
        let assign60480_body1_e94383: f64 = (var_m0 + 1.0);
        (assign60480_body1_e94383,)
    } else {
        (var_m0,)
    }
};
            var_m0 = assign60480_body1_e94385;
            var_m0_rv = 0.0;
        }

        *var_arg_slot = var_arg;
        *var_arg_dn0_slot = var_arg_dn0;
        *var_arg_dn10_slot = var_arg_dn10;
        *var_arg_dn13_slot = var_arg_dn13;
        *var_arg_dn2_slot = var_arg_dn2;
        *var_arg_dn4_slot = var_arg_dn4;
        *var_arg_dn5_slot = var_arg_dn5;
        *var_arg_dn6_slot = var_arg_dn6;
        *var_arg_dn7_slot = var_arg_dn7;
        *var_arg_dn8_slot = var_arg_dn8;
        *var_arg_dn9_slot = var_arg_dn9;
        *var_arg_rv_slot = var_arg_rv;
        *var_dnm_slot = var_dnm;
        *var_dnm_dn0_slot = var_dnm_dn0;
        *var_dnm_dn10_slot = var_dnm_dn10;
        *var_dnm_dn13_slot = var_dnm_dn13;
        *var_dnm_dn2_slot = var_dnm_dn2;
        *var_dnm_dn4_slot = var_dnm_dn4;
        *var_dnm_dn5_slot = var_dnm_dn5;
        *var_dnm_dn6_slot = var_dnm_dn6;
        *var_dnm_dn7_slot = var_dnm_dn7;
        *var_dnm_dn8_slot = var_dnm_dn8;
        *var_dnm_dn9_slot = var_dnm_dn9;
        *var_dnm_rv_slot = var_dnm_rv;
        *var_guard1469_slot = var_guard1469;
        *var_guard1469_rv_slot = var_guard1469_rv;
        *var_guard1470_slot = var_guard1470;
        *var_guard1470_rv_slot = var_guard1470_rv;
        *var_guard1471_slot = var_guard1471;
        *var_guard1471_rv_slot = var_guard1471_rv;
        *var_guard1472_slot = var_guard1472;
        *var_guard1472_rv_slot = var_guard1472_rv;
        *var_guard1473_slot = var_guard1473;
        *var_guard1473_rv_slot = var_guard1473_rv;
        *var_guard1474_slot = var_guard1474;
        *var_guard1474_rv_slot = var_guard1474_rv;
        *var_m0_slot = var_m0;
        *var_m0_rv_slot = var_m0_rv;
        *var_mm_slot = var_mm;
        *var_mm_rv_slot = var_mm_rv;
        *var_psdl_slot = var_psdl;
        *var_psdl_dn0_slot = var_psdl_dn0;
        *var_psdl_dn10_slot = var_psdl_dn10;
        *var_psdl_dn13_slot = var_psdl_dn13;
        *var_psdl_dn2_slot = var_psdl_dn2;
        *var_psdl_dn4_slot = var_psdl_dn4;
        *var_psdl_dn5_slot = var_psdl_dn5;
        *var_psdl_dn6_slot = var_psdl_dn6;
        *var_psdl_dn7_slot = var_psdl_dn7;
        *var_psdl_dn8_slot = var_psdl_dn8;
        *var_psdl_dn9_slot = var_psdl_dn9;
        *var_psdl_rv_slot = var_psdl_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn13_slot = var_t1_dn13;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn13_slot = var_t4_dn13;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_dn9_slot = var_t4_dn9;
        *var_t4_rv_slot = var_t4_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn13_slot = var_tmf1_dn13;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn7_slot = var_tmf1_dn7;
        *var_tmf1_dn8_slot = var_tmf1_dn8;
        *var_tmf1_dn9_slot = var_tmf1_dn9;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_x2_slot = var_x2;
        *var_x2_dn0_slot = var_x2_dn0;
        *var_x2_dn10_slot = var_x2_dn10;
        *var_x2_dn13_slot = var_x2_dn13;
        *var_x2_dn2_slot = var_x2_dn2;
        *var_x2_dn4_slot = var_x2_dn4;
        *var_x2_dn5_slot = var_x2_dn5;
        *var_x2_dn6_slot = var_x2_dn6;
        *var_x2_dn7_slot = var_x2_dn7;
        *var_x2_dn8_slot = var_x2_dn8;
        *var_x2_dn9_slot = var_x2_dn9;
        *var_x2_rv_slot = var_x2_rv;
        *var_xmax2_slot = var_xmax2;
        *var_xmax2_dn0_slot = var_xmax2_dn0;
        *var_xmax2_dn10_slot = var_xmax2_dn10;
        *var_xmax2_dn13_slot = var_xmax2_dn13;
        *var_xmax2_dn2_slot = var_xmax2_dn2;
        *var_xmax2_dn4_slot = var_xmax2_dn4;
        *var_xmax2_dn5_slot = var_xmax2_dn5;
        *var_xmax2_dn6_slot = var_xmax2_dn6;
        *var_xmax2_dn7_slot = var_xmax2_dn7;
        *var_xmax2_dn8_slot = var_xmax2_dn8;
        *var_xmax2_dn9_slot = var_xmax2_dn9;
        *var_xmax2_rv_slot = var_xmax2_rv;
        *var_xmp_slot = var_xmp;
        *var_xmp_dn0_slot = var_xmp_dn0;
        *var_xmp_dn10_slot = var_xmp_dn10;
        *var_xmp_dn13_slot = var_xmp_dn13;
        *var_xmp_dn2_slot = var_xmp_dn2;
        *var_xmp_dn4_slot = var_xmp_dn4;
        *var_xmp_dn5_slot = var_xmp_dn5;
        *var_xmp_dn6_slot = var_xmp_dn6;
        *var_xmp_dn7_slot = var_xmp_dn7;
        *var_xmp_dn8_slot = var_xmp_dn8;
        *var_xmp_dn9_slot = var_xmp_dn9;
        *var_xmp_rv_slot = var_xmp_rv;
        *var_xp_slot = var_xp;
        *var_xp_dn0_slot = var_xp_dn0;
        *var_xp_dn10_slot = var_xp_dn10;
        *var_xp_dn13_slot = var_xp_dn13;
        *var_xp_dn2_slot = var_xp_dn2;
        *var_xp_dn4_slot = var_xp_dn4;
        *var_xp_dn5_slot = var_xp_dn5;
        *var_xp_dn6_slot = var_xp_dn6;
        *var_xp_dn7_slot = var_xp_dn7;
        *var_xp_dn8_slot = var_xp_dn8;
        *var_xp_dn9_slot = var_xp_dn9;
        *var_xp_rv_slot = var_xp_rv;
    }

    pub(super) fn stamp_reactive_block_216(
        var_arg: f64,
        var_arg_dn0: f64,
        var_arg_dn10: f64,
        var_arg_dn13: f64,
        var_arg_dn2: f64,
        var_arg_dn4: f64,
        var_arg_dn5: f64,
        var_arg_dn6: f64,
        var_arg_dn7: f64,
        var_arg_dn8: f64,
        var_arg_dn9: f64,
        var_beta: f64,
        var_beta_dn0: f64,
        var_beta_dn10: f64,
        var_beta_dn13: f64,
        var_beta_dn2: f64,
        var_beta_dn4: f64,
        var_beta_dn5: f64,
        var_beta_dn6: f64,
        var_beta_dn7: f64,
        var_beta_dn8: f64,
        var_beta_dn9: f64,
        var_fdd: f64,
        var_fdd_dn0: f64,
        var_fdd_dn10: f64,
        var_fdd_dn13: f64,
        var_fdd_dn2: f64,
        var_fdd_dn4: f64,
        var_fdd_dn5: f64,
        var_fdd_dn6: f64,
        var_fdd_dn7: f64,
        var_fdd_dn8: f64,
        var_fdd_dn9: f64,
        var_guard1430: f64,
        var_guard1461: f64,
        var_guard1462: f64,
        var_guard1469: f64,
        var_guard1470: f64,
        var_guard443: f64,
        var_leff: f64,
        var_pds: f64,
        var_pds_dn0: f64,
        var_pds_dn10: f64,
        var_pds_dn13: f64,
        var_pds_dn2: f64,
        var_pds_dn4: f64,
        var_pds_dn5: f64,
        var_pds_dn6: f64,
        var_pds_dn7: f64,
        var_pds_dn8: f64,
        var_pds_dn9: f64,
        var_ps0: f64,
        var_ps0_dn0: f64,
        var_ps0_dn10: f64,
        var_ps0_dn13: f64,
        var_ps0_dn2: f64,
        var_ps0_dn4: f64,
        var_ps0_dn5: f64,
        var_ps0_dn6: f64,
        var_ps0_dn7: f64,
        var_ps0_dn8: f64,
        var_ps0_dn9: f64,
        var_psl: f64,
        var_psl_dn0: f64,
        var_psl_dn10: f64,
        var_psl_dn13: f64,
        var_psl_dn2: f64,
        var_psl_dn4: f64,
        var_psl_dn5: f64,
        var_psl_dn6: f64,
        var_psl_dn7: f64,
        var_psl_dn8: f64,
        var_psl_dn9: f64,
        var_q_nsub: f64,
        var_q_nsub_dn0: f64,
        var_q_nsub_dn10: f64,
        var_q_nsub_dn13: f64,
        var_q_nsub_dn2: f64,
        var_q_nsub_dn4: f64,
        var_q_nsub_dn5: f64,
        var_q_nsub_dn6: f64,
        var_q_nsub_dn7: f64,
        var_q_nsub_dn8: f64,
        var_q_nsub_dn9: f64,
        var_qn0: f64,
        var_qn0_dn0: f64,
        var_qn0_dn10: f64,
        var_qn0_dn13: f64,
        var_qn0_dn2: f64,
        var_qn0_dn4: f64,
        var_qn0_dn5: f64,
        var_qn0_dn6: f64,
        var_qn0_dn7: f64,
        var_qn0_dn8: f64,
        var_qn0_dn9: f64,
        var_t4: f64,
        var_t4_dn0: f64,
        var_t4_dn10: f64,
        var_t4_dn13: f64,
        var_t4_dn2: f64,
        var_t4_dn4: f64,
        var_t4_dn5: f64,
        var_t4_dn6: f64,
        var_t4_dn7: f64,
        var_t4_dn8: f64,
        var_t4_dn9: f64,
        var_tmf1: f64,
        var_tmf1_dn0: f64,
        var_tmf1_dn10: f64,
        var_tmf1_dn13: f64,
        var_tmf1_dn2: f64,
        var_tmf1_dn4: f64,
        var_tmf1_dn5: f64,
        var_tmf1_dn6: f64,
        var_tmf1_dn7: f64,
        var_tmf1_dn8: f64,
        var_tmf1_dn9: f64,
        var_vds: f64,
        var_vds_dn0: f64,
        var_vds_dn10: f64,
        var_vds_dn13: f64,
        var_vds_dn2: f64,
        var_vds_dn4: f64,
        var_vds_dn5: f64,
        var_vds_dn6: f64,
        var_vds_dn7: f64,
        var_vds_dn8: f64,
        var_vds_dn9: f64,
        var_xmp: f64,
        var_xmp_dn0: f64,
        var_xmp_dn10: f64,
        var_xmp_dn13: f64,
        var_xmp_dn2: f64,
        var_xmp_dn4: f64,
        var_xmp_dn5: f64,
        var_xmp_dn6: f64,
        var_xmp_dn7: f64,
        var_xmp_dn8: f64,
        var_xmp_dn9: f64,
        var_dnm_slot: &mut f64,
        var_dnm_dn0_slot: &mut f64,
        var_dnm_dn10_slot: &mut f64,
        var_dnm_dn13_slot: &mut f64,
        var_dnm_dn2_slot: &mut f64,
        var_dnm_dn4_slot: &mut f64,
        var_dnm_dn5_slot: &mut f64,
        var_dnm_dn6_slot: &mut f64,
        var_dnm_dn7_slot: &mut f64,
        var_dnm_dn8_slot: &mut f64,
        var_dnm_dn9_slot: &mut f64,
        var_dnm_rv_slot: &mut f64,
        var_psdl_slot: &mut f64,
        var_psdl_dn0_slot: &mut f64,
        var_psdl_dn10_slot: &mut f64,
        var_psdl_dn13_slot: &mut f64,
        var_psdl_dn2_slot: &mut f64,
        var_psdl_dn4_slot: &mut f64,
        var_psdl_dn5_slot: &mut f64,
        var_psdl_dn6_slot: &mut f64,
        var_psdl_dn7_slot: &mut f64,
        var_psdl_dn8_slot: &mut f64,
        var_psdl_dn9_slot: &mut f64,
        var_psdl_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn13_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t10_slot: &mut f64,
        var_t10_dn0_slot: &mut f64,
        var_t10_dn10_slot: &mut f64,
        var_t10_dn13_slot: &mut f64,
        var_t10_dn2_slot: &mut f64,
        var_t10_dn4_slot: &mut f64,
        var_t10_dn5_slot: &mut f64,
        var_t10_dn6_slot: &mut f64,
        var_t10_dn7_slot: &mut f64,
        var_t10_dn8_slot: &mut f64,
        var_t10_dn9_slot: &mut f64,
        var_t10_rv_slot: &mut f64,
        var_t11_slot: &mut f64,
        var_t11_dn0_slot: &mut f64,
        var_t11_dn10_slot: &mut f64,
        var_t11_dn13_slot: &mut f64,
        var_t11_dn2_slot: &mut f64,
        var_t11_dn4_slot: &mut f64,
        var_t11_dn5_slot: &mut f64,
        var_t11_dn6_slot: &mut f64,
        var_t11_dn7_slot: &mut f64,
        var_t11_dn8_slot: &mut f64,
        var_t11_dn9_slot: &mut f64,
        var_t11_rv_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn13_slot: &mut f64,
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
        var_t2_dn13_slot: &mut f64,
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
        var_t3_dn13_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn10_slot: &mut f64,
        var_t5_dn13_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t5_dn9_slot: &mut f64,
        var_t5_rv_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn13_slot: &mut f64,
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
        var_t7_dn13_slot: &mut f64,
        var_t7_dn2_slot: &mut f64,
        var_t7_dn4_slot: &mut f64,
        var_t7_dn5_slot: &mut f64,
        var_t7_dn6_slot: &mut f64,
        var_t7_dn7_slot: &mut f64,
        var_t7_dn8_slot: &mut f64,
        var_t7_dn9_slot: &mut f64,
        var_t7_rv_slot: &mut f64,
        var_tmf0_slot: &mut f64,
        var_tmf0_dn0_slot: &mut f64,
        var_tmf0_dn10_slot: &mut f64,
        var_tmf0_dn13_slot: &mut f64,
        var_tmf0_dn2_slot: &mut f64,
        var_tmf0_dn4_slot: &mut f64,
        var_tmf0_dn5_slot: &mut f64,
        var_tmf0_dn6_slot: &mut f64,
        var_tmf0_dn7_slot: &mut f64,
        var_tmf0_dn8_slot: &mut f64,
        var_tmf0_dn9_slot: &mut f64,
        var_tmf0_rv_slot: &mut f64,
    ) {
        let mut var_dnm: f64 = *var_dnm_slot;
        let mut var_dnm_dn0: f64 = *var_dnm_dn0_slot;
        let mut var_dnm_dn10: f64 = *var_dnm_dn10_slot;
        let mut var_dnm_dn13: f64 = *var_dnm_dn13_slot;
        let mut var_dnm_dn2: f64 = *var_dnm_dn2_slot;
        let mut var_dnm_dn4: f64 = *var_dnm_dn4_slot;
        let mut var_dnm_dn5: f64 = *var_dnm_dn5_slot;
        let mut var_dnm_dn6: f64 = *var_dnm_dn6_slot;
        let mut var_dnm_dn7: f64 = *var_dnm_dn7_slot;
        let mut var_dnm_dn8: f64 = *var_dnm_dn8_slot;
        let mut var_dnm_dn9: f64 = *var_dnm_dn9_slot;
        let mut var_dnm_rv: f64 = *var_dnm_rv_slot;
        let mut var_psdl: f64 = *var_psdl_slot;
        let mut var_psdl_dn0: f64 = *var_psdl_dn0_slot;
        let mut var_psdl_dn10: f64 = *var_psdl_dn10_slot;
        let mut var_psdl_dn13: f64 = *var_psdl_dn13_slot;
        let mut var_psdl_dn2: f64 = *var_psdl_dn2_slot;
        let mut var_psdl_dn4: f64 = *var_psdl_dn4_slot;
        let mut var_psdl_dn5: f64 = *var_psdl_dn5_slot;
        let mut var_psdl_dn6: f64 = *var_psdl_dn6_slot;
        let mut var_psdl_dn7: f64 = *var_psdl_dn7_slot;
        let mut var_psdl_dn8: f64 = *var_psdl_dn8_slot;
        let mut var_psdl_dn9: f64 = *var_psdl_dn9_slot;
        let mut var_psdl_rv: f64 = *var_psdl_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn13: f64 = *var_t0_dn13_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t10: f64 = *var_t10_slot;
        let mut var_t10_dn0: f64 = *var_t10_dn0_slot;
        let mut var_t10_dn10: f64 = *var_t10_dn10_slot;
        let mut var_t10_dn13: f64 = *var_t10_dn13_slot;
        let mut var_t10_dn2: f64 = *var_t10_dn2_slot;
        let mut var_t10_dn4: f64 = *var_t10_dn4_slot;
        let mut var_t10_dn5: f64 = *var_t10_dn5_slot;
        let mut var_t10_dn6: f64 = *var_t10_dn6_slot;
        let mut var_t10_dn7: f64 = *var_t10_dn7_slot;
        let mut var_t10_dn8: f64 = *var_t10_dn8_slot;
        let mut var_t10_dn9: f64 = *var_t10_dn9_slot;
        let mut var_t10_rv: f64 = *var_t10_rv_slot;
        let mut var_t11: f64 = *var_t11_slot;
        let mut var_t11_dn0: f64 = *var_t11_dn0_slot;
        let mut var_t11_dn10: f64 = *var_t11_dn10_slot;
        let mut var_t11_dn13: f64 = *var_t11_dn13_slot;
        let mut var_t11_dn2: f64 = *var_t11_dn2_slot;
        let mut var_t11_dn4: f64 = *var_t11_dn4_slot;
        let mut var_t11_dn5: f64 = *var_t11_dn5_slot;
        let mut var_t11_dn6: f64 = *var_t11_dn6_slot;
        let mut var_t11_dn7: f64 = *var_t11_dn7_slot;
        let mut var_t11_dn8: f64 = *var_t11_dn8_slot;
        let mut var_t11_dn9: f64 = *var_t11_dn9_slot;
        let mut var_t11_rv: f64 = *var_t11_rv_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
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
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
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
        let mut var_t3_dn13: f64 = *var_t3_dn13_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn10: f64 = *var_t5_dn10_slot;
        let mut var_t5_dn13: f64 = *var_t5_dn13_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t5_dn9: f64 = *var_t5_dn9_slot;
        let mut var_t5_rv: f64 = *var_t5_rv_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn13: f64 = *var_t6_dn13_slot;
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
        let mut var_t7_dn13: f64 = *var_t7_dn13_slot;
        let mut var_t7_dn2: f64 = *var_t7_dn2_slot;
        let mut var_t7_dn4: f64 = *var_t7_dn4_slot;
        let mut var_t7_dn5: f64 = *var_t7_dn5_slot;
        let mut var_t7_dn6: f64 = *var_t7_dn6_slot;
        let mut var_t7_dn7: f64 = *var_t7_dn7_slot;
        let mut var_t7_dn8: f64 = *var_t7_dn8_slot;
        let mut var_t7_dn9: f64 = *var_t7_dn9_slot;
        let mut var_t7_rv: f64 = *var_t7_rv_slot;
        let mut var_tmf0: f64 = *var_tmf0_slot;
        let mut var_tmf0_dn0: f64 = *var_tmf0_dn0_slot;
        let mut var_tmf0_dn10: f64 = *var_tmf0_dn10_slot;
        let mut var_tmf0_dn13: f64 = *var_tmf0_dn13_slot;
        let mut var_tmf0_dn2: f64 = *var_tmf0_dn2_slot;
        let mut var_tmf0_dn4: f64 = *var_tmf0_dn4_slot;
        let mut var_tmf0_dn5: f64 = *var_tmf0_dn5_slot;
        let mut var_tmf0_dn6: f64 = *var_tmf0_dn6_slot;
        let mut var_tmf0_dn7: f64 = *var_tmf0_dn7_slot;
        let mut var_tmf0_dn8: f64 = *var_tmf0_dn8_slot;
        let mut var_tmf0_dn9: f64 = *var_tmf0_dn9_slot;
        let mut var_tmf0_rv: f64 = *var_tmf0_rv_slot;

        let (assign60490_e94413, assign60490_e94413_d_n0, assign60490_e94413_d_n2, assign60490_e94413_d_n4, assign60490_e94413_d_n5, assign60490_e94413_d_n6, assign60490_e94413_d_n7, assign60490_e94413_d_n8, assign60490_e94413_d_n9, assign60490_e94413_d_n10, assign60490_e94413_d_n13,) = {
    if ((((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 != 0.0)) && (var_guard1470 == 0.0)) {
        let (assign60490_e94411, assign60490_e94411_d_n0, assign60490_e94411_d_n2, assign60490_e94411_d_n4, assign60490_e94411_d_n5, assign60490_e94411_d_n6, assign60490_e94411_d_n7, assign60490_e94411_d_n8, assign60490_e94411_d_n9, assign60490_e94411_d_n10, assign60490_e94411_d_n13,) = {
            if (var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign60490_e94408: f64 = (2.0 * 2.0);
                let assign60490_e94409: f64 = (1.0 / assign60490_e94408);
                let assign60490_e94410: f64 = (var_dnm).powf(assign60490_e94409);
                (assign60490_e94410, if 0.0 == 0.0 && ((assign60490_e94409) as f64).is_finite() && ((assign60490_e94409) as f64).fract() == 0.0 { if assign60490_e94409 == 0.0 { 0.0 } else { (assign60490_e94409 * ((var_dnm).powf(assign60490_e94409 - 1.0) * var_dnm_dn0)) } } else { (assign60490_e94410 * (assign60490_e94409 * (var_dnm_dn0 / var_dnm))) }, if 0.0 == 0.0 && ((assign60490_e94409) as f64).is_finite() && ((assign60490_e94409) as f64).fract() == 0.0 { if assign60490_e94409 == 0.0 { 0.0 } else { (assign60490_e94409 * ((var_dnm).powf(assign60490_e94409 - 1.0) * var_dnm_dn2)) } } else { (assign60490_e94410 * (assign60490_e94409 * (var_dnm_dn2 / var_dnm))) }, if 0.0 == 0.0 && ((assign60490_e94409) as f64).is_finite() && ((assign60490_e94409) as f64).fract() == 0.0 { if assign60490_e94409 == 0.0 { 0.0 } else { (assign60490_e94409 * ((var_dnm).powf(assign60490_e94409 - 1.0) * var_dnm_dn4)) } } else { (assign60490_e94410 * (assign60490_e94409 * (var_dnm_dn4 / var_dnm))) }, if 0.0 == 0.0 && ((assign60490_e94409) as f64).is_finite() && ((assign60490_e94409) as f64).fract() == 0.0 { if assign60490_e94409 == 0.0 { 0.0 } else { (assign60490_e94409 * ((var_dnm).powf(assign60490_e94409 - 1.0) * var_dnm_dn5)) } } else { (assign60490_e94410 * (assign60490_e94409 * (var_dnm_dn5 / var_dnm))) }, if 0.0 == 0.0 && ((assign60490_e94409) as f64).is_finite() && ((assign60490_e94409) as f64).fract() == 0.0 { if assign60490_e94409 == 0.0 { 0.0 } else { (assign60490_e94409 * ((var_dnm).powf(assign60490_e94409 - 1.0) * var_dnm_dn6)) } } else { (assign60490_e94410 * (assign60490_e94409 * (var_dnm_dn6 / var_dnm))) }, if 0.0 == 0.0 && ((assign60490_e94409) as f64).is_finite() && ((assign60490_e94409) as f64).fract() == 0.0 { if assign60490_e94409 == 0.0 { 0.0 } else { (assign60490_e94409 * ((var_dnm).powf(assign60490_e94409 - 1.0) * var_dnm_dn7)) } } else { (assign60490_e94410 * (assign60490_e94409 * (var_dnm_dn7 / var_dnm))) }, if 0.0 == 0.0 && ((assign60490_e94409) as f64).is_finite() && ((assign60490_e94409) as f64).fract() == 0.0 { if assign60490_e94409 == 0.0 { 0.0 } else { (assign60490_e94409 * ((var_dnm).powf(assign60490_e94409 - 1.0) * var_dnm_dn8)) } } else { (assign60490_e94410 * (assign60490_e94409 * (var_dnm_dn8 / var_dnm))) }, if 0.0 == 0.0 && ((assign60490_e94409) as f64).is_finite() && ((assign60490_e94409) as f64).fract() == 0.0 { if assign60490_e94409 == 0.0 { 0.0 } else { (assign60490_e94409 * ((var_dnm).powf(assign60490_e94409 - 1.0) * var_dnm_dn9)) } } else { (assign60490_e94410 * (assign60490_e94409 * (var_dnm_dn9 / var_dnm))) }, if 0.0 == 0.0 && ((assign60490_e94409) as f64).is_finite() && ((assign60490_e94409) as f64).fract() == 0.0 { if assign60490_e94409 == 0.0 { 0.0 } else { (assign60490_e94409 * ((var_dnm).powf(assign60490_e94409 - 1.0) * var_dnm_dn10)) } } else { (assign60490_e94410 * (assign60490_e94409 * (var_dnm_dn10 / var_dnm))) }, if 0.0 == 0.0 && ((assign60490_e94409) as f64).is_finite() && ((assign60490_e94409) as f64).fract() == 0.0 { if assign60490_e94409 == 0.0 { 0.0 } else { (assign60490_e94409 * ((var_dnm).powf(assign60490_e94409 - 1.0) * var_dnm_dn13)) } } else { (assign60490_e94410 * (assign60490_e94409 * (var_dnm_dn13 / var_dnm))) },)
            }
        };
        (assign60490_e94411, assign60490_e94411_d_n0, assign60490_e94411_d_n2, assign60490_e94411_d_n4, assign60490_e94411_d_n5, assign60490_e94411_d_n6, assign60490_e94411_d_n7, assign60490_e94411_d_n8, assign60490_e94411_d_n9, assign60490_e94411_d_n10, assign60490_e94411_d_n13,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
        var_dnm = assign60490_e94413;
        var_dnm_dn0 = assign60490_e94413_d_n0;
        var_dnm_dn2 = assign60490_e94413_d_n2;
        var_dnm_dn4 = assign60490_e94413_d_n4;
        var_dnm_dn5 = assign60490_e94413_d_n5;
        var_dnm_dn6 = assign60490_e94413_d_n6;
        var_dnm_dn7 = assign60490_e94413_d_n7;
        var_dnm_dn8 = assign60490_e94413_d_n8;
        var_dnm_dn9 = assign60490_e94413_d_n9;
        var_dnm_dn10 = assign60490_e94413_d_n10;
        var_dnm_dn13 = assign60490_e94413_d_n13;
        var_dnm_rv = 0.0;

        let (assign60500_e94429, assign60500_e94429_d_n0, assign60500_e94429_d_n2, assign60500_e94429_d_n4, assign60500_e94429_d_n5, assign60500_e94429_d_n6, assign60500_e94429_d_n7, assign60500_e94429_d_n8, assign60500_e94429_d_n9, assign60500_e94429_d_n10, assign60500_e94429_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 != 0.0)) {
        let assign60500_e94427: f64 = (1.0 / var_dnm);
        (assign60500_e94427, (-(var_dnm_dn0 / (var_dnm * var_dnm))), (-(var_dnm_dn2 / (var_dnm * var_dnm))), (-(var_dnm_dn4 / (var_dnm * var_dnm))), (-(var_dnm_dn5 / (var_dnm * var_dnm))), (-(var_dnm_dn6 / (var_dnm * var_dnm))), (-(var_dnm_dn7 / (var_dnm * var_dnm))), (-(var_dnm_dn8 / (var_dnm * var_dnm))), (-(var_dnm_dn9 / (var_dnm * var_dnm))), (-(var_dnm_dn10 / (var_dnm * var_dnm))), (-(var_dnm_dn13 / (var_dnm * var_dnm))),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
        var_dnm = assign60500_e94429;
        var_dnm_dn0 = assign60500_e94429_d_n0;
        var_dnm_dn2 = assign60500_e94429_d_n2;
        var_dnm_dn4 = assign60500_e94429_d_n4;
        var_dnm_dn5 = assign60500_e94429_d_n5;
        var_dnm_dn6 = assign60500_e94429_d_n6;
        var_dnm_dn7 = assign60500_e94429_d_n7;
        var_dnm_dn8 = assign60500_e94429_d_n8;
        var_dnm_dn9 = assign60500_e94429_d_n9;
        var_dnm_dn10 = assign60500_e94429_d_n10;
        var_dnm_dn13 = assign60500_e94429_d_n13;
        var_dnm_rv = 0.0;

        let (assign60510_e94449, assign60510_e94449_d_n0, assign60510_e94449_d_n2, assign60510_e94449_d_n4, assign60510_e94449_d_n5, assign60510_e94449_d_n6, assign60510_e94449_d_n7, assign60510_e94449_d_n8, assign60510_e94449_d_n9, assign60510_e94449_d_n10, assign60510_e94449_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 != 0.0)) {
        let assign60510_e94444: f64 = (10.0 * 2.220446049250313e-16);
        let assign60510_e94445: f64 = (var_tmf1 * assign60510_e94444);
        let assign60510_e94447: f64 = (assign60510_e94445 * var_dnm);
        (assign60510_e94447, (((var_tmf1_dn0 * assign60510_e94444) * var_dnm) + (assign60510_e94445 * var_dnm_dn0)), (((var_tmf1_dn2 * assign60510_e94444) * var_dnm) + (assign60510_e94445 * var_dnm_dn2)), (((var_tmf1_dn4 * assign60510_e94444) * var_dnm) + (assign60510_e94445 * var_dnm_dn4)), (((var_tmf1_dn5 * assign60510_e94444) * var_dnm) + (assign60510_e94445 * var_dnm_dn5)), (((var_tmf1_dn6 * assign60510_e94444) * var_dnm) + (assign60510_e94445 * var_dnm_dn6)), (((var_tmf1_dn7 * assign60510_e94444) * var_dnm) + (assign60510_e94445 * var_dnm_dn7)), (((var_tmf1_dn8 * assign60510_e94444) * var_dnm) + (assign60510_e94445 * var_dnm_dn8)), (((var_tmf1_dn9 * assign60510_e94444) * var_dnm) + (assign60510_e94445 * var_dnm_dn9)), (((var_tmf1_dn10 * assign60510_e94444) * var_dnm) + (assign60510_e94445 * var_dnm_dn10)), (((var_tmf1_dn13 * assign60510_e94444) * var_dnm) + (assign60510_e94445 * var_dnm_dn13)),)
    } else {
        (var_tmf0, var_tmf0_dn0, var_tmf0_dn2, var_tmf0_dn4, var_tmf0_dn5, var_tmf0_dn6, var_tmf0_dn7, var_tmf0_dn8, var_tmf0_dn9, var_tmf0_dn10, var_tmf0_dn13,)
    }
};
        var_tmf0 = assign60510_e94449;
        var_tmf0_dn0 = assign60510_e94449_d_n0;
        var_tmf0_dn2 = assign60510_e94449_d_n2;
        var_tmf0_dn4 = assign60510_e94449_d_n4;
        var_tmf0_dn5 = assign60510_e94449_d_n5;
        var_tmf0_dn6 = assign60510_e94449_d_n6;
        var_tmf0_dn7 = assign60510_e94449_d_n7;
        var_tmf0_dn8 = assign60510_e94449_d_n8;
        var_tmf0_dn9 = assign60510_e94449_d_n9;
        var_tmf0_dn10 = assign60510_e94449_d_n10;
        var_tmf0_dn13 = assign60510_e94449_d_n13;
        var_tmf0_rv = 0.0;

        let (assign60520_e94471, assign60520_e94471_d_n0, assign60520_e94471_d_n2, assign60520_e94471_d_n4, assign60520_e94471_d_n5, assign60520_e94471_d_n6, assign60520_e94471_d_n7, assign60520_e94471_d_n8, assign60520_e94471_d_n9, assign60520_e94471_d_n10, assign60520_e94471_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 != 0.0)) {
        let assign60520_e94463: f64 = (10.0 * 2.220446049250313e-16);
        let assign60520_e94465: f64 = (assign60520_e94463 * var_xmp);
        let assign60520_e94467: f64 = (assign60520_e94465 * var_dnm);
        let assign60520_e94469: f64 = (assign60520_e94467 / var_arg);
        (assign60520_e94469, ((((((assign60520_e94463 * var_xmp_dn0) * var_dnm) + (assign60520_e94465 * var_dnm_dn0)) * var_arg) - (assign60520_e94467 * var_arg_dn0)) / (var_arg * var_arg)), ((((((assign60520_e94463 * var_xmp_dn2) * var_dnm) + (assign60520_e94465 * var_dnm_dn2)) * var_arg) - (assign60520_e94467 * var_arg_dn2)) / (var_arg * var_arg)), ((((((assign60520_e94463 * var_xmp_dn4) * var_dnm) + (assign60520_e94465 * var_dnm_dn4)) * var_arg) - (assign60520_e94467 * var_arg_dn4)) / (var_arg * var_arg)), ((((((assign60520_e94463 * var_xmp_dn5) * var_dnm) + (assign60520_e94465 * var_dnm_dn5)) * var_arg) - (assign60520_e94467 * var_arg_dn5)) / (var_arg * var_arg)), ((((((assign60520_e94463 * var_xmp_dn6) * var_dnm) + (assign60520_e94465 * var_dnm_dn6)) * var_arg) - (assign60520_e94467 * var_arg_dn6)) / (var_arg * var_arg)), ((((((assign60520_e94463 * var_xmp_dn7) * var_dnm) + (assign60520_e94465 * var_dnm_dn7)) * var_arg) - (assign60520_e94467 * var_arg_dn7)) / (var_arg * var_arg)), ((((((assign60520_e94463 * var_xmp_dn8) * var_dnm) + (assign60520_e94465 * var_dnm_dn8)) * var_arg) - (assign60520_e94467 * var_arg_dn8)) / (var_arg * var_arg)), ((((((assign60520_e94463 * var_xmp_dn9) * var_dnm) + (assign60520_e94465 * var_dnm_dn9)) * var_arg) - (assign60520_e94467 * var_arg_dn9)) / (var_arg * var_arg)), ((((((assign60520_e94463 * var_xmp_dn10) * var_dnm) + (assign60520_e94465 * var_dnm_dn10)) * var_arg) - (assign60520_e94467 * var_arg_dn10)) / (var_arg * var_arg)), ((((((assign60520_e94463 * var_xmp_dn13) * var_dnm) + (assign60520_e94465 * var_dnm_dn13)) * var_arg) - (assign60520_e94467 * var_arg_dn13)) / (var_arg * var_arg)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign60520_e94471;
        var_t0_dn0 = assign60520_e94471_d_n0;
        var_t0_dn2 = assign60520_e94471_d_n2;
        var_t0_dn4 = assign60520_e94471_d_n4;
        var_t0_dn5 = assign60520_e94471_d_n5;
        var_t0_dn6 = assign60520_e94471_d_n6;
        var_t0_dn7 = assign60520_e94471_d_n7;
        var_t0_dn8 = assign60520_e94471_d_n8;
        var_t0_dn9 = assign60520_e94471_d_n9;
        var_t0_dn10 = assign60520_e94471_d_n10;
        var_t0_dn13 = assign60520_e94471_d_n13;
        var_t0_rv = 0.0;

        let (assign60530_e94497, assign60530_e94497_d_n0, assign60530_e94497_d_n2, assign60530_e94497_d_n4, assign60530_e94497_d_n5, assign60530_e94497_d_n6, assign60530_e94497_d_n7, assign60530_e94497_d_n8, assign60530_e94497_d_n9, assign60530_e94497_d_n10, assign60530_e94497_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 != 0.0)) {
        let assign60530_e94485: f64 = (var_ps0 + var_vds);
        let assign60530_e94488: f64 = (10.0 * 2.220446049250313e-16);
        let assign60530_e94489: f64 = (assign60530_e94485 - assign60530_e94488);
        let assign60530_e94492: f64 = (10.0 * 2.220446049250313e-16);
        let assign60530_e94493: f64 = (assign60530_e94489 - assign60530_e94492);
        let assign60530_e94495: f64 = (assign60530_e94493 + var_tmf0);
        (assign60530_e94495, ((var_ps0_dn0 + var_vds_dn0) + var_tmf0_dn0), ((var_ps0_dn2 + var_vds_dn2) + var_tmf0_dn2), ((var_ps0_dn4 + var_vds_dn4) + var_tmf0_dn4), ((var_ps0_dn5 + var_vds_dn5) + var_tmf0_dn5), ((var_ps0_dn6 + var_vds_dn6) + var_tmf0_dn6), ((var_ps0_dn7 + var_vds_dn7) + var_tmf0_dn7), ((var_ps0_dn8 + var_vds_dn8) + var_tmf0_dn8), ((var_ps0_dn9 + var_vds_dn9) + var_tmf0_dn9), ((var_ps0_dn10 + var_vds_dn10) + var_tmf0_dn10), ((var_ps0_dn13 + var_vds_dn13) + var_tmf0_dn13),)
    } else {
        (var_psdl, var_psdl_dn0, var_psdl_dn2, var_psdl_dn4, var_psdl_dn5, var_psdl_dn6, var_psdl_dn7, var_psdl_dn8, var_psdl_dn9, var_psdl_dn10, var_psdl_dn13,)
    }
};
        var_psdl = assign60530_e94497;
        var_psdl_dn0 = assign60530_e94497_d_n0;
        var_psdl_dn2 = assign60530_e94497_d_n2;
        var_psdl_dn4 = assign60530_e94497_d_n4;
        var_psdl_dn5 = assign60530_e94497_d_n5;
        var_psdl_dn6 = assign60530_e94497_d_n6;
        var_psdl_dn7 = assign60530_e94497_d_n7;
        var_psdl_dn8 = assign60530_e94497_d_n8;
        var_psdl_dn9 = assign60530_e94497_d_n9;
        var_psdl_dn10 = assign60530_e94497_d_n10;
        var_psdl_dn13 = assign60530_e94497_d_n13;
        var_psdl_rv = 0.0;

        let (assign60540_e94511, assign60540_e94511_d_n0, assign60540_e94511_d_n2, assign60540_e94511_d_n4, assign60540_e94511_d_n5, assign60540_e94511_d_n6, assign60540_e94511_d_n7, assign60540_e94511_d_n8, assign60540_e94511_d_n9, assign60540_e94511_d_n10, assign60540_e94511_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 != 0.0)) {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign60540_e94511;
        var_t0_dn0 = assign60540_e94511_d_n0;
        var_t0_dn2 = assign60540_e94511_d_n2;
        var_t0_dn4 = assign60540_e94511_d_n4;
        var_t0_dn5 = assign60540_e94511_d_n5;
        var_t0_dn6 = assign60540_e94511_d_n6;
        var_t0_dn7 = assign60540_e94511_d_n7;
        var_t0_dn8 = assign60540_e94511_d_n8;
        var_t0_dn9 = assign60540_e94511_d_n9;
        var_t0_dn10 = assign60540_e94511_d_n10;
        var_t0_dn13 = assign60540_e94511_d_n13;
        var_t0_rv = 0.0;

        let (assign60550_e94526, assign60550_e94526_d_n0, assign60550_e94526_d_n2, assign60550_e94526_d_n4, assign60550_e94526_d_n5, assign60550_e94526_d_n6, assign60550_e94526_d_n7, assign60550_e94526_d_n8, assign60550_e94526_d_n9, assign60550_e94526_d_n10, assign60550_e94526_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 == 0.0)) {
        (var_psdl, var_psdl_dn0, var_psdl_dn2, var_psdl_dn4, var_psdl_dn5, var_psdl_dn6, var_psdl_dn7, var_psdl_dn8, var_psdl_dn9, var_psdl_dn10, var_psdl_dn13,)
    } else {
        (var_psdl, var_psdl_dn0, var_psdl_dn2, var_psdl_dn4, var_psdl_dn5, var_psdl_dn6, var_psdl_dn7, var_psdl_dn8, var_psdl_dn9, var_psdl_dn10, var_psdl_dn13,)
    }
};
        var_psdl = assign60550_e94526;
        var_psdl_dn0 = assign60550_e94526_d_n0;
        var_psdl_dn2 = assign60550_e94526_d_n2;
        var_psdl_dn4 = assign60550_e94526_d_n4;
        var_psdl_dn5 = assign60550_e94526_d_n5;
        var_psdl_dn6 = assign60550_e94526_d_n6;
        var_psdl_dn7 = assign60550_e94526_d_n7;
        var_psdl_dn8 = assign60550_e94526_d_n8;
        var_psdl_dn9 = assign60550_e94526_d_n9;
        var_psdl_dn10 = assign60550_e94526_d_n10;
        var_psdl_dn13 = assign60550_e94526_d_n13;
        var_psdl_rv = 0.0;

        let (assign60560_e94541, assign60560_e94541_d_n0, assign60560_e94541_d_n2, assign60560_e94541_d_n4, assign60560_e94541_d_n5, assign60560_e94541_d_n6, assign60560_e94541_d_n7, assign60560_e94541_d_n8, assign60560_e94541_d_n9, assign60560_e94541_d_n10, assign60560_e94541_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) && (var_guard1469 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign60560_e94541;
        var_t0_dn0 = assign60560_e94541_d_n0;
        var_t0_dn2 = assign60560_e94541_d_n2;
        var_t0_dn4 = assign60560_e94541_d_n4;
        var_t0_dn5 = assign60560_e94541_d_n5;
        var_t0_dn6 = assign60560_e94541_d_n6;
        var_t0_dn7 = assign60560_e94541_d_n7;
        var_t0_dn8 = assign60560_e94541_d_n8;
        var_t0_dn9 = assign60560_e94541_d_n9;
        var_t0_dn10 = assign60560_e94541_d_n10;
        var_t0_dn13 = assign60560_e94541_d_n13;
        var_t0_rv = 0.0;

        let (assign60570_e94555, assign60570_e94555_d_n0, assign60570_e94555_d_n2, assign60570_e94555_d_n4, assign60570_e94555_d_n5, assign60570_e94555_d_n6, assign60570_e94555_d_n7, assign60570_e94555_d_n8, assign60570_e94555_d_n9, assign60570_e94555_d_n10, assign60570_e94555_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let assign60570_e94553: f64 = (var_psdl - var_psl);
        (assign60570_e94553, (var_psdl_dn0 - var_psl_dn0), (var_psdl_dn2 - var_psl_dn2), (var_psdl_dn4 - var_psl_dn4), (var_psdl_dn5 - var_psl_dn5), (var_psdl_dn6 - var_psl_dn6), (var_psdl_dn7 - var_psl_dn7), (var_psdl_dn8 - var_psl_dn8), (var_psdl_dn9 - var_psl_dn9), (var_psdl_dn10 - var_psl_dn10), (var_psdl_dn13 - var_psl_dn13),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn13,)
    }
};
        var_t6 = assign60570_e94555;
        var_t6_dn0 = assign60570_e94555_d_n0;
        var_t6_dn2 = assign60570_e94555_d_n2;
        var_t6_dn4 = assign60570_e94555_d_n4;
        var_t6_dn5 = assign60570_e94555_d_n5;
        var_t6_dn6 = assign60570_e94555_d_n6;
        var_t6_dn7 = assign60570_e94555_d_n7;
        var_t6_dn8 = assign60570_e94555_d_n8;
        var_t6_dn9 = assign60570_e94555_d_n9;
        var_t6_dn10 = assign60570_e94555_d_n10;
        var_t6_dn13 = assign60570_e94555_d_n13;
        var_t6_rv = 0.0;

        let (assign60580_e94569, assign60580_e94569_d_n0, assign60580_e94569_d_n2, assign60580_e94569_d_n4, assign60580_e94569_d_n5, assign60580_e94569_d_n6, assign60580_e94569_d_n7, assign60580_e94569_d_n8, assign60580_e94569_d_n9, assign60580_e94569_d_n10, assign60580_e94569_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let assign60580_e94567: f64 = (var_beta * var_qn0);
        (assign60580_e94567, ((var_beta_dn0 * var_qn0) + (var_beta * var_qn0_dn0)), ((var_beta_dn2 * var_qn0) + (var_beta * var_qn0_dn2)), ((var_beta_dn4 * var_qn0) + (var_beta * var_qn0_dn4)), ((var_beta_dn5 * var_qn0) + (var_beta * var_qn0_dn5)), ((var_beta_dn6 * var_qn0) + (var_beta * var_qn0_dn6)), ((var_beta_dn7 * var_qn0) + (var_beta * var_qn0_dn7)), ((var_beta_dn8 * var_qn0) + (var_beta * var_qn0_dn8)), ((var_beta_dn9 * var_qn0) + (var_beta * var_qn0_dn9)), ((var_beta_dn10 * var_qn0) + (var_beta * var_qn0_dn10)), ((var_beta_dn13 * var_qn0) + (var_beta * var_qn0_dn13)),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn13,)
    }
};
        var_t3 = assign60580_e94569;
        var_t3_dn0 = assign60580_e94569_d_n0;
        var_t3_dn2 = assign60580_e94569_d_n2;
        var_t3_dn4 = assign60580_e94569_d_n4;
        var_t3_dn5 = assign60580_e94569_d_n5;
        var_t3_dn6 = assign60580_e94569_d_n6;
        var_t3_dn7 = assign60580_e94569_d_n7;
        var_t3_dn8 = assign60580_e94569_d_n8;
        var_t3_dn9 = assign60580_e94569_d_n9;
        var_t3_dn10 = assign60580_e94569_d_n10;
        var_t3_dn13 = assign60580_e94569_d_n13;
        var_t3_rv = 0.0;

        let (assign60590_e94583, assign60590_e94583_d_n0, assign60590_e94583_d_n2, assign60590_e94583_d_n4, assign60590_e94583_d_n5, assign60590_e94583_d_n6, assign60590_e94583_d_n7, assign60590_e94583_d_n8, assign60590_e94583_d_n9, assign60590_e94583_d_n10, assign60590_e94583_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let assign60590_e94581: f64 = (1.0 / var_t3);
        (assign60590_e94581, (-(var_t3_dn0 / (var_t3 * var_t3))), (-(var_t3_dn2 / (var_t3 * var_t3))), (-(var_t3_dn4 / (var_t3 * var_t3))), (-(var_t3_dn5 / (var_t3 * var_t3))), (-(var_t3_dn6 / (var_t3 * var_t3))), (-(var_t3_dn7 / (var_t3 * var_t3))), (-(var_t3_dn8 / (var_t3 * var_t3))), (-(var_t3_dn9 / (var_t3 * var_t3))), (-(var_t3_dn10 / (var_t3 * var_t3))), (-(var_t3_dn13 / (var_t3 * var_t3))),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign60590_e94583;
        var_t1_dn0 = assign60590_e94583_d_n0;
        var_t1_dn2 = assign60590_e94583_d_n2;
        var_t1_dn4 = assign60590_e94583_d_n4;
        var_t1_dn5 = assign60590_e94583_d_n5;
        var_t1_dn6 = assign60590_e94583_d_n6;
        var_t1_dn7 = assign60590_e94583_d_n7;
        var_t1_dn8 = assign60590_e94583_d_n8;
        var_t1_dn9 = assign60590_e94583_d_n9;
        var_t1_dn10 = assign60590_e94583_d_n10;
        var_t1_dn13 = assign60590_e94583_d_n13;
        var_t1_rv = 0.0;

        let (assign60600_e94603, assign60600_e94603_d_n0, assign60600_e94603_d_n2, assign60600_e94603_d_n4, assign60600_e94603_d_n5, assign60600_e94603_d_n6, assign60600_e94603_d_n7, assign60600_e94603_d_n8, assign60600_e94603_d_n9, assign60600_e94603_d_n10, assign60600_e94603_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let assign60600_e94596: f64 = (10.0 * 2.220446049250313e-16);
        let assign60600_e94597: f64 = (var_pds + assign60600_e94596);
        let assign60600_e94599: f64 = (assign60600_e94597 * var_fdd);
        let assign60600_e94601: f64 = (assign60600_e94599 * var_t1);
        (assign60600_e94601, ((((var_pds_dn0 * var_fdd) + (assign60600_e94597 * var_fdd_dn0)) * var_t1) + (assign60600_e94599 * var_t1_dn0)), ((((var_pds_dn2 * var_fdd) + (assign60600_e94597 * var_fdd_dn2)) * var_t1) + (assign60600_e94599 * var_t1_dn2)), ((((var_pds_dn4 * var_fdd) + (assign60600_e94597 * var_fdd_dn4)) * var_t1) + (assign60600_e94599 * var_t1_dn4)), ((((var_pds_dn5 * var_fdd) + (assign60600_e94597 * var_fdd_dn5)) * var_t1) + (assign60600_e94599 * var_t1_dn5)), ((((var_pds_dn6 * var_fdd) + (assign60600_e94597 * var_fdd_dn6)) * var_t1) + (assign60600_e94599 * var_t1_dn6)), ((((var_pds_dn7 * var_fdd) + (assign60600_e94597 * var_fdd_dn7)) * var_t1) + (assign60600_e94599 * var_t1_dn7)), ((((var_pds_dn8 * var_fdd) + (assign60600_e94597 * var_fdd_dn8)) * var_t1) + (assign60600_e94599 * var_t1_dn8)), ((((var_pds_dn9 * var_fdd) + (assign60600_e94597 * var_fdd_dn9)) * var_t1) + (assign60600_e94599 * var_t1_dn9)), ((((var_pds_dn10 * var_fdd) + (assign60600_e94597 * var_fdd_dn10)) * var_t1) + (assign60600_e94599 * var_t1_dn10)), ((((var_pds_dn13 * var_fdd) + (assign60600_e94597 * var_fdd_dn13)) * var_t1) + (assign60600_e94599 * var_t1_dn13)),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn13,)
    }
};
        var_t5 = assign60600_e94603;
        var_t5_dn0 = assign60600_e94603_d_n0;
        var_t5_dn2 = assign60600_e94603_d_n2;
        var_t5_dn4 = assign60600_e94603_d_n4;
        var_t5_dn5 = assign60600_e94603_d_n5;
        var_t5_dn6 = assign60600_e94603_d_n6;
        var_t5_dn7 = assign60600_e94603_d_n7;
        var_t5_dn8 = assign60600_e94603_d_n8;
        var_t5_dn9 = assign60600_e94603_d_n9;
        var_t5_dn10 = assign60600_e94603_d_n10;
        var_t5_dn13 = assign60600_e94603_d_n13;
        var_t5_rv = 0.0;

        let (assign60610_e94617, assign60610_e94617_d_n0, assign60610_e94617_d_n2, assign60610_e94617_d_n4, assign60610_e94617_d_n5, assign60610_e94617_d_n6, assign60610_e94617_d_n7, assign60610_e94617_d_n8, assign60610_e94617_d_n9, assign60610_e94617_d_n10, assign60610_e94617_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let assign60610_e94615: f64 = (var_t5 * var_beta);
        (assign60610_e94615, ((var_t5_dn0 * var_beta) + (var_t5 * var_beta_dn0)), ((var_t5_dn2 * var_beta) + (var_t5 * var_beta_dn2)), ((var_t5_dn4 * var_beta) + (var_t5 * var_beta_dn4)), ((var_t5_dn5 * var_beta) + (var_t5 * var_beta_dn5)), ((var_t5_dn6 * var_beta) + (var_t5 * var_beta_dn6)), ((var_t5_dn7 * var_beta) + (var_t5 * var_beta_dn7)), ((var_t5_dn8 * var_beta) + (var_t5 * var_beta_dn8)), ((var_t5_dn9 * var_beta) + (var_t5 * var_beta_dn9)), ((var_t5_dn10 * var_beta) + (var_t5 * var_beta_dn10)), ((var_t5_dn13 * var_beta) + (var_t5 * var_beta_dn13)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign60610_e94617;
        var_t2_dn0 = assign60610_e94617_d_n0;
        var_t2_dn2 = assign60610_e94617_d_n2;
        var_t2_dn4 = assign60610_e94617_d_n4;
        var_t2_dn5 = assign60610_e94617_d_n5;
        var_t2_dn6 = assign60610_e94617_d_n6;
        var_t2_dn7 = assign60610_e94617_d_n7;
        var_t2_dn8 = assign60610_e94617_d_n8;
        var_t2_dn9 = assign60610_e94617_d_n9;
        var_t2_dn10 = assign60610_e94617_d_n10;
        var_t2_dn13 = assign60610_e94617_d_n13;
        var_t2_rv = 0.0;

        let (assign60620_e94631, assign60620_e94631_d_n0, assign60620_e94631_d_n2, assign60620_e94631_d_n4, assign60620_e94631_d_n5, assign60620_e94631_d_n6, assign60620_e94631_d_n7, assign60620_e94631_d_n8, assign60620_e94631_d_n9, assign60620_e94631_d_n10, assign60620_e94631_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let assign60620_e94629: f64 = (var_q_nsub / 1.034943e-10);
        (assign60620_e94629, (var_q_nsub_dn0 / 1.034943e-10), (var_q_nsub_dn2 / 1.034943e-10), (var_q_nsub_dn4 / 1.034943e-10), (var_q_nsub_dn5 / 1.034943e-10), (var_q_nsub_dn6 / 1.034943e-10), (var_q_nsub_dn7 / 1.034943e-10), (var_q_nsub_dn8 / 1.034943e-10), (var_q_nsub_dn9 / 1.034943e-10), (var_q_nsub_dn10 / 1.034943e-10), (var_q_nsub_dn13 / 1.034943e-10),)
    } else {
        (var_t10, var_t10_dn0, var_t10_dn2, var_t10_dn4, var_t10_dn5, var_t10_dn6, var_t10_dn7, var_t10_dn8, var_t10_dn9, var_t10_dn10, var_t10_dn13,)
    }
};
        var_t10 = assign60620_e94631;
        var_t10_dn0 = assign60620_e94631_d_n0;
        var_t10_dn2 = assign60620_e94631_d_n2;
        var_t10_dn4 = assign60620_e94631_d_n4;
        var_t10_dn5 = assign60620_e94631_d_n5;
        var_t10_dn6 = assign60620_e94631_d_n6;
        var_t10_dn7 = assign60620_e94631_d_n7;
        var_t10_dn8 = assign60620_e94631_d_n8;
        var_t10_dn9 = assign60620_e94631_d_n9;
        var_t10_dn10 = assign60620_e94631_d_n10;
        var_t10_dn13 = assign60620_e94631_d_n13;
        var_t10_rv = 0.0;

        let (assign60630_e94643, assign60630_e94643_d_n0, assign60630_e94643_d_n2, assign60630_e94643_d_n4, assign60630_e94643_d_n5, assign60630_e94643_d_n6, assign60630_e94643_d_n7, assign60630_e94643_d_n8, assign60630_e94643_d_n9, assign60630_e94643_d_n10, assign60630_e94643_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        (100000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign60630_e94643;
        var_t1_dn0 = assign60630_e94643_d_n0;
        var_t1_dn2 = assign60630_e94643_d_n2;
        var_t1_dn4 = assign60630_e94643_d_n4;
        var_t1_dn5 = assign60630_e94643_d_n5;
        var_t1_dn6 = assign60630_e94643_d_n6;
        var_t1_dn7 = assign60630_e94643_d_n7;
        var_t1_dn8 = assign60630_e94643_d_n8;
        var_t1_dn9 = assign60630_e94643_d_n9;
        var_t1_dn10 = assign60630_e94643_d_n10;
        var_t1_dn13 = assign60630_e94643_d_n13;
        var_t1_rv = 0.0;

        let (assign60640_e94657, assign60640_e94657_d_n0, assign60640_e94657_d_n2, assign60640_e94657_d_n4, assign60640_e94657_d_n5, assign60640_e94657_d_n6, assign60640_e94657_d_n7, assign60640_e94657_d_n8, assign60640_e94657_d_n9, assign60640_e94657_d_n10, assign60640_e94657_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let assign60640_e94655: f64 = (1.0 / var_leff);
        (assign60640_e94655, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign60640_e94657;
        var_t2_dn0 = assign60640_e94657_d_n0;
        var_t2_dn2 = assign60640_e94657_d_n2;
        var_t2_dn4 = assign60640_e94657_d_n4;
        var_t2_dn5 = assign60640_e94657_d_n5;
        var_t2_dn6 = assign60640_e94657_d_n6;
        var_t2_dn7 = assign60640_e94657_d_n7;
        var_t2_dn8 = assign60640_e94657_d_n8;
        var_t2_dn9 = assign60640_e94657_d_n9;
        var_t2_dn10 = assign60640_e94657_d_n10;
        var_t2_dn13 = assign60640_e94657_d_n13;
        var_t2_rv = 0.0;

        let (assign60650_e94685, assign60650_e94685_d_n0, assign60650_e94685_d_n2, assign60650_e94685_d_n4, assign60650_e94685_d_n5, assign60650_e94685_d_n6, assign60650_e94685_d_n7, assign60650_e94685_d_n8, assign60650_e94685_d_n9, assign60650_e94685_d_n10, assign60650_e94685_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let assign60650_e94669: f64 = (2.0 * var_t5);
        let assign60650_e94672: f64 = (2.0 * var_t10);
        let assign60650_e94674: f64 = (assign60650_e94672 * var_t6);
        let assign60650_e94676: f64 = (assign60650_e94674 * var_t4);
        let assign60650_e94677: f64 = (assign60650_e94669 + assign60650_e94676);
        let assign60650_e94680: f64 = (var_t1 * var_t4);
        let assign60650_e94681: f64 = (assign60650_e94677 + assign60650_e94680);
        let assign60650_e94683: f64 = (assign60650_e94681 * var_t2);
        (assign60650_e94683, (((((2.0 * var_t5_dn0) + (((((2.0 * var_t10_dn0) * var_t6) + (assign60650_e94672 * var_t6_dn0)) * var_t4) + (assign60650_e94674 * var_t4_dn0))) + ((var_t1_dn0 * var_t4) + (var_t1 * var_t4_dn0))) * var_t2) + (assign60650_e94681 * var_t2_dn0)), (((((2.0 * var_t5_dn2) + (((((2.0 * var_t10_dn2) * var_t6) + (assign60650_e94672 * var_t6_dn2)) * var_t4) + (assign60650_e94674 * var_t4_dn2))) + ((var_t1_dn2 * var_t4) + (var_t1 * var_t4_dn2))) * var_t2) + (assign60650_e94681 * var_t2_dn2)), (((((2.0 * var_t5_dn4) + (((((2.0 * var_t10_dn4) * var_t6) + (assign60650_e94672 * var_t6_dn4)) * var_t4) + (assign60650_e94674 * var_t4_dn4))) + ((var_t1_dn4 * var_t4) + (var_t1 * var_t4_dn4))) * var_t2) + (assign60650_e94681 * var_t2_dn4)), (((((2.0 * var_t5_dn5) + (((((2.0 * var_t10_dn5) * var_t6) + (assign60650_e94672 * var_t6_dn5)) * var_t4) + (assign60650_e94674 * var_t4_dn5))) + ((var_t1_dn5 * var_t4) + (var_t1 * var_t4_dn5))) * var_t2) + (assign60650_e94681 * var_t2_dn5)), (((((2.0 * var_t5_dn6) + (((((2.0 * var_t10_dn6) * var_t6) + (assign60650_e94672 * var_t6_dn6)) * var_t4) + (assign60650_e94674 * var_t4_dn6))) + ((var_t1_dn6 * var_t4) + (var_t1 * var_t4_dn6))) * var_t2) + (assign60650_e94681 * var_t2_dn6)), (((((2.0 * var_t5_dn7) + (((((2.0 * var_t10_dn7) * var_t6) + (assign60650_e94672 * var_t6_dn7)) * var_t4) + (assign60650_e94674 * var_t4_dn7))) + ((var_t1_dn7 * var_t4) + (var_t1 * var_t4_dn7))) * var_t2) + (assign60650_e94681 * var_t2_dn7)), (((((2.0 * var_t5_dn8) + (((((2.0 * var_t10_dn8) * var_t6) + (assign60650_e94672 * var_t6_dn8)) * var_t4) + (assign60650_e94674 * var_t4_dn8))) + ((var_t1_dn8 * var_t4) + (var_t1 * var_t4_dn8))) * var_t2) + (assign60650_e94681 * var_t2_dn8)), (((((2.0 * var_t5_dn9) + (((((2.0 * var_t10_dn9) * var_t6) + (assign60650_e94672 * var_t6_dn9)) * var_t4) + (assign60650_e94674 * var_t4_dn9))) + ((var_t1_dn9 * var_t4) + (var_t1 * var_t4_dn9))) * var_t2) + (assign60650_e94681 * var_t2_dn9)), (((((2.0 * var_t5_dn10) + (((((2.0 * var_t10_dn10) * var_t6) + (assign60650_e94672 * var_t6_dn10)) * var_t4) + (assign60650_e94674 * var_t4_dn10))) + ((var_t1_dn10 * var_t4) + (var_t1 * var_t4_dn10))) * var_t2) + (assign60650_e94681 * var_t2_dn10)), (((((2.0 * var_t5_dn13) + (((((2.0 * var_t10_dn13) * var_t6) + (assign60650_e94672 * var_t6_dn13)) * var_t4) + (assign60650_e94674 * var_t4_dn13))) + ((var_t1_dn13 * var_t4) + (var_t1 * var_t4_dn13))) * var_t2) + (assign60650_e94681 * var_t2_dn13)),)
    } else {
        (var_t11, var_t11_dn0, var_t11_dn2, var_t11_dn4, var_t11_dn5, var_t11_dn6, var_t11_dn7, var_t11_dn8, var_t11_dn9, var_t11_dn10, var_t11_dn13,)
    }
};
        var_t11 = assign60650_e94685;
        var_t11_dn0 = assign60650_e94685_d_n0;
        var_t11_dn2 = assign60650_e94685_d_n2;
        var_t11_dn4 = assign60650_e94685_d_n4;
        var_t11_dn5 = assign60650_e94685_d_n5;
        var_t11_dn6 = assign60650_e94685_d_n6;
        var_t11_dn7 = assign60650_e94685_d_n7;
        var_t11_dn8 = assign60650_e94685_d_n8;
        var_t11_dn9 = assign60650_e94685_d_n9;
        var_t11_dn10 = assign60650_e94685_d_n10;
        var_t11_dn13 = assign60650_e94685_d_n13;
        var_t11_rv = 0.0;

        let (assign60660_e94699, assign60660_e94699_d_n0, assign60660_e94699_d_n2, assign60660_e94699_d_n4, assign60660_e94699_d_n5, assign60660_e94699_d_n6, assign60660_e94699_d_n7, assign60660_e94699_d_n8, assign60660_e94699_d_n9, assign60660_e94699_d_n10, assign60660_e94699_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let assign60660_e94697: f64 = (var_t2 * var_t4);
        (assign60660_e94697, ((var_t2_dn0 * var_t4) + (var_t2 * var_t4_dn0)), ((var_t2_dn2 * var_t4) + (var_t2 * var_t4_dn2)), ((var_t2_dn4 * var_t4) + (var_t2 * var_t4_dn4)), ((var_t2_dn5 * var_t4) + (var_t2 * var_t4_dn5)), ((var_t2_dn6 * var_t4) + (var_t2 * var_t4_dn6)), ((var_t2_dn7 * var_t4) + (var_t2 * var_t4_dn7)), ((var_t2_dn8 * var_t4) + (var_t2 * var_t4_dn8)), ((var_t2_dn9 * var_t4) + (var_t2 * var_t4_dn9)), ((var_t2_dn10 * var_t4) + (var_t2 * var_t4_dn10)), ((var_t2_dn13 * var_t4) + (var_t2 * var_t4_dn13)),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn13,)
    }
};
        var_t3 = assign60660_e94699;
        var_t3_dn0 = assign60660_e94699_d_n0;
        var_t3_dn2 = assign60660_e94699_d_n2;
        var_t3_dn4 = assign60660_e94699_d_n4;
        var_t3_dn5 = assign60660_e94699_d_n5;
        var_t3_dn6 = assign60660_e94699_d_n6;
        var_t3_dn7 = assign60660_e94699_d_n7;
        var_t3_dn8 = assign60660_e94699_d_n8;
        var_t3_dn9 = assign60660_e94699_d_n9;
        var_t3_dn10 = assign60660_e94699_d_n10;
        var_t3_dn13 = assign60660_e94699_d_n13;
        var_t3_rv = 0.0;

        let (assign60670_e94713, assign60670_e94713_d_n0, assign60670_e94713_d_n2, assign60670_e94713_d_n4, assign60670_e94713_d_n5, assign60670_e94713_d_n6, assign60670_e94713_d_n7, assign60670_e94713_d_n8, assign60670_e94713_d_n9, assign60670_e94713_d_n10, assign60670_e94713_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let assign60670_e94711: f64 = (var_t11 * var_t4);
        (assign60670_e94711, ((var_t11_dn0 * var_t4) + (var_t11 * var_t4_dn0)), ((var_t11_dn2 * var_t4) + (var_t11 * var_t4_dn2)), ((var_t11_dn4 * var_t4) + (var_t11 * var_t4_dn4)), ((var_t11_dn5 * var_t4) + (var_t11 * var_t4_dn5)), ((var_t11_dn6 * var_t4) + (var_t11 * var_t4_dn6)), ((var_t11_dn7 * var_t4) + (var_t11 * var_t4_dn7)), ((var_t11_dn8 * var_t4) + (var_t11 * var_t4_dn8)), ((var_t11_dn9 * var_t4) + (var_t11 * var_t4_dn9)), ((var_t11_dn10 * var_t4) + (var_t11 * var_t4_dn10)), ((var_t11_dn13 * var_t4) + (var_t11 * var_t4_dn13)),)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn7, var_t7_dn8, var_t7_dn9, var_t7_dn10, var_t7_dn13,)
    }
};
        var_t7 = assign60670_e94713;
        var_t7_dn0 = assign60670_e94713_d_n0;
        var_t7_dn2 = assign60670_e94713_d_n2;
        var_t7_dn4 = assign60670_e94713_d_n4;
        var_t7_dn5 = assign60670_e94713_d_n5;
        var_t7_dn6 = assign60670_e94713_d_n6;
        var_t7_dn7 = assign60670_e94713_d_n7;
        var_t7_dn8 = assign60670_e94713_d_n8;
        var_t7_dn9 = assign60670_e94713_d_n9;
        var_t7_dn10 = assign60670_e94713_d_n10;
        var_t7_dn13 = assign60670_e94713_d_n13;
        var_t7_rv = 0.0;

        let (assign60680_e94733, assign60680_e94733_d_n0, assign60680_e94733_d_n2, assign60680_e94733_d_n4, assign60680_e94733_d_n5, assign60680_e94733_d_n6, assign60680_e94733_d_n7, assign60680_e94733_d_n8, assign60680_e94733_d_n9, assign60680_e94733_d_n10, assign60680_e94733_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let assign60680_e94726: f64 = (2.0 * var_t10);
        let assign60680_e94728: f64 = (assign60680_e94726 * var_t6);
        let assign60680_e94730: f64 = (assign60680_e94728 + var_t1);
        let assign60680_e94731: f64 = (4.0 * assign60680_e94730);
        (assign60680_e94731, (4.0 * ((((2.0 * var_t10_dn0) * var_t6) + (assign60680_e94726 * var_t6_dn0)) + var_t1_dn0)), (4.0 * ((((2.0 * var_t10_dn2) * var_t6) + (assign60680_e94726 * var_t6_dn2)) + var_t1_dn2)), (4.0 * ((((2.0 * var_t10_dn4) * var_t6) + (assign60680_e94726 * var_t6_dn4)) + var_t1_dn4)), (4.0 * ((((2.0 * var_t10_dn5) * var_t6) + (assign60680_e94726 * var_t6_dn5)) + var_t1_dn5)), (4.0 * ((((2.0 * var_t10_dn6) * var_t6) + (assign60680_e94726 * var_t6_dn6)) + var_t1_dn6)), (4.0 * ((((2.0 * var_t10_dn7) * var_t6) + (assign60680_e94726 * var_t6_dn7)) + var_t1_dn7)), (4.0 * ((((2.0 * var_t10_dn8) * var_t6) + (assign60680_e94726 * var_t6_dn8)) + var_t1_dn8)), (4.0 * ((((2.0 * var_t10_dn9) * var_t6) + (assign60680_e94726 * var_t6_dn9)) + var_t1_dn9)), (4.0 * ((((2.0 * var_t10_dn10) * var_t6) + (assign60680_e94726 * var_t6_dn10)) + var_t1_dn10)), (4.0 * ((((2.0 * var_t10_dn13) * var_t6) + (assign60680_e94726 * var_t6_dn13)) + var_t1_dn13)),)
    } else {
        (var_t11, var_t11_dn0, var_t11_dn2, var_t11_dn4, var_t11_dn5, var_t11_dn6, var_t11_dn7, var_t11_dn8, var_t11_dn9, var_t11_dn10, var_t11_dn13,)
    }
};
        var_t11 = assign60680_e94733;
        var_t11_dn0 = assign60680_e94733_d_n0;
        var_t11_dn2 = assign60680_e94733_d_n2;
        var_t11_dn4 = assign60680_e94733_d_n4;
        var_t11_dn5 = assign60680_e94733_d_n5;
        var_t11_dn6 = assign60680_e94733_d_n6;
        var_t11_dn7 = assign60680_e94733_d_n7;
        var_t11_dn8 = assign60680_e94733_d_n8;
        var_t11_dn9 = assign60680_e94733_d_n9;
        var_t11_dn10 = assign60680_e94733_d_n10;
        var_t11_dn13 = assign60680_e94733_d_n13;
        var_t11_rv = 0.0;

        let (assign60690_e94751, assign60690_e94751_d_n0, assign60690_e94751_d_n2, assign60690_e94751_d_n4, assign60690_e94751_d_n5, assign60690_e94751_d_n6, assign60690_e94751_d_n7, assign60690_e94751_d_n8, assign60690_e94751_d_n9, assign60690_e94751_d_n10, assign60690_e94751_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let assign60690_e94745: f64 = (8.0 * var_t10);
        let assign60690_e94747: f64 = (assign60690_e94745 * var_t4);
        let assign60690_e94749: f64 = (assign60690_e94747 * var_t4);
        (assign60690_e94749, (((((8.0 * var_t10_dn0) * var_t4) + (assign60690_e94745 * var_t4_dn0)) * var_t4) + (assign60690_e94747 * var_t4_dn0)), (((((8.0 * var_t10_dn2) * var_t4) + (assign60690_e94745 * var_t4_dn2)) * var_t4) + (assign60690_e94747 * var_t4_dn2)), (((((8.0 * var_t10_dn4) * var_t4) + (assign60690_e94745 * var_t4_dn4)) * var_t4) + (assign60690_e94747 * var_t4_dn4)), (((((8.0 * var_t10_dn5) * var_t4) + (assign60690_e94745 * var_t4_dn5)) * var_t4) + (assign60690_e94747 * var_t4_dn5)), (((((8.0 * var_t10_dn6) * var_t4) + (assign60690_e94745 * var_t4_dn6)) * var_t4) + (assign60690_e94747 * var_t4_dn6)), (((((8.0 * var_t10_dn7) * var_t4) + (assign60690_e94745 * var_t4_dn7)) * var_t4) + (assign60690_e94747 * var_t4_dn7)), (((((8.0 * var_t10_dn8) * var_t4) + (assign60690_e94745 * var_t4_dn8)) * var_t4) + (assign60690_e94747 * var_t4_dn8)), (((((8.0 * var_t10_dn9) * var_t4) + (assign60690_e94745 * var_t4_dn9)) * var_t4) + (assign60690_e94747 * var_t4_dn9)), (((((8.0 * var_t10_dn10) * var_t4) + (assign60690_e94745 * var_t4_dn10)) * var_t4) + (assign60690_e94747 * var_t4_dn10)), (((((8.0 * var_t10_dn13) * var_t4) + (assign60690_e94745 * var_t4_dn13)) * var_t4) + (assign60690_e94747 * var_t4_dn13)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign60690_e94751;
        var_t1_dn0 = assign60690_e94751_d_n0;
        var_t1_dn2 = assign60690_e94751_d_n2;
        var_t1_dn4 = assign60690_e94751_d_n4;
        var_t1_dn5 = assign60690_e94751_d_n5;
        var_t1_dn6 = assign60690_e94751_d_n6;
        var_t1_dn7 = assign60690_e94751_d_n7;
        var_t1_dn8 = assign60690_e94751_d_n8;
        var_t1_dn9 = assign60690_e94751_d_n9;
        var_t1_dn10 = assign60690_e94751_d_n10;
        var_t1_dn13 = assign60690_e94751_d_n13;
        var_t1_rv = 0.0;

        let (assign60700_e94767, assign60700_e94767_d_n0, assign60700_e94767_d_n2, assign60700_e94767_d_n4, assign60700_e94767_d_n5, assign60700_e94767_d_n6, assign60700_e94767_d_n7, assign60700_e94767_d_n8, assign60700_e94767_d_n9, assign60700_e94767_d_n10, assign60700_e94767_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let assign60700_e94763: f64 = (2.0 * var_t11);
        let assign60700_e94765: f64 = (assign60700_e94763 * var_t4);
        (assign60700_e94765, (((2.0 * var_t11_dn0) * var_t4) + (assign60700_e94763 * var_t4_dn0)), (((2.0 * var_t11_dn2) * var_t4) + (assign60700_e94763 * var_t4_dn2)), (((2.0 * var_t11_dn4) * var_t4) + (assign60700_e94763 * var_t4_dn4)), (((2.0 * var_t11_dn5) * var_t4) + (assign60700_e94763 * var_t4_dn5)), (((2.0 * var_t11_dn6) * var_t4) + (assign60700_e94763 * var_t4_dn6)), (((2.0 * var_t11_dn7) * var_t4) + (assign60700_e94763 * var_t4_dn7)), (((2.0 * var_t11_dn8) * var_t4) + (assign60700_e94763 * var_t4_dn8)), (((2.0 * var_t11_dn9) * var_t4) + (assign60700_e94763 * var_t4_dn9)), (((2.0 * var_t11_dn10) * var_t4) + (assign60700_e94763 * var_t4_dn10)), (((2.0 * var_t11_dn13) * var_t4) + (assign60700_e94763 * var_t4_dn13)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign60700_e94767;
        var_t2_dn0 = assign60700_e94767_d_n0;
        var_t2_dn2 = assign60700_e94767_d_n2;
        var_t2_dn4 = assign60700_e94767_d_n4;
        var_t2_dn5 = assign60700_e94767_d_n5;
        var_t2_dn6 = assign60700_e94767_d_n6;
        var_t2_dn7 = assign60700_e94767_d_n7;
        var_t2_dn8 = assign60700_e94767_d_n8;
        var_t2_dn9 = assign60700_e94767_d_n9;
        var_t2_dn10 = assign60700_e94767_d_n10;
        var_t2_dn13 = assign60700_e94767_d_n13;
        var_t2_rv = 0.0;

        *var_dnm_slot = var_dnm;
        *var_dnm_dn0_slot = var_dnm_dn0;
        *var_dnm_dn10_slot = var_dnm_dn10;
        *var_dnm_dn13_slot = var_dnm_dn13;
        *var_dnm_dn2_slot = var_dnm_dn2;
        *var_dnm_dn4_slot = var_dnm_dn4;
        *var_dnm_dn5_slot = var_dnm_dn5;
        *var_dnm_dn6_slot = var_dnm_dn6;
        *var_dnm_dn7_slot = var_dnm_dn7;
        *var_dnm_dn8_slot = var_dnm_dn8;
        *var_dnm_dn9_slot = var_dnm_dn9;
        *var_dnm_rv_slot = var_dnm_rv;
        *var_psdl_slot = var_psdl;
        *var_psdl_dn0_slot = var_psdl_dn0;
        *var_psdl_dn10_slot = var_psdl_dn10;
        *var_psdl_dn13_slot = var_psdl_dn13;
        *var_psdl_dn2_slot = var_psdl_dn2;
        *var_psdl_dn4_slot = var_psdl_dn4;
        *var_psdl_dn5_slot = var_psdl_dn5;
        *var_psdl_dn6_slot = var_psdl_dn6;
        *var_psdl_dn7_slot = var_psdl_dn7;
        *var_psdl_dn8_slot = var_psdl_dn8;
        *var_psdl_dn9_slot = var_psdl_dn9;
        *var_psdl_rv_slot = var_psdl_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn13_slot = var_t0_dn13;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t10_slot = var_t10;
        *var_t10_dn0_slot = var_t10_dn0;
        *var_t10_dn10_slot = var_t10_dn10;
        *var_t10_dn13_slot = var_t10_dn13;
        *var_t10_dn2_slot = var_t10_dn2;
        *var_t10_dn4_slot = var_t10_dn4;
        *var_t10_dn5_slot = var_t10_dn5;
        *var_t10_dn6_slot = var_t10_dn6;
        *var_t10_dn7_slot = var_t10_dn7;
        *var_t10_dn8_slot = var_t10_dn8;
        *var_t10_dn9_slot = var_t10_dn9;
        *var_t10_rv_slot = var_t10_rv;
        *var_t11_slot = var_t11;
        *var_t11_dn0_slot = var_t11_dn0;
        *var_t11_dn10_slot = var_t11_dn10;
        *var_t11_dn13_slot = var_t11_dn13;
        *var_t11_dn2_slot = var_t11_dn2;
        *var_t11_dn4_slot = var_t11_dn4;
        *var_t11_dn5_slot = var_t11_dn5;
        *var_t11_dn6_slot = var_t11_dn6;
        *var_t11_dn7_slot = var_t11_dn7;
        *var_t11_dn8_slot = var_t11_dn8;
        *var_t11_dn9_slot = var_t11_dn9;
        *var_t11_rv_slot = var_t11_rv;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn13_slot = var_t1_dn13;
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
        *var_t2_dn13_slot = var_t2_dn13;
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
        *var_t3_dn13_slot = var_t3_dn13;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t3_rv_slot = var_t3_rv;
        *var_t5_slot = var_t5;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn10_slot = var_t5_dn10;
        *var_t5_dn13_slot = var_t5_dn13;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t5_dn9_slot = var_t5_dn9;
        *var_t5_rv_slot = var_t5_rv;
        *var_t6_slot = var_t6;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn13_slot = var_t6_dn13;
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
        *var_t7_dn13_slot = var_t7_dn13;
        *var_t7_dn2_slot = var_t7_dn2;
        *var_t7_dn4_slot = var_t7_dn4;
        *var_t7_dn5_slot = var_t7_dn5;
        *var_t7_dn6_slot = var_t7_dn6;
        *var_t7_dn7_slot = var_t7_dn7;
        *var_t7_dn8_slot = var_t7_dn8;
        *var_t7_dn9_slot = var_t7_dn9;
        *var_t7_rv_slot = var_t7_rv;
        *var_tmf0_slot = var_tmf0;
        *var_tmf0_dn0_slot = var_tmf0_dn0;
        *var_tmf0_dn10_slot = var_tmf0_dn10;
        *var_tmf0_dn13_slot = var_tmf0_dn13;
        *var_tmf0_dn2_slot = var_tmf0_dn2;
        *var_tmf0_dn4_slot = var_tmf0_dn4;
        *var_tmf0_dn5_slot = var_tmf0_dn5;
        *var_tmf0_dn6_slot = var_tmf0_dn6;
        *var_tmf0_dn7_slot = var_tmf0_dn7;
        *var_tmf0_dn8_slot = var_tmf0_dn8;
        *var_tmf0_dn9_slot = var_tmf0_dn9;
        *var_tmf0_rv_slot = var_tmf0_rv;
    }

    pub(super) fn stamp_reactive_block_217(
        var_beta: f64,
        var_beta_dn0: f64,
        var_beta_dn10: f64,
        var_beta_dn13: f64,
        var_beta_dn2: f64,
        var_beta_dn4: f64,
        var_beta_dn5: f64,
        var_beta_dn6: f64,
        var_beta_dn7: f64,
        var_beta_dn8: f64,
        var_beta_dn9: f64,
        var_beta_inv: f64,
        var_beta_inv_dn0: f64,
        var_beta_inv_dn10: f64,
        var_beta_inv_dn13: f64,
        var_beta_inv_dn2: f64,
        var_beta_inv_dn4: f64,
        var_beta_inv_dn5: f64,
        var_beta_inv_dn6: f64,
        var_beta_inv_dn7: f64,
        var_beta_inv_dn8: f64,
        var_beta_inv_dn9: f64,
        var_clmmod: f64,
        var_cnst0: f64,
        var_cnst0_dn0: f64,
        var_cnst0_dn10: f64,
        var_cnst0_dn13: f64,
        var_cnst0_dn2: f64,
        var_cnst0_dn4: f64,
        var_cnst0_dn5: f64,
        var_cnst0_dn6: f64,
        var_cnst0_dn7: f64,
        var_cnst0_dn8: f64,
        var_cnst0_dn9: f64,
        var_cox: f64,
        var_cox_dn0: f64,
        var_cox_dn10: f64,
        var_cox_dn13: f64,
        var_cox_dn2: f64,
        var_cox_dn4: f64,
        var_cox_dn5: f64,
        var_cox_dn6: f64,
        var_cox_dn7: f64,
        var_cox_dn8: f64,
        var_cox_dn9: f64,
        var_f10: f64,
        var_f10_dn0: f64,
        var_f10_dn10: f64,
        var_f10_dn13: f64,
        var_f10_dn2: f64,
        var_f10_dn4: f64,
        var_f10_dn5: f64,
        var_f10_dn6: f64,
        var_f10_dn7: f64,
        var_f10_dn8: f64,
        var_f10_dn9: f64,
        var_f11: f64,
        var_f11_dn0: f64,
        var_f11_dn10: f64,
        var_f11_dn13: f64,
        var_f11_dn2: f64,
        var_f11_dn4: f64,
        var_f11_dn5: f64,
        var_f11_dn6: f64,
        var_f11_dn7: f64,
        var_f11_dn8: f64,
        var_f11_dn9: f64,
        var_fac1: f64,
        var_fac1_dn0: f64,
        var_fac1_dn10: f64,
        var_fac1_dn13: f64,
        var_fac1_dn2: f64,
        var_fac1_dn4: f64,
        var_fac1_dn5: f64,
        var_fac1_dn6: f64,
        var_fac1_dn7: f64,
        var_fac1_dn8: f64,
        var_fac1_dn9: f64,
        var_fdd: f64,
        var_fdd_dn0: f64,
        var_fdd_dn10: f64,
        var_fdd_dn13: f64,
        var_fdd_dn2: f64,
        var_fdd_dn4: f64,
        var_fdd_dn5: f64,
        var_fdd_dn6: f64,
        var_fdd_dn7: f64,
        var_fdd_dn8: f64,
        var_fdd_dn9: f64,
        var_fmdvds: f64,
        var_fmdvds_dn0: f64,
        var_fmdvds_dn10: f64,
        var_fmdvds_dn13: f64,
        var_fmdvds_dn2: f64,
        var_fmdvds_dn4: f64,
        var_fmdvds_dn5: f64,
        var_fmdvds_dn6: f64,
        var_fmdvds_dn7: f64,
        var_fmdvds_dn8: f64,
        var_fmdvds_dn9: f64,
        var_guard1430: f64,
        var_guard1461: f64,
        var_guard1462: f64,
        var_guard443: f64,
        var_pds: f64,
        var_pds_dn0: f64,
        var_pds_dn10: f64,
        var_pds_dn13: f64,
        var_pds_dn2: f64,
        var_pds_dn4: f64,
        var_pds_dn5: f64,
        var_pds_dn6: f64,
        var_pds_dn7: f64,
        var_pds_dn8: f64,
        var_pds_dn9: f64,
        var_t11: f64,
        var_t11_dn0: f64,
        var_t11_dn10: f64,
        var_t11_dn13: f64,
        var_t11_dn2: f64,
        var_t11_dn4: f64,
        var_t11_dn5: f64,
        var_t11_dn6: f64,
        var_t11_dn7: f64,
        var_t11_dn8: f64,
        var_t11_dn9: f64,
        var_t4: f64,
        var_t4_dn0: f64,
        var_t4_dn10: f64,
        var_t4_dn13: f64,
        var_t4_dn2: f64,
        var_t4_dn4: f64,
        var_t4_dn5: f64,
        var_t4_dn6: f64,
        var_t4_dn7: f64,
        var_t4_dn8: f64,
        var_t4_dn9: f64,
        var_t7: f64,
        var_t7_dn0: f64,
        var_t7_dn10: f64,
        var_t7_dn13: f64,
        var_t7_dn2: f64,
        var_t7_dn4: f64,
        var_t7_dn5: f64,
        var_t7_dn6: f64,
        var_t7_dn7: f64,
        var_t7_dn8: f64,
        var_t7_dn9: f64,
        var_vgp: f64,
        var_vgp_dn0: f64,
        var_vgp_dn10: f64,
        var_vgp_dn13: f64,
        var_vgp_dn2: f64,
        var_vgp_dn4: f64,
        var_vgp_dn5: f64,
        var_vgp_dn6: f64,
        var_vgp_dn7: f64,
        var_vgp_dn8: f64,
        var_vgp_dn9: f64,
        var_vgvt: f64,
        var_vgvt_dn0: f64,
        var_vgvt_dn10: f64,
        var_vgvt_dn13: f64,
        var_vgvt_dn2: f64,
        var_vgvt_dn4: f64,
        var_vgvt_dn5: f64,
        var_vgvt_dn6: f64,
        var_vgvt_dn7: f64,
        var_vgvt_dn8: f64,
        var_vgvt_dn9: f64,
        var_xi0: f64,
        var_xi0_dn0: f64,
        var_xi0_dn10: f64,
        var_xi0_dn13: f64,
        var_xi0_dn2: f64,
        var_xi0_dn4: f64,
        var_xi0_dn5: f64,
        var_xi0_dn6: f64,
        var_xi0_dn7: f64,
        var_xi0_dn8: f64,
        var_xi0_dn9: f64,
        var_xi0p12: f64,
        var_xi0p12_dn0: f64,
        var_xi0p12_dn10: f64,
        var_xi0p12_dn13: f64,
        var_xi0p12_dn2: f64,
        var_xi0p12_dn4: f64,
        var_xi0p12_dn5: f64,
        var_xi0p12_dn6: f64,
        var_xi0p12_dn7: f64,
        var_xi0p12_dn8: f64,
        var_xi0p12_dn9: f64,
        var_achi_slot: &mut f64,
        var_achi_dn0_slot: &mut f64,
        var_achi_dn10_slot: &mut f64,
        var_achi_dn13_slot: &mut f64,
        var_achi_dn2_slot: &mut f64,
        var_achi_dn4_slot: &mut f64,
        var_achi_dn5_slot: &mut f64,
        var_achi_dn6_slot: &mut f64,
        var_achi_dn7_slot: &mut f64,
        var_achi_dn8_slot: &mut f64,
        var_achi_dn9_slot: &mut f64,
        var_achi_rv_slot: &mut f64,
        var_dtpds_slot: &mut f64,
        var_dtpds_dn0_slot: &mut f64,
        var_dtpds_dn10_slot: &mut f64,
        var_dtpds_dn13_slot: &mut f64,
        var_dtpds_dn2_slot: &mut f64,
        var_dtpds_dn4_slot: &mut f64,
        var_dtpds_dn5_slot: &mut f64,
        var_dtpds_dn6_slot: &mut f64,
        var_dtpds_dn7_slot: &mut f64,
        var_dtpds_dn8_slot: &mut f64,
        var_dtpds_dn9_slot: &mut f64,
        var_dtpds_rv_slot: &mut f64,
        var_lred_slot: &mut f64,
        var_lred_dn0_slot: &mut f64,
        var_lred_dn10_slot: &mut f64,
        var_lred_dn13_slot: &mut f64,
        var_lred_dn2_slot: &mut f64,
        var_lred_dn4_slot: &mut f64,
        var_lred_dn5_slot: &mut f64,
        var_lred_dn6_slot: &mut f64,
        var_lred_dn7_slot: &mut f64,
        var_lred_dn8_slot: &mut f64,
        var_lred_dn9_slot: &mut f64,
        var_lred_rv_slot: &mut f64,
        var_m0_slot: &mut f64,
        var_m0_rv_slot: &mut f64,
        var_qbnm_slot: &mut f64,
        var_qbnm_dn0_slot: &mut f64,
        var_qbnm_dn10_slot: &mut f64,
        var_qbnm_dn13_slot: &mut f64,
        var_qbnm_dn2_slot: &mut f64,
        var_qbnm_dn4_slot: &mut f64,
        var_qbnm_dn5_slot: &mut f64,
        var_qbnm_dn6_slot: &mut f64,
        var_qbnm_dn7_slot: &mut f64,
        var_qbnm_dn8_slot: &mut f64,
        var_qbnm_dn9_slot: &mut f64,
        var_qbnm_rv_slot: &mut f64,
        var_qbu_slot: &mut f64,
        var_qbu_dn0_slot: &mut f64,
        var_qbu_dn10_slot: &mut f64,
        var_qbu_dn13_slot: &mut f64,
        var_qbu_dn2_slot: &mut f64,
        var_qbu_dn4_slot: &mut f64,
        var_qbu_dn5_slot: &mut f64,
        var_qbu_dn6_slot: &mut f64,
        var_qbu_dn7_slot: &mut f64,
        var_qbu_dn8_slot: &mut f64,
        var_qbu_dn9_slot: &mut f64,
        var_qbu_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn13_slot: &mut f64,
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
        var_t2_dn13_slot: &mut f64,
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
        var_t3_dn13_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_t8_slot: &mut f64,
        var_t8_dn0_slot: &mut f64,
        var_t8_dn10_slot: &mut f64,
        var_t8_dn13_slot: &mut f64,
        var_t8_dn2_slot: &mut f64,
        var_t8_dn4_slot: &mut f64,
        var_t8_dn5_slot: &mut f64,
        var_t8_dn6_slot: &mut f64,
        var_t8_dn7_slot: &mut f64,
        var_t8_dn8_slot: &mut f64,
        var_t8_dn9_slot: &mut f64,
        var_t8_rv_slot: &mut f64,
        var_t9_slot: &mut f64,
        var_t9_dn0_slot: &mut f64,
        var_t9_dn10_slot: &mut f64,
        var_t9_dn13_slot: &mut f64,
        var_t9_dn2_slot: &mut f64,
        var_t9_dn4_slot: &mut f64,
        var_t9_dn5_slot: &mut f64,
        var_t9_dn6_slot: &mut f64,
        var_t9_dn7_slot: &mut f64,
        var_t9_dn8_slot: &mut f64,
        var_t9_dn9_slot: &mut f64,
        var_t9_rv_slot: &mut f64,
        var_tx_slot: &mut f64,
        var_tx_dn0_slot: &mut f64,
        var_tx_dn10_slot: &mut f64,
        var_tx_dn13_slot: &mut f64,
        var_tx_dn2_slot: &mut f64,
        var_tx_dn4_slot: &mut f64,
        var_tx_dn5_slot: &mut f64,
        var_tx_dn6_slot: &mut f64,
        var_tx_dn7_slot: &mut f64,
        var_tx_dn8_slot: &mut f64,
        var_tx_dn9_slot: &mut f64,
        var_tx_rv_slot: &mut f64,
        var_x2_slot: &mut f64,
        var_x2_dn0_slot: &mut f64,
        var_x2_dn10_slot: &mut f64,
        var_x2_dn13_slot: &mut f64,
        var_x2_dn2_slot: &mut f64,
        var_x2_dn4_slot: &mut f64,
        var_x2_dn5_slot: &mut f64,
        var_x2_dn6_slot: &mut f64,
        var_x2_dn7_slot: &mut f64,
        var_x2_dn8_slot: &mut f64,
        var_x2_dn9_slot: &mut f64,
        var_x2_rv_slot: &mut f64,
        var_xmax2_slot: &mut f64,
        var_xmax2_dn0_slot: &mut f64,
        var_xmax2_dn10_slot: &mut f64,
        var_xmax2_dn13_slot: &mut f64,
        var_xmax2_dn2_slot: &mut f64,
        var_xmax2_dn4_slot: &mut f64,
        var_xmax2_dn5_slot: &mut f64,
        var_xmax2_dn6_slot: &mut f64,
        var_xmax2_dn7_slot: &mut f64,
        var_xmax2_dn8_slot: &mut f64,
        var_xmax2_dn9_slot: &mut f64,
        var_xmax2_rv_slot: &mut f64,
        var_xmp_slot: &mut f64,
        var_xmp_dn0_slot: &mut f64,
        var_xmp_dn10_slot: &mut f64,
        var_xmp_dn13_slot: &mut f64,
        var_xmp_dn2_slot: &mut f64,
        var_xmp_dn4_slot: &mut f64,
        var_xmp_dn5_slot: &mut f64,
        var_xmp_dn6_slot: &mut f64,
        var_xmp_dn7_slot: &mut f64,
        var_xmp_dn8_slot: &mut f64,
        var_xmp_dn9_slot: &mut f64,
        var_xmp_rv_slot: &mut f64,
        var_xp_slot: &mut f64,
        var_xp_dn0_slot: &mut f64,
        var_xp_dn10_slot: &mut f64,
        var_xp_dn13_slot: &mut f64,
        var_xp_dn2_slot: &mut f64,
        var_xp_dn4_slot: &mut f64,
        var_xp_dn5_slot: &mut f64,
        var_xp_dn6_slot: &mut f64,
        var_xp_dn7_slot: &mut f64,
        var_xp_dn8_slot: &mut f64,
        var_xp_dn9_slot: &mut f64,
        var_xp_rv_slot: &mut f64,
    ) {
        let mut var_achi: f64 = *var_achi_slot;
        let mut var_achi_dn0: f64 = *var_achi_dn0_slot;
        let mut var_achi_dn10: f64 = *var_achi_dn10_slot;
        let mut var_achi_dn13: f64 = *var_achi_dn13_slot;
        let mut var_achi_dn2: f64 = *var_achi_dn2_slot;
        let mut var_achi_dn4: f64 = *var_achi_dn4_slot;
        let mut var_achi_dn5: f64 = *var_achi_dn5_slot;
        let mut var_achi_dn6: f64 = *var_achi_dn6_slot;
        let mut var_achi_dn7: f64 = *var_achi_dn7_slot;
        let mut var_achi_dn8: f64 = *var_achi_dn8_slot;
        let mut var_achi_dn9: f64 = *var_achi_dn9_slot;
        let mut var_achi_rv: f64 = *var_achi_rv_slot;
        let mut var_dtpds: f64 = *var_dtpds_slot;
        let mut var_dtpds_dn0: f64 = *var_dtpds_dn0_slot;
        let mut var_dtpds_dn10: f64 = *var_dtpds_dn10_slot;
        let mut var_dtpds_dn13: f64 = *var_dtpds_dn13_slot;
        let mut var_dtpds_dn2: f64 = *var_dtpds_dn2_slot;
        let mut var_dtpds_dn4: f64 = *var_dtpds_dn4_slot;
        let mut var_dtpds_dn5: f64 = *var_dtpds_dn5_slot;
        let mut var_dtpds_dn6: f64 = *var_dtpds_dn6_slot;
        let mut var_dtpds_dn7: f64 = *var_dtpds_dn7_slot;
        let mut var_dtpds_dn8: f64 = *var_dtpds_dn8_slot;
        let mut var_dtpds_dn9: f64 = *var_dtpds_dn9_slot;
        let mut var_dtpds_rv: f64 = *var_dtpds_rv_slot;
        let mut var_lred: f64 = *var_lred_slot;
        let mut var_lred_dn0: f64 = *var_lred_dn0_slot;
        let mut var_lred_dn10: f64 = *var_lred_dn10_slot;
        let mut var_lred_dn13: f64 = *var_lred_dn13_slot;
        let mut var_lred_dn2: f64 = *var_lred_dn2_slot;
        let mut var_lred_dn4: f64 = *var_lred_dn4_slot;
        let mut var_lred_dn5: f64 = *var_lred_dn5_slot;
        let mut var_lred_dn6: f64 = *var_lred_dn6_slot;
        let mut var_lred_dn7: f64 = *var_lred_dn7_slot;
        let mut var_lred_dn8: f64 = *var_lred_dn8_slot;
        let mut var_lred_dn9: f64 = *var_lred_dn9_slot;
        let mut var_lred_rv: f64 = *var_lred_rv_slot;
        let mut var_m0: f64 = *var_m0_slot;
        let mut var_m0_rv: f64 = *var_m0_rv_slot;
        let mut var_qbnm: f64 = *var_qbnm_slot;
        let mut var_qbnm_dn0: f64 = *var_qbnm_dn0_slot;
        let mut var_qbnm_dn10: f64 = *var_qbnm_dn10_slot;
        let mut var_qbnm_dn13: f64 = *var_qbnm_dn13_slot;
        let mut var_qbnm_dn2: f64 = *var_qbnm_dn2_slot;
        let mut var_qbnm_dn4: f64 = *var_qbnm_dn4_slot;
        let mut var_qbnm_dn5: f64 = *var_qbnm_dn5_slot;
        let mut var_qbnm_dn6: f64 = *var_qbnm_dn6_slot;
        let mut var_qbnm_dn7: f64 = *var_qbnm_dn7_slot;
        let mut var_qbnm_dn8: f64 = *var_qbnm_dn8_slot;
        let mut var_qbnm_dn9: f64 = *var_qbnm_dn9_slot;
        let mut var_qbnm_rv: f64 = *var_qbnm_rv_slot;
        let mut var_qbu: f64 = *var_qbu_slot;
        let mut var_qbu_dn0: f64 = *var_qbu_dn0_slot;
        let mut var_qbu_dn10: f64 = *var_qbu_dn10_slot;
        let mut var_qbu_dn13: f64 = *var_qbu_dn13_slot;
        let mut var_qbu_dn2: f64 = *var_qbu_dn2_slot;
        let mut var_qbu_dn4: f64 = *var_qbu_dn4_slot;
        let mut var_qbu_dn5: f64 = *var_qbu_dn5_slot;
        let mut var_qbu_dn6: f64 = *var_qbu_dn6_slot;
        let mut var_qbu_dn7: f64 = *var_qbu_dn7_slot;
        let mut var_qbu_dn8: f64 = *var_qbu_dn8_slot;
        let mut var_qbu_dn9: f64 = *var_qbu_dn9_slot;
        let mut var_qbu_rv: f64 = *var_qbu_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
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
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
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
        let mut var_t3_dn13: f64 = *var_t3_dn13_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_t8: f64 = *var_t8_slot;
        let mut var_t8_dn0: f64 = *var_t8_dn0_slot;
        let mut var_t8_dn10: f64 = *var_t8_dn10_slot;
        let mut var_t8_dn13: f64 = *var_t8_dn13_slot;
        let mut var_t8_dn2: f64 = *var_t8_dn2_slot;
        let mut var_t8_dn4: f64 = *var_t8_dn4_slot;
        let mut var_t8_dn5: f64 = *var_t8_dn5_slot;
        let mut var_t8_dn6: f64 = *var_t8_dn6_slot;
        let mut var_t8_dn7: f64 = *var_t8_dn7_slot;
        let mut var_t8_dn8: f64 = *var_t8_dn8_slot;
        let mut var_t8_dn9: f64 = *var_t8_dn9_slot;
        let mut var_t8_rv: f64 = *var_t8_rv_slot;
        let mut var_t9: f64 = *var_t9_slot;
        let mut var_t9_dn0: f64 = *var_t9_dn0_slot;
        let mut var_t9_dn10: f64 = *var_t9_dn10_slot;
        let mut var_t9_dn13: f64 = *var_t9_dn13_slot;
        let mut var_t9_dn2: f64 = *var_t9_dn2_slot;
        let mut var_t9_dn4: f64 = *var_t9_dn4_slot;
        let mut var_t9_dn5: f64 = *var_t9_dn5_slot;
        let mut var_t9_dn6: f64 = *var_t9_dn6_slot;
        let mut var_t9_dn7: f64 = *var_t9_dn7_slot;
        let mut var_t9_dn8: f64 = *var_t9_dn8_slot;
        let mut var_t9_dn9: f64 = *var_t9_dn9_slot;
        let mut var_t9_rv: f64 = *var_t9_rv_slot;
        let mut var_tx: f64 = *var_tx_slot;
        let mut var_tx_dn0: f64 = *var_tx_dn0_slot;
        let mut var_tx_dn10: f64 = *var_tx_dn10_slot;
        let mut var_tx_dn13: f64 = *var_tx_dn13_slot;
        let mut var_tx_dn2: f64 = *var_tx_dn2_slot;
        let mut var_tx_dn4: f64 = *var_tx_dn4_slot;
        let mut var_tx_dn5: f64 = *var_tx_dn5_slot;
        let mut var_tx_dn6: f64 = *var_tx_dn6_slot;
        let mut var_tx_dn7: f64 = *var_tx_dn7_slot;
        let mut var_tx_dn8: f64 = *var_tx_dn8_slot;
        let mut var_tx_dn9: f64 = *var_tx_dn9_slot;
        let mut var_tx_rv: f64 = *var_tx_rv_slot;
        let mut var_x2: f64 = *var_x2_slot;
        let mut var_x2_dn0: f64 = *var_x2_dn0_slot;
        let mut var_x2_dn10: f64 = *var_x2_dn10_slot;
        let mut var_x2_dn13: f64 = *var_x2_dn13_slot;
        let mut var_x2_dn2: f64 = *var_x2_dn2_slot;
        let mut var_x2_dn4: f64 = *var_x2_dn4_slot;
        let mut var_x2_dn5: f64 = *var_x2_dn5_slot;
        let mut var_x2_dn6: f64 = *var_x2_dn6_slot;
        let mut var_x2_dn7: f64 = *var_x2_dn7_slot;
        let mut var_x2_dn8: f64 = *var_x2_dn8_slot;
        let mut var_x2_dn9: f64 = *var_x2_dn9_slot;
        let mut var_x2_rv: f64 = *var_x2_rv_slot;
        let mut var_xmax2: f64 = *var_xmax2_slot;
        let mut var_xmax2_dn0: f64 = *var_xmax2_dn0_slot;
        let mut var_xmax2_dn10: f64 = *var_xmax2_dn10_slot;
        let mut var_xmax2_dn13: f64 = *var_xmax2_dn13_slot;
        let mut var_xmax2_dn2: f64 = *var_xmax2_dn2_slot;
        let mut var_xmax2_dn4: f64 = *var_xmax2_dn4_slot;
        let mut var_xmax2_dn5: f64 = *var_xmax2_dn5_slot;
        let mut var_xmax2_dn6: f64 = *var_xmax2_dn6_slot;
        let mut var_xmax2_dn7: f64 = *var_xmax2_dn7_slot;
        let mut var_xmax2_dn8: f64 = *var_xmax2_dn8_slot;
        let mut var_xmax2_dn9: f64 = *var_xmax2_dn9_slot;
        let mut var_xmax2_rv: f64 = *var_xmax2_rv_slot;
        let mut var_xmp: f64 = *var_xmp_slot;
        let mut var_xmp_dn0: f64 = *var_xmp_dn0_slot;
        let mut var_xmp_dn10: f64 = *var_xmp_dn10_slot;
        let mut var_xmp_dn13: f64 = *var_xmp_dn13_slot;
        let mut var_xmp_dn2: f64 = *var_xmp_dn2_slot;
        let mut var_xmp_dn4: f64 = *var_xmp_dn4_slot;
        let mut var_xmp_dn5: f64 = *var_xmp_dn5_slot;
        let mut var_xmp_dn6: f64 = *var_xmp_dn6_slot;
        let mut var_xmp_dn7: f64 = *var_xmp_dn7_slot;
        let mut var_xmp_dn8: f64 = *var_xmp_dn8_slot;
        let mut var_xmp_dn9: f64 = *var_xmp_dn9_slot;
        let mut var_xmp_rv: f64 = *var_xmp_rv_slot;
        let mut var_xp: f64 = *var_xp_slot;
        let mut var_xp_dn0: f64 = *var_xp_dn0_slot;
        let mut var_xp_dn10: f64 = *var_xp_dn10_slot;
        let mut var_xp_dn13: f64 = *var_xp_dn13_slot;
        let mut var_xp_dn2: f64 = *var_xp_dn2_slot;
        let mut var_xp_dn4: f64 = *var_xp_dn4_slot;
        let mut var_xp_dn5: f64 = *var_xp_dn5_slot;
        let mut var_xp_dn6: f64 = *var_xp_dn6_slot;
        let mut var_xp_dn7: f64 = *var_xp_dn7_slot;
        let mut var_xp_dn8: f64 = *var_xp_dn8_slot;
        let mut var_xp_dn9: f64 = *var_xp_dn9_slot;
        let mut var_xp_rv: f64 = *var_xp_rv_slot;

        let (assign60710_e94783, assign60710_e94783_d_n0, assign60710_e94783_d_n2, assign60710_e94783_d_n4, assign60710_e94783_d_n5, assign60710_e94783_d_n6, assign60710_e94783_d_n7, assign60710_e94783_d_n8, assign60710_e94783_d_n9, assign60710_e94783_d_n10, assign60710_e94783_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let assign60710_e94779: f64 = (var_t11 * var_t4);
        let assign60710_e94781: f64 = (assign60710_e94779 * var_t4);
        (assign60710_e94781, ((((var_t11_dn0 * var_t4) + (var_t11 * var_t4_dn0)) * var_t4) + (assign60710_e94779 * var_t4_dn0)), ((((var_t11_dn2 * var_t4) + (var_t11 * var_t4_dn2)) * var_t4) + (assign60710_e94779 * var_t4_dn2)), ((((var_t11_dn4 * var_t4) + (var_t11 * var_t4_dn4)) * var_t4) + (assign60710_e94779 * var_t4_dn4)), ((((var_t11_dn5 * var_t4) + (var_t11 * var_t4_dn5)) * var_t4) + (assign60710_e94779 * var_t4_dn5)), ((((var_t11_dn6 * var_t4) + (var_t11 * var_t4_dn6)) * var_t4) + (assign60710_e94779 * var_t4_dn6)), ((((var_t11_dn7 * var_t4) + (var_t11 * var_t4_dn7)) * var_t4) + (assign60710_e94779 * var_t4_dn7)), ((((var_t11_dn8 * var_t4) + (var_t11 * var_t4_dn8)) * var_t4) + (assign60710_e94779 * var_t4_dn8)), ((((var_t11_dn9 * var_t4) + (var_t11 * var_t4_dn9)) * var_t4) + (assign60710_e94779 * var_t4_dn9)), ((((var_t11_dn10 * var_t4) + (var_t11 * var_t4_dn10)) * var_t4) + (assign60710_e94779 * var_t4_dn10)), ((((var_t11_dn13 * var_t4) + (var_t11 * var_t4_dn13)) * var_t4) + (assign60710_e94779 * var_t4_dn13)),)
    } else {
        (var_t8, var_t8_dn0, var_t8_dn2, var_t8_dn4, var_t8_dn5, var_t8_dn6, var_t8_dn7, var_t8_dn8, var_t8_dn9, var_t8_dn10, var_t8_dn13,)
    }
};
        var_t8 = assign60710_e94783;
        var_t8_dn0 = assign60710_e94783_d_n0;
        var_t8_dn2 = assign60710_e94783_d_n2;
        var_t8_dn4 = assign60710_e94783_d_n4;
        var_t8_dn5 = assign60710_e94783_d_n5;
        var_t8_dn6 = assign60710_e94783_d_n6;
        var_t8_dn7 = assign60710_e94783_d_n7;
        var_t8_dn8 = assign60710_e94783_d_n8;
        var_t8_dn9 = assign60710_e94783_d_n9;
        var_t8_dn10 = assign60710_e94783_d_n10;
        var_t8_dn13 = assign60710_e94783_d_n13;
        var_t8_rv = 0.0;

        let (assign60720_e94800, assign60720_e94800_d_n0, assign60720_e94800_d_n2, assign60720_e94800_d_n4, assign60720_e94800_d_n5, assign60720_e94800_d_n6, assign60720_e94800_d_n7, assign60720_e94800_d_n8, assign60720_e94800_d_n9, assign60720_e94800_d_n10, assign60720_e94800_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let assign60720_e94795: f64 = (var_t7 * var_t7);
        let assign60720_e94797: f64 = (assign60720_e94795 + var_t8);
        let assign60720_e94798: f64 = (assign60720_e94797).sqrt();
        (assign60720_e94798, ((((var_t7_dn0 * var_t7) + (var_t7 * var_t7_dn0)) + var_t8_dn0) / (2.0 * assign60720_e94798)), ((((var_t7_dn2 * var_t7) + (var_t7 * var_t7_dn2)) + var_t8_dn2) / (2.0 * assign60720_e94798)), ((((var_t7_dn4 * var_t7) + (var_t7 * var_t7_dn4)) + var_t8_dn4) / (2.0 * assign60720_e94798)), ((((var_t7_dn5 * var_t7) + (var_t7 * var_t7_dn5)) + var_t8_dn5) / (2.0 * assign60720_e94798)), ((((var_t7_dn6 * var_t7) + (var_t7 * var_t7_dn6)) + var_t8_dn6) / (2.0 * assign60720_e94798)), ((((var_t7_dn7 * var_t7) + (var_t7 * var_t7_dn7)) + var_t8_dn7) / (2.0 * assign60720_e94798)), ((((var_t7_dn8 * var_t7) + (var_t7 * var_t7_dn8)) + var_t8_dn8) / (2.0 * assign60720_e94798)), ((((var_t7_dn9 * var_t7) + (var_t7 * var_t7_dn9)) + var_t8_dn9) / (2.0 * assign60720_e94798)), ((((var_t7_dn10 * var_t7) + (var_t7 * var_t7_dn10)) + var_t8_dn10) / (2.0 * assign60720_e94798)), ((((var_t7_dn13 * var_t7) + (var_t7 * var_t7_dn13)) + var_t8_dn13) / (2.0 * assign60720_e94798)),)
    } else {
        (var_t9, var_t9_dn0, var_t9_dn2, var_t9_dn4, var_t9_dn5, var_t9_dn6, var_t9_dn7, var_t9_dn8, var_t9_dn9, var_t9_dn10, var_t9_dn13,)
    }
};
        var_t9 = assign60720_e94800;
        var_t9_dn0 = assign60720_e94800_d_n0;
        var_t9_dn2 = assign60720_e94800_d_n2;
        var_t9_dn4 = assign60720_e94800_d_n4;
        var_t9_dn5 = assign60720_e94800_d_n5;
        var_t9_dn6 = assign60720_e94800_d_n6;
        var_t9_dn7 = assign60720_e94800_d_n7;
        var_t9_dn8 = assign60720_e94800_d_n8;
        var_t9_dn9 = assign60720_e94800_d_n9;
        var_t9_dn10 = assign60720_e94800_d_n10;
        var_t9_dn13 = assign60720_e94800_d_n13;
        var_t9_rv = 0.0;

        let (assign60730_e94817, assign60730_e94817_d_n0, assign60730_e94817_d_n2, assign60730_e94817_d_n4, assign60730_e94817_d_n5, assign60730_e94817_d_n6, assign60730_e94817_d_n7, assign60730_e94817_d_n8, assign60730_e94817_d_n9, assign60730_e94817_d_n10, assign60730_e94817_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let assign60730_e94812: f64 = (-var_t7);
        let assign60730_e94814: f64 = (assign60730_e94812 + var_t9);
        let assign60730_e94815: f64 = (0.5 * assign60730_e94814);
        (assign60730_e94815, (0.5 * ((-var_t7_dn0) + var_t9_dn0)), (0.5 * ((-var_t7_dn2) + var_t9_dn2)), (0.5 * ((-var_t7_dn4) + var_t9_dn4)), (0.5 * ((-var_t7_dn5) + var_t9_dn5)), (0.5 * ((-var_t7_dn6) + var_t9_dn6)), (0.5 * ((-var_t7_dn7) + var_t9_dn7)), (0.5 * ((-var_t7_dn8) + var_t9_dn8)), (0.5 * ((-var_t7_dn9) + var_t9_dn9)), (0.5 * ((-var_t7_dn10) + var_t9_dn10)), (0.5 * ((-var_t7_dn13) + var_t9_dn13)),)
    } else {
        (var_lred, var_lred_dn0, var_lred_dn2, var_lred_dn4, var_lred_dn5, var_lred_dn6, var_lred_dn7, var_lred_dn8, var_lred_dn9, var_lred_dn10, var_lred_dn13,)
    }
};
        var_lred = assign60730_e94817;
        var_lred_dn0 = assign60730_e94817_d_n0;
        var_lred_dn2 = assign60730_e94817_d_n2;
        var_lred_dn4 = assign60730_e94817_d_n4;
        var_lred_dn5 = assign60730_e94817_d_n5;
        var_lred_dn6 = assign60730_e94817_d_n6;
        var_lred_dn7 = assign60730_e94817_d_n7;
        var_lred_dn8 = assign60730_e94817_d_n8;
        var_lred_dn9 = assign60730_e94817_d_n9;
        var_lred_dn10 = assign60730_e94817_d_n10;
        var_lred_dn13 = assign60730_e94817_d_n13;
        var_lred_rv = 0.0;

        let (assign60740_e94829, assign60740_e94829_d_n0, assign60740_e94829_d_n2, assign60740_e94829_d_n4, assign60740_e94829_d_n5, assign60740_e94829_d_n6, assign60740_e94829_d_n7, assign60740_e94829_d_n8, assign60740_e94829_d_n9, assign60740_e94829_d_n10, assign60740_e94829_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        (var_lred, var_lred_dn0, var_lred_dn2, var_lred_dn4, var_lred_dn5, var_lred_dn6, var_lred_dn7, var_lred_dn8, var_lred_dn9, var_lred_dn10, var_lred_dn13,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign60740_e94829;
        var_t1_dn0 = assign60740_e94829_d_n0;
        var_t1_dn2 = assign60740_e94829_d_n2;
        var_t1_dn4 = assign60740_e94829_d_n4;
        var_t1_dn5 = assign60740_e94829_d_n5;
        var_t1_dn6 = assign60740_e94829_d_n6;
        var_t1_dn7 = assign60740_e94829_d_n7;
        var_t1_dn8 = assign60740_e94829_d_n8;
        var_t1_dn9 = assign60740_e94829_d_n9;
        var_t1_dn10 = assign60740_e94829_d_n10;
        var_t1_dn13 = assign60740_e94829_d_n13;
        var_t1_rv = 0.0;

        let (assign60750_e94843, assign60750_e94843_d_n0, assign60750_e94843_d_n2, assign60750_e94843_d_n4, assign60750_e94843_d_n5, assign60750_e94843_d_n6, assign60750_e94843_d_n7, assign60750_e94843_d_n8, assign60750_e94843_d_n9, assign60750_e94843_d_n10, assign60750_e94843_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1462 == 0.0)) {
        let assign60750_e94841: f64 = (var_fmdvds * var_t1);
        (assign60750_e94841, ((var_fmdvds_dn0 * var_t1) + (var_fmdvds * var_t1_dn0)), ((var_fmdvds_dn2 * var_t1) + (var_fmdvds * var_t1_dn2)), ((var_fmdvds_dn4 * var_t1) + (var_fmdvds * var_t1_dn4)), ((var_fmdvds_dn5 * var_t1) + (var_fmdvds * var_t1_dn5)), ((var_fmdvds_dn6 * var_t1) + (var_fmdvds * var_t1_dn6)), ((var_fmdvds_dn7 * var_t1) + (var_fmdvds * var_t1_dn7)), ((var_fmdvds_dn8 * var_t1) + (var_fmdvds * var_t1_dn8)), ((var_fmdvds_dn9 * var_t1) + (var_fmdvds * var_t1_dn9)), ((var_fmdvds_dn10 * var_t1) + (var_fmdvds * var_t1_dn10)), ((var_fmdvds_dn13 * var_t1) + (var_fmdvds * var_t1_dn13)),)
    } else {
        (var_lred, var_lred_dn0, var_lred_dn2, var_lred_dn4, var_lred_dn5, var_lred_dn6, var_lred_dn7, var_lred_dn8, var_lred_dn9, var_lred_dn10, var_lred_dn13,)
    }
};
        var_lred = assign60750_e94843;
        var_lred_dn0 = assign60750_e94843_d_n0;
        var_lred_dn2 = assign60750_e94843_d_n2;
        var_lred_dn4 = assign60750_e94843_d_n4;
        var_lred_dn5 = assign60750_e94843_d_n5;
        var_lred_dn6 = assign60750_e94843_d_n6;
        var_lred_dn7 = assign60750_e94843_d_n7;
        var_lred_dn8 = assign60750_e94843_d_n8;
        var_lred_dn9 = assign60750_e94843_d_n9;
        var_lred_dn10 = assign60750_e94843_d_n10;
        var_lred_dn13 = assign60750_e94843_d_n13;
        var_lred_rv = 0.0;

        let (assign60760_e94854, assign60760_e94854_d_n0, assign60760_e94854_d_n2, assign60760_e94854_d_n4, assign60760_e94854_d_n5, assign60760_e94854_d_n6, assign60760_e94854_d_n7, assign60760_e94854_d_n8, assign60760_e94854_d_n9, assign60760_e94854_d_n10, assign60760_e94854_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign60760_e94852: f64 = (var_lred * var_clmmod);
        (assign60760_e94852, (var_lred_dn0 * var_clmmod), (var_lred_dn2 * var_clmmod), (var_lred_dn4 * var_clmmod), (var_lred_dn5 * var_clmmod), (var_lred_dn6 * var_clmmod), (var_lred_dn7 * var_clmmod), (var_lred_dn8 * var_clmmod), (var_lred_dn9 * var_clmmod), (var_lred_dn10 * var_clmmod), (var_lred_dn13 * var_clmmod),)
    } else {
        (var_lred, var_lred_dn0, var_lred_dn2, var_lred_dn4, var_lred_dn5, var_lred_dn6, var_lred_dn7, var_lred_dn8, var_lred_dn9, var_lred_dn10, var_lred_dn13,)
    }
};
        var_lred = assign60760_e94854;
        var_lred_dn0 = assign60760_e94854_d_n0;
        var_lred_dn2 = assign60760_e94854_d_n2;
        var_lred_dn4 = assign60760_e94854_d_n4;
        var_lred_dn5 = assign60760_e94854_d_n5;
        var_lred_dn6 = assign60760_e94854_d_n6;
        var_lred_dn7 = assign60760_e94854_d_n7;
        var_lred_dn8 = assign60760_e94854_d_n8;
        var_lred_dn9 = assign60760_e94854_d_n9;
        var_lred_dn10 = assign60760_e94854_d_n10;
        var_lred_dn13 = assign60760_e94854_d_n13;
        var_lred_rv = 0.0;

        let (assign60770_e94865, assign60770_e94865_d_n0, assign60770_e94865_d_n2, assign60770_e94865_d_n4, assign60770_e94865_d_n5, assign60770_e94865_d_n6, assign60770_e94865_d_n7, assign60770_e94865_d_n8, assign60770_e94865_d_n9, assign60770_e94865_d_n10, assign60770_e94865_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign60770_e94863: f64 = (var_vgp + var_beta_inv);
        (assign60770_e94863, (var_vgp_dn0 + var_beta_inv_dn0), (var_vgp_dn2 + var_beta_inv_dn2), (var_vgp_dn4 + var_beta_inv_dn4), (var_vgp_dn5 + var_beta_inv_dn5), (var_vgp_dn6 + var_beta_inv_dn6), (var_vgp_dn7 + var_beta_inv_dn7), (var_vgp_dn8 + var_beta_inv_dn8), (var_vgp_dn9 + var_beta_inv_dn9), (var_vgp_dn10 + var_beta_inv_dn10), (var_vgp_dn13 + var_beta_inv_dn13),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign60770_e94865;
        var_t1_dn0 = assign60770_e94865_d_n0;
        var_t1_dn2 = assign60770_e94865_d_n2;
        var_t1_dn4 = assign60770_e94865_d_n4;
        var_t1_dn5 = assign60770_e94865_d_n5;
        var_t1_dn6 = assign60770_e94865_d_n6;
        var_t1_dn7 = assign60770_e94865_d_n7;
        var_t1_dn8 = assign60770_e94865_d_n8;
        var_t1_dn9 = assign60770_e94865_d_n9;
        var_t1_dn10 = assign60770_e94865_d_n10;
        var_t1_dn13 = assign60770_e94865_d_n13;
        var_t1_rv = 0.0;

        let (assign60780_e94878, assign60780_e94878_d_n0, assign60780_e94878_d_n2, assign60780_e94878_d_n4, assign60780_e94878_d_n5, assign60780_e94878_d_n6, assign60780_e94878_d_n7, assign60780_e94878_d_n8, assign60780_e94878_d_n9, assign60780_e94878_d_n10, assign60780_e94878_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign60780_e94874: f64 = (var_t1 * var_f10);
        let assign60780_e94876: f64 = (assign60780_e94874 - var_f11);
        (assign60780_e94876, (((var_t1_dn0 * var_f10) + (var_t1 * var_f10_dn0)) - var_f11_dn0), (((var_t1_dn2 * var_f10) + (var_t1 * var_f10_dn2)) - var_f11_dn2), (((var_t1_dn4 * var_f10) + (var_t1 * var_f10_dn4)) - var_f11_dn4), (((var_t1_dn5 * var_f10) + (var_t1 * var_f10_dn5)) - var_f11_dn5), (((var_t1_dn6 * var_f10) + (var_t1 * var_f10_dn6)) - var_f11_dn6), (((var_t1_dn7 * var_f10) + (var_t1 * var_f10_dn7)) - var_f11_dn7), (((var_t1_dn8 * var_f10) + (var_t1 * var_f10_dn8)) - var_f11_dn8), (((var_t1_dn9 * var_f10) + (var_t1 * var_f10_dn9)) - var_f11_dn9), (((var_t1_dn10 * var_f10) + (var_t1 * var_f10_dn10)) - var_f11_dn10), (((var_t1_dn13 * var_f10) + (var_t1 * var_f10_dn13)) - var_f11_dn13),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign60780_e94878;
        var_t2_dn0 = assign60780_e94878_d_n0;
        var_t2_dn2 = assign60780_e94878_d_n2;
        var_t2_dn4 = assign60780_e94878_d_n4;
        var_t2_dn5 = assign60780_e94878_d_n5;
        var_t2_dn6 = assign60780_e94878_d_n6;
        var_t2_dn7 = assign60780_e94878_d_n7;
        var_t2_dn8 = assign60780_e94878_d_n8;
        var_t2_dn9 = assign60780_e94878_d_n9;
        var_t2_dn10 = assign60780_e94878_d_n10;
        var_t2_dn13 = assign60780_e94878_d_n13;
        var_t2_rv = 0.0;

        let (assign60790_e94905, assign60790_e94905_d_n0, assign60790_e94905_d_n2, assign60790_e94905_d_n4, assign60790_e94905_d_n5, assign60790_e94905_d_n6, assign60790_e94905_d_n7, assign60790_e94905_d_n8, assign60790_e94905_d_n9, assign60790_e94905_d_n10, assign60790_e94905_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign60790_e94890: f64 = (var_xi0 + 1.0);
        let assign60790_e94891: f64 = (1.5 - assign60790_e94890);
        let assign60790_e94894: f64 = (0.5 * var_beta);
        let assign60790_e94896: f64 = (assign60790_e94894 * var_pds);
        let assign60790_e94897: f64 = (assign60790_e94891 - assign60790_e94896);
        let assign60790_e94898: f64 = (var_cnst0 * assign60790_e94897);
        let assign60790_e94901: f64 = (var_cox * var_t2);
        let assign60790_e94902: f64 = (assign60790_e94898 + assign60790_e94901);
        let assign60790_e94903: f64 = (var_cnst0 * assign60790_e94902);
        (assign60790_e94903, ((var_cnst0_dn0 * assign60790_e94902) + (var_cnst0 * (((var_cnst0_dn0 * assign60790_e94897) + (var_cnst0 * ((-var_xi0_dn0) - (((0.5 * var_beta_dn0) * var_pds) + (assign60790_e94894 * var_pds_dn0))))) + ((var_cox_dn0 * var_t2) + (var_cox * var_t2_dn0))))), ((var_cnst0_dn2 * assign60790_e94902) + (var_cnst0 * (((var_cnst0_dn2 * assign60790_e94897) + (var_cnst0 * ((-var_xi0_dn2) - (((0.5 * var_beta_dn2) * var_pds) + (assign60790_e94894 * var_pds_dn2))))) + ((var_cox_dn2 * var_t2) + (var_cox * var_t2_dn2))))), ((var_cnst0_dn4 * assign60790_e94902) + (var_cnst0 * (((var_cnst0_dn4 * assign60790_e94897) + (var_cnst0 * ((-var_xi0_dn4) - (((0.5 * var_beta_dn4) * var_pds) + (assign60790_e94894 * var_pds_dn4))))) + ((var_cox_dn4 * var_t2) + (var_cox * var_t2_dn4))))), ((var_cnst0_dn5 * assign60790_e94902) + (var_cnst0 * (((var_cnst0_dn5 * assign60790_e94897) + (var_cnst0 * ((-var_xi0_dn5) - (((0.5 * var_beta_dn5) * var_pds) + (assign60790_e94894 * var_pds_dn5))))) + ((var_cox_dn5 * var_t2) + (var_cox * var_t2_dn5))))), ((var_cnst0_dn6 * assign60790_e94902) + (var_cnst0 * (((var_cnst0_dn6 * assign60790_e94897) + (var_cnst0 * ((-var_xi0_dn6) - (((0.5 * var_beta_dn6) * var_pds) + (assign60790_e94894 * var_pds_dn6))))) + ((var_cox_dn6 * var_t2) + (var_cox * var_t2_dn6))))), ((var_cnst0_dn7 * assign60790_e94902) + (var_cnst0 * (((var_cnst0_dn7 * assign60790_e94897) + (var_cnst0 * ((-var_xi0_dn7) - (((0.5 * var_beta_dn7) * var_pds) + (assign60790_e94894 * var_pds_dn7))))) + ((var_cox_dn7 * var_t2) + (var_cox * var_t2_dn7))))), ((var_cnst0_dn8 * assign60790_e94902) + (var_cnst0 * (((var_cnst0_dn8 * assign60790_e94897) + (var_cnst0 * ((-var_xi0_dn8) - (((0.5 * var_beta_dn8) * var_pds) + (assign60790_e94894 * var_pds_dn8))))) + ((var_cox_dn8 * var_t2) + (var_cox * var_t2_dn8))))), ((var_cnst0_dn9 * assign60790_e94902) + (var_cnst0 * (((var_cnst0_dn9 * assign60790_e94897) + (var_cnst0 * ((-var_xi0_dn9) - (((0.5 * var_beta_dn9) * var_pds) + (assign60790_e94894 * var_pds_dn9))))) + ((var_cox_dn9 * var_t2) + (var_cox * var_t2_dn9))))), ((var_cnst0_dn10 * assign60790_e94902) + (var_cnst0 * (((var_cnst0_dn10 * assign60790_e94897) + (var_cnst0 * ((-var_xi0_dn10) - (((0.5 * var_beta_dn10) * var_pds) + (assign60790_e94894 * var_pds_dn10))))) + ((var_cox_dn10 * var_t2) + (var_cox * var_t2_dn10))))), ((var_cnst0_dn13 * assign60790_e94902) + (var_cnst0 * (((var_cnst0_dn13 * assign60790_e94897) + (var_cnst0 * ((-var_xi0_dn13) - (((0.5 * var_beta_dn13) * var_pds) + (assign60790_e94894 * var_pds_dn13))))) + ((var_cox_dn13 * var_t2) + (var_cox * var_t2_dn13))))),)
    } else {
        (var_qbnm, var_qbnm_dn0, var_qbnm_dn2, var_qbnm_dn4, var_qbnm_dn5, var_qbnm_dn6, var_qbnm_dn7, var_qbnm_dn8, var_qbnm_dn9, var_qbnm_dn10, var_qbnm_dn13,)
    }
};
        var_qbnm = assign60790_e94905;
        var_qbnm_dn0 = assign60790_e94905_d_n0;
        var_qbnm_dn2 = assign60790_e94905_d_n2;
        var_qbnm_dn4 = assign60790_e94905_d_n4;
        var_qbnm_dn5 = assign60790_e94905_d_n5;
        var_qbnm_dn6 = assign60790_e94905_d_n6;
        var_qbnm_dn7 = assign60790_e94905_d_n7;
        var_qbnm_dn8 = assign60790_e94905_d_n8;
        var_qbnm_dn9 = assign60790_e94905_d_n9;
        var_qbnm_dn10 = assign60790_e94905_d_n10;
        var_qbnm_dn13 = assign60790_e94905_d_n13;
        var_qbnm_rv = 0.0;

        let (assign60800_e94914, assign60800_e94914_d_n0, assign60800_e94914_d_n2, assign60800_e94914_d_n4, assign60800_e94914_d_n5, assign60800_e94914_d_n6, assign60800_e94914_d_n7, assign60800_e94914_d_n8, assign60800_e94914_d_n9, assign60800_e94914_d_n10, assign60800_e94914_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        (var_beta, var_beta_dn0, var_beta_dn2, var_beta_dn4, var_beta_dn5, var_beta_dn6, var_beta_dn7, var_beta_dn8, var_beta_dn9, var_beta_dn10, var_beta_dn13,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign60800_e94914;
        var_t1_dn0 = assign60800_e94914_d_n0;
        var_t1_dn2 = assign60800_e94914_d_n2;
        var_t1_dn4 = assign60800_e94914_d_n4;
        var_t1_dn5 = assign60800_e94914_d_n5;
        var_t1_dn6 = assign60800_e94914_d_n6;
        var_t1_dn7 = assign60800_e94914_d_n7;
        var_t1_dn8 = assign60800_e94914_d_n8;
        var_t1_dn9 = assign60800_e94914_d_n9;
        var_t1_dn10 = assign60800_e94914_d_n10;
        var_t1_dn13 = assign60800_e94914_d_n13;
        var_t1_rv = 0.0;

        let (assign60810_e94927, assign60810_e94927_d_n0, assign60810_e94927_d_n2, assign60810_e94927_d_n4, assign60810_e94927_d_n5, assign60810_e94927_d_n6, assign60810_e94927_d_n7, assign60810_e94927_d_n8, assign60810_e94927_d_n9, assign60810_e94927_d_n10, assign60810_e94927_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign60810_e94923: f64 = (var_t1 * var_qbnm);
        let assign60810_e94925: f64 = (assign60810_e94923 / var_fdd);
        (assign60810_e94925, (((((var_t1_dn0 * var_qbnm) + (var_t1 * var_qbnm_dn0)) * var_fdd) - (assign60810_e94923 * var_fdd_dn0)) / (var_fdd * var_fdd)), (((((var_t1_dn2 * var_qbnm) + (var_t1 * var_qbnm_dn2)) * var_fdd) - (assign60810_e94923 * var_fdd_dn2)) / (var_fdd * var_fdd)), (((((var_t1_dn4 * var_qbnm) + (var_t1 * var_qbnm_dn4)) * var_fdd) - (assign60810_e94923 * var_fdd_dn4)) / (var_fdd * var_fdd)), (((((var_t1_dn5 * var_qbnm) + (var_t1 * var_qbnm_dn5)) * var_fdd) - (assign60810_e94923 * var_fdd_dn5)) / (var_fdd * var_fdd)), (((((var_t1_dn6 * var_qbnm) + (var_t1 * var_qbnm_dn6)) * var_fdd) - (assign60810_e94923 * var_fdd_dn6)) / (var_fdd * var_fdd)), (((((var_t1_dn7 * var_qbnm) + (var_t1 * var_qbnm_dn7)) * var_fdd) - (assign60810_e94923 * var_fdd_dn7)) / (var_fdd * var_fdd)), (((((var_t1_dn8 * var_qbnm) + (var_t1 * var_qbnm_dn8)) * var_fdd) - (assign60810_e94923 * var_fdd_dn8)) / (var_fdd * var_fdd)), (((((var_t1_dn9 * var_qbnm) + (var_t1 * var_qbnm_dn9)) * var_fdd) - (assign60810_e94923 * var_fdd_dn9)) / (var_fdd * var_fdd)), (((((var_t1_dn10 * var_qbnm) + (var_t1 * var_qbnm_dn10)) * var_fdd) - (assign60810_e94923 * var_fdd_dn10)) / (var_fdd * var_fdd)), (((((var_t1_dn13 * var_qbnm) + (var_t1 * var_qbnm_dn13)) * var_fdd) - (assign60810_e94923 * var_fdd_dn13)) / (var_fdd * var_fdd)),)
    } else {
        (var_qbu, var_qbu_dn0, var_qbu_dn2, var_qbu_dn4, var_qbu_dn5, var_qbu_dn6, var_qbu_dn7, var_qbu_dn8, var_qbu_dn9, var_qbu_dn10, var_qbu_dn13,)
    }
};
        var_qbu = assign60810_e94927;
        var_qbu_dn0 = assign60810_e94927_d_n0;
        var_qbu_dn2 = assign60810_e94927_d_n2;
        var_qbu_dn4 = assign60810_e94927_d_n4;
        var_qbu_dn5 = assign60810_e94927_d_n5;
        var_qbu_dn6 = assign60810_e94927_d_n6;
        var_qbu_dn7 = assign60810_e94927_d_n7;
        var_qbu_dn8 = assign60810_e94927_d_n8;
        var_qbu_dn9 = assign60810_e94927_d_n9;
        var_qbu_dn10 = assign60810_e94927_d_n10;
        var_qbu_dn13 = assign60810_e94927_d_n13;
        var_qbu_rv = 0.0;

        let (assign60820_e94938, assign60820_e94938_d_n0, assign60820_e94938_d_n2, assign60820_e94938_d_n4, assign60820_e94938_d_n5, assign60820_e94938_d_n6, assign60820_e94938_d_n7, assign60820_e94938_d_n8, assign60820_e94938_d_n9, assign60820_e94938_d_n10, assign60820_e94938_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign60820_e94936: f64 = (2.0 * var_fac1);
        (assign60820_e94936, (2.0 * var_fac1_dn0), (2.0 * var_fac1_dn2), (2.0 * var_fac1_dn4), (2.0 * var_fac1_dn5), (2.0 * var_fac1_dn6), (2.0 * var_fac1_dn7), (2.0 * var_fac1_dn8), (2.0 * var_fac1_dn9), (2.0 * var_fac1_dn10), (2.0 * var_fac1_dn13),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign60820_e94938;
        var_t1_dn0 = assign60820_e94938_d_n0;
        var_t1_dn2 = assign60820_e94938_d_n2;
        var_t1_dn4 = assign60820_e94938_d_n4;
        var_t1_dn5 = assign60820_e94938_d_n5;
        var_t1_dn6 = assign60820_e94938_d_n6;
        var_t1_dn7 = assign60820_e94938_d_n7;
        var_t1_dn8 = assign60820_e94938_d_n8;
        var_t1_dn9 = assign60820_e94938_d_n9;
        var_t1_dn10 = assign60820_e94938_d_n10;
        var_t1_dn13 = assign60820_e94938_d_n13;
        var_t1_rv = 0.0;

        let (assign60830_e94951, assign60830_e94951_d_n0, assign60830_e94951_d_n2, assign60830_e94951_d_n4, assign60830_e94951_d_n5, assign60830_e94951_d_n6, assign60830_e94951_d_n7, assign60830_e94951_d_n8, assign60830_e94951_d_n9, assign60830_e94951_d_n10, assign60830_e94951_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign60830_e94948: f64 = (var_f10 - var_xi0p12);
        let assign60830_e94949: f64 = (var_t1 * assign60830_e94948);
        (assign60830_e94949, ((var_t1_dn0 * assign60830_e94948) + (var_t1 * (var_f10_dn0 - var_xi0p12_dn0))), ((var_t1_dn2 * assign60830_e94948) + (var_t1 * (var_f10_dn2 - var_xi0p12_dn2))), ((var_t1_dn4 * assign60830_e94948) + (var_t1 * (var_f10_dn4 - var_xi0p12_dn4))), ((var_t1_dn5 * assign60830_e94948) + (var_t1 * (var_f10_dn5 - var_xi0p12_dn5))), ((var_t1_dn6 * assign60830_e94948) + (var_t1 * (var_f10_dn6 - var_xi0p12_dn6))), ((var_t1_dn7 * assign60830_e94948) + (var_t1 * (var_f10_dn7 - var_xi0p12_dn7))), ((var_t1_dn8 * assign60830_e94948) + (var_t1 * (var_f10_dn8 - var_xi0p12_dn8))), ((var_t1_dn9 * assign60830_e94948) + (var_t1 * (var_f10_dn9 - var_xi0p12_dn9))), ((var_t1_dn10 * assign60830_e94948) + (var_t1 * (var_f10_dn10 - var_xi0p12_dn10))), ((var_t1_dn13 * assign60830_e94948) + (var_t1 * (var_f10_dn13 - var_xi0p12_dn13))),)
    } else {
        (var_dtpds, var_dtpds_dn0, var_dtpds_dn2, var_dtpds_dn4, var_dtpds_dn5, var_dtpds_dn6, var_dtpds_dn7, var_dtpds_dn8, var_dtpds_dn9, var_dtpds_dn10, var_dtpds_dn13,)
    }
};
        var_dtpds = assign60830_e94951;
        var_dtpds_dn0 = assign60830_e94951_d_n0;
        var_dtpds_dn2 = assign60830_e94951_d_n2;
        var_dtpds_dn4 = assign60830_e94951_d_n4;
        var_dtpds_dn5 = assign60830_e94951_d_n5;
        var_dtpds_dn6 = assign60830_e94951_d_n6;
        var_dtpds_dn7 = assign60830_e94951_d_n7;
        var_dtpds_dn8 = assign60830_e94951_d_n8;
        var_dtpds_dn9 = assign60830_e94951_d_n9;
        var_dtpds_dn10 = assign60830_e94951_d_n10;
        var_dtpds_dn13 = assign60830_e94951_d_n13;
        var_dtpds_rv = 0.0;

        let (assign60840_e94964, assign60840_e94964_d_n0, assign60840_e94964_d_n2, assign60840_e94964_d_n4, assign60840_e94964_d_n5, assign60840_e94964_d_n6, assign60840_e94964_d_n7, assign60840_e94964_d_n8, assign60840_e94964_d_n9, assign60840_e94964_d_n10, assign60840_e94964_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign60840_e94961: f64 = (var_f10 - var_xi0p12);
        let assign60840_e94962: f64 = (2.0 * assign60840_e94961);
        (assign60840_e94962, (2.0 * (var_f10_dn0 - var_xi0p12_dn0)), (2.0 * (var_f10_dn2 - var_xi0p12_dn2)), (2.0 * (var_f10_dn4 - var_xi0p12_dn4)), (2.0 * (var_f10_dn5 - var_xi0p12_dn5)), (2.0 * (var_f10_dn6 - var_xi0p12_dn6)), (2.0 * (var_f10_dn7 - var_xi0p12_dn7)), (2.0 * (var_f10_dn8 - var_xi0p12_dn8)), (2.0 * (var_f10_dn9 - var_xi0p12_dn9)), (2.0 * (var_f10_dn10 - var_xi0p12_dn10)), (2.0 * (var_f10_dn13 - var_xi0p12_dn13)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign60840_e94964;
        var_t2_dn0 = assign60840_e94964_d_n0;
        var_t2_dn2 = assign60840_e94964_d_n2;
        var_t2_dn4 = assign60840_e94964_d_n4;
        var_t2_dn5 = assign60840_e94964_d_n5;
        var_t2_dn6 = assign60840_e94964_d_n6;
        var_t2_dn7 = assign60840_e94964_d_n7;
        var_t2_dn8 = assign60840_e94964_d_n8;
        var_t2_dn9 = assign60840_e94964_d_n9;
        var_t2_dn10 = assign60840_e94964_d_n10;
        var_t2_dn13 = assign60840_e94964_d_n13;
        var_t2_rv = 0.0;

        let (assign60850_e94975, assign60850_e94975_d_n0, assign60850_e94975_d_n2, assign60850_e94975_d_n4, assign60850_e94975_d_n5, assign60850_e94975_d_n6, assign60850_e94975_d_n7, assign60850_e94975_d_n8, assign60850_e94975_d_n9, assign60850_e94975_d_n10, assign60850_e94975_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign60850_e94973: f64 = (var_pds + var_dtpds);
        (assign60850_e94973, (var_pds_dn0 + var_dtpds_dn0), (var_pds_dn2 + var_dtpds_dn2), (var_pds_dn4 + var_dtpds_dn4), (var_pds_dn5 + var_dtpds_dn5), (var_pds_dn6 + var_dtpds_dn6), (var_pds_dn7 + var_dtpds_dn7), (var_pds_dn8 + var_dtpds_dn8), (var_pds_dn9 + var_dtpds_dn9), (var_pds_dn10 + var_dtpds_dn10), (var_pds_dn13 + var_dtpds_dn13),)
    } else {
        (var_achi, var_achi_dn0, var_achi_dn2, var_achi_dn4, var_achi_dn5, var_achi_dn6, var_achi_dn7, var_achi_dn8, var_achi_dn9, var_achi_dn10, var_achi_dn13,)
    }
};
        var_achi = assign60850_e94975;
        var_achi_dn0 = assign60850_e94975_d_n0;
        var_achi_dn2 = assign60850_e94975_d_n2;
        var_achi_dn4 = assign60850_e94975_d_n4;
        var_achi_dn5 = assign60850_e94975_d_n5;
        var_achi_dn6 = assign60850_e94975_d_n6;
        var_achi_dn7 = assign60850_e94975_d_n7;
        var_achi_dn8 = assign60850_e94975_d_n8;
        var_achi_dn9 = assign60850_e94975_d_n9;
        var_achi_dn10 = assign60850_e94975_d_n10;
        var_achi_dn13 = assign60850_e94975_d_n13;
        var_achi_rv = 0.0;

        let (assign60860_e94986, assign60860_e94986_d_n0, assign60860_e94986_d_n2, assign60860_e94986_d_n4, assign60860_e94986_d_n5, assign60860_e94986_d_n6, assign60860_e94986_d_n7, assign60860_e94986_d_n8, assign60860_e94986_d_n9, assign60860_e94986_d_n10, assign60860_e94986_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign60860_e94984: f64 = (1.0 / var_vgvt);
        (assign60860_e94984, (-(var_vgvt_dn0 / (var_vgvt * var_vgvt))), (-(var_vgvt_dn2 / (var_vgvt * var_vgvt))), (-(var_vgvt_dn4 / (var_vgvt * var_vgvt))), (-(var_vgvt_dn5 / (var_vgvt * var_vgvt))), (-(var_vgvt_dn6 / (var_vgvt * var_vgvt))), (-(var_vgvt_dn7 / (var_vgvt * var_vgvt))), (-(var_vgvt_dn8 / (var_vgvt * var_vgvt))), (-(var_vgvt_dn9 / (var_vgvt * var_vgvt))), (-(var_vgvt_dn10 / (var_vgvt * var_vgvt))), (-(var_vgvt_dn13 / (var_vgvt * var_vgvt))),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign60860_e94986;
        var_t1_dn0 = assign60860_e94986_d_n0;
        var_t1_dn2 = assign60860_e94986_d_n2;
        var_t1_dn4 = assign60860_e94986_d_n4;
        var_t1_dn5 = assign60860_e94986_d_n5;
        var_t1_dn6 = assign60860_e94986_d_n6;
        var_t1_dn7 = assign60860_e94986_d_n7;
        var_t1_dn8 = assign60860_e94986_d_n8;
        var_t1_dn9 = assign60860_e94986_d_n9;
        var_t1_dn10 = assign60860_e94986_d_n10;
        var_t1_dn13 = assign60860_e94986_d_n13;
        var_t1_rv = 0.0;

        let (assign60870_e94997, assign60870_e94997_d_n0, assign60870_e94997_d_n2, assign60870_e94997_d_n4, assign60870_e94997_d_n5, assign60870_e94997_d_n6, assign60870_e94997_d_n7, assign60870_e94997_d_n8, assign60870_e94997_d_n9, assign60870_e94997_d_n10, assign60870_e94997_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign60870_e94995: f64 = (var_achi * var_t1);
        (assign60870_e94995, ((var_achi_dn0 * var_t1) + (var_achi * var_t1_dn0)), ((var_achi_dn2 * var_t1) + (var_achi * var_t1_dn2)), ((var_achi_dn4 * var_t1) + (var_achi * var_t1_dn4)), ((var_achi_dn5 * var_t1) + (var_achi * var_t1_dn5)), ((var_achi_dn6 * var_t1) + (var_achi * var_t1_dn6)), ((var_achi_dn7 * var_t1) + (var_achi * var_t1_dn7)), ((var_achi_dn8 * var_t1) + (var_achi * var_t1_dn8)), ((var_achi_dn9 * var_t1) + (var_achi * var_t1_dn9)), ((var_achi_dn10 * var_t1) + (var_achi * var_t1_dn10)), ((var_achi_dn13 * var_t1) + (var_achi * var_t1_dn13)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign60870_e94997;
        var_t2_dn0 = assign60870_e94997_d_n0;
        var_t2_dn2 = assign60870_e94997_d_n2;
        var_t2_dn4 = assign60870_e94997_d_n4;
        var_t2_dn5 = assign60870_e94997_d_n5;
        var_t2_dn6 = assign60870_e94997_d_n6;
        var_t2_dn7 = assign60870_e94997_d_n7;
        var_t2_dn8 = assign60870_e94997_d_n8;
        var_t2_dn9 = assign60870_e94997_d_n9;
        var_t2_dn10 = assign60870_e94997_d_n10;
        var_t2_dn13 = assign60870_e94997_d_n13;
        var_t2_rv = 0.0;

        let (assign60880_e95008, assign60880_e95008_d_n0, assign60880_e95008_d_n2, assign60880_e95008_d_n4, assign60880_e95008_d_n5, assign60880_e95008_d_n6, assign60880_e95008_d_n7, assign60880_e95008_d_n8, assign60880_e95008_d_n9, assign60880_e95008_d_n10, assign60880_e95008_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign60880_e95006: f64 = (1.0 - var_t2);
        (assign60880_e95006, (-var_t2_dn0), (-var_t2_dn2), (-var_t2_dn4), (-var_t2_dn5), (-var_t2_dn6), (-var_t2_dn7), (-var_t2_dn8), (-var_t2_dn9), (-var_t2_dn10), (-var_t2_dn13),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn13,)
    }
};
        var_t3 = assign60880_e95008;
        var_t3_dn0 = assign60880_e95008_d_n0;
        var_t3_dn2 = assign60880_e95008_d_n2;
        var_t3_dn4 = assign60880_e95008_d_n4;
        var_t3_dn5 = assign60880_e95008_d_n5;
        var_t3_dn6 = assign60880_e95008_d_n6;
        var_t3_dn7 = assign60880_e95008_d_n7;
        var_t3_dn8 = assign60880_e95008_d_n8;
        var_t3_dn9 = assign60880_e95008_d_n9;
        var_t3_dn10 = assign60880_e95008_d_n10;
        var_t3_dn13 = assign60880_e95008_d_n13;
        var_t3_rv = 0.0;

        let (assign60890_e95019, assign60890_e95019_d_n0, assign60890_e95019_d_n2, assign60890_e95019_d_n4, assign60890_e95019_d_n5, assign60890_e95019_d_n6, assign60890_e95019_d_n7, assign60890_e95019_d_n8, assign60890_e95019_d_n9, assign60890_e95019_d_n10, assign60890_e95019_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign60890_e95017: f64 = (1.0 - var_t3);
        (assign60890_e95017, (-var_t3_dn0), (-var_t3_dn2), (-var_t3_dn4), (-var_t3_dn5), (-var_t3_dn6), (-var_t3_dn7), (-var_t3_dn8), (-var_t3_dn9), (-var_t3_dn10), (-var_t3_dn13),)
    } else {
        (var_tx, var_tx_dn0, var_tx_dn2, var_tx_dn4, var_tx_dn5, var_tx_dn6, var_tx_dn7, var_tx_dn8, var_tx_dn9, var_tx_dn10, var_tx_dn13,)
    }
};
        var_tx = assign60890_e95019;
        var_tx_dn0 = assign60890_e95019_d_n0;
        var_tx_dn2 = assign60890_e95019_d_n2;
        var_tx_dn4 = assign60890_e95019_d_n4;
        var_tx_dn5 = assign60890_e95019_d_n5;
        var_tx_dn6 = assign60890_e95019_d_n6;
        var_tx_dn7 = assign60890_e95019_d_n7;
        var_tx_dn8 = assign60890_e95019_d_n8;
        var_tx_dn9 = assign60890_e95019_d_n9;
        var_tx_dn10 = assign60890_e95019_d_n10;
        var_tx_dn13 = assign60890_e95019_d_n13;
        var_tx_rv = 0.0;

        let (assign60900_e95030, assign60900_e95030_d_n0, assign60900_e95030_d_n2, assign60900_e95030_d_n4, assign60900_e95030_d_n5, assign60900_e95030_d_n6, assign60900_e95030_d_n7, assign60900_e95030_d_n8, assign60900_e95030_d_n9, assign60900_e95030_d_n10, assign60900_e95030_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign60900_e95028: f64 = (var_tx * var_tx);
        (assign60900_e95028, ((var_tx_dn0 * var_tx) + (var_tx * var_tx_dn0)), ((var_tx_dn2 * var_tx) + (var_tx * var_tx_dn2)), ((var_tx_dn4 * var_tx) + (var_tx * var_tx_dn4)), ((var_tx_dn5 * var_tx) + (var_tx * var_tx_dn5)), ((var_tx_dn6 * var_tx) + (var_tx * var_tx_dn6)), ((var_tx_dn7 * var_tx) + (var_tx * var_tx_dn7)), ((var_tx_dn8 * var_tx) + (var_tx * var_tx_dn8)), ((var_tx_dn9 * var_tx) + (var_tx * var_tx_dn9)), ((var_tx_dn10 * var_tx) + (var_tx * var_tx_dn10)), ((var_tx_dn13 * var_tx) + (var_tx * var_tx_dn13)),)
    } else {
        (var_x2, var_x2_dn0, var_x2_dn2, var_x2_dn4, var_x2_dn5, var_x2_dn6, var_x2_dn7, var_x2_dn8, var_x2_dn9, var_x2_dn10, var_x2_dn13,)
    }
};
        var_x2 = assign60900_e95030;
        var_x2_dn0 = assign60900_e95030_d_n0;
        var_x2_dn2 = assign60900_e95030_d_n2;
        var_x2_dn4 = assign60900_e95030_d_n4;
        var_x2_dn5 = assign60900_e95030_d_n5;
        var_x2_dn6 = assign60900_e95030_d_n6;
        var_x2_dn7 = assign60900_e95030_d_n7;
        var_x2_dn8 = assign60900_e95030_d_n8;
        var_x2_dn9 = assign60900_e95030_d_n9;
        var_x2_dn10 = assign60900_e95030_d_n10;
        var_x2_dn13 = assign60900_e95030_d_n13;
        var_x2_rv = 0.0;

        let (assign60910_e95041, assign60910_e95041_d_n0, assign60910_e95041_d_n2, assign60910_e95041_d_n4, assign60910_e95041_d_n5, assign60910_e95041_d_n6, assign60910_e95041_d_n7, assign60910_e95041_d_n8, assign60910_e95041_d_n9, assign60910_e95041_d_n10, assign60910_e95041_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign60910_e95039: f64 = 1.0;
        (assign60910_e95039, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmax2, var_xmax2_dn0, var_xmax2_dn2, var_xmax2_dn4, var_xmax2_dn5, var_xmax2_dn6, var_xmax2_dn7, var_xmax2_dn8, var_xmax2_dn9, var_xmax2_dn10, var_xmax2_dn13,)
    }
};
        var_xmax2 = assign60910_e95041;
        var_xmax2_dn0 = assign60910_e95041_d_n0;
        var_xmax2_dn2 = assign60910_e95041_d_n2;
        var_xmax2_dn4 = assign60910_e95041_d_n4;
        var_xmax2_dn5 = assign60910_e95041_d_n5;
        var_xmax2_dn6 = assign60910_e95041_d_n6;
        var_xmax2_dn7 = assign60910_e95041_d_n7;
        var_xmax2_dn8 = assign60910_e95041_d_n8;
        var_xmax2_dn9 = assign60910_e95041_d_n9;
        var_xmax2_dn10 = assign60910_e95041_d_n10;
        var_xmax2_dn13 = assign60910_e95041_d_n13;
        var_xmax2_rv = 0.0;

        let (assign60920_e95050, assign60920_e95050_d_n0, assign60920_e95050_d_n2, assign60920_e95050_d_n4, assign60920_e95050_d_n5, assign60920_e95050_d_n6, assign60920_e95050_d_n7, assign60920_e95050_d_n8, assign60920_e95050_d_n9, assign60920_e95050_d_n10, assign60920_e95050_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn7, var_xp_dn8, var_xp_dn9, var_xp_dn10, var_xp_dn13,)
    }
};
        var_xp = assign60920_e95050;
        var_xp_dn0 = assign60920_e95050_d_n0;
        var_xp_dn2 = assign60920_e95050_d_n2;
        var_xp_dn4 = assign60920_e95050_d_n4;
        var_xp_dn5 = assign60920_e95050_d_n5;
        var_xp_dn6 = assign60920_e95050_d_n6;
        var_xp_dn7 = assign60920_e95050_d_n7;
        var_xp_dn8 = assign60920_e95050_d_n8;
        var_xp_dn9 = assign60920_e95050_d_n9;
        var_xp_dn10 = assign60920_e95050_d_n10;
        var_xp_dn13 = assign60920_e95050_d_n13;
        var_xp_rv = 0.0;

        let (assign60930_e95059, assign60930_e95059_d_n0, assign60930_e95059_d_n2, assign60930_e95059_d_n4, assign60930_e95059_d_n5, assign60930_e95059_d_n6, assign60930_e95059_d_n7, assign60930_e95059_d_n8, assign60930_e95059_d_n9, assign60930_e95059_d_n10, assign60930_e95059_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn7, var_xmp_dn8, var_xmp_dn9, var_xmp_dn10, var_xmp_dn13,)
    }
};
        var_xmp = assign60930_e95059;
        var_xmp_dn0 = assign60930_e95059_d_n0;
        var_xmp_dn2 = assign60930_e95059_d_n2;
        var_xmp_dn4 = assign60930_e95059_d_n4;
        var_xmp_dn5 = assign60930_e95059_d_n5;
        var_xmp_dn6 = assign60930_e95059_d_n6;
        var_xmp_dn7 = assign60930_e95059_d_n7;
        var_xmp_dn8 = assign60930_e95059_d_n8;
        var_xmp_dn9 = assign60930_e95059_d_n9;
        var_xmp_dn10 = assign60930_e95059_d_n10;
        var_xmp_dn13 = assign60930_e95059_d_n13;
        var_xmp_rv = 0.0;

        let (assign60940_e95068,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign60940_e95068;
        var_m0_rv = 0.0;

        *var_achi_slot = var_achi;
        *var_achi_dn0_slot = var_achi_dn0;
        *var_achi_dn10_slot = var_achi_dn10;
        *var_achi_dn13_slot = var_achi_dn13;
        *var_achi_dn2_slot = var_achi_dn2;
        *var_achi_dn4_slot = var_achi_dn4;
        *var_achi_dn5_slot = var_achi_dn5;
        *var_achi_dn6_slot = var_achi_dn6;
        *var_achi_dn7_slot = var_achi_dn7;
        *var_achi_dn8_slot = var_achi_dn8;
        *var_achi_dn9_slot = var_achi_dn9;
        *var_achi_rv_slot = var_achi_rv;
        *var_dtpds_slot = var_dtpds;
        *var_dtpds_dn0_slot = var_dtpds_dn0;
        *var_dtpds_dn10_slot = var_dtpds_dn10;
        *var_dtpds_dn13_slot = var_dtpds_dn13;
        *var_dtpds_dn2_slot = var_dtpds_dn2;
        *var_dtpds_dn4_slot = var_dtpds_dn4;
        *var_dtpds_dn5_slot = var_dtpds_dn5;
        *var_dtpds_dn6_slot = var_dtpds_dn6;
        *var_dtpds_dn7_slot = var_dtpds_dn7;
        *var_dtpds_dn8_slot = var_dtpds_dn8;
        *var_dtpds_dn9_slot = var_dtpds_dn9;
        *var_dtpds_rv_slot = var_dtpds_rv;
        *var_lred_slot = var_lred;
        *var_lred_dn0_slot = var_lred_dn0;
        *var_lred_dn10_slot = var_lred_dn10;
        *var_lred_dn13_slot = var_lred_dn13;
        *var_lred_dn2_slot = var_lred_dn2;
        *var_lred_dn4_slot = var_lred_dn4;
        *var_lred_dn5_slot = var_lred_dn5;
        *var_lred_dn6_slot = var_lred_dn6;
        *var_lred_dn7_slot = var_lred_dn7;
        *var_lred_dn8_slot = var_lred_dn8;
        *var_lred_dn9_slot = var_lred_dn9;
        *var_lred_rv_slot = var_lred_rv;
        *var_m0_slot = var_m0;
        *var_m0_rv_slot = var_m0_rv;
        *var_qbnm_slot = var_qbnm;
        *var_qbnm_dn0_slot = var_qbnm_dn0;
        *var_qbnm_dn10_slot = var_qbnm_dn10;
        *var_qbnm_dn13_slot = var_qbnm_dn13;
        *var_qbnm_dn2_slot = var_qbnm_dn2;
        *var_qbnm_dn4_slot = var_qbnm_dn4;
        *var_qbnm_dn5_slot = var_qbnm_dn5;
        *var_qbnm_dn6_slot = var_qbnm_dn6;
        *var_qbnm_dn7_slot = var_qbnm_dn7;
        *var_qbnm_dn8_slot = var_qbnm_dn8;
        *var_qbnm_dn9_slot = var_qbnm_dn9;
        *var_qbnm_rv_slot = var_qbnm_rv;
        *var_qbu_slot = var_qbu;
        *var_qbu_dn0_slot = var_qbu_dn0;
        *var_qbu_dn10_slot = var_qbu_dn10;
        *var_qbu_dn13_slot = var_qbu_dn13;
        *var_qbu_dn2_slot = var_qbu_dn2;
        *var_qbu_dn4_slot = var_qbu_dn4;
        *var_qbu_dn5_slot = var_qbu_dn5;
        *var_qbu_dn6_slot = var_qbu_dn6;
        *var_qbu_dn7_slot = var_qbu_dn7;
        *var_qbu_dn8_slot = var_qbu_dn8;
        *var_qbu_dn9_slot = var_qbu_dn9;
        *var_qbu_rv_slot = var_qbu_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn13_slot = var_t1_dn13;
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
        *var_t2_dn13_slot = var_t2_dn13;
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
        *var_t3_dn13_slot = var_t3_dn13;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t3_rv_slot = var_t3_rv;
        *var_t8_slot = var_t8;
        *var_t8_dn0_slot = var_t8_dn0;
        *var_t8_dn10_slot = var_t8_dn10;
        *var_t8_dn13_slot = var_t8_dn13;
        *var_t8_dn2_slot = var_t8_dn2;
        *var_t8_dn4_slot = var_t8_dn4;
        *var_t8_dn5_slot = var_t8_dn5;
        *var_t8_dn6_slot = var_t8_dn6;
        *var_t8_dn7_slot = var_t8_dn7;
        *var_t8_dn8_slot = var_t8_dn8;
        *var_t8_dn9_slot = var_t8_dn9;
        *var_t8_rv_slot = var_t8_rv;
        *var_t9_slot = var_t9;
        *var_t9_dn0_slot = var_t9_dn0;
        *var_t9_dn10_slot = var_t9_dn10;
        *var_t9_dn13_slot = var_t9_dn13;
        *var_t9_dn2_slot = var_t9_dn2;
        *var_t9_dn4_slot = var_t9_dn4;
        *var_t9_dn5_slot = var_t9_dn5;
        *var_t9_dn6_slot = var_t9_dn6;
        *var_t9_dn7_slot = var_t9_dn7;
        *var_t9_dn8_slot = var_t9_dn8;
        *var_t9_dn9_slot = var_t9_dn9;
        *var_t9_rv_slot = var_t9_rv;
        *var_tx_slot = var_tx;
        *var_tx_dn0_slot = var_tx_dn0;
        *var_tx_dn10_slot = var_tx_dn10;
        *var_tx_dn13_slot = var_tx_dn13;
        *var_tx_dn2_slot = var_tx_dn2;
        *var_tx_dn4_slot = var_tx_dn4;
        *var_tx_dn5_slot = var_tx_dn5;
        *var_tx_dn6_slot = var_tx_dn6;
        *var_tx_dn7_slot = var_tx_dn7;
        *var_tx_dn8_slot = var_tx_dn8;
        *var_tx_dn9_slot = var_tx_dn9;
        *var_tx_rv_slot = var_tx_rv;
        *var_x2_slot = var_x2;
        *var_x2_dn0_slot = var_x2_dn0;
        *var_x2_dn10_slot = var_x2_dn10;
        *var_x2_dn13_slot = var_x2_dn13;
        *var_x2_dn2_slot = var_x2_dn2;
        *var_x2_dn4_slot = var_x2_dn4;
        *var_x2_dn5_slot = var_x2_dn5;
        *var_x2_dn6_slot = var_x2_dn6;
        *var_x2_dn7_slot = var_x2_dn7;
        *var_x2_dn8_slot = var_x2_dn8;
        *var_x2_dn9_slot = var_x2_dn9;
        *var_x2_rv_slot = var_x2_rv;
        *var_xmax2_slot = var_xmax2;
        *var_xmax2_dn0_slot = var_xmax2_dn0;
        *var_xmax2_dn10_slot = var_xmax2_dn10;
        *var_xmax2_dn13_slot = var_xmax2_dn13;
        *var_xmax2_dn2_slot = var_xmax2_dn2;
        *var_xmax2_dn4_slot = var_xmax2_dn4;
        *var_xmax2_dn5_slot = var_xmax2_dn5;
        *var_xmax2_dn6_slot = var_xmax2_dn6;
        *var_xmax2_dn7_slot = var_xmax2_dn7;
        *var_xmax2_dn8_slot = var_xmax2_dn8;
        *var_xmax2_dn9_slot = var_xmax2_dn9;
        *var_xmax2_rv_slot = var_xmax2_rv;
        *var_xmp_slot = var_xmp;
        *var_xmp_dn0_slot = var_xmp_dn0;
        *var_xmp_dn10_slot = var_xmp_dn10;
        *var_xmp_dn13_slot = var_xmp_dn13;
        *var_xmp_dn2_slot = var_xmp_dn2;
        *var_xmp_dn4_slot = var_xmp_dn4;
        *var_xmp_dn5_slot = var_xmp_dn5;
        *var_xmp_dn6_slot = var_xmp_dn6;
        *var_xmp_dn7_slot = var_xmp_dn7;
        *var_xmp_dn8_slot = var_xmp_dn8;
        *var_xmp_dn9_slot = var_xmp_dn9;
        *var_xmp_rv_slot = var_xmp_rv;
        *var_xp_slot = var_xp;
        *var_xp_dn0_slot = var_xp_dn0;
        *var_xp_dn10_slot = var_xp_dn10;
        *var_xp_dn13_slot = var_xp_dn13;
        *var_xp_dn2_slot = var_xp_dn2;
        *var_xp_dn4_slot = var_xp_dn4;
        *var_xp_dn5_slot = var_xp_dn5;
        *var_xp_dn6_slot = var_xp_dn6;
        *var_xp_dn7_slot = var_xp_dn7;
        *var_xp_dn8_slot = var_xp_dn8;
        *var_xp_dn9_slot = var_xp_dn9;
        *var_xp_rv_slot = var_xp_rv;
    }

    pub(super) fn stamp_reactive_block_218(
        var_guard1430: f64,
        var_guard1461: f64,
        var_guard443: f64,
        var_tx: f64,
        var_tx_dn0: f64,
        var_tx_dn10: f64,
        var_tx_dn13: f64,
        var_tx_dn2: f64,
        var_tx_dn4: f64,
        var_tx_dn5: f64,
        var_tx_dn6: f64,
        var_tx_dn7: f64,
        var_tx_dn8: f64,
        var_tx_dn9: f64,
        var_x2: f64,
        var_x2_dn0: f64,
        var_x2_dn10: f64,
        var_x2_dn13: f64,
        var_x2_dn2: f64,
        var_x2_dn4: f64,
        var_x2_dn5: f64,
        var_x2_dn6: f64,
        var_x2_dn7: f64,
        var_x2_dn8: f64,
        var_x2_dn9: f64,
        var_xmax2: f64,
        var_xmax2_dn0: f64,
        var_xmax2_dn10: f64,
        var_xmax2_dn13: f64,
        var_xmax2_dn2: f64,
        var_xmax2_dn4: f64,
        var_xmax2_dn5: f64,
        var_xmax2_dn6: f64,
        var_xmax2_dn7: f64,
        var_xmax2_dn8: f64,
        var_xmax2_dn9: f64,
        var_alpha_slot: &mut f64,
        var_alpha_dn0_slot: &mut f64,
        var_alpha_dn10_slot: &mut f64,
        var_alpha_dn13_slot: &mut f64,
        var_alpha_dn2_slot: &mut f64,
        var_alpha_dn4_slot: &mut f64,
        var_alpha_dn5_slot: &mut f64,
        var_alpha_dn6_slot: &mut f64,
        var_alpha_dn7_slot: &mut f64,
        var_alpha_dn8_slot: &mut f64,
        var_alpha_dn9_slot: &mut f64,
        var_alpha_rv_slot: &mut f64,
        var_arg_slot: &mut f64,
        var_arg_dn0_slot: &mut f64,
        var_arg_dn10_slot: &mut f64,
        var_arg_dn13_slot: &mut f64,
        var_arg_dn2_slot: &mut f64,
        var_arg_dn4_slot: &mut f64,
        var_arg_dn5_slot: &mut f64,
        var_arg_dn6_slot: &mut f64,
        var_arg_dn7_slot: &mut f64,
        var_arg_dn8_slot: &mut f64,
        var_arg_dn9_slot: &mut f64,
        var_arg_rv_slot: &mut f64,
        var_dnm_slot: &mut f64,
        var_dnm_dn0_slot: &mut f64,
        var_dnm_dn10_slot: &mut f64,
        var_dnm_dn13_slot: &mut f64,
        var_dnm_dn2_slot: &mut f64,
        var_dnm_dn4_slot: &mut f64,
        var_dnm_dn5_slot: &mut f64,
        var_dnm_dn6_slot: &mut f64,
        var_dnm_dn7_slot: &mut f64,
        var_dnm_dn8_slot: &mut f64,
        var_dnm_dn9_slot: &mut f64,
        var_dnm_rv_slot: &mut f64,
        var_guard1475_slot: &mut f64,
        var_guard1475_rv_slot: &mut f64,
        var_guard1476_slot: &mut f64,
        var_guard1476_rv_slot: &mut f64,
        var_guard1477_slot: &mut f64,
        var_guard1477_rv_slot: &mut f64,
        var_guard1478_slot: &mut f64,
        var_guard1478_rv_slot: &mut f64,
        var_guard1479_slot: &mut f64,
        var_guard1479_rv_slot: &mut f64,
        var_m0_slot: &mut f64,
        var_m0_rv_slot: &mut f64,
        var_mm_slot: &mut f64,
        var_mm_rv_slot: &mut f64,
        var_qinm_slot: &mut f64,
        var_qinm_dn0_slot: &mut f64,
        var_qinm_dn10_slot: &mut f64,
        var_qinm_dn13_slot: &mut f64,
        var_qinm_dn2_slot: &mut f64,
        var_qinm_dn4_slot: &mut f64,
        var_qinm_dn5_slot: &mut f64,
        var_qinm_dn6_slot: &mut f64,
        var_qinm_dn7_slot: &mut f64,
        var_qinm_dn8_slot: &mut f64,
        var_qinm_dn9_slot: &mut f64,
        var_qinm_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn13_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_dn9_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
        var_ty_slot: &mut f64,
        var_ty_dn0_slot: &mut f64,
        var_ty_dn10_slot: &mut f64,
        var_ty_dn13_slot: &mut f64,
        var_ty_dn2_slot: &mut f64,
        var_ty_dn4_slot: &mut f64,
        var_ty_dn5_slot: &mut f64,
        var_ty_dn6_slot: &mut f64,
        var_ty_dn7_slot: &mut f64,
        var_ty_dn8_slot: &mut f64,
        var_ty_dn9_slot: &mut f64,
        var_ty_rv_slot: &mut f64,
        var_xmp_slot: &mut f64,
        var_xmp_dn0_slot: &mut f64,
        var_xmp_dn10_slot: &mut f64,
        var_xmp_dn13_slot: &mut f64,
        var_xmp_dn2_slot: &mut f64,
        var_xmp_dn4_slot: &mut f64,
        var_xmp_dn5_slot: &mut f64,
        var_xmp_dn6_slot: &mut f64,
        var_xmp_dn7_slot: &mut f64,
        var_xmp_dn8_slot: &mut f64,
        var_xmp_dn9_slot: &mut f64,
        var_xmp_rv_slot: &mut f64,
        var_xp_slot: &mut f64,
        var_xp_dn0_slot: &mut f64,
        var_xp_dn10_slot: &mut f64,
        var_xp_dn13_slot: &mut f64,
        var_xp_dn2_slot: &mut f64,
        var_xp_dn4_slot: &mut f64,
        var_xp_dn5_slot: &mut f64,
        var_xp_dn6_slot: &mut f64,
        var_xp_dn7_slot: &mut f64,
        var_xp_dn8_slot: &mut f64,
        var_xp_dn9_slot: &mut f64,
        var_xp_rv_slot: &mut f64,
    ) {
        let mut var_alpha: f64 = *var_alpha_slot;
        let mut var_alpha_dn0: f64 = *var_alpha_dn0_slot;
        let mut var_alpha_dn10: f64 = *var_alpha_dn10_slot;
        let mut var_alpha_dn13: f64 = *var_alpha_dn13_slot;
        let mut var_alpha_dn2: f64 = *var_alpha_dn2_slot;
        let mut var_alpha_dn4: f64 = *var_alpha_dn4_slot;
        let mut var_alpha_dn5: f64 = *var_alpha_dn5_slot;
        let mut var_alpha_dn6: f64 = *var_alpha_dn6_slot;
        let mut var_alpha_dn7: f64 = *var_alpha_dn7_slot;
        let mut var_alpha_dn8: f64 = *var_alpha_dn8_slot;
        let mut var_alpha_dn9: f64 = *var_alpha_dn9_slot;
        let mut var_alpha_rv: f64 = *var_alpha_rv_slot;
        let mut var_arg: f64 = *var_arg_slot;
        let mut var_arg_dn0: f64 = *var_arg_dn0_slot;
        let mut var_arg_dn10: f64 = *var_arg_dn10_slot;
        let mut var_arg_dn13: f64 = *var_arg_dn13_slot;
        let mut var_arg_dn2: f64 = *var_arg_dn2_slot;
        let mut var_arg_dn4: f64 = *var_arg_dn4_slot;
        let mut var_arg_dn5: f64 = *var_arg_dn5_slot;
        let mut var_arg_dn6: f64 = *var_arg_dn6_slot;
        let mut var_arg_dn7: f64 = *var_arg_dn7_slot;
        let mut var_arg_dn8: f64 = *var_arg_dn8_slot;
        let mut var_arg_dn9: f64 = *var_arg_dn9_slot;
        let mut var_arg_rv: f64 = *var_arg_rv_slot;
        let mut var_dnm: f64 = *var_dnm_slot;
        let mut var_dnm_dn0: f64 = *var_dnm_dn0_slot;
        let mut var_dnm_dn10: f64 = *var_dnm_dn10_slot;
        let mut var_dnm_dn13: f64 = *var_dnm_dn13_slot;
        let mut var_dnm_dn2: f64 = *var_dnm_dn2_slot;
        let mut var_dnm_dn4: f64 = *var_dnm_dn4_slot;
        let mut var_dnm_dn5: f64 = *var_dnm_dn5_slot;
        let mut var_dnm_dn6: f64 = *var_dnm_dn6_slot;
        let mut var_dnm_dn7: f64 = *var_dnm_dn7_slot;
        let mut var_dnm_dn8: f64 = *var_dnm_dn8_slot;
        let mut var_dnm_dn9: f64 = *var_dnm_dn9_slot;
        let mut var_dnm_rv: f64 = *var_dnm_rv_slot;
        let mut var_guard1475: f64 = *var_guard1475_slot;
        let mut var_guard1475_rv: f64 = *var_guard1475_rv_slot;
        let mut var_guard1476: f64 = *var_guard1476_slot;
        let mut var_guard1476_rv: f64 = *var_guard1476_rv_slot;
        let mut var_guard1477: f64 = *var_guard1477_slot;
        let mut var_guard1477_rv: f64 = *var_guard1477_rv_slot;
        let mut var_guard1478: f64 = *var_guard1478_slot;
        let mut var_guard1478_rv: f64 = *var_guard1478_rv_slot;
        let mut var_guard1479: f64 = *var_guard1479_slot;
        let mut var_guard1479_rv: f64 = *var_guard1479_rv_slot;
        let mut var_m0: f64 = *var_m0_slot;
        let mut var_m0_rv: f64 = *var_m0_rv_slot;
        let mut var_mm: f64 = *var_mm_slot;
        let mut var_mm_rv: f64 = *var_mm_rv_slot;
        let mut var_qinm: f64 = *var_qinm_slot;
        let mut var_qinm_dn0: f64 = *var_qinm_dn0_slot;
        let mut var_qinm_dn10: f64 = *var_qinm_dn10_slot;
        let mut var_qinm_dn13: f64 = *var_qinm_dn13_slot;
        let mut var_qinm_dn2: f64 = *var_qinm_dn2_slot;
        let mut var_qinm_dn4: f64 = *var_qinm_dn4_slot;
        let mut var_qinm_dn5: f64 = *var_qinm_dn5_slot;
        let mut var_qinm_dn6: f64 = *var_qinm_dn6_slot;
        let mut var_qinm_dn7: f64 = *var_qinm_dn7_slot;
        let mut var_qinm_dn8: f64 = *var_qinm_dn8_slot;
        let mut var_qinm_dn9: f64 = *var_qinm_dn9_slot;
        let mut var_qinm_rv: f64 = *var_qinm_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn13: f64 = *var_t4_dn13_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_dn9: f64 = *var_t4_dn9_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;
        let mut var_ty: f64 = *var_ty_slot;
        let mut var_ty_dn0: f64 = *var_ty_dn0_slot;
        let mut var_ty_dn10: f64 = *var_ty_dn10_slot;
        let mut var_ty_dn13: f64 = *var_ty_dn13_slot;
        let mut var_ty_dn2: f64 = *var_ty_dn2_slot;
        let mut var_ty_dn4: f64 = *var_ty_dn4_slot;
        let mut var_ty_dn5: f64 = *var_ty_dn5_slot;
        let mut var_ty_dn6: f64 = *var_ty_dn6_slot;
        let mut var_ty_dn7: f64 = *var_ty_dn7_slot;
        let mut var_ty_dn8: f64 = *var_ty_dn8_slot;
        let mut var_ty_dn9: f64 = *var_ty_dn9_slot;
        let mut var_ty_rv: f64 = *var_ty_rv_slot;
        let mut var_xmp: f64 = *var_xmp_slot;
        let mut var_xmp_dn0: f64 = *var_xmp_dn0_slot;
        let mut var_xmp_dn10: f64 = *var_xmp_dn10_slot;
        let mut var_xmp_dn13: f64 = *var_xmp_dn13_slot;
        let mut var_xmp_dn2: f64 = *var_xmp_dn2_slot;
        let mut var_xmp_dn4: f64 = *var_xmp_dn4_slot;
        let mut var_xmp_dn5: f64 = *var_xmp_dn5_slot;
        let mut var_xmp_dn6: f64 = *var_xmp_dn6_slot;
        let mut var_xmp_dn7: f64 = *var_xmp_dn7_slot;
        let mut var_xmp_dn8: f64 = *var_xmp_dn8_slot;
        let mut var_xmp_dn9: f64 = *var_xmp_dn9_slot;
        let mut var_xmp_rv: f64 = *var_xmp_rv_slot;
        let mut var_xp: f64 = *var_xp_slot;
        let mut var_xp_dn0: f64 = *var_xp_dn0_slot;
        let mut var_xp_dn10: f64 = *var_xp_dn10_slot;
        let mut var_xp_dn13: f64 = *var_xp_dn13_slot;
        let mut var_xp_dn2: f64 = *var_xp_dn2_slot;
        let mut var_xp_dn4: f64 = *var_xp_dn4_slot;
        let mut var_xp_dn5: f64 = *var_xp_dn5_slot;
        let mut var_xp_dn6: f64 = *var_xp_dn6_slot;
        let mut var_xp_dn7: f64 = *var_xp_dn7_slot;
        let mut var_xp_dn8: f64 = *var_xp_dn8_slot;
        let mut var_xp_dn9: f64 = *var_xp_dn9_slot;
        let mut var_xp_rv: f64 = *var_xp_rv_slot;

        let (assign60950_e95077,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        (0.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign60950_e95077;
        var_mm_rv = 0.0;

        let (assign60960_e95086, assign60960_e95086_d_n0, assign60960_e95086_d_n2, assign60960_e95086_d_n4, assign60960_e95086_d_n5, assign60960_e95086_d_n6, assign60960_e95086_d_n7, assign60960_e95086_d_n8, assign60960_e95086_d_n9, assign60960_e95086_d_n10, assign60960_e95086_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_dn10, var_arg_dn13,)
    }
};
        var_arg = assign60960_e95086;
        var_arg_dn0 = assign60960_e95086_d_n0;
        var_arg_dn2 = assign60960_e95086_d_n2;
        var_arg_dn4 = assign60960_e95086_d_n4;
        var_arg_dn5 = assign60960_e95086_d_n5;
        var_arg_dn6 = assign60960_e95086_d_n6;
        var_arg_dn7 = assign60960_e95086_d_n7;
        var_arg_dn8 = assign60960_e95086_d_n8;
        var_arg_dn9 = assign60960_e95086_d_n9;
        var_arg_dn10 = assign60960_e95086_d_n10;
        var_arg_dn13 = assign60960_e95086_d_n13;
        var_arg_rv = 0.0;

        let (assign60970_e95095, assign60970_e95095_d_n0, assign60970_e95095_d_n2, assign60970_e95095_d_n4, assign60970_e95095_d_n5, assign60970_e95095_d_n6, assign60970_e95095_d_n7, assign60970_e95095_d_n8, assign60970_e95095_d_n9, assign60970_e95095_d_n10, assign60970_e95095_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
        var_dnm = assign60970_e95095;
        var_dnm_dn0 = assign60970_e95095_d_n0;
        var_dnm_dn2 = assign60970_e95095_d_n2;
        var_dnm_dn4 = assign60970_e95095_d_n4;
        var_dnm_dn5 = assign60970_e95095_d_n5;
        var_dnm_dn6 = assign60970_e95095_d_n6;
        var_dnm_dn7 = assign60970_e95095_d_n7;
        var_dnm_dn8 = assign60970_e95095_d_n8;
        var_dnm_dn9 = assign60970_e95095_d_n9;
        var_dnm_dn10 = assign60970_e95095_d_n10;
        var_dnm_dn13 = assign60970_e95095_d_n13;
        var_dnm_rv = 0.0;

        let (assign60980_e95106, assign60980_e95106_d_n0, assign60980_e95106_d_n2, assign60980_e95106_d_n4, assign60980_e95106_d_n5, assign60980_e95106_d_n6, assign60980_e95106_d_n7, assign60980_e95106_d_n8, assign60980_e95106_d_n9, assign60980_e95106_d_n10, assign60980_e95106_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign60980_e95104: f64 = (var_xp * var_x2);
        (assign60980_e95104, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn4 * var_x2) + (var_xp * var_x2_dn4)), ((var_xp_dn5 * var_x2) + (var_xp * var_x2_dn5)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn8 * var_x2) + (var_xp * var_x2_dn8)), ((var_xp_dn9 * var_x2) + (var_xp * var_x2_dn9)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn13 * var_x2) + (var_xp * var_x2_dn13)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn7, var_xp_dn8, var_xp_dn9, var_xp_dn10, var_xp_dn13,)
    }
};
        var_xp = assign60980_e95106;
        var_xp_dn0 = assign60980_e95106_d_n0;
        var_xp_dn2 = assign60980_e95106_d_n2;
        var_xp_dn4 = assign60980_e95106_d_n4;
        var_xp_dn5 = assign60980_e95106_d_n5;
        var_xp_dn6 = assign60980_e95106_d_n6;
        var_xp_dn7 = assign60980_e95106_d_n7;
        var_xp_dn8 = assign60980_e95106_d_n8;
        var_xp_dn9 = assign60980_e95106_d_n9;
        var_xp_dn10 = assign60980_e95106_d_n10;
        var_xp_dn13 = assign60980_e95106_d_n13;
        var_xp_rv = 0.0;

        let (assign60990_e95117, assign60990_e95117_d_n0, assign60990_e95117_d_n2, assign60990_e95117_d_n4, assign60990_e95117_d_n5, assign60990_e95117_d_n6, assign60990_e95117_d_n7, assign60990_e95117_d_n8, assign60990_e95117_d_n9, assign60990_e95117_d_n10, assign60990_e95117_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign60990_e95115: f64 = (var_xmp * var_xmax2);
        (assign60990_e95115, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn4 * var_xmax2) + (var_xmp * var_xmax2_dn4)), ((var_xmp_dn5 * var_xmax2) + (var_xmp * var_xmax2_dn5)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn8 * var_xmax2) + (var_xmp * var_xmax2_dn8)), ((var_xmp_dn9 * var_xmax2) + (var_xmp * var_xmax2_dn9)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn13 * var_xmax2) + (var_xmp * var_xmax2_dn13)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn7, var_xmp_dn8, var_xmp_dn9, var_xmp_dn10, var_xmp_dn13,)
    }
};
        var_xmp = assign60990_e95117;
        var_xmp_dn0 = assign60990_e95117_d_n0;
        var_xmp_dn2 = assign60990_e95117_d_n2;
        var_xmp_dn4 = assign60990_e95117_d_n4;
        var_xmp_dn5 = assign60990_e95117_d_n5;
        var_xmp_dn6 = assign60990_e95117_d_n6;
        var_xmp_dn7 = assign60990_e95117_d_n7;
        var_xmp_dn8 = assign60990_e95117_d_n8;
        var_xmp_dn9 = assign60990_e95117_d_n9;
        var_xmp_dn10 = assign60990_e95117_d_n10;
        var_xmp_dn13 = assign60990_e95117_d_n13;
        var_xmp_rv = 0.0;

        let (assign61000_e95128, assign61000_e95128_d_n0, assign61000_e95128_d_n2, assign61000_e95128_d_n4, assign61000_e95128_d_n5, assign61000_e95128_d_n6, assign61000_e95128_d_n7, assign61000_e95128_d_n8, assign61000_e95128_d_n9, assign61000_e95128_d_n10, assign61000_e95128_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign61000_e95126: f64 = (var_xp * var_x2);
        (assign61000_e95126, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn4 * var_x2) + (var_xp * var_x2_dn4)), ((var_xp_dn5 * var_x2) + (var_xp * var_x2_dn5)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn8 * var_x2) + (var_xp * var_x2_dn8)), ((var_xp_dn9 * var_x2) + (var_xp * var_x2_dn9)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn13 * var_x2) + (var_xp * var_x2_dn13)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn7, var_xp_dn8, var_xp_dn9, var_xp_dn10, var_xp_dn13,)
    }
};
        var_xp = assign61000_e95128;
        var_xp_dn0 = assign61000_e95128_d_n0;
        var_xp_dn2 = assign61000_e95128_d_n2;
        var_xp_dn4 = assign61000_e95128_d_n4;
        var_xp_dn5 = assign61000_e95128_d_n5;
        var_xp_dn6 = assign61000_e95128_d_n6;
        var_xp_dn7 = assign61000_e95128_d_n7;
        var_xp_dn8 = assign61000_e95128_d_n8;
        var_xp_dn9 = assign61000_e95128_d_n9;
        var_xp_dn10 = assign61000_e95128_d_n10;
        var_xp_dn13 = assign61000_e95128_d_n13;
        var_xp_rv = 0.0;

        let (assign61010_e95139, assign61010_e95139_d_n0, assign61010_e95139_d_n2, assign61010_e95139_d_n4, assign61010_e95139_d_n5, assign61010_e95139_d_n6, assign61010_e95139_d_n7, assign61010_e95139_d_n8, assign61010_e95139_d_n9, assign61010_e95139_d_n10, assign61010_e95139_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign61010_e95137: f64 = (var_xmp * var_xmax2);
        (assign61010_e95137, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn4 * var_xmax2) + (var_xmp * var_xmax2_dn4)), ((var_xmp_dn5 * var_xmax2) + (var_xmp * var_xmax2_dn5)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn8 * var_xmax2) + (var_xmp * var_xmax2_dn8)), ((var_xmp_dn9 * var_xmax2) + (var_xmp * var_xmax2_dn9)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn13 * var_xmax2) + (var_xmp * var_xmax2_dn13)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn7, var_xmp_dn8, var_xmp_dn9, var_xmp_dn10, var_xmp_dn13,)
    }
};
        var_xmp = assign61010_e95139;
        var_xmp_dn0 = assign61010_e95139_d_n0;
        var_xmp_dn2 = assign61010_e95139_d_n2;
        var_xmp_dn4 = assign61010_e95139_d_n4;
        var_xmp_dn5 = assign61010_e95139_d_n5;
        var_xmp_dn6 = assign61010_e95139_d_n6;
        var_xmp_dn7 = assign61010_e95139_d_n7;
        var_xmp_dn8 = assign61010_e95139_d_n8;
        var_xmp_dn9 = assign61010_e95139_d_n9;
        var_xmp_dn10 = assign61010_e95139_d_n10;
        var_xmp_dn13 = assign61010_e95139_d_n13;
        var_xmp_rv = 0.0;

        let (assign61020_e95150, assign61020_e95150_d_n0, assign61020_e95150_d_n2, assign61020_e95150_d_n4, assign61020_e95150_d_n5, assign61020_e95150_d_n6, assign61020_e95150_d_n7, assign61020_e95150_d_n8, assign61020_e95150_d_n9, assign61020_e95150_d_n10, assign61020_e95150_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign61020_e95148: f64 = (var_xp * var_x2);
        (assign61020_e95148, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn4 * var_x2) + (var_xp * var_x2_dn4)), ((var_xp_dn5 * var_x2) + (var_xp * var_x2_dn5)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn8 * var_x2) + (var_xp * var_x2_dn8)), ((var_xp_dn9 * var_x2) + (var_xp * var_x2_dn9)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn13 * var_x2) + (var_xp * var_x2_dn13)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn7, var_xp_dn8, var_xp_dn9, var_xp_dn10, var_xp_dn13,)
    }
};
        var_xp = assign61020_e95150;
        var_xp_dn0 = assign61020_e95150_d_n0;
        var_xp_dn2 = assign61020_e95150_d_n2;
        var_xp_dn4 = assign61020_e95150_d_n4;
        var_xp_dn5 = assign61020_e95150_d_n5;
        var_xp_dn6 = assign61020_e95150_d_n6;
        var_xp_dn7 = assign61020_e95150_d_n7;
        var_xp_dn8 = assign61020_e95150_d_n8;
        var_xp_dn9 = assign61020_e95150_d_n9;
        var_xp_dn10 = assign61020_e95150_d_n10;
        var_xp_dn13 = assign61020_e95150_d_n13;
        var_xp_rv = 0.0;

        let (assign61030_e95161, assign61030_e95161_d_n0, assign61030_e95161_d_n2, assign61030_e95161_d_n4, assign61030_e95161_d_n5, assign61030_e95161_d_n6, assign61030_e95161_d_n7, assign61030_e95161_d_n8, assign61030_e95161_d_n9, assign61030_e95161_d_n10, assign61030_e95161_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign61030_e95159: f64 = (var_xmp * var_xmax2);
        (assign61030_e95159, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn4 * var_xmax2) + (var_xmp * var_xmax2_dn4)), ((var_xmp_dn5 * var_xmax2) + (var_xmp * var_xmax2_dn5)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn8 * var_xmax2) + (var_xmp * var_xmax2_dn8)), ((var_xmp_dn9 * var_xmax2) + (var_xmp * var_xmax2_dn9)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn13 * var_xmax2) + (var_xmp * var_xmax2_dn13)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn7, var_xmp_dn8, var_xmp_dn9, var_xmp_dn10, var_xmp_dn13,)
    }
};
        var_xmp = assign61030_e95161;
        var_xmp_dn0 = assign61030_e95161_d_n0;
        var_xmp_dn2 = assign61030_e95161_d_n2;
        var_xmp_dn4 = assign61030_e95161_d_n4;
        var_xmp_dn5 = assign61030_e95161_d_n5;
        var_xmp_dn6 = assign61030_e95161_d_n6;
        var_xmp_dn7 = assign61030_e95161_d_n7;
        var_xmp_dn8 = assign61030_e95161_d_n8;
        var_xmp_dn9 = assign61030_e95161_d_n9;
        var_xmp_dn10 = assign61030_e95161_d_n10;
        var_xmp_dn13 = assign61030_e95161_d_n13;
        var_xmp_rv = 0.0;

        let (assign61040_e95172, assign61040_e95172_d_n0, assign61040_e95172_d_n2, assign61040_e95172_d_n4, assign61040_e95172_d_n5, assign61040_e95172_d_n6, assign61040_e95172_d_n7, assign61040_e95172_d_n8, assign61040_e95172_d_n9, assign61040_e95172_d_n10, assign61040_e95172_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign61040_e95170: f64 = (var_xp * var_x2);
        (assign61040_e95170, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn4 * var_x2) + (var_xp * var_x2_dn4)), ((var_xp_dn5 * var_x2) + (var_xp * var_x2_dn5)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn8 * var_x2) + (var_xp * var_x2_dn8)), ((var_xp_dn9 * var_x2) + (var_xp * var_x2_dn9)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn13 * var_x2) + (var_xp * var_x2_dn13)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn7, var_xp_dn8, var_xp_dn9, var_xp_dn10, var_xp_dn13,)
    }
};
        var_xp = assign61040_e95172;
        var_xp_dn0 = assign61040_e95172_d_n0;
        var_xp_dn2 = assign61040_e95172_d_n2;
        var_xp_dn4 = assign61040_e95172_d_n4;
        var_xp_dn5 = assign61040_e95172_d_n5;
        var_xp_dn6 = assign61040_e95172_d_n6;
        var_xp_dn7 = assign61040_e95172_d_n7;
        var_xp_dn8 = assign61040_e95172_d_n8;
        var_xp_dn9 = assign61040_e95172_d_n9;
        var_xp_dn10 = assign61040_e95172_d_n10;
        var_xp_dn13 = assign61040_e95172_d_n13;
        var_xp_rv = 0.0;

        let (assign61050_e95183, assign61050_e95183_d_n0, assign61050_e95183_d_n2, assign61050_e95183_d_n4, assign61050_e95183_d_n5, assign61050_e95183_d_n6, assign61050_e95183_d_n7, assign61050_e95183_d_n8, assign61050_e95183_d_n9, assign61050_e95183_d_n10, assign61050_e95183_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign61050_e95181: f64 = (var_xmp * var_xmax2);
        (assign61050_e95181, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn4 * var_xmax2) + (var_xmp * var_xmax2_dn4)), ((var_xmp_dn5 * var_xmax2) + (var_xmp * var_xmax2_dn5)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn8 * var_xmax2) + (var_xmp * var_xmax2_dn8)), ((var_xmp_dn9 * var_xmax2) + (var_xmp * var_xmax2_dn9)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn13 * var_xmax2) + (var_xmp * var_xmax2_dn13)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn7, var_xmp_dn8, var_xmp_dn9, var_xmp_dn10, var_xmp_dn13,)
    }
};
        var_xmp = assign61050_e95183;
        var_xmp_dn0 = assign61050_e95183_d_n0;
        var_xmp_dn2 = assign61050_e95183_d_n2;
        var_xmp_dn4 = assign61050_e95183_d_n4;
        var_xmp_dn5 = assign61050_e95183_d_n5;
        var_xmp_dn6 = assign61050_e95183_d_n6;
        var_xmp_dn7 = assign61050_e95183_d_n7;
        var_xmp_dn8 = assign61050_e95183_d_n8;
        var_xmp_dn9 = assign61050_e95183_d_n9;
        var_xmp_dn10 = assign61050_e95183_d_n10;
        var_xmp_dn13 = assign61050_e95183_d_n13;
        var_xmp_rv = 0.0;

        let (assign61060_e95194, assign61060_e95194_d_n0, assign61060_e95194_d_n2, assign61060_e95194_d_n4, assign61060_e95194_d_n5, assign61060_e95194_d_n6, assign61060_e95194_d_n7, assign61060_e95194_d_n8, assign61060_e95194_d_n9, assign61060_e95194_d_n10, assign61060_e95194_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign61060_e95192: f64 = (var_xp + var_xmp);
        (assign61060_e95192, (var_xp_dn0 + var_xmp_dn0), (var_xp_dn2 + var_xmp_dn2), (var_xp_dn4 + var_xmp_dn4), (var_xp_dn5 + var_xmp_dn5), (var_xp_dn6 + var_xmp_dn6), (var_xp_dn7 + var_xmp_dn7), (var_xp_dn8 + var_xmp_dn8), (var_xp_dn9 + var_xmp_dn9), (var_xp_dn10 + var_xmp_dn10), (var_xp_dn13 + var_xmp_dn13),)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_dn10, var_arg_dn13,)
    }
};
        var_arg = assign61060_e95194;
        var_arg_dn0 = assign61060_e95194_d_n0;
        var_arg_dn2 = assign61060_e95194_d_n2;
        var_arg_dn4 = assign61060_e95194_d_n4;
        var_arg_dn5 = assign61060_e95194_d_n5;
        var_arg_dn6 = assign61060_e95194_d_n6;
        var_arg_dn7 = assign61060_e95194_d_n7;
        var_arg_dn8 = assign61060_e95194_d_n8;
        var_arg_dn9 = assign61060_e95194_d_n9;
        var_arg_dn10 = assign61060_e95194_d_n10;
        var_arg_dn13 = assign61060_e95194_d_n13;
        var_arg_rv = 0.0;

        let (assign61070_e95203, assign61070_e95203_d_n0, assign61070_e95203_d_n2, assign61070_e95203_d_n4, assign61070_e95203_d_n5, assign61070_e95203_d_n6, assign61070_e95203_d_n7, assign61070_e95203_d_n8, assign61070_e95203_d_n9, assign61070_e95203_d_n10, assign61070_e95203_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_dn10, var_arg_dn13,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
        var_dnm = assign61070_e95203;
        var_dnm_dn0 = assign61070_e95203_d_n0;
        var_dnm_dn2 = assign61070_e95203_d_n2;
        var_dnm_dn4 = assign61070_e95203_d_n4;
        var_dnm_dn5 = assign61070_e95203_d_n5;
        var_dnm_dn6 = assign61070_e95203_d_n6;
        var_dnm_dn7 = assign61070_e95203_d_n7;
        var_dnm_dn8 = assign61070_e95203_d_n8;
        var_dnm_dn9 = assign61070_e95203_d_n9;
        var_dnm_dn10 = assign61070_e95203_d_n10;
        var_dnm_dn13 = assign61070_e95203_d_n13;
        var_dnm_rv = 0.0;

        let assign61080_e95218: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        var_guard1475 = assign61080_e95218;
        var_guard1475_rv = 0.0;

        let assign61090_e95221: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard1476 = assign61090_e95221;
        var_guard1476_rv = 0.0;

        let (assign61100_e95234,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1475 != 0.0)) && (var_guard1476 != 0.0)) {
        (1.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign61100_e95234;
        var_mm_rv = 0.0;

        let assign61110_e95237: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        var_guard1477 = assign61110_e95237;
        var_guard1477_rv = 0.0;

        let (assign61120_e95253,) = {
    if ((((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1475 != 0.0)) && (var_guard1476 == 0.0)) && (var_guard1477 != 0.0)) {
        (2.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign61120_e95253;
        var_mm_rv = 0.0;

        let assign61130_e95256: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        var_guard1478 = assign61130_e95256;
        var_guard1478_rv = 0.0;

        let (assign61140_e95275,) = {
    if (((((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1475 != 0.0)) && (var_guard1476 == 0.0)) && (var_guard1477 == 0.0)) && (var_guard1478 != 0.0)) {
        (3.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign61140_e95275;
        var_mm_rv = 0.0;

        let assign61150_e95278: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        var_guard1479 = assign61150_e95278;
        var_guard1479_rv = 0.0;

        let (assign61160_e95300,) = {
    if ((((((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1475 != 0.0)) && (var_guard1476 == 0.0)) && (var_guard1477 == 0.0)) && (var_guard1478 == 0.0)) && (var_guard1479 != 0.0)) {
        (4.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign61160_e95300;
        var_mm_rv = 0.0;

        let (assign61170_e95311,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1475 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign61170_e95311;
        var_m0_rv = 0.0;

        let mut assign61180_loop_guard: usize = 0;
        while {
            let assign61180_cond_e95323: f64 = if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1475 != 0.0)) && (var_m0 < var_mm)) { 1.0 } else { 0.0 };
            assign61180_cond_e95323 != 0.0
        } {
            assign61180_loop_guard += 1;
            assert!(assign61180_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign61180_body0_e95335, assign61180_body0_e95335_d_n0, assign61180_body0_e95335_d_n2, assign61180_body0_e95335_d_n4, assign61180_body0_e95335_d_n5, assign61180_body0_e95335_d_n6, assign61180_body0_e95335_d_n7, assign61180_body0_e95335_d_n8, assign61180_body0_e95335_d_n9, assign61180_body0_e95335_d_n10, assign61180_body0_e95335_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1475 != 0.0)) {
        let assign61180_body0_e95333: f64 = (var_dnm).sqrt();
        (assign61180_body0_e95333, (var_dnm_dn0 / (2.0 * assign61180_body0_e95333)), (var_dnm_dn2 / (2.0 * assign61180_body0_e95333)), (var_dnm_dn4 / (2.0 * assign61180_body0_e95333)), (var_dnm_dn5 / (2.0 * assign61180_body0_e95333)), (var_dnm_dn6 / (2.0 * assign61180_body0_e95333)), (var_dnm_dn7 / (2.0 * assign61180_body0_e95333)), (var_dnm_dn8 / (2.0 * assign61180_body0_e95333)), (var_dnm_dn9 / (2.0 * assign61180_body0_e95333)), (var_dnm_dn10 / (2.0 * assign61180_body0_e95333)), (var_dnm_dn13 / (2.0 * assign61180_body0_e95333)),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
            var_dnm = assign61180_body0_e95335;
            var_dnm_dn0 = assign61180_body0_e95335_d_n0;
            var_dnm_dn2 = assign61180_body0_e95335_d_n2;
            var_dnm_dn4 = assign61180_body0_e95335_d_n4;
            var_dnm_dn5 = assign61180_body0_e95335_d_n5;
            var_dnm_dn6 = assign61180_body0_e95335_d_n6;
            var_dnm_dn7 = assign61180_body0_e95335_d_n7;
            var_dnm_dn8 = assign61180_body0_e95335_d_n8;
            var_dnm_dn9 = assign61180_body0_e95335_d_n9;
            var_dnm_dn10 = assign61180_body0_e95335_d_n10;
            var_dnm_dn13 = assign61180_body0_e95335_d_n13;
            var_dnm_rv = 0.0;
            let (assign61180_body1_e95348,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1475 != 0.0)) {
        let assign61180_body1_e95346: f64 = (var_m0 + 1.0);
        (assign61180_body1_e95346,)
    } else {
        (var_m0,)
    }
};
            var_m0 = assign61180_body1_e95348;
            var_m0_rv = 0.0;
        }

        let (assign61190_e95371, assign61190_e95371_d_n0, assign61190_e95371_d_n2, assign61190_e95371_d_n4, assign61190_e95371_d_n5, assign61190_e95371_d_n6, assign61190_e95371_d_n7, assign61190_e95371_d_n8, assign61190_e95371_d_n9, assign61190_e95371_d_n10, assign61190_e95371_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1475 == 0.0)) {
        let (assign61190_e95369, assign61190_e95369_d_n0, assign61190_e95369_d_n2, assign61190_e95369_d_n4, assign61190_e95369_d_n5, assign61190_e95369_d_n6, assign61190_e95369_d_n7, assign61190_e95369_d_n8, assign61190_e95369_d_n9, assign61190_e95369_d_n10, assign61190_e95369_d_n13,) = {
            if (var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign61190_e95366: f64 = (2.0 * 4.0);
                let assign61190_e95367: f64 = (1.0 / assign61190_e95366);
                let assign61190_e95368: f64 = (var_dnm).powf(assign61190_e95367);
                (assign61190_e95368, if 0.0 == 0.0 && ((assign61190_e95367) as f64).is_finite() && ((assign61190_e95367) as f64).fract() == 0.0 { if assign61190_e95367 == 0.0 { 0.0 } else { (assign61190_e95367 * ((var_dnm).powf(assign61190_e95367 - 1.0) * var_dnm_dn0)) } } else { (assign61190_e95368 * (assign61190_e95367 * (var_dnm_dn0 / var_dnm))) }, if 0.0 == 0.0 && ((assign61190_e95367) as f64).is_finite() && ((assign61190_e95367) as f64).fract() == 0.0 { if assign61190_e95367 == 0.0 { 0.0 } else { (assign61190_e95367 * ((var_dnm).powf(assign61190_e95367 - 1.0) * var_dnm_dn2)) } } else { (assign61190_e95368 * (assign61190_e95367 * (var_dnm_dn2 / var_dnm))) }, if 0.0 == 0.0 && ((assign61190_e95367) as f64).is_finite() && ((assign61190_e95367) as f64).fract() == 0.0 { if assign61190_e95367 == 0.0 { 0.0 } else { (assign61190_e95367 * ((var_dnm).powf(assign61190_e95367 - 1.0) * var_dnm_dn4)) } } else { (assign61190_e95368 * (assign61190_e95367 * (var_dnm_dn4 / var_dnm))) }, if 0.0 == 0.0 && ((assign61190_e95367) as f64).is_finite() && ((assign61190_e95367) as f64).fract() == 0.0 { if assign61190_e95367 == 0.0 { 0.0 } else { (assign61190_e95367 * ((var_dnm).powf(assign61190_e95367 - 1.0) * var_dnm_dn5)) } } else { (assign61190_e95368 * (assign61190_e95367 * (var_dnm_dn5 / var_dnm))) }, if 0.0 == 0.0 && ((assign61190_e95367) as f64).is_finite() && ((assign61190_e95367) as f64).fract() == 0.0 { if assign61190_e95367 == 0.0 { 0.0 } else { (assign61190_e95367 * ((var_dnm).powf(assign61190_e95367 - 1.0) * var_dnm_dn6)) } } else { (assign61190_e95368 * (assign61190_e95367 * (var_dnm_dn6 / var_dnm))) }, if 0.0 == 0.0 && ((assign61190_e95367) as f64).is_finite() && ((assign61190_e95367) as f64).fract() == 0.0 { if assign61190_e95367 == 0.0 { 0.0 } else { (assign61190_e95367 * ((var_dnm).powf(assign61190_e95367 - 1.0) * var_dnm_dn7)) } } else { (assign61190_e95368 * (assign61190_e95367 * (var_dnm_dn7 / var_dnm))) }, if 0.0 == 0.0 && ((assign61190_e95367) as f64).is_finite() && ((assign61190_e95367) as f64).fract() == 0.0 { if assign61190_e95367 == 0.0 { 0.0 } else { (assign61190_e95367 * ((var_dnm).powf(assign61190_e95367 - 1.0) * var_dnm_dn8)) } } else { (assign61190_e95368 * (assign61190_e95367 * (var_dnm_dn8 / var_dnm))) }, if 0.0 == 0.0 && ((assign61190_e95367) as f64).is_finite() && ((assign61190_e95367) as f64).fract() == 0.0 { if assign61190_e95367 == 0.0 { 0.0 } else { (assign61190_e95367 * ((var_dnm).powf(assign61190_e95367 - 1.0) * var_dnm_dn9)) } } else { (assign61190_e95368 * (assign61190_e95367 * (var_dnm_dn9 / var_dnm))) }, if 0.0 == 0.0 && ((assign61190_e95367) as f64).is_finite() && ((assign61190_e95367) as f64).fract() == 0.0 { if assign61190_e95367 == 0.0 { 0.0 } else { (assign61190_e95367 * ((var_dnm).powf(assign61190_e95367 - 1.0) * var_dnm_dn10)) } } else { (assign61190_e95368 * (assign61190_e95367 * (var_dnm_dn10 / var_dnm))) }, if 0.0 == 0.0 && ((assign61190_e95367) as f64).is_finite() && ((assign61190_e95367) as f64).fract() == 0.0 { if assign61190_e95367 == 0.0 { 0.0 } else { (assign61190_e95367 * ((var_dnm).powf(assign61190_e95367 - 1.0) * var_dnm_dn13)) } } else { (assign61190_e95368 * (assign61190_e95367 * (var_dnm_dn13 / var_dnm))) },)
            }
        };
        (assign61190_e95369, assign61190_e95369_d_n0, assign61190_e95369_d_n2, assign61190_e95369_d_n4, assign61190_e95369_d_n5, assign61190_e95369_d_n6, assign61190_e95369_d_n7, assign61190_e95369_d_n8, assign61190_e95369_d_n9, assign61190_e95369_d_n10, assign61190_e95369_d_n13,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
        var_dnm = assign61190_e95371;
        var_dnm_dn0 = assign61190_e95371_d_n0;
        var_dnm_dn2 = assign61190_e95371_d_n2;
        var_dnm_dn4 = assign61190_e95371_d_n4;
        var_dnm_dn5 = assign61190_e95371_d_n5;
        var_dnm_dn6 = assign61190_e95371_d_n6;
        var_dnm_dn7 = assign61190_e95371_d_n7;
        var_dnm_dn8 = assign61190_e95371_d_n8;
        var_dnm_dn9 = assign61190_e95371_d_n9;
        var_dnm_dn10 = assign61190_e95371_d_n10;
        var_dnm_dn13 = assign61190_e95371_d_n13;
        var_dnm_rv = 0.0;

        let (assign61200_e95382, assign61200_e95382_d_n0, assign61200_e95382_d_n2, assign61200_e95382_d_n4, assign61200_e95382_d_n5, assign61200_e95382_d_n6, assign61200_e95382_d_n7, assign61200_e95382_d_n8, assign61200_e95382_d_n9, assign61200_e95382_d_n10, assign61200_e95382_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign61200_e95380: f64 = (1.0 / var_dnm);
        (assign61200_e95380, (-(var_dnm_dn0 / (var_dnm * var_dnm))), (-(var_dnm_dn2 / (var_dnm * var_dnm))), (-(var_dnm_dn4 / (var_dnm * var_dnm))), (-(var_dnm_dn5 / (var_dnm * var_dnm))), (-(var_dnm_dn6 / (var_dnm * var_dnm))), (-(var_dnm_dn7 / (var_dnm * var_dnm))), (-(var_dnm_dn8 / (var_dnm * var_dnm))), (-(var_dnm_dn9 / (var_dnm * var_dnm))), (-(var_dnm_dn10 / (var_dnm * var_dnm))), (-(var_dnm_dn13 / (var_dnm * var_dnm))),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
        var_dnm = assign61200_e95382;
        var_dnm_dn0 = assign61200_e95382_d_n0;
        var_dnm_dn2 = assign61200_e95382_d_n2;
        var_dnm_dn4 = assign61200_e95382_d_n4;
        var_dnm_dn5 = assign61200_e95382_d_n5;
        var_dnm_dn6 = assign61200_e95382_d_n6;
        var_dnm_dn7 = assign61200_e95382_d_n7;
        var_dnm_dn8 = assign61200_e95382_d_n8;
        var_dnm_dn9 = assign61200_e95382_d_n9;
        var_dnm_dn10 = assign61200_e95382_d_n10;
        var_dnm_dn13 = assign61200_e95382_d_n13;
        var_dnm_rv = 0.0;

        let (assign61210_e95395, assign61210_e95395_d_n0, assign61210_e95395_d_n2, assign61210_e95395_d_n4, assign61210_e95395_d_n5, assign61210_e95395_d_n6, assign61210_e95395_d_n7, assign61210_e95395_d_n8, assign61210_e95395_d_n9, assign61210_e95395_d_n10, assign61210_e95395_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign61210_e95391: f64 = var_tx;
        let assign61210_e95393: f64 = (assign61210_e95391 * var_dnm);
        (assign61210_e95393, ((var_tx_dn0 * var_dnm) + (assign61210_e95391 * var_dnm_dn0)), ((var_tx_dn2 * var_dnm) + (assign61210_e95391 * var_dnm_dn2)), ((var_tx_dn4 * var_dnm) + (assign61210_e95391 * var_dnm_dn4)), ((var_tx_dn5 * var_dnm) + (assign61210_e95391 * var_dnm_dn5)), ((var_tx_dn6 * var_dnm) + (assign61210_e95391 * var_dnm_dn6)), ((var_tx_dn7 * var_dnm) + (assign61210_e95391 * var_dnm_dn7)), ((var_tx_dn8 * var_dnm) + (assign61210_e95391 * var_dnm_dn8)), ((var_tx_dn9 * var_dnm) + (assign61210_e95391 * var_dnm_dn9)), ((var_tx_dn10 * var_dnm) + (assign61210_e95391 * var_dnm_dn10)), ((var_tx_dn13 * var_dnm) + (assign61210_e95391 * var_dnm_dn13)),)
    } else {
        (var_ty, var_ty_dn0, var_ty_dn2, var_ty_dn4, var_ty_dn5, var_ty_dn6, var_ty_dn7, var_ty_dn8, var_ty_dn9, var_ty_dn10, var_ty_dn13,)
    }
};
        var_ty = assign61210_e95395;
        var_ty_dn0 = assign61210_e95395_d_n0;
        var_ty_dn2 = assign61210_e95395_d_n2;
        var_ty_dn4 = assign61210_e95395_d_n4;
        var_ty_dn5 = assign61210_e95395_d_n5;
        var_ty_dn6 = assign61210_e95395_d_n6;
        var_ty_dn7 = assign61210_e95395_d_n7;
        var_ty_dn8 = assign61210_e95395_d_n8;
        var_ty_dn9 = assign61210_e95395_d_n9;
        var_ty_dn10 = assign61210_e95395_d_n10;
        var_ty_dn13 = assign61210_e95395_d_n13;
        var_ty_rv = 0.0;

        let (assign61220_e95410, assign61220_e95410_d_n0, assign61220_e95410_d_n2, assign61220_e95410_d_n4, assign61220_e95410_d_n5, assign61220_e95410_d_n6, assign61220_e95410_d_n7, assign61220_e95410_d_n8, assign61220_e95410_d_n9, assign61220_e95410_d_n10, assign61220_e95410_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign61220_e95404: f64 = var_xmp;
        let assign61220_e95406: f64 = (assign61220_e95404 * var_dnm);
        let assign61220_e95408: f64 = (assign61220_e95406 / var_arg);
        (assign61220_e95408, (((((var_xmp_dn0 * var_dnm) + (assign61220_e95404 * var_dnm_dn0)) * var_arg) - (assign61220_e95406 * var_arg_dn0)) / (var_arg * var_arg)), (((((var_xmp_dn2 * var_dnm) + (assign61220_e95404 * var_dnm_dn2)) * var_arg) - (assign61220_e95406 * var_arg_dn2)) / (var_arg * var_arg)), (((((var_xmp_dn4 * var_dnm) + (assign61220_e95404 * var_dnm_dn4)) * var_arg) - (assign61220_e95406 * var_arg_dn4)) / (var_arg * var_arg)), (((((var_xmp_dn5 * var_dnm) + (assign61220_e95404 * var_dnm_dn5)) * var_arg) - (assign61220_e95406 * var_arg_dn5)) / (var_arg * var_arg)), (((((var_xmp_dn6 * var_dnm) + (assign61220_e95404 * var_dnm_dn6)) * var_arg) - (assign61220_e95406 * var_arg_dn6)) / (var_arg * var_arg)), (((((var_xmp_dn7 * var_dnm) + (assign61220_e95404 * var_dnm_dn7)) * var_arg) - (assign61220_e95406 * var_arg_dn7)) / (var_arg * var_arg)), (((((var_xmp_dn8 * var_dnm) + (assign61220_e95404 * var_dnm_dn8)) * var_arg) - (assign61220_e95406 * var_arg_dn8)) / (var_arg * var_arg)), (((((var_xmp_dn9 * var_dnm) + (assign61220_e95404 * var_dnm_dn9)) * var_arg) - (assign61220_e95406 * var_arg_dn9)) / (var_arg * var_arg)), (((((var_xmp_dn10 * var_dnm) + (assign61220_e95404 * var_dnm_dn10)) * var_arg) - (assign61220_e95406 * var_arg_dn10)) / (var_arg * var_arg)), (((((var_xmp_dn13 * var_dnm) + (assign61220_e95404 * var_dnm_dn13)) * var_arg) - (assign61220_e95406 * var_arg_dn13)) / (var_arg * var_arg)),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn13,)
    }
};
        var_t4 = assign61220_e95410;
        var_t4_dn0 = assign61220_e95410_d_n0;
        var_t4_dn2 = assign61220_e95410_d_n2;
        var_t4_dn4 = assign61220_e95410_d_n4;
        var_t4_dn5 = assign61220_e95410_d_n5;
        var_t4_dn6 = assign61220_e95410_d_n6;
        var_t4_dn7 = assign61220_e95410_d_n7;
        var_t4_dn8 = assign61220_e95410_d_n8;
        var_t4_dn9 = assign61220_e95410_d_n9;
        var_t4_dn10 = assign61220_e95410_d_n10;
        var_t4_dn13 = assign61220_e95410_d_n13;
        var_t4_rv = 0.0;

        let (assign61230_e95421, assign61230_e95421_d_n0, assign61230_e95421_d_n2, assign61230_e95421_d_n4, assign61230_e95421_d_n5, assign61230_e95421_d_n6, assign61230_e95421_d_n7, assign61230_e95421_d_n8, assign61230_e95421_d_n9, assign61230_e95421_d_n10, assign61230_e95421_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign61230_e95419: f64 = (1.0 - var_ty);
        (assign61230_e95419, (-var_ty_dn0), (-var_ty_dn2), (-var_ty_dn4), (-var_ty_dn5), (-var_ty_dn6), (-var_ty_dn7), (-var_ty_dn8), (-var_ty_dn9), (-var_ty_dn10), (-var_ty_dn13),)
    } else {
        (var_alpha, var_alpha_dn0, var_alpha_dn2, var_alpha_dn4, var_alpha_dn5, var_alpha_dn6, var_alpha_dn7, var_alpha_dn8, var_alpha_dn9, var_alpha_dn10, var_alpha_dn13,)
    }
};
        var_alpha = assign61230_e95421;
        var_alpha_dn0 = assign61230_e95421_d_n0;
        var_alpha_dn2 = assign61230_e95421_d_n2;
        var_alpha_dn4 = assign61230_e95421_d_n4;
        var_alpha_dn5 = assign61230_e95421_d_n5;
        var_alpha_dn6 = assign61230_e95421_d_n6;
        var_alpha_dn7 = assign61230_e95421_d_n7;
        var_alpha_dn8 = assign61230_e95421_d_n8;
        var_alpha_dn9 = assign61230_e95421_d_n9;
        var_alpha_dn10 = assign61230_e95421_d_n10;
        var_alpha_dn13 = assign61230_e95421_d_n13;
        var_alpha_rv = 0.0;

        let (assign61240_e95436, assign61240_e95436_d_n0, assign61240_e95436_d_n2, assign61240_e95436_d_n4, assign61240_e95436_d_n5, assign61240_e95436_d_n6, assign61240_e95436_d_n7, assign61240_e95436_d_n8, assign61240_e95436_d_n9, assign61240_e95436_d_n10, assign61240_e95436_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign61240_e95432: f64 = (1.0 + var_alpha);
        let assign61240_e95433: f64 = (var_alpha * assign61240_e95432);
        let assign61240_e95434: f64 = (1.0 + assign61240_e95433);
        (assign61240_e95434, ((var_alpha_dn0 * assign61240_e95432) + (var_alpha * var_alpha_dn0)), ((var_alpha_dn2 * assign61240_e95432) + (var_alpha * var_alpha_dn2)), ((var_alpha_dn4 * assign61240_e95432) + (var_alpha * var_alpha_dn4)), ((var_alpha_dn5 * assign61240_e95432) + (var_alpha * var_alpha_dn5)), ((var_alpha_dn6 * assign61240_e95432) + (var_alpha * var_alpha_dn6)), ((var_alpha_dn7 * assign61240_e95432) + (var_alpha * var_alpha_dn7)), ((var_alpha_dn8 * assign61240_e95432) + (var_alpha * var_alpha_dn8)), ((var_alpha_dn9 * assign61240_e95432) + (var_alpha * var_alpha_dn9)), ((var_alpha_dn10 * assign61240_e95432) + (var_alpha * var_alpha_dn10)), ((var_alpha_dn13 * assign61240_e95432) + (var_alpha * var_alpha_dn13)),)
    } else {
        (var_qinm, var_qinm_dn0, var_qinm_dn2, var_qinm_dn4, var_qinm_dn5, var_qinm_dn6, var_qinm_dn7, var_qinm_dn8, var_qinm_dn9, var_qinm_dn10, var_qinm_dn13,)
    }
};
        var_qinm = assign61240_e95436;
        var_qinm_dn0 = assign61240_e95436_d_n0;
        var_qinm_dn2 = assign61240_e95436_d_n2;
        var_qinm_dn4 = assign61240_e95436_d_n4;
        var_qinm_dn5 = assign61240_e95436_d_n5;
        var_qinm_dn6 = assign61240_e95436_d_n6;
        var_qinm_dn7 = assign61240_e95436_d_n7;
        var_qinm_dn8 = assign61240_e95436_d_n8;
        var_qinm_dn9 = assign61240_e95436_d_n9;
        var_qinm_dn10 = assign61240_e95436_d_n10;
        var_qinm_dn13 = assign61240_e95436_d_n13;
        var_qinm_rv = 0.0;

        *var_alpha_slot = var_alpha;
        *var_alpha_dn0_slot = var_alpha_dn0;
        *var_alpha_dn10_slot = var_alpha_dn10;
        *var_alpha_dn13_slot = var_alpha_dn13;
        *var_alpha_dn2_slot = var_alpha_dn2;
        *var_alpha_dn4_slot = var_alpha_dn4;
        *var_alpha_dn5_slot = var_alpha_dn5;
        *var_alpha_dn6_slot = var_alpha_dn6;
        *var_alpha_dn7_slot = var_alpha_dn7;
        *var_alpha_dn8_slot = var_alpha_dn8;
        *var_alpha_dn9_slot = var_alpha_dn9;
        *var_alpha_rv_slot = var_alpha_rv;
        *var_arg_slot = var_arg;
        *var_arg_dn0_slot = var_arg_dn0;
        *var_arg_dn10_slot = var_arg_dn10;
        *var_arg_dn13_slot = var_arg_dn13;
        *var_arg_dn2_slot = var_arg_dn2;
        *var_arg_dn4_slot = var_arg_dn4;
        *var_arg_dn5_slot = var_arg_dn5;
        *var_arg_dn6_slot = var_arg_dn6;
        *var_arg_dn7_slot = var_arg_dn7;
        *var_arg_dn8_slot = var_arg_dn8;
        *var_arg_dn9_slot = var_arg_dn9;
        *var_arg_rv_slot = var_arg_rv;
        *var_dnm_slot = var_dnm;
        *var_dnm_dn0_slot = var_dnm_dn0;
        *var_dnm_dn10_slot = var_dnm_dn10;
        *var_dnm_dn13_slot = var_dnm_dn13;
        *var_dnm_dn2_slot = var_dnm_dn2;
        *var_dnm_dn4_slot = var_dnm_dn4;
        *var_dnm_dn5_slot = var_dnm_dn5;
        *var_dnm_dn6_slot = var_dnm_dn6;
        *var_dnm_dn7_slot = var_dnm_dn7;
        *var_dnm_dn8_slot = var_dnm_dn8;
        *var_dnm_dn9_slot = var_dnm_dn9;
        *var_dnm_rv_slot = var_dnm_rv;
        *var_guard1475_slot = var_guard1475;
        *var_guard1475_rv_slot = var_guard1475_rv;
        *var_guard1476_slot = var_guard1476;
        *var_guard1476_rv_slot = var_guard1476_rv;
        *var_guard1477_slot = var_guard1477;
        *var_guard1477_rv_slot = var_guard1477_rv;
        *var_guard1478_slot = var_guard1478;
        *var_guard1478_rv_slot = var_guard1478_rv;
        *var_guard1479_slot = var_guard1479;
        *var_guard1479_rv_slot = var_guard1479_rv;
        *var_m0_slot = var_m0;
        *var_m0_rv_slot = var_m0_rv;
        *var_mm_slot = var_mm;
        *var_mm_rv_slot = var_mm_rv;
        *var_qinm_slot = var_qinm;
        *var_qinm_dn0_slot = var_qinm_dn0;
        *var_qinm_dn10_slot = var_qinm_dn10;
        *var_qinm_dn13_slot = var_qinm_dn13;
        *var_qinm_dn2_slot = var_qinm_dn2;
        *var_qinm_dn4_slot = var_qinm_dn4;
        *var_qinm_dn5_slot = var_qinm_dn5;
        *var_qinm_dn6_slot = var_qinm_dn6;
        *var_qinm_dn7_slot = var_qinm_dn7;
        *var_qinm_dn8_slot = var_qinm_dn8;
        *var_qinm_dn9_slot = var_qinm_dn9;
        *var_qinm_rv_slot = var_qinm_rv;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn13_slot = var_t4_dn13;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_dn9_slot = var_t4_dn9;
        *var_t4_rv_slot = var_t4_rv;
        *var_ty_slot = var_ty;
        *var_ty_dn0_slot = var_ty_dn0;
        *var_ty_dn10_slot = var_ty_dn10;
        *var_ty_dn13_slot = var_ty_dn13;
        *var_ty_dn2_slot = var_ty_dn2;
        *var_ty_dn4_slot = var_ty_dn4;
        *var_ty_dn5_slot = var_ty_dn5;
        *var_ty_dn6_slot = var_ty_dn6;
        *var_ty_dn7_slot = var_ty_dn7;
        *var_ty_dn8_slot = var_ty_dn8;
        *var_ty_dn9_slot = var_ty_dn9;
        *var_ty_rv_slot = var_ty_rv;
        *var_xmp_slot = var_xmp;
        *var_xmp_dn0_slot = var_xmp_dn0;
        *var_xmp_dn10_slot = var_xmp_dn10;
        *var_xmp_dn13_slot = var_xmp_dn13;
        *var_xmp_dn2_slot = var_xmp_dn2;
        *var_xmp_dn4_slot = var_xmp_dn4;
        *var_xmp_dn5_slot = var_xmp_dn5;
        *var_xmp_dn6_slot = var_xmp_dn6;
        *var_xmp_dn7_slot = var_xmp_dn7;
        *var_xmp_dn8_slot = var_xmp_dn8;
        *var_xmp_dn9_slot = var_xmp_dn9;
        *var_xmp_rv_slot = var_xmp_rv;
        *var_xp_slot = var_xp;
        *var_xp_dn0_slot = var_xp_dn0;
        *var_xp_dn10_slot = var_xp_dn10;
        *var_xp_dn13_slot = var_xp_dn13;
        *var_xp_dn2_slot = var_xp_dn2;
        *var_xp_dn4_slot = var_xp_dn4;
        *var_xp_dn5_slot = var_xp_dn5;
        *var_xp_dn6_slot = var_xp_dn6;
        *var_xp_dn7_slot = var_xp_dn7;
        *var_xp_dn8_slot = var_xp_dn8;
        *var_xp_dn9_slot = var_xp_dn9;
        *var_xp_rv_slot = var_xp_rv;
    }

    pub(super) fn stamp_reactive_block_219(
        var_alpha: f64,
        var_alpha_dn0: f64,
        var_alpha_dn10: f64,
        var_alpha_dn13: f64,
        var_alpha_dn2: f64,
        var_alpha_dn4: f64,
        var_alpha_dn5: f64,
        var_alpha_dn6: f64,
        var_alpha_dn7: f64,
        var_alpha_dn8: f64,
        var_alpha_dn9: f64,
        var_guard1430: f64,
        var_guard1461: f64,
        var_guard443: f64,
        var_arg_slot: &mut f64,
        var_arg_dn0_slot: &mut f64,
        var_arg_dn10_slot: &mut f64,
        var_arg_dn13_slot: &mut f64,
        var_arg_dn2_slot: &mut f64,
        var_arg_dn4_slot: &mut f64,
        var_arg_dn5_slot: &mut f64,
        var_arg_dn6_slot: &mut f64,
        var_arg_dn7_slot: &mut f64,
        var_arg_dn8_slot: &mut f64,
        var_arg_dn9_slot: &mut f64,
        var_arg_rv_slot: &mut f64,
        var_dnm_slot: &mut f64,
        var_dnm_dn0_slot: &mut f64,
        var_dnm_dn10_slot: &mut f64,
        var_dnm_dn13_slot: &mut f64,
        var_dnm_dn2_slot: &mut f64,
        var_dnm_dn4_slot: &mut f64,
        var_dnm_dn5_slot: &mut f64,
        var_dnm_dn6_slot: &mut f64,
        var_dnm_dn7_slot: &mut f64,
        var_dnm_dn8_slot: &mut f64,
        var_dnm_dn9_slot: &mut f64,
        var_dnm_rv_slot: &mut f64,
        var_guard1480_slot: &mut f64,
        var_guard1480_rv_slot: &mut f64,
        var_guard1481_slot: &mut f64,
        var_guard1481_rv_slot: &mut f64,
        var_guard1482_slot: &mut f64,
        var_guard1482_rv_slot: &mut f64,
        var_guard1483_slot: &mut f64,
        var_guard1483_rv_slot: &mut f64,
        var_guard1484_slot: &mut f64,
        var_guard1484_rv_slot: &mut f64,
        var_guard1485_slot: &mut f64,
        var_guard1485_rv_slot: &mut f64,
        var_m0_slot: &mut f64,
        var_m0_rv_slot: &mut f64,
        var_mm_slot: &mut f64,
        var_mm_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn13_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_tmf0_slot: &mut f64,
        var_tmf0_dn0_slot: &mut f64,
        var_tmf0_dn10_slot: &mut f64,
        var_tmf0_dn13_slot: &mut f64,
        var_tmf0_dn2_slot: &mut f64,
        var_tmf0_dn4_slot: &mut f64,
        var_tmf0_dn5_slot: &mut f64,
        var_tmf0_dn6_slot: &mut f64,
        var_tmf0_dn7_slot: &mut f64,
        var_tmf0_dn8_slot: &mut f64,
        var_tmf0_dn9_slot: &mut f64,
        var_tmf0_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn13_slot: &mut f64,
        var_tmf1_dn2_slot: &mut f64,
        var_tmf1_dn4_slot: &mut f64,
        var_tmf1_dn5_slot: &mut f64,
        var_tmf1_dn6_slot: &mut f64,
        var_tmf1_dn7_slot: &mut f64,
        var_tmf1_dn8_slot: &mut f64,
        var_tmf1_dn9_slot: &mut f64,
        var_tmf1_rv_slot: &mut f64,
        var_x2_slot: &mut f64,
        var_x2_dn0_slot: &mut f64,
        var_x2_dn10_slot: &mut f64,
        var_x2_dn13_slot: &mut f64,
        var_x2_dn2_slot: &mut f64,
        var_x2_dn4_slot: &mut f64,
        var_x2_dn5_slot: &mut f64,
        var_x2_dn6_slot: &mut f64,
        var_x2_dn7_slot: &mut f64,
        var_x2_dn8_slot: &mut f64,
        var_x2_dn9_slot: &mut f64,
        var_x2_rv_slot: &mut f64,
        var_xmax2_slot: &mut f64,
        var_xmax2_dn0_slot: &mut f64,
        var_xmax2_dn10_slot: &mut f64,
        var_xmax2_dn13_slot: &mut f64,
        var_xmax2_dn2_slot: &mut f64,
        var_xmax2_dn4_slot: &mut f64,
        var_xmax2_dn5_slot: &mut f64,
        var_xmax2_dn6_slot: &mut f64,
        var_xmax2_dn7_slot: &mut f64,
        var_xmax2_dn8_slot: &mut f64,
        var_xmax2_dn9_slot: &mut f64,
        var_xmax2_rv_slot: &mut f64,
        var_xmp_slot: &mut f64,
        var_xmp_dn0_slot: &mut f64,
        var_xmp_dn10_slot: &mut f64,
        var_xmp_dn13_slot: &mut f64,
        var_xmp_dn2_slot: &mut f64,
        var_xmp_dn4_slot: &mut f64,
        var_xmp_dn5_slot: &mut f64,
        var_xmp_dn6_slot: &mut f64,
        var_xmp_dn7_slot: &mut f64,
        var_xmp_dn8_slot: &mut f64,
        var_xmp_dn9_slot: &mut f64,
        var_xmp_rv_slot: &mut f64,
        var_xp_slot: &mut f64,
        var_xp_dn0_slot: &mut f64,
        var_xp_dn10_slot: &mut f64,
        var_xp_dn13_slot: &mut f64,
        var_xp_dn2_slot: &mut f64,
        var_xp_dn4_slot: &mut f64,
        var_xp_dn5_slot: &mut f64,
        var_xp_dn6_slot: &mut f64,
        var_xp_dn7_slot: &mut f64,
        var_xp_dn8_slot: &mut f64,
        var_xp_dn9_slot: &mut f64,
        var_xp_rv_slot: &mut f64,
    ) {
        let mut var_arg: f64 = *var_arg_slot;
        let mut var_arg_dn0: f64 = *var_arg_dn0_slot;
        let mut var_arg_dn10: f64 = *var_arg_dn10_slot;
        let mut var_arg_dn13: f64 = *var_arg_dn13_slot;
        let mut var_arg_dn2: f64 = *var_arg_dn2_slot;
        let mut var_arg_dn4: f64 = *var_arg_dn4_slot;
        let mut var_arg_dn5: f64 = *var_arg_dn5_slot;
        let mut var_arg_dn6: f64 = *var_arg_dn6_slot;
        let mut var_arg_dn7: f64 = *var_arg_dn7_slot;
        let mut var_arg_dn8: f64 = *var_arg_dn8_slot;
        let mut var_arg_dn9: f64 = *var_arg_dn9_slot;
        let mut var_arg_rv: f64 = *var_arg_rv_slot;
        let mut var_dnm: f64 = *var_dnm_slot;
        let mut var_dnm_dn0: f64 = *var_dnm_dn0_slot;
        let mut var_dnm_dn10: f64 = *var_dnm_dn10_slot;
        let mut var_dnm_dn13: f64 = *var_dnm_dn13_slot;
        let mut var_dnm_dn2: f64 = *var_dnm_dn2_slot;
        let mut var_dnm_dn4: f64 = *var_dnm_dn4_slot;
        let mut var_dnm_dn5: f64 = *var_dnm_dn5_slot;
        let mut var_dnm_dn6: f64 = *var_dnm_dn6_slot;
        let mut var_dnm_dn7: f64 = *var_dnm_dn7_slot;
        let mut var_dnm_dn8: f64 = *var_dnm_dn8_slot;
        let mut var_dnm_dn9: f64 = *var_dnm_dn9_slot;
        let mut var_dnm_rv: f64 = *var_dnm_rv_slot;
        let mut var_guard1480: f64 = *var_guard1480_slot;
        let mut var_guard1480_rv: f64 = *var_guard1480_rv_slot;
        let mut var_guard1481: f64 = *var_guard1481_slot;
        let mut var_guard1481_rv: f64 = *var_guard1481_rv_slot;
        let mut var_guard1482: f64 = *var_guard1482_slot;
        let mut var_guard1482_rv: f64 = *var_guard1482_rv_slot;
        let mut var_guard1483: f64 = *var_guard1483_slot;
        let mut var_guard1483_rv: f64 = *var_guard1483_rv_slot;
        let mut var_guard1484: f64 = *var_guard1484_slot;
        let mut var_guard1484_rv: f64 = *var_guard1484_rv_slot;
        let mut var_guard1485: f64 = *var_guard1485_slot;
        let mut var_guard1485_rv: f64 = *var_guard1485_rv_slot;
        let mut var_m0: f64 = *var_m0_slot;
        let mut var_m0_rv: f64 = *var_m0_rv_slot;
        let mut var_mm: f64 = *var_mm_slot;
        let mut var_mm_rv: f64 = *var_mm_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn13: f64 = *var_t0_dn13_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_tmf0: f64 = *var_tmf0_slot;
        let mut var_tmf0_dn0: f64 = *var_tmf0_dn0_slot;
        let mut var_tmf0_dn10: f64 = *var_tmf0_dn10_slot;
        let mut var_tmf0_dn13: f64 = *var_tmf0_dn13_slot;
        let mut var_tmf0_dn2: f64 = *var_tmf0_dn2_slot;
        let mut var_tmf0_dn4: f64 = *var_tmf0_dn4_slot;
        let mut var_tmf0_dn5: f64 = *var_tmf0_dn5_slot;
        let mut var_tmf0_dn6: f64 = *var_tmf0_dn6_slot;
        let mut var_tmf0_dn7: f64 = *var_tmf0_dn7_slot;
        let mut var_tmf0_dn8: f64 = *var_tmf0_dn8_slot;
        let mut var_tmf0_dn9: f64 = *var_tmf0_dn9_slot;
        let mut var_tmf0_rv: f64 = *var_tmf0_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn13: f64 = *var_tmf1_dn13_slot;
        let mut var_tmf1_dn2: f64 = *var_tmf1_dn2_slot;
        let mut var_tmf1_dn4: f64 = *var_tmf1_dn4_slot;
        let mut var_tmf1_dn5: f64 = *var_tmf1_dn5_slot;
        let mut var_tmf1_dn6: f64 = *var_tmf1_dn6_slot;
        let mut var_tmf1_dn7: f64 = *var_tmf1_dn7_slot;
        let mut var_tmf1_dn8: f64 = *var_tmf1_dn8_slot;
        let mut var_tmf1_dn9: f64 = *var_tmf1_dn9_slot;
        let mut var_tmf1_rv: f64 = *var_tmf1_rv_slot;
        let mut var_x2: f64 = *var_x2_slot;
        let mut var_x2_dn0: f64 = *var_x2_dn0_slot;
        let mut var_x2_dn10: f64 = *var_x2_dn10_slot;
        let mut var_x2_dn13: f64 = *var_x2_dn13_slot;
        let mut var_x2_dn2: f64 = *var_x2_dn2_slot;
        let mut var_x2_dn4: f64 = *var_x2_dn4_slot;
        let mut var_x2_dn5: f64 = *var_x2_dn5_slot;
        let mut var_x2_dn6: f64 = *var_x2_dn6_slot;
        let mut var_x2_dn7: f64 = *var_x2_dn7_slot;
        let mut var_x2_dn8: f64 = *var_x2_dn8_slot;
        let mut var_x2_dn9: f64 = *var_x2_dn9_slot;
        let mut var_x2_rv: f64 = *var_x2_rv_slot;
        let mut var_xmax2: f64 = *var_xmax2_slot;
        let mut var_xmax2_dn0: f64 = *var_xmax2_dn0_slot;
        let mut var_xmax2_dn10: f64 = *var_xmax2_dn10_slot;
        let mut var_xmax2_dn13: f64 = *var_xmax2_dn13_slot;
        let mut var_xmax2_dn2: f64 = *var_xmax2_dn2_slot;
        let mut var_xmax2_dn4: f64 = *var_xmax2_dn4_slot;
        let mut var_xmax2_dn5: f64 = *var_xmax2_dn5_slot;
        let mut var_xmax2_dn6: f64 = *var_xmax2_dn6_slot;
        let mut var_xmax2_dn7: f64 = *var_xmax2_dn7_slot;
        let mut var_xmax2_dn8: f64 = *var_xmax2_dn8_slot;
        let mut var_xmax2_dn9: f64 = *var_xmax2_dn9_slot;
        let mut var_xmax2_rv: f64 = *var_xmax2_rv_slot;
        let mut var_xmp: f64 = *var_xmp_slot;
        let mut var_xmp_dn0: f64 = *var_xmp_dn0_slot;
        let mut var_xmp_dn10: f64 = *var_xmp_dn10_slot;
        let mut var_xmp_dn13: f64 = *var_xmp_dn13_slot;
        let mut var_xmp_dn2: f64 = *var_xmp_dn2_slot;
        let mut var_xmp_dn4: f64 = *var_xmp_dn4_slot;
        let mut var_xmp_dn5: f64 = *var_xmp_dn5_slot;
        let mut var_xmp_dn6: f64 = *var_xmp_dn6_slot;
        let mut var_xmp_dn7: f64 = *var_xmp_dn7_slot;
        let mut var_xmp_dn8: f64 = *var_xmp_dn8_slot;
        let mut var_xmp_dn9: f64 = *var_xmp_dn9_slot;
        let mut var_xmp_rv: f64 = *var_xmp_rv_slot;
        let mut var_xp: f64 = *var_xp_slot;
        let mut var_xp_dn0: f64 = *var_xp_dn0_slot;
        let mut var_xp_dn10: f64 = *var_xp_dn10_slot;
        let mut var_xp_dn13: f64 = *var_xp_dn13_slot;
        let mut var_xp_dn2: f64 = *var_xp_dn2_slot;
        let mut var_xp_dn4: f64 = *var_xp_dn4_slot;
        let mut var_xp_dn5: f64 = *var_xp_dn5_slot;
        let mut var_xp_dn6: f64 = *var_xp_dn6_slot;
        let mut var_xp_dn7: f64 = *var_xp_dn7_slot;
        let mut var_xp_dn8: f64 = *var_xp_dn8_slot;
        let mut var_xp_dn9: f64 = *var_xp_dn9_slot;
        let mut var_xp_rv: f64 = *var_xp_rv_slot;

        let assign61250_e95439: f64 = (1.0 + var_alpha);
        let assign61250_e95442: f64 = (10.0 * 2.220446049250313e-16);
        let assign61250_e95445: f64 = (10.0 * 2.220446049250313e-16);
        let assign61250_e95446: f64 = (assign61250_e95442 + assign61250_e95445);
        let assign61250_e95450: f64 = (10.0 * 2.220446049250313e-16);
        let assign61250_e95453: f64 = if ((assign61250_e95439 < assign61250_e95446) && (assign61250_e95450 >= 0.0)) { 1.0 } else { 0.0 };
        var_guard1480 = assign61250_e95453;
        var_guard1480_rv = 0.0;

        let (assign61260_e95474, assign61260_e95474_d_n0, assign61260_e95474_d_n2, assign61260_e95474_d_n4, assign61260_e95474_d_n5, assign61260_e95474_d_n6, assign61260_e95474_d_n7, assign61260_e95474_d_n8, assign61260_e95474_d_n9, assign61260_e95474_d_n10, assign61260_e95474_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 != 0.0)) {
        let assign61260_e95464: f64 = (10.0 * 2.220446049250313e-16);
        let assign61260_e95467: f64 = (10.0 * 2.220446049250313e-16);
        let assign61260_e95468: f64 = (assign61260_e95464 + assign61260_e95467);
        let assign61260_e95471: f64 = (1.0 + var_alpha);
        let assign61260_e95472: f64 = (assign61260_e95468 - assign61260_e95471);
        (assign61260_e95472, (-var_alpha_dn0), (-var_alpha_dn2), (-var_alpha_dn4), (-var_alpha_dn5), (-var_alpha_dn6), (-var_alpha_dn7), (-var_alpha_dn8), (-var_alpha_dn9), (-var_alpha_dn10), (-var_alpha_dn13),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn13,)
    }
};
        var_tmf1 = assign61260_e95474;
        var_tmf1_dn0 = assign61260_e95474_d_n0;
        var_tmf1_dn2 = assign61260_e95474_d_n2;
        var_tmf1_dn4 = assign61260_e95474_d_n4;
        var_tmf1_dn5 = assign61260_e95474_d_n5;
        var_tmf1_dn6 = assign61260_e95474_d_n6;
        var_tmf1_dn7 = assign61260_e95474_d_n7;
        var_tmf1_dn8 = assign61260_e95474_d_n8;
        var_tmf1_dn9 = assign61260_e95474_d_n9;
        var_tmf1_dn10 = assign61260_e95474_d_n10;
        var_tmf1_dn13 = assign61260_e95474_d_n13;
        var_tmf1_rv = 0.0;

        let (assign61270_e95487, assign61270_e95487_d_n0, assign61270_e95487_d_n2, assign61270_e95487_d_n4, assign61270_e95487_d_n5, assign61270_e95487_d_n6, assign61270_e95487_d_n7, assign61270_e95487_d_n8, assign61270_e95487_d_n9, assign61270_e95487_d_n10, assign61270_e95487_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 != 0.0)) {
        let assign61270_e95485: f64 = (var_tmf1 * var_tmf1);
        (assign61270_e95485, ((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)), ((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)), ((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)), ((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)), ((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)), ((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)), ((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)), ((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)), ((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)), ((var_tmf1_dn13 * var_tmf1) + (var_tmf1 * var_tmf1_dn13)),)
    } else {
        (var_x2, var_x2_dn0, var_x2_dn2, var_x2_dn4, var_x2_dn5, var_x2_dn6, var_x2_dn7, var_x2_dn8, var_x2_dn9, var_x2_dn10, var_x2_dn13,)
    }
};
        var_x2 = assign61270_e95487;
        var_x2_dn0 = assign61270_e95487_d_n0;
        var_x2_dn2 = assign61270_e95487_d_n2;
        var_x2_dn4 = assign61270_e95487_d_n4;
        var_x2_dn5 = assign61270_e95487_d_n5;
        var_x2_dn6 = assign61270_e95487_d_n6;
        var_x2_dn7 = assign61270_e95487_d_n7;
        var_x2_dn8 = assign61270_e95487_d_n8;
        var_x2_dn9 = assign61270_e95487_d_n9;
        var_x2_dn10 = assign61270_e95487_d_n10;
        var_x2_dn13 = assign61270_e95487_d_n13;
        var_x2_rv = 0.0;

        let (assign61280_e95504, assign61280_e95504_d_n0, assign61280_e95504_d_n2, assign61280_e95504_d_n4, assign61280_e95504_d_n5, assign61280_e95504_d_n6, assign61280_e95504_d_n7, assign61280_e95504_d_n8, assign61280_e95504_d_n9, assign61280_e95504_d_n10, assign61280_e95504_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 != 0.0)) {
        let assign61280_e95498: f64 = (10.0 * 2.220446049250313e-16);
        let assign61280_e95501: f64 = (10.0 * 2.220446049250313e-16);
        let assign61280_e95502: f64 = (assign61280_e95498 * assign61280_e95501);
        (assign61280_e95502, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmax2, var_xmax2_dn0, var_xmax2_dn2, var_xmax2_dn4, var_xmax2_dn5, var_xmax2_dn6, var_xmax2_dn7, var_xmax2_dn8, var_xmax2_dn9, var_xmax2_dn10, var_xmax2_dn13,)
    }
};
        var_xmax2 = assign61280_e95504;
        var_xmax2_dn0 = assign61280_e95504_d_n0;
        var_xmax2_dn2 = assign61280_e95504_d_n2;
        var_xmax2_dn4 = assign61280_e95504_d_n4;
        var_xmax2_dn5 = assign61280_e95504_d_n5;
        var_xmax2_dn6 = assign61280_e95504_d_n6;
        var_xmax2_dn7 = assign61280_e95504_d_n7;
        var_xmax2_dn8 = assign61280_e95504_d_n8;
        var_xmax2_dn9 = assign61280_e95504_d_n9;
        var_xmax2_dn10 = assign61280_e95504_d_n10;
        var_xmax2_dn13 = assign61280_e95504_d_n13;
        var_xmax2_rv = 0.0;

        let (assign61290_e95515, assign61290_e95515_d_n0, assign61290_e95515_d_n2, assign61290_e95515_d_n4, assign61290_e95515_d_n5, assign61290_e95515_d_n6, assign61290_e95515_d_n7, assign61290_e95515_d_n8, assign61290_e95515_d_n9, assign61290_e95515_d_n10, assign61290_e95515_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn7, var_xp_dn8, var_xp_dn9, var_xp_dn10, var_xp_dn13,)
    }
};
        var_xp = assign61290_e95515;
        var_xp_dn0 = assign61290_e95515_d_n0;
        var_xp_dn2 = assign61290_e95515_d_n2;
        var_xp_dn4 = assign61290_e95515_d_n4;
        var_xp_dn5 = assign61290_e95515_d_n5;
        var_xp_dn6 = assign61290_e95515_d_n6;
        var_xp_dn7 = assign61290_e95515_d_n7;
        var_xp_dn8 = assign61290_e95515_d_n8;
        var_xp_dn9 = assign61290_e95515_d_n9;
        var_xp_dn10 = assign61290_e95515_d_n10;
        var_xp_dn13 = assign61290_e95515_d_n13;
        var_xp_rv = 0.0;

        let (assign61300_e95526, assign61300_e95526_d_n0, assign61300_e95526_d_n2, assign61300_e95526_d_n4, assign61300_e95526_d_n5, assign61300_e95526_d_n6, assign61300_e95526_d_n7, assign61300_e95526_d_n8, assign61300_e95526_d_n9, assign61300_e95526_d_n10, assign61300_e95526_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn7, var_xmp_dn8, var_xmp_dn9, var_xmp_dn10, var_xmp_dn13,)
    }
};
        var_xmp = assign61300_e95526;
        var_xmp_dn0 = assign61300_e95526_d_n0;
        var_xmp_dn2 = assign61300_e95526_d_n2;
        var_xmp_dn4 = assign61300_e95526_d_n4;
        var_xmp_dn5 = assign61300_e95526_d_n5;
        var_xmp_dn6 = assign61300_e95526_d_n6;
        var_xmp_dn7 = assign61300_e95526_d_n7;
        var_xmp_dn8 = assign61300_e95526_d_n8;
        var_xmp_dn9 = assign61300_e95526_d_n9;
        var_xmp_dn10 = assign61300_e95526_d_n10;
        var_xmp_dn13 = assign61300_e95526_d_n13;
        var_xmp_rv = 0.0;

        let (assign61310_e95537,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign61310_e95537;
        var_m0_rv = 0.0;

        let (assign61320_e95548,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 != 0.0)) {
        (0.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign61320_e95548;
        var_mm_rv = 0.0;

        let (assign61330_e95559, assign61330_e95559_d_n0, assign61330_e95559_d_n2, assign61330_e95559_d_n4, assign61330_e95559_d_n5, assign61330_e95559_d_n6, assign61330_e95559_d_n7, assign61330_e95559_d_n8, assign61330_e95559_d_n9, assign61330_e95559_d_n10, assign61330_e95559_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_dn10, var_arg_dn13,)
    }
};
        var_arg = assign61330_e95559;
        var_arg_dn0 = assign61330_e95559_d_n0;
        var_arg_dn2 = assign61330_e95559_d_n2;
        var_arg_dn4 = assign61330_e95559_d_n4;
        var_arg_dn5 = assign61330_e95559_d_n5;
        var_arg_dn6 = assign61330_e95559_d_n6;
        var_arg_dn7 = assign61330_e95559_d_n7;
        var_arg_dn8 = assign61330_e95559_d_n8;
        var_arg_dn9 = assign61330_e95559_d_n9;
        var_arg_dn10 = assign61330_e95559_d_n10;
        var_arg_dn13 = assign61330_e95559_d_n13;
        var_arg_rv = 0.0;

        let (assign61340_e95570, assign61340_e95570_d_n0, assign61340_e95570_d_n2, assign61340_e95570_d_n4, assign61340_e95570_d_n5, assign61340_e95570_d_n6, assign61340_e95570_d_n7, assign61340_e95570_d_n8, assign61340_e95570_d_n9, assign61340_e95570_d_n10, assign61340_e95570_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
        var_dnm = assign61340_e95570;
        var_dnm_dn0 = assign61340_e95570_d_n0;
        var_dnm_dn2 = assign61340_e95570_d_n2;
        var_dnm_dn4 = assign61340_e95570_d_n4;
        var_dnm_dn5 = assign61340_e95570_d_n5;
        var_dnm_dn6 = assign61340_e95570_d_n6;
        var_dnm_dn7 = assign61340_e95570_d_n7;
        var_dnm_dn8 = assign61340_e95570_d_n8;
        var_dnm_dn9 = assign61340_e95570_d_n9;
        var_dnm_dn10 = assign61340_e95570_d_n10;
        var_dnm_dn13 = assign61340_e95570_d_n13;
        var_dnm_rv = 0.0;

        let (assign61350_e95583, assign61350_e95583_d_n0, assign61350_e95583_d_n2, assign61350_e95583_d_n4, assign61350_e95583_d_n5, assign61350_e95583_d_n6, assign61350_e95583_d_n7, assign61350_e95583_d_n8, assign61350_e95583_d_n9, assign61350_e95583_d_n10, assign61350_e95583_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 != 0.0)) {
        let assign61350_e95581: f64 = (var_xp * var_x2);
        (assign61350_e95581, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn4 * var_x2) + (var_xp * var_x2_dn4)), ((var_xp_dn5 * var_x2) + (var_xp * var_x2_dn5)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn8 * var_x2) + (var_xp * var_x2_dn8)), ((var_xp_dn9 * var_x2) + (var_xp * var_x2_dn9)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn13 * var_x2) + (var_xp * var_x2_dn13)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn7, var_xp_dn8, var_xp_dn9, var_xp_dn10, var_xp_dn13,)
    }
};
        var_xp = assign61350_e95583;
        var_xp_dn0 = assign61350_e95583_d_n0;
        var_xp_dn2 = assign61350_e95583_d_n2;
        var_xp_dn4 = assign61350_e95583_d_n4;
        var_xp_dn5 = assign61350_e95583_d_n5;
        var_xp_dn6 = assign61350_e95583_d_n6;
        var_xp_dn7 = assign61350_e95583_d_n7;
        var_xp_dn8 = assign61350_e95583_d_n8;
        var_xp_dn9 = assign61350_e95583_d_n9;
        var_xp_dn10 = assign61350_e95583_d_n10;
        var_xp_dn13 = assign61350_e95583_d_n13;
        var_xp_rv = 0.0;

        let (assign61360_e95596, assign61360_e95596_d_n0, assign61360_e95596_d_n2, assign61360_e95596_d_n4, assign61360_e95596_d_n5, assign61360_e95596_d_n6, assign61360_e95596_d_n7, assign61360_e95596_d_n8, assign61360_e95596_d_n9, assign61360_e95596_d_n10, assign61360_e95596_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 != 0.0)) {
        let assign61360_e95594: f64 = (var_xmp * var_xmax2);
        (assign61360_e95594, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn4 * var_xmax2) + (var_xmp * var_xmax2_dn4)), ((var_xmp_dn5 * var_xmax2) + (var_xmp * var_xmax2_dn5)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn8 * var_xmax2) + (var_xmp * var_xmax2_dn8)), ((var_xmp_dn9 * var_xmax2) + (var_xmp * var_xmax2_dn9)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn13 * var_xmax2) + (var_xmp * var_xmax2_dn13)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn7, var_xmp_dn8, var_xmp_dn9, var_xmp_dn10, var_xmp_dn13,)
    }
};
        var_xmp = assign61360_e95596;
        var_xmp_dn0 = assign61360_e95596_d_n0;
        var_xmp_dn2 = assign61360_e95596_d_n2;
        var_xmp_dn4 = assign61360_e95596_d_n4;
        var_xmp_dn5 = assign61360_e95596_d_n5;
        var_xmp_dn6 = assign61360_e95596_d_n6;
        var_xmp_dn7 = assign61360_e95596_d_n7;
        var_xmp_dn8 = assign61360_e95596_d_n8;
        var_xmp_dn9 = assign61360_e95596_d_n9;
        var_xmp_dn10 = assign61360_e95596_d_n10;
        var_xmp_dn13 = assign61360_e95596_d_n13;
        var_xmp_rv = 0.0;

        let (assign61370_e95609, assign61370_e95609_d_n0, assign61370_e95609_d_n2, assign61370_e95609_d_n4, assign61370_e95609_d_n5, assign61370_e95609_d_n6, assign61370_e95609_d_n7, assign61370_e95609_d_n8, assign61370_e95609_d_n9, assign61370_e95609_d_n10, assign61370_e95609_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 != 0.0)) {
        let assign61370_e95607: f64 = (var_xp * var_x2);
        (assign61370_e95607, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn4 * var_x2) + (var_xp * var_x2_dn4)), ((var_xp_dn5 * var_x2) + (var_xp * var_x2_dn5)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn8 * var_x2) + (var_xp * var_x2_dn8)), ((var_xp_dn9 * var_x2) + (var_xp * var_x2_dn9)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn13 * var_x2) + (var_xp * var_x2_dn13)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn7, var_xp_dn8, var_xp_dn9, var_xp_dn10, var_xp_dn13,)
    }
};
        var_xp = assign61370_e95609;
        var_xp_dn0 = assign61370_e95609_d_n0;
        var_xp_dn2 = assign61370_e95609_d_n2;
        var_xp_dn4 = assign61370_e95609_d_n4;
        var_xp_dn5 = assign61370_e95609_d_n5;
        var_xp_dn6 = assign61370_e95609_d_n6;
        var_xp_dn7 = assign61370_e95609_d_n7;
        var_xp_dn8 = assign61370_e95609_d_n8;
        var_xp_dn9 = assign61370_e95609_d_n9;
        var_xp_dn10 = assign61370_e95609_d_n10;
        var_xp_dn13 = assign61370_e95609_d_n13;
        var_xp_rv = 0.0;

        let (assign61380_e95622, assign61380_e95622_d_n0, assign61380_e95622_d_n2, assign61380_e95622_d_n4, assign61380_e95622_d_n5, assign61380_e95622_d_n6, assign61380_e95622_d_n7, assign61380_e95622_d_n8, assign61380_e95622_d_n9, assign61380_e95622_d_n10, assign61380_e95622_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 != 0.0)) {
        let assign61380_e95620: f64 = (var_xmp * var_xmax2);
        (assign61380_e95620, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn4 * var_xmax2) + (var_xmp * var_xmax2_dn4)), ((var_xmp_dn5 * var_xmax2) + (var_xmp * var_xmax2_dn5)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn8 * var_xmax2) + (var_xmp * var_xmax2_dn8)), ((var_xmp_dn9 * var_xmax2) + (var_xmp * var_xmax2_dn9)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn13 * var_xmax2) + (var_xmp * var_xmax2_dn13)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn7, var_xmp_dn8, var_xmp_dn9, var_xmp_dn10, var_xmp_dn13,)
    }
};
        var_xmp = assign61380_e95622;
        var_xmp_dn0 = assign61380_e95622_d_n0;
        var_xmp_dn2 = assign61380_e95622_d_n2;
        var_xmp_dn4 = assign61380_e95622_d_n4;
        var_xmp_dn5 = assign61380_e95622_d_n5;
        var_xmp_dn6 = assign61380_e95622_d_n6;
        var_xmp_dn7 = assign61380_e95622_d_n7;
        var_xmp_dn8 = assign61380_e95622_d_n8;
        var_xmp_dn9 = assign61380_e95622_d_n9;
        var_xmp_dn10 = assign61380_e95622_d_n10;
        var_xmp_dn13 = assign61380_e95622_d_n13;
        var_xmp_rv = 0.0;

        let (assign61390_e95635, assign61390_e95635_d_n0, assign61390_e95635_d_n2, assign61390_e95635_d_n4, assign61390_e95635_d_n5, assign61390_e95635_d_n6, assign61390_e95635_d_n7, assign61390_e95635_d_n8, assign61390_e95635_d_n9, assign61390_e95635_d_n10, assign61390_e95635_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 != 0.0)) {
        let assign61390_e95633: f64 = (var_xp + var_xmp);
        (assign61390_e95633, (var_xp_dn0 + var_xmp_dn0), (var_xp_dn2 + var_xmp_dn2), (var_xp_dn4 + var_xmp_dn4), (var_xp_dn5 + var_xmp_dn5), (var_xp_dn6 + var_xmp_dn6), (var_xp_dn7 + var_xmp_dn7), (var_xp_dn8 + var_xmp_dn8), (var_xp_dn9 + var_xmp_dn9), (var_xp_dn10 + var_xmp_dn10), (var_xp_dn13 + var_xmp_dn13),)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_dn10, var_arg_dn13,)
    }
};
        var_arg = assign61390_e95635;
        var_arg_dn0 = assign61390_e95635_d_n0;
        var_arg_dn2 = assign61390_e95635_d_n2;
        var_arg_dn4 = assign61390_e95635_d_n4;
        var_arg_dn5 = assign61390_e95635_d_n5;
        var_arg_dn6 = assign61390_e95635_d_n6;
        var_arg_dn7 = assign61390_e95635_d_n7;
        var_arg_dn8 = assign61390_e95635_d_n8;
        var_arg_dn9 = assign61390_e95635_d_n9;
        var_arg_dn10 = assign61390_e95635_d_n10;
        var_arg_dn13 = assign61390_e95635_d_n13;
        var_arg_rv = 0.0;

        let (assign61400_e95646, assign61400_e95646_d_n0, assign61400_e95646_d_n2, assign61400_e95646_d_n4, assign61400_e95646_d_n5, assign61400_e95646_d_n6, assign61400_e95646_d_n7, assign61400_e95646_d_n8, assign61400_e95646_d_n9, assign61400_e95646_d_n10, assign61400_e95646_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 != 0.0)) {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_dn10, var_arg_dn13,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
        var_dnm = assign61400_e95646;
        var_dnm_dn0 = assign61400_e95646_d_n0;
        var_dnm_dn2 = assign61400_e95646_d_n2;
        var_dnm_dn4 = assign61400_e95646_d_n4;
        var_dnm_dn5 = assign61400_e95646_d_n5;
        var_dnm_dn6 = assign61400_e95646_d_n6;
        var_dnm_dn7 = assign61400_e95646_d_n7;
        var_dnm_dn8 = assign61400_e95646_d_n8;
        var_dnm_dn9 = assign61400_e95646_d_n9;
        var_dnm_dn10 = assign61400_e95646_d_n10;
        var_dnm_dn13 = assign61400_e95646_d_n13;
        var_dnm_rv = 0.0;

        let assign61410_e95661: f64 = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };
        var_guard1481 = assign61410_e95661;
        var_guard1481_rv = 0.0;

        let assign61420_e95664: f64 = if 2.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard1482 = assign61420_e95664;
        var_guard1482_rv = 0.0;

        let (assign61430_e95679,) = {
    if ((((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 != 0.0)) && (var_guard1481 != 0.0)) && (var_guard1482 != 0.0)) {
        (1.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign61430_e95679;
        var_mm_rv = 0.0;

        let assign61440_e95682: f64 = if 2.0 == 2.0 { 1.0 } else { 0.0 };
        var_guard1483 = assign61440_e95682;
        var_guard1483_rv = 0.0;

        let (assign61450_e95700,) = {
    if (((((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 != 0.0)) && (var_guard1481 != 0.0)) && (var_guard1482 == 0.0)) && (var_guard1483 != 0.0)) {
        (2.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign61450_e95700;
        var_mm_rv = 0.0;

        let assign61460_e95703: f64 = if 2.0 == 4.0 { 1.0 } else { 0.0 };
        var_guard1484 = assign61460_e95703;
        var_guard1484_rv = 0.0;

        let (assign61470_e95724,) = {
    if ((((((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 != 0.0)) && (var_guard1481 != 0.0)) && (var_guard1482 == 0.0)) && (var_guard1483 == 0.0)) && (var_guard1484 != 0.0)) {
        (3.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign61470_e95724;
        var_mm_rv = 0.0;

        let assign61480_e95727: f64 = if 2.0 == 8.0 { 1.0 } else { 0.0 };
        var_guard1485 = assign61480_e95727;
        var_guard1485_rv = 0.0;

        let (assign61490_e95751,) = {
    if (((((((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 != 0.0)) && (var_guard1481 != 0.0)) && (var_guard1482 == 0.0)) && (var_guard1483 == 0.0)) && (var_guard1484 == 0.0)) && (var_guard1485 != 0.0)) {
        (4.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign61490_e95751;
        var_mm_rv = 0.0;

        let (assign61500_e95764,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 != 0.0)) && (var_guard1481 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign61500_e95764;
        var_m0_rv = 0.0;

        let mut assign61510_loop_guard: usize = 0;
        while {
            let assign61510_cond_e95778: f64 = if ((((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 != 0.0)) && (var_guard1481 != 0.0)) && (var_m0 < var_mm)) { 1.0 } else { 0.0 };
            assign61510_cond_e95778 != 0.0
        } {
            assign61510_loop_guard += 1;
            assert!(assign61510_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign61510_body0_e95792, assign61510_body0_e95792_d_n0, assign61510_body0_e95792_d_n2, assign61510_body0_e95792_d_n4, assign61510_body0_e95792_d_n5, assign61510_body0_e95792_d_n6, assign61510_body0_e95792_d_n7, assign61510_body0_e95792_d_n8, assign61510_body0_e95792_d_n9, assign61510_body0_e95792_d_n10, assign61510_body0_e95792_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 != 0.0)) && (var_guard1481 != 0.0)) {
        let assign61510_body0_e95790: f64 = (var_dnm).sqrt();
        (assign61510_body0_e95790, (var_dnm_dn0 / (2.0 * assign61510_body0_e95790)), (var_dnm_dn2 / (2.0 * assign61510_body0_e95790)), (var_dnm_dn4 / (2.0 * assign61510_body0_e95790)), (var_dnm_dn5 / (2.0 * assign61510_body0_e95790)), (var_dnm_dn6 / (2.0 * assign61510_body0_e95790)), (var_dnm_dn7 / (2.0 * assign61510_body0_e95790)), (var_dnm_dn8 / (2.0 * assign61510_body0_e95790)), (var_dnm_dn9 / (2.0 * assign61510_body0_e95790)), (var_dnm_dn10 / (2.0 * assign61510_body0_e95790)), (var_dnm_dn13 / (2.0 * assign61510_body0_e95790)),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
            var_dnm = assign61510_body0_e95792;
            var_dnm_dn0 = assign61510_body0_e95792_d_n0;
            var_dnm_dn2 = assign61510_body0_e95792_d_n2;
            var_dnm_dn4 = assign61510_body0_e95792_d_n4;
            var_dnm_dn5 = assign61510_body0_e95792_d_n5;
            var_dnm_dn6 = assign61510_body0_e95792_d_n6;
            var_dnm_dn7 = assign61510_body0_e95792_d_n7;
            var_dnm_dn8 = assign61510_body0_e95792_d_n8;
            var_dnm_dn9 = assign61510_body0_e95792_d_n9;
            var_dnm_dn10 = assign61510_body0_e95792_d_n10;
            var_dnm_dn13 = assign61510_body0_e95792_d_n13;
            var_dnm_rv = 0.0;
            let (assign61510_body1_e95807,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 != 0.0)) && (var_guard1481 != 0.0)) {
        let assign61510_body1_e95805: f64 = (var_m0 + 1.0);
        (assign61510_body1_e95805,)
    } else {
        (var_m0,)
    }
};
            var_m0 = assign61510_body1_e95807;
            var_m0_rv = 0.0;
        }

        let (assign61520_e95832, assign61520_e95832_d_n0, assign61520_e95832_d_n2, assign61520_e95832_d_n4, assign61520_e95832_d_n5, assign61520_e95832_d_n6, assign61520_e95832_d_n7, assign61520_e95832_d_n8, assign61520_e95832_d_n9, assign61520_e95832_d_n10, assign61520_e95832_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 != 0.0)) && (var_guard1481 == 0.0)) {
        let (assign61520_e95830, assign61520_e95830_d_n0, assign61520_e95830_d_n2, assign61520_e95830_d_n4, assign61520_e95830_d_n5, assign61520_e95830_d_n6, assign61520_e95830_d_n7, assign61520_e95830_d_n8, assign61520_e95830_d_n9, assign61520_e95830_d_n10, assign61520_e95830_d_n13,) = {
            if (var_dnm == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign61520_e95827: f64 = (2.0 * 2.0);
                let assign61520_e95828: f64 = (1.0 / assign61520_e95827);
                let assign61520_e95829: f64 = (var_dnm).powf(assign61520_e95828);
                (assign61520_e95829, if 0.0 == 0.0 && ((assign61520_e95828) as f64).is_finite() && ((assign61520_e95828) as f64).fract() == 0.0 { if assign61520_e95828 == 0.0 { 0.0 } else { (assign61520_e95828 * ((var_dnm).powf(assign61520_e95828 - 1.0) * var_dnm_dn0)) } } else { (assign61520_e95829 * (assign61520_e95828 * (var_dnm_dn0 / var_dnm))) }, if 0.0 == 0.0 && ((assign61520_e95828) as f64).is_finite() && ((assign61520_e95828) as f64).fract() == 0.0 { if assign61520_e95828 == 0.0 { 0.0 } else { (assign61520_e95828 * ((var_dnm).powf(assign61520_e95828 - 1.0) * var_dnm_dn2)) } } else { (assign61520_e95829 * (assign61520_e95828 * (var_dnm_dn2 / var_dnm))) }, if 0.0 == 0.0 && ((assign61520_e95828) as f64).is_finite() && ((assign61520_e95828) as f64).fract() == 0.0 { if assign61520_e95828 == 0.0 { 0.0 } else { (assign61520_e95828 * ((var_dnm).powf(assign61520_e95828 - 1.0) * var_dnm_dn4)) } } else { (assign61520_e95829 * (assign61520_e95828 * (var_dnm_dn4 / var_dnm))) }, if 0.0 == 0.0 && ((assign61520_e95828) as f64).is_finite() && ((assign61520_e95828) as f64).fract() == 0.0 { if assign61520_e95828 == 0.0 { 0.0 } else { (assign61520_e95828 * ((var_dnm).powf(assign61520_e95828 - 1.0) * var_dnm_dn5)) } } else { (assign61520_e95829 * (assign61520_e95828 * (var_dnm_dn5 / var_dnm))) }, if 0.0 == 0.0 && ((assign61520_e95828) as f64).is_finite() && ((assign61520_e95828) as f64).fract() == 0.0 { if assign61520_e95828 == 0.0 { 0.0 } else { (assign61520_e95828 * ((var_dnm).powf(assign61520_e95828 - 1.0) * var_dnm_dn6)) } } else { (assign61520_e95829 * (assign61520_e95828 * (var_dnm_dn6 / var_dnm))) }, if 0.0 == 0.0 && ((assign61520_e95828) as f64).is_finite() && ((assign61520_e95828) as f64).fract() == 0.0 { if assign61520_e95828 == 0.0 { 0.0 } else { (assign61520_e95828 * ((var_dnm).powf(assign61520_e95828 - 1.0) * var_dnm_dn7)) } } else { (assign61520_e95829 * (assign61520_e95828 * (var_dnm_dn7 / var_dnm))) }, if 0.0 == 0.0 && ((assign61520_e95828) as f64).is_finite() && ((assign61520_e95828) as f64).fract() == 0.0 { if assign61520_e95828 == 0.0 { 0.0 } else { (assign61520_e95828 * ((var_dnm).powf(assign61520_e95828 - 1.0) * var_dnm_dn8)) } } else { (assign61520_e95829 * (assign61520_e95828 * (var_dnm_dn8 / var_dnm))) }, if 0.0 == 0.0 && ((assign61520_e95828) as f64).is_finite() && ((assign61520_e95828) as f64).fract() == 0.0 { if assign61520_e95828 == 0.0 { 0.0 } else { (assign61520_e95828 * ((var_dnm).powf(assign61520_e95828 - 1.0) * var_dnm_dn9)) } } else { (assign61520_e95829 * (assign61520_e95828 * (var_dnm_dn9 / var_dnm))) }, if 0.0 == 0.0 && ((assign61520_e95828) as f64).is_finite() && ((assign61520_e95828) as f64).fract() == 0.0 { if assign61520_e95828 == 0.0 { 0.0 } else { (assign61520_e95828 * ((var_dnm).powf(assign61520_e95828 - 1.0) * var_dnm_dn10)) } } else { (assign61520_e95829 * (assign61520_e95828 * (var_dnm_dn10 / var_dnm))) }, if 0.0 == 0.0 && ((assign61520_e95828) as f64).is_finite() && ((assign61520_e95828) as f64).fract() == 0.0 { if assign61520_e95828 == 0.0 { 0.0 } else { (assign61520_e95828 * ((var_dnm).powf(assign61520_e95828 - 1.0) * var_dnm_dn13)) } } else { (assign61520_e95829 * (assign61520_e95828 * (var_dnm_dn13 / var_dnm))) },)
            }
        };
        (assign61520_e95830, assign61520_e95830_d_n0, assign61520_e95830_d_n2, assign61520_e95830_d_n4, assign61520_e95830_d_n5, assign61520_e95830_d_n6, assign61520_e95830_d_n7, assign61520_e95830_d_n8, assign61520_e95830_d_n9, assign61520_e95830_d_n10, assign61520_e95830_d_n13,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
        var_dnm = assign61520_e95832;
        var_dnm_dn0 = assign61520_e95832_d_n0;
        var_dnm_dn2 = assign61520_e95832_d_n2;
        var_dnm_dn4 = assign61520_e95832_d_n4;
        var_dnm_dn5 = assign61520_e95832_d_n5;
        var_dnm_dn6 = assign61520_e95832_d_n6;
        var_dnm_dn7 = assign61520_e95832_d_n7;
        var_dnm_dn8 = assign61520_e95832_d_n8;
        var_dnm_dn9 = assign61520_e95832_d_n9;
        var_dnm_dn10 = assign61520_e95832_d_n10;
        var_dnm_dn13 = assign61520_e95832_d_n13;
        var_dnm_rv = 0.0;

        let (assign61530_e95845, assign61530_e95845_d_n0, assign61530_e95845_d_n2, assign61530_e95845_d_n4, assign61530_e95845_d_n5, assign61530_e95845_d_n6, assign61530_e95845_d_n7, assign61530_e95845_d_n8, assign61530_e95845_d_n9, assign61530_e95845_d_n10, assign61530_e95845_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 != 0.0)) {
        let assign61530_e95843: f64 = (1.0 / var_dnm);
        (assign61530_e95843, (-(var_dnm_dn0 / (var_dnm * var_dnm))), (-(var_dnm_dn2 / (var_dnm * var_dnm))), (-(var_dnm_dn4 / (var_dnm * var_dnm))), (-(var_dnm_dn5 / (var_dnm * var_dnm))), (-(var_dnm_dn6 / (var_dnm * var_dnm))), (-(var_dnm_dn7 / (var_dnm * var_dnm))), (-(var_dnm_dn8 / (var_dnm * var_dnm))), (-(var_dnm_dn9 / (var_dnm * var_dnm))), (-(var_dnm_dn10 / (var_dnm * var_dnm))), (-(var_dnm_dn13 / (var_dnm * var_dnm))),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
        var_dnm = assign61530_e95845;
        var_dnm_dn0 = assign61530_e95845_d_n0;
        var_dnm_dn2 = assign61530_e95845_d_n2;
        var_dnm_dn4 = assign61530_e95845_d_n4;
        var_dnm_dn5 = assign61530_e95845_d_n5;
        var_dnm_dn6 = assign61530_e95845_d_n6;
        var_dnm_dn7 = assign61530_e95845_d_n7;
        var_dnm_dn8 = assign61530_e95845_d_n8;
        var_dnm_dn9 = assign61530_e95845_d_n9;
        var_dnm_dn10 = assign61530_e95845_d_n10;
        var_dnm_dn13 = assign61530_e95845_d_n13;
        var_dnm_rv = 0.0;

        let (assign61540_e95862, assign61540_e95862_d_n0, assign61540_e95862_d_n2, assign61540_e95862_d_n4, assign61540_e95862_d_n5, assign61540_e95862_d_n6, assign61540_e95862_d_n7, assign61540_e95862_d_n8, assign61540_e95862_d_n9, assign61540_e95862_d_n10, assign61540_e95862_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 != 0.0)) {
        let assign61540_e95857: f64 = (10.0 * 2.220446049250313e-16);
        let assign61540_e95858: f64 = (var_tmf1 * assign61540_e95857);
        let assign61540_e95860: f64 = (assign61540_e95858 * var_dnm);
        (assign61540_e95860, (((var_tmf1_dn0 * assign61540_e95857) * var_dnm) + (assign61540_e95858 * var_dnm_dn0)), (((var_tmf1_dn2 * assign61540_e95857) * var_dnm) + (assign61540_e95858 * var_dnm_dn2)), (((var_tmf1_dn4 * assign61540_e95857) * var_dnm) + (assign61540_e95858 * var_dnm_dn4)), (((var_tmf1_dn5 * assign61540_e95857) * var_dnm) + (assign61540_e95858 * var_dnm_dn5)), (((var_tmf1_dn6 * assign61540_e95857) * var_dnm) + (assign61540_e95858 * var_dnm_dn6)), (((var_tmf1_dn7 * assign61540_e95857) * var_dnm) + (assign61540_e95858 * var_dnm_dn7)), (((var_tmf1_dn8 * assign61540_e95857) * var_dnm) + (assign61540_e95858 * var_dnm_dn8)), (((var_tmf1_dn9 * assign61540_e95857) * var_dnm) + (assign61540_e95858 * var_dnm_dn9)), (((var_tmf1_dn10 * assign61540_e95857) * var_dnm) + (assign61540_e95858 * var_dnm_dn10)), (((var_tmf1_dn13 * assign61540_e95857) * var_dnm) + (assign61540_e95858 * var_dnm_dn13)),)
    } else {
        (var_tmf0, var_tmf0_dn0, var_tmf0_dn2, var_tmf0_dn4, var_tmf0_dn5, var_tmf0_dn6, var_tmf0_dn7, var_tmf0_dn8, var_tmf0_dn9, var_tmf0_dn10, var_tmf0_dn13,)
    }
};
        var_tmf0 = assign61540_e95862;
        var_tmf0_dn0 = assign61540_e95862_d_n0;
        var_tmf0_dn2 = assign61540_e95862_d_n2;
        var_tmf0_dn4 = assign61540_e95862_d_n4;
        var_tmf0_dn5 = assign61540_e95862_d_n5;
        var_tmf0_dn6 = assign61540_e95862_d_n6;
        var_tmf0_dn7 = assign61540_e95862_d_n7;
        var_tmf0_dn8 = assign61540_e95862_d_n8;
        var_tmf0_dn9 = assign61540_e95862_d_n9;
        var_tmf0_dn10 = assign61540_e95862_d_n10;
        var_tmf0_dn13 = assign61540_e95862_d_n13;
        var_tmf0_rv = 0.0;

        let (assign61550_e95881, assign61550_e95881_d_n0, assign61550_e95881_d_n2, assign61550_e95881_d_n4, assign61550_e95881_d_n5, assign61550_e95881_d_n6, assign61550_e95881_d_n7, assign61550_e95881_d_n8, assign61550_e95881_d_n9, assign61550_e95881_d_n10, assign61550_e95881_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 != 0.0)) {
        let assign61550_e95873: f64 = (10.0 * 2.220446049250313e-16);
        let assign61550_e95875: f64 = (assign61550_e95873 * var_xmp);
        let assign61550_e95877: f64 = (assign61550_e95875 * var_dnm);
        let assign61550_e95879: f64 = (assign61550_e95877 / var_arg);
        (assign61550_e95879, ((((((assign61550_e95873 * var_xmp_dn0) * var_dnm) + (assign61550_e95875 * var_dnm_dn0)) * var_arg) - (assign61550_e95877 * var_arg_dn0)) / (var_arg * var_arg)), ((((((assign61550_e95873 * var_xmp_dn2) * var_dnm) + (assign61550_e95875 * var_dnm_dn2)) * var_arg) - (assign61550_e95877 * var_arg_dn2)) / (var_arg * var_arg)), ((((((assign61550_e95873 * var_xmp_dn4) * var_dnm) + (assign61550_e95875 * var_dnm_dn4)) * var_arg) - (assign61550_e95877 * var_arg_dn4)) / (var_arg * var_arg)), ((((((assign61550_e95873 * var_xmp_dn5) * var_dnm) + (assign61550_e95875 * var_dnm_dn5)) * var_arg) - (assign61550_e95877 * var_arg_dn5)) / (var_arg * var_arg)), ((((((assign61550_e95873 * var_xmp_dn6) * var_dnm) + (assign61550_e95875 * var_dnm_dn6)) * var_arg) - (assign61550_e95877 * var_arg_dn6)) / (var_arg * var_arg)), ((((((assign61550_e95873 * var_xmp_dn7) * var_dnm) + (assign61550_e95875 * var_dnm_dn7)) * var_arg) - (assign61550_e95877 * var_arg_dn7)) / (var_arg * var_arg)), ((((((assign61550_e95873 * var_xmp_dn8) * var_dnm) + (assign61550_e95875 * var_dnm_dn8)) * var_arg) - (assign61550_e95877 * var_arg_dn8)) / (var_arg * var_arg)), ((((((assign61550_e95873 * var_xmp_dn9) * var_dnm) + (assign61550_e95875 * var_dnm_dn9)) * var_arg) - (assign61550_e95877 * var_arg_dn9)) / (var_arg * var_arg)), ((((((assign61550_e95873 * var_xmp_dn10) * var_dnm) + (assign61550_e95875 * var_dnm_dn10)) * var_arg) - (assign61550_e95877 * var_arg_dn10)) / (var_arg * var_arg)), ((((((assign61550_e95873 * var_xmp_dn13) * var_dnm) + (assign61550_e95875 * var_dnm_dn13)) * var_arg) - (assign61550_e95877 * var_arg_dn13)) / (var_arg * var_arg)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign61550_e95881;
        var_t0_dn0 = assign61550_e95881_d_n0;
        var_t0_dn2 = assign61550_e95881_d_n2;
        var_t0_dn4 = assign61550_e95881_d_n4;
        var_t0_dn5 = assign61550_e95881_d_n5;
        var_t0_dn6 = assign61550_e95881_d_n6;
        var_t0_dn7 = assign61550_e95881_d_n7;
        var_t0_dn8 = assign61550_e95881_d_n8;
        var_t0_dn9 = assign61550_e95881_d_n9;
        var_t0_dn10 = assign61550_e95881_d_n10;
        var_t0_dn13 = assign61550_e95881_d_n13;
        var_t0_rv = 0.0;

        *var_arg_slot = var_arg;
        *var_arg_dn0_slot = var_arg_dn0;
        *var_arg_dn10_slot = var_arg_dn10;
        *var_arg_dn13_slot = var_arg_dn13;
        *var_arg_dn2_slot = var_arg_dn2;
        *var_arg_dn4_slot = var_arg_dn4;
        *var_arg_dn5_slot = var_arg_dn5;
        *var_arg_dn6_slot = var_arg_dn6;
        *var_arg_dn7_slot = var_arg_dn7;
        *var_arg_dn8_slot = var_arg_dn8;
        *var_arg_dn9_slot = var_arg_dn9;
        *var_arg_rv_slot = var_arg_rv;
        *var_dnm_slot = var_dnm;
        *var_dnm_dn0_slot = var_dnm_dn0;
        *var_dnm_dn10_slot = var_dnm_dn10;
        *var_dnm_dn13_slot = var_dnm_dn13;
        *var_dnm_dn2_slot = var_dnm_dn2;
        *var_dnm_dn4_slot = var_dnm_dn4;
        *var_dnm_dn5_slot = var_dnm_dn5;
        *var_dnm_dn6_slot = var_dnm_dn6;
        *var_dnm_dn7_slot = var_dnm_dn7;
        *var_dnm_dn8_slot = var_dnm_dn8;
        *var_dnm_dn9_slot = var_dnm_dn9;
        *var_dnm_rv_slot = var_dnm_rv;
        *var_guard1480_slot = var_guard1480;
        *var_guard1480_rv_slot = var_guard1480_rv;
        *var_guard1481_slot = var_guard1481;
        *var_guard1481_rv_slot = var_guard1481_rv;
        *var_guard1482_slot = var_guard1482;
        *var_guard1482_rv_slot = var_guard1482_rv;
        *var_guard1483_slot = var_guard1483;
        *var_guard1483_rv_slot = var_guard1483_rv;
        *var_guard1484_slot = var_guard1484;
        *var_guard1484_rv_slot = var_guard1484_rv;
        *var_guard1485_slot = var_guard1485;
        *var_guard1485_rv_slot = var_guard1485_rv;
        *var_m0_slot = var_m0;
        *var_m0_rv_slot = var_m0_rv;
        *var_mm_slot = var_mm;
        *var_mm_rv_slot = var_mm_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn13_slot = var_t0_dn13;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t0_rv_slot = var_t0_rv;
        *var_tmf0_slot = var_tmf0;
        *var_tmf0_dn0_slot = var_tmf0_dn0;
        *var_tmf0_dn10_slot = var_tmf0_dn10;
        *var_tmf0_dn13_slot = var_tmf0_dn13;
        *var_tmf0_dn2_slot = var_tmf0_dn2;
        *var_tmf0_dn4_slot = var_tmf0_dn4;
        *var_tmf0_dn5_slot = var_tmf0_dn5;
        *var_tmf0_dn6_slot = var_tmf0_dn6;
        *var_tmf0_dn7_slot = var_tmf0_dn7;
        *var_tmf0_dn8_slot = var_tmf0_dn8;
        *var_tmf0_dn9_slot = var_tmf0_dn9;
        *var_tmf0_rv_slot = var_tmf0_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn13_slot = var_tmf1_dn13;
        *var_tmf1_dn2_slot = var_tmf1_dn2;
        *var_tmf1_dn4_slot = var_tmf1_dn4;
        *var_tmf1_dn5_slot = var_tmf1_dn5;
        *var_tmf1_dn6_slot = var_tmf1_dn6;
        *var_tmf1_dn7_slot = var_tmf1_dn7;
        *var_tmf1_dn8_slot = var_tmf1_dn8;
        *var_tmf1_dn9_slot = var_tmf1_dn9;
        *var_tmf1_rv_slot = var_tmf1_rv;
        *var_x2_slot = var_x2;
        *var_x2_dn0_slot = var_x2_dn0;
        *var_x2_dn10_slot = var_x2_dn10;
        *var_x2_dn13_slot = var_x2_dn13;
        *var_x2_dn2_slot = var_x2_dn2;
        *var_x2_dn4_slot = var_x2_dn4;
        *var_x2_dn5_slot = var_x2_dn5;
        *var_x2_dn6_slot = var_x2_dn6;
        *var_x2_dn7_slot = var_x2_dn7;
        *var_x2_dn8_slot = var_x2_dn8;
        *var_x2_dn9_slot = var_x2_dn9;
        *var_x2_rv_slot = var_x2_rv;
        *var_xmax2_slot = var_xmax2;
        *var_xmax2_dn0_slot = var_xmax2_dn0;
        *var_xmax2_dn10_slot = var_xmax2_dn10;
        *var_xmax2_dn13_slot = var_xmax2_dn13;
        *var_xmax2_dn2_slot = var_xmax2_dn2;
        *var_xmax2_dn4_slot = var_xmax2_dn4;
        *var_xmax2_dn5_slot = var_xmax2_dn5;
        *var_xmax2_dn6_slot = var_xmax2_dn6;
        *var_xmax2_dn7_slot = var_xmax2_dn7;
        *var_xmax2_dn8_slot = var_xmax2_dn8;
        *var_xmax2_dn9_slot = var_xmax2_dn9;
        *var_xmax2_rv_slot = var_xmax2_rv;
        *var_xmp_slot = var_xmp;
        *var_xmp_dn0_slot = var_xmp_dn0;
        *var_xmp_dn10_slot = var_xmp_dn10;
        *var_xmp_dn13_slot = var_xmp_dn13;
        *var_xmp_dn2_slot = var_xmp_dn2;
        *var_xmp_dn4_slot = var_xmp_dn4;
        *var_xmp_dn5_slot = var_xmp_dn5;
        *var_xmp_dn6_slot = var_xmp_dn6;
        *var_xmp_dn7_slot = var_xmp_dn7;
        *var_xmp_dn8_slot = var_xmp_dn8;
        *var_xmp_dn9_slot = var_xmp_dn9;
        *var_xmp_rv_slot = var_xmp_rv;
        *var_xp_slot = var_xp;
        *var_xp_dn0_slot = var_xp_dn0;
        *var_xp_dn10_slot = var_xp_dn10;
        *var_xp_dn13_slot = var_xp_dn13;
        *var_xp_dn2_slot = var_xp_dn2;
        *var_xp_dn4_slot = var_xp_dn4;
        *var_xp_dn5_slot = var_xp_dn5;
        *var_xp_dn6_slot = var_xp_dn6;
        *var_xp_dn7_slot = var_xp_dn7;
        *var_xp_dn8_slot = var_xp_dn8;
        *var_xp_dn9_slot = var_xp_dn9;
        *var_xp_rv_slot = var_xp_rv;
    }

    pub(super) fn stamp_reactive_block_220(
        var_alpha: f64,
        var_alpha_dn0: f64,
        var_alpha_dn10: f64,
        var_alpha_dn13: f64,
        var_alpha_dn2: f64,
        var_alpha_dn4: f64,
        var_alpha_dn5: f64,
        var_alpha_dn6: f64,
        var_alpha_dn7: f64,
        var_alpha_dn8: f64,
        var_alpha_dn9: f64,
        var_cox: f64,
        var_cox_dn0: f64,
        var_cox_dn10: f64,
        var_cox_dn13: f64,
        var_cox_dn2: f64,
        var_cox_dn4: f64,
        var_cox_dn5: f64,
        var_cox_dn6: f64,
        var_cox_dn7: f64,
        var_cox_dn8: f64,
        var_cox_dn9: f64,
        var_fd2: f64,
        var_fd2_dn0: f64,
        var_fd2_dn10: f64,
        var_fd2_dn13: f64,
        var_fd2_dn2: f64,
        var_fd2_dn4: f64,
        var_fd2_dn5: f64,
        var_fd2_dn6: f64,
        var_fd2_dn7: f64,
        var_fd2_dn8: f64,
        var_fd2_dn9: f64,
        var_flg_zone: f64,
        var_guard1430: f64,
        var_guard1461: f64,
        var_guard1480: f64,
        var_guard443: f64,
        var_leff: f64,
        var_qb0: f64,
        var_qb0_dn0: f64,
        var_qb0_dn10: f64,
        var_qb0_dn13: f64,
        var_qb0_dn2: f64,
        var_qb0_dn4: f64,
        var_qb0_dn5: f64,
        var_qb0_dn6: f64,
        var_qb0_dn7: f64,
        var_qb0_dn8: f64,
        var_qb0_dn9: f64,
        var_qinm: f64,
        var_qinm_dn0: f64,
        var_qinm_dn10: f64,
        var_qinm_dn13: f64,
        var_qinm_dn2: f64,
        var_qinm_dn4: f64,
        var_qinm_dn5: f64,
        var_qinm_dn6: f64,
        var_qinm_dn7: f64,
        var_qinm_dn8: f64,
        var_qinm_dn9: f64,
        var_qn0: f64,
        var_qn0_dn0: f64,
        var_qn0_dn10: f64,
        var_qn0_dn13: f64,
        var_qn0_dn2: f64,
        var_qn0_dn4: f64,
        var_qn0_dn5: f64,
        var_qn0_dn6: f64,
        var_qn0_dn7: f64,
        var_qn0_dn8: f64,
        var_qn0_dn9: f64,
        var_tmf0: f64,
        var_tmf0_dn0: f64,
        var_tmf0_dn10: f64,
        var_tmf0_dn13: f64,
        var_tmf0_dn2: f64,
        var_tmf0_dn4: f64,
        var_tmf0_dn5: f64,
        var_tmf0_dn6: f64,
        var_tmf0_dn7: f64,
        var_tmf0_dn8: f64,
        var_tmf0_dn9: f64,
        var_vgvt: f64,
        var_vgvt_dn0: f64,
        var_vgvt_dn10: f64,
        var_vgvt_dn13: f64,
        var_vgvt_dn2: f64,
        var_vgvt_dn4: f64,
        var_vgvt_dn5: f64,
        var_vgvt_dn6: f64,
        var_vgvt_dn7: f64,
        var_vgvt_dn8: f64,
        var_vgvt_dn9: f64,
        var_guard1486_slot: &mut f64,
        var_guard1486_rv_slot: &mut f64,
        var_guard1487_slot: &mut f64,
        var_guard1487_rv_slot: &mut f64,
        var_guard1488_slot: &mut f64,
        var_guard1488_rv_slot: &mut f64,
        var_guard1489_slot: &mut f64,
        var_guard1489_rv_slot: &mut f64,
        var_guard1490_slot: &mut f64,
        var_guard1490_rv_slot: &mut f64,
        var_lch_slot: &mut f64,
        var_lch_dn0_slot: &mut f64,
        var_lch_dn10_slot: &mut f64,
        var_lch_dn13_slot: &mut f64,
        var_lch_dn2_slot: &mut f64,
        var_lch_dn4_slot: &mut f64,
        var_lch_dn5_slot: &mut f64,
        var_lch_dn6_slot: &mut f64,
        var_lch_dn7_slot: &mut f64,
        var_lch_dn8_slot: &mut f64,
        var_lch_dn9_slot: &mut f64,
        var_lch_rv_slot: &mut f64,
        var_lred_slot: &mut f64,
        var_lred_dn0_slot: &mut f64,
        var_lred_dn10_slot: &mut f64,
        var_lred_dn13_slot: &mut f64,
        var_lred_dn2_slot: &mut f64,
        var_lred_dn4_slot: &mut f64,
        var_lred_dn5_slot: &mut f64,
        var_lred_dn6_slot: &mut f64,
        var_lred_dn7_slot: &mut f64,
        var_lred_dn8_slot: &mut f64,
        var_lred_dn9_slot: &mut f64,
        var_lred_rv_slot: &mut f64,
        var_qbu_slot: &mut f64,
        var_qbu_dn0_slot: &mut f64,
        var_qbu_dn10_slot: &mut f64,
        var_qbu_dn13_slot: &mut f64,
        var_qbu_dn2_slot: &mut f64,
        var_qbu_dn4_slot: &mut f64,
        var_qbu_dn5_slot: &mut f64,
        var_qbu_dn6_slot: &mut f64,
        var_qbu_dn7_slot: &mut f64,
        var_qbu_dn8_slot: &mut f64,
        var_qbu_dn9_slot: &mut f64,
        var_qbu_rv_slot: &mut f64,
        var_qddn_slot: &mut f64,
        var_qddn_dn0_slot: &mut f64,
        var_qddn_dn10_slot: &mut f64,
        var_qddn_dn13_slot: &mut f64,
        var_qddn_dn2_slot: &mut f64,
        var_qddn_dn4_slot: &mut f64,
        var_qddn_dn5_slot: &mut f64,
        var_qddn_dn6_slot: &mut f64,
        var_qddn_dn7_slot: &mut f64,
        var_qddn_dn8_slot: &mut f64,
        var_qddn_dn9_slot: &mut f64,
        var_qddn_rv_slot: &mut f64,
        var_qdnm_slot: &mut f64,
        var_qdnm_dn0_slot: &mut f64,
        var_qdnm_dn10_slot: &mut f64,
        var_qdnm_dn13_slot: &mut f64,
        var_qdnm_dn2_slot: &mut f64,
        var_qdnm_dn4_slot: &mut f64,
        var_qdnm_dn5_slot: &mut f64,
        var_qdnm_dn6_slot: &mut f64,
        var_qdnm_dn7_slot: &mut f64,
        var_qdnm_dn8_slot: &mut f64,
        var_qdnm_dn9_slot: &mut f64,
        var_qdnm_rv_slot: &mut f64,
        var_qdrat_slot: &mut f64,
        var_qdrat_dn0_slot: &mut f64,
        var_qdrat_dn10_slot: &mut f64,
        var_qdrat_dn13_slot: &mut f64,
        var_qdrat_dn2_slot: &mut f64,
        var_qdrat_dn4_slot: &mut f64,
        var_qdrat_dn5_slot: &mut f64,
        var_qdrat_dn6_slot: &mut f64,
        var_qdrat_dn7_slot: &mut f64,
        var_qdrat_dn8_slot: &mut f64,
        var_qdrat_dn9_slot: &mut f64,
        var_qdrat_rv_slot: &mut f64,
        var_qidn_slot: &mut f64,
        var_qidn_dn0_slot: &mut f64,
        var_qidn_dn10_slot: &mut f64,
        var_qidn_dn13_slot: &mut f64,
        var_qidn_dn2_slot: &mut f64,
        var_qidn_dn4_slot: &mut f64,
        var_qidn_dn5_slot: &mut f64,
        var_qidn_dn6_slot: &mut f64,
        var_qidn_dn7_slot: &mut f64,
        var_qidn_dn8_slot: &mut f64,
        var_qidn_dn9_slot: &mut f64,
        var_qidn_rv_slot: &mut f64,
        var_qiu_slot: &mut f64,
        var_qiu_dn0_slot: &mut f64,
        var_qiu_dn10_slot: &mut f64,
        var_qiu_dn13_slot: &mut f64,
        var_qiu_dn2_slot: &mut f64,
        var_qiu_dn4_slot: &mut f64,
        var_qiu_dn5_slot: &mut f64,
        var_qiu_dn6_slot: &mut f64,
        var_qiu_dn7_slot: &mut f64,
        var_qiu_dn8_slot: &mut f64,
        var_qiu_dn9_slot: &mut f64,
        var_qiu_rv_slot: &mut f64,
        var_quot_slot: &mut f64,
        var_quot_dn0_slot: &mut f64,
        var_quot_dn10_slot: &mut f64,
        var_quot_dn13_slot: &mut f64,
        var_quot_dn2_slot: &mut f64,
        var_quot_dn4_slot: &mut f64,
        var_quot_dn5_slot: &mut f64,
        var_quot_dn6_slot: &mut f64,
        var_quot_dn7_slot: &mut f64,
        var_quot_dn8_slot: &mut f64,
        var_quot_dn9_slot: &mut f64,
        var_quot_rv_slot: &mut f64,
        var_start_of_mobility_slot: &mut f64,
        var_start_of_mobility_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn13_slot: &mut f64,
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
        var_t1_dn13_slot: &mut f64,
        var_t1_dn2_slot: &mut f64,
        var_t1_dn4_slot: &mut f64,
        var_t1_dn5_slot: &mut f64,
        var_t1_dn6_slot: &mut f64,
        var_t1_dn7_slot: &mut f64,
        var_t1_dn8_slot: &mut f64,
        var_t1_dn9_slot: &mut f64,
        var_t1_rv_slot: &mut f64,
    ) {
        let mut var_guard1486: f64 = *var_guard1486_slot;
        let mut var_guard1486_rv: f64 = *var_guard1486_rv_slot;
        let mut var_guard1487: f64 = *var_guard1487_slot;
        let mut var_guard1487_rv: f64 = *var_guard1487_rv_slot;
        let mut var_guard1488: f64 = *var_guard1488_slot;
        let mut var_guard1488_rv: f64 = *var_guard1488_rv_slot;
        let mut var_guard1489: f64 = *var_guard1489_slot;
        let mut var_guard1489_rv: f64 = *var_guard1489_rv_slot;
        let mut var_guard1490: f64 = *var_guard1490_slot;
        let mut var_guard1490_rv: f64 = *var_guard1490_rv_slot;
        let mut var_lch: f64 = *var_lch_slot;
        let mut var_lch_dn0: f64 = *var_lch_dn0_slot;
        let mut var_lch_dn10: f64 = *var_lch_dn10_slot;
        let mut var_lch_dn13: f64 = *var_lch_dn13_slot;
        let mut var_lch_dn2: f64 = *var_lch_dn2_slot;
        let mut var_lch_dn4: f64 = *var_lch_dn4_slot;
        let mut var_lch_dn5: f64 = *var_lch_dn5_slot;
        let mut var_lch_dn6: f64 = *var_lch_dn6_slot;
        let mut var_lch_dn7: f64 = *var_lch_dn7_slot;
        let mut var_lch_dn8: f64 = *var_lch_dn8_slot;
        let mut var_lch_dn9: f64 = *var_lch_dn9_slot;
        let mut var_lch_rv: f64 = *var_lch_rv_slot;
        let mut var_lred: f64 = *var_lred_slot;
        let mut var_lred_dn0: f64 = *var_lred_dn0_slot;
        let mut var_lred_dn10: f64 = *var_lred_dn10_slot;
        let mut var_lred_dn13: f64 = *var_lred_dn13_slot;
        let mut var_lred_dn2: f64 = *var_lred_dn2_slot;
        let mut var_lred_dn4: f64 = *var_lred_dn4_slot;
        let mut var_lred_dn5: f64 = *var_lred_dn5_slot;
        let mut var_lred_dn6: f64 = *var_lred_dn6_slot;
        let mut var_lred_dn7: f64 = *var_lred_dn7_slot;
        let mut var_lred_dn8: f64 = *var_lred_dn8_slot;
        let mut var_lred_dn9: f64 = *var_lred_dn9_slot;
        let mut var_lred_rv: f64 = *var_lred_rv_slot;
        let mut var_qbu: f64 = *var_qbu_slot;
        let mut var_qbu_dn0: f64 = *var_qbu_dn0_slot;
        let mut var_qbu_dn10: f64 = *var_qbu_dn10_slot;
        let mut var_qbu_dn13: f64 = *var_qbu_dn13_slot;
        let mut var_qbu_dn2: f64 = *var_qbu_dn2_slot;
        let mut var_qbu_dn4: f64 = *var_qbu_dn4_slot;
        let mut var_qbu_dn5: f64 = *var_qbu_dn5_slot;
        let mut var_qbu_dn6: f64 = *var_qbu_dn6_slot;
        let mut var_qbu_dn7: f64 = *var_qbu_dn7_slot;
        let mut var_qbu_dn8: f64 = *var_qbu_dn8_slot;
        let mut var_qbu_dn9: f64 = *var_qbu_dn9_slot;
        let mut var_qbu_rv: f64 = *var_qbu_rv_slot;
        let mut var_qddn: f64 = *var_qddn_slot;
        let mut var_qddn_dn0: f64 = *var_qddn_dn0_slot;
        let mut var_qddn_dn10: f64 = *var_qddn_dn10_slot;
        let mut var_qddn_dn13: f64 = *var_qddn_dn13_slot;
        let mut var_qddn_dn2: f64 = *var_qddn_dn2_slot;
        let mut var_qddn_dn4: f64 = *var_qddn_dn4_slot;
        let mut var_qddn_dn5: f64 = *var_qddn_dn5_slot;
        let mut var_qddn_dn6: f64 = *var_qddn_dn6_slot;
        let mut var_qddn_dn7: f64 = *var_qddn_dn7_slot;
        let mut var_qddn_dn8: f64 = *var_qddn_dn8_slot;
        let mut var_qddn_dn9: f64 = *var_qddn_dn9_slot;
        let mut var_qddn_rv: f64 = *var_qddn_rv_slot;
        let mut var_qdnm: f64 = *var_qdnm_slot;
        let mut var_qdnm_dn0: f64 = *var_qdnm_dn0_slot;
        let mut var_qdnm_dn10: f64 = *var_qdnm_dn10_slot;
        let mut var_qdnm_dn13: f64 = *var_qdnm_dn13_slot;
        let mut var_qdnm_dn2: f64 = *var_qdnm_dn2_slot;
        let mut var_qdnm_dn4: f64 = *var_qdnm_dn4_slot;
        let mut var_qdnm_dn5: f64 = *var_qdnm_dn5_slot;
        let mut var_qdnm_dn6: f64 = *var_qdnm_dn6_slot;
        let mut var_qdnm_dn7: f64 = *var_qdnm_dn7_slot;
        let mut var_qdnm_dn8: f64 = *var_qdnm_dn8_slot;
        let mut var_qdnm_dn9: f64 = *var_qdnm_dn9_slot;
        let mut var_qdnm_rv: f64 = *var_qdnm_rv_slot;
        let mut var_qdrat: f64 = *var_qdrat_slot;
        let mut var_qdrat_dn0: f64 = *var_qdrat_dn0_slot;
        let mut var_qdrat_dn10: f64 = *var_qdrat_dn10_slot;
        let mut var_qdrat_dn13: f64 = *var_qdrat_dn13_slot;
        let mut var_qdrat_dn2: f64 = *var_qdrat_dn2_slot;
        let mut var_qdrat_dn4: f64 = *var_qdrat_dn4_slot;
        let mut var_qdrat_dn5: f64 = *var_qdrat_dn5_slot;
        let mut var_qdrat_dn6: f64 = *var_qdrat_dn6_slot;
        let mut var_qdrat_dn7: f64 = *var_qdrat_dn7_slot;
        let mut var_qdrat_dn8: f64 = *var_qdrat_dn8_slot;
        let mut var_qdrat_dn9: f64 = *var_qdrat_dn9_slot;
        let mut var_qdrat_rv: f64 = *var_qdrat_rv_slot;
        let mut var_qidn: f64 = *var_qidn_slot;
        let mut var_qidn_dn0: f64 = *var_qidn_dn0_slot;
        let mut var_qidn_dn10: f64 = *var_qidn_dn10_slot;
        let mut var_qidn_dn13: f64 = *var_qidn_dn13_slot;
        let mut var_qidn_dn2: f64 = *var_qidn_dn2_slot;
        let mut var_qidn_dn4: f64 = *var_qidn_dn4_slot;
        let mut var_qidn_dn5: f64 = *var_qidn_dn5_slot;
        let mut var_qidn_dn6: f64 = *var_qidn_dn6_slot;
        let mut var_qidn_dn7: f64 = *var_qidn_dn7_slot;
        let mut var_qidn_dn8: f64 = *var_qidn_dn8_slot;
        let mut var_qidn_dn9: f64 = *var_qidn_dn9_slot;
        let mut var_qidn_rv: f64 = *var_qidn_rv_slot;
        let mut var_qiu: f64 = *var_qiu_slot;
        let mut var_qiu_dn0: f64 = *var_qiu_dn0_slot;
        let mut var_qiu_dn10: f64 = *var_qiu_dn10_slot;
        let mut var_qiu_dn13: f64 = *var_qiu_dn13_slot;
        let mut var_qiu_dn2: f64 = *var_qiu_dn2_slot;
        let mut var_qiu_dn4: f64 = *var_qiu_dn4_slot;
        let mut var_qiu_dn5: f64 = *var_qiu_dn5_slot;
        let mut var_qiu_dn6: f64 = *var_qiu_dn6_slot;
        let mut var_qiu_dn7: f64 = *var_qiu_dn7_slot;
        let mut var_qiu_dn8: f64 = *var_qiu_dn8_slot;
        let mut var_qiu_dn9: f64 = *var_qiu_dn9_slot;
        let mut var_qiu_rv: f64 = *var_qiu_rv_slot;
        let mut var_quot: f64 = *var_quot_slot;
        let mut var_quot_dn0: f64 = *var_quot_dn0_slot;
        let mut var_quot_dn10: f64 = *var_quot_dn10_slot;
        let mut var_quot_dn13: f64 = *var_quot_dn13_slot;
        let mut var_quot_dn2: f64 = *var_quot_dn2_slot;
        let mut var_quot_dn4: f64 = *var_quot_dn4_slot;
        let mut var_quot_dn5: f64 = *var_quot_dn5_slot;
        let mut var_quot_dn6: f64 = *var_quot_dn6_slot;
        let mut var_quot_dn7: f64 = *var_quot_dn7_slot;
        let mut var_quot_dn8: f64 = *var_quot_dn8_slot;
        let mut var_quot_dn9: f64 = *var_quot_dn9_slot;
        let mut var_quot_rv: f64 = *var_quot_rv_slot;
        let mut var_start_of_mobility: f64 = *var_start_of_mobility_slot;
        let mut var_start_of_mobility_rv: f64 = *var_start_of_mobility_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn13: f64 = *var_t0_dn13_slot;
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
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
        let mut var_t1_dn2: f64 = *var_t1_dn2_slot;
        let mut var_t1_dn4: f64 = *var_t1_dn4_slot;
        let mut var_t1_dn5: f64 = *var_t1_dn5_slot;
        let mut var_t1_dn6: f64 = *var_t1_dn6_slot;
        let mut var_t1_dn7: f64 = *var_t1_dn7_slot;
        let mut var_t1_dn8: f64 = *var_t1_dn8_slot;
        let mut var_t1_dn9: f64 = *var_t1_dn9_slot;
        let mut var_t1_rv: f64 = *var_t1_rv_slot;

        let (assign61560_e95900, assign61560_e95900_d_n0, assign61560_e95900_d_n2, assign61560_e95900_d_n4, assign61560_e95900_d_n5, assign61560_e95900_d_n6, assign61560_e95900_d_n7, assign61560_e95900_d_n8, assign61560_e95900_d_n9, assign61560_e95900_d_n10, assign61560_e95900_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 != 0.0)) {
        let assign61560_e95892: f64 = (10.0 * 2.220446049250313e-16);
        let assign61560_e95895: f64 = (10.0 * 2.220446049250313e-16);
        let assign61560_e95896: f64 = (assign61560_e95892 + assign61560_e95895);
        let assign61560_e95898: f64 = (assign61560_e95896 - var_tmf0);
        (assign61560_e95898, (-var_tmf0_dn0), (-var_tmf0_dn2), (-var_tmf0_dn4), (-var_tmf0_dn5), (-var_tmf0_dn6), (-var_tmf0_dn7), (-var_tmf0_dn8), (-var_tmf0_dn9), (-var_tmf0_dn10), (-var_tmf0_dn13),)
    } else {
        (var_qidn, var_qidn_dn0, var_qidn_dn2, var_qidn_dn4, var_qidn_dn5, var_qidn_dn6, var_qidn_dn7, var_qidn_dn8, var_qidn_dn9, var_qidn_dn10, var_qidn_dn13,)
    }
};
        var_qidn = assign61560_e95900;
        var_qidn_dn0 = assign61560_e95900_d_n0;
        var_qidn_dn2 = assign61560_e95900_d_n2;
        var_qidn_dn4 = assign61560_e95900_d_n4;
        var_qidn_dn5 = assign61560_e95900_d_n5;
        var_qidn_dn6 = assign61560_e95900_d_n6;
        var_qidn_dn7 = assign61560_e95900_d_n7;
        var_qidn_dn8 = assign61560_e95900_d_n8;
        var_qidn_dn9 = assign61560_e95900_d_n9;
        var_qidn_dn10 = assign61560_e95900_d_n10;
        var_qidn_dn13 = assign61560_e95900_d_n13;
        var_qidn_rv = 0.0;

        let (assign61570_e95911, assign61570_e95911_d_n0, assign61570_e95911_d_n2, assign61570_e95911_d_n4, assign61570_e95911_d_n5, assign61570_e95911_d_n6, assign61570_e95911_d_n7, assign61570_e95911_d_n8, assign61570_e95911_d_n9, assign61570_e95911_d_n10, assign61570_e95911_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 != 0.0)) {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign61570_e95911;
        var_t0_dn0 = assign61570_e95911_d_n0;
        var_t0_dn2 = assign61570_e95911_d_n2;
        var_t0_dn4 = assign61570_e95911_d_n4;
        var_t0_dn5 = assign61570_e95911_d_n5;
        var_t0_dn6 = assign61570_e95911_d_n6;
        var_t0_dn7 = assign61570_e95911_d_n7;
        var_t0_dn8 = assign61570_e95911_d_n8;
        var_t0_dn9 = assign61570_e95911_d_n9;
        var_t0_dn10 = assign61570_e95911_d_n10;
        var_t0_dn13 = assign61570_e95911_d_n13;
        var_t0_rv = 0.0;

        let (assign61580_e95925, assign61580_e95925_d_n0, assign61580_e95925_d_n2, assign61580_e95925_d_n4, assign61580_e95925_d_n5, assign61580_e95925_d_n6, assign61580_e95925_d_n7, assign61580_e95925_d_n8, assign61580_e95925_d_n9, assign61580_e95925_d_n10, assign61580_e95925_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 == 0.0)) {
        let assign61580_e95923: f64 = (1.0 + var_alpha);
        (assign61580_e95923, var_alpha_dn0, var_alpha_dn2, var_alpha_dn4, var_alpha_dn5, var_alpha_dn6, var_alpha_dn7, var_alpha_dn8, var_alpha_dn9, var_alpha_dn10, var_alpha_dn13,)
    } else {
        (var_qidn, var_qidn_dn0, var_qidn_dn2, var_qidn_dn4, var_qidn_dn5, var_qidn_dn6, var_qidn_dn7, var_qidn_dn8, var_qidn_dn9, var_qidn_dn10, var_qidn_dn13,)
    }
};
        var_qidn = assign61580_e95925;
        var_qidn_dn0 = assign61580_e95925_d_n0;
        var_qidn_dn2 = assign61580_e95925_d_n2;
        var_qidn_dn4 = assign61580_e95925_d_n4;
        var_qidn_dn5 = assign61580_e95925_d_n5;
        var_qidn_dn6 = assign61580_e95925_d_n6;
        var_qidn_dn7 = assign61580_e95925_d_n7;
        var_qidn_dn8 = assign61580_e95925_d_n8;
        var_qidn_dn9 = assign61580_e95925_d_n9;
        var_qidn_dn10 = assign61580_e95925_d_n10;
        var_qidn_dn13 = assign61580_e95925_d_n13;
        var_qidn_rv = 0.0;

        let (assign61590_e95937, assign61590_e95937_d_n0, assign61590_e95937_d_n2, assign61590_e95937_d_n4, assign61590_e95937_d_n5, assign61590_e95937_d_n6, assign61590_e95937_d_n7, assign61590_e95937_d_n8, assign61590_e95937_d_n9, assign61590_e95937_d_n10, assign61590_e95937_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1480 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign61590_e95937;
        var_t0_dn0 = assign61590_e95937_d_n0;
        var_t0_dn2 = assign61590_e95937_d_n2;
        var_t0_dn4 = assign61590_e95937_d_n4;
        var_t0_dn5 = assign61590_e95937_d_n5;
        var_t0_dn6 = assign61590_e95937_d_n6;
        var_t0_dn7 = assign61590_e95937_d_n7;
        var_t0_dn8 = assign61590_e95937_d_n8;
        var_t0_dn9 = assign61590_e95937_d_n9;
        var_t0_dn10 = assign61590_e95937_d_n10;
        var_t0_dn13 = assign61590_e95937_d_n13;
        var_t0_rv = 0.0;

        let (assign61600_e95952, assign61600_e95952_d_n0, assign61600_e95952_d_n2, assign61600_e95952_d_n4, assign61600_e95952_d_n5, assign61600_e95952_d_n6, assign61600_e95952_d_n7, assign61600_e95952_d_n8, assign61600_e95952_d_n9, assign61600_e95952_d_n10, assign61600_e95952_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign61600_e95946: f64 = (0.6666666666666667 * var_vgvt);
        let assign61600_e95948: f64 = (assign61600_e95946 * var_qinm);
        let assign61600_e95950: f64 = (assign61600_e95948 / var_qidn);
        (assign61600_e95950, ((((((0.6666666666666667 * var_vgvt_dn0) * var_qinm) + (assign61600_e95946 * var_qinm_dn0)) * var_qidn) - (assign61600_e95948 * var_qidn_dn0)) / (var_qidn * var_qidn)), ((((((0.6666666666666667 * var_vgvt_dn2) * var_qinm) + (assign61600_e95946 * var_qinm_dn2)) * var_qidn) - (assign61600_e95948 * var_qidn_dn2)) / (var_qidn * var_qidn)), ((((((0.6666666666666667 * var_vgvt_dn4) * var_qinm) + (assign61600_e95946 * var_qinm_dn4)) * var_qidn) - (assign61600_e95948 * var_qidn_dn4)) / (var_qidn * var_qidn)), ((((((0.6666666666666667 * var_vgvt_dn5) * var_qinm) + (assign61600_e95946 * var_qinm_dn5)) * var_qidn) - (assign61600_e95948 * var_qidn_dn5)) / (var_qidn * var_qidn)), ((((((0.6666666666666667 * var_vgvt_dn6) * var_qinm) + (assign61600_e95946 * var_qinm_dn6)) * var_qidn) - (assign61600_e95948 * var_qidn_dn6)) / (var_qidn * var_qidn)), ((((((0.6666666666666667 * var_vgvt_dn7) * var_qinm) + (assign61600_e95946 * var_qinm_dn7)) * var_qidn) - (assign61600_e95948 * var_qidn_dn7)) / (var_qidn * var_qidn)), ((((((0.6666666666666667 * var_vgvt_dn8) * var_qinm) + (assign61600_e95946 * var_qinm_dn8)) * var_qidn) - (assign61600_e95948 * var_qidn_dn8)) / (var_qidn * var_qidn)), ((((((0.6666666666666667 * var_vgvt_dn9) * var_qinm) + (assign61600_e95946 * var_qinm_dn9)) * var_qidn) - (assign61600_e95948 * var_qidn_dn9)) / (var_qidn * var_qidn)), ((((((0.6666666666666667 * var_vgvt_dn10) * var_qinm) + (assign61600_e95946 * var_qinm_dn10)) * var_qidn) - (assign61600_e95948 * var_qidn_dn10)) / (var_qidn * var_qidn)), ((((((0.6666666666666667 * var_vgvt_dn13) * var_qinm) + (assign61600_e95946 * var_qinm_dn13)) * var_qidn) - (assign61600_e95948 * var_qidn_dn13)) / (var_qidn * var_qidn)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign61600_e95952;
        var_t1_dn0 = assign61600_e95952_d_n0;
        var_t1_dn2 = assign61600_e95952_d_n2;
        var_t1_dn4 = assign61600_e95952_d_n4;
        var_t1_dn5 = assign61600_e95952_d_n5;
        var_t1_dn6 = assign61600_e95952_d_n6;
        var_t1_dn7 = assign61600_e95952_d_n7;
        var_t1_dn8 = assign61600_e95952_d_n8;
        var_t1_dn9 = assign61600_e95952_d_n9;
        var_t1_dn10 = assign61600_e95952_d_n10;
        var_t1_dn13 = assign61600_e95952_d_n13;
        var_t1_rv = 0.0;

        let (assign61610_e95963, assign61610_e95963_d_n0, assign61610_e95963_d_n2, assign61610_e95963_d_n4, assign61610_e95963_d_n5, assign61610_e95963_d_n6, assign61610_e95963_d_n7, assign61610_e95963_d_n8, assign61610_e95963_d_n9, assign61610_e95963_d_n10, assign61610_e95963_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign61610_e95961: f64 = (var_t1 * var_cox);
        (assign61610_e95961, ((var_t1_dn0 * var_cox) + (var_t1 * var_cox_dn0)), ((var_t1_dn2 * var_cox) + (var_t1 * var_cox_dn2)), ((var_t1_dn4 * var_cox) + (var_t1 * var_cox_dn4)), ((var_t1_dn5 * var_cox) + (var_t1 * var_cox_dn5)), ((var_t1_dn6 * var_cox) + (var_t1 * var_cox_dn6)), ((var_t1_dn7 * var_cox) + (var_t1 * var_cox_dn7)), ((var_t1_dn8 * var_cox) + (var_t1 * var_cox_dn8)), ((var_t1_dn9 * var_cox) + (var_t1 * var_cox_dn9)), ((var_t1_dn10 * var_cox) + (var_t1 * var_cox_dn10)), ((var_t1_dn13 * var_cox) + (var_t1 * var_cox_dn13)),)
    } else {
        (var_qiu, var_qiu_dn0, var_qiu_dn2, var_qiu_dn4, var_qiu_dn5, var_qiu_dn6, var_qiu_dn7, var_qiu_dn8, var_qiu_dn9, var_qiu_dn10, var_qiu_dn13,)
    }
};
        var_qiu = assign61610_e95963;
        var_qiu_dn0 = assign61610_e95963_d_n0;
        var_qiu_dn2 = assign61610_e95963_d_n2;
        var_qiu_dn4 = assign61610_e95963_d_n4;
        var_qiu_dn5 = assign61610_e95963_d_n5;
        var_qiu_dn6 = assign61610_e95963_d_n6;
        var_qiu_dn7 = assign61610_e95963_d_n7;
        var_qiu_dn8 = assign61610_e95963_d_n8;
        var_qiu_dn9 = assign61610_e95963_d_n9;
        var_qiu_dn10 = assign61610_e95963_d_n10;
        var_qiu_dn13 = assign61610_e95963_d_n13;
        var_qiu_rv = 0.0;

        let (assign61620_e95974, assign61620_e95974_d_n0, assign61620_e95974_d_n2, assign61620_e95974_d_n4, assign61620_e95974_d_n5, assign61620_e95974_d_n6, assign61620_e95974_d_n7, assign61620_e95974_d_n8, assign61620_e95974_d_n9, assign61620_e95974_d_n10, assign61620_e95974_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign61620_e95972: f64 = (0.5 + var_alpha);
        (assign61620_e95972, var_alpha_dn0, var_alpha_dn2, var_alpha_dn4, var_alpha_dn5, var_alpha_dn6, var_alpha_dn7, var_alpha_dn8, var_alpha_dn9, var_alpha_dn10, var_alpha_dn13,)
    } else {
        (var_qdnm, var_qdnm_dn0, var_qdnm_dn2, var_qdnm_dn4, var_qdnm_dn5, var_qdnm_dn6, var_qdnm_dn7, var_qdnm_dn8, var_qdnm_dn9, var_qdnm_dn10, var_qdnm_dn13,)
    }
};
        var_qdnm = assign61620_e95974;
        var_qdnm_dn0 = assign61620_e95974_d_n0;
        var_qdnm_dn2 = assign61620_e95974_d_n2;
        var_qdnm_dn4 = assign61620_e95974_d_n4;
        var_qdnm_dn5 = assign61620_e95974_d_n5;
        var_qdnm_dn6 = assign61620_e95974_d_n6;
        var_qdnm_dn7 = assign61620_e95974_d_n7;
        var_qdnm_dn8 = assign61620_e95974_d_n8;
        var_qdnm_dn9 = assign61620_e95974_d_n9;
        var_qdnm_dn10 = assign61620_e95974_d_n10;
        var_qdnm_dn13 = assign61620_e95974_d_n13;
        var_qdnm_rv = 0.0;

        let (assign61630_e95985, assign61630_e95985_d_n0, assign61630_e95985_d_n2, assign61630_e95985_d_n4, assign61630_e95985_d_n5, assign61630_e95985_d_n6, assign61630_e95985_d_n7, assign61630_e95985_d_n8, assign61630_e95985_d_n9, assign61630_e95985_d_n10, assign61630_e95985_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign61630_e95983: f64 = (var_qidn * var_qinm);
        (assign61630_e95983, ((var_qidn_dn0 * var_qinm) + (var_qidn * var_qinm_dn0)), ((var_qidn_dn2 * var_qinm) + (var_qidn * var_qinm_dn2)), ((var_qidn_dn4 * var_qinm) + (var_qidn * var_qinm_dn4)), ((var_qidn_dn5 * var_qinm) + (var_qidn * var_qinm_dn5)), ((var_qidn_dn6 * var_qinm) + (var_qidn * var_qinm_dn6)), ((var_qidn_dn7 * var_qinm) + (var_qidn * var_qinm_dn7)), ((var_qidn_dn8 * var_qinm) + (var_qidn * var_qinm_dn8)), ((var_qidn_dn9 * var_qinm) + (var_qidn * var_qinm_dn9)), ((var_qidn_dn10 * var_qinm) + (var_qidn * var_qinm_dn10)), ((var_qidn_dn13 * var_qinm) + (var_qidn * var_qinm_dn13)),)
    } else {
        (var_qddn, var_qddn_dn0, var_qddn_dn2, var_qddn_dn4, var_qddn_dn5, var_qddn_dn6, var_qddn_dn7, var_qddn_dn8, var_qddn_dn9, var_qddn_dn10, var_qddn_dn13,)
    }
};
        var_qddn = assign61630_e95985;
        var_qddn_dn0 = assign61630_e95985_d_n0;
        var_qddn_dn2 = assign61630_e95985_d_n2;
        var_qddn_dn4 = assign61630_e95985_d_n4;
        var_qddn_dn5 = assign61630_e95985_d_n5;
        var_qddn_dn6 = assign61630_e95985_d_n6;
        var_qddn_dn7 = assign61630_e95985_d_n7;
        var_qddn_dn8 = assign61630_e95985_d_n8;
        var_qddn_dn9 = assign61630_e95985_d_n9;
        var_qddn_dn10 = assign61630_e95985_d_n10;
        var_qddn_dn13 = assign61630_e95985_d_n13;
        var_qddn_rv = 0.0;

        let (assign61640_e95998, assign61640_e95998_d_n0, assign61640_e95998_d_n2, assign61640_e95998_d_n4, assign61640_e95998_d_n5, assign61640_e95998_d_n6, assign61640_e95998_d_n7, assign61640_e95998_d_n8, assign61640_e95998_d_n9, assign61640_e95998_d_n10, assign61640_e95998_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign61640_e95994: f64 = (0.4 * var_qdnm);
        let assign61640_e95996: f64 = (assign61640_e95994 / var_qddn);
        (assign61640_e95996, ((((0.4 * var_qdnm_dn0) * var_qddn) - (assign61640_e95994 * var_qddn_dn0)) / (var_qddn * var_qddn)), ((((0.4 * var_qdnm_dn2) * var_qddn) - (assign61640_e95994 * var_qddn_dn2)) / (var_qddn * var_qddn)), ((((0.4 * var_qdnm_dn4) * var_qddn) - (assign61640_e95994 * var_qddn_dn4)) / (var_qddn * var_qddn)), ((((0.4 * var_qdnm_dn5) * var_qddn) - (assign61640_e95994 * var_qddn_dn5)) / (var_qddn * var_qddn)), ((((0.4 * var_qdnm_dn6) * var_qddn) - (assign61640_e95994 * var_qddn_dn6)) / (var_qddn * var_qddn)), ((((0.4 * var_qdnm_dn7) * var_qddn) - (assign61640_e95994 * var_qddn_dn7)) / (var_qddn * var_qddn)), ((((0.4 * var_qdnm_dn8) * var_qddn) - (assign61640_e95994 * var_qddn_dn8)) / (var_qddn * var_qddn)), ((((0.4 * var_qdnm_dn9) * var_qddn) - (assign61640_e95994 * var_qddn_dn9)) / (var_qddn * var_qddn)), ((((0.4 * var_qdnm_dn10) * var_qddn) - (assign61640_e95994 * var_qddn_dn10)) / (var_qddn * var_qddn)), ((((0.4 * var_qdnm_dn13) * var_qddn) - (assign61640_e95994 * var_qddn_dn13)) / (var_qddn * var_qddn)),)
    } else {
        (var_quot, var_quot_dn0, var_quot_dn2, var_quot_dn4, var_quot_dn5, var_quot_dn6, var_quot_dn7, var_quot_dn8, var_quot_dn9, var_quot_dn10, var_quot_dn13,)
    }
};
        var_quot = assign61640_e95998;
        var_quot_dn0 = assign61640_e95998_d_n0;
        var_quot_dn2 = assign61640_e95998_d_n2;
        var_quot_dn4 = assign61640_e95998_d_n4;
        var_quot_dn5 = assign61640_e95998_d_n5;
        var_quot_dn6 = assign61640_e95998_d_n6;
        var_quot_dn7 = assign61640_e95998_d_n7;
        var_quot_dn8 = assign61640_e95998_d_n8;
        var_quot_dn9 = assign61640_e95998_d_n9;
        var_quot_dn10 = assign61640_e95998_d_n10;
        var_quot_dn13 = assign61640_e95998_d_n13;
        var_quot_rv = 0.0;

        let (assign61650_e96009, assign61650_e96009_d_n0, assign61650_e96009_d_n2, assign61650_e96009_d_n4, assign61650_e96009_d_n5, assign61650_e96009_d_n6, assign61650_e96009_d_n7, assign61650_e96009_d_n8, assign61650_e96009_d_n9, assign61650_e96009_d_n10, assign61650_e96009_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) {
        let assign61650_e96007: f64 = (0.6 - var_quot);
        (assign61650_e96007, (-var_quot_dn0), (-var_quot_dn2), (-var_quot_dn4), (-var_quot_dn5), (-var_quot_dn6), (-var_quot_dn7), (-var_quot_dn8), (-var_quot_dn9), (-var_quot_dn10), (-var_quot_dn13),)
    } else {
        (var_qdrat, var_qdrat_dn0, var_qdrat_dn2, var_qdrat_dn4, var_qdrat_dn5, var_qdrat_dn6, var_qdrat_dn7, var_qdrat_dn8, var_qdrat_dn9, var_qdrat_dn10, var_qdrat_dn13,)
    }
};
        var_qdrat = assign61650_e96009;
        var_qdrat_dn0 = assign61650_e96009_d_n0;
        var_qdrat_dn2 = assign61650_e96009_d_n2;
        var_qdrat_dn4 = assign61650_e96009_d_n4;
        var_qdrat_dn5 = assign61650_e96009_d_n5;
        var_qdrat_dn6 = assign61650_e96009_d_n6;
        var_qdrat_dn7 = assign61650_e96009_d_n7;
        var_qdrat_dn8 = assign61650_e96009_d_n8;
        var_qdrat_dn9 = assign61650_e96009_d_n9;
        var_qdrat_dn10 = assign61650_e96009_d_n10;
        var_qdrat_dn13 = assign61650_e96009_d_n13;
        var_qdrat_rv = 0.0;

        let assign61660_e96012: f64 = if var_qdrat > 0.5 { 1.0 } else { 0.0 };
        var_guard1486 = assign61660_e96012;
        var_guard1486_rv = 0.0;

        let (assign61670_e96023, assign61670_e96023_d_n0, assign61670_e96023_d_n2, assign61670_e96023_d_n4, assign61670_e96023_d_n5, assign61670_e96023_d_n6, assign61670_e96023_d_n7, assign61670_e96023_d_n8, assign61670_e96023_d_n9, assign61670_e96023_d_n10, assign61670_e96023_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1486 != 0.0)) {
        (0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qdrat, var_qdrat_dn0, var_qdrat_dn2, var_qdrat_dn4, var_qdrat_dn5, var_qdrat_dn6, var_qdrat_dn7, var_qdrat_dn8, var_qdrat_dn9, var_qdrat_dn10, var_qdrat_dn13,)
    }
};
        var_qdrat = assign61670_e96023;
        var_qdrat_dn0 = assign61670_e96023_d_n0;
        var_qdrat_dn2 = assign61670_e96023_d_n2;
        var_qdrat_dn4 = assign61670_e96023_d_n4;
        var_qdrat_dn5 = assign61670_e96023_d_n5;
        var_qdrat_dn6 = assign61670_e96023_d_n6;
        var_qdrat_dn7 = assign61670_e96023_d_n7;
        var_qdrat_dn8 = assign61670_e96023_d_n8;
        var_qdrat_dn9 = assign61670_e96023_d_n9;
        var_qdrat_dn10 = assign61670_e96023_d_n10;
        var_qdrat_dn13 = assign61670_e96023_d_n13;
        var_qdrat_rv = 0.0;

        let assign61680_e96026: f64 = if var_flg_zone == 2.0 { 1.0 } else { 0.0 };
        var_guard1487 = assign61680_e96026;
        var_guard1487_rv = 0.0;

        let (assign61690_e96037, assign61690_e96037_d_n0, assign61690_e96037_d_n2, assign61690_e96037_d_n4, assign61690_e96037_d_n5, assign61690_e96037_d_n6, assign61690_e96037_d_n7, assign61690_e96037_d_n8, assign61690_e96037_d_n9, assign61690_e96037_d_n10, assign61690_e96037_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1487 != 0.0)) {
        (var_qbu, var_qbu_dn0, var_qbu_dn2, var_qbu_dn4, var_qbu_dn5, var_qbu_dn6, var_qbu_dn7, var_qbu_dn8, var_qbu_dn9, var_qbu_dn10, var_qbu_dn13,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign61690_e96037;
        var_t1_dn0 = assign61690_e96037_d_n0;
        var_t1_dn2 = assign61690_e96037_d_n2;
        var_t1_dn4 = assign61690_e96037_d_n4;
        var_t1_dn5 = assign61690_e96037_d_n5;
        var_t1_dn6 = assign61690_e96037_d_n6;
        var_t1_dn7 = assign61690_e96037_d_n7;
        var_t1_dn8 = assign61690_e96037_d_n8;
        var_t1_dn9 = assign61690_e96037_d_n9;
        var_t1_dn10 = assign61690_e96037_d_n10;
        var_t1_dn13 = assign61690_e96037_d_n13;
        var_t1_rv = 0.0;

        let (assign61700_e96056, assign61700_e96056_d_n0, assign61700_e96056_d_n2, assign61700_e96056_d_n4, assign61700_e96056_d_n5, assign61700_e96056_d_n6, assign61700_e96056_d_n7, assign61700_e96056_d_n8, assign61700_e96056_d_n9, assign61700_e96056_d_n10, assign61700_e96056_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1487 != 0.0)) {
        let assign61700_e96048: f64 = (var_fd2 * var_qbu);
        let assign61700_e96051: f64 = (1.0 - var_fd2);
        let assign61700_e96053: f64 = (assign61700_e96051 * var_qb0);
        let assign61700_e96054: f64 = (assign61700_e96048 + assign61700_e96053);
        (assign61700_e96054, (((var_fd2_dn0 * var_qbu) + (var_fd2 * var_qbu_dn0)) + (((-var_fd2_dn0) * var_qb0) + (assign61700_e96051 * var_qb0_dn0))), (((var_fd2_dn2 * var_qbu) + (var_fd2 * var_qbu_dn2)) + (((-var_fd2_dn2) * var_qb0) + (assign61700_e96051 * var_qb0_dn2))), (((var_fd2_dn4 * var_qbu) + (var_fd2 * var_qbu_dn4)) + (((-var_fd2_dn4) * var_qb0) + (assign61700_e96051 * var_qb0_dn4))), (((var_fd2_dn5 * var_qbu) + (var_fd2 * var_qbu_dn5)) + (((-var_fd2_dn5) * var_qb0) + (assign61700_e96051 * var_qb0_dn5))), (((var_fd2_dn6 * var_qbu) + (var_fd2 * var_qbu_dn6)) + (((-var_fd2_dn6) * var_qb0) + (assign61700_e96051 * var_qb0_dn6))), (((var_fd2_dn7 * var_qbu) + (var_fd2 * var_qbu_dn7)) + (((-var_fd2_dn7) * var_qb0) + (assign61700_e96051 * var_qb0_dn7))), (((var_fd2_dn8 * var_qbu) + (var_fd2 * var_qbu_dn8)) + (((-var_fd2_dn8) * var_qb0) + (assign61700_e96051 * var_qb0_dn8))), (((var_fd2_dn9 * var_qbu) + (var_fd2 * var_qbu_dn9)) + (((-var_fd2_dn9) * var_qb0) + (assign61700_e96051 * var_qb0_dn9))), (((var_fd2_dn10 * var_qbu) + (var_fd2 * var_qbu_dn10)) + (((-var_fd2_dn10) * var_qb0) + (assign61700_e96051 * var_qb0_dn10))), (((var_fd2_dn13 * var_qbu) + (var_fd2 * var_qbu_dn13)) + (((-var_fd2_dn13) * var_qb0) + (assign61700_e96051 * var_qb0_dn13))),)
    } else {
        (var_qbu, var_qbu_dn0, var_qbu_dn2, var_qbu_dn4, var_qbu_dn5, var_qbu_dn6, var_qbu_dn7, var_qbu_dn8, var_qbu_dn9, var_qbu_dn10, var_qbu_dn13,)
    }
};
        var_qbu = assign61700_e96056;
        var_qbu_dn0 = assign61700_e96056_d_n0;
        var_qbu_dn2 = assign61700_e96056_d_n2;
        var_qbu_dn4 = assign61700_e96056_d_n4;
        var_qbu_dn5 = assign61700_e96056_d_n5;
        var_qbu_dn6 = assign61700_e96056_d_n6;
        var_qbu_dn7 = assign61700_e96056_d_n7;
        var_qbu_dn8 = assign61700_e96056_d_n8;
        var_qbu_dn9 = assign61700_e96056_d_n9;
        var_qbu_dn10 = assign61700_e96056_d_n10;
        var_qbu_dn13 = assign61700_e96056_d_n13;
        var_qbu_rv = 0.0;

        let assign61710_e96059: f64 = if var_qbu < 0.0 { 1.0 } else { 0.0 };
        var_guard1488 = assign61710_e96059;
        var_guard1488_rv = 0.0;

        let (assign61720_e96072, assign61720_e96072_d_n0, assign61720_e96072_d_n2, assign61720_e96072_d_n4, assign61720_e96072_d_n5, assign61720_e96072_d_n6, assign61720_e96072_d_n7, assign61720_e96072_d_n8, assign61720_e96072_d_n9, assign61720_e96072_d_n10, assign61720_e96072_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1487 != 0.0)) && (var_guard1488 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qbu, var_qbu_dn0, var_qbu_dn2, var_qbu_dn4, var_qbu_dn5, var_qbu_dn6, var_qbu_dn7, var_qbu_dn8, var_qbu_dn9, var_qbu_dn10, var_qbu_dn13,)
    }
};
        var_qbu = assign61720_e96072;
        var_qbu_dn0 = assign61720_e96072_d_n0;
        var_qbu_dn2 = assign61720_e96072_d_n2;
        var_qbu_dn4 = assign61720_e96072_d_n4;
        var_qbu_dn5 = assign61720_e96072_d_n5;
        var_qbu_dn6 = assign61720_e96072_d_n6;
        var_qbu_dn7 = assign61720_e96072_d_n7;
        var_qbu_dn8 = assign61720_e96072_d_n8;
        var_qbu_dn9 = assign61720_e96072_d_n9;
        var_qbu_dn10 = assign61720_e96072_d_n10;
        var_qbu_dn13 = assign61720_e96072_d_n13;
        var_qbu_rv = 0.0;

        let (assign61730_e96083, assign61730_e96083_d_n0, assign61730_e96083_d_n2, assign61730_e96083_d_n4, assign61730_e96083_d_n5, assign61730_e96083_d_n6, assign61730_e96083_d_n7, assign61730_e96083_d_n8, assign61730_e96083_d_n9, assign61730_e96083_d_n10, assign61730_e96083_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1487 != 0.0)) {
        (var_qiu, var_qiu_dn0, var_qiu_dn2, var_qiu_dn4, var_qiu_dn5, var_qiu_dn6, var_qiu_dn7, var_qiu_dn8, var_qiu_dn9, var_qiu_dn10, var_qiu_dn13,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign61730_e96083;
        var_t1_dn0 = assign61730_e96083_d_n0;
        var_t1_dn2 = assign61730_e96083_d_n2;
        var_t1_dn4 = assign61730_e96083_d_n4;
        var_t1_dn5 = assign61730_e96083_d_n5;
        var_t1_dn6 = assign61730_e96083_d_n6;
        var_t1_dn7 = assign61730_e96083_d_n7;
        var_t1_dn8 = assign61730_e96083_d_n8;
        var_t1_dn9 = assign61730_e96083_d_n9;
        var_t1_dn10 = assign61730_e96083_d_n10;
        var_t1_dn13 = assign61730_e96083_d_n13;
        var_t1_rv = 0.0;

        let (assign61740_e96102, assign61740_e96102_d_n0, assign61740_e96102_d_n2, assign61740_e96102_d_n4, assign61740_e96102_d_n5, assign61740_e96102_d_n6, assign61740_e96102_d_n7, assign61740_e96102_d_n8, assign61740_e96102_d_n9, assign61740_e96102_d_n10, assign61740_e96102_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1487 != 0.0)) {
        let assign61740_e96094: f64 = (var_fd2 * var_qiu);
        let assign61740_e96097: f64 = (1.0 - var_fd2);
        let assign61740_e96099: f64 = (assign61740_e96097 * var_qn0);
        let assign61740_e96100: f64 = (assign61740_e96094 + assign61740_e96099);
        (assign61740_e96100, (((var_fd2_dn0 * var_qiu) + (var_fd2 * var_qiu_dn0)) + (((-var_fd2_dn0) * var_qn0) + (assign61740_e96097 * var_qn0_dn0))), (((var_fd2_dn2 * var_qiu) + (var_fd2 * var_qiu_dn2)) + (((-var_fd2_dn2) * var_qn0) + (assign61740_e96097 * var_qn0_dn2))), (((var_fd2_dn4 * var_qiu) + (var_fd2 * var_qiu_dn4)) + (((-var_fd2_dn4) * var_qn0) + (assign61740_e96097 * var_qn0_dn4))), (((var_fd2_dn5 * var_qiu) + (var_fd2 * var_qiu_dn5)) + (((-var_fd2_dn5) * var_qn0) + (assign61740_e96097 * var_qn0_dn5))), (((var_fd2_dn6 * var_qiu) + (var_fd2 * var_qiu_dn6)) + (((-var_fd2_dn6) * var_qn0) + (assign61740_e96097 * var_qn0_dn6))), (((var_fd2_dn7 * var_qiu) + (var_fd2 * var_qiu_dn7)) + (((-var_fd2_dn7) * var_qn0) + (assign61740_e96097 * var_qn0_dn7))), (((var_fd2_dn8 * var_qiu) + (var_fd2 * var_qiu_dn8)) + (((-var_fd2_dn8) * var_qn0) + (assign61740_e96097 * var_qn0_dn8))), (((var_fd2_dn9 * var_qiu) + (var_fd2 * var_qiu_dn9)) + (((-var_fd2_dn9) * var_qn0) + (assign61740_e96097 * var_qn0_dn9))), (((var_fd2_dn10 * var_qiu) + (var_fd2 * var_qiu_dn10)) + (((-var_fd2_dn10) * var_qn0) + (assign61740_e96097 * var_qn0_dn10))), (((var_fd2_dn13 * var_qiu) + (var_fd2 * var_qiu_dn13)) + (((-var_fd2_dn13) * var_qn0) + (assign61740_e96097 * var_qn0_dn13))),)
    } else {
        (var_qiu, var_qiu_dn0, var_qiu_dn2, var_qiu_dn4, var_qiu_dn5, var_qiu_dn6, var_qiu_dn7, var_qiu_dn8, var_qiu_dn9, var_qiu_dn10, var_qiu_dn13,)
    }
};
        var_qiu = assign61740_e96102;
        var_qiu_dn0 = assign61740_e96102_d_n0;
        var_qiu_dn2 = assign61740_e96102_d_n2;
        var_qiu_dn4 = assign61740_e96102_d_n4;
        var_qiu_dn5 = assign61740_e96102_d_n5;
        var_qiu_dn6 = assign61740_e96102_d_n6;
        var_qiu_dn7 = assign61740_e96102_d_n7;
        var_qiu_dn8 = assign61740_e96102_d_n8;
        var_qiu_dn9 = assign61740_e96102_d_n9;
        var_qiu_dn10 = assign61740_e96102_d_n10;
        var_qiu_dn13 = assign61740_e96102_d_n13;
        var_qiu_rv = 0.0;

        let assign61750_e96105: f64 = if var_qiu < 0.0 { 1.0 } else { 0.0 };
        var_guard1489 = assign61750_e96105;
        var_guard1489_rv = 0.0;

        let (assign61760_e96118, assign61760_e96118_d_n0, assign61760_e96118_d_n2, assign61760_e96118_d_n4, assign61760_e96118_d_n5, assign61760_e96118_d_n6, assign61760_e96118_d_n7, assign61760_e96118_d_n8, assign61760_e96118_d_n9, assign61760_e96118_d_n10, assign61760_e96118_d_n13,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1487 != 0.0)) && (var_guard1489 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_qiu, var_qiu_dn0, var_qiu_dn2, var_qiu_dn4, var_qiu_dn5, var_qiu_dn6, var_qiu_dn7, var_qiu_dn8, var_qiu_dn9, var_qiu_dn10, var_qiu_dn13,)
    }
};
        var_qiu = assign61760_e96118;
        var_qiu_dn0 = assign61760_e96118_d_n0;
        var_qiu_dn2 = assign61760_e96118_d_n2;
        var_qiu_dn4 = assign61760_e96118_d_n4;
        var_qiu_dn5 = assign61760_e96118_d_n5;
        var_qiu_dn6 = assign61760_e96118_d_n6;
        var_qiu_dn7 = assign61760_e96118_d_n7;
        var_qiu_dn8 = assign61760_e96118_d_n8;
        var_qiu_dn9 = assign61760_e96118_d_n9;
        var_qiu_dn10 = assign61760_e96118_d_n10;
        var_qiu_dn13 = assign61760_e96118_d_n13;
        var_qiu_rv = 0.0;

        let (assign61770_e96129, assign61770_e96129_d_n0, assign61770_e96129_d_n2, assign61770_e96129_d_n4, assign61770_e96129_d_n5, assign61770_e96129_d_n6, assign61770_e96129_d_n7, assign61770_e96129_d_n8, assign61770_e96129_d_n9, assign61770_e96129_d_n10, assign61770_e96129_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1487 != 0.0)) {
        (var_qdrat, var_qdrat_dn0, var_qdrat_dn2, var_qdrat_dn4, var_qdrat_dn5, var_qdrat_dn6, var_qdrat_dn7, var_qdrat_dn8, var_qdrat_dn9, var_qdrat_dn10, var_qdrat_dn13,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign61770_e96129;
        var_t1_dn0 = assign61770_e96129_d_n0;
        var_t1_dn2 = assign61770_e96129_d_n2;
        var_t1_dn4 = assign61770_e96129_d_n4;
        var_t1_dn5 = assign61770_e96129_d_n5;
        var_t1_dn6 = assign61770_e96129_d_n6;
        var_t1_dn7 = assign61770_e96129_d_n7;
        var_t1_dn8 = assign61770_e96129_d_n8;
        var_t1_dn9 = assign61770_e96129_d_n9;
        var_t1_dn10 = assign61770_e96129_d_n10;
        var_t1_dn13 = assign61770_e96129_d_n13;
        var_t1_rv = 0.0;

        let (assign61780_e96148, assign61780_e96148_d_n0, assign61780_e96148_d_n2, assign61780_e96148_d_n4, assign61780_e96148_d_n5, assign61780_e96148_d_n6, assign61780_e96148_d_n7, assign61780_e96148_d_n8, assign61780_e96148_d_n9, assign61780_e96148_d_n10, assign61780_e96148_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1487 != 0.0)) {
        let assign61780_e96140: f64 = (var_fd2 * var_qdrat);
        let assign61780_e96143: f64 = (1.0 - var_fd2);
        let assign61780_e96145: f64 = (assign61780_e96143 * 0.5);
        let assign61780_e96146: f64 = (assign61780_e96140 + assign61780_e96145);
        (assign61780_e96146, (((var_fd2_dn0 * var_qdrat) + (var_fd2 * var_qdrat_dn0)) + ((-var_fd2_dn0) * 0.5)), (((var_fd2_dn2 * var_qdrat) + (var_fd2 * var_qdrat_dn2)) + ((-var_fd2_dn2) * 0.5)), (((var_fd2_dn4 * var_qdrat) + (var_fd2 * var_qdrat_dn4)) + ((-var_fd2_dn4) * 0.5)), (((var_fd2_dn5 * var_qdrat) + (var_fd2 * var_qdrat_dn5)) + ((-var_fd2_dn5) * 0.5)), (((var_fd2_dn6 * var_qdrat) + (var_fd2 * var_qdrat_dn6)) + ((-var_fd2_dn6) * 0.5)), (((var_fd2_dn7 * var_qdrat) + (var_fd2 * var_qdrat_dn7)) + ((-var_fd2_dn7) * 0.5)), (((var_fd2_dn8 * var_qdrat) + (var_fd2 * var_qdrat_dn8)) + ((-var_fd2_dn8) * 0.5)), (((var_fd2_dn9 * var_qdrat) + (var_fd2 * var_qdrat_dn9)) + ((-var_fd2_dn9) * 0.5)), (((var_fd2_dn10 * var_qdrat) + (var_fd2 * var_qdrat_dn10)) + ((-var_fd2_dn10) * 0.5)), (((var_fd2_dn13 * var_qdrat) + (var_fd2 * var_qdrat_dn13)) + ((-var_fd2_dn13) * 0.5)),)
    } else {
        (var_qdrat, var_qdrat_dn0, var_qdrat_dn2, var_qdrat_dn4, var_qdrat_dn5, var_qdrat_dn6, var_qdrat_dn7, var_qdrat_dn8, var_qdrat_dn9, var_qdrat_dn10, var_qdrat_dn13,)
    }
};
        var_qdrat = assign61780_e96148;
        var_qdrat_dn0 = assign61780_e96148_d_n0;
        var_qdrat_dn2 = assign61780_e96148_d_n2;
        var_qdrat_dn4 = assign61780_e96148_d_n4;
        var_qdrat_dn5 = assign61780_e96148_d_n5;
        var_qdrat_dn6 = assign61780_e96148_d_n6;
        var_qdrat_dn7 = assign61780_e96148_d_n7;
        var_qdrat_dn8 = assign61780_e96148_d_n8;
        var_qdrat_dn9 = assign61780_e96148_d_n9;
        var_qdrat_dn10 = assign61780_e96148_d_n10;
        var_qdrat_dn13 = assign61780_e96148_d_n13;
        var_qdrat_rv = 0.0;

        let (assign61790_e96159, assign61790_e96159_d_n0, assign61790_e96159_d_n2, assign61790_e96159_d_n4, assign61790_e96159_d_n5, assign61790_e96159_d_n6, assign61790_e96159_d_n7, assign61790_e96159_d_n8, assign61790_e96159_d_n9, assign61790_e96159_d_n10, assign61790_e96159_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1487 != 0.0)) {
        (var_lred, var_lred_dn0, var_lred_dn2, var_lred_dn4, var_lred_dn5, var_lred_dn6, var_lred_dn7, var_lred_dn8, var_lred_dn9, var_lred_dn10, var_lred_dn13,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign61790_e96159;
        var_t1_dn0 = assign61790_e96159_d_n0;
        var_t1_dn2 = assign61790_e96159_d_n2;
        var_t1_dn4 = assign61790_e96159_d_n4;
        var_t1_dn5 = assign61790_e96159_d_n5;
        var_t1_dn6 = assign61790_e96159_d_n6;
        var_t1_dn7 = assign61790_e96159_d_n7;
        var_t1_dn8 = assign61790_e96159_d_n8;
        var_t1_dn9 = assign61790_e96159_d_n9;
        var_t1_dn10 = assign61790_e96159_d_n10;
        var_t1_dn13 = assign61790_e96159_d_n13;
        var_t1_rv = 0.0;

        let (assign61800_e96172, assign61800_e96172_d_n0, assign61800_e96172_d_n2, assign61800_e96172_d_n4, assign61800_e96172_d_n5, assign61800_e96172_d_n6, assign61800_e96172_d_n7, assign61800_e96172_d_n8, assign61800_e96172_d_n9, assign61800_e96172_d_n10, assign61800_e96172_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1461 != 0.0)) && (var_guard1487 != 0.0)) {
        let assign61800_e96170: f64 = (var_fd2 * var_lred);
        (assign61800_e96170, ((var_fd2_dn0 * var_lred) + (var_fd2 * var_lred_dn0)), ((var_fd2_dn2 * var_lred) + (var_fd2 * var_lred_dn2)), ((var_fd2_dn4 * var_lred) + (var_fd2 * var_lred_dn4)), ((var_fd2_dn5 * var_lred) + (var_fd2 * var_lred_dn5)), ((var_fd2_dn6 * var_lred) + (var_fd2 * var_lred_dn6)), ((var_fd2_dn7 * var_lred) + (var_fd2 * var_lred_dn7)), ((var_fd2_dn8 * var_lred) + (var_fd2 * var_lred_dn8)), ((var_fd2_dn9 * var_lred) + (var_fd2 * var_lred_dn9)), ((var_fd2_dn10 * var_lred) + (var_fd2 * var_lred_dn10)), ((var_fd2_dn13 * var_lred) + (var_fd2 * var_lred_dn13)),)
    } else {
        (var_lred, var_lred_dn0, var_lred_dn2, var_lred_dn4, var_lred_dn5, var_lred_dn6, var_lred_dn7, var_lred_dn8, var_lred_dn9, var_lred_dn10, var_lred_dn13,)
    }
};
        var_lred = assign61800_e96172;
        var_lred_dn0 = assign61800_e96172_d_n0;
        var_lred_dn2 = assign61800_e96172_d_n2;
        var_lred_dn4 = assign61800_e96172_d_n4;
        var_lred_dn5 = assign61800_e96172_d_n5;
        var_lred_dn6 = assign61800_e96172_d_n6;
        var_lred_dn7 = assign61800_e96172_d_n7;
        var_lred_dn8 = assign61800_e96172_d_n8;
        var_lred_dn9 = assign61800_e96172_d_n9;
        var_lred_dn10 = assign61800_e96172_d_n10;
        var_lred_dn13 = assign61800_e96172_d_n13;
        var_lred_rv = 0.0;

        let (assign61810_e96181,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_start_of_mobility != 0.0)) {
        (0.0,)
    } else {
        (var_start_of_mobility,)
    }
};
        var_start_of_mobility = assign61810_e96181;
        var_start_of_mobility_rv = 0.0;

        let (assign61820_e96190, assign61820_e96190_d_n0, assign61820_e96190_d_n2, assign61820_e96190_d_n4, assign61820_e96190_d_n5, assign61820_e96190_d_n6, assign61820_e96190_d_n7, assign61820_e96190_d_n8, assign61820_e96190_d_n9, assign61820_e96190_d_n10, assign61820_e96190_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign61820_e96188: f64 = (var_leff - var_lred);
        (assign61820_e96188, (-var_lred_dn0), (-var_lred_dn2), (-var_lred_dn4), (-var_lred_dn5), (-var_lred_dn6), (-var_lred_dn7), (-var_lred_dn8), (-var_lred_dn9), (-var_lred_dn10), (-var_lred_dn13),)
    } else {
        (var_lch, var_lch_dn0, var_lch_dn2, var_lch_dn4, var_lch_dn5, var_lch_dn6, var_lch_dn7, var_lch_dn8, var_lch_dn9, var_lch_dn10, var_lch_dn13,)
    }
};
        var_lch = assign61820_e96190;
        var_lch_dn0 = assign61820_e96190_d_n0;
        var_lch_dn2 = assign61820_e96190_d_n2;
        var_lch_dn4 = assign61820_e96190_d_n4;
        var_lch_dn5 = assign61820_e96190_d_n5;
        var_lch_dn6 = assign61820_e96190_d_n6;
        var_lch_dn7 = assign61820_e96190_d_n7;
        var_lch_dn8 = assign61820_e96190_d_n8;
        var_lch_dn9 = assign61820_e96190_d_n9;
        var_lch_dn10 = assign61820_e96190_d_n10;
        var_lch_dn13 = assign61820_e96190_d_n13;
        var_lch_rv = 0.0;

        let assign61830_e96193: f64 = if var_lch < 1e-9 { 1.0 } else { 0.0 };
        var_guard1490 = assign61830_e96193;
        var_guard1490_rv = 0.0;

        *var_guard1486_slot = var_guard1486;
        *var_guard1486_rv_slot = var_guard1486_rv;
        *var_guard1487_slot = var_guard1487;
        *var_guard1487_rv_slot = var_guard1487_rv;
        *var_guard1488_slot = var_guard1488;
        *var_guard1488_rv_slot = var_guard1488_rv;
        *var_guard1489_slot = var_guard1489;
        *var_guard1489_rv_slot = var_guard1489_rv;
        *var_guard1490_slot = var_guard1490;
        *var_guard1490_rv_slot = var_guard1490_rv;
        *var_lch_slot = var_lch;
        *var_lch_dn0_slot = var_lch_dn0;
        *var_lch_dn10_slot = var_lch_dn10;
        *var_lch_dn13_slot = var_lch_dn13;
        *var_lch_dn2_slot = var_lch_dn2;
        *var_lch_dn4_slot = var_lch_dn4;
        *var_lch_dn5_slot = var_lch_dn5;
        *var_lch_dn6_slot = var_lch_dn6;
        *var_lch_dn7_slot = var_lch_dn7;
        *var_lch_dn8_slot = var_lch_dn8;
        *var_lch_dn9_slot = var_lch_dn9;
        *var_lch_rv_slot = var_lch_rv;
        *var_lred_slot = var_lred;
        *var_lred_dn0_slot = var_lred_dn0;
        *var_lred_dn10_slot = var_lred_dn10;
        *var_lred_dn13_slot = var_lred_dn13;
        *var_lred_dn2_slot = var_lred_dn2;
        *var_lred_dn4_slot = var_lred_dn4;
        *var_lred_dn5_slot = var_lred_dn5;
        *var_lred_dn6_slot = var_lred_dn6;
        *var_lred_dn7_slot = var_lred_dn7;
        *var_lred_dn8_slot = var_lred_dn8;
        *var_lred_dn9_slot = var_lred_dn9;
        *var_lred_rv_slot = var_lred_rv;
        *var_qbu_slot = var_qbu;
        *var_qbu_dn0_slot = var_qbu_dn0;
        *var_qbu_dn10_slot = var_qbu_dn10;
        *var_qbu_dn13_slot = var_qbu_dn13;
        *var_qbu_dn2_slot = var_qbu_dn2;
        *var_qbu_dn4_slot = var_qbu_dn4;
        *var_qbu_dn5_slot = var_qbu_dn5;
        *var_qbu_dn6_slot = var_qbu_dn6;
        *var_qbu_dn7_slot = var_qbu_dn7;
        *var_qbu_dn8_slot = var_qbu_dn8;
        *var_qbu_dn9_slot = var_qbu_dn9;
        *var_qbu_rv_slot = var_qbu_rv;
        *var_qddn_slot = var_qddn;
        *var_qddn_dn0_slot = var_qddn_dn0;
        *var_qddn_dn10_slot = var_qddn_dn10;
        *var_qddn_dn13_slot = var_qddn_dn13;
        *var_qddn_dn2_slot = var_qddn_dn2;
        *var_qddn_dn4_slot = var_qddn_dn4;
        *var_qddn_dn5_slot = var_qddn_dn5;
        *var_qddn_dn6_slot = var_qddn_dn6;
        *var_qddn_dn7_slot = var_qddn_dn7;
        *var_qddn_dn8_slot = var_qddn_dn8;
        *var_qddn_dn9_slot = var_qddn_dn9;
        *var_qddn_rv_slot = var_qddn_rv;
        *var_qdnm_slot = var_qdnm;
        *var_qdnm_dn0_slot = var_qdnm_dn0;
        *var_qdnm_dn10_slot = var_qdnm_dn10;
        *var_qdnm_dn13_slot = var_qdnm_dn13;
        *var_qdnm_dn2_slot = var_qdnm_dn2;
        *var_qdnm_dn4_slot = var_qdnm_dn4;
        *var_qdnm_dn5_slot = var_qdnm_dn5;
        *var_qdnm_dn6_slot = var_qdnm_dn6;
        *var_qdnm_dn7_slot = var_qdnm_dn7;
        *var_qdnm_dn8_slot = var_qdnm_dn8;
        *var_qdnm_dn9_slot = var_qdnm_dn9;
        *var_qdnm_rv_slot = var_qdnm_rv;
        *var_qdrat_slot = var_qdrat;
        *var_qdrat_dn0_slot = var_qdrat_dn0;
        *var_qdrat_dn10_slot = var_qdrat_dn10;
        *var_qdrat_dn13_slot = var_qdrat_dn13;
        *var_qdrat_dn2_slot = var_qdrat_dn2;
        *var_qdrat_dn4_slot = var_qdrat_dn4;
        *var_qdrat_dn5_slot = var_qdrat_dn5;
        *var_qdrat_dn6_slot = var_qdrat_dn6;
        *var_qdrat_dn7_slot = var_qdrat_dn7;
        *var_qdrat_dn8_slot = var_qdrat_dn8;
        *var_qdrat_dn9_slot = var_qdrat_dn9;
        *var_qdrat_rv_slot = var_qdrat_rv;
        *var_qidn_slot = var_qidn;
        *var_qidn_dn0_slot = var_qidn_dn0;
        *var_qidn_dn10_slot = var_qidn_dn10;
        *var_qidn_dn13_slot = var_qidn_dn13;
        *var_qidn_dn2_slot = var_qidn_dn2;
        *var_qidn_dn4_slot = var_qidn_dn4;
        *var_qidn_dn5_slot = var_qidn_dn5;
        *var_qidn_dn6_slot = var_qidn_dn6;
        *var_qidn_dn7_slot = var_qidn_dn7;
        *var_qidn_dn8_slot = var_qidn_dn8;
        *var_qidn_dn9_slot = var_qidn_dn9;
        *var_qidn_rv_slot = var_qidn_rv;
        *var_qiu_slot = var_qiu;
        *var_qiu_dn0_slot = var_qiu_dn0;
        *var_qiu_dn10_slot = var_qiu_dn10;
        *var_qiu_dn13_slot = var_qiu_dn13;
        *var_qiu_dn2_slot = var_qiu_dn2;
        *var_qiu_dn4_slot = var_qiu_dn4;
        *var_qiu_dn5_slot = var_qiu_dn5;
        *var_qiu_dn6_slot = var_qiu_dn6;
        *var_qiu_dn7_slot = var_qiu_dn7;
        *var_qiu_dn8_slot = var_qiu_dn8;
        *var_qiu_dn9_slot = var_qiu_dn9;
        *var_qiu_rv_slot = var_qiu_rv;
        *var_quot_slot = var_quot;
        *var_quot_dn0_slot = var_quot_dn0;
        *var_quot_dn10_slot = var_quot_dn10;
        *var_quot_dn13_slot = var_quot_dn13;
        *var_quot_dn2_slot = var_quot_dn2;
        *var_quot_dn4_slot = var_quot_dn4;
        *var_quot_dn5_slot = var_quot_dn5;
        *var_quot_dn6_slot = var_quot_dn6;
        *var_quot_dn7_slot = var_quot_dn7;
        *var_quot_dn8_slot = var_quot_dn8;
        *var_quot_dn9_slot = var_quot_dn9;
        *var_quot_rv_slot = var_quot_rv;
        *var_start_of_mobility_slot = var_start_of_mobility;
        *var_start_of_mobility_rv_slot = var_start_of_mobility_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn13_slot = var_t0_dn13;
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
        *var_t1_dn13_slot = var_t1_dn13;
        *var_t1_dn2_slot = var_t1_dn2;
        *var_t1_dn4_slot = var_t1_dn4;
        *var_t1_dn5_slot = var_t1_dn5;
        *var_t1_dn6_slot = var_t1_dn6;
        *var_t1_dn7_slot = var_t1_dn7;
        *var_t1_dn8_slot = var_t1_dn8;
        *var_t1_dn9_slot = var_t1_dn9;
        *var_t1_rv_slot = var_t1_rv;
    }

    pub(super) fn stamp_reactive_block_221(
        p: &Parameters,
        var_beta: f64,
        var_beta_dn0: f64,
        var_beta_dn10: f64,
        var_beta_dn13: f64,
        var_beta_dn2: f64,
        var_beta_dn4: f64,
        var_beta_dn5: f64,
        var_beta_dn6: f64,
        var_beta_dn7: f64,
        var_beta_dn8: f64,
        var_beta_dn9: f64,
        var_guard1430: f64,
        var_guard1490: f64,
        var_guard443: f64,
        var_mphn0: f64,
        var_mphn0_dn0: f64,
        var_mphn0_dn10: f64,
        var_mphn0_dn13: f64,
        var_mphn0_dn2: f64,
        var_mphn0_dn4: f64,
        var_mphn0_dn5: f64,
        var_mphn0_dn6: f64,
        var_mphn0_dn7: f64,
        var_mphn0_dn8: f64,
        var_mphn0_dn9: f64,
        var_muesr: f64,
        var_ndep_o_esi: f64,
        var_ndep_o_esi_dn0: f64,
        var_ndep_o_esi_dn10: f64,
        var_ndep_o_esi_dn13: f64,
        var_ndep_o_esi_dn2: f64,
        var_ndep_o_esi_dn4: f64,
        var_ndep_o_esi_dn5: f64,
        var_ndep_o_esi_dn6: f64,
        var_ndep_o_esi_dn7: f64,
        var_ndep_o_esi_dn8: f64,
        var_ndep_o_esi_dn9: f64,
        var_ninv_o_esi: f64,
        var_ninvde: f64,
        var_ninvde_dn0: f64,
        var_ninvde_dn10: f64,
        var_ninvde_dn13: f64,
        var_ninvde_dn2: f64,
        var_ninvde_dn4: f64,
        var_ninvde_dn5: f64,
        var_ninvde_dn6: f64,
        var_ninvde_dn7: f64,
        var_ninvde_dn8: f64,
        var_ninvde_dn9: f64,
        var_ps0: f64,
        var_ps0_dn0: f64,
        var_ps0_dn10: f64,
        var_ps0_dn13: f64,
        var_ps0_dn2: f64,
        var_ps0_dn4: f64,
        var_ps0_dn5: f64,
        var_ps0_dn6: f64,
        var_ps0_dn7: f64,
        var_ps0_dn8: f64,
        var_ps0_dn9: f64,
        var_psl: f64,
        var_psl_dn0: f64,
        var_psl_dn10: f64,
        var_psl_dn13: f64,
        var_psl_dn2: f64,
        var_psl_dn4: f64,
        var_psl_dn5: f64,
        var_psl_dn6: f64,
        var_psl_dn7: f64,
        var_psl_dn8: f64,
        var_psl_dn9: f64,
        var_qbu: f64,
        var_qbu_dn0: f64,
        var_qbu_dn10: f64,
        var_qbu_dn13: f64,
        var_qbu_dn2: f64,
        var_qbu_dn4: f64,
        var_qbu_dn5: f64,
        var_qbu_dn6: f64,
        var_qbu_dn7: f64,
        var_qbu_dn8: f64,
        var_qbu_dn9: f64,
        var_qiu: f64,
        var_qiu_dn0: f64,
        var_qiu_dn10: f64,
        var_qiu_dn13: f64,
        var_qiu_dn2: f64,
        var_qiu_dn4: f64,
        var_qiu_dn5: f64,
        var_qiu_dn6: f64,
        var_qiu_dn7: f64,
        var_qiu_dn8: f64,
        var_qiu_dn9: f64,
        var_qn0: f64,
        var_qn0_dn0: f64,
        var_qn0_dn10: f64,
        var_qn0_dn13: f64,
        var_qn0_dn2: f64,
        var_qn0_dn4: f64,
        var_qn0_dn5: f64,
        var_qn0_dn6: f64,
        var_qn0_dn7: f64,
        var_qn0_dn8: f64,
        var_qn0_dn9: f64,
        var_uc_muecb0: f64,
        var_uc_muecb1: f64,
        var_uc_muesr1: f64,
        var_vbsz__blk438: f64,
        var_vbsz__blk438_dn0: f64,
        var_vbsz__blk438_dn10: f64,
        var_vbsz__blk438_dn13: f64,
        var_vbsz__blk438_dn2: f64,
        var_vbsz__blk438_dn4: f64,
        var_vbsz__blk438_dn5: f64,
        var_vbsz__blk438_dn6: f64,
        var_vbsz__blk438_dn7: f64,
        var_vbsz__blk438_dn8: f64,
        var_vbsz__blk438_dn9: f64,
        var_eeff_slot: &mut f64,
        var_eeff_dn0_slot: &mut f64,
        var_eeff_dn10_slot: &mut f64,
        var_eeff_dn13_slot: &mut f64,
        var_eeff_dn2_slot: &mut f64,
        var_eeff_dn4_slot: &mut f64,
        var_eeff_dn5_slot: &mut f64,
        var_eeff_dn6_slot: &mut f64,
        var_eeff_dn7_slot: &mut f64,
        var_eeff_dn8_slot: &mut f64,
        var_eeff_dn9_slot: &mut f64,
        var_eeff_rv_slot: &mut f64,
        var_lch_slot: &mut f64,
        var_lch_dn0_slot: &mut f64,
        var_lch_dn10_slot: &mut f64,
        var_lch_dn13_slot: &mut f64,
        var_lch_dn2_slot: &mut f64,
        var_lch_dn4_slot: &mut f64,
        var_lch_dn5_slot: &mut f64,
        var_lch_dn6_slot: &mut f64,
        var_lch_dn7_slot: &mut f64,
        var_lch_dn8_slot: &mut f64,
        var_lch_dn9_slot: &mut f64,
        var_lch_rv_slot: &mut f64,
        var_muun_slot: &mut f64,
        var_muun_dn0_slot: &mut f64,
        var_muun_dn10_slot: &mut f64,
        var_muun_dn13_slot: &mut f64,
        var_muun_dn2_slot: &mut f64,
        var_muun_dn4_slot: &mut f64,
        var_muun_dn5_slot: &mut f64,
        var_muun_dn6_slot: &mut f64,
        var_muun_dn7_slot: &mut f64,
        var_muun_dn8_slot: &mut f64,
        var_muun_dn9_slot: &mut f64,
        var_muun_rv_slot: &mut f64,
        var_rns_slot: &mut f64,
        var_rns_dn0_slot: &mut f64,
        var_rns_dn10_slot: &mut f64,
        var_rns_dn13_slot: &mut f64,
        var_rns_dn2_slot: &mut f64,
        var_rns_dn4_slot: &mut f64,
        var_rns_dn5_slot: &mut f64,
        var_rns_dn6_slot: &mut f64,
        var_rns_dn7_slot: &mut f64,
        var_rns_dn8_slot: &mut f64,
        var_rns_dn9_slot: &mut f64,
        var_rns_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn13_slot: &mut f64,
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
        var_t1_dn13_slot: &mut f64,
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
        var_t2_dn13_slot: &mut f64,
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
        var_t3_dn13_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn13_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_dn9_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn10_slot: &mut f64,
        var_t5_dn13_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t5_dn9_slot: &mut f64,
        var_t5_rv_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn13_slot: &mut f64,
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
        var_t7_dn13_slot: &mut f64,
        var_t7_dn2_slot: &mut f64,
        var_t7_dn4_slot: &mut f64,
        var_t7_dn5_slot: &mut f64,
        var_t7_dn6_slot: &mut f64,
        var_t7_dn7_slot: &mut f64,
        var_t7_dn8_slot: &mut f64,
        var_t7_dn9_slot: &mut f64,
        var_t7_rv_slot: &mut f64,
        var_t8_slot: &mut f64,
        var_t8_dn0_slot: &mut f64,
        var_t8_dn10_slot: &mut f64,
        var_t8_dn13_slot: &mut f64,
        var_t8_dn2_slot: &mut f64,
        var_t8_dn4_slot: &mut f64,
        var_t8_dn5_slot: &mut f64,
        var_t8_dn6_slot: &mut f64,
        var_t8_dn7_slot: &mut f64,
        var_t8_dn8_slot: &mut f64,
        var_t8_dn9_slot: &mut f64,
        var_t8_rv_slot: &mut f64,
        var_t9_slot: &mut f64,
        var_t9_dn0_slot: &mut f64,
        var_t9_dn10_slot: &mut f64,
        var_t9_dn13_slot: &mut f64,
        var_t9_dn2_slot: &mut f64,
        var_t9_dn4_slot: &mut f64,
        var_t9_dn5_slot: &mut f64,
        var_t9_dn6_slot: &mut f64,
        var_t9_dn7_slot: &mut f64,
        var_t9_dn8_slot: &mut f64,
        var_t9_dn9_slot: &mut f64,
        var_t9_rv_slot: &mut f64,
    ) {
        let mut var_eeff: f64 = *var_eeff_slot;
        let mut var_eeff_dn0: f64 = *var_eeff_dn0_slot;
        let mut var_eeff_dn10: f64 = *var_eeff_dn10_slot;
        let mut var_eeff_dn13: f64 = *var_eeff_dn13_slot;
        let mut var_eeff_dn2: f64 = *var_eeff_dn2_slot;
        let mut var_eeff_dn4: f64 = *var_eeff_dn4_slot;
        let mut var_eeff_dn5: f64 = *var_eeff_dn5_slot;
        let mut var_eeff_dn6: f64 = *var_eeff_dn6_slot;
        let mut var_eeff_dn7: f64 = *var_eeff_dn7_slot;
        let mut var_eeff_dn8: f64 = *var_eeff_dn8_slot;
        let mut var_eeff_dn9: f64 = *var_eeff_dn9_slot;
        let mut var_eeff_rv: f64 = *var_eeff_rv_slot;
        let mut var_lch: f64 = *var_lch_slot;
        let mut var_lch_dn0: f64 = *var_lch_dn0_slot;
        let mut var_lch_dn10: f64 = *var_lch_dn10_slot;
        let mut var_lch_dn13: f64 = *var_lch_dn13_slot;
        let mut var_lch_dn2: f64 = *var_lch_dn2_slot;
        let mut var_lch_dn4: f64 = *var_lch_dn4_slot;
        let mut var_lch_dn5: f64 = *var_lch_dn5_slot;
        let mut var_lch_dn6: f64 = *var_lch_dn6_slot;
        let mut var_lch_dn7: f64 = *var_lch_dn7_slot;
        let mut var_lch_dn8: f64 = *var_lch_dn8_slot;
        let mut var_lch_dn9: f64 = *var_lch_dn9_slot;
        let mut var_lch_rv: f64 = *var_lch_rv_slot;
        let mut var_muun: f64 = *var_muun_slot;
        let mut var_muun_dn0: f64 = *var_muun_dn0_slot;
        let mut var_muun_dn10: f64 = *var_muun_dn10_slot;
        let mut var_muun_dn13: f64 = *var_muun_dn13_slot;
        let mut var_muun_dn2: f64 = *var_muun_dn2_slot;
        let mut var_muun_dn4: f64 = *var_muun_dn4_slot;
        let mut var_muun_dn5: f64 = *var_muun_dn5_slot;
        let mut var_muun_dn6: f64 = *var_muun_dn6_slot;
        let mut var_muun_dn7: f64 = *var_muun_dn7_slot;
        let mut var_muun_dn8: f64 = *var_muun_dn8_slot;
        let mut var_muun_dn9: f64 = *var_muun_dn9_slot;
        let mut var_muun_rv: f64 = *var_muun_rv_slot;
        let mut var_rns: f64 = *var_rns_slot;
        let mut var_rns_dn0: f64 = *var_rns_dn0_slot;
        let mut var_rns_dn10: f64 = *var_rns_dn10_slot;
        let mut var_rns_dn13: f64 = *var_rns_dn13_slot;
        let mut var_rns_dn2: f64 = *var_rns_dn2_slot;
        let mut var_rns_dn4: f64 = *var_rns_dn4_slot;
        let mut var_rns_dn5: f64 = *var_rns_dn5_slot;
        let mut var_rns_dn6: f64 = *var_rns_dn6_slot;
        let mut var_rns_dn7: f64 = *var_rns_dn7_slot;
        let mut var_rns_dn8: f64 = *var_rns_dn8_slot;
        let mut var_rns_dn9: f64 = *var_rns_dn9_slot;
        let mut var_rns_rv: f64 = *var_rns_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn13: f64 = *var_t0_dn13_slot;
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
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
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
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
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
        let mut var_t3_dn13: f64 = *var_t3_dn13_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn13: f64 = *var_t4_dn13_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_dn9: f64 = *var_t4_dn9_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn10: f64 = *var_t5_dn10_slot;
        let mut var_t5_dn13: f64 = *var_t5_dn13_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t5_dn9: f64 = *var_t5_dn9_slot;
        let mut var_t5_rv: f64 = *var_t5_rv_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn13: f64 = *var_t6_dn13_slot;
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
        let mut var_t7_dn13: f64 = *var_t7_dn13_slot;
        let mut var_t7_dn2: f64 = *var_t7_dn2_slot;
        let mut var_t7_dn4: f64 = *var_t7_dn4_slot;
        let mut var_t7_dn5: f64 = *var_t7_dn5_slot;
        let mut var_t7_dn6: f64 = *var_t7_dn6_slot;
        let mut var_t7_dn7: f64 = *var_t7_dn7_slot;
        let mut var_t7_dn8: f64 = *var_t7_dn8_slot;
        let mut var_t7_dn9: f64 = *var_t7_dn9_slot;
        let mut var_t7_rv: f64 = *var_t7_rv_slot;
        let mut var_t8: f64 = *var_t8_slot;
        let mut var_t8_dn0: f64 = *var_t8_dn0_slot;
        let mut var_t8_dn10: f64 = *var_t8_dn10_slot;
        let mut var_t8_dn13: f64 = *var_t8_dn13_slot;
        let mut var_t8_dn2: f64 = *var_t8_dn2_slot;
        let mut var_t8_dn4: f64 = *var_t8_dn4_slot;
        let mut var_t8_dn5: f64 = *var_t8_dn5_slot;
        let mut var_t8_dn6: f64 = *var_t8_dn6_slot;
        let mut var_t8_dn7: f64 = *var_t8_dn7_slot;
        let mut var_t8_dn8: f64 = *var_t8_dn8_slot;
        let mut var_t8_dn9: f64 = *var_t8_dn9_slot;
        let mut var_t8_rv: f64 = *var_t8_rv_slot;
        let mut var_t9: f64 = *var_t9_slot;
        let mut var_t9_dn0: f64 = *var_t9_dn0_slot;
        let mut var_t9_dn10: f64 = *var_t9_dn10_slot;
        let mut var_t9_dn13: f64 = *var_t9_dn13_slot;
        let mut var_t9_dn2: f64 = *var_t9_dn2_slot;
        let mut var_t9_dn4: f64 = *var_t9_dn4_slot;
        let mut var_t9_dn5: f64 = *var_t9_dn5_slot;
        let mut var_t9_dn6: f64 = *var_t9_dn6_slot;
        let mut var_t9_dn7: f64 = *var_t9_dn7_slot;
        let mut var_t9_dn8: f64 = *var_t9_dn8_slot;
        let mut var_t9_dn9: f64 = *var_t9_dn9_slot;
        let mut var_t9_rv: f64 = *var_t9_rv_slot;

        let (assign61840_e96202, assign61840_e96202_d_n0, assign61840_e96202_d_n2, assign61840_e96202_d_n4, assign61840_e96202_d_n5, assign61840_e96202_d_n6, assign61840_e96202_d_n7, assign61840_e96202_d_n8, assign61840_e96202_d_n9, assign61840_e96202_d_n10, assign61840_e96202_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1490 != 0.0)) {
        (1e-9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_lch, var_lch_dn0, var_lch_dn2, var_lch_dn4, var_lch_dn5, var_lch_dn6, var_lch_dn7, var_lch_dn8, var_lch_dn9, var_lch_dn10, var_lch_dn13,)
    }
};
        var_lch = assign61840_e96202;
        var_lch_dn0 = assign61840_e96202_d_n0;
        var_lch_dn2 = assign61840_e96202_d_n2;
        var_lch_dn4 = assign61840_e96202_d_n4;
        var_lch_dn5 = assign61840_e96202_d_n5;
        var_lch_dn6 = assign61840_e96202_d_n6;
        var_lch_dn7 = assign61840_e96202_d_n7;
        var_lch_dn8 = assign61840_e96202_d_n8;
        var_lch_dn9 = assign61840_e96202_d_n9;
        var_lch_dn10 = assign61840_e96202_d_n10;
        var_lch_dn13 = assign61840_e96202_d_n13;
        var_lch_rv = 0.0;

        let (assign61850_e96211, assign61850_e96211_d_n0, assign61850_e96211_d_n2, assign61850_e96211_d_n4, assign61850_e96211_d_n5, assign61850_e96211_d_n6, assign61850_e96211_d_n7, assign61850_e96211_d_n8, assign61850_e96211_d_n9, assign61850_e96211_d_n10, assign61850_e96211_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign61850_e96209: f64 = (var_ndep_o_esi / 100.0);
        (assign61850_e96209, (var_ndep_o_esi_dn0 / 100.0), (var_ndep_o_esi_dn2 / 100.0), (var_ndep_o_esi_dn4 / 100.0), (var_ndep_o_esi_dn5 / 100.0), (var_ndep_o_esi_dn6 / 100.0), (var_ndep_o_esi_dn7 / 100.0), (var_ndep_o_esi_dn8 / 100.0), (var_ndep_o_esi_dn9 / 100.0), (var_ndep_o_esi_dn10 / 100.0), (var_ndep_o_esi_dn13 / 100.0),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign61850_e96211;
        var_t1_dn0 = assign61850_e96211_d_n0;
        var_t1_dn2 = assign61850_e96211_d_n2;
        var_t1_dn4 = assign61850_e96211_d_n4;
        var_t1_dn5 = assign61850_e96211_d_n5;
        var_t1_dn6 = assign61850_e96211_d_n6;
        var_t1_dn7 = assign61850_e96211_d_n7;
        var_t1_dn8 = assign61850_e96211_d_n8;
        var_t1_dn9 = assign61850_e96211_d_n9;
        var_t1_dn10 = assign61850_e96211_d_n10;
        var_t1_dn13 = assign61850_e96211_d_n13;
        var_t1_rv = 0.0;

        let (assign61860_e96220, assign61860_e96220_d_n0, assign61860_e96220_d_n2, assign61860_e96220_d_n4, assign61860_e96220_d_n5, assign61860_e96220_d_n6, assign61860_e96220_d_n7, assign61860_e96220_d_n8, assign61860_e96220_d_n9, assign61860_e96220_d_n10, assign61860_e96220_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign61860_e96218: f64 = (var_ninv_o_esi / 100.0);
        (assign61860_e96218, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign61860_e96220;
        var_t2_dn0 = assign61860_e96220_d_n0;
        var_t2_dn2 = assign61860_e96220_d_n2;
        var_t2_dn4 = assign61860_e96220_d_n4;
        var_t2_dn5 = assign61860_e96220_d_n5;
        var_t2_dn6 = assign61860_e96220_d_n6;
        var_t2_dn7 = assign61860_e96220_d_n7;
        var_t2_dn8 = assign61860_e96220_d_n8;
        var_t2_dn9 = assign61860_e96220_d_n9;
        var_t2_dn10 = assign61860_e96220_d_n10;
        var_t2_dn13 = assign61860_e96220_d_n13;
        var_t2_rv = 0.0;

        let (assign61870_e96227, assign61870_e96227_d_n0, assign61870_e96227_d_n2, assign61870_e96227_d_n4, assign61870_e96227_d_n5, assign61870_e96227_d_n6, assign61870_e96227_d_n7, assign61870_e96227_d_n8, assign61870_e96227_d_n9, assign61870_e96227_d_n10, assign61870_e96227_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        (var_ninvde, var_ninvde_dn0, var_ninvde_dn2, var_ninvde_dn4, var_ninvde_dn5, var_ninvde_dn6, var_ninvde_dn7, var_ninvde_dn8, var_ninvde_dn9, var_ninvde_dn10, var_ninvde_dn13,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign61870_e96227;
        var_t0_dn0 = assign61870_e96227_d_n0;
        var_t0_dn2 = assign61870_e96227_d_n2;
        var_t0_dn4 = assign61870_e96227_d_n4;
        var_t0_dn5 = assign61870_e96227_d_n5;
        var_t0_dn6 = assign61870_e96227_d_n6;
        var_t0_dn7 = assign61870_e96227_d_n7;
        var_t0_dn8 = assign61870_e96227_d_n8;
        var_t0_dn9 = assign61870_e96227_d_n9;
        var_t0_dn10 = assign61870_e96227_d_n10;
        var_t0_dn13 = assign61870_e96227_d_n13;
        var_t0_rv = 0.0;

        let (assign61880_e96240, assign61880_e96240_d_n0, assign61880_e96240_d_n2, assign61880_e96240_d_n4, assign61880_e96240_d_n5, assign61880_e96240_d_n6, assign61880_e96240_d_n7, assign61880_e96240_d_n8, assign61880_e96240_d_n9, assign61880_e96240_d_n10, assign61880_e96240_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign61880_e96235: f64 = (var_psl - var_ps0);
        let assign61880_e96237: f64 = (assign61880_e96235 * var_t0);
        let assign61880_e96238: f64 = (1.0 + assign61880_e96237);
        (assign61880_e96238, (((var_psl_dn0 - var_ps0_dn0) * var_t0) + (assign61880_e96235 * var_t0_dn0)), (((var_psl_dn2 - var_ps0_dn2) * var_t0) + (assign61880_e96235 * var_t0_dn2)), (((var_psl_dn4 - var_ps0_dn4) * var_t0) + (assign61880_e96235 * var_t0_dn4)), (((var_psl_dn5 - var_ps0_dn5) * var_t0) + (assign61880_e96235 * var_t0_dn5)), (((var_psl_dn6 - var_ps0_dn6) * var_t0) + (assign61880_e96235 * var_t0_dn6)), (((var_psl_dn7 - var_ps0_dn7) * var_t0) + (assign61880_e96235 * var_t0_dn7)), (((var_psl_dn8 - var_ps0_dn8) * var_t0) + (assign61880_e96235 * var_t0_dn8)), (((var_psl_dn9 - var_ps0_dn9) * var_t0) + (assign61880_e96235 * var_t0_dn9)), (((var_psl_dn10 - var_ps0_dn10) * var_t0) + (assign61880_e96235 * var_t0_dn10)), (((var_psl_dn13 - var_ps0_dn13) * var_t0) + (assign61880_e96235 * var_t0_dn13)),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn13,)
    }
};
        var_t4 = assign61880_e96240;
        var_t4_dn0 = assign61880_e96240_d_n0;
        var_t4_dn2 = assign61880_e96240_d_n2;
        var_t4_dn4 = assign61880_e96240_d_n4;
        var_t4_dn5 = assign61880_e96240_d_n5;
        var_t4_dn6 = assign61880_e96240_d_n6;
        var_t4_dn7 = assign61880_e96240_d_n7;
        var_t4_dn8 = assign61880_e96240_d_n8;
        var_t4_dn9 = assign61880_e96240_d_n9;
        var_t4_dn10 = assign61880_e96240_d_n10;
        var_t4_dn13 = assign61880_e96240_d_n13;
        var_t4_rv = 0.0;

        let (assign61890_e96253, assign61890_e96253_d_n0, assign61890_e96253_d_n2, assign61890_e96253_d_n4, assign61890_e96253_d_n5, assign61890_e96253_d_n6, assign61890_e96253_d_n7, assign61890_e96253_d_n8, assign61890_e96253_d_n9, assign61890_e96253_d_n10, assign61890_e96253_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign61890_e96247: f64 = (var_t1 * var_qbu);
        let assign61890_e96250: f64 = (var_t2 * var_qiu);
        let assign61890_e96251: f64 = (assign61890_e96247 + assign61890_e96250);
        (assign61890_e96251, (((var_t1_dn0 * var_qbu) + (var_t1 * var_qbu_dn0)) + ((var_t2_dn0 * var_qiu) + (var_t2 * var_qiu_dn0))), (((var_t1_dn2 * var_qbu) + (var_t1 * var_qbu_dn2)) + ((var_t2_dn2 * var_qiu) + (var_t2 * var_qiu_dn2))), (((var_t1_dn4 * var_qbu) + (var_t1 * var_qbu_dn4)) + ((var_t2_dn4 * var_qiu) + (var_t2 * var_qiu_dn4))), (((var_t1_dn5 * var_qbu) + (var_t1 * var_qbu_dn5)) + ((var_t2_dn5 * var_qiu) + (var_t2 * var_qiu_dn5))), (((var_t1_dn6 * var_qbu) + (var_t1 * var_qbu_dn6)) + ((var_t2_dn6 * var_qiu) + (var_t2 * var_qiu_dn6))), (((var_t1_dn7 * var_qbu) + (var_t1 * var_qbu_dn7)) + ((var_t2_dn7 * var_qiu) + (var_t2 * var_qiu_dn7))), (((var_t1_dn8 * var_qbu) + (var_t1 * var_qbu_dn8)) + ((var_t2_dn8 * var_qiu) + (var_t2 * var_qiu_dn8))), (((var_t1_dn9 * var_qbu) + (var_t1 * var_qbu_dn9)) + ((var_t2_dn9 * var_qiu) + (var_t2 * var_qiu_dn9))), (((var_t1_dn10 * var_qbu) + (var_t1 * var_qbu_dn10)) + ((var_t2_dn10 * var_qiu) + (var_t2 * var_qiu_dn10))), (((var_t1_dn13 * var_qbu) + (var_t1 * var_qbu_dn13)) + ((var_t2_dn13 * var_qiu) + (var_t2 * var_qiu_dn13))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn13,)
    }
};
        var_t5 = assign61890_e96253;
        var_t5_dn0 = assign61890_e96253_d_n0;
        var_t5_dn2 = assign61890_e96253_d_n2;
        var_t5_dn4 = assign61890_e96253_d_n4;
        var_t5_dn5 = assign61890_e96253_d_n5;
        var_t5_dn6 = assign61890_e96253_d_n6;
        var_t5_dn7 = assign61890_e96253_d_n7;
        var_t5_dn8 = assign61890_e96253_d_n8;
        var_t5_dn9 = assign61890_e96253_d_n9;
        var_t5_dn10 = assign61890_e96253_d_n10;
        var_t5_dn13 = assign61890_e96253_d_n13;
        var_t5_rv = 0.0;

        let (assign61900_e96262, assign61900_e96262_d_n0, assign61900_e96262_d_n2, assign61900_e96262_d_n4, assign61900_e96262_d_n5, assign61900_e96262_d_n6, assign61900_e96262_d_n7, assign61900_e96262_d_n8, assign61900_e96262_d_n9, assign61900_e96262_d_n10, assign61900_e96262_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign61900_e96260: f64 = (var_t5 / var_t4);
        (assign61900_e96260, (((var_t5_dn0 * var_t4) - (var_t5 * var_t4_dn0)) / (var_t4 * var_t4)), (((var_t5_dn2 * var_t4) - (var_t5 * var_t4_dn2)) / (var_t4 * var_t4)), (((var_t5_dn4 * var_t4) - (var_t5 * var_t4_dn4)) / (var_t4 * var_t4)), (((var_t5_dn5 * var_t4) - (var_t5 * var_t4_dn5)) / (var_t4 * var_t4)), (((var_t5_dn6 * var_t4) - (var_t5 * var_t4_dn6)) / (var_t4 * var_t4)), (((var_t5_dn7 * var_t4) - (var_t5 * var_t4_dn7)) / (var_t4 * var_t4)), (((var_t5_dn8 * var_t4) - (var_t5 * var_t4_dn8)) / (var_t4 * var_t4)), (((var_t5_dn9 * var_t4) - (var_t5 * var_t4_dn9)) / (var_t4 * var_t4)), (((var_t5_dn10 * var_t4) - (var_t5 * var_t4_dn10)) / (var_t4 * var_t4)), (((var_t5_dn13 * var_t4) - (var_t5 * var_t4_dn13)) / (var_t4 * var_t4)),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn13,)
    }
};
        var_t3 = assign61900_e96262;
        var_t3_dn0 = assign61900_e96262_d_n0;
        var_t3_dn2 = assign61900_e96262_d_n2;
        var_t3_dn4 = assign61900_e96262_d_n4;
        var_t3_dn5 = assign61900_e96262_d_n5;
        var_t3_dn6 = assign61900_e96262_d_n6;
        var_t3_dn7 = assign61900_e96262_d_n7;
        var_t3_dn8 = assign61900_e96262_d_n8;
        var_t3_dn9 = assign61900_e96262_d_n9;
        var_t3_dn10 = assign61900_e96262_d_n10;
        var_t3_dn13 = assign61900_e96262_d_n13;
        var_t3_rv = 0.0;

        let (assign61910_e96275, assign61910_e96275_d_n0, assign61910_e96275_d_n2, assign61910_e96275_d_n4, assign61910_e96275_d_n5, assign61910_e96275_d_n6, assign61910_e96275_d_n7, assign61910_e96275_d_n8, assign61910_e96275_d_n9, assign61910_e96275_d_n10, assign61910_e96275_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign61910_e96271: f64 = (p.p166 * var_vbsz__blk438);
        let assign61910_e96272: f64 = (1.0 + assign61910_e96271);
        let assign61910_e96273: f64 = (var_t3 * assign61910_e96272);
        (assign61910_e96273, ((var_t3_dn0 * assign61910_e96272) + (var_t3 * (p.p166 * var_vbsz__blk438_dn0))), ((var_t3_dn2 * assign61910_e96272) + (var_t3 * (p.p166 * var_vbsz__blk438_dn2))), ((var_t3_dn4 * assign61910_e96272) + (var_t3 * (p.p166 * var_vbsz__blk438_dn4))), ((var_t3_dn5 * assign61910_e96272) + (var_t3 * (p.p166 * var_vbsz__blk438_dn5))), ((var_t3_dn6 * assign61910_e96272) + (var_t3 * (p.p166 * var_vbsz__blk438_dn6))), ((var_t3_dn7 * assign61910_e96272) + (var_t3 * (p.p166 * var_vbsz__blk438_dn7))), ((var_t3_dn8 * assign61910_e96272) + (var_t3 * (p.p166 * var_vbsz__blk438_dn8))), ((var_t3_dn9 * assign61910_e96272) + (var_t3 * (p.p166 * var_vbsz__blk438_dn9))), ((var_t3_dn10 * assign61910_e96272) + (var_t3 * (p.p166 * var_vbsz__blk438_dn10))), ((var_t3_dn13 * assign61910_e96272) + (var_t3 * (p.p166 * var_vbsz__blk438_dn13))),)
    } else {
        (var_eeff, var_eeff_dn0, var_eeff_dn2, var_eeff_dn4, var_eeff_dn5, var_eeff_dn6, var_eeff_dn7, var_eeff_dn8, var_eeff_dn9, var_eeff_dn10, var_eeff_dn13,)
    }
};
        var_eeff = assign61910_e96275;
        var_eeff_dn0 = assign61910_e96275_d_n0;
        var_eeff_dn2 = assign61910_e96275_d_n2;
        var_eeff_dn4 = assign61910_e96275_d_n4;
        var_eeff_dn5 = assign61910_e96275_d_n5;
        var_eeff_dn6 = assign61910_e96275_d_n6;
        var_eeff_dn7 = assign61910_e96275_d_n7;
        var_eeff_dn8 = assign61910_e96275_d_n8;
        var_eeff_dn9 = assign61910_e96275_d_n9;
        var_eeff_dn10 = assign61910_e96275_d_n10;
        var_eeff_dn13 = assign61910_e96275_d_n13;
        var_eeff_rv = 0.0;

        let (assign61920_e96291, assign61920_e96291_d_n0, assign61920_e96291_d_n2, assign61920_e96291_d_n4, assign61920_e96291_d_n5, assign61920_e96291_d_n6, assign61920_e96291_d_n7, assign61920_e96291_d_n8, assign61920_e96291_d_n9, assign61920_e96291_d_n10, assign61920_e96291_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let (assign61920_e96289, assign61920_e96289_d_n0, assign61920_e96289_d_n2, assign61920_e96289_d_n4, assign61920_e96289_d_n5, assign61920_e96289_d_n6, assign61920_e96289_d_n7, assign61920_e96289_d_n8, assign61920_e96289_d_n9, assign61920_e96289_d_n10, assign61920_e96289_d_n13,) = {
            if (var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign61920_e96287: f64 = (p.p160 - 1.0);
                let assign61920_e96288: f64 = (var_eeff).powf(assign61920_e96287);
                (assign61920_e96288, if 0.0 == 0.0 && ((assign61920_e96287) as f64).is_finite() && ((assign61920_e96287) as f64).fract() == 0.0 { if assign61920_e96287 == 0.0 { 0.0 } else { (assign61920_e96287 * ((var_eeff).powf(assign61920_e96287 - 1.0) * var_eeff_dn0)) } } else { (assign61920_e96288 * (assign61920_e96287 * (var_eeff_dn0 / var_eeff))) }, if 0.0 == 0.0 && ((assign61920_e96287) as f64).is_finite() && ((assign61920_e96287) as f64).fract() == 0.0 { if assign61920_e96287 == 0.0 { 0.0 } else { (assign61920_e96287 * ((var_eeff).powf(assign61920_e96287 - 1.0) * var_eeff_dn2)) } } else { (assign61920_e96288 * (assign61920_e96287 * (var_eeff_dn2 / var_eeff))) }, if 0.0 == 0.0 && ((assign61920_e96287) as f64).is_finite() && ((assign61920_e96287) as f64).fract() == 0.0 { if assign61920_e96287 == 0.0 { 0.0 } else { (assign61920_e96287 * ((var_eeff).powf(assign61920_e96287 - 1.0) * var_eeff_dn4)) } } else { (assign61920_e96288 * (assign61920_e96287 * (var_eeff_dn4 / var_eeff))) }, if 0.0 == 0.0 && ((assign61920_e96287) as f64).is_finite() && ((assign61920_e96287) as f64).fract() == 0.0 { if assign61920_e96287 == 0.0 { 0.0 } else { (assign61920_e96287 * ((var_eeff).powf(assign61920_e96287 - 1.0) * var_eeff_dn5)) } } else { (assign61920_e96288 * (assign61920_e96287 * (var_eeff_dn5 / var_eeff))) }, if 0.0 == 0.0 && ((assign61920_e96287) as f64).is_finite() && ((assign61920_e96287) as f64).fract() == 0.0 { if assign61920_e96287 == 0.0 { 0.0 } else { (assign61920_e96287 * ((var_eeff).powf(assign61920_e96287 - 1.0) * var_eeff_dn6)) } } else { (assign61920_e96288 * (assign61920_e96287 * (var_eeff_dn6 / var_eeff))) }, if 0.0 == 0.0 && ((assign61920_e96287) as f64).is_finite() && ((assign61920_e96287) as f64).fract() == 0.0 { if assign61920_e96287 == 0.0 { 0.0 } else { (assign61920_e96287 * ((var_eeff).powf(assign61920_e96287 - 1.0) * var_eeff_dn7)) } } else { (assign61920_e96288 * (assign61920_e96287 * (var_eeff_dn7 / var_eeff))) }, if 0.0 == 0.0 && ((assign61920_e96287) as f64).is_finite() && ((assign61920_e96287) as f64).fract() == 0.0 { if assign61920_e96287 == 0.0 { 0.0 } else { (assign61920_e96287 * ((var_eeff).powf(assign61920_e96287 - 1.0) * var_eeff_dn8)) } } else { (assign61920_e96288 * (assign61920_e96287 * (var_eeff_dn8 / var_eeff))) }, if 0.0 == 0.0 && ((assign61920_e96287) as f64).is_finite() && ((assign61920_e96287) as f64).fract() == 0.0 { if assign61920_e96287 == 0.0 { 0.0 } else { (assign61920_e96287 * ((var_eeff).powf(assign61920_e96287 - 1.0) * var_eeff_dn9)) } } else { (assign61920_e96288 * (assign61920_e96287 * (var_eeff_dn9 / var_eeff))) }, if 0.0 == 0.0 && ((assign61920_e96287) as f64).is_finite() && ((assign61920_e96287) as f64).fract() == 0.0 { if assign61920_e96287 == 0.0 { 0.0 } else { (assign61920_e96287 * ((var_eeff).powf(assign61920_e96287 - 1.0) * var_eeff_dn10)) } } else { (assign61920_e96288 * (assign61920_e96287 * (var_eeff_dn10 / var_eeff))) }, if 0.0 == 0.0 && ((assign61920_e96287) as f64).is_finite() && ((assign61920_e96287) as f64).fract() == 0.0 { if assign61920_e96287 == 0.0 { 0.0 } else { (assign61920_e96287 * ((var_eeff).powf(assign61920_e96287 - 1.0) * var_eeff_dn13)) } } else { (assign61920_e96288 * (assign61920_e96287 * (var_eeff_dn13 / var_eeff))) },)
            }
        };
        (assign61920_e96289, assign61920_e96289_d_n0, assign61920_e96289_d_n2, assign61920_e96289_d_n4, assign61920_e96289_d_n5, assign61920_e96289_d_n6, assign61920_e96289_d_n7, assign61920_e96289_d_n8, assign61920_e96289_d_n9, assign61920_e96289_d_n10, assign61920_e96289_d_n13,)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn13,)
    }
};
        var_t5 = assign61920_e96291;
        var_t5_dn0 = assign61920_e96291_d_n0;
        var_t5_dn2 = assign61920_e96291_d_n2;
        var_t5_dn4 = assign61920_e96291_d_n4;
        var_t5_dn5 = assign61920_e96291_d_n5;
        var_t5_dn6 = assign61920_e96291_d_n6;
        var_t5_dn7 = assign61920_e96291_d_n7;
        var_t5_dn8 = assign61920_e96291_d_n8;
        var_t5_dn9 = assign61920_e96291_d_n9;
        var_t5_dn10 = assign61920_e96291_d_n10;
        var_t5_dn13 = assign61920_e96291_d_n13;
        var_t5_rv = 0.0;

        let (assign61930_e96300, assign61930_e96300_d_n0, assign61930_e96300_d_n2, assign61930_e96300_d_n4, assign61930_e96300_d_n5, assign61930_e96300_d_n6, assign61930_e96300_d_n7, assign61930_e96300_d_n8, assign61930_e96300_d_n9, assign61930_e96300_d_n10, assign61930_e96300_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign61930_e96298: f64 = (var_t5 * var_eeff);
        (assign61930_e96298, ((var_t5_dn0 * var_eeff) + (var_t5 * var_eeff_dn0)), ((var_t5_dn2 * var_eeff) + (var_t5 * var_eeff_dn2)), ((var_t5_dn4 * var_eeff) + (var_t5 * var_eeff_dn4)), ((var_t5_dn5 * var_eeff) + (var_t5 * var_eeff_dn5)), ((var_t5_dn6 * var_eeff) + (var_t5 * var_eeff_dn6)), ((var_t5_dn7 * var_eeff) + (var_t5 * var_eeff_dn7)), ((var_t5_dn8 * var_eeff) + (var_t5 * var_eeff_dn8)), ((var_t5_dn9 * var_eeff) + (var_t5 * var_eeff_dn9)), ((var_t5_dn10 * var_eeff) + (var_t5 * var_eeff_dn10)), ((var_t5_dn13 * var_eeff) + (var_t5 * var_eeff_dn13)),)
    } else {
        (var_t8, var_t8_dn0, var_t8_dn2, var_t8_dn4, var_t8_dn5, var_t8_dn6, var_t8_dn7, var_t8_dn8, var_t8_dn9, var_t8_dn10, var_t8_dn13,)
    }
};
        var_t8 = assign61930_e96300;
        var_t8_dn0 = assign61930_e96300_d_n0;
        var_t8_dn2 = assign61930_e96300_d_n2;
        var_t8_dn4 = assign61930_e96300_d_n4;
        var_t8_dn5 = assign61930_e96300_d_n5;
        var_t8_dn6 = assign61930_e96300_d_n6;
        var_t8_dn7 = assign61930_e96300_d_n7;
        var_t8_dn8 = assign61930_e96300_d_n8;
        var_t8_dn9 = assign61930_e96300_d_n9;
        var_t8_dn10 = assign61930_e96300_d_n10;
        var_t8_dn13 = assign61930_e96300_d_n13;
        var_t8_rv = 0.0;

        let (assign61940_e96316, assign61940_e96316_d_n0, assign61940_e96316_d_n2, assign61940_e96316_d_n4, assign61940_e96316_d_n5, assign61940_e96316_d_n6, assign61940_e96316_d_n7, assign61940_e96316_d_n8, assign61940_e96316_d_n9, assign61940_e96316_d_n10, assign61940_e96316_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let (assign61940_e96314, assign61940_e96314_d_n0, assign61940_e96314_d_n2, assign61940_e96314_d_n4, assign61940_e96314_d_n5, assign61940_e96314_d_n6, assign61940_e96314_d_n7, assign61940_e96314_d_n8, assign61940_e96314_d_n9, assign61940_e96314_d_n10, assign61940_e96314_d_n13,) = {
            if (var_eeff == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign61940_e96312: f64 = (var_muesr - 1.0);
                let assign61940_e96313: f64 = (var_eeff).powf(assign61940_e96312);
                (assign61940_e96313, if 0.0 == 0.0 && ((assign61940_e96312) as f64).is_finite() && ((assign61940_e96312) as f64).fract() == 0.0 { if assign61940_e96312 == 0.0 { 0.0 } else { (assign61940_e96312 * ((var_eeff).powf(assign61940_e96312 - 1.0) * var_eeff_dn0)) } } else { (assign61940_e96313 * (assign61940_e96312 * (var_eeff_dn0 / var_eeff))) }, if 0.0 == 0.0 && ((assign61940_e96312) as f64).is_finite() && ((assign61940_e96312) as f64).fract() == 0.0 { if assign61940_e96312 == 0.0 { 0.0 } else { (assign61940_e96312 * ((var_eeff).powf(assign61940_e96312 - 1.0) * var_eeff_dn2)) } } else { (assign61940_e96313 * (assign61940_e96312 * (var_eeff_dn2 / var_eeff))) }, if 0.0 == 0.0 && ((assign61940_e96312) as f64).is_finite() && ((assign61940_e96312) as f64).fract() == 0.0 { if assign61940_e96312 == 0.0 { 0.0 } else { (assign61940_e96312 * ((var_eeff).powf(assign61940_e96312 - 1.0) * var_eeff_dn4)) } } else { (assign61940_e96313 * (assign61940_e96312 * (var_eeff_dn4 / var_eeff))) }, if 0.0 == 0.0 && ((assign61940_e96312) as f64).is_finite() && ((assign61940_e96312) as f64).fract() == 0.0 { if assign61940_e96312 == 0.0 { 0.0 } else { (assign61940_e96312 * ((var_eeff).powf(assign61940_e96312 - 1.0) * var_eeff_dn5)) } } else { (assign61940_e96313 * (assign61940_e96312 * (var_eeff_dn5 / var_eeff))) }, if 0.0 == 0.0 && ((assign61940_e96312) as f64).is_finite() && ((assign61940_e96312) as f64).fract() == 0.0 { if assign61940_e96312 == 0.0 { 0.0 } else { (assign61940_e96312 * ((var_eeff).powf(assign61940_e96312 - 1.0) * var_eeff_dn6)) } } else { (assign61940_e96313 * (assign61940_e96312 * (var_eeff_dn6 / var_eeff))) }, if 0.0 == 0.0 && ((assign61940_e96312) as f64).is_finite() && ((assign61940_e96312) as f64).fract() == 0.0 { if assign61940_e96312 == 0.0 { 0.0 } else { (assign61940_e96312 * ((var_eeff).powf(assign61940_e96312 - 1.0) * var_eeff_dn7)) } } else { (assign61940_e96313 * (assign61940_e96312 * (var_eeff_dn7 / var_eeff))) }, if 0.0 == 0.0 && ((assign61940_e96312) as f64).is_finite() && ((assign61940_e96312) as f64).fract() == 0.0 { if assign61940_e96312 == 0.0 { 0.0 } else { (assign61940_e96312 * ((var_eeff).powf(assign61940_e96312 - 1.0) * var_eeff_dn8)) } } else { (assign61940_e96313 * (assign61940_e96312 * (var_eeff_dn8 / var_eeff))) }, if 0.0 == 0.0 && ((assign61940_e96312) as f64).is_finite() && ((assign61940_e96312) as f64).fract() == 0.0 { if assign61940_e96312 == 0.0 { 0.0 } else { (assign61940_e96312 * ((var_eeff).powf(assign61940_e96312 - 1.0) * var_eeff_dn9)) } } else { (assign61940_e96313 * (assign61940_e96312 * (var_eeff_dn9 / var_eeff))) }, if 0.0 == 0.0 && ((assign61940_e96312) as f64).is_finite() && ((assign61940_e96312) as f64).fract() == 0.0 { if assign61940_e96312 == 0.0 { 0.0 } else { (assign61940_e96312 * ((var_eeff).powf(assign61940_e96312 - 1.0) * var_eeff_dn10)) } } else { (assign61940_e96313 * (assign61940_e96312 * (var_eeff_dn10 / var_eeff))) }, if 0.0 == 0.0 && ((assign61940_e96312) as f64).is_finite() && ((assign61940_e96312) as f64).fract() == 0.0 { if assign61940_e96312 == 0.0 { 0.0 } else { (assign61940_e96312 * ((var_eeff).powf(assign61940_e96312 - 1.0) * var_eeff_dn13)) } } else { (assign61940_e96313 * (assign61940_e96312 * (var_eeff_dn13 / var_eeff))) },)
            }
        };
        (assign61940_e96314, assign61940_e96314_d_n0, assign61940_e96314_d_n2, assign61940_e96314_d_n4, assign61940_e96314_d_n5, assign61940_e96314_d_n6, assign61940_e96314_d_n7, assign61940_e96314_d_n8, assign61940_e96314_d_n9, assign61940_e96314_d_n10, assign61940_e96314_d_n13,)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn7, var_t7_dn8, var_t7_dn9, var_t7_dn10, var_t7_dn13,)
    }
};
        var_t7 = assign61940_e96316;
        var_t7_dn0 = assign61940_e96316_d_n0;
        var_t7_dn2 = assign61940_e96316_d_n2;
        var_t7_dn4 = assign61940_e96316_d_n4;
        var_t7_dn5 = assign61940_e96316_d_n5;
        var_t7_dn6 = assign61940_e96316_d_n6;
        var_t7_dn7 = assign61940_e96316_d_n7;
        var_t7_dn8 = assign61940_e96316_d_n8;
        var_t7_dn9 = assign61940_e96316_d_n9;
        var_t7_dn10 = assign61940_e96316_d_n10;
        var_t7_dn13 = assign61940_e96316_d_n13;
        var_t7_rv = 0.0;

        let (assign61950_e96325, assign61950_e96325_d_n0, assign61950_e96325_d_n2, assign61950_e96325_d_n4, assign61950_e96325_d_n5, assign61950_e96325_d_n6, assign61950_e96325_d_n7, assign61950_e96325_d_n8, assign61950_e96325_d_n9, assign61950_e96325_d_n10, assign61950_e96325_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign61950_e96323: f64 = (var_t7 * var_eeff);
        (assign61950_e96323, ((var_t7_dn0 * var_eeff) + (var_t7 * var_eeff_dn0)), ((var_t7_dn2 * var_eeff) + (var_t7 * var_eeff_dn2)), ((var_t7_dn4 * var_eeff) + (var_t7 * var_eeff_dn4)), ((var_t7_dn5 * var_eeff) + (var_t7 * var_eeff_dn5)), ((var_t7_dn6 * var_eeff) + (var_t7 * var_eeff_dn6)), ((var_t7_dn7 * var_eeff) + (var_t7 * var_eeff_dn7)), ((var_t7_dn8 * var_eeff) + (var_t7 * var_eeff_dn8)), ((var_t7_dn9 * var_eeff) + (var_t7 * var_eeff_dn9)), ((var_t7_dn10 * var_eeff) + (var_t7 * var_eeff_dn10)), ((var_t7_dn13 * var_eeff) + (var_t7 * var_eeff_dn13)),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn13,)
    }
};
        var_t6 = assign61950_e96325;
        var_t6_dn0 = assign61950_e96325_d_n0;
        var_t6_dn2 = assign61950_e96325_d_n2;
        var_t6_dn4 = assign61950_e96325_d_n4;
        var_t6_dn5 = assign61950_e96325_d_n5;
        var_t6_dn6 = assign61950_e96325_d_n6;
        var_t6_dn7 = assign61950_e96325_d_n7;
        var_t6_dn8 = assign61950_e96325_d_n8;
        var_t6_dn9 = assign61950_e96325_d_n9;
        var_t6_dn10 = assign61950_e96325_d_n10;
        var_t6_dn13 = assign61950_e96325_d_n13;
        var_t6_rv = 0.0;

        let (assign61960_e96334, assign61960_e96334_d_n0, assign61960_e96334_d_n2, assign61960_e96334_d_n4, assign61960_e96334_d_n5, assign61960_e96334_d_n6, assign61960_e96334_d_n7, assign61960_e96334_d_n8, assign61960_e96334_d_n9, assign61960_e96334_d_n10, assign61960_e96334_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign61960_e96332: f64 = (1.6021918e-19 * 10000.0);
        (assign61960_e96332, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t9, var_t9_dn0, var_t9_dn2, var_t9_dn4, var_t9_dn5, var_t9_dn6, var_t9_dn7, var_t9_dn8, var_t9_dn9, var_t9_dn10, var_t9_dn13,)
    }
};
        var_t9 = assign61960_e96334;
        var_t9_dn0 = assign61960_e96334_d_n0;
        var_t9_dn2 = assign61960_e96334_d_n2;
        var_t9_dn4 = assign61960_e96334_d_n4;
        var_t9_dn5 = assign61960_e96334_d_n5;
        var_t9_dn6 = assign61960_e96334_d_n6;
        var_t9_dn7 = assign61960_e96334_d_n7;
        var_t9_dn8 = assign61960_e96334_d_n8;
        var_t9_dn9 = assign61960_e96334_d_n9;
        var_t9_dn10 = assign61960_e96334_d_n10;
        var_t9_dn13 = assign61960_e96334_d_n13;
        var_t9_rv = 0.0;

        let (assign61970_e96343, assign61970_e96343_d_n0, assign61970_e96343_d_n2, assign61970_e96343_d_n4, assign61970_e96343_d_n5, assign61970_e96343_d_n6, assign61970_e96343_d_n7, assign61970_e96343_d_n8, assign61970_e96343_d_n9, assign61970_e96343_d_n10, assign61970_e96343_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign61970_e96341: f64 = (var_qiu / var_t9);
        (assign61970_e96341, (((var_qiu_dn0 * var_t9) - (var_qiu * var_t9_dn0)) / (var_t9 * var_t9)), (((var_qiu_dn2 * var_t9) - (var_qiu * var_t9_dn2)) / (var_t9 * var_t9)), (((var_qiu_dn4 * var_t9) - (var_qiu * var_t9_dn4)) / (var_t9 * var_t9)), (((var_qiu_dn5 * var_t9) - (var_qiu * var_t9_dn5)) / (var_t9 * var_t9)), (((var_qiu_dn6 * var_t9) - (var_qiu * var_t9_dn6)) / (var_t9 * var_t9)), (((var_qiu_dn7 * var_t9) - (var_qiu * var_t9_dn7)) / (var_t9 * var_t9)), (((var_qiu_dn8 * var_t9) - (var_qiu * var_t9_dn8)) / (var_t9 * var_t9)), (((var_qiu_dn9 * var_t9) - (var_qiu * var_t9_dn9)) / (var_t9 * var_t9)), (((var_qiu_dn10 * var_t9) - (var_qiu * var_t9_dn10)) / (var_t9 * var_t9)), (((var_qiu_dn13 * var_t9) - (var_qiu * var_t9_dn13)) / (var_t9 * var_t9)),)
    } else {
        (var_rns, var_rns_dn0, var_rns_dn2, var_rns_dn4, var_rns_dn5, var_rns_dn6, var_rns_dn7, var_rns_dn8, var_rns_dn9, var_rns_dn10, var_rns_dn13,)
    }
};
        var_rns = assign61970_e96343;
        var_rns_dn0 = assign61970_e96343_d_n0;
        var_rns_dn2 = assign61970_e96343_d_n2;
        var_rns_dn4 = assign61970_e96343_d_n4;
        var_rns_dn5 = assign61970_e96343_d_n5;
        var_rns_dn6 = assign61970_e96343_d_n6;
        var_rns_dn7 = assign61970_e96343_d_n7;
        var_rns_dn8 = assign61970_e96343_d_n8;
        var_rns_dn9 = assign61970_e96343_d_n9;
        var_rns_dn10 = assign61970_e96343_d_n10;
        var_rns_dn13 = assign61970_e96343_d_n13;
        var_rns_rv = 0.0;

        let (assign61980_e96366, assign61980_e96366_d_n0, assign61980_e96366_d_n2, assign61980_e96366_d_n4, assign61980_e96366_d_n5, assign61980_e96366_d_n6, assign61980_e96366_d_n7, assign61980_e96366_d_n8, assign61980_e96366_d_n9, assign61980_e96366_d_n10, assign61980_e96366_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign61980_e96352: f64 = (var_uc_muecb1 * var_rns);
        let assign61980_e96354: f64 = (assign61980_e96352 / 100000000000.0);
        let assign61980_e96355: f64 = (var_uc_muecb0 + assign61980_e96354);
        let assign61980_e96356: f64 = (1.0 / assign61980_e96355);
        let assign61980_e96359: f64 = (var_mphn0 * var_t8);
        let assign61980_e96360: f64 = (assign61980_e96356 + assign61980_e96359);
        let assign61980_e96363: f64 = (var_t6 / var_uc_muesr1);
        let assign61980_e96364: f64 = (assign61980_e96360 + assign61980_e96363);
        (assign61980_e96364, (((-(((var_uc_muecb1 * var_rns_dn0) / 100000000000.0) / (assign61980_e96355 * assign61980_e96355))) + ((var_mphn0_dn0 * var_t8) + (var_mphn0 * var_t8_dn0))) + (var_t6_dn0 / var_uc_muesr1)), (((-(((var_uc_muecb1 * var_rns_dn2) / 100000000000.0) / (assign61980_e96355 * assign61980_e96355))) + ((var_mphn0_dn2 * var_t8) + (var_mphn0 * var_t8_dn2))) + (var_t6_dn2 / var_uc_muesr1)), (((-(((var_uc_muecb1 * var_rns_dn4) / 100000000000.0) / (assign61980_e96355 * assign61980_e96355))) + ((var_mphn0_dn4 * var_t8) + (var_mphn0 * var_t8_dn4))) + (var_t6_dn4 / var_uc_muesr1)), (((-(((var_uc_muecb1 * var_rns_dn5) / 100000000000.0) / (assign61980_e96355 * assign61980_e96355))) + ((var_mphn0_dn5 * var_t8) + (var_mphn0 * var_t8_dn5))) + (var_t6_dn5 / var_uc_muesr1)), (((-(((var_uc_muecb1 * var_rns_dn6) / 100000000000.0) / (assign61980_e96355 * assign61980_e96355))) + ((var_mphn0_dn6 * var_t8) + (var_mphn0 * var_t8_dn6))) + (var_t6_dn6 / var_uc_muesr1)), (((-(((var_uc_muecb1 * var_rns_dn7) / 100000000000.0) / (assign61980_e96355 * assign61980_e96355))) + ((var_mphn0_dn7 * var_t8) + (var_mphn0 * var_t8_dn7))) + (var_t6_dn7 / var_uc_muesr1)), (((-(((var_uc_muecb1 * var_rns_dn8) / 100000000000.0) / (assign61980_e96355 * assign61980_e96355))) + ((var_mphn0_dn8 * var_t8) + (var_mphn0 * var_t8_dn8))) + (var_t6_dn8 / var_uc_muesr1)), (((-(((var_uc_muecb1 * var_rns_dn9) / 100000000000.0) / (assign61980_e96355 * assign61980_e96355))) + ((var_mphn0_dn9 * var_t8) + (var_mphn0 * var_t8_dn9))) + (var_t6_dn9 / var_uc_muesr1)), (((-(((var_uc_muecb1 * var_rns_dn10) / 100000000000.0) / (assign61980_e96355 * assign61980_e96355))) + ((var_mphn0_dn10 * var_t8) + (var_mphn0 * var_t8_dn10))) + (var_t6_dn10 / var_uc_muesr1)), (((-(((var_uc_muecb1 * var_rns_dn13) / 100000000000.0) / (assign61980_e96355 * assign61980_e96355))) + ((var_mphn0_dn13 * var_t8) + (var_mphn0 * var_t8_dn13))) + (var_t6_dn13 / var_uc_muesr1)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign61980_e96366;
        var_t1_dn0 = assign61980_e96366_d_n0;
        var_t1_dn2 = assign61980_e96366_d_n2;
        var_t1_dn4 = assign61980_e96366_d_n4;
        var_t1_dn5 = assign61980_e96366_d_n5;
        var_t1_dn6 = assign61980_e96366_d_n6;
        var_t1_dn7 = assign61980_e96366_d_n7;
        var_t1_dn8 = assign61980_e96366_d_n8;
        var_t1_dn9 = assign61980_e96366_d_n9;
        var_t1_dn10 = assign61980_e96366_d_n10;
        var_t1_dn13 = assign61980_e96366_d_n13;
        var_t1_rv = 0.0;

        let (assign61990_e96375, assign61990_e96375_d_n0, assign61990_e96375_d_n2, assign61990_e96375_d_n4, assign61990_e96375_d_n5, assign61990_e96375_d_n6, assign61990_e96375_d_n7, assign61990_e96375_d_n8, assign61990_e96375_d_n9, assign61990_e96375_d_n10, assign61990_e96375_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign61990_e96373: f64 = (1.0 / var_t1);
        (assign61990_e96373, (-(var_t1_dn0 / (var_t1 * var_t1))), (-(var_t1_dn2 / (var_t1 * var_t1))), (-(var_t1_dn4 / (var_t1 * var_t1))), (-(var_t1_dn5 / (var_t1 * var_t1))), (-(var_t1_dn6 / (var_t1 * var_t1))), (-(var_t1_dn7 / (var_t1 * var_t1))), (-(var_t1_dn8 / (var_t1 * var_t1))), (-(var_t1_dn9 / (var_t1 * var_t1))), (-(var_t1_dn10 / (var_t1 * var_t1))), (-(var_t1_dn13 / (var_t1 * var_t1))),)
    } else {
        (var_muun, var_muun_dn0, var_muun_dn2, var_muun_dn4, var_muun_dn5, var_muun_dn6, var_muun_dn7, var_muun_dn8, var_muun_dn9, var_muun_dn10, var_muun_dn13,)
    }
};
        var_muun = assign61990_e96375;
        var_muun_dn0 = assign61990_e96375_d_n0;
        var_muun_dn2 = assign61990_e96375_d_n2;
        var_muun_dn4 = assign61990_e96375_d_n4;
        var_muun_dn5 = assign61990_e96375_d_n5;
        var_muun_dn6 = assign61990_e96375_d_n6;
        var_muun_dn7 = assign61990_e96375_d_n7;
        var_muun_dn8 = assign61990_e96375_d_n8;
        var_muun_dn9 = assign61990_e96375_d_n9;
        var_muun_dn10 = assign61990_e96375_d_n10;
        var_muun_dn13 = assign61990_e96375_d_n13;
        var_muun_rv = 0.0;

        let (assign62000_e96384, assign62000_e96384_d_n0, assign62000_e96384_d_n2, assign62000_e96384_d_n4, assign62000_e96384_d_n5, assign62000_e96384_d_n6, assign62000_e96384_d_n7, assign62000_e96384_d_n8, assign62000_e96384_d_n9, assign62000_e96384_d_n10, assign62000_e96384_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign62000_e96382: f64 = (var_muun / 10000.0);
        (assign62000_e96382, (var_muun_dn0 / 10000.0), (var_muun_dn2 / 10000.0), (var_muun_dn4 / 10000.0), (var_muun_dn5 / 10000.0), (var_muun_dn6 / 10000.0), (var_muun_dn7 / 10000.0), (var_muun_dn8 / 10000.0), (var_muun_dn9 / 10000.0), (var_muun_dn10 / 10000.0), (var_muun_dn13 / 10000.0),)
    } else {
        (var_muun, var_muun_dn0, var_muun_dn2, var_muun_dn4, var_muun_dn5, var_muun_dn6, var_muun_dn7, var_muun_dn8, var_muun_dn9, var_muun_dn10, var_muun_dn13,)
    }
};
        var_muun = assign62000_e96384;
        var_muun_dn0 = assign62000_e96384_d_n0;
        var_muun_dn2 = assign62000_e96384_d_n2;
        var_muun_dn4 = assign62000_e96384_d_n4;
        var_muun_dn5 = assign62000_e96384_d_n5;
        var_muun_dn6 = assign62000_e96384_d_n6;
        var_muun_dn7 = assign62000_e96384_d_n7;
        var_muun_dn8 = assign62000_e96384_d_n8;
        var_muun_dn9 = assign62000_e96384_d_n9;
        var_muun_dn10 = assign62000_e96384_d_n10;
        var_muun_dn13 = assign62000_e96384_d_n13;
        var_muun_rv = 0.0;

        let (assign62010_e96397, assign62010_e96397_d_n0, assign62010_e96397_d_n2, assign62010_e96397_d_n4, assign62010_e96397_d_n5, assign62010_e96397_d_n6, assign62010_e96397_d_n7, assign62010_e96397_d_n8, assign62010_e96397_d_n9, assign62010_e96397_d_n10, assign62010_e96397_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign62010_e96392: f64 = (var_qn0 + 1e-25);
        let assign62010_e96393: f64 = (var_beta * assign62010_e96392);
        let assign62010_e96395: f64 = (assign62010_e96393 * var_lch);
        (assign62010_e96395, ((((var_beta_dn0 * assign62010_e96392) + (var_beta * var_qn0_dn0)) * var_lch) + (assign62010_e96393 * var_lch_dn0)), ((((var_beta_dn2 * assign62010_e96392) + (var_beta * var_qn0_dn2)) * var_lch) + (assign62010_e96393 * var_lch_dn2)), ((((var_beta_dn4 * assign62010_e96392) + (var_beta * var_qn0_dn4)) * var_lch) + (assign62010_e96393 * var_lch_dn4)), ((((var_beta_dn5 * assign62010_e96392) + (var_beta * var_qn0_dn5)) * var_lch) + (assign62010_e96393 * var_lch_dn5)), ((((var_beta_dn6 * assign62010_e96392) + (var_beta * var_qn0_dn6)) * var_lch) + (assign62010_e96393 * var_lch_dn6)), ((((var_beta_dn7 * assign62010_e96392) + (var_beta * var_qn0_dn7)) * var_lch) + (assign62010_e96393 * var_lch_dn7)), ((((var_beta_dn8 * assign62010_e96392) + (var_beta * var_qn0_dn8)) * var_lch) + (assign62010_e96393 * var_lch_dn8)), ((((var_beta_dn9 * assign62010_e96392) + (var_beta * var_qn0_dn9)) * var_lch) + (assign62010_e96393 * var_lch_dn9)), ((((var_beta_dn10 * assign62010_e96392) + (var_beta * var_qn0_dn10)) * var_lch) + (assign62010_e96393 * var_lch_dn10)), ((((var_beta_dn13 * assign62010_e96392) + (var_beta * var_qn0_dn13)) * var_lch) + (assign62010_e96393 * var_lch_dn13)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign62010_e96397;
        var_t2_dn0 = assign62010_e96397_d_n0;
        var_t2_dn2 = assign62010_e96397_d_n2;
        var_t2_dn4 = assign62010_e96397_d_n4;
        var_t2_dn5 = assign62010_e96397_d_n5;
        var_t2_dn6 = assign62010_e96397_d_n6;
        var_t2_dn7 = assign62010_e96397_d_n7;
        var_t2_dn8 = assign62010_e96397_d_n8;
        var_t2_dn9 = assign62010_e96397_d_n9;
        var_t2_dn10 = assign62010_e96397_d_n10;
        var_t2_dn13 = assign62010_e96397_d_n13;
        var_t2_rv = 0.0;

        let (assign62020_e96406, assign62020_e96406_d_n0, assign62020_e96406_d_n2, assign62020_e96406_d_n4, assign62020_e96406_d_n5, assign62020_e96406_d_n6, assign62020_e96406_d_n7, assign62020_e96406_d_n8, assign62020_e96406_d_n9, assign62020_e96406_d_n10, assign62020_e96406_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign62020_e96404: f64 = (1.0 / var_t2);
        (assign62020_e96404, (-(var_t2_dn0 / (var_t2 * var_t2))), (-(var_t2_dn2 / (var_t2 * var_t2))), (-(var_t2_dn4 / (var_t2 * var_t2))), (-(var_t2_dn5 / (var_t2 * var_t2))), (-(var_t2_dn6 / (var_t2 * var_t2))), (-(var_t2_dn7 / (var_t2 * var_t2))), (-(var_t2_dn8 / (var_t2 * var_t2))), (-(var_t2_dn9 / (var_t2 * var_t2))), (-(var_t2_dn10 / (var_t2 * var_t2))), (-(var_t2_dn13 / (var_t2 * var_t2))),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign62020_e96406;
        var_t1_dn0 = assign62020_e96406_d_n0;
        var_t1_dn2 = assign62020_e96406_d_n2;
        var_t1_dn4 = assign62020_e96406_d_n4;
        var_t1_dn5 = assign62020_e96406_d_n5;
        var_t1_dn6 = assign62020_e96406_d_n6;
        var_t1_dn7 = assign62020_e96406_d_n7;
        var_t1_dn8 = assign62020_e96406_d_n8;
        var_t1_dn9 = assign62020_e96406_d_n9;
        var_t1_dn10 = assign62020_e96406_d_n10;
        var_t1_dn13 = assign62020_e96406_d_n13;
        var_t1_rv = 0.0;

        let (assign62030_e96415, assign62030_e96415_d_n0, assign62030_e96415_d_n2, assign62030_e96415_d_n4, assign62030_e96415_d_n5, assign62030_e96415_d_n6, assign62030_e96415_d_n7, assign62030_e96415_d_n8, assign62030_e96415_d_n9, assign62030_e96415_d_n10, assign62030_e96415_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign62030_e96413: f64 = (var_t1 * var_t1);
        (assign62030_e96413, ((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)), ((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)), ((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)), ((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)), ((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)), ((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7)), ((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)), ((var_t1_dn9 * var_t1) + (var_t1 * var_t1_dn9)), ((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10)), ((var_t1_dn13 * var_t1) + (var_t1 * var_t1_dn13)),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn13,)
    }
};
        var_t3 = assign62030_e96415;
        var_t3_dn0 = assign62030_e96415_d_n0;
        var_t3_dn2 = assign62030_e96415_d_n2;
        var_t3_dn4 = assign62030_e96415_d_n4;
        var_t3_dn5 = assign62030_e96415_d_n5;
        var_t3_dn6 = assign62030_e96415_d_n6;
        var_t3_dn7 = assign62030_e96415_d_n7;
        var_t3_dn8 = assign62030_e96415_d_n8;
        var_t3_dn9 = assign62030_e96415_d_n9;
        var_t3_dn10 = assign62030_e96415_d_n10;
        var_t3_dn13 = assign62030_e96415_d_n13;
        var_t3_rv = 0.0;

        let (assign62040_e96425, assign62040_e96425_d_n0, assign62040_e96425_d_n2, assign62040_e96425_d_n4, assign62040_e96425_d_n5, assign62040_e96425_d_n6, assign62040_e96425_d_n7, assign62040_e96425_d_n8, assign62040_e96425_d_n9, assign62040_e96425_d_n10, assign62040_e96425_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign62040_e96421: f64 = (-var_beta);
        let assign62040_e96423: f64 = (assign62040_e96421 * var_t3);
        (assign62040_e96423, (((-var_beta_dn0) * var_t3) + (assign62040_e96421 * var_t3_dn0)), (((-var_beta_dn2) * var_t3) + (assign62040_e96421 * var_t3_dn2)), (((-var_beta_dn4) * var_t3) + (assign62040_e96421 * var_t3_dn4)), (((-var_beta_dn5) * var_t3) + (assign62040_e96421 * var_t3_dn5)), (((-var_beta_dn6) * var_t3) + (assign62040_e96421 * var_t3_dn6)), (((-var_beta_dn7) * var_t3) + (assign62040_e96421 * var_t3_dn7)), (((-var_beta_dn8) * var_t3) + (assign62040_e96421 * var_t3_dn8)), (((-var_beta_dn9) * var_t3) + (assign62040_e96421 * var_t3_dn9)), (((-var_beta_dn10) * var_t3) + (assign62040_e96421 * var_t3_dn10)), (((-var_beta_dn13) * var_t3) + (assign62040_e96421 * var_t3_dn13)),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn13,)
    }
};
        var_t4 = assign62040_e96425;
        var_t4_dn0 = assign62040_e96425_d_n0;
        var_t4_dn2 = assign62040_e96425_d_n2;
        var_t4_dn4 = assign62040_e96425_d_n4;
        var_t4_dn5 = assign62040_e96425_d_n5;
        var_t4_dn6 = assign62040_e96425_d_n6;
        var_t4_dn7 = assign62040_e96425_d_n7;
        var_t4_dn8 = assign62040_e96425_d_n8;
        var_t4_dn9 = assign62040_e96425_d_n9;
        var_t4_dn10 = assign62040_e96425_d_n10;
        var_t4_dn13 = assign62040_e96425_d_n13;
        var_t4_rv = 0.0;

        let (assign62050_e96434, assign62050_e96434_d_n0, assign62050_e96434_d_n2, assign62050_e96434_d_n4, assign62050_e96434_d_n5, assign62050_e96434_d_n6, assign62050_e96434_d_n7, assign62050_e96434_d_n8, assign62050_e96434_d_n9, assign62050_e96434_d_n10, assign62050_e96434_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign62050_e96432: f64 = (var_t4 * var_lch);
        (assign62050_e96432, ((var_t4_dn0 * var_lch) + (var_t4 * var_lch_dn0)), ((var_t4_dn2 * var_lch) + (var_t4 * var_lch_dn2)), ((var_t4_dn4 * var_lch) + (var_t4 * var_lch_dn4)), ((var_t4_dn5 * var_lch) + (var_t4 * var_lch_dn5)), ((var_t4_dn6 * var_lch) + (var_t4 * var_lch_dn6)), ((var_t4_dn7 * var_lch) + (var_t4 * var_lch_dn7)), ((var_t4_dn8 * var_lch) + (var_t4 * var_lch_dn8)), ((var_t4_dn9 * var_lch) + (var_t4 * var_lch_dn9)), ((var_t4_dn10 * var_lch) + (var_t4 * var_lch_dn10)), ((var_t4_dn13 * var_lch) + (var_t4 * var_lch_dn13)),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn13,)
    }
};
        var_t5 = assign62050_e96434;
        var_t5_dn0 = assign62050_e96434_d_n0;
        var_t5_dn2 = assign62050_e96434_d_n2;
        var_t5_dn4 = assign62050_e96434_d_n4;
        var_t5_dn5 = assign62050_e96434_d_n5;
        var_t5_dn6 = assign62050_e96434_d_n6;
        var_t5_dn7 = assign62050_e96434_d_n7;
        var_t5_dn8 = assign62050_e96434_d_n8;
        var_t5_dn9 = assign62050_e96434_d_n9;
        var_t5_dn10 = assign62050_e96434_d_n10;
        var_t5_dn13 = assign62050_e96434_d_n13;
        var_t5_rv = 0.0;

        *var_eeff_slot = var_eeff;
        *var_eeff_dn0_slot = var_eeff_dn0;
        *var_eeff_dn10_slot = var_eeff_dn10;
        *var_eeff_dn13_slot = var_eeff_dn13;
        *var_eeff_dn2_slot = var_eeff_dn2;
        *var_eeff_dn4_slot = var_eeff_dn4;
        *var_eeff_dn5_slot = var_eeff_dn5;
        *var_eeff_dn6_slot = var_eeff_dn6;
        *var_eeff_dn7_slot = var_eeff_dn7;
        *var_eeff_dn8_slot = var_eeff_dn8;
        *var_eeff_dn9_slot = var_eeff_dn9;
        *var_eeff_rv_slot = var_eeff_rv;
        *var_lch_slot = var_lch;
        *var_lch_dn0_slot = var_lch_dn0;
        *var_lch_dn10_slot = var_lch_dn10;
        *var_lch_dn13_slot = var_lch_dn13;
        *var_lch_dn2_slot = var_lch_dn2;
        *var_lch_dn4_slot = var_lch_dn4;
        *var_lch_dn5_slot = var_lch_dn5;
        *var_lch_dn6_slot = var_lch_dn6;
        *var_lch_dn7_slot = var_lch_dn7;
        *var_lch_dn8_slot = var_lch_dn8;
        *var_lch_dn9_slot = var_lch_dn9;
        *var_lch_rv_slot = var_lch_rv;
        *var_muun_slot = var_muun;
        *var_muun_dn0_slot = var_muun_dn0;
        *var_muun_dn10_slot = var_muun_dn10;
        *var_muun_dn13_slot = var_muun_dn13;
        *var_muun_dn2_slot = var_muun_dn2;
        *var_muun_dn4_slot = var_muun_dn4;
        *var_muun_dn5_slot = var_muun_dn5;
        *var_muun_dn6_slot = var_muun_dn6;
        *var_muun_dn7_slot = var_muun_dn7;
        *var_muun_dn8_slot = var_muun_dn8;
        *var_muun_dn9_slot = var_muun_dn9;
        *var_muun_rv_slot = var_muun_rv;
        *var_rns_slot = var_rns;
        *var_rns_dn0_slot = var_rns_dn0;
        *var_rns_dn10_slot = var_rns_dn10;
        *var_rns_dn13_slot = var_rns_dn13;
        *var_rns_dn2_slot = var_rns_dn2;
        *var_rns_dn4_slot = var_rns_dn4;
        *var_rns_dn5_slot = var_rns_dn5;
        *var_rns_dn6_slot = var_rns_dn6;
        *var_rns_dn7_slot = var_rns_dn7;
        *var_rns_dn8_slot = var_rns_dn8;
        *var_rns_dn9_slot = var_rns_dn9;
        *var_rns_rv_slot = var_rns_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn13_slot = var_t0_dn13;
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
        *var_t1_dn13_slot = var_t1_dn13;
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
        *var_t2_dn13_slot = var_t2_dn13;
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
        *var_t3_dn13_slot = var_t3_dn13;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t3_rv_slot = var_t3_rv;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn13_slot = var_t4_dn13;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_dn9_slot = var_t4_dn9;
        *var_t4_rv_slot = var_t4_rv;
        *var_t5_slot = var_t5;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn10_slot = var_t5_dn10;
        *var_t5_dn13_slot = var_t5_dn13;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t5_dn9_slot = var_t5_dn9;
        *var_t5_rv_slot = var_t5_rv;
        *var_t6_slot = var_t6;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn13_slot = var_t6_dn13;
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
        *var_t7_dn13_slot = var_t7_dn13;
        *var_t7_dn2_slot = var_t7_dn2;
        *var_t7_dn4_slot = var_t7_dn4;
        *var_t7_dn5_slot = var_t7_dn5;
        *var_t7_dn6_slot = var_t7_dn6;
        *var_t7_dn7_slot = var_t7_dn7;
        *var_t7_dn8_slot = var_t7_dn8;
        *var_t7_dn9_slot = var_t7_dn9;
        *var_t7_rv_slot = var_t7_rv;
        *var_t8_slot = var_t8;
        *var_t8_dn0_slot = var_t8_dn0;
        *var_t8_dn10_slot = var_t8_dn10;
        *var_t8_dn13_slot = var_t8_dn13;
        *var_t8_dn2_slot = var_t8_dn2;
        *var_t8_dn4_slot = var_t8_dn4;
        *var_t8_dn5_slot = var_t8_dn5;
        *var_t8_dn6_slot = var_t8_dn6;
        *var_t8_dn7_slot = var_t8_dn7;
        *var_t8_dn8_slot = var_t8_dn8;
        *var_t8_dn9_slot = var_t8_dn9;
        *var_t8_rv_slot = var_t8_rv;
        *var_t9_slot = var_t9;
        *var_t9_dn0_slot = var_t9_dn0;
        *var_t9_dn10_slot = var_t9_dn10;
        *var_t9_dn13_slot = var_t9_dn13;
        *var_t9_dn2_slot = var_t9_dn2;
        *var_t9_dn4_slot = var_t9_dn4;
        *var_t9_dn5_slot = var_t9_dn5;
        *var_t9_dn6_slot = var_t9_dn6;
        *var_t9_dn7_slot = var_t9_dn7;
        *var_t9_dn8_slot = var_t9_dn8;
        *var_t9_dn9_slot = var_t9_dn9;
        *var_t9_rv_slot = var_t9_rv;
    }

    pub(super) fn stamp_reactive_block_222(
        p: &Parameters,
        var_beta_inv: f64,
        var_beta_inv_dn0: f64,
        var_beta_inv_dn10: f64,
        var_beta_inv_dn13: f64,
        var_beta_inv_dn2: f64,
        var_beta_inv_dn4: f64,
        var_beta_inv_dn5: f64,
        var_beta_inv_dn6: f64,
        var_beta_inv_dn7: f64,
        var_beta_inv_dn8: f64,
        var_beta_inv_dn9: f64,
        var_fdd: f64,
        var_fdd_dn0: f64,
        var_fdd_dn10: f64,
        var_fdd_dn13: f64,
        var_fdd_dn2: f64,
        var_fdd_dn4: f64,
        var_fdd_dn5: f64,
        var_fdd_dn6: f64,
        var_fdd_dn7: f64,
        var_fdd_dn8: f64,
        var_fdd_dn9: f64,
        var_guard1430: f64,
        var_guard443: f64,
        var_idd: f64,
        var_idd_dn0: f64,
        var_idd_dn10: f64,
        var_idd_dn13: f64,
        var_idd_dn2: f64,
        var_idd_dn4: f64,
        var_idd_dn5: f64,
        var_idd_dn6: f64,
        var_idd_dn7: f64,
        var_idd_dn8: f64,
        var_idd_dn9: f64,
        var_lch: f64,
        var_lch_dn0: f64,
        var_lch_dn10: f64,
        var_lch_dn13: f64,
        var_lch_dn2: f64,
        var_lch_dn4: f64,
        var_lch_dn5: f64,
        var_lch_dn6: f64,
        var_lch_dn7: f64,
        var_lch_dn8: f64,
        var_lch_dn9: f64,
        var_muun: f64,
        var_muun_dn0: f64,
        var_muun_dn10: f64,
        var_muun_dn13: f64,
        var_muun_dn2: f64,
        var_muun_dn4: f64,
        var_muun_dn5: f64,
        var_muun_dn6: f64,
        var_muun_dn7: f64,
        var_muun_dn8: f64,
        var_muun_dn9: f64,
        var_pds: f64,
        var_pds_dn0: f64,
        var_pds_dn10: f64,
        var_pds_dn13: f64,
        var_pds_dn2: f64,
        var_pds_dn4: f64,
        var_pds_dn5: f64,
        var_pds_dn6: f64,
        var_pds_dn7: f64,
        var_pds_dn8: f64,
        var_pds_dn9: f64,
        var_qn0: f64,
        var_qn0_dn0: f64,
        var_qn0_dn10: f64,
        var_qn0_dn13: f64,
        var_qn0_dn2: f64,
        var_qn0_dn4: f64,
        var_qn0_dn5: f64,
        var_qn0_dn6: f64,
        var_qn0_dn7: f64,
        var_qn0_dn8: f64,
        var_qn0_dn9: f64,
        var_vmaxe: f64,
        var_vmaxe_dn0: f64,
        var_vmaxe_dn10: f64,
        var_vmaxe_dn13: f64,
        var_vmaxe_dn2: f64,
        var_vmaxe_dn4: f64,
        var_vmaxe_dn5: f64,
        var_vmaxe_dn6: f64,
        var_vmaxe_dn7: f64,
        var_vmaxe_dn8: f64,
        var_vmaxe_dn9: f64,
        var_weff_nf: f64,
        var_betawl_slot: &mut f64,
        var_betawl_dn0_slot: &mut f64,
        var_betawl_dn10_slot: &mut f64,
        var_betawl_dn13_slot: &mut f64,
        var_betawl_dn2_slot: &mut f64,
        var_betawl_dn4_slot: &mut f64,
        var_betawl_dn5_slot: &mut f64,
        var_betawl_dn6_slot: &mut f64,
        var_betawl_dn7_slot: &mut f64,
        var_betawl_dn8_slot: &mut f64,
        var_betawl_dn9_slot: &mut f64,
        var_betawl_rv_slot: &mut f64,
        var_em_slot: &mut f64,
        var_em_dn0_slot: &mut f64,
        var_em_dn10_slot: &mut f64,
        var_em_dn13_slot: &mut f64,
        var_em_dn2_slot: &mut f64,
        var_em_dn4_slot: &mut f64,
        var_em_dn5_slot: &mut f64,
        var_em_dn6_slot: &mut f64,
        var_em_dn7_slot: &mut f64,
        var_em_dn8_slot: &mut f64,
        var_em_dn9_slot: &mut f64,
        var_em_rv_slot: &mut f64,
        var_ey_slot: &mut f64,
        var_ey_dn0_slot: &mut f64,
        var_ey_dn10_slot: &mut f64,
        var_ey_dn13_slot: &mut f64,
        var_ey_dn2_slot: &mut f64,
        var_ey_dn4_slot: &mut f64,
        var_ey_dn5_slot: &mut f64,
        var_ey_dn6_slot: &mut f64,
        var_ey_dn7_slot: &mut f64,
        var_ey_dn8_slot: &mut f64,
        var_ey_dn9_slot: &mut f64,
        var_ey_rv_slot: &mut f64,
        var_guard1491_slot: &mut f64,
        var_guard1491_rv_slot: &mut f64,
        var_guard1492_slot: &mut f64,
        var_guard1492_rv_slot: &mut f64,
        var_guard1493_slot: &mut f64,
        var_guard1493_rv_slot: &mut f64,
        var_guard1494_slot: &mut f64,
        var_guard1494_rv_slot: &mut f64,
        var_guard1495_slot: &mut f64,
        var_guard1495_rv_slot: &mut f64,
        var_ids0_slot: &mut f64,
        var_ids0_dn0_slot: &mut f64,
        var_ids0_dn10_slot: &mut f64,
        var_ids0_dn13_slot: &mut f64,
        var_ids0_dn2_slot: &mut f64,
        var_ids0_dn4_slot: &mut f64,
        var_ids0_dn5_slot: &mut f64,
        var_ids0_dn6_slot: &mut f64,
        var_ids0_dn7_slot: &mut f64,
        var_ids0_dn8_slot: &mut f64,
        var_ids0_dn9_slot: &mut f64,
        var_ids0_rv_slot: &mut f64,
        var_mu_slot: &mut f64,
        var_mu_dn0_slot: &mut f64,
        var_mu_dn10_slot: &mut f64,
        var_mu_dn13_slot: &mut f64,
        var_mu_dn2_slot: &mut f64,
        var_mu_dn4_slot: &mut f64,
        var_mu_dn5_slot: &mut f64,
        var_mu_dn6_slot: &mut f64,
        var_mu_dn7_slot: &mut f64,
        var_mu_dn8_slot: &mut f64,
        var_mu_dn9_slot: &mut f64,
        var_mu_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn13_slot: &mut f64,
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
        var_t2_dn13_slot: &mut f64,
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
        var_t3_dn13_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn13_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_dn9_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn10_slot: &mut f64,
        var_t5_dn13_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t5_dn9_slot: &mut f64,
        var_t5_rv_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn13_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t6_dn9_slot: &mut f64,
        var_t6_rv_slot: &mut f64,
        var_ty_slot: &mut f64,
        var_ty_dn0_slot: &mut f64,
        var_ty_dn10_slot: &mut f64,
        var_ty_dn13_slot: &mut f64,
        var_ty_dn2_slot: &mut f64,
        var_ty_dn4_slot: &mut f64,
        var_ty_dn5_slot: &mut f64,
        var_ty_dn6_slot: &mut f64,
        var_ty_dn7_slot: &mut f64,
        var_ty_dn8_slot: &mut f64,
        var_ty_dn9_slot: &mut f64,
        var_ty_rv_slot: &mut f64,
    ) {
        let mut var_betawl: f64 = *var_betawl_slot;
        let mut var_betawl_dn0: f64 = *var_betawl_dn0_slot;
        let mut var_betawl_dn10: f64 = *var_betawl_dn10_slot;
        let mut var_betawl_dn13: f64 = *var_betawl_dn13_slot;
        let mut var_betawl_dn2: f64 = *var_betawl_dn2_slot;
        let mut var_betawl_dn4: f64 = *var_betawl_dn4_slot;
        let mut var_betawl_dn5: f64 = *var_betawl_dn5_slot;
        let mut var_betawl_dn6: f64 = *var_betawl_dn6_slot;
        let mut var_betawl_dn7: f64 = *var_betawl_dn7_slot;
        let mut var_betawl_dn8: f64 = *var_betawl_dn8_slot;
        let mut var_betawl_dn9: f64 = *var_betawl_dn9_slot;
        let mut var_betawl_rv: f64 = *var_betawl_rv_slot;
        let mut var_em: f64 = *var_em_slot;
        let mut var_em_dn0: f64 = *var_em_dn0_slot;
        let mut var_em_dn10: f64 = *var_em_dn10_slot;
        let mut var_em_dn13: f64 = *var_em_dn13_slot;
        let mut var_em_dn2: f64 = *var_em_dn2_slot;
        let mut var_em_dn4: f64 = *var_em_dn4_slot;
        let mut var_em_dn5: f64 = *var_em_dn5_slot;
        let mut var_em_dn6: f64 = *var_em_dn6_slot;
        let mut var_em_dn7: f64 = *var_em_dn7_slot;
        let mut var_em_dn8: f64 = *var_em_dn8_slot;
        let mut var_em_dn9: f64 = *var_em_dn9_slot;
        let mut var_em_rv: f64 = *var_em_rv_slot;
        let mut var_ey: f64 = *var_ey_slot;
        let mut var_ey_dn0: f64 = *var_ey_dn0_slot;
        let mut var_ey_dn10: f64 = *var_ey_dn10_slot;
        let mut var_ey_dn13: f64 = *var_ey_dn13_slot;
        let mut var_ey_dn2: f64 = *var_ey_dn2_slot;
        let mut var_ey_dn4: f64 = *var_ey_dn4_slot;
        let mut var_ey_dn5: f64 = *var_ey_dn5_slot;
        let mut var_ey_dn6: f64 = *var_ey_dn6_slot;
        let mut var_ey_dn7: f64 = *var_ey_dn7_slot;
        let mut var_ey_dn8: f64 = *var_ey_dn8_slot;
        let mut var_ey_dn9: f64 = *var_ey_dn9_slot;
        let mut var_ey_rv: f64 = *var_ey_rv_slot;
        let mut var_guard1491: f64 = *var_guard1491_slot;
        let mut var_guard1491_rv: f64 = *var_guard1491_rv_slot;
        let mut var_guard1492: f64 = *var_guard1492_slot;
        let mut var_guard1492_rv: f64 = *var_guard1492_rv_slot;
        let mut var_guard1493: f64 = *var_guard1493_slot;
        let mut var_guard1493_rv: f64 = *var_guard1493_rv_slot;
        let mut var_guard1494: f64 = *var_guard1494_slot;
        let mut var_guard1494_rv: f64 = *var_guard1494_rv_slot;
        let mut var_guard1495: f64 = *var_guard1495_slot;
        let mut var_guard1495_rv: f64 = *var_guard1495_rv_slot;
        let mut var_ids0: f64 = *var_ids0_slot;
        let mut var_ids0_dn0: f64 = *var_ids0_dn0_slot;
        let mut var_ids0_dn10: f64 = *var_ids0_dn10_slot;
        let mut var_ids0_dn13: f64 = *var_ids0_dn13_slot;
        let mut var_ids0_dn2: f64 = *var_ids0_dn2_slot;
        let mut var_ids0_dn4: f64 = *var_ids0_dn4_slot;
        let mut var_ids0_dn5: f64 = *var_ids0_dn5_slot;
        let mut var_ids0_dn6: f64 = *var_ids0_dn6_slot;
        let mut var_ids0_dn7: f64 = *var_ids0_dn7_slot;
        let mut var_ids0_dn8: f64 = *var_ids0_dn8_slot;
        let mut var_ids0_dn9: f64 = *var_ids0_dn9_slot;
        let mut var_ids0_rv: f64 = *var_ids0_rv_slot;
        let mut var_mu: f64 = *var_mu_slot;
        let mut var_mu_dn0: f64 = *var_mu_dn0_slot;
        let mut var_mu_dn10: f64 = *var_mu_dn10_slot;
        let mut var_mu_dn13: f64 = *var_mu_dn13_slot;
        let mut var_mu_dn2: f64 = *var_mu_dn2_slot;
        let mut var_mu_dn4: f64 = *var_mu_dn4_slot;
        let mut var_mu_dn5: f64 = *var_mu_dn5_slot;
        let mut var_mu_dn6: f64 = *var_mu_dn6_slot;
        let mut var_mu_dn7: f64 = *var_mu_dn7_slot;
        let mut var_mu_dn8: f64 = *var_mu_dn8_slot;
        let mut var_mu_dn9: f64 = *var_mu_dn9_slot;
        let mut var_mu_rv: f64 = *var_mu_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
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
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
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
        let mut var_t3_dn13: f64 = *var_t3_dn13_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn13: f64 = *var_t4_dn13_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_dn9: f64 = *var_t4_dn9_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn10: f64 = *var_t5_dn10_slot;
        let mut var_t5_dn13: f64 = *var_t5_dn13_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t5_dn9: f64 = *var_t5_dn9_slot;
        let mut var_t5_rv: f64 = *var_t5_rv_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn13: f64 = *var_t6_dn13_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t6_dn9: f64 = *var_t6_dn9_slot;
        let mut var_t6_rv: f64 = *var_t6_rv_slot;
        let mut var_ty: f64 = *var_ty_slot;
        let mut var_ty_dn0: f64 = *var_ty_dn0_slot;
        let mut var_ty_dn10: f64 = *var_ty_dn10_slot;
        let mut var_ty_dn13: f64 = *var_ty_dn13_slot;
        let mut var_ty_dn2: f64 = *var_ty_dn2_slot;
        let mut var_ty_dn4: f64 = *var_ty_dn4_slot;
        let mut var_ty_dn5: f64 = *var_ty_dn5_slot;
        let mut var_ty_dn6: f64 = *var_ty_dn6_slot;
        let mut var_ty_dn7: f64 = *var_ty_dn7_slot;
        let mut var_ty_dn8: f64 = *var_ty_dn8_slot;
        let mut var_ty_dn9: f64 = *var_ty_dn9_slot;
        let mut var_ty_rv: f64 = *var_ty_rv_slot;

        let (assign62060_e96445, assign62060_e96445_d_n0, assign62060_e96445_d_n2, assign62060_e96445_d_n4, assign62060_e96445_d_n5, assign62060_e96445_d_n6, assign62060_e96445_d_n7, assign62060_e96445_d_n8, assign62060_e96445_d_n9, assign62060_e96445_d_n10, assign62060_e96445_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign62060_e96442: f64 = (var_qn0 + 1e-25);
        let assign62060_e96443: f64 = (var_t4 * assign62060_e96442);
        (assign62060_e96443, ((var_t4_dn0 * assign62060_e96442) + (var_t4 * var_qn0_dn0)), ((var_t4_dn2 * assign62060_e96442) + (var_t4 * var_qn0_dn2)), ((var_t4_dn4 * assign62060_e96442) + (var_t4 * var_qn0_dn4)), ((var_t4_dn5 * assign62060_e96442) + (var_t4 * var_qn0_dn5)), ((var_t4_dn6 * assign62060_e96442) + (var_t4 * var_qn0_dn6)), ((var_t4_dn7 * assign62060_e96442) + (var_t4 * var_qn0_dn7)), ((var_t4_dn8 * assign62060_e96442) + (var_t4 * var_qn0_dn8)), ((var_t4_dn9 * assign62060_e96442) + (var_t4 * var_qn0_dn9)), ((var_t4_dn10 * assign62060_e96442) + (var_t4 * var_qn0_dn10)), ((var_t4_dn13 * assign62060_e96442) + (var_t4 * var_qn0_dn13)),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn13,)
    }
};
        var_t6 = assign62060_e96445;
        var_t6_dn0 = assign62060_e96445_d_n0;
        var_t6_dn2 = assign62060_e96445_d_n2;
        var_t6_dn4 = assign62060_e96445_d_n4;
        var_t6_dn5 = assign62060_e96445_d_n5;
        var_t6_dn6 = assign62060_e96445_d_n6;
        var_t6_dn7 = assign62060_e96445_d_n7;
        var_t6_dn8 = assign62060_e96445_d_n8;
        var_t6_dn9 = assign62060_e96445_d_n9;
        var_t6_dn10 = assign62060_e96445_d_n10;
        var_t6_dn13 = assign62060_e96445_d_n13;
        var_t6_rv = 0.0;

        let (assign62070_e96460, assign62070_e96460_d_n0, assign62070_e96460_d_n2, assign62070_e96460_d_n4, assign62070_e96460_d_n5, assign62070_e96460_d_n6, assign62070_e96460_d_n7, assign62070_e96460_d_n8, assign62070_e96460_d_n9, assign62070_e96460_d_n10, assign62070_e96460_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign62070_e96453: f64 = (10.0 * 2.220446049250313e-16);
        let assign62070_e96454: f64 = (var_pds + assign62070_e96453);
        let assign62070_e96456: f64 = (assign62070_e96454 * var_fdd);
        let assign62070_e96458: f64 = (assign62070_e96456 * var_t1);
        (assign62070_e96458, ((((var_pds_dn0 * var_fdd) + (assign62070_e96454 * var_fdd_dn0)) * var_t1) + (assign62070_e96456 * var_t1_dn0)), ((((var_pds_dn2 * var_fdd) + (assign62070_e96454 * var_fdd_dn2)) * var_t1) + (assign62070_e96456 * var_t1_dn2)), ((((var_pds_dn4 * var_fdd) + (assign62070_e96454 * var_fdd_dn4)) * var_t1) + (assign62070_e96456 * var_t1_dn4)), ((((var_pds_dn5 * var_fdd) + (assign62070_e96454 * var_fdd_dn5)) * var_t1) + (assign62070_e96456 * var_t1_dn5)), ((((var_pds_dn6 * var_fdd) + (assign62070_e96454 * var_fdd_dn6)) * var_t1) + (assign62070_e96456 * var_t1_dn6)), ((((var_pds_dn7 * var_fdd) + (assign62070_e96454 * var_fdd_dn7)) * var_t1) + (assign62070_e96456 * var_t1_dn7)), ((((var_pds_dn8 * var_fdd) + (assign62070_e96454 * var_fdd_dn8)) * var_t1) + (assign62070_e96456 * var_t1_dn8)), ((((var_pds_dn9 * var_fdd) + (assign62070_e96454 * var_fdd_dn9)) * var_t1) + (assign62070_e96456 * var_t1_dn9)), ((((var_pds_dn10 * var_fdd) + (assign62070_e96454 * var_fdd_dn10)) * var_t1) + (assign62070_e96456 * var_t1_dn10)), ((((var_pds_dn13 * var_fdd) + (assign62070_e96454 * var_fdd_dn13)) * var_t1) + (assign62070_e96456 * var_t1_dn13)),)
    } else {
        (var_ty, var_ty_dn0, var_ty_dn2, var_ty_dn4, var_ty_dn5, var_ty_dn6, var_ty_dn7, var_ty_dn8, var_ty_dn9, var_ty_dn10, var_ty_dn13,)
    }
};
        var_ty = assign62070_e96460;
        var_ty_dn0 = assign62070_e96460_d_n0;
        var_ty_dn2 = assign62070_e96460_d_n2;
        var_ty_dn4 = assign62070_e96460_d_n4;
        var_ty_dn5 = assign62070_e96460_d_n5;
        var_ty_dn6 = assign62070_e96460_d_n6;
        var_ty_dn7 = assign62070_e96460_d_n7;
        var_ty_dn8 = assign62070_e96460_d_n8;
        var_ty_dn9 = assign62070_e96460_d_n9;
        var_ty_dn10 = assign62070_e96460_d_n10;
        var_ty_dn13 = assign62070_e96460_d_n13;
        var_ty_rv = 0.0;

        let (assign62080_e96471, assign62080_e96471_d_n0, assign62080_e96471_d_n2, assign62080_e96471_d_n4, assign62080_e96471_d_n5, assign62080_e96471_d_n6, assign62080_e96471_d_n7, assign62080_e96471_d_n8, assign62080_e96471_d_n9, assign62080_e96471_d_n10, assign62080_e96471_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign62080_e96467: f64 = (0.2 * var_vmaxe);
        let assign62080_e96469: f64 = (assign62080_e96467 / var_muun);
        (assign62080_e96469, ((((0.2 * var_vmaxe_dn0) * var_muun) - (assign62080_e96467 * var_muun_dn0)) / (var_muun * var_muun)), ((((0.2 * var_vmaxe_dn2) * var_muun) - (assign62080_e96467 * var_muun_dn2)) / (var_muun * var_muun)), ((((0.2 * var_vmaxe_dn4) * var_muun) - (assign62080_e96467 * var_muun_dn4)) / (var_muun * var_muun)), ((((0.2 * var_vmaxe_dn5) * var_muun) - (assign62080_e96467 * var_muun_dn5)) / (var_muun * var_muun)), ((((0.2 * var_vmaxe_dn6) * var_muun) - (assign62080_e96467 * var_muun_dn6)) / (var_muun * var_muun)), ((((0.2 * var_vmaxe_dn7) * var_muun) - (assign62080_e96467 * var_muun_dn7)) / (var_muun * var_muun)), ((((0.2 * var_vmaxe_dn8) * var_muun) - (assign62080_e96467 * var_muun_dn8)) / (var_muun * var_muun)), ((((0.2 * var_vmaxe_dn9) * var_muun) - (assign62080_e96467 * var_muun_dn9)) / (var_muun * var_muun)), ((((0.2 * var_vmaxe_dn10) * var_muun) - (assign62080_e96467 * var_muun_dn10)) / (var_muun * var_muun)), ((((0.2 * var_vmaxe_dn13) * var_muun) - (assign62080_e96467 * var_muun_dn13)) / (var_muun * var_muun)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign62080_e96471;
        var_t2_dn0 = assign62080_e96471_d_n0;
        var_t2_dn2 = assign62080_e96471_d_n2;
        var_t2_dn4 = assign62080_e96471_d_n4;
        var_t2_dn5 = assign62080_e96471_d_n5;
        var_t2_dn6 = assign62080_e96471_d_n6;
        var_t2_dn7 = assign62080_e96471_d_n7;
        var_t2_dn8 = assign62080_e96471_d_n8;
        var_t2_dn9 = assign62080_e96471_d_n9;
        var_t2_dn10 = assign62080_e96471_d_n10;
        var_t2_dn13 = assign62080_e96471_d_n13;
        var_t2_rv = 0.0;

        let (assign62090_e96481, assign62090_e96481_d_n0, assign62090_e96481_d_n2, assign62090_e96481_d_n4, assign62090_e96481_d_n5, assign62090_e96481_d_n6, assign62090_e96481_d_n7, assign62090_e96481_d_n8, assign62090_e96481_d_n9, assign62090_e96481_d_n10, assign62090_e96481_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign62090_e96477: f64 = (-var_t2);
        let assign62090_e96479: f64 = (assign62090_e96477 / var_muun);
        (assign62090_e96479, ((((-var_t2_dn0) * var_muun) - (assign62090_e96477 * var_muun_dn0)) / (var_muun * var_muun)), ((((-var_t2_dn2) * var_muun) - (assign62090_e96477 * var_muun_dn2)) / (var_muun * var_muun)), ((((-var_t2_dn4) * var_muun) - (assign62090_e96477 * var_muun_dn4)) / (var_muun * var_muun)), ((((-var_t2_dn5) * var_muun) - (assign62090_e96477 * var_muun_dn5)) / (var_muun * var_muun)), ((((-var_t2_dn6) * var_muun) - (assign62090_e96477 * var_muun_dn6)) / (var_muun * var_muun)), ((((-var_t2_dn7) * var_muun) - (assign62090_e96477 * var_muun_dn7)) / (var_muun * var_muun)), ((((-var_t2_dn8) * var_muun) - (assign62090_e96477 * var_muun_dn8)) / (var_muun * var_muun)), ((((-var_t2_dn9) * var_muun) - (assign62090_e96477 * var_muun_dn9)) / (var_muun * var_muun)), ((((-var_t2_dn10) * var_muun) - (assign62090_e96477 * var_muun_dn10)) / (var_muun * var_muun)), ((((-var_t2_dn13) * var_muun) - (assign62090_e96477 * var_muun_dn13)) / (var_muun * var_muun)),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn13,)
    }
};
        var_t3 = assign62090_e96481;
        var_t3_dn0 = assign62090_e96481_d_n0;
        var_t3_dn2 = assign62090_e96481_d_n2;
        var_t3_dn4 = assign62090_e96481_d_n4;
        var_t3_dn5 = assign62090_e96481_d_n5;
        var_t3_dn6 = assign62090_e96481_d_n6;
        var_t3_dn7 = assign62090_e96481_d_n7;
        var_t3_dn8 = assign62090_e96481_d_n8;
        var_t3_dn9 = assign62090_e96481_d_n9;
        var_t3_dn10 = assign62090_e96481_d_n10;
        var_t3_dn13 = assign62090_e96481_d_n13;
        var_t3_rv = 0.0;

        let (assign62100_e96495, assign62100_e96495_d_n0, assign62100_e96495_d_n2, assign62100_e96495_d_n4, assign62100_e96495_d_n5, assign62100_e96495_d_n6, assign62100_e96495_d_n7, assign62100_e96495_d_n8, assign62100_e96495_d_n9, assign62100_e96495_d_n10, assign62100_e96495_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign62100_e96488: f64 = (var_ty * var_ty);
        let assign62100_e96491: f64 = (var_t2 * var_t2);
        let assign62100_e96492: f64 = (assign62100_e96488 + assign62100_e96491);
        let assign62100_e96493: f64 = (assign62100_e96492).sqrt();
        (assign62100_e96493, ((((var_ty_dn0 * var_ty) + (var_ty * var_ty_dn0)) + ((var_t2_dn0 * var_t2) + (var_t2 * var_t2_dn0))) / (2.0 * assign62100_e96493)), ((((var_ty_dn2 * var_ty) + (var_ty * var_ty_dn2)) + ((var_t2_dn2 * var_t2) + (var_t2 * var_t2_dn2))) / (2.0 * assign62100_e96493)), ((((var_ty_dn4 * var_ty) + (var_ty * var_ty_dn4)) + ((var_t2_dn4 * var_t2) + (var_t2 * var_t2_dn4))) / (2.0 * assign62100_e96493)), ((((var_ty_dn5 * var_ty) + (var_ty * var_ty_dn5)) + ((var_t2_dn5 * var_t2) + (var_t2 * var_t2_dn5))) / (2.0 * assign62100_e96493)), ((((var_ty_dn6 * var_ty) + (var_ty * var_ty_dn6)) + ((var_t2_dn6 * var_t2) + (var_t2 * var_t2_dn6))) / (2.0 * assign62100_e96493)), ((((var_ty_dn7 * var_ty) + (var_ty * var_ty_dn7)) + ((var_t2_dn7 * var_t2) + (var_t2 * var_t2_dn7))) / (2.0 * assign62100_e96493)), ((((var_ty_dn8 * var_ty) + (var_ty * var_ty_dn8)) + ((var_t2_dn8 * var_t2) + (var_t2 * var_t2_dn8))) / (2.0 * assign62100_e96493)), ((((var_ty_dn9 * var_ty) + (var_ty * var_ty_dn9)) + ((var_t2_dn9 * var_t2) + (var_t2 * var_t2_dn9))) / (2.0 * assign62100_e96493)), ((((var_ty_dn10 * var_ty) + (var_ty * var_ty_dn10)) + ((var_t2_dn10 * var_t2) + (var_t2 * var_t2_dn10))) / (2.0 * assign62100_e96493)), ((((var_ty_dn13 * var_ty) + (var_ty * var_ty_dn13)) + ((var_t2_dn13 * var_t2) + (var_t2 * var_t2_dn13))) / (2.0 * assign62100_e96493)),)
    } else {
        (var_ey, var_ey_dn0, var_ey_dn2, var_ey_dn4, var_ey_dn5, var_ey_dn6, var_ey_dn7, var_ey_dn8, var_ey_dn9, var_ey_dn10, var_ey_dn13,)
    }
};
        var_ey = assign62100_e96495;
        var_ey_dn0 = assign62100_e96495_d_n0;
        var_ey_dn2 = assign62100_e96495_d_n2;
        var_ey_dn4 = assign62100_e96495_d_n4;
        var_ey_dn5 = assign62100_e96495_d_n5;
        var_ey_dn6 = assign62100_e96495_d_n6;
        var_ey_dn7 = assign62100_e96495_d_n7;
        var_ey_dn8 = assign62100_e96495_d_n8;
        var_ey_dn9 = assign62100_e96495_d_n9;
        var_ey_dn10 = assign62100_e96495_d_n10;
        var_ey_dn13 = assign62100_e96495_d_n13;
        var_ey_rv = 0.0;

        let (assign62110_e96504, assign62110_e96504_d_n0, assign62110_e96504_d_n2, assign62110_e96504_d_n4, assign62110_e96504_d_n5, assign62110_e96504_d_n6, assign62110_e96504_d_n7, assign62110_e96504_d_n8, assign62110_e96504_d_n9, assign62110_e96504_d_n10, assign62110_e96504_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign62110_e96502: f64 = (1.0 / var_ey);
        (assign62110_e96502, (-(var_ey_dn0 / (var_ey * var_ey))), (-(var_ey_dn2 / (var_ey * var_ey))), (-(var_ey_dn4 / (var_ey * var_ey))), (-(var_ey_dn5 / (var_ey * var_ey))), (-(var_ey_dn6 / (var_ey * var_ey))), (-(var_ey_dn7 / (var_ey * var_ey))), (-(var_ey_dn8 / (var_ey * var_ey))), (-(var_ey_dn9 / (var_ey * var_ey))), (-(var_ey_dn10 / (var_ey * var_ey))), (-(var_ey_dn13 / (var_ey * var_ey))),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn13,)
    }
};
        var_t4 = assign62110_e96504;
        var_t4_dn0 = assign62110_e96504_d_n0;
        var_t4_dn2 = assign62110_e96504_d_n2;
        var_t4_dn4 = assign62110_e96504_d_n4;
        var_t4_dn5 = assign62110_e96504_d_n5;
        var_t4_dn6 = assign62110_e96504_d_n6;
        var_t4_dn7 = assign62110_e96504_d_n7;
        var_t4_dn8 = assign62110_e96504_d_n8;
        var_t4_dn9 = assign62110_e96504_d_n9;
        var_t4_dn10 = assign62110_e96504_d_n10;
        var_t4_dn13 = assign62110_e96504_d_n13;
        var_t4_rv = 0.0;

        let (assign62120_e96513, assign62120_e96513_d_n0, assign62120_e96513_d_n2, assign62120_e96513_d_n4, assign62120_e96513_d_n5, assign62120_e96513_d_n6, assign62120_e96513_d_n7, assign62120_e96513_d_n8, assign62120_e96513_d_n9, assign62120_e96513_d_n10, assign62120_e96513_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign62120_e96511: f64 = (var_muun * var_ey);
        (assign62120_e96511, ((var_muun_dn0 * var_ey) + (var_muun * var_ey_dn0)), ((var_muun_dn2 * var_ey) + (var_muun * var_ey_dn2)), ((var_muun_dn4 * var_ey) + (var_muun * var_ey_dn4)), ((var_muun_dn5 * var_ey) + (var_muun * var_ey_dn5)), ((var_muun_dn6 * var_ey) + (var_muun * var_ey_dn6)), ((var_muun_dn7 * var_ey) + (var_muun * var_ey_dn7)), ((var_muun_dn8 * var_ey) + (var_muun * var_ey_dn8)), ((var_muun_dn9 * var_ey) + (var_muun * var_ey_dn9)), ((var_muun_dn10 * var_ey) + (var_muun * var_ey_dn10)), ((var_muun_dn13 * var_ey) + (var_muun * var_ey_dn13)),)
    } else {
        (var_em, var_em_dn0, var_em_dn2, var_em_dn4, var_em_dn5, var_em_dn6, var_em_dn7, var_em_dn8, var_em_dn9, var_em_dn10, var_em_dn13,)
    }
};
        var_em = assign62120_e96513;
        var_em_dn0 = assign62120_e96513_d_n0;
        var_em_dn2 = assign62120_e96513_d_n2;
        var_em_dn4 = assign62120_e96513_d_n4;
        var_em_dn5 = assign62120_e96513_d_n5;
        var_em_dn6 = assign62120_e96513_d_n6;
        var_em_dn7 = assign62120_e96513_d_n7;
        var_em_dn8 = assign62120_e96513_d_n8;
        var_em_dn9 = assign62120_e96513_d_n9;
        var_em_dn10 = assign62120_e96513_d_n10;
        var_em_dn13 = assign62120_e96513_d_n13;
        var_em_rv = 0.0;

        let (assign62130_e96522, assign62130_e96522_d_n0, assign62130_e96522_d_n2, assign62130_e96522_d_n4, assign62130_e96522_d_n5, assign62130_e96522_d_n6, assign62130_e96522_d_n7, assign62130_e96522_d_n8, assign62130_e96522_d_n9, assign62130_e96522_d_n10, assign62130_e96522_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign62130_e96520: f64 = (var_em / var_vmaxe);
        (assign62130_e96520, (((var_em_dn0 * var_vmaxe) - (var_em * var_vmaxe_dn0)) / (var_vmaxe * var_vmaxe)), (((var_em_dn2 * var_vmaxe) - (var_em * var_vmaxe_dn2)) / (var_vmaxe * var_vmaxe)), (((var_em_dn4 * var_vmaxe) - (var_em * var_vmaxe_dn4)) / (var_vmaxe * var_vmaxe)), (((var_em_dn5 * var_vmaxe) - (var_em * var_vmaxe_dn5)) / (var_vmaxe * var_vmaxe)), (((var_em_dn6 * var_vmaxe) - (var_em * var_vmaxe_dn6)) / (var_vmaxe * var_vmaxe)), (((var_em_dn7 * var_vmaxe) - (var_em * var_vmaxe_dn7)) / (var_vmaxe * var_vmaxe)), (((var_em_dn8 * var_vmaxe) - (var_em * var_vmaxe_dn8)) / (var_vmaxe * var_vmaxe)), (((var_em_dn9 * var_vmaxe) - (var_em * var_vmaxe_dn9)) / (var_vmaxe * var_vmaxe)), (((var_em_dn10 * var_vmaxe) - (var_em * var_vmaxe_dn10)) / (var_vmaxe * var_vmaxe)), (((var_em_dn13 * var_vmaxe) - (var_em * var_vmaxe_dn13)) / (var_vmaxe * var_vmaxe)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign62130_e96522;
        var_t1_dn0 = assign62130_e96522_d_n0;
        var_t1_dn2 = assign62130_e96522_d_n2;
        var_t1_dn4 = assign62130_e96522_d_n4;
        var_t1_dn5 = assign62130_e96522_d_n5;
        var_t1_dn6 = assign62130_e96522_d_n6;
        var_t1_dn7 = assign62130_e96522_d_n7;
        var_t1_dn8 = assign62130_e96522_d_n8;
        var_t1_dn9 = assign62130_e96522_d_n9;
        var_t1_dn10 = assign62130_e96522_d_n10;
        var_t1_dn13 = assign62130_e96522_d_n13;
        var_t1_rv = 0.0;

        let assign62140_e96526: f64 = (10.0 * 2.220446049250313e-16);
        let assign62140_e96527: f64 = (1.0 - assign62140_e96526);
        let assign62140_e96534: f64 = (10.0 * 2.220446049250313e-16);
        let assign62140_e96535: f64 = (1.0 + assign62140_e96534);
        let assign62140_e96537: f64 = if ((assign62140_e96527 <= p.p178) && (p.p178 <= assign62140_e96535)) { 1.0 } else { 0.0 };
        var_guard1491 = assign62140_e96537;
        var_guard1491_rv = 0.0;

        let (assign62150_e96546, assign62150_e96546_d_n0, assign62150_e96546_d_n2, assign62150_e96546_d_n4, assign62150_e96546_d_n5, assign62150_e96546_d_n6, assign62150_e96546_d_n7, assign62150_e96546_d_n8, assign62150_e96546_d_n9, assign62150_e96546_d_n10, assign62150_e96546_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1491 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn13,)
    }
};
        var_t3 = assign62150_e96546;
        var_t3_dn0 = assign62150_e96546_d_n0;
        var_t3_dn2 = assign62150_e96546_d_n2;
        var_t3_dn4 = assign62150_e96546_d_n4;
        var_t3_dn5 = assign62150_e96546_d_n5;
        var_t3_dn6 = assign62150_e96546_d_n6;
        var_t3_dn7 = assign62150_e96546_d_n7;
        var_t3_dn8 = assign62150_e96546_d_n8;
        var_t3_dn9 = assign62150_e96546_d_n9;
        var_t3_dn10 = assign62150_e96546_d_n10;
        var_t3_dn13 = assign62150_e96546_d_n13;
        var_t3_rv = 0.0;

        let assign62160_e96550: f64 = (10.0 * 2.220446049250313e-16);
        let assign62160_e96551: f64 = (2.0 - assign62160_e96550);
        let assign62160_e96558: f64 = (10.0 * 2.220446049250313e-16);
        let assign62160_e96559: f64 = (2.0 + assign62160_e96558);
        let assign62160_e96561: f64 = if ((assign62160_e96551 <= p.p178) && (p.p178 <= assign62160_e96559)) { 1.0 } else { 0.0 };
        var_guard1492 = assign62160_e96561;
        var_guard1492_rv = 0.0;

        let (assign62170_e96573, assign62170_e96573_d_n0, assign62170_e96573_d_n2, assign62170_e96573_d_n4, assign62170_e96573_d_n5, assign62170_e96573_d_n6, assign62170_e96573_d_n7, assign62170_e96573_d_n8, assign62170_e96573_d_n9, assign62170_e96573_d_n10, assign62170_e96573_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1491 == 0.0)) && (var_guard1492 != 0.0)) {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn13,)
    }
};
        var_t3 = assign62170_e96573;
        var_t3_dn0 = assign62170_e96573_d_n0;
        var_t3_dn2 = assign62170_e96573_d_n2;
        var_t3_dn4 = assign62170_e96573_d_n4;
        var_t3_dn5 = assign62170_e96573_d_n5;
        var_t3_dn6 = assign62170_e96573_d_n6;
        var_t3_dn7 = assign62170_e96573_d_n7;
        var_t3_dn8 = assign62170_e96573_d_n8;
        var_t3_dn9 = assign62170_e96573_d_n9;
        var_t3_dn10 = assign62170_e96573_d_n10;
        var_t3_dn13 = assign62170_e96573_d_n13;
        var_t3_rv = 0.0;

        let (assign62180_e96595, assign62180_e96595_d_n0, assign62180_e96595_d_n2, assign62180_e96595_d_n4, assign62180_e96595_d_n5, assign62180_e96595_d_n6, assign62180_e96595_d_n7, assign62180_e96595_d_n8, assign62180_e96595_d_n9, assign62180_e96595_d_n10, assign62180_e96595_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1491 == 0.0)) && (var_guard1492 == 0.0)) {
        let (assign62180_e96593, assign62180_e96593_d_n0, assign62180_e96593_d_n2, assign62180_e96593_d_n4, assign62180_e96593_d_n5, assign62180_e96593_d_n6, assign62180_e96593_d_n7, assign62180_e96593_d_n8, assign62180_e96593_d_n9, assign62180_e96593_d_n10, assign62180_e96593_d_n13,) = {
            if (var_t1 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign62180_e96591: f64 = (p.p178 - 1.0);
                let assign62180_e96592: f64 = (var_t1).powf(assign62180_e96591);
                (assign62180_e96592, if 0.0 == 0.0 && ((assign62180_e96591) as f64).is_finite() && ((assign62180_e96591) as f64).fract() == 0.0 { if assign62180_e96591 == 0.0 { 0.0 } else { (assign62180_e96591 * ((var_t1).powf(assign62180_e96591 - 1.0) * var_t1_dn0)) } } else { (assign62180_e96592 * (assign62180_e96591 * (var_t1_dn0 / var_t1))) }, if 0.0 == 0.0 && ((assign62180_e96591) as f64).is_finite() && ((assign62180_e96591) as f64).fract() == 0.0 { if assign62180_e96591 == 0.0 { 0.0 } else { (assign62180_e96591 * ((var_t1).powf(assign62180_e96591 - 1.0) * var_t1_dn2)) } } else { (assign62180_e96592 * (assign62180_e96591 * (var_t1_dn2 / var_t1))) }, if 0.0 == 0.0 && ((assign62180_e96591) as f64).is_finite() && ((assign62180_e96591) as f64).fract() == 0.0 { if assign62180_e96591 == 0.0 { 0.0 } else { (assign62180_e96591 * ((var_t1).powf(assign62180_e96591 - 1.0) * var_t1_dn4)) } } else { (assign62180_e96592 * (assign62180_e96591 * (var_t1_dn4 / var_t1))) }, if 0.0 == 0.0 && ((assign62180_e96591) as f64).is_finite() && ((assign62180_e96591) as f64).fract() == 0.0 { if assign62180_e96591 == 0.0 { 0.0 } else { (assign62180_e96591 * ((var_t1).powf(assign62180_e96591 - 1.0) * var_t1_dn5)) } } else { (assign62180_e96592 * (assign62180_e96591 * (var_t1_dn5 / var_t1))) }, if 0.0 == 0.0 && ((assign62180_e96591) as f64).is_finite() && ((assign62180_e96591) as f64).fract() == 0.0 { if assign62180_e96591 == 0.0 { 0.0 } else { (assign62180_e96591 * ((var_t1).powf(assign62180_e96591 - 1.0) * var_t1_dn6)) } } else { (assign62180_e96592 * (assign62180_e96591 * (var_t1_dn6 / var_t1))) }, if 0.0 == 0.0 && ((assign62180_e96591) as f64).is_finite() && ((assign62180_e96591) as f64).fract() == 0.0 { if assign62180_e96591 == 0.0 { 0.0 } else { (assign62180_e96591 * ((var_t1).powf(assign62180_e96591 - 1.0) * var_t1_dn7)) } } else { (assign62180_e96592 * (assign62180_e96591 * (var_t1_dn7 / var_t1))) }, if 0.0 == 0.0 && ((assign62180_e96591) as f64).is_finite() && ((assign62180_e96591) as f64).fract() == 0.0 { if assign62180_e96591 == 0.0 { 0.0 } else { (assign62180_e96591 * ((var_t1).powf(assign62180_e96591 - 1.0) * var_t1_dn8)) } } else { (assign62180_e96592 * (assign62180_e96591 * (var_t1_dn8 / var_t1))) }, if 0.0 == 0.0 && ((assign62180_e96591) as f64).is_finite() && ((assign62180_e96591) as f64).fract() == 0.0 { if assign62180_e96591 == 0.0 { 0.0 } else { (assign62180_e96591 * ((var_t1).powf(assign62180_e96591 - 1.0) * var_t1_dn9)) } } else { (assign62180_e96592 * (assign62180_e96591 * (var_t1_dn9 / var_t1))) }, if 0.0 == 0.0 && ((assign62180_e96591) as f64).is_finite() && ((assign62180_e96591) as f64).fract() == 0.0 { if assign62180_e96591 == 0.0 { 0.0 } else { (assign62180_e96591 * ((var_t1).powf(assign62180_e96591 - 1.0) * var_t1_dn10)) } } else { (assign62180_e96592 * (assign62180_e96591 * (var_t1_dn10 / var_t1))) }, if 0.0 == 0.0 && ((assign62180_e96591) as f64).is_finite() && ((assign62180_e96591) as f64).fract() == 0.0 { if assign62180_e96591 == 0.0 { 0.0 } else { (assign62180_e96591 * ((var_t1).powf(assign62180_e96591 - 1.0) * var_t1_dn13)) } } else { (assign62180_e96592 * (assign62180_e96591 * (var_t1_dn13 / var_t1))) },)
            }
        };
        (assign62180_e96593, assign62180_e96593_d_n0, assign62180_e96593_d_n2, assign62180_e96593_d_n4, assign62180_e96593_d_n5, assign62180_e96593_d_n6, assign62180_e96593_d_n7, assign62180_e96593_d_n8, assign62180_e96593_d_n9, assign62180_e96593_d_n10, assign62180_e96593_d_n13,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn13,)
    }
};
        var_t3 = assign62180_e96595;
        var_t3_dn0 = assign62180_e96595_d_n0;
        var_t3_dn2 = assign62180_e96595_d_n2;
        var_t3_dn4 = assign62180_e96595_d_n4;
        var_t3_dn5 = assign62180_e96595_d_n5;
        var_t3_dn6 = assign62180_e96595_d_n6;
        var_t3_dn7 = assign62180_e96595_d_n7;
        var_t3_dn8 = assign62180_e96595_d_n8;
        var_t3_dn9 = assign62180_e96595_d_n9;
        var_t3_dn10 = assign62180_e96595_d_n10;
        var_t3_dn13 = assign62180_e96595_d_n13;
        var_t3_rv = 0.0;

        let (assign62190_e96604, assign62190_e96604_d_n0, assign62190_e96604_d_n2, assign62190_e96604_d_n4, assign62190_e96604_d_n5, assign62190_e96604_d_n6, assign62190_e96604_d_n7, assign62190_e96604_d_n8, assign62190_e96604_d_n9, assign62190_e96604_d_n10, assign62190_e96604_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign62190_e96602: f64 = (var_t1 * var_t3);
        (assign62190_e96602, ((var_t1_dn0 * var_t3) + (var_t1 * var_t3_dn0)), ((var_t1_dn2 * var_t3) + (var_t1 * var_t3_dn2)), ((var_t1_dn4 * var_t3) + (var_t1 * var_t3_dn4)), ((var_t1_dn5 * var_t3) + (var_t1 * var_t3_dn5)), ((var_t1_dn6 * var_t3) + (var_t1 * var_t3_dn6)), ((var_t1_dn7 * var_t3) + (var_t1 * var_t3_dn7)), ((var_t1_dn8 * var_t3) + (var_t1 * var_t3_dn8)), ((var_t1_dn9 * var_t3) + (var_t1 * var_t3_dn9)), ((var_t1_dn10 * var_t3) + (var_t1 * var_t3_dn10)), ((var_t1_dn13 * var_t3) + (var_t1 * var_t3_dn13)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign62190_e96604;
        var_t2_dn0 = assign62190_e96604_d_n0;
        var_t2_dn2 = assign62190_e96604_d_n2;
        var_t2_dn4 = assign62190_e96604_d_n4;
        var_t2_dn5 = assign62190_e96604_d_n5;
        var_t2_dn6 = assign62190_e96604_d_n6;
        var_t2_dn7 = assign62190_e96604_d_n7;
        var_t2_dn8 = assign62190_e96604_d_n8;
        var_t2_dn9 = assign62190_e96604_d_n9;
        var_t2_dn10 = assign62190_e96604_d_n10;
        var_t2_dn13 = assign62190_e96604_d_n13;
        var_t2_rv = 0.0;

        let (assign62200_e96613, assign62200_e96613_d_n0, assign62200_e96613_d_n2, assign62200_e96613_d_n4, assign62200_e96613_d_n5, assign62200_e96613_d_n6, assign62200_e96613_d_n7, assign62200_e96613_d_n8, assign62200_e96613_d_n9, assign62200_e96613_d_n10, assign62200_e96613_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign62200_e96611: f64 = (1.0 + var_t2);
        (assign62200_e96611, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn13,)
    }
};
        var_t4 = assign62200_e96613;
        var_t4_dn0 = assign62200_e96613_d_n0;
        var_t4_dn2 = assign62200_e96613_d_n2;
        var_t4_dn4 = assign62200_e96613_d_n4;
        var_t4_dn5 = assign62200_e96613_d_n5;
        var_t4_dn6 = assign62200_e96613_d_n6;
        var_t4_dn7 = assign62200_e96613_d_n7;
        var_t4_dn8 = assign62200_e96613_d_n8;
        var_t4_dn9 = assign62200_e96613_d_n9;
        var_t4_dn10 = assign62200_e96613_d_n10;
        var_t4_dn13 = assign62200_e96613_d_n13;
        var_t4_rv = 0.0;

        let assign62210_e96617: f64 = (10.0 * 2.220446049250313e-16);
        let assign62210_e96618: f64 = (1.0 - assign62210_e96617);
        let assign62210_e96625: f64 = (10.0 * 2.220446049250313e-16);
        let assign62210_e96626: f64 = (1.0 + assign62210_e96625);
        let assign62210_e96628: f64 = if ((assign62210_e96618 <= p.p178) && (p.p178 <= assign62210_e96626)) { 1.0 } else { 0.0 };
        var_guard1493 = assign62210_e96628;
        var_guard1493_rv = 0.0;

        let (assign62220_e96639, assign62220_e96639_d_n0, assign62220_e96639_d_n2, assign62220_e96639_d_n4, assign62220_e96639_d_n5, assign62220_e96639_d_n6, assign62220_e96639_d_n7, assign62220_e96639_d_n8, assign62220_e96639_d_n9, assign62220_e96639_d_n10, assign62220_e96639_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1493 != 0.0)) {
        let assign62220_e96637: f64 = (1.0 / var_t4);
        (assign62220_e96637, (-(var_t4_dn0 / (var_t4 * var_t4))), (-(var_t4_dn2 / (var_t4 * var_t4))), (-(var_t4_dn4 / (var_t4 * var_t4))), (-(var_t4_dn5 / (var_t4 * var_t4))), (-(var_t4_dn6 / (var_t4 * var_t4))), (-(var_t4_dn7 / (var_t4 * var_t4))), (-(var_t4_dn8 / (var_t4 * var_t4))), (-(var_t4_dn9 / (var_t4 * var_t4))), (-(var_t4_dn10 / (var_t4 * var_t4))), (-(var_t4_dn13 / (var_t4 * var_t4))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn13,)
    }
};
        var_t5 = assign62220_e96639;
        var_t5_dn0 = assign62220_e96639_d_n0;
        var_t5_dn2 = assign62220_e96639_d_n2;
        var_t5_dn4 = assign62220_e96639_d_n4;
        var_t5_dn5 = assign62220_e96639_d_n5;
        var_t5_dn6 = assign62220_e96639_d_n6;
        var_t5_dn7 = assign62220_e96639_d_n7;
        var_t5_dn8 = assign62220_e96639_d_n8;
        var_t5_dn9 = assign62220_e96639_d_n9;
        var_t5_dn10 = assign62220_e96639_d_n10;
        var_t5_dn13 = assign62220_e96639_d_n13;
        var_t5_rv = 0.0;

        let assign62230_e96643: f64 = (10.0 * 2.220446049250313e-16);
        let assign62230_e96644: f64 = (2.0 - assign62230_e96643);
        let assign62230_e96651: f64 = (10.0 * 2.220446049250313e-16);
        let assign62230_e96652: f64 = (2.0 + assign62230_e96651);
        let assign62230_e96654: f64 = if ((assign62230_e96644 <= p.p178) && (p.p178 <= assign62230_e96652)) { 1.0 } else { 0.0 };
        var_guard1494 = assign62230_e96654;
        var_guard1494_rv = 0.0;

        let (assign62240_e96669, assign62240_e96669_d_n0, assign62240_e96669_d_n2, assign62240_e96669_d_n4, assign62240_e96669_d_n5, assign62240_e96669_d_n6, assign62240_e96669_d_n7, assign62240_e96669_d_n8, assign62240_e96669_d_n9, assign62240_e96669_d_n10, assign62240_e96669_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1493 == 0.0)) && (var_guard1494 != 0.0)) {
        let assign62240_e96666: f64 = (var_t4).sqrt();
        let assign62240_e96667: f64 = (1.0 / assign62240_e96666);
        (assign62240_e96667, (-((var_t4_dn0 / (2.0 * assign62240_e96666)) / (assign62240_e96666 * assign62240_e96666))), (-((var_t4_dn2 / (2.0 * assign62240_e96666)) / (assign62240_e96666 * assign62240_e96666))), (-((var_t4_dn4 / (2.0 * assign62240_e96666)) / (assign62240_e96666 * assign62240_e96666))), (-((var_t4_dn5 / (2.0 * assign62240_e96666)) / (assign62240_e96666 * assign62240_e96666))), (-((var_t4_dn6 / (2.0 * assign62240_e96666)) / (assign62240_e96666 * assign62240_e96666))), (-((var_t4_dn7 / (2.0 * assign62240_e96666)) / (assign62240_e96666 * assign62240_e96666))), (-((var_t4_dn8 / (2.0 * assign62240_e96666)) / (assign62240_e96666 * assign62240_e96666))), (-((var_t4_dn9 / (2.0 * assign62240_e96666)) / (assign62240_e96666 * assign62240_e96666))), (-((var_t4_dn10 / (2.0 * assign62240_e96666)) / (assign62240_e96666 * assign62240_e96666))), (-((var_t4_dn13 / (2.0 * assign62240_e96666)) / (assign62240_e96666 * assign62240_e96666))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn13,)
    }
};
        var_t5 = assign62240_e96669;
        var_t5_dn0 = assign62240_e96669_d_n0;
        var_t5_dn2 = assign62240_e96669_d_n2;
        var_t5_dn4 = assign62240_e96669_d_n4;
        var_t5_dn5 = assign62240_e96669_d_n5;
        var_t5_dn6 = assign62240_e96669_d_n6;
        var_t5_dn7 = assign62240_e96669_d_n7;
        var_t5_dn8 = assign62240_e96669_d_n8;
        var_t5_dn9 = assign62240_e96669_d_n9;
        var_t5_dn10 = assign62240_e96669_d_n10;
        var_t5_dn13 = assign62240_e96669_d_n13;
        var_t5_rv = 0.0;

        let (assign62250_e96694, assign62250_e96694_d_n0, assign62250_e96694_d_n2, assign62250_e96694_d_n4, assign62250_e96694_d_n5, assign62250_e96694_d_n6, assign62250_e96694_d_n7, assign62250_e96694_d_n8, assign62250_e96694_d_n9, assign62250_e96694_d_n10, assign62250_e96694_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1493 == 0.0)) && (var_guard1494 == 0.0)) {
        let (assign62250_e96692, assign62250_e96692_d_n0, assign62250_e96692_d_n2, assign62250_e96692_d_n4, assign62250_e96692_d_n5, assign62250_e96692_d_n6, assign62250_e96692_d_n7, assign62250_e96692_d_n8, assign62250_e96692_d_n9, assign62250_e96692_d_n10, assign62250_e96692_d_n13,) = {
            if (var_t4 == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign62250_e96686: f64 = (-1.0);
                let assign62250_e96688: f64 = (assign62250_e96686 / p.p178);
                let assign62250_e96690: f64 = (assign62250_e96688 - 1.0);
                let assign62250_e96691: f64 = (var_t4).powf(assign62250_e96690);
                (assign62250_e96691, if 0.0 == 0.0 && ((assign62250_e96690) as f64).is_finite() && ((assign62250_e96690) as f64).fract() == 0.0 { if assign62250_e96690 == 0.0 { 0.0 } else { (assign62250_e96690 * ((var_t4).powf(assign62250_e96690 - 1.0) * var_t4_dn0)) } } else { (assign62250_e96691 * (assign62250_e96690 * (var_t4_dn0 / var_t4))) }, if 0.0 == 0.0 && ((assign62250_e96690) as f64).is_finite() && ((assign62250_e96690) as f64).fract() == 0.0 { if assign62250_e96690 == 0.0 { 0.0 } else { (assign62250_e96690 * ((var_t4).powf(assign62250_e96690 - 1.0) * var_t4_dn2)) } } else { (assign62250_e96691 * (assign62250_e96690 * (var_t4_dn2 / var_t4))) }, if 0.0 == 0.0 && ((assign62250_e96690) as f64).is_finite() && ((assign62250_e96690) as f64).fract() == 0.0 { if assign62250_e96690 == 0.0 { 0.0 } else { (assign62250_e96690 * ((var_t4).powf(assign62250_e96690 - 1.0) * var_t4_dn4)) } } else { (assign62250_e96691 * (assign62250_e96690 * (var_t4_dn4 / var_t4))) }, if 0.0 == 0.0 && ((assign62250_e96690) as f64).is_finite() && ((assign62250_e96690) as f64).fract() == 0.0 { if assign62250_e96690 == 0.0 { 0.0 } else { (assign62250_e96690 * ((var_t4).powf(assign62250_e96690 - 1.0) * var_t4_dn5)) } } else { (assign62250_e96691 * (assign62250_e96690 * (var_t4_dn5 / var_t4))) }, if 0.0 == 0.0 && ((assign62250_e96690) as f64).is_finite() && ((assign62250_e96690) as f64).fract() == 0.0 { if assign62250_e96690 == 0.0 { 0.0 } else { (assign62250_e96690 * ((var_t4).powf(assign62250_e96690 - 1.0) * var_t4_dn6)) } } else { (assign62250_e96691 * (assign62250_e96690 * (var_t4_dn6 / var_t4))) }, if 0.0 == 0.0 && ((assign62250_e96690) as f64).is_finite() && ((assign62250_e96690) as f64).fract() == 0.0 { if assign62250_e96690 == 0.0 { 0.0 } else { (assign62250_e96690 * ((var_t4).powf(assign62250_e96690 - 1.0) * var_t4_dn7)) } } else { (assign62250_e96691 * (assign62250_e96690 * (var_t4_dn7 / var_t4))) }, if 0.0 == 0.0 && ((assign62250_e96690) as f64).is_finite() && ((assign62250_e96690) as f64).fract() == 0.0 { if assign62250_e96690 == 0.0 { 0.0 } else { (assign62250_e96690 * ((var_t4).powf(assign62250_e96690 - 1.0) * var_t4_dn8)) } } else { (assign62250_e96691 * (assign62250_e96690 * (var_t4_dn8 / var_t4))) }, if 0.0 == 0.0 && ((assign62250_e96690) as f64).is_finite() && ((assign62250_e96690) as f64).fract() == 0.0 { if assign62250_e96690 == 0.0 { 0.0 } else { (assign62250_e96690 * ((var_t4).powf(assign62250_e96690 - 1.0) * var_t4_dn9)) } } else { (assign62250_e96691 * (assign62250_e96690 * (var_t4_dn9 / var_t4))) }, if 0.0 == 0.0 && ((assign62250_e96690) as f64).is_finite() && ((assign62250_e96690) as f64).fract() == 0.0 { if assign62250_e96690 == 0.0 { 0.0 } else { (assign62250_e96690 * ((var_t4).powf(assign62250_e96690 - 1.0) * var_t4_dn10)) } } else { (assign62250_e96691 * (assign62250_e96690 * (var_t4_dn10 / var_t4))) }, if 0.0 == 0.0 && ((assign62250_e96690) as f64).is_finite() && ((assign62250_e96690) as f64).fract() == 0.0 { if assign62250_e96690 == 0.0 { 0.0 } else { (assign62250_e96690 * ((var_t4).powf(assign62250_e96690 - 1.0) * var_t4_dn13)) } } else { (assign62250_e96691 * (assign62250_e96690 * (var_t4_dn13 / var_t4))) },)
            }
        };
        (assign62250_e96692, assign62250_e96692_d_n0, assign62250_e96692_d_n2, assign62250_e96692_d_n4, assign62250_e96692_d_n5, assign62250_e96692_d_n6, assign62250_e96692_d_n7, assign62250_e96692_d_n8, assign62250_e96692_d_n9, assign62250_e96692_d_n10, assign62250_e96692_d_n13,)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn13,)
    }
};
        var_t6 = assign62250_e96694;
        var_t6_dn0 = assign62250_e96694_d_n0;
        var_t6_dn2 = assign62250_e96694_d_n2;
        var_t6_dn4 = assign62250_e96694_d_n4;
        var_t6_dn5 = assign62250_e96694_d_n5;
        var_t6_dn6 = assign62250_e96694_d_n6;
        var_t6_dn7 = assign62250_e96694_d_n7;
        var_t6_dn8 = assign62250_e96694_d_n8;
        var_t6_dn9 = assign62250_e96694_d_n9;
        var_t6_dn10 = assign62250_e96694_d_n10;
        var_t6_dn13 = assign62250_e96694_d_n13;
        var_t6_rv = 0.0;

        let (assign62260_e96709, assign62260_e96709_d_n0, assign62260_e96709_d_n2, assign62260_e96709_d_n4, assign62260_e96709_d_n5, assign62260_e96709_d_n6, assign62260_e96709_d_n7, assign62260_e96709_d_n8, assign62260_e96709_d_n9, assign62260_e96709_d_n10, assign62260_e96709_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1493 == 0.0)) && (var_guard1494 == 0.0)) {
        let assign62260_e96707: f64 = (var_t4 * var_t6);
        (assign62260_e96707, ((var_t4_dn0 * var_t6) + (var_t4 * var_t6_dn0)), ((var_t4_dn2 * var_t6) + (var_t4 * var_t6_dn2)), ((var_t4_dn4 * var_t6) + (var_t4 * var_t6_dn4)), ((var_t4_dn5 * var_t6) + (var_t4 * var_t6_dn5)), ((var_t4_dn6 * var_t6) + (var_t4 * var_t6_dn6)), ((var_t4_dn7 * var_t6) + (var_t4 * var_t6_dn7)), ((var_t4_dn8 * var_t6) + (var_t4 * var_t6_dn8)), ((var_t4_dn9 * var_t6) + (var_t4 * var_t6_dn9)), ((var_t4_dn10 * var_t6) + (var_t4 * var_t6_dn10)), ((var_t4_dn13 * var_t6) + (var_t4 * var_t6_dn13)),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn13,)
    }
};
        var_t5 = assign62260_e96709;
        var_t5_dn0 = assign62260_e96709_d_n0;
        var_t5_dn2 = assign62260_e96709_d_n2;
        var_t5_dn4 = assign62260_e96709_d_n4;
        var_t5_dn5 = assign62260_e96709_d_n5;
        var_t5_dn6 = assign62260_e96709_d_n6;
        var_t5_dn7 = assign62260_e96709_d_n7;
        var_t5_dn8 = assign62260_e96709_d_n8;
        var_t5_dn9 = assign62260_e96709_d_n9;
        var_t5_dn10 = assign62260_e96709_d_n10;
        var_t5_dn13 = assign62260_e96709_d_n13;
        var_t5_rv = 0.0;

        let (assign62270_e96718, assign62270_e96718_d_n0, assign62270_e96718_d_n2, assign62270_e96718_d_n4, assign62270_e96718_d_n5, assign62270_e96718_d_n6, assign62270_e96718_d_n7, assign62270_e96718_d_n8, assign62270_e96718_d_n9, assign62270_e96718_d_n10, assign62270_e96718_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign62270_e96716: f64 = (var_muun * var_t5);
        (assign62270_e96716, ((var_muun_dn0 * var_t5) + (var_muun * var_t5_dn0)), ((var_muun_dn2 * var_t5) + (var_muun * var_t5_dn2)), ((var_muun_dn4 * var_t5) + (var_muun * var_t5_dn4)), ((var_muun_dn5 * var_t5) + (var_muun * var_t5_dn5)), ((var_muun_dn6 * var_t5) + (var_muun * var_t5_dn6)), ((var_muun_dn7 * var_t5) + (var_muun * var_t5_dn7)), ((var_muun_dn8 * var_t5) + (var_muun * var_t5_dn8)), ((var_muun_dn9 * var_t5) + (var_muun * var_t5_dn9)), ((var_muun_dn10 * var_t5) + (var_muun * var_t5_dn10)), ((var_muun_dn13 * var_t5) + (var_muun * var_t5_dn13)),)
    } else {
        (var_mu, var_mu_dn0, var_mu_dn2, var_mu_dn4, var_mu_dn5, var_mu_dn6, var_mu_dn7, var_mu_dn8, var_mu_dn9, var_mu_dn10, var_mu_dn13,)
    }
};
        var_mu = assign62270_e96718;
        var_mu_dn0 = assign62270_e96718_d_n0;
        var_mu_dn2 = assign62270_e96718_d_n2;
        var_mu_dn4 = assign62270_e96718_d_n4;
        var_mu_dn5 = assign62270_e96718_d_n5;
        var_mu_dn6 = assign62270_e96718_d_n6;
        var_mu_dn7 = assign62270_e96718_d_n7;
        var_mu_dn8 = assign62270_e96718_d_n8;
        var_mu_dn9 = assign62270_e96718_d_n9;
        var_mu_dn10 = assign62270_e96718_d_n10;
        var_mu_dn13 = assign62270_e96718_d_n13;
        var_mu_rv = 0.0;

        let (assign62280_e96729, assign62280_e96729_d_n0, assign62280_e96729_d_n2, assign62280_e96729_d_n4, assign62280_e96729_d_n5, assign62280_e96729_d_n6, assign62280_e96729_d_n7, assign62280_e96729_d_n8, assign62280_e96729_d_n9, assign62280_e96729_d_n10, assign62280_e96729_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign62280_e96725: f64 = (var_weff_nf * var_beta_inv);
        let assign62280_e96727: f64 = (assign62280_e96725 / var_lch);
        (assign62280_e96727, ((((var_weff_nf * var_beta_inv_dn0) * var_lch) - (assign62280_e96725 * var_lch_dn0)) / (var_lch * var_lch)), ((((var_weff_nf * var_beta_inv_dn2) * var_lch) - (assign62280_e96725 * var_lch_dn2)) / (var_lch * var_lch)), ((((var_weff_nf * var_beta_inv_dn4) * var_lch) - (assign62280_e96725 * var_lch_dn4)) / (var_lch * var_lch)), ((((var_weff_nf * var_beta_inv_dn5) * var_lch) - (assign62280_e96725 * var_lch_dn5)) / (var_lch * var_lch)), ((((var_weff_nf * var_beta_inv_dn6) * var_lch) - (assign62280_e96725 * var_lch_dn6)) / (var_lch * var_lch)), ((((var_weff_nf * var_beta_inv_dn7) * var_lch) - (assign62280_e96725 * var_lch_dn7)) / (var_lch * var_lch)), ((((var_weff_nf * var_beta_inv_dn8) * var_lch) - (assign62280_e96725 * var_lch_dn8)) / (var_lch * var_lch)), ((((var_weff_nf * var_beta_inv_dn9) * var_lch) - (assign62280_e96725 * var_lch_dn9)) / (var_lch * var_lch)), ((((var_weff_nf * var_beta_inv_dn10) * var_lch) - (assign62280_e96725 * var_lch_dn10)) / (var_lch * var_lch)), ((((var_weff_nf * var_beta_inv_dn13) * var_lch) - (assign62280_e96725 * var_lch_dn13)) / (var_lch * var_lch)),)
    } else {
        (var_betawl, var_betawl_dn0, var_betawl_dn2, var_betawl_dn4, var_betawl_dn5, var_betawl_dn6, var_betawl_dn7, var_betawl_dn8, var_betawl_dn9, var_betawl_dn10, var_betawl_dn13,)
    }
};
        var_betawl = assign62280_e96729;
        var_betawl_dn0 = assign62280_e96729_d_n0;
        var_betawl_dn2 = assign62280_e96729_d_n2;
        var_betawl_dn4 = assign62280_e96729_d_n4;
        var_betawl_dn5 = assign62280_e96729_d_n5;
        var_betawl_dn6 = assign62280_e96729_d_n6;
        var_betawl_dn7 = assign62280_e96729_d_n7;
        var_betawl_dn8 = assign62280_e96729_d_n8;
        var_betawl_dn9 = assign62280_e96729_d_n9;
        var_betawl_dn10 = assign62280_e96729_d_n10;
        var_betawl_dn13 = assign62280_e96729_d_n13;
        var_betawl_rv = 0.0;

        let (assign62290_e96739, assign62290_e96739_d_n0, assign62290_e96739_d_n2, assign62290_e96739_d_n4, assign62290_e96739_d_n5, assign62290_e96739_d_n6, assign62290_e96739_d_n7, assign62290_e96739_d_n8, assign62290_e96739_d_n9, assign62290_e96739_d_n10, assign62290_e96739_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign62290_e96735: f64 = (-var_betawl);
        let assign62290_e96737: f64 = (assign62290_e96735 / var_lch);
        (assign62290_e96737, ((((-var_betawl_dn0) * var_lch) - (assign62290_e96735 * var_lch_dn0)) / (var_lch * var_lch)), ((((-var_betawl_dn2) * var_lch) - (assign62290_e96735 * var_lch_dn2)) / (var_lch * var_lch)), ((((-var_betawl_dn4) * var_lch) - (assign62290_e96735 * var_lch_dn4)) / (var_lch * var_lch)), ((((-var_betawl_dn5) * var_lch) - (assign62290_e96735 * var_lch_dn5)) / (var_lch * var_lch)), ((((-var_betawl_dn6) * var_lch) - (assign62290_e96735 * var_lch_dn6)) / (var_lch * var_lch)), ((((-var_betawl_dn7) * var_lch) - (assign62290_e96735 * var_lch_dn7)) / (var_lch * var_lch)), ((((-var_betawl_dn8) * var_lch) - (assign62290_e96735 * var_lch_dn8)) / (var_lch * var_lch)), ((((-var_betawl_dn9) * var_lch) - (assign62290_e96735 * var_lch_dn9)) / (var_lch * var_lch)), ((((-var_betawl_dn10) * var_lch) - (assign62290_e96735 * var_lch_dn10)) / (var_lch * var_lch)), ((((-var_betawl_dn13) * var_lch) - (assign62290_e96735 * var_lch_dn13)) / (var_lch * var_lch)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign62290_e96739;
        var_t1_dn0 = assign62290_e96739_d_n0;
        var_t1_dn2 = assign62290_e96739_d_n2;
        var_t1_dn4 = assign62290_e96739_d_n4;
        var_t1_dn5 = assign62290_e96739_d_n5;
        var_t1_dn6 = assign62290_e96739_d_n6;
        var_t1_dn7 = assign62290_e96739_d_n7;
        var_t1_dn8 = assign62290_e96739_d_n8;
        var_t1_dn9 = assign62290_e96739_d_n9;
        var_t1_dn10 = assign62290_e96739_d_n10;
        var_t1_dn13 = assign62290_e96739_d_n13;
        var_t1_rv = 0.0;

        let (assign62300_e96750, assign62300_e96750_d_n0, assign62300_e96750_d_n2, assign62300_e96750_d_n4, assign62300_e96750_d_n5, assign62300_e96750_d_n6, assign62300_e96750_d_n7, assign62300_e96750_d_n8, assign62300_e96750_d_n9, assign62300_e96750_d_n10, assign62300_e96750_d_n13,) = {
    if ((var_guard443 == 0.0) && (var_guard1430 != 0.0)) {
        let assign62300_e96746: f64 = (var_betawl * var_idd);
        let assign62300_e96748: f64 = (assign62300_e96746 * var_mu);
        (assign62300_e96748, ((((var_betawl_dn0 * var_idd) + (var_betawl * var_idd_dn0)) * var_mu) + (assign62300_e96746 * var_mu_dn0)), ((((var_betawl_dn2 * var_idd) + (var_betawl * var_idd_dn2)) * var_mu) + (assign62300_e96746 * var_mu_dn2)), ((((var_betawl_dn4 * var_idd) + (var_betawl * var_idd_dn4)) * var_mu) + (assign62300_e96746 * var_mu_dn4)), ((((var_betawl_dn5 * var_idd) + (var_betawl * var_idd_dn5)) * var_mu) + (assign62300_e96746 * var_mu_dn5)), ((((var_betawl_dn6 * var_idd) + (var_betawl * var_idd_dn6)) * var_mu) + (assign62300_e96746 * var_mu_dn6)), ((((var_betawl_dn7 * var_idd) + (var_betawl * var_idd_dn7)) * var_mu) + (assign62300_e96746 * var_mu_dn7)), ((((var_betawl_dn8 * var_idd) + (var_betawl * var_idd_dn8)) * var_mu) + (assign62300_e96746 * var_mu_dn8)), ((((var_betawl_dn9 * var_idd) + (var_betawl * var_idd_dn9)) * var_mu) + (assign62300_e96746 * var_mu_dn9)), ((((var_betawl_dn10 * var_idd) + (var_betawl * var_idd_dn10)) * var_mu) + (assign62300_e96746 * var_mu_dn10)), ((((var_betawl_dn13 * var_idd) + (var_betawl * var_idd_dn13)) * var_mu) + (assign62300_e96746 * var_mu_dn13)),)
    } else {
        (var_ids0, var_ids0_dn0, var_ids0_dn2, var_ids0_dn4, var_ids0_dn5, var_ids0_dn6, var_ids0_dn7, var_ids0_dn8, var_ids0_dn9, var_ids0_dn10, var_ids0_dn13,)
    }
};
        var_ids0 = assign62300_e96750;
        var_ids0_dn0 = assign62300_e96750_d_n0;
        var_ids0_dn2 = assign62300_e96750_d_n2;
        var_ids0_dn4 = assign62300_e96750_d_n4;
        var_ids0_dn5 = assign62300_e96750_d_n5;
        var_ids0_dn6 = assign62300_e96750_d_n6;
        var_ids0_dn7 = assign62300_e96750_d_n7;
        var_ids0_dn8 = assign62300_e96750_d_n8;
        var_ids0_dn9 = assign62300_e96750_d_n9;
        var_ids0_dn10 = assign62300_e96750_d_n10;
        var_ids0_dn13 = assign62300_e96750_d_n13;
        var_ids0_rv = 0.0;

        let assign62310_e96753: f64 = if p.p283 != 0.0 { 1.0 } else { 0.0 };
        var_guard1495 = assign62310_e96753;
        var_guard1495_rv = 0.0;

        *var_betawl_slot = var_betawl;
        *var_betawl_dn0_slot = var_betawl_dn0;
        *var_betawl_dn10_slot = var_betawl_dn10;
        *var_betawl_dn13_slot = var_betawl_dn13;
        *var_betawl_dn2_slot = var_betawl_dn2;
        *var_betawl_dn4_slot = var_betawl_dn4;
        *var_betawl_dn5_slot = var_betawl_dn5;
        *var_betawl_dn6_slot = var_betawl_dn6;
        *var_betawl_dn7_slot = var_betawl_dn7;
        *var_betawl_dn8_slot = var_betawl_dn8;
        *var_betawl_dn9_slot = var_betawl_dn9;
        *var_betawl_rv_slot = var_betawl_rv;
        *var_em_slot = var_em;
        *var_em_dn0_slot = var_em_dn0;
        *var_em_dn10_slot = var_em_dn10;
        *var_em_dn13_slot = var_em_dn13;
        *var_em_dn2_slot = var_em_dn2;
        *var_em_dn4_slot = var_em_dn4;
        *var_em_dn5_slot = var_em_dn5;
        *var_em_dn6_slot = var_em_dn6;
        *var_em_dn7_slot = var_em_dn7;
        *var_em_dn8_slot = var_em_dn8;
        *var_em_dn9_slot = var_em_dn9;
        *var_em_rv_slot = var_em_rv;
        *var_ey_slot = var_ey;
        *var_ey_dn0_slot = var_ey_dn0;
        *var_ey_dn10_slot = var_ey_dn10;
        *var_ey_dn13_slot = var_ey_dn13;
        *var_ey_dn2_slot = var_ey_dn2;
        *var_ey_dn4_slot = var_ey_dn4;
        *var_ey_dn5_slot = var_ey_dn5;
        *var_ey_dn6_slot = var_ey_dn6;
        *var_ey_dn7_slot = var_ey_dn7;
        *var_ey_dn8_slot = var_ey_dn8;
        *var_ey_dn9_slot = var_ey_dn9;
        *var_ey_rv_slot = var_ey_rv;
        *var_guard1491_slot = var_guard1491;
        *var_guard1491_rv_slot = var_guard1491_rv;
        *var_guard1492_slot = var_guard1492;
        *var_guard1492_rv_slot = var_guard1492_rv;
        *var_guard1493_slot = var_guard1493;
        *var_guard1493_rv_slot = var_guard1493_rv;
        *var_guard1494_slot = var_guard1494;
        *var_guard1494_rv_slot = var_guard1494_rv;
        *var_guard1495_slot = var_guard1495;
        *var_guard1495_rv_slot = var_guard1495_rv;
        *var_ids0_slot = var_ids0;
        *var_ids0_dn0_slot = var_ids0_dn0;
        *var_ids0_dn10_slot = var_ids0_dn10;
        *var_ids0_dn13_slot = var_ids0_dn13;
        *var_ids0_dn2_slot = var_ids0_dn2;
        *var_ids0_dn4_slot = var_ids0_dn4;
        *var_ids0_dn5_slot = var_ids0_dn5;
        *var_ids0_dn6_slot = var_ids0_dn6;
        *var_ids0_dn7_slot = var_ids0_dn7;
        *var_ids0_dn8_slot = var_ids0_dn8;
        *var_ids0_dn9_slot = var_ids0_dn9;
        *var_ids0_rv_slot = var_ids0_rv;
        *var_mu_slot = var_mu;
        *var_mu_dn0_slot = var_mu_dn0;
        *var_mu_dn10_slot = var_mu_dn10;
        *var_mu_dn13_slot = var_mu_dn13;
        *var_mu_dn2_slot = var_mu_dn2;
        *var_mu_dn4_slot = var_mu_dn4;
        *var_mu_dn5_slot = var_mu_dn5;
        *var_mu_dn6_slot = var_mu_dn6;
        *var_mu_dn7_slot = var_mu_dn7;
        *var_mu_dn8_slot = var_mu_dn8;
        *var_mu_dn9_slot = var_mu_dn9;
        *var_mu_rv_slot = var_mu_rv;
        *var_t1_slot = var_t1;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn13_slot = var_t1_dn13;
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
        *var_t2_dn13_slot = var_t2_dn13;
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
        *var_t3_dn13_slot = var_t3_dn13;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t3_rv_slot = var_t3_rv;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn13_slot = var_t4_dn13;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_dn9_slot = var_t4_dn9;
        *var_t4_rv_slot = var_t4_rv;
        *var_t5_slot = var_t5;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn10_slot = var_t5_dn10;
        *var_t5_dn13_slot = var_t5_dn13;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t5_dn9_slot = var_t5_dn9;
        *var_t5_rv_slot = var_t5_rv;
        *var_t6_slot = var_t6;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn13_slot = var_t6_dn13;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t6_dn9_slot = var_t6_dn9;
        *var_t6_rv_slot = var_t6_rv;
        *var_ty_slot = var_ty;
        *var_ty_dn0_slot = var_ty_dn0;
        *var_ty_dn10_slot = var_ty_dn10;
        *var_ty_dn13_slot = var_ty_dn13;
        *var_ty_dn2_slot = var_ty_dn2;
        *var_ty_dn4_slot = var_ty_dn4;
        *var_ty_dn5_slot = var_ty_dn5;
        *var_ty_dn6_slot = var_ty_dn6;
        *var_ty_dn7_slot = var_ty_dn7;
        *var_ty_dn8_slot = var_ty_dn8;
        *var_ty_dn9_slot = var_ty_dn9;
        *var_ty_rv_slot = var_ty_rv;
    }

    pub(super) fn stamp_reactive_block_223(
        p: &Parameters,
        var_beta: f64,
        var_beta_dn0: f64,
        var_beta_dn10: f64,
        var_beta_dn13: f64,
        var_beta_dn2: f64,
        var_beta_dn4: f64,
        var_beta_dn5: f64,
        var_beta_dn6: f64,
        var_beta_dn7: f64,
        var_beta_dn8: f64,
        var_beta_dn9: f64,
        var_cox: f64,
        var_cox_dn0: f64,
        var_cox_dn10: f64,
        var_cox_dn13: f64,
        var_cox_dn2: f64,
        var_cox_dn4: f64,
        var_cox_dn5: f64,
        var_cox_dn6: f64,
        var_cox_dn7: f64,
        var_cox_dn8: f64,
        var_cox_dn9: f64,
        var_guard1430: f64,
        var_guard1495: f64,
        var_guard443: f64,
        var_pds: f64,
        var_pds_dn0: f64,
        var_pds_dn10: f64,
        var_pds_dn13: f64,
        var_pds_dn2: f64,
        var_pds_dn4: f64,
        var_pds_dn5: f64,
        var_pds_dn6: f64,
        var_pds_dn7: f64,
        var_pds_dn8: f64,
        var_pds_dn9: f64,
        var_ps0: f64,
        var_ps0_dn0: f64,
        var_ps0_dn10: f64,
        var_ps0_dn13: f64,
        var_ps0_dn2: f64,
        var_ps0_dn4: f64,
        var_ps0_dn5: f64,
        var_ps0_dn6: f64,
        var_ps0_dn7: f64,
        var_ps0_dn8: f64,
        var_ps0_dn9: f64,
        var_pt40: f64,
        var_ptl0: f64,
        var_vbsz__blk438: f64,
        var_vbsz__blk438_dn0: f64,
        var_vbsz__blk438_dn10: f64,
        var_vbsz__blk438_dn13: f64,
        var_vbsz__blk438_dn2: f64,
        var_vbsz__blk438_dn4: f64,
        var_vbsz__blk438_dn5: f64,
        var_vbsz__blk438_dn6: f64,
        var_vbsz__blk438_dn7: f64,
        var_vbsz__blk438_dn8: f64,
        var_vbsz__blk438_dn9: f64,
        var_vds: f64,
        var_vds_dn0: f64,
        var_vds_dn10: f64,
        var_vds_dn13: f64,
        var_vds_dn2: f64,
        var_vds_dn4: f64,
        var_vds_dn5: f64,
        var_vds_dn6: f64,
        var_vds_dn7: f64,
        var_vds_dn8: f64,
        var_vds_dn9: f64,
        var_vdsz__blk439: f64,
        var_vdsz__blk439_dn0: f64,
        var_vdsz__blk439_dn10: f64,
        var_vdsz__blk439_dn13: f64,
        var_vdsz__blk439_dn2: f64,
        var_vdsz__blk439_dn4: f64,
        var_vdsz__blk439_dn5: f64,
        var_vdsz__blk439_dn6: f64,
        var_vdsz__blk439_dn7: f64,
        var_vdsz__blk439_dn8: f64,
        var_vdsz__blk439_dn9: f64,
        var_guard1496_slot: &mut f64,
        var_guard1496_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn13_slot: &mut f64,
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
        var_t1_dn13_slot: &mut f64,
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
        var_t2_dn13_slot: &mut f64,
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
        var_t3_dn13_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn13_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_dn9_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn10_slot: &mut f64,
        var_t5_dn13_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t5_dn9_slot: &mut f64,
        var_t5_rv_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn13_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t6_dn9_slot: &mut f64,
        var_t6_rv_slot: &mut f64,
        var_t9_slot: &mut f64,
        var_t9_dn0_slot: &mut f64,
        var_t9_dn10_slot: &mut f64,
        var_t9_dn13_slot: &mut f64,
        var_t9_dn2_slot: &mut f64,
        var_t9_dn4_slot: &mut f64,
        var_t9_dn5_slot: &mut f64,
        var_t9_dn6_slot: &mut f64,
        var_t9_dn7_slot: &mut f64,
        var_t9_dn8_slot: &mut f64,
        var_t9_dn9_slot: &mut f64,
        var_t9_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn13_slot: &mut f64,
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
        var_tmf2_dn13_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn7_slot: &mut f64,
        var_tmf2_dn8_slot: &mut f64,
        var_tmf2_dn9_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_tmf3_slot: &mut f64,
        var_tmf3_dn0_slot: &mut f64,
        var_tmf3_dn10_slot: &mut f64,
        var_tmf3_dn13_slot: &mut f64,
        var_tmf3_dn2_slot: &mut f64,
        var_tmf3_dn4_slot: &mut f64,
        var_tmf3_dn5_slot: &mut f64,
        var_tmf3_dn6_slot: &mut f64,
        var_tmf3_dn7_slot: &mut f64,
        var_tmf3_dn8_slot: &mut f64,
        var_tmf3_dn9_slot: &mut f64,
        var_tmf3_rv_slot: &mut f64,
    ) {
        let mut var_guard1496: f64 = *var_guard1496_slot;
        let mut var_guard1496_rv: f64 = *var_guard1496_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn13: f64 = *var_t0_dn13_slot;
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
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
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
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
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
        let mut var_t3_dn13: f64 = *var_t3_dn13_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn13: f64 = *var_t4_dn13_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_dn9: f64 = *var_t4_dn9_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn10: f64 = *var_t5_dn10_slot;
        let mut var_t5_dn13: f64 = *var_t5_dn13_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t5_dn9: f64 = *var_t5_dn9_slot;
        let mut var_t5_rv: f64 = *var_t5_rv_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn13: f64 = *var_t6_dn13_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t6_dn9: f64 = *var_t6_dn9_slot;
        let mut var_t6_rv: f64 = *var_t6_rv_slot;
        let mut var_t9: f64 = *var_t9_slot;
        let mut var_t9_dn0: f64 = *var_t9_dn0_slot;
        let mut var_t9_dn10: f64 = *var_t9_dn10_slot;
        let mut var_t9_dn13: f64 = *var_t9_dn13_slot;
        let mut var_t9_dn2: f64 = *var_t9_dn2_slot;
        let mut var_t9_dn4: f64 = *var_t9_dn4_slot;
        let mut var_t9_dn5: f64 = *var_t9_dn5_slot;
        let mut var_t9_dn6: f64 = *var_t9_dn6_slot;
        let mut var_t9_dn7: f64 = *var_t9_dn7_slot;
        let mut var_t9_dn8: f64 = *var_t9_dn8_slot;
        let mut var_t9_dn9: f64 = *var_t9_dn9_slot;
        let mut var_t9_rv: f64 = *var_t9_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn13: f64 = *var_tmf1_dn13_slot;
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
        let mut var_tmf2_dn13: f64 = *var_tmf2_dn13_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn7: f64 = *var_tmf2_dn7_slot;
        let mut var_tmf2_dn8: f64 = *var_tmf2_dn8_slot;
        let mut var_tmf2_dn9: f64 = *var_tmf2_dn9_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_tmf3: f64 = *var_tmf3_slot;
        let mut var_tmf3_dn0: f64 = *var_tmf3_dn0_slot;
        let mut var_tmf3_dn10: f64 = *var_tmf3_dn10_slot;
        let mut var_tmf3_dn13: f64 = *var_tmf3_dn13_slot;
        let mut var_tmf3_dn2: f64 = *var_tmf3_dn2_slot;
        let mut var_tmf3_dn4: f64 = *var_tmf3_dn4_slot;
        let mut var_tmf3_dn5: f64 = *var_tmf3_dn5_slot;
        let mut var_tmf3_dn6: f64 = *var_tmf3_dn6_slot;
        let mut var_tmf3_dn7: f64 = *var_tmf3_dn7_slot;
        let mut var_tmf3_dn8: f64 = *var_tmf3_dn8_slot;
        let mut var_tmf3_dn9: f64 = *var_tmf3_dn9_slot;
        let mut var_tmf3_rv: f64 = *var_tmf3_rv_slot;

        let (assign62320_e96766, assign62320_e96766_d_n0, assign62320_e96766_d_n2, assign62320_e96766_d_n4, assign62320_e96766_d_n5, assign62320_e96766_d_n6, assign62320_e96766_d_n7, assign62320_e96766_d_n8, assign62320_e96766_d_n9, assign62320_e96766_d_n10, assign62320_e96766_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1495 != 0.0)) {
        let assign62320_e96763: f64 = (var_vds - var_pds);
        let assign62320_e96764: f64 = (0.5 * assign62320_e96763);
        (assign62320_e96764, (0.5 * (var_vds_dn0 - var_pds_dn0)), (0.5 * (var_vds_dn2 - var_pds_dn2)), (0.5 * (var_vds_dn4 - var_pds_dn4)), (0.5 * (var_vds_dn5 - var_pds_dn5)), (0.5 * (var_vds_dn6 - var_pds_dn6)), (0.5 * (var_vds_dn7 - var_pds_dn7)), (0.5 * (var_vds_dn8 - var_pds_dn8)), (0.5 * (var_vds_dn9 - var_pds_dn9)), (0.5 * (var_vds_dn10 - var_pds_dn10)), (0.5 * (var_vds_dn13 - var_pds_dn13)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign62320_e96766;
        var_t1_dn0 = assign62320_e96766_d_n0;
        var_t1_dn2 = assign62320_e96766_d_n2;
        var_t1_dn4 = assign62320_e96766_d_n4;
        var_t1_dn5 = assign62320_e96766_d_n5;
        var_t1_dn6 = assign62320_e96766_d_n6;
        var_t1_dn7 = assign62320_e96766_d_n7;
        var_t1_dn8 = assign62320_e96766_d_n8;
        var_t1_dn9 = assign62320_e96766_d_n9;
        var_t1_dn10 = assign62320_e96766_d_n10;
        var_t1_dn13 = assign62320_e96766_d_n13;
        var_t1_rv = 0.0;

        let (assign62330_e96779, assign62330_e96779_d_n0, assign62330_e96779_d_n2, assign62330_e96779_d_n4, assign62330_e96779_d_n5, assign62330_e96779_d_n6, assign62330_e96779_d_n7, assign62330_e96779_d_n8, assign62330_e96779_d_n9, assign62330_e96779_d_n10, assign62330_e96779_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1495 != 0.0)) {
        let assign62330_e96775: f64 = (2.0 * var_t1);
        let assign62330_e96777: f64 = (assign62330_e96775 / 0.01);
        (assign62330_e96777, ((2.0 * var_t1_dn0) / 0.01), ((2.0 * var_t1_dn2) / 0.01), ((2.0 * var_t1_dn4) / 0.01), ((2.0 * var_t1_dn5) / 0.01), ((2.0 * var_t1_dn6) / 0.01), ((2.0 * var_t1_dn7) / 0.01), ((2.0 * var_t1_dn8) / 0.01), ((2.0 * var_t1_dn9) / 0.01), ((2.0 * var_t1_dn10) / 0.01), ((2.0 * var_t1_dn13) / 0.01),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn13,)
    }
};
        var_tmf1 = assign62330_e96779;
        var_tmf1_dn0 = assign62330_e96779_d_n0;
        var_tmf1_dn2 = assign62330_e96779_d_n2;
        var_tmf1_dn4 = assign62330_e96779_d_n4;
        var_tmf1_dn5 = assign62330_e96779_d_n5;
        var_tmf1_dn6 = assign62330_e96779_d_n6;
        var_tmf1_dn7 = assign62330_e96779_d_n7;
        var_tmf1_dn8 = assign62330_e96779_d_n8;
        var_tmf1_dn9 = assign62330_e96779_d_n9;
        var_tmf1_dn10 = assign62330_e96779_d_n10;
        var_tmf1_dn13 = assign62330_e96779_d_n13;
        var_tmf1_rv = 0.0;

        let (assign62340_e96824, assign62340_e96824_d_n0, assign62340_e96824_d_n2, assign62340_e96824_d_n4, assign62340_e96824_d_n5, assign62340_e96824_d_n6, assign62340_e96824_d_n7, assign62340_e96824_d_n8, assign62340_e96824_d_n9, assign62340_e96824_d_n10, assign62340_e96824_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1495 != 0.0)) {
        let assign62340_e96790: f64 = (1.0 / 2.0);
        let assign62340_e96794: f64 = (1.0 / 6.0);
        let assign62340_e96798: f64 = (1.0 / 24.0);
        let assign62340_e96802: f64 = (1.0 / 120.0);
        let assign62340_e96806: f64 = (1.0 / 720.0);
        let assign62340_e96810: f64 = (1.0 / 5040.0);
        let assign62340_e96811: f64 = (var_tmf1 * assign62340_e96810);
        let assign62340_e96812: f64 = (assign62340_e96806 + assign62340_e96811);
        let assign62340_e96813: f64 = (var_tmf1 * assign62340_e96812);
        let assign62340_e96814: f64 = (assign62340_e96802 + assign62340_e96813);
        let assign62340_e96815: f64 = (var_tmf1 * assign62340_e96814);
        let assign62340_e96816: f64 = (assign62340_e96798 + assign62340_e96815);
        let assign62340_e96817: f64 = (var_tmf1 * assign62340_e96816);
        let assign62340_e96818: f64 = (assign62340_e96794 + assign62340_e96817);
        let assign62340_e96819: f64 = (var_tmf1 * assign62340_e96818);
        let assign62340_e96820: f64 = (assign62340_e96790 + assign62340_e96819);
        let assign62340_e96821: f64 = (var_tmf1 * assign62340_e96820);
        let assign62340_e96822: f64 = (1.0 + assign62340_e96821);
        (assign62340_e96822, ((var_tmf1_dn0 * assign62340_e96820) + (var_tmf1 * ((var_tmf1_dn0 * assign62340_e96818) + (var_tmf1 * ((var_tmf1_dn0 * assign62340_e96816) + (var_tmf1 * ((var_tmf1_dn0 * assign62340_e96814) + (var_tmf1 * ((var_tmf1_dn0 * assign62340_e96812) + (var_tmf1 * (var_tmf1_dn0 * assign62340_e96810))))))))))), ((var_tmf1_dn2 * assign62340_e96820) + (var_tmf1 * ((var_tmf1_dn2 * assign62340_e96818) + (var_tmf1 * ((var_tmf1_dn2 * assign62340_e96816) + (var_tmf1 * ((var_tmf1_dn2 * assign62340_e96814) + (var_tmf1 * ((var_tmf1_dn2 * assign62340_e96812) + (var_tmf1 * (var_tmf1_dn2 * assign62340_e96810))))))))))), ((var_tmf1_dn4 * assign62340_e96820) + (var_tmf1 * ((var_tmf1_dn4 * assign62340_e96818) + (var_tmf1 * ((var_tmf1_dn4 * assign62340_e96816) + (var_tmf1 * ((var_tmf1_dn4 * assign62340_e96814) + (var_tmf1 * ((var_tmf1_dn4 * assign62340_e96812) + (var_tmf1 * (var_tmf1_dn4 * assign62340_e96810))))))))))), ((var_tmf1_dn5 * assign62340_e96820) + (var_tmf1 * ((var_tmf1_dn5 * assign62340_e96818) + (var_tmf1 * ((var_tmf1_dn5 * assign62340_e96816) + (var_tmf1 * ((var_tmf1_dn5 * assign62340_e96814) + (var_tmf1 * ((var_tmf1_dn5 * assign62340_e96812) + (var_tmf1 * (var_tmf1_dn5 * assign62340_e96810))))))))))), ((var_tmf1_dn6 * assign62340_e96820) + (var_tmf1 * ((var_tmf1_dn6 * assign62340_e96818) + (var_tmf1 * ((var_tmf1_dn6 * assign62340_e96816) + (var_tmf1 * ((var_tmf1_dn6 * assign62340_e96814) + (var_tmf1 * ((var_tmf1_dn6 * assign62340_e96812) + (var_tmf1 * (var_tmf1_dn6 * assign62340_e96810))))))))))), ((var_tmf1_dn7 * assign62340_e96820) + (var_tmf1 * ((var_tmf1_dn7 * assign62340_e96818) + (var_tmf1 * ((var_tmf1_dn7 * assign62340_e96816) + (var_tmf1 * ((var_tmf1_dn7 * assign62340_e96814) + (var_tmf1 * ((var_tmf1_dn7 * assign62340_e96812) + (var_tmf1 * (var_tmf1_dn7 * assign62340_e96810))))))))))), ((var_tmf1_dn8 * assign62340_e96820) + (var_tmf1 * ((var_tmf1_dn8 * assign62340_e96818) + (var_tmf1 * ((var_tmf1_dn8 * assign62340_e96816) + (var_tmf1 * ((var_tmf1_dn8 * assign62340_e96814) + (var_tmf1 * ((var_tmf1_dn8 * assign62340_e96812) + (var_tmf1 * (var_tmf1_dn8 * assign62340_e96810))))))))))), ((var_tmf1_dn9 * assign62340_e96820) + (var_tmf1 * ((var_tmf1_dn9 * assign62340_e96818) + (var_tmf1 * ((var_tmf1_dn9 * assign62340_e96816) + (var_tmf1 * ((var_tmf1_dn9 * assign62340_e96814) + (var_tmf1 * ((var_tmf1_dn9 * assign62340_e96812) + (var_tmf1 * (var_tmf1_dn9 * assign62340_e96810))))))))))), ((var_tmf1_dn10 * assign62340_e96820) + (var_tmf1 * ((var_tmf1_dn10 * assign62340_e96818) + (var_tmf1 * ((var_tmf1_dn10 * assign62340_e96816) + (var_tmf1 * ((var_tmf1_dn10 * assign62340_e96814) + (var_tmf1 * ((var_tmf1_dn10 * assign62340_e96812) + (var_tmf1 * (var_tmf1_dn10 * assign62340_e96810))))))))))), ((var_tmf1_dn13 * assign62340_e96820) + (var_tmf1 * ((var_tmf1_dn13 * assign62340_e96818) + (var_tmf1 * ((var_tmf1_dn13 * assign62340_e96816) + (var_tmf1 * ((var_tmf1_dn13 * assign62340_e96814) + (var_tmf1 * ((var_tmf1_dn13 * assign62340_e96812) + (var_tmf1 * (var_tmf1_dn13 * assign62340_e96810))))))))))),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
    }
};
        var_tmf2 = assign62340_e96824;
        var_tmf2_dn0 = assign62340_e96824_d_n0;
        var_tmf2_dn2 = assign62340_e96824_d_n2;
        var_tmf2_dn4 = assign62340_e96824_d_n4;
        var_tmf2_dn5 = assign62340_e96824_d_n5;
        var_tmf2_dn6 = assign62340_e96824_d_n6;
        var_tmf2_dn7 = assign62340_e96824_d_n7;
        var_tmf2_dn8 = assign62340_e96824_d_n8;
        var_tmf2_dn9 = assign62340_e96824_d_n9;
        var_tmf2_dn10 = assign62340_e96824_d_n10;
        var_tmf2_dn13 = assign62340_e96824_d_n13;
        var_tmf2_rv = 0.0;

        let (assign62350_e96865, assign62350_e96865_d_n0, assign62350_e96865_d_n2, assign62350_e96865_d_n4, assign62350_e96865_d_n5, assign62350_e96865_d_n6, assign62350_e96865_d_n7, assign62350_e96865_d_n8, assign62350_e96865_d_n9, assign62350_e96865_d_n10, assign62350_e96865_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1495 != 0.0)) {
        let assign62350_e96833: f64 = (1.0 / 2.0);
        let assign62350_e96837: f64 = (1.0 / 3.0);
        let assign62350_e96841: f64 = (1.0 / 8.0);
        let assign62350_e96845: f64 = (1.0 / 30.0);
        let assign62350_e96849: f64 = (1.0 / 144.0);
        let assign62350_e96853: f64 = (1.0 / 840.0);
        let assign62350_e96854: f64 = (var_tmf1 * assign62350_e96853);
        let assign62350_e96855: f64 = (assign62350_e96849 + assign62350_e96854);
        let assign62350_e96856: f64 = (var_tmf1 * assign62350_e96855);
        let assign62350_e96857: f64 = (assign62350_e96845 + assign62350_e96856);
        let assign62350_e96858: f64 = (var_tmf1 * assign62350_e96857);
        let assign62350_e96859: f64 = (assign62350_e96841 + assign62350_e96858);
        let assign62350_e96860: f64 = (var_tmf1 * assign62350_e96859);
        let assign62350_e96861: f64 = (assign62350_e96837 + assign62350_e96860);
        let assign62350_e96862: f64 = (var_tmf1 * assign62350_e96861);
        let assign62350_e96863: f64 = (assign62350_e96833 + assign62350_e96862);
        (assign62350_e96863, ((var_tmf1_dn0 * assign62350_e96861) + (var_tmf1 * ((var_tmf1_dn0 * assign62350_e96859) + (var_tmf1 * ((var_tmf1_dn0 * assign62350_e96857) + (var_tmf1 * ((var_tmf1_dn0 * assign62350_e96855) + (var_tmf1 * (var_tmf1_dn0 * assign62350_e96853))))))))), ((var_tmf1_dn2 * assign62350_e96861) + (var_tmf1 * ((var_tmf1_dn2 * assign62350_e96859) + (var_tmf1 * ((var_tmf1_dn2 * assign62350_e96857) + (var_tmf1 * ((var_tmf1_dn2 * assign62350_e96855) + (var_tmf1 * (var_tmf1_dn2 * assign62350_e96853))))))))), ((var_tmf1_dn4 * assign62350_e96861) + (var_tmf1 * ((var_tmf1_dn4 * assign62350_e96859) + (var_tmf1 * ((var_tmf1_dn4 * assign62350_e96857) + (var_tmf1 * ((var_tmf1_dn4 * assign62350_e96855) + (var_tmf1 * (var_tmf1_dn4 * assign62350_e96853))))))))), ((var_tmf1_dn5 * assign62350_e96861) + (var_tmf1 * ((var_tmf1_dn5 * assign62350_e96859) + (var_tmf1 * ((var_tmf1_dn5 * assign62350_e96857) + (var_tmf1 * ((var_tmf1_dn5 * assign62350_e96855) + (var_tmf1 * (var_tmf1_dn5 * assign62350_e96853))))))))), ((var_tmf1_dn6 * assign62350_e96861) + (var_tmf1 * ((var_tmf1_dn6 * assign62350_e96859) + (var_tmf1 * ((var_tmf1_dn6 * assign62350_e96857) + (var_tmf1 * ((var_tmf1_dn6 * assign62350_e96855) + (var_tmf1 * (var_tmf1_dn6 * assign62350_e96853))))))))), ((var_tmf1_dn7 * assign62350_e96861) + (var_tmf1 * ((var_tmf1_dn7 * assign62350_e96859) + (var_tmf1 * ((var_tmf1_dn7 * assign62350_e96857) + (var_tmf1 * ((var_tmf1_dn7 * assign62350_e96855) + (var_tmf1 * (var_tmf1_dn7 * assign62350_e96853))))))))), ((var_tmf1_dn8 * assign62350_e96861) + (var_tmf1 * ((var_tmf1_dn8 * assign62350_e96859) + (var_tmf1 * ((var_tmf1_dn8 * assign62350_e96857) + (var_tmf1 * ((var_tmf1_dn8 * assign62350_e96855) + (var_tmf1 * (var_tmf1_dn8 * assign62350_e96853))))))))), ((var_tmf1_dn9 * assign62350_e96861) + (var_tmf1 * ((var_tmf1_dn9 * assign62350_e96859) + (var_tmf1 * ((var_tmf1_dn9 * assign62350_e96857) + (var_tmf1 * ((var_tmf1_dn9 * assign62350_e96855) + (var_tmf1 * (var_tmf1_dn9 * assign62350_e96853))))))))), ((var_tmf1_dn10 * assign62350_e96861) + (var_tmf1 * ((var_tmf1_dn10 * assign62350_e96859) + (var_tmf1 * ((var_tmf1_dn10 * assign62350_e96857) + (var_tmf1 * ((var_tmf1_dn10 * assign62350_e96855) + (var_tmf1 * (var_tmf1_dn10 * assign62350_e96853))))))))), ((var_tmf1_dn13 * assign62350_e96861) + (var_tmf1 * ((var_tmf1_dn13 * assign62350_e96859) + (var_tmf1 * ((var_tmf1_dn13 * assign62350_e96857) + (var_tmf1 * ((var_tmf1_dn13 * assign62350_e96855) + (var_tmf1 * (var_tmf1_dn13 * assign62350_e96853))))))))),)
    } else {
        (var_tmf3, var_tmf3_dn0, var_tmf3_dn2, var_tmf3_dn4, var_tmf3_dn5, var_tmf3_dn6, var_tmf3_dn7, var_tmf3_dn8, var_tmf3_dn9, var_tmf3_dn10, var_tmf3_dn13,)
    }
};
        var_tmf3 = assign62350_e96865;
        var_tmf3_dn0 = assign62350_e96865_d_n0;
        var_tmf3_dn2 = assign62350_e96865_d_n2;
        var_tmf3_dn4 = assign62350_e96865_d_n4;
        var_tmf3_dn5 = assign62350_e96865_d_n5;
        var_tmf3_dn6 = assign62350_e96865_d_n6;
        var_tmf3_dn7 = assign62350_e96865_d_n7;
        var_tmf3_dn8 = assign62350_e96865_d_n8;
        var_tmf3_dn9 = assign62350_e96865_d_n9;
        var_tmf3_dn10 = assign62350_e96865_d_n10;
        var_tmf3_dn13 = assign62350_e96865_d_n13;
        var_tmf3_rv = 0.0;

        let (assign62360_e96876, assign62360_e96876_d_n0, assign62360_e96876_d_n2, assign62360_e96876_d_n4, assign62360_e96876_d_n5, assign62360_e96876_d_n6, assign62360_e96876_d_n7, assign62360_e96876_d_n8, assign62360_e96876_d_n9, assign62360_e96876_d_n10, assign62360_e96876_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1495 != 0.0)) {
        let assign62360_e96874: f64 = (0.01 / var_tmf2);
        (assign62360_e96874, (-((0.01 * var_tmf2_dn0) / (var_tmf2 * var_tmf2))), (-((0.01 * var_tmf2_dn2) / (var_tmf2 * var_tmf2))), (-((0.01 * var_tmf2_dn4) / (var_tmf2 * var_tmf2))), (-((0.01 * var_tmf2_dn5) / (var_tmf2 * var_tmf2))), (-((0.01 * var_tmf2_dn6) / (var_tmf2 * var_tmf2))), (-((0.01 * var_tmf2_dn7) / (var_tmf2 * var_tmf2))), (-((0.01 * var_tmf2_dn8) / (var_tmf2 * var_tmf2))), (-((0.01 * var_tmf2_dn9) / (var_tmf2 * var_tmf2))), (-((0.01 * var_tmf2_dn10) / (var_tmf2 * var_tmf2))), (-((0.01 * var_tmf2_dn13) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn13,)
    }
};
        var_t6 = assign62360_e96876;
        var_t6_dn0 = assign62360_e96876_d_n0;
        var_t6_dn2 = assign62360_e96876_d_n2;
        var_t6_dn4 = assign62360_e96876_d_n4;
        var_t6_dn5 = assign62360_e96876_d_n5;
        var_t6_dn6 = assign62360_e96876_d_n6;
        var_t6_dn7 = assign62360_e96876_d_n7;
        var_t6_dn8 = assign62360_e96876_d_n8;
        var_t6_dn9 = assign62360_e96876_d_n9;
        var_t6_dn10 = assign62360_e96876_d_n10;
        var_t6_dn13 = assign62360_e96876_d_n13;
        var_t6_rv = 0.0;

        let (assign62370_e96892, assign62370_e96892_d_n0, assign62370_e96892_d_n2, assign62370_e96892_d_n4, assign62370_e96892_d_n5, assign62370_e96892_d_n6, assign62370_e96892_d_n7, assign62370_e96892_d_n8, assign62370_e96892_d_n9, assign62370_e96892_d_n10, assign62370_e96892_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1495 != 0.0)) {
        let assign62370_e96884: f64 = (-2.0);
        let assign62370_e96886: f64 = (assign62370_e96884 * var_tmf3);
        let assign62370_e96889: f64 = (var_tmf2 * var_tmf2);
        let assign62370_e96890: f64 = (assign62370_e96886 / assign62370_e96889);
        (assign62370_e96890, ((((assign62370_e96884 * var_tmf3_dn0) * assign62370_e96889) - (assign62370_e96886 * ((var_tmf2_dn0 * var_tmf2) + (var_tmf2 * var_tmf2_dn0)))) / (assign62370_e96889 * assign62370_e96889)), ((((assign62370_e96884 * var_tmf3_dn2) * assign62370_e96889) - (assign62370_e96886 * ((var_tmf2_dn2 * var_tmf2) + (var_tmf2 * var_tmf2_dn2)))) / (assign62370_e96889 * assign62370_e96889)), ((((assign62370_e96884 * var_tmf3_dn4) * assign62370_e96889) - (assign62370_e96886 * ((var_tmf2_dn4 * var_tmf2) + (var_tmf2 * var_tmf2_dn4)))) / (assign62370_e96889 * assign62370_e96889)), ((((assign62370_e96884 * var_tmf3_dn5) * assign62370_e96889) - (assign62370_e96886 * ((var_tmf2_dn5 * var_tmf2) + (var_tmf2 * var_tmf2_dn5)))) / (assign62370_e96889 * assign62370_e96889)), ((((assign62370_e96884 * var_tmf3_dn6) * assign62370_e96889) - (assign62370_e96886 * ((var_tmf2_dn6 * var_tmf2) + (var_tmf2 * var_tmf2_dn6)))) / (assign62370_e96889 * assign62370_e96889)), ((((assign62370_e96884 * var_tmf3_dn7) * assign62370_e96889) - (assign62370_e96886 * ((var_tmf2_dn7 * var_tmf2) + (var_tmf2 * var_tmf2_dn7)))) / (assign62370_e96889 * assign62370_e96889)), ((((assign62370_e96884 * var_tmf3_dn8) * assign62370_e96889) - (assign62370_e96886 * ((var_tmf2_dn8 * var_tmf2) + (var_tmf2 * var_tmf2_dn8)))) / (assign62370_e96889 * assign62370_e96889)), ((((assign62370_e96884 * var_tmf3_dn9) * assign62370_e96889) - (assign62370_e96886 * ((var_tmf2_dn9 * var_tmf2) + (var_tmf2 * var_tmf2_dn9)))) / (assign62370_e96889 * assign62370_e96889)), ((((assign62370_e96884 * var_tmf3_dn10) * assign62370_e96889) - (assign62370_e96886 * ((var_tmf2_dn10 * var_tmf2) + (var_tmf2 * var_tmf2_dn10)))) / (assign62370_e96889 * assign62370_e96889)), ((((assign62370_e96884 * var_tmf3_dn13) * assign62370_e96889) - (assign62370_e96886 * ((var_tmf2_dn13 * var_tmf2) + (var_tmf2 * var_tmf2_dn13)))) / (assign62370_e96889 * assign62370_e96889)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign62370_e96892;
        var_t2_dn0 = assign62370_e96892_d_n0;
        var_t2_dn2 = assign62370_e96892_d_n2;
        var_t2_dn4 = assign62370_e96892_d_n4;
        var_t2_dn5 = assign62370_e96892_d_n5;
        var_t2_dn6 = assign62370_e96892_d_n6;
        var_t2_dn7 = assign62370_e96892_d_n7;
        var_t2_dn8 = assign62370_e96892_d_n8;
        var_t2_dn9 = assign62370_e96892_d_n9;
        var_t2_dn10 = assign62370_e96892_d_n10;
        var_t2_dn13 = assign62370_e96892_d_n13;
        var_t2_rv = 0.0;

        let (assign62380_e96903, assign62380_e96903_d_n0, assign62380_e96903_d_n2, assign62380_e96903_d_n4, assign62380_e96903_d_n5, assign62380_e96903_d_n6, assign62380_e96903_d_n7, assign62380_e96903_d_n8, assign62380_e96903_d_n9, assign62380_e96903_d_n10, assign62380_e96903_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1495 != 0.0)) {
        let assign62380_e96901: f64 = (var_t2 * 0.5);
        (assign62380_e96901, (var_t2_dn0 * 0.5), (var_t2_dn2 * 0.5), (var_t2_dn4 * 0.5), (var_t2_dn5 * 0.5), (var_t2_dn6 * 0.5), (var_t2_dn7 * 0.5), (var_t2_dn8 * 0.5), (var_t2_dn9 * 0.5), (var_t2_dn10 * 0.5), (var_t2_dn13 * 0.5),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign62380_e96903;
        var_t2_dn0 = assign62380_e96903_d_n0;
        var_t2_dn2 = assign62380_e96903_d_n2;
        var_t2_dn4 = assign62380_e96903_d_n4;
        var_t2_dn5 = assign62380_e96903_d_n5;
        var_t2_dn6 = assign62380_e96903_d_n6;
        var_t2_dn7 = assign62380_e96903_d_n7;
        var_t2_dn8 = assign62380_e96903_d_n8;
        var_t2_dn9 = assign62380_e96903_d_n9;
        var_t2_dn10 = assign62380_e96903_d_n10;
        var_t2_dn13 = assign62380_e96903_d_n13;
        var_t2_rv = 0.0;

        let (assign62390_e96916, assign62390_e96916_d_n0, assign62390_e96916_d_n2, assign62390_e96916_d_n4, assign62390_e96916_d_n5, assign62390_e96916_d_n6, assign62390_e96916_d_n7, assign62390_e96916_d_n8, assign62390_e96916_d_n9, assign62390_e96916_d_n10, assign62390_e96916_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1495 != 0.0)) {
        let assign62390_e96913: f64 = (var_ps0 + var_t6);
        let assign62390_e96914: f64 = (1.1 - assign62390_e96913);
        (assign62390_e96914, (-(var_ps0_dn0 + var_t6_dn0)), (-(var_ps0_dn2 + var_t6_dn2)), (-(var_ps0_dn4 + var_t6_dn4)), (-(var_ps0_dn5 + var_t6_dn5)), (-(var_ps0_dn6 + var_t6_dn6)), (-(var_ps0_dn7 + var_t6_dn7)), (-(var_ps0_dn8 + var_t6_dn8)), (-(var_ps0_dn9 + var_t6_dn9)), (-(var_ps0_dn10 + var_t6_dn10)), (-(var_ps0_dn13 + var_t6_dn13)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign62390_e96916;
        var_t1_dn0 = assign62390_e96916_d_n0;
        var_t1_dn2 = assign62390_e96916_d_n2;
        var_t1_dn4 = assign62390_e96916_d_n4;
        var_t1_dn5 = assign62390_e96916_d_n5;
        var_t1_dn6 = assign62390_e96916_d_n6;
        var_t1_dn7 = assign62390_e96916_d_n7;
        var_t1_dn8 = assign62390_e96916_d_n8;
        var_t1_dn9 = assign62390_e96916_d_n9;
        var_t1_dn10 = assign62390_e96916_d_n10;
        var_t1_dn13 = assign62390_e96916_d_n13;
        var_t1_rv = 0.0;

        let (assign62400_e96934, assign62400_e96934_d_n0, assign62400_e96934_d_n2, assign62400_e96934_d_n4, assign62400_e96934_d_n5, assign62400_e96934_d_n6, assign62400_e96934_d_n7, assign62400_e96934_d_n8, assign62400_e96934_d_n9, assign62400_e96934_d_n10, assign62400_e96934_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1495 != 0.0)) {
        let assign62400_e96925: f64 = (var_t1 * var_t1);
        let assign62400_e96928: f64 = (4.0 * 0.05);
        let assign62400_e96930: f64 = (assign62400_e96928 * 0.05);
        let assign62400_e96931: f64 = (assign62400_e96925 + assign62400_e96930);
        let assign62400_e96932: f64 = (assign62400_e96931).sqrt();
        (assign62400_e96932, (((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)) / (2.0 * assign62400_e96932)), (((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)) / (2.0 * assign62400_e96932)), (((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)) / (2.0 * assign62400_e96932)), (((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)) / (2.0 * assign62400_e96932)), (((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)) / (2.0 * assign62400_e96932)), (((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7)) / (2.0 * assign62400_e96932)), (((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)) / (2.0 * assign62400_e96932)), (((var_t1_dn9 * var_t1) + (var_t1 * var_t1_dn9)) / (2.0 * assign62400_e96932)), (((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10)) / (2.0 * assign62400_e96932)), (((var_t1_dn13 * var_t1) + (var_t1 * var_t1_dn13)) / (2.0 * assign62400_e96932)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
    }
};
        var_tmf2 = assign62400_e96934;
        var_tmf2_dn0 = assign62400_e96934_d_n0;
        var_tmf2_dn2 = assign62400_e96934_d_n2;
        var_tmf2_dn4 = assign62400_e96934_d_n4;
        var_tmf2_dn5 = assign62400_e96934_d_n5;
        var_tmf2_dn6 = assign62400_e96934_d_n6;
        var_tmf2_dn7 = assign62400_e96934_d_n7;
        var_tmf2_dn8 = assign62400_e96934_d_n8;
        var_tmf2_dn9 = assign62400_e96934_d_n9;
        var_tmf2_dn10 = assign62400_e96934_d_n10;
        var_tmf2_dn13 = assign62400_e96934_d_n13;
        var_tmf2_rv = 0.0;

        let (assign62410_e96949, assign62410_e96949_d_n0, assign62410_e96949_d_n2, assign62410_e96949_d_n4, assign62410_e96949_d_n5, assign62410_e96949_d_n6, assign62410_e96949_d_n7, assign62410_e96949_d_n8, assign62410_e96949_d_n9, assign62410_e96949_d_n10, assign62410_e96949_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1495 != 0.0)) {
        let assign62410_e96945: f64 = (var_t1 / var_tmf2);
        let assign62410_e96946: f64 = (1.0 + assign62410_e96945);
        let assign62410_e96947: f64 = (0.5 * assign62410_e96946);
        (assign62410_e96947, (0.5 * (((var_t1_dn0 * var_tmf2) - (var_t1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn2 * var_tmf2) - (var_t1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn4 * var_tmf2) - (var_t1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn5 * var_tmf2) - (var_t1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn6 * var_tmf2) - (var_t1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn7 * var_tmf2) - (var_t1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn8 * var_tmf2) - (var_t1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn9 * var_tmf2) - (var_t1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn10 * var_tmf2) - (var_t1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn13 * var_tmf2) - (var_t1 * var_tmf2_dn13)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign62410_e96949;
        var_t0_dn0 = assign62410_e96949_d_n0;
        var_t0_dn2 = assign62410_e96949_d_n2;
        var_t0_dn4 = assign62410_e96949_d_n4;
        var_t0_dn5 = assign62410_e96949_d_n5;
        var_t0_dn6 = assign62410_e96949_d_n6;
        var_t0_dn7 = assign62410_e96949_d_n7;
        var_t0_dn8 = assign62410_e96949_d_n8;
        var_t0_dn9 = assign62410_e96949_d_n9;
        var_t0_dn10 = assign62410_e96949_d_n10;
        var_t0_dn13 = assign62410_e96949_d_n13;
        var_t0_rv = 0.0;

        let (assign62420_e96962, assign62420_e96962_d_n0, assign62420_e96962_d_n2, assign62420_e96962_d_n4, assign62420_e96962_d_n5, assign62420_e96962_d_n6, assign62420_e96962_d_n7, assign62420_e96962_d_n8, assign62420_e96962_d_n9, assign62420_e96962_d_n10, assign62420_e96962_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1495 != 0.0)) {
        let assign62420_e96959: f64 = (var_t1 + var_tmf2);
        let assign62420_e96960: f64 = (0.5 * assign62420_e96959);
        (assign62420_e96960, (0.5 * (var_t1_dn0 + var_tmf2_dn0)), (0.5 * (var_t1_dn2 + var_tmf2_dn2)), (0.5 * (var_t1_dn4 + var_tmf2_dn4)), (0.5 * (var_t1_dn5 + var_tmf2_dn5)), (0.5 * (var_t1_dn6 + var_tmf2_dn6)), (0.5 * (var_t1_dn7 + var_tmf2_dn7)), (0.5 * (var_t1_dn8 + var_tmf2_dn8)), (0.5 * (var_t1_dn9 + var_tmf2_dn9)), (0.5 * (var_t1_dn10 + var_tmf2_dn10)), (0.5 * (var_t1_dn13 + var_tmf2_dn13)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign62420_e96962;
        var_t2_dn0 = assign62420_e96962_d_n0;
        var_t2_dn2 = assign62420_e96962_d_n2;
        var_t2_dn4 = assign62420_e96962_d_n4;
        var_t2_dn5 = assign62420_e96962_d_n5;
        var_t2_dn6 = assign62420_e96962_d_n6;
        var_t2_dn7 = assign62420_e96962_d_n7;
        var_t2_dn8 = assign62420_e96962_d_n8;
        var_t2_dn9 = assign62420_e96962_d_n9;
        var_t2_dn10 = assign62420_e96962_d_n10;
        var_t2_dn13 = assign62420_e96962_d_n13;
        var_t2_rv = 0.0;

        let assign62430_e96965: f64 = if var_t2 < 0.0 { 1.0 } else { 0.0 };
        var_guard1496 = assign62430_e96965;
        var_guard1496_rv = 0.0;

        let (assign62440_e96976, assign62440_e96976_d_n0, assign62440_e96976_d_n2, assign62440_e96976_d_n4, assign62440_e96976_d_n5, assign62440_e96976_d_n6, assign62440_e96976_d_n7, assign62440_e96976_d_n8, assign62440_e96976_d_n9, assign62440_e96976_d_n10, assign62440_e96976_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1495 != 0.0)) && (var_guard1496 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign62440_e96976;
        var_t2_dn0 = assign62440_e96976_d_n0;
        var_t2_dn2 = assign62440_e96976_d_n2;
        var_t2_dn4 = assign62440_e96976_d_n4;
        var_t2_dn5 = assign62440_e96976_d_n5;
        var_t2_dn6 = assign62440_e96976_d_n6;
        var_t2_dn7 = assign62440_e96976_d_n7;
        var_t2_dn8 = assign62440_e96976_d_n8;
        var_t2_dn9 = assign62440_e96976_d_n9;
        var_t2_dn10 = assign62440_e96976_d_n10;
        var_t2_dn13 = assign62440_e96976_d_n13;
        var_t2_rv = 0.0;

        let (assign62450_e96987, assign62450_e96987_d_n0, assign62450_e96987_d_n2, assign62450_e96987_d_n4, assign62450_e96987_d_n5, assign62450_e96987_d_n6, assign62450_e96987_d_n7, assign62450_e96987_d_n8, assign62450_e96987_d_n9, assign62450_e96987_d_n10, assign62450_e96987_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1495 != 0.0)) && (var_guard1496 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign62450_e96987;
        var_t0_dn0 = assign62450_e96987_d_n0;
        var_t0_dn2 = assign62450_e96987_d_n2;
        var_t0_dn4 = assign62450_e96987_d_n4;
        var_t0_dn5 = assign62450_e96987_d_n5;
        var_t0_dn6 = assign62450_e96987_d_n6;
        var_t0_dn7 = assign62450_e96987_d_n7;
        var_t0_dn8 = assign62450_e96987_d_n8;
        var_t0_dn9 = assign62450_e96987_d_n9;
        var_t0_dn10 = assign62450_e96987_d_n10;
        var_t0_dn13 = assign62450_e96987_d_n13;
        var_t0_rv = 0.0;

        let (assign62460_e96998, assign62460_e96998_d_n0, assign62460_e96998_d_n2, assign62460_e96998_d_n4, assign62460_e96998_d_n5, assign62460_e96998_d_n6, assign62460_e96998_d_n7, assign62460_e96998_d_n8, assign62460_e96998_d_n9, assign62460_e96998_d_n10, assign62460_e96998_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1495 != 0.0)) {
        let assign62460_e96996: f64 = (var_t2 + 1e-25);
        (assign62460_e96996, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign62460_e96998;
        var_t2_dn0 = assign62460_e96998_d_n0;
        var_t2_dn2 = assign62460_e96998_d_n2;
        var_t2_dn4 = assign62460_e96998_d_n4;
        var_t2_dn5 = assign62460_e96998_d_n5;
        var_t2_dn6 = assign62460_e96998_d_n6;
        var_t2_dn7 = assign62460_e96998_d_n7;
        var_t2_dn8 = assign62460_e96998_d_n8;
        var_t2_dn9 = assign62460_e96998_d_n9;
        var_t2_dn10 = assign62460_e96998_d_n10;
        var_t2_dn13 = assign62460_e96998_d_n13;
        var_t2_rv = 0.0;

        let (assign62470_e97009, assign62470_e97009_d_n0, assign62470_e97009_d_n2, assign62470_e97009_d_n4, assign62470_e97009_d_n5, assign62470_e97009_d_n6, assign62470_e97009_d_n7, assign62470_e97009_d_n8, assign62470_e97009_d_n9, assign62470_e97009_d_n10, assign62470_e97009_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1495 != 0.0)) {
        let assign62470_e97007: f64 = (var_beta * var_ptl0);
        (assign62470_e97007, (var_beta_dn0 * var_ptl0), (var_beta_dn2 * var_ptl0), (var_beta_dn4 * var_ptl0), (var_beta_dn5 * var_ptl0), (var_beta_dn6 * var_ptl0), (var_beta_dn7 * var_ptl0), (var_beta_dn8 * var_ptl0), (var_beta_dn9 * var_ptl0), (var_beta_dn10 * var_ptl0), (var_beta_dn13 * var_ptl0),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign62470_e97009;
        var_t0_dn0 = assign62470_e97009_d_n0;
        var_t0_dn2 = assign62470_e97009_d_n2;
        var_t0_dn4 = assign62470_e97009_d_n4;
        var_t0_dn5 = assign62470_e97009_d_n5;
        var_t0_dn6 = assign62470_e97009_d_n6;
        var_t0_dn7 = assign62470_e97009_d_n7;
        var_t0_dn8 = assign62470_e97009_d_n8;
        var_t0_dn9 = assign62470_e97009_d_n9;
        var_t0_dn10 = assign62470_e97009_d_n10;
        var_t0_dn13 = assign62470_e97009_d_n13;
        var_t0_rv = 0.0;

        let (assign62480_e97020, assign62480_e97020_d_n0, assign62480_e97020_d_n2, assign62480_e97020_d_n4, assign62480_e97020_d_n5, assign62480_e97020_d_n6, assign62480_e97020_d_n7, assign62480_e97020_d_n8, assign62480_e97020_d_n9, assign62480_e97020_d_n10, assign62480_e97020_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1495 != 0.0)) {
        let assign62480_e97018: f64 = (var_cox * var_t0);
        (assign62480_e97018, ((var_cox_dn0 * var_t0) + (var_cox * var_t0_dn0)), ((var_cox_dn2 * var_t0) + (var_cox * var_t0_dn2)), ((var_cox_dn4 * var_t0) + (var_cox * var_t0_dn4)), ((var_cox_dn5 * var_t0) + (var_cox * var_t0_dn5)), ((var_cox_dn6 * var_t0) + (var_cox * var_t0_dn6)), ((var_cox_dn7 * var_t0) + (var_cox * var_t0_dn7)), ((var_cox_dn8 * var_t0) + (var_cox * var_t0_dn8)), ((var_cox_dn9 * var_t0) + (var_cox * var_t0_dn9)), ((var_cox_dn10 * var_t0) + (var_cox * var_t0_dn10)), ((var_cox_dn13 * var_t0) + (var_cox * var_t0_dn13)),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn13,)
    }
};
        var_t3 = assign62480_e97020;
        var_t3_dn0 = assign62480_e97020_d_n0;
        var_t3_dn2 = assign62480_e97020_d_n2;
        var_t3_dn4 = assign62480_e97020_d_n4;
        var_t3_dn5 = assign62480_e97020_d_n5;
        var_t3_dn6 = assign62480_e97020_d_n6;
        var_t3_dn7 = assign62480_e97020_d_n7;
        var_t3_dn8 = assign62480_e97020_d_n8;
        var_t3_dn9 = assign62480_e97020_d_n9;
        var_t3_dn10 = assign62480_e97020_d_n10;
        var_t3_dn13 = assign62480_e97020_d_n13;
        var_t3_rv = 0.0;

        let (assign62490_e97031, assign62490_e97031_d_n0, assign62490_e97031_d_n2, assign62490_e97031_d_n4, assign62490_e97031_d_n5, assign62490_e97031_d_n6, assign62490_e97031_d_n7, assign62490_e97031_d_n8, assign62490_e97031_d_n9, assign62490_e97031_d_n10, assign62490_e97031_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1495 != 0.0)) {
        let assign62490_e97029: f64 = (var_t2).powf(p.p284);
        (assign62490_e97029, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((var_t2).powf(p.p284 - 1.0) * var_t2_dn0)) } } else { (assign62490_e97029 * (p.p284 * (var_t2_dn0 / var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((var_t2).powf(p.p284 - 1.0) * var_t2_dn2)) } } else { (assign62490_e97029 * (p.p284 * (var_t2_dn2 / var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((var_t2).powf(p.p284 - 1.0) * var_t2_dn4)) } } else { (assign62490_e97029 * (p.p284 * (var_t2_dn4 / var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((var_t2).powf(p.p284 - 1.0) * var_t2_dn5)) } } else { (assign62490_e97029 * (p.p284 * (var_t2_dn5 / var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((var_t2).powf(p.p284 - 1.0) * var_t2_dn6)) } } else { (assign62490_e97029 * (p.p284 * (var_t2_dn6 / var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((var_t2).powf(p.p284 - 1.0) * var_t2_dn7)) } } else { (assign62490_e97029 * (p.p284 * (var_t2_dn7 / var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((var_t2).powf(p.p284 - 1.0) * var_t2_dn8)) } } else { (assign62490_e97029 * (p.p284 * (var_t2_dn8 / var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((var_t2).powf(p.p284 - 1.0) * var_t2_dn9)) } } else { (assign62490_e97029 * (p.p284 * (var_t2_dn9 / var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((var_t2).powf(p.p284 - 1.0) * var_t2_dn10)) } } else { (assign62490_e97029 * (p.p284 * (var_t2_dn10 / var_t2))) }, if 0.0 == 0.0 && ((p.p284) as f64).is_finite() && ((p.p284) as f64).fract() == 0.0 { if p.p284 == 0.0 { 0.0 } else { (p.p284 * ((var_t2).powf(p.p284 - 1.0) * var_t2_dn13)) } } else { (assign62490_e97029 * (p.p284 * (var_t2_dn13 / var_t2))) },)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign62490_e97031;
        var_t0_dn0 = assign62490_e97031_d_n0;
        var_t0_dn2 = assign62490_e97031_d_n2;
        var_t0_dn4 = assign62490_e97031_d_n4;
        var_t0_dn5 = assign62490_e97031_d_n5;
        var_t0_dn6 = assign62490_e97031_d_n6;
        var_t0_dn7 = assign62490_e97031_d_n7;
        var_t0_dn8 = assign62490_e97031_d_n8;
        var_t0_dn9 = assign62490_e97031_d_n9;
        var_t0_dn10 = assign62490_e97031_d_n10;
        var_t0_dn13 = assign62490_e97031_d_n13;
        var_t0_rv = 0.0;

        let (assign62500_e97042, assign62500_e97042_d_n0, assign62500_e97042_d_n2, assign62500_e97042_d_n4, assign62500_e97042_d_n5, assign62500_e97042_d_n6, assign62500_e97042_d_n7, assign62500_e97042_d_n8, assign62500_e97042_d_n9, assign62500_e97042_d_n10, assign62500_e97042_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1495 != 0.0)) {
        let assign62500_e97040: f64 = (var_t3 * var_t0);
        (assign62500_e97040, ((var_t3_dn0 * var_t0) + (var_t3 * var_t0_dn0)), ((var_t3_dn2 * var_t0) + (var_t3 * var_t0_dn2)), ((var_t3_dn4 * var_t0) + (var_t3 * var_t0_dn4)), ((var_t3_dn5 * var_t0) + (var_t3 * var_t0_dn5)), ((var_t3_dn6 * var_t0) + (var_t3 * var_t0_dn6)), ((var_t3_dn7 * var_t0) + (var_t3 * var_t0_dn7)), ((var_t3_dn8 * var_t0) + (var_t3 * var_t0_dn8)), ((var_t3_dn9 * var_t0) + (var_t3 * var_t0_dn9)), ((var_t3_dn10 * var_t0) + (var_t3 * var_t0_dn10)), ((var_t3_dn13 * var_t0) + (var_t3 * var_t0_dn13)),)
    } else {
        (var_t9, var_t9_dn0, var_t9_dn2, var_t9_dn4, var_t9_dn5, var_t9_dn6, var_t9_dn7, var_t9_dn8, var_t9_dn9, var_t9_dn10, var_t9_dn13,)
    }
};
        var_t9 = assign62500_e97042;
        var_t9_dn0 = assign62500_e97042_d_n0;
        var_t9_dn2 = assign62500_e97042_d_n2;
        var_t9_dn4 = assign62500_e97042_d_n4;
        var_t9_dn5 = assign62500_e97042_d_n5;
        var_t9_dn6 = assign62500_e97042_d_n6;
        var_t9_dn7 = assign62500_e97042_d_n7;
        var_t9_dn8 = assign62500_e97042_d_n8;
        var_t9_dn9 = assign62500_e97042_d_n9;
        var_t9_dn10 = assign62500_e97042_d_n10;
        var_t9_dn13 = assign62500_e97042_d_n13;
        var_t9_rv = 0.0;

        let (assign62510_e97055, assign62510_e97055_d_n0, assign62510_e97055_d_n2, assign62510_e97055_d_n4, assign62510_e97055_d_n5, assign62510_e97055_d_n6, assign62510_e97055_d_n7, assign62510_e97055_d_n8, assign62510_e97055_d_n9, assign62510_e97055_d_n10, assign62510_e97055_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1495 != 0.0)) {
        let assign62510_e97052: f64 = (var_vdsz__blk439 * p.p285);
        let assign62510_e97053: f64 = (1.0 + assign62510_e97052);
        (assign62510_e97053, (var_vdsz__blk439_dn0 * p.p285), (var_vdsz__blk439_dn2 * p.p285), (var_vdsz__blk439_dn4 * p.p285), (var_vdsz__blk439_dn5 * p.p285), (var_vdsz__blk439_dn6 * p.p285), (var_vdsz__blk439_dn7 * p.p285), (var_vdsz__blk439_dn8 * p.p285), (var_vdsz__blk439_dn9 * p.p285), (var_vdsz__blk439_dn10 * p.p285), (var_vdsz__blk439_dn13 * p.p285),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn13,)
    }
};
        var_t4 = assign62510_e97055;
        var_t4_dn0 = assign62510_e97055_d_n0;
        var_t4_dn2 = assign62510_e97055_d_n2;
        var_t4_dn4 = assign62510_e97055_d_n4;
        var_t4_dn5 = assign62510_e97055_d_n5;
        var_t4_dn6 = assign62510_e97055_d_n6;
        var_t4_dn7 = assign62510_e97055_d_n7;
        var_t4_dn8 = assign62510_e97055_d_n8;
        var_t4_dn9 = assign62510_e97055_d_n9;
        var_t4_dn10 = assign62510_e97055_d_n10;
        var_t4_dn13 = assign62510_e97055_d_n13;
        var_t4_rv = 0.0;

        let (assign62520_e97064, assign62520_e97064_d_n0, assign62520_e97064_d_n2, assign62520_e97064_d_n4, assign62520_e97064_d_n5, assign62520_e97064_d_n6, assign62520_e97064_d_n7, assign62520_e97064_d_n8, assign62520_e97064_d_n9, assign62520_e97064_d_n10, assign62520_e97064_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1495 != 0.0)) {
        (var_pt40, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign62520_e97064;
        var_t0_dn0 = assign62520_e97064_d_n0;
        var_t0_dn2 = assign62520_e97064_d_n2;
        var_t0_dn4 = assign62520_e97064_d_n4;
        var_t0_dn5 = assign62520_e97064_d_n5;
        var_t0_dn6 = assign62520_e97064_d_n6;
        var_t0_dn7 = assign62520_e97064_d_n7;
        var_t0_dn8 = assign62520_e97064_d_n8;
        var_t0_dn9 = assign62520_e97064_d_n9;
        var_t0_dn10 = assign62520_e97064_d_n10;
        var_t0_dn13 = assign62520_e97064_d_n13;
        var_t0_rv = 0.0;

        let (assign62530_e97077, assign62530_e97077_d_n0, assign62530_e97077_d_n2, assign62530_e97077_d_n4, assign62530_e97077_d_n5, assign62530_e97077_d_n6, assign62530_e97077_d_n7, assign62530_e97077_d_n8, assign62530_e97077_d_n9, assign62530_e97077_d_n10, assign62530_e97077_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1495 != 0.0)) {
        let assign62530_e97073: f64 = (var_ps0 + var_t6);
        let assign62530_e97075: f64 = (assign62530_e97073 - var_vbsz__blk438);
        (assign62530_e97075, ((var_ps0_dn0 + var_t6_dn0) - var_vbsz__blk438_dn0), ((var_ps0_dn2 + var_t6_dn2) - var_vbsz__blk438_dn2), ((var_ps0_dn4 + var_t6_dn4) - var_vbsz__blk438_dn4), ((var_ps0_dn5 + var_t6_dn5) - var_vbsz__blk438_dn5), ((var_ps0_dn6 + var_t6_dn6) - var_vbsz__blk438_dn6), ((var_ps0_dn7 + var_t6_dn7) - var_vbsz__blk438_dn7), ((var_ps0_dn8 + var_t6_dn8) - var_vbsz__blk438_dn8), ((var_ps0_dn9 + var_t6_dn9) - var_vbsz__blk438_dn9), ((var_ps0_dn10 + var_t6_dn10) - var_vbsz__blk438_dn10), ((var_ps0_dn13 + var_t6_dn13) - var_vbsz__blk438_dn13),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn13,)
    }
};
        var_t5 = assign62530_e97077;
        var_t5_dn0 = assign62530_e97077_d_n0;
        var_t5_dn2 = assign62530_e97077_d_n2;
        var_t5_dn4 = assign62530_e97077_d_n4;
        var_t5_dn5 = assign62530_e97077_d_n5;
        var_t5_dn6 = assign62530_e97077_d_n6;
        var_t5_dn7 = assign62530_e97077_d_n7;
        var_t5_dn8 = assign62530_e97077_d_n8;
        var_t5_dn9 = assign62530_e97077_d_n9;
        var_t5_dn10 = assign62530_e97077_d_n10;
        var_t5_dn13 = assign62530_e97077_d_n13;
        var_t5_rv = 0.0;

        let (assign62540_e97092, assign62540_e97092_d_n0, assign62540_e97092_d_n2, assign62540_e97092_d_n4, assign62540_e97092_d_n5, assign62540_e97092_d_n6, assign62540_e97092_d_n7, assign62540_e97092_d_n8, assign62540_e97092_d_n9, assign62540_e97092_d_n10, assign62540_e97092_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1495 != 0.0)) {
        let assign62540_e97087: f64 = (var_vdsz__blk439 * var_t0);
        let assign62540_e97089: f64 = (assign62540_e97087 * var_t5);
        let assign62540_e97090: f64 = (var_t4 + assign62540_e97089);
        (assign62540_e97090, (var_t4_dn0 + ((((var_vdsz__blk439_dn0 * var_t0) + (var_vdsz__blk439 * var_t0_dn0)) * var_t5) + (assign62540_e97087 * var_t5_dn0))), (var_t4_dn2 + ((((var_vdsz__blk439_dn2 * var_t0) + (var_vdsz__blk439 * var_t0_dn2)) * var_t5) + (assign62540_e97087 * var_t5_dn2))), (var_t4_dn4 + ((((var_vdsz__blk439_dn4 * var_t0) + (var_vdsz__blk439 * var_t0_dn4)) * var_t5) + (assign62540_e97087 * var_t5_dn4))), (var_t4_dn5 + ((((var_vdsz__blk439_dn5 * var_t0) + (var_vdsz__blk439 * var_t0_dn5)) * var_t5) + (assign62540_e97087 * var_t5_dn5))), (var_t4_dn6 + ((((var_vdsz__blk439_dn6 * var_t0) + (var_vdsz__blk439 * var_t0_dn6)) * var_t5) + (assign62540_e97087 * var_t5_dn6))), (var_t4_dn7 + ((((var_vdsz__blk439_dn7 * var_t0) + (var_vdsz__blk439 * var_t0_dn7)) * var_t5) + (assign62540_e97087 * var_t5_dn7))), (var_t4_dn8 + ((((var_vdsz__blk439_dn8 * var_t0) + (var_vdsz__blk439 * var_t0_dn8)) * var_t5) + (assign62540_e97087 * var_t5_dn8))), (var_t4_dn9 + ((((var_vdsz__blk439_dn9 * var_t0) + (var_vdsz__blk439 * var_t0_dn9)) * var_t5) + (assign62540_e97087 * var_t5_dn9))), (var_t4_dn10 + ((((var_vdsz__blk439_dn10 * var_t0) + (var_vdsz__blk439 * var_t0_dn10)) * var_t5) + (assign62540_e97087 * var_t5_dn10))), (var_t4_dn13 + ((((var_vdsz__blk439_dn13 * var_t0) + (var_vdsz__blk439 * var_t0_dn13)) * var_t5) + (assign62540_e97087 * var_t5_dn13))),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn13,)
    }
};
        var_t4 = assign62540_e97092;
        var_t4_dn0 = assign62540_e97092_d_n0;
        var_t4_dn2 = assign62540_e97092_d_n2;
        var_t4_dn4 = assign62540_e97092_d_n4;
        var_t4_dn5 = assign62540_e97092_d_n5;
        var_t4_dn6 = assign62540_e97092_d_n6;
        var_t4_dn7 = assign62540_e97092_d_n7;
        var_t4_dn8 = assign62540_e97092_d_n8;
        var_t4_dn9 = assign62540_e97092_d_n9;
        var_t4_dn10 = assign62540_e97092_d_n10;
        var_t4_dn13 = assign62540_e97092_d_n13;
        var_t4_rv = 0.0;

        *var_guard1496_slot = var_guard1496;
        *var_guard1496_rv_slot = var_guard1496_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn13_slot = var_t0_dn13;
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
        *var_t1_dn13_slot = var_t1_dn13;
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
        *var_t2_dn13_slot = var_t2_dn13;
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
        *var_t3_dn13_slot = var_t3_dn13;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t3_rv_slot = var_t3_rv;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn13_slot = var_t4_dn13;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_dn9_slot = var_t4_dn9;
        *var_t4_rv_slot = var_t4_rv;
        *var_t5_slot = var_t5;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn10_slot = var_t5_dn10;
        *var_t5_dn13_slot = var_t5_dn13;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t5_dn9_slot = var_t5_dn9;
        *var_t5_rv_slot = var_t5_rv;
        *var_t6_slot = var_t6;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn13_slot = var_t6_dn13;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t6_dn9_slot = var_t6_dn9;
        *var_t6_rv_slot = var_t6_rv;
        *var_t9_slot = var_t9;
        *var_t9_dn0_slot = var_t9_dn0;
        *var_t9_dn10_slot = var_t9_dn10;
        *var_t9_dn13_slot = var_t9_dn13;
        *var_t9_dn2_slot = var_t9_dn2;
        *var_t9_dn4_slot = var_t9_dn4;
        *var_t9_dn5_slot = var_t9_dn5;
        *var_t9_dn6_slot = var_t9_dn6;
        *var_t9_dn7_slot = var_t9_dn7;
        *var_t9_dn8_slot = var_t9_dn8;
        *var_t9_dn9_slot = var_t9_dn9;
        *var_t9_rv_slot = var_t9_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn13_slot = var_tmf1_dn13;
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
        *var_tmf2_dn13_slot = var_tmf2_dn13;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn7_slot = var_tmf2_dn7;
        *var_tmf2_dn8_slot = var_tmf2_dn8;
        *var_tmf2_dn9_slot = var_tmf2_dn9;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_tmf3_slot = var_tmf3;
        *var_tmf3_dn0_slot = var_tmf3_dn0;
        *var_tmf3_dn10_slot = var_tmf3_dn10;
        *var_tmf3_dn13_slot = var_tmf3_dn13;
        *var_tmf3_dn2_slot = var_tmf3_dn2;
        *var_tmf3_dn4_slot = var_tmf3_dn4;
        *var_tmf3_dn5_slot = var_tmf3_dn5;
        *var_tmf3_dn6_slot = var_tmf3_dn6;
        *var_tmf3_dn7_slot = var_tmf3_dn7;
        *var_tmf3_dn8_slot = var_tmf3_dn8;
        *var_tmf3_dn9_slot = var_tmf3_dn9;
        *var_tmf3_rv_slot = var_tmf3_rv;
    }

    pub(super) fn stamp_reactive_block_224(
        p: &Parameters,
        var_beta: f64,
        var_beta_dn0: f64,
        var_beta_dn10: f64,
        var_beta_dn13: f64,
        var_beta_dn2: f64,
        var_beta_dn4: f64,
        var_beta_dn5: f64,
        var_beta_dn6: f64,
        var_beta_dn7: f64,
        var_beta_dn8: f64,
        var_beta_dn9: f64,
        var_betawl: f64,
        var_betawl_dn0: f64,
        var_betawl_dn10: f64,
        var_betawl_dn13: f64,
        var_betawl_dn2: f64,
        var_betawl_dn4: f64,
        var_betawl_dn5: f64,
        var_betawl_dn6: f64,
        var_betawl_dn7: f64,
        var_betawl_dn8: f64,
        var_betawl_dn9: f64,
        var_cox: f64,
        var_cox_dn0: f64,
        var_cox_dn10: f64,
        var_cox_dn13: f64,
        var_cox_dn2: f64,
        var_cox_dn4: f64,
        var_cox_dn5: f64,
        var_cox_dn6: f64,
        var_cox_dn7: f64,
        var_cox_dn8: f64,
        var_cox_dn9: f64,
        var_flg_rsrd: f64,
        var_gdl0: f64,
        var_guard1430: f64,
        var_guard1495: f64,
        var_guard443: f64,
        var_mu: f64,
        var_mu_dn0: f64,
        var_mu_dn10: f64,
        var_mu_dn13: f64,
        var_mu_dn2: f64,
        var_mu_dn4: f64,
        var_mu_dn5: f64,
        var_mu_dn6: f64,
        var_mu_dn7: f64,
        var_mu_dn8: f64,
        var_mu_dn9: f64,
        var_pds: f64,
        var_pds_dn0: f64,
        var_pds_dn10: f64,
        var_pds_dn13: f64,
        var_pds_dn2: f64,
        var_pds_dn4: f64,
        var_pds_dn5: f64,
        var_pds_dn6: f64,
        var_pds_dn7: f64,
        var_pds_dn8: f64,
        var_pds_dn9: f64,
        var_rd23e: f64,
        var_rd23e_dn0: f64,
        var_rd23e_dn10: f64,
        var_rd23e_dn13: f64,
        var_rd23e_dn2: f64,
        var_rd23e_dn4: f64,
        var_rd23e_dn5: f64,
        var_rd23e_dn6: f64,
        var_rd23e_dn7: f64,
        var_rd23e_dn8: f64,
        var_rd23e_dn9: f64,
        var_uc_rd24: f64,
        var_vdsz__blk439: f64,
        var_vdsz__blk439_dn0: f64,
        var_vdsz__blk439_dn10: f64,
        var_vdsz__blk439_dn13: f64,
        var_vdsz__blk439_dn2: f64,
        var_vdsz__blk439_dn4: f64,
        var_vdsz__blk439_dn5: f64,
        var_vdsz__blk439_dn6: f64,
        var_vdsz__blk439_dn7: f64,
        var_vdsz__blk439_dn8: f64,
        var_vdsz__blk439_dn9: f64,
        var_vgse: f64,
        var_vgse_dn0: f64,
        var_vgse_dn2: f64,
        var_vgse_dn6: f64,
        var_guard1497_slot: &mut f64,
        var_guard1497_rv_slot: &mut f64,
        var_guard1498_slot: &mut f64,
        var_guard1498_rv_slot: &mut f64,
        var_guard1499_slot: &mut f64,
        var_guard1499_rv_slot: &mut f64,
        var_guard1500_slot: &mut f64,
        var_guard1500_rv_slot: &mut f64,
        var_idd1_slot: &mut f64,
        var_idd1_dn0_slot: &mut f64,
        var_idd1_dn10_slot: &mut f64,
        var_idd1_dn13_slot: &mut f64,
        var_idd1_dn2_slot: &mut f64,
        var_idd1_dn4_slot: &mut f64,
        var_idd1_dn5_slot: &mut f64,
        var_idd1_dn6_slot: &mut f64,
        var_idd1_dn7_slot: &mut f64,
        var_idd1_dn8_slot: &mut f64,
        var_idd1_dn9_slot: &mut f64,
        var_idd1_rv_slot: &mut f64,
        var_ids0_slot: &mut f64,
        var_ids0_dn0_slot: &mut f64,
        var_ids0_dn10_slot: &mut f64,
        var_ids0_dn13_slot: &mut f64,
        var_ids0_dn2_slot: &mut f64,
        var_ids0_dn4_slot: &mut f64,
        var_ids0_dn5_slot: &mut f64,
        var_ids0_dn6_slot: &mut f64,
        var_ids0_dn7_slot: &mut f64,
        var_ids0_dn8_slot: &mut f64,
        var_ids0_dn9_slot: &mut f64,
        var_ids0_rv_slot: &mut f64,
        var_idspt0_slot: &mut f64,
        var_idspt0_dn0_slot: &mut f64,
        var_idspt0_dn10_slot: &mut f64,
        var_idspt0_dn13_slot: &mut f64,
        var_idspt0_dn2_slot: &mut f64,
        var_idspt0_dn4_slot: &mut f64,
        var_idspt0_dn5_slot: &mut f64,
        var_idspt0_dn6_slot: &mut f64,
        var_idspt0_dn7_slot: &mut f64,
        var_idspt0_dn8_slot: &mut f64,
        var_idspt0_dn9_slot: &mut f64,
        var_idspt0_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn13_slot: &mut f64,
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
        var_t1_dn13_slot: &mut f64,
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
        var_t2_dn13_slot: &mut f64,
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
        var_t3_dn13_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn13_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_dn9_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn13_slot: &mut f64,
        var_t6_dn2_slot: &mut f64,
        var_t6_dn4_slot: &mut f64,
        var_t6_dn5_slot: &mut f64,
        var_t6_dn6_slot: &mut f64,
        var_t6_dn7_slot: &mut f64,
        var_t6_dn8_slot: &mut f64,
        var_t6_dn9_slot: &mut f64,
        var_t6_rv_slot: &mut f64,
        var_t8_slot: &mut f64,
        var_t8_dn0_slot: &mut f64,
        var_t8_dn10_slot: &mut f64,
        var_t8_dn13_slot: &mut f64,
        var_t8_dn2_slot: &mut f64,
        var_t8_dn4_slot: &mut f64,
        var_t8_dn5_slot: &mut f64,
        var_t8_dn6_slot: &mut f64,
        var_t8_dn7_slot: &mut f64,
        var_t8_dn8_slot: &mut f64,
        var_t8_dn9_slot: &mut f64,
        var_t8_rv_slot: &mut f64,
        var_t9_slot: &mut f64,
        var_t9_dn0_slot: &mut f64,
        var_t9_dn10_slot: &mut f64,
        var_t9_dn13_slot: &mut f64,
        var_t9_dn2_slot: &mut f64,
        var_t9_dn4_slot: &mut f64,
        var_t9_dn5_slot: &mut f64,
        var_t9_dn6_slot: &mut f64,
        var_t9_dn7_slot: &mut f64,
        var_t9_dn8_slot: &mut f64,
        var_t9_dn9_slot: &mut f64,
        var_t9_rv_slot: &mut f64,
        var_tmf1_slot: &mut f64,
        var_tmf1_dn0_slot: &mut f64,
        var_tmf1_dn10_slot: &mut f64,
        var_tmf1_dn13_slot: &mut f64,
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
        var_tmf2_dn13_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn7_slot: &mut f64,
        var_tmf2_dn8_slot: &mut f64,
        var_tmf2_dn9_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
    ) {
        let mut var_guard1497: f64 = *var_guard1497_slot;
        let mut var_guard1497_rv: f64 = *var_guard1497_rv_slot;
        let mut var_guard1498: f64 = *var_guard1498_slot;
        let mut var_guard1498_rv: f64 = *var_guard1498_rv_slot;
        let mut var_guard1499: f64 = *var_guard1499_slot;
        let mut var_guard1499_rv: f64 = *var_guard1499_rv_slot;
        let mut var_guard1500: f64 = *var_guard1500_slot;
        let mut var_guard1500_rv: f64 = *var_guard1500_rv_slot;
        let mut var_idd1: f64 = *var_idd1_slot;
        let mut var_idd1_dn0: f64 = *var_idd1_dn0_slot;
        let mut var_idd1_dn10: f64 = *var_idd1_dn10_slot;
        let mut var_idd1_dn13: f64 = *var_idd1_dn13_slot;
        let mut var_idd1_dn2: f64 = *var_idd1_dn2_slot;
        let mut var_idd1_dn4: f64 = *var_idd1_dn4_slot;
        let mut var_idd1_dn5: f64 = *var_idd1_dn5_slot;
        let mut var_idd1_dn6: f64 = *var_idd1_dn6_slot;
        let mut var_idd1_dn7: f64 = *var_idd1_dn7_slot;
        let mut var_idd1_dn8: f64 = *var_idd1_dn8_slot;
        let mut var_idd1_dn9: f64 = *var_idd1_dn9_slot;
        let mut var_idd1_rv: f64 = *var_idd1_rv_slot;
        let mut var_ids0: f64 = *var_ids0_slot;
        let mut var_ids0_dn0: f64 = *var_ids0_dn0_slot;
        let mut var_ids0_dn10: f64 = *var_ids0_dn10_slot;
        let mut var_ids0_dn13: f64 = *var_ids0_dn13_slot;
        let mut var_ids0_dn2: f64 = *var_ids0_dn2_slot;
        let mut var_ids0_dn4: f64 = *var_ids0_dn4_slot;
        let mut var_ids0_dn5: f64 = *var_ids0_dn5_slot;
        let mut var_ids0_dn6: f64 = *var_ids0_dn6_slot;
        let mut var_ids0_dn7: f64 = *var_ids0_dn7_slot;
        let mut var_ids0_dn8: f64 = *var_ids0_dn8_slot;
        let mut var_ids0_dn9: f64 = *var_ids0_dn9_slot;
        let mut var_ids0_rv: f64 = *var_ids0_rv_slot;
        let mut var_idspt0: f64 = *var_idspt0_slot;
        let mut var_idspt0_dn0: f64 = *var_idspt0_dn0_slot;
        let mut var_idspt0_dn10: f64 = *var_idspt0_dn10_slot;
        let mut var_idspt0_dn13: f64 = *var_idspt0_dn13_slot;
        let mut var_idspt0_dn2: f64 = *var_idspt0_dn2_slot;
        let mut var_idspt0_dn4: f64 = *var_idspt0_dn4_slot;
        let mut var_idspt0_dn5: f64 = *var_idspt0_dn5_slot;
        let mut var_idspt0_dn6: f64 = *var_idspt0_dn6_slot;
        let mut var_idspt0_dn7: f64 = *var_idspt0_dn7_slot;
        let mut var_idspt0_dn8: f64 = *var_idspt0_dn8_slot;
        let mut var_idspt0_dn9: f64 = *var_idspt0_dn9_slot;
        let mut var_idspt0_rv: f64 = *var_idspt0_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn13: f64 = *var_t0_dn13_slot;
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
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
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
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
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
        let mut var_t3_dn13: f64 = *var_t3_dn13_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn13: f64 = *var_t4_dn13_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_dn9: f64 = *var_t4_dn9_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn13: f64 = *var_t6_dn13_slot;
        let mut var_t6_dn2: f64 = *var_t6_dn2_slot;
        let mut var_t6_dn4: f64 = *var_t6_dn4_slot;
        let mut var_t6_dn5: f64 = *var_t6_dn5_slot;
        let mut var_t6_dn6: f64 = *var_t6_dn6_slot;
        let mut var_t6_dn7: f64 = *var_t6_dn7_slot;
        let mut var_t6_dn8: f64 = *var_t6_dn8_slot;
        let mut var_t6_dn9: f64 = *var_t6_dn9_slot;
        let mut var_t6_rv: f64 = *var_t6_rv_slot;
        let mut var_t8: f64 = *var_t8_slot;
        let mut var_t8_dn0: f64 = *var_t8_dn0_slot;
        let mut var_t8_dn10: f64 = *var_t8_dn10_slot;
        let mut var_t8_dn13: f64 = *var_t8_dn13_slot;
        let mut var_t8_dn2: f64 = *var_t8_dn2_slot;
        let mut var_t8_dn4: f64 = *var_t8_dn4_slot;
        let mut var_t8_dn5: f64 = *var_t8_dn5_slot;
        let mut var_t8_dn6: f64 = *var_t8_dn6_slot;
        let mut var_t8_dn7: f64 = *var_t8_dn7_slot;
        let mut var_t8_dn8: f64 = *var_t8_dn8_slot;
        let mut var_t8_dn9: f64 = *var_t8_dn9_slot;
        let mut var_t8_rv: f64 = *var_t8_rv_slot;
        let mut var_t9: f64 = *var_t9_slot;
        let mut var_t9_dn0: f64 = *var_t9_dn0_slot;
        let mut var_t9_dn10: f64 = *var_t9_dn10_slot;
        let mut var_t9_dn13: f64 = *var_t9_dn13_slot;
        let mut var_t9_dn2: f64 = *var_t9_dn2_slot;
        let mut var_t9_dn4: f64 = *var_t9_dn4_slot;
        let mut var_t9_dn5: f64 = *var_t9_dn5_slot;
        let mut var_t9_dn6: f64 = *var_t9_dn6_slot;
        let mut var_t9_dn7: f64 = *var_t9_dn7_slot;
        let mut var_t9_dn8: f64 = *var_t9_dn8_slot;
        let mut var_t9_dn9: f64 = *var_t9_dn9_slot;
        let mut var_t9_rv: f64 = *var_t9_rv_slot;
        let mut var_tmf1: f64 = *var_tmf1_slot;
        let mut var_tmf1_dn0: f64 = *var_tmf1_dn0_slot;
        let mut var_tmf1_dn10: f64 = *var_tmf1_dn10_slot;
        let mut var_tmf1_dn13: f64 = *var_tmf1_dn13_slot;
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
        let mut var_tmf2_dn13: f64 = *var_tmf2_dn13_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn7: f64 = *var_tmf2_dn7_slot;
        let mut var_tmf2_dn8: f64 = *var_tmf2_dn8_slot;
        let mut var_tmf2_dn9: f64 = *var_tmf2_dn9_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;

        let (assign62550_e97103, assign62550_e97103_d_n0, assign62550_e97103_d_n2, assign62550_e97103_d_n4, assign62550_e97103_d_n5, assign62550_e97103_d_n6, assign62550_e97103_d_n7, assign62550_e97103_d_n8, assign62550_e97103_d_n9, assign62550_e97103_d_n10, assign62550_e97103_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1495 != 0.0)) {
        let assign62550_e97101: f64 = (var_t9 * var_t4);
        (assign62550_e97101, ((var_t9_dn0 * var_t4) + (var_t9 * var_t4_dn0)), ((var_t9_dn2 * var_t4) + (var_t9 * var_t4_dn2)), ((var_t9_dn4 * var_t4) + (var_t9 * var_t4_dn4)), ((var_t9_dn5 * var_t4) + (var_t9 * var_t4_dn5)), ((var_t9_dn6 * var_t4) + (var_t9 * var_t4_dn6)), ((var_t9_dn7 * var_t4) + (var_t9 * var_t4_dn7)), ((var_t9_dn8 * var_t4) + (var_t9 * var_t4_dn8)), ((var_t9_dn9 * var_t4) + (var_t9 * var_t4_dn9)), ((var_t9_dn10 * var_t4) + (var_t9 * var_t4_dn10)), ((var_t9_dn13 * var_t4) + (var_t9 * var_t4_dn13)),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn13,)
    }
};
        var_t6 = assign62550_e97103;
        var_t6_dn0 = assign62550_e97103_d_n0;
        var_t6_dn2 = assign62550_e97103_d_n2;
        var_t6_dn4 = assign62550_e97103_d_n4;
        var_t6_dn5 = assign62550_e97103_d_n5;
        var_t6_dn6 = assign62550_e97103_d_n6;
        var_t6_dn7 = assign62550_e97103_d_n7;
        var_t6_dn8 = assign62550_e97103_d_n8;
        var_t6_dn9 = assign62550_e97103_d_n9;
        var_t6_dn10 = assign62550_e97103_d_n10;
        var_t6_dn13 = assign62550_e97103_d_n13;
        var_t6_rv = 0.0;

        let (assign62560_e97112, assign62560_e97112_d_n0, assign62560_e97112_d_n2, assign62560_e97112_d_n4, assign62560_e97112_d_n5, assign62560_e97112_d_n6, assign62560_e97112_d_n7, assign62560_e97112_d_n8, assign62560_e97112_d_n9, assign62560_e97112_d_n10, assign62560_e97112_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1495 != 0.0)) {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn13,)
    } else {
        (var_t9, var_t9_dn0, var_t9_dn2, var_t9_dn4, var_t9_dn5, var_t9_dn6, var_t9_dn7, var_t9_dn8, var_t9_dn9, var_t9_dn10, var_t9_dn13,)
    }
};
        var_t9 = assign62560_e97112;
        var_t9_dn0 = assign62560_e97112_d_n0;
        var_t9_dn2 = assign62560_e97112_d_n2;
        var_t9_dn4 = assign62560_e97112_d_n4;
        var_t9_dn5 = assign62560_e97112_d_n5;
        var_t9_dn6 = assign62560_e97112_d_n6;
        var_t9_dn7 = assign62560_e97112_d_n7;
        var_t9_dn8 = assign62560_e97112_d_n8;
        var_t9_dn9 = assign62560_e97112_d_n9;
        var_t9_dn10 = assign62560_e97112_d_n10;
        var_t9_dn13 = assign62560_e97112_d_n13;
        var_t9_rv = 0.0;

        let (assign62570_e97122, assign62570_e97122_d_n0, assign62570_e97122_d_n2, assign62570_e97122_d_n4, assign62570_e97122_d_n5, assign62570_e97122_d_n6, assign62570_e97122_d_n7, assign62570_e97122_d_n8, assign62570_e97122_d_n9, assign62570_e97122_d_n10, assign62570_e97122_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1495 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t9, var_t9_dn0, var_t9_dn2, var_t9_dn4, var_t9_dn5, var_t9_dn6, var_t9_dn7, var_t9_dn8, var_t9_dn9, var_t9_dn10, var_t9_dn13,)
    }
};
        var_t9 = assign62570_e97122;
        var_t9_dn0 = assign62570_e97122_d_n0;
        var_t9_dn2 = assign62570_e97122_d_n2;
        var_t9_dn4 = assign62570_e97122_d_n4;
        var_t9_dn5 = assign62570_e97122_d_n5;
        var_t9_dn6 = assign62570_e97122_d_n6;
        var_t9_dn7 = assign62570_e97122_d_n7;
        var_t9_dn8 = assign62570_e97122_d_n8;
        var_t9_dn9 = assign62570_e97122_d_n9;
        var_t9_dn10 = assign62570_e97122_d_n10;
        var_t9_dn13 = assign62570_e97122_d_n13;
        var_t9_rv = 0.0;

        let assign62580_e97125: f64 = if p.p287 != 0.0 { 1.0 } else { 0.0 };
        var_guard1497 = assign62580_e97125;
        var_guard1497_rv = 0.0;

        let (assign62590_e97136, assign62590_e97136_d_n0, assign62590_e97136_d_n2, assign62590_e97136_d_n4, assign62590_e97136_d_n5, assign62590_e97136_d_n6, assign62590_e97136_d_n7, assign62590_e97136_d_n8, assign62590_e97136_d_n9, assign62590_e97136_d_n10, assign62590_e97136_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1497 != 0.0)) {
        let assign62590_e97134: f64 = (var_beta * var_gdl0);
        (assign62590_e97134, (var_beta_dn0 * var_gdl0), (var_beta_dn2 * var_gdl0), (var_beta_dn4 * var_gdl0), (var_beta_dn5 * var_gdl0), (var_beta_dn6 * var_gdl0), (var_beta_dn7 * var_gdl0), (var_beta_dn8 * var_gdl0), (var_beta_dn9 * var_gdl0), (var_beta_dn10 * var_gdl0), (var_beta_dn13 * var_gdl0),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign62590_e97136;
        var_t1_dn0 = assign62590_e97136_d_n0;
        var_t1_dn2 = assign62590_e97136_d_n2;
        var_t1_dn4 = assign62590_e97136_d_n4;
        var_t1_dn5 = assign62590_e97136_d_n5;
        var_t1_dn6 = assign62590_e97136_d_n6;
        var_t1_dn7 = assign62590_e97136_d_n7;
        var_t1_dn8 = assign62590_e97136_d_n8;
        var_t1_dn9 = assign62590_e97136_d_n9;
        var_t1_dn10 = assign62590_e97136_d_n10;
        var_t1_dn13 = assign62590_e97136_d_n13;
        var_t1_rv = 0.0;

        let (assign62600_e97147, assign62600_e97147_d_n0, assign62600_e97147_d_n2, assign62600_e97147_d_n4, assign62600_e97147_d_n5, assign62600_e97147_d_n6, assign62600_e97147_d_n7, assign62600_e97147_d_n8, assign62600_e97147_d_n9, assign62600_e97147_d_n10, assign62600_e97147_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1497 != 0.0)) {
        let assign62600_e97145: f64 = (var_cox * var_t1);
        (assign62600_e97145, ((var_cox_dn0 * var_t1) + (var_cox * var_t1_dn0)), ((var_cox_dn2 * var_t1) + (var_cox * var_t1_dn2)), ((var_cox_dn4 * var_t1) + (var_cox * var_t1_dn4)), ((var_cox_dn5 * var_t1) + (var_cox * var_t1_dn5)), ((var_cox_dn6 * var_t1) + (var_cox * var_t1_dn6)), ((var_cox_dn7 * var_t1) + (var_cox * var_t1_dn7)), ((var_cox_dn8 * var_t1) + (var_cox * var_t1_dn8)), ((var_cox_dn9 * var_t1) + (var_cox * var_t1_dn9)), ((var_cox_dn10 * var_t1) + (var_cox * var_t1_dn10)), ((var_cox_dn13 * var_t1) + (var_cox * var_t1_dn13)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign62600_e97147;
        var_t2_dn0 = assign62600_e97147_d_n0;
        var_t2_dn2 = assign62600_e97147_d_n2;
        var_t2_dn4 = assign62600_e97147_d_n4;
        var_t2_dn5 = assign62600_e97147_d_n5;
        var_t2_dn6 = assign62600_e97147_d_n6;
        var_t2_dn7 = assign62600_e97147_d_n7;
        var_t2_dn8 = assign62600_e97147_d_n8;
        var_t2_dn9 = assign62600_e97147_d_n9;
        var_t2_dn10 = assign62600_e97147_d_n10;
        var_t2_dn13 = assign62600_e97147_d_n13;
        var_t2_rv = 0.0;

        let (assign62610_e97158, assign62610_e97158_d_n0, assign62610_e97158_d_n2, assign62610_e97158_d_n4, assign62610_e97158_d_n5, assign62610_e97158_d_n6, assign62610_e97158_d_n7, assign62610_e97158_d_n8, assign62610_e97158_d_n9, assign62610_e97158_d_n10, assign62610_e97158_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1497 != 0.0)) {
        let assign62610_e97156: f64 = (var_t2 * var_vdsz__blk439);
        (assign62610_e97156, ((var_t2_dn0 * var_vdsz__blk439) + (var_t2 * var_vdsz__blk439_dn0)), ((var_t2_dn2 * var_vdsz__blk439) + (var_t2 * var_vdsz__blk439_dn2)), ((var_t2_dn4 * var_vdsz__blk439) + (var_t2 * var_vdsz__blk439_dn4)), ((var_t2_dn5 * var_vdsz__blk439) + (var_t2 * var_vdsz__blk439_dn5)), ((var_t2_dn6 * var_vdsz__blk439) + (var_t2 * var_vdsz__blk439_dn6)), ((var_t2_dn7 * var_vdsz__blk439) + (var_t2 * var_vdsz__blk439_dn7)), ((var_t2_dn8 * var_vdsz__blk439) + (var_t2 * var_vdsz__blk439_dn8)), ((var_t2_dn9 * var_vdsz__blk439) + (var_t2 * var_vdsz__blk439_dn9)), ((var_t2_dn10 * var_vdsz__blk439) + (var_t2 * var_vdsz__blk439_dn10)), ((var_t2_dn13 * var_vdsz__blk439) + (var_t2 * var_vdsz__blk439_dn13)),)
    } else {
        (var_t8, var_t8_dn0, var_t8_dn2, var_t8_dn4, var_t8_dn5, var_t8_dn6, var_t8_dn7, var_t8_dn8, var_t8_dn9, var_t8_dn10, var_t8_dn13,)
    }
};
        var_t8 = assign62610_e97158;
        var_t8_dn0 = assign62610_e97158_d_n0;
        var_t8_dn2 = assign62610_e97158_d_n2;
        var_t8_dn4 = assign62610_e97158_d_n4;
        var_t8_dn5 = assign62610_e97158_d_n5;
        var_t8_dn6 = assign62610_e97158_d_n6;
        var_t8_dn7 = assign62610_e97158_d_n7;
        var_t8_dn8 = assign62610_e97158_d_n8;
        var_t8_dn9 = assign62610_e97158_d_n9;
        var_t8_dn10 = assign62610_e97158_d_n10;
        var_t8_dn13 = assign62610_e97158_d_n13;
        var_t8_rv = 0.0;

        let (assign62620_e97168, assign62620_e97168_d_n0, assign62620_e97168_d_n2, assign62620_e97168_d_n4, assign62620_e97168_d_n5, assign62620_e97168_d_n6, assign62620_e97168_d_n7, assign62620_e97168_d_n8, assign62620_e97168_d_n9, assign62620_e97168_d_n10, assign62620_e97168_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1497 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t8, var_t8_dn0, var_t8_dn2, var_t8_dn4, var_t8_dn5, var_t8_dn6, var_t8_dn7, var_t8_dn8, var_t8_dn9, var_t8_dn10, var_t8_dn13,)
    }
};
        var_t8 = assign62620_e97168;
        var_t8_dn0 = assign62620_e97168_d_n0;
        var_t8_dn2 = assign62620_e97168_d_n2;
        var_t8_dn4 = assign62620_e97168_d_n4;
        var_t8_dn5 = assign62620_e97168_d_n5;
        var_t8_dn6 = assign62620_e97168_d_n6;
        var_t8_dn7 = assign62620_e97168_d_n7;
        var_t8_dn8 = assign62620_e97168_d_n8;
        var_t8_dn9 = assign62620_e97168_d_n9;
        var_t8_dn10 = assign62620_e97168_d_n10;
        var_t8_dn13 = assign62620_e97168_d_n13;
        var_t8_rv = 0.0;

        let assign62630_e97171: f64 = (var_t9 + var_t8);
        let assign62630_e97173: f64 = if assign62630_e97171 > 0.0 { 1.0 } else { 0.0 };
        var_guard1498 = assign62630_e97173;
        var_guard1498_rv = 0.0;

        let (assign62640_e97186, assign62640_e97186_d_n0, assign62640_e97186_d_n2, assign62640_e97186_d_n4, assign62640_e97186_d_n5, assign62640_e97186_d_n6, assign62640_e97186_d_n7, assign62640_e97186_d_n8, assign62640_e97186_d_n9, assign62640_e97186_d_n10, assign62640_e97186_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1498 != 0.0)) {
        let assign62640_e97183: f64 = (var_t9 + var_t8);
        let assign62640_e97184: f64 = (var_pds * assign62640_e97183);
        (assign62640_e97184, ((var_pds_dn0 * assign62640_e97183) + (var_pds * (var_t9_dn0 + var_t8_dn0))), ((var_pds_dn2 * assign62640_e97183) + (var_pds * (var_t9_dn2 + var_t8_dn2))), ((var_pds_dn4 * assign62640_e97183) + (var_pds * (var_t9_dn4 + var_t8_dn4))), ((var_pds_dn5 * assign62640_e97183) + (var_pds * (var_t9_dn5 + var_t8_dn5))), ((var_pds_dn6 * assign62640_e97183) + (var_pds * (var_t9_dn6 + var_t8_dn6))), ((var_pds_dn7 * assign62640_e97183) + (var_pds * (var_t9_dn7 + var_t8_dn7))), ((var_pds_dn8 * assign62640_e97183) + (var_pds * (var_t9_dn8 + var_t8_dn8))), ((var_pds_dn9 * assign62640_e97183) + (var_pds * (var_t9_dn9 + var_t8_dn9))), ((var_pds_dn10 * assign62640_e97183) + (var_pds * (var_t9_dn10 + var_t8_dn10))), ((var_pds_dn13 * assign62640_e97183) + (var_pds * (var_t9_dn13 + var_t8_dn13))),)
    } else {
        (var_idd1, var_idd1_dn0, var_idd1_dn2, var_idd1_dn4, var_idd1_dn5, var_idd1_dn6, var_idd1_dn7, var_idd1_dn8, var_idd1_dn9, var_idd1_dn10, var_idd1_dn13,)
    }
};
        var_idd1 = assign62640_e97186;
        var_idd1_dn0 = assign62640_e97186_d_n0;
        var_idd1_dn2 = assign62640_e97186_d_n2;
        var_idd1_dn4 = assign62640_e97186_d_n4;
        var_idd1_dn5 = assign62640_e97186_d_n5;
        var_idd1_dn6 = assign62640_e97186_d_n6;
        var_idd1_dn7 = assign62640_e97186_d_n7;
        var_idd1_dn8 = assign62640_e97186_d_n8;
        var_idd1_dn9 = assign62640_e97186_d_n9;
        var_idd1_dn10 = assign62640_e97186_d_n10;
        var_idd1_dn13 = assign62640_e97186_d_n13;
        var_idd1_rv = 0.0;

        let (assign62650_e97199, assign62650_e97199_d_n0, assign62650_e97199_d_n2, assign62650_e97199_d_n4, assign62650_e97199_d_n5, assign62650_e97199_d_n6, assign62650_e97199_d_n7, assign62650_e97199_d_n8, assign62650_e97199_d_n9, assign62650_e97199_d_n10, assign62650_e97199_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1498 != 0.0)) {
        let assign62650_e97195: f64 = (var_betawl * var_idd1);
        let assign62650_e97197: f64 = (assign62650_e97195 * var_mu);
        (assign62650_e97197, ((((var_betawl_dn0 * var_idd1) + (var_betawl * var_idd1_dn0)) * var_mu) + (assign62650_e97195 * var_mu_dn0)), ((((var_betawl_dn2 * var_idd1) + (var_betawl * var_idd1_dn2)) * var_mu) + (assign62650_e97195 * var_mu_dn2)), ((((var_betawl_dn4 * var_idd1) + (var_betawl * var_idd1_dn4)) * var_mu) + (assign62650_e97195 * var_mu_dn4)), ((((var_betawl_dn5 * var_idd1) + (var_betawl * var_idd1_dn5)) * var_mu) + (assign62650_e97195 * var_mu_dn5)), ((((var_betawl_dn6 * var_idd1) + (var_betawl * var_idd1_dn6)) * var_mu) + (assign62650_e97195 * var_mu_dn6)), ((((var_betawl_dn7 * var_idd1) + (var_betawl * var_idd1_dn7)) * var_mu) + (assign62650_e97195 * var_mu_dn7)), ((((var_betawl_dn8 * var_idd1) + (var_betawl * var_idd1_dn8)) * var_mu) + (assign62650_e97195 * var_mu_dn8)), ((((var_betawl_dn9 * var_idd1) + (var_betawl * var_idd1_dn9)) * var_mu) + (assign62650_e97195 * var_mu_dn9)), ((((var_betawl_dn10 * var_idd1) + (var_betawl * var_idd1_dn10)) * var_mu) + (assign62650_e97195 * var_mu_dn10)), ((((var_betawl_dn13 * var_idd1) + (var_betawl * var_idd1_dn13)) * var_mu) + (assign62650_e97195 * var_mu_dn13)),)
    } else {
        (var_idspt0, var_idspt0_dn0, var_idspt0_dn2, var_idspt0_dn4, var_idspt0_dn5, var_idspt0_dn6, var_idspt0_dn7, var_idspt0_dn8, var_idspt0_dn9, var_idspt0_dn10, var_idspt0_dn13,)
    }
};
        var_idspt0 = assign62650_e97199;
        var_idspt0_dn0 = assign62650_e97199_d_n0;
        var_idspt0_dn2 = assign62650_e97199_d_n2;
        var_idspt0_dn4 = assign62650_e97199_d_n4;
        var_idspt0_dn5 = assign62650_e97199_d_n5;
        var_idspt0_dn6 = assign62650_e97199_d_n6;
        var_idspt0_dn7 = assign62650_e97199_d_n7;
        var_idspt0_dn8 = assign62650_e97199_d_n8;
        var_idspt0_dn9 = assign62650_e97199_d_n9;
        var_idspt0_dn10 = assign62650_e97199_d_n10;
        var_idspt0_dn13 = assign62650_e97199_d_n13;
        var_idspt0_rv = 0.0;

        let (assign62660_e97210, assign62660_e97210_d_n0, assign62660_e97210_d_n2, assign62660_e97210_d_n4, assign62660_e97210_d_n5, assign62660_e97210_d_n6, assign62660_e97210_d_n7, assign62660_e97210_d_n8, assign62660_e97210_d_n9, assign62660_e97210_d_n10, assign62660_e97210_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1498 != 0.0)) {
        let assign62660_e97208: f64 = (var_ids0 + var_idspt0);
        (assign62660_e97208, (var_ids0_dn0 + var_idspt0_dn0), (var_ids0_dn2 + var_idspt0_dn2), (var_ids0_dn4 + var_idspt0_dn4), (var_ids0_dn5 + var_idspt0_dn5), (var_ids0_dn6 + var_idspt0_dn6), (var_ids0_dn7 + var_idspt0_dn7), (var_ids0_dn8 + var_idspt0_dn8), (var_ids0_dn9 + var_idspt0_dn9), (var_ids0_dn10 + var_idspt0_dn10), (var_ids0_dn13 + var_idspt0_dn13),)
    } else {
        (var_ids0, var_ids0_dn0, var_ids0_dn2, var_ids0_dn4, var_ids0_dn5, var_ids0_dn6, var_ids0_dn7, var_ids0_dn8, var_ids0_dn9, var_ids0_dn10, var_ids0_dn13,)
    }
};
        var_ids0 = assign62660_e97210;
        var_ids0_dn0 = assign62660_e97210_d_n0;
        var_ids0_dn2 = assign62660_e97210_d_n2;
        var_ids0_dn4 = assign62660_e97210_d_n4;
        var_ids0_dn5 = assign62660_e97210_d_n5;
        var_ids0_dn6 = assign62660_e97210_d_n6;
        var_ids0_dn7 = assign62660_e97210_d_n7;
        var_ids0_dn8 = assign62660_e97210_d_n8;
        var_ids0_dn9 = assign62660_e97210_d_n9;
        var_ids0_dn10 = assign62660_e97210_d_n10;
        var_ids0_dn13 = assign62660_e97210_d_n13;
        var_ids0_rv = 0.0;

        let (assign62670_e97220, assign62670_e97220_d_n0, assign62670_e97220_d_n2, assign62670_e97220_d_n4, assign62670_e97220_d_n5, assign62670_e97220_d_n6, assign62670_e97220_d_n7, assign62670_e97220_d_n8, assign62670_e97220_d_n9, assign62670_e97220_d_n10, assign62670_e97220_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1498 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_idspt0, var_idspt0_dn0, var_idspt0_dn2, var_idspt0_dn4, var_idspt0_dn5, var_idspt0_dn6, var_idspt0_dn7, var_idspt0_dn8, var_idspt0_dn9, var_idspt0_dn10, var_idspt0_dn13,)
    }
};
        var_idspt0 = assign62670_e97220;
        var_idspt0_dn0 = assign62670_e97220_d_n0;
        var_idspt0_dn2 = assign62670_e97220_d_n2;
        var_idspt0_dn4 = assign62670_e97220_d_n4;
        var_idspt0_dn5 = assign62670_e97220_d_n5;
        var_idspt0_dn6 = assign62670_e97220_d_n6;
        var_idspt0_dn7 = assign62670_e97220_d_n7;
        var_idspt0_dn8 = assign62670_e97220_d_n8;
        var_idspt0_dn9 = assign62670_e97220_d_n9;
        var_idspt0_dn10 = assign62670_e97220_d_n10;
        var_idspt0_dn13 = assign62670_e97220_d_n13;
        var_idspt0_rv = 0.0;

        let assign62680_e97227: f64 = if ((var_flg_rsrd == 2.0) || (var_flg_rsrd == 3.0)) { 1.0 } else { 0.0 };
        var_guard1499 = assign62680_e97227;
        var_guard1499_rv = 0.0;

        let assign62690_e97230: f64 = if p.p296 > 0.0 { 1.0 } else { 0.0 };
        var_guard1500 = assign62690_e97230;
        var_guard1500_rv = 0.0;

        let (assign62700_e97241, assign62700_e97241_d_n0, assign62700_e97241_d_n2, assign62700_e97241_d_n4, assign62700_e97241_d_n5, assign62700_e97241_d_n6, assign62700_e97241_d_n7, assign62700_e97241_d_n8, assign62700_e97241_d_n9, assign62700_e97241_d_n10, assign62700_e97241_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 != 0.0)) && (var_guard1500 != 0.0)) {
        (var_rd23e, var_rd23e_dn0, var_rd23e_dn2, var_rd23e_dn4, var_rd23e_dn5, var_rd23e_dn6, var_rd23e_dn7, var_rd23e_dn8, var_rd23e_dn9, var_rd23e_dn10, var_rd23e_dn13,)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn13,)
    }
};
        var_t4 = assign62700_e97241;
        var_t4_dn0 = assign62700_e97241_d_n0;
        var_t4_dn2 = assign62700_e97241_d_n2;
        var_t4_dn4 = assign62700_e97241_d_n4;
        var_t4_dn5 = assign62700_e97241_d_n5;
        var_t4_dn6 = assign62700_e97241_d_n6;
        var_t4_dn7 = assign62700_e97241_d_n7;
        var_t4_dn8 = assign62700_e97241_d_n8;
        var_t4_dn9 = assign62700_e97241_d_n9;
        var_t4_dn10 = assign62700_e97241_d_n10;
        var_t4_dn13 = assign62700_e97241_d_n13;
        var_t4_rv = 0.0;

        let (assign62710_e97256, assign62710_e97256_d_n0, assign62710_e97256_d_n2, assign62710_e97256_d_n4, assign62710_e97256_d_n5, assign62710_e97256_d_n6, assign62710_e97256_d_n7, assign62710_e97256_d_n8, assign62710_e97256_d_n9, assign62710_e97256_d_n10, assign62710_e97256_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 != 0.0)) && (var_guard1500 != 0.0)) {
        let assign62710_e97253: f64 = (var_vgse - p.p300);
        let assign62710_e97254: f64 = (var_uc_rd24 * assign62710_e97253);
        (assign62710_e97254, (var_uc_rd24 * var_vgse_dn0), (var_uc_rd24 * var_vgse_dn2), 0.0, 0.0, (var_uc_rd24 * var_vgse_dn6), 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign62710_e97256;
        var_t1_dn0 = assign62710_e97256_d_n0;
        var_t1_dn2 = assign62710_e97256_d_n2;
        var_t1_dn4 = assign62710_e97256_d_n4;
        var_t1_dn5 = assign62710_e97256_d_n5;
        var_t1_dn6 = assign62710_e97256_d_n6;
        var_t1_dn7 = assign62710_e97256_d_n7;
        var_t1_dn8 = assign62710_e97256_d_n8;
        var_t1_dn9 = assign62710_e97256_d_n9;
        var_t1_dn10 = assign62710_e97256_d_n10;
        var_t1_dn13 = assign62710_e97256_d_n13;
        var_t1_rv = 0.0;

        let (assign62720_e97273, assign62720_e97273_d_n0, assign62720_e97273_d_n2, assign62720_e97273_d_n4, assign62720_e97273_d_n5, assign62720_e97273_d_n6, assign62720_e97273_d_n7, assign62720_e97273_d_n8, assign62720_e97273_d_n9, assign62720_e97273_d_n10, assign62720_e97273_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 != 0.0)) && (var_guard1500 != 0.0)) {
        let assign62720_e97267: f64 = (var_t1 - var_t4);
        let assign62720_e97270: f64 = (0.01 * 0.01);
        let assign62720_e97271: f64 = (assign62720_e97267 - assign62720_e97270);
        (assign62720_e97271, (var_t1_dn0 - var_t4_dn0), (var_t1_dn2 - var_t4_dn2), (var_t1_dn4 - var_t4_dn4), (var_t1_dn5 - var_t4_dn5), (var_t1_dn6 - var_t4_dn6), (var_t1_dn7 - var_t4_dn7), (var_t1_dn8 - var_t4_dn8), (var_t1_dn9 - var_t4_dn9), (var_t1_dn10 - var_t4_dn10), (var_t1_dn13 - var_t4_dn13),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn13,)
    }
};
        var_tmf1 = assign62720_e97273;
        var_tmf1_dn0 = assign62720_e97273_d_n0;
        var_tmf1_dn2 = assign62720_e97273_d_n2;
        var_tmf1_dn4 = assign62720_e97273_d_n4;
        var_tmf1_dn5 = assign62720_e97273_d_n5;
        var_tmf1_dn6 = assign62720_e97273_d_n6;
        var_tmf1_dn7 = assign62720_e97273_d_n7;
        var_tmf1_dn8 = assign62720_e97273_d_n8;
        var_tmf1_dn9 = assign62720_e97273_d_n9;
        var_tmf1_dn10 = assign62720_e97273_d_n10;
        var_tmf1_dn13 = assign62720_e97273_d_n13;
        var_tmf1_rv = 0.0;

        let (assign62730_e97290, assign62730_e97290_d_n0, assign62730_e97290_d_n2, assign62730_e97290_d_n4, assign62730_e97290_d_n5, assign62730_e97290_d_n6, assign62730_e97290_d_n7, assign62730_e97290_d_n8, assign62730_e97290_d_n9, assign62730_e97290_d_n10, assign62730_e97290_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 != 0.0)) && (var_guard1500 != 0.0)) {
        let assign62730_e97284: f64 = (4.0 * var_t4);
        let assign62730_e97287: f64 = (0.01 * 0.01);
        let assign62730_e97288: f64 = (assign62730_e97284 * assign62730_e97287);
        (assign62730_e97288, ((4.0 * var_t4_dn0) * assign62730_e97287), ((4.0 * var_t4_dn2) * assign62730_e97287), ((4.0 * var_t4_dn4) * assign62730_e97287), ((4.0 * var_t4_dn5) * assign62730_e97287), ((4.0 * var_t4_dn6) * assign62730_e97287), ((4.0 * var_t4_dn7) * assign62730_e97287), ((4.0 * var_t4_dn8) * assign62730_e97287), ((4.0 * var_t4_dn9) * assign62730_e97287), ((4.0 * var_t4_dn10) * assign62730_e97287), ((4.0 * var_t4_dn13) * assign62730_e97287),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
    }
};
        var_tmf2 = assign62730_e97290;
        var_tmf2_dn0 = assign62730_e97290_d_n0;
        var_tmf2_dn2 = assign62730_e97290_d_n2;
        var_tmf2_dn4 = assign62730_e97290_d_n4;
        var_tmf2_dn5 = assign62730_e97290_d_n5;
        var_tmf2_dn6 = assign62730_e97290_d_n6;
        var_tmf2_dn7 = assign62730_e97290_d_n7;
        var_tmf2_dn8 = assign62730_e97290_d_n8;
        var_tmf2_dn9 = assign62730_e97290_d_n9;
        var_tmf2_dn10 = assign62730_e97290_d_n10;
        var_tmf2_dn13 = assign62730_e97290_d_n13;
        var_tmf2_rv = 0.0;

        let (assign62740_e97307, assign62740_e97307_d_n0, assign62740_e97307_d_n2, assign62740_e97307_d_n4, assign62740_e97307_d_n5, assign62740_e97307_d_n6, assign62740_e97307_d_n7, assign62740_e97307_d_n8, assign62740_e97307_d_n9, assign62740_e97307_d_n10, assign62740_e97307_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 != 0.0)) && (var_guard1500 != 0.0)) {
        let (assign62740_e97305, assign62740_e97305_d_n0, assign62740_e97305_d_n2, assign62740_e97305_d_n4, assign62740_e97305_d_n5, assign62740_e97305_d_n6, assign62740_e97305_d_n7, assign62740_e97305_d_n8, assign62740_e97305_d_n9, assign62740_e97305_d_n10, assign62740_e97305_d_n13,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
            } else {
                let assign62740_e97304: f64 = (-var_tmf2);
                (assign62740_e97304, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn8), (-var_tmf2_dn9), (-var_tmf2_dn10), (-var_tmf2_dn13),)
            }
        };
        (assign62740_e97305, assign62740_e97305_d_n0, assign62740_e97305_d_n2, assign62740_e97305_d_n4, assign62740_e97305_d_n5, assign62740_e97305_d_n6, assign62740_e97305_d_n7, assign62740_e97305_d_n8, assign62740_e97305_d_n9, assign62740_e97305_d_n10, assign62740_e97305_d_n13,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
    }
};
        var_tmf2 = assign62740_e97307;
        var_tmf2_dn0 = assign62740_e97307_d_n0;
        var_tmf2_dn2 = assign62740_e97307_d_n2;
        var_tmf2_dn4 = assign62740_e97307_d_n4;
        var_tmf2_dn5 = assign62740_e97307_d_n5;
        var_tmf2_dn6 = assign62740_e97307_d_n6;
        var_tmf2_dn7 = assign62740_e97307_d_n7;
        var_tmf2_dn8 = assign62740_e97307_d_n8;
        var_tmf2_dn9 = assign62740_e97307_d_n9;
        var_tmf2_dn10 = assign62740_e97307_d_n10;
        var_tmf2_dn13 = assign62740_e97307_d_n13;
        var_tmf2_rv = 0.0;

        let (assign62750_e97323, assign62750_e97323_d_n0, assign62750_e97323_d_n2, assign62750_e97323_d_n4, assign62750_e97323_d_n5, assign62750_e97323_d_n6, assign62750_e97323_d_n7, assign62750_e97323_d_n8, assign62750_e97323_d_n9, assign62750_e97323_d_n10, assign62750_e97323_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 != 0.0)) && (var_guard1500 != 0.0)) {
        let assign62750_e97318: f64 = (var_tmf1 * var_tmf1);
        let assign62750_e97320: f64 = (assign62750_e97318 + var_tmf2);
        let assign62750_e97321: f64 = (assign62750_e97320).sqrt();
        (assign62750_e97321, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign62750_e97321)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign62750_e97321)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign62750_e97321)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign62750_e97321)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign62750_e97321)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign62750_e97321)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign62750_e97321)), ((((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)) + var_tmf2_dn9) / (2.0 * assign62750_e97321)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign62750_e97321)), ((((var_tmf1_dn13 * var_tmf1) + (var_tmf1 * var_tmf1_dn13)) + var_tmf2_dn13) / (2.0 * assign62750_e97321)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
    }
};
        var_tmf2 = assign62750_e97323;
        var_tmf2_dn0 = assign62750_e97323_d_n0;
        var_tmf2_dn2 = assign62750_e97323_d_n2;
        var_tmf2_dn4 = assign62750_e97323_d_n4;
        var_tmf2_dn5 = assign62750_e97323_d_n5;
        var_tmf2_dn6 = assign62750_e97323_d_n6;
        var_tmf2_dn7 = assign62750_e97323_d_n7;
        var_tmf2_dn8 = assign62750_e97323_d_n8;
        var_tmf2_dn9 = assign62750_e97323_d_n9;
        var_tmf2_dn10 = assign62750_e97323_d_n10;
        var_tmf2_dn13 = assign62750_e97323_d_n13;
        var_tmf2_rv = 0.0;

        let (assign62760_e97340, assign62760_e97340_d_n0, assign62760_e97340_d_n2, assign62760_e97340_d_n4, assign62760_e97340_d_n5, assign62760_e97340_d_n6, assign62760_e97340_d_n7, assign62760_e97340_d_n8, assign62760_e97340_d_n9, assign62760_e97340_d_n10, assign62760_e97340_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 != 0.0)) && (var_guard1500 != 0.0)) {
        let assign62760_e97336: f64 = (var_tmf1 / var_tmf2);
        let assign62760_e97337: f64 = (1.0 + assign62760_e97336);
        let assign62760_e97338: f64 = (0.5 * assign62760_e97337);
        (assign62760_e97338, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn7 * var_tmf2) - (var_tmf1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn9 * var_tmf2) - (var_tmf1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn13 * var_tmf2) - (var_tmf1 * var_tmf2_dn13)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign62760_e97340;
        var_t0_dn0 = assign62760_e97340_d_n0;
        var_t0_dn2 = assign62760_e97340_d_n2;
        var_t0_dn4 = assign62760_e97340_d_n4;
        var_t0_dn5 = assign62760_e97340_d_n5;
        var_t0_dn6 = assign62760_e97340_d_n6;
        var_t0_dn7 = assign62760_e97340_d_n7;
        var_t0_dn8 = assign62760_e97340_d_n8;
        var_t0_dn9 = assign62760_e97340_d_n9;
        var_t0_dn10 = assign62760_e97340_d_n10;
        var_t0_dn13 = assign62760_e97340_d_n13;
        var_t0_rv = 0.0;

        let (assign62770_e97357, assign62770_e97357_d_n0, assign62770_e97357_d_n2, assign62770_e97357_d_n4, assign62770_e97357_d_n5, assign62770_e97357_d_n6, assign62770_e97357_d_n7, assign62770_e97357_d_n8, assign62770_e97357_d_n9, assign62770_e97357_d_n10, assign62770_e97357_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 != 0.0)) && (var_guard1500 != 0.0)) {
        let assign62770_e97353: f64 = (var_tmf1 + var_tmf2);
        let assign62770_e97354: f64 = (0.5 * assign62770_e97353);
        let assign62770_e97355: f64 = (var_t4 + assign62770_e97354);
        (assign62770_e97355, (var_t4_dn0 + (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (var_t4_dn2 + (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (var_t4_dn4 + (0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (var_t4_dn5 + (0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (var_t4_dn6 + (0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), (var_t4_dn7 + (0.5 * (var_tmf1_dn7 + var_tmf2_dn7))), (var_t4_dn8 + (0.5 * (var_tmf1_dn8 + var_tmf2_dn8))), (var_t4_dn9 + (0.5 * (var_tmf1_dn9 + var_tmf2_dn9))), (var_t4_dn10 + (0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), (var_t4_dn13 + (0.5 * (var_tmf1_dn13 + var_tmf2_dn13))),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign62770_e97357;
        var_t2_dn0 = assign62770_e97357_d_n0;
        var_t2_dn2 = assign62770_e97357_d_n2;
        var_t2_dn4 = assign62770_e97357_d_n4;
        var_t2_dn5 = assign62770_e97357_d_n5;
        var_t2_dn6 = assign62770_e97357_d_n6;
        var_t2_dn7 = assign62770_e97357_d_n7;
        var_t2_dn8 = assign62770_e97357_d_n8;
        var_t2_dn9 = assign62770_e97357_d_n9;
        var_t2_dn10 = assign62770_e97357_d_n10;
        var_t2_dn13 = assign62770_e97357_d_n13;
        var_t2_rv = 0.0;

        let (assign62780_e97372, assign62780_e97372_d_n0, assign62780_e97372_d_n2, assign62780_e97372_d_n4, assign62780_e97372_d_n5, assign62780_e97372_d_n6, assign62780_e97372_d_n7, assign62780_e97372_d_n8, assign62780_e97372_d_n9, assign62780_e97372_d_n10, assign62780_e97372_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 != 0.0)) && (var_guard1500 != 0.0)) {
        let assign62780_e97369: f64 = (p.p296 + 1.0);
        let assign62780_e97370: f64 = (var_t4 * assign62780_e97369);
        (assign62780_e97370, (var_t4_dn0 * assign62780_e97369), (var_t4_dn2 * assign62780_e97369), (var_t4_dn4 * assign62780_e97369), (var_t4_dn5 * assign62780_e97369), (var_t4_dn6 * assign62780_e97369), (var_t4_dn7 * assign62780_e97369), (var_t4_dn8 * assign62780_e97369), (var_t4_dn9 * assign62780_e97369), (var_t4_dn10 * assign62780_e97369), (var_t4_dn13 * assign62780_e97369),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn13,)
    }
};
        var_t3 = assign62780_e97372;
        var_t3_dn0 = assign62780_e97372_d_n0;
        var_t3_dn2 = assign62780_e97372_d_n2;
        var_t3_dn4 = assign62780_e97372_d_n4;
        var_t3_dn5 = assign62780_e97372_d_n5;
        var_t3_dn6 = assign62780_e97372_d_n6;
        var_t3_dn7 = assign62780_e97372_d_n7;
        var_t3_dn8 = assign62780_e97372_d_n8;
        var_t3_dn9 = assign62780_e97372_d_n9;
        var_t3_dn10 = assign62780_e97372_d_n10;
        var_t3_dn13 = assign62780_e97372_d_n13;
        var_t3_rv = 0.0;

        let (assign62790_e97389, assign62790_e97389_d_n0, assign62790_e97389_d_n2, assign62790_e97389_d_n4, assign62790_e97389_d_n5, assign62790_e97389_d_n6, assign62790_e97389_d_n7, assign62790_e97389_d_n8, assign62790_e97389_d_n9, assign62790_e97389_d_n10, assign62790_e97389_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 != 0.0)) && (var_guard1500 != 0.0)) {
        let assign62790_e97383: f64 = (var_t3 - var_t2);
        let assign62790_e97386: f64 = (0.01 * 0.01);
        let assign62790_e97387: f64 = (assign62790_e97383 - assign62790_e97386);
        (assign62790_e97387, (var_t3_dn0 - var_t2_dn0), (var_t3_dn2 - var_t2_dn2), (var_t3_dn4 - var_t2_dn4), (var_t3_dn5 - var_t2_dn5), (var_t3_dn6 - var_t2_dn6), (var_t3_dn7 - var_t2_dn7), (var_t3_dn8 - var_t2_dn8), (var_t3_dn9 - var_t2_dn9), (var_t3_dn10 - var_t2_dn10), (var_t3_dn13 - var_t2_dn13),)
    } else {
        (var_tmf1, var_tmf1_dn0, var_tmf1_dn2, var_tmf1_dn4, var_tmf1_dn5, var_tmf1_dn6, var_tmf1_dn7, var_tmf1_dn8, var_tmf1_dn9, var_tmf1_dn10, var_tmf1_dn13,)
    }
};
        var_tmf1 = assign62790_e97389;
        var_tmf1_dn0 = assign62790_e97389_d_n0;
        var_tmf1_dn2 = assign62790_e97389_d_n2;
        var_tmf1_dn4 = assign62790_e97389_d_n4;
        var_tmf1_dn5 = assign62790_e97389_d_n5;
        var_tmf1_dn6 = assign62790_e97389_d_n6;
        var_tmf1_dn7 = assign62790_e97389_d_n7;
        var_tmf1_dn8 = assign62790_e97389_d_n8;
        var_tmf1_dn9 = assign62790_e97389_d_n9;
        var_tmf1_dn10 = assign62790_e97389_d_n10;
        var_tmf1_dn13 = assign62790_e97389_d_n13;
        var_tmf1_rv = 0.0;

        let (assign62800_e97406, assign62800_e97406_d_n0, assign62800_e97406_d_n2, assign62800_e97406_d_n4, assign62800_e97406_d_n5, assign62800_e97406_d_n6, assign62800_e97406_d_n7, assign62800_e97406_d_n8, assign62800_e97406_d_n9, assign62800_e97406_d_n10, assign62800_e97406_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 != 0.0)) && (var_guard1500 != 0.0)) {
        let assign62800_e97400: f64 = (4.0 * var_t3);
        let assign62800_e97403: f64 = (0.01 * 0.01);
        let assign62800_e97404: f64 = (assign62800_e97400 * assign62800_e97403);
        (assign62800_e97404, ((4.0 * var_t3_dn0) * assign62800_e97403), ((4.0 * var_t3_dn2) * assign62800_e97403), ((4.0 * var_t3_dn4) * assign62800_e97403), ((4.0 * var_t3_dn5) * assign62800_e97403), ((4.0 * var_t3_dn6) * assign62800_e97403), ((4.0 * var_t3_dn7) * assign62800_e97403), ((4.0 * var_t3_dn8) * assign62800_e97403), ((4.0 * var_t3_dn9) * assign62800_e97403), ((4.0 * var_t3_dn10) * assign62800_e97403), ((4.0 * var_t3_dn13) * assign62800_e97403),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
    }
};
        var_tmf2 = assign62800_e97406;
        var_tmf2_dn0 = assign62800_e97406_d_n0;
        var_tmf2_dn2 = assign62800_e97406_d_n2;
        var_tmf2_dn4 = assign62800_e97406_d_n4;
        var_tmf2_dn5 = assign62800_e97406_d_n5;
        var_tmf2_dn6 = assign62800_e97406_d_n6;
        var_tmf2_dn7 = assign62800_e97406_d_n7;
        var_tmf2_dn8 = assign62800_e97406_d_n8;
        var_tmf2_dn9 = assign62800_e97406_d_n9;
        var_tmf2_dn10 = assign62800_e97406_d_n10;
        var_tmf2_dn13 = assign62800_e97406_d_n13;
        var_tmf2_rv = 0.0;

        *var_guard1497_slot = var_guard1497;
        *var_guard1497_rv_slot = var_guard1497_rv;
        *var_guard1498_slot = var_guard1498;
        *var_guard1498_rv_slot = var_guard1498_rv;
        *var_guard1499_slot = var_guard1499;
        *var_guard1499_rv_slot = var_guard1499_rv;
        *var_guard1500_slot = var_guard1500;
        *var_guard1500_rv_slot = var_guard1500_rv;
        *var_idd1_slot = var_idd1;
        *var_idd1_dn0_slot = var_idd1_dn0;
        *var_idd1_dn10_slot = var_idd1_dn10;
        *var_idd1_dn13_slot = var_idd1_dn13;
        *var_idd1_dn2_slot = var_idd1_dn2;
        *var_idd1_dn4_slot = var_idd1_dn4;
        *var_idd1_dn5_slot = var_idd1_dn5;
        *var_idd1_dn6_slot = var_idd1_dn6;
        *var_idd1_dn7_slot = var_idd1_dn7;
        *var_idd1_dn8_slot = var_idd1_dn8;
        *var_idd1_dn9_slot = var_idd1_dn9;
        *var_idd1_rv_slot = var_idd1_rv;
        *var_ids0_slot = var_ids0;
        *var_ids0_dn0_slot = var_ids0_dn0;
        *var_ids0_dn10_slot = var_ids0_dn10;
        *var_ids0_dn13_slot = var_ids0_dn13;
        *var_ids0_dn2_slot = var_ids0_dn2;
        *var_ids0_dn4_slot = var_ids0_dn4;
        *var_ids0_dn5_slot = var_ids0_dn5;
        *var_ids0_dn6_slot = var_ids0_dn6;
        *var_ids0_dn7_slot = var_ids0_dn7;
        *var_ids0_dn8_slot = var_ids0_dn8;
        *var_ids0_dn9_slot = var_ids0_dn9;
        *var_ids0_rv_slot = var_ids0_rv;
        *var_idspt0_slot = var_idspt0;
        *var_idspt0_dn0_slot = var_idspt0_dn0;
        *var_idspt0_dn10_slot = var_idspt0_dn10;
        *var_idspt0_dn13_slot = var_idspt0_dn13;
        *var_idspt0_dn2_slot = var_idspt0_dn2;
        *var_idspt0_dn4_slot = var_idspt0_dn4;
        *var_idspt0_dn5_slot = var_idspt0_dn5;
        *var_idspt0_dn6_slot = var_idspt0_dn6;
        *var_idspt0_dn7_slot = var_idspt0_dn7;
        *var_idspt0_dn8_slot = var_idspt0_dn8;
        *var_idspt0_dn9_slot = var_idspt0_dn9;
        *var_idspt0_rv_slot = var_idspt0_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn13_slot = var_t0_dn13;
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
        *var_t1_dn13_slot = var_t1_dn13;
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
        *var_t2_dn13_slot = var_t2_dn13;
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
        *var_t3_dn13_slot = var_t3_dn13;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t3_rv_slot = var_t3_rv;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn13_slot = var_t4_dn13;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_dn9_slot = var_t4_dn9;
        *var_t4_rv_slot = var_t4_rv;
        *var_t6_slot = var_t6;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn13_slot = var_t6_dn13;
        *var_t6_dn2_slot = var_t6_dn2;
        *var_t6_dn4_slot = var_t6_dn4;
        *var_t6_dn5_slot = var_t6_dn5;
        *var_t6_dn6_slot = var_t6_dn6;
        *var_t6_dn7_slot = var_t6_dn7;
        *var_t6_dn8_slot = var_t6_dn8;
        *var_t6_dn9_slot = var_t6_dn9;
        *var_t6_rv_slot = var_t6_rv;
        *var_t8_slot = var_t8;
        *var_t8_dn0_slot = var_t8_dn0;
        *var_t8_dn10_slot = var_t8_dn10;
        *var_t8_dn13_slot = var_t8_dn13;
        *var_t8_dn2_slot = var_t8_dn2;
        *var_t8_dn4_slot = var_t8_dn4;
        *var_t8_dn5_slot = var_t8_dn5;
        *var_t8_dn6_slot = var_t8_dn6;
        *var_t8_dn7_slot = var_t8_dn7;
        *var_t8_dn8_slot = var_t8_dn8;
        *var_t8_dn9_slot = var_t8_dn9;
        *var_t8_rv_slot = var_t8_rv;
        *var_t9_slot = var_t9;
        *var_t9_dn0_slot = var_t9_dn0;
        *var_t9_dn10_slot = var_t9_dn10;
        *var_t9_dn13_slot = var_t9_dn13;
        *var_t9_dn2_slot = var_t9_dn2;
        *var_t9_dn4_slot = var_t9_dn4;
        *var_t9_dn5_slot = var_t9_dn5;
        *var_t9_dn6_slot = var_t9_dn6;
        *var_t9_dn7_slot = var_t9_dn7;
        *var_t9_dn8_slot = var_t9_dn8;
        *var_t9_dn9_slot = var_t9_dn9;
        *var_t9_rv_slot = var_t9_rv;
        *var_tmf1_slot = var_tmf1;
        *var_tmf1_dn0_slot = var_tmf1_dn0;
        *var_tmf1_dn10_slot = var_tmf1_dn10;
        *var_tmf1_dn13_slot = var_tmf1_dn13;
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
        *var_tmf2_dn13_slot = var_tmf2_dn13;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn7_slot = var_tmf2_dn7;
        *var_tmf2_dn8_slot = var_tmf2_dn8;
        *var_tmf2_dn9_slot = var_tmf2_dn9;
        *var_tmf2_rv_slot = var_tmf2_rv;
    }

    pub(super) fn stamp_reactive_block_225(
        p: &Parameters,
        var_cox_inv: f64,
        var_cox_inv_dn0: f64,
        var_cox_inv_dn10: f64,
        var_cox_inv_dn13: f64,
        var_cox_inv_dn2: f64,
        var_cox_inv_dn4: f64,
        var_cox_inv_dn5: f64,
        var_cox_inv_dn6: f64,
        var_cox_inv_dn7: f64,
        var_cox_inv_dn8: f64,
        var_cox_inv_dn9: f64,
        var_guard1430: f64,
        var_guard1499: f64,
        var_guard1500: f64,
        var_guard443: f64,
        var_ids0: f64,
        var_ids0_dn0: f64,
        var_ids0_dn10: f64,
        var_ids0_dn13: f64,
        var_ids0_dn2: f64,
        var_ids0_dn4: f64,
        var_ids0_dn5: f64,
        var_ids0_dn6: f64,
        var_ids0_dn7: f64,
        var_ids0_dn8: f64,
        var_ids0_dn9: f64,
        var_rd23e: f64,
        var_rd23e_dn0: f64,
        var_rd23e_dn10: f64,
        var_rd23e_dn13: f64,
        var_rd23e_dn2: f64,
        var_rd23e_dn4: f64,
        var_rd23e_dn5: f64,
        var_rd23e_dn6: f64,
        var_rd23e_dn7: f64,
        var_rd23e_dn8: f64,
        var_rd23e_dn9: f64,
        var_tmf1: f64,
        var_tmf1_dn0: f64,
        var_tmf1_dn10: f64,
        var_tmf1_dn13: f64,
        var_tmf1_dn2: f64,
        var_tmf1_dn4: f64,
        var_tmf1_dn5: f64,
        var_tmf1_dn6: f64,
        var_tmf1_dn7: f64,
        var_tmf1_dn8: f64,
        var_tmf1_dn9: f64,
        var_uc_rd22: f64,
        var_vbse: f64,
        var_vbse_dn0: f64,
        var_vbse_dn2: f64,
        var_vbse_dn8: f64,
        var_vds: f64,
        var_vds_dn0: f64,
        var_vds_dn10: f64,
        var_vds_dn13: f64,
        var_vds_dn2: f64,
        var_vds_dn4: f64,
        var_vds_dn5: f64,
        var_vds_dn6: f64,
        var_vds_dn7: f64,
        var_vds_dn8: f64,
        var_vds_dn9: f64,
        var_vdse: f64,
        var_vdse_dn0: f64,
        var_vdse_dn2: f64,
        var_wdpl: f64,
        var_wdpl_dn0: f64,
        var_wdpl_dn10: f64,
        var_wdpl_dn13: f64,
        var_wdpl_dn2: f64,
        var_wdpl_dn4: f64,
        var_wdpl_dn5: f64,
        var_wdpl_dn6: f64,
        var_wdpl_dn7: f64,
        var_wdpl_dn8: f64,
        var_wdpl_dn9: f64,
        var_weff_nf: f64,
        var_guard1501_slot: &mut f64,
        var_guard1501_rv_slot: &mut f64,
        var_guard1502_slot: &mut f64,
        var_guard1502_rv_slot: &mut f64,
        var_guard1503_slot: &mut f64,
        var_guard1503_rv_slot: &mut f64,
        var_ids_slot: &mut f64,
        var_ids_dn0_slot: &mut f64,
        var_ids_dn10_slot: &mut f64,
        var_ids_dn13_slot: &mut f64,
        var_ids_dn2_slot: &mut f64,
        var_ids_dn4_slot: &mut f64,
        var_ids_dn5_slot: &mut f64,
        var_ids_dn6_slot: &mut f64,
        var_ids_dn7_slot: &mut f64,
        var_ids_dn8_slot: &mut f64,
        var_ids_dn9_slot: &mut f64,
        var_ids_rv_slot: &mut f64,
        var_ra_slot: &mut f64,
        var_ra_alpha_slot: &mut f64,
        var_ra_alpha_rv_slot: &mut f64,
        var_ra_beta_slot: &mut f64,
        var_ra_beta_rv_slot: &mut f64,
        var_ra_dn0_slot: &mut f64,
        var_ra_dn10_slot: &mut f64,
        var_ra_dn13_slot: &mut f64,
        var_ra_dn2_slot: &mut f64,
        var_ra_dn4_slot: &mut f64,
        var_ra_dn5_slot: &mut f64,
        var_ra_dn6_slot: &mut f64,
        var_ra_dn7_slot: &mut f64,
        var_ra_dn8_slot: &mut f64,
        var_ra_dn9_slot: &mut f64,
        var_ra_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn13_slot: &mut f64,
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
        var_t1_dn13_slot: &mut f64,
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
        var_t2_dn13_slot: &mut f64,
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
        var_t3_dn13_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn13_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_dn9_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
        var_t7_slot: &mut f64,
        var_t7_dn0_slot: &mut f64,
        var_t7_dn10_slot: &mut f64,
        var_t7_dn13_slot: &mut f64,
        var_t7_dn2_slot: &mut f64,
        var_t7_dn4_slot: &mut f64,
        var_t7_dn5_slot: &mut f64,
        var_t7_dn6_slot: &mut f64,
        var_t7_dn7_slot: &mut f64,
        var_t7_dn8_slot: &mut f64,
        var_t7_dn9_slot: &mut f64,
        var_t7_rv_slot: &mut f64,
        var_t9_slot: &mut f64,
        var_t9_dn0_slot: &mut f64,
        var_t9_dn10_slot: &mut f64,
        var_t9_dn13_slot: &mut f64,
        var_t9_dn2_slot: &mut f64,
        var_t9_dn4_slot: &mut f64,
        var_t9_dn5_slot: &mut f64,
        var_t9_dn6_slot: &mut f64,
        var_t9_dn7_slot: &mut f64,
        var_t9_dn8_slot: &mut f64,
        var_t9_dn9_slot: &mut f64,
        var_t9_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn13_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn7_slot: &mut f64,
        var_tmf2_dn8_slot: &mut f64,
        var_tmf2_dn9_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
        var_vdse_eff_slot: &mut f64,
        var_vdse_eff_dn0_slot: &mut f64,
        var_vdse_eff_dn2_slot: &mut f64,
        var_vdse_eff_rv_slot: &mut f64,
    ) {
        let mut var_guard1501: f64 = *var_guard1501_slot;
        let mut var_guard1501_rv: f64 = *var_guard1501_rv_slot;
        let mut var_guard1502: f64 = *var_guard1502_slot;
        let mut var_guard1502_rv: f64 = *var_guard1502_rv_slot;
        let mut var_guard1503: f64 = *var_guard1503_slot;
        let mut var_guard1503_rv: f64 = *var_guard1503_rv_slot;
        let mut var_ids: f64 = *var_ids_slot;
        let mut var_ids_dn0: f64 = *var_ids_dn0_slot;
        let mut var_ids_dn10: f64 = *var_ids_dn10_slot;
        let mut var_ids_dn13: f64 = *var_ids_dn13_slot;
        let mut var_ids_dn2: f64 = *var_ids_dn2_slot;
        let mut var_ids_dn4: f64 = *var_ids_dn4_slot;
        let mut var_ids_dn5: f64 = *var_ids_dn5_slot;
        let mut var_ids_dn6: f64 = *var_ids_dn6_slot;
        let mut var_ids_dn7: f64 = *var_ids_dn7_slot;
        let mut var_ids_dn8: f64 = *var_ids_dn8_slot;
        let mut var_ids_dn9: f64 = *var_ids_dn9_slot;
        let mut var_ids_rv: f64 = *var_ids_rv_slot;
        let mut var_ra: f64 = *var_ra_slot;
        let mut var_ra_alpha: f64 = *var_ra_alpha_slot;
        let mut var_ra_alpha_rv: f64 = *var_ra_alpha_rv_slot;
        let mut var_ra_beta: f64 = *var_ra_beta_slot;
        let mut var_ra_beta_rv: f64 = *var_ra_beta_rv_slot;
        let mut var_ra_dn0: f64 = *var_ra_dn0_slot;
        let mut var_ra_dn10: f64 = *var_ra_dn10_slot;
        let mut var_ra_dn13: f64 = *var_ra_dn13_slot;
        let mut var_ra_dn2: f64 = *var_ra_dn2_slot;
        let mut var_ra_dn4: f64 = *var_ra_dn4_slot;
        let mut var_ra_dn5: f64 = *var_ra_dn5_slot;
        let mut var_ra_dn6: f64 = *var_ra_dn6_slot;
        let mut var_ra_dn7: f64 = *var_ra_dn7_slot;
        let mut var_ra_dn8: f64 = *var_ra_dn8_slot;
        let mut var_ra_dn9: f64 = *var_ra_dn9_slot;
        let mut var_ra_rv: f64 = *var_ra_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn13: f64 = *var_t0_dn13_slot;
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
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
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
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
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
        let mut var_t3_dn13: f64 = *var_t3_dn13_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn13: f64 = *var_t4_dn13_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_dn9: f64 = *var_t4_dn9_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;
        let mut var_t7: f64 = *var_t7_slot;
        let mut var_t7_dn0: f64 = *var_t7_dn0_slot;
        let mut var_t7_dn10: f64 = *var_t7_dn10_slot;
        let mut var_t7_dn13: f64 = *var_t7_dn13_slot;
        let mut var_t7_dn2: f64 = *var_t7_dn2_slot;
        let mut var_t7_dn4: f64 = *var_t7_dn4_slot;
        let mut var_t7_dn5: f64 = *var_t7_dn5_slot;
        let mut var_t7_dn6: f64 = *var_t7_dn6_slot;
        let mut var_t7_dn7: f64 = *var_t7_dn7_slot;
        let mut var_t7_dn8: f64 = *var_t7_dn8_slot;
        let mut var_t7_dn9: f64 = *var_t7_dn9_slot;
        let mut var_t7_rv: f64 = *var_t7_rv_slot;
        let mut var_t9: f64 = *var_t9_slot;
        let mut var_t9_dn0: f64 = *var_t9_dn0_slot;
        let mut var_t9_dn10: f64 = *var_t9_dn10_slot;
        let mut var_t9_dn13: f64 = *var_t9_dn13_slot;
        let mut var_t9_dn2: f64 = *var_t9_dn2_slot;
        let mut var_t9_dn4: f64 = *var_t9_dn4_slot;
        let mut var_t9_dn5: f64 = *var_t9_dn5_slot;
        let mut var_t9_dn6: f64 = *var_t9_dn6_slot;
        let mut var_t9_dn7: f64 = *var_t9_dn7_slot;
        let mut var_t9_dn8: f64 = *var_t9_dn8_slot;
        let mut var_t9_dn9: f64 = *var_t9_dn9_slot;
        let mut var_t9_rv: f64 = *var_t9_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn13: f64 = *var_tmf2_dn13_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn7: f64 = *var_tmf2_dn7_slot;
        let mut var_tmf2_dn8: f64 = *var_tmf2_dn8_slot;
        let mut var_tmf2_dn9: f64 = *var_tmf2_dn9_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;
        let mut var_vdse_eff: f64 = *var_vdse_eff_slot;
        let mut var_vdse_eff_dn0: f64 = *var_vdse_eff_dn0_slot;
        let mut var_vdse_eff_dn2: f64 = *var_vdse_eff_dn2_slot;
        let mut var_vdse_eff_rv: f64 = *var_vdse_eff_rv_slot;

        let (assign62810_e97423, assign62810_e97423_d_n0, assign62810_e97423_d_n2, assign62810_e97423_d_n4, assign62810_e97423_d_n5, assign62810_e97423_d_n6, assign62810_e97423_d_n7, assign62810_e97423_d_n8, assign62810_e97423_d_n9, assign62810_e97423_d_n10, assign62810_e97423_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 != 0.0)) && (var_guard1500 != 0.0)) {
        let (assign62810_e97421, assign62810_e97421_d_n0, assign62810_e97421_d_n2, assign62810_e97421_d_n4, assign62810_e97421_d_n5, assign62810_e97421_d_n6, assign62810_e97421_d_n7, assign62810_e97421_d_n8, assign62810_e97421_d_n9, assign62810_e97421_d_n10, assign62810_e97421_d_n13,) = {
            if (var_tmf2 > 0.0) {
                (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
            } else {
                let assign62810_e97420: f64 = (-var_tmf2);
                (assign62810_e97420, (-var_tmf2_dn0), (-var_tmf2_dn2), (-var_tmf2_dn4), (-var_tmf2_dn5), (-var_tmf2_dn6), (-var_tmf2_dn7), (-var_tmf2_dn8), (-var_tmf2_dn9), (-var_tmf2_dn10), (-var_tmf2_dn13),)
            }
        };
        (assign62810_e97421, assign62810_e97421_d_n0, assign62810_e97421_d_n2, assign62810_e97421_d_n4, assign62810_e97421_d_n5, assign62810_e97421_d_n6, assign62810_e97421_d_n7, assign62810_e97421_d_n8, assign62810_e97421_d_n9, assign62810_e97421_d_n10, assign62810_e97421_d_n13,)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
    }
};
        var_tmf2 = assign62810_e97423;
        var_tmf2_dn0 = assign62810_e97423_d_n0;
        var_tmf2_dn2 = assign62810_e97423_d_n2;
        var_tmf2_dn4 = assign62810_e97423_d_n4;
        var_tmf2_dn5 = assign62810_e97423_d_n5;
        var_tmf2_dn6 = assign62810_e97423_d_n6;
        var_tmf2_dn7 = assign62810_e97423_d_n7;
        var_tmf2_dn8 = assign62810_e97423_d_n8;
        var_tmf2_dn9 = assign62810_e97423_d_n9;
        var_tmf2_dn10 = assign62810_e97423_d_n10;
        var_tmf2_dn13 = assign62810_e97423_d_n13;
        var_tmf2_rv = 0.0;

        let (assign62820_e97439, assign62820_e97439_d_n0, assign62820_e97439_d_n2, assign62820_e97439_d_n4, assign62820_e97439_d_n5, assign62820_e97439_d_n6, assign62820_e97439_d_n7, assign62820_e97439_d_n8, assign62820_e97439_d_n9, assign62820_e97439_d_n10, assign62820_e97439_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 != 0.0)) && (var_guard1500 != 0.0)) {
        let assign62820_e97434: f64 = (var_tmf1 * var_tmf1);
        let assign62820_e97436: f64 = (assign62820_e97434 + var_tmf2);
        let assign62820_e97437: f64 = (assign62820_e97436).sqrt();
        (assign62820_e97437, ((((var_tmf1_dn0 * var_tmf1) + (var_tmf1 * var_tmf1_dn0)) + var_tmf2_dn0) / (2.0 * assign62820_e97437)), ((((var_tmf1_dn2 * var_tmf1) + (var_tmf1 * var_tmf1_dn2)) + var_tmf2_dn2) / (2.0 * assign62820_e97437)), ((((var_tmf1_dn4 * var_tmf1) + (var_tmf1 * var_tmf1_dn4)) + var_tmf2_dn4) / (2.0 * assign62820_e97437)), ((((var_tmf1_dn5 * var_tmf1) + (var_tmf1 * var_tmf1_dn5)) + var_tmf2_dn5) / (2.0 * assign62820_e97437)), ((((var_tmf1_dn6 * var_tmf1) + (var_tmf1 * var_tmf1_dn6)) + var_tmf2_dn6) / (2.0 * assign62820_e97437)), ((((var_tmf1_dn7 * var_tmf1) + (var_tmf1 * var_tmf1_dn7)) + var_tmf2_dn7) / (2.0 * assign62820_e97437)), ((((var_tmf1_dn8 * var_tmf1) + (var_tmf1 * var_tmf1_dn8)) + var_tmf2_dn8) / (2.0 * assign62820_e97437)), ((((var_tmf1_dn9 * var_tmf1) + (var_tmf1 * var_tmf1_dn9)) + var_tmf2_dn9) / (2.0 * assign62820_e97437)), ((((var_tmf1_dn10 * var_tmf1) + (var_tmf1 * var_tmf1_dn10)) + var_tmf2_dn10) / (2.0 * assign62820_e97437)), ((((var_tmf1_dn13 * var_tmf1) + (var_tmf1 * var_tmf1_dn13)) + var_tmf2_dn13) / (2.0 * assign62820_e97437)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
    }
};
        var_tmf2 = assign62820_e97439;
        var_tmf2_dn0 = assign62820_e97439_d_n0;
        var_tmf2_dn2 = assign62820_e97439_d_n2;
        var_tmf2_dn4 = assign62820_e97439_d_n4;
        var_tmf2_dn5 = assign62820_e97439_d_n5;
        var_tmf2_dn6 = assign62820_e97439_d_n6;
        var_tmf2_dn7 = assign62820_e97439_d_n7;
        var_tmf2_dn8 = assign62820_e97439_d_n8;
        var_tmf2_dn9 = assign62820_e97439_d_n9;
        var_tmf2_dn10 = assign62820_e97439_d_n10;
        var_tmf2_dn13 = assign62820_e97439_d_n13;
        var_tmf2_rv = 0.0;

        let (assign62830_e97456, assign62830_e97456_d_n0, assign62830_e97456_d_n2, assign62830_e97456_d_n4, assign62830_e97456_d_n5, assign62830_e97456_d_n6, assign62830_e97456_d_n7, assign62830_e97456_d_n8, assign62830_e97456_d_n9, assign62830_e97456_d_n10, assign62830_e97456_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 != 0.0)) && (var_guard1500 != 0.0)) {
        let assign62830_e97452: f64 = (var_tmf1 / var_tmf2);
        let assign62830_e97453: f64 = (1.0 + assign62830_e97452);
        let assign62830_e97454: f64 = (0.5 * assign62830_e97453);
        (assign62830_e97454, (0.5 * (((var_tmf1_dn0 * var_tmf2) - (var_tmf1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn2 * var_tmf2) - (var_tmf1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn4 * var_tmf2) - (var_tmf1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn5 * var_tmf2) - (var_tmf1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn6 * var_tmf2) - (var_tmf1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn7 * var_tmf2) - (var_tmf1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn8 * var_tmf2) - (var_tmf1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn9 * var_tmf2) - (var_tmf1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn10 * var_tmf2) - (var_tmf1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_tmf1_dn13 * var_tmf2) - (var_tmf1 * var_tmf2_dn13)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign62830_e97456;
        var_t0_dn0 = assign62830_e97456_d_n0;
        var_t0_dn2 = assign62830_e97456_d_n2;
        var_t0_dn4 = assign62830_e97456_d_n4;
        var_t0_dn5 = assign62830_e97456_d_n5;
        var_t0_dn6 = assign62830_e97456_d_n6;
        var_t0_dn7 = assign62830_e97456_d_n7;
        var_t0_dn8 = assign62830_e97456_d_n8;
        var_t0_dn9 = assign62830_e97456_d_n9;
        var_t0_dn10 = assign62830_e97456_d_n10;
        var_t0_dn13 = assign62830_e97456_d_n13;
        var_t0_rv = 0.0;

        let (assign62840_e97473, assign62840_e97473_d_n0, assign62840_e97473_d_n2, assign62840_e97473_d_n4, assign62840_e97473_d_n5, assign62840_e97473_d_n6, assign62840_e97473_d_n7, assign62840_e97473_d_n8, assign62840_e97473_d_n9, assign62840_e97473_d_n10, assign62840_e97473_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 != 0.0)) && (var_guard1500 != 0.0)) {
        let assign62840_e97469: f64 = (var_tmf1 + var_tmf2);
        let assign62840_e97470: f64 = (0.5 * assign62840_e97469);
        let assign62840_e97471: f64 = (var_t3 - assign62840_e97470);
        (assign62840_e97471, (var_t3_dn0 - (0.5 * (var_tmf1_dn0 + var_tmf2_dn0))), (var_t3_dn2 - (0.5 * (var_tmf1_dn2 + var_tmf2_dn2))), (var_t3_dn4 - (0.5 * (var_tmf1_dn4 + var_tmf2_dn4))), (var_t3_dn5 - (0.5 * (var_tmf1_dn5 + var_tmf2_dn5))), (var_t3_dn6 - (0.5 * (var_tmf1_dn6 + var_tmf2_dn6))), (var_t3_dn7 - (0.5 * (var_tmf1_dn7 + var_tmf2_dn7))), (var_t3_dn8 - (0.5 * (var_tmf1_dn8 + var_tmf2_dn8))), (var_t3_dn9 - (0.5 * (var_tmf1_dn9 + var_tmf2_dn9))), (var_t3_dn10 - (0.5 * (var_tmf1_dn10 + var_tmf2_dn10))), (var_t3_dn13 - (0.5 * (var_tmf1_dn13 + var_tmf2_dn13))),)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn7, var_t7_dn8, var_t7_dn9, var_t7_dn10, var_t7_dn13,)
    }
};
        var_t7 = assign62840_e97473;
        var_t7_dn0 = assign62840_e97473_d_n0;
        var_t7_dn2 = assign62840_e97473_d_n2;
        var_t7_dn4 = assign62840_e97473_d_n4;
        var_t7_dn5 = assign62840_e97473_d_n5;
        var_t7_dn6 = assign62840_e97473_d_n6;
        var_t7_dn7 = assign62840_e97473_d_n7;
        var_t7_dn8 = assign62840_e97473_d_n8;
        var_t7_dn9 = assign62840_e97473_d_n9;
        var_t7_dn10 = assign62840_e97473_d_n10;
        var_t7_dn13 = assign62840_e97473_d_n13;
        var_t7_rv = 0.0;

        let (assign62850_e97485, assign62850_e97485_d_n0, assign62850_e97485_d_n2, assign62850_e97485_d_n4, assign62850_e97485_d_n5, assign62850_e97485_d_n6, assign62850_e97485_d_n7, assign62850_e97485_d_n8, assign62850_e97485_d_n9, assign62850_e97485_d_n10, assign62850_e97485_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 != 0.0)) && (var_guard1500 == 0.0)) {
        (var_rd23e, var_rd23e_dn0, var_rd23e_dn2, var_rd23e_dn4, var_rd23e_dn5, var_rd23e_dn6, var_rd23e_dn7, var_rd23e_dn8, var_rd23e_dn9, var_rd23e_dn10, var_rd23e_dn13,)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn7, var_t7_dn8, var_t7_dn9, var_t7_dn10, var_t7_dn13,)
    }
};
        var_t7 = assign62850_e97485;
        var_t7_dn0 = assign62850_e97485_d_n0;
        var_t7_dn2 = assign62850_e97485_d_n2;
        var_t7_dn4 = assign62850_e97485_d_n4;
        var_t7_dn5 = assign62850_e97485_d_n5;
        var_t7_dn6 = assign62850_e97485_d_n6;
        var_t7_dn7 = assign62850_e97485_d_n7;
        var_t7_dn8 = assign62850_e97485_d_n8;
        var_t7_dn9 = assign62850_e97485_d_n9;
        var_t7_dn10 = assign62850_e97485_d_n10;
        var_t7_dn13 = assign62850_e97485_d_n13;
        var_t7_rv = 0.0;

        let assign62860_e97488: f64 = if var_vdse >= 0.0 { 1.0 } else { 0.0 };
        var_guard1501 = assign62860_e97488;
        var_guard1501_rv = 0.0;

        let (assign62870_e97499, assign62870_e97499_d_n0, assign62870_e97499_d_n2,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 != 0.0)) && (var_guard1501 != 0.0)) {
        (var_vdse, var_vdse_dn0, var_vdse_dn2,)
    } else {
        (var_vdse_eff, var_vdse_eff_dn0, var_vdse_eff_dn2,)
    }
};
        var_vdse_eff = assign62870_e97499;
        var_vdse_eff_dn0 = assign62870_e97499_d_n0;
        var_vdse_eff_dn2 = assign62870_e97499_d_n2;
        var_vdse_eff_rv = 0.0;

        let (assign62880_e97511, assign62880_e97511_d_n0, assign62880_e97511_d_n2,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 != 0.0)) && (var_guard1501 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (var_vdse_eff, var_vdse_eff_dn0, var_vdse_eff_dn2,)
    }
};
        var_vdse_eff = assign62880_e97511;
        var_vdse_eff_dn0 = assign62880_e97511_d_n0;
        var_vdse_eff_dn2 = assign62880_e97511_d_n2;
        var_vdse_eff_rv = 0.0;

        let assign62890_e97515: f64 = (20.0 * 1e-12);
        let assign62890_e97516: f64 = if var_vdse_eff < assign62890_e97515 { 1.0 } else { 0.0 };
        var_guard1502 = assign62890_e97516;
        var_guard1502_rv = 0.0;

        let (assign62900_e97547,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 != 0.0)) && (var_guard1502 != 0.0)) {
        let assign62900_e97527: f64 = (20.0 + 1.0);
        let assign62900_e97530: f64 = (p.p297 - 1.0);
        let assign62900_e97531: f64 = (assign62900_e97527).powf(assign62900_e97530);
        let assign62900_e97534: f64 = (20.0 + 1.0);
        let assign62900_e97537: f64 = (0.5 * p.p297);
        let assign62900_e97539: f64 = (assign62900_e97537 * 20.0);
        let assign62900_e97540: f64 = (assign62900_e97534 - assign62900_e97539);
        let assign62900_e97541: f64 = (assign62900_e97531 * assign62900_e97540);
        let assign62900_e97544: f64 = (1e-12_f64).powf(p.p297);
        let assign62900_e97545: f64 = (assign62900_e97541 * assign62900_e97544);
        (assign62900_e97545,)
    } else {
        (var_ra_alpha,)
    }
};
        var_ra_alpha = assign62900_e97547;
        var_ra_alpha_rv = 0.0;

        let (assign62910_e97576,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 != 0.0)) && (var_guard1502 != 0.0)) {
        let assign62910_e97558: f64 = (0.5 * p.p297);
        let assign62910_e97561: f64 = (20.0 + 1.0);
        let assign62910_e97564: f64 = (p.p297 - 1.0);
        let assign62910_e97565: f64 = (assign62910_e97561).powf(assign62910_e97564);
        let assign62910_e97566: f64 = (assign62910_e97558 * assign62910_e97565);
        let assign62910_e97568: f64 = (assign62910_e97566 / 20.0);
        let assign62910_e97572: f64 = (p.p297 - 2.0);
        let assign62910_e97573: f64 = (1e-12_f64).powf(assign62910_e97572);
        let assign62910_e97574: f64 = (assign62910_e97568 * assign62910_e97573);
        (assign62910_e97574,)
    } else {
        (var_ra_beta,)
    }
};
        var_ra_beta = assign62910_e97576;
        var_ra_beta_rv = 0.0;

        let (assign62920_e97593, assign62920_e97593_d_n0, assign62920_e97593_d_n2, assign62920_e97593_d_n4, assign62920_e97593_d_n5, assign62920_e97593_d_n6, assign62920_e97593_d_n7, assign62920_e97593_d_n8, assign62920_e97593_d_n9, assign62920_e97593_d_n10, assign62920_e97593_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 != 0.0)) && (var_guard1502 != 0.0)) {
        let assign62920_e97588: f64 = (var_ra_beta * var_vdse_eff);
        let assign62920_e97590: f64 = (assign62920_e97588 * var_vdse_eff);
        let assign62920_e97591: f64 = (var_ra_alpha + assign62920_e97590);
        (assign62920_e97591, (((var_ra_beta * var_vdse_eff_dn0) * var_vdse_eff) + (assign62920_e97588 * var_vdse_eff_dn0)), (((var_ra_beta * var_vdse_eff_dn2) * var_vdse_eff) + (assign62920_e97588 * var_vdse_eff_dn2)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign62920_e97593;
        var_t1_dn0 = assign62920_e97593_d_n0;
        var_t1_dn2 = assign62920_e97593_d_n2;
        var_t1_dn4 = assign62920_e97593_d_n4;
        var_t1_dn5 = assign62920_e97593_d_n5;
        var_t1_dn6 = assign62920_e97593_d_n6;
        var_t1_dn7 = assign62920_e97593_d_n7;
        var_t1_dn8 = assign62920_e97593_d_n8;
        var_t1_dn9 = assign62920_e97593_d_n9;
        var_t1_dn10 = assign62920_e97593_d_n10;
        var_t1_dn13 = assign62920_e97593_d_n13;
        var_t1_rv = 0.0;

        let (assign62930_e97609, assign62930_e97609_d_n0, assign62930_e97609_d_n2, assign62930_e97609_d_n4, assign62930_e97609_d_n5, assign62930_e97609_d_n6, assign62930_e97609_d_n7, assign62930_e97609_d_n8, assign62930_e97609_d_n9, assign62930_e97609_d_n10, assign62930_e97609_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 != 0.0)) && (var_guard1502 == 0.0)) {
        let assign62930_e97605: f64 = (var_vdse_eff + 1e-12);
        let assign62930_e97607: f64 = (assign62930_e97605).powf(p.p297);
        (assign62930_e97607, if 0.0 == 0.0 && ((p.p297) as f64).is_finite() && ((p.p297) as f64).fract() == 0.0 { if p.p297 == 0.0 { 0.0 } else { (p.p297 * ((assign62930_e97605).powf(p.p297 - 1.0) * var_vdse_eff_dn0)) } } else { (assign62930_e97607 * (p.p297 * (var_vdse_eff_dn0 / assign62930_e97605))) }, if 0.0 == 0.0 && ((p.p297) as f64).is_finite() && ((p.p297) as f64).fract() == 0.0 { if p.p297 == 0.0 { 0.0 } else { (p.p297 * ((assign62930_e97605).powf(p.p297 - 1.0) * var_vdse_eff_dn2)) } } else { (assign62930_e97607 * (p.p297 * (var_vdse_eff_dn2 / assign62930_e97605))) }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign62930_e97609;
        var_t1_dn0 = assign62930_e97609_d_n0;
        var_t1_dn2 = assign62930_e97609_d_n2;
        var_t1_dn4 = assign62930_e97609_d_n4;
        var_t1_dn5 = assign62930_e97609_d_n5;
        var_t1_dn6 = assign62930_e97609_d_n6;
        var_t1_dn7 = assign62930_e97609_d_n7;
        var_t1_dn8 = assign62930_e97609_d_n8;
        var_t1_dn9 = assign62930_e97609_d_n9;
        var_t1_dn10 = assign62930_e97609_d_n10;
        var_t1_dn13 = assign62930_e97609_d_n13;
        var_t1_rv = 0.0;

        let (assign62940_e97622, assign62940_e97622_d_n0, assign62940_e97622_d_n2, assign62940_e97622_d_n4, assign62940_e97622_d_n5, assign62940_e97622_d_n6, assign62940_e97622_d_n7, assign62940_e97622_d_n8, assign62940_e97622_d_n9, assign62940_e97622_d_n10, assign62940_e97622_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 != 0.0)) {
        let assign62940_e97618: f64 = (var_vdse_eff + 1e-12);
        let assign62940_e97620: f64 = (assign62940_e97618).powf(p.p299);
        (assign62940_e97620, if 0.0 == 0.0 && ((p.p299) as f64).is_finite() && ((p.p299) as f64).fract() == 0.0 { if p.p299 == 0.0 { 0.0 } else { (p.p299 * ((assign62940_e97618).powf(p.p299 - 1.0) * var_vdse_eff_dn0)) } } else { (assign62940_e97620 * (p.p299 * (var_vdse_eff_dn0 / assign62940_e97618))) }, if 0.0 == 0.0 && ((p.p299) as f64).is_finite() && ((p.p299) as f64).fract() == 0.0 { if p.p299 == 0.0 { 0.0 } else { (p.p299 * ((assign62940_e97618).powf(p.p299 - 1.0) * var_vdse_eff_dn2)) } } else { (assign62940_e97620 * (p.p299 * (var_vdse_eff_dn2 / assign62940_e97618))) }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t9, var_t9_dn0, var_t9_dn2, var_t9_dn4, var_t9_dn5, var_t9_dn6, var_t9_dn7, var_t9_dn8, var_t9_dn9, var_t9_dn10, var_t9_dn13,)
    }
};
        var_t9 = assign62940_e97622;
        var_t9_dn0 = assign62940_e97622_d_n0;
        var_t9_dn2 = assign62940_e97622_d_n2;
        var_t9_dn4 = assign62940_e97622_d_n4;
        var_t9_dn5 = assign62940_e97622_d_n5;
        var_t9_dn6 = assign62940_e97622_d_n6;
        var_t9_dn7 = assign62940_e97622_d_n7;
        var_t9_dn8 = assign62940_e97622_d_n8;
        var_t9_dn9 = assign62940_e97622_d_n9;
        var_t9_dn10 = assign62940_e97622_d_n10;
        var_t9_dn13 = assign62940_e97622_d_n13;
        var_t9_rv = 0.0;

        let (assign62950_e97641, assign62950_e97641_d_n0, assign62950_e97641_d_n2, assign62950_e97641_d_n4, assign62950_e97641_d_n5, assign62950_e97641_d_n6, assign62950_e97641_d_n7, assign62950_e97641_d_n8, assign62950_e97641_d_n9, assign62950_e97641_d_n10, assign62950_e97641_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 != 0.0)) {
        let assign62950_e97631: f64 = (var_t7 * var_t1);
        let assign62950_e97634: f64 = (var_vbse * var_uc_rd22);
        let assign62950_e97636: f64 = (assign62950_e97634 * var_t9);
        let assign62950_e97637: f64 = (assign62950_e97631 + assign62950_e97636);
        let assign62950_e97639: f64 = (assign62950_e97637 / var_weff_nf);
        (assign62950_e97639, ((((var_t7_dn0 * var_t1) + (var_t7 * var_t1_dn0)) + (((var_vbse_dn0 * var_uc_rd22) * var_t9) + (assign62950_e97634 * var_t9_dn0))) / var_weff_nf), ((((var_t7_dn2 * var_t1) + (var_t7 * var_t1_dn2)) + (((var_vbse_dn2 * var_uc_rd22) * var_t9) + (assign62950_e97634 * var_t9_dn2))) / var_weff_nf), ((((var_t7_dn4 * var_t1) + (var_t7 * var_t1_dn4)) + (assign62950_e97634 * var_t9_dn4)) / var_weff_nf), ((((var_t7_dn5 * var_t1) + (var_t7 * var_t1_dn5)) + (assign62950_e97634 * var_t9_dn5)) / var_weff_nf), ((((var_t7_dn6 * var_t1) + (var_t7 * var_t1_dn6)) + (assign62950_e97634 * var_t9_dn6)) / var_weff_nf), ((((var_t7_dn7 * var_t1) + (var_t7 * var_t1_dn7)) + (assign62950_e97634 * var_t9_dn7)) / var_weff_nf), ((((var_t7_dn8 * var_t1) + (var_t7 * var_t1_dn8)) + (((var_vbse_dn8 * var_uc_rd22) * var_t9) + (assign62950_e97634 * var_t9_dn8))) / var_weff_nf), ((((var_t7_dn9 * var_t1) + (var_t7 * var_t1_dn9)) + (assign62950_e97634 * var_t9_dn9)) / var_weff_nf), ((((var_t7_dn10 * var_t1) + (var_t7 * var_t1_dn10)) + (assign62950_e97634 * var_t9_dn10)) / var_weff_nf), ((((var_t7_dn13 * var_t1) + (var_t7 * var_t1_dn13)) + (assign62950_e97634 * var_t9_dn13)) / var_weff_nf),)
    } else {
        (var_ra, var_ra_dn0, var_ra_dn2, var_ra_dn4, var_ra_dn5, var_ra_dn6, var_ra_dn7, var_ra_dn8, var_ra_dn9, var_ra_dn10, var_ra_dn13,)
    }
};
        var_ra = assign62950_e97641;
        var_ra_dn0 = assign62950_e97641_d_n0;
        var_ra_dn2 = assign62950_e97641_d_n2;
        var_ra_dn4 = assign62950_e97641_d_n4;
        var_ra_dn5 = assign62950_e97641_d_n5;
        var_ra_dn6 = assign62950_e97641_d_n6;
        var_ra_dn7 = assign62950_e97641_d_n7;
        var_ra_dn8 = assign62950_e97641_d_n8;
        var_ra_dn9 = assign62950_e97641_d_n9;
        var_ra_dn10 = assign62950_e97641_d_n10;
        var_ra_dn13 = assign62950_e97641_d_n13;
        var_ra_rv = 0.0;

        let (assign62960_e97652, assign62960_e97652_d_n0, assign62960_e97652_d_n2, assign62960_e97652_d_n4, assign62960_e97652_d_n5, assign62960_e97652_d_n6, assign62960_e97652_d_n7, assign62960_e97652_d_n8, assign62960_e97652_d_n9, assign62960_e97652_d_n10, assign62960_e97652_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 != 0.0)) {
        let assign62960_e97650: f64 = (var_ra * var_ids0);
        (assign62960_e97650, ((var_ra_dn0 * var_ids0) + (var_ra * var_ids0_dn0)), ((var_ra_dn2 * var_ids0) + (var_ra * var_ids0_dn2)), ((var_ra_dn4 * var_ids0) + (var_ra * var_ids0_dn4)), ((var_ra_dn5 * var_ids0) + (var_ra * var_ids0_dn5)), ((var_ra_dn6 * var_ids0) + (var_ra * var_ids0_dn6)), ((var_ra_dn7 * var_ids0) + (var_ra * var_ids0_dn7)), ((var_ra_dn8 * var_ids0) + (var_ra * var_ids0_dn8)), ((var_ra_dn9 * var_ids0) + (var_ra * var_ids0_dn9)), ((var_ra_dn10 * var_ids0) + (var_ra * var_ids0_dn10)), ((var_ra_dn13 * var_ids0) + (var_ra * var_ids0_dn13)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign62960_e97652;
        var_t0_dn0 = assign62960_e97652_d_n0;
        var_t0_dn2 = assign62960_e97652_d_n2;
        var_t0_dn4 = assign62960_e97652_d_n4;
        var_t0_dn5 = assign62960_e97652_d_n5;
        var_t0_dn6 = assign62960_e97652_d_n6;
        var_t0_dn7 = assign62960_e97652_d_n7;
        var_t0_dn8 = assign62960_e97652_d_n8;
        var_t0_dn9 = assign62960_e97652_d_n9;
        var_t0_dn10 = assign62960_e97652_d_n10;
        var_t0_dn13 = assign62960_e97652_d_n13;
        var_t0_rv = 0.0;

        let (assign62970_e97663, assign62970_e97663_d_n0, assign62970_e97663_d_n2, assign62970_e97663_d_n4, assign62970_e97663_d_n5, assign62970_e97663_d_n6, assign62970_e97663_d_n7, assign62970_e97663_d_n8, assign62970_e97663_d_n9, assign62970_e97663_d_n10, assign62970_e97663_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 != 0.0)) {
        let assign62970_e97661: f64 = (var_vds + 1e-12);
        (assign62970_e97661, var_vds_dn0, var_vds_dn2, var_vds_dn4, var_vds_dn5, var_vds_dn6, var_vds_dn7, var_vds_dn8, var_vds_dn9, var_vds_dn10, var_vds_dn13,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign62970_e97663;
        var_t1_dn0 = assign62970_e97663_d_n0;
        var_t1_dn2 = assign62970_e97663_d_n2;
        var_t1_dn4 = assign62970_e97663_d_n4;
        var_t1_dn5 = assign62970_e97663_d_n5;
        var_t1_dn6 = assign62970_e97663_d_n6;
        var_t1_dn7 = assign62970_e97663_d_n7;
        var_t1_dn8 = assign62970_e97663_d_n8;
        var_t1_dn9 = assign62970_e97663_d_n9;
        var_t1_dn10 = assign62970_e97663_d_n10;
        var_t1_dn13 = assign62970_e97663_d_n13;
        var_t1_rv = 0.0;

        let (assign62980_e97674, assign62980_e97674_d_n0, assign62980_e97674_d_n2, assign62980_e97674_d_n4, assign62980_e97674_d_n5, assign62980_e97674_d_n6, assign62980_e97674_d_n7, assign62980_e97674_d_n8, assign62980_e97674_d_n9, assign62980_e97674_d_n10, assign62980_e97674_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 != 0.0)) {
        let assign62980_e97672: f64 = (1.0 / var_t1);
        (assign62980_e97672, (-(var_t1_dn0 / (var_t1 * var_t1))), (-(var_t1_dn2 / (var_t1 * var_t1))), (-(var_t1_dn4 / (var_t1 * var_t1))), (-(var_t1_dn5 / (var_t1 * var_t1))), (-(var_t1_dn6 / (var_t1 * var_t1))), (-(var_t1_dn7 / (var_t1 * var_t1))), (-(var_t1_dn8 / (var_t1 * var_t1))), (-(var_t1_dn9 / (var_t1 * var_t1))), (-(var_t1_dn10 / (var_t1 * var_t1))), (-(var_t1_dn13 / (var_t1 * var_t1))),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign62980_e97674;
        var_t2_dn0 = assign62980_e97674_d_n0;
        var_t2_dn2 = assign62980_e97674_d_n2;
        var_t2_dn4 = assign62980_e97674_d_n4;
        var_t2_dn5 = assign62980_e97674_d_n5;
        var_t2_dn6 = assign62980_e97674_d_n6;
        var_t2_dn7 = assign62980_e97674_d_n7;
        var_t2_dn8 = assign62980_e97674_d_n8;
        var_t2_dn9 = assign62980_e97674_d_n9;
        var_t2_dn10 = assign62980_e97674_d_n10;
        var_t2_dn13 = assign62980_e97674_d_n13;
        var_t2_rv = 0.0;

        let (assign62990_e97687, assign62990_e97687_d_n0, assign62990_e97687_d_n2, assign62990_e97687_d_n4, assign62990_e97687_d_n5, assign62990_e97687_d_n6, assign62990_e97687_d_n7, assign62990_e97687_d_n8, assign62990_e97687_d_n9, assign62990_e97687_d_n10, assign62990_e97687_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 != 0.0)) {
        let assign62990_e97684: f64 = (var_t0 * var_t2);
        let assign62990_e97685: f64 = (1.0 + assign62990_e97684);
        (assign62990_e97685, ((var_t0_dn0 * var_t2) + (var_t0 * var_t2_dn0)), ((var_t0_dn2 * var_t2) + (var_t0 * var_t2_dn2)), ((var_t0_dn4 * var_t2) + (var_t0 * var_t2_dn4)), ((var_t0_dn5 * var_t2) + (var_t0 * var_t2_dn5)), ((var_t0_dn6 * var_t2) + (var_t0 * var_t2_dn6)), ((var_t0_dn7 * var_t2) + (var_t0 * var_t2_dn7)), ((var_t0_dn8 * var_t2) + (var_t0 * var_t2_dn8)), ((var_t0_dn9 * var_t2) + (var_t0 * var_t2_dn9)), ((var_t0_dn10 * var_t2) + (var_t0 * var_t2_dn10)), ((var_t0_dn13 * var_t2) + (var_t0 * var_t2_dn13)),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn13,)
    }
};
        var_t3 = assign62990_e97687;
        var_t3_dn0 = assign62990_e97687_d_n0;
        var_t3_dn2 = assign62990_e97687_d_n2;
        var_t3_dn4 = assign62990_e97687_d_n4;
        var_t3_dn5 = assign62990_e97687_d_n5;
        var_t3_dn6 = assign62990_e97687_d_n6;
        var_t3_dn7 = assign62990_e97687_d_n7;
        var_t3_dn8 = assign62990_e97687_d_n8;
        var_t3_dn9 = assign62990_e97687_d_n9;
        var_t3_dn10 = assign62990_e97687_d_n10;
        var_t3_dn13 = assign62990_e97687_d_n13;
        var_t3_rv = 0.0;

        let (assign63000_e97698, assign63000_e97698_d_n0, assign63000_e97698_d_n2, assign63000_e97698_d_n4, assign63000_e97698_d_n5, assign63000_e97698_d_n6, assign63000_e97698_d_n7, assign63000_e97698_d_n8, assign63000_e97698_d_n9, assign63000_e97698_d_n10, assign63000_e97698_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 != 0.0)) {
        let assign63000_e97696: f64 = (1.0 / var_t3);
        (assign63000_e97696, (-(var_t3_dn0 / (var_t3 * var_t3))), (-(var_t3_dn2 / (var_t3 * var_t3))), (-(var_t3_dn4 / (var_t3 * var_t3))), (-(var_t3_dn5 / (var_t3 * var_t3))), (-(var_t3_dn6 / (var_t3 * var_t3))), (-(var_t3_dn7 / (var_t3 * var_t3))), (-(var_t3_dn8 / (var_t3 * var_t3))), (-(var_t3_dn9 / (var_t3 * var_t3))), (-(var_t3_dn10 / (var_t3 * var_t3))), (-(var_t3_dn13 / (var_t3 * var_t3))),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn13,)
    }
};
        var_t4 = assign63000_e97698;
        var_t4_dn0 = assign63000_e97698_d_n0;
        var_t4_dn2 = assign63000_e97698_d_n2;
        var_t4_dn4 = assign63000_e97698_d_n4;
        var_t4_dn5 = assign63000_e97698_d_n5;
        var_t4_dn6 = assign63000_e97698_d_n6;
        var_t4_dn7 = assign63000_e97698_d_n7;
        var_t4_dn8 = assign63000_e97698_d_n8;
        var_t4_dn9 = assign63000_e97698_d_n9;
        var_t4_dn10 = assign63000_e97698_d_n10;
        var_t4_dn13 = assign63000_e97698_d_n13;
        var_t4_rv = 0.0;

        let (assign63010_e97709, assign63010_e97709_d_n0, assign63010_e97709_d_n2, assign63010_e97709_d_n4, assign63010_e97709_d_n5, assign63010_e97709_d_n6, assign63010_e97709_d_n7, assign63010_e97709_d_n8, assign63010_e97709_d_n9, assign63010_e97709_d_n10, assign63010_e97709_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 != 0.0)) {
        let assign63010_e97707: f64 = (var_ids0 * var_t4);
        (assign63010_e97707, ((var_ids0_dn0 * var_t4) + (var_ids0 * var_t4_dn0)), ((var_ids0_dn2 * var_t4) + (var_ids0 * var_t4_dn2)), ((var_ids0_dn4 * var_t4) + (var_ids0 * var_t4_dn4)), ((var_ids0_dn5 * var_t4) + (var_ids0 * var_t4_dn5)), ((var_ids0_dn6 * var_t4) + (var_ids0 * var_t4_dn6)), ((var_ids0_dn7 * var_t4) + (var_ids0 * var_t4_dn7)), ((var_ids0_dn8 * var_t4) + (var_ids0 * var_t4_dn8)), ((var_ids0_dn9 * var_t4) + (var_ids0 * var_t4_dn9)), ((var_ids0_dn10 * var_t4) + (var_ids0 * var_t4_dn10)), ((var_ids0_dn13 * var_t4) + (var_ids0 * var_t4_dn13)),)
    } else {
        (var_ids, var_ids_dn0, var_ids_dn2, var_ids_dn4, var_ids_dn5, var_ids_dn6, var_ids_dn7, var_ids_dn8, var_ids_dn9, var_ids_dn10, var_ids_dn13,)
    }
};
        var_ids = assign63010_e97709;
        var_ids_dn0 = assign63010_e97709_d_n0;
        var_ids_dn2 = assign63010_e97709_d_n2;
        var_ids_dn4 = assign63010_e97709_d_n4;
        var_ids_dn5 = assign63010_e97709_d_n5;
        var_ids_dn6 = assign63010_e97709_d_n6;
        var_ids_dn7 = assign63010_e97709_d_n7;
        var_ids_dn8 = assign63010_e97709_d_n8;
        var_ids_dn9 = assign63010_e97709_d_n9;
        var_ids_dn10 = assign63010_e97709_d_n10;
        var_ids_dn13 = assign63010_e97709_d_n13;
        var_ids_rv = 0.0;

        let (assign63020_e97719, assign63020_e97719_d_n0, assign63020_e97719_d_n2, assign63020_e97719_d_n4, assign63020_e97719_d_n5, assign63020_e97719_d_n6, assign63020_e97719_d_n7, assign63020_e97719_d_n8, assign63020_e97719_d_n9, assign63020_e97719_d_n10, assign63020_e97719_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 == 0.0)) {
        (var_ids0, var_ids0_dn0, var_ids0_dn2, var_ids0_dn4, var_ids0_dn5, var_ids0_dn6, var_ids0_dn7, var_ids0_dn8, var_ids0_dn9, var_ids0_dn10, var_ids0_dn13,)
    } else {
        (var_ids, var_ids_dn0, var_ids_dn2, var_ids_dn4, var_ids_dn5, var_ids_dn6, var_ids_dn7, var_ids_dn8, var_ids_dn9, var_ids_dn10, var_ids_dn13,)
    }
};
        var_ids = assign63020_e97719;
        var_ids_dn0 = assign63020_e97719_d_n0;
        var_ids_dn2 = assign63020_e97719_d_n2;
        var_ids_dn4 = assign63020_e97719_d_n4;
        var_ids_dn5 = assign63020_e97719_d_n5;
        var_ids_dn6 = assign63020_e97719_d_n6;
        var_ids_dn7 = assign63020_e97719_d_n7;
        var_ids_dn8 = assign63020_e97719_d_n8;
        var_ids_dn9 = assign63020_e97719_d_n9;
        var_ids_dn10 = assign63020_e97719_d_n10;
        var_ids_dn13 = assign63020_e97719_d_n13;
        var_ids_rv = 0.0;

        let (assign63030_e97729, assign63030_e97729_d_n0, assign63030_e97729_d_n2, assign63030_e97729_d_n4, assign63030_e97729_d_n5, assign63030_e97729_d_n6, assign63030_e97729_d_n7, assign63030_e97729_d_n8, assign63030_e97729_d_n9, assign63030_e97729_d_n10, assign63030_e97729_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1499 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ra, var_ra_dn0, var_ra_dn2, var_ra_dn4, var_ra_dn5, var_ra_dn6, var_ra_dn7, var_ra_dn8, var_ra_dn9, var_ra_dn10, var_ra_dn13,)
    }
};
        var_ra = assign63030_e97729;
        var_ra_dn0 = assign63030_e97729_d_n0;
        var_ra_dn2 = assign63030_e97729_d_n2;
        var_ra_dn4 = assign63030_e97729_d_n4;
        var_ra_dn5 = assign63030_e97729_d_n5;
        var_ra_dn6 = assign63030_e97729_d_n6;
        var_ra_dn7 = assign63030_e97729_d_n7;
        var_ra_dn8 = assign63030_e97729_d_n8;
        var_ra_dn9 = assign63030_e97729_d_n9;
        var_ra_dn10 = assign63030_e97729_d_n10;
        var_ra_dn13 = assign63030_e97729_d_n13;
        var_ra_rv = 0.0;

        let assign63040_e97732: f64 = if p.p27 != 0.0 { 1.0 } else { 0.0 };
        var_guard1503 = assign63040_e97732;
        var_guard1503_rv = 0.0;

        let (assign63050_e97743, assign63050_e97743_d_n0, assign63050_e97743_d_n2, assign63050_e97743_d_n4, assign63050_e97743_d_n5, assign63050_e97743_d_n6, assign63050_e97743_d_n7, assign63050_e97743_d_n8, assign63050_e97743_d_n9, assign63050_e97743_d_n10, assign63050_e97743_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63050_e97741: f64 = (1.034943e-10 * var_cox_inv);
        (assign63050_e97741, (1.034943e-10 * var_cox_inv_dn0), (1.034943e-10 * var_cox_inv_dn2), (1.034943e-10 * var_cox_inv_dn4), (1.034943e-10 * var_cox_inv_dn5), (1.034943e-10 * var_cox_inv_dn6), (1.034943e-10 * var_cox_inv_dn7), (1.034943e-10 * var_cox_inv_dn8), (1.034943e-10 * var_cox_inv_dn9), (1.034943e-10 * var_cox_inv_dn10), (1.034943e-10 * var_cox_inv_dn13),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign63050_e97743;
        var_t1_dn0 = assign63050_e97743_d_n0;
        var_t1_dn2 = assign63050_e97743_d_n2;
        var_t1_dn4 = assign63050_e97743_d_n4;
        var_t1_dn5 = assign63050_e97743_d_n5;
        var_t1_dn6 = assign63050_e97743_d_n6;
        var_t1_dn7 = assign63050_e97743_d_n7;
        var_t1_dn8 = assign63050_e97743_d_n8;
        var_t1_dn9 = assign63050_e97743_d_n9;
        var_t1_dn10 = assign63050_e97743_d_n10;
        var_t1_dn13 = assign63050_e97743_d_n13;
        var_t1_rv = 0.0;

        let (assign63060_e97752, assign63060_e97752_d_n0, assign63060_e97752_d_n2, assign63060_e97752_d_n4, assign63060_e97752_d_n5, assign63060_e97752_d_n6, assign63060_e97752_d_n7, assign63060_e97752_d_n8, assign63060_e97752_d_n9, assign63060_e97752_d_n10, assign63060_e97752_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        (var_wdpl, var_wdpl_dn0, var_wdpl_dn2, var_wdpl_dn4, var_wdpl_dn5, var_wdpl_dn6, var_wdpl_dn7, var_wdpl_dn8, var_wdpl_dn9, var_wdpl_dn10, var_wdpl_dn13,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign63060_e97752;
        var_t2_dn0 = assign63060_e97752_d_n0;
        var_t2_dn2 = assign63060_e97752_d_n2;
        var_t2_dn4 = assign63060_e97752_d_n4;
        var_t2_dn5 = assign63060_e97752_d_n5;
        var_t2_dn6 = assign63060_e97752_d_n6;
        var_t2_dn7 = assign63060_e97752_d_n7;
        var_t2_dn8 = assign63060_e97752_d_n8;
        var_t2_dn9 = assign63060_e97752_d_n9;
        var_t2_dn10 = assign63060_e97752_d_n10;
        var_t2_dn13 = assign63060_e97752_d_n13;
        var_t2_rv = 0.0;

        *var_guard1501_slot = var_guard1501;
        *var_guard1501_rv_slot = var_guard1501_rv;
        *var_guard1502_slot = var_guard1502;
        *var_guard1502_rv_slot = var_guard1502_rv;
        *var_guard1503_slot = var_guard1503;
        *var_guard1503_rv_slot = var_guard1503_rv;
        *var_ids_slot = var_ids;
        *var_ids_dn0_slot = var_ids_dn0;
        *var_ids_dn10_slot = var_ids_dn10;
        *var_ids_dn13_slot = var_ids_dn13;
        *var_ids_dn2_slot = var_ids_dn2;
        *var_ids_dn4_slot = var_ids_dn4;
        *var_ids_dn5_slot = var_ids_dn5;
        *var_ids_dn6_slot = var_ids_dn6;
        *var_ids_dn7_slot = var_ids_dn7;
        *var_ids_dn8_slot = var_ids_dn8;
        *var_ids_dn9_slot = var_ids_dn9;
        *var_ids_rv_slot = var_ids_rv;
        *var_ra_slot = var_ra;
        *var_ra_alpha_slot = var_ra_alpha;
        *var_ra_alpha_rv_slot = var_ra_alpha_rv;
        *var_ra_beta_slot = var_ra_beta;
        *var_ra_beta_rv_slot = var_ra_beta_rv;
        *var_ra_dn0_slot = var_ra_dn0;
        *var_ra_dn10_slot = var_ra_dn10;
        *var_ra_dn13_slot = var_ra_dn13;
        *var_ra_dn2_slot = var_ra_dn2;
        *var_ra_dn4_slot = var_ra_dn4;
        *var_ra_dn5_slot = var_ra_dn5;
        *var_ra_dn6_slot = var_ra_dn6;
        *var_ra_dn7_slot = var_ra_dn7;
        *var_ra_dn8_slot = var_ra_dn8;
        *var_ra_dn9_slot = var_ra_dn9;
        *var_ra_rv_slot = var_ra_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn13_slot = var_t0_dn13;
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
        *var_t1_dn13_slot = var_t1_dn13;
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
        *var_t2_dn13_slot = var_t2_dn13;
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
        *var_t3_dn13_slot = var_t3_dn13;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t3_rv_slot = var_t3_rv;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn13_slot = var_t4_dn13;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_dn9_slot = var_t4_dn9;
        *var_t4_rv_slot = var_t4_rv;
        *var_t7_slot = var_t7;
        *var_t7_dn0_slot = var_t7_dn0;
        *var_t7_dn10_slot = var_t7_dn10;
        *var_t7_dn13_slot = var_t7_dn13;
        *var_t7_dn2_slot = var_t7_dn2;
        *var_t7_dn4_slot = var_t7_dn4;
        *var_t7_dn5_slot = var_t7_dn5;
        *var_t7_dn6_slot = var_t7_dn6;
        *var_t7_dn7_slot = var_t7_dn7;
        *var_t7_dn8_slot = var_t7_dn8;
        *var_t7_dn9_slot = var_t7_dn9;
        *var_t7_rv_slot = var_t7_rv;
        *var_t9_slot = var_t9;
        *var_t9_dn0_slot = var_t9_dn0;
        *var_t9_dn10_slot = var_t9_dn10;
        *var_t9_dn13_slot = var_t9_dn13;
        *var_t9_dn2_slot = var_t9_dn2;
        *var_t9_dn4_slot = var_t9_dn4;
        *var_t9_dn5_slot = var_t9_dn5;
        *var_t9_dn6_slot = var_t9_dn6;
        *var_t9_dn7_slot = var_t9_dn7;
        *var_t9_dn8_slot = var_t9_dn8;
        *var_t9_dn9_slot = var_t9_dn9;
        *var_t9_rv_slot = var_t9_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn13_slot = var_tmf2_dn13;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn7_slot = var_tmf2_dn7;
        *var_tmf2_dn8_slot = var_tmf2_dn8;
        *var_tmf2_dn9_slot = var_tmf2_dn9;
        *var_tmf2_rv_slot = var_tmf2_rv;
        *var_vdse_eff_slot = var_vdse_eff;
        *var_vdse_eff_dn0_slot = var_vdse_eff_dn0;
        *var_vdse_eff_dn2_slot = var_vdse_eff_dn2;
        *var_vdse_eff_rv_slot = var_vdse_eff_rv;
    }

    pub(super) fn stamp_reactive_block_226(
        p: &Parameters,
        var_beta: f64,
        var_beta_dn0: f64,
        var_beta_dn10: f64,
        var_beta_dn13: f64,
        var_beta_dn2: f64,
        var_beta_dn4: f64,
        var_beta_dn5: f64,
        var_beta_dn6: f64,
        var_beta_dn7: f64,
        var_beta_dn8: f64,
        var_beta_dn9: f64,
        var_beta_inv: f64,
        var_beta_inv_dn0: f64,
        var_beta_inv_dn10: f64,
        var_beta_inv_dn13: f64,
        var_beta_inv_dn2: f64,
        var_beta_inv_dn4: f64,
        var_beta_inv_dn5: f64,
        var_beta_inv_dn6: f64,
        var_beta_inv_dn7: f64,
        var_beta_inv_dn8: f64,
        var_beta_inv_dn9: f64,
        var_costi0_p2: f64,
        var_costi0_p2_dn0: f64,
        var_costi0_p2_dn10: f64,
        var_costi0_p2_dn13: f64,
        var_costi0_p2_dn2: f64,
        var_costi0_p2_dn4: f64,
        var_costi0_p2_dn5: f64,
        var_costi0_p2_dn6: f64,
        var_costi0_p2_dn7: f64,
        var_costi0_p2_dn8: f64,
        var_costi0_p2_dn9: f64,
        var_cox_inv: f64,
        var_cox_inv_dn0: f64,
        var_cox_inv_dn10: f64,
        var_cox_inv_dn13: f64,
        var_cox_inv_dn2: f64,
        var_cox_inv_dn4: f64,
        var_cox_inv_dn5: f64,
        var_cox_inv_dn6: f64,
        var_cox_inv_dn7: f64,
        var_cox_inv_dn8: f64,
        var_cox_inv_dn9: f64,
        var_guard1430: f64,
        var_guard1503: f64,
        var_guard443: f64,
        var_lgatesm: f64,
        var_pb20b: f64,
        var_pb20b_dn0: f64,
        var_pb20b_dn10: f64,
        var_pb20b_dn13: f64,
        var_pb20b_dn2: f64,
        var_pb20b_dn4: f64,
        var_pb20b_dn5: f64,
        var_pb20b_dn6: f64,
        var_pb20b_dn7: f64,
        var_pb20b_dn8: f64,
        var_pb20b_dn9: f64,
        var_sqrt_pbsum: f64,
        var_sqrt_pbsum_dn0: f64,
        var_sqrt_pbsum_dn10: f64,
        var_sqrt_pbsum_dn13: f64,
        var_sqrt_pbsum_dn2: f64,
        var_sqrt_pbsum_dn4: f64,
        var_sqrt_pbsum_dn5: f64,
        var_sqrt_pbsum_dn6: f64,
        var_sqrt_pbsum_dn7: f64,
        var_sqrt_pbsum_dn8: f64,
        var_sqrt_pbsum_dn9: f64,
        var_uc_scsti1: f64,
        var_uc_scsti2: f64,
        var_uc_vthsti: f64,
        var_vbsz__blk438: f64,
        var_vbsz__blk438_dn0: f64,
        var_vbsz__blk438_dn10: f64,
        var_vbsz__blk438_dn13: f64,
        var_vbsz__blk438_dn2: f64,
        var_vbsz__blk438_dn4: f64,
        var_vbsz__blk438_dn5: f64,
        var_vbsz__blk438_dn6: f64,
        var_vbsz__blk438_dn7: f64,
        var_vbsz__blk438_dn8: f64,
        var_vbsz__blk438_dn9: f64,
        var_vds: f64,
        var_vds_dn0: f64,
        var_vds_dn10: f64,
        var_vds_dn13: f64,
        var_vds_dn2: f64,
        var_vds_dn4: f64,
        var_vds_dn5: f64,
        var_vds_dn6: f64,
        var_vds_dn7: f64,
        var_vds_dn8: f64,
        var_vds_dn9: f64,
        var_vdsz__blk439: f64,
        var_vdsz__blk439_dn0: f64,
        var_vdsz__blk439_dn10: f64,
        var_vdsz__blk439_dn13: f64,
        var_vdsz__blk439_dn2: f64,
        var_vdsz__blk439_dn4: f64,
        var_vdsz__blk439_dn5: f64,
        var_vdsz__blk439_dn6: f64,
        var_vdsz__blk439_dn7: f64,
        var_vdsz__blk439_dn8: f64,
        var_vdsz__blk439_dn9: f64,
        var_vfb: f64,
        var_vgsz__blk440: f64,
        var_vgsz__blk440_dn0: f64,
        var_vgsz__blk440_dn10: f64,
        var_vgsz__blk440_dn13: f64,
        var_vgsz__blk440_dn2: f64,
        var_vgsz__blk440_dn4: f64,
        var_vgsz__blk440_dn5: f64,
        var_vgsz__blk440_dn6: f64,
        var_vgsz__blk440_dn7: f64,
        var_vgsz__blk440_dn8: f64,
        var_vgsz__blk440_dn9: f64,
        var_costi3_slot: &mut f64,
        var_costi3_dn0_slot: &mut f64,
        var_costi3_dn10_slot: &mut f64,
        var_costi3_dn13_slot: &mut f64,
        var_costi3_dn2_slot: &mut f64,
        var_costi3_dn4_slot: &mut f64,
        var_costi3_dn5_slot: &mut f64,
        var_costi3_dn6_slot: &mut f64,
        var_costi3_dn7_slot: &mut f64,
        var_costi3_dn8_slot: &mut f64,
        var_costi3_dn9_slot: &mut f64,
        var_costi3_rv_slot: &mut f64,
        var_costi4_slot: &mut f64,
        var_costi4_dn0_slot: &mut f64,
        var_costi4_dn10_slot: &mut f64,
        var_costi4_dn13_slot: &mut f64,
        var_costi4_dn2_slot: &mut f64,
        var_costi4_dn4_slot: &mut f64,
        var_costi4_dn5_slot: &mut f64,
        var_costi4_dn6_slot: &mut f64,
        var_costi4_dn7_slot: &mut f64,
        var_costi4_dn8_slot: &mut f64,
        var_costi4_dn9_slot: &mut f64,
        var_costi4_rv_slot: &mut f64,
        var_costi5_slot: &mut f64,
        var_costi5_dn0_slot: &mut f64,
        var_costi5_dn10_slot: &mut f64,
        var_costi5_dn13_slot: &mut f64,
        var_costi5_dn2_slot: &mut f64,
        var_costi5_dn4_slot: &mut f64,
        var_costi5_dn5_slot: &mut f64,
        var_costi5_dn6_slot: &mut f64,
        var_costi5_dn7_slot: &mut f64,
        var_costi5_dn8_slot: &mut f64,
        var_costi5_dn9_slot: &mut f64,
        var_costi5_rv_slot: &mut f64,
        var_dvth0_slot: &mut f64,
        var_dvth0_dn0_slot: &mut f64,
        var_dvth0_dn10_slot: &mut f64,
        var_dvth0_dn13_slot: &mut f64,
        var_dvth0_dn2_slot: &mut f64,
        var_dvth0_dn4_slot: &mut f64,
        var_dvth0_dn5_slot: &mut f64,
        var_dvth0_dn6_slot: &mut f64,
        var_dvth0_dn7_slot: &mut f64,
        var_dvth0_dn8_slot: &mut f64,
        var_dvth0_dn9_slot: &mut f64,
        var_dvth0_rv_slot: &mut f64,
        var_dvthscsti_slot: &mut f64,
        var_dvthscsti_dn0_slot: &mut f64,
        var_dvthscsti_dn10_slot: &mut f64,
        var_dvthscsti_dn13_slot: &mut f64,
        var_dvthscsti_dn2_slot: &mut f64,
        var_dvthscsti_dn4_slot: &mut f64,
        var_dvthscsti_dn5_slot: &mut f64,
        var_dvthscsti_dn6_slot: &mut f64,
        var_dvthscsti_dn7_slot: &mut f64,
        var_dvthscsti_dn8_slot: &mut f64,
        var_dvthscsti_dn9_slot: &mut f64,
        var_dvthscsti_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn13_slot: &mut f64,
        var_t0_dn2_slot: &mut f64,
        var_t0_dn4_slot: &mut f64,
        var_t0_dn5_slot: &mut f64,
        var_t0_dn6_slot: &mut f64,
        var_t0_dn7_slot: &mut f64,
        var_t0_dn8_slot: &mut f64,
        var_t0_dn9_slot: &mut f64,
        var_t0_rv_slot: &mut f64,
        var_t1_slot: &mut f64,
        var_t10_slot: &mut f64,
        var_t10_dn0_slot: &mut f64,
        var_t10_dn10_slot: &mut f64,
        var_t10_dn13_slot: &mut f64,
        var_t10_dn2_slot: &mut f64,
        var_t10_dn4_slot: &mut f64,
        var_t10_dn5_slot: &mut f64,
        var_t10_dn6_slot: &mut f64,
        var_t10_dn7_slot: &mut f64,
        var_t10_dn8_slot: &mut f64,
        var_t10_dn9_slot: &mut f64,
        var_t10_rv_slot: &mut f64,
        var_t11_slot: &mut f64,
        var_t11_dn0_slot: &mut f64,
        var_t11_dn10_slot: &mut f64,
        var_t11_dn13_slot: &mut f64,
        var_t11_dn2_slot: &mut f64,
        var_t11_dn4_slot: &mut f64,
        var_t11_dn5_slot: &mut f64,
        var_t11_dn6_slot: &mut f64,
        var_t11_dn7_slot: &mut f64,
        var_t11_dn8_slot: &mut f64,
        var_t11_dn9_slot: &mut f64,
        var_t11_rv_slot: &mut f64,
        var_t1_dn0_slot: &mut f64,
        var_t1_dn10_slot: &mut f64,
        var_t1_dn13_slot: &mut f64,
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
        var_t2_dn13_slot: &mut f64,
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
        var_t3_dn13_slot: &mut f64,
        var_t3_dn2_slot: &mut f64,
        var_t3_dn4_slot: &mut f64,
        var_t3_dn5_slot: &mut f64,
        var_t3_dn6_slot: &mut f64,
        var_t3_dn7_slot: &mut f64,
        var_t3_dn8_slot: &mut f64,
        var_t3_dn9_slot: &mut f64,
        var_t3_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn13_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_dn9_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn10_slot: &mut f64,
        var_t5_dn13_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t5_dn9_slot: &mut f64,
        var_t5_rv_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn13_slot: &mut f64,
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
        var_t7_dn13_slot: &mut f64,
        var_t7_dn2_slot: &mut f64,
        var_t7_dn4_slot: &mut f64,
        var_t7_dn5_slot: &mut f64,
        var_t7_dn6_slot: &mut f64,
        var_t7_dn7_slot: &mut f64,
        var_t7_dn8_slot: &mut f64,
        var_t7_dn9_slot: &mut f64,
        var_t7_rv_slot: &mut f64,
        var_t8_slot: &mut f64,
        var_t8_dn0_slot: &mut f64,
        var_t8_dn10_slot: &mut f64,
        var_t8_dn13_slot: &mut f64,
        var_t8_dn2_slot: &mut f64,
        var_t8_dn4_slot: &mut f64,
        var_t8_dn5_slot: &mut f64,
        var_t8_dn6_slot: &mut f64,
        var_t8_dn7_slot: &mut f64,
        var_t8_dn8_slot: &mut f64,
        var_t8_dn9_slot: &mut f64,
        var_t8_rv_slot: &mut f64,
        var_vgssti_slot: &mut f64,
        var_vgssti_dn0_slot: &mut f64,
        var_vgssti_dn10_slot: &mut f64,
        var_vgssti_dn13_slot: &mut f64,
        var_vgssti_dn2_slot: &mut f64,
        var_vgssti_dn4_slot: &mut f64,
        var_vgssti_dn5_slot: &mut f64,
        var_vgssti_dn6_slot: &mut f64,
        var_vgssti_dn7_slot: &mut f64,
        var_vgssti_dn8_slot: &mut f64,
        var_vgssti_dn9_slot: &mut f64,
        var_vgssti_rv_slot: &mut f64,
    ) {
        let mut var_costi3: f64 = *var_costi3_slot;
        let mut var_costi3_dn0: f64 = *var_costi3_dn0_slot;
        let mut var_costi3_dn10: f64 = *var_costi3_dn10_slot;
        let mut var_costi3_dn13: f64 = *var_costi3_dn13_slot;
        let mut var_costi3_dn2: f64 = *var_costi3_dn2_slot;
        let mut var_costi3_dn4: f64 = *var_costi3_dn4_slot;
        let mut var_costi3_dn5: f64 = *var_costi3_dn5_slot;
        let mut var_costi3_dn6: f64 = *var_costi3_dn6_slot;
        let mut var_costi3_dn7: f64 = *var_costi3_dn7_slot;
        let mut var_costi3_dn8: f64 = *var_costi3_dn8_slot;
        let mut var_costi3_dn9: f64 = *var_costi3_dn9_slot;
        let mut var_costi3_rv: f64 = *var_costi3_rv_slot;
        let mut var_costi4: f64 = *var_costi4_slot;
        let mut var_costi4_dn0: f64 = *var_costi4_dn0_slot;
        let mut var_costi4_dn10: f64 = *var_costi4_dn10_slot;
        let mut var_costi4_dn13: f64 = *var_costi4_dn13_slot;
        let mut var_costi4_dn2: f64 = *var_costi4_dn2_slot;
        let mut var_costi4_dn4: f64 = *var_costi4_dn4_slot;
        let mut var_costi4_dn5: f64 = *var_costi4_dn5_slot;
        let mut var_costi4_dn6: f64 = *var_costi4_dn6_slot;
        let mut var_costi4_dn7: f64 = *var_costi4_dn7_slot;
        let mut var_costi4_dn8: f64 = *var_costi4_dn8_slot;
        let mut var_costi4_dn9: f64 = *var_costi4_dn9_slot;
        let mut var_costi4_rv: f64 = *var_costi4_rv_slot;
        let mut var_costi5: f64 = *var_costi5_slot;
        let mut var_costi5_dn0: f64 = *var_costi5_dn0_slot;
        let mut var_costi5_dn10: f64 = *var_costi5_dn10_slot;
        let mut var_costi5_dn13: f64 = *var_costi5_dn13_slot;
        let mut var_costi5_dn2: f64 = *var_costi5_dn2_slot;
        let mut var_costi5_dn4: f64 = *var_costi5_dn4_slot;
        let mut var_costi5_dn5: f64 = *var_costi5_dn5_slot;
        let mut var_costi5_dn6: f64 = *var_costi5_dn6_slot;
        let mut var_costi5_dn7: f64 = *var_costi5_dn7_slot;
        let mut var_costi5_dn8: f64 = *var_costi5_dn8_slot;
        let mut var_costi5_dn9: f64 = *var_costi5_dn9_slot;
        let mut var_costi5_rv: f64 = *var_costi5_rv_slot;
        let mut var_dvth0: f64 = *var_dvth0_slot;
        let mut var_dvth0_dn0: f64 = *var_dvth0_dn0_slot;
        let mut var_dvth0_dn10: f64 = *var_dvth0_dn10_slot;
        let mut var_dvth0_dn13: f64 = *var_dvth0_dn13_slot;
        let mut var_dvth0_dn2: f64 = *var_dvth0_dn2_slot;
        let mut var_dvth0_dn4: f64 = *var_dvth0_dn4_slot;
        let mut var_dvth0_dn5: f64 = *var_dvth0_dn5_slot;
        let mut var_dvth0_dn6: f64 = *var_dvth0_dn6_slot;
        let mut var_dvth0_dn7: f64 = *var_dvth0_dn7_slot;
        let mut var_dvth0_dn8: f64 = *var_dvth0_dn8_slot;
        let mut var_dvth0_dn9: f64 = *var_dvth0_dn9_slot;
        let mut var_dvth0_rv: f64 = *var_dvth0_rv_slot;
        let mut var_dvthscsti: f64 = *var_dvthscsti_slot;
        let mut var_dvthscsti_dn0: f64 = *var_dvthscsti_dn0_slot;
        let mut var_dvthscsti_dn10: f64 = *var_dvthscsti_dn10_slot;
        let mut var_dvthscsti_dn13: f64 = *var_dvthscsti_dn13_slot;
        let mut var_dvthscsti_dn2: f64 = *var_dvthscsti_dn2_slot;
        let mut var_dvthscsti_dn4: f64 = *var_dvthscsti_dn4_slot;
        let mut var_dvthscsti_dn5: f64 = *var_dvthscsti_dn5_slot;
        let mut var_dvthscsti_dn6: f64 = *var_dvthscsti_dn6_slot;
        let mut var_dvthscsti_dn7: f64 = *var_dvthscsti_dn7_slot;
        let mut var_dvthscsti_dn8: f64 = *var_dvthscsti_dn8_slot;
        let mut var_dvthscsti_dn9: f64 = *var_dvthscsti_dn9_slot;
        let mut var_dvthscsti_rv: f64 = *var_dvthscsti_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn13: f64 = *var_t0_dn13_slot;
        let mut var_t0_dn2: f64 = *var_t0_dn2_slot;
        let mut var_t0_dn4: f64 = *var_t0_dn4_slot;
        let mut var_t0_dn5: f64 = *var_t0_dn5_slot;
        let mut var_t0_dn6: f64 = *var_t0_dn6_slot;
        let mut var_t0_dn7: f64 = *var_t0_dn7_slot;
        let mut var_t0_dn8: f64 = *var_t0_dn8_slot;
        let mut var_t0_dn9: f64 = *var_t0_dn9_slot;
        let mut var_t0_rv: f64 = *var_t0_rv_slot;
        let mut var_t1: f64 = *var_t1_slot;
        let mut var_t10: f64 = *var_t10_slot;
        let mut var_t10_dn0: f64 = *var_t10_dn0_slot;
        let mut var_t10_dn10: f64 = *var_t10_dn10_slot;
        let mut var_t10_dn13: f64 = *var_t10_dn13_slot;
        let mut var_t10_dn2: f64 = *var_t10_dn2_slot;
        let mut var_t10_dn4: f64 = *var_t10_dn4_slot;
        let mut var_t10_dn5: f64 = *var_t10_dn5_slot;
        let mut var_t10_dn6: f64 = *var_t10_dn6_slot;
        let mut var_t10_dn7: f64 = *var_t10_dn7_slot;
        let mut var_t10_dn8: f64 = *var_t10_dn8_slot;
        let mut var_t10_dn9: f64 = *var_t10_dn9_slot;
        let mut var_t10_rv: f64 = *var_t10_rv_slot;
        let mut var_t11: f64 = *var_t11_slot;
        let mut var_t11_dn0: f64 = *var_t11_dn0_slot;
        let mut var_t11_dn10: f64 = *var_t11_dn10_slot;
        let mut var_t11_dn13: f64 = *var_t11_dn13_slot;
        let mut var_t11_dn2: f64 = *var_t11_dn2_slot;
        let mut var_t11_dn4: f64 = *var_t11_dn4_slot;
        let mut var_t11_dn5: f64 = *var_t11_dn5_slot;
        let mut var_t11_dn6: f64 = *var_t11_dn6_slot;
        let mut var_t11_dn7: f64 = *var_t11_dn7_slot;
        let mut var_t11_dn8: f64 = *var_t11_dn8_slot;
        let mut var_t11_dn9: f64 = *var_t11_dn9_slot;
        let mut var_t11_rv: f64 = *var_t11_rv_slot;
        let mut var_t1_dn0: f64 = *var_t1_dn0_slot;
        let mut var_t1_dn10: f64 = *var_t1_dn10_slot;
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
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
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
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
        let mut var_t3_dn13: f64 = *var_t3_dn13_slot;
        let mut var_t3_dn2: f64 = *var_t3_dn2_slot;
        let mut var_t3_dn4: f64 = *var_t3_dn4_slot;
        let mut var_t3_dn5: f64 = *var_t3_dn5_slot;
        let mut var_t3_dn6: f64 = *var_t3_dn6_slot;
        let mut var_t3_dn7: f64 = *var_t3_dn7_slot;
        let mut var_t3_dn8: f64 = *var_t3_dn8_slot;
        let mut var_t3_dn9: f64 = *var_t3_dn9_slot;
        let mut var_t3_rv: f64 = *var_t3_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn13: f64 = *var_t4_dn13_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_dn9: f64 = *var_t4_dn9_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn10: f64 = *var_t5_dn10_slot;
        let mut var_t5_dn13: f64 = *var_t5_dn13_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t5_dn9: f64 = *var_t5_dn9_slot;
        let mut var_t5_rv: f64 = *var_t5_rv_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn13: f64 = *var_t6_dn13_slot;
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
        let mut var_t7_dn13: f64 = *var_t7_dn13_slot;
        let mut var_t7_dn2: f64 = *var_t7_dn2_slot;
        let mut var_t7_dn4: f64 = *var_t7_dn4_slot;
        let mut var_t7_dn5: f64 = *var_t7_dn5_slot;
        let mut var_t7_dn6: f64 = *var_t7_dn6_slot;
        let mut var_t7_dn7: f64 = *var_t7_dn7_slot;
        let mut var_t7_dn8: f64 = *var_t7_dn8_slot;
        let mut var_t7_dn9: f64 = *var_t7_dn9_slot;
        let mut var_t7_rv: f64 = *var_t7_rv_slot;
        let mut var_t8: f64 = *var_t8_slot;
        let mut var_t8_dn0: f64 = *var_t8_dn0_slot;
        let mut var_t8_dn10: f64 = *var_t8_dn10_slot;
        let mut var_t8_dn13: f64 = *var_t8_dn13_slot;
        let mut var_t8_dn2: f64 = *var_t8_dn2_slot;
        let mut var_t8_dn4: f64 = *var_t8_dn4_slot;
        let mut var_t8_dn5: f64 = *var_t8_dn5_slot;
        let mut var_t8_dn6: f64 = *var_t8_dn6_slot;
        let mut var_t8_dn7: f64 = *var_t8_dn7_slot;
        let mut var_t8_dn8: f64 = *var_t8_dn8_slot;
        let mut var_t8_dn9: f64 = *var_t8_dn9_slot;
        let mut var_t8_rv: f64 = *var_t8_rv_slot;
        let mut var_vgssti: f64 = *var_vgssti_slot;
        let mut var_vgssti_dn0: f64 = *var_vgssti_dn0_slot;
        let mut var_vgssti_dn10: f64 = *var_vgssti_dn10_slot;
        let mut var_vgssti_dn13: f64 = *var_vgssti_dn13_slot;
        let mut var_vgssti_dn2: f64 = *var_vgssti_dn2_slot;
        let mut var_vgssti_dn4: f64 = *var_vgssti_dn4_slot;
        let mut var_vgssti_dn5: f64 = *var_vgssti_dn5_slot;
        let mut var_vgssti_dn6: f64 = *var_vgssti_dn6_slot;
        let mut var_vgssti_dn7: f64 = *var_vgssti_dn7_slot;
        let mut var_vgssti_dn8: f64 = *var_vgssti_dn8_slot;
        let mut var_vgssti_dn9: f64 = *var_vgssti_dn9_slot;
        let mut var_vgssti_rv: f64 = *var_vgssti_rv_slot;

        let (assign63070_e97763, assign63070_e97763_d_n0, assign63070_e97763_d_n2, assign63070_e97763_d_n4, assign63070_e97763_d_n5, assign63070_e97763_d_n6, assign63070_e97763_d_n7, assign63070_e97763_d_n8, assign63070_e97763_d_n9, assign63070_e97763_d_n10, assign63070_e97763_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63070_e97761: f64 = (var_lgatesm - p.p139);
        (assign63070_e97761, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn13,)
    }
};
        var_t3 = assign63070_e97763;
        var_t3_dn0 = assign63070_e97763_d_n0;
        var_t3_dn2 = assign63070_e97763_d_n2;
        var_t3_dn4 = assign63070_e97763_d_n4;
        var_t3_dn5 = assign63070_e97763_d_n5;
        var_t3_dn6 = assign63070_e97763_d_n6;
        var_t3_dn7 = assign63070_e97763_d_n7;
        var_t3_dn8 = assign63070_e97763_d_n8;
        var_t3_dn9 = assign63070_e97763_d_n9;
        var_t3_dn10 = assign63070_e97763_d_n10;
        var_t3_dn13 = assign63070_e97763_d_n13;
        var_t3_rv = 0.0;

        let (assign63080_e97776, assign63080_e97776_d_n0, assign63080_e97776_d_n2, assign63080_e97776_d_n4, assign63080_e97776_d_n5, assign63080_e97776_d_n6, assign63080_e97776_d_n7, assign63080_e97776_d_n8, assign63080_e97776_d_n9, assign63080_e97776_d_n10, assign63080_e97776_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63080_e97773: f64 = (var_t3 * var_t3);
        let assign63080_e97774: f64 = (1.0 / assign63080_e97773);
        (assign63080_e97774, (-(((var_t3_dn0 * var_t3) + (var_t3 * var_t3_dn0)) / (assign63080_e97773 * assign63080_e97773))), (-(((var_t3_dn2 * var_t3) + (var_t3 * var_t3_dn2)) / (assign63080_e97773 * assign63080_e97773))), (-(((var_t3_dn4 * var_t3) + (var_t3 * var_t3_dn4)) / (assign63080_e97773 * assign63080_e97773))), (-(((var_t3_dn5 * var_t3) + (var_t3 * var_t3_dn5)) / (assign63080_e97773 * assign63080_e97773))), (-(((var_t3_dn6 * var_t3) + (var_t3 * var_t3_dn6)) / (assign63080_e97773 * assign63080_e97773))), (-(((var_t3_dn7 * var_t3) + (var_t3 * var_t3_dn7)) / (assign63080_e97773 * assign63080_e97773))), (-(((var_t3_dn8 * var_t3) + (var_t3 * var_t3_dn8)) / (assign63080_e97773 * assign63080_e97773))), (-(((var_t3_dn9 * var_t3) + (var_t3 * var_t3_dn9)) / (assign63080_e97773 * assign63080_e97773))), (-(((var_t3_dn10 * var_t3) + (var_t3 * var_t3_dn10)) / (assign63080_e97773 * assign63080_e97773))), (-(((var_t3_dn13 * var_t3) + (var_t3 * var_t3_dn13)) / (assign63080_e97773 * assign63080_e97773))),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn13,)
    }
};
        var_t4 = assign63080_e97776;
        var_t4_dn0 = assign63080_e97776_d_n0;
        var_t4_dn2 = assign63080_e97776_d_n2;
        var_t4_dn4 = assign63080_e97776_d_n4;
        var_t4_dn5 = assign63080_e97776_d_n5;
        var_t4_dn6 = assign63080_e97776_d_n6;
        var_t4_dn7 = assign63080_e97776_d_n7;
        var_t4_dn8 = assign63080_e97776_d_n8;
        var_t4_dn9 = assign63080_e97776_d_n9;
        var_t4_dn10 = assign63080_e97776_d_n10;
        var_t4_dn13 = assign63080_e97776_d_n13;
        var_t4_rv = 0.0;

        let (assign63090_e97795, assign63090_e97795_d_n0, assign63090_e97795_d_n2, assign63090_e97795_d_n4, assign63090_e97795_d_n5, assign63090_e97795_d_n6, assign63090_e97795_d_n7, assign63090_e97795_d_n8, assign63090_e97795_d_n9, assign63090_e97795_d_n10, assign63090_e97795_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63090_e97786: f64 = (p.p137 - var_pb20b);
        let assign63090_e97787: f64 = (2.0 * assign63090_e97786);
        let assign63090_e97789: f64 = (assign63090_e97787 * var_t1);
        let assign63090_e97791: f64 = (assign63090_e97789 * var_t2);
        let assign63090_e97793: f64 = (assign63090_e97791 * var_t4);
        (assign63090_e97793, (((((((2.0 * (-var_pb20b_dn0)) * var_t1) + (assign63090_e97787 * var_t1_dn0)) * var_t2) + (assign63090_e97789 * var_t2_dn0)) * var_t4) + (assign63090_e97791 * var_t4_dn0)), (((((((2.0 * (-var_pb20b_dn2)) * var_t1) + (assign63090_e97787 * var_t1_dn2)) * var_t2) + (assign63090_e97789 * var_t2_dn2)) * var_t4) + (assign63090_e97791 * var_t4_dn2)), (((((((2.0 * (-var_pb20b_dn4)) * var_t1) + (assign63090_e97787 * var_t1_dn4)) * var_t2) + (assign63090_e97789 * var_t2_dn4)) * var_t4) + (assign63090_e97791 * var_t4_dn4)), (((((((2.0 * (-var_pb20b_dn5)) * var_t1) + (assign63090_e97787 * var_t1_dn5)) * var_t2) + (assign63090_e97789 * var_t2_dn5)) * var_t4) + (assign63090_e97791 * var_t4_dn5)), (((((((2.0 * (-var_pb20b_dn6)) * var_t1) + (assign63090_e97787 * var_t1_dn6)) * var_t2) + (assign63090_e97789 * var_t2_dn6)) * var_t4) + (assign63090_e97791 * var_t4_dn6)), (((((((2.0 * (-var_pb20b_dn7)) * var_t1) + (assign63090_e97787 * var_t1_dn7)) * var_t2) + (assign63090_e97789 * var_t2_dn7)) * var_t4) + (assign63090_e97791 * var_t4_dn7)), (((((((2.0 * (-var_pb20b_dn8)) * var_t1) + (assign63090_e97787 * var_t1_dn8)) * var_t2) + (assign63090_e97789 * var_t2_dn8)) * var_t4) + (assign63090_e97791 * var_t4_dn8)), (((((((2.0 * (-var_pb20b_dn9)) * var_t1) + (assign63090_e97787 * var_t1_dn9)) * var_t2) + (assign63090_e97789 * var_t2_dn9)) * var_t4) + (assign63090_e97791 * var_t4_dn9)), (((((((2.0 * (-var_pb20b_dn10)) * var_t1) + (assign63090_e97787 * var_t1_dn10)) * var_t2) + (assign63090_e97789 * var_t2_dn10)) * var_t4) + (assign63090_e97791 * var_t4_dn10)), (((((((2.0 * (-var_pb20b_dn13)) * var_t1) + (assign63090_e97787 * var_t1_dn13)) * var_t2) + (assign63090_e97789 * var_t2_dn13)) * var_t4) + (assign63090_e97791 * var_t4_dn13)),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn13,)
    }
};
        var_t5 = assign63090_e97795;
        var_t5_dn0 = assign63090_e97795_d_n0;
        var_t5_dn2 = assign63090_e97795_d_n2;
        var_t5_dn4 = assign63090_e97795_d_n4;
        var_t5_dn5 = assign63090_e97795_d_n5;
        var_t5_dn6 = assign63090_e97795_d_n6;
        var_t5_dn7 = assign63090_e97795_d_n7;
        var_t5_dn8 = assign63090_e97795_d_n8;
        var_t5_dn9 = assign63090_e97795_d_n9;
        var_t5_dn10 = assign63090_e97795_d_n10;
        var_t5_dn13 = assign63090_e97795_d_n13;
        var_t5_rv = 0.0;

        let (assign63100_e97806, assign63100_e97806_d_n0, assign63100_e97806_d_n2, assign63100_e97806_d_n4, assign63100_e97806_d_n5, assign63100_e97806_d_n6, assign63100_e97806_d_n7, assign63100_e97806_d_n8, assign63100_e97806_d_n9, assign63100_e97806_d_n10, assign63100_e97806_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63100_e97804: f64 = (var_t5 * var_sqrt_pbsum);
        (assign63100_e97804, ((var_t5_dn0 * var_sqrt_pbsum) + (var_t5 * var_sqrt_pbsum_dn0)), ((var_t5_dn2 * var_sqrt_pbsum) + (var_t5 * var_sqrt_pbsum_dn2)), ((var_t5_dn4 * var_sqrt_pbsum) + (var_t5 * var_sqrt_pbsum_dn4)), ((var_t5_dn5 * var_sqrt_pbsum) + (var_t5 * var_sqrt_pbsum_dn5)), ((var_t5_dn6 * var_sqrt_pbsum) + (var_t5 * var_sqrt_pbsum_dn6)), ((var_t5_dn7 * var_sqrt_pbsum) + (var_t5 * var_sqrt_pbsum_dn7)), ((var_t5_dn8 * var_sqrt_pbsum) + (var_t5 * var_sqrt_pbsum_dn8)), ((var_t5_dn9 * var_sqrt_pbsum) + (var_t5 * var_sqrt_pbsum_dn9)), ((var_t5_dn10 * var_sqrt_pbsum) + (var_t5 * var_sqrt_pbsum_dn10)), ((var_t5_dn13 * var_sqrt_pbsum) + (var_t5 * var_sqrt_pbsum_dn13)),)
    } else {
        (var_dvth0, var_dvth0_dn0, var_dvth0_dn2, var_dvth0_dn4, var_dvth0_dn5, var_dvth0_dn6, var_dvth0_dn7, var_dvth0_dn8, var_dvth0_dn9, var_dvth0_dn10, var_dvth0_dn13,)
    }
};
        var_dvth0 = assign63100_e97806;
        var_dvth0_dn0 = assign63100_e97806_d_n0;
        var_dvth0_dn2 = assign63100_e97806_d_n2;
        var_dvth0_dn4 = assign63100_e97806_d_n4;
        var_dvth0_dn5 = assign63100_e97806_d_n5;
        var_dvth0_dn6 = assign63100_e97806_d_n6;
        var_dvth0_dn7 = assign63100_e97806_d_n7;
        var_dvth0_dn8 = assign63100_e97806_d_n8;
        var_dvth0_dn9 = assign63100_e97806_d_n9;
        var_dvth0_dn10 = assign63100_e97806_d_n10;
        var_dvth0_dn13 = assign63100_e97806_d_n13;
        var_dvth0_rv = 0.0;

        let (assign63110_e97819, assign63110_e97819_d_n0, assign63110_e97819_d_n2, assign63110_e97819_d_n4, assign63110_e97819_d_n5, assign63110_e97819_d_n6, assign63110_e97819_d_n7, assign63110_e97819_d_n8, assign63110_e97819_d_n9, assign63110_e97819_d_n10, assign63110_e97819_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63110_e97815: f64 = (var_t5 * 0.5);
        let assign63110_e97817: f64 = (assign63110_e97815 / var_sqrt_pbsum);
        (assign63110_e97817, ((((var_t5_dn0 * 0.5) * var_sqrt_pbsum) - (assign63110_e97815 * var_sqrt_pbsum_dn0)) / (var_sqrt_pbsum * var_sqrt_pbsum)), ((((var_t5_dn2 * 0.5) * var_sqrt_pbsum) - (assign63110_e97815 * var_sqrt_pbsum_dn2)) / (var_sqrt_pbsum * var_sqrt_pbsum)), ((((var_t5_dn4 * 0.5) * var_sqrt_pbsum) - (assign63110_e97815 * var_sqrt_pbsum_dn4)) / (var_sqrt_pbsum * var_sqrt_pbsum)), ((((var_t5_dn5 * 0.5) * var_sqrt_pbsum) - (assign63110_e97815 * var_sqrt_pbsum_dn5)) / (var_sqrt_pbsum * var_sqrt_pbsum)), ((((var_t5_dn6 * 0.5) * var_sqrt_pbsum) - (assign63110_e97815 * var_sqrt_pbsum_dn6)) / (var_sqrt_pbsum * var_sqrt_pbsum)), ((((var_t5_dn7 * 0.5) * var_sqrt_pbsum) - (assign63110_e97815 * var_sqrt_pbsum_dn7)) / (var_sqrt_pbsum * var_sqrt_pbsum)), ((((var_t5_dn8 * 0.5) * var_sqrt_pbsum) - (assign63110_e97815 * var_sqrt_pbsum_dn8)) / (var_sqrt_pbsum * var_sqrt_pbsum)), ((((var_t5_dn9 * 0.5) * var_sqrt_pbsum) - (assign63110_e97815 * var_sqrt_pbsum_dn9)) / (var_sqrt_pbsum * var_sqrt_pbsum)), ((((var_t5_dn10 * 0.5) * var_sqrt_pbsum) - (assign63110_e97815 * var_sqrt_pbsum_dn10)) / (var_sqrt_pbsum * var_sqrt_pbsum)), ((((var_t5_dn13 * 0.5) * var_sqrt_pbsum) - (assign63110_e97815 * var_sqrt_pbsum_dn13)) / (var_sqrt_pbsum * var_sqrt_pbsum)),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn13,)
    }
};
        var_t6 = assign63110_e97819;
        var_t6_dn0 = assign63110_e97819_d_n0;
        var_t6_dn2 = assign63110_e97819_d_n2;
        var_t6_dn4 = assign63110_e97819_d_n4;
        var_t6_dn5 = assign63110_e97819_d_n5;
        var_t6_dn6 = assign63110_e97819_d_n6;
        var_t6_dn7 = assign63110_e97819_d_n7;
        var_t6_dn8 = assign63110_e97819_d_n8;
        var_t6_dn9 = assign63110_e97819_d_n9;
        var_t6_dn10 = assign63110_e97819_d_n10;
        var_t6_dn13 = assign63110_e97819_d_n13;
        var_t6_rv = 0.0;

        let (assign63120_e97840, assign63120_e97840_d_n0, assign63120_e97840_d_n2, assign63120_e97840_d_n4, assign63120_e97840_d_n5, assign63120_e97840_d_n6, assign63120_e97840_d_n7, assign63120_e97840_d_n8, assign63120_e97840_d_n9, assign63120_e97840_d_n10, assign63120_e97840_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63120_e97829: f64 = (p.p137 - var_pb20b);
        let assign63120_e97830: f64 = (2.0 * assign63120_e97829);
        let assign63120_e97832: f64 = (assign63120_e97830 * 1.034943e-10);
        let assign63120_e97834: f64 = (assign63120_e97832 * var_t2);
        let assign63120_e97836: f64 = (assign63120_e97834 * var_t4);
        let assign63120_e97838: f64 = (assign63120_e97836 * var_sqrt_pbsum);
        (assign63120_e97838, ((((((((2.0 * (-var_pb20b_dn0)) * 1.034943e-10) * var_t2) + (assign63120_e97832 * var_t2_dn0)) * var_t4) + (assign63120_e97834 * var_t4_dn0)) * var_sqrt_pbsum) + (assign63120_e97836 * var_sqrt_pbsum_dn0)), ((((((((2.0 * (-var_pb20b_dn2)) * 1.034943e-10) * var_t2) + (assign63120_e97832 * var_t2_dn2)) * var_t4) + (assign63120_e97834 * var_t4_dn2)) * var_sqrt_pbsum) + (assign63120_e97836 * var_sqrt_pbsum_dn2)), ((((((((2.0 * (-var_pb20b_dn4)) * 1.034943e-10) * var_t2) + (assign63120_e97832 * var_t2_dn4)) * var_t4) + (assign63120_e97834 * var_t4_dn4)) * var_sqrt_pbsum) + (assign63120_e97836 * var_sqrt_pbsum_dn4)), ((((((((2.0 * (-var_pb20b_dn5)) * 1.034943e-10) * var_t2) + (assign63120_e97832 * var_t2_dn5)) * var_t4) + (assign63120_e97834 * var_t4_dn5)) * var_sqrt_pbsum) + (assign63120_e97836 * var_sqrt_pbsum_dn5)), ((((((((2.0 * (-var_pb20b_dn6)) * 1.034943e-10) * var_t2) + (assign63120_e97832 * var_t2_dn6)) * var_t4) + (assign63120_e97834 * var_t4_dn6)) * var_sqrt_pbsum) + (assign63120_e97836 * var_sqrt_pbsum_dn6)), ((((((((2.0 * (-var_pb20b_dn7)) * 1.034943e-10) * var_t2) + (assign63120_e97832 * var_t2_dn7)) * var_t4) + (assign63120_e97834 * var_t4_dn7)) * var_sqrt_pbsum) + (assign63120_e97836 * var_sqrt_pbsum_dn7)), ((((((((2.0 * (-var_pb20b_dn8)) * 1.034943e-10) * var_t2) + (assign63120_e97832 * var_t2_dn8)) * var_t4) + (assign63120_e97834 * var_t4_dn8)) * var_sqrt_pbsum) + (assign63120_e97836 * var_sqrt_pbsum_dn8)), ((((((((2.0 * (-var_pb20b_dn9)) * 1.034943e-10) * var_t2) + (assign63120_e97832 * var_t2_dn9)) * var_t4) + (assign63120_e97834 * var_t4_dn9)) * var_sqrt_pbsum) + (assign63120_e97836 * var_sqrt_pbsum_dn9)), ((((((((2.0 * (-var_pb20b_dn10)) * 1.034943e-10) * var_t2) + (assign63120_e97832 * var_t2_dn10)) * var_t4) + (assign63120_e97834 * var_t4_dn10)) * var_sqrt_pbsum) + (assign63120_e97836 * var_sqrt_pbsum_dn10)), ((((((((2.0 * (-var_pb20b_dn13)) * 1.034943e-10) * var_t2) + (assign63120_e97832 * var_t2_dn13)) * var_t4) + (assign63120_e97834 * var_t4_dn13)) * var_sqrt_pbsum) + (assign63120_e97836 * var_sqrt_pbsum_dn13)),)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn7, var_t7_dn8, var_t7_dn9, var_t7_dn10, var_t7_dn13,)
    }
};
        var_t7 = assign63120_e97840;
        var_t7_dn0 = assign63120_e97840_d_n0;
        var_t7_dn2 = assign63120_e97840_d_n2;
        var_t7_dn4 = assign63120_e97840_d_n4;
        var_t7_dn5 = assign63120_e97840_d_n5;
        var_t7_dn6 = assign63120_e97840_d_n6;
        var_t7_dn7 = assign63120_e97840_d_n7;
        var_t7_dn8 = assign63120_e97840_d_n8;
        var_t7_dn9 = assign63120_e97840_d_n9;
        var_t7_dn10 = assign63120_e97840_d_n10;
        var_t7_dn13 = assign63120_e97840_d_n13;
        var_t7_rv = 0.0;

        let (assign63130_e97858, assign63130_e97858_d_n0, assign63130_e97858_d_n2, assign63130_e97858_d_n4, assign63130_e97858_d_n5, assign63130_e97858_d_n6, assign63130_e97858_d_n7, assign63130_e97858_d_n8, assign63130_e97858_d_n9, assign63130_e97858_d_n10, assign63130_e97858_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63130_e97848: f64 = (-2.0);
        let assign63130_e97850: f64 = (assign63130_e97848 * var_t1);
        let assign63130_e97852: f64 = (assign63130_e97850 * var_t2);
        let assign63130_e97854: f64 = (assign63130_e97852 * var_t4);
        let assign63130_e97856: f64 = (assign63130_e97854 * var_sqrt_pbsum);
        (assign63130_e97856, (((((((assign63130_e97848 * var_t1_dn0) * var_t2) + (assign63130_e97850 * var_t2_dn0)) * var_t4) + (assign63130_e97852 * var_t4_dn0)) * var_sqrt_pbsum) + (assign63130_e97854 * var_sqrt_pbsum_dn0)), (((((((assign63130_e97848 * var_t1_dn2) * var_t2) + (assign63130_e97850 * var_t2_dn2)) * var_t4) + (assign63130_e97852 * var_t4_dn2)) * var_sqrt_pbsum) + (assign63130_e97854 * var_sqrt_pbsum_dn2)), (((((((assign63130_e97848 * var_t1_dn4) * var_t2) + (assign63130_e97850 * var_t2_dn4)) * var_t4) + (assign63130_e97852 * var_t4_dn4)) * var_sqrt_pbsum) + (assign63130_e97854 * var_sqrt_pbsum_dn4)), (((((((assign63130_e97848 * var_t1_dn5) * var_t2) + (assign63130_e97850 * var_t2_dn5)) * var_t4) + (assign63130_e97852 * var_t4_dn5)) * var_sqrt_pbsum) + (assign63130_e97854 * var_sqrt_pbsum_dn5)), (((((((assign63130_e97848 * var_t1_dn6) * var_t2) + (assign63130_e97850 * var_t2_dn6)) * var_t4) + (assign63130_e97852 * var_t4_dn6)) * var_sqrt_pbsum) + (assign63130_e97854 * var_sqrt_pbsum_dn6)), (((((((assign63130_e97848 * var_t1_dn7) * var_t2) + (assign63130_e97850 * var_t2_dn7)) * var_t4) + (assign63130_e97852 * var_t4_dn7)) * var_sqrt_pbsum) + (assign63130_e97854 * var_sqrt_pbsum_dn7)), (((((((assign63130_e97848 * var_t1_dn8) * var_t2) + (assign63130_e97850 * var_t2_dn8)) * var_t4) + (assign63130_e97852 * var_t4_dn8)) * var_sqrt_pbsum) + (assign63130_e97854 * var_sqrt_pbsum_dn8)), (((((((assign63130_e97848 * var_t1_dn9) * var_t2) + (assign63130_e97850 * var_t2_dn9)) * var_t4) + (assign63130_e97852 * var_t4_dn9)) * var_sqrt_pbsum) + (assign63130_e97854 * var_sqrt_pbsum_dn9)), (((((((assign63130_e97848 * var_t1_dn10) * var_t2) + (assign63130_e97850 * var_t2_dn10)) * var_t4) + (assign63130_e97852 * var_t4_dn10)) * var_sqrt_pbsum) + (assign63130_e97854 * var_sqrt_pbsum_dn10)), (((((((assign63130_e97848 * var_t1_dn13) * var_t2) + (assign63130_e97850 * var_t2_dn13)) * var_t4) + (assign63130_e97852 * var_t4_dn13)) * var_sqrt_pbsum) + (assign63130_e97854 * var_sqrt_pbsum_dn13)),)
    } else {
        (var_t8, var_t8_dn0, var_t8_dn2, var_t8_dn4, var_t8_dn5, var_t8_dn6, var_t8_dn7, var_t8_dn8, var_t8_dn9, var_t8_dn10, var_t8_dn13,)
    }
};
        var_t8 = assign63130_e97858;
        var_t8_dn0 = assign63130_e97858_d_n0;
        var_t8_dn2 = assign63130_e97858_d_n2;
        var_t8_dn4 = assign63130_e97858_d_n4;
        var_t8_dn5 = assign63130_e97858_d_n5;
        var_t8_dn6 = assign63130_e97858_d_n6;
        var_t8_dn7 = assign63130_e97858_d_n7;
        var_t8_dn8 = assign63130_e97858_d_n8;
        var_t8_dn9 = assign63130_e97858_d_n9;
        var_t8_dn10 = assign63130_e97858_d_n10;
        var_t8_dn13 = assign63130_e97858_d_n13;
        var_t8_rv = 0.0;

        let (assign63140_e97867, assign63140_e97867_d_n0, assign63140_e97867_d_n2, assign63140_e97867_d_n4, assign63140_e97867_d_n5, assign63140_e97867_d_n6, assign63140_e97867_d_n7, assign63140_e97867_d_n8, assign63140_e97867_d_n9, assign63140_e97867_d_n10, assign63140_e97867_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        (var_uc_scsti1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn13,)
    }
};
        var_t4 = assign63140_e97867;
        var_t4_dn0 = assign63140_e97867_d_n0;
        var_t4_dn2 = assign63140_e97867_d_n2;
        var_t4_dn4 = assign63140_e97867_d_n4;
        var_t4_dn5 = assign63140_e97867_d_n5;
        var_t4_dn6 = assign63140_e97867_d_n6;
        var_t4_dn7 = assign63140_e97867_d_n7;
        var_t4_dn8 = assign63140_e97867_d_n8;
        var_t4_dn9 = assign63140_e97867_d_n9;
        var_t4_dn10 = assign63140_e97867_d_n10;
        var_t4_dn13 = assign63140_e97867_d_n13;
        var_t4_rv = 0.0;

        let (assign63150_e97876, assign63150_e97876_d_n0, assign63150_e97876_d_n2, assign63150_e97876_d_n4, assign63150_e97876_d_n5, assign63150_e97876_d_n6, assign63150_e97876_d_n7, assign63150_e97876_d_n8, assign63150_e97876_d_n9, assign63150_e97876_d_n10, assign63150_e97876_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        (var_uc_scsti2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn13,)
    }
};
        var_t6 = assign63150_e97876;
        var_t6_dn0 = assign63150_e97876_d_n0;
        var_t6_dn2 = assign63150_e97876_d_n2;
        var_t6_dn4 = assign63150_e97876_d_n4;
        var_t6_dn5 = assign63150_e97876_d_n5;
        var_t6_dn6 = assign63150_e97876_d_n6;
        var_t6_dn7 = assign63150_e97876_d_n7;
        var_t6_dn8 = assign63150_e97876_d_n8;
        var_t6_dn9 = assign63150_e97876_d_n9;
        var_t6_dn10 = assign63150_e97876_d_n10;
        var_t6_dn13 = assign63150_e97876_d_n13;
        var_t6_rv = 0.0;

        let (assign63160_e97889, assign63160_e97889_d_n0, assign63160_e97889_d_n2, assign63160_e97889_d_n4, assign63160_e97889_d_n5, assign63160_e97889_d_n6, assign63160_e97889_d_n7, assign63160_e97889_d_n8, assign63160_e97889_d_n9, assign63160_e97889_d_n10, assign63160_e97889_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63160_e97886: f64 = (var_t6 * var_vdsz__blk439);
        let assign63160_e97887: f64 = (var_t4 + assign63160_e97886);
        (assign63160_e97887, (var_t4_dn0 + ((var_t6_dn0 * var_vdsz__blk439) + (var_t6 * var_vdsz__blk439_dn0))), (var_t4_dn2 + ((var_t6_dn2 * var_vdsz__blk439) + (var_t6 * var_vdsz__blk439_dn2))), (var_t4_dn4 + ((var_t6_dn4 * var_vdsz__blk439) + (var_t6 * var_vdsz__blk439_dn4))), (var_t4_dn5 + ((var_t6_dn5 * var_vdsz__blk439) + (var_t6 * var_vdsz__blk439_dn5))), (var_t4_dn6 + ((var_t6_dn6 * var_vdsz__blk439) + (var_t6 * var_vdsz__blk439_dn6))), (var_t4_dn7 + ((var_t6_dn7 * var_vdsz__blk439) + (var_t6 * var_vdsz__blk439_dn7))), (var_t4_dn8 + ((var_t6_dn8 * var_vdsz__blk439) + (var_t6 * var_vdsz__blk439_dn8))), (var_t4_dn9 + ((var_t6_dn9 * var_vdsz__blk439) + (var_t6 * var_vdsz__blk439_dn9))), (var_t4_dn10 + ((var_t6_dn10 * var_vdsz__blk439) + (var_t6 * var_vdsz__blk439_dn10))), (var_t4_dn13 + ((var_t6_dn13 * var_vdsz__blk439) + (var_t6 * var_vdsz__blk439_dn13))),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign63160_e97889;
        var_t1_dn0 = assign63160_e97889_d_n0;
        var_t1_dn2 = assign63160_e97889_d_n2;
        var_t1_dn4 = assign63160_e97889_d_n4;
        var_t1_dn5 = assign63160_e97889_d_n5;
        var_t1_dn6 = assign63160_e97889_d_n6;
        var_t1_dn7 = assign63160_e97889_d_n7;
        var_t1_dn8 = assign63160_e97889_d_n8;
        var_t1_dn9 = assign63160_e97889_d_n9;
        var_t1_dn10 = assign63160_e97889_d_n10;
        var_t1_dn13 = assign63160_e97889_d_n13;
        var_t1_rv = 0.0;

        let (assign63170_e97900, assign63170_e97900_d_n0, assign63170_e97900_d_n2, assign63170_e97900_d_n4, assign63170_e97900_d_n5, assign63170_e97900_d_n6, assign63170_e97900_d_n7, assign63170_e97900_d_n8, assign63170_e97900_d_n9, assign63170_e97900_d_n10, assign63170_e97900_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63170_e97898: f64 = (var_dvth0 * var_t1);
        (assign63170_e97898, ((var_dvth0_dn0 * var_t1) + (var_dvth0 * var_t1_dn0)), ((var_dvth0_dn2 * var_t1) + (var_dvth0 * var_t1_dn2)), ((var_dvth0_dn4 * var_t1) + (var_dvth0 * var_t1_dn4)), ((var_dvth0_dn5 * var_t1) + (var_dvth0 * var_t1_dn5)), ((var_dvth0_dn6 * var_t1) + (var_dvth0 * var_t1_dn6)), ((var_dvth0_dn7 * var_t1) + (var_dvth0 * var_t1_dn7)), ((var_dvth0_dn8 * var_t1) + (var_dvth0 * var_t1_dn8)), ((var_dvth0_dn9 * var_t1) + (var_dvth0 * var_t1_dn9)), ((var_dvth0_dn10 * var_t1) + (var_dvth0 * var_t1_dn10)), ((var_dvth0_dn13 * var_t1) + (var_dvth0 * var_t1_dn13)),)
    } else {
        (var_dvthscsti, var_dvthscsti_dn0, var_dvthscsti_dn2, var_dvthscsti_dn4, var_dvthscsti_dn5, var_dvthscsti_dn6, var_dvthscsti_dn7, var_dvthscsti_dn8, var_dvthscsti_dn9, var_dvthscsti_dn10, var_dvthscsti_dn13,)
    }
};
        var_dvthscsti = assign63170_e97900;
        var_dvthscsti_dn0 = assign63170_e97900_d_n0;
        var_dvthscsti_dn2 = assign63170_e97900_d_n2;
        var_dvthscsti_dn4 = assign63170_e97900_d_n4;
        var_dvthscsti_dn5 = assign63170_e97900_d_n5;
        var_dvthscsti_dn6 = assign63170_e97900_d_n6;
        var_dvthscsti_dn7 = assign63170_e97900_d_n7;
        var_dvthscsti_dn8 = assign63170_e97900_d_n8;
        var_dvthscsti_dn9 = assign63170_e97900_d_n9;
        var_dvthscsti_dn10 = assign63170_e97900_d_n10;
        var_dvthscsti_dn13 = assign63170_e97900_d_n13;
        var_dvthscsti_rv = 0.0;

        let (assign63180_e97913, assign63180_e97913_d_n0, assign63180_e97913_d_n2, assign63180_e97913_d_n4, assign63180_e97913_d_n5, assign63180_e97913_d_n6, assign63180_e97913_d_n7, assign63180_e97913_d_n8, assign63180_e97913_d_n9, assign63180_e97913_d_n10, assign63180_e97913_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63180_e97910: f64 = (p.p213 * var_vds);
        let assign63180_e97911: f64 = (var_uc_vthsti - assign63180_e97910);
        (assign63180_e97911, (-(p.p213 * var_vds_dn0)), (-(p.p213 * var_vds_dn2)), (-(p.p213 * var_vds_dn4)), (-(p.p213 * var_vds_dn5)), (-(p.p213 * var_vds_dn6)), (-(p.p213 * var_vds_dn7)), (-(p.p213 * var_vds_dn8)), (-(p.p213 * var_vds_dn9)), (-(p.p213 * var_vds_dn10)), (-(p.p213 * var_vds_dn13)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign63180_e97913;
        var_t1_dn0 = assign63180_e97913_d_n0;
        var_t1_dn2 = assign63180_e97913_d_n2;
        var_t1_dn4 = assign63180_e97913_d_n4;
        var_t1_dn5 = assign63180_e97913_d_n5;
        var_t1_dn6 = assign63180_e97913_d_n6;
        var_t1_dn7 = assign63180_e97913_d_n7;
        var_t1_dn8 = assign63180_e97913_d_n8;
        var_t1_dn9 = assign63180_e97913_d_n9;
        var_t1_dn10 = assign63180_e97913_d_n10;
        var_t1_dn13 = assign63180_e97913_d_n13;
        var_t1_rv = 0.0;

        let (assign63190_e97928, assign63190_e97928_d_n0, assign63190_e97928_d_n2, assign63190_e97928_d_n4, assign63190_e97928_d_n5, assign63190_e97928_d_n6, assign63190_e97928_d_n7, assign63190_e97928_d_n8, assign63190_e97928_d_n9, assign63190_e97928_d_n10, assign63190_e97928_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63190_e97922: f64 = (var_vgsz__blk440 - var_vfb);
        let assign63190_e97924: f64 = (assign63190_e97922 + var_t1);
        let assign63190_e97926: f64 = (assign63190_e97924 + var_dvthscsti);
        (assign63190_e97926, ((var_vgsz__blk440_dn0 + var_t1_dn0) + var_dvthscsti_dn0), ((var_vgsz__blk440_dn2 + var_t1_dn2) + var_dvthscsti_dn2), ((var_vgsz__blk440_dn4 + var_t1_dn4) + var_dvthscsti_dn4), ((var_vgsz__blk440_dn5 + var_t1_dn5) + var_dvthscsti_dn5), ((var_vgsz__blk440_dn6 + var_t1_dn6) + var_dvthscsti_dn6), ((var_vgsz__blk440_dn7 + var_t1_dn7) + var_dvthscsti_dn7), ((var_vgsz__blk440_dn8 + var_t1_dn8) + var_dvthscsti_dn8), ((var_vgsz__blk440_dn9 + var_t1_dn9) + var_dvthscsti_dn9), ((var_vgsz__blk440_dn10 + var_t1_dn10) + var_dvthscsti_dn10), ((var_vgsz__blk440_dn13 + var_t1_dn13) + var_dvthscsti_dn13),)
    } else {
        (var_vgssti, var_vgssti_dn0, var_vgssti_dn2, var_vgssti_dn4, var_vgssti_dn5, var_vgssti_dn6, var_vgssti_dn7, var_vgssti_dn8, var_vgssti_dn9, var_vgssti_dn10, var_vgssti_dn13,)
    }
};
        var_vgssti = assign63190_e97928;
        var_vgssti_dn0 = assign63190_e97928_d_n0;
        var_vgssti_dn2 = assign63190_e97928_d_n2;
        var_vgssti_dn4 = assign63190_e97928_d_n4;
        var_vgssti_dn5 = assign63190_e97928_d_n5;
        var_vgssti_dn6 = assign63190_e97928_d_n6;
        var_vgssti_dn7 = assign63190_e97928_d_n7;
        var_vgssti_dn8 = assign63190_e97928_d_n8;
        var_vgssti_dn9 = assign63190_e97928_d_n9;
        var_vgssti_dn10 = assign63190_e97928_d_n10;
        var_vgssti_dn13 = assign63190_e97928_d_n13;
        var_vgssti_rv = 0.0;

        let (assign63200_e97941, assign63200_e97941_d_n0, assign63200_e97941_d_n2, assign63200_e97941_d_n4, assign63200_e97941_d_n5, assign63200_e97941_d_n6, assign63200_e97941_d_n7, assign63200_e97941_d_n8, assign63200_e97941_d_n9, assign63200_e97941_d_n10, assign63200_e97941_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63200_e97937: f64 = (var_costi0_p2 * var_cox_inv);
        let assign63200_e97939: f64 = (assign63200_e97937 * var_cox_inv);
        (assign63200_e97939, ((((var_costi0_p2_dn0 * var_cox_inv) + (var_costi0_p2 * var_cox_inv_dn0)) * var_cox_inv) + (assign63200_e97937 * var_cox_inv_dn0)), ((((var_costi0_p2_dn2 * var_cox_inv) + (var_costi0_p2 * var_cox_inv_dn2)) * var_cox_inv) + (assign63200_e97937 * var_cox_inv_dn2)), ((((var_costi0_p2_dn4 * var_cox_inv) + (var_costi0_p2 * var_cox_inv_dn4)) * var_cox_inv) + (assign63200_e97937 * var_cox_inv_dn4)), ((((var_costi0_p2_dn5 * var_cox_inv) + (var_costi0_p2 * var_cox_inv_dn5)) * var_cox_inv) + (assign63200_e97937 * var_cox_inv_dn5)), ((((var_costi0_p2_dn6 * var_cox_inv) + (var_costi0_p2 * var_cox_inv_dn6)) * var_cox_inv) + (assign63200_e97937 * var_cox_inv_dn6)), ((((var_costi0_p2_dn7 * var_cox_inv) + (var_costi0_p2 * var_cox_inv_dn7)) * var_cox_inv) + (assign63200_e97937 * var_cox_inv_dn7)), ((((var_costi0_p2_dn8 * var_cox_inv) + (var_costi0_p2 * var_cox_inv_dn8)) * var_cox_inv) + (assign63200_e97937 * var_cox_inv_dn8)), ((((var_costi0_p2_dn9 * var_cox_inv) + (var_costi0_p2 * var_cox_inv_dn9)) * var_cox_inv) + (assign63200_e97937 * var_cox_inv_dn9)), ((((var_costi0_p2_dn10 * var_cox_inv) + (var_costi0_p2 * var_cox_inv_dn10)) * var_cox_inv) + (assign63200_e97937 * var_cox_inv_dn10)), ((((var_costi0_p2_dn13 * var_cox_inv) + (var_costi0_p2 * var_cox_inv_dn13)) * var_cox_inv) + (assign63200_e97937 * var_cox_inv_dn13)),)
    } else {
        (var_costi3, var_costi3_dn0, var_costi3_dn2, var_costi3_dn4, var_costi3_dn5, var_costi3_dn6, var_costi3_dn7, var_costi3_dn8, var_costi3_dn9, var_costi3_dn10, var_costi3_dn13,)
    }
};
        var_costi3 = assign63200_e97941;
        var_costi3_dn0 = assign63200_e97941_d_n0;
        var_costi3_dn2 = assign63200_e97941_d_n2;
        var_costi3_dn4 = assign63200_e97941_d_n4;
        var_costi3_dn5 = assign63200_e97941_d_n5;
        var_costi3_dn6 = assign63200_e97941_d_n6;
        var_costi3_dn7 = assign63200_e97941_d_n7;
        var_costi3_dn8 = assign63200_e97941_d_n8;
        var_costi3_dn9 = assign63200_e97941_d_n9;
        var_costi3_dn10 = assign63200_e97941_d_n10;
        var_costi3_dn13 = assign63200_e97941_d_n13;
        var_costi3_rv = 0.0;

        let (assign63210_e97954, assign63210_e97954_d_n0, assign63210_e97954_d_n2, assign63210_e97954_d_n4, assign63210_e97954_d_n5, assign63210_e97954_d_n6, assign63210_e97954_d_n7, assign63210_e97954_d_n8, assign63210_e97954_d_n9, assign63210_e97954_d_n10, assign63210_e97954_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63210_e97950: f64 = (var_costi3 * var_beta);
        let assign63210_e97952: f64 = (assign63210_e97950 * 0.5);
        (assign63210_e97952, (((var_costi3_dn0 * var_beta) + (var_costi3 * var_beta_dn0)) * 0.5), (((var_costi3_dn2 * var_beta) + (var_costi3 * var_beta_dn2)) * 0.5), (((var_costi3_dn4 * var_beta) + (var_costi3 * var_beta_dn4)) * 0.5), (((var_costi3_dn5 * var_beta) + (var_costi3 * var_beta_dn5)) * 0.5), (((var_costi3_dn6 * var_beta) + (var_costi3 * var_beta_dn6)) * 0.5), (((var_costi3_dn7 * var_beta) + (var_costi3 * var_beta_dn7)) * 0.5), (((var_costi3_dn8 * var_beta) + (var_costi3 * var_beta_dn8)) * 0.5), (((var_costi3_dn9 * var_beta) + (var_costi3 * var_beta_dn9)) * 0.5), (((var_costi3_dn10 * var_beta) + (var_costi3 * var_beta_dn10)) * 0.5), (((var_costi3_dn13 * var_beta) + (var_costi3 * var_beta_dn13)) * 0.5),)
    } else {
        (var_costi4, var_costi4_dn0, var_costi4_dn2, var_costi4_dn4, var_costi4_dn5, var_costi4_dn6, var_costi4_dn7, var_costi4_dn8, var_costi4_dn9, var_costi4_dn10, var_costi4_dn13,)
    }
};
        var_costi4 = assign63210_e97954;
        var_costi4_dn0 = assign63210_e97954_d_n0;
        var_costi4_dn2 = assign63210_e97954_d_n2;
        var_costi4_dn4 = assign63210_e97954_d_n4;
        var_costi4_dn5 = assign63210_e97954_d_n5;
        var_costi4_dn6 = assign63210_e97954_d_n6;
        var_costi4_dn7 = assign63210_e97954_d_n7;
        var_costi4_dn8 = assign63210_e97954_d_n8;
        var_costi4_dn9 = assign63210_e97954_d_n9;
        var_costi4_dn10 = assign63210_e97954_d_n10;
        var_costi4_dn13 = assign63210_e97954_d_n13;
        var_costi4_rv = 0.0;

        let (assign63220_e97967, assign63220_e97967_d_n0, assign63220_e97967_d_n2, assign63220_e97967_d_n4, assign63220_e97967_d_n5, assign63220_e97967_d_n6, assign63220_e97967_d_n7, assign63220_e97967_d_n8, assign63220_e97967_d_n9, assign63220_e97967_d_n10, assign63220_e97967_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63220_e97963: f64 = (var_costi4 * var_beta);
        let assign63220_e97965: f64 = (assign63220_e97963 * 2.0);
        (assign63220_e97965, (((var_costi4_dn0 * var_beta) + (var_costi4 * var_beta_dn0)) * 2.0), (((var_costi4_dn2 * var_beta) + (var_costi4 * var_beta_dn2)) * 2.0), (((var_costi4_dn4 * var_beta) + (var_costi4 * var_beta_dn4)) * 2.0), (((var_costi4_dn5 * var_beta) + (var_costi4 * var_beta_dn5)) * 2.0), (((var_costi4_dn6 * var_beta) + (var_costi4 * var_beta_dn6)) * 2.0), (((var_costi4_dn7 * var_beta) + (var_costi4 * var_beta_dn7)) * 2.0), (((var_costi4_dn8 * var_beta) + (var_costi4 * var_beta_dn8)) * 2.0), (((var_costi4_dn9 * var_beta) + (var_costi4 * var_beta_dn9)) * 2.0), (((var_costi4_dn10 * var_beta) + (var_costi4 * var_beta_dn10)) * 2.0), (((var_costi4_dn13 * var_beta) + (var_costi4 * var_beta_dn13)) * 2.0),)
    } else {
        (var_costi5, var_costi5_dn0, var_costi5_dn2, var_costi5_dn4, var_costi5_dn5, var_costi5_dn6, var_costi5_dn7, var_costi5_dn8, var_costi5_dn9, var_costi5_dn10, var_costi5_dn13,)
    }
};
        var_costi5 = assign63220_e97967;
        var_costi5_dn0 = assign63220_e97967_d_n0;
        var_costi5_dn2 = assign63220_e97967_d_n2;
        var_costi5_dn4 = assign63220_e97967_d_n4;
        var_costi5_dn5 = assign63220_e97967_d_n5;
        var_costi5_dn6 = assign63220_e97967_d_n6;
        var_costi5_dn7 = assign63220_e97967_d_n7;
        var_costi5_dn8 = assign63220_e97967_d_n8;
        var_costi5_dn9 = assign63220_e97967_d_n9;
        var_costi5_dn10 = assign63220_e97967_d_n10;
        var_costi5_dn13 = assign63220_e97967_d_n13;
        var_costi5_rv = 0.0;

        let (assign63230_e97978, assign63230_e97978_d_n0, assign63230_e97978_d_n2, assign63230_e97978_d_n4, assign63230_e97978_d_n5, assign63230_e97978_d_n6, assign63230_e97978_d_n7, assign63230_e97978_d_n8, assign63230_e97978_d_n9, assign63230_e97978_d_n10, assign63230_e97978_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63230_e97976: f64 = (var_beta * 0.25);
        (assign63230_e97976, (var_beta_dn0 * 0.25), (var_beta_dn2 * 0.25), (var_beta_dn4 * 0.25), (var_beta_dn5 * 0.25), (var_beta_dn6 * 0.25), (var_beta_dn7 * 0.25), (var_beta_dn8 * 0.25), (var_beta_dn9 * 0.25), (var_beta_dn10 * 0.25), (var_beta_dn13 * 0.25),)
    } else {
        (var_t11, var_t11_dn0, var_t11_dn2, var_t11_dn4, var_t11_dn5, var_t11_dn6, var_t11_dn7, var_t11_dn8, var_t11_dn9, var_t11_dn10, var_t11_dn13,)
    }
};
        var_t11 = assign63230_e97978;
        var_t11_dn0 = assign63230_e97978_d_n0;
        var_t11_dn2 = assign63230_e97978_d_n2;
        var_t11_dn4 = assign63230_e97978_d_n4;
        var_t11_dn5 = assign63230_e97978_d_n5;
        var_t11_dn6 = assign63230_e97978_d_n6;
        var_t11_dn7 = assign63230_e97978_d_n7;
        var_t11_dn8 = assign63230_e97978_d_n8;
        var_t11_dn9 = assign63230_e97978_d_n9;
        var_t11_dn10 = assign63230_e97978_d_n10;
        var_t11_dn13 = assign63230_e97978_d_n13;
        var_t11_rv = 0.0;

        let (assign63240_e97999, assign63240_e97999_d_n0, assign63240_e97999_d_n2, assign63240_e97999_d_n4, assign63240_e97999_d_n5, assign63240_e97999_d_n6, assign63240_e97999_d_n7, assign63240_e97999_d_n8, assign63240_e97999_d_n9, assign63240_e97999_d_n10, assign63240_e97999_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63240_e97988: f64 = (var_costi3 * var_t11);
        let assign63240_e97989: f64 = (var_beta_inv - assign63240_e97988);
        let assign63240_e97991: f64 = (assign63240_e97989 + var_vfb);
        let assign63240_e97993: f64 = (assign63240_e97991 - var_uc_vthsti);
        let assign63240_e97995: f64 = (assign63240_e97993 - var_dvthscsti);
        let assign63240_e97997: f64 = (assign63240_e97995 + 1e-25);
        (assign63240_e97997, ((var_beta_inv_dn0 - ((var_costi3_dn0 * var_t11) + (var_costi3 * var_t11_dn0))) - var_dvthscsti_dn0), ((var_beta_inv_dn2 - ((var_costi3_dn2 * var_t11) + (var_costi3 * var_t11_dn2))) - var_dvthscsti_dn2), ((var_beta_inv_dn4 - ((var_costi3_dn4 * var_t11) + (var_costi3 * var_t11_dn4))) - var_dvthscsti_dn4), ((var_beta_inv_dn5 - ((var_costi3_dn5 * var_t11) + (var_costi3 * var_t11_dn5))) - var_dvthscsti_dn5), ((var_beta_inv_dn6 - ((var_costi3_dn6 * var_t11) + (var_costi3 * var_t11_dn6))) - var_dvthscsti_dn6), ((var_beta_inv_dn7 - ((var_costi3_dn7 * var_t11) + (var_costi3 * var_t11_dn7))) - var_dvthscsti_dn7), ((var_beta_inv_dn8 - ((var_costi3_dn8 * var_t11) + (var_costi3 * var_t11_dn8))) - var_dvthscsti_dn8), ((var_beta_inv_dn9 - ((var_costi3_dn9 * var_t11) + (var_costi3 * var_t11_dn9))) - var_dvthscsti_dn9), ((var_beta_inv_dn10 - ((var_costi3_dn10 * var_t11) + (var_costi3 * var_t11_dn10))) - var_dvthscsti_dn10), ((var_beta_inv_dn13 - ((var_costi3_dn13 * var_t11) + (var_costi3 * var_t11_dn13))) - var_dvthscsti_dn13),)
    } else {
        (var_t10, var_t10_dn0, var_t10_dn2, var_t10_dn4, var_t10_dn5, var_t10_dn6, var_t10_dn7, var_t10_dn8, var_t10_dn9, var_t10_dn10, var_t10_dn13,)
    }
};
        var_t10 = assign63240_e97999;
        var_t10_dn0 = assign63240_e97999_d_n0;
        var_t10_dn2 = assign63240_e97999_d_n2;
        var_t10_dn4 = assign63240_e97999_d_n4;
        var_t10_dn5 = assign63240_e97999_d_n5;
        var_t10_dn6 = assign63240_e97999_d_n6;
        var_t10_dn7 = assign63240_e97999_d_n7;
        var_t10_dn8 = assign63240_e97999_d_n8;
        var_t10_dn9 = assign63240_e97999_d_n9;
        var_t10_dn10 = assign63240_e97999_d_n10;
        var_t10_dn13 = assign63240_e97999_d_n13;
        var_t10_rv = 0.0;

        let (assign63250_e98012, assign63250_e98012_d_n0, assign63250_e98012_d_n2, assign63250_e98012_d_n4, assign63250_e98012_d_n5, assign63250_e98012_d_n6, assign63250_e98012_d_n7, assign63250_e98012_d_n8, assign63250_e98012_d_n9, assign63250_e98012_d_n10, assign63250_e98012_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63250_e98008: f64 = (var_vgsz__blk440 - var_t10);
        let assign63250_e98010: f64 = (assign63250_e98008 - 0.005);
        (assign63250_e98010, (var_vgsz__blk440_dn0 - var_t10_dn0), (var_vgsz__blk440_dn2 - var_t10_dn2), (var_vgsz__blk440_dn4 - var_t10_dn4), (var_vgsz__blk440_dn5 - var_t10_dn5), (var_vgsz__blk440_dn6 - var_t10_dn6), (var_vgsz__blk440_dn7 - var_t10_dn7), (var_vgsz__blk440_dn8 - var_t10_dn8), (var_vgsz__blk440_dn9 - var_t10_dn9), (var_vgsz__blk440_dn10 - var_t10_dn10), (var_vgsz__blk440_dn13 - var_t10_dn13),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign63250_e98012;
        var_t1_dn0 = assign63250_e98012_d_n0;
        var_t1_dn2 = assign63250_e98012_d_n2;
        var_t1_dn4 = assign63250_e98012_d_n4;
        var_t1_dn5 = assign63250_e98012_d_n5;
        var_t1_dn6 = assign63250_e98012_d_n6;
        var_t1_dn7 = assign63250_e98012_d_n7;
        var_t1_dn8 = assign63250_e98012_d_n8;
        var_t1_dn9 = assign63250_e98012_d_n9;
        var_t1_dn10 = assign63250_e98012_d_n10;
        var_t1_dn13 = assign63250_e98012_d_n13;
        var_t1_rv = 0.0;

        let (assign63260_e98027, assign63260_e98027_d_n0, assign63260_e98027_d_n2, assign63260_e98027_d_n4, assign63260_e98027_d_n5, assign63260_e98027_d_n6, assign63260_e98027_d_n7, assign63260_e98027_d_n8, assign63260_e98027_d_n9, assign63260_e98027_d_n10, assign63260_e98027_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let (assign63260_e98025,) = {
            if (var_t10 >= 0.0) {
                (1.0,)
            } else {
                let assign63260_e98024: f64 = (-1.0);
                (assign63260_e98024,)
            }
        };
        (assign63260_e98025, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign63260_e98027;
        var_t0_dn0 = assign63260_e98027_d_n0;
        var_t0_dn2 = assign63260_e98027_d_n2;
        var_t0_dn4 = assign63260_e98027_d_n4;
        var_t0_dn5 = assign63260_e98027_d_n5;
        var_t0_dn6 = assign63260_e98027_d_n6;
        var_t0_dn7 = assign63260_e98027_d_n7;
        var_t0_dn8 = assign63260_e98027_d_n8;
        var_t0_dn9 = assign63260_e98027_d_n9;
        var_t0_dn10 = assign63260_e98027_d_n10;
        var_t0_dn13 = assign63260_e98027_d_n13;
        var_t0_rv = 0.0;

        let (assign63270_e98047, assign63270_e98047_d_n0, assign63270_e98047_d_n2, assign63270_e98047_d_n4, assign63270_e98047_d_n5, assign63270_e98047_d_n6, assign63270_e98047_d_n7, assign63270_e98047_d_n8, assign63270_e98047_d_n9, assign63270_e98047_d_n10, assign63270_e98047_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63270_e98036: f64 = (var_t1 * var_t1);
        let assign63270_e98039: f64 = (var_t0 * 4.0);
        let assign63270_e98041: f64 = (assign63270_e98039 * var_t10);
        let assign63270_e98043: f64 = (assign63270_e98041 * 0.005);
        let assign63270_e98044: f64 = (assign63270_e98036 + assign63270_e98043);
        let assign63270_e98045: f64 = (assign63270_e98044).sqrt();
        (assign63270_e98045, ((((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)) + ((((var_t0_dn0 * 4.0) * var_t10) + (assign63270_e98039 * var_t10_dn0)) * 0.005)) / (2.0 * assign63270_e98045)), ((((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)) + ((((var_t0_dn2 * 4.0) * var_t10) + (assign63270_e98039 * var_t10_dn2)) * 0.005)) / (2.0 * assign63270_e98045)), ((((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)) + ((((var_t0_dn4 * 4.0) * var_t10) + (assign63270_e98039 * var_t10_dn4)) * 0.005)) / (2.0 * assign63270_e98045)), ((((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)) + ((((var_t0_dn5 * 4.0) * var_t10) + (assign63270_e98039 * var_t10_dn5)) * 0.005)) / (2.0 * assign63270_e98045)), ((((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)) + ((((var_t0_dn6 * 4.0) * var_t10) + (assign63270_e98039 * var_t10_dn6)) * 0.005)) / (2.0 * assign63270_e98045)), ((((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7)) + ((((var_t0_dn7 * 4.0) * var_t10) + (assign63270_e98039 * var_t10_dn7)) * 0.005)) / (2.0 * assign63270_e98045)), ((((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)) + ((((var_t0_dn8 * 4.0) * var_t10) + (assign63270_e98039 * var_t10_dn8)) * 0.005)) / (2.0 * assign63270_e98045)), ((((var_t1_dn9 * var_t1) + (var_t1 * var_t1_dn9)) + ((((var_t0_dn9 * 4.0) * var_t10) + (assign63270_e98039 * var_t10_dn9)) * 0.005)) / (2.0 * assign63270_e98045)), ((((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10)) + ((((var_t0_dn10 * 4.0) * var_t10) + (assign63270_e98039 * var_t10_dn10)) * 0.005)) / (2.0 * assign63270_e98045)), ((((var_t1_dn13 * var_t1) + (var_t1 * var_t1_dn13)) + ((((var_t0_dn13 * 4.0) * var_t10) + (assign63270_e98039 * var_t10_dn13)) * 0.005)) / (2.0 * assign63270_e98045)),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign63270_e98047;
        var_t2_dn0 = assign63270_e98047_d_n0;
        var_t2_dn2 = assign63270_e98047_d_n2;
        var_t2_dn4 = assign63270_e98047_d_n4;
        var_t2_dn5 = assign63270_e98047_d_n5;
        var_t2_dn6 = assign63270_e98047_d_n6;
        var_t2_dn7 = assign63270_e98047_d_n7;
        var_t2_dn8 = assign63270_e98047_d_n8;
        var_t2_dn9 = assign63270_e98047_d_n9;
        var_t2_dn10 = assign63270_e98047_d_n10;
        var_t2_dn13 = assign63270_e98047_d_n13;
        var_t2_rv = 0.0;

        let (assign63280_e98070, assign63280_e98070_d_n0, assign63280_e98070_d_n2, assign63280_e98070_d_n4, assign63280_e98070_d_n5, assign63280_e98070_d_n6, assign63280_e98070_d_n7, assign63280_e98070_d_n8, assign63280_e98070_d_n9, assign63280_e98070_d_n10, assign63280_e98070_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63280_e98058: f64 = (var_t1 + var_t2);
        let assign63280_e98059: f64 = (0.5 * assign63280_e98058);
        let assign63280_e98060: f64 = (var_t10 + assign63280_e98059);
        let assign63280_e98062: f64 = (assign63280_e98060 - var_vfb);
        let assign63280_e98064: f64 = (assign63280_e98062 + var_uc_vthsti);
        let assign63280_e98066: f64 = (assign63280_e98064 + var_dvthscsti);
        let assign63280_e98068: f64 = (assign63280_e98066 - var_vbsz__blk438);
        (assign63280_e98068, (((var_t10_dn0 + (0.5 * (var_t1_dn0 + var_t2_dn0))) + var_dvthscsti_dn0) - var_vbsz__blk438_dn0), (((var_t10_dn2 + (0.5 * (var_t1_dn2 + var_t2_dn2))) + var_dvthscsti_dn2) - var_vbsz__blk438_dn2), (((var_t10_dn4 + (0.5 * (var_t1_dn4 + var_t2_dn4))) + var_dvthscsti_dn4) - var_vbsz__blk438_dn4), (((var_t10_dn5 + (0.5 * (var_t1_dn5 + var_t2_dn5))) + var_dvthscsti_dn5) - var_vbsz__blk438_dn5), (((var_t10_dn6 + (0.5 * (var_t1_dn6 + var_t2_dn6))) + var_dvthscsti_dn6) - var_vbsz__blk438_dn6), (((var_t10_dn7 + (0.5 * (var_t1_dn7 + var_t2_dn7))) + var_dvthscsti_dn7) - var_vbsz__blk438_dn7), (((var_t10_dn8 + (0.5 * (var_t1_dn8 + var_t2_dn8))) + var_dvthscsti_dn8) - var_vbsz__blk438_dn8), (((var_t10_dn9 + (0.5 * (var_t1_dn9 + var_t2_dn9))) + var_dvthscsti_dn9) - var_vbsz__blk438_dn9), (((var_t10_dn10 + (0.5 * (var_t1_dn10 + var_t2_dn10))) + var_dvthscsti_dn10) - var_vbsz__blk438_dn10), (((var_t10_dn13 + (0.5 * (var_t1_dn13 + var_t2_dn13))) + var_dvthscsti_dn13) - var_vbsz__blk438_dn13),)
    } else {
        (var_t3, var_t3_dn0, var_t3_dn2, var_t3_dn4, var_t3_dn5, var_t3_dn6, var_t3_dn7, var_t3_dn8, var_t3_dn9, var_t3_dn10, var_t3_dn13,)
    }
};
        var_t3 = assign63280_e98070;
        var_t3_dn0 = assign63280_e98070_d_n0;
        var_t3_dn2 = assign63280_e98070_d_n2;
        var_t3_dn4 = assign63280_e98070_d_n4;
        var_t3_dn5 = assign63280_e98070_d_n5;
        var_t3_dn6 = assign63280_e98070_d_n6;
        var_t3_dn7 = assign63280_e98070_d_n7;
        var_t3_dn8 = assign63280_e98070_d_n8;
        var_t3_dn9 = assign63280_e98070_d_n9;
        var_t3_dn10 = assign63280_e98070_d_n10;
        var_t3_dn13 = assign63280_e98070_d_n13;
        var_t3_rv = 0.0;

        *var_costi3_slot = var_costi3;
        *var_costi3_dn0_slot = var_costi3_dn0;
        *var_costi3_dn10_slot = var_costi3_dn10;
        *var_costi3_dn13_slot = var_costi3_dn13;
        *var_costi3_dn2_slot = var_costi3_dn2;
        *var_costi3_dn4_slot = var_costi3_dn4;
        *var_costi3_dn5_slot = var_costi3_dn5;
        *var_costi3_dn6_slot = var_costi3_dn6;
        *var_costi3_dn7_slot = var_costi3_dn7;
        *var_costi3_dn8_slot = var_costi3_dn8;
        *var_costi3_dn9_slot = var_costi3_dn9;
        *var_costi3_rv_slot = var_costi3_rv;
        *var_costi4_slot = var_costi4;
        *var_costi4_dn0_slot = var_costi4_dn0;
        *var_costi4_dn10_slot = var_costi4_dn10;
        *var_costi4_dn13_slot = var_costi4_dn13;
        *var_costi4_dn2_slot = var_costi4_dn2;
        *var_costi4_dn4_slot = var_costi4_dn4;
        *var_costi4_dn5_slot = var_costi4_dn5;
        *var_costi4_dn6_slot = var_costi4_dn6;
        *var_costi4_dn7_slot = var_costi4_dn7;
        *var_costi4_dn8_slot = var_costi4_dn8;
        *var_costi4_dn9_slot = var_costi4_dn9;
        *var_costi4_rv_slot = var_costi4_rv;
        *var_costi5_slot = var_costi5;
        *var_costi5_dn0_slot = var_costi5_dn0;
        *var_costi5_dn10_slot = var_costi5_dn10;
        *var_costi5_dn13_slot = var_costi5_dn13;
        *var_costi5_dn2_slot = var_costi5_dn2;
        *var_costi5_dn4_slot = var_costi5_dn4;
        *var_costi5_dn5_slot = var_costi5_dn5;
        *var_costi5_dn6_slot = var_costi5_dn6;
        *var_costi5_dn7_slot = var_costi5_dn7;
        *var_costi5_dn8_slot = var_costi5_dn8;
        *var_costi5_dn9_slot = var_costi5_dn9;
        *var_costi5_rv_slot = var_costi5_rv;
        *var_dvth0_slot = var_dvth0;
        *var_dvth0_dn0_slot = var_dvth0_dn0;
        *var_dvth0_dn10_slot = var_dvth0_dn10;
        *var_dvth0_dn13_slot = var_dvth0_dn13;
        *var_dvth0_dn2_slot = var_dvth0_dn2;
        *var_dvth0_dn4_slot = var_dvth0_dn4;
        *var_dvth0_dn5_slot = var_dvth0_dn5;
        *var_dvth0_dn6_slot = var_dvth0_dn6;
        *var_dvth0_dn7_slot = var_dvth0_dn7;
        *var_dvth0_dn8_slot = var_dvth0_dn8;
        *var_dvth0_dn9_slot = var_dvth0_dn9;
        *var_dvth0_rv_slot = var_dvth0_rv;
        *var_dvthscsti_slot = var_dvthscsti;
        *var_dvthscsti_dn0_slot = var_dvthscsti_dn0;
        *var_dvthscsti_dn10_slot = var_dvthscsti_dn10;
        *var_dvthscsti_dn13_slot = var_dvthscsti_dn13;
        *var_dvthscsti_dn2_slot = var_dvthscsti_dn2;
        *var_dvthscsti_dn4_slot = var_dvthscsti_dn4;
        *var_dvthscsti_dn5_slot = var_dvthscsti_dn5;
        *var_dvthscsti_dn6_slot = var_dvthscsti_dn6;
        *var_dvthscsti_dn7_slot = var_dvthscsti_dn7;
        *var_dvthscsti_dn8_slot = var_dvthscsti_dn8;
        *var_dvthscsti_dn9_slot = var_dvthscsti_dn9;
        *var_dvthscsti_rv_slot = var_dvthscsti_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn13_slot = var_t0_dn13;
        *var_t0_dn2_slot = var_t0_dn2;
        *var_t0_dn4_slot = var_t0_dn4;
        *var_t0_dn5_slot = var_t0_dn5;
        *var_t0_dn6_slot = var_t0_dn6;
        *var_t0_dn7_slot = var_t0_dn7;
        *var_t0_dn8_slot = var_t0_dn8;
        *var_t0_dn9_slot = var_t0_dn9;
        *var_t0_rv_slot = var_t0_rv;
        *var_t1_slot = var_t1;
        *var_t10_slot = var_t10;
        *var_t10_dn0_slot = var_t10_dn0;
        *var_t10_dn10_slot = var_t10_dn10;
        *var_t10_dn13_slot = var_t10_dn13;
        *var_t10_dn2_slot = var_t10_dn2;
        *var_t10_dn4_slot = var_t10_dn4;
        *var_t10_dn5_slot = var_t10_dn5;
        *var_t10_dn6_slot = var_t10_dn6;
        *var_t10_dn7_slot = var_t10_dn7;
        *var_t10_dn8_slot = var_t10_dn8;
        *var_t10_dn9_slot = var_t10_dn9;
        *var_t10_rv_slot = var_t10_rv;
        *var_t11_slot = var_t11;
        *var_t11_dn0_slot = var_t11_dn0;
        *var_t11_dn10_slot = var_t11_dn10;
        *var_t11_dn13_slot = var_t11_dn13;
        *var_t11_dn2_slot = var_t11_dn2;
        *var_t11_dn4_slot = var_t11_dn4;
        *var_t11_dn5_slot = var_t11_dn5;
        *var_t11_dn6_slot = var_t11_dn6;
        *var_t11_dn7_slot = var_t11_dn7;
        *var_t11_dn8_slot = var_t11_dn8;
        *var_t11_dn9_slot = var_t11_dn9;
        *var_t11_rv_slot = var_t11_rv;
        *var_t1_dn0_slot = var_t1_dn0;
        *var_t1_dn10_slot = var_t1_dn10;
        *var_t1_dn13_slot = var_t1_dn13;
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
        *var_t2_dn13_slot = var_t2_dn13;
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
        *var_t3_dn13_slot = var_t3_dn13;
        *var_t3_dn2_slot = var_t3_dn2;
        *var_t3_dn4_slot = var_t3_dn4;
        *var_t3_dn5_slot = var_t3_dn5;
        *var_t3_dn6_slot = var_t3_dn6;
        *var_t3_dn7_slot = var_t3_dn7;
        *var_t3_dn8_slot = var_t3_dn8;
        *var_t3_dn9_slot = var_t3_dn9;
        *var_t3_rv_slot = var_t3_rv;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn13_slot = var_t4_dn13;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_dn9_slot = var_t4_dn9;
        *var_t4_rv_slot = var_t4_rv;
        *var_t5_slot = var_t5;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn10_slot = var_t5_dn10;
        *var_t5_dn13_slot = var_t5_dn13;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t5_dn9_slot = var_t5_dn9;
        *var_t5_rv_slot = var_t5_rv;
        *var_t6_slot = var_t6;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn13_slot = var_t6_dn13;
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
        *var_t7_dn13_slot = var_t7_dn13;
        *var_t7_dn2_slot = var_t7_dn2;
        *var_t7_dn4_slot = var_t7_dn4;
        *var_t7_dn5_slot = var_t7_dn5;
        *var_t7_dn6_slot = var_t7_dn6;
        *var_t7_dn7_slot = var_t7_dn7;
        *var_t7_dn8_slot = var_t7_dn8;
        *var_t7_dn9_slot = var_t7_dn9;
        *var_t7_rv_slot = var_t7_rv;
        *var_t8_slot = var_t8;
        *var_t8_dn0_slot = var_t8_dn0;
        *var_t8_dn10_slot = var_t8_dn10;
        *var_t8_dn13_slot = var_t8_dn13;
        *var_t8_dn2_slot = var_t8_dn2;
        *var_t8_dn4_slot = var_t8_dn4;
        *var_t8_dn5_slot = var_t8_dn5;
        *var_t8_dn6_slot = var_t8_dn6;
        *var_t8_dn7_slot = var_t8_dn7;
        *var_t8_dn8_slot = var_t8_dn8;
        *var_t8_dn9_slot = var_t8_dn9;
        *var_t8_rv_slot = var_t8_rv;
        *var_vgssti_slot = var_vgssti;
        *var_vgssti_dn0_slot = var_vgssti_dn0;
        *var_vgssti_dn10_slot = var_vgssti_dn10;
        *var_vgssti_dn13_slot = var_vgssti_dn13;
        *var_vgssti_dn2_slot = var_vgssti_dn2;
        *var_vgssti_dn4_slot = var_vgssti_dn4;
        *var_vgssti_dn5_slot = var_vgssti_dn5;
        *var_vgssti_dn6_slot = var_vgssti_dn6;
        *var_vgssti_dn7_slot = var_vgssti_dn7;
        *var_vgssti_dn8_slot = var_vgssti_dn8;
        *var_vgssti_dn9_slot = var_vgssti_dn9;
        *var_vgssti_rv_slot = var_vgssti_rv;
    }

    pub(super) fn stamp_reactive_block_227(
        var_beta: f64,
        var_beta_dn0: f64,
        var_beta_dn10: f64,
        var_beta_dn13: f64,
        var_beta_dn2: f64,
        var_beta_dn4: f64,
        var_beta_dn5: f64,
        var_beta_dn6: f64,
        var_beta_dn7: f64,
        var_beta_dn8: f64,
        var_beta_dn9: f64,
        var_costi1: f64,
        var_costi1_dn0: f64,
        var_costi1_dn10: f64,
        var_costi1_dn13: f64,
        var_costi1_dn2: f64,
        var_costi1_dn4: f64,
        var_costi1_dn5: f64,
        var_costi1_dn6: f64,
        var_costi1_dn7: f64,
        var_costi1_dn8: f64,
        var_costi1_dn9: f64,
        var_costi3: f64,
        var_costi3_dn0: f64,
        var_costi3_dn10: f64,
        var_costi3_dn13: f64,
        var_costi3_dn2: f64,
        var_costi3_dn4: f64,
        var_costi3_dn5: f64,
        var_costi3_dn6: f64,
        var_costi3_dn7: f64,
        var_costi3_dn8: f64,
        var_costi3_dn9: f64,
        var_costi4: f64,
        var_costi4_dn0: f64,
        var_costi4_dn10: f64,
        var_costi4_dn13: f64,
        var_costi4_dn2: f64,
        var_costi4_dn4: f64,
        var_costi4_dn5: f64,
        var_costi4_dn6: f64,
        var_costi4_dn7: f64,
        var_costi4_dn8: f64,
        var_costi4_dn9: f64,
        var_costi5: f64,
        var_costi5_dn0: f64,
        var_costi5_dn10: f64,
        var_costi5_dn13: f64,
        var_costi5_dn2: f64,
        var_costi5_dn4: f64,
        var_costi5_dn5: f64,
        var_costi5_dn6: f64,
        var_costi5_dn7: f64,
        var_costi5_dn8: f64,
        var_costi5_dn9: f64,
        var_guard1430: f64,
        var_guard1503: f64,
        var_guard443: f64,
        var_t3: f64,
        var_t3_dn0: f64,
        var_t3_dn10: f64,
        var_t3_dn13: f64,
        var_t3_dn2: f64,
        var_t3_dn4: f64,
        var_t3_dn5: f64,
        var_t3_dn6: f64,
        var_t3_dn7: f64,
        var_t3_dn8: f64,
        var_t3_dn9: f64,
        var_vbsz__blk438: f64,
        var_vbsz__blk438_dn0: f64,
        var_vbsz__blk438_dn10: f64,
        var_vbsz__blk438_dn13: f64,
        var_vbsz__blk438_dn2: f64,
        var_vbsz__blk438_dn4: f64,
        var_vbsz__blk438_dn5: f64,
        var_vbsz__blk438_dn6: f64,
        var_vbsz__blk438_dn7: f64,
        var_vbsz__blk438_dn8: f64,
        var_vbsz__blk438_dn9: f64,
        var_vgssti: f64,
        var_vgssti_dn0: f64,
        var_vgssti_dn10: f64,
        var_vgssti_dn13: f64,
        var_vgssti_dn2: f64,
        var_vgssti_dn4: f64,
        var_vgssti_dn5: f64,
        var_vgssti_dn6: f64,
        var_vgssti_dn7: f64,
        var_vgssti_dn8: f64,
        var_vgssti_dn9: f64,
        var_costi6_slot: &mut f64,
        var_costi6_dn0_slot: &mut f64,
        var_costi6_dn10_slot: &mut f64,
        var_costi6_dn13_slot: &mut f64,
        var_costi6_dn2_slot: &mut f64,
        var_costi6_dn4_slot: &mut f64,
        var_costi6_dn5_slot: &mut f64,
        var_costi6_dn6_slot: &mut f64,
        var_costi6_dn7_slot: &mut f64,
        var_costi6_dn8_slot: &mut f64,
        var_costi6_dn9_slot: &mut f64,
        var_costi6_rv_slot: &mut f64,
        var_guard1504_slot: &mut f64,
        var_guard1504_rv_slot: &mut f64,
        var_psab_slot: &mut f64,
        var_psab_dn0_slot: &mut f64,
        var_psab_dn10_slot: &mut f64,
        var_psab_dn13_slot: &mut f64,
        var_psab_dn2_slot: &mut f64,
        var_psab_dn4_slot: &mut f64,
        var_psab_dn5_slot: &mut f64,
        var_psab_dn6_slot: &mut f64,
        var_psab_dn7_slot: &mut f64,
        var_psab_dn8_slot: &mut f64,
        var_psab_dn9_slot: &mut f64,
        var_psab_rv_slot: &mut f64,
        var_psasti_slot: &mut f64,
        var_psasti_dn0_slot: &mut f64,
        var_psasti_dn10_slot: &mut f64,
        var_psasti_dn13_slot: &mut f64,
        var_psasti_dn2_slot: &mut f64,
        var_psasti_dn4_slot: &mut f64,
        var_psasti_dn5_slot: &mut f64,
        var_psasti_dn6_slot: &mut f64,
        var_psasti_dn7_slot: &mut f64,
        var_psasti_dn8_slot: &mut f64,
        var_psasti_dn9_slot: &mut f64,
        var_psasti_rv_slot: &mut f64,
        var_psbsti_slot: &mut f64,
        var_psbsti_dn0_slot: &mut f64,
        var_psbsti_dn10_slot: &mut f64,
        var_psbsti_dn13_slot: &mut f64,
        var_psbsti_dn2_slot: &mut f64,
        var_psbsti_dn4_slot: &mut f64,
        var_psbsti_dn5_slot: &mut f64,
        var_psbsti_dn6_slot: &mut f64,
        var_psbsti_dn7_slot: &mut f64,
        var_psbsti_dn8_slot: &mut f64,
        var_psbsti_dn9_slot: &mut f64,
        var_psbsti_rv_slot: &mut f64,
        var_psti_slot: &mut f64,
        var_psti_dn0_slot: &mut f64,
        var_psti_dn10_slot: &mut f64,
        var_psti_dn13_slot: &mut f64,
        var_psti_dn2_slot: &mut f64,
        var_psti_dn4_slot: &mut f64,
        var_psti_dn5_slot: &mut f64,
        var_psti_dn6_slot: &mut f64,
        var_psti_dn7_slot: &mut f64,
        var_psti_dn8_slot: &mut f64,
        var_psti_dn9_slot: &mut f64,
        var_psti_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn13_slot: &mut f64,
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
        var_t1_dn13_slot: &mut f64,
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
        var_t2_dn13_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_t4_slot: &mut f64,
        var_t4_dn0_slot: &mut f64,
        var_t4_dn10_slot: &mut f64,
        var_t4_dn13_slot: &mut f64,
        var_t4_dn2_slot: &mut f64,
        var_t4_dn4_slot: &mut f64,
        var_t4_dn5_slot: &mut f64,
        var_t4_dn6_slot: &mut f64,
        var_t4_dn7_slot: &mut f64,
        var_t4_dn8_slot: &mut f64,
        var_t4_dn9_slot: &mut f64,
        var_t4_rv_slot: &mut f64,
        var_t5_slot: &mut f64,
        var_t5_dn0_slot: &mut f64,
        var_t5_dn10_slot: &mut f64,
        var_t5_dn13_slot: &mut f64,
        var_t5_dn2_slot: &mut f64,
        var_t5_dn4_slot: &mut f64,
        var_t5_dn5_slot: &mut f64,
        var_t5_dn6_slot: &mut f64,
        var_t5_dn7_slot: &mut f64,
        var_t5_dn8_slot: &mut f64,
        var_t5_dn9_slot: &mut f64,
        var_t5_rv_slot: &mut f64,
        var_t6_slot: &mut f64,
        var_t6_dn0_slot: &mut f64,
        var_t6_dn10_slot: &mut f64,
        var_t6_dn13_slot: &mut f64,
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
        var_t7_dn13_slot: &mut f64,
        var_t7_dn2_slot: &mut f64,
        var_t7_dn4_slot: &mut f64,
        var_t7_dn5_slot: &mut f64,
        var_t7_dn6_slot: &mut f64,
        var_t7_dn7_slot: &mut f64,
        var_t7_dn8_slot: &mut f64,
        var_t7_dn9_slot: &mut f64,
        var_t7_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn13_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn7_slot: &mut f64,
        var_tmf2_dn8_slot: &mut f64,
        var_tmf2_dn9_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
    ) {
        let mut var_costi6: f64 = *var_costi6_slot;
        let mut var_costi6_dn0: f64 = *var_costi6_dn0_slot;
        let mut var_costi6_dn10: f64 = *var_costi6_dn10_slot;
        let mut var_costi6_dn13: f64 = *var_costi6_dn13_slot;
        let mut var_costi6_dn2: f64 = *var_costi6_dn2_slot;
        let mut var_costi6_dn4: f64 = *var_costi6_dn4_slot;
        let mut var_costi6_dn5: f64 = *var_costi6_dn5_slot;
        let mut var_costi6_dn6: f64 = *var_costi6_dn6_slot;
        let mut var_costi6_dn7: f64 = *var_costi6_dn7_slot;
        let mut var_costi6_dn8: f64 = *var_costi6_dn8_slot;
        let mut var_costi6_dn9: f64 = *var_costi6_dn9_slot;
        let mut var_costi6_rv: f64 = *var_costi6_rv_slot;
        let mut var_guard1504: f64 = *var_guard1504_slot;
        let mut var_guard1504_rv: f64 = *var_guard1504_rv_slot;
        let mut var_psab: f64 = *var_psab_slot;
        let mut var_psab_dn0: f64 = *var_psab_dn0_slot;
        let mut var_psab_dn10: f64 = *var_psab_dn10_slot;
        let mut var_psab_dn13: f64 = *var_psab_dn13_slot;
        let mut var_psab_dn2: f64 = *var_psab_dn2_slot;
        let mut var_psab_dn4: f64 = *var_psab_dn4_slot;
        let mut var_psab_dn5: f64 = *var_psab_dn5_slot;
        let mut var_psab_dn6: f64 = *var_psab_dn6_slot;
        let mut var_psab_dn7: f64 = *var_psab_dn7_slot;
        let mut var_psab_dn8: f64 = *var_psab_dn8_slot;
        let mut var_psab_dn9: f64 = *var_psab_dn9_slot;
        let mut var_psab_rv: f64 = *var_psab_rv_slot;
        let mut var_psasti: f64 = *var_psasti_slot;
        let mut var_psasti_dn0: f64 = *var_psasti_dn0_slot;
        let mut var_psasti_dn10: f64 = *var_psasti_dn10_slot;
        let mut var_psasti_dn13: f64 = *var_psasti_dn13_slot;
        let mut var_psasti_dn2: f64 = *var_psasti_dn2_slot;
        let mut var_psasti_dn4: f64 = *var_psasti_dn4_slot;
        let mut var_psasti_dn5: f64 = *var_psasti_dn5_slot;
        let mut var_psasti_dn6: f64 = *var_psasti_dn6_slot;
        let mut var_psasti_dn7: f64 = *var_psasti_dn7_slot;
        let mut var_psasti_dn8: f64 = *var_psasti_dn8_slot;
        let mut var_psasti_dn9: f64 = *var_psasti_dn9_slot;
        let mut var_psasti_rv: f64 = *var_psasti_rv_slot;
        let mut var_psbsti: f64 = *var_psbsti_slot;
        let mut var_psbsti_dn0: f64 = *var_psbsti_dn0_slot;
        let mut var_psbsti_dn10: f64 = *var_psbsti_dn10_slot;
        let mut var_psbsti_dn13: f64 = *var_psbsti_dn13_slot;
        let mut var_psbsti_dn2: f64 = *var_psbsti_dn2_slot;
        let mut var_psbsti_dn4: f64 = *var_psbsti_dn4_slot;
        let mut var_psbsti_dn5: f64 = *var_psbsti_dn5_slot;
        let mut var_psbsti_dn6: f64 = *var_psbsti_dn6_slot;
        let mut var_psbsti_dn7: f64 = *var_psbsti_dn7_slot;
        let mut var_psbsti_dn8: f64 = *var_psbsti_dn8_slot;
        let mut var_psbsti_dn9: f64 = *var_psbsti_dn9_slot;
        let mut var_psbsti_rv: f64 = *var_psbsti_rv_slot;
        let mut var_psti: f64 = *var_psti_slot;
        let mut var_psti_dn0: f64 = *var_psti_dn0_slot;
        let mut var_psti_dn10: f64 = *var_psti_dn10_slot;
        let mut var_psti_dn13: f64 = *var_psti_dn13_slot;
        let mut var_psti_dn2: f64 = *var_psti_dn2_slot;
        let mut var_psti_dn4: f64 = *var_psti_dn4_slot;
        let mut var_psti_dn5: f64 = *var_psti_dn5_slot;
        let mut var_psti_dn6: f64 = *var_psti_dn6_slot;
        let mut var_psti_dn7: f64 = *var_psti_dn7_slot;
        let mut var_psti_dn8: f64 = *var_psti_dn8_slot;
        let mut var_psti_dn9: f64 = *var_psti_dn9_slot;
        let mut var_psti_rv: f64 = *var_psti_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn13: f64 = *var_t0_dn13_slot;
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
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
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
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_t4: f64 = *var_t4_slot;
        let mut var_t4_dn0: f64 = *var_t4_dn0_slot;
        let mut var_t4_dn10: f64 = *var_t4_dn10_slot;
        let mut var_t4_dn13: f64 = *var_t4_dn13_slot;
        let mut var_t4_dn2: f64 = *var_t4_dn2_slot;
        let mut var_t4_dn4: f64 = *var_t4_dn4_slot;
        let mut var_t4_dn5: f64 = *var_t4_dn5_slot;
        let mut var_t4_dn6: f64 = *var_t4_dn6_slot;
        let mut var_t4_dn7: f64 = *var_t4_dn7_slot;
        let mut var_t4_dn8: f64 = *var_t4_dn8_slot;
        let mut var_t4_dn9: f64 = *var_t4_dn9_slot;
        let mut var_t4_rv: f64 = *var_t4_rv_slot;
        let mut var_t5: f64 = *var_t5_slot;
        let mut var_t5_dn0: f64 = *var_t5_dn0_slot;
        let mut var_t5_dn10: f64 = *var_t5_dn10_slot;
        let mut var_t5_dn13: f64 = *var_t5_dn13_slot;
        let mut var_t5_dn2: f64 = *var_t5_dn2_slot;
        let mut var_t5_dn4: f64 = *var_t5_dn4_slot;
        let mut var_t5_dn5: f64 = *var_t5_dn5_slot;
        let mut var_t5_dn6: f64 = *var_t5_dn6_slot;
        let mut var_t5_dn7: f64 = *var_t5_dn7_slot;
        let mut var_t5_dn8: f64 = *var_t5_dn8_slot;
        let mut var_t5_dn9: f64 = *var_t5_dn9_slot;
        let mut var_t5_rv: f64 = *var_t5_rv_slot;
        let mut var_t6: f64 = *var_t6_slot;
        let mut var_t6_dn0: f64 = *var_t6_dn0_slot;
        let mut var_t6_dn10: f64 = *var_t6_dn10_slot;
        let mut var_t6_dn13: f64 = *var_t6_dn13_slot;
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
        let mut var_t7_dn13: f64 = *var_t7_dn13_slot;
        let mut var_t7_dn2: f64 = *var_t7_dn2_slot;
        let mut var_t7_dn4: f64 = *var_t7_dn4_slot;
        let mut var_t7_dn5: f64 = *var_t7_dn5_slot;
        let mut var_t7_dn6: f64 = *var_t7_dn6_slot;
        let mut var_t7_dn7: f64 = *var_t7_dn7_slot;
        let mut var_t7_dn8: f64 = *var_t7_dn8_slot;
        let mut var_t7_dn9: f64 = *var_t7_dn9_slot;
        let mut var_t7_rv: f64 = *var_t7_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn13: f64 = *var_tmf2_dn13_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn7: f64 = *var_tmf2_dn7_slot;
        let mut var_tmf2_dn8: f64 = *var_tmf2_dn8_slot;
        let mut var_tmf2_dn9: f64 = *var_tmf2_dn9_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;

        let (assign63290_e98083, assign63290_e98083_d_n0, assign63290_e98083_d_n2, assign63290_e98083_d_n4, assign63290_e98083_d_n5, assign63290_e98083_d_n6, assign63290_e98083_d_n7, assign63290_e98083_d_n8, assign63290_e98083_d_n9, assign63290_e98083_d_n10, assign63290_e98083_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63290_e98079: f64 = (var_beta * var_t3);
        let assign63290_e98081: f64 = (assign63290_e98079 - 1.0);
        (assign63290_e98081, ((var_beta_dn0 * var_t3) + (var_beta * var_t3_dn0)), ((var_beta_dn2 * var_t3) + (var_beta * var_t3_dn2)), ((var_beta_dn4 * var_t3) + (var_beta * var_t3_dn4)), ((var_beta_dn5 * var_t3) + (var_beta * var_t3_dn5)), ((var_beta_dn6 * var_t3) + (var_beta * var_t3_dn6)), ((var_beta_dn7 * var_t3) + (var_beta * var_t3_dn7)), ((var_beta_dn8 * var_t3) + (var_beta * var_t3_dn8)), ((var_beta_dn9 * var_t3) + (var_beta * var_t3_dn9)), ((var_beta_dn10 * var_t3) + (var_beta * var_t3_dn10)), ((var_beta_dn13 * var_t3) + (var_beta * var_t3_dn13)),)
    } else {
        (var_t4, var_t4_dn0, var_t4_dn2, var_t4_dn4, var_t4_dn5, var_t4_dn6, var_t4_dn7, var_t4_dn8, var_t4_dn9, var_t4_dn10, var_t4_dn13,)
    }
};
        var_t4 = assign63290_e98083;
        var_t4_dn0 = assign63290_e98083_d_n0;
        var_t4_dn2 = assign63290_e98083_d_n2;
        var_t4_dn4 = assign63290_e98083_d_n4;
        var_t4_dn5 = assign63290_e98083_d_n5;
        var_t4_dn6 = assign63290_e98083_d_n6;
        var_t4_dn7 = assign63290_e98083_d_n7;
        var_t4_dn8 = assign63290_e98083_d_n8;
        var_t4_dn9 = assign63290_e98083_d_n9;
        var_t4_dn10 = assign63290_e98083_d_n10;
        var_t4_dn13 = assign63290_e98083_d_n13;
        var_t4_rv = 0.0;

        let (assign63300_e98094, assign63300_e98094_d_n0, assign63300_e98094_d_n2, assign63300_e98094_d_n4, assign63300_e98094_d_n5, assign63300_e98094_d_n6, assign63300_e98094_d_n7, assign63300_e98094_d_n8, assign63300_e98094_d_n9, assign63300_e98094_d_n10, assign63300_e98094_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63300_e98092: f64 = (4.0 / var_costi5);
        (assign63300_e98092, (-((4.0 * var_costi5_dn0) / (var_costi5 * var_costi5))), (-((4.0 * var_costi5_dn2) / (var_costi5 * var_costi5))), (-((4.0 * var_costi5_dn4) / (var_costi5 * var_costi5))), (-((4.0 * var_costi5_dn5) / (var_costi5 * var_costi5))), (-((4.0 * var_costi5_dn6) / (var_costi5 * var_costi5))), (-((4.0 * var_costi5_dn7) / (var_costi5 * var_costi5))), (-((4.0 * var_costi5_dn8) / (var_costi5 * var_costi5))), (-((4.0 * var_costi5_dn9) / (var_costi5 * var_costi5))), (-((4.0 * var_costi5_dn10) / (var_costi5 * var_costi5))), (-((4.0 * var_costi5_dn13) / (var_costi5 * var_costi5))),)
    } else {
        (var_t5, var_t5_dn0, var_t5_dn2, var_t5_dn4, var_t5_dn5, var_t5_dn6, var_t5_dn7, var_t5_dn8, var_t5_dn9, var_t5_dn10, var_t5_dn13,)
    }
};
        var_t5 = assign63300_e98094;
        var_t5_dn0 = assign63300_e98094_d_n0;
        var_t5_dn2 = assign63300_e98094_d_n2;
        var_t5_dn4 = assign63300_e98094_d_n4;
        var_t5_dn5 = assign63300_e98094_d_n5;
        var_t5_dn6 = assign63300_e98094_d_n6;
        var_t5_dn7 = assign63300_e98094_d_n7;
        var_t5_dn8 = assign63300_e98094_d_n8;
        var_t5_dn9 = assign63300_e98094_d_n9;
        var_t5_dn10 = assign63300_e98094_d_n10;
        var_t5_dn13 = assign63300_e98094_d_n13;
        var_t5_rv = 0.0;

        let (assign63310_e98107, assign63310_e98107_d_n0, assign63310_e98107_d_n2, assign63310_e98107_d_n4, assign63310_e98107_d_n5, assign63310_e98107_d_n6, assign63310_e98107_d_n7, assign63310_e98107_d_n8, assign63310_e98107_d_n9, assign63310_e98107_d_n10, assign63310_e98107_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63310_e98104: f64 = (var_t4 * var_t5);
        let assign63310_e98105: f64 = (1.0 + assign63310_e98104);
        (assign63310_e98105, ((var_t4_dn0 * var_t5) + (var_t4 * var_t5_dn0)), ((var_t4_dn2 * var_t5) + (var_t4 * var_t5_dn2)), ((var_t4_dn4 * var_t5) + (var_t4 * var_t5_dn4)), ((var_t4_dn5 * var_t5) + (var_t4 * var_t5_dn5)), ((var_t4_dn6 * var_t5) + (var_t4 * var_t5_dn6)), ((var_t4_dn7 * var_t5) + (var_t4 * var_t5_dn7)), ((var_t4_dn8 * var_t5) + (var_t4 * var_t5_dn8)), ((var_t4_dn9 * var_t5) + (var_t4 * var_t5_dn9)), ((var_t4_dn10 * var_t5) + (var_t4 * var_t5_dn10)), ((var_t4_dn13 * var_t5) + (var_t4 * var_t5_dn13)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign63310_e98107;
        var_t1_dn0 = assign63310_e98107_d_n0;
        var_t1_dn2 = assign63310_e98107_d_n2;
        var_t1_dn4 = assign63310_e98107_d_n4;
        var_t1_dn5 = assign63310_e98107_d_n5;
        var_t1_dn6 = assign63310_e98107_d_n6;
        var_t1_dn7 = assign63310_e98107_d_n7;
        var_t1_dn8 = assign63310_e98107_d_n8;
        var_t1_dn9 = assign63310_e98107_d_n9;
        var_t1_dn10 = assign63310_e98107_d_n10;
        var_t1_dn13 = assign63310_e98107_d_n13;
        var_t1_rv = 0.0;

        let (assign63320_e98118, assign63320_e98118_d_n0, assign63320_e98118_d_n2, assign63320_e98118_d_n4, assign63320_e98118_d_n5, assign63320_e98118_d_n6, assign63320_e98118_d_n7, assign63320_e98118_d_n8, assign63320_e98118_d_n9, assign63320_e98118_d_n10, assign63320_e98118_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63320_e98116: f64 = (var_beta * var_t5);
        (assign63320_e98116, ((var_beta_dn0 * var_t5) + (var_beta * var_t5_dn0)), ((var_beta_dn2 * var_t5) + (var_beta * var_t5_dn2)), ((var_beta_dn4 * var_t5) + (var_beta * var_t5_dn4)), ((var_beta_dn5 * var_t5) + (var_beta * var_t5_dn5)), ((var_beta_dn6 * var_t5) + (var_beta * var_t5_dn6)), ((var_beta_dn7 * var_t5) + (var_beta * var_t5_dn7)), ((var_beta_dn8 * var_t5) + (var_beta * var_t5_dn8)), ((var_beta_dn9 * var_t5) + (var_beta * var_t5_dn9)), ((var_beta_dn10 * var_t5) + (var_beta * var_t5_dn10)), ((var_beta_dn13 * var_t5) + (var_beta * var_t5_dn13)),)
    } else {
        (var_t6, var_t6_dn0, var_t6_dn2, var_t6_dn4, var_t6_dn5, var_t6_dn6, var_t6_dn7, var_t6_dn8, var_t6_dn9, var_t6_dn10, var_t6_dn13,)
    }
};
        var_t6 = assign63320_e98118;
        var_t6_dn0 = assign63320_e98118_d_n0;
        var_t6_dn2 = assign63320_e98118_d_n2;
        var_t6_dn4 = assign63320_e98118_d_n4;
        var_t6_dn5 = assign63320_e98118_d_n5;
        var_t6_dn6 = assign63320_e98118_d_n6;
        var_t6_dn7 = assign63320_e98118_d_n7;
        var_t6_dn8 = assign63320_e98118_d_n8;
        var_t6_dn9 = assign63320_e98118_d_n9;
        var_t6_dn10 = assign63320_e98118_d_n10;
        var_t6_dn13 = assign63320_e98118_d_n13;
        var_t6_rv = 0.0;

        let (assign63330_e98129, assign63330_e98129_d_n0, assign63330_e98129_d_n2, assign63330_e98129_d_n4, assign63330_e98129_d_n5, assign63330_e98129_d_n6, assign63330_e98129_d_n7, assign63330_e98129_d_n8, assign63330_e98129_d_n9, assign63330_e98129_d_n10, assign63330_e98129_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63330_e98127: f64 = (var_t4 * var_t5);
        (assign63330_e98127, ((var_t4_dn0 * var_t5) + (var_t4 * var_t5_dn0)), ((var_t4_dn2 * var_t5) + (var_t4 * var_t5_dn2)), ((var_t4_dn4 * var_t5) + (var_t4 * var_t5_dn4)), ((var_t4_dn5 * var_t5) + (var_t4 * var_t5_dn5)), ((var_t4_dn6 * var_t5) + (var_t4 * var_t5_dn6)), ((var_t4_dn7 * var_t5) + (var_t4 * var_t5_dn7)), ((var_t4_dn8 * var_t5) + (var_t4 * var_t5_dn8)), ((var_t4_dn9 * var_t5) + (var_t4 * var_t5_dn9)), ((var_t4_dn10 * var_t5) + (var_t4 * var_t5_dn10)), ((var_t4_dn13 * var_t5) + (var_t4 * var_t5_dn13)),)
    } else {
        (var_t7, var_t7_dn0, var_t7_dn2, var_t7_dn4, var_t7_dn5, var_t7_dn6, var_t7_dn7, var_t7_dn8, var_t7_dn9, var_t7_dn10, var_t7_dn13,)
    }
};
        var_t7 = assign63330_e98129;
        var_t7_dn0 = assign63330_e98129_d_n0;
        var_t7_dn2 = assign63330_e98129_d_n2;
        var_t7_dn4 = assign63330_e98129_d_n4;
        var_t7_dn5 = assign63330_e98129_d_n5;
        var_t7_dn6 = assign63330_e98129_d_n6;
        var_t7_dn7 = assign63330_e98129_d_n7;
        var_t7_dn8 = assign63330_e98129_d_n8;
        var_t7_dn9 = assign63330_e98129_d_n9;
        var_t7_dn10 = assign63330_e98129_d_n10;
        var_t7_dn13 = assign63330_e98129_d_n13;
        var_t7_rv = 0.0;

        let (assign63340_e98147, assign63340_e98147_d_n0, assign63340_e98147_d_n2, assign63340_e98147_d_n4, assign63340_e98147_d_n5, assign63340_e98147_d_n6, assign63340_e98147_d_n7, assign63340_e98147_d_n8, assign63340_e98147_d_n9, assign63340_e98147_d_n10, assign63340_e98147_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63340_e98138: f64 = (var_t1 * var_t1);
        let assign63340_e98141: f64 = (4.0 * 0.01);
        let assign63340_e98143: f64 = (assign63340_e98141 * 0.01);
        let assign63340_e98144: f64 = (assign63340_e98138 + assign63340_e98143);
        let assign63340_e98145: f64 = (assign63340_e98144).sqrt();
        (assign63340_e98145, (((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)) / (2.0 * assign63340_e98145)), (((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)) / (2.0 * assign63340_e98145)), (((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)) / (2.0 * assign63340_e98145)), (((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)) / (2.0 * assign63340_e98145)), (((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)) / (2.0 * assign63340_e98145)), (((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7)) / (2.0 * assign63340_e98145)), (((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)) / (2.0 * assign63340_e98145)), (((var_t1_dn9 * var_t1) + (var_t1 * var_t1_dn9)) / (2.0 * assign63340_e98145)), (((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10)) / (2.0 * assign63340_e98145)), (((var_t1_dn13 * var_t1) + (var_t1 * var_t1_dn13)) / (2.0 * assign63340_e98145)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
    }
};
        var_tmf2 = assign63340_e98147;
        var_tmf2_dn0 = assign63340_e98147_d_n0;
        var_tmf2_dn2 = assign63340_e98147_d_n2;
        var_tmf2_dn4 = assign63340_e98147_d_n4;
        var_tmf2_dn5 = assign63340_e98147_d_n5;
        var_tmf2_dn6 = assign63340_e98147_d_n6;
        var_tmf2_dn7 = assign63340_e98147_d_n7;
        var_tmf2_dn8 = assign63340_e98147_d_n8;
        var_tmf2_dn9 = assign63340_e98147_d_n9;
        var_tmf2_dn10 = assign63340_e98147_d_n10;
        var_tmf2_dn13 = assign63340_e98147_d_n13;
        var_tmf2_rv = 0.0;

        let (assign63350_e98162, assign63350_e98162_d_n0, assign63350_e98162_d_n2, assign63350_e98162_d_n4, assign63350_e98162_d_n5, assign63350_e98162_d_n6, assign63350_e98162_d_n7, assign63350_e98162_d_n8, assign63350_e98162_d_n9, assign63350_e98162_d_n10, assign63350_e98162_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63350_e98158: f64 = (var_t1 / var_tmf2);
        let assign63350_e98159: f64 = (1.0 + assign63350_e98158);
        let assign63350_e98160: f64 = (0.5 * assign63350_e98159);
        (assign63350_e98160, (0.5 * (((var_t1_dn0 * var_tmf2) - (var_t1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn2 * var_tmf2) - (var_t1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn4 * var_tmf2) - (var_t1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn5 * var_tmf2) - (var_t1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn6 * var_tmf2) - (var_t1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn7 * var_tmf2) - (var_t1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn8 * var_tmf2) - (var_t1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn9 * var_tmf2) - (var_t1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn10 * var_tmf2) - (var_t1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn13 * var_tmf2) - (var_t1 * var_tmf2_dn13)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign63350_e98162;
        var_t2_dn0 = assign63350_e98162_d_n0;
        var_t2_dn2 = assign63350_e98162_d_n2;
        var_t2_dn4 = assign63350_e98162_d_n4;
        var_t2_dn5 = assign63350_e98162_d_n5;
        var_t2_dn6 = assign63350_e98162_d_n6;
        var_t2_dn7 = assign63350_e98162_d_n7;
        var_t2_dn8 = assign63350_e98162_d_n8;
        var_t2_dn9 = assign63350_e98162_d_n9;
        var_t2_dn10 = assign63350_e98162_d_n10;
        var_t2_dn13 = assign63350_e98162_d_n13;
        var_t2_rv = 0.0;

        let (assign63360_e98175, assign63360_e98175_d_n0, assign63360_e98175_d_n2, assign63360_e98175_d_n4, assign63360_e98175_d_n5, assign63360_e98175_d_n6, assign63360_e98175_d_n7, assign63360_e98175_d_n8, assign63360_e98175_d_n9, assign63360_e98175_d_n10, assign63360_e98175_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63360_e98172: f64 = (var_t1 + var_tmf2);
        let assign63360_e98173: f64 = (0.5 * assign63360_e98172);
        (assign63360_e98173, (0.5 * (var_t1_dn0 + var_tmf2_dn0)), (0.5 * (var_t1_dn2 + var_tmf2_dn2)), (0.5 * (var_t1_dn4 + var_tmf2_dn4)), (0.5 * (var_t1_dn5 + var_tmf2_dn5)), (0.5 * (var_t1_dn6 + var_tmf2_dn6)), (0.5 * (var_t1_dn7 + var_tmf2_dn7)), (0.5 * (var_t1_dn8 + var_tmf2_dn8)), (0.5 * (var_t1_dn9 + var_tmf2_dn9)), (0.5 * (var_t1_dn10 + var_tmf2_dn10)), (0.5 * (var_t1_dn13 + var_tmf2_dn13)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign63360_e98175;
        var_t1_dn0 = assign63360_e98175_d_n0;
        var_t1_dn2 = assign63360_e98175_d_n2;
        var_t1_dn4 = assign63360_e98175_d_n4;
        var_t1_dn5 = assign63360_e98175_d_n5;
        var_t1_dn6 = assign63360_e98175_d_n6;
        var_t1_dn7 = assign63360_e98175_d_n7;
        var_t1_dn8 = assign63360_e98175_d_n8;
        var_t1_dn9 = assign63360_e98175_d_n9;
        var_t1_dn10 = assign63360_e98175_d_n10;
        var_t1_dn13 = assign63360_e98175_d_n13;
        var_t1_rv = 0.0;

        let assign63370_e98178: f64 = if var_t1 < 0.0 { 1.0 } else { 0.0 };
        var_guard1504 = assign63370_e98178;
        var_guard1504_rv = 0.0;

        let (assign63380_e98189, assign63380_e98189_d_n0, assign63380_e98189_d_n2, assign63380_e98189_d_n4, assign63380_e98189_d_n5, assign63380_e98189_d_n6, assign63380_e98189_d_n7, assign63380_e98189_d_n8, assign63380_e98189_d_n9, assign63380_e98189_d_n10, assign63380_e98189_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) && (var_guard1504 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign63380_e98189;
        var_t1_dn0 = assign63380_e98189_d_n0;
        var_t1_dn2 = assign63380_e98189_d_n2;
        var_t1_dn4 = assign63380_e98189_d_n4;
        var_t1_dn5 = assign63380_e98189_d_n5;
        var_t1_dn6 = assign63380_e98189_d_n6;
        var_t1_dn7 = assign63380_e98189_d_n7;
        var_t1_dn8 = assign63380_e98189_d_n8;
        var_t1_dn9 = assign63380_e98189_d_n9;
        var_t1_dn10 = assign63380_e98189_d_n10;
        var_t1_dn13 = assign63380_e98189_d_n13;
        var_t1_rv = 0.0;

        let (assign63390_e98200, assign63390_e98200_d_n0, assign63390_e98200_d_n2, assign63390_e98200_d_n4, assign63390_e98200_d_n5, assign63390_e98200_d_n6, assign63390_e98200_d_n7, assign63390_e98200_d_n8, assign63390_e98200_d_n9, assign63390_e98200_d_n10, assign63390_e98200_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) && (var_guard1504 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign63390_e98200;
        var_t2_dn0 = assign63390_e98200_d_n0;
        var_t2_dn2 = assign63390_e98200_d_n2;
        var_t2_dn4 = assign63390_e98200_d_n4;
        var_t2_dn5 = assign63390_e98200_d_n5;
        var_t2_dn6 = assign63390_e98200_d_n6;
        var_t2_dn7 = assign63390_e98200_d_n7;
        var_t2_dn8 = assign63390_e98200_d_n8;
        var_t2_dn9 = assign63390_e98200_d_n9;
        var_t2_dn10 = assign63390_e98200_d_n10;
        var_t2_dn13 = assign63390_e98200_d_n13;
        var_t2_rv = 0.0;

        let (assign63400_e98211, assign63400_e98211_d_n0, assign63400_e98211_d_n2, assign63400_e98211_d_n4, assign63400_e98211_d_n5, assign63400_e98211_d_n6, assign63400_e98211_d_n7, assign63400_e98211_d_n8, assign63400_e98211_d_n9, assign63400_e98211_d_n10, assign63400_e98211_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63400_e98209: f64 = (var_t1 + 1e-25);
        (assign63400_e98209, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign63400_e98211;
        var_t1_dn0 = assign63400_e98211_d_n0;
        var_t1_dn2 = assign63400_e98211_d_n2;
        var_t1_dn4 = assign63400_e98211_d_n4;
        var_t1_dn5 = assign63400_e98211_d_n5;
        var_t1_dn6 = assign63400_e98211_d_n6;
        var_t1_dn7 = assign63400_e98211_d_n7;
        var_t1_dn8 = assign63400_e98211_d_n8;
        var_t1_dn9 = assign63400_e98211_d_n9;
        var_t1_dn10 = assign63400_e98211_d_n10;
        var_t1_dn13 = assign63400_e98211_d_n13;
        var_t1_rv = 0.0;

        let (assign63410_e98221, assign63410_e98221_d_n0, assign63410_e98221_d_n2, assign63410_e98221_d_n4, assign63410_e98221_d_n5, assign63410_e98221_d_n6, assign63410_e98221_d_n7, assign63410_e98221_d_n8, assign63410_e98221_d_n9, assign63410_e98221_d_n10, assign63410_e98221_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63410_e98219: f64 = (var_t1).sqrt();
        (assign63410_e98219, (var_t1_dn0 / (2.0 * assign63410_e98219)), (var_t1_dn2 / (2.0 * assign63410_e98219)), (var_t1_dn4 / (2.0 * assign63410_e98219)), (var_t1_dn5 / (2.0 * assign63410_e98219)), (var_t1_dn6 / (2.0 * assign63410_e98219)), (var_t1_dn7 / (2.0 * assign63410_e98219)), (var_t1_dn8 / (2.0 * assign63410_e98219)), (var_t1_dn9 / (2.0 * assign63410_e98219)), (var_t1_dn10 / (2.0 * assign63410_e98219)), (var_t1_dn13 / (2.0 * assign63410_e98219)),)
    } else {
        (var_costi6, var_costi6_dn0, var_costi6_dn2, var_costi6_dn4, var_costi6_dn5, var_costi6_dn6, var_costi6_dn7, var_costi6_dn8, var_costi6_dn9, var_costi6_dn10, var_costi6_dn13,)
    }
};
        var_costi6 = assign63410_e98221;
        var_costi6_dn0 = assign63410_e98221_d_n0;
        var_costi6_dn2 = assign63410_e98221_d_n2;
        var_costi6_dn4 = assign63410_e98221_d_n4;
        var_costi6_dn5 = assign63410_e98221_d_n5;
        var_costi6_dn6 = assign63410_e98221_d_n6;
        var_costi6_dn7 = assign63410_e98221_d_n7;
        var_costi6_dn8 = assign63410_e98221_d_n8;
        var_costi6_dn9 = assign63410_e98221_d_n9;
        var_costi6_dn10 = assign63410_e98221_d_n10;
        var_costi6_dn13 = assign63410_e98221_d_n13;
        var_costi6_rv = 0.0;

        let (assign63420_e98234, assign63420_e98234_d_n0, assign63420_e98234_d_n2, assign63420_e98234_d_n4, assign63420_e98234_d_n5, assign63420_e98234_d_n6, assign63420_e98234_d_n7, assign63420_e98234_d_n8, assign63420_e98234_d_n9, assign63420_e98234_d_n10, assign63420_e98234_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63420_e98231: f64 = (1.0 - var_costi6);
        let assign63420_e98232: f64 = (var_costi4 * assign63420_e98231);
        (assign63420_e98232, ((var_costi4_dn0 * assign63420_e98231) + (var_costi4 * (-var_costi6_dn0))), ((var_costi4_dn2 * assign63420_e98231) + (var_costi4 * (-var_costi6_dn2))), ((var_costi4_dn4 * assign63420_e98231) + (var_costi4 * (-var_costi6_dn4))), ((var_costi4_dn5 * assign63420_e98231) + (var_costi4 * (-var_costi6_dn5))), ((var_costi4_dn6 * assign63420_e98231) + (var_costi4 * (-var_costi6_dn6))), ((var_costi4_dn7 * assign63420_e98231) + (var_costi4 * (-var_costi6_dn7))), ((var_costi4_dn8 * assign63420_e98231) + (var_costi4 * (-var_costi6_dn8))), ((var_costi4_dn9 * assign63420_e98231) + (var_costi4 * (-var_costi6_dn9))), ((var_costi4_dn10 * assign63420_e98231) + (var_costi4 * (-var_costi6_dn10))), ((var_costi4_dn13 * assign63420_e98231) + (var_costi4 * (-var_costi6_dn13))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign63420_e98234;
        var_t0_dn0 = assign63420_e98234_d_n0;
        var_t0_dn2 = assign63420_e98234_d_n2;
        var_t0_dn4 = assign63420_e98234_d_n4;
        var_t0_dn5 = assign63420_e98234_d_n5;
        var_t0_dn6 = assign63420_e98234_d_n6;
        var_t0_dn7 = assign63420_e98234_d_n7;
        var_t0_dn8 = assign63420_e98234_d_n8;
        var_t0_dn9 = assign63420_e98234_d_n9;
        var_t0_dn10 = assign63420_e98234_d_n10;
        var_t0_dn13 = assign63420_e98234_d_n13;
        var_t0_rv = 0.0;

        let (assign63430_e98245, assign63430_e98245_d_n0, assign63430_e98245_d_n2, assign63430_e98245_d_n4, assign63430_e98245_d_n5, assign63430_e98245_d_n6, assign63430_e98245_d_n7, assign63430_e98245_d_n8, assign63430_e98245_d_n9, assign63430_e98245_d_n10, assign63430_e98245_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63430_e98243: f64 = (var_vgssti + var_t0);
        (assign63430_e98243, (var_vgssti_dn0 + var_t0_dn0), (var_vgssti_dn2 + var_t0_dn2), (var_vgssti_dn4 + var_t0_dn4), (var_vgssti_dn5 + var_t0_dn5), (var_vgssti_dn6 + var_t0_dn6), (var_vgssti_dn7 + var_t0_dn7), (var_vgssti_dn8 + var_t0_dn8), (var_vgssti_dn9 + var_t0_dn9), (var_vgssti_dn10 + var_t0_dn10), (var_vgssti_dn13 + var_t0_dn13),)
    } else {
        (var_psasti, var_psasti_dn0, var_psasti_dn2, var_psasti_dn4, var_psasti_dn5, var_psasti_dn6, var_psasti_dn7, var_psasti_dn8, var_psasti_dn9, var_psasti_dn10, var_psasti_dn13,)
    }
};
        var_psasti = assign63430_e98245;
        var_psasti_dn0 = assign63430_e98245_d_n0;
        var_psasti_dn2 = assign63430_e98245_d_n2;
        var_psasti_dn4 = assign63430_e98245_d_n4;
        var_psasti_dn5 = assign63430_e98245_d_n5;
        var_psasti_dn6 = assign63430_e98245_d_n6;
        var_psasti_dn7 = assign63430_e98245_d_n7;
        var_psasti_dn8 = assign63430_e98245_d_n8;
        var_psasti_dn9 = assign63430_e98245_d_n9;
        var_psasti_dn10 = assign63430_e98245_d_n10;
        var_psasti_dn13 = assign63430_e98245_d_n13;
        var_psasti_rv = 0.0;

        let (assign63440_e98262, assign63440_e98262_d_n0, assign63440_e98262_d_n2, assign63440_e98262_d_n4, assign63440_e98262_d_n5, assign63440_e98262_d_n6, assign63440_e98262_d_n7, assign63440_e98262_d_n8, assign63440_e98262_d_n9, assign63440_e98262_d_n10, assign63440_e98262_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63440_e98257: f64 = (var_vgssti + 1e-25);
        let assign63440_e98258: f64 = (2.0 / assign63440_e98257);
        let assign63440_e98259: f64 = (var_beta + assign63440_e98258);
        let assign63440_e98260: f64 = (1.0 / assign63440_e98259);
        (assign63440_e98260, (-((var_beta_dn0 + (-((2.0 * var_vgssti_dn0) / (assign63440_e98257 * assign63440_e98257)))) / (assign63440_e98259 * assign63440_e98259))), (-((var_beta_dn2 + (-((2.0 * var_vgssti_dn2) / (assign63440_e98257 * assign63440_e98257)))) / (assign63440_e98259 * assign63440_e98259))), (-((var_beta_dn4 + (-((2.0 * var_vgssti_dn4) / (assign63440_e98257 * assign63440_e98257)))) / (assign63440_e98259 * assign63440_e98259))), (-((var_beta_dn5 + (-((2.0 * var_vgssti_dn5) / (assign63440_e98257 * assign63440_e98257)))) / (assign63440_e98259 * assign63440_e98259))), (-((var_beta_dn6 + (-((2.0 * var_vgssti_dn6) / (assign63440_e98257 * assign63440_e98257)))) / (assign63440_e98259 * assign63440_e98259))), (-((var_beta_dn7 + (-((2.0 * var_vgssti_dn7) / (assign63440_e98257 * assign63440_e98257)))) / (assign63440_e98259 * assign63440_e98259))), (-((var_beta_dn8 + (-((2.0 * var_vgssti_dn8) / (assign63440_e98257 * assign63440_e98257)))) / (assign63440_e98259 * assign63440_e98259))), (-((var_beta_dn9 + (-((2.0 * var_vgssti_dn9) / (assign63440_e98257 * assign63440_e98257)))) / (assign63440_e98259 * assign63440_e98259))), (-((var_beta_dn10 + (-((2.0 * var_vgssti_dn10) / (assign63440_e98257 * assign63440_e98257)))) / (assign63440_e98259 * assign63440_e98259))), (-((var_beta_dn13 + (-((2.0 * var_vgssti_dn13) / (assign63440_e98257 * assign63440_e98257)))) / (assign63440_e98259 * assign63440_e98259))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign63440_e98262;
        var_t0_dn0 = assign63440_e98262_d_n0;
        var_t0_dn2 = assign63440_e98262_d_n2;
        var_t0_dn4 = assign63440_e98262_d_n4;
        var_t0_dn5 = assign63440_e98262_d_n5;
        var_t0_dn6 = assign63440_e98262_d_n6;
        var_t0_dn7 = assign63440_e98262_d_n7;
        var_t0_dn8 = assign63440_e98262_d_n8;
        var_t0_dn9 = assign63440_e98262_d_n9;
        var_t0_dn10 = assign63440_e98262_d_n10;
        var_t0_dn13 = assign63440_e98262_d_n13;
        var_t0_rv = 0.0;

        let (assign63450_e98282, assign63450_e98282_d_n0, assign63450_e98282_d_n2, assign63450_e98282_d_n4, assign63450_e98282_d_n5, assign63450_e98282_d_n6, assign63450_e98282_d_n7, assign63450_e98282_d_n8, assign63450_e98282_d_n9, assign63450_e98282_d_n10, assign63450_e98282_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63450_e98271: f64 = (1.0 / var_costi1);
        let assign63450_e98273: f64 = (assign63450_e98271 / var_costi3);
        let assign63450_e98276: f64 = (var_vgssti * var_vgssti);
        let assign63450_e98277: f64 = (assign63450_e98273 * assign63450_e98276);
        let assign63450_e98278: f64 = (assign63450_e98277).ln();
        let assign63450_e98280: f64 = (assign63450_e98278 * var_t0);
        (assign63450_e98280, (((((((((-(var_costi1_dn0 / (var_costi1 * var_costi1))) * var_costi3) - (assign63450_e98271 * var_costi3_dn0)) / (var_costi3 * var_costi3)) * assign63450_e98276) + (assign63450_e98273 * ((var_vgssti_dn0 * var_vgssti) + (var_vgssti * var_vgssti_dn0)))) / assign63450_e98277) * var_t0) + (assign63450_e98278 * var_t0_dn0)), (((((((((-(var_costi1_dn2 / (var_costi1 * var_costi1))) * var_costi3) - (assign63450_e98271 * var_costi3_dn2)) / (var_costi3 * var_costi3)) * assign63450_e98276) + (assign63450_e98273 * ((var_vgssti_dn2 * var_vgssti) + (var_vgssti * var_vgssti_dn2)))) / assign63450_e98277) * var_t0) + (assign63450_e98278 * var_t0_dn2)), (((((((((-(var_costi1_dn4 / (var_costi1 * var_costi1))) * var_costi3) - (assign63450_e98271 * var_costi3_dn4)) / (var_costi3 * var_costi3)) * assign63450_e98276) + (assign63450_e98273 * ((var_vgssti_dn4 * var_vgssti) + (var_vgssti * var_vgssti_dn4)))) / assign63450_e98277) * var_t0) + (assign63450_e98278 * var_t0_dn4)), (((((((((-(var_costi1_dn5 / (var_costi1 * var_costi1))) * var_costi3) - (assign63450_e98271 * var_costi3_dn5)) / (var_costi3 * var_costi3)) * assign63450_e98276) + (assign63450_e98273 * ((var_vgssti_dn5 * var_vgssti) + (var_vgssti * var_vgssti_dn5)))) / assign63450_e98277) * var_t0) + (assign63450_e98278 * var_t0_dn5)), (((((((((-(var_costi1_dn6 / (var_costi1 * var_costi1))) * var_costi3) - (assign63450_e98271 * var_costi3_dn6)) / (var_costi3 * var_costi3)) * assign63450_e98276) + (assign63450_e98273 * ((var_vgssti_dn6 * var_vgssti) + (var_vgssti * var_vgssti_dn6)))) / assign63450_e98277) * var_t0) + (assign63450_e98278 * var_t0_dn6)), (((((((((-(var_costi1_dn7 / (var_costi1 * var_costi1))) * var_costi3) - (assign63450_e98271 * var_costi3_dn7)) / (var_costi3 * var_costi3)) * assign63450_e98276) + (assign63450_e98273 * ((var_vgssti_dn7 * var_vgssti) + (var_vgssti * var_vgssti_dn7)))) / assign63450_e98277) * var_t0) + (assign63450_e98278 * var_t0_dn7)), (((((((((-(var_costi1_dn8 / (var_costi1 * var_costi1))) * var_costi3) - (assign63450_e98271 * var_costi3_dn8)) / (var_costi3 * var_costi3)) * assign63450_e98276) + (assign63450_e98273 * ((var_vgssti_dn8 * var_vgssti) + (var_vgssti * var_vgssti_dn8)))) / assign63450_e98277) * var_t0) + (assign63450_e98278 * var_t0_dn8)), (((((((((-(var_costi1_dn9 / (var_costi1 * var_costi1))) * var_costi3) - (assign63450_e98271 * var_costi3_dn9)) / (var_costi3 * var_costi3)) * assign63450_e98276) + (assign63450_e98273 * ((var_vgssti_dn9 * var_vgssti) + (var_vgssti * var_vgssti_dn9)))) / assign63450_e98277) * var_t0) + (assign63450_e98278 * var_t0_dn9)), (((((((((-(var_costi1_dn10 / (var_costi1 * var_costi1))) * var_costi3) - (assign63450_e98271 * var_costi3_dn10)) / (var_costi3 * var_costi3)) * assign63450_e98276) + (assign63450_e98273 * ((var_vgssti_dn10 * var_vgssti) + (var_vgssti * var_vgssti_dn10)))) / assign63450_e98277) * var_t0) + (assign63450_e98278 * var_t0_dn10)), (((((((((-(var_costi1_dn13 / (var_costi1 * var_costi1))) * var_costi3) - (assign63450_e98271 * var_costi3_dn13)) / (var_costi3 * var_costi3)) * assign63450_e98276) + (assign63450_e98273 * ((var_vgssti_dn13 * var_vgssti) + (var_vgssti * var_vgssti_dn13)))) / assign63450_e98277) * var_t0) + (assign63450_e98278 * var_t0_dn13)),)
    } else {
        (var_psbsti, var_psbsti_dn0, var_psbsti_dn2, var_psbsti_dn4, var_psbsti_dn5, var_psbsti_dn6, var_psbsti_dn7, var_psbsti_dn8, var_psbsti_dn9, var_psbsti_dn10, var_psbsti_dn13,)
    }
};
        var_psbsti = assign63450_e98282;
        var_psbsti_dn0 = assign63450_e98282_d_n0;
        var_psbsti_dn2 = assign63450_e98282_d_n2;
        var_psbsti_dn4 = assign63450_e98282_d_n4;
        var_psbsti_dn5 = assign63450_e98282_d_n5;
        var_psbsti_dn6 = assign63450_e98282_d_n6;
        var_psbsti_dn7 = assign63450_e98282_d_n7;
        var_psbsti_dn8 = assign63450_e98282_d_n8;
        var_psbsti_dn9 = assign63450_e98282_d_n9;
        var_psbsti_dn10 = assign63450_e98282_d_n10;
        var_psbsti_dn13 = assign63450_e98282_d_n13;
        var_psbsti_rv = 0.0;

        let (assign63460_e98295, assign63460_e98295_d_n0, assign63460_e98295_d_n2, assign63460_e98295_d_n4, assign63460_e98295_d_n5, assign63460_e98295_d_n6, assign63460_e98295_d_n7, assign63460_e98295_d_n8, assign63460_e98295_d_n9, assign63460_e98295_d_n10, assign63460_e98295_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63460_e98291: f64 = (var_psbsti - var_psasti);
        let assign63460_e98293: f64 = (assign63460_e98291 - 0.002);
        (assign63460_e98293, (var_psbsti_dn0 - var_psasti_dn0), (var_psbsti_dn2 - var_psasti_dn2), (var_psbsti_dn4 - var_psasti_dn4), (var_psbsti_dn5 - var_psasti_dn5), (var_psbsti_dn6 - var_psasti_dn6), (var_psbsti_dn7 - var_psasti_dn7), (var_psbsti_dn8 - var_psasti_dn8), (var_psbsti_dn9 - var_psasti_dn9), (var_psbsti_dn10 - var_psasti_dn10), (var_psbsti_dn13 - var_psasti_dn13),)
    } else {
        (var_psab, var_psab_dn0, var_psab_dn2, var_psab_dn4, var_psab_dn5, var_psab_dn6, var_psab_dn7, var_psab_dn8, var_psab_dn9, var_psab_dn10, var_psab_dn13,)
    }
};
        var_psab = assign63460_e98295;
        var_psab_dn0 = assign63460_e98295_d_n0;
        var_psab_dn2 = assign63460_e98295_d_n2;
        var_psab_dn4 = assign63460_e98295_d_n4;
        var_psab_dn5 = assign63460_e98295_d_n5;
        var_psab_dn6 = assign63460_e98295_d_n6;
        var_psab_dn7 = assign63460_e98295_d_n7;
        var_psab_dn8 = assign63460_e98295_d_n8;
        var_psab_dn9 = assign63460_e98295_d_n9;
        var_psab_dn10 = assign63460_e98295_d_n10;
        var_psab_dn13 = assign63460_e98295_d_n13;
        var_psab_rv = 0.0;

        let (assign63470_e98313, assign63470_e98313_d_n0, assign63470_e98313_d_n2, assign63470_e98313_d_n4, assign63470_e98313_d_n5, assign63470_e98313_d_n6, assign63470_e98313_d_n7, assign63470_e98313_d_n8, assign63470_e98313_d_n9, assign63470_e98313_d_n10, assign63470_e98313_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63470_e98304: f64 = (var_psab * var_psab);
        let assign63470_e98307: f64 = (4.0 * 0.002);
        let assign63470_e98309: f64 = (assign63470_e98307 * var_psbsti);
        let assign63470_e98310: f64 = (assign63470_e98304 + assign63470_e98309);
        let assign63470_e98311: f64 = (assign63470_e98310).sqrt();
        (assign63470_e98311, ((((var_psab_dn0 * var_psab) + (var_psab * var_psab_dn0)) + (assign63470_e98307 * var_psbsti_dn0)) / (2.0 * assign63470_e98311)), ((((var_psab_dn2 * var_psab) + (var_psab * var_psab_dn2)) + (assign63470_e98307 * var_psbsti_dn2)) / (2.0 * assign63470_e98311)), ((((var_psab_dn4 * var_psab) + (var_psab * var_psab_dn4)) + (assign63470_e98307 * var_psbsti_dn4)) / (2.0 * assign63470_e98311)), ((((var_psab_dn5 * var_psab) + (var_psab * var_psab_dn5)) + (assign63470_e98307 * var_psbsti_dn5)) / (2.0 * assign63470_e98311)), ((((var_psab_dn6 * var_psab) + (var_psab * var_psab_dn6)) + (assign63470_e98307 * var_psbsti_dn6)) / (2.0 * assign63470_e98311)), ((((var_psab_dn7 * var_psab) + (var_psab * var_psab_dn7)) + (assign63470_e98307 * var_psbsti_dn7)) / (2.0 * assign63470_e98311)), ((((var_psab_dn8 * var_psab) + (var_psab * var_psab_dn8)) + (assign63470_e98307 * var_psbsti_dn8)) / (2.0 * assign63470_e98311)), ((((var_psab_dn9 * var_psab) + (var_psab * var_psab_dn9)) + (assign63470_e98307 * var_psbsti_dn9)) / (2.0 * assign63470_e98311)), ((((var_psab_dn10 * var_psab) + (var_psab * var_psab_dn10)) + (assign63470_e98307 * var_psbsti_dn10)) / (2.0 * assign63470_e98311)), ((((var_psab_dn13 * var_psab) + (var_psab * var_psab_dn13)) + (assign63470_e98307 * var_psbsti_dn13)) / (2.0 * assign63470_e98311)),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign63470_e98313;
        var_t0_dn0 = assign63470_e98313_d_n0;
        var_t0_dn2 = assign63470_e98313_d_n2;
        var_t0_dn4 = assign63470_e98313_d_n4;
        var_t0_dn5 = assign63470_e98313_d_n5;
        var_t0_dn6 = assign63470_e98313_d_n6;
        var_t0_dn7 = assign63470_e98313_d_n7;
        var_t0_dn8 = assign63470_e98313_d_n8;
        var_t0_dn9 = assign63470_e98313_d_n9;
        var_t0_dn10 = assign63470_e98313_d_n10;
        var_t0_dn13 = assign63470_e98313_d_n13;
        var_t0_rv = 0.0;

        let (assign63480_e98328, assign63480_e98328_d_n0, assign63480_e98328_d_n2, assign63480_e98328_d_n4, assign63480_e98328_d_n5, assign63480_e98328_d_n6, assign63480_e98328_d_n7, assign63480_e98328_d_n8, assign63480_e98328_d_n9, assign63480_e98328_d_n10, assign63480_e98328_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63480_e98324: f64 = (var_psab + var_t0);
        let assign63480_e98325: f64 = (0.5 * assign63480_e98324);
        let assign63480_e98326: f64 = (var_psbsti - assign63480_e98325);
        (assign63480_e98326, (var_psbsti_dn0 - (0.5 * (var_psab_dn0 + var_t0_dn0))), (var_psbsti_dn2 - (0.5 * (var_psab_dn2 + var_t0_dn2))), (var_psbsti_dn4 - (0.5 * (var_psab_dn4 + var_t0_dn4))), (var_psbsti_dn5 - (0.5 * (var_psab_dn5 + var_t0_dn5))), (var_psbsti_dn6 - (0.5 * (var_psab_dn6 + var_t0_dn6))), (var_psbsti_dn7 - (0.5 * (var_psab_dn7 + var_t0_dn7))), (var_psbsti_dn8 - (0.5 * (var_psab_dn8 + var_t0_dn8))), (var_psbsti_dn9 - (0.5 * (var_psab_dn9 + var_t0_dn9))), (var_psbsti_dn10 - (0.5 * (var_psab_dn10 + var_t0_dn10))), (var_psbsti_dn13 - (0.5 * (var_psab_dn13 + var_t0_dn13))),)
    } else {
        (var_psti, var_psti_dn0, var_psti_dn2, var_psti_dn4, var_psti_dn5, var_psti_dn6, var_psti_dn7, var_psti_dn8, var_psti_dn9, var_psti_dn10, var_psti_dn13,)
    }
};
        var_psti = assign63480_e98328;
        var_psti_dn0 = assign63480_e98328_d_n0;
        var_psti_dn2 = assign63480_e98328_d_n2;
        var_psti_dn4 = assign63480_e98328_d_n4;
        var_psti_dn5 = assign63480_e98328_d_n5;
        var_psti_dn6 = assign63480_e98328_d_n6;
        var_psti_dn7 = assign63480_e98328_d_n7;
        var_psti_dn8 = assign63480_e98328_d_n8;
        var_psti_dn9 = assign63480_e98328_d_n9;
        var_psti_dn10 = assign63480_e98328_d_n10;
        var_psti_dn13 = assign63480_e98328_d_n13;
        var_psti_rv = 0.0;

        let (assign63490_e98342, assign63490_e98342_d_n0, assign63490_e98342_d_n2, assign63490_e98342_d_n4, assign63490_e98342_d_n5, assign63490_e98342_d_n6, assign63490_e98342_d_n7, assign63490_e98342_d_n8, assign63490_e98342_d_n9, assign63490_e98342_d_n10, assign63490_e98342_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63490_e98338: f64 = (var_beta * var_psti);
        let assign63490_e98339: f64 = (assign63490_e98338).exp();
        let assign63490_e98340: f64 = (var_costi1 * assign63490_e98339);
        (assign63490_e98340, ((var_costi1_dn0 * assign63490_e98339) + (var_costi1 * (assign63490_e98339 * ((var_beta_dn0 * var_psti) + (var_beta * var_psti_dn0))))), ((var_costi1_dn2 * assign63490_e98339) + (var_costi1 * (assign63490_e98339 * ((var_beta_dn2 * var_psti) + (var_beta * var_psti_dn2))))), ((var_costi1_dn4 * assign63490_e98339) + (var_costi1 * (assign63490_e98339 * ((var_beta_dn4 * var_psti) + (var_beta * var_psti_dn4))))), ((var_costi1_dn5 * assign63490_e98339) + (var_costi1 * (assign63490_e98339 * ((var_beta_dn5 * var_psti) + (var_beta * var_psti_dn5))))), ((var_costi1_dn6 * assign63490_e98339) + (var_costi1 * (assign63490_e98339 * ((var_beta_dn6 * var_psti) + (var_beta * var_psti_dn6))))), ((var_costi1_dn7 * assign63490_e98339) + (var_costi1 * (assign63490_e98339 * ((var_beta_dn7 * var_psti) + (var_beta * var_psti_dn7))))), ((var_costi1_dn8 * assign63490_e98339) + (var_costi1 * (assign63490_e98339 * ((var_beta_dn8 * var_psti) + (var_beta * var_psti_dn8))))), ((var_costi1_dn9 * assign63490_e98339) + (var_costi1 * (assign63490_e98339 * ((var_beta_dn9 * var_psti) + (var_beta * var_psti_dn9))))), ((var_costi1_dn10 * assign63490_e98339) + (var_costi1 * (assign63490_e98339 * ((var_beta_dn10 * var_psti) + (var_beta * var_psti_dn10))))), ((var_costi1_dn13 * assign63490_e98339) + (var_costi1 * (assign63490_e98339 * ((var_beta_dn13 * var_psti) + (var_beta * var_psti_dn13))))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign63490_e98342;
        var_t0_dn0 = assign63490_e98342_d_n0;
        var_t0_dn2 = assign63490_e98342_d_n2;
        var_t0_dn4 = assign63490_e98342_d_n4;
        var_t0_dn5 = assign63490_e98342_d_n5;
        var_t0_dn6 = assign63490_e98342_d_n6;
        var_t0_dn7 = assign63490_e98342_d_n7;
        var_t0_dn8 = assign63490_e98342_d_n8;
        var_t0_dn9 = assign63490_e98342_d_n9;
        var_t0_dn10 = assign63490_e98342_d_n10;
        var_t0_dn13 = assign63490_e98342_d_n13;
        var_t0_rv = 0.0;

        let (assign63500_e98359, assign63500_e98359_d_n0, assign63500_e98359_d_n2, assign63500_e98359_d_n4, assign63500_e98359_d_n5, assign63500_e98359_d_n6, assign63500_e98359_d_n7, assign63500_e98359_d_n8, assign63500_e98359_d_n9, assign63500_e98359_d_n10, assign63500_e98359_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63500_e98352: f64 = (var_psti - var_vbsz__blk438);
        let assign63500_e98353: f64 = (var_beta * assign63500_e98352);
        let assign63500_e98355: f64 = (assign63500_e98353 - 1.0);
        let assign63500_e98357: f64 = (assign63500_e98355 + var_t0);
        (assign63500_e98357, (((var_beta_dn0 * assign63500_e98352) + (var_beta * (var_psti_dn0 - var_vbsz__blk438_dn0))) + var_t0_dn0), (((var_beta_dn2 * assign63500_e98352) + (var_beta * (var_psti_dn2 - var_vbsz__blk438_dn2))) + var_t0_dn2), (((var_beta_dn4 * assign63500_e98352) + (var_beta * (var_psti_dn4 - var_vbsz__blk438_dn4))) + var_t0_dn4), (((var_beta_dn5 * assign63500_e98352) + (var_beta * (var_psti_dn5 - var_vbsz__blk438_dn5))) + var_t0_dn5), (((var_beta_dn6 * assign63500_e98352) + (var_beta * (var_psti_dn6 - var_vbsz__blk438_dn6))) + var_t0_dn6), (((var_beta_dn7 * assign63500_e98352) + (var_beta * (var_psti_dn7 - var_vbsz__blk438_dn7))) + var_t0_dn7), (((var_beta_dn8 * assign63500_e98352) + (var_beta * (var_psti_dn8 - var_vbsz__blk438_dn8))) + var_t0_dn8), (((var_beta_dn9 * assign63500_e98352) + (var_beta * (var_psti_dn9 - var_vbsz__blk438_dn9))) + var_t0_dn9), (((var_beta_dn10 * assign63500_e98352) + (var_beta * (var_psti_dn10 - var_vbsz__blk438_dn10))) + var_t0_dn10), (((var_beta_dn13 * assign63500_e98352) + (var_beta * (var_psti_dn13 - var_vbsz__blk438_dn13))) + var_t0_dn13),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign63500_e98359;
        var_t1_dn0 = assign63500_e98359_d_n0;
        var_t1_dn2 = assign63500_e98359_d_n2;
        var_t1_dn4 = assign63500_e98359_d_n4;
        var_t1_dn5 = assign63500_e98359_d_n5;
        var_t1_dn6 = assign63500_e98359_d_n6;
        var_t1_dn7 = assign63500_e98359_d_n7;
        var_t1_dn8 = assign63500_e98359_d_n8;
        var_t1_dn9 = assign63500_e98359_d_n9;
        var_t1_dn10 = assign63500_e98359_d_n10;
        var_t1_dn13 = assign63500_e98359_d_n13;
        var_t1_rv = 0.0;

        let (assign63510_e98377, assign63510_e98377_d_n0, assign63510_e98377_d_n2, assign63510_e98377_d_n4, assign63510_e98377_d_n5, assign63510_e98377_d_n6, assign63510_e98377_d_n7, assign63510_e98377_d_n8, assign63510_e98377_d_n9, assign63510_e98377_d_n10, assign63510_e98377_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63510_e98368: f64 = (var_t1 * var_t1);
        let assign63510_e98371: f64 = (4.0 * 0.01);
        let assign63510_e98373: f64 = (assign63510_e98371 * 0.01);
        let assign63510_e98374: f64 = (assign63510_e98368 + assign63510_e98373);
        let assign63510_e98375: f64 = (assign63510_e98374).sqrt();
        (assign63510_e98375, (((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)) / (2.0 * assign63510_e98375)), (((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)) / (2.0 * assign63510_e98375)), (((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)) / (2.0 * assign63510_e98375)), (((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)) / (2.0 * assign63510_e98375)), (((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)) / (2.0 * assign63510_e98375)), (((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7)) / (2.0 * assign63510_e98375)), (((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)) / (2.0 * assign63510_e98375)), (((var_t1_dn9 * var_t1) + (var_t1 * var_t1_dn9)) / (2.0 * assign63510_e98375)), (((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10)) / (2.0 * assign63510_e98375)), (((var_t1_dn13 * var_t1) + (var_t1 * var_t1_dn13)) / (2.0 * assign63510_e98375)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
    }
};
        var_tmf2 = assign63510_e98377;
        var_tmf2_dn0 = assign63510_e98377_d_n0;
        var_tmf2_dn2 = assign63510_e98377_d_n2;
        var_tmf2_dn4 = assign63510_e98377_d_n4;
        var_tmf2_dn5 = assign63510_e98377_d_n5;
        var_tmf2_dn6 = assign63510_e98377_d_n6;
        var_tmf2_dn7 = assign63510_e98377_d_n7;
        var_tmf2_dn8 = assign63510_e98377_d_n8;
        var_tmf2_dn9 = assign63510_e98377_d_n9;
        var_tmf2_dn10 = assign63510_e98377_d_n10;
        var_tmf2_dn13 = assign63510_e98377_d_n13;
        var_tmf2_rv = 0.0;

        *var_costi6_slot = var_costi6;
        *var_costi6_dn0_slot = var_costi6_dn0;
        *var_costi6_dn10_slot = var_costi6_dn10;
        *var_costi6_dn13_slot = var_costi6_dn13;
        *var_costi6_dn2_slot = var_costi6_dn2;
        *var_costi6_dn4_slot = var_costi6_dn4;
        *var_costi6_dn5_slot = var_costi6_dn5;
        *var_costi6_dn6_slot = var_costi6_dn6;
        *var_costi6_dn7_slot = var_costi6_dn7;
        *var_costi6_dn8_slot = var_costi6_dn8;
        *var_costi6_dn9_slot = var_costi6_dn9;
        *var_costi6_rv_slot = var_costi6_rv;
        *var_guard1504_slot = var_guard1504;
        *var_guard1504_rv_slot = var_guard1504_rv;
        *var_psab_slot = var_psab;
        *var_psab_dn0_slot = var_psab_dn0;
        *var_psab_dn10_slot = var_psab_dn10;
        *var_psab_dn13_slot = var_psab_dn13;
        *var_psab_dn2_slot = var_psab_dn2;
        *var_psab_dn4_slot = var_psab_dn4;
        *var_psab_dn5_slot = var_psab_dn5;
        *var_psab_dn6_slot = var_psab_dn6;
        *var_psab_dn7_slot = var_psab_dn7;
        *var_psab_dn8_slot = var_psab_dn8;
        *var_psab_dn9_slot = var_psab_dn9;
        *var_psab_rv_slot = var_psab_rv;
        *var_psasti_slot = var_psasti;
        *var_psasti_dn0_slot = var_psasti_dn0;
        *var_psasti_dn10_slot = var_psasti_dn10;
        *var_psasti_dn13_slot = var_psasti_dn13;
        *var_psasti_dn2_slot = var_psasti_dn2;
        *var_psasti_dn4_slot = var_psasti_dn4;
        *var_psasti_dn5_slot = var_psasti_dn5;
        *var_psasti_dn6_slot = var_psasti_dn6;
        *var_psasti_dn7_slot = var_psasti_dn7;
        *var_psasti_dn8_slot = var_psasti_dn8;
        *var_psasti_dn9_slot = var_psasti_dn9;
        *var_psasti_rv_slot = var_psasti_rv;
        *var_psbsti_slot = var_psbsti;
        *var_psbsti_dn0_slot = var_psbsti_dn0;
        *var_psbsti_dn10_slot = var_psbsti_dn10;
        *var_psbsti_dn13_slot = var_psbsti_dn13;
        *var_psbsti_dn2_slot = var_psbsti_dn2;
        *var_psbsti_dn4_slot = var_psbsti_dn4;
        *var_psbsti_dn5_slot = var_psbsti_dn5;
        *var_psbsti_dn6_slot = var_psbsti_dn6;
        *var_psbsti_dn7_slot = var_psbsti_dn7;
        *var_psbsti_dn8_slot = var_psbsti_dn8;
        *var_psbsti_dn9_slot = var_psbsti_dn9;
        *var_psbsti_rv_slot = var_psbsti_rv;
        *var_psti_slot = var_psti;
        *var_psti_dn0_slot = var_psti_dn0;
        *var_psti_dn10_slot = var_psti_dn10;
        *var_psti_dn13_slot = var_psti_dn13;
        *var_psti_dn2_slot = var_psti_dn2;
        *var_psti_dn4_slot = var_psti_dn4;
        *var_psti_dn5_slot = var_psti_dn5;
        *var_psti_dn6_slot = var_psti_dn6;
        *var_psti_dn7_slot = var_psti_dn7;
        *var_psti_dn8_slot = var_psti_dn8;
        *var_psti_dn9_slot = var_psti_dn9;
        *var_psti_rv_slot = var_psti_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn13_slot = var_t0_dn13;
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
        *var_t1_dn13_slot = var_t1_dn13;
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
        *var_t2_dn13_slot = var_t2_dn13;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t2_rv_slot = var_t2_rv;
        *var_t4_slot = var_t4;
        *var_t4_dn0_slot = var_t4_dn0;
        *var_t4_dn10_slot = var_t4_dn10;
        *var_t4_dn13_slot = var_t4_dn13;
        *var_t4_dn2_slot = var_t4_dn2;
        *var_t4_dn4_slot = var_t4_dn4;
        *var_t4_dn5_slot = var_t4_dn5;
        *var_t4_dn6_slot = var_t4_dn6;
        *var_t4_dn7_slot = var_t4_dn7;
        *var_t4_dn8_slot = var_t4_dn8;
        *var_t4_dn9_slot = var_t4_dn9;
        *var_t4_rv_slot = var_t4_rv;
        *var_t5_slot = var_t5;
        *var_t5_dn0_slot = var_t5_dn0;
        *var_t5_dn10_slot = var_t5_dn10;
        *var_t5_dn13_slot = var_t5_dn13;
        *var_t5_dn2_slot = var_t5_dn2;
        *var_t5_dn4_slot = var_t5_dn4;
        *var_t5_dn5_slot = var_t5_dn5;
        *var_t5_dn6_slot = var_t5_dn6;
        *var_t5_dn7_slot = var_t5_dn7;
        *var_t5_dn8_slot = var_t5_dn8;
        *var_t5_dn9_slot = var_t5_dn9;
        *var_t5_rv_slot = var_t5_rv;
        *var_t6_slot = var_t6;
        *var_t6_dn0_slot = var_t6_dn0;
        *var_t6_dn10_slot = var_t6_dn10;
        *var_t6_dn13_slot = var_t6_dn13;
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
        *var_t7_dn13_slot = var_t7_dn13;
        *var_t7_dn2_slot = var_t7_dn2;
        *var_t7_dn4_slot = var_t7_dn4;
        *var_t7_dn5_slot = var_t7_dn5;
        *var_t7_dn6_slot = var_t7_dn6;
        *var_t7_dn7_slot = var_t7_dn7;
        *var_t7_dn8_slot = var_t7_dn8;
        *var_t7_dn9_slot = var_t7_dn9;
        *var_t7_rv_slot = var_t7_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn13_slot = var_tmf2_dn13;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn7_slot = var_tmf2_dn7;
        *var_tmf2_dn8_slot = var_tmf2_dn8;
        *var_tmf2_dn9_slot = var_tmf2_dn9;
        *var_tmf2_rv_slot = var_tmf2_rv;
    }

    pub(super) fn stamp_reactive_block_228(
        var_beta: f64,
        var_beta_dn0: f64,
        var_beta_dn10: f64,
        var_beta_dn13: f64,
        var_beta_dn2: f64,
        var_beta_dn4: f64,
        var_beta_dn5: f64,
        var_beta_dn6: f64,
        var_beta_dn7: f64,
        var_beta_dn8: f64,
        var_beta_dn9: f64,
        var_costi0: f64,
        var_costi0_dn0: f64,
        var_costi0_dn10: f64,
        var_costi0_dn13: f64,
        var_costi0_dn2: f64,
        var_costi0_dn4: f64,
        var_costi0_dn5: f64,
        var_costi0_dn6: f64,
        var_costi0_dn7: f64,
        var_costi0_dn8: f64,
        var_costi0_dn9: f64,
        var_guard1430: f64,
        var_guard1503: f64,
        var_guard443: f64,
        var_psasti: f64,
        var_psasti_dn0: f64,
        var_psasti_dn10: f64,
        var_psasti_dn13: f64,
        var_psasti_dn2: f64,
        var_psasti_dn4: f64,
        var_psasti_dn5: f64,
        var_psasti_dn6: f64,
        var_psasti_dn7: f64,
        var_psasti_dn8: f64,
        var_psasti_dn9: f64,
        var_psti: f64,
        var_psti_dn0: f64,
        var_psti_dn10: f64,
        var_psti_dn13: f64,
        var_psti_dn2: f64,
        var_psti_dn4: f64,
        var_psti_dn5: f64,
        var_psti_dn6: f64,
        var_psti_dn7: f64,
        var_psti_dn8: f64,
        var_psti_dn9: f64,
        var_vbsz__blk438: f64,
        var_vbsz__blk438_dn0: f64,
        var_vbsz__blk438_dn10: f64,
        var_vbsz__blk438_dn13: f64,
        var_vbsz__blk438_dn2: f64,
        var_vbsz__blk438_dn4: f64,
        var_vbsz__blk438_dn5: f64,
        var_vbsz__blk438_dn6: f64,
        var_vbsz__blk438_dn7: f64,
        var_vbsz__blk438_dn8: f64,
        var_vbsz__blk438_dn9: f64,
        var_guard1505_slot: &mut f64,
        var_guard1505_rv_slot: &mut f64,
        var_guard1506_slot: &mut f64,
        var_guard1506_rv_slot: &mut f64,
        var_guard1507_slot: &mut f64,
        var_guard1507_rv_slot: &mut f64,
        var_qn0sti_slot: &mut f64,
        var_qn0sti_dn0_slot: &mut f64,
        var_qn0sti_dn10_slot: &mut f64,
        var_qn0sti_dn13_slot: &mut f64,
        var_qn0sti_dn2_slot: &mut f64,
        var_qn0sti_dn4_slot: &mut f64,
        var_qn0sti_dn5_slot: &mut f64,
        var_qn0sti_dn6_slot: &mut f64,
        var_qn0sti_dn7_slot: &mut f64,
        var_qn0sti_dn8_slot: &mut f64,
        var_qn0sti_dn9_slot: &mut f64,
        var_qn0sti_rv_slot: &mut f64,
        var_sq1sti_slot: &mut f64,
        var_sq1sti_dn0_slot: &mut f64,
        var_sq1sti_dn10_slot: &mut f64,
        var_sq1sti_dn13_slot: &mut f64,
        var_sq1sti_dn2_slot: &mut f64,
        var_sq1sti_dn4_slot: &mut f64,
        var_sq1sti_dn5_slot: &mut f64,
        var_sq1sti_dn6_slot: &mut f64,
        var_sq1sti_dn7_slot: &mut f64,
        var_sq1sti_dn8_slot: &mut f64,
        var_sq1sti_dn9_slot: &mut f64,
        var_sq1sti_rv_slot: &mut f64,
        var_sq2sti_slot: &mut f64,
        var_sq2sti_dn0_slot: &mut f64,
        var_sq2sti_dn10_slot: &mut f64,
        var_sq2sti_dn13_slot: &mut f64,
        var_sq2sti_dn2_slot: &mut f64,
        var_sq2sti_dn4_slot: &mut f64,
        var_sq2sti_dn5_slot: &mut f64,
        var_sq2sti_dn6_slot: &mut f64,
        var_sq2sti_dn7_slot: &mut f64,
        var_sq2sti_dn8_slot: &mut f64,
        var_sq2sti_dn9_slot: &mut f64,
        var_sq2sti_rv_slot: &mut f64,
        var_t0_slot: &mut f64,
        var_t0_dn0_slot: &mut f64,
        var_t0_dn10_slot: &mut f64,
        var_t0_dn13_slot: &mut f64,
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
        var_t1_dn13_slot: &mut f64,
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
        var_t2_dn13_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_tmf2_slot: &mut f64,
        var_tmf2_dn0_slot: &mut f64,
        var_tmf2_dn10_slot: &mut f64,
        var_tmf2_dn13_slot: &mut f64,
        var_tmf2_dn2_slot: &mut f64,
        var_tmf2_dn4_slot: &mut f64,
        var_tmf2_dn5_slot: &mut f64,
        var_tmf2_dn6_slot: &mut f64,
        var_tmf2_dn7_slot: &mut f64,
        var_tmf2_dn8_slot: &mut f64,
        var_tmf2_dn9_slot: &mut f64,
        var_tmf2_rv_slot: &mut f64,
    ) {
        let mut var_guard1505: f64 = *var_guard1505_slot;
        let mut var_guard1505_rv: f64 = *var_guard1505_rv_slot;
        let mut var_guard1506: f64 = *var_guard1506_slot;
        let mut var_guard1506_rv: f64 = *var_guard1506_rv_slot;
        let mut var_guard1507: f64 = *var_guard1507_slot;
        let mut var_guard1507_rv: f64 = *var_guard1507_rv_slot;
        let mut var_qn0sti: f64 = *var_qn0sti_slot;
        let mut var_qn0sti_dn0: f64 = *var_qn0sti_dn0_slot;
        let mut var_qn0sti_dn10: f64 = *var_qn0sti_dn10_slot;
        let mut var_qn0sti_dn13: f64 = *var_qn0sti_dn13_slot;
        let mut var_qn0sti_dn2: f64 = *var_qn0sti_dn2_slot;
        let mut var_qn0sti_dn4: f64 = *var_qn0sti_dn4_slot;
        let mut var_qn0sti_dn5: f64 = *var_qn0sti_dn5_slot;
        let mut var_qn0sti_dn6: f64 = *var_qn0sti_dn6_slot;
        let mut var_qn0sti_dn7: f64 = *var_qn0sti_dn7_slot;
        let mut var_qn0sti_dn8: f64 = *var_qn0sti_dn8_slot;
        let mut var_qn0sti_dn9: f64 = *var_qn0sti_dn9_slot;
        let mut var_qn0sti_rv: f64 = *var_qn0sti_rv_slot;
        let mut var_sq1sti: f64 = *var_sq1sti_slot;
        let mut var_sq1sti_dn0: f64 = *var_sq1sti_dn0_slot;
        let mut var_sq1sti_dn10: f64 = *var_sq1sti_dn10_slot;
        let mut var_sq1sti_dn13: f64 = *var_sq1sti_dn13_slot;
        let mut var_sq1sti_dn2: f64 = *var_sq1sti_dn2_slot;
        let mut var_sq1sti_dn4: f64 = *var_sq1sti_dn4_slot;
        let mut var_sq1sti_dn5: f64 = *var_sq1sti_dn5_slot;
        let mut var_sq1sti_dn6: f64 = *var_sq1sti_dn6_slot;
        let mut var_sq1sti_dn7: f64 = *var_sq1sti_dn7_slot;
        let mut var_sq1sti_dn8: f64 = *var_sq1sti_dn8_slot;
        let mut var_sq1sti_dn9: f64 = *var_sq1sti_dn9_slot;
        let mut var_sq1sti_rv: f64 = *var_sq1sti_rv_slot;
        let mut var_sq2sti: f64 = *var_sq2sti_slot;
        let mut var_sq2sti_dn0: f64 = *var_sq2sti_dn0_slot;
        let mut var_sq2sti_dn10: f64 = *var_sq2sti_dn10_slot;
        let mut var_sq2sti_dn13: f64 = *var_sq2sti_dn13_slot;
        let mut var_sq2sti_dn2: f64 = *var_sq2sti_dn2_slot;
        let mut var_sq2sti_dn4: f64 = *var_sq2sti_dn4_slot;
        let mut var_sq2sti_dn5: f64 = *var_sq2sti_dn5_slot;
        let mut var_sq2sti_dn6: f64 = *var_sq2sti_dn6_slot;
        let mut var_sq2sti_dn7: f64 = *var_sq2sti_dn7_slot;
        let mut var_sq2sti_dn8: f64 = *var_sq2sti_dn8_slot;
        let mut var_sq2sti_dn9: f64 = *var_sq2sti_dn9_slot;
        let mut var_sq2sti_rv: f64 = *var_sq2sti_rv_slot;
        let mut var_t0: f64 = *var_t0_slot;
        let mut var_t0_dn0: f64 = *var_t0_dn0_slot;
        let mut var_t0_dn10: f64 = *var_t0_dn10_slot;
        let mut var_t0_dn13: f64 = *var_t0_dn13_slot;
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
        let mut var_t1_dn13: f64 = *var_t1_dn13_slot;
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
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_tmf2: f64 = *var_tmf2_slot;
        let mut var_tmf2_dn0: f64 = *var_tmf2_dn0_slot;
        let mut var_tmf2_dn10: f64 = *var_tmf2_dn10_slot;
        let mut var_tmf2_dn13: f64 = *var_tmf2_dn13_slot;
        let mut var_tmf2_dn2: f64 = *var_tmf2_dn2_slot;
        let mut var_tmf2_dn4: f64 = *var_tmf2_dn4_slot;
        let mut var_tmf2_dn5: f64 = *var_tmf2_dn5_slot;
        let mut var_tmf2_dn6: f64 = *var_tmf2_dn6_slot;
        let mut var_tmf2_dn7: f64 = *var_tmf2_dn7_slot;
        let mut var_tmf2_dn8: f64 = *var_tmf2_dn8_slot;
        let mut var_tmf2_dn9: f64 = *var_tmf2_dn9_slot;
        let mut var_tmf2_rv: f64 = *var_tmf2_rv_slot;

        let (assign63520_e98392, assign63520_e98392_d_n0, assign63520_e98392_d_n2, assign63520_e98392_d_n4, assign63520_e98392_d_n5, assign63520_e98392_d_n6, assign63520_e98392_d_n7, assign63520_e98392_d_n8, assign63520_e98392_d_n9, assign63520_e98392_d_n10, assign63520_e98392_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63520_e98388: f64 = (var_t1 / var_tmf2);
        let assign63520_e98389: f64 = (1.0 + assign63520_e98388);
        let assign63520_e98390: f64 = (0.5 * assign63520_e98389);
        (assign63520_e98390, (0.5 * (((var_t1_dn0 * var_tmf2) - (var_t1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn2 * var_tmf2) - (var_t1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn4 * var_tmf2) - (var_t1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn5 * var_tmf2) - (var_t1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn6 * var_tmf2) - (var_t1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn7 * var_tmf2) - (var_t1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn8 * var_tmf2) - (var_t1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn9 * var_tmf2) - (var_t1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn10 * var_tmf2) - (var_t1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn13 * var_tmf2) - (var_t1 * var_tmf2_dn13)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign63520_e98392;
        var_t0_dn0 = assign63520_e98392_d_n0;
        var_t0_dn2 = assign63520_e98392_d_n2;
        var_t0_dn4 = assign63520_e98392_d_n4;
        var_t0_dn5 = assign63520_e98392_d_n5;
        var_t0_dn6 = assign63520_e98392_d_n6;
        var_t0_dn7 = assign63520_e98392_d_n7;
        var_t0_dn8 = assign63520_e98392_d_n8;
        var_t0_dn9 = assign63520_e98392_d_n9;
        var_t0_dn10 = assign63520_e98392_d_n10;
        var_t0_dn13 = assign63520_e98392_d_n13;
        var_t0_rv = 0.0;

        let (assign63530_e98405, assign63530_e98405_d_n0, assign63530_e98405_d_n2, assign63530_e98405_d_n4, assign63530_e98405_d_n5, assign63530_e98405_d_n6, assign63530_e98405_d_n7, assign63530_e98405_d_n8, assign63530_e98405_d_n9, assign63530_e98405_d_n10, assign63530_e98405_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63530_e98402: f64 = (var_t1 + var_tmf2);
        let assign63530_e98403: f64 = (0.5 * assign63530_e98402);
        (assign63530_e98403, (0.5 * (var_t1_dn0 + var_tmf2_dn0)), (0.5 * (var_t1_dn2 + var_tmf2_dn2)), (0.5 * (var_t1_dn4 + var_tmf2_dn4)), (0.5 * (var_t1_dn5 + var_tmf2_dn5)), (0.5 * (var_t1_dn6 + var_tmf2_dn6)), (0.5 * (var_t1_dn7 + var_tmf2_dn7)), (0.5 * (var_t1_dn8 + var_tmf2_dn8)), (0.5 * (var_t1_dn9 + var_tmf2_dn9)), (0.5 * (var_t1_dn10 + var_tmf2_dn10)), (0.5 * (var_t1_dn13 + var_tmf2_dn13)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign63530_e98405;
        var_t1_dn0 = assign63530_e98405_d_n0;
        var_t1_dn2 = assign63530_e98405_d_n2;
        var_t1_dn4 = assign63530_e98405_d_n4;
        var_t1_dn5 = assign63530_e98405_d_n5;
        var_t1_dn6 = assign63530_e98405_d_n6;
        var_t1_dn7 = assign63530_e98405_d_n7;
        var_t1_dn8 = assign63530_e98405_d_n8;
        var_t1_dn9 = assign63530_e98405_d_n9;
        var_t1_dn10 = assign63530_e98405_d_n10;
        var_t1_dn13 = assign63530_e98405_d_n13;
        var_t1_rv = 0.0;

        let assign63540_e98408: f64 = if var_t1 < 0.0 { 1.0 } else { 0.0 };
        var_guard1505 = assign63540_e98408;
        var_guard1505_rv = 0.0;

        let (assign63550_e98419, assign63550_e98419_d_n0, assign63550_e98419_d_n2, assign63550_e98419_d_n4, assign63550_e98419_d_n5, assign63550_e98419_d_n6, assign63550_e98419_d_n7, assign63550_e98419_d_n8, assign63550_e98419_d_n9, assign63550_e98419_d_n10, assign63550_e98419_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) && (var_guard1505 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign63550_e98419;
        var_t1_dn0 = assign63550_e98419_d_n0;
        var_t1_dn2 = assign63550_e98419_d_n2;
        var_t1_dn4 = assign63550_e98419_d_n4;
        var_t1_dn5 = assign63550_e98419_d_n5;
        var_t1_dn6 = assign63550_e98419_d_n6;
        var_t1_dn7 = assign63550_e98419_d_n7;
        var_t1_dn8 = assign63550_e98419_d_n8;
        var_t1_dn9 = assign63550_e98419_d_n9;
        var_t1_dn10 = assign63550_e98419_d_n10;
        var_t1_dn13 = assign63550_e98419_d_n13;
        var_t1_rv = 0.0;

        let (assign63560_e98430, assign63560_e98430_d_n0, assign63560_e98430_d_n2, assign63560_e98430_d_n4, assign63560_e98430_d_n5, assign63560_e98430_d_n6, assign63560_e98430_d_n7, assign63560_e98430_d_n8, assign63560_e98430_d_n9, assign63560_e98430_d_n10, assign63560_e98430_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) && (var_guard1505 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign63560_e98430;
        var_t0_dn0 = assign63560_e98430_d_n0;
        var_t0_dn2 = assign63560_e98430_d_n2;
        var_t0_dn4 = assign63560_e98430_d_n4;
        var_t0_dn5 = assign63560_e98430_d_n5;
        var_t0_dn6 = assign63560_e98430_d_n6;
        var_t0_dn7 = assign63560_e98430_d_n7;
        var_t0_dn8 = assign63560_e98430_d_n8;
        var_t0_dn9 = assign63560_e98430_d_n9;
        var_t0_dn10 = assign63560_e98430_d_n10;
        var_t0_dn13 = assign63560_e98430_d_n13;
        var_t0_rv = 0.0;

        let (assign63570_e98441, assign63570_e98441_d_n0, assign63570_e98441_d_n2, assign63570_e98441_d_n4, assign63570_e98441_d_n5, assign63570_e98441_d_n6, assign63570_e98441_d_n7, assign63570_e98441_d_n8, assign63570_e98441_d_n9, assign63570_e98441_d_n10, assign63570_e98441_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63570_e98439: f64 = (var_t1 + 1e-25);
        (assign63570_e98439, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign63570_e98441;
        var_t1_dn0 = assign63570_e98441_d_n0;
        var_t1_dn2 = assign63570_e98441_d_n2;
        var_t1_dn4 = assign63570_e98441_d_n4;
        var_t1_dn5 = assign63570_e98441_d_n5;
        var_t1_dn6 = assign63570_e98441_d_n6;
        var_t1_dn7 = assign63570_e98441_d_n7;
        var_t1_dn8 = assign63570_e98441_d_n8;
        var_t1_dn9 = assign63570_e98441_d_n9;
        var_t1_dn10 = assign63570_e98441_d_n10;
        var_t1_dn13 = assign63570_e98441_d_n13;
        var_t1_rv = 0.0;

        let (assign63580_e98451, assign63580_e98451_d_n0, assign63580_e98451_d_n2, assign63580_e98451_d_n4, assign63580_e98451_d_n5, assign63580_e98451_d_n6, assign63580_e98451_d_n7, assign63580_e98451_d_n8, assign63580_e98451_d_n9, assign63580_e98451_d_n10, assign63580_e98451_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63580_e98449: f64 = (var_t1).sqrt();
        (assign63580_e98449, (var_t1_dn0 / (2.0 * assign63580_e98449)), (var_t1_dn2 / (2.0 * assign63580_e98449)), (var_t1_dn4 / (2.0 * assign63580_e98449)), (var_t1_dn5 / (2.0 * assign63580_e98449)), (var_t1_dn6 / (2.0 * assign63580_e98449)), (var_t1_dn7 / (2.0 * assign63580_e98449)), (var_t1_dn8 / (2.0 * assign63580_e98449)), (var_t1_dn9 / (2.0 * assign63580_e98449)), (var_t1_dn10 / (2.0 * assign63580_e98449)), (var_t1_dn13 / (2.0 * assign63580_e98449)),)
    } else {
        (var_sq1sti, var_sq1sti_dn0, var_sq1sti_dn2, var_sq1sti_dn4, var_sq1sti_dn5, var_sq1sti_dn6, var_sq1sti_dn7, var_sq1sti_dn8, var_sq1sti_dn9, var_sq1sti_dn10, var_sq1sti_dn13,)
    }
};
        var_sq1sti = assign63580_e98451;
        var_sq1sti_dn0 = assign63580_e98451_d_n0;
        var_sq1sti_dn2 = assign63580_e98451_d_n2;
        var_sq1sti_dn4 = assign63580_e98451_d_n4;
        var_sq1sti_dn5 = assign63580_e98451_d_n5;
        var_sq1sti_dn6 = assign63580_e98451_d_n6;
        var_sq1sti_dn7 = assign63580_e98451_d_n7;
        var_sq1sti_dn8 = assign63580_e98451_d_n8;
        var_sq1sti_dn9 = assign63580_e98451_d_n9;
        var_sq1sti_dn10 = assign63580_e98451_d_n10;
        var_sq1sti_dn13 = assign63580_e98451_d_n13;
        var_sq1sti_rv = 0.0;

        let (assign63590_e98466, assign63590_e98466_d_n0, assign63590_e98466_d_n2, assign63590_e98466_d_n4, assign63590_e98466_d_n5, assign63590_e98466_d_n6, assign63590_e98466_d_n7, assign63590_e98466_d_n8, assign63590_e98466_d_n9, assign63590_e98466_d_n10, assign63590_e98466_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63590_e98461: f64 = (var_psti - var_vbsz__blk438);
        let assign63590_e98462: f64 = (var_beta * assign63590_e98461);
        let assign63590_e98464: f64 = (assign63590_e98462 - 1.0);
        (assign63590_e98464, ((var_beta_dn0 * assign63590_e98461) + (var_beta * (var_psti_dn0 - var_vbsz__blk438_dn0))), ((var_beta_dn2 * assign63590_e98461) + (var_beta * (var_psti_dn2 - var_vbsz__blk438_dn2))), ((var_beta_dn4 * assign63590_e98461) + (var_beta * (var_psti_dn4 - var_vbsz__blk438_dn4))), ((var_beta_dn5 * assign63590_e98461) + (var_beta * (var_psti_dn5 - var_vbsz__blk438_dn5))), ((var_beta_dn6 * assign63590_e98461) + (var_beta * (var_psti_dn6 - var_vbsz__blk438_dn6))), ((var_beta_dn7 * assign63590_e98461) + (var_beta * (var_psti_dn7 - var_vbsz__blk438_dn7))), ((var_beta_dn8 * assign63590_e98461) + (var_beta * (var_psti_dn8 - var_vbsz__blk438_dn8))), ((var_beta_dn9 * assign63590_e98461) + (var_beta * (var_psti_dn9 - var_vbsz__blk438_dn9))), ((var_beta_dn10 * assign63590_e98461) + (var_beta * (var_psti_dn10 - var_vbsz__blk438_dn10))), ((var_beta_dn13 * assign63590_e98461) + (var_beta * (var_psti_dn13 - var_vbsz__blk438_dn13))),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign63590_e98466;
        var_t1_dn0 = assign63590_e98466_d_n0;
        var_t1_dn2 = assign63590_e98466_d_n2;
        var_t1_dn4 = assign63590_e98466_d_n4;
        var_t1_dn5 = assign63590_e98466_d_n5;
        var_t1_dn6 = assign63590_e98466_d_n6;
        var_t1_dn7 = assign63590_e98466_d_n7;
        var_t1_dn8 = assign63590_e98466_d_n8;
        var_t1_dn9 = assign63590_e98466_d_n9;
        var_t1_dn10 = assign63590_e98466_d_n10;
        var_t1_dn13 = assign63590_e98466_d_n13;
        var_t1_rv = 0.0;

        let (assign63600_e98484, assign63600_e98484_d_n0, assign63600_e98484_d_n2, assign63600_e98484_d_n4, assign63600_e98484_d_n5, assign63600_e98484_d_n6, assign63600_e98484_d_n7, assign63600_e98484_d_n8, assign63600_e98484_d_n9, assign63600_e98484_d_n10, assign63600_e98484_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63600_e98475: f64 = (var_t1 * var_t1);
        let assign63600_e98478: f64 = (4.0 * 0.01);
        let assign63600_e98480: f64 = (assign63600_e98478 * 0.01);
        let assign63600_e98481: f64 = (assign63600_e98475 + assign63600_e98480);
        let assign63600_e98482: f64 = (assign63600_e98481).sqrt();
        (assign63600_e98482, (((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)) / (2.0 * assign63600_e98482)), (((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)) / (2.0 * assign63600_e98482)), (((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)) / (2.0 * assign63600_e98482)), (((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)) / (2.0 * assign63600_e98482)), (((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)) / (2.0 * assign63600_e98482)), (((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7)) / (2.0 * assign63600_e98482)), (((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)) / (2.0 * assign63600_e98482)), (((var_t1_dn9 * var_t1) + (var_t1 * var_t1_dn9)) / (2.0 * assign63600_e98482)), (((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10)) / (2.0 * assign63600_e98482)), (((var_t1_dn13 * var_t1) + (var_t1 * var_t1_dn13)) / (2.0 * assign63600_e98482)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
    }
};
        var_tmf2 = assign63600_e98484;
        var_tmf2_dn0 = assign63600_e98484_d_n0;
        var_tmf2_dn2 = assign63600_e98484_d_n2;
        var_tmf2_dn4 = assign63600_e98484_d_n4;
        var_tmf2_dn5 = assign63600_e98484_d_n5;
        var_tmf2_dn6 = assign63600_e98484_d_n6;
        var_tmf2_dn7 = assign63600_e98484_d_n7;
        var_tmf2_dn8 = assign63600_e98484_d_n8;
        var_tmf2_dn9 = assign63600_e98484_d_n9;
        var_tmf2_dn10 = assign63600_e98484_d_n10;
        var_tmf2_dn13 = assign63600_e98484_d_n13;
        var_tmf2_rv = 0.0;

        let (assign63610_e98499, assign63610_e98499_d_n0, assign63610_e98499_d_n2, assign63610_e98499_d_n4, assign63610_e98499_d_n5, assign63610_e98499_d_n6, assign63610_e98499_d_n7, assign63610_e98499_d_n8, assign63610_e98499_d_n9, assign63610_e98499_d_n10, assign63610_e98499_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63610_e98495: f64 = (var_t1 / var_tmf2);
        let assign63610_e98496: f64 = (1.0 + assign63610_e98495);
        let assign63610_e98497: f64 = (0.5 * assign63610_e98496);
        (assign63610_e98497, (0.5 * (((var_t1_dn0 * var_tmf2) - (var_t1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn2 * var_tmf2) - (var_t1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn4 * var_tmf2) - (var_t1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn5 * var_tmf2) - (var_t1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn6 * var_tmf2) - (var_t1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn7 * var_tmf2) - (var_t1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn8 * var_tmf2) - (var_t1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn9 * var_tmf2) - (var_t1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn10 * var_tmf2) - (var_t1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn13 * var_tmf2) - (var_t1 * var_tmf2_dn13)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign63610_e98499;
        var_t0_dn0 = assign63610_e98499_d_n0;
        var_t0_dn2 = assign63610_e98499_d_n2;
        var_t0_dn4 = assign63610_e98499_d_n4;
        var_t0_dn5 = assign63610_e98499_d_n5;
        var_t0_dn6 = assign63610_e98499_d_n6;
        var_t0_dn7 = assign63610_e98499_d_n7;
        var_t0_dn8 = assign63610_e98499_d_n8;
        var_t0_dn9 = assign63610_e98499_d_n9;
        var_t0_dn10 = assign63610_e98499_d_n10;
        var_t0_dn13 = assign63610_e98499_d_n13;
        var_t0_rv = 0.0;

        let (assign63620_e98512, assign63620_e98512_d_n0, assign63620_e98512_d_n2, assign63620_e98512_d_n4, assign63620_e98512_d_n5, assign63620_e98512_d_n6, assign63620_e98512_d_n7, assign63620_e98512_d_n8, assign63620_e98512_d_n9, assign63620_e98512_d_n10, assign63620_e98512_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63620_e98509: f64 = (var_t1 + var_tmf2);
        let assign63620_e98510: f64 = (0.5 * assign63620_e98509);
        (assign63620_e98510, (0.5 * (var_t1_dn0 + var_tmf2_dn0)), (0.5 * (var_t1_dn2 + var_tmf2_dn2)), (0.5 * (var_t1_dn4 + var_tmf2_dn4)), (0.5 * (var_t1_dn5 + var_tmf2_dn5)), (0.5 * (var_t1_dn6 + var_tmf2_dn6)), (0.5 * (var_t1_dn7 + var_tmf2_dn7)), (0.5 * (var_t1_dn8 + var_tmf2_dn8)), (0.5 * (var_t1_dn9 + var_tmf2_dn9)), (0.5 * (var_t1_dn10 + var_tmf2_dn10)), (0.5 * (var_t1_dn13 + var_tmf2_dn13)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign63620_e98512;
        var_t1_dn0 = assign63620_e98512_d_n0;
        var_t1_dn2 = assign63620_e98512_d_n2;
        var_t1_dn4 = assign63620_e98512_d_n4;
        var_t1_dn5 = assign63620_e98512_d_n5;
        var_t1_dn6 = assign63620_e98512_d_n6;
        var_t1_dn7 = assign63620_e98512_d_n7;
        var_t1_dn8 = assign63620_e98512_d_n8;
        var_t1_dn9 = assign63620_e98512_d_n9;
        var_t1_dn10 = assign63620_e98512_d_n10;
        var_t1_dn13 = assign63620_e98512_d_n13;
        var_t1_rv = 0.0;

        let assign63630_e98515: f64 = if var_t1 < 0.0 { 1.0 } else { 0.0 };
        var_guard1506 = assign63630_e98515;
        var_guard1506_rv = 0.0;

        let (assign63640_e98526, assign63640_e98526_d_n0, assign63640_e98526_d_n2, assign63640_e98526_d_n4, assign63640_e98526_d_n5, assign63640_e98526_d_n6, assign63640_e98526_d_n7, assign63640_e98526_d_n8, assign63640_e98526_d_n9, assign63640_e98526_d_n10, assign63640_e98526_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) && (var_guard1506 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign63640_e98526;
        var_t1_dn0 = assign63640_e98526_d_n0;
        var_t1_dn2 = assign63640_e98526_d_n2;
        var_t1_dn4 = assign63640_e98526_d_n4;
        var_t1_dn5 = assign63640_e98526_d_n5;
        var_t1_dn6 = assign63640_e98526_d_n6;
        var_t1_dn7 = assign63640_e98526_d_n7;
        var_t1_dn8 = assign63640_e98526_d_n8;
        var_t1_dn9 = assign63640_e98526_d_n9;
        var_t1_dn10 = assign63640_e98526_d_n10;
        var_t1_dn13 = assign63640_e98526_d_n13;
        var_t1_rv = 0.0;

        let (assign63650_e98537, assign63650_e98537_d_n0, assign63650_e98537_d_n2, assign63650_e98537_d_n4, assign63650_e98537_d_n5, assign63650_e98537_d_n6, assign63650_e98537_d_n7, assign63650_e98537_d_n8, assign63650_e98537_d_n9, assign63650_e98537_d_n10, assign63650_e98537_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) && (var_guard1506 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t0, var_t0_dn0, var_t0_dn2, var_t0_dn4, var_t0_dn5, var_t0_dn6, var_t0_dn7, var_t0_dn8, var_t0_dn9, var_t0_dn10, var_t0_dn13,)
    }
};
        var_t0 = assign63650_e98537;
        var_t0_dn0 = assign63650_e98537_d_n0;
        var_t0_dn2 = assign63650_e98537_d_n2;
        var_t0_dn4 = assign63650_e98537_d_n4;
        var_t0_dn5 = assign63650_e98537_d_n5;
        var_t0_dn6 = assign63650_e98537_d_n6;
        var_t0_dn7 = assign63650_e98537_d_n7;
        var_t0_dn8 = assign63650_e98537_d_n8;
        var_t0_dn9 = assign63650_e98537_d_n9;
        var_t0_dn10 = assign63650_e98537_d_n10;
        var_t0_dn13 = assign63650_e98537_d_n13;
        var_t0_rv = 0.0;

        let (assign63660_e98548, assign63660_e98548_d_n0, assign63660_e98548_d_n2, assign63660_e98548_d_n4, assign63660_e98548_d_n5, assign63660_e98548_d_n6, assign63660_e98548_d_n7, assign63660_e98548_d_n8, assign63660_e98548_d_n9, assign63660_e98548_d_n10, assign63660_e98548_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63660_e98546: f64 = (var_t1 + 1e-25);
        (assign63660_e98546, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign63660_e98548;
        var_t1_dn0 = assign63660_e98548_d_n0;
        var_t1_dn2 = assign63660_e98548_d_n2;
        var_t1_dn4 = assign63660_e98548_d_n4;
        var_t1_dn5 = assign63660_e98548_d_n5;
        var_t1_dn6 = assign63660_e98548_d_n6;
        var_t1_dn7 = assign63660_e98548_d_n7;
        var_t1_dn8 = assign63660_e98548_d_n8;
        var_t1_dn9 = assign63660_e98548_d_n9;
        var_t1_dn10 = assign63660_e98548_d_n10;
        var_t1_dn13 = assign63660_e98548_d_n13;
        var_t1_rv = 0.0;

        let (assign63670_e98558, assign63670_e98558_d_n0, assign63670_e98558_d_n2, assign63670_e98558_d_n4, assign63670_e98558_d_n5, assign63670_e98558_d_n6, assign63670_e98558_d_n7, assign63670_e98558_d_n8, assign63670_e98558_d_n9, assign63670_e98558_d_n10, assign63670_e98558_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63670_e98556: f64 = (var_t1).sqrt();
        (assign63670_e98556, (var_t1_dn0 / (2.0 * assign63670_e98556)), (var_t1_dn2 / (2.0 * assign63670_e98556)), (var_t1_dn4 / (2.0 * assign63670_e98556)), (var_t1_dn5 / (2.0 * assign63670_e98556)), (var_t1_dn6 / (2.0 * assign63670_e98556)), (var_t1_dn7 / (2.0 * assign63670_e98556)), (var_t1_dn8 / (2.0 * assign63670_e98556)), (var_t1_dn9 / (2.0 * assign63670_e98556)), (var_t1_dn10 / (2.0 * assign63670_e98556)), (var_t1_dn13 / (2.0 * assign63670_e98556)),)
    } else {
        (var_sq2sti, var_sq2sti_dn0, var_sq2sti_dn2, var_sq2sti_dn4, var_sq2sti_dn5, var_sq2sti_dn6, var_sq2sti_dn7, var_sq2sti_dn8, var_sq2sti_dn9, var_sq2sti_dn10, var_sq2sti_dn13,)
    }
};
        var_sq2sti = assign63670_e98558;
        var_sq2sti_dn0 = assign63670_e98558_d_n0;
        var_sq2sti_dn2 = assign63670_e98558_d_n2;
        var_sq2sti_dn4 = assign63670_e98558_d_n4;
        var_sq2sti_dn5 = assign63670_e98558_d_n5;
        var_sq2sti_dn6 = assign63670_e98558_d_n6;
        var_sq2sti_dn7 = assign63670_e98558_d_n7;
        var_sq2sti_dn8 = assign63670_e98558_d_n8;
        var_sq2sti_dn9 = assign63670_e98558_d_n9;
        var_sq2sti_dn10 = assign63670_e98558_d_n10;
        var_sq2sti_dn13 = assign63670_e98558_d_n13;
        var_sq2sti_rv = 0.0;

        let (assign63680_e98569, assign63680_e98569_d_n0, assign63680_e98569_d_n2, assign63680_e98569_d_n4, assign63680_e98569_d_n5, assign63680_e98569_d_n6, assign63680_e98569_d_n7, assign63680_e98569_d_n8, assign63680_e98569_d_n9, assign63680_e98569_d_n10, assign63680_e98569_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63680_e98567: f64 = (0.5 / var_sq2sti);
        (assign63680_e98567, (-((0.5 * var_sq2sti_dn0) / (var_sq2sti * var_sq2sti))), (-((0.5 * var_sq2sti_dn2) / (var_sq2sti * var_sq2sti))), (-((0.5 * var_sq2sti_dn4) / (var_sq2sti * var_sq2sti))), (-((0.5 * var_sq2sti_dn5) / (var_sq2sti * var_sq2sti))), (-((0.5 * var_sq2sti_dn6) / (var_sq2sti * var_sq2sti))), (-((0.5 * var_sq2sti_dn7) / (var_sq2sti * var_sq2sti))), (-((0.5 * var_sq2sti_dn8) / (var_sq2sti * var_sq2sti))), (-((0.5 * var_sq2sti_dn9) / (var_sq2sti * var_sq2sti))), (-((0.5 * var_sq2sti_dn10) / (var_sq2sti * var_sq2sti))), (-((0.5 * var_sq2sti_dn13) / (var_sq2sti * var_sq2sti))),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign63680_e98569;
        var_t2_dn0 = assign63680_e98569_d_n0;
        var_t2_dn2 = assign63680_e98569_d_n2;
        var_t2_dn4 = assign63680_e98569_d_n4;
        var_t2_dn5 = assign63680_e98569_d_n5;
        var_t2_dn6 = assign63680_e98569_d_n6;
        var_t2_dn7 = assign63680_e98569_d_n7;
        var_t2_dn8 = assign63680_e98569_d_n8;
        var_t2_dn9 = assign63680_e98569_d_n9;
        var_t2_dn10 = assign63680_e98569_d_n10;
        var_t2_dn13 = assign63680_e98569_d_n13;
        var_t2_rv = 0.0;

        let (assign63690_e98582, assign63690_e98582_d_n0, assign63690_e98582_d_n2, assign63690_e98582_d_n4, assign63690_e98582_d_n5, assign63690_e98582_d_n6, assign63690_e98582_d_n7, assign63690_e98582_d_n8, assign63690_e98582_d_n9, assign63690_e98582_d_n10, assign63690_e98582_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63690_e98579: f64 = (var_sq1sti - var_sq2sti);
        let assign63690_e98580: f64 = (var_costi0 * assign63690_e98579);
        (assign63690_e98580, ((var_costi0_dn0 * assign63690_e98579) + (var_costi0 * (var_sq1sti_dn0 - var_sq2sti_dn0))), ((var_costi0_dn2 * assign63690_e98579) + (var_costi0 * (var_sq1sti_dn2 - var_sq2sti_dn2))), ((var_costi0_dn4 * assign63690_e98579) + (var_costi0 * (var_sq1sti_dn4 - var_sq2sti_dn4))), ((var_costi0_dn5 * assign63690_e98579) + (var_costi0 * (var_sq1sti_dn5 - var_sq2sti_dn5))), ((var_costi0_dn6 * assign63690_e98579) + (var_costi0 * (var_sq1sti_dn6 - var_sq2sti_dn6))), ((var_costi0_dn7 * assign63690_e98579) + (var_costi0 * (var_sq1sti_dn7 - var_sq2sti_dn7))), ((var_costi0_dn8 * assign63690_e98579) + (var_costi0 * (var_sq1sti_dn8 - var_sq2sti_dn8))), ((var_costi0_dn9 * assign63690_e98579) + (var_costi0 * (var_sq1sti_dn9 - var_sq2sti_dn9))), ((var_costi0_dn10 * assign63690_e98579) + (var_costi0 * (var_sq1sti_dn10 - var_sq2sti_dn10))), ((var_costi0_dn13 * assign63690_e98579) + (var_costi0 * (var_sq1sti_dn13 - var_sq2sti_dn13))),)
    } else {
        (var_qn0sti, var_qn0sti_dn0, var_qn0sti_dn2, var_qn0sti_dn4, var_qn0sti_dn5, var_qn0sti_dn6, var_qn0sti_dn7, var_qn0sti_dn8, var_qn0sti_dn9, var_qn0sti_dn10, var_qn0sti_dn13,)
    }
};
        var_qn0sti = assign63690_e98582;
        var_qn0sti_dn0 = assign63690_e98582_d_n0;
        var_qn0sti_dn2 = assign63690_e98582_d_n2;
        var_qn0sti_dn4 = assign63690_e98582_d_n4;
        var_qn0sti_dn5 = assign63690_e98582_d_n5;
        var_qn0sti_dn6 = assign63690_e98582_d_n6;
        var_qn0sti_dn7 = assign63690_e98582_d_n7;
        var_qn0sti_dn8 = assign63690_e98582_d_n8;
        var_qn0sti_dn9 = assign63690_e98582_d_n9;
        var_qn0sti_dn10 = assign63690_e98582_d_n10;
        var_qn0sti_dn13 = assign63690_e98582_d_n13;
        var_qn0sti_rv = 0.0;

        let (assign63700_e98593, assign63700_e98593_d_n0, assign63700_e98593_d_n2, assign63700_e98593_d_n4, assign63700_e98593_d_n5, assign63700_e98593_d_n6, assign63700_e98593_d_n7, assign63700_e98593_d_n8, assign63700_e98593_d_n9, assign63700_e98593_d_n10, assign63700_e98593_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63700_e98591: f64 = (var_psasti - var_psti);
        (assign63700_e98591, (var_psasti_dn0 - var_psti_dn0), (var_psasti_dn2 - var_psti_dn2), (var_psasti_dn4 - var_psti_dn4), (var_psasti_dn5 - var_psti_dn5), (var_psasti_dn6 - var_psti_dn6), (var_psasti_dn7 - var_psti_dn7), (var_psasti_dn8 - var_psti_dn8), (var_psasti_dn9 - var_psti_dn9), (var_psasti_dn10 - var_psti_dn10), (var_psasti_dn13 - var_psti_dn13),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign63700_e98593;
        var_t1_dn0 = assign63700_e98593_d_n0;
        var_t1_dn2 = assign63700_e98593_d_n2;
        var_t1_dn4 = assign63700_e98593_d_n4;
        var_t1_dn5 = assign63700_e98593_d_n5;
        var_t1_dn6 = assign63700_e98593_d_n6;
        var_t1_dn7 = assign63700_e98593_d_n7;
        var_t1_dn8 = assign63700_e98593_d_n8;
        var_t1_dn9 = assign63700_e98593_d_n9;
        var_t1_dn10 = assign63700_e98593_d_n10;
        var_t1_dn13 = assign63700_e98593_d_n13;
        var_t1_rv = 0.0;

        let (assign63710_e98611, assign63710_e98611_d_n0, assign63710_e98611_d_n2, assign63710_e98611_d_n4, assign63710_e98611_d_n5, assign63710_e98611_d_n6, assign63710_e98611_d_n7, assign63710_e98611_d_n8, assign63710_e98611_d_n9, assign63710_e98611_d_n10, assign63710_e98611_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63710_e98602: f64 = (var_t1 * var_t1);
        let assign63710_e98605: f64 = (4.0 * 0.1);
        let assign63710_e98607: f64 = (assign63710_e98605 * 0.1);
        let assign63710_e98608: f64 = (assign63710_e98602 + assign63710_e98607);
        let assign63710_e98609: f64 = (assign63710_e98608).sqrt();
        (assign63710_e98609, (((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)) / (2.0 * assign63710_e98609)), (((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)) / (2.0 * assign63710_e98609)), (((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)) / (2.0 * assign63710_e98609)), (((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)) / (2.0 * assign63710_e98609)), (((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)) / (2.0 * assign63710_e98609)), (((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7)) / (2.0 * assign63710_e98609)), (((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)) / (2.0 * assign63710_e98609)), (((var_t1_dn9 * var_t1) + (var_t1 * var_t1_dn9)) / (2.0 * assign63710_e98609)), (((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10)) / (2.0 * assign63710_e98609)), (((var_t1_dn13 * var_t1) + (var_t1 * var_t1_dn13)) / (2.0 * assign63710_e98609)),)
    } else {
        (var_tmf2, var_tmf2_dn0, var_tmf2_dn2, var_tmf2_dn4, var_tmf2_dn5, var_tmf2_dn6, var_tmf2_dn7, var_tmf2_dn8, var_tmf2_dn9, var_tmf2_dn10, var_tmf2_dn13,)
    }
};
        var_tmf2 = assign63710_e98611;
        var_tmf2_dn0 = assign63710_e98611_d_n0;
        var_tmf2_dn2 = assign63710_e98611_d_n2;
        var_tmf2_dn4 = assign63710_e98611_d_n4;
        var_tmf2_dn5 = assign63710_e98611_d_n5;
        var_tmf2_dn6 = assign63710_e98611_d_n6;
        var_tmf2_dn7 = assign63710_e98611_d_n7;
        var_tmf2_dn8 = assign63710_e98611_d_n8;
        var_tmf2_dn9 = assign63710_e98611_d_n9;
        var_tmf2_dn10 = assign63710_e98611_d_n10;
        var_tmf2_dn13 = assign63710_e98611_d_n13;
        var_tmf2_rv = 0.0;

        let (assign63720_e98626, assign63720_e98626_d_n0, assign63720_e98626_d_n2, assign63720_e98626_d_n4, assign63720_e98626_d_n5, assign63720_e98626_d_n6, assign63720_e98626_d_n7, assign63720_e98626_d_n8, assign63720_e98626_d_n9, assign63720_e98626_d_n10, assign63720_e98626_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63720_e98622: f64 = (var_t1 / var_tmf2);
        let assign63720_e98623: f64 = (1.0 + assign63720_e98622);
        let assign63720_e98624: f64 = (0.5 * assign63720_e98623);
        (assign63720_e98624, (0.5 * (((var_t1_dn0 * var_tmf2) - (var_t1 * var_tmf2_dn0)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn2 * var_tmf2) - (var_t1 * var_tmf2_dn2)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn4 * var_tmf2) - (var_t1 * var_tmf2_dn4)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn5 * var_tmf2) - (var_t1 * var_tmf2_dn5)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn6 * var_tmf2) - (var_t1 * var_tmf2_dn6)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn7 * var_tmf2) - (var_t1 * var_tmf2_dn7)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn8 * var_tmf2) - (var_t1 * var_tmf2_dn8)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn9 * var_tmf2) - (var_t1 * var_tmf2_dn9)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn10 * var_tmf2) - (var_t1 * var_tmf2_dn10)) / (var_tmf2 * var_tmf2))), (0.5 * (((var_t1_dn13 * var_tmf2) - (var_t1 * var_tmf2_dn13)) / (var_tmf2 * var_tmf2))),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign63720_e98626;
        var_t2_dn0 = assign63720_e98626_d_n0;
        var_t2_dn2 = assign63720_e98626_d_n2;
        var_t2_dn4 = assign63720_e98626_d_n4;
        var_t2_dn5 = assign63720_e98626_d_n5;
        var_t2_dn6 = assign63720_e98626_d_n6;
        var_t2_dn7 = assign63720_e98626_d_n7;
        var_t2_dn8 = assign63720_e98626_d_n8;
        var_t2_dn9 = assign63720_e98626_d_n9;
        var_t2_dn10 = assign63720_e98626_d_n10;
        var_t2_dn13 = assign63720_e98626_d_n13;
        var_t2_rv = 0.0;

        let (assign63730_e98639, assign63730_e98639_d_n0, assign63730_e98639_d_n2, assign63730_e98639_d_n4, assign63730_e98639_d_n5, assign63730_e98639_d_n6, assign63730_e98639_d_n7, assign63730_e98639_d_n8, assign63730_e98639_d_n9, assign63730_e98639_d_n10, assign63730_e98639_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63730_e98636: f64 = (var_t1 + var_tmf2);
        let assign63730_e98637: f64 = (0.5 * assign63730_e98636);
        (assign63730_e98637, (0.5 * (var_t1_dn0 + var_tmf2_dn0)), (0.5 * (var_t1_dn2 + var_tmf2_dn2)), (0.5 * (var_t1_dn4 + var_tmf2_dn4)), (0.5 * (var_t1_dn5 + var_tmf2_dn5)), (0.5 * (var_t1_dn6 + var_tmf2_dn6)), (0.5 * (var_t1_dn7 + var_tmf2_dn7)), (0.5 * (var_t1_dn8 + var_tmf2_dn8)), (0.5 * (var_t1_dn9 + var_tmf2_dn9)), (0.5 * (var_t1_dn10 + var_tmf2_dn10)), (0.5 * (var_t1_dn13 + var_tmf2_dn13)),)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign63730_e98639;
        var_t1_dn0 = assign63730_e98639_d_n0;
        var_t1_dn2 = assign63730_e98639_d_n2;
        var_t1_dn4 = assign63730_e98639_d_n4;
        var_t1_dn5 = assign63730_e98639_d_n5;
        var_t1_dn6 = assign63730_e98639_d_n6;
        var_t1_dn7 = assign63730_e98639_d_n7;
        var_t1_dn8 = assign63730_e98639_d_n8;
        var_t1_dn9 = assign63730_e98639_d_n9;
        var_t1_dn10 = assign63730_e98639_d_n10;
        var_t1_dn13 = assign63730_e98639_d_n13;
        var_t1_rv = 0.0;

        let assign63740_e98642: f64 = if var_t1 < 0.0 { 1.0 } else { 0.0 };
        var_guard1507 = assign63740_e98642;
        var_guard1507_rv = 0.0;

        let (assign63750_e98653, assign63750_e98653_d_n0, assign63750_e98653_d_n2, assign63750_e98653_d_n4, assign63750_e98653_d_n5, assign63750_e98653_d_n6, assign63750_e98653_d_n7, assign63750_e98653_d_n8, assign63750_e98653_d_n9, assign63750_e98653_d_n10, assign63750_e98653_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) && (var_guard1507 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign63750_e98653;
        var_t1_dn0 = assign63750_e98653_d_n0;
        var_t1_dn2 = assign63750_e98653_d_n2;
        var_t1_dn4 = assign63750_e98653_d_n4;
        var_t1_dn5 = assign63750_e98653_d_n5;
        var_t1_dn6 = assign63750_e98653_d_n6;
        var_t1_dn7 = assign63750_e98653_d_n7;
        var_t1_dn8 = assign63750_e98653_d_n8;
        var_t1_dn9 = assign63750_e98653_d_n9;
        var_t1_dn10 = assign63750_e98653_d_n10;
        var_t1_dn13 = assign63750_e98653_d_n13;
        var_t1_rv = 0.0;

        let (assign63760_e98664, assign63760_e98664_d_n0, assign63760_e98664_d_n2, assign63760_e98664_d_n4, assign63760_e98664_d_n5, assign63760_e98664_d_n6, assign63760_e98664_d_n7, assign63760_e98664_d_n8, assign63760_e98664_d_n9, assign63760_e98664_d_n10, assign63760_e98664_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) && (var_guard1507 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign63760_e98664;
        var_t2_dn0 = assign63760_e98664_d_n0;
        var_t2_dn2 = assign63760_e98664_d_n2;
        var_t2_dn4 = assign63760_e98664_d_n4;
        var_t2_dn5 = assign63760_e98664_d_n5;
        var_t2_dn6 = assign63760_e98664_d_n6;
        var_t2_dn7 = assign63760_e98664_d_n7;
        var_t2_dn8 = assign63760_e98664_d_n8;
        var_t2_dn9 = assign63760_e98664_d_n9;
        var_t2_dn10 = assign63760_e98664_d_n10;
        var_t2_dn13 = assign63760_e98664_d_n13;
        var_t2_rv = 0.0;

        let (assign63770_e98675, assign63770_e98675_d_n0, assign63770_e98675_d_n2, assign63770_e98675_d_n4, assign63770_e98675_d_n5, assign63770_e98675_d_n6, assign63770_e98675_d_n7, assign63770_e98675_d_n8, assign63770_e98675_d_n9, assign63770_e98675_d_n10, assign63770_e98675_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63770_e98673: f64 = (var_t1 + 1e-25);
        (assign63770_e98673, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    } else {
        (var_t1, var_t1_dn0, var_t1_dn2, var_t1_dn4, var_t1_dn5, var_t1_dn6, var_t1_dn7, var_t1_dn8, var_t1_dn9, var_t1_dn10, var_t1_dn13,)
    }
};
        var_t1 = assign63770_e98675;
        var_t1_dn0 = assign63770_e98675_d_n0;
        var_t1_dn2 = assign63770_e98675_d_n2;
        var_t1_dn4 = assign63770_e98675_d_n4;
        var_t1_dn5 = assign63770_e98675_d_n5;
        var_t1_dn6 = assign63770_e98675_d_n6;
        var_t1_dn7 = assign63770_e98675_d_n7;
        var_t1_dn8 = assign63770_e98675_d_n8;
        var_t1_dn9 = assign63770_e98675_d_n9;
        var_t1_dn10 = assign63770_e98675_d_n10;
        var_t1_dn13 = assign63770_e98675_d_n13;
        var_t1_rv = 0.0;

        *var_guard1505_slot = var_guard1505;
        *var_guard1505_rv_slot = var_guard1505_rv;
        *var_guard1506_slot = var_guard1506;
        *var_guard1506_rv_slot = var_guard1506_rv;
        *var_guard1507_slot = var_guard1507;
        *var_guard1507_rv_slot = var_guard1507_rv;
        *var_qn0sti_slot = var_qn0sti;
        *var_qn0sti_dn0_slot = var_qn0sti_dn0;
        *var_qn0sti_dn10_slot = var_qn0sti_dn10;
        *var_qn0sti_dn13_slot = var_qn0sti_dn13;
        *var_qn0sti_dn2_slot = var_qn0sti_dn2;
        *var_qn0sti_dn4_slot = var_qn0sti_dn4;
        *var_qn0sti_dn5_slot = var_qn0sti_dn5;
        *var_qn0sti_dn6_slot = var_qn0sti_dn6;
        *var_qn0sti_dn7_slot = var_qn0sti_dn7;
        *var_qn0sti_dn8_slot = var_qn0sti_dn8;
        *var_qn0sti_dn9_slot = var_qn0sti_dn9;
        *var_qn0sti_rv_slot = var_qn0sti_rv;
        *var_sq1sti_slot = var_sq1sti;
        *var_sq1sti_dn0_slot = var_sq1sti_dn0;
        *var_sq1sti_dn10_slot = var_sq1sti_dn10;
        *var_sq1sti_dn13_slot = var_sq1sti_dn13;
        *var_sq1sti_dn2_slot = var_sq1sti_dn2;
        *var_sq1sti_dn4_slot = var_sq1sti_dn4;
        *var_sq1sti_dn5_slot = var_sq1sti_dn5;
        *var_sq1sti_dn6_slot = var_sq1sti_dn6;
        *var_sq1sti_dn7_slot = var_sq1sti_dn7;
        *var_sq1sti_dn8_slot = var_sq1sti_dn8;
        *var_sq1sti_dn9_slot = var_sq1sti_dn9;
        *var_sq1sti_rv_slot = var_sq1sti_rv;
        *var_sq2sti_slot = var_sq2sti;
        *var_sq2sti_dn0_slot = var_sq2sti_dn0;
        *var_sq2sti_dn10_slot = var_sq2sti_dn10;
        *var_sq2sti_dn13_slot = var_sq2sti_dn13;
        *var_sq2sti_dn2_slot = var_sq2sti_dn2;
        *var_sq2sti_dn4_slot = var_sq2sti_dn4;
        *var_sq2sti_dn5_slot = var_sq2sti_dn5;
        *var_sq2sti_dn6_slot = var_sq2sti_dn6;
        *var_sq2sti_dn7_slot = var_sq2sti_dn7;
        *var_sq2sti_dn8_slot = var_sq2sti_dn8;
        *var_sq2sti_dn9_slot = var_sq2sti_dn9;
        *var_sq2sti_rv_slot = var_sq2sti_rv;
        *var_t0_slot = var_t0;
        *var_t0_dn0_slot = var_t0_dn0;
        *var_t0_dn10_slot = var_t0_dn10;
        *var_t0_dn13_slot = var_t0_dn13;
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
        *var_t1_dn13_slot = var_t1_dn13;
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
        *var_t2_dn13_slot = var_t2_dn13;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t2_rv_slot = var_t2_rv;
        *var_tmf2_slot = var_tmf2;
        *var_tmf2_dn0_slot = var_tmf2_dn0;
        *var_tmf2_dn10_slot = var_tmf2_dn10;
        *var_tmf2_dn13_slot = var_tmf2_dn13;
        *var_tmf2_dn2_slot = var_tmf2_dn2;
        *var_tmf2_dn4_slot = var_tmf2_dn4;
        *var_tmf2_dn5_slot = var_tmf2_dn5;
        *var_tmf2_dn6_slot = var_tmf2_dn6;
        *var_tmf2_dn7_slot = var_tmf2_dn7;
        *var_tmf2_dn8_slot = var_tmf2_dn8;
        *var_tmf2_dn9_slot = var_tmf2_dn9;
        *var_tmf2_rv_slot = var_tmf2_rv;
    }

    pub(super) fn stamp_reactive_block_229(
        var_guard1430: f64,
        var_guard1503: f64,
        var_guard443: f64,
        var_t1: f64,
        var_t1_dn0: f64,
        var_t1_dn10: f64,
        var_t1_dn13: f64,
        var_t1_dn2: f64,
        var_t1_dn4: f64,
        var_t1_dn5: f64,
        var_t1_dn6: f64,
        var_t1_dn7: f64,
        var_t1_dn8: f64,
        var_t1_dn9: f64,
        var_vds: f64,
        var_vds_dn0: f64,
        var_vds_dn10: f64,
        var_vds_dn13: f64,
        var_vds_dn2: f64,
        var_vds_dn4: f64,
        var_vds_dn5: f64,
        var_vds_dn6: f64,
        var_vds_dn7: f64,
        var_vds_dn8: f64,
        var_vds_dn9: f64,
        var_arg_slot: &mut f64,
        var_arg_dn0_slot: &mut f64,
        var_arg_dn10_slot: &mut f64,
        var_arg_dn13_slot: &mut f64,
        var_arg_dn2_slot: &mut f64,
        var_arg_dn4_slot: &mut f64,
        var_arg_dn5_slot: &mut f64,
        var_arg_dn6_slot: &mut f64,
        var_arg_dn7_slot: &mut f64,
        var_arg_dn8_slot: &mut f64,
        var_arg_dn9_slot: &mut f64,
        var_arg_rv_slot: &mut f64,
        var_dnm_slot: &mut f64,
        var_dnm_dn0_slot: &mut f64,
        var_dnm_dn10_slot: &mut f64,
        var_dnm_dn13_slot: &mut f64,
        var_dnm_dn2_slot: &mut f64,
        var_dnm_dn4_slot: &mut f64,
        var_dnm_dn5_slot: &mut f64,
        var_dnm_dn6_slot: &mut f64,
        var_dnm_dn7_slot: &mut f64,
        var_dnm_dn8_slot: &mut f64,
        var_dnm_dn9_slot: &mut f64,
        var_dnm_rv_slot: &mut f64,
        var_guard1508_slot: &mut f64,
        var_guard1508_rv_slot: &mut f64,
        var_guard1509_slot: &mut f64,
        var_guard1509_rv_slot: &mut f64,
        var_guard1510_slot: &mut f64,
        var_guard1510_rv_slot: &mut f64,
        var_guard1511_slot: &mut f64,
        var_guard1511_rv_slot: &mut f64,
        var_guard1512_slot: &mut f64,
        var_guard1512_rv_slot: &mut f64,
        var_m0_slot: &mut f64,
        var_m0_rv_slot: &mut f64,
        var_mm_slot: &mut f64,
        var_mm_rv_slot: &mut f64,
        var_t2_slot: &mut f64,
        var_t2_dn0_slot: &mut f64,
        var_t2_dn10_slot: &mut f64,
        var_t2_dn13_slot: &mut f64,
        var_t2_dn2_slot: &mut f64,
        var_t2_dn4_slot: &mut f64,
        var_t2_dn5_slot: &mut f64,
        var_t2_dn6_slot: &mut f64,
        var_t2_dn7_slot: &mut f64,
        var_t2_dn8_slot: &mut f64,
        var_t2_dn9_slot: &mut f64,
        var_t2_rv_slot: &mut f64,
        var_tx_slot: &mut f64,
        var_tx_dn0_slot: &mut f64,
        var_tx_dn10_slot: &mut f64,
        var_tx_dn13_slot: &mut f64,
        var_tx_dn2_slot: &mut f64,
        var_tx_dn4_slot: &mut f64,
        var_tx_dn5_slot: &mut f64,
        var_tx_dn6_slot: &mut f64,
        var_tx_dn7_slot: &mut f64,
        var_tx_dn8_slot: &mut f64,
        var_tx_dn9_slot: &mut f64,
        var_tx_rv_slot: &mut f64,
        var_x2_slot: &mut f64,
        var_x2_dn0_slot: &mut f64,
        var_x2_dn10_slot: &mut f64,
        var_x2_dn13_slot: &mut f64,
        var_x2_dn2_slot: &mut f64,
        var_x2_dn4_slot: &mut f64,
        var_x2_dn5_slot: &mut f64,
        var_x2_dn6_slot: &mut f64,
        var_x2_dn7_slot: &mut f64,
        var_x2_dn8_slot: &mut f64,
        var_x2_dn9_slot: &mut f64,
        var_x2_rv_slot: &mut f64,
        var_xmax2_slot: &mut f64,
        var_xmax2_dn0_slot: &mut f64,
        var_xmax2_dn10_slot: &mut f64,
        var_xmax2_dn13_slot: &mut f64,
        var_xmax2_dn2_slot: &mut f64,
        var_xmax2_dn4_slot: &mut f64,
        var_xmax2_dn5_slot: &mut f64,
        var_xmax2_dn6_slot: &mut f64,
        var_xmax2_dn7_slot: &mut f64,
        var_xmax2_dn8_slot: &mut f64,
        var_xmax2_dn9_slot: &mut f64,
        var_xmax2_rv_slot: &mut f64,
        var_xmp_slot: &mut f64,
        var_xmp_dn0_slot: &mut f64,
        var_xmp_dn10_slot: &mut f64,
        var_xmp_dn13_slot: &mut f64,
        var_xmp_dn2_slot: &mut f64,
        var_xmp_dn4_slot: &mut f64,
        var_xmp_dn5_slot: &mut f64,
        var_xmp_dn6_slot: &mut f64,
        var_xmp_dn7_slot: &mut f64,
        var_xmp_dn8_slot: &mut f64,
        var_xmp_dn9_slot: &mut f64,
        var_xmp_rv_slot: &mut f64,
        var_xp_slot: &mut f64,
        var_xp_dn0_slot: &mut f64,
        var_xp_dn10_slot: &mut f64,
        var_xp_dn13_slot: &mut f64,
        var_xp_dn2_slot: &mut f64,
        var_xp_dn4_slot: &mut f64,
        var_xp_dn5_slot: &mut f64,
        var_xp_dn6_slot: &mut f64,
        var_xp_dn7_slot: &mut f64,
        var_xp_dn8_slot: &mut f64,
        var_xp_dn9_slot: &mut f64,
        var_xp_rv_slot: &mut f64,
    ) {
        let mut var_arg: f64 = *var_arg_slot;
        let mut var_arg_dn0: f64 = *var_arg_dn0_slot;
        let mut var_arg_dn10: f64 = *var_arg_dn10_slot;
        let mut var_arg_dn13: f64 = *var_arg_dn13_slot;
        let mut var_arg_dn2: f64 = *var_arg_dn2_slot;
        let mut var_arg_dn4: f64 = *var_arg_dn4_slot;
        let mut var_arg_dn5: f64 = *var_arg_dn5_slot;
        let mut var_arg_dn6: f64 = *var_arg_dn6_slot;
        let mut var_arg_dn7: f64 = *var_arg_dn7_slot;
        let mut var_arg_dn8: f64 = *var_arg_dn8_slot;
        let mut var_arg_dn9: f64 = *var_arg_dn9_slot;
        let mut var_arg_rv: f64 = *var_arg_rv_slot;
        let mut var_dnm: f64 = *var_dnm_slot;
        let mut var_dnm_dn0: f64 = *var_dnm_dn0_slot;
        let mut var_dnm_dn10: f64 = *var_dnm_dn10_slot;
        let mut var_dnm_dn13: f64 = *var_dnm_dn13_slot;
        let mut var_dnm_dn2: f64 = *var_dnm_dn2_slot;
        let mut var_dnm_dn4: f64 = *var_dnm_dn4_slot;
        let mut var_dnm_dn5: f64 = *var_dnm_dn5_slot;
        let mut var_dnm_dn6: f64 = *var_dnm_dn6_slot;
        let mut var_dnm_dn7: f64 = *var_dnm_dn7_slot;
        let mut var_dnm_dn8: f64 = *var_dnm_dn8_slot;
        let mut var_dnm_dn9: f64 = *var_dnm_dn9_slot;
        let mut var_dnm_rv: f64 = *var_dnm_rv_slot;
        let mut var_guard1508: f64 = *var_guard1508_slot;
        let mut var_guard1508_rv: f64 = *var_guard1508_rv_slot;
        let mut var_guard1509: f64 = *var_guard1509_slot;
        let mut var_guard1509_rv: f64 = *var_guard1509_rv_slot;
        let mut var_guard1510: f64 = *var_guard1510_slot;
        let mut var_guard1510_rv: f64 = *var_guard1510_rv_slot;
        let mut var_guard1511: f64 = *var_guard1511_slot;
        let mut var_guard1511_rv: f64 = *var_guard1511_rv_slot;
        let mut var_guard1512: f64 = *var_guard1512_slot;
        let mut var_guard1512_rv: f64 = *var_guard1512_rv_slot;
        let mut var_m0: f64 = *var_m0_slot;
        let mut var_m0_rv: f64 = *var_m0_rv_slot;
        let mut var_mm: f64 = *var_mm_slot;
        let mut var_mm_rv: f64 = *var_mm_rv_slot;
        let mut var_t2: f64 = *var_t2_slot;
        let mut var_t2_dn0: f64 = *var_t2_dn0_slot;
        let mut var_t2_dn10: f64 = *var_t2_dn10_slot;
        let mut var_t2_dn13: f64 = *var_t2_dn13_slot;
        let mut var_t2_dn2: f64 = *var_t2_dn2_slot;
        let mut var_t2_dn4: f64 = *var_t2_dn4_slot;
        let mut var_t2_dn5: f64 = *var_t2_dn5_slot;
        let mut var_t2_dn6: f64 = *var_t2_dn6_slot;
        let mut var_t2_dn7: f64 = *var_t2_dn7_slot;
        let mut var_t2_dn8: f64 = *var_t2_dn8_slot;
        let mut var_t2_dn9: f64 = *var_t2_dn9_slot;
        let mut var_t2_rv: f64 = *var_t2_rv_slot;
        let mut var_tx: f64 = *var_tx_slot;
        let mut var_tx_dn0: f64 = *var_tx_dn0_slot;
        let mut var_tx_dn10: f64 = *var_tx_dn10_slot;
        let mut var_tx_dn13: f64 = *var_tx_dn13_slot;
        let mut var_tx_dn2: f64 = *var_tx_dn2_slot;
        let mut var_tx_dn4: f64 = *var_tx_dn4_slot;
        let mut var_tx_dn5: f64 = *var_tx_dn5_slot;
        let mut var_tx_dn6: f64 = *var_tx_dn6_slot;
        let mut var_tx_dn7: f64 = *var_tx_dn7_slot;
        let mut var_tx_dn8: f64 = *var_tx_dn8_slot;
        let mut var_tx_dn9: f64 = *var_tx_dn9_slot;
        let mut var_tx_rv: f64 = *var_tx_rv_slot;
        let mut var_x2: f64 = *var_x2_slot;
        let mut var_x2_dn0: f64 = *var_x2_dn0_slot;
        let mut var_x2_dn10: f64 = *var_x2_dn10_slot;
        let mut var_x2_dn13: f64 = *var_x2_dn13_slot;
        let mut var_x2_dn2: f64 = *var_x2_dn2_slot;
        let mut var_x2_dn4: f64 = *var_x2_dn4_slot;
        let mut var_x2_dn5: f64 = *var_x2_dn5_slot;
        let mut var_x2_dn6: f64 = *var_x2_dn6_slot;
        let mut var_x2_dn7: f64 = *var_x2_dn7_slot;
        let mut var_x2_dn8: f64 = *var_x2_dn8_slot;
        let mut var_x2_dn9: f64 = *var_x2_dn9_slot;
        let mut var_x2_rv: f64 = *var_x2_rv_slot;
        let mut var_xmax2: f64 = *var_xmax2_slot;
        let mut var_xmax2_dn0: f64 = *var_xmax2_dn0_slot;
        let mut var_xmax2_dn10: f64 = *var_xmax2_dn10_slot;
        let mut var_xmax2_dn13: f64 = *var_xmax2_dn13_slot;
        let mut var_xmax2_dn2: f64 = *var_xmax2_dn2_slot;
        let mut var_xmax2_dn4: f64 = *var_xmax2_dn4_slot;
        let mut var_xmax2_dn5: f64 = *var_xmax2_dn5_slot;
        let mut var_xmax2_dn6: f64 = *var_xmax2_dn6_slot;
        let mut var_xmax2_dn7: f64 = *var_xmax2_dn7_slot;
        let mut var_xmax2_dn8: f64 = *var_xmax2_dn8_slot;
        let mut var_xmax2_dn9: f64 = *var_xmax2_dn9_slot;
        let mut var_xmax2_rv: f64 = *var_xmax2_rv_slot;
        let mut var_xmp: f64 = *var_xmp_slot;
        let mut var_xmp_dn0: f64 = *var_xmp_dn0_slot;
        let mut var_xmp_dn10: f64 = *var_xmp_dn10_slot;
        let mut var_xmp_dn13: f64 = *var_xmp_dn13_slot;
        let mut var_xmp_dn2: f64 = *var_xmp_dn2_slot;
        let mut var_xmp_dn4: f64 = *var_xmp_dn4_slot;
        let mut var_xmp_dn5: f64 = *var_xmp_dn5_slot;
        let mut var_xmp_dn6: f64 = *var_xmp_dn6_slot;
        let mut var_xmp_dn7: f64 = *var_xmp_dn7_slot;
        let mut var_xmp_dn8: f64 = *var_xmp_dn8_slot;
        let mut var_xmp_dn9: f64 = *var_xmp_dn9_slot;
        let mut var_xmp_rv: f64 = *var_xmp_rv_slot;
        let mut var_xp: f64 = *var_xp_slot;
        let mut var_xp_dn0: f64 = *var_xp_dn0_slot;
        let mut var_xp_dn10: f64 = *var_xp_dn10_slot;
        let mut var_xp_dn13: f64 = *var_xp_dn13_slot;
        let mut var_xp_dn2: f64 = *var_xp_dn2_slot;
        let mut var_xp_dn4: f64 = *var_xp_dn4_slot;
        let mut var_xp_dn5: f64 = *var_xp_dn5_slot;
        let mut var_xp_dn6: f64 = *var_xp_dn6_slot;
        let mut var_xp_dn7: f64 = *var_xp_dn7_slot;
        let mut var_xp_dn8: f64 = *var_xp_dn8_slot;
        let mut var_xp_dn9: f64 = *var_xp_dn9_slot;
        let mut var_xp_rv: f64 = *var_xp_rv_slot;

        let (assign63780_e98686, assign63780_e98686_d_n0, assign63780_e98686_d_n2, assign63780_e98686_d_n4, assign63780_e98686_d_n5, assign63780_e98686_d_n6, assign63780_e98686_d_n7, assign63780_e98686_d_n8, assign63780_e98686_d_n9, assign63780_e98686_d_n10, assign63780_e98686_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63780_e98684: f64 = (var_vds / var_t1);
        (assign63780_e98684, (((var_vds_dn0 * var_t1) - (var_vds * var_t1_dn0)) / (var_t1 * var_t1)), (((var_vds_dn2 * var_t1) - (var_vds * var_t1_dn2)) / (var_t1 * var_t1)), (((var_vds_dn4 * var_t1) - (var_vds * var_t1_dn4)) / (var_t1 * var_t1)), (((var_vds_dn5 * var_t1) - (var_vds * var_t1_dn5)) / (var_t1 * var_t1)), (((var_vds_dn6 * var_t1) - (var_vds * var_t1_dn6)) / (var_t1 * var_t1)), (((var_vds_dn7 * var_t1) - (var_vds * var_t1_dn7)) / (var_t1 * var_t1)), (((var_vds_dn8 * var_t1) - (var_vds * var_t1_dn8)) / (var_t1 * var_t1)), (((var_vds_dn9 * var_t1) - (var_vds * var_t1_dn9)) / (var_t1 * var_t1)), (((var_vds_dn10 * var_t1) - (var_vds * var_t1_dn10)) / (var_t1 * var_t1)), (((var_vds_dn13 * var_t1) - (var_vds * var_t1_dn13)) / (var_t1 * var_t1)),)
    } else {
        (var_tx, var_tx_dn0, var_tx_dn2, var_tx_dn4, var_tx_dn5, var_tx_dn6, var_tx_dn7, var_tx_dn8, var_tx_dn9, var_tx_dn10, var_tx_dn13,)
    }
};
        var_tx = assign63780_e98686;
        var_tx_dn0 = assign63780_e98686_d_n0;
        var_tx_dn2 = assign63780_e98686_d_n2;
        var_tx_dn4 = assign63780_e98686_d_n4;
        var_tx_dn5 = assign63780_e98686_d_n5;
        var_tx_dn6 = assign63780_e98686_d_n6;
        var_tx_dn7 = assign63780_e98686_d_n7;
        var_tx_dn8 = assign63780_e98686_d_n8;
        var_tx_dn9 = assign63780_e98686_d_n9;
        var_tx_dn10 = assign63780_e98686_d_n10;
        var_tx_dn13 = assign63780_e98686_d_n13;
        var_tx_rv = 0.0;

        let (assign63790_e98699, assign63790_e98699_d_n0, assign63790_e98699_d_n2, assign63790_e98699_d_n4, assign63790_e98699_d_n5, assign63790_e98699_d_n6, assign63790_e98699_d_n7, assign63790_e98699_d_n8, assign63790_e98699_d_n9, assign63790_e98699_d_n10, assign63790_e98699_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63790_e98696: f64 = (var_t1 * var_t1);
        let assign63790_e98697: f64 = (1.0 / assign63790_e98696);
        (assign63790_e98697, (-(((var_t1_dn0 * var_t1) + (var_t1 * var_t1_dn0)) / (assign63790_e98696 * assign63790_e98696))), (-(((var_t1_dn2 * var_t1) + (var_t1 * var_t1_dn2)) / (assign63790_e98696 * assign63790_e98696))), (-(((var_t1_dn4 * var_t1) + (var_t1 * var_t1_dn4)) / (assign63790_e98696 * assign63790_e98696))), (-(((var_t1_dn5 * var_t1) + (var_t1 * var_t1_dn5)) / (assign63790_e98696 * assign63790_e98696))), (-(((var_t1_dn6 * var_t1) + (var_t1 * var_t1_dn6)) / (assign63790_e98696 * assign63790_e98696))), (-(((var_t1_dn7 * var_t1) + (var_t1 * var_t1_dn7)) / (assign63790_e98696 * assign63790_e98696))), (-(((var_t1_dn8 * var_t1) + (var_t1 * var_t1_dn8)) / (assign63790_e98696 * assign63790_e98696))), (-(((var_t1_dn9 * var_t1) + (var_t1 * var_t1_dn9)) / (assign63790_e98696 * assign63790_e98696))), (-(((var_t1_dn10 * var_t1) + (var_t1 * var_t1_dn10)) / (assign63790_e98696 * assign63790_e98696))), (-(((var_t1_dn13 * var_t1) + (var_t1 * var_t1_dn13)) / (assign63790_e98696 * assign63790_e98696))),)
    } else {
        (var_t2, var_t2_dn0, var_t2_dn2, var_t2_dn4, var_t2_dn5, var_t2_dn6, var_t2_dn7, var_t2_dn8, var_t2_dn9, var_t2_dn10, var_t2_dn13,)
    }
};
        var_t2 = assign63790_e98699;
        var_t2_dn0 = assign63790_e98699_d_n0;
        var_t2_dn2 = assign63790_e98699_d_n2;
        var_t2_dn4 = assign63790_e98699_d_n4;
        var_t2_dn5 = assign63790_e98699_d_n5;
        var_t2_dn6 = assign63790_e98699_d_n6;
        var_t2_dn7 = assign63790_e98699_d_n7;
        var_t2_dn8 = assign63790_e98699_d_n8;
        var_t2_dn9 = assign63790_e98699_d_n9;
        var_t2_dn10 = assign63790_e98699_d_n10;
        var_t2_dn13 = assign63790_e98699_d_n13;
        var_t2_rv = 0.0;

        let (assign63800_e98710, assign63800_e98710_d_n0, assign63800_e98710_d_n2, assign63800_e98710_d_n4, assign63800_e98710_d_n5, assign63800_e98710_d_n6, assign63800_e98710_d_n7, assign63800_e98710_d_n8, assign63800_e98710_d_n9, assign63800_e98710_d_n10, assign63800_e98710_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63800_e98708: f64 = (var_tx * var_tx);
        (assign63800_e98708, ((var_tx_dn0 * var_tx) + (var_tx * var_tx_dn0)), ((var_tx_dn2 * var_tx) + (var_tx * var_tx_dn2)), ((var_tx_dn4 * var_tx) + (var_tx * var_tx_dn4)), ((var_tx_dn5 * var_tx) + (var_tx * var_tx_dn5)), ((var_tx_dn6 * var_tx) + (var_tx * var_tx_dn6)), ((var_tx_dn7 * var_tx) + (var_tx * var_tx_dn7)), ((var_tx_dn8 * var_tx) + (var_tx * var_tx_dn8)), ((var_tx_dn9 * var_tx) + (var_tx * var_tx_dn9)), ((var_tx_dn10 * var_tx) + (var_tx * var_tx_dn10)), ((var_tx_dn13 * var_tx) + (var_tx * var_tx_dn13)),)
    } else {
        (var_x2, var_x2_dn0, var_x2_dn2, var_x2_dn4, var_x2_dn5, var_x2_dn6, var_x2_dn7, var_x2_dn8, var_x2_dn9, var_x2_dn10, var_x2_dn13,)
    }
};
        var_x2 = assign63800_e98710;
        var_x2_dn0 = assign63800_e98710_d_n0;
        var_x2_dn2 = assign63800_e98710_d_n2;
        var_x2_dn4 = assign63800_e98710_d_n4;
        var_x2_dn5 = assign63800_e98710_d_n5;
        var_x2_dn6 = assign63800_e98710_d_n6;
        var_x2_dn7 = assign63800_e98710_d_n7;
        var_x2_dn8 = assign63800_e98710_d_n8;
        var_x2_dn9 = assign63800_e98710_d_n9;
        var_x2_dn10 = assign63800_e98710_d_n10;
        var_x2_dn13 = assign63800_e98710_d_n13;
        var_x2_rv = 0.0;

        let (assign63810_e98721, assign63810_e98721_d_n0, assign63810_e98721_d_n2, assign63810_e98721_d_n4, assign63810_e98721_d_n5, assign63810_e98721_d_n6, assign63810_e98721_d_n7, assign63810_e98721_d_n8, assign63810_e98721_d_n9, assign63810_e98721_d_n10, assign63810_e98721_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63810_e98719: f64 = 1.0;
        (assign63810_e98719, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmax2, var_xmax2_dn0, var_xmax2_dn2, var_xmax2_dn4, var_xmax2_dn5, var_xmax2_dn6, var_xmax2_dn7, var_xmax2_dn8, var_xmax2_dn9, var_xmax2_dn10, var_xmax2_dn13,)
    }
};
        var_xmax2 = assign63810_e98721;
        var_xmax2_dn0 = assign63810_e98721_d_n0;
        var_xmax2_dn2 = assign63810_e98721_d_n2;
        var_xmax2_dn4 = assign63810_e98721_d_n4;
        var_xmax2_dn5 = assign63810_e98721_d_n5;
        var_xmax2_dn6 = assign63810_e98721_d_n6;
        var_xmax2_dn7 = assign63810_e98721_d_n7;
        var_xmax2_dn8 = assign63810_e98721_d_n8;
        var_xmax2_dn9 = assign63810_e98721_d_n9;
        var_xmax2_dn10 = assign63810_e98721_d_n10;
        var_xmax2_dn13 = assign63810_e98721_d_n13;
        var_xmax2_rv = 0.0;

        let (assign63820_e98730, assign63820_e98730_d_n0, assign63820_e98730_d_n2, assign63820_e98730_d_n4, assign63820_e98730_d_n5, assign63820_e98730_d_n6, assign63820_e98730_d_n7, assign63820_e98730_d_n8, assign63820_e98730_d_n9, assign63820_e98730_d_n10, assign63820_e98730_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn7, var_xp_dn8, var_xp_dn9, var_xp_dn10, var_xp_dn13,)
    }
};
        var_xp = assign63820_e98730;
        var_xp_dn0 = assign63820_e98730_d_n0;
        var_xp_dn2 = assign63820_e98730_d_n2;
        var_xp_dn4 = assign63820_e98730_d_n4;
        var_xp_dn5 = assign63820_e98730_d_n5;
        var_xp_dn6 = assign63820_e98730_d_n6;
        var_xp_dn7 = assign63820_e98730_d_n7;
        var_xp_dn8 = assign63820_e98730_d_n8;
        var_xp_dn9 = assign63820_e98730_d_n9;
        var_xp_dn10 = assign63820_e98730_d_n10;
        var_xp_dn13 = assign63820_e98730_d_n13;
        var_xp_rv = 0.0;

        let (assign63830_e98739, assign63830_e98739_d_n0, assign63830_e98739_d_n2, assign63830_e98739_d_n4, assign63830_e98739_d_n5, assign63830_e98739_d_n6, assign63830_e98739_d_n7, assign63830_e98739_d_n8, assign63830_e98739_d_n9, assign63830_e98739_d_n10, assign63830_e98739_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn7, var_xmp_dn8, var_xmp_dn9, var_xmp_dn10, var_xmp_dn13,)
    }
};
        var_xmp = assign63830_e98739;
        var_xmp_dn0 = assign63830_e98739_d_n0;
        var_xmp_dn2 = assign63830_e98739_d_n2;
        var_xmp_dn4 = assign63830_e98739_d_n4;
        var_xmp_dn5 = assign63830_e98739_d_n5;
        var_xmp_dn6 = assign63830_e98739_d_n6;
        var_xmp_dn7 = assign63830_e98739_d_n7;
        var_xmp_dn8 = assign63830_e98739_d_n8;
        var_xmp_dn9 = assign63830_e98739_d_n9;
        var_xmp_dn10 = assign63830_e98739_d_n10;
        var_xmp_dn13 = assign63830_e98739_d_n13;
        var_xmp_rv = 0.0;

        let (assign63840_e98748,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign63840_e98748;
        var_m0_rv = 0.0;

        let (assign63850_e98757,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        (0.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign63850_e98757;
        var_mm_rv = 0.0;

        let (assign63860_e98766, assign63860_e98766_d_n0, assign63860_e98766_d_n2, assign63860_e98766_d_n4, assign63860_e98766_d_n5, assign63860_e98766_d_n6, assign63860_e98766_d_n7, assign63860_e98766_d_n8, assign63860_e98766_d_n9, assign63860_e98766_d_n10, assign63860_e98766_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_dn10, var_arg_dn13,)
    }
};
        var_arg = assign63860_e98766;
        var_arg_dn0 = assign63860_e98766_d_n0;
        var_arg_dn2 = assign63860_e98766_d_n2;
        var_arg_dn4 = assign63860_e98766_d_n4;
        var_arg_dn5 = assign63860_e98766_d_n5;
        var_arg_dn6 = assign63860_e98766_d_n6;
        var_arg_dn7 = assign63860_e98766_d_n7;
        var_arg_dn8 = assign63860_e98766_d_n8;
        var_arg_dn9 = assign63860_e98766_d_n9;
        var_arg_dn10 = assign63860_e98766_d_n10;
        var_arg_dn13 = assign63860_e98766_d_n13;
        var_arg_rv = 0.0;

        let (assign63870_e98775, assign63870_e98775_d_n0, assign63870_e98775_d_n2, assign63870_e98775_d_n4, assign63870_e98775_d_n5, assign63870_e98775_d_n6, assign63870_e98775_d_n7, assign63870_e98775_d_n8, assign63870_e98775_d_n9, assign63870_e98775_d_n10, assign63870_e98775_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
        var_dnm = assign63870_e98775;
        var_dnm_dn0 = assign63870_e98775_d_n0;
        var_dnm_dn2 = assign63870_e98775_d_n2;
        var_dnm_dn4 = assign63870_e98775_d_n4;
        var_dnm_dn5 = assign63870_e98775_d_n5;
        var_dnm_dn6 = assign63870_e98775_d_n6;
        var_dnm_dn7 = assign63870_e98775_d_n7;
        var_dnm_dn8 = assign63870_e98775_d_n8;
        var_dnm_dn9 = assign63870_e98775_d_n9;
        var_dnm_dn10 = assign63870_e98775_d_n10;
        var_dnm_dn13 = assign63870_e98775_d_n13;
        var_dnm_rv = 0.0;

        let (assign63880_e98786, assign63880_e98786_d_n0, assign63880_e98786_d_n2, assign63880_e98786_d_n4, assign63880_e98786_d_n5, assign63880_e98786_d_n6, assign63880_e98786_d_n7, assign63880_e98786_d_n8, assign63880_e98786_d_n9, assign63880_e98786_d_n10, assign63880_e98786_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63880_e98784: f64 = (var_xp * var_x2);
        (assign63880_e98784, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn4 * var_x2) + (var_xp * var_x2_dn4)), ((var_xp_dn5 * var_x2) + (var_xp * var_x2_dn5)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn8 * var_x2) + (var_xp * var_x2_dn8)), ((var_xp_dn9 * var_x2) + (var_xp * var_x2_dn9)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn13 * var_x2) + (var_xp * var_x2_dn13)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn7, var_xp_dn8, var_xp_dn9, var_xp_dn10, var_xp_dn13,)
    }
};
        var_xp = assign63880_e98786;
        var_xp_dn0 = assign63880_e98786_d_n0;
        var_xp_dn2 = assign63880_e98786_d_n2;
        var_xp_dn4 = assign63880_e98786_d_n4;
        var_xp_dn5 = assign63880_e98786_d_n5;
        var_xp_dn6 = assign63880_e98786_d_n6;
        var_xp_dn7 = assign63880_e98786_d_n7;
        var_xp_dn8 = assign63880_e98786_d_n8;
        var_xp_dn9 = assign63880_e98786_d_n9;
        var_xp_dn10 = assign63880_e98786_d_n10;
        var_xp_dn13 = assign63880_e98786_d_n13;
        var_xp_rv = 0.0;

        let (assign63890_e98797, assign63890_e98797_d_n0, assign63890_e98797_d_n2, assign63890_e98797_d_n4, assign63890_e98797_d_n5, assign63890_e98797_d_n6, assign63890_e98797_d_n7, assign63890_e98797_d_n8, assign63890_e98797_d_n9, assign63890_e98797_d_n10, assign63890_e98797_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63890_e98795: f64 = (var_xmp * var_xmax2);
        (assign63890_e98795, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn4 * var_xmax2) + (var_xmp * var_xmax2_dn4)), ((var_xmp_dn5 * var_xmax2) + (var_xmp * var_xmax2_dn5)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn8 * var_xmax2) + (var_xmp * var_xmax2_dn8)), ((var_xmp_dn9 * var_xmax2) + (var_xmp * var_xmax2_dn9)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn13 * var_xmax2) + (var_xmp * var_xmax2_dn13)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn7, var_xmp_dn8, var_xmp_dn9, var_xmp_dn10, var_xmp_dn13,)
    }
};
        var_xmp = assign63890_e98797;
        var_xmp_dn0 = assign63890_e98797_d_n0;
        var_xmp_dn2 = assign63890_e98797_d_n2;
        var_xmp_dn4 = assign63890_e98797_d_n4;
        var_xmp_dn5 = assign63890_e98797_d_n5;
        var_xmp_dn6 = assign63890_e98797_d_n6;
        var_xmp_dn7 = assign63890_e98797_d_n7;
        var_xmp_dn8 = assign63890_e98797_d_n8;
        var_xmp_dn9 = assign63890_e98797_d_n9;
        var_xmp_dn10 = assign63890_e98797_d_n10;
        var_xmp_dn13 = assign63890_e98797_d_n13;
        var_xmp_rv = 0.0;

        let (assign63900_e98808, assign63900_e98808_d_n0, assign63900_e98808_d_n2, assign63900_e98808_d_n4, assign63900_e98808_d_n5, assign63900_e98808_d_n6, assign63900_e98808_d_n7, assign63900_e98808_d_n8, assign63900_e98808_d_n9, assign63900_e98808_d_n10, assign63900_e98808_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63900_e98806: f64 = (var_xp * var_x2);
        (assign63900_e98806, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn4 * var_x2) + (var_xp * var_x2_dn4)), ((var_xp_dn5 * var_x2) + (var_xp * var_x2_dn5)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn8 * var_x2) + (var_xp * var_x2_dn8)), ((var_xp_dn9 * var_x2) + (var_xp * var_x2_dn9)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn13 * var_x2) + (var_xp * var_x2_dn13)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn7, var_xp_dn8, var_xp_dn9, var_xp_dn10, var_xp_dn13,)
    }
};
        var_xp = assign63900_e98808;
        var_xp_dn0 = assign63900_e98808_d_n0;
        var_xp_dn2 = assign63900_e98808_d_n2;
        var_xp_dn4 = assign63900_e98808_d_n4;
        var_xp_dn5 = assign63900_e98808_d_n5;
        var_xp_dn6 = assign63900_e98808_d_n6;
        var_xp_dn7 = assign63900_e98808_d_n7;
        var_xp_dn8 = assign63900_e98808_d_n8;
        var_xp_dn9 = assign63900_e98808_d_n9;
        var_xp_dn10 = assign63900_e98808_d_n10;
        var_xp_dn13 = assign63900_e98808_d_n13;
        var_xp_rv = 0.0;

        let (assign63910_e98819, assign63910_e98819_d_n0, assign63910_e98819_d_n2, assign63910_e98819_d_n4, assign63910_e98819_d_n5, assign63910_e98819_d_n6, assign63910_e98819_d_n7, assign63910_e98819_d_n8, assign63910_e98819_d_n9, assign63910_e98819_d_n10, assign63910_e98819_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63910_e98817: f64 = (var_xmp * var_xmax2);
        (assign63910_e98817, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn4 * var_xmax2) + (var_xmp * var_xmax2_dn4)), ((var_xmp_dn5 * var_xmax2) + (var_xmp * var_xmax2_dn5)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn8 * var_xmax2) + (var_xmp * var_xmax2_dn8)), ((var_xmp_dn9 * var_xmax2) + (var_xmp * var_xmax2_dn9)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn13 * var_xmax2) + (var_xmp * var_xmax2_dn13)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn7, var_xmp_dn8, var_xmp_dn9, var_xmp_dn10, var_xmp_dn13,)
    }
};
        var_xmp = assign63910_e98819;
        var_xmp_dn0 = assign63910_e98819_d_n0;
        var_xmp_dn2 = assign63910_e98819_d_n2;
        var_xmp_dn4 = assign63910_e98819_d_n4;
        var_xmp_dn5 = assign63910_e98819_d_n5;
        var_xmp_dn6 = assign63910_e98819_d_n6;
        var_xmp_dn7 = assign63910_e98819_d_n7;
        var_xmp_dn8 = assign63910_e98819_d_n8;
        var_xmp_dn9 = assign63910_e98819_d_n9;
        var_xmp_dn10 = assign63910_e98819_d_n10;
        var_xmp_dn13 = assign63910_e98819_d_n13;
        var_xmp_rv = 0.0;

        let (assign63920_e98830, assign63920_e98830_d_n0, assign63920_e98830_d_n2, assign63920_e98830_d_n4, assign63920_e98830_d_n5, assign63920_e98830_d_n6, assign63920_e98830_d_n7, assign63920_e98830_d_n8, assign63920_e98830_d_n9, assign63920_e98830_d_n10, assign63920_e98830_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63920_e98828: f64 = (var_xp * var_x2);
        (assign63920_e98828, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn4 * var_x2) + (var_xp * var_x2_dn4)), ((var_xp_dn5 * var_x2) + (var_xp * var_x2_dn5)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn8 * var_x2) + (var_xp * var_x2_dn8)), ((var_xp_dn9 * var_x2) + (var_xp * var_x2_dn9)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn13 * var_x2) + (var_xp * var_x2_dn13)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn7, var_xp_dn8, var_xp_dn9, var_xp_dn10, var_xp_dn13,)
    }
};
        var_xp = assign63920_e98830;
        var_xp_dn0 = assign63920_e98830_d_n0;
        var_xp_dn2 = assign63920_e98830_d_n2;
        var_xp_dn4 = assign63920_e98830_d_n4;
        var_xp_dn5 = assign63920_e98830_d_n5;
        var_xp_dn6 = assign63920_e98830_d_n6;
        var_xp_dn7 = assign63920_e98830_d_n7;
        var_xp_dn8 = assign63920_e98830_d_n8;
        var_xp_dn9 = assign63920_e98830_d_n9;
        var_xp_dn10 = assign63920_e98830_d_n10;
        var_xp_dn13 = assign63920_e98830_d_n13;
        var_xp_rv = 0.0;

        let (assign63930_e98841, assign63930_e98841_d_n0, assign63930_e98841_d_n2, assign63930_e98841_d_n4, assign63930_e98841_d_n5, assign63930_e98841_d_n6, assign63930_e98841_d_n7, assign63930_e98841_d_n8, assign63930_e98841_d_n9, assign63930_e98841_d_n10, assign63930_e98841_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63930_e98839: f64 = (var_xmp * var_xmax2);
        (assign63930_e98839, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn4 * var_xmax2) + (var_xmp * var_xmax2_dn4)), ((var_xmp_dn5 * var_xmax2) + (var_xmp * var_xmax2_dn5)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn8 * var_xmax2) + (var_xmp * var_xmax2_dn8)), ((var_xmp_dn9 * var_xmax2) + (var_xmp * var_xmax2_dn9)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn13 * var_xmax2) + (var_xmp * var_xmax2_dn13)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn7, var_xmp_dn8, var_xmp_dn9, var_xmp_dn10, var_xmp_dn13,)
    }
};
        var_xmp = assign63930_e98841;
        var_xmp_dn0 = assign63930_e98841_d_n0;
        var_xmp_dn2 = assign63930_e98841_d_n2;
        var_xmp_dn4 = assign63930_e98841_d_n4;
        var_xmp_dn5 = assign63930_e98841_d_n5;
        var_xmp_dn6 = assign63930_e98841_d_n6;
        var_xmp_dn7 = assign63930_e98841_d_n7;
        var_xmp_dn8 = assign63930_e98841_d_n8;
        var_xmp_dn9 = assign63930_e98841_d_n9;
        var_xmp_dn10 = assign63930_e98841_d_n10;
        var_xmp_dn13 = assign63930_e98841_d_n13;
        var_xmp_rv = 0.0;

        let (assign63940_e98852, assign63940_e98852_d_n0, assign63940_e98852_d_n2, assign63940_e98852_d_n4, assign63940_e98852_d_n5, assign63940_e98852_d_n6, assign63940_e98852_d_n7, assign63940_e98852_d_n8, assign63940_e98852_d_n9, assign63940_e98852_d_n10, assign63940_e98852_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63940_e98850: f64 = (var_xp * var_x2);
        (assign63940_e98850, ((var_xp_dn0 * var_x2) + (var_xp * var_x2_dn0)), ((var_xp_dn2 * var_x2) + (var_xp * var_x2_dn2)), ((var_xp_dn4 * var_x2) + (var_xp * var_x2_dn4)), ((var_xp_dn5 * var_x2) + (var_xp * var_x2_dn5)), ((var_xp_dn6 * var_x2) + (var_xp * var_x2_dn6)), ((var_xp_dn7 * var_x2) + (var_xp * var_x2_dn7)), ((var_xp_dn8 * var_x2) + (var_xp * var_x2_dn8)), ((var_xp_dn9 * var_x2) + (var_xp * var_x2_dn9)), ((var_xp_dn10 * var_x2) + (var_xp * var_x2_dn10)), ((var_xp_dn13 * var_x2) + (var_xp * var_x2_dn13)),)
    } else {
        (var_xp, var_xp_dn0, var_xp_dn2, var_xp_dn4, var_xp_dn5, var_xp_dn6, var_xp_dn7, var_xp_dn8, var_xp_dn9, var_xp_dn10, var_xp_dn13,)
    }
};
        var_xp = assign63940_e98852;
        var_xp_dn0 = assign63940_e98852_d_n0;
        var_xp_dn2 = assign63940_e98852_d_n2;
        var_xp_dn4 = assign63940_e98852_d_n4;
        var_xp_dn5 = assign63940_e98852_d_n5;
        var_xp_dn6 = assign63940_e98852_d_n6;
        var_xp_dn7 = assign63940_e98852_d_n7;
        var_xp_dn8 = assign63940_e98852_d_n8;
        var_xp_dn9 = assign63940_e98852_d_n9;
        var_xp_dn10 = assign63940_e98852_d_n10;
        var_xp_dn13 = assign63940_e98852_d_n13;
        var_xp_rv = 0.0;

        let (assign63950_e98863, assign63950_e98863_d_n0, assign63950_e98863_d_n2, assign63950_e98863_d_n4, assign63950_e98863_d_n5, assign63950_e98863_d_n6, assign63950_e98863_d_n7, assign63950_e98863_d_n8, assign63950_e98863_d_n9, assign63950_e98863_d_n10, assign63950_e98863_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63950_e98861: f64 = (var_xmp * var_xmax2);
        (assign63950_e98861, ((var_xmp_dn0 * var_xmax2) + (var_xmp * var_xmax2_dn0)), ((var_xmp_dn2 * var_xmax2) + (var_xmp * var_xmax2_dn2)), ((var_xmp_dn4 * var_xmax2) + (var_xmp * var_xmax2_dn4)), ((var_xmp_dn5 * var_xmax2) + (var_xmp * var_xmax2_dn5)), ((var_xmp_dn6 * var_xmax2) + (var_xmp * var_xmax2_dn6)), ((var_xmp_dn7 * var_xmax2) + (var_xmp * var_xmax2_dn7)), ((var_xmp_dn8 * var_xmax2) + (var_xmp * var_xmax2_dn8)), ((var_xmp_dn9 * var_xmax2) + (var_xmp * var_xmax2_dn9)), ((var_xmp_dn10 * var_xmax2) + (var_xmp * var_xmax2_dn10)), ((var_xmp_dn13 * var_xmax2) + (var_xmp * var_xmax2_dn13)),)
    } else {
        (var_xmp, var_xmp_dn0, var_xmp_dn2, var_xmp_dn4, var_xmp_dn5, var_xmp_dn6, var_xmp_dn7, var_xmp_dn8, var_xmp_dn9, var_xmp_dn10, var_xmp_dn13,)
    }
};
        var_xmp = assign63950_e98863;
        var_xmp_dn0 = assign63950_e98863_d_n0;
        var_xmp_dn2 = assign63950_e98863_d_n2;
        var_xmp_dn4 = assign63950_e98863_d_n4;
        var_xmp_dn5 = assign63950_e98863_d_n5;
        var_xmp_dn6 = assign63950_e98863_d_n6;
        var_xmp_dn7 = assign63950_e98863_d_n7;
        var_xmp_dn8 = assign63950_e98863_d_n8;
        var_xmp_dn9 = assign63950_e98863_d_n9;
        var_xmp_dn10 = assign63950_e98863_d_n10;
        var_xmp_dn13 = assign63950_e98863_d_n13;
        var_xmp_rv = 0.0;

        let (assign63960_e98874, assign63960_e98874_d_n0, assign63960_e98874_d_n2, assign63960_e98874_d_n4, assign63960_e98874_d_n5, assign63960_e98874_d_n6, assign63960_e98874_d_n7, assign63960_e98874_d_n8, assign63960_e98874_d_n9, assign63960_e98874_d_n10, assign63960_e98874_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        let assign63960_e98872: f64 = (var_xp + var_xmp);
        (assign63960_e98872, (var_xp_dn0 + var_xmp_dn0), (var_xp_dn2 + var_xmp_dn2), (var_xp_dn4 + var_xmp_dn4), (var_xp_dn5 + var_xmp_dn5), (var_xp_dn6 + var_xmp_dn6), (var_xp_dn7 + var_xmp_dn7), (var_xp_dn8 + var_xmp_dn8), (var_xp_dn9 + var_xmp_dn9), (var_xp_dn10 + var_xmp_dn10), (var_xp_dn13 + var_xmp_dn13),)
    } else {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_dn10, var_arg_dn13,)
    }
};
        var_arg = assign63960_e98874;
        var_arg_dn0 = assign63960_e98874_d_n0;
        var_arg_dn2 = assign63960_e98874_d_n2;
        var_arg_dn4 = assign63960_e98874_d_n4;
        var_arg_dn5 = assign63960_e98874_d_n5;
        var_arg_dn6 = assign63960_e98874_d_n6;
        var_arg_dn7 = assign63960_e98874_d_n7;
        var_arg_dn8 = assign63960_e98874_d_n8;
        var_arg_dn9 = assign63960_e98874_d_n9;
        var_arg_dn10 = assign63960_e98874_d_n10;
        var_arg_dn13 = assign63960_e98874_d_n13;
        var_arg_rv = 0.0;

        let (assign63970_e98883, assign63970_e98883_d_n0, assign63970_e98883_d_n2, assign63970_e98883_d_n4, assign63970_e98883_d_n5, assign63970_e98883_d_n6, assign63970_e98883_d_n7, assign63970_e98883_d_n8, assign63970_e98883_d_n9, assign63970_e98883_d_n10, assign63970_e98883_d_n13,) = {
    if (((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) {
        (var_arg, var_arg_dn0, var_arg_dn2, var_arg_dn4, var_arg_dn5, var_arg_dn6, var_arg_dn7, var_arg_dn8, var_arg_dn9, var_arg_dn10, var_arg_dn13,)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
        var_dnm = assign63970_e98883;
        var_dnm_dn0 = assign63970_e98883_d_n0;
        var_dnm_dn2 = assign63970_e98883_d_n2;
        var_dnm_dn4 = assign63970_e98883_d_n4;
        var_dnm_dn5 = assign63970_e98883_d_n5;
        var_dnm_dn6 = assign63970_e98883_d_n6;
        var_dnm_dn7 = assign63970_e98883_d_n7;
        var_dnm_dn8 = assign63970_e98883_d_n8;
        var_dnm_dn9 = assign63970_e98883_d_n9;
        var_dnm_dn10 = assign63970_e98883_d_n10;
        var_dnm_dn13 = assign63970_e98883_d_n13;
        var_dnm_rv = 0.0;

        let assign63980_e98898: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        var_guard1508 = assign63980_e98898;
        var_guard1508_rv = 0.0;

        let assign63990_e98901: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard1509 = assign63990_e98901;
        var_guard1509_rv = 0.0;

        let (assign64000_e98914,) = {
    if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) && (var_guard1508 != 0.0)) && (var_guard1509 != 0.0)) {
        (1.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign64000_e98914;
        var_mm_rv = 0.0;

        let assign64010_e98917: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        var_guard1510 = assign64010_e98917;
        var_guard1510_rv = 0.0;

        let (assign64020_e98933,) = {
    if ((((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) && (var_guard1508 != 0.0)) && (var_guard1509 == 0.0)) && (var_guard1510 != 0.0)) {
        (2.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign64020_e98933;
        var_mm_rv = 0.0;

        let assign64030_e98936: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        var_guard1511 = assign64030_e98936;
        var_guard1511_rv = 0.0;

        let (assign64040_e98955,) = {
    if (((((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) && (var_guard1508 != 0.0)) && (var_guard1509 == 0.0)) && (var_guard1510 == 0.0)) && (var_guard1511 != 0.0)) {
        (3.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign64040_e98955;
        var_mm_rv = 0.0;

        let assign64050_e98958: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        var_guard1512 = assign64050_e98958;
        var_guard1512_rv = 0.0;

        let (assign64060_e98980,) = {
    if ((((((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) && (var_guard1508 != 0.0)) && (var_guard1509 == 0.0)) && (var_guard1510 == 0.0)) && (var_guard1511 == 0.0)) && (var_guard1512 != 0.0)) {
        (4.0,)
    } else {
        (var_mm,)
    }
};
        var_mm = assign64060_e98980;
        var_mm_rv = 0.0;

        let (assign64070_e98991,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) && (var_guard1508 != 0.0)) {
        (0.0,)
    } else {
        (var_m0,)
    }
};
        var_m0 = assign64070_e98991;
        var_m0_rv = 0.0;

        let mut assign64080_loop_guard: usize = 0;
        while {
            let assign64080_cond_e99003: f64 = if (((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) && (var_guard1508 != 0.0)) && (var_m0 < var_mm)) { 1.0 } else { 0.0 };
            assign64080_cond_e99003 != 0.0
        } {
            assign64080_loop_guard += 1;
            assert!(assign64080_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign64080_body0_e99015, assign64080_body0_e99015_d_n0, assign64080_body0_e99015_d_n2, assign64080_body0_e99015_d_n4, assign64080_body0_e99015_d_n5, assign64080_body0_e99015_d_n6, assign64080_body0_e99015_d_n7, assign64080_body0_e99015_d_n8, assign64080_body0_e99015_d_n9, assign64080_body0_e99015_d_n10, assign64080_body0_e99015_d_n13,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) && (var_guard1508 != 0.0)) {
        let assign64080_body0_e99013: f64 = (var_dnm).sqrt();
        (assign64080_body0_e99013, (var_dnm_dn0 / (2.0 * assign64080_body0_e99013)), (var_dnm_dn2 / (2.0 * assign64080_body0_e99013)), (var_dnm_dn4 / (2.0 * assign64080_body0_e99013)), (var_dnm_dn5 / (2.0 * assign64080_body0_e99013)), (var_dnm_dn6 / (2.0 * assign64080_body0_e99013)), (var_dnm_dn7 / (2.0 * assign64080_body0_e99013)), (var_dnm_dn8 / (2.0 * assign64080_body0_e99013)), (var_dnm_dn9 / (2.0 * assign64080_body0_e99013)), (var_dnm_dn10 / (2.0 * assign64080_body0_e99013)), (var_dnm_dn13 / (2.0 * assign64080_body0_e99013)),)
    } else {
        (var_dnm, var_dnm_dn0, var_dnm_dn2, var_dnm_dn4, var_dnm_dn5, var_dnm_dn6, var_dnm_dn7, var_dnm_dn8, var_dnm_dn9, var_dnm_dn10, var_dnm_dn13,)
    }
};
            var_dnm = assign64080_body0_e99015;
            var_dnm_dn0 = assign64080_body0_e99015_d_n0;
            var_dnm_dn2 = assign64080_body0_e99015_d_n2;
            var_dnm_dn4 = assign64080_body0_e99015_d_n4;
            var_dnm_dn5 = assign64080_body0_e99015_d_n5;
            var_dnm_dn6 = assign64080_body0_e99015_d_n6;
            var_dnm_dn7 = assign64080_body0_e99015_d_n7;
            var_dnm_dn8 = assign64080_body0_e99015_d_n8;
            var_dnm_dn9 = assign64080_body0_e99015_d_n9;
            var_dnm_dn10 = assign64080_body0_e99015_d_n10;
            var_dnm_dn13 = assign64080_body0_e99015_d_n13;
            var_dnm_rv = 0.0;
            let (assign64080_body1_e99028,) = {
    if ((((var_guard443 == 0.0) && (var_guard1430 != 0.0)) && (var_guard1503 != 0.0)) && (var_guard1508 != 0.0)) {
        let assign64080_body1_e99026: f64 = (var_m0 + 1.0);
        (assign64080_body1_e99026,)
    } else {
        (var_m0,)
    }
};
            var_m0 = assign64080_body1_e99028;
            var_m0_rv = 0.0;
        }

        *var_arg_slot = var_arg;
        *var_arg_dn0_slot = var_arg_dn0;
        *var_arg_dn10_slot = var_arg_dn10;
        *var_arg_dn13_slot = var_arg_dn13;
        *var_arg_dn2_slot = var_arg_dn2;
        *var_arg_dn4_slot = var_arg_dn4;
        *var_arg_dn5_slot = var_arg_dn5;
        *var_arg_dn6_slot = var_arg_dn6;
        *var_arg_dn7_slot = var_arg_dn7;
        *var_arg_dn8_slot = var_arg_dn8;
        *var_arg_dn9_slot = var_arg_dn9;
        *var_arg_rv_slot = var_arg_rv;
        *var_dnm_slot = var_dnm;
        *var_dnm_dn0_slot = var_dnm_dn0;
        *var_dnm_dn10_slot = var_dnm_dn10;
        *var_dnm_dn13_slot = var_dnm_dn13;
        *var_dnm_dn2_slot = var_dnm_dn2;
        *var_dnm_dn4_slot = var_dnm_dn4;
        *var_dnm_dn5_slot = var_dnm_dn5;
        *var_dnm_dn6_slot = var_dnm_dn6;
        *var_dnm_dn7_slot = var_dnm_dn7;
        *var_dnm_dn8_slot = var_dnm_dn8;
        *var_dnm_dn9_slot = var_dnm_dn9;
        *var_dnm_rv_slot = var_dnm_rv;
        *var_guard1508_slot = var_guard1508;
        *var_guard1508_rv_slot = var_guard1508_rv;
        *var_guard1509_slot = var_guard1509;
        *var_guard1509_rv_slot = var_guard1509_rv;
        *var_guard1510_slot = var_guard1510;
        *var_guard1510_rv_slot = var_guard1510_rv;
        *var_guard1511_slot = var_guard1511;
        *var_guard1511_rv_slot = var_guard1511_rv;
        *var_guard1512_slot = var_guard1512;
        *var_guard1512_rv_slot = var_guard1512_rv;
        *var_m0_slot = var_m0;
        *var_m0_rv_slot = var_m0_rv;
        *var_mm_slot = var_mm;
        *var_mm_rv_slot = var_mm_rv;
        *var_t2_slot = var_t2;
        *var_t2_dn0_slot = var_t2_dn0;
        *var_t2_dn10_slot = var_t2_dn10;
        *var_t2_dn13_slot = var_t2_dn13;
        *var_t2_dn2_slot = var_t2_dn2;
        *var_t2_dn4_slot = var_t2_dn4;
        *var_t2_dn5_slot = var_t2_dn5;
        *var_t2_dn6_slot = var_t2_dn6;
        *var_t2_dn7_slot = var_t2_dn7;
        *var_t2_dn8_slot = var_t2_dn8;
        *var_t2_dn9_slot = var_t2_dn9;
        *var_t2_rv_slot = var_t2_rv;
        *var_tx_slot = var_tx;
        *var_tx_dn0_slot = var_tx_dn0;
        *var_tx_dn10_slot = var_tx_dn10;
        *var_tx_dn13_slot = var_tx_dn13;
        *var_tx_dn2_slot = var_tx_dn2;
        *var_tx_dn4_slot = var_tx_dn4;
        *var_tx_dn5_slot = var_tx_dn5;
        *var_tx_dn6_slot = var_tx_dn6;
        *var_tx_dn7_slot = var_tx_dn7;
        *var_tx_dn8_slot = var_tx_dn8;
        *var_tx_dn9_slot = var_tx_dn9;
        *var_tx_rv_slot = var_tx_rv;
        *var_x2_slot = var_x2;
        *var_x2_dn0_slot = var_x2_dn0;
        *var_x2_dn10_slot = var_x2_dn10;
        *var_x2_dn13_slot = var_x2_dn13;
        *var_x2_dn2_slot = var_x2_dn2;
        *var_x2_dn4_slot = var_x2_dn4;
        *var_x2_dn5_slot = var_x2_dn5;
        *var_x2_dn6_slot = var_x2_dn6;
        *var_x2_dn7_slot = var_x2_dn7;
        *var_x2_dn8_slot = var_x2_dn8;
        *var_x2_dn9_slot = var_x2_dn9;
        *var_x2_rv_slot = var_x2_rv;
        *var_xmax2_slot = var_xmax2;
        *var_xmax2_dn0_slot = var_xmax2_dn0;
        *var_xmax2_dn10_slot = var_xmax2_dn10;
        *var_xmax2_dn13_slot = var_xmax2_dn13;
        *var_xmax2_dn2_slot = var_xmax2_dn2;
        *var_xmax2_dn4_slot = var_xmax2_dn4;
        *var_xmax2_dn5_slot = var_xmax2_dn5;
        *var_xmax2_dn6_slot = var_xmax2_dn6;
        *var_xmax2_dn7_slot = var_xmax2_dn7;
        *var_xmax2_dn8_slot = var_xmax2_dn8;
        *var_xmax2_dn9_slot = var_xmax2_dn9;
        *var_xmax2_rv_slot = var_xmax2_rv;
        *var_xmp_slot = var_xmp;
        *var_xmp_dn0_slot = var_xmp_dn0;
        *var_xmp_dn10_slot = var_xmp_dn10;
        *var_xmp_dn13_slot = var_xmp_dn13;
        *var_xmp_dn2_slot = var_xmp_dn2;
        *var_xmp_dn4_slot = var_xmp_dn4;
        *var_xmp_dn5_slot = var_xmp_dn5;
        *var_xmp_dn6_slot = var_xmp_dn6;
        *var_xmp_dn7_slot = var_xmp_dn7;
        *var_xmp_dn8_slot = var_xmp_dn8;
        *var_xmp_dn9_slot = var_xmp_dn9;
        *var_xmp_rv_slot = var_xmp_rv;
        *var_xp_slot = var_xp;
        *var_xp_dn0_slot = var_xp_dn0;
        *var_xp_dn10_slot = var_xp_dn10;
        *var_xp_dn13_slot = var_xp_dn13;
        *var_xp_dn2_slot = var_xp_dn2;
        *var_xp_dn4_slot = var_xp_dn4;
        *var_xp_dn5_slot = var_xp_dn5;
        *var_xp_dn6_slot = var_xp_dn6;
        *var_xp_dn7_slot = var_xp_dn7;
        *var_xp_dn8_slot = var_xp_dn8;
        *var_xp_dn9_slot = var_xp_dn9;
        *var_xp_rv_slot = var_xp_rv;
    }
}
